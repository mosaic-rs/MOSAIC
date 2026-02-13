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
    curve fitting 
    Calculates cubic curve coefficients (ax^3 + bx^2 + cx + d) for a set of points 
    using Chord Length Parameterization and SVD Least Squares solving.

    You can give it as many points as you want (minimum of 4 really)


    I will make an uncertainty portion for uncertainty of coefficients 
    but lord knows I am not smart enough to do that right now
*/

use crate::UMD::UMD::{UMD};
use nalgebra::{DMatrix, DVector, SVD};
use polars::prelude::*;
use std::fs::File;

#[derive(Debug, Clone)]
pub struct CurveCoefficients {
    pub a: f64,
    pub b: f64,
    pub c: f64,
    pub d: f64,
}

pub struct CurvePointTypes {
    pub a: String,
    pub b: String,
    pub c: String,
    pub d: String
}

pub struct CoreCurve {
    pub frame: Vec<u32>,
    pub timestamp: Vec<f32>,
    pub types_included: Vec<String>, // List of landmarks used for this curve
                                     // In the you used it will tell you what curve calculations you can do. For example,
                                     // you can not do a curve calculation of the tongue using OpenFace data (currently being implemented)
    
    // coefficients for the three dimensions
    pub x_coeffs: Vec<CurveCoefficients>,
    pub y_coeffs: Vec<CurveCoefficients>,
    pub z_coeffs: Vec<CurveCoefficients>,
}

impl CoreCurve {
    pub fn construction(estimated_frames: usize) -> Self {
        Self {
            frame: Vec::with_capacity(estimated_frames),
            timestamp: Vec::with_capacity(estimated_frames),
            types_included: Vec::with_capacity(estimated_frames),
            x_coeffs: Vec::with_capacity(estimated_frames),
            y_coeffs: Vec::with_capacity(estimated_frames),
            z_coeffs: Vec::with_capacity(estimated_frames),
        }
    }

    pub fn add_point(
        &mut self, frame: u32, timestamp: f32, 
        types_included: String, // currently only just takes in the 4 outer points of openface lip points BUT will be made more data blind
        x_coeffs: CurveCoefficients,
        y_coeffs: CurveCoefficients,
        z_coeffs: CurveCoefficients,
    ) {
        self.frame.push(frame);
        self.timestamp.push(timestamp);
        
        self.types_included.push(types_included);
        self.x_coeffs.push(x_coeffs);
        self.y_coeffs.push(y_coeffs);
        self.z_coeffs.push(z_coeffs);
    }

    pub fn save_curve_to_parquet(curve: &CoreCurve, file_path: &str) -> PolarsResult<()> {
        let s_frame = Series::new("frame", &curve.frame);
        let s_time = Series::new("timestamp", &curve.timestamp);

        let s_coord_1 = Series::new("types_included", &curve.types_included);
        let s_x_a = Series::new("x_a", curve.x_coeffs.iter().map(|c| c.a).collect::<Vec<f64>>());
        let s_x_b = Series::new("x_b", curve.x_coeffs.iter().map(|c| c.b).collect::<Vec<f64>>());
        let s_x_c = Series::new("x_c", curve.x_coeffs.iter().map(|c| c.c).collect::<Vec<f64>>());
        let s_x_d = Series::new("x_d", curve.x_coeffs.iter().map(|c| c.d).collect::<Vec<f64>>());

        let s_y_a = Series::new("y_a", curve.y_coeffs.iter().map(|c| c.a).collect::<Vec<f64>>());
        let s_y_b = Series::new("y_b", curve.y_coeffs.iter().map(|c| c.b).collect::<Vec<f64>>());
        let s_y_c = Series::new("y_c", curve.y_coeffs.iter().map(|c| c.c).collect::<Vec<f64>>());
        let s_y_d = Series::new("y_d", curve.y_coeffs.iter().map(|c| c.d).collect::<Vec<f64>>());

        let s_z_a = Series::new("z_a", curve.z_coeffs.iter().map(|c| c.a).collect::<Vec<f64>>());
        let s_z_b = Series::new("z_b", curve.z_coeffs.iter().map(|c| c.b).collect::<Vec<f64>>());
        let s_z_c = Series::new("z_c", curve.z_coeffs.iter().map(|c| c.c).collect::<Vec<f64>>());
        let s_z_d = Series::new("z_d", curve.z_coeffs.iter().map(|c| c.d).collect::<Vec<f64>>());
        //let x_coeffs = Series::new("x_coeffs", &curve.x_coeffs);
        //let y_coeffs = Series::new("y_coeffs", &curve.y_coeffs);
        //let z_coeffs = Series::new("z_coeffs", &curve.z_coeffs);

        let mut df = DataFrame::new(vec![ 
            s_frame, s_time, s_coord_1, 
            s_x_a, s_x_b, s_x_c, s_x_d, 
            s_y_a, s_y_b, s_y_c, s_y_d, 
            s_z_a, s_z_b, s_z_c, s_z_d,
        ])?;
        
        let file = File::create(file_path).map_err(PolarsError::from)?;
        ParquetWriter::new(file).finish(&mut df)?;
        println!("Successfully exported curve data to: {}", file_path);
        Ok(())
    }

}

