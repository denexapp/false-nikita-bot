use std::net::SocketAddr;
use teloxide::{prelude::*, update_listeners::webhooks};

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
            bot.send_message(msg.chat.id, "pong").await?;
            Ok(())
        },
        listener,
    )
    .await;
}
