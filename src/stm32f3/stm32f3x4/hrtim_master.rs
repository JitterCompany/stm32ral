#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! High Resolution Timer: Master Timers

use crate::{RORegister, RWRegister, WORegister};
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// Master Timer Control Register
pub mod MCR {

    /// Burst DMA Update
    pub mod BRSTDMA {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (2 bits: 0b11 << 30)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Update done independently from the DMA burst transfer completion
            pub const Independent: u32 = 0b00;

            /// 0b01: Update done when the DMA burst transfer is completed
            pub const Completion: u32 = 0b01;

            /// 0b10: Update done on master timer roll-over following a DMA burst transfer completion
            pub const Rollover: u32 = 0b10;
        }
    }

    /// Master Timer Repetition update
    pub mod MREPU {
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

            /// 0b0: Update on repetition disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Update on repetition enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Preload enable
    pub mod PREEN {
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

            /// 0b0: Preload disabled: the write access is directly done into the active register
            pub const Disabled: u32 = 0b0;

            /// 0b1: Preload enabled: the write access is done into the preload register
            pub const Enabled: u32 = 0b1;
        }
    }

    /// AC Synchronization
    pub mod DACSYNC {
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

            /// 0b00: No DAC trigger generated
            pub const Disabled: u32 = 0b00;

            /// 0b01: Trigger generated on DACSync1
            pub const DACSync1: u32 = 0b01;

            /// 0b10: Trigger generated on DACSync2
            pub const DACSync2: u32 = 0b10;

            /// 0b11: Trigger generated on DACSync3
            pub const DACSync3: u32 = 0b11;
        }
    }

    /// Timer E counter enable
    pub mod TECEN {
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

            /// 0b0: Timer counter disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Timer counter enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Timer D counter enable
    pub mod TDCEN {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (1 bit: 1 << 20)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TECEN::RW;
    }

    /// Timer C counter enable
    pub mod TCCEN {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (1 bit: 1 << 19)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TECEN::RW;
    }

    /// Timer B counter enable
    pub mod TBCEN {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (1 bit: 1 << 18)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TECEN::RW;
    }

    /// Timer A counter enable
    pub mod TACEN {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TECEN::RW;
    }

    /// Master Counter enable
    pub mod MCEN {
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

            /// 0b0: Master timer counter disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Master timer counter enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Synchronization source
    pub mod SYNCSRC {
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

            /// 0b00: Master timer Start
            pub const MasterStart: u32 = 0b00;

            /// 0b01: Master timer Compare 1 event
            pub const MasterCompare1: u32 = 0b01;

            /// 0b10: Timer A start/reset
            pub const TimerAStart: u32 = 0b10;

            /// 0b11: Timer A Compare 1 event
            pub const TimerACompare1: u32 = 0b11;
        }
    }

    /// Synchronization output
    pub mod SYNCOUT {
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

            /// 0b00: Disabled
            pub const Disabled: u32 = 0b00;

            /// 0b10: Positive pulse on SCOUT output (16x f_HRTIM clock cycles)
            pub const PositivePulse: u32 = 0b10;

            /// 0b11: Negative pulse on SCOUT output (16x f_HRTIM clock cycles)
            pub const NegativePulse: u32 = 0b11;
        }
    }

    /// Synchronization Starts Master
    pub mod SYNCSTRTM {
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

            /// 0b0: No effect on the master timer
            pub const Disabled: u32 = 0b0;

            /// 0b1: A synchroniation input event starts the master timer
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Synchronization Resets Master
    pub mod SYNCRSTM {
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

            /// 0b0: No effect on the master timer
            pub const Disabled: u32 = 0b0;

            /// 0b1: A synchroniation input event resets the master timer
            pub const Enabled: u32 = 0b1;
        }
    }

    /// ynchronization input
    pub mod SYNCIN {
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

            /// 0b00: Disabled. HRTIM is not synchronized and runs in standalone mode
            pub const Disabled: u32 = 0b00;

            /// 0b10: Internal event: the HRTIM is synchronized with the on-chip timer
            pub const Internal: u32 = 0b10;

            /// 0b11: External event: a positive pulse on HRTIM_SCIN input triggers the HRTIM
            pub const External: u32 = 0b11;
        }
    }

    /// Half mode enable
    pub mod HALF {
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

            /// 0b0: Half mode disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Half mode enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Master Re-triggerable mode
    pub mod RETRIG {
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

            /// 0b0: The timer is not re-triggerable: a counter reset can be done only if the counter is stopped
            pub const Disabled: u32 = 0b0;

            /// 0b1: The timer is retriggerable: a counter reset is done whatever the counter state
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Master Continuous mode
    pub mod CONT {
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

            /// 0b0: The timer operates in single-shot mode and stops when it reaches the MPER value
            pub const SingleShot: u32 = 0b0;

            /// 0b1: The timer operates in continuous (free-running) mode and rolls over to zero when it reaches the MPER value
            pub const Continuous: u32 = 0b1;
        }
    }

    /// HRTIM Master Clock prescaler
    pub mod CKPSC {
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

/// Master Timer Interrupt Status Register
pub mod MISR {

    /// Master Update Interrupt Flag
    pub mod MUPD {
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

            /// 0b0: No master update interrupt occurred
            pub const NoEvent: u32 = 0b0;

            /// 0b1: Master update interrupt occurred
            pub const Event: u32 = 0b1;
        }
    }

    /// Sync Input Interrupt Flag
    pub mod SYNC {
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

            /// 0b0: No sync input interrupt occurred
            pub const NoEvent: u32 = 0b0;

            /// 0b1: Sync input interrupt occurred
            pub const Event: u32 = 0b1;
        }
    }

    /// Master Repetition Interrupt Flag
    pub mod MREP {
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

            /// 0b0: No master repetition interrupt occurred
            pub const NoEvent: u32 = 0b0;

            /// 0b1: Master repetition interrupt occurred
            pub const Event: u32 = 0b1;
        }
    }

    /// Master Compare 4 Interrupt Flag
    pub mod MCMP4 {
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

            /// 0b0: No master compare interrupt occurred
            pub const NoEvent: u32 = 0b0;

            /// 0b1: Master compare interrupt occurred
            pub const Event: u32 = 0b1;
        }
    }

    /// Master Compare 3 Interrupt Flag
    pub mod MCMP3 {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MCMP4::RW;
    }

    /// Master Compare 2 Interrupt Flag
    pub mod MCMP2 {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MCMP4::RW;
    }

    /// Master Compare 1 Interrupt Flag
    pub mod MCMP1 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MCMP4::RW;
    }
}

