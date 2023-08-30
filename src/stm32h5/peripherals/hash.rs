#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! HASH register bank
//!
//! Used by: stm32h562, stm32h563, stm32h573

use crate::{RORegister, RWRegister, WORegister};
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// HASH control register
pub mod CR {

    /// Initialize message digest calculation Writing this bit to 1 resets the hash processor core, so that the HASH is ready to compute the message digest of a new message. Writing this bit to 0 has no effect. Reading this bit always returns 0.
    pub mod INIT {
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

    /// DMA enable After this bit is set, it is cleared by hardware while the last data of the message is written into the hash processor. Setting this bit to 0 while a DMA transfer is ongoing does not abort the current transfer. Instead, the DMA interface of the HASH remains internally enabled until the transfer is completed or INIT is written to 1. Setting INIT bit to 1 does not clear DMAE bit.
    pub mod DMAE {
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

    /// Data type selection This bitfield defines the format of the data entered into the HASH_DIN register:
    pub mod DATATYPE {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (2 bits: 0b11 << 4)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Mode selection This bit selects the normal or the keyed HMAC mode for the selected algorithm: This selection is only taken into account when the INIT bit is set. Changing this bit during a computation has no effect.
    pub mod MODE {
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

    /// Number of words already pushed Refer to NBWP\[3:0\] bitfield of HASH_SR for a description of NBW\[3:0\] bitfield. This bit is read-only.
    pub mod NBW {
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

    /// DIN not empty Refer to DINNE bit of HASH_SR for a description of DINNE bit. This bit is read-only.
    pub mod DINNE {
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

    /// Multiple DMA transfers This bit is set when hashing large files when multiple DMA transfers are needed.
    pub mod MDMAT {
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

    /// Long key selection The application must set this bit if the HMAC key is greater than the block size corresponding to the hash algorithm (see algorithms for details). For example the block size is 64 bytes for SHA2-256. This selection is only taken into account when the INIT and MODE bits are set (HMAC mode selected). Changing this bit during a computation has no effect.
    pub mod LKEY {
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

    /// Algorithm selection These bits select the hash algorithm: This selection is only taken into account when the INIT bit is set. Changing this bitfield during a computation has no effect. When the ALGO bitfield is updated and INIT bit is set, NBWE in HASH_SR is automatically updated to 0x11.
    pub mod ALGO {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (4 bits: 0b1111 << 17)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// HASH data input register
pub mod DIN {

    /// Data input Writing this register pushes the current register content into the FIFO, and the register takes the new value presented on the AHB bus. Reading this register returns zeros.
    pub mod DATAIN {
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

/// HASH start register
pub mod STR {

    /// Number of valid bits in the last word When the last word of the message bit string is written to HASH_DIN register, the hash processor takes only the valid bits, specified as below, after internal data swapping: ... The above mechanism is valid only if DCAL = 0. If NBLW bits are written while DCAL is set to 1, the NBLW bitfield remains unchanged. In other words it is not possible to configure NBLW and set DCAL at the same time. Reading NBLW bits returns the last value written to NBLW.
    pub mod NBLW {
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

    /// Digest calculation Writing this bit to 1 starts the message padding using the previously written value of NBLW, and starts the calculation of the final message digest with all the data words written to the input FIFO since the INIT bit was last written to 1. Reading this bit returns 0.
    pub mod DCAL {
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
}

/// HASH aliased digest register 0
pub mod HRA0 {

    /// Hash data x Refer to introduction.
    pub mod Hx {
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

/// HASH aliased digest register 1
pub mod HRA1 {
    pub use super::HRA0::Hx;
}

/// HASH aliased digest register 2
pub mod HRA2 {
    pub use super::HRA0::Hx;
}

/// HASH aliased digest register 3
pub mod HRA3 {
    pub use super::HRA0::Hx;
}

/// HASH aliased digest register 4
pub mod HRA4 {
    pub use super::HRA0::Hx;
}

/// HASH interrupt enable register
pub mod IMR {

    /// Data input interrupt enable
    pub mod DINIE {
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

