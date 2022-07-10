#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! HSEM

use crate::{RORegister, RWRegister};
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// HSEM register HSEM_R%s HSEM_R31
pub mod R0 {

    /// Semaphore ProcessID
    pub mod PROCID {
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

    /// Semaphore MasterID
    pub mod MASTERID {
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

    /// Lock indication
    pub mod LOCK {
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

/// HSEM register HSEM_R%s HSEM_R31
pub mod R1 {
    pub use super::R0::LOCK;
    pub use super::R0::MASTERID;
    pub use super::R0::PROCID;
}

/// HSEM register HSEM_R%s HSEM_R31
pub mod R2 {
    pub use super::R0::LOCK;
    pub use super::R0::MASTERID;
    pub use super::R0::PROCID;
}

/// HSEM register HSEM_R%s HSEM_R31
pub mod R3 {
    pub use super::R0::LOCK;
    pub use super::R0::MASTERID;
    pub use super::R0::PROCID;
}

/// HSEM register HSEM_R%s HSEM_R31
pub mod R4 {
    pub use super::R0::LOCK;
    pub use super::R0::MASTERID;
    pub use super::R0::PROCID;
}

/// HSEM register HSEM_R%s HSEM_R31
pub mod R5 {
    pub use super::R0::LOCK;
    pub use super::R0::MASTERID;
    pub use super::R0::PROCID;
}

/// HSEM register HSEM_R%s HSEM_R31
pub mod R6 {
    pub use super::R0::LOCK;
    pub use super::R0::MASTERID;
    pub use super::R0::PROCID;
}

/// HSEM register HSEM_R%s HSEM_R31
pub mod R7 {
    pub use super::R0::LOCK;
    pub use super::R0::MASTERID;
    pub use super::R0::PROCID;
}

/// HSEM register HSEM_R%s HSEM_R31
pub mod R8 {
    pub use super::R0::LOCK;
    pub use super::R0::MASTERID;
    pub use super::R0::PROCID;
}

/// HSEM register HSEM_R%s HSEM_R31
pub mod R9 {
    pub use super::R0::LOCK;
    pub use super::R0::MASTERID;
    pub use super::R0::PROCID;
}

/// HSEM register HSEM_R%s HSEM_R31
pub mod R10 {
    pub use super::R0::LOCK;
    pub use super::R0::MASTERID;
    pub use super::R0::PROCID;
}

/// HSEM register HSEM_R%s HSEM_R31
pub mod R11 {
    pub use super::R0::LOCK;
    pub use super::R0::MASTERID;
    pub use super::R0::PROCID;
}

/// HSEM register HSEM_R%s HSEM_R31
pub mod R12 {
    pub use super::R0::LOCK;
    pub use super::R0::MASTERID;
    pub use super::R0::PROCID;
}

/// HSEM register HSEM_R%s HSEM_R31
pub mod R13 {
    pub use super::R0::LOCK;
    pub use super::R0::MASTERID;
    pub use super::R0::PROCID;
}

/// HSEM register HSEM_R%s HSEM_R31
pub mod R14 {
    pub use super::R0::LOCK;
    pub use super::R0::MASTERID;
    pub use super::R0::PROCID;
}

/// HSEM register HSEM_R%s HSEM_R31
pub mod R15 {
    pub use super::R0::LOCK;
    pub use super::R0::MASTERID;
    pub use super::R0::PROCID;
}

/// HSEM register HSEM_R%s HSEM_R31
pub mod R16 {
    pub use super::R0::LOCK;
    pub use super::R0::MASTERID;
    pub use super::R0::PROCID;
}

/// HSEM register HSEM_R%s HSEM_R31
pub mod R17 {
    pub use super::R0::LOCK;
    pub use super::R0::MASTERID;
    pub use super::R0::PROCID;
}

/// HSEM register HSEM_R%s HSEM_R31
pub mod R18 {
    pub use super::R0::LOCK;
    pub use super::R0::MASTERID;
    pub use super::R0::PROCID;
}

/// HSEM register HSEM_R%s HSEM_R31
pub mod R19 {
    pub use super::R0::LOCK;
    pub use super::R0::MASTERID;
    pub use super::R0::PROCID;
}

/// HSEM register HSEM_R%s HSEM_R31
pub mod R20 {
    pub use super::R0::LOCK;
    pub use super::R0::MASTERID;
    pub use super::R0::PROCID;
}

/// HSEM register HSEM_R%s HSEM_R31
pub mod R21 {
    pub use super::R0::LOCK;
    pub use super::R0::MASTERID;
    pub use super::R0::PROCID;
}

/// HSEM register HSEM_R%s HSEM_R31
pub mod R22 {
    pub use super::R0::LOCK;
    pub use super::R0::MASTERID;
    pub use super::R0::PROCID;
}

/// HSEM register HSEM_R%s HSEM_R31
pub mod R23 {
    pub use super::R0::LOCK;
    pub use super::R0::MASTERID;
    pub use super::R0::PROCID;
}

/// HSEM register HSEM_R%s HSEM_R31
pub mod R24 {
    pub use super::R0::LOCK;
    pub use super::R0::MASTERID;
    pub use super::R0::PROCID;
}

/// HSEM register HSEM_R%s HSEM_R31
pub mod R25 {
    pub use super::R0::LOCK;
    pub use super::R0::MASTERID;
    pub use super::R0::PROCID;
}

/// HSEM register HSEM_R%s HSEM_R31
pub mod R26 {
    pub use super::R0::LOCK;
    pub use super::R0::MASTERID;
    pub use super::R0::PROCID;
}

/// HSEM register HSEM_R%s HSEM_R31
pub mod R27 {
    pub use super::R0::LOCK;
    pub use super::R0::MASTERID;
    pub use super::R0::PROCID;
}

/// HSEM register HSEM_R%s HSEM_R31
pub mod R28 {
    pub use super::R0::LOCK;
    pub use super::R0::MASTERID;
    pub use super::R0::PROCID;
}

/// HSEM register HSEM_R%s HSEM_R31
pub mod R29 {
    pub use super::R0::LOCK;
    pub use super::R0::MASTERID;
    pub use super::R0::PROCID;
}

/// HSEM register HSEM_R%s HSEM_R31
pub mod R30 {
    pub use super::R0::LOCK;
    pub use super::R0::MASTERID;
    pub use super::R0::PROCID;
}

/// HSEM register HSEM_R%s HSEM_R31
pub mod R31 {
    pub use super::R0::LOCK;
    pub use super::R0::MASTERID;
    pub use super::R0::PROCID;
}

/// HSEM Read lock register
pub mod RLR0 {

