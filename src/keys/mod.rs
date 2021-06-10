/*
This module file is used to separate the API keys from main.rs
by doing this, the main file is looking less like unorganised code,
*/

/* Example code of what a buy and sell market order would consist of in code. */
    //client.buy_market_funds("ETH-GBP", 10.0).expect("Could not buy market funds");
    //client.sell_market_funds("ETH-GBP", 10.11).expect("Could not sell market");

    /*
    loop {
        client
            .buy_market_funds("ETH-GBP", 10.0)
            .expect("could not buy market funds");
        client
            .sell_market_funds("ETH-GBP", 10.11)
            .expect("Could not sell market funds");
    }
    */

pub fn key() -> &'static str {
    static KEY: &str = "c03a48b790134be03121bcefd7e272d9";
    KEY
}

pub fn secret() -> &'static str {
    static SECRET: &str =
        "w0ImJh09g1OjeV+k1Ie43/8JNMrK3Y1EDZKIrMMICE1y3hLq7Mw301wCRgrESlbcpYVseJEhsPrFx3Gr7rhCEg==";
    SECRET
}

pub fn passphrase() -> &'static str {
    static PASSPHRASE: &str = "r95bdnrvpx";
    PASSPHRASE
}
