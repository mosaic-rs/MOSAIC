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
    This file is the session manager and checks where they are within a MOSAIC project.
*/

// test path for now

// const PATH: &str = "path/file.txt"; // this will be replaced by an arg passed through CLI

// Session Data Structure:

#[derive(Debug)]
pub struct SessionStructure{
    project_directory: String,
    participant_directory: String,
    trial_directory: String,
}

// Onwer of SessionStructure struct:

#[derive(Debug)]
pub struct SessionData{
    data: SessionStructure,
}


// Directory verifiers
// Check whether or now the user is in a project, participant, or trial directory, 
// and also checks whether or not they exist.

pub struct DirectoryVerifiers;
impl DirectoryVerifiers{

    pub fn check_project_directory(path: &str){
        println!("Checking if project directory '{}' exists.", path);
    }

    pub fn check_participant_directory(path: &str){
        println!("Checking if participant directory '{}' exists.", path);
    }

    pub fn check_trial_directory(path: &str){
        println!("Checking if trial directory '{}' exists.", path);
    }

    pub fn check_any_directory(path: &str){
        // this func checks the directory exists before updating
        println!("Checking if {} exists...", path);
    }

}

// Session read and write func
impl SessionData{
    // Session Reader
    // This reads the session data of mosaic

    pub fn read_session_data() -> Self{
        // in here we can read the session data
        println!("Reading and returning session data"); 

        // placeholder:

        let project_path: String = "desktop/MOSAIC/project2".to_string();
        let participant_path: String = "desktop/MOSAIC/project2/participants".to_string();
        let trial_path: String =  String::new();
        // pretend above is the data we have got from reading session json

        SessionData {
            data: SessionStructure {
                project_directory: project_path,
                participant_directory: participant_path,
                trial_directory: trial_path,
            }
        }




    }

    pub fn write_session_data(data: &mut SessionData){
        // we write the session data in here
        println!("Writing session data...\n\n{:#?}", data)
    }



}

pub struct SessionUpdate;
impl SessionUpdate{
    // Session Update System
    // Updates where the user is within a mosaic project

    pub fn update_project_directory(path: &str){
        // need to verify the project exists
        DirectoryVerifiers::check_any_directory(&path);

        // when that function succesffuly runs, we can update the session data
        // because they are changing project root, we can just reset all session data
        // and set project_directory to the one they entered.

        let mut updated_project_directory = SessionData {
            data: SessionStructure {
                project_directory: path.to_string(),
                participant_directory: String::new(),
                trial_directory: String::new(),
            }
        };

        // Session structure is written by calling the write function

        SessionData::write_session_data(&mut updated_project_directory);

        // Notifaction to terminal:

        println!("Opened project path: '{}'.", path)
    }
}