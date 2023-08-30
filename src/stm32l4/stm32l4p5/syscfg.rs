#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! System configuration controller

use crate::{RWRegister, WORegister};
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// memory remap register
pub mod MEMRMP {

    /// Flash Bank mode selection
    pub mod FB_MODE {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Flash Bank 1 mapped at 0x0800 0000 (and aliased @0x0000 0000(1)) and Flash Bank 2 mapped at offset
            pub const Normal: u32 = 0b0;

            /// 0b1: Flash Bank 2 mapped at 0x0800 0000 (and aliased @0x0000 0000(1)) and Flash Bank 1 mapped at offset
            pub const Inverted: u32 = 0b1;
        }
    }

    /// Memory mapping selection
    pub mod MEM_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (3 bits: 0b111 << 0)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: Main Flash memory mapped at 0x00000000
            pub const MainFlash: u32 = 0b000;

            /// 0b001: System Flash memory mapped at 0x00000000
            pub const SystemFlash: u32 = 0b001;

            /// 0b010: FMC bank 1 (NOR/PSRAM 1 and 2) mapped at 0x00000000
            pub const FMC: u32 = 0b010;

            /// 0b011: SRAM1 mapped at 0x00000000
            pub const SRAM1: u32 = 0b011;

            /// 0b100: OCTOSPI1 memory mapped at 0x00000000
            pub const OCTOSPI1: u32 = 0b100;

            /// 0b101: OCTOSPI2 memory mapped at 0x00000000
            pub const OCTOSPI2: u32 = 0b101;
        }
    }
}

/// configuration register 1
pub mod CFGR1 {

    /// I2C3 Fast-mode Plus driving capability activation
    pub mod I2C3_FMP {
        /// Offset (22 bits)
        pub const offset: u32 = 22;
        /// Mask (1 bit: 1 << 22)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Fm+ mode is not enabled on I2Cx pins selected through AF selection bits
            pub const Disabled: u32 = 0b0;

            /// 0b1: Fm+ mode is enabled on I2Cx pins selected through AF selection bits
            pub const Enabled: u32 = 0b1;
        }
    }

    /// I2C2 Fast-mode Plus driving capability activation
    pub mod I2C2_FMP {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (1 bit: 1 << 21)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::I2C3_FMP::RW;
    }

    /// I2C1 Fast-mode Plus driving capability activation
    pub mod I2C1_FMP {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (1 bit: 1 << 20)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::I2C3_FMP::RW;
    }

    /// Fast-mode Plus (Fm+) driving capability activation on PB9
    pub mod I2C_PB9_FMP {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (1 bit: 1 << 19)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: PBx pin operates in standard mode
            pub const Disabled: u32 = 0b0;

            /// 0b1: Fm+ mode enabled on PB7 pin, and the Speed control is bypassed
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Fast-mode Plus (Fm+) driving capability activation on PB8
    pub mod I2C_PB8_FMP {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (1 bit: 1 << 18)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::I2C_PB9_FMP::RW;
    }

    /// Fast-mode Plus (Fm+) driving capability activation on PB7
    pub mod I2C_PB7_FMP {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::I2C_PB9_FMP::RW;
    }

    /// Fast-mode Plus (Fm+) driving capability activation on PB6
    pub mod I2C_PB6_FMP {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::I2C_PB9_FMP::RW;
    }

    /// I/O analog switch voltage booster enable
    pub mod BOOSTEN {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: I/O analog switches are supplied by VDDA voltage. This is the recommended configuration when using the ADC in high VDDA voltage operation
            pub const Disabled: u32 = 0b0;

            /// 0b1: I/O analog switches are supplied by a dedicated voltage booster (supplied by VDD). This is the recommended configuration when using the ADC in low VDDA voltage operation
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Firewall disable
    pub mod FWDIS {
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

            /// 0b0: Firewall protection enabled
            pub const Enabled: u32 = 0b0;

            /// 0b1: Firewall protection disabled
            pub const Disabled: u32 = 0b1;
        }
    }

    /// Inexact interrupt enable
    pub mod FPU_IE5 {
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

            /// 0b0: Interrupt disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Interrupt enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Input denormal interrupt enable
    pub mod FPU_IE4 {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (1 bit: 1 << 30)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::FPU_IE5::RW;
    }

    /// Overflow interrupt enable
    pub mod FPU_IE3 {
        /// Offset (29 bits)
        pub const offset: u32 = 29;
        /// Mask (1 bit: 1 << 29)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::FPU_IE5::RW;
    }

    /// Underflow interrupt enable
    pub mod FPU_IE2 {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (1 bit: 1 << 28)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::FPU_IE5::RW;
    }

    /// Divide-by-zero interrupt enable
    pub mod FPU_IE1 {
        /// Offset (27 bits)
        pub const offset: u32 = 27;
        /// Mask (1 bit: 1 << 27)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::FPU_IE5::RW;
    }

    /// Invalid operation interrupt enable
    pub mod FPU_IE0 {
        /// Offset (26 bits)
        pub const offset: u32 = 26;
        /// Mask (1 bit: 1 << 26)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::FPU_IE5::RW;
    }

    /// GPIO analog switch control voltage selection when at least one analog peripheral supplied by VDDA is enabled (COMP, OPAMP, VREFBUF, ADC,...)
    pub mod ANASWVDD {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: I/O analog switches supplied by VDDA or booster when booster is ON
            pub const VDDA: u32 = 0b0;

            /// 0b1: I/O analog switches supplied by VDD
            pub const VDD: u32 = 0b1;
        }
    }

    /// I2C3 Fast-mode Plus driving capability activation
    pub mod I2C4_FMP {
        /// Offset (23 bits)
        pub const offset: u32 = 23;
        /// Mask (1 bit: 1 << 23)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::I2C3_FMP::RW;
    }
}

