/*
This file is part of MOSAIC.

MOSAIC is free software: you can redistribute it and/or modify it under 
the terms of the GNU General Public License as published by the Free 
Software Foundation, either version 3 of the License, or any later version.

MOSAIC is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; 
without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR 
PURPOSE. See the GNU General Public License for more details.

You should have received a copy of the GNU General Public License along with 
MOSAIC. If not, see <https://www.gnu.org/licenses/>.
*/

// Pose Correction
// Takes X/Y/Z pose values and uses them to correct landmark points.

// Assumes values are radians 
// Drivers will convert degrees to radians where applicable

// MOSAIC 0.2.1 is sort of made with OpenFace in mind but there will be changes to the UMD 
// make it general purpose by v1.0.0

use crate::UMD::{UMDCentered, UMDPose};
use crate::errors::{MosaicError};
use polars::prelude::*;
use std::fs::File;
//use nalgebra as na; // not used
//use na::{Matrix3, Vector3}; // not used
//use trig::Trig;

pub struct PoseProcessor;

impl PoseProcessor{
    pub fn calculate_pose_corr(raw_centered_data: &UMDCentered) -> Result<UMDPose, MosaicError>{

        /*
            Using UMDPose::construction to reserve the struct size in memory

            We assume 68 points but it is often less
            Keep in mind, this was made with OpenFace in mind to make an MVP
        */
        let total_points = raw_centered_data.frame.len();
        if total_points == 0 {
            return Ok(UMDPose::construction(0, 0));
        }

        let estimated_frames = (total_points as u32 / 68) + 1;
        let mut pose_corr_data = UMDPose::construction(estimated_frames, 68);

        /*
            To correct for pose, an euler rotation matrix is used

            This works even if we are using 2d OpenFace data as we just assume Z = 0

            See https://en.wikipedia.org/wiki/Rotation_matrix for more info
            You can also see pose/matricies.svg to see the exact matricies used
        */

        for i in 0..total_points {
            let current_frame = raw_centered_data.frame[i];

            /* 
                We can now apply the pose correction as each row consistes of an x/y/z coordinate and a corresponding pose value
                in radians.
            */

            let poseX = raw_centered_data.pose_x[i]; // Rx value for frame i
            let poseY = raw_centered_data.pose_y[i]; // Ry value for frame i
            let poseZ = raw_centered_data.pose_z[i]; // Rz value for frame i

            let x = raw_centered_data.x[i];
            let y = raw_centered_data.y[i];
            let z = raw_centered_data.z[i];

            // What is shown below is the post matrix multiplication 
            // I think

            let x_pri = (
                  (poseZ.cos() * poseY.cos() * x)
                + (poseZ.cos() * poseY.sin() * poseX.sin() - poseZ.sin() * poseX.cos() * y)
                + (poseZ.cos() * poseY.sin() * poseX.cos() + poseZ.sin() * poseX.sin() * z)
            );
            let y_pri = (
                  (poseZ.sin() * poseY.cos() * x)
                + (poseZ.sin() * poseY.sin() * poseX.sin() + poseZ.cos() * poseX.cos() * y)
                + (poseZ.sin() * poseY.sin() * poseX.cos() - poseZ.cos() * poseX.sin() * z)
            );
            let z_pri = (
                  (-poseY.sin() * x)
                + (poseY.cos() * poseX.sin() * y)
                + (poseY.cos() * poseX.cos() * z)
            );

            /*
                Important note:

                Even if you are using pure 2d values, this pose correction system will give you a z value greater or less than 0
                depending on the pose values. 

                This does the opposite of what we want when we are pose correcting as it will subtract the "corrected" movement
                from the coordinate under the assumption the Z value is not 0. As a result, there will be a way to make z constantly 0 
                through a metadata section in the UMD. 
            */

            pose_corr_data.add_point(
                current_frame,
                raw_centered_data.timestamp[i],
                0.0, // confidence val will go here,
                raw_centered_data.pose[i],

                // pose vals
                raw_centered_data.pose_x[i],
                raw_centered_data.pose_y[i],
                raw_centered_data.pose_z[i],

                // coord info
                raw_centered_data.coordinate_number[i],
                raw_centered_data.types[i].clone(),
                x_pri,
                y_pri,
                z_pri,
            )

        }
        Ok(pose_corr_data)

    }

    /*
        Test function to export the struct to a parquet so I can visualize the data
    */

    pub fn save_pose_to_parquet(data: &UMDPose, file_path: &str) -> PolarsResult<()> {
        let s_frame = Series::new("frame", &data.frame);
        let s_time = Series::new("timestamp", &data.timestamp);
        let s_pose = Series::new("pose_detected", &data.pose);
        let s_px = Series::new("pose_rx", &data.pose_x);
        let s_py = Series::new("pose_ry", &data.pose_y);
        let s_pz = Series::new("pose_rz", &data.pose_z);
        
        let s_num = Series::new("point_id", &data.coordinate_number);
        let s_type = Series::new("label", &data.types);
        
        let s_x = Series::new("x_centered", &data.x);
        let s_y = Series::new("y_centered", &data.y);
        let s_z = Series::new("z_centered", &data.z);

        let mut df = DataFrame::new(vec![
            s_frame, s_time, s_pose, 
            s_px, s_py, s_pz, 
            s_num, s_type, 
            s_x, s_y, s_z
        ])?;

        let file = File::create(file_path).map_err(PolarsError::from)?;
        ParquetWriter::new(file).finish(&mut df)?;

        println!("Successfully exported pose corrected coordinates to {}", file_path);
        Ok(()) 
    }
}

