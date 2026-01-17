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
use polars::prelude::*;

pub struct UMD {
    // admin info
    pub frame: Vec<u32>,
    pub timestamp: Vec<f32>,
    pub pose: Vec<bool>, // Because some pose values might be 0, we need a seperate bool value to determine if we are processing 

    // pose values
    pub pose_x: Vec<f64>,
    pub pose_y: Vec<f64>,
    pub pose_z: Vec<f64>,

    pub coordinate_number: Vec<u32>,
    pub types: Vec<String>, // we need to know whether or not this point was a commissure, philtrum, etc - defulat lip points can just be called "point"
    pub x: Vec<f64>,
    pub y: Vec<f64>,
    pub z: Vec<f64>,
}

// POINT ORDER:
// LIPS (CLOCKWISE ALWAYS) -> TONGUE -> JAW -> OTHERS
// LANDMARK TYPE STRUCTURE HAS BEEN MOVED TO BE DRIVER SPECIFIC

// UMD STRUCTURE:
// u32      f32          bool    f64        f64        f64        u32                        u32/String       f64         f64         f64  
// frame: | time_stamp | pose |  pose_x  |  pose_y  |  pose_z  |  coordinate_number |        type        |     x     |     y     |     z    
// 1        0.01        1      0.235      ...        ...         1                   InnerLeftCommissure  0.0234      323.3276    10942
// 1        0.01        1      4.252      ...        ...         2                   InnerUpperLip        2.3234      323.3276    10942
// 1        0.01        1      ...        ...        ...         3                   ...                  ...         ...         ...
// 1        0.01        1      ...        ...        ...         4                   ...                  ...         ...         ...
// 1        0.01        1      ...        ...        ...         5                   ...                  ...         ...         ...
// 1        0.01        1      ...        ...        ...         6                   ...                  ...         ...         ...

impl UMD{
    pub fn construction(total_frames: u32, points_per_frame: u32)-> Self{
        // reserves the memory needed based on the frame count and the points per frame
        let mut total_entries = total_frames * points_per_frame;

        Self {
            frame: Vec::with_capacity(total_entries.try_into().unwrap()),
            timestamp: Vec::with_capacity(total_entries.try_into().unwrap()),
            pose: Vec::with_capacity(total_entries.try_into().unwrap()),
            pose_x: Vec::with_capacity(total_entries.try_into().unwrap()),
            pose_y: Vec::with_capacity(total_entries.try_into().unwrap()),
            pose_z: Vec::with_capacity(total_entries.try_into().unwrap()),
            coordinate_number: Vec::with_capacity(total_entries.try_into().unwrap()),
            types: Vec::with_capacity(total_entries.try_into().unwrap()),
            x: Vec::with_capacity(total_entries.try_into().unwrap()),
            y: Vec::with_capacity(total_entries.try_into().unwrap()),
            z: Vec::with_capacity(total_entries.try_into().unwrap()),
        }

    }

    pub fn add_point(&mut self, frame: u32, time: f32, confidence: f32, pose: bool, pose_x: f64, pose_y: f64, pose_z: f64, number: u32, types: String, x: f64, y: f64, z: f64) {
        
        self.frame.push(frame);
        self.timestamp.push(time);
        self.pose.push(pose);
        self.pose_x.push(pose_x);
        self.pose_y.push(pose_y);
        self.pose_z.push(pose_z);
        self.coordinate_number.push(number);
        self.types.push(types);
        self.x.push(x);
        self.z.push(z);
        self.y.push(y);
        
    }
}

// saving to UMD parquet


// ANCHOR UMD
// Defining a point anchor is essential to centering the points to (0,0,0)


#[derive(Debug)]
pub struct UMDAnchor {
    // adming info
    pub frame: Vec<u32>,
    pub timestamp: Vec<f32>,

    // anchor values
    pub x_anchor: Vec<f64>,
    pub y_anchor: Vec<f64>,
    pub z_anchor: Vec<f64>,

    // uncertainty
    pub x_anchor_uncertainty: Vec<f64>,
    pub y_anchor_uncertainty: Vec<f64>,
    pub z_anchor_uncertainty: Vec<f64>,
}

