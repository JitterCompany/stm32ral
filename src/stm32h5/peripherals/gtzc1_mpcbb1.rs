#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! GTZC1_MPCBB1
//!
//! Used by: stm32h562, stm32h563, stm32h573

use crate::RWRegister;
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// GTZC1 SRAM1 MPCBB control register
pub mod GTZC1_MPCBB1_CR {

    /// lock the control register of the MPCBB until next reset This bit is cleared by default and once set, it can not be reset until system reset.
    pub mod GLOCK {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// SRAMx clocks security state This bit is used to define the internal SRAMs clocks control in RCC as secure or not.
    pub mod INVSECSTATE {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (1 bit: 1 << 30)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// secure read/write illegal access disable This bit disables the detection of an illegal access when a secure read/write transaction access a non-secure blocks of the block-based SRAM (secure fetch on non-secure block is always considered illegal).
    pub mod SRWILADIS {
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

/// GTZC1 SRAM1 MPCBB configuration lock register 1
pub mod GTZC1_MPCBB1_CFGLOCK1 {

    /// Security/privilege configuration lock for super-block This bit is set by software and can be cleared only by system reset.
    pub mod SPLCK0 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Security/privilege configuration lock for super-block This bit is set by software and can be cleared only by system reset.
    pub mod SPLCK1 {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Security/privilege configuration lock for super-block This bit is set by software and can be cleared only by system reset.
    pub mod SPLCK2 {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Security/privilege configuration lock for super-block This bit is set by software and can be cleared only by system reset.
    pub mod SPLCK3 {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Security/privilege configuration lock for super-block This bit is set by software and can be cleared only by system reset.
    pub mod SPLCK4 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Security/privilege configuration lock for super-block This bit is set by software and can be cleared only by system reset.
    pub mod SPLCK5 {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Security/privilege configuration lock for super-block This bit is set by software and can be cleared only by system reset.
    pub mod SPLCK6 {
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

    /// Security/privilege configuration lock for super-block This bit is set by software and can be cleared only by system reset.
    pub mod SPLCK7 {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Security/privilege configuration lock for super-block This bit is set by software and can be cleared only by system reset.
    pub mod SPLCK8 {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Security/privilege configuration lock for super-block This bit is set by software and can be cleared only by system reset.
    pub mod SPLCK9 {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Security/privilege configuration lock for super-block This bit is set by software and can be cleared only by system reset.
    pub mod SPLCK10 {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Security/privilege configuration lock for super-block This bit is set by software and can be cleared only by system reset.
    pub mod SPLCK11 {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Security/privilege configuration lock for super-block This bit is set by software and can be cleared only by system reset.
    pub mod SPLCK12 {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Security/privilege configuration lock for super-block This bit is set by software and can be cleared only by system reset.
    pub mod SPLCK13 {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Security/privilege configuration lock for super-block This bit is set by software and can be cleared only by system reset.
    pub mod SPLCK14 {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Security/privilege configuration lock for super-block This bit is set by software and can be cleared only by system reset.
    pub mod SPLCK15 {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Security/privilege configuration lock for super-block This bit is set by software and can be cleared only by system reset.
    pub mod SPLCK16 {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Security/privilege configuration lock for super-block This bit is set by software and can be cleared only by system reset.
    pub mod SPLCK17 {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Security/privilege configuration lock for super-block This bit is set by software and can be cleared only by system reset.
    pub mod SPLCK18 {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (1 bit: 1 << 18)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Security/privilege configuration lock for super-block This bit is set by software and can be cleared only by system reset.
    pub mod SPLCK19 {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (1 bit: 1 << 19)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Security/privilege configuration lock for super-block This bit is set by software and can be cleared only by system reset.
    pub mod SPLCK20 {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (1 bit: 1 << 20)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Security/privilege configuration lock for super-block This bit is set by software and can be cleared only by system reset.
    pub mod SPLCK21 {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (1 bit: 1 << 21)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Security/privilege configuration lock for super-block This bit is set by software and can be cleared only by system reset.
    pub mod SPLCK22 {
        /// Offset (22 bits)
        pub const offset: u32 = 22;
        /// Mask (1 bit: 1 << 22)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Security/privilege configuration lock for super-block This bit is set by software and can be cleared only by system reset.
    pub mod SPLCK23 {
        /// Offset (23 bits)
        pub const offset: u32 = 23;
        /// Mask (1 bit: 1 << 23)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Security/privilege configuration lock for super-block This bit is set by software and can be cleared only by system reset.
    pub mod SPLCK24 {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (1 bit: 1 << 24)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Security/privilege configuration lock for super-block This bit is set by software and can be cleared only by system reset.
    pub mod SPLCK25 {
        /// Offset (25 bits)
        pub const offset: u32 = 25;
        /// Mask (1 bit: 1 << 25)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Security/privilege configuration lock for super-block This bit is set by software and can be cleared only by system reset.
    pub mod SPLCK26 {
        /// Offset (26 bits)
        pub const offset: u32 = 26;
        /// Mask (1 bit: 1 << 26)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Security/privilege configuration lock for super-block This bit is set by software and can be cleared only by system reset.
    pub mod SPLCK27 {
        /// Offset (27 bits)
        pub const offset: u32 = 27;
        /// Mask (1 bit: 1 << 27)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Security/privilege configuration lock for super-block This bit is set by software and can be cleared only by system reset.
    pub mod SPLCK28 {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (1 bit: 1 << 28)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Security/privilege configuration lock for super-block This bit is set by software and can be cleared only by system reset.
    pub mod SPLCK29 {
        /// Offset (29 bits)
        pub const offset: u32 = 29;
        /// Mask (1 bit: 1 << 29)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Security/privilege configuration lock for super-block This bit is set by software and can be cleared only by system reset.
    pub mod SPLCK30 {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (1 bit: 1 << 30)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Security/privilege configuration lock for super-block This bit is set by software and can be cleared only by system reset.
    pub mod SPLCK31 {
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

/// GTZC1 SRAM1 MPCBB security configuration for super-block 0 register
pub mod GTZC1_MPCBB1_SECCFGR0 {

    /// Security configuration for block y Unprivileged write to this bit is ignored if PRIVy bit is set in GTZC1_MPCBBz_PRIVCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK.
    pub mod SEC0 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Security configuration for block y Unprivileged write to this bit is ignored if PRIVy bit is set in GTZC1_MPCBBz_PRIVCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK.
    pub mod SEC1 {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Security configuration for block y Unprivileged write to this bit is ignored if PRIVy bit is set in GTZC1_MPCBBz_PRIVCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK.
    pub mod SEC2 {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Security configuration for block y Unprivileged write to this bit is ignored if PRIVy bit is set in GTZC1_MPCBBz_PRIVCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK.
    pub mod SEC3 {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Security configuration for block y Unprivileged write to this bit is ignored if PRIVy bit is set in GTZC1_MPCBBz_PRIVCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK.
    pub mod SEC4 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Security configuration for block y Unprivileged write to this bit is ignored if PRIVy bit is set in GTZC1_MPCBBz_PRIVCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK.
    pub mod SEC5 {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Security configuration for block y Unprivileged write to this bit is ignored if PRIVy bit is set in GTZC1_MPCBBz_PRIVCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK.
    pub mod SEC6 {
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

