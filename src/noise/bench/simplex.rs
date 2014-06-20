use std::rand::{weak_rng, Rng, XorShiftRng};
use test::Bencher;

use NoiseGen;
use gen::Simplex;

#[bench]
fn bench_simplex_new(b: &mut Bencher) {
  b.iter(|| {
    Simplex::new();
  })
}

#[bench]
fn bench_simplex_from_rng(b: &mut Bencher) {
  let mut rng: XorShiftRng = weak_rng();
  b.iter(|| {
    Simplex::from_rng(&mut rng);
  })
}

#[bench]
fn bench_simplex_noise2d(b: &mut Bencher) {
  let mut rng: XorShiftRng = weak_rng();
  let simplex = Simplex::from_rng(&mut rng);
  b.iter(|| {
    simplex.noise2d(
      rng.gen(),
      rng.gen()
    );
  })
}

#[bench]
fn bench_simplex_noise3d(b: &mut Bencher) {
  let mut rng: XorShiftRng = weak_rng();
  let simplex = Simplex::from_rng(&mut rng);
  b.iter(|| {
    simplex.noise3d(
      rng.gen(),
      rng.gen(),
      rng.gen()
    );
  })
}
