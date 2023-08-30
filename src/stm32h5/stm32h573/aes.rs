#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Advanced encryption standard hardware accelerator

use crate::{RORegister, RWRegister, WORegister};
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// AES control register
pub mod CR {

    /// AES enable This bit enables/disables the AES peripheral: At any moment, clearing then setting the bit re-initializes the AES peripheral. This bit is automatically cleared by hardware upon the completion of the key preparation (Mode 2) and upon the completion of GCM/GMAC/CCM initial phase. The bit cannot be set as long as KEYVALID = 0. Note: With KMOD\[1:0\] other than 00, use the IPRST bit rather than the bit EN.
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

    /// Data type selection This bitfield defines the format of data written in the AES_DINR register or read from the AES_DOUTR register, through selecting the mode of data swapping: For more details, refer to . Attempts to write the bitfield are ignored when the BUSY flag of AES_SR register is set, as well as when the EN bit of the AES_CR register is set before the write access and it is not cleared by that write access.
    pub mod DATATYPE {
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

    /// AES operating mode This bitfield selects the AES operating mode: Attempts to write the bitfield are ignored when the BUSY flag of AES_SR register is set, as well as when the EN bit of the AES_CR register is set before the write access and it is not cleared by that write access.
    pub mod MODE {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (2 bits: 0b11 << 3)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Chaining mode selection This bitfield selects the AES chaining mode: others: Reserved Attempts to write the bitfield are ignored when the BUSY flag of AES_SR register is set, as well as when the EN bit of the AES_CR register is set before the write access and it is not cleared by that write access.
    pub mod CHMOD1 {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (2 bits: 0b11 << 5)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// DMA input enable This bit enables/disables data transferring with DMA, in the input phase: When the bit is set, DMA requests are automatically generated by AES during the input data phase. This feature is only effective when Mode 1 or Mode 3 is selected through the MODE\[1:0\] bitfield. It is not effective for Mode 2 (key derivation).
    pub mod DMAINEN {
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

    /// DMA output enable This bit enables/disables data transferring with DMA, in the output phase: When the bit is set, DMA requests are automatically generated by AES during the output data phase. This feature is only effective when Mode 1 or Mode 3 is selected through the MODE\[1:0\] bitfield. It is not effective for Mode 2 (key derivation).
    pub mod DMAOUTEN {
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

    /// GCM or CCM phase selection This bitfield selects the phase of GCM, GMAC or CCM algorithm: The bitfield has no effect if other than GCM, GMAC or CCM algorithms are selected (through the ALGOMODE bitfield).
    pub mod GCMPH {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (2 bits: 0b11 << 13)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Chaining mode selection This bitfield selects the AES chaining mode: others: Reserved Attempts to write the bitfield are ignored when the BUSY flag of AES_SR register is set, as well as when the EN bit of the AES_CR register is set before the write access and it is not cleared by that write access.
    pub mod CHMOD2 {
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

    /// Key size selection This bitfield defines the length of the key used in the AES cryptographic core, in bits: Attempts to write the bit are ignored when the BUSY flag of AES_SR register is set, as well as when the EN bit of the AES_CR register is set before the write access and it is not cleared by that write access.
    pub mod KEYSIZE {
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

    /// Number of padding bytes in last block The bitfield sets the number of padding bytes in last block of payload: ...
    pub mod NPBLB {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (4 bits: 0b1111 << 20)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Key mode selection The bitfield defines how the AES key can be used by the application: Others: Reserved With normal key selection, the key registers are freely usable, no specific usage or protection applies to AES_DIN and AES_DOUT registers. With selection of shared key from SAES co-processor, the AES peripheral automatically loads its key registers with the data stored in the key registers of the SAES peripheral. The key value is available in key registers when BUSY bit is cleared and KEYVALID is set in the AES_SR register. Key error flag KEIF is set otherwise in the AES_ISR register. The bitfield must be set only when KEYSIZE is correct, and when a shared key decryption sequence has been successfully completed in SAES co-processor. N/AAttempts to write the bitfield are ignored when the BUSY flag of AES_SR register is set, as well as when the EN bit of the AES_CR register is set before the write access and it is not cleared by that write access.
    pub mod KMOD {
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

