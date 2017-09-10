// Zinc, the bare metal stack for rust.
// Copyright 2014 Vladimir "farcaller" Pouzanov <farcaller@gmail.com>
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Pin configuration for ST STM32F4.
//!
//! Some pins that could be configured here may be missing from actual MCU
//! depending on the package.

use hal::pin::{Gpio, GpioDirection, GpioLevel};
use super::peripheral_clock;
use core::intrinsics::abort;

use self::Port::*;

#[path="../../util/ioreg.rs"]
#[macro_use] mod ioreg;

/// Available port names.
#[allow(missing_docs)]
#[derive(Clone, Copy)]
pub enum Port {
  PortA,
  PortB,
  PortC,
  PortD,
  PortE,
  PortF,
  PortG,
  PortH,
  PortI,
}

/// Pin functions.
#[allow(missing_docs)]
#[derive(Clone, Copy)]
pub enum Function {
  GPIOIn      = 0,
  GPIOOut     = 1,
  AltFunction = 2,
  Analog      = 3,
}

impl Port {
  fn clock(self) -> peripheral_clock::PeripheralClock {
    use hal::stm32f4::peripheral_clock::PeripheralClock::*;
    match self {
      PortA => GPIOAClock,
      PortB => GPIOBClock,
      PortC => GPIOCClock,
      PortD => GPIODClock,
      PortE => GPIOEClock,
      PortF => GPIOFClock,
      PortG => GPIOGClock,
      PortH => GPIOHClock,
      PortI => GPIOIClock,
    }
  }
}

/// Pin configuration
#[derive(Clone, Copy)]
pub struct Pin {
  /// Pin port, mcu-specific.
  pub port: Port,
  /// Pin number.
  pub pin: u8,
  /// Pin function, mcu-specific.
  pub function: Function,
}

impl Pin {
  /// Setup the pin.
  #[inline(always)]
  pub fn setup(&self) {
    use self::Function::*;
    use self::reg::GPIO_moder_mode as RegMode;

    self.port.clock().enable();  // TODO(farcaller): should be done once per port

    let offset = self.pin as usize;
    let gpreg = self.get_reg();

    let val = match self.function {
      GPIOOut => RegMode::Output,
      GPIOIn  => RegMode::Input,
      _       => unsafe { abort() },  // FIXME(farcaller): not implemented
    };

    gpreg.moder.set_mode(offset, val);
  }

  /// Toggles the GPIO value
  pub fn toggle(&self) {
    let reg = self.get_reg();
    let offset = self.pin as usize;

    reg.odr.set_od(offset, !reg.odr.od(offset));
  }

  fn get_reg(&self) -> &reg::GPIO {
    match self.port {
      PortA => &reg::GPIO_A,
      PortB => &reg::GPIO_B,
      PortC => &reg::GPIO_C,
      PortD => &reg::GPIO_D,
      PortE => &reg::GPIO_E,
      PortF => &reg::GPIO_F,
      PortG => &reg::GPIO_G,
      PortH => &reg::GPIO_H,
      PortI => &reg::GPIO_I,
    }
  }
}

impl Gpio for Pin {
  /// Sets output GPIO value to high.
  fn set_high(&self) {
    let offset = self.pin as usize;
    self.get_reg().bsrr.set_bs(offset, true);
  }

  /// Sets output GPIO value to low.
  fn set_low(&self) {
    let offset = self.pin as usize;
    self.get_reg().bsrr.set_br(offset, true);
  }

  /// Returns input GPIO level.
  fn level(&self) -> GpioLevel {
    let offset = self.pin as usize;
    let reg = self.get_reg();

    match reg.idr.id(offset) {
      false => GpioLevel::Low,
      _ => GpioLevel::High,
    }
  }

  /// Sets output GPIO direction.
  fn set_direction(&self, new_mode: GpioDirection) {
    // TODO(darayus): Verify that this works
    // TODO(darayus): Change the Pin.function field to the new mode
    use self::reg::GPIO_moder_mode as RegMode;
    let offset = self.pin as usize;
    let reg = self.get_reg();

    let val = match new_mode {
      GpioDirection::Out => RegMode::Output,
      GpioDirection::In  => RegMode::Input,
    };

    reg.moder.set_mode(offset, val);
  }
}

