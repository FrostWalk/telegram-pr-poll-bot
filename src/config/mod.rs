use std::{fs, mem};
use std::ops::{Deref, DerefMut};
use std::sync::Mutex;

use lazy_static::lazy_static;
use serde::Deserialize;

#[derive(Deserialize)]
#[derive(Clone)]
struct Telegram {
    token: String,
}

#[derive(Deserialize)]
#[derive(Clone)]
struct Github {
    token: String,
}

#[derive(Deserialize)]
#[derive(Clone)]
struct Config {
    github: Github,
    telegram: Telegram,
}

impl Telegram {
    fn new() -> Telegram {
        Telegram {
            token: String::new()
        }
    }
}

impl Github {
    fn new() -> Github {
        Github {
            token: String::new()
        }
    }
}

impl Config {
    fn new() -> Config {
        Config {
            github: Github::new(),
            telegram: Telegram::new(),
        }
    }
}

lazy_static! {
    static ref APP_CONFIG: Mutex<Config> = Mutex::new(Config::new());
}

pub fn load(file_path: &str) {
    let content = match fs::read_to_string(file_path) {
        Ok(c) => c,
        Err(e) => {
            panic!("{}", e);
        }
    };

    let red: Config = match toml::from_str(&content) {
        Ok(c) => { c }
        Err(e) => {
            panic!("{}", e);
        }
    };

    let _ = mem::replace(APP_CONFIG.lock().unwrap().deref_mut(), red);
}
pub fn get_github_token() -> String {
    APP_CONFIG.lock().unwrap().deref().clone().github.token
}

pub fn get_telegram_token() -> String {
    APP_CONFIG.lock().unwrap().deref().clone().telegram.token
}
