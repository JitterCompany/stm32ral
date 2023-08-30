#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Flash

use crate::{RWRegister, WORegister};
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// Access control register
pub mod ACR {

    /// Latency
    pub mod LATENCY {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: 0 wait states
            pub const WS0: u32 = 0b0000;

            /// 0b0001: 1 wait states
            pub const WS1: u32 = 0b0001;

            /// 0b0010: 2 wait states
            pub const WS2: u32 = 0b0010;

            /// 0b0011: 3 wait states
            pub const WS3: u32 = 0b0011;

            /// 0b0100: 4 wait states
            pub const WS4: u32 = 0b0100;

            /// 0b0101: 5 wait states
            pub const WS5: u32 = 0b0101;

            /// 0b0110: 6 wait states
            pub const WS6: u32 = 0b0110;

            /// 0b0111: 7 wait states
            pub const WS7: u32 = 0b0111;

            /// 0b1000: 8 wait states
            pub const WS8: u32 = 0b1000;

            /// 0b1001: 9 wait states
            pub const WS9: u32 = 0b1001;

            /// 0b1010: 10 wait states
            pub const WS10: u32 = 0b1010;

            /// 0b1011: 11 wait states
            pub const WS11: u32 = 0b1011;

            /// 0b1100: 12 wait states
            pub const WS12: u32 = 0b1100;

            /// 0b1101: 13 wait states
            pub const WS13: u32 = 0b1101;

            /// 0b1110: 14 wait states
            pub const WS14: u32 = 0b1110;

            /// 0b1111: 15 wait states
            pub const WS15: u32 = 0b1111;
        }
    }

    /// Prefetch enable
    pub mod PRFTEN {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Prefetch is disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Prefetch is enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Instruction cache enable
    pub mod ICEN {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Instruction cache is disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Instruction cache is enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Data cache enable
    pub mod DCEN {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Data cache is disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Data cache is enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Instruction cache reset
    pub mod ICRST {
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

            /// 0b0: Instruction cache is not reset
            pub const NotReset: u32 = 0b0;

            /// 0b1: Instruction cache is reset
            pub const Reset: u32 = 0b1;
        }
    }

    /// Data cache reset
    pub mod DCRST {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Data cache is not reset
            pub const NotReset: u32 = 0b0;

            /// 0b1: Data cache is reset
            pub const Reset: u32 = 0b1;
        }
    }

    /// Flash Power-down mode during Low-power run mode
    pub mod RUN_PD {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Flash in idle mode
            pub const Idle: u32 = 0b0;

            /// 0b1: Flash in Power-down mode
            pub const PowerDown: u32 = 0b1;
        }
    }

    /// Flash Power-down mode during Low-power sleep mode
    pub mod SLEEP_PD {
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

            /// 0b0: Flash in idle mode during Sleep and Low-power sleep modes
            pub const Idle: u32 = 0b0;

            /// 0b1: Flash in Power-down mode during Sleep and Low-power sleep modes
            pub const PowerDown: u32 = 0b1;
        }
    }
}

/// Power down key register
pub mod PDKEYR {

    /// RUN_PD in FLASH_ACR key
    pub mod PDKEY {
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

/// Flash key register
pub mod KEYR {

    /// KEYR
    pub mod KEY {
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

/// Option byte key register
pub mod OPTKEYR {

    /// Option byte key
    pub mod OPTKEY {
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

/// Status register
pub mod SR {

    /// End of operation
    pub mod EOP {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: No error
            pub const NoError: u32 = 0b0;

            /// 0b1: Set by hardware when one or more Flash memory operation (programming / erase) has been completed successfully
            pub const Error: u32 = 0b1;
        }
        /// Write-only values
        pub mod W {

            /// 0b1: Cleared by writing 1
            pub const Clear: u32 = 0b1;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Operation error
    pub mod OPERR {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: No error
            pub const NoError: u32 = 0b0;

