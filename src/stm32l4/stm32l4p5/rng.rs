#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Random number generator

use crate::{RORegister, RWRegister};
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// control register
pub mod CR {

    /// Interrupt enable
    pub mod IE {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: RNG interrupt is disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: RNG interrupt is enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Random number generator enable
    pub mod RNGEN {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Random number generator is disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Random number generator is enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// RNG Config lock
    pub mod CONFIGLOCK {
        /// Offset (31 bits)
        pub const offset: u32 = 31;
        /// Mask (1 bit: 1 << 31)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Writes to the RNG_CR configuration bits \[29:4\] are allowed
            pub const Enabled: u32 = 0b0;

            /// 0b1: Writes to the RNG_CR configuration bits \[29:4\] are ignored until the next RNG reset
            pub const Disabled: u32 = 0b1;
        }
    }

    /// Conditioning soft reset
    pub mod CONDRST {
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

    /// RNG configuration 1
    pub mod RNG_CONFIG1 {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (6 bits: 0x3f << 20)
        pub const mask: u32 = 0x3f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b001111: Recommended value for config A (NIST certifiable)
            pub const ConfigA: u32 = 0b001111;

            /// 0b011000: Recommended value for config B (not NIST certifiable)
            pub const ConfigB: u32 = 0b011000;
        }
    }

    /// Clock divider factor
    pub mod CLKDIV {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (4 bits: 0b1111 << 16)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Internal RNG clock after divider is similar to incoming RNG clock
            pub const NoDiv: u32 = 0b0000;

            /// 0b0001: Divide RNG clock by 2^1
            pub const Div_2_1: u32 = 0b0001;

            /// 0b0010: Divide RNG clock by 2^2
            pub const Div_2_2: u32 = 0b0010;

            /// 0b0011: Divide RNG clock by 2^3
            pub const Div_2_3: u32 = 0b0011;

            /// 0b0100: Divide RNG clock by 2^4
            pub const Div_2_4: u32 = 0b0100;

            /// 0b0101: Divide RNG clock by 2^5
            pub const Div_2_5: u32 = 0b0101;

            /// 0b0110: Divide RNG clock by 2^6
            pub const Div_2_6: u32 = 0b0110;

            /// 0b0111: Divide RNG clock by 2^7
            pub const Div_2_7: u32 = 0b0111;

            /// 0b1000: Divide RNG clock by 2^8
            pub const Div_2_8: u32 = 0b1000;

            /// 0b1001: Divide RNG clock by 2^9
            pub const Div_2_9: u32 = 0b1001;

            /// 0b1010: Divide RNG clock by 2^10
            pub const Div_2_10: u32 = 0b1010;

            /// 0b1011: Divide RNG clock by 2^11
            pub const Div_2_11: u32 = 0b1011;

            /// 0b1100: Divide RNG clock by 2^12
            pub const Div_2_12: u32 = 0b1100;

            /// 0b1101: Divide RNG clock by 2^13
            pub const Div_2_13: u32 = 0b1101;

            /// 0b1110: Divide RNG clock by 2^14
            pub const Div_2_14: u32 = 0b1110;

            /// 0b1111: Divide RNG clock by 2^15
            pub const Div_2_15: u32 = 0b1111;
        }
    }

    /// RNG configuration 2
    pub mod RNG_CONFIG2 {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (3 bits: 0b111 << 13)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: Recommended value for config A and B
            pub const ConfigA_B: u32 = 0b000;
        }
    }

    /// Non NIST compliant
    pub mod NISTC {
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

            /// 0b0: Hardware default values for NIST compliant RNG. In this configuration per 128-bit output two conditioning loops are performed and 256 bits of noise source are used
            pub const Default: u32 = 0b0;

            /// 0b1: Custom values for NIST compliant RNG
            pub const Custom: u32 = 0b1;
        }
    }

    /// RNG configuration 3
    pub mod RNG_CONFIG3 {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (4 bits: 0b1111 << 8)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Recommended value for config B (not NIST certifiable)
            pub const ConfigB: u32 = 0b0000;

            /// 0b1101: Recommended value for config A (NIST certifiable)
            pub const ConfigA: u32 = 0b1101;
        }
    }

    /// Clock error detection
    pub mod CED {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Clock error detection is enabled
            pub const Enabled: u32 = 0b0;

            /// 0b1: Clock error detection is disabled
            pub const Disabled: u32 = 0b1;
        }
    }
}

/// status register
pub mod SR {

    /// Seed error interrupt status
    pub mod SEIS {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: No faulty sequence detected
            pub const NoFault: u32 = 0b0;

            /// 0b1: At least one faulty sequence has been detected
            pub const Fault: u32 = 0b1;
        }
    }

    /// Clock error interrupt status
    pub mod CEIS {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: The RNG clock is correct (fRNGCLK> fHCLK/32)
            pub const Correct: u32 = 0b0;

            /// 0b1: The RNG clock before internal divider has been detected too slow (fRNGCLK< fHCLK/32)
            pub const Slow: u32 = 0b1;
        }
    }

    /// Seed error current status
    pub mod SECS {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: No faulty sequence has currently been detected. If the SEIS bit is set, this means that a faulty sequence was detected and the situation has been recovered
            pub const NoFault: u32 = 0b0;

            /// 0b1: At least one faulty sequence has been detected - see ref manual for details
            pub const Fault: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Clock error current status
    pub mod CECS {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: The RNG clock is correct (fRNGCLK> fHCLK/32)
            pub const Correct: u32 = 0b0;

            /// 0b1: The RNG clock before internal divider has been detected too slow (fRNGCLK< fHCLK/32)
            pub const Slow: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Data ready
    pub mod DRDY {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: The RNG_DR register is not yet valid, no random data is available
            pub const Invalid: u32 = 0b0;

            /// 0b1: The RNG_DR register contains valid random data
            pub const Valid: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// data register
pub mod DR {

    /// Random data
    pub mod RNDATA {
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

/// health test control register
pub mod HTCR {

    /// health test configuration
    pub mod HTCFG {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00000000000000001010101001110100: Recommended value for RNG certification (0x0000_AA74)
            pub const Recommended: u32 = 0b00000000000000001010101001110100;

            /// 0b00010111010110010000101010111100: Magic number to be written before any write (0x1759_0ABC)
            pub const Magic: u32 = 0b00010111010110010000101010111100;
        }
    }
}
#[repr(C)]
pub struct RegisterBlock {
    /// control register
    pub CR: RWRegister<u32>,

    /// status register
    pub SR: RWRegister<u32>,

    /// data register
    pub DR: RORegister<u32>,

    _reserved1: [u8; 4],

    /// health test control register
    pub HTCR: RWRegister<u32>,
}
pub struct ResetValues {
    pub CR: u32,
    pub SR: u32,
    pub DR: u32,
    pub HTCR: u32,
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

/// Access functions for the RNG peripheral instance
pub mod RNG {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x50060800,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in RNG
    pub const reset: ResetValues = ResetValues {
        CR: 0x00000000,
        SR: 0x00000000,
        DR: 0x00000000,
        HTCR: 0x00005A4E,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut RNG_TAKEN: bool = false;

    /// Safe access to RNG
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
            if RNG_TAKEN {
                None
            } else {
                RNG_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to RNG
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if RNG_TAKEN && inst.addr == INSTANCE.addr {
                RNG_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal RNG
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        RNG_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to RNG
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const RNG: *const RegisterBlock = 0x50060800 as *const _;
