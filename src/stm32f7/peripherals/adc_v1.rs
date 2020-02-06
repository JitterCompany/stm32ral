#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Analog-to-digital converter
//!
//! Used by: stm32f745, stm32f765, stm32f7x6, stm32f7x9

use crate::{RORegister, RWRegister};
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// status register
pub mod SR {

    /// Overrun
    pub mod OVR {
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

            /// 0b0: No overrun occurred
            pub const NoOverrun: u32 = 0b0;

            /// 0b1: Overrun occurred
            pub const Overrun: u32 = 0b1;
        }
    }

    /// Regular channel start flag
    pub mod STRT {
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

            /// 0b0: No regular channel conversion started
            pub const NotStarted: u32 = 0b0;

            /// 0b1: Regular channel conversion has started
            pub const Started: u32 = 0b1;
        }
    }

    /// Injected channel start flag
    pub mod JSTRT {
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

            /// 0b0: No injected channel conversion started
            pub const NotStarted: u32 = 0b0;

            /// 0b1: Injected channel conversion has started
            pub const Started: u32 = 0b1;
        }
    }

    /// Injected channel end of conversion
    pub mod JEOC {
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

            /// 0b0: Conversion is not complete
            pub const NotComplete: u32 = 0b0;

            /// 0b1: Conversion complete
            pub const Complete: u32 = 0b1;
        }
    }

    /// Regular channel end of conversion
    pub mod EOC {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::JEOC::RW;
    }

    /// Analog watchdog flag
    pub mod AWD {
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

            /// 0b0: No analog watchdog event occurred
            pub const NoEvent: u32 = 0b0;

            /// 0b1: Analog watchdog event occurred
            pub const Event: u32 = 0b1;
        }
    }
}

/// control register 1
pub mod CR1 {

    /// Overrun interrupt enable
    pub mod OVRIE {
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

            /// 0b0: Overrun interrupt disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Overrun interrupt enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Resolution
    pub mod RES {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (2 bits: 0b11 << 24)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: 12-bit (15 ADCCLK cycles)
            pub const TwelveBit: u32 = 0b00;

            /// 0b01: 10-bit (13 ADCCLK cycles)
            pub const TenBit: u32 = 0b01;

            /// 0b10: 8-bit (11 ADCCLK cycles)
            pub const EightBit: u32 = 0b10;

            /// 0b11: 6-bit (9 ADCCLK cycles)
            pub const SixBit: u32 = 0b11;
        }
    }

    /// Analog watchdog enable on regular channels
    pub mod AWDEN {
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

            /// 0b0: Analog watchdog disabled on regular channels
            pub const Disabled: u32 = 0b0;

            /// 0b1: Analog watchdog enabled on regular channels
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Analog watchdog enable on injected channels
    pub mod JAWDEN {
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

            /// 0b0: Analog watchdog disabled on injected channels
            pub const Disabled: u32 = 0b0;

            /// 0b1: Analog watchdog enabled on injected channels
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Discontinuous mode channel count
    pub mod DISCNUM {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (3 bits: 0b111 << 13)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Discontinuous mode on injected channels
    pub mod JDISCEN {
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

            /// 0b0: Discontinuous mode on injected channels disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Discontinuous mode on injected channels enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Discontinuous mode on regular channels
    pub mod DISCEN {
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

            /// 0b0: Discontinuous mode on regular channels disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Discontinuous mode on regular channels enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Automatic injected group conversion
    pub mod JAUTO {
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

            /// 0b0: Automatic injected group conversion disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Automatic injected group conversion enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Enable the watchdog on a single channel in scan mode
    pub mod AWDSGL {
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

            /// 0b0: Analog watchdog enabled on all channels
            pub const AllChannels: u32 = 0b0;

            /// 0b1: Analog watchdog enabled on a single channel
            pub const SingleChannel: u32 = 0b1;
        }
    }

    /// Scan mode
    pub mod SCAN {
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

            /// 0b0: Scan mode disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Scan mode enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Interrupt enable for injected channels
    pub mod JEOCIE {
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

            /// 0b0: JEOC interrupt disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: JEOC interrupt enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Analog watchdog interrupt enable
    pub mod AWDIE {
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

            /// 0b0: Analogue watchdog interrupt disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Analogue watchdog interrupt enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Interrupt enable for EOC
    pub mod EOCIE {
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

