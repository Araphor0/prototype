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
    let accounts = client.get_accounts().unwrap();

    println!("Great British Pound: ");
    let gbp = accounts.iter().find(|x| x.currency == "GBP").unwrap();
    println!("{}.  balance: {:?}", gbp.currency, gbp.balance);
    println!("{}.available: {:?}", gbp.currency, gbp.available);
    println!("{}.     hold: {:?}", gbp.currency, gbp.hold);

    println!("United States Dollar:");
    let usd = accounts.iter().find(|x| x.currency == "USD").unwrap();
    println!("{}.  balance: {:?}", usd.currency, usd.balance);
    println!("{}.available: {:?}", usd.currency, usd.available);
    println!("{}.     hold: {:?}", usd.currency, usd.hold);
}
