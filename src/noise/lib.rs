/*!
# noise

**noise** is a procedural noise generation library written for Rust.

## Using **noise**
All the generators of **noise** are grouped in one place: the `gen` module.

* You can import all the generators using:

```ignore
use noise::gen::*;
```

The preferred way to use **noise** is to import generators explicitly:

```rust
extern crate noise;
use noise::NoiseGen;
use noise::gen::Simplex;

fn main() {
    let simplex = Simplex::new();

    let val = simplex.noise3d(1.0, 2.0, 3.0);
    println!("{}", val);
}
```

## Features
**noise** is meant to be a general-purpose purpose procedural noise generation library that
includes a variety of generators including:

* Simplex noise.
* Imporoved Perlin noise (not implemented).
* Perlin noise (not implemented).

## Compilation
You will need the last rust compiler from the master branch.
If you encounter problems, make sure you have the last version before creating an issue.

```ignore
git clone --recursive git://github.com/cacteye/noise.git
cd noise
make deps
make
```

You can build the documentation using:

```ignore
make doc
```

You can build the included examples using:

```ignore
make examples
```
*/

#![crate_id = "noise#0.1"]
#![crate_type = "lib"]
#![warn(missing_doc)]
#![feature(macro_rules)]

extern crate rand;
extern crate nalgebra;

#[cfg(test)]
extern crate test;

/// Procedural noise generators.
pub mod gen {
  pub use self::simplex::Simplex;

  pub mod simplex;
}

/// A procedural noise generator.
pub trait NoiseGen {
  /// For a given x coordinate, return a value between -1 and 1.
  fn noise1d(&self, xin: f64) -> f64;

  /// For a given (x, y) coordinate, return a value between -1 and 1.
  fn noise2d(&self, xin: f64, yin: f64) -> f64;

  /// For a given (x, y, z) coordinate, return a value between -1 and 1.
  fn noise3d(&self, xin: f64, yin: f64, zin: f64) -> f64;
}

#[cfg(test)]
mod tests {
  mod simplex;
}

#[cfg(test)]
mod bench {
  mod simplex;
}
