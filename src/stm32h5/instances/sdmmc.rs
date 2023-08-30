#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Secure digital input/output MultiMediaCard interface
//!
//! Used by: stm32h563, stm32h573

#[cfg(not(feature = "nosync"))]
pub use crate::stm32h5::peripherals::sdmmc1::Instance;
pub use crate::stm32h5::peripherals::sdmmc1::{RegisterBlock, ResetValues};
pub use crate::stm32h5::peripherals::sdmmc1::{
    ACKTIMER, ARGR, CLKCR, CMDR, DCNTR, DCTRL, DLENR, DTIMER, FIFOR0, FIFOR1, FIFOR10, FIFOR11,
    FIFOR12, FIFOR13, FIFOR14, FIFOR15, FIFOR2, FIFOR3, FIFOR4, FIFOR5, FIFOR6, FIFOR7, FIFOR8,
    FIFOR9, ICR, IDMABAR, IDMABASER, IDMABSIZER, IDMACTRLR, IDMALAR, MASKR, POWER, RESP1R, RESP2R,
    RESP3R, RESP4R, RESPCMDR, STAR,
};

/// Access functions for the SDMMC1 peripheral instance
pub mod SDMMC1 {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x46008000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in SDMMC1
    pub const reset: ResetValues = ResetValues {
        POWER: 0x00000000,
        CLKCR: 0x00000000,
        ARGR: 0x00000000,
        CMDR: 0x00000000,
        RESPCMDR: 0x00000000,
        RESP1R: 0x00000000,
        RESP2R: 0x00000000,
        RESP3R: 0x00000000,
        RESP4R: 0x00000000,
        DTIMER: 0x00000000,
        DLENR: 0x00000000,
        DCTRL: 0x00000000,
        DCNTR: 0x00000000,
        STAR: 0x00000000,
        ICR: 0x00000000,
        MASKR: 0x00000000,
        ACKTIMER: 0x00000000,
        IDMACTRLR: 0x00000000,
        IDMABSIZER: 0x00000000,
        IDMABASER: 0x00000000,
        IDMALAR: 0x00000000,
        IDMABAR: 0x00000000,
        FIFOR0: 0x00000000,
        FIFOR1: 0x00000000,
        FIFOR2: 0x00000000,
        FIFOR3: 0x00000000,
        FIFOR4: 0x00000000,
        FIFOR5: 0x00000000,
        FIFOR6: 0x00000000,
        FIFOR7: 0x00000000,
        FIFOR8: 0x00000000,
        FIFOR9: 0x00000000,
        FIFOR10: 0x00000000,
        FIFOR11: 0x00000000,
        FIFOR12: 0x00000000,
        FIFOR13: 0x00000000,
        FIFOR14: 0x00000000,
        FIFOR15: 0x00000000,
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
pub const SDMMC1: *const RegisterBlock = 0x46008000 as *const _;

/// Access functions for the SDMMC2 peripheral instance
pub mod SDMMC2 {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x46008c00,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in SDMMC2
    pub const reset: ResetValues = ResetValues {
        POWER: 0x00000000,
        CLKCR: 0x00000000,
        ARGR: 0x00000000,
        CMDR: 0x00000000,
        RESPCMDR: 0x00000000,
        RESP1R: 0x00000000,
        RESP2R: 0x00000000,
        RESP3R: 0x00000000,
        RESP4R: 0x00000000,
        DTIMER: 0x00000000,
        DLENR: 0x00000000,
        DCTRL: 0x00000000,
        DCNTR: 0x00000000,
        STAR: 0x00000000,
        ICR: 0x00000000,
        MASKR: 0x00000000,
        ACKTIMER: 0x00000000,
        IDMACTRLR: 0x00000000,
        IDMABSIZER: 0x00000000,
        IDMABASER: 0x00000000,
        IDMALAR: 0x00000000,
        IDMABAR: 0x00000000,
        FIFOR0: 0x00000000,
        FIFOR1: 0x00000000,
        FIFOR2: 0x00000000,
        FIFOR3: 0x00000000,
        FIFOR4: 0x00000000,
        FIFOR5: 0x00000000,
        FIFOR6: 0x00000000,
        FIFOR7: 0x00000000,
        FIFOR8: 0x00000000,
        FIFOR9: 0x00000000,
        FIFOR10: 0x00000000,
        FIFOR11: 0x00000000,
        FIFOR12: 0x00000000,
        FIFOR13: 0x00000000,
        FIFOR14: 0x00000000,
        FIFOR15: 0x00000000,
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
pub const SDMMC2: *const RegisterBlock = 0x46008c00 as *const _;

/// Access functions for the SEC_SDMMC1 peripheral instance
pub mod SEC_SDMMC1 {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x56008000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in SEC_SDMMC1
    pub const reset: ResetValues = ResetValues {
        POWER: 0x00000000,
        CLKCR: 0x00000000,
        ARGR: 0x00000000,
        CMDR: 0x00000000,
        RESPCMDR: 0x00000000,
        RESP1R: 0x00000000,
        RESP2R: 0x00000000,
        RESP3R: 0x00000000,
        RESP4R: 0x00000000,
        DTIMER: 0x00000000,
        DLENR: 0x00000000,
        DCTRL: 0x00000000,
        DCNTR: 0x00000000,
        STAR: 0x00000000,
        ICR: 0x00000000,
        MASKR: 0x00000000,
        ACKTIMER: 0x00000000,
        IDMACTRLR: 0x00000000,
        IDMABSIZER: 0x00000000,
        IDMABASER: 0x00000000,
        IDMALAR: 0x00000000,
        IDMABAR: 0x00000000,
        FIFOR0: 0x00000000,
        FIFOR1: 0x00000000,
        FIFOR2: 0x00000000,
        FIFOR3: 0x00000000,
        FIFOR4: 0x00000000,
        FIFOR5: 0x00000000,
        FIFOR6: 0x00000000,
        FIFOR7: 0x00000000,
        FIFOR8: 0x00000000,
        FIFOR9: 0x00000000,
        FIFOR10: 0x00000000,
        FIFOR11: 0x00000000,
        FIFOR12: 0x00000000,
        FIFOR13: 0x00000000,
        FIFOR14: 0x00000000,
        FIFOR15: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut SEC_SDMMC1_TAKEN: bool = false;

    /// Safe access to SEC_SDMMC1
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
            if SEC_SDMMC1_TAKEN {
                None
            } else {
                SEC_SDMMC1_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to SEC_SDMMC1
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if SEC_SDMMC1_TAKEN && inst.addr == INSTANCE.addr {
                SEC_SDMMC1_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal SEC_SDMMC1
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        SEC_SDMMC1_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to SEC_SDMMC1
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const SEC_SDMMC1: *const RegisterBlock = 0x56008000 as *const _;

/// Access functions for the SEC_SDMMC2 peripheral instance
pub mod SEC_SDMMC2 {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x56008c00,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in SEC_SDMMC2
    pub const reset: ResetValues = ResetValues {
        POWER: 0x00000000,
        CLKCR: 0x00000000,
        ARGR: 0x00000000,
        CMDR: 0x00000000,
        RESPCMDR: 0x00000000,
        RESP1R: 0x00000000,
        RESP2R: 0x00000000,
        RESP3R: 0x00000000,
        RESP4R: 0x00000000,
        DTIMER: 0x00000000,
        DLENR: 0x00000000,
        DCTRL: 0x00000000,
        DCNTR: 0x00000000,
        STAR: 0x00000000,
        ICR: 0x00000000,
        MASKR: 0x00000000,
        ACKTIMER: 0x00000000,
        IDMACTRLR: 0x00000000,
        IDMABSIZER: 0x00000000,
        IDMABASER: 0x00000000,
        IDMALAR: 0x00000000,
        IDMABAR: 0x00000000,
        FIFOR0: 0x00000000,
        FIFOR1: 0x00000000,
        FIFOR2: 0x00000000,
        FIFOR3: 0x00000000,
        FIFOR4: 0x00000000,
        FIFOR5: 0x00000000,
        FIFOR6: 0x00000000,
        FIFOR7: 0x00000000,
        FIFOR8: 0x00000000,
        FIFOR9: 0x00000000,
        FIFOR10: 0x00000000,
        FIFOR11: 0x00000000,
        FIFOR12: 0x00000000,
        FIFOR13: 0x00000000,
        FIFOR14: 0x00000000,
        FIFOR15: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut SEC_SDMMC2_TAKEN: bool = false;

    /// Safe access to SEC_SDMMC2
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
            if SEC_SDMMC2_TAKEN {
                None
            } else {
                SEC_SDMMC2_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to SEC_SDMMC2
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if SEC_SDMMC2_TAKEN && inst.addr == INSTANCE.addr {
                SEC_SDMMC2_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal SEC_SDMMC2
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        SEC_SDMMC2_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to SEC_SDMMC2
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const SEC_SDMMC2: *const RegisterBlock = 0x56008c00 as *const _;
