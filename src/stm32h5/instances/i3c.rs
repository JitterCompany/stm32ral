#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Improved inter-integrated circuit
//!
//! Used by: stm32h562, stm32h563, stm32h573

#[cfg(not(feature = "nosync"))]
pub use crate::stm32h5::peripherals::i3c::Instance;
pub use crate::stm32h5::peripherals::i3c::{RegisterBlock, ResetValues};
pub use crate::stm32h5::peripherals::i3c::{
    BCR, CEVR, CFGR, CR, CRCAPR, DCR, DEVR0, DEVR1, DEVR2, DEVR3, DEVR4, EPIDR, EVR, GETCAPR,
    GETMXDSR, IBIDR, IER, MAXRLR, MAXWLR, RDR, RDWR, RMR, SER, SR, TDR, TDWR, TGTTDR, TIMINGR0,
    TIMINGR1, TIMINGR2,
};

/// Access functions for the I3C peripheral instance
pub mod I3C {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x40005c00,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in I3C
    pub const reset: ResetValues = ResetValues {
        CR: 0x00000000,
        CFGR: 0x00000000,
        RDR: 0x00000000,
        RDWR: 0x00000000,
        TDR: 0x00000000,
        TDWR: 0x00000000,
        IBIDR: 0x00000000,
        TGTTDR: 0x00000000,
        SR: 0x00000000,
        SER: 0x00000000,
        RMR: 0x00000000,
        EVR: 0x00000003,
        IER: 0x00000000,
        CEVR: 0x00000000,
        DEVR0: 0x00000000,
        DEVR1: 0x00000000,
        DEVR2: 0x00000000,
        DEVR3: 0x00000000,
        DEVR4: 0x00000000,
        MAXRLR: 0x00000000,
        MAXWLR: 0x00000000,
        TIMINGR0: 0x00000000,
        TIMINGR1: 0x00000000,
        TIMINGR2: 0x00000000,
        BCR: 0x00000000,
        DCR: 0x00000000,
        GETCAPR: 0x00000000,
        CRCAPR: 0x00000000,
        GETMXDSR: 0x00000000,
        EPIDR: 0x02080000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut I3C_TAKEN: bool = false;

    /// Safe access to I3C
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
            if I3C_TAKEN {
                None
            } else {
                I3C_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to I3C
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if I3C_TAKEN && inst.addr == INSTANCE.addr {
                I3C_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal I3C
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        I3C_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to I3C
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const I3C: *const RegisterBlock = 0x40005c00 as *const _;

/// Access functions for the SEC_I3C peripheral instance
pub mod SEC_I3C {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x50005c00,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in SEC_I3C
    pub const reset: ResetValues = ResetValues {
        CR: 0x00000000,
        CFGR: 0x00000000,
        RDR: 0x00000000,
        RDWR: 0x00000000,
        TDR: 0x00000000,
        TDWR: 0x00000000,
        IBIDR: 0x00000000,
        TGTTDR: 0x00000000,
        SR: 0x00000000,
        SER: 0x00000000,
        RMR: 0x00000000,
        EVR: 0x00000003,
        IER: 0x00000000,
        CEVR: 0x00000000,
        DEVR0: 0x00000000,
        DEVR1: 0x00000000,
        DEVR2: 0x00000000,
        DEVR3: 0x00000000,
        DEVR4: 0x00000000,
        MAXRLR: 0x00000000,
        MAXWLR: 0x00000000,
        TIMINGR0: 0x00000000,
        TIMINGR1: 0x00000000,
        TIMINGR2: 0x00000000,
        BCR: 0x00000000,
        DCR: 0x00000000,
        GETCAPR: 0x00000000,
        CRCAPR: 0x00000000,
        GETMXDSR: 0x00000000,
        EPIDR: 0x02080000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut SEC_I3C_TAKEN: bool = false;

    /// Safe access to SEC_I3C
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
            if SEC_I3C_TAKEN {
                None
            } else {
                SEC_I3C_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to SEC_I3C
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if SEC_I3C_TAKEN && inst.addr == INSTANCE.addr {
                SEC_I3C_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal SEC_I3C
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        SEC_I3C_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to SEC_I3C
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const SEC_I3C: *const RegisterBlock = 0x50005c00 as *const _;
