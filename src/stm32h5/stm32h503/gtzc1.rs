#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Global privilege controller

use crate::RWRegister;
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// GTZC1 TZSC privilege configuration register 1
pub mod GTZC1_TZSC_PRIVCFGR1 {

    /// privileged access mode for TIM2
    pub mod TIM2PRIV {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// privileged access mode for TIM3
    pub mod TIM3PRIV {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// privileged access mode for TIM6
    pub mod TIM6PRIV {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// privileged access mode for TIM7
    pub mod TIM7PRIV {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// privileged access mode for WWDG
    pub mod WWDGPRIV {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// privileged access mode for IWDG
    pub mod IWDGPRIV {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// privileged access mode for SPI2
    pub mod SPI2PRIV {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// privileged access mode for SPI3
    pub mod SPI3PRIV {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// privileged access mode for USART2
    pub mod USART2PRIV {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// privileged access mode for USART3
    pub mod USART3PRIV {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// privileged access mode for I2C1
    pub mod I2C1PRIV {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// privileged access mode for I2C2
    pub mod I2C2PRIV {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (1 bit: 1 << 18)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// privileged access mode for I3C1
    pub mod I3C1PRIV {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (1 bit: 1 << 19)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// privileged access mode for CRS
    pub mod CRSPRIV {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (1 bit: 1 << 20)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// privileged access mode for DAC1
    pub mod DAC1PRIV {
        /// Offset (25 bits)
        pub const offset: u32 = 25;
        /// Mask (1 bit: 1 << 25)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// privileged access mode for DTS
    pub mod DTSPRIV {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (1 bit: 1 << 30)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// privileged access mode for LPTIM2
    pub mod LPTIM2PRIV {
        /// Offset (31 bits)
        pub const offset: u32 = 31;
        /// Mask (1 bit: 1 << 31)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// GTZC1 TZSC privilege configuration register 2
pub mod GTZC1_TZSC_PRIVCFGR2 {

    /// privileged access mode for FDCAN1
    pub mod FDCAN1PRIV {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// privileged access mode for OPAMP
    pub mod OPAMPPRIV {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// privileged access mode for COMP
    pub mod COMPPRIV {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// privileged access mode for TIM1
    pub mod TIM1PRIV {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// privileged access mode for SPI1
    pub mod SPI1PRIV {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// privileged access mode for USART1
    pub mod USART1PRIV {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// privileged access mode for USBSF
    pub mod USBFSPRIV {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (1 bit: 1 << 19)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// privileged access mode for LPUART
    pub mod LPUART1PRIV {
        /// Offset (25 bits)
        pub const offset: u32 = 25;
        /// Mask (1 bit: 1 << 25)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// privileged access mode for LPTIM1
    pub mod LPTIM1PRIV {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (1 bit: 1 << 28)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// GTZC1 TZSC privilege configuration register 3
pub mod GTZC1_TZSC_PRIVCFGR3 {

    /// privileged access mode for I3C2
    pub mod I3C2PRIV {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// privileged access mode for CRC
    pub mod CRCPRIV {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// privileged access mode for ICACHE
    pub mod ICACHEPRIV {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// privileged access mode for ADC1
    pub mod ADC1PRIV {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// privileged access mode for HASH
    pub mod HASHPRIV {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// privileged access mode for RNG
    pub mod RNGPRIV {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (1 bit: 1 << 18)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// privileged access mode for RAMSCFG
    pub mod RAMCFGPRIV {
        /// Offset (26 bits)
        pub const offset: u32 = 26;
        /// Mask (1 bit: 1 << 26)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// GTZC1 TZSC BKPSRAM sub-region A watermark configuration register
pub mod GTZC1_TZSC_MPCWM4ACFGR {

    /// Sub-region z enable
    pub mod SREN {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Sub-region z lock This bit, once set, can be cleared only by a system reset.
    pub mod SRLOCK {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Privileged sub-region z This bit is taken into account only if SREN is set.
    pub mod PRIV {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// GTZC1 TZSC BKPSRAM sub-region A watermark register
pub mod GTZC1_TZSC_MPCWM4AR {

    /// Start of sub-region A This field defines the address offset of the sub-region A, to be multiplied by the granularity defined in Table 16.
    pub mod SUBA_START {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (11 bits: 0x7ff << 0)
        pub const mask: u32 = 0x7ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Length of sub-region A This field defines the length of the sub-region A, to be multiplied by the granularity defined in Table 16. When SUBA_START + SUBA_LENGTH is higher than the maximum size allowed for the memory, a saturation of SUBA_LENGTH is applied automatically. If SUBA_LENGTH = 0, the sub-region A is disabled (SREN bit in GTZC1_TZSC_MPCMWACFGR is cleared).
    pub mod SUBA_LENGTH {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (12 bits: 0xfff << 16)
        pub const mask: u32 = 0xfff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// GTZC1 TZSC BKPSRAM sub-region B watermark configuration register
pub mod GTZC1_TZSC_MPCWM4BCFGR {
    pub use super::GTZC1_TZSC_MPCWM4ACFGR::PRIV;
    pub use super::GTZC1_TZSC_MPCWM4ACFGR::SREN;
    pub use super::GTZC1_TZSC_MPCWM4ACFGR::SRLOCK;
}

/// GTZC1 TZSC BKPSRAM sub-region B watermark register
pub mod GTZC1_TZSC_MPCWM4BR {

    /// Start of sub-region B This field defines the address offset of the sub-region B, to be multiplied by the granularity defined in Table 16.
    pub mod SUBB_START {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (11 bits: 0x7ff << 0)
        pub const mask: u32 = 0x7ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Length of sub-region B This field defines the length of the sub-region B, to be multiplied by the granularity defined in Table 16. When SUBB_START + SUBB_LENGTH is higher than the maximum size allowed for the memory, a saturation of SUBB_LENGTH is applied automatically. If SUBB_LENGTH = 0, the sub-region B is disabled (SREN bit in GTZC1_TZSC_MPCMWBCFGR is cleared).
    pub mod SUBB_LENGTH {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (12 bits: 0xfff << 16)
        pub const mask: u32 = 0xfff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// GTZC1 SRAM1 MPCBB privileged configuration for super-block 0 register
pub mod GTZC1_MPCBB1_PRIVCFGR0 {

