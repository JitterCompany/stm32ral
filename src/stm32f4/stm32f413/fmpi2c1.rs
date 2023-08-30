#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! fast-mode Inter-integrated circuit

use crate::{RORegister, RWRegister, WORegister};
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// Control register 1
pub mod CR1 {

    /// Peripheral enable
    pub mod PE {
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

            /// 0b0: Peripheral disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Peripheral enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// TXIE
    pub mod TXIE {
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

            /// 0b0: Transmit (TXIS) interrupt disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Transmit (TXIS) interrupt enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// RXIE
    pub mod RXIE {
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

            /// 0b0: Receive (RXNE) interrupt disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Receive (RXNE) interrupt enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// ADDRE
    pub mod ADDRIE {
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

            /// 0b0: Address match (ADDR) interrupts disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Address match (ADDR) interrupts enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// NACKIE
    pub mod NACKIE {
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

            /// 0b0: Not acknowledge (NACKF) received interrupts disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Not acknowledge (NACKF) received interrupts enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// STOPIE
    pub mod STOPIE {
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

            /// 0b0: Stop detection (STOPF) interrupt disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Stop detection (STOPF) interrupt enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// TCIE
    pub mod TCIE {
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

            /// 0b0: Transfer Complete interrupt disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Transfer Complete interrupt enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// ERRIE
    pub mod ERRIE {
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

            /// 0b0: Error detection interrupts disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Error detection interrupts enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// DNF
    pub mod DNF {
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

            /// 0b0000: Digital filter disabled
            pub const NoFilter: u32 = 0b0000;

            /// 0b0001: Digital filter enabled and filtering capability up to 1 tI2CCLK
            pub const Filter1: u32 = 0b0001;

            /// 0b0010: Digital filter enabled and filtering capability up to 2 tI2CCLK
            pub const Filter2: u32 = 0b0010;

            /// 0b0011: Digital filter enabled and filtering capability up to 3 tI2CCLK
            pub const Filter3: u32 = 0b0011;

            /// 0b0100: Digital filter enabled and filtering capability up to 4 tI2CCLK
            pub const Filter4: u32 = 0b0100;

            /// 0b0101: Digital filter enabled and filtering capability up to 5 tI2CCLK
            pub const Filter5: u32 = 0b0101;

            /// 0b0110: Digital filter enabled and filtering capability up to 6 tI2CCLK
            pub const Filter6: u32 = 0b0110;

            /// 0b0111: Digital filter enabled and filtering capability up to 7 tI2CCLK
            pub const Filter7: u32 = 0b0111;

            /// 0b1000: Digital filter enabled and filtering capability up to 8 tI2CCLK
            pub const Filter8: u32 = 0b1000;

            /// 0b1001: Digital filter enabled and filtering capability up to 9 tI2CCLK
            pub const Filter9: u32 = 0b1001;

            /// 0b1010: Digital filter enabled and filtering capability up to 10 tI2CCLK
            pub const Filter10: u32 = 0b1010;

            /// 0b1011: Digital filter enabled and filtering capability up to 11 tI2CCLK
            pub const Filter11: u32 = 0b1011;

            /// 0b1100: Digital filter enabled and filtering capability up to 12 tI2CCLK
            pub const Filter12: u32 = 0b1100;

            /// 0b1101: Digital filter enabled and filtering capability up to 13 tI2CCLK
            pub const Filter13: u32 = 0b1101;

            /// 0b1110: Digital filter enabled and filtering capability up to 14 tI2CCLK
            pub const Filter14: u32 = 0b1110;

            /// 0b1111: Digital filter enabled and filtering capability up to 15 tI2CCLK
            pub const Filter15: u32 = 0b1111;
        }
    }

    /// ANFOFF
    pub mod ANFOFF {
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

            /// 0b0: Analog noise filter enabled
            pub const Enabled: u32 = 0b0;

            /// 0b1: Analog noise filter disabled
            pub const Disabled: u32 = 0b1;
        }
    }

    /// TCDMAEN
    pub mod TXDMAEN {
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

            /// 0b0: DMA mode disabled for transmission
            pub const Disabled: u32 = 0b0;

            /// 0b1: DMA mode enabled for transmission
            pub const Enabled: u32 = 0b1;
        }
    }

    /// RXDMAEN
    pub mod RXDMAEN {
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

            /// 0b0: DMA mode disabled for reception
            pub const Disabled: u32 = 0b0;

