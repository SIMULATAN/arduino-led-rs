use arduino_hal::Spi;
use smart_leds::{RGB8, SmartLedsWrite};
use crate::effects::Effect;
use crate::ws2812::Ws2812;

use crate::utils::color::hsv_to_rgb;

#[derive(Debug)]
pub struct RainbowRun {}

impl Effect for RainbowRun {
    fn run(&self, mut ws: Ws2812<Spi>, num_leds: usize, data: &mut [RGB8]) -> ! {
        let mut hue: f32 = 0.0;
        loop {
            for i in 0..num_leds {
                let (r, g, b) = hsv_to_rgb(hue, 1.0, 1.0);
                data[i] = RGB8 { r, g, b, };
                hue = (hue + 1.0 / num_leds as f32) % 1.0;
            }
            hue = (hue + 0.001) % 1.0;
            ws.write(data.iter().cloned()).unwrap();
        }
    }
}
