use std::net::SocketAddr;

use teloxide::{prelude::*, update_listeners::webhooks};

use crate::{
    cloud_run::get_project_id,
    database::get_database,
    logging::log_incoming_message,
    parse_message::{parse_message, MessageKind},
    replies::{
        send_random_video_joke, send_unknown_command_warning, send_video_note_back_with_file_id,
    },
    webhook::get_webhook_url,
};

mod cloud_run;
mod database;
mod keyboard;
mod logging;
mod parse_message;
mod replies;
mod webhook;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting false-nikita-bot...");

    let client = reqwest::Client::new();
    let project_id = get_project_id(&client).await;
    let database = get_database(&project_id).await;
    let url = get_webhook_url(&client, &project_id).await;
    let bot = Bot::from_env();
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));

    let listener = webhooks::axum(bot.clone(), webhooks::Options::new(addr, url))
        .await
        .expect("Couldn't setup webhook");

    teloxide::repl_with_listener(
        bot,
        move |bot: Bot, message: Message| {
            let database = database.clone();
            async move {
                let chat_id = message.chat.id;
                let message_kind = parse_message(&message);
                log_incoming_message(&message, &message_kind);
                match message_kind {
                    MessageKind::VideoJokeButton => {
                        send_random_video_joke(&bot, &chat_id, &database).await?
                    }
                    MessageKind::VideoNote(video_note) => {
                        send_video_note_back_with_file_id(&bot, &chat_id, video_note).await?
                    }
                    MessageKind::UnknownMessageWithText(_) | MessageKind::UnknownMessage => {
                        send_unknown_command_warning(&bot, &chat_id).await?
                    }
                }
                Ok(())
            }
        },
        listener,
    )
    .await;
}
