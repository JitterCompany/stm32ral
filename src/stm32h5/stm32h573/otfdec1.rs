#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! On-The-Fly Decryption engine

use crate::{RORegister, RWRegister, WORegister};
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// OTFDEC control register
pub mod CR {

    /// Encryption mode bit When this bit is set, OTFDEC is used in encryption mode, during which application can write clear text data then read back encrypted data. When this bit is cleared (default), OTFDEC is used in decryption mode, during which application only read back decrypted data. For both modes, cryptographic context (keys, nonces, firmware versions) must be properly initialized. When this bit is set, only data accesses are allowed (zeros are returned otherwise, and XONEIF is set). When MODE = 11, enhanced encryption mode is automatically selected. Note: When ENC bit is set, no access to OCTOSPI must be done (registers and Memory‑mapped region).
    pub mod ENC {
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

/// OTFDEC_PRIVCFGR
pub mod PRIVCFGR {

    /// Privileged access protection. Unprivileged read accesses to registers return zeros Unprivileged write accesses to registers are ignored. Note: This bit can only be written in privileged mode. There is no limitations on reads.
    pub mod PRIV {
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

/// OTFDEC region 1 configuration register
pub mod R1CFGR {

    /// region on-the-fly decryption enable Note: Garbage is decrypted if region context (version, key, nonce) is not valid when this bit is set.
    pub mod REG_EN {
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

    /// region config lock Note: This bit is set once. If this bit is set, it can only be reset to 0 if OTFDEC is reset. Setting this bit forces KEYLOCK bit to 1.
    pub mod CONFIGLOCK {
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

    /// region key lock Note: This bit is set once: if this bit is set, it can only be reset to 0 if the OTFDEC is reset.
    pub mod KEYLOCK {
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

    /// operating mode This bitfield selects the OTFDEC operating mode for this region: Others: Reserved When MODE ≠ 11, the standard AES encryption mode is activated. When either of the MODE bits are changed, the region key and associated CRC are zeroed.
    pub mod MODE {
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

    /// region key 8-bit CRC When KEYLOCK = 0, KEYCRC bitfield is automatically computed by hardware while loading the key of this region in this exact sequence: KEYR0 then KEYR1 then KEYR2 then finally KEYR3 (all written once). A new computation starts as soon as a new valid sequence is initiated, and KEYCRC is read as zero until a valid sequence is completed. When KEYLOCK = 1, KEYCRC remains unchanged until the next reset. CRC computation is an 8-bit checksum using the standard CRC-8-CCITT algorithm X8 + X2 + X + 1 (according the convention). Source code is available in . This field is read only. Note: CRC information is updated only after the last bit of the key has been written.
    pub mod KEYCRC {
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

    /// region firmware version This 16-bit bitfield must be correctly initialized before the region corresponding REG_EN bit is set in OTFDEC_RxCFGR.
    pub mod REGx_VERSION {
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

/// OTFDEC region 1 start address register
pub mod R1STARTADDR {

    /// Region AHB start address This register must be written before the region corresponding REG_EN bit in the OTFDEC_RxCFGR register is set. Writing to this register is discarded if performed while the region CONFIGLOCK bit in the OTFDEC_RxCFGR register is set. Note: When determining the region the first 12 bits (lsb) and the last 4 bits (msb) are ignored. When this register is accessed in read the 4 msb bits and the 12 lsb bits return zeros .
    pub mod REGx_START_ADDR {
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

/// OTFDEC region 1 end address register
pub mod R1ENDADDR {

    /// Region AHB end address This register must be written before the region corresponding REG_EN bit in the OTFDEC_RxCFGR register is set, and OTFDEC_RxENDADDR must be strictly greater than OTFDEC_RxSTARTADDR to be valid. Writing to this register is discarded if performed while the region CONFIGLOCK bit in OTFDEC_RxCFGR is set. Note: When determining the region the first 12 bits (lsb) and the last 4 bits (msb) are ignored. When this register is accessed in read the 4 msb bits return zeros and the 12 lsb bits return ones.
    pub mod REGx_END_ADDR {
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

/// OTFDEC region 1 nonce register 0
pub mod R1NONCER0 {

    /// Region nonce, bits \[31:0\] This register must be written before the region corresponding REG_EN bit in OTFDEC_RxCFGR is set. Writing is discarded in this register if performed while the region CONFIGLOCK bit in the OTFDEC_RxCFGR is set.
    pub mod REGx_NONCE {
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

/// OTFDEC region 1 nonce register 1
pub mod R1NONCER1 {
    pub use super::R1NONCER0::REGx_NONCE;
}

/// OTFDEC region 1 key register 0
pub mod R1KEYR0 {

