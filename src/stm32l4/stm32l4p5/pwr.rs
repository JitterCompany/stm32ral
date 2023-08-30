#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Power control

use crate::{RORegister, RWRegister, WORegister};
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// Power control register 1
pub mod CR1 {

    /// Low-power run
    pub mod LPR {
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

            /// 0b0: Main Mode
            pub const MainMode: u32 = 0b0;

            /// 0b1: Low Power Mode
            pub const LowPowerMode: u32 = 0b1;
        }
    }

    /// Voltage scaling range selection
    pub mod VOS {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (2 bits: 0b11 << 9)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b01: Range 1
            pub const Range1: u32 = 0b01;

            /// 0b10: Range 1
            pub const Range2: u32 = 0b10;
        }
    }

    /// Disable backup domain write protection
    pub mod DBP {
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

            /// 0b0: Access to RTC and Backup registers disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Access to RTC and Backup registers enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Low-power mode selection
    pub mod LPMS {
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

            /// 0b000: Stop 0 mode
            pub const Stop0: u32 = 0b000;

            /// 0b001: Stop 1 mode
            pub const Stop1: u32 = 0b001;

            /// 0b010: Stop 2 mode
            pub const Stop2: u32 = 0b010;

            /// 0b011: Standby mode
            pub const Standby: u32 = 0b011;

            /// 0b100: Shutdown mode
            pub const Shutdown: u32 = 0b100;
        }
    }

    /// SRAM3 retention in Stop 2 mode
    pub mod RRSTP {
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

            /// 0b0: SRAM3 is powered off in Stop 2 mode (SRAM3 content is lost)
            pub const Disabled: u32 = 0b0;

            /// 0b1: SRAM3 is powered in Stop 2 mode (RAM3 content is kept)
            pub const Enabled: u32 = 0b1;
        }
    }
}

/// Power control register 2
pub mod CR2 {

    /// VDDUSB USB supply valid
    pub mod USV {
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

            /// 0b0: VDDUSB is not present. Logical and electrical isolation is applied to ignore this supply
            pub const NotPresent: u32 = 0b0;

            /// 0b1: VDDUSB is valid
            pub const Valid: u32 = 0b1;
        }
    }

    /// VDDIO2 Independent I/Os supply valid
    pub mod IOSV {
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

            /// 0b0: VDDIO2 is not present. Logical and electrical isolation is applied to ignore this supply
            pub const NotPresent: u32 = 0b0;

            /// 0b1: VDDIO2 is valid
            pub const Valid: u32 = 0b1;
        }
    }

    /// Peripheral voltage monitoring 4 enable: VDDA vs. 2.2V
    pub mod PVME4 {
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

            /// 0b0: PVM4 (VDDA monitoring vs. 2.2V threshold) disable
            pub const Disabled: u32 = 0b0;

            /// 0b1: PVM4 (VDDA monitoring vs. 2.2V threshold) enable
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Peripheral voltage monitoring 3 enable: VDDA vs. 1.62V
    pub mod PVME3 {
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

            /// 0b0: PVM3 (VDDA monitoring vs. 1.62V threshold) disable
            pub const Disabled: u32 = 0b0;

            /// 0b1: PVM3 (VDDA monitoring vs. 1.62V threshold) enable
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Peripheral voltage monitoring 2 enable: VDDIO2 vs. 0.9V
    pub mod PVME2 {
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

            /// 0b0: PVM2 (VDDIO2 monitoring vs. 0.9V threshold) disable
            pub const Disabled: u32 = 0b0;

            /// 0b1: PVM2 (VDDIO2 monitoring vs. 0.9V threshold) enable
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Peripheral voltage monitoring 1 enable: VDDUSB vs. 1.2V
    pub mod PVME1 {
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

            /// 0b0: PVM2 (VDDUSB monitoring vs. 1.2V threshold) disable
            pub const Disabled: u32 = 0b0;

            /// 0b1: PVM2 (VDDUSB monitoring vs. 1.2V threshold) enable
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Power voltage detector level selection
    pub mod PLS {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (3 bits: 0b111 << 1)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: VPVD0 around 2.0 V
            pub const VPVD0: u32 = 0b000;

            /// 0b001: VPVD1 around 2.2 V
            pub const VPVD1: u32 = 0b001;

            /// 0b010: VPVD2 around 2.4 V
            pub const VPVD2: u32 = 0b010;

            /// 0b011: VPVD3 around 2.5 V
            pub const VPVD3: u32 = 0b011;

            /// 0b100: VPVD4 around 2.6 V
            pub const VPVD4: u32 = 0b100;

            /// 0b101: VPVD5 around 2.8 V
            pub const VPVD5: u32 = 0b101;

            /// 0b110: VPVD6 around 2.9 V
            pub const VPVD6: u32 = 0b110;

            /// 0b111: External input analog voltage PVD_IN (compared internally to VREFINT)
            pub const PVDIN: u32 = 0b111;
        }
    }

    /// Power voltage detector enable
    pub mod PVDE {
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

            /// 0b0: Power voltage detector disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Power voltage detector enabled
            pub const Enabled: u32 = 0b1;
        }
    }
}

/// Power control register 3
pub mod CR3 {

    /// Enable internal wakeup line
    pub mod EIWUL {
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

            /// 0b0: Internal wakeup line disable
            pub const Disabled: u32 = 0b0;

            /// 0b1: Internal wakeup line enable
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Apply pull-up and pull-down configuration
    pub mod APC {
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

            /// 0b0: Configurations are not applied
            pub const Disabled: u32 = 0b0;

            /// 0b1: When this bit is set, the I/O pull-up and pull-down configurations defined in the PWR_PUCRx and PWR_PDCRx registers are applied. When this bit is cleared, the PWR_PUCRx and PWR_PDCRx registers are not applied to the I/Os, instead the I/Os will be in floating mode during Standby or configured according GPIO controller GPIOx_PUPDR register during Run mode
            pub const Enabled: u32 = 0b1;
        }
    }

    /// SRAM2 retention in Standby mode
    pub mod RRS {
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

            /// 0b00: SRAM2 is powered off in Standby mode (SRAM2 content is lost)
            pub const PoweredOff: u32 = 0b00;

            /// 0b01: Full SRAM2 is powered by the low-power regulator in Standby mode (SRAM2 full content is kept)
            pub const PoweredOn: u32 = 0b01;

            /// 0b10: Only 4 Kbytes of SRAM2 is powered by the low-power regulator in Standby mode (4 Kbytes of SRAM2 content is kept)
            pub const PartialPoweredOn: u32 = 0b10;
        }
    }

    /// Enable Wakeup pin WKUP5
    pub mod EWUP5 {
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

            /// 0b0: External Wakeup pin WKUPx is disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: When this bit is set, the external wakeup pin WKUPx is enabled and triggers a wakeup from Standby or Shutdown event when a rising or a falling edge occurs. The active edge is configured via the WPx bit in the PWR_CR4 register
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Enable Wakeup pin WKUP4
    pub mod EWUP4 {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::EWUP5::RW;
    }

    /// Enable Wakeup pin WKUP3
    pub mod EWUP3 {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::EWUP5::RW;
    }

    /// Enable Wakeup pin WKUP2
    pub mod EWUP2 {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::EWUP5::RW;
    }

    /// Enable Wakeup pin WKUP1
    pub mod EWUP1 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::EWUP5::RW;
    }

    /// Enable Pull-down activation on DSI pins
    pub mod DSIPDEN {
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

            /// 0b0: Pull-Down is disabled on DSI pins
            pub const Disabled: u32 = 0b0;

            /// 0b1: Pull-Down is enabled on DSI pins
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Enable ULP sampling
    pub mod ENULP {
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

            /// 0b0: Sampling disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: When this bit is set, the BORL, BORH and PVD are periodically sampled instead continuous monitoring to reduce power consumption. Fast supply drop between two sample/compare phases is not detected in this mode. This bit has impact only on STOP2, Standby and shutdown low power modes
            pub const Enabled: u32 = 0b1;
        }
    }
}

