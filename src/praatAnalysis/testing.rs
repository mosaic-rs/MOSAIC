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

use pyo3::prelude::*;
use pyo3::types::PyModule;
use std::ffi::{CStr, CString}; 

pub fn test_function(py: Python<'_>) -> PyResult<()> {
    let code_rust_str = include_str!("test.py");

    let code_c_string = CString::new(code_rust_str)
        .expect("Python script contained a null byte!");

    let file_name = c"test.py";
    let module_name = c"test";

    let module = PyModule::from_code(
        py, 
        &code_c_string, 
        file_name, 
        module_name
    )?;

    let func = module.getattr(c"beans")?;
    let result: String = func.call1(("Heinz",))?.extract()?;

    println!("Result from Python: {}", result);
    Ok(())
}