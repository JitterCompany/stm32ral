#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Universal synchronous asynchronous receiver transmitter
//!
//! Used by: stm32g07x, stm32g081

#[cfg(not(feature = "nosync"))]
pub use crate::stm32g0::peripherals::lpuart_v2::Instance;
pub use crate::stm32g0::peripherals::lpuart_v2::{RegisterBlock, ResetValues};
pub use crate::stm32g0::peripherals::lpuart_v2::{
    BRR, CR1, CR2, CR3, HWCFGR1, HWCFGR2, ICR, IPIDR, ISR, PRESC, RDR, RQR, SIDR, TDR, VERR,
};

/// Access functions for the LPUART peripheral instance
pub mod LPUART {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x40008000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in LPUART
    pub const reset: ResetValues = ResetValues {
        CR1: 0x00000000,
        CR2: 0x00000000,
        CR3: 0x00000000,
        BRR: 0x00000000,
        RQR: 0x00000000,
        ISR: 0x000000C0,
        ICR: 0x00000000,
        RDR: 0x00000000,
        TDR: 0x00000000,
        PRESC: 0x00000000,
        HWCFGR2: 0x00000013,
        HWCFGR1: 0x31100000,
        VERR: 0x00000023,
        IPIDR: 0x00130003,
        SIDR: 0xA3C5DD01,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut LPUART_TAKEN: bool = false;

    /// Safe access to LPUART
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
            if LPUART_TAKEN {
                None
            } else {
                LPUART_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to LPUART
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if LPUART_TAKEN && inst.addr == INSTANCE.addr {
                LPUART_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal LPUART
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        LPUART_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to LPUART
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const LPUART: *const RegisterBlock = 0x40008000 as *const _;