            /// 0b1: DMA mode enabled for reception
            pub const Enabled: u32 = 0b1;
        }
    }

    /// SBC
    pub mod SBC {
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

            /// 0b0: Slave byte control disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Slave byte control enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// NOSTRETCH
    pub mod NOSTRETCH {
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

            /// 0b0: Clock stretching enabled
            pub const Enabled: u32 = 0b0;

            /// 0b1: Clock stretching disabled
            pub const Disabled: u32 = 0b1;
        }
    }

    /// GCEN
    pub mod GCEN {
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

            /// 0b0: General call disabled. Address 0b00000000 is NACKed
            pub const Disabled: u32 = 0b0;

            /// 0b1: General call enabled. Address 0b00000000 is ACKed
            pub const Enabled: u32 = 0b1;
        }
    }

    /// SMBHEN
    pub mod SMBHEN {
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

            /// 0b0: Host address disabled. Address 0b0001000x is NACKed
            pub const Disabled: u32 = 0b0;

            /// 0b1: Host address enabled. Address 0b0001000x is ACKed
            pub const Enabled: u32 = 0b1;
        }
    }

    /// SMBDEN
    pub mod SMBDEN {
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

            /// 0b0: Device default address disabled. Address 0b1100001x is NACKed
            pub const Disabled: u32 = 0b0;

            /// 0b1: Device default address enabled. Address 0b1100001x is ACKed
            pub const Enabled: u32 = 0b1;
        }
    }

    /// ALERTEN
    pub mod ALERTEN {
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

            /// 0b0: In device mode (SMBHEN=Disabled) Releases SMBA pin high and Alert Response Address Header disabled (0001100x) followed by NACK. In host mode (SMBHEN=Enabled) SMBus Alert pin (SMBA) not supported
            pub const Disabled: u32 = 0b0;

            /// 0b1: In device mode (SMBHEN=Disabled) Drives SMBA pin low and Alert Response Address Header enabled (0001100x) followed by ACK.In host mode (SMBHEN=Enabled) SMBus Alert pin (SMBA) supported
            pub const Enabled: u32 = 0b1;
        }
    }

    /// PECEN
    pub mod PECEN {
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

            /// 0b0: PEC calculation disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: PEC calculation enabled
            pub const Enabled: u32 = 0b1;
        }
    }
}

/// Control register 2
pub mod CR2 {

    /// Transfer direction
    pub mod RD_WRN {
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

            /// 0b0: Master requests a write transfer
            pub const Write: u32 = 0b0;

