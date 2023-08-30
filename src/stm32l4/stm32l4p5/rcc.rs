#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Reset and clock control

use crate::{RORegister, RWRegister, WORegister};
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// Clock control register
pub mod CR {

    /// SAI2 PLL clock ready flag
    pub mod PLLSAI2RDY {
        /// Offset (29 bits)
        pub const offset: u32 = 29;
        /// Mask (1 bit: 1 << 29)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: PLLSAI2 unlocked
            pub const Unlocked: u32 = 0b0;

            /// 0b1: PLLSAI2 locked
            pub const Locked: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// SAI2 PLL enable
    pub mod PLLSAI2ON {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (1 bit: 1 << 28)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: PLLSAI2 OFF
            pub const Disabled: u32 = 0b0;

            /// 0b1: PLLSAI2 ON
            pub const Enabled: u32 = 0b1;
        }
    }

    /// SAI1 PLL clock ready flag
    pub mod PLLSAI1RDY {
        /// Offset (27 bits)
        pub const offset: u32 = 27;
        /// Mask (1 bit: 1 << 27)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: PLLSAI1 unlocked
            pub const Unlocked: u32 = 0b0;

            /// 0b1: PLLSAI1 locked
            pub const Locked: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// SAI1 PLL enable
    pub mod PLLSAI1ON {
        /// Offset (26 bits)
        pub const offset: u32 = 26;
        /// Mask (1 bit: 1 << 26)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: PLLSAI1 OFF
            pub const Disabled: u32 = 0b0;

            /// 0b1: PLLSAI1 ON
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Main PLL clock ready flag
    pub mod PLLRDY {
        /// Offset (25 bits)
        pub const offset: u32 = 25;
        /// Mask (1 bit: 1 << 25)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: PLL unlocked
            pub const Unlocked: u32 = 0b0;

            /// 0b1: PLL locked
            pub const Locked: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Main PLL enable
    pub mod PLLON {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (1 bit: 1 << 24)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: PLL OFF
            pub const Disabled: u32 = 0b0;

            /// 0b1: PLL ON
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Clock security system enable
    pub mod CSSON {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (1 bit: 1 << 19)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values
        pub mod W {

            /// 0b0: Clock security system OFF (clock detector OFF)
            pub const Disabled: u32 = 0b0;

            /// 0b1: Clock security system ON (Clock detector ON if the HSE oscillator is stable, OFF if not)
            pub const Enabled: u32 = 0b1;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// HSE crystal oscillator bypass
    pub mod HSEBYP {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (1 bit: 1 << 18)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: HSE crystal oscillator not bypassed
            pub const NotBypassed: u32 = 0b0;

            /// 0b1: HSE crystal oscillator bypassed with external clock
            pub const Bypassed: u32 = 0b1;
        }
    }

    /// HSE clock ready flag
    pub mod HSERDY {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: HSE oscillator not ready
            pub const NotReady: u32 = 0b0;

            /// 0b1: HSE oscillator ready
            pub const Ready: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// HSE clock enable
    pub mod HSEON {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: HSE oscillator OFF
            pub const Disabled: u32 = 0b0;

            /// 0b1: HSE oscillator ON
            pub const Enabled: u32 = 0b1;
        }
    }

    /// HSI automatic start from Stop
    pub mod HSIASFS {
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

            /// 0b0: HSI16 oscillator is not enabled by hardware when exiting Stop mode with MSI as wakeup clock
            pub const Disabled: u32 = 0b0;

            /// 0b1: HSI16 oscillator is enabled by hardware when exiting Stop mode with MSI as wakeup clock
            pub const Enabled: u32 = 0b1;
        }
    }

    /// HSI clock ready flag
    pub mod HSIRDY {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: HSI16 oscillator not ready
            pub const NotReady: u32 = 0b0;

            /// 0b1: HSI16 oscillator ready
            pub const Ready: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// HSI always enable for peripheral kernels
    pub mod HSIKERON {
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

            /// 0b0: No effect on HSI16 oscillator
            pub const Disabled: u32 = 0b0;

            /// 0b1: HSI16 oscillator is forced ON even in Stop mode
            pub const Enabled: u32 = 0b1;
        }
    }

    /// HSI clock enable
    pub mod HSION {
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

            /// 0b0: HSI16 oscillator OFF
            pub const Disabled: u32 = 0b0;

            /// 0b1: HSI16 oscillator ON
            pub const Enabled: u32 = 0b1;
        }
    }

    /// MSI clock ranges
    pub mod MSIRANGE {
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

            /// 0b0000: range 0 around 100 kHz
            pub const Range100K: u32 = 0b0000;

            /// 0b0001: range 1 around 200 kHz
            pub const Range200K: u32 = 0b0001;

            /// 0b0010: range 2 around 400 kHz
            pub const Range400K: u32 = 0b0010;

            /// 0b0011: range 3 around 800 kHz
            pub const Range800K: u32 = 0b0011;

            /// 0b0100: range 4 around 1 MHz
            pub const Range1M: u32 = 0b0100;

            /// 0b0101: range 5 around 2 MHz
            pub const Range2M: u32 = 0b0101;

            /// 0b0110: range 6 around 4 MHz
            pub const Range4M: u32 = 0b0110;

            /// 0b0111: range 7 around 8 MHz
            pub const Range8M: u32 = 0b0111;

            /// 0b1000: range 8 around 16 MHz
            pub const Range16M: u32 = 0b1000;

            /// 0b1001: range 9 around 24 MHz
            pub const Range24M: u32 = 0b1001;

            /// 0b1010: range 10 around 32 MHz
            pub const Range32M: u32 = 0b1010;

            /// 0b1011: range 11 around 48 MHz
            pub const Range48M: u32 = 0b1011;
        }
    }

    /// MSI clock range selection
    pub mod MSIRGSEL {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values
        pub mod W {

            /// 0b0: MSI Range is provided by MSISRANGE\[3:0\] in RCC_CSR register
            pub const CSR: u32 = 0b0;

            /// 0b1: MSI Range is provided by MSIRANGE\[3:0\] in the RCC_CR register
            pub const CR: u32 = 0b1;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// MSI clock PLL enable
    pub mod MSIPLLEN {
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

            /// 0b0: MSI PLL OFF
            pub const Disabled: u32 = 0b0;

            /// 0b1: MSI PLL ON
            pub const Enabled: u32 = 0b1;
        }
    }

    /// MSI clock ready flag
    pub mod MSIRDY {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: MSI oscillator not ready
            pub const NotReady: u32 = 0b0;

            /// 0b1: MSI oscillator ready
            pub const Ready: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// MSI clock enable
    pub mod MSION {
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

            /// 0b0: MSI oscillator OFF
            pub const Disabled: u32 = 0b0;

            /// 0b1: MSI oscillator ON
            pub const Enabled: u32 = 0b1;
        }
    }
}

/// Internal clock sources calibration register
pub mod ICSCR {

    /// HSI clock trimming
    pub mod HSITRIM {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (7 bits: 0x7f << 24)
        pub const mask: u32 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// HSI clock calibration
    pub mod HSICAL {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (8 bits: 0xff << 16)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// MSI clock trimming
    pub mod MSITRIM {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (8 bits: 0xff << 8)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// MSI clock calibration
    pub mod MSICAL {
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

/// Clock configuration register
pub mod CFGR {

    /// Microcontroller clock output prescaler
    pub mod MCOPRE {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (3 bits: 0b111 << 28)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values
        pub mod R {

            /// 0b000: MCO is divided by 1
            pub const Divider1: u32 = 0b000;

            /// 0b001: MCO is divided by 2
            pub const Divider2: u32 = 0b001;

            /// 0b010: MCO is divided by 4
            pub const Divider4: u32 = 0b010;

            /// 0b011: MCO is divided by 8
            pub const Divider8: u32 = 0b011;

            /// 0b100: MCO is divided by 16
            pub const Divider16: u32 = 0b100;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Microcontroller clock output
    pub mod MCOSEL {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (4 bits: 0b1111 << 24)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: MCO output disabled, no clock on MCO
            pub const Disabled: u32 = 0b0000;

            /// 0b0001: SYSCLK system clock selected
            pub const SYSCLK: u32 = 0b0001;

            /// 0b0010: MSI clock selected.
            pub const MSI: u32 = 0b0010;

            /// 0b0011: HSI16 clock selected.
            pub const HSI16: u32 = 0b0011;

            /// 0b0100: HSE clock selected
            pub const HSE: u32 = 0b0100;

            /// 0b0101: Main PLL clock selected
            pub const MainPLL: u32 = 0b0101;

            /// 0b0110: LSI clock selected
            pub const LSI: u32 = 0b0110;

            /// 0b0111: LSE clock selected
            pub const LSE: u32 = 0b0111;

            /// 0b1000: Internal HSI48 clock selected
            pub const HSI48: u32 = 0b1000;
        }
    }

    /// Wakeup from Stop and CSS backup clock selection
    pub mod STOPWUCK {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: MSI oscillator selected as wakeup from stop clock and CSS backup clock
            pub const MSI: u32 = 0b0;

            /// 0b1: HSI16 oscillator selected as wakeup from stop clock and CSS backup clock
            pub const HSI16: u32 = 0b1;
        }
    }

    /// APB high-speed prescaler (APB2)
    pub mod PPRE2 {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (3 bits: 0b111 << 11)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: HCLK not divided
            pub const Div1: u32 = 0b000;

            /// 0b100: HCLK divided by 2
            pub const Div2: u32 = 0b100;

            /// 0b101: HCLK divided by 4
            pub const Div4: u32 = 0b101;

            /// 0b110: HCLK divided by 8
            pub const Div8: u32 = 0b110;

            /// 0b111: HCLK divided by 16
            pub const Div16: u32 = 0b111;
        }
    }

    /// PB low-speed prescaler (APB1)
    pub mod PPRE1 {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (3 bits: 0b111 << 8)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PPRE2::RW;
    }

    /// AHB prescaler
    pub mod HPRE {
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

            /// 0b0000: SYSCLK not divided
            pub const Div1: u32 = 0b0000;

            /// 0b1000: SYSCLK divided by 2
            pub const Div2: u32 = 0b1000;

            /// 0b1001: SYSCLK divided by 4
            pub const Div4: u32 = 0b1001;

            /// 0b1010: SYSCLK divided by 8
            pub const Div8: u32 = 0b1010;

            /// 0b1011: SYSCLK divided by 16
            pub const Div16: u32 = 0b1011;

            /// 0b1100: SYSCLK divided by 64
            pub const Div64: u32 = 0b1100;

            /// 0b1101: SYSCLK divided by 128
            pub const Div128: u32 = 0b1101;

            /// 0b1110: SYSCLK divided by 256
            pub const Div256: u32 = 0b1110;

            /// 0b1111: SYSCLK divided by 512
            pub const Div512: u32 = 0b1111;
        }
    }

    /// System clock switch status
    pub mod SWS {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (2 bits: 0b11 << 2)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values
        pub mod R {

            /// 0b00: MSI oscillator used as system clock
            pub const MSI: u32 = 0b00;

            /// 0b01: HSI16 oscillator used as system clock
            pub const HSI16: u32 = 0b01;

            /// 0b10: HSE used as system clock
            pub const HSE: u32 = 0b10;

            /// 0b11: PLL used as system clock
            pub const PLL: u32 = 0b11;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// System clock switch
    pub mod SW {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (2 bits: 0b11 << 0)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: MSI selected as system clock
            pub const MSI: u32 = 0b00;

            /// 0b01: HSI16 selected as system clock
            pub const HSI16: u32 = 0b01;

            /// 0b10: HSE selected as system clock
            pub const HSE: u32 = 0b10;

            /// 0b11: PLL selected as system clock
            pub const PLL: u32 = 0b11;
        }
    }
}

/// PLL configuration register
pub mod PLLCFGR {

    /// Main PLL division factor for PLLSAI2CLK
    pub mod PLLPDIV {
        /// Offset (27 bits)
        pub const offset: u32 = 27;
        /// Mask (5 bits: 0b11111 << 27)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00000: PLLSAI3CLK is controlled by the bit PLLP
            pub const PLLP: u32 = 0b00000;

            /// 0b00010: PLLSAI3CLK = VCO / 2
            pub const Div2: u32 = 0b00010;

            /// 0b00011: PLLSAI3CLK = VCO / 3
            pub const Div3: u32 = 0b00011;

            /// 0b00100: PLLSAI3CLK = VCO / 4
            pub const Div4: u32 = 0b00100;

            /// 0b00101: PLLSAI3CLK = VCO / 5
            pub const Div5: u32 = 0b00101;

            /// 0b00110: PLLSAI3CLK = VCO / 6
            pub const Div6: u32 = 0b00110;

            /// 0b00111: PLLSAI3CLK = VCO / 7
            pub const Div7: u32 = 0b00111;

            /// 0b01000: PLLSAI3CLK = VCO / 8
            pub const Div8: u32 = 0b01000;

            /// 0b01001: PLLSAI3CLK = VCO / 9
            pub const Div9: u32 = 0b01001;

            /// 0b01010: PLLSAI3CLK = VCO / 10
            pub const Div10: u32 = 0b01010;

            /// 0b01011: PLLSAI3CLK = VCO / 11
            pub const Div11: u32 = 0b01011;

            /// 0b01100: PLLSAI3CLK = VCO / 12
            pub const Div12: u32 = 0b01100;

            /// 0b01101: PLLSAI3CLK = VCO / 13
            pub const Div13: u32 = 0b01101;

            /// 0b01110: PLLSAI3CLK = VCO / 14
            pub const Div14: u32 = 0b01110;

            /// 0b01111: PLLSAI3CLK = VCO / 15
            pub const Div15: u32 = 0b01111;

            /// 0b10000: PLLSAI3CLK = VCO / 16
            pub const Div16: u32 = 0b10000;

            /// 0b10001: PLLSAI3CLK = VCO / 17
            pub const Div17: u32 = 0b10001;

            /// 0b10010: PLLSAI3CLK = VCO / 18
            pub const Div18: u32 = 0b10010;