    /// Semaphore ProcessID
    pub mod PROCID {
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

    /// Semaphore MasterID
    pub mod MASTERID {
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

    /// Lock indication
    pub mod LOCK {
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

/// HSEM Read lock register
pub mod RLR1 {
    pub use super::RLR0::LOCK;
    pub use super::RLR0::MASTERID;
    pub use super::RLR0::PROCID;
}

/// HSEM Read lock register
pub mod RLR2 {
    pub use super::RLR0::LOCK;
    pub use super::RLR0::MASTERID;
    pub use super::RLR0::PROCID;
}

/// HSEM Read lock register
pub mod RLR3 {
    pub use super::RLR0::LOCK;
    pub use super::RLR0::MASTERID;
    pub use super::RLR0::PROCID;
}

/// HSEM Read lock register
pub mod RLR4 {
    pub use super::RLR0::LOCK;
    pub use super::RLR0::MASTERID;
    pub use super::RLR0::PROCID;
}

/// HSEM Read lock register
pub mod RLR5 {
    pub use super::RLR0::LOCK;
    pub use super::RLR0::MASTERID;
    pub use super::RLR0::PROCID;
}

/// HSEM Read lock register
pub mod RLR6 {
    pub use super::RLR0::LOCK;
    pub use super::RLR0::MASTERID;
    pub use super::RLR0::PROCID;
}

/// HSEM Read lock register
pub mod RLR7 {
    pub use super::RLR0::LOCK;
    pub use super::RLR0::MASTERID;
    pub use super::RLR0::PROCID;
}

/// HSEM Read lock register
pub mod RLR8 {
    pub use super::RLR0::LOCK;
    pub use super::RLR0::MASTERID;
    pub use super::RLR0::PROCID;
}

/// HSEM Read lock register
pub mod RLR9 {
    pub use super::RLR0::LOCK;
    pub use super::RLR0::MASTERID;
    pub use super::RLR0::PROCID;
}

/// HSEM Read lock register
pub mod RLR10 {
    pub use super::RLR0::LOCK;
    pub use super::RLR0::MASTERID;
    pub use super::RLR0::PROCID;
}

/// HSEM Read lock register
pub mod RLR11 {
    pub use super::RLR0::LOCK;
    pub use super::RLR0::MASTERID;
    pub use super::RLR0::PROCID;
}

/// HSEM Read lock register
pub mod RLR12 {
    pub use super::RLR0::LOCK;
    pub use super::RLR0::MASTERID;
    pub use super::RLR0::PROCID;
}

/// HSEM Read lock register
pub mod RLR13 {
    pub use super::RLR0::LOCK;
    pub use super::RLR0::MASTERID;
    pub use super::RLR0::PROCID;
}

/// HSEM Read lock register
pub mod RLR14 {
    pub use super::RLR0::LOCK;
    pub use super::RLR0::MASTERID;
    pub use super::RLR0::PROCID;
}

/// HSEM Read lock register
pub mod RLR15 {
    pub use super::RLR0::LOCK;
    pub use super::RLR0::MASTERID;
    pub use super::RLR0::PROCID;
}

/// HSEM Read lock register
pub mod RLR16 {
    pub use super::RLR0::LOCK;
    pub use super::RLR0::MASTERID;
    pub use super::RLR0::PROCID;
}

/// HSEM Read lock register
pub mod RLR17 {
    pub use super::RLR0::LOCK;
    pub use super::RLR0::MASTERID;
    pub use super::RLR0::PROCID;
}

/// HSEM Read lock register
pub mod RLR18 {
    pub use super::RLR0::LOCK;
    pub use super::RLR0::MASTERID;
    pub use super::RLR0::PROCID;
}

/// HSEM Read lock register
pub mod RLR19 {
    pub use super::RLR0::LOCK;
    pub use super::RLR0::MASTERID;
    pub use super::RLR0::PROCID;
}

/// HSEM Read lock register
pub mod RLR20 {
    pub use super::RLR0::LOCK;
    pub use super::RLR0::MASTERID;
    pub use super::RLR0::PROCID;
}

/// HSEM Read lock register
pub mod RLR21 {
    pub use super::RLR0::LOCK;
    pub use super::RLR0::MASTERID;
    pub use super::RLR0::PROCID;
}

/// HSEM Read lock register
pub mod RLR22 {
    pub use super::RLR0::LOCK;
    pub use super::RLR0::MASTERID;
    pub use super::RLR0::PROCID;
}

/// HSEM Read lock register
pub mod RLR23 {
    pub use super::RLR0::LOCK;
    pub use super::RLR0::MASTERID;
    pub use super::RLR0::PROCID;
}

/// HSEM Read lock register
pub mod RLR24 {
    pub use super::RLR0::LOCK;
    pub use super::RLR0::MASTERID;
    pub use super::RLR0::PROCID;
}

/// HSEM Read lock register
pub mod RLR25 {
    pub use super::RLR0::LOCK;
    pub use super::RLR0::MASTERID;
    pub use super::RLR0::PROCID;
}

/// HSEM Read lock register
pub mod RLR26 {
    pub use super::RLR0::LOCK;
    pub use super::RLR0::MASTERID;
    pub use super::RLR0::PROCID;
}

/// HSEM Read lock register
pub mod RLR27 {
    pub use super::RLR0::LOCK;
    pub use super::RLR0::MASTERID;
    pub use super::RLR0::PROCID;
}

/// HSEM Read lock register
pub mod RLR28 {
    pub use super::RLR0::LOCK;
    pub use super::RLR0::MASTERID;
    pub use super::RLR0::PROCID;
}

/// HSEM Read lock register
pub mod RLR29 {
    pub use super::RLR0::LOCK;
    pub use super::RLR0::MASTERID;
    pub use super::RLR0::PROCID;
}

/// HSEM Read lock register
pub mod RLR30 {
    pub use super::RLR0::LOCK;
    pub use super::RLR0::MASTERID;
    pub use super::RLR0::PROCID;
}

/// HSEM Read lock register
pub mod RLR31 {
    pub use super::RLR0::LOCK;
    pub use super::RLR0::MASTERID;
    pub use super::RLR0::PROCID;
}

/// HSEM Interrupt enable register
pub mod C1IER {

