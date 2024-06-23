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

use stm32f3_discovery::stm32f3xx_hal::delay::Delay;
use stm32f3_discovery::stm32f3xx_hal::pac;
use stm32f3_discovery::stm32f3xx_hal::prelude::*;

use stm32f3_discovery::leds::Leds;
use stm32f3_discovery::switch_hal::ToggleableOutputSwitch;

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

    // Set up delay capability.
    //
    // Use the same unwrap and constrain methods to get the core periphs and
    // flash periph, set up the clocks, which requires passing in the flash ACR
    // register so the appropriate latency for flash memory can be set based on
    // the config of the clocks. Then, a Delay instance is created with the
    // clocks config and SysTick (SYST).
    //
    let core_periphs = cortex_m::Peripherals::take().unwrap_or_else(|| {
        loop {
            // Failed to take cortex_m::Peripherals.
            asm::nop(); // If real app, replace with actual error handling code.
        }
    });
    let mut flash = device_periphs.FLASH.constrain();
    let clocks = reset_and_clock_control.cfgr.freeze(&mut flash.acr);
    let mut delay = Delay::new(core_periphs.SYST, clocks);

    // Get GPIO Port E.
    //
    // The split method here splits out the functionality of the GPIO Port E
    // while taking a mutable borrow of an "enabler" that enables the clock for
    // the port at the same time. The mutable borrow allows modification of the
    // borrowed value while ensuring exclusive access.
    //
    let mut gpioe: stm32f3_discovery::stm32f3xx_hal::gpio::gpioe::Parts =
        device_periphs.GPIOE.split(&mut reset_and_clock_control.ahb);

    // Create an instance of the board's LEDs.
    //
    // The constructor of the Leds type takes the specific pins from GPIO Port
    // E that are attached to the LEDs on the board plus the mode and output
    // type registers for Port E.
    //
    let mut leds = Leds::new(
        gpioe.pe8,
        gpioe.pe9,
        gpioe.pe10,
        gpioe.pe11,
        gpioe.pe12,
        gpioe.pe13,
        gpioe.pe14,
        gpioe.pe15,
        &mut gpioe.moder,
        &mut gpioe.otyper,
    );

    // Delay in milliseconds between toggles of the LEDs.
    //
    const LED_TOGGLE_DELAY_MS: u16 = 500;

    // Main loop.
    //
    loop {
        leds.ld3.toggle().ok();
        leds.ld4.toggle().ok();
        leds.ld5.toggle().ok();
        leds.ld6.toggle().ok();
        leds.ld7.toggle().ok();
        leds.ld8.toggle().ok();
        leds.ld9.toggle().ok();
        leds.ld10.toggle().ok();
        delay.delay_ms(LED_TOGGLE_DELAY_MS);
    }
}
