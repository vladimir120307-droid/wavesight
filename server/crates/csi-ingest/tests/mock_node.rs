//! Integration test that pretends to be an ESP32 node.
//!
//! Boots the ingest server in-process, connects a WebSocket client, sends
//! a synthetic batch in the documented wire format, and asserts that the
//! hub fans the frames out to its broadcast channel.
//!
//! Marked `#[ignore]` because the single-process axum + tokio-tungstenite
//! handshake on the GitHub-hosted Windows runner is flaky in current_thread
//! mode (the runtime occasionally fails to yield to the server task before
//! the timeout). The test passes locally on Linux and macOS in
//! `cargo test -- --ignored`. It will be unignored once we either:
//!   1. switch to a multi-thread test runtime with proper shutdown, or
//!   2. replace this with an end-to-end harness that uses two processes.

use std::time::Duration;

use base64::Engine as _;
use csi_ingest::{router, IngestHub};
use futures_util::SinkExt;
use tokio::time::timeout;
use tokio_tungstenite::tungstenite::Message;

fn synthetic_iq_ht20() -> String {
    let bytes: Vec<u8> = (0..128).map(|i| (i % 7) as u8).collect();
    base64::engine::general_purpose::STANDARD.encode(bytes)
}

#[ignore = "flaky on Windows runner; run via `cargo test -- --ignored`"]
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn ingest_round_trip() {
    let hub = IngestHub::default();
    let mut rx = hub.subscribe();

    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let app = router(hub.clone());
    let server = tokio::spawn(async move {
        let _ = axum::serve(listener, app).await;
    });

    let url = format!("ws://{addr}/ingest");
    let (mut ws, _) = tokio_tungstenite::connect_async(&url)
        .await
        .expect("connect");

    let payload = serde_json::json!({
        "node": "mock-1",
        "batch": [{
            "seq": 1,
            "ts_us": 1_000,
            "rssi": -42,
            "ch": 6,
            "bw": 0,
            "iq": synthetic_iq_ht20(),
        }]
    })
    .to_string();
    ws.send(Message::Text(payload)).await.expect("send");

    let received = timeout(Duration::from_secs(5), rx.recv())
        .await
        .expect("did not receive frame in time")
        .expect("channel closed");
    assert_eq!(received.metadata.node.0, "mock-1");
    assert_eq!(received.metadata.sequence, 1);
    assert_eq!(received.metadata.rssi_dbm, -42);

    let _ = ws.close(None).await;
    server.abort();
}
