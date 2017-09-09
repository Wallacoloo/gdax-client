use ws;
use ws::{connect, Handler, Sender, Handshake, CloseCode, Error, ErrorKind};
use std;
use std::thread;
use std::sync::mpsc::channel;
use std::sync::mpsc::{Sender as TSender, Receiver as TReceiver};
use std::sync::Arc;

// We implement the Handler trait for Client so that we can get more
// fine-grained control of the connection.
struct Client {
    ws_out: Sender,
    products: String,
    event_sender: TSender<Event>
}

enum Event {
    Connect(Sender),
    Disconnect
}

enum Message {
    Open,
    Heartbeat
}

pub struct WebsocketClient {
    event_receiver: TReceiver<Event>,
    event_sender: TSender<Event>,
    //message_receiver: TReceiver<Message>>,
    websocket: Option<Sender>
}

impl Handler for Client {
    fn on_open(&mut self, _: Handshake) -> ws::Result<()> {
        // Now we don't need to call unwrap since `on_open` returns a `Result<()>`.
        // If this call fails, it will only result in this connection disconnecting.
        let sub = json!({
            "type": "subscribe",
            "product_ids": vec![self.products.clone()]
            });
        // Queue a message to be sent when the WebSocket is open
        if let Err(_) = self.ws_out.send(sub.to_string()) {
            println!("Websocket couldn't queue an initial message.")
        } else {
            println!("Client sent message 'Hello WebSocket'. ")
        }

        let heartbeat = json!({
            "type":"heartbeat",
            "on": true
            });
        self.ws_out.send(heartbeat.to_string());

        self.event_sender
            .send(Event::Connect(self.ws_out.clone()))
            .map_err(|err| Error::new(
                ErrorKind::Internal,
                format!("Unable to communicate between threads: {:?}.", err)));

        Ok(())
    }

    fn on_message(&mut self, msg: ws::Message) -> ws::Result<()> {
        println!("{}", msg);
        Ok(())
    }

    fn on_close(&mut self, code: CloseCode, reason: &str) {
        println!("Closing websocket");
        if reason.is_empty() {
            println!("Closing<({:?})>\n", code);
        } else {
            println!("Closing<({:?}) {}>\n", code, reason);
        }

        if let Err(err) = self.event_sender.send(Event::Disconnect) {
            println!("{:?}\n", err);
        }
    }

    fn on_error(&mut self, err: Error) {
        println!("Error<{:?}>\n", err);
    }
}

impl WebsocketClient {
    pub fn new() -> Self {
        let (tx, rx) = channel();
        return Self {
            event_receiver: rx,
            event_sender: tx,
            websocket: None
        };
    }

    pub fn connect(&mut self) {
        if let None = self.websocket {
            // Connect to the url and call the closure
            let (tx, rx) = channel();
            self.event_receiver = rx;
            //let sender = self.event_sender.clone();
            thread::spawn(move || {
                if let Err(error) = connect("wss://ws-feed.gdax.com".to_string(), |out| Client {
                    products: "BTC-USD".to_string(),
                    ws_out: out,
                    event_sender: tx.clone()
                }) {
                    // Inform the user of failure
                    println!("Failed to create WebSocket due to: {:?}", error);
                }
            });

            if let Ok(Event::Connect(sender)) = self.event_receiver.recv() {
                self.websocket = Some(sender);
                println!("Connected!");
            }
        }
    }

    pub fn is_open(&self) -> bool {
        match self.websocket {
            Some(_) => true,
            None => false
        }
    }
    pub fn close(&mut self) -> std::result::Result<(), ws::Error> {
        let res = match self.websocket {
            Some(ref ws_out) => ws_out.close(CloseCode::Normal),
            None => Ok(())
        };

        if let Ok(Event::Disconnect) = self.event_receiver.recv() {
            println!("Websocket disconnected!");
        }

        self.websocket = None;
        return res;
    }
}