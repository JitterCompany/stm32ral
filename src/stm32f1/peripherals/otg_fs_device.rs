#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! USB on the go full speed
//!
//! Used by: stm32f101, stm32f102, stm32f103, stm32f107

use crate::{RORegister, RWRegister};
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// OTG_FS device configuration register (OTG_FS_DCFG)
pub mod DCFG {

    /// Device speed
    pub mod DSPD {
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

    /// Non-zero-length status OUT handshake
    pub mod NZLSOHSK {
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

    /// Device address
    pub mod DAD {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (7 bits: 0x7f << 4)
        pub const mask: u32 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Periodic frame interval
    pub mod PFIVL {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (2 bits: 0b11 << 11)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// OTG_FS device control register (OTG_FS_DCTL)
pub mod DCTL {

    /// Remote wakeup signaling
    pub mod RWUSIG {
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

    /// Soft disconnect
    pub mod SDIS {
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

    /// Global IN NAK status
    pub mod GINSTS {
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

    /// Global OUT NAK status
    pub mod GONSTS {
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

    /// Test control
    pub mod TCTL {
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

    /// Set global IN NAK
    pub mod SGINAK {
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

    /// Clear global IN NAK
    pub mod CGINAK {
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

    /// Set global OUT NAK
    pub mod SGONAK {
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

    /// Clear global OUT NAK
    pub mod CGONAK {
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

    /// Power-on programming done
    pub mod POPRGDNE {
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
}

/// OTG_FS device status register (OTG_FS_DSTS)
pub mod DSTS {

    /// Suspend status
    pub mod SUSPSTS {
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

    /// Enumerated speed
    pub mod ENUMSPD {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (2 bits: 0b11 << 1)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Erratic error
    pub mod EERR {
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

    /// Frame number of the received SOF
    pub mod FNSOF {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (14 bits: 0x3fff << 8)
        pub const mask: u32 = 0x3fff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// OTG_FS device IN endpoint common interrupt mask register (OTG_FS_DIEPMSK)
pub mod DIEPMSK {

    /// Transfer completed interrupt mask
    pub mod XFRCM {
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

    /// Endpoint disabled interrupt mask
    pub mod EPDM {
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

    /// Timeout condition mask (Non-isochronous endpoints)
    pub mod TOM {
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

    /// IN token received when TxFIFO empty mask
    pub mod ITTXFEMSK {
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

    /// IN token received with EP mismatch mask
    pub mod INEPNMM {
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

    /// IN endpoint NAK effective mask
    pub mod INEPNEM {
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
}

/// OTG_FS device OUT endpoint common interrupt mask register (OTG_FS_DOEPMSK)
pub mod DOEPMSK {

    /// Transfer completed interrupt mask
    pub mod XFRCM {
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

    /// Endpoint disabled interrupt mask
    pub mod EPDM {
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

    /// SETUP phase done mask
    pub mod STUPM {
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

    /// OUT token received when endpoint disabled mask
    pub mod OTEPDM {
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

/// OTG_FS device all endpoints interrupt register (OTG_FS_DAINT)
pub mod DAINT {

    /// IN endpoint interrupt bits
    pub mod IEPINT {
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

    /// OUT endpoint interrupt bits
    pub mod OEPINT {
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

/// OTG_FS all endpoints interrupt mask register (OTG_FS_DAINTMSK)
pub mod DAINTMSK {

    /// IN EP interrupt mask bits
    pub mod IEPM {
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

    /// OUT EP interrupt mask bits
    pub mod OEPM {
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

/// OTG_FS device VBUS discharge time register
pub mod DVBUSDIS {

    /// Device VBUS discharge time
    pub mod VBUSDT {
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

/// OTG_FS device VBUS pulsing time register
pub mod DVBUSPULSE {

    /// Device VBUS pulsing time
    pub mod DVBUSP {
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
}

/// OTG_FS device IN endpoint FIFO empty interrupt mask register
pub mod DIEPEMPMSK {

    /// IN EP Tx FIFO empty interrupt mask bits
    pub mod INEPTXFEM {
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

/// OTG_FS device control IN endpoint 0 control register (OTG_FS_DIEPCTL0)
pub mod CTL {

    /// Maximum packet size
    pub mod MPSIZ {
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

    /// USB active endpoint
    pub mod USBAEP {
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

    /// NAK status
    pub mod NAKSTS {
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

    /// Endpoint type
    pub mod EPTYP {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (2 bits: 0b11 << 18)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// STALL handshake
    pub mod STALL {
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

