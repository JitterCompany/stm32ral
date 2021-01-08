#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Ethernet: media access control

use crate::RWRegister;
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// Ethernet MAC configuration register (ETH_MACCR)
pub mod MACCR {

    /// Receiver enable
    pub mod RE {
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

            /// 0b0: MAC receive state machine is disabled after the completion of the reception of the current frame
            pub const Disabled: u32 = 0b0;

            /// 0b1: MAC receive state machine is enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Transmitter enable
    pub mod TE {
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

            /// 0b0: MAC transmit state machine is disabled after completion of the transmission of the current frame
            pub const Disabled: u32 = 0b0;

            /// 0b1: MAC transmit state machine is enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Deferral check
    pub mod DC {
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

            /// 0b0: MAC defers until CRS signal goes inactive
            pub const Disabled: u32 = 0b0;

            /// 0b1: Deferral check function enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Back-off limit
    pub mod BL {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (2 bits: 0b11 << 5)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: For retransmission n, wait up to 2^min(n, 10) time slots
            pub const BL10: u32 = 0b00;

            /// 0b01: For retransmission n, wait up to 2^min(n, 8) time slots
            pub const BL8: u32 = 0b01;

            /// 0b10: For retransmission n, wait up to 2^min(n, 4) time slots
            pub const BL4: u32 = 0b10;

            /// 0b11: For retransmission n, wait up to 2^min(n, 1) time slots
            pub const BL1: u32 = 0b11;
        }
    }

    /// Automatic pad/CRC stripping
    pub mod APCS {
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

            /// 0b0: MAC passes all incoming frames unmodified
            pub const Disabled: u32 = 0b0;

            /// 0b1: MAC strips the Pad/FCS field on incoming frames only for lengths less than or equal to 1500 bytes
            pub const Strip: u32 = 0b1;
        }
    }

    /// Retry disable
    pub mod RD {
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

            /// 0b0: MAC attempts retries based on the settings of BL
            pub const Enabled: u32 = 0b0;

            /// 0b1: MAC attempts only 1 transmission
            pub const Disabled: u32 = 0b1;
        }
    }

    /// IPv4 checksum offload
    pub mod IPCO {
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

            /// 0b0: IPv4 checksum offload disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: IPv4 checksums are checked in received frames
            pub const Offload: u32 = 0b1;
        }
    }

    /// Duplex mode
    pub mod DM {
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

            /// 0b0: MAC operates in half-duplex mode
            pub const HalfDuplex: u32 = 0b0;

            /// 0b1: MAC operates in full-duplex mode
            pub const FullDuplex: u32 = 0b1;
        }
    }

    /// Loopback mode
    pub mod LM {
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

            /// 0b0: Normal mode
            pub const Normal: u32 = 0b0;

            /// 0b1: MAC operates in loopback mode at the MII
            pub const Loopback: u32 = 0b1;
        }
    }

    /// Receive own disable
    pub mod ROD {
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

            /// 0b0: MAC receives all packets from PHY while transmitting
            pub const Enabled: u32 = 0b0;

            /// 0b1: MAC disables reception of frames in half-duplex mode
            pub const Disabled: u32 = 0b1;
        }
    }

    /// Fast Ethernet speed
    pub mod FES {
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

            /// 0b0: 10 Mbit/s
            pub const FES10: u32 = 0b0;

            /// 0b1: 100 Mbit/s
            pub const FES100: u32 = 0b1;
        }
    }

    /// Carrier sense disable
    pub mod CSD {
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

            /// 0b0: Errors generated due to loss of carrier
            pub const Enabled: u32 = 0b0;

            /// 0b1: No error generated due to loss of carrier
            pub const Disabled: u32 = 0b1;
        }
    }

    /// Interframe gap
    pub mod IFG {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (3 bits: 0b111 << 17)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: 96 bit times
            pub const IFG96: u32 = 0b000;

            /// 0b001: 88 bit times
            pub const IFG88: u32 = 0b001;

            /// 0b110: 48 bit times
            pub const IFG80: u32 = 0b110;

            /// 0b111: 40 bit times
            pub const IFG40: u32 = 0b111;
        }
    }

    /// Jabber disable
    pub mod JD {
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

            /// 0b0: Jabber enabled, transmit frames up to 2048 bytes
            pub const Enabled: u32 = 0b0;

            /// 0b1: Jabber disabled, transmit frames up to 16384 bytes
            pub const Disabled: u32 = 0b1;
        }
    }

    /// Watchdog disable
    pub mod WD {
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

            /// 0b0: Watchdog enabled, receive frames limited to 2048 bytes
            pub const Enabled: u32 = 0b0;

            /// 0b1: Watchdog disabled, receive frames may be up to to 16384 bytes
            pub const Disabled: u32 = 0b1;
        }
    }
}

