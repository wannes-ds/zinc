//! Pin configuration for TI AM335X.
//!
//! Some pins that could be configured here may be missing from actual MCU
//! depending on the package.

use hal::pin::{Gpio, GpioDirection, GpioLevel};

use self::Module::*;

#[path = "../../util/ioreg.rs"]
#[macro_use]
mod ioreg;

/// Available modules names.
#[allow(missing_docs)]
#[derive(Clone, Copy)]
pub enum Module {
    Module1,
   // Module2,
   // Module3,
   // Module4,
}

/// Pin configuration
#[derive(Clone, Copy)]
pub struct Pin {
    /// Pin port, mcu-specific.
    pub module: Module,
    /// Pin number.
    pub pin: u32,
    /// Mode
    pub mode: GpioDirection
}

impl Pin {
    /// Setup the pin.
    #[inline(always)]
    pub fn setup(&self) {
        use self::reg::GPIO_oe_mode as RegMode;

        let offset = self.pin as usize;
        let gpreg = self.get_reg();

        let val = match self.mode {
            GpioDirection::Out => RegMode::Output,
            GpioDirection::In => RegMode::Input,
        };

        gpreg.oe.set_mode(offset, val);
    }

    /// Toggles the GPIO value
    pub fn toggle(&self) {
        let reg = self.get_reg();
        let offset = self.pin as usize;

        reg.data_out.set_data(offset, !reg.data_in.data(offset));
    }

    fn get_reg(&self) -> &reg::GPIO {
        match self.module {
            Module1 => &reg::GPIO_1,
        }
    }
}

impl Gpio for Pin {
    /// Sets output GPIO value to high.
    fn set_high(&self) {
        let offset = self.pin as usize;
        self.get_reg().data_out.set_data(offset, true);
    }

    /// Sets output GPIO value to low.
    fn set_low(&self) {
        let offset = self.pin as usize;
        self.get_reg().data_out.set_data(offset, false);
    }

    /// Returns input GPIO level.
    fn level(&self) -> GpioLevel {
        let offset = self.pin as usize;
        let reg = self.get_reg();

        match reg.data_in.data(offset) {
            false => GpioLevel::Low,
            _ => GpioLevel::High,
        }
    }

    /// Sets output GPIO direction.
    fn set_direction(&self, new_mode: GpioDirection) {
        use self::reg::GPIO_oe_mode as RegMode;
        let offset = self.pin as usize;
        let reg = self.get_reg();

        let val = match new_mode {
            GpioDirection::Out => RegMode::Output,
            GpioDirection::In => RegMode::Input,
        };

        reg.oe.set_mode(offset, val);
    }
}

#[allow(dead_code)]
mod reg {
    use core::ops::Drop;
    use volatile_cell::VolatileCell;

    ioregs!(GPIO = {
        0x10 => reg32 sysconfig {
            4..3 => idle_mode,
            2 => wakeup_enabled,
            1 => software_reset,
            0 => auto_idle
        }
        0x130 => reg32 ctrl {
            2..1 => gating_info,
            0 => disable_module
        }
        0x134 => reg32 oe {
            0..31 => mode[32] {
                0 => Output,
                1 => Input
            }
        }
        0x138 => reg32 data_in {
            0..31 => data[32]: ro
        }
        0x13C => reg32 data_out {
            0..31 => data[32]
        }
    });

    extern {
        #[link_name="am335x_iomem_GPIO1"] pub static GPIO_1: GPIO;
    }
}