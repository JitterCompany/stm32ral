#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Digital filter for sigma delta modulators
//!
//! Used by: stm32h743, stm32h743v, stm32h747cm7, stm32h753, stm32h753v, stm32h757cm7

use crate::{RORegister, RWRegister};
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// DFSDM channel configuration 0 register 1
pub mod DFSDM_CHCFG0R1 {

    /// Serial interface type for channel 0
    pub mod SITP {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (2 bits: 0b11 << 0)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// SPI clock select for channel 0
    pub mod SPICKSEL {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (2 bits: 0b11 << 2)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Short-circuit detector enable on channel 0
    pub mod SCDEN {
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

    /// Clock absence detector enable on channel 0
    pub mod CKABEN {
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

    /// Channel 0 enable
    pub mod CHEN {
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

    /// Channel inputs selection
    pub mod CHINSEL {
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

    /// Input data multiplexer for channel 0
    pub mod DATMPX {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (2 bits: 0b11 << 12)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Data packing mode in DFSDM_CHDATINyR register
    pub mod DATPACK {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (2 bits: 0b11 << 14)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Output serial clock divider
    pub mod CKOUTDIV {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (8 bits: 0xff << 16)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Output serial clock source selection
    pub mod CKOUTSRC {
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

    /// Global enable for DFSDM interface
    pub mod DFSDMEN {
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

/// DFSDM channel configuration 1 register 1
pub mod DFSDM_CHCFG1R1 {
    pub use super::DFSDM_CHCFG0R1::CHEN;
    pub use super::DFSDM_CHCFG0R1::CHINSEL;
    pub use super::DFSDM_CHCFG0R1::CKABEN;
    pub use super::DFSDM_CHCFG0R1::CKOUTDIV;
    pub use super::DFSDM_CHCFG0R1::CKOUTSRC;
    pub use super::DFSDM_CHCFG0R1::DATMPX;
    pub use super::DFSDM_CHCFG0R1::DATPACK;
    pub use super::DFSDM_CHCFG0R1::DFSDMEN;
    pub use super::DFSDM_CHCFG0R1::SCDEN;
    pub use super::DFSDM_CHCFG0R1::SITP;
    pub use super::DFSDM_CHCFG0R1::SPICKSEL;
}

/// DFSDM channel configuration 2 register 1
pub mod DFSDM_CHCFG2R1 {
    pub use super::DFSDM_CHCFG0R1::CHEN;
    pub use super::DFSDM_CHCFG0R1::CHINSEL;
    pub use super::DFSDM_CHCFG0R1::CKABEN;
    pub use super::DFSDM_CHCFG0R1::CKOUTDIV;
    pub use super::DFSDM_CHCFG0R1::CKOUTSRC;
    pub use super::DFSDM_CHCFG0R1::DATMPX;
    pub use super::DFSDM_CHCFG0R1::DATPACK;
    pub use super::DFSDM_CHCFG0R1::DFSDMEN;
    pub use super::DFSDM_CHCFG0R1::SCDEN;
    pub use super::DFSDM_CHCFG0R1::SITP;
    pub use super::DFSDM_CHCFG0R1::SPICKSEL;
}

/// DFSDM channel configuration 3 register 1
pub mod DFSDM_CHCFG3R1 {
    pub use super::DFSDM_CHCFG0R1::CHEN;
    pub use super::DFSDM_CHCFG0R1::CHINSEL;
    pub use super::DFSDM_CHCFG0R1::CKABEN;
    pub use super::DFSDM_CHCFG0R1::CKOUTDIV;
    pub use super::DFSDM_CHCFG0R1::CKOUTSRC;
    pub use super::DFSDM_CHCFG0R1::DATMPX;
    pub use super::DFSDM_CHCFG0R1::DATPACK;
    pub use super::DFSDM_CHCFG0R1::DFSDMEN;
    pub use super::DFSDM_CHCFG0R1::SCDEN;
    pub use super::DFSDM_CHCFG0R1::SITP;
    pub use super::DFSDM_CHCFG0R1::SPICKSEL;
}

/// DFSDM channel configuration 4 register 1
pub mod DFSDM_CHCFG4R1 {
    pub use super::DFSDM_CHCFG0R1::CHEN;
    pub use super::DFSDM_CHCFG0R1::CHINSEL;
    pub use super::DFSDM_CHCFG0R1::CKABEN;
    pub use super::DFSDM_CHCFG0R1::CKOUTDIV;
    pub use super::DFSDM_CHCFG0R1::CKOUTSRC;
    pub use super::DFSDM_CHCFG0R1::DATMPX;
    pub use super::DFSDM_CHCFG0R1::DATPACK;
    pub use super::DFSDM_CHCFG0R1::DFSDMEN;
    pub use super::DFSDM_CHCFG0R1::SCDEN;
    pub use super::DFSDM_CHCFG0R1::SITP;
    pub use super::DFSDM_CHCFG0R1::SPICKSEL;
}

/// DFSDM channel configuration 5 register 1
pub mod DFSDM_CHCFG5R1 {
    pub use super::DFSDM_CHCFG0R1::CHEN;
    pub use super::DFSDM_CHCFG0R1::CHINSEL;
    pub use super::DFSDM_CHCFG0R1::CKABEN;
    pub use super::DFSDM_CHCFG0R1::CKOUTDIV;
    pub use super::DFSDM_CHCFG0R1::CKOUTSRC;
    pub use super::DFSDM_CHCFG0R1::DATMPX;
    pub use super::DFSDM_CHCFG0R1::DATPACK;
    pub use super::DFSDM_CHCFG0R1::DFSDMEN;
    pub use super::DFSDM_CHCFG0R1::SCDEN;
    pub use super::DFSDM_CHCFG0R1::SITP;
    pub use super::DFSDM_CHCFG0R1::SPICKSEL;
}

/// DFSDM channel configuration 6 register 1
pub mod DFSDM_CHCFG6R1 {
    pub use super::DFSDM_CHCFG0R1::CHEN;
    pub use super::DFSDM_CHCFG0R1::CHINSEL;
    pub use super::DFSDM_CHCFG0R1::CKABEN;
    pub use super::DFSDM_CHCFG0R1::CKOUTDIV;
    pub use super::DFSDM_CHCFG0R1::CKOUTSRC;
    pub use super::DFSDM_CHCFG0R1::DATMPX;
    pub use super::DFSDM_CHCFG0R1::DATPACK;
    pub use super::DFSDM_CHCFG0R1::DFSDMEN;
    pub use super::DFSDM_CHCFG0R1::SCDEN;
    pub use super::DFSDM_CHCFG0R1::SITP;
    pub use super::DFSDM_CHCFG0R1::SPICKSEL;
}

/// DFSDM channel configuration 7 register 1
pub mod DFSDM_CHCFG7R1 {
    pub use super::DFSDM_CHCFG0R1::CHEN;
    pub use super::DFSDM_CHCFG0R1::CHINSEL;
    pub use super::DFSDM_CHCFG0R1::CKABEN;
    pub use super::DFSDM_CHCFG0R1::CKOUTDIV;
    pub use super::DFSDM_CHCFG0R1::CKOUTSRC;
    pub use super::DFSDM_CHCFG0R1::DATMPX;
    pub use super::DFSDM_CHCFG0R1::DATPACK;
    pub use super::DFSDM_CHCFG0R1::DFSDMEN;
    pub use super::DFSDM_CHCFG0R1::SCDEN;
    pub use super::DFSDM_CHCFG0R1::SITP;
    pub use super::DFSDM_CHCFG0R1::SPICKSEL;
}

/// DFSDM channel configuration 0 register 2
pub mod DFSDM_CHCFG0R2 {