    /// Interrupt semaphore n enable bit
    pub mod ISEM0 {
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

    /// Interrupt semaphore n enable bit
    pub mod ISEM1 {
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

    /// Interrupt semaphore n enable bit
    pub mod ISEM2 {
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

    /// Interrupt semaphore n enable bit
    pub mod ISEM3 {
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

    /// Interrupt semaphore n enable bit
    pub mod ISEM4 {
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

    /// Interrupt semaphore n enable bit
    pub mod ISEM5 {
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

    /// Interrupt semaphore n enable bit
    pub mod ISEM6 {
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

    /// Interrupt semaphore n enable bit
    pub mod ISEM7 {
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

    /// Interrupt semaphore n enable bit
    pub mod ISEM8 {
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

    /// Interrupt semaphore n enable bit
    pub mod ISEM9 {
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

    /// Interrupt semaphore n enable bit
    pub mod ISEM10 {
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

    /// Interrupt semaphore n enable bit
    pub mod ISEM11 {
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

    /// Interrupt semaphore n enable bit
    pub mod ISEM12 {
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

    /// Interrupt semaphore n enable bit
    pub mod ISEM13 {
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

    /// Interrupt semaphore n enable bit
    pub mod ISEM14 {
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

    /// Interrupt semaphore n enable bit
    pub mod ISEM15 {
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

