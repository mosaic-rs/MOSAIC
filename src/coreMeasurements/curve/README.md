# CURVE FITTING
Unlike the Python version of MOSAIC, the Rust version takes an entirely new aproach to curve fitting.

## How is it different?
- It is data agnostic. It takes raw data from the UMD and calculates curves. As a result, you can calculate tongue curvature from EMA, lip curvature, etc.
- It can take an arbitrary amount of points. Whether you have 4 points or 400, the curve fitting algorithm is able to create an accurate cubic curve, tell you the coefficients, and the points the curve passes through. 

## Mathematics:

This is an example of converting 5 points of data into a cubic curve.

### 1. Parameterization (Chord Length)

$$d_i = \sqrt{(x_{i+1}-x_i)^2 + (y_{i+1}-y_i)^2 + (z_{i+1}-z_i)^2}$$

$$D_k = \sum_{i=0}^{k-1} d_i \quad \text{for } k=0 \dots 4, \quad D_0 = 0$$

$$L = \sum_{i=0}^{3} d_i$$

$$t_i = \frac{D_i}{L} \in [0, 1]$$

### 2. Matrix Definition

$$M = \begin{bmatrix}

t_0^3 & t_0^2 & t_0 & 1 \\

t_1^3 & t_1^2 & t_1 & 1 \\

t_2^3 & t_2^2 & t_2 & 1 \\

t_3^3 & t_3^2 & t_3 & 1 \\

t_4^3 & t_4^2 & t_4 & 1

\end{bmatrix}, \quad

\mathbf{P}_x = \begin{bmatrix} x_0 \ x_1 \ x_2 \ x_3 \ x_4 \end{bmatrix}, \quad

\mathbf{P}_y = \begin{bmatrix} y_0 \ y_1 \ y_2 \ y_3 \ y_4 \end{bmatrix}, \quad

\mathbf{P}_z = \begin{bmatrix} z_0 \ z_1 \ z_2 \ z_3 \ z_4 \end{bmatrix}$$

### 3. SVD Solver (Singular Value Decomposition)

$$M = U \Sigma V^T$$

$$U \in \mathbb{R}^{5 \times 5}, \quad \Sigma \in \mathbb{R}^{5 \times 4}, \quad V^T \in \mathbb{R}^{4 \times 4}$$

$$\Sigma = \begin{bmatrix}

\sigma_1 & 0 & 0 & 0 \\

0 & \sigma_2 & 0 & 0 \\

0 & 0 & \sigma_3 & 0 \\

0 & 0 & 0 & \sigma_4 \\

0 & 0 & 0 & 0

\end{bmatrix}$$

$$\mathbf{C} = V \Sigma^+ U^T \mathbf{P}$$

$$\Sigma^+ = \text{diag}(1/\sigma_i) \text{ for } \sigma_i > \epsilon, \text{ else } 0$$

### 4. Coefficient Vectors

$$\mathbf{C}_x = \begin{bmatrix} a_x \ b_x \ c_x \ d_x \end{bmatrix}, \quad

\mathbf{C}_y = \begin{bmatrix} a_y \ b_y \ c_y \ d_y \end{bmatrix}, \quad

\mathbf{C}_z = \begin{bmatrix} a_z \ b_z \ c_z \ d_z \end{bmatrix}$$

### 5. Final Curve Construction

$$\vec{f}(t) = \begin{cases}

x(t) = a_x t^3 + b_x t^2 + c_x t + d_x \\

y(t) = a_y t^3 + b_y t^2 + c_y t + d_y \\

z(t) = a_z t^3 + b_z t^2 + c_z t + d_z

\end{cases}$$