/// external interrupt configuration register 1
pub mod EXTICR1 {

    /// EXTI 3 configuration bits
    pub mod EXTI3 {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (4 bits: 0b1111 << 12)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: PA3 pin
            pub const PA3: u32 = 0b0000;

            /// 0b0001: PB3 pin
            pub const PB3: u32 = 0b0001;

            /// 0b0010: PC3 pin
            pub const PC3: u32 = 0b0010;

            /// 0b0011: PD3 pin
            pub const PD3: u32 = 0b0011;

            /// 0b0100: PE3 pin
            pub const PE3: u32 = 0b0100;

            /// 0b0101: PF3 pin
            pub const PF3: u32 = 0b0101;

            /// 0b0110: PG3 pin
            pub const PG3: u32 = 0b0110;

            /// 0b0111: PH3 pin
            pub const PH3: u32 = 0b0111;

            /// 0b1000: PI3 pin
            pub const PI3: u32 = 0b1000;
        }
    }

    /// EXTI 2 configuration bits
    pub mod EXTI2 {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (4 bits: 0b1111 << 8)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: PA2 pin
            pub const PA2: u32 = 0b0000;

            /// 0b0001: PB2 pin
            pub const PB2: u32 = 0b0001;

            /// 0b0010: PC2 pin
            pub const PC2: u32 = 0b0010;

            /// 0b0011: PD2 pin
            pub const PD2: u32 = 0b0011;

            /// 0b0100: PE2 pin
            pub const PE2: u32 = 0b0100;

            /// 0b0101: PF2 pin
            pub const PF2: u32 = 0b0101;

            /// 0b0110: PG2 pin
            pub const PG2: u32 = 0b0110;

            /// 0b0111: PH2 pin
            pub const PH2: u32 = 0b0111;

            /// 0b1000: PI2 pin
            pub const PI2: u32 = 0b1000;
        }
    }

    /// EXTI 1 configuration bits
    pub mod EXTI1 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: PA1 pin
            pub const PA1: u32 = 0b0000;

            /// 0b0001: PB1 pin
            pub const PB1: u32 = 0b0001;

            /// 0b0010: PC1 pin
            pub const PC1: u32 = 0b0010;

            /// 0b0011: PD1 pin
            pub const PD1: u32 = 0b0011;

            /// 0b0100: PE1 pin
            pub const PE1: u32 = 0b0100;

            /// 0b0101: PF1 pin
            pub const PF1: u32 = 0b0101;

            /// 0b0110: PG1 pin
            pub const PG1: u32 = 0b0110;

            /// 0b0111: PH1 pin
            pub const PH1: u32 = 0b0111;

            /// 0b1000: PI1 pin
            pub const PI1: u32 = 0b1000;
        }
    }

    /// EXTI 0 configuration bits
    pub mod EXTI0 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: PA0 pin
            pub const PA0: u32 = 0b0000;

            /// 0b0001: PB0 pin
            pub const PB0: u32 = 0b0001;

            /// 0b0010: PC0 pin
            pub const PC0: u32 = 0b0010;

            /// 0b0011: PD0 pin
            pub const PD0: u32 = 0b0011;

            /// 0b0100: PE0 pin
            pub const PE0: u32 = 0b0100;

            /// 0b0101: PF0 pin
            pub const PF0: u32 = 0b0101;

            /// 0b0110: PG0 pin
            pub const PG0: u32 = 0b0110;

            /// 0b0111: PH0 pin
            pub const PH0: u32 = 0b0111;

            /// 0b1000: PI0 pin
            pub const PI0: u32 = 0b1000;
        }
    }
}

