#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! General-purpose timers
//!
//! Used by: stm32h562, stm32h563, stm32h573

#[cfg(not(feature = "nosync"))]
pub use crate::stm32h5::peripherals::tim13::Instance;
pub use crate::stm32h5::peripherals::tim13::{RegisterBlock, ResetValues};
pub use crate::stm32h5::peripherals::tim13::{
    TIM2_ARR, TIM2_CCER, TIM2_CCMR1, TIM2_CCR1, TIM2_CNT, TIM2_CR1, TIM2_DIER, TIM2_EGR, TIM2_PSC,
    TIM2_SR, TIM2_TISEL,
};

/// Access functions for the SEC_TIM14 peripheral instance
pub mod SEC_TIM14 {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x50002000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in SEC_TIM14
    pub const reset: ResetValues = ResetValues {
        TIM2_CR1: 0x00000000,
        TIM2_DIER: 0x00000000,
        TIM2_SR: 0x00000000,
        TIM2_EGR: 0x00000000,
        TIM2_CCMR1: 0x00000000,
        TIM2_CCER: 0x00000000,
        TIM2_CNT: 0x00000000,
        TIM2_PSC: 0x00000000,
        TIM2_ARR: 0xFFFFFFFF,
        TIM2_CCR1: 0x00000000,
        TIM2_TISEL: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut SEC_TIM14_TAKEN: bool = false;

    /// Safe access to SEC_TIM14
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
            if SEC_TIM14_TAKEN {
                None
            } else {
                SEC_TIM14_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to SEC_TIM14
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if SEC_TIM14_TAKEN && inst.addr == INSTANCE.addr {
                SEC_TIM14_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal SEC_TIM14
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        SEC_TIM14_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to SEC_TIM14
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const SEC_TIM14: *const RegisterBlock = 0x50002000 as *const _;