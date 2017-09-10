#![feature(plugin, start)]
#![no_std]
#![plugin(macro_zinc)]

extern crate zinc;

use zinc::hal::timer::Timer;
use zinc::hal::pin::Gpio;
use zinc::hal::stm32f4::{pin, timer, i2c};

#[zinc_main]
pub fn main() {
    zinc::hal::mem_init::init_stack();
    zinc::hal::mem_init::init_data();

    let led1 = pin::Pin {
        port: pin::Port::PortD,
        pin: 13u8,
        function: pin::Function::GPIOOut
    };
    let led2 = pin::Pin {
        port: pin::Port::PortD,
        pin: 14u8,
        function: pin::Function::GPIOOut
    };
    let led3 = pin::Pin {
        port: pin::Port::PortD,
        pin: 15u8,
        function: pin::Function::GPIOOut
    };
    let led4 = pin::Pin {
        port: pin::Port::PortD,
        pin: 12u8,
        function: pin::Function::GPIOOut
    };

    led1.setup();
    led2.setup();
    led3.setup();
    led4.setup();

    let i2c = i2c::I2C::new(i2c::I2CBus::I2C1);
    i2c.conf_pin(6, pin::Port::PortB);
    i2c.conf_pin(7, pin::Port::PortB);

    i2c.setup(100_000);
    i2c.slave(8);

    let timer = timer::Timer::new(timer::TimerPeripheral::Timer2, 16u32);

    loop {
        let val = i2c.read();
        if val == 0 {
            continue;
        }

        led1.set_low();
        led2.set_low();
        led3.set_low();
        led4.set_low();

        match val {
            1 => led1.set_high(),
            2 => led2.set_high(),
            3 => led3.set_high(),
            4 => led4.set_high(),
            _ => ()
        }
    }
}
