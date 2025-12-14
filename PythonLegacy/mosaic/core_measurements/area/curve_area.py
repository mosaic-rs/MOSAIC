"""

NOT FUNCTIONAL !

CURVE AREA FINDING - SEE README

Finds the area of a bezier cubic and quadratic curve for respective quadrants of the mouth.

"""
#TODO: Make "rectangles" be average of mouth - not Y0 --- Make Quadratic Area Function

import matplotlib.pyplot as plt


import pandas as pd
import numpy as np
from pathlib import Path
from mosaic.config import (
    UPPER_OUTER_RIGHT_COORDS, UPPER_OUTER_LEFT_COORDS,
    LOWER_OUTER_RIGHT_COORDS, LOWER_OUTER_LEFT_COORDS,
    UPPER_INNER_RIGHT_COORDS, UPPER_INNER_LEFT_COORDS,
    LOWER_INNER_RIGHT_COORDS, LOWER_INNER_LEFT_COORDS,
)

class CurveArea:
    _REGIONS = {
        "upper_outer_right": (UPPER_OUTER_RIGHT_COORDS, 3),
        "upper_outer_left":  (UPPER_OUTER_LEFT_COORDS,  3),
        "lower_outer_right": (LOWER_OUTER_RIGHT_COORDS, 3),
        "lower_outer_left":  (LOWER_OUTER_LEFT_COORDS,  3),
        "upper_inner_right": (UPPER_INNER_RIGHT_COORDS, 2),
        "upper_inner_left":  (UPPER_INNER_LEFT_COORDS,  2),
        "lower_inner_right": (LOWER_INNER_RIGHT_COORDS, 2),
        "lower_inner_left":  (LOWER_INNER_LEFT_COORDS,  2),
    }

    def __init__(self, src: str | pd.DataFrame, region, row: int, t: float):
        self.coeffs = pd.read_csv(src) if isinstance(src, str) else src
        self.row = row
        self.column = "frame"
        self.t = float(t)
        self._rowdata = self.coeffs[self.coeffs[self.column] == self.row]
        self.region = region
        # self._coeff_cache: Dict[str, Tuple[np.ndarray, np.ndarray]] = {}

    def _commissures(self, frame: pd.DataFrame):
        """
        Return (xL,yL) and (xR,yR) for the outer-lip commissures in *this* frame.
        Assumes you always have both outer curves in the CSV.
        """
        left = frame.loc[frame["region"] == "upper_outer_left"].iloc[0]
        right = frame.loc[frame["region"] == "upper_outer_right"].iloc[0]

        # The left commissure is P0 of the left curve; right one is P0 of the right curve
        xL, yL = left["Dx"], left["Dy"]  # P0 = (Dx,Dy)
        xR, yR = right["Dx"], right["Dy"]
        return (xL, yL), (xR, yR)

    def _frame_geometry(self):
        """
        Geometry for the current frame.

        Returns
        -------
        m, b          … slope and intercept of the commissure baseline
        x_mid, y_mid  … centre of mouth (average of philtrum & lower-mid outer lip)
        """
        frame = self._rowdata

        # ----- commissure line (exactly as before) -------------------------------
        (xL, yL), (xR, yR) = self._commissures(frame)

        # ensure numeric left-then-right
        if xL > xR:
            (xL, yL), (xR, yR) = (xR, yR), (xL, yL)

        m = (yR - yL) / (xR - xL + 1e-9)
        b = yL - m * xL

        # ----- philtrum (outer-upper mid) ---------------------------------------
        upper_row = frame.loc[frame["region"] == "upper_outer_right"]
        if upper_row.empty:  # fallback to _left side
            upper_row = frame.loc[frame["region"] == "upper_outer_left"]

        c_up = upper_row[["Ax", "Bx", "Cx", "Dx"]].iloc[0].to_numpy()
        x_up = np.poly1d(c_up)(1.0)  # x at t = 1  (P3)

        # ----- lower-mid (outer-lower mid) --------------------------------------
        lower_row = frame.loc[frame["region"] == "lower_outer_right"]
        if lower_row.empty:
            lower_row = frame.loc[frame["region"] == "lower_outer_left"]

        c_lo = lower_row[["Ax", "Bx", "Cx", "Dx"]].iloc[0].to_numpy()
        x_lo = np.poly1d(c_lo)(1.0)  # x at t = 1  (P3)

        # ----- centre of mouth ---------------------------------------------------
        x_mid = 0.5 * (x_up + x_lo)
        y_mid = 0.5 * (yL + yR)  # vertical midpoint on commissure baseline

        return m, b, x_mid, y_mid

    def _Yb_M(self, data: pd.DataFrame):
        yb = data["Y0"].iloc[0]
        M = data["Y3"].iloc[0] - yb
        return yb, M

    def _cubic_area(self, plot: bool = True):
        data = self._rowdata.loc[self._rowdata["region"] == self.region]
        if data.empty:
            raise ValueError(f"No data for region {self.region} @ frame {self.row}")

        cx = data[["Ax", "Bx", "Cx", "Dx"]].iloc[0].to_numpy()
        cy = data[["Ay", "By", "Cy", "Dy"]].iloc[0].to_numpy()

        x_poly = np.poly1d(cx)
        y_poly = np.poly1d(cy)
        dx_poly = np.polyder(x_poly)

        # --- new tilted baseline ---------------------------------------------
        m, b, x_mid, y_mid = self._frame_geometry()
        y_base_poly = m * x_poly + b
        integrand = np.polymul(y_poly - y_base_poly, dx_poly)

        # --- integrate --------------------------------------------------------
        area_signed = np.polyint(integrand)(1.0) - np.polyint(integrand)(0.0)
        area = abs(area_signed)

        return area

    def _quadratic_area(self, plot: bool=True):
        data = self._rowdata.loc[self._rowdata["region"] == self.region]

        cx = data[["Ax", "Bx", "Cx"]].iloc[0].to_numpy()
        cy = data[["Ay", "By", "Cy"]].iloc[0].to_numpy()


    def upper_outer_right_area(self): return self._cubic_area()
    def upper_outer_left_area(self): return self._cubic_area()
    def lower_outer_right_area(self): return self._cubic_area()
    def lower_outer_left_area(self): return self._cubic_area()
    def upper_inner_right_area(self): return self._quadratic_area()
    def upper_inner_left_area(self): return self._quadratic_area()
    def lower_inner_right_area(self): return self._quadratic_area()
    def lower_inner_left_area(self): return self._quadratic_area()
