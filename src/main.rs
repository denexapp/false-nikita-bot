use std::net::SocketAddr;
use teloxide::{
    prelude::*,
    types::{InputFile, KeyboardButton, KeyboardMarkup, VideoNote},
    update_listeners::webhooks,
};

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

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting false-nikita-bot...");

    let bot = Bot::from_env();

    let url = format!("https://false-nikita-bot-kliudduhya-lm.a.run.app/webhook")
        .parse()
        .unwrap();

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));

    let listener = webhooks::axum(bot.clone(), webhooks::Options::new(addr, url))
        .await
        .expect("Couldn't setup webhook");

    teloxide::repl_with_listener(
        bot,
        |bot: Bot, msg: Message| async move {
            if let Some(video_note) = msg.video_note() {
                send_video_note_back_with_file_id(&bot, &msg, video_note).await?;
            } else {
                bot.send_message(msg.chat.id, "pong")
                    .reply_markup(make_keyboard())
                    .await?;
            }
            Ok(())
        },
        listener,
    )
    .await;
}
