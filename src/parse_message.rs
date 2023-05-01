use teloxide::types::{Message, VideoNote};

use crate::keyboard::{TEXT_MESSAGE_BUTTON_TEXT, VIDEO_NOTE_BUTTON_TEXT};

// I don't know what a lifetime is tbh
pub enum MessageKind<'a> {
    VideoNote(&'a VideoNote),
    TextMessageButton,
    VideoNoteButton,
    UnknownMessageWithText(String),
    UnknownMessage,
}

pub fn parse_message(message: &Message) -> MessageKind {
    if let Some(video_note) = message.video_note() {
        MessageKind::VideoNote(video_note)
    } else if let Some(text) = message.text() {
        match text {
            TEXT_MESSAGE_BUTTON_TEXT => MessageKind::TextMessageButton,
            VIDEO_NOTE_BUTTON_TEXT => MessageKind::VideoNoteButton,
            _ => MessageKind::UnknownMessageWithText(text.to_string()),
        }
    } else {
        MessageKind::UnknownMessage
    }
}
