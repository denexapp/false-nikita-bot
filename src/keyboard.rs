use teloxide::types::{KeyboardButton, KeyboardMarkup};

pub const VIDEO_NOTE_BUTTON_TEXT: &str = "Кружочком";
pub const TEXT_MESSAGE_BUTTON_TEXT: &str = "Текстом";

pub fn make_keyboard() -> KeyboardMarkup {
    let video_note = KeyboardButton::new(VIDEO_NOTE_BUTTON_TEXT);
    let text_message = KeyboardButton::new(TEXT_MESSAGE_BUTTON_TEXT);
    let keyboard: Vec<Vec<KeyboardButton>> = vec![vec![video_note, text_message]];
    KeyboardMarkup::new(keyboard)
        .resize_keyboard(true)
        .persistent()
        .input_field_placeholder(String::from("Анеки от Никиты"))
}
