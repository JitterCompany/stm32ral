#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! RAMs configuration controller
//!
//! Used by: stm32h562, stm32h563, stm32h573

use crate::{RORegister, RWRegister, WORegister};
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// RAMCFG memory 1 control register
pub mod M1CR {

    /// ECC enable. This bit reset value is defined by the user option bit configuration. When set, it can be cleared by software only after writing the unlock sequence in the RAMCFG_MxECCKEYR register. Note: This bit is reserved and must be kept at reset value in SRAM1 control register.
    pub mod ECCE {
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

    /// Address latch enable Note: This bit is reserved and must be kept at reset value in SRAM1 control register.
    pub mod ALE {
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

    /// SRAM erase This bit can be set by software only after writing the unlock sequence in the ERASEKEY field of the RAMCFG_MxERKEYR register. Setting this bit starts the SRAM erase. This bit is automatically cleared by hardware at the end of the erase operation.
    pub mod SRAMER {
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
}

/// RAMCFG memory interrupt status register
pub mod M1ISR {

    /// ECC single error detected and corrected Note: This bit is reserved and must be kept at reset value in SRAM1 interrupt status register.
    pub mod SEDC {
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

    /// ECC double error detected Note: This bit is reserved and must be kept at reset value in SRAM1 interrupt status register.
    pub mod DED {
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

    /// SRAM busy with erase operation Note: Depending on the SRAM, the erase operation can be performed due to software request, system reset if the option bit is enabled, tamper detection or readout protection regression. Refer to Table 18: Internal SRAMs features.
    pub mod SRAMBUSY {
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
}

/// RAMCFG memory 1 erase key register
pub mod M1ERKEYR {

    /// Erase write protection key The following steps are required to unlock the write protection of the SRAMER bit in the RAMCFG_MxCR register. 1) Write 0xCA into ERASEKEY\[7:0\]. 2) Write 0x53 into ERASEKEY\[7:0\]. Note: Writing a wrong key reactivates the write protection.
    pub mod ERASEKEY {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (8 bits: 0xff << 0)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// RAMCFG memory 2 control register
pub mod M2CR {
    pub use super::M1CR::ALE;
    pub use super::M1CR::ECCE;
    pub use super::M1CR::SRAMER;
}

/// RAMCFG memory 2 interrupt enable register
pub mod M2IER {

    /// ECC single error interrupt enable
    pub mod SEIE {
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

    /// ECC double error interrupt enable
    pub mod DEIE {
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

    /// Double error NMI This bit is set by software and cleared only by a global RAMCFG reset. Note: if ECCNMI is set, the RAMCFG maskable interrupt is not generated whatever DEIE bit value.
    pub mod ECCNMI {
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
}

/// RAMCFG memory interrupt status register
pub mod M2ISR {
    pub use super::M1ISR::DED;
    pub use super::M1ISR::SEDC;
    pub use super::M1ISR::SRAMBUSY;
}

/// RAMCFG memory 2 ECC single error address register
pub mod M2SEAR {

    /// ECC single error address When the ALE bit is set in the RAMCFG_MxCR register, this field is updated with the address corresponding to the ECC single error.
    pub mod ESEA {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// RAMCFG memory 2 ECC double error address register
pub mod M2DEAR {

    /// ECC double error address When the ALE bit is set in the RAMCFG_MxCR register, this field is updated with the address corresponding to the ECC double error.
    pub mod EDEA {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// RAMCFG memory 2 interrupt clear register 2
pub mod M2ICR {

    /// Clear ECC single error detected and corrected Writing 1 to this flag clears the SEDC bit in the RAMCFG_MxISR register. Reading this flag returns the SEDC value.
    pub mod CSEDC {
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

    /// Clear ECC double error detected Writing 1 to this flag clears the DED bit in the RAMCFG_MxISR register. Reading this flag returns the DED value.
    pub mod CDED {
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
}

/// RAMCFG memory 2 write protection register 1
pub mod M2WPR1 {

