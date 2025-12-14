"""
This is the most important command file as it runs openface

It has calibration as an option
"""

# CORE MEASUREMENTS

from mosaic.core_measurements.landmarks.landmarks import run as run_landmarks
from mosaic.core_measurements.anchor.anchors import run as run_anchors
from mosaic.core_measurements.angles.angles import run as run_angles
from mosaic.core_measurements.area.bio_based_area.bio_area_run import run as run_bio_area
from mosaic.core_measurements.area.quadrant_based_area.quadrant_area_run import run as run_quad_area
from mosaic.core_measurements.centering.centering import run as run_centering
from mosaic.core_measurements.curves.curves import export_curve_coefficients as run_curve_fitting
from mosaic.core_measurements.euclidean_distance.euclidean import run as run_euclidean_distance
from mosaic.core_measurements.pose_correction.pose import run as run_pose_correction

# COMPLEX MEASUREMENTS

from mosaic.complex_measurements.temporal.velocity.velocity import run as run_velocity

import os
from pathlib import Path
from mosaic.Session.session_manager import SessionManagement
from mosaic.io import initialize_csvs

class TrialRun:
    def __init__(self):
        # to get trial path we have to confirm they are in a trial
        self.session = SessionManagement

        self.session.require_trial()
        self.trial_path = self.session._read_session().get("currentTrial")
        print("RUNNING TRIAL THINGY THING")

    @staticmethod
    def _check_calibration(trial_path=None):
        """I think for now caliration will be an optional thing where we will check for calibratuon and inform the user
        that if they do not calibrate, X and Y SD will be set at 0.5

        for now, so I can get back to coding OAS, it will not check calibration and will assume 0.5px for x/y SD"""

        return "CALIBRATED"

    @staticmethod
    def _run_oas(trial_path):

        """
        I NEED TO ADD THE OPTION TO HAVE POSE AND FORCE BUT FOR NOW I WILL HAVE THEM AS CONST TRUE
        trial_path = outpath

        also the input file is hard coded just so I can test it

        """

        file = "/Users/harrywoodhouse/Desktop/MOSAIC/MOSAIC-Engine/data/test-data/v15044gf0000d1dlc67og65r2deqmhd0.csv"
        file_name = os.path.basename(file).split('.')[0]
        trial_path = Path(trial_path)
        out_path = initialize_csvs(file_name, trial_path, force=True)

        """CORE MEASUREMENTS
        
        """

        # ANCHOR MODULE

        run_anchors(file, out_path, pose_corr=True, force=True)

        # RUNNING LANDMARK UNCERTAINTY

        run_landmarks(file, out_path, force=True)

        # RUNNING CENTERING MODULE

        run_centering(file, out_path, force=True, pose_corr=True)

        # RUNNING POSE CORRECTION

        #if args.pose == True:
        run_pose_correction(out_path, out_path, force=True)

        # RUNNING EUCLIDEAN DISTANCE CALC

        run_euclidean_distance(out_path, out_path, force=True)

        # RUNNING ANGLE CALC

        run_angles(out_path, out_path, force=True)

        # RUNNING CURVE FITTING

        run_curve_fitting(out_path, out_path, force=True)

        # RUNNING QUADRANT BASED AREA

        run_quad_area(out_path, out_path, force=True)

        # RUNNING BIO BASED AREA

        run_bio_area(out_path, out_path, force=True)


        """
        COMPLEX MEASUREMENTS
        """

        velocity = run_velocity()
        print("RUNNIN LANDMARK VELOCITY")
        velocity.run_landmark_velocity(out_path, out_path, force=True)

        print("RUNNIN CURVE VELOCITY")

        velocity.run_curve_velocity(out_path, out_path, force=True)






