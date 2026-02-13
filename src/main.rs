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


// CORE MEASUREMENTS
//use MOSAIC::coreMeasurements::anchor::anchor::{AnchorCoordinate};

// SHELL
use MOSAIC::shell::projectManager::session::{DirectoryVerifiers, SessionData};
use MOSAIC::shell::shell::{shell_initiation};
use MOSAIC::drivers::OpenFace::openface::{parse_openface_data};

// core measurement crates
use MOSAIC::coreMeasurements::angle::{angle};
use MOSAIC::coreMeasurements::area::{area};
use MOSAIC::coreMeasurements::curve::{curve};
use MOSAIC::coreMeasurements::euclidean::{euclidean};

fn main(){

    /*
    println!("MOSAIC -- v0.2.1 pre-release (GLPv3)");
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

    //let test_coords = [[12.5, 45.0, 0.1], [34.2, 88.9, 1.5], [102.3, 14.7, -5.2], [110.0, 20.1, -4.8], [255.0, 512.0, 10.0], [260.5, 515.2, 11.2], [7.0, 3.1, 0.0], [9.4, 2.8, 0.2]];
    //let time: f32 = 3.4;
    //let anchor = AnchorCoordinate::anchor(time, &test_coords);
    //println!("Anchor: {:?}", anchor);
    

    let mut session = SessionData::read_session_data();

    shell_initiation(&mut session);

}