    /// Digest calculation completion interrupt enable
    pub mod DCIE {
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

/// HASH status register
pub mod SR {

    /// Data input interrupt status This bit is set by hardware when the FIFO is ready to get a new block (16 locations are free). It is cleared by writing it to 0 or by writing the HASH_DIN register. When DINIS = 0, HASH_CSRx registers reads as zero.
    pub mod DINIS {
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

    /// Digest calculation completion interrupt status This bit is set by hardware when a digest becomes ready (the whole message has been processed). It is cleared by writing it to 0 or by writing the INIT bit to 1 in the HASH_CR register.
    pub mod DCIS {
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

    /// DMA Status This bit provides information on the DMA interface activity. It is set with DMAE and cleared when DMAE = 0 and no DMA transfer is ongoing. No interrupt is associated with this bit.
    pub mod DMAS {
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

    /// Busy bit
    pub mod BUSY {
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

    /// Number of words already pushed This bitfield is the exact number of words in the message that have already been pushed into the FIFO. NBWP is incremented by 1 when a write access is performed to the HASH_DIN register. When a digest calculation starts, NBWP is updated to NBWP- block size (in words), and NBWP goes to zero when the INIT bit is written to 1.
    pub mod NBWP {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (5 bits: 0b11111 << 9)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// DIN not empty This bit is set when the HASH_DIN register holds valid data (that is after being written at least once). It is cleared when either the INIT bit (initialization) or the DCAL bit (completion of the previous message processing) is written to 1.
    pub mod DINNE {
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

    /// Number of words expected This bitfield reflects the number of words in the message that must be pushed into the FIFO to trigger a partial computation. NBWE is decremented by 1 when a write access is performed to the HASH_DIN register. NBWE is set to the expected block size +1 in words (0x11) when INIT bit is set in HASH_CR. It is set to the expected block size (0x10) when the partial digest calculation ends.
    pub mod NBWE {
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
}

/// HASH context swap register 0
pub mod CSR0 {

