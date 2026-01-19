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

// Pose Correction
// Takes X/Y/Z pose values and uses them to correct landmark points.

// Assumes values are radians 
// Drivers will convert degrees to radians where applicable

// MOSAIC 0.2.1 is sort of made with OpenFace in mind but there will be changes to the UMD 
// make it general purpose by v1.0.0

