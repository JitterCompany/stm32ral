#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Liquid crystal display controller
//!
//! Used by: stm32l4r5, stm32l4r9

use crate::{RORegister, RWRegister, UnsafeRWRegister, WORegister};
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// LTDC Synchronization Size Configuration Register
pub mod SSCR {

    /// Vertical Synchronization Height (in units of horizontal scan line)
    pub mod VSH {
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

    /// Horizontal Synchronization Width (in units of pixel clock period)
    pub mod HSW {
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

/// LTDC Back Porch Configuration Register
pub mod BPCR {

    /// Accumulated Vertical back porch (in units of horizontal scan line)
    pub mod AVBP {
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

    /// Accumulated Horizontal back porch (in units of pixel clock period)
    pub mod AHBP {
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

/// LTDC Active Width Configuration Register
pub mod AWCR {

    /// Accumulated Active Height (in units of horizontal scan line)
    pub mod AAH {
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

    /// Accumulated Active Width (in units of pixel clock period)
    pub mod AAW {
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

/// LTDC Total Width Configuration Register
pub mod TWCR {

    /// Total Height (in units of horizontal scan line)
    pub mod TOTALH {
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

    /// Total Width (in units of pixel clock period)
    pub mod TOTALW {
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

/// LTDC Global Control Register
pub mod GCR {

    /// LCD-TFT controller enable bit
    pub mod LTDCEN {
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

    /// Dither Blue Width
    pub mod DBW {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (3 bits: 0b111 << 4)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Dither Green Width
    pub mod DGW {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (3 bits: 0b111 << 8)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Dither Red Width
    pub mod DRW {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (3 bits: 0b111 << 12)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Dither Enable
    pub mod DEN {
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

    /// Pixel Clock Polarity
    pub mod PCPOL {
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

    /// Not Data Enable Polarity
    pub mod DEPOL {
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

    /// Vertical Synchronization Polarity
    pub mod VSPOL {
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

    /// Horizontal Synchronization Polarity
    pub mod HSPOL {
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

/// LTDC Shadow Reload Configuration Register
pub mod SRCR {

    /// Immediate Reload
    pub mod IMR {
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

    /// Vertical Blanking Reload
    pub mod VBR {
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
}

/// LTDC Background Color Configuration Register
pub mod BCCR {

    /// Background Color Blue value
    pub mod BCBLUE {
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

    /// Background Color Green value
    pub mod BCGREEN {
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

    /// Background Color Red value
    pub mod BCRED {
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

/// LTDC Interrupt Enable Register
pub mod IER {

    /// Line Interrupt Enable
    pub mod LIE {
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

    /// FIFO Underrun Interrupt Enable
    pub mod FUIE {
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

    /// Transfer Error Interrupt Enable
    pub mod TERRIE {
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

    /// Register Reload interrupt enable
    pub mod RRIE {
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
}

/// LTDC Interrupt Status Register
pub mod ISR {

    /// Line Interrupt flag
    pub mod LIF {
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

    /// FIFO Underrun Interrupt flag
    pub mod FUIF {
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

    /// Transfer Error interrupt flag
    pub mod TERRIF {
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

    /// Register Reload Interrupt Flag
    pub mod RRIF {
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
}

/// LTDC Interrupt Clear Register
pub mod ICR {

    /// Clears the Line Interrupt Flag
    pub mod CLIF {
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

    /// Clears the FIFO Underrun Interrupt flag
    pub mod CFUIF {
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

    /// Clears the Transfer Error Interrupt Flag
    pub mod CTERRIF {
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

    /// Clears Register Reload Interrupt Flag
    pub mod CRRIF {
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
}

/// LTDC Line Interrupt Position Configuration Register
pub mod LIPCR {

    /// Line Interrupt Position
    pub mod LIPOS {
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
}

/// LTDC Current Position Status Register
pub mod CPSR {

    /// Current Y Position
    pub mod CYPOS {
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

