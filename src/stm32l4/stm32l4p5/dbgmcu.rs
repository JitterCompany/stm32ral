#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Debug support

use crate::{RORegister, RWRegister};
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// MCU Device ID Code Register
pub mod IDCODE {

    /// Device Identifier
    pub mod DEV_ID {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (16 bits: 0xffff << 0)
        pub const mask: u32 = 0xffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Revision Identifier
    pub mod REV_ID {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (16 bits: 0xffff << 16)
        pub const mask: u32 = 0xffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Debug MCU Configuration Register
pub mod CR {

    /// Debug Sleep Mode
    pub mod DBG_SLEEP {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Debug Sleep Mode Disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Debug Sleep Mode Enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Debug Stop Mode
    pub mod DBG_STOP {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Debug Stop Mode Disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Debug Stop Mode Enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Debug Standby Mode
    pub mod DBG_STANDBY {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Debug Standby Mode Disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Debug Standby Mode Enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Trace pin assignment control
    pub mod TRACE_IOEN {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Trace pins not assigned (default state)
            pub const Disabled: u32 = 0b0;

            /// 0b1: Trace pins assigned
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Trace pin assignment control
    pub mod TRACE_MODE {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (2 bits: 0b11 << 6)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: TRACE pin assignment for Asynchronous Mode
            pub const Asynchronous: u32 = 0b00;

            /// 0b01: TRACE pin assignment for Synchronous Mode with a TRACEDATA size of 1
            pub const Size1: u32 = 0b01;

            /// 0b10: TRACE pin assignment for Synchronous Mode with a TRACEDATA size of 2
            pub const Size2: u32 = 0b10;

            /// 0b11: TRACE pin assignment for Synchronous Mode with a TRACEDATA size of 4
            pub const Size4: u32 = 0b11;
        }
    }
}

/// APB Low Freeze Register 1
pub mod APB1FZR1 {

    /// Debug Timer 2 stopped when Core is halted
    pub mod DBG_TIMER2_STOP {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: The counter clock of TIMx is fed even if the core is halted
            pub const Continue: u32 = 0b0;

            /// 0b1: The counter clock of TIMx is stopped when the core is halted
            pub const Stop: u32 = 0b1;
        }
    }

    /// TIM3 counter stopped when core is halted
    pub mod DBG_TIM3_STOP {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::DBG_TIMER2_STOP::RW;
    }

    /// TIM4 counter stopped when core is halted
    pub mod DBG_TIM4_STOP {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::DBG_TIMER2_STOP::RW;
    }

    /// TIM5 counter stopped when core is halted
    pub mod DBG_TIM5_STOP {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::DBG_TIMER2_STOP::RW;
    }

    /// Debug Timer 6 stopped when Core is halted
    pub mod DBG_TIMER6_STOP {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::DBG_TIMER2_STOP::RW;
    }

    /// TIM7 counter stopped when core is halted
    pub mod DBG_TIM7_STOP {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::DBG_TIMER2_STOP::RW;
    }

    /// Debug RTC stopped when Core is halted
    pub mod DBG_RTC_STOP {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: The clock of the RTC counter is fed even if the core is halted
            pub const Continue: u32 = 0b0;

            /// 0b1: The clock of the RTC counter is stopped when the core is halted
            pub const Stop: u32 = 0b1;
        }
    }

    /// Debug Window Wachdog stopped when Core is halted
    pub mod DBG_WWDG_STOP {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: The window watchdog counter clock continues even if the core is halted
            pub const Continue: u32 = 0b0;

            /// 0b1: The window watchdog counter clock is stopped when the core is halted
            pub const Stop: u32 = 0b1;
        }
    }

    /// Debug Independent Wachdog stopped when Core is halted
    pub mod DBG_IWDG_STOP {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: The independent watchdog counter clock continues even if the core is halted
            pub const Continue: u32 = 0b0;

            /// 0b1: The independent watchdog counter clock is stopped when the core is halted
            pub const Stop: u32 = 0b1;
        }
    }

    /// I2C1 SMBUS timeout mode stopped when core is halted
    pub mod DBG_I2C1_STOP {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (1 bit: 1 << 21)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Same behavior as in normal mode
            pub const NormalMode: u32 = 0b0;

            /// 0b1: I2Cx SMBUS timeout is frozen
            pub const SMBusTimeoutFrozen: u32 = 0b1;
        }
    }

    /// I2C2 SMBUS timeout mode stopped when core is halted
    pub mod DBG_I2C2_STOP {
        /// Offset (22 bits)
        pub const offset: u32 = 22;
        /// Mask (1 bit: 1 << 22)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::DBG_I2C1_STOP::RW;
    }