    /// Context swap 0 Refer to introduction.
    pub mod CSx {
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

/// HASH context swap register 1
pub mod CSR1 {
    pub use super::CSR0::CSx;
}

/// HASH context swap register 2
pub mod CSR2 {
    pub use super::CSR0::CSx;
}

/// HASH context swap register 3
pub mod CSR3 {
    pub use super::CSR0::CSx;
}

/// HASH context swap register 4
pub mod CSR4 {
    pub use super::CSR0::CSx;
}

/// HASH context swap register 5
pub mod CSR5 {
    pub use super::CSR0::CSx;
}

/// HASH context swap register 6
pub mod CSR6 {
    pub use super::CSR0::CSx;
}

/// HASH context swap register 7
pub mod CSR7 {
    pub use super::CSR0::CSx;
}

/// HASH context swap register 8
pub mod CSR8 {
    pub use super::CSR0::CSx;
}

/// HASH context swap register 9
pub mod CSR9 {
    pub use super::CSR0::CSx;
}

/// HASH context swap register 10
pub mod CSR10 {
    pub use super::CSR0::CSx;
}

/// HASH context swap register 11
pub mod CSR11 {
    pub use super::CSR0::CSx;
}

/// HASH context swap register 12
pub mod CSR12 {
    pub use super::CSR0::CSx;
}

/// HASH context swap register 13
pub mod CSR13 {
    pub use super::CSR0::CSx;
}

/// HASH context swap register 14
pub mod CSR14 {
    pub use super::CSR0::CSx;
}

/// HASH context swap register 15
pub mod CSR15 {
    pub use super::CSR0::CSx;
}

/// HASH context swap register 16
pub mod CSR16 {
    pub use super::CSR0::CSx;
}

/// HASH context swap register 17
pub mod CSR17 {
    pub use super::CSR0::CSx;
}

/// HASH context swap register 18
pub mod CSR18 {
    pub use super::CSR0::CSx;
}

/// HASH context swap register 19
pub mod CSR19 {
    pub use super::CSR0::CSx;
}

/// HASH context swap register 20
pub mod CSR20 {
    pub use super::CSR0::CSx;
}

/// HASH context swap register 21
pub mod CSR21 {
    pub use super::CSR0::CSx;
}

/// HASH context swap register 22
pub mod CSR22 {
    pub use super::CSR0::CSx;
}

/// HASH context swap register 23
pub mod CSR23 {
    pub use super::CSR0::CSx;
}

/// HASH context swap register 24
pub mod CSR24 {
    pub use super::CSR0::CSx;
}

/// HASH context swap register 25
pub mod CSR25 {
    pub use super::CSR0::CSx;
}

/// HASH context swap register 26
pub mod CSR26 {
    pub use super::CSR0::CSx;
}

/// HASH context swap register 27
pub mod CSR27 {
    pub use super::CSR0::CSx;
}

/// HASH context swap register 28
pub mod CSR28 {
    pub use super::CSR0::CSx;
}

/// HASH context swap register 29
pub mod CSR29 {
    pub use super::CSR0::CSx;
}

/// HASH context swap register 30
pub mod CSR30 {
    pub use super::CSR0::CSx;
}

/// HASH context swap register 31
pub mod CSR31 {
    pub use super::CSR0::CSx;
}

/// HASH context swap register 32
pub mod CSR32 {
    pub use super::CSR0::CSx;
}

/// HASH context swap register 33
pub mod CSR33 {
    pub use super::CSR0::CSx;
}

/// HASH context swap register 34
pub mod CSR34 {
    pub use super::CSR0::CSx;
}

/// HASH context swap register 35
pub mod CSR35 {
    pub use super::CSR0::CSx;
}

/// HASH context swap register 36
pub mod CSR36 {
    pub use super::CSR0::CSx;
}

/// HASH context swap register 37
pub mod CSR37 {
    pub use super::CSR0::CSx;
}

/// HASH context swap register 38
pub mod CSR38 {
    pub use super::CSR0::CSx;
}

/// HASH context swap register 39
pub mod CSR39 {
    pub use super::CSR0::CSx;
}

/// HASH context swap register 40
pub mod CSR40 {
    pub use super::CSR0::CSx;
}

/// HASH context swap register 41
pub mod CSR41 {
    pub use super::CSR0::CSx;
}

/// HASH context swap register 42
pub mod CSR42 {
    pub use super::CSR0::CSx;
}

/// HASH context swap register 43
pub mod CSR43 {
    pub use super::CSR0::CSx;
}

/// HASH context swap register 44
pub mod CSR44 {
    pub use super::CSR0::CSx;
}

/// HASH context swap register 45
pub mod CSR45 {
    pub use super::CSR0::CSx;
}

/// HASH context swap register 46
pub mod CSR46 {
    pub use super::CSR0::CSx;
}

/// HASH context swap register 47
pub mod CSR47 {
    pub use super::CSR0::CSx;
}

/// HASH context swap register 48
pub mod CSR48 {
    pub use super::CSR0::CSx;
}

/// HASH context swap register 49
pub mod CSR49 {
    pub use super::CSR0::CSx;
}

/// HASH context swap register 50
pub mod CSR50 {
    pub use super::CSR0::CSx;
}

/// HASH context swap register 51
pub mod CSR51 {
    pub use super::CSR0::CSx;
}

/// HASH context swap register 52
pub mod CSR52 {
    pub use super::CSR0::CSx;
}

/// HASH context swap register 53
pub mod CSR53 {
    pub use super::CSR0::CSx;
}

/// HASH context swap register 54
pub mod CSR54 {
    pub use super::CSR0::CSx;
}

/// HASH context swap register 55
pub mod CSR55 {
    pub use super::CSR0::CSx;
}

/// HASH context swap register 56
pub mod CSR56 {
    pub use super::CSR0::CSx;
}

/// HASH context swap register 57
pub mod CSR57 {
    pub use super::CSR0::CSx;
}

/// HASH context swap register 58
pub mod CSR58 {
    pub use super::CSR0::CSx;
}

/// HASH context swap register 59
pub mod CSR59 {
    pub use super::CSR0::CSx;
}

/// HASH context swap register 60
pub mod CSR60 {
    pub use super::CSR0::CSx;
}

/// HASH context swap register 61
pub mod CSR61 {
    pub use super::CSR0::CSx;
}

/// HASH context swap register 62
pub mod CSR62 {
    pub use super::CSR0::CSx;
}

/// HASH context swap register 63
pub mod CSR63 {
    pub use super::CSR0::CSx;
}

/// HASH context swap register 64
pub mod CSR64 {
    pub use super::CSR0::CSx;
}

/// HASH context swap register 65
pub mod CSR65 {
    pub use super::CSR0::CSx;
}

/// HASH context swap register 66
pub mod CSR66 {
    pub use super::CSR0::CSx;
}

/// HASH context swap register 67
pub mod CSR67 {
    pub use super::CSR0::CSx;
}

/// HASH context swap register 68
pub mod CSR68 {
    pub use super::CSR0::CSx;
}

/// HASH context swap register 69
pub mod CSR69 {
    pub use super::CSR0::CSx;
}

/// HASH context swap register 70
pub mod CSR70 {
    pub use super::CSR0::CSx;
}

/// HASH context swap register 71
pub mod CSR71 {
    pub use super::CSR0::CSx;
}

/// HASH context swap register 72
pub mod CSR72 {
    pub use super::CSR0::CSx;
}

/// HASH context swap register 73
pub mod CSR73 {
    pub use super::CSR0::CSx;
}

/// HASH context swap register 74
pub mod CSR74 {
    pub use super::CSR0::CSx;
}

/// HASH context swap register 75
pub mod CSR75 {
    pub use super::CSR0::CSx;
}

/// HASH context swap register 76
pub mod CSR76 {
    pub use super::CSR0::CSx;
}

/// HASH context swap register 77
pub mod CSR77 {
    pub use super::CSR0::CSx;
}

/// HASH context swap register 78
pub mod CSR78 {
    pub use super::CSR0::CSx;
}

/// HASH context swap register 79
pub mod CSR79 {
    pub use super::CSR0::CSx;
}

/// HASH context swap register 80
pub mod CSR80 {
    pub use super::CSR0::CSx;
}

/// HASH context swap register 81
pub mod CSR81 {
    pub use super::CSR0::CSx;
}

/// HASH context swap register 82
pub mod CSR82 {
    pub use super::CSR0::CSx;
}

/// HASH context swap register 83
pub mod CSR83 {
    pub use super::CSR0::CSx;
}

/// HASH context swap register 84
pub mod CSR84 {
    pub use super::CSR0::CSx;
}

/// HASH context swap register 85
pub mod CSR85 {
    pub use super::CSR0::CSx;
}

/// HASH context swap register 86
pub mod CSR86 {
    pub use super::CSR0::CSx;
}

/// HASH context swap register 87
pub mod CSR87 {
    pub use super::CSR0::CSx;
}

/// HASH context swap register 88
pub mod CSR88 {
    pub use super::CSR0::CSx;
}

/// HASH context swap register 89
pub mod CSR89 {
    pub use super::CSR0::CSx;
}

/// HASH context swap register 90
pub mod CSR90 {
    pub use super::CSR0::CSx;
}

/// HASH context swap register 91
pub mod CSR91 {
    pub use super::CSR0::CSx;
}

/// HASH context swap register 92
pub mod CSR92 {
    pub use super::CSR0::CSx;
}

/// HASH context swap register 93
pub mod CSR93 {
    pub use super::CSR0::CSx;
}

/// HASH context swap register 94
pub mod CSR94 {
    pub use super::CSR0::CSx;
}

/// HASH context swap register 95
pub mod CSR95 {
    pub use super::CSR0::CSx;
}

/// HASH context swap register 96
pub mod CSR96 {
    pub use super::CSR0::CSx;
}

/// HASH context swap register 97
pub mod CSR97 {
    pub use super::CSR0::CSx;
}

/// HASH context swap register 98
pub mod CSR98 {
    pub use super::CSR0::CSx;
}

/// HASH context swap register 99
pub mod CSR99 {
    pub use super::CSR0::CSx;
}

/// HASH context swap register 100
pub mod CSR100 {
    pub use super::CSR0::CSx;
}

/// HASH context swap register 101
pub mod CSR101 {
    pub use super::CSR0::CSx;
}

/// HASH context swap register 102
pub mod CSR102 {
    pub use super::CSR0::CSx;
}

/// HASH digest register
pub mod HR0 {
    pub use super::HRA0::Hx;
}

/// HASH digest register
pub mod HR1 {
    pub use super::HRA0::Hx;
}

/// HASH digest register
pub mod HR2 {
    pub use super::HRA0::Hx;
}

/// HASH digest register
pub mod HR3 {
    pub use super::HRA0::Hx;
}

/// HASH digest register
pub mod HR4 {
    pub use super::HRA0::Hx;
}

/// HASH digest register
pub mod HR5 {
    pub use super::HRA0::Hx;
}

/// HASH digest register
pub mod HR6 {
    pub use super::HRA0::Hx;
}

/// HASH digest register
pub mod HR7 {
    pub use super::HRA0::Hx;
}

/// HASH digest register
pub mod HR8 {
    pub use super::HRA0::Hx;
}

/// HASH digest register
pub mod HR9 {
    pub use super::HRA0::Hx;
}

/// HASH digest register
pub mod HR10 {
    pub use super::HRA0::Hx;
}

/// HASH digest register
pub mod HR11 {
    pub use super::HRA0::Hx;
}

/// HASH digest register
pub mod HR12 {
    pub use super::HRA0::Hx;
}

/// HASH digest register
pub mod HR13 {
    pub use super::HRA0::Hx;
}

/// HASH digest register
pub mod HR14 {
    pub use super::HRA0::Hx;
}

/// HASH digest register
pub mod HR15 {
    pub use super::HRA0::Hx;
}
#[repr(C)]
pub struct RegisterBlock {
    /// HASH control register
    pub CR: RWRegister<u32>,

