//! Outputs a [check pattern](http://en.wikipedia.org/wiki/Check_(pattern))

use utils::if_else;
use gen::NoiseGen;


/// A check pattern generator.
#[derive(Copy, Clone)]
pub struct Checkerboard;

impl Checkerboard {
    /// Initializes a new simplex instance with a random seed using XorShiftRng.
    ///
    /// # Example
    ///
    /// ```rust
    /// use noisy::gen::Checkerboard;
    ///
    /// let checkerboard = Checkerboard::new();
    /// ```
    pub fn new() -> Checkerboard {
        Checkerboard
    }
}

impl NoiseGen for Checkerboard {
    /// Given an x coordinate, return a value in the interval [-1, 1].
    ///
    /// # Example
    ///
    /// ```rust
    /// use noisy::gen::{NoiseGen, Checkerboard};
    ///
    /// let checkerboard = Checkerboard::new();
    /// let val = checkerboard.noise1d(1.0);
    /// ```
    fn noise1d(&self, xin: f64) -> f64 {
        let ix: i64 = xin.floor() as i64;

        if_else(ix & 1 == 1, -1.0, 1.0)
    }

    /// Given a (x, y) coordinate, return a value in the interval [-1, 1].
    ///
    /// # Example
    ///
    /// ```rust
    /// use noisy::gen::{NoiseGen, Checkerboard};
    ///
    /// let checkerboard = Checkerboard::new();
    /// let val = checkerboard.noise2d(1.0, 2.0);
    /// ```
    fn noise2d(&self, xin: f64, yin: f64) -> f64 {
        let ix: i64 = xin.floor() as i64;
        let iy: i64 = yin.floor() as i64;

        if_else(ix & 1 ^ iy & 1 == 1, -1.0, 1.0)
    }

    /// Given a (x, y, z) coordinate, return a value in the interval [-1, 1].
    ///
    /// # Example
    ///
    /// ```rust
    /// use noisy::gen::{NoiseGen, Checkerboard};
    ///
    /// let checkerboard = Checkerboard::new();
    /// let val = checkerboard.noise3d(1.0, 2.0, 3.0);
    /// ```
    fn noise3d(&self, xin: f64, yin: f64, zin: f64) -> f64 {
        let ix: i64 = xin.floor() as i64;
        let iy: i64 = yin.floor() as i64;
        let iz: i64 = zin.floor() as i64;

        if_else(ix & 1 ^ iy & 1 ^ iz & 1 == 1, -1.0, 1.0)
    }
}
