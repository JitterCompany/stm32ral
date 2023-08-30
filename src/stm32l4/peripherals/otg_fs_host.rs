#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! USB on the go full speed
//!
//! Used by: stm32l4r9, stm32l4x6

use crate::{RORegister, RWRegister};
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// OTG_FS host configuration register (OTG_FS_HCFG)
pub mod HCFG {

    /// FS/LS PHY clock select
    pub mod FSLSPCS {
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

    /// FS- and LS-only support
    pub mod FSLSS {
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

/// OTG_FS Host frame interval register
pub mod HFIR {

    /// Frame interval
    pub mod FRIVL {
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

/// OTG_FS host frame number/frame time remaining register (OTG_FS_HFNUM)
pub mod HFNUM {

    /// Frame number
    pub mod FRNUM {
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

    /// Frame time remaining
    pub mod FTREM {
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

/// OTG_FS_Host periodic transmit FIFO/queue status register (OTG_FS_HPTXSTS)
pub mod HPTXSTS {

    /// Periodic transmit data FIFO space available
    pub mod PTXFSAVL {
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

    /// Periodic transmit request queue space available
    pub mod PTXQSAV {
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

    /// Top of the periodic transmit request queue
    pub mod PTXQTOP {
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

/// OTG_FS Host all channels interrupt register
pub mod HAINT {

    /// Channel interrupts
    pub mod HAINT {
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

/// OTG_FS host all channels interrupt mask register
pub mod HAINTMSK {

    /// Channel interrupt mask
    pub mod HAINTM {
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

/// OTG_FS host port control and status register (OTG_FS_HPRT)
pub mod HPRT {

    /// Port connect status
    pub mod PCSTS {
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

    /// Port connect detected
    pub mod PCDET {
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

    /// Port enable
    pub mod PENA {
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

    /// Port enable/disable change
    pub mod PENCHNG {
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

    /// Port overcurrent active
    pub mod POCA {
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

    /// Port overcurrent change
    pub mod POCCHNG {
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

    /// Port resume
    pub mod PRES {
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

    /// Port suspend
    pub mod PSUSP {
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

    /// Port reset
    pub mod PRST {
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

    /// Port line status
    pub mod PLSTS {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (2 bits: 0b11 << 10)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Port power
    pub mod PPWR {
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

    /// Port test control
    pub mod PTCTL {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (4 bits: 0b1111 << 13)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Port speed
    pub mod PSPD {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (2 bits: 0b11 << 17)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// OTG_FS host channel-0 characteristics register (OTG_FS_HCCHAR0)
pub mod CHAR0 {

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

    /// Endpoint number
    pub mod EPNUM {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (4 bits: 0b1111 << 11)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Endpoint direction
    pub mod EPDIR {
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

    /// Low-speed device
    pub mod LSDEV {
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

    /// Multicount
    pub mod MCNT {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (2 bits: 0b11 << 20)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Device address
    pub mod DAD {
        /// Offset (22 bits)
        pub const offset: u32 = 22;
        /// Mask (7 bits: 0x7f << 22)
        pub const mask: u32 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Odd frame
    pub mod ODDFRM {
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

    /// Channel disable
    pub mod CHDIS {
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

    /// Channel enable
    pub mod CHENA {
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

/// OTG_FS host channel-0 interrupt register (OTG_FS_HCINT0)
pub mod INT0 {

    /// Transfer completed
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

    /// Channel halted
    pub mod CHH {
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

    /// STALL response received interrupt
    pub mod STALL {
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

    /// NAK response received interrupt
    pub mod NAK {
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

    /// ACK response received/transmitted interrupt
    pub mod ACK {
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

    /// Transaction error
    pub mod TXERR {
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

    /// Babble error
    pub mod BBERR {
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

    /// Frame overrun
    pub mod FRMOR {
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

    /// Data toggle error
    pub mod DTERR {
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
}

/// OTG_FS host channel-0 mask register (OTG_FS_HCINTMSK0)
pub mod INTMSK0 {

    /// Transfer completed mask
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

    /// Channel halted mask
    pub mod CHHM {
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

    /// STALL response received interrupt mask
    pub mod STALLM {
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

    /// NAK response received interrupt mask
    pub mod NAKM {
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

    /// ACK response received/transmitted interrupt mask
    pub mod ACKM {
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

    /// response received interrupt mask
    pub mod NYET {
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

    /// Transaction error mask
    pub mod TXERRM {
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

    /// Babble error mask
    pub mod BBERRM {
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

