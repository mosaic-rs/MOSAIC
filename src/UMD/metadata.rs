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
METADATA.rs 

Defines metadata struct as a hashmap to be stored in the UMD Parquet footer
*/

use std::collections::HashMap;

pub struct Metadata {
    pub UMD_Version: String,
    pub driver: String,
    pub dimension: String,
    pub centered: bool,
    pub pose_correction: bool,
}

impl Metadata {
    pub fn new(UMD_Version: String, driver: String, dimension: String, 
               centered: bool, pose_correction: bool) -> Self {
        Self {
            UMD_Version,
            driver,
            dimension,
            centered,
            pose_correction,
        }
    }

    pub fn to_kv_vec(&self) -> Vec<(String, String)> {
        vec![
            ("UMD_Version".to_string(), self.UMD_Version.clone()),
            ("driver".to_string(), self.driver.clone()),
            ("dimension".to_string(), self.dimension.clone()),
            ("centered".to_string(), self.centered.to_string()),
            ("pose_correction".to_string(), self.pose_correction.to_string()),
        ]
    }
} 