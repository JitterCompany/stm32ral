#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Nested Vectored Interrupt Controller
//!
//! Used by: stm32h743, stm32h743v, stm32h753, stm32h753v

use crate::{RORegister, RWRegister};
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// Interrupt Set-Enable Register
pub mod ISER0 {

    /// SETENA
    pub mod SETENA {
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

/// Interrupt Set-Enable Register
pub mod ISER1 {
    pub use super::ISER0::SETENA;
}

/// Interrupt Set-Enable Register
pub mod ISER2 {
    pub use super::ISER0::SETENA;
}

/// Interrupt Clear-Enable Register
pub mod ICER0 {

    /// CLRENA
    pub mod CLRENA {
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

/// Interrupt Clear-Enable Register
pub mod ICER1 {
    pub use super::ICER0::CLRENA;
}

/// Interrupt Clear-Enable Register
pub mod ICER2 {
    pub use super::ICER0::CLRENA;
}

/// Interrupt Set-Pending Register
pub mod ISPR0 {

    /// SETPEND
    pub mod SETPEND {
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

/// Interrupt Set-Pending Register
pub mod ISPR1 {
    pub use super::ISPR0::SETPEND;
}

/// Interrupt Set-Pending Register
pub mod ISPR2 {
    pub use super::ISPR0::SETPEND;
}

/// Interrupt Clear-Pending Register
pub mod ICPR0 {

    /// CLRPEND
    pub mod CLRPEND {
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

/// Interrupt Clear-Pending Register
pub mod ICPR1 {
    pub use super::ICPR0::CLRPEND;
}

/// Interrupt Clear-Pending Register
pub mod ICPR2 {
    pub use super::ICPR0::CLRPEND;
}

/// Interrupt Active Bit Register
pub mod IABR0 {

    /// ACTIVE
    pub mod ACTIVE {
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

/// Interrupt Active Bit Register
pub mod IABR1 {
    pub use super::IABR0::ACTIVE;
}

/// Interrupt Active Bit Register
pub mod IABR2 {
    pub use super::IABR0::ACTIVE;
}

/// Interrupt Priority Register
pub mod IPR0 {

    /// IPR_N0
    pub mod IPR_N0 {
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

    /// IPR_N1
    pub mod IPR_N1 {
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

    /// IPR_N2
    pub mod IPR_N2 {
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

