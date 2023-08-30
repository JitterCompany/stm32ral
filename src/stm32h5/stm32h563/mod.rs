//! stm32ral module for stm32h563

/// Number of priority bits implemented by the NVIC
pub const NVIC_PRIO_BITS: u8 = 4;

/// Interrupt-related magic for this device
pub mod interrupts;
pub use self::interrupts::Interrupt;
pub use self::interrupts::Interrupt as interrupt;

pub use super::instances::adc;
pub use super::instances::cordic;
pub use super::instances::crc;
pub use super::instances::crs;
pub use super::instances::dac;
pub use super::instances::dbgmcu;
pub use super::instances::dcache;
pub use super::instances::dcmi;
pub use super::instances::dlyb;
pub use super::instances::dts;
pub use super::instances::eth;
pub use super::instances::exti;
pub use super::instances::fdcan;
pub use super::instances::flash;
pub use super::instances::fmac;
pub use super::instances::fmc;
pub use super::instances::gtzc1_mpcbb1;
pub use super::instances::gtzc1_mpcbb2;
pub use super::instances::gtzc1_mpcbb3;
pub mod gtzc1_tzic;
pub mod gtzc1_tzsc;
pub use super::instances::gpdma;
pub use super::instances::gpio;
pub use super::instances::gpioi;
pub use super::instances::hash;
pub use super::instances::i2c;
pub use super::instances::i3c;
pub use super::instances::icache;
pub use super::instances::iwdg;
pub use super::instances::lptim;
pub use super::instances::lpuart;
pub use super::instances::octospi;
pub use super::instances::pssi;
pub use super::instances::pwr;
pub use super::instances::ramcfg;
pub use super::instances::rtc;
pub use super::instances::s;
pub use super::instances::sbs;
pub use super::instances::sdmmc;
pub use super::instances::sec_tim1;
pub use super::instances::sec_tim12;
pub use super::instances::sec_tim13;
pub use super::instances::sec_tim14;
pub use super::instances::sec_tim15;
pub use super::instances::sec_tim16;
pub use super::instances::sec_tim17;
pub use super::instances::sec_tim2;
pub use super::instances::sec_tim3;
pub use super::instances::sec_tim4;
pub use super::instances::sec_tim5;
pub use super::instances::sec_tim6;
pub use super::instances::sec_tim7;
pub use super::instances::sec_tim8;
pub use super::instances::spi;
pub use super::instances::tamp;
pub use super::instances::tim1;
pub use super::instances::tim12;
pub use super::instances::tim13;
pub use super::instances::tim14;
pub use super::instances::tim15;
pub use super::instances::tim16;
pub use super::instances::tim17;
pub use super::instances::tim2;
pub use super::instances::tim3;
pub use super::instances::tim4;
pub use super::instances::tim5;
pub use super::instances::tim6;
pub use super::instances::tim7;
pub use super::instances::tim8;
pub use super::instances::ucpd1;
pub use super::instances::usart;
pub use super::instances::usb;
pub mod rcc;
pub use super::instances::rng;
pub use super::instances::vrefbuf;
pub use super::instances::wwdg;

