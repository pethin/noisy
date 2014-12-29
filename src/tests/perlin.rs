use std::rand::{ task_rng, random };
use std::rand::{ Rng, OsRng, StdRng, TaskRng, IsaacRng, Isaac64Rng, XorShiftRng };

use gen::{NoiseGen, Perlin};

macro_rules! test_perlin_from_rng(
    ($t: ident) => ({
        let mut rng = match $t::new() {
            Ok(r) => r,
            Err(e) => panic!("Failed to create RNG: {}", e)
        };

        Perlin::from_rng(&mut rng);
    });
);

macro_rules! test_perlin_from_rand_rng(
    ($t: ty) => ({
        let mut rng: $t = match OsRng::new() {
            Ok(mut r) => r.gen(),
            Err(e) => panic!("Failed to create seeded RNG: {}", e)
        };

        Perlin::from_rng(&mut rng);
    });
);

#[test]
fn test_perlin_new() {
    Perlin::new();
}

#[test]
fn test_perlin_from_osrng() {
    test_perlin_from_rng!(OsRng);
}

#[test]
fn test_perlin_from_stdrng() {
    test_perlin_from_rng!(StdRng);
}

#[test]
fn test_perlin_from_isaacrng() {
    test_perlin_from_rand_rng!(IsaacRng);
}

#[test]
fn test_perlin_from_isaac64rng() {
    test_perlin_from_rand_rng!(Isaac64Rng);
}

#[test]
fn test_perlin_from_xorshiftrng() {
    test_perlin_from_rand_rng!(XorShiftRng);
}

#[test]
fn test_perlin_from_taskrng() {
    let mut task_rng: TaskRng = task_rng();

    Perlin::from_rng(&mut task_rng);
}

#[test]
fn test_perlin_noise1d() {
    let perlin = Perlin::new();
    for _ in range(0u, 10000) {
        perlin.noise1d(random());
    }
}

#[test]
fn test_perlin_noise2d() {
    let perlin = Perlin::new();
    for _ in range(0u, 10000) {
        perlin.noise2d(
            random(),
            random()
        );
    }
}

#[test]
fn test_perlin_noise3d() {
    let perlin = Perlin::new();
    for _ in range(0u, 10000) {
        perlin.noise3d(
            random(),
            random(),
            random()
        );
    }
}
