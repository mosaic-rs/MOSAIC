# Velocity

This section of complexMeasurements calculates velocity of measurements. 

## Sections

### Landmarks: 

Calculates the velocity of landmarks and the uncertainty of the velocity. 
$$

r_i =
\begin{bmatrix}
x_i \\
y_i \\
z_i
\end{bmatrix}
$$

$$
v_{i} = \frac{r_{i} - r_{i - 1}}{t_{i} - t_{i-1}}
$$

Where,
$$
v_{x, i} = \frac{x_i - x_{i-1}}{t_i - t_{i-1}}
\hspace{10pt}
v_{y, i} = \frac{y_i - x_{i-1}}{t_i - t_{i-1}}
\hspace{10pt}
v_{z, i} = \frac{z_i - x_{i-1}}{t_i - t_{i-1}}
$$

We can also get the radius velocity

$$
v_{r, i} = \sqrt{v_{x, i}^2 + v_{y, i}^2 + v_{z, i}^2}
$$

#### Landmark Velocity Undertainty

##### Partial derivative with respect to x_i, y_i, and z_i
$$
\frac{\partial{v_x}}{\partial{x_{i}}} = \frac{1}{\Delta{t}}
\hspace{10pt}
\frac{\partial{v_y}}{\partial{y_{i}}} = \frac{1}{\Delta{t}}
\hspace{10pt}
\frac{\partial{v_z}}{\partial{z_{i}}} = \frac{1}{\Delta{t}}
$$

##### Partial Derivative with respect to x_{i - 1}, y_{i - 1}, z_{i - 1}

$$
\frac{\partial{v_x}}{\partial{x_{i-1}}} = - \frac{1}{\Delta{t}}
\hspace{10pt}
\frac{\partial{v_y}}{\partial{y_{i-1}}} = - \frac{1}{\Delta{t}}
\hspace{10pt}
\frac{\partial{v_z}}{\partial{z_{i-1}}} = - \frac{1}{\Delta{t}}
$$
##### Uncertainty Propogation
$$
\sigma{_{v_{x}}} = \sqrt{(\frac{1}{\Delta{t}}\partial_{x_{i}})^2 + (-\frac{1}{\Delta{t}}\sigma_{x_{i-1}})^2}
$$
$$
\sigma{_{v_{y}}} = \sqrt{(\frac{1}{\Delta{t}}\partial_{y_{i}})^2 + (-\frac{1}{\Delta{t}}\sigma_{y_{i-1}})^2}
$$
$$
\sigma{_{v_{z}}} = \sqrt{(\frac{1}{\Delta{t}}\partial_{z_{i}})^2 + (-\frac{1}{\Delta{t}}\sigma_{z_{i-1}})^2}
$$

### Curve:

$$
v_{a_{x, i}} = \frac{a_{x, i} - a_{x, i-1}}{t_i - t_{i - 1}}
\hspace{10pt}
v_{b_{x, i}} = \frac{b_{x, i} - b_{x, i-1}}{t_i - t_{i - 1}}
\hspace{10pt}
v_{c_{x, i}} = \frac{c_{x, i} - c_{x, i-1}}{t_i - t_{i - 1}}
\hspace{10pt}
v_{d_{x, i}} = \frac{d_{x, i} - d_{x, i-1}}{t_i - t_{i - 1}}
$$
$$
\textbf{v}_{x, i}(t) = v_{a_{x, i}}^3(t) + v_{b_{x, i}}^2(t) + v_{c_{x, i}}(t) + v_{d_{x, i}}(t)

$$
And so on for y and z...

#### Velocity Across the Curve
$$
\textbf{v}_{total, i}(t) = \sqrt{\textbf{v}_{x, i}(t)^2 + \textbf{v}_{y, i}(t)^2 + \textbf{v}_{z, i}(t)^2}
$$
Allows us to see where the velocity is highest/lowest across the curve (t). (i.e. We can see if during a bilabial polsive, if velocity is highest at a certain portion of the lips)

#### Velocity Per Coefficient

