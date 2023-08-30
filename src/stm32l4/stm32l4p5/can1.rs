#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Controller area network

use crate::{RORegister, RWRegister};
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// master control register
pub mod MCR {

    /// DBF
    pub mod DBF {
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

    /// RESET
    pub mod RESET {
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

    /// TTCM
    pub mod TTCM {
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

    /// ABOM
    pub mod ABOM {
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

    /// AWUM
    pub mod AWUM {
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

    /// NART
    pub mod NART {
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

    /// RFLM
    pub mod RFLM {
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

    /// TXFP
    pub mod TXFP {
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

    /// SLEEP
    pub mod SLEEP {
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

    /// INRQ
    pub mod INRQ {
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
}

/// master status register
pub mod MSR {

    /// RX
    pub mod RX {
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

    /// SAMP
    pub mod SAMP {
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

    /// RXM
    pub mod RXM {
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

    /// TXM
    pub mod TXM {
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

    /// SLAKI
    pub mod SLAKI {
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

    /// WKUI
    pub mod WKUI {
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

    /// ERRI
    pub mod ERRI {
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

    /// SLAK
    pub mod SLAK {
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

    /// INAK
    pub mod INAK {
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
}

/// transmit status register
pub mod TSR {

    /// CODE
    pub mod CODE {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (2 bits: 0b11 << 24)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// ABRQ2
    pub mod ABRQ2 {
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

    /// TERR2
    pub mod TERR2 {
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

    /// ALST2
    pub mod ALST2 {
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

    /// TXOK2
    pub mod TXOK2 {
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

    /// RQCP2
    pub mod RQCP2 {
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

    /// ABRQ1
    pub mod ABRQ1 {
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

    /// TERR1
    pub mod TERR1 {
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

    /// ALST1
    pub mod ALST1 {
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

    /// TXOK1
    pub mod TXOK1 {
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

    /// RQCP1
    pub mod RQCP1 {
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

    /// ABRQ0
    pub mod ABRQ0 {
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

    /// TERR0
    pub mod TERR0 {
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

    /// ALST0
    pub mod ALST0 {
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

    /// TXOK0
    pub mod TXOK0 {
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

    /// RQCP0
    pub mod RQCP0 {
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

    /// Lowest priority flag for mailbox 2
    pub mod TME {
        /// Offset (26 bits)
        pub const offset: u32 = 26;
        /// Mask (3 bits: 0b111 << 26)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Lowest priority flag for mailbox 2
    pub mod LOW {
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

/// receive FIFO 0 register
pub mod RF0R {

    /// RFOM0
    pub mod RFOM0 {
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

    /// FOVR0
    pub mod FOVR0 {
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

    /// FULL0
    pub mod FULL0 {
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

    /// FMP0
    pub mod FMP0 {
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
}

/// receive FIFO 1 register
pub mod RF1R {

    /// RFOM1
    pub mod RFOM1 {
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

    /// FOVR1
    pub mod FOVR1 {
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

    /// FULL1
    pub mod FULL1 {
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

    /// FMP1
    pub mod FMP1 {
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
}

/// interrupt enable register
pub mod IER {

    /// SLKIE
    pub mod SLKIE {
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

    /// WKUIE
    pub mod WKUIE {
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

    /// ERRIE
    pub mod ERRIE {
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

    /// LECIE
    pub mod LECIE {
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

    /// BOFIE
    pub mod BOFIE {
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

    /// EPVIE
    pub mod EPVIE {
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

    /// EWGIE
    pub mod EWGIE {
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

    /// FOVIE1
    pub mod FOVIE1 {
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

    /// FFIE1
    pub mod FFIE1 {
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

    /// FMPIE1
    pub mod FMPIE1 {
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

    /// FOVIE0
    pub mod FOVIE0 {
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

    /// FFIE0
    pub mod FFIE0 {
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

    /// FMPIE0
    pub mod FMPIE0 {
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

    /// TMEIE
    pub mod TMEIE {
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
}

/// interrupt enable register
pub mod ESR {

    /// REC
    pub mod REC {
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

    /// TEC
    pub mod TEC {
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

    /// LEC
    pub mod LEC {
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

    /// BOFF
    pub mod BOFF {
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

    /// EPVF
    pub mod EPVF {
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

    /// EWGF
    pub mod EWGF {
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
}

/// bit timing register
pub mod BTR {