/// Power control register 4
pub mod CR4 {

    /// VBAT battery charging resistor selection
    pub mod VBRS {
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

            /// 0b0: Charge VBAT through a 5 kOhms resistor
            pub const R5k: u32 = 0b0;

            /// 0b1: Charge VBAT through a 1.5 kOhms resistor
            pub const R1k5: u32 = 0b1;
        }
    }

    /// VBAT battery charging enable
    pub mod VBE {
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

            /// 0b0: VBAT battery charging disable
            pub const Disabled: u32 = 0b0;

            /// 0b1: VBAT battery charging enable
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Wakeup pin WKUP5 polarity
    pub mod WP5 {
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

            /// 0b0: Detection on high level (rising edge)
            pub const RisingEdge: u32 = 0b0;

            /// 0b1: Detection on low level (falling edge)
            pub const FallingEdge: u32 = 0b1;
        }
    }

    /// Wakeup pin WKUP4 polarity
    pub mod WP4 {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::WP5::RW;
    }

    /// Wakeup pin WKUP3 polarity
    pub mod WP3 {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::WP5::RW;
    }

    /// Wakeup pin WKUP2 polarity
    pub mod WP2 {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::WP5::RW;
    }

    /// Wakeup pin WKUP1 polarity
    pub mod WP1 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::WP5::RW;
    }

    /// External SMPS on
    pub mod EXT_SMPS_ON {
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

            /// 0b0: The external SMPS switch is open
            pub const Disabled: u32 = 0b0;

            /// 0b1: The external SMPS switch is closed, internal regulator output is set to 0.95 V
            pub const Enabled: u32 = 0b1;
        }
    }
}

/// Power status register 1
pub mod SR1 {

    /// Wakeup flag internal
    pub mod WUFI {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: This bit is set when a wakeup is detected on the internal wakeup line
            pub const Set: u32 = 0b0;

            /// 0b1: It is cleared when all internal wakeup sources are cleared
            pub const Cleared: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Standby flag
    pub mod SBF {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: The device did not enter the Standby mode
            pub const Set: u32 = 0b0;

            /// 0b1: The device entered the Standby mode
            pub const Cleared: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Wakeup flag 5
    pub mod WUF5 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: This bit is set when a wakeup event is detected on wakeup pin, WKUPx
            pub const Set: u32 = 0b0;

