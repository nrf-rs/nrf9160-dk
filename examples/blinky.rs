#![no_std]
#![no_main]

//! This example is for the nRF9160-DK board. It prints to the UART and blinks
//! an LED. Open the lowest-numbered USB Serial Port presented by your
//! nRF9160-DK to see the UART output.

use cortex_m_rt as rt;
use nrf9160_dk_bsp as bsp;

use bsp::{hal::Timer, prelude::*, Board};
use core::fmt::Write;
use nb::block;
use rt::entry;

/// What to do if we get a panic!()
#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {
        cortex_m::asm::bkpt();
    }
}

#[entry]
fn main() -> ! {
    let mut board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0_NS);

    writeln!(board.cdc_uart, "Hello, world!").unwrap();

    let mut led_is_on = false;
    loop {
        if led_is_on {
            board.leds.led_1.disable();
        } else {
            board.leds.led_1.enable();
        }
        timer.start(1_000_000_u32);
        block!(timer.wait()).unwrap();
        led_is_on = !led_is_on;
    }
}
