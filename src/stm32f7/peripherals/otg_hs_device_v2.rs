#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! USB on the go high speed
//!
//! Used by: stm32f7x2, stm32f7x3

use crate::{RORegister, RWRegister};
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// OTG_HS device configuration register
pub mod OTG_HS_DCFG {

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

    /// Nonzero-length status OUT handshake
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

    /// Periodic (micro)frame interval
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

    /// Periodic scheduling interval
    pub mod PERSCHIVL {
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
}

/// OTG_HS device control register
pub mod OTG_HS_DCTL {

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

/// OTG_HS device status register
pub mod OTG_HS_DSTS {

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

/// OTG_HS device IN endpoint common interrupt mask register
pub mod OTG_HS_DIEPMSK {

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

    /// Timeout condition mask (nonisochronous endpoints)
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

    /// FIFO underrun mask
    pub mod TXFURM {
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

    /// BNA interrupt mask
    pub mod BIM {
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

/// OTG_HS device OUT endpoint common interrupt mask register
pub mod OTG_HS_DOEPMSK {

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

    /// Back-to-back SETUP packets received mask
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

    /// OUT packet error mask
    pub mod OPEM {
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

    /// BNA interrupt mask
    pub mod BOIM {
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

/// OTG_HS device all endpoints interrupt register
pub mod OTG_HS_DAINT {

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

/// OTG_HS all endpoints interrupt mask register
pub mod OTG_HS_DAINTMSK {

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

/// OTG_HS device VBUS discharge time register
pub mod OTG_HS_DVBUSDIS {

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

/// OTG_HS device VBUS pulsing time register
pub mod OTG_HS_DVBUSPULSE {

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

/// OTG_HS Device threshold control register
pub mod OTG_HS_DTHRCTL {

    /// Nonisochronous IN endpoints threshold enable
    pub mod NONISOTHREN {
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

    /// ISO IN endpoint threshold enable
    pub mod ISOTHREN {
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

    /// Transmit threshold length
    pub mod TXTHRLEN {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (9 bits: 0x1ff << 2)
        pub const mask: u32 = 0x1ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Receive threshold enable
    pub mod RXTHREN {
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

    /// Receive threshold length
    pub mod RXTHRLEN {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (9 bits: 0x1ff << 17)
        pub const mask: u32 = 0x1ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Arbiter parking enable
    pub mod ARPEN {
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
}

/// OTG_HS device IN endpoint FIFO empty interrupt mask register
pub mod OTG_HS_DIEPEMPMSK {

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

/// OTG_HS device each endpoint interrupt register
pub mod OTG_HS_DEACHINT {

    /// IN endpoint 1interrupt bit
    pub mod IEP1INT {
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

    /// OUT endpoint 1 interrupt bit
    pub mod OEP1INT {
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
}

/// OTG_HS device each endpoint interrupt register mask
pub mod OTG_HS_DEACHINTMSK {

    /// IN Endpoint 1 interrupt mask bit
    pub mod IEP1INTM {
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

    /// OUT Endpoint 1 interrupt mask bit
    pub mod OEP1INTM {
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
}

/// OTG device endpoint-0 control register
pub mod OTG_HS_DIEPCTL0 {

    /// Maximum packet size
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

    /// Even/odd frame
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
    pub mod Stall {
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

    /// Set DATA0 PID
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

    /// Set odd frame
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

/// OTG device endpoint-1 control register
pub mod OTG_HS_DIEPCTL1 {
    pub use super::OTG_HS_DIEPCTL0::Stall;
    pub use super::OTG_HS_DIEPCTL0::CNAK;
    pub use super::OTG_HS_DIEPCTL0::EONUM_DPID;
    pub use super::OTG_HS_DIEPCTL0::EPDIS;
    pub use super::OTG_HS_DIEPCTL0::EPENA;
    pub use super::OTG_HS_DIEPCTL0::EPTYP;
    pub use super::OTG_HS_DIEPCTL0::MPSIZ;
    pub use super::OTG_HS_DIEPCTL0::NAKSTS;
    pub use super::OTG_HS_DIEPCTL0::SD0PID_SEVNFRM;
    pub use super::OTG_HS_DIEPCTL0::SNAK;
    pub use super::OTG_HS_DIEPCTL0::SODDFRM;
    pub use super::OTG_HS_DIEPCTL0::TXFNUM;
    pub use super::OTG_HS_DIEPCTL0::USBAEP;
}

/// OTG device endpoint-2 control register
pub mod OTG_HS_DIEPCTL2 {
    pub use super::OTG_HS_DIEPCTL0::Stall;
    pub use super::OTG_HS_DIEPCTL0::CNAK;
    pub use super::OTG_HS_DIEPCTL0::EONUM_DPID;
    pub use super::OTG_HS_DIEPCTL0::EPDIS;
    pub use super::OTG_HS_DIEPCTL0::EPENA;
    pub use super::OTG_HS_DIEPCTL0::EPTYP;
    pub use super::OTG_HS_DIEPCTL0::MPSIZ;
    pub use super::OTG_HS_DIEPCTL0::NAKSTS;
    pub use super::OTG_HS_DIEPCTL0::SD0PID_SEVNFRM;
    pub use super::OTG_HS_DIEPCTL0::SNAK;
    pub use super::OTG_HS_DIEPCTL0::SODDFRM;
    pub use super::OTG_HS_DIEPCTL0::TXFNUM;
    pub use super::OTG_HS_DIEPCTL0::USBAEP;
}

/// OTG device endpoint-3 control register
pub mod OTG_HS_DIEPCTL3 {
    pub use super::OTG_HS_DIEPCTL0::Stall;
    pub use super::OTG_HS_DIEPCTL0::CNAK;
    pub use super::OTG_HS_DIEPCTL0::EONUM_DPID;
    pub use super::OTG_HS_DIEPCTL0::EPDIS;
    pub use super::OTG_HS_DIEPCTL0::EPENA;
    pub use super::OTG_HS_DIEPCTL0::EPTYP;
    pub use super::OTG_HS_DIEPCTL0::MPSIZ;
    pub use super::OTG_HS_DIEPCTL0::NAKSTS;
    pub use super::OTG_HS_DIEPCTL0::SD0PID_SEVNFRM;
    pub use super::OTG_HS_DIEPCTL0::SNAK;
    pub use super::OTG_HS_DIEPCTL0::SODDFRM;
    pub use super::OTG_HS_DIEPCTL0::TXFNUM;
    pub use super::OTG_HS_DIEPCTL0::USBAEP;
}

/// OTG device endpoint-4 control register
pub mod OTG_HS_DIEPCTL4 {
    pub use super::OTG_HS_DIEPCTL0::Stall;
    pub use super::OTG_HS_DIEPCTL0::CNAK;
    pub use super::OTG_HS_DIEPCTL0::EONUM_DPID;
    pub use super::OTG_HS_DIEPCTL0::EPDIS;
    pub use super::OTG_HS_DIEPCTL0::EPENA;
    pub use super::OTG_HS_DIEPCTL0::EPTYP;
    pub use super::OTG_HS_DIEPCTL0::MPSIZ;
    pub use super::OTG_HS_DIEPCTL0::NAKSTS;
    pub use super::OTG_HS_DIEPCTL0::SD0PID_SEVNFRM;
    pub use super::OTG_HS_DIEPCTL0::SNAK;
    pub use super::OTG_HS_DIEPCTL0::SODDFRM;
    pub use super::OTG_HS_DIEPCTL0::TXFNUM;
    pub use super::OTG_HS_DIEPCTL0::USBAEP;
}

/// OTG device endpoint-5 control register
pub mod OTG_HS_DIEPCTL5 {
    pub use super::OTG_HS_DIEPCTL0::Stall;
    pub use super::OTG_HS_DIEPCTL0::CNAK;
    pub use super::OTG_HS_DIEPCTL0::EONUM_DPID;
    pub use super::OTG_HS_DIEPCTL0::EPDIS;
    pub use super::OTG_HS_DIEPCTL0::EPENA;
    pub use super::OTG_HS_DIEPCTL0::EPTYP;
    pub use super::OTG_HS_DIEPCTL0::MPSIZ;
    pub use super::OTG_HS_DIEPCTL0::NAKSTS;
    pub use super::OTG_HS_DIEPCTL0::SD0PID_SEVNFRM;
    pub use super::OTG_HS_DIEPCTL0::SNAK;
    pub use super::OTG_HS_DIEPCTL0::SODDFRM;
    pub use super::OTG_HS_DIEPCTL0::TXFNUM;
    pub use super::OTG_HS_DIEPCTL0::USBAEP;
}

/// OTG device endpoint-6 control register
pub mod OTG_HS_DIEPCTL6 {
    pub use super::OTG_HS_DIEPCTL0::Stall;
    pub use super::OTG_HS_DIEPCTL0::CNAK;
    pub use super::OTG_HS_DIEPCTL0::EONUM_DPID;
    pub use super::OTG_HS_DIEPCTL0::EPDIS;
    pub use super::OTG_HS_DIEPCTL0::EPENA;
    pub use super::OTG_HS_DIEPCTL0::EPTYP;
    pub use super::OTG_HS_DIEPCTL0::MPSIZ;
    pub use super::OTG_HS_DIEPCTL0::NAKSTS;
    pub use super::OTG_HS_DIEPCTL0::SD0PID_SEVNFRM;
    pub use super::OTG_HS_DIEPCTL0::SNAK;
    pub use super::OTG_HS_DIEPCTL0::SODDFRM;
    pub use super::OTG_HS_DIEPCTL0::TXFNUM;
    pub use super::OTG_HS_DIEPCTL0::USBAEP;
}

/// OTG device endpoint-7 control register
pub mod OTG_HS_DIEPCTL7 {
    pub use super::OTG_HS_DIEPCTL0::Stall;
    pub use super::OTG_HS_DIEPCTL0::CNAK;
    pub use super::OTG_HS_DIEPCTL0::EONUM_DPID;
    pub use super::OTG_HS_DIEPCTL0::EPDIS;
    pub use super::OTG_HS_DIEPCTL0::EPENA;
    pub use super::OTG_HS_DIEPCTL0::EPTYP;
    pub use super::OTG_HS_DIEPCTL0::MPSIZ;
    pub use super::OTG_HS_DIEPCTL0::NAKSTS;
    pub use super::OTG_HS_DIEPCTL0::SD0PID_SEVNFRM;
    pub use super::OTG_HS_DIEPCTL0::SNAK;
    pub use super::OTG_HS_DIEPCTL0::SODDFRM;
    pub use super::OTG_HS_DIEPCTL0::TXFNUM;
    pub use super::OTG_HS_DIEPCTL0::USBAEP;
}

/// OTG device endpoint-0 interrupt register
pub mod OTG_HS_DIEPINT0 {

