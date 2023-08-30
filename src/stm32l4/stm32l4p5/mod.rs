//! stm32ral module for stm32l4p5

/// Number of priority bits implemented by the NVIC
pub const NVIC_PRIO_BITS: u8 = 4;

/// Interrupt-related magic for this device
pub mod interrupts;
pub use self::interrupts::Interrupt;
pub use self::interrupts::Interrupt as interrupt;

pub mod dac;
pub use super::instances::dma_l4p5_l4r5_l4r9 as dma;
pub mod dmamux;
pub use super::instances::crc;
pub mod iwdg;
pub mod ltdc;
pub mod tsc;
pub use super::instances::wwdg;
pub mod comp;
pub mod firewall;
pub use super::instances::i2c_l4p5_l4r5_l4r9_l4x6 as i2c;
pub mod adc1;
pub mod adc_common;
pub mod aes;
pub mod dbgmcu;
pub mod dfsdm1;
pub mod flash;
pub mod octospi;
pub mod pwr;
pub mod rcc;
pub mod rng;
pub mod syscfg;
pub use super::instances::gpio_l4p5_l4r5_l4r9_l4x6 as gpio;
pub use super::instances::gpioi;
pub mod exti;
pub mod lptim;
pub mod lpuart1;
pub mod sai;
pub mod sdmmc;
pub mod spi;
pub mod tim1;
pub mod tim15;
pub mod tim16;
pub mod tim17;
pub mod tim2;
pub mod tim3;
pub mod tim4;
pub mod tim5;
pub mod tim6;
pub mod tim7;
pub mod tim8;
pub mod usart;
pub use super::instances::vrefbuf;
pub mod can1;
pub mod otg_fs_device;
pub mod otg_fs_global;
pub mod otg_fs_host;
pub mod otg_fs_pwrclk;
pub mod rtc;
pub use super::instances::opamp_l412_l4p5_l4r5_l4r9 as opamp;
pub mod fmc;
pub use super::instances::crs;
pub use super::instances::nvic_l4p5_l4r5 as nvic;
pub mod dcmi;
pub mod dma2d;
pub mod hash;
pub mod octospim;
pub use super::instances::fpu;
pub mod mpu;
pub use super::instances::fpu_cpacr;
pub use super::instances::nvic_stir;
pub use super::instances::scb_actrl;
pub use super::instances::scb_l4p5_l4r5_l4r9 as scb;
pub use super::instances::stk;
pub mod pssi;