    /// Frame overrun mask
    pub mod FRMORM {
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

    /// Data toggle error mask
    pub mod DTERRM {
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
}

/// OTG_FS host channel-0 transfer size register
pub mod TSIZ0 {

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

    /// Data PID
    pub mod DPID {
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

/// OTG_FS host channel-0 characteristics register (OTG_FS_HCCHAR0)
pub mod CHAR1 {
    pub use super::CHAR0::CHDIS;
    pub use super::CHAR0::CHENA;
    pub use super::CHAR0::DAD;
    pub use super::CHAR0::EPDIR;
    pub use super::CHAR0::EPNUM;
    pub use super::CHAR0::EPTYP;
    pub use super::CHAR0::LSDEV;
    pub use super::CHAR0::MCNT;
    pub use super::CHAR0::MPSIZ;
    pub use super::CHAR0::ODDFRM;
}

/// OTG_FS host channel-0 interrupt register (OTG_FS_HCINT0)
pub mod INT1 {
    pub use super::INT0::ACK;
    pub use super::INT0::BBERR;
    pub use super::INT0::CHH;
    pub use super::INT0::DTERR;
    pub use super::INT0::FRMOR;
    pub use super::INT0::NAK;
    pub use super::INT0::STALL;
    pub use super::INT0::TXERR;
    pub use super::INT0::XFRC;
}

/// OTG_FS host channel-0 mask register (OTG_FS_HCINTMSK0)
pub mod INTMSK1 {
    pub use super::INTMSK0::ACKM;
    pub use super::INTMSK0::BBERRM;
    pub use super::INTMSK0::CHHM;
    pub use super::INTMSK0::DTERRM;
    pub use super::INTMSK0::FRMORM;
    pub use super::INTMSK0::NAKM;
    pub use super::INTMSK0::NYET;
    pub use super::INTMSK0::STALLM;
    pub use super::INTMSK0::TXERRM;
    pub use super::INTMSK0::XFRCM;
}

/// OTG_FS host channel-0 transfer size register
pub mod TSIZ1 {
    pub use super::TSIZ0::DPID;
    pub use super::TSIZ0::PKTCNT;
    pub use super::TSIZ0::XFRSIZ;
}

/// OTG_FS host channel-0 characteristics register (OTG_FS_HCCHAR0)
pub mod CHAR2 {
    pub use super::CHAR0::CHDIS;
    pub use super::CHAR0::CHENA;
    pub use super::CHAR0::DAD;
    pub use super::CHAR0::EPDIR;
    pub use super::CHAR0::EPNUM;
    pub use super::CHAR0::EPTYP;
    pub use super::CHAR0::LSDEV;
    pub use super::CHAR0::MCNT;
    pub use super::CHAR0::MPSIZ;
    pub use super::CHAR0::ODDFRM;
}

/// OTG_FS host channel-0 interrupt register (OTG_FS_HCINT0)
pub mod INT2 {
    pub use super::INT0::ACK;
    pub use super::INT0::BBERR;
    pub use super::INT0::CHH;
    pub use super::INT0::DTERR;
    pub use super::INT0::FRMOR;
    pub use super::INT0::NAK;
    pub use super::INT0::STALL;
    pub use super::INT0::TXERR;
    pub use super::INT0::XFRC;
}

/// OTG_FS host channel-0 mask register (OTG_FS_HCINTMSK0)
pub mod INTMSK2 {
    pub use super::INTMSK0::ACKM;
    pub use super::INTMSK0::BBERRM;
    pub use super::INTMSK0::CHHM;
    pub use super::INTMSK0::DTERRM;
    pub use super::INTMSK0::FRMORM;
    pub use super::INTMSK0::NAKM;
    pub use super::INTMSK0::NYET;
    pub use super::INTMSK0::STALLM;
    pub use super::INTMSK0::TXERRM;
    pub use super::INTMSK0::XFRCM;
}

/// OTG_FS host channel-0 transfer size register
pub mod TSIZ2 {
    pub use super::TSIZ0::DPID;
    pub use super::TSIZ0::PKTCNT;
    pub use super::TSIZ0::XFRSIZ;
}

/// OTG_FS host channel-0 characteristics register (OTG_FS_HCCHAR0)
pub mod CHAR3 {
    pub use super::CHAR0::CHDIS;
    pub use super::CHAR0::CHENA;
    pub use super::CHAR0::DAD;
    pub use super::CHAR0::EPDIR;
    pub use super::CHAR0::EPNUM;
    pub use super::CHAR0::EPTYP;
    pub use super::CHAR0::LSDEV;
    pub use super::CHAR0::MCNT;
    pub use super::CHAR0::MPSIZ;
    pub use super::CHAR0::ODDFRM;
}

/// OTG_FS host channel-0 interrupt register (OTG_FS_HCINT0)
pub mod INT3 {
    pub use super::INT0::ACK;
    pub use super::INT0::BBERR;
    pub use super::INT0::CHH;
    pub use super::INT0::DTERR;
    pub use super::INT0::FRMOR;
    pub use super::INT0::NAK;
    pub use super::INT0::STALL;
    pub use super::INT0::TXERR;
    pub use super::INT0::XFRC;
}

/// OTG_FS host channel-0 mask register (OTG_FS_HCINTMSK0)
pub mod INTMSK3 {
    pub use super::INTMSK0::ACKM;
    pub use super::INTMSK0::BBERRM;
    pub use super::INTMSK0::CHHM;
    pub use super::INTMSK0::DTERRM;
    pub use super::INTMSK0::FRMORM;
    pub use super::INTMSK0::NAKM;
    pub use super::INTMSK0::NYET;
    pub use super::INTMSK0::STALLM;
    pub use super::INTMSK0::TXERRM;
    pub use super::INTMSK0::XFRCM;
}

/// OTG_FS host channel-0 transfer size register
pub mod TSIZ3 {
    pub use super::TSIZ0::DPID;
    pub use super::TSIZ0::PKTCNT;
    pub use super::TSIZ0::XFRSIZ;
}

/// OTG_FS host channel-0 characteristics register (OTG_FS_HCCHAR0)
pub mod CHAR4 {
    pub use super::CHAR0::CHDIS;
    pub use super::CHAR0::CHENA;
    pub use super::CHAR0::DAD;
    pub use super::CHAR0::EPDIR;
    pub use super::CHAR0::EPNUM;
    pub use super::CHAR0::EPTYP;
    pub use super::CHAR0::LSDEV;
    pub use super::CHAR0::MCNT;
    pub use super::CHAR0::MPSIZ;
    pub use super::CHAR0::ODDFRM;
}

/// OTG_FS host channel-0 interrupt register (OTG_FS_HCINT0)
pub mod INT4 {
    pub use super::INT0::ACK;
    pub use super::INT0::BBERR;
    pub use super::INT0::CHH;
    pub use super::INT0::DTERR;
    pub use super::INT0::FRMOR;
    pub use super::INT0::NAK;
    pub use super::INT0::STALL;
    pub use super::INT0::TXERR;
    pub use super::INT0::XFRC;
}

/// OTG_FS host channel-0 mask register (OTG_FS_HCINTMSK0)
pub mod INTMSK4 {
    pub use super::INTMSK0::ACKM;
    pub use super::INTMSK0::BBERRM;
    pub use super::INTMSK0::CHHM;
    pub use super::INTMSK0::DTERRM;
    pub use super::INTMSK0::FRMORM;
    pub use super::INTMSK0::NAKM;
    pub use super::INTMSK0::NYET;
    pub use super::INTMSK0::STALLM;
    pub use super::INTMSK0::TXERRM;
    pub use super::INTMSK0::XFRCM;
}

/// OTG_FS host channel-0 transfer size register
pub mod TSIZ4 {
    pub use super::TSIZ0::DPID;
    pub use super::TSIZ0::PKTCNT;
    pub use super::TSIZ0::XFRSIZ;
}

/// OTG_FS host channel-0 characteristics register (OTG_FS_HCCHAR0)
pub mod CHAR5 {
    pub use super::CHAR0::CHDIS;
    pub use super::CHAR0::CHENA;
    pub use super::CHAR0::DAD;
    pub use super::CHAR0::EPDIR;
    pub use super::CHAR0::EPNUM;
    pub use super::CHAR0::EPTYP;
    pub use super::CHAR0::LSDEV;
    pub use super::CHAR0::MCNT;
    pub use super::CHAR0::MPSIZ;
    pub use super::CHAR0::ODDFRM;
}

/// OTG_FS host channel-0 interrupt register (OTG_FS_HCINT0)
pub mod INT5 {
    pub use super::INT0::ACK;
    pub use super::INT0::BBERR;
    pub use super::INT0::CHH;
    pub use super::INT0::DTERR;
    pub use super::INT0::FRMOR;
    pub use super::INT0::NAK;
    pub use super::INT0::STALL;
    pub use super::INT0::TXERR;
    pub use super::INT0::XFRC;
}

/// OTG_FS host channel-0 mask register (OTG_FS_HCINTMSK0)
pub mod INTMSK5 {
    pub use super::INTMSK0::ACKM;
    pub use super::INTMSK0::BBERRM;
    pub use super::INTMSK0::CHHM;
    pub use super::INTMSK0::DTERRM;
    pub use super::INTMSK0::FRMORM;
    pub use super::INTMSK0::NAKM;
    pub use super::INTMSK0::NYET;
    pub use super::INTMSK0::STALLM;
    pub use super::INTMSK0::TXERRM;
    pub use super::INTMSK0::XFRCM;
}

/// OTG_FS host channel-0 transfer size register
pub mod TSIZ5 {
    pub use super::TSIZ0::DPID;
    pub use super::TSIZ0::PKTCNT;
    pub use super::TSIZ0::XFRSIZ;
}

/// OTG_FS host channel-0 characteristics register (OTG_FS_HCCHAR0)
pub mod CHAR6 {
    pub use super::CHAR0::CHDIS;
    pub use super::CHAR0::CHENA;
    pub use super::CHAR0::DAD;
    pub use super::CHAR0::EPDIR;
    pub use super::CHAR0::EPNUM;
    pub use super::CHAR0::EPTYP;
    pub use super::CHAR0::LSDEV;
    pub use super::CHAR0::MCNT;
    pub use super::CHAR0::MPSIZ;
    pub use super::CHAR0::ODDFRM;
}

/// OTG_FS host channel-0 interrupt register (OTG_FS_HCINT0)
pub mod INT6 {
    pub use super::INT0::ACK;
    pub use super::INT0::BBERR;
    pub use super::INT0::CHH;
    pub use super::INT0::DTERR;
    pub use super::INT0::FRMOR;
    pub use super::INT0::NAK;
    pub use super::INT0::STALL;
    pub use super::INT0::TXERR;
    pub use super::INT0::XFRC;
}

/// OTG_FS host channel-0 mask register (OTG_FS_HCINTMSK0)
pub mod INTMSK6 {
    pub use super::INTMSK0::ACKM;
    pub use super::INTMSK0::BBERRM;
    pub use super::INTMSK0::CHHM;
    pub use super::INTMSK0::DTERRM;
    pub use super::INTMSK0::FRMORM;
    pub use super::INTMSK0::NAKM;
    pub use super::INTMSK0::NYET;
    pub use super::INTMSK0::STALLM;
    pub use super::INTMSK0::TXERRM;
    pub use super::INTMSK0::XFRCM;
}

/// OTG_FS host channel-0 transfer size register
pub mod TSIZ6 {
    pub use super::TSIZ0::DPID;
    pub use super::TSIZ0::PKTCNT;
    pub use super::TSIZ0::XFRSIZ;
}

/// OTG_FS host channel-0 characteristics register (OTG_FS_HCCHAR0)
pub mod CHAR7 {
    pub use super::CHAR0::CHDIS;
    pub use super::CHAR0::CHENA;
    pub use super::CHAR0::DAD;
    pub use super::CHAR0::EPDIR;
    pub use super::CHAR0::EPNUM;
    pub use super::CHAR0::EPTYP;
    pub use super::CHAR0::LSDEV;
    pub use super::CHAR0::MCNT;
    pub use super::CHAR0::MPSIZ;
    pub use super::CHAR0::ODDFRM;
}

/// OTG_FS host channel-0 interrupt register (OTG_FS_HCINT0)
pub mod INT7 {
    pub use super::INT0::ACK;
    pub use super::INT0::BBERR;
    pub use super::INT0::CHH;
    pub use super::INT0::DTERR;
    pub use super::INT0::FRMOR;
    pub use super::INT0::NAK;
    pub use super::INT0::STALL;
    pub use super::INT0::TXERR;
    pub use super::INT0::XFRC;
}

/// OTG_FS host channel-0 mask register (OTG_FS_HCINTMSK0)
pub mod INTMSK7 {
    pub use super::INTMSK0::ACKM;
    pub use super::INTMSK0::BBERRM;
    pub use super::INTMSK0::CHHM;
    pub use super::INTMSK0::DTERRM;
    pub use super::INTMSK0::FRMORM;
    pub use super::INTMSK0::NAKM;
    pub use super::INTMSK0::NYET;
    pub use super::INTMSK0::STALLM;
    pub use super::INTMSK0::TXERRM;
    pub use super::INTMSK0::XFRCM;
}

/// OTG_FS host channel-0 transfer size register
pub mod TSIZ7 {
    pub use super::TSIZ0::DPID;
    pub use super::TSIZ0::PKTCNT;
    pub use super::TSIZ0::XFRSIZ;
}

/// OTG_FS host channel-0 characteristics register (OTG_FS_HCCHAR0)
pub mod CHAR8 {
    pub use super::CHAR0::CHDIS;
    pub use super::CHAR0::CHENA;
    pub use super::CHAR0::DAD;
    pub use super::CHAR0::EPDIR;
    pub use super::CHAR0::EPNUM;
    pub use super::CHAR0::EPTYP;
    pub use super::CHAR0::LSDEV;
    pub use super::CHAR0::MCNT;
    pub use super::CHAR0::MPSIZ;
    pub use super::CHAR0::ODDFRM;
}

/// OTG_FS host channel-0 interrupt register (OTG_FS_HCINT0)
pub mod INT8 {
    pub use super::INT0::ACK;
    pub use super::INT0::BBERR;
    pub use super::INT0::CHH;
    pub use super::INT0::DTERR;
    pub use super::INT0::FRMOR;
    pub use super::INT0::NAK;
    pub use super::INT0::STALL;
    pub use super::INT0::TXERR;
    pub use super::INT0::XFRC;
}

/// OTG_FS host channel-0 mask register (OTG_FS_HCINTMSK0)
pub mod INTMSK8 {
    pub use super::INTMSK0::ACKM;
    pub use super::INTMSK0::BBERRM;
    pub use super::INTMSK0::CHHM;
    pub use super::INTMSK0::DTERRM;
    pub use super::INTMSK0::FRMORM;
    pub use super::INTMSK0::NAKM;
    pub use super::INTMSK0::NYET;
    pub use super::INTMSK0::STALLM;
    pub use super::INTMSK0::TXERRM;
    pub use super::INTMSK0::XFRCM;
}

/// OTG_FS host channel-0 transfer size register
pub mod TSIZ8 {
    pub use super::TSIZ0::DPID;
    pub use super::TSIZ0::PKTCNT;
    pub use super::TSIZ0::XFRSIZ;
}

/// OTG_FS host channel-0 characteristics register (OTG_FS_HCCHAR0)
pub mod CHAR9 {
    pub use super::CHAR0::CHDIS;
    pub use super::CHAR0::CHENA;
    pub use super::CHAR0::DAD;
    pub use super::CHAR0::EPDIR;
    pub use super::CHAR0::EPNUM;
    pub use super::CHAR0::EPTYP;
    pub use super::CHAR0::LSDEV;
    pub use super::CHAR0::MCNT;
    pub use super::CHAR0::MPSIZ;
    pub use super::CHAR0::ODDFRM;
}

/// OTG_FS host channel-0 interrupt register (OTG_FS_HCINT0)
pub mod INT9 {
    pub use super::INT0::ACK;
    pub use super::INT0::BBERR;
    pub use super::INT0::CHH;
    pub use super::INT0::DTERR;
    pub use super::INT0::FRMOR;
    pub use super::INT0::NAK;
    pub use super::INT0::STALL;
    pub use super::INT0::TXERR;
    pub use super::INT0::XFRC;
}

/// OTG_FS host channel-0 mask register (OTG_FS_HCINTMSK0)
pub mod INTMSK9 {
    pub use super::INTMSK0::ACKM;
    pub use super::INTMSK0::BBERRM;
    pub use super::INTMSK0::CHHM;
    pub use super::INTMSK0::DTERRM;
    pub use super::INTMSK0::FRMORM;
    pub use super::INTMSK0::NAKM;
    pub use super::INTMSK0::NYET;
    pub use super::INTMSK0::STALLM;
    pub use super::INTMSK0::TXERRM;
    pub use super::INTMSK0::XFRCM;
}

/// OTG_FS host channel-0 transfer size register
pub mod TSIZ9 {
    pub use super::TSIZ0::DPID;
    pub use super::TSIZ0::PKTCNT;
    pub use super::TSIZ0::XFRSIZ;
}

/// OTG_FS host channel-0 characteristics register (OTG_FS_HCCHAR0)
pub mod CHAR10 {
    pub use super::CHAR0::CHDIS;
    pub use super::CHAR0::CHENA;
    pub use super::CHAR0::DAD;
    pub use super::CHAR0::EPDIR;
    pub use super::CHAR0::EPNUM;
    pub use super::CHAR0::EPTYP;
    pub use super::CHAR0::LSDEV;
    pub use super::CHAR0::MCNT;
    pub use super::CHAR0::MPSIZ;
    pub use super::CHAR0::ODDFRM;
}

/// OTG_FS host channel-0 interrupt register (OTG_FS_HCINT0)
pub mod INT10 {
    pub use super::INT0::ACK;
    pub use super::INT0::BBERR;
    pub use super::INT0::CHH;
    pub use super::INT0::DTERR;
    pub use super::INT0::FRMOR;
    pub use super::INT0::NAK;
    pub use super::INT0::STALL;
    pub use super::INT0::TXERR;
    pub use super::INT0::XFRC;
}

/// OTG_FS host channel-0 mask register (OTG_FS_HCINTMSK0)
pub mod INTMSK10 {
    pub use super::INTMSK0::ACKM;
    pub use super::INTMSK0::BBERRM;
    pub use super::INTMSK0::CHHM;
    pub use super::INTMSK0::DTERRM;
    pub use super::INTMSK0::FRMORM;
    pub use super::INTMSK0::NAKM;
    pub use super::INTMSK0::NYET;
    pub use super::INTMSK0::STALLM;
    pub use super::INTMSK0::TXERRM;
    pub use super::INTMSK0::XFRCM;
}

/// OTG_FS host channel-0 transfer size register
pub mod TSIZ10 {
    pub use super::TSIZ0::DPID;
    pub use super::TSIZ0::PKTCNT;
    pub use super::TSIZ0::XFRSIZ;
}

/// OTG_FS host channel-0 characteristics register (OTG_FS_HCCHAR0)
pub mod CHAR11 {
    pub use super::CHAR0::CHDIS;
    pub use super::CHAR0::CHENA;
    pub use super::CHAR0::DAD;
    pub use super::CHAR0::EPDIR;
    pub use super::CHAR0::EPNUM;
    pub use super::CHAR0::EPTYP;
    pub use super::CHAR0::LSDEV;
    pub use super::CHAR0::MCNT;
    pub use super::CHAR0::MPSIZ;
    pub use super::CHAR0::ODDFRM;
}

/// OTG_FS host channel-0 interrupt register (OTG_FS_HCINT0)
pub mod INT11 {
    pub use super::INT0::ACK;
    pub use super::INT0::BBERR;
    pub use super::INT0::CHH;
    pub use super::INT0::DTERR;
    pub use super::INT0::FRMOR;
    pub use super::INT0::NAK;
    pub use super::INT0::STALL;
    pub use super::INT0::TXERR;
    pub use super::INT0::XFRC;
}

/// OTG_FS host channel-0 mask register (OTG_FS_HCINTMSK0)
pub mod INTMSK11 {
    pub use super::INTMSK0::ACKM;
    pub use super::INTMSK0::BBERRM;
    pub use super::INTMSK0::CHHM;
    pub use super::INTMSK0::DTERRM;
    pub use super::INTMSK0::FRMORM;
    pub use super::INTMSK0::NAKM;
    pub use super::INTMSK0::NYET;
    pub use super::INTMSK0::STALLM;
    pub use super::INTMSK0::TXERRM;
    pub use super::INTMSK0::XFRCM;
}

/// OTG_FS host channel-0 transfer size register
pub mod TSIZ11 {
    pub use super::TSIZ0::DPID;
    pub use super::TSIZ0::PKTCNT;
    pub use super::TSIZ0::XFRSIZ;
}
#[repr(C)]
pub struct RegisterBlock {
    /// OTG_FS host configuration register (OTG_FS_HCFG)
    pub HCFG: RWRegister<u32>,

