// src/server/mod.rs

// TODO: Add the following modules
// pub mod request_handler;
// pub mod response_builder;
pub mod recursive_resolver;


// server/mod.rs
mod server;
pub use server::Server;

mod request_handler;
pub use request_handler::{DnsRequestHandler, RequestHandler};