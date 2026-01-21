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
use std::fs::File;

/*
As a note, this file is quite grandiose as it is helpful in visualising exactly what
is happening to the data. This is not a finished product.
*/

// POINT ORDER:
// LIPS (CLOCKWISE ALWAYS) -> TONGUE -> JAW -> OTHERS
// LANDMARK TYPE STRUCTURE HAS BEEN MOVED TO BE DRIVER SPECIFIC

#[derive(Debug)]
pub struct UMD {
    pub frame: Vec<u32>,
    pub timestamp: Vec<f32>,
    pub confidence: Vec<f32>, // openface confidence value between 0-1
    pub pose: Vec<bool>,
    pub coordinate_number: Vec<u32>,
    pub types: Vec<String>,

    // pose
    pub pose_x: Vec<f64>,
    pub pose_y: Vec<f64>,
    pub pose_z: Vec<f64>,
    pub pose_x_uncertainty: Vec<f64>, // data from calibration process - OPTIONAL (i.e. calibration is not required)
    pub pose_y_uncertainty: Vec<f64>, // data from calibration process - OPTIONAL (i.e. calibration is not required)
    pub pose_z_uncertainty: Vec<f64>, // data from calibration process - OPTIONAL (i.e. calibration is not required & z axis is optional)

    // raw coordinates
    pub x_raw: Vec<f64>,
    pub y_raw: Vec<f64>,
    pub z_raw: Vec<f64>, // OPTIONAL (i.e. z axis is optional)
    pub x_raw_uncertainty: Vec<f64>, // data from calibration process - OPTIONAL (i.e. calibration is not required)
    pub y_raw_uncertainty: Vec<f64>, // data from calibration process - OPTIONAL (i.e. calibration is not required)
    pub z_raw_uncertainty: Vec<f64>, // data from calibration process - OPTIONAL (i.e. calibration is not required & z axis is optional)

    // centered coordinates
    pub x_centered: Vec<f64>,
    pub y_centered: Vec<f64>,
    pub z_centered: Vec<f64>, // OPTIONAL (i.e. z axis is optional)
    // I suspect centering which combines the raw_uncertainty and anchor_uncertainty will change uncertainty for x/y/z_centered
    pub x_centered_uncertainty: Vec<f64>,
    pub y_centered_uncertainty: Vec<f64>,
    pub z_centered_uncertainty: Vec<f64>,

    // pose corrected coordinates
    pub x_rotated: Vec<f64>,
    pub y_rotated: Vec<f64>,
    pub z_rotated: Vec<f64>, // OPTIONAL (i.e. z axis is optional)
    // I suspect centering which combines the x/y/z_centered_uncertainty and pose_uncertainty will change uncertainty for x/y/z_rotated
    pub x_rotated_uncertainty: Vec<f64>, // data from calibration process - OPTIONAL (i.e. calibration is not required)
    pub y_rotated_uncertainty: Vec<f64>, // data from calibration process - OPTIONAL (i.e. calibration is not required)
    pub z_rotated_uncertainty: Vec<f64>, // data from calibration process - OPTIONAL (i.e. calibration is not required & z axis is optional)


    // anchor
    pub x_anchor: Vec<f64>,
    pub y_anchor: Vec<f64>,
    pub z_anchor: Vec<f64>, // OPTIONAL (i.e. z axis is optional)
    pub x_anchor_uncertainty: Vec<f64>, // data from calibration process - OPTIONAL (i.e. calibration is not required)
    pub y_anchor_uncertainty: Vec<f64>, // data from calibration process - OPTIONAL (i.e. calibration is not required)
    pub z_anchor_uncertainty: Vec<f64>, // data from calibration process - OPTIONAL (i.e. calibration is not required & z axis is optional)
}

