//! An implementation of Improved [Perlin Noise]
//! (https://en.wikipedia.org/wiki/Perlin_noise).
//!
//! Based on a improved perlin noise algorithm for 2D, 3D and 4D in C.
//! Which is based on example code by Ken Perlin at Siggraph 2002.
//! With optimisations by Stefan Gustavson (stegu@itn.liu.se).

use std::rand::{Rng, XorShiftRng, weak_rng};

use utils::{fade, fastfloor, lerp};
use utils::grad::{grad1, grad2, grad3};
use gen::NoiseGen;

/// A ImprovedPerlin noise generator.
#[deriving(Clone, PartialEq, Eq)]
pub struct ImprovedPerlin {
  perm: Vec<u8>
}

impl ImprovedPerlin {
  /// Initializes a new ImprovedPerlin instance with a random seed using XorShiftRng.
  ///
  /// # Example
  ///
  /// ```rust
  /// use noise::gen::ImprovedPerlin;
  ///
  /// let improved_perlin = ImprovedPerlin::new();
  /// ```
  pub fn new() -> ImprovedPerlin {
    let mut rng: XorShiftRng = weak_rng();

    let p: Vec<u8> = Vec::from_fn(256, |_| rng.gen::<u8>());
    let perm: Vec<u8> = Vec::from_fn(512, |idx| *p.get(idx & 255));

    ImprovedPerlin {
      perm: perm
    }
  }

  /// Initializes a new ImprovedPerlin instance with a random number generator.
  ///
  /// # Example
  ///
  /// ```rust
  /// # use std::rand::StdRng;
  /// use noise::gen::ImprovedPerlin;
  ///
  /// let mut rng: StdRng = StdRng::new().unwrap();
  /// let improved_perlin = ImprovedPerlin::from_rng(&mut rng);
  /// ```
  ///
  /// This also allows you to initialize the instance with a seed:
  ///
  /// # Example
  ///
  /// ```rust
  /// # use std::rand::{StdRng, SeedableRng};
  /// use noise::gen::ImprovedPerlin;
  ///
  /// let mut rng: StdRng = SeedableRng::from_seed(&[1337]);
  /// let improved_perlin = ImprovedPerlin::from_rng(&mut rng);
  /// ```
  pub fn from_rng<R: Rng>(rng: &mut R) -> ImprovedPerlin {
    let p: Vec<u8> = Vec::from_fn(256, |_| rng.gen::<u8>());
    let perm: Vec<u8> = Vec::from_fn(512, |idx| *p.get(idx & 255));

    ImprovedPerlin {
      perm: perm
    }
  }
}

impl NoiseGen for ImprovedPerlin {
  /// Given an x coordinate, return a value in the interval [-1, 1].
  ///
  /// # Example
  ///
  /// ```rust
  /// use noise::gen::{NoiseGen, ImprovedPerlin};
  ///
  /// let improved_perlin = ImprovedPerlin::new();
  /// let val = improved_perlin.noise1d(123.0 * 0.04);
  /// ```
  fn noise1d(&self, xin: f64) -> f64 {
    // View the permutation table as a slice
    let perm = self.perm.as_slice();

    let ix0: int = fastfloor(xin); // Integer part of x
    let fx0: f64 = xin - ix0 as f64; // Fractional part of x
    let fx1: f64 = fx0 - 1.0;
    let ix1: int = ix0 + 1;

    // Wrap the integer indices at 256, to avoid indexing perm[] out of bounds
    let ii: uint = (ix0 & 255) as uint;
    let jj: uint = (ix1 & 255) as uint;

    // Compute the fade curve.
    let s: f64 = fade(fx0);

    // Work out the hashed gradient indices.
    let gi0: uint = perm[ii] as uint;
    let gi1: uint = perm[jj] as uint;

    // Calculate the gradients.
    let nx0 = grad1(gi0, fx0);
    let nx1 = grad1(gi1, fx1);

    // The result is scaled to return values in the interval [-1, 1].
    0.188 * lerp(s, nx0, nx1)
  }

