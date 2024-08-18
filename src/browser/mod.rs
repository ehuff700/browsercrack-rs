use alloc::string::String;

pub mod chromium;
pub mod nss;

#[derive(Debug)]
pub struct Login {
    username: String,
    password: String,
    url: String,
}

#[derive(Debug)]
pub struct Cookie {}