impl UMD {
    pub fn construction(total_frames: u32, points_per_frame: u32)-> Self{
        // reserves the memory needed based on the frame count and the points per frame
        let mut total_entries = total_frames * points_per_frame;

        Self {
            frame: Vec::with_capacity(total_entries.try_into().unwrap()),
            timestamp: Vec::with_capacity(total_entries.try_into().unwrap()),
            confidence: Vec::with_capacity(total_entries.try_into().unwrap()),
            pose: Vec::with_capacity(total_entries.try_into().unwrap()),
            coordinate_number: Vec::with_capacity(total_entries.try_into().unwrap()),
            types: Vec::with_capacity(total_entries.try_into().unwrap()),

            // pose
            pose_x: Vec::with_capacity(total_entries.try_into().unwrap()),
            pose_y: Vec::with_capacity(total_entries.try_into().unwrap()),
            pose_z: Vec::with_capacity(total_entries.try_into().unwrap()),
            pose_x_uncertainty: Vec::with_capacity(total_entries.try_into().unwrap()),
            pose_y_uncertainty: Vec::with_capacity(total_entries.try_into().unwrap()),
            pose_z_uncertainty: Vec::with_capacity(total_entries.try_into().unwrap()),

            // raw coordinates
            x_raw: Vec::with_capacity(total_entries.try_into().unwrap()),
            y_raw: Vec::with_capacity(total_entries.try_into().unwrap()),
            z_raw: Vec::with_capacity(total_entries.try_into().unwrap()),
            x_raw_uncertainty: Vec::with_capacity(total_entries.try_into().unwrap()),
            y_raw_uncertainty: Vec::with_capacity(total_entries.try_into().unwrap()),
            z_raw_uncertainty: Vec::with_capacity(total_entries.try_into().unwrap()),

            // centered coordinates
            x_centered: Vec::with_capacity(total_entries.try_into().unwrap()),
            y_centered: Vec::with_capacity(total_entries.try_into().unwrap()),
            z_centered: Vec::with_capacity(total_entries.try_into().unwrap()),
            x_centered_uncertainty: Vec::with_capacity(total_entries.try_into().unwrap()),
            y_centered_uncertainty: Vec::with_capacity(total_entries.try_into().unwrap()),
            z_centered_uncertainty: Vec::with_capacity(total_entries.try_into().unwrap()),

            x_rotated: Vec::with_capacity(total_entries.try_into().unwrap()),
            y_rotated: Vec::with_capacity(total_entries.try_into().unwrap()),
            z_rotated: Vec::with_capacity(total_entries.try_into().unwrap()),
            x_rotated_uncertainty: Vec::with_capacity(total_entries.try_into().unwrap()),
            y_rotated_uncertainty: Vec::with_capacity(total_entries.try_into().unwrap()),
            z_rotated_uncertainty: Vec::with_capacity(total_entries.try_into().unwrap()),


            // anchor
            x_anchor: Vec::with_capacity(total_entries.try_into().unwrap()),
            y_anchor: Vec::with_capacity(total_entries.try_into().unwrap()),
            z_anchor: Vec::with_capacity(total_entries.try_into().unwrap()),
            x_anchor_uncertainty: Vec::with_capacity(total_entries.try_into().unwrap()),
            y_anchor_uncertainty: Vec::with_capacity(total_entries.try_into().unwrap()),
            z_anchor_uncertainty: Vec::with_capacity(total_entries.try_into().unwrap()),
        }

    }