    /// Transfer completed interrupt
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

    /// Endpoint disabled interrupt
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

    /// Timeout condition
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

    /// IN token received when TxFIFO is empty
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

    /// IN endpoint NAK effective
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

    /// Transmit FIFO empty
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

    /// Transmit Fifo Underrun
    pub mod TXFIFOUDRN {
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

    /// Buffer not available interrupt
    pub mod BNA {
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

    /// Packet dropped status
    pub mod PKTDRPSTS {
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

    /// Babble error interrupt
    pub mod BERR {
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

    /// NAK interrupt
    pub mod NAK {
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
}

/// OTG device endpoint-1 interrupt register
pub mod OTG_HS_DIEPINT1 {
    pub use super::OTG_HS_DIEPINT0::BERR;
    pub use super::OTG_HS_DIEPINT0::BNA;
    pub use super::OTG_HS_DIEPINT0::EPDISD;
    pub use super::OTG_HS_DIEPINT0::INEPNE;
    pub use super::OTG_HS_DIEPINT0::ITTXFE;
    pub use super::OTG_HS_DIEPINT0::NAK;
    pub use super::OTG_HS_DIEPINT0::PKTDRPSTS;
    pub use super::OTG_HS_DIEPINT0::TOC;
    pub use super::OTG_HS_DIEPINT0::TXFE;
    pub use super::OTG_HS_DIEPINT0::TXFIFOUDRN;
    pub use super::OTG_HS_DIEPINT0::XFRC;
}

/// OTG device endpoint-2 interrupt register
pub mod OTG_HS_DIEPINT2 {
    pub use super::OTG_HS_DIEPINT0::BERR;
    pub use super::OTG_HS_DIEPINT0::BNA;
    pub use super::OTG_HS_DIEPINT0::EPDISD;
    pub use super::OTG_HS_DIEPINT0::INEPNE;
    pub use super::OTG_HS_DIEPINT0::ITTXFE;
    pub use super::OTG_HS_DIEPINT0::NAK;
    pub use super::OTG_HS_DIEPINT0::PKTDRPSTS;
    pub use super::OTG_HS_DIEPINT0::TOC;
    pub use super::OTG_HS_DIEPINT0::TXFE;
    pub use super::OTG_HS_DIEPINT0::TXFIFOUDRN;
    pub use super::OTG_HS_DIEPINT0::XFRC;
}

/// OTG device endpoint-3 interrupt register
pub mod OTG_HS_DIEPINT3 {
    pub use super::OTG_HS_DIEPINT0::BERR;
    pub use super::OTG_HS_DIEPINT0::BNA;
    pub use super::OTG_HS_DIEPINT0::EPDISD;
    pub use super::OTG_HS_DIEPINT0::INEPNE;
    pub use super::OTG_HS_DIEPINT0::ITTXFE;
    pub use super::OTG_HS_DIEPINT0::NAK;
    pub use super::OTG_HS_DIEPINT0::PKTDRPSTS;
    pub use super::OTG_HS_DIEPINT0::TOC;
    pub use super::OTG_HS_DIEPINT0::TXFE;
    pub use super::OTG_HS_DIEPINT0::TXFIFOUDRN;
    pub use super::OTG_HS_DIEPINT0::XFRC;
}

/// OTG device endpoint-4 interrupt register
pub mod OTG_HS_DIEPINT4 {
    pub use super::OTG_HS_DIEPINT0::BERR;
    pub use super::OTG_HS_DIEPINT0::BNA;
    pub use super::OTG_HS_DIEPINT0::EPDISD;
    pub use super::OTG_HS_DIEPINT0::INEPNE;
    pub use super::OTG_HS_DIEPINT0::ITTXFE;
    pub use super::OTG_HS_DIEPINT0::NAK;
    pub use super::OTG_HS_DIEPINT0::PKTDRPSTS;
    pub use super::OTG_HS_DIEPINT0::TOC;
    pub use super::OTG_HS_DIEPINT0::TXFE;
    pub use super::OTG_HS_DIEPINT0::TXFIFOUDRN;
    pub use super::OTG_HS_DIEPINT0::XFRC;
}

/// OTG device endpoint-5 interrupt register
pub mod OTG_HS_DIEPINT5 {
    pub use super::OTG_HS_DIEPINT0::BERR;
    pub use super::OTG_HS_DIEPINT0::BNA;
    pub use super::OTG_HS_DIEPINT0::EPDISD;
    pub use super::OTG_HS_DIEPINT0::INEPNE;
    pub use super::OTG_HS_DIEPINT0::ITTXFE;
    pub use super::OTG_HS_DIEPINT0::NAK;
    pub use super::OTG_HS_DIEPINT0::PKTDRPSTS;
    pub use super::OTG_HS_DIEPINT0::TOC;
    pub use super::OTG_HS_DIEPINT0::TXFE;
    pub use super::OTG_HS_DIEPINT0::TXFIFOUDRN;
    pub use super::OTG_HS_DIEPINT0::XFRC;
}

/// OTG device endpoint-6 interrupt register
pub mod OTG_HS_DIEPINT6 {
    pub use super::OTG_HS_DIEPINT0::BERR;
    pub use super::OTG_HS_DIEPINT0::BNA;
    pub use super::OTG_HS_DIEPINT0::EPDISD;
    pub use super::OTG_HS_DIEPINT0::INEPNE;
    pub use super::OTG_HS_DIEPINT0::ITTXFE;
    pub use super::OTG_HS_DIEPINT0::NAK;
    pub use super::OTG_HS_DIEPINT0::PKTDRPSTS;
    pub use super::OTG_HS_DIEPINT0::TOC;
    pub use super::OTG_HS_DIEPINT0::TXFE;
    pub use super::OTG_HS_DIEPINT0::TXFIFOUDRN;
    pub use super::OTG_HS_DIEPINT0::XFRC;
}

/// OTG device endpoint-7 interrupt register
pub mod OTG_HS_DIEPINT7 {
    pub use super::OTG_HS_DIEPINT0::BERR;
    pub use super::OTG_HS_DIEPINT0::BNA;
    pub use super::OTG_HS_DIEPINT0::EPDISD;
    pub use super::OTG_HS_DIEPINT0::INEPNE;
    pub use super::OTG_HS_DIEPINT0::ITTXFE;
    pub use super::OTG_HS_DIEPINT0::NAK;
    pub use super::OTG_HS_DIEPINT0::PKTDRPSTS;
    pub use super::OTG_HS_DIEPINT0::TOC;
    pub use super::OTG_HS_DIEPINT0::TXFE;
    pub use super::OTG_HS_DIEPINT0::TXFIFOUDRN;
    pub use super::OTG_HS_DIEPINT0::XFRC;
}

/// OTG_HS device IN endpoint 0 transfer size register
pub mod OTG_HS_DIEPTSIZ0 {

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
}

/// OTG_HS device endpoint-1 DMA address register
pub mod OTG_HS_DIEPDMA0 {

