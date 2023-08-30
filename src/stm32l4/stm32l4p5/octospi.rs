#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! OctoSPI

use crate::{RWRegister, WORegister};
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// control register
pub mod CR {

    /// Functional mode
    pub mod FMODE {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (2 bits: 0b11 << 28)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Indirect-write mode
            pub const IndirectWrite: u32 = 0b00;

            /// 0b01: Indirect-read mode
            pub const IndirectRead: u32 = 0b01;

            /// 0b10: Automatic status-polling mode
            pub const AutomaticPolling: u32 = 0b10;

            /// 0b11: Memory-mapped mode
            pub const MemoryMapped: u32 = 0b11;
        }
    }

    /// Polling match mode
    pub mod PMM {
        /// Offset (23 bits)
        pub const offset: u32 = 23;
        /// Mask (1 bit: 1 << 23)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: AND-match mode, SMF is set if all the unmasked bits received from the device match the corresponding bits in the match register
            pub const ANDMatchMode: u32 = 0b0;

            /// 0b1: OR-match mode, SMF is set if any of the unmasked bits received from the device matches its corresponding bit in the match register
            pub const ORMatchmode: u32 = 0b1;
        }
    }

    /// Automatic poll mode stop
    pub mod APMS {
        /// Offset (22 bits)
        pub const offset: u32 = 22;
        /// Mask (1 bit: 1 << 22)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Automatic status-polling mode is stopped only by abort or by disabling the OCTOSPI
            pub const Running: u32 = 0b0;

            /// 0b1: Automatic status-polling mode stops as soon as there is a match
            pub const StopMatch: u32 = 0b1;
        }
    }

    /// TimeOut interrupt enable
    pub mod TOIE {
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

            /// 0b0: Interrupt disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Interrupt enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Status match interrupt enable
    pub mod SMIE {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (1 bit: 1 << 19)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TOIE::RW;
    }

    /// FIFO threshold interrupt enable
    pub mod FTIE {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (1 bit: 1 << 18)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TOIE::RW;
    }

    /// Transfer complete interrupt enable
    pub mod TCIE {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TOIE::RW;
    }

    /// Transfer error interrupt enable
    pub mod TEIE {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TOIE::RW;
    }

    /// IFO threshold level
    pub mod FTHRES {
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

    /// FLASH memory selection
    pub mod FSEL {
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

            /// 0b0: FLASH 1 selected (data exchanged over IO\[3:0\])
            pub const FLASH1: u32 = 0b0;

            /// 0b1: FLASH 2 selected (data exchanged over IO\[7:4\])
            pub const FLASH2: u32 = 0b1;
        }
    }

    /// Dual-memory configuration
    pub mod DMM {
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

            /// 0b0: Dual-quad configuration disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Dual-quad configuration enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Timeout counter enable
    pub mod TCEN {
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

            /// 0b0: Timeout counter is disabled, and thus the chip-select (NCS) remains active indefinitely after an access in Memory-mapped mode
            pub const Disabled: u32 = 0b0;

            /// 0b1: Timeout counter is enabled, and thus the chip-select is released in the Memory-mapped mode after TIMEOUT\[15:0\] cycles of external device inactivity
            pub const Enabled: u32 = 0b1;
        }
    }

    /// DMA enable
    pub mod DMAEN {
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

            /// 0b0: DMA disabled for Indirect mode
            pub const Disabled: u32 = 0b0;

            /// 0b1: DMA enabled for Indirect mode
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Abort request
    pub mod ABORT {
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

            /// 0b0: No abort requested
            pub const NotRequested: u32 = 0b0;

            /// 0b1: Abort requested
            pub const Requested: u32 = 0b1;
        }
    }

    /// Enable
    pub mod EN {
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

            /// 0b0: OCTOSPI disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: OCTOSPI enabled
            pub const Enabled: u32 = 0b1;
        }
    }
}

/// device configuration register
pub mod DCR1 {

    /// Mode 0 / mode 3
    pub mod CKMODE {
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

            /// 0b0: CLK must stay low while NCS is high (chip-select released). This is referred to as Mode 0
            pub const Mode0: u32 = 0b0;