    /// OTG_FS Host frame interval register
    pub HFIR: RWRegister<u32>,

    /// OTG_FS host frame number/frame time remaining register (OTG_FS_HFNUM)
    pub HFNUM: RORegister<u32>,

    _reserved1: [u8; 4],

    /// OTG_FS_Host periodic transmit FIFO/queue status register (OTG_FS_HPTXSTS)
    pub HPTXSTS: RWRegister<u32>,

    /// OTG_FS Host all channels interrupt register
    pub HAINT: RORegister<u32>,

    /// OTG_FS host all channels interrupt mask register
    pub HAINTMSK: RWRegister<u32>,

    _reserved2: [u8; 36],

    /// OTG_FS host port control and status register (OTG_FS_HPRT)
    pub HPRT: RWRegister<u32>,

    _reserved3: [u8; 188],

    /// OTG_FS host channel-0 characteristics register (OTG_FS_HCCHAR0)
    pub CHAR0: RWRegister<u32>,

    _reserved4: [u8; 4],

    /// OTG_FS host channel-0 interrupt register (OTG_FS_HCINT0)
    pub INT0: RWRegister<u32>,

    /// OTG_FS host channel-0 mask register (OTG_FS_HCINTMSK0)
    pub INTMSK0: RWRegister<u32>,