    /// Data right bit-shift for channel 0
    pub mod DTRBS {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (5 bits: 0b11111 << 3)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// 24-bit calibration offset for channel 0
    pub mod OFFSET {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (24 bits: 0xffffff << 8)
        pub const mask: u32 = 0xffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// DFSDM channel configuration 1 register 2
pub mod DFSDM_CHCFG1R2 {
    pub use super::DFSDM_CHCFG0R2::DTRBS;
    pub use super::DFSDM_CHCFG0R2::OFFSET;
}

/// DFSDM channel configuration 2 register 2
pub mod DFSDM_CHCFG2R2 {
    pub use super::DFSDM_CHCFG0R2::DTRBS;
    pub use super::DFSDM_CHCFG0R2::OFFSET;
}

/// DFSDM channel configuration 3 register 2
pub mod DFSDM_CHCFG3R2 {
    pub use super::DFSDM_CHCFG0R2::DTRBS;
    pub use super::DFSDM_CHCFG0R2::OFFSET;
}

/// DFSDM channel configuration 4 register 2
pub mod DFSDM_CHCFG4R2 {
    pub use super::DFSDM_CHCFG0R2::DTRBS;
    pub use super::DFSDM_CHCFG0R2::OFFSET;
}

/// DFSDM channel configuration 5 register 2
pub mod DFSDM_CHCFG5R2 {
    pub use super::DFSDM_CHCFG0R2::DTRBS;
    pub use super::DFSDM_CHCFG0R2::OFFSET;
}

/// DFSDM channel configuration 6 register 2
pub mod DFSDM_CHCFG6R2 {
    pub use super::DFSDM_CHCFG0R2::DTRBS;
    pub use super::DFSDM_CHCFG0R2::OFFSET;
}

/// DFSDM channel configuration 7 register 2
pub mod DFSDM_CHCFG7R2 {
    pub use super::DFSDM_CHCFG0R2::DTRBS;
    pub use super::DFSDM_CHCFG0R2::OFFSET;
}

/// DFSDM analog watchdog and short-circuit detector register
pub mod DFSDM_AWSCD0R {

    /// short-circuit detector threshold for channel 0
    pub mod SCDT {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (8 bits: 0xff << 0)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Break signal assignment for short-circuit detector on channel 0
    pub mod BKSCD {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (4 bits: 0b1111 << 12)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Analog watchdog filter oversampling ratio (decimation rate) on channel 0
    pub mod AWFOSR {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (5 bits: 0b11111 << 16)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Analog watchdog Sinc filter order on channel 0
    pub mod AWFORD {
        /// Offset (22 bits)
        pub const offset: u32 = 22;
        /// Mask (2 bits: 0b11 << 22)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// DFSDM analog watchdog and short-circuit detector register
pub mod DFSDM_AWSCD1R {
    pub use super::DFSDM_AWSCD0R::AWFORD;
    pub use super::DFSDM_AWSCD0R::AWFOSR;
    pub use super::DFSDM_AWSCD0R::BKSCD;
    pub use super::DFSDM_AWSCD0R::SCDT;
}

/// DFSDM analog watchdog and short-circuit detector register
pub mod DFSDM_AWSCD2R {
    pub use super::DFSDM_AWSCD0R::AWFORD;
    pub use super::DFSDM_AWSCD0R::AWFOSR;
    pub use super::DFSDM_AWSCD0R::BKSCD;
    pub use super::DFSDM_AWSCD0R::SCDT;
}

/// DFSDM analog watchdog and short-circuit detector register
pub mod DFSDM_AWSCD3R {
    pub use super::DFSDM_AWSCD0R::AWFORD;
    pub use super::DFSDM_AWSCD0R::AWFOSR;
    pub use super::DFSDM_AWSCD0R::BKSCD;
    pub use super::DFSDM_AWSCD0R::SCDT;
}

/// DFSDM analog watchdog and short-circuit detector register
pub mod DFSDM_AWSCD4R {
    pub use super::DFSDM_AWSCD0R::AWFORD;
    pub use super::DFSDM_AWSCD0R::AWFOSR;
    pub use super::DFSDM_AWSCD0R::BKSCD;
    pub use super::DFSDM_AWSCD0R::SCDT;
}

/// DFSDM analog watchdog and short-circuit detector register
pub mod DFSDM_AWSCD5R {
    pub use super::DFSDM_AWSCD0R::AWFORD;
    pub use super::DFSDM_AWSCD0R::AWFOSR;
    pub use super::DFSDM_AWSCD0R::BKSCD;
    pub use super::DFSDM_AWSCD0R::SCDT;
}

/// DFSDM analog watchdog and short-circuit detector register
pub mod DFSDM_AWSCD6R {
    pub use super::DFSDM_AWSCD0R::AWFORD;
    pub use super::DFSDM_AWSCD0R::AWFOSR;
    pub use super::DFSDM_AWSCD0R::BKSCD;
    pub use super::DFSDM_AWSCD0R::SCDT;
}

/// DFSDM analog watchdog and short-circuit detector register
pub mod DFSDM_AWSCD7R {
    pub use super::DFSDM_AWSCD0R::AWFORD;
    pub use super::DFSDM_AWSCD0R::AWFOSR;
    pub use super::DFSDM_AWSCD0R::BKSCD;
    pub use super::DFSDM_AWSCD0R::SCDT;
}

/// DFSDM channel watchdog filter data register
pub mod DFSDM_CHWDAT0R {

