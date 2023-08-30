#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! HASH register bank
//!
//! Used by: stm32h562, stm32h563, stm32h573

#[cfg(not(feature = "nosync"))]
pub use crate::stm32h5::peripherals::hash::Instance;
pub use crate::stm32h5::peripherals::hash::{RegisterBlock, ResetValues};
pub use crate::stm32h5::peripherals::hash::{
    CR, CSR0, CSR1, CSR10, CSR100, CSR101, CSR102, CSR11, CSR12, CSR13, CSR14, CSR15, CSR16, CSR17,
    CSR18, CSR19, CSR2, CSR20, CSR21, CSR22, CSR23, CSR24, CSR25, CSR26, CSR27, CSR28, CSR29, CSR3,
    CSR30, CSR31, CSR32, CSR33, CSR34, CSR35, CSR36, CSR37, CSR38, CSR39, CSR4, CSR40, CSR41,
    CSR42, CSR43, CSR44, CSR45, CSR46, CSR47, CSR48, CSR49, CSR5, CSR50, CSR51, CSR52, CSR53,
    CSR54, CSR55, CSR56, CSR57, CSR58, CSR59, CSR6, CSR60, CSR61, CSR62, CSR63, CSR64, CSR65,
    CSR66, CSR67, CSR68, CSR69, CSR7, CSR70, CSR71, CSR72, CSR73, CSR74, CSR75, CSR76, CSR77,
    CSR78, CSR79, CSR8, CSR80, CSR81, CSR82, CSR83, CSR84, CSR85, CSR86, CSR87, CSR88, CSR89, CSR9,
    CSR90, CSR91, CSR92, CSR93, CSR94, CSR95, CSR96, CSR97, CSR98, CSR99, DIN, HR0, HR1, HR10,
    HR11, HR12, HR13, HR14, HR15, HR2, HR3, HR4, HR5, HR6, HR7, HR8, HR9, HRA0, HRA1, HRA2, HRA3,
    HRA4, IMR, SR, STR,
};

