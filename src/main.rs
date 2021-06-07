use coinbase_pro_rs::{Public, Sync, SANDBOX_URL};
/*
This is the prototype of the Rust bot project
*/

fn main() {
    println!("Keep building!");
    let client: Public<Sync> = Public::new(SANDBOX_URL);
    let time = client.get_time().unwrap();
    println!("Coinbase.time: {}", time.iso);
}