    /// DMA address
    pub mod DMAADDR {
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

/// OTG_HS device endpoint-2 DMA address register
pub mod OTG_HS_DIEPDMA1 {
    pub use super::OTG_HS_DIEPDMA0::DMAADDR;
}

/// OTG_HS device endpoint-3 DMA address register
pub mod OTG_HS_DIEPDMA2 {
    pub use super::OTG_HS_DIEPDMA0::DMAADDR;
}

/// OTG_HS device endpoint-4 DMA address register
pub mod OTG_HS_DIEPDMA3 {
    pub use super::OTG_HS_DIEPDMA0::DMAADDR;
}

/// OTG_HS device endpoint-5 DMA address register
pub mod OTG_HS_DIEPDMA4 {
    pub use super::OTG_HS_DIEPDMA0::DMAADDR;
}

/// OTG_HS device IN endpoint transmit FIFO status register
pub mod OTG_HS_DTXFSTS0 {

    /// IN endpoint TxFIFO space avail
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

/// OTG_HS device IN endpoint transmit FIFO status register
pub mod OTG_HS_DTXFSTS1 {
    pub use super::OTG_HS_DTXFSTS0::INEPTFSAV;
}

/// OTG_HS device IN endpoint transmit FIFO status register
pub mod OTG_HS_DTXFSTS2 {
    pub use super::OTG_HS_DTXFSTS0::INEPTFSAV;
}

/// OTG_HS device IN endpoint transmit FIFO status register
pub mod OTG_HS_DTXFSTS3 {
    pub use super::OTG_HS_DTXFSTS0::INEPTFSAV;
}

/// OTG_HS device IN endpoint transmit FIFO status register
pub mod OTG_HS_DTXFSTS4 {
    pub use super::OTG_HS_DTXFSTS0::INEPTFSAV;
}

/// OTG_HS device IN endpoint transmit FIFO status register
pub mod OTG_HS_DTXFSTS5 {
    pub use super::OTG_HS_DTXFSTS0::INEPTFSAV;
}

/// OTG_HS device endpoint transfer size register
pub mod OTG_HS_DIEPTSIZ1 {

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
}

/// OTG_HS device endpoint transfer size register
pub mod OTG_HS_DIEPTSIZ2 {
    pub use super::OTG_HS_DIEPTSIZ1::MCNT;
    pub use super::OTG_HS_DIEPTSIZ1::PKTCNT;
    pub use super::OTG_HS_DIEPTSIZ1::XFRSIZ;
}

/// OTG_HS device endpoint transfer size register
pub mod OTG_HS_DIEPTSIZ3 {
    pub use super::OTG_HS_DIEPTSIZ1::MCNT;
    pub use super::OTG_HS_DIEPTSIZ1::PKTCNT;
    pub use super::OTG_HS_DIEPTSIZ1::XFRSIZ;
}

/// OTG_HS device endpoint transfer size register
pub mod OTG_HS_DIEPTSIZ4 {
    pub use super::OTG_HS_DIEPTSIZ1::MCNT;
    pub use super::OTG_HS_DIEPTSIZ1::PKTCNT;
    pub use super::OTG_HS_DIEPTSIZ1::XFRSIZ;
}

/// OTG_HS device endpoint transfer size register
pub mod OTG_HS_DIEPTSIZ5 {
    pub use super::OTG_HS_DIEPTSIZ1::MCNT;
    pub use super::OTG_HS_DIEPTSIZ1::PKTCNT;
    pub use super::OTG_HS_DIEPTSIZ1::XFRSIZ;
}

/// OTG_HS device control OUT endpoint 0 control register
pub mod OTG_HS_DOEPCTL0 {

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

    /// Snoop mode
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

    /// STALL handshake
    pub mod Stall {
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

/// OTG device endpoint-1 control register
pub mod OTG_HS_DOEPCTL1 {

    /// Maximum packet size
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

    /// Even odd frame/Endpoint data PID
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

    /// Snoop mode
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

    /// STALL handshake
    pub mod Stall {
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

    /// Set DATA0 PID/Set even frame
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

