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

        // if point one is the origin
        let p1_is_origin = pairs[0].to_lowercase() == "origin";

        let mut current_frame = umd.frame[0];
        let mut p1_idx: Option<usize> = None;
        let mut p2_idx: Option<usize> = None;

        for i in 0..total_points {
            if umd.frame[i] != current_frame {
                if p1_is_origin && p2_idx.is_some() {
                    Self::process_with_origin(&mut euclidean_data, umd, p2_idx.unwrap());
                } else if let (Some(idx1), Some(idx2)) = (p1_idx, p2_idx) {
                    Self::process_pair(&mut euclidean_data, umd, idx1, idx2);
                }

                current_frame = umd.frame[i];
                p1_idx = None;
                p2_idx = None;
            }

            if !p1_is_origin && &umd.types[i] == &pairs[0] {
                p1_idx = Some(i);
            }
            if &umd.types[i] == &pairs[1] {
                p2_idx = Some(i);
            }
        }

        if p1_is_origin && p2_idx.is_some() {
            Self::process_with_origin(&mut euclidean_data, umd, p2_idx.unwrap());
        } else if let (Some(idx1), Some(idx2)) = (p1_idx, p2_idx) {
            Self::process_pair(&mut euclidean_data, umd, idx1, idx2);
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
            return 0.0; // no division by 0 :)
        } 
        
        let term_x = (x * sx).powi(2);
        let term_y = (y * sy).powi(2);
        let term_z = (z * sz).powi(2);

        (term_x + term_y + term_z).sqrt() / r
    }
}