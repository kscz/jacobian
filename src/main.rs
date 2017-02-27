#[macro_use]
extern crate serde_derive;

extern crate serde_json;
extern crate reqwest;

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

    println!("Logged in! Got: {:#?}", login);

    let pub_rooms = match client.list_public_rooms() {
        Ok(x) => x,
        Err(e) => {
            println!("Failed to get publicly visible rooms!");
            println!("{:?}", e);
            return;
        }
    };

    println!("Publicly visible rooms:");
    println!("{:#?}", pub_rooms);

    for chunk in pub_rooms.chunk.iter() {
        if chunk.name == "Room I Want to Join" {
            println!("Found room! Attempting to join...");
            match client.join_room(&chunk.room_id) {
                Ok(x) => {
                    println!("Joined room with room_id: {:#?}", x);
                },
                Err(e) => {
                    println!("Unable to join room with error: {:#?}", e);
                }
            };
        }
    }

    println!("Attempting a sync!");
    match client.sync(None, None, Some(false), 30000) {
        Ok(_) => (),
        Err(e) => println!("{:#?}", e)
    };

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