    /// HASH data input register
    pub DIN: WORegister<u32>,

    /// HASH start register
    pub STR: RWRegister<u32>,

    /// HASH aliased digest register 0
    pub HRA0: RORegister<u32>,

    /// HASH aliased digest register 1
    pub HRA1: RORegister<u32>,

    /// HASH aliased digest register 2
    pub HRA2: RORegister<u32>,

    /// HASH aliased digest register 3
    pub HRA3: RORegister<u32>,

    /// HASH aliased digest register 4
    pub HRA4: RORegister<u32>,

    /// HASH interrupt enable register
    pub IMR: RWRegister<u32>,

    /// HASH status register
    pub SR: RWRegister<u32>,

    _reserved1: [u8; 208],

    /// HASH context swap register 0
    pub CSR0: RWRegister<u32>,

    /// HASH context swap register 1
    pub CSR1: RWRegister<u32>,

    /// HASH context swap register 2
    pub CSR2: RWRegister<u32>,

    /// HASH context swap register 3
    pub CSR3: RWRegister<u32>,

    /// HASH context swap register 4
    pub CSR4: RWRegister<u32>,

    /// HASH context swap register 5
    pub CSR5: RWRegister<u32>,

    /// HASH context swap register 6
    pub CSR6: RWRegister<u32>,