    /// Security configuration for block y Unprivileged write to this bit is ignored if PRIVy bit is set in GTZC1_MPCBBz_PRIVCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK.
    pub mod SEC7 {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Security configuration for block y Unprivileged write to this bit is ignored if PRIVy bit is set in GTZC1_MPCBBz_PRIVCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK.
    pub mod SEC8 {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Security configuration for block y Unprivileged write to this bit is ignored if PRIVy bit is set in GTZC1_MPCBBz_PRIVCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK.
    pub mod SEC9 {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Security configuration for block y Unprivileged write to this bit is ignored if PRIVy bit is set in GTZC1_MPCBBz_PRIVCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK.
    pub mod SEC10 {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Security configuration for block y Unprivileged write to this bit is ignored if PRIVy bit is set in GTZC1_MPCBBz_PRIVCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK.
    pub mod SEC11 {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Security configuration for block y Unprivileged write to this bit is ignored if PRIVy bit is set in GTZC1_MPCBBz_PRIVCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK.
    pub mod SEC12 {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Security configuration for block y Unprivileged write to this bit is ignored if PRIVy bit is set in GTZC1_MPCBBz_PRIVCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK.
    pub mod SEC13 {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Security configuration for block y Unprivileged write to this bit is ignored if PRIVy bit is set in GTZC1_MPCBBz_PRIVCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK.
    pub mod SEC14 {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Security configuration for block y Unprivileged write to this bit is ignored if PRIVy bit is set in GTZC1_MPCBBz_PRIVCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK.
    pub mod SEC15 {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Security configuration for block y Unprivileged write to this bit is ignored if PRIVy bit is set in GTZC1_MPCBBz_PRIVCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK.
    pub mod SEC16 {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Security configuration for block y Unprivileged write to this bit is ignored if PRIVy bit is set in GTZC1_MPCBBz_PRIVCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK.
    pub mod SEC17 {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Security configuration for block y Unprivileged write to this bit is ignored if PRIVy bit is set in GTZC1_MPCBBz_PRIVCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK.
    pub mod SEC18 {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (1 bit: 1 << 18)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Security configuration for block y Unprivileged write to this bit is ignored if PRIVy bit is set in GTZC1_MPCBBz_PRIVCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK.
    pub mod SEC19 {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (1 bit: 1 << 19)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Security configuration for block y Unprivileged write to this bit is ignored if PRIVy bit is set in GTZC1_MPCBBz_PRIVCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK.
    pub mod SEC20 {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (1 bit: 1 << 20)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Security configuration for block y Unprivileged write to this bit is ignored if PRIVy bit is set in GTZC1_MPCBBz_PRIVCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK.
    pub mod SEC21 {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (1 bit: 1 << 21)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Security configuration for block y Unprivileged write to this bit is ignored if PRIVy bit is set in GTZC1_MPCBBz_PRIVCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK.
    pub mod SEC22 {
        /// Offset (22 bits)
        pub const offset: u32 = 22;
        /// Mask (1 bit: 1 << 22)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Security configuration for block y Unprivileged write to this bit is ignored if PRIVy bit is set in GTZC1_MPCBBz_PRIVCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK.
    pub mod SEC23 {
        /// Offset (23 bits)
        pub const offset: u32 = 23;
        /// Mask (1 bit: 1 << 23)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Security configuration for block y Unprivileged write to this bit is ignored if PRIVy bit is set in GTZC1_MPCBBz_PRIVCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK.
    pub mod SEC24 {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (1 bit: 1 << 24)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Security configuration for block y Unprivileged write to this bit is ignored if PRIVy bit is set in GTZC1_MPCBBz_PRIVCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK.
    pub mod SEC25 {
        /// Offset (25 bits)
        pub const offset: u32 = 25;
        /// Mask (1 bit: 1 << 25)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Security configuration for block y Unprivileged write to this bit is ignored if PRIVy bit is set in GTZC1_MPCBBz_PRIVCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK.
    pub mod SEC26 {
        /// Offset (26 bits)
        pub const offset: u32 = 26;
        /// Mask (1 bit: 1 << 26)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Security configuration for block y Unprivileged write to this bit is ignored if PRIVy bit is set in GTZC1_MPCBBz_PRIVCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK.
    pub mod SEC27 {
        /// Offset (27 bits)
        pub const offset: u32 = 27;
        /// Mask (1 bit: 1 << 27)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Security configuration for block y Unprivileged write to this bit is ignored if PRIVy bit is set in GTZC1_MPCBBz_PRIVCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK.
    pub mod SEC28 {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (1 bit: 1 << 28)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Security configuration for block y Unprivileged write to this bit is ignored if PRIVy bit is set in GTZC1_MPCBBz_PRIVCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK.
    pub mod SEC29 {
        /// Offset (29 bits)
        pub const offset: u32 = 29;
        /// Mask (1 bit: 1 << 29)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Security configuration for block y Unprivileged write to this bit is ignored if PRIVy bit is set in GTZC1_MPCBBz_PRIVCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK.
    pub mod SEC30 {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (1 bit: 1 << 30)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Security configuration for block y Unprivileged write to this bit is ignored if PRIVy bit is set in GTZC1_MPCBBz_PRIVCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK.
    pub mod SEC31 {
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

/// GTZC1 SRAM1 MPCBB security configuration for super-block 1 register
pub mod GTZC1_MPCBB1_SECCFGR1 {
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC0;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC1;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC10;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC11;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC12;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC13;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC14;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC15;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC16;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC17;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC18;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC19;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC2;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC20;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC21;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC22;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC23;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC24;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC25;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC26;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC27;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC28;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC29;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC3;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC30;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC31;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC4;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC5;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC6;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC7;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC8;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC9;
}

/// GTZC1 SRAM1 MPCBB security configuration for super-block 2 register
pub mod GTZC1_MPCBB1_SECCFGR2 {
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC0;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC1;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC10;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC11;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC12;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC13;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC14;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC15;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC16;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC17;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC18;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC19;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC2;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC20;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC21;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC22;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC23;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC24;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC25;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC26;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC27;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC28;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC29;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC3;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC30;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC31;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC4;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC5;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC6;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC7;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC8;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC9;
}

/// GTZC1 SRAM1 MPCBB security configuration for super-block 3 register
pub mod GTZC1_MPCBB1_SECCFGR3 {
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC0;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC1;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC10;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC11;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC12;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC13;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC14;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC15;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC16;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC17;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC18;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC19;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC2;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC20;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC21;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC22;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC23;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC24;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC25;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC26;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC27;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC28;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC29;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC3;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC30;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC31;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC4;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC5;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC6;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC7;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC8;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC9;
}

/// GTZC1 SRAM1 MPCBB security configuration for super-block 4 register
pub mod GTZC1_MPCBB1_SECCFGR4 {
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC0;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC1;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC10;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC11;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC12;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC13;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC14;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC15;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC16;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC17;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC18;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC19;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC2;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC20;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC21;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC22;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC23;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC24;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC25;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC26;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC27;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC28;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC29;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC3;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC30;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC31;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC4;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC5;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC6;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC7;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC8;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC9;
}

/// GTZC1 SRAM1 MPCBB security configuration for super-block 5 register
pub mod GTZC1_MPCBB1_SECCFGR5 {
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC0;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC1;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC10;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC11;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC12;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC13;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC14;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC15;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC16;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC17;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC18;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC19;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC2;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC20;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC21;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC22;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC23;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC24;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC25;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC26;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC27;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC28;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC29;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC3;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC30;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC31;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC4;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC5;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC6;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC7;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC8;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC9;
}

/// GTZC1 SRAM1 MPCBB security configuration for super-block 6 register
pub mod GTZC1_MPCBB1_SECCFGR6 {
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC0;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC1;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC10;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC11;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC12;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC13;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC14;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC15;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC16;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC17;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC18;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC19;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC2;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC20;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC21;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC22;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC23;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC24;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC25;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC26;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC27;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC28;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC29;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC3;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC30;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC31;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC4;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC5;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC6;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC7;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC8;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC9;
}

/// GTZC1 SRAM1 MPCBB security configuration for super-block 7 register
pub mod GTZC1_MPCBB1_SECCFGR7 {
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC0;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC1;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC10;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC11;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC12;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC13;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC14;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC15;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC16;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC17;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC18;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC19;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC2;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC20;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC21;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC22;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC23;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC24;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC25;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC26;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC27;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC28;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC29;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC3;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC30;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC31;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC4;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC5;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC6;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC7;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC8;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC9;
}

/// GTZC1 SRAM1 MPCBB security configuration for super-block 8 register
pub mod GTZC1_MPCBB1_SECCFGR8 {
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC0;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC1;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC10;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC11;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC12;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC13;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC14;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC15;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC16;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC17;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC18;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC19;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC2;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC20;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC21;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC22;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC23;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC24;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC25;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC26;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC27;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC28;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC29;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC3;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC30;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC31;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC4;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC5;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC6;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC7;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC8;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC9;
}

/// GTZC1 SRAM1 MPCBB security configuration for super-block 9 register
pub mod GTZC1_MPCBB1_SECCFGR9 {
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC0;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC1;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC10;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC11;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC12;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC13;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC14;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC15;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC16;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC17;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC18;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC19;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC2;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC20;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC21;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC22;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC23;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC24;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC25;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC26;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC27;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC28;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC29;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC3;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC30;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC31;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC4;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC5;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC6;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC7;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC8;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC9;
}

/// GTZC1 SRAM1 MPCBB security configuration for super-block 10 register
pub mod GTZC1_MPCBB1_SECCFGR10 {
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC0;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC1;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC10;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC11;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC12;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC13;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC14;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC15;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC16;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC17;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC18;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC19;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC2;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC20;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC21;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC22;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC23;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC24;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC25;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC26;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC27;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC28;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC29;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC3;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC30;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC31;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC4;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC5;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC6;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC7;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC8;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC9;
}

/// GTZC1 SRAM1 MPCBB security configuration for super-block 11 register
pub mod GTZC1_MPCBB1_SECCFGR11 {
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC0;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC1;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC10;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC11;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC12;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC13;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC14;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC15;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC16;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC17;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC18;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC19;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC2;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC20;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC21;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC22;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC23;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC24;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC25;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC26;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC27;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC28;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC29;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC3;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC30;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC31;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC4;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC5;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC6;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC7;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC8;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC9;
}

/// GTZC1 SRAM1 MPCBB security configuration for super-block 12 register
pub mod GTZC1_MPCBB1_SECCFGR12 {
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC0;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC1;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC10;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC11;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC12;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC13;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC14;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC15;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC16;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC17;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC18;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC19;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC2;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC20;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC21;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC22;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC23;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC24;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC25;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC26;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC27;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC28;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC29;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC3;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC30;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC31;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC4;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC5;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC6;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC7;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC8;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC9;
}

/// GTZC1 SRAM1 MPCBB security configuration for super-block 13 register
pub mod GTZC1_MPCBB1_SECCFGR13 {
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC0;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC1;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC10;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC11;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC12;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC13;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC14;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC15;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC16;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC17;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC18;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC19;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC2;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC20;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC21;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC22;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC23;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC24;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC25;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC26;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC27;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC28;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC29;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC3;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC30;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC31;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC4;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC5;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC6;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC7;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC8;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC9;
}

/// GTZC1 SRAM1 MPCBB security configuration for super-block 14 register
pub mod GTZC1_MPCBB1_SECCFGR14 {
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC0;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC1;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC10;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC11;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC12;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC13;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC14;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC15;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC16;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC17;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC18;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC19;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC2;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC20;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC21;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC22;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC23;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC24;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC25;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC26;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC27;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC28;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC29;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC3;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC30;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC31;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC4;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC5;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC6;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC7;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC8;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC9;
}

/// GTZC1 SRAM1 MPCBB security configuration for super-block 15 register
pub mod GTZC1_MPCBB1_SECCFGR15 {
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC0;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC1;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC10;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC11;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC12;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC13;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC14;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC15;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC16;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC17;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC18;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC19;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC2;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC20;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC21;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC22;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC23;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC24;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC25;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC26;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC27;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC28;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC29;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC3;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC30;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC31;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC4;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC5;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC6;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC7;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC8;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC9;
}

/// GTZC1 SRAM1 MPCBB security configuration for super-block 16 register
pub mod GTZC1_MPCBB1_SECCFGR16 {
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC0;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC1;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC10;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC11;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC12;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC13;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC14;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC15;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC16;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC17;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC18;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC19;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC2;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC20;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC21;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC22;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC23;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC24;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC25;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC26;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC27;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC28;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC29;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC3;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC30;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC31;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC4;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC5;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC6;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC7;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC8;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC9;
}

/// GTZC1 SRAM1 MPCBB security configuration for super-block 17 register
pub mod GTZC1_MPCBB1_SECCFGR17 {
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC0;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC1;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC10;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC11;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC12;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC13;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC14;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC15;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC16;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC17;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC18;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC19;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC2;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC20;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC21;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC22;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC23;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC24;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC25;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC26;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC27;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC28;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC29;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC3;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC30;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC31;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC4;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC5;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC6;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC7;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC8;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC9;
}

/// GTZC1 SRAM1 MPCBB security configuration for super-block 18 register
pub mod GTZC1_MPCBB1_SECCFGR18 {
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC0;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC1;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC10;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC11;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC12;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC13;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC14;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC15;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC16;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC17;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC18;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC19;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC2;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC20;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC21;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC22;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC23;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC24;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC25;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC26;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC27;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC28;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC29;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC3;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC30;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC31;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC4;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC5;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC6;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC7;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC8;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC9;
}

/// GTZC1 SRAM1 MPCBB security configuration for super-block 19 register
pub mod GTZC1_MPCBB1_SECCFGR19 {
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC0;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC1;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC10;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC11;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC12;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC13;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC14;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC15;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC16;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC17;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC18;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC19;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC2;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC20;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC21;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC22;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC23;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC24;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC25;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC26;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC27;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC28;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC29;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC3;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC30;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC31;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC4;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC5;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC6;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC7;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC8;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC9;
}

/// GTZC1 SRAM1 MPCBB security configuration for super-block 20 register
pub mod GTZC1_MPCBB1_SECCFGR20 {
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC0;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC1;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC10;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC11;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC12;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC13;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC14;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC15;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC16;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC17;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC18;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC19;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC2;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC20;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC21;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC22;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC23;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC24;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC25;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC26;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC27;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC28;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC29;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC3;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC30;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC31;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC4;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC5;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC6;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC7;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC8;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC9;
}

/// GTZC1 SRAM1 MPCBB security configuration for super-block 21 register
pub mod GTZC1_MPCBB1_SECCFGR21 {
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC0;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC1;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC10;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC11;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC12;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC13;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC14;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC15;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC16;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC17;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC18;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC19;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC2;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC20;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC21;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC22;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC23;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC24;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC25;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC26;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC27;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC28;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC29;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC3;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC30;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC31;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC4;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC5;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC6;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC7;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC8;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC9;
}

/// GTZC1 SRAM1 MPCBB security configuration for super-block 22 register
pub mod GTZC1_MPCBB1_SECCFGR22 {
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC0;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC1;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC10;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC11;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC12;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC13;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC14;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC15;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC16;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC17;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC18;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC19;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC2;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC20;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC21;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC22;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC23;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC24;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC25;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC26;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC27;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC28;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC29;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC3;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC30;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC31;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC4;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC5;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC6;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC7;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC8;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC9;
}

/// GTZC1 SRAM1 MPCBB security configuration for super-block 23 register
pub mod GTZC1_MPCBB1_SECCFGR23 {
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC0;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC1;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC10;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC11;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC12;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC13;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC14;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC15;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC16;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC17;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC18;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC19;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC2;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC20;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC21;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC22;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC23;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC24;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC25;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC26;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC27;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC28;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC29;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC3;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC30;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC31;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC4;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC5;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC6;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC7;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC8;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC9;
}

/// GTZC1 SRAM1 MPCBB security configuration for super-block 24 register
pub mod GTZC1_MPCBB1_SECCFGR24 {
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC0;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC1;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC10;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC11;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC12;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC13;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC14;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC15;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC16;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC17;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC18;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC19;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC2;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC20;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC21;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC22;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC23;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC24;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC25;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC26;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC27;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC28;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC29;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC3;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC30;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC31;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC4;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC5;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC6;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC7;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC8;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC9;
}

/// GTZC1 SRAM1 MPCBB security configuration for super-block 25 register
pub mod GTZC1_MPCBB1_SECCFGR25 {
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC0;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC1;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC10;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC11;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC12;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC13;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC14;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC15;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC16;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC17;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC18;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC19;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC2;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC20;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC21;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC22;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC23;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC24;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC25;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC26;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC27;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC28;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC29;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC3;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC30;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC31;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC4;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC5;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC6;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC7;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC8;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC9;
}

/// GTZC1 SRAM1 MPCBB security configuration for super-block 26 register
pub mod GTZC1_MPCBB1_SECCFGR26 {
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC0;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC1;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC10;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC11;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC12;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC13;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC14;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC15;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC16;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC17;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC18;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC19;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC2;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC20;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC21;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC22;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC23;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC24;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC25;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC26;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC27;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC28;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC29;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC3;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC30;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC31;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC4;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC5;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC6;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC7;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC8;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC9;
}

/// GTZC1 SRAM1 MPCBB security configuration for super-block 27 register
pub mod GTZC1_MPCBB1_SECCFGR27 {
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC0;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC1;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC10;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC11;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC12;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC13;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC14;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC15;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC16;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC17;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC18;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC19;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC2;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC20;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC21;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC22;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC23;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC24;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC25;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC26;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC27;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC28;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC29;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC3;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC30;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC31;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC4;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC5;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC6;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC7;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC8;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC9;
}

/// GTZC1 SRAM1 MPCBB security configuration for super-block 28 register
pub mod GTZC1_MPCBB1_SECCFGR28 {
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC0;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC1;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC10;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC11;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC12;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC13;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC14;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC15;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC16;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC17;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC18;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC19;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC2;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC20;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC21;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC22;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC23;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC24;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC25;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC26;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC27;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC28;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC29;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC3;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC30;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC31;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC4;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC5;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC6;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC7;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC8;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC9;
}

/// GTZC1 SRAM1 MPCBB security configuration for super-block 29 register
pub mod GTZC1_MPCBB1_SECCFGR29 {
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC0;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC1;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC10;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC11;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC12;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC13;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC14;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC15;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC16;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC17;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC18;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC19;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC2;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC20;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC21;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC22;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC23;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC24;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC25;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC26;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC27;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC28;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC29;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC3;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC30;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC31;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC4;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC5;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC6;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC7;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC8;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC9;
}

/// GTZC1 SRAM1 MPCBB security configuration for super-block 30 register
pub mod GTZC1_MPCBB1_SECCFGR30 {
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC0;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC1;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC10;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC11;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC12;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC13;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC14;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC15;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC16;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC17;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC18;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC19;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC2;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC20;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC21;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC22;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC23;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC24;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC25;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC26;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC27;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC28;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC29;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC3;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC30;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC31;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC4;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC5;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC6;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC7;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC8;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC9;
}

/// GTZC1 SRAM1 MPCBB security configuration for super-block 31 register
pub mod GTZC1_MPCBB1_SECCFGR31 {
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC0;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC1;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC10;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC11;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC12;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC13;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC14;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC15;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC16;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC17;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC18;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC19;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC2;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC20;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC21;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC22;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC23;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC24;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC25;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC26;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC27;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC28;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC29;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC3;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC30;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC31;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC4;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC5;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC6;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC7;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC8;
    pub use super::GTZC1_MPCBB1_SECCFGR0::SEC9;
}

/// GTZC1 SRAM1 MPCBB privileged configuration for super-block 0 register
pub mod GTZC1_MPCBB1_PRIVCFGR0 {

