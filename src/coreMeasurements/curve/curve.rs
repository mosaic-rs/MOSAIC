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

pub struct CoreCurve {
    pub frame: Vec<u32>,
    pub timestamp: Vec<f32>,
    pub types_included: Vec<String>, 
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
        types_included: String, 
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
    pub fn fit_curve(umd: &UMD, landmark_sets: &[&[&str]]) -> CoreCurve {
        let total_entries = umd.frame.len();
        if total_entries == 0 { return CoreCurve::construction(0); }

        let mut curve_data = CoreCurve::construction(total_entries / 68);
        let set_identifiers: Vec<String> = landmark_sets.iter().map(|s| s.join(",")).collect();

        let mut i = 0;
        while i < total_entries {
            let current_frame = umd.frame[i];
            let start_idx = i;
            while i < total_entries && umd.frame[i] == current_frame { i += 1; }
            let end_idx = i;

            for (k, set) in landmark_sets.iter().enumerate() {
                let mut points = Vec::new();
                for &target in *set {
                    for idx in start_idx..end_idx {
                        if umd.types[idx] == target {
                            points.push((umd.x_rotated[idx], umd.y_rotated[idx], umd.z_rotated[idx]));
                            break; 
                        }
                    }
                }
                Self::process_frame(&mut curve_data, current_frame, umd.timestamp[start_idx], &points, &set_identifiers[k]);
            }
        }
        curve_data
    }

    fn process_frame(data: &mut CoreCurve, frame: u32, ts: f32, points: &[(f64, f64, f64)], types_str: &str) {
        if points.len() < 3 { return; } 

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
            t.push(if l > 0.0 { cumulative_d / l } else { 0.0 });
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
        let cx = svd.solve(&px, 1e-9).unwrap_or(DVector::from_element(4, 0.0));
        let cy = svd.solve(&py, 1e-9).unwrap_or(DVector::from_element(4, 0.0));
        let cz = svd.solve(&pz, 1e-9).unwrap_or(DVector::from_element(4, 0.0));

        data.add_point(
            frame, ts, types_str.to_string(),
            CurveCoefficients { a: cx[0], b: cx[1], c: cx[2], d: cx[3] },
            CurveCoefficients { a: cy[0], b: cy[1], c: cy[2], d: cy[3] },
            CurveCoefficients { a: cz[0], b: cz[1], c: cz[2], d: cz[3] },
        );
    }
}