    /// SILM
    pub mod SILM {
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

    /// LBKM
    pub mod LBKM {
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

    /// SJW
    pub mod SJW {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (2 bits: 0b11 << 24)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// TS2
    pub mod TS2 {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (3 bits: 0b111 << 20)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// TS1
    pub mod TS1 {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (4 bits: 0b1111 << 16)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// BRP
    pub mod BRP {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (10 bits: 0x3ff << 0)
        pub const mask: u32 = 0x3ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// TX mailbox identifier register
pub mod TI0R {

    /// STID
    pub mod STID {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (11 bits: 0x7ff << 21)
        pub const mask: u32 = 0x7ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// EXID
    pub mod EXID {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (18 bits: 0x3ffff << 3)
        pub const mask: u32 = 0x3ffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// IDE
    pub mod IDE {
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

    /// RTR
    pub mod RTR {
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

    /// TXRQ
    pub mod TXRQ {
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
}

/// mailbox data length control and time stamp register
pub mod TDT0R {

    /// TIME
    pub mod TIME {
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

    /// TGT
    pub mod TGT {
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

    /// DLC
    pub mod DLC {
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
}

/// mailbox data low register
pub mod TDL0R {

    /// DATA3
    pub mod DATA3 {
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

    /// DATA2
    pub mod DATA2 {
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

    /// DATA1
    pub mod DATA1 {
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

    /// DATA0
    pub mod DATA0 {
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

/// mailbox data high register
pub mod TDH0R {

    /// DATA7
    pub mod DATA7 {
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

    /// DATA6
    pub mod DATA6 {
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

    /// DATA5
    pub mod DATA5 {
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

    /// DATA4
    pub mod DATA4 {
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

/// mailbox identifier register
pub mod TI1R {
    pub use super::TI0R::EXID;
    pub use super::TI0R::IDE;
    pub use super::TI0R::RTR;
    pub use super::TI0R::STID;
    pub use super::TI0R::TXRQ;
}

/// mailbox data length control and time stamp register
pub mod TDT1R {
    pub use super::TDT0R::DLC;
    pub use super::TDT0R::TGT;
    pub use super::TDT0R::TIME;
}

/// mailbox data low register
pub mod TDL1R {
    pub use super::TDL0R::DATA0;
    pub use super::TDL0R::DATA1;
    pub use super::TDL0R::DATA2;
    pub use super::TDL0R::DATA3;
}

/// mailbox data high register
pub mod TDH1R {
    pub use super::TDH0R::DATA4;
    pub use super::TDH0R::DATA5;
    pub use super::TDH0R::DATA6;
    pub use super::TDH0R::DATA7;
}

/// mailbox identifier register
pub mod TI2R {
    pub use super::TI0R::EXID;
    pub use super::TI0R::IDE;
    pub use super::TI0R::RTR;
    pub use super::TI0R::STID;
    pub use super::TI0R::TXRQ;
}

/// mailbox data length control and time stamp register
pub mod TDT2R {
    pub use super::TDT0R::DLC;
    pub use super::TDT0R::TGT;
    pub use super::TDT0R::TIME;
}

/// mailbox data low register
pub mod TDL2R {
    pub use super::TDL0R::DATA0;
    pub use super::TDL0R::DATA1;
    pub use super::TDL0R::DATA2;
    pub use super::TDL0R::DATA3;
}

/// mailbox data high register
pub mod TDH2R {
    pub use super::TDH0R::DATA4;
    pub use super::TDH0R::DATA5;
    pub use super::TDH0R::DATA6;
    pub use super::TDH0R::DATA7;
}

/// receive FIFO mailbox identifier register
pub mod RI0R {

    /// STID
    pub mod STID {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (11 bits: 0x7ff << 21)
        pub const mask: u32 = 0x7ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// EXID
    pub mod EXID {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (18 bits: 0x3ffff << 3)
        pub const mask: u32 = 0x3ffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// IDE
    pub mod IDE {
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

    /// RTR
    pub mod RTR {
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

/// mailbox data high register
pub mod RDT0R {

    /// TIME
    pub mod TIME {
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

    /// FMI
    pub mod FMI {
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

    /// DLC
    pub mod DLC {
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
}

/// mailbox data high register
pub mod RDL0R {

    /// DATA3
    pub mod DATA3 {
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