#[cfg(all(feature = "rtic", not(feature = "nosync")))]
#[allow(non_snake_case)]
pub struct Peripherals {
    pub ADC1: adc::Instance,
    pub SEC_ADC1: adc::Instance,
    pub ADC2: adc::Instance,
    pub SEC_ADC2: adc::Instance,
    pub CRC: crc::Instance,
    pub SEC_CRC: crc::Instance,
    pub CRS: crs::Instance,
    pub SEC_CRS: crs::Instance,
    pub CORDIC: cordic::Instance,
    pub SEC_CORDIC: cordic::Instance,
    pub DAC: dac::Instance,
    pub SEC_DAC: dac::Instance,
    pub DBGMCU: dbgmcu::Instance,
    pub DCACHE: dcache::Instance,
    pub SEC_DCACHE: dcache::Instance,
    pub DCMI: dcmi::Instance,
    pub SEC_DCMI: dcmi::Instance,
    pub DLYBOS1: dlyb::Instance,
    pub SEC_DLYBOS1: dlyb::Instance,
    pub DLYBSD1: dlyb::Instance,
    pub SEC_DLYBSD1: dlyb::Instance,
    pub DLYBSD2: dlyb::Instance,
    pub SEC_DLYBSD2: dlyb::Instance,
    pub DTS: dts::Instance,
    pub SEC_DTS: dts::Instance,
    pub ETH: eth::Instance,
    pub SEC_ETH: eth::Instance,
    pub EXTI: exti::Instance,
    pub SEC_EXTI: exti::Instance,
    pub FDCAN1: fdcan::Instance,
    pub SEC_FDCAN1: fdcan::Instance,
    pub FDCAN2: fdcan::Instance,
    pub SEC_FDCAN2: fdcan::Instance,
    pub FLASH: flash::Instance,
    pub SEC_FLASH: flash::Instance,
    pub FMAC: fmac::Instance,
    pub SEC_FMAC: fmac::Instance,
    pub FMC: fmc::Instance,
    pub SEC_FMC: fmc::Instance,
    pub GTZC1_MPCBB1: gtzc1_mpcbb1::Instance,
    pub SEC_GTZC1_MPCBB1: gtzc1_mpcbb1::Instance,
    pub GTZC1_MPCBB2: gtzc1_mpcbb2::Instance,
    pub SEC_GTZC1_MPCBB2: gtzc1_mpcbb2::Instance,
    pub GTZC1_MPCBB3: gtzc1_mpcbb3::Instance,
    pub SEC_GTZC1_MPCBB3: gtzc1_mpcbb3::Instance,
    pub GTZC1_TZIC: gtzc1_tzic::Instance,
    pub SEC_GTZC1_TZIC: gtzc1_tzic::Instance,
    pub GTZC1_TZSC: gtzc1_tzsc::Instance,
    pub SEC_GTZC1_TZSC: gtzc1_tzsc::Instance,
    pub GPDMA1: gpdma::Instance,
    pub SEC_GPDMA1: gpdma::Instance,
    pub GPDMA2: gpdma::Instance,
    pub SEC_GPDMA2: gpdma::Instance,
    pub GPIOA: gpio::Instance,
    pub SEC_GPIOA: gpio::Instance,
    pub GPIOB: gpio::Instance,
    pub SEC_GPIOB: gpio::Instance,
    pub GPIOC: gpio::Instance,
    pub SEC_GPIOC: gpio::Instance,
    pub GPIOD: gpio::Instance,
    pub SEC_GPIOD: gpio::Instance,
    pub GPIOE: gpio::Instance,
    pub SEC_GPIOE: gpio::Instance,
    pub GPIOF: gpio::Instance,
    pub SEC_GPIOF: gpio::Instance,
    pub GPIOG: gpio::Instance,
    pub SEC_GPIOG: gpio::Instance,
    pub GPIOH: gpio::Instance,
    pub SEC_GPIOH: gpio::Instance,
    pub GPIOI: gpioi::Instance,
    pub SEC_GPIOI: gpioi::Instance,
    pub HASH: hash::Instance,
    pub SEC_HASH: hash::Instance,
    pub ICACHE: icache::Instance,
    pub SEC_ICACHE: icache::Instance,
    pub IWDG: iwdg::Instance,
    pub SEC_IWDG: iwdg::Instance,
    pub I2C1: i2c::Instance,
    pub SEC_I2C1: i2c::Instance,
    pub I2C2: i2c::Instance,
    pub SEC_I2C2: i2c::Instance,
    pub I3C: i3c::Instance,
    pub SEC_I3C: i3c::Instance,
    pub LPTIM1: lptim::Instance,
    pub SEC_LPTIM1: lptim::Instance,
    pub LPTIM2: lptim::Instance,
    pub SEC_LPTIM2: lptim::Instance,
    pub LPTIM3: lptim::Instance,
    pub SEC_LPTIM3: lptim::Instance,
    pub LPTIM4: lptim::Instance,
    pub SEC_LPTIM4: lptim::Instance,
    pub LPTIM5: lptim::Instance,
    pub SEC_LPTIM5: lptim::Instance,
    pub LPTIM6: lptim::Instance,
    pub SEC_LPTIM6: lptim::Instance,
    pub LPUART: lpuart::Instance,
    pub SEC_LPUART1: lpuart::Instance,
    pub OCTOSPI: octospi::Instance,
    pub SEC_OCTOSPI: octospi::Instance,
    pub PWR: pwr::Instance,
    pub SEC_PWR: pwr::Instance,
    pub RTC: rtc::Instance,
    pub SEC_RTC: rtc::Instance,
    pub SAI1: s::Instance,
    pub SEC_SAI1: s::Instance,
    pub SAI2: s::Instance,
    pub SEC_SAI2: s::Instance,
    pub SBS: sbs::Instance,
    pub SEC_SBS: sbs::Instance,
    pub SDMMC1: sdmmc::Instance,
    pub SEC_SDMMC1: sdmmc::Instance,
    pub SDMMC2: sdmmc::Instance,
    pub SEC_SDMMC2: sdmmc::Instance,
    pub SPI1: spi::Instance,
    pub SEC_SPI1: spi::Instance,
    pub SPI2: spi::Instance,
    pub SEC_SPI2: spi::Instance,
    pub SPI3: spi::Instance,
    pub SEC_SPI3: spi::Instance,
    pub SPI4: spi::Instance,
    pub SEC_SPI4: spi::Instance,
    pub SPI5: spi::Instance,
    pub SEC_SPI5: spi::Instance,
    pub SPI6: spi::Instance,
    pub SEC_SPI6: spi::Instance,
    pub TAMP: tamp::Instance,
    pub SEC_TAMP: tamp::Instance,
    pub TIM1: tim1::Instance,
    pub SEC_TIM1: sec_tim1::Instance,
    pub TIM2: tim2::Instance,
    pub SEC_TIM2: sec_tim2::Instance,
    pub TIM3: tim3::Instance,
    pub SEC_TIM3: sec_tim3::Instance,
    pub TIM4: tim4::Instance,
    pub SEC_TIM4: sec_tim4::Instance,
    pub TIM5: tim5::Instance,
    pub SEC_TIM5: sec_tim5::Instance,
    pub TIM6: tim6::Instance,
    pub SEC_TIM6: sec_tim6::Instance,
    pub TIM7: tim7::Instance,
    pub SEC_TIM7: sec_tim7::Instance,
    pub TIM8: tim8::Instance,
    pub SEC_TIM8: sec_tim8::Instance,
    pub TIM12: tim12::Instance,
    pub SEC_TIM12: sec_tim12::Instance,
    pub TIM13: tim13::Instance,
    pub SEC_TIM13: sec_tim13::Instance,
    pub TIM14: tim14::Instance,
    pub SEC_TIM14: sec_tim14::Instance,
    pub TIM15: tim15::Instance,
    pub SEC_TIM15: sec_tim15::Instance,
    pub TIM16: tim16::Instance,
    pub SEC_TIM16: sec_tim16::Instance,
    pub TIM17: tim17::Instance,
    pub SEC_TIM17: sec_tim17::Instance,
    pub UCPD1: ucpd1::Instance,
    pub SEC_UCPD1: ucpd1::Instance,
    pub USART1: usart::Instance,
    pub SEC_USART1: usart::Instance,
    pub USART2: usart::Instance,
    pub SEC_USART2: usart::Instance,
    pub USART3: usart::Instance,
    pub SEC_USART3: usart::Instance,
    pub UART4: usart::Instance,
    pub SEC_UART4: usart::Instance,
    pub UART5: usart::Instance,
    pub SEC_UART5: usart::Instance,
    pub USART6: usart::Instance,
    pub SEC_USART6: usart::Instance,
    pub UART7: usart::Instance,
    pub SEC_UART7: usart::Instance,
    pub UART8: usart::Instance,
    pub SEC_UART8: usart::Instance,
    pub UART9: usart::Instance,
    pub SEC_UART9: usart::Instance,
    pub USART10: usart::Instance,
    pub SEC_USART10: usart::Instance,
    pub USART11: usart::Instance,
    pub SEC_USART11: usart::Instance,
    pub UART12: usart::Instance,
    pub SEC_UART12: usart::Instance,
    pub USB: usb::Instance,
    pub SEC_USB: usb::Instance,
    pub PSSI: pssi::Instance,
    pub SEC_PSSI: pssi::Instance,
    pub RAMCFG: ramcfg::Instance,
    pub SEC_RAMCFG: ramcfg::Instance,
    pub RCC: rcc::Instance,
    pub SEC_RCC: rcc::Instance,
    pub RNG: rng::Instance,
    pub SEC_RNG: rng::Instance,
    pub VREFBUF: vrefbuf::Instance,
    pub SEC_VREFBUF: vrefbuf::Instance,
    pub WWDG: wwdg::Instance,
    pub SEC_WWDG: wwdg::Instance,
}

