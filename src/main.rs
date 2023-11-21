use config::load;

mod config;
fn main() {
    load("config.toml");
}