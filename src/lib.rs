pub use handler::EventHandler;

pub mod handler;
pub mod commands;
pub mod compiler_interface;

pub trait HandlerFromEnv: serenity::client::EventHandler {
    fn from_env() -> Self;
}