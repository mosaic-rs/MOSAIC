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
    pub driver: String,
    pub dimension: String,
    pub pose_correction: bool,
}

impl Metadata {
    pub fn into_metadata(self) -> HashMap<String, String> {
        let mut metadata = HashMap::new();

        /*
        I will try to add metadata rules to the drivers. For example, maybe a certain driver HAS to be 3D, or a certain driver
        doesn't support pose correction. 
        */

        metadata.insert("driver".to_string(), self.driver); 
        metadata.insert("dimension".to_string(), self.dimension);
        metadata.insert("pose_correction".to_string(), self.pose_correction.to_string());

        metadata
    }
} 