#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Universal synchronous asynchronous receiver transmitter
//!
//! Used by: stm32l412, stm32l4x1, stm32l4x2

#[cfg(not(feature = "nosync"))]
pub use crate::stm32l4::peripherals::usart3::Instance;
pub use crate::stm32l4::peripherals::usart3::{RegisterBlock, ResetValues};
pub use crate::stm32l4::peripherals::usart3::{
    BRR, CR1, CR2, CR3, GTPR, ICR, ISR, RDR, RQR, RTOR, TDR,
};

/// Access functions for the USART3 peripheral instance
pub mod USART3 {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x40004800,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in USART3
    pub const reset: ResetValues = ResetValues {
        CR1: 0x00000000,
        CR2: 0x00000000,
        CR3: 0x00000000,
        BRR: 0x00000000,
        GTPR: 0x00000000,
        RTOR: 0x00000000,
        RQR: 0x00000000,
        ISR: 0x000000C0,
        ICR: 0x00000000,
        RDR: 0x00000000,
        TDR: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut USART3_TAKEN: bool = false;

    /// Safe access to USART3
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
            if USART3_TAKEN {
                None
            } else {
                USART3_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to USART3
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if USART3_TAKEN && inst.addr == INSTANCE.addr {
                USART3_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal USART3
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        USART3_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to USART3
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const USART3: *const RegisterBlock = 0x40004800 as *const _;