    /// HASH context swap register 7
    pub CSR7: RWRegister<u32>,

    /// HASH context swap register 8
    pub CSR8: RWRegister<u32>,

    /// HASH context swap register 9
    pub CSR9: RWRegister<u32>,

    /// HASH context swap register 10
    pub CSR10: RWRegister<u32>,

    /// HASH context swap register 11
    pub CSR11: RWRegister<u32>,

    /// HASH context swap register 12
    pub CSR12: RWRegister<u32>,

    /// HASH context swap register 13
    pub CSR13: RWRegister<u32>,

    /// HASH context swap register 14
    pub CSR14: RWRegister<u32>,

    /// HASH context swap register 15
    pub CSR15: RWRegister<u32>,

    /// HASH context swap register 16
    pub CSR16: RWRegister<u32>,

    /// HASH context swap register 17
    pub CSR17: RWRegister<u32>,

    /// HASH context swap register 18
    pub CSR18: RWRegister<u32>,

    /// HASH context swap register 19
    pub CSR19: RWRegister<u32>,

    /// HASH context swap register 20
    pub CSR20: RWRegister<u32>,

    /// HASH context swap register 21
    pub CSR21: RWRegister<u32>,

    /// HASH context swap register 22
    pub CSR22: RWRegister<u32>,

