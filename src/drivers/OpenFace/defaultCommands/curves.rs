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
    curve.rs
    Default curve groupings for the OpenFace driver. 
*/

// lip groups:

// outer lips
pub const right_upper_lip: &[&str] = &["OuterRightCommissure",
                             "OuterRightUpperLip",
                             "OuterRightUpperLip",
                             "OuterPhiltrum",
                            ];
pub const left_upper_lip: &[&str] = &["OuterPhiltrum",
                             "OuterLeftUpperLip",
                             "OuterLeftUpperLip",
                             "OuterLeftCommissure",
                            ];
pub const left_lower_lip: &[&str] = &["OuterLeftCommissure",
                                "OuterLeftLowerLip",
                                "OuterLeftLowerLip",
                                "OuterLowerVermillionBorder",
                                ];
pub const right_lower_lip: &[&str] = &[ "OuterLowerVermillionBorder",
                                "OuterRightLowerLip",
                                "OuterRightLowerLip",
                                "OuterRightCommissure",
                                ];

// inner lips
// Warning: inner lips are made up of three coordinates which is belwo the ideal number of coordinates for the curve fitting func (4)

pub const right_upper_inner_lip: &[&str] = &["InnerRightCommissure",
                                       "InnerRightUpperLip",
                                       "InnerPhiltrum",
                                      ];
pub const left_upper_inner_lip: &[&str] = &["InnerPhiltrum",
                                      "InnerLeftUpperLip",
                                      "InnerLeftCommissure",
                                     ];
pub const left_lower_inner_lip: &[&str] = &["InnerLeftCommissure",
                                      "InnerLeftLowerLip",
                                      "InnerLowerVermillionBorder",
                                     ];
pub const right_lower_inner_lip: &[&str] = &["InnerRightCommissure",
                                      "InnerLowerVermillionBorder",
                                      "InnerRightLowerLip",
                                     ];




