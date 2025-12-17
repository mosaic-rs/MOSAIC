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
use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor, Result};

use crate::shell::projectManager::session::{SessionData, DirectoryVerifiers, SystemVerifier};

pub fn shell_initiation(session: &mut SessionData) -> Result<()> {
    println!("MOSAIC -- v0.2.0 pre-release (GLPv3)\n"); // opening message
    let mut rl = DefaultEditor::new()?;

    loop {
        let readline = rl.readline("MOSAIC >> ");
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

                if line.trim() == "project"{
                    SystemVerifier::project();                    
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
                eprintln!("Error: {:?}", err);
                break
            }
        }

        
    }
    Ok(())
}