    /// Set odd frame
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

/// OTG device endpoint-2 control register
pub mod OTG_HS_DOEPCTL2 {
    pub use super::OTG_HS_DOEPCTL1::Stall;
    pub use super::OTG_HS_DOEPCTL1::CNAK;
    pub use super::OTG_HS_DOEPCTL1::EONUM_DPID;
    pub use super::OTG_HS_DOEPCTL1::EPDIS;
    pub use super::OTG_HS_DOEPCTL1::EPENA;
    pub use super::OTG_HS_DOEPCTL1::EPTYP;
    pub use super::OTG_HS_DOEPCTL1::MPSIZ;
    pub use super::OTG_HS_DOEPCTL1::NAKSTS;
    pub use super::OTG_HS_DOEPCTL1::SD0PID_SEVNFRM;
    pub use super::OTG_HS_DOEPCTL1::SNAK;
    pub use super::OTG_HS_DOEPCTL1::SNPM;
    pub use super::OTG_HS_DOEPCTL1::SODDFRM;
    pub use super::OTG_HS_DOEPCTL1::USBAEP;
}

/// OTG device endpoint-3 control register
pub mod OTG_HS_DOEPCTL3 {
    pub use super::OTG_HS_DOEPCTL1::Stall;
    pub use super::OTG_HS_DOEPCTL1::CNAK;
    pub use super::OTG_HS_DOEPCTL1::EONUM_DPID;
    pub use super::OTG_HS_DOEPCTL1::EPDIS;
    pub use super::OTG_HS_DOEPCTL1::EPENA;
    pub use super::OTG_HS_DOEPCTL1::EPTYP;
    pub use super::OTG_HS_DOEPCTL1::MPSIZ;
    pub use super::OTG_HS_DOEPCTL1::NAKSTS;
    pub use super::OTG_HS_DOEPCTL1::SD0PID_SEVNFRM;
    pub use super::OTG_HS_DOEPCTL1::SNAK;
    pub use super::OTG_HS_DOEPCTL1::SNPM;
    pub use super::OTG_HS_DOEPCTL1::SODDFRM;
    pub use super::OTG_HS_DOEPCTL1::USBAEP;
}

/// OTG_HS device endpoint-0 interrupt register
pub mod OTG_HS_DOEPINT0 {

    /// Transfer completed interrupt
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

    /// Endpoint disabled interrupt
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

    /// SETUP phase done
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

    /// OUT token received when endpoint disabled
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

    /// Back-to-back SETUP packets received
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

    /// NYET interrupt
    pub mod NYET {
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
}

/// OTG_HS device endpoint-1 interrupt register
pub mod OTG_HS_DOEPINT1 {
    pub use super::OTG_HS_DOEPINT0::B2BSTUP;
    pub use super::OTG_HS_DOEPINT0::EPDISD;
    pub use super::OTG_HS_DOEPINT0::NYET;
    pub use super::OTG_HS_DOEPINT0::OTEPDIS;
    pub use super::OTG_HS_DOEPINT0::STUP;
    pub use super::OTG_HS_DOEPINT0::XFRC;
}

/// OTG_HS device endpoint-2 interrupt register
pub mod OTG_HS_DOEPINT2 {
    pub use super::OTG_HS_DOEPINT0::B2BSTUP;
    pub use super::OTG_HS_DOEPINT0::EPDISD;
    pub use super::OTG_HS_DOEPINT0::NYET;
    pub use super::OTG_HS_DOEPINT0::OTEPDIS;
    pub use super::OTG_HS_DOEPINT0::STUP;
    pub use super::OTG_HS_DOEPINT0::XFRC;
}

/// OTG_HS device endpoint-3 interrupt register
pub mod OTG_HS_DOEPINT3 {
    pub use super::OTG_HS_DOEPINT0::B2BSTUP;
    pub use super::OTG_HS_DOEPINT0::EPDISD;
    pub use super::OTG_HS_DOEPINT0::NYET;
    pub use super::OTG_HS_DOEPINT0::OTEPDIS;
    pub use super::OTG_HS_DOEPINT0::STUP;
    pub use super::OTG_HS_DOEPINT0::XFRC;
}

/// OTG_HS device endpoint-4 interrupt register
pub mod OTG_HS_DOEPINT4 {
    pub use super::OTG_HS_DOEPINT0::B2BSTUP;
    pub use super::OTG_HS_DOEPINT0::EPDISD;
    pub use super::OTG_HS_DOEPINT0::NYET;
    pub use super::OTG_HS_DOEPINT0::OTEPDIS;
    pub use super::OTG_HS_DOEPINT0::STUP;
    pub use super::OTG_HS_DOEPINT0::XFRC;
}

/// OTG_HS device endpoint-5 interrupt register
pub mod OTG_HS_DOEPINT5 {
    pub use super::OTG_HS_DOEPINT0::B2BSTUP;
    pub use super::OTG_HS_DOEPINT0::EPDISD;
    pub use super::OTG_HS_DOEPINT0::NYET;
    pub use super::OTG_HS_DOEPINT0::OTEPDIS;
    pub use super::OTG_HS_DOEPINT0::STUP;
    pub use super::OTG_HS_DOEPINT0::XFRC;
}

/// OTG_HS device endpoint-6 interrupt register
pub mod OTG_HS_DOEPINT6 {
    pub use super::OTG_HS_DOEPINT0::B2BSTUP;
    pub use super::OTG_HS_DOEPINT0::EPDISD;
    pub use super::OTG_HS_DOEPINT0::NYET;
    pub use super::OTG_HS_DOEPINT0::OTEPDIS;
    pub use super::OTG_HS_DOEPINT0::STUP;
    pub use super::OTG_HS_DOEPINT0::XFRC;
}

/// OTG_HS device endpoint-7 interrupt register
pub mod OTG_HS_DOEPINT7 {
    pub use super::OTG_HS_DOEPINT0::B2BSTUP;
    pub use super::OTG_HS_DOEPINT0::EPDISD;
    pub use super::OTG_HS_DOEPINT0::NYET;
    pub use super::OTG_HS_DOEPINT0::OTEPDIS;
    pub use super::OTG_HS_DOEPINT0::STUP;
    pub use super::OTG_HS_DOEPINT0::XFRC;
}

/// OTG_HS device endpoint-0 transfer size register
pub mod OTG_HS_DOEPTSIZ0 {

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
}

/// OTG_HS device endpoint-1 transfer size register
pub mod OTG_HS_DOEPTSIZ1 {

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
}

/// OTG_HS device endpoint-2 transfer size register
pub mod OTG_HS_DOEPTSIZ2 {
    pub use super::OTG_HS_DOEPTSIZ1::PKTCNT;
    pub use super::OTG_HS_DOEPTSIZ1::RXDPID_STUPCNT;
    pub use super::OTG_HS_DOEPTSIZ1::XFRSIZ;
}

/// OTG_HS device endpoint-3 transfer size register
pub mod OTG_HS_DOEPTSIZ3 {
    pub use super::OTG_HS_DOEPTSIZ1::PKTCNT;
    pub use super::OTG_HS_DOEPTSIZ1::RXDPID_STUPCNT;
    pub use super::OTG_HS_DOEPTSIZ1::XFRSIZ;
}

/// OTG_HS device endpoint-4 transfer size register
pub mod OTG_HS_DOEPTSIZ4 {
    pub use super::OTG_HS_DOEPTSIZ1::PKTCNT;
    pub use super::OTG_HS_DOEPTSIZ1::RXDPID_STUPCNT;
    pub use super::OTG_HS_DOEPTSIZ1::XFRSIZ;
}

/// OTG_HS device endpoint transfer size register
pub mod OTG_HS_DIEPTSIZ6 {
    pub use super::OTG_HS_DIEPTSIZ1::MCNT;
    pub use super::OTG_HS_DIEPTSIZ1::PKTCNT;
    pub use super::OTG_HS_DIEPTSIZ1::XFRSIZ;
}

/// OTG_HS device IN endpoint transmit FIFO status register
pub mod OTG_HS_DTXFSTS6 {

