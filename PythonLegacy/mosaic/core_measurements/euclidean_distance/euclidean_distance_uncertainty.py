"""
Uncertainty calc for euclidean_distance.py.py
"""

from mosaic.config import LANDMARK_PAIRS, LANDMARK_PAIRS_UNC
from pathlib import Path
from typing import Sequence
import numpy as np
import pandas as pd

class EuclideanDistanceUncertainty:
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
    def _uncertainty(row: pd.Series, pairs: Sequence[tuple[str, str]], pair_unc: Sequence[tuple[str, str]]):
        d_unc = []
        for (c1, c2), (a1, a2) in zip(pairs, pair_unc):
            x = row[c1]
            y = row[c2]
            x_unc = row[a1]
            y_unc = row[a2]

            delta_d = (1/np.sqrt(x**2+y**2))*(np.sqrt(((x**2)*(x_unc**2))+((y**2)*(y_unc**2))))
            d_unc.append(float(delta_d))

        return d_unc

    def EuclideanDistUncerCalc(self, row: int):
        return self._uncertainty(self.landmarks.iloc[row], LANDMARK_PAIRS, LANDMARK_PAIRS_UNC)