    pub fn add_point(&mut self, raw: &UMDDriver, anchor: &UMDAnchor, centered: &UMDCentered, rotated: &UMDPose) {
        
            self.frame = (raw.frame);
            self.timestamp = (raw.timestamp);
            self.confidence = (raw.confidence); // needs adding - IMPLEMENTED IN SUBSCTRUCT
            self.pose = (raw.pose);
            self.coordinate_number = (raw.coordinate_number);
            self.types = (raw.types);

            // pose
            self.pose_x = (raw.pose_x);
            self.pose_y = (raw.pose_y);
            self.pose_z = (raw.pose_z);
            self.pose_x_uncertainty = (raw.pose_x_uncertainty); // needs adding - IMPLEMENTED IN SUBSCTRUCT
            self.pose_y_uncertainty = (raw.pose_y_uncertainty); // needs adding - IMPLEMENTED IN SUBSCTRUCT
            self.pose_z_uncertainty = (raw.pose_z_uncertainty); // needs adding - IMPLEMENTED IN SUBSCTRUCT

            // raw coordinates
            self.x_raw = (raw.x);
            self.y_raw = (raw.y);
            self.z_raw = (raw.z);
            self.x_raw_uncertainty = (raw.x_uncertainty); // needs adding - IMPLEMENTED IN SUBSCTRUCT
            self.y_raw_uncertainty = (raw.y_uncertainty); // needs adding - IMPLEMENTED IN SUBSCTRUCT
            self.z_raw_uncertainty = (raw.z_uncertainty); // needs adding - IMPLEMENTED IN SUBSCTRUCT

            // centered coordinates
            self.x_centered = (centered.x);
            self.y_centered = (centered.y);
            self.z_centered = (centered.z);
            self.x_centered_uncertainty = (centered.x_uncertainty); // needs adding - IMPLEMENTED IN SUBSCTRUCT
            self.y_centered_uncertainty = (centered.y_uncertainty); // needs adding - IMPLEMENTED IN SUBSCTRUCT
            self.z_centered_uncertainty = (centered.z_uncertainty); // needs adding - IMPLEMENTED IN SUBSCTRUCT

            // pose corrected coordinates
            self.x_rotated = (rotated.x);
            self.y_rotated = (rotated.y);
            self.z_rotated = (rotated.z);
            self.x_rotated_uncertainty = (rotated.x_uncertainty); // needs adding - IMPLEMENTED IN SUBSCTRUCT
            self.y_rotated_uncertainty = (rotated.y_uncertainty); // needs adding - IMPLEMENTED IN SUBSCTRUCT
            self.z_rotated_uncertainty = (rotated.z_uncertainty); // needs adding - IMPLEMENTED IN SUBSCTRUCT

            // anchor
            self.x_anchor = (anchor.x_anchor);
            self.y_anchor = (anchor.y_anchor);
            self.z_anchor = (anchor.z_anchor);
            self.x_anchor_uncertainty = (anchor.x_anchor_uncertainty); // needs adding - IMPLEMENTED IN SUBSCTRUCT
            self.y_anchor_uncertainty = (anchor.y_anchor_uncertainty); // needs adding - IMPLEMENTED IN SUBSCTRUCT
            self.z_anchor_uncertainty = (anchor.z_anchor_uncertainty); // needs adding - IMPLEMENTED IN SUBSCTRUCT
    }