    /// Input channel y watchdog data
    pub mod WDATA {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (16 bits: 0xffff << 0)
        pub const mask: u32 = 0xffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// DFSDM channel watchdog filter data register
pub mod DFSDM_CHWDAT1R {
    pub use super::DFSDM_CHWDAT0R::WDATA;
}

/// DFSDM channel watchdog filter data register
pub mod DFSDM_CHWDAT2R {
    pub use super::DFSDM_CHWDAT0R::WDATA;
}

/// DFSDM channel watchdog filter data register
pub mod DFSDM_CHWDAT3R {
    pub use super::DFSDM_CHWDAT0R::WDATA;
}

/// DFSDM channel watchdog filter data register
pub mod DFSDM_CHWDAT4R {
    pub use super::DFSDM_CHWDAT0R::WDATA;
}

/// DFSDM channel watchdog filter data register
pub mod DFSDM_CHWDAT5R {
    pub use super::DFSDM_CHWDAT0R::WDATA;
}

/// DFSDM channel watchdog filter data register
pub mod DFSDM_CHWDAT6R {
    pub use super::DFSDM_CHWDAT0R::WDATA;
}

/// DFSDM channel watchdog filter data register
pub mod DFSDM_CHWDAT7R {
    pub use super::DFSDM_CHWDAT0R::WDATA;
}

/// DFSDM channel data input register
pub mod DFSDM_CHDATIN0R {

    /// Input data for channel 0
    pub mod INDAT0 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (16 bits: 0xffff << 0)
        pub const mask: u32 = 0xffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Input data for channel 1
    pub mod INDAT1 {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (16 bits: 0xffff << 16)
        pub const mask: u32 = 0xffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// DFSDM channel data input register
pub mod DFSDM_CHDATIN1R {
    pub use super::DFSDM_CHDATIN0R::INDAT0;
    pub use super::DFSDM_CHDATIN0R::INDAT1;
}

/// DFSDM channel data input register
pub mod DFSDM_CHDATIN2R {
    pub use super::DFSDM_CHDATIN0R::INDAT0;
    pub use super::DFSDM_CHDATIN0R::INDAT1;
}

/// DFSDM channel data input register
pub mod DFSDM_CHDATIN3R {
    pub use super::DFSDM_CHDATIN0R::INDAT0;
    pub use super::DFSDM_CHDATIN0R::INDAT1;
}

/// DFSDM channel data input register
pub mod DFSDM_CHDATIN4R {
    pub use super::DFSDM_CHDATIN0R::INDAT0;
    pub use super::DFSDM_CHDATIN0R::INDAT1;
}

/// DFSDM channel data input register
pub mod DFSDM_CHDATIN5R {
    pub use super::DFSDM_CHDATIN0R::INDAT0;
    pub use super::DFSDM_CHDATIN0R::INDAT1;
}

/// DFSDM channel data input register
pub mod DFSDM_CHDATIN6R {
    pub use super::DFSDM_CHDATIN0R::INDAT0;
    pub use super::DFSDM_CHDATIN0R::INDAT1;
}

/// DFSDM channel data input register
pub mod DFSDM_CHDATIN7R {
    pub use super::DFSDM_CHDATIN0R::INDAT0;
    pub use super::DFSDM_CHDATIN0R::INDAT1;
}

/// DFSDM control register 1
pub mod DFSDM0_CR1 {

    /// DFSDM enable
    pub mod DFEN {
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

    /// Start a conversion of the injected group of channels
    pub mod JSWSTART {
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

    /// Launch an injected conversion synchronously with the DFSDM0 JSWSTART trigger
    pub mod JSYNC {
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

    /// Scanning conversion mode for injected conversions
    pub mod JSCAN {
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

    /// DMA channel enabled to read data for the injected channel group
    pub mod JDMAEN {
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

    /// Trigger signal selection for launching injected conversions
    pub mod JEXTSEL {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (5 bits: 0b11111 << 8)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Trigger enable and trigger edge selection for injected conversions
    pub mod JEXTEN {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (2 bits: 0b11 << 13)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Software start of a conversion on the regular channel
    pub mod RSWSTART {
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

    /// Continuous mode selection for regular conversions
    pub mod RCONT {
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

    /// Launch regular conversion synchronously with DFSDM0
    pub mod RSYNC {
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

    /// DMA channel enabled to read data for the regular conversion
    pub mod RDMAEN {
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

    /// Regular channel selection
    pub mod RCH {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (3 bits: 0b111 << 24)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Fast conversion mode selection for regular conversions
    pub mod FAST {
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

    /// Analog watchdog fast mode select
    pub mod AWFSEL {
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
}

/// DFSDM control register 1
pub mod DFSDM1_CR1 {
    pub use super::DFSDM0_CR1::AWFSEL;
    pub use super::DFSDM0_CR1::DFEN;
    pub use super::DFSDM0_CR1::FAST;
    pub use super::DFSDM0_CR1::JDMAEN;
    pub use super::DFSDM0_CR1::JEXTEN;
    pub use super::DFSDM0_CR1::JEXTSEL;
    pub use super::DFSDM0_CR1::JSCAN;
    pub use super::DFSDM0_CR1::JSWSTART;
    pub use super::DFSDM0_CR1::JSYNC;
    pub use super::DFSDM0_CR1::RCH;
    pub use super::DFSDM0_CR1::RCONT;
    pub use super::DFSDM0_CR1::RDMAEN;
    pub use super::DFSDM0_CR1::RSWSTART;
    pub use super::DFSDM0_CR1::RSYNC;
}

/// DFSDM control register 1
pub mod DFSDM2_CR1 {
    pub use super::DFSDM0_CR1::AWFSEL;
    pub use super::DFSDM0_CR1::DFEN;
    pub use super::DFSDM0_CR1::FAST;
    pub use super::DFSDM0_CR1::JDMAEN;
    pub use super::DFSDM0_CR1::JEXTEN;
    pub use super::DFSDM0_CR1::JEXTSEL;
    pub use super::DFSDM0_CR1::JSCAN;
    pub use super::DFSDM0_CR1::JSWSTART;
    pub use super::DFSDM0_CR1::JSYNC;
    pub use super::DFSDM0_CR1::RCH;
    pub use super::DFSDM0_CR1::RCONT;
    pub use super::DFSDM0_CR1::RDMAEN;
    pub use super::DFSDM0_CR1::RSWSTART;
    pub use super::DFSDM0_CR1::RSYNC;
}

/// DFSDM control register 1
pub mod DFSDM3_CR1 {
    pub use super::DFSDM0_CR1::AWFSEL;
    pub use super::DFSDM0_CR1::DFEN;
    pub use super::DFSDM0_CR1::FAST;
    pub use super::DFSDM0_CR1::JDMAEN;
    pub use super::DFSDM0_CR1::JEXTEN;
    pub use super::DFSDM0_CR1::JEXTSEL;
    pub use super::DFSDM0_CR1::JSCAN;
    pub use super::DFSDM0_CR1::JSWSTART;
    pub use super::DFSDM0_CR1::JSYNC;
    pub use super::DFSDM0_CR1::RCH;
    pub use super::DFSDM0_CR1::RCONT;
    pub use super::DFSDM0_CR1::RDMAEN;
    pub use super::DFSDM0_CR1::RSWSTART;
    pub use super::DFSDM0_CR1::RSYNC;
}

/// DFSDM control register 2
pub mod DFSDM0_CR2 {

