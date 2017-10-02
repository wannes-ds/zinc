//! Wakeup clock
//!
use hal::am335x::util;

#[path="../../util/ioreg.rs"]
#[macro_use] mod ioreg;

#[allow(missing_docs, non_camel_case)]
#[derive(Clone, Copy)]
pub enum WakeUpClocks {
    M3 = 0,
    UART0 = 1,
    I2C0 = 2,
    ADC_TSC = 3,
    SMARTREFFLEX0 = 4,
    TIMER1 = 5,
    SMARTREFLEX1 = 6,
    WDT1 = 8,
}

/// Wakeupclock config
#[derive(Clone, Copy)]
pub struct WakeUpClock {}

impl WakeUpClock {
    fn reg() -> &'static reg::CM {
        &self::reg::WAKEUP_CLOCK
    }

    /// Enables clock
    #[inline(always)]
    pub fn enable(clock: WakeUpClocks) {
        let clkidx = clock as usize;
        WakeUpClock::reg().clkctrl[clkidx].set_module_mode(2);
    }
}

#[allow(dead_code)]
mod reg {
    use volatile_cell::VolatileCell;
    use core::ops::Drop;

    ioregs!(CM = {
        0xB0 => reg32 clkctrl[7] {
            18 => standby,
            17..16 => idle_state,
            1..0 => module_mode,
        }
    });

    extern {
        #[link_name = "am335x_iomem_CM_WKUP"]
        pub static WAKEUP_CLOCK: CM;
    }
}