  /// Given a (x, y) coordinate, return a value in the interval [-1, 1].
  ///
  /// # Example
  ///
  /// ```rust
  /// use noise::gen::{NoiseGen, ImprovedPerlin};
  ///
  /// let improved_perlin = ImprovedPerlin::new();
  /// let val = improved_perlin.noise2d(
  ///   123.0 * 0.04,
  ///   132.0 * 0.04
  /// );
  /// ```
  fn noise2d(&self, xin: f64, yin: f64) -> f64 {
    // View the permutation table as a slice
    let perm = self.perm.as_slice();

    let ix0: int = fastfloor(xin); // Integer part of x
    let iy0: int = fastfloor(yin); // Integer part of y
    let fx0: f64 = xin - ix0 as f64; // Fractional part of x
    let fy0: f64 = yin - iy0 as f64; // Fractional part of y
    let fx1: f64 = fx0 - 1.0;
    let fy1: f64 = fy0 - 1.0;

    // Wrap the integer indices at 256, to avoid indexing perm[] out of bounds
    let ix1: uint = ((ix0 + 1) & 255) as uint;
    let iy1: uint = ((iy0 + 1) & 255) as uint;
    let ii: uint = (ix0 & 255) as uint;
    let jj: uint = (iy0 & 255) as uint;

    // Compute the fade curves.
    let t: f64 = fade(fy0);
    let s: f64 = fade(fx0);

    // Work out the hashed gradient indices.
    let gi0: uint = perm[ii + (perm[jj] as uint)] as uint;
    let gi1: uint = perm[ii + (perm[iy1] as uint)] as uint;
    let gi2: uint = perm[ix1 + (perm[jj] as uint)] as uint;
    let gi3: uint = perm[ix1 + (perm[iy1] as uint)] as uint;

    // Calculate the gradients.
    let nx0: f64 = grad2(gi0, fx0, fy0);
    let nx1: f64 = grad2(gi1, fx0, fy1);
    let nx2: f64 = grad2(gi2, fx1, fy0);
    let nx3: f64 = grad2(gi3, fx1, fy1);

    let n0: f64 = lerp(t, nx0, nx1);
    let n1: f64 = lerp(t, nx2, nx3);

    // The result is scaled to return values in the interval [-1, 1].
    0.507 * lerp(s, n0, n1)
  }

  /// Given a (x, y, z) coordinate, return a value in the interval [-1, 1].
  ///
  /// # Example
  ///
  /// ```rust
  /// use noise::gen::{NoiseGen, ImprovedPerlin};
  ///
  /// let improved_perlin = ImprovedPerlin::new();
  /// let val = improved_perlin.noise3d(
  ///   123.0 * 0.04,
  ///   231.0 * 0.04,
  ///   321.0 * 0.04
  /// );
  /// ```
  fn noise3d(&self, xin: f64, yin: f64, zin: f64) -> f64 {
    // View the permutation table as a slice
    let perm = self.perm.as_slice();

    let ix0: int = fastfloor(xin); // Integer part of x
    let iy0: int = fastfloor(yin); // Integer part of y
    let iz0: int = fastfloor(zin); // Integer part of z
    let fx0: f64 = xin - ix0 as f64; // Fractional part of x
    let fy0: f64 = yin - iy0 as f64; // Fractional part of y
    let fz0: f64 = zin - iz0 as f64; // Fractional part of z
    let fx1: f64 = fx0 - 1.0;
    let fy1: f64 = fy0 - 1.0;
    let fz1: f64 = fz0 - 1.0;

    // Wrap the integer indices at 256, to avoid indexing perm[] out of bounds
    let ix1: uint = ((ix0 + 1) & 255) as uint;
    let iy1: uint = ((iy0 + 1) & 255) as uint;
    let iz1: uint = ((iz0 + 1) & 255) as uint;
    let ii: uint = (ix0 & 255) as uint;
    let jj: uint = (iy0 & 255) as uint;
    let kk: uint = (iz0 & 255) as uint;

    // Compute the fade curves.
    let r: f64 = fade(fz0);
    let t: f64 = fade(fy0);
    let s: f64 = fade(fx0);

    // Work out the hashed gradient indices.
    let gi0: uint = perm[ii + (perm[jj + (perm[kk] as uint)] as uint)] as uint;
    let gi1: uint = perm[ii + (perm[jj + (perm[iz1] as uint)] as uint)] as uint;
    let gi2: uint = perm[ii + (perm[iy1 + (perm[kk] as uint)] as uint)] as uint;
    let gi3: uint = perm[ii + (perm[iy1 + (perm[iz1] as uint)] as uint)] as uint;
    let gi4: uint = perm[ix1 + (perm[jj + (perm[kk] as uint)] as uint)] as uint;
    let gi5: uint = perm[ix1 + (perm[jj + (perm[iz1] as uint)] as uint)] as uint;
    let gi6: uint = perm[ix1 + (perm[iy1 + (perm[kk] as uint)] as uint)] as uint;
    let gi7: uint = perm[ix1 + (perm[iy1 + (perm[iz1] as uint)] as uint)] as uint;

    // Calculate the gradients.
    let nxy0: f64 = grad3(gi0, fx0, fy0, fz0);
    let nxy1: f64 = grad3(gi1, fx0, fy0, fz1);
    let nxy2: f64 = grad3(gi2, fx0, fy1, fz0);
    let nxy3: f64 = grad3(gi3, fx0, fy1, fz1);
    let nxy4: f64 = grad3(gi4, fx1, fy0, fz0);
    let nxy5: f64 = grad3(gi5, fx1, fy0, fz1);
    let nxy6: f64 = grad3(gi6, fx1, fy1, fz0);
    let nxy7: f64 = grad3(gi7, fx1, fy1, fz1);

    let nx0: f64 = lerp(r, nxy0, nxy1);
    let nx1: f64 = lerp(r, nxy2, nxy3);
    let nx2: f64 = lerp(r, nxy4, nxy5);
    let nx3: f64 = lerp(r, nxy6, nxy7);

    let n0: f64 = lerp(t, nx0, nx1);
    let n1: f64 = lerp(t, nx2, nx3);

    // The result is scaled to return values in the interval [-1, 1].
    0.936 * lerp(s, n0, n1)
  }
}
