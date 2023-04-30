use crate::{
    database::{get_database, get_text_messages},
    keyboard::make_keyboard,
    video_note::send_video_note_back_with_file_id,
};
use std::net::SocketAddr;
use teloxide::{prelude::*, update_listeners::webhooks};
mod consts;
mod database;
mod keyboard;
mod video_note;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting false-nikita-bot...");

    let client = reqwest::Client::new();

    let db = get_database(&client).await;
    let bot = Bot::from_env();

    let url = "https://false-nikita-bot-kliudduhya-lm.a.run.app/webhook"
        .parse()
        .unwrap();

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));

    let listener = webhooks::axum(bot.clone(), webhooks::Options::new(addr, url))
        .await
        .expect("Couldn't setup webhook");

    teloxide::repl_with_listener(
        bot,
        move |bot: Bot, msg: Message| {
            let db = db.clone();
            async move {
                if let Some(video_note) = msg.video_note() {
                    send_video_note_back_with_file_id(&bot, &msg, video_note).await?;
                } else {
                    if let Ok(result) = get_text_messages(&db.clone()).await {
                        bot.send_message(msg.chat.id, &result[0].text).await?;
                    } else {
                        bot.send_message(msg.chat.id, "Database request Error")
                            .reply_markup(make_keyboard())
                            .await?;
                        return Ok(());
                    }
                }
                Ok(())
            }
        },
        listener,
    )
    .await;
}