    /// Injected end of conversion interrupt enable
    pub mod JEOCIE {
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

    /// Regular end of conversion interrupt enable
    pub mod REOCIE {
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

    /// Injected data overrun interrupt enable
    pub mod JOVRIE {
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

    /// Regular data overrun interrupt enable
    pub mod ROVRIE {
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

    /// Analog watchdog interrupt enable
    pub mod AWDIE {
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

    /// Short-circuit detector interrupt enable
    pub mod SCDIE {
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

    /// Clock absence interrupt enable
    pub mod CKABIE {
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

    /// Extremes detector channel selection
    pub mod EXCH {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (8 bits: 0xff << 8)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Analog watchdog channel selection
    pub mod AWDCH {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (8 bits: 0xff << 16)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// DFSDM control register 2
pub mod DFSDM1_CR2 {
    pub use super::DFSDM0_CR2::AWDCH;
    pub use super::DFSDM0_CR2::AWDIE;
    pub use super::DFSDM0_CR2::CKABIE;
    pub use super::DFSDM0_CR2::EXCH;
    pub use super::DFSDM0_CR2::JEOCIE;
    pub use super::DFSDM0_CR2::JOVRIE;
    pub use super::DFSDM0_CR2::REOCIE;
    pub use super::DFSDM0_CR2::ROVRIE;
    pub use super::DFSDM0_CR2::SCDIE;
}

/// DFSDM control register 2
pub mod DFSDM2_CR2 {
    pub use super::DFSDM0_CR2::AWDCH;
    pub use super::DFSDM0_CR2::AWDIE;
    pub use super::DFSDM0_CR2::CKABIE;
    pub use super::DFSDM0_CR2::EXCH;
    pub use super::DFSDM0_CR2::JEOCIE;
    pub use super::DFSDM0_CR2::JOVRIE;
    pub use super::DFSDM0_CR2::REOCIE;
    pub use super::DFSDM0_CR2::ROVRIE;
    pub use super::DFSDM0_CR2::SCDIE;
}

/// DFSDM control register 2
pub mod DFSDM3_CR2 {
    pub use super::DFSDM0_CR2::AWDCH;
    pub use super::DFSDM0_CR2::AWDIE;
    pub use super::DFSDM0_CR2::CKABIE;
    pub use super::DFSDM0_CR2::EXCH;
    pub use super::DFSDM0_CR2::JEOCIE;
    pub use super::DFSDM0_CR2::JOVRIE;
    pub use super::DFSDM0_CR2::REOCIE;
    pub use super::DFSDM0_CR2::ROVRIE;
    pub use super::DFSDM0_CR2::SCDIE;
}

/// DFSDM interrupt and status register
pub mod DFSDM0_ISR {

    /// End of injected conversion flag
    pub mod JEOCF {
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

    /// End of regular conversion flag
    pub mod REOCF {
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

    /// Injected conversion overrun flag
    pub mod JOVRF {
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

    /// Regular conversion overrun flag
    pub mod ROVRF {
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

    /// Analog watchdog
    pub mod AWDF {
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

    /// Injected conversion in progress status
    pub mod JCIP {
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

    /// Regular conversion in progress status
    pub mod RCIP {
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

    /// Clock absence flag
    pub mod CKABF {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (8 bits: 0xff << 16)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// short-circuit detector flag
    pub mod SCDF {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (8 bits: 0xff << 24)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// DFSDM interrupt and status register
pub mod DFSDM1_ISR {
    pub use super::DFSDM0_ISR::AWDF;
    pub use super::DFSDM0_ISR::CKABF;
    pub use super::DFSDM0_ISR::JCIP;
    pub use super::DFSDM0_ISR::JEOCF;
    pub use super::DFSDM0_ISR::JOVRF;
    pub use super::DFSDM0_ISR::RCIP;
    pub use super::DFSDM0_ISR::REOCF;
    pub use super::DFSDM0_ISR::ROVRF;
    pub use super::DFSDM0_ISR::SCDF;
}

/// DFSDM interrupt and status register
pub mod DFSDM2_ISR {
    pub use super::DFSDM0_ISR::AWDF;
    pub use super::DFSDM0_ISR::CKABF;
    pub use super::DFSDM0_ISR::JCIP;
    pub use super::DFSDM0_ISR::JEOCF;
    pub use super::DFSDM0_ISR::JOVRF;
    pub use super::DFSDM0_ISR::RCIP;
    pub use super::DFSDM0_ISR::REOCF;
    pub use super::DFSDM0_ISR::ROVRF;
    pub use super::DFSDM0_ISR::SCDF;
}

/// DFSDM interrupt and status register
pub mod DFSDM3_ISR {
    pub use super::DFSDM0_ISR::AWDF;
    pub use super::DFSDM0_ISR::CKABF;
    pub use super::DFSDM0_ISR::JCIP;
    pub use super::DFSDM0_ISR::JEOCF;
    pub use super::DFSDM0_ISR::JOVRF;
    pub use super::DFSDM0_ISR::RCIP;
    pub use super::DFSDM0_ISR::REOCF;
    pub use super::DFSDM0_ISR::ROVRF;
    pub use super::DFSDM0_ISR::SCDF;
}

/// DFSDM interrupt flag clear register
pub mod DFSDM0_ICR {

    /// Clear the injected conversion overrun flag
    pub mod CLRJOVRF {
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

    /// Clear the regular conversion overrun flag
    pub mod CLRROVRF {
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

    /// Clear the clock absence flag
    pub mod CLRCKABF {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (8 bits: 0xff << 16)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Clear the short-circuit detector flag
    pub mod CLRSCDF {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (8 bits: 0xff << 24)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// DFSDM interrupt flag clear register
pub mod DFSDM1_ICR {
    pub use super::DFSDM0_ICR::CLRCKABF;
    pub use super::DFSDM0_ICR::CLRJOVRF;
    pub use super::DFSDM0_ICR::CLRROVRF;
    pub use super::DFSDM0_ICR::CLRSCDF;
}

/// DFSDM interrupt flag clear register
pub mod DFSDM2_ICR {
    pub use super::DFSDM0_ICR::CLRCKABF;
    pub use super::DFSDM0_ICR::CLRJOVRF;
    pub use super::DFSDM0_ICR::CLRROVRF;
    pub use super::DFSDM0_ICR::CLRSCDF;
}

/// DFSDM interrupt flag clear register
pub mod DFSDM3_ICR {
    pub use super::DFSDM0_ICR::CLRCKABF;
    pub use super::DFSDM0_ICR::CLRJOVRF;
    pub use super::DFSDM0_ICR::CLRROVRF;
    pub use super::DFSDM0_ICR::CLRSCDF;
}

/// DFSDM injected channel group selection register
pub mod DFSDM0_JCHGR {