    /// IN endpoint TxFIFO space avail
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

/// OTG_HS device endpoint transfer size register
pub mod OTG_HS_DIEPTSIZ7 {
    pub use super::OTG_HS_DIEPTSIZ1::MCNT;
    pub use super::OTG_HS_DIEPTSIZ1::PKTCNT;
    pub use super::OTG_HS_DIEPTSIZ1::XFRSIZ;
}

/// OTG_HS device IN endpoint transmit FIFO status register
pub mod OTG_HS_DTXFSTS7 {
    pub use super::OTG_HS_DTXFSTS6::INEPTFSAV;
}

/// OTG device endpoint-4 control register
pub mod OTG_HS_DOEPCTL4 {
    pub use super::OTG_HS_DOEPCTL1::Stall;
    pub use super::OTG_HS_DOEPCTL1::CNAK;
    pub use super::OTG_HS_DOEPCTL1::EONUM_DPID;
    pub use super::OTG_HS_DOEPCTL1::EPDIS;
    pub use super::OTG_HS_DOEPCTL1::EPENA;
    pub use super::OTG_HS_DOEPCTL1::EPTYP;
    pub use super::OTG_HS_DOEPCTL1::MPSIZ;
    pub use super::OTG_HS_DOEPCTL1::NAKSTS;
    pub use super::OTG_HS_DOEPCTL1::SD0PID_SEVNFRM;
    pub use super::OTG_HS_DOEPCTL1::SNAK;
    pub use super::OTG_HS_DOEPCTL1::SNPM;
    pub use super::OTG_HS_DOEPCTL1::SODDFRM;
    pub use super::OTG_HS_DOEPCTL1::USBAEP;
}

/// OTG device endpoint-5 control register
pub mod OTG_HS_DOEPCTL5 {
    pub use super::OTG_HS_DOEPCTL1::Stall;
    pub use super::OTG_HS_DOEPCTL1::CNAK;
    pub use super::OTG_HS_DOEPCTL1::EONUM_DPID;
    pub use super::OTG_HS_DOEPCTL1::EPDIS;
    pub use super::OTG_HS_DOEPCTL1::EPENA;
    pub use super::OTG_HS_DOEPCTL1::EPTYP;
    pub use super::OTG_HS_DOEPCTL1::MPSIZ;
    pub use super::OTG_HS_DOEPCTL1::NAKSTS;
    pub use super::OTG_HS_DOEPCTL1::SD0PID_SEVNFRM;
    pub use super::OTG_HS_DOEPCTL1::SNAK;
    pub use super::OTG_HS_DOEPCTL1::SNPM;
    pub use super::OTG_HS_DOEPCTL1::SODDFRM;
    pub use super::OTG_HS_DOEPCTL1::USBAEP;
}

/// OTG device endpoint-6 control register
pub mod OTG_HS_DOEPCTL6 {
    pub use super::OTG_HS_DOEPCTL1::Stall;
    pub use super::OTG_HS_DOEPCTL1::CNAK;
    pub use super::OTG_HS_DOEPCTL1::EONUM_DPID;
    pub use super::OTG_HS_DOEPCTL1::EPDIS;
    pub use super::OTG_HS_DOEPCTL1::EPENA;
    pub use super::OTG_HS_DOEPCTL1::EPTYP;
    pub use super::OTG_HS_DOEPCTL1::MPSIZ;
    pub use super::OTG_HS_DOEPCTL1::NAKSTS;
    pub use super::OTG_HS_DOEPCTL1::SD0PID_SEVNFRM;
    pub use super::OTG_HS_DOEPCTL1::SNAK;
    pub use super::OTG_HS_DOEPCTL1::SNPM;
    pub use super::OTG_HS_DOEPCTL1::SODDFRM;
    pub use super::OTG_HS_DOEPCTL1::USBAEP;
}

/// OTG device endpoint-7 control register
pub mod OTG_HS_DOEPCTL7 {
    pub use super::OTG_HS_DOEPCTL1::Stall;
    pub use super::OTG_HS_DOEPCTL1::CNAK;
    pub use super::OTG_HS_DOEPCTL1::EONUM_DPID;
    pub use super::OTG_HS_DOEPCTL1::EPDIS;
    pub use super::OTG_HS_DOEPCTL1::EPENA;
    pub use super::OTG_HS_DOEPCTL1::EPTYP;
    pub use super::OTG_HS_DOEPCTL1::MPSIZ;
    pub use super::OTG_HS_DOEPCTL1::NAKSTS;
    pub use super::OTG_HS_DOEPCTL1::SD0PID_SEVNFRM;
    pub use super::OTG_HS_DOEPCTL1::SNAK;
    pub use super::OTG_HS_DOEPCTL1::SNPM;
    pub use super::OTG_HS_DOEPCTL1::SODDFRM;
    pub use super::OTG_HS_DOEPCTL1::USBAEP;
}

/// OTG_HS device endpoint-5 transfer size register
pub mod OTG_HS_DOEPTSIZ5 {
    pub use super::OTG_HS_DOEPTSIZ1::PKTCNT;
    pub use super::OTG_HS_DOEPTSIZ1::RXDPID_STUPCNT;
    pub use super::OTG_HS_DOEPTSIZ1::XFRSIZ;
}

/// OTG_HS device endpoint-6 transfer size register
pub mod OTG_HS_DOEPTSIZ6 {
    pub use super::OTG_HS_DOEPTSIZ1::PKTCNT;
    pub use super::OTG_HS_DOEPTSIZ1::RXDPID_STUPCNT;
    pub use super::OTG_HS_DOEPTSIZ1::XFRSIZ;
}

/// OTG_HS device endpoint-7 transfer size register
pub mod OTG_HS_DOEPTSIZ7 {
    pub use super::OTG_HS_DOEPTSIZ1::PKTCNT;
    pub use super::OTG_HS_DOEPTSIZ1::RXDPID_STUPCNT;
    pub use super::OTG_HS_DOEPTSIZ1::XFRSIZ;
}

/// OTG Device channel-x DMA address register
pub mod OTG_HS_DOEPDMA0 {}

/// OTG Device channel-x DMA address register
pub mod OTG_HS_DOEPDMA1 {}

/// OTG Device channel-x DMA address register
pub mod OTG_HS_DOEPDMA2 {}

/// OTG Device channel-x DMA address register
pub mod OTG_HS_DOEPDMA3 {}

/// OTG Device channel-x DMA address register
pub mod OTG_HS_DOEPDMA4 {}

/// OTG Device channel-x DMA address register
pub mod OTG_HS_DOEPDMA5 {}

/// OTG Device channel-x DMA address register
pub mod OTG_HS_DOEPDMA6 {}

/// OTG Device channel-x DMA address register
pub mod OTG_HS_DOEPDMA7 {}

/// OTG Device channel-x DMA address register
pub mod OTG_HS_DOEPDMA8 {}

/// OTG Device channel-x DMA address register
pub mod OTG_HS_DOEPDMA9 {}

/// OTG Device channel-x DMA address register
pub mod OTG_HS_DOEPDMA10 {}

/// OTG Device channel-x DMA address register
pub mod OTG_HS_DOEPDMA11 {}

/// OTG Device channel-x DMA address register
pub mod OTG_HS_DOEPDMA12 {}

/// OTG Device channel-x DMA address register
pub mod OTG_HS_DOEPDMA13 {}

/// OTG Device channel-x DMA address register
pub mod OTG_HS_DOEPDMA14 {}

/// OTG Device channel-x DMA address register
pub mod OTG_HS_DOEPDMA15 {}

/// OTG Device channel-x DMA address register
pub mod OTG_HS_DIEPDMA5 {}

/// OTG Device channel-x DMA address register
pub mod OTG_HS_DIEPDMA6 {}

/// OTG Device channel-x DMA address register
pub mod OTG_HS_DIEPDMA7 {}

/// OTG Device channel-x DMA address register
pub mod OTG_HS_DIEPDMA8 {}

/// OTG Device channel-x DMA address register
pub mod OTG_HS_DIEPDMA9 {}

/// OTG Device channel-x DMA address register
pub mod OTG_HS_DIEPDMA10 {}

/// OTG Device channel-x DMA address register
pub mod OTG_HS_DIEPDMA11 {}

/// OTG Device channel-x DMA address register
pub mod OTG_HS_DIEPDMA12 {}

/// OTG Device channel-x DMA address register
pub mod OTG_HS_DIEPDMA13 {}

/// OTG Device channel-x DMA address register
pub mod OTG_HS_DIEPDMA14 {}

/// OTG Device channel-x DMA address register
pub mod OTG_HS_DIEPDMA15 {}
pub struct RegisterBlock {
    /// OTG_HS device configuration register
    pub OTG_HS_DCFG: RWRegister<u32>,

