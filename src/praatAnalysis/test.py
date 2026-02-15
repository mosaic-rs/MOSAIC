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
Because RustMouth is too unstable to be used in MOSAIC (for now), ParselMouth will be use :) 
"""

import sys

print("PYTHON TEST CONNECTION FILE\n\nHello World!")
if len(sys.argv) > 1:
    print(f"Testing Argument parsing:\n\n    Argument: {sys.argv[1]}")