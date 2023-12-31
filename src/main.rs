//! The starter code slowly blinks the LED, sets up
//! USB logging, and creates a UART driver using pins
//! 14 and 15. The UART baud rate is [`UART_BAUD`].
//!
//! Despite targeting the Teensy 4.0, this starter code
//! also works on the Teensy 4.1.

#![no_std]
#![no_main]

use bsp::board;
use teensy4_bsp as bsp;
use teensy4_panic as _;

use bsp::hal::timer::Blocking;

use core::fmt::Write;
use ssd1306::{prelude::*, I2CDisplayInterface, Ssd1306};

/// Milliseconds to delay before toggling the LED
/// and writing text outputs.
const DELAY_MS: u32 = 500;

#[bsp::rt::entry]
fn main() -> ! {
    // These are peripheral instances. Let the board configure these for us.
    // This function can only be called once!
    let instances = board::instances();

    // Driver resources that are configured by the board. For more information,
    // see the `board` documentation.
    let board::Resources {
        // `pins` has objects that represent the physical pins. The object
        // for pin 13 is `p13`.
        pins,
        // This is a hardware timer. We'll use it for blocking delays.
        mut gpt1,
        // This is the GPIO2 port. We need this to configure the LED as a
        // GPIO output.
        mut gpio2,
        lpi2c1,
        ..
    } = board::t40(instances);

    let lpi2c: board::Lpi2c1 = board::lpi2c(
        lpi2c1,
        pins.p19,
        pins.p18,
        board::Lpi2cClockSpeed::KHz400,
    );
    let interface = I2CDisplayInterface::new(lpi2c);
    let mut display = Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
    .into_terminal_mode();

    // This configures the LED as a GPIO output.
    let led = board::led(&mut gpio2, pins.p13);

    // Configures the GPT1 timer to run at GPT1_FREQUENCY. See the
    // constants below for more information.
    gpt1.disable();
    gpt1.set_divider(GPT1_DIVIDER);
    gpt1.set_clock_source(GPT1_CLOCK_SOURCE);

    // Convenience for blocking delays.
    let mut delay = Blocking::<_, GPT1_FREQUENCY>::from_gpt(gpt1);

    display.init().unwrap();

    let _ = display.clear().unwrap();

    let mut counter: u32 = 0;
    loop {
        //max length: 16
  //      let input = "1234";
  //      let length = 16; // Specific length
  //      let _ = display.write_str(&input[..length.min(input.len())]);

       // let _ = display.write_str(unsafe { core::str::from_utf8_unchecked(&[10]) });
        let _ = display.set_column(1);
        let _ = display.write_fmt(format_args!("Counter: {}", counter));

        let _ = display.write_str(unsafe { core::str::from_utf8_unchecked(&[10]) });
        let _ = display.set_column(1);
        let _ = display.write_fmt(format_args!("Counter2: {}", counter));

        led.toggle();
        delay.block_ms(DELAY_MS);
        counter = counter.wrapping_add(1);

        let _ = display.write_str("                                                                               ");
        let _ = display.clear().unwrap();
    }
}

// We're responsible for configuring our timers.
// This example uses PERCLK_CLK as the GPT1 clock source,
// and it configures a 1 KHz GPT1 frequency by computing a
// GPT1 divider.
use bsp::hal::gpt::ClockSource;

/// The intended GPT1 frequency (Hz).
const GPT1_FREQUENCY: u32 = 1_000;
/// Given this clock source...
const GPT1_CLOCK_SOURCE: ClockSource = ClockSource::HighFrequencyReferenceClock;
/// ... the root clock is PERCLK_CLK. To configure a GPT1 frequency,
/// we need a divider of...
const GPT1_DIVIDER: u32 = board::PERCLK_FREQUENCY / GPT1_FREQUENCY;
