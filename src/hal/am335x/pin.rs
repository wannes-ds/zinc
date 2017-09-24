//! Pin configuration for TI AM335X.
//!
//! Some pins that could be configured here may be missing from actual MCU
//! depending on the package.
//!

use core::intrinsics::abort;

#[path = "../../util/ioreg.rs"]
#[macro_use]
mod ioreg;

#[allow(missing_docs)]
#[derive(Copy, Clone)]
pub enum ModulePin {
    UART_0_CTSN = 90,
    UART_0_RTSN = 91,
    UART_0_RXD = 92,
    UART_0_TXD = 93
}

/// Pin configuration helper
pub fn conf_pin(pin: ModulePin, slow_slew: bool, receiver: bool, pullup: bool, pull: bool, mux: u8) {
    if mux > 7 {
        // Pin has up to 7 mux options
        unsafe { abort() }
    }

    let modpin = pin as usize;
    &reg::CONTROL_MODULE.conf_mod_pin[modpin]
        .set_slow_slew_rate(slow_slew)
        .set_receiver_enabled(receiver)
        .set_pullup(pullup)
        .set_pull_enabled(pull)
        .set_pin_mux(mux as u32);
}

#[allow(dead_code)]
mod reg {
    use core::ops::Drop;
    use volatile_cell::VolatileCell;

    ioregs!(CONTROL_MODULE = {
        0x0 => reg32 control_revision {
            31..30 => ip_rev_scheme,
            27..24 => ip_rev_func,
            15..11 => ip_rev_rtl,
            10..8 => ip_rev_major,
            7..6 => ip_rev_custom,
            5..0 => ip_rev_minor
        }
        0x800 => reg32 conf_mod_pin[123] {
            6 => slow_slew_rate,
            5 => receiver_enabled,
            4 => pullup,
            3 => pull_enabled,
            2..0 => pin_mux
        }
    });

    extern {
        #[link_name="am335x_iomem_CONTROL_MODULE"] pub static CONTROL_MODULE: CONTROL_MODULE;
    }
}