    /// AES peripheral software reset Setting the bit resets the AES peripheral, putting all registers to their default values, except the IPRST bit itself. Hence, any key-relative data is lost. For this reason, it is recommended to set the bit before handing over the AES to a less secure application. The bit must be low while writing any configuration registers.
    pub mod IPRST {
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

/// AES status register
pub mod SR {

    /// Computation completed flag This bit mirrors the CCF bit of the AES_ISR register.
    pub mod CCF {
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

    /// Read error flag This flag indicates the detection of an unexpected read operation from the AES_DOUTR register (during computation or data input phase): The flag is set by hardware. It is cleared by software upon setting the RWEIF bit of the AES_ICR register. Upon the flag setting, an interrupt is generated if enabled through the RWEIE bit of the AES_ICR register. The flag setting has no impact on the AES operation. Unexpected read returns zero.
    pub mod RDERR {
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

    /// Write error This flag indicates the detection of an unexpected write operation to the AES_DINR register (during computation or data output phase): The flag is set by hardware. It is cleared by software upon setting the RWEIF bit of the AES_ICR register. Upon the flag setting, an interrupt is generated if enabled through the RWEIE bit of the AES_ICR register. The flag setting has no impact on the AES operation. Unexpected write is ignored.
    pub mod WRERR {
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

    /// Busy This flag indicates whether AES is idle or busy during GCM payload encryption phase: When the flag indicates “idle”, the current GCM encryption processing may be suspended to process a higher-priority message. In other chaining modes, or in GCM phases other than payload encryption, the flag must be ignored for the suspend process. The flag is set when transferring a shared key from SAES peripheral.
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

    /// Key Valid flag This bit is set by hardware when the amount of key information defined by KEYSIZE in AES_CR has been loaded in AES_KEYx key registers. In normal mode when KEYSEL equals to zero, the application must write the key registers in the correct sequence, otherwise the KEIF flag of the AES_ISR register is set and KEYVALID stays at zero. When KEYSEL is different from zero the BUSY flag is automatically set by AES. When key is loaded successfully, the BUSY flag is cleared and KEYVALID set. Upon an error, the KEIF flag of the AES_ISR register is set, the BUSY flag cleared and KEYVALID kept at zero. When the KEIF flag is set, the application must clear it through the AES_ICR register, otherwise KEYVALID cannot be set. See the KEIF bit description for more details. For more information on key loading please refer to .
    pub mod KEYVALID {
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
}

/// AES data input register
pub mod DINR {

    /// Input data word A four-fold sequential write to this bitfield during the input phase results in writing a complete 128-bit block of input data to the AES peripheral. From the first to the fourth write, the corresponding data weights are \[127:96\], \[95:64\], \[63:32\], and \[31:0\]. Upon each write, the data from the 32-bit input buffer are handled by the data swap block according to the DATATYPE\[1:0\] bitfield, then written into the AES core 128-bit input buffer. The data signification of the input data block depends on the AES operating mode: - Mode 1 (encryption): plaintext - Mode 2 (key derivation): the bitfield is not used (AES_KEYRx registers used for input) - Mode 3 (decryption): ciphertext The data swap operation is described in page 1149.
    pub mod DIN {
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

/// AES data output register
pub mod DOUTR {

    /// Output data word This read-only bitfield fetches a 32-bit output buffer. A four-fold sequential read of this bitfield, upon the computation completion (CCF set), virtually reads a complete 128-bit block of output data from the AES peripheral. Before reaching the output buffer, the data produced by the AES core are handled by the data swap block according to the DATATYPE\[1:0\] bitfield. Data weights from the first to the fourth read operation are: \[127:96\], \[95:64\], \[63:32\], and \[31:0\]. The data signification of the output data block depends on the AES operating mode: - Mode 1 (encryption): ciphertext - Mode 2 (key derivation): the bitfield is not used - Mode 3 (decryption): plaintext The data swap operation is described in page 1149.
    pub mod DOUT {
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

/// AES key register 0
pub mod KEYR0 {

