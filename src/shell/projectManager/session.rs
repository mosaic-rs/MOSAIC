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
        let contents = std::fs::read_to_string(SESSION_FILE).unwrap_or_else(|_| String::from("{}"));
        
        serde_json::from_str(&contents).unwrap_or_else(|_| Self::new_blank())
    }

    pub fn write_session_data(&self) {
        let json = serde_json::to_string_pretty(&self).unwrap();
        std::fs::write(SESSION_FILE, json).expect("[FATAL MOSAIC ERROR] Failed to save!");
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
                project_directory: String::from("None"),
                participant_directory: String::from("None"),
                trial_directory: String::from("None"),
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

        let mut session = SessionData::read_session_data();
        session.data.project_directory = path.to_string();
        session.write_session_data();

        println!("Opened project path: '{}'.", path)
    }

    pub fn update_participant_directory(_path: &str){
        println!("Updating participant directory!")
    }

    pub fn update_trial_directory(_path: &str){
        println!("Updating project directory!")
    }
}

#[tauri::command]
pub fn update_project_directory(path: String) {
    SessionUpdate::update_project_directory(&path);
}