/*
This module file is used to separate the API keys from main.rs
by doing this, the main file is looking less like unorganised code,
*/

pub fn key() -> &'static str {
    static KEY: &str = "1d0dc0f7b4e808d430b95d8fed7df3ea";
    KEY
}

pub fn secret() -> &'static str {
    static SECRET: &str =
        "dTUic8DZPqkS77vxhJFEX5IBr13FcFHTzWYOARgT9kDWGdN03uvxBbH/hVy8f4O5RDmuf+9wNpEfhYhw2FCWyA==";
    SECRET
}

pub fn passphrase() -> &'static str {
    static PASSPHRASE: &str = "sandbox";
    PASSPHRASE
}
