# Area Logic

## Some Info
Because we conform all curves to cubic curves, we are able to reuse the same logic as in the Python script version. 

### 1. Coordinate Basis Transformation (3D)

$$\vec{v}_1 = \vec{P}_{LC} - \vec{P}_{RC}, \quad \vec{v}_2 = \vec{P}_{LM} - \vec{P}_{PH}$$

$$\vec{v}_3 = \vec{v}_1 \times \vec{v}_2$$

$$B = \begin{bmatrix} \vec{v}_1 & \vec{v}_2 & \vec{v}_3 \end{bmatrix}, \quad B \in \mathbb{R}^{3 \times 3}$$

$$\vec{P}_{bio} = B^{-1}(\vec{P}_{raw} - \vec{P}_{origin})$$

$$\text{scale} = |\det(B)|$$

### 2. Cubic Bézier Midpoint (Parametric )

$$\vec{B}(0.5) = \frac{1}{8}\vec{P}_0 + \frac{3}{8}\vec{P}_1 + \frac{3}{8}\vec{P}_2 + \frac{1}{8}\vec{P}_3$$

### 3. Cubic Signed Area (Green's Theorem / Cross Product)

$$\mathcal{C}(\vec{u}, \vec{v}) = u_x v_y - u_y v_x$$

$$\mathcal{A}_{cubic} = \frac{1}{20} \left[ 6\mathcal{C}(\vec{P}_0, \vec{P}_1) + 3\mathcal{C}(\vec{P}_0, \vec{P}_2) + \mathcal{C}(\vec{P}_0, \vec{P}_3) + 3\mathcal{C}(\vec{P}_1, \vec{P}_2) + 3\mathcal{C}(\vec{P}_1, \vec{P}_3) + 6\mathcal{C}(\vec{P}_2, \vec{P}_3) \right]$$

### 4. de Casteljau Sub-segment Splitting (at )

$$\vec{P}_{01} = (1-t)\vec{P}_0 + t\vec{P}_1, \quad \vec{P}_{12} = (1-t)\vec{P}_1 + t\vec{P}_2, \quad \vec{P}_{23} = (1-t)\vec{P}_2 + t\vec{P}_3$$

$$\vec{P}_{012} = (1-t)\vec{P}_{01} + t\vec{P}_{12}, \quad \vec{P}_{123} = (1-t)\vec{P}_{12} + t\vec{P}_{23}$$

$$\vec{P}_{0123} = (1-t)\vec{P}_{012} + t\vec{P}_{123}$$

$$\text{Left} = \{\vec{P}_0, \vec{P}_{01}, \vec{P}_{012}, \vec{P}_{0123}\}, \quad \text{Right} = \{\vec{P}_{0123}, \vec{P}_{123}, \vec{P}_{23}, \vec{P}_3\}$$

### 5. Root Finding for Quadrant Intersection

$$\vec{B}(t) = (1-t)^3\vec{P}_0 + 3(1-t)^2t\vec{P}_1 + 3(1-t)t^2\vec{P}_2 + t^3\vec{P}_3 = 0$$

$$\text{Solve for } t \in (0, 1):$$

$$t^3(-\vec{P}_0 + 3\vec{P}_1 - 3\vec{P}_2 + \vec{P}_3) + t^2(3\vec{P}_0 - 6\vec{P}_1 + 3\vec{P}_2) + t(-3\vec{P}_0 + 3\vec{P}_1) + \vec{P}_0 = 0$$

### 6. Quadrant Accumulation and Scaling

$$\text{Area}_q = \text{scale} \cdot \sum \mathcal{A}_{cubic}(\text{pieces} \in \text{Quadrant}_q)$$

$$\text{Area}_{total} = \sum_{q=1}^{4} |\text{Area}_q|$$
