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
Area logic 

Calculates the area of the mouth and also the 4 quadrants defined by the commissures and a vertical line through the philtrum

Rn kind of catered toward openface lip curves but I wouold love to make it so you could implement 3 curves (i.e. 3 curves tracking
the entire shape of the tongue) and get the area from that.


I will do uncertainty for this but it is a little scary so I leaving it for a minute
*/

use crate::coreMeasurements::curve::curve::{CoreCurve};
use crate::UMD::UMD::{UMD};
use std::f64::consts::PI;
use polars::prelude::*;
use std::fs::File;

pub struct CoreArea {
    pub frame: Vec<u32>,
    pub timestamp: Vec<f32>,
    pub types_included: Vec<String>,
    pub total_area: Vec<f64>,
    pub q1_area: Vec<f64>,
    pub q2_area: Vec<f64>,
    pub q3_area: Vec<f64>,
    pub q4_area: Vec<f64>,
}

impl CoreArea {
    pub fn construction(estimated_entries: usize) -> Self {
        Self {
            frame: Vec::with_capacity(estimated_entries),
            timestamp: Vec::with_capacity(estimated_entries),
            types_included: Vec::with_capacity(estimated_entries),
            total_area: Vec::with_capacity(estimated_entries),
            q1_area: Vec::with_capacity(estimated_entries),
            q2_area: Vec::with_capacity(estimated_entries),
            q3_area: Vec::with_capacity(estimated_entries),
            q4_area: Vec::with_capacity(estimated_entries),
        }
    }

    pub fn save_area_to_parquet(area: &CoreArea, file_path: &str) -> PolarsResult<()> {
        let s_frame = Series::new("frame", &area.frame);
        let s_time = Series::new("timestamp", &area.timestamp);
        let s_types = Series::new("types_included", &area.types_included);
        let s_total = Series::new("total_area", &area.total_area);
        let s_q1 = Series::new("q1_area", &area.q1_area);
        let s_q2 = Series::new("q2_area", &area.q2_area);
        let s_q3 = Series::new("q3_area", &area.q3_area);
        let s_q4 = Series::new("q4_area", &area.q4_area);

        let mut df = DataFrame::new(vec![
            s_frame, s_time, s_types, s_total, s_q1, s_q2, s_q3, s_q4
        ])?;

        let file = File::create(file_path).map_err(PolarsError::from)?;
        ParquetWriter::new(file).finish(&mut df)?;
        Ok(())
    }
}

pub struct AreaCalculator;

