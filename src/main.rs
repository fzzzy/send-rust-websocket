

#[macro_use]
extern crate serde_json;

use itertools::Itertools;
use ring::rand::{SystemRandom, SecureRandom};

use std::net::{SocketAddr, TcpStream};
use std::thread;

use websocket::OwnedMessage;
use websocket::sender::Writer;
use websocket::sync::Server;

fn close(mut sender: Writer<TcpStream>, ip: SocketAddr) {
    let message = OwnedMessage::Close(None);
    sender.send_message(&message).unwrap();
    println!("DISCONNECTED: {}", ip);
}

fn main() {
    let server = Server::bind("127.0.0.1:12223").unwrap();

    for request in server.filter_map(Result::ok) {
        thread::spawn(move || {
            let rand = SystemRandom::new();
            let client = request.accept().unwrap();
            let ip = client.peer_addr().unwrap();
            let (mut receiver, mut sender) = client.split().unwrap();
            let mut first_message = true;

            println!("CONNECTED: {}", ip);

            for message in receiver.incoming_messages() {
                let message = message.unwrap();

                match message {
                    OwnedMessage::Close(_) => {
                        return close(sender, ip);
                    }
                    OwnedMessage::Ping(ping) => {
                        let message = OwnedMessage::Pong(ping);
                        sender.send_message(&message).unwrap();
                    }
                    OwnedMessage::Text(_msg) => {
                        if first_message {
                            first_message = false;
                            //println!("First message {}", msg);
                            // TODO Randomly generate values here
                            let mut id: [u8; 5] = [0; 5];
                            let mut owner: [u8; 10] = [0; 10];
                            rand.fill(&mut id).unwrap();
                            rand.fill(&mut owner).unwrap();

                            let json_data = json!({
                                "url": format!("http://localhost:8080/download/{:02x}/", id.iter().format("")),
                                "ownerToken": format!("{:02x}", owner.iter().format("")),
                                "id": format!("{:02x}", id.iter().format(""))
                            });
                            let reply = OwnedMessage::Text(json_data.to_string());
                            sender.send_message(&reply).unwrap();
                        } else {
                            println!("{}: Got non-first message; closing socket", ip);
                            return close(sender, ip);
                        }
                    }
                    OwnedMessage::Binary(bin) => {
                        if first_message {
                            println!("{}: Got binary message before json metadata; closing socket", ip);
                            return close(sender, ip);
                        }

                        if bin.len() == 1 &&  bin[0] == 0 {
                            let json_data = json!({
                                "ok": true
                            });
                            let reply = OwnedMessage::Text(json_data.to_string());
                            sender.send_message(&reply).unwrap();
                        }
                    }
                    OwnedMessage::Pong(_) => {
                        println!("{}: Got pong message, but we should never get a pong; closing socket", ip);
                        return close(sender, ip);
                    }
                }
            }
        });
    }
}


