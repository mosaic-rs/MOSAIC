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
core shell file which parses commands and allocates them to seperate files which exeute the command

shell.rs sort of acts like a receptionist
*/

use std::process::Command;
use std::path::Path;
use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor, Result};

use crate::shell::projectManager::session::{SessionData, DirectoryVerifiers, SystemVerifier};
use crate::analysis::{run};

pub fn shell_initiation(session: &mut SessionData) -> Result<()> {
    println!("MOSAIC -- v0.2.0 pre-release (GLPv3)\n"); // opening message
    let mut rl = DefaultEditor::new()?;

    loop {
        let readline = rl.readline("MOSAIC >> ");
        println!(""); // adding space before shell output
        match readline {
            Ok(line) => {
                if line.trim() == "quit"{
                    break;
                }
                if line.trim() == "exit"{
                    break;
                }

                if line.trim() == "session"{
                    let data = SessionData::read_session_data();

                    println!("Session Data:\n{:#?}", data);
                    
                }

                // FOLLOWING FOUR COMMANDS ARE JUST FOR TESING THE SYSTEM VERIFIER FUNCS IN "session.rs"
                if line.trim() == "project"{
                    let project_path = SystemVerifier::project();
                    match project_path {
                        Ok(_) => println!("Project Path: {:?}", project_path.unwrap()),

                        Err(err) => {
                            eprintln!("[MOSAIC ERROR] {}", err)
                        }

                    }               
                }

                if line.trim() == "participant"{
                    let participant_path = SystemVerifier::participant();
                    match participant_path {
                        Ok(_) => println!("Participant Path: {:?}", participant_path.unwrap()),

                        Err(err) => {
                            eprintln!("[MOSAIC ERROR] {}", err)
                        }

                    }               
                }

                if line.trim() == "trial"{
                    let trial_path = SystemVerifier::trial();
                    match trial_path {
                        Ok(_) => println!("Path: {:?}", trial_path.unwrap()),

                        Err(err) => {
                            eprintln!("[MOSAIC ERROR] {}", err)
                        }

                    }               
                }

                if line.trim() == "UMD"{
                    let input_path = "/Users/harrywoodhouse/MOSAIC/MOSAIC/MOSAIC-Engine/test_data/v15044gf0000d1dlc67og65r2deqmhd0.csv";
                    let output_path = "/Users/harrywoodhouse/MOSAIC/MOSAIC/MOSAIC-Engine/data/";

                    // defining placeholder metadata: 
                    let driver = "OpenFace v2.2";
                    let dimension = "2D";
                    let pose_correction = true;

                    run::init(input_path, output_path);
                }
            }
            Err(ReadlineError::Interrupted) => { // Handles Ctrl-C
                println!("CTRL-C");
                break
            },
            Err(ReadlineError::Eof) => { // Handles Ctrl-D
                println!("CTRL-D");
                break
            },
            Err(err) => {
                eprintln!("Something went wrong: {:?}", err); // Note when coming back: Find out why this error is not printing
            }
        }
        println!(""); // adding space after shell output

        
    }
    Ok(())
}