    /// HASH context swap register 23
    pub CSR23: RWRegister<u32>,

    /// HASH context swap register 24
    pub CSR24: RWRegister<u32>,

    /// HASH context swap register 25
    pub CSR25: RWRegister<u32>,

    /// HASH context swap register 26
    pub CSR26: RWRegister<u32>,

    /// HASH context swap register 27
    pub CSR27: RWRegister<u32>,

    /// HASH context swap register 28
    pub CSR28: RWRegister<u32>,

    /// HASH context swap register 29
    pub CSR29: RWRegister<u32>,

    /// HASH context swap register 30
    pub CSR30: RWRegister<u32>,

    /// HASH context swap register 31
    pub CSR31: RWRegister<u32>,

    /// HASH context swap register 32
    pub CSR32: RWRegister<u32>,

    /// HASH context swap register 33
    pub CSR33: RWRegister<u32>,

    /// HASH context swap register 34
    pub CSR34: RWRegister<u32>,

    /// HASH context swap register 35
    pub CSR35: RWRegister<u32>,

    /// HASH context swap register 36
    pub CSR36: RWRegister<u32>,

    /// HASH context swap register 37
    pub CSR37: RWRegister<u32>,

    /// HASH context swap register 38
    pub CSR38: RWRegister<u32>,

    /// HASH context swap register 39
    pub CSR39: RWRegister<u32>,

    /// HASH context swap register 40
    pub CSR40: RWRegister<u32>,

    /// HASH context swap register 41
    pub CSR41: RWRegister<u32>,

    /// HASH context swap register 42
    pub CSR42: RWRegister<u32>,

    /// HASH context swap register 43
    pub CSR43: RWRegister<u32>,

    /// HASH context swap register 44
    pub CSR44: RWRegister<u32>,

    /// HASH context swap register 45
    pub CSR45: RWRegister<u32>,

    /// HASH context swap register 46
    pub CSR46: RWRegister<u32>,

    /// HASH context swap register 47
    pub CSR47: RWRegister<u32>,

    /// HASH context swap register 48
    pub CSR48: RWRegister<u32>,

    /// HASH context swap register 49
    pub CSR49: RWRegister<u32>,

    /// HASH context swap register 50
    pub CSR50: RWRegister<u32>,

    /// HASH context swap register 51
    pub CSR51: RWRegister<u32>,

    /// HASH context swap register 52
    pub CSR52: RWRegister<u32>,

    /// HASH context swap register 53
    pub CSR53: RWRegister<u32>,

    /// HASH context swap register 54
    pub CSR54: RWRegister<u32>,

    /// HASH context swap register 55
    pub CSR55: RWRegister<u32>,

    /// HASH context swap register 56
    pub CSR56: RWRegister<u32>,

    /// HASH context swap register 57
    pub CSR57: RWRegister<u32>,

    /// HASH context swap register 58
    pub CSR58: RWRegister<u32>,

    /// HASH context swap register 59
    pub CSR59: RWRegister<u32>,

    /// HASH context swap register 60
    pub CSR60: RWRegister<u32>,

    /// HASH context swap register 61
    pub CSR61: RWRegister<u32>,

    /// HASH context swap register 62
    pub CSR62: RWRegister<u32>,

    /// HASH context swap register 63
    pub CSR63: RWRegister<u32>,

    /// HASH context swap register 64
    pub CSR64: RWRegister<u32>,

