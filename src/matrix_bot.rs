use matrix_client::*;

pub struct MatrixBot {
    matrix_client: MatrixClient,
    username: String,
    password: String,
    should_quit: bool
}

impl MatrixBot {
    pub fn new(homeserver: String, username: String, password: String) -> MatrixBot {
        MatrixBot {
            matrix_client: MatrixClient::new(homeserver, None),
            username: username,
            password: password,
            should_quit: false
        }
    }

    pub fn run(&mut self) -> () {
        let login = match self.matrix_client.login(self.username.as_ref(), self.password.as_ref()) {
            Ok(x) => x,
            Err(e) => {
                println!("Failed to login!");
                println!("{:?}", e);
                return;
            }
        };

        println!("Logged in! Got: {:#?}", login);

        println!("Attempting initial sync!");
        let mut next_batch = None;
        loop {
            println!("Using batch {:?}", next_batch);
            let sync_response = match self.matrix_client.sync(None, next_batch.as_ref(), Some(false), Some(30000)) {
                Ok(x) => {
                    println!("Got a sync!");
                    x
                },
                Err(e) => {
                    println!("Got error attempting sync: {:#?}", e);
                    break
                }
            };

            if next_batch.is_some() {
                self.process_sync(&sync_response)
            } else {
                println!("Not processing initial sync...");
            };

            next_batch = sync_response.next_batch;
            if !self.should_quit && next_batch.is_some() {
                println!("Got a sync! Attempting another...");
            } else if next_batch.is_none() {
                println!("No next batch! Terminating!");
                break;
            } else {
                break;
            }
        }

        println!("Attempting to log back out...");
        match self.matrix_client.logout() {
            Ok(_) => {
                println!("Success!");
            },
            Err(e) => {
                println!("Failed to logout!");
                println!("{:?}", e);
            }
        };

    }

    fn process_sync(&mut self, sync: &SyncResponse) -> () {
        for (room_name, room_data) in sync.rooms.join.iter() {
            self.process_joined_room(room_name, room_data);
        }
    }

    fn process_joined_room(&mut self, room_name: &str, room_data: &JoinedRoom) -> () {
        let timeline = match room_data.timeline {
            Some(ref timeline) => timeline,
            None => { return; }
        };

        for event in timeline.events.iter() {
            let room_msg = match event {
                &Event::RoomMessage(ref room_msg) => room_msg,
                _ => { continue; }
            };

            match room_msg.content {
                RoomMessageTypes::TextMessage(ref txt) => {
                    let body: &str = txt.body.as_ref();
                    if body.starts_with(format!("{}: ", self.username).as_str()) == true {
                        self.process_command(room_name, room_msg.sender.as_ref(), body);
                    }
                },
                _ => ()
            }
        }
    }

    fn process_command(&mut self, room_name: &str, sender: Option<&String>, message: &str) -> () {
        if message == format!("{}: quit", self.username) {
            println!("\"{:?}\" in room \"{}\" told us to quit! QUITTIN'!", sender, room_name);
            self.should_quit = true;
        } else if message.starts_with(format!("{}: say ", self.username).as_str()) {
            let (_, last) = message.split_at(format!("{}: say ", self.username).len());
            let send_message = if last.len() > 0 {
                TextMessageType { body: String::from(last) }
            } else {
                TextMessageType { body: String::from("I can't say nothing.  That would be weird!") }
            };

            match self.matrix_client.send_room_message(room_name, &RoomMessageTypes::TextMessage(send_message)) {
                Ok(event) => {
                    println!("Successfully said a thing! Got: {:?}", event);
                },
                Err(e) => {
                    println!("Failed to respond!");
                    println!("{:?}", e);
                }
            }
        }
    }
}

