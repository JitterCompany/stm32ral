#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Low power timer
//!
//! Used by: stm32h562, stm32h563, stm32h573

#[cfg(not(feature = "nosync"))]
pub use crate::stm32h5::peripherals::lptim::Instance;
pub use crate::stm32h5::peripherals::lptim::{RegisterBlock, ResetValues};
pub use crate::stm32h5::peripherals::lptim::{
    ARR, CCMR1, CCR1, CCR2, CFGR, CFGR2, CNT, CR, DIER, ICR, ISR, RCR,
};

/// Access functions for the LPTIM1 peripheral instance
pub mod LPTIM1 {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x44004400,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in LPTIM1
    pub const reset: ResetValues = ResetValues {
        ISR: 0x00000000,
        ICR: 0x00000000,
        DIER: 0x00000000,
        CFGR: 0x00000000,
        CR: 0x00000000,
        CCR1: 0x00000000,
        ARR: 0x00000001,
        CNT: 0x00000000,
        CFGR2: 0x00000000,
        RCR: 0x00000000,
        CCMR1: 0x00000000,
        CCR2: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut LPTIM1_TAKEN: bool = false;

    /// Safe access to LPTIM1
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
            if LPTIM1_TAKEN {
                None
            } else {
                LPTIM1_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to LPTIM1
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if LPTIM1_TAKEN && inst.addr == INSTANCE.addr {
                LPTIM1_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal LPTIM1
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        LPTIM1_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to LPTIM1
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const LPTIM1: *const RegisterBlock = 0x44004400 as *const _;

/// Access functions for the LPTIM2 peripheral instance
pub mod LPTIM2 {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x40009400,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in LPTIM2
    pub const reset: ResetValues = ResetValues {
        ISR: 0x00000000,
        ICR: 0x00000000,
        DIER: 0x00000000,
        CFGR: 0x00000000,
        CR: 0x00000000,
        CCR1: 0x00000000,
        ARR: 0x00000001,
        CNT: 0x00000000,
        CFGR2: 0x00000000,
        RCR: 0x00000000,
        CCMR1: 0x00000000,
        CCR2: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut LPTIM2_TAKEN: bool = false;

    /// Safe access to LPTIM2
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
            if LPTIM2_TAKEN {
                None
            } else {
                LPTIM2_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to LPTIM2
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if LPTIM2_TAKEN && inst.addr == INSTANCE.addr {
                LPTIM2_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal LPTIM2
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        LPTIM2_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to LPTIM2
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const LPTIM2: *const RegisterBlock = 0x40009400 as *const _;

/// Access functions for the LPTIM3 peripheral instance
pub mod LPTIM3 {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x44004800,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in LPTIM3
    pub const reset: ResetValues = ResetValues {
        ISR: 0x00000000,
        ICR: 0x00000000,
        DIER: 0x00000000,
        CFGR: 0x00000000,
        CR: 0x00000000,
        CCR1: 0x00000000,
        ARR: 0x00000001,
        CNT: 0x00000000,
        CFGR2: 0x00000000,
        RCR: 0x00000000,
        CCMR1: 0x00000000,
        CCR2: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut LPTIM3_TAKEN: bool = false;

    /// Safe access to LPTIM3
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
            if LPTIM3_TAKEN {
                None
            } else {
                LPTIM3_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to LPTIM3
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if LPTIM3_TAKEN && inst.addr == INSTANCE.addr {
                LPTIM3_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal LPTIM3
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        LPTIM3_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to LPTIM3
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const LPTIM3: *const RegisterBlock = 0x44004800 as *const _;

/// Access functions for the LPTIM4 peripheral instance
pub mod LPTIM4 {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x44004c00,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in LPTIM4
    pub const reset: ResetValues = ResetValues {
        ISR: 0x00000000,
        ICR: 0x00000000,
        DIER: 0x00000000,
        CFGR: 0x00000000,
        CR: 0x00000000,
        CCR1: 0x00000000,
        ARR: 0x00000001,
        CNT: 0x00000000,
        CFGR2: 0x00000000,
        RCR: 0x00000000,
        CCMR1: 0x00000000,
        CCR2: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut LPTIM4_TAKEN: bool = false;

    /// Safe access to LPTIM4
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
            if LPTIM4_TAKEN {
                None
            } else {
                LPTIM4_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to LPTIM4
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if LPTIM4_TAKEN && inst.addr == INSTANCE.addr {
                LPTIM4_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal LPTIM4
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        LPTIM4_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to LPTIM4
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const LPTIM4: *const RegisterBlock = 0x44004c00 as *const _;

/// Access functions for the LPTIM5 peripheral instance
pub mod LPTIM5 {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x44005000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in LPTIM5
    pub const reset: ResetValues = ResetValues {
        ISR: 0x00000000,
        ICR: 0x00000000,
        DIER: 0x00000000,
        CFGR: 0x00000000,
        CR: 0x00000000,
        CCR1: 0x00000000,
        ARR: 0x00000001,
        CNT: 0x00000000,
        CFGR2: 0x00000000,
        RCR: 0x00000000,
        CCMR1: 0x00000000,
        CCR2: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut LPTIM5_TAKEN: bool = false;

    /// Safe access to LPTIM5
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
            if LPTIM5_TAKEN {
                None
            } else {
                LPTIM5_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to LPTIM5
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if LPTIM5_TAKEN && inst.addr == INSTANCE.addr {
                LPTIM5_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal LPTIM5
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        LPTIM5_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to LPTIM5
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const LPTIM5: *const RegisterBlock = 0x44005000 as *const _;

/// Access functions for the LPTIM6 peripheral instance
pub mod LPTIM6 {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x44005400,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in LPTIM6
    pub const reset: ResetValues = ResetValues {
        ISR: 0x00000000,
        ICR: 0x00000000,
        DIER: 0x00000000,
        CFGR: 0x00000000,
        CR: 0x00000000,
        CCR1: 0x00000000,
        ARR: 0x00000001,
        CNT: 0x00000000,
        CFGR2: 0x00000000,
        RCR: 0x00000000,
        CCMR1: 0x00000000,
        CCR2: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut LPTIM6_TAKEN: bool = false;

    /// Safe access to LPTIM6
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
            if LPTIM6_TAKEN {
                None
            } else {
                LPTIM6_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to LPTIM6
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if LPTIM6_TAKEN && inst.addr == INSTANCE.addr {
                LPTIM6_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal LPTIM6
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        LPTIM6_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to LPTIM6
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const LPTIM6: *const RegisterBlock = 0x44005400 as *const _;

/// Access functions for the SEC_LPTIM1 peripheral instance
pub mod SEC_LPTIM1 {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x54004400,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in SEC_LPTIM1
    pub const reset: ResetValues = ResetValues {
        ISR: 0x00000000,
        ICR: 0x00000000,
        DIER: 0x00000000,
        CFGR: 0x00000000,
        CR: 0x00000000,
        CCR1: 0x00000000,
        ARR: 0x00000001,
        CNT: 0x00000000,
        CFGR2: 0x00000000,
        RCR: 0x00000000,
        CCMR1: 0x00000000,
        CCR2: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut SEC_LPTIM1_TAKEN: bool = false;

    /// Safe access to SEC_LPTIM1
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
            if SEC_LPTIM1_TAKEN {
                None
            } else {
                SEC_LPTIM1_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to SEC_LPTIM1
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if SEC_LPTIM1_TAKEN && inst.addr == INSTANCE.addr {
                SEC_LPTIM1_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal SEC_LPTIM1
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        SEC_LPTIM1_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to SEC_LPTIM1
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const SEC_LPTIM1: *const RegisterBlock = 0x54004400 as *const _;

/// Access functions for the SEC_LPTIM2 peripheral instance
pub mod SEC_LPTIM2 {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x50009400,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in SEC_LPTIM2
    pub const reset: ResetValues = ResetValues {
        ISR: 0x00000000,
        ICR: 0x00000000,
        DIER: 0x00000000,
        CFGR: 0x00000000,
        CR: 0x00000000,
        CCR1: 0x00000000,
        ARR: 0x00000001,
        CNT: 0x00000000,
        CFGR2: 0x00000000,
        RCR: 0x00000000,
        CCMR1: 0x00000000,
        CCR2: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut SEC_LPTIM2_TAKEN: bool = false;

    /// Safe access to SEC_LPTIM2
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
            if SEC_LPTIM2_TAKEN {
                None
            } else {
                SEC_LPTIM2_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to SEC_LPTIM2
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if SEC_LPTIM2_TAKEN && inst.addr == INSTANCE.addr {
                SEC_LPTIM2_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal SEC_LPTIM2
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        SEC_LPTIM2_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to SEC_LPTIM2
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const SEC_LPTIM2: *const RegisterBlock = 0x50009400 as *const _;

/// Access functions for the SEC_LPTIM3 peripheral instance
pub mod SEC_LPTIM3 {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x54004800,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in SEC_LPTIM3
    pub const reset: ResetValues = ResetValues {
        ISR: 0x00000000,
        ICR: 0x00000000,
        DIER: 0x00000000,
        CFGR: 0x00000000,
        CR: 0x00000000,
        CCR1: 0x00000000,
        ARR: 0x00000001,
        CNT: 0x00000000,
        CFGR2: 0x00000000,
        RCR: 0x00000000,
        CCMR1: 0x00000000,
        CCR2: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut SEC_LPTIM3_TAKEN: bool = false;

    /// Safe access to SEC_LPTIM3
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
            if SEC_LPTIM3_TAKEN {
                None
            } else {
                SEC_LPTIM3_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to SEC_LPTIM3
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if SEC_LPTIM3_TAKEN && inst.addr == INSTANCE.addr {
                SEC_LPTIM3_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal SEC_LPTIM3
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        SEC_LPTIM3_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to SEC_LPTIM3
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const SEC_LPTIM3: *const RegisterBlock = 0x54004800 as *const _;

/// Access functions for the SEC_LPTIM4 peripheral instance
pub mod SEC_LPTIM4 {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x54004c00,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in SEC_LPTIM4
    pub const reset: ResetValues = ResetValues {
        ISR: 0x00000000,
        ICR: 0x00000000,
        DIER: 0x00000000,
        CFGR: 0x00000000,
        CR: 0x00000000,
        CCR1: 0x00000000,
        ARR: 0x00000001,
        CNT: 0x00000000,
        CFGR2: 0x00000000,
        RCR: 0x00000000,
        CCMR1: 0x00000000,
        CCR2: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut SEC_LPTIM4_TAKEN: bool = false;

    /// Safe access to SEC_LPTIM4
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
            if SEC_LPTIM4_TAKEN {
                None
            } else {
                SEC_LPTIM4_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to SEC_LPTIM4
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if SEC_LPTIM4_TAKEN && inst.addr == INSTANCE.addr {
                SEC_LPTIM4_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal SEC_LPTIM4
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        SEC_LPTIM4_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to SEC_LPTIM4
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const SEC_LPTIM4: *const RegisterBlock = 0x54004c00 as *const _;

/// Access functions for the SEC_LPTIM5 peripheral instance
pub mod SEC_LPTIM5 {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x54005000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in SEC_LPTIM5
    pub const reset: ResetValues = ResetValues {
        ISR: 0x00000000,
        ICR: 0x00000000,
        DIER: 0x00000000,
        CFGR: 0x00000000,
        CR: 0x00000000,
        CCR1: 0x00000000,
        ARR: 0x00000001,
        CNT: 0x00000000,
        CFGR2: 0x00000000,
        RCR: 0x00000000,
        CCMR1: 0x00000000,
        CCR2: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut SEC_LPTIM5_TAKEN: bool = false;

    /// Safe access to SEC_LPTIM5
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
            if SEC_LPTIM5_TAKEN {
                None
            } else {
                SEC_LPTIM5_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to SEC_LPTIM5
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if SEC_LPTIM5_TAKEN && inst.addr == INSTANCE.addr {
                SEC_LPTIM5_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal SEC_LPTIM5
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        SEC_LPTIM5_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to SEC_LPTIM5
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const SEC_LPTIM5: *const RegisterBlock = 0x54005000 as *const _;

/// Access functions for the SEC_LPTIM6 peripheral instance
pub mod SEC_LPTIM6 {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x54005400,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in SEC_LPTIM6
    pub const reset: ResetValues = ResetValues {
        ISR: 0x00000000,
        ICR: 0x00000000,
        DIER: 0x00000000,
        CFGR: 0x00000000,
        CR: 0x00000000,
        CCR1: 0x00000000,
        ARR: 0x00000001,
        CNT: 0x00000000,
        CFGR2: 0x00000000,
        RCR: 0x00000000,
        CCMR1: 0x00000000,
        CCR2: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut SEC_LPTIM6_TAKEN: bool = false;

    /// Safe access to SEC_LPTIM6
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
            if SEC_LPTIM6_TAKEN {
                None
            } else {
                SEC_LPTIM6_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to SEC_LPTIM6
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if SEC_LPTIM6_TAKEN && inst.addr == INSTANCE.addr {
                SEC_LPTIM6_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal SEC_LPTIM6
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        SEC_LPTIM6_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to SEC_LPTIM6
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const SEC_LPTIM6: *const RegisterBlock = 0x54005400 as *const _;
