extern crate gdax_client;

use gdax_client::WebsocketClient;
use std::time::Duration;
use std::thread::sleep;

fn main() {
    let mut ws = WebsocketClient::new();

    // Now, instead of a closure, the Factory returns a new instance of our Handler.
    ws.connect();

    sleep(Duration::from_secs(5));

    ws.close();

    return;
}