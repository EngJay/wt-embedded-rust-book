#![deny(unsafe_code)]
#![no_std]
#![no_main]

// pick a panicking behavior
use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics

// use panic_abort as _; // requires nightly
// use panic_itm as _; // logs messages over ITM; requires ITM support
// use panic_semihosting as _; // logs messages to the host stderr; requires a debugger

use cortex_m::asm;
use cortex_m_rt::entry;

use stm32f3_discovery::stm32f3xx_hal::delay::Delay;
use stm32f3_discovery::stm32f3xx_hal::pac;
use stm32f3_discovery::stm32f3xx_hal::prelude::*;

use stm32f3_discovery::leds::Leds;
use stm32f3_discovery::switch_hal::{OutputSwitch, ToggleableOutputSwitch};

#[entry]
fn main() -> ! {
    // Get peripherals.
    //
    // take() returns an Option, which requires handling the possibility of the
    // return of an Err or None instead of the desired value, which is of type
    // Peripherals in this case.
    //
    // Since this is an embedded application, it's not as simple as writing to,
    // stdout. This is a minimal example, so we'll drop into an inifinite loop
    // to allow a debugger to find where the failure.
    //
    let device_periphs = pac::Peripherals::take().unwrap_or_else(|| {
        loop {
            // Failed to take Peripherals.
            asm::nop(); // If real app, replace with actual error handling code.
        }
    });

    // Get RCC peripheral.
    //
    // The constrain() method is used here to provide a higher-level abstraction
    // of the peripheral rather than raw register access. The method consumes
    // the raw peripheral and returns an instance of the RCC peripheral with
    // higher-level safe abstractions provided by the HAL, which is of type Rcc.
    //
    let mut reset_and_clock_control = device_periphs.RCC.constrain();

    loop {
        // your code goes here
    }
}