    /// Privileged configuration for block y, belonging to super-block x (y = 31 to 0). Non-secure write to this bit is ignored if SECy bit is set in GTZC1_MPCBBz_SECCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK.
    pub mod PRIV0 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Privileged configuration for block y, belonging to super-block x (y = 31 to 0). Non-secure write to this bit is ignored if SECy bit is set in GTZC1_MPCBBz_SECCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK.
    pub mod PRIV1 {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Privileged configuration for block y, belonging to super-block x (y = 31 to 0). Non-secure write to this bit is ignored if SECy bit is set in GTZC1_MPCBBz_SECCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK.
    pub mod PRIV2 {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Privileged configuration for block y, belonging to super-block x (y = 31 to 0). Non-secure write to this bit is ignored if SECy bit is set in GTZC1_MPCBBz_SECCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK.
    pub mod PRIV3 {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Privileged configuration for block y, belonging to super-block x (y = 31 to 0). Non-secure write to this bit is ignored if SECy bit is set in GTZC1_MPCBBz_SECCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK.
    pub mod PRIV4 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Privileged configuration for block y, belonging to super-block x (y = 31 to 0). Non-secure write to this bit is ignored if SECy bit is set in GTZC1_MPCBBz_SECCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK.
    pub mod PRIV5 {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Privileged configuration for block y, belonging to super-block x (y = 31 to 0). Non-secure write to this bit is ignored if SECy bit is set in GTZC1_MPCBBz_SECCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK.
    pub mod PRIV6 {
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