            /// 0b0: EOC interrupt disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: EOC interrupt enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Analog watchdog channel select bits
    pub mod AWDCH {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (5 bits: 0b11111 << 0)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// control register 2
pub mod CR2 {

    /// Start conversion of regular channels
    pub mod SWSTART {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (1 bit: 1 << 30)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values
        pub mod W {

            /// 0b1: Starts conversion of regular channels
            pub const Start: u32 = 0b1;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// External trigger enable for regular channels
    pub mod EXTEN {
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

            /// 0b00: Trigger detection disabled
            pub const Disabled: u32 = 0b00;

            /// 0b01: Trigger detection on the rising edge
            pub const RisingEdge: u32 = 0b01;

            /// 0b10: Trigger detection on the falling edge
            pub const FallingEdge: u32 = 0b10;

            /// 0b11: Trigger detection on both the rising and falling edges
            pub const BothEdges: u32 = 0b11;
        }
    }

    /// External event select for regular group
    pub mod EXTSEL {
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

            /// 0b0000: Timer 1 CC1 event
            pub const TIM1CC1: u32 = 0b0000;

            /// 0b0001: Timer 1 CC2 event
            pub const TIM1CC2: u32 = 0b0001;

            /// 0b0010: Timer 1 CC3 event
            pub const TIM1CC3: u32 = 0b0010;

            /// 0b0011: Timer 2 CC2 event
            pub const TIM2CC2: u32 = 0b0011;

            /// 0b0100: Timer 2 CC3 event
            pub const TIM2CC3: u32 = 0b0100;

            /// 0b0101: Timer 2 CC4 event
            pub const TIM2CC4: u32 = 0b0101;

            /// 0b0110: Timer 2 TRGO event
            pub const TIM2TRGO: u32 = 0b0110;
        }
    }

    /// Start conversion of injected channels
    pub mod JSWSTART {
        /// Offset (22 bits)
        pub const offset: u32 = 22;
        /// Mask (1 bit: 1 << 22)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values
        pub mod W {

            /// 0b1: Starts conversion of injected channels
            pub const Start: u32 = 0b1;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// External trigger enable for injected channels
    pub mod JEXTEN {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (2 bits: 0b11 << 20)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::EXTEN::RW;
    }

    /// External event select for injected group
    pub mod JEXTSEL {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (4 bits: 0b1111 << 16)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Timer 1 TRGO event
            pub const TIM1TRGO: u32 = 0b0000;

            /// 0b0001: Timer 1 CC4 event
            pub const TIM1CC4: u32 = 0b0001;

            /// 0b0010: Timer 2 TRGO event
            pub const TIM2TRGO: u32 = 0b0010;

            /// 0b0011: Timer 2 CC1 event
            pub const TIM2CC1: u32 = 0b0011;

            /// 0b0100: Timer 3 CC4 event
            pub const TIM3CC4: u32 = 0b0100;

            /// 0b0101: Timer 4 TRGO event
            pub const TIM4TRGO: u32 = 0b0101;

            /// 0b0111: Timer 8 CC4 event
            pub const TIM8CC4: u32 = 0b0111;

            /// 0b1000: Timer 1 TRGO(2) event
            pub const TIM1TRGO2: u32 = 0b1000;

            /// 0b1001: Timer 8 TRGO event
            pub const TIM8TRGO: u32 = 0b1001;

            /// 0b1010: Timer 8 TRGO(2) event
            pub const TIM8TRGO2: u32 = 0b1010;

            /// 0b1011: Timer 3 CC3 event
            pub const TIM3CC3: u32 = 0b1011;

            /// 0b1100: Timer 5 TRGO event
            pub const TIM5TRGO: u32 = 0b1100;

            /// 0b1101: Timer 3 CC1 event
            pub const TIM3CC1: u32 = 0b1101;

            /// 0b1110: Timer 6 TRGO event
            pub const TIM6TRGO: u32 = 0b1110;
        }
    }

    /// Data alignment
    pub mod ALIGN {
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

            /// 0b0: Right alignment
            pub const Right: u32 = 0b0;

            /// 0b1: Left alignment
            pub const Left: u32 = 0b1;
        }
    }

    /// End of conversion selection
    pub mod EOCS {
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

