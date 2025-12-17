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
    InvalidPath,
    Project(ProjectError),
    Io(std::io::Error),
}

// PORJECT LEVEL ERRORS

#[derive(Debug)]
pub enum ProjectError{
    MissingMetaData,
    NoOpenProject,
    RequireProject,
}

impl fmt::Display for MosaicError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        match self{
            // MOSAIC/HIGHER LEVEL ERRORS
            MosaicError::Io(e) => write!(f, "System Error: {}", e),

            // PROJECT ERRORS
            MosaicError::Project(ProjectError::MissingMetaData) => write!(f, "Missing project metadata.\n\n 
                Either this is not a valid project path\n or the 'metadata.mosaicproj' file has been deleted."),

            MosaicError::Project(ProjectError::NoOpenProject) => write!(f, "You are not in a project directory."),

            MosaicError::Project(ProjectError::RequireProject) => write!(f, "You must be in a project directory to use that command.\n\n.   
            Use <open project 'project path'>."),

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