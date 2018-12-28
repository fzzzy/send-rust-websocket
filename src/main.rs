

#[macro_use]
extern crate serde_json;

pub mod sendws;

fn main() {
    sendws::websocket();
}