    /// Privileged configuration for block y, belonging to super-block x (y = 31 to 0).
    pub mod PRIV0 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Privileged configuration for block y, belonging to super-block x (y = 31 to 0).
    pub mod PRIV1 {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Privileged configuration for block y, belonging to super-block x (y = 31 to 0).
    pub mod PRIV2 {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Privileged configuration for block y, belonging to super-block x (y = 31 to 0).
    pub mod PRIV3 {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Privileged configuration for block y, belonging to super-block x (y = 31 to 0).
    pub mod PRIV4 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Privileged configuration for block y, belonging to super-block x (y = 31 to 0).
    pub mod PRIV5 {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Privileged configuration for block y, belonging to super-block x (y = 31 to 0).
    pub mod PRIV6 {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Privileged configuration for block y, belonging to super-block x (y = 31 to 0).
    pub mod PRIV7 {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Privileged configuration for block y, belonging to super-block x (y = 31 to 0).
    pub mod PRIV8 {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Privileged configuration for block y, belonging to super-block x (y = 31 to 0).
    pub mod PRIV9 {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Privileged configuration for block y, belonging to super-block x (y = 31 to 0).
    pub mod PRIV10 {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Privileged configuration for block y, belonging to super-block x (y = 31 to 0).
    pub mod PRIV11 {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Privileged configuration for block y, belonging to super-block x (y = 31 to 0).
    pub mod PRIV12 {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Privileged configuration for block y, belonging to super-block x (y = 31 to 0).
    pub mod PRIV13 {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Privileged configuration for block y, belonging to super-block x (y = 31 to 0).
    pub mod PRIV14 {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Privileged configuration for block y, belonging to super-block x (y = 31 to 0).
    pub mod PRIV15 {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Privileged configuration for block y, belonging to super-block x (y = 31 to 0).
    pub mod PRIV16 {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Privileged configuration for block y, belonging to super-block x (y = 31 to 0).
    pub mod PRIV17 {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Privileged configuration for block y, belonging to super-block x (y = 31 to 0).
    pub mod PRIV18 {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (1 bit: 1 << 18)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Privileged configuration for block y, belonging to super-block x (y = 31 to 0).
    pub mod PRIV19 {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (1 bit: 1 << 19)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Privileged configuration for block y, belonging to super-block x (y = 31 to 0).
    pub mod PRIV20 {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (1 bit: 1 << 20)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Privileged configuration for block y, belonging to super-block x (y = 31 to 0).
    pub mod PRIV21 {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (1 bit: 1 << 21)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Privileged configuration for block y, belonging to super-block x (y = 31 to 0).
    pub mod PRIV22 {
        /// Offset (22 bits)
        pub const offset: u32 = 22;
        /// Mask (1 bit: 1 << 22)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Privileged configuration for block y, belonging to super-block x (y = 31 to 0).
    pub mod PRIV23 {
        /// Offset (23 bits)
        pub const offset: u32 = 23;
        /// Mask (1 bit: 1 << 23)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Privileged configuration for block y, belonging to super-block x (y = 31 to 0).
    pub mod PRIV24 {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (1 bit: 1 << 24)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Privileged configuration for block y, belonging to super-block x (y = 31 to 0).
    pub mod PRIV25 {
        /// Offset (25 bits)
        pub const offset: u32 = 25;
        /// Mask (1 bit: 1 << 25)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Privileged configuration for block y, belonging to super-block x (y = 31 to 0).
    pub mod PRIV26 {
        /// Offset (26 bits)
        pub const offset: u32 = 26;
        /// Mask (1 bit: 1 << 26)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Privileged configuration for block y, belonging to super-block x (y = 31 to 0).
    pub mod PRIV27 {
        /// Offset (27 bits)
        pub const offset: u32 = 27;
        /// Mask (1 bit: 1 << 27)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Privileged configuration for block y, belonging to super-block x (y = 31 to 0).
    pub mod PRIV28 {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (1 bit: 1 << 28)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Privileged configuration for block y, belonging to super-block x (y = 31 to 0).
    pub mod PRIV29 {
        /// Offset (29 bits)
        pub const offset: u32 = 29;
        /// Mask (1 bit: 1 << 29)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Privileged configuration for block y, belonging to super-block x (y = 31 to 0).
    pub mod PRIV30 {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (1 bit: 1 << 30)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Privileged configuration for block y, belonging to super-block x (y = 31 to 0).
    pub mod PRIV31 {
        /// Offset (31 bits)
        pub const offset: u32 = 31;
        /// Mask (1 bit: 1 << 31)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// GTZC1 SRAM1 MPCBB privileged configuration for super-block 1 register
pub mod GTZC1_MPCBB1_PRIVCFGR1 {
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV0;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV1;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV10;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV11;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV12;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV13;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV14;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV15;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV16;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV17;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV18;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV19;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV2;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV20;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV21;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV22;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV23;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV24;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV25;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV26;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV27;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV28;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV29;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV3;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV30;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV31;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV4;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV5;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV6;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV7;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV8;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV9;
}

/// GTZC1 SRAM1 MPCBB privileged configuration for super-block 2 register
pub mod GTZC1_MPCBB1_PRIVCFGR2 {
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV0;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV1;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV10;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV11;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV12;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV13;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV14;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV15;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV16;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV17;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV18;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV19;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV2;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV20;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV21;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV22;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV23;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV24;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV25;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV26;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV27;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV28;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV29;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV3;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV30;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV31;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV4;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV5;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV6;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV7;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV8;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV9;
}

/// GTZC1 SRAM1 MPCBB privileged configuration for super-block 3 register
pub mod GTZC1_MPCBB1_PRIVCFGR3 {
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV0;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV1;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV10;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV11;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV12;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV13;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV14;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV15;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV16;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV17;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV18;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV19;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV2;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV20;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV21;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV22;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV23;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV24;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV25;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV26;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV27;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV28;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV29;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV3;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV30;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV31;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV4;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV5;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV6;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV7;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV8;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV9;
}

/// GTZC1 SRAM1 MPCBB privileged configuration for super-block 4 register
pub mod GTZC1_MPCBB1_PRIVCFGR4 {
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV0;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV1;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV10;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV11;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV12;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV13;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV14;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV15;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV16;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV17;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV18;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV19;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV2;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV20;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV21;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV22;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV23;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV24;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV25;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV26;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV27;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV28;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV29;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV3;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV30;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV31;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV4;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV5;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV6;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV7;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV8;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV9;
}

/// GTZC1 SRAM1 MPCBB privileged configuration for super-block 5 register
pub mod GTZC1_MPCBB1_PRIVCFGR5 {
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV0;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV1;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV10;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV11;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV12;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV13;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV14;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV15;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV16;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV17;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV18;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV19;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV2;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV20;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV21;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV22;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV23;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV24;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV25;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV26;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV27;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV28;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV29;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV3;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV30;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV31;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV4;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV5;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV6;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV7;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV8;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV9;
}

/// GTZC1 SRAM1 MPCBB privileged configuration for super-block 6 register
pub mod GTZC1_MPCBB1_PRIVCFGR6 {
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV0;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV1;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV10;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV11;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV12;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV13;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV14;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV15;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV16;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV17;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV18;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV19;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV2;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV20;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV21;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV22;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV23;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV24;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV25;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV26;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV27;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV28;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV29;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV3;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV30;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV31;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV4;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV5;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV6;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV7;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV8;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV9;
}

/// GTZC1 SRAM1 MPCBB privileged configuration for super-block 7 register
pub mod GTZC1_MPCBB1_PRIVCFGR7 {
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV0;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV1;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV10;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV11;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV12;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV13;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV14;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV15;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV16;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV17;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV18;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV19;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV2;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV20;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV21;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV22;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV23;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV24;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV25;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV26;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV27;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV28;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV29;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV3;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV30;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV31;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV4;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV5;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV6;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV7;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV8;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV9;
}

/// GTZC1 SRAM1 MPCBB privileged configuration for super-block 8 register
pub mod GTZC1_MPCBB1_PRIVCFGR8 {
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV0;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV1;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV10;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV11;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV12;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV13;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV14;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV15;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV16;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV17;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV18;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV19;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV2;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV20;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV21;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV22;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV23;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV24;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV25;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV26;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV27;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV28;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV29;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV3;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV30;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV31;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV4;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV5;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV6;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV7;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV8;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV9;
}

/// GTZC1 SRAM1 MPCBB privileged configuration for super-block 9 register
pub mod GTZC1_MPCBB1_PRIVCFGR9 {
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV0;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV1;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV10;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV11;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV12;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV13;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV14;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV15;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV16;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV17;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV18;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV19;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV2;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV20;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV21;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV22;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV23;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV24;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV25;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV26;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV27;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV28;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV29;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV3;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV30;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV31;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV4;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV5;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV6;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV7;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV8;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV9;
}

/// GTZC1 SRAM1 MPCBB privileged configuration for super-block 10 register
pub mod GTZC1_MPCBB1_PRIVCFGR10 {
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV0;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV1;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV10;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV11;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV12;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV13;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV14;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV15;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV16;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV17;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV18;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV19;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV2;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV20;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV21;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV22;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV23;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV24;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV25;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV26;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV27;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV28;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV29;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV3;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV30;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV31;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV4;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV5;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV6;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV7;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV8;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV9;
}

/// GTZC1 SRAM1 MPCBB privileged configuration for super-block 11 register
pub mod GTZC1_MPCBB1_PRIVCFGR11 {
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV0;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV1;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV10;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV11;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV12;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV13;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV14;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV15;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV16;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV17;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV18;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV19;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV2;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV20;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV21;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV22;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV23;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV24;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV25;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV26;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV27;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV28;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV29;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV3;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV30;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV31;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV4;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV5;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV6;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV7;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV8;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV9;
}

/// GTZC1 SRAM1 MPCBB privileged configuration for super-block 12 register
pub mod GTZC1_MPCBB1_PRIVCFGR12 {
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV0;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV1;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV10;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV11;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV12;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV13;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV14;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV15;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV16;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV17;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV18;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV19;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV2;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV20;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV21;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV22;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV23;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV24;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV25;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV26;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV27;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV28;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV29;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV3;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV30;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV31;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV4;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV5;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV6;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV7;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV8;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV9;
}

/// GTZC1 SRAM1 MPCBB privileged configuration for super-block 13 register
pub mod GTZC1_MPCBB1_PRIVCFGR13 {
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV0;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV1;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV10;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV11;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV12;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV13;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV14;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV15;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV16;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV17;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV18;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV19;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV2;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV20;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV21;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV22;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV23;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV24;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV25;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV26;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV27;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV28;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV29;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV3;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV30;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV31;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV4;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV5;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV6;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV7;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV8;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV9;
}

/// GTZC1 SRAM1 MPCBB privileged configuration for super-block 14 register
pub mod GTZC1_MPCBB1_PRIVCFGR14 {
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV0;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV1;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV10;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV11;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV12;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV13;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV14;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV15;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV16;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV17;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV18;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV19;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV2;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV20;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV21;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV22;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV23;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV24;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV25;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV26;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV27;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV28;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV29;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV3;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV30;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV31;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV4;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV5;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV6;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV7;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV8;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV9;
}

/// GTZC1 SRAM1 MPCBB privileged configuration for super-block 15 register
pub mod GTZC1_MPCBB1_PRIVCFGR15 {
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV0;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV1;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV10;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV11;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV12;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV13;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV14;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV15;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV16;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV17;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV18;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV19;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV2;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV20;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV21;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV22;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV23;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV24;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV25;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV26;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV27;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV28;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV29;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV3;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV30;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV31;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV4;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV5;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV6;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV7;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV8;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV9;
}

/// GTZC1 SRAM1 MPCBB privileged configuration for super-block 16 register
pub mod GTZC1_MPCBB1_PRIVCFGR16 {
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV0;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV1;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV10;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV11;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV12;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV13;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV14;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV15;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV16;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV17;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV18;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV19;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV2;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV20;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV21;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV22;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV23;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV24;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV25;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV26;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV27;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV28;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV29;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV3;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV30;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV31;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV4;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV5;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV6;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV7;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV8;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV9;
}

/// GTZC1 SRAM1 MPCBB privileged configuration for super-block 17 register
pub mod GTZC1_MPCBB1_PRIVCFGR17 {
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV0;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV1;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV10;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV11;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV12;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV13;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV14;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV15;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV16;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV17;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV18;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV19;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV2;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV20;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV21;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV22;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV23;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV24;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV25;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV26;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV27;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV28;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV29;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV3;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV30;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV31;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV4;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV5;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV6;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV7;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV8;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV9;
}

/// GTZC1 SRAM1 MPCBB privileged configuration for super-block 18 register
pub mod GTZC1_MPCBB1_PRIVCFGR18 {
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV0;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV1;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV10;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV11;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV12;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV13;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV14;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV15;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV16;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV17;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV18;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV19;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV2;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV20;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV21;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV22;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV23;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV24;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV25;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV26;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV27;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV28;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV29;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV3;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV30;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV31;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV4;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV5;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV6;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV7;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV8;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV9;
}

/// GTZC1 SRAM1 MPCBB privileged configuration for super-block 19 register
pub mod GTZC1_MPCBB1_PRIVCFGR19 {
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV0;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV1;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV10;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV11;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV12;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV13;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV14;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV15;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV16;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV17;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV18;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV19;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV2;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV20;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV21;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV22;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV23;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV24;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV25;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV26;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV27;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV28;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV29;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV3;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV30;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV31;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV4;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV5;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV6;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV7;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV8;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV9;
}

/// GTZC1 SRAM1 MPCBB privileged configuration for super-block 20 register
pub mod GTZC1_MPCBB1_PRIVCFGR20 {
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV0;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV1;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV10;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV11;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV12;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV13;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV14;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV15;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV16;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV17;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV18;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV19;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV2;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV20;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV21;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV22;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV23;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV24;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV25;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV26;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV27;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV28;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV29;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV3;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV30;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV31;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV4;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV5;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV6;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV7;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV8;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV9;
}

/// GTZC1 SRAM1 MPCBB privileged configuration for super-block 21 register
pub mod GTZC1_MPCBB1_PRIVCFGR21 {
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV0;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV1;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV10;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV11;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV12;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV13;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV14;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV15;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV16;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV17;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV18;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV19;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV2;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV20;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV21;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV22;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV23;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV24;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV25;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV26;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV27;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV28;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV29;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV3;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV30;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV31;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV4;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV5;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV6;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV7;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV8;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV9;
}

/// GTZC1 SRAM1 MPCBB privileged configuration for super-block 22 register
pub mod GTZC1_MPCBB1_PRIVCFGR22 {
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV0;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV1;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV10;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV11;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV12;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV13;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV14;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV15;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV16;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV17;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV18;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV19;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV2;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV20;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV21;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV22;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV23;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV24;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV25;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV26;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV27;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV28;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV29;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV3;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV30;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV31;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV4;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV5;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV6;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV7;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV8;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV9;
}

/// GTZC1 SRAM1 MPCBB privileged configuration for super-block 23 register
pub mod GTZC1_MPCBB1_PRIVCFGR23 {
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV0;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV1;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV10;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV11;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV12;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV13;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV14;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV15;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV16;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV17;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV18;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV19;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV2;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV20;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV21;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV22;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV23;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV24;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV25;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV26;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV27;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV28;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV29;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV3;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV30;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV31;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV4;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV5;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV6;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV7;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV8;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV9;
}

/// GTZC1 SRAM1 MPCBB privileged configuration for super-block 24 register
pub mod GTZC1_MPCBB1_PRIVCFGR24 {
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV0;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV1;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV10;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV11;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV12;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV13;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV14;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV15;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV16;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV17;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV18;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV19;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV2;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV20;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV21;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV22;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV23;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV24;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV25;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV26;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV27;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV28;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV29;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV3;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV30;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV31;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV4;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV5;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV6;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV7;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV8;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV9;
}

/// GTZC1 SRAM1 MPCBB privileged configuration for super-block 25 register
pub mod GTZC1_MPCBB1_PRIVCFGR25 {
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV0;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV1;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV10;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV11;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV12;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV13;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV14;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV15;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV16;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV17;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV18;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV19;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV2;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV20;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV21;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV22;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV23;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV24;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV25;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV26;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV27;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV28;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV29;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV3;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV30;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV31;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV4;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV5;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV6;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV7;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV8;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV9;
}

/// GTZC1 SRAM1 MPCBB privileged configuration for super-block 26 register
pub mod GTZC1_MPCBB1_PRIVCFGR26 {
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV0;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV1;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV10;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV11;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV12;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV13;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV14;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV15;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV16;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV17;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV18;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV19;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV2;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV20;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV21;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV22;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV23;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV24;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV25;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV26;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV27;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV28;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV29;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV3;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV30;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV31;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV4;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV5;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV6;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV7;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV8;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV9;
}

/// GTZC1 SRAM1 MPCBB privileged configuration for super-block 27 register
pub mod GTZC1_MPCBB1_PRIVCFGR27 {
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV0;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV1;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV10;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV11;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV12;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV13;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV14;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV15;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV16;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV17;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV18;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV19;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV2;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV20;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV21;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV22;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV23;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV24;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV25;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV26;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV27;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV28;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV29;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV3;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV30;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV31;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV4;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV5;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV6;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV7;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV8;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV9;
}

/// GTZC1 SRAM1 MPCBB privileged configuration for super-block 28 register
pub mod GTZC1_MPCBB1_PRIVCFGR28 {
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV0;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV1;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV10;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV11;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV12;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV13;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV14;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV15;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV16;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV17;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV18;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV19;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV2;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV20;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV21;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV22;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV23;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV24;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV25;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV26;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV27;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV28;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV29;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV3;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV30;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV31;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV4;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV5;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV6;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV7;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV8;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV9;
}

/// GTZC1 SRAM1 MPCBB privileged configuration for super-block 29 register
pub mod GTZC1_MPCBB1_PRIVCFGR29 {
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV0;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV1;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV10;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV11;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV12;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV13;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV14;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV15;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV16;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV17;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV18;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV19;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV2;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV20;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV21;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV22;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV23;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV24;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV25;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV26;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV27;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV28;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV29;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV3;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV30;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV31;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV4;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV5;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV6;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV7;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV8;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV9;
}

/// GTZC1 SRAM1 MPCBB privileged configuration for super-block 30 register
pub mod GTZC1_MPCBB1_PRIVCFGR30 {
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV0;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV1;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV10;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV11;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV12;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV13;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV14;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV15;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV16;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV17;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV18;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV19;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV2;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV20;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV21;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV22;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV23;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV24;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV25;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV26;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV27;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV28;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV29;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV3;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV30;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV31;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV4;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV5;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV6;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV7;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV8;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV9;
}

/// GTZC1 SRAM1 MPCBB privileged configuration for super-block 31 register
pub mod GTZC1_MPCBB1_PRIVCFGR31 {
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV0;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV1;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV10;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV11;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV12;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV13;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV14;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV15;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV16;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV17;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV18;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV19;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV2;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV20;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV21;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV22;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV23;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV24;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV25;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV26;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV27;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV28;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV29;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV3;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV30;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV31;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV4;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV5;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV6;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV7;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV8;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV9;
}

/// GTZC1 SRAM2 MPCBB privileged configuration for super-block 0 register
pub mod GTZC1_MPCBB2_PRIVCFGR0 {
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV0;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV1;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV10;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV11;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV12;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV13;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV14;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV15;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV16;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV17;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV18;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV19;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV2;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV20;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV21;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV22;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV23;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV24;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV25;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV26;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV27;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV28;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV29;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV3;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV30;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV31;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV4;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV5;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV6;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV7;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV8;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV9;
}

/// GTZC1 SRAM2 MPCBB privileged configuration for super-block 1 register
pub mod GTZC1_MPCBB2_PRIVCFGR1 {
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV0;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV1;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV10;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV11;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV12;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV13;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV14;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV15;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV16;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV17;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV18;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV19;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV2;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV20;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV21;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV22;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV23;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV24;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV25;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV26;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV27;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV28;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV29;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV3;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV30;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV31;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV4;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV5;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV6;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV7;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV8;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV9;
}

/// GTZC1 SRAM2 MPCBB privileged configuration for super-block 2 register
pub mod GTZC1_MPCBB2_PRIVCFGR2 {
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV0;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV1;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV10;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV11;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV12;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV13;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV14;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV15;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV16;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV17;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV18;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV19;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV2;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV20;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV21;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV22;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV23;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV24;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV25;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV26;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV27;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV28;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV29;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV3;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV30;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV31;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV4;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV5;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV6;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV7;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV8;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV9;
}

/// GTZC1 SRAM2 MPCBB privileged configuration for super-block 3 register
pub mod GTZC1_MPCBB2_PRIVCFGR3 {
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV0;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV1;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV10;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV11;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV12;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV13;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV14;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV15;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV16;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV17;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV18;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV19;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV2;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV20;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV21;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV22;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV23;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV24;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV25;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV26;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV27;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV28;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV29;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV3;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV30;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV31;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV4;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV5;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV6;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV7;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV8;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV9;
}

/// GTZC1 SRAM2 MPCBB privileged configuration for super-block 4 register
pub mod GTZC1_MPCBB2_PRIVCFGR4 {
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV0;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV1;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV10;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV11;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV12;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV13;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV14;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV15;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV16;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV17;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV18;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV19;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV2;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV20;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV21;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV22;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV23;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV24;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV25;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV26;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV27;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV28;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV29;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV3;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV30;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV31;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV4;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV5;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV6;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV7;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV8;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV9;
}

/// GTZC1 SRAM2 MPCBB privileged configuration for super-block 5 register
pub mod GTZC1_MPCBB2_PRIVCFGR5 {
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV0;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV1;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV10;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV11;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV12;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV13;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV14;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV15;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV16;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV17;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV18;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV19;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV2;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV20;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV21;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV22;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV23;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV24;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV25;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV26;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV27;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV28;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV29;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV3;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV30;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV31;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV4;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV5;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV6;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV7;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV8;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV9;
}

/// GTZC1 SRAM2 MPCBB privileged configuration for super-block 6 register
pub mod GTZC1_MPCBB2_PRIVCFGR6 {
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV0;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV1;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV10;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV11;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV12;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV13;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV14;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV15;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV16;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV17;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV18;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV19;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV2;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV20;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV21;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV22;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV23;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV24;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV25;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV26;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV27;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV28;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV29;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV3;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV30;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV31;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV4;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV5;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV6;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV7;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV8;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV9;
}

/// GTZC1 SRAM2 MPCBB privileged configuration for super-block 7 register
pub mod GTZC1_MPCBB2_PRIVCFGR7 {
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV0;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV1;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV10;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV11;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV12;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV13;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV14;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV15;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV16;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV17;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV18;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV19;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV2;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV20;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV21;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV22;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV23;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV24;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV25;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV26;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV27;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV28;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV29;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV3;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV30;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV31;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV4;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV5;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV6;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV7;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV8;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV9;
}

/// GTZC1 SRAM2 MPCBB privileged configuration for super-block 8 register
pub mod GTZC1_MPCBB2_PRIVCFGR8 {
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV0;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV1;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV10;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV11;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV12;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV13;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV14;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV15;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV16;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV17;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV18;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV19;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV2;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV20;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV21;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV22;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV23;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV24;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV25;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV26;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV27;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV28;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV29;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV3;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV30;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV31;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV4;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV5;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV6;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV7;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV8;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV9;
}

/// GTZC1 SRAM2 MPCBB privileged configuration for super-block 9 register
pub mod GTZC1_MPCBB2_PRIVCFGR9 {
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV0;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV1;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV10;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV11;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV12;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV13;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV14;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV15;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV16;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV17;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV18;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV19;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV2;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV20;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV21;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV22;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV23;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV24;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV25;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV26;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV27;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV28;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV29;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV3;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV30;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV31;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV4;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV5;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV6;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV7;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV8;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV9;
}

/// GTZC1 SRAM2 MPCBB privileged configuration for super-block 10 register
pub mod GTZC1_MPCBB2_PRIVCFGR10 {
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV0;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV1;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV10;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV11;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV12;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV13;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV14;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV15;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV16;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV17;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV18;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV19;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV2;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV20;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV21;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV22;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV23;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV24;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV25;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV26;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV27;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV28;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV29;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV3;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV30;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV31;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV4;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV5;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV6;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV7;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV8;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV9;
}

/// GTZC1 SRAM2 MPCBB privileged configuration for super-block 11 register
pub mod GTZC1_MPCBB2_PRIVCFGR11 {
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV0;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV1;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV10;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV11;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV12;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV13;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV14;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV15;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV16;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV17;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV18;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV19;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV2;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV20;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV21;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV22;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV23;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV24;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV25;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV26;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV27;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV28;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV29;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV3;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV30;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV31;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV4;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV5;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV6;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV7;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV8;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV9;
}

/// GTZC1 SRAM2 MPCBB privileged configuration for super-block 12 register
pub mod GTZC1_MPCBB2_PRIVCFGR12 {
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV0;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV1;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV10;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV11;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV12;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV13;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV14;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV15;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV16;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV17;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV18;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV19;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV2;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV20;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV21;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV22;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV23;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV24;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV25;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV26;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV27;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV28;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV29;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV3;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV30;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV31;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV4;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV5;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV6;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV7;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV8;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV9;
}

/// GTZC1 SRAM2 MPCBB privileged configuration for super-block 13 register
pub mod GTZC1_MPCBB2_PRIVCFGR13 {
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV0;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV1;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV10;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV11;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV12;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV13;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV14;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV15;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV16;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV17;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV18;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV19;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV2;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV20;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV21;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV22;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV23;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV24;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV25;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV26;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV27;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV28;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV29;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV3;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV30;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV31;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV4;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV5;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV6;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV7;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV8;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV9;
}

/// GTZC1 SRAM2 MPCBB privileged configuration for super-block 14 register
pub mod GTZC1_MPCBB2_PRIVCFGR14 {
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV0;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV1;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV10;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV11;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV12;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV13;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV14;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV15;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV16;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV17;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV18;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV19;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV2;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV20;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV21;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV22;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV23;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV24;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV25;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV26;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV27;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV28;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV29;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV3;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV30;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV31;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV4;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV5;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV6;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV7;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV8;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV9;
}

/// GTZC1 SRAM2 MPCBB privileged configuration for super-block 15 register
pub mod GTZC1_MPCBB2_PRIVCFGR15 {
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV0;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV1;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV10;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV11;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV12;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV13;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV14;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV15;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV16;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV17;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV18;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV19;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV2;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV20;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV21;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV22;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV23;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV24;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV25;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV26;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV27;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV28;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV29;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV3;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV30;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV31;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV4;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV5;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV6;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV7;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV8;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV9;
}

/// GTZC1 SRAM2 MPCBB privileged configuration for super-block 16 register
pub mod GTZC1_MPCBB2_PRIVCFGR16 {
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV0;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV1;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV10;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV11;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV12;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV13;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV14;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV15;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV16;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV17;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV18;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV19;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV2;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV20;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV21;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV22;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV23;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV24;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV25;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV26;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV27;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV28;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV29;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV3;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV30;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV31;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV4;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV5;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV6;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV7;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV8;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV9;
}

/// GTZC1 SRAM2 MPCBB privileged configuration for super-block 17 register
pub mod GTZC1_MPCBB2_PRIVCFGR17 {
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV0;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV1;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV10;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV11;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV12;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV13;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV14;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV15;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV16;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV17;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV18;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV19;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV2;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV20;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV21;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV22;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV23;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV24;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV25;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV26;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV27;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV28;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV29;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV3;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV30;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV31;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV4;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV5;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV6;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV7;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV8;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV9;
}

/// GTZC1 SRAM2 MPCBB privileged configuration for super-block 18 register
pub mod GTZC1_MPCBB2_PRIVCFGR18 {
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV0;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV1;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV10;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV11;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV12;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV13;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV14;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV15;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV16;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV17;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV18;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV19;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV2;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV20;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV21;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV22;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV23;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV24;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV25;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV26;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV27;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV28;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV29;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV3;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV30;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV31;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV4;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV5;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV6;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV7;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV8;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV9;
}

/// GTZC1 SRAM2 MPCBB privileged configuration for super-block 19 register
pub mod GTZC1_MPCBB2_PRIVCFGR19 {
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV0;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV1;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV10;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV11;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV12;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV13;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV14;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV15;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV16;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV17;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV18;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV19;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV2;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV20;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV21;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV22;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV23;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV24;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV25;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV26;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV27;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV28;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV29;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV3;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV30;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV31;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV4;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV5;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV6;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV7;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV8;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV9;
}

/// GTZC1 SRAM2 MPCBB privileged configuration for super-block 20 register
pub mod GTZC1_MPCBB2_PRIVCFGR20 {
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV0;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV1;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV10;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV11;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV12;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV13;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV14;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV15;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV16;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV17;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV18;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV19;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV2;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV20;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV21;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV22;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV23;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV24;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV25;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV26;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV27;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV28;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV29;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV3;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV30;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV31;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV4;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV5;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV6;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV7;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV8;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV9;
}

/// GTZC1 SRAM2 MPCBB privileged configuration for super-block 21 register
pub mod GTZC1_MPCBB2_PRIVCFGR21 {
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV0;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV1;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV10;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV11;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV12;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV13;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV14;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV15;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV16;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV17;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV18;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV19;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV2;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV20;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV21;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV22;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV23;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV24;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV25;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV26;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV27;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV28;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV29;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV3;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV30;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV31;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV4;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV5;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV6;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV7;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV8;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV9;
}

/// GTZC1 SRAM2 MPCBB privileged configuration for super-block 22 register
pub mod GTZC1_MPCBB2_PRIVCFGR22 {
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV0;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV1;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV10;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV11;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV12;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV13;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV14;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV15;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV16;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV17;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV18;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV19;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV2;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV20;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV21;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV22;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV23;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV24;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV25;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV26;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV27;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV28;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV29;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV3;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV30;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV31;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV4;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV5;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV6;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV7;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV8;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV9;
}

/// GTZC1 SRAM2 MPCBB privileged configuration for super-block 23 register
pub mod GTZC1_MPCBB2_PRIVCFGR23 {
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV0;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV1;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV10;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV11;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV12;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV13;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV14;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV15;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV16;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV17;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV18;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV19;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV2;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV20;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV21;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV22;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV23;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV24;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV25;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV26;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV27;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV28;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV29;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV3;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV30;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV31;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV4;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV5;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV6;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV7;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV8;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV9;
}

/// GTZC1 SRAM2 MPCBB privileged configuration for super-block 24 register
pub mod GTZC1_MPCBB2_PRIVCFGR24 {
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV0;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV1;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV10;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV11;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV12;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV13;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV14;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV15;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV16;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV17;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV18;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV19;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV2;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV20;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV21;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV22;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV23;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV24;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV25;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV26;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV27;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV28;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV29;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV3;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV30;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV31;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV4;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV5;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV6;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV7;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV8;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV9;
}

/// GTZC1 SRAM2 MPCBB privileged configuration for super-block 25 register
pub mod GTZC1_MPCBB2_PRIVCFGR25 {
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV0;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV1;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV10;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV11;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV12;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV13;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV14;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV15;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV16;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV17;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV18;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV19;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV2;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV20;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV21;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV22;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV23;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV24;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV25;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV26;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV27;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV28;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV29;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV3;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV30;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV31;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV4;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV5;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV6;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV7;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV8;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV9;
}

/// GTZC1 SRAM2 MPCBB privileged configuration for super-block 26 register
pub mod GTZC1_MPCBB2_PRIVCFGR26 {
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV0;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV1;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV10;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV11;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV12;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV13;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV14;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV15;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV16;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV17;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV18;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV19;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV2;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV20;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV21;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV22;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV23;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV24;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV25;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV26;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV27;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV28;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV29;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV3;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV30;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV31;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV4;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV5;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV6;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV7;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV8;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV9;
}

/// GTZC1 SRAM2 MPCBB privileged configuration for super-block 27 register
pub mod GTZC1_MPCBB2_PRIVCFGR27 {
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV0;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV1;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV10;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV11;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV12;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV13;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV14;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV15;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV16;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV17;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV18;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV19;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV2;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV20;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV21;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV22;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV23;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV24;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV25;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV26;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV27;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV28;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV29;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV3;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV30;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV31;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV4;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV5;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV6;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV7;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV8;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV9;
}

/// GTZC1 SRAM2 MPCBB privileged configuration for super-block 28 register
pub mod GTZC1_MPCBB2_PRIVCFGR28 {
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV0;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV1;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV10;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV11;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV12;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV13;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV14;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV15;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV16;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV17;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV18;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV19;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV2;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV20;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV21;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV22;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV23;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV24;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV25;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV26;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV27;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV28;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV29;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV3;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV30;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV31;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV4;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV5;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV6;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV7;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV8;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV9;
}

/// GTZC1 SRAM2 MPCBB privileged configuration for super-block 29 register
pub mod GTZC1_MPCBB2_PRIVCFGR29 {
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV0;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV1;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV10;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV11;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV12;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV13;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV14;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV15;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV16;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV17;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV18;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV19;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV2;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV20;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV21;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV22;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV23;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV24;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV25;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV26;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV27;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV28;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV29;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV3;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV30;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV31;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV4;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV5;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV6;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV7;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV8;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV9;
}

/// GTZC1 SRAM2 MPCBB privileged configuration for super-block 30 register
pub mod GTZC1_MPCBB2_PRIVCFGR30 {
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV0;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV1;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV10;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV11;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV12;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV13;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV14;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV15;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV16;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV17;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV18;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV19;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV2;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV20;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV21;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV22;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV23;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV24;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV25;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV26;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV27;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV28;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV29;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV3;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV30;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV31;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV4;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV5;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV6;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV7;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV8;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV9;
}

/// GTZC1 SRAM2 MPCBB privileged configuration for super-block 31 register
pub mod GTZC1_MPCBB2_PRIVCFGR31 {
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV0;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV1;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV10;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV11;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV12;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV13;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV14;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV15;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV16;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV17;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV18;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV19;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV2;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV20;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV21;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV22;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV23;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV24;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV25;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV26;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV27;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV28;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV29;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV3;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV30;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV31;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV4;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV5;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV6;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV7;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV8;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV9;
}
#[repr(C)]
pub struct RegisterBlock {
    _reserved1: [u8; 32],