            /// 0b0: The EOC bit is set at the end of each sequence of regular conversions
            pub const EachSequence: u32 = 0b0;

            /// 0b1: The EOC bit is set at the end of each regular conversion
            pub const EachConversion: u32 = 0b1;
        }
    }

    /// DMA disable selection (for single ADC mode)
    pub mod DDS {
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

            /// 0b0: No new DMA request is issued after the last transfer
            pub const Single: u32 = 0b0;

            /// 0b1: DMA requests are issued as long as data are converted and DMA=1
            pub const Continuous: u32 = 0b1;
        }
    }

    /// Direct memory access mode (for single ADC mode)
    pub mod DMA {
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

            /// 0b0: DMA mode disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: DMA mode enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Continuous conversion
    pub mod CONT {
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

            /// 0b0: Single conversion mode
            pub const Single: u32 = 0b0;

            /// 0b1: Continuous conversion mode
            pub const Continuous: u32 = 0b1;
        }
    }

    /// A/D Converter ON / OFF
    pub mod ADON {
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

            /// 0b0: Disable ADC conversion and go to power down mode
            pub const Disabled: u32 = 0b0;

            /// 0b1: Enable ADC
            pub const Enabled: u32 = 0b1;
        }
    }
}

/// sample time register 1
pub mod SMPR1 {

    /// Sample time bits
    pub mod SMPx_x {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00000000000000000000000000000000: 3 cycles
            pub const Cycles3: u32 = 0b00000000000000000000000000000000;

            /// 0b00000000000000000000000000000001: 15 cycles
            pub const Cycles15: u32 = 0b00000000000000000000000000000001;

            /// 0b00000000000000000000000000000010: 28 cycles
            pub const Cycles28: u32 = 0b00000000000000000000000000000010;

            /// 0b00000000000000000000000000000011: 56 cycles
            pub const Cycles56: u32 = 0b00000000000000000000000000000011;

            /// 0b00000000000000000000000000000100: 84 cycles
            pub const Cycles84: u32 = 0b00000000000000000000000000000100;

            /// 0b00000000000000000000000000000101: 112 cycles
            pub const Cycles112: u32 = 0b00000000000000000000000000000101;

            /// 0b00000000000000000000000000000110: 144 cycles
            pub const Cycles144: u32 = 0b00000000000000000000000000000110;

            /// 0b00000000000000000000000000000111: 480 cycles
            pub const Cycles480: u32 = 0b00000000000000000000000000000111;
        }
    }
}

/// sample time register 2
pub mod SMPR2 {
    pub use super::SMPR1::SMPx_x;
}

/// injected channel data offset register x
pub mod JOFR1 {

    /// Data offset for injected channel x
    pub mod JOFFSET {
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
}

/// injected channel data offset register x
pub mod JOFR2 {
    pub use super::JOFR1::JOFFSET;
}

/// injected channel data offset register x
pub mod JOFR3 {
    pub use super::JOFR1::JOFFSET;
}

/// injected channel data offset register x
pub mod JOFR4 {
    pub use super::JOFR1::JOFFSET;
}

/// watchdog higher threshold register
pub mod HTR {

    /// Analog watchdog higher threshold
    pub mod HT {
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
}

/// watchdog lower threshold register
pub mod LTR {

    /// Analog watchdog lower threshold
    pub mod LT {
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
}

/// regular sequence register 1
pub mod SQR1 {

    /// Regular channel sequence length
    pub mod L {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (4 bits: 0b1111 << 20)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// 16th conversion in regular sequence
    pub mod SQ16 {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (5 bits: 0b11111 << 15)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// 15th conversion in regular sequence
    pub mod SQ15 {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (5 bits: 0b11111 << 10)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// 14th conversion in regular sequence
    pub mod SQ14 {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (5 bits: 0b11111 << 5)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// 13th conversion in regular sequence
    pub mod SQ13 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (5 bits: 0b11111 << 0)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// regular sequence register 2
pub mod SQR2 {

    /// 12th conversion in regular sequence
    pub mod SQ12 {
        /// Offset (25 bits)
        pub const offset: u32 = 25;
        /// Mask (5 bits: 0b11111 << 25)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// 11th conversion in regular sequence
    pub mod SQ11 {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (5 bits: 0b11111 << 20)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// 10th conversion in regular sequence
    pub mod SQ10 {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (5 bits: 0b11111 << 15)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// 9th conversion in regular sequence
    pub mod SQ9 {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (5 bits: 0b11111 << 10)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// 8th conversion in regular sequence
    pub mod SQ8 {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (5 bits: 0b11111 << 5)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// 7th conversion in regular sequence
    pub mod SQ7 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (5 bits: 0b11111 << 0)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// regular sequence register 3
pub mod SQR3 {