    /// SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset.
    pub mod P0WP {
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

    /// SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset.
    pub mod P1WP {
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

    /// SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset.
    pub mod P2WP {
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

    /// SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset.
    pub mod P3WP {
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

    /// SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset.
    pub mod P4WP {
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

    /// SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset.
    pub mod P5WP {
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

    /// SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset.
    pub mod P6WP {
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

    /// SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset.
    pub mod P7WP {
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

    /// SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset.
    pub mod P8WP {
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

    /// SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset.
    pub mod P9WP {
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

    /// SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset.
    pub mod P10WP {
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

    /// SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset.
    pub mod P11WP {
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

    /// SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset.
    pub mod P12WP {
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

    /// SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset.
    pub mod P13WP {
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

    /// SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset.
    pub mod P14WP {
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

    /// SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset.
    pub mod P15WP {
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

    /// SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset.
    pub mod P16WP {
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

    /// SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset.
    pub mod P17WP {
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

    /// SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset.
    pub mod P18WP {
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

    /// SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset.
    pub mod P19WP {
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

    /// SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset.
    pub mod P20WP {
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

    /// SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset.
    pub mod P21WP {
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

    /// SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset.
    pub mod P22WP {
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

    /// SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset.
    pub mod P23WP {
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

    /// SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset.
    pub mod P24WP {
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

    /// SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset.
    pub mod P25WP {
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

    /// SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset.
    pub mod P26WP {
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

    /// SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset.
    pub mod P27WP {
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

    /// SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset.
    pub mod P28WP {
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

    /// SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset.
    pub mod P29WP {
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

    /// SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset.
    pub mod P30WP {
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

    /// SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset.
    pub mod P31WP {
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

/// RAMCFG memory 2 write protection register 2
pub mod M2WPR2 {

    /// SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset.
    pub mod P32WP {
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

    /// SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset.
    pub mod P33WP {
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

    /// SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset.
    pub mod P34WP {
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

    /// SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset.
    pub mod P35WP {
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

    /// SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset.
    pub mod P36WP {
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

    /// SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset.
    pub mod P37WP {
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

    /// SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset.
    pub mod P38WP {
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

    /// SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset.
    pub mod P39WP {
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

    /// SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset.
    pub mod P40WP {
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

    /// SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset.
    pub mod P41WP {
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

    /// SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset.
    pub mod P42WP {
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

    /// SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset.
    pub mod P43WP {
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

    /// SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset.
    pub mod P44WP {
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

    /// SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset.
    pub mod P45WP {
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

    /// SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset.
    pub mod P46WP {
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

    /// SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset.
    pub mod P47WP {
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

    /// SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset.
    pub mod P48WP {
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

    /// SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset.
    pub mod P49WP {
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

    /// SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset.
    pub mod P50WP {
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

    /// SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset.
    pub mod P51WP {
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

    /// SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset.
    pub mod P52WP {
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

    /// SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset.
    pub mod P53WP {
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

    /// SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset.
    pub mod P54WP {
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

    /// SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset.
    pub mod P55WP {
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

    /// SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset.
    pub mod P56WP {
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

    /// SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset.
    pub mod P57WP {
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

    /// SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset.
    pub mod P58WP {
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

    /// SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset.
    pub mod P59WP {
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

    /// SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset.
    pub mod P60WP {
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

    /// SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset.
    pub mod P61WP {
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

    /// SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset.
    pub mod P62WP {
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

    /// SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset.
    pub mod P63WP {
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

/// RAMCFG memory 2 ECC key register
pub mod M2ECCKEYR {