    /// Current X Position
    pub mod CXPOS {
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

/// LTDC Current Display Status Register
pub mod CDSR {

    /// Vertical Data Enable display Status
    pub mod VDES {
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

    /// Horizontal Data Enable display Status
    pub mod HDES {
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

    /// Vertical Synchronization display Status
    pub mod VSYNCS {
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

    /// Horizontal Synchronization display Status
    pub mod HSYNCS {
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
}

/// LTDC Layer Control Register
pub mod L1CR {

    /// Layer Enable
    pub mod LEN {
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

    /// Color Keying Enable
    pub mod COLKEN {
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

    /// Color Look-Up Table Enable
    pub mod CLUTEN {
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
}

/// LTDC Layer Control Register
pub mod L2CR {
    pub use super::L1CR::CLUTEN;
    pub use super::L1CR::COLKEN;
    pub use super::L1CR::LEN;
}

/// LTDC Layer Window Horizontal Position Configuration Register
pub mod L1WHPCR {

    /// Window Horizontal Start Position
    pub mod WHSTPOS {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (12 bits: 0xfff << 0)
        pub const mask: u32 = 0xfff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Window Horizontal Stop Position
    pub mod WHSPPOS {
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

/// LTDC Layerx Window Horizontal Position Configuration Register
pub mod L2WHPCR {
    pub use super::L1WHPCR::WHSPPOS;
    pub use super::L1WHPCR::WHSTPOS;
}

/// LTDC Layer Window Vertical Position Configuration Register
pub mod L1WVPCR {

    /// Window Vertical Start Position
    pub mod WVSTPOS {
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

    /// Window Vertical Stop Position
    pub mod WVSPPOS {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (11 bits: 0x7ff << 16)
        pub const mask: u32 = 0x7ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// LTDC Layer Window Vertical Position Configuration Register
pub mod L2WVPCR {
    pub use super::L1WVPCR::WVSPPOS;
    pub use super::L1WVPCR::WVSTPOS;
}

/// LTDC Layer Color Keying Configuration Register
pub mod L1CKCR {

    /// Color Key Blue value
    pub mod CKBLUE {
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

    /// Color Key Green value
    pub mod CKGREEN {
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

    /// Color Key Red value
    pub mod CKRED {
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

/// LTDC Layer Color Keying Configuration Register
pub mod L2CKCR {
    pub use super::L1CKCR::CKBLUE;
    pub use super::L1CKCR::CKGREEN;
    pub use super::L1CKCR::CKRED;
}

/// LTDC Layer Pixel Format Configuration Register
pub mod L1PFCR {

    /// Pixel Format
    pub mod PF {
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
}

/// LTDC Layer Pixel Format Configuration Register
pub mod L2PFCR {
    pub use super::L1PFCR::PF;
}

/// LTDC Layer Constant Alpha Configuration Register
pub mod L1CACR {

    /// Constant Alpha
    pub mod CONSTA {
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

/// LTDC Layer Constant Alpha Configuration Register
pub mod L2CACR {
    pub use super::L1CACR::CONSTA;
}

/// LTDC Layer Default Color Configuration Register
pub mod L1DCCR {

    /// Default Color Blue
    pub mod DCBLUE {
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

    /// Default Color Green
    pub mod DCGREEN {
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

    /// Default Color Red
    pub mod DCRED {
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

    /// Default Color Alpha
    pub mod DCALPHA {
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

/// LTDC Layer Default Color Configuration Register
pub mod L2DCCR {
    pub use super::L1DCCR::DCALPHA;
    pub use super::L1DCCR::DCBLUE;
    pub use super::L1DCCR::DCGREEN;
    pub use super::L1DCCR::DCRED;
}

/// LTDC Layer Blending Factors Configuration Register
pub mod L1BFCR {

    /// Blending Factor 2
    pub mod BF2 {
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