    /// OTG_HS device control register
    pub OTG_HS_DCTL: RWRegister<u32>,

    /// OTG_HS device status register
    pub OTG_HS_DSTS: RORegister<u32>,

    _reserved1: [u32; 1],

    /// OTG_HS device IN endpoint common interrupt mask register
    pub OTG_HS_DIEPMSK: RWRegister<u32>,

    /// OTG_HS device OUT endpoint common interrupt mask register
    pub OTG_HS_DOEPMSK: RWRegister<u32>,

    /// OTG_HS device all endpoints interrupt register
    pub OTG_HS_DAINT: RORegister<u32>,

    /// OTG_HS all endpoints interrupt mask register
    pub OTG_HS_DAINTMSK: RWRegister<u32>,

    _reserved2: [u32; 2],

    /// OTG_HS device VBUS discharge time register
    pub OTG_HS_DVBUSDIS: RWRegister<u32>,

    /// OTG_HS device VBUS pulsing time register
    pub OTG_HS_DVBUSPULSE: RWRegister<u32>,

    /// OTG_HS Device threshold control register
    pub OTG_HS_DTHRCTL: RWRegister<u32>,

    /// OTG_HS device IN endpoint FIFO empty interrupt mask register
    pub OTG_HS_DIEPEMPMSK: RWRegister<u32>,

    /// OTG_HS device each endpoint interrupt register
    pub OTG_HS_DEACHINT: RWRegister<u32>,

    /// OTG_HS device each endpoint interrupt register mask
    pub OTG_HS_DEACHINTMSK: RWRegister<u32>,

    _reserved3: [u32; 48],

    /// OTG device endpoint-0 control register
    pub OTG_HS_DIEPCTL0: RWRegister<u32>,

    _reserved4: [u32; 1],

    /// OTG device endpoint-0 interrupt register
    pub OTG_HS_DIEPINT0: RWRegister<u32>,

    _reserved5: [u32; 1],

    /// OTG_HS device IN endpoint 0 transfer size register
    pub OTG_HS_DIEPTSIZ0: RWRegister<u32>,

    /// OTG_HS device endpoint-1 DMA address register
    pub OTG_HS_DIEPDMA0: RWRegister<u32>,

    /// OTG_HS device IN endpoint transmit FIFO status register
    pub OTG_HS_DTXFSTS0: RORegister<u32>,

    _reserved6: [u32; 1],

    /// OTG device endpoint-1 control register
    pub OTG_HS_DIEPCTL1: RWRegister<u32>,

    _reserved7: [u32; 1],

    /// OTG device endpoint-1 interrupt register
    pub OTG_HS_DIEPINT1: RWRegister<u32>,

    _reserved8: [u32; 1],

    /// OTG_HS device endpoint transfer size register
    pub OTG_HS_DIEPTSIZ1: RWRegister<u32>,

    /// OTG_HS device endpoint-2 DMA address register
    pub OTG_HS_DIEPDMA1: RWRegister<u32>,

    /// OTG_HS device IN endpoint transmit FIFO status register
    pub OTG_HS_DTXFSTS1: RORegister<u32>,

    _reserved9: [u32; 1],

    /// OTG device endpoint-2 control register
    pub OTG_HS_DIEPCTL2: RWRegister<u32>,

    _reserved10: [u32; 1],

    /// OTG device endpoint-2 interrupt register
    pub OTG_HS_DIEPINT2: RWRegister<u32>,

    _reserved11: [u32; 1],

    /// OTG_HS device endpoint transfer size register
    pub OTG_HS_DIEPTSIZ2: RWRegister<u32>,

    /// OTG_HS device endpoint-3 DMA address register
    pub OTG_HS_DIEPDMA2: RWRegister<u32>,

    /// OTG_HS device IN endpoint transmit FIFO status register
    pub OTG_HS_DTXFSTS2: RORegister<u32>,

    _reserved12: [u32; 1],

    /// OTG device endpoint-3 control register
    pub OTG_HS_DIEPCTL3: RWRegister<u32>,

    _reserved13: [u32; 1],

    /// OTG device endpoint-3 interrupt register
    pub OTG_HS_DIEPINT3: RWRegister<u32>,

    _reserved14: [u32; 1],

    /// OTG_HS device endpoint transfer size register
    pub OTG_HS_DIEPTSIZ3: RWRegister<u32>,

    /// OTG_HS device endpoint-4 DMA address register
    pub OTG_HS_DIEPDMA3: RWRegister<u32>,

    /// OTG_HS device IN endpoint transmit FIFO status register
    pub OTG_HS_DTXFSTS3: RORegister<u32>,

    _reserved15: [u32; 1],

    /// OTG device endpoint-4 control register
    pub OTG_HS_DIEPCTL4: RWRegister<u32>,

    _reserved16: [u32; 1],

    /// OTG device endpoint-4 interrupt register
    pub OTG_HS_DIEPINT4: RWRegister<u32>,

    _reserved17: [u32; 1],

    /// OTG_HS device endpoint transfer size register
    pub OTG_HS_DIEPTSIZ4: RWRegister<u32>,

    /// OTG_HS device endpoint-5 DMA address register
    pub OTG_HS_DIEPDMA4: RWRegister<u32>,

    /// OTG_HS device IN endpoint transmit FIFO status register
    pub OTG_HS_DTXFSTS4: RORegister<u32>,

    _reserved18: [u32; 1],

    /// OTG device endpoint-5 control register
    pub OTG_HS_DIEPCTL5: RWRegister<u32>,

    _reserved19: [u32; 1],

    /// OTG device endpoint-5 interrupt register
    pub OTG_HS_DIEPINT5: RWRegister<u32>,

    _reserved20: [u32; 1],

    /// OTG_HS device endpoint transfer size register
    pub OTG_HS_DIEPTSIZ5: RWRegister<u32>,

    /// OTG Device channel-x DMA address register
    pub OTG_HS_DIEPDMA5: RWRegister<u32>,

    /// OTG_HS device IN endpoint transmit FIFO status register
    pub OTG_HS_DTXFSTS5: RORegister<u32>,