    /// TxFIFO number
    pub mod TXFNUM {
        /// Offset (22 bits)
        pub const offset: u32 = 22;
        /// Mask (4 bits: 0b1111 << 22)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Clear NAK
    pub mod CNAK {
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

    /// Set NAK
    pub mod SNAK {
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

    /// Endpoint disable
    pub mod EPDIS {
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

    /// Endpoint enable
    pub mod EPENA {
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

/// device endpoint-x interrupt register
pub mod INT {

    /// TXFE
    pub mod TXFE {
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

    /// INEPNE
    pub mod INEPNE {
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

    /// ITTXFE
    pub mod ITTXFE {
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

    /// TOC
    pub mod TOC {
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

    /// EPDISD
    pub mod EPDISD {
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

    /// XFRC
    pub mod XFRC {
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

/// device endpoint-0 transfer size register
pub mod TSIZ {

    /// Packet count
    pub mod PKTCNT {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (2 bits: 0b11 << 19)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Transfer size
    pub mod XFRSIZ {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (7 bits: 0x7f << 0)
        pub const mask: u32 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// OTG_FS device IN endpoint transmit FIFO status register
pub mod TXFSTS {

    /// IN endpoint TxFIFO space available
    pub mod INEPTFSAV {
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

/// OTG device endpoint-1 control register
pub mod CTL1 {

    /// EPENA
    pub mod EPENA {
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

    /// EPDIS
    pub mod EPDIS {
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

    /// SODDFRM/SD1PID
    pub mod SODDFRM_SD1PID {
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

    /// SD0PID/SEVNFRM
    pub mod SD0PID_SEVNFRM {
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

    /// SNAK
    pub mod SNAK {
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

    /// CNAK
    pub mod CNAK {
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

    /// TXFNUM
    pub mod TXFNUM {
        /// Offset (22 bits)
        pub const offset: u32 = 22;
        /// Mask (4 bits: 0b1111 << 22)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// STALL handshake
    pub mod STALL {
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

    /// EPTYP
    pub mod EPTYP {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (2 bits: 0b11 << 18)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// NAKSTS
    pub mod NAKSTS {
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

    /// EONUM/DPID
    pub mod EONUM_DPID {
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

    /// USBAEP
    pub mod USBAEP {
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

    /// MPSIZ
    pub mod MPSIZ {
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

/// device endpoint-1 interrupt register
pub mod INT1 {
    pub use super::INT::EPDISD;
    pub use super::INT::INEPNE;
    pub use super::INT::ITTXFE;
    pub use super::INT::TOC;
    pub use super::INT::TXFE;
    pub use super::INT::XFRC;
}

/// device endpoint-1 transfer size register
pub mod TSIZ1 {

    /// Multi count
    pub mod MCNT {
        /// Offset (29 bits)
        pub const offset: u32 = 29;
        /// Mask (2 bits: 0b11 << 29)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Packet count
    pub mod PKTCNT {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (10 bits: 0x3ff << 19)
        pub const mask: u32 = 0x3ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Transfer size
    pub mod XFRSIZ {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (19 bits: 0x7ffff << 0)
        pub const mask: u32 = 0x7ffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// OTG_FS device IN endpoint transmit FIFO status register
pub mod TXFSTS1 {
    pub use super::TXFSTS::INEPTFSAV;
}

/// OTG device endpoint-1 control register
pub mod CTL2 {
    pub use super::CTL1::CNAK;
    pub use super::CTL1::EONUM_DPID;
    pub use super::CTL1::EPDIS;
    pub use super::CTL1::EPENA;
    pub use super::CTL1::EPTYP;
    pub use super::CTL1::MPSIZ;
    pub use super::CTL1::NAKSTS;
    pub use super::CTL1::SD0PID_SEVNFRM;
    pub use super::CTL1::SNAK;
    pub use super::CTL1::SODDFRM_SD1PID;
    pub use super::CTL1::STALL;
    pub use super::CTL1::TXFNUM;
    pub use super::CTL1::USBAEP;
}

/// device endpoint-1 interrupt register
pub mod INT2 {
    pub use super::INT::EPDISD;
    pub use super::INT::INEPNE;
    pub use super::INT::ITTXFE;
    pub use super::INT::TOC;
    pub use super::INT::TXFE;
    pub use super::INT::XFRC;
}

/// device endpoint-1 transfer size register
pub mod TSIZ2 {
    pub use super::TSIZ1::MCNT;
    pub use super::TSIZ1::PKTCNT;
    pub use super::TSIZ1::XFRSIZ;
}

/// OTG_FS device IN endpoint transmit FIFO status register
pub mod TXFSTS2 {
    pub use super::TXFSTS::INEPTFSAV;
}

/// OTG device endpoint-1 control register
pub mod CTL3 {
    pub use super::CTL1::CNAK;
    pub use super::CTL1::EONUM_DPID;
    pub use super::CTL1::EPDIS;
    pub use super::CTL1::EPENA;
    pub use super::CTL1::EPTYP;
    pub use super::CTL1::MPSIZ;
    pub use super::CTL1::NAKSTS;
    pub use super::CTL1::SD0PID_SEVNFRM;
    pub use super::CTL1::SNAK;
    pub use super::CTL1::SODDFRM_SD1PID;
    pub use super::CTL1::STALL;
    pub use super::CTL1::TXFNUM;
    pub use super::CTL1::USBAEP;
}

/// device endpoint-1 interrupt register
pub mod INT3 {
    pub use super::INT::EPDISD;
    pub use super::INT::INEPNE;
    pub use super::INT::ITTXFE;
    pub use super::INT::TOC;
    pub use super::INT::TXFE;
    pub use super::INT::XFRC;
}

/// device endpoint-1 transfer size register
pub mod TSIZ3 {
    pub use super::TSIZ1::MCNT;
    pub use super::TSIZ1::PKTCNT;
    pub use super::TSIZ1::XFRSIZ;
}

/// OTG_FS device IN endpoint transmit FIFO status register
pub mod TXFSTS3 {
    pub use super::TXFSTS::INEPTFSAV;
}

/// device endpoint-0 control register
pub mod CTL {

