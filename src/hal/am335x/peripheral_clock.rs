//! Peripheral clock
//!
use hal::am335x::util;

#[path="../../util/ioreg.rs"]
#[macro_use] mod ioreg;

/// Bus configuration
#[derive(Clone, Copy)]
pub struct GPIOClock {
    /// registers
    reg: &'static reg::CM,
    /// id
    id: usize
}

impl GPIOClock {
    /// Enables GPIO clock
    #[inline(always)]
    pub fn enable(&self) {
        // GPIO0 is not in CM_PER, we start with 1 which is at idx 0
        // todo BUG crashes in NEON/VFP3 mode, does not work in softfp mode
        //let gpioreg = self.reg.gpio[self.id - 1];
        //gpioreg.set_module_mode(0x2);
        //gpioreg.set_optional_func_clock(true);
        // because ^ doesn't remotely touch the CM_PER reg, use some good ol' asm instead
        let address = 0x44E000AC + ((self.id - 1) * 4);
        let value = 0x4002;
        util::put32(address, value);
    }
}

/// Utility class to access different clock modules
#[derive(Clone, Copy)]
pub struct PeripheralClockDomain {
}

impl PeripheralClockDomain {
    /// Get the clock for GPIO num
    pub fn gpio(id: usize) -> GPIOClock {
        GPIOClock {
            reg: &reg::PERIPHERAL_CLOCK,
            id: id,
        }
    }
}

#[allow(dead_code)]
mod reg {
    use volatile_cell::VolatileCell;

    ioregs!(CM = {
        0x0 => reg32 l4ls_clkstctrl {
            28 => timer6_active: ro,
            27 => timer5_active: ro,
            25 => spi_active: ro,
            24 => i2c_active: ro,
            21 => gpio3_active: ro,
            20 => gpio2_active: ro,
            19 => gpio1_active: ro,
            17 => lcd_active: ro,
            16 => timer4_active: ro,
            15 => timer3_active: ro,
            14 => timer2_active: ro,
            13 => timer7_active: ro,
            11 => can_active: ro,
            10 => uart_active: ro,
            8 => l4ls_active: ro,
            1..0 => clkctrl,

        }
        0xAC => reg32 gpio[3] {
            18 => optional_func_clock,
            17..16 => idle_state,
            1..0 => module_mode,
        }
    });

    extern {
        #[link_name = "am335x_iomem_CM_PER"]
        pub static PERIPHERAL_CLOCK: CM;
    }
}