/// Master Timer Interrupt Clear Register
pub mod MICR {

    /// Master update Interrupt flag clear
    pub mod MUPDC {
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

            /// 0b1: Clears flag in MISR register
            pub const Clear: u32 = 0b1;
        }
    }

    /// Sync Input Interrupt flag clear
    pub mod SYNCC {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MUPDC::RW;
    }

    /// Repetition Interrupt flag clear
    pub mod MREPC {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MUPDC::RW;
    }

    /// Master Compare 4 Interrupt flag clear
    pub mod MCMP4C {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MUPDC::RW;
    }

    /// Master Compare 3 Interrupt flag clear
    pub mod MCMP3C {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MUPDC::RW;
    }

    /// Master Compare 2 Interrupt flag clear
    pub mod MCMP2C {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MUPDC::RW;
    }

    /// Master Compare 1 Interrupt flag clear
    pub mod MCMP1C {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MUPDC::RW;
    }
}

/// MDIER4
pub mod MDIER {

    /// MUPDDE
    pub mod MUPDDE {
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

            /// 0b0: DMA request disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: DMA request enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// SYNCDE
    pub mod SYNCDE {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (1 bit: 1 << 21)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MUPDDE::RW;
    }

    /// MREPDE
    pub mod MREPDE {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (1 bit: 1 << 20)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MUPDDE::RW;
    }

    /// MCMP4DE
    pub mod MCMP4DE {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (1 bit: 1 << 19)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MUPDDE::RW;
    }

    /// MCMP3DE
    pub mod MCMP3DE {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (1 bit: 1 << 18)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MUPDDE::RW;
    }

    /// MCMP2DE
    pub mod MCMP2DE {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MUPDDE::RW;
    }

