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

/// ANALYSIS
// Acts as a conductor for mosaic
// Keeps shell lean


// Session info:
use crate::shell::projectManager::session::{SessionData, DirectoryVerifiers, SystemVerifier};

// Drivers
use crate::drivers::OpenFace::openface::{parse_openface_data};

// UMD
use crate::UMD::anchor::anchor::{AnchorProcessor};
use crate::UMD::centering::centering::{CenteringProcessor};
use crate::UMD::pose::pose::{PoseProcessor};
use crate::UMD::UMD::{UMD, UMDDriver};
use crate::UMD::metadata::{Metadata};

// praat analysis
use crate::praatAnalysis::testing::{tests};


use std::path::Path;

pub struct run;

impl run {
    // currently very simple as we aren't passing complex arguments
    // rn it is just for testing stuff

    pub fn init(input_path: &str, output_path: &str) -> Result<(), Box<dyn std::error::Error>> {
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
        Ok(())
    }

    pub fn test_python() -> Result<(), Box<dyn std::error::Error>> {
        tests::test_main();

        Ok(())
    }

}

