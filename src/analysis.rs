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


use std::path::Path;

pub struct run;

impl run {
    // currently very simple as we aren't passing complex arguments
    // rn it is just for testing stuff

    pub fn init() -> Result<(), Box<dyn std::error::Error>> {
        // init is a general run command
        // we can edit it to pass paremeters through later
        let PATH_TEMP: &str = "/Users/harrywoodhouse/MOSAIC/MOSAIC/MOSAIC-Engine/test_data/v15044gf0000d1dlc67og65r2deqmhd0.csv";
        let umd_data = parse_openface_data(Path::new(PATH_TEMP)).expect("Failed to parse data");
        UMDDriver::save_umd_driver_to_parquet(&umd_data, "data/umd_driver.parquet");

        // anchor testing

        let anchor_results = AnchorProcessor::calculate_umd_anchors(&umd_data)?;
        AnchorProcessor::save_anchors_to_parquet(&anchor_results, "/Users/harrywoodhouse/MOSAIC/MOSAIC/MOSAIC-Engine/data/output_anchors.parquet")?;

        // centering testing

        let centering_results = CenteringProcessor::calculate_centering(&umd_data, &anchor_results)?;
        CenteringProcessor::save_centered_to_parquet(&centering_results, "/Users/harrywoodhouse/MOSAIC/MOSAIC/MOSAIC-Engine/data/output_centering.parquet")?;

        // pose correction testing

        let pose_correction_results = PoseProcessor::calculate_pose_corr(&centering_results)?;
        PoseProcessor::save_pose_to_parquet(&pose_correction_results, "/Users/harrywoodhouse/MOSAIC/MOSAIC/MOSAIC-Engine/data/output_pose_correction.parquet")?;

        // Final UMD output

        //aw: &UMDDriver, anchor: &UMDAnchor, centered: &UMDCentered, rotated: &UMDPose
        let total_entries = centering_results.x.len() as u32;        
        let mut umd_instance = UMD::construction(total_entries, 1);
        umd_instance.add_point(&umd_data, &anchor_results, &centering_results, &pose_correction_results);

        UMD::save_umd_to_parquet(&umd_instance, "/Users/harrywoodhouse/MOSAIC/MOSAIC/MOSAIC-Engine/data/UMD.parquet")?;
        Ok(())
    }

}

