#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Reset and clock control
//!
//! Used by: stm32g07x, stm32g081

#[cfg(not(feature = "nosync"))]
pub use crate::stm32g0::peripherals::rcc_v2::Instance;
pub use crate::stm32g0::peripherals::rcc_v2::{RegisterBlock, ResetValues};
pub use crate::stm32g0::peripherals::rcc_v2::{
    AHBENR, AHBRSTR, AHBSMENR, APBENR1, APBENR2, APBRSTR1, APBRSTR2, APBSMENR1, APBSMENR2, BDCR,
    CCIPR, CFGR, CICR, CIER, CIFR, CR, CSR, ICSCR, IOPENR, IOPRSTR, IOPSMENR, PLLSYSCFGR,
};

/// Access functions for the RCC peripheral instance
pub mod RCC {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x40021000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in RCC
    pub const reset: ResetValues = ResetValues {
        CR: 0x00000063,
        ICSCR: 0x10000000,
        CFGR: 0x00000000,
        PLLSYSCFGR: 0x00001000,
        CIER: 0x00000000,
        CIFR: 0x00000000,
        CICR: 0x00000000,
        AHBRSTR: 0x00000000,
        IOPRSTR: 0x00000000,
        APBRSTR1: 0x00000000,
        APBRSTR2: 0x00000000,
        IOPENR: 0x00000000,
        AHBENR: 0x00000000,
        APBENR1: 0x00000000,
        APBENR2: 0x00000000,
        IOPSMENR: 0x00000000,
        AHBSMENR: 0x00000000,
        APBSMENR1: 0x00000000,
        APBSMENR2: 0x00000000,
        CCIPR: 0x00000000,
        BDCR: 0x00000000,
        CSR: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut RCC_TAKEN: bool = false;

    /// Safe access to RCC
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
            if RCC_TAKEN {
                None
            } else {
                RCC_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to RCC
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if RCC_TAKEN && inst.addr == INSTANCE.addr {
                RCC_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal RCC
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        RCC_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to RCC
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const RCC: *const RegisterBlock = 0x40021000 as *const _;