/// Ethernet MAC frame filter register (ETH_MACCFFR)
pub mod MACFFR {

    /// Promiscuous mode
    pub mod PM {
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

            /// 0b0: Normal address filtering
            pub const Disabled: u32 = 0b0;

            /// 0b1: Address filters pass all incoming frames regardless of their destination or source address
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Hash unicast
    pub mod HU {
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

            /// 0b0: MAC performs a perfect destination address filtering for unicast frames
            pub const Perfect: u32 = 0b0;

            /// 0b1: MAC performs destination address filtering of received unicast frames according to the hash table
            pub const Hash: u32 = 0b1;
        }
    }

    /// Hash multicast
    pub mod HM {
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

            /// 0b0: MAC performs a perfect destination address filtering for multicast frames
            pub const Perfect: u32 = 0b0;

            /// 0b1: MAC performs destination address filtering of received multicast frames according to the hash table
            pub const Hash: u32 = 0b1;
        }
    }

    /// Destination address unique filtering
    pub mod DAIF {
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

            /// 0b0: Normal filtering of frames
            pub const Normal: u32 = 0b0;

            /// 0b1: Address check block operates in inverse filtering mode for the DA address comparison
            pub const Invert: u32 = 0b1;
        }
    }

    /// Pass all multicast
    pub mod PAM {
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

            /// 0b0: Filtering of multicast frames depends on HM
            pub const Disabled: u32 = 0b0;

            /// 0b1: All received frames with a multicast destination address are passed
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Broadcast frames disable
    pub mod BFD {
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

            /// 0b0: Address filters pass all received broadcast frames
            pub const Enabled: u32 = 0b0;

            /// 0b1: Address filters filter all incoming broadcast frames
            pub const Disabled: u32 = 0b1;
        }
    }

    /// Pass control frames
    pub mod PCF {
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

            /// 0b00: MAC prevents all control frames from reaching the application
            pub const PreventAll: u32 = 0b00;

            /// 0b01: MAC forwards all control frames to application except Pause
            pub const ForwardAllExceptPause: u32 = 0b01;

            /// 0b10: MAC forwards all control frames to application even if they fail the address filter
            pub const ForwardAll: u32 = 0b10;

            /// 0b11: MAC forwards control frames that pass the address filter
            pub const ForwardAllFiltered: u32 = 0b11;
        }
    }

    /// Source address inverse filtering
    pub mod SAIF {
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

            /// 0b0: Source address filter operates normally
            pub const Normal: u32 = 0b0;

            /// 0b1: Source address filter operation inverted
            pub const Invert: u32 = 0b1;
        }
    }

    /// Source address filter
    pub mod SAF {
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

            /// 0b0: Source address ignored
            pub const Disabled: u32 = 0b0;

            /// 0b1: MAC drops frames that fail the source address filter
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Hash or perfect filter
    pub mod HPF {
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

            /// 0b0: If HM or HU is set, only frames that match the Hash filter are passed
            pub const HashOnly: u32 = 0b0;

            /// 0b1: If HM or HU is set, frames that match either the perfect filter or the hash filter are passed
            pub const HashOrPerfect: u32 = 0b1;
        }
    }

    /// Receive all
    pub mod RA {
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

            /// 0b0: MAC receiver passes on to the application only those frames that have passed the SA/DA address file
            pub const Disabled: u32 = 0b0;

            /// 0b1: MAC receiver passes oll received frames on to the application
            pub const Enabled: u32 = 0b1;
        }
    }
}