            /// 0b1: No wakeup event detected on WKUPx
            pub const Cleared: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Wakeup flag 4
    pub mod WUF4 {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        pub use super::WUF5::R;
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Wakeup flag 3
    pub mod WUF3 {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        pub use super::WUF5::R;
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Wakeup flag 2
    pub mod WUF2 {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        pub use super::WUF5::R;
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Wakeup flag 1
    pub mod WUF1 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        pub use super::WUF5::R;
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// External SMPS ready
    pub mod EXT_SMPS_RDY {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: Internal regulator not ready in Range 2, the external SMPS cannot be connected
            pub const NotReady: u32 = 0b0;

            /// 0b1: Internal regulator ready in Range 2, the external SMPS can be connected
            pub const Ready: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Power status register 2
pub mod SR2 {

    /// Peripheral voltage monitoring output: VDDA vs. 2.2 V
    pub mod PVMO4 {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: VDDA voltage is above PVM4 threshold (around 2.2 V)
            pub const Above: u32 = 0b0;

            /// 0b1: VDDA voltage is below PVM4 threshold (around 2.2 V)
            pub const Below: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Peripheral voltage monitoring output: VDDA vs. 1.62 V
    pub mod PVMO3 {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: VDDA voltage is above PVM3 threshold (around 1.62 V)
            pub const Above: u32 = 0b0;

            /// 0b1: VDDA voltage is below PVM3 threshold (around 1.62 V)
            pub const Below: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Peripheral voltage monitoring output: VDDIO2 vs. 0.9 V
    pub mod PVMO2 {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: VDDIO2 voltage is above PVM2 threshold (around 0.9 V)
            pub const Above: u32 = 0b0;

            /// 0b1: VDDIO2 voltage is below PVM2 threshold (around 0.9 V)
            pub const Below: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Peripheral voltage monitoring output: VDDUSB vs. 1.2 V
    pub mod PVMO1 {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: VDDUSB voltage is above PVM1 threshold (around 1.2 V)
            pub const Above: u32 = 0b0;

            /// 0b1: VDDUSB voltage is below PVM1 threshold (around 1.2 V)
            pub const Below: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Power voltage detector output
    pub mod PVDO {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: VDD is above the selected PVD threshold
            pub const Above: u32 = 0b0;

            /// 0b1: VDD is below the selected PVD threshold
            pub const Below: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Voltage scaling flag
    pub mod VOSF {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: The regulator is ready in the selected voltage range
            pub const Ready: u32 = 0b0;

            /// 0b1: The regulator output voltage is changing to the required voltage level
            pub const NotReady: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Low-power regulator flag
    pub mod REGLPF {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: The regulator is ready in main mode (MR)
            pub const MR: u32 = 0b0;

            /// 0b1: The regulator is in low-power mode (LPR)
            pub const LPR: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Low-power regulator started
    pub mod REGLPS {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: The low-power regulator is not ready
            pub const NotReady: u32 = 0b0;

            /// 0b1: The low-power regulator is ready
            pub const Ready: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Power status clear register
pub mod SCR {

    /// Clear standby flag
    pub mod CSBF {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values
        pub mod W {

            /// 0b1: Setting this bit clears the SBF flag in the PWR_SR1 register
            pub const Clear: u32 = 0b1;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Clear wakeup flag 5
    pub mod CWUF5 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values
        pub mod W {

            /// 0b1: Setting this bit clears the WUFx flag in the PWR_SR1 register
            pub const Clear: u32 = 0b1;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Clear wakeup flag 4
    pub mod CWUF4 {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::CWUF5::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Clear wakeup flag 3
    pub mod CWUF3 {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::CWUF5::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Clear wakeup flag 2
    pub mod CWUF2 {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::CWUF5::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Clear wakeup flag 1
    pub mod CWUF1 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::CWUF5::W;
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Power Port A pull-up control register
pub mod PUCRA {

    /// Port A pull-up bit y (y=0..15)
    pub mod PU15 {
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

            /// 0b0: Pull-Up on Pxx is disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Pull-Up on Pxx is enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Port A pull-up bit y (y=0..15)
    pub mod PU13 {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PU15::RW;
    }

    /// Port A pull-up bit y (y=0..15)
    pub mod PU12 {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PU15::RW;
    }

    /// Port A pull-up bit y (y=0..15)
    pub mod PU11 {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PU15::RW;
    }

    /// Port A pull-up bit y (y=0..15)
    pub mod PU10 {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PU15::RW;
    }

    /// Port A pull-up bit y (y=0..15)
    pub mod PU9 {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PU15::RW;
    }

    /// Port A pull-up bit y (y=0..15)
    pub mod PU8 {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PU15::RW;
    }

    /// Port A pull-up bit y (y=0..15)
    pub mod PU7 {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PU15::RW;
    }

    /// Port A pull-up bit y (y=0..15)
    pub mod PU6 {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PU15::RW;
    }

    /// Port A pull-up bit y (y=0..15)
    pub mod PU5 {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PU15::RW;
    }

