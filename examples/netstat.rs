#![no_std]
#![no_main]

//! This example is for the nRF9160-DK board. It to connect to the cellular
//! network and prints out cell status repeatedly.

extern crate tinyrlibc;

use cortex_m_rt as rt;
use nrf9160_dk_bsp as bsp;

use bsp::{hal::Timer, pac::interrupt, prelude::*, Board};
use nb::block;
use rt::entry;

/// A UART we can access from anywhere (with run-time lock checking).
static GLOBAL_UART: spin::Mutex<Option<bsp::hal::uarte::Uarte<bsp::pac::UARTE0_NS>>> =
    spin::Mutex::new(None);

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        {
            use core::fmt::Write as _;
            if let Some(ref mut uart) = *crate::GLOBAL_UART.lock() {
                let _err = write!(*uart, $($arg)*);
            }
        }
    };
}

#[macro_export]
macro_rules! println {
    () => (print!("\n"));
    ($($arg:tt)*) => {
        {
            use core::fmt::Write as _;
            if let Some(ref mut uart) = *crate::GLOBAL_UART.lock() {
                let _err = writeln!(*uart, $($arg)*);
            }
        }
    };
}

/// What to do if we get a panic!()
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    println!("Panic! {:?}", info);
    loop {
        cortex_m::asm::bkpt();
    }
}

/// Interrupt Handler for LTE related hardware. Defer straight to the library.
#[interrupt]
fn EGU1() {
    nrfxlib::application_irq_handler();
    cortex_m::asm::sev();
}

/// Interrupt Handler for LTE related hardware. Defer straight to the library.
#[interrupt]
fn EGU2() {
    nrfxlib::trace_irq_handler();
    cortex_m::asm::sev();
}

/// Interrupt Handler for LTE related hardware. Defer straight to the library.
#[interrupt]
fn IPC() {
    nrfxlib::ipc_irq_handler();
    cortex_m::asm::sev();
}

/// The entry-point to our program.
#[entry]
fn main() -> ! {
    let mut board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0_NS);

    unsafe {
        bsp::pac::NVIC::unmask(bsp::pac::Interrupt::EGU1);
        bsp::pac::NVIC::unmask(bsp::pac::Interrupt::EGU2);
    }

    *GLOBAL_UART.lock() = Some(board.cdc_uart);

    println!("This is Rust on the nRF9160 LTE SiP");
    println!("Developed by 42 Technology Ltd (www.42technology.com)");

    // Work around https://www.nordicsemi.com/DocLib/Content/Errata/nRF9160_EngA/latest/ERR/nRF9160/EngineeringA/latest/anomaly_160_17
    // *(volatile uint32_t *)0x40005C04 = 0x02ul;
    unsafe {
        core::ptr::write_volatile(0x4000_5C04 as *mut u32, 0x02);
    }

    nrfxlib::init().expect("nrfxlib::init");

    let at_socket = nrfxlib::at::AtSocket::new().expect("AT socket");

    let mut led_is_on = false;
    loop {
        if led_is_on {
            println!("Off");
            board.leds.led_1.disable();
        } else {
            println!("On");
            board.leds.led_1.enable();
        }
        timer.start(1_000_000_u32);
        block!(timer.wait()).unwrap();
        led_is_on = !led_is_on;

        at_socket.write(b"AT+CEREG?\r\n").expect("at_socket.write");
        let mut buffer = [0u8; 128];
        match at_socket.recv(&mut buffer) {
            Ok(Some(n)) => {
                let s = core::str::from_utf8(&buffer[..n]);
                println!("{:?}", s);
            }
            Ok(None) => {
                // Do nothing
            }
            Err(e) => {
                println!("Got error {:?} from AT socket!", e);
            }
        }
    }
}
