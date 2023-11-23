use teloxide::{Bot, dptree};
use teloxide::dispatching::Dispatcher;

mod config;
mod telegram;
mod github;

#[tokio::main]
async fn main() {
    config::load("config.toml");

    let bot = Bot::new(config::get_telegram_token());
 //   Dispatcher::builder(bot, schema()).dependencies(dptree::deps![InMemStorage::<telegram::State>::new()]).enable_ctrlc_handler().build().dispatch().await;
}
