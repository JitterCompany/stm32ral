#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! LCD-TFT display controller

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
        /// Read-write values
        pub mod RW {

            /// 0b0: LCD-TFT controller disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: LCD-TFT controller enabled
            pub const Enabled: u32 = 0b1;
        }
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
        /// Read-write values
        pub mod RW {

            /// 0b0: Dither disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Dither enabled
            pub const Enabled: u32 = 0b1;
        }
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
        /// Read-write values
        pub mod RW {

            /// 0b0: Pixel clock on rising edge
            pub const RisingEdge: u32 = 0b0;

            /// 0b1: Pixel clock on falling edge
            pub const FallingEdge: u32 = 0b1;
        }
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
        /// Read-write values
        pub mod RW {

            /// 0b0: Data enable polarity is active low
            pub const ActiveLow: u32 = 0b0;

            /// 0b1: Data enable polarity is active high
            pub const ActiveHigh: u32 = 0b1;
        }
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
        /// Read-write values
        pub mod RW {

            /// 0b0: Vertical synchronization polarity is active low
            pub const ActiveLow: u32 = 0b0;

            /// 0b1: Vertical synchronization polarity is active high
            pub const ActiveHigh: u32 = 0b1;
        }
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
        /// Read-write values
        pub mod RW {

            /// 0b0: Horizontal synchronization polarity is active low
            pub const ActiveLow: u32 = 0b0;

            /// 0b1: Horizontal synchronization polarity is active high
            pub const ActiveHigh: u32 = 0b1;
        }
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
        /// Read-write values
        pub mod RW {

            /// 0b0: This bit is set by software and cleared only by hardware after reload (it cannot be cleared through register write once it is set)
            pub const NoEffect: u32 = 0b0;

            /// 0b1: The shadow registers are reloaded immediately. This bit is set by software and cleared only by hardware after reload
            pub const Reload: u32 = 0b1;
        }
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
        /// Read-write values
        pub mod RW {

            /// 0b0: This bit is set by software and cleared only by hardware after reload (it cannot be cleared through register write once it is set)
            pub const NoEffect: u32 = 0b0;

            /// 0b1: The shadow registers are reloaded during the vertical blanking period (at the beginning of the first line after the active display area).
            pub const Reload: u32 = 0b1;
        }
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
        /// Read-write values
        pub mod RW {

            /// 0b0: Line interrupt disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Line interrupt enabled
            pub const Enabled: u32 = 0b1;
        }
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
        /// Read-write values
        pub mod RW {

            /// 0b0: FIFO underrun interrupt disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: FIFO underrun interrupt enabled
            pub const Enabled: u32 = 0b1;
        }
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
        /// Read-write values
        pub mod RW {

            /// 0b0: Transfer error interrupt disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Transfer error interrupt enabled
            pub const Enabled: u32 = 0b1;
        }
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
        /// Read-write values
        pub mod RW {

            /// 0b0: Register reload interrupt disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Register reload interrupt enabled
            pub const Enabled: u32 = 0b1;
        }
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
        /// Read-only values
        pub mod R {

            /// 0b0: Programmed line not reached
            pub const NotReached: u32 = 0b0;

            /// 0b1: Line interrupt generated when a programmed line is reached
            pub const Reached: u32 = 0b1;
        }
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
        /// Read-only values
        pub mod R {

            /// 0b0: No FIFO underrun
            pub const NoUnderrun: u32 = 0b0;

            /// 0b1: FIFO underrun interrupt generated, if one of the layer FIFOs is empty and pixel data is read from the FIFO
            pub const Underrun: u32 = 0b1;
        }
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
        /// Read-only values
        pub mod R {

            /// 0b0: No transfer error
            pub const NoError: u32 = 0b0;

            /// 0b1: Transfer error interrupt generated when a bus error occurs
            pub const Error: u32 = 0b1;
        }
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
        /// Read-only values
        pub mod R {

            /// 0b0: No register reload
            pub const NoReload: u32 = 0b0;

            /// 0b1: Register reload interrupt generated when a vertical blanking reload occurs (and the first line after the active area is reached)
            pub const Reload: u32 = 0b1;
        }
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
        /// Write-only values
        pub mod W {

            /// 0b1: Clears the LIF flag in the ISR register
            pub const Clear: u32 = 0b1;
        }
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
        /// Write-only values
        pub mod W {

            /// 0b1: Clears the FUIF flag in the ISR register
            pub const Clear: u32 = 0b1;
        }
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
        /// Write-only values
        pub mod W {

            /// 0b1: Clears the TERRIF flag in the ISR register
            pub const Clear: u32 = 0b1;
        }
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
        /// Write-only values
        pub mod W {

            /// 0b1: Clears the RRIF flag in the ISR register
            pub const Clear: u32 = 0b1;
        }
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
        /// Read-only values
        pub mod R {

            /// 0b0: Currently not in vertical Data Enable phase
            pub const NotActive: u32 = 0b0;

            /// 0b1: Currently in vertical Data Enable phase
            pub const Active: u32 = 0b1;
        }
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
        /// Read-only values
        pub mod R {

            /// 0b0: Currently not in horizontal Data Enable phase
            pub const NotActive: u32 = 0b0;

            /// 0b1: Currently in horizontal Data Enable phase
            pub const Active: u32 = 0b1;
        }
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
        /// Read-only values
        pub mod R {

            /// 0b0: Currently not in VSYNC phase
            pub const NotActive: u32 = 0b0;

            /// 0b1: Currently in VSYNC phase
            pub const Active: u32 = 0b1;
        }
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
        /// Read-only values
        pub mod R {

            /// 0b0: Currently not in HSYNC phase
            pub const NotActive: u32 = 0b0;

            /// 0b1: Currently in HSYNC phase
            pub const Active: u32 = 0b1;
        }
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
        /// Read-write values
        pub mod RW {

            /// 0b0: Layer disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Layer enabled
            pub const Enabled: u32 = 0b1;
        }
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
        /// Read-write values
        pub mod RW {

            /// 0b0: Color keying disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Color keying enabled
            pub const Enabled: u32 = 0b1;
        }
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
        /// Read-write values
        pub mod RW {

            /// 0b0: Color look-up table disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Color look-up table enabled
            pub const Enabled: u32 = 0b1;
        }
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
        /// Read-write values
        pub mod RW {

            /// 0b000: ARGB8888
            pub const ARGB8888: u32 = 0b000;

            /// 0b001: RGB888
            pub const RGB888: u32 = 0b001;

            /// 0b010: RGB565
            pub const RGB565: u32 = 0b010;

            /// 0b011: ARGB1555
            pub const ARGB1555: u32 = 0b011;

            /// 0b100: ARGB4444
            pub const ARGB4444: u32 = 0b100;

            /// 0b101: L8 (8-bit luminance)
            pub const L8: u32 = 0b101;

            /// 0b110: AL44 (4-bit alpha, 4-bit luminance)
            pub const AL44: u32 = 0b110;

            /// 0b111: AL88 (8-bit alpha, 8-bit luminance)
            pub const AL88: u32 = 0b111;
        }
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
        /// Read-write values
        pub mod RW {

            /// 0b101: BF2 = 1 - constant alpha
            pub const Constant: u32 = 0b101;

            /// 0b111: BF2 = 1 - pixel alpha * constant alpha
            pub const Pixel: u32 = 0b111;
        }
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
        /// Read-write values
        pub mod RW {

            /// 0b100: BF1 = constant alpha
            pub const Constant: u32 = 0b100;

            /// 0b110: BF1 = pixel alpha * constant alpha
            pub const Pixel: u32 = 0b110;
        }
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

