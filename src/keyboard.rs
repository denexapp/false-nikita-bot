use teloxide::types::{KeyboardButton, KeyboardMarkup};

pub const VIDEO_JOKE_BUTTON_TEXT: &str = "Никита, расскажи анекдот";

pub fn make_keyboard() -> KeyboardMarkup {
    let video_joke = KeyboardButton::new(VIDEO_JOKE_BUTTON_TEXT);
    let keyboard: Vec<Vec<KeyboardButton>> = vec![vec![video_joke]];
    KeyboardMarkup::new(keyboard)
        .resize_keyboard(true)
        .persistent()
        .input_field_placeholder(String::from("Анеки от Никиты"))
}
