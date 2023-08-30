#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! TIM16 address block description
//!
//! Used by: stm32h562, stm32h563, stm32h573

#[cfg(not(feature = "nosync"))]
pub use crate::stm32h5::peripherals::tim16::Instance;
pub use crate::stm32h5::peripherals::tim16::{RegisterBlock, ResetValues};
pub use crate::stm32h5::peripherals::tim16::{
    TIM16_AF1, TIM16_AF2, TIM16_ARR, TIM16_BDTR, TIM16_CCER, TIM16_CCMR1, TIM16_CCR1, TIM16_CNT,
    TIM16_CR1, TIM16_CR2, TIM16_DCR, TIM16_DIER, TIM16_DMAR, TIM16_DTR2, TIM16_EGR, TIM16_PSC,
    TIM16_RCR, TIM16_SR, TIM16_TISEL,
};

/// Access functions for the TIM16 peripheral instance
pub mod TIM16 {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x40014400,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in TIM16
    pub const reset: ResetValues = ResetValues {
        TIM16_CR1: 0x00000000,
        TIM16_CR2: 0x00000000,
        TIM16_DIER: 0x00000000,
        TIM16_SR: 0x00000000,
        TIM16_EGR: 0x00000000,
        TIM16_CCMR1: 0x00000000,
        TIM16_CCER: 0x00000000,
        TIM16_CNT: 0x00000000,
        TIM16_PSC: 0x00000000,
        TIM16_ARR: 0x0000FFFF,
        TIM16_RCR: 0x00000000,
        TIM16_CCR1: 0x00000000,
        TIM16_BDTR: 0x00000000,
        TIM16_DTR2: 0x00000000,
        TIM16_TISEL: 0x00000000,
        TIM16_AF1: 0x00000001,
        TIM16_AF2: 0x00000000,
        TIM16_DCR: 0x00000000,
        TIM16_DMAR: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut TIM16_TAKEN: bool = false;

    /// Safe access to TIM16
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
            if TIM16_TAKEN {
                None
            } else {
                TIM16_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to TIM16
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if TIM16_TAKEN && inst.addr == INSTANCE.addr {
                TIM16_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal TIM16
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        TIM16_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to TIM16
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const TIM16: *const RegisterBlock = 0x40014400 as *const _;
