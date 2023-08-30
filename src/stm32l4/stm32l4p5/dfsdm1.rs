#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Digital filter for sigma delta modulators

use crate::{RORegister, RWRegister};
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// channel configuration y register
pub mod CFGR10 {

    /// DFSDMEN
    pub mod DFSDMEN {
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

            /// 0b0: DFSDM interface disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: DFSDM interface enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// CKOUTSRC
    pub mod CKOUTSRC {
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

            /// 0b0: Source for output clock is from system clock
            pub const SYSCLK: u32 = 0b0;

            /// 0b1: Source for output clock is from audio clock
            pub const AUDCLK: u32 = 0b1;
        }
    }

    /// CKOUTDIV
    pub mod CKOUTDIV {
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

    /// DATPACK
    pub mod DATPACK {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (2 bits: 0b11 << 14)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Standard: input data in DFSDM_CHyDATINR register are stored only in INDAT0\[15:0\]. To empty DFSDM_CHyDATINR register one sample must be read by the DFSDM filter from channel y
            pub const Standard: u32 = 0b00;

            /// 0b01: : Interleaved: input data in DFSDM_CHyDATINR register are stored as two samples: –first sample in INDAT0\[15:0\] (assigned to channel y) –second sample INDAT1\[15:0\] (assigned to channel y)
            pub const Interleaved: u32 = 0b01;

            /// 0b10: Dual: input data in DFSDM_CHyDATINR register are stored as two samples: –first sample INDAT0\[15:0\] (assigned to channel y) –second sample INDAT1\[15:0\] (assigned to channel y+1)
            pub const Dual: u32 = 0b10;
        }
    }

    /// DATMPX
    pub mod DATMPX {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (2 bits: 0b11 << 12)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Data to channel y are taken from external serial inputs as 1-bit values. DFSDM_CHyDATINR register is write protected
            pub const External: u32 = 0b00;

            /// 0b01: Data to channel y are taken from internal analog to digital converter ADCy+1 output register update as 16-bit values (if ADCy+1 is available). Data from ADCs are written into INDAT0\[15:0\] part of DFSDM_CHyDATINR register
            pub const ADC: u32 = 0b01;

            /// 0b10: Data to channel y are taken from internal DFSDM_CHyDATINR register by direct CPU/DMA write. There can be written one or two 16-bit data samples according DATPACK\[1:0\] bit field setting
            pub const Internal: u32 = 0b10;
        }
    }

    /// CHINSEL
    pub mod CHINSEL {
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

            /// 0b0: Channel inputs are taken from pins of the same channel y
            pub const SameChannel: u32 = 0b0;

            /// 0b1: Channel inputs are taken from pins of the following channel (channel (y+1) modulo 8)
            pub const FollowingChannel: u32 = 0b1;
        }
    }

    /// CHEN
    pub mod CHEN {
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

            /// 0b0: Channel y disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Channel y enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// CKABEN
    pub mod CKABEN {
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

            /// 0b0: Clock absence detector disabled on channel y
            pub const Disabled: u32 = 0b0;

            /// 0b1: Clock absence detector enabled on channel y
            pub const Enabled: u32 = 0b1;
        }
    }

    /// SCDEN
    pub mod SCDEN {
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

            /// 0b0: Input channel y will not be guarded by the short-circuit detector
            pub const Disabled: u32 = 0b0;

            /// 0b1: Input channel y will be continuously guarded by the short-circuit detector
            pub const Enabled: u32 = 0b1;
        }
    }

    /// SPICKSEL
    pub mod SPICKSEL {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (2 bits: 0b11 << 2)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Clock coming from external CKINy input - sampling point according SITP\[1:0\]
            pub const CKIN: u32 = 0b00;

            /// 0b01: Clock coming from internal CKOUT output - sampling point according SITP\[1:0\]
            pub const CKOUT: u32 = 0b01;

            /// 0b10: Clock coming from internal CKOUT - sampling point on each second CKOUT falling edge. For connection to external Σ∆ modulator which divides its clock input (from CKOUT) by 2 to generate its output serial communication clock (and this output clock change is active on each clock input rising edge)
            pub const CKOUTSecondFalling: u32 = 0b10;

            /// 0b11: Clock coming from internal CKOUT output - sampling point on each second CKOUT rising edge. For connection to external Σ∆ modulator which divides its clock input (from CKOUT) by 2 to generate its output serial communication clock (and this output clock change is active on each clock input falling edge)
            pub const CKOUTSecondRising: u32 = 0b11;
        }
    }

    /// SITP
    pub mod SITP {
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

            /// 0b00: SPI with rising edge to strobe data
            pub const SPIRisingEdge: u32 = 0b00;

            /// 0b01: SPI with falling edge to strobe data
            pub const SPIFallingEdge: u32 = 0b01;

            /// 0b10: Manchester coded input on DATINy pin: rising edge = logic 0, falling edge = logic 1
            pub const Manchester: u32 = 0b10;

            /// 0b11: Manchester coded input on DATINy pin: rising edge = logic 1, falling edge = logic 0
            pub const ManchesterInverted: u32 = 0b11;
        }
    }
}

/// channel configuration y register
pub mod CFGR20 {

    /// OFFSET
    pub mod OFFSET {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (24 bits: 0xffffff << 8)
        pub const mask: u32 = 0xffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// DTRBS
    pub mod DTRBS {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (5 bits: 0b11111 << 3)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// analog watchdog and short-circuit detector register
pub mod AWSCDR0 {

    /// AWFORD
    pub mod AWFORD {
        /// Offset (22 bits)
        pub const offset: u32 = 22;
        /// Mask (2 bits: 0b11 << 22)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: FastSinc filter type
            pub const FastSinc: u32 = 0b00;

            /// 0b01: Sinc1 filter type
            pub const Sinc1: u32 = 0b01;

            /// 0b10: Sinc2 filter type
            pub const Sinc2: u32 = 0b10;

            /// 0b11: Sinc3 filter type
            pub const Sinc3: u32 = 0b11;
        }
    }

    /// AWFOSR
    pub mod AWFOSR {
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

    /// BKSCD
    pub mod BKSCD {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (4 bits: 0b1111 << 12)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// SCDT
    pub mod SCDT {
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

/// channel watchdog filter data register
pub mod WDATR0 {

    /// WDATA
    pub mod WDATA {
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

/// channel data input register
pub mod DATINR0 {

    /// INDAT1
    pub mod INDAT1 {
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

    /// INDAT0
    pub mod INDAT0 {
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

/// channel y delay register
pub mod DLYR0 {

