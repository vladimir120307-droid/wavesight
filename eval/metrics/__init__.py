"""Honest metrics computation for WaveSight benchmarks.

Each function returns a point estimate together with a 90% bootstrap
confidence interval (see ADR-004). Functions never silently coerce
mismatched lengths — that is always an error.
"""

from .core import (
    binary_precision_recall,
    mean_absolute_error,
    pck_at_threshold,
)

__all__ = [
    "binary_precision_recall",
    "mean_absolute_error",
    "pck_at_threshold",
]
