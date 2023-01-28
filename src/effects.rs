use arduino_hal::Spi;
use smart_leds::RGB8;
use ws2812_spi::Ws2812;

pub mod rainbow_run;
pub mod rainbow;

pub trait Effect {
    fn run(&self, ws: Ws2812<Spi>, num_leds: usize, data: &mut [RGB8]) -> !;
}
