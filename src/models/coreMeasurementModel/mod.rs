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
Models is what creates the Universal Measurement Data (UMD) model which is the standard data
structure used by MOSAIC. The goal is to make this software as adapbtable as possible. For example,
if one is processing data with 60 points on the lips or 10 points on the lips, they can calculate
lip curvature regardless of the amount of points (of course, a minimum amount of points is reauired).

./Models contains a few subdirectories and each one has a specific role outlined below:

    /anchor - name scheme follows "anchor" + x/y/z (where applicable).
              Uncertainty follows the same sche,e + "unc".
        i.e. anchorx, anchory, anchorz
        i.e. anchorxunc, anchoryunc, anchoryunc

    /pose - name scheme is "pose" + Rx/Ry/Rz.                       - Not all software provides pose
            Uncertainty follows the same scheme + "unc".              and it is very driver dependent.
        i.e. poseRx, poseRy, poseRz                                   Uncertainty is calculated in
        i.e. poseRxunc, poseRyunc, poseRzunc                          calibration phase.

    /coordinates - coordinates are location + X, Y, and Z (if applicable) and are numbered from 1 - n.    - We need to specifiy locations
                   Uncertainty follows the same numbers followed by "unc".                                  so we know where these points are
        i.e. URX1, URY1, URZ1, URX2, URY2, URZ3,... LLXn, LLYn, LLZn                                        when treating the mouth as having
        i.e. URX1unc, URY1unc, URX1unc,... LLXnunc, LLYnunx, LLZnunc                                        4 quadrants (UR, UL, LR, LL). You
                                                                                                            can also have "inner" locations like
                   Euclidan points follow the scheme of location + d + n.                                   IUR, IUL, ILR, ILL.
                   Uncertainty follows the same numbers followed by "unc".                              
        i.e. URd1, URd2, URd3,... LLdn,                                                                     Coords are also centred to the origin
        i.e. URd1unc, URd2unc, URd3unc,... LLdnunc                                                          which occurs during the anchor calcu-
                                                                                                            lation.
    /angle - Units θ (theta) and ɸ (phi) (where applicable) and are numbered from 1 - n.                    
             Uncertainty follows the same numbers follwed by "unc".
        i.e. theta1, phi1, theta2, phi2,... thetan, phin
        i.e. theta1unc, phi1unc,... thetanunc, phinunc
    
    /area - MOSAIC calculates 2 forms of mouth area and are organised as such below:
        /bioArea - name scheme follows location + area type and a "TOTAL" + type for total mouth area.
                   Uncertainty follows the same scheme followed by "unc".
            i.e. URbio, ULbio, LRbio, LLbio, TOTALbio
            i.e. URbiounc, ULbiounc, LRbiounc, TOTALbiounc
        
        /quadrantArea - name scheme follows location + area type and a "TOTAL" + type for total mouth area.
                        Uncertainty follows the same scheme followed by "unc".
            i.e. URquad, ULquad, LRquad, LLquad, TOTALquad
            i.e. URquadunc, ULquadunc, LLquadunc, TOTALquadunc

    /curve - name scheme follows location + representative letter + "_" + X/Y/Z.                - The most notable change in the Rust
             Uncertainty follows the same scheme followed by "unce".                              is that curves can have an unlimited
        i.e. URA_x, URA_y, URA_z,... URAC_x, URAC_y, URAC_z - represents a curve with 29 coeffs   ammount of coeffs (up until ZZZ which)
        i.e. URA_xunc, URA_yunc, URA_zunc,... URAC_xunc, URAC_yunc, URAC_zunc                     no one will really get to.

             We also store the points the curve passes through at whatever we make t.
             Naming scheme follows location + t + i/n + X/Y/Z.
             Uncertainty follows the same scheme + "unc"
        i.e. URt1/10x, URt1/10y, URt1/10z,... URt10/10x, URt10/10y, URt10/10z
        i.e. URt1/10xunc, URt1/10yunc, URt1/10zunc,... URt10/10xunc, URt10/10yunc, URt10/10zunc
            
I think a hasmap is best for this type of variable dataset
*/