    /// GTZC1 TZSC privilege configuration register 1
    pub GTZC1_TZSC_PRIVCFGR1: RWRegister<u32>,

    /// GTZC1 TZSC privilege configuration register 2
    pub GTZC1_TZSC_PRIVCFGR2: RWRegister<u32>,

    /// GTZC1 TZSC privilege configuration register 3
    pub GTZC1_TZSC_PRIVCFGR3: RWRegister<u32>,

    _reserved2: [u8; 68],

    /// GTZC1 TZSC BKPSRAM sub-region A watermark configuration register
    pub GTZC1_TZSC_MPCWM4ACFGR: RWRegister<u32>,

    /// GTZC1 TZSC BKPSRAM sub-region A watermark register
    pub GTZC1_TZSC_MPCWM4AR: RWRegister<u32>,

    /// GTZC1 TZSC BKPSRAM sub-region B watermark configuration register
    pub GTZC1_TZSC_MPCWM4BCFGR: RWRegister<u32>,

    /// GTZC1 TZSC BKPSRAM sub-region B watermark register
    pub GTZC1_TZSC_MPCWM4BR: RWRegister<u32>,

    _reserved3: [u8; 384],

    /// GTZC1 SRAM1 MPCBB privileged configuration for super-block 0 register
    pub GTZC1_MPCBB1_PRIVCFGR0: RWRegister<u32>,