    /// 6th conversion in regular sequence
    pub mod SQ6 {
        /// Offset (25 bits)
        pub const offset: u32 = 25;
        /// Mask (5 bits: 0b11111 << 25)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// 5th conversion in regular sequence
    pub mod SQ5 {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (5 bits: 0b11111 << 20)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// 4th conversion in regular sequence
    pub mod SQ4 {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (5 bits: 0b11111 << 15)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// 3rd conversion in regular sequence
    pub mod SQ3 {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (5 bits: 0b11111 << 10)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// 2nd conversion in regular sequence
    pub mod SQ2 {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (5 bits: 0b11111 << 5)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// 1st conversion in regular sequence
    pub mod SQ1 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (5 bits: 0b11111 << 0)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// injected sequence register
pub mod JSQR {

    /// Injected sequence length
    pub mod JL {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (2 bits: 0b11 << 20)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// 4th conversion in injected sequence
    pub mod JSQ4 {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (5 bits: 0b11111 << 15)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// 3rd conversion in injected sequence
    pub mod JSQ3 {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (5 bits: 0b11111 << 10)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// 2nd conversion in injected sequence
    pub mod JSQ2 {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (5 bits: 0b11111 << 5)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// 1st conversion in injected sequence
    pub mod JSQ1 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (5 bits: 0b11111 << 0)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// injected data register x
pub mod JDR1 {

    /// Injected data
    pub mod JDATA {
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
}

/// injected data register x
pub mod JDR2 {
    pub use super::JDR1::JDATA;
}

/// injected data register x
pub mod JDR3 {
    pub use super::JDR1::JDATA;
}

/// injected data register x
pub mod JDR4 {
    pub use super::JDR1::JDATA;
}

/// regular data register
pub mod DR {

    /// Regular data
    pub mod DATA {
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
}
pub struct RegisterBlock {
    /// status register
    pub SR: RWRegister<u32>,

    /// control register 1
    pub CR1: RWRegister<u32>,

    /// control register 2
    pub CR2: RWRegister<u32>,

    /// sample time register 1
    pub SMPR1: RWRegister<u32>,

    /// sample time register 2
    pub SMPR2: RWRegister<u32>,

    /// injected channel data offset register x
    pub JOFR1: RWRegister<u32>,

    /// injected channel data offset register x
    pub JOFR2: RWRegister<u32>,

    /// injected channel data offset register x
    pub JOFR3: RWRegister<u32>,

    /// injected channel data offset register x
    pub JOFR4: RWRegister<u32>,

    /// watchdog higher threshold register
    pub HTR: RWRegister<u32>,

    /// watchdog lower threshold register
    pub LTR: RWRegister<u32>,

    /// regular sequence register 1
    pub SQR1: RWRegister<u32>,

    /// regular sequence register 2
    pub SQR2: RWRegister<u32>,

    /// regular sequence register 3
    pub SQR3: RWRegister<u32>,

    /// injected sequence register
    pub JSQR: RWRegister<u32>,

    /// injected data register x
    pub JDR1: RORegister<u32>,

    /// injected data register x
    pub JDR2: RORegister<u32>,

    /// injected data register x
    pub JDR3: RORegister<u32>,

    /// injected data register x
    pub JDR4: RORegister<u32>,

    /// regular data register
    pub DR: RORegister<u32>,
}
pub struct ResetValues {
    pub SR: u32,
    pub CR1: u32,
    pub CR2: u32,
    pub SMPR1: u32,
    pub SMPR2: u32,
    pub JOFR1: u32,
    pub JOFR2: u32,
    pub JOFR3: u32,
    pub JOFR4: u32,
    pub HTR: u32,
    pub LTR: u32,
    pub SQR1: u32,
    pub SQR2: u32,
    pub SQR3: u32,
    pub JSQR: u32,
    pub JDR1: u32,
    pub JDR2: u32,
    pub JDR3: u32,
    pub JDR4: u32,
    pub DR: u32,
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