    /// IPR_N3
    pub mod IPR_N3 {
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

/// Interrupt Priority Register
pub mod IPR1 {
    pub use super::IPR0::IPR_N0;
    pub use super::IPR0::IPR_N1;
    pub use super::IPR0::IPR_N2;
    pub use super::IPR0::IPR_N3;
}

/// Interrupt Priority Register
pub mod IPR2 {
    pub use super::IPR0::IPR_N0;
    pub use super::IPR0::IPR_N1;
    pub use super::IPR0::IPR_N2;
    pub use super::IPR0::IPR_N3;
}

/// Interrupt Priority Register
pub mod IPR3 {
    pub use super::IPR0::IPR_N0;
    pub use super::IPR0::IPR_N1;
    pub use super::IPR0::IPR_N2;
    pub use super::IPR0::IPR_N3;
}

/// Interrupt Priority Register
pub mod IPR4 {
    pub use super::IPR0::IPR_N0;
    pub use super::IPR0::IPR_N1;
    pub use super::IPR0::IPR_N2;
    pub use super::IPR0::IPR_N3;
}

/// Interrupt Priority Register
pub mod IPR5 {
    pub use super::IPR0::IPR_N0;
    pub use super::IPR0::IPR_N1;
    pub use super::IPR0::IPR_N2;
    pub use super::IPR0::IPR_N3;
}

/// Interrupt Priority Register
pub mod IPR6 {
    pub use super::IPR0::IPR_N0;
    pub use super::IPR0::IPR_N1;
    pub use super::IPR0::IPR_N2;
    pub use super::IPR0::IPR_N3;
}

/// Interrupt Priority Register
pub mod IPR7 {
    pub use super::IPR0::IPR_N0;
    pub use super::IPR0::IPR_N1;
    pub use super::IPR0::IPR_N2;
    pub use super::IPR0::IPR_N3;
}

/// Interrupt Priority Register
pub mod IPR8 {
    pub use super::IPR0::IPR_N0;
    pub use super::IPR0::IPR_N1;
    pub use super::IPR0::IPR_N2;
    pub use super::IPR0::IPR_N3;
}

/// Interrupt Priority Register
pub mod IPR9 {
    pub use super::IPR0::IPR_N0;
    pub use super::IPR0::IPR_N1;
    pub use super::IPR0::IPR_N2;
    pub use super::IPR0::IPR_N3;
}

/// Interrupt Priority Register
pub mod IPR10 {
    pub use super::IPR0::IPR_N0;
    pub use super::IPR0::IPR_N1;
    pub use super::IPR0::IPR_N2;
    pub use super::IPR0::IPR_N3;
}

/// Interrupt Priority Register
pub mod IPR11 {
    pub use super::IPR0::IPR_N0;
    pub use super::IPR0::IPR_N1;
    pub use super::IPR0::IPR_N2;
    pub use super::IPR0::IPR_N3;
}

/// Interrupt Priority Register
pub mod IPR12 {
    pub use super::IPR0::IPR_N0;
    pub use super::IPR0::IPR_N1;
    pub use super::IPR0::IPR_N2;
    pub use super::IPR0::IPR_N3;
}

/// Interrupt Priority Register
pub mod IPR13 {
    pub use super::IPR0::IPR_N0;
    pub use super::IPR0::IPR_N1;
    pub use super::IPR0::IPR_N2;
    pub use super::IPR0::IPR_N3;
}

/// Interrupt Priority Register
pub mod IPR14 {
    pub use super::IPR0::IPR_N0;
    pub use super::IPR0::IPR_N1;
    pub use super::IPR0::IPR_N2;
    pub use super::IPR0::IPR_N3;
}

/// Interrupt Priority Register
pub mod IPR15 {
    pub use super::IPR0::IPR_N0;
    pub use super::IPR0::IPR_N1;
    pub use super::IPR0::IPR_N2;
    pub use super::IPR0::IPR_N3;
}

/// Interrupt Priority Register
pub mod IPR16 {
    pub use super::IPR0::IPR_N0;
    pub use super::IPR0::IPR_N1;
    pub use super::IPR0::IPR_N2;
    pub use super::IPR0::IPR_N3;
}

/// Interrupt Priority Register
pub mod IPR17 {
    pub use super::IPR0::IPR_N0;
    pub use super::IPR0::IPR_N1;
    pub use super::IPR0::IPR_N2;
    pub use super::IPR0::IPR_N3;
}

/// Interrupt Priority Register
pub mod IPR18 {
    pub use super::IPR0::IPR_N0;
    pub use super::IPR0::IPR_N1;
    pub use super::IPR0::IPR_N2;
    pub use super::IPR0::IPR_N3;
}

/// Interrupt Priority Register
pub mod IPR19 {
    pub use super::IPR0::IPR_N0;
    pub use super::IPR0::IPR_N1;
    pub use super::IPR0::IPR_N2;
    pub use super::IPR0::IPR_N3;
}

/// Interrupt Priority Register
pub mod IPR20 {
    pub use super::IPR0::IPR_N0;
    pub use super::IPR0::IPR_N1;
    pub use super::IPR0::IPR_N2;
    pub use super::IPR0::IPR_N3;
}

/// Interrupt Priority Register
pub mod IPR21 {
    pub use super::IPR0::IPR_N0;
    pub use super::IPR0::IPR_N1;
    pub use super::IPR0::IPR_N2;
    pub use super::IPR0::IPR_N3;
}

/// Interrupt Priority Register
pub mod IPR22 {
    pub use super::IPR0::IPR_N0;
    pub use super::IPR0::IPR_N1;
    pub use super::IPR0::IPR_N2;
    pub use super::IPR0::IPR_N3;
}

/// Interrupt Priority Register
pub mod IPR23 {
    pub use super::IPR0::IPR_N0;
    pub use super::IPR0::IPR_N1;
    pub use super::IPR0::IPR_N2;
    pub use super::IPR0::IPR_N3;
}

/// Interrupt Priority Register
pub mod IPR24 {
    pub use super::IPR0::IPR_N0;
    pub use super::IPR0::IPR_N1;
    pub use super::IPR0::IPR_N2;
    pub use super::IPR0::IPR_N3;
}

/// Interrupt Priority Register
pub mod IPR25 {
    pub use super::IPR0::IPR_N0;
    pub use super::IPR0::IPR_N1;
    pub use super::IPR0::IPR_N2;
    pub use super::IPR0::IPR_N3;
}

/// Interrupt Priority Register
pub mod IPR26 {
    pub use super::IPR0::IPR_N0;
    pub use super::IPR0::IPR_N1;
    pub use super::IPR0::IPR_N2;
    pub use super::IPR0::IPR_N3;
}

/// Interrupt Priority Register
pub mod IPR27 {
    pub use super::IPR0::IPR_N0;
    pub use super::IPR0::IPR_N1;
    pub use super::IPR0::IPR_N2;
    pub use super::IPR0::IPR_N3;
}

/// Interrupt Priority Register
pub mod IPR28 {
    pub use super::IPR0::IPR_N0;
    pub use super::IPR0::IPR_N1;
    pub use super::IPR0::IPR_N2;
    pub use super::IPR0::IPR_N3;
}

/// Interrupt Priority Register
pub mod IPR29 {
    pub use super::IPR0::IPR_N0;
    pub use super::IPR0::IPR_N1;
    pub use super::IPR0::IPR_N2;
    pub use super::IPR0::IPR_N3;
}

/// Interrupt Priority Register
pub mod IPR30 {
    pub use super::IPR0::IPR_N0;
    pub use super::IPR0::IPR_N1;
    pub use super::IPR0::IPR_N2;
    pub use super::IPR0::IPR_N3;
}

/// Interrupt Priority Register
pub mod IPR31 {
    pub use super::IPR0::IPR_N0;
    pub use super::IPR0::IPR_N1;
    pub use super::IPR0::IPR_N2;
    pub use super::IPR0::IPR_N3;
}

/// Interrupt Priority Register
pub mod IPR32 {
    pub use super::IPR0::IPR_N0;
    pub use super::IPR0::IPR_N1;
    pub use super::IPR0::IPR_N2;
    pub use super::IPR0::IPR_N3;
}

/// Interrupt Priority Register
pub mod IPR33 {
    pub use super::IPR0::IPR_N0;
    pub use super::IPR0::IPR_N1;
    pub use super::IPR0::IPR_N2;
    pub use super::IPR0::IPR_N3;
}

/// Interrupt Priority Register
pub mod IPR34 {
    pub use super::IPR0::IPR_N0;
    pub use super::IPR0::IPR_N1;
    pub use super::IPR0::IPR_N2;
    pub use super::IPR0::IPR_N3;
}

/// Interrupt Priority Register
pub mod IPR35 {
    pub use super::IPR0::IPR_N0;
    pub use super::IPR0::IPR_N1;
    pub use super::IPR0::IPR_N2;
    pub use super::IPR0::IPR_N3;
}

/// Interrupt Priority Register
pub mod IPR36 {
    pub use super::IPR0::IPR_N0;
    pub use super::IPR0::IPR_N1;
    pub use super::IPR0::IPR_N2;
    pub use super::IPR0::IPR_N3;
}

/// Interrupt Priority Register
pub mod IPR37 {
    pub use super::IPR0::IPR_N0;
    pub use super::IPR0::IPR_N1;
    pub use super::IPR0::IPR_N2;
    pub use super::IPR0::IPR_N3;
}

/// Interrupt Priority Register
pub mod IPR38 {
    pub use super::IPR0::IPR_N0;
    pub use super::IPR0::IPR_N1;
    pub use super::IPR0::IPR_N2;
    pub use super::IPR0::IPR_N3;
}

/// Interrupt Priority Register
pub mod IPR39 {
    pub use super::IPR0::IPR_N0;
    pub use super::IPR0::IPR_N1;
    pub use super::IPR0::IPR_N2;
    pub use super::IPR0::IPR_N3;
}

/// Interrupt Set-Enable Register
pub mod ISER3 {}

/// Interrupt Set-Enable Register
pub mod ISER4 {}

/// Interrupt Clear-Enable Register
pub mod ICER3 {}

/// Interrupt Clear-Enable Register
pub mod ICER4 {}

/// Interrupt Set-Pending Register
pub mod ISPR3 {}

/// Interrupt Set-Pending Register
pub mod ISPR4 {}

/// Interrupt Clear-Pending Register
pub mod ICPR3 {}

/// Interrupt Clear-Pending Register
pub mod ICPR4 {}

/// Interrupt Active Bit Register
pub mod IABR3 {}

/// Interrupt Active Bit Register
pub mod IABR4 {}
#[repr(C)]
pub struct RegisterBlock {
    /// Interrupt Set-Enable Register
    pub ISER0: RWRegister<u32>,