/// Alternate Function Pin
pub struct AlternateFunctionPin {
  /// Pin
  pub pin: Pin,
  /// Alternate function number
  pub alternate_function: u16,
}

impl AlternateFunctionPin {
  /// Create new alterate function pin
  pub fn new(pin: u8, port: Port, alt_func: u16) -> AlternateFunctionPin {
    AlternateFunctionPin {
      pin: Pin {
        pin: pin,
        port: port,
        function: Function::AltFunction,
      },
      alternate_function: alt_func,
    }
  }

  /// Setup the pin
  #[inline(always)]
  pub fn setup(&self) {
    assert!(match self.pin.function {
      Function::AltFunction => true,
      _ => false
    });

    use self::reg::GPIO_moder_mode as RegMode;

    self.pin.port.clock().enable();

    let offset = self.pin.pin as usize;
    let gpreg = self.pin.get_reg();

    gpreg.moder.set_mode(offset, RegMode::Alternate);
    if offset > 7 {
      gpreg.afrh.set_afrh(offset - 8, self.alternate_function as u32);
    } else {
      gpreg.afrl.set_afrl(offset, self.alternate_function as u32);
    }
  }

  /// Configure pin for 5V operating (I2C)
  #[inline(always)]
  pub fn five_volt(&self) {
    assert!(match self.pin.function {
      Function::AltFunction => true,
      _ => false
    });

    use self::reg::GPIO_otyper_ot as OutputTypeMode;
    use self::reg::GPIO_pupdr_pupd as PullUpDown;

    let offset = self.pin.pin as usize;
    let gpreg = self.pin.get_reg();

    gpreg.otyper.set_ot(offset, OutputTypeMode::OpenDrain);
    gpreg.pupdr.set_pupd(offset, PullUpDown::None);
  }
}

#[allow(dead_code)]
mod reg {
  use core::ops::Drop;
  use volatile_cell::VolatileCell;

  ioregs!(GPIO = {
    0x0 => reg32 moder {
      0..31 => mode[16] {
        0 => Input,
        1 => Output,
        2 => Alternate,
        3 => Analog
      }
    }
    0x04 => reg32 otyper {
      0..15 => ot[16] {
        0 => PushPull,
        1 => OpenDrain
      }
    }
    0x08 => reg32 ospeedr {
      0..31 => ospeed[16] {
        0 => Low,
        1 => Medium,
        2 => Fast,
        3 => High
      }
    }
    0x0c => reg32 pupdr {
      0..31 => pupd[16] {
        0 => None,
        1 => PullUp,
        2 => PullDown
      }
    }
    0x10 => reg32 idr {
      0..15 => id[16]: ro
    }
    0x14 => reg32 odr {
      0..15 => od[16]
    }
    0x18 => reg32 bsrr {
      0..15 => bs[16]: wo,
      16..31 => br[16]: wo
    }
    0x1c => reg32 lckr {
      0..15 => lck[16],
      16 => lckk
    }
    0x20 => reg32 afrl {
      0..31 => afrl[8]
    }
    0x24 => reg32 afrh {
      0..31 => afrh[8]
    }
  });

  extern {
    #[link_name="stm32f4_iomem_GPIOA"] pub static GPIO_A: GPIO;
    #[link_name="stm32f4_iomem_GPIOB"] pub static GPIO_B: GPIO;
    #[link_name="stm32f4_iomem_GPIOC"] pub static GPIO_C: GPIO;
    #[link_name="stm32f4_iomem_GPIOD"] pub static GPIO_D: GPIO;
    #[link_name="stm32f4_iomem_GPIOE"] pub static GPIO_E: GPIO;
    #[link_name="stm32f4_iomem_GPIOF"] pub static GPIO_F: GPIO;
    #[link_name="stm32f4_iomem_GPIOG"] pub static GPIO_G: GPIO;
    #[link_name="stm32f4_iomem_GPIOH"] pub static GPIO_H: GPIO;
    #[link_name="stm32f4_iomem_GPIOI"] pub static GPIO_I: GPIO;
    // define_reg!(GPIO_J: GPIO @ 0x40022400)
    // define_reg!(GPIO_K: GPIO @ 0x40022800)
  }
}
