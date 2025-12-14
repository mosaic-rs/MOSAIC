
import pandas as pd
from pathlib import Path
from mosaic.core_measurements.landmarks.calibration_and_uncertainty.landmark_uncertainty import LandmarkCalibration
from mosaic.Session.session_manager import SessionManagement
import os

class XYUncertainty:
    def __init__(self):
        self.session = SessionManagement()

    def run(self, frames_index: pd.Index):
        updates = pd.DataFrame(index=frames_index)


        landmark_video = "/Users/harrywoodhouse/Desktop/MOSAIC/MOSAIC-Engine/data/test-data/v15044gf0000d1dlc67og65r2deqmhd0.mp4" # hardcoded for now but will change
        output_file = os.path.join(self.session._read_session().get("currentTrial"), "calibration")

        calibration = LandmarkCalibration(landmark_video, output_file)

        SD = calibration.calibrate()

        for i in range(48, 68):
            try:
                updates[f"x_{i}_unc"] = SD[f"x_{i}"]
                updates[f"y_{i}_unc"] = SD[f"y_{i}"]
            except KeyError:
                raise KeyError(f"Missing uncertainty value for landmark {i}. Calibration output incomplete.")
        return updates