    /// Port A pull-up bit y (y=0..15)
    pub mod PU4 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PU15::RW;
    }

    /// Port A pull-up bit y (y=0..15)
    pub mod PU3 {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PU15::RW;
    }

    /// Port A pull-up bit y (y=0..15)
    pub mod PU2 {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PU15::RW;
    }

    /// Port A pull-up bit y (y=0..15)
    pub mod PU1 {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PU15::RW;
    }

    /// Port A pull-up bit y (y=0..15)
    pub mod PU0 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PU15::RW;
    }
}

/// Power Port A pull-down control register
pub mod PDCRA {

    /// Port A pull-down bit y (y=0..15)
    pub mod PD14 {
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

            /// 0b0: Pull-Down on Pxx is disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Pull-Down on Pxx is enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Port A pull-down bit y (y=0..15)
    pub mod PD12 {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PD14::RW;
    }

    /// Port A pull-down bit y (y=0..15)
    pub mod PD11 {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PD14::RW;
    }

    /// Port A pull-down bit y (y=0..15)
    pub mod PD10 {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PD14::RW;
    }

    /// Port A pull-down bit y (y=0..15)
    pub mod PD9 {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PD14::RW;
    }

    /// Port A pull-down bit y (y=0..15)
    pub mod PD8 {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PD14::RW;
    }

    /// Port A pull-down bit y (y=0..15)
    pub mod PD7 {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PD14::RW;
    }

    /// Port A pull-down bit y (y=0..15)
    pub mod PD6 {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PD14::RW;
    }

    /// Port A pull-down bit y (y=0..15)
    pub mod PD5 {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PD14::RW;
    }

    /// Port A pull-down bit y (y=0..15)
    pub mod PD4 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PD14::RW;
    }

    /// Port A pull-down bit y (y=0..15)
    pub mod PD3 {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PD14::RW;
    }

    /// Port A pull-down bit y (y=0..15)
    pub mod PD2 {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PD14::RW;
    }

    /// Port A pull-down bit y (y=0..15)
    pub mod PD1 {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PD14::RW;
    }

    /// Port A pull-down bit y (y=0..15)
    pub mod PD0 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PD14::RW;
    }
}

/// Power Port B pull-up control register
pub mod PUCRB {

    /// Port B pull-up bit y (y=0..15)
    pub mod PU15 {
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

            /// 0b0: Pull-Up on Pxx is disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Pull-Up on Pxx is enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Port B pull-up bit y (y=0..15)
    pub mod PU14 {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PU15::RW;
    }

    /// Port B pull-up bit y (y=0..15)
    pub mod PU13 {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PU15::RW;
    }

    /// Port B pull-up bit y (y=0..15)
    pub mod PU12 {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PU15::RW;
    }

    /// Port B pull-up bit y (y=0..15)
    pub mod PU11 {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PU15::RW;
    }

    /// Port B pull-up bit y (y=0..15)
    pub mod PU10 {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PU15::RW;
    }

    /// Port B pull-up bit y (y=0..15)
    pub mod PU9 {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PU15::RW;
    }

    /// Port B pull-up bit y (y=0..15)
    pub mod PU8 {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PU15::RW;
    }

    /// Port B pull-up bit y (y=0..15)
    pub mod PU7 {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PU15::RW;
    }

    /// Port B pull-up bit y (y=0..15)
    pub mod PU6 {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PU15::RW;
    }

    /// Port B pull-up bit y (y=0..15)
    pub mod PU5 {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PU15::RW;
    }

    /// Port B pull-up bit y (y=0..15)
    pub mod PU4 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PU15::RW;
    }

    /// Port B pull-up bit y (y=0..15)
    pub mod PU3 {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PU15::RW;
    }

    /// Port B pull-up bit y (y=0..15)
    pub mod PU2 {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PU15::RW;
    }

    /// Port B pull-up bit y (y=0..15)
    pub mod PU1 {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PU15::RW;
    }

    /// Port B pull-up bit y (y=0..15)
    pub mod PU0 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PU15::RW;
    }
}

/// Power Port B pull-down control register
pub mod PDCRB {

    /// Port B pull-down bit y (y=0..15)
    pub mod PD15 {
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

            /// 0b0: Pull-Down on Pxx is disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Pull-Down on Pxx is enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Port B pull-down bit y (y=0..15)
    pub mod PD14 {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PD15::RW;
    }

    /// Port B pull-down bit y (y=0..15)
    pub mod PD13 {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PD15::RW;
    }

    /// Port B pull-down bit y (y=0..15)
    pub mod PD12 {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PD15::RW;
    }

    /// Port B pull-down bit y (y=0..15)
    pub mod PD11 {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PD15::RW;
    }

    /// Port B pull-down bit y (y=0..15)
    pub mod PD10 {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PD15::RW;
    }

    /// Port B pull-down bit y (y=0..15)
    pub mod PD9 {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PD15::RW;
    }

    /// Port B pull-down bit y (y=0..15)
    pub mod PD8 {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PD15::RW;
    }

    /// Port B pull-down bit y (y=0..15)
    pub mod PD7 {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PD15::RW;
    }

    /// Port B pull-down bit y (y=0..15)
    pub mod PD6 {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PD15::RW;
    }

