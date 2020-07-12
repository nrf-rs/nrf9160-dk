# Board Support Package for the Nordic nRF9160-DK

This crate is a Board Support Package (BSP). It wraps the HAL crate
(`nrf9160-hal`) for the on-board nRF9160, and provides high level wrappers for
the onboard features.

## Usage

You will require the `thumbv8m.main-none-eabihf` target installed. To build one
of the examples:

```console
$ rustup target add thumbv8m.main-none-eabihf
$ git clone https://github.com/nrf-rs/nrf9160-dk.git
$ cd nrf9160-dk
$ cargo build --target=thumbv8m.main-none-eabihf --example blinky
```

To use in your own application, add as a dependency and call the
`Board::take()` function.

## Minimum Supported Rust Version

This crate is guaranteed to build on stable Rust 1.41 and higher.

## Licence

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.