    /// ECC write protection key The following steps are required to unlock the write protection of the ECCE bit in the RAMCFG_MxCR register. 1) Write 0xAE into ECCKEY\[7:0\]. 2) Write 0x75 into ECCKEY\[7:0\]. Note: Writing a wrong key reactivates the write protection.
    pub mod ECCKEY {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (8 bits: 0xff << 0)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// RAMCFG memory 2 erase key register
pub mod M2ERKEYR {
    pub use super::M1ERKEYR::ERASEKEY;
}

/// RAMCFG memory 3 control register
pub mod M3CR {
    pub use super::M1CR::ALE;
    pub use super::M1CR::ECCE;
    pub use super::M1CR::SRAMER;
}

/// RAMCFG memory 3 interrupt enable register
pub mod M3IER {
    pub use super::M2IER::DEIE;
    pub use super::M2IER::ECCNMI;
    pub use super::M2IER::SEIE;
}

/// RAMCFG memory interrupt status register
pub mod M3ISR {
    pub use super::M1ISR::DED;
    pub use super::M1ISR::SEDC;
    pub use super::M1ISR::SRAMBUSY;
}

/// RAMCFG memory 3 ECC single error address register
pub mod M3SEAR {
    pub use super::M2SEAR::ESEA;
}

/// RAMCFG memory 3 ECC double error address register
pub mod M3DEAR {
    pub use super::M2DEAR::EDEA;
}

/// RAMCFG memory 3 interrupt clear register 3
pub mod M3ICR {
    pub use super::M2ICR::CDED;
    pub use super::M2ICR::CSEDC;
}

/// RAMCFG memory 3 ECC key register
pub mod M3ECCKEYR {
    pub use super::M2ECCKEYR::ECCKEY;
}

/// RAMCFG memory 3 erase key register
pub mod M3ERKEYR {
    pub use super::M1ERKEYR::ERASEKEY;
}

/// RAMCFG memory 4 erase key register
pub mod M4ERKEYR {
    pub use super::M1ERKEYR::ERASEKEY;
}

/// RAMCFG memory 5 control register
pub mod M5CR {
    pub use super::M1CR::ALE;
    pub use super::M1CR::ECCE;
    pub use super::M1CR::SRAMER;
}

/// RAMCFG memory 5 interrupt enable register
pub mod M5IER {
    pub use super::M2IER::DEIE;
    pub use super::M2IER::ECCNMI;
    pub use super::M2IER::SEIE;
}

/// RAMCFG memory interrupt status register
pub mod M5ISR {
    pub use super::M1ISR::DED;
    pub use super::M1ISR::SEDC;
    pub use super::M1ISR::SRAMBUSY;
}

/// RAMCFG memory 5 ECC single error address register
pub mod M5SEAR {
    pub use super::M2SEAR::ESEA;
}

/// RAMCFG memory 5 ECC double error address register
pub mod M5DEAR {
    pub use super::M2DEAR::EDEA;
}

/// RAMCFG memory 5 interrupt clear register 5
pub mod M5ICR {
    pub use super::M2ICR::CDED;
    pub use super::M2ICR::CSEDC;
}

/// RAMCFG memory 5 ECC key register
pub mod M5ECCKEYR {
    pub use super::M2ECCKEYR::ECCKEY;
}

/// RAMCFG memory 5 erase key register
pub mod M5ERKEYR {
    pub use super::M1ERKEYR::ERASEKEY;
}
#[repr(C)]
pub struct RegisterBlock {
    /// RAMCFG memory 1 control register
    pub M1CR: RWRegister<u32>,

    _reserved1: [u8; 4],

    /// RAMCFG memory interrupt status register
    pub M1ISR: RORegister<u32>,

    _reserved2: [u8; 28],

    /// RAMCFG memory 1 erase key register
    pub M1ERKEYR: WORegister<u32>,

    _reserved3: [u8; 20],

    /// RAMCFG memory 2 control register
    pub M2CR: RWRegister<u32>,

    /// RAMCFG memory 2 interrupt enable register
    pub M2IER: RWRegister<u32>,

