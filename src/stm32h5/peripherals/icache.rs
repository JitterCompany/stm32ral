#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Instruction cache
//!
//! Used by: stm32h562, stm32h563, stm32h573

use crate::{RORegister, RWRegister, WORegister};
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// ICACHE control register
pub mod CR {

    /// enable
    pub mod EN {
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

    /// cache invalidation Set by software and cleared by hardware when the BUSYF flag is set (during cache maintenance operation). Writing 0 has no effect.
    pub mod CACHEINV {
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

    /// cache associativity mode selection This bit allows user to choose ICACHE set-associativity. It can be written by software only when cache is disabled (EN = 0).
    pub mod WAYSEL {
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

    /// hit monitor enable
    pub mod HITMEN {
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

    /// miss monitor enable
    pub mod MISSMEN {
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

    /// hit monitor reset
    pub mod HITMRST {
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

    /// miss monitor reset
    pub mod MISSMRST {
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
}

/// ICACHE status register
pub mod SR {

    /// busy flag
    pub mod BUSYF {
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

    /// busy end flag
    pub mod BSYENDF {
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

    /// cache error flag
    pub mod ERRF {
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
}

/// ICACHE interrupt enable register
pub mod IER {

    /// interrupt enable on busy end Set by software to enable an interrupt generation at the end of a cache invalidate operation.
    pub mod BSYENDIE {
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

    /// interrupt enable on cache error Set by software to enable an interrupt generation in case of cache functional error (cacheable write access)
    pub mod ERRIE {
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
}

/// ICACHE flag clear register
pub mod FCR {

    /// clear busy end flag Set by software.
    pub mod CBSYENDF {
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

    /// clear cache error flag Set by software.
    pub mod CERRF {
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
}

/// ICACHE hit monitor register
pub mod HMONR {

    /// cache hit monitor counter
    pub mod HITMON {
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

/// ICACHE miss monitor register
pub mod MMONR {

    /// cache miss monitor counter
    pub mod MISSMON {
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

/// ICACHE region 0 configuration register
pub mod CRR0 {

    /// base address for region x This alias address is replaced by REMAPADDR field. The only useful bits are \[28:RI\], where 21 ≤ RI ≤ 27 is the number of bits of RSIZE (see ). If the programmed value has more LSBs, the useless bits are ignored.
    pub mod BASEADDR {
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

    /// size for region x
    pub mod RSIZE {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (3 bits: 0b111 << 9)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// enable for region x
    pub mod REN {
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

    /// remapped address for region x This field replaces the alias address defined by BASEADDR field. The only useful bits are \[31:RI\], where 21 ≤ RI ≤ 27 is the number of bits of RSIZE (see ). If the programmed value has more LSBs, the useless bits are ignored.
    pub mod REMAPADDR {
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

    /// AHB cache master selection for region x
    pub mod MSTSEL {
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

    /// output burst type for region x
    pub mod HBURST {
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

/// ICACHE region 1 configuration register
pub mod CRR1 {
    pub use super::CRR0::BASEADDR;
    pub use super::CRR0::HBURST;
    pub use super::CRR0::MSTSEL;
    pub use super::CRR0::REMAPADDR;
    pub use super::CRR0::REN;
    pub use super::CRR0::RSIZE;
}

/// ICACHE region 2 configuration register
pub mod CRR2 {
    pub use super::CRR0::BASEADDR;
    pub use super::CRR0::HBURST;
    pub use super::CRR0::MSTSEL;
    pub use super::CRR0::REMAPADDR;
    pub use super::CRR0::REN;
    pub use super::CRR0::RSIZE;
}

/// ICACHE region 3 configuration register
pub mod CRR3 {
    pub use super::CRR0::BASEADDR;
    pub use super::CRR0::HBURST;
    pub use super::CRR0::MSTSEL;
    pub use super::CRR0::REMAPADDR;
    pub use super::CRR0::REN;
    pub use super::CRR0::RSIZE;
}
#[repr(C)]
pub struct RegisterBlock {
    /// ICACHE control register
    pub CR: RWRegister<u32>,

    /// ICACHE status register
    pub SR: RORegister<u32>,

    /// ICACHE interrupt enable register
    pub IER: RWRegister<u32>,

    /// ICACHE flag clear register
    pub FCR: WORegister<u32>,

    /// ICACHE hit monitor register
    pub HMONR: RORegister<u32>,

    /// ICACHE miss monitor register
    pub MMONR: RORegister<u32>,

    _reserved1: [u8; 8],

    /// ICACHE region 0 configuration register
    pub CRR0: RWRegister<u32>,

    /// ICACHE region 1 configuration register
    pub CRR1: RWRegister<u32>,

    /// ICACHE region 2 configuration register
    pub CRR2: RWRegister<u32>,

    /// ICACHE region 3 configuration register
    pub CRR3: RWRegister<u32>,
}
pub struct ResetValues {
    pub CR: u32,
    pub SR: u32,
    pub IER: u32,
    pub FCR: u32,
    pub HMONR: u32,
    pub MMONR: u32,
    pub CRR0: u32,
    pub CRR1: u32,
    pub CRR2: u32,
    pub CRR3: u32,
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