    /// Privileged configuration for block y, belonging to super-block x (y = 31 to 0). Non-secure write to this bit is ignored if SECy bit is set in GTZC1_MPCBBz_SECCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK.
    pub mod PRIV7 {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Privileged configuration for block y, belonging to super-block x (y = 31 to 0). Non-secure write to this bit is ignored if SECy bit is set in GTZC1_MPCBBz_SECCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK.
    pub mod PRIV8 {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Privileged configuration for block y, belonging to super-block x (y = 31 to 0). Non-secure write to this bit is ignored if SECy bit is set in GTZC1_MPCBBz_SECCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK.
    pub mod PRIV9 {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Privileged configuration for block y, belonging to super-block x (y = 31 to 0). Non-secure write to this bit is ignored if SECy bit is set in GTZC1_MPCBBz_SECCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK.
    pub mod PRIV10 {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Privileged configuration for block y, belonging to super-block x (y = 31 to 0). Non-secure write to this bit is ignored if SECy bit is set in GTZC1_MPCBBz_SECCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK.
    pub mod PRIV11 {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Privileged configuration for block y, belonging to super-block x (y = 31 to 0). Non-secure write to this bit is ignored if SECy bit is set in GTZC1_MPCBBz_SECCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK.
    pub mod PRIV12 {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Privileged configuration for block y, belonging to super-block x (y = 31 to 0). Non-secure write to this bit is ignored if SECy bit is set in GTZC1_MPCBBz_SECCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK.
    pub mod PRIV13 {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Privileged configuration for block y, belonging to super-block x (y = 31 to 0). Non-secure write to this bit is ignored if SECy bit is set in GTZC1_MPCBBz_SECCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK.
    pub mod PRIV14 {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Privileged configuration for block y, belonging to super-block x (y = 31 to 0). Non-secure write to this bit is ignored if SECy bit is set in GTZC1_MPCBBz_SECCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK.
    pub mod PRIV15 {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Privileged configuration for block y, belonging to super-block x (y = 31 to 0). Non-secure write to this bit is ignored if SECy bit is set in GTZC1_MPCBBz_SECCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK.
    pub mod PRIV16 {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Privileged configuration for block y, belonging to super-block x (y = 31 to 0). Non-secure write to this bit is ignored if SECy bit is set in GTZC1_MPCBBz_SECCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK.
    pub mod PRIV17 {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Privileged configuration for block y, belonging to super-block x (y = 31 to 0). Non-secure write to this bit is ignored if SECy bit is set in GTZC1_MPCBBz_SECCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK.
    pub mod PRIV18 {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (1 bit: 1 << 18)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Privileged configuration for block y, belonging to super-block x (y = 31 to 0). Non-secure write to this bit is ignored if SECy bit is set in GTZC1_MPCBBz_SECCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK.
    pub mod PRIV19 {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (1 bit: 1 << 19)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Privileged configuration for block y, belonging to super-block x (y = 31 to 0). Non-secure write to this bit is ignored if SECy bit is set in GTZC1_MPCBBz_SECCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK.
    pub mod PRIV20 {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (1 bit: 1 << 20)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Privileged configuration for block y, belonging to super-block x (y = 31 to 0). Non-secure write to this bit is ignored if SECy bit is set in GTZC1_MPCBBz_SECCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK.
    pub mod PRIV21 {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (1 bit: 1 << 21)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Privileged configuration for block y, belonging to super-block x (y = 31 to 0). Non-secure write to this bit is ignored if SECy bit is set in GTZC1_MPCBBz_SECCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK.
    pub mod PRIV22 {
        /// Offset (22 bits)
        pub const offset: u32 = 22;
        /// Mask (1 bit: 1 << 22)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Privileged configuration for block y, belonging to super-block x (y = 31 to 0). Non-secure write to this bit is ignored if SECy bit is set in GTZC1_MPCBBz_SECCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK.
    pub mod PRIV23 {
        /// Offset (23 bits)
        pub const offset: u32 = 23;
        /// Mask (1 bit: 1 << 23)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Privileged configuration for block y, belonging to super-block x (y = 31 to 0). Non-secure write to this bit is ignored if SECy bit is set in GTZC1_MPCBBz_SECCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK.
    pub mod PRIV24 {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (1 bit: 1 << 24)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Privileged configuration for block y, belonging to super-block x (y = 31 to 0). Non-secure write to this bit is ignored if SECy bit is set in GTZC1_MPCBBz_SECCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK.
    pub mod PRIV25 {
        /// Offset (25 bits)
        pub const offset: u32 = 25;
        /// Mask (1 bit: 1 << 25)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Privileged configuration for block y, belonging to super-block x (y = 31 to 0). Non-secure write to this bit is ignored if SECy bit is set in GTZC1_MPCBBz_SECCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK.
    pub mod PRIV26 {
        /// Offset (26 bits)
        pub const offset: u32 = 26;
        /// Mask (1 bit: 1 << 26)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Privileged configuration for block y, belonging to super-block x (y = 31 to 0). Non-secure write to this bit is ignored if SECy bit is set in GTZC1_MPCBBz_SECCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK.
    pub mod PRIV27 {
        /// Offset (27 bits)
        pub const offset: u32 = 27;
        /// Mask (1 bit: 1 << 27)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Privileged configuration for block y, belonging to super-block x (y = 31 to 0). Non-secure write to this bit is ignored if SECy bit is set in GTZC1_MPCBBz_SECCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK.
    pub mod PRIV28 {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (1 bit: 1 << 28)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Privileged configuration for block y, belonging to super-block x (y = 31 to 0). Non-secure write to this bit is ignored if SECy bit is set in GTZC1_MPCBBz_SECCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK.
    pub mod PRIV29 {
        /// Offset (29 bits)
        pub const offset: u32 = 29;
        /// Mask (1 bit: 1 << 29)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Privileged configuration for block y, belonging to super-block x (y = 31 to 0). Non-secure write to this bit is ignored if SECy bit is set in GTZC1_MPCBBz_SECCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK.
    pub mod PRIV30 {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (1 bit: 1 << 30)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Privileged configuration for block y, belonging to super-block x (y = 31 to 0). Non-secure write to this bit is ignored if SECy bit is set in GTZC1_MPCBBz_SECCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK.
    pub mod PRIV31 {
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

/// GTZC1 SRAM1 MPCBB privileged configuration for super-block 1 register
pub mod GTZC1_MPCBB1_PRIVCFGR1 {
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV0;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV1;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV10;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV11;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV12;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV13;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV14;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV15;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV16;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV17;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV18;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV19;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV2;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV20;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV21;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV22;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV23;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV24;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV25;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV26;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV27;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV28;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV29;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV3;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV30;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV31;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV4;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV5;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV6;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV7;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV8;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV9;
}

/// GTZC1 SRAM1 MPCBB privileged configuration for super-block 2 register
pub mod GTZC1_MPCBB1_PRIVCFGR2 {
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV0;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV1;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV10;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV11;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV12;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV13;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV14;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV15;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV16;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV17;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV18;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV19;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV2;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV20;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV21;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV22;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV23;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV24;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV25;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV26;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV27;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV28;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV29;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV3;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV30;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV31;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV4;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV5;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV6;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV7;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV8;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV9;
}

/// GTZC1 SRAM1 MPCBB privileged configuration for super-block 3 register
pub mod GTZC1_MPCBB1_PRIVCFGR3 {
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV0;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV1;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV10;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV11;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV12;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV13;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV14;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV15;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV16;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV17;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV18;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV19;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV2;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV20;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV21;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV22;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV23;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV24;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV25;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV26;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV27;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV28;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV29;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV3;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV30;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV31;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV4;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV5;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV6;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV7;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV8;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV9;
}

/// GTZC1 SRAM1 MPCBB privileged configuration for super-block 4 register
pub mod GTZC1_MPCBB1_PRIVCFGR4 {
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV0;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV1;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV10;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV11;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV12;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV13;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV14;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV15;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV16;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV17;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV18;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV19;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV2;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV20;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV21;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV22;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV23;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV24;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV25;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV26;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV27;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV28;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV29;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV3;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV30;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV31;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV4;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV5;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV6;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV7;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV8;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV9;
}

/// GTZC1 SRAM1 MPCBB privileged configuration for super-block 5 register
pub mod GTZC1_MPCBB1_PRIVCFGR5 {
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV0;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV1;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV10;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV11;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV12;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV13;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV14;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV15;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV16;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV17;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV18;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV19;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV2;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV20;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV21;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV22;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV23;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV24;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV25;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV26;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV27;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV28;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV29;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV3;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV30;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV31;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV4;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV5;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV6;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV7;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV8;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV9;
}

/// GTZC1 SRAM1 MPCBB privileged configuration for super-block 6 register
pub mod GTZC1_MPCBB1_PRIVCFGR6 {
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV0;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV1;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV10;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV11;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV12;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV13;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV14;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV15;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV16;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV17;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV18;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV19;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV2;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV20;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV21;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV22;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV23;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV24;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV25;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV26;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV27;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV28;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV29;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV3;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV30;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV31;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV4;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV5;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV6;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV7;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV8;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV9;
}

/// GTZC1 SRAM1 MPCBB privileged configuration for super-block 7 register
pub mod GTZC1_MPCBB1_PRIVCFGR7 {
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV0;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV1;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV10;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV11;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV12;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV13;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV14;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV15;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV16;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV17;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV18;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV19;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV2;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV20;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV21;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV22;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV23;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV24;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV25;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV26;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV27;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV28;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV29;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV3;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV30;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV31;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV4;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV5;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV6;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV7;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV8;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV9;
}

/// GTZC1 SRAM1 MPCBB privileged configuration for super-block 8 register
pub mod GTZC1_MPCBB1_PRIVCFGR8 {
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV0;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV1;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV10;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV11;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV12;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV13;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV14;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV15;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV16;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV17;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV18;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV19;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV2;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV20;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV21;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV22;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV23;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV24;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV25;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV26;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV27;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV28;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV29;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV3;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV30;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV31;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV4;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV5;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV6;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV7;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV8;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV9;
}

/// GTZC1 SRAM1 MPCBB privileged configuration for super-block 9 register
pub mod GTZC1_MPCBB1_PRIVCFGR9 {
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV0;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV1;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV10;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV11;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV12;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV13;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV14;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV15;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV16;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV17;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV18;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV19;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV2;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV20;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV21;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV22;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV23;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV24;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV25;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV26;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV27;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV28;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV29;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV3;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV30;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV31;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV4;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV5;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV6;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV7;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV8;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV9;
}

/// GTZC1 SRAM1 MPCBB privileged configuration for super-block 10 register
pub mod GTZC1_MPCBB1_PRIVCFGR10 {
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV0;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV1;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV10;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV11;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV12;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV13;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV14;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV15;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV16;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV17;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV18;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV19;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV2;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV20;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV21;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV22;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV23;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV24;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV25;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV26;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV27;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV28;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV29;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV3;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV30;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV31;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV4;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV5;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV6;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV7;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV8;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV9;
}

/// GTZC1 SRAM1 MPCBB privileged configuration for super-block 11 register
pub mod GTZC1_MPCBB1_PRIVCFGR11 {
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV0;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV1;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV10;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV11;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV12;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV13;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV14;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV15;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV16;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV17;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV18;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV19;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV2;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV20;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV21;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV22;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV23;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV24;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV25;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV26;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV27;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV28;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV29;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV3;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV30;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV31;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV4;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV5;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV6;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV7;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV8;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV9;
}

/// GTZC1 SRAM1 MPCBB privileged configuration for super-block 12 register
pub mod GTZC1_MPCBB1_PRIVCFGR12 {
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV0;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV1;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV10;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV11;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV12;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV13;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV14;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV15;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV16;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV17;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV18;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV19;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV2;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV20;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV21;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV22;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV23;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV24;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV25;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV26;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV27;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV28;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV29;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV3;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV30;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV31;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV4;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV5;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV6;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV7;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV8;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV9;
}

/// GTZC1 SRAM1 MPCBB privileged configuration for super-block 13 register
pub mod GTZC1_MPCBB1_PRIVCFGR13 {
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV0;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV1;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV10;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV11;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV12;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV13;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV14;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV15;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV16;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV17;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV18;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV19;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV2;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV20;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV21;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV22;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV23;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV24;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV25;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV26;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV27;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV28;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV29;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV3;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV30;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV31;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV4;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV5;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV6;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV7;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV8;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV9;
}

/// GTZC1 SRAM1 MPCBB privileged configuration for super-block 14 register
pub mod GTZC1_MPCBB1_PRIVCFGR14 {
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV0;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV1;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV10;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV11;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV12;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV13;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV14;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV15;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV16;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV17;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV18;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV19;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV2;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV20;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV21;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV22;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV23;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV24;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV25;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV26;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV27;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV28;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV29;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV3;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV30;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV31;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV4;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV5;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV6;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV7;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV8;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV9;
}

/// GTZC1 SRAM1 MPCBB privileged configuration for super-block 15 register
pub mod GTZC1_MPCBB1_PRIVCFGR15 {
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV0;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV1;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV10;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV11;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV12;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV13;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV14;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV15;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV16;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV17;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV18;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV19;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV2;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV20;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV21;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV22;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV23;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV24;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV25;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV26;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV27;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV28;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV29;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV3;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV30;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV31;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV4;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV5;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV6;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV7;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV8;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV9;
}

/// GTZC1 SRAM1 MPCBB privileged configuration for super-block 16 register
pub mod GTZC1_MPCBB1_PRIVCFGR16 {
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV0;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV1;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV10;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV11;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV12;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV13;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV14;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV15;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV16;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV17;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV18;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV19;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV2;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV20;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV21;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV22;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV23;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV24;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV25;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV26;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV27;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV28;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV29;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV3;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV30;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV31;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV4;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV5;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV6;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV7;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV8;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV9;
}

/// GTZC1 SRAM1 MPCBB privileged configuration for super-block 17 register
pub mod GTZC1_MPCBB1_PRIVCFGR17 {
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV0;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV1;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV10;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV11;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV12;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV13;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV14;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV15;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV16;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV17;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV18;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV19;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV2;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV20;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV21;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV22;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV23;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV24;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV25;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV26;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV27;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV28;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV29;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV3;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV30;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV31;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV4;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV5;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV6;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV7;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV8;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV9;
}

/// GTZC1 SRAM1 MPCBB privileged configuration for super-block 18 register
pub mod GTZC1_MPCBB1_PRIVCFGR18 {
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV0;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV1;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV10;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV11;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV12;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV13;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV14;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV15;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV16;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV17;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV18;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV19;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV2;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV20;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV21;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV22;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV23;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV24;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV25;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV26;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV27;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV28;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV29;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV3;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV30;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV31;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV4;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV5;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV6;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV7;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV8;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV9;
}

/// GTZC1 SRAM1 MPCBB privileged configuration for super-block 19 register
pub mod GTZC1_MPCBB1_PRIVCFGR19 {
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV0;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV1;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV10;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV11;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV12;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV13;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV14;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV15;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV16;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV17;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV18;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV19;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV2;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV20;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV21;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV22;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV23;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV24;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV25;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV26;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV27;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV28;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV29;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV3;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV30;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV31;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV4;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV5;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV6;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV7;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV8;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV9;
}

/// GTZC1 SRAM1 MPCBB privileged configuration for super-block 20 register
pub mod GTZC1_MPCBB1_PRIVCFGR20 {
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV0;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV1;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV10;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV11;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV12;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV13;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV14;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV15;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV16;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV17;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV18;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV19;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV2;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV20;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV21;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV22;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV23;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV24;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV25;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV26;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV27;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV28;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV29;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV3;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV30;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV31;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV4;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV5;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV6;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV7;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV8;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV9;
}

/// GTZC1 SRAM1 MPCBB privileged configuration for super-block 21 register
pub mod GTZC1_MPCBB1_PRIVCFGR21 {
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV0;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV1;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV10;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV11;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV12;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV13;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV14;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV15;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV16;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV17;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV18;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV19;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV2;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV20;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV21;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV22;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV23;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV24;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV25;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV26;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV27;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV28;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV29;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV3;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV30;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV31;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV4;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV5;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV6;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV7;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV8;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV9;
}

/// GTZC1 SRAM1 MPCBB privileged configuration for super-block 22 register
pub mod GTZC1_MPCBB1_PRIVCFGR22 {
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV0;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV1;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV10;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV11;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV12;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV13;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV14;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV15;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV16;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV17;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV18;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV19;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV2;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV20;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV21;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV22;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV23;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV24;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV25;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV26;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV27;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV28;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV29;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV3;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV30;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV31;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV4;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV5;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV6;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV7;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV8;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV9;
}

/// GTZC1 SRAM1 MPCBB privileged configuration for super-block 23 register
pub mod GTZC1_MPCBB1_PRIVCFGR23 {
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV0;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV1;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV10;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV11;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV12;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV13;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV14;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV15;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV16;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV17;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV18;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV19;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV2;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV20;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV21;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV22;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV23;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV24;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV25;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV26;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV27;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV28;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV29;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV3;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV30;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV31;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV4;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV5;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV6;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV7;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV8;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV9;
}

/// GTZC1 SRAM1 MPCBB privileged configuration for super-block 24 register
pub mod GTZC1_MPCBB1_PRIVCFGR24 {
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV0;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV1;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV10;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV11;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV12;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV13;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV14;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV15;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV16;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV17;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV18;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV19;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV2;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV20;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV21;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV22;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV23;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV24;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV25;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV26;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV27;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV28;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV29;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV3;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV30;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV31;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV4;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV5;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV6;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV7;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV8;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV9;
}

/// GTZC1 SRAM1 MPCBB privileged configuration for super-block 25 register
pub mod GTZC1_MPCBB1_PRIVCFGR25 {
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV0;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV1;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV10;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV11;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV12;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV13;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV14;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV15;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV16;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV17;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV18;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV19;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV2;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV20;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV21;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV22;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV23;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV24;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV25;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV26;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV27;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV28;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV29;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV3;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV30;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV31;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV4;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV5;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV6;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV7;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV8;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV9;
}

/// GTZC1 SRAM1 MPCBB privileged configuration for super-block 26 register
pub mod GTZC1_MPCBB1_PRIVCFGR26 {
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV0;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV1;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV10;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV11;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV12;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV13;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV14;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV15;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV16;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV17;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV18;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV19;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV2;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV20;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV21;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV22;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV23;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV24;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV25;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV26;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV27;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV28;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV29;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV3;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV30;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV31;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV4;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV5;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV6;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV7;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV8;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV9;
}

/// GTZC1 SRAM1 MPCBB privileged configuration for super-block 27 register
pub mod GTZC1_MPCBB1_PRIVCFGR27 {
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV0;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV1;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV10;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV11;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV12;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV13;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV14;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV15;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV16;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV17;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV18;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV19;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV2;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV20;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV21;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV22;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV23;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV24;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV25;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV26;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV27;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV28;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV29;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV3;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV30;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV31;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV4;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV5;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV6;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV7;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV8;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV9;
}

/// GTZC1 SRAM1 MPCBB privileged configuration for super-block 28 register
pub mod GTZC1_MPCBB1_PRIVCFGR28 {
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV0;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV1;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV10;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV11;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV12;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV13;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV14;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV15;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV16;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV17;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV18;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV19;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV2;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV20;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV21;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV22;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV23;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV24;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV25;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV26;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV27;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV28;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV29;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV3;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV30;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV31;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV4;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV5;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV6;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV7;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV8;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV9;
}

/// GTZC1 SRAM1 MPCBB privileged configuration for super-block 29 register
pub mod GTZC1_MPCBB1_PRIVCFGR29 {
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV0;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV1;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV10;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV11;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV12;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV13;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV14;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV15;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV16;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV17;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV18;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV19;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV2;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV20;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV21;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV22;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV23;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV24;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV25;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV26;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV27;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV28;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV29;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV3;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV30;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV31;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV4;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV5;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV6;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV7;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV8;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV9;
}

/// GTZC1 SRAM1 MPCBB privileged configuration for super-block 30 register
pub mod GTZC1_MPCBB1_PRIVCFGR30 {
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV0;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV1;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV10;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV11;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV12;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV13;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV14;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV15;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV16;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV17;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV18;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV19;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV2;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV20;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV21;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV22;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV23;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV24;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV25;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV26;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV27;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV28;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV29;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV3;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV30;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV31;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV4;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV5;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV6;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV7;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV8;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV9;
}

/// GTZC1 SRAM1 MPCBB privileged configuration for super-block 31 register
pub mod GTZC1_MPCBB1_PRIVCFGR31 {
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV0;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV1;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV10;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV11;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV12;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV13;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV14;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV15;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV16;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV17;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV18;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV19;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV2;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV20;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV21;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV22;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV23;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV24;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV25;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV26;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV27;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV28;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV29;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV3;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV30;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV31;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV4;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV5;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV6;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV7;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV8;
    pub use super::GTZC1_MPCBB1_PRIVCFGR0::PRIV9;
}
#[repr(C)]
pub struct RegisterBlock {
    /// GTZC1 SRAM1 MPCBB control register
    pub GTZC1_MPCBB1_CR: RWRegister<u32>,