    /// Cryptographic key, bits \[31:0\] This write-only bitfield contains the bits \[31:0\] of the AES encryption or decryption key, depending on the operating mode: - In Mode 1 (encryption), Mode 2 (key derivation): the value to write into the bitfield is the encryption key. - In Mode 3 (decryption): the value to write into the bitfield is the encryption key to be derived before being used for decryption. The AES_KEYRx registers may be written only when KEYSIZE value is correct and when the AES peripheral is disabled (EN bit of the AES_CR register cleared). A special writing sequence is also required, as described in KEYVALID bit of the AES_SR register. Note that, if KMOD\[1:0\] = 10 (shared key), the key is directly loaded from SAES peripheral to AES_KEYRx registers (hence writes to key register is ignored and KEIF is set). Refer to for more details.
    pub mod KEY {
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

/// AES key register 1
pub mod KEYR1 {
    pub use super::KEYR0::KEY;
}

/// AES key register 2
pub mod KEYR2 {
    pub use super::KEYR0::KEY;
}

/// AES key register 3
pub mod KEYR3 {
    pub use super::KEYR0::KEY;
}

/// AES initialization vector register 0
pub mod IVR0 {

    /// Initialization vector input, bits \[31:0\] Refer to for description of the IVI\[127:0\] bitfield. The initialization vector is only used in chaining modes other than ECB. The AES_IVRx registers may be written only when the AES peripheral is disabled
    pub mod IVI {
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

/// AES initialization vector register 1
pub mod IVR1 {
    pub use super::IVR0::IVI;
}

/// AES initialization vector register 2
pub mod IVR2 {
    pub use super::IVR0::IVI;
}

/// AES initialization vector register 3
pub mod IVR3 {
    pub use super::IVR0::IVI;
}

/// AES key register 4
pub mod KEYR4 {
    pub use super::KEYR0::KEY;
}

/// AES key register 5
pub mod KEYR5 {
    pub use super::KEYR0::KEY;
}

/// AES key register 6
pub mod KEYR6 {
    pub use super::KEYR0::KEY;
}

/// AES key register 7
pub mod KEYR7 {
    pub use super::KEYR0::KEY;
}

/// AES suspend registers
pub mod SUSP0R {

    /// AES suspend Upon suspend operation, this bitfield of the corresponding AES_SUSPxR register takes the value of one of internal AES registers.
    pub mod SUSP {
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

/// AES suspend registers
pub mod SUSP1R {
    pub use super::SUSP0R::SUSP;
}

/// AES suspend registers
pub mod SUSP2R {
    pub use super::SUSP0R::SUSP;
}

/// AES suspend registers
pub mod SUSP3R {
    pub use super::SUSP0R::SUSP;
}

/// AES suspend registers
pub mod SUSP4R {
    pub use super::SUSP0R::SUSP;
}

/// AES suspend registers
pub mod SUSP5R {
    pub use super::SUSP0R::SUSP;
}

/// AES suspend registers
pub mod SUSP6R {
    pub use super::SUSP0R::SUSP;
}

/// AES suspend registers
pub mod SUSP7R {
    pub use super::SUSP0R::SUSP;
}

/// AES interrupt enable register
pub mod IER {

    /// Computation complete flag interrupt enable This bit enables or disables (masks) the AES interrupt generation when CCF (computation complete flag) is set.
    pub mod CCFIE {
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

    /// Read or write error interrupt enable This bit enables or disables (masks) the AES interrupt generation when RWEIF (read and/or write error flag) is set.
    pub mod RWEIE {
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

    /// Key error interrupt enable This bit enables or disables (masks) the AES interrupt generation when KEIF (key error flag) is set.
    pub mod KEIE {
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

/// AES interrupt status register
pub mod ISR {

