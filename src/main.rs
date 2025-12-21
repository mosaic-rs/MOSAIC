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
use std::path::Path;


// CORE MEASUREMENTS
use MOSAIC::coreMeasurements::anchor::anchor::Anchor;

// SHELL
use MOSAIC::shell::projectManager::session::{DirectoryVerifiers, SessionData};
use MOSAIC::shell::shell::{shell_initiation};
use MOSAIC::drivers::OpenFace::openface::{parse_openface_data};

fn main(){

    /*
    println!("MOSAIC -- v0.2.0 pre-release (GLPv3)");
    Anchor();
    println!("");
    DirectoryVerifiers::check_project_directory("Hello World");

    let data = SessionData::read_session_data();

    println!("Session data below: \n\n{:#?}", data);

    */

    // testing func
    // const PATH_TEMP: &str = "/Users/harrywoodhouse/Desktop/MOSAIC/MOSAIC-Engine/test_data/v15044gf0000d1dlc67og65r2deqmhd0.csv";
    // parse_openface_data(Path::new(PATH_TEMP)).expect("Failed to parse data");


    // running cli
    

    let mut session = SessionData::read_session_data();

    shell_initiation(&mut session);

}