    /// HASH context swap register 65
    pub CSR65: RWRegister<u32>,

    /// HASH context swap register 66
    pub CSR66: RWRegister<u32>,

    /// HASH context swap register 67
    pub CSR67: RWRegister<u32>,

    /// HASH context swap register 68
    pub CSR68: RWRegister<u32>,

    /// HASH context swap register 69
    pub CSR69: RWRegister<u32>,

    /// HASH context swap register 70
    pub CSR70: RWRegister<u32>,

    /// HASH context swap register 71
    pub CSR71: RWRegister<u32>,

    /// HASH context swap register 72
    pub CSR72: RWRegister<u32>,

    /// HASH context swap register 73
    pub CSR73: RWRegister<u32>,

    /// HASH context swap register 74
    pub CSR74: RWRegister<u32>,

    /// HASH context swap register 75
    pub CSR75: RWRegister<u32>,

    /// HASH context swap register 76
    pub CSR76: RWRegister<u32>,

    /// HASH context swap register 77
    pub CSR77: RWRegister<u32>,

    /// HASH context swap register 78
    pub CSR78: RWRegister<u32>,

    /// HASH context swap register 79
    pub CSR79: RWRegister<u32>,

    /// HASH context swap register 80
    pub CSR80: RWRegister<u32>,

    /// HASH context swap register 81
    pub CSR81: RWRegister<u32>,

    /// HASH context swap register 82
    pub CSR82: RWRegister<u32>,

    /// HASH context swap register 83
    pub CSR83: RWRegister<u32>,

    /// HASH context swap register 84
    pub CSR84: RWRegister<u32>,

    /// HASH context swap register 85
    pub CSR85: RWRegister<u32>,

    /// HASH context swap register 86
    pub CSR86: RWRegister<u32>,

    /// HASH context swap register 87
    pub CSR87: RWRegister<u32>,

    /// HASH context swap register 88
    pub CSR88: RWRegister<u32>,

    /// HASH context swap register 89
    pub CSR89: RWRegister<u32>,

    /// HASH context swap register 90
    pub CSR90: RWRegister<u32>,

    /// HASH context swap register 91
    pub CSR91: RWRegister<u32>,

    /// HASH context swap register 92
    pub CSR92: RWRegister<u32>,

    /// HASH context swap register 93
    pub CSR93: RWRegister<u32>,

    /// HASH context swap register 94
    pub CSR94: RWRegister<u32>,

    /// HASH context swap register 95
    pub CSR95: RWRegister<u32>,

    /// HASH context swap register 96
    pub CSR96: RWRegister<u32>,

    /// HASH context swap register 97
    pub CSR97: RWRegister<u32>,

    /// HASH context swap register 98
    pub CSR98: RWRegister<u32>,

    /// HASH context swap register 99
    pub CSR99: RWRegister<u32>,

    /// HASH context swap register 100
    pub CSR100: RWRegister<u32>,

    /// HASH context swap register 101
    pub CSR101: RWRegister<u32>,

    /// HASH context swap register 102
    pub CSR102: RWRegister<u32>,

    _reserved2: [u8; 124],

    /// HASH digest register
    pub HR0: RORegister<u32>,

    /// HASH digest register
    pub HR1: RORegister<u32>,

    /// HASH digest register
    pub HR2: RORegister<u32>,

    /// HASH digest register
    pub HR3: RORegister<u32>,

    /// HASH digest register
    pub HR4: RORegister<u32>,

    /// HASH digest register
    pub HR5: RORegister<u32>,

    /// HASH digest register
    pub HR6: RORegister<u32>,

    /// HASH digest register
    pub HR7: RORegister<u32>,

    /// HASH digest register
    pub HR8: RORegister<u32>,

    /// HASH digest register
    pub HR9: RORegister<u32>,

    /// HASH digest register
    pub HR10: RORegister<u32>,

    /// HASH digest register
    pub HR11: RORegister<u32>,

    /// HASH digest register
    pub HR12: RORegister<u32>,

    /// HASH digest register
    pub HR13: RORegister<u32>,

