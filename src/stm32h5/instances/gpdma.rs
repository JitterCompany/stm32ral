#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! General purpose direct memory access controller
//!
//! Used by: stm32h562, stm32h563, stm32h573

#[cfg(not(feature = "nosync"))]
pub use crate::stm32h5::peripherals::gpdma::Instance;
pub use crate::stm32h5::peripherals::gpdma::{RegisterBlock, ResetValues};
pub use crate::stm32h5::peripherals::gpdma::{
    C0BR1, C0CR, C0DAR, C0FCR, C0LBAR, C0LLR, C0SAR, C0SR, C0TR1, C0TR2, C1BR1, C1CR, C1DAR, C1FCR,
    C1LBAR, C1LLR, C1SAR, C1SR, C1TR1, C1TR2, C2BR1, C2CR, C2DAR, C2FCR, C2LBAR, C2LLR, C2SAR,
    C2SR, C2TR1, C2TR2, C3BR1, C3CR, C3DAR, C3FCR, C3LBAR, C3LLR, C3SAR, C3SR, C3TR1, C3TR2, C4BR1,
    C4CR, C4DAR, C4FCR, C4LBAR, C4LLR, C4SAR, C4SR, C4TR1, C4TR2, C5BR1, C5CR, C5DAR, C5FCR,
    C5LBAR, C5LLR, C5SAR, C5SR, C5TR1, C5TR2, C6BR1, C6BR2, C6CR, C6DAR, C6FCR, C6LBAR, C6LLR,
    C6SAR, C6SR, C6TR1, C6TR2, C6TR3, C7BR1, C7BR2, C7CR, C7DAR, C7FCR, C7LBAR, C7LLR, C7SAR, C7SR,
    C7TR1, C7TR2, C7TR3, MISR, PRIVCFGR, RCFGLOCKR, SECCFGR, SMISR,
};

