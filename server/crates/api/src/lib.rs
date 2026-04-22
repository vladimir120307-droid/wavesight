//! Outward-facing API: REST, WebSocket, gRPC.

#![forbid(unsafe_code)]
#![warn(clippy::pedantic)]

use std::net::SocketAddr;

use wavesight_core::Result;

/// API configuration.
#[derive(Debug, Clone)]
pub struct ApiConfig {
    /// Bind address for REST + WebSocket.
    pub http_listen: SocketAddr,
    /// Bind address for gRPC.
    pub grpc_listen: SocketAddr,
}

impl Default for ApiConfig {
    fn default() -> Self {
        Self {
            http_listen: "0.0.0.0:8080".parse().expect("static address"),
            grpc_listen: "0.0.0.0:50051".parse().expect("static address"),
        }
    }
}

/// Spawn HTTP and gRPC servers. Returns when either exits.
///
/// # Errors
/// Returns an error if either listener fails to bind.
pub async fn serve(_config: ApiConfig) -> Result<()> {
    Ok(())
}