            /// 0b10011: PLLSAI3CLK = VCO / 19
            pub const Div19: u32 = 0b10011;

            /// 0b10100: PLLSAI3CLK = VCO / 20
            pub const Div20: u32 = 0b10100;

            /// 0b10101: PLLSAI3CLK = VCO / 21
            pub const Div21: u32 = 0b10101;

            /// 0b10110: PLLSAI3CLK = VCO / 22
            pub const Div22: u32 = 0b10110;

            /// 0b10111: PLLSAI3CLK = VCO / 23
            pub const Div23: u32 = 0b10111;

            /// 0b11000: PLLSAI3CLK = VCO / 24
            pub const Div24: u32 = 0b11000;

            /// 0b11001: PLLSAI3CLK = VCO / 25
            pub const Div25: u32 = 0b11001;

            /// 0b11010: PLLSAI3CLK = VCO / 26
            pub const Div26: u32 = 0b11010;

            /// 0b11011: PLLSAI3CLK = VCO / 27
            pub const Div27: u32 = 0b11011;

            /// 0b11100: PLLSAI3CLK = VCO / 28
            pub const Div28: u32 = 0b11100;

            /// 0b11101: PLLSAI3CLK = VCO / 29
            pub const Div29: u32 = 0b11101;

            /// 0b11110: PLLSAI3CLK = VCO / 30
            pub const Div30: u32 = 0b11110;

            /// 0b11111: PLLSAI3CLK = VCO / 31
            pub const Div31: u32 = 0b11111;
        }
    }

    /// Main PLL division factor for PLLCLK (system clock)
    pub mod PLLR {
        /// Offset (25 bits)
        pub const offset: u32 = 25;
        /// Mask (2 bits: 0b11 << 25)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: PLLx = 2
            pub const Div2: u32 = 0b00;

            /// 0b01: PLLx = 4
            pub const Div4: u32 = 0b01;

            /// 0b10: PLLx = 6
            pub const Div6: u32 = 0b10;

            /// 0b11: PLLx = 8
            pub const Div8: u32 = 0b11;
        }
    }

    /// Main PLL PLLCLK output enable
    pub mod PLLREN {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (1 bit: 1 << 24)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: PLLCLK output disable
            pub const Disabled: u32 = 0b0;

            /// 0b1: PLLCLK output enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Main PLL division factor for PLLUSB1CLK(48 MHz clock)
    pub mod PLLQ {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (2 bits: 0b11 << 21)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PLLR::RW;
    }

    /// Main PLL PLLUSB1CLK output enable
    pub mod PLLQEN {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (1 bit: 1 << 20)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: PLL48M1CLK output disable
            pub const Disabled: u32 = 0b0;

            /// 0b1: PLL48M1CLK output enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Main PLL division factor for PLLSAI3CLK (SAI1 and SAI2 clock)
    pub mod PLLP {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: PLLP = 7
            pub const Div7: u32 = 0b0;

            /// 0b1: PLLP = 17
            pub const Div17: u32 = 0b1;
        }
    }

    /// Main PLL PLLSAI3CLK output enable
    pub mod PLLPEN {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: PLLSAI3CLK output disable
            pub const Disabled: u32 = 0b0;

            /// 0b1: PLLSAI3CLK output enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Main PLL multiplication factor for VCO
    pub mod PLLN {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (7 bits: 0x7f << 8)
        pub const mask: u32 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Division factor for the main PLL and audio PLL (PLLSAI1 and PLLSAI2) input clock
    pub mod PLLM {
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

            /// 0b0000: PLLM = 1
            pub const Div1: u32 = 0b0000;

            /// 0b0001: PLLM = 2
            pub const Div2: u32 = 0b0001;

            /// 0b0010: PLLM = 3
            pub const Div3: u32 = 0b0010;

            /// 0b0011: PLLM = 4
            pub const Div4: u32 = 0b0011;

            /// 0b0100: PLLM = 5
            pub const Div5: u32 = 0b0100;

            /// 0b0101: PLLM = 6
            pub const Div6: u32 = 0b0101;

            /// 0b0110: PLLM = 7
            pub const Div7: u32 = 0b0110;

            /// 0b0111: PLLM = 8
            pub const Div8: u32 = 0b0111;

            /// 0b1000: PLLM = 9
            pub const Div9: u32 = 0b1000;

            /// 0b1001: PLLM = 11
            pub const Div10: u32 = 0b1001;

            /// 0b1010: PLLM = 12
            pub const Div11: u32 = 0b1010;

            /// 0b1011: PLLM = 13
            pub const Div12: u32 = 0b1011;

            /// 0b1100: PLLM = 13
            pub const Div13: u32 = 0b1100;

            /// 0b1101: PLLM = 14
            pub const Div14: u32 = 0b1101;

            /// 0b1110: PLLM = 15
            pub const Div15: u32 = 0b1110;

            /// 0b1111: PLLM = 16
            pub const Div16: u32 = 0b1111;
        }
    }

    /// Main PLL, PLLSAI1 and PLLSAI2 entry clock source
    pub mod PLLSRC {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (2 bits: 0b11 << 0)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: No clock sent to PLL
            pub const NoClock: u32 = 0b00;

            /// 0b01: MSI clock selected as PLL clock entry
            pub const MSI: u32 = 0b01;

            /// 0b10: HSI16 clock selected as PLL clock entry
            pub const HSI16: u32 = 0b10;

            /// 0b11: HSE clock selected as PLL clock entry
            pub const HSE: u32 = 0b11;
        }
    }
}

/// PLLSAI1 configuration register
pub mod PLLSAI1CFGR {

    /// PLLSAI1 division factor for PLLSAI1CLK
    pub mod PLLSAI1PDIV {
        /// Offset (27 bits)
        pub const offset: u32 = 27;
        /// Mask (5 bits: 0b11111 << 27)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00000: PLLSAI1CLK is controlled by the bit PLLSAI1P
            pub const PLLSAI1P: u32 = 0b00000;

            /// 0b00010: PLLSAI1CLK = VCOSAI / 2
            pub const Div2: u32 = 0b00010;

            /// 0b00011: PLLSAI1CLK = VCOSAI / 3
            pub const Div3: u32 = 0b00011;

            /// 0b00100: PLLSAI1CLK = VCOSAI / 4
            pub const Div4: u32 = 0b00100;

            /// 0b00101: PLLSAI1CLK = VCOSAI / 5
            pub const Div5: u32 = 0b00101;

            /// 0b00110: PLLSAI1CLK = VCOSAI / 6
            pub const Div6: u32 = 0b00110;

            /// 0b00111: PLLSAI1CLK = VCOSAI / 7
            pub const Div7: u32 = 0b00111;

            /// 0b01000: PLLSAI1CLK = VCOSAI / 8
            pub const Div8: u32 = 0b01000;

            /// 0b01001: PLLSAI1CLK = VCOSAI / 9
            pub const Div9: u32 = 0b01001;

            /// 0b01010: PLLSAI1CLK = VCOSAI / 10
            pub const Div10: u32 = 0b01010;

            /// 0b01011: PLLSAI1CLK = VCOSAI / 11
            pub const Div11: u32 = 0b01011;

            /// 0b01100: PLLSAI1CLK = VCOSAI / 12
            pub const Div12: u32 = 0b01100;

            /// 0b01101: PLLSAI1CLK = VCOSAI / 13
            pub const Div13: u32 = 0b01101;

            /// 0b01110: PLLSAI1CLK = VCOSAI / 14
            pub const Div14: u32 = 0b01110;

            /// 0b01111: PLLSAI1CLK = VCOSAI / 15
            pub const Div15: u32 = 0b01111;

            /// 0b10000: PLLSAI1CLK = VCOSAI / 16
            pub const Div16: u32 = 0b10000;

            /// 0b10001: PLLSAI1CLK = VCOSAI / 17
            pub const Div17: u32 = 0b10001;

            /// 0b10010: PLLSAI1CLK = VCOSAI / 18
            pub const Div18: u32 = 0b10010;

            /// 0b10011: PLLSAI1CLK = VCOSAI / 19
            pub const Div19: u32 = 0b10011;

            /// 0b10100: PLLSAI1CLK = VCOSAI / 20
            pub const Div20: u32 = 0b10100;

            /// 0b10101: PLLSAI1CLK = VCOSAI / 21
            pub const Div21: u32 = 0b10101;

            /// 0b10110: PLLSAI1CLK = VCOSAI / 22
            pub const Div22: u32 = 0b10110;

            /// 0b10111: PLLSAI1CLK = VCOSAI / 23
            pub const Div23: u32 = 0b10111;

            /// 0b11000: PLLSAI1CLK = VCOSAI / 24
            pub const Div24: u32 = 0b11000;

            /// 0b11001: PLLSAI1CLK = VCOSAI / 25
            pub const Div25: u32 = 0b11001;

            /// 0b11010: PLLSAI1CLK = VCOSAI / 26
            pub const Div26: u32 = 0b11010;

            /// 0b11011: PLLSAI1CLK = VCOSAI / 27
            pub const Div27: u32 = 0b11011;

            /// 0b11100: PLLSAI1CLK = VCOSAI / 28
            pub const Div28: u32 = 0b11100;

            /// 0b11101: PLLSAI1CLK = VCOSAI / 29
            pub const Div29: u32 = 0b11101;

            /// 0b11110: PLLSAI1CLK = VCOSAI / 30
            pub const Div30: u32 = 0b11110;

            /// 0b11111: PLLSAI1CLK = VCOSAI / 31
            pub const Div31: u32 = 0b11111;
        }
    }

    /// PLLSAI1 division factor for PLLADC1CLK (ADC clock)
    pub mod PLLSAI1R {
        /// Offset (25 bits)
        pub const offset: u32 = 25;
        /// Mask (2 bits: 0b11 << 25)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: PLLSAI1x = 2
            pub const Div2: u32 = 0b00;

            /// 0b01: PLLSAI1x = 4
            pub const Div4: u32 = 0b01;

            /// 0b10: PLLSAI1x = 6
            pub const Div6: u32 = 0b10;

            /// 0b11: PLLSAI1x = 8
            pub const Div8: u32 = 0b11;
        }
    }

    /// PLLSAI1 PLLADC1CLK output enable
    pub mod PLLSAI1REN {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (1 bit: 1 << 24)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: PLLADC1CLK output disable
            pub const Disabled: u32 = 0b0;

            /// 0b1: PLLADC1CLK output enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// SAI1PLL division factor for PLLUSB2CLK (48 MHz clock)
    pub mod PLLSAI1Q {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (2 bits: 0b11 << 21)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PLLSAI1R::RW;
    }

    /// SAI1PLL PLLUSB2CLK output enable
    pub mod PLLSAI1QEN {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (1 bit: 1 << 20)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: PLL48M2CLK output disable
            pub const Disabled: u32 = 0b0;

            /// 0b1: PLL48M2CLK output enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// SAI1PLL division factor for PLLSAI1CLK (SAI1 or SAI2 clock)
    pub mod PLLSAI1P {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: PLLSAI1P = 7
            pub const Div7: u32 = 0b0;

            /// 0b1: PLLSAI1P = 17
            pub const Div17: u32 = 0b1;
        }
    }

    /// SAI1PLL PLLSAI1CLK output enable
    pub mod PLLSAI1PEN {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: PLLSAI1CLK output disable
            pub const Disabled: u32 = 0b0;

            /// 0b1: PLLSAI1CLK output enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// SAI1PLL multiplication factor for VCO
    pub mod PLLSAI1N {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (7 bits: 0x7f << 8)
        pub const mask: u32 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Division factor for PLLSAI1 input clock
    pub mod PLLSAI1M {
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

            /// 0b0000: PLLSAI1M = 1
            pub const Div1: u32 = 0b0000;

            /// 0b0001: PLLSAI1M = 2
            pub const Div2: u32 = 0b0001;

            /// 0b0010: PLLSAI1M = 3
            pub const Div3: u32 = 0b0010;

            /// 0b0011: PLLSAI1M = 4
            pub const Div4: u32 = 0b0011;

            /// 0b0100: PLLSAI1M = 5
            pub const Div5: u32 = 0b0100;

            /// 0b0101: PLLSAI1M = 6
            pub const Div6: u32 = 0b0101;

            /// 0b0110: PLLSAI1M = 7
            pub const Div7: u32 = 0b0110;

            /// 0b0111: PLLSAI1M = 8
            pub const Div8: u32 = 0b0111;

            /// 0b1000: PLLSAI1M = 9
            pub const Div9: u32 = 0b1000;

            /// 0b1001: PLLSAI1M = 11
            pub const Div10: u32 = 0b1001;

            /// 0b1010: PLLSAI1M = 12
            pub const Div11: u32 = 0b1010;

            /// 0b1011: PLLSAI1M = 13
            pub const Div12: u32 = 0b1011;

            /// 0b1100: PLLSAI1M = 13
            pub const Div13: u32 = 0b1100;

            /// 0b1101: PLLSAI1M = 14
            pub const Div14: u32 = 0b1101;

            /// 0b1110: PLLSAI1M = 15
            pub const Div15: u32 = 0b1110;

            /// 0b1111: PLLSAI1M = 16
            pub const Div16: u32 = 0b1111;
        }
    }
}

/// PLLSAI2 configuration register
pub mod PLLSAI2CFGR {

    /// PLLSAI2 division factor for PLLSAI2CLK
    pub mod PLLSAI2PDIV {
        /// Offset (27 bits)
        pub const offset: u32 = 27;
        /// Mask (5 bits: 0b11111 << 27)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00000: PLLSAI2CLK is controlled by the bit PLLSAI2P
            pub const PLLSAI1P: u32 = 0b00000;

            /// 0b00010: PLLSAI2CLK = VCOSAI2 / 2
            pub const Div2: u32 = 0b00010;

            /// 0b00011: PLLSAI2CLK = VCOSAI2 / 3
            pub const Div3: u32 = 0b00011;