    /// OTG_FS host channel-0 transfer size register
    pub TSIZ0: RWRegister<u32>,

    _reserved5: [u8; 12],

    /// OTG_FS host channel-0 characteristics register (OTG_FS_HCCHAR0)
    pub CHAR1: RWRegister<u32>,

    _reserved6: [u8; 4],

    /// OTG_FS host channel-0 interrupt register (OTG_FS_HCINT0)
    pub INT1: RWRegister<u32>,

    /// OTG_FS host channel-0 mask register (OTG_FS_HCINTMSK0)
    pub INTMSK1: RWRegister<u32>,

    /// OTG_FS host channel-0 transfer size register
    pub TSIZ1: RWRegister<u32>,

    _reserved7: [u8; 12],

    /// OTG_FS host channel-0 characteristics register (OTG_FS_HCCHAR0)
    pub CHAR2: RWRegister<u32>,

    _reserved8: [u8; 4],

    /// OTG_FS host channel-0 interrupt register (OTG_FS_HCINT0)
    pub INT2: RWRegister<u32>,

    /// OTG_FS host channel-0 mask register (OTG_FS_HCINTMSK0)
    pub INTMSK2: RWRegister<u32>,

    /// OTG_FS host channel-0 transfer size register
    pub TSIZ2: RWRegister<u32>,

