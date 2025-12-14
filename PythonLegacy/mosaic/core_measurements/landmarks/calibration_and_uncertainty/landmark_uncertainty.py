import os
import shutil
import subprocess
import pandas as pd
from pathlib import Path


class LandmarkCalibration:
    """
    Runs OpenFace on a calibration video and computes the SD of
    all x/y facial landmarks (48–67 inclusive).
    """

    OPENFACE_PATH = "/Users/harrywoodhouse/OpenFace/build/bin/FeatureExtraction" # I will definitely make this a custom dir and probs store it in project metadata

    def __init__(self, video_path: str, output_dir: str):
        video_path = Path(video_path)
        output_dir = Path(output_dir)

        if not video_path.exists():
            raise FileNotFoundError(f"{video_path} does not exist.")

        self.video_path = video_path
        self.output_dir = output_dir

    def run_openface(self) -> Path:

        calib_dir = self.output_dir / "calibration"
        calib_dir.mkdir(parents=True, exist_ok=True)

        # Copy video into calibration directory
        local_vid = calib_dir / self.video_path.name
        shutil.copy2(self.video_path, local_vid)

        command = [
            self.OPENFACE_PATH,
            "-f", str(local_vid),
            "-out_dir", str(calib_dir),
            "-2Dfp", "-tracked"
        ]

        print(f"Running OAS calibration via OpenFace on: {local_vid}")
        result = subprocess.run(command, stdout=subprocess.PIPE, stderr=subprocess.PIPE)

        if result.returncode != 0:
            raise RuntimeError(
                f"OpenFace failed:\n{result.stderr.decode()}"
            )

        # Now we gotta find the CSV file that OpenFace gave
        csv_files = list(calib_dir.glob("*.csv"))
        if not csv_files:
            raise FileNotFoundError("No OpenFace CSV output found in calibration directory.")

        return csv_files[0]

    @staticmethod
    def compute_sd(csv_path: Path) -> dict:
        # now that we have ran openface we can actually get the SD of each x/y val and then return it as a dict
        df = pd.read_csv(csv_path)

        sd_dict = {}
        for lm in range(48, 68):  # inclusive 48-67
            x_col = f"x_{lm}"
            y_col = f"y_{lm}"

            if x_col not in df.columns or y_col not in df.columns:
                raise ValueError(f"Missing {x_col}/{y_col} in CSV.")

            sd_dict[x_col] = df[x_col].std()
            sd_dict[y_col] = df[y_col].std()

        return sd_dict

    def calibrate(self) -> dict:

        csv_file = self.run_openface()
        return self.compute_sd(csv_file)