impl AreaCalculator {
    pub fn calculate_area(
        curve: &CoreCurve,
        umd: &UMD,
        basis_sets: &[&[&str; 4]],
        area_sets: &[&[&str]]
    ) -> CoreArea {
        let mut area_data = CoreArea::construction(curve.frame.len());
        
        let total_curve_entries = curve.frame.len();
        let total_umd_entries = umd.frame.len();
        let mut curve_idx = 0;
        let mut umd_idx = 0;

        while curve_idx < total_curve_entries {
            let current_frame = curve.frame[curve_idx];
            
            let umd_start = umd_idx;
            while umd_idx < total_umd_entries && umd.frame[umd_idx] == current_frame {
                umd_idx += 1;
            }
            let umd_end = umd_idx;

            let curve_start = curve_idx;
            while curve_idx < total_curve_entries && curve.frame[curve_idx] == current_frame {
                curve_idx += 1;
            }
            let curve_end = curve_idx;

            for (k, set) in area_sets.iter().enumerate() {
                let basis_names = basis_sets[k];

                let (mut p_lc, mut p_rc, mut p_lm, mut p_ph) = (Vec3::zero(), Vec3::zero(), Vec3::zero(), Vec3::zero());

                for j in umd_start..umd_end {
                    let pos = Vec3::new(umd.x_rotated[j], umd.y_rotated[j], umd.z_rotated[j]);
                    if &umd.types[j] == basis_names[0] { p_lc = pos; }
                    if &umd.types[j] == basis_names[1] { p_rc = pos; }
                    if &umd.types[j] == basis_names[2] { p_lm = pos; }
                    if &umd.types[j] == basis_names[3] { p_ph = pos; }
                }

                let (basis_inv, scale) = match calculate_basis(p_lc, p_rc, p_lm, p_ph) {
                    Ok(res) => res,
                    Err(_) => continue, 
                };
                
                let origin = p_lc.add(p_rc).scale(0.5);

                let mut curves = Vec::new();
                for j in curve_start..curve_end {
                    let current_type = &curve.types_included[j];
                    if set.contains(&current_type.as_str()) {
                         let cb = CubicBezier::new(
                            Vec3::new(curve.x_coeffs[j].d, curve.y_coeffs[j].d, curve.z_coeffs[j].d),
                            Vec3::new(curve.x_coeffs[j].c, curve.y_coeffs[j].c, curve.z_coeffs[j].c),
                            Vec3::new(curve.x_coeffs[j].b, curve.y_coeffs[j].b, curve.z_coeffs[j].b),
                            Vec3::new(curve.x_coeffs[j].a, curve.y_coeffs[j].a, curve.z_coeffs[j].a),
                        );
                        curves.push(transform_curve(&cb, &basis_inv, origin));
                    }
                }

                let (total, q1, q2, q3, q4) = calculate_total_area(curves, scale);

                area_data.frame.push(current_frame);
                area_data.timestamp.push(curve.timestamp[curve_start]);
                area_data.types_included.push(set.join(","));
                area_data.total_area.push(total);
                area_data.q1_area.push(q1);
                area_data.q2_area.push(q2);
                area_data.q3_area.push(q3);
                area_data.q4_area.push(q4);
            }
        }
        area_data
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Vec3 { x: f64, y: f64, z: f64 }

impl Vec3 {
    fn new(x: f64, y: f64, z: f64) -> Self { Self { x, y, z } }
    fn zero() -> Self { Self { x: 0.0, y: 0.0, z: 0.0 } }
    fn add(self, other: Vec3) -> Vec3 { Vec3::new(self.x + other.x, self.y + other.y, self.z + other.z) }
    fn sub(self, other: Vec3) -> Vec3 { Vec3::new(self.x - other.x, self.y - other.y, self.z - other.z) }
    fn scale(self, s: f64) -> Vec3 { Vec3::new(self.x * s, self.y * s, self.z * s) }
    fn magnitude(self) -> f64 { (self.x * self.x + self.y * self.y + self.z * self.z).sqrt() }
    fn cross(self, other: Vec3) -> Vec3 {
        Vec3::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }
    fn cross_2d(self, other: Vec3) -> f64 {
        self.x * other.y - self.y * other.x
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Mat3 { cols: [Vec3; 3] }

impl Mat3 {
    fn from_cols(c1: Vec3, c2: Vec3, c3: Vec3) -> Self { Self { cols: [c1, c2, c3] } }
    
    fn transform_vector(&self, v: Vec3) -> Vec3 {
        let x = self.cols[0].x * v.x + self.cols[1].x * v.y + self.cols[2].x * v.z;
        let y = self.cols[0].y * v.x + self.cols[1].y * v.y + self.cols[2].y * v.z;
        let z = self.cols[0].z * v.x + self.cols[1].z * v.y + self.cols[2].z * v.z;
        Vec3::new(x, y, z)
    }

    pub fn determinant(&self) -> f64 {
        let [c1, c2, c3] = self.cols;
        c1.x * (c2.y * c3.z - c2.z * c3.y) 
        - c1.y * (c2.x * c3.z - c2.z * c3.x) 
        + c1.z * (c2.x * c3.y - c2.y * c3.x)
    }

    pub fn inverse(&self) -> Option<Mat3> {
        let det = self.determinant();
        if det.abs() < 1e-12 { return None; }
        let inv_det = 1.0 / det;
        
        let [c1, c2, c3] = self.cols;

        let v1 = Vec3::new(
            (c2.y * c3.z - c2.z * c3.y) * inv_det,
            (c3.y * c1.z - c3.z * c1.y) * inv_det,
            (c1.y * c2.z - c1.z * c2.y) * inv_det,
        );
        let v2 = Vec3::new(
            (c2.z * c3.x - c2.x * c3.z) * inv_det,
            (c3.z * c1.x - c3.x * c1.z) * inv_det,
            (c1.z * c2.x - c1.x * c2.z) * inv_det,
        );
        let v3 = Vec3::new(
            (c2.x * c3.y - c2.y * c3.x) * inv_det,
            (c3.x * c1.y - c3.y * c1.x) * inv_det,
            (c1.x * c2.y - c1.y * c2.x) * inv_det,
        );

        Some(Mat3::from_cols(v1, v2, v3))
    }
}

#[derive(Clone, Copy, Debug)]
pub struct CubicBezier { p0: Vec3, p1: Vec3, p2: Vec3, p3: Vec3 }

impl CubicBezier {
    fn new(p0: Vec3, p1: Vec3, p2: Vec3, p3: Vec3) -> Self { Self { p0, p1, p2, p3 } }

    fn eval(&self, t: f64) -> Vec3 {
        let one_minus_t = 1.0 - t;
        let a = one_minus_t.powi(3);
        let b = 3.0 * one_minus_t.powi(2) * t;
        let c = 3.0 * one_minus_t * t.powi(2);
        let d = t.powi(3);
        self.p0.scale(a).add(self.p1.scale(b)).add(self.p2.scale(c)).add(self.p3.scale(d))
    }

    fn split(&self, t: f64) -> (CubicBezier, CubicBezier) {
        let p01 = self.p0.scale(1.0 - t).add(self.p1.scale(t));
        let p12 = self.p1.scale(1.0 - t).add(self.p2.scale(t));
        let p23 = self.p2.scale(1.0 - t).add(self.p3.scale(t));
        let p012 = p01.scale(1.0 - t).add(p12.scale(t));
        let p123 = p12.scale(1.0 - t).add(p23.scale(t));
        let p0123 = p012.scale(1.0 - t).add(p123.scale(t));
        (
            CubicBezier::new(self.p0, p01, p012, p0123),
            CubicBezier::new(p0123, p123, p23, self.p3)
        )
    }

    fn signed_area(&self) -> f64 {
        let term1 = self.p0.cross_2d(self.p1);
        let term2 = self.p0.cross_2d(self.p2);
        let term3 = self.p0.cross_2d(self.p3);
        let term4 = self.p1.cross_2d(self.p2);
        let term5 = self.p1.cross_2d(self.p3);
        let term6 = self.p2.cross_2d(self.p3);
        
        (6.0 * term1 + 3.0 * term2 + term3 + 3.0 * term4 + 3.0 * term5 + 6.0 * term6) / 20.0
    }
}

fn solve_cubic_roots(p0: f64, p1: f64, p2: f64, p3: f64) -> Vec<f64> {
    let a = -p0 + 3.0 * p1 - 3.0 * p2 + p3;
    let b = 3.0 * p0 - 6.0 * p1 + 3.0 * p2;
    let c = -3.0 * p0 + 3.0 * p1;
    let d = p0;

    let epsilon = 1e-9;
    let mut roots = Vec::new();

    if a.abs() < epsilon {
        if b.abs() < epsilon {
            if c.abs() > epsilon {
                let t = -d / c;
                if t > epsilon && t < 1.0 - epsilon { roots.push(t); }
            }
        } else {
            let delta = c * c - 4.0 * b * d;
            if delta >= 0.0 {
                let sqrt_delta = delta.sqrt();
                let t1 = (-c - sqrt_delta) / (2.0 * b);
                let t2 = (-c + sqrt_delta) / (2.0 * b);
                if t1 > epsilon && t1 < 1.0 - epsilon { roots.push(t1); }
                if t2 > epsilon && t2 < 1.0 - epsilon { roots.push(t2); }
            }
        }
    } else {
        let A = b / a; 
        let B = c / a; 
        let C = d / a;
        let Q = (3.0 * B - A * A) / 9.0;
        let R = (9.0 * A * B - 27.0 * C - 2.0 * A * A * A) / 54.0;
        let D = Q * Q * Q + R * R;

        if D >= 0.0 {
            let sqrt_D = D.sqrt();
            let S = (R + sqrt_D).cbrt();
            let T = (R - sqrt_D).cbrt();
            let t = S + T - A / 3.0;
            if t > epsilon && t < 1.0 - epsilon { roots.push(t); }
        } else {
            let theta = (R / (-Q * Q * Q).sqrt()).acos();
            let sqrt_minus_Q = (-Q).sqrt();
            for k in 0..3 {
                let t = 2.0 * sqrt_minus_Q * ((theta + 2.0 * PI * k as f64) / 3.0).cos() - A / 3.0;
                if t > epsilon && t < 1.0 - epsilon { roots.push(t); }
            }
        }
    }
    roots
}

fn calculate_basis(p_lc: Vec3, p_rc: Vec3, p_lm: Vec3, p_ph: Vec3) -> Result<(Mat3, f64), String> {
    let v1 = p_lc.sub(p_rc);
    let v2 = p_lm.sub(p_ph);
    let v3 = v1.cross(v2);

    let basis = Mat3::from_cols(v1, v2, v3);
    let scale = v3.magnitude(); 

    if scale < 1e-12 { return Err("Singular basis".to_string()); }

    basis.inverse().map(|inv| (inv, scale)).ok_or_else(|| "Singular basis".to_string())
}

pub fn transform_curve(curve: &CubicBezier, basis_inv: &Mat3, origin: Vec3) -> CubicBezier {
    let tr = |p: Vec3| basis_inv.transform_vector(p.sub(origin));
    CubicBezier::new(tr(curve.p0), tr(curve.p1), tr(curve.p2), tr(curve.p3))
}

pub fn calculate_total_area(
    curves: Vec<CubicBezier>,
    scale: f64
) -> (f64, f64, f64, f64, f64) {
    let mut q_areas = [0.0; 4];

    for curve in curves {
        let mut split_params = Vec::new();
        
        let roots_x = solve_cubic_roots(curve.p0.x, curve.p1.x, curve.p2.x, curve.p3.x);
        split_params.extend(roots_x);

        let roots_y = solve_cubic_roots(curve.p0.y, curve.p1.y, curve.p2.y, curve.p3.y);
        split_params.extend(roots_y);

        split_params.sort_by(|a, b| a.partial_cmp(b).unwrap());
        split_params.dedup();

        let mut segments = Vec::new();
        let mut current_curve = curve.clone();
        let mut t_start = 0.0;

        for t_global in split_params {
            let t_local = (t_global - t_start) / (1.0 - t_start);
            if t_local > 1e-6 && t_local < 1.0 - 1e-6 {
                let (left, right) = current_curve.split(t_local);
                segments.push(left);
                current_curve = right;
                t_start = t_global;
            }
        }
        segments.push(current_curve);

        for seg in segments {
            let mid = seg.eval(0.5); 
            let area = seg.signed_area() * scale;
            
            if mid.x >= 0.0 && mid.y >= 0.0 { q_areas[0] += area; }      
            else if mid.x < 0.0 && mid.y >= 0.0 { q_areas[1] += area; } 
            else if mid.x < 0.0 && mid.y < 0.0 { q_areas[2] += area; }  
            else { q_areas[3] += area; }                                
        }
    }

    let q1 = q_areas[0].abs();
    let q2 = q_areas[1].abs();
    let q3 = q_areas[2].abs();
    let q4 = q_areas[3].abs();
    let total = q1 + q2 + q3 + q4;

    (total, q1, q2, q3, q4)
}