    /// Region key, bits \[31:0\] This register must be written before the region corresponding REG_EN bit in OTFDEC_RxCFGR is set. Reading this register returns a zero value. Writing to this register is discarded if performed while the region CONFIGLOCK or KEYLOCK bit is set in the OTFDEC_RxCFGR. Note: When application successfully changes MODE bits in OTFDEC_RxCFGR and OTFDEC_RxKEYR, and associated KEYCRC are erased.
    pub mod REGx_KEY {
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

/// OTFDEC region 1 key register 1
pub mod R1KEYR1 {
    pub use super::R1KEYR0::REGx_KEY;
}

/// OTFDEC region 1 key register 2
pub mod R1KEYR2 {
    pub use super::R1KEYR0::REGx_KEY;
}

/// OTFDEC region 1 key register 3
pub mod R1KEYR3 {
    pub use super::R1KEYR0::REGx_KEY;
}

/// OTFDEC region 2 configuration register
pub mod R2CFGR {
    pub use super::R1CFGR::REGx_VERSION;
    pub use super::R1CFGR::CONFIGLOCK;
    pub use super::R1CFGR::KEYCRC;
    pub use super::R1CFGR::KEYLOCK;
    pub use super::R1CFGR::MODE;
    pub use super::R1CFGR::REG_EN;
}

/// OTFDEC region 2 start address register
pub mod R2STARTADDR {
    pub use super::R1STARTADDR::REGx_START_ADDR;
}

/// OTFDEC region 2 end address register
pub mod R2ENDADDR {
    pub use super::R1ENDADDR::REGx_END_ADDR;
}

/// OTFDEC region 2 nonce register 0
pub mod R2NONCER0 {
    pub use super::R1NONCER0::REGx_NONCE;
}

/// OTFDEC region 2 nonce register 1
pub mod R2NONCER1 {
    pub use super::R1NONCER0::REGx_NONCE;
}

/// OTFDEC region 2 key register 0
pub mod R2KEYR0 {
    pub use super::R1KEYR0::REGx_KEY;
}

/// OTFDEC region 2 key register 1
pub mod R2KEYR1 {
    pub use super::R1KEYR0::REGx_KEY;
}

/// OTFDEC region 2 key register 2
pub mod R2KEYR2 {
    pub use super::R1KEYR0::REGx_KEY;
}

/// OTFDEC region 2 key register 3
pub mod R2KEYR3 {
    pub use super::R1KEYR0::REGx_KEY;
}

/// OTFDEC region 3 configuration register
pub mod R3CFGR {
    pub use super::R1CFGR::REGx_VERSION;
    pub use super::R1CFGR::CONFIGLOCK;
    pub use super::R1CFGR::KEYCRC;
    pub use super::R1CFGR::KEYLOCK;
    pub use super::R1CFGR::MODE;
    pub use super::R1CFGR::REG_EN;
}

/// OTFDEC region 3 start address register
pub mod R3STARTADDR {
    pub use super::R1STARTADDR::REGx_START_ADDR;
}

/// OTFDEC region 3 end address register
pub mod R3ENDADDR {
    pub use super::R1ENDADDR::REGx_END_ADDR;
}

/// OTFDEC region 3 nonce register 0
pub mod R3NONCER0 {
    pub use super::R1NONCER0::REGx_NONCE;
}

/// OTFDEC region 3 nonce register 1
pub mod R3NONCER1 {
    pub use super::R1NONCER0::REGx_NONCE;
}

/// OTFDEC region 3 key register 0
pub mod R3KEYR0 {
    pub use super::R1KEYR0::REGx_KEY;
}

/// OTFDEC region 3 key register 1
pub mod R3KEYR1 {
    pub use super::R1KEYR0::REGx_KEY;
}

/// OTFDEC region 3 key register 2
pub mod R3KEYR2 {
    pub use super::R1KEYR0::REGx_KEY;
}

/// OTFDEC region 3 key register 3
pub mod R3KEYR3 {
    pub use super::R1KEYR0::REGx_KEY;
}

/// OTFDEC region 4 configuration register
pub mod R4CFGR {
    pub use super::R1CFGR::REGx_VERSION;
    pub use super::R1CFGR::CONFIGLOCK;
    pub use super::R1CFGR::KEYCRC;
    pub use super::R1CFGR::KEYLOCK;
    pub use super::R1CFGR::MODE;
    pub use super::R1CFGR::REG_EN;
}

/// OTFDEC region 4 start address register
pub mod R4STARTADDR {
    pub use super::R1STARTADDR::REGx_START_ADDR;
}

/// OTFDEC region 4 end address register
pub mod R4ENDADDR {
    pub use super::R1ENDADDR::REGx_END_ADDR;
}

/// OTFDEC region 4 nonce register 0
pub mod R4NONCER0 {
    pub use super::R1NONCER0::REGx_NONCE;
}

/// OTFDEC region 4 nonce register 1
pub mod R4NONCER1 {
    pub use super::R1NONCER0::REGx_NONCE;
}

/// OTFDEC region 4 key register 0
pub mod R4KEYR0 {
    pub use super::R1KEYR0::REGx_KEY;
}

/// OTFDEC region 4 key register 1
pub mod R4KEYR1 {
    pub use super::R1KEYR0::REGx_KEY;
}

/// OTFDEC region 4 key register 2
pub mod R4KEYR2 {
    pub use super::R1KEYR0::REGx_KEY;
}

/// OTFDEC region 4 key register 3
pub mod R4KEYR3 {
    pub use super::R1KEYR0::REGx_KEY;
}

/// OTFDEC interrupt status register
pub mod ISR {

