//! Per-node fan-out hub.
//!
//! The hub keeps one Tokio channel per connected node, exposes a snapshot
//! of currently-active nodes, and lets downstream crates subscribe to
//! frames as they arrive.

use std::sync::Arc;

use dashmap::DashMap;
use tokio::sync::{broadcast, mpsc};
use tracing::warn;
use wavesight_core::{CsiFrame, NodeId};

/// One entry per connected node.
#[derive(Debug)]
pub struct NodeHandle {
    sender: mpsc::Sender<CsiFrame>,
}

impl NodeHandle {
    fn new(buffer: usize) -> (Self, mpsc::Receiver<CsiFrame>) {
        let (sender, rx) = mpsc::channel(buffer);
        (Self { sender }, rx)
    }

    /// Push a frame towards the DSP layer. Drops on backpressure with a
    /// warning instead of blocking the WebSocket reader.
    pub fn push(&self, frame: CsiFrame) {
        if self.sender.try_send(frame).is_err() {
            warn!("dsp channel full, dropping frame");
        }
    }
}

/// Shared, cloneable handle to the ingest hub.
#[derive(Clone, Debug)]
pub struct IngestHub {
    inner: Arc<Inner>,
}

#[derive(Debug)]
struct Inner {
    nodes: DashMap<NodeId, NodeHandle>,
    receivers: DashMap<NodeId, mpsc::Receiver<CsiFrame>>,
    fanout: broadcast::Sender<CsiFrame>,
    per_node_buffer: usize,
}

impl IngestHub {
    /// Construct a new hub.
    ///
    /// `per_node_buffer` controls how many frames each node's mpsc channel
    /// can hold before applying backpressure.
    #[must_use]
    pub fn new(per_node_buffer: usize) -> Self {
        let (fanout, _) = broadcast::channel(per_node_buffer.max(64));
        Self {
            inner: Arc::new(Inner {
                nodes: DashMap::new(),
                receivers: DashMap::new(),
                fanout,
                per_node_buffer,
            }),
        }
    }

    /// Register a node (or return its existing handle if already present).
    pub fn register(&self, node: NodeId) {
        if self.inner.nodes.contains_key(&node) {
            return;
        }
        let (handle, rx) = NodeHandle::new(self.inner.per_node_buffer);
        self.inner.nodes.insert(node.clone(), handle);
        self.inner.receivers.insert(node, rx);
    }

    /// Push a frame to the node's dedicated channel and broadcast it on
    /// the global fanout (for the dashboard live view).
    pub fn push(&self, frame: CsiFrame) {
        if let Some(handle) = self.inner.nodes.get(&frame.metadata.node) {
            handle.push(frame.clone());
        }
        let _ = self.inner.fanout.send(frame);
    }

    /// Subscribe to the global fan-out. Each subscriber receives every
    /// frame from every node; lagging subscribers drop messages rather
    /// than block the producers.
    #[must_use]
    pub fn subscribe(&self) -> broadcast::Receiver<CsiFrame> {
        self.inner.fanout.subscribe()
    }

    /// Snapshot of currently-known nodes.
    #[must_use]
    pub fn nodes(&self) -> Vec<NodeId> {
        self.inner.nodes.iter().map(|e| e.key().clone()).collect()
    }
}

impl Default for IngestHub {
    fn default() -> Self {
        Self::new(256)
    }
}
