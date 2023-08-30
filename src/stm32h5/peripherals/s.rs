#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Serial audio interface
//!
//! Used by: stm32h562, stm32h563, stm32h573

use crate::{RORegister, RWRegister, WORegister};
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// SAI global configuration register
pub mod GCR {

    /// Synchronization inputs These bits are set and cleared by software. Refer to for information on how to program this field. These bits must be set when both audio blocks (A and B) are disabled. They are meaningful if one of the two audio blocks is defined to operate in synchronous mode with an external SAI (SYNCEN\[1:0\] = 10 in SAI_ACR1 or in SAI_BCR1 registers).
    pub mod SYNCIN {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (2 bits: 0b11 << 0)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Synchronization outputs These bits are set and cleared by software.
    pub mod SYNCOUT {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (2 bits: 0b11 << 4)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// SAI configuration register 1
pub mod ACR1 {

    /// SAIx audio block mode These bits are set and cleared by software. They must be configured when SAIx audio block is disabled. Note: When the audio block is configured in SPDIF mode, the master transmitter mode is forced (MODE\[1:0\] = 00).
    pub mod MODE {
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

            /// 0b00: Master transmitter
            pub const MasterTx: u32 = 0b00;

            /// 0b01: Master receiver
            pub const MasterRx: u32 = 0b01;

            /// 0b10: Slave transmitter
            pub const SlaveTx: u32 = 0b10;

            /// 0b11: Slave receiver
            pub const SlaveRx: u32 = 0b11;
        }
    }

    /// Protocol configuration These bits are set and cleared by software. These bits have to be configured when the audio block is disabled.
    pub mod PRTCFG {
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

            /// 0b00: Free protocol. Free protocol allows to use the powerful configuration of the audio block to address a specific audio protocol
            pub const Free: u32 = 0b00;

            /// 0b01: SPDIF protocol
            pub const Spdif: u32 = 0b01;

            /// 0b10: AC’97 protocol
            pub const Ac97: u32 = 0b10;
        }
    }

    /// Data size These bits are set and cleared by software. These bits are ignored when the SPDIF protocols are selected (bit PRTCFG\[1:0\]), because the frame and the data size are fixed in such case. When the companding mode is selected through COMP\[1:0\] bits, DS\[1:0\] are ignored since the data size is fixed to 8 bits by the algorithm. These bits must be configured when the audio block is disabled.
    pub mod DS {
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

            /// 0b010: 8 bits
            pub const Bit8: u32 = 0b010;

            /// 0b011: 10 bits
            pub const Bit10: u32 = 0b011;

            /// 0b100: 16 bits
            pub const Bit16: u32 = 0b100;

            /// 0b101: 20 bits
            pub const Bit20: u32 = 0b101;

            /// 0b110: 24 bits
            pub const Bit24: u32 = 0b110;

            /// 0b111: 32 bits
            pub const Bit32: u32 = 0b111;
        }
    }

    /// Least significant bit first This bit is set and cleared by software. It must be configured when the audio block is disabled. This bit has no meaning in AC’97 audio protocol since AC’97 data are always transferred with the MSB first. This bit has no meaning in SPDIF audio protocol since in SPDIF data are always transferred with LSB first.
    pub mod LSBFIRST {
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

            /// 0b0: Data are transferred with MSB first
            pub const MsbFirst: u32 = 0b0;

