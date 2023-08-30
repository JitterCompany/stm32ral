#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! GTZC1_MPCBB1
//!
//! Used by: stm32h562, stm32h563, stm32h573

#[cfg(not(feature = "nosync"))]
pub use crate::stm32h5::peripherals::gtzc1_mpcbb1::Instance;
pub use crate::stm32h5::peripherals::gtzc1_mpcbb1::{RegisterBlock, ResetValues};
pub use crate::stm32h5::peripherals::gtzc1_mpcbb1::{
    GTZC1_MPCBB1_CFGLOCK1, GTZC1_MPCBB1_CR, GTZC1_MPCBB1_PRIVCFGR0, GTZC1_MPCBB1_PRIVCFGR1,
    GTZC1_MPCBB1_PRIVCFGR10, GTZC1_MPCBB1_PRIVCFGR11, GTZC1_MPCBB1_PRIVCFGR12,
    GTZC1_MPCBB1_PRIVCFGR13, GTZC1_MPCBB1_PRIVCFGR14, GTZC1_MPCBB1_PRIVCFGR15,
    GTZC1_MPCBB1_PRIVCFGR16, GTZC1_MPCBB1_PRIVCFGR17, GTZC1_MPCBB1_PRIVCFGR18,
    GTZC1_MPCBB1_PRIVCFGR19, GTZC1_MPCBB1_PRIVCFGR2, GTZC1_MPCBB1_PRIVCFGR20,
    GTZC1_MPCBB1_PRIVCFGR21, GTZC1_MPCBB1_PRIVCFGR22, GTZC1_MPCBB1_PRIVCFGR23,
    GTZC1_MPCBB1_PRIVCFGR24, GTZC1_MPCBB1_PRIVCFGR25, GTZC1_MPCBB1_PRIVCFGR26,
    GTZC1_MPCBB1_PRIVCFGR27, GTZC1_MPCBB1_PRIVCFGR28, GTZC1_MPCBB1_PRIVCFGR29,
    GTZC1_MPCBB1_PRIVCFGR3, GTZC1_MPCBB1_PRIVCFGR30, GTZC1_MPCBB1_PRIVCFGR31,
    GTZC1_MPCBB1_PRIVCFGR4, GTZC1_MPCBB1_PRIVCFGR5, GTZC1_MPCBB1_PRIVCFGR6, GTZC1_MPCBB1_PRIVCFGR7,
    GTZC1_MPCBB1_PRIVCFGR8, GTZC1_MPCBB1_PRIVCFGR9, GTZC1_MPCBB1_SECCFGR0, GTZC1_MPCBB1_SECCFGR1,
    GTZC1_MPCBB1_SECCFGR10, GTZC1_MPCBB1_SECCFGR11, GTZC1_MPCBB1_SECCFGR12, GTZC1_MPCBB1_SECCFGR13,
    GTZC1_MPCBB1_SECCFGR14, GTZC1_MPCBB1_SECCFGR15, GTZC1_MPCBB1_SECCFGR16, GTZC1_MPCBB1_SECCFGR17,
    GTZC1_MPCBB1_SECCFGR18, GTZC1_MPCBB1_SECCFGR19, GTZC1_MPCBB1_SECCFGR2, GTZC1_MPCBB1_SECCFGR20,
    GTZC1_MPCBB1_SECCFGR21, GTZC1_MPCBB1_SECCFGR22, GTZC1_MPCBB1_SECCFGR23, GTZC1_MPCBB1_SECCFGR24,
    GTZC1_MPCBB1_SECCFGR25, GTZC1_MPCBB1_SECCFGR26, GTZC1_MPCBB1_SECCFGR27, GTZC1_MPCBB1_SECCFGR28,
    GTZC1_MPCBB1_SECCFGR29, GTZC1_MPCBB1_SECCFGR3, GTZC1_MPCBB1_SECCFGR30, GTZC1_MPCBB1_SECCFGR31,
    GTZC1_MPCBB1_SECCFGR4, GTZC1_MPCBB1_SECCFGR5, GTZC1_MPCBB1_SECCFGR6, GTZC1_MPCBB1_SECCFGR7,
    GTZC1_MPCBB1_SECCFGR8, GTZC1_MPCBB1_SECCFGR9,
};

