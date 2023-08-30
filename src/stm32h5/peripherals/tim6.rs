#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Basic timers
//!
//! Used by: stm32h503, stm32h562, stm32h563, stm32h573

use crate::{RWRegister, WORegister};
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// TIM6 control register 1
pub mod TIM6_CR1 {

    /// Counter enable CEN is cleared automatically in one-pulse mode, when an update event occurs.
    pub mod CEN {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Update disable This bit is set and cleared by software to enable/disable UEV event generation. Counter overflow/underflow Setting the UG bit Update generation through the slave mode controller Buffered registers are then loaded with their preload values.
    pub mod UDIS {
        /// Offset (1 bits)
        pub const offset: u16 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Update request source This bit is set and cleared by software to select the UEV event sources. Counter overflow/underflow Setting the UG bit Update generation through the slave mode controller
    pub mod URS {
        /// Offset (2 bits)
        pub const offset: u16 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// One-pulse mode
    pub mod OPM {
        /// Offset (3 bits)
        pub const offset: u16 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Auto-reload preload enable
    pub mod ARPE {
        /// Offset (7 bits)
        pub const offset: u16 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// UIF status bit remapping
    pub mod UIFREMAP {
        /// Offset (11 bits)
        pub const offset: u16 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Dithering enable Note: The DITHEN bit can only be modified when CEN bit is reset.
    pub mod DITHEN {
        /// Offset (12 bits)
        pub const offset: u16 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// TIM6 control register 2
pub mod TIM6_CR2 {

    /// Master mode selection These bits are used to select the information to be sent in master mode to slave timers for synchronization (TRGO). The combination is as follows: Note: The clock of the slave timer or he peripheral receiving the tim_trgo must be enabled prior to receive events from the master timer, and must not be changed on-the-fly while triggers are received from the master timer.
    pub mod MMS {
        /// Offset (4 bits)
        pub const offset: u16 = 4;
        /// Mask (3 bits: 0b111 << 4)
        pub const mask: u16 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// TIM6 DMA/Interrupt enable register
pub mod TIM6_DIER {

    /// Update interrupt enable
    pub mod UIE {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Update DMA request enable
    pub mod UDE {
        /// Offset (8 bits)
        pub const offset: u16 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// TIM6 status register
pub mod TIM6_SR {

    /// Update interrupt flag This bit is set by hardware on an update event. It is cleared by software. On counter overflow if UDIS = 0 in the TIMx_CR1 register. When CNT is reinitialized by software using the UG bit in the TIMx_EGR register, if URS = 0 and UDIS = 0 in the TIMx_CR1 register.
    pub mod UIF {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// TIM6 event generation register
pub mod TIM6_EGR {

    /// Update generation This bit can be set by software, it is automatically cleared by hardware.
    pub mod UG {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// TIM6 counter
pub mod TIM6_CNT {

    /// Counter value Non-dithering mode (DITHEN = 0) The register holds the counter value. Dithering mode (DITHEN = 1) The register only holds the non-dithered part in CNT\[15:0\]. The fractional part is not available.
    pub mod CNT {
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

    /// UIF copy This bit is a read-only copy of the UIF bit of the TIMx_ISR register. If the UIFREMAP bit in TIMx_CR1 is reset, bit 31 is reserved and read as 0.
    pub mod UIFCPY {
        /// Offset (31 bits)
        pub const offset: u32 = 31;
        /// Mask (1 bit: 1 << 31)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// TIM6 prescaler
pub mod TIM6_PSC {

    /// Prescaler value The counter clock frequency ftim_cnt_ck is equal to ftim_psc_ck / (PSC\[15:0\] + 1). PSC contains the value to be loaded into the active prescaler register at each update event. (including when the counter is cleared through UG bit of TIMx_EGR register.
    pub mod PSC {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (16 bits: 0xffff << 0)
        pub const mask: u16 = 0xffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// TIM6 auto-reload register
pub mod TIM6_ARR {

    /// Auto-reload value ARR is the value to be loaded into the actual auto-reload register. Refer to for more details about ARR update and behavior. The counter is blocked while the auto-reload value is null. Non-dithering mode (DITHEN = 0) The register holds the auto-reload value in ARR\[15:0\]. The ARR\[19:16\] bits are reserved. Dithering mode (DITHEN = 1) The register holds the integer part in ARR\[19:4\]. The ARR\[3:0\] bitfield contains the dithered part.
    pub mod ARR {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (20 bits: 0xfffff << 0)
        pub const mask: u32 = 0xfffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}
#[repr(C)]
pub struct RegisterBlock {
    /// TIM6 control register 1
    pub TIM6_CR1: RWRegister<u16>,

    _reserved1: [u8; 2],

    /// TIM6 control register 2
    pub TIM6_CR2: RWRegister<u16>,

    _reserved2: [u8; 6],

    /// TIM6 DMA/Interrupt enable register
    pub TIM6_DIER: RWRegister<u16>,

    _reserved3: [u8; 2],

    /// TIM6 status register
    pub TIM6_SR: RWRegister<u16>,

    _reserved4: [u8; 2],

    /// TIM6 event generation register
    pub TIM6_EGR: WORegister<u16>,

    _reserved5: [u8; 14],

    /// TIM6 counter
    pub TIM6_CNT: RWRegister<u32>,

    /// TIM6 prescaler
    pub TIM6_PSC: RWRegister<u16>,

    _reserved6: [u8; 2],

    /// TIM6 auto-reload register
    pub TIM6_ARR: RWRegister<u32>,
}
pub struct ResetValues {
    pub TIM6_CR1: u16,
    pub TIM6_CR2: u16,
    pub TIM6_DIER: u16,
    pub TIM6_SR: u16,
    pub TIM6_EGR: u16,
    pub TIM6_CNT: u32,
    pub TIM6_PSC: u16,
    pub TIM6_ARR: u32,
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