    /// Interrupt semaphore n enable bit
    pub mod ISEM16 {
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

    /// Interrupt semaphore n enable bit
    pub mod ISEM17 {
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

    /// Interrupt semaphore n enable bit
    pub mod ISEM18 {
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

    /// Interrupt semaphore n enable bit
    pub mod ISEM19 {
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

    /// Interrupt semaphore n enable bit
    pub mod ISEM20 {
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

    /// Interrupt semaphore n enable bit
    pub mod ISEM21 {
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

    /// Interrupt semaphore n enable bit
    pub mod ISEM22 {
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

    /// Interrupt semaphore n enable bit
    pub mod ISEM23 {
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

    /// Interrupt semaphore n enable bit
    pub mod ISEM24 {
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

    /// Interrupt semaphore n enable bit
    pub mod ISEM25 {
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

    /// Interrupt semaphore n enable bit
    pub mod ISEM26 {
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

    /// Interrupt semaphore n enable bit
    pub mod ISEM27 {
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

    /// Interrupt semaphore n enable bit
    pub mod ISEM28 {
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

    /// Interrupt semaphore n enable bit
    pub mod ISEM29 {
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

    /// Interrupt semaphore n enable bit
    pub mod ISEM30 {
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

    /// Interrupt(N) semaphore n enable bit.
    pub mod ISEM31 {
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

/// HSEM Interrupt clear register
pub mod C1ICR {

    /// Interrupt(N) semaphore n clear bit
    pub mod ISEM0 {
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

    /// Interrupt(N) semaphore n clear bit
    pub mod ISEM1 {
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

    /// Interrupt(N) semaphore n clear bit
    pub mod ISEM2 {
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

    /// Interrupt(N) semaphore n clear bit
    pub mod ISEM3 {
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

    /// Interrupt(N) semaphore n clear bit
    pub mod ISEM4 {
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

    /// Interrupt(N) semaphore n clear bit
    pub mod ISEM5 {
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

    /// Interrupt(N) semaphore n clear bit
    pub mod ISEM6 {
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

    /// Interrupt(N) semaphore n clear bit
    pub mod ISEM7 {
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

    /// Interrupt(N) semaphore n clear bit
    pub mod ISEM8 {
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

    /// Interrupt(N) semaphore n clear bit
    pub mod ISEM9 {
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

    /// Interrupt(N) semaphore n clear bit
    pub mod ISEM10 {
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

