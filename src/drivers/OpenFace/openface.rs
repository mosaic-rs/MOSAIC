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

use crate::UMD::UMD::{UMDDriver};
use crate::errors::{MosaicError, UMDError, FileError};

use std::path::Path;
use std::io::Read;
use std::fs::File;
use csv::Reader;
use std::io::{BufRead, BufReader};
use std::u32;


// all we have to do is 
//  read csv
//  extract landmarks
//  map landmarks to UMD struct
//  return that landmark


fn count_csv_rows(path: &Path) -> Result<u32, MosaicError> {
    let file = File::open(path)?; 
    let reader = BufReader::new(file);
    Ok(reader.lines().count() as u32)
}

#[derive(Debug, Clone, Copy)]
pub enum OpenFaceLandmarkType{
    LeftCommissure,
    RightCommissure,
    Philtrum,
    LowerVermillionBorder, // middle of the lip
    UpperLip, // all other points along the lip
    LowerLip, // all other points along the lip

    // Software like openface has outer and inner coordinates
    OuterLeftCommissure,
    OuterRightCommissure,
    InnerLeftCommissure,
    InnerRightCommissure,
    OuterPhiltrum,
    InnerPhiltrum,
    InnerLowerVermillionBorder,
    OuterLowerVermillionBorder,
    InnerUpperLip, 
    InnerLowerLip,
    OuterUpperLip,
    OuterLowerLip,

    // OTHER POINTS
    LeftJaw,
    RightJaw,
    CentreJaw, // chin
    NasalBridge, // top to bottom
    LeftNasalTip,
    NasalTip, // from left - right
    RightNasalTip,

    Unknown
}


impl OpenFaceLandmarkType{
    pub fn openface_index(index: usize) -> Self {
        // DEFINING MOUTH POINTS FIRST
        // LANDMARK MAP IS MIRRORED FROM IRL SO 48 IS REALLY THE RIGHT COMMISSURE NOT LEFT
        // LANDMARK DIRECTION: CLOCKWISE (BASED ON LANDMARK NUMBER MAP) - RIGHT TO LEFT (human perspective)
        match index {
            // Outer points
            48 => Self::OuterRightCommissure, // right
            49 => Self::OuterUpperLip, // right
            50 => Self::OuterUpperLip, // rught
            51 => Self::OuterPhiltrum, // mid
            52 => Self::OuterUpperLip, // left
            53 => Self::OuterUpperLip, // left
            54 => Self::OuterLeftCommissure, // left
            55 => Self::OuterLowerLip, // left
            56 => Self::OuterLowerLip, // left
            57 => Self::OuterLowerVermillionBorder, // mid
            58 => Self::OuterLowerLip, // right
            59 => Self::OuterLowerLip, // right
            // goes back to point 48 to make a closed loop

            // Inner points
            60 => Self::InnerRightCommissure, // right
            61 => Self::InnerUpperLip, // right
            62 => Self::InnerPhiltrum, // mid
            63 => Self::InnerUpperLip, // left
            64 => Self::InnerLeftCommissure, // left
            65 => Self::InnerLowerLip, // left
            66 => Self::InnerLowerVermillionBorder, // mid
            67 => Self::InnerLowerLip, // right
            // goes back to 60 to make closed loop

            // Jaw
            0 => Self::RightJaw,
            1 => Self::RightJaw,
            2 => Self::RightJaw,
            3 => Self::RightJaw,
            4 => Self::RightJaw,
            5 => Self::RightJaw,
            6 => Self::RightJaw,
            7 => Self::RightJaw,
            8 => Self::CentreJaw,
            9 => Self::LeftJaw,
            10 => Self::LeftJaw,
            11 => Self::LeftJaw,
            12 => Self::LeftJaw,
            13 => Self::LeftJaw,
            14 => Self::LeftJaw,
            15 => Self::LeftJaw,
            16 => Self::LeftJaw,

            // Nose
            // Nasal Bridge - top-to-bottom
            27 => Self::NasalBridge,
            28 => Self::NasalBridge,
            29 => Self::NasalBridge,
            30 => Self::NasalBridge,

            // Nasal Tip right - left
            31 => Self::RightNasalTip,
            32 => Self::RightNasalTip,
            33 => Self::NasalTip,
            34 => Self::LeftNasalTip,
            35 => Self::LeftNasalTip,

            _ => Self::Unknown,
        }
    }

