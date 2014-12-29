//! An implementation of [Simplex Noise]
//! (https://en.wikipedia.org/wiki/Simplex_noise).
//!
//! Based on a speed-improved simplex noise algorithm for 2D, 3D and 4D in Java.
//! Which is based on example code by Stefan Gustavson (stegu@itn.liu.se).
//! With Optimisations by Peter Eastman (peastman@drizzle.stanford.edu).
//! Better rank ordering method by Stefan Gustavson in 2012.

use std::rand::{ Rng, XorShiftRng, weak_rng };

use utils::fast_floor;
use utils::grad::{ grad1, grad2, grad3 };
use gen::NoiseGen;

static F2: f64 = 0.366025403784_f64;
static G2: f64 = 0.211324865405_f64;
static F3: f64 = 0.333333333333_f64;
static G3: f64 = 0.166666666667_f64;

/// A simplex noise generator.
#[deriving(Clone, PartialEq, Eq)]
pub struct Simplex {
    perm: Vec<u8>
}

impl Simplex {
    /// Initializes a new simplex instance with a random seed using XorShiftRng.
    ///
    /// # Example
    ///
    /// ```rust
    /// use noisy::gen::Simplex;
    ///
    /// let simplex = Simplex::new();
    /// ```
    pub fn new() -> Simplex {
        let mut rng: XorShiftRng = weak_rng();

        let p: Vec<u8> = Vec::from_fn(256, |_| rng.gen::<u8>());
        let perm: Vec<u8> = Vec::from_fn(512, |idx| p[idx & 255]);

        Simplex { perm: perm }
    }

    /// Initializes a new simplex instance with a random number generator.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use std::rand::StdRng;
    /// use noisy::gen::Simplex;
    ///
    /// let mut rng: StdRng = StdRng::new().unwrap();
    /// let simplex = Simplex::from_rng(&mut rng);
    /// ```
    ///
    /// This also allows you to initialize the instance with a seed:
    ///
    /// # Example
    ///
    /// ```rust
    /// # use std::rand::{StdRng, SeedableRng};
    /// use noisy::gen::Simplex;
    ///
    /// let seed: &[_] = &[1337];
    /// let mut rng: StdRng = SeedableRng::from_seed(seed);
    /// let simplex = Simplex::from_rng(&mut rng);
    /// ```
    pub fn from_rng<R: Rng>(rng: &mut R) -> Simplex {
        let p: Vec<u8> = Vec::from_fn(256, |_| rng.gen::<u8>());
        let perm: Vec<u8> = Vec::from_fn(512, |idx| p[idx & 255]);

        Simplex { perm: perm }
    }
}

impl NoiseGen for Simplex {
    /// Given an x coordinate, return a value in the interval [-1, 1].
    ///
    /// # Example
    ///
    /// ```rust
    /// use noisy::gen::{NoiseGen, Simplex};
    ///
    /// let simplex = Simplex::new();
    /// let val = simplex.noise1d(123.0 * 0.02);
    /// ```
    #[allow(non_snake_case)]
    fn noise1d(&self, xin: f64) -> f64 {
        // Noise contributions
        let mut n0: f64;
        let mut n1: f64;

        let i0: int = fast_floor(xin);
        let i1: int = i0 + 1;
        let x0: f64 = xin - i0 as f64;
        let x1: f64 = x0 - 1.0;

        // Work out the hashed gradient indices
        let gi0: uint = self.perm[(i0 & 255) as uint] as uint;
        let gi1: uint = self.perm[(i1 & 255) as uint] as uint;

        // Calculate the contributions
        let mut t0: f64 = 1.0 - x0 * x0;
        t0 *= t0;
        n0 = t0 * t0 * grad1(gi0, x0);

        let mut t1: f64 = 1.0 - x1 * x1;
        t1 *= t1;
        n1 = t1 * t1 * grad1(gi1, x1);

        // The maximum value of this noise is 8*(3/4)^4 = 2.53125.
        // A factor of 0.395 scales to fit exactly within [-1,1].
        0.395 * (n0 + n1)
    }

