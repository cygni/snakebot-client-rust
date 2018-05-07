#![allow(deprecated)]

#[macro_use]
extern crate clap;
extern crate config;
#[macro_use]
extern crate log;
extern crate log4rs;
extern crate rustc_version;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate target_info;
extern crate ws;

mod client;
mod snake;
mod types;
mod utils;

use clap::Arg;
use client::{Client, Config};
use snake::Snake;
use std::path::Path;

const LOG_TARGET: &'static str = "client";
const CONFIG_FILE: &'static str = "snake.conf";
const DEFAULT_HOST: &'static str = "snake.cygni.se";
const DEFAULT_PORT: &'static str = "80";
const DEFAULT_SNAKE_NAME: &'static str = "default-rust-snake-name";
const DEFAULT_VENUE: &'static str = "training";

fn read_config() -> Config {
    let config_path = Path::new(CONFIG_FILE);
    info!(
        target: LOG_TARGET,
        "Reading config from file at {:?}",
        config_path.canonicalize()
    );
    let matches = app_from_crate!(",\n")
        .arg(
            Arg::with_name("host")
                .short("h")
                .long("host")
                .help("The host to connect to")
                .takes_value(true)
                .default_value(DEFAULT_HOST),
        )
        .arg(
            Arg::with_name("port")
                .short("p")
                .long("port")
                .help("The port to connect to")
                .takes_value(true)
                .default_value(DEFAULT_PORT),
        )
        .arg(
            Arg::with_name("venue")
                .short("v")
                .long("venue")
                .help("The venue (tournament or training)")
                .takes_value(true)
                .default_value(DEFAULT_VENUE)
                .possible_values(&["tournament", "training"]),
        )
        .arg(
            Arg::with_name("snake-name")
                .short("n")
                .long("snake-name")
                .help("The name of the snake")
                .takes_value(true)
                .default_value(DEFAULT_SNAKE_NAME),
        )
        .get_matches();

    Config {
        host: matches.value_of("host").unwrap_or(DEFAULT_HOST).to_string(),
        port: matches.value_of("port").unwrap_or(DEFAULT_PORT).parse::<i32>().unwrap(),
        venue: matches.value_of("venue").unwrap_or(DEFAULT_VENUE).to_string(),
        snake_name: matches.value_of("snake-name").unwrap_or(DEFAULT_SNAKE_NAME).to_string(),
    }
}

fn main() {
    if let Err(_) = log4rs::init_file("log4rs.toml", Default::default()) {
        log4rs::init_file("../log4rs.toml", Default::default()).unwrap();
    }

    let config = read_config();

    Client::connect(config, Snake::new).unwrap();
}
