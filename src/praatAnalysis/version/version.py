"""
This file is part of MOSAIC.

MOSAIC is free software: you can redistribute it and/or modify it under 
the terms of the GNU General Public License as published by the Free 
Software Foundation, either version 3 of the License, or any later version.

MOSAIC is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; 
without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR 
PURPOSE. See the GNU General Public License for more details.

You should have received a copy of the GNU General Public License along with 
MOSAIC. If not, see <https://www.gnu.org/licenses/>.
"""


"""
    Gets version information for parselmouth and praat. 

    This is really more like a test command
"""

import parselmouth


def parselmouth_version():
    return parselmouth.VERSION

def praat_version():
    return parselmouth.PRAAT_VERSION

def praat_version_date():
    return parselmouth.PRAAT_VERSION_DATE
