extern crate noisy;

use noisy::gen::{ NoiseGen, Checkerboard };

// Width of the output in columns.
static WIDTH: uint = 80;
// Height of the output in rows.
static HEIGHT: uint = 80;

// A five color gradient used for the output.
static GRADIENT: [&'static str, ..2] = [" ", "█"];

fn main() {
    // Create a new simplex instance.
    let checkerboard = Checkerboard::new();

    // Iterate over the rows.
    // HEIGHT is divided by two for a better aspect ratio.
    for y in range(0, HEIGHT / 2) {
        // Iterate over the columns in the rows.
        for x in range(0, WIDTH) {
            // Generate a noise value using the x and y coordinates.
            let mut val = checkerboard.noise2d(
                x as f64 * 0.5,
                y as f64 * 0.5
            );

            // Since the result is within [-1, 1],
            // scale and offset the result to [0, 1].
            val = (val + 1.0) * 0.5;

            // Print the columns in the row.
            print!("{}", GRADIENT[val as uint]);
        }
        // Start a new row.
        println!("");
    }
}
