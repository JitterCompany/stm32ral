#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Data cache
//!
//! Used by: stm32h562, stm32h563, stm32h573

#[cfg(not(feature = "nosync"))]
pub use crate::stm32h5::peripherals::dcache::Instance;
pub use crate::stm32h5::peripherals::dcache::{RegisterBlock, ResetValues};
pub use crate::stm32h5::peripherals::dcache::{
    CMDREADDRR, CMDRSADDRR, CR, FCR, IER, RHMONR, RMMONR, SR, WHMONR, WMMONR,
};

/// Access functions for the DCACHE peripheral instance
pub mod DCACHE {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x40031400,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in DCACHE
    pub const reset: ResetValues = ResetValues {
        CR: 0x00000000,
        SR: 0x00000001,
        IER: 0x00000000,
        FCR: 0x00000000,
        RHMONR: 0x00000000,
        RMMONR: 0x00000000,
        WHMONR: 0x00000000,
        WMMONR: 0x00000000,
        CMDRSADDRR: 0x00000000,
        CMDREADDRR: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut DCACHE_TAKEN: bool = false;

    /// Safe access to DCACHE
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
            if DCACHE_TAKEN {
                None
            } else {
                DCACHE_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to DCACHE
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if DCACHE_TAKEN && inst.addr == INSTANCE.addr {
                DCACHE_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal DCACHE
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        DCACHE_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to DCACHE
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const DCACHE: *const RegisterBlock = 0x40031400 as *const _;

/// Access functions for the SEC_DCACHE peripheral instance
pub mod SEC_DCACHE {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x50031400,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in SEC_DCACHE
    pub const reset: ResetValues = ResetValues {
        CR: 0x00000000,
        SR: 0x00000001,
        IER: 0x00000000,
        FCR: 0x00000000,
        RHMONR: 0x00000000,
        RMMONR: 0x00000000,
        WHMONR: 0x00000000,
        WMMONR: 0x00000000,
        CMDRSADDRR: 0x00000000,
        CMDREADDRR: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut SEC_DCACHE_TAKEN: bool = false;

    /// Safe access to SEC_DCACHE
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
            if SEC_DCACHE_TAKEN {
                None
            } else {
                SEC_DCACHE_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to SEC_DCACHE
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if SEC_DCACHE_TAKEN && inst.addr == INSTANCE.addr {
                SEC_DCACHE_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal SEC_DCACHE
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        SEC_DCACHE_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to SEC_DCACHE
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const SEC_DCACHE: *const RegisterBlock = 0x50031400 as *const _;
