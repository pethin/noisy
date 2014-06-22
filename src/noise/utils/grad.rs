//! Helper functions to compute gradients-dot-residualvectors (1D to 4D).

use utils::if_else;

/// Compute 1D gradient-dot-residualvector.
pub fn grad1(hash: uint, x: f64) -> f64 {
  let h: uint = hash & 15;
  let mut grad: f64 = 1.0 + (h & 7) as f64; // Gradient value 1.0, 2.0, ..., 8.0
  if (h & 8) != 0 {
    grad = -grad; // Set a random sign for the gradient
  }

  grad * x // Multiply the gradient with the distance
}

/// Compute 2D gradient-dot-residualvector.
pub fn grad2(hash: uint, x: f64, y: f64) -> f64 {
  // Convert low 3 bits of hash code into 8 simple gradient directions,
  // and compute the dot product with (x,y).
  let h: uint = hash & 7;
  let u: f64 = if_else(h < 4, x, y);
  let v: f64 = if_else(h < 4, y, x);

  if_else(h&1 != 0, -u, u) + if_else(h&2 != 0, -2.0*v, 2.0*v)
}

/// Compute 3D gradient-dot-residualvector.
pub fn grad3(hash: uint, x: f64, y: f64, z: f64) -> f64 {
  // Convert low 4 bits of hash code into 12 simple gradient directions,
  // and compute dot product.
  let h: uint = hash & 15;
  let u: f64 = if_else(h < 8, x, y);
  // Fix repeats at h = 12 to 15
  let v: f64 = if_else(h < 4, y, if_else(h == 12 || h == 14, x, z));

  if_else(h&1 != 0, -u, u) + if_else(h&2 != 0, -v, v)
}
