use teloxide::prelude::{Message, ResponseResult};
use teloxide::Bot;
use teloxide::{prelude::*, utils::command::BotCommands};

#[derive(BotCommands, Clone)]
#[command(
    rename_rule = "lowercase",
    description = "These commands are supported:"
)]
pub enum Command {
    #[command(description = "display this text.")]
    Help,
    #[command(description = "rolls dice.")]
    Dice,
}

pub async fn answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    match cmd {
        Command::Help => {
            let help_response = format!("{}\n\n{}", Command::descriptions().to_string(), "PLEASE REMAIN CALM.");

            bot.send_message(msg.chat.id, help_response)
                .await?
        }
        Command::Dice => {
            bot.send_dice(msg.chat.id).await?
        }
    };

    respond(())
}