    _reserved9: [u8; 12],

    /// OTG_FS host channel-0 characteristics register (OTG_FS_HCCHAR0)
    pub CHAR3: RWRegister<u32>,

    _reserved10: [u8; 4],

    /// OTG_FS host channel-0 interrupt register (OTG_FS_HCINT0)
    pub INT3: RWRegister<u32>,

    /// OTG_FS host channel-0 mask register (OTG_FS_HCINTMSK0)
    pub INTMSK3: RWRegister<u32>,

    /// OTG_FS host channel-0 transfer size register
    pub TSIZ3: RWRegister<u32>,

    _reserved11: [u8; 12],

    /// OTG_FS host channel-0 characteristics register (OTG_FS_HCCHAR0)
    pub CHAR4: RWRegister<u32>,

    _reserved12: [u8; 4],

    /// OTG_FS host channel-0 interrupt register (OTG_FS_HCINT0)
    pub INT4: RWRegister<u32>,

    /// OTG_FS host channel-0 mask register (OTG_FS_HCINTMSK0)
    pub INTMSK4: RWRegister<u32>,

    /// OTG_FS host channel-0 transfer size register
    pub TSIZ4: RWRegister<u32>,

    _reserved13: [u8; 12],

    /// OTG_FS host channel-0 characteristics register (OTG_FS_HCCHAR0)
    pub CHAR5: RWRegister<u32>,