    /// Interrupt Set-Enable Register
    pub ISER1: RWRegister<u32>,

    /// Interrupt Set-Enable Register
    pub ISER2: RWRegister<u32>,

    /// Interrupt Set-Enable Register
    pub ISER3: RWRegister<u32>,

    /// Interrupt Set-Enable Register
    pub ISER4: RWRegister<u32>,

    _reserved1: [u8; 108],

    /// Interrupt Clear-Enable Register
    pub ICER0: RWRegister<u32>,

    /// Interrupt Clear-Enable Register
    pub ICER1: RWRegister<u32>,

    /// Interrupt Clear-Enable Register
    pub ICER2: RWRegister<u32>,

    /// Interrupt Clear-Enable Register
    pub ICER3: RWRegister<u32>,

    /// Interrupt Clear-Enable Register
    pub ICER4: RWRegister<u32>,

    _reserved2: [u8; 108],

    /// Interrupt Set-Pending Register
    pub ISPR0: RWRegister<u32>,

    /// Interrupt Set-Pending Register
    pub ISPR1: RWRegister<u32>,

    /// Interrupt Set-Pending Register
    pub ISPR2: RWRegister<u32>,

    /// Interrupt Set-Pending Register
    pub ISPR3: RWRegister<u32>,

    /// Interrupt Set-Pending Register
    pub ISPR4: RWRegister<u32>,

