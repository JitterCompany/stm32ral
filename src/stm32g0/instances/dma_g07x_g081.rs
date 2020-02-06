#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! DMA controller
//!
//! Used by: stm32g07x, stm32g081

#[cfg(not(feature = "nosync"))]
pub use crate::stm32g0::peripherals::dma_v2::Instance;
pub use crate::stm32g0::peripherals::dma_v2::{RegisterBlock, ResetValues};
pub use crate::stm32g0::peripherals::dma_v2::{
    CCR1, CCR2, CCR3, CCR4, CCR5, CCR6, CCR7, CMAR1, CMAR2, CMAR3, CMAR4, CMAR5, CMAR6, CMAR7,
    CNDTR1, CNDTR2, CNDTR3, CNDTR4, CNDTR5, CNDTR6, CNDTR7, CPAR1, CPAR2, CPAR3, CPAR4, CPAR5,
    CPAR6, CPAR7, IFCR, ISR,
};

/// Access functions for the DMA peripheral instance
pub mod DMA {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x40020000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in DMA
    pub const reset: ResetValues = ResetValues {
        ISR: 0x00000000,
        IFCR: 0x00000000,
        CCR1: 0x00000000,
        CCR2: 0x00000000,
        CCR3: 0x00000000,
        CCR4: 0x00000000,
        CCR5: 0x00000000,
        CCR6: 0x00000000,
        CCR7: 0x00000000,
        CNDTR1: 0x00000000,
        CNDTR2: 0x00000000,
        CNDTR3: 0x00000000,
        CNDTR4: 0x00000000,
        CNDTR5: 0x00000000,
        CNDTR6: 0x00000000,
        CNDTR7: 0x00000000,
        CPAR1: 0x00000000,
        CPAR2: 0x00000000,
        CPAR3: 0x00000000,
        CPAR4: 0x00000000,
        CPAR5: 0x00000000,
        CPAR6: 0x00000000,
        CPAR7: 0x00000000,
        CMAR1: 0x00000000,
        CMAR2: 0x00000000,
        CMAR3: 0x00000000,
        CMAR4: 0x00000000,
        CMAR5: 0x00000000,
        CMAR6: 0x00000000,
        CMAR7: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut DMA_TAKEN: bool = false;

    /// Safe access to DMA
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
            if DMA_TAKEN {
                None
            } else {
                DMA_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to DMA
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if DMA_TAKEN && inst.addr == INSTANCE.addr {
                DMA_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal DMA
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        DMA_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to DMA
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const DMA: *const RegisterBlock = 0x40020000 as *const _;
