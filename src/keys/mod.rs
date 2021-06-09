/*
This module file is used to separate the API keys from main.rs
by doing this, the main file is looking less like unorganised code,
*/

pub fn key() -> &'static str {
    static KEY: &str = "";
    KEY
}

pub fn secret() -> &'static str {
    static SECRET: &str =
        "";
    SECRET
}

pub fn passphrase() -> &'static str {
    static PASSPHRASE: &str = "";
    PASSPHRASE
}
