

#[macro_use]
extern crate serde_json;

pub mod sendws;
pub mod sendapi;

use std::thread;

fn main() {
    thread::spawn(move || {
        sendws::websocket();
    });
    sendapi::api();
}


