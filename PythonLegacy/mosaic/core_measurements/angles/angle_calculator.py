"""

Calculates the angle of a landmark relative to the anchor/origin

"""

import numpy as np
import pandas as pd
from typing import Sequence

class AngleCalculator:
    def __init__(self, src: str | pd.DataFrame):
        if isinstance(src, str):
            self.landmarks = pd.read_csv(src)
        elif isinstance(src, pd.DataFrame):
            self.landmarks = src
        else:
            raise TypeError("landmarks must be a CSV path or a pandas DataFrame")

    @staticmethod
    def _calc_row_angles(row: pd.Series, pairs: Sequence[tuple[str, str]]):
        angles = []
        for x_col, y_col in pairs:
            x = row[x_col]
            y = row[y_col]
            theta = np.arctan2(y, x)
            angles.append(theta)
        return angles

    def compute_angles(self, row: int, pairs: Sequence[tuple[str, str]]):
        return self._calc_row_angles(self.landmarks.loc[row], pairs)
