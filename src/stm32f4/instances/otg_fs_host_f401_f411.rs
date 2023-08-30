#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! USB on the go full speed
//!
//! Used by: stm32f401, stm32f411

#[cfg(not(feature = "nosync"))]
pub use crate::stm32f4::peripherals::otg_fs_host_v1::Instance;
pub use crate::stm32f4::peripherals::otg_fs_host_v1::{RegisterBlock, ResetValues};
pub use crate::stm32f4::peripherals::otg_fs_host_v1::{
    CHAR0, CHAR1, CHAR2, CHAR3, CHAR4, CHAR5, CHAR6, CHAR7, HAINT, HAINTMSK, HCFG, HFIR, HFNUM,
    HPRT, HPTXSTS, INT0, INT1, INT2, INT3, INT4, INT5, INT6, INT7, INTMSK0, INTMSK1, INTMSK2,
    INTMSK3, INTMSK4, INTMSK5, INTMSK6, INTMSK7, TSIZ0, TSIZ1, TSIZ2, TSIZ3, TSIZ4, TSIZ5, TSIZ6,
    TSIZ7,
};

/// Access functions for the OTG_FS_HOST peripheral instance
pub mod OTG_FS_HOST {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x50000400,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in OTG_FS_HOST
    pub const reset: ResetValues = ResetValues {
        HCFG: 0x00000000,
        HFIR: 0x0000EA60,
        HFNUM: 0x00003FFF,
        HPTXSTS: 0x00080100,
        HAINT: 0x00000000,
        HAINTMSK: 0x00000000,
        HPRT: 0x00000000,
        CHAR0: 0x00000000,
        INT0: 0x00000000,
        INTMSK0: 0x00000000,
        TSIZ0: 0x00000000,
        CHAR1: 0x00000000,
        INT1: 0x00000000,
        INTMSK1: 0x00000000,
        TSIZ1: 0x00000000,
        CHAR2: 0x00000000,
        INT2: 0x00000000,
        INTMSK2: 0x00000000,
        TSIZ2: 0x00000000,
        CHAR3: 0x00000000,
        INT3: 0x00000000,
        INTMSK3: 0x00000000,
        TSIZ3: 0x00000000,
        CHAR4: 0x00000000,
        INT4: 0x00000000,
        INTMSK4: 0x00000000,
        TSIZ4: 0x00000000,
        CHAR5: 0x00000000,
        INT5: 0x00000000,
        INTMSK5: 0x00000000,
        TSIZ5: 0x00000000,
        CHAR6: 0x00000000,
        INT6: 0x00000000,
        INTMSK6: 0x00000000,
        TSIZ6: 0x00000000,
        CHAR7: 0x00000000,
        INT7: 0x00000000,
        INTMSK7: 0x00000000,
        TSIZ7: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut OTG_FS_HOST_TAKEN: bool = false;

    /// Safe access to OTG_FS_HOST
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
            if OTG_FS_HOST_TAKEN {
                None
            } else {
                OTG_FS_HOST_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to OTG_FS_HOST
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if OTG_FS_HOST_TAKEN && inst.addr == INSTANCE.addr {
                OTG_FS_HOST_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal OTG_FS_HOST
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        OTG_FS_HOST_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to OTG_FS_HOST
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const OTG_FS_HOST: *const RegisterBlock = 0x50000400 as *const _;
