/*
This is the prototype of the Rust bot project
*/

use coinbase_pro_rs::{Private, Sync, SANDBOX_URL};
use std::thread;
use std::time::Duration;

//I have created this module to remove the unnecessary keys from the main
//code.
mod keys;
use keys::{key, passphrase, secret};
fn main() {
    let client: Private<Sync> = Private::new(SANDBOX_URL, key(), secret(), passphrase());
    println!("Keep building!");
    let accounts = client.get_accounts().unwrap();

    /*
    println!("Great British Pound: ");
    let gbp = accounts.iter().find(|x| x.currency == "GBP").unwrap();
    println!("{}.  balance: {:?}", gbp.currency, gbp.balance);
    println!("{}.available: {:?}", gbp.currency, gbp.available);
    println!("{}.     hold: {:?}", gbp.currency, gbp.hold);
    thread::sleep(Duration::from_millis(200));
    */
    println!("United States Dollar:");
    let usd = accounts.iter().find(|x| x.currency == "USD").unwrap();
    println!("{}.  balance: {:?}", usd.currency, usd.balance);
    println!("{}.available: {:?}", usd.currency, usd.available);
    println!("{}.     hold: {:?}", usd.currency, usd.hold);

    let mut number_of_trades: i32 = 1;
    loop {
        if number_of_trades == 3600 {
            println!("[Finishing]");
            break;
        } else {
            client
                .buy_market_funds("BTC-USD", 10.00)
                .expect("Could not buy_market_funds");
            client
                .sell_market_funds("BTC-USD", 10.00)
                .expect("Could not sell_market_funds");

            number_of_trades += 1;

            //Stop Loss function.
            if usd.balance <= 5.00 {
                println!("[Breaking loop]");
                break;
            }
        }
        thread::sleep(Duration::from_millis(200));
    }
    println!("United States Dollar:");
    let usd = accounts.iter().find(|x| x.currency == "USD").unwrap();
    println!("{}.  balance: {:?}", usd.currency, usd.balance);
    println!("{}.available: {:?}", usd.currency, usd.available);
    println!("{}.     hold: {:?}", usd.currency, usd.hold);
}