#[cfg(all(feature = "rtic", feature = "nosync"))]
#[allow(non_snake_case)]
pub struct Peripherals {}

#[cfg(all(feature = "rtic", not(feature = "nosync")))]
impl Peripherals {
    pub unsafe fn steal() -> Self {
        Peripherals {
            ADC1: adc::ADC1::steal(),
            SEC_ADC1: adc::SEC_ADC1::steal(),
            ADC2: adc::ADC2::steal(),
            SEC_ADC2: adc::SEC_ADC2::steal(),
            CRC: crc::CRC::steal(),
            SEC_CRC: crc::SEC_CRC::steal(),
            CRS: crs::CRS::steal(),
            SEC_CRS: crs::SEC_CRS::steal(),
            CORDIC: cordic::CORDIC::steal(),
            SEC_CORDIC: cordic::SEC_CORDIC::steal(),
            DAC: dac::DAC::steal(),
            SEC_DAC: dac::SEC_DAC::steal(),
            DBGMCU: dbgmcu::DBGMCU::steal(),
            DCACHE: dcache::DCACHE::steal(),
            SEC_DCACHE: dcache::SEC_DCACHE::steal(),
            DCMI: dcmi::DCMI::steal(),
            SEC_DCMI: dcmi::SEC_DCMI::steal(),
            DLYBOS1: dlyb::DLYBOS1::steal(),
            SEC_DLYBOS1: dlyb::SEC_DLYBOS1::steal(),
            DLYBSD1: dlyb::DLYBSD1::steal(),
            SEC_DLYBSD1: dlyb::SEC_DLYBSD1::steal(),
            DLYBSD2: dlyb::DLYBSD2::steal(),
            SEC_DLYBSD2: dlyb::SEC_DLYBSD2::steal(),
            DTS: dts::DTS::steal(),
            SEC_DTS: dts::SEC_DTS::steal(),
            ETH: eth::ETH::steal(),
            SEC_ETH: eth::SEC_ETH::steal(),
            EXTI: exti::EXTI::steal(),
            SEC_EXTI: exti::SEC_EXTI::steal(),
            FDCAN1: fdcan::FDCAN1::steal(),
            SEC_FDCAN1: fdcan::SEC_FDCAN1::steal(),
            FDCAN2: fdcan::FDCAN2::steal(),
            SEC_FDCAN2: fdcan::SEC_FDCAN2::steal(),
            FLASH: flash::FLASH::steal(),
            SEC_FLASH: flash::SEC_FLASH::steal(),
            FMAC: fmac::FMAC::steal(),
            SEC_FMAC: fmac::SEC_FMAC::steal(),
            FMC: fmc::FMC::steal(),
            SEC_FMC: fmc::SEC_FMC::steal(),
            GTZC1_MPCBB1: gtzc1_mpcbb1::GTZC1_MPCBB1::steal(),
            SEC_GTZC1_MPCBB1: gtzc1_mpcbb1::SEC_GTZC1_MPCBB1::steal(),
            GTZC1_MPCBB2: gtzc1_mpcbb2::GTZC1_MPCBB2::steal(),
            SEC_GTZC1_MPCBB2: gtzc1_mpcbb2::SEC_GTZC1_MPCBB2::steal(),
            GTZC1_MPCBB3: gtzc1_mpcbb3::GTZC1_MPCBB3::steal(),
            SEC_GTZC1_MPCBB3: gtzc1_mpcbb3::SEC_GTZC1_MPCBB3::steal(),
            GTZC1_TZIC: gtzc1_tzic::GTZC1_TZIC::steal(),
            SEC_GTZC1_TZIC: gtzc1_tzic::SEC_GTZC1_TZIC::steal(),
            GTZC1_TZSC: gtzc1_tzsc::GTZC1_TZSC::steal(),
            SEC_GTZC1_TZSC: gtzc1_tzsc::SEC_GTZC1_TZSC::steal(),
            GPDMA1: gpdma::GPDMA1::steal(),
            SEC_GPDMA1: gpdma::SEC_GPDMA1::steal(),
            GPDMA2: gpdma::GPDMA2::steal(),
            SEC_GPDMA2: gpdma::SEC_GPDMA2::steal(),
            GPIOA: gpio::GPIOA::steal(),
            SEC_GPIOA: gpio::SEC_GPIOA::steal(),
            GPIOB: gpio::GPIOB::steal(),
            SEC_GPIOB: gpio::SEC_GPIOB::steal(),
            GPIOC: gpio::GPIOC::steal(),
            SEC_GPIOC: gpio::SEC_GPIOC::steal(),
            GPIOD: gpio::GPIOD::steal(),
            SEC_GPIOD: gpio::SEC_GPIOD::steal(),
            GPIOE: gpio::GPIOE::steal(),
            SEC_GPIOE: gpio::SEC_GPIOE::steal(),
            GPIOF: gpio::GPIOF::steal(),
            SEC_GPIOF: gpio::SEC_GPIOF::steal(),
            GPIOG: gpio::GPIOG::steal(),
            SEC_GPIOG: gpio::SEC_GPIOG::steal(),
            GPIOH: gpio::GPIOH::steal(),
            SEC_GPIOH: gpio::SEC_GPIOH::steal(),
            GPIOI: gpioi::GPIOI::steal(),
            SEC_GPIOI: gpioi::SEC_GPIOI::steal(),
            HASH: hash::HASH::steal(),
            SEC_HASH: hash::SEC_HASH::steal(),
            ICACHE: icache::ICACHE::steal(),
            SEC_ICACHE: icache::SEC_ICACHE::steal(),
            IWDG: iwdg::IWDG::steal(),
            SEC_IWDG: iwdg::SEC_IWDG::steal(),
            I2C1: i2c::I2C1::steal(),
            SEC_I2C1: i2c::SEC_I2C1::steal(),
            I2C2: i2c::I2C2::steal(),
            SEC_I2C2: i2c::SEC_I2C2::steal(),
            I3C: i3c::I3C::steal(),
            SEC_I3C: i3c::SEC_I3C::steal(),
            LPTIM1: lptim::LPTIM1::steal(),
            SEC_LPTIM1: lptim::SEC_LPTIM1::steal(),
            LPTIM2: lptim::LPTIM2::steal(),
            SEC_LPTIM2: lptim::SEC_LPTIM2::steal(),
            LPTIM3: lptim::LPTIM3::steal(),
            SEC_LPTIM3: lptim::SEC_LPTIM3::steal(),
            LPTIM4: lptim::LPTIM4::steal(),
            SEC_LPTIM4: lptim::SEC_LPTIM4::steal(),
            LPTIM5: lptim::LPTIM5::steal(),
            SEC_LPTIM5: lptim::SEC_LPTIM5::steal(),
            LPTIM6: lptim::LPTIM6::steal(),
            SEC_LPTIM6: lptim::SEC_LPTIM6::steal(),
            LPUART: lpuart::LPUART::steal(),
            SEC_LPUART1: lpuart::SEC_LPUART1::steal(),
            OCTOSPI: octospi::OCTOSPI::steal(),
            SEC_OCTOSPI: octospi::SEC_OCTOSPI::steal(),
            PWR: pwr::PWR::steal(),
            SEC_PWR: pwr::SEC_PWR::steal(),
            RTC: rtc::RTC::steal(),
            SEC_RTC: rtc::SEC_RTC::steal(),
            SAI1: s::SAI1::steal(),
            SEC_SAI1: s::SEC_SAI1::steal(),
            SAI2: s::SAI2::steal(),
            SEC_SAI2: s::SEC_SAI2::steal(),
            SBS: sbs::SBS::steal(),
            SEC_SBS: sbs::SEC_SBS::steal(),
            SDMMC1: sdmmc::SDMMC1::steal(),
            SEC_SDMMC1: sdmmc::SEC_SDMMC1::steal(),
            SDMMC2: sdmmc::SDMMC2::steal(),
            SEC_SDMMC2: sdmmc::SEC_SDMMC2::steal(),
            SPI1: spi::SPI1::steal(),
            SEC_SPI1: spi::SEC_SPI1::steal(),
            SPI2: spi::SPI2::steal(),
            SEC_SPI2: spi::SEC_SPI2::steal(),
            SPI3: spi::SPI3::steal(),
            SEC_SPI3: spi::SEC_SPI3::steal(),
            SPI4: spi::SPI4::steal(),
            SEC_SPI4: spi::SEC_SPI4::steal(),
            SPI5: spi::SPI5::steal(),
            SEC_SPI5: spi::SEC_SPI5::steal(),
            SPI6: spi::SPI6::steal(),
            SEC_SPI6: spi::SEC_SPI6::steal(),
            TAMP: tamp::TAMP::steal(),
            SEC_TAMP: tamp::SEC_TAMP::steal(),
            TIM1: tim1::TIM1::steal(),
            SEC_TIM1: sec_tim1::SEC_TIM1::steal(),
            TIM2: tim2::TIM2::steal(),
            SEC_TIM2: sec_tim2::SEC_TIM2::steal(),
            TIM3: tim3::TIM3::steal(),
            SEC_TIM3: sec_tim3::SEC_TIM3::steal(),
            TIM4: tim4::TIM4::steal(),
            SEC_TIM4: sec_tim4::SEC_TIM4::steal(),
            TIM5: tim5::TIM5::steal(),
            SEC_TIM5: sec_tim5::SEC_TIM5::steal(),
            TIM6: tim6::TIM6::steal(),
            SEC_TIM6: sec_tim6::SEC_TIM6::steal(),
            TIM7: tim7::TIM7::steal(),
            SEC_TIM7: sec_tim7::SEC_TIM7::steal(),
            TIM8: tim8::TIM8::steal(),
            SEC_TIM8: sec_tim8::SEC_TIM8::steal(),
            TIM12: tim12::TIM12::steal(),
            SEC_TIM12: sec_tim12::SEC_TIM12::steal(),
            TIM13: tim13::TIM13::steal(),
            SEC_TIM13: sec_tim13::SEC_TIM13::steal(),
            TIM14: tim14::TIM14::steal(),
            SEC_TIM14: sec_tim14::SEC_TIM14::steal(),
            TIM15: tim15::TIM15::steal(),
            SEC_TIM15: sec_tim15::SEC_TIM15::steal(),
            TIM16: tim16::TIM16::steal(),
            SEC_TIM16: sec_tim16::SEC_TIM16::steal(),
            TIM17: tim17::TIM17::steal(),
            SEC_TIM17: sec_tim17::SEC_TIM17::steal(),
            UCPD1: ucpd1::UCPD1::steal(),
            SEC_UCPD1: ucpd1::SEC_UCPD1::steal(),
            USART1: usart::USART1::steal(),
            SEC_USART1: usart::SEC_USART1::steal(),
            USART2: usart::USART2::steal(),
            SEC_USART2: usart::SEC_USART2::steal(),
            USART3: usart::USART3::steal(),
            SEC_USART3: usart::SEC_USART3::steal(),
            UART4: usart::UART4::steal(),
            SEC_UART4: usart::SEC_UART4::steal(),
            UART5: usart::UART5::steal(),
            SEC_UART5: usart::SEC_UART5::steal(),
            USART6: usart::USART6::steal(),
            SEC_USART6: usart::SEC_USART6::steal(),
            UART7: usart::UART7::steal(),
            SEC_UART7: usart::SEC_UART7::steal(),
            UART8: usart::UART8::steal(),
            SEC_UART8: usart::SEC_UART8::steal(),
            UART9: usart::UART9::steal(),
            SEC_UART9: usart::SEC_UART9::steal(),
            USART10: usart::USART10::steal(),
            SEC_USART10: usart::SEC_USART10::steal(),
            USART11: usart::USART11::steal(),
            SEC_USART11: usart::SEC_USART11::steal(),
            UART12: usart::UART12::steal(),
            SEC_UART12: usart::SEC_UART12::steal(),
            USB: usb::USB::steal(),
            SEC_USB: usb::SEC_USB::steal(),
            PSSI: pssi::PSSI::steal(),
            SEC_PSSI: pssi::SEC_PSSI::steal(),
            RAMCFG: ramcfg::RAMCFG::steal(),
            SEC_RAMCFG: ramcfg::SEC_RAMCFG::steal(),
            RCC: rcc::RCC::steal(),
            SEC_RCC: rcc::SEC_RCC::steal(),
            RNG: rng::RNG::steal(),
            SEC_RNG: rng::SEC_RNG::steal(),
            VREFBUF: vrefbuf::VREFBUF::steal(),
            SEC_VREFBUF: vrefbuf::SEC_VREFBUF::steal(),
            WWDG: wwdg::WWDG::steal(),
            SEC_WWDG: wwdg::SEC_WWDG::steal(),
        }
    }
}

#[cfg(all(feature = "rtic", feature = "nosync"))]
impl Peripherals {
    pub fn steal() -> Self {
        Peripherals {}
    }
}