    /// Interrupt(N) semaphore n clear bit
    pub mod ISEM11 {
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

    /// Interrupt(N) semaphore n clear bit
    pub mod ISEM12 {
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

    /// Interrupt(N) semaphore n clear bit
    pub mod ISEM13 {
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

    /// Interrupt(N) semaphore n clear bit
    pub mod ISEM14 {
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

    /// Interrupt(N) semaphore n clear bit
    pub mod ISEM15 {
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

    /// Interrupt(N) semaphore n clear bit
    pub mod ISEM16 {
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

    /// Interrupt(N) semaphore n clear bit
    pub mod ISEM17 {
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

    /// Interrupt(N) semaphore n clear bit
    pub mod ISEM18 {
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

    /// Interrupt(N) semaphore n clear bit
    pub mod ISEM19 {
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

    /// Interrupt(N) semaphore n clear bit
    pub mod ISEM20 {
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

    /// Interrupt(N) semaphore n clear bit
    pub mod ISEM21 {
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

    /// Interrupt(N) semaphore n clear bit
    pub mod ISEM22 {
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

    /// Interrupt(N) semaphore n clear bit
    pub mod ISEM23 {
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

    /// Interrupt(N) semaphore n clear bit
    pub mod ISEM24 {
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

    /// Interrupt(N) semaphore n clear bit
    pub mod ISEM25 {
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

    /// Interrupt(N) semaphore n clear bit
    pub mod ISEM26 {
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

    /// Interrupt(N) semaphore n clear bit
    pub mod ISEM27 {
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

    /// Interrupt(N) semaphore n clear bit
    pub mod ISEM28 {
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

    /// Interrupt(N) semaphore n clear bit
    pub mod ISEM29 {
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

    /// Interrupt(N) semaphore n clear bit
    pub mod ISEM30 {
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

    /// Interrupt(N) semaphore n clear bit
    pub mod ISEM31 {
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

/// HSEM Interrupt status register
pub mod C1ISR {
    pub use super::C1ICR::ISEM0;
    pub use super::C1ICR::ISEM1;
    pub use super::C1ICR::ISEM10;
    pub use super::C1ICR::ISEM11;
    pub use super::C1ICR::ISEM12;
    pub use super::C1ICR::ISEM13;
    pub use super::C1ICR::ISEM14;
    pub use super::C1ICR::ISEM15;
    pub use super::C1ICR::ISEM16;
    pub use super::C1ICR::ISEM17;
    pub use super::C1ICR::ISEM18;
    pub use super::C1ICR::ISEM19;
    pub use super::C1ICR::ISEM2;
    pub use super::C1ICR::ISEM20;
    pub use super::C1ICR::ISEM21;
    pub use super::C1ICR::ISEM22;
    pub use super::C1ICR::ISEM23;
    pub use super::C1ICR::ISEM24;
    pub use super::C1ICR::ISEM25;
    pub use super::C1ICR::ISEM26;
    pub use super::C1ICR::ISEM27;
    pub use super::C1ICR::ISEM28;
    pub use super::C1ICR::ISEM29;
    pub use super::C1ICR::ISEM3;
    pub use super::C1ICR::ISEM30;
    pub use super::C1ICR::ISEM31;
    pub use super::C1ICR::ISEM4;
    pub use super::C1ICR::ISEM5;
    pub use super::C1ICR::ISEM6;
    pub use super::C1ICR::ISEM7;
    pub use super::C1ICR::ISEM8;
    pub use super::C1ICR::ISEM9;
}

/// HSEM Masked interrupt status register
pub mod C1MISR {
    pub use super::C1ICR::ISEM0;
    pub use super::C1ICR::ISEM1;
    pub use super::C1ICR::ISEM10;
    pub use super::C1ICR::ISEM11;
    pub use super::C1ICR::ISEM12;
    pub use super::C1ICR::ISEM13;
    pub use super::C1ICR::ISEM14;
    pub use super::C1ICR::ISEM15;
    pub use super::C1ICR::ISEM16;
    pub use super::C1ICR::ISEM17;
    pub use super::C1ICR::ISEM18;
    pub use super::C1ICR::ISEM19;
    pub use super::C1ICR::ISEM2;
    pub use super::C1ICR::ISEM20;
    pub use super::C1ICR::ISEM21;
    pub use super::C1ICR::ISEM22;
    pub use super::C1ICR::ISEM23;
    pub use super::C1ICR::ISEM24;
    pub use super::C1ICR::ISEM25;
    pub use super::C1ICR::ISEM26;
    pub use super::C1ICR::ISEM27;
    pub use super::C1ICR::ISEM28;
    pub use super::C1ICR::ISEM29;
    pub use super::C1ICR::ISEM3;
    pub use super::C1ICR::ISEM30;
    pub use super::C1ICR::ISEM31;
    pub use super::C1ICR::ISEM4;
    pub use super::C1ICR::ISEM5;
    pub use super::C1ICR::ISEM6;
    pub use super::C1ICR::ISEM7;
    pub use super::C1ICR::ISEM8;
    pub use super::C1ICR::ISEM9;
}

/// HSEM Clear register
pub mod CR {

