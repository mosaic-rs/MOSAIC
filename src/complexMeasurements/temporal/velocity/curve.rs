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

pub struct curveVelocity {
    pub frame: Vec<u32>,
    pub timestamp: Vec<f32>,
    pub is_reliable: Vec<bool>, // for undertermined points (i.e. the 3 "inner" lip openface points) - as we are fitting to a cubic curve
    pub types_included: Vec<String>, 
    pub vx_coeffs: Vec<CurveCoefficients>,
    pub vy_coeffs: Vec<CurveCoefficients>,
    pub vz_coeffs: Vec<CurveCoefficients>,
    pub vx_coeffs_uncertainty: Vec<CurveCoefficients>,
    pub vy_coeffs_uncertainty: Vec<CurveCoefficients>,
    pub vz_coeffs_uncertainty: Vec<CurveCoefficients>,
}