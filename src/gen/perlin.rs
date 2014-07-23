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

/// A Perlin noise generator.
#[deriving(Clone, PartialEq, Eq)]
pub struct Perlin {
  perm: Vec<u8>
}

impl Perlin {
  /// Initializes a new Perlin instance with a random seed using XorShiftRng.
  ///
  /// # Example
  ///
  /// ```rust
  /// use noise::gen::Perlin;
  ///
  /// let perlin = Perlin::new();
  /// ```
  pub fn new() -> Perlin {
    let mut rng: XorShiftRng = weak_rng();

    let p: Vec<u8> = Vec::from_fn(256, |_| rng.gen::<u8>());
    let perm: Vec<u8> = Vec::from_fn(512, |idx| p[idx & 255]);

    Perlin {
      perm: perm
    }
  }

  /// Initializes a new Perlin instance with a random number generator.
  ///
  /// # Example
  ///
  /// ```rust
  /// # use std::rand::StdRng;
  /// use noise::gen::Perlin;
  ///
  /// let mut rng: StdRng = StdRng::new().unwrap();
  /// let perlin = Perlin::from_rng(&mut rng);
  /// ```
  ///
  /// This also allows you to initialize the instance with a seed:
  ///
  /// # Example
  ///
  /// ```rust
  /// # use std::rand::{StdRng, SeedableRng};
  /// use noise::gen::Perlin;
  ///
  /// let mut rng: StdRng = SeedableRng::from_seed(&[1337]);
  /// let perlin = Perlin::from_rng(&mut rng);
  /// ```
  pub fn from_rng<R: Rng>(rng: &mut R) -> Perlin {
    let p: Vec<u8> = Vec::from_fn(256, |_| rng.gen::<u8>());
    let perm: Vec<u8> = Vec::from_fn(512, |idx| p[idx & 255]);

    Perlin {
      perm: perm
    }
  }
}

impl NoiseGen for Perlin {
  /// Given an x coordinate, return a value in the interval [-1, 1].
  ///
  /// # Example
  ///
  /// ```rust
  /// use noise::gen::{NoiseGen, Perlin};
  ///
  /// let perlin = Perlin::new();
  /// let val = perlin.noise1d(123.0 * 0.04);
  /// ```
  fn noise1d(&self, xin: f64) -> f64 {
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
    let gi0: uint = self.perm[ii] as uint;
    let gi1: uint = self.perm[jj] as uint;

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
  /// use noise::gen::{NoiseGen, Perlin};
  ///
  /// let perlin = Perlin::new();
  /// let val = perlin.noise2d(
  ///   123.0 * 0.04,
  ///   132.0 * 0.04
  /// );
  /// ```
  fn noise2d(&self, xin: f64, yin: f64) -> f64 {
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
    let gi0: uint = self.perm[ii + (self.perm[jj] as uint)] as uint;
    let gi1: uint = self.perm[ii + (self.perm[iy1] as uint)] as uint;
    let gi2: uint = self.perm[ix1 + (self.perm[jj] as uint)] as uint;
    let gi3: uint = self.perm[ix1 + (self.perm[iy1] as uint)] as uint;

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
  /// use noise::gen::{NoiseGen, Perlin};
  ///
  /// let perlin = Perlin::new();
  /// let val = perlin.noise3d(
  ///   123.0 * 0.04,
  ///   231.0 * 0.04,
  ///   321.0 * 0.04
  /// );
  /// ```
  fn noise3d(&self, xin: f64, yin: f64, zin: f64) -> f64 {
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
    let gi0: uint = self.perm[ii + (self.perm[jj + (self.perm[kk] as uint)] as uint)] as uint;
    let gi1: uint = self.perm[ii + (self.perm[jj + (self.perm[iz1] as uint)] as uint)] as uint;
    let gi2: uint = self.perm[ii + (self.perm[iy1 + (self.perm[kk] as uint)] as uint)] as uint;
    let gi3: uint = self.perm[ii + (self.perm[iy1 + (self.perm[iz1] as uint)] as uint)] as uint;
    let gi4: uint = self.perm[ix1 + (self.perm[jj + (self.perm[kk] as uint)] as uint)] as uint;
    let gi5: uint = self.perm[ix1 + (self.perm[jj + (self.perm[iz1] as uint)] as uint)] as uint;
    let gi6: uint = self.perm[ix1 + (self.perm[iy1 + (self.perm[kk] as uint)] as uint)] as uint;
    let gi7: uint = self.perm[ix1 + (self.perm[iy1 + (self.perm[iz1] as uint)] as uint)] as uint;

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