    pub fn save_umd_driver_to_parquet(data: &UMD, file_path: &str) -> PolarsResult<()> {
        let s_frame = Series::new("frame", &data.frame);
        let s_timestamp = Series::new("timestamp", &data.timestamp);
        let s_confidence = Series::new("confidence", &data.confidence); // needs adding
        let s_pose = Series::new("pose", &data.pose);
        let s_coordinate_number = Series::new("coordinate_number", &data.coordinate_number);
        let s_type = Series::new("type", &data.types);
        
        let s_pose_x = Series::new("pose_x", &data.pose_x);
        let s_pose_y = Series::new("pose_y", &data.pose_y);
        let s_pose_z = Series::new("pose_z", &data.pose_z);
        let s_pose_x_uncertainty = Series::new("pose_x_uncertainty", &data.pose_x_uncertainty); // needs adding - IMPLEMENTED IN SUBSCTRUCT
        let s_pose_y_uncertainty = Series::new("pose_y_uncertainty", &data.pose_y_uncertainty); // needs adding - IMPLEMENTED IN SUBSCTRUCT
        let s_pose_z_uncertainty = Series::new("pose_z_uncertainty", &data.pose_z_uncertainty); // needs adding - IMPLEMENTED IN SUBSCTRUCT

        let s_x_raw = Series::new("x_raw", &data.x_raw);
        let s_y_raw = Series::new("y_raw", &data.y_raw);
        let s_z_raw = Series::new("z_raw", &data.z_raw);
        let s_x_raw_uncertainty = Series::new("x_raw_uncertainty", &data.x_raw_uncertainty); // needs adding - IMPLEMENTED IN SUBSCTRUCT
        let s_y_raw_uncertainty = Series::new("y_raw_uncertainty", &data.y_raw_uncertainty); // needs adding - IMPLEMENTED IN SUBSCTRUCT
        let s_z_raw_uncertainty = Series::new("z_raw_uncertainty", &data.z_raw_uncertainty); // needs adding - IMPLEMENTED IN SUBSCTRUCT

        let s_x_centered = Series::new("x_centered", &data.x_centered);
        let s_y_centered = Series::new("y_centered", &data.y_centered);
        let s_z_centered = Series::new("z_centered", &data.z_centered);
        let s_x_centered_uncertainty = Series::new("x_centered_uncertainty", &data.x_centered_uncertainty); // needs adding - IMPLEMENTED IN SUBSCTRUCT
        let s_y_centered_uncertainty = Series::new("y_centered_uncertainty", &data.y_centered_uncertainty); // needs adding - IMPLEMENTED IN SUBSCTRUCT
        let s_z_centered_uncertainty = Series::new("z_centered_uncertainty", &data.z_centered_uncertainty); // needs adding - IMPLEMENTED IN SUBSCTRUCT

        let s_x_rotated = Series::new("x_rotated", &data.x_rotated);
        let s_y_rotated = Series::new("y_rotated", &data.y_rotated);
        let s_z_rotated = Series::new("z_rotated", &data.z_rotated);
        let s_x_rotated_uncertainty = Series::new("x_rotated_uncertainty", &data.x_rotated_uncertainty); // needs adding - IMPLEMENTED IN SUBSCTRUCT
        let s_y_rotated_uncertainty = Series::new("y_rotated_uncertainty", &data.y_rotated_uncertainty); // needs adding - IMPLEMENTED IN SUBSCTRUCT
        let s_z_rotated_uncertainty = Series::new("z_rotated_uncertainty", &data.z_rotated_uncertainty); // needs adding - IMPLEMENTED IN SUBSCTRUCT

        let s_x_anchor = Series::new("x_anchor", &data.x_anchor);
        let s_y_anchor = Series::new("y_anchor", &data.y_anchor);
        let s_z_anchor = Series::new("z_anchor", &data.z_anchor);
        let s_x_anchor_uncertainty = Series::new("x_anchor_uncertainty", &data.x_anchor_uncertainty); // needs adding
        let s_y_anchor_uncertainty = Series::new("y_anchor_uncertainty", &data.y_anchor_uncertainty); // needs adding
        let s_z_anchor_uncertainty = Series::new("z_anchor_uncertainty", &data.z_anchor_uncertainty); // needs adding

        let mut df = DataFrame::new(vec![
            s_frame, s_timestamp, s_confidence, s_pose, s_coordinate_number, s_type, 
            s_pose_x, s_pose_y, s_pose_z, s_pose_x_uncertainty, s_pose_y_uncertainty, s_pose_z_uncertainty, 
            s_x_raw, s_y_raw, s_z_raw, s_x_raw_uncertainty, s_y_raw_uncertainty, s_z_raw_uncertainty, 
            s_x_centered, s_y_centered, s_z_centered, s_x_centered_uncertainty, s_y_centered_uncertainty, s_z_centered_uncertainty, 
            s_x_rotated, s_y_rotated, s_z_rotated, s_x_rotated_uncertainty, s_y_rotated_uncertainty, s_z_rotated_uncertainty, 
            s_x_anchor, s_y_anchor, s_z_anchor, s_x_anchor_uncertainty, s_y_anchor_uncertainty, s_z_anchor_uncertainty
        ])?;

        let file = File::create(file_path).map_err(PolarsError::from)?;
        ParquetWriter::new(file).finish(&mut df)?;

        println!("Successfully exported raw UMD data to: {}", file_path);
        Ok(())
    }

}

// UMDDriver STRUCTURE: (Also the same for UMDCenter and UMDPose)
// u32      f32          bool    f64        f64        f64        u32                        u32/String       f64         f64         f64  
// frame: | time_stamp | pose |  pose_x  |  pose_y  |  pose_z  |  coordinate_number |        type        |     x     |     y     |     z    
// 1        0.01        1      0.235      ...        ...         1                   InnerLeftCommissure  0.0234      323.3276    10942
// 1        0.01        1      4.252      ...        ...         2                   InnerUpperLip        2.3234      323.3276    10942
// 1        0.01        1      ...        ...        ...         3                   ...                  ...         ...         ...
// 1        0.01        1      ...        ...        ...         4                   ...                  ...         ...         ...
// 1        0.01        1      ...        ...        ...         5                   ...                  ...         ...         ...
// 1        0.01        1      ...        ...        ...         6                   ...                  ...         ...         ...

// DRIVER UMD
// Communicates with the driver to extract raw data

pub struct UMDDriver {
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
    pub x_raw_uncertainty: Vec<f64>,
    pub y_raw_uncertainty: Vec<f64>,
    pub z_raw_uncertainty: Vec<f64>,


}

