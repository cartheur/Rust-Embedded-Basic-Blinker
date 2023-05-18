/*Embedded Systems for a...
Arduino Nano 33 IoT
SAMD21 Cortex-NO 32-bit MCU
256kb flash memory
32KB SRAM
ONboard WiFi/Bluetooth
cortex-m architecture crate
atsamd21g PAC crate
atsamd HAL crate
*/

#![no_std]
#![no_main]
//have to define entry function

//import board support crate
use arduino_nano33iot as bsp;
//the bsp re-exports the hal
use bsp::{hal, Pins};
//the panic_halt (linked)
use panic_halt as _;

use bsp::entry;

//import peripherals
use hal::pac::{CorePeripherals, Peripherals};
use hal::clock::GenericClockController;

//hal prelude with convenient traits
use hal::prelude::*;
use crate::hal::delay::Delay;

//import GPIO
use bsp::hal::gpio::v2::{Pin, PushPullOutput, PB10};

//entry macro
#[entry]
fn alt_main() -> !
{
    //take hal peripherals *need ownership*
    let mut peripherals:Peripherals = Peripherals::take().unwrap();

    //take peripherals from uarch crate (cortex m)
    let core:CorePeripherals = CorePeripherals::take().unwrap();

    //initilaize clock through internal peripherals
    let mut clock = GenericClockController::with_external_32kosc(
            peripherals.GCLK,
            &mut peripherals.PM,
            &mut peripherals.SYSCTRL,
            &mut peripherals.NVMCTRL,
    );

    //delay device with clock
    let mut delay = Delay::new(core.SYST, &mut clock);

    //setup pins
    let pins:bsp::Pins = bsp::Pins::new(peripherals.PORT);
    let mut led:Pin<PB10, PushPullOutput> = pins.d2.into();

    loop
    {
        delay.delay_ms(200u8);
        led.set_high().unwrap();
        delay.delay_ms(200u8);
        led.set_low().unwrap();
    }
}
