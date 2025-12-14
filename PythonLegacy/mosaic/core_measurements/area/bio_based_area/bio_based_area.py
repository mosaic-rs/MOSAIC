"""

This is the simplest area calculator because it is based off the start/end points of the curves

It calculates the area of the mouth based on the commissures and philtrum to determine 4 up/down left/right quadrants

Why this is important: because it shows which part of the mouth is open **more** or **less** than other parts. However,
what separates it from the quadrant based area is that it does not show the overall direction of the mouth. I think.

"""

from __future__ import annotations
from pathlib import Path
from typing import Sequence
import numpy as np
import pandas as pd

EPS = 1e-12

class BioArea:
    """
    This one works a little differently than the quad based area because we have one class for both the inner
    and outer curves.
    """
    def __init__(self, src: str | Path | pd.DataFrame):
        if isinstance(src, (str, Path)):
            self.landmarks = pd.read_csv(src)
        elif isinstance(src, pd.DataFrame):
            self.landmarks = src
        else:
            raise TypeError("landmarks must be a CSV path or a pandas DataFrame")


    def _lm(self, row, idx):
        return np.array([
            self.landmarks.loc[row, f"x_{idx}"],
            self.landmarks.loc[row, f"y_{idx}"]
        ], dtype=float)

    @staticmethod
    def _landmark_extraction(LC, RC, PH, LM):
        LC = np.asarray(LC, float)
        RC = np.asarray(RC, float)
        PH = np.asarray(PH, float)
        LM = np.asarray(LM, float)

        v1 = LC - RC
        v2 = LM - PH

        origin = np.array([0.0, 0.0])

        B = np.column_stack((v1, v2))

        Binv = np.linalg.inv(B)
        scale = abs(np.linalg.det(B))

        return origin, Binv, scale

    @staticmethod
    def _to_bio(P, origin, Binv):
        return Binv @ (np.asarray(P, float) - origin)

    @staticmethod
    def _cross(a, b):
        return a[0] * b[1] - a[1] * b[0]

    @staticmethod
    def _which_quadrant(pt):
        x, y = pt
        if abs(x) <= EPS or abs(y) <= EPS:
            return None
        if x > 0 and y > 0: return 1
        if x < 0 and y > 0: return 2
        if x < 0 and y < 0: return 3
        if x > 0 and y < 0: return 4

    @staticmethod
    def _midpoint_cubic(P):
        P0, P1, P2, P3 = [np.asarray(p, float) for p in P]
        t = 0.5
        return ((1 - t)**3)*P0 + 3*((1 - t)**2)*t*P1 + 3*(1 - t)*(t**2)*P2 + (t**3)*P3

    @staticmethod
    def _midpoint_quadratic(P):
        P0, P1, P2 = [np.asarray(p, float) for p in P]
        t = 0.5
        return ((1 - t)**2)*P0 + 2*(1 - t)*t*P1 + (t**2)*P2

    @staticmethod
    def _area_cubic(P):
        P0, P1, P2, P3 = [np.asarray(p, float) for p in P]
        c = BioArea._cross
        return (1/20.0)*(
            6*c(P0, P1) + 3*c(P0, P2) + c(P0, P3) +
            3*c(P1, P2) + 3*c(P1, P3) + 6*c(P2, P3)
        )

    @staticmethod
    def _area_quadratic(P):
        P0, P1, P2 = [np.asarray(p, float) for p in P]
        c = BioArea._cross
        return (1/6.0)*(2*c(P0, P1) + c(P0, P2) + 2*c(P1, P2))

    @staticmethod
    def _split_cubic(P, t):
        P0, P1, P2, P3 = [np.asarray(p, float) for p in P]
        P01 = (1 - t)*P0 + t*P1
        P12 = (1 - t)*P1 + t*P2
        P23 = (1 - t)*P2 + t*P3
        P012 = (1 - t)*P01 + t*P12
        P123 = (1 - t)*P12 + t*P23
        P0123 = (1 - t)*P012 + t*P123
        return [P0, P01, P012, P0123], [P0123, P123, P23, P3]

    @staticmethod
    def _split_quadratic(P, t):
        P0, P1, P2 = [np.asarray(p, float) for p in P]
        P01 = (1 - t)*P0 + t*P1
        P12 = (1 - t)*P1 + t*P2
        P012 = (1 - t)*P01 + t*P12
        return [P0, P01, P012], [P012, P12, P2]

    @staticmethod
    def _roots_quadratic(P, axis=0):
        c0, c1, c2 = [float(p[axis]) for p in P]
        a = c0 - 2*c1 + c2
        b = 2*(c1 - c0)
        c = c0
        if abs(a) < 1e-12 and abs(b) < 1e-12:
            return []
        r = np.roots([a, b, c])
        r = r[np.isreal(r)].real
        return [t for t in r if EPS < t < 1 - EPS]

    @staticmethod
    def _roots_cubic(P, axis=0):
        c0, c1, c2, c3 = [float(p[axis]) for p in P]

        d = c0
        c = 3*(c1 - c0)
        b = 3*(c2 - 2*c1 + c0)
        a = c3 - 3*c2 + 3*c1 - c0

        if abs(a) < 1e-18 and abs(b) < 1e-18 and abs(c) < 1e-18:
            return []

        r = np.roots([a, b, c, d])
        r = r[np.isreal(r)].real
        return [t for t in r if EPS < t < 1 - EPS]

    def _split_at_axes_cubic(self, P):
        roots = self._roots_cubic(P, 0) + self._roots_cubic(P, 1)
        roots = sorted(set(round(t, 10) for t in roots))
        if not roots:
            return [P]
        segs = []
        cur = P
        prev = 0.0
        for t in roots + [1.0]:
            local = (t - prev) / (1 - prev)
            L, R = self._split_cubic(cur, local)
            segs.append(L)
            cur = R
            prev = t
        return segs

    def _split_at_axes_quadratic(self, P):
        roots = self._roots_quadratic(P, 0) + self._roots_quadratic(P, 1)
        roots = sorted(set(round(t, 10) for t in roots))
        if not roots:
            return [P]
        segs = []
        cur = P
        prev = 0.0
        for t in roots + [1.0]:
            local = (t - prev) / (1 - prev)
            L, R = self._split_quadratic(cur, local)
            segs.append(L)
            cur = R
            prev = t
        return segs

    # Gives us (x', y') because it rotates and shifts it to the "new" x and y axis
    def _to_bio(self, P, origin, Binv):
        P = np.asarray(P, float)
        return Binv @ (P - origin)

    def compute_outer_bio(self, row, cubics_ccw):
        LC = self._lm(row, 48)
        RC = self._lm(row, 54)
        PH = self._lm(row, 51)
        LM = self._lm(row, 57)
        origin, Binv, scale = self._landmark_extraction(LC, RC, PH, LM)

        #print("ROW", row)
        #print("LC:", LC, "RC:", RC)
        #print("PH:", PH, "LM:", LM)
        #print("v1:", v1, "||v1||:", np.linalg.norm(v1))
        #print("v2:", v2, "||v2||:", np.linalg.norm(v2))

        cubics_bio = []
        for seg in cubics_ccw:
            P0, P1, P2, P3 = seg
            cubics_bio.append([
                self._to_bio(P0, origin, Binv),
                self._to_bio(P1, origin, Binv),
                self._to_bio(P2, origin, Binv),
                self._to_bio(P3, origin, Binv),
            ])

        pieces = []
        for seg in cubics_bio:
            parts = self._split_at_axes_cubic(seg)
            pieces.extend(self._split_at_axes_cubic(seg))

        buckets = {1: [], 2: [], 3: [], 4: []}

        for pc in pieces:
            mid = self._midpoint_cubic(pc)
            q = self._which_quadrant(self._midpoint_cubic(pc))
            if q:
                buckets[q].append(pc)

        areas = {1: 0.0, 2: 0.0, 3: 0.0, 4: 0.0}

        for q in (1, 2, 3, 4):
            for pc in buckets[q]:
                areas[q] += self._area_cubic(pc)

        for q in (1, 2, 3, 4):
            areas[q] *= scale

        areas["total"] = sum(abs(areas[q]) for q in (1, 2, 3, 4))

        return areas

    # Bio inner area now which is basicallt the same just less points

    def compute_inner_bio(self, row, quads_ccw):
        LC = self._lm(row, 48)
        RC = self._lm(row, 54)
        PH = self._lm(row, 51)
        LM = self._lm(row, 57)
        origin, Binv, scale = self._landmark_extraction(LC, RC, PH, LM)

        quads_bio = []
        for seg in quads_ccw:
            P0, P1, P2 = seg
            quads_bio.append([
                self._to_bio(P0, origin, Binv),
                self._to_bio(P1, origin, Binv),
                self._to_bio(P2, origin, Binv),
            ])

        pieces = []
        for seg in quads_bio:
            pieces.extend(self._split_at_axes_quadratic(seg))


        buckets = {1: [], 2: [], 3: [], 4: []}
        for pc in pieces:
            mid = self._midpoint_quadratic(pc)
            q = self._which_quadrant(self._midpoint_quadratic(pc))
            if q:
                buckets[q].append(pc)

        areas = {1: 0.0, 2: 0.0, 3: 0.0, 4: 0.0}
        for q in (1, 2, 3, 4):
            for pc in buckets[q]:
                areas[q] += self._area_quadratic(pc)

        for q in (1, 2, 3, 4):
            areas[q] *= scale

        areas["total"] = sum(abs(areas[q]) for q in (1, 2, 3, 4))

        return areas

