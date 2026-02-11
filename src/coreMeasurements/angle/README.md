# Angle Logic
Calculates the angle between two points. Right now it only calculates the angle between a point and the origin but it will be poassible to calculate the angle between two distinct points later. 

## Math:

### Theta (θ)
$$\theta = \text{arctan2}(y, x)$$

### Phi (ɸ)
$$\phi = \text{arccos}\left(\frac{z}{\sqrt{x^2 + y^2 + z^2}}\right)$$

If Z is 0 (like when we use 2D openface coords), then Phi will just be 0