    /// RAMCFG memory interrupt status register
    pub M2ISR: RORegister<u32>,

    /// RAMCFG memory 2 ECC single error address register
    pub M2SEAR: RORegister<u32>,

    /// RAMCFG memory 2 ECC double error address register
    pub M2DEAR: RORegister<u32>,

    /// RAMCFG memory 2 interrupt clear register 2
    pub M2ICR: RWRegister<u32>,

    /// RAMCFG memory 2 write protection register 1
    pub M2WPR1: RWRegister<u32>,

    /// RAMCFG memory 2 write protection register 2
    pub M2WPR2: RWRegister<u32>,

    _reserved4: [u8; 4],

    /// RAMCFG memory 2 ECC key register
    pub M2ECCKEYR: WORegister<u32>,

    /// RAMCFG memory 2 erase key register
    pub M2ERKEYR: WORegister<u32>,

    _reserved5: [u8; 20],

    /// RAMCFG memory 3 control register
    pub M3CR: RWRegister<u32>,

    /// RAMCFG memory 3 interrupt enable register
    pub M3IER: RWRegister<u32>,

    /// RAMCFG memory interrupt status register
    pub M3ISR: RORegister<u32>,

    /// RAMCFG memory 3 ECC single error address register
    pub M3SEAR: RORegister<u32>,

    /// RAMCFG memory 3 ECC double error address register
    pub M3DEAR: RORegister<u32>,

    /// RAMCFG memory 3 interrupt clear register 3
    pub M3ICR: RWRegister<u32>,

    _reserved6: [u8; 12],

    /// RAMCFG memory 3 ECC key register
    pub M3ECCKEYR: WORegister<u32>,

    /// RAMCFG memory 3 erase key register
    pub M3ERKEYR: WORegister<u32>,

    _reserved7: [u8; 60],

    /// RAMCFG memory 4 erase key register
    pub M4ERKEYR: WORegister<u32>,

    _reserved8: [u8; 20],

    /// RAMCFG memory 5 control register
    pub M5CR: RWRegister<u32>,

    /// RAMCFG memory 5 interrupt enable register
    pub M5IER: RWRegister<u32>,

    /// RAMCFG memory interrupt status register
    pub M5ISR: RORegister<u32>,

    /// RAMCFG memory 5 ECC single error address register
    pub M5SEAR: RORegister<u32>,

    /// RAMCFG memory 5 ECC double error address register
    pub M5DEAR: RORegister<u32>,

    /// RAMCFG memory 5 interrupt clear register 5
    pub M5ICR: RWRegister<u32>,

    _reserved9: [u8; 12],

    /// RAMCFG memory 5 ECC key register
    pub M5ECCKEYR: WORegister<u32>,

    /// RAMCFG memory 5 erase key register
    pub M5ERKEYR: WORegister<u32>,
}
pub struct ResetValues {
    pub M1CR: u32,
    pub M1ISR: u32,
    pub M1ERKEYR: u32,
    pub M2CR: u32,
    pub M2IER: u32,
    pub M2ISR: u32,
    pub M2SEAR: u32,
    pub M2DEAR: u32,
    pub M2ICR: u32,
    pub M2WPR1: u32,
    pub M2WPR2: u32,
    pub M2ECCKEYR: u32,
    pub M2ERKEYR: u32,
    pub M3CR: u32,
    pub M3IER: u32,
    pub M3ISR: u32,
    pub M3SEAR: u32,
    pub M3DEAR: u32,
    pub M3ICR: u32,
    pub M3ECCKEYR: u32,
    pub M3ERKEYR: u32,
    pub M4ERKEYR: u32,
    pub M5CR: u32,
    pub M5IER: u32,
    pub M5ISR: u32,
    pub M5SEAR: u32,
    pub M5DEAR: u32,
    pub M5ICR: u32,
    pub M5ECCKEYR: u32,
    pub M5ERKEYR: u32,
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
