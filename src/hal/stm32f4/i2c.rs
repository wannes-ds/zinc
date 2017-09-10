//! I2C Module
//! Wannes

use hal::pin::Gpio;
use core::intrinsics::abort;
use super::{pin, timer, peripheral_clock};

#[path = "../../util/ioreg.rs"]
#[macro_use]
mod ioreg;

#[path = "../../util/wait_for.rs"]
#[macro_use]
mod wait_for;

#[allow(missing_docs)]
#[derive(Clone, Copy)]
pub enum I2CBus {
    I2C1,
    I2C2,
    I2C3
}

impl I2CBus {
    fn clock(self) -> peripheral_clock::PeripheralClock {
        use hal::stm32f4::peripheral_clock::PeripheralClock::*;

        match self {
            self::I2CBus::I2C1 => I2C1Clock,
            self::I2CBus::I2C2 => I2C2Clock,
            self::I2CBus::I2C3 => I2C3Clock,
        }
    }
}

/// Bus configuration
#[derive(Clone, Copy)]
pub struct I2C {
    /// Bus number
    pub bus: I2CBus,
    /// Regs
    reg: &'static reg::I2C
}

impl I2C {
    /// Create new I2C bus
    pub fn new(bus: I2CBus) -> I2C {
        let clock = bus.clock();
        clock.enable();
        let reg = match bus {
            I2CBus::I2C1 => &reg::I2C1,
            I2CBus::I2C2 => &reg::I2C2,
            I2CBus::I2C3 => &reg::I2C3
        };

        I2C {
            bus: bus,
            reg: reg,
        }
    }

    /// Set pins
    pub fn conf_pin(&self, pin: u8, port: pin::Port) -> pin::AlternateFunctionPin {
        let i2c_pin = pin::AlternateFunctionPin::new(pin, port, 4);
        i2c_pin.setup();
        i2c_pin.five_volt();

        i2c_pin
    }

    /// Setup
    pub fn setup(&self, clock: u32) {
        let pclk1 = 16_000_000;
        let freqrange = (pclk1 / 1_000_000);
        // Our APB clock is 16 MHz
        self.reg.cr2.set_peripheralclock(freqrange);

        if clock <= 100_000 {
            // Calculate speed for standard mode
            let mut res = (pclk1 / (clock << 1));
            if res < 0x04 {
                res = 0x04;
            }

            // Rise time (see doc, FREQ + 1)
            self.reg.trise.set_trise(freqrange + 1);

            // As our base clock is 16MHz, divide it by 80 (0x50) to get to 200KHz
            self.reg.ccr.set_ccr(res);
        } else {
            // todo fast mode
        }

        // Enable I2C
        self.reg.cr1.set_peripheral_enable(true);
        wait_for!(self.reg.cr1.peripheral_enable());
    }

    /// Set own address
    pub fn slave(&self, addr: u8) {
        self.reg.oar1.set_address(addr as u32);
        self.reg.oar1.set_high(true);
        self.reg.cr1.set_enable_ack(true);
    }

    /// Read from bus
    pub fn read(&self) -> u8 {
        let mut val: u8 = 0;

        loop {
            let sr1 = self.reg.sr1.get();

            if sr1.byte_transfer_finished() || sr1.data_register_not_empty() {
                val = self.reg.dr.data() as u8;
                return val;
            }

            if sr1.address() {
                // We have received our address
                self.reg.sr1.get();
                self.reg.sr2.get();
            }

            if sr1.stop() {
                self.reg.sr1.get();
                self.reg.cr1.set_peripheral_enable(true);

                return 0;
            }

            if sr1.bus_error() {
                self.reg.sr1.set_bus_error(false);
            }
        }

        0
    }

    /// Start mastering the bus
    pub fn start(&self) {
        // Standard mode
        self.reg.ccr.set_master_mode(false);
        // Set START bit
        self.reg.cr1.set_start(true);
        // Wait for START condition generation
        wait_for!(self.reg.sr1.start_bit());
        wait_for!(self.reg.sr2.master_slave_mode());
        // We have become master!
    }

    /// Writes something on the I2C bus
    pub fn write(&self, addr: u8, value: u8) {
        let write_addr = addr << 1;
        self.reg.dr.set_data(write_addr as u32);
        wait_for!(self.reg.sr1.address());
        wait_for!(self.reg.sr1.data_register_empty());

        self.reg.dr.set_data(value as u32);
        wait_for!(self.reg.sr1.data_register_empty());

        // Set STOP bit
        self.reg.cr1.set_stop(true);
    }
}

#[allow(dead_code)]
mod reg {
    use core::ops::Drop;
    use volatile_cell::VolatileCell;

    ioregs!(I2C = {
        0x0 => reg32 cr1 {
            0 => peripheral_enable,
            1 => smbus,
            3 => smb_type,
            4 => enable_arp,
            5 => enable_pec,
            6 => enable_general_call,
            7 => disable_clock_stretching,
            8 => start,
            9 => stop,
            10 => enable_ack,
            11 => ack,
            12 => packet_error_checking,
            13 => smbus_alert,
            15 => swreset,
        }
        0x4 => reg32 cr2 {
            5..0 => peripheralclock,
            8 => enable_error_interrupt,
            9 => enable_event_interrupt,
            10 => enable_buffer_interrupt,
            11 => enable_dma_requests,
            12 => dma_last_transfer
        }
        0x8 => reg32 oar1 {
            7..1 => address,
            14 => high,
            15 => addressing_mode
        }
        0xC => reg32 oar2 {
            7..0 => oa2[8]
        }
        0x10 => reg32 dr {
            7..0 => data
        }
        0x14 => reg32 sr1 {
            0 => start_bit,
            1 => address,
            2 => byte_transfer_finished,
            3 => ten_bit_header_sent,
            4 => stop,
            6 => data_register_not_empty,
            7 => data_register_empty,
            8 => bus_error,
            9 => arbitration_lost,
            10 => acknowledge_failure,
            11 => underrun_overrun,
            12 => pec_error,
            14 => timeout,
            15 => smbus_alert
        }
        0x18 => reg32 sr2 {
            0 => master_slave_mode,
            1 => busy,
            2 => transmitter_receiver,
            4 => general_call_address,
            5 => smbus_default_address,
            6 => smbus_host_header,
            7 => dual
        }
        0x1C => reg32 ccr {
            11..0 => ccr,
            14 => duty,
            15 => master_mode
        }
        0x20 => reg32 trise {
            5..0 => trise
        }
    });

    extern {
        #[link_name = "stm32f4_iomem_I2C1"]
        pub static I2C1: I2C;
        #[link_name = "stm32f4_iomem_I2C2"]
        pub static I2C2: I2C;
        #[link_name = "stm32f4_iomem_I2C3"]
        pub static I2C3: I2C;
    }
}