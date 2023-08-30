//! stm32ral module for stm32h503

/// Number of priority bits implemented by the NVIC
pub const NVIC_PRIO_BITS: u8 = 4;

/// Interrupt-related magic for this device
pub mod interrupts;
pub use self::interrupts::Interrupt;
pub use self::interrupts::Interrupt as interrupt;

pub mod adc;
pub mod comp;
pub mod crc;
pub mod crs;
pub mod dac;
pub mod dbgmcu;
pub mod dts;
pub mod exti;
pub mod fdcan1;
pub mod flash;
pub mod gpdma;
pub mod gpio;
pub mod gtzc1;
pub mod hash;
pub mod i2c;
pub mod i3c;
pub mod icache;
pub mod iwdg;
pub mod lptim;
pub mod lpuart;
pub mod opamp1;
pub mod pwr;
pub mod ramcfg;
pub mod rcc;
pub mod rng;
pub mod rtc;
pub mod sbs;
pub mod spi;
pub mod tamp;
pub use super::instances::tim1;
pub use super::instances::tim2;
pub use super::instances::tim3;
pub use super::instances::tim6;
pub use super::instances::tim7;
pub mod usart;
pub mod usb;
pub mod wwdg;

#[cfg(all(feature = "rtic", not(feature = "nosync")))]
#[allow(non_snake_case)]
pub struct Peripherals {
    pub ADC: adc::Instance,
    pub COMP: comp::Instance,
    pub CRC: crc::Instance,
    pub CRS: crs::Instance,
    pub DAC: dac::Instance,
    pub DBGMCU: dbgmcu::Instance,
    pub DTS: dts::Instance,
    pub EXTI: exti::Instance,
    pub FDCAN1: fdcan1::Instance,
    pub FLASH: flash::Instance,
    pub GPIOA: gpio::Instance,
    pub GPIOB: gpio::Instance,
    pub GPIOC: gpio::Instance,
    pub GPIOD: gpio::Instance,
    pub GPIOH: gpio::Instance,
    pub GTZC1: gtzc1::Instance,
    pub GPDMA1: gpdma::Instance,
    pub GPDMA2: gpdma::Instance,
    pub HASH: hash::Instance,
    pub ICACHE: icache::Instance,
    pub IWDG: iwdg::Instance,
    pub I2C1: i2c::Instance,
    pub I2C2: i2c::Instance,
    pub I3C1: i3c::Instance,
    pub I3C2: i3c::Instance,
    pub LPTIM1: lptim::Instance,
    pub LPTIM2: lptim::Instance,
    pub LPUART: lpuart::Instance,
    pub OPAMP1: opamp1::Instance,
    pub PWR: pwr::Instance,
    pub RAMCFG: ramcfg::Instance,
    pub RCC: rcc::Instance,
    pub RNG: rng::Instance,
    pub RTC: rtc::Instance,
    pub SBS: sbs::Instance,
    pub SPI1: spi::Instance,
    pub SPI2: spi::Instance,
    pub SPI3: spi::Instance,
    pub TAMP: tamp::Instance,
    pub TIM1: tim1::Instance,
    pub TIM2: tim2::Instance,
    pub TIM3: tim3::Instance,
    pub TIM6: tim6::Instance,
    pub TIM7: tim7::Instance,
    pub USART1: usart::Instance,
    pub USART2: usart::Instance,
    pub USART3: usart::Instance,
    pub USB: usb::Instance,
    pub WWDG: wwdg::Instance,
}

#[cfg(all(feature = "rtic", feature = "nosync"))]
#[allow(non_snake_case)]
pub struct Peripherals {}

#[cfg(all(feature = "rtic", not(feature = "nosync")))]
impl Peripherals {
    pub unsafe fn steal() -> Self {
        Peripherals {
            ADC: adc::ADC::steal(),
            COMP: comp::COMP::steal(),
            CRC: crc::CRC::steal(),
            CRS: crs::CRS::steal(),
            DAC: dac::DAC::steal(),
            DBGMCU: dbgmcu::DBGMCU::steal(),
            DTS: dts::DTS::steal(),
            EXTI: exti::EXTI::steal(),
            FDCAN1: fdcan1::FDCAN1::steal(),
            FLASH: flash::FLASH::steal(),
            GPIOA: gpio::GPIOA::steal(),
            GPIOB: gpio::GPIOB::steal(),
            GPIOC: gpio::GPIOC::steal(),
            GPIOD: gpio::GPIOD::steal(),
            GPIOH: gpio::GPIOH::steal(),
            GTZC1: gtzc1::GTZC1::steal(),
            GPDMA1: gpdma::GPDMA1::steal(),
            GPDMA2: gpdma::GPDMA2::steal(),
            HASH: hash::HASH::steal(),
            ICACHE: icache::ICACHE::steal(),
            IWDG: iwdg::IWDG::steal(),
            I2C1: i2c::I2C1::steal(),
            I2C2: i2c::I2C2::steal(),
            I3C1: i3c::I3C1::steal(),
            I3C2: i3c::I3C2::steal(),
            LPTIM1: lptim::LPTIM1::steal(),
            LPTIM2: lptim::LPTIM2::steal(),
            LPUART: lpuart::LPUART::steal(),
            OPAMP1: opamp1::OPAMP1::steal(),
            PWR: pwr::PWR::steal(),
            RAMCFG: ramcfg::RAMCFG::steal(),
            RCC: rcc::RCC::steal(),
            RNG: rng::RNG::steal(),
            RTC: rtc::RTC::steal(),
            SBS: sbs::SBS::steal(),
            SPI1: spi::SPI1::steal(),
            SPI2: spi::SPI2::steal(),
            SPI3: spi::SPI3::steal(),
            TAMP: tamp::TAMP::steal(),
            TIM1: tim1::TIM1::steal(),
            TIM2: tim2::TIM2::steal(),
            TIM3: tim3::TIM3::steal(),
            TIM6: tim6::TIM6::steal(),
            TIM7: tim7::TIM7::steal(),
            USART1: usart::USART1::steal(),
            USART2: usart::USART2::steal(),
            USART3: usart::USART3::steal(),
            USB: usb::USB::steal(),
            WWDG: wwdg::WWDG::steal(),
        }
    }
}

#[cfg(all(feature = "rtic", feature = "nosync"))]
impl Peripherals {
    pub fn steal() -> Self {
        Peripherals {}
    }
}
