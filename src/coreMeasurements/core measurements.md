# CORE MEASUREMENT INFO FILE:

This is a short information file about the "core measurements" section of MOSAIC. 

P.S. Any variables marked with an astersic (*) are optional.

## Notes:
Currently, MOSAIC assumes values are pose corrected, which in this case, is true as you can not change it unless you edit the source code. This will change once the metadata is written to the UMD parquet which it is not.

### What are core measurements?
Core measurements are static calculations which tells you how the coordinates are moving in a singular "frame". For example, it can tell you the distance a point is from the origin, the curve of the lips or tongue, etc. This is not to be confused with the "complex measurements" section which calculates mostly temporal measurements. 

### Input Data Structure
Core Measurements ONLY read Mosaic UMD files, version 0.9.0 alpha and beyond. If I code MOSAIC smartly, there should always be backwards compatability for all 1.0.0 alpha UMD files (when metadata is written). 

### Output Data Structure
Each measurement (outlined below) is held within its own struct on the stack. Once all core measurements are calculated, they are then combined into a master struct which is written as a parquet to the designated path. Once the project, participant, and trial system is finalized, you should not have to specify a path as it stores data based on the session data stores in .mosaic in your applications folder. 

## Core Measurements: 
A list of all the calculations that take place. Please note that translating your data into the UMD does not guarantee that it can calculate the same static measurements as others. For example, a 16 sensor EMA with focus on the tongue and velum may not have enough data about the lips to calculate lip curvature, lip area, etc. Coordinate information (i.e. what that coordinate is representing) is stored in the UMD file. Same restrictions apply based on if the UMD is setup for 2D or 3D data.

### Euclidean Distances
Takes the X, Y, and Z* values from the origin and calculates the euclidean distance of that point from the origin. 

These formulas can also be used to calulate the distance of points from one another, not just the origin.

#### 2D/3D Euclidean Distance Formula 
Where Z is 0, it makes no difference to the output. 
$$
r = \sqrt(x_2-x_1)^2 + (y_2-y_1)^2+(z_2-z_1)^2
$$

### Angle Calculator
Calculates theta for 2D and theta + phi for 3D points. 

#### For 3D Coordinates: 

##### Theta:
$$
\theta = arccos(\frac{z}{r})
$$

##### Phi:
$$
\phi = atan2(y,x)
$$

#### For 2D Coordinates:
##### Theta:
$$
\theta = arcos(\frac{x}{r})
$$

# There will be more added soon