    /// Computation complete flag This flag indicates whether the computation is completed: The flag is set by hardware upon the completion of the computation. It is cleared by software, upon setting the CCF bit of the AES_ICR register. Upon the flag setting, an interrupt is generated if enabled through the CCFIE bit of the AES_IER register. The flag is significant only when the DMAOUTEN bit is 0. It may stay high when DMA_EN is 1.
    pub mod CCF {
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

    /// Read or write error interrupt flag This read-only bit is set by hardware when a RDERR or a WRERR error flag is set in the AES_SR register. RWEIF bit is cleared when application sets the corresponding bit of AES_ICR register. An interrupt is generated if the RWEIE bit has been previously set in the AES_IER register. This flags has no meaning when key derivation mode is selected.
    pub mod RWEIF {
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

    /// Key error interrupt flag This read-only bit is set by hardware when key information failed to load into key registers. Setting the corresponding bit of the AES_ICR register clears the KEIF and generates interrupt if the KEIE bit of the AES_IER register is set. KEIF is triggered upon any of the following errors: AES_KEYRx register write does not respect the correct order. (For KEYSIZE = 0, AES_KEYR0 then AES_KEYR1 then AES_KEYR2 then AES_KEYR3 register, or reverse. For KEYSIZE = 1, AES_KEYR0 then AES_KEYR1 then AES_KEYR2 then AES_KEYR3 then AES_KEYR4 then AES_KEYR5 then AES_KEYR6 then AES_KEYR7, or reverse). KEIF must be cleared by the application software, otherwise KEYVALID cannot be set.
    pub mod KEIF {
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

/// AES interrupt clear register
pub mod ICR {

    /// Computation complete flag clear Setting this bit clears the CCF status bit of the AES_SR and AES_ISR registers.
    pub mod CCF {
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

    /// Read or write error interrupt flag clear Setting this bit clears the RWEIF status bit of the AES_ISR register, and both RDERR and WRERR flags in the AES_SR register.
    pub mod RWEIF {
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

    /// Key error interrupt flag clear Setting this bit clears the KEIF status bit of the AES_ISR register.
    pub mod KEIF {
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
#[repr(C)]
pub struct RegisterBlock {
    /// AES control register
    pub CR: RWRegister<u32>,

    /// AES status register
    pub SR: RORegister<u32>,

    /// AES data input register
    pub DINR: WORegister<u32>,

    /// AES data output register
    pub DOUTR: RORegister<u32>,

    /// AES key register 0
    pub KEYR0: WORegister<u32>,

    /// AES key register 1
    pub KEYR1: WORegister<u32>,

    /// AES key register 2
    pub KEYR2: WORegister<u32>,

    /// AES key register 3
    pub KEYR3: WORegister<u32>,

    /// AES initialization vector register 0
    pub IVR0: RWRegister<u32>,

    /// AES initialization vector register 1
    pub IVR1: RWRegister<u32>,

    /// AES initialization vector register 2
    pub IVR2: RWRegister<u32>,

    /// AES initialization vector register 3
    pub IVR3: RWRegister<u32>,

    /// AES key register 4
    pub KEYR4: WORegister<u32>,

    /// AES key register 5
    pub KEYR5: WORegister<u32>,

    /// AES key register 6
    pub KEYR6: WORegister<u32>,

    /// AES key register 7
    pub KEYR7: WORegister<u32>,

    /// AES suspend registers
    pub SUSP0R: RWRegister<u32>,

    /// AES suspend registers
    pub SUSP1R: RWRegister<u32>,

    /// AES suspend registers
    pub SUSP2R: RWRegister<u32>,

    /// AES suspend registers
    pub SUSP3R: RWRegister<u32>,

    /// AES suspend registers
    pub SUSP4R: RWRegister<u32>,

    /// AES suspend registers
    pub SUSP5R: RWRegister<u32>,

    /// AES suspend registers
    pub SUSP6R: RWRegister<u32>,

    /// AES suspend registers
    pub SUSP7R: RWRegister<u32>,

    _reserved1: [u8; 672],

    /// AES interrupt enable register
    pub IER: RWRegister<u32>,

    /// AES interrupt status register
    pub ISR: RORegister<u32>,