    _reserved1: [u8; 12],

    /// GTZC1 SRAM1 MPCBB configuration lock register 1
    pub GTZC1_MPCBB1_CFGLOCK1: RWRegister<u32>,

    _reserved2: [u8; 236],

    /// GTZC1 SRAM1 MPCBB security configuration for super-block 0 register
    pub GTZC1_MPCBB1_SECCFGR0: RWRegister<u32>,

    /// GTZC1 SRAM1 MPCBB security configuration for super-block 1 register
    pub GTZC1_MPCBB1_SECCFGR1: RWRegister<u32>,

    /// GTZC1 SRAM1 MPCBB security configuration for super-block 2 register
    pub GTZC1_MPCBB1_SECCFGR2: RWRegister<u32>,

    /// GTZC1 SRAM1 MPCBB security configuration for super-block 3 register
    pub GTZC1_MPCBB1_SECCFGR3: RWRegister<u32>,

    /// GTZC1 SRAM1 MPCBB security configuration for super-block 4 register
    pub GTZC1_MPCBB1_SECCFGR4: RWRegister<u32>,

    /// GTZC1 SRAM1 MPCBB security configuration for super-block 5 register
    pub GTZC1_MPCBB1_SECCFGR5: RWRegister<u32>,

    /// GTZC1 SRAM1 MPCBB security configuration for super-block 6 register
    pub GTZC1_MPCBB1_SECCFGR6: RWRegister<u32>,

