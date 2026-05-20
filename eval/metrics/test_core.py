"""Sanity tests for the metrics module.

Run with `pytest -q metrics/` from the `eval/` directory; the module is
self-contained and does not require the `training/` package to be installed.
"""

from __future__ import annotations

import sys
from pathlib import Path

import numpy as np
import pytest

sys.path.insert(0, str(Path(__file__).resolve().parent.parent))

from metrics.core import (  # noqa: E402
    binary_precision_recall,
    mean_absolute_error,
    pck_at_threshold,
)


def test_perfect_prediction():
    pred = [True, True, False, False]
    tru = [True, True, False, False]
    result = binary_precision_recall(pred, tru)
    assert result["precision"].point == 1.0
    assert result["recall"].point == 1.0


def test_mae_zero_for_identical():
    pred = [1.0, 2.0, 3.0]
    tru = [1.0, 2.0, 3.0]
    result = mean_absolute_error(pred, tru)
    assert result.point == 0.0


def test_mae_known_value():
    pred = [1.0, 2.0]
    tru = [3.0, 4.0]
    result = mean_absolute_error(pred, tru)
    assert result.point == 2.0


def test_length_mismatch_raises():
    with pytest.raises(ValueError):
        binary_precision_recall([True], [True, False])
    with pytest.raises(ValueError):
        mean_absolute_error([1.0], [1.0, 2.0])


def test_pck_perfect():
    pred = np.zeros((5, 17, 2))
    tru = np.zeros((5, 17, 2))
    result = pck_at_threshold(pred, tru, 5.0)
    assert result.point == 1.0
