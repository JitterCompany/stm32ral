#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Advanced-control timers
//!
//! Used by: stm32h562, stm32h563, stm32h573

#[cfg(not(feature = "nosync"))]
pub use crate::stm32h5::peripherals::tim1::Instance;
pub use crate::stm32h5::peripherals::tim1::{RegisterBlock, ResetValues};
pub use crate::stm32h5::peripherals::tim1::{
    AF1, AF2, ARR, BDTR, CCER, CCMR1, CCMR2, CCMR3, CCR1, CCR2, CCR3, CCR4, CCR5, CCR6, CNT, CR1,
    CR2, DCR, DIER, DMAR, DTR2, ECR, EGR, PSC, RCR, SMCR, SR, TISEL,
};

/// Access functions for the SEC_TIM1 peripheral instance
pub mod SEC_TIM1 {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x50012c00,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in SEC_TIM1
    pub const reset: ResetValues = ResetValues {
        CR1: 0x00000000,
        CR2: 0x00000000,
        SMCR: 0x00000000,
        DIER: 0x00000000,
        SR: 0x00000000,
        EGR: 0x00000000,
        CCMR1: 0x00000000,
        CCMR2: 0x00000000,
        CCER: 0x00000000,
        CNT: 0x00000000,
        PSC: 0x00000000,
        ARR: 0x0000FFFF,
        RCR: 0x00000000,
        CCR1: 0x00000000,
        CCR2: 0x00000000,
        CCR3: 0x00000000,
        CCR4: 0x00000000,
        BDTR: 0x00000000,
        CCR5: 0x00000000,
        CCR6: 0x00000000,
        CCMR3: 0x00000000,
        DTR2: 0x00000000,
        ECR: 0x00000000,
        TISEL: 0x00000000,
        AF1: 0x00000001,
        AF2: 0x00000001,
        DCR: 0x00000000,
        DMAR: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut SEC_TIM1_TAKEN: bool = false;

    /// Safe access to SEC_TIM1
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
            if SEC_TIM1_TAKEN {
                None
            } else {
                SEC_TIM1_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to SEC_TIM1
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if SEC_TIM1_TAKEN && inst.addr == INSTANCE.addr {
                SEC_TIM1_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal SEC_TIM1
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        SEC_TIM1_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to SEC_TIM1
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const SEC_TIM1: *const RegisterBlock = 0x50012c00 as *const _;
