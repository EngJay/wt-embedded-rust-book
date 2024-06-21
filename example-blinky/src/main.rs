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
    asm::nop(); // To not have main optimize to abort in release mode, remove when you add code

    loop {
        // your code goes here
    }
}
