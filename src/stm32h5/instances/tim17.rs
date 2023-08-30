#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! TIM17 address block description
//!
//! Used by: stm32h562, stm32h563, stm32h573

#[cfg(not(feature = "nosync"))]
pub use crate::stm32h5::peripherals::tim17::Instance;
pub use crate::stm32h5::peripherals::tim17::{RegisterBlock, ResetValues};
pub use crate::stm32h5::peripherals::tim17::{
    TIM17_AF1, TIM17_AF2, TIM17_ARR, TIM17_BDTR, TIM17_CCER, TIM17_CCMR1, TIM17_CCR1, TIM17_CNT,
    TIM17_CR1, TIM17_CR2, TIM17_DCR, TIM17_DIER, TIM17_DMAR, TIM17_DTR2, TIM17_EGR, TIM17_PSC,
    TIM17_RCR, TIM17_SR, TIM17_TISEL,
};

/// Access functions for the TIM17 peripheral instance
pub mod TIM17 {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x40014800,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in TIM17
    pub const reset: ResetValues = ResetValues {
        TIM17_CR1: 0x00000000,
        TIM17_CR2: 0x00000000,
        TIM17_DIER: 0x00000000,
        TIM17_SR: 0x00000000,
        TIM17_EGR: 0x00000000,
        TIM17_CCMR1: 0x00000000,
        TIM17_CCER: 0x00000000,
        TIM17_CNT: 0x00000000,
        TIM17_PSC: 0x00000000,
        TIM17_ARR: 0x0000FFFF,
        TIM17_RCR: 0x00000000,
        TIM17_CCR1: 0x00000000,
        TIM17_BDTR: 0x00000000,
        TIM17_DTR2: 0x00000000,
        TIM17_TISEL: 0x00000000,
        TIM17_AF1: 0x00000001,
        TIM17_AF2: 0x00000000,
        TIM17_DCR: 0x00000000,
        TIM17_DMAR: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut TIM17_TAKEN: bool = false;

    /// Safe access to TIM17
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
            if TIM17_TAKEN {
                None
            } else {
                TIM17_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to TIM17
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if TIM17_TAKEN && inst.addr == INSTANCE.addr {
                TIM17_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal TIM17
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        TIM17_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to TIM17
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const TIM17: *const RegisterBlock = 0x40014800 as *const _;