    /// DATA2
    pub mod DATA2 {
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

    /// DATA1
    pub mod DATA1 {
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

    /// DATA0
    pub mod DATA0 {
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

/// receive FIFO mailbox data high register
pub mod RDH0R {

    /// DATA7
    pub mod DATA7 {
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

    /// DATA6
    pub mod DATA6 {
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

    /// DATA5
    pub mod DATA5 {
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

    /// DATA4
    pub mod DATA4 {
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

/// mailbox data high register
pub mod RI1R {
    pub use super::RI0R::EXID;
    pub use super::RI0R::IDE;
    pub use super::RI0R::RTR;
    pub use super::RI0R::STID;
}

/// mailbox data high register
pub mod RDT1R {
    pub use super::RDT0R::DLC;
    pub use super::RDT0R::FMI;
    pub use super::RDT0R::TIME;
}

/// mailbox data high register
pub mod RDL1R {
    pub use super::RDL0R::DATA0;
    pub use super::RDL0R::DATA1;
    pub use super::RDL0R::DATA2;
    pub use super::RDL0R::DATA3;
}

/// mailbox data high register
pub mod RDH1R {
    pub use super::RDH0R::DATA4;
    pub use super::RDH0R::DATA5;
    pub use super::RDH0R::DATA6;
    pub use super::RDH0R::DATA7;
}

/// filter scale register
pub mod FS1R {

    /// Filter scale configuration
    pub mod FSC {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (28 bits: 0xfffffff << 0)
        pub const mask: u32 = 0xfffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// filter FIFO assignment register
pub mod FFA1R {

    /// Filter FIFO assignment for filter 0
    pub mod FFA {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (28 bits: 0xfffffff << 0)
        pub const mask: u32 = 0xfffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// filter activation register
pub mod FA1R {

    /// Filter active
    pub mod FACT {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (28 bits: 0xfffffff << 0)
        pub const mask: u32 = 0xfffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Filter bank 0 register 1
pub mod F0R1 {

