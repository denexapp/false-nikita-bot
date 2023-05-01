use log::info;
use teloxide::types::Message;

use crate::parse_message::MessageKind;

fn get_username(message: &Message) -> String {
    if let Some(user) = message.from() {
        if let Some(username) = &user.username {
            format!("{} (@{})", user.full_name(), username)
        } else {
            user.full_name()
        }
    } else {
        "Неизвестный пользователь".to_string()
    }
}

pub fn log_incoming_message(message: &Message, message_kind: &MessageKind) {
    let username = get_username(&message);
    let action = match message_kind {
        MessageKind::VideoNote(video_note) => format!(
            "sent a video note with the following file id: {}",
            video_note.file.id
        ),
        MessageKind::TextMessageButton => "pressed the text message button".to_string(),
        MessageKind::VideoNoteButton => "pressed the video note button".to_string(),
        MessageKind::UnknownMessageWithText(text) => {
            format!("sent an unknown message with the following text: {}", text)
        }
        MessageKind::UnknownMessage => "sent an unknown message".to_string(),
    };
    info!("{} {}", username, action);
}