            /// 0b1: CLK must stay high while NCS is high (chip-select released). This is referred to as Mode 3
            pub const Mode3: u32 = 0b1;
        }
    }

    /// Free running clock
    pub mod FRCK {
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

            /// 0b0: CLK is not free running
            pub const Disabled: u32 = 0b0;

            /// 0b1: CLK is free running (always provided)
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Chip-select high time
    pub mod CSHT {
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

    /// Device size
    pub mod DEVSIZE {
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

    /// Memory type
    pub mod MTYP {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (3 bits: 0b111 << 24)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: Micron mode, D0/D1 ordering in DTR 8-data-bit mode. Regular-command protocol in Single-, Dual-, Quad- and Octal-SPI modes
            pub const MicronMode: u32 = 0b000;

            /// 0b001: Macronix mode, D1/D0 ordering in DTR 8-data-bit mode. Regular-command protocol in Single-, Dual-, Quad- and Octal-SPI modes
            pub const MacronixMode: u32 = 0b001;

            /// 0b010: Standard Mode
            pub const StandardMode: u32 = 0b010;

            /// 0b011: Macronix RAM mode, D1/D0 ordering in DTR 8-data-bit mode. Regular-command protocol in Single-, Dual-, Quad- and Octal-SPI modes with dedicated address mapping
            pub const MacronixRamMode: u32 = 0b011;

            /// 0b100: HyperBus memory mode, the protocol follows the HyperBus specification. 8-data-bit DTR mode must be selected
            pub const HyperBusMemoryMode: u32 = 0b100;

            /// 0b101: HyperBus register mode, addressing register space. The memory-mapped accesses in this mode must be non-cacheable, or Indirect read/write modes must be used
            pub const HyperBusMode: u32 = 0b101;
        }
    }

    /// Delay block bypass
    pub mod DLYBYP {
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

            /// 0b0: The internal sampling clock (called feedback clock) or the DQS data strobe external signal is delayed by the delay block (for more details on this block, refer to the dedicated section of the reference manual as it is not part of the OCTOSPI peripheral)
            pub const DelayBlockEnabled: u32 = 0b0;

            /// 0b1: The delay block is bypassed, so the internal sampling clock or the DQS data strobe external signal is not affected by the delay block. The delay is shorter than when the delay block is not bypassed, even with the delay value set to minimum value in delay block
            pub const DelayBlockBypassed: u32 = 0b1;
        }
    }
}

/// device configuration register 2
pub mod DCR2 {

    /// Clock prescaler
    pub mod PRESCALER {
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

    /// Wrap size
    pub mod WRAPSIZE {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (3 bits: 0b111 << 16)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: Wrapped reads are not supported by the memory
            pub const NoWrappingSupport: u32 = 0b000;

            /// 0b010: External memory supports wrap size of 16 bytes
            pub const WrappingSize16: u32 = 0b010;

            /// 0b011: External memory supports wrap size of 16 bytes
            pub const WrappingSize32: u32 = 0b011;

            /// 0b100: External memory supports wrap size of 16 bytes
            pub const WrappingSize64: u32 = 0b100;

            /// 0b101: External memory supports wrap size of 16 bytes
            pub const WrappingSize128: u32 = 0b101;
        }
    }
}

/// device configuration register 3
pub mod DCR3 {

    /// CS boundary
    pub mod CSBOUND {
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

    /// Maximum transfer
    pub mod MAXTRAN {
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

/// status register
pub mod SR {

    /// Transfer error flag
    pub mod TEF {
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

            /// 0b0: This bit is cleared by writing 1 to CTEF
            pub const Cleared: u32 = 0b0;

            /// 0b1: This bit is set in Indirect mode when an invalid address is being accessed in Indirect mode
            pub const InvalidAddressAccessed: u32 = 0b1;
        }
    }

    /// Transfer complete flag
    pub mod TCF {
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

            /// 0b0: This bit is cleared by writing 1 to CTCF
            pub const Cleared: u32 = 0b0;

            /// 0b1: This bit is set when the programmed number of data has been transferred
            pub const TransferCompleted: u32 = 0b1;
        }
    }

    /// FIFO threshold flag
    pub mod FTF {
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

            /// 0b0: It is cleared automatically as soon as the threshold condition is no longer true
            pub const Cleared: u32 = 0b0;

            /// 0b1: This bit is set when the FIFO threshold has been reached
            pub const ThresholdReached: u32 = 0b1;
        }
    }

    /// Status match flag
    pub mod SMF {
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

            /// 0b0: It is cleared by writing 1 to CSMF
            pub const Cleared: u32 = 0b0;