    _reserved3: [u8; 108],

    /// Interrupt Clear-Pending Register
    pub ICPR0: RWRegister<u32>,

    /// Interrupt Clear-Pending Register
    pub ICPR1: RWRegister<u32>,

    /// Interrupt Clear-Pending Register
    pub ICPR2: RWRegister<u32>,

    _reserved4: [u8; 52],

    /// Interrupt Clear-Pending Register
    pub ICPR3: RWRegister<u32>,

    /// Interrupt Clear-Pending Register
    pub ICPR4: RWRegister<u32>,

    _reserved5: [u8; 56],

    /// Interrupt Active Bit Register
    pub IABR0: RORegister<u32>,

    /// Interrupt Active Bit Register
    pub IABR1: RORegister<u32>,

    /// Interrupt Active Bit Register
    pub IABR2: RORegister<u32>,

    /// Interrupt Active Bit Register
    pub IABR3: RWRegister<u32>,

    /// Interrupt Active Bit Register
    pub IABR4: RWRegister<u32>,

    _reserved6: [u8; 236],

    /// Interrupt Priority Register
    pub IPR0: RWRegister<u32>,

    /// Interrupt Priority Register
    pub IPR1: RWRegister<u32>,

    /// Interrupt Priority Register
    pub IPR2: RWRegister<u32>,

    /// Interrupt Priority Register
    pub IPR3: RWRegister<u32>,

    /// Interrupt Priority Register
    pub IPR4: RWRegister<u32>,

    /// Interrupt Priority Register
    pub IPR5: RWRegister<u32>,

    /// Interrupt Priority Register
    pub IPR6: RWRegister<u32>,

    /// Interrupt Priority Register
    pub IPR7: RWRegister<u32>,

    /// Interrupt Priority Register
    pub IPR8: RWRegister<u32>,

    /// Interrupt Priority Register
    pub IPR9: RWRegister<u32>,

    /// Interrupt Priority Register
    pub IPR10: RWRegister<u32>,

    /// Interrupt Priority Register
    pub IPR11: RWRegister<u32>,

    /// Interrupt Priority Register
    pub IPR12: RWRegister<u32>,

    /// Interrupt Priority Register
    pub IPR13: RWRegister<u32>,

