//! Network infrastructure
//!
//! Handles WebSocket and HTTP server implementations.

pub mod handlers;
pub mod server;

// Re-export
pub use server::Server;
