# SNAKE CLIENT

Do you want the most annoying compiler ever?
Do you want to constantly think of what is owning what variable?
Do you want to stare angrily at the screen and wonder what the hell it means that some dumb value can't be moved?
Then here is the ultimate snake client for you, written for the beautiful language Rust.

## Requirements

* Rust nightly (use [rustup](https://github.com/rust-lang-nursery/rustup.rs))
* Snake server (local or remote)

## Setup

A. Clone the repository: `git clone https://github.com/cygni/snakebot-client-rust.git`;

B. Open the repo: `cd snakebot-client-rust`;

C. Default to nightly rust for the repo: `rustup override set nightly-2016-08-27`;

D. Run the snake: `cargo run`;

E. Improve the snake: edit `src/snake.rs`, and more specifically `get_next_move`.
