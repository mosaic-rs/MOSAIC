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

use std::fmt;

// MOSAIC LEVEL ERRORS
#[derive(Debug)]
pub enum MosaicError {
    InvalidPath(String),
    Project(ProjectError),
    Participant(ParticipantError),
    Trial(TrialError),
    UMD(UMDError),
    Io(std::io::Error),
    File(FileError),
}

// PORJECT LEVEL ERRORS
#[derive(Debug)]
pub enum ProjectError{
    MissingMetaData,
    NoOpenProject,
    RequireProject,
}

// PARTICIPANT LEVEL ERRORS
#[derive(Debug)]
pub enum ParticipantError{
    MissingMetaData,
    NoOpenParticipant,
    RequireParticipant,
    InvalidParentUUIDError,
}

// TRIAL LEVEL ERRORS
#[derive(Debug)]
pub enum TrialError{
    MissingMetaData,
    NoOpenTrial,
    RequireTrial,
    InvalidParentUUIDError,
    InvalideGrandparentUUIDError,
}

// FILE ERRORS
#[derive(Debug)]
pub enum FileError{
    // CSV ERRORS
    MalformedCSV,
    MissingColumn,
}

// UMD ERRORS
#[derive(Debug)]
pub enum UMDError{
    MissingCommisures, // row based error
    MissingLeftCommisure, // row based error 
    MissingRightCommisure, // row based error
    MissingPhiltrum, // row based error
    MissingLowerVermillionBorder, // row based error
    MissingValue, // for when there is a missing value in like a csv file, json, etc
    MissingRow, // most common for csv files like OpenFace
    MissingColumn, // same as above - useful for csv files like OpenFace 
    InvalidValueType, // When a value is not the correct one - like an x value should never be a bool
    
    // OpenFace/Video based tracking errors:
    MissingFrameValue, // when that row is missing frame value
    MissingPoseX, // Only enabled if the user selected pose correction when running command
    MissingPoseY, // Only enabled if the user selected pose correction when running command
    MissingPoseZ, // Only enabled if the user selected pose correction when running command
    MissingConfidence, // we need OpenFace confidence levels when using fram filtering to excluce bad frames

}

impl From<FileError> for MosaicError {
    fn from(error: FileError) -> Self {
        MosaicError::File(error)
    }
}

impl fmt::Display for MosaicError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        match self{
            // MOSAIC/HIGHER LEVEL ERRORS
            MosaicError::InvalidPath(p) => write!(f, "Path provided '{}' is invalid. ", p),
            MosaicError::Io(e) => write!(f, "System Error: {}", e),

            // FILE ERRORS
            // CSV
            MosaicError::File(FileError::MalformedCSV) =>
                write!(f, "Malformed CSV. Please check CSV file to ensure it is properly formatted."),
            
            MosaicError::File(FileError::MissingColumn) =>
                write!(f, "CSV File missing column. Please verify your csv has correct formatting for the selected driver."),

            // PROJECT ERRORS
            MosaicError::Project(ProjectError::MissingMetaData) => 
                write!(f, "Missing project metadata.\n\n
                Either this is not a valid project path or the 'project.mosaicproj' file has been deleted."),

            MosaicError::Project(ProjectError::NoOpenProject) => 
                write!(f, "You are not in a project directory."),

            MosaicError::Project(ProjectError::RequireProject) => 
                write!(f, "You must be in a project directory to use that command.\n\n
                Use <open project 'project path'>."),

            // PARTICIPANT ERRORS
            MosaicError::Participant(ParticipantError::MissingMetaData) => 
                write!(f, "Missing participant metadata.\n\n
                Either this is not a valid participant path\n
                or the ',<participant_id>.mosaicproj' file has been deleted."),
            
            MosaicError::Participant(ParticipantError::NoOpenParticipant) =>
                write!(f, "You are not in a participant directory."),

            MosaicError::Participant(ParticipantError::RequireParticipant) => 
                write!(f, "You must be in a participant directory to use that command.\n\n    
                Use <open participant 'participant path'>."),

            MosaicError::Participant(ParticipantError::InvalidParentUUIDError) =>
                write!(f, "Current participant does not correspond to this project.\nThis is due
                to the UUID in '<participant_id>.mosaicproj' not matching with the UUID in 'project.mosaicproj'.\n\n
                Make sure to create participant directories through the <create> command - not through file manager."),

            // TRIAL ERRORS
            MosaicError::Trial(TrialError::MissingMetaData) => 
                write!(f, "Missing trial metadata.\n\n 
                Either this is not a valid trial path\nor the 'trial.mosaicproj' file has been deleted."),

            MosaicError::Trial(TrialError::NoOpenTrial) =>
                write!(f, "You are not in a trial directory."),
            
            MosaicError::Trial(TrialError::RequireTrial) =>
                write!(f, "You must be in a trial directory to use that command.\n\n    
                Use <open trial 'trial path'>."),

            MosaicError::Trial(TrialError::InvalidParentUUIDError) =>
                write!(f, "Current trial does not correspond to this participant.\n
                This is due to the UUID in '<trial_id>.mosaicproj' not matching with the UUID in 'participant.mosaicproj'.\n\n
                Make sure to create trial directories through the <create> command - not through file manager."),

            MosaicError::Trial(TrialError::InvalideGrandparentUUIDError) =>
                write!(f, "Current trial does not correspond to this project.\n
                This is due to the UUID in '<trial_id>.mosaicproj' not matching with the UUID in 'project.mosaicproj'.\n\n
                Make sure to create trial directories through the <create> command - not through file manager."),
            
                

            // LAST CASE ERRORS
            _ => write!(f, "{:?}", self)
            
        }
    }
}

impl std::error::Error for MosaicError {}

impl From<std::io::Error> for MosaicError {
    fn from(error: std::io::Error) -> Self {
        MosaicError::Io(error)
    }
}