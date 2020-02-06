#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Operational amplifiers
//!
//! Used by: stm32g473, stm32g474, stm32g483, stm32g484

#[cfg(not(feature = "nosync"))]
pub use crate::stm32g4::peripherals::opamp_v2::Instance;
pub use crate::stm32g4::peripherals::opamp_v2::{RegisterBlock, ResetValues};
pub use crate::stm32g4::peripherals::opamp_v2::{
    OPAMP1_CSR, OPAMP1_TCMR, OPAMP2_CSR, OPAMP2_TCMR, OPAMP3_CSR, OPAMP3_TCMR, OPAMP4_CSR,
    OPAMP4_TCMR, OPAMP5_CSR, OPAMP5_TCMR, OPAMP6_CSR, OPAMP6_TCMR,
};

/// Access functions for the OPAMP peripheral instance
pub mod OPAMP {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x40010300,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in OPAMP
    pub const reset: ResetValues = ResetValues {
        OPAMP1_CSR: 0x00000000,
        OPAMP2_CSR: 0x00000000,
        OPAMP3_CSR: 0x00000000,
        OPAMP4_CSR: 0x00000000,
        OPAMP5_CSR: 0x00000000,
        OPAMP6_CSR: 0x00000000,
        OPAMP1_TCMR: 0x00000000,
        OPAMP2_TCMR: 0x00000000,
        OPAMP3_TCMR: 0x00000000,
        OPAMP4_TCMR: 0x00000000,
        OPAMP5_TCMR: 0x00000000,
        OPAMP6_TCMR: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut OPAMP_TAKEN: bool = false;

    /// Safe access to OPAMP
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
            if OPAMP_TAKEN {
                None
            } else {
                OPAMP_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to OPAMP
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if OPAMP_TAKEN && inst.addr == INSTANCE.addr {
                OPAMP_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal OPAMP
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        OPAMP_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to OPAMP
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const OPAMP: *const RegisterBlock = 0x40010300 as *const _;
