#![no_std]
#![no_main]

use arduino_hal::{Peripherals, Spi, spi};
// Pull in the panic handler from panic-halt
use panic_halt as _;
use smart_leds::RGB8;
use ws2812_spi as ws2812;
use crate::effects::Effect;
use crate::effects::rainbow::Rainbow;

use crate::ws2812::Ws2812;

mod effects;
mod utils;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    // Create SPI interface.

    // create periodic timer at 3 MHz
    let (spi, _) = Spi::new(
        dp.SPI,
        pins.d13.into_output(),
        pins.d11.into_output(),
        pins.d12.into_pull_up_input(),
        pins.d10.into_output(),
        spi::Settings::default()
    );

    const NUM_LEDS: usize = 150;

    let mut data: [RGB8; NUM_LEDS] = [RGB8::default(); NUM_LEDS];
    let ws = Ws2812::new(spi);

    let rainbow = Rainbow {};
    rainbow.run(ws, NUM_LEDS, &mut data)
}

trait Abs {
    fn abs(self) -> Self;
}

impl Abs for f32 {
    fn abs(self) -> Self {
        if self < 0.0 {
            -self
        } else {
            self
        }
    }
}