    /// Security error interrupt flag status This bit is set by hardware and read only by application. This bit is set when at least one security error has been detected. This bit is cleared when application sets in OTFDEC_ICR the corresponding bit to 1.
    pub mod SEIF {
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

    /// Execute-only execute-never error interrupt flag status This bit is set by hardware and read only by application. This bit is set when a read access and not an instruction fetch is detected on any encrypted region with MODE bits set to 11. Lastly, XONEIF is also set when an execute access is detected while encryption mode is enabled. This bit is cleared when application sets in OTFDEC_ICR the corresponding bit to 1.
    pub mod XONEIF {
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

    /// Key error interrupt flag status This bit is set by hardware and read only by application. The bit is set when a read access occurs on an encrypted region, while its key registers is null or not properly initialized (KEYCRC = 0x0). This bit is cleared when the application sets in OTFDEC_ICR the corresponding bit to 1. After KEIF is set any subsequent read to the region with bad key registers returns a zeroed value. This state remains until those key registers are properly initialized (KEYCRC not zero).
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

/// OTFDEC interrupt clear register
pub mod ICR {

    /// Security error interrupt flag clear This bit is written by application, and always read as 0.
    pub mod SEIF {
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

    /// Execute-only execute-never error interrupt flag clear This bit is written by application, and always read as 0.
    pub mod XONEIF {
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

    /// Key error interrupt flag clear This bit is written by application, and always read as 0. Note: Clearing KEIF does not solve the source of the problem (bad key registers). To be able to access again any encrypted region, OTFDEC key registers must be properly initialized again.
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

/// OTFDEC interrupt enable register
pub mod IER {

    /// Security error interrupt enable This bit is read and written by application. It controls the OTFDEC interrupt generation when SEIF flag status is set.
    pub mod SEIE {
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

    /// Execute-only execute-never error interrupt enable This bit is read and written by application. It controls the OTFDEC interrupt generation when XONEIF flag status is set.
    pub mod XONEIE {
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

    /// Key error interrupt enable This bit is read and written by application. It controls the OTFDEC interrupt generation when KEIF flag status is set.
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
#[repr(C)]
pub struct RegisterBlock {
    /// OTFDEC control register
    pub CR: RWRegister<u32>,

    _reserved1: [u8; 12],

    /// OTFDEC_PRIVCFGR
    pub PRIVCFGR: RWRegister<u32>,

    _reserved2: [u8; 12],

    /// OTFDEC region 1 configuration register
    pub R1CFGR: RWRegister<u32>,

    /// OTFDEC region 1 start address register
    pub R1STARTADDR: RWRegister<u32>,

    /// OTFDEC region 1 end address register
    pub R1ENDADDR: RWRegister<u32>,

    /// OTFDEC region 1 nonce register 0
    pub R1NONCER0: RWRegister<u32>,

    /// OTFDEC region 1 nonce register 1
    pub R1NONCER1: RWRegister<u32>,

    /// OTFDEC region 1 key register 0
    pub R1KEYR0: WORegister<u32>,

    /// OTFDEC region 1 key register 1
    pub R1KEYR1: WORegister<u32>,

    /// OTFDEC region 1 key register 2
    pub R1KEYR2: WORegister<u32>,

    /// OTFDEC region 1 key register 3
    pub R1KEYR3: WORegister<u32>,

    _reserved3: [u8; 12],

    /// OTFDEC region 2 configuration register
    pub R2CFGR: RWRegister<u32>,

    /// OTFDEC region 2 start address register
    pub R2STARTADDR: RWRegister<u32>,

    /// OTFDEC region 2 end address register
    pub R2ENDADDR: RWRegister<u32>,