    /// Filter bits
    pub mod FB {
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

/// Filter bank 0 register 2
pub mod F0R2 {
    pub use super::F0R1::FB;
}

/// Filter bank 1 register 1
pub mod F1R1 {
    pub use super::F0R1::FB;
}

/// Filter bank 1 register 2
pub mod F1R2 {
    pub use super::F0R1::FB;
}

/// Filter bank 2 register 1
pub mod F2R1 {
    pub use super::F0R1::FB;
}

/// Filter bank 2 register 2
pub mod F2R2 {
    pub use super::F0R1::FB;
}

/// Filter bank 3 register 1
pub mod F3R1 {
    pub use super::F0R1::FB;
}

/// Filter bank 3 register 2
pub mod F3R2 {
    pub use super::F0R1::FB;
}

/// Filter bank 4 register 1
pub mod F4R1 {
    pub use super::F0R1::FB;
}

/// Filter bank 4 register 2
pub mod F4R2 {
    pub use super::F0R1::FB;
}

/// Filter bank 5 register 1
pub mod F5R1 {
    pub use super::F0R1::FB;
}

/// Filter bank 5 register 2
pub mod F5R2 {
    pub use super::F0R1::FB;
}

/// Filter bank 6 register 1
pub mod F6R1 {
    pub use super::F0R1::FB;
}

/// Filter bank 6 register 2
pub mod F6R2 {
    pub use super::F0R1::FB;
}

/// Filter bank 7 register 1
pub mod F7R1 {
    pub use super::F0R1::FB;
}

/// Filter bank 7 register 2
pub mod F7R2 {
    pub use super::F0R1::FB;
}

/// Filter bank 8 register 1
pub mod F8R1 {
    pub use super::F0R1::FB;
}

/// Filter bank 8 register 2
pub mod F8R2 {
    pub use super::F0R1::FB;
}

/// Filter bank 9 register 1
pub mod F9R1 {
    pub use super::F0R1::FB;
}

/// Filter bank 9 register 2
pub mod F9R2 {
    pub use super::F0R1::FB;
}

/// Filter bank 10 register 1
pub mod F10R1 {
    pub use super::F0R1::FB;
}

/// Filter bank 10 register 2
pub mod F10R2 {
    pub use super::F0R1::FB;
}

/// Filter bank 11 register 1
pub mod F11R1 {
    pub use super::F0R1::FB;
}

/// Filter bank 11 register 2
pub mod F11R2 {
    pub use super::F0R1::FB;
}

/// Filter bank 4 register 1
pub mod F12R1 {
    pub use super::F0R1::FB;
}

/// Filter bank 12 register 2
pub mod F12R2 {
    pub use super::F0R1::FB;
}

/// Filter bank 13 register 1
pub mod F13R1 {
    pub use super::F0R1::FB;
}

/// Filter bank 13 register 2
pub mod F13R2 {
    pub use super::F0R1::FB;
}

/// Filter bank 14 register 1
pub mod F14R1 {
    pub use super::F0R1::FB;
}

/// Filter bank 14 register 2
pub mod F14R2 {
    pub use super::F0R1::FB;
}

/// Filter bank 15 register 1
pub mod F15R1 {
    pub use super::F0R1::FB;
}

/// Filter bank 15 register 2
pub mod F15R2 {
    pub use super::F0R1::FB;
}

/// Filter bank 16 register 1
pub mod F16R1 {
    pub use super::F0R1::FB;
}

/// Filter bank 16 register 2
pub mod F16R2 {
    pub use super::F0R1::FB;
}

/// Filter bank 17 register 1
pub mod F17R1 {
    pub use super::F0R1::FB;
}

/// Filter bank 17 register 2
pub mod F17R2 {
    pub use super::F0R1::FB;
}

/// Filter bank 18 register 1
pub mod F18R1 {
    pub use super::F0R1::FB;
}

/// Filter bank 18 register 2
pub mod F18R2 {
    pub use super::F0R1::FB;
}

/// Filter bank 19 register 1
pub mod F19R1 {
    pub use super::F0R1::FB;
}

/// Filter bank 19 register 2
pub mod F19R2 {
    pub use super::F0R1::FB;
}

/// Filter bank 20 register 1
pub mod F20R1 {
    pub use super::F0R1::FB;
}

/// Filter bank 20 register 2
pub mod F20R2 {
    pub use super::F0R1::FB;
}

/// Filter bank 21 register 1
pub mod F21R1 {
    pub use super::F0R1::FB;
}

/// Filter bank 21 register 2
pub mod F21R2 {
    pub use super::F0R1::FB;
}

/// Filter bank 22 register 1
pub mod F22R1 {
    pub use super::F0R1::FB;
}

/// Filter bank 22 register 2
pub mod F22R2 {
    pub use super::F0R1::FB;
}

/// Filter bank 23 register 1
pub mod F23R1 {
    pub use super::F0R1::FB;
}

/// Filter bank 23 register 2
pub mod F23R2 {
    pub use super::F0R1::FB;
}

/// Filter bank 24 register 1
pub mod F24R1 {
    pub use super::F0R1::FB;
}

/// Filter bank 24 register 2
pub mod F24R2 {
    pub use super::F0R1::FB;
}

/// Filter bank 25 register 1
pub mod F25R1 {
    pub use super::F0R1::FB;
}

/// Filter bank 25 register 2
pub mod F25R2 {
    pub use super::F0R1::FB;
}

/// Filter bank 26 register 1
pub mod F26R1 {
    pub use super::F0R1::FB;
}

/// Filter bank 26 register 2
pub mod F26R2 {
    pub use super::F0R1::FB;
}

/// Filter bank 27 register 1
pub mod F27R1 {
    pub use super::F0R1::FB;
}

/// Filter bank 27 register 2
pub mod F27R2 {
    pub use super::F0R1::FB;
}
#[repr(C)]
pub struct RegisterBlock {
    /// master control register
    pub MCR: RWRegister<u32>,

    /// master status register
    pub MSR: RWRegister<u32>,

    /// transmit status register
    pub TSR: RWRegister<u32>,

    /// receive FIFO 0 register
    pub RF0R: RWRegister<u32>,

    /// receive FIFO 1 register
    pub RF1R: RWRegister<u32>,

    /// interrupt enable register
    pub IER: RWRegister<u32>,

    /// interrupt enable register
    pub ESR: RWRegister<u32>,

    /// bit timing register
    pub BTR: RWRegister<u32>,

    _reserved1: [u8; 352],

    /// TX mailbox identifier register
    pub TI0R: RWRegister<u32>,

    /// mailbox data length control and time stamp register
    pub TDT0R: RWRegister<u32>,

