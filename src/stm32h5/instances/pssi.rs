#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Parallel synchronous slave interface
//!
//! Used by: stm32h562, stm32h563, stm32h573

#[cfg(not(feature = "nosync"))]
pub use crate::stm32h5::peripherals::pssi::Instance;
pub use crate::stm32h5::peripherals::pssi::{RegisterBlock, ResetValues};
pub use crate::stm32h5::peripherals::pssi::{CR, DR, ICR, IER, MIS, RIS, SR};

/// Access functions for the PSSI peripheral instance
pub mod PSSI {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x4202c400,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in PSSI
    pub const reset: ResetValues = ResetValues {
        CR: 0x40000000,
        SR: 0x00000000,
        RIS: 0x00000000,
        IER: 0x00000000,
        MIS: 0x00000000,
        ICR: 0x00000000,
        DR: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut PSSI_TAKEN: bool = false;

    /// Safe access to PSSI
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
            if PSSI_TAKEN {
                None
            } else {
                PSSI_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to PSSI
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if PSSI_TAKEN && inst.addr == INSTANCE.addr {
                PSSI_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal PSSI
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        PSSI_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to PSSI
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const PSSI: *const RegisterBlock = 0x4202c400 as *const _;

/// Access functions for the SEC_PSSI peripheral instance
pub mod SEC_PSSI {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x5202c400,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in SEC_PSSI
    pub const reset: ResetValues = ResetValues {
        CR: 0x40000000,
        SR: 0x00000000,
        RIS: 0x00000000,
        IER: 0x00000000,
        MIS: 0x00000000,
        ICR: 0x00000000,
        DR: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut SEC_PSSI_TAKEN: bool = false;

    /// Safe access to SEC_PSSI
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
            if SEC_PSSI_TAKEN {
                None
            } else {
                SEC_PSSI_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to SEC_PSSI
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if SEC_PSSI_TAKEN && inst.addr == INSTANCE.addr {
                SEC_PSSI_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal SEC_PSSI
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        SEC_PSSI_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to SEC_PSSI
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const SEC_PSSI: *const RegisterBlock = 0x5202c400 as *const _;