    /// Injected channel group selection
    pub mod JCHG {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (8 bits: 0xff << 0)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// DFSDM injected channel group selection register
pub mod DFSDM1_JCHGR {
    pub use super::DFSDM0_JCHGR::JCHG;
}

/// DFSDM injected channel group selection register
pub mod DFSDM2_JCHGR {
    pub use super::DFSDM0_JCHGR::JCHG;
}

/// DFSDM injected channel group selection register
pub mod DFSDM3_JCHGR {
    pub use super::DFSDM0_JCHGR::JCHG;
}

/// DFSDM filter control register
pub mod DFSDM0_FCR {

    /// Integrator oversampling ratio (averaging length)
    pub mod IOSR {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (8 bits: 0xff << 0)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Sinc filter oversampling ratio (decimation rate)
    pub mod FOSR {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (10 bits: 0x3ff << 16)
        pub const mask: u32 = 0x3ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Sinc filter order
    pub mod FORD {
        /// Offset (29 bits)
        pub const offset: u32 = 29;
        /// Mask (3 bits: 0b111 << 29)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// DFSDM filter control register
pub mod DFSDM1_FCR {
    pub use super::DFSDM0_FCR::FORD;
    pub use super::DFSDM0_FCR::FOSR;
    pub use super::DFSDM0_FCR::IOSR;
}

/// DFSDM filter control register
pub mod DFSDM2_FCR {
    pub use super::DFSDM0_FCR::FORD;
    pub use super::DFSDM0_FCR::FOSR;
    pub use super::DFSDM0_FCR::IOSR;
}

/// DFSDM filter control register
pub mod DFSDM3_FCR {
    pub use super::DFSDM0_FCR::FORD;
    pub use super::DFSDM0_FCR::FOSR;
    pub use super::DFSDM0_FCR::IOSR;
}

/// DFSDM data register for injected group
pub mod DFSDM0_JDATAR {

    /// Injected channel most recently converted
    pub mod JDATACH {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (3 bits: 0b111 << 0)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Injected group conversion data
    pub mod JDATA {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (24 bits: 0xffffff << 8)
        pub const mask: u32 = 0xffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// DFSDM data register for injected group
pub mod DFSDM1_JDATAR {
    pub use super::DFSDM0_JDATAR::JDATA;
    pub use super::DFSDM0_JDATAR::JDATACH;
}

/// DFSDM data register for injected group
pub mod DFSDM2_JDATAR {
    pub use super::DFSDM0_JDATAR::JDATA;
    pub use super::DFSDM0_JDATAR::JDATACH;
}

/// DFSDM data register for injected group
pub mod DFSDM3_JDATAR {
    pub use super::DFSDM0_JDATAR::JDATA;
    pub use super::DFSDM0_JDATAR::JDATACH;
}

/// DFSDM data register for the regular channel
pub mod DFSDM0_RDATAR {

    /// Regular channel most recently converted
    pub mod RDATACH {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (3 bits: 0b111 << 0)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Regular channel pending data
    pub mod RPEND {
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

    /// Regular channel conversion data
    pub mod RDATA {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (24 bits: 0xffffff << 8)
        pub const mask: u32 = 0xffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// DFSDM data register for the regular channel
pub mod DFSDM1_RDATAR {
    pub use super::DFSDM0_RDATAR::RDATA;
    pub use super::DFSDM0_RDATAR::RDATACH;
    pub use super::DFSDM0_RDATAR::RPEND;
}

/// DFSDM data register for the regular channel
pub mod DFSDM2_RDATAR {
    pub use super::DFSDM0_RDATAR::RDATA;
    pub use super::DFSDM0_RDATAR::RDATACH;
    pub use super::DFSDM0_RDATAR::RPEND;
}

/// DFSDM data register for the regular channel
pub mod DFSDM3_RDATAR {
    pub use super::DFSDM0_RDATAR::RDATA;
    pub use super::DFSDM0_RDATAR::RDATACH;
    pub use super::DFSDM0_RDATAR::RPEND;
}

/// DFSDM analog watchdog high threshold register
pub mod DFSDM0_AWHTR {

    /// Break signal assignment to analog watchdog high threshold event
    pub mod BKAWH {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Analog watchdog high threshold
    pub mod AWHT {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (24 bits: 0xffffff << 8)
        pub const mask: u32 = 0xffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// DFSDM analog watchdog high threshold register
pub mod DFSDM1_AWHTR {
    pub use super::DFSDM0_AWHTR::AWHT;
    pub use super::DFSDM0_AWHTR::BKAWH;
}

/// DFSDM analog watchdog high threshold register
pub mod DFSDM2_AWHTR {
    pub use super::DFSDM0_AWHTR::AWHT;
    pub use super::DFSDM0_AWHTR::BKAWH;
}

/// DFSDM analog watchdog high threshold register
pub mod DFSDM3_AWHTR {
    pub use super::DFSDM0_AWHTR::AWHT;
    pub use super::DFSDM0_AWHTR::BKAWH;
}

/// DFSDM analog watchdog low threshold register
pub mod DFSDM0_AWLTR {

    /// Break signal assignment to analog watchdog low threshold event
    pub mod BKAWL {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Analog watchdog low threshold
    pub mod AWLT {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (24 bits: 0xffffff << 8)
        pub const mask: u32 = 0xffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// DFSDM analog watchdog low threshold register
pub mod DFSDM1_AWLTR {
    pub use super::DFSDM0_AWLTR::AWLT;
    pub use super::DFSDM0_AWLTR::BKAWL;
}

/// DFSDM analog watchdog low threshold register
pub mod DFSDM2_AWLTR {
    pub use super::DFSDM0_AWLTR::AWLT;
    pub use super::DFSDM0_AWLTR::BKAWL;
}

/// DFSDM analog watchdog low threshold register
pub mod DFSDM3_AWLTR {
    pub use super::DFSDM0_AWLTR::AWLT;
    pub use super::DFSDM0_AWLTR::BKAWL;
}

/// DFSDM analog watchdog status register
pub mod DFSDM0_AWSR {

    /// Analog watchdog low threshold flag
    pub mod AWLTF {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (8 bits: 0xff << 0)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Analog watchdog high threshold flag
    pub mod AWHTF {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (8 bits: 0xff << 8)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// DFSDM analog watchdog status register
pub mod DFSDM1_AWSR {
    pub use super::DFSDM0_AWSR::AWHTF;
    pub use super::DFSDM0_AWSR::AWLTF;
}

/// DFSDM analog watchdog status register
pub mod DFSDM2_AWSR {
    pub use super::DFSDM0_AWSR::AWHTF;
    pub use super::DFSDM0_AWSR::AWLTF;
}

/// DFSDM analog watchdog status register
pub mod DFSDM3_AWSR {
    pub use super::DFSDM0_AWSR::AWHTF;
    pub use super::DFSDM0_AWSR::AWLTF;
}

/// DFSDM analog watchdog clear flag register
pub mod DFSDM0_AWCFR {

