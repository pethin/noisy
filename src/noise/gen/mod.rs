//! Procedural noise generators.

pub use gen::simplex::Simplex;
pub use gen::perlin::Perlin;

mod simplex;
mod perlin;

/// A procedural noise generator.
pub trait NoiseGen {
  /// For a given x coordinate, return a value between -1 and 1.
  fn noise1d(&self, xin: f64) -> f64;

  /// For a given (x, y) coordinate, return a value between -1 and 1.
  fn noise2d(&self, xin: f64, yin: f64) -> f64;

  /// For a given (x, y, z) coordinate, return a value between -1 and 1.
  fn noise3d(&self, xin: f64, yin: f64, zin: f64) -> f64;
}