    pub fn openface_index_label(&self) -> String{
        match self {
            Self::OuterLeftCommissure => "OuterLeftCommissure".to_string(),
            Self::OuterRightCommissure => "OuterRightCommissure".to_string(),
            Self::InnerLeftCommissure => "InnerLeftCommissure".to_string(),
            Self::InnerRightCommissure => "InnerRightCommissure".to_string(),
            Self::OuterPhiltrum => "OuterPhiltrum".to_string(),
            Self::InnerPhiltrum => "InnerPhiltrum".to_string(),
            Self::InnerLowerVermillionBorder => "InnerLowerVermillionBorder".to_string(),
            Self::OuterLowerVermillionBorder => "OuterLowerVermillionBorder".to_string(),
            Self::InnerUpperLip => "InnerUpperLip".to_string(),
            Self::InnerLowerLip => "InnerLowerLip".to_string(),
            Self::OuterUpperLip => "OuterUpperLip".to_string(),
            Self::OuterLowerLip => "OuterLowerLip".to_string(),

            Self::LeftJaw => "LeftJaw".to_string(),
            Self::RightJaw => "RightJaw".to_string(),
            Self::CentreJaw => "CentreJaw".to_string(), // chin
            Self::NasalBridge => "NasalBridge".to_string(), // top to bottom
            Self::LeftNasalTip => "LeftNasalTip".to_string(),
            Self::NasalTip => "NasalTip".to_string(), // from left - right
            Self::RightNasalTip => "RightNasalTip".to_string(),

            other => format!("{:?}", other),
        }
    }
}

pub struct OpenFaceHeaderMap {
    pub frame: usize,
    pub timestamp: usize,
    pub confidence: usize,
    pub pose_x: usize,
    pub pose_y: usize,
    pub pose_z: usize,
    pub x_start: usize,
    pub y_start: usize,
    pub z_start: Option<usize>,
}

impl OpenFaceHeaderMap {
    pub fn new(headers: &csv::StringRecord) -> Result<Self, MosaicError> {
        let find_col = |name: &str| {
            headers.iter()
                .position(|h| h.trim() == name)
                .ok_or_else(|| FileError::MissingColumn) 
        };

        let x_start = find_col("x_0")?;
        let y_start = find_col("y_0")?;
        let z_start = headers.iter().position(|h| h.trim() == "z_0");

        Ok(Self {
            frame: find_col("frame")?,
            timestamp: find_col("timestamp")?,
            confidence: find_col("confidence")?,
            pose_x: find_col("pose_Rx")?,
            pose_y: find_col("pose_Ry")?,
            pose_z: find_col("pose_Rz")?,
            x_start,
            y_start,
            z_start,
        })
    }
}

pub fn parse_openface_data(path: &Path) -> Result<UMDDriver, MosaicError> {
    if !path.is_file() {
        return Err(MosaicError::InvalidPath(path.display().to_string()));
    }

    let total_frames = count_csv_rows(path)?; 
    let points_per_frame = 20; // we should not hardcode this

    let mut umd = UMDDriver::construction(total_frames, points_per_frame);

    let file = File::open(path)?; 
    let mut reader = csv::ReaderBuilder::new()
        .trim(csv::Trim::All)
        .flexible(true)
        .has_headers(true)
        .from_path(path)
        .map_err(|e| FileError::MissingColumn)?;

    let headers = reader.headers().map_err(|e| FileError::MissingColumn)?.clone();
    let x0_index = headers.iter().position(|h| h == "x_0")
        .ok_or_else(|| FileError::MissingColumn)?;

    let header_map = OpenFaceHeaderMap::new(&headers)?;

    for result in reader.records() {

        let record: csv::StringRecord = result.map_err(|_| FileError::MalformedCSV)?;
    
        let frame_val: u32 = record[header_map.frame].parse().unwrap_or(0);
        let timestamp: f32 = record[header_map.timestamp].parse().unwrap_or(0.0);
        let confidence: f32 = record[header_map.confidence].parse().unwrap_or(0.0);
        let pose_x: f64 = record[header_map.pose_x].parse().unwrap_or(0.0);
        let pose_y: f64 = record[header_map.pose_y].parse().unwrap_or(0.0);
        let pose_z: f64 = record[header_map.pose_z].parse().unwrap_or(0.0);

        for i in 48..68 { // hardcoded only to get lip points but will change
            let landmark_enum = OpenFaceLandmarkType::openface_index(i);
            let label = landmark_enum.openface_index_label();

            let x: f64 = record[header_map.x_start + i].parse().unwrap_or(0.0);
            let y: f64 = record[header_map.y_start + i].parse().unwrap_or(0.0);
            let z: f64 = header_map.z_start
                .and_then(|start_idx| record.get(start_idx + i))
                .and_then(|v| v.parse().ok())
                .unwrap_or(0.0);

            // temp making pose bool = true
            let pose = true;

            /* if frame_val == 10 { // Just check frame 10
            println!("Frame: {} - Timestamp: {} - Confidence: {} - Pose: {} - Pose_X: {} - Pose_Y: {} - Pose_Z: {} - Point #: {} - Label: {} - X: {} - Y: {} - Z: {}", frame_val, timestamp, confidence, pose, pose_x, pose_y, pose_z, i, label, x, y, z);
            }*/
            
            umd.add_point(frame_val, timestamp, confidence, pose, pose_x, pose_y, pose_z, i as u32, label, x, y, z);
    
        }
    }

    println!("File read successfully."); // for testing
    Ok(umd)
}

/*
NOW THAT THE UMD HAS BEEN SETUP - WE NO LONGER NEED THE OPENFACE DATA

ALL MODULES ARE NOW RAN BY THE ANALYSIS SECTION
*/