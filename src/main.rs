#![feature(proc_macro)]

#[macro_use]
extern crate serde_derive;

extern crate hyper;

mod matrix_client;

use matrix_client::*;

fn main() {
    let mut client = MatrixClient::new(String::from("https://example.com"), None);

    let supported_versions = match client.get_supported_versions() {
        Ok(x) => x,
        Err(e) => {
            println!("Got error: {:?}", e);
            return;
        }
    };

    print!("Supported versions: ");
    for ver in supported_versions.versions {
        print!("{}, ", ver);
    }
    println!("");

    let login = match client.login("username", "password") {
        Ok(x) => x,
        Err(e) => {
            println!("Failed to login!");
            println!("{:?}", e);
            return;
        }
    };

    println!("Logged in! Got: {:?}", login);

    println!("Attempting to log back out...");
    match client.logout() {
        Ok(_) => (),
        Err(e) => {
            println!("Failed to logout!");
            println!("{:?}", e);
            return;
        }
    };

    println!("Success!");
}