/// external interrupt configuration register 2
pub mod EXTICR2 {

    /// EXTI 7 configuration bits
    pub mod EXTI7 {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (4 bits: 0b1111 << 12)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: PA7 pin
            pub const PA7: u32 = 0b0000;

            /// 0b0001: PB7 pin
            pub const PB7: u32 = 0b0001;

            /// 0b0010: PC7 pin
            pub const PC7: u32 = 0b0010;

            /// 0b0011: PD7 pin
            pub const PD7: u32 = 0b0011;

            /// 0b0100: PE7 pin
            pub const PE7: u32 = 0b0100;

            /// 0b0101: PF7 pin
            pub const PF7: u32 = 0b0101;

            /// 0b0110: PG7 pin
            pub const PG7: u32 = 0b0110;

            /// 0b0111: PH7 pin
            pub const PH7: u32 = 0b0111;

            /// 0b1000: PI7 pin
            pub const PI7: u32 = 0b1000;
        }
    }

    /// EXTI 6 configuration bits
    pub mod EXTI6 {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (4 bits: 0b1111 << 8)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: PA6 pin
            pub const PA6: u32 = 0b0000;

            /// 0b0001: PB6 pin
            pub const PB6: u32 = 0b0001;

            /// 0b0010: PC6 pin
            pub const PC6: u32 = 0b0010;

            /// 0b0011: PD6 pin
            pub const PD6: u32 = 0b0011;

            /// 0b0100: PE6 pin
            pub const PE6: u32 = 0b0100;

            /// 0b0101: PF6 pin
            pub const PF6: u32 = 0b0101;

            /// 0b0110: PG6 pin
            pub const PG6: u32 = 0b0110;

            /// 0b0111: PH6 pin
            pub const PH6: u32 = 0b0111;

            /// 0b1000: PI6 pin
            pub const PI6: u32 = 0b1000;
        }
    }

    /// EXTI 5 configuration bits
    pub mod EXTI5 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: PA5 pin
            pub const PA5: u32 = 0b0000;

            /// 0b0001: PB5 pin
            pub const PB5: u32 = 0b0001;

            /// 0b0010: PC5 pin
            pub const PC5: u32 = 0b0010;

            /// 0b0011: PD5 pin
            pub const PD5: u32 = 0b0011;

            /// 0b0100: PE5 pin
            pub const PE5: u32 = 0b0100;

            /// 0b0101: PF5 pin
            pub const PF5: u32 = 0b0101;

            /// 0b0110: PG5 pin
            pub const PG5: u32 = 0b0110;

            /// 0b0111: PH5 pin
            pub const PH5: u32 = 0b0111;

            /// 0b1000: PI5 pin
            pub const PI5: u32 = 0b1000;
        }
    }

    /// EXTI 4 configuration bits
    pub mod EXTI4 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: PA4 pin
            pub const PA4: u32 = 0b0000;

            /// 0b0001: PB4 pin
            pub const PB4: u32 = 0b0001;

            /// 0b0010: PC4 pin
            pub const PC4: u32 = 0b0010;

            /// 0b0011: PD4 pin
            pub const PD4: u32 = 0b0011;

            /// 0b0100: PE4 pin
            pub const PE4: u32 = 0b0100;

            /// 0b0101: PF4 pin
            pub const PF4: u32 = 0b0101;

            /// 0b0110: PG4 pin
            pub const PG4: u32 = 0b0110;

            /// 0b0111: PH4 pin
            pub const PH4: u32 = 0b0111;

            /// 0b1000: PI4 pin
            pub const PI4: u32 = 0b1000;
        }
    }
}

