#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Octo-SPI interface
//!
//! Used by: stm32h562, stm32h563, stm32h573

#[cfg(not(feature = "nosync"))]
pub use crate::stm32h5::peripherals::octospi::Instance;
pub use crate::stm32h5::peripherals::octospi::{RegisterBlock, ResetValues};
pub use crate::stm32h5::peripherals::octospi::{
    ABR, AR, CCR, CR, DCR1, DCR2, DCR3, DCR4, DLR, DR, FCR, HLCR, IR, LPTR, PIR, PSMAR, PSMKR, SR,
    TCR, WABR, WCCR, WIR, WPABR, WPCCR, WPIR, WPTCR, WTCR,
};

/// Access functions for the OCTOSPI peripheral instance
pub mod OCTOSPI {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x47001400,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in OCTOSPI
    pub const reset: ResetValues = ResetValues {
        CR: 0x00000000,
        DCR1: 0x00000000,
        DCR2: 0x00000000,
        DCR3: 0x00000000,
        DCR4: 0x00000000,
        SR: 0x00000000,
        FCR: 0x00000000,
        DLR: 0x00000000,
        AR: 0x00000000,
        DR: 0x00000000,
        PSMKR: 0x00000000,
        PSMAR: 0x00000000,
        PIR: 0x00000000,
        CCR: 0x00000000,
        TCR: 0x00000000,
        IR: 0x00000000,
        ABR: 0x00000000,
        LPTR: 0x00000000,
        WPCCR: 0x00000000,
        WPTCR: 0x00000000,
        WPIR: 0x00000000,
        WPABR: 0x00000000,
        WCCR: 0x00000000,
        WTCR: 0x00000000,
        WIR: 0x00000000,
        WABR: 0x00000000,
        HLCR: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut OCTOSPI_TAKEN: bool = false;

    /// Safe access to OCTOSPI
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
            if OCTOSPI_TAKEN {
                None
            } else {
                OCTOSPI_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to OCTOSPI
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if OCTOSPI_TAKEN && inst.addr == INSTANCE.addr {
                OCTOSPI_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal OCTOSPI
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        OCTOSPI_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to OCTOSPI
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const OCTOSPI: *const RegisterBlock = 0x47001400 as *const _;

/// Access functions for the SEC_OCTOSPI peripheral instance
pub mod SEC_OCTOSPI {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x57001400,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in SEC_OCTOSPI
    pub const reset: ResetValues = ResetValues {
        CR: 0x00000000,
        DCR1: 0x00000000,
        DCR2: 0x00000000,
        DCR3: 0x00000000,
        DCR4: 0x00000000,
        SR: 0x00000000,
        FCR: 0x00000000,
        DLR: 0x00000000,
        AR: 0x00000000,
        DR: 0x00000000,
        PSMKR: 0x00000000,
        PSMAR: 0x00000000,
        PIR: 0x00000000,
        CCR: 0x00000000,
        TCR: 0x00000000,
        IR: 0x00000000,
        ABR: 0x00000000,
        LPTR: 0x00000000,
        WPCCR: 0x00000000,
        WPTCR: 0x00000000,
        WPIR: 0x00000000,
        WPABR: 0x00000000,
        WCCR: 0x00000000,
        WTCR: 0x00000000,
        WIR: 0x00000000,
        WABR: 0x00000000,
        HLCR: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut SEC_OCTOSPI_TAKEN: bool = false;

    /// Safe access to SEC_OCTOSPI
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
            if SEC_OCTOSPI_TAKEN {
                None
            } else {
                SEC_OCTOSPI_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to SEC_OCTOSPI
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if SEC_OCTOSPI_TAKEN && inst.addr == INSTANCE.addr {
                SEC_OCTOSPI_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal SEC_OCTOSPI
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        SEC_OCTOSPI_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to SEC_OCTOSPI
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const SEC_OCTOSPI: *const RegisterBlock = 0x57001400 as *const _;
