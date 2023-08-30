#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! USB 1 on the go high speed
//!
//! Used by: stm32h735, stm32h743, stm32h743v, stm32h747cm4, stm32h747cm7, stm32h753, stm32h753v

#[cfg(not(feature = "nosync"))]
pub use crate::stm32h7::peripherals::otg_hs_device::Instance;
pub use crate::stm32h7::peripherals::otg_hs_device::{RegisterBlock, ResetValues};
pub use crate::stm32h7::peripherals::otg_hs_device::{
    CTL, CTL, CTL1, CTL1, CTL2, CTL2, CTL3, CTL3, CTL4, CTL4, CTL5, CTL5, CTL6, CTL6, CTL7, CTL7,
    CTL8, CTL8, DAINT, DAINTMSK, DCFG, DCTL, DEACHINT, DEACHINTMSK, DIEPEACHMSK1, DIEPEMPMSK,
    DIEPMSK, DMA, DMA, DMA1, DMA1, DMA2, DMA2, DMA3, DMA3, DMA4, DMA4, DMA5, DMA5, DMA6, DMA6,
    DMA7, DMA7, DMA8, DMA8, DOEPEACHMSK1, DOEPMSK, DSTS, DTHRCTL, DVBUSDIS, DVBUSPULSE, INT, INT,
    INT1, INT1, INT2, INT2, INT3, INT3, INT4, INT4, INT5, INT5, INT6, INT6, INT7, INT7, INT8, INT8,
    TSIZ, TSIZ, TSIZ1, TSIZ1, TSIZ2, TSIZ2, TSIZ3, TSIZ3, TSIZ4, TSIZ4, TSIZ5, TSIZ5, TSIZ6, TSIZ6,
    TSIZ7, TSIZ7, TSIZ8, TSIZ8, TXFSTS, TXFSTS1, TXFSTS2, TXFSTS3, TXFSTS4, TXFSTS5, TXFSTS6,
    TXFSTS7, TXFSTS8,
};

