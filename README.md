# SNAKE CLIENT

[![Build Status](http://jenkins.snake.cygni.se/buildStatus/icon?job=snake client rust)](http://jenkins.snake.cygni.se/job/snake%20client%20rust/)

Do you want the most annoying compiler ever?
Do you want to constantly think of what is owning what variable?
Do you want to stare angrily at the screen and wonder what the hell it means that some dumb value can't be moved?
Then here is the ultimate snake client for you, written for the beautiful language Rust.

## Requirements

- Rust (which should be installed via [rustup](https://github.com/rust-lang-nursery/rustup.rs))
- Snake server (local or remote)

## Setup

A. Clone the repository: `git clone https://github.com/cygni/snakebot-client-rust.git`

B. Open the repo: `cd snakebot-client-rust`

C. Run the snake: `cargo run`

D. Improve the snake: edit `src/snake.rs`, and more specifically `get_next_move`.
