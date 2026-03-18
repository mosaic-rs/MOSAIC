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
pub struct LandmarkVelocity {
    pub frame: Vec<u32>,
    pub timestamp: Vec<f32>,
    pub confidence: Vec<f32>,
    pub pose: Vec<bool>, 

    pub coordinate_number: Vec<u32>,
    pub types: Vec<String>, 
    pub vx: Vec<f64>,
    pub vy: Vec<f64>,
    pub vz: Vec<f64>,
    pub dx: Vec<f64>,
    pub dy: Vec<f64>,
    pub dz: Vec<f64>,
}

impl LandmarkVelocity {
    pub fn construction(estimated_entries: usize) -> Self {
        Self {
            frame: Vec::with_capacity(estimated_entries),
            timestamp: Vec::with_capacity(estimated_entries),
            confidence: Vec::with_capacity(estimated_entries),
            pose: Vec::with_capacity(estimated_entries),

            coordinate_number: Vec::with_capacity(estimated_entries),
            types: Vec::with_capacity(estimated_entries),

            vx: Vec::with_capacity(estimated_entries), 
            vy: Vec::with_capacity(estimated_entries), 
            vz: Vec::with_capacity(estimated_entries), 

            dx: Vec::with_capacity(estimated_entries), 
            dy: Vec::with_capacity(estimated_entries), 
            dz: Vec::with_capacity(estimated_entries), 
        }
    }

    pub fn add_point(
        &mut self, frame: u32, time: f32, confidence: f32, pose: bool,
        number: u32, types: String, vx: f64, vy: f64, vz: f64, dx: f64, dy: f64, dz: f64
    ) {
        self.frame.push(frame);
        self.timestamp.push(time);
        self.confidence.push(confidence);
        self.pose.push(pose);
        
        self.coordinate_number.push(number);
        self.types.push(types);

        self.vx.push(vx);
        self.vy.push(vy);
        self.vz.push(vz);
        self.dx.push(dx);
        self.dy.push(dy);
        self.dz.push(dz);
    }

    pub fn save_landmark_velocity_to_parquet(data: &LandmarkVelocity, file_path: &str) -> PolarsResult<()> {
        let s_frame = Series::new("frame", &data.frame);
        let s_time = Series::new("timestamp", &data.timestamp);
        let s_confidence = Series::new("confidence", &data.confidence);
        let s_pose = Series::new("pose_detected", &data.pose);

        let s_num = Series::new("point_id", &data.coordinate_number);
        let s_type = Series::new("label", &data.types);

        let s_vx = Series::new("vx", &data.vx);
        let s_vy = Series::new("vy", &data.vy);
        let s_vz = Series::new("vz", &data.vz);

        let s_dx = Series::new("dx", &data.dx);
        let s_dy = Series::new("dy", &data.dy);
        let s_dz = Series::new("dz", &data.dz);

        let mut df = DataFrame::new(vec![
            s_frame, s_time, s_confidence, s_pose,
            s_num, s_type, 
            s_vx, s_vy, s_vz,
            s_dx, s_dy, s_dz,
        ])?;

        let file = File::create(file_path).map_err(PolarsError::from)?;
        ParquetWriter::new(file).finish(&mut df)?;

        println!("Successfully exported Landmark Velocity data to: {}", file_path);
        Ok(())
    }
}

pub struct CalculateVelocity;

impl CalculateVelocity {
    pub fn velocity(umd: &UMD) -> LandmarkVelocity {
        let total_points = umd.frame.len();
        if total_points == 0 {
            return LandmarkVelocity::construction(0);
        }

        let mut points_per_frame = 0;
        let first_frame = umd.frame[0];
        while points_per_frame < total_points && umd.frame[points_per_frame] == first_frame {
            points_per_frame += 1;
        }

        let mut velocity_data = LandmarkVelocity::construction(total_points);

        for i in 0..total_points {
            let mut vx = 0.0;
            let mut vy = 0.0;
            let mut vz = 0.0;
            let mut dx = 0.0;
            let mut dy = 0.0;
            let mut dz = 0.0;

            if i >= points_per_frame {
                let prev_i = i - points_per_frame;
                let dt = (umd.timestamp[i] - umd.timestamp[prev_i]) as f64;

                if dt > 0.0 {
                    let x_i = umd.x_rotated[i];
                    let y_i = umd.y_rotated[i];
                    let z_i = umd.z_rotated[i];

                    let x_prev = umd.x_rotated[prev_i];
                    let y_prev = umd.y_rotated[prev_i];
                    let z_prev = umd.z_rotated[prev_i];

                    vx = (x_i - x_prev) / dt;
                    vy = (y_i - y_prev) / dt;
                    vz = (z_i - z_prev) / dt;

                    let sx_i = if i < umd.x_rotated_uncertainty.len() { umd.x_rotated_uncertainty[i] } else { 0.0 };
                    let sy_i = if i < umd.y_rotated_uncertainty.len() { umd.y_rotated_uncertainty[i] } else { 0.0 };
                    let sz_i = if i < umd.z_rotated_uncertainty.len() { umd.z_rotated_uncertainty[i] } else { 0.0 };

                    let sx_prev = if prev_i < umd.x_rotated_uncertainty.len() { umd.x_rotated_uncertainty[prev_i] } else { 0.0 };
                    let sy_prev = if prev_i < umd.y_rotated_uncertainty.len() { umd.y_rotated_uncertainty[prev_i] } else { 0.0 };
                    let sz_prev = if prev_i < umd.z_rotated_uncertainty.len() { umd.z_rotated_uncertainty[prev_i] } else { 0.0 };

                    dx = ((sx_i.powi(2) + sx_prev.powi(2)) / dt.powi(2)).sqrt();
                    dy = ((sy_i.powi(2) + sy_prev.powi(2)) / dt.powi(2)).sqrt();
                    dz = ((sz_i.powi(2) + sz_prev.powi(2)) / dt.powi(2)).sqrt();
                }
            }

            velocity_data.add_point(
                umd.frame[i],
                umd.timestamp[i],
                umd.confidence[i],
                umd.pose[i],
                umd.coordinate_number[i],
                umd.types[i].clone(),
                vx, vy, vz, dx, dy, dz
            );
        }

        velocity_data
    }
}