impl UMDDriver{
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
            types: Vec::with_capacity(total_entries.try_into().unwrap()),
            x: Vec::with_capacity(total_entries.try_into().unwrap()),
            y: Vec::with_capacity(total_entries.try_into().unwrap()),
            z: Vec::with_capacity(total_entries.try_into().unwrap()),
            x_raw_uncertainty: Vec::with_capacity(total_entries.try_into().unwrap()),
            y_raw_uncertainty: Vec::with_capacity(total_entries.try_into().unwrap()),
            z_raw_uncertainty: Vec::with_capacity(total_entries.try_into().unwrap()),
        }

    }

    pub fn add_point(&mut self, frame: u32, time: f32, confidence: f32, pose: bool, pose_x: f64, pose_y: f64, pose_z: f64, 
                     number: u32, types: String, x: f64, y: f64, z: f64, x_raw_uncertainty: f64, y_raw_uncertainty: f64, z_raw_uncertainty: f64) {
        
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
        self.x_raw_uncertainty.push(x_raw_uncertainty);
        self.y_raw_uncertainty.push(y_raw_uncertainty);
        self.z_raw_uncertainty.push(z_raw_uncertainty);
        
    }

    // writing UMDDriver to parquet (this logic is really just for testing so I can visualize the testing data better)

    pub fn save_umd_driver_to_parquet(data: &UMDDriver, file_path: &str) -> PolarsResult<()> {
        let s_frame = Series::new("frame", &data.frame);
        let s_time = Series::new("timestamp", &data.timestamp);
        let s_pose = Series::new("pose_detected", &data.pose);
        
        let s_px = Series::new("pose_Rx", &data.pose_x);
        let s_py = Series::new("pose_Ry", &data.pose_y);
        let s_pz = Series::new("pose_Rz", &data.pose_z);

        let s_num = Series::new("point_id", &data.coordinate_number);
        let s_type = Series::new("label", &data.types);

        let s_x = Series::new("x_raw", &data.x);
        let s_y = Series::new("y_raw", &data.y);
        let s_z = Series::new("z_raw", &data.z);

        let s_x_raw_uncertainty = Series::new("x_raw_uncertainty", &data.x_raw_uncertainty);
        let s_y_raw_uncertainty = Series::new("y_raw_uncertainty", &data.y_raw_uncertainty);
        let s_z_raw_uncertainty = Series::new("z_raw_uncertainty", &data.z_raw_uncertainty);

        let mut df = DataFrame::new(vec![
            s_frame, s_time, /*s_conf,*/s_pose, // I will add  confidence later
            s_px, s_py, s_pz, 
            s_num, s_type, 
            s_x, s_y, s_z,
            s_x_raw_uncertainty, s_y_raw_uncertainty, s_z_raw_uncertainty,
        ])?;

        let file = File::create(file_path).map_err(PolarsError::from)?;
        ParquetWriter::new(file).finish(&mut df)?;

        println!("Successfully exported raw UMD data to: {}", file_path);
        Ok(())
    }
}

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
pub struct UMDCentered {
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
    pub x_centered_uncertainty: Ved<f64>,
    pub y_centered_uncertainty: Ved<f64>,
    pub z_centered_uncertainty: Ved<f64>,
}


impl UMDCentered{
    pub fn construction(total_frames: u32, points_per_frame: u32) -> Self {
        // reserving space for data in memory
        // the same as the UMD struct
        let total_entries = total_frames * points_per_frame;
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
            x_centered_uncertainty: Vec::with_capacity(total_entries.try_into().unwrap()),
            y_centered_uncertainty: Vec::with_capacity(total_entries.try_into().unwrap()),
            z_centered_uncertainty: Vec::with_capacity(total_entries.try_into().unwrap()),
        }
    }

    pub fn add_point(&mut self, frame: u32, time: f32, confidence: f32, pose: bool, pose_x: f64, pose_y: f64, pose_z: f64, 
                     number: u32, types: String, x: f64, y: f64, z: f64, x_centered_uncertainty: f64, y_centered_uncertainty: f64, z_centered_uncertainty: f64) {
        
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
        self.x_centered_uncertainty.pushc(x_centered_uncertainty);
        self.y_centered_uncertainty.pushc(y_centered_uncertainty);
        self.z_centered_uncertainty.pushc(z_centered_uncertainty);

    }
}

// POSE CORRECTION UMD
// This should be the last major part of the UMD as all other measurements are dependent
// on the set of points extracted (i.e. curve fitting needs lip points which an EMA dataset
// may not include)

