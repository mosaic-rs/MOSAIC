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

use serde::{Deserialize, Serialize};
use crate::errors::{MosaicError, ProjectError, ParticipantError, TrialError};

const SESSION_FILE: &str = ".mosaic";

#[derive(Debug, Serialize, Deserialize)] 
pub struct SessionStructure {
    pub project_directory: String,
    pub participant_directory: String,
    pub trial_directory: String,
}

#[derive(Debug, Serialize, Deserialize)] 
pub struct SessionData {
    pub data: SessionStructure,
}

pub struct DirectoryVerifiers;
impl DirectoryVerifiers {
    pub fn check_any_directory(path: &str) {
        println!("Checking if {} exists...", path);
    }
}

impl SessionData {
    pub fn read_session_data() -> Self {
        let contents = std::fs::read_to_string(".mosaic").unwrap_or_else(|_| String::from("{}"));
        
        serde_json::from_str(&contents).unwrap_or_else(|_| Self::new_blank())
    }

    pub fn write_session_data(&self) {
        let json = serde_json::to_string_pretty(&self).unwrap();
        std::fs::write(".mosaic", json).expect("[FATAL MOSAIC ERROR] Failed to save!");
    }

    pub fn initialize() -> Self {
        let path = SESSION_FILE;

        if std::path::Path::new(path).exists() {
            Self::read_session_data()
        }
        else {
            Self::create_file(path)
        }
    }

    pub fn create_file(_path: &str) -> Self {
        let new_session = Self::new_blank();
        new_session.write_session_data();
        new_session
    }

    pub fn new_blank() -> Self {

        SessionData {
            data: SessionStructure {
                project_directory: String::from("None"), //Path/supermegapath/project.mosaicproj
                participant_directory: String::from("None"), //Path/supermegapath/participant.mosaicproj
                trial_directory: String::from("None"), //Path/supermegapath/trial.mosaicproj
            }
        }
    }

    pub fn reset_session(&mut self) {
        self.data.project_directory = String::from("None");
        self.data.participant_directory = String::from("None");
        self.data.trial_directory = String::from("None");
        self.write_session_data(); 
    }
}

pub struct SystemVerifier;
impl SystemVerifier {
    pub fn project() -> Result<String, MosaicError>{
        let session_info = SessionData::read_session_data();
        let project_path = session_info.data.project_directory;
        
        // this logic is a bit weird but this is only a test bit of code 
        if project_path != "None" {
            return std::result::Result::Ok(project_path)

        }else if project_path == "None" {
            return Err(MosaicError::Project(ProjectError::RequireProject))

        }else {
            return Err(MosaicError::Project(ProjectError::RequireProject))
        }
        Ok(String::from(project_path))
    }

    pub fn participant() -> Result<String, MosaicError>{
        let session_info = SessionData::read_session_data();
        let participant_path = session_info.data.participant_directory;
        
        // this logic is a bit weird but this is only a test bit of code 
        if participant_path != "None" {
            return std::result::Result::Ok(participant_path)
            
        }else if participant_path == "None" {
            return Err(MosaicError::Participant(ParticipantError::RequireParticipant))

        }else {
            return Err(MosaicError::Participant(ParticipantError::RequireParticipant))
        }
        Ok(String::from(participant_path))
    }

    pub fn trial() -> Result<String, MosaicError>{
        let session_info = SessionData::read_session_data();
        let trial_path = session_info.data.trial_directory;
        
        // this logic is a bit weird but this is only a test bit of code
        if trial_path != "None" {
            return std::result::Result::Ok(trial_path)
            
        }else if trial_path == "None" {
            return Err(MosaicError::Trial(TrialError::RequireTrial))

        }else {
            return Err(MosaicError::Trial(TrialError::RequireTrial))
        }
        Ok(String::from(trial_path))
    }
}

pub struct SessionUpdate;
impl SessionUpdate {
    pub fn update_project_directory(path: &str){
        DirectoryVerifiers::check_any_directory(path);

        let updated_project_directory = SessionData {
            data: SessionStructure {
                project_directory: path.to_string(),
                participant_directory: String::from("None"),
                trial_directory: String::from("None"),
            }
        };

        updated_project_directory.write_session_data();

        println!("Opened project path: '{}'.", path)
    }

    pub fn update_participant_directory(path: &str){
        // we need to make sure they are actually in a project dir
        println!("Updating participant directory!")
    }

    pub fn update_trial_directory(path: &str){
        // need to confirm they are in project and participant dir
        println!("Updating project directory!")
    }
}

