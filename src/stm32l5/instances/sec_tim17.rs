#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! General purpose timers
//!
//! Used by: stm32l552, stm32l562

#[cfg(not(feature = "nosync"))]
pub use crate::stm32l5::peripherals::tim17::Instance;
pub use crate::stm32l5::peripherals::tim17::{RegisterBlock, ResetValues};
pub use crate::stm32l5::peripherals::tim17::{
    ARR, BDTR, CCER, CCMR1, CCR1, CNT, CR1, CR2, DCR, DIER, DMAR, EGR, OR1, OR2, PSC, RCR, SR,
};

/// Access functions for the SEC_TIM17 peripheral instance
pub mod SEC_TIM17 {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x50014800,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in SEC_TIM17
    pub const reset: ResetValues = ResetValues {
        CR1: 0x00000000,
        CR2: 0x00000000,
        DIER: 0x00000000,
        SR: 0x00000000,
        EGR: 0x00000000,
        CCMR1: 0x00000000,
        CCER: 0x00000000,
        CNT: 0x00000000,
        PSC: 0x00000000,
        ARR: 0x0000FFFF,
        RCR: 0x00000000,
        CCR1: 0x00000000,
        BDTR: 0x00000000,
        DCR: 0x00000000,
        DMAR: 0x00000000,
        OR1: 0x00000000,
        OR2: 0x00000001,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut SEC_TIM17_TAKEN: bool = false;

    /// Safe access to SEC_TIM17
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
            if SEC_TIM17_TAKEN {
                None
            } else {
                SEC_TIM17_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to SEC_TIM17
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if SEC_TIM17_TAKEN && inst.addr == INSTANCE.addr {
                SEC_TIM17_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal SEC_TIM17
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        SEC_TIM17_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to SEC_TIM17
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const SEC_TIM17: *const RegisterBlock = 0x50014800 as *const _;