    /// EPENA
    pub mod EPENA {
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

    /// EPDIS
    pub mod EPDIS {
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

    /// SNAK
    pub mod SNAK {
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

    /// CNAK
    pub mod CNAK {
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

    /// STALL handshake
    pub mod STALL {
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

    /// SNPM
    pub mod SNPM {
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

    /// EPTYP
    pub mod EPTYP {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (2 bits: 0b11 << 18)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// NAKSTS
    pub mod NAKSTS {
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

    /// USBAEP
    pub mod USBAEP {
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

    /// MPSIZ
    pub mod MPSIZ {
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

/// device endpoint-0 interrupt register
pub mod INT {

    /// B2BSTUP
    pub mod B2BSTUP {
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

    /// OTEPDIS
    pub mod OTEPDIS {
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

    /// STUP
    pub mod STUP {
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

    /// EPDISD
    pub mod EPDISD {
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

    /// XFRC
    pub mod XFRC {
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

/// device OUT endpoint-0 transfer size register
pub mod TSIZ {

    /// SETUP packet count
    pub mod STUPCNT {
        /// Offset (29 bits)
        pub const offset: u32 = 29;
        /// Mask (2 bits: 0b11 << 29)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Packet count
    pub mod PKTCNT {
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

    /// Transfer size
    pub mod XFRSIZ {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (7 bits: 0x7f << 0)
        pub const mask: u32 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// device endpoint-1 control register
pub mod CTL1 {

    /// EPENA
    pub mod EPENA {
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

    /// EPDIS
    pub mod EPDIS {
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

    /// SODDFRM
    pub mod SODDFRM {
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

    /// SD0PID/SEVNFRM
    pub mod SD0PID_SEVNFRM {
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

    /// SNAK
    pub mod SNAK {
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

    /// CNAK
    pub mod CNAK {
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

    /// STALL handshake
    pub mod STALL {
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

    /// SNPM
    pub mod SNPM {
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

    /// EPTYP
    pub mod EPTYP {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (2 bits: 0b11 << 18)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// NAKSTS
    pub mod NAKSTS {
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

    /// EONUM/DPID
    pub mod EONUM_DPID {
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

    /// USBAEP
    pub mod USBAEP {
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

    /// MPSIZ
    pub mod MPSIZ {
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

/// device endpoint-1 interrupt register
pub mod INT1 {
    pub use super::INT::B2BSTUP;
    pub use super::INT::EPDISD;
    pub use super::INT::OTEPDIS;
    pub use super::INT::STUP;
    pub use super::INT::XFRC;
}

/// device OUT endpoint-1 transfer size register
pub mod TSIZ1 {

