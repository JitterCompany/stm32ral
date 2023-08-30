#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! USB on the go high speed
//!
//! Used by: stm32f730, stm32f745, stm32f750, stm32f765, stm32f7x2, stm32f7x3, stm32f7x6, stm32f7x7, stm32f7x9

#[cfg(not(feature = "nosync"))]
pub use crate::stm32f7::peripherals::otg_hs_host::Instance;
pub use crate::stm32f7::peripherals::otg_hs_host::{RegisterBlock, ResetValues};
pub use crate::stm32f7::peripherals::otg_hs_host::{
    CHAR0, CHAR1, CHAR10, CHAR11, CHAR12, CHAR13, CHAR14, CHAR15, CHAR2, CHAR3, CHAR4, CHAR5,
    CHAR6, CHAR7, CHAR8, CHAR9, DMA0, DMA1, DMA10, DMA11, DMA12, DMA13, DMA14, DMA15, DMA2, DMA3,
    DMA4, DMA5, DMA6, DMA7, DMA8, DMA9, HAINT, HAINTMSK, HCFG, HFIR, HFNUM, HPRT, HPTXSTS, INT0,
    INT1, INT10, INT11, INT12, INT13, INT14, INT15, INT2, INT3, INT4, INT5, INT6, INT7, INT8, INT9,
    INTMSK0, INTMSK1, INTMSK10, INTMSK11, INTMSK12, INTMSK13, INTMSK14, INTMSK15, INTMSK2, INTMSK3,
    INTMSK4, INTMSK5, INTMSK6, INTMSK7, INTMSK8, INTMSK9, SPLT0, SPLT1, SPLT10, SPLT11, SPLT12,
    SPLT13, SPLT14, SPLT15, SPLT2, SPLT3, SPLT4, SPLT5, SPLT6, SPLT7, SPLT8, SPLT9, TSIZ0, TSIZ1,
    TSIZ10, TSIZ11, TSIZ12, TSIZ13, TSIZ14, TSIZ15, TSIZ2, TSIZ3, TSIZ4, TSIZ5, TSIZ6, TSIZ7,
    TSIZ8, TSIZ9,
};

/// Access functions for the OTG_HS_HOST peripheral instance
pub mod OTG_HS_HOST {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x40040400,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in OTG_HS_HOST
    pub const reset: ResetValues = ResetValues {
        HCFG: 0x00000000,
        HFIR: 0x0000EA60,
        HFNUM: 0x00003FFF,
        HPTXSTS: 0x00080100,
        HAINT: 0x00000000,
        HAINTMSK: 0x00000000,
        HPRT: 0x00000000,
        CHAR0: 0x00000000,
        SPLT0: 0x00000000,
        INT0: 0x00000000,
        INTMSK0: 0x00000000,
        TSIZ0: 0x00000000,
        DMA0: 0x00000000,
        CHAR1: 0x00000000,
        SPLT1: 0x00000000,
        INT1: 0x00000000,
        INTMSK1: 0x00000000,
        TSIZ1: 0x00000000,
        DMA1: 0x00000000,
        CHAR2: 0x00000000,
        SPLT2: 0x00000000,
        INT2: 0x00000000,
        INTMSK2: 0x00000000,
        TSIZ2: 0x00000000,
        DMA2: 0x00000000,
        CHAR3: 0x00000000,
        SPLT3: 0x00000000,
        INT3: 0x00000000,
        INTMSK3: 0x00000000,
        TSIZ3: 0x00000000,
        DMA3: 0x00000000,
        CHAR4: 0x00000000,
        SPLT4: 0x00000000,
        INT4: 0x00000000,
        INTMSK4: 0x00000000,
        TSIZ4: 0x00000000,
        DMA4: 0x00000000,
        CHAR5: 0x00000000,
        SPLT5: 0x00000000,
        INT5: 0x00000000,
        INTMSK5: 0x00000000,
        TSIZ5: 0x00000000,
        DMA5: 0x00000000,
        CHAR6: 0x00000000,
        SPLT6: 0x00000000,
        INT6: 0x00000000,
        INTMSK6: 0x00000000,
        TSIZ6: 0x00000000,
        DMA6: 0x00000000,
        CHAR7: 0x00000000,
        SPLT7: 0x00000000,
        INT7: 0x00000000,
        INTMSK7: 0x00000000,
        TSIZ7: 0x00000000,
        DMA7: 0x00000000,
        CHAR8: 0x00000000,
        SPLT8: 0x00000000,
        INT8: 0x00000000,
        INTMSK8: 0x00000000,
        TSIZ8: 0x00000000,
        DMA8: 0x00000000,
        CHAR9: 0x00000000,
        SPLT9: 0x00000000,
        INT9: 0x00000000,
        INTMSK9: 0x00000000,
        TSIZ9: 0x00000000,
        DMA9: 0x00000000,
        CHAR10: 0x00000000,
        SPLT10: 0x00000000,
        INT10: 0x00000000,
        INTMSK10: 0x00000000,
        TSIZ10: 0x00000000,
        DMA10: 0x00000000,
        CHAR11: 0x00000000,
        SPLT11: 0x00000000,
        INT11: 0x00000000,
        INTMSK11: 0x00000000,
        TSIZ11: 0x00000000,
        DMA11: 0x00000000,
        CHAR12: 0x00000000,
        SPLT12: 0x00000000,
        INT12: 0x00000000,
        INTMSK12: 0x00000000,
        TSIZ12: 0x00000000,
        DMA12: 0x00000000,
        CHAR13: 0x00000000,
        SPLT13: 0x00000000,
        INT13: 0x00000000,
        INTMSK13: 0x00000000,
        TSIZ13: 0x00000000,
        DMA13: 0x00000000,
        CHAR14: 0x00000000,
        SPLT14: 0x00000000,
        INT14: 0x00000000,
        INTMSK14: 0x00000000,
        TSIZ14: 0x00000000,
        DMA14: 0x00000000,
        CHAR15: 0x00000000,
        SPLT15: 0x00000000,
        INT15: 0x00000000,
        INTMSK15: 0x00000000,
        TSIZ15: 0x00000000,
        DMA15: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut OTG_HS_HOST_TAKEN: bool = false;

    /// Safe access to OTG_HS_HOST
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
            if OTG_HS_HOST_TAKEN {
                None
            } else {
                OTG_HS_HOST_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to OTG_HS_HOST
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if OTG_HS_HOST_TAKEN && inst.addr == INSTANCE.addr {
                OTG_HS_HOST_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal OTG_HS_HOST
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        OTG_HS_HOST_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to OTG_HS_HOST
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const OTG_HS_HOST: *const RegisterBlock = 0x40040400 as *const _;
