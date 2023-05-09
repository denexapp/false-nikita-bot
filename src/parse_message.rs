use teloxide::types::{Message, VideoNote};

use crate::keyboard::VIDEO_JOKE_BUTTON_TEXT;

// I don't know what a lifetime is tbh
pub enum MessageKind<'a> {
    VideoNote(&'a VideoNote),
    VideoJokeButton,
    UnknownMessageWithText(String),
    UnknownMessage,
}

pub fn parse_message(message: &Message) -> MessageKind {
    if let Some(video_note) = message.video_note() {
        MessageKind::VideoNote(video_note)
    } else if let Some(text) = message.text() {
        match text {
            VIDEO_JOKE_BUTTON_TEXT => MessageKind::VideoJokeButton,
            _ => MessageKind::UnknownMessageWithText(text.to_string()),
        }
    } else {
        MessageKind::UnknownMessage
    }
}
