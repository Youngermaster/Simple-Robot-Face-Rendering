//! Infrastructure Layer - External Adapters
//!
//! This layer handles communication with external systems:
//! - Network (WebSocket, HTTP)
//! - File I/O (future)
//! - Database (future)
//!
//! It depends on the domain and application layers but not vice versa.

pub mod network;

// Re-export
pub use network::Server;
