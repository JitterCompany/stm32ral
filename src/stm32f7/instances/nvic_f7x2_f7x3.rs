#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Nested Vectored Interrupt Controller
//!
//! Used by: stm32f7x2, stm32f7x3

#[cfg(not(feature = "nosync"))]
pub use crate::stm32f7::peripherals::nvic_v2::Instance;
pub use crate::stm32f7::peripherals::nvic_v2::{RegisterBlock, ResetValues};
pub use crate::stm32f7::peripherals::nvic_v2::{
    IABR0, IABR1, IABR2, ICER0, ICER1, ICER2, ICPR0, ICPR1, ICPR2, IPR0, IPR1, IPR10, IPR11, IPR12,
    IPR13, IPR14, IPR15, IPR16, IPR17, IPR18, IPR19, IPR2, IPR20, IPR21, IPR22, IPR23, IPR24,
    IPR25, IPR26, IPR27, IPR3, IPR4, IPR5, IPR6, IPR7, IPR8, IPR9, ISER0, ISER1, ISER2, ISPR0,
    ISPR1, ISPR2,
};

/// Access functions for the NVIC peripheral instance
pub mod NVIC {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0xe000e100,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in NVIC
    pub const reset: ResetValues = ResetValues {
        ISER0: 0x00000000,
        ISER1: 0x00000000,
        ISER2: 0x00000000,
        ICER0: 0x00000000,
        ICER1: 0x00000000,
        ICER2: 0x00000000,
        ISPR0: 0x00000000,
        ISPR1: 0x00000000,
        ISPR2: 0x00000000,
        ICPR0: 0x00000000,
        ICPR1: 0x00000000,
        ICPR2: 0x00000000,
        IABR0: 0x00000000,
        IABR1: 0x00000000,
        IABR2: 0x00000000,
        IPR0: 0x00000000,
        IPR1: 0x00000000,
        IPR2: 0x00000000,
        IPR3: 0x00000000,
        IPR4: 0x00000000,
        IPR5: 0x00000000,
        IPR6: 0x00000000,
        IPR7: 0x00000000,
        IPR8: 0x00000000,
        IPR9: 0x00000000,
        IPR10: 0x00000000,
        IPR11: 0x00000000,
        IPR12: 0x00000000,
        IPR13: 0x00000000,
        IPR14: 0x00000000,
        IPR15: 0x00000000,
        IPR16: 0x00000000,
        IPR17: 0x00000000,
        IPR18: 0x00000000,
        IPR19: 0x00000000,
        IPR20: 0x00000000,
        IPR21: 0x00000000,
        IPR22: 0x00000000,
        IPR23: 0x00000000,
        IPR24: 0x00000000,
        IPR25: 0x00000000,
        IPR26: 0x00000000,
        IPR27: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut NVIC_TAKEN: bool = false;

    /// Safe access to NVIC
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
            if NVIC_TAKEN {
                None
            } else {
                NVIC_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to NVIC
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if NVIC_TAKEN && inst.addr == INSTANCE.addr {
                NVIC_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal NVIC
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        NVIC_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to NVIC
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const NVIC: *const RegisterBlock = 0xe000e100 as *const _;