pub struct CurveCalculator;

impl CurveCalculator {
    /// fits a cubic curve to a specific set of landmarks across all frames.
    /// landmarks: e.g., ["LowerLipLeft", "LowerLipCenter", "LowerLipRight"]
    /// I am gonna make default functions for drivers to save you from having to put in coordinate types yourself.
    pub fn fit_curve(umd: &UMD, landmarks: &[String]) -> CoreCurve {
        let total_entries = umd.frame.len();
        if total_entries == 0 || landmarks.len() < 4 {
            return CoreCurve::construction(0);
        }

        let mut curve_data = CoreCurve::construction(total_entries / 68);
        
        // Create a string representation of the landmarks involved for the column
        let types_str = landmarks.join(",");

        let mut current_frame = umd.frame[0];
        let mut frame_points: Vec<(f64, f64, f64)> = Vec::new();

        for i in 0..total_entries {
            if umd.frame[i] != current_frame {
                Self::process_frame(&mut curve_data, current_frame, umd.timestamp[i-1], &frame_points, &types_str);
                frame_points.clear();
                current_frame = umd.frame[i];
            }

            if landmarks.contains(&umd.types[i]) {
                frame_points.push((umd.x_rotated[i], umd.y_rotated[i], umd.z_rotated[i]));
            }
        }

        Self::process_frame(&mut curve_data, current_frame, *umd.timestamp.last().unwrap(), &frame_points, &types_str);

        curve_data
    }

    fn process_frame(data: &mut CoreCurve, frame: u32, ts: f32, points: &[(f64, f64, f64)], types_str: &str) {
        if points.len() < 4 { return; }

        // chord length 
        let mut d = Vec::with_capacity(points.len() - 1);
        for i in 0..points.len() - 1 {
            let dist = ((points[i+1].0 - points[i].0).powi(2) + 
                        (points[i+1].1 - points[i].1).powi(2) + 
                        (points[i+1].2 - points[i].2).powi(2)).sqrt();
            d.push(dist);
        }

        let l: f64 = d.iter().sum();
        let mut t = vec![0.0];
        let mut cumulative_d = 0.0;
        for dist in d {
            cumulative_d += dist;
            t.push(cumulative_d / l);
        }

        let rows = points.len();
        let mut m_data = Vec::with_capacity(rows * 4);
        for val_t in &t {
            m_data.push(val_t.powi(3));
            m_data.push(val_t.powi(2));
            m_data.push(*val_t);
            m_data.push(1.0);
        }
        let m = DMatrix::from_row_slice(rows, 4, &m_data);

        let px = DVector::from_iterator(rows, points.iter().map(|p| p.0));
        let py = DVector::from_iterator(rows, points.iter().map(|p| p.1));
        let pz = DVector::from_iterator(rows, points.iter().map(|p| p.2));

        let svd = m.svd(true, true);
        
        let cx = svd.solve(&px, 1e-9).unwrap();
        let cy = svd.solve(&py, 1e-9).unwrap();
        let cz = svd.solve(&pz, 1e-9).unwrap();

        data.add_point(
            frame,
            ts,
            types_str.to_string(),
            CurveCoefficients { a: cx[0], b: cx[1], c: cx[2], d: cx[3] },
            CurveCoefficients { a: cy[0], b: cy[1], c: cy[2], d: cy[3] },
            CurveCoefficients { a: cz[0], b: cz[1], c: cz[2], d: cz[3] },
        );
    }
}