            /// 0b1: Set by hardware when a Flash memory operation (program / erase) completes unsuccessfully
            pub const Error: u32 = 0b1;
        }
        pub use super::EOP::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Programming error
    pub mod PROGERR {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: No error
            pub const NoError: u32 = 0b0;

            /// 0b1: Set by hardware when a double-word address to be programmed contains a value different from '0xFFFF FFFF' before programming, except if the data to write is '0x0000 0000'
            pub const Error: u32 = 0b1;
        }
        pub use super::EOP::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Write protected error
    pub mod WRPERR {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: No error
            pub const NoError: u32 = 0b0;

            /// 0b1: Set by hardware when an address to be erased/programmed belongs to a writeprotected part (by WRP, PCROP or RDP level 1) of the Flash memory
            pub const Error: u32 = 0b1;
        }
        pub use super::EOP::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Programming alignment error
    pub mod PGAERR {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: No error
            pub const NoError: u32 = 0b0;

            /// 0b1: Set by hardware when the data to program cannot be contained in the same 64-bit Flash memory row in case of standard programming, or if there is a change of page during fast programming
            pub const Error: u32 = 0b1;
        }
        pub use super::EOP::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Size error
    pub mod SIZERR {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: No error
            pub const NoError: u32 = 0b0;

            /// 0b1: Set by hardware when the size of the access is a byte or half-word during a program or a fast program sequence. Only double word programming is allowed (consequently: word access)
            pub const Error: u32 = 0b1;
        }
        pub use super::EOP::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Programming sequence error
    pub mod PGSERR {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: No error
            pub const NoError: u32 = 0b0;

            /// 0b1: Set by hardware when a write access to the Flash memory is performed by the code while PG or FSTPG have not been set previously. Set also by hardware when PROGERR, SIZERR, PGAERR, WRPERR, MISSERR or FASTERR is set due to a previous programming error. Set also when trying to perform bank erase when DBANK=0 (or DB1M = 0)
            pub const Error: u32 = 0b1;
        }
        pub use super::EOP::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Fast programming data miss error
    pub mod MISERR {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: No error
            pub const NoError: u32 = 0b0;

            /// 0b1: In fast programming mode, 32 double words must be sent to Flash successively, and the new data must be sent to the Flash logic control before the current data is fully programmed. MISSERR is set by hardware when the new data is not present in time
            pub const Error: u32 = 0b1;
        }
        pub use super::EOP::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Fast programming error
    pub mod FASTERR {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: No error
            pub const NoError: u32 = 0b0;

            /// 0b1: Set by hardware when a fast programming sequence (activated by FSTPG) is interrupted due to an error (alignment, size, write protection or data miss). The corresponding status bit (PGAERR, SIZERR, WRPERR or MISSERR) is set at the same time
            pub const Error: u32 = 0b1;
        }
        pub use super::EOP::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// PCROP read error
    pub mod RDERR {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: No error
            pub const NoError: u32 = 0b0;

            /// 0b1: Set by hardware when an address to be read through the D-bus belongs to a read protected area of the Flash (PCROP protection)
            pub const Error: u32 = 0b1;
        }
        pub use super::EOP::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Option validity error
    pub mod OPTVERR {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: No error
            pub const NoError: u32 = 0b0;

            /// 0b1: Set by hardware when the options read may not be the one configured by the user. If option haven’t been properly loaded, OPTVERR is set again after each system reset
            pub const Error: u32 = 0b1;
        }
        pub use super::EOP::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Busy
    pub mod BSY {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: Not busy
            pub const NotBusy: u32 = 0b0;

            /// 0b1: Busy
            pub const Busy: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    ///
    pub mod PEMPTY {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: The bit value is toggling
            pub const Toggling: u32 = 0b0;

            /// 0b1: No effect
            pub const NoEffect: u32 = 0b1;
        }
    }
}

/// Flash control register
pub mod CR {

    /// Programming
    pub mod PG {
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

            /// 0b0: Flash programming disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Flash programming enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Page erase
    pub mod PER {
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

            /// 0b0: Page erase disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Page erase enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Bank 1 Mass erase
    pub mod MER1 {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values
        pub mod W {