    /// mailbox data low register
    pub TDL0R: RWRegister<u32>,

    /// mailbox data high register
    pub TDH0R: RWRegister<u32>,

    /// mailbox identifier register
    pub TI1R: RWRegister<u32>,

    /// mailbox data length control and time stamp register
    pub TDT1R: RWRegister<u32>,

    /// mailbox data low register
    pub TDL1R: RWRegister<u32>,

    /// mailbox data high register
    pub TDH1R: RWRegister<u32>,

    /// mailbox identifier register
    pub TI2R: RWRegister<u32>,

    /// mailbox data length control and time stamp register
    pub TDT2R: RWRegister<u32>,

    /// mailbox data low register
    pub TDL2R: RWRegister<u32>,

    /// mailbox data high register
    pub TDH2R: RWRegister<u32>,

    /// receive FIFO mailbox identifier register
    pub RI0R: RORegister<u32>,

    /// mailbox data high register
    pub RDT0R: RORegister<u32>,

    /// mailbox data high register
    pub RDL0R: RORegister<u32>,

    /// receive FIFO mailbox data high register
    pub RDH0R: RORegister<u32>,

    /// mailbox data high register
    pub RI1R: RORegister<u32>,

    /// mailbox data high register
    pub RDT1R: RORegister<u32>,

    /// mailbox data high register
    pub RDL1R: RORegister<u32>,

    /// mailbox data high register
    pub RDH1R: RORegister<u32>,

    _reserved2: [u8; 60],

    /// filter scale register
    pub FS1R: RWRegister<u32>,

    _reserved3: [u8; 4],

    /// filter FIFO assignment register
    pub FFA1R: RWRegister<u32>,

    _reserved4: [u8; 4],

    /// filter activation register
    pub FA1R: RWRegister<u32>,

    _reserved5: [u8; 32],

    /// Filter bank 0 register 1
    pub F0R1: RWRegister<u32>,

    /// Filter bank 0 register 2
    pub F0R2: RWRegister<u32>,

    /// Filter bank 1 register 1
    pub F1R1: RWRegister<u32>,

    /// Filter bank 1 register 2
    pub F1R2: RWRegister<u32>,

    /// Filter bank 2 register 1
    pub F2R1: RWRegister<u32>,

    /// Filter bank 2 register 2
    pub F2R2: RWRegister<u32>,

    /// Filter bank 3 register 1
    pub F3R1: RWRegister<u32>,

    /// Filter bank 3 register 2
    pub F3R2: RWRegister<u32>,

    /// Filter bank 4 register 1
    pub F4R1: RWRegister<u32>,

    /// Filter bank 4 register 2
    pub F4R2: RWRegister<u32>,

    /// Filter bank 5 register 1
    pub F5R1: RWRegister<u32>,

    /// Filter bank 5 register 2
    pub F5R2: RWRegister<u32>,

    /// Filter bank 6 register 1
    pub F6R1: RWRegister<u32>,

    /// Filter bank 6 register 2
    pub F6R2: RWRegister<u32>,

    /// Filter bank 7 register 1
    pub F7R1: RWRegister<u32>,

    /// Filter bank 7 register 2
    pub F7R2: RWRegister<u32>,

    /// Filter bank 8 register 1
    pub F8R1: RWRegister<u32>,

    /// Filter bank 8 register 2
    pub F8R2: RWRegister<u32>,

    /// Filter bank 9 register 1
    pub F9R1: RWRegister<u32>,

    /// Filter bank 9 register 2
    pub F9R2: RWRegister<u32>,

    /// Filter bank 10 register 1
    pub F10R1: RWRegister<u32>,

    /// Filter bank 10 register 2
    pub F10R2: RWRegister<u32>,

    /// Filter bank 11 register 1
    pub F11R1: RWRegister<u32>,

    /// Filter bank 11 register 2
    pub F11R2: RWRegister<u32>,

    /// Filter bank 4 register 1
    pub F12R1: RWRegister<u32>,

    /// Filter bank 12 register 2
    pub F12R2: RWRegister<u32>,

    /// Filter bank 13 register 1
    pub F13R1: RWRegister<u32>,

    /// Filter bank 13 register 2
    pub F13R2: RWRegister<u32>,

    /// Filter bank 14 register 1
    pub F14R1: RWRegister<u32>,

