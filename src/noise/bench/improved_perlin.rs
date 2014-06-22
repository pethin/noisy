use std::rand::{weak_rng, Rng, XorShiftRng};
use test::Bencher;

use gen::{NoiseGen, ImprovedPerlin};

#[bench]
fn bench_improved_perlin_new(b: &mut Bencher) {
  b.iter(|| {
    ImprovedPerlin::new();
  })
}

#[bench]
fn bench_improved_perlin_from_rng(b: &mut Bencher) {
  let mut rng: XorShiftRng = weak_rng();
  b.iter(|| {
    ImprovedPerlin::from_rng(&mut rng);
  })
}

#[bench]
fn bench_improved_perlin_noise1d(b: &mut Bencher) {
  let mut rng: XorShiftRng = weak_rng();
  let improved_perlin = ImprovedPerlin::from_rng(&mut rng);
  b.iter(|| {
    improved_perlin.noise1d(rng.gen());
  })
}

#[bench]
fn bench_improved_perlin_noise2d(b: &mut Bencher) {
  let mut rng: XorShiftRng = weak_rng();
  let improved_perlin = ImprovedPerlin::from_rng(&mut rng);
  b.iter(|| {
    improved_perlin.noise2d(
      rng.gen(),
      rng.gen()
    );
  })
}

#[bench]
fn bench_improved_perlin_noise3d(b: &mut Bencher) {
  let mut rng: XorShiftRng = weak_rng();
  let improved_perlin = ImprovedPerlin::from_rng(&mut rng);
  b.iter(|| {
    improved_perlin.noise3d(
      rng.gen(),
      rng.gen(),
      rng.gen()
    );
  })
}
