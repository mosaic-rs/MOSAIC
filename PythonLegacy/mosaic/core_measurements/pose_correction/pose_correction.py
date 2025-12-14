# pose_correction.py
"""

this file is horrendously messy I do apologize

Centers landmarks and applies roll, yaw, and pitch correction

    Yaw and pitch are correcting 3D rotations on 2D landmarks so there is bound to be higher uncertainty
    Roll is a 2D rotation so ideally it will increase uncertainty less than yaw and pitch

    source: https://en.wikipedia.org/wiki/Rotation_of_axes_in_two_dimensions
            https://en.wikipedia.org/wiki/Rotation_matrix
"""
import pandas as pd
import numpy as np
import math
from typing import Union
from mosaic.config import LANDMARK_PAIRS



class LandmarkCorrection:
    def __init__(self, file: Union[pd.DataFrame, str], row: int, pitch: bool=False, yaw: bool=False, roll: bool=False):
        # NOTE: keep ALL comments; just fix the type checks
        if isinstance(file, str):
            self.data = pd.read_csv(file)
        elif isinstance(file, pd.DataFrame):
            self.data = file
        else:
            raise ValueError("ERROR: file MUST be a string or pd.DataFrame (csv)")
        self.row = int(row)
        self.pitch = bool(pitch)
        self.yaw = bool(yaw)
        self.roll = bool(roll)

    def pose_correction(self, Rx, Ry, Rz):
        """
        For given frame (self.row):
          read Rx, Ry, Rz (assumed radians, like OpenFace)
          apply rotation to each landmark pair in LANDMARK_PAIRS
          return list of (x', y') in the SAME order as LANDMARK_PAIRS
        """
        # Getting data
        rowdata = self.data.iloc[self.row]

        corrected = []
        for col1, col2 in LANDMARK_PAIRS:
            x_val = float(rowdata[col1])
            y_val = float(rowdata[col2])

            XY_Pri = self._rotation(x_val, y_val, Rx, Ry, Rz)
            # print(f"X: {XY_Pri[0]} ----- Y: {XY_Pri[1]}")
            corrected.append((float(XY_Pri[0]), float(XY_Pri[1])))

        return corrected  # list[(x', y')]

    def _yaw_correction(self, x, y, Rz):
        """
        Really only affects the X value in yaw

        Formula: x' = xcos(θ) + ysin(θ)

        returns (x', y)
        """
        X_pri = (x * math.cos(Rz)) + (y * math.sin(Rz))
        return X_pri

    def _pitch_correction(self, x, y, Ry):
        """
        Really only Y is affected by Pitch
        OpenFace provides: Rx, Ry, and Rz in radians

        formula: y' = -xsin(θ) + ycos(θ)

        :return: (x,y')
        """
        Y_pri = ((-x) * math.sin(Ry)) + (y * math.cos(Ry))  # fix missing operator
        return Y_pri

    def _roll_correction(self, x, y, Rx):
        """
        Affects both X and Y values in roll

        I expect low uncertainty as it is a 2d rotation

        :return: (x', y')
        """
        X_pri = (x * math.cos(Rx)) + (y * math.sin(Rx))
        Y_pri = ((-x) * math.sin(Rx)) + (y * math.cos(Rx))
        return X_pri, Y_pri

    def _rotation(self, x, y, Rx, Ry, Rz):
        """
        θ: Roll, ɸ: Pitch, ⍺: Yaw

        Make Z = 0

        :return: (x', y')

        """

        """
        (𝛄) Rx = np.array([[1, 0, 0],
                       [0, math.cos(Rx), (-1 * math.sin(Rx))],
                       [0, math.sin(Rx), math.cos(Rx)]])

        (β) Ry = np.array([[math.cos(Ry), 0, math.sin(Ry)],
                       [0, 1, 0],
                       [(-1 * math.sin(Ry)), 0, math.cos(Ry)]])

        (⍺) Rz = np.array([[math.cos(Rz), (-1 * math.sin(Rz)), 0],
                       [math.sin(Rz), math.cos(Rz), 0],
                       [0, 0, 1]])

        R = Rz (⍺) * Ry (β) * Rx (𝛄)
        
        R = np.array([[(math.cos(Rz)*math.cos(Ry)), ((math.sin(Rz)*math.sin(Ry)*math.cos(Rx)) - (math.sin(Rz)*math.cos(Rx))), ((math.cos(Rz)*math.sin(Ry)*math.cos(Rx)) + (math.sin(Rz)*math.sin(Rx)))],
                      [(math.sin(Rz)*math.cos(Ry)), ((math.sin(Ry)*math.sin(Rx)*math.sin(Rx)) + (math.cos(Rz)*math.cos(Rx))), ((math.sin(Rz)*math.sin(Ry)*math.cos(Rx)) - (math.cos(Rz)*math.sin(Rx)))],
                      [(-1*math.sin(Ry)), (math.cos(Ry)*math.sin(Rx)), (math.cos(Rz)*math.cos(Rx))]])
                      
        Different type:
        
            cosz = math.cos(Rz)
            sinz = math.sin(Rz)
            cosy = math.cos(Ry)
            siny = math.sin(Ry)
            cosx = math.cos(Rx)
            sinx = math.sin(Rx)

            # Construct the matrix
            R = np.array([
                [ cosz * cosy,  cosz * siny * sinx - sinz * cosx,  cosz * siny * cosx + sinz * sinx],
                [ sinz * cosy,  sinz * siny * sinx + cosz * cosx,  sinz * siny * cosx - cosz * sinx],
                [      -siny,                   cosy * sinx,                   cosy * cosx]
                
        R = np.array([[math.cos(Rz)*math.cos(Ry), math.cos(Rz)*math.sin(Ry)*math.sin(Rx) - math.sin(Rz)*math.cos(Rx), math.cos(Rz)*math.sin(Ry)*math.cos(Rx) + math.sin(Rz)*math.sin(Rx)],
                      [math.sin(Rz)*math.cos(Ry), math.sin(Rz)*math.sin(Ry)*math.sin(Rx) + math.cos(Rz)*math.cos(Rx), math.sin(Rz)*math.sin(Ry)*math.cos(Rx) - math.cos(Rz)*math.sin(Rx)],
                      [-math.sin(Ry), math.cos(Ry)*math.sin(Rx), math.cos(Ry)*math.cos(Rx)]
        ])
        """

        # Construct the matrix
        R = np.array([[math.cos(Rz)*math.cos(Ry), math.cos(Rz)*math.sin(Ry)*math.sin(Rx) - math.sin(Rz)*math.cos(Rx), math.cos(Rz)*math.sin(Ry)*math.cos(Rx) + math.sin(Rz)*math.sin(Rx)],
        [math.sin(Rz)*math.cos(Ry), math.sin(Rz)*math.sin(Ry)*math.sin(Rx) + math.cos(Rz)*math.cos(Rx), math.sin(Rz)*math.sin(Ry)*math.cos(Rx) - math.cos(Rz)*math.sin(Rx)],
        [-math.sin(Ry), math.cos(Ry)*math.sin(Rx), math.cos(Ry)*math.cos(Rx)]
        ])


        # Apply the composed rotation to (x, y, z=0) and project back to XY
        X_pri = (
                (math.cos(Rz) * math.cos(Ry)) * x
                + (math.cos(Rz) * math.sin(Ry) * math.sin(Rx) - math.sin(Rz) * math.cos(Rx)) * y
                + 0.0  # z term is zero
        )

        Y_pri = (
                (math.sin(Rz) * math.cos(Ry)) * x
                + (math.sin(Rz) * math.sin(Ry) * math.sin(Rx) + math.cos(Rz) * math.cos(Rx)) * y
                + 0.0  # z term is zero
        )

        # Z_pri = 0  # Z is ALWAYS = 0 - noted out cuz it is irrelevant but wrote it so I remember it

        return X_pri, Y_pri
