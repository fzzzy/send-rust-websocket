

#[macro_use]
extern crate serde_json;

pub mod sendws;

use hyper::server::{Server, Request, Response};
use std::thread;

const HTML: &'static str = include_str!("websockets.html");

fn main() {
    thread::spawn(move || {
        sendws::websocket();
    });
    Server::http("0.0.0.0:8088")
        .unwrap()
        .handle(move |_req: Request, res: Response| {
            res.send(HTML.as_bytes()).unwrap();
        }).unwrap();
}


