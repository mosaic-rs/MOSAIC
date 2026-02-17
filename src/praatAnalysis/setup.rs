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

use std::process::Command;
use std::env;
use std::path::PathBuf;

pub struct PythonEnvironment; 

impl PythonEnvironment {
    pub fn get_venv_path() -> PathBuf {
        env::current_exe()
            .expect("Failed to get current executable path")
            .parent()
            .unwrap()
            .join(".mosaic_venv")
    }

    pub fn ensure_python_bridge() -> std::io::Result<()> {
        let venv_dir = Self::get_venv_path();

        if !venv_dir.exists() {
            println!("Initializing MOSAIC Python venv");
            
            // Create venv
            Command::new("python3")
                .args(&["-m", "venv", venv_dir.to_str().unwrap()])
                .status()?;

            // Install Parselmouth
            let pip_bin = if cfg!(windows) { "Scripts/pip.exe" } else { "bin/pip" };
            let pip_path = venv_dir.join(pip_bin);

            Command::new(pip_path)
                .args(&["install", "praat-parselmouth"])
                .status()?;
            
            println!("venv Created Successfully");
        }
        Ok(())
    }
}