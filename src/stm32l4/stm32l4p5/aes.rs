#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Advanced encryption standard hardware accelerator

use crate::{RORegister, RWRegister};
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// control register
pub mod CR {

    /// Enable DMA management of data output phase
    pub mod DMAOUTEN {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Disable DMA Output
            pub const Disabled: u32 = 0b0;

            /// 0b1: Enabled DMA Output
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Enable DMA management of data input phase
    pub mod DMAINEN {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Disable DMA Input
            pub const Disabled: u32 = 0b0;

            /// 0b1: Enable DMA Input
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Error interrupt enable
    pub mod ERRIE {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Disable (mask) error interrupt
            pub const Disabled: u32 = 0b0;

            /// 0b1: Enable error interrupt
            pub const Enabled: u32 = 0b1;
        }
    }

    /// CCF flag interrupt enable
    pub mod CCFIE {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Disable (mask) CCF interrupt
            pub const Disabled: u32 = 0b0;

            /// 0b1: Enable CCF interrupt
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Error clear
    pub mod ERRC {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values
        pub mod W {

            /// 0b1: Clear RDERR and WRERR flags
            pub const Clear: u32 = 0b1;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Computation Complete Flag Clear
    pub mod CCFC {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values
        pub mod W {

            /// 0b1: Clear computation complete flag
            pub const Clear: u32 = 0b1;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// AES chaining mode
    pub mod CHMOD {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (2 bits: 0b11 << 5)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Electronic codebook (ECB)
            pub const ECB: u32 = 0b00;

            /// 0b01: Cipher-Block Chaining (CBC)
            pub const CBC: u32 = 0b01;

            /// 0b10: Counter Mode (CTR)
            pub const CTR: u32 = 0b10;
        }
    }

    /// AES operating mode
    pub mod MODE {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (2 bits: 0b11 << 3)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Mode 1: encryption
            pub const Mode1: u32 = 0b00;

            /// 0b01: Mode 2: key derivation (or key preparation for ECB/CBC decryption)
            pub const Mode2: u32 = 0b01;

            /// 0b10: Mode 3: decryption
            pub const Mode3: u32 = 0b10;

            /// 0b11: Mode 4: key derivation then single decryption
            pub const Mode4: u32 = 0b11;
        }
    }

    /// Data type selection (for data in and data out to/from the cryptographic block)
    pub mod DATATYPE {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (2 bits: 0b11 << 1)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Word
            pub const None: u32 = 0b00;

            /// 0b01: Half-word (16-bit)
            pub const HalfWord: u32 = 0b01;

            /// 0b10: Byte (8-bit)
            pub const Byte: u32 = 0b10;

            /// 0b11: Bit
            pub const Bit: u32 = 0b11;
        }
    }

    /// AES enable
    pub mod EN {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Disable AES
            pub const Disabled: u32 = 0b0;

            /// 0b1: Enable AES
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Number of padding bytes in last block
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

    /// Key size selection
    pub mod KEYSIZE {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (1 bit: 1 << 18)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: 128
            pub const AES128: u32 = 0b0;

            /// 0b1: 256
            pub const AES256: u32 = 0b1;
        }
    }

    /// GCM or CCM phase selection
    pub mod GCMPH {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (2 bits: 0b11 << 13)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Init phase
            pub const Init: u32 = 0b00;

            /// 0b01: Header phase
            pub const Header: u32 = 0b01;

            /// 0b10: Payload phase
            pub const Payload: u32 = 0b10;

            /// 0b11: Final Phase
            pub const Final: u32 = 0b11;
        }
    }
}

/// status register
pub mod SR {

    /// Write error flag
    pub mod WRERR {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: Write error not detected
            pub const NoError: u32 = 0b0;

            /// 0b1: Write error detected
            pub const Error: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Read error flag
    pub mod RDERR {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: Read error not detected
            pub const NoError: u32 = 0b0;

            /// 0b1: Read error detected
            pub const Error: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Computation complete flag
    pub mod CCF {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: Computation complete
            pub const Complete: u32 = 0b0;

            /// 0b1: Computation not complete
            pub const NotComplete: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Busy
    pub mod BUSY {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: Idle
            pub const Idle: u32 = 0b0;

