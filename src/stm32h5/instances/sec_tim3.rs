#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! General-purpose timers
//!
//! Used by: stm32h562, stm32h563, stm32h573

#[cfg(not(feature = "nosync"))]
pub use crate::stm32h5::peripherals::tim3::Instance;
pub use crate::stm32h5::peripherals::tim3::{RegisterBlock, ResetValues};
pub use crate::stm32h5::peripherals::tim3::{
    TIM3_AF1, TIM3_AF2, TIM3_ARR, TIM3_CCER, TIM3_CCMR1, TIM3_CCMR2, TIM3_CCR1, TIM3_CCR2,
    TIM3_CCR3, TIM3_CCR4, TIM3_CNT, TIM3_CR1, TIM3_CR2, TIM3_DCR, TIM3_DIER, TIM3_DMAR, TIM3_ECR,
    TIM3_EGR, TIM3_PSC, TIM3_SMCR, TIM3_SR, TIM3_TISEL,
};

/// Access functions for the SEC_TIM3 peripheral instance
pub mod SEC_TIM3 {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x50000400,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in SEC_TIM3
    pub const reset: ResetValues = ResetValues {
        TIM3_CR1: 0x00000000,
        TIM3_CR2: 0x00000000,
        TIM3_SMCR: 0x00000000,
        TIM3_DIER: 0x00000000,
        TIM3_SR: 0x00000000,
        TIM3_EGR: 0x00000000,
        TIM3_CCMR1: 0x00000000,
        TIM3_CCMR2: 0x00000000,
        TIM3_CCER: 0x00000000,
        TIM3_CNT: 0x00000000,
        TIM3_PSC: 0x00000000,
        TIM3_ARR: 0xFFFFFFFF,
        TIM3_CCR1: 0x00000000,
        TIM3_CCR2: 0x00000000,
        TIM3_CCR3: 0x00000000,
        TIM3_CCR4: 0x00000000,
        TIM3_ECR: 0x00000000,
        TIM3_TISEL: 0x00000000,
        TIM3_AF1: 0x00000000,
        TIM3_AF2: 0x00000000,
        TIM3_DCR: 0x00000000,
        TIM3_DMAR: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut SEC_TIM3_TAKEN: bool = false;

    /// Safe access to SEC_TIM3
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
            if SEC_TIM3_TAKEN {
                None
            } else {
                SEC_TIM3_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to SEC_TIM3
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if SEC_TIM3_TAKEN && inst.addr == INSTANCE.addr {
                SEC_TIM3_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal SEC_TIM3
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        SEC_TIM3_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to SEC_TIM3
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const SEC_TIM3: *const RegisterBlock = 0x50000400 as *const _;
