# ADR-009 — ML stack: PyTorch training, Candle inference

- **Status**: accepted
- **Date**: 2026-05-24
- **Deciders**: @vladimir120307-droid

## Context

We need a training environment that the ML research community is already productive in, and an inference environment that fits inside our Rust server with no Python in the hot path.

## Decision

Training and inference are split:

- **Training** lives in `training/` and uses **PyTorch 2.4+**, **Lightning** for trainer scaffolding, **Weights & Biases** for experiment tracking. Datasets are stored as **WebDataset** shards on HuggingFace Hub.
- **Inference** lives in `server/crates/inference` and uses **Candle** (HuggingFace's pure-Rust ML framework). Weights are exported via `safetensors` and converted with `candle-export`.
- For ops Candle does not yet support, we fall back to **ONNX Runtime** as a secondary backend selectable via feature flag.

Models are versioned with **semver**; an `inference` registry maps `(model_name, version)` to a `safetensors` file under `server/models/`.

## Consequences

### Positive
- PyTorch is the lingua franca of ML research → contributions and pre-trained backbones are abundant.
- Candle keeps the runtime in Rust → no GIL, no Python packaging hell on edge devices.
- safetensors is fast, deterministic, secure (no pickle exec).

### Negative
- Two stacks to maintain. Some PyTorch features (e.g. custom CUDA ops) require manual Candle re-implementation.
- ONNX Runtime fallback brings a ~30 MB binary dependency we'd rather avoid; gated behind a feature.

### Neutral
- We will publish pre-trained weights on HuggingFace under `huggingface.co/wavesight`.

## Alternatives considered

### PyTorch end-to-end (Python in production)
Rejected. Edge devices and serverless deployments suffer from Python packaging fragility.

### Candle end-to-end (Rust training too)
Rejected. Research velocity in pure-Rust ML is far behind PyTorch; we would lose contributors.

### TensorFlow + TFLite
Rejected. TFLite-on-Linux is unmaintained; community momentum is firmly with PyTorch.

## References

- Candle: https://github.com/huggingface/candle
- safetensors: https://github.com/huggingface/safetensors
- WebDataset: https://github.com/webdataset/webdataset