            /// 0b00100: PLLSAI2CLK = VCOSAI2 / 4
            pub const Div4: u32 = 0b00100;

            /// 0b00101: PLLSAI2CLK = VCOSAI2 / 5
            pub const Div5: u32 = 0b00101;

            /// 0b00110: PLLSAI2CLK = VCOSAI2 / 6
            pub const Div6: u32 = 0b00110;

            /// 0b00111: PLLSAI2CLK = VCOSAI2 / 7
            pub const Div7: u32 = 0b00111;

            /// 0b01000: PLLSAI2CLK = VCOSAI2 / 8
            pub const Div8: u32 = 0b01000;

            /// 0b01001: PLLSAI2CLK = VCOSAI2 / 9
            pub const Div9: u32 = 0b01001;

            /// 0b01010: PLLSAI2CLK = VCOSAI2 / 10
            pub const Div10: u32 = 0b01010;

            /// 0b01011: PLLSAI2CLK = VCOSAI2 / 11
            pub const Div11: u32 = 0b01011;

            /// 0b01100: PLLSAI2CLK = VCOSAI2 / 12
            pub const Div12: u32 = 0b01100;

            /// 0b01101: PLLSAI2CLK = VCOSAI2 / 13
            pub const Div13: u32 = 0b01101;

            /// 0b01110: PLLSAI2CLK = VCOSAI2 / 14
            pub const Div14: u32 = 0b01110;

            /// 0b01111: PLLSAI2CLK = VCOSAI2 / 15
            pub const Div15: u32 = 0b01111;

            /// 0b10000: PLLSAI2CLK = VCOSAI2 / 16
            pub const Div16: u32 = 0b10000;

            /// 0b10001: PLLSAI2CLK = VCOSAI2 / 17
            pub const Div17: u32 = 0b10001;

            /// 0b10010: PLLSAI2CLK = VCOSAI2 / 18
            pub const Div18: u32 = 0b10010;

            /// 0b10011: PLLSAI2CLK = VCOSAI2 / 19
            pub const Div19: u32 = 0b10011;

            /// 0b10100: PLLSAI2CLK = VCOSAI2 / 20
            pub const Div20: u32 = 0b10100;

            /// 0b10101: PLLSAI2CLK = VCOSAI2 / 21
            pub const Div21: u32 = 0b10101;

            /// 0b10110: PLLSAI2CLK = VCOSAI2 / 22
            pub const Div22: u32 = 0b10110;

            /// 0b10111: PLLSAI2CLK = VCOSAI2 / 23
            pub const Div23: u32 = 0b10111;

            /// 0b11000: PLLSAI2CLK = VCOSAI2 / 24
            pub const Div24: u32 = 0b11000;

            /// 0b11001: PLLSAI2CLK = VCOSAI2 / 25
            pub const Div25: u32 = 0b11001;

            /// 0b11010: PLLSAI2CLK = VCOSAI2 / 26
            pub const Div26: u32 = 0b11010;

            /// 0b11011: PLLSAI2CLK = VCOSAI2 / 27
            pub const Div27: u32 = 0b11011;

            /// 0b11100: PLLSAI2CLK = VCOSAI2 / 28
            pub const Div28: u32 = 0b11100;

            /// 0b11101: PLLSAI2CLK = VCOSAI2 / 29
            pub const Div29: u32 = 0b11101;

            /// 0b11110: PLLSAI2CLK = VCOSAI2 / 30
            pub const Div30: u32 = 0b11110;

            /// 0b11111: PLLSAI2CLK = VCOSAI2 / 31
            pub const Div31: u32 = 0b11111;
        }
    }

    /// PLLSAI2 division factor for PLLADC2CLK (ADC clock)
    pub mod PLLSAI2R {
        /// Offset (25 bits)
        pub const offset: u32 = 25;
        /// Mask (2 bits: 0b11 << 25)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: PLLSAI2x = 2
            pub const Div2: u32 = 0b00;

            /// 0b01: PLLSAI2x = 4
            pub const Div4: u32 = 0b01;

            /// 0b10: PLLSAI2x = 6
            pub const Div6: u32 = 0b10;

            /// 0b11: PLLSAI2x = 8
            pub const Div8: u32 = 0b11;
        }
    }

    /// PLLSAI2 PLLADC2CLK output enable
    pub mod PLLSAI2REN {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (1 bit: 1 << 24)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: PLLLCDCLK output disable
            pub const Disabled: u32 = 0b0;

            /// 0b1: PLLLCDCLK output enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// SAI2PLL PLLSAI2CLK output enable
    pub mod PLLSAI2Q {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (2 bits: 0b11 << 21)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PLLSAI2R::RW;
    }

    /// PLLSAI2 division factor for PLLDISCLK
    pub mod PLLSAI2QEN {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (1 bit: 1 << 20)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: PLLDSICLK output disable
            pub const Disabled: u32 = 0b0;

            /// 0b1: PLLDSICLK output enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// SAI1PLL division factor for PLLSAI2CLK (SAI1 or SAI2 clock)
    pub mod PLLSAI2P {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: PLLSAI2P = 7
            pub const Div7: u32 = 0b0;

            /// 0b1: PLLSAI2P = 17
            pub const Div17: u32 = 0b1;
        }
    }

    /// SAI2PLL PLLSAI2CLK output enable
    pub mod PLLSAI2PEN {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: PLLSAI2CLK output disable
            pub const Disabled: u32 = 0b0;

            /// 0b1: PLLSAI2CLK output enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// SAI2PLL multiplication factor for VCO
    pub mod PLLSAI2N {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (7 bits: 0x7f << 8)
        pub const mask: u32 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Division factor for PLLSAI2 input clock
    pub mod PLLSAI2M {
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

            /// 0b0000: PLLSAI2M = 1
            pub const Div1: u32 = 0b0000;

            /// 0b0001: PLLSAI2M = 2
            pub const Div2: u32 = 0b0001;

            /// 0b0010: PLLSAI2M = 3
            pub const Div3: u32 = 0b0010;

            /// 0b0011: PLLSAI2M = 4
            pub const Div4: u32 = 0b0011;

            /// 0b0100: PLLSAI2M = 5
            pub const Div5: u32 = 0b0100;

            /// 0b0101: PLLSAI2M = 6
            pub const Div6: u32 = 0b0101;

            /// 0b0110: PLLSAI2M = 7
            pub const Div7: u32 = 0b0110;

            /// 0b0111: PLLSAI2M = 8
            pub const Div8: u32 = 0b0111;

            /// 0b1000: PLLSAI2M = 9
            pub const Div9: u32 = 0b1000;

            /// 0b1001: PLLSAI2M = 11
            pub const Div10: u32 = 0b1001;

            /// 0b1010: PLLSAI2M = 12
            pub const Div11: u32 = 0b1010;

            /// 0b1011: PLLSAI2M = 13
            pub const Div12: u32 = 0b1011;

            /// 0b1100: PLLSAI2M = 13
            pub const Div13: u32 = 0b1100;

            /// 0b1101: PLLSAI2M = 14
            pub const Div14: u32 = 0b1101;

            /// 0b1110: PLLSAI2M = 15
            pub const Div15: u32 = 0b1110;

            /// 0b1111: PLLSAI2M = 16
            pub const Div16: u32 = 0b1111;
        }
    }
}

/// Clock interrupt enable register
pub mod CIER {

    /// LSI ready interrupt enable
    pub mod LSIRDYIE {
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

            /// 0b0: LSI ready interrupt disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: LSI ready interrupt enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// LSE ready interrupt enable
    pub mod LSERDYIE {
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

            /// 0b0: LSE ready interrupt disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: LSE ready interrupt enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// MSI ready interrupt enable
    pub mod MSIRDYIE {
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

            /// 0b0: MSI ready interrupt disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: MSI ready interrupt enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// HSI ready interrupt enable
    pub mod HSIRDYIE {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: HSI16 ready interrupt disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: HSI16 ready interrupt enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// HSE ready interrupt enable
    pub mod HSERDYIE {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: HSE ready interrupt disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: HSE ready interrupt enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// PLL ready interrupt enable
    pub mod PLLRDYIE {
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

            /// 0b0: PLL lock interrupt disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: PLL lock interrupt enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// PLLSAI1 ready interrupt enable
    pub mod PLLSAI1RDYIE {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: PLLSAI1 lock interrupt disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: PLLSAI1 lock interrupt enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// PLLSAI2 ready interrupt enable
    pub mod PLLSAI2RDYIE {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: PLLSAI2 lock interrupt disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: PLLSAI2 lock interrupt enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// LSE clock security system interrupt enable
    pub mod LSECSSIE {
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

            /// 0b0: Clock security interrupt caused by LSE clock failure disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Clock security interrupt caused by LSE clock failure enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// HSI48 ready interrupt enable
    pub mod HSI48RDYIE {
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

            /// 0b0: HSI48 ready interrupt disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: HSI48 ready interrupt enabled
            pub const Enabled: u32 = 0b1;
        }
    }
}

/// Clock interrupt flag register
pub mod CIFR {

    /// LSI ready interrupt flag
    pub mod LSIRDYF {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: No clock ready interrupt caused by the LSI oscillator
            pub const NoInterrupt: u32 = 0b0;

            /// 0b1: Clock ready interrupt caused by the LSI oscillator
            pub const Interrupt: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// LSE ready interrupt flag
    pub mod LSERDYF {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: No clock ready interrupt caused by the LSE oscillator
            pub const NoInterrupt: u32 = 0b0;

            /// 0b1: Clock ready interrupt caused by the LSE oscillator
            pub const Interrupt: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// MSI ready interrupt flag
    pub mod MSIRDYF {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: No clock ready interrupt caused by the MSI oscillator
            pub const NoInterrupt: u32 = 0b0;

            /// 0b1: Clock ready interrupt caused by the MSI oscillator
            pub const Interrupt: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// HSI ready interrupt flag
    pub mod HSIRDYF {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: No clock ready interrupt caused by the HSI16 oscillator
            pub const NoInterrupt: u32 = 0b0;

            /// 0b1: Clock ready interrupt caused by the HSI16 oscillator
            pub const Interrupt: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// HSE ready interrupt flag
    pub mod HSERDYF {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: No clock ready interrupt caused by the HSE oscillator
            pub const NoInterrupt: u32 = 0b0;

            /// 0b1: Clock ready interrupt caused by the HSE oscillator
            pub const Interrupt: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// PLL ready interrupt flag
    pub mod PLLRDYF {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: No clock ready interrupt caused by PLL lock
            pub const NoInterrupt: u32 = 0b0;

            /// 0b1: Clock ready interrupt caused by PLL lock
            pub const Interrupt: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// PLLSAI1 ready interrupt flag
    pub mod PLLSAI1RDYF {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: No clock ready interrupt caused by PLLSAI1 lock
            pub const NoInterrupt: u32 = 0b0;

            /// 0b1: Clock ready interrupt caused by PLLSAI1 lock
            pub const Interrupt: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// PLLSAI2 ready interrupt flag
    pub mod PLLSAI2RDYF {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: No clock ready interrupt caused by PLLSAI2 lock
            pub const NoInterrupt: u32 = 0b0;

            /// 0b1: Clock ready interrupt caused by PLLSAI2 lock
            pub const Interrupt: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Clock security system interrupt flag
    pub mod CSSF {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: No clock security interrupt caused by HSE clock failure
            pub const NoInterrupt: u32 = 0b0;

            /// 0b1: Clock security interrupt caused by HSE clock failure
            pub const Interrupt: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// LSE Clock security system interrupt flag
    pub mod LSECSSF {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: No clock security interrupt caused by LSE clock failure
            pub const NoInterrupt: u32 = 0b0;

            /// 0b1: Clock security interrupt caused by LSE clock failure
            pub const Interrupt: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// HSI48 ready interrupt flag
    pub mod HSI48RDYF {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: No clock ready interrupt caused by the HSI48 oscillator
            pub const NoInterrupt: u32 = 0b0;

            /// 0b1: Clock ready interrupt caused by the HSI48 oscillator
            pub const Interrupt: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Clock interrupt clear register
pub mod CICR {

    /// LSI ready interrupt clear
    pub mod LSIRDYC {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values
        pub mod W {

            /// 0b1: Clear the LSIRDYF flag
            pub const Clear: u32 = 0b1;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// LSE ready interrupt clear
    pub mod LSERDYC {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values
        pub mod W {

            /// 0b1: Clear the LSERDYF flag
            pub const Clear: u32 = 0b1;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// MSI ready interrupt clear
    pub mod MSIRDYC {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values
        pub mod W {

            /// 0b1: Clear the MSIRDYF flag
            pub const Clear: u32 = 0b1;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// HSI ready interrupt clear
    pub mod HSIRDYC {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values
        pub mod W {

            /// 0b1: Clear HSIRDYF flag
            pub const Clear: u32 = 0b1;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// HSE ready interrupt clear
    pub mod HSERDYC {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values
        pub mod W {

            /// 0b1: Clear HSERDYF flag
            pub const Clear: u32 = 0b1;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// PLL ready interrupt clear
    pub mod PLLRDYC {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values
        pub mod W {

            /// 0b1: Clear PLLRDYF flag
            pub const Clear: u32 = 0b1;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// PLLSAI1 ready interrupt clear
    pub mod PLLSAI1RDYC {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values
        pub mod W {

            /// 0b1: Clear PLLSAI1RDYF flag
            pub const Clear: u32 = 0b1;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// PLLSAI2 ready interrupt clear
    pub mod PLLSAI2RDYC {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values
        pub mod W {

            /// 0b1: Clear PLLSAI2RDYF flag
            pub const Clear: u32 = 0b1;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Clock security system interrupt clear
    pub mod CSSC {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values
        pub mod W {

            /// 0b1: Clear the CSSF flag
            pub const Clear: u32 = 0b1;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// LSE Clock security system interrupt clear
    pub mod LSECSSC {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values
        pub mod W {

            /// 0b1: Clear the LSECSSF flag
            pub const Clear: u32 = 0b1;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// HSI48 oscillator ready interrupt clear
    pub mod HSI48RDYC {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values
        pub mod W {