    /// GTZC1 SRAM1 MPCBB privileged configuration for super-block 1 register
    pub GTZC1_MPCBB1_PRIVCFGR1: RWRegister<u32>,

    /// GTZC1 SRAM1 MPCBB privileged configuration for super-block 2 register
    pub GTZC1_MPCBB1_PRIVCFGR2: RWRegister<u32>,

    /// GTZC1 SRAM1 MPCBB privileged configuration for super-block 3 register
    pub GTZC1_MPCBB1_PRIVCFGR3: RWRegister<u32>,

    /// GTZC1 SRAM1 MPCBB privileged configuration for super-block 4 register
    pub GTZC1_MPCBB1_PRIVCFGR4: RWRegister<u32>,

    /// GTZC1 SRAM1 MPCBB privileged configuration for super-block 5 register
    pub GTZC1_MPCBB1_PRIVCFGR5: RWRegister<u32>,

    /// GTZC1 SRAM1 MPCBB privileged configuration for super-block 6 register
    pub GTZC1_MPCBB1_PRIVCFGR6: RWRegister<u32>,

    /// GTZC1 SRAM1 MPCBB privileged configuration for super-block 7 register
    pub GTZC1_MPCBB1_PRIVCFGR7: RWRegister<u32>,

    /// GTZC1 SRAM1 MPCBB privileged configuration for super-block 8 register
    pub GTZC1_MPCBB1_PRIVCFGR8: RWRegister<u32>,