    /// Clear the analog watchdog low threshold flag
    pub mod CLRAWLTF {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (8 bits: 0xff << 0)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Clear the analog watchdog high threshold flag
    pub mod CLRAWHTF {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (8 bits: 0xff << 8)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// DFSDM analog watchdog clear flag register
pub mod DFSDM1_AWCFR {
    pub use super::DFSDM0_AWCFR::CLRAWHTF;
    pub use super::DFSDM0_AWCFR::CLRAWLTF;
}

/// DFSDM analog watchdog clear flag register
pub mod DFSDM2_AWCFR {
    pub use super::DFSDM0_AWCFR::CLRAWHTF;
    pub use super::DFSDM0_AWCFR::CLRAWLTF;
}

/// DFSDM analog watchdog clear flag register
pub mod DFSDM3_AWCFR {
    pub use super::DFSDM0_AWCFR::CLRAWHTF;
    pub use super::DFSDM0_AWCFR::CLRAWLTF;
}

/// DFSDM Extremes detector maximum register
pub mod DFSDM0_EXMAX {

    /// Extremes detector maximum data channel
    pub mod EXMAXCH {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (3 bits: 0b111 << 0)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Extremes detector maximum value
    pub mod EXMAX {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (24 bits: 0xffffff << 8)
        pub const mask: u32 = 0xffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// DFSDM Extremes detector maximum register
pub mod DFSDM1_EXMAX {
    pub use super::DFSDM0_EXMAX::EXMAX;
    pub use super::DFSDM0_EXMAX::EXMAXCH;
}

/// DFSDM Extremes detector maximum register
pub mod DFSDM2_EXMAX {
    pub use super::DFSDM0_EXMAX::EXMAX;
    pub use super::DFSDM0_EXMAX::EXMAXCH;
}

/// DFSDM Extremes detector maximum register
pub mod DFSDM3_EXMAX {
    pub use super::DFSDM0_EXMAX::EXMAX;
    pub use super::DFSDM0_EXMAX::EXMAXCH;
}

/// DFSDM Extremes detector minimum register
pub mod DFSDM0_EXMIN {

    /// Extremes detector minimum data channel
    pub mod EXMINCH {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (3 bits: 0b111 << 0)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Extremes detector minimum value
    pub mod EXMIN {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (24 bits: 0xffffff << 8)
        pub const mask: u32 = 0xffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// DFSDM Extremes detector minimum register
pub mod DFSDM1_EXMIN {
    pub use super::DFSDM0_EXMIN::EXMIN;
    pub use super::DFSDM0_EXMIN::EXMINCH;
}

/// DFSDM Extremes detector minimum register
pub mod DFSDM2_EXMIN {
    pub use super::DFSDM0_EXMIN::EXMIN;
    pub use super::DFSDM0_EXMIN::EXMINCH;
}

/// DFSDM Extremes detector minimum register
pub mod DFSDM3_EXMIN {
    pub use super::DFSDM0_EXMIN::EXMIN;
    pub use super::DFSDM0_EXMIN::EXMINCH;
}

/// DFSDM conversion timer register
pub mod DFSDM0_CNVTIMR {

    /// 28-bit timer counting conversion time
    pub mod CNVCNT {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (28 bits: 0xfffffff << 4)
        pub const mask: u32 = 0xfffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// DFSDM conversion timer register
pub mod DFSDM1_CNVTIMR {
    pub use super::DFSDM0_CNVTIMR::CNVCNT;
}

/// DFSDM conversion timer register
pub mod DFSDM2_CNVTIMR {
    pub use super::DFSDM0_CNVTIMR::CNVCNT;
}

/// DFSDM conversion timer register
pub mod DFSDM3_CNVTIMR {
    pub use super::DFSDM0_CNVTIMR::CNVCNT;
}
pub struct RegisterBlock {
    /// DFSDM channel configuration 0 register 1
    pub DFSDM_CHCFG0R1: RWRegister<u32>,

    /// DFSDM channel configuration 1 register 1
    pub DFSDM_CHCFG1R1: RWRegister<u32>,

    /// DFSDM channel configuration 2 register 1
    pub DFSDM_CHCFG2R1: RWRegister<u32>,

    /// DFSDM channel configuration 3 register 1
    pub DFSDM_CHCFG3R1: RWRegister<u32>,

    /// DFSDM channel configuration 4 register 1
    pub DFSDM_CHCFG4R1: RWRegister<u32>,

    /// DFSDM channel configuration 5 register 1
    pub DFSDM_CHCFG5R1: RWRegister<u32>,

    /// DFSDM channel configuration 6 register 1
    pub DFSDM_CHCFG6R1: RWRegister<u32>,

    /// DFSDM channel configuration 7 register 1
    pub DFSDM_CHCFG7R1: RWRegister<u32>,

    /// DFSDM channel configuration 0 register 2
    pub DFSDM_CHCFG0R2: RWRegister<u32>,

    /// DFSDM channel configuration 1 register 2
    pub DFSDM_CHCFG1R2: RWRegister<u32>,

    /// DFSDM channel configuration 2 register 2
    pub DFSDM_CHCFG2R2: RWRegister<u32>,

    /// DFSDM channel configuration 3 register 2
    pub DFSDM_CHCFG3R2: RWRegister<u32>,

    /// DFSDM channel configuration 4 register 2
    pub DFSDM_CHCFG4R2: RWRegister<u32>,

    /// DFSDM channel configuration 5 register 2
    pub DFSDM_CHCFG5R2: RWRegister<u32>,

    /// DFSDM channel configuration 6 register 2
    pub DFSDM_CHCFG6R2: RWRegister<u32>,

    /// DFSDM channel configuration 7 register 2
    pub DFSDM_CHCFG7R2: RWRegister<u32>,

    /// DFSDM analog watchdog and short-circuit detector register
    pub DFSDM_AWSCD0R: RWRegister<u32>,

    /// DFSDM analog watchdog and short-circuit detector register
    pub DFSDM_AWSCD1R: RWRegister<u32>,

    /// DFSDM analog watchdog and short-circuit detector register
    pub DFSDM_AWSCD2R: RWRegister<u32>,

    /// DFSDM analog watchdog and short-circuit detector register
    pub DFSDM_AWSCD3R: RWRegister<u32>,

    /// DFSDM analog watchdog and short-circuit detector register
    pub DFSDM_AWSCD4R: RWRegister<u32>,

    /// DFSDM analog watchdog and short-circuit detector register
    pub DFSDM_AWSCD5R: RWRegister<u32>,