    _reserved14: [u8; 4],

    /// OTG_FS host channel-0 interrupt register (OTG_FS_HCINT0)
    pub INT5: RWRegister<u32>,

    /// OTG_FS host channel-0 mask register (OTG_FS_HCINTMSK0)
    pub INTMSK5: RWRegister<u32>,

    /// OTG_FS host channel-0 transfer size register
    pub TSIZ5: RWRegister<u32>,

    _reserved15: [u8; 12],

    /// OTG_FS host channel-0 characteristics register (OTG_FS_HCCHAR0)
    pub CHAR6: RWRegister<u32>,

    _reserved16: [u8; 4],

    /// OTG_FS host channel-0 interrupt register (OTG_FS_HCINT0)
    pub INT6: RWRegister<u32>,

    /// OTG_FS host channel-0 mask register (OTG_FS_HCINTMSK0)
    pub INTMSK6: RWRegister<u32>,

    /// OTG_FS host channel-0 transfer size register
    pub TSIZ6: RWRegister<u32>,

    _reserved17: [u8; 12],

    /// OTG_FS host channel-0 characteristics register (OTG_FS_HCCHAR0)
    pub CHAR7: RWRegister<u32>,

    _reserved18: [u8; 4],

    /// OTG_FS host channel-0 interrupt register (OTG_FS_HCINT0)
    pub INT7: RWRegister<u32>,