#[cfg(all(feature = "rtic", not(feature = "nosync")))]
#[allow(non_snake_case)]
pub struct Peripherals {
    pub DAC: dac::Instance,
    pub DMA1: dma::Instance,
    pub DMA2: dma::Instance,
    pub DMAMUX: dmamux::Instance,
    pub CRC: crc::Instance,
    pub LTDC: ltdc::Instance,
    pub TSC: tsc::Instance,
    pub IWDG: iwdg::Instance,
    pub WWDG: wwdg::Instance,
    pub COMP: comp::Instance,
    pub FIREWALL: firewall::Instance,
    pub I2C1: i2c::Instance,
    pub I2C2: i2c::Instance,
    pub I2C3: i2c::Instance,
    pub I2C4: i2c::Instance,
    pub FLASH: flash::Instance,
    pub DBGMCU: dbgmcu::Instance,
    pub OCTOSPI1: octospi::Instance,
    pub OCTOSPI2: octospi::Instance,
    pub RCC: rcc::Instance,
    pub PWR: pwr::Instance,
    pub SYSCFG: syscfg::Instance,
    pub DFSDM1: dfsdm1::Instance,
    pub RNG: rng::Instance,
    pub AES: aes::Instance,
    pub ADC1: adc1::Instance,
    pub ADC_Common: adc_common::Instance,
    pub GPIOA: gpio::Instance,
    pub GPIOB: gpio::Instance,
    pub GPIOC: gpio::Instance,
    pub GPIOD: gpio::Instance,
    pub GPIOE: gpio::Instance,
    pub GPIOF: gpio::Instance,
    pub GPIOG: gpio::Instance,
    pub GPIOH: gpio::Instance,
    pub GPIOI: gpioi::Instance,
    pub SAI1: sai::Instance,
    pub SAI2: sai::Instance,
    pub TIM2: tim2::Instance,
    pub TIM3: tim3::Instance,
    pub TIM4: tim4::Instance,
    pub TIM5: tim5::Instance,
    pub TIM15: tim15::Instance,
    pub TIM16: tim16::Instance,
    pub TIM17: tim17::Instance,
    pub TIM1: tim1::Instance,
    pub TIM8: tim8::Instance,
    pub TIM6: tim6::Instance,
    pub TIM7: tim7::Instance,
    pub LPTIM1: lptim::Instance,
    pub LPTIM2: lptim::Instance,
    pub USART1: usart::Instance,
    pub USART2: usart::Instance,
    pub USART3: usart::Instance,
    pub UART4: usart::Instance,
    pub UART5: usart::Instance,
    pub LPUART1: lpuart1::Instance,
    pub SPI1: spi::Instance,
    pub SPI2: spi::Instance,
    pub SPI3: spi::Instance,
    pub SDMMC1: sdmmc::Instance,
    pub SDMMC2: sdmmc::Instance,
    pub EXTI: exti::Instance,
    pub VREFBUF: vrefbuf::Instance,
    pub CAN1: can1::Instance,
    pub RTC: rtc::Instance,
    pub OTG_FS_GLOBAL: otg_fs_global::Instance,
    pub OTG_FS_HOST: otg_fs_host::Instance,
    pub OTG_FS_DEVICE: otg_fs_device::Instance,
    pub OTG_FS_PWRCLK: otg_fs_pwrclk::Instance,
    pub OPAMP: opamp::Instance,
    pub FMC: fmc::Instance,
    pub NVIC: nvic::Instance,
    pub CRS: crs::Instance,
    pub DCMI: dcmi::Instance,
    pub HASH: hash::Instance,
    pub DMA2D: dma2d::Instance,
    pub OCTOSPIM: octospim::Instance,
    pub FPU: fpu::Instance,
    pub MPU: mpu::Instance,
    pub STK: stk::Instance,
    pub SCB: scb::Instance,
    pub NVIC_STIR: nvic_stir::Instance,
    pub FPU_CPACR: fpu_cpacr::Instance,
    pub SCB_ACTRL: scb_actrl::Instance,
    pub PSSI: pssi::Instance,
}

#[cfg(all(feature = "rtic", feature = "nosync"))]
#[allow(non_snake_case)]
pub struct Peripherals {}

