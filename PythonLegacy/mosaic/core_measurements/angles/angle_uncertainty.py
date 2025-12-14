"""

Calculates the uncertainty for the angles by taking the partial derivative

"""

import numpy as np
import pandas as pd
from typing import Sequence

class AngleUncertaintyCalculator:
    def __init__(self, src: str | pd.DataFrame):
        if isinstance(src, str):
            self.df = pd.read_csv(src)
        elif isinstance(src, pd.DataFrame):
            self.df = src
        else:
            raise TypeError("landmarks must be a CSV path or a pandas DataFrame")

    @staticmethod
    def _calc_row_sigma_theta(row: pd.Series, pairs: Sequence[tuple[str, str]]):
        out = []
        for x_col, y_col in pairs:
            x = float(row.get(x_col, np.nan))
            y = float(row.get(y_col, np.nan))
            sx = float(row.get(f"{x_col}_unc", np.nan))
            sy = float(row.get(f"{y_col}_unc", np.nan))
            r2 = x * x + y * y
            if not np.isfinite(r2) or r2 == 0.0 or not (np.isfinite(sx) and np.isfinite(sy)):
                out.append(np.nan)
                continue
            dth_dx = y / r2
            dth_dy = x / r2
            s2 = (dth_dx * sx) ** 2 + (dth_dy * sy) ** 2
            out.append(np.sqrt(s2))
        return out

    def compute_sigma_theta(self, row: int, pairs: Sequence[tuple[str, str]]):
        return self._calc_row_sigma_theta(self.df.loc[row], pairs)
