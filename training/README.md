# WaveSight training

PyTorch + Lightning + Weights & Biases. Datasets live on HuggingFace,
caching local. Models are exported to `safetensors` for on-edge
inference via the Candle runtime.

## Install

```bash
python -m venv .venv && source .venv/bin/activate
pip install -e .[dev]
```

## Layout

```
training/
├── pyproject.toml
├── wavesight_train/
│   ├── __init__.py
│   ├── cli.py             # `wavesight-train` command
│   ├── data.py            # HuggingFace streaming dataset
│   ├── models/            # PyTorch model definitions (presence, vitals, ...)
│   ├── trainers/          # Lightning trainers
│   └── export.py          # checkpoint → safetensors converter
└── tests/
```

## Run

```bash
wavesight-train train --config configs/presence_v0.yaml
wavesight-train export --checkpoint runs/presence/epoch=20.ckpt --out models/presence-v0.safetensors
```

## Status

The CLI is in place; concrete training loops land progressively as each
milestone reaches its model (presence M2, vitals M3, fall + pose M4,
sleep M5).
