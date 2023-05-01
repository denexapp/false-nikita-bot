use crate::{
    database::get_database,
    keyboard::{TEXT_MESSAGE_BUTTON_TEXT, VIDEO_NOTE_BUTTON_TEXT},
    replies::{
        send_random_text_message, send_random_video_note, send_unknown_command_warning,
        send_video_note_back_with_file_id,
    },
};
use std::net::SocketAddr;
use teloxide::{prelude::*, update_listeners::webhooks};
mod database;
mod keyboard;
mod replies;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting false-nikita-bot...");

    let client = reqwest::Client::new();
    let database = get_database(&client).await;
    let bot = Bot::from_env();
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));

    let url = "https://false-nikita-bot-kliudduhya-lm.a.run.app/webhook"
        .parse()
        .expect("Can't parse webhook callback url");

    let listener = webhooks::axum(bot.clone(), webhooks::Options::new(addr, url))
        .await
        .expect("Couldn't setup webhook");

    teloxide::repl_with_listener(
        bot,
        move |bot: Bot, msg: Message| {
            let database = database.clone();
            async move {
                let chat_id = msg.chat.id;
                if let Some(video_note) = msg.video_note() {
                    send_video_note_back_with_file_id(&bot, &chat_id, video_note).await?;
                } else if let Some(text) = msg.text() {
                    match text {
                        TEXT_MESSAGE_BUTTON_TEXT => {
                            send_random_text_message(&bot, &chat_id, &database).await?
                        }
                        VIDEO_NOTE_BUTTON_TEXT => {
                            send_random_video_note(&bot, &chat_id, &database).await?
                        }
                        _ => send_unknown_command_warning(&bot, &chat_id).await?,
                    };
                }
                Ok(())
            }
        },
        listener,
    )
    .await;
}
