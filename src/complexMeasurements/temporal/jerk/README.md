# Jerk
This section of complexMeasurements calculates jerk (rate of change of acceleration) of measurements.
## Landmarks:
Calculates the jerk of landmarks and the uncertainty of the jerk.

$$a_i =
\begin{bmatrix}
a_{x, i} \\
a_{y, i} \\
a_{z, i}
\end{bmatrix}$$
$$j_{i} = \frac{a_{i} - a_{i - 1}}{t_{i} - t_{i-1}}$$
Where,
$$j_{x, i} = \frac{a_{x, i} - a_{x, i-1}}{t_i - t_{i-1}}
\hspace{10pt}
j_{y, i} = \frac{a_{y, i} - a_{y, i-1}}{t_i - t_{i-1}}
\hspace{10pt}
j_{z, i} = \frac{a_{z, i} - a_{z, i-1}}{t_i - t_{i-1}}$$
We can also get the radius jerk
$$j_{r, i} = \sqrt{j_{x, i}^2 + j_{y, i}^2 + j_{z, i}^2}$$
### Landmark Jerk Uncertainty
#### Partial derivative with respect to $a_{x, i}$, $a_{y, i}$, and $a_{z, i}$
$$\frac{\partial{j_x}}{\partial{a_{x, i}}} = \frac{1}{\Delta{t}}
\hspace{10pt}
\frac{\partial{j_y}}{\partial{a_{y, i}}} = \frac{1}{\Delta{t}}
\hspace{10pt}
\frac{\partial{j_z}}{\partial{a_{z, i}}} = \frac{1}{\Delta{t}}$$
#### Partial Derivative with respect to $a_{x, i-1}$, $a_{y, i-1}$, $a_{z, i-1}$
$$\frac{\partial{j_x}}{\partial{a_{x, i-1}}} = - \frac{1}{\Delta{t}}
\hspace{10pt}
\frac{\partial{j_y}}{\partial{a_{y, i-1}}} = - \frac{1}{\Delta{t}}
\hspace{10pt}
\frac{\partial{j_z}}{\partial{a_{z, i-1}}} = - \frac{1}{\Delta{t}}$$
#### Uncertainty Propagation
$$\sigma_{j_{x}} = \sqrt{\left(\frac{1}{\Delta{t}}\sigma_{a_{x, i}}\right)^2 + \left(-\frac{1}{\Delta{t}}\sigma_{a_{x, i-1}}\right)^2}$$
$$\sigma_{j_{y}} = \sqrt{\left(\frac{1}{\Delta{t}}\sigma_{a_{y, i}}\right)^2 + \left(-\frac{1}{\Delta{t}}\sigma_{a_{y, i-1}}\right)^2}$$
$$\sigma_{j_{z}} = \sqrt{\left(\frac{1}{\Delta{t}}\sigma_{a_{z, i}}\right)^2 + \left(-\frac{1}{\Delta{t}}\sigma_{a_{z, i-1}}\right)^2}$$
## Curve:
$$j_{a_{x, i}} = \frac{a_{a_{x, i}} - a_{a_{x, i-1}}}{t_i - t_{i - 1}}
\hspace{10pt}
j_{b_{x, i}} = \frac{a_{b_{x, i}} - a_{b_{x, i-1}}}{t_i - t_{i - 1}}
\hspace{10pt}
j_{c_{x, i}} = \frac{a_{c_{x, i}} - a_{c_{x, i-1}}}{t_i - t_{i - 1}}
\hspace{10pt}
j_{d_{x, i}} = \frac{a_{d_{x, i}} - a_{d_{x, i-1}}}{t_i - t_{i - 1}}$$
$$\textbf{j}_{x, i}(t) = j_{a_{x, i}}^3(t) + j_{b_{x, i}}^2(t) + j_{c_{x, i}}(t) + j_{d_{x, i}}(t)$$
### Jerk Across the Curve
$$\textbf{j}_{total, i}(t) = \sqrt{\textbf{j}_{x, i}(t)^2 + \textbf{j}_{y, i}(t)^2 + \textbf{j}_{z, i}(t)^2}$$
### Jerk Per Coefficient
Cubic Jerk: Rate of change of pucker/S-curve acceleration
$$j_{a, i} = \sqrt{j_{a_{x, i}}^2 + j_{a_{y, i}}^2 + j_{a_{z, i}}^2}$$
Quadratic Jerk: Rate of change of lip rounding acceleration
$$j_{b, i} = \sqrt{j_{b_{x, i}}^2 + j_{b_{y, i}}^2 + j_{b_{z, i}}^2}$$
Linear Jerk: Rate of change of start/end acceleration
$$j_{c, i} = \sqrt{j_{c_{x, i}}^2 + j_{c_{y, i}}^2 + j_{c_{z, i}}^2}$$
Vertical Jerk: Rate of change of vertical acceleration
$$j_{d, i} = \sqrt{j_{d_{x, i}}^2 + j_{d_{y, i}}^2 + j_{d_{z, i}}^2}$$
### Curve Jerk Uncertainty
#### Partial Derivatives with respect to $a_{a_{x, i}}$, $a_{b_{x, i}}$, $a_{c_{x, i}}$, $a_{d_{x, i}}$
$$\frac{\partial{j_{a_x}}}{\partial{a_{a_{x, i}}}} = \frac{1}{\Delta{t}}
\hspace{10pt}
\frac{\partial{j_{b_x}}}{\partial{a_{b_{x, i}}}} = \frac{1}{\Delta{t}}
\hspace{10pt}
\frac{\partial{j_{c_x}}}{\partial{a_{c_{x, i}}}} = \frac{1}{\Delta{t}}
\hspace{10pt}
\frac{\partial{j_{d_x}}}{\partial{a_{d_{x, i}}}} = \frac{1}{\Delta{t}}$$
#### Uncertainty Propagation
$$\sigma_{j_{a_x}} = \sqrt{\left(\frac{1}{\Delta{t}}\sigma_{a_{a_{x, i}}}\right)^2 + \left(-\frac{1}{\Delta{t}}\sigma_{a_{a_{x, i-1}}}\right)^2}$$
(And so on for b, c, and d)

## Area:
Calculates the jerk of the area and its uncertainty.
$$j_{area, i} = \frac{a_{area, i} - a_{area, i-1}}{t_i - t_{i-1}}$$
### Area Jerk Uncertainty

#### Partial derivative with respect to $a_{area, i}$
$$\frac{\partial{j_{area}}}{\partial{a_{area, i}}} = \frac{1}{\Delta{t}}$$
#### Uncertainty Propagation
$$\sigma_{j_{area}} = \sqrt{\left(\frac{1}{\Delta{t}}\sigma_{a_{area, i}}\right)^2 + \left(-\frac{1}{\Delta{t}}\sigma_{a_{area, i-1}}\right)^2}$$