    /// Port B pull-down bit y (y=0..15)
    pub mod PD5 {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PD15::RW;
    }

    /// Port B pull-down bit y (y=0..15)
    pub mod PD3 {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PD15::RW;
    }

    /// Port B pull-down bit y (y=0..15)
    pub mod PD2 {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PD15::RW;
    }

    /// Port B pull-down bit y (y=0..15)
    pub mod PD1 {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PD15::RW;
    }

    /// Port B pull-down bit y (y=0..15)
    pub mod PD0 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PD15::RW;
    }
}

/// Power Port C pull-up control register
pub mod PUCRC {
    pub use super::PUCRB::PU0;
    pub use super::PUCRB::PU1;
    pub use super::PUCRB::PU10;
    pub use super::PUCRB::PU11;
    pub use super::PUCRB::PU12;
    pub use super::PUCRB::PU13;
    pub use super::PUCRB::PU14;
    pub use super::PUCRB::PU15;
    pub use super::PUCRB::PU2;
    pub use super::PUCRB::PU3;
    pub use super::PUCRB::PU4;
    pub use super::PUCRB::PU5;
    pub use super::PUCRB::PU6;
    pub use super::PUCRB::PU7;
    pub use super::PUCRB::PU8;
    pub use super::PUCRB::PU9;
}

/// Power Port C pull-down control register
pub mod PDCRC {

    /// Port C pull-down bit y (y=0..15)
    pub mod PD15 {
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

            /// 0b0: Pull-Down on Pxx is disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Pull-Down on Pxx is enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Port C pull-down bit y (y=0..15)
    pub mod PD14 {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PD15::RW;
    }

    /// Port C pull-down bit y (y=0..15)
    pub mod PD13 {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PD15::RW;
    }

    /// Port C pull-down bit y (y=0..15)
    pub mod PD12 {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PD15::RW;
    }

    /// Port C pull-down bit y (y=0..15)
    pub mod PD11 {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PD15::RW;
    }

    /// Port C pull-down bit y (y=0..15)
    pub mod PD10 {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PD15::RW;
    }

    /// Port C pull-down bit y (y=0..15)
    pub mod PD9 {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PD15::RW;
    }

    /// Port C pull-down bit y (y=0..15)
    pub mod PD8 {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PD15::RW;
    }

    /// Port C pull-down bit y (y=0..15)
    pub mod PD7 {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PD15::RW;
    }

    /// Port C pull-down bit y (y=0..15)
    pub mod PD6 {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PD15::RW;
    }

    /// Port C pull-down bit y (y=0..15)
    pub mod PD5 {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PD15::RW;
    }

    /// Port C pull-down bit y (y=0..15)
    pub mod PD4 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PD15::RW;
    }

    /// Port C pull-down bit y (y=0..15)
    pub mod PD3 {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PD15::RW;
    }

    /// Port C pull-down bit y (y=0..15)
    pub mod PD2 {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PD15::RW;
    }

    /// Port C pull-down bit y (y=0..15)
    pub mod PD1 {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PD15::RW;
    }