/// Access functions for the LTDC peripheral instance
pub mod LTDC {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x40016800,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in LTDC
    pub const reset: ResetValues = ResetValues {
        SSCR: 0x00000000,
        BPCR: 0x00000000,
        AWCR: 0x00000000,
        TWCR: 0x00000000,
        GCR: 0x00002220,
        SRCR: 0x00000000,
        BCCR: 0x00000000,
        IER: 0x00000000,
        ISR: 0x00000000,
        ICR: 0x00000000,
        LIPCR: 0x00000000,
        CPSR: 0x00000000,
        CDSR: 0x0000000F,
        L1CR: 0x00000000,
        L2CR: 0x00000000,
        L1WHPCR: 0x00000000,
        L2WHPCR: 0x00000000,
        L1WVPCR: 0x00000000,
        L2WVPCR: 0x00000000,
        L1CKCR: 0x00000000,
        L2CKCR: 0x00000000,
        L1PFCR: 0x00000000,
        L2PFCR: 0x00000000,
        L1CACR: 0x00000000,
        L2CACR: 0x00000000,
        L1DCCR: 0x00000000,
        L2DCCR: 0x00000000,
        L1BFCR: 0x00000000,
        L2BFCR: 0x00000000,
        L1CFBAR: 0x00000000,
        L2CFBAR: 0x00000000,
        L1CFBLR: 0x00000000,
        L2CFBLR: 0x00000000,
        L1CFBLNR: 0x00000000,
        L2CFBLNR: 0x00000000,
        L1CLUTWR: 0x00000000,
        L2CLUTWR: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut LTDC_TAKEN: bool = false;

    /// Safe access to LTDC
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
            if LTDC_TAKEN {
                None
            } else {
                LTDC_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to LTDC
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if LTDC_TAKEN && inst.addr == INSTANCE.addr {
                LTDC_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal LTDC
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        LTDC_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to LTDC
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const LTDC: *const RegisterBlock = 0x40016800 as *const _;
