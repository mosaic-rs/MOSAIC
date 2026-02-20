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

// Centers coordinates
// Runs after anchor

// Accesses the UMD and Anchor data to center all points to the origin

use crate::errors::{MosaicError};
use crate::UMD::UMD::{UMDDriver, UMDAnchor, UMDCentered};

use polars::prelude::*;
use std::fs::File;

pub struct CenteringProcessor;

pub struct CenterUncertainty {
    pub x_uncertainty: f64,
    pub y_uncertainty: f64,
    pub z_uncertainty: f64,
}

impl CenterUncertainty{
    pub fn UncertaintyProcessor(_x_uncertainty: f64, _y_uncertainty: f64, _z_uncertainty: f64, 
                                _x_anchor_uncertainty: f64, _y_anchor_uncertainty: f64, _z_anchor_uncertainty: f64) {
        // will calculate the uncertainty of the new centered x/y/z values
    }
}

impl CenteringProcessor {
    pub fn calculate_centering(raw_coord_data: &UMDDriver, raw_anchor_data: &UMDAnchor) -> Result<UMDCentered, MosaicError>{
        let total_points = raw_coord_data.frame.len();
        if total_points == 0 {
            return Ok(UMDCentered::construction(0, 0));
        }

        // estimating frame count for UMDCenter::construction
        let estimated_frames = (total_points as u32 / 68) + 1;
        let mut centered_data = UMDCentered::construction(estimated_frames, 68);

        let mut anchor_id = 0;
        let total_anchors = raw_anchor_data.frame.len(); // gets total amount of anchors 

        for i in 0..total_points {
            let current_frame = raw_coord_data.frame[i];

            while anchor_id < total_anchors && raw_anchor_data.frame[anchor_id] < current_frame {
                // basically checks if frame for anchor is less than the frame for the points
                // and if it is lower, increase by 1
                anchor_id += 1;
            }

            if anchor_id < total_anchors && raw_anchor_data.frame[anchor_id] == current_frame { // just checks that current anchor frame == current point frame

                // we gotta get the anchor coords for the current frame
                let x_anchor = raw_anchor_data.x_anchor[anchor_id];
                let y_anchor = raw_anchor_data.y_anchor[anchor_id];
                let z_anchor = raw_anchor_data.z_anchor[anchor_id];

                // after we calculate the centered landmarks
                // by subracting the correspondong anchor from the umd value

                let x_centered = raw_coord_data.x[i] - x_anchor;
                let y_centered = raw_coord_data.y[i] - y_anchor;
                let z_centered = raw_coord_data.z[i] - z_anchor;

                // temporarily defininf uncertainty vals - will be replaced by CenterUncertainty struct
                let x_uncertainty: f64 = 0.0;
                let y_uncertainty: f64 = 0.0;
                let z_uncertainty: f64 = 0.0;

                // passing to struct 

                centered_data.add_point(
                    current_frame,
                    raw_coord_data.timestamp[i],
                    0.0, // confidence val will go here,
                    raw_coord_data.pose[i],

                    // pose vals
                    raw_coord_data.pose_x[i],
                    raw_coord_data.pose_y[i],
                    raw_coord_data.pose_z[i],

                    // coord info
                    raw_coord_data.coordinate_number[i],
                    raw_coord_data.types[i].clone(),
                    x_centered,
                    y_centered,
                    z_centered,
                    x_uncertainty,
                    y_uncertainty,
                    z_uncertainty,

                )
            }
        }
        Ok(centered_data)
    }

    pub fn save_centered_to_parquet(data: &UMDCentered, file_path: &str) -> PolarsResult<()> {
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

        println!("Successfully exported centered coordinates to {}", file_path);
        Ok(())
    }
}