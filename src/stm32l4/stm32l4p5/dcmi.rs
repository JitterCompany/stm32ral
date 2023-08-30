#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Digital camera interface

use crate::{RORegister, RWRegister, WORegister};
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// control register 1
pub mod CR {

    /// DCMI enable
    pub mod ENABLE {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: DCMI disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: DCMI enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Extended data mode
    pub mod EDM {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (2 bits: 0b11 << 10)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Interface captures 8-bit data on every pixel clock
            pub const BitWidth8: u32 = 0b00;

            /// 0b01: Interface captures 10-bit data on every pixel clock
            pub const BitWidth10: u32 = 0b01;

            /// 0b10: Interface captures 12-bit data on every pixel clock
            pub const BitWidth12: u32 = 0b10;

            /// 0b11: Interface captures 14-bit data on every pixel clock
            pub const BitWidth14: u32 = 0b11;
        }
    }

    /// Frame capture rate control
    pub mod FCRC {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (2 bits: 0b11 << 8)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: All frames are captured
            pub const All: u32 = 0b00;

            /// 0b01: Every alternate frame captured (50% bandwidth reduction)
            pub const Alternate: u32 = 0b01;

            /// 0b10: One frame out of four captured (75% bandwidth reduction)
            pub const OneOfFour: u32 = 0b10;
        }
    }

    /// Vertical synchronization polarity
    pub mod VSPOL {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: DCMI_VSYNC active low
            pub const ActiveLow: u32 = 0b0;

            /// 0b1: DCMI_VSYNC active high
            pub const ActiveHigh: u32 = 0b1;
        }
    }

    /// Horizontal synchronization polarity
    pub mod HSPOL {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: DCMI_HSYNC active low
            pub const ActiveLow: u32 = 0b0;

            /// 0b1: DCMI_HSYNC active high
            pub const ActiveHigh: u32 = 0b1;
        }
    }

    /// Pixel clock polarity
    pub mod PCKPOL {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Falling edge active
            pub const FallingEdge: u32 = 0b0;

            /// 0b1: Rising edge active
            pub const RisingEdge: u32 = 0b1;
        }
    }

    /// Embedded synchronization select
    pub mod ESS {
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

            /// 0b0: Hardware synchronization data capture (frame/line start/stop) is synchronized with the DCMI_HSYNC/DCMI_VSYNC signals
            pub const Hardware: u32 = 0b0;

            /// 0b1: Embedded synchronization data capture is synchronized with synchronization codes embedded in the data flow
            pub const Embedded: u32 = 0b1;
        }
    }

    /// JPEG format
    pub mod JPEG {
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

            /// 0b0: Uncompressed video format
            pub const Uncompressed: u32 = 0b0;

            /// 0b1: This bit is used for JPEG data transfers. The DCMI_HSYNC signal is used as data enable. The crop and embedded synchronization features (ESS bit) cannot be used in this mode
            pub const JPEG: u32 = 0b1;
        }
    }

    /// Crop feature
    pub mod CROP {
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

            /// 0b0: The full image is captured. In this case the total number of bytes in an image frame must be a multiple of four
            pub const Full: u32 = 0b0;

            /// 0b1: Only the data inside the window specified by the crop register is captured. If the size of the crop window exceeds the picture size, then only the picture size is captured
            pub const Cropped: u32 = 0b1;
        }
    }

    /// Capture mode
    pub mod CM {
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

            /// 0b0: Continuous grab mode - The received data are transferred into the destination memory through the DMA. The buffer location and mode (linear or circular buffer) is controlled through the system DMA
            pub const Continuous: u32 = 0b0;

            /// 0b1: Snapshot mode (single frame) - Once activated, the interface waits for the start of frame and then transfers a single frame through the DMA. At the end of the frame, the CAPTURE bit is automatically reset
            pub const Snapshot: u32 = 0b1;
        }
    }

    /// Capture enable
    pub mod CAPTURE {
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

            /// 0b0: Capture disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Capture enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Odd/Even Line Select (Line Select Start)
    pub mod OELS {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (1 bit: 1 << 20)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Interface captures first line after the frame start, second one being dropped
            pub const Odd: u32 = 0b0;

            /// 0b1: Interface captures second line from the frame start, first one being dropped
            pub const Even: u32 = 0b1;
        }
    }

    /// Line Select mode
    pub mod LSM {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (1 bit: 1 << 19)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Interface captures all received lines
            pub const All: u32 = 0b0;

            /// 0b1: Interface captures one line out of two
            pub const Half: u32 = 0b1;
        }
    }

    /// Odd/Even Byte Select (Byte Select Start)
    pub mod OEBS {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (1 bit: 1 << 18)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Interface captures first data (byte or double byte) from the frame/line start, second one being dropped
            pub const Odd: u32 = 0b0;

            /// 0b1: Interface captures second data (byte or double byte) from the frame/line start, first one being dropped
            pub const Even: u32 = 0b1;
        }
    }

    /// Byte Select mode
    pub mod BSM {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (2 bits: 0b11 << 16)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Interface captures all received data
            pub const All: u32 = 0b00;

            /// 0b01: Interface captures every other byte from the received data
            pub const EveryOther: u32 = 0b01;

            /// 0b10: Interface captures one byte out of four
            pub const Fourth: u32 = 0b10;

            /// 0b11: Interface captures two bytes out of four
            pub const TwoOfFour: u32 = 0b11;
        }
    }
}

