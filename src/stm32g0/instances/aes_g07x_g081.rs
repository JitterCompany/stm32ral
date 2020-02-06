#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Advanced encryption standard hardware accelerator 1
//!
//! Used by: stm32g07x, stm32g081

#[cfg(not(feature = "nosync"))]
pub use crate::stm32g0::peripherals::aes_v2::Instance;
pub use crate::stm32g0::peripherals::aes_v2::{RegisterBlock, ResetValues};
pub use crate::stm32g0::peripherals::aes_v2::{
    CR, DINR, DOUTR, HWCFR, IPIDR, IVR0, IVR1, IVR2, IVR3, KEYR0, KEYR1, KEYR2, KEYR3, KEYR4,
    KEYR5, KEYR6, KEYR7, SIDR, SR, SUSP0R, SUSP1R, SUSP2R, SUSP3R, SUSP4R, SUSP5R, SUSP6R, SUSP7R,
    VERR,
};

/// Access functions for the AES peripheral instance
pub mod AES {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x40026000,
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
        HWCFR: 0x00000002,
        VERR: 0x00000010,
        IPIDR: 0x00170023,
        SIDR: 0x00170023,
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
pub const AES: *const RegisterBlock = 0x40026000 as *const _;
