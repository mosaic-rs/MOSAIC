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
    Calculates the Euclidean distance between 2 different points, or a point and the origin.
*/

use crate::UMD::UMD::{UMD};
use crate::errors::{MosaicError};
use polars::prelude::*;
use std::fs::File;

#[derive(Debug, Clone)]
pub struct CoreEuclidean {
    pub frame: Vec<u32>,
    pub timestamp: Vec<f32>,

    // point 1 - another point or the origin
    pub coordinate_number_1: Vec<u32>,
    pub coordinate_type_1: Vec<String>,
    pub x1: Vec<f64>,
    pub y1: Vec<f64>,
    pub z1: Vec<f64>,

    // point 2
    pub coordinate_number_2: Vec<u32>, 
    pub coordinate_type_2: Vec<String>,
    pub x2: Vec<f64>, 
    pub y2: Vec<f64>, 
    pub z2: Vec<f64>,

    // r (distance/radius - whatever you wanna call it really)
    pub r: Vec<f64>,
    pub r_uncertainty: Vec<f64>,
}

impl CoreEuclidean {
    pub fn construction(estimated_entries: usize) -> Self {
        Self {
            frame: Vec::with_capacity(estimated_entries),
            timestamp: Vec::with_capacity(estimated_entries),

            coordinate_number_1: Vec::with_capacity(estimated_entries),
            coordinate_type_1: Vec::with_capacity(estimated_entries),
            x1: Vec::with_capacity(estimated_entries),
            y1: Vec::with_capacity(estimated_entries),
            z1: Vec::with_capacity(estimated_entries),

            coordinate_number_2: Vec::with_capacity(estimated_entries),
            coordinate_type_2: Vec::with_capacity(estimated_entries),
            x2: Vec::with_capacity(estimated_entries),
            y2: Vec::with_capacity(estimated_entries),
            z2: Vec::with_capacity(estimated_entries),

            r: Vec::with_capacity(estimated_entries),
            r_uncertainty: Vec::with_capacity(estimated_entries),
        }
    }

    pub fn add_point(
        &mut self, frame: u32, timestamp: f32, 
        coord_1: (u32, String, f64, f64, f64), 
        coord_2: (u32, String, f64, f64, f64),
        r: f64, r_uncertainty: f64
    ) {
        self.frame.push(frame);
        self.timestamp.push(timestamp);
        
        self.coordinate_number_1.push(coord_1.0);
        self.coordinate_type_1.push(coord_1.1);
        self.x1.push(coord_1.2);
        self.y1.push(coord_1.3);
        self.z1.push(coord_1.4);

        self.coordinate_number_2.push(coord_2.0);
        self.coordinate_type_2.push(coord_2.1);
        self.x2.push(coord_2.2);
        self.y2.push(coord_2.3);
        self.z2.push(coord_2.4);

        self.r.push(r);
        self.r_uncertainty.push(r_uncertainty);
    }

    pub fn save_euclidean_to_parquet(euclidean: &CoreEuclidean, file_path: &str) -> PolarsResult<()> {
        let s_frame = Series::new("frame", &euclidean.frame);
        let s_time = Series::new("timestamp", &euclidean.timestamp);

        let s_coord_1_num = Series::new("coordinate_number_1", &euclidean.coordinate_number_1);
        let s_coord_1_type = Series::new("coordinate_type_1", &euclidean.coordinate_type_1);
        let s_x1 = Series::new("x1", &euclidean.x1);
        let s_y1 = Series::new("y1", &euclidean.y1);
        let s_z1 = Series::new("z1", &euclidean.z1);

        let s_coord_2_num = Series::new("coordinate_number_2", &euclidean.coordinate_number_2);
        let s_coord_2_type = Series::new("coordinate_type_2", &euclidean.coordinate_type_2);
        let s_x2 = Series::new("x2", &euclidean.x2);
        let s_y2 = Series::new("y2", &euclidean.y2);
        let s_z2 = Series::new("z2", &euclidean.z2);
        
        let s_r = Series::new("r", &euclidean.r);
        let s_r_uncertainty = Series::new("r_uncertainty", &euclidean.r_uncertainty);

        let mut df = DataFrame::new(vec![
            s_frame, s_time, s_coord_1_num, s_coord_1_type, 
            s_x1, s_y1, s_z1, s_coord_2_num, s_coord_2_type, 
            s_x2, s_y2, s_z2, s_r, s_r_uncertainty,
        ])?;

        let file = File::create(file_path).map_err(PolarsError::from)?;
        ParquetWriter::new(file).finish(&mut df)?;
        println!("Successfully exported euclidean data to: {}", file_path);
        Ok(())
    }
}

pub struct EuclideanCalculator;

struct DistanceCalc;

impl EuclideanCalculator {