    /// GTZC1 SRAM1 MPCBB security configuration for super-block 7 register
    pub GTZC1_MPCBB1_SECCFGR7: RWRegister<u32>,

    /// GTZC1 SRAM1 MPCBB security configuration for super-block 8 register
    pub GTZC1_MPCBB1_SECCFGR8: RWRegister<u32>,

    /// GTZC1 SRAM1 MPCBB security configuration for super-block 9 register
    pub GTZC1_MPCBB1_SECCFGR9: RWRegister<u32>,

    /// GTZC1 SRAM1 MPCBB security configuration for super-block 10 register
    pub GTZC1_MPCBB1_SECCFGR10: RWRegister<u32>,

    /// GTZC1 SRAM1 MPCBB security configuration for super-block 11 register
    pub GTZC1_MPCBB1_SECCFGR11: RWRegister<u32>,

    /// GTZC1 SRAM1 MPCBB security configuration for super-block 12 register
    pub GTZC1_MPCBB1_SECCFGR12: RWRegister<u32>,

    /// GTZC1 SRAM1 MPCBB security configuration for super-block 13 register
    pub GTZC1_MPCBB1_SECCFGR13: RWRegister<u32>,

    /// GTZC1 SRAM1 MPCBB security configuration for super-block 14 register
    pub GTZC1_MPCBB1_SECCFGR14: RWRegister<u32>,