    /// Interrupt Priority Register
    pub IPR14: RWRegister<u32>,

    /// Interrupt Priority Register
    pub IPR15: RWRegister<u32>,

    /// Interrupt Priority Register
    pub IPR16: RWRegister<u32>,

    /// Interrupt Priority Register
    pub IPR17: RWRegister<u32>,

    /// Interrupt Priority Register
    pub IPR18: RWRegister<u32>,

    /// Interrupt Priority Register
    pub IPR19: RWRegister<u32>,

    /// Interrupt Priority Register
    pub IPR20: RWRegister<u32>,

    /// Interrupt Priority Register
    pub IPR21: RWRegister<u32>,

    /// Interrupt Priority Register
    pub IPR22: RWRegister<u32>,

    /// Interrupt Priority Register
    pub IPR23: RWRegister<u32>,

    /// Interrupt Priority Register
    pub IPR24: RWRegister<u32>,

    /// Interrupt Priority Register
    pub IPR25: RWRegister<u32>,

    /// Interrupt Priority Register
    pub IPR26: RWRegister<u32>,

    /// Interrupt Priority Register
    pub IPR27: RWRegister<u32>,

    /// Interrupt Priority Register
    pub IPR28: RWRegister<u32>,

    /// Interrupt Priority Register
    pub IPR29: RWRegister<u32>,

    /// Interrupt Priority Register
    pub IPR30: RWRegister<u32>,

    /// Interrupt Priority Register
    pub IPR31: RWRegister<u32>,

    /// Interrupt Priority Register
    pub IPR32: RWRegister<u32>,

    /// Interrupt Priority Register
    pub IPR33: RWRegister<u32>,

    /// Interrupt Priority Register
    pub IPR34: RWRegister<u32>,

    /// Interrupt Priority Register
    pub IPR35: RWRegister<u32>,

    /// Interrupt Priority Register
    pub IPR36: RWRegister<u32>,

    /// Interrupt Priority Register
    pub IPR37: RWRegister<u32>,

    /// Interrupt Priority Register
    pub IPR38: RWRegister<u32>,

    /// Interrupt Priority Register
    pub IPR39: RWRegister<u32>,
}
pub struct ResetValues {
    pub ISER0: u32,
    pub ISER1: u32,
    pub ISER2: u32,
    pub ISER3: u32,
    pub ISER4: u32,
    pub ICER0: u32,
    pub ICER1: u32,
    pub ICER2: u32,
    pub ICER3: u32,
    pub ICER4: u32,
    pub ISPR0: u32,
    pub ISPR1: u32,
    pub ISPR2: u32,
    pub ISPR3: u32,
    pub ISPR4: u32,
    pub ICPR0: u32,
    pub ICPR1: u32,
    pub ICPR2: u32,
    pub ICPR3: u32,
    pub ICPR4: u32,
    pub IABR0: u32,
    pub IABR1: u32,
    pub IABR2: u32,
    pub IABR3: u32,
    pub IABR4: u32,
    pub IPR0: u32,
    pub IPR1: u32,
    pub IPR2: u32,
    pub IPR3: u32,
    pub IPR4: u32,
    pub IPR5: u32,
    pub IPR6: u32,
    pub IPR7: u32,
    pub IPR8: u32,
    pub IPR9: u32,
    pub IPR10: u32,
    pub IPR11: u32,
    pub IPR12: u32,
    pub IPR13: u32,
    pub IPR14: u32,
    pub IPR15: u32,
    pub IPR16: u32,
    pub IPR17: u32,
    pub IPR18: u32,
    pub IPR19: u32,
    pub IPR20: u32,
    pub IPR21: u32,
    pub IPR22: u32,
    pub IPR23: u32,
    pub IPR24: u32,
    pub IPR25: u32,
    pub IPR26: u32,
    pub IPR27: u32,
    pub IPR28: u32,
    pub IPR29: u32,
    pub IPR30: u32,
    pub IPR31: u32,
    pub IPR32: u32,
    pub IPR33: u32,
    pub IPR34: u32,
    pub IPR35: u32,
    pub IPR36: u32,
    pub IPR37: u32,
    pub IPR38: u32,
    pub IPR39: u32,
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