            /// 0b1: Clear the HSI48RDYC flag
            pub const Clear: u32 = 0b1;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// AHB1 peripheral reset register
pub mod AHB1RSTR {

    /// DMA1 reset
    pub mod DMA1RST {
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

            /// 0b0: No effect
            pub const NoEffect: u32 = 0b0;

            /// 0b1: Reset DMA1
            pub const Reset: u32 = 0b1;
        }
    }

    /// DMA2 reset
    pub mod DMA2RST {
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

            /// 0b0: No effect
            pub const NoEffect: u32 = 0b0;

            /// 0b1: Reset DMA2
            pub const Reset: u32 = 0b1;
        }
    }

    /// DMAMUXRST
    pub mod DMAMUX1RST {
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

            /// 0b0: No effect
            pub const NoEffect: u32 = 0b0;

            /// 0b1: Reset DMAMUX1
            pub const Reset: u32 = 0b1;
        }
    }

    /// Flash memory interface reset
    pub mod FLASHRST {
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

            /// 0b0: No effect
            pub const NoEffect: u32 = 0b0;

            /// 0b1: Reset Flash memory interface
            pub const Reset: u32 = 0b1;
        }
    }

    /// CRC reset
    pub mod CRCRST {
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

            /// 0b0: No effect
            pub const NoEffect: u32 = 0b0;

            /// 0b1: Reset CRC
            pub const Reset: u32 = 0b1;
        }
    }

    /// Touch Sensing Controller reset
    pub mod TSCRST {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: No effect
            pub const NoEffect: u32 = 0b0;

            /// 0b1: Reset TSC
            pub const Reset: u32 = 0b1;
        }
    }

    /// DMA2D reset
    pub mod DMA2DRST {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: No effect
            pub const NoEffect: u32 = 0b0;

            /// 0b1: Reset DMA2D
            pub const Reset: u32 = 0b1;
        }
    }

    /// GFXMMU reset
    pub mod GFXMMURST {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (1 bit: 1 << 18)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: No effect
            pub const NoEffect: u32 = 0b0;

            /// 0b1: Reset GFXMMU
            pub const Reset: u32 = 0b1;
        }
    }
}

/// AHB2 peripheral reset register
pub mod AHB2RSTR {

    /// IO port A reset
    pub mod GPIOARST {
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

            /// 0b0: No effect
            pub const NoEffect: u32 = 0b0;

            /// 0b1: Reset GPIO port x
            pub const Reset: u32 = 0b1;
        }
    }

    /// IO port B reset
    pub mod GPIOBRST {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::GPIOARST::RW;
    }

    /// IO port C reset
    pub mod GPIOCRST {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::GPIOARST::RW;
    }

    /// IO port D reset
    pub mod GPIODRST {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::GPIOARST::RW;
    }

    /// IO port E reset
    pub mod GPIOERST {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::GPIOARST::RW;
    }

    /// IO port F reset
    pub mod GPIOFRST {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::GPIOARST::RW;
    }

    /// IO port G reset
    pub mod GPIOGRST {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::GPIOARST::RW;
    }

    /// IO port H reset
    pub mod GPIOHRST {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::GPIOARST::RW;
    }

    /// IO port I reset
    pub mod GPIOIRST {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::GPIOARST::RW;
    }

    /// USB OTG FS reset
    pub mod OTGFSRST {
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

            /// 0b0: No effect
            pub const NoEffect: u32 = 0b0;

            /// 0b1: Reset USB OTG FS
            pub const Reset: u32 = 0b1;
        }
    }

    /// ADC reset
    pub mod ADCRST {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: No effect
            pub const NoEffect: u32 = 0b0;

            /// 0b1: Reset ADC
            pub const Reset: u32 = 0b1;
        }
    }

    /// Digital Camera Interface reset
    pub mod DCMIRST {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: No effect
            pub const NoEffect: u32 = 0b0;

            /// 0b1: Reset DCMI/PSSI interface
            pub const Reset: u32 = 0b1;
        }
    }

    /// AES hardware accelerator reset
    pub mod AESRST {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: No effect
            pub const NoEffect: u32 = 0b0;

            /// 0b1: Reset AES
            pub const Reset: u32 = 0b1;
        }
    }

    /// Hash reset
    pub mod HASHRST {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: No effect
            pub const NoEffect: u32 = 0b0;

            /// 0b1: Reset HASH
            pub const Reset: u32 = 0b1;
        }
    }

    /// Random number generator reset
    pub mod RNGRST {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (1 bit: 1 << 18)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: No effect
            pub const NoEffect: u32 = 0b0;

            /// 0b1: Reset RNG
            pub const Reset: u32 = 0b1;
        }
    }

    /// OCTOSPI IO manager reset
    pub mod OSPIMRST {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (1 bit: 1 << 20)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: No effect
            pub const NoEffect: u32 = 0b0;

            /// 0b1: Reset OctoSPI IO manager
            pub const Reset: u32 = 0b1;
        }
    }

    /// SDMMC1 reset
    pub mod SDMMC1RST {
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

            /// 0b0: No effect
            pub const NoEffect: u32 = 0b0;

            /// 0b1: Reset SDMMC1
            pub const Reset: u32 = 0b1;
        }
    }

    /// SDMMC2 reset
    pub mod SDMMC2RST {
        /// Offset (23 bits)
        pub const offset: u32 = 23;
        /// Mask (1 bit: 1 << 23)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: No effect
            pub const NoEffect: u32 = 0b0;

            /// 0b1: Reset SDMMC2
            pub const Reset: u32 = 0b1;
        }
    }

    /// PKA reset
    pub mod PKARST {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: No effect
            pub const NoEffect: u32 = 0b0;

            /// 0b1: Reset PKA
            pub const Reset: u32 = 0b1;
        }
    }
}

/// AHB3 peripheral reset register
pub mod AHB3RSTR {

    /// Flexible memory controller reset
    pub mod FMCRST {
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

            /// 0b0: No effect
            pub const NoEffect: u32 = 0b0;

            /// 0b1: Reset FMC
            pub const Reset: u32 = 0b1;
        }
    }

    /// OctOSPI2 memory interface reset
    pub mod OSPI2RST {
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

            /// 0b0: No effect
            pub const NoEffect: u32 = 0b0;

            /// 0b1: Reset OctoSPIx
            pub const Reset: u32 = 0b1;
        }
    }

    /// OctoSPI1 memory interface reset
    pub mod OSPI1RST {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::OSPI2RST::RW;
    }
}

/// APB1 peripheral reset register 1
pub mod APB1RSTR1 {

    /// Low Power Timer 1 reset
    pub mod LPTIM1RST {
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

            /// 0b0: No effect
            pub const NoEffect: u32 = 0b0;

            /// 0b1: Reset LPTIM1
            pub const Reset: u32 = 0b1;
        }
    }

    /// OPAMP interface reset
    pub mod OPAMPRST {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (1 bit: 1 << 30)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: No effect
            pub const NoEffect: u32 = 0b0;

            /// 0b1: Reset OPAMP
            pub const Reset: u32 = 0b1;
        }
    }

    /// DAC1 interface reset
    pub mod DAC1RST {
        /// Offset (29 bits)
        pub const offset: u32 = 29;
        /// Mask (1 bit: 1 << 29)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: No effect
            pub const NoEffect: u32 = 0b0;

            /// 0b1: Reset DAC1
            pub const Reset: u32 = 0b1;
        }
    }

    /// Power interface reset
    pub mod PWRRST {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (1 bit: 1 << 28)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: No effect
            pub const NoEffect: u32 = 0b0;

            /// 0b1: Reset PWR
            pub const Reset: u32 = 0b1;
        }
    }

    /// CAN1 reset
    pub mod CAN1RST {
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

            /// 0b0: No effect
            pub const NoEffect: u32 = 0b0;

            /// 0b1: Reset CAN1
            pub const Reset: u32 = 0b1;
        }
    }

    /// CRS reset
    pub mod CRSRST {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (1 bit: 1 << 24)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: No effect
            pub const NoEffect: u32 = 0b0;

            /// 0b1: Reset CRS
            pub const Reset: u32 = 0b1;
        }
    }

    /// I2C3 reset
    pub mod I2C3RST {
        /// Offset (23 bits)
        pub const offset: u32 = 23;
        /// Mask (1 bit: 1 << 23)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: No effect
            pub const NoEffect: u32 = 0b0;

            /// 0b1: Reset I2Cx
            pub const Reset: u32 = 0b1;
        }
    }

    /// I2C2 reset
    pub mod I2C2RST {
        /// Offset (22 bits)
        pub const offset: u32 = 22;
        /// Mask (1 bit: 1 << 22)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::I2C3RST::RW;
    }

    /// I2C1 reset
    pub mod I2C1RST {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (1 bit: 1 << 21)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::I2C3RST::RW;
    }

    /// UART5 reset
    pub mod UART5RST {
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

    /// UART4 reset
    pub mod UART4RST {
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

    /// USART3 reset
    pub mod USART3RST {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (1 bit: 1 << 18)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: No effect
            pub const NoEffect: u32 = 0b0;

            /// 0b1: Reset UARTx
            pub const Reset: u32 = 0b1;
        }
    }

    /// USART2 reset
    pub mod USART2RST {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::USART3RST::RW;
    }

    /// SPI3 reset
    pub mod SPI3RST {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: No effect
            pub const NoEffect: u32 = 0b0;

            /// 0b1: Reset SPIx
            pub const Reset: u32 = 0b1;
        }
    }

    /// SPI2 reset
    pub mod SPI2RST {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::SPI3RST::RW;
    }

    /// TIM7 timer reset
    pub mod TIM7RST {
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

            /// 0b0: No effect
            pub const NoEffect: u32 = 0b0;

            /// 0b1: Reset TIMx
            pub const Reset: u32 = 0b1;
        }
    }

    /// TIM6 timer reset
    pub mod TIM6RST {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TIM7RST::RW;
    }

    /// TIM5 timer reset
    pub mod TIM5RST {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TIM7RST::RW;
    }

    /// TIM3 timer reset
    pub mod TIM4RST {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TIM7RST::RW;
    }

    /// TIM3 timer reset
    pub mod TIM3RST {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TIM7RST::RW;
    }

    /// TIM2 timer reset
    pub mod TIM2RST {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TIM7RST::RW;
    }
}

/// APB1 peripheral reset register 2
pub mod APB1RSTR2 {

    /// Low-power UART 1 reset
    pub mod LPUART1RST {
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

            /// 0b0: No effect
            pub const NoEffect: u32 = 0b0;

            /// 0b1: Reset LPUART1
            pub const Reset: u32 = 0b1;
        }
    }

    /// I2C4 reset
    pub mod I2C4RST {
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

            /// 0b0: No effect
            pub const NoEffect: u32 = 0b0;

            /// 0b1: Reset I2C4
            pub const Reset: u32 = 0b1;
        }
    }

    /// Low-power timer 2 reset
    pub mod LPTIM2RST {
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

            /// 0b0: No effect
            pub const NoEffect: u32 = 0b0;

            /// 0b1: Reset LPTIM2
            pub const Reset: u32 = 0b1;
        }
    }
}

/// APB2 peripheral reset register
pub mod APB2RSTR {

    /// System configuration (SYSCFG) reset
    pub mod SYSCFGRST {
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

            /// 0b0: No effect
            pub const NoEffect: u32 = 0b0;

            /// 0b1: Reset SYSCFG + COMP + VREFBUF
            pub const Reset: u32 = 0b1;
        }
    }

    /// TIM1 timer reset
    pub mod TIM1RST {
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

            /// 0b0: No effect
            pub const NoEffect: u32 = 0b0;

            /// 0b1: Reset TIMx
            pub const Reset: u32 = 0b1;
        }
    }

    /// SPI1 reset
    pub mod SPI1RST {
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

            /// 0b0: No effect
            pub const NoEffect: u32 = 0b0;

            /// 0b1: Reset SPI1
            pub const Reset: u32 = 0b1;
        }
    }

    /// TIM8 timer reset
    pub mod TIM8RST {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TIM1RST::RW;
    }

    /// USART1 reset
    pub mod USART1RST {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: No effect
            pub const NoEffect: u32 = 0b0;

            /// 0b1: Reset UARTx
            pub const Reset: u32 = 0b1;
        }
    }

    /// TIM15 timer reset
    pub mod TIM15RST {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TIM1RST::RW;
    }

    /// TIM16 timer reset
    pub mod TIM16RST {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TIM1RST::RW;
    }

    /// TIM17 timer reset
    pub mod TIM17RST {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (1 bit: 1 << 18)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TIM1RST::RW;
    }

    /// Serial audio interface 1 (SAI1) reset
    pub mod SAI1RST {
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

            /// 0b0: No effect
            pub const NoEffect: u32 = 0b0;

            /// 0b1: Reset SAIx
            pub const Reset: u32 = 0b1;
        }
    }

    /// Serial audio interface 2 (SAI2) reset
    pub mod SAI2RST {
        /// Offset (22 bits)
        pub const offset: u32 = 22;
        /// Mask (1 bit: 1 << 22)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::SAI1RST::RW;
    }

    /// Digital filters for sigma-delata modulators (DFSDM) reset
    pub mod DFSDM1RST {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (1 bit: 1 << 24)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: No effect
            pub const NoEffect: u32 = 0b0;

            /// 0b1: Reset DFSDM1
            pub const Reset: u32 = 0b1;
        }
    }

    /// LCD-TFT reset
    pub mod LTDCRST {
        /// Offset (26 bits)
        pub const offset: u32 = 26;
        /// Mask (1 bit: 1 << 26)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: No effect
            pub const NoEffect: u32 = 0b0;

            /// 0b1: Reset LCD-TFT
            pub const Reset: u32 = 0b1;
        }
    }

    /// DSI reset
    pub mod DSIRST {
        /// Offset (27 bits)
        pub const offset: u32 = 27;
        /// Mask (1 bit: 1 << 27)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: No effect
            pub const NoEffect: u32 = 0b0;

