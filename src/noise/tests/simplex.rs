use std::rand::{task_rng, random};
use std::rand::{Rng, OsRng, StdRng, TaskRng, IsaacRng, Isaac64Rng, XorShiftRng};

use NoiseGen;
use gen::Simplex;

macro_rules! test_simplex_from_rng(
  ($t: ident) => (
    for _ in range(0, 10) {
      let mut rng = match $t::new() {
        Ok(r) => r,
        Err(e) => fail!("Failed to create RNG: {}", e)
      };

      Simplex::from_rng(&mut rng);
    }
  );
)

macro_rules! test_simplex_from_rand_rng(
  ($t: ty) => (
    for _ in range(0, 10000) {
      let mut rng: $t = match OsRng::new() {
        Ok(mut r) => r.gen(),
        Err(e) => fail!("Failed to create seeded RNG: {}", e)
      };

      Simplex::from_rng(&mut rng);
    }
  );
)

#[test]
fn test_simplex_new() {
  for _ in range(0, 10000) {
    let simplex1 = Simplex::new();
    let simplex2 = Simplex::new();

    assert!(simplex1 != simplex2);
  }
}

#[test]
fn test_simplex_from_osrng() {
  test_simplex_from_rng!(OsRng);
}

#[test]
fn test_simplex_from_stdrng() {
  test_simplex_from_rng!(StdRng);
}

#[test]
fn test_simplex_from_isaacrng() {
  test_simplex_from_rand_rng!(IsaacRng);
}

#[test]
fn test_simplex_from_isaac64rng() {
  test_simplex_from_rand_rng!(Isaac64Rng);
}

#[test]
fn test_simplex_from_xorshiftrng() {
  test_simplex_from_rand_rng!(XorShiftRng);
}

#[test]
fn test_simplex_from_taskrng() {
  for _ in range(0, 10) {
    let mut taskRng: TaskRng = task_rng();

    Simplex::from_rng(&mut taskRng);
  }
}

#[test]
fn test_simplex_noise2d() {
  let simplex = Simplex::new();
  for _ in range(0, 10000) {
    simplex.noise2d(
      random(),
      random()
    );
  }
}

#[test]
fn test_simplex_noise3d() {
  let simplex = Simplex::new();
  for _ in range(0, 10000) {
    simplex.noise3d(
      random(),
      random(),
      random()
    );
  }
}