/// status register
pub mod SR {

    /// FIFO not empty
    pub mod FNE {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: FIFO contains valid data
            pub const NotEmpty: u32 = 0b0;

            /// 0b1: FIFO empty
            pub const Empty: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// VSYNC
    pub mod VSYNC {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: Active frame
            pub const ActiveFrame: u32 = 0b0;

            /// 0b1: Synchronization between frames
            pub const BetweenFrames: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// HSYNC
    pub mod HSYNC {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: Active line
            pub const ActiveLine: u32 = 0b0;

            /// 0b1: Synchronization between lines
            pub const BetweenLines: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// raw interrupt status register
pub mod RIS {

    /// Line raw interrupt status
    pub mod LINE_RIS {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: Interrupt cleared
            pub const Cleared: u32 = 0b0;

            /// 0b1: Interrupt set
            pub const Set: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// VSYNC raw interrupt status
    pub mod VSYNC_RIS {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        pub use super::LINE_RIS::R;
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Synchronization error raw interrupt status
    pub mod ERR_RIS {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: No synchronization error detected
            pub const NoError: u32 = 0b0;

            /// 0b1: Embedded synchronization characters are not received in the correct order
            pub const SynchronizationError: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Overrun raw interrupt status
    pub mod OVR_RIS {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: No data buffer overrun occurred
            pub const NoOverrun: u32 = 0b0;

            /// 0b1: A data buffer overrun occurred and the data FIFO is corrupted. The bit is cleared by setting the OVR_ISC bit of the DCMI_ICR register
            pub const OverrunOccured: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Capture complete raw interrupt status
    pub mod FRAME_RIS {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: No new capture
            pub const NoNewCapture: u32 = 0b0;

            /// 0b1: A frame has been captured
            pub const FrameCaptured: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// interrupt enable register
pub mod IER {

    /// Line interrupt enable
    pub mod LINE_IE {
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

            /// 0b0: No interrupt generation when the line is received
            pub const Disabled: u32 = 0b0;

            /// 0b1: An Interrupt is generated when a line has been completely received
            pub const Enabled: u32 = 0b1;
        }
    }

    /// VSYNC interrupt enable
    pub mod VSYNC_IE {
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

            /// 0b0: No interrupt generation
            pub const Disabled: u32 = 0b0;

            /// 0b1: An interrupt is generated on each DCMI_VSYNC transition from the inactive to the active state
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Synchronization error interrupt enable
    pub mod ERR_IE {
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

            /// 0b0: No interrupt generation
            pub const Disabled: u32 = 0b0;

            /// 0b1: An interrupt is generated if the embedded synchronization codes are not received in the correct order
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Overrun interrupt enable
    pub mod OVR_IE {
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

            /// 0b0: No interrupt generation
            pub const Disabled: u32 = 0b0;

            /// 0b1: An interrupt is generated if the DMA was not able to transfer the last data before new data (32-bit) are received
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Capture complete interrupt enable
    pub mod FRAME_IE {
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

            /// 0b0: No interrupt generation
            pub const Disabled: u32 = 0b0;

            /// 0b1: An interrupt is generated at the end of each received frame/crop window (in crop mode)
            pub const Enabled: u32 = 0b1;
        }
    }
}

/// masked interrupt status register
pub mod MIS {

    /// Line masked interrupt status
    pub mod LINE_MIS {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: No interrupt generation when the line is received
            pub const Disabled: u32 = 0b0;

            /// 0b1: An Interrupt is generated when a line has been completely received and the LINE_IE bit is set in DCMI_IER
            pub const Enabled: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// VSYNC masked interrupt status
    pub mod VSYNC_MIS {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: No interrupt is generated on DCMI_VSYNC transitions
            pub const Disabled: u32 = 0b0;

            /// 0b1: An interrupt is generated on each DCMI_VSYNC transition from the inactive to the active state and the VSYNC_IE bit is set in DCMI_IER
            pub const Enabled: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Synchronization error masked interrupt status
    pub mod ERR_MIS {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: No interrupt is generated on a synchronization error
            pub const Disabled: u32 = 0b0;

            /// 0b1: An interrupt is generated if the embedded synchronization codes are not received in the correct order and the ERR_IE bit in DCMI_IER is set
            pub const Enabled: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Overrun masked interrupt status
    pub mod OVR_MIS {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: No interrupt is generated on overrun
            pub const Disabled: u32 = 0b0;

            /// 0b1: An interrupt is generated if the DMA was not able to transfer the last data before new data (32-bit) are received and the OVR_IE bit is set in DCMI_IER
            pub const Enabled: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Capture complete masked interrupt status
    pub mod FRAME_MIS {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: No interrupt is generated after a complete capture
            pub const Disabled: u32 = 0b0;

