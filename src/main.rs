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

// CORE MEASUREMENTS
use MOSAIC::coreMeasurements::anchor::anchor::Anchor;

// SHELL
use MOSAIC::shell::statefulCLI::session_manager::{DirectoryVerifiers, SessionData};
use MOSAIC::shell::shell::{shell_initiation};

fn main(){

    /*
    println!("MOSAIC -- v0.2.0 pre-release (GLPv3)");
    Anchor();
    println!("");
    DirectoryVerifiers::check_project_directory("Hello World");

    let data = SessionData::read_session_data();

    println!("Session data below: \n\n{:#?}", data);

    */

    // running cli

    shell_initiation();

}