    /// Blending Factor 1
    pub mod BF1 {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (3 bits: 0b111 << 8)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// LTDC Layer Blending Factors Configuration Register
pub mod L2BFCR {
    pub use super::L1BFCR::BF1;
    pub use super::L1BFCR::BF2;
}

/// LTDC Layer Color Frame Buffer Address Register
pub mod L1CFBAR {

    /// Color Frame Buffer Start Address
    pub mod CFBADD {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// LTDC Layer Color Frame Buffer Address Register
pub mod L2CFBAR {
    pub use super::L1CFBAR::CFBADD;
}

/// LTDC Layer Color Frame Buffer Length Register
pub mod L1CFBLR {

    /// Color Frame Buffer Line Length
    pub mod CFBLL {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (13 bits: 0x1fff << 0)
        pub const mask: u32 = 0x1fff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Color Frame Buffer Pitch in bytes
    pub mod CFBP {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (13 bits: 0x1fff << 16)
        pub const mask: u32 = 0x1fff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// LTDC Layer Color Frame Buffer Length Register
pub mod L2CFBLR {
    pub use super::L1CFBLR::CFBLL;
    pub use super::L1CFBLR::CFBP;
}

/// LTDC Layer ColorFrame Buffer Line Number Register
pub mod L1CFBLNR {

    /// Frame Buffer Line Number
    pub mod CFBLNBR {
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
}

/// LTDC Layer ColorFrame Buffer Line Number Register
pub mod L2CFBLNR {
    pub use super::L1CFBLNR::CFBLNBR;
}

/// LTDC Layerx CLUT Write Register
pub mod L1CLUTWR {

    /// Blue value
    pub mod BLUE {
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

    /// Green value
    pub mod GREEN {
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

    /// Red value
    pub mod RED {
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

    /// CLUT Address
    pub mod CLUTADD {
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

/// LTDC Layerx CLUT Write Register
pub mod L2CLUTWR {
    pub use super::L1CLUTWR::BLUE;
    pub use super::L1CLUTWR::CLUTADD;
    pub use super::L1CLUTWR::GREEN;
    pub use super::L1CLUTWR::RED;
}
#[repr(C)]
pub struct RegisterBlock {
    _reserved1: [u8; 8],

    /// LTDC Synchronization Size Configuration Register
    pub SSCR: RWRegister<u32>,

    /// LTDC Back Porch Configuration Register
    pub BPCR: RWRegister<u32>,

    /// LTDC Active Width Configuration Register
    pub AWCR: RWRegister<u32>,

    /// LTDC Total Width Configuration Register
    pub TWCR: RWRegister<u32>,

    /// LTDC Global Control Register
    pub GCR: RWRegister<u32>,

    _reserved2: [u8; 8],

    /// LTDC Shadow Reload Configuration Register
    pub SRCR: RWRegister<u32>,

    _reserved3: [u8; 4],

    /// LTDC Background Color Configuration Register
    pub BCCR: RWRegister<u32>,

    _reserved4: [u8; 4],

    /// LTDC Interrupt Enable Register
    pub IER: RWRegister<u32>,

    /// LTDC Interrupt Status Register
    pub ISR: RORegister<u32>,

    /// LTDC Interrupt Clear Register
    pub ICR: WORegister<u32>,

    /// LTDC Line Interrupt Position Configuration Register
    pub LIPCR: RWRegister<u32>,

    /// LTDC Current Position Status Register
    pub CPSR: RORegister<u32>,

    /// LTDC Current Display Status Register
    pub CDSR: RORegister<u32>,

    _reserved5: [u8; 56],

    /// LTDC Layer Control Register
    pub L1CR: RWRegister<u32>,

    /// LTDC Layer Window Horizontal Position Configuration Register
    pub L1WHPCR: RWRegister<u32>,

    /// LTDC Layer Window Vertical Position Configuration Register
    pub L1WVPCR: RWRegister<u32>,