    /// OTG_FS host channel-0 mask register (OTG_FS_HCINTMSK0)
    pub INTMSK7: RWRegister<u32>,

    /// OTG_FS host channel-0 transfer size register
    pub TSIZ7: RWRegister<u32>,

    _reserved19: [u8; 12],

    /// OTG_FS host channel-0 characteristics register (OTG_FS_HCCHAR0)
    pub CHAR8: RWRegister<u32>,

    _reserved20: [u8; 4],

    /// OTG_FS host channel-0 interrupt register (OTG_FS_HCINT0)
    pub INT8: RWRegister<u32>,

    /// OTG_FS host channel-0 mask register (OTG_FS_HCINTMSK0)
    pub INTMSK8: RWRegister<u32>,

    /// OTG_FS host channel-0 transfer size register
    pub TSIZ8: RWRegister<u32>,

    _reserved21: [u8; 12],

    /// OTG_FS host channel-0 characteristics register (OTG_FS_HCCHAR0)
    pub CHAR9: RWRegister<u32>,

    _reserved22: [u8; 4],

    /// OTG_FS host channel-0 interrupt register (OTG_FS_HCINT0)
    pub INT9: RWRegister<u32>,

    /// OTG_FS host channel-0 mask register (OTG_FS_HCINTMSK0)
    pub INTMSK9: RWRegister<u32>,

