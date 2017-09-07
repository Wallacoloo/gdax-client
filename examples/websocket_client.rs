extern crate gdax_client;

use gdax_client::WebsocketClient;

fn main() {
    println!("testing");
    let ws = WebsocketClient::new();

        // Now, instead of a closure, the Factory returns a new instance of our Handler.
    ws.connect();

}