    /// GTZC1 SRAM1 MPCBB security configuration for super-block 15 register
    pub GTZC1_MPCBB1_SECCFGR15: RWRegister<u32>,

    /// GTZC1 SRAM1 MPCBB security configuration for super-block 16 register
    pub GTZC1_MPCBB1_SECCFGR16: RWRegister<u32>,

    /// GTZC1 SRAM1 MPCBB security configuration for super-block 17 register
    pub GTZC1_MPCBB1_SECCFGR17: RWRegister<u32>,

    /// GTZC1 SRAM1 MPCBB security configuration for super-block 18 register
    pub GTZC1_MPCBB1_SECCFGR18: RWRegister<u32>,

    /// GTZC1 SRAM1 MPCBB security configuration for super-block 19 register
    pub GTZC1_MPCBB1_SECCFGR19: RWRegister<u32>,

    /// GTZC1 SRAM1 MPCBB security configuration for super-block 20 register
    pub GTZC1_MPCBB1_SECCFGR20: RWRegister<u32>,

    /// GTZC1 SRAM1 MPCBB security configuration for super-block 21 register
    pub GTZC1_MPCBB1_SECCFGR21: RWRegister<u32>,

    /// GTZC1 SRAM1 MPCBB security configuration for super-block 22 register
    pub GTZC1_MPCBB1_SECCFGR22: RWRegister<u32>,

    /// GTZC1 SRAM1 MPCBB security configuration for super-block 23 register
    pub GTZC1_MPCBB1_SECCFGR23: RWRegister<u32>,

    /// GTZC1 SRAM1 MPCBB security configuration for super-block 24 register
    pub GTZC1_MPCBB1_SECCFGR24: RWRegister<u32>,

    /// GTZC1 SRAM1 MPCBB security configuration for super-block 25 register
    pub GTZC1_MPCBB1_SECCFGR25: RWRegister<u32>,

    /// GTZC1 SRAM1 MPCBB security configuration for super-block 26 register
    pub GTZC1_MPCBB1_SECCFGR26: RWRegister<u32>,

    /// GTZC1 SRAM1 MPCBB security configuration for super-block 27 register
    pub GTZC1_MPCBB1_SECCFGR27: RWRegister<u32>,

    /// GTZC1 SRAM1 MPCBB security configuration for super-block 28 register
    pub GTZC1_MPCBB1_SECCFGR28: RWRegister<u32>,

    /// GTZC1 SRAM1 MPCBB security configuration for super-block 29 register
    pub GTZC1_MPCBB1_SECCFGR29: RWRegister<u32>,

    /// GTZC1 SRAM1 MPCBB security configuration for super-block 30 register
    pub GTZC1_MPCBB1_SECCFGR30: RWRegister<u32>,

    /// GTZC1 SRAM1 MPCBB security configuration for super-block 31 register
    pub GTZC1_MPCBB1_SECCFGR31: RWRegister<u32>,

    _reserved3: [u8; 128],

    /// GTZC1 SRAM1 MPCBB privileged configuration for super-block 0 register
    pub GTZC1_MPCBB1_PRIVCFGR0: RWRegister<u32>,

    /// GTZC1 SRAM1 MPCBB privileged configuration for super-block 1 register
    pub GTZC1_MPCBB1_PRIVCFGR1: RWRegister<u32>,

    /// GTZC1 SRAM1 MPCBB privileged configuration for super-block 2 register
    pub GTZC1_MPCBB1_PRIVCFGR2: RWRegister<u32>,

    /// GTZC1 SRAM1 MPCBB privileged configuration for super-block 3 register
    pub GTZC1_MPCBB1_PRIVCFGR3: RWRegister<u32>,

    /// GTZC1 SRAM1 MPCBB privileged configuration for super-block 4 register
    pub GTZC1_MPCBB1_PRIVCFGR4: RWRegister<u32>,

    /// GTZC1 SRAM1 MPCBB privileged configuration for super-block 5 register
    pub GTZC1_MPCBB1_PRIVCFGR5: RWRegister<u32>,

    /// GTZC1 SRAM1 MPCBB privileged configuration for super-block 6 register
    pub GTZC1_MPCBB1_PRIVCFGR6: RWRegister<u32>,

    /// GTZC1 SRAM1 MPCBB privileged configuration for super-block 7 register
    pub GTZC1_MPCBB1_PRIVCFGR7: RWRegister<u32>,

    /// GTZC1 SRAM1 MPCBB privileged configuration for super-block 8 register
    pub GTZC1_MPCBB1_PRIVCFGR8: RWRegister<u32>,

    /// GTZC1 SRAM1 MPCBB privileged configuration for super-block 9 register
    pub GTZC1_MPCBB1_PRIVCFGR9: RWRegister<u32>,

    /// GTZC1 SRAM1 MPCBB privileged configuration for super-block 10 register
    pub GTZC1_MPCBB1_PRIVCFGR10: RWRegister<u32>,

    /// GTZC1 SRAM1 MPCBB privileged configuration for super-block 11 register
    pub GTZC1_MPCBB1_PRIVCFGR11: RWRegister<u32>,