    /// OTFDEC region 2 nonce register 0
    pub R2NONCER0: RWRegister<u32>,

    /// OTFDEC region 2 nonce register 1
    pub R2NONCER1: RWRegister<u32>,

    /// OTFDEC region 2 key register 0
    pub R2KEYR0: WORegister<u32>,

    /// OTFDEC region 2 key register 1
    pub R2KEYR1: WORegister<u32>,

    /// OTFDEC region 2 key register 2
    pub R2KEYR2: WORegister<u32>,

    /// OTFDEC region 2 key register 3
    pub R2KEYR3: WORegister<u32>,

    _reserved4: [u8; 12],

    /// OTFDEC region 3 configuration register
    pub R3CFGR: RWRegister<u32>,

    /// OTFDEC region 3 start address register
    pub R3STARTADDR: RWRegister<u32>,

    /// OTFDEC region 3 end address register
    pub R3ENDADDR: RWRegister<u32>,

    /// OTFDEC region 3 nonce register 0
    pub R3NONCER0: RWRegister<u32>,

    /// OTFDEC region 3 nonce register 1
    pub R3NONCER1: RWRegister<u32>,

    /// OTFDEC region 3 key register 0
    pub R3KEYR0: WORegister<u32>,

    /// OTFDEC region 3 key register 1
    pub R3KEYR1: WORegister<u32>,

    /// OTFDEC region 3 key register 2
    pub R3KEYR2: WORegister<u32>,

    /// OTFDEC region 3 key register 3
    pub R3KEYR3: WORegister<u32>,

    _reserved5: [u8; 12],

    /// OTFDEC region 4 configuration register
    pub R4CFGR: RWRegister<u32>,

    /// OTFDEC region 4 start address register
    pub R4STARTADDR: RWRegister<u32>,

    /// OTFDEC region 4 end address register
    pub R4ENDADDR: RWRegister<u32>,

    /// OTFDEC region 4 nonce register 0
    pub R4NONCER0: RWRegister<u32>,

    /// OTFDEC region 4 nonce register 1
    pub R4NONCER1: RWRegister<u32>,

    /// OTFDEC region 4 key register 0
    pub R4KEYR0: WORegister<u32>,

    /// OTFDEC region 4 key register 1
    pub R4KEYR1: WORegister<u32>,

    /// OTFDEC region 4 key register 2
    pub R4KEYR2: WORegister<u32>,

    /// OTFDEC region 4 key register 3
    pub R4KEYR3: WORegister<u32>,

    _reserved6: [u8; 556],

    /// OTFDEC interrupt status register
    pub ISR: RORegister<u32>,

    /// OTFDEC interrupt clear register
    pub ICR: WORegister<u32>,