    /// Port C pull-down bit y (y=0..15)
    pub mod PD0 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PD15::RW;
    }
}

/// Power Port D pull-up control register
pub mod PUCRD {
    pub use super::PUCRB::PU0;
    pub use super::PUCRB::PU1;
    pub use super::PUCRB::PU10;
    pub use super::PUCRB::PU11;
    pub use super::PUCRB::PU12;
    pub use super::PUCRB::PU13;
    pub use super::PUCRB::PU14;
    pub use super::PUCRB::PU15;
    pub use super::PUCRB::PU2;
    pub use super::PUCRB::PU3;
    pub use super::PUCRB::PU4;
    pub use super::PUCRB::PU5;
    pub use super::PUCRB::PU6;
    pub use super::PUCRB::PU7;
    pub use super::PUCRB::PU8;
    pub use super::PUCRB::PU9;
}

/// Power Port D pull-down control register
pub mod PDCRD {
    pub use super::PDCRC::PD0;
    pub use super::PDCRC::PD1;
    pub use super::PDCRC::PD10;
    pub use super::PDCRC::PD11;
    pub use super::PDCRC::PD12;
    pub use super::PDCRC::PD13;
    pub use super::PDCRC::PD14;
    pub use super::PDCRC::PD15;
    pub use super::PDCRC::PD2;
    pub use super::PDCRC::PD3;
    pub use super::PDCRC::PD4;
    pub use super::PDCRC::PD5;
    pub use super::PDCRC::PD6;
    pub use super::PDCRC::PD7;
    pub use super::PDCRC::PD8;
    pub use super::PDCRC::PD9;
}

/// Power Port E pull-up control register
pub mod PUCRE {
    pub use super::PUCRB::PU0;
    pub use super::PUCRB::PU1;
    pub use super::PUCRB::PU10;
    pub use super::PUCRB::PU11;
    pub use super::PUCRB::PU12;
    pub use super::PUCRB::PU13;
    pub use super::PUCRB::PU14;
    pub use super::PUCRB::PU15;
    pub use super::PUCRB::PU2;
    pub use super::PUCRB::PU3;
    pub use super::PUCRB::PU4;
    pub use super::PUCRB::PU5;
    pub use super::PUCRB::PU6;
    pub use super::PUCRB::PU7;
    pub use super::PUCRB::PU8;
    pub use super::PUCRB::PU9;
}

/// Power Port E pull-down control register
pub mod PDCRE {
    pub use super::PDCRC::PD0;
    pub use super::PDCRC::PD1;
    pub use super::PDCRC::PD10;
    pub use super::PDCRC::PD11;
    pub use super::PDCRC::PD12;
    pub use super::PDCRC::PD13;
    pub use super::PDCRC::PD14;
    pub use super::PDCRC::PD15;
    pub use super::PDCRC::PD2;
    pub use super::PDCRC::PD3;
    pub use super::PDCRC::PD4;
    pub use super::PDCRC::PD5;
    pub use super::PDCRC::PD6;
    pub use super::PDCRC::PD7;
    pub use super::PDCRC::PD8;
    pub use super::PDCRC::PD9;
}

/// Power Port F pull-up control register
pub mod PUCRF {
    pub use super::PUCRB::PU0;
    pub use super::PUCRB::PU1;
    pub use super::PUCRB::PU10;
    pub use super::PUCRB::PU11;
    pub use super::PUCRB::PU12;
    pub use super::PUCRB::PU13;
    pub use super::PUCRB::PU14;
    pub use super::PUCRB::PU15;
    pub use super::PUCRB::PU2;
    pub use super::PUCRB::PU3;
    pub use super::PUCRB::PU4;
    pub use super::PUCRB::PU5;
    pub use super::PUCRB::PU6;
    pub use super::PUCRB::PU7;
    pub use super::PUCRB::PU8;
    pub use super::PUCRB::PU9;
}

/// Power Port F pull-down control register
pub mod PDCRF {
    pub use super::PDCRC::PD0;
    pub use super::PDCRC::PD1;
    pub use super::PDCRC::PD10;
    pub use super::PDCRC::PD11;
    pub use super::PDCRC::PD12;
    pub use super::PDCRC::PD13;
    pub use super::PDCRC::PD14;
    pub use super::PDCRC::PD15;
    pub use super::PDCRC::PD2;
    pub use super::PDCRC::PD3;
    pub use super::PDCRC::PD4;
    pub use super::PDCRC::PD5;
    pub use super::PDCRC::PD6;
    pub use super::PDCRC::PD7;
    pub use super::PDCRC::PD8;
    pub use super::PDCRC::PD9;
}

/// Power Port G pull-up control register
pub mod PUCRG {
    pub use super::PUCRB::PU0;
    pub use super::PUCRB::PU1;
    pub use super::PUCRB::PU10;
    pub use super::PUCRB::PU11;
    pub use super::PUCRB::PU12;
    pub use super::PUCRB::PU13;
    pub use super::PUCRB::PU14;
    pub use super::PUCRB::PU15;
    pub use super::PUCRB::PU2;
    pub use super::PUCRB::PU3;
    pub use super::PUCRB::PU4;
    pub use super::PUCRB::PU5;
    pub use super::PUCRB::PU6;
    pub use super::PUCRB::PU7;
    pub use super::PUCRB::PU8;
    pub use super::PUCRB::PU9;
}

/// Power Port G pull-down control register
pub mod PDCRG {
    pub use super::PDCRC::PD0;
    pub use super::PDCRC::PD1;
    pub use super::PDCRC::PD10;
    pub use super::PDCRC::PD11;
    pub use super::PDCRC::PD12;
    pub use super::PDCRC::PD13;
    pub use super::PDCRC::PD14;
    pub use super::PDCRC::PD15;
    pub use super::PDCRC::PD2;
    pub use super::PDCRC::PD3;
    pub use super::PDCRC::PD4;
    pub use super::PDCRC::PD5;
    pub use super::PDCRC::PD6;
    pub use super::PDCRC::PD7;
    pub use super::PDCRC::PD8;
    pub use super::PDCRC::PD9;
}

/// Power Port H pull-up control register
pub mod PUCRH {

    /// Port H pull-up bit y (y=0..15)
    pub mod PU1 {
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

            /// 0b0: Pull-Up on Pxx is disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Pull-Up on Pxx is enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Port H pull-up bit y (y=0..15)
    pub mod PU0 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PU1::RW;
    }

    /// Port H pull-up bit y (y=0..15)
    pub mod PU2 {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PU1::RW;
    }

    /// Port H pull-up bit y (y=0..15)
    pub mod PU3 {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PU1::RW;
    }

    /// Port H pull-up bit y (y=0..15)
    pub mod PU4 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PU1::RW;
    }

    /// Port H pull-up bit y (y=0..15)
    pub mod PU5 {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PU1::RW;
    }

    /// Port H pull-up bit y (y=0..15)
    pub mod PU6 {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PU1::RW;
    }

    /// Port H pull-up bit y (y=0..15)
    pub mod PU7 {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PU1::RW;
    }

    /// Port H pull-up bit y (y=0..15)
    pub mod PU8 {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PU1::RW;
    }

