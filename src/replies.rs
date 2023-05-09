use firestore::FirestoreDb;
use rand::{rngs::OsRng, seq::SliceRandom};
use teloxide::{
    prelude::*,
    types::{InputFile, VideoNote},
    RequestError,
};

use crate::{
    database::get_video_jokes,
    keyboard::make_keyboard,
};

pub async fn send_video_note_back_with_file_id(
    bot: &Bot,
    chat_id: &ChatId,
    video_note: &VideoNote,
) -> Result<(), RequestError> {
    let id = &video_note.file.id;
    let video_note = InputFile::file_id(id);
    bot.send_message(*chat_id, format!("Video note file id is {}", id))
        .reply_markup(make_keyboard())
        .await?;
    bot.send_video_note(*chat_id, video_note)
        .reply_markup(make_keyboard())
        .await?;
    Ok(())
}

pub async fn send_random_video_joke(
    bot: &Bot,
    chat_id: &ChatId,
    database: &FirestoreDb,
) -> Result<(), RequestError> {
    if let Ok(video_jokes) = get_video_jokes(database).await {
        let mut rng = OsRng::default();
        if let Some(random_video_joke) = video_jokes.choose(&mut rng) {
            for file_id in &random_video_joke.file_ids {
                let video_note = InputFile::file_id(file_id);
                bot.send_video_note(*chat_id, video_note).await?;
            }
        } else {
            bot.send_message(*chat_id, "There's no video notes in the database")
                .await?;
        }
    } else {
        bot.send_message(*chat_id, "Database request Error")
            .reply_markup(make_keyboard())
            .await?;
    }
    Ok(())
}

pub async fn send_unknown_command_warning(bot: &Bot, chat_id: &ChatId) -> Result<(), RequestError> {
    bot.send_message(*chat_id, "Нажми на кнопку")
        .reply_markup(make_keyboard())
        .await?;
    Ok(())
}