    /// MasterID of semaphores to be cleared
    pub mod COREID {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (4 bits: 0b1111 << 8)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Semaphore clear Key
    pub mod KEY {
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

/// HSEM Interrupt clear register
pub mod KEYR {

    /// Semaphore Clear Key
    pub mod KEY {
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
#[repr(C)]
pub struct RegisterBlock {
    /// HSEM register HSEM_R%s HSEM_R31
    pub R0: RWRegister<u32>,

    /// HSEM register HSEM_R%s HSEM_R31
    pub R1: RWRegister<u32>,

    /// HSEM register HSEM_R%s HSEM_R31
    pub R2: RWRegister<u32>,

    /// HSEM register HSEM_R%s HSEM_R31
    pub R3: RWRegister<u32>,

    /// HSEM register HSEM_R%s HSEM_R31
    pub R4: RWRegister<u32>,

    /// HSEM register HSEM_R%s HSEM_R31
    pub R5: RWRegister<u32>,

    /// HSEM register HSEM_R%s HSEM_R31
    pub R6: RWRegister<u32>,

    /// HSEM register HSEM_R%s HSEM_R31
    pub R7: RWRegister<u32>,

    /// HSEM register HSEM_R%s HSEM_R31
    pub R8: RWRegister<u32>,

    /// HSEM register HSEM_R%s HSEM_R31
    pub R9: RWRegister<u32>,

    /// HSEM register HSEM_R%s HSEM_R31
    pub R10: RWRegister<u32>,

    /// HSEM register HSEM_R%s HSEM_R31
    pub R11: RWRegister<u32>,

    /// HSEM register HSEM_R%s HSEM_R31
    pub R12: RWRegister<u32>,

    /// HSEM register HSEM_R%s HSEM_R31
    pub R13: RWRegister<u32>,

    /// HSEM register HSEM_R%s HSEM_R31
    pub R14: RWRegister<u32>,

    /// HSEM register HSEM_R%s HSEM_R31
    pub R15: RWRegister<u32>,

    /// HSEM register HSEM_R%s HSEM_R31
    pub R16: RWRegister<u32>,

    /// HSEM register HSEM_R%s HSEM_R31
    pub R17: RWRegister<u32>,

    /// HSEM register HSEM_R%s HSEM_R31
    pub R18: RWRegister<u32>,

    /// HSEM register HSEM_R%s HSEM_R31
    pub R19: RWRegister<u32>,

    /// HSEM register HSEM_R%s HSEM_R31
    pub R20: RWRegister<u32>,

    /// HSEM register HSEM_R%s HSEM_R31
    pub R21: RWRegister<u32>,

    /// HSEM register HSEM_R%s HSEM_R31
    pub R22: RWRegister<u32>,

    /// HSEM register HSEM_R%s HSEM_R31
    pub R23: RWRegister<u32>,

    /// HSEM register HSEM_R%s HSEM_R31
    pub R24: RWRegister<u32>,

    /// HSEM register HSEM_R%s HSEM_R31
    pub R25: RWRegister<u32>,

    /// HSEM register HSEM_R%s HSEM_R31
    pub R26: RWRegister<u32>,

    /// HSEM register HSEM_R%s HSEM_R31
    pub R27: RWRegister<u32>,

