#[macro_use]
extern crate serde_derive;

extern crate chrono;
extern crate clap;
extern crate reqwest;
extern crate serde_json;

mod matrix_bot;
mod matrix_client;

use matrix_bot::*;
use clap::{Arg, App};

fn main() {
    let matches = App::new("Jacobian")
                          .version("0.1")
                          .author("Kit Sczudlo <kit@kitkorp.com>")
                          .about("A bot for Matrix written in Rust")
                          .arg(Arg::with_name("username")
                               .short("u")
                               .long("username")
                               .required(true)
                               .help("The username to login to the server with")
                               .takes_value(true))
                          .arg(Arg::with_name("password")
                               .short("p")
                               .long("password")
                               .required(true)
                               .help("The password to login to the server with")
                               .takes_value(true))
                          .arg(Arg::with_name("server")
                               .short("s")
                               .long("server")
                               .required(true)
                               .help("The server to login to")
                               .takes_value(true))
                          .get_matches();

    let mut bot = MatrixBot::new(
            matches.value_of("server").unwrap(),
            matches.value_of("username").unwrap(),
            matches.value_of("password").unwrap());
    bot.run();
}


