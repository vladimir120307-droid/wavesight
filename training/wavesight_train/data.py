"""Dataset loaders for WaveSight training.

The dataset is published on HuggingFace as `wavesight/csi-{version}`. Each
sample is a `(amplitude, phase, label)` tuple plus metadata. We use the
HuggingFace `datasets` library for caching and shuffling.
"""

from __future__ import annotations

from collections.abc import Iterator
from dataclasses import dataclass
from pathlib import Path

import numpy as np

try:
    from datasets import load_dataset  # type: ignore
except ImportError:  # pragma: no cover — optional in editable installs
    load_dataset = None  # type: ignore


@dataclass
class CsiSample:
    """One labelled CSI sample."""

    amplitude: np.ndarray  # shape (n_subcarriers,)
    phase: np.ndarray  # shape (n_subcarriers,)
    rssi_dbm: int
    label: dict[str, float | int | bool]


def stream_dataset(
    name: str = "wavesight/csi-v0",
    split: str = "train",
    *,
    cache_dir: Path | None = None,
) -> Iterator[CsiSample]:
    """Stream samples from a HuggingFace-hosted dataset shard."""
    if load_dataset is None:
        raise RuntimeError("install the 'datasets' package to use stream_dataset")
    ds = load_dataset(
        name, split=split, streaming=True, cache_dir=str(cache_dir) if cache_dir else None
    )
    for row in ds:
        yield CsiSample(
            amplitude=np.asarray(row["amplitude"], dtype=np.float32),
            phase=np.asarray(row["phase"], dtype=np.float32),
            rssi_dbm=int(row.get("rssi_dbm", 0)),
            label=dict(row.get("label", {})),
        )