    /// HSEM register HSEM_R%s HSEM_R31
    pub R28: RWRegister<u32>,

    /// HSEM register HSEM_R%s HSEM_R31
    pub R29: RWRegister<u32>,

    /// HSEM register HSEM_R%s HSEM_R31
    pub R30: RWRegister<u32>,

    /// HSEM register HSEM_R%s HSEM_R31
    pub R31: RWRegister<u32>,

    /// HSEM Read lock register
    pub RLR0: RORegister<u32>,

    /// HSEM Read lock register
    pub RLR1: RORegister<u32>,

    /// HSEM Read lock register
    pub RLR2: RORegister<u32>,

    /// HSEM Read lock register
    pub RLR3: RORegister<u32>,

    /// HSEM Read lock register
    pub RLR4: RORegister<u32>,

    /// HSEM Read lock register
    pub RLR5: RORegister<u32>,

    /// HSEM Read lock register
    pub RLR6: RORegister<u32>,

    /// HSEM Read lock register
    pub RLR7: RORegister<u32>,

    /// HSEM Read lock register
    pub RLR8: RORegister<u32>,

    /// HSEM Read lock register
    pub RLR9: RORegister<u32>,

    /// HSEM Read lock register
    pub RLR10: RORegister<u32>,

    /// HSEM Read lock register
    pub RLR11: RORegister<u32>,

    /// HSEM Read lock register
    pub RLR12: RORegister<u32>,

    /// HSEM Read lock register
    pub RLR13: RORegister<u32>,

    /// HSEM Read lock register
    pub RLR14: RORegister<u32>,

    /// HSEM Read lock register
    pub RLR15: RORegister<u32>,

    /// HSEM Read lock register
    pub RLR16: RORegister<u32>,

    /// HSEM Read lock register
    pub RLR17: RORegister<u32>,

    /// HSEM Read lock register
    pub RLR18: RORegister<u32>,

    /// HSEM Read lock register
    pub RLR19: RORegister<u32>,

    /// HSEM Read lock register
    pub RLR20: RORegister<u32>,

    /// HSEM Read lock register
    pub RLR21: RORegister<u32>,

    /// HSEM Read lock register
    pub RLR22: RORegister<u32>,

    /// HSEM Read lock register
    pub RLR23: RORegister<u32>,

    /// HSEM Read lock register
    pub RLR24: RORegister<u32>,

    /// HSEM Read lock register
    pub RLR25: RORegister<u32>,

    /// HSEM Read lock register
    pub RLR26: RORegister<u32>,

    /// HSEM Read lock register
    pub RLR27: RORegister<u32>,

    /// HSEM Read lock register
    pub RLR28: RORegister<u32>,

    /// HSEM Read lock register
    pub RLR29: RORegister<u32>,

    /// HSEM Read lock register
    pub RLR30: RORegister<u32>,

    /// HSEM Read lock register
    pub RLR31: RORegister<u32>,

    /// HSEM Interrupt enable register
    pub C1IER: RWRegister<u32>,

    /// HSEM Interrupt clear register
    pub C1ICR: RORegister<u32>,

    /// HSEM Interrupt status register
    pub C1ISR: RORegister<u32>,

    /// HSEM Masked interrupt status register
    pub C1MISR: RORegister<u32>,

    _reserved1: [u8; 48],

    /// HSEM Clear register
    pub CR: RWRegister<u32>,