    /// HASH digest register
    pub HR14: RORegister<u32>,

    /// HASH digest register
    pub HR15: RORegister<u32>,
}
pub struct ResetValues {
    pub CR: u32,
    pub DIN: u32,
    pub STR: u32,
    pub HRA0: u32,
    pub HRA1: u32,
    pub HRA2: u32,
    pub HRA3: u32,
    pub HRA4: u32,
    pub IMR: u32,
    pub SR: u32,
    pub CSR0: u32,
    pub CSR1: u32,
    pub CSR2: u32,
    pub CSR3: u32,
    pub CSR4: u32,
    pub CSR5: u32,
    pub CSR6: u32,
    pub CSR7: u32,
    pub CSR8: u32,
    pub CSR9: u32,
    pub CSR10: u32,
    pub CSR11: u32,
    pub CSR12: u32,
    pub CSR13: u32,
    pub CSR14: u32,
    pub CSR15: u32,
    pub CSR16: u32,
    pub CSR17: u32,
    pub CSR18: u32,
    pub CSR19: u32,
    pub CSR20: u32,
    pub CSR21: u32,
    pub CSR22: u32,
    pub CSR23: u32,
    pub CSR24: u32,
    pub CSR25: u32,
    pub CSR26: u32,
    pub CSR27: u32,
    pub CSR28: u32,
    pub CSR29: u32,
    pub CSR30: u32,
    pub CSR31: u32,
    pub CSR32: u32,
    pub CSR33: u32,
    pub CSR34: u32,
    pub CSR35: u32,
    pub CSR36: u32,
    pub CSR37: u32,
    pub CSR38: u32,
    pub CSR39: u32,
    pub CSR40: u32,
    pub CSR41: u32,
    pub CSR42: u32,
    pub CSR43: u32,
    pub CSR44: u32,
    pub CSR45: u32,
    pub CSR46: u32,
    pub CSR47: u32,
    pub CSR48: u32,
    pub CSR49: u32,
    pub CSR50: u32,
    pub CSR51: u32,
    pub CSR52: u32,
    pub CSR53: u32,
    pub CSR54: u32,
    pub CSR55: u32,
    pub CSR56: u32,
    pub CSR57: u32,
    pub CSR58: u32,
    pub CSR59: u32,
    pub CSR60: u32,
    pub CSR61: u32,
    pub CSR62: u32,
    pub CSR63: u32,
    pub CSR64: u32,
    pub CSR65: u32,
    pub CSR66: u32,
    pub CSR67: u32,
    pub CSR68: u32,
    pub CSR69: u32,
    pub CSR70: u32,
    pub CSR71: u32,
    pub CSR72: u32,
    pub CSR73: u32,
    pub CSR74: u32,
    pub CSR75: u32,
    pub CSR76: u32,
    pub CSR77: u32,
    pub CSR78: u32,
    pub CSR79: u32,
    pub CSR80: u32,
    pub CSR81: u32,
    pub CSR82: u32,
    pub CSR83: u32,
    pub CSR84: u32,
    pub CSR85: u32,
    pub CSR86: u32,
    pub CSR87: u32,
    pub CSR88: u32,
    pub CSR89: u32,
    pub CSR90: u32,
    pub CSR91: u32,
    pub CSR92: u32,
    pub CSR93: u32,
    pub CSR94: u32,
    pub CSR95: u32,
    pub CSR96: u32,
    pub CSR97: u32,
    pub CSR98: u32,
    pub CSR99: u32,
    pub CSR100: u32,
    pub CSR101: u32,
    pub CSR102: u32,
    pub HR0: u32,
    pub HR1: u32,
    pub HR2: u32,
    pub HR3: u32,
    pub HR4: u32,
    pub HR5: u32,
    pub HR6: u32,
    pub HR7: u32,
    pub HR8: u32,
    pub HR9: u32,
    pub HR10: u32,
    pub HR11: u32,
    pub HR12: u32,
    pub HR13: u32,
    pub HR14: u32,
    pub HR15: u32,
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
