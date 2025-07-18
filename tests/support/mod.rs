#![allow(dead_code)]

pub mod delay_layer;
pub mod delay_server;
pub mod error;
pub mod server;

// TODO: remove once done converting to new support server?
#[allow(unused)]
pub static DEFAULT_USER_AGENT: &str =
    concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));