impl UMDAnchor{
    pub fn construction(total_frames: u32) -> Self{
        let mut total_entries = total_frames;
        Self {
            frame: Vec::with_capacity(total_entries.try_into().unwrap()),
            timestamp: Vec::with_capacity(total_entries.try_into().unwrap()), 
            x_anchor: Vec::with_capacity(total_entries.try_into().unwrap()),
            y_anchor: Vec::with_capacity(total_entries.try_into().unwrap()),
            z_anchor: Vec::with_capacity(total_entries.try_into().unwrap()),
            x_anchor_uncertainty: Vec::with_capacity(total_entries.try_into().unwrap()),
            y_anchor_uncertainty: Vec::with_capacity(total_entries.try_into().unwrap()),
            z_anchor_uncertainty: Vec::with_capacity(total_entries.try_into().unwrap()),
        }
    }

    pub fn add_anchor(&mut self, frame: u32, timestamp: f32, x_anchor: f64, y_anchor: f64, z_anchor: f64, x_anchor_uncertainty: f64, y_anchor_uncertainty: f64, z_anchor_uncertainty: f64) { //, x_anchor_uncertainty: f64, y_anchor_uncertainty: f64, z_anchor_uncertainty: f64
        self.frame.push(frame);
        self.timestamp.push(timestamp);
        self.x_anchor.push(x_anchor);
        self.y_anchor.push(y_anchor);
        self.z_anchor.push(z_anchor);
        self.x_anchor_uncertainty.push(x_anchor_uncertainty);
        self.y_anchor_uncertainty.push(y_anchor_uncertainty);
        self.z_anchor_uncertainty.push(z_anchor_uncertainty);

        // println!("{:?}", self);
    }
}


// CENTERED POINTS UMD
// Same struct as UMD but for centered points

#[derive(Debug)]
pub struct CenteredUMD {
    pub frame: Vec<u32>,
    pub timestamp: Vec<f32>,
    pub pose: Vec<bool>, // Because some pose values might be 0, we need a seperate bool value to determine if we are processing 

    // pose values
    pub pose_x: Vec<f64>,
    pub pose_y: Vec<f64>,
    pub pose_z: Vec<f64>,

    pub coordinate_number: Vec<u32>,
    pub types: Vec<String>, // we need to know whether or not this point was a commissure, philtrum, etc - defulat lip points can just be called "point"
    pub x: Vec<f64>,
    pub y: Vec<f64>,
    pub z: Vec<f64>,
}

impl CenteredUMD {
    pub fn construction(total_frames: u32, points_per_frame: u32) -> Self {
        // reserving space for data in memory
        // the same as the UMD struct
        
        frame: Vec::with_capacity(total_entries.try_into().unwrap()),
        timestamp: Vec::with_capacity(total_entries.try_into().unwrap()),
        pose: Vec::with_capacity(total_entries.try_into().unwrap()),
        pose_x: Vec::with_capacity(total_entries.try_into().unwrap()),
        pose_y: Vec::with_capacity(total_entries.try_into().unwrap()),
        pose_z: Vec::with_capacity(total_entries.try_into().unwrap()),
        coordinate_number: Vec::with_capacity(total_entries.try_into().unwrap()),
        types: Vec::with_capacity(total_entries.try_into().unwrap()),
        x: Vec::with_capacity(total_entries.try_into().unwrap()),
        y: Vec::with_capacity(total_entries.try_into().unwrap()),
        z: Vec::with_capacity(total_entries.try_into().unwrap()),
    }

    pub fn add_point(&mut self, frame: u32, time: f32, confidence: f32, pose: bool, pose_x: f64, pose_y: f64, pose_z: f64, number: u32, types: String, x: f64, y: f64, z: f64) {
        
        self.frame.push(frame);
        self.timestamp.push(time);
        self.pose.push(pose);
        self.pose_x.push(pose_x);
        self.pose_y.push(pose_y);
        self.pose_z.push(pose_z);
        self.coordinate_number.push(number);
        self.types.push(types);
        self.x.push(x);
        self.z.push(z);
        self.y.push(y);

    }
}