    /// Port H pull-up bit y (y=0..15)
    pub mod PU9 {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PU1::RW;
    }

    /// Port H pull-up bit y (y=0..15)
    pub mod PU10 {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PU1::RW;
    }

    /// Port H pull-up bit y (y=0..15)
    pub mod PU11 {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PU1::RW;
    }

    /// Port H pull-up bit y (y=0..15)
    pub mod PU12 {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PU1::RW;
    }

    /// Port H pull-up bit y (y=0..15)
    pub mod PU13 {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PU1::RW;
    }

    /// Port H pull-up bit y (y=0..15)
    pub mod PU14 {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PU1::RW;
    }

    /// Port H pull-up bit y (y=0..15)
    pub mod PU15 {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PU1::RW;
    }
}

/// Power Port H pull-down control register
pub mod PDCRH {

    /// Port H pull-down bit y (y=0..15)
    pub mod PD1 {
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

            /// 0b0: Pull-Down on Pxx is disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Pull-Down on Pxx is enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Port H pull-down bit y (y=0..15)
    pub mod PD0 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PD1::RW;
    }

    /// Port H pull-down bit y (y=0..15)
    pub mod PD2 {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PD1::RW;
    }

    /// Port H pull-down bit y (y=0..15)
    pub mod PD3 {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PD1::RW;
    }

    /// Port H pull-down bit y (y=0..15)
    pub mod PD4 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PD1::RW;
    }

    /// Port H pull-down bit y (y=0..15)
    pub mod PD5 {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PD1::RW;
    }

    /// Port H pull-down bit y (y=0..15)
    pub mod PD6 {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PD1::RW;
    }

    /// Port H pull-down bit y (y=0..15)
    pub mod PD7 {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PD1::RW;
    }

    /// Port H pull-down bit y (y=0..15)
    pub mod PD8 {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PD1::RW;
    }

    /// Port H pull-down bit y (y=0..15)
    pub mod PD9 {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PD1::RW;
    }

    /// Port H pull-down bit y (y=0..15)
    pub mod PD10 {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PD1::RW;
    }

    /// Port H pull-down bit y (y=0..15)
    pub mod PD11 {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PD1::RW;
    }

    /// Port H pull-down bit y (y=0..15)
    pub mod PD12 {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PD1::RW;
    }

    /// Port H pull-down bit y (y=0..15)
    pub mod PD13 {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PD1::RW;
    }

    /// Port H pull-down bit y (y=0..15)
    pub mod PD14 {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PD1::RW;
    }

    /// Port H pull-down bit y (y=0..15)
    pub mod PD15 {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PD1::RW;
    }
}

/// Power Port I pull-up control register
pub mod PUCRI {

    /// Port I pull-up bit y (y=0..11)
    pub mod PU11 {
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

            /// 0b0: Pull-Up on Pxx is disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Pull-Up on Pxx is enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Port I pull-up bit y (y=0..11)
    pub mod PU10 {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PU11::RW;
    }

    /// Port I pull-up bit y (y=0..11)
    pub mod PU9 {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PU11::RW;
    }

    /// Port I pull-up bit y (y=0..11)
    pub mod PU8 {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PU11::RW;
    }

    /// Port I pull-up bit y (y=0..11)
    pub mod PU7 {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PU11::RW;
    }

    /// Port I pull-up bit y (y=0..11)
    pub mod PU6 {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PU11::RW;
    }

    /// Port I pull-up bit y (y=0..11)
    pub mod PU5 {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PU11::RW;
    }

    /// Port I pull-up bit y (y=0..11)
    pub mod PU4 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PU11::RW;
    }

    /// Port I pull-up bit y (y=0..11)
    pub mod PU3 {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PU11::RW;
    }

    /// Port I pull-up bit y (y=0..11)
    pub mod PU2 {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PU11::RW;
    }

    /// Port I pull-up bit y (y=0..11)
    pub mod PU1 {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PU11::RW;
    }

    /// Port I pull-up bit y (y=0..11)
    pub mod PU0 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PU11::RW;
    }
}

/// Power Port I pull-down control register
pub mod PDCRI {

    /// Port I pull-down bit y (y=0..11)
    pub mod PD11 {
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

            /// 0b0: Pull-Down on Pxx is disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Pull-Down on Pxx is enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Port I pull-down bit y (y=0..11)
    pub mod PD10 {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PD11::RW;
    }

    /// Port I pull-down bit y (y=0..11)
    pub mod PD9 {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PD11::RW;
    }

    /// Port I pull-down bit y (y=0..11)
    pub mod PD8 {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PD11::RW;
    }

    /// Port I pull-down bit y (y=0..11)
    pub mod PD7 {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PD11::RW;
    }

    /// Port I pull-down bit y (y=0..11)
    pub mod PD6 {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PD11::RW;
    }

    /// Port I pull-down bit y (y=0..11)
    pub mod PD5 {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PD11::RW;
    }

    /// Port I pull-down bit y (y=0..11)
    pub mod PD4 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PD11::RW;
    }

    /// Port I pull-down bit y (y=0..11)
    pub mod PD3 {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PD11::RW;
    }

    /// Port I pull-down bit y (y=0..11)
    pub mod PD2 {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PD11::RW;
    }