    /// Received data PID/SETUP packet count
    pub mod RXDPID_STUPCNT {
        /// Offset (29 bits)
        pub const offset: u32 = 29;
        /// Mask (2 bits: 0b11 << 29)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Packet count
    pub mod PKTCNT {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (10 bits: 0x3ff << 19)
        pub const mask: u32 = 0x3ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Transfer size
    pub mod XFRSIZ {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (19 bits: 0x7ffff << 0)
        pub const mask: u32 = 0x7ffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// device endpoint-1 control register
pub mod CTL2 {
    pub use super::CTL1::CNAK;
    pub use super::CTL1::EONUM_DPID;
    pub use super::CTL1::EPDIS;
    pub use super::CTL1::EPENA;
    pub use super::CTL1::EPTYP;
    pub use super::CTL1::MPSIZ;
    pub use super::CTL1::NAKSTS;
    pub use super::CTL1::SD0PID_SEVNFRM;
    pub use super::CTL1::SNAK;
    pub use super::CTL1::SNPM;
    pub use super::CTL1::SODDFRM;
    pub use super::CTL1::STALL;
    pub use super::CTL1::USBAEP;
}

/// device endpoint-1 interrupt register
pub mod INT2 {
    pub use super::INT::B2BSTUP;
    pub use super::INT::EPDISD;
    pub use super::INT::OTEPDIS;
    pub use super::INT::STUP;
    pub use super::INT::XFRC;
}

/// device OUT endpoint-1 transfer size register
pub mod TSIZ2 {
    pub use super::TSIZ1::PKTCNT;
    pub use super::TSIZ1::RXDPID_STUPCNT;
    pub use super::TSIZ1::XFRSIZ;
}

/// device endpoint-1 control register
pub mod CTL3 {
    pub use super::CTL1::CNAK;
    pub use super::CTL1::EONUM_DPID;
    pub use super::CTL1::EPDIS;
    pub use super::CTL1::EPENA;
    pub use super::CTL1::EPTYP;
    pub use super::CTL1::MPSIZ;
    pub use super::CTL1::NAKSTS;
    pub use super::CTL1::SD0PID_SEVNFRM;
    pub use super::CTL1::SNAK;
    pub use super::CTL1::SNPM;
    pub use super::CTL1::SODDFRM;
    pub use super::CTL1::STALL;
    pub use super::CTL1::USBAEP;
}

/// device endpoint-1 interrupt register
pub mod INT3 {
    pub use super::INT::B2BSTUP;
    pub use super::INT::EPDISD;
    pub use super::INT::OTEPDIS;
    pub use super::INT::STUP;
    pub use super::INT::XFRC;
}

/// device OUT endpoint-1 transfer size register
pub mod TSIZ3 {
    pub use super::TSIZ1::PKTCNT;
    pub use super::TSIZ1::RXDPID_STUPCNT;
    pub use super::TSIZ1::XFRSIZ;
}
#[repr(C)]
pub struct RegisterBlock {
    /// OTG_FS device configuration register (OTG_FS_DCFG)
    pub DCFG: RWRegister<u32>,

    /// OTG_FS device control register (OTG_FS_DCTL)
    pub DCTL: RWRegister<u32>,

    /// OTG_FS device status register (OTG_FS_DSTS)
    pub DSTS: RORegister<u32>,

    _reserved1: [u8; 4],

    /// OTG_FS device IN endpoint common interrupt mask register (OTG_FS_DIEPMSK)
    pub DIEPMSK: RWRegister<u32>,

    /// OTG_FS device OUT endpoint common interrupt mask register (OTG_FS_DOEPMSK)
    pub DOEPMSK: RWRegister<u32>,

    /// OTG_FS device all endpoints interrupt register (OTG_FS_DAINT)
    pub DAINT: RORegister<u32>,

    /// OTG_FS all endpoints interrupt mask register (OTG_FS_DAINTMSK)
    pub DAINTMSK: RWRegister<u32>,

    _reserved2: [u8; 8],

    /// OTG_FS device VBUS discharge time register
    pub DVBUSDIS: RWRegister<u32>,

    /// OTG_FS device VBUS pulsing time register
    pub DVBUSPULSE: RWRegister<u32>,

    _reserved3: [u8; 4],

    /// OTG_FS device IN endpoint FIFO empty interrupt mask register
    pub DIEPEMPMSK: RWRegister<u32>,

    _reserved4: [u8; 200],

    /// OTG_FS device control IN endpoint 0 control register (OTG_FS_DIEPCTL0)
    pub CTL: RWRegister<u32>,

    _reserved5: [u8; 4],

    /// device endpoint-x interrupt register
    pub INT: RWRegister<u32>,

    _reserved6: [u8; 4],

    /// device endpoint-0 transfer size register
    pub TSIZ: RWRegister<u32>,

    _reserved7: [u8; 4],

