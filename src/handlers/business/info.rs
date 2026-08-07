use crate::utils::greeting::is_portfolio_request;
use crate::utils::message::stream_bs_text;
use teloxide::RequestError;
use teloxide::prelude::*;
use teloxide::types::{MessageKind, ParseMode, UserId};

pub async fn info_getter(
    bot: Bot,
    msg: Message,
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
                    bot.send_message(
                        msg.chat.id,
                        r#"
                    <h2>Portfolio</h2>
                    <hr/>

                    <h3>Havolalar</h3>

                    <ul>
                    <li>✮ <a href="https://uzbekdew.vercel.app">Portfolio©</a></li>
                    <li>✮ <a href="https://github.com/backenduzb">GitHub©</a></li>
                    <li>✮ <a href="https://t.me/python_dev_junior">Instagram©</a></li>
                    </ul>

                    <h3>Yoqtiradigan dasturlash tillari va teznologiyalari</h3>

                    <ul>
                    <li>★ Rust</li>
                    <li>★ Go</li>
                    <li>✿ Python</li>
                    <li>✦ TypeScritp</li>
                    <li>✦ Docker</li>
                    <li>✦ Linux</li>
                    </ul>

                    <blockquote>
                    Asosan optimizatsiyaga mukkasidan ketgan 😅
                    <cite>Mini info</cite>
                    </blockquote>

                    <details>
                    <summary>Bemalol ishlay oladigan sohalari</summary>

                    • Telegram bot

                    • Fullstack developing

                    • Desktop&mobile ilovalar

                    • Web ilovalar

                    • 2D/3D O'yinlar

                    </details>

                    <hr/>

                    <p>Men ham bu loyihalar qatorida borman 😆</p>"#,
                    )
                    .parse_mode(ParseMode::Html)
                    .await?;
                }
            }
        }
    } else {
    }

    Ok(())
}
