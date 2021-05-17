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

## Debugging

The nRF9160-DK has an on-board SEGGER JLink debug probe. You need to run the SEGGER JLink-to-GDB server software, and you can then debug the board using any GDB interface.

```console
$ /opt/SEGGER/JLink/JLinkGDBServer &
$ cargo build --target=thumbv8m.main-none-eabihf --example blinky
$ gdb-multiarch ./target/thumbv8m.main-none-eabihf/debug/examples/blinky
GNU gdb (Ubuntu 9.2-0ubuntu1~20.04) 9.2
Copyright (C) 2020 Free Software Foundation, Inc.
License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>
This is free software: you are free to change and redistribute it.
There is NO WARRANTY, to the extent permitted by law.
Type "show copying" and "show warranty" for details.
This GDB was configured as "x86_64-linux-gnu".
Type "show configuration" for configuration details.
For bug reporting instructions, please see:
<http://www.gnu.org/software/gdb/bugs/>.
Find the GDB manual and other documentation resources online at:
    <http://www.gnu.org/software/gdb/documentation/>.

For help, type "help".
Type "apropos word" to search for commands related to "word"...
Reading symbols from ./target/thumbv8m.main-none-eabihf/debug/examples/blinky...
(gdb) target extended-remote :2331
Remote debugging using :2331
0x00050aaa in nrf_hal_common::timer::Instance::timer_running (self=0x2003ff7c) at /home/jpallant/.cargo/git/checkouts/nrf-hal-eaee0cb5ab64b08f/a8dddf1/nrf-hal-common/src/timer.rs:398
398         fn timer_running(&self) -> bool {
(gdb) monitor halt
(gdb) load
Loading section .vector_table, size 0x144 lma 0x50000
Loading section .text, size 0x4e4c lma 0x50144
Loading section .rodata, size 0xae0 lma 0x54f90
Start address 0x00050144, load size 23152
Transfer rate: 11304 KB/sec, 5788 bytes/write.
(gdb) break main
Breakpoint 1 at 0x515f4: file examples/blinky.rs, line 24.
(gdb) monitor reset
Resetting target
(gdb) continue
Continuing.

Breakpoint 1, main () at examples/blinky.rs:24
24      #[entry]
(gdb) bt
#0  main () at examples/blinky.rs:24
(gdb) 
```

You can also follow [this guide on useing SEGGER J-Link with Visual Studio Code](https://wiki.segger.com/J-Link_Visual_Studio_Code). The `"device"` parameter should be set to `"nrf9160"`.

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
