Interpolate
========

A library for interpolating between values.

This library is a work in progress, it currently supports:

* Linear interpolation
* Bilinear interpolation

## Usage

### Linear interpolation
Basic example of linear interpolation for primitive types:

```rust
extern crate interpolate;
use interpolate::Interpolate;

fn main() {
    let start = 0.0;
    let end = 100.0;

    println!("{}", f32::lerp([start, end], 0.5));
}
```

An example of usage for a custom type:

```rust
extern crate interpolate;
use interpolate::Interpolate;

#[derive(Clone, Copy)]
struct Rgb {
    r: u8,
    g: u8,
    b: u8,
}

impl Interpolate for Rgb {
    fn lerp(v: [Rgb; 2], x: f64) -> Rgb {
        Rgb {
            r: u8::lerp([v[0].r, v[1].r], x),
            g: u8::lerp([v[0].g, v[1].g], x),
            b: u8::lerp([v[0].b, v[1].b], x)
        }
    }
}

fn main() {
    let start = Rgb { r: 0, g: 255, b: 0 };
    let end = Rgb { r: 100, g: 0, b: 200 };

    let blended_color = Rgb::lerp([start, end], 0.5);
}
```