    /// GTZC1 SRAM1 MPCBB privileged configuration for super-block 12 register
    pub GTZC1_MPCBB1_PRIVCFGR12: RWRegister<u32>,

    /// GTZC1 SRAM1 MPCBB privileged configuration for super-block 13 register
    pub GTZC1_MPCBB1_PRIVCFGR13: RWRegister<u32>,

    /// GTZC1 SRAM1 MPCBB privileged configuration for super-block 14 register
    pub GTZC1_MPCBB1_PRIVCFGR14: RWRegister<u32>,

    /// GTZC1 SRAM1 MPCBB privileged configuration for super-block 15 register
    pub GTZC1_MPCBB1_PRIVCFGR15: RWRegister<u32>,

    /// GTZC1 SRAM1 MPCBB privileged configuration for super-block 16 register
    pub GTZC1_MPCBB1_PRIVCFGR16: RWRegister<u32>,

    /// GTZC1 SRAM1 MPCBB privileged configuration for super-block 17 register
    pub GTZC1_MPCBB1_PRIVCFGR17: RWRegister<u32>,

    /// GTZC1 SRAM1 MPCBB privileged configuration for super-block 18 register
    pub GTZC1_MPCBB1_PRIVCFGR18: RWRegister<u32>,

    /// GTZC1 SRAM1 MPCBB privileged configuration for super-block 19 register
    pub GTZC1_MPCBB1_PRIVCFGR19: RWRegister<u32>,

    /// GTZC1 SRAM1 MPCBB privileged configuration for super-block 20 register
    pub GTZC1_MPCBB1_PRIVCFGR20: RWRegister<u32>,

    /// GTZC1 SRAM1 MPCBB privileged configuration for super-block 21 register
    pub GTZC1_MPCBB1_PRIVCFGR21: RWRegister<u32>,

    /// GTZC1 SRAM1 MPCBB privileged configuration for super-block 22 register
    pub GTZC1_MPCBB1_PRIVCFGR22: RWRegister<u32>,

    /// GTZC1 SRAM1 MPCBB privileged configuration for super-block 23 register
    pub GTZC1_MPCBB1_PRIVCFGR23: RWRegister<u32>,

    /// GTZC1 SRAM1 MPCBB privileged configuration for super-block 24 register
    pub GTZC1_MPCBB1_PRIVCFGR24: RWRegister<u32>,

    /// GTZC1 SRAM1 MPCBB privileged configuration for super-block 25 register
    pub GTZC1_MPCBB1_PRIVCFGR25: RWRegister<u32>,

    /// GTZC1 SRAM1 MPCBB privileged configuration for super-block 26 register
    pub GTZC1_MPCBB1_PRIVCFGR26: RWRegister<u32>,

    /// GTZC1 SRAM1 MPCBB privileged configuration for super-block 27 register
    pub GTZC1_MPCBB1_PRIVCFGR27: RWRegister<u32>,

    /// GTZC1 SRAM1 MPCBB privileged configuration for super-block 28 register
    pub GTZC1_MPCBB1_PRIVCFGR28: RWRegister<u32>,

    /// GTZC1 SRAM1 MPCBB privileged configuration for super-block 29 register
    pub GTZC1_MPCBB1_PRIVCFGR29: RWRegister<u32>,

    /// GTZC1 SRAM1 MPCBB privileged configuration for super-block 30 register
    pub GTZC1_MPCBB1_PRIVCFGR30: RWRegister<u32>,

    /// GTZC1 SRAM1 MPCBB privileged configuration for super-block 31 register
    pub GTZC1_MPCBB1_PRIVCFGR31: RWRegister<u32>,
}
pub struct ResetValues {
    pub GTZC1_MPCBB1_CR: u32,
    pub GTZC1_MPCBB1_CFGLOCK1: u32,
    pub GTZC1_MPCBB1_SECCFGR0: u32,
    pub GTZC1_MPCBB1_SECCFGR1: u32,
    pub GTZC1_MPCBB1_SECCFGR2: u32,
    pub GTZC1_MPCBB1_SECCFGR3: u32,
    pub GTZC1_MPCBB1_SECCFGR4: u32,
    pub GTZC1_MPCBB1_SECCFGR5: u32,
    pub GTZC1_MPCBB1_SECCFGR6: u32,
    pub GTZC1_MPCBB1_SECCFGR7: u32,
    pub GTZC1_MPCBB1_SECCFGR8: u32,
    pub GTZC1_MPCBB1_SECCFGR9: u32,
    pub GTZC1_MPCBB1_SECCFGR10: u32,
    pub GTZC1_MPCBB1_SECCFGR11: u32,
    pub GTZC1_MPCBB1_SECCFGR12: u32,
    pub GTZC1_MPCBB1_SECCFGR13: u32,
    pub GTZC1_MPCBB1_SECCFGR14: u32,
    pub GTZC1_MPCBB1_SECCFGR15: u32,
    pub GTZC1_MPCBB1_SECCFGR16: u32,
    pub GTZC1_MPCBB1_SECCFGR17: u32,
    pub GTZC1_MPCBB1_SECCFGR18: u32,
    pub GTZC1_MPCBB1_SECCFGR19: u32,
    pub GTZC1_MPCBB1_SECCFGR20: u32,
    pub GTZC1_MPCBB1_SECCFGR21: u32,
    pub GTZC1_MPCBB1_SECCFGR22: u32,
    pub GTZC1_MPCBB1_SECCFGR23: u32,
    pub GTZC1_MPCBB1_SECCFGR24: u32,
    pub GTZC1_MPCBB1_SECCFGR25: u32,
    pub GTZC1_MPCBB1_SECCFGR26: u32,
    pub GTZC1_MPCBB1_SECCFGR27: u32,
    pub GTZC1_MPCBB1_SECCFGR28: u32,
    pub GTZC1_MPCBB1_SECCFGR29: u32,
    pub GTZC1_MPCBB1_SECCFGR30: u32,
    pub GTZC1_MPCBB1_SECCFGR31: u32,
    pub GTZC1_MPCBB1_PRIVCFGR0: u32,
    pub GTZC1_MPCBB1_PRIVCFGR1: u32,
    pub GTZC1_MPCBB1_PRIVCFGR2: u32,
    pub GTZC1_MPCBB1_PRIVCFGR3: u32,
    pub GTZC1_MPCBB1_PRIVCFGR4: u32,
    pub GTZC1_MPCBB1_PRIVCFGR5: u32,
    pub GTZC1_MPCBB1_PRIVCFGR6: u32,
    pub GTZC1_MPCBB1_PRIVCFGR7: u32,
    pub GTZC1_MPCBB1_PRIVCFGR8: u32,
    pub GTZC1_MPCBB1_PRIVCFGR9: u32,
    pub GTZC1_MPCBB1_PRIVCFGR10: u32,
    pub GTZC1_MPCBB1_PRIVCFGR11: u32,
    pub GTZC1_MPCBB1_PRIVCFGR12: u32,
    pub GTZC1_MPCBB1_PRIVCFGR13: u32,
    pub GTZC1_MPCBB1_PRIVCFGR14: u32,
    pub GTZC1_MPCBB1_PRIVCFGR15: u32,
    pub GTZC1_MPCBB1_PRIVCFGR16: u32,
    pub GTZC1_MPCBB1_PRIVCFGR17: u32,
    pub GTZC1_MPCBB1_PRIVCFGR18: u32,
    pub GTZC1_MPCBB1_PRIVCFGR19: u32,
    pub GTZC1_MPCBB1_PRIVCFGR20: u32,
    pub GTZC1_MPCBB1_PRIVCFGR21: u32,
    pub GTZC1_MPCBB1_PRIVCFGR22: u32,
    pub GTZC1_MPCBB1_PRIVCFGR23: u32,
    pub GTZC1_MPCBB1_PRIVCFGR24: u32,
    pub GTZC1_MPCBB1_PRIVCFGR25: u32,
    pub GTZC1_MPCBB1_PRIVCFGR26: u32,
    pub GTZC1_MPCBB1_PRIVCFGR27: u32,
    pub GTZC1_MPCBB1_PRIVCFGR28: u32,
    pub GTZC1_MPCBB1_PRIVCFGR29: u32,
    pub GTZC1_MPCBB1_PRIVCFGR30: u32,
    pub GTZC1_MPCBB1_PRIVCFGR31: u32,
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
