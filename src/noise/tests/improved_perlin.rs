use std::rand::{task_rng, random};
use std::rand::{Rng, OsRng, StdRng, TaskRng, IsaacRng, Isaac64Rng, XorShiftRng};

use gen::{NoiseGen, ImprovedPerlin};

macro_rules! test_improved_perlin_from_rng(
  ($t: ident) => ({
    let mut rng = match $t::new() {
      Ok(r) => r,
      Err(e) => fail!("Failed to create RNG: {}", e)
    };

    ImprovedPerlin::from_rng(&mut rng);
  });
)

macro_rules! test_improved_perlin_from_rand_rng(
  ($t: ty) => ({
    let mut rng: $t = match OsRng::new() {
      Ok(mut r) => r.gen(),
      Err(e) => fail!("Failed to create seeded RNG: {}", e)
    };

    ImprovedPerlin::from_rng(&mut rng);
  });
)

#[test]
fn test_improved_perlin_new() {
  ImprovedPerlin::new();
}

#[test]
fn test_improved_perlin_from_osrng() {
  test_improved_perlin_from_rng!(OsRng);
}

#[test]
fn test_improved_perlin_from_stdrng() {
  test_improved_perlin_from_rng!(StdRng);
}

#[test]
fn test_improved_perlin_from_isaacrng() {
  test_improved_perlin_from_rand_rng!(IsaacRng);
}

#[test]
fn test_improved_perlin_from_isaac64rng() {
  test_improved_perlin_from_rand_rng!(Isaac64Rng);
}

#[test]
fn test_improved_perlin_from_xorshiftrng() {
  test_improved_perlin_from_rand_rng!(XorShiftRng);
}

#[test]
fn test_improved_perlin_from_taskrng() {
  let mut taskRng: TaskRng = task_rng();

  ImprovedPerlin::from_rng(&mut taskRng);
}

#[test]
fn test_improved_perlin_noise1d() {
  let improved_perlin = ImprovedPerlin::new();
  for _ in range(0, 10000) {
    improved_perlin.noise1d(random());
  }
}

#[test]
fn test_improved_perlin_noise2d() {
  let improved_perlin = ImprovedPerlin::new();
  for _ in range(0, 10000) {
    improved_perlin.noise2d(
      random(),
      random()
    );
  }
}

#[test]
fn test_improved_perlin_noise3d() {
  let improved_perlin = ImprovedPerlin::new();
  for _ in range(0, 10000) {
    improved_perlin.noise3d(
      random(),
      random(),
      random()
    );
  }
}
