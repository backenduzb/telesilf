//! Tug'ilgan kun tabrigi xizmati.
//!
//! `/birthday_for Ism` buyrug'i berilgan chatda animatsion ASCII harflardan
//! yasalgan, HTML marquee oynasiga o'xshab harakatlanuvchi tabrikni ishga
//! tushiradi va `/stopbirthday` aytilgunicha (yoki chatdagi boshqa
//! `/birthday_for` boshlanguncha) davom ettiradi.
//!
//! Animatsiya har 400 ms da xabarni tahrirlash orqali yangilanadi:
//! silliq (ease-in-out) harakat, ramka uslublarining almashinuvi, harflar
//! ustidan suzib o'tuvchi yorug'lik to'lqini va uchqun/puls fazasi.

use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::Duration;

use teloxide::prelude::*;
use teloxide::types::{MessageId, ParseMode};
use teloxide::{ApiError, RequestError};
use tokio_util::sync::CancellationToken;

use crate::app::{AppState, BirthdayHandle};

/// Har bir boshlangan tabrik uchun noyob ID (eski tabrikni o'chirishda ishlatiladi).
static BIRTHDAY_SEQ: AtomicU64 = AtomicU64::new(1);

/// Marquee oynasi kengligi — banner shu kenglikdagi oyna ichida harakatlanadi.
const VIEWPORT: usize = 32;

/// Ikki kadr (tahrirlash) orasidagi pauza — 400 ms.
///
/// Telegram har bir chat uchun tahrirlashni cheklashi mumkin; xatolik
/// bo'lsa `animate_edit` avtomatik kutib, qayta urinadi.
const STEP_MS: u64 = 400;

/// Kirish/chiqish fazasidagi kadrlar soni (silliq tezlashuv uchun).
const MOTION_FRAMES: usize = 12;

/// Yorug'lik to'lqini fazasidagi kadrlar soni.
const SHIMMER_FRAMES: usize = 8;

/// Uchqun/puls fazasidagi kadrlar soni.
const SPARKLE_FRAMES: usize = 5;

/// Bitta kadrning ko'rinishini tavsiflaydi.
#[derive(Clone, Copy)]
struct Frame {
    /// Oynaning banner ichidagi o'rni (siljish).
    pos: usize,
    /// Ramka uslubi indeksi (0, 1, 2 — tsikl).
    style: usize,
    /// Yuqori yulduz qatori uchun belgi.
    star: char,
    /// Pastki yulduz qatori uchun belgi.
    star_bottom: char,
    /// Burchaklar uchqun belgilariga almashadimi.
    corners: bool,
    /// Yorug'lik effekti.
    brightness: Brightness,
}

#[derive(Clone, Copy)]
enum Brightness {
    /// Oddiy — harflar `#` bilan.
    Normal,
    /// To'lqin — oyna koordinatalaridagi `wave` ustuni atrofidagi
    /// harflar `█` (yorug') ko'rinishida.
    Band(usize),
    /// Hammasi yorug' — butun matn `█` bilan.
    Full,
}

/// Chatda yangi tug'ilgan kun tabrigini boshlaydi.
///
/// Agar o'sha chatda avvalgi tabrik ishlayotgan bo'lsa, u avtomatik
/// to'xtatiladi (faqat eng oxirgisi davom etadi).
pub fn start_birthday(app: &Arc<AppState>, bot: Bot, chat_id: ChatId, name: String) {
    if let Some(existing) = app.birthdays.get(&chat_id) {
        existing.token.cancel();
    }

    let token = CancellationToken::new();
    let id = BIRTHDAY_SEQ.fetch_add(1, Ordering::Relaxed);
    app.birthdays
        .insert(chat_id, BirthdayHandle { id, token: token.clone() });

    let app = Arc::clone(app);
    tokio::spawn(async move {
        run_animation(bot, chat_id, name, token).await;

        // Agar biz hali ham "oxirgi" tabrik bo'lsak — ro'yxatdan o'chiramiz.
        if let Some(handle) = app.birthdays.get(&chat_id) {
            if handle.id == id {
                app.birthdays.remove(&chat_id);
            }
        }
    });
}

/// Chatdagi faol tabrikni to'xtatadi. Faol tabrik bo'lsa `true` qaytaradi.
pub fn stop_birthday(app: &AppState, chat_id: ChatId) -> bool {
    if let Some((_, handle)) = app.birthdays.remove(&chat_id) {
        handle.token.cancel();
        true
    } else {
        false
    }
}

