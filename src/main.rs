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
use shell::projectManager::session::SessionData;
use shell::shell::shell_initiation;

// venv
use praatAnalysis::setup::{PythonEnvironment};

use std::env;
use std::path::PathBuf;

/*fn setup_python_paths() {
    let exe_path = env::current_exe().expect("Failed to get exe path");
    
    let is_bundled = exe_path.to_string_lossy().contains(".app/Contents/MacOS");

    let resources = if is_bundled {
        exe_path.parent().unwrap().parent().unwrap().join("Resources")
    } else {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
    };

    let python_lib = resources.join("python_lib");
    let stdlib = python_lib.join("stdlib");
    let site_packages = python_lib.join("site-packages");

    unsafe {
        env::set_var("PYTHONHOME", &python_lib);
    }
    
    let new_path = format!("{}:{}:{}", 
        python_lib.to_string_lossy(),
        stdlib.to_string_lossy(),
        site_packages.to_string_lossy()
    );

    unsafe {
        env::set_var("PYTHONPATH", new_path);
    }
}*/

fn main() -> std::io::Result<()>{

    PythonEnvironment::setup_python_paths();

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