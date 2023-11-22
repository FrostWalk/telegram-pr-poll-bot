use teloxide::{
    prelude::*,
};

mod config;
mod telegram;

#[tokio::main]
async fn main() {
    config::load("config.toml");

    pretty_env_logger::init();
    log::info!("Starting command bot...");

    let bot = Bot::new(config::get_telegram_token());
    //Dispatcher::builder(bot, schema()).dependencies(dptree::deps![InMemStorage::<telegram::State>::new()]).enable_ctrlc_handler().build().dispatch().await;
}