    /// GTZC1 SRAM1 MPCBB privileged configuration for super-block 9 register
    pub GTZC1_MPCBB1_PRIVCFGR9: RWRegister<u32>,

    /// GTZC1 SRAM1 MPCBB privileged configuration for super-block 10 register
    pub GTZC1_MPCBB1_PRIVCFGR10: RWRegister<u32>,

    /// GTZC1 SRAM1 MPCBB privileged configuration for super-block 11 register
    pub GTZC1_MPCBB1_PRIVCFGR11: RWRegister<u32>,

    /// GTZC1 SRAM1 MPCBB privileged configuration for super-block 12 register
    pub GTZC1_MPCBB1_PRIVCFGR12: RWRegister<u32>,

    /// GTZC1 SRAM1 MPCBB privileged configuration for super-block 13 register
    pub GTZC1_MPCBB1_PRIVCFGR13: RWRegister<u32>,

    /// GTZC1 SRAM1 MPCBB privileged configuration for super-block 14 register
    pub GTZC1_MPCBB1_PRIVCFGR14: RWRegister<u32>,

    /// GTZC1 SRAM1 MPCBB privileged configuration for super-block 15 register
    pub GTZC1_MPCBB1_PRIVCFGR15: RWRegister<u32>,

    /// GTZC1 SRAM1 MPCBB privileged configuration for super-block 16 register
    pub GTZC1_MPCBB1_PRIVCFGR16: RWRegister<u32>,

