//! UART module
//!

use hal::am335x::pin;
use hal::am335x::util;
use drivers::chario::CharIO;

#[path = "../../util/ioreg.rs"]
#[macro_use]
mod ioreg;

#[path = "../../util/wait_for.rs"]
#[macro_use]
mod wait_for;

#[allow(missing_docs)]
#[derive(Clone, Copy)]
pub enum UARTModule {
    Module0,
}

/// UART module
#[derive(Clone, Copy)]
pub struct UART {
    /// Module
    module: UARTModule,
    /// Reg
    reg: &'static reg::UART,
    /// Reg
    reg_mode_b: &'static reg::UART_B,

}

impl UART {
    /// Create new UART module instance
    pub fn new(module: UARTModule) -> UART {
        let reg = match module {
            UARTModule::Module0 => &reg::UART_0
        };

        let ramb = match module {
            UARTModule::Module0 => &reg::UART_0_B
        };

        UART {
            module: module,
            reg: reg,
            reg_mode_b: ramb
        }
    }

    /// Start UART module
    pub fn start(&self) {
        pin::conf_pin(pin::ModulePin::UART_0_RXD, false, true, true, false, 0);
        pin::conf_pin(pin::ModulePin::UART_0_TXD, false, false, true, false, 0);

        use self::reg::UART_mdr1_mode_select as UARTMode;

        self.reg.sysc.set_soft_reset(true);
        wait_for!(self.reg.syss.reset_done());

        // disable idle mode
        self.reg.sysc.set(0x8);
        // switch to mode B
        self.reg.lcr.set(0x83);
        self.reg_mode_b.dll.set_clock_lsb(0x1A);

        // 115.2 kbps
        // switch to mode A
        self.reg.mdr1.set(0x0);
        // switch to operational mode
        self.reg.lcr         // mode set 8b1s
            .set(0x3)
            .set_divisor_latch_enable(false)
            .set_break_control(false)
            .set_char_length(0x3)
            .set_stop_bits(false);


        // hw flow
        /*let lcr: u16 = self.reg.lcr.get().raw();
        self.reg.lcr.set(0x80);
        self.reg.mcr.set_tcr_tlr_enable(true);
        self.reg.lcr.set(0xBF);
        self.reg.efr.set_enhanced_functions(true);
        self.reg.efr.set_auto_cts_enable(true);
        self.reg.efr.set_auto_rts_enable(true);
        self.reg.lcr.set(0x80);
        self.reg.mcr.set_tcr_tlr_enable(false);
        self.reg.lcr.set(lcr);*/
    }

    /// Writes byte to FIFO register
    pub fn write(&self, byte: u8) {
        wait_for!(self.reg.lsr_uart.tx_fifo_shift_empty());
        self.reg.hr.set(byte as u16);
    }
}

impl CharIO for UART {
    fn putc(&self, value: char) {
        self.write(value as u8);
    }
}

mod reg {
    use volatile_cell::VolatileCell;
    use core::ops::Drop;

    ioregs!(UART = {
        0x0 => reg16 hr {
            7..0 => data
        }
        0x4 => reg16 ier_uart {
            7 => cts_int_enable,
            6 => rts_int_enable,
            5 => xoff_int_enable,
            4 => sleep_mode,
            3 => modem_status_int_enable,
            2 => line_status_int_enable,
            1 => thr_int_enable,
            0 => rhr_int_enable
        }
        0x8 => reg16 efr {
            7 => auto_cts_enable,
            6 => auto_rts_enable,
            5 => special_char_detection,
            4 => enhanced_functions,
            3..0 => sw_flow_control
        }
        0x10 => reg16 mcr {
            6 => tcr_tlr_enable,
            5 => xon_function_enable,
            4 => loopback_enable,
            3 => loopback_dcd_low_irq_inactive,
            2 => loopback_ri_low,
            1 => rts,
            0 => loopback_dtr_low
        }
        0xC => reg16 lcr {
            7 => divisor_latch_enable,
            6 => break_control,
            5 => parity_type_2,
            4 => parity_type_1,
            3 => parity_enable,
            2 => stop_bits,
            1..0 => char_length,
        }
        0x14 => reg16 lsr_uart {
            7 => rx_fifo_error,
            6 => tx_fifo_shift_empty,
            5 => tx_fifo_empty,
            4 => rx_break,
            3 => rx_framing_error,
            2 => rx_parity_error,
            1 => rx_overrun_error,
            0 => rx_fifo_not_empty
        }
        0x18 => reg16 tcr {
            7..4 => rx_fifo_restore_trigger,
            3..0 => rx_fifo_halt_trigger
        }
        0x20 => reg16 mdr1 {
            7 => frame_end_mode,
            6 => sip_mode,
            5 => sct,
            4 => set_tx_ir,
            3 => irsleep,
            2..0 => mode_select {
                0 => UART_16,
                1 => SIR,
                2 => UART_16_AUTO_BAUD,
                3 => UART_13,
                4 => MIR,
                5 => FIR,
                6 => CIR,
                7 => DISABLE
            }
        }
        0x54 => reg16 sysc {
            4..3 => idle_mode,
            2 => wakeup_enable,
            1 => soft_reset,
            0 => auto_idle
        }
        0x58 => reg16 syss {
            0 => reset_done: ro
        }
    });

    ioregs!(UART_B = {
        0x0 => reg16 dll {
            7..0 => clock_lsb
        }
        0x4 => reg16 dlh {
            5..0 => clock_msb
        }
    });

    extern {
        #[link_name = "am335x_iomem_UART0"]
        pub static UART_0: UART;
        #[link_name = "am335x_iomem_UART0"]
        pub static UART_0_B: UART_B;
    }
}