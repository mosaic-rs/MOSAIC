# Acceleration
This section of complexMeasurements calculates acceleration of measurements.

## Landmarks:
Calculates the acceleration of landmarks and the uncertainty of the acceleration.
$$v_i =
\begin{bmatrix}
v_{x, i} \\
v_{y, i} \\
v_{z, i}
\end{bmatrix}$$
$$a_{i} = \frac{v_{i} - v_{i - 1}}{t_{i} - t_{i-1}}$$
Where,
$$a_{x, i} = \frac{v_{x, i} - v_{x, i-1}}{t_i - t_{i-1}}
\hspace{10pt}
a_{y, i} = \frac{v_{y, i} - v_{y, i-1}}{t_i - t_{i-1}}
\hspace{10pt}
a_{z, i} = \frac{v_{z, i} - v_{z, i-1}}{t_i - t_{i-1}}$$
We can also get the radius acceleration
$$a_{r, i} = \sqrt{a_{x, i}^2 + a_{y, i}^2 + a_{z, i}^2}$$

## Landmark Acceleration Uncertainty
### Partial derivative with respect to $v_{x, i}$, $v_{y, i}$, and $v_{z, i}$

$$\frac{\partial{a_x}}{\partial{v_{x, i}}} = \frac{1}{\Delta{t}}
\hspace{10pt}
\frac{\partial{a_y}}{\partial{v_{y, i}}} = \frac{1}{\Delta{t}}
\hspace{10pt}
\frac{\partial{a_z}}{\partial{v_{z, i}}} = \frac{1}{\Delta{t}}$$

### Partial Derivative with respect to $v_{x, i-1}$, $v_{y, i-1}$, $v_{z, i-1}$
$$\frac{\partial{a_x}}{\partial{v_{x, i-1}}} = - \frac{1}{\Delta{t}}
\hspace{10pt}
\frac{\partial{a_y}}{\partial{v_{y, i-1}}} = - \frac{1}{\Delta{t}}
\hspace{10pt}
\frac{\partial{a_z}}{\partial{v_{z, i-1}}} = - \frac{1}{\Delta{t}}$$

### Uncertainty Propagation

$$\sigma_{a_{x}} = \sqrt{\left(\frac{1}{\Delta{t}}\sigma_{v_{x, i}}\right)^2 + \left(-\frac{1}{\Delta{t}}\sigma_{v_{x, i-1}}\right)^2}$$

$$\sigma_{a_{y}} = \sqrt{\left(\frac{1}{\Delta{t}}\sigma_{v_{y, i}}\right)^2 + \left(-\frac{1}{\Delta{t}}\sigma_{v_{y, i-1}}\right)^2}$$

$$\sigma_{a_{z}} = \sqrt{\left(\frac{1}{\Delta{t}}\sigma_{v_{z, i}}\right)^2 + \left(-\frac{1}{\Delta{t}}\sigma_{v_{z, i-1}}\right)^2}$$

## Curve:
$$a_{a_{x, i}} = \frac{v_{a_{x, i}} - v_{a_{x, i-1}}}{t_i - t_{i - 1}}
\hspace{10pt}
a_{b_{x, i}} = \frac{v_{b_{x, i}} - v_{b_{x, i-1}}}{t_i - t_{i - 1}}
\hspace{10pt}
a_{c_{x, i}} = \frac{v_{c_{x, i}} - v_{c_{x, i-1}}}{t_i - t_{i - 1}}
\hspace{10pt}
a_{d_{x, i}} = \frac{v_{d_{x, i}} - v_{d_{x, i-1}}}{t_i - t_{i - 1}}$$

$$\textbf{a}_{x, i}(t) = a_{a_{x, i}}^3(t) + a_{b_{x, i}}^2(t) + a_{c_{x, i}}(t) + a_{d_{x, i}}(t)$$
And so on for y and z...Acceleration Across the Curve
$$\textbf{a}_{total, i}(t) = \sqrt{\textbf{a}_{x, i}(t)^2 + \textbf{a}_{y, i}(t)^2 + \textbf{a}_{z, i}(t)^2}$$

### Acceleration Per Coefficient

### Cubic Acceleration: 
Rate of change of the "S-curve" speed
$$a_{a, i} = \sqrt{a_{a_{x, i}}^2 + a_{a_{y, i}}^2 + a_{a_{z, i}}^2}$$

Quadratic Acceleration: Rate of change of lip rounding speed
$$a_{b, i} = \sqrt{a_{b_{x, i}}^2 + a_{b_{y, i}}^2 + a_{b_{z, i}}^2}$$

Linear Acceleration: Rate of change of start/end curve speed
$$a_{c, i} = \sqrt{a_{c_{x, i}}^2 + a_{c_{y, i}}^2 + a_{c_{z, i}}^2}$$

Vertical Acceleration: Rate of change of vertical movement speed
$$a_{d, i} = \sqrt{a_{d_{x, i}}^2 + a_{d_{y, i}}^2 + a_{d_{z, i}}^2}$$

### Curve Acceleration Uncertainty
#### Partial Derivatives with respect to $v_{a_{x, i}}$, $v_{b_{x, i}}$, $v_{c_{x, i}}$, $v_{d_{x, i}}$(And so on for y and z)

$$\frac{\partial{a_{a_x}}}{\partial{v_{a_{x, i}}}} = \frac{1}{\Delta{t}}
\hspace{10pt}
\frac{\partial{a_{b_x}}}{\partial{v_{b_{x, i}}}} = \frac{1}{\Delta{t}}
\hspace{10pt}
\frac{\partial{a_{c_x}}}{\partial{v_{c_{x, i}}}} = \frac{1}{\Delta{t}}
\hspace{10pt}
\frac{\partial{a_{d_x}}}{\partial{v_{d_{x, i}}}} = \frac{1}{\Delta{t}}$$

#### Uncertainty Propagation
$$\sigma_{a_{a_x}} = \sqrt{\left(\frac{1}{\Delta{t}}\sigma_{v_{a_{x, i}}}\right)^2 + \left(-\frac{1}{\Delta{t}}\sigma_{v_{a_{x, i-1}}}\right)^2}$$
(And so on for b, c, and d)

## Area:
Calculates the acceleration of the area and its uncertainty.
$$a_{area, i} = \frac{v_{a, i} - v_{a, i-1}}{t_i - t_{i-1}}$$
### Area Acceleration Uncertainty
#### Partial derivative with respect to $v_{a, i}$
$$\frac{\partial{a_{area}}}{\partial{v_{a, i}}} = \frac{1}{\Delta{t}}$$
#### Uncertainty Propagation
$$\sigma_{a_{area}} = \sqrt{\left(\frac{1}{\Delta{t}}\sigma_{v_{a, i}}\right)^2 + \left(-\frac{1}{\Delta{t}}\sigma_{v_{a, i-1}}\right)^2}$$