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
Area logic 

Calculates the area of the mouth and also the 4 quadrants defined by the commissures and a vertical line through the philtrum
*/

use std::f64::consts::PI;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn zero() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }

    pub fn sub(&self, other: Vec3) -> Vec3 {
        Vec3::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }

    pub fn add(&self, other: Vec3) -> Vec3 {
        Vec3::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }

    pub fn scale(&self, s: f64) -> Vec3 {
        Vec3::new(self.x * s, self.y * s, self.z * s)
    }

    pub fn cross(&self, other: Vec3) -> Vec3 {
        Vec3::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }

    // u_x * v_y - u_y * v_x (2d vers)
    pub fn cross_2d(&self, other: Vec3) -> f64 {
        self.x * other.y - self.y * other.x
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Mat3 {
    pub cols: [Vec3; 3],
}

impl Mat3 {
    pub fn from_cols(v1: Vec3, v2: Vec3, v3: Vec3) -> Self {
        Self { cols: [v1, v2, v3] }
    }

    pub fn determinant(&self) -> f64 {
        let [c1, c2, c3] = self.cols;
        c1.x * (c2.y * c3.z - c2.z * c3.y) -
        c1.y * (c2.x * c3.z - c2.z * c3.x) +
        c1.z * (c2.x * c3.y - c2.y * c3.x)
    }

    pub fn inverse(&self) -> Option<Mat3> {
        let det = self.determinant();
        if det.abs() < 1e-9 {
            return None;
        }
        let inv_det = 1.0 / det;

        let [c1, c2, c3] = self.cols;

        let v1 = Vec3::new(
            (c2.y * c3.z - c2.z * c3.y) * inv_det,
            (c3.y * c1.z - c3.z * c1.y) * inv_det,
            (c1.y * c2.z - c1.z * c2.y) * inv_det,
        );
        let v2 = Vec3::new(
            (c2.z * c3.x - c2.x * c3.z) * inv_det,
            (c3.z * c1.x - c3.x * c1.z) * inv_det,
            (c1.z * c2.x - c1.x * c2.z) * inv_det,
        );
        let v3 = Vec3::new(
            (c2.x * c3.y - c2.y * c3.x) * inv_det,
            (c3.x * c1.y - c3.y * c1.x) * inv_det,
            (c1.x * c2.y - c1.y * c2.x) * inv_det,
        );

        // note: The rows/cols might need transposing depending on multiplication convention.
        // standard formula P_new = M_inv * P_old implies standard matrix mult.
        // The adjugate construction above transposes the cofactors correctly for standard M^-1.
        
        // We construct column-wise
        Some(Mat3 { cols: [
            Vec3::new(v1.x, v2.x, v3.x), // Row 1 becomes Col 1
            Vec3::new(v1.y, v2.y, v3.y),
            Vec3::new(v1.z, v2.z, v3.z)
        ]})
    }

    pub fn transform_vector(&self, v: Vec3) -> Vec3 {
        let [c1, c2, c3] = self.cols;
        Vec3::new(
            c1.x * v.x + c2.x * v.y + c3.x * v.z,
            c1.y * v.x + c2.y * v.y + c3.y * v.z,
            c1.z * v.x + c2.z * v.y + c3.z * v.z,
        )
    }
}

#[derive(Debug, Clone)]
pub struct CubicBezier {
    pub p0: Vec3,
    pub p1: Vec3,
    pub p2: Vec3,
    pub p3: Vec3,
}

impl CubicBezier {
    pub fn new(p0: Vec3, p1: Vec3, p2: Vec3, p3: Vec3) -> Self {
        Self { p0, p1, p2, p3 }
    }

    pub fn midpoint(&self) -> Vec3 {
        // B(0.5) = 1/8 P0 + 3/8 P1 + 3/8 P2 + 1/8 P3
        let term1 = self.p0.add(self.p3).scale(1.0 / 8.0);
        let term2 = self.p1.add(self.p2).scale(3.0 / 8.0);
        term1.add(term2)
    }

    // cubic Signed Area (Green's Theorem / Cross Product)
    // A_cubic = 1/20 * [6C(P0, P1) + 3C(P0, P2) + C(P0, P3) + 3C(P1, P2) + 3C(P1, P3) + 6C(P2, P3)]
    pub fn signed_area_2d(&self) -> f64 {
        let c01 = self.p0.cross_2d(self.p1);
        let c02 = self.p0.cross_2d(self.p2);
        let c03 = self.p0.cross_2d(self.p3);
        let c12 = self.p1.cross_2d(self.p2);
        let c13 = self.p1.cross_2d(self.p3);
        let c23 = self.p2.cross_2d(self.p3);

        (1.0 / 20.0) * (6.0 * c01 + 3.0 * c02 + c03 + 3.0 * c12 + 3.0 * c13 + 6.0 * c23)
    }

    // de Casteljau Sub-segment Splitting at t
    pub fn split(&self, t: f64) -> (CubicBezier, CubicBezier) {
        let p0 = self.p0;
        let p1 = self.p1;
        let p2 = self.p2;
        let p3 = self.p3;

        let p01 = p0.scale(1.0 - t).add(p1.scale(t));
        let p12 = p1.scale(1.0 - t).add(p2.scale(t));
        let p23 = p2.scale(1.0 - t).add(p3.scale(t));

        let p012 = p01.scale(1.0 - t).add(p12.scale(t));
        let p123 = p12.scale(1.0 - t).add(p23.scale(t));

        let p0123 = p012.scale(1.0 - t).add(p123.scale(t));

        let left = CubicBezier::new(p0, p01, p012, p0123);
        let right = CubicBezier::new(p0123, p123, p23, p3);

        (left, right)
    }

    // root findinf for quad intersec (solve B(t) = 0 for one dimension)
    // Returns sorted list of t in (0, 1) where the curve crosses zero on the specified axis.
    // axis_selector: function to extract value (e.g., |v| v.x)
    pub fn find_roots_1d<F>(&self, axis_selector: F) -> Vec<f64>
    where
        F: Fn(Vec3) -> f64,
    {
        let v0 = axis_selector(self.p0);
        let v1 = axis_selector(self.p1);
        let v2 = axis_selector(self.p2);
        let v3 = axis_selector(self.p3);

        // Coefficients for At^3 + Bt^2 + Ct + D = 0
        let d = v0;
        let c = -3.0 * v0 + 3.0 * v1;
        let b = 3.0 * v0 - 6.0 * v1 + 3.0 * v2;
        let a = -v0 + 3.0 * v1 - 3.0 * v2 + v3;

        solve_cubic(a, b, c, d)
    }
}

// solves At^3 + Bt^2 + Ct + D = 0 for t in (0, 1)
// uses Cardano's method or numerical approximation logic as needed.
// For stability with floating points, a simplified analytic approach is used.
fn solve_cubic(a: f64, b: f64, c: f64, d: f64) -> Vec<f64> {
    let mut roots = Vec::new();
    const EPSILON: f64 = 1e-9;

    if a.abs() < EPSILON {
        // Quadratic: Bt^2 + Ct + D = 0
        if b.abs() < EPSILON {
            // Linear: Ct + D = 0 -> t = -D/C
            if c.abs() > EPSILON {
                let t = -d / c;
                if t > EPSILON && t < 1.0 - EPSILON {
                    roots.push(t);
                }
            }
        } else {
            let discriminant = c * c - 4.0 * b * d;
            if discriminant >= 0.0 {
                let sqrt_d = discriminant.sqrt();
                let t1 = (-c - sqrt_d) / (2.0 * b);
                let t2 = (-c + sqrt_d) / (2.0 * b);
                if t1 > EPSILON && t1 < 1.0 - EPSILON { roots.push(t1); }
                if t2 > EPSILON && t2 < 1.0 - EPSILON { roots.push(t2); }
            }
        }
    } else {
        let a_n = b / a;
        let b_n = c / a;
        let c_n = d / a;

        let q = (3.0 * b_n - a_n * a_n) / 9.0;
        let r = (9.0 * a_n * b_n - 27.0 * c_n - 2.0 * a_n.powi(3)) / 54.0;
        let d_n = q.powi(3) + r.powi(2); 

        if d_n > 0.0 {
            let s = (r + d_n.sqrt()).cbrt();
            let t = (r - d_n.sqrt()).cbrt();
            let root = s + t - a_n / 3.0;
            if root > EPSILON && root < 1.0 - EPSILON { roots.push(root); }
        } else {
            let theta = (r / (-q.powi(3)).sqrt()).acos();
            let sqrt_neg_q = (-q).sqrt();
            let t1 = 2.0 * sqrt_neg_q * (theta / 3.0).cos() - a_n / 3.0;
            let t2 = 2.0 * sqrt_neg_q * ((theta + 2.0 * PI) / 3.0).cos() - a_n / 3.0;
            let t3 = 2.0 * sqrt_neg_q * ((theta + 4.0 * PI) / 3.0).cos() - a_n / 3.0;

            for t in [t1, t2, t3] {
                if t > EPSILON && t < 1.0 - EPSILON { roots.push(t); }
            }
        }
    }
    roots.sort_by(|a, b| a.partial_cmp(b).unwrap());
    roots
}

pub fn calculate_basis(p_lc: Vec3, p_rc: Vec3, p_lm: Vec3, p_ph: Vec3) -> Result<(Mat3, f64), String> {
    // v1 = PLC - PRC
    let v1 = p_lc.sub(p_rc);
    // v2 = PLM - PPH
    let v2 = p_lm.sub(p_ph);
    // v3 = v1 x v2
    let v3 = v1.cross(v2);

    let basis = Mat3::from_cols(v1, v2, v3);
    let scale = basis.determinant().abs();

    match basis.inverse() {
        Some(inv) => Ok((inv, scale)),
        None => Err("Basis matrix is singular (determinant is 0)".to_string()),
    }
}

pub fn transform_curve(curve: &CubicBezier, basis_inv: &Mat3, origin: Vec3) -> CubicBezier {
    let transform = |p: Vec3| basis_inv.transform_vector(p.sub(origin));
    CubicBezier::new(
        transform(curve.p0),
        transform(curve.p1),
        transform(curve.p2),
        transform(curve.p3),
    )
}

pub fn calculate_total_area(
    raw_curves: Vec<CubicBezier>,
    basis_inv: Mat3,
    scale: f64,
    origin: Vec3,
) -> f64 {
    let mut area_q1 = 0.0;
    let mut area_q2 = 0.0;
    let mut area_q3 = 0.0;
    let mut area_q4 = 0.0;

    for raw_curve in raw_curves {
        let bio_curve = transform_curve(&raw_curve, &basis_inv, origin);

        let mut t_values = Vec::new();
        t_values.extend(bio_curve.find_roots_1d(|v| v.x));
        t_values.extend(bio_curve.find_roots_1d(|v| v.y));
        t_values.sort_by(|a, b| a.partial_cmp(b).unwrap());
        t_values.dedup();

        let mut segments = Vec::new();
        let mut current_curve = bio_curve;
        let mut accumulated_t = 0.0; // Track local t relative to start

        for t in t_values {

            if t <= accumulated_t { continue; }
            let local_t = (t - accumulated_t) / (1.0 - accumulated_t);
            
            if local_t > 0.0 && local_t < 1.0 {
                let (left, right) = current_curve.split(local_t);
                segments.push(left);
                current_curve = right;
                accumulated_t = t;
            }
        }
        segments.push(current_curve); 

        for segment in segments {
            let mid = segment.midpoint();
            let segment_area = segment.signed_area_2d();
            let contribution = scale * segment_area;

            if mid.x >= 0.0 && mid.y >= 0.0 {
                area_q1 += contribution;
            } else if mid.x < 0.0 && mid.y >= 0.0 {
                area_q2 += contribution;
            } else if mid.x < 0.0 && mid.y < 0.0 {
                area_q3 += contribution;
            } else {
                area_q4 += contribution;
            }
        }
    }

    area_q1.abs() + area_q2.abs() + area_q3.abs() + area_q4.abs()
}