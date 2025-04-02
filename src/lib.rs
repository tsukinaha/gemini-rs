#![warn(unreachable_pub, unused_qualifications)]

mod client;
mod error;
pub mod chat;
pub mod types;

pub type Result<T> = std::result::Result<T, Error>;

pub use chat::Chat;
pub use client::Client;
pub use error::Error;

pub fn client() -> Client {
    Client::instance()
}

pub fn chat(model: &str) -> Chat<chat::Text> {
    client().chat(model)
}