/// Ethernet MAC hash table high register
pub mod MACHTHR {

    /// Upper 32 bits of hash table
    pub mod HTH {
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

/// Ethernet MAC hash table low register
pub mod MACHTLR {

    /// Lower 32 bits of hash table
    pub mod HTL {
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

/// Ethernet MAC MII address register (ETH_MACMIIAR)
pub mod MACMIIAR {

    /// MII busy
    pub mod MB {
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

            /// 0b1: This bit is set to 1 by the application to indicate that a read or write access is in progress
            pub const Busy: u32 = 0b1;
        }
    }

    /// MII write
    pub mod MW {
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

            /// 0b0: Read operation
            pub const Read: u32 = 0b0;

            /// 0b1: Write operation
            pub const Write: u32 = 0b1;
        }
    }

    /// Clock range
    pub mod CR {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (3 bits: 0b111 << 2)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: 60-100MHz HCLK/42
            pub const CR_60_100: u32 = 0b000;

            /// 0b001: 100-150 MHz HCLK/62
            pub const CR_100_150: u32 = 0b001;

            /// 0b010: 20-35MHz HCLK/16
            pub const CR_20_35: u32 = 0b010;

            /// 0b011: 35-60MHz HCLK/16
            pub const CR_35_60: u32 = 0b011;

            /// 0b100: 150-168MHz HCLK/102
            pub const CR_150_168: u32 = 0b100;
        }
    }

    /// MII register - select the desired MII register in the PHY device
    pub mod MR {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (5 bits: 0b11111 << 6)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// PHY address - select which of possible 32 PHYs is being accessed
    pub mod PA {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (5 bits: 0b11111 << 11)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Ethernet MAC MII data register (ETH_MACMIIDR)
pub mod MACMIIDR {

    /// MII data read from/written to the PHY
    pub mod MD {
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

/// Ethernet MAC flow control register (ETH_MACFCR)
pub mod MACFCR {

    /// Flow control busy/back pressure activate
    pub mod FCB {
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

            /// 0b1: In full duplex, initiate a Pause control frame. In half duplex, assert back pressure
            pub const PauseOrBackPressure: u32 = 0b1;

            /// 0b0: In half duplex only, deasserts back pressure
            pub const DisableBackPressure: u32 = 0b0;
        }
    }

    /// Transmit flow control enable
    pub mod TFCE {
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

            /// 0b0: In full duplex, flow control is disabled. In half duplex, back pressure is disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: In full duplex, flow control is enabled. In half duplex, back pressure is enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Receive flow control enable
    pub mod RFCE {
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

            /// 0b0: Pause frames are not decoded
            pub const Disabled: u32 = 0b0;

            /// 0b1: MAC decodes received Pause frames and disables its transmitted for a specified time
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Unicast pause frame detect
    pub mod UPFD {
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

            /// 0b0: MAC detects only a Pause frame with the multicast address specified in the 802.3x standard
            pub const Disabled: u32 = 0b0;

            /// 0b1: MAC additionally detects Pause frames with the station's unicast address
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Pause low threshold
    pub mod PLT {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (2 bits: 0b11 << 4)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Pause time minus 4 slot times
            pub const PLT4: u32 = 0b00;

            /// 0b01: Pause time minus 28 slot times
            pub const PLT28: u32 = 0b01;

            /// 0b10: Pause time minus 144 slot times
            pub const PLT144: u32 = 0b10;

            /// 0b11: Pause time minus 256 slot times
            pub const PLT256: u32 = 0b11;
        }
    }