    /// LTDC Layer Color Keying Configuration Register
    pub L1CKCR: RWRegister<u32>,

    /// LTDC Layer Pixel Format Configuration Register
    pub L1PFCR: RWRegister<u32>,

    /// LTDC Layer Constant Alpha Configuration Register
    pub L1CACR: RWRegister<u32>,

    /// LTDC Layer Default Color Configuration Register
    pub L1DCCR: RWRegister<u32>,

    /// LTDC Layer Blending Factors Configuration Register
    pub L1BFCR: RWRegister<u32>,

    _reserved6: [u8; 8],

    /// LTDC Layer Color Frame Buffer Address Register
    pub L1CFBAR: UnsafeRWRegister<u32>,

    /// LTDC Layer Color Frame Buffer Length Register
    pub L1CFBLR: RWRegister<u32>,

    /// LTDC Layer ColorFrame Buffer Line Number Register
    pub L1CFBLNR: RWRegister<u32>,

    _reserved7: [u8; 12],

    /// LTDC Layerx CLUT Write Register
    pub L1CLUTWR: WORegister<u32>,

    _reserved8: [u8; 60],

    /// LTDC Layer Control Register
    pub L2CR: RWRegister<u32>,

    /// LTDC Layerx Window Horizontal Position Configuration Register
    pub L2WHPCR: RWRegister<u32>,

    /// LTDC Layer Window Vertical Position Configuration Register
    pub L2WVPCR: RWRegister<u32>,

    /// LTDC Layer Color Keying Configuration Register
    pub L2CKCR: RWRegister<u32>,

    /// LTDC Layer Pixel Format Configuration Register
    pub L2PFCR: RWRegister<u32>,

    /// LTDC Layer Constant Alpha Configuration Register
    pub L2CACR: RWRegister<u32>,

    /// LTDC Layer Default Color Configuration Register
    pub L2DCCR: RWRegister<u32>,

    _reserved9: [u8; 4],

    /// LTDC Layer Blending Factors Configuration Register
    pub L2BFCR: RWRegister<u32>,

    _reserved10: [u8; 4],

    /// LTDC Layer Color Frame Buffer Address Register
    pub L2CFBAR: UnsafeRWRegister<u32>,

    /// LTDC Layer Color Frame Buffer Length Register
    pub L2CFBLR: RWRegister<u32>,

    /// LTDC Layer ColorFrame Buffer Line Number Register
    pub L2CFBLNR: RWRegister<u32>,

    _reserved11: [u8; 12],

    /// LTDC Layerx CLUT Write Register
    pub L2CLUTWR: WORegister<u32>,
}
pub struct ResetValues {
    pub SSCR: u32,
    pub BPCR: u32,
    pub AWCR: u32,
    pub TWCR: u32,
    pub GCR: u32,
    pub SRCR: u32,
    pub BCCR: u32,
    pub IER: u32,
    pub ISR: u32,
    pub ICR: u32,
    pub LIPCR: u32,
    pub CPSR: u32,
    pub CDSR: u32,
    pub L1CR: u32,
    pub L1WHPCR: u32,
    pub L1WVPCR: u32,
    pub L1CKCR: u32,
    pub L1PFCR: u32,
    pub L1CACR: u32,
    pub L1DCCR: u32,
    pub L1BFCR: u32,
    pub L1CFBAR: u32,
    pub L1CFBLR: u32,
    pub L1CFBLNR: u32,
    pub L1CLUTWR: u32,
    pub L2CR: u32,
    pub L2WHPCR: u32,
    pub L2WVPCR: u32,
    pub L2CKCR: u32,
    pub L2PFCR: u32,
    pub L2CACR: u32,
    pub L2DCCR: u32,
    pub L2BFCR: u32,
    pub L2CFBAR: u32,
    pub L2CFBLR: u32,
    pub L2CFBLNR: u32,
    pub L2CLUTWR: u32,
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
