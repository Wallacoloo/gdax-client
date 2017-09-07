use ws;
use ws::{connect, Handler, Sender, Handshake, Result, Message, CloseCode};
use std;


// We implement the Handler trait for Client so that we can get more
// fine-grained control of the connection.
struct Client {
    out: Sender
}

impl Handler for Client {
    // `on_open` will be called only after the WebSocket handshake is successful
    // so at this point we know that the connection is ready to send/receive messages.
    // We ignore the `Handshake` for now, but you could also use this method to setup
    // Handler state or reject the connection based on the details of the Request
    // or Response, such as by checking cookies or Auth headers.
    fn on_open(&mut self, _: Handshake) -> Result<()> {
        // Now we don't need to call unwrap since `on_open` returns a `Result<()>`.
        // If this call fails, it will only result in this connection disconnecting.
        println!("testing");
        let sub = json!({
            "type": "subscribe",
            "product_ids": ["BTC-USD"]
            });
        // Queue a message to be sent when the WebSocket is open
        if let Err(_) = self.out.send(sub.to_string()) {
            println!("Websocket couldn't queue an initial message.")
        } else {
            println!("Client sent message 'Hello WebSocket'. ")
        }

        let heartbeat = json!({
            "type":"heartbeat",
            "on": true
            });
        self.out.send(heartbeat.to_string());
        Ok(())
    }

    // `on_message` is roughly equivalent to the Handler closure. It takes a `Message`
    // and returns a `Result<()>`.
    fn on_message(&mut self, msg: Message) -> Result<()> {
        // Close the connection when we get a response from the server
        println!("{}", msg);
        Ok(())
    }
}

pub struct WebsocketClient {
    products: String,
    url: String,
    websocket: Option<Client>
}

impl WebsocketClient {
    pub fn new() -> Self {
        return Self {
            products: "BTC-USD".to_string(),
            url: "wss://ws-feed.gdax.com".to_string(),
            websocket: None
        };
    }

    pub fn connect(&self) {
        // Connect to the url and call the closure
        println!("test2");
        if let Err(error) = connect(self.url.clone(), |out| Client {
            out: out
        }) {
            // Inform the user of failure
            println!("Failed to create WebSocket due to: {:?}", error);
        }
    }
    pub fn close(&mut self) -> std::result::Result<(), ws::Error> {
        let res = match self.websocket {
            Some(ref client) =>  client.out.close(CloseCode::Normal),
            None => Ok(())
        };
        self.websocket = None;
        return res;
    }
}