    /// Zero-quanta pause disable
    pub mod ZQPD {
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

            /// 0b0: Normal operation with automatic zero-quanta pause control frame generation
            pub const Enabled: u32 = 0b0;

            /// 0b1: Automatic generation of zero-quanta pause control frames is disabled
            pub const Disabled: u32 = 0b1;
        }
    }

    /// Pause time
    pub mod PT {
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
}

/// Ethernet MAC VLAN tag register (ETH_MACVLANTR)
pub mod MACVLANTR {

    /// VLAN tag identifier (for receive frames)
    pub mod VLANTI {
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

    /// 12-bit VLAN tag comparison
    pub mod VLANTC {
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

            /// 0b0: Full 16 bit VLAN identifiers are used for comparison and filtering
            pub const VLANTC16: u32 = 0b0;

            /// 0b1: 12 bit VLAN identifies are used for comparison and filtering
            pub const VLANTC12: u32 = 0b1;
        }
    }
}

/// Ethernet MAC remote wakeup frame filter register (ETH_MACRWUFFR)
pub mod MACRWUFFR {}

/// Ethernet MAC PMT control and status register (ETH_MACPMTCSR)
pub mod MACPMTCSR {

    /// Power down
    pub mod PD {
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

            /// 0b1: All received frames will be dropped. Cleared automatically when a magic packet or wakeup frame is received
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Magic packet enable
    pub mod MPE {
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

            /// 0b0: No power management event generated due to Magic Packet reception
            pub const Disabled: u32 = 0b0;

            /// 0b1: Enable generation of a power management event due to Magic Packet reception
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Wakeup frame enable
    pub mod WFE {
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

            /// 0b0: No power management event generated due to wakeup frame reception
            pub const Disabled: u32 = 0b0;

            /// 0b1: Enable generation of a power management event due to wakeup frame reception
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Magic packet received
    pub mod MPR {
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

    /// Wakeup frame received
    pub mod WFR {
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

    /// Global unicast
    pub mod GU {
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

            /// 0b0: Normal operation
            pub const Disabled: u32 = 0b0;

            /// 0b1: Any unicast packet filtered by the MAC address recognition may be a wakeup frame
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Wakeup frame filter register pointer reset
    pub mod WFFRPR {
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

            /// 0b1: Reset wakeup frame filter register point to 0b000. Automatically cleared
            pub const Reset: u32 = 0b1;
        }
    }
}

/// Ethernet MAC interrupt status register (ETH_MACSR)
pub mod MACSR {

    /// PMT status
    pub mod PMTS {
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

    /// MMC status
    pub mod MMCS {
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

    /// MMC receive status
    pub mod MMCRS {
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

    /// MMC transmit status
    pub mod MMCTS {
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

    /// Time stamp trigger status
    pub mod TSTS {
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
}

/// Ethernet MAC interrupt mask register (ETH_MACIMR)
pub mod MACIMR {

    /// PMT interrupt mask
    pub mod PMTIM {
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

            /// 0b0: PMT Status interrupt generation enabled
            pub const Unmasked: u32 = 0b0;

            /// 0b1: PMT Status interrupt generation disabled
            pub const Masked: u32 = 0b1;
        }
    }

    /// Time stamp trigger interrupt mask
    pub mod TSTIM {
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

            /// 0b0: Time stamp interrupt generation enabled
            pub const Unmasked: u32 = 0b0;

            /// 0b1: Time stamp interrupt generation disabled
            pub const Masked: u32 = 0b1;
        }
    }
}

/// Ethernet MAC address 0 high register (ETH_MACA0HR)
pub mod MACA0HR {

    /// MAC address0 high
    pub mod MACA0H {
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

    /// Always 1
    pub mod MO {
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

/// Ethernet MAC address 0 low register
pub mod MACA0LR {

