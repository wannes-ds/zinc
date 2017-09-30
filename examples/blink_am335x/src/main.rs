#![feature(plugin, start)]
#![no_std]
#![plugin(macro_zinc)]

extern crate zinc;

use zinc::hal::pin::{Gpio, GpioDirection};
use zinc::hal::am335x::{gpio, uart};
use zinc::drivers::chario::CharIO;

#[zinc_main]
pub fn main() {
    zinc::hal::mem_init::init_stack();
    zinc::hal::mem_init::init_data();
    zinc::hal::am335x::init::enable_vfp();
    zinc::hal::am335x::peripheral_clock::PeripheralClockDomain::gpio(1).enable();
    zinc::hal::am335x::wakeup_clock::WakeUpClockDomain::uart0().enable();

    let led1 = gpio::GpioPin {
        module: gpio::Module::Module1,
        pin: 21u32,
        mode: GpioDirection::Out
    };
    led1.setup();

    let uart0 = uart::UART::new(uart::UARTModule::Module0);
    uart0.start();

    let div = 8.6;
    let mut i = 0.1;

    led1.set_high();

    loop {
        // :)
        uart0.puts("I am a little panda 🐼\r\n");
    }
}