    /// PLSSKP
    pub mod PLSSKP {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (6 bits: 0x3f << 0)
        pub const mask: u32 = 0x3f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// channel configuration y register
pub mod CFGR11 {
    pub use super::CFGR10::CHEN;
    pub use super::CFGR10::CHINSEL;
    pub use super::CFGR10::CKABEN;
    pub use super::CFGR10::CKOUTDIV;
    pub use super::CFGR10::CKOUTSRC;
    pub use super::CFGR10::DATMPX;
    pub use super::CFGR10::DATPACK;
    pub use super::CFGR10::DFSDMEN;
    pub use super::CFGR10::SCDEN;
    pub use super::CFGR10::SITP;
    pub use super::CFGR10::SPICKSEL;
}

/// channel configuration y register
pub mod CFGR21 {
    pub use super::CFGR20::DTRBS;
    pub use super::CFGR20::OFFSET;
}

/// analog watchdog and short-circuit detector register
pub mod AWSCDR1 {
    pub use super::AWSCDR0::AWFORD;
    pub use super::AWSCDR0::AWFOSR;
    pub use super::AWSCDR0::BKSCD;
    pub use super::AWSCDR0::SCDT;
}

/// channel watchdog filter data register
pub mod WDATR1 {
    pub use super::WDATR0::WDATA;
}

/// channel data input register
pub mod DATINR1 {
    pub use super::DATINR0::INDAT0;
    pub use super::DATINR0::INDAT1;
}

/// channel y delay register
pub mod DLYR1 {
    pub use super::DLYR0::PLSSKP;
}

/// channel configuration y register
pub mod CFGR12 {
    pub use super::CFGR10::CHEN;
    pub use super::CFGR10::CHINSEL;
    pub use super::CFGR10::CKABEN;
    pub use super::CFGR10::CKOUTDIV;
    pub use super::CFGR10::CKOUTSRC;
    pub use super::CFGR10::DATMPX;
    pub use super::CFGR10::DATPACK;
    pub use super::CFGR10::DFSDMEN;
    pub use super::CFGR10::SCDEN;
    pub use super::CFGR10::SITP;
    pub use super::CFGR10::SPICKSEL;
}

/// channel configuration y register
pub mod CFGR22 {
    pub use super::CFGR20::DTRBS;
    pub use super::CFGR20::OFFSET;
}

/// analog watchdog and short-circuit detector register
pub mod AWSCDR2 {
    pub use super::AWSCDR0::AWFORD;
    pub use super::AWSCDR0::AWFOSR;
    pub use super::AWSCDR0::BKSCD;
    pub use super::AWSCDR0::SCDT;
}

/// channel watchdog filter data register
pub mod WDATR2 {
    pub use super::WDATR0::WDATA;
}

/// channel data input register
pub mod DATINR2 {
    pub use super::DATINR0::INDAT0;
    pub use super::DATINR0::INDAT1;
}

/// channel y delay register
pub mod DLYR2 {
    pub use super::DLYR0::PLSSKP;
}

/// channel configuration y register
pub mod CFGR13 {
    pub use super::CFGR10::CHEN;
    pub use super::CFGR10::CHINSEL;
    pub use super::CFGR10::CKABEN;
    pub use super::CFGR10::CKOUTDIV;
    pub use super::CFGR10::CKOUTSRC;
    pub use super::CFGR10::DATMPX;
    pub use super::CFGR10::DATPACK;
    pub use super::CFGR10::DFSDMEN;
    pub use super::CFGR10::SCDEN;
    pub use super::CFGR10::SITP;
    pub use super::CFGR10::SPICKSEL;
}

/// channel configuration y register
pub mod CFGR23 {
    pub use super::CFGR20::DTRBS;
    pub use super::CFGR20::OFFSET;
}

/// analog watchdog and short-circuit detector register
pub mod AWSCDR3 {
    pub use super::AWSCDR0::AWFORD;
    pub use super::AWSCDR0::AWFOSR;
    pub use super::AWSCDR0::BKSCD;
    pub use super::AWSCDR0::SCDT;
}

/// channel watchdog filter data register
pub mod WDATR3 {
    pub use super::WDATR0::WDATA;
}

/// channel data input register
pub mod DATINR3 {
    pub use super::DATINR0::INDAT0;
    pub use super::DATINR0::INDAT1;
}

/// channel y delay register
pub mod DLYR3 {
    pub use super::DLYR0::PLSSKP;
}

/// channel configuration y register
pub mod CFGR14 {
    pub use super::CFGR10::CHEN;
    pub use super::CFGR10::CHINSEL;
    pub use super::CFGR10::CKABEN;
    pub use super::CFGR10::CKOUTDIV;
    pub use super::CFGR10::CKOUTSRC;
    pub use super::CFGR10::DATMPX;
    pub use super::CFGR10::DATPACK;
    pub use super::CFGR10::DFSDMEN;
    pub use super::CFGR10::SCDEN;
    pub use super::CFGR10::SITP;
    pub use super::CFGR10::SPICKSEL;
}

/// channel configuration y register
pub mod CFGR24 {
    pub use super::CFGR20::DTRBS;
    pub use super::CFGR20::OFFSET;
}

/// analog watchdog and short-circuit detector register
pub mod AWSCDR4 {
    pub use super::AWSCDR0::AWFORD;
    pub use super::AWSCDR0::AWFOSR;
    pub use super::AWSCDR0::BKSCD;
    pub use super::AWSCDR0::SCDT;
}

/// channel watchdog filter data register
pub mod WDATR4 {
    pub use super::WDATR0::WDATA;
}

/// channel data input register
pub mod DATINR4 {
    pub use super::DATINR0::INDAT0;
    pub use super::DATINR0::INDAT1;
}

/// channel y delay register
pub mod DLYR4 {
    pub use super::DLYR0::PLSSKP;
}

/// channel configuration y register
pub mod CFGR15 {
    pub use super::CFGR10::CHEN;
    pub use super::CFGR10::CHINSEL;
    pub use super::CFGR10::CKABEN;
    pub use super::CFGR10::CKOUTDIV;
    pub use super::CFGR10::CKOUTSRC;
    pub use super::CFGR10::DATMPX;
    pub use super::CFGR10::DATPACK;
    pub use super::CFGR10::DFSDMEN;
    pub use super::CFGR10::SCDEN;
    pub use super::CFGR10::SITP;
    pub use super::CFGR10::SPICKSEL;
}

/// channel configuration y register
pub mod CFGR25 {
    pub use super::CFGR20::DTRBS;
    pub use super::CFGR20::OFFSET;
}

/// analog watchdog and short-circuit detector register
pub mod AWSCDR5 {
    pub use super::AWSCDR0::AWFORD;
    pub use super::AWSCDR0::AWFOSR;
    pub use super::AWSCDR0::BKSCD;
    pub use super::AWSCDR0::SCDT;
}

/// channel watchdog filter data register
pub mod WDATR5 {
    pub use super::WDATR0::WDATA;
}

/// channel data input register
pub mod DATINR5 {
    pub use super::DATINR0::INDAT0;
    pub use super::DATINR0::INDAT1;
}

/// channel y delay register
pub mod DLYR5 {
    pub use super::DLYR0::PLSSKP;
}

/// channel configuration y register
pub mod CFGR16 {
    pub use super::CFGR10::CHEN;
    pub use super::CFGR10::CHINSEL;
    pub use super::CFGR10::CKABEN;
    pub use super::CFGR10::CKOUTDIV;
    pub use super::CFGR10::CKOUTSRC;
    pub use super::CFGR10::DATMPX;
    pub use super::CFGR10::DATPACK;
    pub use super::CFGR10::DFSDMEN;
    pub use super::CFGR10::SCDEN;
    pub use super::CFGR10::SITP;
    pub use super::CFGR10::SPICKSEL;
}

/// channel configuration y register
pub mod CFGR26 {
    pub use super::CFGR20::DTRBS;
    pub use super::CFGR20::OFFSET;
}

/// analog watchdog and short-circuit detector register
pub mod AWSCDR6 {
    pub use super::AWSCDR0::AWFORD;
    pub use super::AWSCDR0::AWFOSR;
    pub use super::AWSCDR0::BKSCD;
    pub use super::AWSCDR0::SCDT;
}

/// channel watchdog filter data register
pub mod WDATR6 {
    pub use super::WDATR0::WDATA;
}

/// channel data input register
pub mod DATINR6 {
    pub use super::DATINR0::INDAT0;
    pub use super::DATINR0::INDAT1;
}

/// channel y delay register
pub mod DLYR6 {
    pub use super::DLYR0::PLSSKP;
}

/// channel configuration y register
pub mod CFGR17 {
    pub use super::CFGR10::CHEN;
    pub use super::CFGR10::CHINSEL;
    pub use super::CFGR10::CKABEN;
    pub use super::CFGR10::CKOUTDIV;
    pub use super::CFGR10::CKOUTSRC;
    pub use super::CFGR10::DATMPX;
    pub use super::CFGR10::DATPACK;
    pub use super::CFGR10::DFSDMEN;
    pub use super::CFGR10::SCDEN;
    pub use super::CFGR10::SITP;
    pub use super::CFGR10::SPICKSEL;
}

/// channel configuration y register
pub mod CFGR27 {
    pub use super::CFGR20::DTRBS;
    pub use super::CFGR20::OFFSET;
}

/// analog watchdog and short-circuit detector register
pub mod AWSCDR7 {
    pub use super::AWSCDR0::AWFORD;
    pub use super::AWSCDR0::AWFOSR;
    pub use super::AWSCDR0::BKSCD;
    pub use super::AWSCDR0::SCDT;
}

/// channel watchdog filter data register
pub mod WDATR7 {
    pub use super::WDATR0::WDATA;
}

/// channel data input register
pub mod DATINR7 {
    pub use super::DATINR0::INDAT0;
    pub use super::DATINR0::INDAT1;
}

/// channel y delay register
pub mod DLYR7 {
    pub use super::DLYR0::PLSSKP;
}

/// control register 1
pub mod CR10 {

