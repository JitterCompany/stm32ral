#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Ethernet: Precision time protocol
//!
//! Used by: stm32f407, stm32f427, stm32f429, stm32f469

use crate::{RORegister, RWRegister};
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// Ethernet PTP time stamp control register
pub mod PTPTSCR {

    /// TSE
    pub mod TSE {
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

    /// TSFCU
    pub mod TSFCU {
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

    /// TSPTPPSV2E
    pub mod TSPTPPSV2E {
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

    /// TSSPTPOEFE
    pub mod TSSPTPOEFE {
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

    /// TSSIPV6FE
    pub mod TSSIPV6FE {
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

    /// TSSIPV4FE
    pub mod TSSIPV4FE {
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

    /// TSSEME
    pub mod TSSEME {
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

    /// TSSMRME
    pub mod TSSMRME {
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

    /// TSCNT
    pub mod TSCNT {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (2 bits: 0b11 << 16)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// TSPFFMAE
    pub mod TSPFFMAE {
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

    /// TSSTI
    pub mod TSSTI {
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

    /// TSSTU
    pub mod TSSTU {
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

    /// TSITE
    pub mod TSITE {
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

    /// TTSARU
    pub mod TTSARU {
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

    /// TSSARFE
    pub mod TSSARFE {
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

    /// TSSSR
    pub mod TSSSR {
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

/// Ethernet PTP subsecond increment register
pub mod PTPSSIR {

    /// STSSI
    pub mod STSSI {
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

/// Ethernet PTP time stamp high register
pub mod PTPTSHR {

    /// STS
    pub mod STS {
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

/// Ethernet PTP time stamp low register
pub mod PTPTSLR {

    /// STSS
    pub mod STSS {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (31 bits: 0x7fffffff << 0)
        pub const mask: u32 = 0x7fffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// STPNS
    pub mod STPNS {
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

/// Ethernet PTP time stamp high update register
pub mod PTPTSHUR {

    /// TSUS
    pub mod TSUS {
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

/// Ethernet PTP time stamp low update register
pub mod PTPTSLUR {

    /// TSUSS
    pub mod TSUSS {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (31 bits: 0x7fffffff << 0)
        pub const mask: u32 = 0x7fffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// TSUPNS
    pub mod TSUPNS {
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

/// Ethernet PTP time stamp addend register
pub mod PTPTSAR {

    /// TSA
    pub mod TSA {
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

/// Ethernet PTP target time high register
pub mod PTPTTHR {

    /// 0
    pub mod TTSH {
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

/// Ethernet PTP target time low register
pub mod PTPTTLR {

    /// TTSL
    pub mod TTSL {
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

/// Ethernet PTP time stamp status register
pub mod PTPTSSR {

    /// TSSO
    pub mod TSSO {
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

    /// TSTTR
    pub mod TSTTR {
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

/// Ethernet PTP PPS control register
pub mod PTPPPSCR {

    /// PPS frequency selection
    pub mod PPSFREQ {
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

            /// 0b0000: 1 Hz with a pulse width of 125 ms for binary rollover and of 100 ms for digital rollover
            pub const Hz_1: u32 = 0b0000;

            /// 0b0001: 2 Hz with 50% duty cycle for binary rollover (digital rollover not recommended)
            pub const Hz_2: u32 = 0b0001;

            /// 0b0010: 4 Hz with 50% duty cycle for binary rollover (digital rollover not recommended)
            pub const Hz_4: u32 = 0b0010;

            /// 0b0011: 8 Hz with 50% duty cycle for binary rollover (digital rollover not recommended)
            pub const Hz_8: u32 = 0b0011;

            /// 0b0100: 16 Hz with 50% duty cycle for binary rollover (digital rollover not recommended)
            pub const Hz_16: u32 = 0b0100;

            /// 0b0101: 32 Hz with 50% duty cycle for binary rollover (digital rollover not recommended)
            pub const Hz_32: u32 = 0b0101;

            /// 0b0110: 64 Hz with 50% duty cycle for binary rollover (digital rollover not recommended)
            pub const Hz_64: u32 = 0b0110;

            /// 0b0111: 128 Hz with 50% duty cycle for binary rollover (digital rollover not recommended)
            pub const Hz_128: u32 = 0b0111;

            /// 0b1000: 256 Hz with 50% duty cycle for binary rollover (digital rollover not recommended)
            pub const Hz_256: u32 = 0b1000;

            /// 0b1001: 512 Hz with 50% duty cycle for binary rollover (digital rollover not recommended)
            pub const Hz_512: u32 = 0b1001;

            /// 0b1010: 1024 Hz with 50% duty cycle for binary rollover (digital rollover not recommended)
            pub const Hz_1024: u32 = 0b1010;

            /// 0b1011: 2048 Hz with 50% duty cycle for binary rollover (digital rollover not recommended)
            pub const Hz_2048: u32 = 0b1011;

            /// 0b1100: 4096 Hz with 50% duty cycle for binary rollover (digital rollover not recommended)
            pub const Hz_4096: u32 = 0b1100;

            /// 0b1101: 8192 Hz with 50% duty cycle for binary rollover (digital rollover not recommended)
            pub const Hz_8192: u32 = 0b1101;

            /// 0b1110: 16384 Hz with 50% duty cycle for binary rollover (digital rollover not recommended)
            pub const Hz_16384: u32 = 0b1110;

            /// 0b1111: 32768 Hz with 50% duty cycle for binary rollover (digital rollover not recommended)
            pub const Hz_32768: u32 = 0b1111;
        }
    }
}
#[repr(C)]
pub struct RegisterBlock {
    /// Ethernet PTP time stamp control register
    pub PTPTSCR: RWRegister<u32>,

    /// Ethernet PTP subsecond increment register
    pub PTPSSIR: RWRegister<u32>,

    /// Ethernet PTP time stamp high register
    pub PTPTSHR: RORegister<u32>,

    /// Ethernet PTP time stamp low register
    pub PTPTSLR: RORegister<u32>,

    /// Ethernet PTP time stamp high update register
    pub PTPTSHUR: RWRegister<u32>,

    /// Ethernet PTP time stamp low update register
    pub PTPTSLUR: RWRegister<u32>,

    /// Ethernet PTP time stamp addend register
    pub PTPTSAR: RWRegister<u32>,

    /// Ethernet PTP target time high register
    pub PTPTTHR: RWRegister<u32>,

    /// Ethernet PTP target time low register
    pub PTPTTLR: RWRegister<u32>,

    _reserved1: [u8; 4],

    /// Ethernet PTP time stamp status register
    pub PTPTSSR: RORegister<u32>,

    /// Ethernet PTP PPS control register
    pub PTPPPSCR: RORegister<u32>,
}
pub struct ResetValues {
    pub PTPTSCR: u32,
    pub PTPSSIR: u32,
    pub PTPTSHR: u32,
    pub PTPTSLR: u32,
    pub PTPTSHUR: u32,
    pub PTPTSLUR: u32,
    pub PTPTSAR: u32,
    pub PTPTTHR: u32,
    pub PTPTTLR: u32,
    pub PTPTSSR: u32,
    pub PTPPPSCR: u32,
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
