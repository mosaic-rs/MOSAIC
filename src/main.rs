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

// Note to self:
// RUSTFLAGS="-A warnings" cargo run     --  to run cargo without all the warnings
use std::path::Path;



// SHELL
use MOSAIC::shell::projectManager::session::SessionData;
use MOSAIC::shell::shell::shell_initiation;

// venv
use MOSAIC::praatAnalysis::setup::{PythonEnvironment};

fn main() -> std::io::Result<()>{

    if let Err(e) = PythonEnvironment::ensure_python_bridge() {
        eprintln!("[MOSAIC FATAL ERROR] Could not initialize Python environment.");
        eprintln!("Reason: {}", e);
        eprintln!("Please ensure Python 3 is installed on your system.\nVisit https://www.python.org/ to download python3");
        std::process::exit(1);
    }
    

    let mut session = SessionData::read_session_data();

    shell_initiation(&mut session);

    Ok(())
}