    /// Given a (x, y) coordinate, return a value in the interval [-1, 1].
    ///
    /// # Example
    ///
    /// ```rust
    /// use noisy::gen::{NoiseGen, Simplex};
    ///
    /// let simplex = Simplex::new();
    /// let val = simplex.noise2d(
    ///     123.0 * 0.02,
    ///     132.0 * 0.02
    /// );
    /// ```
    #[allow(non_snake_case)]
    fn noise2d(&self, xin: f64, yin: f64) -> f64 {
        // Noise contributions from the three corners
        let mut n0: f64;
        let mut n1: f64;
        let mut n2: f64;

        // Skew the input space to determine which simplex cell we're in
        let s: f64 = (xin + yin) * F2; // Hairy factor for 2D
        let i: int = fast_floor(xin + s);
        let j: int = fast_floor(yin + s);
        let t: f64 = ((i + j) as f64) * G2;

        // Unskew the cell origin back to (x, y) space
        let X0: f64 = (i as f64) - t;
        let Y0: f64 = (j as f64) - t;
        // The x and y distances from the cell origin
        let x0: f64 = xin - X0;
        let y0: f64 = yin - Y0;

        // For the 2D case, the simplex shape is an equilateral triangle.
        // Determine which shape we are in.
        let i1: uint; // Offsets for second (middle) corner of simplex in (i, j) coords
        let j1: uint;
        if x0 > y0 { // Lower triangle, XY order: (0, 0) -> (1, 0) -> (1, 1)
            i1 = 1;
            j1 = 0;
        } else { // Upper triangle, YX order: (0, 0) -> (0, 1) -> (1, 1)
            i1 = 0;
            j1 = 1;
        }

        // A step of (1, 0) in (i, j) means a step of (1 - c, -c) in (x, y), and
        // a step of (0, 1) in (i, j) means a step of (-c, 1 - c) in (x, y), where
        // c = (3 - sqrt(3.0))/6.

        // Offsets for middle corner in (x,y) unskewed coords
        let x1: f64 = x0 - (i1 as f64) + G2;
        let y1: f64 = y0 - (j1 as f64) + G2;
        // Offsets for last corner in (x,y) unskewed coords
        let x2: f64 = x0 - 1.0 + 2.0 * G2;
        let y2: f64 = y0 - 1.0 + 2.0 * G2;

        // Wrap the integer indices at 256, to avoid indexing perm[] out of bounds
        let ii: uint = (i & 255) as uint;
        let jj: uint = (j & 255) as uint;
        // Work out the hashed gradient indices of the three simplex corners
        let gi0: uint = self.perm[ii + self.perm[jj] as uint] as uint;
        let gi1: uint = self.perm[ii + i1 + (self.perm[jj + j1] as uint)] as uint;
        let gi2: uint = self.perm[ii + 1 + (self.perm[jj + 1] as uint)] as uint;

        // Calculate the contribution from the three corners
        let mut t0: f64 = 0.5 - x0 * x0 - y0 * y0;
        if t0 < 0.0 {
            n0 = 0.0;
        } else {
            t0 *= t0;
            n0 = t0 * t0 * grad2(gi0, x0, y0);
        }

        let mut t1: f64 = 0.5 - x1 * x1 - y1 * y1;
        if t1 < 0.0 {
            n1 = 0.0;
        } else {
            t1 *= t1;
            n1 = t1 * t1 * grad2(gi1, x1, y1);
        }

        let mut t2: f64 = 0.5 - x2 * x2 - y2 * y2;
        if t2 < 0.0 {
            n2 = 0.0;
        } else {
            t2 *= t2;
            n2 = t2 * t2 * grad2(gi2, x2, y2);
        }

        // Add contributions from each corner to get the final noise value.
        // The result is scaled to return values in the interval [-1, 1].
        40.0 * (n0 + n1 + n2)
    }