            /// 0b1: This bit triggers the bank 1 mass erase (all bank 1 user pages) when set
            pub const MassErase: u32 = 0b1;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Page number
    pub mod PNB {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (8 bits: 0xff << 3)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Bank erase
    pub mod BKER {
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

            /// 0b0: Bank 1 is selected for page erase
            pub const Bank1: u32 = 0b0;

            /// 0b1: Bank 2 is selected for page erase
            pub const Bank2: u32 = 0b1;
        }
    }

    /// Bank 2 Mass erase
    pub mod MER2 {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values
        pub mod W {

            /// 0b1: This bit triggers the bank 2 mass erase (all bank 2 user pages) when set
            pub const MassErase: u32 = 0b1;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Start
    pub mod START {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: Cleared when BSY bit is cleared in SR
            pub const Complete: u32 = 0b0;

            /// 0b1: Erase operation requested
            pub const Requested: u32 = 0b1;
        }
        /// Write-only values
        pub mod W {

            /// 0b1: Trigger an erase operation
            pub const Start: u32 = 0b1;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Options modification start
    pub mod OPTSTRT {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: Cleared when BSY bit is cleared in SR
            pub const Complete: u32 = 0b0;

            /// 0b1: Options modification requested
            pub const Requested: u32 = 0b1;
        }
        /// Write-only values
        pub mod W {

            /// 0b1: This bit triggers an options operation when set
            pub const Set: u32 = 0b1;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Fast programming
    pub mod FSTPG {
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

            /// 0b0: Fast programming disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Fast programming enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// End of operation interrupt enable
    pub mod EOPIE {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (1 bit: 1 << 24)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: End of operation interrupt disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: End of operation interrupt enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Error interrupt enable
    pub mod ERRIE {
        /// Offset (25 bits)
        pub const offset: u32 = 25;
        /// Mask (1 bit: 1 << 25)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Error interrupt generation disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Error interrupt generation enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// PCROP read error interrupt enable
    pub mod RDERRIE {
        /// Offset (26 bits)
        pub const offset: u32 = 26;
        /// Mask (1 bit: 1 << 26)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: PCROP read error interrupt disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: PCROP read error interrupt enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Force the option byte loading
    pub mod OBL_LAUNCH {
        /// Offset (27 bits)
        pub const offset: u32 = 27;
        /// Mask (1 bit: 1 << 27)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: Option byte loading complete
            pub const Complete: u32 = 0b0;

            /// 0b1: Option byte loading requested
            pub const Requested: u32 = 0b1;
        }
        /// Write-only values
        pub mod W {

            /// 0b1: Force option byte reloading
            pub const Set: u32 = 0b1;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Options Lock
    pub mod OPTLOCK {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (1 bit: 1 << 30)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: Option page is unlocked
            pub const Unlocked: u32 = 0b0;

            /// 0b1: All bits concerning user option in FLASH_CR register and so option page are locked
            pub const Locked: u32 = 0b1;
        }
        /// Write-only values
        pub mod W {

            /// 0b1: This bit is set only. When set, all bits concerning user option in FLASH_CR register and so option page are locked
            pub const Set: u32 = 0b1;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// FLASH_CR Lock
    pub mod LOCK {
        /// Offset (31 bits)
        pub const offset: u32 = 31;
        /// Mask (1 bit: 1 << 31)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: FLASH_CR register is unlocked
            pub const Unlocked: u32 = 0b0;

            /// 0b1: FLASH_CR register is locked
            pub const Locked: u32 = 0b1;
        }
        /// Write-only values
        pub mod W {

            /// 0b1: This bit is set only. When set, the FLASH_CR register is locked
            pub const Set: u32 = 0b1;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Flash ECC register
pub mod ECCR {

    /// ECC fail address
    pub mod ADDR_ECC {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (21 bits: 0x1fffff << 0)
        pub const mask: u32 = 0x1fffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// ECC fail bank
    pub mod BK_ECC {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (1 bit: 1 << 19)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: Bank 1
            pub const Bank1: u32 = 0b0;

