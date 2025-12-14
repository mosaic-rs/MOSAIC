"""
Measures the change in asymmetric change acceleration (basically how fast the mouth moves asymmetrically)

This file does a few things
    1. Measures the velocity of individual landmarks
    2. Measure the velocity of lip curvature
    3. Measure the velocity of area (both types)

- Very useful for detecting things like motor planning errors
"""

from pathlib import Path
from typing import List, Optional
import pandas as pd
from tqdm import tqdm


from mosaic.complex_measurements.temporal.velocity.landmark_velocity import LandmarkVelocity
from mosaic.complex_measurements.temporal.velocity.area_velocity import AreaVelocity
from mosaic.complex_measurements.temporal.velocity.curve_velocity import CurveVelocity
from mosaic.config import OUTER_QUAD_AREA, INNER_QUAD_AREA, OUTER_BIO_AREA, INNER_BIO_AREA, INNER_BEZ_CURVE_LIST, OUTER_BEZ_CURVE_LIST

class run:
    def __init__(self):
       pass


    def run_landmark_velocity(self, input_file: str, output_file: Path, *, force: bool=False, start: int=0, end: Optional[int]=None):
        """
        This function just runs the velocity code for landmark, area, and curves
        """

        df = pd.read_csv(input_file)

        # LANDMARK VELOCITY:
        landmarks = LandmarkVelocity(df)
        print("RUNNING LANDMARK VELOCITY - RUN PAGE")

        total = len(df)
        if end is None or end > total:
            end = total
        if start < 0:
            start = 0
        if start >= end:
            print("Nothing to do: start >= end.")
            return

        landmark_velo = []

        for row in tqdm(range(start, end), desc="Landmark Velocity", unit="frame"):
            try:
                landmark_velo.append(landmarks.landmark_velocity(row))
            except Exception as e:
                print(f"Unknown error: {e}\n")


    def run_curve_velocity(self, input_file: str, output_file: Path, *, force: bool=False, start: int=0, end: Optional[int]=None):
        df = pd.read_csv(input_file)

        # CURVE VELOCITY:
        print("RUNNING CURVE VELOCITY - RUN PAGE")

        total = len(df)
        if end is None or end > total:
            end = total
        if start < 0:
            start = 0
        if start >= end:
            print("Nothing to do: start >= end.")
            return

        curve_velo = []

        for row in tqdm(range(start, end), desc="Curve Velocity", unit="frame"):
            curves = CurveVelocity(df, OUTER_BEZ_CURVE_LIST, row)
            try:
                curves._cubic_coeffs()

            except Exception as e:
                print(f"Unknown error: - HELLO {e}\n")

        print("CURVE VELO RAN")

    def run_area_velocity(self):
        pass

