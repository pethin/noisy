//! An implementation of Improved [Perlin Noise]
//! (https://en.wikipedia.org/wiki/Perlin_noise).
//!
//! Based on a improved perlin noise algorithm for 2D, 3D and 4D in C.
//! Which is based on example code by Ken Perlin at Siggraph 2002.
//! With optimisations by Stefan Gustavson (stegu@itn.liu.se).

use std::rand::{ Rng, XorShiftRng, weak_rng };

use utils::{ fade, fast_floor, lerp };
use utils::grad::{ grad1, grad2, grad3 };
use gen::NoiseGen;

/// A Perlin noise generator.
#[derive(Clone, PartialEq, Eq)]
pub struct Perlin {
    perm: Vec<u8>
}

impl Perlin {
    /// Initializes a new Perlin instance with a random seed using XorShiftRng.
    ///
    /// # Example
    ///
    /// ```rust
    /// use noisy::gen::Perlin;
    ///
    /// let perlin = Perlin::new();
    /// ```
    pub fn new() -> Perlin {
        let mut rng: XorShiftRng = weak_rng();

        let p: Vec<u8> = (0..256).map(|_| rng.gen::<u8>()).collect();
        let perm: Vec<u8> = (0..512).map(|idx:i32| {p[(idx & 255) as usize]}).collect();

        Perlin { perm: perm }
    }

    /// Initializes a new Perlin instance with a random number generator.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use std::rand::StdRng;
    /// use noisy::gen::Perlin;
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
    /// use noisy::gen::Perlin;
    ///
    /// let seed: &[_] = &[1337];
    /// let mut rng: StdRng = SeedableRng::from_seed(seed);
    /// let perlin = Perlin::from_rng(&mut rng);
    /// ```
    pub fn from_rng<R: Rng>(rng: &mut R) -> Perlin {
        let p: Vec<u8> = (0..256).map(|_| rng.gen::<u8>()).collect();
        let perm: Vec<u8> = (0..512).map(|idx:i32| {p[(idx & 255) as usize]}).collect();

        Perlin { perm: perm }
    }
}

impl NoiseGen for Perlin {
    /// Given an x coordinate, return a value in the interval [-1, 1].
    ///
    /// # Example
    ///
    /// ```rust
    /// use noisy::gen::{NoiseGen, Perlin};
    ///
    /// let perlin = Perlin::new();
    /// let val = perlin.noise1d(123.0 * 0.04);
    /// ```
    fn noise1d(&self, xin: f64) -> f64 {
        let ix0: i64 = fast_floor(xin); // Integer part of x
        let fx0: f64 = xin - ix0 as f64; // Fractional part of x
        let fx1: f64 = fx0 - 1.0;
        let ix1: i64 = ix0 + 1;

        // Wrap the integer indices at 256, to avoid indexing perm[] out of bounds
        let ii: usize = (ix0 & 255) as usize;
        let jj: usize = (ix1 & 255) as usize;

        // Compute the fade curve.
        let s: f64 = fade(fx0);

        // Work out the hashed gradient indices.
        let gi0: u8 = self.perm[ii] as u8;
        let gi1: u8 = self.perm[jj] as u8;

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
    /// use noisy::gen::{NoiseGen, Perlin};
    ///
    /// let perlin = Perlin::new();
    /// let val = perlin.noise2d(
    ///     123.0 * 0.04,
    ///     132.0 * 0.04
    /// );
    /// ```
    fn noise2d(&self, xin: f64, yin: f64) -> f64 {
        let ix0: i64 = fast_floor(xin); // Integer part of x
        let iy0: i64 = fast_floor(yin); // Integer part of y
        let fx0: f64 = xin - ix0 as f64; // Fractional part of x
        let fy0: f64 = yin - iy0 as f64; // Fractional part of y
        let fx1: f64 = fx0 - 1.0;
        let fy1: f64 = fy0 - 1.0;

        // Wrap the integer indices at 256, to avoid indexing perm[] out of bounds
        let ix1: usize = ((ix0 + 1) & 255) as usize;
        let iy1: usize = ((iy0 + 1) & 255) as usize;
        let ii: usize = (ix0 & 255) as usize;
        let jj: usize = (iy0 & 255) as usize;

        // Compute the fade curves.
        let t: f64 = fade(fy0);
        let s: f64 = fade(fx0);

        // Work out the hashed gradient indices.
        let gi0: u8 = self.perm[ii + (self.perm[jj] as usize)] as u8;
        let gi1: u8 = self.perm[ii + (self.perm[iy1] as usize)] as u8;
        let gi2: u8 = self.perm[ix1 + (self.perm[jj] as usize)] as u8;
        let gi3: u8 = self.perm[ix1 + (self.perm[iy1] as usize)] as u8;

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
    /// use noisy::gen::{NoiseGen, Perlin};
    ///
    /// let perlin = Perlin::new();
    /// let val = perlin.noise3d(
    ///     123.0 * 0.04,
    ///     231.0 * 0.04,
    ///     321.0 * 0.04
    /// );
    /// ```
    fn noise3d(&self, xin: f64, yin: f64, zin: f64) -> f64 {
        let ix0: i64 = fast_floor(xin); // Integer part of x
        let iy0: i64 = fast_floor(yin); // Integer part of y
        let iz0: i64 = fast_floor(zin); // Integer part of z
        let fx0: f64 = xin - ix0 as f64; // Fractional part of x
        let fy0: f64 = yin - iy0 as f64; // Fractional part of y
        let fz0: f64 = zin - iz0 as f64; // Fractional part of z
        let fx1: f64 = fx0 - 1.0;
        let fy1: f64 = fy0 - 1.0;
        let fz1: f64 = fz0 - 1.0;

        // Wrap the integer indices at 256, to avoid indexing perm[] out of bounds
        let ix1: usize = ((ix0 + 1) & 255) as usize;
        let iy1: usize = ((iy0 + 1) & 255) as usize;
        let iz1: usize = ((iz0 + 1) & 255) as usize;
        let ii: usize = (ix0 & 255) as usize;
        let jj: usize = (iy0 & 255) as usize;
        let kk: usize = (iz0 & 255) as usize;

        // Compute the fade curves.
        let r: f64 = fade(fz0);
        let t: f64 = fade(fy0);
        let s: f64 = fade(fx0);

        // Work out the hashed gradient indices.
        let gi0: u8 = self.perm[ii + (self.perm[jj + (self.perm[kk] as usize)] as usize)] as u8;
        let gi1: u8 = self.perm[ii + (self.perm[jj + (self.perm[iz1] as usize)] as usize)] as u8;
        let gi2: u8 = self.perm[ii + (self.perm[iy1 + (self.perm[kk] as usize)] as usize)] as u8;
        let gi3: u8 = self.perm[ii + (self.perm[iy1 + (self.perm[iz1] as usize)] as usize)] as u8;
        let gi4: u8 = self.perm[ix1 + (self.perm[jj + (self.perm[kk] as usize)] as usize)] as u8;
        let gi5: u8 = self.perm[ix1 + (self.perm[jj + (self.perm[iz1] as usize)] as usize)] as u8;
        let gi6: u8 = self.perm[ix1 + (self.perm[iy1 + (self.perm[kk] as usize)] as usize)] as u8;
        let gi7: u8 = self.perm[ix1 + (self.perm[iy1 + (self.perm[iz1] as usize)] as usize)] as u8;

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
