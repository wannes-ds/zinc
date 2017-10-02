#![feature(plugin, start, asm)]
#![no_std]
#![plugin(macro_zinc)]

extern crate zinc;

use zinc::hal::pin::{Gpio, GpioDirection};
use zinc::hal::am335x::{gpio, uart};
use zinc::drivers::chario::CharIO;
use zinc::hal::am335x::wakeup_clock::{WakeUpClock, WakeUpClocks};

#[zinc_main]
pub fn main() {
    zinc::hal::mem_init::init_stack();
    zinc::hal::mem_init::init_data();
    zinc::hal::am335x::init::enable_vfp();
    zinc::hal::am335x::peripheral_clock::PeripheralClockDomain::gpio(1).enable();
    WakeUpClock::enable(WakeUpClocks::UART0);

    let led1 = gpio::GpioPin {
        module: gpio::Module::Module1,
        pin: 21u32,
        mode: GpioDirection::Out
    };
    led1.setup();

    let uart0 = uart::UART::new(uart::UARTModule::Module0);
    uart0.start();

    led1.set_high();

    let adc = &zinc::hal::am335x::adc::ADC;

    loop {
        // :)
        let gvd = adc.read_input(7) as u32;
        uart0.puti(gvd);
        uart0.puts(" 🐼\r\n");
    }
}
