#[macro_use]
extern crate serde_derive;

extern crate serde_json;
extern crate reqwest;
extern crate chrono;

mod matrix_client;
mod matrix_bot;

use matrix_bot::*;


fn main() {
    let mut bot = MatrixBot::new(
            String::from("https://example.com"),
            String::from("username"),
            String::from("password")
            );
    bot.run();
}