    _reserved21: [u32; 1],

    /// OTG device endpoint-6 control register
    pub OTG_HS_DIEPCTL6: RWRegister<u32>,

    _reserved22: [u32; 1],

    /// OTG device endpoint-6 interrupt register
    pub OTG_HS_DIEPINT6: RWRegister<u32>,

    _reserved23: [u32; 1],

    /// OTG_HS device endpoint transfer size register
    pub OTG_HS_DIEPTSIZ6: RWRegister<u32>,

    /// OTG Device channel-x DMA address register
    pub OTG_HS_DIEPDMA6: RWRegister<u32>,

    /// OTG_HS device IN endpoint transmit FIFO status register
    pub OTG_HS_DTXFSTS6: RWRegister<u32>,

    _reserved24: [u32; 1],

    /// OTG device endpoint-7 control register
    pub OTG_HS_DIEPCTL7: RWRegister<u32>,

    _reserved25: [u32; 1],

    /// OTG device endpoint-7 interrupt register
    pub OTG_HS_DIEPINT7: RWRegister<u32>,

    _reserved26: [u32; 1],

    /// OTG_HS device endpoint transfer size register
    pub OTG_HS_DIEPTSIZ7: RWRegister<u32>,

    /// OTG Device channel-x DMA address register
    pub OTG_HS_DIEPDMA7: RWRegister<u32>,

    /// OTG_HS device IN endpoint transmit FIFO status register
    pub OTG_HS_DTXFSTS7: RWRegister<u32>,

    _reserved27: [u32; 6],

    /// OTG Device channel-x DMA address register
    pub OTG_HS_DIEPDMA8: RWRegister<u32>,

    _reserved28: [u32; 7],

    /// OTG Device channel-x DMA address register
    pub OTG_HS_DIEPDMA9: RWRegister<u32>,

    _reserved29: [u32; 7],

    /// OTG Device channel-x DMA address register
    pub OTG_HS_DIEPDMA10: RWRegister<u32>,

    _reserved30: [u32; 7],

    /// OTG Device channel-x DMA address register
    pub OTG_HS_DIEPDMA11: RWRegister<u32>,

    _reserved31: [u32; 7],

    /// OTG Device channel-x DMA address register
    pub OTG_HS_DIEPDMA12: RWRegister<u32>,

    _reserved32: [u32; 7],

    /// OTG Device channel-x DMA address register
    pub OTG_HS_DIEPDMA13: RWRegister<u32>,

    _reserved33: [u32; 7],

    /// OTG Device channel-x DMA address register
    pub OTG_HS_DIEPDMA14: RWRegister<u32>,

    _reserved34: [u32; 7],

    /// OTG Device channel-x DMA address register
    pub OTG_HS_DIEPDMA15: RWRegister<u32>,

    _reserved35: [u32; 2],

    /// OTG_HS device control OUT endpoint 0 control register
    pub OTG_HS_DOEPCTL0: RWRegister<u32>,

    _reserved36: [u32; 1],

    /// OTG_HS device endpoint-0 interrupt register
    pub OTG_HS_DOEPINT0: RWRegister<u32>,

    _reserved37: [u32; 1],

    /// OTG_HS device endpoint-0 transfer size register
    pub OTG_HS_DOEPTSIZ0: RWRegister<u32>,

    /// OTG Device channel-x DMA address register
    pub OTG_HS_DOEPDMA0: RWRegister<u32>,

    _reserved38: [u32; 2],

    /// OTG device endpoint-1 control register
    pub OTG_HS_DOEPCTL1: RWRegister<u32>,

    _reserved39: [u32; 1],

    /// OTG_HS device endpoint-1 interrupt register
    pub OTG_HS_DOEPINT1: RWRegister<u32>,

    _reserved40: [u32; 1],

    /// OTG_HS device endpoint-1 transfer size register
    pub OTG_HS_DOEPTSIZ1: RWRegister<u32>,

    /// OTG Device channel-x DMA address register
    pub OTG_HS_DOEPDMA1: RWRegister<u32>,

    _reserved41: [u32; 2],

    /// OTG device endpoint-2 control register
    pub OTG_HS_DOEPCTL2: RWRegister<u32>,

    _reserved42: [u32; 1],

    /// OTG_HS device endpoint-2 interrupt register
    pub OTG_HS_DOEPINT2: RWRegister<u32>,

    _reserved43: [u32; 1],

    /// OTG_HS device endpoint-2 transfer size register
    pub OTG_HS_DOEPTSIZ2: RWRegister<u32>,

    /// OTG Device channel-x DMA address register
    pub OTG_HS_DOEPDMA2: RWRegister<u32>,

    _reserved44: [u32; 2],

    /// OTG device endpoint-3 control register
    pub OTG_HS_DOEPCTL3: RWRegister<u32>,

    _reserved45: [u32; 1],

    /// OTG_HS device endpoint-3 interrupt register
    pub OTG_HS_DOEPINT3: RWRegister<u32>,

    _reserved46: [u32; 1],

    /// OTG_HS device endpoint-3 transfer size register
    pub OTG_HS_DOEPTSIZ3: RWRegister<u32>,

    /// OTG Device channel-x DMA address register
    pub OTG_HS_DOEPDMA3: RWRegister<u32>,

    _reserved47: [u32; 2],

    /// OTG device endpoint-4 control register
    pub OTG_HS_DOEPCTL4: RWRegister<u32>,

    _reserved48: [u32; 1],

    /// OTG_HS device endpoint-4 interrupt register
    pub OTG_HS_DOEPINT4: RWRegister<u32>,

    _reserved49: [u32; 1],

    /// OTG_HS device endpoint-4 transfer size register
    pub OTG_HS_DOEPTSIZ4: RWRegister<u32>,

    /// OTG Device channel-x DMA address register
    pub OTG_HS_DOEPDMA4: RWRegister<u32>,

    _reserved50: [u32; 2],

    /// OTG device endpoint-5 control register
    pub OTG_HS_DOEPCTL5: RWRegister<u32>,

    _reserved51: [u32; 1],

    /// OTG_HS device endpoint-5 interrupt register
    pub OTG_HS_DOEPINT5: RWRegister<u32>,

    _reserved52: [u32; 1],

    /// OTG_HS device endpoint-5 transfer size register
    pub OTG_HS_DOEPTSIZ5: RWRegister<u32>,

    /// OTG Device channel-x DMA address register
    pub OTG_HS_DOEPDMA5: RWRegister<u32>,

    _reserved53: [u32; 2],

    /// OTG device endpoint-6 control register
    pub OTG_HS_DOEPCTL6: RWRegister<u32>,

    _reserved54: [u32; 1],

    /// OTG_HS device endpoint-6 interrupt register
    pub OTG_HS_DOEPINT6: RWRegister<u32>,

    _reserved55: [u32; 1],

    /// OTG_HS device endpoint-6 transfer size register
    pub OTG_HS_DOEPTSIZ6: RWRegister<u32>,

    /// OTG Device channel-x DMA address register
    pub OTG_HS_DOEPDMA6: RWRegister<u32>,

    _reserved56: [u32; 2],

    /// OTG device endpoint-7 control register
    pub OTG_HS_DOEPCTL7: RWRegister<u32>,

    _reserved57: [u32; 1],

    /// OTG_HS device endpoint-7 interrupt register
    pub OTG_HS_DOEPINT7: RWRegister<u32>,

