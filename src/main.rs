

#[macro_use]
extern crate serde_json;

use std::thread;
use std::net::{SocketAddr, TcpStream};
use websocket::sync::Server;
use websocket::OwnedMessage;
use websocket::sender::Writer;

fn close(mut sender: Writer<TcpStream>, ip: SocketAddr) {
    let message = OwnedMessage::Close(None);
    sender.send_message(&message).unwrap();
    println!("DISCONNECTED: {}", ip);
}

fn main() {
    let server = Server::bind("127.0.0.1:12223").unwrap();

    for request in server.filter_map(Result::ok) {
        thread::spawn(move || {
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
                            let json_data = json!({
                                "url":"http://localhost:8080/download/000/",
                                "ownerToken":"000",
                                "id":"000"
                            });
                            let reply = OwnedMessage::Text(json_data.to_string());
                            sender.send_message(&reply).unwrap();
                        } else {
                            println!("Got non-first message; closing socket");
                            return close(sender, ip);
                        }
                    }
                    OwnedMessage::Binary(bin) => {
                        if first_message {
                            println!("Got binary message before json metadata; closing socket");
                            return close(sender, ip);
                        }

                        //println!("Got binary message!");
                        if bin.len() == 1 &&  bin[0] == 0 {
                            //println!("Got last binary packet!");
                            let json_data = json!({
                                "ok": true
                            });
                            let reply = OwnedMessage::Text(json_data.to_string());
                            sender.send_message(&reply).unwrap();
                        }
                    }
                    OwnedMessage::Pong(_) => {
                        println!("Got pong message, but we should never get a pong; closing socket");
                        return close(sender, ip);
                    }
                }
            }
        });
    }
}


