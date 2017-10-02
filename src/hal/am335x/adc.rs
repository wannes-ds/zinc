//! ADC module

#[path = "../../util/ioreg.rs"]
#[macro_use]
mod ioreg;

#[path = "../../util/wait_for.rs"]
#[macro_use]
mod wait_for;

use hal::am335x::wakeup_clock;

/// ADC module
#[derive(Clone, Copy)]
#[allow(non_camel_case)]
pub struct ADC_TSC {
    /// Reg
    reg: &'static reg::ADC_TSC,
}

impl ADC_TSC {
    /// Create ADC_TSC instance
    pub fn new() -> ADC_TSC {
        wakeup_clock::WakeUpClock::enable(wakeup_clock::WakeUpClocks::ADC_TSC);

        ADC_TSC {
            reg: &self::reg::ADC_TSC
        }
    }

    /// Read analog input
    pub fn read_input(&self, input: u8) -> u16 {
        self.reg.step[0].ctrl
            .set_input_select(input as u32)
            .set_mode(self::reg::ADC_TSC_step_ctrl_mode::SW_ONE_SHOT);
        self.reg.step_enable.set_step_enable(0, true);
        self.reg.ctrl.set_enable(true);

        wait_for!(self.reg.fifo[0].count.words() > 0);

        self.reg.fifo_data[0].adc_data() as u16
    }
}

lazy_static! {
    /// ADC module
    pub static ref ADC: ADC_TSC = ADC_TSC::new();
}

mod reg {
    use volatile_cell::VolatileCell;
    use core::ops::Drop;

    ioregs!(ADC_TSC = {
        0x40 => reg32 ctrl {
            9 => hw_preempt,
            8 => hw_event_mapping,
            7 => touch_screen_enable,
            6..5 => afe_pen_control,
            4 => power_down,
            3 => adc_bias_select,
            2 => step_config_write_protect,
            1 => step_id_tag,
            0 => enable
        }
        0x54 => reg32 step_enable {
            16..1 => step_enable[16]
        }
        0x64 => group step[16] {
            0x0 => reg32 ctrl {
                27 => out_of_range_check_enable,
                26 => fifo_select,
                25 => differenctial_control,
                24..23 => rfm_select,
                22..19 => input_select,
                18..15 => negative_diff_select,
                14..12 => rfp_select,
                11 => wpnsw,
                10 => ypnsw,
                9 => xpnsw,
                8 => ynnsw,
                7 => yppsw,
                6 => xnnsw,
                5 => xppsw,
                4..2 => samples_average,
                1..0 => mode {
                    0 => SW_ONE_SHOT,
                    1 => SW_CONTINUOUS,
                    2 => HW_ONE_SHOT,
                    3 => HW_CONTINUOUS
                }
            }
            0x4 => reg32 delay {
                31..24 => sample_delay,
                17..0 => open_delay
            }
        }
        0xE4 => group fifo[2] {
            0x0 => reg32 count {
                6..0 => words
            }
            0x4 => reg32 threshold {
                5..0 => threshold
            }
            0x8 => reg32 dma {
                5..0 => dma_request_level
            }
        }
        0x100 => reg32 fifo_data[2] {
            19..16 => adc_channel_tag,
            11..0 => adc_data
        }
    });

    extern {
        #[link_name = "am335x_iomem_ADC_TSC"]
        pub static ADC_TSC: ADC_TSC;
    }
}