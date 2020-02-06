#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Secure digital input/output interface
//!
//! Used by: stm32f765, stm32f7x7, stm32f7x9

#[cfg(not(feature = "nosync"))]
pub use crate::stm32f7::peripherals::sdmmc1::Instance;
pub use crate::stm32f7::peripherals::sdmmc1::{RegisterBlock, ResetValues};
pub use crate::stm32f7::peripherals::sdmmc1::{
    ARG, CLKCR, CMD, DCOUNT, DCTRL, DLEN, DTIMER, FIFO, FIFOCNT, ICR, MASK, POWER, RESP1, RESP2,
    RESP3, RESP4, RESPCMD, STA,
};

/// Access functions for the SDMMC1 peripheral instance
pub mod SDMMC1 {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x40012c00,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in SDMMC1
    pub const reset: ResetValues = ResetValues {
        POWER: 0x00000000,
        CLKCR: 0x00000000,
        ARG: 0x00000000,
        CMD: 0x00000000,
        RESPCMD: 0x00000000,
        RESP1: 0x00000000,
        RESP2: 0x00000000,
        RESP3: 0x00000000,
        RESP4: 0x00000000,
        DTIMER: 0x00000000,
        DLEN: 0x00000000,
        DCTRL: 0x00000000,
        DCOUNT: 0x00000000,
        STA: 0x00000000,
        ICR: 0x00000000,
        MASK: 0x00000000,
        FIFOCNT: 0x00000000,
        FIFO: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut SDMMC1_TAKEN: bool = false;

    /// Safe access to SDMMC1
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
            if SDMMC1_TAKEN {
                None
            } else {
                SDMMC1_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to SDMMC1
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if SDMMC1_TAKEN && inst.addr == INSTANCE.addr {
                SDMMC1_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal SDMMC1
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        SDMMC1_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to SDMMC1
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const SDMMC1: *const RegisterBlock = 0x40012c00 as *const _;

/// Access functions for the SDMMC2 peripheral instance
pub mod SDMMC2 {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x40011c00,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in SDMMC2
    pub const reset: ResetValues = ResetValues {
        POWER: 0x00000000,
        CLKCR: 0x00000000,
        ARG: 0x00000000,
        CMD: 0x00000000,
        RESPCMD: 0x00000000,
        RESP1: 0x00000000,
        RESP2: 0x00000000,
        RESP3: 0x00000000,
        RESP4: 0x00000000,
        DTIMER: 0x00000000,
        DLEN: 0x00000000,
        DCTRL: 0x00000000,
        DCOUNT: 0x00000000,
        STA: 0x00000000,
        ICR: 0x00000000,
        MASK: 0x00000000,
        FIFOCNT: 0x00000000,
        FIFO: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut SDMMC2_TAKEN: bool = false;

    /// Safe access to SDMMC2
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
            if SDMMC2_TAKEN {
                None
            } else {
                SDMMC2_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to SDMMC2
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if SDMMC2_TAKEN && inst.addr == INSTANCE.addr {
                SDMMC2_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal SDMMC2
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        SDMMC2_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to SDMMC2
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const SDMMC2: *const RegisterBlock = 0x40011c00 as *const _;