            /// 0b1: Bank 2
            pub const Bank2: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// System Flash ECC fail
    pub mod SYSF_ECC {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (1 bit: 1 << 20)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b1: This bit indicates that the ECC error correction or double ECC error detection is located in the System Flash
            pub const InSystemFlash: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// ECC correction interrupt enable
    pub mod ECCIE {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (1 bit: 1 << 24)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: ECCC interrupt disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: ECCC interrupt enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// ECC correction
    pub mod ECCC {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (1 bit: 1 << 30)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: No ECC error detected on LSB
            pub const NoError: u32 = 0b0;

            /// 0b1: Set by hardware when one ECC errors have been detected and corrected on LSB
            pub const Error: u32 = 0b1;
        }
        /// Write-only values
        pub mod W {

            /// 0b1: Cleared by writing 1
            pub const Clear: u32 = 0b1;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// ECC detection
    pub mod ECCD {
        /// Offset (31 bits)
        pub const offset: u32 = 31;
        /// Mask (1 bit: 1 << 31)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: No double ECC errors detected on LSB
            pub const NoError: u32 = 0b0;

            /// 0b1: Set by hardware when two ECC errors have been detected on LSB
            pub const Error: u32 = 0b1;
        }
        pub use super::ECCC::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// ECC2 detection
    pub mod ECCD2 {
        /// Offset (29 bits)
        pub const offset: u32 = 29;
        /// Mask (1 bit: 1 << 29)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: No double ECC errors detected on MSB
            pub const NoError: u32 = 0b0;

            /// 0b1: Set by hardware when two ECC errors have been detected on MSB
            pub const Error: u32 = 0b1;
        }
        pub use super::ECCC::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// ECC2 correction
    pub mod ECCC2 {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (1 bit: 1 << 28)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: No ECC error detected on MSB
            pub const NoError: u32 = 0b0;

            /// 0b1: Set by hardware when one ECC errors have been detected and corrected on MSB
            pub const Error: u32 = 0b1;
        }
        pub use super::ECCC::W;
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Flash option register
pub mod OPTR {

    /// Read protection level
    pub mod RDP {
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

    /// BOR reset Level
    pub mod BOR_LEV {
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

    /// nRST_STOP
    pub mod nRST_STOP {
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

    /// nRST_STDBY
    pub mod nRST_STDBY {
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

    /// Independent watchdog selection
    pub mod IDWG_SW {
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

            /// 0b0: Hardware independent watchdog
            pub const Hardware: u32 = 0b0;

            /// 0b1: Software independent watchdog
            pub const Software: u32 = 0b1;
        }
    }

    /// Independent watchdog counter freeze in Stop mode
    pub mod IWDG_STOP {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Independent watchdog counter is frozen in Stop mode
            pub const Frozen: u32 = 0b0;

            /// 0b1: Independent watchdog counter is running in Stop mode
            pub const Running: u32 = 0b1;
        }
    }

    /// Independent watchdog counter freeze in Standby mode
    pub mod IWDG_STDBY {
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

            /// 0b0: Independent watchdog counter is frozen in Standby mode
            pub const Frozen: u32 = 0b0;

            /// 0b1: Independent watchdog counter is running in Standby mode
            pub const Running: u32 = 0b1;
        }
    }

    /// Window watchdog selection
    pub mod WWDG_SW {
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

            /// 0b0: Hardware window watchdog
            pub const Hardware: u32 = 0b0;

            /// 0b1: Software window watchdog
            pub const Software: u32 = 0b1;
        }
    }

    /// Dual-bank boot
    pub mod BFB2 {
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

            /// 0b0: Dual-bank boot disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Dual-bank boot enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Boot configuration
    pub mod nBOOT1 {
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

    /// SRAM2 parity check enable
    pub mod SRAM2_PE {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (1 bit: 1 << 24)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: SRAM2 parity check enabled
            pub const Enabled: u32 = 0b0;

