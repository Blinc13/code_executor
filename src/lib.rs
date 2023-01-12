pub use handler::EventHandler;

pub mod handler;
pub mod commands;
pub mod executor;
pub mod utils;

pub trait HandlerFromEnv: serenity::client::EventHandler {
    fn from_env() -> Self;
}