    /// DFSDM analog watchdog and short-circuit detector register
    pub DFSDM_AWSCD6R: RWRegister<u32>,

    /// DFSDM analog watchdog and short-circuit detector register
    pub DFSDM_AWSCD7R: RWRegister<u32>,

    /// DFSDM channel watchdog filter data register
    pub DFSDM_CHWDAT0R: RORegister<u32>,

    /// DFSDM channel watchdog filter data register
    pub DFSDM_CHWDAT1R: RORegister<u32>,

    /// DFSDM channel watchdog filter data register
    pub DFSDM_CHWDAT2R: RORegister<u32>,

    /// DFSDM channel watchdog filter data register
    pub DFSDM_CHWDAT3R: RORegister<u32>,

    /// DFSDM channel watchdog filter data register
    pub DFSDM_CHWDAT4R: RORegister<u32>,

    /// DFSDM channel watchdog filter data register
    pub DFSDM_CHWDAT5R: RORegister<u32>,

    /// DFSDM channel watchdog filter data register
    pub DFSDM_CHWDAT6R: RORegister<u32>,

    /// DFSDM channel watchdog filter data register
    pub DFSDM_CHWDAT7R: RORegister<u32>,

    /// DFSDM channel data input register
    pub DFSDM_CHDATIN0R: RWRegister<u32>,

    /// DFSDM channel data input register
    pub DFSDM_CHDATIN1R: RWRegister<u32>,

    /// DFSDM channel data input register
    pub DFSDM_CHDATIN2R: RWRegister<u32>,

    /// DFSDM channel data input register
    pub DFSDM_CHDATIN3R: RWRegister<u32>,

    /// DFSDM channel data input register
    pub DFSDM_CHDATIN4R: RWRegister<u32>,

    /// DFSDM channel data input register
    pub DFSDM_CHDATIN5R: RWRegister<u32>,

    /// DFSDM channel data input register
    pub DFSDM_CHDATIN6R: RWRegister<u32>,

    /// DFSDM channel data input register
    pub DFSDM_CHDATIN7R: RWRegister<u32>,

    /// DFSDM control register 1
    pub DFSDM0_CR1: RWRegister<u32>,

    /// DFSDM control register 1
    pub DFSDM1_CR1: RWRegister<u32>,

    /// DFSDM control register 1
    pub DFSDM2_CR1: RWRegister<u32>,

    /// DFSDM control register 1
    pub DFSDM3_CR1: RWRegister<u32>,

    /// DFSDM control register 2
    pub DFSDM0_CR2: RWRegister<u32>,

    /// DFSDM control register 2
    pub DFSDM1_CR2: RWRegister<u32>,

    /// DFSDM control register 2
    pub DFSDM2_CR2: RWRegister<u32>,

    /// DFSDM control register 2
    pub DFSDM3_CR2: RWRegister<u32>,

    /// DFSDM interrupt and status register
    pub DFSDM0_ISR: RORegister<u32>,

    /// DFSDM interrupt and status register
    pub DFSDM1_ISR: RORegister<u32>,

    /// DFSDM interrupt and status register
    pub DFSDM2_ISR: RORegister<u32>,

    /// DFSDM interrupt and status register
    pub DFSDM3_ISR: RORegister<u32>,

    /// DFSDM interrupt flag clear register
    pub DFSDM0_ICR: RWRegister<u32>,

    /// DFSDM interrupt flag clear register
    pub DFSDM1_ICR: RWRegister<u32>,

    /// DFSDM interrupt flag clear register
    pub DFSDM2_ICR: RWRegister<u32>,

    /// DFSDM interrupt flag clear register
    pub DFSDM3_ICR: RWRegister<u32>,

    /// DFSDM injected channel group selection register
    pub DFSDM0_JCHGR: RWRegister<u32>,

    /// DFSDM injected channel group selection register
    pub DFSDM1_JCHGR: RWRegister<u32>,

    /// DFSDM injected channel group selection register
    pub DFSDM2_JCHGR: RWRegister<u32>,

    /// DFSDM injected channel group selection register
    pub DFSDM3_JCHGR: RWRegister<u32>,

    /// DFSDM filter control register
    pub DFSDM0_FCR: RWRegister<u32>,

    /// DFSDM filter control register
    pub DFSDM1_FCR: RWRegister<u32>,

    /// DFSDM filter control register
    pub DFSDM2_FCR: RWRegister<u32>,

    /// DFSDM filter control register
    pub DFSDM3_FCR: RWRegister<u32>,

    /// DFSDM data register for injected group
    pub DFSDM0_JDATAR: RORegister<u32>,

    /// DFSDM data register for injected group
    pub DFSDM1_JDATAR: RORegister<u32>,

    /// DFSDM data register for injected group
    pub DFSDM2_JDATAR: RORegister<u32>,

    /// DFSDM data register for injected group
    pub DFSDM3_JDATAR: RORegister<u32>,

    /// DFSDM data register for the regular channel
    pub DFSDM0_RDATAR: RORegister<u32>,

    /// DFSDM data register for the regular channel
    pub DFSDM1_RDATAR: RORegister<u32>,

    /// DFSDM data register for the regular channel
    pub DFSDM2_RDATAR: RORegister<u32>,

    /// DFSDM data register for the regular channel
    pub DFSDM3_RDATAR: RORegister<u32>,

    /// DFSDM analog watchdog high threshold register
    pub DFSDM0_AWHTR: RWRegister<u32>,

    /// DFSDM analog watchdog high threshold register
    pub DFSDM1_AWHTR: RWRegister<u32>,

    /// DFSDM analog watchdog high threshold register
    pub DFSDM2_AWHTR: RWRegister<u32>,

    /// DFSDM analog watchdog high threshold register
    pub DFSDM3_AWHTR: RWRegister<u32>,

    /// DFSDM analog watchdog low threshold register
    pub DFSDM0_AWLTR: RWRegister<u32>,

    /// DFSDM analog watchdog low threshold register
    pub DFSDM1_AWLTR: RWRegister<u32>,

    /// DFSDM analog watchdog low threshold register
    pub DFSDM2_AWLTR: RWRegister<u32>,

    /// DFSDM analog watchdog low threshold register
    pub DFSDM3_AWLTR: RWRegister<u32>,

    /// DFSDM analog watchdog status register
    pub DFSDM0_AWSR: RORegister<u32>,

    /// DFSDM analog watchdog status register
    pub DFSDM1_AWSR: RORegister<u32>,

    /// DFSDM analog watchdog status register
    pub DFSDM2_AWSR: RORegister<u32>,

    /// DFSDM analog watchdog status register
    pub DFSDM3_AWSR: RORegister<u32>,

    /// DFSDM analog watchdog clear flag register
    pub DFSDM0_AWCFR: RWRegister<u32>,

    /// DFSDM analog watchdog clear flag register
    pub DFSDM1_AWCFR: RWRegister<u32>,