            /// 0b1: SRAM2 parity check disabled
            pub const Disabled: u32 = 0b1;
        }
    }

    /// SRAM2 Erase when system reset
    pub mod SRAM2_RST {
        /// Offset (25 bits)
        pub const offset: u32 = 25;
        /// Mask (1 bit: 1 << 25)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: SRAM2 erased when a system reset occurs
            pub const Enabled: u32 = 0b0;

            /// 0b1: SRAM2 is not erased when a system reset occurs
            pub const Disabled: u32 = 0b1;
        }
    }

    /// nBOOT0 option bit
    pub mod nBOOT0 {
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

            /// 0b0: nBOOT0 = 0
            pub const Disabled: u32 = 0b0;

            /// 0b1: nBOOT0 = 1
            pub const Enabled: u32 = 0b1;
        }
    }

    /// nSWBOOT0 option bit
    pub mod nSWBOOT0 {
        /// Offset (26 bits)
        pub const offset: u32 = 26;
        /// Mask (1 bit: 1 << 26)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: BOOT0 taken from the option bit nBOOT0
            pub const OptionBit: u32 = 0b0;

            /// 0b1: BOOT0 taken from PH3/BOOT0 pin
            pub const Pin: u32 = 0b1;
        }
    }

    ///
    pub mod DBANK {
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

            /// 0b0: Single-bank mode with 128 bits data read width
            pub const SingleBankMode: u32 = 0b0;

            /// 0b1: Dual-bank mode with 64 bits data
            pub const DualBankMode: u32 = 0b1;
        }
    }

    ///
    pub mod DB1M {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (1 bit: 1 << 21)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Single Flash contiguous address in Bank 1
            pub const SingleBank: u32 = 0b0;

            /// 0b1: Dual-bank Flash with contiguous addresses
            pub const DualBank: u32 = 0b1;
        }
    }
}

/// Flash Bank 1 PCROP Start address register
pub mod PCROP1SR {

    /// Bank 1 PCROP area start offset
    pub mod PCROP1_STRT {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (17 bits: 0x1ffff << 0)
        pub const mask: u32 = 0x1ffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Flash Bank 1 PCROP End address register
pub mod PCROP1ER {

    /// Bank 1 PCROP area end offset
    pub mod PCROP1_END {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (17 bits: 0x1ffff << 0)
        pub const mask: u32 = 0x1ffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// PCROP area preserved when RDP level decreased
    pub mod PCROP_RDP {
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

            /// 0b0: PCROP area is not erased when the RDP level is decreased from Level 1 to Level 0
            pub const Disabled: u32 = 0b0;

            /// 0b1: PCROP area is erased when the RDP level is decreased from Level 1 to Level 0
            pub const Enabled: u32 = 0b1;
        }
    }
}

/// Flash Bank 1 WRP area A address register
pub mod WRP1AR {

    /// Bank 1 WRP first area start offset
    pub mod WRP1A_STRT {
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

    /// Bank 1 WRP first area A end offset
    pub mod WRP1A_END {
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

/// Flash Bank 1 WRP area B address register
pub mod WRP1BR {

    /// Bank 1 WRP second area B end offset
    pub mod WRP1B_END {
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

    /// Bank 1 WRP second area B start offset
    pub mod WRP1B_STRT {
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

/// Flash Bank 2 PCROP Start address register
pub mod PCROP2SR {

    /// Bank 2 PCROP area start offset
    pub mod PCROP2_STRT {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (17 bits: 0x1ffff << 0)
        pub const mask: u32 = 0x1ffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Flash Bank 2 PCROP End address register
pub mod PCROP2ER {

    /// Bank 2 PCROP area end offset
    pub mod PCROP2_END {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (17 bits: 0x1ffff << 0)
        pub const mask: u32 = 0x1ffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Flash Bank 2 WRP area A address register
pub mod WRP2AR {

    /// Bank 2 WRP first area A start offset
    pub mod WRP2A_STRT {
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

