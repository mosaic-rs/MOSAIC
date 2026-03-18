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

use crate::UMD::UMD::{UMD};
use polars::prelude::*;
use std::fs::File;

#[derive(Debug, Clone)]
pub struct AreaVelocity {
    pub frame: Vec<u32>,
    pub timestamp: Vec<f32>,
    pub confidence: Vec<f32>,
    pub pose: Vec<bool>, 

    pub coordinate_number: Vec<u32>,
    pub types: Vec<String>, 
    pub va: Vec<f64>,
    pub da: Vec<f64>,
}

impl AreaVelocity {
    pub fn construction(estimated_entries: usize) -> Self {
        Self {
            frame: Vec::with_capacity(estimated_entries),
            timestamp: Vec::with_capacity(estimated_entries),
            confidence: Vec::with_capacity(estimated_entries),
            pose: Vec::with_capacity(estimated_entries),

            coordinate_number: Vec::with_capacity(estimated_entries),
            types: Vec::with_capacity(estimated_entries),

            va: Vec::with_capacity(estimated_entries), 
            da: Vec::with_capacity(estimated_entries), 
        }
    }

    pub fn add_point(
        &mut self, frame: u32, time: f32, confidence: f32, pose: bool,
        number: u32, types: String, va: f64, da: f64
    ) {
        self.frame.push(frame);
        self.timestamp.push(time);
        self.confidence.push(confidence);
        self.pose.push(pose);
        
        self.coordinate_number.push(number);
        self.types.push(types);

        self.va.push(va);
        self.da.push(da);
    }

    pub fn save_area_velocity_to_parquet(data: &AreaVelocity, file_path: &str) -> PolarsResult<()> {
        let s_frame = Series::new("frame", &data.frame);
        let s_time = Series::new("timestamp", &data.timestamp);
        let s_confidence = Series::new("confidence", &data.confidence);
        let s_pose = Series::new("pose_detected", &data.pose);

        let s_num = Series::new("point_id", &data.coordinate_number);
        let s_type = Series::new("label", &data.types);

        let s_va = Series::new("va", &data.va);
        let s_da = Series::new("da", &data.da);

        let mut df = DataFrame::new(vec![
            s_frame, s_time, s_confidence, s_pose,
            s_num, s_type, 
            s_va, s_da,
        ])?;

        let file = File::create(file_path).map_err(PolarsError::from)?;
        ParquetWriter::new(file).finish(&mut df)?;

        println!("Successfully exported Area Velocity data to: {}", file_path);
        Ok(())
    }
}

pub struct CalculateVelocity;

impl CalculateVelocity {
    pub fn velocity(umd: &UMD) -> AreaVelocity {
        let total_points = umd.frame.len();
        if total_points == 0 {
            return AreaVelocity::construction(0);
        }

        let mut points_per_frame = 0;
        let first_frame = umd.frame[0];
        while points_per_frame < total_points && umd.frame[points_per_frame] == first_frame {
            points_per_frame += 1;
        }

        let mut velocity_data = AreaVelocity::construction(total_points);

        for i in 0..total_points {
            let mut va = 0.0;
            let mut da = 0.0;

            if i >= points_per_frame {
                let prev_i = i - points_per_frame;
                let dt = (umd.timestamp[i] - umd.timestamp[prev_i]) as f64;

                if dt > 0.0 {
                    let a_i = umd.area[i];
                    let a_prev = umd.area[prev_i];

                    va = (a_i - a_prev) / dt;

                    let sa_i = if i < umd.area_uncertainty.len() { umd.area_uncertainty[i] } else { 0.0 };
                    let sa_prev = if prev_i < umd.area_uncertainty.len() { umd.area_uncertainty[prev_i] } else { 0.0 };

                    da = ((sa_i.powi(2) + sa_prev.powi(2)) / dt.powi(2)).sqrt();
                }
            }

            velocity_data.add_point(
                umd.frame[i],
                umd.timestamp[i],
                umd.confidence[i],
                umd.pose[i],
                umd.coordinate_number[i],
                umd.types[i].clone(),
                va, da
            );
        }

        velocity_data
    }
}