    /// Analog watchdog fast mode select
    pub mod AWFSEL {
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

            /// 0b0: Analog watchdog on data output value (after the digital filter). The comparison is done after offset correction and shift
            pub const Output: u32 = 0b0;

            /// 0b1: Analog watchdog on channel transceivers value (after watchdog filter)
            pub const Transceiver: u32 = 0b1;
        }
    }

    /// Fast conversion mode selection for regular conversions
    pub mod FAST {
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

            /// 0b0: Fast conversion mode disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Fast conversion mode enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Regular channel selection
    pub mod RCH {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (3 bits: 0b111 << 24)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: Channel 0 is selected as regular channel
            pub const Channel0: u32 = 0b000;

            /// 0b001: Channel 1 is selected as regular channel
            pub const Channel1: u32 = 0b001;

            /// 0b010: Channel 2 is selected as regular channel
            pub const Channel2: u32 = 0b010;

            /// 0b011: Channel 3 is selected as regular channel
            pub const Channel3: u32 = 0b011;

            /// 0b100: Channel 4 is selected as regular channel
            pub const Channel4: u32 = 0b100;

            /// 0b101: Channel 5 is selected as regular channel
            pub const Channel5: u32 = 0b101;

            /// 0b110: Channel 6 is selected as regular channel
            pub const Channel6: u32 = 0b110;

            /// 0b111: Channel 7 is selected as regular channel
            pub const Channel7: u32 = 0b111;
        }
    }

    /// DMA channel enabled to read data for the regular conversion
    pub mod RDMAEN {
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

            /// 0b0: The DMA channel is not enabled to read regular data
            pub const Disabled: u32 = 0b0;

            /// 0b1: The DMA channel is enabled to read regular data
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Launch regular conversion synchronously with DFSDM0
    pub mod RSYNC {
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

            /// 0b0: Do not launch a regular conversion synchronously with DFSDM_FLT0
            pub const NoLaunch: u32 = 0b0;

            /// 0b1: Launch a regular conversion in this DFSDM_FLTx at the very moment when a regular conversion is launched in DFSDM_FLT0
            pub const Launch: u32 = 0b1;
        }
    }

    /// Continuous mode selection for regular conversions
    pub mod RCONT {
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

            /// 0b0: The regular channel is converted just once for each conversion request
            pub const Once: u32 = 0b0;

            /// 0b1: The regular channel is converted repeatedly after each conversion request
            pub const Continuous: u32 = 0b1;
        }
    }

    /// Software start of a conversion on the regular channel
    pub mod RSWSTART {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values
        pub mod W {

            /// 0b1: Writing ‘1’ makes a request to start a conversion on the regular channel and causes RCIP to become ‘1’. If RCIP=1 already, writing to RSWSTART has no effect. Writing ‘1’ has no effect if RSYNC=1
            pub const Start: u32 = 0b1;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Trigger enable and trigger edge selection for injected conversions
    pub mod JEXTEN {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (2 bits: 0b11 << 13)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Trigger detection is disabled
            pub const Disabled: u32 = 0b00;

            /// 0b01: Each rising edge on the selected trigger makes a request to launch an injected conversion
            pub const RisingEdge: u32 = 0b01;

            /// 0b10: Each falling edge on the selected trigger makes a request to launch an injected conversion
            pub const FallingEdge: u32 = 0b10;

            /// 0b11: Both rising edges and falling edges on the selected trigger make requests to launch injected conversions
            pub const BothEdges: u32 = 0b11;
        }
    }

    /// Trigger signal selection for launching injected conversions
    pub mod JEXTSEL {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (3 bits: 0b111 << 8)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// DMA channel enabled to read data for the injected channel group
    pub mod JDMAEN {
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

            /// 0b0: The DMA channel is not enabled to read injected data
            pub const Disabled: u32 = 0b0;

            /// 0b1: The DMA channel is enabled to read injected data
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Scanning conversion mode for injected conversions
    pub mod JSCAN {
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

            /// 0b0: One channel conversion is performed from the injected channel group and next the selected channel from this group is selected
            pub const Single: u32 = 0b0;

            /// 0b1: The series of conversions for the injected group channels is executed, starting over with the lowest selected channel
            pub const Series: u32 = 0b1;
        }
    }

    /// Launch an injected conversion synchronously with the DFSDM0 JSWSTART trigger
    pub mod JSYNC {
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

            /// 0b0: Do not launch an injected conversion synchronously with DFSDM_FLT0
            pub const Disabled: u32 = 0b0;

            /// 0b1: Launch an injected conversion in this DFSDM_FLTx at the very moment when an injected conversion is launched in DFSDM_FLT0 by its JSWSTART trigger
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Start a conversion of the injected group of channels
    pub mod JSWSTART {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values
        pub mod W {

            /// 0b1: Writing ‘1’ makes a request to convert the channels in the injected conversion group, causing JCIP to become ‘1’ at the same time. If JCIP=1 already, then writing to JSWSTART has no effect. Writing ‘1’ has no effect if JSYNC=1
            pub const Start: u32 = 0b1;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// DFSDM enable
    pub mod DFEN {
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

            /// 0b0: DFSDM_FLTx is disabled. All conversions of given DFSDM_FLTx are stopped immediately and all DFSDM_FLTx functions are stopped
            pub const Disabled: u32 = 0b0;

            /// 0b1: DFSDM_FLTx is enabled. If DFSDM_FLTx is enabled, then DFSDM_FLTx starts operating according to its setting
            pub const Enabled: u32 = 0b1;
        }
    }
}

/// control register 2
pub mod CR20 {

    /// Analog watchdog channel selection
    pub mod AWDCH {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (8 bits: 0xff << 16)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00000000: Analog watchdog is disabled on channel y
            pub const Disabled: u32 = 0b00000000;