            /// 0b1: Reset DSI
            pub const Reset: u32 = 0b1;
        }
    }
}

/// AHB1 peripheral clock enable register
pub mod AHB1ENR {

    /// DMA1 clock enable
    pub mod DMA1EN {
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

            /// 0b0: DMAx clock disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: DMAx clock enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// DMA2 clock enable
    pub mod DMA2EN {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::DMA1EN::RW;
    }

    /// DMAMUX clock enable
    pub mod DMAMUX1EN {
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

            /// 0b0: DMAMUX1 clock disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: DMAMUX1 clock enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Flash memory interface clock enable
    pub mod FLASHEN {
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

            /// 0b0: Flash memory interface clock disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Flash memory interface clock enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// CRC clock enable
    pub mod CRCEN {
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

            /// 0b0: CRC clock disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: CRC clock enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Touch Sensing Controller clock enable
    pub mod TSCEN {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: TSC clock disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: TSC clock enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// DMA2D clock enable
    pub mod DMA2DEN {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: DMA2D clock disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: DMA2D clock enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Graphic MMU clock enable
    pub mod GFXMMUEN {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (1 bit: 1 << 18)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: GFXMMU clock disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: GFXMMU clock enabled
            pub const Enabled: u32 = 0b1;
        }
    }
}

/// AHB2 peripheral clock enable register
pub mod AHB2ENR {

    /// IO port A clock enable
    pub mod GPIOAEN {
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

            /// 0b0: IO port x clock disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: IO port x clock enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// IO port B clock enable
    pub mod GPIOBEN {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::GPIOAEN::RW;
    }

    /// IO port C clock enable
    pub mod GPIOCEN {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::GPIOAEN::RW;
    }

    /// IO port D clock enable
    pub mod GPIODEN {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::GPIOAEN::RW;
    }

    /// IO port E clock enable
    pub mod GPIOEEN {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::GPIOAEN::RW;
    }

    /// IO port F clock enable
    pub mod GPIOFEN {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::GPIOAEN::RW;
    }

    /// IO port G clock enable
    pub mod GPIOGEN {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::GPIOAEN::RW;
    }

    /// IO port H clock enable
    pub mod GPIOHEN {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::GPIOAEN::RW;
    }

    /// IO port I clock enable
    pub mod GPIOIEN {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::GPIOAEN::RW;
    }

    /// OTG full speed clock enable
    pub mod OTGFSEN {
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

            /// 0b0: USB OTG full speed clock disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: USB OTG full speed clock enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// ADC clock enable
    pub mod ADCEN {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: ADC clock disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: ADC clock enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// DCMI clock enable
    pub mod DCMIEN {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: DCMI/PSSI clock disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: DCMI/PSSI clock enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// AES accelerator clock enable
    pub mod AESEN {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: AES clock disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: AES clock enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// HASH clock enable
    pub mod HASHEN {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: HASH clock disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: HASH clock enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Random Number Generator clock enable
    pub mod RNGEN {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (1 bit: 1 << 18)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Random Number Generator clock disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Random Number Generator clock enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// OctoSPI IO manager clock enable
    pub mod OSPIMEN {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (1 bit: 1 << 20)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: OctoSPI IO manager clock disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: OctoSPI IO manager clock enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// SDMMC1 clock enable
    pub mod SDMMC1EN {
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

            /// 0b0: SDMMCx clock disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: SDMMCx clock enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// SDMMC2 clock enable
    pub mod SDMMC2EN {
        /// Offset (23 bits)
        pub const offset: u32 = 23;
        /// Mask (1 bit: 1 << 23)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::SDMMC1EN::RW;
    }

    /// PKA clock enable
    pub mod PKAEN {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: PKA clock disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: PKA clock enabled
            pub const Enabled: u32 = 0b1;
        }
    }
}

/// AHB3 peripheral clock enable register
pub mod AHB3ENR {

    /// Flexible memory controller clock enable
    pub mod FMCEN {
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

            /// 0b0: FMC clock disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: FMC clock enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// OSPI2EN memory interface clock enable
    pub mod OSPI2EN {
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

            /// 0b0: OctoSPI x clock disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: OctoSPI x clock enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// OctoSPI1 memory interface clock enable
    pub mod OSPI1EN {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::OSPI2EN::RW;
    }
}

/// APB1ENR1
pub mod APB1ENR1 {

    /// TIM2 timer clock enable
    pub mod TIM2EN {
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

            /// 0b0: TIMx clock disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: TIMx clock enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// TIM3 timer clock enable
    pub mod TIM3EN {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TIM2EN::RW;
    }

    /// TIM4 timer clock enable
    pub mod TIM4EN {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TIM2EN::RW;
    }

    /// TIM5 timer clock enable
    pub mod TIM5EN {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TIM2EN::RW;
    }

    /// TIM6 timer clock enable
    pub mod TIM6EN {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TIM2EN::RW;
    }

    /// TIM7 timer clock enable
    pub mod TIM7EN {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TIM2EN::RW;
    }

    /// RTC APB clock enable
    pub mod RTCAPBEN {
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

            /// 0b0: RTC APB clock disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: RTC APB clock enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Window watchdog clock enable
    pub mod WWDGEN {
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

            /// 0b0: Window watchdog clock disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Window watchdog clock enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// SPI2 clock enable
    pub mod SPI2EN {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: SPIx clock disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: SPIx clock enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// SPI3 clock enable
    pub mod SPI3EN {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::SPI2EN::RW;
    }

    /// USART2 clock enable
    pub mod USART2EN {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: USARTx clock disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: USARTx clock enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// USART3 clock enable
    pub mod USART3EN {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (1 bit: 1 << 18)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::USART2EN::RW;
    }

    /// UART4 clock enable
    pub mod UART4EN {
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

            /// 0b0: UARTx clock disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: UARTx clock enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// UART5 clock enable
    pub mod UART5EN {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (1 bit: 1 << 20)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::UART4EN::RW;
    }

    /// I2C1 clock enable
    pub mod I2C1EN {
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

            /// 0b0: I2C1 clock disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: I2C1 clock enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// I2C2 clock enable
    pub mod I2C2EN {
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

            /// 0b0: I2C2 clock disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: I2C2 clock enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// I2C3 clock enable
    pub mod I2C3EN {
        /// Offset (23 bits)
        pub const offset: u32 = 23;
        /// Mask (1 bit: 1 << 23)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: I2C3 clock disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: I2C3 clock enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Clock Recovery System clock enable
    pub mod CRSEN {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (1 bit: 1 << 24)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: CRS clock disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: CRS clock enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// CAN1 clock enable
    pub mod CAN1EN {
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

            /// 0b0: CAN1 clock disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: CAN1 clock enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Power interface clock enable
    pub mod PWREN {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (1 bit: 1 << 28)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Power interface clock disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Power interface clock enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// DAC1 interface clock enable
    pub mod DAC1EN {
        /// Offset (29 bits)
        pub const offset: u32 = 29;
        /// Mask (1 bit: 1 << 29)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: DAC1 clock disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: DAC1 clock enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// OPAMP interface clock enable
    pub mod OPAMPEN {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (1 bit: 1 << 30)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: OPAMP clock disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: OPAMP clock enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Low power timer 1 clock enable
    pub mod LPTIM1EN {
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

            /// 0b0: LPTIM1 clock disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: LPTIM1 clock enabled
            pub const Enabled: u32 = 0b1;
        }
    }
}

/// APB1 peripheral clock enable register 2
pub mod APB1ENR2 {

    /// Low power UART 1 clock enable
    pub mod LPUART1EN {
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

            /// 0b0: LPUART1 clock disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: LPUART1 clock enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// I2C4 clock enable
    pub mod I2C4EN {
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

            /// 0b0: I2C4 clock disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: I2C4 clock enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// LPTIM2EN
    pub mod LPTIM2EN {
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

            /// 0b0: LPTIM2 clock disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: LPTIM2 clock enabled
            pub const Enabled: u32 = 0b1;
        }
    }
}

/// APB2ENR
pub mod APB2ENR {

    /// SYSCFG clock enable
    pub mod SYSCFGEN {
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

            /// 0b0: SYSCFG + COMP + VREFBUF clock disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: SYSCFG + COMP + VREFBUF clock enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Firewall clock enable
    pub mod FWEN {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Firewall clock disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Firewall clock enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// TIM1 timer clock enable
    pub mod TIM1EN {
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

            /// 0b0: TIMx clock disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: TIMx clock enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// SPI1 clock enable
    pub mod SPI1EN {
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

            /// 0b0: SPI1 clock disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: SPI1 clock enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// TIM8 timer clock enable
    pub mod TIM8EN {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TIM1EN::RW;
    }

    /// USART1clock enable
    pub mod USART1EN {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: USART1 clock disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: USART1 clock enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// TIM15 timer clock enable
    pub mod TIM15EN {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TIM1EN::RW;
    }

    /// TIM16 timer clock enable
    pub mod TIM16EN {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TIM1EN::RW;
    }

    /// TIM17 timer clock enable
    pub mod TIM17EN {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (1 bit: 1 << 18)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TIM1EN::RW;
    }

    /// SAI1 clock enable
    pub mod SAI1EN {
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

            /// 0b0: SAIx clock disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: SAIx clock enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// SAI2 clock enable
    pub mod SAI2EN {
        /// Offset (22 bits)
        pub const offset: u32 = 22;
        /// Mask (1 bit: 1 << 22)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::SAI1EN::RW;
    }

    /// DFSDM timer clock enable
    pub mod DFSDM1EN {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (1 bit: 1 << 24)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: DFSDM1 clock disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: DFSDM1 clock enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// LCD-TFT clock enable
    pub mod LTDCEN {
        /// Offset (26 bits)
        pub const offset: u32 = 26;
        /// Mask (1 bit: 1 << 26)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: LTDC clock disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: LTDC clock enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// DSI clock enable
    pub mod DSIEN {
        /// Offset (27 bits)
        pub const offset: u32 = 27;
        /// Mask (1 bit: 1 << 27)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: DSI clock disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: DSI clock enabled
            pub const Enabled: u32 = 0b1;
        }
    }
}

/// AHB1 peripheral clocks enable in Sleep and Stop modes register
pub mod AHB1SMENR {

    /// DMA1 clocks enable during Sleep and Stop modes
    pub mod DMA1SMEN {
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

            /// 0b0: DMAx clocks disabled by the clock gating during Sleep and Stop modes
            pub const Disabled: u32 = 0b0;

            /// 0b1: DMAx clocks enabled by the clock gating(1) during Sleep and Stop modes
            pub const Enabled: u32 = 0b1;
        }
    }

    /// DMA2 clocks enable during Sleep and Stop modes
    pub mod DMA2SMEN {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::DMA1SMEN::RW;
    }

    /// DMAMUX clock enable during Sleep and Stop modes
    pub mod DMAMUX1SMEN {
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

            /// 0b0: DMAMUX1 clocks disabled by the clock gating during Sleep and Stop modes
            pub const Disabled: u32 = 0b0;

            /// 0b1: DMAMUX1 clocks enabled by the clock gating(1) during Sleep and Stop modes
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Flash memory interface clocks enable during Sleep and Stop modes
    pub mod FLASHSMEN {
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

            /// 0b0: Flash memory interface clocks disabled by the clock gating during Sleep and Stop modes
            pub const Disabled: u32 = 0b0;

            /// 0b1: Flash memory interface clocks enabled by the clock gating(1) during Sleep and Stop modes
            pub const Enabled: u32 = 0b1;
        }
    }

    /// SRAM1 interface clocks enable during Sleep and Stop modes
    pub mod SRAM1SMEN {
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

            /// 0b0: SRAM1 clocks disabled by the clock gating during Sleep and Stop modes
            pub const Disabled: u32 = 0b0;

            /// 0b1: SRAM1 clocks enabled by the clock gating(1) during Sleep and Stop modes
            pub const Enabled: u32 = 0b1;
        }
    }

    /// CRCSMEN
    pub mod CRCSMEN {
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

            /// 0b0: CRC clocks disabled by the clock gating during Sleep and Stop modes
            pub const Disabled: u32 = 0b0;

            /// 0b1: CRC clocks enabled by the clock gating(1) during Sleep and Stop modes
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Touch Sensing Controller clocks enable during Sleep and Stop modes
    pub mod TSCSMEN {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: TSC clocks disabled by the clock gating during Sleep and Stop modes
            pub const Disabled: u32 = 0b0;

            /// 0b1: TSC clocks enabled by the clock gating(1) during Sleep and Stop modes
            pub const Enabled: u32 = 0b1;
        }
    }

    /// DMA2D clock enable during Sleep and Stop modes
    pub mod DMA2DSMEN {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: DMA2D clocks disabled by the clock gating during Sleep and Stop modes
            pub const Disabled: u32 = 0b0;

            /// 0b1: DMA2D clocks enabled by the clock gating(1) during Sleep and Stop modes
            pub const Enabled: u32 = 0b1;
        }
    }

    /// GFXMMU clock enable during Sleep and Stop modes
    pub mod GFXMMUSMEN {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (1 bit: 1 << 18)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: GFXMMU clocks disabled by the clock gating during Sleep and Stop modes
            pub const Disabled: u32 = 0b0;

            /// 0b1: GFXMMU clocks enabled by the clock gating(1) during Sleep and Stop modes
            pub const Enabled: u32 = 0b1;
        }
    }
}

/// AHB2 peripheral clocks enable in Sleep and Stop modes register
pub mod AHB2SMENR {

    /// IO port A clocks enable during Sleep and Stop modes
    pub mod GPIOASMEN {
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

            /// 0b0: IO port x clocks disabled by the clock gating during Sleep and Stop modes
            pub const Disabled: u32 = 0b0;

            /// 0b1: IO port x clocks enabled by the clock gating(1) during Sleep and Stop modes
            pub const Enabled: u32 = 0b1;
        }
    }