    _reserved58: [u32; 1],

    /// OTG_HS device endpoint-7 transfer size register
    pub OTG_HS_DOEPTSIZ7: RWRegister<u32>,

    /// OTG Device channel-x DMA address register
    pub OTG_HS_DOEPDMA7: RWRegister<u32>,

    _reserved59: [u32; 7],

    /// OTG Device channel-x DMA address register
    pub OTG_HS_DOEPDMA8: RWRegister<u32>,

    _reserved60: [u32; 7],

    /// OTG Device channel-x DMA address register
    pub OTG_HS_DOEPDMA9: RWRegister<u32>,

    _reserved61: [u32; 7],

    /// OTG Device channel-x DMA address register
    pub OTG_HS_DOEPDMA10: RWRegister<u32>,

    _reserved62: [u32; 7],

    /// OTG Device channel-x DMA address register
    pub OTG_HS_DOEPDMA11: RWRegister<u32>,

    _reserved63: [u32; 7],

    /// OTG Device channel-x DMA address register
    pub OTG_HS_DOEPDMA12: RWRegister<u32>,

    _reserved64: [u32; 7],

    /// OTG Device channel-x DMA address register
    pub OTG_HS_DOEPDMA13: RWRegister<u32>,

    _reserved65: [u32; 7],

    /// OTG Device channel-x DMA address register
    pub OTG_HS_DOEPDMA14: RWRegister<u32>,

    _reserved66: [u32; 7],

    /// OTG Device channel-x DMA address register
    pub OTG_HS_DOEPDMA15: RWRegister<u32>,
}
pub struct ResetValues {
    pub OTG_HS_DCFG: u32,
    pub OTG_HS_DCTL: u32,
    pub OTG_HS_DSTS: u32,
    pub OTG_HS_DIEPMSK: u32,
    pub OTG_HS_DOEPMSK: u32,
    pub OTG_HS_DAINT: u32,
    pub OTG_HS_DAINTMSK: u32,
    pub OTG_HS_DVBUSDIS: u32,
    pub OTG_HS_DVBUSPULSE: u32,
    pub OTG_HS_DTHRCTL: u32,
    pub OTG_HS_DIEPEMPMSK: u32,
    pub OTG_HS_DEACHINT: u32,
    pub OTG_HS_DEACHINTMSK: u32,
    pub OTG_HS_DIEPCTL0: u32,
    pub OTG_HS_DIEPINT0: u32,
    pub OTG_HS_DIEPTSIZ0: u32,
    pub OTG_HS_DIEPDMA0: u32,
    pub OTG_HS_DTXFSTS0: u32,
    pub OTG_HS_DIEPCTL1: u32,
    pub OTG_HS_DIEPINT1: u32,
    pub OTG_HS_DIEPTSIZ1: u32,
    pub OTG_HS_DIEPDMA1: u32,
    pub OTG_HS_DTXFSTS1: u32,
    pub OTG_HS_DIEPCTL2: u32,
    pub OTG_HS_DIEPINT2: u32,
    pub OTG_HS_DIEPTSIZ2: u32,
    pub OTG_HS_DIEPDMA2: u32,
    pub OTG_HS_DTXFSTS2: u32,
    pub OTG_HS_DIEPCTL3: u32,
    pub OTG_HS_DIEPINT3: u32,
    pub OTG_HS_DIEPTSIZ3: u32,
    pub OTG_HS_DIEPDMA3: u32,
    pub OTG_HS_DTXFSTS3: u32,
    pub OTG_HS_DIEPCTL4: u32,
    pub OTG_HS_DIEPINT4: u32,
    pub OTG_HS_DIEPTSIZ4: u32,
    pub OTG_HS_DIEPDMA4: u32,
    pub OTG_HS_DTXFSTS4: u32,
    pub OTG_HS_DIEPCTL5: u32,
    pub OTG_HS_DIEPINT5: u32,
    pub OTG_HS_DIEPTSIZ5: u32,
    pub OTG_HS_DIEPDMA5: u32,
    pub OTG_HS_DTXFSTS5: u32,
    pub OTG_HS_DIEPCTL6: u32,
    pub OTG_HS_DIEPINT6: u32,
    pub OTG_HS_DIEPTSIZ6: u32,
    pub OTG_HS_DIEPDMA6: u32,
    pub OTG_HS_DTXFSTS6: u32,
    pub OTG_HS_DIEPCTL7: u32,
    pub OTG_HS_DIEPINT7: u32,
    pub OTG_HS_DIEPTSIZ7: u32,
    pub OTG_HS_DIEPDMA7: u32,
    pub OTG_HS_DTXFSTS7: u32,
    pub OTG_HS_DIEPDMA8: u32,
    pub OTG_HS_DIEPDMA9: u32,
    pub OTG_HS_DIEPDMA10: u32,
    pub OTG_HS_DIEPDMA11: u32,
    pub OTG_HS_DIEPDMA12: u32,
    pub OTG_HS_DIEPDMA13: u32,
    pub OTG_HS_DIEPDMA14: u32,
    pub OTG_HS_DIEPDMA15: u32,
    pub OTG_HS_DOEPCTL0: u32,
    pub OTG_HS_DOEPINT0: u32,
    pub OTG_HS_DOEPTSIZ0: u32,
    pub OTG_HS_DOEPDMA0: u32,
    pub OTG_HS_DOEPCTL1: u32,
    pub OTG_HS_DOEPINT1: u32,
    pub OTG_HS_DOEPTSIZ1: u32,
    pub OTG_HS_DOEPDMA1: u32,
    pub OTG_HS_DOEPCTL2: u32,
    pub OTG_HS_DOEPINT2: u32,
    pub OTG_HS_DOEPTSIZ2: u32,
    pub OTG_HS_DOEPDMA2: u32,
    pub OTG_HS_DOEPCTL3: u32,
    pub OTG_HS_DOEPINT3: u32,
    pub OTG_HS_DOEPTSIZ3: u32,
    pub OTG_HS_DOEPDMA3: u32,
    pub OTG_HS_DOEPCTL4: u32,
    pub OTG_HS_DOEPINT4: u32,
    pub OTG_HS_DOEPTSIZ4: u32,
    pub OTG_HS_DOEPDMA4: u32,
    pub OTG_HS_DOEPCTL5: u32,
    pub OTG_HS_DOEPINT5: u32,
    pub OTG_HS_DOEPTSIZ5: u32,
    pub OTG_HS_DOEPDMA5: u32,
    pub OTG_HS_DOEPCTL6: u32,
    pub OTG_HS_DOEPINT6: u32,
    pub OTG_HS_DOEPTSIZ6: u32,
    pub OTG_HS_DOEPDMA6: u32,
    pub OTG_HS_DOEPCTL7: u32,
    pub OTG_HS_DOEPINT7: u32,
    pub OTG_HS_DOEPTSIZ7: u32,
    pub OTG_HS_DOEPDMA7: u32,
    pub OTG_HS_DOEPDMA8: u32,
    pub OTG_HS_DOEPDMA9: u32,
    pub OTG_HS_DOEPDMA10: u32,
    pub OTG_HS_DOEPDMA11: u32,
    pub OTG_HS_DOEPDMA12: u32,
    pub OTG_HS_DOEPDMA13: u32,
    pub OTG_HS_DOEPDMA14: u32,
    pub OTG_HS_DOEPDMA15: u32,
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
#[cfg(feature = "rtfm")]
unsafe impl Send for Instance {}
