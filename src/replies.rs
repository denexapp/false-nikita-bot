use crate::{
    database::{get_text_messages, get_video_notes},
    keyboard::make_keyboard,
};
use firestore::FirestoreDb;
use teloxide::{
    prelude::*,
    types::{InputFile, VideoNote},
    RequestError,
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

pub async fn send_random_video_note(
    bot: &Bot,
    chat_id: &ChatId,
    database: &FirestoreDb,
) -> Result<(), RequestError> {
    if let Ok(result) = get_video_notes(database).await {
        let video_note = InputFile::file_id(&result[0].file_id);
        bot.send_video_note(*chat_id, video_note).await?;
    } else {
        bot.send_message(*chat_id, "Database request Error")
            .reply_markup(make_keyboard())
            .await?;
    }
    Ok(())
}

pub async fn send_random_text_message(
    bot: &Bot,
    chat_id: &ChatId,
    database: &FirestoreDb,
) -> Result<(), RequestError> {
    if let Ok(result) = get_text_messages(database).await {
        bot.send_message(*chat_id, &result[0].text).await?;
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