            /// 0b1: This bit is set in Automatic status-polling mode when the unmasked received data matches the corresponding bits in the match register (OCTOSPI_PSMAR)
            pub const Matched: u32 = 0b1;
        }
    }

    /// Timeout flag
    pub mod TOF {
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

            /// 0b0: This bit is cleared by writing 1 to CTOF
            pub const Cleared: u32 = 0b0;

            /// 0b1: This bit is set when timeout occurs
            pub const Timeout: u32 = 0b1;
        }
    }

    /// BUSY
    pub mod BUSY {
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

            /// 0b0: This bit is cleared automatically when the operation with the external device is finished and the FIFO is empty
            pub const Cleared: u32 = 0b0;

            /// 0b1: This bit is set when an operation is ongoing
            pub const Busy: u32 = 0b1;
        }
    }

    /// FIFO level
    pub mod FLEVEL {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (6 bits: 0x3f << 8)
        pub const mask: u32 = 0x3f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// flag clear register
pub mod FCR {

    /// Clear transfer error flag
    pub mod CTEF {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values
        pub mod W {

            /// 0b1: Writing 1 clears the TEF flag in the OCTOSPI_SR register
            pub const Clear: u32 = 0b1;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Clear transfer complete flag
    pub mod CTCF {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values
        pub mod W {

            /// 0b1: Writing 1 clears the TCF flag in the OCTOSPI_SR register
            pub const Clear: u32 = 0b1;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Clear status match flag
    pub mod CSMF {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values
        pub mod W {

            /// 0b1: Writing 1 clears the SMF flag in the OCTOSPI_SR register
            pub const Clear: u32 = 0b1;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Clear timeout flag
    pub mod CTOF {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values
        pub mod W {

            /// 0b1: Writing 1 clears the TOF flag in the OCTOSPI_SR register
            pub const Clear: u32 = 0b1;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// data length register
pub mod DLR {

    /// Data length
    pub mod DL {
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

/// address register
pub mod AR {

    /// ADDRESS
    pub mod ADDRESS {
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

/// data register
pub mod DR {

    /// Data
    pub mod DATA {
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

/// polling status mask register
pub mod PSMKR {

    /// Status mask
    pub mod MASK {
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

/// polling status match register
pub mod PSMAR {

    /// Status match
    pub mod MATCH {
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

/// polling interval register
pub mod PIR {

    /// Polling interval
    pub mod INTERVAL {
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

/// communication configuration register
pub mod CCR {

    /// Instruction mode
    pub mod IMODE {
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

            /// 0b000: No instruction
            pub const NoInstruction: u32 = 0b000;

            /// 0b001: Instruction on a single line
            pub const SingleLine: u32 = 0b001;

            /// 0b010: Instruction on two lines
            pub const TwoLines: u32 = 0b010;

            /// 0b011: Instruction on four lines
            pub const FourLines: u32 = 0b011;

            /// 0b100: Instruction on eight lines
            pub const EightLines: u32 = 0b100;
        }
    }

    /// Instruction double transfer rate
    pub mod IDTR {
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

            /// 0b0: DTR mode disabled for instruction phase
            pub const Disabled: u32 = 0b0;

            /// 0b1: DTR mode enabled for instruction phase
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Instruction size
    pub mod ISIZE {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (2 bits: 0b11 << 4)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: 8-bit instruction
            pub const Bits8: u32 = 0b00;

            /// 0b01: 16-bit instruction
            pub const Bits16: u32 = 0b01;

            /// 0b10: 24-bit instruction
            pub const Bits24: u32 = 0b10;

            /// 0b11: 32-bit instruction
            pub const Bits32: u32 = 0b11;
        }
    }

    /// Address mode
    pub mod ADMODE {
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

            /// 0b000: No address
            pub const NoAddress: u32 = 0b000;

            /// 0b001: Address on a single line
            pub const SingleLine: u32 = 0b001;

            /// 0b010: Address on two lines
            pub const TwoLines: u32 = 0b010;

            /// 0b011: Address on four lines
            pub const FourLines: u32 = 0b011;

            /// 0b100: Address on eight lines
            pub const EightLines: u32 = 0b100;
        }
    }

    /// Address double transfer rate
    pub mod ADDTR {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: DTR mode disabled for address phase
            pub const Disabled: u32 = 0b0;

            /// 0b1: DTR mode enabled for address phase
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Address size
    pub mod ADSIZE {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (2 bits: 0b11 << 12)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: 8-bit address
            pub const Bits8: u32 = 0b00;

            /// 0b01: 16-bit address
            pub const Bits16: u32 = 0b01;

            /// 0b10: 24-bit address
            pub const Bits24: u32 = 0b10;

            /// 0b11: 32-bit address
            pub const Bits32: u32 = 0b11;
        }
    }

    /// Alternate byte mode
    pub mod ABMODE {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (3 bits: 0b111 << 16)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: No alternate bytes
            pub const NoAlternateBytes: u32 = 0b000;

            /// 0b001: Alternate bytes on a single line
            pub const SingleLine: u32 = 0b001;

            /// 0b010: Alternate bytes on two lines
            pub const TwoLines: u32 = 0b010;

            /// 0b011: Alternate bytes on four lines
            pub const FourLines: u32 = 0b011;

            /// 0b100: Alternate bytes on eight lines
            pub const EightLines: u32 = 0b100;
        }
    }

    /// Alternate bytes double transfer rate
    pub mod ABDTR {
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

            /// 0b0: DTR mode disabled for alternate bytes phase
            pub const Disabled: u32 = 0b0;

            /// 0b1: DTR mode enabled for alternate bytes phase
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Alternate bytes size
    pub mod ABSIZE {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (2 bits: 0b11 << 20)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: 8-bit alternate bytes
            pub const Bits8: u32 = 0b00;

            /// 0b01: 16-bit alternate bytes
            pub const Bits16: u32 = 0b01;

            /// 0b10: 24-bit alternate bytes
            pub const Bits24: u32 = 0b10;

            /// 0b11: 32-bit alternate bytes
            pub const Bits32: u32 = 0b11;
        }
    }

    /// Data mode
    pub mod DMODE {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (3 bits: 0b111 << 24)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: No data
            pub const NoData: u32 = 0b000;

            /// 0b001: Data on a single line
            pub const SingleLine: u32 = 0b001;

            /// 0b010: Data on two lines
            pub const TwoLines: u32 = 0b010;

            /// 0b011: Data on four lines
            pub const FourLines: u32 = 0b011;

            /// 0b100: Data on eight lines
            pub const EightLines: u32 = 0b100;
        }
    }

    /// Alternate bytes double transfer rate
    pub mod DDTR {
        /// Offset (27 bits)
        pub const offset: u32 = 27;
        /// Mask (1 bit: 1 << 27)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: DTR mode disabled for data phase
            pub const Disabled: u32 = 0b0;

            /// 0b1: DTR mode enabled for data phase
            pub const Enabled: u32 = 0b1;
        }
    }

    /// DQS enable
    pub mod DQSE {
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

            /// 0b0: DQS disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: DQS enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Send instruction only once mode
    pub mod SIOO {
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

            /// 0b0: Send instruction on every transaction
            pub const SendEveryTransaction: u32 = 0b0;

            /// 0b1: Send instruction only for the first command
            pub const SendOnlyFirstCmd: u32 = 0b1;
        }
    }
}

/// timing configuration register
pub mod TCR {

    /// Number of dummy cycles
    pub mod DCYC {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (5 bits: 0b11111 << 0)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Delay hold quarter cycle
    pub mod DHQC {
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

            /// 0b0: No delay hold
            pub const NoDelay: u32 = 0b0;

            /// 0b1: 1/4 cycle hold
            pub const QuarterCycleHold: u32 = 0b1;
        }
    }

    /// Sample shift
    pub mod SSHIFT {
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

            /// 0b0: No shift
            pub const NoShift: u32 = 0b0;

            /// 0b1: 1/2 cycle shift
            pub const HalfCycleShift: u32 = 0b1;
        }
    }
}

/// instruction register
pub mod IR {

    /// INSTRUCTION
    pub mod INSTRUCTION {
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

/// alternate bytes register
pub mod ABR {

    /// Alternate bytes
    pub mod ALTERNATE {
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

/// low-power timeout register
pub mod LPTR {

    /// Timeout period
    pub mod TIMEOUT {
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

/// write communication configuration register
pub mod WCCR {

    /// Instruction mode
    pub mod IMODE {
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

            /// 0b000: No instruction
            pub const NoInstruction: u32 = 0b000;

            /// 0b001: Instruction on a single line
            pub const SingleLine: u32 = 0b001;

            /// 0b010: Instruction on two lines
            pub const TwoLines: u32 = 0b010;

            /// 0b011: Instruction on four lines
            pub const FourLines: u32 = 0b011;

            /// 0b100: Instruction on eight lines
            pub const EightLines: u32 = 0b100;
        }
    }

    /// Instruction double transfer rate
    pub mod IDTR {
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

            /// 0b0: DTR mode disabled for instruction phase
            pub const Disabled: u32 = 0b0;

            /// 0b1: DTR mode enabled for instruction phase
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Instruction size
    pub mod ISIZE {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (2 bits: 0b11 << 4)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: 8-bit instruction
            pub const Bits8: u32 = 0b00;

            /// 0b01: 16-bit instruction
            pub const Bits16: u32 = 0b01;

            /// 0b10: 24-bit instruction
            pub const Bits24: u32 = 0b10;

            /// 0b11: 32-bit instruction
            pub const Bits32: u32 = 0b11;
        }
    }

    /// Address mode
    pub mod ADMODE {
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

            /// 0b000: No address
            pub const NoAddress: u32 = 0b000;

            /// 0b001: Address on a single line
            pub const SingleLine: u32 = 0b001;

            /// 0b010: Address on two lines
            pub const TwoLines: u32 = 0b010;

            /// 0b011: Address on four lines
            pub const FourLines: u32 = 0b011;

            /// 0b100: Address on eight lines
            pub const EightLines: u32 = 0b100;
        }
    }

    /// Address double transfer rate
    pub mod ADDTR {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: DTR mode disabled for address phase
            pub const Disabled: u32 = 0b0;

            /// 0b1: DTR mode enabled for address phase
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Address size
    pub mod ADSIZE {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (2 bits: 0b11 << 12)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: 8-bit address
            pub const Bits8: u32 = 0b00;

            /// 0b01: 16-bit address
            pub const Bits16: u32 = 0b01;

            /// 0b10: 24-bit address
            pub const Bits24: u32 = 0b10;

            /// 0b11: 32-bit address
            pub const Bits32: u32 = 0b11;
        }
    }

    /// Alternate byte mode
    pub mod ABMODE {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (3 bits: 0b111 << 16)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: No alternate bytes
            pub const NoAlternateBytes: u32 = 0b000;

            /// 0b001: Alternate bytes on a single line
            pub const SingleLine: u32 = 0b001;

            /// 0b010: Alternate bytes on two lines
            pub const TwoLines: u32 = 0b010;

            /// 0b011: Alternate bytes on four lines
            pub const FourLines: u32 = 0b011;

            /// 0b100: Alternate bytes on eight lines
            pub const EightLines: u32 = 0b100;
        }
    }

    /// Alternate bytes double transfer rate
    pub mod ABDTR {
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

            /// 0b0: DTR mode disabled for alternate bytes phase
            pub const Disabled: u32 = 0b0;

            /// 0b1: DTR mode enabled for alternate bytes phase
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Alternate bytes size
    pub mod ABSIZE {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (2 bits: 0b11 << 20)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: 8-bit alternate bytes
            pub const Bits8: u32 = 0b00;

            /// 0b01: 16-bit alternate bytes
            pub const Bits16: u32 = 0b01;

            /// 0b10: 24-bit alternate bytes
            pub const Bits24: u32 = 0b10;

            /// 0b11: 32-bit alternate bytes
            pub const Bits32: u32 = 0b11;
        }
    }

    /// Data mode
    pub mod DMODE {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (3 bits: 0b111 << 24)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: No data
            pub const NoData: u32 = 0b000;

            /// 0b001: Data on a single line
            pub const SingleLine: u32 = 0b001;

            /// 0b010: Data on two lines
            pub const TwoLines: u32 = 0b010;

            /// 0b011: Data on four lines
            pub const FourLines: u32 = 0b011;

            /// 0b100: Data on eight lines
            pub const EightLines: u32 = 0b100;
        }
    }

    /// alternate bytes double transfer rate
    pub mod DDTR {
        /// Offset (27 bits)
        pub const offset: u32 = 27;
        /// Mask (1 bit: 1 << 27)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: DTR mode disabled for data phase
            pub const Disabled: u32 = 0b0;

            /// 0b1: DTR mode enabled for data phase
            pub const Enabled: u32 = 0b1;
        }
    }

    /// DQS enable
    pub mod DQSE {
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

            /// 0b0: DQS disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: DQS enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Send instruction only once mode
    pub mod SIOO {
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

/// write timing configuration register
pub mod WTCR {

    /// Number of dummy cycles
    pub mod DCYC {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (5 bits: 0b11111 << 0)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// write instruction register
pub mod WIR {
    pub use super::IR::INSTRUCTION;
}

/// write alternate bytes register
pub mod WABR {
    pub use super::ABR::ALTERNATE;
}

/// HyperBusTM latency configuration register
pub mod HLCR {

    /// Latency mode
    pub mod LM {
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

            /// 0b0: Variable initial latency
            pub const Variable: u32 = 0b0;

            /// 0b1: Fixed latency
            pub const Fixed: u32 = 0b1;
        }
    }

    /// Write zero latency
    pub mod WZL {
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

            /// 0b0: Latency on write accesses
            pub const Enabled: u32 = 0b0;

            /// 0b1: No latency on write accesses
            pub const Disabled: u32 = 0b1;
        }
    }

    /// Access time
    pub mod TACC {
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

    /// Read write recovery time
    pub mod TRWR {
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

/// Device configuration register 4
pub mod DCR4 {

    /// Refresh rate
    pub mod REFRESH {
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

/// wrap communication configuration register
pub mod WPCCR {

    /// DQS enable
    pub mod DQSE {
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

            /// 0b0: DQS disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: DQS enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Data double transfer rate
    pub mod DDTR {
        /// Offset (27 bits)
        pub const offset: u32 = 27;
        /// Mask (1 bit: 1 << 27)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: DTR mode disabled for data phase
            pub const Disabled: u32 = 0b0;

            /// 0b1: DTR mode enabled for data phase
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Data mode
    pub mod DMODE {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (3 bits: 0b111 << 24)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: No data
            pub const NoData: u32 = 0b000;

            /// 0b001: Data on a single line
            pub const SingleLine: u32 = 0b001;

            /// 0b010: Data on two lines
            pub const TwoLines: u32 = 0b010;

            /// 0b011: Data on four lines
            pub const FourLines: u32 = 0b011;

            /// 0b100: Data on eight lines
            pub const EightLines: u32 = 0b100;
        }
    }

    /// Alternate bytes size
    pub mod ABSIZE {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (2 bits: 0b11 << 20)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: 8-bit alternate bytes
            pub const Bits8: u32 = 0b00;

            /// 0b01: 16-bit alternate bytes
            pub const Bits16: u32 = 0b01;

            /// 0b10: 24-bit alternate bytes
            pub const Bits24: u32 = 0b10;

            /// 0b11: 32-bit alternate bytes
            pub const Bits32: u32 = 0b11;
        }
    }

    /// Alternate bytes double transfer rate
    pub mod ABDTR {
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

            /// 0b0: DTR mode disabled for alternate bytes phase
            pub const Disabled: u32 = 0b0;

            /// 0b1: DTR mode enabled for alternate bytes phase
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Alternate-byte mode
    pub mod ABMODE {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (3 bits: 0b111 << 16)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: No alternate bytes
            pub const NoAlternateBytes: u32 = 0b000;

            /// 0b001: Alternate bytes on a single line
            pub const SingleLine: u32 = 0b001;

            /// 0b010: Alternate bytes on two lines
            pub const TwoLines: u32 = 0b010;

            /// 0b011: Alternate bytes on four lines
            pub const FourLines: u32 = 0b011;

            /// 0b100: Alternate bytes on eight lines
            pub const EightLines: u32 = 0b100;
        }
    }

    /// Address size
    pub mod ADSIZE {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (2 bits: 0b11 << 12)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: 8-bit address
            pub const Bits8: u32 = 0b00;

            /// 0b01: 16-bit address
            pub const Bits16: u32 = 0b01;

            /// 0b10: 24-bit address
            pub const Bits24: u32 = 0b10;

            /// 0b11: 32-bit address
            pub const Bits32: u32 = 0b11;
        }
    }

    /// Address double transfer rate
    pub mod ADDTR {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: DTR mode disabled for address phase
            pub const Disabled: u32 = 0b0;

            /// 0b1: DTR mode enabled for address phase
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Address mode
    pub mod ADMODE {
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

            /// 0b000: No address
            pub const NoAddress: u32 = 0b000;

            /// 0b001: Address on a single line
            pub const SingleLine: u32 = 0b001;

            /// 0b010: Address on two lines
            pub const TwoLines: u32 = 0b010;

            /// 0b011: Address on four lines
            pub const FourLines: u32 = 0b011;

            /// 0b100: Address on eight lines
            pub const EightLines: u32 = 0b100;
        }
    }

    /// Instruction size
    pub mod ISIZE {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (2 bits: 0b11 << 4)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: 8-bit instruction
            pub const Bits8: u32 = 0b00;

            /// 0b01: 16-bit instruction
            pub const Bits16: u32 = 0b01;

            /// 0b10: 24-bit instruction
            pub const Bits24: u32 = 0b10;

            /// 0b11: 32-bit instruction
            pub const Bits32: u32 = 0b11;
        }
    }

    /// Instruction double transfer rate
    pub mod IDTR {
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

            /// 0b0: DTR mode disabled for instruction phase
            pub const Disabled: u32 = 0b0;

            /// 0b1: DTR mode enabled for instruction phase
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Instruction mode
    pub mod IMODE {
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

            /// 0b000: No instruction
            pub const NoInstruction: u32 = 0b000;

            /// 0b001: Instruction on a single line
            pub const SingleLine: u32 = 0b001;

            /// 0b010: Instruction on two lines
            pub const TwoLines: u32 = 0b010;

            /// 0b011: Instruction on four lines
            pub const FourLines: u32 = 0b011;

            /// 0b100: Instruction on eight lines
            pub const EightLines: u32 = 0b100;
        }
    }
}

/// Wrap timing configuration register
pub mod WPTCR {

    /// Sample shift
    pub mod SSHIFT {
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

            /// 0b0: No shift
            pub const NoShift: u32 = 0b0;

            /// 0b1: 1/2 cycle shift
            pub const HalfCycleShift: u32 = 0b1;
        }
    }

    /// Delay hold quarter cycle
    pub mod DHQC {
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

            /// 0b0: No delay hold
            pub const NoDelay: u32 = 0b0;

            /// 0b1: 1/4 cycle hold
            pub const QuarterCycleHold: u32 = 0b1;
        }
    }

    /// Number of dummy cycles
    pub mod DCYC {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (5 bits: 0b11111 << 0)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Wrap instruction register
pub mod WPIR {

    /// Instruction
    pub mod INSTRUCTION {
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

/// Wrap alternate bytes register
pub mod WPABR {

    /// Alternate bytes
    pub mod ALTERNATE {
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
#[repr(C)]
pub struct RegisterBlock {
    /// control register
    pub CR: RWRegister<u32>,

    _reserved1: [u8; 4],

    /// device configuration register
    pub DCR1: RWRegister<u32>,

    /// device configuration register 2
    pub DCR2: RWRegister<u32>,

    /// device configuration register 3
    pub DCR3: RWRegister<u32>,

    /// Device configuration register 4
    pub DCR4: RWRegister<u32>,

    _reserved2: [u8; 8],

    /// status register
    pub SR: RWRegister<u32>,

    /// flag clear register
    pub FCR: WORegister<u32>,

    _reserved3: [u8; 24],

    /// data length register
    pub DLR: RWRegister<u32>,

    _reserved4: [u8; 4],

    /// address register
    pub AR: RWRegister<u32>,

    _reserved5: [u8; 4],

    /// data register
    pub DR: RWRegister<u32>,

    _reserved6: [u8; 44],

    /// polling status mask register
    pub PSMKR: RWRegister<u32>,

    _reserved7: [u8; 4],

    /// polling status match register
    pub PSMAR: RWRegister<u32>,

    _reserved8: [u8; 4],

    /// polling interval register
    pub PIR: RWRegister<u32>,

    _reserved9: [u8; 108],

    /// communication configuration register
    pub CCR: RWRegister<u32>,

    _reserved10: [u8; 4],

    /// timing configuration register
    pub TCR: RWRegister<u32>,

    _reserved11: [u8; 4],

    /// instruction register
    pub IR: RWRegister<u32>,

    _reserved12: [u8; 12],

    /// alternate bytes register
    pub ABR: RWRegister<u32>,

    _reserved13: [u8; 12],

    /// low-power timeout register
    pub LPTR: RWRegister<u32>,

    _reserved14: [u8; 12],

    /// wrap communication configuration register
    pub WPCCR: RWRegister<u32>,

    _reserved15: [u8; 4],

    /// Wrap timing configuration register
    pub WPTCR: RWRegister<u32>,

    _reserved16: [u8; 4],

    /// Wrap instruction register
    pub WPIR: RWRegister<u32>,

    _reserved17: [u8; 12],

    /// Wrap alternate bytes register
    pub WPABR: RWRegister<u32>,

    _reserved18: [u8; 28],

    /// write communication configuration register
    pub WCCR: RWRegister<u32>,

    _reserved19: [u8; 4],

    /// write timing configuration register
    pub WTCR: RWRegister<u32>,

    _reserved20: [u8; 4],

    /// write instruction register
    pub WIR: RWRegister<u32>,

    _reserved21: [u8; 12],

    /// write alternate bytes register
    pub WABR: RWRegister<u32>,

    _reserved22: [u8; 92],

    /// HyperBusTM latency configuration register
    pub HLCR: RWRegister<u32>,
}
pub struct ResetValues {
    pub CR: u32,
    pub DCR1: u32,
    pub DCR2: u32,
    pub DCR3: u32,
    pub DCR4: u32,
    pub SR: u32,
    pub FCR: u32,
    pub DLR: u32,
    pub AR: u32,
    pub DR: u32,
    pub PSMKR: u32,
    pub PSMAR: u32,
    pub PIR: u32,
    pub CCR: u32,
    pub TCR: u32,
    pub IR: u32,
    pub ABR: u32,
    pub LPTR: u32,
    pub WPCCR: u32,
    pub WPTCR: u32,
    pub WPIR: u32,
    pub WPABR: u32,
    pub WCCR: u32,
    pub WTCR: u32,
    pub WIR: u32,
    pub WABR: u32,
    pub HLCR: u32,
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

/// Access functions for the OCTOSPI1 peripheral instance
pub mod OCTOSPI1 {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0xa0001000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in OCTOSPI1
    pub const reset: ResetValues = ResetValues {
        CR: 0x00000000,
        DCR1: 0x00000000,
        DCR2: 0x00000000,
        DCR3: 0x00000000,
        SR: 0x00000000,
        FCR: 0x00000000,
        DLR: 0x00000000,
        AR: 0x00000000,
        DR: 0x00000000,
        PSMKR: 0x00000000,
        PSMAR: 0x00000000,
        PIR: 0x00000000,
        CCR: 0x00000000,
        TCR: 0x00000000,
        IR: 0x00000000,
        ABR: 0x00000000,
        LPTR: 0x00000000,
        WCCR: 0x00000000,
        WTCR: 0x00000000,
        WIR: 0x00000000,
        WABR: 0x00000000,
        HLCR: 0x00000000,
        DCR4: 0x00000000,
        WPCCR: 0x00000000,
        WPTCR: 0x00000000,
        WPIR: 0x00000000,
        WPABR: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut OCTOSPI1_TAKEN: bool = false;

    /// Safe access to OCTOSPI1
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
            if OCTOSPI1_TAKEN {
                None
            } else {
                OCTOSPI1_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to OCTOSPI1
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if OCTOSPI1_TAKEN && inst.addr == INSTANCE.addr {
                OCTOSPI1_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal OCTOSPI1
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        OCTOSPI1_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to OCTOSPI1
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const OCTOSPI1: *const RegisterBlock = 0xa0001000 as *const _;

/// Access functions for the OCTOSPI2 peripheral instance
pub mod OCTOSPI2 {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0xa0001400,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in OCTOSPI2
    pub const reset: ResetValues = ResetValues {
        CR: 0x00000000,
        DCR1: 0x00000000,
        DCR2: 0x00000000,
        DCR3: 0x00000000,
        SR: 0x00000000,
        FCR: 0x00000000,
        DLR: 0x00000000,
        AR: 0x00000000,
        DR: 0x00000000,
        PSMKR: 0x00000000,
        PSMAR: 0x00000000,
        PIR: 0x00000000,
        CCR: 0x00000000,
        TCR: 0x00000000,
        IR: 0x00000000,
        ABR: 0x00000000,
        LPTR: 0x00000000,
        WCCR: 0x00000000,
        WTCR: 0x00000000,
        WIR: 0x00000000,
        WABR: 0x00000000,
        HLCR: 0x00000000,
        DCR4: 0x00000000,
        WPCCR: 0x00000000,
        WPTCR: 0x00000000,
        WPIR: 0x00000000,
        WPABR: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut OCTOSPI2_TAKEN: bool = false;

    /// Safe access to OCTOSPI2
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
            if OCTOSPI2_TAKEN {
                None
            } else {
                OCTOSPI2_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to OCTOSPI2
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if OCTOSPI2_TAKEN && inst.addr == INSTANCE.addr {
                OCTOSPI2_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal OCTOSPI2
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        OCTOSPI2_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to OCTOSPI2
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const OCTOSPI2: *const RegisterBlock = 0xa0001400 as *const _;
