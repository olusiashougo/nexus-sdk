#![doc = include_str!("../README.md")]

use nexus_toolkit::bootstrap;

mod coinbase_client;
mod error;
mod tools;

#[tokio::main]
async fn main() {
    bootstrap!([
        tools::get_spot_price::GetSpotPrice,
        tools::get_product_ticker::GetProductTicker,
    ]);
}
