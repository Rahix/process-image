`process-image` [![crates.io page](https://img.shields.io/crates/v/process-image)](https://crates.io/crates/process-image) [![docs.rs](https://docs.rs/process-image/badge.svg)](https://docs.rs/process-image)
===============
`process-image` is a Rust crate for conveniently accessing data inside a
_process image_ using zero-cost abstractions.  A _process image_ (short PI) is
nothing else than a block of memory describing state.  This concept stems from
the industrial automation world where such process images are used to represent
the state of all inputs and outputs used to control a machine or process.

`process-image` provides abstractions for absolute addressing of tags (=values)
and for building tag tables that then allow symbolic access.

## Example
#### Absolute Addressing
```rust
use process_image as pi;
let mut buf = [0x00; 8];

// Absolute tag addressing
let sensor_limit_1 = pi::tag!(&mut buf, X, 5, 6);  // Read %MX3.6
let sensor_limit_2 = pi::tag!(&mut buf, X, 5, 7);  // Read %MX3.7
let temperature: u16 = pi::tag!(&mut buf, W, 2); // Read %MW2

*pi::tag_mut!(&mut buf, X, 0, 2) = true;  // Set %MX0.2 := TRUE;
*pi::tag_mut!(&mut buf, W, 6) = 2300;  // Set %MW6 := 2300;
```

#### Symbolic Addressing
```rust
use process_image as pi;
let mut buf = [0x00; 8];

// Build tag table definition
pi::process_image! {
    // Process Image over 8 bytes
    pub struct PiExample, mut PiExampleMut: 8 {
        pub indicator_light: (X, 0, 2),
        pub sensor_limit_1: (X, 5, 6),
        pub sensor_limit_2: (X, 5, 7),
        pub temperature: (W, 2),
        pub setpoint: (W, 6),
    }
}

// Read-access only
let pi = PiExample::try_from(&buf).unwrap();
dbg!(pi.sensor_limit_1());
dbg!(pi.sensor_limit_2());
dbg!(pi.temperature());

// Read-write access
let mut pi = PiExampleMut::try_from(&mut buf).unwrap();
if *pi.sensor_limit_1() {
  *pi.indicator_light() = true;
  *pi.setpoint() = 2300;
}
```

## License
Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or
  <http://opensource.org/licenses/MIT>)

at your option.

### Contribution
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
