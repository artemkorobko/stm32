# Basic MCU setup

This is the basic firmware for STM32F103 MCU using [rtic](https://rtic.rs/0.5/book/en/) framework.

- Set up clocks.
- Disable JTAG to use PA15, PB3, PB4 pins.

## Build and upload firmware
```
cargo build --release && \
openocd -f ../openocd.cfg -c "init" -c "reset init" -c "flash write_image erase ./target/thumbv7m-none-eabi/release/stm32f103" -c "reset run" -c "exit"
```