    /// Filter bank 14 register 2
    pub F14R2: RWRegister<u32>,

    /// Filter bank 15 register 1
    pub F15R1: RWRegister<u32>,

    /// Filter bank 15 register 2
    pub F15R2: RWRegister<u32>,

    /// Filter bank 16 register 1
    pub F16R1: RWRegister<u32>,

    /// Filter bank 16 register 2
    pub F16R2: RWRegister<u32>,

    /// Filter bank 17 register 1
    pub F17R1: RWRegister<u32>,

    /// Filter bank 17 register 2
    pub F17R2: RWRegister<u32>,

    /// Filter bank 18 register 1
    pub F18R1: RWRegister<u32>,

    /// Filter bank 18 register 2
    pub F18R2: RWRegister<u32>,

    /// Filter bank 19 register 1
    pub F19R1: RWRegister<u32>,

    /// Filter bank 19 register 2
    pub F19R2: RWRegister<u32>,

    /// Filter bank 20 register 1
    pub F20R1: RWRegister<u32>,

    /// Filter bank 20 register 2
    pub F20R2: RWRegister<u32>,

    /// Filter bank 21 register 1
    pub F21R1: RWRegister<u32>,

    /// Filter bank 21 register 2
    pub F21R2: RWRegister<u32>,

    /// Filter bank 22 register 1
    pub F22R1: RWRegister<u32>,

    /// Filter bank 22 register 2
    pub F22R2: RWRegister<u32>,

    /// Filter bank 23 register 1
    pub F23R1: RWRegister<u32>,

    /// Filter bank 23 register 2
    pub F23R2: RWRegister<u32>,

    /// Filter bank 24 register 1
    pub F24R1: RWRegister<u32>,

    /// Filter bank 24 register 2
    pub F24R2: RWRegister<u32>,

    /// Filter bank 25 register 1
    pub F25R1: RWRegister<u32>,

    /// Filter bank 25 register 2
    pub F25R2: RWRegister<u32>,

    /// Filter bank 26 register 1
    pub F26R1: RWRegister<u32>,

    /// Filter bank 26 register 2
    pub F26R2: RWRegister<u32>,

    /// Filter bank 27 register 1
    pub F27R1: RWRegister<u32>,

