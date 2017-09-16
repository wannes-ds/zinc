#![feature(plugin, start)]
#![no_std]
#![plugin(macro_zinc)]

extern crate zinc;

use zinc::hal::pin::{Gpio, GpioDirection};
use zinc::hal::am335x::pin;

#[zinc_main]
pub fn main() {
    zinc::hal::mem_init::init_stack();
    zinc::hal::mem_init::init_data();
    zinc::hal::am335x::init::enable_vfp();
    zinc::hal::am335x::peripheral_clock::PeripheralClockDomain::gpio(1).enable();

    let led1 = pin::Pin {
        module: pin::Module::Module1,
        pin: 21u32,
        mode: GpioDirection::Out
    };
    led1.setup();

    let div = 8.6;
    let mut i = 0.1;

    loop {
        led1.set_high();
        let test = "moop";
        let float_test = i / 8.6;
        i = i + 1.0;
    }
}
