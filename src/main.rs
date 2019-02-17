#![deny(clippy::all)]
use crate::{
    client::{Client, Config},
    snake::Snake,
};
use clap::{app_from_crate, crate_authors, crate_description, crate_name, crate_version, Arg};
use env_logger::Builder;
use log::{info, LevelFilter};
use std::path::Path;
mod client;
mod snake;
mod types;
mod utils;

const CONFIG_FILE: &str = "snake.conf";
const DEFAULT_HOST: &str = "snake.cygni.se";
const DEFAULT_PORT: &str = "80";
const DEFAULT_SNAKE_NAME: &str = "default-rust-snake-name";
const DEFAULT_VENUE: &str = "training";

fn read_config() -> Config {
    info!("Reading config from file at {:?}", Path::new(CONFIG_FILE).canonicalize());
    let matches = app_from_crate!()
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
    Builder::from_default_env().filter_module(crate_name!(), LevelFilter::Info).init();

    let config = read_config();
    Client::connect(config, Snake::new).unwrap();
}
