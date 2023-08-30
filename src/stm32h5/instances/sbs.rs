#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! SBS register block
//!
//! Used by: stm32h563, stm32h573

#[cfg(not(feature = "nosync"))]
pub use crate::stm32h5::peripherals::sbs::Instance;
pub use crate::stm32h5::peripherals::sbs::{RegisterBlock, ResetValues};
pub use crate::stm32h5::peripherals::sbs::{
    CCCSR, CCSWCR, CCVALR, CFGR2, CNSLCKR, CSLCKR, DBGCR, DBGLOCKR, ECCNMIR, EPOCHSELCR, FPUIMR,
    HDPLCR, HDPLSR, MESR, NEXTHDPLCR, PMCR, RSSCMDR, SECCFGR,
};

/// Access functions for the SBS peripheral instance
pub mod SBS {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x44000400,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in SBS
    pub const reset: ResetValues = ResetValues {
        HDPLCR: 0x000000B4,
        HDPLSR: 0x00000000,
        NEXTHDPLCR: 0x00000000,
        DBGCR: 0x00000000,
        DBGLOCKR: 0x000000B4,
        RSSCMDR: 0x00000000,
        EPOCHSELCR: 0x00000000,
        SECCFGR: 0x00000000,
        PMCR: 0x00000000,
        FPUIMR: 0x0000001F,
        MESR: 0x00000000,
        CCCSR: 0x00000000,
        CCVALR: 0x00000088,
        CCSWCR: 0x00007878,
        CFGR2: 0x00000000,
        CNSLCKR: 0x00000000,
        CSLCKR: 0x00000000,
        ECCNMIR: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut SBS_TAKEN: bool = false;

    /// Safe access to SBS
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
            if SBS_TAKEN {
                None
            } else {
                SBS_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to SBS
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if SBS_TAKEN && inst.addr == INSTANCE.addr {
                SBS_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal SBS
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        SBS_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to SBS
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const SBS: *const RegisterBlock = 0x44000400 as *const _;

/// Access functions for the SEC_SBS peripheral instance
pub mod SEC_SBS {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x54000400,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in SEC_SBS
    pub const reset: ResetValues = ResetValues {
        HDPLCR: 0x000000B4,
        HDPLSR: 0x00000000,
        NEXTHDPLCR: 0x00000000,
        DBGCR: 0x00000000,
        DBGLOCKR: 0x000000B4,
        RSSCMDR: 0x00000000,
        EPOCHSELCR: 0x00000000,
        SECCFGR: 0x00000000,
        PMCR: 0x00000000,
        FPUIMR: 0x0000001F,
        MESR: 0x00000000,
        CCCSR: 0x00000000,
        CCVALR: 0x00000088,
        CCSWCR: 0x00007878,
        CFGR2: 0x00000000,
        CNSLCKR: 0x00000000,
        CSLCKR: 0x00000000,
        ECCNMIR: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut SEC_SBS_TAKEN: bool = false;

    /// Safe access to SEC_SBS
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
            if SEC_SBS_TAKEN {
                None
            } else {
                SEC_SBS_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to SEC_SBS
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if SEC_SBS_TAKEN && inst.addr == INSTANCE.addr {
                SEC_SBS_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal SEC_SBS
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        SEC_SBS_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to SEC_SBS
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const SEC_SBS: *const RegisterBlock = 0x54000400 as *const _;