    /// MAC address0 low
    pub mod MACA0L {
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

/// Ethernet MAC address 1 high register (ETH_MACA1HR)
pub mod MACA1HR {

    /// MAC address1 high
    pub mod MACA1H {
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

    /// Mask byte control
    pub mod MBC {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (6 bits: 0x3f << 24)
        pub const mask: u32 = 0x3f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Source address
    pub mod SA {
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

            /// 0b0: This address is used for comparison with DA fields of the received frame
            pub const Destination: u32 = 0b0;

            /// 0b1: This address is used for comparison with SA fields of received frames
            pub const Source: u32 = 0b1;
        }
    }

    /// Address enable
    pub mod AE {
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

            /// 0b0: Address filters ignore this address
            pub const Disabled: u32 = 0b0;

            /// 0b1: Address filters use this address
            pub const Enabled: u32 = 0b1;
        }
    }
}

/// Ethernet MAC address1 low register
pub mod MACA1LR {

    /// MAC address1 low
    pub mod MACA1L {
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

/// Ethernet MAC address 2 high register (ETH_MACA2HR)
pub mod MACA2HR {

    /// Ethernet MAC address 2 high register
    pub mod MACA2H {
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

    /// Mask byte control
    pub mod MBC {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (6 bits: 0x3f << 24)
        pub const mask: u32 = 0x3f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Source address
    pub mod SA {
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

            /// 0b0: This address is used for comparison with DA fields of the received frame
            pub const Destination: u32 = 0b0;

            /// 0b1: This address is used for comparison with SA fields of received frames
            pub const Source: u32 = 0b1;
        }
    }

    /// Address enable
    pub mod AE {
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

            /// 0b0: Address filters ignore this address
            pub const Disabled: u32 = 0b0;

            /// 0b1: Address filters use this address
            pub const Enabled: u32 = 0b1;
        }
    }
}

/// Ethernet MAC address 2 low register
pub mod MACA2LR {

    /// MAC address2 low
    pub mod MACA2L {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (31 bits: 0x7fffffff << 0)
        pub const mask: u32 = 0x7fffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Ethernet MAC address 3 high register (ETH_MACA3HR)
pub mod MACA3HR {

    /// MAC address3 high
    pub mod MACA3H {
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

    /// Mask byte control
    pub mod MBC {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (6 bits: 0x3f << 24)
        pub const mask: u32 = 0x3f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Source address
    pub mod SA {
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

            /// 0b0: This address is used for comparison with DA fields of the received frame
            pub const Destination: u32 = 0b0;

            /// 0b1: This address is used for comparison with SA fields of received frames
            pub const Source: u32 = 0b1;
        }
    }

    /// Address enable
    pub mod AE {
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

            /// 0b0: Address filters ignore this address
            pub const Disabled: u32 = 0b0;

            /// 0b1: Address filters use this address
            pub const Enabled: u32 = 0b1;
        }
    }
}

/// Ethernet MAC address 3 low register
pub mod MACA3LR {

    /// MAC address3 low
    pub mod MACA3L {
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
#[repr(C)]
pub struct RegisterBlock {
    /// Ethernet MAC configuration register (ETH_MACCR)
    pub MACCR: RWRegister<u32>,

    /// Ethernet MAC frame filter register (ETH_MACCFFR)
    pub MACFFR: RWRegister<u32>,

    /// Ethernet MAC hash table high register
    pub MACHTHR: RWRegister<u32>,

    /// Ethernet MAC hash table low register
    pub MACHTLR: RWRegister<u32>,

    /// Ethernet MAC MII address register (ETH_MACMIIAR)
    pub MACMIIAR: RWRegister<u32>,

    /// Ethernet MAC MII data register (ETH_MACMIIDR)
    pub MACMIIDR: RWRegister<u32>,

    /// Ethernet MAC flow control register (ETH_MACFCR)
    pub MACFCR: RWRegister<u32>,

    /// Ethernet MAC VLAN tag register (ETH_MACVLANTR)
    pub MACVLANTR: RWRegister<u32>,

