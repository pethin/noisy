extern crate noisy;

use noisy::gen::{ NoiseGen, Checkerboard };

// Width of the output in columns.
static WIDTH: int = 80;

// A five color gradient used for the output.
static GRADIENT: [&'static str; 2] = [" ", "█"];

fn main() {
    // Create a new simplex instance.
    let checkerboard = Checkerboard::new();

    // Iterate over the columnss.
    for x in range(-WIDTH/2, WIDTH/2) {
        // Generate a noise value using the x coordinate.
        let mut val = checkerboard.noise1d(x as f64 * 0.5);

        // Since the result is within [-1, 1], scale and offset the result to [0, 1].
        val = (val + 1.0) * 0.5;

        // Print the columns in the row.
        print!("{}", GRADIENT[val as uint]);
    }
    // Start a new row.
    println!("");
}
