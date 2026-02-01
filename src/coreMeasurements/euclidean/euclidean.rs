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
EUCLIDEAN CALCULATOR

Currently it can only calculated the euclidean distance (or radius) of points from the origin but I hope to 
make it so you can calculate the distance between points. It is just a logic problem I won't tackle yet so 
I can get MOSAIC 1.0.0 out.
*/

use crate::UMD::UMD::{UMD};

pub struct CoreEuclidean {
    pub frame: Vec<u32>,
    pub timestamp: Vec<f32>,

    // Point 1 (Almost ALWAYS origin but I want to make it so people can calculate the distanc between other points too)

    pub coordinate_number_1: Vec<u32>, // if not a specified point it'll just say 0
    pub coordinate_type_1: Vec<String>, // if not specified, point will just be "origin" 
                                        // but this is important when ca;culating the distance between points
    pub x1: Vec<f64>, // USUALLY 0 
    pub y1: Vec<f64>, // USUALLY 0 
    pub z1: Vec<f64>, // optional - USUALLY 0 

    // Point 2
    pub coordinate_number_2: Vec<u32>, 
    pub coordinate_type_2: Vec<String>,
                                       
    pub x2: Vec<f64>, 
    pub y2: Vec<f64>, 
    pub z2: Vec<f64>, // optional    

    // r
    pub r: Vec<f64>,
    pub r_uncertainty: Vec<f64>,
}

// DATA STUFF:

impl CoreEuclidean {
    pub fn construction(total_frames: u32, points_per_frame: u32) -> Self {
        let total_entries = total_frames * points_per_frame;

        Self {
            frame: Vec::with_capacity(total_entries.try_into().unwrap()),
            timestamp: Vec::with_capacity(total_entries.try_into().unwrap()),

            coordinate_number_1: Vec::with_capacity(total_entries.try_into().unwrap()),
            coordinate_type_1: Vec::with_capacity(total_entries.try_into().unwrap()),
            x1: Vec::with_capacity(total_entries.try_into().unwrap()),
            y1: Vec::with_capacity(total_entries.try_into().unwrap()),
            z1: Vec::with_capacity(total_entries.try_into().unwrap()),

            coordinate_number_2: Vec::with_capacity(total_entries.try_into().unwrap()),
            coordinate_type_2: Vec::with_capacity(total_entries.try_into().unwrap()),
            x2: Vec::with_capacity(total_entries.try_into().unwrap()),
            y2: Vec::with_capacity(total_entries.try_into().unwrap()),
            z2: Vec::with_capacity(total_entries.try_into().unwrap()),

            r: Vec::with_capacity(total_entries.try_into().unwrap()),
            r_uncertainty: Vec::with_capacity(total_entries.try_into().unwrap()),
        }
    }

    pub fn add_point(
        &mut self, frame: u32, timestamp: f32, 
        coordinate_number_1: u32, coordinate_type_1: String, x1: f64, y1: f64, z1: f64, 
        coordinate_number_2: u32, coordinate_type_2: String, x2: f64, y2: f64, z2: f64, 
        r: f64, r_uncertainty: f64
    ) {
        self.frame.push(frame);
        self.timestamp.push(timestamp);
        self.coordinate_number_1.push(coordinate_number_1);
        self.coordinate_type_1.push(coordinate_type_1);
        self.x1.push(x1);
        self.y1.push(y1);
        self.z1.push(z1);
        self.coordinate_number_2.push(coordinate_number_2);
        self.coordinate_type_2.push(coordinate_type_2);
        self.x2.push(x2);
        self.y2.push(y2);
        self.z2.push(z2);
        self.r.push(r);
        self.r_uncertainty.push(r_uncertainty);
    }
}

// MATH PART:

pub struct CoreMeasurementAnchor;

impl CoreMeasurementAnchor {
    pub fn calculate_euclidean_distance(){
        println!("calc euclid dist func placeholder")
    }
}