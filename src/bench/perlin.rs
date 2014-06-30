use std::rand::{weak_rng, Rng, XorShiftRng};
use test::Bencher;

use gen::{NoiseGen, Perlin};

#[bench]
fn bench_perlin_new(b: &mut Bencher) {
  b.iter(|| {
    Perlin::new();
  })
}

#[bench]
fn bench_perlin_from_rng(b: &mut Bencher) {
  let mut rng: XorShiftRng = weak_rng();
  b.iter(|| {
    Perlin::from_rng(&mut rng);
  })
}

#[bench]
fn bench_perlin_noise1d(b: &mut Bencher) {
  let mut rng: XorShiftRng = weak_rng();
  let perlin = Perlin::from_rng(&mut rng);
  b.iter(|| {
    perlin.noise1d(rng.gen());
  })
}

#[bench]
fn bench_perlin_noise2d(b: &mut Bencher) {
  let mut rng: XorShiftRng = weak_rng();
  let perlin = Perlin::from_rng(&mut rng);
  b.iter(|| {
    perlin.noise2d(
      rng.gen(),
      rng.gen()
    );
  })
}

#[bench]
fn bench_perlin_noise3d(b: &mut Bencher) {
  let mut rng: XorShiftRng = weak_rng();
  let perlin = Perlin::from_rng(&mut rng);
  b.iter(|| {
    perlin.noise3d(
      rng.gen(),
      rng.gen(),
      rng.gen()
    );
  })
}