    /// IO port B clocks enable during Sleep and Stop modes
    pub mod GPIOBSMEN {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::GPIOASMEN::RW;
    }

    /// IO port C clocks enable during Sleep and Stop modes
    pub mod GPIOCSMEN {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::GPIOASMEN::RW;
    }

    /// IO port D clocks enable during Sleep and Stop modes
    pub mod GPIODSMEN {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::GPIOASMEN::RW;
    }

    /// IO port E clocks enable during Sleep and Stop modes
    pub mod GPIOESMEN {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::GPIOASMEN::RW;
    }

    /// IO port F clocks enable during Sleep and Stop modes
    pub mod GPIOFSMEN {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::GPIOASMEN::RW;
    }

    /// IO port G clocks enable during Sleep and Stop modes
    pub mod GPIOGSMEN {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::GPIOASMEN::RW;
    }

    /// IO port H clocks enable during Sleep and Stop modes
    pub mod GPIOHSMEN {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::GPIOASMEN::RW;
    }

    /// IO port I clocks enable during Sleep and Stop modes
    pub mod GPIOISMEN {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::GPIOASMEN::RW;
    }

    /// SRAM2 interface clocks enable during Sleep and Stop modes
    pub mod SRAM2SMEN {
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

            /// 0b0: SRAMx clocks disabled by the clock gating during Sleep and Stop modes
            pub const Disabled: u32 = 0b0;

            /// 0b1: SRAMx clocks enabled by the clock gating(1) during Sleep and Stop modes
            pub const Enabled: u32 = 0b1;
        }
    }

    /// SRAM2 interface clocks enable during Sleep and Stop modes
    pub mod SRAM3SMEN {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::SRAM2SMEN::RW;
    }

    /// OTG full speed clocks enable during Sleep and Stop modes
    pub mod OTGFSSMEN {
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

            /// 0b0: USB OTG full speed clocks disabled by the clock gating during Sleep and Stop modes
            pub const Disabled: u32 = 0b0;

            /// 0b1: USB OTG full speed clocks enabled by the clock gating(1) during Sleep and Stop modes
            pub const Enabled: u32 = 0b1;
        }
    }

    /// ADC clocks enable during Sleep and Stop modes
    pub mod ADCSMEN {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: ADC clocks disabled by the clock gating during Sleep and Stop modes
            pub const Disabled: u32 = 0b0;

            /// 0b1: ADC clocks enabled by the clock gating(1) during Sleep and Stop modes
            pub const Enabled: u32 = 0b1;
        }
    }

    /// DCMI clock enable during Sleep and Stop modes
    pub mod DCMISMEN {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: DCMI/PSSI clocks disabled by the clock gating during Sleep and Stop modes
            pub const Disabled: u32 = 0b0;

            /// 0b1: DCMI/PSSI clocks enabled by the clock gating(1) during Sleep and Stop modes
            pub const Enabled: u32 = 0b1;
        }
    }

    /// AES accelerator clocks enable during Sleep and Stop modes
    pub mod AESSMEN {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: AES clocks disabled by the clock gating during Sleep and Stop modes
            pub const Disabled: u32 = 0b0;

            /// 0b1: AES clocks enabled by the clock gating(1) during Sleep and Stop modes
            pub const Enabled: u32 = 0b1;
        }
    }

    /// HASH clock enable during Sleep and Stop modes
    pub mod HASHSMEN {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: HASH clocks disabled by the clock gating during Sleep and Stop modes
            pub const Disabled: u32 = 0b0;

            /// 0b1: HASH clocks enabled by the clock gating(1) during Sleep and Stop modes
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Random Number Generator clocks enable during Sleep and Stop modes
    pub mod RNGSMEN {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (1 bit: 1 << 18)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Random Number Generator clocks disabled by the clock gating during Sleep and Stop modes
            pub const Disabled: u32 = 0b0;

            /// 0b1: Random Number Generator clocks enabled by the clock gating(1) during Sleep and Stop modes
            pub const Enabled: u32 = 0b1;
        }
    }

    /// OctoSPI IO manager clocks enable during Sleep and Stop modes
    pub mod OSPIMSMEN {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (1 bit: 1 << 20)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: OCTOSPIM clocks disabled by the clock gating during Sleep and Stop modes
            pub const Disabled: u32 = 0b0;

            /// 0b1: OCTOSPIM clocks enabled by the clock gating(1) during Sleep and Stop modes
            pub const Enabled: u32 = 0b1;
        }
    }

    /// SDMMC1 clocks enable during Sleep and Stop modes
    pub mod SDMMC1SMEN {
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

            /// 0b0: SDMMCx clocks disabled by the clock gating during Sleep and Stop modes
            pub const Disabled: u32 = 0b0;

            /// 0b1: SDMMCx clocks enabled by the clock gating(1) during Sleep and Stop modes
            pub const Enabled: u32 = 0b1;
        }
    }

    /// SDMMC2 clocks enable during Sleep and Stop modes
    pub mod SDMMC2SMEN {
        /// Offset (23 bits)
        pub const offset: u32 = 23;
        /// Mask (1 bit: 1 << 23)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::SDMMC1SMEN::RW;
    }

    /// PKA clocks enable during Sleep and Stop modes
    pub mod PKASMEN {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: PKA clocks disabled by the clock gating during Sleep and Stop modes
            pub const Disabled: u32 = 0b0;

            /// 0b1: PKA clocks enabled by the clock gating(1) during Sleep and Stop modes
            pub const Enabled: u32 = 0b1;
        }
    }
}

/// AHB3 peripheral clocks enable in Sleep and Stop modes register
pub mod AHB3SMENR {

    /// Flexible memory controller clocks enable during Sleep and Stop modes
    pub mod FMCSMEN {
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

            /// 0b0: FMC clocks disabled by the clock gating during Sleep and Stop modes
            pub const Disabled: u32 = 0b0;

            /// 0b1: FMC clocks enabled by the clock gating(1) during Sleep and Stop modes
            pub const Enabled: u32 = 0b1;
        }
    }

    /// OctoSPI2 memory interface clocks enable during Sleep and Stop modes
    pub mod OCTOSPI2 {
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

            /// 0b0: OctoSPI2 clocks disabled by the clock gating during Sleep and Stop modes
            pub const Disabled: u32 = 0b0;

            /// 0b1: OctoSPI2 clocks enabled by the clock gating(1) during Sleep and Stop modes
            pub const Enabled: u32 = 0b1;
        }
    }

    /// OctoSPI1 memory interface clocks enable during Sleep and Stop modes
    pub mod OSPI1SMEN {
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

            /// 0b0: OctoSPI1 clocks disabled by the clock gating during Sleep and Stop modes
            pub const Disabled: u32 = 0b0;

            /// 0b1: OctoSPI1 clocks enabled by the clock gating(1) during Sleep and Stop modes
            pub const Enabled: u32 = 0b1;
        }
    }
}

/// APB1SMENR1
pub mod APB1SMENR1 {

    /// TIM2 timer clocks enable during Sleep and Stop modes
    pub mod TIM2SMEN {
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

            /// 0b0: TIMx clocks disabled by the clock gating during Sleep and Stop modes
            pub const Disabled: u32 = 0b0;

            /// 0b1: TIMx clocks enabled by the clock gating(1) during Sleep and Stop modes
            pub const Enabled: u32 = 0b1;
        }
    }

    /// TIM3 timer clocks enable during Sleep and Stop modes
    pub mod TIM3SMEN {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TIM2SMEN::RW;
    }

    /// TIM4 timer clocks enable during Sleep and Stop modes
    pub mod TIM4SMEN {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TIM2SMEN::RW;
    }

    /// TIM5 timer clocks enable during Sleep and Stop modes
    pub mod TIM5SMEN {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TIM2SMEN::RW;
    }

    /// TIM6 timer clocks enable during Sleep and Stop modes
    pub mod TIM6SMEN {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TIM2SMEN::RW;
    }

    /// TIM7 timer clocks enable during Sleep and Stop modes
    pub mod TIM7SMEN {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TIM2SMEN::RW;
    }

    /// RTC APB clock enable during Sleep and Stop modes
    pub mod RTCAPBSMEN {
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

            /// 0b0: RTC APB clocks disabled by the clock gating during Sleep and Stop modes
            pub const Disabled: u32 = 0b0;

            /// 0b1: RTC APB clocks enabled by the clock gating(1) during Sleep and Stop modes
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Window watchdog clocks enable during Sleep and Stop modes
    pub mod WWDGSMEN {
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

            /// 0b0: Window watchdog clocks disabled by the clock gating during Sleep and Stop modes
            pub const Disabled: u32 = 0b0;

            /// 0b1: Window watchdog clocks enabled by the clock gating(1) during Sleep and Stop modes
            pub const Enabled: u32 = 0b1;
        }
    }

    /// SPI2 clocks enable during Sleep and Stop modes
    pub mod SPI2SMEN {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: SPIx clocks disabled by the clock gating during Sleep and Stop modes
            pub const Disabled: u32 = 0b0;

            /// 0b1: SPIx clocks enabled by the clock gating(1) during Sleep and Stop modes
            pub const Enabled: u32 = 0b1;
        }
    }

    /// SPI3 clocks enable during Sleep and Stop modes
    pub mod SP3SMEN {
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

    /// USART2 clocks enable during Sleep and Stop modes
    pub mod USART2SMEN {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: USARTx clocks disabled by the clock gating during Sleep and Stop modes
            pub const Disabled: u32 = 0b0;

            /// 0b1: USARTx clocks enabled by the clock gating(1) during Sleep and Stop modes
            pub const Enabled: u32 = 0b1;
        }
    }

    /// USART3 clocks enable during Sleep and Stop modes
    pub mod USART3SMEN {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (1 bit: 1 << 18)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::USART2SMEN::RW;
    }

    /// UART4 clocks enable during Sleep and Stop modes
    pub mod UART4SMEN {
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

            /// 0b0: UARTx clocks disabled by the clock gating during Sleep and Stop modes
            pub const Disabled: u32 = 0b0;

            /// 0b1: UARTx clocks enabled by the clock gating(1) during Sleep and Stop modes
            pub const Enabled: u32 = 0b1;
        }
    }

    /// UART5 clocks enable during Sleep and Stop modes
    pub mod UART5SMEN {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (1 bit: 1 << 20)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::UART4SMEN::RW;
    }

    /// I2C1 clocks enable during Sleep and Stop modes
    pub mod I2C1SMEN {
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

            /// 0b0: I2Cx clocks disabled by the clock gating during Sleep and Stop modes
            pub const Disabled: u32 = 0b0;

            /// 0b1: I2Cx clocks enabled by the clock gating(1) during Sleep and Stop modes
            pub const Enabled: u32 = 0b1;
        }
    }

    /// I2C2 clocks enable during Sleep and Stop modes
    pub mod I2C2SMEN {
        /// Offset (22 bits)
        pub const offset: u32 = 22;
        /// Mask (1 bit: 1 << 22)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::I2C1SMEN::RW;
    }

    /// I2C3 clocks enable during Sleep and Stop modes
    pub mod I2C3SMEN {
        /// Offset (23 bits)
        pub const offset: u32 = 23;
        /// Mask (1 bit: 1 << 23)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::I2C1SMEN::RW;
    }

    /// CRS clock enable during Sleep and Stop modes
    pub mod CRSSMEN {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (1 bit: 1 << 24)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: CRS clocks disabled by the clock gating during Sleep and Stop modes
            pub const Disabled: u32 = 0b0;

            /// 0b1: CRS clocks enabled by the clock gating(1) during Sleep and Stop modes
            pub const Enabled: u32 = 0b1;
        }
    }

    /// CAN1 clocks enable during Sleep and Stop modes
    pub mod CAN1SMEN {
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

            /// 0b0: CAN1 clocks disabled by the clock gating during Sleep and Stop modes
            pub const Disabled: u32 = 0b0;

            /// 0b1: CAN1 clocks enabled by the clock gating(1) during Sleep and Stop modes
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Power interface clocks enable during Sleep and Stop modes
    pub mod PWRSMEN {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (1 bit: 1 << 28)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Power interface clocks disabled by the clock gating during Sleep and Stop modes
            pub const Disabled: u32 = 0b0;

            /// 0b1: Power interface clocks enabled by the clock gating(1) during Sleep and Stop modes
            pub const Enabled: u32 = 0b1;
        }
    }

    /// DAC1 interface clocks enable during Sleep and Stop modes
    pub mod DAC1SMEN {
        /// Offset (29 bits)
        pub const offset: u32 = 29;
        /// Mask (1 bit: 1 << 29)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: DAC1 interface clocks disabled by the clock gating during Sleep and Stop modes
            pub const Disabled: u32 = 0b0;

            /// 0b1: DAC1 interface clocks enabled by the clock gating(1) during Sleep and Stop modes
            pub const Enabled: u32 = 0b1;
        }
    }

    /// OPAMP interface clocks enable during Sleep and Stop modes
    pub mod OPAMPSMEN {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (1 bit: 1 << 30)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: OPAMP interface clocks disabled by the clock gating during Sleep and Stop modes
            pub const Disabled: u32 = 0b0;

            /// 0b1: OPAMP interface clocks enabled by the clock gating(1) during Sleep and Stop modes
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Low power timer 1 clocks enable during Sleep and Stop modes
    pub mod LPTIM1SMEN {
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

            /// 0b0: LPTIM1 clocks disabled by the clock gating during Sleep and Stop modes
            pub const Disabled: u32 = 0b0;

            /// 0b1: LPTIM1 clocks enabled by the clock gating(1) during Sleep and Stop modes
            pub const Enabled: u32 = 0b1;
        }
    }
}

/// APB1 peripheral clocks enable in Sleep and Stop modes register 2
pub mod APB1SMENR2 {

    /// Low power UART 1 clocks enable during Sleep and Stop modes
    pub mod LPUART1SMEN {
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

            /// 0b0: LPUART1 clocks disabled by the clock gating during Sleep and Stop modes
            pub const Disabled: u32 = 0b0;

