# Board Support Package for the Nordic nRF9160-DK

This crate is a Board Support Package (BSP). It wraps the HAL crate
(`nrf9160-hal`) for the on-board nRF9160, and provides high level wrappers for
the onboard features.

## Usage

You will require the `thumbv8m.main-none-eabihf` target installed.

```console
$ rustup target add thumbv8m.main-none-eabihf
```

To use this BSP in your own application, add as a dependency and call the
`Board::take()` function.

To build one of the examples run:

```console
$ git clone https://github.com/nrf-rs/nrf9160-dk.git
$ cd nrf9160-dk
$ cargo objcopy --target=thumbv8m.main-none-eabihf --example blinky -- -O ihex target/thumbv8m.main-none-eabihf/debug/examples/blinky.hex
```

If you don't have `cargo-objcopy` installed, run:

```console
$ rustup component add llvm-tools-preview
$ cargo install cargo-binutils
```

Or you can just run objcopy manually:

```console
$ sudo apt install binutils # provides /usr/bin/objcopy on Ubuntu
$ cargo build --target=thumbv8m.main-none-eabihf --example blinky
$ objcopy -O ihex target/thumbv8m.main-none-eabihf/debug/examples/blinky target/thumbv8m.main-none-eabihf/debug/examples/blinky.hex
```

Either way you can then flash the `blinky.hex` file using SEGGER JFlashLite, or
your favourite flashing tool.

## Secure vs Non-Secure

This BSP is designed to run in non-secure mode, as should most of your
application code. You will therefore need a 'bootloader' which starts in secure
mode, moves the required peripherals into 'non-secure' world, and then jumps to
your application.

We have succesfully used Nordic's [Secure Partition
Manager](https://github.com/nrfconnect/sdk-nrf/tree/master/samples/spm) from nRF
SDK v1.5.1. SPM v1.5.1 is configured to expect your application at address
`0x0005_0000` and so that is what `memory.x` must specify as the start of Flash.
Note that other version of SPM might specify a different start address!

```console
$ west init -m https://github.com/nrfconnect/sdk-nrf --mr v1.5.1 ncs
$ cd ncs
$ west update # This takes *ages*
$ cd nrf/examples/spm
$ west build --board=nrf9160dk_nrf9160
$ west flash
```

Your nRF9160-DK will now have SPM installed between `0x0000_0000` and
`0x0004_FFFF`. Flashing your application at `0x0005_0000` should not affect SPM,
provided you do not select *erase entire chip* or somesuch!

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
