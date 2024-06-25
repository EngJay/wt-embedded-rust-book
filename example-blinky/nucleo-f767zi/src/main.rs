#![deny(unsafe_code)]
#![no_std]
#![no_main]

// Use halt as the panicking behavior.
//
// A breakpoint can be set on `rust_begin_unwind` to catch panics.
//
use panic_halt as _;
// use panic_abort as _; // requires nightly
// use panic_itm as _; // logs messages over ITM; requires ITM support
// use panic_semihosting as _; // logs messages to the host stderr; requires a debugger

use cortex_m::asm;
use cortex_m_rt::entry;

use stm32f7xx_hal::{pac, prelude::*};

#[entry]
fn main() -> ! {
    // Get peripherals.
    //
    // take() returns an Option, which requires handling the possibility of the
    // return of an Err or None instead of the desired value, which is of type
    // pac::Peripherals in this case.
    //
    // Since this is an embedded application, it's not as simple as writing to,
    // stdout. This is a minimal example, so we'll drop into an inifinite loop
    // to allow a debugger to find where the failure.
    //
    let device_periphs = pac::Peripherals::take().unwrap_or_else(|| {
        loop {
            // Failed to take pac::Peripherals.
            asm::nop(); // If real app, replace with actual error handling code.
        }
    });

    // Get RCC peripheral.
    //
    // The constrain() method is used here to provide a higher-level abstraction
    // of the peripheral rather than raw register access. The method consumes
    // the raw peripheral and returns an instance of the RCC peripheral with
    // higher-level safe abstractions provided by the HAL, which is of type Rcc,
    // while setting the system clock frequency.
    //
    let reset_and_clock_control = device_periphs.RCC.constrain();
    let clocks = reset_and_clock_control.cfgr.sysclk(48.MHz()).freeze();

    // Set up delay capability.
    //
    // Use the same unwrap and constrain methods to get the core periphs, then
    // create a delay abstrction using SysTick (SYST).
    //
    let core_periphs = cortex_m::Peripherals::take().unwrap_or_else(|| {
        loop {
            // Failed to take cortex_m::Peripherals.
            asm::nop(); // If real app, replace with actual error handling code.
        }
    });
    let mut delay = core_periphs.SYST.delay(&clocks);

    // Get GPIO Port B.
    //
    // The split method here splits out the functionality of the GPIO Port B
    // while taking a mutable borrow of an "enabler" that enables the clock for
    // the port at the same time. The mutable borrow allows modification of the
    // borrowed value while ensuring exclusive access.
    //
    let gpiob: stm32f7xx_hal::gpio::gpiob::Parts = device_periphs.GPIOB.split();

    // Create instances of the user LEDs in push-pull output mode.
    //
    // The initial state is low.
    //
    let mut led_ld1 = gpiob.pb0.into_push_pull_output();
    let mut led_ld2 = gpiob.pb7.into_push_pull_output();
    let mut led_ld3 = gpiob.pb14.into_push_pull_output();

    // Delays in milliseconds for LED states.
    //
    const LED_ON_DELAY_MS: u32 = 100;
    const LED_OFF_DELAY_MS: u32 = 400;

    // Main loop.
    //
    loop {
        // Blip the LEDs at 2 Hz for 100ms.
        led_ld1.set_high();
        led_ld2.set_high();
        led_ld3.set_high();
        delay.delay_ms(LED_ON_DELAY_MS);
        led_ld1.set_low();
        led_ld2.set_low();
        led_ld3.set_low();
        delay.delay_ms(LED_OFF_DELAY_MS);
    }
}
