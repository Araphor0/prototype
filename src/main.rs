/*
This is the prototype of the Rust bot project
*/

use coinbase_pro_rs::{Private, Sync, SANDBOX_URL};

//I have created this module to remove the unnecessary keys from the main
//code.
mod keys;
use keys::{key, passphrase, secret};
fn main() {
    let _client: Private<Sync> = Private::new(SANDBOX_URL, key(), secret(), passphrase());
    println!("Keep building!");
}