    /// HSEM Interrupt clear register
    pub KEYR: RWRegister<u32>,
}
pub struct ResetValues {
    pub R0: u32,
    pub R1: u32,
    pub R2: u32,
    pub R3: u32,
    pub R4: u32,
    pub R5: u32,
    pub R6: u32,
    pub R7: u32,
    pub R8: u32,
    pub R9: u32,
    pub R10: u32,
    pub R11: u32,
    pub R12: u32,
    pub R13: u32,
    pub R14: u32,
    pub R15: u32,
    pub R16: u32,
    pub R17: u32,
    pub R18: u32,
    pub R19: u32,
    pub R20: u32,
    pub R21: u32,
    pub R22: u32,
    pub R23: u32,
    pub R24: u32,
    pub R25: u32,
    pub R26: u32,
    pub R27: u32,
    pub R28: u32,
    pub R29: u32,
    pub R30: u32,
    pub R31: u32,
    pub RLR0: u32,
    pub RLR1: u32,
    pub RLR2: u32,
    pub RLR3: u32,
    pub RLR4: u32,
    pub RLR5: u32,
    pub RLR6: u32,
    pub RLR7: u32,
    pub RLR8: u32,
    pub RLR9: u32,
    pub RLR10: u32,
    pub RLR11: u32,
    pub RLR12: u32,
    pub RLR13: u32,
    pub RLR14: u32,
    pub RLR15: u32,
    pub RLR16: u32,
    pub RLR17: u32,
    pub RLR18: u32,
    pub RLR19: u32,
    pub RLR20: u32,
    pub RLR21: u32,
    pub RLR22: u32,
    pub RLR23: u32,
    pub RLR24: u32,
    pub RLR25: u32,
    pub RLR26: u32,
    pub RLR27: u32,
    pub RLR28: u32,
    pub RLR29: u32,
    pub RLR30: u32,
    pub RLR31: u32,
    pub C1IER: u32,
    pub C1ICR: u32,
    pub C1ISR: u32,
    pub C1MISR: u32,
    pub CR: u32,
    pub KEYR: u32,
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

/// Access functions for the HSEM peripheral instance
pub mod HSEM {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x58026400,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in HSEM
    pub const reset: ResetValues = ResetValues {
        R0: 0x00000000,
        R1: 0x00000000,
        R2: 0x00000000,
        R3: 0x00000000,
        R4: 0x00000000,
        R5: 0x00000000,
        R6: 0x00000000,
        R7: 0x00000000,
        R8: 0x00000000,
        R9: 0x00000000,
        R10: 0x00000000,
        R11: 0x00000000,
        R12: 0x00000000,
        R13: 0x00000000,
        R14: 0x00000000,
        R15: 0x00000000,
        R16: 0x00000000,
        R17: 0x00000000,
        R18: 0x00000000,
        R19: 0x00000000,
        R20: 0x00000000,
        R21: 0x00000000,
        R22: 0x00000000,
        R23: 0x00000000,
        R24: 0x00000000,
        R25: 0x00000000,
        R26: 0x00000000,
        R27: 0x00000000,
        R28: 0x00000000,
        R29: 0x00000000,
        R30: 0x00000000,
        R31: 0x00000000,
        RLR0: 0x00000000,
        RLR1: 0x00000000,
        RLR2: 0x00000000,
        RLR3: 0x00000000,
        RLR4: 0x00000000,
        RLR5: 0x00000000,
        RLR6: 0x00000000,
        RLR7: 0x00000000,
        RLR8: 0x00000000,
        RLR9: 0x00000000,
        RLR10: 0x00000000,
        RLR11: 0x00000000,
        RLR12: 0x00000000,
        RLR13: 0x00000000,
        RLR14: 0x00000000,
        RLR15: 0x00000000,
        RLR16: 0x00000000,
        RLR17: 0x00000000,
        RLR18: 0x00000000,
        RLR19: 0x00000000,
        RLR20: 0x00000000,
        RLR21: 0x00000000,
        RLR22: 0x00000000,
        RLR23: 0x00000000,
        RLR24: 0x00000000,
        RLR25: 0x00000000,
        RLR26: 0x00000000,
        RLR27: 0x00000000,
        RLR28: 0x00000000,
        RLR29: 0x00000000,
        RLR30: 0x00000000,
        RLR31: 0x00000000,
        C1IER: 0x00000000,
        C1ICR: 0x00000000,
        C1ISR: 0x00000000,
        C1MISR: 0x00000000,
        CR: 0x00000000,
        KEYR: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut HSEM_TAKEN: bool = false;

    /// Safe access to HSEM
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
            if HSEM_TAKEN {
                None
            } else {
                HSEM_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to HSEM
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if HSEM_TAKEN && inst.addr == INSTANCE.addr {
                HSEM_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal HSEM
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        HSEM_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to HSEM
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const HSEM: *const RegisterBlock = 0x58026400 as *const _;
