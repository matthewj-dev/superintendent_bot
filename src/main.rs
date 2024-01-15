use teloxide::prelude::*;
mod commands;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting superintendent bot...");

    let bot = Bot::from_env();

    commands::main::Command::repl(bot, commands::main::answer).await;
}
