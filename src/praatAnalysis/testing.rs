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

/// Just a test file to call a python script

pub struct tests;
use std::process::Command;


impl tests{
    pub fn test_main() {
        let script_path = "src/praatAnalysis/test.py";
        let data_path = "test_data.parquet"; // fake data to see how it handles data transmission
        
        let status = Command::new("python3")
            .arg(script_path) // the script
            .arg(data_path)   // the data
            .status();
    }
}