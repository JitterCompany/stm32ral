#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! USB on the go full speed
//!
//! Used by: stm32f215, stm32f217

#[cfg(not(feature = "nosync"))]
pub use crate::stm32f2::peripherals::otg_fs_device::Instance;
pub use crate::stm32f2::peripherals::otg_fs_device::{RegisterBlock, ResetValues};
pub use crate::stm32f2::peripherals::otg_fs_device::{
    CTL, CTL, CTL1, CTL1, CTL2, CTL2, CTL3, CTL3, DAINT, DAINTMSK, DCFG, DCTL, DIEPEMPMSK, DIEPMSK,
    DOEPMSK, DSTS, DVBUSDIS, DVBUSPULSE, INT, INT, INT1, INT1, INT2, INT2, INT3, INT3, TSIZ, TSIZ,
    TSIZ1, TSIZ1, TSIZ2, TSIZ2, TSIZ3, TSIZ3, TXFSTS, TXFSTS1, TXFSTS2, TXFSTS3,
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
        CTL: 0x00000000,
        INT: 0x00000080,
        TSIZ: 0x00000000,
        TXFSTS: 0x00000000,
        CTL1: 0x00000000,
        INT1: 0x00000080,
        TSIZ1: 0x00000000,
        TXFSTS1: 0x00000000,
        CTL2: 0x00000000,
        INT2: 0x00000080,
        TSIZ2: 0x00000000,
        TXFSTS2: 0x00000000,
        CTL3: 0x00000000,
        INT3: 0x00000080,
        TSIZ3: 0x00000000,
        TXFSTS3: 0x00000000,
        CTL: 0x00008000,
        INT: 0x00000080,
        TSIZ: 0x00000000,
        CTL1: 0x00000000,
        INT1: 0x00000080,
        TSIZ1: 0x00000000,
        CTL2: 0x00000000,
        INT2: 0x00000080,
        TSIZ2: 0x00000000,
        CTL3: 0x00000000,
        INT3: 0x00000080,
        TSIZ3: 0x00000000,
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
