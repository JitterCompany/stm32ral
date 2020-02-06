#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Debug support
//!
//! Used by: stm32l0x1, stm32l0x2, stm32l0x3

use crate::{RORegister, RWRegister};
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// MCU Device ID Code Register
pub mod IDCODE {

    /// Device Identifier
    pub mod DEV_ID {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (12 bits: 0xfff << 0)
        pub const mask: u32 = 0xfff << offset;
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
}

/// APB Low Freeze Register
pub mod APB1_FZ {

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

            /// 0b1: I2C3 SMBUS timeout is frozen
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

    /// LPTIM1 counter stopped when core is halted
    pub mod DBG_LPTIMER_STOP {
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

            /// 0b0: LPTIM1 counter clock is fed even if the core is halted
            pub const Continue: u32 = 0b0;

            /// 0b1: LPTIM1 counter clock is stopped when the core is halted
            pub const Stop: u32 = 0b1;
        }
    }
}

/// APB High Freeze Register
pub mod APB2_FZ {

    /// Debug Timer 21 stopped when Core is halted
    pub mod DBG_TIMER21_STOP {
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

            /// 0b0: The counter clock of TIMx is fed even if the core is halted
            pub const Continue: u32 = 0b0;

            /// 0b1: The counter clock of TIMx is stopped when the core is halted
            pub const Stop: u32 = 0b1;
        }
    }

    /// Debug Timer 22 stopped when Core is halted
    pub mod DBG_TIMER22_STO {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}
pub struct RegisterBlock {
    /// MCU Device ID Code Register
    pub IDCODE: RORegister<u32>,

    /// Debug MCU Configuration Register
    pub CR: RWRegister<u32>,

    /// APB Low Freeze Register
    pub APB1_FZ: RWRegister<u32>,

    /// APB High Freeze Register
    pub APB2_FZ: RWRegister<u32>,
}
pub struct ResetValues {
    pub IDCODE: u32,
    pub CR: u32,
    pub APB1_FZ: u32,
    pub APB2_FZ: u32,
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
#[cfg(feature = "rtfm")]
unsafe impl Send for Instance {}
