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

use crate::errors::{MosaicError};
use crate::UMD::{UMD, UMDAnchor};

pub struct AnchorProcessor;

impl AnchorProcessor {
    /// We take the extracted coords from the UMD file 
    pub fn calculate_umd_anchors(raw_data: &UMD) -> Result<UMDAnchor, MosaicError> {
        // 
        let total_points = raw_data.frame.len();
        if total_points == 0 {
            return Ok(UMDAnchor::construction(0));
        }
        
        let estimated_frames = (total_points / 68) + 1;
        let mut anchors = UMDAnchor::construction(estimated_frames as u32);

        let mut current_frame = raw_data.frame[0];
        let mut current_timestamp = raw_data.timestamp[0];
        let mut x_sum = 0.0;
        let mut y_sum = 0.0;
        let mut z_sum = 0.0;
        let mut point_count = 0;

        // We iterate through every point within every frame in UMD
        for i in 0..total_points {
            let frame = raw_data.frame[i];

            // If we detect a new frame, save the average of the PREVIOUS frame
            if frame != current_frame {
                let count_f = point_count as f64;
                anchors.add_anchor(
                    current_frame,
                    current_timestamp,
                    x_sum / count_f,
                    y_sum / count_f,
                    z_sum / count_f,

                    // wcount_f is the ammount of points
                );
                println!("Frame: {} X: {:.3}, Y: {:.3}, Z: {:.3}", current_frame, x_sum / count_f, y_sum / count_f, z_sum / count_f);

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
                point_count = 0;
            }

            x_sum += raw_data.x[i];
            y_sum += raw_data.y[i];
            z_sum += raw_data.z[i];
            point_count += 1;
        }

        if point_count > 0 {
            let count_f = point_count as f64;
            anchors.add_anchor(
                current_frame,
                current_timestamp,
                x_sum / count_f,
                y_sum / count_f,
                z_sum / count_f,
            );
            
        }

        

        Ok(anchors)
    }
}