            /// 0b1: LPUART1 clocks enabled by the clock gating(1) during Sleep and Stop modes
            pub const Enabled: u32 = 0b1;
        }
    }

    /// I2C4 clocks enable during Sleep and Stop modes
    pub mod I2C4SMEN {
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

            /// 0b0: I2C4 clocks disabled by the clock gating during Sleep and Stop modes
            pub const Disabled: u32 = 0b0;

            /// 0b1: I2C4 clocks enabled by the clock gating(1) during Sleep and Stop modes
            pub const Enabled: u32 = 0b1;
        }
    }

    /// LPTIM2SMEN
    pub mod LPTIM2SMEN {
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

            /// 0b0: LPTIM2 clocks disabled by the clock gating during Sleep and Stop modes
            pub const Disabled: u32 = 0b0;

            /// 0b1: LPTIM2 clocks enabled by the clock gating(1) during Sleep and Stop modes
            pub const Enabled: u32 = 0b1;
        }
    }
}

/// APB2SMENR
pub mod APB2SMENR {

    /// SYSCFG clocks enable during Sleep and Stop modes
    pub mod SYSCFGSMEN {
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

            /// 0b0: SYSCFG + COMP + VREFBUF clocks disabled by the clock gating during Sleep and Stop modes
            pub const Disabled: u32 = 0b0;

            /// 0b1: SYSCFG + COMP + VREFBUF clocks enabled by the clock gating(1) during Sleep and Stop modes
            pub const Enabled: u32 = 0b1;
        }
    }

    /// TIM1 timer clocks enable during Sleep and Stop modes
    pub mod TIM1SMEN {
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

            /// 0b0: TIMx clocks disabled by the clock gating during Sleep and Stop modes
            pub const Disabled: u32 = 0b0;

            /// 0b1: TIMx clocks enabled by the clock gating(1) during Sleep and Stop modes
            pub const Enabled: u32 = 0b1;
        }
    }

    /// SPI1 clocks enable during Sleep and Stop modes
    pub mod SPI1SMEN {
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

            /// 0b0: SPI1 clocks disabled by the clock gating during Sleep and Stop modes
            pub const Disabled: u32 = 0b0;

            /// 0b1: SPI1 clocks enabled by the clock gating(1) during Sleep and Stop modes
            pub const Enabled: u32 = 0b1;
        }
    }

    /// TIM8 timer clocks enable during Sleep and Stop modes
    pub mod TIM8SMEN {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TIM1SMEN::RW;
    }

    /// USART1clocks enable during Sleep and Stop modes
    pub mod USART1SMEN {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: USART1 clocks disabled by the clock gating during Sleep and Stop modes
            pub const Disabled: u32 = 0b0;

            /// 0b1: USART1 clocks enabled by the clock gating(1) during Sleep and Stop modes
            pub const Enabled: u32 = 0b1;
        }
    }

    /// TIM15 timer clocks enable during Sleep and Stop modes
    pub mod TIM15SMEN {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TIM1SMEN::RW;
    }

    /// TIM16 timer clocks enable during Sleep and Stop modes
    pub mod TIM16SMEN {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TIM1SMEN::RW;
    }

    /// TIM17 timer clocks enable during Sleep and Stop modes
    pub mod TIM17SMEN {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (1 bit: 1 << 18)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TIM1SMEN::RW;
    }

    /// SAI1 clocks enable during Sleep and Stop modes
    pub mod SAI1SMEN {
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

            /// 0b0: SAIx clocks disabled by the clock gating during Sleep and Stop modes
            pub const Disabled: u32 = 0b0;

            /// 0b1: SAIx clocks enabled by the clock gating(1) during Sleep and Stop modes
            pub const Enabled: u32 = 0b1;
        }
    }

    /// SAI2 clocks enable during Sleep and Stop modes
    pub mod SAI2SMEN {
        /// Offset (22 bits)
        pub const offset: u32 = 22;
        /// Mask (1 bit: 1 << 22)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::SAI1SMEN::RW;
    }

    /// DFSDM timer clocks enable during Sleep and Stop modes
    pub mod DFSDM1SMEN {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (1 bit: 1 << 24)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: DFSDM1 clocks disabled by the clock gating during Sleep and Stop modes
            pub const Disabled: u32 = 0b0;

            /// 0b1: DFSDM1 clocks enabled by the clock gating(1) during Sleep and Stop modes
            pub const Enabled: u32 = 0b1;
        }
    }

    /// LCD-TFT timer clocks enable during Sleep and Stop modes
    pub mod LTDCSMEN {
        /// Offset (26 bits)
        pub const offset: u32 = 26;
        /// Mask (1 bit: 1 << 26)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: LCD-TFT clocks disabled by the clock gating during Sleep and Stop modes
            pub const Disabled: u32 = 0b0;

            /// 0b1: LCD-TFT clocks enabled by the clock gating(1) during Sleep and Stop modes
            pub const Enabled: u32 = 0b1;
        }
    }

    /// DSI clocks enable during Sleep and Stop modes
    pub mod DSISMEN {
        /// Offset (27 bits)
        pub const offset: u32 = 27;
        /// Mask (1 bit: 1 << 27)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: DSI clocks disabled by the clock gating during Sleep and Stop modes
            pub const Disabled: u32 = 0b0;

            /// 0b1: DSI clocks enabled by the clock gating(1) during Sleep and Stop modes
            pub const Enabled: u32 = 0b1;
        }
    }
}

/// CCIPR
pub mod CCIPR {

    /// ADCs clock source selection
    pub mod ADCSEL {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (2 bits: 0b11 << 28)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: No clock selected
            pub const NoClock: u32 = 0b00;

            /// 0b01: PLLADC1CLK clock selected
            pub const PLLADC1CLK: u32 = 0b01;

            /// 0b11: SYSCLK clock selected
            pub const SYSCLK: u32 = 0b11;
        }
    }

    /// 48 MHz clock source selection
    pub mod CLK48SEL {
        /// Offset (26 bits)
        pub const offset: u32 = 26;
        /// Mask (2 bits: 0b11 << 26)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: HSI48 clock selected (only for STM32L41x/L42x/L43x/L44x/L45x/L46x/L49x/L4Ax devices, otherwise no clock selected)
            pub const HSI48: u32 = 0b00;

            /// 0b01: PLL48M2CLK clock selected
            pub const PLL48M2CLK: u32 = 0b01;

            /// 0b10: PLL48M1CLK clock selected
            pub const PLL48M1CLK: u32 = 0b10;

            /// 0b11: MSI clock selected
            pub const MSI: u32 = 0b11;
        }
    }

    /// SAI2 clock source selection
    pub mod SAI2SEL {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (2 bits: 0b11 << 24)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// SAI1 clock source selection
    pub mod SAI1SEL {
        /// Offset (22 bits)
        pub const offset: u32 = 22;
        /// Mask (2 bits: 0b11 << 22)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Low power timer 2 clock source selection
    pub mod LPTIM2SEL {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (2 bits: 0b11 << 20)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: PCLK clock selected
            pub const PCLK: u32 = 0b00;

            /// 0b01: LSI clock selected
            pub const LSI: u32 = 0b01;

            /// 0b10: HSI16 clock selected
            pub const HSI16: u32 = 0b10;

            /// 0b11: LSE clock selected
            pub const LSE: u32 = 0b11;
        }
    }

    /// Low power timer 1 clock source selection
    pub mod LPTIM1SEL {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (2 bits: 0b11 << 18)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LPTIM2SEL::RW;
    }

    /// I2C3 clock source selection
    pub mod I2C3SEL {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (2 bits: 0b11 << 16)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: PCLK clock selected
            pub const PCLK: u32 = 0b00;

            /// 0b01: SYSCLK clock selected
            pub const SYSCLK: u32 = 0b01;

            /// 0b10: HSI16 clock selected
            pub const HSI16: u32 = 0b10;
        }
    }

    /// I2C2 clock source selection
    pub mod I2C2SEL {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (2 bits: 0b11 << 14)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::I2C3SEL::RW;
    }

    /// I2C1 clock source selection
    pub mod I2C1SEL {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (2 bits: 0b11 << 12)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::I2C3SEL::RW;
    }

    /// LPUART1 clock source selection
    pub mod LPUART1SEL {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (2 bits: 0b11 << 10)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: PCLK clock selected
            pub const PCLK: u32 = 0b00;

            /// 0b01: SYSCLK clock selected
            pub const SYSCLK: u32 = 0b01;

            /// 0b10: HSI16 clock selected
            pub const HSI16: u32 = 0b10;

            /// 0b11: LSE clock selected
            pub const LSE: u32 = 0b11;
        }
    }

    /// UART5 clock source selection
    pub mod UART5SEL {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (2 bits: 0b11 << 8)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LPUART1SEL::RW;
    }

    /// UART4 clock source selection
    pub mod UART4SEL {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (2 bits: 0b11 << 6)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LPUART1SEL::RW;
    }

    /// USART3 clock source selection
    pub mod USART3SEL {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (2 bits: 0b11 << 4)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LPUART1SEL::RW;
    }

    /// USART2 clock source selection
    pub mod USART2SEL {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (2 bits: 0b11 << 2)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LPUART1SEL::RW;
    }

    /// USART1 clock source selection
    pub mod USART1SEL {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (2 bits: 0b11 << 0)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LPUART1SEL::RW;
    }
}

/// BDCR
pub mod BDCR {

    /// Low speed clock output selection
    pub mod LSCOSEL {
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

            /// 0b0: LSI clock selected
            pub const LSI: u32 = 0b0;

            /// 0b1: LSE clock selected
            pub const LSE: u32 = 0b1;
        }
    }

    /// Low speed clock output enable
    pub mod LSCOEN {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (1 bit: 1 << 24)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Low speed clock output (LSCO) disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Low speed clock output (LSCO) enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Backup domain software reset
    pub mod BDRST {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Reset not activated
            pub const NoReset: u32 = 0b0;

            /// 0b1: Reset the entire Backup domain
            pub const Reset: u32 = 0b1;
        }
    }

    /// RTC clock enable
    pub mod RTCEN {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: RTC clock disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: RTC clock enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// RTC clock source selection
    pub mod RTCSEL {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (2 bits: 0b11 << 8)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: No clock
            pub const NoClock: u32 = 0b00;

            /// 0b01: LSE oscillator clock selected
            pub const LSE: u32 = 0b01;

            /// 0b10: LSI oscillator clock selected
            pub const LSI: u32 = 0b10;

            /// 0b11: HSE oscillator clock divided by 32 selected
            pub const HSE: u32 = 0b11;
        }
    }

    /// LSECSSD
    pub mod LSECSSD {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: No failure detected on LSE (32 kHz oscillator)
            pub const NoFailure: u32 = 0b0;

            /// 0b1: Failure detected on LSE (32 kHz oscillator)
            pub const FailureDetected: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// LSECSSON
    pub mod LSECSSON {
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

            /// 0b0: CSS on LSE (32 kHz external oscillator) OFF
            pub const Disabled: u32 = 0b0;

            /// 0b1: CSS on LSE (32 kHz external oscillator) ON
            pub const Enabled: u32 = 0b1;
        }
    }

    /// SE oscillator drive capability
    pub mod LSEDRV {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (2 bits: 0b11 << 3)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: ‘Xtal mode’ lower driving capability
            pub const Low: u32 = 0b00;

            /// 0b01: ‘Xtal mode’ medium low driving capability
            pub const MediumLow: u32 = 0b01;

            /// 0b10: ‘Xtal mode’ medium high driving capability
            pub const MediumHigh: u32 = 0b10;

            /// 0b11: ‘Xtal mode’ higher driving capability
            pub const High: u32 = 0b11;
        }
    }

    /// LSE oscillator bypass
    pub mod LSEBYP {
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

            /// 0b0: LSE oscillator not bypassed
            pub const NotBypassed: u32 = 0b0;

            /// 0b1: LSE oscillator bypassed
            pub const Bypassed: u32 = 0b1;
        }
    }

    /// LSE oscillator ready
    pub mod LSERDY {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: LSE oscillator not ready
            pub const NotReady: u32 = 0b0;

            /// 0b1: LSE oscillator ready
            pub const Ready: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// LSE oscillator enable
    pub mod LSEON {
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

            /// 0b0: LSE oscillator OFF
            pub const Disabled: u32 = 0b0;

            /// 0b1: LSE oscillator ON
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Disable the Clock LSE propagation to the system
    pub mod LSESYSDIS {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: No clock LSE propagation
            pub const Disabled: u32 = 0b0;

            /// 0b1: Clock LSE propagation enabled
            pub const Enabled: u32 = 0b1;
        }
    }
}

/// CSR
pub mod CSR {

    /// Low-power reset flag
    pub mod LPWRRSTF {
        /// Offset (31 bits)
        pub const offset: u32 = 31;
        /// Mask (1 bit: 1 << 31)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: No illegal mode reset occurred
            pub const NotOccured: u32 = 0b0;

            /// 0b1: Illegal mode reset occurred
            pub const Occured: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Window watchdog reset flag
    pub mod WWDGRSTF {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (1 bit: 1 << 30)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: No window watchdog reset occurred
            pub const NotOccured: u32 = 0b0;

            /// 0b1: Window watchdog reset occurred
            pub const Occured: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Independent window watchdog reset flag
    pub mod IWDGRSTF {
        /// Offset (29 bits)
        pub const offset: u32 = 29;
        /// Mask (1 bit: 1 << 29)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: No independent watchdog reset occurred
            pub const NotOccured: u32 = 0b0;

            /// 0b1: Independent watchdog reset occurred
            pub const Occured: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Software reset flag
    pub mod SFTRSTF {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (1 bit: 1 << 28)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: No software reset occurred
            pub const NotOccured: u32 = 0b0;

            /// 0b1: Software reset occurred
            pub const Occured: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// BOR flag
    pub mod BORRSTF {
        /// Offset (27 bits)
        pub const offset: u32 = 27;
        /// Mask (1 bit: 1 << 27)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: No BOR occurred
            pub const NotOccured: u32 = 0b0;