    /// MCMP1DE
    pub mod MCMP1DE {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MUPDDE::RW;
    }

    /// MUPDIE
    pub mod MUPDIE {
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

            /// 0b0: Interrupt disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Interrupt enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// SYNCIE
    pub mod SYNCIE {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MUPDIE::RW;
    }

    /// MREPIE
    pub mod MREPIE {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MUPDIE::RW;
    }

    /// MCMP4IE
    pub mod MCMP4IE {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MUPDIE::RW;
    }

    /// MCMP3IE
    pub mod MCMP3IE {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MUPDIE::RW;
    }

    /// MCMP2IE
    pub mod MCMP2IE {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MUPDIE::RW;
    }

    /// MCMP1IE
    pub mod MCMP1IE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MUPDIE::RW;
    }
}

/// Master Timer Counter Register
pub mod MCNTR {

    /// Counter value
    pub mod MCNT {
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

/// Master Timer Period Register
pub mod MPER {

    /// Master Timer Period value
    pub mod MPER {
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

/// Master Timer Repetition Register
pub mod MREP {

    /// Master Timer Repetition counter value
    pub mod MREP {
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

/// Master Timer Compare 1 Register
pub mod MCMP1R {

    /// Master Timer Compare 1 value
    pub mod MCMP1 {
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

/// Master Timer Compare 2 Register
pub mod MCMP2R {

    /// Master Timer Compare 2 value
    pub mod MCMP2 {
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

/// Master Timer Compare 3 Register
pub mod MCMP3R {

    /// Master Timer Compare 3 value
    pub mod MCMP3 {
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

/// Master Timer Compare 4 Register
pub mod MCMP4R {

    /// Master Timer Compare 4 value
    pub mod MCMP4 {
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
    /// Master Timer Control Register
    pub MCR: RWRegister<u32>,

    /// Master Timer Interrupt Status Register
    pub MISR: RORegister<u32>,

    /// Master Timer Interrupt Clear Register
    pub MICR: WORegister<u32>,

    /// MDIER4
    pub MDIER: RWRegister<u32>,

    /// Master Timer Counter Register
    pub MCNTR: RWRegister<u32>,

    /// Master Timer Period Register
    pub MPER: RWRegister<u32>,

    /// Master Timer Repetition Register
    pub MREP: RWRegister<u32>,

    /// Master Timer Compare 1 Register
    pub MCMP1R: RWRegister<u32>,

    _reserved1: [u32; 1],

    /// Master Timer Compare 2 Register
    pub MCMP2R: RWRegister<u32>,

    /// Master Timer Compare 3 Register
    pub MCMP3R: RWRegister<u32>,

    /// Master Timer Compare 4 Register
    pub MCMP4R: RWRegister<u32>,
}
pub struct ResetValues {
    pub MCR: u32,
    pub MISR: u32,
    pub MICR: u32,
    pub MDIER: u32,
    pub MCNTR: u32,
    pub MPER: u32,
    pub MREP: u32,
    pub MCMP1R: u32,
    pub MCMP2R: u32,
    pub MCMP3R: u32,
    pub MCMP4R: u32,
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

/// Access functions for the HRTIM_Master peripheral instance
pub mod HRTIM_Master {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x40017400,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in HRTIM_Master
    pub const reset: ResetValues = ResetValues {
        MCR: 0x00000000,
        MISR: 0x00000000,
        MICR: 0x00000000,
        MDIER: 0x00000000,
        MCNTR: 0x00000000,
        MPER: 0x0000FFFF,
        MREP: 0x00000000,
        MCMP1R: 0x00000000,
        MCMP2R: 0x00000000,
        MCMP3R: 0x00000000,
        MCMP4R: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut HRTIM_Master_TAKEN: bool = false;

    /// Safe access to HRTIM_Master
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
            if HRTIM_Master_TAKEN {
                None
            } else {
                HRTIM_Master_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to HRTIM_Master
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if HRTIM_Master_TAKEN && inst.addr == INSTANCE.addr {
                HRTIM_Master_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal HRTIM_Master
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        HRTIM_Master_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to HRTIM_Master
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const HRTIM_Master: *const RegisterBlock = 0x40017400 as *const _;