/// Access functions for the GTZC1_MPCBB1 peripheral instance
pub mod GTZC1_MPCBB1 {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x40032c00,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in GTZC1_MPCBB1
    pub const reset: ResetValues = ResetValues {
        GTZC1_MPCBB1_CR: 0x00000000,
        GTZC1_MPCBB1_CFGLOCK1: 0x00000000,
        GTZC1_MPCBB1_SECCFGR0: 0xFFFFFFFF,
        GTZC1_MPCBB1_SECCFGR1: 0xFFFFFFFF,
        GTZC1_MPCBB1_SECCFGR2: 0xFFFFFFFF,
        GTZC1_MPCBB1_SECCFGR3: 0xFFFFFFFF,
        GTZC1_MPCBB1_SECCFGR4: 0xFFFFFFFF,
        GTZC1_MPCBB1_SECCFGR5: 0xFFFFFFFF,
        GTZC1_MPCBB1_SECCFGR6: 0xFFFFFFFF,
        GTZC1_MPCBB1_SECCFGR7: 0xFFFFFFFF,
        GTZC1_MPCBB1_SECCFGR8: 0xFFFFFFFF,
        GTZC1_MPCBB1_SECCFGR9: 0xFFFFFFFF,
        GTZC1_MPCBB1_SECCFGR10: 0xFFFFFFFF,
        GTZC1_MPCBB1_SECCFGR11: 0xFFFFFFFF,
        GTZC1_MPCBB1_SECCFGR12: 0xFFFFFFFF,
        GTZC1_MPCBB1_SECCFGR13: 0xFFFFFFFF,
        GTZC1_MPCBB1_SECCFGR14: 0xFFFFFFFF,
        GTZC1_MPCBB1_SECCFGR15: 0xFFFFFFFF,
        GTZC1_MPCBB1_SECCFGR16: 0xFFFFFFFF,
        GTZC1_MPCBB1_SECCFGR17: 0xFFFFFFFF,
        GTZC1_MPCBB1_SECCFGR18: 0xFFFFFFFF,
        GTZC1_MPCBB1_SECCFGR19: 0xFFFFFFFF,
        GTZC1_MPCBB1_SECCFGR20: 0xFFFFFFFF,
        GTZC1_MPCBB1_SECCFGR21: 0xFFFFFFFF,
        GTZC1_MPCBB1_SECCFGR22: 0xFFFFFFFF,
        GTZC1_MPCBB1_SECCFGR23: 0xFFFFFFFF,
        GTZC1_MPCBB1_SECCFGR24: 0xFFFFFFFF,
        GTZC1_MPCBB1_SECCFGR25: 0xFFFFFFFF,
        GTZC1_MPCBB1_SECCFGR26: 0xFFFFFFFF,
        GTZC1_MPCBB1_SECCFGR27: 0xFFFFFFFF,
        GTZC1_MPCBB1_SECCFGR28: 0xFFFFFFFF,
        GTZC1_MPCBB1_SECCFGR29: 0xFFFFFFFF,
        GTZC1_MPCBB1_SECCFGR30: 0xFFFFFFFF,
        GTZC1_MPCBB1_SECCFGR31: 0xFFFFFFFF,
        GTZC1_MPCBB1_PRIVCFGR0: 0xFFFFFFFF,
        GTZC1_MPCBB1_PRIVCFGR1: 0xFFFFFFFF,
        GTZC1_MPCBB1_PRIVCFGR2: 0xFFFFFFFF,
        GTZC1_MPCBB1_PRIVCFGR3: 0xFFFFFFFF,
        GTZC1_MPCBB1_PRIVCFGR4: 0xFFFFFFFF,
        GTZC1_MPCBB1_PRIVCFGR5: 0xFFFFFFFF,
        GTZC1_MPCBB1_PRIVCFGR6: 0xFFFFFFFF,
        GTZC1_MPCBB1_PRIVCFGR7: 0xFFFFFFFF,
        GTZC1_MPCBB1_PRIVCFGR8: 0xFFFFFFFF,
        GTZC1_MPCBB1_PRIVCFGR9: 0xFFFFFFFF,
        GTZC1_MPCBB1_PRIVCFGR10: 0xFFFFFFFF,
        GTZC1_MPCBB1_PRIVCFGR11: 0xFFFFFFFF,
        GTZC1_MPCBB1_PRIVCFGR12: 0xFFFFFFFF,
        GTZC1_MPCBB1_PRIVCFGR13: 0xFFFFFFFF,
        GTZC1_MPCBB1_PRIVCFGR14: 0xFFFFFFFF,
        GTZC1_MPCBB1_PRIVCFGR15: 0xFFFFFFFF,
        GTZC1_MPCBB1_PRIVCFGR16: 0xFFFFFFFF,
        GTZC1_MPCBB1_PRIVCFGR17: 0xFFFFFFFF,
        GTZC1_MPCBB1_PRIVCFGR18: 0xFFFFFFFF,
        GTZC1_MPCBB1_PRIVCFGR19: 0xFFFFFFFF,
        GTZC1_MPCBB1_PRIVCFGR20: 0xFFFFFFFF,
        GTZC1_MPCBB1_PRIVCFGR21: 0xFFFFFFFF,
        GTZC1_MPCBB1_PRIVCFGR22: 0xFFFFFFFF,
        GTZC1_MPCBB1_PRIVCFGR23: 0xFFFFFFFF,
        GTZC1_MPCBB1_PRIVCFGR24: 0xFFFFFFFF,
        GTZC1_MPCBB1_PRIVCFGR25: 0xFFFFFFFF,
        GTZC1_MPCBB1_PRIVCFGR26: 0xFFFFFFFF,
        GTZC1_MPCBB1_PRIVCFGR27: 0xFFFFFFFF,
        GTZC1_MPCBB1_PRIVCFGR28: 0xFFFFFFFF,
        GTZC1_MPCBB1_PRIVCFGR29: 0xFFFFFFFF,
        GTZC1_MPCBB1_PRIVCFGR30: 0xFFFFFFFF,
        GTZC1_MPCBB1_PRIVCFGR31: 0xFFFFFFFF,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut GTZC1_MPCBB1_TAKEN: bool = false;

    /// Safe access to GTZC1_MPCBB1
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
            if GTZC1_MPCBB1_TAKEN {
                None
            } else {
                GTZC1_MPCBB1_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to GTZC1_MPCBB1
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if GTZC1_MPCBB1_TAKEN && inst.addr == INSTANCE.addr {
                GTZC1_MPCBB1_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal GTZC1_MPCBB1
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        GTZC1_MPCBB1_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to GTZC1_MPCBB1
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const GTZC1_MPCBB1: *const RegisterBlock = 0x40032c00 as *const _;

/// Access functions for the SEC_GTZC1_MPCBB1 peripheral instance
pub mod SEC_GTZC1_MPCBB1 {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x50032c00,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in SEC_GTZC1_MPCBB1
    pub const reset: ResetValues = ResetValues {
        GTZC1_MPCBB1_CR: 0x00000000,
        GTZC1_MPCBB1_CFGLOCK1: 0x00000000,
        GTZC1_MPCBB1_SECCFGR0: 0xFFFFFFFF,
        GTZC1_MPCBB1_SECCFGR1: 0xFFFFFFFF,
        GTZC1_MPCBB1_SECCFGR2: 0xFFFFFFFF,
        GTZC1_MPCBB1_SECCFGR3: 0xFFFFFFFF,
        GTZC1_MPCBB1_SECCFGR4: 0xFFFFFFFF,
        GTZC1_MPCBB1_SECCFGR5: 0xFFFFFFFF,
        GTZC1_MPCBB1_SECCFGR6: 0xFFFFFFFF,
        GTZC1_MPCBB1_SECCFGR7: 0xFFFFFFFF,
        GTZC1_MPCBB1_SECCFGR8: 0xFFFFFFFF,
        GTZC1_MPCBB1_SECCFGR9: 0xFFFFFFFF,
        GTZC1_MPCBB1_SECCFGR10: 0xFFFFFFFF,
        GTZC1_MPCBB1_SECCFGR11: 0xFFFFFFFF,
        GTZC1_MPCBB1_SECCFGR12: 0xFFFFFFFF,
        GTZC1_MPCBB1_SECCFGR13: 0xFFFFFFFF,
        GTZC1_MPCBB1_SECCFGR14: 0xFFFFFFFF,
        GTZC1_MPCBB1_SECCFGR15: 0xFFFFFFFF,
        GTZC1_MPCBB1_SECCFGR16: 0xFFFFFFFF,
        GTZC1_MPCBB1_SECCFGR17: 0xFFFFFFFF,
        GTZC1_MPCBB1_SECCFGR18: 0xFFFFFFFF,
        GTZC1_MPCBB1_SECCFGR19: 0xFFFFFFFF,
        GTZC1_MPCBB1_SECCFGR20: 0xFFFFFFFF,
        GTZC1_MPCBB1_SECCFGR21: 0xFFFFFFFF,
        GTZC1_MPCBB1_SECCFGR22: 0xFFFFFFFF,
        GTZC1_MPCBB1_SECCFGR23: 0xFFFFFFFF,
        GTZC1_MPCBB1_SECCFGR24: 0xFFFFFFFF,
        GTZC1_MPCBB1_SECCFGR25: 0xFFFFFFFF,
        GTZC1_MPCBB1_SECCFGR26: 0xFFFFFFFF,
        GTZC1_MPCBB1_SECCFGR27: 0xFFFFFFFF,
        GTZC1_MPCBB1_SECCFGR28: 0xFFFFFFFF,
        GTZC1_MPCBB1_SECCFGR29: 0xFFFFFFFF,
        GTZC1_MPCBB1_SECCFGR30: 0xFFFFFFFF,
        GTZC1_MPCBB1_SECCFGR31: 0xFFFFFFFF,
        GTZC1_MPCBB1_PRIVCFGR0: 0xFFFFFFFF,
        GTZC1_MPCBB1_PRIVCFGR1: 0xFFFFFFFF,
        GTZC1_MPCBB1_PRIVCFGR2: 0xFFFFFFFF,
        GTZC1_MPCBB1_PRIVCFGR3: 0xFFFFFFFF,
        GTZC1_MPCBB1_PRIVCFGR4: 0xFFFFFFFF,
        GTZC1_MPCBB1_PRIVCFGR5: 0xFFFFFFFF,
        GTZC1_MPCBB1_PRIVCFGR6: 0xFFFFFFFF,
        GTZC1_MPCBB1_PRIVCFGR7: 0xFFFFFFFF,
        GTZC1_MPCBB1_PRIVCFGR8: 0xFFFFFFFF,
        GTZC1_MPCBB1_PRIVCFGR9: 0xFFFFFFFF,
        GTZC1_MPCBB1_PRIVCFGR10: 0xFFFFFFFF,
        GTZC1_MPCBB1_PRIVCFGR11: 0xFFFFFFFF,
        GTZC1_MPCBB1_PRIVCFGR12: 0xFFFFFFFF,
        GTZC1_MPCBB1_PRIVCFGR13: 0xFFFFFFFF,
        GTZC1_MPCBB1_PRIVCFGR14: 0xFFFFFFFF,
        GTZC1_MPCBB1_PRIVCFGR15: 0xFFFFFFFF,
        GTZC1_MPCBB1_PRIVCFGR16: 0xFFFFFFFF,
        GTZC1_MPCBB1_PRIVCFGR17: 0xFFFFFFFF,
        GTZC1_MPCBB1_PRIVCFGR18: 0xFFFFFFFF,
        GTZC1_MPCBB1_PRIVCFGR19: 0xFFFFFFFF,
        GTZC1_MPCBB1_PRIVCFGR20: 0xFFFFFFFF,
        GTZC1_MPCBB1_PRIVCFGR21: 0xFFFFFFFF,
        GTZC1_MPCBB1_PRIVCFGR22: 0xFFFFFFFF,
        GTZC1_MPCBB1_PRIVCFGR23: 0xFFFFFFFF,
        GTZC1_MPCBB1_PRIVCFGR24: 0xFFFFFFFF,
        GTZC1_MPCBB1_PRIVCFGR25: 0xFFFFFFFF,
        GTZC1_MPCBB1_PRIVCFGR26: 0xFFFFFFFF,
        GTZC1_MPCBB1_PRIVCFGR27: 0xFFFFFFFF,
        GTZC1_MPCBB1_PRIVCFGR28: 0xFFFFFFFF,
        GTZC1_MPCBB1_PRIVCFGR29: 0xFFFFFFFF,
        GTZC1_MPCBB1_PRIVCFGR30: 0xFFFFFFFF,
        GTZC1_MPCBB1_PRIVCFGR31: 0xFFFFFFFF,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut SEC_GTZC1_MPCBB1_TAKEN: bool = false;

    /// Safe access to SEC_GTZC1_MPCBB1
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
            if SEC_GTZC1_MPCBB1_TAKEN {
                None
            } else {
                SEC_GTZC1_MPCBB1_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to SEC_GTZC1_MPCBB1
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if SEC_GTZC1_MPCBB1_TAKEN && inst.addr == INSTANCE.addr {
                SEC_GTZC1_MPCBB1_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal SEC_GTZC1_MPCBB1
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        SEC_GTZC1_MPCBB1_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to SEC_GTZC1_MPCBB1
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const SEC_GTZC1_MPCBB1: *const RegisterBlock = 0x50032c00 as *const _;