            /// 0b1: BOR occurred
            pub const Occured: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Pin reset flag
    pub mod PINRSTF {
        /// Offset (26 bits)
        pub const offset: u32 = 26;
        /// Mask (1 bit: 1 << 26)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: No reset from NRST pin occurred
            pub const NotOccured: u32 = 0b0;

            /// 0b1: Reset from NRST pin occurred
            pub const Occured: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Option byte loader reset flag
    pub mod OBLRSTF {
        /// Offset (25 bits)
        pub const offset: u32 = 25;
        /// Mask (1 bit: 1 << 25)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: No reset from Option Byte loading occurred
            pub const NotOccured: u32 = 0b0;

            /// 0b1: Reset from Option Byte loading occurred
            pub const Occured: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Firewall reset flag
    pub mod FWRSTF {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (1 bit: 1 << 24)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: No reset from the firewall occurred
            pub const NotOccured: u32 = 0b0;

            /// 0b1: Reset from the firewall occurred
            pub const Occured: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Remove reset flag
    pub mod RMVF {
        /// Offset (23 bits)
        pub const offset: u32 = 23;
        /// Mask (1 bit: 1 << 23)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: No effect
            pub const NoEffect: u32 = 0b0;

            /// 0b1: Clear the reset flags
            pub const Clear: u32 = 0b1;
        }
    }

    /// SI range after Standby mode
    pub mod MSISRANGE {
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

            /// 0b0100: range 4 around 1 MHz
            pub const Range1M: u32 = 0b0100;

            /// 0b0101: range 5 around 2 MHz
            pub const Range2M: u32 = 0b0101;

            /// 0b0110: range 6 around 4 MHz
            pub const Range4M: u32 = 0b0110;

            /// 0b0111: range 7 around 8 MHz
            pub const Range8M: u32 = 0b0111;
        }
    }

    /// LSI oscillator ready
    pub mod LSIRDY {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: LSI oscillator not ready
            pub const NotReady: u32 = 0b0;

            /// 0b1: LSI oscillator ready
            pub const Ready: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// LSI oscillator enable
    pub mod LSION {
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

            /// 0b0: LSI oscillator OFF
            pub const Disabled: u32 = 0b0;

            /// 0b1: LSI oscillator ON
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Internal low-speed oscillator predivided by 128
    pub mod LSIPREDIV {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: LSI PREDIV OFF
            pub const Disabled: u32 = 0b0;

            /// 0b1: LSI PREDIV ON
            pub const Enabled: u32 = 0b1;
        }
    }
}

/// Clock recovery RC register
pub mod CRRCR {

    /// HSI48 clock enable
    pub mod HSI48ON {
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

            /// 0b0: HSI48 oscillator OFF
            pub const Disabled: u32 = 0b0;

            /// 0b1: HSI48 oscillator ON
            pub const Enabled: u32 = 0b1;
        }
    }

    /// HSI48 clock ready flag
    pub mod HSI48RDY {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: HSI48 oscillator not ready
            pub const NotReady: u32 = 0b0;

            /// 0b1: HSI48 oscillator ready
            pub const Ready: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// HSI48 clock calibration
    pub mod HSI48CAL {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (9 bits: 0x1ff << 7)
        pub const mask: u32 = 0x1ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Peripherals independent clock configuration register
pub mod CCIPR2 {

    /// I2C4 clock source selection
    pub mod I2C4SEL {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (2 bits: 0b11 << 0)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: PCLK clock selected
            pub const PCLK: u32 = 0b00;

            /// 0b01: SYSCLK clock selected
            pub const SYSCLK: u32 = 0b01;

            /// 0b10: HSI16 clock selected
            pub const HSI16: u32 = 0b10;
        }
    }

    /// Digital filter for sigma delta modulator kernel clock source selection
    pub mod DFSDMSEL {
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

            /// 0b0: APB2 clock (PCLK2) selected as DFSDM kernel clock
            pub const PCLK2: u32 = 0b0;

            /// 0b1: System clock selected as DFSDM kernel clock
            pub const SYSCLK: u32 = 0b1;
        }
    }

    /// Digital filter for sigma delta modulator audio clock source selection
    pub mod ADFSDMSEL {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (2 bits: 0b11 << 3)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: SAI1clock selected as DFSDM audio clock
            pub const SAI1: u32 = 0b00;

            /// 0b01: HSI clock selected as DFSDM audio clock
            pub const HSI: u32 = 0b01;

            /// 0b10: MSI clock selected as DFSDM audio clock
            pub const MSI: u32 = 0b10;
        }
    }

    /// SAI1 clock source selection
    pub mod SAI1SEL {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (3 bits: 0b111 << 5)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: PLLSAI1CLK clock is selected as SAIx clock
            pub const PLLSAI1CLK: u32 = 0b000;

            /// 0b001: PLLSAI2CLK clock is selected as SAIx clock
            pub const PLLSAI2CLK: u32 = 0b001;

            /// 0b010: PLLSAI3CLK clock is selected as SAIx clock
            pub const PLLSAI3CLK: u32 = 0b010;

            /// 0b011: External clock SAIx_EXTCLK clock selected as SAIx clock
            pub const SAI2_EXTCLK: u32 = 0b011;

            /// 0b100: HSI clock selected as SAIx clock
            pub const HSI: u32 = 0b100;
        }
    }

    /// SAI2 clock source selection
    pub mod SAI2SEL {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (3 bits: 0b111 << 8)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::SAI1SEL::RW;
    }

    /// clock selection
    pub mod DSISEL {
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

            /// 0b0: DSI-PHY is selected as DSI byte lane clock source (usual case)
            pub const DSIPHY: u32 = 0b0;

            /// 0b1: PLLDSICLK is selected as DSI byte lane clock source, used in case DSI PLL and DSIPHY are off (low-power mode)
            pub const PLLDSICLK: u32 = 0b1;
        }
    }

    /// SDMMC clock selection
    pub mod SDMMCSEL {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: 48 MHz clock is selected as SDMMC kernel clock
            pub const HSI48: u32 = 0b0;

            /// 0b1: PLLSAI3CLK is selected as SDMMC kernel clock, used in case higher frequency than 48MHz is needed (for SDR50 mode)
            pub const PLLSAI3CLK: u32 = 0b1;
        }
    }

    /// division factor for LTDC clock
    pub mod PLLSAI2DIVR {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (2 bits: 0b11 << 16)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: PLLSAI2DIVR = /2
            pub const Div2: u32 = 0b00;

            /// 0b01: PLLSAI2DIVR = /4
            pub const Div4: u32 = 0b01;

            /// 0b10: PLLSAI2DIVR = /8
            pub const Div8: u32 = 0b10;

            /// 0b11: PLLSAI2DIVR = /16
            pub const Div16: u32 = 0b11;
        }
    }

    /// Octospi clock source selection
    pub mod OSPISEL {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (2 bits: 0b11 << 20)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: System clock selected as OctoSPI kernel clock
            pub const SYSCLK: u32 = 0b00;

            /// 0b01: MSI clock selected as OctoSPI kernel clock
            pub const MSI: u32 = 0b01;

            /// 0b10: PLL48M1CLK clock selected as OctoSPI kernel clock
            pub const PLL48M1CLK: u32 = 0b10;
        }
    }
}

/// delay configuration register
pub mod DLYCFGR {

    /// Delay sampling configuration on OCTOSPI2 to be used for internal sampling clock (called feedback clock) or for DQS data strobe
    pub mod OCTOSPI2_DLY {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Delay sampling configuration on OCTOSPI1 to be used for internal sampling clock (called feedback clock) or for DQS data strobe
    pub mod OCTOSPI1_DLY {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
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
    /// Clock control register
    pub CR: RWRegister<u32>,

    /// Internal clock sources calibration register
    pub ICSCR: RWRegister<u32>,

    /// Clock configuration register
    pub CFGR: RWRegister<u32>,

    /// PLL configuration register
    pub PLLCFGR: RWRegister<u32>,

    /// PLLSAI1 configuration register
    pub PLLSAI1CFGR: RWRegister<u32>,

    /// PLLSAI2 configuration register
    pub PLLSAI2CFGR: RWRegister<u32>,

    /// Clock interrupt enable register
    pub CIER: RWRegister<u32>,

    /// Clock interrupt flag register
    pub CIFR: RORegister<u32>,

    /// Clock interrupt clear register
    pub CICR: WORegister<u32>,

    _reserved1: [u8; 4],

    /// AHB1 peripheral reset register
    pub AHB1RSTR: RWRegister<u32>,

    /// AHB2 peripheral reset register
    pub AHB2RSTR: RWRegister<u32>,

    /// AHB3 peripheral reset register
    pub AHB3RSTR: RWRegister<u32>,

    _reserved2: [u8; 4],

    /// APB1 peripheral reset register 1
    pub APB1RSTR1: RWRegister<u32>,

    /// APB1 peripheral reset register 2
    pub APB1RSTR2: RWRegister<u32>,

    /// APB2 peripheral reset register
    pub APB2RSTR: RWRegister<u32>,

    _reserved3: [u8; 4],

    /// AHB1 peripheral clock enable register
    pub AHB1ENR: RWRegister<u32>,

    /// AHB2 peripheral clock enable register
    pub AHB2ENR: RWRegister<u32>,

    /// AHB3 peripheral clock enable register
    pub AHB3ENR: RWRegister<u32>,

    _reserved4: [u8; 4],

    /// APB1ENR1
    pub APB1ENR1: RWRegister<u32>,

    /// APB1 peripheral clock enable register 2
    pub APB1ENR2: RWRegister<u32>,

    /// APB2ENR
    pub APB2ENR: RWRegister<u32>,

    _reserved5: [u8; 4],

    /// AHB1 peripheral clocks enable in Sleep and Stop modes register
    pub AHB1SMENR: RWRegister<u32>,

    /// AHB2 peripheral clocks enable in Sleep and Stop modes register
    pub AHB2SMENR: RWRegister<u32>,

    /// AHB3 peripheral clocks enable in Sleep and Stop modes register
    pub AHB3SMENR: RWRegister<u32>,

    _reserved6: [u8; 4],

    /// APB1SMENR1
    pub APB1SMENR1: RWRegister<u32>,

    /// APB1 peripheral clocks enable in Sleep and Stop modes register 2
    pub APB1SMENR2: RWRegister<u32>,

    /// APB2SMENR
    pub APB2SMENR: RWRegister<u32>,

    _reserved7: [u8; 4],

    /// CCIPR
    pub CCIPR: RWRegister<u32>,

    _reserved8: [u8; 4],

    /// BDCR
    pub BDCR: RWRegister<u32>,

    /// CSR
    pub CSR: RWRegister<u32>,

    /// Clock recovery RC register
    pub CRRCR: RWRegister<u32>,

    /// Peripherals independent clock configuration register
    pub CCIPR2: RWRegister<u32>,

    _reserved9: [u8; 4],

    /// delay configuration register
    pub DLYCFGR: RWRegister<u32>,
}
pub struct ResetValues {
    pub CR: u32,
    pub ICSCR: u32,
    pub CFGR: u32,
    pub PLLCFGR: u32,
    pub PLLSAI1CFGR: u32,
    pub PLLSAI2CFGR: u32,
    pub CIER: u32,
    pub CIFR: u32,
    pub CICR: u32,
    pub AHB1RSTR: u32,
    pub AHB2RSTR: u32,
    pub AHB3RSTR: u32,
    pub APB1RSTR1: u32,
    pub APB1RSTR2: u32,
    pub APB2RSTR: u32,
    pub AHB1ENR: u32,
    pub AHB2ENR: u32,
    pub AHB3ENR: u32,
    pub APB1ENR1: u32,
    pub APB1ENR2: u32,
    pub APB2ENR: u32,
    pub AHB1SMENR: u32,
    pub AHB2SMENR: u32,
    pub AHB3SMENR: u32,
    pub APB1SMENR1: u32,
    pub APB1SMENR2: u32,
    pub APB2SMENR: u32,
    pub CCIPR: u32,
    pub BDCR: u32,
    pub CSR: u32,
    pub CRRCR: u32,
    pub CCIPR2: u32,
    pub DLYCFGR: u32,
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

/// Access functions for the RCC peripheral instance
pub mod RCC {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x40021000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in RCC
    pub const reset: ResetValues = ResetValues {
        CR: 0x00000063,
        ICSCR: 0x10000000,
        CFGR: 0x00000000,
        PLLCFGR: 0x00001000,
        PLLSAI1CFGR: 0x00001000,
        PLLSAI2CFGR: 0x00001000,
        CIER: 0x00000000,
        CIFR: 0x00000000,
        CICR: 0x00000000,
        AHB1RSTR: 0x00000000,
        AHB2RSTR: 0x00000000,
        AHB3RSTR: 0x00000000,
        APB1RSTR1: 0x00000000,
        APB1RSTR2: 0x00000000,
        APB2RSTR: 0x00000000,
        AHB1ENR: 0x00000100,
        AHB2ENR: 0x00000000,
        AHB3ENR: 0x00000000,
        APB1ENR1: 0x00000400,
        APB1ENR2: 0x00000000,
        APB2ENR: 0x00000000,
        AHB1SMENR: 0x00071307,
        AHB2SMENR: 0x005777FF,
        AHB3SMENR: 0x00000301,
        APB1SMENR1: 0xF3FECC3F,
        APB1SMENR2: 0x00000023,
        APB2SMENR: 0x0D677801,
        CCIPR: 0x00000000,
        BDCR: 0x00000000,
        CSR: 0x0C000600,
        CRRCR: 0x00000000,
        CCIPR2: 0x00000000,
        DLYCFGR: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut RCC_TAKEN: bool = false;

    /// Safe access to RCC
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
            if RCC_TAKEN {
                None
            } else {
                RCC_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to RCC
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if RCC_TAKEN && inst.addr == INSTANCE.addr {
                RCC_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal RCC
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        RCC_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to RCC
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const RCC: *const RegisterBlock = 0x40021000 as *const _;