            /// 0b00000001: Analog watchdog is enabled on channel y
            pub const Enabled: u32 = 0b00000001;
        }
    }

    /// Extremes detector channel selection
    pub mod EXCH {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (8 bits: 0xff << 8)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00000000: Extremes detector does not accept data from channel y
            pub const Disabled: u32 = 0b00000000;

            /// 0b00000001: Extremes detector accepts data from channel y
            pub const Enabled: u32 = 0b00000001;
        }
    }

    /// Clock absence interrupt enable
    pub mod CKABIE {
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

            /// 0b0: Detection of channel input clock absence interrupt is disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Detection of channel input clock absence interrupt is enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Short-circuit detector interrupt enable
    pub mod SCDIE {
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

            /// 0b0: Short-circuit detector interrupt is disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Short-circuit detector interrupt is enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Analog watchdog interrupt enable
    pub mod AWDIE {
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

            /// 0b0: Analog watchdog interrupt is disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Analog watchdog interrupt is enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Regular data overrun interrupt enable
    pub mod ROVRIE {
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

            /// 0b0: Regular data overrun interrupt is disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Regular data overrun interrupt is enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Injected data overrun interrupt enable
    pub mod JOVRIE {
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

            /// 0b0: Injected data overrun interrupt is disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Injected data overrun interrupt is enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Regular end of conversion interrupt enable
    pub mod REOCIE {
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

            /// 0b0: Regular end of conversion interrupt is disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Regular end of conversion interrupt is enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Injected end of conversion interrupt enable
    pub mod JEOCIE {
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

            /// 0b0: Injected end of conversion interrupt is disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Injected end of conversion interrupt is enabled
            pub const Enabled: u32 = 0b1;
        }
    }
}

/// interrupt and status register
pub mod ISR0 {

    /// short-circuit detector flag
    pub mod SCDF {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (8 bits: 0xff << 24)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values
        pub mod R {

            /// 0b00000000: No short-circuit detector event occurred on channel y
            pub const Clear: u32 = 0b00000000;

            /// 0b00000001: The short-circuit detector counter reaches, on channel y, the value programmed in the DFSDM_CHyAWSCDR registers
            pub const Set: u32 = 0b00000001;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Clock absence flag
    pub mod CKABF {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (8 bits: 0xff << 16)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values
        pub mod R {

            /// 0b00000000: Clock signal on channel y is present.
            pub const Clear: u32 = 0b00000000;

            /// 0b00000001: Clock signal on channel y is not present
            pub const Set: u32 = 0b00000001;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Regular conversion in progress status
    pub mod RCIP {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: No request to convert the regular channel has been issued
            pub const NotInProgress: u32 = 0b0;

            /// 0b1: The conversion of the regular channel is in progress or a request for a regular conversion is pending
            pub const InProgress: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Injected conversion in progress status
    pub mod JCIP {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: No request to convert the injected channel group (neither by software nor by trigger) has been issued
            pub const NotInProgress: u32 = 0b0;

            /// 0b1: The conversion of the injected channel group is in progress or a request for a injected conversion is pending, due either to ‘1’ being written to JSWSTART or to a trigger detection
            pub const InProgress: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Analog watchdog
    pub mod AWDF {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: No Analog watchdog event occurred
            pub const Clear: u32 = 0b0;

            /// 0b1: The analog watchdog block detected voltage which crosses the value programmed in the DFSDM_FLTxAWLTR or DFSDM_FLTxAWHTR registers
            pub const Set: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Regular conversion overrun flag
    pub mod ROVRF {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: No regular conversion overrun has occurred
            pub const Clear: u32 = 0b0;

            /// 0b1: A regular conversion overrun has occurred, which means that a regular conversion finished while REOCF was already ‘1’. RDATAR is not affected by overruns
            pub const Set: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Injected conversion overrun flag
    pub mod JOVRF {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: No injected conversion overrun has occurred
            pub const Clear: u32 = 0b0;

            /// 0b1: An injected conversion overrun has occurred, which means that an injected conversion finished while JEOCF was already ‘1’. JDATAR is not affected by overruns
            pub const Set: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// End of regular conversion flag
    pub mod REOCF {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: No regular conversion has completed
            pub const Clear: u32 = 0b0;

            /// 0b1: A regular conversion has completed and its data may be read
            pub const Set: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// End of injected conversion flag
    pub mod JEOCF {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: No injected conversion has completed
            pub const Clear: u32 = 0b0;

            /// 0b1: An injected conversion has completed and its data may be read
            pub const Set: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// interrupt flag clear register
pub mod ICR0 {

    /// Clear the short-circuit detector flag
    pub mod CLRSCDF {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (8 bits: 0xff << 24)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Clear the clock absence flag
    pub mod CLRCKABF {
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

    /// Clear the regular conversion overrun flag
    pub mod CLRROVRF {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values
        pub mod W {

            /// 0b1: Writing ‘1’ clears the ROVRF bit in the DFSDM_FLTxISR register
            pub const Clear: u32 = 0b1;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Clear the injected conversion overrun flag
    pub mod CLRJOVRF {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values
        pub mod W {

            /// 0b1: Writing ‘1’ clears the JOVRF bit in the DFSDM_FLTxISR register
            pub const Clear: u32 = 0b1;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// injected channel group selection register
pub mod JCHGR0 {

    /// Injected channel group selection
    pub mod JCHG {
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

/// filter control register
pub mod FCR0 {

    /// Sinc filter order
    pub mod FORD {
        /// Offset (29 bits)
        pub const offset: u32 = 29;
        /// Mask (3 bits: 0b111 << 29)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: FastSinc filter type
            pub const FastSinc: u32 = 0b000;

            /// 0b001: Sinc1 filter type
            pub const Sinc1: u32 = 0b001;

            /// 0b010: Sinc2 filter type
            pub const Sinc2: u32 = 0b010;

            /// 0b011: Sinc3 filter type
            pub const Sinc3: u32 = 0b011;

            /// 0b100: Sinc4 filter type
            pub const Sinc4: u32 = 0b100;

            /// 0b101: Sinc5 filter type
            pub const Sinc5: u32 = 0b101;
        }
    }

    /// Sinc filter oversampling ratio (decimation rate)
    pub mod FOSR {
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

    /// Integrator oversampling ratio (averaging length)
    pub mod IOSR {
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

/// data register for injected group
pub mod JDATAR0 {

    /// Injected group conversion data
    pub mod JDATA {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (24 bits: 0xffffff << 8)
        pub const mask: u32 = 0xffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Injected channel most recently converted
    pub mod JDATACH {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (3 bits: 0b111 << 0)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// data register for the regular channel
pub mod RDATAR0 {

    /// Regular channel conversion data
    pub mod RDATA {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (24 bits: 0xffffff << 8)
        pub const mask: u32 = 0xffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Regular channel pending data
    pub mod RPEND {
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

    /// Regular channel most recently converted
    pub mod RDATACH {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (3 bits: 0b111 << 0)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// analog watchdog high threshold register
pub mod AWHTR0 {

    /// Analog watchdog high threshold
    pub mod AWHT {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (24 bits: 0xffffff << 8)
        pub const mask: u32 = 0xffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Break signal assignment to analog watchdog high threshold event
    pub mod BKAWH {
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

/// analog watchdog low threshold register
pub mod AWLTR0 {

