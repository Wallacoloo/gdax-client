extern crate chrono;
extern crate gdax_client;

use chrono::prelude::*;

use gdax_client::PublicClient;

fn main() {
    let public_client = PublicClient::new();

    println!("Products:\n{:?}", public_client.get_products());
    println!("Product Order Book: \n{:?} \n{:?} \n{:?}",
             public_client.get_product_order_book("BTC-USD", 1),
             public_client.get_product_order_book("BTC-USD", 2),
             public_client.get_product_order_book("BTC-USD", 3));
    println!("Product Ticker: {:?}", public_client.get_product_ticker("BTC-USD"));
    println!("Latest Trades: {:?}", public_client.get_product_trades("BTC-USD"));
    println!("Historic Rates: {:?}",
             public_client.get_product_historic_rates("BTC-USD",
                                                      Utc.ymd(2016, 6, 11).and_hms(0, 0, 0),
                                                      Utc.ymd(2016, 6, 10).and_hms(12, 0, 0),
                                                      30 * 60));
    println!("24Hr stats: {:?}", public_client.get_product_24hr_stats("BTC-USD"));
    println!("Currencies: {:?}", public_client.get_currencies());
    println!("Time: {:?}", public_client.get_time());
}