    /// I2C3 SMBUS timeout counter stopped when core is halted
    pub mod DBG_I2C3_STOP {
        /// Offset (23 bits)
        pub const offset: u32 = 23;
        /// Mask (1 bit: 1 << 23)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::DBG_I2C1_STOP::RW;
    }

    /// bxCAN stopped when core is halted
    pub mod DBG_CAN1_STOP {
        /// Offset (25 bits)
        pub const offset: u32 = 25;
        /// Mask (1 bit: 1 << 25)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Same behavior as in normal mode
            pub const NormalMode: u32 = 0b0;

            /// 0b1: The bxCAN1 receive registers are frozen
            pub const ReceiveRegistersFrozen: u32 = 0b1;
        }
    }

    /// LPTIM1 counter stopped when core is halted
    pub mod DBG_LPTIM1_STOP {
        /// Offset (31 bits)
        pub const offset: u32 = 31;
        /// Mask (1 bit: 1 << 31)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: LPTIMx counter clock is fed even if the core is halted
            pub const Continue: u32 = 0b0;

            /// 0b1: LPTIMx counter clock is stopped when the core is halted
            pub const Stop: u32 = 0b1;
        }
    }
}

/// APB Low Freeze Register 2
pub mod APB1FZR2 {

    /// LPTIM2 counter stopped when core is halted
    pub mod DBG_LPTIM2_STOP {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: LPTIMx counter clock is fed even if the core is halted
            pub const Continue: u32 = 0b0;

            /// 0b1: LPTIMx counter clock is stopped when the core is halted
            pub const Stop: u32 = 0b1;
        }
    }

    /// I2C4 SMBUS timeout counter stopped when core is halted
    pub mod DBG_I2C4_STOP {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Same behavior as in normal mode
            pub const NormalMode: u32 = 0b0;

            /// 0b1: I2Cx SMBUS timeout is frozen
            pub const SMBusTimeoutFrozen: u32 = 0b1;
        }
    }
}

/// APB High Freeze Register
pub mod APB2FZR {

    /// TIM1 counter stopped when core is halted
    pub mod DBG_TIM1_STOP {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: The counter clock of TIMx is fed even if the core is halted
            pub const Continue: u32 = 0b0;

            /// 0b1: The counter clock of TIMx is stopped when the core is halted
            pub const Stop: u32 = 0b1;
        }
    }

    /// TIM8 counter stopped when core is halted
    pub mod DBG_TIM8_STOP {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::DBG_TIM1_STOP::RW;
    }

    /// TIM15 counter stopped when core is halted
    pub mod DBG_TIM15_STOP {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::DBG_TIM1_STOP::RW;
    }

    /// TIM16 counter stopped when core is halted
    pub mod DBG_TIM16_STOP {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::DBG_TIM1_STOP::RW;
    }

    /// TIM17 counter stopped when core is halted
    pub mod DBG_TIM17_STOP {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (1 bit: 1 << 18)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::DBG_TIM1_STOP::RW;
    }
}
#[repr(C)]
pub struct RegisterBlock {
    /// MCU Device ID Code Register
    pub IDCODE: RORegister<u32>,

    /// Debug MCU Configuration Register
    pub CR: RWRegister<u32>,

    /// APB Low Freeze Register 1
    pub APB1FZR1: RWRegister<u32>,

    /// APB Low Freeze Register 2
    pub APB1FZR2: RWRegister<u32>,

    /// APB High Freeze Register
    pub APB2FZR: RWRegister<u32>,
}
pub struct ResetValues {
    pub IDCODE: u32,
    pub CR: u32,
    pub APB1FZR1: u32,
    pub APB1FZR2: u32,
    pub APB2FZR: u32,
}
#[cfg(not(feature = "nosync"))]
pub struct Instance {
    pub(crate) addr: u32,
    pub(crate) _marker: PhantomData<*const RegisterBlock>,
}
#[cfg(not(feature = "nosync"))]
impl ::core::ops::Deref for Instance {
    type Target = RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &RegisterBlock {
        unsafe { &*(self.addr as *const _) }
    }
}
#[cfg(feature = "rtic")]
unsafe impl Send for Instance {}

/// Access functions for the DBGMCU peripheral instance
pub mod DBGMCU {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0xe0042000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in DBGMCU
    pub const reset: ResetValues = ResetValues {
        IDCODE: 0x00000000,
        CR: 0x00000000,
        APB1FZR1: 0x00000000,
        APB1FZR2: 0x00000000,
        APB2FZR: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut DBGMCU_TAKEN: bool = false;

    /// Safe access to DBGMCU
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
            if DBGMCU_TAKEN {
                None
            } else {
                DBGMCU_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to DBGMCU
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if DBGMCU_TAKEN && inst.addr == INSTANCE.addr {
                DBGMCU_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal DBGMCU
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        DBGMCU_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to DBGMCU
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const DBGMCU: *const RegisterBlock = 0xe0042000 as *const _;
