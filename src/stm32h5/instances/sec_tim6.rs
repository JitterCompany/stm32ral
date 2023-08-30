#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Basic timers
//!
//! Used by: stm32h562, stm32h563, stm32h573

#[cfg(not(feature = "nosync"))]
pub use crate::stm32h5::peripherals::tim6::Instance;
pub use crate::stm32h5::peripherals::tim6::{RegisterBlock, ResetValues};
pub use crate::stm32h5::peripherals::tim6::{
    TIM6_ARR, TIM6_CNT, TIM6_CR1, TIM6_CR2, TIM6_DIER, TIM6_EGR, TIM6_PSC, TIM6_SR,
};

/// Access functions for the SEC_TIM6 peripheral instance
pub mod SEC_TIM6 {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x50001000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in SEC_TIM6
    pub const reset: ResetValues = ResetValues {
        TIM6_CR1: 0x00000000,
        TIM6_CR2: 0x00000000,
        TIM6_DIER: 0x00000000,
        TIM6_SR: 0x00000000,
        TIM6_EGR: 0x00000000,
        TIM6_CNT: 0x00000000,
        TIM6_PSC: 0x00000000,
        TIM6_ARR: 0x0000FFFF,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut SEC_TIM6_TAKEN: bool = false;

    /// Safe access to SEC_TIM6
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
            if SEC_TIM6_TAKEN {
                None
            } else {
                SEC_TIM6_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to SEC_TIM6
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if SEC_TIM6_TAKEN && inst.addr == INSTANCE.addr {
                SEC_TIM6_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal SEC_TIM6
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        SEC_TIM6_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to SEC_TIM6
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const SEC_TIM6: *const RegisterBlock = 0x50001000 as *const _;
