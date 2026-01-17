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


// module imports:
use crate::shell::projectManager::session::{SessionData, DirectoryVerifiers, SystemVerifier};
use crate::drivers::OpenFace::openface::{parse_openface_data};
use crate::coreMeasurements::anchor::anchor::{AnchorProcessor};


use std::path::Path;

pub struct run;

impl run {
    // currently very simple as we aren't passing complex arguments
    // rn it is just for testing stuff

    pub fn init() {
        // init is a general run command
        // we can edit it to pass paremeters through later
        let PATH_TEMP: &str = "test_data/v15044gf0000d1dlc67og65r2deqmhd0.csv";
        let umd_data = parse_openface_data(Path::new(PATH_TEMP)).expect("Failed to parse data");

        let anchor_results = AnchorProcessor::calculate_umd_anchors(&umd_data);
    }

}

