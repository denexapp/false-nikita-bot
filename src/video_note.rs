use crate::keyboard::make_keyboard;
use teloxide::{
    prelude::*,
    types::{InputFile, VideoNote},
};

pub async fn send_video_note_back_with_file_id(
    bot: &Bot,
    message: &Message,
    video_note: &VideoNote,
) -> Result<(), teloxide::errors::RequestError> {
    let id = &video_note.file.id;
    let video_note = InputFile::file_id(id);
    bot.send_message(message.chat.id, format!("Video note file id is {}", id))
        .reply_markup(make_keyboard())
        .await?;
    bot.send_video_note(message.chat.id, video_note)
        .reply_markup(make_keyboard())
        .await?;
    Ok(())
}
