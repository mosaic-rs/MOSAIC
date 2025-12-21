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

// This will act as an API for the drivers so they can append data properly
use crate::drivers::OpenFace::openface::{OpenFaceLandmarkType};

pub struct UMD {
    frame: Vec<u32>,
    timestamp: Vec<f32>,
    coordinate_number: Vec<u32>,
    types: Vec<String>, // we need to know whether or not this point was a commissure, philtrum, etc - defulat lip points can just be called "point"
    x: Vec<f64>,
    y: Vec<f64>,
    z: Vec<f64>,
}

// POINT ORDER:
// LIPS (CLOCKWISE ALWAYS) -> TONGUE -> JAW -> OTHERS
// LANDMARK TYPE STRUCTURE HAS BEEN MOVED TO BE DRIVER SPECIFIC

// UMD STRUCTURE:

// frame: | time_stamp | coordinate_number |        type        |     x     |     y     |     z    
// 1        0.01        1                   InnerLeftCommissure  0.0234      323.3276    10942
// 1        0.01        2                   InnerUpperLip        2.3234      323.3276    10942
// 1        0.01        3                   ...                  ...         ...         ...
// 1        0.01        4                   ...                  ...         ...         ...
// 1        0.01        5                   ...                  ...         ...         ...
// 1        0.01        6                   ...                  ...         ...         ...


impl UMD{
    pub fn construction(total_frames: u32, points_per_frame: u32)-> Self{
        // reserves the memory needed based on the frame count and the points per frame
        let mut total_entries = total_frames * points_per_frame;

        Self {
            frame: Vec::with_capacity(total_entries.try_into().unwrap()),
            timestamp: Vec::with_capacity(total_entries.try_into().unwrap()),
            coordinate_number: Vec::with_capacity(total_entries.try_into().unwrap()),
            types: Vec::with_capacity(total_entries.try_into().unwrap()),
            x: Vec::with_capacity(total_entries.try_into().unwrap()),
            y: Vec::with_capacity(total_entries.try_into().unwrap()),
            z: Vec::with_capacity(total_entries.try_into().unwrap()),
        }

    }

    pub fn add_point(&mut self, frame: u32, time: f32, number: u32, types: String, x: f64, y: f64, z: f64) {
        
        self.frame.push(frame);
        self.timestamp.push(time);
        self.coordinate_number.push(number);
        self.types.push(types);
        self.x.push(x);
        self.z.push(z);
        self.y.push(y);
        
    }
}
