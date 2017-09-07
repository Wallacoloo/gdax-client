use chrono::DateTime;
use chrono::offset::Utc;
use hyper::client::Client as HttpClient;
use hyper::header::UserAgent;
use hyper::net::HttpsConnector;
use hyper_native_tls::NativeTlsClient;
use serde::Deserialize;
use serde_json::de;
use uuid::Uuid;

use super::Error;
use super::Side;

const PUBLIC_API_URL: &'static str = "https://api.gdax.com";

#[derive(Deserialize, Debug)]
pub struct Product {
    pub id: String,
    pub base_currency: String,
    pub quote_currency: String,
    pub base_min_size: String,
    pub base_max_size: String,
    pub quote_increment: String
}

#[derive(Deserialize, Debug)]
pub struct BookEntry {
    pub price: String,
    pub size: String,
    pub num_orders: u64
}

#[derive(Deserialize, Debug)]
pub struct FullBookEntry {
    pub price: String,
    pub size: String,
    pub order_id: Uuid
}

#[derive(Deserialize, Debug)]
pub struct OrderBook<T> {
    pub sequence: usize,
    pub bids: Vec<T>,
    pub asks: Vec<T>
}

#[derive(Deserialize, Debug)]
pub struct Tick {
    pub trade_id: u64,
    pub price: String,
    pub size: String,
    pub bid: String,
    pub ask: String,
    pub volume: String,
    pub time: DateTime<Utc>
}

#[derive(Deserialize, Debug)]
pub struct Trade {
    pub time: DateTime<Utc>,
    pub trade_id: u64,
    pub price: String,
    pub size: String,
    pub side: Side,
}

#[derive(Deserialize, Debug)]
pub struct Candle {
    pub time: u64,
    pub low: f64,
    pub high: f64,
    pub open: f64,
    pub close: f64,
    pub volume: f64
}

#[derive(Deserialize, Debug)]
pub struct Stats {
    pub open: String,
    pub high: String,
    pub low: String,
    pub volume: String
}

#[derive(Deserialize, Debug)]
pub struct Currency {
    pub id: String,
    pub name: String,
    pub min_size: String
}

#[derive(Deserialize, Debug)]
pub struct Time {
    pub iso: DateTime<Utc>,
    pub epoch: f64
}

pub struct Client {
    http_client: HttpClient,
}

impl Client {
    pub fn new() -> Client {
        let ssl = NativeTlsClient::new().expect("Tls Client");
        let connector = HttpsConnector::new(ssl);

        Client {
            http_client: HttpClient::with_connector(connector)
        }
    }

    fn get_and_decode<T>(&self, url: &str) -> Result<T, Error>
        where for<'de> T: Deserialize<'de>
    {

        let mut res = self.http_client.get(url)
                                      .header(UserAgent("rust-gdax-client/0.1.0".to_owned()))
                                      .send()?;

        if !res.status.is_success() {
            return Err(Error::Api(de::from_reader(&mut res)?));
        }

        Ok(de::from_reader(&mut res)?)
    }

    pub fn get_products(&self) -> Result<Vec<Product>, Error> {
        self.get_and_decode(&format!("{}/products", PUBLIC_API_URL))
    }

    pub fn get_product_order_book(&self, product: &str, level: u8) -> Result<OrderBook<BookEntry>, Error> {
        match level {
            1| 2| 3 => self.get_and_decode(&format!("{}/products/{}/book?level={}",
                                                    PUBLIC_API_URL,
                                                    product,
                                                    level)),
            _ => Err(Error::InvalidArgument("Orderbook level must be 1,2, or 3".to_string()))
        }
    }

    pub fn get_product_ticker(&self, product: &str) -> Result<Tick, Error> {
        self.get_and_decode(&format!("{}/products/{}/ticker", PUBLIC_API_URL, product))
    }

    pub fn get_product_trades(&self, product: &str) -> Result<Vec<Trade>, Error> {
        self.get_and_decode(&format!("{}/products/{}/trades", PUBLIC_API_URL, product))
    }

    // XXX: Returns invalid interval?
    pub fn get_product_historic_rates(&self,
                              product: &str,
                              start_time: DateTime<Utc>,
                              end_time: DateTime<Utc>,
                              granularity: u64)
        -> Result<Vec<Candle>, Error> {

        self.get_and_decode(&format!("{}/products/{}/candles?start={}&end={}&granularity={}",
                                     PUBLIC_API_URL,
                                     product,
                                     start_time.to_rfc3339(),
                                     end_time.to_rfc3339(),
                                     granularity))
    }

    pub fn get_product_24hr_stats(&self, product: &str) -> Result<Stats, Error> {
        self.get_and_decode(&format!("{}/products/{}/stats", PUBLIC_API_URL, product))
    }

    pub fn get_currencies(&self) -> Result<Vec<Currency>, Error> {
        self.get_and_decode(&format!("{}/currencies", PUBLIC_API_URL))
    }

    pub fn get_time(&self) -> Result<Time, Error> {
        self.get_and_decode(&format!("{}/time", PUBLIC_API_URL))
    }
}