/// Asosiy animatsiya sikli: bitta xabar yuboriladi va uni
/// har 400 ms da qayta-qayta tahrirlab turamiz.
async fn run_animation(bot: Bot, chat_id: ChatId, name: String, token: CancellationToken) {
    let banner = ascii_banner(&format!("HAPPY BIRTHDAY {name}!"));

    // Ikkala tomondan bo'sh joy qo'shamiz — shunda banner oynaga to'liq
    // kirib, to'liq chiqib keta oladi (haqiqiy marquee effekti).
    let padded: Vec<String> = banner
        .iter()
        .map(|row| format!("{}{}{}", " ".repeat(VIEWPORT), row, " ".repeat(VIEWPORT)))
        .collect();

    let row_len = padded[0].len();
    let max_pos = row_len.saturating_sub(VIEWPORT);
    let center = max_pos / 2;

    let first = render_frame(
        &padded,
        Frame {
            pos: 0,
            style: 0,
            star: '★',
            star_bottom: '☆',
            corners: false,
            brightness: Brightness::Normal,
        },
    );
    let sent = match bot
        .send_message(chat_id, wrap_code(&first))
        .parse_mode(ParseMode::Html)
        .await
    {
        Ok(message) => message,
        Err(error) => {
            log::warn!("Tug'ilgan kun xabarini yuborib bo'lmadi: {error}");
            return;
        }
    };
    let message_id = sent.id;

    loop {
        if token.is_cancelled() {
            return;
        }

        // 1) Kirish: banner o'ngdan kirib, markazda to'xtaydi — silliq tezlashuv.
        for i in 0..=MOTION_FRAMES {
            let t = i as f32 / MOTION_FRAMES as f32;
            let pos = (ease_in_out(t) * center as f32).round() as usize;
            let frame = Frame {
                pos,
                style: i % 3,
                star: star_cycle(i),
                star_bottom: star_cycle(i + 1),
                corners: false,
                brightness: Brightness::Normal,
            };
            if !animate_edit(&bot, chat_id, message_id, &padded, frame, &token).await {
                return;
            }
        }

        // 2) Shimmer: yorug'lik to'lqini matn ustidan chapdan o'ngga suzib o'tadi.
        for i in 0..SHIMMER_FRAMES {
            let progress = i as f32 / (SHIMMER_FRAMES - 1) as f32;
            let wave = (progress * (VIEWPORT as f32 + 6.0)).round() as usize;
            let frame = Frame {
                pos: center,
                style: (i + 1) % 3,
                star: star_cycle(i + 1),
                star_bottom: star_cycle(i + 2),
                corners: false,
                brightness: Brightness::Band(wave),
            };
            if !animate_edit(&bot, chat_id, message_id, &padded, frame, &token).await {
                return;
            }
        }

        // 3) Uchqun/puls: burchaklar yonadi, matn porlaydi, ramka jonlanadi.
        for i in 0..SPARKLE_FRAMES {
            let brightness = if i % 2 == 0 {
                Brightness::Full
            } else {
                Brightness::Normal
            };
            let frame = Frame {
                pos: center,
                style: (i + 2) % 3,
                star: star_cycle(i + 2),
                star_bottom: '✧',
                corners: true,
                brightness,
            };
            if !animate_edit(&bot, chat_id, message_id, &padded, frame, &token).await {
                return;
            }
        }

        // 4) Chiqish: banner markazdan chapga chiqib ketadi — silliq sekinlashuv.
        for i in 0..=MOTION_FRAMES {
            let t = i as f32 / MOTION_FRAMES as f32;
            let travel = (ease_in_out(t) * (max_pos - center) as f32).round() as usize;
            let pos = center + travel;
            let frame = Frame {
                pos,
                style: (i + 3) % 3,
                star: star_cycle(i + 3),
                star_bottom: star_cycle(i + 4),
                corners: false,
                brightness: Brightness::Normal,
            };
            if !animate_edit(&bot, chat_id, message_id, &padded, frame, &token).await {
                return;
            }
        }
    }
}

/// Bitta kadrni tahrirlaydi va keyin `STEP_MS` kutadi.
///
/// Muvaffaqiyat bo'lsa `true`, xato yoki bekor qilingan bo'lsa `false`.
async fn animate_edit(
    bot: &Bot,
    chat_id: ChatId,
    message_id: MessageId,
    padded: &[String],
    frame: Frame,
    token: &CancellationToken,
) -> bool {
    if token.is_cancelled() {
        return false;
    }

    let text = wrap_code(&render_frame(padded, frame));
    let mut attempts = 0;

    loop {
        match bot
            .edit_message_text(chat_id, message_id, text.clone())
            .parse_mode(ParseMode::Html)
            .await
        {
            Ok(_) => {
                tokio::time::sleep(Duration::from_millis(STEP_MS)).await;
                return true;
            }
            // Bir xil kadr qayta yuborilsa Telegram xato qaytaradi — bu zararsiz.
            Err(RequestError::Api(ApiError::Unknown(message)))
                if message.contains("not modified") =>
            {
                tokio::time::sleep(Duration::from_millis(STEP_MS)).await;
                return true;
            }
            Err(_) => {
                attempts += 1;
                if attempts >= 3 {
                    return false;
                }
                tokio::time::sleep(Duration::from_millis(900)).await;
                if token.is_cancelled() {
                    return false;
                }
            }
        }
    }
}

