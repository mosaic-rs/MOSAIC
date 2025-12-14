from __future__ import annotations
from pathlib import Path
from typing import Sequence
import numpy as np
import pandas as pd

EPS = 1e-12

# CUBIC

class QuadrantArea:
    """
    Compute quadrant-based OUTER mouth area for a single frame.

    Each outer lip perimeter is represented as 4 cubic Bézier curves
    """

    def __init__(self, src: str | Path | pd.DataFrame):
        if isinstance(src, (str, Path)):
            self.landmarks = pd.read_csv(src)
        elif isinstance(src, pd.DataFrame):
            self.landmarks = src
        else:
            raise TypeError("landmarks must be a CSV path or a pandas DataFrame")

    @staticmethod
    def _cross(a, b):
        return a[0] * b[1] - a[1] * b[0]

    @staticmethod
    def _area_cubic(P):
        P0, P1, P2, P3 = [np.asarray(p, float) for p in P]
        c = QuadrantArea._cross
        return (1.0 / 20.0) * (
            6 * c(P0, P1) + 3 * c(P0, P2) + 1 * c(P0, P3)
            + 3 * c(P1, P2) + 3 * c(P1, P3) + 6 * c(P2, P3)
        )

    @staticmethod
    def _bezier_to_power_1d(c0, c1, c2, c3):
        d = c0
        c = 3 * (c1 - c0)
        b = 3 * (c2 - 2 * c1 + c0)
        a = c3 - 3 * c2 + 3 * c1 - c0
        return np.array([a, b, c, d], float)

    @staticmethod
    def _roots_in_01(P, axis=0):
        c0, c1, c2, c3 = [float(p[axis]) for p in P]
        coeffs = QuadrantArea._bezier_to_power_1d(c0, c1, c2, c3)
        if np.all(np.abs(coeffs[:3]) < 1e-18):
            return []
        r = np.roots(coeffs)
        r = r[np.isreal(r)].real
        return sorted([t for t in r if EPS < t < 1 - EPS])

    @staticmethod
    def _split_cubic(P, t):
        P0, P1, P2, P3 = [np.asarray(p, float) for p in P]
        P01 = (1-t)*P0 + t*P1
        P12 = (1-t)*P1 + t*P2
        P23 = (1-t)*P2 + t*P3
        P012 = (1-t)*P01 + t*P12
        P123 = (1-t)*P12 + t*P23
        P0123 = (1-t)*P012 + t*P123
        return [P0, P01, P012, P0123], [P0123, P123, P23, P3]

    @staticmethod
    def _split_at_axes(P):
        roots = QuadrantArea._roots_in_01(P, 0) + QuadrantArea._roots_in_01(P, 1)
        roots = sorted(set(round(t, 10) for t in roots))
        if not roots:
            return [P]
        segs, cur, prev_t = [], P, 0.0
        for t in roots + [1.0]:
            local = (t - prev_t) / (1.0 - prev_t)
            L, R = QuadrantArea._split_cubic(cur, local)
            segs.append(L)
            cur, prev_t = R, t
        return segs

    @staticmethod
    def _midpoint(P):
        P0, P1, P2, P3 = [np.asarray(p, float) for p in P]
        t = 0.5
        return ((1-t)**3)*P0 + 3*((1-t)**2)*t*P1 + 3*(1-t)*(t**2)*P2 + (t**3)*P3

    @staticmethod
    def _which_quadrant(pt):
        x, y = pt
        if abs(x) <= EPS or abs(y) <= EPS:
            return None
        if x > 0 and y > 0: return 1
        if x < 0 and y > 0: return 2
        if x < 0 and y < 0: return 3
        if x > 0 and y < 0: return 4

    def compute_outer(self, cubics_ccw: list[list[Sequence[float]]]) -> dict[int, float]:

        pieces = []
        for seg in cubics_ccw:
            pieces.extend(self._split_at_axes(seg))

        buckets = {1: [], 2: [], 3: [], 4: []}
        for pc in pieces:
            q = self._which_quadrant(self._midpoint(pc))
            if q:
                buckets[q].append(pc)

        areas = {1: 0.0, 2: 0.0, 3: 0.0, 4: 0.0}
        for q in (1, 2, 3, 4):
            for pc in buckets[q]:
                areas[q] += self._area_cubic(pc)

        areas["total"] = sum(abs(v) for v in areas.values())
        return areas

