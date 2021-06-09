/*
This is the prototype of the Rust bot project
*/

use coinbase_pro_rs::{Private, Sync, SANDBOX_URL};

//I have created this module to remove the unnecessary keys from the main
//code.
mod keys;
use keys::{key, passphrase, secret};
fn main() {
    let client: Private<Sync> = Private::new(SANDBOX_URL, key(), secret(), passphrase());
    println!("Keep building!");

    /* Example code of what a buy and sell market order would consist of in code. */
    //client.buy_market_funds("ETH-GBP", 10.0).expect("Could not buy market funds");
    //client.sell_market_funds("ETH-GBP", 10.11).expect("Could not sell market");

    loop {
        client
            .buy_market_funds("ETH-GBP", 10.0)
            .expect("could not buy market funds");
        client
            .sell_market_funds("ETH-GBP", 10.11)
            .expect("Could not sell market funds");
    }
}