/// Ramka uslubi uchun chegara belgilari: `(tepachap, gorizontal, tepa-o'ng,
/// vertikal, past-chap, past-o'ng)`.
fn border_for(style: usize) -> (char, char, char, char, char, char) {
    match style % 3 {
        0 => ('╔', '═', '╗', '║', '╚', '╝'),
        1 => ('┌', '─', '┐', '│', '└', '┘'),
        _ => ('╭', '─', '╮', '│', '╰', '╯'),
    }
}

/// Yulduz qatori belgilarining 4 xil sikli.
fn star_cycle(i: usize) -> char {
    ['★', '☆', '✦', '✧'][i % 4]
}

/// Kubik ease-in-out: harakat boshida va oxirida silliq sekinlashadi.
fn ease_in_out(t: f32) -> f32 {
    if t < 0.5 {
        4.0 * t * t * t
    } else {
        1.0 - (-2.0 * t + 2.0).powi(3) / 2.0
    }
}

/// `pos` o'rnidan boshlangan `VIEWPORT` kenglikdagi oynani
/// jonli ramka ichida chizadi.
fn render_frame(padded: &[String], frame: Frame) -> String {
    let (tl, h, tr, v, bl, br) = border_for(frame.style);
    let width = VIEWPORT;

    // Uchqun fazasida burchaklar porlaydi.
    let tl = if frame.corners { '✦' } else { tl };
    let tr = if frame.corners { '✧' } else { tr };
    let bl = if frame.corners { '✧' } else { bl };
    let br = if frame.corners { '✦' } else { br };

    let top = format!("{tl}{}{tr}", h.to_string().repeat(width));
    let bottom = format!("{bl}{}{br}", h.to_string().repeat(width));
    let stars_top = star_row(frame.star, width);
    let stars_bottom = star_row(frame.star_bottom, width);

    let mut lines = vec![top];
    lines.push(format!("{v}{stars_top}{v}"));
    for row in padded {
        let window: String = row.chars().skip(frame.pos).take(width).collect();
        let window = apply_brightness(&window, frame.brightness);
        lines.push(format!("{v}{window}{v}"));
    }
    lines.push(format!("{v}{stars_bottom}{v}"));
    lines.push(bottom);

    lines.join("\n")
}

/// Yorug'lik effektini qo'llaydi: to'lqin o'tgan ustunlardagi
/// `#` belgilarini yorug' `█` ga almashtiradi.
fn apply_brightness(window: &str, brightness: Brightness) -> String {
    match brightness {
        Brightness::Normal => window.to_string(),
        Brightness::Full => window.replace('#', "█"),
        Brightness::Band(wave) => {
            let mut result = String::with_capacity(window.len());
            for (col, ch) in window.chars().enumerate() {
                let dist = (col as isize - wave as isize).unsigned_abs();
                if ch == '#' && dist <= 4 {
                    result.push('█');
                } else {
                    result.push(ch);
                }
            }
            result
        }
    }
}

/// `★   ★   ★   ...` naqshini berilgan kenglikka moslab qaytaradi.
///
/// Yulduzlar kamroq bo'lishi uchun har 4 ustunda bittadan: 32 kenglikda 8 ta yulduz.
fn star_row(star: char, width: usize) -> String {
    let pattern = format!("{star}   ");
    let pattern_len = pattern.chars().count();
    pattern.repeat(width / pattern_len + 1).chars().take(width).collect()
}

/// Monospace ko'rinishi uchun HTML `<code>` bloki.
fn wrap_code(text: &str) -> String {
    format!("<code>{text}</code>")
}

/// Matnni 5 qatorli katta ASCII harflardan yasalgan bannerga aylantiradi.
fn ascii_banner(text: &str) -> Vec<String> {
    let mut rows = vec![String::new(); 5];
    for ch in text.chars() {
        let glyph = glyph(ch);
        for (i, line) in glyph.iter().enumerate() {
            if !rows[i].is_empty() {
                rows[i].push(' ');
            }
            rows[i].push_str(line);
        }
    }
    rows
}

