#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Digital-to-analog converter

use crate::{RORegister, RWRegister, WORegister};
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// DAC control register
pub mod CR {

    /// DAC channel1 enable
    pub mod EN1 {
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

            /// 0b0: DAC Channel X disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: DAC Channel X enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// DAC channel1 trigger enable
    pub mod TEN1 {
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

            /// 0b0: DAC Channel X trigger disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: DAC Channel X trigger enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// DAC channel1 trigger selection
    pub mod TSEL1 {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (4 bits: 0b1111 << 2)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: TIM6_TRGO event trigger for DAC conversion, if TEN is enabled
            pub const TIM6_TRGO: u32 = 0b0000;

            /// 0b0001: TIM8_TRGO
            pub const TIM8_TRGO: u32 = 0b0001;

            /// 0b0010: TIM7_TRGO (Note: Reserved on STM32L45xxx and STM32L46xxx devices)
            pub const TIM7_TRGO: u32 = 0b0010;

            /// 0b0011: TIM5_TRGO
            pub const TIM5_TRGO: u32 = 0b0011;

            /// 0b0100: TIM2_TRGO
            pub const TIM2_TRGO: u32 = 0b0100;

            /// 0b0101: TIM4_TRGO
            pub const TIM4_TRGO: u32 = 0b0101;

            /// 0b0110: External pin
            pub const EXTI9: u32 = 0b0110;

            /// 0b0111: Software triger
            pub const SWTRIG: u32 = 0b0111;
        }
    }

    /// DAC channel1 noise/triangle wave generation enable
    pub mod WAVE1 {
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

            /// 0b00: Wave generation disabled
            pub const Disabled: u32 = 0b00;

            /// 0b01: Noise wave generation enabled
            pub const Noise: u32 = 0b01;

            /// 0b10: Triangle wave generation enabled
            pub const Triangle: u32 = 0b10;
        }
    }

    /// DAC channel1 mask/amplitude selector
    pub mod MAMP1 {
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

            /// 0b0000: Unmask bit0 of LFSR/ triangle amplitude equal to 1
            pub const Amp1: u32 = 0b0000;

            /// 0b0001: Unmask bits\[1:0\] of LFSR/ triangle amplitude equal to 3
            pub const Amp3: u32 = 0b0001;

            /// 0b0010: Unmask bits\[2:0\] of LFSR/ triangle amplitude equal to 7
            pub const Amp7: u32 = 0b0010;

            /// 0b0011: Unmask bits\[3:0\] of LFSR/ triangle amplitude equal to 15
            pub const Amp15: u32 = 0b0011;

            /// 0b0100: Unmask bits\[4:0\] of LFSR/ triangle amplitude equal to 31
            pub const Amp31: u32 = 0b0100;

            /// 0b0101: Unmask bits\[5:0\] of LFSR/ triangle amplitude equal 63
            pub const Amp63: u32 = 0b0101;

            /// 0b0110: Unmask bits\[6:0\] of LFSR/ triangle amplitude equal to 127
            pub const Amp127: u32 = 0b0110;

            /// 0b0111: Unmask bits\[7:0\] of LFSR/ triangle amplitude equal to 255
            pub const Amp255: u32 = 0b0111;

            /// 0b1000: Unmask bits\[8:0\] of LFSR/ triangle amplitude equal to 511
            pub const Amp511: u32 = 0b1000;

            /// 0b1001: Unmask bits\[9:0\] of LFSR/ triangle amplitude equal to 1023
            pub const Amp1023: u32 = 0b1001;

            /// 0b1010: Unmask bits\[10:0\] of LFSR/ triangle amplitude equal to 2047
            pub const Amp2047: u32 = 0b1010;

