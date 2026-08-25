//! Tug'ilgan kun tabrigi xizmati.
//!
//! `/birthday_for Ism` buyrug'i berilgan chatda animatsion ASCII harflardan
//! yasalgan, HTML marquee oynasiga o'xshab harakatlanuvchi tabrikni ishga
//! tushiradi va `/stopbirthday` aytilgunicha (yoki chatdagi boshqa
//! `/birthday_for` boshlanguncha) davom ettiradi.

use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Duration;

use teloxide::prelude::*;
use teloxide::types::{MessageId, ParseMode};
use teloxide::{ApiError, RequestError};
use tokio_util::sync::CancellationToken;

use crate::app::{AppState, BirthdayHandle};

/// Har bir boshlangan tabrik uchun noyob ID (eski tabrikni o'chirishda ishlatiladi).
static BIRTHDAY_SEQ: AtomicU64 = AtomicU64::new(1);

/// Marquee oynasi kengligi — banner shu kenglikdagi oyna ichida harakatlanadi.
const VIEWPORT: usize = 28;

/// Ikki kadr orasidagi pauza (millisekundlarda).
const STEP_MS: u64 = 80;

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
/// marquee kabi qayta-qayta tahrirlab turamiz.
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
    let centered = max_pos / 2;

    let first = render_frame(&padded, 0, '★');
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

        // 1) O'ngdan chapga — klassik HTML marquee yo'nalishi.
        for pos in 0..=max_pos {
            if !edit_frame(&bot, chat_id, message_id, &padded, pos, '★', &token).await {
                return;
            }
        }

        // 2) "Blink" — yulduzlar miltillaydi, banner markazda turadi.
        for _ in 0..3 {
            if token.is_cancelled() {
                return;
            }
            for (star, shift) in [('★', 0usize), ('☆', 1), ('★', 0), ('☆', 1)] {
                if !edit_frame(&bot, chat_id, message_id, &padded, centered + shift, star, &token)
                    .await
                {
                    return;
                }
            }
        }

        // 3) Chapdan o'ngga — teskari yo'nalish.
        for pos in (0..=max_pos).rev() {
            if !edit_frame(&bot, chat_id, message_id, &padded, pos, '★', &token).await {
                return;
            }
        }
    }
}

/// Bitta kadrni tahrirlaydi. Muvaffaqiyat bo'lsa `true`, aniqlik uchun
/// `false` qaytaradi (xato yoki bekor qilingan).
async fn edit_frame(
    bot: &Bot,
    chat_id: ChatId,
    message_id: MessageId,
    padded: &[String],
    pos: usize,
    star: char,
    token: &CancellationToken,
) -> bool {
    if token.is_cancelled() {
        return false;
    }

    let frame = render_frame(padded, pos, star);
    let mut attempts = 0;

    loop {
        match bot
            .edit_message_text(chat_id, message_id, wrap_code(&frame))
            .parse_mode(ParseMode::Html)
            .await
        {
            Ok(_) => return true,
            // Bir xil kadr qayta yuborilsa Telegram xato qaytaradi — bu zararsiz.
            Err(RequestError::Api(ApiError::Unknown(message)))
                if message.contains("not modified") =>
            {
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

/// `pos` o'rnidan boshlangan `VIEWPORT` kenglikdagi oynani
/// bezakli ramka ichida chizadi.
fn render_frame(padded: &[String], pos: usize, star: char) -> String {
    let top = format!("╔{}╗", "═".repeat(VIEWPORT));
    let bottom = format!("╚{}╝", "═".repeat(VIEWPORT));
    let stars = star_row(star, VIEWPORT);

    let mut lines = vec![top];
    lines.push(format!("║{stars}║"));
    for row in padded {
        let window: String = row.chars().skip(pos).take(VIEWPORT).collect();
        lines.push(format!("║{window}║"));
    }
    lines.push(format!("║{stars}║"));
    lines.push(bottom);

    lines.join("\n")
}

/// `★ ★ ★ ★ ...` naqshini berilgan kenglikka moslab qaytaradi.
fn star_row(star: char, width: usize) -> String {
    let pattern = format!("{star} ");
    pattern.repeat(width).chars().take(width).collect()
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