            /// 0b1: An interrupt is generated at the end of each received frame/crop window (in crop mode) and the FRAME_IE bit is set in DCMI_IER
            pub const Enabled: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// interrupt clear register
pub mod ICR {

    /// line interrupt status clear
    pub mod LINE_ISC {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values
        pub mod W {

            /// 0b1: Setting this bit clears the LINE_RIS flag in the DCMI_RIS register
            pub const Clear: u32 = 0b1;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Vertical synch interrupt status clear
    pub mod VSYNC_ISC {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values
        pub mod W {

            /// 0b1: Setting this bit clears the VSYNC_RIS flag in the DCMI_RIS register
            pub const Clear: u32 = 0b1;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Synchronization error interrupt status clear
    pub mod ERR_ISC {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values
        pub mod W {

            /// 0b1: Setting this bit clears the ERR_RIS flag in the DCMI_RIS register
            pub const Clear: u32 = 0b1;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Overrun interrupt status clear
    pub mod OVR_ISC {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values
        pub mod W {

            /// 0b1: Setting this bit clears the OVR_RIS flag in the DCMI_RIS register
            pub const Clear: u32 = 0b1;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Capture complete interrupt status clear
    pub mod FRAME_ISC {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values
        pub mod W {

            /// 0b1: Setting this bit clears the FRAME_RIS flag in the DCMI_RIS register
            pub const Clear: u32 = 0b1;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// embedded synchronization code register
pub mod ESCR {

    /// Frame end delimiter code
    pub mod FEC {
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

    /// Line end delimiter code
    pub mod LEC {
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

    /// Line start delimiter code
    pub mod LSC {
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

    /// Frame start delimiter code
    pub mod FSC {
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

/// embedded synchronization unmask register
pub mod ESUR {

    /// Frame end delimiter unmask
    pub mod FEU {
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

    /// Line end delimiter unmask
    pub mod LEU {
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

    /// Line start delimiter unmask
    pub mod LSU {
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

    /// Frame start delimiter unmask
    pub mod FSU {
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

/// crop window start
pub mod CWSTRT {

    /// Vertical start line count
    pub mod VST {
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

    /// Horizontal offset count
    pub mod HOFFCNT {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (14 bits: 0x3fff << 0)
        pub const mask: u32 = 0x3fff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// crop window size
pub mod CWSIZE {

    /// Vertical line count
    pub mod VLINE {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (14 bits: 0x3fff << 16)
        pub const mask: u32 = 0x3fff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Capture count
    pub mod CAPCNT {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (14 bits: 0x3fff << 0)
        pub const mask: u32 = 0x3fff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// data register
pub mod DR {

    /// Data byte 3
    pub mod Byte3 {
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

    /// Data byte 2
    pub mod Byte2 {
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

    /// Data byte 1
    pub mod Byte1 {
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

    /// Data byte 0
    pub mod Byte0 {
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
#[repr(C)]
pub struct RegisterBlock {
    /// control register 1
    pub CR: RWRegister<u32>,

    /// status register
    pub SR: RORegister<u32>,

    /// raw interrupt status register
    pub RIS: RORegister<u32>,

    /// interrupt enable register
    pub IER: RWRegister<u32>,

    /// masked interrupt status register
    pub MIS: RORegister<u32>,

    /// interrupt clear register
    pub ICR: WORegister<u32>,

    /// embedded synchronization code register
    pub ESCR: RWRegister<u32>,

    /// embedded synchronization unmask register
    pub ESUR: RWRegister<u32>,

    /// crop window start
    pub CWSTRT: RWRegister<u32>,

    /// crop window size
    pub CWSIZE: RWRegister<u32>,

    /// data register
    pub DR: RORegister<u32>,
}
pub struct ResetValues {
    pub CR: u32,
    pub SR: u32,
    pub RIS: u32,
    pub IER: u32,
    pub MIS: u32,
    pub ICR: u32,
    pub ESCR: u32,
    pub ESUR: u32,
    pub CWSTRT: u32,
    pub CWSIZE: u32,
    pub DR: u32,
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

/// Access functions for the DCMI peripheral instance
pub mod DCMI {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x50050000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in DCMI
    pub const reset: ResetValues = ResetValues {
        CR: 0x00000000,
        SR: 0x00000000,
        RIS: 0x00000000,
        IER: 0x00000000,
        MIS: 0x00000000,
        ICR: 0x00000000,
        ESCR: 0x00000000,
        ESUR: 0x00000000,
        CWSTRT: 0x00000000,
        CWSIZE: 0x00000000,
        DR: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut DCMI_TAKEN: bool = false;

    /// Safe access to DCMI
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
            if DCMI_TAKEN {
                None
            } else {
                DCMI_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to DCMI
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if DCMI_TAKEN && inst.addr == INSTANCE.addr {
                DCMI_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal DCMI
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        DCMI_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to DCMI
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const DCMI: *const RegisterBlock = 0x50050000 as *const _;