            /// 0b1011: Unmask bits\[11:0\] of LFSR/ triangle amplitude equal to 4095
            pub const Amp4095: u32 = 0b1011;
        }
    }

    /// DAC channel1 DMA enable
    pub mod DMAEN1 {
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

            /// 0b0: DAC Channel X DMA mode disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: DAC Channel X DMA mode enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// DAC channel1 DMA Underrun Interrupt enable
    pub mod DMAUDRIE1 {
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

            /// 0b0: DAC Channel X DMA Underrun Interrupt disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: DAC Channel X DMA Underrun Interrupt enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// DAC Channel 1 calibration enable
    pub mod CEN1 {
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

            /// 0b0: DAC Channel X Normal operating mode
            pub const Normal: u32 = 0b0;

            /// 0b1: DAC Channel X calibration mode
            pub const Calibration: u32 = 0b1;
        }
    }

    /// DAC channel2 enable
    pub mod EN2 {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::EN1::RW;
    }

    /// DAC channel2 trigger enable
    pub mod TEN2 {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TEN1::RW;
    }

    /// DAC channel2 trigger selection
    pub mod TSEL2 {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (4 bits: 0b1111 << 18)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TSEL1::RW;
    }

    /// DAC channel2 noise/triangle wave generation enable
    pub mod WAVE2 {
        /// Offset (22 bits)
        pub const offset: u32 = 22;
        /// Mask (2 bits: 0b11 << 22)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::WAVE1::RW;
    }

    /// DAC channel2 mask/amplitude selector
    pub mod MAMP2 {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (4 bits: 0b1111 << 24)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MAMP1::RW;
    }

    /// DAC channel2 DMA enable
    pub mod DMAEN2 {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (1 bit: 1 << 28)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::DMAEN1::RW;
    }

    /// DAC channel2 DMA underrun interrupt enable
    pub mod DMAUDRIE2 {
        /// Offset (29 bits)
        pub const offset: u32 = 29;
        /// Mask (1 bit: 1 << 29)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::DMAUDRIE1::RW;
    }

    /// DAC Channel 2 calibration enable
    pub mod CEN2 {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (1 bit: 1 << 30)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::CEN1::RW;
    }

    /// High frequency interface mode enable
    pub mod HFSEL {
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

            /// 0b0: High frequency interface mode disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: High frequency interface mode enabled
            pub const Enabled: u32 = 0b1;
        }
    }
}

/// software trigger register
pub mod SWTRIGR {

    /// DAC channel1 software trigger
    pub mod SWTRIG1 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values
        pub mod W {

            /// 0b0: No trigger
            pub const NoTrigger: u32 = 0b0;

            /// 0b1: Trigger
            pub const Trigger: u32 = 0b1;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// DAC channel2 software trigger
    pub mod SWTRIG2 {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::SWTRIG1::W;
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// channel1 12-bit right-aligned data holding register
pub mod DHR12R1 {

    /// DAC channel1 12-bit right-aligned data
    pub mod DACC1DHR {
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

/// channel1 12-bit left-aligned data holding register
pub mod DHR12L1 {

    /// DAC channel1 12-bit left-aligned data
    pub mod DACC1DHR {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (12 bits: 0xfff << 4)
        pub const mask: u32 = 0xfff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// channel1 8-bit right-aligned data holding register
pub mod DHR8R1 {

    /// DAC channel1 8-bit right-aligned data
    pub mod DACC1DHR {
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

/// channel2 12-bit right aligned data holding register
pub mod DHR12R2 {

    /// DAC channel2 12-bit right-aligned data
    pub mod DACC2DHR {
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

/// channel2 12-bit left aligned data holding register
pub mod DHR12L2 {

    /// DAC channel2 12-bit left-aligned data
    pub mod DACC2DHR {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (12 bits: 0xfff << 4)
        pub const mask: u32 = 0xfff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// channel2 8-bit right-aligned data holding register
pub mod DHR8R2 {

    /// DAC channel2 8-bit right-aligned data
    pub mod DACC2DHR {
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

/// Dual DAC 12-bit right-aligned data holding register
pub mod DHR12RD {

    /// DAC channel1 12-bit right-aligned data
    pub mod DACC1DHR {
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

    /// DAC channel2 12-bit right-aligned data
    pub mod DACC2DHR {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (12 bits: 0xfff << 16)
        pub const mask: u32 = 0xfff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// DUAL DAC 12-bit left aligned data holding register
pub mod DHR12LD {

    /// DAC channel1 12-bit left-aligned data
    pub mod DACC1DHR {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (12 bits: 0xfff << 4)
        pub const mask: u32 = 0xfff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// DAC channel2 12-bit left-aligned data
    pub mod DACC2DHR {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (12 bits: 0xfff << 20)
        pub const mask: u32 = 0xfff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// DUAL DAC 8-bit right aligned data holding register
pub mod DHR8RD {

    /// DAC channel1 8-bit right-aligned data
    pub mod DACC1DHR {
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

    /// DAC channel2 8-bit right-aligned data
    pub mod DACC2DHR {
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
}

/// channel1 data output register
pub mod DOR1 {

