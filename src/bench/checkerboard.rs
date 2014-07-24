use std::rand::{ weak_rng, Rng, XorShiftRng };
use test::Bencher;

use gen::{ NoiseGen, Checkerboard };

#[bench]
fn bench_checkerboard_new(b: &mut Bencher) {
    b.iter(|| {
        Checkerboard::new();
    })
}

#[bench]
fn bench_checkerboard_noise1d(b: &mut Bencher) {
    let mut rng: XorShiftRng = weak_rng();
    let checkerboard = Checkerboard::new();
    b.iter(|| {
        checkerboard.noise1d(rng.gen());
    })
}

#[bench]
fn bench_checkerboard_noise2d(b: &mut Bencher) {
    let mut rng: XorShiftRng = weak_rng();
    let checkerboard = Checkerboard::new();
    b.iter(|| {
        checkerboard.noise2d(
            rng.gen(),
            rng.gen()
        );
    })
}

#[bench]
fn bench_checkerboard_noise3d(b: &mut Bencher) {
    let mut rng: XorShiftRng = weak_rng();
    let checkerboard = Checkerboard::new();
    b.iter(|| {
        checkerboard.noise3d(
            rng.gen(),
            rng.gen(),
            rng.gen()
        );
    })
}
