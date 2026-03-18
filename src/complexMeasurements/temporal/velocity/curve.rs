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
pub struct CoefficientVelocity {
    pub frame: Vec<u32>,
    pub timestamp: Vec<f32>,
    pub confidence: Vec<f32>,
    pub pose: Vec<bool>,

    pub va: Vec<f64>,
    pub vb: Vec<f64>,
    pub vc: Vec<f64>,
    pub vd: Vec<f64>,

    pub sa: Vec<f64>,
    pub sb: Vec<f64>,
    pub sc: Vec<f64>,
    pub sd: Vec<f64>,
}

impl CoefficientVelocity {
    pub fn construction(estimated_entries: usize) -> Self {
        Self {
            frame: Vec::with_capacity(estimated_entries),
            timestamp: Vec::with_capacity(estimated_entries),
            confidence: Vec::with_capacity(estimated_entries),
            pose: Vec::with_capacity(estimated_entries),

            va: Vec::with_capacity(estimated_entries),
            vb: Vec::with_capacity(estimated_entries),
            vc: Vec::with_capacity(estimated_entries),
            vd: Vec::with_capacity(estimated_entries)
            sa: Vec::with_capacity(estimated_entries),
            sb: Vec::with_capacity(estimated_entries),
            sc: Vec::with_capacity(estimated_entries),
            sd: Vec::with_capacity(estimated_entries),
        }
    }

pub fn add_point(
    &mut self, frame: u32, time: f32, confidence: f32, pose: bool,
    va: f64, vb: f64, vc: f64, vd: f64, sa: f64, sb: f64, sc: f64, sd: f64) {
        self.frame.push(frame);
        self.timestamp.push(time);
        self.confidence.push(confidence);
        self.pose.push(pose);

        self.va.push(va);
        self.vb.push(vb);
        self.vc.push(vc);
        self.vd.push(vd);

        self.sa.push(sa);
        self.sb.push(sb);
        self.sc.push(sc);
        self.sd.push(sd);
    }

pub fn save_coefficient_velocity_to_parquet(data: &CoefficientVelocity, file_path: &str) -> PolarsResult<()> {
    let s_frame = Series::new("frame", &data.frame);
    let s_time = Series::new("timestamp", &data.timestamp);
    let s_confidence = Series::new("confidence", &data.confidence);
    let s_pose = Series::new("pose_detected", &data.pose);

    let s_va = Series::new("va", &data.va);
    let s_vb = Series::new("vb", &data.vb);
    let s_vc = Series::new("vc", &data.vc);
    let s_vd = Series::new("vd", &data.vd);

    let s_sa = Series::new("sa", &data.sa);
    let s_sb = Series::new("sb", &data.sb);
    let s_sc = Series::new("sc", &data.sc);
    let s_sd = Series::new("sd", &data.sd);

    let mut df = DataFrame::new(vec![
        s_frame, s_time, s_confidence, s_pose,
        s_va, s_vb, s_vc, s_vd,
        s_sa, s_sb, s_sc, s_sd,
    ])?;

    let file = File::create(file_path).map_err(PolarsError::from)?;
    ParquetWriter::new(file).finish(&mut df)?;

    println!("Successfully exported Coefficient Velocity data to: {}", file_path);
    Ok(())
}
}

#[derive(Debug, Clone)]
pub struct CurveVelocityAcrossT {
    pub frame: Vec<u32>,
    pub timestamp: Vec<f32>,
    pub t_step: Vec<f32>,
    pub v_total: Vec<f64>,
}

impl CurveVelocityAcrossT {
    pub fn construction(estimated_entries: usize) -> Self {
        Self {
            frame: Vec::with_capacity(estimated_entries),
            timestamp: Vec::with_capacity(estimated_entries),
            t_step: Vec::with_capacity(estimated_entries),
            v_total: Vec::with_capacity(estimated_entries),
        }
    }

    pub fn add_point(&mut self, frame: u32, time: f32, t: f32, v_total: f64) {
        self.frame.push(frame);
        self.timestamp.push(time);
        self.t_step.push(t);
        self.v_total.push(v_total);
    }