/// external interrupt configuration register 3
pub mod EXTICR3 {

    /// EXTI 11 configuration bits
    pub mod EXTI11 {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (4 bits: 0b1111 << 12)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: PA11 pin
            pub const PA11: u32 = 0b0000;

            /// 0b0001: PB11 pin
            pub const PB11: u32 = 0b0001;

            /// 0b0010: PC11 pin
            pub const PC11: u32 = 0b0010;

            /// 0b0011: PD11 pin
            pub const PD11: u32 = 0b0011;

            /// 0b0100: PE11 pin
            pub const PE11: u32 = 0b0100;

            /// 0b0101: PF11 pin
            pub const PF11: u32 = 0b0101;

            /// 0b0110: PG11 pin
            pub const PG11: u32 = 0b0110;

            /// 0b0111: PH11 pin
            pub const PH11: u32 = 0b0111;

            /// 0b1000: PI11 pin
            pub const PI11: u32 = 0b1000;
        }
    }

    /// EXTI 10 configuration bits
    pub mod EXTI10 {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (4 bits: 0b1111 << 8)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: PA10 pin
            pub const PA10: u32 = 0b0000;

            /// 0b0001: PB10 pin
            pub const PB10: u32 = 0b0001;

            /// 0b0010: PC10 pin
            pub const PC10: u32 = 0b0010;

            /// 0b0011: PD10 pin
            pub const PD10: u32 = 0b0011;

            /// 0b0100: PE10 pin
            pub const PE10: u32 = 0b0100;

            /// 0b0101: PF10 pin
            pub const PF10: u32 = 0b0101;

            /// 0b0110: PG10 pin
            pub const PG10: u32 = 0b0110;

            /// 0b0111: PH10 pin
            pub const PH10: u32 = 0b0111;

            /// 0b1000: PI10 pin
            pub const PI10: u32 = 0b1000;
        }
    }

    /// EXTI 9 configuration bits
    pub mod EXTI9 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: PA9 pin
            pub const PA9: u32 = 0b0000;

            /// 0b0001: PB9 pin
            pub const PB9: u32 = 0b0001;

            /// 0b0010: PC9 pin
            pub const PC9: u32 = 0b0010;

            /// 0b0011: PD9 pin
            pub const PD9: u32 = 0b0011;

            /// 0b0100: PE9 pin
            pub const PE9: u32 = 0b0100;

            /// 0b0101: PF9 pin
            pub const PF9: u32 = 0b0101;

            /// 0b0110: PG9 pin
            pub const PG9: u32 = 0b0110;

            /// 0b0111: PH9 pin
            pub const PH9: u32 = 0b0111;

            /// 0b1000: PI9 pin
            pub const PI9: u32 = 0b1000;
        }
    }

    /// EXTI 8 configuration bits
    pub mod EXTI8 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: PA8 pin
            pub const PA8: u32 = 0b0000;

            /// 0b0001: PB8 pin
            pub const PB8: u32 = 0b0001;

            /// 0b0010: PC8 pin
            pub const PC8: u32 = 0b0010;

            /// 0b0011: PD8 pin
            pub const PD8: u32 = 0b0011;

            /// 0b0100: PE8 pin
            pub const PE8: u32 = 0b0100;

            /// 0b0101: PF8 pin
            pub const PF8: u32 = 0b0101;

            /// 0b0110: PG8 pin
            pub const PG8: u32 = 0b0110;

            /// 0b0111: PH8 pin
            pub const PH8: u32 = 0b0111;

            /// 0b1000: PI8 pin
            pub const PI8: u32 = 0b1000;
        }
    }
}