    /// DAC channel1 data output
    pub mod DACC1DOR {
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

/// channel2 data output register
pub mod DOR2 {

    /// DAC channel2 data output
    pub mod DACC2DOR {
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

/// status register
pub mod SR {

    /// DAC channel1 DMA underrun flag
    pub mod DMAUDR1 {
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

            /// 0b0: No DMA underrun error condition occurred for DAC channel x
            pub const NoError: u32 = 0b0;

            /// 0b1: DMA underrun error condition occurred for DAC channel x (the currently selected trigger is driving DAC channel1 conversion at a frequency higher than the DMA service capability rate)
            pub const Error: u32 = 0b1;
        }
    }

    /// DAC Channel 1 calibration offset status
    pub mod CAL_FLAG1 {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: Calibration trimming value is lower than the offset correction value
            pub const Lower: u32 = 0b0;

            /// 0b1: Calibration trimming value is equal or greater than the offset correction value
            pub const Equal_Higher: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// DAC Channel 1 busy writing sample time flag
    pub mod BWST1 {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: There is no write operation of DAC_SHSR1 ongoing: DAC_SHSR1 can be written
            pub const Idle: u32 = 0b0;

            /// 0b1: There is a write operation of DAC_SHSR1 ongoing: DAC_SHSR1 cannot be written
            pub const Busy: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// DAC channel2 DMA underrun flag
    pub mod DMAUDR2 {
        /// Offset (29 bits)
        pub const offset: u32 = 29;
        /// Mask (1 bit: 1 << 29)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::DMAUDR1::RW;
    }

    /// DAC Channel 2 calibration offset status
    pub mod CAL_FLAG2 {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (1 bit: 1 << 30)
        pub const mask: u32 = 1 << offset;
        pub use super::CAL_FLAG1::R;
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// DAC Channel 2 busy writing sample time flag
    pub mod BWST2 {
        /// Offset (31 bits)
        pub const offset: u32 = 31;
        /// Mask (1 bit: 1 << 31)
        pub const mask: u32 = 1 << offset;
        pub use super::BWST1::R;
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// calibration control register
pub mod CCR {

    /// DAC Channel 1 offset trimming value
    pub mod OTRIM1 {
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

    /// DAC Channel 2 offset trimming value
    pub mod OTRIM2 {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (5 bits: 0b11111 << 16)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// mode control register
pub mod MCR {

    /// DAC Channel 1 mode
    pub mod MODE1 {
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

            /// 0b000: Normal mode - DAC channelx is connected to external pin with Buffer enabled
            pub const NormalPinBuffer: u32 = 0b000;

            /// 0b001: Normal mode - DAC channelx is connected to external pin and to on chip peripherals with Buffer enabled
            pub const NormalPinChipBuffer: u32 = 0b001;

            /// 0b010: Normal mode - DAC channelx is connected to external pin with Buffer disabled
            pub const NormalPinNoBuffer: u32 = 0b010;

            /// 0b011: Normal mode - DAC channelx is connected to on chip peripherals with Buffer disabled
            pub const NormalChipNoBuffer: u32 = 0b011;

            /// 0b100: S&H mode - DAC channelx is connected to external pin with Buffer enabled
            pub const SHPinBuffer: u32 = 0b100;

            /// 0b101: S&H mode - DAC channelx is connected to external pin and to on chip peripherals with Buffer enabled
            pub const SHPinChipBuffer: u32 = 0b101;

            /// 0b110: S&H mode - DAC channelx is connected to external pin and to on chip peripherals with Buffer disabled
            pub const SHPinNoBuffer: u32 = 0b110;

            /// 0b111: S&H mode - DAC channelx is connected to on chip peripherals with Buffer disabled
            pub const SHChipNoBuffer: u32 = 0b111;
        }
    }

    /// DAC Channel 2 mode
    pub mod MODE2 {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (3 bits: 0b111 << 16)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MODE1::RW;
    }
}

/// Sample and Hold sample time register 1
pub mod SHSR1 {

    /// DAC Channel 1 sample Time
    pub mod TSAMPLE1 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (10 bits: 0x3ff << 0)
        pub const mask: u32 = 0x3ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Sample and Hold sample time register 2
pub mod SHSR2 {

    /// DAC Channel 2 sample Time
    pub mod TSAMPLE2 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (10 bits: 0x3ff << 0)
        pub const mask: u32 = 0x3ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Sample and Hold hold time register
pub mod SHHR {