#[cfg(all(feature = "rtic", not(feature = "nosync")))]
impl Peripherals {
    pub unsafe fn steal() -> Self {
        Peripherals {
            DAC: dac::DAC::steal(),
            DMA1: dma::DMA1::steal(),
            DMA2: dma::DMA2::steal(),
            DMAMUX: dmamux::DMAMUX::steal(),
            CRC: crc::CRC::steal(),
            LTDC: ltdc::LTDC::steal(),
            TSC: tsc::TSC::steal(),
            IWDG: iwdg::IWDG::steal(),
            WWDG: wwdg::WWDG::steal(),
            COMP: comp::COMP::steal(),
            FIREWALL: firewall::FIREWALL::steal(),
            I2C1: i2c::I2C1::steal(),
            I2C2: i2c::I2C2::steal(),
            I2C3: i2c::I2C3::steal(),
            I2C4: i2c::I2C4::steal(),
            FLASH: flash::FLASH::steal(),
            DBGMCU: dbgmcu::DBGMCU::steal(),
            OCTOSPI1: octospi::OCTOSPI1::steal(),
            OCTOSPI2: octospi::OCTOSPI2::steal(),
            RCC: rcc::RCC::steal(),
            PWR: pwr::PWR::steal(),
            SYSCFG: syscfg::SYSCFG::steal(),
            DFSDM1: dfsdm1::DFSDM1::steal(),
            RNG: rng::RNG::steal(),
            AES: aes::AES::steal(),
            ADC1: adc1::ADC1::steal(),
            ADC_Common: adc_common::ADC_Common::steal(),
            GPIOA: gpio::GPIOA::steal(),
            GPIOB: gpio::GPIOB::steal(),
            GPIOC: gpio::GPIOC::steal(),
            GPIOD: gpio::GPIOD::steal(),
            GPIOE: gpio::GPIOE::steal(),
            GPIOF: gpio::GPIOF::steal(),
            GPIOG: gpio::GPIOG::steal(),
            GPIOH: gpio::GPIOH::steal(),
            GPIOI: gpioi::GPIOI::steal(),
            SAI1: sai::SAI1::steal(),
            SAI2: sai::SAI2::steal(),
            TIM2: tim2::TIM2::steal(),
            TIM3: tim3::TIM3::steal(),
            TIM4: tim4::TIM4::steal(),
            TIM5: tim5::TIM5::steal(),
            TIM15: tim15::TIM15::steal(),
            TIM16: tim16::TIM16::steal(),
            TIM17: tim17::TIM17::steal(),
            TIM1: tim1::TIM1::steal(),
            TIM8: tim8::TIM8::steal(),
            TIM6: tim6::TIM6::steal(),
            TIM7: tim7::TIM7::steal(),
            LPTIM1: lptim::LPTIM1::steal(),
            LPTIM2: lptim::LPTIM2::steal(),
            USART1: usart::USART1::steal(),
            USART2: usart::USART2::steal(),
            USART3: usart::USART3::steal(),
            UART4: usart::UART4::steal(),
            UART5: usart::UART5::steal(),
            LPUART1: lpuart1::LPUART1::steal(),
            SPI1: spi::SPI1::steal(),
            SPI2: spi::SPI2::steal(),
            SPI3: spi::SPI3::steal(),
            SDMMC1: sdmmc::SDMMC1::steal(),
            SDMMC2: sdmmc::SDMMC2::steal(),
            EXTI: exti::EXTI::steal(),
            VREFBUF: vrefbuf::VREFBUF::steal(),
            CAN1: can1::CAN1::steal(),
            RTC: rtc::RTC::steal(),
            OTG_FS_GLOBAL: otg_fs_global::OTG_FS_GLOBAL::steal(),
            OTG_FS_HOST: otg_fs_host::OTG_FS_HOST::steal(),
            OTG_FS_DEVICE: otg_fs_device::OTG_FS_DEVICE::steal(),
            OTG_FS_PWRCLK: otg_fs_pwrclk::OTG_FS_PWRCLK::steal(),
            OPAMP: opamp::OPAMP::steal(),
            FMC: fmc::FMC::steal(),
            NVIC: nvic::NVIC::steal(),
            CRS: crs::CRS::steal(),
            DCMI: dcmi::DCMI::steal(),
            HASH: hash::HASH::steal(),
            DMA2D: dma2d::DMA2D::steal(),
            OCTOSPIM: octospim::OCTOSPIM::steal(),
            FPU: fpu::FPU::steal(),
            MPU: mpu::MPU::steal(),
            STK: stk::STK::steal(),
            SCB: scb::SCB::steal(),
            NVIC_STIR: nvic_stir::NVIC_STIR::steal(),
            FPU_CPACR: fpu_cpacr::FPU_CPACR::steal(),
            SCB_ACTRL: scb_actrl::SCB_ACTRL::steal(),
            PSSI: pssi::PSSI::steal(),
        }
    }
}

#[cfg(all(feature = "rtic", feature = "nosync"))]
impl Peripherals {
    pub fn steal() -> Self {
        Peripherals {}
    }
}