    /// GTZC1 SRAM1 MPCBB privileged configuration for super-block 17 register
    pub GTZC1_MPCBB1_PRIVCFGR17: RWRegister<u32>,

    /// GTZC1 SRAM1 MPCBB privileged configuration for super-block 18 register
    pub GTZC1_MPCBB1_PRIVCFGR18: RWRegister<u32>,

    /// GTZC1 SRAM1 MPCBB privileged configuration for super-block 19 register
    pub GTZC1_MPCBB1_PRIVCFGR19: RWRegister<u32>,

    /// GTZC1 SRAM1 MPCBB privileged configuration for super-block 20 register
    pub GTZC1_MPCBB1_PRIVCFGR20: RWRegister<u32>,

    /// GTZC1 SRAM1 MPCBB privileged configuration for super-block 21 register
    pub GTZC1_MPCBB1_PRIVCFGR21: RWRegister<u32>,

    /// GTZC1 SRAM1 MPCBB privileged configuration for super-block 22 register
    pub GTZC1_MPCBB1_PRIVCFGR22: RWRegister<u32>,

    /// GTZC1 SRAM1 MPCBB privileged configuration for super-block 23 register
    pub GTZC1_MPCBB1_PRIVCFGR23: RWRegister<u32>,

    /// GTZC1 SRAM1 MPCBB privileged configuration for super-block 24 register
    pub GTZC1_MPCBB1_PRIVCFGR24: RWRegister<u32>,

    /// GTZC1 SRAM1 MPCBB privileged configuration for super-block 25 register
    pub GTZC1_MPCBB1_PRIVCFGR25: RWRegister<u32>,

    /// GTZC1 SRAM1 MPCBB privileged configuration for super-block 26 register
    pub GTZC1_MPCBB1_PRIVCFGR26: RWRegister<u32>,

    /// GTZC1 SRAM1 MPCBB privileged configuration for super-block 27 register
    pub GTZC1_MPCBB1_PRIVCFGR27: RWRegister<u32>,

    /// GTZC1 SRAM1 MPCBB privileged configuration for super-block 28 register
    pub GTZC1_MPCBB1_PRIVCFGR28: RWRegister<u32>,

    /// GTZC1 SRAM1 MPCBB privileged configuration for super-block 29 register
    pub GTZC1_MPCBB1_PRIVCFGR29: RWRegister<u32>,

    /// GTZC1 SRAM1 MPCBB privileged configuration for super-block 30 register
    pub GTZC1_MPCBB1_PRIVCFGR30: RWRegister<u32>,

    /// GTZC1 SRAM1 MPCBB privileged configuration for super-block 31 register
    pub GTZC1_MPCBB1_PRIVCFGR31: RWRegister<u32>,

    _reserved4: [u8; 896],

    /// GTZC1 SRAM2 MPCBB privileged configuration for super-block 0 register
    pub GTZC1_MPCBB2_PRIVCFGR0: RWRegister<u32>,

    /// GTZC1 SRAM2 MPCBB privileged configuration for super-block 1 register
    pub GTZC1_MPCBB2_PRIVCFGR1: RWRegister<u32>,

    /// GTZC1 SRAM2 MPCBB privileged configuration for super-block 2 register
    pub GTZC1_MPCBB2_PRIVCFGR2: RWRegister<u32>,

    /// GTZC1 SRAM2 MPCBB privileged configuration for super-block 3 register
    pub GTZC1_MPCBB2_PRIVCFGR3: RWRegister<u32>,

    /// GTZC1 SRAM2 MPCBB privileged configuration for super-block 4 register
    pub GTZC1_MPCBB2_PRIVCFGR4: RWRegister<u32>,

    /// GTZC1 SRAM2 MPCBB privileged configuration for super-block 5 register
    pub GTZC1_MPCBB2_PRIVCFGR5: RWRegister<u32>,

    /// GTZC1 SRAM2 MPCBB privileged configuration for super-block 6 register
    pub GTZC1_MPCBB2_PRIVCFGR6: RWRegister<u32>,

    /// GTZC1 SRAM2 MPCBB privileged configuration for super-block 7 register
    pub GTZC1_MPCBB2_PRIVCFGR7: RWRegister<u32>,

    /// GTZC1 SRAM2 MPCBB privileged configuration for super-block 8 register
    pub GTZC1_MPCBB2_PRIVCFGR8: RWRegister<u32>,