    /// DAC Channel 1 hold Time
    pub mod THOLD1 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (10 bits: 0x3ff << 0)
        pub const mask: u32 = 0x3ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// DAC Channel 2 hold time
    pub mod THOLD2 {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (10 bits: 0x3ff << 16)
        pub const mask: u32 = 0x3ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Sample and Hold refresh time register
pub mod SHRR {

    /// DAC Channel 1 refresh Time
    pub mod TREFRESH1 {
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

    /// DAC Channel 2 refresh Time
    pub mod TREFRESH2 {
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
}
#[repr(C)]
pub struct RegisterBlock {
    /// DAC control register
    pub CR: RWRegister<u32>,

    /// software trigger register
    pub SWTRIGR: WORegister<u32>,

    /// channel1 12-bit right-aligned data holding register
    pub DHR12R1: RWRegister<u32>,

    /// channel1 12-bit left-aligned data holding register
    pub DHR12L1: RWRegister<u32>,

    /// channel1 8-bit right-aligned data holding register
    pub DHR8R1: RWRegister<u32>,

    /// channel2 12-bit right aligned data holding register
    pub DHR12R2: RWRegister<u32>,

    /// channel2 12-bit left aligned data holding register
    pub DHR12L2: RWRegister<u32>,

    /// channel2 8-bit right-aligned data holding register
    pub DHR8R2: RWRegister<u32>,

    /// Dual DAC 12-bit right-aligned data holding register
    pub DHR12RD: RWRegister<u32>,

    /// DUAL DAC 12-bit left aligned data holding register
    pub DHR12LD: RWRegister<u32>,

    /// DUAL DAC 8-bit right aligned data holding register
    pub DHR8RD: RWRegister<u32>,

    /// channel1 data output register
    pub DOR1: RORegister<u32>,

    /// channel2 data output register
    pub DOR2: RORegister<u32>,

    /// status register
    pub SR: RWRegister<u32>,

    /// calibration control register
    pub CCR: RWRegister<u32>,

    /// mode control register
    pub MCR: RWRegister<u32>,

    /// Sample and Hold sample time register 1
    pub SHSR1: RWRegister<u32>,

    /// Sample and Hold sample time register 2
    pub SHSR2: RWRegister<u32>,

    /// Sample and Hold hold time register
    pub SHHR: RWRegister<u32>,

    /// Sample and Hold refresh time register
    pub SHRR: RWRegister<u32>,
}
pub struct ResetValues {
    pub CR: u32,
    pub SWTRIGR: u32,
    pub DHR12R1: u32,
    pub DHR12L1: u32,
    pub DHR8R1: u32,
    pub DHR12R2: u32,
    pub DHR12L2: u32,
    pub DHR8R2: u32,
    pub DHR12RD: u32,
    pub DHR12LD: u32,
    pub DHR8RD: u32,
    pub DOR1: u32,
    pub DOR2: u32,
    pub SR: u32,
    pub CCR: u32,
    pub MCR: u32,
    pub SHSR1: u32,
    pub SHSR2: u32,
    pub SHHR: u32,
    pub SHRR: u32,
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

/// Access functions for the DAC peripheral instance
pub mod DAC {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x40007400,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in DAC
    pub const reset: ResetValues = ResetValues {
        CR: 0x00000000,
        SWTRIGR: 0x00000000,
        DHR12R1: 0x00000000,
        DHR12L1: 0x00000000,
        DHR8R1: 0x00000000,
        DHR12R2: 0x00000000,
        DHR12L2: 0x00000000,
        DHR8R2: 0x00000000,
        DHR12RD: 0x00000000,
        DHR12LD: 0x00000000,
        DHR8RD: 0x00000000,
        DOR1: 0x00000000,
        DOR2: 0x00000000,
        SR: 0x00000000,
        CCR: 0x00000000,
        MCR: 0x00000000,
        SHSR1: 0x00000000,
        SHSR2: 0x00000000,
        SHHR: 0x00010001,
        SHRR: 0x00000001,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut DAC_TAKEN: bool = false;

    /// Safe access to DAC
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
            if DAC_TAKEN {
                None
            } else {
                DAC_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to DAC
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if DAC_TAKEN && inst.addr == INSTANCE.addr {
                DAC_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal DAC
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        DAC_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to DAC
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const DAC: *const RegisterBlock = 0x40007400 as *const _;
