#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Hash processor

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

    /// Long key selection The application must set this bit if the HMAC key is greater than the block size (64 bytes) This selection is only taken into account when the INIT and MODE bits are set (HMAC mode selected). Changing this bit during a computation has no effect.
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

    /// Hash data x Refer to Section 24.7.4: HASH digest registers introduction.
    pub mod H0 {
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

    /// Hash data x Refer to Section 24.7.4: HASH digest registers introduction.
    pub mod H1 {
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

/// HASH aliased digest register 2
pub mod HRA2 {

    /// Hash data x Refer to Section 24.7.4: HASH digest registers introduction.
    pub mod H2 {
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

/// HASH aliased digest register 3
pub mod HRA3 {

    /// Hash data x Refer to Section 24.7.4: HASH digest registers introduction.
    pub mod H3 {
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

/// HASH aliased digest register 4
pub mod HRA4 {

    /// Hash data x Refer to Section 24.7.4: HASH digest registers introduction.
    pub mod H4 {
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

    /// Context swap x Refer to Section 24.7.7: HASH context swap registers introduction.
    pub mod CS0 {
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

    /// Context swap x Refer to Section 24.7.7: HASH context swap registers introduction.
    pub mod CS1 {
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

/// HASH context swap register 2
pub mod CSR2 {

    /// Context swap x Refer to Section 24.7.7: HASH context swap registers introduction.
    pub mod CS2 {
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

/// HASH context swap register 3
pub mod CSR3 {

    /// Context swap x Refer to Section 24.7.7: HASH context swap registers introduction.
    pub mod CS3 {
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

/// HASH context swap register 4
pub mod CSR4 {

    /// Context swap x Refer to Section 24.7.7: HASH context swap registers introduction.
    pub mod CS4 {
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

/// HASH context swap register 5
pub mod CSR5 {

    /// Context swap x Refer to Section 24.7.7: HASH context swap registers introduction.
    pub mod CS5 {
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

/// HASH context swap register 6
pub mod CSR6 {

    /// Context swap x Refer to Section 24.7.7: HASH context swap registers introduction.
    pub mod CS6 {
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

/// HASH context swap register 7
pub mod CSR7 {

    /// Context swap x Refer to Section 24.7.7: HASH context swap registers introduction.
    pub mod CS7 {
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

/// HASH context swap register 8
pub mod CSR8 {

    /// Context swap x Refer to Section 24.7.7: HASH context swap registers introduction.
    pub mod CS8 {
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

/// HASH context swap register 9
pub mod CSR9 {

    /// Context swap x Refer to Section 24.7.7: HASH context swap registers introduction.
    pub mod CS9 {
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

/// HASH context swap register 10
pub mod CSR10 {

    /// Context swap x Refer to Section 24.7.7: HASH context swap registers introduction.
    pub mod CS10 {
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

/// HASH context swap register 11
pub mod CSR11 {

    /// Context swap x Refer to Section 24.7.7: HASH context swap registers introduction.
    pub mod CS11 {
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

/// HASH context swap register 12
pub mod CSR12 {

    /// Context swap x Refer to Section 24.7.7: HASH context swap registers introduction.
    pub mod CS12 {
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

/// HASH context swap register 13
pub mod CSR13 {

    /// Context swap x Refer to Section 24.7.7: HASH context swap registers introduction.
    pub mod CS13 {
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

/// HASH context swap register 14
pub mod CSR14 {

    /// Context swap x Refer to Section 24.7.7: HASH context swap registers introduction.
    pub mod CS14 {
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

/// HASH context swap register 15
pub mod CSR15 {

    /// Context swap x Refer to Section 24.7.7: HASH context swap registers introduction.
    pub mod CS15 {
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

/// HASH context swap register 16
pub mod CSR16 {

    /// Context swap x Refer to Section 24.7.7: HASH context swap registers introduction.
    pub mod CS16 {
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

/// HASH context swap register 17
pub mod CSR17 {

    /// Context swap x Refer to Section 24.7.7: HASH context swap registers introduction.
    pub mod CS17 {
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

/// HASH context swap register 18
pub mod CSR18 {

    /// Context swap x Refer to Section 24.7.7: HASH context swap registers introduction.
    pub mod CS18 {
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

/// HASH context swap register 19
pub mod CSR19 {

    /// Context swap x Refer to Section 24.7.7: HASH context swap registers introduction.
    pub mod CS19 {
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

/// HASH context swap register 20
pub mod CSR20 {

    /// Context swap x Refer to Section 24.7.7: HASH context swap registers introduction.
    pub mod CS20 {
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

/// HASH context swap register 21
pub mod CSR21 {

    /// Context swap x Refer to Section 24.7.7: HASH context swap registers introduction.
    pub mod CS21 {
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

/// HASH context swap register 22
pub mod CSR22 {

    /// Context swap x Refer to Section 24.7.7: HASH context swap registers introduction.
    pub mod CS22 {
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

/// HASH context swap register 23
pub mod CSR23 {

    /// Context swap x Refer to Section 24.7.7: HASH context swap registers introduction.
    pub mod CS23 {
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

/// HASH context swap register 24
pub mod CSR24 {

    /// Context swap x Refer to Section 24.7.7: HASH context swap registers introduction.
    pub mod CS24 {
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

/// HASH context swap register 25
pub mod CSR25 {

    /// Context swap x Refer to Section 24.7.7: HASH context swap registers introduction.
    pub mod CS25 {
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

/// HASH context swap register 26
pub mod CSR26 {

    /// Context swap x Refer to Section 24.7.7: HASH context swap registers introduction.
    pub mod CS26 {
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

/// HASH context swap register 27
pub mod CSR27 {

    /// Context swap x Refer to Section 24.7.7: HASH context swap registers introduction.
    pub mod CS27 {
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

/// HASH context swap register 28
pub mod CSR28 {

    /// Context swap x Refer to Section 24.7.7: HASH context swap registers introduction.
    pub mod CS28 {
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

/// HASH context swap register 29
pub mod CSR29 {

    /// Context swap x Refer to Section 24.7.7: HASH context swap registers introduction.
    pub mod CS29 {
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

/// HASH context swap register 30
pub mod CSR30 {

    /// Context swap x Refer to Section 24.7.7: HASH context swap registers introduction.
    pub mod CS30 {
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

/// HASH context swap register 31
pub mod CSR31 {

    /// Context swap x Refer to Section 24.7.7: HASH context swap registers introduction.
    pub mod CS31 {
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

/// HASH context swap register 32
pub mod CSR32 {

    /// Context swap x Refer to Section 24.7.7: HASH context swap registers introduction.
    pub mod CS32 {
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

/// HASH context swap register 33
pub mod CSR33 {

    /// Context swap x Refer to Section 24.7.7: HASH context swap registers introduction.
    pub mod CS33 {
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

/// HASH context swap register 34
pub mod CSR34 {

    /// Context swap x Refer to Section 24.7.7: HASH context swap registers introduction.
    pub mod CS34 {
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

/// HASH context swap register 35
pub mod CSR35 {

    /// Context swap x Refer to Section 24.7.7: HASH context swap registers introduction.
    pub mod CS35 {
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

/// HASH context swap register 36
pub mod CSR36 {

    /// Context swap x Refer to Section 24.7.7: HASH context swap registers introduction.
    pub mod CS36 {
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

/// HASH context swap register 37
pub mod CSR37 {

    /// Context swap x Refer to Section 24.7.7: HASH context swap registers introduction.
    pub mod CS37 {
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

/// HASH context swap register 38
pub mod CSR38 {

    /// Context swap x Refer to Section 24.7.7: HASH context swap registers introduction.
    pub mod CS38 {
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

/// HASH context swap register 39
pub mod CSR39 {

    /// Context swap x Refer to Section 24.7.7: HASH context swap registers introduction.
    pub mod CS39 {
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

/// HASH context swap register 40
pub mod CSR40 {

    /// Context swap x Refer to Section 24.7.7: HASH context swap registers introduction.
    pub mod CS40 {
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

/// HASH context swap register 41
pub mod CSR41 {

    /// Context swap x Refer to Section 24.7.7: HASH context swap registers introduction.
    pub mod CS41 {
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

/// HASH context swap register 42
pub mod CSR42 {

    /// Context swap x Refer to Section 24.7.7: HASH context swap registers introduction.
    pub mod CS42 {
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

/// HASH context swap register 43
pub mod CSR43 {

    /// Context swap x Refer to Section 24.7.7: HASH context swap registers introduction.
    pub mod CS43 {
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

/// HASH context swap register 44
pub mod CSR44 {

    /// Context swap x Refer to Section 24.7.7: HASH context swap registers introduction.
    pub mod CS44 {
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

/// HASH context swap register 45
pub mod CSR45 {

    /// Context swap x Refer to Section 24.7.7: HASH context swap registers introduction.
    pub mod CS45 {
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

/// HASH context swap register 46
pub mod CSR46 {

    /// Context swap x Refer to Section 24.7.7: HASH context swap registers introduction.
    pub mod CS46 {
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

/// HASH context swap register 47
pub mod CSR47 {

    /// Context swap x Refer to Section 24.7.7: HASH context swap registers introduction.
    pub mod CS47 {
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

/// HASH context swap register 48
pub mod CSR48 {

    /// Context swap x Refer to Section 24.7.7: HASH context swap registers introduction.
    pub mod CS48 {
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

/// HASH context swap register 49
pub mod CSR49 {

    /// Context swap x Refer to Section 24.7.7: HASH context swap registers introduction.
    pub mod CS49 {
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

/// HASH context swap register 50
pub mod CSR50 {

    /// Context swap x Refer to Section 24.7.7: HASH context swap registers introduction.
    pub mod CS50 {
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

/// HASH context swap register 51
pub mod CSR51 {

    /// Context swap x Refer to Section 24.7.7: HASH context swap registers introduction.
    pub mod CS51 {
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

/// HASH context swap register 52
pub mod CSR52 {

    /// Context swap x Refer to Section 24.7.7: HASH context swap registers introduction.
    pub mod CS52 {
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

/// HASH context swap register 53
pub mod CSR53 {

    /// Context swap x Refer to Section 24.7.7: HASH context swap registers introduction.
    pub mod CS53 {
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

/// HASH digest register 0
pub mod HR0 {
    pub use super::HRA0::H0;
}

/// HASH digest register 1
pub mod HR1 {
    pub use super::HRA1::H1;
}

/// HASH digest register 2
pub mod HR2 {
    pub use super::HRA2::H2;
}

/// HASH digest register 3
pub mod HR3 {
    pub use super::HRA3::H3;
}

/// HASH digest register 4
pub mod HR4 {
    pub use super::HRA4::H4;
}

/// HASH supplementary digest register 5
pub mod HR5 {

    /// Hash data x Refer to Section 24.7.4: HASH digest registers introduction.
    pub mod H5 {
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

/// HASH supplementary digest register 6
pub mod HR6 {

    /// Hash data x Refer to Section 24.7.4: HASH digest registers introduction.
    pub mod H6 {
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

/// HASH supplementary digest register 7
pub mod HR7 {

    /// Hash data x Refer to Section 24.7.4: HASH digest registers introduction.
    pub mod H7 {
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

    _reserved2: [u8; 320],

    /// HASH digest register 0
    pub HR0: RORegister<u32>,

    /// HASH digest register 1
    pub HR1: RORegister<u32>,

    /// HASH digest register 2
    pub HR2: RORegister<u32>,

    /// HASH digest register 3
    pub HR3: RORegister<u32>,

    /// HASH digest register 4
    pub HR4: RORegister<u32>,

    /// HASH supplementary digest register 5
    pub HR5: RORegister<u32>,

    /// HASH supplementary digest register 6
    pub HR6: RORegister<u32>,

    /// HASH supplementary digest register 7
    pub HR7: RORegister<u32>,
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
    pub HR0: u32,
    pub HR1: u32,
    pub HR2: u32,
    pub HR3: u32,
    pub HR4: u32,
    pub HR5: u32,
    pub HR6: u32,
    pub HR7: u32,
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

/// Access functions for the HASH peripheral instance
pub mod HASH {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x420c0400,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in HASH
    pub const reset: ResetValues = ResetValues {
        CR: 0x00000000,
        DIN: 0x00000000,
        STR: 0x00000000,
        HRA0: 0x00000000,
        HRA1: 0x00000000,
        HRA2: 0x00000000,
        HRA3: 0x00000000,
        HRA4: 0x00000000,
        IMR: 0x00000000,
        SR: 0x00110001,
        CSR0: 0x00000000,
        CSR1: 0x00000000,
        CSR2: 0x00000000,
        CSR3: 0x00000000,
        CSR4: 0x00000000,
        CSR5: 0x00000000,
        CSR6: 0x00000000,
        CSR7: 0x00000000,
        CSR8: 0x00000000,
        CSR9: 0x00000000,
        CSR10: 0x00000000,
        CSR11: 0x00000000,
        CSR12: 0x00000000,
        CSR13: 0x00000000,
        CSR14: 0x00000000,
        CSR15: 0x00000000,
        CSR16: 0x00000000,
        CSR17: 0x00000000,
        CSR18: 0x00000000,
        CSR19: 0x00000000,
        CSR20: 0x00000000,
        CSR21: 0x00000000,
        CSR22: 0x00000000,
        CSR23: 0x00000000,
        CSR24: 0x00000000,
        CSR25: 0x00000000,
        CSR26: 0x00000000,
        CSR27: 0x00000000,
        CSR28: 0x00000000,
        CSR29: 0x00000000,
        CSR30: 0x00000000,
        CSR31: 0x00000000,
        CSR32: 0x00000000,
        CSR33: 0x00000000,
        CSR34: 0x00000000,
        CSR35: 0x00000000,
        CSR36: 0x00000000,
        CSR37: 0x00000000,
        CSR38: 0x00000000,
        CSR39: 0x00000000,
        CSR40: 0x00000000,
        CSR41: 0x00000000,
        CSR42: 0x00000000,
        CSR43: 0x00000000,
        CSR44: 0x00000000,
        CSR45: 0x00000000,
        CSR46: 0x00000000,
        CSR47: 0x00000000,
        CSR48: 0x00000000,
        CSR49: 0x00000000,
        CSR50: 0x00000000,
        CSR51: 0x00000000,
        CSR52: 0x00000000,
        CSR53: 0x00000000,
        HR0: 0x00000000,
        HR1: 0x00000000,
        HR2: 0x00000000,
        HR3: 0x00000000,
        HR4: 0x00000000,
        HR5: 0x00000000,
        HR6: 0x00000000,
        HR7: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut HASH_TAKEN: bool = false;

    /// Safe access to HASH
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
            if HASH_TAKEN {
                None
            } else {
                HASH_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to HASH
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if HASH_TAKEN && inst.addr == INSTANCE.addr {
                HASH_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal HASH
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        HASH_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to HASH
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const HASH: *const RegisterBlock = 0x420c0400 as *const _;
