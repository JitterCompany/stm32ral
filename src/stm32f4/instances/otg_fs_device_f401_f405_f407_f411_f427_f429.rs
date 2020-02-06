#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! USB on the go full speed
//!
//! Used by: stm32f401, stm32f405, stm32f407, stm32f411, stm32f427, stm32f429

#[cfg(not(feature = "nosync"))]
pub use crate::stm32f4::peripherals::otg_fs_device_v1::Instance;
pub use crate::stm32f4::peripherals::otg_fs_device_v1::{RegisterBlock, ResetValues};
pub use crate::stm32f4::peripherals::otg_fs_device_v1::{
    DAINT, DAINTMSK, DCFG, DCTL, DIEPCTL0, DIEPCTL1, DIEPCTL2, DIEPCTL3, DIEPEMPMSK, DIEPINT0,
    DIEPINT1, DIEPINT2, DIEPINT3, DIEPMSK, DIEPTSIZ0, DIEPTSIZ1, DIEPTSIZ2, DIEPTSIZ3, DOEPCTL0,
    DOEPCTL1, DOEPCTL2, DOEPCTL3, DOEPINT0, DOEPINT1, DOEPINT2, DOEPINT3, DOEPMSK, DOEPTSIZ0,
    DOEPTSIZ1, DOEPTSIZ2, DOEPTSIZ3, DSTS, DTXFSTS0, DTXFSTS1, DTXFSTS2, DTXFSTS3, DVBUSDIS,
    DVBUSPULSE,
};

/// Access functions for the OTG_FS_DEVICE peripheral instance
pub mod OTG_FS_DEVICE {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x50000800,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in OTG_FS_DEVICE
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
        DIEPEMPMSK: 0x00000000,
        DIEPCTL0: 0x00000000,
        DIEPCTL1: 0x00000000,
        DIEPCTL2: 0x00000000,
        DIEPCTL3: 0x00000000,
        DOEPCTL0: 0x00008000,
        DOEPCTL1: 0x00000000,
        DOEPCTL2: 0x00000000,
        DOEPCTL3: 0x00000000,
        DIEPINT0: 0x00000080,
        DIEPINT1: 0x00000080,
        DIEPINT2: 0x00000080,
        DIEPINT3: 0x00000080,
        DOEPINT0: 0x00000080,
        DOEPINT1: 0x00000080,
        DOEPINT2: 0x00000080,
        DOEPINT3: 0x00000080,
        DIEPTSIZ0: 0x00000000,
        DOEPTSIZ0: 0x00000000,
        DIEPTSIZ1: 0x00000000,
        DIEPTSIZ2: 0x00000000,
        DIEPTSIZ3: 0x00000000,
        DTXFSTS0: 0x00000000,
        DTXFSTS1: 0x00000000,
        DTXFSTS2: 0x00000000,
        DTXFSTS3: 0x00000000,
        DOEPTSIZ1: 0x00000000,
        DOEPTSIZ2: 0x00000000,
        DOEPTSIZ3: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut OTG_FS_DEVICE_TAKEN: bool = false;

    /// Safe access to OTG_FS_DEVICE
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
            if OTG_FS_DEVICE_TAKEN {
                None
            } else {
                OTG_FS_DEVICE_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to OTG_FS_DEVICE
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if OTG_FS_DEVICE_TAKEN && inst.addr == INSTANCE.addr {
                OTG_FS_DEVICE_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal OTG_FS_DEVICE
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        OTG_FS_DEVICE_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to OTG_FS_DEVICE
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const OTG_FS_DEVICE: *const RegisterBlock = 0x50000800 as *const _;
