use teloxide::types::{KeyboardButton, KeyboardMarkup};

pub fn make_keyboard() -> KeyboardMarkup {
    let video_note = KeyboardButton::new("Кружочек");
    let text_message = KeyboardButton::new("Текст");
    let keyboard: Vec<Vec<KeyboardButton>> = vec![vec![video_note, text_message]];
    KeyboardMarkup::new(keyboard)
        .resize_keyboard(true)
        .persistent()
        .input_field_placeholder(String::from("Анеки от Никиты"))
}
