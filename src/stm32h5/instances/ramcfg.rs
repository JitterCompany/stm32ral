#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! RAMs configuration controller
//!
//! Used by: stm32h562, stm32h563, stm32h573

#[cfg(not(feature = "nosync"))]
pub use crate::stm32h5::peripherals::ramcfg::Instance;
pub use crate::stm32h5::peripherals::ramcfg::{RegisterBlock, ResetValues};
pub use crate::stm32h5::peripherals::ramcfg::{
    M1CR, M1ERKEYR, M1ISR, M2CR, M2DEAR, M2ECCKEYR, M2ERKEYR, M2ICR, M2IER, M2ISR, M2SEAR, M2WPR1,
    M2WPR2, M3CR, M3DEAR, M3ECCKEYR, M3ERKEYR, M3ICR, M3IER, M3ISR, M3SEAR, M4ERKEYR, M5CR, M5DEAR,
    M5ECCKEYR, M5ERKEYR, M5ICR, M5IER, M5ISR, M5SEAR,
};

/// Access functions for the RAMCFG peripheral instance
pub mod RAMCFG {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x40026000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in RAMCFG
    pub const reset: ResetValues = ResetValues {
        M1CR: 0x00000000,
        M1ISR: 0x00000000,
        M1ERKEYR: 0x00000000,
        M2CR: 0x00000000,
        M2IER: 0x00000000,
        M2ISR: 0x00000000,
        M2SEAR: 0x00000000,
        M2DEAR: 0x00000000,
        M2ICR: 0x00000000,
        M2WPR1: 0x00000000,
        M2WPR2: 0x00000000,
        M2ECCKEYR: 0x00000000,
        M2ERKEYR: 0x00000000,
        M3CR: 0x00000000,
        M3IER: 0x00000000,
        M3ISR: 0x00000000,
        M3SEAR: 0x00000000,
        M3DEAR: 0x00000000,
        M3ICR: 0x00000000,
        M3ECCKEYR: 0x00000000,
        M3ERKEYR: 0x00000000,
        M4ERKEYR: 0x00000000,
        M5CR: 0x00000000,
        M5IER: 0x00000000,
        M5ISR: 0x00000000,
        M5SEAR: 0x00000000,
        M5DEAR: 0x00000000,
        M5ICR: 0x00000000,
        M5ECCKEYR: 0x00000000,
        M5ERKEYR: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut RAMCFG_TAKEN: bool = false;

    /// Safe access to RAMCFG
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
            if RAMCFG_TAKEN {
                None
            } else {
                RAMCFG_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to RAMCFG
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if RAMCFG_TAKEN && inst.addr == INSTANCE.addr {
                RAMCFG_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal RAMCFG
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        RAMCFG_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to RAMCFG
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const RAMCFG: *const RegisterBlock = 0x40026000 as *const _;

/// Access functions for the SEC_RAMCFG peripheral instance
pub mod SEC_RAMCFG {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x50026000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in SEC_RAMCFG
    pub const reset: ResetValues = ResetValues {
        M1CR: 0x00000000,
        M1ISR: 0x00000000,
        M1ERKEYR: 0x00000000,
        M2CR: 0x00000000,
        M2IER: 0x00000000,
        M2ISR: 0x00000000,
        M2SEAR: 0x00000000,
        M2DEAR: 0x00000000,
        M2ICR: 0x00000000,
        M2WPR1: 0x00000000,
        M2WPR2: 0x00000000,
        M2ECCKEYR: 0x00000000,
        M2ERKEYR: 0x00000000,
        M3CR: 0x00000000,
        M3IER: 0x00000000,
        M3ISR: 0x00000000,
        M3SEAR: 0x00000000,
        M3DEAR: 0x00000000,
        M3ICR: 0x00000000,
        M3ECCKEYR: 0x00000000,
        M3ERKEYR: 0x00000000,
        M4ERKEYR: 0x00000000,
        M5CR: 0x00000000,
        M5IER: 0x00000000,
        M5ISR: 0x00000000,
        M5SEAR: 0x00000000,
        M5DEAR: 0x00000000,
        M5ICR: 0x00000000,
        M5ECCKEYR: 0x00000000,
        M5ERKEYR: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut SEC_RAMCFG_TAKEN: bool = false;

    /// Safe access to SEC_RAMCFG
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
            if SEC_RAMCFG_TAKEN {
                None
            } else {
                SEC_RAMCFG_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to SEC_RAMCFG
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if SEC_RAMCFG_TAKEN && inst.addr == INSTANCE.addr {
                SEC_RAMCFG_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal SEC_RAMCFG
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        SEC_RAMCFG_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to SEC_RAMCFG
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const SEC_RAMCFG: *const RegisterBlock = 0x50026000 as *const _;
