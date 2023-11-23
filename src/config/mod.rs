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
    organization: String,
    repository: String,
    elements_per_page: u8,
    approve_label: String,
    reject_label: String,
    filter_label: String,
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
            token: String::new(),
            organization: String::new(),
            repository: String::new(),
            elements_per_page: 0,
            approve_label: String::new(),
            reject_label: String::new(),
            filter_label: String::new(),
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


pub fn get_github_token() -> String { APP_CONFIG.lock().unwrap().deref().clone().github.token }

pub fn get_github_repository() -> String { APP_CONFIG.lock().unwrap().deref().clone().github.repository }

pub fn get_github_organization() -> String { APP_CONFIG.lock().unwrap().deref().clone().github.organization }

pub fn get_github_elements_per_page() -> u8 { APP_CONFIG.lock().unwrap().deref().clone().github.elements_per_page }

pub fn get_github_approve_label() -> String { APP_CONFIG.lock().unwrap().deref().clone().github.approve_label }

pub fn get_github_reject_label() -> String { APP_CONFIG.lock().unwrap().deref().clone().github.reject_label }

pub fn get_github_filter_label() -> String { APP_CONFIG.lock().unwrap().deref().clone().github.filter_label }

pub fn get_telegram_token() -> String {
    APP_CONFIG.lock().unwrap().deref().clone().telegram.token
}