    /// GTZC1 SRAM2 MPCBB privileged configuration for super-block 9 register
    pub GTZC1_MPCBB2_PRIVCFGR9: RWRegister<u32>,

    /// GTZC1 SRAM2 MPCBB privileged configuration for super-block 10 register
    pub GTZC1_MPCBB2_PRIVCFGR10: RWRegister<u32>,

    /// GTZC1 SRAM2 MPCBB privileged configuration for super-block 11 register
    pub GTZC1_MPCBB2_PRIVCFGR11: RWRegister<u32>,

    /// GTZC1 SRAM2 MPCBB privileged configuration for super-block 12 register
    pub GTZC1_MPCBB2_PRIVCFGR12: RWRegister<u32>,

    /// GTZC1 SRAM2 MPCBB privileged configuration for super-block 13 register
    pub GTZC1_MPCBB2_PRIVCFGR13: RWRegister<u32>,

    /// GTZC1 SRAM2 MPCBB privileged configuration for super-block 14 register
    pub GTZC1_MPCBB2_PRIVCFGR14: RWRegister<u32>,

    /// GTZC1 SRAM2 MPCBB privileged configuration for super-block 15 register
    pub GTZC1_MPCBB2_PRIVCFGR15: RWRegister<u32>,

    /// GTZC1 SRAM2 MPCBB privileged configuration for super-block 16 register
    pub GTZC1_MPCBB2_PRIVCFGR16: RWRegister<u32>,

    /// GTZC1 SRAM2 MPCBB privileged configuration for super-block 17 register
    pub GTZC1_MPCBB2_PRIVCFGR17: RWRegister<u32>,

    /// GTZC1 SRAM2 MPCBB privileged configuration for super-block 18 register
    pub GTZC1_MPCBB2_PRIVCFGR18: RWRegister<u32>,

    /// GTZC1 SRAM2 MPCBB privileged configuration for super-block 19 register
    pub GTZC1_MPCBB2_PRIVCFGR19: RWRegister<u32>,

    /// GTZC1 SRAM2 MPCBB privileged configuration for super-block 20 register
    pub GTZC1_MPCBB2_PRIVCFGR20: RWRegister<u32>,

    /// GTZC1 SRAM2 MPCBB privileged configuration for super-block 21 register
    pub GTZC1_MPCBB2_PRIVCFGR21: RWRegister<u32>,

    /// GTZC1 SRAM2 MPCBB privileged configuration for super-block 22 register
    pub GTZC1_MPCBB2_PRIVCFGR22: RWRegister<u32>,

    /// GTZC1 SRAM2 MPCBB privileged configuration for super-block 23 register
    pub GTZC1_MPCBB2_PRIVCFGR23: RWRegister<u32>,

    /// GTZC1 SRAM2 MPCBB privileged configuration for super-block 24 register
    pub GTZC1_MPCBB2_PRIVCFGR24: RWRegister<u32>,

    /// GTZC1 SRAM2 MPCBB privileged configuration for super-block 25 register
    pub GTZC1_MPCBB2_PRIVCFGR25: RWRegister<u32>,

    /// GTZC1 SRAM2 MPCBB privileged configuration for super-block 26 register
    pub GTZC1_MPCBB2_PRIVCFGR26: RWRegister<u32>,

    /// GTZC1 SRAM2 MPCBB privileged configuration for super-block 27 register
    pub GTZC1_MPCBB2_PRIVCFGR27: RWRegister<u32>,

    /// GTZC1 SRAM2 MPCBB privileged configuration for super-block 28 register
    pub GTZC1_MPCBB2_PRIVCFGR28: RWRegister<u32>,

    /// GTZC1 SRAM2 MPCBB privileged configuration for super-block 29 register
    pub GTZC1_MPCBB2_PRIVCFGR29: RWRegister<u32>,

    /// GTZC1 SRAM2 MPCBB privileged configuration for super-block 30 register
    pub GTZC1_MPCBB2_PRIVCFGR30: RWRegister<u32>,

