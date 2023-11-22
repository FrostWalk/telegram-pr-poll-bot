use teloxide::{prelude::*, utils::command::BotCommands};
use teloxide::{
    prelude::*,
};

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "These commands are supported:")]
pub enum Command {
    #[command(description = "display this text.")]
    Help,
    #[command(description = "Creates a poll about a pull request")]
    PullRequest,
    #[command(description = "Creates a poll about an issue")]
    Issue,
}


#[derive(Clone)]
pub enum State {
    ChoosePr,
    ChooseIssue,
}

pub async fn answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    match cmd {
        Command::Help => bot.send_message(msg.chat.id, Command::descriptions().to_string()).await?,
        Command::Issue => {
            bot.delete_message(msg.chat.id, msg.id).await?;
            bot.send_message(msg.chat.id, "").await?
        }
        Command::PullRequest => {
            bot.delete_message(msg.chat.id, msg.id).await?;
            bot.send_message(msg.chat.id, "").await?
        }
    };

    Ok(())
}