Cubic Velocity: Speed of the "S-curve" or complex curvature like puckering
$$
v_{a, i} = \sqrt{v_{a_{x, i}}^2 + v_{a_{y, i}}^2 + v_{a_{z, i}}^2}
$$
Quadratic Velocity: Speed of lip rounding
$$
v_{b, i} = \sqrt{v_{b_{x, i}}^2 + v_{b_{y, i}}^2 + v_{b_{z, i}}^2}
$$
Linear Velocity: Speed of the start and end of the curve
$$
v_{c, i} = \sqrt{v_{c_{x, i}}^2 + v_{c_{y, i}}^2 + v_{c_{z, i}}^2}
$$
Verticle Velocity: Speed of the verticle movement
$$
v_{d, i} = \sqrt{v_{d_{x, i}}^2 + v_{d_{y, i}}^2 + v_{d_{z, i}}^2}
$$

### Curve Velocity Uncertainty
#### Partial Derivitives with respect to $a_{x, i}$, $b_{x, i}$, $c_{x, i}$, $d_{x, i}$ (and so on for y and z)

$$\frac{\partial{v_{a_x}}}{\partial{a_{x, i}}} = \frac{1}{\Delta{t}}
\hspace{10pt}
\frac{\partial{v_{b_x}}}{\partial{b_{x, i}}} = \frac{1}{\Delta{t}}
\hspace{10pt}
\frac{\partial{v_{c_x}}}{\partial{c_{x, i}}} = \frac{1}{\Delta{t}}
\hspace{10pt}
\frac{\partial{v_{d_x}}}{\partial{d_{x, i}}} = \frac{1}{\Delta{t}}
$$

#### Partial Derivatives with respect to $a_{x, i-1}$, $b_{x, i-1}$, $c_{x, i-1}$, $d_{x, i-1}$(And so on for y and z)

$$\frac{\partial{v_{a_x}}}{\partial{a_{x, i-1}}} = - \frac{1}{\Delta{t}}
\hspace{10pt}
\frac{\partial{v_{b_x}}}{\partial{b_{x, i-1}}} = - \frac{1}{\Delta{t}}
\hspace{10pt}
\frac{\partial{v_{c_x}}}{\partial{c_{x, i-1}}} = - \frac{1}{\Delta{t}}
\hspace{10pt}
\frac{\partial{v_{d_x}}}{\partial{d_{x, i-1}}} = - \frac{1}{\Delta{t}}$$

#### Uncertainty Propagation(And so on for y and z)

$$\sigma_{v_{a_x}} = \sqrt{\left(\frac{1}{\Delta{t}}\sigma_{a_{x, i}}\right)^2 + \left(-\frac{1}{\Delta{t}}\sigma_{a_{x, i-1}}\right)^2}$$

$$\sigma_{v_{b_x}} = \sqrt{\left(\frac{1}{\Delta{t}}\sigma_{b_{x, i}}\right)^2 + \left(-\frac{1}{\Delta{t}}\sigma_{b_{x, i-1}}\right)^2}$$

$$\sigma_{v_{c_x}} = \sqrt{\left(\frac{1}{\Delta{t}}\sigma_{c_{x, i}}\right)^2 + \left(-\frac{1}{\Delta{t}}\sigma_{c_{x, i-1}}\right)^2}$$

$$\sigma_{v_{d_x}} = \sqrt{\left(\frac{1}{\Delta{t}}\sigma_{d_{x, i}}\right)^2 + \left(-\frac{1}{\Delta{t}}\sigma_{d_{x, i-1}}\right)^2}$$

### Area:

Calculates the velocity of the area and its uncertainty. This applies to sub-areas too like looking at the upper right area of the mouth using OpenFace

$$v_{a, i} = \frac{a_i - a_{i-1}}{t_i - t_{i-1}}$$

### Area Velocity Uncertainty

#### Partial derivative with respect to $a_i$

$$\frac{\partial{v_a}}{\partial{a_i}} = \frac{1}{\Delta{t}}$$

#### Partial Derivative with respect to $a_{i-1}$
$$\frac{\partial{v_a}}{\partial{a_{i-1}}} = - \frac{1}{\Delta{t}}$$

#### Uncertainty Propagation

$$\sigma_{v_a} = \sqrt{\left(\frac{1}{\Delta{t}}\sigma_{a_i}\right)^2 + \left(-\frac{1}{\Delta{t}}\sigma_{a_{i-1}}\right)^2}$$