    /// Analog watchdog low threshold
    pub mod AWLT {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (24 bits: 0xffffff << 8)
        pub const mask: u32 = 0xffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Break signal assignment to analog watchdog low threshold event
    pub mod BKAWL {
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

/// analog watchdog status register
pub mod AWSR0 {

    /// Analog watchdog high threshold flag
    pub mod AWHTF {
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

    /// Analog watchdog low threshold flag
    pub mod AWLTF {
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

/// analog watchdog clear flag register
pub mod AWCFR0 {

    /// Clear the analog watchdog high threshold flag
    pub mod CLRAWHTF {
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

    /// Clear the analog watchdog low threshold flag
    pub mod CLRAWLTF {
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

/// Extremes detector maximum register
pub mod EXMAX0 {

    /// Extremes detector maximum value
    pub mod EXMAX {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (24 bits: 0xffffff << 8)
        pub const mask: u32 = 0xffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Extremes detector maximum data channel
    pub mod EXMAXCH {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (3 bits: 0b111 << 0)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Extremes detector minimum register
pub mod EXMIN0 {

    /// EXMIN
    pub mod EXMIN {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (24 bits: 0xffffff << 8)
        pub const mask: u32 = 0xffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Extremes detector minimum data channel
    pub mod EXMINCH {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (3 bits: 0b111 << 0)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// conversion timer register
pub mod CNVTIMR0 {

    /// 28-bit timer counting conversion time t = CNVCNT\[27:0\] / fDFSDM_CKIN
    pub mod CNVCNT {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (28 bits: 0xfffffff << 4)
        pub const mask: u32 = 0xfffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// control register 1
pub mod CR11 {
    pub use super::CR10::AWFSEL;
    pub use super::CR10::DFEN;
    pub use super::CR10::FAST;
    pub use super::CR10::JDMAEN;
    pub use super::CR10::JEXTEN;
    pub use super::CR10::JEXTSEL;
    pub use super::CR10::JSCAN;
    pub use super::CR10::JSWSTART;
    pub use super::CR10::JSYNC;
    pub use super::CR10::RCH;
    pub use super::CR10::RCONT;
    pub use super::CR10::RDMAEN;
    pub use super::CR10::RSWSTART;
    pub use super::CR10::RSYNC;
}

/// control register 2
pub mod CR21 {
    pub use super::CR20::AWDCH;
    pub use super::CR20::AWDIE;
    pub use super::CR20::CKABIE;
    pub use super::CR20::EXCH;
    pub use super::CR20::JEOCIE;
    pub use super::CR20::JOVRIE;
    pub use super::CR20::REOCIE;
    pub use super::CR20::ROVRIE;
    pub use super::CR20::SCDIE;
}

/// interrupt and status register
pub mod ISR1 {
    pub use super::ISR0::AWDF;
    pub use super::ISR0::CKABF;
    pub use super::ISR0::JCIP;
    pub use super::ISR0::JEOCF;
    pub use super::ISR0::JOVRF;
    pub use super::ISR0::RCIP;
    pub use super::ISR0::REOCF;
    pub use super::ISR0::ROVRF;
    pub use super::ISR0::SCDF;
}

/// interrupt flag clear register
pub mod ICR1 {
    pub use super::ICR0::CLRCKABF;
    pub use super::ICR0::CLRJOVRF;
    pub use super::ICR0::CLRROVRF;
    pub use super::ICR0::CLRSCDF;
}

/// injected channel group selection register
pub mod JCHGR1 {
    pub use super::JCHGR0::JCHG;
}

/// filter control register
pub mod FCR1 {
    pub use super::FCR0::FORD;
    pub use super::FCR0::FOSR;
    pub use super::FCR0::IOSR;
}

/// data register for injected group
pub mod JDATAR1 {
    pub use super::JDATAR0::JDATA;
    pub use super::JDATAR0::JDATACH;
}

/// data register for the regular channel
pub mod RDATAR1 {
    pub use super::RDATAR0::RDATA;
    pub use super::RDATAR0::RDATACH;
    pub use super::RDATAR0::RPEND;
}

/// analog watchdog high threshold register
pub mod AWHTR1 {
    pub use super::AWHTR0::AWHT;
    pub use super::AWHTR0::BKAWH;
}

/// analog watchdog low threshold register
pub mod AWLTR1 {
    pub use super::AWLTR0::AWLT;
    pub use super::AWLTR0::BKAWL;
}

/// analog watchdog status register
pub mod AWSR1 {
    pub use super::AWSR0::AWHTF;
    pub use super::AWSR0::AWLTF;
}

/// analog watchdog clear flag register
pub mod AWCFR1 {
    pub use super::AWCFR0::CLRAWHTF;
    pub use super::AWCFR0::CLRAWLTF;
}

/// Extremes detector maximum register
pub mod EXMAX1 {
    pub use super::EXMAX0::EXMAX;
    pub use super::EXMAX0::EXMAXCH;
}

/// Extremes detector minimum register
pub mod EXMIN1 {
    pub use super::EXMIN0::EXMIN;
    pub use super::EXMIN0::EXMINCH;
}

/// conversion timer register
pub mod CNVTIMR1 {
    pub use super::CNVTIMR0::CNVCNT;
}

/// control register 1
pub mod CR12 {
    pub use super::CR10::AWFSEL;
    pub use super::CR10::DFEN;
    pub use super::CR10::FAST;
    pub use super::CR10::JDMAEN;
    pub use super::CR10::JEXTEN;
    pub use super::CR10::JEXTSEL;
    pub use super::CR10::JSCAN;
    pub use super::CR10::JSWSTART;
    pub use super::CR10::JSYNC;
    pub use super::CR10::RCH;
    pub use super::CR10::RCONT;
    pub use super::CR10::RDMAEN;
    pub use super::CR10::RSWSTART;
    pub use super::CR10::RSYNC;
}

/// control register 2
pub mod CR22 {
    pub use super::CR20::AWDCH;
    pub use super::CR20::AWDIE;
    pub use super::CR20::CKABIE;
    pub use super::CR20::EXCH;
    pub use super::CR20::JEOCIE;
    pub use super::CR20::JOVRIE;
    pub use super::CR20::REOCIE;
    pub use super::CR20::ROVRIE;
    pub use super::CR20::SCDIE;
}

/// interrupt and status register
pub mod ISR2 {
    pub use super::ISR0::AWDF;
    pub use super::ISR0::CKABF;
    pub use super::ISR0::JCIP;
    pub use super::ISR0::JEOCF;
    pub use super::ISR0::JOVRF;
    pub use super::ISR0::RCIP;
    pub use super::ISR0::REOCF;
    pub use super::ISR0::ROVRF;
    pub use super::ISR0::SCDF;
}

/// interrupt flag clear register
pub mod ICR2 {
    pub use super::ICR0::CLRCKABF;
    pub use super::ICR0::CLRJOVRF;
    pub use super::ICR0::CLRROVRF;
    pub use super::ICR0::CLRSCDF;
}

/// injected channel group selection register
pub mod JCHGR2 {
    pub use super::JCHGR0::JCHG;
}

/// filter control register
pub mod FCR2 {
    pub use super::FCR0::FORD;
    pub use super::FCR0::FOSR;
    pub use super::FCR0::IOSR;
}

/// data register for injected group
pub mod JDATAR2 {
    pub use super::JDATAR0::JDATA;
    pub use super::JDATAR0::JDATACH;
}

/// data register for the regular channel
pub mod RDATAR2 {
    pub use super::RDATAR0::RDATA;
    pub use super::RDATAR0::RDATACH;
    pub use super::RDATAR0::RPEND;
}

/// analog watchdog high threshold register
pub mod AWHTR2 {
    pub use super::AWHTR0::AWHT;
    pub use super::AWHTR0::BKAWH;
}

/// analog watchdog low threshold register
pub mod AWLTR2 {
    pub use super::AWLTR0::AWLT;
    pub use super::AWLTR0::BKAWL;
}

/// analog watchdog status register
pub mod AWSR2 {
    pub use super::AWSR0::AWHTF;
    pub use super::AWSR0::AWLTF;
}

/// analog watchdog clear flag register
pub mod AWCFR2 {
    pub use super::AWCFR0::CLRAWHTF;
    pub use super::AWCFR0::CLRAWLTF;
}

/// Extremes detector maximum register
pub mod EXMAX2 {
    pub use super::EXMAX0::EXMAX;
    pub use super::EXMAX0::EXMAXCH;
}

/// Extremes detector minimum register
pub mod EXMIN2 {
    pub use super::EXMIN0::EXMIN;
    pub use super::EXMIN0::EXMINCH;
}

/// conversion timer register
pub mod CNVTIMR2 {
    pub use super::CNVTIMR0::CNVCNT;
}

/// control register 1
pub mod CR13 {
    pub use super::CR10::AWFSEL;
    pub use super::CR10::DFEN;
    pub use super::CR10::FAST;
    pub use super::CR10::JDMAEN;
    pub use super::CR10::JEXTEN;
    pub use super::CR10::JEXTSEL;
    pub use super::CR10::JSCAN;
    pub use super::CR10::JSWSTART;
    pub use super::CR10::JSYNC;
    pub use super::CR10::RCH;
    pub use super::CR10::RCONT;
    pub use super::CR10::RDMAEN;
    pub use super::CR10::RSWSTART;
    pub use super::CR10::RSYNC;
}

/// control register 2
pub mod CR23 {
    pub use super::CR20::AWDCH;
    pub use super::CR20::AWDIE;
    pub use super::CR20::CKABIE;
    pub use super::CR20::EXCH;
    pub use super::CR20::JEOCIE;
    pub use super::CR20::JOVRIE;
    pub use super::CR20::REOCIE;
    pub use super::CR20::ROVRIE;
    pub use super::CR20::SCDIE;
}

/// interrupt and status register
pub mod ISR3 {
    pub use super::ISR0::AWDF;
    pub use super::ISR0::CKABF;
    pub use super::ISR0::JCIP;
    pub use super::ISR0::JEOCF;
    pub use super::ISR0::JOVRF;
    pub use super::ISR0::RCIP;
    pub use super::ISR0::REOCF;
    pub use super::ISR0::ROVRF;
    pub use super::ISR0::SCDF;
}

/// interrupt flag clear register
pub mod ICR3 {
    pub use super::ICR0::CLRCKABF;
    pub use super::ICR0::CLRJOVRF;
    pub use super::ICR0::CLRROVRF;
    pub use super::ICR0::CLRSCDF;
}

/// injected channel group selection register
pub mod JCHGR3 {
    pub use super::JCHGR0::JCHG;
}

/// filter control register
pub mod FCR3 {
    pub use super::FCR0::FORD;
    pub use super::FCR0::FOSR;
    pub use super::FCR0::IOSR;
}

/// data register for injected group
pub mod JDATAR3 {
    pub use super::JDATAR0::JDATA;
    pub use super::JDATAR0::JDATACH;
}

/// data register for the regular channel
pub mod RDATAR3 {
    pub use super::RDATAR0::RDATA;
    pub use super::RDATAR0::RDATACH;
    pub use super::RDATAR0::RPEND;
}

/// analog watchdog high threshold register
pub mod AWHTR3 {
    pub use super::AWHTR0::AWHT;
    pub use super::AWHTR0::BKAWH;
}

/// analog watchdog low threshold register
pub mod AWLTR3 {
    pub use super::AWLTR0::AWLT;
    pub use super::AWLTR0::BKAWL;
}

/// analog watchdog status register
pub mod AWSR3 {
    pub use super::AWSR0::AWHTF;
    pub use super::AWSR0::AWLTF;
}

/// analog watchdog clear flag register
pub mod AWCFR3 {
    pub use super::AWCFR0::CLRAWHTF;
    pub use super::AWCFR0::CLRAWLTF;
}

/// Extremes detector maximum register
pub mod EXMAX3 {
    pub use super::EXMAX0::EXMAX;
    pub use super::EXMAX0::EXMAXCH;
}

/// Extremes detector minimum register
pub mod EXMIN3 {
    pub use super::EXMIN0::EXMIN;
    pub use super::EXMIN0::EXMINCH;
}

/// conversion timer register
pub mod CNVTIMR3 {
    pub use super::CNVTIMR0::CNVCNT;
}
#[repr(C)]
pub struct RegisterBlock {
    /// channel configuration y register
    pub CFGR10: RWRegister<u32>,