    /// Given a (x, y, z) coordinate, return a value in the interval [-1, 1].
    ///
    /// # Example
    ///
    /// ```rust
    /// use noisy::gen::{NoiseGen, Simplex};
    ///
    /// let simplex = Simplex::new();
    /// let val = simplex.noise3d(
    ///     123.0 * 0.02,
    ///     231.0 * 0.02,
    ///     321.0 * 0.02
    /// );
    /// ```
    #[allow(non_snake_case)]
    fn noise3d(&self, xin: f64, yin: f64, zin: f64) -> f64 {
        // Noise contributions from the four corners
        let mut n0: f64;
        let mut n1: f64;
        let mut n2: f64;
        let mut n3: f64;

        // Skew the input space to determine which simplex cell we're in
        let s: f64 = (xin + yin + zin) * F3; // Very nice and simple skew factor for 3D
        let i: int = fast_floor(xin + s);
        let j: int = fast_floor(yin + s);
        let k: int = fast_floor(zin + s);
        let t: f64 = ((i + j + k) as f64) * G3;

        // Unskew the cell origin back to (x, y, z) space
        let X0: f64 = (i as f64) - t;
        let Y0: f64 = (j as f64) - t;
        let Z0: f64 = (k as f64) - t;
        // The x, y, and distances from the cell origin
        let x0: f64 = xin - X0;
        let y0: f64 = yin - Y0;
        let z0: f64 = zin - Z0;

        // For the 3D case, the simplex shape is a slightly irregular tetrahedron.
        // Determine which simplex we are in.
        let i1: uint; // Offsets for second corner of simplex in (i, j, k) coords
        let j1: uint;
        let k1: uint;
        let i2: uint; // Offsets for third corner of simplex in (i, j, k) coords
        let j2: uint;
        let k2: uint;
        if x0 >= y0 {
            if y0 >= z0 { // X Y Z order
                i1 = 1;
                j1 = 0;
                k1 = 0;
                i2 = 1;
                j2 = 1;
                k2 = 0;
            } else if x0 >= z0 { // X Z Y order
                i1 = 1;
                j1 = 0;
                k1 = 0;
                i2 = 1;
                j2 = 0;
                k2 = 1;
            } else {  // Z X Y order
                i1 = 0;
                j1 = 0;
                k1 = 1;
                i2 = 1;
                j2 = 0;
                k2 = 1;
            }
        } else { // x0 < y0
            if y0 < z0 { // Z Y X order
                i1 = 0;
                j1 = 0;
                k1 = 1;
                i2 = 0;
                j2 = 1;
                k2 = 1;
            } else if x0 < z0 { // Y Z X order
                i1 = 0;
                j1 = 1;
                k1 = 0;
                i2 = 0;
                j2 = 1;
                k2 = 1;
            } else { // Y X Z order
                i1 = 0;
                j1 = 1;
                k1 = 0;
                i2 = 1;
                j2 = 1;
                k2 = 0;
            }
        }

        // A step of (1, 0, 0) in (i, j, k) means a step of (1 - c, -c, -c) in (x, y, z),
        // a step of (0, 1, 0) in (i, j, k) means a step of (-c, 1 - c, -c) in (x, y, z), and
        // a step of (0, 0, 1) in (i, j, k) means a step of (-c, -c, 1 - c) in (x, y, z), where
        // c = 1/6.

        // Offsets for second corner in (x, y, z) coords
        let x1: f64 = x0 - (i1 as f64) + G3;
        let y1: f64 = y0 - (j1 as f64) + G3;
        let z1: f64 = z0 - (k1 as f64) + G3;
        // Offsets for third corner in (x, y, z) coords
        let x2: f64 = x0 - (i2 as f64) + 2.0 * G3;
        let y2: f64 = y0 - (j2 as f64) + 2.0 * G3;
        let z2: f64 = z0 - (k2 as f64) + 2.0 * G3;
        // Offsets for last corner in (x, y, z) coords
        let x3: f64 = x0 - 1.0 + 3.0 * G3;
        let y3: f64 = y0 - 1.0 + 3.0 * G3;
        let z3: f64 = z0 - 1.0 + 3.0 * G3;

        // Wrap the integer indices at 256, to avoid indexing perm[] out of bounds
        let ii: uint = (i & 255) as uint;
        let jj: uint = (j & 255) as uint;
        let kk: uint = (k & 255) as uint;
        // Work out the hashed gradient indices of the four simplex corners
        let gi0: uint = self.perm[ii + (self.perm[jj + (self.perm[kk] as uint)] as uint)] as uint;
        let gi1: uint = self.perm[ii + i1 + (self.perm[jj + j1 + (self.perm[kk + k1] as uint)] as uint)] as uint;
        let gi2: uint = self.perm[ii + i2 + (self.perm[jj + j2 + (self.perm[kk + k2] as uint)] as uint)] as uint;
        let gi3: uint = self.perm[ii + 1 + (self.perm[jj + 1 + (self.perm[kk + 1] as uint)] as uint)] as uint;

        // Calculate the contribution from the four corners
        let mut t0: f64 = 0.6 - x0 * x0 - y0 * y0 - z0 * z0;
        if t0 < 0.0 {
            n0 = 0.0;
        } else {
            t0 *= t0;
            n0 = t0 * t0 * grad3(gi0, x0, y0, z0);
        }

        let mut t1: f64 = 0.6 - x1 * x1 - y1 * y1 - z1 * z1;
        if t1 < 0.0 {
            n1 = 0.0;
        } else {
            t1 *= t1;
            n1 = t1 * t1 * grad3(gi1, x1, y1, z1);
        }

        let mut t2: f64 = 0.6 - x2 * x2 - y2 * y2 - z2 * z2;
        if t2 < 0.0 {
            n2 = 0.0;
        } else {
            t2 *= t2;
            n2 = t2 * t2 * grad3(gi2, x2, y2, z2);
        }

        let mut t3: f64 = 0.6 - x3 * x3 - y3 * y3 - z3 * z3;
        if t3 < 0.0 {
            n3 = 0.0;
        } else {
            t3 *= t3;
            n3 = t3 * t3 * grad3(gi3, x3, y3, z3);
        }

        // Add contributions from each corner to get the final noise value.
        // The result is scaled to return values in the interval [-1,1].
        32.0 * (n0 + n1 + n2 + n3)
    }
}