    _reserved1: [u32; 2],

    /// Ethernet MAC remote wakeup frame filter register (ETH_MACRWUFFR)
    pub MACRWUFFR: RWRegister<u32>,

    /// Ethernet MAC PMT control and status register (ETH_MACPMTCSR)
    pub MACPMTCSR: RWRegister<u32>,

    _reserved2: [u32; 2],

    /// Ethernet MAC interrupt status register (ETH_MACSR)
    pub MACSR: RWRegister<u32>,

    /// Ethernet MAC interrupt mask register (ETH_MACIMR)
    pub MACIMR: RWRegister<u32>,

    /// Ethernet MAC address 0 high register (ETH_MACA0HR)
    pub MACA0HR: RWRegister<u32>,

    /// Ethernet MAC address 0 low register
    pub MACA0LR: RWRegister<u32>,

    /// Ethernet MAC address 1 high register (ETH_MACA1HR)
    pub MACA1HR: RWRegister<u32>,

    /// Ethernet MAC address1 low register
    pub MACA1LR: RWRegister<u32>,

    /// Ethernet MAC address 2 high register (ETH_MACA2HR)
    pub MACA2HR: RWRegister<u32>,

    /// Ethernet MAC address 2 low register
    pub MACA2LR: RWRegister<u32>,

    /// Ethernet MAC address 3 high register (ETH_MACA3HR)
    pub MACA3HR: RWRegister<u32>,

    /// Ethernet MAC address 3 low register
    pub MACA3LR: RWRegister<u32>,
}
pub struct ResetValues {
    pub MACCR: u32,
    pub MACFFR: u32,
    pub MACHTHR: u32,
    pub MACHTLR: u32,
    pub MACMIIAR: u32,
    pub MACMIIDR: u32,
    pub MACFCR: u32,
    pub MACVLANTR: u32,
    pub MACRWUFFR: u32,
    pub MACPMTCSR: u32,
    pub MACSR: u32,
    pub MACIMR: u32,
    pub MACA0HR: u32,
    pub MACA0LR: u32,
    pub MACA1HR: u32,
    pub MACA1LR: u32,
    pub MACA2HR: u32,
    pub MACA2LR: u32,
    pub MACA3HR: u32,
    pub MACA3LR: u32,
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

/// Access functions for the Ethernet_MAC peripheral instance
pub mod Ethernet_MAC {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x40028000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in Ethernet_MAC
    pub const reset: ResetValues = ResetValues {
        MACCR: 0x00008000,
        MACFFR: 0x00000000,
        MACHTHR: 0x00000000,
        MACHTLR: 0x00000000,
        MACMIIAR: 0x00000000,
        MACMIIDR: 0x00000000,
        MACFCR: 0x00000000,
        MACVLANTR: 0x00000000,
        MACRWUFFR: 0x00000000,
        MACPMTCSR: 0x00000000,
        MACSR: 0x00000000,
        MACIMR: 0x00000000,
        MACA0HR: 0x0010FFFF,
        MACA0LR: 0xFFFFFFFF,
        MACA1HR: 0x0000FFFF,
        MACA1LR: 0xFFFFFFFF,
        MACA2HR: 0x00000050,
        MACA2LR: 0xFFFFFFFF,
        MACA3HR: 0x0000FFFF,
        MACA3LR: 0xFFFFFFFF,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut Ethernet_MAC_TAKEN: bool = false;

    /// Safe access to Ethernet_MAC
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
            if Ethernet_MAC_TAKEN {
                None
            } else {
                Ethernet_MAC_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to Ethernet_MAC
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if Ethernet_MAC_TAKEN && inst.addr == INSTANCE.addr {
                Ethernet_MAC_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal Ethernet_MAC
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        Ethernet_MAC_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to Ethernet_MAC
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const Ethernet_MAC: *const RegisterBlock = 0x40028000 as *const _;