    /// DFSDM analog watchdog clear flag register
    pub DFSDM2_AWCFR: RWRegister<u32>,

    /// DFSDM analog watchdog clear flag register
    pub DFSDM3_AWCFR: RWRegister<u32>,

    /// DFSDM Extremes detector maximum register
    pub DFSDM0_EXMAX: RORegister<u32>,

    /// DFSDM Extremes detector maximum register
    pub DFSDM1_EXMAX: RORegister<u32>,

    /// DFSDM Extremes detector maximum register
    pub DFSDM2_EXMAX: RORegister<u32>,

    /// DFSDM Extremes detector maximum register
    pub DFSDM3_EXMAX: RORegister<u32>,

    /// DFSDM Extremes detector minimum register
    pub DFSDM0_EXMIN: RORegister<u32>,

    /// DFSDM Extremes detector minimum register
    pub DFSDM1_EXMIN: RORegister<u32>,

    /// DFSDM Extremes detector minimum register
    pub DFSDM2_EXMIN: RORegister<u32>,

    /// DFSDM Extremes detector minimum register
    pub DFSDM3_EXMIN: RORegister<u32>,

    /// DFSDM conversion timer register
    pub DFSDM0_CNVTIMR: RORegister<u32>,

    /// DFSDM conversion timer register
    pub DFSDM1_CNVTIMR: RORegister<u32>,

    /// DFSDM conversion timer register
    pub DFSDM2_CNVTIMR: RORegister<u32>,

    /// DFSDM conversion timer register
    pub DFSDM3_CNVTIMR: RORegister<u32>,
}
pub struct ResetValues {
    pub DFSDM_CHCFG0R1: u32,
    pub DFSDM_CHCFG1R1: u32,
    pub DFSDM_CHCFG2R1: u32,
    pub DFSDM_CHCFG3R1: u32,
    pub DFSDM_CHCFG4R1: u32,
    pub DFSDM_CHCFG5R1: u32,
    pub DFSDM_CHCFG6R1: u32,
    pub DFSDM_CHCFG7R1: u32,
    pub DFSDM_CHCFG0R2: u32,
    pub DFSDM_CHCFG1R2: u32,
    pub DFSDM_CHCFG2R2: u32,
    pub DFSDM_CHCFG3R2: u32,
    pub DFSDM_CHCFG4R2: u32,
    pub DFSDM_CHCFG5R2: u32,
    pub DFSDM_CHCFG6R2: u32,
    pub DFSDM_CHCFG7R2: u32,
    pub DFSDM_AWSCD0R: u32,
    pub DFSDM_AWSCD1R: u32,
    pub DFSDM_AWSCD2R: u32,
    pub DFSDM_AWSCD3R: u32,
    pub DFSDM_AWSCD4R: u32,
    pub DFSDM_AWSCD5R: u32,
    pub DFSDM_AWSCD6R: u32,
    pub DFSDM_AWSCD7R: u32,
    pub DFSDM_CHWDAT0R: u32,
    pub DFSDM_CHWDAT1R: u32,
    pub DFSDM_CHWDAT2R: u32,
    pub DFSDM_CHWDAT3R: u32,
    pub DFSDM_CHWDAT4R: u32,
    pub DFSDM_CHWDAT5R: u32,
    pub DFSDM_CHWDAT6R: u32,
    pub DFSDM_CHWDAT7R: u32,
    pub DFSDM_CHDATIN0R: u32,
    pub DFSDM_CHDATIN1R: u32,
    pub DFSDM_CHDATIN2R: u32,
    pub DFSDM_CHDATIN3R: u32,
    pub DFSDM_CHDATIN4R: u32,
    pub DFSDM_CHDATIN5R: u32,
    pub DFSDM_CHDATIN6R: u32,
    pub DFSDM_CHDATIN7R: u32,
    pub DFSDM0_CR1: u32,
    pub DFSDM1_CR1: u32,
    pub DFSDM2_CR1: u32,
    pub DFSDM3_CR1: u32,
    pub DFSDM0_CR2: u32,
    pub DFSDM1_CR2: u32,
    pub DFSDM2_CR2: u32,
    pub DFSDM3_CR2: u32,
    pub DFSDM0_ISR: u32,
    pub DFSDM1_ISR: u32,
    pub DFSDM2_ISR: u32,
    pub DFSDM3_ISR: u32,
    pub DFSDM0_ICR: u32,
    pub DFSDM1_ICR: u32,
    pub DFSDM2_ICR: u32,
    pub DFSDM3_ICR: u32,
    pub DFSDM0_JCHGR: u32,
    pub DFSDM1_JCHGR: u32,
    pub DFSDM2_JCHGR: u32,
    pub DFSDM3_JCHGR: u32,
    pub DFSDM0_FCR: u32,
    pub DFSDM1_FCR: u32,
    pub DFSDM2_FCR: u32,
    pub DFSDM3_FCR: u32,
    pub DFSDM0_JDATAR: u32,
    pub DFSDM1_JDATAR: u32,
    pub DFSDM2_JDATAR: u32,
    pub DFSDM3_JDATAR: u32,
    pub DFSDM0_RDATAR: u32,
    pub DFSDM1_RDATAR: u32,
    pub DFSDM2_RDATAR: u32,
    pub DFSDM3_RDATAR: u32,
    pub DFSDM0_AWHTR: u32,
    pub DFSDM1_AWHTR: u32,
    pub DFSDM2_AWHTR: u32,
    pub DFSDM3_AWHTR: u32,
    pub DFSDM0_AWLTR: u32,
    pub DFSDM1_AWLTR: u32,
    pub DFSDM2_AWLTR: u32,
    pub DFSDM3_AWLTR: u32,
    pub DFSDM0_AWSR: u32,
    pub DFSDM1_AWSR: u32,
    pub DFSDM2_AWSR: u32,
    pub DFSDM3_AWSR: u32,
    pub DFSDM0_AWCFR: u32,
    pub DFSDM1_AWCFR: u32,
    pub DFSDM2_AWCFR: u32,
    pub DFSDM3_AWCFR: u32,
    pub DFSDM0_EXMAX: u32,
    pub DFSDM1_EXMAX: u32,
    pub DFSDM2_EXMAX: u32,
    pub DFSDM3_EXMAX: u32,
    pub DFSDM0_EXMIN: u32,
    pub DFSDM1_EXMIN: u32,
    pub DFSDM2_EXMIN: u32,
    pub DFSDM3_EXMIN: u32,
    pub DFSDM0_CNVTIMR: u32,
    pub DFSDM1_CNVTIMR: u32,
    pub DFSDM2_CNVTIMR: u32,
    pub DFSDM3_CNVTIMR: u32,
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
#[cfg(feature = "rtfm")]
unsafe impl Send for Instance {}
