use firestore::{errors::FirestoreError, FirestoreDb};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use teloxide::{
    prelude::*,
    types::{InputFile, KeyboardButton, KeyboardMarkup, VideoNote},
    update_listeners::webhooks,
};
mod consts;

fn make_keyboard() -> KeyboardMarkup {
    let button = KeyboardButton::new("Нажми меня!");
    let keyboard: Vec<Vec<KeyboardButton>> = vec![vec![button]];
    KeyboardMarkup::new(keyboard)
        .resize_keyboard(true)
        .persistent()
        .input_field_placeholder(String::from("test"))
}

async fn send_video_note_back_with_file_id(
    bot: &Bot,
    message: &Message,
    video_note: &VideoNote,
) -> Result<(), teloxide::errors::RequestError> {
    let id = &video_note.file.id;
    let video_note = InputFile::file_id(id);
    bot.send_message(message.chat.id, format!("Video note file id is {}", id))
        .await?;
    bot.send_video_note(message.chat.id, video_note).await?;
    Ok(())
}

async fn get_db(client: &Client) -> FirestoreDb {
    let project_id = client
        .get("http://metadata.google.internal/computeMetadata/v1/project/project-id")
        .header("Metadata-Flavor", "Google")
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    FirestoreDb::new(project_id).await.unwrap()
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct TextMessage {
    text: String,
}

async fn get_text_messages(db: &FirestoreDb) -> Result<Vec<TextMessage>, FirestoreError> {
    Ok(db
        .fluent()
        .select()
        .from(consts::TEXT_MESSAGES_COLLECTION)
        .obj()
        .query()
        .await?)
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting false-nikita-bot...");

    let client = reqwest::Client::new();

    let db = get_db(&client).await;
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