/// Access functions for the HASH peripheral instance
pub mod HASH {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x420c0400,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in HASH
    pub const reset: ResetValues = ResetValues {
        CR: 0x00000000,
        DIN: 0x00000000,
        STR: 0x00000000,
        HRA0: 0x00000000,
        HRA1: 0x00000000,
        HRA2: 0x00000000,
        HRA3: 0x00000000,
        HRA4: 0x00000000,
        IMR: 0x00000000,
        SR: 0x00110001,
        CSR0: 0x00220002,
        CSR1: 0x00220002,
        CSR2: 0x00220002,
        CSR3: 0x00220002,
        CSR4: 0x00220002,
        CSR5: 0x00220002,
        CSR6: 0x00220002,
        CSR7: 0x00220002,
        CSR8: 0x00220002,
        CSR9: 0x00220002,
        CSR10: 0x00220002,
        CSR11: 0x00220002,
        CSR12: 0x00220002,
        CSR13: 0x00220002,
        CSR14: 0x00220002,
        CSR15: 0x00220002,
        CSR16: 0x00220002,
        CSR17: 0x00220002,
        CSR18: 0x00220002,
        CSR19: 0x00220002,
        CSR20: 0x00220002,
        CSR21: 0x00220002,
        CSR22: 0x00220002,
        CSR23: 0x00220002,
        CSR24: 0x00220002,
        CSR25: 0x00220002,
        CSR26: 0x00220002,
        CSR27: 0x00220002,
        CSR28: 0x00220002,
        CSR29: 0x00220002,
        CSR30: 0x00220002,
        CSR31: 0x00220002,
        CSR32: 0x00220002,
        CSR33: 0x00220002,
        CSR34: 0x00220002,
        CSR35: 0x00220002,
        CSR36: 0x00220002,
        CSR37: 0x00220002,
        CSR38: 0x00220002,
        CSR39: 0x00220002,
        CSR40: 0x00220002,
        CSR41: 0x00220002,
        CSR42: 0x00220002,
        CSR43: 0x00220002,
        CSR44: 0x00220002,
        CSR45: 0x00220002,
        CSR46: 0x00220002,
        CSR47: 0x00220002,
        CSR48: 0x00220002,
        CSR49: 0x00220002,
        CSR50: 0x00220002,
        CSR51: 0x00220002,
        CSR52: 0x00220002,
        CSR53: 0x00220002,
        CSR54: 0x00220002,
        CSR55: 0x00220002,
        CSR56: 0x00220002,
        CSR57: 0x00220002,
        CSR58: 0x00220002,
        CSR59: 0x00220002,
        CSR60: 0x00220002,
        CSR61: 0x00220002,
        CSR62: 0x00220002,
        CSR63: 0x00220002,
        CSR64: 0x00220002,
        CSR65: 0x00220002,
        CSR66: 0x00220002,
        CSR67: 0x00220002,
        CSR68: 0x00220002,
        CSR69: 0x00220002,
        CSR70: 0x00220002,
        CSR71: 0x00220002,
        CSR72: 0x00220002,
        CSR73: 0x00220002,
        CSR74: 0x00220002,
        CSR75: 0x00220002,
        CSR76: 0x00220002,
        CSR77: 0x00220002,
        CSR78: 0x00220002,
        CSR79: 0x00220002,
        CSR80: 0x00220002,
        CSR81: 0x00220002,
        CSR82: 0x00220002,
        CSR83: 0x00220002,
        CSR84: 0x00220002,
        CSR85: 0x00220002,
        CSR86: 0x00220002,
        CSR87: 0x00220002,
        CSR88: 0x00220002,
        CSR89: 0x00220002,
        CSR90: 0x00220002,
        CSR91: 0x00220002,
        CSR92: 0x00220002,
        CSR93: 0x00220002,
        CSR94: 0x00220002,
        CSR95: 0x00220002,
        CSR96: 0x00220002,
        CSR97: 0x00220002,
        CSR98: 0x00220002,
        CSR99: 0x00220002,
        CSR100: 0x00220002,
        CSR101: 0x00220002,
        CSR102: 0x00220002,
        HR0: 0x00000000,
        HR1: 0x00000000,
        HR2: 0x00000000,
        HR3: 0x00000000,
        HR4: 0x00000000,
        HR5: 0x00000000,
        HR6: 0x00000000,
        HR7: 0x00000000,
        HR8: 0x00000000,
        HR9: 0x00000000,
        HR10: 0x00000000,
        HR11: 0x00000000,
        HR12: 0x00000000,
        HR13: 0x00000000,
        HR14: 0x00000000,
        HR15: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut HASH_TAKEN: bool = false;

    /// Safe access to HASH
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
            if HASH_TAKEN {
                None
            } else {
                HASH_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to HASH
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if HASH_TAKEN && inst.addr == INSTANCE.addr {
                HASH_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal HASH
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        HASH_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to HASH
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const HASH: *const RegisterBlock = 0x420c0400 as *const _;

/// Access functions for the SEC_HASH peripheral instance
pub mod SEC_HASH {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x520c0400,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in SEC_HASH
    pub const reset: ResetValues = ResetValues {
        CR: 0x00000000,
        DIN: 0x00000000,
        STR: 0x00000000,
        HRA0: 0x00000000,
        HRA1: 0x00000000,
        HRA2: 0x00000000,
        HRA3: 0x00000000,
        HRA4: 0x00000000,
        IMR: 0x00000000,
        SR: 0x00110001,
        CSR0: 0x00220002,
        CSR1: 0x00220002,
        CSR2: 0x00220002,
        CSR3: 0x00220002,
        CSR4: 0x00220002,
        CSR5: 0x00220002,
        CSR6: 0x00220002,
        CSR7: 0x00220002,
        CSR8: 0x00220002,
        CSR9: 0x00220002,
        CSR10: 0x00220002,
        CSR11: 0x00220002,
        CSR12: 0x00220002,
        CSR13: 0x00220002,
        CSR14: 0x00220002,
        CSR15: 0x00220002,
        CSR16: 0x00220002,
        CSR17: 0x00220002,
        CSR18: 0x00220002,
        CSR19: 0x00220002,
        CSR20: 0x00220002,
        CSR21: 0x00220002,
        CSR22: 0x00220002,
        CSR23: 0x00220002,
        CSR24: 0x00220002,
        CSR25: 0x00220002,
        CSR26: 0x00220002,
        CSR27: 0x00220002,
        CSR28: 0x00220002,
        CSR29: 0x00220002,
        CSR30: 0x00220002,
        CSR31: 0x00220002,
        CSR32: 0x00220002,
        CSR33: 0x00220002,
        CSR34: 0x00220002,
        CSR35: 0x00220002,
        CSR36: 0x00220002,
        CSR37: 0x00220002,
        CSR38: 0x00220002,
        CSR39: 0x00220002,
        CSR40: 0x00220002,
        CSR41: 0x00220002,
        CSR42: 0x00220002,
        CSR43: 0x00220002,
        CSR44: 0x00220002,
        CSR45: 0x00220002,
        CSR46: 0x00220002,
        CSR47: 0x00220002,
        CSR48: 0x00220002,
        CSR49: 0x00220002,
        CSR50: 0x00220002,
        CSR51: 0x00220002,
        CSR52: 0x00220002,
        CSR53: 0x00220002,
        CSR54: 0x00220002,
        CSR55: 0x00220002,
        CSR56: 0x00220002,
        CSR57: 0x00220002,
        CSR58: 0x00220002,
        CSR59: 0x00220002,
        CSR60: 0x00220002,
        CSR61: 0x00220002,
        CSR62: 0x00220002,
        CSR63: 0x00220002,
        CSR64: 0x00220002,
        CSR65: 0x00220002,
        CSR66: 0x00220002,
        CSR67: 0x00220002,
        CSR68: 0x00220002,
        CSR69: 0x00220002,
        CSR70: 0x00220002,
        CSR71: 0x00220002,
        CSR72: 0x00220002,
        CSR73: 0x00220002,
        CSR74: 0x00220002,
        CSR75: 0x00220002,
        CSR76: 0x00220002,
        CSR77: 0x00220002,
        CSR78: 0x00220002,
        CSR79: 0x00220002,
        CSR80: 0x00220002,
        CSR81: 0x00220002,
        CSR82: 0x00220002,
        CSR83: 0x00220002,
        CSR84: 0x00220002,
        CSR85: 0x00220002,
        CSR86: 0x00220002,
        CSR87: 0x00220002,
        CSR88: 0x00220002,
        CSR89: 0x00220002,
        CSR90: 0x00220002,
        CSR91: 0x00220002,
        CSR92: 0x00220002,
        CSR93: 0x00220002,
        CSR94: 0x00220002,
        CSR95: 0x00220002,
        CSR96: 0x00220002,
        CSR97: 0x00220002,
        CSR98: 0x00220002,
        CSR99: 0x00220002,
        CSR100: 0x00220002,
        CSR101: 0x00220002,
        CSR102: 0x00220002,
        HR0: 0x00000000,
        HR1: 0x00000000,
        HR2: 0x00000000,
        HR3: 0x00000000,
        HR4: 0x00000000,
        HR5: 0x00000000,
        HR6: 0x00000000,
        HR7: 0x00000000,
        HR8: 0x00000000,
        HR9: 0x00000000,
        HR10: 0x00000000,
        HR11: 0x00000000,
        HR12: 0x00000000,
        HR13: 0x00000000,
        HR14: 0x00000000,
        HR15: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut SEC_HASH_TAKEN: bool = false;

    /// Safe access to SEC_HASH
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
            if SEC_HASH_TAKEN {
                None
            } else {
                SEC_HASH_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to SEC_HASH
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if SEC_HASH_TAKEN && inst.addr == INSTANCE.addr {
                SEC_HASH_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal SEC_HASH
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        SEC_HASH_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to SEC_HASH
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const SEC_HASH: *const RegisterBlock = 0x520c0400 as *const _;
