# Rust-Embedded-Basic-Blinker

A project to get started evolving from using *.ino files in the Arduino IDE or the ArduinoUploader C# project into Rust.

## Building

### Install dependencies:

```sh
arduino-cli core install arduino:samd
cargo install cargo-binutils cargo-make
rustup component add llvm-tools-preview
```

### Build & flash:

```sh
cargo build --release
rust-objcopy -O binary target/thumbv6m-none-eabi/release/aeon-tokens-iot target/aeon-tokens-iot.bin
arduino-cli upload -i target/aeon-tokens-iot.bin -b arduino:samd:nano_33_iot -p /dev/ttyACM0
```
### Automated build and flash

To save time, use the makefile for cargo-make so you can just run following command while your arduino in bootloader mode.

```sh
cargo make flash
```

### NOTE
After flashing, serial communication between your arduino and pc will be gone. So if you want to put your arduino into bootloader mode you just need to press `rst` button on the board twice. Then you will be able to flash your application to arduino again. Very handy when using the same board between this implementation in VSCode verses the Arduino IDE,

