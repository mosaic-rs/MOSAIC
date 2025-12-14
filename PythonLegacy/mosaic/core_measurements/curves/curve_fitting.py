# mosaic/curve_fitting.py
from __future__ import annotations
from typing import Sequence, Tuple, Dict, Callable
import numpy as np
import pandas as pd
from mosaic.config import (
    UPPER_OUTER_RIGHT_COORDS, UPPER_OUTER_LEFT_COORDS,
    LOWER_OUTER_RIGHT_COORDS, LOWER_OUTER_LEFT_COORDS,
    UPPER_INNER_RIGHT_COORDS, UPPER_INNER_LEFT_COORDS,
    LOWER_INNER_RIGHT_COORDS, LOWER_INNER_LEFT_COORDS,
)


def _polyval(c: Sequence[float], t: float) -> float:
    return float(np.polyval(c, t))


def _cubic_coeffs(pts: np.ndarray) -> Tuple[np.ndarray, np.ndarray]:
    """Return (cx, cy) power-basis coeffs for cubic Bézier through pts[0,1,2,3]."""
    P0, Q1, Q2, P3 = pts
    t1, t2 = 1 / 3, 2 / 3

    A = np.array([[3*(1-t1)**2*t1, 3*(1-t1)*t1**2],
                  [3*(1-t2)**2*t2, 3*(1-t2)*t2**2]])
    bx = np.array([
        Q1[0] - ((1-t1)**3*P0[0] + t1**3*P3[0]),
        Q2[0] - ((1-t2)**3*P0[0] + t2**3*P3[0])
    ])
    by = np.array([
        Q1[1] - ((1-t1)**3*P0[1] + t1**3*P3[1]),
        Q2[1] - ((1-t2)**3*P0[1] + t2**3*P3[1])
    ])
    P1x, P2x = np.linalg.solve(A, bx)
    P1y, P2y = np.linalg.solve(A, by)
    P1, P2 = np.array([P1x, P1y]), np.array([P2x, P2y])

    cx = np.array([-P0[0] + 3*P1[0] - 3*P2[0] + P3[0],
                    3*P0[0] - 6*P1[0] + 3*P2[0],
                   -3*P0[0] + 3*P1[0],
                    P0[0]])
    cy = np.array([-P0[1] + 3*P1[1] - 3*P2[1] + P3[1],
                    3*P0[1] - 6*P1[1] + 3*P2[1],
                   -3*P0[1] + 3*P1[1],
                    P0[1]])
    return cx, cy


def _quad_coeffs(pts: np.ndarray) -> Tuple[np.ndarray, np.ndarray]:
    """Return (cx, cy) power-basis coeffs for quadratic Bézier through pts[0], pts[1] (mid), pts[2]."""
    P0, Q, P2 = pts
    P1 = 2*Q - 0.5*(P0+P2)          # control point forcing pass through Q at t=0.5

    cx = np.array([P0[0]-2*P1[0]+P2[0], 2*(P1[0]-P0[0]), P0[0]])
    cy = np.array([P0[1]-2*P1[1]+P2[1], 2*(P1[1]-P0[1]), P0[1]])
    return cx, cy


def _eval_coeffs(cx: Sequence[float], cy: Sequence[float], t: float) -> Tuple[float, float]:
    return _polyval(cx, t), _polyval(cy, t)


class CurveFitting:
    # region metadata table keeps code DRY
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
    def __init__(self, src: str | pd.DataFrame, row: int, t: float):
        self.landmarks = pd.read_csv(src) if isinstance(src, str) else src
        self.row = row
        self.t = float(t)
        self._rowdata = self.landmarks.iloc[row]
        self._coeff_cache: Dict[str, Tuple[np.ndarray, np.ndarray]] = {}

    @staticmethod
    def mirror_curve_vertically(points):
        P0, *_, P3 = points
        ymid = (P0[1] + P3[1]) / 2
        return [(x, 2*ymid - y) for x, y in points]

    @staticmethod
    def adjust_commissure_control(points, threshold=4):
        P0, Q1, Q2, P3 = points

        dist = lambda a, b: np.hypot(a[0]-b[0], a[1]-b[1])
        if dist(Q1, Q2) < threshold and dist(P0, P3) < 2*threshold:
            midx, midy = (P0[0]+P3[0])/2, (P0[1]+P3[1])/2
            offset = 8
            Q1, Q2 = (midx, midy+offset), (midx, midy-offset)
        return [P0, Q1, Q2, P3]

    # point + coeffs
    def upper_outer_right_curve(self):  return self._curve("upper_outer_right")
    def upper_outer_left_curve(self):   return self._curve("upper_outer_left")
    def lower_outer_right_curve(self):  return self._curve("lower_outer_right")
    def lower_outer_left_curve(self):   return self._curve("lower_outer_left")
    def upper_inner_right_curve(self):  return self._curve("upper_inner_right")
    def upper_inner_left_curve(self):   return self._curve("upper_inner_left")
    def lower_inner_right_curve(self):  return self._curve("lower_inner_right")
    def lower_inner_left_curve(self):   return self._curve("lower_inner_left")

    # coeffs only (handy for CSV export)
    def upper_outer_right_coeffs(self): return self._coeffs("upper_outer_right")
    def upper_outer_left_coeffs(self):  return self._coeffs("upper_outer_left")
    def lower_outer_right_coeffs(self): return self._coeffs("lower_outer_right")
    def lower_outer_left_coeffs(self):  return self._coeffs("lower_outer_left")
    def upper_inner_right_coeffs(self): return self._coeffs("upper_inner_right")
    def upper_inner_left_coeffs(self):  return self._coeffs("upper_inner_left")
    def lower_inner_right_coeffs(self): return self._coeffs("lower_inner_right")
    def lower_inner_left_coeffs(self):  return self._coeffs("lower_inner_left")

    # ------------ internals -----------------------------------------------
    def _curve(self, region: str):
        cx, cy = self._coeffs(region)
        x, y = _eval_coeffs(cx, cy, self.t)
        return (x, y), (tuple(cx), tuple(cy))

    def _coeffs(self, region: str) -> Tuple[np.ndarray, np.ndarray]:
        if region in self._coeff_cache:
            return self._coeff_cache[region]
        coord_list, deg = self._REGIONS[region]
        pts = np.array([[self._rowdata[x], self._rowdata[y]] for x, y in coord_list], float)
        cx, cy = (_cubic_coeffs if deg == 3 else _quad_coeffs)(pts)
        self._coeff_cache[region] = (cx, cy)
        return cx, cy

    def _coeffs_with_pts(self, region: str) -> Tuple[np.ndarray, np.ndarray, np.ndarray]:
        coord_list, deg = self._REGIONS[region]
        pts = np.array([[self._rowdata[x], self._rowdata[y]] for x, y in coord_list], float)
        cx, cy = (_cubic_coeffs if deg == 3 else _quad_coeffs)(pts)
        return cx, cy, pts

# frame,region,function_type,Ax,Bx,Cx,Dx,Ay,By,Cy,Dy,P0x,P0y,P1x,P1y,P2x,P2y,P3x,P3y,X0,Y0,X1,Y1,X2,Y2,X3,Y3