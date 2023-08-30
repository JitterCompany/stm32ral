#[cfg(any(feature="doc", feature="stm32f302", feature="stm32f303"))]
pub mod gpio;

#[cfg(any(feature="doc", feature="stm32f301", feature="stm32f302", feature="stm32f303", feature="stm32f373", feature="stm32f3x4"))]
pub mod tsc;

#[cfg(any(feature="doc", feature="stm32f301", feature="stm32f302", feature="stm32f303", feature="stm32f373"))]
pub mod crc;

#[cfg(any(feature="doc", feature="stm32f301", feature="stm32f302", feature="stm32f303", feature="stm32f373"))]
pub mod dma;

#[cfg(any(feature="doc", feature="stm32f301", feature="stm32f373"))]
pub mod tim16_f301_f373;

#[cfg(any(feature="doc", feature="stm32f301", feature="stm32f373"))]
pub mod tim17_f301_f373;

#[cfg(any(feature="doc", feature="stm32f301", feature="stm32f373", feature="stm32f3x4"))]
pub mod usart_f301_f373_f3x4;

#[cfg(any(feature="doc", feature="stm32f302", feature="stm32f303"))]
pub mod usart_f302_f303;

#[cfg(any(feature="doc", feature="stm32f302", feature="stm32f303"))]
pub mod spi;

#[cfg(any(feature="doc", feature="stm32f301", feature="stm32f373"))]
pub mod cec;

#[cfg(any(feature="doc", feature="stm32f301", feature="stm32f373"))]
pub mod pwr;

#[cfg(any(feature="doc", feature="stm32f301", feature="stm32f302", feature="stm32f303", feature="stm32f373", feature="stm32f3x4"))]
pub mod can;

#[cfg(any(feature="doc", feature="stm32f301", feature="stm32f302", feature="stm32f303", feature="stm32f373"))]
pub mod usb;

#[cfg(any(feature="doc", feature="stm32f301", feature="stm32f302", feature="stm32f303", feature="stm32f3x4"))]
pub mod i2c;

#[cfg(any(feature="doc", feature="stm32f301", feature="stm32f302", feature="stm32f303", feature="stm32f373", feature="stm32f3x4"))]
pub mod iwdg;

#[cfg(any(feature="doc", feature="stm32f301", feature="stm32f302", feature="stm32f303", feature="stm32f373", feature="stm32f3x4"))]
pub mod wwdg;

#[cfg(any(feature="doc", feature="stm32f301", feature="stm32f373"))]
pub mod rtc_f301_f373;

#[cfg(any(feature="doc", feature="stm32f301", feature="stm32f373"))]
pub mod sdadc;

#[cfg(any(feature="doc", feature="stm32f301", feature="stm32f373"))]
pub mod tim6_f301_f373;

#[cfg(any(feature="doc", feature="stm32f301", feature="stm32f373"))]
pub mod tim7_f301_f373;

#[cfg(any(feature="doc", feature="stm32f301", feature="stm32f373"))]
pub mod tim18;

#[cfg(any(feature="doc", feature="stm32f301", feature="stm32f302", feature="stm32f303", feature="stm32f373", feature="stm32f3x4"))]
pub mod dbgmcu;

#[cfg(any(feature="doc", feature="stm32f301", feature="stm32f302", feature="stm32f303", feature="stm32f373", feature="stm32f3x4"))]
pub mod nvic;

#[cfg(any(feature="doc", feature="stm32f301", feature="stm32f302", feature="stm32f303", feature="stm32f373", feature="stm32f3x4"))]
pub mod fpu;

#[cfg(any(feature="doc", feature="stm32f301", feature="stm32f302", feature="stm32f303", feature="stm32f373", feature="stm32f3x4"))]
pub mod mpu;

#[cfg(any(feature="doc", feature="stm32f301", feature="stm32f302", feature="stm32f303", feature="stm32f373", feature="stm32f3x4"))]
pub mod stk;

#[cfg(any(feature="doc", feature="stm32f301", feature="stm32f302", feature="stm32f303", feature="stm32f373", feature="stm32f3x4"))]
pub mod scb;

#[cfg(any(feature="doc", feature="stm32f301", feature="stm32f302", feature="stm32f303", feature="stm32f373", feature="stm32f3x4"))]
pub mod nvic_stir;

#[cfg(any(feature="doc", feature="stm32f301", feature="stm32f302", feature="stm32f303", feature="stm32f373", feature="stm32f3x4"))]
pub mod fpu_cpacr;

#[cfg(any(feature="doc", feature="stm32f301", feature="stm32f302", feature="stm32f303", feature="stm32f373", feature="stm32f3x4"))]
pub mod scb_actrl;

#[cfg(any(feature="doc", feature="stm32f302", feature="stm32f303", feature="stm32f3x4"))]
pub mod exti;

#[cfg(any(feature="doc", feature="stm32f301", feature="stm32f302", feature="stm32f303", feature="stm32f3x4"))]
pub mod tim1;

#[cfg(any(feature="doc", feature="stm32f302", feature="stm32f303"))]
pub mod tim20;

#[cfg(any(feature="doc", feature="stm32f301", feature="stm32f3x4"))]
pub mod opamp;

#[cfg(any(feature="doc", feature="stm32f302", feature="stm32f303"))]
pub mod flash;

#[cfg(any(feature="doc", feature="stm32f302", feature="stm32f303", feature="stm32f3x4"))]
pub mod tim2;

#[cfg(any(feature="doc", feature="stm32f302", feature="stm32f303"))]
pub mod tim3;

#[cfg(any(feature="doc", feature="stm32f302", feature="stm32f303"))]
pub mod tim4;

#[cfg(any(feature="doc", feature="stm32f302", feature="stm32f303", feature="stm32f3x4"))]
pub mod tim15;

#[cfg(any(feature="doc", feature="stm32f302", feature="stm32f303", feature="stm32f3x4"))]
pub mod tim16_f302_f303_f3x4;

#[cfg(any(feature="doc", feature="stm32f302", feature="stm32f303", feature="stm32f3x4"))]
pub mod tim17_f302_f303_f3x4;

#[cfg(any(feature="doc", feature="stm32f302", feature="stm32f303", feature="stm32f3x4"))]
pub mod rtc_f302_f303_f3x4;

#[cfg(any(feature="doc", feature="stm32f302", feature="stm32f303", feature="stm32f3x4"))]
pub mod tim6_f302_f303_f3x4;

#[cfg(any(feature="doc", feature="stm32f302", feature="stm32f303", feature="stm32f3x4"))]
pub mod tim7_f302_f303_f3x4;

#[cfg(any(feature="doc", feature="stm32f303", feature="stm32f3x4"))]
pub mod dac;

#[cfg(any(feature="doc", feature="stm32f302", feature="stm32f303"))]
pub mod tim8;

#[cfg(any(feature="doc", feature="stm32f302", feature="stm32f3x4"))]
pub mod adc;

