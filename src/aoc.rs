use reqwest::header::COOKIE;
use reqwest::header::USER_AGENT;
use serde::Deserialize;
use std::fs;
use std::path::Path;
use std::path::PathBuf;

#[derive(Deserialize)]
struct Config {
    token: String,
    storage: Option<String>,
}

fn read_config(raw: &str) -> Result<Config, &'static str> {
    match toml::from_str::<Config>(raw) {
        Ok(value) => return Ok(value),
        Err(_) => return Err("Failed parsing."),
    }
}

fn get_config() -> Result<Config, &'static str> {
    let contents = fs::read_to_string("settings.toml");
    match contents {
        Ok(value) => return read_config(&value),
        Err(_) => return Err("Failed reading!"),
    };
}

fn get_data(url: String) -> Result<String, &'static str> {
    let aoc_user_agent = "bianchignocchi";

    let config = get_config();

    let client = reqwest::blocking::Client::new();
    let formated_token = format!("session={}", config?.token);

    let resp = client
        .get(url)
        .header(USER_AGENT, aoc_user_agent)
        .header(COOKIE, formated_token)
        .send();

    let text = match resp {
        Ok(r) => match r.text() {
            Ok(t) => t,
            Err(_) => return Err("Couldn't get data"),
        },
        Err(_) => return Err("Couldn't get data"),
    };
    return Ok(text);
}

fn get_base_path() -> PathBuf {
    let config = get_config();
    let default = String::from(".storage");
    let base_path = match config {
        Ok(config) => config.storage.unwrap_or(default),
        Err(_) => default,
    };

    return Path::new(&base_path).to_path_buf();
}

fn store_file_path(day: u32) -> PathBuf {
    let file_name = format!("{day}.txt");
    let base_path = get_base_path();
    return base_path.join(file_name);
}

fn get_stored(day: u32) -> Option<String> {
    let file_path = store_file_path(day);

    match fs::read_to_string(file_path) {
        Ok(data) => Some(data),
        Err(_) => {
            println!("Cache miss");
            None
        }
    }
}

fn store(data: &String, day: u32) {
    let target_path = store_file_path(day);
    let _ = match target_path.parent().expect("No parent").to_str() {
        Some(parent) => fs::create_dir_all(parent),
        None => {
            println!("Couldn't create parent dir {:?}", target_path);
            return ();
        }
    };

    let path_str = target_path.clone();
    match fs::write(target_path, data) {
        Ok(_) => println!("Sucessfully stored to {:?}", path_str),
        Err(err) => println!("Failed to write file {:?} err: {:?}", path_str, err),
    }
}

pub fn get_puzzle(day: u32) -> String {
    let url = format!("https://adventofcode.com/2023/day/{day}/input");
    let raw = get_stored(day);
    if raw.is_some() {
        return raw.unwrap();
    }
    let raw = get_data(url);
    let result = match raw {
        Ok(t) => t,
        Err(err) => panic!("Couldn't get data for {:?}: {:?}", day, err),
    };
    store(&result, day);
    return result;
}
