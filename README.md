# noise [![Build Status](https://travis-ci.org/Cacteye/noise.svg?branch=master)](https://travis-ci.org/Cacteye/noise)

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
use noise::gen::{NoiseGen, Simplex};

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
* Perlin noise.
* Checkerboard.

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