    pub fn euclidean(umd: &UMD, pairs: &[String; 2]) -> CoreEuclidean {
        let total_points = umd.frame.len();
        if total_points == 0 {
            return CoreEuclidean::construction(0);
        }

        let estimated_frames = (total_points / 68) + 1;
        let mut euclidean_data = CoreEuclidean::construction(estimated_frames);

        let p1_is_origin = pairs[0].to_lowercase() == "origin";
        let target_all = pairs[1] == "*";

        let mut i = 0;
        while i < total_points {
            let start_idx = i;
            let current_frame = umd.frame[i];
            
            // identify frame block
            while i < total_points && umd.frame[i] == current_frame {
                i += 1;
            }
            let end_idx = i;

            if p1_is_origin {
                if target_all {
                    for k in start_idx..end_idx {
                        Self::process_with_origin(&mut euclidean_data, umd, k);
                    }
                } else {
                    for k in start_idx..end_idx {
                        if &umd.types[k] == &pairs[1] {
                            Self::process_with_origin(&mut euclidean_data, umd, k);
                        }
                    }
                }
            } else {
                let mut p1_idx = None;
                for k in start_idx..end_idx {
                    if &umd.types[k] == &pairs[0] {
                        p1_idx = Some(k);
                        break;
                    }
                }

                if let Some(idx1) = p1_idx {
                    if target_all {
                        for k in start_idx..end_idx {
                            Self::process_pair(&mut euclidean_data, umd, idx1, k);
                        }
                    } else {
                        for k in start_idx..end_idx {
                            if &umd.types[k] == &pairs[1] {
                                Self::process_pair(&mut euclidean_data, umd, idx1, k);
                            }
                        }
                    }
                }
            }
        }

        euclidean_data
    }

    fn process_pair(data: &mut CoreEuclidean, umd: &UMD, idx1: usize, idx2: usize) {
        let x1 = umd.x_rotated[idx1];
        let y1 = umd.y_rotated[idx1];
        let z1 = umd.z_rotated[idx1];
        
        let dx1 = if idx1 < umd.x_rotated_uncertainty.len() { 
            umd.x_rotated_uncertainty[idx1] 
        } else { 0.0 };
        let dy1 = if idx1 < umd.y_rotated_uncertainty.len() { 
            umd.y_rotated_uncertainty[idx1] 
        } else { 0.0 };
        let dz1 = if idx1 < umd.z_rotated_uncertainty.len() { 
            umd.z_rotated_uncertainty[idx1] 
        } else { 0.0 };

        let x2 = umd.x_rotated[idx2];
        let y2 = umd.y_rotated[idx2];
        let z2 = umd.z_rotated[idx2];

        let dx2 = if idx2 < umd.x_rotated_uncertainty.len() { 
            umd.x_rotated_uncertainty[idx2] 
        } else { 0.0 };
        let dy2 = if idx2 < umd.y_rotated_uncertainty.len() { 
            umd.y_rotated_uncertainty[idx2] 
        } else { 0.0 };
        let dz2 = if idx2 < umd.z_rotated_uncertainty.len() { 
            umd.z_rotated_uncertainty[idx2] 
        } else { 0.0 };

        let v_x = x2 - x1;
        let v_y = y2 - y1;
        let v_z = z2 - z1;

        let dv_x = (dx1.powi(2) + dx2.powi(2)).sqrt();
        let dv_y = (dy1.powi(2) + dy2.powi(2)).sqrt();
        let dv_z = (dz1.powi(2) + dz2.powi(2)).sqrt();

        let r = DistanceCalc::calculate(v_x, v_y, v_z);
        let r_unc = DistanceCalc::calculate_uncertainty(r, v_x, v_y, v_z, dv_x, dv_y, dv_z);

        data.add_point(
            umd.frame[idx1],
            umd.timestamp[idx1],
            (umd.coordinate_number[idx1], umd.types[idx1].clone(), x1, y1, z1),
            (umd.coordinate_number[idx2], umd.types[idx2].clone(), x2, y2, z2),
            r,
            r_unc
        );
    }

    fn process_with_origin(data: &mut CoreEuclidean, umd: &UMD, idx2: usize) {
        let x2 = umd.x_rotated[idx2];
        let y2 = umd.y_rotated[idx2];
        let z2 = umd.z_rotated[idx2];

        let v_x = x2;
        let v_y = y2;
        let v_z = z2;

        let dv_x = if idx2 < umd.x_rotated_uncertainty.len() { 
            umd.x_rotated_uncertainty[idx2] 
        } else { 0.0 };
        let dv_y = if idx2 < umd.y_rotated_uncertainty.len() { 
            umd.y_rotated_uncertainty[idx2] 
        } else { 0.0 };
        let dv_z = if idx2 < umd.z_rotated_uncertainty.len() { 
            umd.z_rotated_uncertainty[idx2] 
        } else { 0.0 };

        let r = DistanceCalc::calculate(v_x, v_y, v_z);
        let r_unc = DistanceCalc::calculate_uncertainty(r, v_x, v_y, v_z, dv_x, dv_y, dv_z);

        data.add_point(
            umd.frame[idx2],
            umd.timestamp[idx2],
            (0, "origin".to_string(), 0.0, 0.0, 0.0),
            (umd.coordinate_number[idx2], umd.types[idx2].clone(), x2, y2, z2),
            r,
            r_unc
        );
    }
}


impl DistanceCalc {
    fn calculate(x: f64, y: f64, z: f64) -> f64 {
        (x.powi(2) + y.powi(2) + z.powi(2)).sqrt()
    }

    fn calculate_uncertainty(r: f64, x: f64, y: f64, z: f64, sx: f64, sy: f64, sz: f64) -> f64 {
        if r == 0.0 { 
            return 0.0;
        } 
        
        let term_x = (x * sx).powi(2);
        let term_y = (y * sy).powi(2);
        let term_z = (z * sz).powi(2);

        (term_x + term_y + term_z).sqrt() / r
    }
}