/// Access functions for the GPDMA1 peripheral instance
pub mod GPDMA1 {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x40020000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in GPDMA1
    pub const reset: ResetValues = ResetValues {
        SECCFGR: 0x00000000,
        PRIVCFGR: 0x00000000,
        RCFGLOCKR: 0x00000000,
        MISR: 0x00000000,
        SMISR: 0x00000000,
        C0LBAR: 0x00000000,
        C0FCR: 0x00000000,
        C0SR: 0x00000001,
        C0CR: 0x00000000,
        C0TR1: 0x00000000,
        C0TR2: 0x00000000,
        C0BR1: 0x00000000,
        C0SAR: 0x00000000,
        C0DAR: 0x00000000,
        C0LLR: 0x00000000,
        C1LBAR: 0x00000000,
        C1FCR: 0x00000000,
        C1SR: 0x00000001,
        C1CR: 0x00000000,
        C1TR1: 0x00000000,
        C1TR2: 0x00000000,
        C1BR1: 0x00000000,
        C1SAR: 0x00000000,
        C1DAR: 0x00000000,
        C1LLR: 0x00000000,
        C2LBAR: 0x00000000,
        C2FCR: 0x00000000,
        C2SR: 0x00000001,
        C2CR: 0x00000000,
        C2TR1: 0x00000000,
        C2TR2: 0x00000000,
        C2BR1: 0x00000000,
        C2SAR: 0x00000000,
        C2DAR: 0x00000000,
        C2LLR: 0x00000000,
        C3LBAR: 0x00000000,
        C3FCR: 0x00000000,
        C3SR: 0x00000001,
        C3CR: 0x00000000,
        C3TR1: 0x00000000,
        C3TR2: 0x00000000,
        C3BR1: 0x00000000,
        C3SAR: 0x00000000,
        C3DAR: 0x00000000,
        C3LLR: 0x00000000,
        C4LBAR: 0x00000000,
        C4FCR: 0x00000000,
        C4SR: 0x00000001,
        C4CR: 0x00000000,
        C4TR1: 0x00000000,
        C4TR2: 0x00000000,
        C4BR1: 0x00000000,
        C4SAR: 0x00000000,
        C4DAR: 0x00000000,
        C4LLR: 0x00000000,
        C5LBAR: 0x00000000,
        C5FCR: 0x00000000,
        C5SR: 0x00000001,
        C5CR: 0x00000000,
        C5TR1: 0x00000000,
        C5TR2: 0x00000000,
        C5BR1: 0x00000000,
        C5SAR: 0x00000000,
        C5DAR: 0x00000000,
        C5LLR: 0x00000000,
        C6LBAR: 0x00000000,
        C6FCR: 0x00000000,
        C6SR: 0x00000001,
        C6CR: 0x00000000,
        C6TR1: 0x00000000,
        C6TR2: 0x00000000,
        C6BR1: 0x00000000,
        C6SAR: 0x00000000,
        C6DAR: 0x00000000,
        C6TR3: 0x00000000,
        C6BR2: 0x00000000,
        C6LLR: 0x00000000,
        C7LBAR: 0x00000000,
        C7FCR: 0x00000000,
        C7SR: 0x00000001,
        C7CR: 0x00000000,
        C7TR1: 0x00000000,
        C7TR2: 0x00000000,
        C7BR1: 0x00000000,
        C7SAR: 0x00000000,
        C7DAR: 0x00000000,
        C7TR3: 0x00000000,
        C7BR2: 0x00000000,
        C7LLR: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut GPDMA1_TAKEN: bool = false;

    /// Safe access to GPDMA1
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
            if GPDMA1_TAKEN {
                None
            } else {
                GPDMA1_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to GPDMA1
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if GPDMA1_TAKEN && inst.addr == INSTANCE.addr {
                GPDMA1_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal GPDMA1
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        GPDMA1_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to GPDMA1
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const GPDMA1: *const RegisterBlock = 0x40020000 as *const _;

/// Access functions for the GPDMA2 peripheral instance
pub mod GPDMA2 {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x40021000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in GPDMA2
    pub const reset: ResetValues = ResetValues {
        SECCFGR: 0x00000000,
        PRIVCFGR: 0x00000000,
        RCFGLOCKR: 0x00000000,
        MISR: 0x00000000,
        SMISR: 0x00000000,
        C0LBAR: 0x00000000,
        C0FCR: 0x00000000,
        C0SR: 0x00000001,
        C0CR: 0x00000000,
        C0TR1: 0x00000000,
        C0TR2: 0x00000000,
        C0BR1: 0x00000000,
        C0SAR: 0x00000000,
        C0DAR: 0x00000000,
        C0LLR: 0x00000000,
        C1LBAR: 0x00000000,
        C1FCR: 0x00000000,
        C1SR: 0x00000001,
        C1CR: 0x00000000,
        C1TR1: 0x00000000,
        C1TR2: 0x00000000,
        C1BR1: 0x00000000,
        C1SAR: 0x00000000,
        C1DAR: 0x00000000,
        C1LLR: 0x00000000,
        C2LBAR: 0x00000000,
        C2FCR: 0x00000000,
        C2SR: 0x00000001,
        C2CR: 0x00000000,
        C2TR1: 0x00000000,
        C2TR2: 0x00000000,
        C2BR1: 0x00000000,
        C2SAR: 0x00000000,
        C2DAR: 0x00000000,
        C2LLR: 0x00000000,
        C3LBAR: 0x00000000,
        C3FCR: 0x00000000,
        C3SR: 0x00000001,
        C3CR: 0x00000000,
        C3TR1: 0x00000000,
        C3TR2: 0x00000000,
        C3BR1: 0x00000000,
        C3SAR: 0x00000000,
        C3DAR: 0x00000000,
        C3LLR: 0x00000000,
        C4LBAR: 0x00000000,
        C4FCR: 0x00000000,
        C4SR: 0x00000001,
        C4CR: 0x00000000,
        C4TR1: 0x00000000,
        C4TR2: 0x00000000,
        C4BR1: 0x00000000,
        C4SAR: 0x00000000,
        C4DAR: 0x00000000,
        C4LLR: 0x00000000,
        C5LBAR: 0x00000000,
        C5FCR: 0x00000000,
        C5SR: 0x00000001,
        C5CR: 0x00000000,
        C5TR1: 0x00000000,
        C5TR2: 0x00000000,
        C5BR1: 0x00000000,
        C5SAR: 0x00000000,
        C5DAR: 0x00000000,
        C5LLR: 0x00000000,
        C6LBAR: 0x00000000,
        C6FCR: 0x00000000,
        C6SR: 0x00000001,
        C6CR: 0x00000000,
        C6TR1: 0x00000000,
        C6TR2: 0x00000000,
        C6BR1: 0x00000000,
        C6SAR: 0x00000000,
        C6DAR: 0x00000000,
        C6TR3: 0x00000000,
        C6BR2: 0x00000000,
        C6LLR: 0x00000000,
        C7LBAR: 0x00000000,
        C7FCR: 0x00000000,
        C7SR: 0x00000001,
        C7CR: 0x00000000,
        C7TR1: 0x00000000,
        C7TR2: 0x00000000,
        C7BR1: 0x00000000,
        C7SAR: 0x00000000,
        C7DAR: 0x00000000,
        C7TR3: 0x00000000,
        C7BR2: 0x00000000,
        C7LLR: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut GPDMA2_TAKEN: bool = false;

    /// Safe access to GPDMA2
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
            if GPDMA2_TAKEN {
                None
            } else {
                GPDMA2_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to GPDMA2
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if GPDMA2_TAKEN && inst.addr == INSTANCE.addr {
                GPDMA2_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal GPDMA2
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        GPDMA2_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to GPDMA2
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const GPDMA2: *const RegisterBlock = 0x40021000 as *const _;

/// Access functions for the SEC_GPDMA1 peripheral instance
pub mod SEC_GPDMA1 {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x50020000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in SEC_GPDMA1
    pub const reset: ResetValues = ResetValues {
        SECCFGR: 0x00000000,
        PRIVCFGR: 0x00000000,
        RCFGLOCKR: 0x00000000,
        MISR: 0x00000000,
        SMISR: 0x00000000,
        C0LBAR: 0x00000000,
        C0FCR: 0x00000000,
        C0SR: 0x00000001,
        C0CR: 0x00000000,
        C0TR1: 0x00000000,
        C0TR2: 0x00000000,
        C0BR1: 0x00000000,
        C0SAR: 0x00000000,
        C0DAR: 0x00000000,
        C0LLR: 0x00000000,
        C1LBAR: 0x00000000,
        C1FCR: 0x00000000,
        C1SR: 0x00000001,
        C1CR: 0x00000000,
        C1TR1: 0x00000000,
        C1TR2: 0x00000000,
        C1BR1: 0x00000000,
        C1SAR: 0x00000000,
        C1DAR: 0x00000000,
        C1LLR: 0x00000000,
        C2LBAR: 0x00000000,
        C2FCR: 0x00000000,
        C2SR: 0x00000001,
        C2CR: 0x00000000,
        C2TR1: 0x00000000,
        C2TR2: 0x00000000,
        C2BR1: 0x00000000,
        C2SAR: 0x00000000,
        C2DAR: 0x00000000,
        C2LLR: 0x00000000,
        C3LBAR: 0x00000000,
        C3FCR: 0x00000000,
        C3SR: 0x00000001,
        C3CR: 0x00000000,
        C3TR1: 0x00000000,
        C3TR2: 0x00000000,
        C3BR1: 0x00000000,
        C3SAR: 0x00000000,
        C3DAR: 0x00000000,
        C3LLR: 0x00000000,
        C4LBAR: 0x00000000,
        C4FCR: 0x00000000,
        C4SR: 0x00000001,
        C4CR: 0x00000000,
        C4TR1: 0x00000000,
        C4TR2: 0x00000000,
        C4BR1: 0x00000000,
        C4SAR: 0x00000000,
        C4DAR: 0x00000000,
        C4LLR: 0x00000000,
        C5LBAR: 0x00000000,
        C5FCR: 0x00000000,
        C5SR: 0x00000001,
        C5CR: 0x00000000,
        C5TR1: 0x00000000,
        C5TR2: 0x00000000,
        C5BR1: 0x00000000,
        C5SAR: 0x00000000,
        C5DAR: 0x00000000,
        C5LLR: 0x00000000,
        C6LBAR: 0x00000000,
        C6FCR: 0x00000000,
        C6SR: 0x00000001,
        C6CR: 0x00000000,
        C6TR1: 0x00000000,
        C6TR2: 0x00000000,
        C6BR1: 0x00000000,
        C6SAR: 0x00000000,
        C6DAR: 0x00000000,
        C6TR3: 0x00000000,
        C6BR2: 0x00000000,
        C6LLR: 0x00000000,
        C7LBAR: 0x00000000,
        C7FCR: 0x00000000,
        C7SR: 0x00000001,
        C7CR: 0x00000000,
        C7TR1: 0x00000000,
        C7TR2: 0x00000000,
        C7BR1: 0x00000000,
        C7SAR: 0x00000000,
        C7DAR: 0x00000000,
        C7TR3: 0x00000000,
        C7BR2: 0x00000000,
        C7LLR: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut SEC_GPDMA1_TAKEN: bool = false;

    /// Safe access to SEC_GPDMA1
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
            if SEC_GPDMA1_TAKEN {
                None
            } else {
                SEC_GPDMA1_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to SEC_GPDMA1
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if SEC_GPDMA1_TAKEN && inst.addr == INSTANCE.addr {
                SEC_GPDMA1_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal SEC_GPDMA1
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        SEC_GPDMA1_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to SEC_GPDMA1
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const SEC_GPDMA1: *const RegisterBlock = 0x50020000 as *const _;

/// Access functions for the SEC_GPDMA2 peripheral instance
pub mod SEC_GPDMA2 {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x50021000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in SEC_GPDMA2
    pub const reset: ResetValues = ResetValues {
        SECCFGR: 0x00000000,
        PRIVCFGR: 0x00000000,
        RCFGLOCKR: 0x00000000,
        MISR: 0x00000000,
        SMISR: 0x00000000,
        C0LBAR: 0x00000000,
        C0FCR: 0x00000000,
        C0SR: 0x00000001,
        C0CR: 0x00000000,
        C0TR1: 0x00000000,
        C0TR2: 0x00000000,
        C0BR1: 0x00000000,
        C0SAR: 0x00000000,
        C0DAR: 0x00000000,
        C0LLR: 0x00000000,
        C1LBAR: 0x00000000,
        C1FCR: 0x00000000,
        C1SR: 0x00000001,
        C1CR: 0x00000000,
        C1TR1: 0x00000000,
        C1TR2: 0x00000000,
        C1BR1: 0x00000000,
        C1SAR: 0x00000000,
        C1DAR: 0x00000000,
        C1LLR: 0x00000000,
        C2LBAR: 0x00000000,
        C2FCR: 0x00000000,
        C2SR: 0x00000001,
        C2CR: 0x00000000,
        C2TR1: 0x00000000,
        C2TR2: 0x00000000,
        C2BR1: 0x00000000,
        C2SAR: 0x00000000,
        C2DAR: 0x00000000,
        C2LLR: 0x00000000,
        C3LBAR: 0x00000000,
        C3FCR: 0x00000000,
        C3SR: 0x00000001,
        C3CR: 0x00000000,
        C3TR1: 0x00000000,
        C3TR2: 0x00000000,
        C3BR1: 0x00000000,
        C3SAR: 0x00000000,
        C3DAR: 0x00000000,
        C3LLR: 0x00000000,
        C4LBAR: 0x00000000,
        C4FCR: 0x00000000,
        C4SR: 0x00000001,
        C4CR: 0x00000000,
        C4TR1: 0x00000000,
        C4TR2: 0x00000000,
        C4BR1: 0x00000000,
        C4SAR: 0x00000000,
        C4DAR: 0x00000000,
        C4LLR: 0x00000000,
        C5LBAR: 0x00000000,
        C5FCR: 0x00000000,
        C5SR: 0x00000001,
        C5CR: 0x00000000,
        C5TR1: 0x00000000,
        C5TR2: 0x00000000,
        C5BR1: 0x00000000,
        C5SAR: 0x00000000,
        C5DAR: 0x00000000,
        C5LLR: 0x00000000,
        C6LBAR: 0x00000000,
        C6FCR: 0x00000000,
        C6SR: 0x00000001,
        C6CR: 0x00000000,
        C6TR1: 0x00000000,
        C6TR2: 0x00000000,
        C6BR1: 0x00000000,
        C6SAR: 0x00000000,
        C6DAR: 0x00000000,
        C6TR3: 0x00000000,
        C6BR2: 0x00000000,
        C6LLR: 0x00000000,
        C7LBAR: 0x00000000,
        C7FCR: 0x00000000,
        C7SR: 0x00000001,
        C7CR: 0x00000000,
        C7TR1: 0x00000000,
        C7TR2: 0x00000000,
        C7BR1: 0x00000000,
        C7SAR: 0x00000000,
        C7DAR: 0x00000000,
        C7TR3: 0x00000000,
        C7BR2: 0x00000000,
        C7LLR: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut SEC_GPDMA2_TAKEN: bool = false;

    /// Safe access to SEC_GPDMA2
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
            if SEC_GPDMA2_TAKEN {
                None
            } else {
                SEC_GPDMA2_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to SEC_GPDMA2
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if SEC_GPDMA2_TAKEN && inst.addr == INSTANCE.addr {
                SEC_GPDMA2_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal SEC_GPDMA2
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        SEC_GPDMA2_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to SEC_GPDMA2
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const SEC_GPDMA2: *const RegisterBlock = 0x50021000 as *const _;
