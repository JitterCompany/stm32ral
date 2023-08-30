#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! General-purpose I/Os
//!
//! Used by: stm32h562, stm32h563, stm32h573

#[cfg(not(feature = "nosync"))]
pub use crate::stm32h5::peripherals::gpioi::Instance;
pub use crate::stm32h5::peripherals::gpioi::{RegisterBlock, ResetValues};
pub use crate::stm32h5::peripherals::gpioi::{
    AFRL, BRR, BSRR, HSLVR, IDR, LCKR, MODER, ODR, OSPEEDR, OTYPER, PUPDR, SECCFGR,
};

/// Access functions for the GPIOI peripheral instance
pub mod GPIOI {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x42022000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in GPIOI
    pub const reset: ResetValues = ResetValues {
        MODER: 0xABFFFFFF,
        OTYPER: 0x00000000,
        OSPEEDR: 0x0C000000,
        PUPDR: 0x64000000,
        IDR: 0x00000000,
        ODR: 0x00000000,
        BSRR: 0x00000000,
        LCKR: 0x00000000,
        AFRL: 0x00000000,
        BRR: 0x00000000,
        HSLVR: 0x00000000,
        SECCFGR: 0x0000FFFF,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut GPIOI_TAKEN: bool = false;

    /// Safe access to GPIOI
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
            if GPIOI_TAKEN {
                None
            } else {
                GPIOI_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to GPIOI
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if GPIOI_TAKEN && inst.addr == INSTANCE.addr {
                GPIOI_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal GPIOI
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        GPIOI_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to GPIOI
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const GPIOI: *const RegisterBlock = 0x42022000 as *const _;

/// Access functions for the SEC_GPIOI peripheral instance
pub mod SEC_GPIOI {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x52022000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in SEC_GPIOI
    pub const reset: ResetValues = ResetValues {
        MODER: 0xABFFFFFF,
        OTYPER: 0x00000000,
        OSPEEDR: 0x0C000000,
        PUPDR: 0x64000000,
        IDR: 0x00000000,
        ODR: 0x00000000,
        BSRR: 0x00000000,
        LCKR: 0x00000000,
        AFRL: 0x00000000,
        BRR: 0x00000000,
        HSLVR: 0x00000000,
        SECCFGR: 0x0000FFFF,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut SEC_GPIOI_TAKEN: bool = false;

    /// Safe access to SEC_GPIOI
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
            if SEC_GPIOI_TAKEN {
                None
            } else {
                SEC_GPIOI_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to SEC_GPIOI
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if SEC_GPIOI_TAKEN && inst.addr == INSTANCE.addr {
                SEC_GPIOI_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal SEC_GPIOI
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        SEC_GPIOI_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to SEC_GPIOI
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const SEC_GPIOI: *const RegisterBlock = 0x52022000 as *const _;
