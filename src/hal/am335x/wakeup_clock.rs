//! Peripheral clock
//!
use hal::am335x::util;

#[path="../../util/ioreg.rs"]
#[macro_use] mod ioreg;

/// Bus configuration
#[derive(Clone, Copy)]
pub struct UARTClock {
    /// registers
    reg: &'static reg::CM,
}

impl UARTClock {
    /// Enables UART clock
    #[inline(always)]
    pub fn enable(&self) {
        self.reg.uart0.set_module_mode(2);
    }
}

/// Utility class to access different clock modules
#[derive(Clone, Copy)]
pub struct WakeUpClockDomain {
}

impl WakeUpClockDomain {
    /// Get the clock for UART0
    #[inline(always)]
    pub fn uart0() -> UARTClock {
        UARTClock {
            reg: &reg::WAKEUP_CLOCK,
        }
    }
}

#[allow(dead_code)]
mod reg {
    use volatile_cell::VolatileCell;
    use core::ops::Drop;

    ioregs!(CM = {
        0xB4 => reg32 uart0 {
            17..16 => idle_state,
            1..0 => module_mode,
        }
    });

    extern {
        #[link_name = "am335x_iomem_CM_WKUP"]
        pub static WAKEUP_CLOCK: CM;
    }
}