            /// 0b1: Data are transferred with LSB first
            pub const LsbFirst: u32 = 0b1;
        }
    }

    /// Clock strobing edge This bit is set and cleared by software. It must be configured when the audio block is disabled. This bit has no meaning in SPDIF audio protocol.
    pub mod CKSTR {
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

            /// 0b0: Data strobing edge is falling edge of SCK
            pub const FallingEdge: u32 = 0b0;

            /// 0b1: Data strobing edge is rising edge of SCK
            pub const RisingEdge: u32 = 0b1;
        }
    }

    /// Synchronization enable These bits are set and cleared by software. They must be configured when the audio subblock is disabled. Note: The audio subblock should be configured as asynchronous when SPDIF mode is enabled.
    pub mod SYNCEN {
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

            /// 0b00: audio sub-block in asynchronous mode
            pub const Asynchronous: u32 = 0b00;

            /// 0b01: audio sub-block is synchronous with the other internal audio sub-block. In this case, the audio sub-block must be configured in slave mode
            pub const Internal: u32 = 0b01;

            /// 0b10: audio sub-block is synchronous with an external SAI embedded peripheral. In this case the audio sub-block should be configured in Slave mode
            pub const External: u32 = 0b10;
        }
    }

    /// Mono mode This bit is set and cleared by software. It is meaningful only when the number of slots is equal to 2. When the mono mode is selected, slot 0 data are duplicated on slot 1 when the audio block operates as a transmitter. In reception mode, the slot1 is discarded and only the data received from slot 0 are stored. Refer to for more details.
    pub mod MONO {
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

            /// 0b0: Stereo mode
            pub const Stereo: u32 = 0b0;

            /// 0b1: Mono mode
            pub const Mono: u32 = 0b1;
        }
    }

    /// Output drive This bit is set and cleared by software. Note: This bit has to be set before enabling the audio block and after the audio block configuration.
    pub mod OUTDRIV {
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

            /// 0b0: Audio block output driven when SAIEN is set
            pub const OnStart: u32 = 0b0;

            /// 0b1: Audio block output driven immediately after the setting of this bit
            pub const Immediately: u32 = 0b1;
        }
    }

    /// Audio block enable This bit is set by software. To switch off the audio block, the application software must program this bit to 0 and poll the bit till it reads back 0, meaning that the block is completely disabled. Before setting this bit to 1, check that it is set to 0, otherwise the enable command is not taken into account. This bit allows controlling the state of the SAI audio block. If it is disabled when an audio frame transfer is ongoing, the ongoing transfer completes and the cell is fully disabled at the end of this audio frame transfer. Note: When the SAI block (A or B) is configured in master mode, the clock must be present on the SAI block input before setting SAIEN bit.
    pub mod SAIEN {
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

            /// 0b0: SAI audio block disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: SAI audio block enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// DMA enable This bit is set and cleared by software. Note: Since the audio block defaults to operate as a transmitter after reset, the MODE\[1:0\] bits must be configured before setting DMAEN to avoid a DMA request in receiver mode.
    pub mod DMAEN {
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

            /// 0b0: DMA disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: DMA enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// No divider This bit is set and cleared by software.
    pub mod NODIV {
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

            /// 0b0: MCLK output is enabled. Forces the ratio between FS and MCLK to 256 or 512 according to the OSR value
            pub const MasterClock: u32 = 0b0;

            /// 0b1: MCLK output enable set by the MCKEN bit (where present, else 0). Ratio between FS and MCLK depends on FRL.
            pub const NoDiv: u32 = 0b1;
        }
    }

    /// Master clock divider These bits are set and cleared by software. Otherwise, The master clock frequency is calculated according to the formula given in . These bits have no meaning when the audio block is slave. They have to be configured when the audio block is disabled.
    pub mod MCKDIV {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (6 bits: 0x3f << 20)
        pub const mask: u32 = 0x3f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Oversampling ratio for master clock This bit is meaningful only when NODIV bit is set to 0.
    pub mod OSR {
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

    /// Master clock generation enable
    pub mod MCKEN {
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
}

/// SAI configuration register 2
pub mod ACR2 {

    /// FIFO threshold. This bit is set and cleared by software.
    pub mod FTH {
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

            /// 0b000: FIFO empty
            pub const Empty: u32 = 0b000;

            /// 0b001: 1⁄4 FIFO
            pub const Quarter1: u32 = 0b001;

            /// 0b010: 1⁄2 FIFO
            pub const Quarter2: u32 = 0b010;

            /// 0b011: 3⁄4 FIFO
            pub const Quarter3: u32 = 0b011;

            /// 0b100: FIFO full
            pub const Full: u32 = 0b100;
        }
    }

    /// FIFO flush. This bit is set by software. It is always read as 0. This bit should be configured when the SAI is disabled.
    pub mod FFLUSH {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values
        pub mod W {

            /// 0b0: No FIFO flush
            pub const NoFlush: u32 = 0b0;

            /// 0b1: FIFO flush. Programming this bit to 1 triggers the FIFO Flush. All the internal FIFO pointers (read and write) are cleared
            pub const Flush: u32 = 0b1;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Tristate management on data line. This bit is set and cleared by software. It is meaningful only if the audio block is configured as a transmitter. This bit is not used when the audio block is configured in SPDIF mode. It should be configured when SAI is disabled. Refer to for more details.
    pub mod TRIS {
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

    /// Mute. This bit is set and cleared by software. It is meaningful only when the audio block operates as a transmitter. The MUTE value is linked to value of MUTEVAL if the number of slots is lower or equal to 2, or equal to 0 if it is greater than 2. Refer to for more details. Note: This bit is meaningless and should not be used for SPDIF audio blocks.
    pub mod MUTE {
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

            /// 0b0: No mute mode
            pub const Disabled: u32 = 0b0;

            /// 0b1: Mute mode enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Mute value. This bit is set and cleared by software.It must be written before enabling the audio block: SAIEN. This bit is meaningful only when the audio block operates as a transmitter, the number of slots is lower or equal to 2 and the MUTE bit is set. If more slots are declared, the bit value sent during the transmission in mute mode is equal to 0, whatever the value of MUTEVAL. if the number of slot is lower or equal to 2 and MUTEVAL = 1, the MUTE value transmitted for each slot is the one sent during the previous frame. Refer to for more details. Note: This bit is meaningless and should not be used for SPDIF audio blocks.
    pub mod MUTEVAL {
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

            /// 0b0: Bit value 0 is sent during the mute mode
            pub const SendZero: u32 = 0b0;

            /// 0b1: Last values are sent during the mute mode
            pub const SendLast: u32 = 0b1;
        }
    }

    /// Mute counter. These bits are set and cleared by software. They are used only in reception mode. The value set in these bits is compared to the number of consecutive mute frames detected in reception. When the number of mute frames is equal to this value, the flag MUTEDET is set and an interrupt is generated if bit MUTEDETIE is set. Refer to for more details.
    pub mod MUTECNT {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (6 bits: 0x3f << 7)
        pub const mask: u32 = 0x3f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Complement bit. This bit is set and cleared by software. It defines the type of complement to be used for companding mode Note: This bit has effect only when the companding mode is �-Law algorithm or A-Law algorithm.
    pub mod CPL {
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

            /// 0b0: 1’s complement representation
            pub const OnesComplement: u32 = 0b0;

            /// 0b1: 2’s complement representation
            pub const TwosComplement: u32 = 0b1;
        }
    }

    /// Companding mode. These bits are set and cleared by software. The �-Law and the A-Law log are a part of the CCITT G.711 recommendation, the type of complement that is used depends on CPL bit. The data expansion or data compression are determined by the state of bit MODE\[0\]. The data compression is applied if the audio block is configured as a transmitter. The data expansion is automatically applied when the audio block is configured as a receiver. Refer to for more details. Note: Companding mode is applicable only when Free protocol mode is selected.
    pub mod COMP {
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

            /// 0b00: No companding algorithm
            pub const NoCompanding: u32 = 0b00;

            /// 0b10: μ-Law algorithm
            pub const MuLaw: u32 = 0b10;

            /// 0b11: A-Law algorithm
            pub const ALaw: u32 = 0b11;
        }
    }
}

/// SAI frame configuration register
pub mod AFRCR {

    /// Frame length. These bits are set and cleared by software. They define the audio frame length expressed in number of SCK clock cycles: the number of bits in the frame is equal to FRL\[7:0\] + 1. The minimum number of bits to transfer in an audio frame must be equal to 8, otherwise the audio block behaves in an unexpected way. This is the case when the data size is 8 bits and only one slot 0 is defined in NBSLOT\[4:0\] of SAI_xSLOTR register (NBSLOT\[3:0\] = 0000). In master mode, if the master clock (available on MCLK_x pin) is used, the frame length should be aligned with a number equal to a power of 2, ranging from 8 to 256. When the master clock is not used (NODIV = 1), it is recommended to program the frame length to an value ranging from 8 to 256. These bits are meaningless and are not used in AC’97 or SPDIF audio block configuration. They must be configured when the audio block is disabled.
    pub mod FRL {
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

    /// Frame synchronization active level length. These bits are set and cleared by software. They specify the length in number of bit clock (SCK) + 1 (FSALL\[6:0\] + 1) of the active level of the FS signal in the audio frame These bits are meaningless and are not used in AC’97 or SPDIF audio block configuration. They must be configured when the audio block is disabled.
    pub mod FSALL {
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

    /// Frame synchronization definition. This bit is set and cleared by software. When the bit is set, the number of slots defined in the SAI_xSLOTR register has to be even. It means that half of this number of slots are dedicated to the left channel and the other slots for the right channel (e.g: this bit has to be set for I2S or MSB/LSB-justified protocols...). This bit is meaningless and is not used in AC’97 or SPDIF audio block configuration. It must be configured when the audio block is disabled.
    pub mod FSDEF {
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

    /// Frame synchronization polarity. This bit is set and cleared by software. It is used to configure the level of the start of frame on the FS signal. It is meaningless and is not used in AC’97 or SPDIF audio block configuration. This bit must be configured when the audio block is disabled.
    pub mod FSPOL {
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

            /// 0b0: FS is active low (falling edge)
            pub const FallingEdge: u32 = 0b0;

            /// 0b1: FS is active high (rising edge)
            pub const RisingEdge: u32 = 0b1;
        }
    }

    /// Frame synchronization offset. This bit is set and cleared by software. It is meaningless and is not used in AC’97 or SPDIF audio block configuration. This bit must be configured when the audio block is disabled.
    pub mod FSOFF {
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

            /// 0b0: FS is asserted on the first bit of the slot 0
            pub const OnFirst: u32 = 0b0;

            /// 0b1: FS is asserted one bit before the first bit of the slot 0
            pub const BeforeFirst: u32 = 0b1;
        }
    }
}

/// SAI slot register
pub mod ASLOTR {

    /// First bit offset These bits are set and cleared by software. The value set in this bitfield defines the position of the first data transfer bit in the slot. It represents an offset value. In transmission mode, the bits outside the data field are forced to 0. In reception mode, the extra received bits are discarded. These bits must be set when the audio block is disabled. They are ignored in AC’97 or SPDIF mode.
    pub mod FBOFF {
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

    /// Slot size This bits is set and cleared by software. The slot size must be higher or equal to the data size. If this condition is not respected, the behavior of the SAI is undetermined. Refer to for information on how to drive SD line. These bits must be set when the audio block is disabled. They are ignored in AC’97 or SPDIF mode.
    pub mod SLOTSZ {
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

            /// 0b00: The slot size is equivalent to the data size (specified in DS\[3:0\] in the SAI_xCR1 register)
            pub const DataSize: u32 = 0b00;

            /// 0b01: 16-bit
            pub const Bit16: u32 = 0b01;

            /// 0b10: 32-bit
            pub const Bit32: u32 = 0b10;
        }
    }

    /// Number of slots in an audio frame. These bits are set and cleared by software. The value set in this bitfield represents the number of slots + 1 in the audio frame (including the number of inactive slots). The maximum number of slots is 16. The number of slots should be even if FSDEF bit in the SAI_xFRCR register is set. The number of slots must be configured when the audio block is disabled. They are ignored in AC’97 or SPDIF mode.
    pub mod NBSLOT {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (4 bits: 0b1111 << 8)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Slot enable. These bits are set and cleared by software. Each SLOTEN bit corresponds to a slot position from 0 to 15 (maximum 16 slots). The slot must be enabled when the audio block is disabled. They are ignored in AC’97 or SPDIF mode.
    pub mod SLOTEN {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (16 bits: 0xffff << 16)
        pub const mask: u32 = 0xffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000000000000000: Inactive slot
            pub const Inactive: u32 = 0b0000000000000000;

            /// 0b0000000000000001: Active slot
            pub const Active: u32 = 0b0000000000000001;
        }
    }
}

/// SAI interrupt mask register
pub mod AIM {

    /// Overrun/underrun interrupt enable. This bit is set and cleared by software. When this bit is set, an interrupt is generated if the OVRUDR bit in the SAI_xSR register is set.
    pub mod OVRUDRIE {
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

            /// 0b0: Interrupt is disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Interrupt is enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Mute detection interrupt enable. This bit is set and cleared by software. When this bit is set, an interrupt is generated if the MUTEDET bit in the SAI_xSR register is set. This bit has a meaning only if the audio block is configured in receiver mode.
    pub mod MUTEDETIE {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::OVRUDRIE::RW;
    }

    /// Wrong clock configuration interrupt enable. This bit is set and cleared by software. This bit is taken into account only if the audio block is configured as a master (MODE\[1\] = 0) and NODIV = 0. It generates an interrupt if the WCKCFG flag in the SAI_xSR register is set. Note: This bit is used only in Free protocol mode and is meaningless in other modes.
    pub mod WCKCFGIE {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::OVRUDRIE::RW;
    }

    /// FIFO request interrupt enable. This bit is set and cleared by software. When this bit is set, an interrupt is generated if the FREQ bit in the SAI_xSR register is set. Since the audio block defaults to operate as a transmitter after reset, the MODE bit must be configured before setting FREQIE to avoid a parasitic interrupt in receiver mode,
    pub mod FREQIE {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::OVRUDRIE::RW;
    }

    /// Codec not ready interrupt enable (AC’97). This bit is set and cleared by software. When the interrupt is enabled, the audio block detects in the slot 0 (tag0) of the AC’97 frame if the Codec connected to this line is ready or not. If it is not ready, the CNRDY flag in the SAI_xSR register is set and an interrupt is generated. This bit has a meaning only if the AC’97 mode is selected through PRTCFG\[1:0\] bits and the audio block is operates as a receiver.
    pub mod CNRDYIE {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::OVRUDRIE::RW;
    }

    /// Anticipated frame synchronization detection interrupt enable. This bit is set and cleared by software. When this bit is set, an interrupt is generated if the AFSDET bit in the SAI_xSR register is set. This bit is meaningless in AC’97, SPDIF mode or when the audio block operates as a master.
    pub mod AFSDETIE {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::OVRUDRIE::RW;
    }

    /// Late frame synchronization detection interrupt enable. This bit is set and cleared by software. When this bit is set, an interrupt is generated if the LFSDET bit is set in the SAI_xSR register. This bit is meaningless in AC’97, SPDIF mode or when the audio block operates as a master.
    pub mod LFSDETIE {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::OVRUDRIE::RW;
    }
}

/// SAI status register
pub mod ASR {

    /// Overrun / underrun. This bit is read only. The overrun and underrun conditions can occur only when the audio block is configured as a receiver and a transmitter, respectively. It can generate an interrupt if OVRUDRIE bit is set in SAI_xIM register. This flag is cleared when the software sets COVRUDR bit in SAI_xCLRFR register.
    pub mod OVRUDR {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: No overrun/underrun error
            pub const NoError: u32 = 0b0;

            /// 0b1: Overrun/underrun error detection
            pub const Overrun: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Mute detection. This bit is read only. This flag is set if consecutive 0 values are received in each slot of a given audio frame and for a consecutive number of audio frames (set in the MUTECNT bit in the SAI_xCR2 register). It can generate an interrupt if MUTEDETIE bit is set in SAI_xIM register. This flag is cleared when the software sets bit CMUTEDET in the SAI_xCLRFR register.
    pub mod MUTEDET {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: No MUTE detection on the SD input line
            pub const NoMute: u32 = 0b0;

            /// 0b1: MUTE value detected on the SD input line (0 value) for a specified number of consecutive audio frame
            pub const Mute: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Wrong clock configuration flag. This bit is read only. This bit is used only when the audio block operates in master mode (MODE\[1\] = 0) and NODIV = 0. It can generate an interrupt if WCKCFGIE bit is set in SAI_xIM register. This flag is cleared when the software sets CWCKCFG bit in SAI_xCLRFR register.
    pub mod WCKCFG {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: Clock configuration is correct
            pub const Correct: u32 = 0b0;

            /// 0b1: Clock configuration does not respect the rule concerning the frame length specification
            pub const Wrong: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// FIFO request. This bit is read only. The request depends on the audio block configuration: If the block is configured in transmission mode, the FIFO request is related to a write request operation in the SAI_xDR. If the block configured in reception, the FIFO request related to a read request operation from the SAI_xDR. This flag can generate an interrupt if FREQIE bit is set in SAI_xIM register.
    pub mod FREQ {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: No FIFO request
            pub const NoRequest: u32 = 0b0;

            /// 0b1: FIFO request to read or to write the SAI_xDR
            pub const Request: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Codec not ready. This bit is read only. This bit is used only when the AC’97 audio protocol is selected in the SAI_xCR1 register and configured in receiver mode. It can generate an interrupt if CNRDYIE bit is set in SAI_xIM register. This flag is cleared when the software sets CCNRDY bit in SAI_xCLRFR register.
    pub mod CNRDY {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: External AC’97 Codec is ready
            pub const Ready: u32 = 0b0;

            /// 0b1: External AC’97 Codec is not ready
            pub const NotReady: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Anticipated frame synchronization detection. This bit is read only. This flag can be set only if the audio block is configured in slave mode. It is not used in AC’97 or SPDIF mode. It can generate an interrupt if AFSDETIE bit is set in SAI_xIM register. This flag is cleared when the software sets CAFSDET bit in SAI_xCLRFR register.
    pub mod AFSDET {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: No error
            pub const NoError: u32 = 0b0;

            /// 0b1: Frame synchronization signal is detected earlier than expected
            pub const EarlySync: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Late frame synchronization detection. This bit is read only. This flag can be set only if the audio block is configured in slave mode. It is not used in AC’97 or SPDIF mode. It can generate an interrupt if LFSDETIE bit is set in the SAI_xIM register. This flag is cleared when the software sets bit CLFSDET in SAI_xCLRFR register
    pub mod LFSDET {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: No error
            pub const NoError: u32 = 0b0;

            /// 0b1: Frame synchronization signal is not present at the right time
            pub const NoSync: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// FIFO level threshold. This bit is read only. The FIFO level threshold flag is managed only by hardware and its setting depends on SAI block configuration (transmitter or receiver mode). Others: Reserved
    pub mod FLVL {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (3 bits: 0b111 << 16)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values
        pub mod R {

            /// 0b000: FIFO empty
            pub const Empty: u32 = 0b000;

            /// 0b001: FIFO <= 1⁄4 but not empty
            pub const Quarter1: u32 = 0b001;

            /// 0b010: 1⁄4 < FIFO <= 1⁄2
            pub const Quarter2: u32 = 0b010;

            /// 0b011: 1⁄2 < FIFO <= 3⁄4
            pub const Quarter3: u32 = 0b011;

            /// 0b100: 3⁄4 < FIFO but not full
            pub const Quarter4: u32 = 0b100;

            /// 0b101: FIFO full
            pub const Full: u32 = 0b101;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// SAI clear flag register
pub mod ACLRFR {

    /// Clear overrun / underrun. This bit is write only. Programming this bit to 1 clears the OVRUDR flag in the SAI_xSR register. Reading this bit always returns the value 0.
    pub mod COVRUDR {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values
        pub mod W {

            /// 0b1: Clears the OVRUDR flag
            pub const Clear: u32 = 0b1;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Mute detection flag. This bit is write only. Programming this bit to 1 clears the MUTEDET flag in the SAI_xSR register. Reading this bit always returns the value 0.
    pub mod CMUTEDET {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values
        pub mod W {

            /// 0b1: Clears the MUTEDET flag
            pub const Clear: u32 = 0b1;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Clear wrong clock configuration flag. This bit is write only. Programming this bit to 1 clears the WCKCFG flag in the SAI_xSR register. This bit is used only when the audio block is set as master (MODE\[1\] = 0) and NODIV = 0 in the SAI_xCR1 register. Reading this bit always returns the value 0.
    pub mod CWCKCFG {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values
        pub mod W {

            /// 0b1: Clears the WCKCFG flag
            pub const Clear: u32 = 0b1;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Clear Codec not ready flag. This bit is write only. Programming this bit to 1 clears the CNRDY flag in the SAI_xSR register. This bit is used only when the AC’97 audio protocol is selected in the SAI_xCR1 register. Reading this bit always returns the value 0.
    pub mod CCNRDY {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values
        pub mod W {

            /// 0b1: Clears the CNRDY flag
            pub const Clear: u32 = 0b1;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Clear anticipated frame synchronization detection flag. This bit is write only. Programming this bit to 1 clears the AFSDET flag in the SAI_xSR register. It is not used in AC’97 or SPDIF mode. Reading this bit always returns the value 0.
    pub mod CAFSDET {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values
        pub mod W {

            /// 0b1: Clears the AFSDET flag
            pub const Clear: u32 = 0b1;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Clear late frame synchronization detection flag. This bit is write only. Programming this bit to 1 clears the LFSDET flag in the SAI_xSR register. This bit is not used in AC’97 or SPDIF mode Reading this bit always returns the value 0.
    pub mod CLFSDET {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values
        pub mod W {

            /// 0b1: Clears the LFSDET flag
            pub const Clear: u32 = 0b1;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// SAI data register
pub mod ADR {

    /// Data A write to this register loads the FIFO provided the FIFO is not full. A read from this register empties the FIFO if the FIFO is not empty.
    pub mod DATA {
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

/// SAI configuration register 1
pub mod BCR1 {
    pub use super::ACR1::CKSTR;
    pub use super::ACR1::DMAEN;
    pub use super::ACR1::DS;
    pub use super::ACR1::LSBFIRST;
    pub use super::ACR1::MCKDIV;
    pub use super::ACR1::MCKEN;
    pub use super::ACR1::MODE;
    pub use super::ACR1::MONO;
    pub use super::ACR1::NODIV;
    pub use super::ACR1::OSR;
    pub use super::ACR1::OUTDRIV;
    pub use super::ACR1::PRTCFG;
    pub use super::ACR1::SAIEN;
    pub use super::ACR1::SYNCEN;
}

/// SAI configuration register 2
pub mod BCR2 {
    pub use super::ACR2::COMP;
    pub use super::ACR2::CPL;
    pub use super::ACR2::FFLUSH;
    pub use super::ACR2::FTH;
    pub use super::ACR2::MUTE;
    pub use super::ACR2::MUTECNT;
    pub use super::ACR2::MUTEVAL;
    pub use super::ACR2::TRIS;
}

/// SAI frame configuration register
pub mod BFRCR {
    pub use super::AFRCR::FRL;
    pub use super::AFRCR::FSALL;
    pub use super::AFRCR::FSDEF;
    pub use super::AFRCR::FSOFF;
    pub use super::AFRCR::FSPOL;
}

/// SAI slot register
pub mod BSLOTR {
    pub use super::ASLOTR::FBOFF;
    pub use super::ASLOTR::NBSLOT;
    pub use super::ASLOTR::SLOTEN;
    pub use super::ASLOTR::SLOTSZ;
}

/// SAI interrupt mask register
pub mod BIM {
    pub use super::AIM::AFSDETIE;
    pub use super::AIM::CNRDYIE;
    pub use super::AIM::FREQIE;
    pub use super::AIM::LFSDETIE;
    pub use super::AIM::MUTEDETIE;
    pub use super::AIM::OVRUDRIE;
    pub use super::AIM::WCKCFGIE;
}

/// SAI status register
pub mod BSR {
    pub use super::ASR::AFSDET;
    pub use super::ASR::CNRDY;
    pub use super::ASR::FLVL;
    pub use super::ASR::FREQ;
    pub use super::ASR::LFSDET;
    pub use super::ASR::MUTEDET;
    pub use super::ASR::OVRUDR;
    pub use super::ASR::WCKCFG;
}

/// SAI clear flag register
pub mod BCLRFR {
    pub use super::ACLRFR::CAFSDET;
    pub use super::ACLRFR::CCNRDY;
    pub use super::ACLRFR::CLFSDET;
    pub use super::ACLRFR::CMUTEDET;
    pub use super::ACLRFR::COVRUDR;
    pub use super::ACLRFR::CWCKCFG;
}

/// SAI data register
pub mod BDR {
    pub use super::ADR::DATA;
}

/// SAI PDM control register
pub mod PDMCR {

    /// PDM enable This bit is set and cleared by software. This bit allows to control the state of the PDM interface block. Make sure that the SAI in already operating in TDM master mode before enabling the PDM interface.
    pub mod PDMEN {
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

    /// Number of microphones This bit is set and cleared by software. Note: It is not recommended to configure this field when PDMEN = 1.* The complete set of data lines might not be available for all SAI instances. Refer to for details.
    pub mod MICNBR {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (2 bits: 0b11 << 4)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Clock enable of bitstream clock number 1 This bit is set and cleared by software. Note: It is not recommended to configure this bit when PDMEN = 1. SAI_CK1 might not be available for all SAI instances. Refer to implementation for details.
    pub mod CKEN1 {
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

    /// Clock enable of bitstream clock number 2 This bit is set and cleared by software. Note: It is not recommended to configure this bit when PDMEN = 1. SAI_CK2 might not be available for all SAI instances. Refer to implementation for details.
    pub mod CKEN2 {
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

/// SAI PDM delay register
pub mod PDMDLY {

    /// Delay line adjust for first microphone of pair 1 This bit is set and cleared by software. ... This field can be changed on-the-fly. Note: This field can be used only if D1 line is available.Refer to to check if it is available.
    pub mod DLYM1L {
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

    /// Delay line adjust for second microphone of pair 1 This bit is set and cleared by software. ... This field can be changed on-the-fly. Note: This field can be used only if D1 line is available.Refer to to check if it is available.
    pub mod DLYM1R {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (3 bits: 0b111 << 4)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Delay line for first microphone of pair 2 This bit is set and cleared by software. ... This field can be changed on-the-fly. Note: This field can be used only if D2 line is available.Refer to to check if it is available.
    pub mod DLYM2L {
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

    /// Delay line for second microphone of pair 2 This bit is set and cleared by software. ... This field can be changed on-the-fly. Note: This field can be used only if D2 line is available.Refer to to check if it is available.
    pub mod DLYM2R {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (3 bits: 0b111 << 12)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Delay line for first microphone of pair 3 This bit is set and cleared by software. ... This field can be changed on-the-fly. Note: This field can be used only if D3 line is available.Refer to to check if it is available.
    pub mod DLYM3L {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (3 bits: 0b111 << 16)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Delay line for second microphone of pair 3 This bit is set and cleared by software. ... This field can be changed on-the-fly. Note: This field can be used only if D3 line is available.Refer to to check if it is available.
    pub mod DLYM3R {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (3 bits: 0b111 << 20)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Delay line for first microphone of pair 4 This bit is set and cleared by software. ... This field can be changed on-the-fly. Note: This field can be used only if D4 line is available.Refer to to check if it is available.
    pub mod DLYM4L {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (3 bits: 0b111 << 24)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Delay line for second microphone of pair 4 This bit is set and cleared by software. ... This field can be changed on-the-fly. Note: This field can be used only if D4 line is available.Refer to to check if it is available.
    pub mod DLYM4R {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (3 bits: 0b111 << 28)
        pub const mask: u32 = 0b111 << offset;
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
    /// SAI global configuration register
    pub GCR: RWRegister<u32>,

    /// SAI configuration register 1
    pub ACR1: RWRegister<u32>,

    /// SAI configuration register 2
    pub ACR2: RWRegister<u32>,

    /// SAI frame configuration register
    pub AFRCR: RWRegister<u32>,

    /// SAI slot register
    pub ASLOTR: RWRegister<u32>,

    /// SAI interrupt mask register
    pub AIM: RWRegister<u32>,

    /// SAI status register
    pub ASR: RORegister<u32>,

    /// SAI clear flag register
    pub ACLRFR: WORegister<u32>,

    /// SAI data register
    pub ADR: RWRegister<u32>,

    /// SAI configuration register 1
    pub BCR1: RWRegister<u32>,

    /// SAI configuration register 2
    pub BCR2: RWRegister<u32>,

    /// SAI frame configuration register
    pub BFRCR: RWRegister<u32>,

    /// SAI slot register
    pub BSLOTR: RWRegister<u32>,

    /// SAI interrupt mask register
    pub BIM: RWRegister<u32>,

    /// SAI status register
    pub BSR: RORegister<u32>,

    /// SAI clear flag register
    pub BCLRFR: WORegister<u32>,

    /// SAI data register
    pub BDR: RWRegister<u32>,

    /// SAI PDM control register
    pub PDMCR: RWRegister<u32>,

    /// SAI PDM delay register
    pub PDMDLY: RWRegister<u32>,
}
pub struct ResetValues {
    pub GCR: u32,
    pub ACR1: u32,
    pub ACR2: u32,
    pub AFRCR: u32,
    pub ASLOTR: u32,
    pub AIM: u32,
    pub ASR: u32,
    pub ACLRFR: u32,
    pub ADR: u32,
    pub BCR1: u32,
    pub BCR2: u32,
    pub BFRCR: u32,
    pub BSLOTR: u32,
    pub BIM: u32,
    pub BSR: u32,
    pub BCLRFR: u32,
    pub BDR: u32,
    pub PDMCR: u32,
    pub PDMDLY: u32,
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