/// 5x5 ASCII shrift. Noma'lum belgilar '?' ko'rinishida chiqadi.
fn glyph(ch: char) -> &'static [&'static str; 5] {
    match ch.to_ascii_uppercase() {
        'A' => &[" ### ", "#   #", "#####", "#   #", "#   #"],
        'B' => &["#### ", "#   #", "#### ", "#   #", "#### "],
        'C' => &[" ### ", "#    ", "#    ", "#    ", " ### "],
        'D' => &["#### ", "#   #", "#   #", "#   #", "#### "],
        'E' => &["#####", "#    ", "#### ", "#    ", "#####"],
        'F' => &["#####", "#    ", "#### ", "#    ", "#    "],
        'G' => &[" ### ", "#    ", "#  ##", "#   #", " ### "],
        'H' => &["#   #", "#   #", "#####", "#   #", "#   #"],
        'I' => &["#####", "  #  ", "  #  ", "  #  ", "#####"],
        'J' => &[" ### ", "   # ", "   # ", "#  # ", " ##  "],
        'K' => &["#   #", "#  # ", "###  ", "#  # ", "#   #"],
        'L' => &["#    ", "#    ", "#    ", "#    ", "#####"],
        'M' => &["#   #", "## ##", "# # #", "#   #", "#   #"],
        'N' => &["#   #", "##  #", "# # #", "#  ##", "#   #"],
        'O' => &[" ### ", "#   #", "#   #", "#   #", " ### "],
        'P' => &["#### ", "#   #", "#### ", "#    ", "#    "],
        'Q' => &[" ### ", "#   #", "#   #", "#  # ", " ## #"],
        'R' => &["#### ", "#   #", "#### ", "#  # ", "#   #"],
        'S' => &[" ####", "#    ", " ### ", "    #", "#### "],
        'T' => &["#####", "  #  ", "  #  ", "  #  ", "  #  "],
        'U' => &["#   #", "#   #", "#   #", "#   #", " ### "],
        'V' => &["#   #", "#   #", "#   #", " # # ", "  #  "],
        'W' => &["#   #", "#   #", "# # #", "## ##", "#   #"],
        'X' => &["#   #", " # # ", "  #  ", " # # ", "#   #"],
        'Y' => &["#   #", " # # ", "  #  ", "  #  ", "  #  "],
        'Z' => &["#####", "   # ", "  #  ", " #   ", "#####"],
        '0' => &[" ### ", "#   #", "#   #", "#   #", " ### "],
        '1' => &["  #  ", " ##  ", "  #  ", "  #  ", "#####"],
        '2' => &[" ### ", "#   #", "   # ", "  #  ", "#####"],
        '3' => &["#####", "   # ", " ### ", "   # ", "#####"],
        '4' => &["   # ", "  ## ", " # # ", "#####", "   # "],
        '5' => &["#####", "#    ", "#### ", "    #", "#### "],
        '6' => &[" ### ", "#    ", "#### ", "#   #", " ### "],
        '7' => &["#####", "    #", "   # ", "  #  ", "  #  "],
        '8' => &[" ### ", "#   #", " ### ", "#   #", " ### "],
        '9' => &[" ### ", "#   #", " ####", "    #", " ### "],
        ' ' => &["     "; 5],
        '!' => &["  #  ", "  #  ", "  #  ", "     ", "  #  "],
        '?' => &[" ### ", "#   #", "  #  ", "     ", "  #  "],
        '\'' => &["  #  ", "  #  ", "     ", "     ", "     "],
        '-' => &["     ", "     ", " ### ", "     ", "     "],
        '.' => &["     ", "     ", "     ", "     ", "  #  "],
        ',' => &["     ", "     ", "     ", "  #  ", " #   "],
        _ => &[" ### ", "#   #", "  #  ", "     ", "  #  "],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Yulduz qatorlari shu qancha kenglikda, shu qancha yulduzdan iborat bo'lishi
    /// kerak — har 4 ustunda bittadan (kamroq yulduz).
    #[test]
    fn star_rows_are_sparser() {
        for star in ['★', '☆', '✦', '✧'] {
            let row = star_row(star, VIEWPORT);
            assert_eq!(row.chars().count(), VIEWPORT);
            assert_eq!(row.matches(star).count(), VIEWPORT / 4);
        }
    }

    /// `/birthday_for CLAY` uchun kadrning ko'rinishi (`cargo test -- --nocapture`).
    #[test]
    fn frame_preview() {
        let banner = ascii_banner("HAPPY BIRTHDAY CLAY!");
        let padded: Vec<String> = banner
            .iter()
            .map(|r| format!("{}{}{}", " ".repeat(VIEWPORT), r, " ".repeat(VIEWPORT)))
            .collect();
        let center = (padded[0].chars().count() - VIEWPORT) / 2;

        let frame = render_frame(
            &padded,
            Frame {
                pos: center,
                style: 0,
                star: '★',
                star_bottom: '☆',
                corners: false,
                brightness: Brightness::Normal,
            },
        );
        println!("{frame}");
    }
}