            /// 0b1: Busy
            pub const Busy: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// data input register
pub mod DINR {

    /// Data Input Register
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

/// data output register
pub mod DOUTR {

    /// Data output register
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

/// key register 0
pub mod KEYR0 {

    /// Data Output Register (LSB key \[31:0\])
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

/// key register 1
pub mod KEYR1 {
    pub use super::KEYR0::KEY;
}

/// key register 2
pub mod KEYR2 {
    pub use super::KEYR0::KEY;
}

/// key register 3
pub mod KEYR3 {
    pub use super::KEYR0::KEY;
}

/// initialization vector register 0
pub mod IVR0 {

    /// initialization vector register (LSB IVR \[31:0\])
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

/// initialization vector register 1
pub mod IVR1 {
    pub use super::IVR0::IVI;
}

/// initialization vector register 2
pub mod IVR2 {
    pub use super::IVR0::IVI;
}

/// initialization vector register 3
pub mod IVR3 {
    pub use super::IVR0::IVI;
}

/// key register 4
pub mod KEYR4 {

    /// Cryptographic key, bits \[159:128\]
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

/// key register 5
pub mod KEYR5 {
    pub use super::KEYR4::KEY;
}

/// key register 6
pub mod KEYR6 {
    pub use super::KEYR4::KEY;
}

/// key register 7
pub mod KEYR7 {
    pub use super::KEYR4::KEY;
}

/// suspend registers
pub mod SUSP0R {

    /// AES suspend
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

/// suspend registers
pub mod SUSP1R {
    pub use super::SUSP0R::SUSP;
}

/// suspend registers
pub mod SUSP2R {
    pub use super::SUSP0R::SUSP;
}

/// suspend registers
pub mod SUSP3R {
    pub use super::SUSP0R::SUSP;
}

/// suspend registers
pub mod SUSP4R {
    pub use super::SUSP0R::SUSP;
}

/// suspend registers
pub mod SUSP5R {
    pub use super::SUSP0R::SUSP;
}

/// suspend registers
pub mod SUSP6R {
    pub use super::SUSP0R::SUSP;
}

/// suspend registers
pub mod SUSP7R {
    pub use super::SUSP0R::SUSP;
}
#[repr(C)]
pub struct RegisterBlock {
    /// control register
    pub CR: RWRegister<u32>,

    /// status register
    pub SR: RORegister<u32>,

    /// data input register
    pub DINR: RWRegister<u32>,

    /// data output register
    pub DOUTR: RORegister<u32>,

    /// key register 0
    pub KEYR0: RWRegister<u32>,

    /// key register 1
    pub KEYR1: RWRegister<u32>,

    /// key register 2
    pub KEYR2: RWRegister<u32>,

    /// key register 3
    pub KEYR3: RWRegister<u32>,

    /// initialization vector register 0
    pub IVR0: RWRegister<u32>,

    /// initialization vector register 1
    pub IVR1: RWRegister<u32>,

    /// initialization vector register 2
    pub IVR2: RWRegister<u32>,

    /// initialization vector register 3
    pub IVR3: RWRegister<u32>,

    /// key register 4
    pub KEYR4: RWRegister<u32>,

    /// key register 5
    pub KEYR5: RWRegister<u32>,

    /// key register 6
    pub KEYR6: RWRegister<u32>,

    /// key register 7
    pub KEYR7: RWRegister<u32>,

    /// suspend registers
    pub SUSP0R: RWRegister<u32>,

    /// suspend registers
    pub SUSP1R: RWRegister<u32>,

    /// suspend registers
    pub SUSP2R: RWRegister<u32>,

    /// suspend registers
    pub SUSP3R: RWRegister<u32>,

    /// suspend registers
    pub SUSP4R: RWRegister<u32>,

    /// suspend registers
    pub SUSP5R: RWRegister<u32>,

    /// suspend registers
    pub SUSP6R: RWRegister<u32>,

    /// suspend registers
    pub SUSP7R: RWRegister<u32>,
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
        addr: 0x50060000,
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
pub const AES: *const RegisterBlock = 0x50060000 as *const _;
