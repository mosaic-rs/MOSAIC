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

use std::env;
use std::path::PathBuf;

pub struct PythonEnvironment; 

impl PythonEnvironment {
    pub fn get_venv_path() -> PathBuf {
        env::current_exe()
            .expect("Failed to get current executable path")
            .parent()
            .unwrap()
            .join("python_lib") 
    }

    pub fn ensure_python_bridge() -> std::io::Result<()> {
        
        Ok(())
    }
    
    // mac
    #[cfg(target_os = "macos")]
    pub fn setup_python_paths() {
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

        let new_path = format!("{}:{}:{}", 
            python_lib.to_string_lossy(),
            stdlib.to_string_lossy(),
            site_packages.to_string_lossy()
        );

        unsafe {
            env::set_var("PYTHONHOME", &python_lib);
            env::set_var("PYTHONPATH", new_path);
        }
    }

    // windows
    #[cfg(target_os = "windows")]
    pub fn setup_python_paths() {
        let exe_path = env::current_exe().expect("Failed to get exe path");
        
        let resources = exe_path.parent().unwrap().to_path_buf();

        let python_lib = resources.join("python_lib");
        let stdlib = python_lib.join("stdlib");
        let site_packages = python_lib.join("site-packages");

        let new_path = format!("{};{};{}", 
            python_lib.to_string_lossy(),
            stdlib.to_string_lossy(),
            site_packages.to_string_lossy()
        );

        unsafe {
            env::set_var("PYTHONHOME", &python_lib);
            env::set_var("PYTHONPATH", new_path);
        }
    }
}