    /// OTG_FS device IN endpoint transmit FIFO status register
    pub TXFSTS: RORegister<u32>,

    _reserved8: [u8; 4],

    /// OTG device endpoint-1 control register
    pub CTL1: RWRegister<u32>,

    _reserved9: [u8; 4],

    /// device endpoint-1 interrupt register
    pub INT1: RWRegister<u32>,

    _reserved10: [u8; 4],

    /// device endpoint-1 transfer size register
    pub TSIZ1: RWRegister<u32>,

    _reserved11: [u8; 4],

    /// OTG_FS device IN endpoint transmit FIFO status register
    pub TXFSTS1: RORegister<u32>,

    _reserved12: [u8; 4],

    /// OTG device endpoint-1 control register
    pub CTL2: RWRegister<u32>,

    _reserved13: [u8; 4],

    /// device endpoint-1 interrupt register
    pub INT2: RWRegister<u32>,

    _reserved14: [u8; 4],

    /// device endpoint-1 transfer size register
    pub TSIZ2: RWRegister<u32>,

    _reserved15: [u8; 4],

    /// OTG_FS device IN endpoint transmit FIFO status register
    pub TXFSTS2: RORegister<u32>,

    _reserved16: [u8; 4],

    /// OTG device endpoint-1 control register
    pub CTL3: RWRegister<u32>,

    _reserved17: [u8; 4],

    /// device endpoint-1 interrupt register
    pub INT3: RWRegister<u32>,

    _reserved18: [u8; 4],

    /// device endpoint-1 transfer size register
    pub TSIZ3: RWRegister<u32>,

    _reserved19: [u8; 4],

    /// OTG_FS device IN endpoint transmit FIFO status register
    pub TXFSTS3: RORegister<u32>,

    _reserved20: [u8; 388],

    /// device endpoint-0 control register
    pub CTL: RWRegister<u32>,

    _reserved21: [u8; 4],

    /// device endpoint-0 interrupt register
    pub INT: RWRegister<u32>,

    _reserved22: [u8; 4],

    /// device OUT endpoint-0 transfer size register
    pub TSIZ: RWRegister<u32>,

    _reserved23: [u8; 12],

    /// device endpoint-1 control register
    pub CTL1: RWRegister<u32>,

    _reserved24: [u8; 4],

    /// device endpoint-1 interrupt register
    pub INT1: RWRegister<u32>,

    _reserved25: [u8; 4],

    /// device OUT endpoint-1 transfer size register
    pub TSIZ1: RWRegister<u32>,

    _reserved26: [u8; 12],

    /// device endpoint-1 control register
    pub CTL2: RWRegister<u32>,

    _reserved27: [u8; 4],

    /// device endpoint-1 interrupt register
    pub INT2: RWRegister<u32>,

    _reserved28: [u8; 4],

    /// device OUT endpoint-1 transfer size register
    pub TSIZ2: RWRegister<u32>,

    _reserved29: [u8; 12],

    /// device endpoint-1 control register
    pub CTL3: RWRegister<u32>,

    _reserved30: [u8; 4],

    /// device endpoint-1 interrupt register
    pub INT3: RWRegister<u32>,

    _reserved31: [u8; 4],

    /// device OUT endpoint-1 transfer size register
    pub TSIZ3: RWRegister<u32>,
}
pub struct ResetValues {
    pub DCFG: u32,
    pub DCTL: u32,
    pub DSTS: u32,
    pub DIEPMSK: u32,
    pub DOEPMSK: u32,
    pub DAINT: u32,
    pub DAINTMSK: u32,
    pub DVBUSDIS: u32,
    pub DVBUSPULSE: u32,
    pub DIEPEMPMSK: u32,
    pub CTL: u32,
    pub INT: u32,
    pub TSIZ: u32,
    pub TXFSTS: u32,
    pub CTL1: u32,
    pub INT1: u32,
    pub TSIZ1: u32,
    pub TXFSTS1: u32,
    pub CTL2: u32,
    pub INT2: u32,
    pub TSIZ2: u32,
    pub TXFSTS2: u32,
    pub CTL3: u32,
    pub INT3: u32,
    pub TSIZ3: u32,
    pub TXFSTS3: u32,
    pub CTL: u32,
    pub INT: u32,
    pub TSIZ: u32,
    pub CTL1: u32,
    pub INT1: u32,
    pub TSIZ1: u32,
    pub CTL2: u32,
    pub INT2: u32,
    pub TSIZ2: u32,
    pub CTL3: u32,
    pub INT3: u32,
    pub TSIZ3: u32,
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