    /// AES interrupt clear register
    pub ICR: WORegister<u32>,
}
pub struct ResetValues {
    pub CR: u32,
    pub SR: u32,
    pub DINR: u32,
    pub DOUTR: u32,
    pub KEYR0: u32,
    pub KEYR1: u32,
    pub KEYR2: u32,
    pub KEYR3: u32,
    pub IVR0: u32,
    pub IVR1: u32,
    pub IVR2: u32,
    pub IVR3: u32,
    pub KEYR4: u32,
    pub KEYR5: u32,
    pub KEYR6: u32,
    pub KEYR7: u32,
    pub SUSP0R: u32,
    pub SUSP1R: u32,
    pub SUSP2R: u32,
    pub SUSP3R: u32,
    pub SUSP4R: u32,
    pub SUSP5R: u32,
    pub SUSP6R: u32,
    pub SUSP7R: u32,
    pub IER: u32,
    pub ISR: u32,
    pub ICR: u32,
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

/// Access functions for the AES peripheral instance
pub mod AES {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x420c0000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in AES
    pub const reset: ResetValues = ResetValues {
        CR: 0x00000000,
        SR: 0x00000000,
        DINR: 0x00000000,
        DOUTR: 0x00000000,
        KEYR0: 0x00000000,
        KEYR1: 0x00000000,
        KEYR2: 0x00000000,
        KEYR3: 0x00000000,
        IVR0: 0x00000000,
        IVR1: 0x00000000,
        IVR2: 0x00000000,
        IVR3: 0x00000000,
        KEYR4: 0x00000000,
        KEYR5: 0x00000000,
        KEYR6: 0x00000000,
        KEYR7: 0x00000000,
        SUSP0R: 0x00000000,
        SUSP1R: 0x00000000,
        SUSP2R: 0x00000000,
        SUSP3R: 0x00000000,
        SUSP4R: 0x00000000,
        SUSP5R: 0x00000000,
        SUSP6R: 0x00000000,
        SUSP7R: 0x00000000,
        IER: 0x00000000,
        ISR: 0x00000000,
        ICR: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut AES_TAKEN: bool = false;

    /// Safe access to AES
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
            if AES_TAKEN {
                None
            } else {
                AES_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to AES
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if AES_TAKEN && inst.addr == INSTANCE.addr {
                AES_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal AES
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        AES_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to AES
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const AES: *const RegisterBlock = 0x420c0000 as *const _;

/// Access functions for the SEC_AES peripheral instance
pub mod SEC_AES {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x520c0000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in SEC_AES
    pub const reset: ResetValues = ResetValues {
        CR: 0x00000000,
        SR: 0x00000000,
        DINR: 0x00000000,
        DOUTR: 0x00000000,
        KEYR0: 0x00000000,
        KEYR1: 0x00000000,
        KEYR2: 0x00000000,
        KEYR3: 0x00000000,
        IVR0: 0x00000000,
        IVR1: 0x00000000,
        IVR2: 0x00000000,
        IVR3: 0x00000000,
        KEYR4: 0x00000000,
        KEYR5: 0x00000000,
        KEYR6: 0x00000000,
        KEYR7: 0x00000000,
        SUSP0R: 0x00000000,
        SUSP1R: 0x00000000,
        SUSP2R: 0x00000000,
        SUSP3R: 0x00000000,
        SUSP4R: 0x00000000,
        SUSP5R: 0x00000000,
        SUSP6R: 0x00000000,
        SUSP7R: 0x00000000,
        IER: 0x00000000,
        ISR: 0x00000000,
        ICR: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut SEC_AES_TAKEN: bool = false;

    /// Safe access to SEC_AES
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
            if SEC_AES_TAKEN {
                None
            } else {
                SEC_AES_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to SEC_AES
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if SEC_AES_TAKEN && inst.addr == INSTANCE.addr {
                SEC_AES_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal SEC_AES
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        SEC_AES_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to SEC_AES
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const SEC_AES: *const RegisterBlock = 0x520c0000 as *const _;