    /// channel configuration y register
    pub CFGR20: RWRegister<u32>,

    /// analog watchdog and short-circuit detector register
    pub AWSCDR0: RWRegister<u32>,

    /// channel watchdog filter data register
    pub WDATR0: RWRegister<u32>,

    /// channel data input register
    pub DATINR0: RWRegister<u32>,

    /// channel y delay register
    pub DLYR0: RWRegister<u32>,

    _reserved1: [u8; 8],

    /// channel configuration y register
    pub CFGR11: RWRegister<u32>,

    /// channel configuration y register
    pub CFGR21: RWRegister<u32>,

    /// analog watchdog and short-circuit detector register
    pub AWSCDR1: RWRegister<u32>,

    /// channel watchdog filter data register
    pub WDATR1: RWRegister<u32>,

    /// channel data input register
    pub DATINR1: RWRegister<u32>,

    /// channel y delay register
    pub DLYR1: RWRegister<u32>,

    _reserved2: [u8; 8],

    /// channel configuration y register
    pub CFGR12: RWRegister<u32>,

    /// channel configuration y register
    pub CFGR22: RWRegister<u32>,

    /// analog watchdog and short-circuit detector register
    pub AWSCDR2: RWRegister<u32>,

    /// channel watchdog filter data register
    pub WDATR2: RWRegister<u32>,

    /// channel data input register
    pub DATINR2: RWRegister<u32>,

    /// channel y delay register
    pub DLYR2: RWRegister<u32>,

    _reserved3: [u8; 8],

    /// channel configuration y register
    pub CFGR13: RWRegister<u32>,

    /// channel configuration y register
    pub CFGR23: RWRegister<u32>,

    /// analog watchdog and short-circuit detector register
    pub AWSCDR3: RWRegister<u32>,

    /// channel watchdog filter data register
    pub WDATR3: RWRegister<u32>,

    /// channel data input register
    pub DATINR3: RWRegister<u32>,

    /// channel y delay register
    pub DLYR3: RWRegister<u32>,

    _reserved4: [u8; 8],

    /// channel configuration y register
    pub CFGR14: RWRegister<u32>,

    /// channel configuration y register
    pub CFGR24: RWRegister<u32>,

    /// analog watchdog and short-circuit detector register
    pub AWSCDR4: RWRegister<u32>,

    /// channel watchdog filter data register
    pub WDATR4: RWRegister<u32>,

    /// channel data input register
    pub DATINR4: RWRegister<u32>,

