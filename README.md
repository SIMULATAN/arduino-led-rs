`arduino-led-rs`
==================
Control your WS2812B LED strip with an Arduino Nano and Rust.

*This project is a work in progress.*

## Usage
If you don't have it already, install `ravedude`:

```bash
cargo install ravedude
```

Then run
```bash
cargo run --release
```

and it should show at least something.

If you get an error along the lines of `no matching serial port`, set the `RAVEDUDE_PORT` environment variable to the correct port (for ex. `/dev/ttyUSB0`).

## Connection
The connection between the Arduino and the LED strip is as follows (Arduino to LED strip):
- `5V` to `VCC`
- `D11` to `DATA`
  - this port is the `SPI MOSI` port on the Arduino Nano
- `GND` to `GND`

## License
This project is licensed under MIT

   ([LICENSE](LICENSE) or <http://opensource.org/licenses/MIT>)


## Contribution
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
