from __future__ import annotations
from pathlib import Path
from typing import Sequence
import numpy as np
import pandas as pd

from mosaic.config import (
    X_INNER_BASED_ANCHOR_PAIRS, X_OUTER_BASED_ANCHOR_PAIRS,
    Y_INNER_BASED_ANCHOR_PAIRS, Y_OUTER_BASED_ANCHOR_PAIRS,
    LANDMARK_PAIRS
)

class Anchor:
    """
    Compute anchor-related metrics for a single frame (row) from a landmarks table.


    """
    def __init__(self, src: str | Path | pd.DataFrame, filtering: bool = True):
        if isinstance(src, (str, Path)):
            self.landmarks = pd.read_csv(src)
        elif isinstance(src, pd.DataFrame):
            self.landmarks = src
        else:
            raise TypeError("landmarks must be a CSV path or a pandas DataFrame")
        if not isinstance(filtering, bool):
            raise TypeError("filtering must be bool")
        self.filtering = filtering

    @staticmethod
    def _pair_midpoints(row: pd.Series, pairs: Sequence[tuple[str,str]]):
        mids = []
        for c1, c2 in pairs:
            v1 = row[c1]; v2 = row[c2]
            mids.append((v1 + v2) / 2.0)
        avg = float(np.mean(mids))
        std = float(np.std(mids))
        return avg, std

    def x_outer(self, row_idx: int): return self._pair_midpoints(self.landmarks.loc[row_idx], X_OUTER_BASED_ANCHOR_PAIRS)
    def x_inner(self, row_idx: int): return self._pair_midpoints(self.landmarks.loc[row_idx], X_INNER_BASED_ANCHOR_PAIRS)
    def y_outer(self, row_idx: int): return self._pair_midpoints(self.landmarks.loc[row_idx], Y_OUTER_BASED_ANCHOR_PAIRS)
    def y_inner(self, row_idx: int): return self._pair_midpoints(self.landmarks.loc[row_idx], Y_INNER_BASED_ANCHOR_PAIRS)

    @staticmethod
    def _combine_uncertainty(u1: float, u2: float) -> float:
        # sqrt((0.5*u1)^2 + (0.5*u2)^2)
        return float(np.hypot(0.5*u1, 0.5*u2))

    def get_all_anchors(self, row_idx: int) -> dict:
        x_out = self.x_outer(row_idx)
        x_in  = self.x_inner(row_idx)
        y_out = self.y_outer(row_idx)
        y_in  = self.y_inner(row_idx)

        x_val = (x_out[0] + x_in[0]) / 2.0
        y_val = (y_out[0] + y_in[0]) / 2.0
        x_unc = self._combine_uncertainty(x_out[1], x_in[1])
        y_unc = self._combine_uncertainty(y_out[1], y_in[1])

        return {
            "x_outer": x_out,
            "x_inner": x_in,
            "y_outer": y_out,
            "y_inner": y_in,
            "x_anchor": (x_val, x_unc),
            "y_anchor": (y_val, y_unc),
        }

class Centering:
    def __init__(self, df: pd.DataFrame, row: int, filtering: bool):
        self.landmarks = df
        self.row = row

    def center_with_anchor(self, x, y, x_uncertainty: float=None, y_uncertainty: float=None):
        new_landmark_values = []
        for col1, col2 in LANDMARK_PAIRS:
            x_val = self.landmarks.loc[self.row, col1] - x
            y_val = (self.landmarks.loc[self.row, col2] - y) * -1
            new_landmark_values.append((float(x_val), float(y_val)))
        return new_landmark_values