    pub fn save_curve_velocity_to_parquet(data: &CurveVelocityAcrossT, file_path: &str) -> PolarsResult<()> {
        let s_frame = Series::new("frame", &data.frame);
        let s_time = Series::new("timestamp", &data.timestamp);
        let s_t = Series::new("t", &data.t_step);
        let s_v = Series::new("v_total", &data.v_total);

        let mut df = DataFrame::new(vec![s_frame, s_time, s_t, s_v])?;

        let file = File::create(file_path).map_err(PolarsError::from)?;
        ParquetWriter::new(file).finish(&mut df)?;

        println!("Successfully exported Curve Velocity data to: {}", file_path);
        Ok(())
        }
}

pub struct CalculateCurveDynamics;

impl CalculateCurveDynamics {
    pub fn calculate(umd: &UMD, t_resolution: usize) -> (CoefficientVelocity, CurveVelocityAcrossT) {
        let total_frames = umd.frame.len();
        if total_frames == 0 {
            return (CoefficientVelocity::construction(0), CurveVelocityAcrossT::construction(0));
        }

        let mut coef_data = CoefficientVelocity::construction(total_frames);
        let mut curve_data = CurveVelocityAcrossT::construction(total_frames * t_resolution);

        for i in 1..total_frames {
            let dt = (umd.timestamp[i] - umd.timestamp[i - 1]) as f64;
            if dt <= 0.0 { continue; }

            // elocity for coeficients
            let v_ax = (umd.ax[i] - umd.ax[i - 1]) / dt;
            let v_ay = (umd.ay[i] - umd.ay[i - 1]) / dt;
            let v_az = (umd.az[i] - umd.az[i - 1]) / dt;

            let v_bx = (umd.bx[i] - umd.bx[i - 1]) / dt;
            let v_by = (umd.by[i] - umd.by[i - 1]) / dt;
            let v_bz = (umd.bz[i] - umd.bz[i - 1]) / dt;

            let v_cx = (umd.cx[i] - umd.cx[i - 1]) / dt;
            let v_cy = (umd.cy[i] - umd.cy[i - 1]) / dt;
            let v_cz = (umd.cz[i] - umd.cz[i - 1]) / dt;

            let v_dx = (umd.dx_coeff[i] - umd.dx_coeff[i - 1]) / dt;
            let v_dy = (umd.dy_coeff[i] - umd.dy_coeff[i - 1]) / dt;
            let v_dz = (umd.dz_coeff[i] - umd.dz_coeff[i - 1]) / dt;

            // magnitude per coefficient
            let va = (v_ax.powi(2) + v_ay.powi(2) + v_az.powi(2)).sqrt();
            let vb = (v_bx.powi(2) + v_by.powi(2) + v_bz.powi(2)).sqrt();
            let vc = (v_cx.powi(2) + v_cy.powi(2) + v_cz.powi(2)).sqrt();
            let vd = (v_dx.powi(2) + v_dy.powi(2) + v_dz.powi(2)).sqrt();

            // unc prop
            let sa = ((umd.sax[i].powi(2) + umd.sax[i - 1].powi(2)) / dt.powi(2)).sqrt();
            let sb = ((umd.sbx[i].powi(2) + umd.sbx[i - 1].powi(2)) / dt.powi(2)).sqrt();
            let sc = ((umd.scx[i].powi(2) + umd.scx[i - 1].powi(2)) / dt.powi(2)).sqrt();
            let sd = ((umd.sdx[i].powi(2) + umd.sdx[i - 1].powi(2)) / dt.powi(2)).sqrt();

            coef_data.add_point(
                umd.frame[i], umd.timestamp[i], umd.confidence[i], umd.pose[i],
                va, vb, vc, vd, sa, sb, sc, sd
            );

        // Velocity across the curve where t = user defined
            for step in 0..t_resolution {
                let t = step as f64 / (t_resolution - 1) as f64;
                
                let vx_t = v_ax * t.powi(3) + v_bx * t.powi(2) + v_cx * t + v_dx;
                let vy_t = v_ay * t.powi(3) + v_by * t.powi(2) + v_cy * t + v_dy;
                let vz_t = v_az * t.powi(3) + v_bz * t.powi(2) + v_cz * t + v_dz;

                let v_total = (vx_t.powi(2) + vy_t.powi(2) + vz_t.powi(2)).sqrt();
                
                curve_data.add_point(umd.frame[i], umd.timestamp[i], t as f32, v_total);
            }
        }

    (coef_data, curve_data)
}
}