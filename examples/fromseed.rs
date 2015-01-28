extern crate noisy;

use std::rand::{ XorShiftRng, SeedableRng };

use noisy::gen::{ NoiseGen, Simplex };

// Seed used to create a random number generator.
static SEED: [u32; 4] = [9123678, 7890124, 6123462, 2789431];
// Width of the output in columns.
static WIDTH: uint = 80;
// Height of the output in rows.
static HEIGHT: uint = 80;

// A five color gradient used for the output.
static GRADIENT: [&'static str; 5] = [" ", "░", "▒", "▓", "█"];

fn main() {
    // Create a new XorShiftRng using the seed.
    let mut rng: XorShiftRng = SeedableRng::from_seed(SEED);
    // Create a new simplex instance from the seeded Rng.
    let simplex = Simplex::from_rng(&mut rng);

    // Iterate over the rows.
    // HEIGHT is divided by two for a better aspect ratio.
    for y in range(0, HEIGHT / 2) {
        // Iterate over the columns in the rows.
        for x in range(0, WIDTH) {
            // Generate a noise value using the x and y coordinates.
            let mut val = simplex.noise2d(
                123.0 + x as f64 * 0.02,
                132.0 + y as f64 * 0.02
            );

            // Since the result is within [-1, 1], scale and offset the result to [0, 1].
            val = (val + 1.0) * 0.5;

            // Apply the result to the 5 color gradient.
            val = val * 5.0;

            // Print the columns in the row.
            print!("{}", GRADIENT[val as uint]);
        }
        // Start a new row.
        println!("");
    }
}
