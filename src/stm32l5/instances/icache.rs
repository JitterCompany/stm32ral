#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! ICache
//!
//! Used by: stm32l552, stm32l562

#[cfg(not(feature = "nosync"))]
pub use crate::stm32l5::peripherals::icache::Instance;
pub use crate::stm32l5::peripherals::icache::{RegisterBlock, ResetValues};
pub use crate::stm32l5::peripherals::icache::{
    CR, CRR0, CRR1, CRR2, CRR3, FCR, HMONR, IER, MMONR, SR,
};

/// Access functions for the ICACHE peripheral instance
pub mod ICACHE {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x40030400,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in ICACHE
    pub const reset: ResetValues = ResetValues {
        CR: 0x00000004,
        SR: 0x00000001,
        IER: 0x00000000,
        FCR: 0x00000000,
        HMONR: 0x00000000,
        MMONR: 0x00000000,
        CRR0: 0x00000200,
        CRR1: 0x00000200,
        CRR2: 0x00000200,
        CRR3: 0x00000200,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut ICACHE_TAKEN: bool = false;

    /// Safe access to ICACHE
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
            if ICACHE_TAKEN {
                None
            } else {
                ICACHE_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to ICACHE
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if ICACHE_TAKEN && inst.addr == INSTANCE.addr {
                ICACHE_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal ICACHE
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        ICACHE_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to ICACHE
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const ICACHE: *const RegisterBlock = 0x40030400 as *const _;

/// Access functions for the SEC_ICache peripheral instance
pub mod SEC_ICache {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x50030400,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in SEC_ICache
    pub const reset: ResetValues = ResetValues {
        CR: 0x00000004,
        SR: 0x00000001,
        IER: 0x00000000,
        FCR: 0x00000000,
        HMONR: 0x00000000,
        MMONR: 0x00000000,
        CRR0: 0x00000200,
        CRR1: 0x00000200,
        CRR2: 0x00000200,
        CRR3: 0x00000200,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut SEC_ICache_TAKEN: bool = false;

    /// Safe access to SEC_ICache
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
            if SEC_ICache_TAKEN {
                None
            } else {
                SEC_ICache_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to SEC_ICache
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if SEC_ICache_TAKEN && inst.addr == INSTANCE.addr {
                SEC_ICache_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal SEC_ICache
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        SEC_ICache_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to SEC_ICache
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const SEC_ICache: *const RegisterBlock = 0x50030400 as *const _;