    /// channel y delay register
    pub DLYR4: RWRegister<u32>,

    _reserved5: [u8; 8],

    /// channel configuration y register
    pub CFGR15: RWRegister<u32>,

    /// channel configuration y register
    pub CFGR25: RWRegister<u32>,

    /// analog watchdog and short-circuit detector register
    pub AWSCDR5: RWRegister<u32>,

    /// channel watchdog filter data register
    pub WDATR5: RWRegister<u32>,

    /// channel data input register
    pub DATINR5: RWRegister<u32>,

    /// channel y delay register
    pub DLYR5: RWRegister<u32>,

    _reserved6: [u8; 8],

    /// channel configuration y register
    pub CFGR16: RWRegister<u32>,

    /// channel configuration y register
    pub CFGR26: RWRegister<u32>,

    /// analog watchdog and short-circuit detector register
    pub AWSCDR6: RWRegister<u32>,

    /// channel watchdog filter data register
    pub WDATR6: RWRegister<u32>,

    /// channel data input register
    pub DATINR6: RWRegister<u32>,

    /// channel y delay register
    pub DLYR6: RWRegister<u32>,

    _reserved7: [u8; 8],

    /// channel configuration y register
    pub CFGR17: RWRegister<u32>,

    /// channel configuration y register
    pub CFGR27: RWRegister<u32>,

    /// analog watchdog and short-circuit detector register
    pub AWSCDR7: RWRegister<u32>,

    /// channel watchdog filter data register
    pub WDATR7: RWRegister<u32>,

    /// channel data input register
    pub DATINR7: RWRegister<u32>,

    /// channel y delay register
    pub DLYR7: RWRegister<u32>,

    _reserved8: [u8; 8],

    /// control register 1
    pub CR10: RWRegister<u32>,

    /// control register 2
    pub CR20: RWRegister<u32>,

    /// interrupt and status register
    pub ISR0: RORegister<u32>,

    /// interrupt flag clear register
    pub ICR0: RWRegister<u32>,

    /// injected channel group selection register
    pub JCHGR0: RWRegister<u32>,

    /// filter control register
    pub FCR0: RWRegister<u32>,

    /// data register for injected group
    pub JDATAR0: RORegister<u32>,

    /// data register for the regular channel
    pub RDATAR0: RORegister<u32>,

    /// analog watchdog high threshold register
    pub AWHTR0: RWRegister<u32>,

    /// analog watchdog low threshold register
    pub AWLTR0: RWRegister<u32>,

    /// analog watchdog status register
    pub AWSR0: RORegister<u32>,

    /// analog watchdog clear flag register
    pub AWCFR0: RWRegister<u32>,

    /// Extremes detector maximum register
    pub EXMAX0: RORegister<u32>,

    /// Extremes detector minimum register
    pub EXMIN0: RORegister<u32>,

    /// conversion timer register
    pub CNVTIMR0: RORegister<u32>,

    _reserved9: [u8; 68],

    /// control register 1
    pub CR11: RWRegister<u32>,

    /// control register 2
    pub CR21: RWRegister<u32>,

    /// interrupt and status register
    pub ISR1: RORegister<u32>,

    /// interrupt flag clear register
    pub ICR1: RWRegister<u32>,

    /// injected channel group selection register
    pub JCHGR1: RWRegister<u32>,

    /// filter control register
    pub FCR1: RWRegister<u32>,

    /// data register for injected group
    pub JDATAR1: RORegister<u32>,

    /// data register for the regular channel
    pub RDATAR1: RORegister<u32>,

    /// analog watchdog high threshold register
    pub AWHTR1: RWRegister<u32>,

    /// analog watchdog low threshold register
    pub AWLTR1: RWRegister<u32>,

    /// analog watchdog status register
    pub AWSR1: RORegister<u32>,

    /// analog watchdog clear flag register
    pub AWCFR1: RWRegister<u32>,

    /// Extremes detector maximum register
    pub EXMAX1: RORegister<u32>,

    /// Extremes detector minimum register
    pub EXMIN1: RORegister<u32>,

    /// conversion timer register
    pub CNVTIMR1: RORegister<u32>,

    _reserved10: [u8; 68],

    /// control register 1
    pub CR12: RWRegister<u32>,

    /// control register 2
    pub CR22: RWRegister<u32>,

    /// interrupt and status register
    pub ISR2: RORegister<u32>,

    /// interrupt flag clear register
    pub ICR2: RWRegister<u32>,

    /// injected channel group selection register
    pub JCHGR2: RWRegister<u32>,

    /// filter control register
    pub FCR2: RWRegister<u32>,

    /// data register for injected group
    pub JDATAR2: RORegister<u32>,

    /// data register for the regular channel
    pub RDATAR2: RORegister<u32>,

    /// analog watchdog high threshold register
    pub AWHTR2: RWRegister<u32>,

    /// analog watchdog low threshold register
    pub AWLTR2: RWRegister<u32>,

    /// analog watchdog status register
    pub AWSR2: RORegister<u32>,

    /// analog watchdog clear flag register
    pub AWCFR2: RWRegister<u32>,

    /// Extremes detector maximum register
    pub EXMAX2: RORegister<u32>,

    /// Extremes detector minimum register
    pub EXMIN2: RORegister<u32>,

    /// conversion timer register
    pub CNVTIMR2: RORegister<u32>,

    _reserved11: [u8; 68],

    /// control register 1
    pub CR13: RWRegister<u32>,

    /// control register 2
    pub CR23: RWRegister<u32>,

    /// interrupt and status register
    pub ISR3: RORegister<u32>,

    /// interrupt flag clear register
    pub ICR3: RWRegister<u32>,

    /// injected channel group selection register
    pub JCHGR3: RWRegister<u32>,

    /// filter control register
    pub FCR3: RWRegister<u32>,

    /// data register for injected group
    pub JDATAR3: RORegister<u32>,

    /// data register for the regular channel
    pub RDATAR3: RORegister<u32>,

    /// analog watchdog high threshold register
    pub AWHTR3: RWRegister<u32>,

    /// analog watchdog low threshold register
    pub AWLTR3: RWRegister<u32>,

    /// analog watchdog status register
    pub AWSR3: RORegister<u32>,

    /// analog watchdog clear flag register
    pub AWCFR3: RWRegister<u32>,

    /// Extremes detector maximum register
    pub EXMAX3: RORegister<u32>,

    /// Extremes detector minimum register
    pub EXMIN3: RORegister<u32>,