    /// Port I pull-down bit y (y=0..11)
    pub mod PD1 {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PD11::RW;
    }

    /// Port I pull-down bit y (y=0..11)
    pub mod PD0 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PD11::RW;
    }
}

/// control register 5
pub mod CR5 {

    /// Main regulator Range 1 mode
    pub mod R1MODE {
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

            /// 0b0: Main regulator in Range 1 boost mode
            pub const BoostMode: u32 = 0b0;

            /// 0b1: Main regulator in Range 1 normal mode
            pub const NormalMode: u32 = 0b1;
        }
    }
}
#[repr(C)]
pub struct RegisterBlock {
    /// Power control register 1
    pub CR1: RWRegister<u32>,

    /// Power control register 2
    pub CR2: RWRegister<u32>,

    /// Power control register 3
    pub CR3: RWRegister<u32>,

    /// Power control register 4
    pub CR4: RWRegister<u32>,

    /// Power status register 1
    pub SR1: RORegister<u32>,

    /// Power status register 2
    pub SR2: RORegister<u32>,

    /// Power status clear register
    pub SCR: WORegister<u32>,

    _reserved1: [u8; 4],

    /// Power Port A pull-up control register
    pub PUCRA: RWRegister<u32>,

    /// Power Port A pull-down control register
    pub PDCRA: RWRegister<u32>,

    /// Power Port B pull-up control register
    pub PUCRB: RWRegister<u32>,

    /// Power Port B pull-down control register
    pub PDCRB: RWRegister<u32>,

    /// Power Port C pull-up control register
    pub PUCRC: RWRegister<u32>,

    /// Power Port C pull-down control register
    pub PDCRC: RWRegister<u32>,

    /// Power Port D pull-up control register
    pub PUCRD: RWRegister<u32>,

    /// Power Port D pull-down control register
    pub PDCRD: RWRegister<u32>,

    /// Power Port E pull-up control register
    pub PUCRE: RWRegister<u32>,

    /// Power Port E pull-down control register
    pub PDCRE: RWRegister<u32>,

    /// Power Port F pull-up control register
    pub PUCRF: RWRegister<u32>,

    /// Power Port F pull-down control register
    pub PDCRF: RWRegister<u32>,

    /// Power Port G pull-up control register
    pub PUCRG: RWRegister<u32>,

    /// Power Port G pull-down control register
    pub PDCRG: RWRegister<u32>,

    /// Power Port H pull-up control register
    pub PUCRH: RWRegister<u32>,

    /// Power Port H pull-down control register
    pub PDCRH: RWRegister<u32>,

    /// Power Port I pull-up control register
    pub PUCRI: RWRegister<u32>,

    /// Power Port I pull-down control register
    pub PDCRI: RWRegister<u32>,

    _reserved2: [u8; 24],

    /// control register 5
    pub CR5: RWRegister<u32>,
}
pub struct ResetValues {
    pub CR1: u32,
    pub CR2: u32,
    pub CR3: u32,
    pub CR4: u32,
    pub SR1: u32,
    pub SR2: u32,
    pub SCR: u32,
    pub PUCRA: u32,
    pub PDCRA: u32,
    pub PUCRB: u32,
    pub PDCRB: u32,
    pub PUCRC: u32,
    pub PDCRC: u32,
    pub PUCRD: u32,
    pub PDCRD: u32,
    pub PUCRE: u32,
    pub PDCRE: u32,
    pub PUCRF: u32,
    pub PDCRF: u32,
    pub PUCRG: u32,
    pub PDCRG: u32,
    pub PUCRH: u32,
    pub PDCRH: u32,
    pub PUCRI: u32,
    pub PDCRI: u32,
    pub CR5: u32,
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

/// Access functions for the PWR peripheral instance
pub mod PWR {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x40007000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in PWR
    pub const reset: ResetValues = ResetValues {
        CR1: 0x00000200,
        CR2: 0x00000000,
        CR3: 0x00008000,
        CR4: 0x00000000,
        SR1: 0x00000000,
        SR2: 0x00000000,
        SCR: 0x00000000,
        PUCRA: 0x00000000,
        PDCRA: 0x00000000,
        PUCRB: 0x00000000,
        PDCRB: 0x00000000,
        PUCRC: 0x00000000,
        PDCRC: 0x00000000,
        PUCRD: 0x00000000,
        PDCRD: 0x00000000,
        PUCRE: 0x00000000,
        PDCRE: 0x00000000,
        PUCRF: 0x00000000,
        PDCRF: 0x00000000,
        PUCRG: 0x00000000,
        PDCRG: 0x00000000,
        PUCRH: 0x00000000,
        PDCRH: 0x00000000,
        PUCRI: 0x00000000,
        PDCRI: 0x00000000,
        CR5: 0x00000100,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut PWR_TAKEN: bool = false;

    /// Safe access to PWR
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
            if PWR_TAKEN {
                None
            } else {
                PWR_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to PWR
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if PWR_TAKEN && inst.addr == INSTANCE.addr {
                PWR_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal PWR
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        PWR_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to PWR
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const PWR: *const RegisterBlock = 0x40007000 as *const _;
