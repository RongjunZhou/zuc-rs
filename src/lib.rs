pub mod core;
pub mod server;
pub mod bash;
pub mod config;
pub mod error;

pub use error::{ZucError, ZucResult};
pub use config::ZucConfig;