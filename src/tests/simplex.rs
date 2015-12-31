use rand::{ thread_rng, random };
use rand::{ Rng, OsRng, StdRng, ThreadRng, IsaacRng, Isaac64Rng, XorShiftRng };

use gen::{NoiseGen, Simplex};

macro_rules! test_simplex_from_rng(
    ($t: ident) => ({
        let mut rng = match $t::new() {
            Ok(r) => r,
            Err(e) => panic!("Failed to create RNG: {}", e)
        };

        Simplex::from_rng(&mut rng);
    });
);

macro_rules! test_simplex_from_rand_rng(
    ($t: ty) => ({
        let mut rng: $t = match OsRng::new() {
            Ok(mut r) => r.gen(),
            Err(e) => panic!("Failed to create seeded RNG: {}", e)
        };

        Simplex::from_rng(&mut rng);
    });
);

#[test]
fn test_simplex_new() {
    Simplex::new();
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
fn test_simplex_from_threadrng() {
    let mut thread_rng: ThreadRng = thread_rng();

    Simplex::from_rng(&mut thread_rng);
}

#[test]
fn test_simplex_noise1d() {
    let simplex = Simplex::new();
    for _ in 0usize..10000 {
        simplex.noise1d(random());
    }
}

#[test]
fn test_simplex_noise2d() {
    let simplex = Simplex::new();
    for _ in 0usize..10000 {
        simplex.noise2d(
            random(),
            random()
        );
    }
}

#[test]
fn test_simplex_noise3d() {
    let simplex = Simplex::new();
    for _ in 0usize..10000 {
        simplex.noise3d(
            random(),
            random(),
            random()
        );
    }
}
