use chrono::Utc;
use criterion::{criterion_group, criterion_main, Criterion};
use dsp::{process, DspConfig};
use wavesight_core::{CsiFrame, FrameMetadata, NodeId, SubcarrierCount};

fn make_frame(n: usize) -> CsiFrame {
    CsiFrame {
        metadata: FrameMetadata {
            node: NodeId::new("bench"),
            captured_at: Utc::now(),
            sequence: 0,
            channel: 6,
            subcarriers: SubcarrierCount::Ht20,
            rssi_dbm: -50,
        },
        samples: vec![1i8; n],
    }
}

fn bench_ht20(c: &mut Criterion) {
    let frame = make_frame(128);
    let cfg = DspConfig::default();
    c.bench_function("dsp_process_ht20", |b| b.iter(|| process(&frame, &cfg)));
}

criterion_group!(benches, bench_ht20);
criterion_main!(benches);