            /// 0b1: Master requests a read transfer
            pub const Read: u32 = 0b1;
        }
    }

    /// 10-bit addressing mode
    pub mod ADD10 {
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

            /// 0b0: The master operates in 7-bit addressing mode
            pub const Bit7: u32 = 0b0;

            /// 0b1: The master operates in 10-bit addressing mode
            pub const Bit10: u32 = 0b1;
        }
    }

    /// 10-bit address header only read direction
    pub mod HEAD10R {
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

            /// 0b0: The master sends the complete 10 bit slave address read sequence
            pub const Complete: u32 = 0b0;

            /// 0b1: The master only sends the 1st 7 bits of the 10 bit address, followed by Read direction
            pub const Partial: u32 = 0b1;
        }
    }

    /// Start generation
    pub mod START {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: No Start generation
            pub const NoStart: u32 = 0b0;

            /// 0b1: Restart/Start generation
            pub const Start: u32 = 0b1;
        }
        /// Write-only values
        pub mod W {

            /// 0b1: Restart/Start generation
            pub const Start: u32 = 0b1;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Stop generation
    pub mod STOP {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: No Stop generation
            pub const NoStop: u32 = 0b0;

            /// 0b1: Stop generation after current byte transfer
            pub const Stop: u32 = 0b1;
        }
        /// Write-only values
        pub mod W {

            /// 0b1: Stop generation after current byte transfer
            pub const Stop: u32 = 0b1;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// NACK generation
    pub mod NACK {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: an ACK is sent after current received byte
            pub const Ack: u32 = 0b0;

            /// 0b1: a NACK is sent after current received byte
            pub const Nack: u32 = 0b1;
        }
        /// Write-only values
        pub mod W {

            /// 0b1: a NACK is sent after current received byte
            pub const Nack: u32 = 0b1;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Number of bytes
    pub mod NBYTES {
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

    /// NBYTES reload mode
    pub mod RELOAD {
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

            /// 0b0: The transfer is completed after the NBYTES data transfer (STOP or RESTART will follow)
            pub const Completed: u32 = 0b0;

            /// 0b1: The transfer is not completed after the NBYTES data transfer (NBYTES will be reloaded)
            pub const NotCompleted: u32 = 0b1;
        }
    }

    /// Automatic end mode
    pub mod AUTOEND {
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

            /// 0b0: Software end mode: TC flag is set when NBYTES data are transferred, stretching SCL low
            pub const Software: u32 = 0b0;

            /// 0b1: Automatic end mode: a STOP condition is automatically sent when NBYTES data are transferred
            pub const Automatic: u32 = 0b1;
        }
    }

    /// Packet error checking byte
    pub mod PECBYTE {
        /// Offset (26 bits)
        pub const offset: u32 = 26;
        /// Mask (1 bit: 1 << 26)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: No PEC transfer
            pub const NoPec: u32 = 0b0;

            /// 0b1: PEC transmission/reception is requested
            pub const Pec: u32 = 0b1;
        }
        /// Write-only values
        pub mod W {

            /// 0b1: PEC transmission/reception is requested
            pub const Pec: u32 = 0b1;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Slave address bit 0
    pub mod SADD {
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

/// Own address register 1
pub mod OAR1 {

    /// OA1
    pub mod OA1 {
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

    /// OA11_7
    pub mod OA11_7 {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (7 bits: 0x7f << 1)
        pub const mask: u32 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// OA18_9
    pub mod OA18_9 {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (2 bits: 0b11 << 8)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// OA1MODE
    pub mod OA1MODE {
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

            /// 0b0: Own address 1 is a 7-bit address
            pub const Bit7: u32 = 0b0;

            /// 0b1: Own address 1 is a 10-bit address
            pub const Bit10: u32 = 0b1;
        }
    }

    /// OA1EN
    pub mod OA1EN {
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

            /// 0b0: Own address 1 disabled. The received slave address OA1 is NACKed
            pub const Disabled: u32 = 0b0;

            /// 0b1: Own address 1 enabled. The received slave address OA1 is ACKed
            pub const Enabled: u32 = 0b1;
        }
    }
}

/// Own address register 2
pub mod OAR2 {

    /// OA21_7
    pub mod OA2 {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (7 bits: 0x7f << 1)
        pub const mask: u32 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// OA2MSK
    pub mod OA2MSK {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (3 bits: 0b111 << 8)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: No mask
            pub const NoMask: u32 = 0b000;

            /// 0b001: OA2\[1\] is masked and don’t care. Only OA2\[7:2\] are compared
            pub const Mask1: u32 = 0b001;

            /// 0b010: OA2\[2:1\] are masked and don’t care. Only OA2\[7:3\] are compared
            pub const Mask2: u32 = 0b010;

            /// 0b011: OA2\[3:1\] are masked and don’t care. Only OA2\[7:4\] are compared
            pub const Mask3: u32 = 0b011;

            /// 0b100: OA2\[4:1\] are masked and don’t care. Only OA2\[7:5\] are compared
            pub const Mask4: u32 = 0b100;

            /// 0b101: OA2\[5:1\] are masked and don’t care. Only OA2\[7:6\] are compared
            pub const Mask5: u32 = 0b101;

            /// 0b110: OA2\[6:1\] are masked and don’t care. Only OA2\[7\] is compared.
            pub const Mask6: u32 = 0b110;

            /// 0b111: OA2\[7:1\] are masked and don’t care. No comparison is done, and all (except reserved) 7-bit received addresses are acknowledged
            pub const Mask7: u32 = 0b111;
        }
    }

    /// OA2EN
    pub mod OA2EN {
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

            /// 0b0: Own address 2 disabled. The received slave address OA2 is NACKed
            pub const Disabled: u32 = 0b0;

            /// 0b1: Own address 2 enabled. The received slave address OA2 is ACKed
            pub const Enabled: u32 = 0b1;
        }
    }
}

/// Timing register
pub mod TIMINGR {

    /// SCLL
    pub mod SCLL {
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

    /// SCLH
    pub mod SCLH {
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

    /// SDADEL
    pub mod SDADEL {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (4 bits: 0b1111 << 16)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// SCLDEL
    pub mod SCLDEL {
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

    /// PRESC
    pub mod PRESC {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (4 bits: 0b1111 << 28)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Timeout register
pub mod TIMEOUTR {

    /// TIMEOUTA
    pub mod TIMEOUTA {
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

    /// TIDLE
    pub mod TIDLE {
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

            /// 0b0: TIMEOUTA is used to detect SCL low timeout
            pub const Disabled: u32 = 0b0;

            /// 0b1: TIMEOUTA is used to detect both SCL and SDA high timeout (bus idle condition)
            pub const Enabled: u32 = 0b1;
        }
    }

    /// TIMOUTEN
    pub mod TIMOUTEN {
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

            /// 0b0: SCL timeout detection is disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: SCL timeout detection is enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// TIMEOUTB
    pub mod TIMEOUTB {
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

    /// TEXTEN
    pub mod TEXTEN {
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

            /// 0b0: Extended clock timeout detection is disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Extended clock timeout detection is enabled
            pub const Enabled: u32 = 0b1;
        }
    }
}

/// Interrupt and Status register
pub mod ISR {

    /// TXE
    pub mod TXE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: TXDR register not empty
            pub const NotEmpty: u32 = 0b0;

            /// 0b1: TXDR register empty
            pub const Empty: u32 = 0b1;
        }
        /// Write-only values
        pub mod W {

            /// 0b1: Flush the transmit data register
            pub const Flush: u32 = 0b1;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// TXIS
    pub mod TXIS {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: The TXDR register is not empty
            pub const NotEmpty: u32 = 0b0;

            /// 0b1: The TXDR register is empty and the data to be transmitted must be written in the TXDR register
            pub const Empty: u32 = 0b1;
        }
        /// Write-only values
        pub mod W {

            /// 0b1: Generate a TXIS event
            pub const Trigger: u32 = 0b1;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// RXNE
    pub mod RXNE {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: The RXDR register is empty
            pub const Empty: u32 = 0b0;

            /// 0b1: Received data is copied into the RXDR register, and is ready to be read
            pub const NotEmpty: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// ADDR
    pub mod ADDR {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: Adress mismatched or not received
            pub const NotMatch: u32 = 0b0;

            /// 0b1: Received slave address matched with one of the enabled slave addresses
            pub const Match: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// NACKF
    pub mod NACKF {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: No NACK has been received
            pub const NoNack: u32 = 0b0;

            /// 0b1: NACK has been received
            pub const Nack: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// STOPF
    pub mod STOPF {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: No Stop condition detected
            pub const NoStop: u32 = 0b0;

            /// 0b1: Stop condition detected
            pub const Stop: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// TC
    pub mod TC {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: Transfer is not complete
            pub const NotComplete: u32 = 0b0;

            /// 0b1: NBYTES has been transfered
            pub const Complete: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// TCR
    pub mod TCR {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        pub use super::TC::R;
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// BERR
    pub mod BERR {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: No bus error
            pub const NoError: u32 = 0b0;

            /// 0b1: Misplaced Start and Stop condition is detected
            pub const Error: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// ARLO
    pub mod ARLO {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: No arbitration lost
            pub const NotLost: u32 = 0b0;

            /// 0b1: Arbitration lost
            pub const Lost: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// OVR
    pub mod OVR {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: No overrun/underrun error occurs
            pub const NoOverrun: u32 = 0b0;

            /// 0b1: slave mode with NOSTRETCH=1, when an overrun/underrun error occurs
            pub const Overrun: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// PECERR
    pub mod PECERR {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: Received PEC does match with PEC register
            pub const Match: u32 = 0b0;

            /// 0b1: Received PEC does not match with PEC register
            pub const NoMatch: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// TIMEOUT
    pub mod TIMEOUT {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: No timeout occured
            pub const NoTimeout: u32 = 0b0;

            /// 0b1: Timeout occured
            pub const Timeout: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// ALERT
    pub mod ALERT {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: SMBA alert is not detected
            pub const NoAlert: u32 = 0b0;

            /// 0b1: SMBA alert event is detected on SMBA pin
            pub const Alert: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// BUSY
    pub mod BUSY {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: No communication is in progress on the bus
            pub const NotBusy: u32 = 0b0;

            /// 0b1: A communication is in progress on the bus
            pub const Busy: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// DIR
    pub mod DIR {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: Write transfer, slave enters receiver mode
            pub const Write: u32 = 0b0;

            /// 0b1: Read transfer, slave enters transmitter mode
            pub const Read: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// ADDCODE
    pub mod ADDCODE {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (7 bits: 0x7f << 17)
        pub const mask: u32 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt clear register
pub mod ICR {

    /// Address matched flag clear
    pub mod ADDRCF {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values
        pub mod W {

            /// 0b1: Clears the ADDR flag in ISR register
            pub const Clear: u32 = 0b1;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Not Acknowledge flag clear
    pub mod NACKCF {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values
        pub mod W {

            /// 0b1: Clears the NACK flag in ISR register
            pub const Clear: u32 = 0b1;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Stop detection flag clear
    pub mod STOPCF {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values
        pub mod W {

            /// 0b1: Clears the STOP flag in ISR register
            pub const Clear: u32 = 0b1;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Bus error flag clear
    pub mod BERRCF {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values
        pub mod W {

            /// 0b1: Clears the BERR flag in ISR register
            pub const Clear: u32 = 0b1;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Arbitration Lost flag clear
    pub mod ARLOCF {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values
        pub mod W {

            /// 0b1: Clears the ARLO flag in ISR register
            pub const Clear: u32 = 0b1;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Overrun/Underrun flag clear
    pub mod OVRCF {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values
        pub mod W {

            /// 0b1: Clears the OVR flag in ISR register
            pub const Clear: u32 = 0b1;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// PEC Error flag clear
    pub mod PECCF {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values
        pub mod W {

            /// 0b1: Clears the PEC flag in ISR register
            pub const Clear: u32 = 0b1;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Timeout detection flag clear
    pub mod TIMOUTCF {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values
        pub mod W {

            /// 0b1: Clears the TIMOUT flag in ISR register
            pub const Clear: u32 = 0b1;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Alert flag clear
    pub mod ALERTCF {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values
        pub mod W {

            /// 0b1: Clears the ALERT flag in ISR register
            pub const Clear: u32 = 0b1;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// PEC register
pub mod PECR {

    /// PEC
    pub mod PEC {
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

/// Receive data register
pub mod RXDR {

    /// RXDATA
    pub mod RXDATA {
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

/// Transmit data register
pub mod TXDR {

    /// TXDATA
    pub mod TXDATA {
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
#[repr(C)]
pub struct RegisterBlock {
    /// Control register 1
    pub CR1: RWRegister<u32>,

    /// Control register 2
    pub CR2: RWRegister<u32>,

    /// Own address register 1
    pub OAR1: RWRegister<u32>,

    /// Own address register 2
    pub OAR2: RWRegister<u32>,

    /// Timing register
    pub TIMINGR: RWRegister<u32>,

    /// Timeout register
    pub TIMEOUTR: RWRegister<u32>,

    /// Interrupt and Status register
    pub ISR: RWRegister<u32>,

    /// Interrupt clear register
    pub ICR: WORegister<u32>,

    /// PEC register
    pub PECR: RORegister<u32>,

    /// Receive data register
    pub RXDR: RORegister<u32>,

    /// Transmit data register
    pub TXDR: RWRegister<u32>,
}
pub struct ResetValues {
    pub CR1: u32,
    pub CR2: u32,
    pub OAR1: u32,
    pub OAR2: u32,
    pub TIMINGR: u32,
    pub TIMEOUTR: u32,
    pub ISR: u32,
    pub ICR: u32,
    pub PECR: u32,
    pub RXDR: u32,
    pub TXDR: u32,
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

/// Access functions for the FMPI2C1 peripheral instance
pub mod FMPI2C1 {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x40006000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in FMPI2C1
    pub const reset: ResetValues = ResetValues {
        CR1: 0x00000000,
        CR2: 0x00000000,
        OAR1: 0x00000000,
        OAR2: 0x00000000,
        TIMINGR: 0x00000000,
        TIMEOUTR: 0x00000000,
        ISR: 0x00000001,
        ICR: 0x00000000,
        PECR: 0x00000000,
        RXDR: 0x00000000,
        TXDR: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut FMPI2C1_TAKEN: bool = false;

    /// Safe access to FMPI2C1
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
            if FMPI2C1_TAKEN {
                None
            } else {
                FMPI2C1_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to FMPI2C1
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if FMPI2C1_TAKEN && inst.addr == INSTANCE.addr {
                FMPI2C1_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal FMPI2C1
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        FMPI2C1_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to FMPI2C1
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const FMPI2C1: *const RegisterBlock = 0x40006000 as *const _;
