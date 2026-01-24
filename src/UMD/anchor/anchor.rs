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


/*
ANCHOR Calculation

This file calculates the anchor as a reference point in the mouth by taking the average x and y coordinates
of points across the mouth and defining a centralised anchor point
*/

// FOR NOW - X/Y/Z SD WILL BE HARDCODED AT 0.5
// ONCE WE ESTABLISH A CALIBRATION SYSTEM, THIS WILL CHANGE
//const x_sd: f64 = 0.5;
//const y_sd: f64 = 0.5;
//const z_sd: f64 = 0.5;

use crate::errors::{MosaicError};
use crate::UMD::UMD::{UMDDriver, UMDAnchor};

use polars::prelude::*;
use std::fs::File;

pub struct AnchorProcessor;

impl AnchorProcessor {
    /// We take the extracted coords from the UMD file 
    pub fn calculate_umd_anchors(raw_data: &UMDDriver) -> Result<UMDAnchor, MosaicError> {
        // 
        let total_points = raw_data.frame.len();
        if total_points == 0 {
            return Ok(UMDAnchor::construction(0));
        }
        
        let estimated_frames = (total_points / 68) + 1;
        let mut anchors = UMDAnchor::construction(estimated_frames as u32);

        let mut current_frame = raw_data.frame[0];
        let mut current_timestamp = raw_data.timestamp[0];
        let mut x_sum: f64 = 0.0;
        let mut y_sum: f64 = 0.0;
        let mut z_sum: f64 = 0.0;
        let mut x_unc_sum: f64 = 0.0;
        let mut z_unc_sum: f64 = 0.0;
        let mut y_unc_sum: f64 = 0.0;
        let mut point_count: u32 = 0;

        // We iterate through every point within every frame in UMD
        for i in 0..total_points {
            let frame = raw_data.frame[i];

            // If we detect a new frame, save the average of the PREVIOUS frame
            if frame != current_frame {
                let count_f = point_count as f64;
                let sqrt_n = count_f.sqrt();

                // anchor uncertainty 
                let x_anchor_uncertainty = (x_unc_sum / count_f) / sqrt_n;
                let y_anchor_uncertainty = (y_unc_sum / count_f) / sqrt_n;
                let z_anchor_uncertainty = (z_unc_sum / count_f) / sqrt_n;

                anchors.add_anchor(
                    current_frame,
                    current_timestamp,
                    x_sum / count_f,
                    y_sum / count_f,
                    z_sum / count_f,
                    x_anchor_uncertainty,
                    y_anchor_uncertainty,
                    z_anchor_uncertainty,

                    // count_f is the ammount of points
                );
                //println!("Frame: {} X: {:.3}, Y: {:.3}, Z: {:.3}", current_frame, x_sum / count_f, y_sum / count_f, z_sum / count_f);

                /*if current_frame == 10 {
                    let avg_x = x_sum / count_f;
                    let avg_y = y_sum / count_f;
                    let avg_z = z_sum / count_f;

                    println!("--- ANCHOR CALCULATION [Frame {}] ---", current_frame);
                    println!("Points Averaged: {}", point_count);
                    println!("Resulting Anchor: X: {:.3}, Y: {:.3}, Z: {:.3}", avg_x, avg_y, avg_z);
                    println!("--------------------------------------");
                }*/

                // makes the points = 0 for new frame
                current_frame = frame;
                current_timestamp = raw_data.timestamp[i];
                x_sum = 0.0;
                y_sum = 0.0;
                z_sum = 0.0;
                x_unc_sum = 0.0;
                y_unc_sum = 0.0;
                z_unc_sum = 0.0;
                point_count = 0;
            }

            x_sum += raw_data.x[i];
            y_sum += raw_data.y[i];
            z_sum += raw_data.z[i];
            x_unc_sum += raw_data.x_uncertainty[i];
            y_unc_sum += raw_data.x_uncertainty[i];
            z_unc_sum += raw_data.x_uncertainty[i];       
            point_count += 1;
        }

        if point_count > 0 {
            let count_f = point_count as f64;

            // getting uncertainty:
            
            let sqrt_n = count_f.sqrt();
            // anchor uncertainty
            let x_anchor_uncertainty = (x_unc_sum / count_f) / sqrt_n;
            let y_anchor_uncertainty = (y_unc_sum / count_f) / sqrt_n;
            let z_anchor_uncertainty = (z_unc_sum / count_f) / sqrt_n;

            anchors.add_anchor(
                current_frame,
                current_timestamp,
                x_sum / count_f,
                y_sum / count_f,
                z_sum / count_f,
                x_anchor_uncertainty,
                y_anchor_uncertainty,
                z_anchor_uncertainty,
            );
            
        }

        

        Ok(anchors)
    }

    pub fn save_anchors_to_parquet(data: &UMDAnchor, file_path: &str) -> PolarsResult<()> {
        let s0 = Series::new("frame", &data.frame);
        let s1 = Series::new("timestamp", &data.timestamp);
        let s2 = Series::new("x_anchor", &data.x_anchor);
        let s3 = Series::new("y_anchor", &data.y_anchor);
        let s4 = Series::new("z_anchor", &data.z_anchor);
        let s5 = Series::new("x_anchor_uncertainty", &data.x_anchor_uncertainty);
        let s6 = Series::new("y_anchor_uncertainty", &data.y_anchor_uncertainty);
        let s7 = Series::new("z_anchor_uncertainty", &data.z_anchor_uncertainty);

        // Making the df
        let mut df = DataFrame::new(vec![s0, s1, s2, s3, s4, s5, s6, s7])?;

        let file = File::create(file_path).map_err(PolarsError::from)?;
        ParquetWriter::new(file).finish(&mut df)?;

        println!("Saved anchors to {}", file_path);
        Ok(())
}
}