    /// OTG_FS host channel-0 transfer size register
    pub TSIZ9: RWRegister<u32>,

    _reserved23: [u8; 12],

    /// OTG_FS host channel-0 characteristics register (OTG_FS_HCCHAR0)
    pub CHAR10: RWRegister<u32>,

    _reserved24: [u8; 4],

    /// OTG_FS host channel-0 interrupt register (OTG_FS_HCINT0)
    pub INT10: RWRegister<u32>,

    /// OTG_FS host channel-0 mask register (OTG_FS_HCINTMSK0)
    pub INTMSK10: RWRegister<u32>,

    /// OTG_FS host channel-0 transfer size register
    pub TSIZ10: RWRegister<u32>,

    _reserved25: [u8; 12],

    /// OTG_FS host channel-0 characteristics register (OTG_FS_HCCHAR0)
    pub CHAR11: RWRegister<u32>,

    _reserved26: [u8; 4],

    /// OTG_FS host channel-0 interrupt register (OTG_FS_HCINT0)
    pub INT11: RWRegister<u32>,

    /// OTG_FS host channel-0 mask register (OTG_FS_HCINTMSK0)
    pub INTMSK11: RWRegister<u32>,

    /// OTG_FS host channel-0 transfer size register
    pub TSIZ11: RWRegister<u32>,
}
pub struct ResetValues {
    pub HCFG: u32,
    pub HFIR: u32,
    pub HFNUM: u32,
    pub HPTXSTS: u32,
    pub HAINT: u32,
    pub HAINTMSK: u32,
    pub HPRT: u32,
    pub CHAR0: u32,
    pub INT0: u32,
    pub INTMSK0: u32,
    pub TSIZ0: u32,
    pub CHAR1: u32,
    pub INT1: u32,
    pub INTMSK1: u32,
    pub TSIZ1: u32,
    pub CHAR2: u32,
    pub INT2: u32,
    pub INTMSK2: u32,
    pub TSIZ2: u32,
    pub CHAR3: u32,
    pub INT3: u32,
    pub INTMSK3: u32,
    pub TSIZ3: u32,
    pub CHAR4: u32,
    pub INT4: u32,
    pub INTMSK4: u32,
    pub TSIZ4: u32,
    pub CHAR5: u32,
    pub INT5: u32,
    pub INTMSK5: u32,
    pub TSIZ5: u32,
    pub CHAR6: u32,
    pub INT6: u32,
    pub INTMSK6: u32,
    pub TSIZ6: u32,
    pub CHAR7: u32,
    pub INT7: u32,
    pub INTMSK7: u32,
    pub TSIZ7: u32,
    pub CHAR8: u32,
    pub INT8: u32,
    pub INTMSK8: u32,
    pub TSIZ8: u32,
    pub CHAR9: u32,
    pub INT9: u32,
    pub INTMSK9: u32,
    pub TSIZ9: u32,
    pub CHAR10: u32,
    pub INT10: u32,
    pub INTMSK10: u32,
    pub TSIZ10: u32,
    pub CHAR11: u32,
    pub INT11: u32,
    pub INTMSK11: u32,
    pub TSIZ11: u32,
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