/// external interrupt configuration register 4
pub mod EXTICR4 {

    /// EXTI15 configuration bits
    pub mod EXTI15 {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (4 bits: 0b1111 << 12)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: PA15 pin
            pub const PA15: u32 = 0b0000;

            /// 0b0001: PB15 pin
            pub const PB15: u32 = 0b0001;

            /// 0b0010: PC15 pin
            pub const PC15: u32 = 0b0010;

            /// 0b0011: PD15 pin
            pub const PD15: u32 = 0b0011;

            /// 0b0100: PE15 pin
            pub const PE15: u32 = 0b0100;

            /// 0b0101: PF15 pin
            pub const PF15: u32 = 0b0101;

            /// 0b0110: PG15 pin
            pub const PG15: u32 = 0b0110;

            /// 0b0111: PH15 pin
            pub const PH15: u32 = 0b0111;
        }
    }

    /// EXTI14 configuration bits
    pub mod EXTI14 {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (4 bits: 0b1111 << 8)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: PA14 pin
            pub const PA14: u32 = 0b0000;

            /// 0b0001: PB14 pin
            pub const PB14: u32 = 0b0001;

            /// 0b0010: PC14 pin
            pub const PC14: u32 = 0b0010;

            /// 0b0011: PD14 pin
            pub const PD14: u32 = 0b0011;

            /// 0b0100: PE14 pin
            pub const PE14: u32 = 0b0100;

            /// 0b0101: PF14 pin
            pub const PF14: u32 = 0b0101;

            /// 0b0110: PG14 pin
            pub const PG14: u32 = 0b0110;

            /// 0b0111: PH14 pin
            pub const PH14: u32 = 0b0111;
        }
    }

    /// EXTI13 configuration bits
    pub mod EXTI13 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: PA13 pin
            pub const PA13: u32 = 0b0000;

            /// 0b0001: PB13 pin
            pub const PB13: u32 = 0b0001;

            /// 0b0010: PC13 pin
            pub const PC13: u32 = 0b0010;

            /// 0b0011: PD13 pin
            pub const PD13: u32 = 0b0011;

            /// 0b0100: PE13 pin
            pub const PE13: u32 = 0b0100;

            /// 0b0101: PF13 pin
            pub const PF13: u32 = 0b0101;

            /// 0b0110: PG13 pin
            pub const PG13: u32 = 0b0110;

            /// 0b0111: PH13 pin
            pub const PH13: u32 = 0b0111;
        }
    }

    /// EXTI12 configuration bits
    pub mod EXTI12 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: PA12 pin
            pub const PA12: u32 = 0b0000;

            /// 0b0001: PB12 pin
            pub const PB12: u32 = 0b0001;

            /// 0b0010: PC12 pin
            pub const PC12: u32 = 0b0010;

            /// 0b0011: PD12 pin
            pub const PD12: u32 = 0b0011;

            /// 0b0100: PE12 pin
            pub const PE12: u32 = 0b0100;

            /// 0b0101: PF12 pin
            pub const PF12: u32 = 0b0101;

            /// 0b0110: PG12 pin
            pub const PG12: u32 = 0b0110;

            /// 0b0111: PH12 pin
            pub const PH12: u32 = 0b0111;
        }
    }
}

/// SCSR
pub mod SCSR {

    /// SRAM2 busy by erase operation
    pub mod SRAM2BS {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: No SRAM2 erase operation is on going
            pub const NotBusy: u32 = 0b0;

            /// 0b1: SRAM2 erase operation is on going
            pub const Busy: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// SRAM2 Erase
    pub mod SRAM2ER {
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

            /// 0b1: Setting this bit starts a hardware SRAM2 erase operation
            pub const Erase: u32 = 0b1;
        }
    }
}

