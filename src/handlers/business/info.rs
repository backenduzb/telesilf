use crate::utils::greeting::is_portfolio_request;
use crate::utils::message::stream_bs_text;
use teloxide::RequestError;
use teloxide::prelude::*;
use teloxide::types::{MessageKind, ParseMode, UserId};
use teloxide::utils::markdown::{self, escape, bold, link};

pub async fn info_getter(
    bot: &Bot,
    msg: &Message,
) -> Result<(), RequestError> {
    if let MessageKind::Common(ref common) = msg.kind {
        if let Some(biz_id) = &common.business_connection_id {
            if let Some(text) = msg.text() {
                if let Some(user) = &msg.from {
                    if user.id == UserId(6400925437) {
                        return Ok(());
                    }
                }

                if is_portfolio_request(text) {
                    let name = msg
                        .from
                        .as_ref()
                        .map(|u| u.first_name.as_str())
                        .unwrap_or("do'stim");

                    let mut req = bot.send_message(msg.chat.id, "...");
                    req = req.business_connection_id(biz_id.clone());

                    let sent = req.await?;

                    stream_bs_text(
                        &bot,
                        msg.chat.id,
                        sent.id,
                        format!(
                            "{} hozir sizga portfolio malumotlarini tashlabberaman lekin online bolganda Javohirni o'zidan ham albatta so'rang!",
                            name
                        ),
                        Some(biz_id.0.as_str()),
                    )
                    .await?;
                    let link_portfolio = link("https://uzbekdew.vercel.app", "Portfolio©");
                        let link_github = link("https://github.com/backenduzb", "GitHub©");
                        let link_telegram = link("https://t.me/python_dev_junior", "Telegram©");
                    
                        let title = bold("Portfolio");
                        let section_links = bold("Havolalar");
                        let section_tech = bold("Yoqtiradigan dasturlash tillari va texnologiyalari");
                        let section_fields = bold("Bemalol ishlay oladigan sohalari:");
                    
                        let divider = escape("───────────────");
                        let quote_text = escape("Asosan optimizatsiyaga mukkasidan ketgan 😅");
                        let footer_text = escape("Men ham bu loyihalar qatorida borman 😆");
                        let apostrophe = escape("'"); 
                    
                        
                    bot.send_message(
                        msg.chat.id,
                        format!(
                            "{title}\n\
                            {divider}\n\n\
                            {section_links}\n\
                            • ✮ {link_portfolio}\n\
                            • ✮ {link_github}\n\
                            • ✮ {link_telegram}\n\n\
                            {section_tech}\n\
                            • ★ Rust\n\
                            • ★ Go\n\
                            • ✿ Python\n\
                            • ✦ TypeScript\n\
                            • ✦ Docker\n\
                            • ✦ Linux\n\n\
                            >{quote_text}\n\n\
                            {section_fields}\n\
                            • Telegram bot\n\
                            • Fullstack development\n\
                            • Desktop & mobile ilovalar\n\
                            • Web ilovalar\n\
                            • 2D/3D O{apostrophe}yinlar\n\n\
                            {divider}\n\
                            {footer_text}"
                        ),
                    )
                    .business_connection_id(biz_id.clone())
                    .parse_mode(ParseMode::MarkdownV2)
                    .await?;
                }
            }
        }
    } else {
    }

    Ok(())
}
