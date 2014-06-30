use std::rand::{random};

use gen::{NoiseGen, Checkerboard};

#[test]
fn test_checkerboard_new() {
  Checkerboard::new();
}

#[test]
fn test_checkerboard_noise1d() {
  let checkerboard = Checkerboard::new();
  for _ in range(0u, 10000) {
    checkerboard.noise1d(random());
  }
}

#[test]
fn test_checkerboard_noise2d() {
  let checkerboard = Checkerboard::new();
  for _ in range(0u, 10000) {
    checkerboard.noise2d(
      random(),
      random()
    );
  }
}

#[test]
fn test_checkerboard_noise3d() {
  let checkerboard = Checkerboard::new();
  for _ in range(0u, 10000) {
    checkerboard.noise3d(
      random(),
      random(),
      random()
    );
  }
}