    /// OTFDEC interrupt enable register
    pub IER: RWRegister<u32>,
}
pub struct ResetValues {
    pub CR: u32,
    pub PRIVCFGR: u32,
    pub R1CFGR: u32,
    pub R1STARTADDR: u32,
    pub R1ENDADDR: u32,
    pub R1NONCER0: u32,
    pub R1NONCER1: u32,
    pub R1KEYR0: u32,
    pub R1KEYR1: u32,
    pub R1KEYR2: u32,
    pub R1KEYR3: u32,
    pub R2CFGR: u32,
    pub R2STARTADDR: u32,
    pub R2ENDADDR: u32,
    pub R2NONCER0: u32,
    pub R2NONCER1: u32,
    pub R2KEYR0: u32,
    pub R2KEYR1: u32,
    pub R2KEYR2: u32,
    pub R2KEYR3: u32,
    pub R3CFGR: u32,
    pub R3STARTADDR: u32,
    pub R3ENDADDR: u32,
    pub R3NONCER0: u32,
    pub R3NONCER1: u32,
    pub R3KEYR0: u32,
    pub R3KEYR1: u32,
    pub R3KEYR2: u32,
    pub R3KEYR3: u32,
    pub R4CFGR: u32,
    pub R4STARTADDR: u32,
    pub R4ENDADDR: u32,
    pub R4NONCER0: u32,
    pub R4NONCER1: u32,
    pub R4KEYR0: u32,
    pub R4KEYR1: u32,
    pub R4KEYR2: u32,
    pub R4KEYR3: u32,
    pub ISR: u32,
    pub ICR: u32,
    pub IER: u32,
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

/// Access functions for the OTFDEC1 peripheral instance
pub mod OTFDEC1 {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x46005000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in OTFDEC1
    pub const reset: ResetValues = ResetValues {
        CR: 0x00000000,
        PRIVCFGR: 0x00000000,
        R1CFGR: 0x00000000,
        R1STARTADDR: 0x00000000,
        R1ENDADDR: 0x00000FFF,
        R1NONCER0: 0x00000000,
        R1NONCER1: 0x00000000,
        R1KEYR0: 0x00000000,
        R1KEYR1: 0x00000000,
        R1KEYR2: 0x00000000,
        R1KEYR3: 0x00000000,
        R2CFGR: 0x00000000,
        R2STARTADDR: 0x00000000,
        R2ENDADDR: 0x00000FFF,
        R2NONCER0: 0x00000000,
        R2NONCER1: 0x00000000,
        R2KEYR0: 0x00000000,
        R2KEYR1: 0x00000000,
        R2KEYR2: 0x00000000,
        R2KEYR3: 0x00000000,
        R3CFGR: 0x00000000,
        R3STARTADDR: 0x00000000,
        R3ENDADDR: 0x00000FFF,
        R3NONCER0: 0x00000000,
        R3NONCER1: 0x00000000,
        R3KEYR0: 0x00000000,
        R3KEYR1: 0x00000000,
        R3KEYR2: 0x00000000,
        R3KEYR3: 0x00000000,
        R4CFGR: 0x00000000,
        R4STARTADDR: 0x00000000,
        R4ENDADDR: 0x00000FFF,
        R4NONCER0: 0x00000000,
        R4NONCER1: 0x00000000,
        R4KEYR0: 0x00000000,
        R4KEYR1: 0x00000000,
        R4KEYR2: 0x00000000,
        R4KEYR3: 0x00000000,
        ISR: 0x00000000,
        ICR: 0x00000000,
        IER: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut OTFDEC1_TAKEN: bool = false;

    /// Safe access to OTFDEC1
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
            if OTFDEC1_TAKEN {
                None
            } else {
                OTFDEC1_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to OTFDEC1
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if OTFDEC1_TAKEN && inst.addr == INSTANCE.addr {
                OTFDEC1_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal OTFDEC1
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        OTFDEC1_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to OTFDEC1
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const OTFDEC1: *const RegisterBlock = 0x46005000 as *const _;

/// Access functions for the SEC_OTFDEC1 peripheral instance
pub mod SEC_OTFDEC1 {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x56005000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in SEC_OTFDEC1
    pub const reset: ResetValues = ResetValues {
        CR: 0x00000000,
        PRIVCFGR: 0x00000000,
        R1CFGR: 0x00000000,
        R1STARTADDR: 0x00000000,
        R1ENDADDR: 0x00000FFF,
        R1NONCER0: 0x00000000,
        R1NONCER1: 0x00000000,
        R1KEYR0: 0x00000000,
        R1KEYR1: 0x00000000,
        R1KEYR2: 0x00000000,
        R1KEYR3: 0x00000000,
        R2CFGR: 0x00000000,
        R2STARTADDR: 0x00000000,
        R2ENDADDR: 0x00000FFF,
        R2NONCER0: 0x00000000,
        R2NONCER1: 0x00000000,
        R2KEYR0: 0x00000000,
        R2KEYR1: 0x00000000,
        R2KEYR2: 0x00000000,
        R2KEYR3: 0x00000000,
        R3CFGR: 0x00000000,
        R3STARTADDR: 0x00000000,
        R3ENDADDR: 0x00000FFF,
        R3NONCER0: 0x00000000,
        R3NONCER1: 0x00000000,
        R3KEYR0: 0x00000000,
        R3KEYR1: 0x00000000,
        R3KEYR2: 0x00000000,
        R3KEYR3: 0x00000000,
        R4CFGR: 0x00000000,
        R4STARTADDR: 0x00000000,
        R4ENDADDR: 0x00000FFF,
        R4NONCER0: 0x00000000,
        R4NONCER1: 0x00000000,
        R4KEYR0: 0x00000000,
        R4KEYR1: 0x00000000,
        R4KEYR2: 0x00000000,
        R4KEYR3: 0x00000000,
        ISR: 0x00000000,
        ICR: 0x00000000,
        IER: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut SEC_OTFDEC1_TAKEN: bool = false;

    /// Safe access to SEC_OTFDEC1
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
            if SEC_OTFDEC1_TAKEN {
                None
            } else {
                SEC_OTFDEC1_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to SEC_OTFDEC1
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if SEC_OTFDEC1_TAKEN && inst.addr == INSTANCE.addr {
                SEC_OTFDEC1_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal SEC_OTFDEC1
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        SEC_OTFDEC1_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to SEC_OTFDEC1
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const SEC_OTFDEC1: *const RegisterBlock = 0x56005000 as *const _;