    /// conversion timer register
    pub CNVTIMR3: RORegister<u32>,
}
pub struct ResetValues {
    pub CFGR10: u32,
    pub CFGR20: u32,
    pub AWSCDR0: u32,
    pub WDATR0: u32,
    pub DATINR0: u32,
    pub DLYR0: u32,
    pub CFGR11: u32,
    pub CFGR21: u32,
    pub AWSCDR1: u32,
    pub WDATR1: u32,
    pub DATINR1: u32,
    pub DLYR1: u32,
    pub CFGR12: u32,
    pub CFGR22: u32,
    pub AWSCDR2: u32,
    pub WDATR2: u32,
    pub DATINR2: u32,
    pub DLYR2: u32,
    pub CFGR13: u32,
    pub CFGR23: u32,
    pub AWSCDR3: u32,
    pub WDATR3: u32,
    pub DATINR3: u32,
    pub DLYR3: u32,
    pub CFGR14: u32,
    pub CFGR24: u32,
    pub AWSCDR4: u32,
    pub WDATR4: u32,
    pub DATINR4: u32,
    pub DLYR4: u32,
    pub CFGR15: u32,
    pub CFGR25: u32,
    pub AWSCDR5: u32,
    pub WDATR5: u32,
    pub DATINR5: u32,
    pub DLYR5: u32,
    pub CFGR16: u32,
    pub CFGR26: u32,
    pub AWSCDR6: u32,
    pub WDATR6: u32,
    pub DATINR6: u32,
    pub DLYR6: u32,
    pub CFGR17: u32,
    pub CFGR27: u32,
    pub AWSCDR7: u32,
    pub WDATR7: u32,
    pub DATINR7: u32,
    pub DLYR7: u32,
    pub CR10: u32,
    pub CR20: u32,
    pub ISR0: u32,
    pub ICR0: u32,
    pub JCHGR0: u32,
    pub FCR0: u32,
    pub JDATAR0: u32,
    pub RDATAR0: u32,
    pub AWHTR0: u32,
    pub AWLTR0: u32,
    pub AWSR0: u32,
    pub AWCFR0: u32,
    pub EXMAX0: u32,
    pub EXMIN0: u32,
    pub CNVTIMR0: u32,
    pub CR11: u32,
    pub CR21: u32,
    pub ISR1: u32,
    pub ICR1: u32,
    pub JCHGR1: u32,
    pub FCR1: u32,
    pub JDATAR1: u32,
    pub RDATAR1: u32,
    pub AWHTR1: u32,
    pub AWLTR1: u32,
    pub AWSR1: u32,
    pub AWCFR1: u32,
    pub EXMAX1: u32,
    pub EXMIN1: u32,
    pub CNVTIMR1: u32,
    pub CR12: u32,
    pub CR22: u32,
    pub ISR2: u32,
    pub ICR2: u32,
    pub JCHGR2: u32,
    pub FCR2: u32,
    pub JDATAR2: u32,
    pub RDATAR2: u32,
    pub AWHTR2: u32,
    pub AWLTR2: u32,
    pub AWSR2: u32,
    pub AWCFR2: u32,
    pub EXMAX2: u32,
    pub EXMIN2: u32,
    pub CNVTIMR2: u32,
    pub CR13: u32,
    pub CR23: u32,
    pub ISR3: u32,
    pub ICR3: u32,
    pub JCHGR3: u32,
    pub FCR3: u32,
    pub JDATAR3: u32,
    pub RDATAR3: u32,
    pub AWHTR3: u32,
    pub AWLTR3: u32,
    pub AWSR3: u32,
    pub AWCFR3: u32,
    pub EXMAX3: u32,
    pub EXMIN3: u32,
    pub CNVTIMR3: u32,
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

/// Access functions for the DFSDM1 peripheral instance
pub mod DFSDM1 {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x40016000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in DFSDM1
    pub const reset: ResetValues = ResetValues {
        CFGR10: 0x00000000,
        CFGR20: 0x00000000,
        AWSCDR0: 0x00000000,
        WDATR0: 0x00000000,
        DATINR0: 0x00000000,
        DLYR0: 0x00000000,
        CFGR11: 0x00000000,
        CFGR21: 0x00000000,
        AWSCDR1: 0x00000000,
        WDATR1: 0x00000000,
        DATINR1: 0x00000000,
        DLYR1: 0x00000000,
        CFGR12: 0x00000000,
        CFGR22: 0x00000000,
        AWSCDR2: 0x00000000,
        WDATR2: 0x00000000,
        DATINR2: 0x00000000,
        DLYR2: 0x00000000,
        CFGR13: 0x00000000,
        CFGR23: 0x00000000,
        AWSCDR3: 0x00000000,
        WDATR3: 0x00000000,
        DATINR3: 0x00000000,
        DLYR3: 0x00000000,
        CFGR14: 0x00000000,
        CFGR24: 0x00000000,
        AWSCDR4: 0x00000000,
        WDATR4: 0x00000000,
        DATINR4: 0x00000000,
        DLYR4: 0x00000000,
        CFGR15: 0x00000000,
        CFGR25: 0x00000000,
        AWSCDR5: 0x00000000,
        WDATR5: 0x00000000,
        DATINR5: 0x00000000,
        DLYR5: 0x00000000,
        CFGR16: 0x00000000,
        CFGR26: 0x00000000,
        AWSCDR6: 0x00000000,
        WDATR6: 0x00000000,
        DATINR6: 0x00000000,
        DLYR6: 0x00000000,
        CFGR17: 0x00000000,
        CFGR27: 0x00000000,
        AWSCDR7: 0x00000000,
        WDATR7: 0x00000000,
        DATINR7: 0x00000000,
        DLYR7: 0x00000000,
        CR10: 0x00000000,
        CR20: 0x00000000,
        ISR0: 0x00FF0000,
        ICR0: 0x00000000,
        JCHGR0: 0x00000001,
        FCR0: 0x00000000,
        JDATAR0: 0x00000000,
        RDATAR0: 0x00000000,
        AWHTR0: 0x00000000,
        AWLTR0: 0x00000000,
        AWSR0: 0x00000000,
        AWCFR0: 0x00000000,
        EXMAX0: 0x80000000,
        EXMIN0: 0x7FFFFF00,
        CNVTIMR0: 0x00000000,
        CR11: 0x00000000,
        CR21: 0x00000000,
        ISR1: 0x00FF0000,
        ICR1: 0x00000000,
        JCHGR1: 0x00000001,
        FCR1: 0x00000000,
        JDATAR1: 0x00000000,
        RDATAR1: 0x00000000,
        AWHTR1: 0x00000000,
        AWLTR1: 0x00000000,
        AWSR1: 0x00000000,
        AWCFR1: 0x00000000,
        EXMAX1: 0x80000000,
        EXMIN1: 0x7FFFFF00,
        CNVTIMR1: 0x00000000,
        CR12: 0x00000000,
        CR22: 0x00000000,
        ISR2: 0x00FF0000,
        ICR2: 0x00000000,
        JCHGR2: 0x00000001,
        FCR2: 0x00000000,
        JDATAR2: 0x00000000,
        RDATAR2: 0x00000000,
        AWHTR2: 0x00000000,
        AWLTR2: 0x00000000,
        AWSR2: 0x00000000,
        AWCFR2: 0x00000000,
        EXMAX2: 0x80000000,
        EXMIN2: 0x7FFFFF00,
        CNVTIMR2: 0x00000000,
        CR13: 0x00000000,
        CR23: 0x00000000,
        ISR3: 0x00FF0000,
        ICR3: 0x00000000,
        JCHGR3: 0x00000001,
        FCR3: 0x00000000,
        JDATAR3: 0x00000000,
        RDATAR3: 0x00000000,
        AWHTR3: 0x00000000,
        AWLTR3: 0x00000000,
        AWSR3: 0x00000000,
        AWCFR3: 0x00000000,
        EXMAX3: 0x80000000,
        EXMIN3: 0x7FFFFF00,
        CNVTIMR3: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut DFSDM1_TAKEN: bool = false;

    /// Safe access to DFSDM1
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
            if DFSDM1_TAKEN {
                None
            } else {
                DFSDM1_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to DFSDM1
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if DFSDM1_TAKEN && inst.addr == INSTANCE.addr {
                DFSDM1_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal DFSDM1
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        DFSDM1_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to DFSDM1
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const DFSDM1: *const RegisterBlock = 0x40016000 as *const _;