    /// Filter bank 27 register 2
    pub F27R2: RWRegister<u32>,
}
pub struct ResetValues {
    pub MCR: u32,
    pub MSR: u32,
    pub TSR: u32,
    pub RF0R: u32,
    pub RF1R: u32,
    pub IER: u32,
    pub ESR: u32,
    pub BTR: u32,
    pub TI0R: u32,
    pub TDT0R: u32,
    pub TDL0R: u32,
    pub TDH0R: u32,
    pub TI1R: u32,
    pub TDT1R: u32,
    pub TDL1R: u32,
    pub TDH1R: u32,
    pub TI2R: u32,
    pub TDT2R: u32,
    pub TDL2R: u32,
    pub TDH2R: u32,
    pub RI0R: u32,
    pub RDT0R: u32,
    pub RDL0R: u32,
    pub RDH0R: u32,
    pub RI1R: u32,
    pub RDT1R: u32,
    pub RDL1R: u32,
    pub RDH1R: u32,
    pub FS1R: u32,
    pub FFA1R: u32,
    pub FA1R: u32,
    pub F0R1: u32,
    pub F0R2: u32,
    pub F1R1: u32,
    pub F1R2: u32,
    pub F2R1: u32,
    pub F2R2: u32,
    pub F3R1: u32,
    pub F3R2: u32,
    pub F4R1: u32,
    pub F4R2: u32,
    pub F5R1: u32,
    pub F5R2: u32,
    pub F6R1: u32,
    pub F6R2: u32,
    pub F7R1: u32,
    pub F7R2: u32,
    pub F8R1: u32,
    pub F8R2: u32,
    pub F9R1: u32,
    pub F9R2: u32,
    pub F10R1: u32,
    pub F10R2: u32,
    pub F11R1: u32,
    pub F11R2: u32,
    pub F12R1: u32,
    pub F12R2: u32,
    pub F13R1: u32,
    pub F13R2: u32,
    pub F14R1: u32,
    pub F14R2: u32,
    pub F15R1: u32,
    pub F15R2: u32,
    pub F16R1: u32,
    pub F16R2: u32,
    pub F17R1: u32,
    pub F17R2: u32,
    pub F18R1: u32,
    pub F18R2: u32,
    pub F19R1: u32,
    pub F19R2: u32,
    pub F20R1: u32,
    pub F20R2: u32,
    pub F21R1: u32,
    pub F21R2: u32,
    pub F22R1: u32,
    pub F22R2: u32,
    pub F23R1: u32,
    pub F23R2: u32,
    pub F24R1: u32,
    pub F24R2: u32,
    pub F25R1: u32,
    pub F25R2: u32,
    pub F26R1: u32,
    pub F26R2: u32,
    pub F27R1: u32,
    pub F27R2: u32,
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

/// Access functions for the CAN1 peripheral instance
pub mod CAN1 {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x40006400,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in CAN1
    pub const reset: ResetValues = ResetValues {
        MCR: 0x00010002,
        MSR: 0x00000C02,
        TSR: 0x1C000000,
        RF0R: 0x00000000,
        RF1R: 0x00000000,
        IER: 0x00000000,
        ESR: 0x00000000,
        BTR: 0x00000000,
        TI0R: 0x00000000,
        TDT0R: 0x00000000,
        TDL0R: 0x00000000,
        TDH0R: 0x00000000,
        TI1R: 0x00000000,
        TDT1R: 0x00000000,
        TDL1R: 0x00000000,
        TDH1R: 0x00000000,
        TI2R: 0x00000000,
        TDT2R: 0x00000000,
        TDL2R: 0x00000000,
        TDH2R: 0x00000000,
        RI0R: 0x00000000,
        RDT0R: 0x00000000,
        RDL0R: 0x00000000,
        RDH0R: 0x00000000,
        RI1R: 0x00000000,
        RDT1R: 0x00000000,
        RDL1R: 0x00000000,
        RDH1R: 0x00000000,
        FS1R: 0x00000000,
        FFA1R: 0x00000000,
        FA1R: 0x00000000,
        F0R1: 0x00000000,
        F0R2: 0x00000000,
        F1R1: 0x00000000,
        F1R2: 0x00000000,
        F2R1: 0x00000000,
        F2R2: 0x00000000,
        F3R1: 0x00000000,
        F3R2: 0x00000000,
        F4R1: 0x00000000,
        F4R2: 0x00000000,
        F5R1: 0x00000000,
        F5R2: 0x00000000,
        F6R1: 0x00000000,
        F6R2: 0x00000000,
        F7R1: 0x00000000,
        F7R2: 0x00000000,
        F8R1: 0x00000000,
        F8R2: 0x00000000,
        F9R1: 0x00000000,
        F9R2: 0x00000000,
        F10R1: 0x00000000,
        F10R2: 0x00000000,
        F11R1: 0x00000000,
        F11R2: 0x00000000,
        F12R1: 0x00000000,
        F12R2: 0x00000000,
        F13R1: 0x00000000,
        F13R2: 0x00000000,
        F14R1: 0x00000000,
        F14R2: 0x00000000,
        F15R1: 0x00000000,
        F15R2: 0x00000000,
        F16R1: 0x00000000,
        F16R2: 0x00000000,
        F17R1: 0x00000000,
        F17R2: 0x00000000,
        F18R1: 0x00000000,
        F18R2: 0x00000000,
        F19R1: 0x00000000,
        F19R2: 0x00000000,
        F20R1: 0x00000000,
        F20R2: 0x00000000,
        F21R1: 0x00000000,
        F21R2: 0x00000000,
        F22R1: 0x00000000,
        F22R2: 0x00000000,
        F23R1: 0x00000000,
        F23R2: 0x00000000,
        F24R1: 0x00000000,
        F24R2: 0x00000000,
        F25R1: 0x00000000,
        F25R2: 0x00000000,
        F26R1: 0x00000000,
        F26R2: 0x00000000,
        F27R1: 0x00000000,
        F27R2: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut CAN1_TAKEN: bool = false;

    /// Safe access to CAN1
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
            if CAN1_TAKEN {
                None
            } else {
                CAN1_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to CAN1
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if CAN1_TAKEN && inst.addr == INSTANCE.addr {
                CAN1_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal CAN1
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        CAN1_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to CAN1
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const CAN1: *const RegisterBlock = 0x40006400 as *const _;