/// Access functions for the OTG1_HS_DEVICE peripheral instance
pub mod OTG1_HS_DEVICE {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x40040800,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in OTG1_HS_DEVICE
    pub const reset: ResetValues = ResetValues {
        DCFG: 0x02200000,
        DCTL: 0x00000000,
        DSTS: 0x00000010,
        DIEPMSK: 0x00000000,
        DOEPMSK: 0x00000000,
        DAINT: 0x00000000,
        DAINTMSK: 0x00000000,
        DVBUSDIS: 0x000017D7,
        DVBUSPULSE: 0x000005B8,
        DTHRCTL: 0x00000000,
        DIEPEMPMSK: 0x00000000,
        DEACHINT: 0x00000000,
        DEACHINTMSK: 0x00000000,
        DIEPEACHMSK1: 0x00000000,
        DOEPEACHMSK1: 0x00000000,
        CTL: 0x00000000,
        INT: 0x00000080,
        TSIZ: 0x00000000,
        DMA: 0x00000000,
        TXFSTS: 0x00000000,
        CTL1: 0x00000000,
        INT1: 0x00000000,
        TSIZ1: 0x00000000,
        DMA1: 0x00000000,
        TXFSTS1: 0x00000000,
        CTL2: 0x00000000,
        INT2: 0x00000000,
        TSIZ2: 0x00000000,
        DMA2: 0x00000000,
        TXFSTS2: 0x00000000,
        CTL3: 0x00000000,
        INT3: 0x00000000,
        TSIZ3: 0x00000000,
        DMA3: 0x00000000,
        TXFSTS3: 0x00000000,
        CTL4: 0x00000000,
        INT4: 0x00000000,
        TSIZ4: 0x00000000,
        DMA4: 0x00000000,
        TXFSTS4: 0x00000000,
        CTL5: 0x00000000,
        INT5: 0x00000000,
        TSIZ5: 0x00000000,
        DMA5: 0x00000000,
        TXFSTS5: 0x00000000,
        CTL6: 0x00000000,
        INT6: 0x00000000,
        TSIZ6: 0x00000000,
        DMA6: 0x00000000,
        TXFSTS6: 0x00000000,
        CTL7: 0x00000000,
        INT7: 0x00000000,
        TSIZ7: 0x00000000,
        DMA7: 0x00000000,
        TXFSTS7: 0x00000000,
        CTL8: 0x00000000,
        INT8: 0x00000000,
        TSIZ8: 0x00000000,
        DMA8: 0x00000000,
        TXFSTS8: 0x00000000,
        CTL: 0x00008000,
        INT: 0x00000080,
        TSIZ: 0x00000000,
        DMA: 0x00000000,
        CTL1: 0x00000000,
        INT1: 0x00000000,
        DMA1: 0x00000000,
        TSIZ1: 0x00000000,
        CTL2: 0x00000000,
        INT2: 0x00000000,
        DMA2: 0x00000000,
        TSIZ2: 0x00000000,
        CTL3: 0x00000000,
        INT3: 0x00000000,
        DMA3: 0x00000000,
        TSIZ3: 0x00000000,
        CTL4: 0x00000000,
        INT4: 0x00000000,
        DMA4: 0x00000000,
        TSIZ4: 0x00000000,
        CTL5: 0x00000000,
        INT5: 0x00000000,
        DMA5: 0x00000000,
        TSIZ5: 0x00000000,
        CTL6: 0x00000000,
        INT6: 0x00000000,
        DMA6: 0x00000000,
        TSIZ6: 0x00000000,
        CTL7: 0x00000000,
        INT7: 0x00000000,
        DMA7: 0x00000000,
        TSIZ7: 0x00000000,
        CTL8: 0x00000000,
        INT8: 0x00000000,
        DMA8: 0x00000000,
        TSIZ8: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut OTG1_HS_DEVICE_TAKEN: bool = false;

    /// Safe access to OTG1_HS_DEVICE
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
            if OTG1_HS_DEVICE_TAKEN {
                None
            } else {
                OTG1_HS_DEVICE_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to OTG1_HS_DEVICE
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if OTG1_HS_DEVICE_TAKEN && inst.addr == INSTANCE.addr {
                OTG1_HS_DEVICE_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal OTG1_HS_DEVICE
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        OTG1_HS_DEVICE_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to OTG1_HS_DEVICE
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const OTG1_HS_DEVICE: *const RegisterBlock = 0x40040800 as *const _;

/// Access functions for the OTG2_HS_DEVICE peripheral instance
pub mod OTG2_HS_DEVICE {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x40080800,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in OTG2_HS_DEVICE
    pub const reset: ResetValues = ResetValues {
        DCFG: 0x02200000,
        DCTL: 0x00000000,
        DSTS: 0x00000010,
        DIEPMSK: 0x00000000,
        DOEPMSK: 0x00000000,
        DAINT: 0x00000000,
        DAINTMSK: 0x00000000,
        DVBUSDIS: 0x000017D7,
        DVBUSPULSE: 0x000005B8,
        DTHRCTL: 0x00000000,
        DIEPEMPMSK: 0x00000000,
        DEACHINT: 0x00000000,
        DEACHINTMSK: 0x00000000,
        DIEPEACHMSK1: 0x00000000,
        DOEPEACHMSK1: 0x00000000,
        CTL: 0x00000000,
        INT: 0x00000080,
        TSIZ: 0x00000000,
        DMA: 0x00000000,
        TXFSTS: 0x00000000,
        CTL1: 0x00000000,
        INT1: 0x00000000,
        TSIZ1: 0x00000000,
        DMA1: 0x00000000,
        TXFSTS1: 0x00000000,
        CTL2: 0x00000000,
        INT2: 0x00000000,
        TSIZ2: 0x00000000,
        DMA2: 0x00000000,
        TXFSTS2: 0x00000000,
        CTL3: 0x00000000,
        INT3: 0x00000000,
        TSIZ3: 0x00000000,
        DMA3: 0x00000000,
        TXFSTS3: 0x00000000,
        CTL4: 0x00000000,
        INT4: 0x00000000,
        TSIZ4: 0x00000000,
        DMA4: 0x00000000,
        TXFSTS4: 0x00000000,
        CTL5: 0x00000000,
        INT5: 0x00000000,
        TSIZ5: 0x00000000,
        DMA5: 0x00000000,
        TXFSTS5: 0x00000000,
        CTL6: 0x00000000,
        INT6: 0x00000000,
        TSIZ6: 0x00000000,
        DMA6: 0x00000000,
        TXFSTS6: 0x00000000,
        CTL7: 0x00000000,
        INT7: 0x00000000,
        TSIZ7: 0x00000000,
        DMA7: 0x00000000,
        TXFSTS7: 0x00000000,
        CTL8: 0x00000000,
        INT8: 0x00000000,
        TSIZ8: 0x00000000,
        DMA8: 0x00000000,
        TXFSTS8: 0x00000000,
        CTL: 0x00008000,
        INT: 0x00000080,
        TSIZ: 0x00000000,
        DMA: 0x00000000,
        CTL1: 0x00000000,
        INT1: 0x00000000,
        DMA1: 0x00000000,
        TSIZ1: 0x00000000,
        CTL2: 0x00000000,
        INT2: 0x00000000,
        DMA2: 0x00000000,
        TSIZ2: 0x00000000,
        CTL3: 0x00000000,
        INT3: 0x00000000,
        DMA3: 0x00000000,
        TSIZ3: 0x00000000,
        CTL4: 0x00000000,
        INT4: 0x00000000,
        DMA4: 0x00000000,
        TSIZ4: 0x00000000,
        CTL5: 0x00000000,
        INT5: 0x00000000,
        DMA5: 0x00000000,
        TSIZ5: 0x00000000,
        CTL6: 0x00000000,
        INT6: 0x00000000,
        DMA6: 0x00000000,
        TSIZ6: 0x00000000,
        CTL7: 0x00000000,
        INT7: 0x00000000,
        DMA7: 0x00000000,
        TSIZ7: 0x00000000,
        CTL8: 0x00000000,
        INT8: 0x00000000,
        DMA8: 0x00000000,
        TSIZ8: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut OTG2_HS_DEVICE_TAKEN: bool = false;

    /// Safe access to OTG2_HS_DEVICE
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
            if OTG2_HS_DEVICE_TAKEN {
                None
            } else {
                OTG2_HS_DEVICE_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to OTG2_HS_DEVICE
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if OTG2_HS_DEVICE_TAKEN && inst.addr == INSTANCE.addr {
                OTG2_HS_DEVICE_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal OTG2_HS_DEVICE
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        OTG2_HS_DEVICE_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to OTG2_HS_DEVICE
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const OTG2_HS_DEVICE: *const RegisterBlock = 0x40080800 as *const _;