    /// Bank 2 WRP first area A end offset
    pub mod WRP2A_END {
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

/// Flash Bank 2 WRP area B address register
pub mod WRP2BR {

    /// Bank 2 WRP second area B start offset
    pub mod WRP2B_STRT {
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

    /// Bank 2 WRP second area B end offset
    pub mod WRP2B_END {
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

/// flash configuration register
pub mod CFGR {

    /// Low voltage enable
    pub mod LVEN {
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

            /// 0b0: Flash low voltage disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Flash low voltage enabled
            pub const Enabled: u32 = 0b1;
        }
    }
}
#[repr(C)]
pub struct RegisterBlock {
    /// Access control register
    pub ACR: RWRegister<u32>,

    /// Power down key register
    pub PDKEYR: WORegister<u32>,

    /// Flash key register
    pub KEYR: WORegister<u32>,

    /// Option byte key register
    pub OPTKEYR: WORegister<u32>,

    /// Status register
    pub SR: RWRegister<u32>,

    /// Flash control register
    pub CR: RWRegister<u32>,

    /// Flash ECC register
    pub ECCR: RWRegister<u32>,

    _reserved1: [u8; 4],

    /// Flash option register
    pub OPTR: RWRegister<u32>,

    /// Flash Bank 1 PCROP Start address register
    pub PCROP1SR: RWRegister<u32>,

    /// Flash Bank 1 PCROP End address register
    pub PCROP1ER: RWRegister<u32>,

    /// Flash Bank 1 WRP area A address register
    pub WRP1AR: RWRegister<u32>,

    /// Flash Bank 2 WRP area A address register
    pub WRP2AR: RWRegister<u32>,

    _reserved2: [u8; 16],

    /// Flash Bank 2 PCROP Start address register
    pub PCROP2SR: RWRegister<u32>,

    /// Flash Bank 2 PCROP End address register
    pub PCROP2ER: RWRegister<u32>,

    /// Flash Bank 1 WRP area B address register
    pub WRP1BR: RWRegister<u32>,

    /// Flash Bank 2 WRP area B address register
    pub WRP2BR: RWRegister<u32>,

    _reserved3: [u8; 220],

    /// flash configuration register
    pub CFGR: RWRegister<u32>,
}
pub struct ResetValues {
    pub ACR: u32,
    pub PDKEYR: u32,
    pub KEYR: u32,
    pub OPTKEYR: u32,
    pub SR: u32,
    pub CR: u32,
    pub ECCR: u32,
    pub OPTR: u32,
    pub PCROP1SR: u32,
    pub PCROP1ER: u32,
    pub WRP1AR: u32,
    pub WRP2AR: u32,
    pub PCROP2SR: u32,
    pub PCROP2ER: u32,
    pub WRP1BR: u32,
    pub WRP2BR: u32,
    pub CFGR: u32,
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

/// Access functions for the FLASH peripheral instance
pub mod FLASH {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x40022000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in FLASH
    pub const reset: ResetValues = ResetValues {
        ACR: 0x00000600,
        PDKEYR: 0x00000000,
        KEYR: 0x00000000,
        OPTKEYR: 0x00000000,
        SR: 0x00000000,
        CR: 0xC0000000,
        ECCR: 0x00000000,
        OPTR: 0xFFEFF8AA,
        PCROP1SR: 0xFFFF0000,
        PCROP1ER: 0x0FFF0000,
        WRP1AR: 0xFF00FF00,
        WRP1BR: 0xFF00FF00,
        PCROP2SR: 0xFFFF0000,
        PCROP2ER: 0xFFFF0000,
        WRP2AR: 0xFF00FF00,
        WRP2BR: 0xFF00FF00,
        CFGR: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut FLASH_TAKEN: bool = false;

    /// Safe access to FLASH
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
            if FLASH_TAKEN {
                None
            } else {
                FLASH_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to FLASH
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if FLASH_TAKEN && inst.addr == INSTANCE.addr {
                FLASH_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal FLASH
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        FLASH_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to FLASH
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const FLASH: *const RegisterBlock = 0x40022000 as *const _;