    /// GTZC1 SRAM2 MPCBB privileged configuration for super-block 31 register
    pub GTZC1_MPCBB2_PRIVCFGR31: RWRegister<u32>,
}
pub struct ResetValues {
    pub GTZC1_TZSC_PRIVCFGR1: u32,
    pub GTZC1_TZSC_PRIVCFGR2: u32,
    pub GTZC1_TZSC_PRIVCFGR3: u32,
    pub GTZC1_TZSC_MPCWM4ACFGR: u32,
    pub GTZC1_TZSC_MPCWM4AR: u32,
    pub GTZC1_TZSC_MPCWM4BCFGR: u32,
    pub GTZC1_TZSC_MPCWM4BR: u32,
    pub GTZC1_MPCBB1_PRIVCFGR0: u32,
    pub GTZC1_MPCBB1_PRIVCFGR1: u32,
    pub GTZC1_MPCBB1_PRIVCFGR2: u32,
    pub GTZC1_MPCBB1_PRIVCFGR3: u32,
    pub GTZC1_MPCBB1_PRIVCFGR4: u32,
    pub GTZC1_MPCBB1_PRIVCFGR5: u32,
    pub GTZC1_MPCBB1_PRIVCFGR6: u32,
    pub GTZC1_MPCBB1_PRIVCFGR7: u32,
    pub GTZC1_MPCBB1_PRIVCFGR8: u32,
    pub GTZC1_MPCBB1_PRIVCFGR9: u32,
    pub GTZC1_MPCBB1_PRIVCFGR10: u32,
    pub GTZC1_MPCBB1_PRIVCFGR11: u32,
    pub GTZC1_MPCBB1_PRIVCFGR12: u32,
    pub GTZC1_MPCBB1_PRIVCFGR13: u32,
    pub GTZC1_MPCBB1_PRIVCFGR14: u32,
    pub GTZC1_MPCBB1_PRIVCFGR15: u32,
    pub GTZC1_MPCBB1_PRIVCFGR16: u32,
    pub GTZC1_MPCBB1_PRIVCFGR17: u32,
    pub GTZC1_MPCBB1_PRIVCFGR18: u32,
    pub GTZC1_MPCBB1_PRIVCFGR19: u32,
    pub GTZC1_MPCBB1_PRIVCFGR20: u32,
    pub GTZC1_MPCBB1_PRIVCFGR21: u32,
    pub GTZC1_MPCBB1_PRIVCFGR22: u32,
    pub GTZC1_MPCBB1_PRIVCFGR23: u32,
    pub GTZC1_MPCBB1_PRIVCFGR24: u32,
    pub GTZC1_MPCBB1_PRIVCFGR25: u32,
    pub GTZC1_MPCBB1_PRIVCFGR26: u32,
    pub GTZC1_MPCBB1_PRIVCFGR27: u32,
    pub GTZC1_MPCBB1_PRIVCFGR28: u32,
    pub GTZC1_MPCBB1_PRIVCFGR29: u32,
    pub GTZC1_MPCBB1_PRIVCFGR30: u32,
    pub GTZC1_MPCBB1_PRIVCFGR31: u32,
    pub GTZC1_MPCBB2_PRIVCFGR0: u32,
    pub GTZC1_MPCBB2_PRIVCFGR1: u32,
    pub GTZC1_MPCBB2_PRIVCFGR2: u32,
    pub GTZC1_MPCBB2_PRIVCFGR3: u32,
    pub GTZC1_MPCBB2_PRIVCFGR4: u32,
    pub GTZC1_MPCBB2_PRIVCFGR5: u32,
    pub GTZC1_MPCBB2_PRIVCFGR6: u32,
    pub GTZC1_MPCBB2_PRIVCFGR7: u32,
    pub GTZC1_MPCBB2_PRIVCFGR8: u32,
    pub GTZC1_MPCBB2_PRIVCFGR9: u32,
    pub GTZC1_MPCBB2_PRIVCFGR10: u32,
    pub GTZC1_MPCBB2_PRIVCFGR11: u32,
    pub GTZC1_MPCBB2_PRIVCFGR12: u32,
    pub GTZC1_MPCBB2_PRIVCFGR13: u32,
    pub GTZC1_MPCBB2_PRIVCFGR14: u32,
    pub GTZC1_MPCBB2_PRIVCFGR15: u32,
    pub GTZC1_MPCBB2_PRIVCFGR16: u32,
    pub GTZC1_MPCBB2_PRIVCFGR17: u32,
    pub GTZC1_MPCBB2_PRIVCFGR18: u32,
    pub GTZC1_MPCBB2_PRIVCFGR19: u32,
    pub GTZC1_MPCBB2_PRIVCFGR20: u32,
    pub GTZC1_MPCBB2_PRIVCFGR21: u32,
    pub GTZC1_MPCBB2_PRIVCFGR22: u32,
    pub GTZC1_MPCBB2_PRIVCFGR23: u32,
    pub GTZC1_MPCBB2_PRIVCFGR24: u32,
    pub GTZC1_MPCBB2_PRIVCFGR25: u32,
    pub GTZC1_MPCBB2_PRIVCFGR26: u32,
    pub GTZC1_MPCBB2_PRIVCFGR27: u32,
    pub GTZC1_MPCBB2_PRIVCFGR28: u32,
    pub GTZC1_MPCBB2_PRIVCFGR29: u32,
    pub GTZC1_MPCBB2_PRIVCFGR30: u32,
    pub GTZC1_MPCBB2_PRIVCFGR31: u32,
}
#[cfg(not(feature = "nosync"))]
pub struct Instance {
    pub(crate) addr: u32,
    pub(crate) _marker: PhantomData<*const RegisterBlock>,
}
#[cfg(not(feature = "nosync"))]
impl ::core::ops::Deref for Instance {
    type Target = RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &RegisterBlock {
        unsafe { &*(self.addr as *const _) }
    }
}
#[cfg(feature = "rtic")]
unsafe impl Send for Instance {}

/// Access functions for the GTZC1 peripheral instance
pub mod GTZC1 {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x40032400,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in GTZC1
    pub const reset: ResetValues = ResetValues {
        GTZC1_TZSC_PRIVCFGR1: 0x00000000,
        GTZC1_TZSC_PRIVCFGR2: 0x00000000,
        GTZC1_TZSC_PRIVCFGR3: 0x00000000,
        GTZC1_TZSC_MPCWM4ACFGR: 0x00000000,
        GTZC1_TZSC_MPCWM4AR: 0x08000000,
        GTZC1_TZSC_MPCWM4BCFGR: 0x00000000,
        GTZC1_TZSC_MPCWM4BR: 0x08000000,
        GTZC1_MPCBB1_PRIVCFGR0: 0x00000000,
        GTZC1_MPCBB1_PRIVCFGR1: 0x00000000,
        GTZC1_MPCBB1_PRIVCFGR2: 0x00000000,
        GTZC1_MPCBB1_PRIVCFGR3: 0x00000000,
        GTZC1_MPCBB1_PRIVCFGR4: 0x00000000,
        GTZC1_MPCBB1_PRIVCFGR5: 0x00000000,
        GTZC1_MPCBB1_PRIVCFGR6: 0x00000000,
        GTZC1_MPCBB1_PRIVCFGR7: 0x00000000,
        GTZC1_MPCBB1_PRIVCFGR8: 0x00000000,
        GTZC1_MPCBB1_PRIVCFGR9: 0x00000000,
        GTZC1_MPCBB1_PRIVCFGR10: 0x00000000,
        GTZC1_MPCBB1_PRIVCFGR11: 0x00000000,
        GTZC1_MPCBB1_PRIVCFGR12: 0x00000000,
        GTZC1_MPCBB1_PRIVCFGR13: 0x00000000,
        GTZC1_MPCBB1_PRIVCFGR14: 0x00000000,
        GTZC1_MPCBB1_PRIVCFGR15: 0x00000000,
        GTZC1_MPCBB1_PRIVCFGR16: 0x00000000,
        GTZC1_MPCBB1_PRIVCFGR17: 0x00000000,
        GTZC1_MPCBB1_PRIVCFGR18: 0x00000000,
        GTZC1_MPCBB1_PRIVCFGR19: 0x00000000,
        GTZC1_MPCBB1_PRIVCFGR20: 0x00000000,
        GTZC1_MPCBB1_PRIVCFGR21: 0x00000000,
        GTZC1_MPCBB1_PRIVCFGR22: 0x00000000,
        GTZC1_MPCBB1_PRIVCFGR23: 0x00000000,
        GTZC1_MPCBB1_PRIVCFGR24: 0x00000000,
        GTZC1_MPCBB1_PRIVCFGR25: 0x00000000,
        GTZC1_MPCBB1_PRIVCFGR26: 0x00000000,
        GTZC1_MPCBB1_PRIVCFGR27: 0x00000000,
        GTZC1_MPCBB1_PRIVCFGR28: 0x00000000,
        GTZC1_MPCBB1_PRIVCFGR29: 0x00000000,
        GTZC1_MPCBB1_PRIVCFGR30: 0x00000000,
        GTZC1_MPCBB1_PRIVCFGR31: 0x00000000,
        GTZC1_MPCBB2_PRIVCFGR0: 0x00000000,
        GTZC1_MPCBB2_PRIVCFGR1: 0x00000000,
        GTZC1_MPCBB2_PRIVCFGR2: 0x00000000,
        GTZC1_MPCBB2_PRIVCFGR3: 0x00000000,
        GTZC1_MPCBB2_PRIVCFGR4: 0x00000000,
        GTZC1_MPCBB2_PRIVCFGR5: 0x00000000,
        GTZC1_MPCBB2_PRIVCFGR6: 0x00000000,
        GTZC1_MPCBB2_PRIVCFGR7: 0x00000000,
        GTZC1_MPCBB2_PRIVCFGR8: 0x00000000,
        GTZC1_MPCBB2_PRIVCFGR9: 0x00000000,
        GTZC1_MPCBB2_PRIVCFGR10: 0x00000000,
        GTZC1_MPCBB2_PRIVCFGR11: 0x00000000,
        GTZC1_MPCBB2_PRIVCFGR12: 0x00000000,
        GTZC1_MPCBB2_PRIVCFGR13: 0x00000000,
        GTZC1_MPCBB2_PRIVCFGR14: 0x00000000,
        GTZC1_MPCBB2_PRIVCFGR15: 0x00000000,
        GTZC1_MPCBB2_PRIVCFGR16: 0x00000000,
        GTZC1_MPCBB2_PRIVCFGR17: 0x00000000,
        GTZC1_MPCBB2_PRIVCFGR18: 0x00000000,
        GTZC1_MPCBB2_PRIVCFGR19: 0x00000000,
        GTZC1_MPCBB2_PRIVCFGR20: 0x00000000,
        GTZC1_MPCBB2_PRIVCFGR21: 0x00000000,
        GTZC1_MPCBB2_PRIVCFGR22: 0x00000000,
        GTZC1_MPCBB2_PRIVCFGR23: 0x00000000,
        GTZC1_MPCBB2_PRIVCFGR24: 0x00000000,
        GTZC1_MPCBB2_PRIVCFGR25: 0x00000000,
        GTZC1_MPCBB2_PRIVCFGR26: 0x00000000,
        GTZC1_MPCBB2_PRIVCFGR27: 0x00000000,
        GTZC1_MPCBB2_PRIVCFGR28: 0x00000000,
        GTZC1_MPCBB2_PRIVCFGR29: 0x00000000,
        GTZC1_MPCBB2_PRIVCFGR30: 0x00000000,
        GTZC1_MPCBB2_PRIVCFGR31: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut GTZC1_TAKEN: bool = false;

    /// Safe access to GTZC1
    ///
    /// This function returns `Some(Instance)` if this instance is not
    /// currently taken, and `None` if it is. This ensures that if you
    /// do get `Some(Instance)`, you are ensured unique access to
    /// the peripheral and there cannot be data races (unless other
    /// code uses `unsafe`, of course). You can then pass the
    /// `Instance` around to other functions as required. When you're
    /// done with it, you can call `release(instance)` to return it.
    ///
    /// `Instance` itself dereferences to a `RegisterBlock`, which
    /// provides access to the peripheral's registers.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn take() -> Option<Instance> {
        external_cortex_m::interrupt::free(|_| unsafe {
            if GTZC1_TAKEN {
                None
            } else {
                GTZC1_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to GTZC1
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if GTZC1_TAKEN && inst.addr == INSTANCE.addr {
                GTZC1_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal GTZC1
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        GTZC1_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to GTZC1
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const GTZC1: *const RegisterBlock = 0x40032400 as *const _;