#[derive(Debug)]
pub struct UMDPose{
    // like UMD and Centering, this is the same struct design
    pub frame: Vec<u32>,
    pub timestamp: Vec<f32>,
    pub pose: Vec<bool>, // Because some pose values might be 0, we need a seperate bool value to determine if we are processing 

    // pose values
    pub pose_x: Vec<f64>,
    pub pose_y: Vec<f64>,
    pub pose_z: Vec<f64>,
    pub pose_x_uncertainty: Vec<f64>, // needs implementing
    pub pose_y_uncertainty: Vec<f64>, // needs implementing
    pub pose_z_uncertainty: Vec<f64>, // needs implementing

    pub coordinate_number: Vec<u32>,
    pub types: Vec<String>, // we need to know whether or not this point was a commissure, philtrum, etc - defulat lip points can just be called "point"
    pub x: Vec<f64>,
    pub y: Vec<f64>,
    pub z: Vec<f64>,
    pub x_rotated_uncertainty: Vec<f64>, // needs implementing
    pub y_rotated_uncertainty: Vec<f64>, // needs implementing
    pub z_rotated_uncertainty: Vec<f64>, // needs implementing
}

// again, quite similar to UMD and UMDCenter

impl UMDPose {
    pub fn construction(total_frames: u32, points_per_frame: u32) -> Self{

       let total_entries = total_frames * points_per_frame;
        Self {
            frame: Vec::with_capacity(total_entries.try_into().unwrap()),
            timestamp: Vec::with_capacity(total_entries.try_into().unwrap()),
            pose: Vec::with_capacity(total_entries.try_into().unwrap()),
            pose_x: Vec::with_capacity(total_entries.try_into().unwrap()),
            pose_y: Vec::with_capacity(total_entries.try_into().unwrap()),
            pose_z: Vec::with_capacity(total_entries.try_into().unwrap()),
            pose_x_uncertainty: Vec::with_capacity(total_entries.try_into().unwrap()), // needs implementing
            pose_y_uncertainty: Vec::with_capacity(total_entries.try_into().unwrap()), // needs implementing
            pose_z_uncertainty: Vec::with_capacity(total_entries.try_into().unwrap()), // needs implementing
             // needs implementing
            coordinate_number: Vec::with_capacity(total_entries.try_into().unwrap()),
            types: Vec::with_capacity(total_entries.try_into().unwrap()),
            x: Vec::with_capacity(total_entries.try_into().unwrap()),
            y: Vec::with_capacity(total_entries.try_into().unwrap()),
            z: Vec::with_capacity(total_entries.try_into().unwrap()),
            x_rotated_uncertainty: Vec::with_capacity(total_entries.try_into().unwrap()), // needs implementing
            y_rotated_uncertainty: Vec::with_capacity(total_entries.try_into().unwrap()), // needs implementing
            z_rotated_uncertainty: Vec::with_capacity(total_entries.try_into().unwrap()), // needs implementing
        } 
    }

    pub fn add_point(&mut self, frame: u32, time: f32, confidence: f32, pose: bool, pose_x: f64, pose_y: f64, pose_z: f64, 
                     pose_x_uncertainty: f64, pose_y_uncertainty: f64, pose_z_uncertainty: f64, number: u32, types: String, 
                     x: f64, y: f64, z: f64, x_rotated_uncertainty: f64, y_rotated_uncertainty: f64, z_rotated_uncertainty: f64) {
        
        self.frame.push(frame);
        self.timestamp.push(time);
        self.pose.push(pose);
        self.pose_x.push(pose_x);
        self.pose_y.push(pose_y);
        self.pose_z.push(pose_z);
        self.pose_x_uncertainty.push(pose_x_uncertainty); // needs implementing
        self.pose_y_uncertainty.push(pose_y_uncertainty); // needs implementing
        self.pose_z_uncertainty.push(pose_z_uncertainty); // needs implementing
        self.coordinate_number.push(number);
        self.types.push(types);
        self.x.push(x);
        self.z.push(z);
        self.y.push(y);
        self.x_rotated_uncertainty.push(x_rotated_uncertainty); // needs implementing
        self.y_rotated_uncertainty.push(y_rotated_uncertainty); // needs implementing
        self.z_rotated_uncertainty.push(z_rotated_uncertainty); // needs implementing

    }
}

/*
End of the core UMD functionality. 

The next work will introduce a cohesive file structure through an io
*/