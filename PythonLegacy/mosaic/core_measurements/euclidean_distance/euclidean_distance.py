"""

By calculating the Euclidean distance of centred landmarks from the anchor point (0,0) we can measure how symmetrical
the mouth is on a horizontal and vertical axis. We can compare points (i.e. commissures) and include that in determining
the quality of asymmetry

The formula:

d = sqrt(x^2 + y^2)


this is stored as an int value in columns d_48, d_49 ... etc. following the same pattern of the x and y value naming system

"""

from mosaic.config import LANDMARK_PAIRS, LANDMARK_PAIRS_UNC
from pathlib import Path
from typing import Sequence
import numpy as np
import pandas as pd

class EuclideanDistance:
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
    def _extracting_data(row: pd.Series, pairs: Sequence[tuple[str,str]]):
        d = []
        for c1, c2 in pairs:
            v1 = row[c1]
            v2 = row[c2]
            frame=row["frame"]
            hyp = (np.hypot(v1, v2)) #  d = sqrt(v1^2 + v2^2)
            # print(f"FRAME: {frame} ----- {c1}: {v1}, {c2}: {v2}, d_{c1[-2:]}: {hyp}")

            d.append(float(hyp))

        return d


    def euclideanDistanceCalc(self, row: int):
        return self._extracting_data(self.landmarks.iloc[row], LANDMARK_PAIRS)