# QUADRATIC

class QuadraticArea:
    """
    Basically the same as the top but for quadratic curves as the inner lips are quadratic not cubic
    """
    def __init__(self, src: str | Path | pd.DataFrame):
        if isinstance(src, (str, Path)):
            self.landmarks = pd.read_csv(src)
        elif isinstance(src, pd.DataFrame):
            self.landmarks = src
        else:
            raise TypeError("landmarks must be a CSV path or a pandas DataFrame")

    @staticmethod
    def _cross(a, b):
        return a[0]*b[1] - a[1]*b[0]

    @staticmethod
    def _area_quadratic(P):
        P0, P1, P2 = [np.asarray(p, float) for p in P]
        c = QuadraticArea._cross
        return (1.0 / 6.0) * (2*c(P0,P1) + c(P0,P2) + 2*c(P1,P2))

    @staticmethod
    def _roots_in_01(P, axis=0):
        c0, c1, c2 = [float(p[axis]) for p in P]
        a = c0 - 2*c1 + c2
        b = 2*(c1 - c0)
        c = c0
        if abs(a) < 1e-12 and abs(b) < 1e-12:
            return []
        roots = np.roots([a, b, c])
        roots = roots[np.isreal(roots)].real
        return sorted([t for t in roots if EPS < t < 1 - EPS])

    @staticmethod
    def _split_quadratic(P, t):
        P0, P1, P2 = [np.asarray(p, float) for p in P]
        P01 = (1-t)*P0 + t*P1
        P12 = (1-t)*P1 + t*P2
        P012 = (1-t)*P01 + t*P12
        return [P0, P01, P012], [P012, P12, P2]

    @staticmethod
    def _split_at_axes(P):
        roots = QuadraticArea._roots_in_01(P, 0) + QuadraticArea._roots_in_01(P, 1)
        roots = sorted(set(round(t, 10) for t in roots))
        if not roots:
            return [P]
        segs, cur, prev_t = [], P, 0.0
        for t in roots + [1.0]:
            local = (t - prev_t) / (1.0 - prev_t)
            L, R = QuadraticArea._split_quadratic(cur, local)
            segs.append(L)
            cur, prev_t = R, t
        return segs

    @staticmethod
    def _midpoint(P):
        P0, P1, P2 = [np.asarray(p, float) for p in P]
        t = 0.5
        return ((1-t)**2)*P0 + 2*(1-t)*t*P1 + (t**2)*P2

    @staticmethod
    def _which_quadrant(pt):
        x, y = pt
        if abs(x) <= EPS or abs(y) <= EPS:
            return None
        if x > 0 and y > 0: return 1
        if x < 0 and y > 0: return 2
        if x < 0 and y < 0: return 3
        if x > 0 and y < 0: return 4

    def compute_inner(self, quads_ccw: list[list[Sequence[float]]]) -> dict[int, float]:
        pieces = []
        for seg in quads_ccw:
            pieces.extend(self._split_at_axes(seg))

        buckets = {1: [], 2: [], 3: [], 4: []}
        for pc in pieces:
            q = self._which_quadrant(self._midpoint(pc))
            if q:
                buckets[q].append(pc)

        areas = {1: 0.0, 2: 0.0, 3: 0.0, 4: 0.0}
        for q in (1, 2, 3, 4):
            for pc in buckets[q]:
                areas[q] += self._area_quadratic(pc)

        areas["total"] = sum(abs(v) for v in areas.values())
        return areas
