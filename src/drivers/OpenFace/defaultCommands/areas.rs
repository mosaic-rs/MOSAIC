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

/*
Just default area command stuff (config i suppose) for the area command
*/

pub const outer_lip_area: &[&str] = &[
    "OuterRightCommissure,OuterRightUpperLip_com,OuterRightUpperLip_phil,OuterPhiltrum",
    "OuterPhiltrum,OuterLeftUpperLip_phil,OuterLeftUpperLip_com,OuterLeftCommissure",
    "OuterLeftCommissure,OuterLeftLowerLip_com,OuterLeftLowerLip_phil,OuterLowerVermillionBorder",
    "OuterLowerVermillionBorder,OuterRightLowerLip_phil,OuterRightLowerLip_com,OuterRightCommissure",
];

pub const outer_basis_landmarks: &[&str; 4] = &[
    "OuterLeftCommissure",
    "OuterRightCommissure",
    "OuterLowerVermillionBorder",
    "OuterPhiltrum",
];

pub const inner_lip_area: &[&str] = &[
    "InnerRightCommissure,InnerRightUpperLip,InnerPhiltrum",
    "InnerPhiltrum,InnerLeftUpperLip,InnerLeftCommissure",
    "InnerLeftCommissure,InnerLeftLowerLip,InnerLowerVermillionBorder",
    "InnerLowerVermillionBorder,InnerRightLowerLip,InnerRightCommissure"
];

pub const inner_basis_landmarks: &[&str; 4] = &[
    "InnerLeftCommissure",
    "InnerRightCommissure",
    "InnerLowerVermillionBorder",
    "InnerPhiltrum",
];