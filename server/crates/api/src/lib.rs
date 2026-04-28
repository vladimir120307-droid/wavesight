//! Outward-facing API: REST + WebSocket.
//!
//! The api crate wires together `csi-ingest`, `dsp`, and `inference` into
//! a single HTTP service that the dashboard consumes.

#![forbid(unsafe_code)]
#![warn(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]

mod scene;
mod stream;

pub use scene::ScenePayload;
pub use stream::ws_handler;

use std::net::SocketAddr;
use std::sync::Arc;

use axum::{routing::get, Router};
use csi_ingest::IngestHub;
use tokio::sync::RwLock;
use tower_http::trace::TraceLayer;
use tracing::info;

/// API server configuration.
#[derive(Debug, Clone)]
pub struct ApiConfig {
    /// Bind address for REST + WebSocket.
    pub http_listen: SocketAddr,
    /// Bind address for ingest WebSocket.
    pub ingest_listen: SocketAddr,
}

impl Default for ApiConfig {
    fn default() -> Self {
        Self {
            http_listen: "0.0.0.0:8081".parse().expect("static address"),
            ingest_listen: "0.0.0.0:8080".parse().expect("static address"),
        }
    }
}

/// Shared application state passed to every handler.
#[derive(Clone)]
pub struct AppState {
    /// Ingest hub for live frame subscription.
    pub hub: IngestHub,
    /// Latest computed scene (updated by the live pipeline).
    pub latest_scene: Arc<RwLock<Option<ScenePayload>>>,
}

impl AppState {
    /// Construct app state with a fresh ingest hub.
    #[must_use]
    pub fn new(hub: IngestHub) -> Self {
        Self {
            hub,
            latest_scene: Arc::new(RwLock::new(None)),
        }
    }
}

/// Build the public-facing axum router.
#[must_use]
pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/healthz", get(healthz))
        .route("/api/v1/scene", get(scene::current_scene))
        .route("/api/v1/nodes", get(scene::nodes))
        .route("/api/v1/stream", get(stream::ws_handler))
        .with_state(state)
        .layer(TraceLayer::new_for_http())
}

async fn healthz() -> &'static str {
    "ok"
}

/// Start the public API server. Returns when the listener stops.
///
/// # Errors
/// Returns an error if the listener fails to bind.
pub async fn serve_api(state: AppState, listen: SocketAddr) -> anyhow::Result<()> {
    let listener = tokio::net::TcpListener::bind(listen).await?;
    info!(%listen, "api server listening");
    axum::serve(listener, router(state)).await?;
    Ok(())
}

/// Start the ingest server (separate port).
///
/// # Errors
/// Returns an error if the listener fails to bind.
pub async fn serve_ingest(hub: IngestHub, listen: SocketAddr) -> anyhow::Result<()> {
    let listener = tokio::net::TcpListener::bind(listen).await?;
    info!(%listen, "ingest server listening");
    axum::serve(listener, csi_ingest::router(hub)).await?;
    Ok(())
}
