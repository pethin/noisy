/*!
# noisy

**noisy** is a procedural noise generation library written for Rust.

## Using **noisy**
All the generators of **noisy** are grouped in one place: the `gen` module.

* You can import all the generators using:

```ignore
use noisy::gen::*;
```

The preferred way to use **noisy** is to import generators explicitly:

```rust
extern crate noisy;
use noisy::gen::{NoiseGen, Simplex};

fn main() {
    let simplex = Simplex::new();

    let val = simplex.noise3d(1.0, 2.0, 3.0);
    println!("{}", val);
}
```

## Features
**noisy** is meant to be a general-purpose purpose procedural noise generation library that
includes a variety of generators including:

* Simplex noise.
* Imporoved Perlin noise.
* Perlin noise (not implemented).

## Compilation
You will need the last rust compiler from the master branch.
If you encounter problems, make sure you have the last version before creating an issue.

```ignore
git clone --recursive git://github.com/cacteye/noisy.git
cd noisy
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

#![warn(missing_docs)]
#![feature(macro_rules)]

extern crate rand;

#[cfg(test)]
extern crate test;

pub mod utils;
pub mod gen;

#[cfg(test)]
mod tests;

#[cfg(test)]
mod bench;
