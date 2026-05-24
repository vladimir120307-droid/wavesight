"""Core metric implementations with bootstrap confidence intervals."""

from __future__ import annotations

from dataclasses import dataclass
from typing import Sequence

import numpy as np


@dataclass
class PointAndInterval:
    point: float
    ci_low: float
    ci_high: float
    level: float


def _bootstrap_ci(
    values: np.ndarray,
    stat_fn,
    iterations: int = 2000,
    level: float = 0.9,
    rng: np.random.Generator | None = None,
) -> tuple[float, float]:
    rng = rng or np.random.default_rng(0)
    if values.size == 0:
        return (float("nan"), float("nan"))
    samples = np.empty(iterations, dtype=np.float64)
    for i in range(iterations):
        sample = rng.choice(values, size=values.size, replace=True)
        samples[i] = stat_fn(sample)
    half = (1 - level) / 2
    low = float(np.quantile(samples, half))
    high = float(np.quantile(samples, 1 - half))
    return low, high


def binary_precision_recall(
    predicted: Sequence[bool],
    truth: Sequence[bool],
) -> dict[str, PointAndInterval]:
    """Compute precision and recall with bootstrap CIs.

    Both inputs must have the same length and contain at least one positive
    in `truth`. Mismatched lengths raise ``ValueError``.
    """
    if len(predicted) != len(truth):
        raise ValueError("predicted and truth must have the same length")
    pred = np.asarray(predicted, dtype=bool)
    tru = np.asarray(truth, dtype=bool)
    tp = int(np.sum(pred & tru))
    fp = int(np.sum(pred & ~tru))
    fn = int(np.sum(~pred & tru))
    precision = tp / (tp + fp) if (tp + fp) else 0.0
    recall = tp / (tp + fn) if (tp + fn) else 0.0

    indices = np.arange(tru.size)

    def precision_stat(idx_sample):
        idx_sample = idx_sample.astype(int)
        p = pred[idx_sample]
        t = tru[idx_sample]
        tp_i = int(np.sum(p & t))
        fp_i = int(np.sum(p & ~t))
        return tp_i / (tp_i + fp_i) if (tp_i + fp_i) else 0.0

    def recall_stat(idx_sample):
        idx_sample = idx_sample.astype(int)
        p = pred[idx_sample]
        t = tru[idx_sample]
        tp_i = int(np.sum(p & t))
        fn_i = int(np.sum(~p & t))
        return tp_i / (tp_i + fn_i) if (tp_i + fn_i) else 0.0

    p_lo, p_hi = _bootstrap_ci(indices.astype(np.float64), precision_stat)
    r_lo, r_hi = _bootstrap_ci(indices.astype(np.float64), recall_stat)

    return {
        "precision": PointAndInterval(precision, p_lo, p_hi, 0.9),
        "recall": PointAndInterval(recall, r_lo, r_hi, 0.9),
    }


def mean_absolute_error(
    predicted: Sequence[float],
    truth: Sequence[float],
) -> PointAndInterval:
    """Compute MAE and a bootstrap CI."""
    if len(predicted) != len(truth):
        raise ValueError("predicted and truth must have the same length")
    pred = np.asarray(predicted, dtype=np.float64)
    tru = np.asarray(truth, dtype=np.float64)
    errors = np.abs(pred - tru)
    mae = float(np.mean(errors)) if errors.size else float("nan")
    low, high = _bootstrap_ci(errors, np.mean)
    return PointAndInterval(mae, low, high, 0.9)


def pck_at_threshold(
    predicted_kp: np.ndarray,
    truth_kp: np.ndarray,
    threshold_px: float,
) -> PointAndInterval:
    """Percentage of correctly localized keypoints at the given threshold.

    Shapes must be `(n_samples, n_keypoints, 2)` for both inputs. Returns
    the fraction of `(sample, keypoint)` pairs whose Euclidean distance
    is at most ``threshold_px``.
    """
    if predicted_kp.shape != truth_kp.shape:
        raise ValueError("keypoint arrays must match shape")
    dist = np.linalg.norm(predicted_kp - truth_kp, axis=-1)
    correct = (dist <= threshold_px).astype(np.float64).flatten()
    pck = float(np.mean(correct)) if correct.size else float("nan")
    low, high = _bootstrap_ci(correct, np.mean)
    return PointAndInterval(pck, low, high, 0.9)
