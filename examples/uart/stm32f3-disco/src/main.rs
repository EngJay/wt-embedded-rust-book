#![deny(unsafe_code)]
#![no_std]
#![no_main]

use core::fmt::Write;

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

use stm32f3xx_hal::{delay::Delay, pac, prelude::*, serial::config, serial::Serial};

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
            asm::nop(); // If real app, replace with actual error handling.
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
    let mut reset_and_clock_control = device_periphs.RCC.constrain();
    let mut flash = device_periphs.FLASH.constrain();
    let clocks = reset_and_clock_control
        .cfgr
        .sysclk(48.MHz())
        .freeze(&mut flash.acr);

    // Set up delay capability.
    //
    // Use the same unwrap method to get the core periphs, then
    // create a delay abstrction using SysTick (SYST).
    //
    let core_periphs = cortex_m::Peripherals::take().unwrap_or_else(|| {
        loop {
            // Failed to take cortex_m::Peripherals.
            asm::nop(); // If real app, replace with actual error handling.
        }
    });
    let mut delay = Delay::new(core_periphs.SYST, clocks);

    // Get GPIO Port C.
    //
    // The split method here splits out the functionality of the GPIO Port C
    // while taking a mutable borrow of an "enabler" that enables the clock for
    // the port at the same time. The mutable borrow allows modification of the
    // borrowed value while ensuring exclusive access.
    //
    let mut gpioc: stm32f3xx_hal::gpio::gpioc::Parts =
        device_periphs.GPIOC.split(&mut reset_and_clock_control.ahb);

    // Configure GPIO pins PC10 as TX and PC11 as RX for UART4.
    let tx_pin = gpioc
        .pc10
        .into_af_push_pull(&mut gpioc.moder, &mut gpioc.otyper, &mut gpioc.afrh);
    let rx_pin = gpioc
        .pc11
        .into_af_push_pull(&mut gpioc.moder, &mut gpioc.otyper, &mut gpioc.afrh);

    // Activate the UART.
    let mut uart4 = Serial::new(
        device_periphs.UART4,
        (tx_pin, rx_pin),
        config::Config::default().baudrate(115_200.Bd()),
        clocks,
        &mut reset_and_clock_control.apb1,
    );

    // Delay in milliseconds between UART writes.
    //
    const UART_WRITE_DELAY_MS: u16 = 2_000;

    loop {
        uart4.write_str("Hello, World!\r\n").unwrap_or_else(|_| {
            loop {
                // Failed to write to UART4.
                asm::nop(); // If real app, replace with actual error handling.
            }
        });
        delay.delay_ms(UART_WRITE_DELAY_MS);
    }
}
