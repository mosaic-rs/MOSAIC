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


// Drivers
use crate::drivers::OpenFace::openface::{parse_openface_data};
use crate::drivers::OpenFace::defaultCommands::{curves, areas};

// UMD
use crate::UMD::anchor::anchor::{AnchorProcessor};
use crate::UMD::centering::centering::{CenteringProcessor};
use crate::UMD::pose::pose::{PoseProcessor};
use crate::UMD::UMD::{UMD, UMDDriver};
use crate::UMD::metadata::{Metadata};

// Core Measurements
use crate::coreMeasurements::euclidean::euclidean::{EuclideanCalculator, CoreEuclidean};
use crate::coreMeasurements::angle::angle::{AngleCalculator, CoreAngle};
use crate::coreMeasurements::curve::curve::{CurveCalculator, CoreCurve};
use crate::coreMeasurements::area::area::{AreaCalculator, CoreArea};

// praat analysis
use crate::praatAnalysis::setup::PythonEnvironment;
use crate::praatAnalysis::testing::test_function;


use std::path::Path;
use pyo3::prelude::*;
use pyo3::Python;



pub struct run;

impl run {
    // currently very simple as we aren't passing complex arguments
    // rn it is just for testing stuff

    pub fn init(input_path: &str, output_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        // the init command is kinda just for testing but it also does work for real input

        // for now we will define the metadata at the top sort of as const vars which will be customisable through the CLI
        let UMD_Version = "0.9.0".to_string();
        let driver = "OpenFace".to_string();
        let dimension = "3D".to_string();
        let centered = true;
        let pose_correction = true;

        let metadata = Metadata::new(UMD_Version, driver, dimension, centered, pose_correction);
        let kv_metadata = metadata.to_kv_vec();

        // init is a general run command
        // we can edit it to pass paremeters through later
        let umd_driver = parse_openface_data(Path::new(input_path)).expect("Failed to parse data");
        let file_name = "umd_driver.parquet";
        let umd_driver_output_path = format!("{output_path}{file_name}");;
        UMDDriver::save_umd_driver_to_parquet(&umd_driver, umd_driver_output_path.as_str());

        // anchor testing

        let anchor_results = AnchorProcessor::calculate_umd_anchors(&umd_driver)?;
        let file_name = "umd_anchor.parquet";
        let umd_anchor_output_path = format!("{output_path}{file_name}");
        AnchorProcessor::save_anchors_to_parquet(&anchor_results, umd_anchor_output_path.as_str())?;

        // centering testing

        let centering_results = CenteringProcessor::calculate_centering(&umd_driver, &anchor_results)?;
        let file_name = "umd_centered.parquet";
        let umd_centering_output_path = format!("{output_path}{file_name}");
        CenteringProcessor::save_centered_to_parquet(&centering_results, umd_centering_output_path.as_str())?;

        // pose correction testing

        let pose_correction_results = PoseProcessor::calculate_pose_corr(&centering_results)?;
        let file_name = "umd_rotated.parquet";
        let umd_rotated_output_path = format!("{output_path}{file_name}");
        PoseProcessor::save_pose_to_parquet(&pose_correction_results, umd_rotated_output_path.as_str())?;

        // Final UMD output

        //aw: &UMDDriver, anchor: &UMDAnchor, centered: &UMDCentered, rotated: &UMDPose
        let total_entries = centering_results.x.len() as u32;        
        let mut umd_instance = UMD::construction(total_entries, 1);
        umd_instance.add_point(&umd_driver, &anchor_results, &centering_results, &pose_correction_results);

        let file_name = "umd.parquet";
        let umd_output_path = format!("{output_path}{file_name}");

        UMD::save_umd_to_parquet(&umd_instance, umd_output_path.as_str(), kv_metadata)?;



        // Core measurement testing:
        
        // Eulidean
        let euclidean_results = EuclideanCalculator::euclidean(&umd_instance, &["origin".to_string(), "*".to_string()]);
        let file_name = "euclidean.parquet";
        let euclidean_output_path = format!("{output_path}{file_name}"); 
        CoreEuclidean::save_euclidean_to_parquet(&euclidean_results, &euclidean_output_path).expect("Failed to write euclidean to parquet");
        println!("Euclidean works");


        // Angle

        let angle_results = AngleCalculator::angle(&umd_instance, &["origin".to_string(), "*".to_string()]);
        let file_name = "angle.parquet";
        let angle_output_path = format!("{output_path}{file_name}"); 
        CoreAngle::save_angle_to_parquet(&angle_results, &angle_output_path).expect("Failed to write angles to parquet");
        println!("Angle worked");



        // Curves


        let curve_sets: &[&[&str]] = &[

            // these are default curve settings from the OpenFace driver (the driver used in the UMD we reference for the testing)
            // I will try to give every driver these sorts of configs 

            curves::right_upper_lip,
            curves::left_upper_lip,
            curves::left_lower_lip,
            curves::right_lower_lip,

            curves::right_upper_inner_lip,
            curves::left_upper_inner_lip,
            curves::left_lower_inner_lip,
            curves::right_lower_inner_lip,
        ];

        let curve_results = CurveCalculator::fit_curve(&umd_instance, curve_sets);
        let file_name = "curves.parquet";
        let curve_output_path = format!("{output_path}{file_name}"); 
        CoreCurve::save_curve_to_parquet(&curve_results, &curve_output_path).expect("Failed to write curves to parquet");

        println!("Curves worked");



        // Area

        let area_sets: &[&[&str]] = &[
            areas::outer_lip_area,
            areas::inner_lip_area,
        ];

        let basis_sets: &[&[&str; 4]] = &[
            &areas::outer_basis_landmarks,
            &areas::inner_basis_landmarks,
        ];

        let area_results = AreaCalculator::calculate_area(&curve_results, &umd_instance, &basis_sets, &area_sets);
        let file_name = "areas.parquet";
        let area_output_path = format!("{output_path}{file_name}"); 
        CoreArea::save_area_to_parquet(&area_results, &area_output_path).expect("Failed to write area to parquet");

        Ok(())
    }

    pub fn test_python() -> PyResult<()> {
        PythonEnvironment::ensure_python_bridge().expect("Venv failed");
    
        Python::attach(|py| {
            if let Err(e) = test_function(py) {
                eprintln!("Failed to attach to Python interpreter: {:?}", e);
            }
        Ok(())
        })
    }
}