/// CFGR2
pub mod CFGR2 {

    /// SRAM2 parity error flag
    pub mod SPF {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: No SRAM2 parity error detected
            pub const Cleared: u32 = 0b0;

            /// 0b1: SRAM2 parity error detected
            pub const Set: u32 = 0b1;
        }
    }

    /// ECC Lock
    pub mod ECCL {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values
        pub mod W {

            /// 0b0: ECC error disconnected from TIM1/8/15/16/17 Break input
            pub const Disconnected: u32 = 0b0;

            /// 0b1: ECC error connected to TIM1/8/15/16/17 Break input
            pub const Connected: u32 = 0b1;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// PVD lock enable bit
    pub mod PVDL {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values
        pub mod W {

            /// 0b0: PVD interrupt disconnected from TIM1/8/15/16/17 Break input. PVDE and PLS\[2:0\] bits can be programmed by the application
            pub const Disconnected: u32 = 0b0;

            /// 0b1: PVD interrupt connected to TIM1/8/15/16/17 Break input, PVDE and PLS\[2:0\] bits are read only
            pub const Connected: u32 = 0b1;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// SRAM2 parity lock bit
    pub mod SPL {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values
        pub mod W {

            /// 0b0: SRAM2 parity error signal disconnected from TIM1/8/15/16/17 Break inputs
            pub const Disconnected: u32 = 0b0;

            /// 0b1: SRAM2 parity error signal connected to TIM1/8/15/16/17 Break inputs
            pub const Connected: u32 = 0b1;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Cortex-M4 LOCKUP (Hardfault) output enable bit
    pub mod CLL {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values
        pub mod W {

            /// 0b0: Cortex®-M4 LOCKUP output disconnected from TIM1/8/15/16/17 Break inputs
            pub const Disconnected: u32 = 0b0;

            /// 0b1: Cortex®-M4 LOCKUP output connected to TIM1/8/15/16/17 Break inputs
            pub const Connected: u32 = 0b1;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// SWPR
pub mod SWPR {

    /// SRAM2 page 31 write protection
    pub mod P31WP {
        /// Offset (31 bits)
        pub const offset: u32 = 31;
        /// Mask (1 bit: 1 << 31)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values
        pub mod W {

            /// 0b0: Write protection of SRAM2 page x is disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Write protection of SRAM2 page x is enabled
            pub const Enabled: u32 = 0b1;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// P30WP
    pub mod P30WP {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (1 bit: 1 << 30)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::P31WP::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// P29WP
    pub mod P29WP {
        /// Offset (29 bits)
        pub const offset: u32 = 29;
        /// Mask (1 bit: 1 << 29)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::P31WP::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// P28WP
    pub mod P28WP {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (1 bit: 1 << 28)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::P31WP::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// P27WP
    pub mod P27WP {
        /// Offset (27 bits)
        pub const offset: u32 = 27;
        /// Mask (1 bit: 1 << 27)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::P31WP::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// P26WP
    pub mod P26WP {
        /// Offset (26 bits)
        pub const offset: u32 = 26;
        /// Mask (1 bit: 1 << 26)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::P31WP::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// P25WP
    pub mod P25WP {
        /// Offset (25 bits)
        pub const offset: u32 = 25;
        /// Mask (1 bit: 1 << 25)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::P31WP::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// P24WP
    pub mod P24WP {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (1 bit: 1 << 24)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::P31WP::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// P23WP
    pub mod P23WP {
        /// Offset (23 bits)
        pub const offset: u32 = 23;
        /// Mask (1 bit: 1 << 23)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::P31WP::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// P22WP
    pub mod P22WP {
        /// Offset (22 bits)
        pub const offset: u32 = 22;
        /// Mask (1 bit: 1 << 22)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::P31WP::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// P21WP
    pub mod P21WP {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (1 bit: 1 << 21)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::P31WP::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// P20WP
    pub mod P20WP {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (1 bit: 1 << 20)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::P31WP::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// P19WP
    pub mod P19WP {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (1 bit: 1 << 19)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::P31WP::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// P18WP
    pub mod P18WP {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (1 bit: 1 << 18)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::P31WP::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// P17WP
    pub mod P17WP {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::P31WP::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// P16WP
    pub mod P16WP {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::P31WP::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// P15WP
    pub mod P15WP {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::P31WP::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// P14WP
    pub mod P14WP {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::P31WP::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// P13WP
    pub mod P13WP {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::P31WP::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// P12WP
    pub mod P12WP {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::P31WP::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// P11WP
    pub mod P11WP {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::P31WP::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// P10WP
    pub mod P10WP {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::P31WP::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// P9WP
    pub mod P9WP {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::P31WP::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// P8WP
    pub mod P8WP {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::P31WP::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// P7WP
    pub mod P7WP {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::P31WP::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// P6WP
    pub mod P6WP {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::P31WP::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// P5WP
    pub mod P5WP {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::P31WP::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// P4WP
    pub mod P4WP {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::P31WP::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// P3WP
    pub mod P3WP {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::P31WP::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// P2WP
    pub mod P2WP {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::P31WP::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// P1WP
    pub mod P1WP {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::P31WP::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// P0WP
    pub mod P0WP {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::P31WP::W;
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// SKR
pub mod SKR {

    /// SRAM2 write protection key for software erase
    pub mod KEY {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (8 bits: 0xff << 0)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values
        pub mod W {

            /// 0b01010011: 2. Write 0x53 into Key\[7:0\]
            pub const Key2: u32 = 0b01010011;

            /// 0b11001010: 1. Write 0xCA into Key\[7:0\]
            pub const Key1: u32 = 0b11001010;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// write protection register 2
pub mod SWPR2 {

    /// SRAM2 page x write protection
    pub mod P63WP {
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

            /// 0b0: Write protection of SRAM2 page x is disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Write protection of SRAM2 page x is enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// SRAM2 page x write protection
    pub mod P62WP {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (1 bit: 1 << 30)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::P63WP::RW;
    }

    /// SRAM2 page x write protection
    pub mod P61WP {
        /// Offset (29 bits)
        pub const offset: u32 = 29;
        /// Mask (1 bit: 1 << 29)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::P63WP::RW;
    }

    /// SRAM2 page x write protection
    pub mod P60WP {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (1 bit: 1 << 28)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::P63WP::RW;
    }

    /// SRAM2 page x write protection
    pub mod P59WP {
        /// Offset (27 bits)
        pub const offset: u32 = 27;
        /// Mask (1 bit: 1 << 27)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::P63WP::RW;
    }

    /// SRAM2 page x write protection
    pub mod P58WP {
        /// Offset (26 bits)
        pub const offset: u32 = 26;
        /// Mask (1 bit: 1 << 26)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::P63WP::RW;
    }

    /// SRAM2 page x write protection
    pub mod P57WP {
        /// Offset (25 bits)
        pub const offset: u32 = 25;
        /// Mask (1 bit: 1 << 25)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::P63WP::RW;
    }

    /// SRAM2 page x write protection
    pub mod P56WP {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (1 bit: 1 << 24)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::P63WP::RW;
    }

    /// SRAM2 page x write protection
    pub mod P55WP {
        /// Offset (23 bits)
        pub const offset: u32 = 23;
        /// Mask (1 bit: 1 << 23)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::P63WP::RW;
    }

    /// SRAM2 page x write protection
    pub mod P54WP {
        /// Offset (22 bits)
        pub const offset: u32 = 22;
        /// Mask (1 bit: 1 << 22)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::P63WP::RW;
    }

    /// SRAM2 page x write protection
    pub mod P53WP {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (1 bit: 1 << 21)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::P63WP::RW;
    }

    /// SRAM2 page x write protection
    pub mod P52WP {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (1 bit: 1 << 20)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::P63WP::RW;
    }

    /// SRAM2 page x write protection
    pub mod P51WP {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (1 bit: 1 << 19)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::P63WP::RW;
    }

    /// SRAM2 page x write protection
    pub mod P50WP {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (1 bit: 1 << 18)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::P63WP::RW;
    }

    /// SRAM2 page x write protection
    pub mod P49WP {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::P63WP::RW;
    }

    /// SRAM2 page x write protection
    pub mod P48WP {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::P63WP::RW;
    }

    /// SRAM2 page x write protection
    pub mod P47WP {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::P63WP::RW;
    }

    /// SRAM2 page x write protection
    pub mod P46WP {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::P63WP::RW;
    }

    /// SRAM2 page x write protection
    pub mod P45WP {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::P63WP::RW;
    }

    /// SRAM2 page x write protection
    pub mod P44WP {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::P63WP::RW;
    }

    /// SRAM2 page x write protection
    pub mod P43WP {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::P63WP::RW;
    }

    /// SRAM2 page x write protection
    pub mod P42WP {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::P63WP::RW;
    }

    /// SRAM2 page x write protection
    pub mod P41WP {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::P63WP::RW;
    }

    /// SRAM2 page x write protection
    pub mod P40WP {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::P63WP::RW;
    }

    /// SRAM2 page x write protection
    pub mod P39WP {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::P63WP::RW;
    }

    /// SRAM2 page x write protection
    pub mod P38WP {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::P63WP::RW;
    }

    /// SRAM2 page x write protection
    pub mod P37WP {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::P63WP::RW;
    }

    /// SRAM2 page x write protection
    pub mod P36WP {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::P63WP::RW;
    }

    /// SRAM2 page x write protection
    pub mod P35WP {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::P63WP::RW;
    }

    /// SRAM2 page x write protection
    pub mod P34WP {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::P63WP::RW;
    }

    /// SRAM2 page x write protection
    pub mod P33WP {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::P63WP::RW;
    }

    /// SRAM2 page x write protection
    pub mod P32WP {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::P63WP::RW;
    }
}
#[repr(C)]
pub struct RegisterBlock {
    /// memory remap register
    pub MEMRMP: RWRegister<u32>,

    /// configuration register 1
    pub CFGR1: RWRegister<u32>,

    /// external interrupt configuration register 1
    pub EXTICR1: RWRegister<u32>,

    /// external interrupt configuration register 2
    pub EXTICR2: RWRegister<u32>,

    /// external interrupt configuration register 3
    pub EXTICR3: RWRegister<u32>,

    /// external interrupt configuration register 4
    pub EXTICR4: RWRegister<u32>,

    /// SCSR
    pub SCSR: RWRegister<u32>,

    /// CFGR2
    pub CFGR2: RWRegister<u32>,

    /// SWPR
    pub SWPR: WORegister<u32>,

    /// SKR
    pub SKR: WORegister<u32>,

    /// write protection register 2
    pub SWPR2: RWRegister<u32>,
}
pub struct ResetValues {
    pub MEMRMP: u32,
    pub CFGR1: u32,
    pub EXTICR1: u32,
    pub EXTICR2: u32,
    pub EXTICR3: u32,
    pub EXTICR4: u32,
    pub SCSR: u32,
    pub CFGR2: u32,
    pub SWPR: u32,
    pub SKR: u32,
    pub SWPR2: u32,
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

/// Access functions for the SYSCFG peripheral instance
pub mod SYSCFG {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x40010000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in SYSCFG
    pub const reset: ResetValues = ResetValues {
        MEMRMP: 0x00000000,
        CFGR1: 0x7C000001,
        EXTICR1: 0x00000000,
        EXTICR2: 0x00000000,
        EXTICR3: 0x00000000,
        EXTICR4: 0x00000000,
        SCSR: 0x00000000,
        CFGR2: 0x00000000,
        SWPR: 0x00000000,
        SKR: 0x00000000,
        SWPR2: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut SYSCFG_TAKEN: bool = false;

    /// Safe access to SYSCFG
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
            if SYSCFG_TAKEN {
                None
            } else {
                SYSCFG_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to SYSCFG
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if SYSCFG_TAKEN && inst.addr == INSTANCE.addr {
                SYSCFG_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal SYSCFG
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        SYSCFG_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to SYSCFG
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const SYSCFG: *const RegisterBlock = 0x40010000 as *const _;
