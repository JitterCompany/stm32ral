#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! General purpose direct memory access controller

use crate::{RORegister, RWRegister, WORegister};
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// GPDMA privileged configuration register
pub mod PRIVCFGR {

    /// privileged state of channel x
    pub mod PRIV0 {
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

    /// privileged state of channel x
    pub mod PRIV1 {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// privileged state of channel x
    pub mod PRIV2 {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// privileged state of channel x
    pub mod PRIV3 {
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

    /// privileged state of channel x
    pub mod PRIV4 {
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

    /// privileged state of channel x
    pub mod PRIV5 {
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

    /// privileged state of channel x
    pub mod PRIV6 {
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

    /// privileged state of channel x
    pub mod PRIV7 {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// GPDMA masked interrupt status register
pub mod MISR {

    /// masked interrupt status of channel x
    pub mod MIS0 {
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

    /// masked interrupt status of channel x
    pub mod MIS1 {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// masked interrupt status of channel x
    pub mod MIS2 {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// masked interrupt status of channel x
    pub mod MIS3 {
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

    /// masked interrupt status of channel x
    pub mod MIS4 {
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

    /// masked interrupt status of channel x
    pub mod MIS5 {
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

    /// masked interrupt status of channel x
    pub mod MIS6 {
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

    /// masked interrupt status of channel x
    pub mod MIS7 {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// GPDMA channel 0 linked-list base address register
pub mod C0LBAR {

    /// linked-list base address of GPDMA channel x
    pub mod LBA {
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

/// GPDMA channel 0 flag clear register
pub mod C0FCR {

    /// transfer complete flag clear
    pub mod TCF {
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

    /// half transfer flag clear
    pub mod HTF {
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

    /// data transfer error flag clear
    pub mod DTEF {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// update link transfer error flag clear
    pub mod ULEF {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// user setting error flag clear
    pub mod USEF {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// completed suspension flag clear
    pub mod SUSPF {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// trigger overrun flag clear
    pub mod TOF {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// GPDMA channel 0 status register
pub mod C0SR {

    /// idle flag This idle flag is deasserted by hardware when the channel is enabled (GPDMA_CxCR.EN = 1) with a valid channel configuration (no USEF to be immediately reported). This idle flag is asserted after hard reset or by hardware when the channel is back in idle state (in suspended or disabled state).
    pub mod IDLEF {
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

    /// transfer complete flag A transfer complete event is either a block transfer complete, a 2D/repeated block transfer complete, or a LLI transfer complete including the upload of the next LLI if any, or the full linked-list completion, depending on the transfer complete event mode (GPDMA_CxTR2.TCEM\[1:0\]).
    pub mod TCF {
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

    /// half transfer flag A half transfer event is either a half block transfer or a half 2D/repeated block transfer, depending on the transfer complete event mode (GPDMA_CxTR2.TCEM\[1:0\]). A half block transfer occurs when half of the bytes of the source block size (rounded up integer of GPDMA_CxBR1.BNDT\[15:0\]/2) has been transferred to the destination. A half 2D/repeated block transfer occurs when half of the repeated blocks (rounded up integer of (GPDMA_CxBR1.BRC\[10:0\]+1)/2)) has been transferred to the destination.
    pub mod HTF {
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

    /// data transfer error flag
    pub mod DTEF {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// update link transfer error flag
    pub mod ULEF {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// user setting error flag
    pub mod USEF {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// completed suspension flag
    pub mod SUSPF {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// trigger overrun flag
    pub mod TOF {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// monitored FIFO level Number of available write beats in the FIFO, in units of the programmed destination data width (see GPDMA_CxTR1.DDW_LOG2\[1:0\], in units of bytes, half-words, or words). Note: After having suspended an active transfer, the user may need to read FIFOL\[7:0\], additionally to GPDMA_CxBR1.BDNT\[15:0\] and GPDMA_CxBR1.BRC\[10:0\], to know how many data have been transferred to the destination. Before reading, the user may wait for the transfer to be suspended (GPDMA_CxSR.SUSPF = 1).
    pub mod FIFOL {
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

/// GPDMA channel 0 control register
pub mod C0CR {

    /// enable Writing 1 into the field RESET (bit 1) causes the hardware to de-assert this bit, whatever is written into this bit 0. Else: this bit is deasserted by hardware when there is a transfer error (master bus error or user setting error) or when there is a channel transfer complete (channel ready to be configured, for example if LSM=1 at the end of a single execution of the LLI). Else, this bit can be asserted by software. Writing 0 into this EN bit is ignored.
    pub mod EN {
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

    /// reset This bit is write only. Writing 0 has no impact. Writing 1 implies the reset of the following: the FIFO, the channel internal state, SUSP and EN bits (whatever is written receptively in bit 2 and bit 0). The reset is effective when the channel is in steady state, meaning one of the following: - active channel in suspended state (GPDMA_CxSR.SUSPF = 1 and GPDMA_CxSR.IDLEF = GPDMA_CxCR.EN = 1) - channel in disabled state (GPDMA_CxSR.IDLEF = 1 and GPDMA_CxCR.EN = 0). After writing a RESET, to continue using this channel, the user must explicitly reconfigure the channel including the hardware-modified configuration registers (GPDMA_CxBR1, GPDMA_CxSAR and GPDMA_CxDAR) before enabling again the channel (see the programming sequence in Figure 44).
    pub mod RESET {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// suspend Writing 1 into the field RESET (bit 1) causes the hardware to de-assert this bit, whatever is written into this bit 2. Else: Software must write 1 in order to suspend an active channel (channel with an ongoing GPDMA transfer over its master ports). The software must write 0 in order to resume a suspended channel, following the programming sequence detailed in Figure 43.
    pub mod SUSP {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// transfer complete interrupt enable
    pub mod TCIE {
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

    /// half transfer complete interrupt enable
    pub mod HTIE {
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

    /// data transfer error interrupt enable
    pub mod DTEIE {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// update link transfer error interrupt enable
    pub mod ULEIE {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// user setting error interrupt enable
    pub mod USEIE {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// completed suspension interrupt enable
    pub mod SUSPIE {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// trigger overrun interrupt enable
    pub mod TOIE {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Link step mode First the (possible 1D/repeated) block transfer is executed as defined by the current internal register file until GPDMA_CxBR1.BNDT\[15:0\] = 0 and GPDMA_CxBR1.BRC\[10:0\] = 0. Secondly the next linked-list data structure is conditionally uploaded from memory as defined by GPDMA_CxLLR. Then channel execution is completed. Note: This bit must be written when EN=0. This bit is read-only when EN=1.
    pub mod LSM {
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

    /// linked-list allocated port This bit is used to allocate the master port for the update of the GPDMA linked-list registers from the memory. Note: This bit must be written when EN=0. This bit is read-only when EN=1.
    pub mod LAP {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// priority level of the channel x GPDMA transfer versus others Note: This bit must be written when EN = 0. This bit is read-only when EN = 1.
    pub mod PRIO {
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
}

/// GPDMA channel 0 transfer register 1
pub mod C0TR1 {

    /// binary logarithm of the source data width of a burst in bytes Setting a 8-byte data width causes a user setting error to be reported and no transfer is issued. A source block size must be a multiple of the source data width (GPDMA_CxBR1.BNDT\[2:0\] versus SDW_LOG2\[1:0\]). Otherwise, a user setting error is reported and no transfer is issued. Note: A source burst transfer must have an aligned address with its data width (start address GPDMA_CxSAR\[2:0\] versus SDW_LOG2\[1:0\]). Otherwise, a user setting error is reported and none transfer is issued.
    pub mod SDW_LOG2 {
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

    /// source incrementing burst The source address, pointed by GPDMA_CxSAR, is kept constant after a burst beat/single transfer or is incremented by the offset value corresponding to a contiguous data after a burst beat/single transfer.
    pub mod SINC {
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

    /// source burst length minus 1, between 0 and 63 The burst length unit is one data named beat within a burst. If SBL_1\[5:0\] =0 , the burst can be named as single. Each data/beat has a width defined by the destination data width SDW_LOG2\[1:0\]. Note: If a burst transfer crossed a 1-Kbyte address boundary on a AHB transfer, the GPDMA modifies and shortens the programmed burst into singles or bursts of lower length, to be compliant with the AHB protocol. Note: If a burst transfer is of length greater than the FIFO size of the channel x, the GPDMA modifies and shortens the programmed burst into singles or bursts of lower length, to be compliant with the FIFO size. Transfer performance is lower, with GPDMA re-arbitration between effective and lower singles/bursts, but the data integrity is guaranteed.
    pub mod SBL_1 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (6 bits: 0x3f << 4)
        pub const mask: u32 = 0x3f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// padding/alignment mode If DDW_LOG2\[1:0\] = SDW_LOG2\[1:0\]: if the data width of a burst destination transfer is equal to the data width of a burst source transfer, these bits are ignored. Else, in the following enumerated values, the condition PAM_1 is when destination data width is higher that source data width, and the condition PAM_2 is when destination data width is higher than source data width. 1x: successive source data are FIFO queued and packed at the destination data width, in a left (LSB) to right (MSB) order (named little endian), before a destination transfer 1x: source data is FIFO queued and unpacked at the destination data width, to be transferred in a left (LSB) to right (MSB) order (named little endian) to the destination Note: If the transfer from the source peripheral is configured with peripheral flow-control mode (SWREQ = 0 and PFREQ = 1 and DREQ = 0), and if the destination data width > the source data width, packing is not supported.
    pub mod PAM {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (2 bits: 0b11 << 11)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// source byte exchange within the unaligned half-word of each source word If the source data width is shorter than a word, this bit is ignored. If the source data width is a word:
    pub mod SBX {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// source allocated port This bit is used to allocate the master port for the source transfer Note: This bit must be written when EN = 0. This bit is read-only when EN = 1.
    pub mod SAP {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// binary logarithm of the destination data width of a burst, in bytes Setting a 8-byte data width causes a user setting error to be reported and none transfer is issued. Note: A destination burst transfer must have an aligned address with its data width (start address GPDMA_CxDAR\[2:0\] and address offset GPDMA_CxTR3.DAO\[2:0\], versus DDW_LOG2\[1:0\]). Otherwise a user setting error is reported and no transfer is issued.
    pub mod DDW_LOG2 {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (2 bits: 0b11 << 16)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// destination incrementing burst The destination address, pointed by GPDMA_CxDAR, is kept constant after a burst beat/single transfer, or is incremented by the offset value corresponding to a contiguous data after a burst beat/single transfer.
    pub mod DINC {
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

    /// destination burst length minus 1, between 0 and 63 The burst length unit is one data named beat within a burst. If DBL_1\[5:0\] =0 , the burst can be named as single. Each data/beat has a width defined by the destination data width DDW_LOG2\[1:0\]. Note: If a burst transfer crossed a 1-Kbyte address boundary on a AHB transfer, the GPDMA modifies and shortens the programmed burst into singles or bursts of lower length, to be compliant with the AHB protocol. Note: If a burst transfer is of length greater than the FIFO size of the channel x, the GPDMA modifies and shortens the programmed burst into singles or bursts of lower length, to be compliant with the FIFO size. Transfer performance is lower, with GPDMA re-arbitration between effective and lower singles/bursts, but the data integrity is guaranteed.
    pub mod DBL_1 {
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

    /// destination byte exchange If the destination data size is a byte, this bit is ignored. If the destination data size is not a byte:
    pub mod DBX {
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

    /// destination half-word exchange If the destination data size is shorter than a word, this bit is ignored. If the destination data size is a word:
    pub mod DHX {
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

    /// destination allocated port This bit is used to allocate the master port for the destination transfer Note: This bit must be written when EN = 0. This bit is read-only when EN = 1.
    pub mod DAP {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (1 bit: 1 << 30)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// GPDMA channel 0 transfer register 2
pub mod C0TR2 {

    /// GPDMA hardware request selection These bits are ignored if channel x is activated (GPDMA_CxCR.EN asserted) with SWREQ = 1 (software request for a memory-to-memory transfer). Else, the selected hardware request is internally taken into account as per Section 14.3.4. The user must not assign a same input hardware request (same REQSEL\[7:0\] value) to different active GPDMA channels (GPDMA_CxCR.EN = 1 and GPDMA_CxTR2.SWREQ = 0 for these channels). GPDMA is not intended to hardware support the case of simultaneous enabled channels incorrectly configured with a same hardware peripheral request signal, and there is no user setting error reporting.
    pub mod REQSEL {
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

    /// software request This bit is internally taken into account when GPDMA_CxCR.EN is asserted.
    pub mod SWREQ {
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

    /// destination hardware request This bit is ignored if channel x is activated (GPDMA_CxCR.EN asserted) with SWREQ = 1 (software request for a memory-to-memory transfer). Else: Note: If the channel x is activated (GPDMA_CxCR.EN is asserted) with SWREQ = 0 and PFREQ = 1 (peripheral hardware request with peripheral flow-control mode), any software assertion to this DREQ bit is ignored: in peripheral flow-control mode, only a peripheral-to-memory transfer is supported.
    pub mod DREQ {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Block hardware request If the channel x is activated (GPDMA_CxCR.EN asserted) with SWREQ = 1 (software request for a memory-to-memory transfer), this bit is ignored. Else:
    pub mod BREQ {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Hardware request in peripheral flow control mode Important: If a given channel x is not implemented with this feature, this bit is reserved and PFREQ is not present (see Section 14.3.2 for the list of the implemented channels with this feature. If the channel x is activated (GPDMA_CxCR.EN asserted) with SWREQ = 1 (software request for a memory-to-memory transfer), this bit is ignored. Else: Note: In peripheral flow control mode, there are the following restrictions: Note: - no 2D/repeated block support (GPDMA_CxBR1.BRC\[10:0\] must be set to 0) Note: - the peripheral must be set as the source of the transfer (DREQ = 0). Note: - data packing to a wider destination width is not supported (if destination width > source data width, GPDMA_CxTR1.PAM\[1\] must be set to 0). Note: - GPDMA_CxBR1.BNDT\[15:0\] must be programmed as a multiple of the source (peripheral) burst size.
    pub mod PFREQ {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// trigger mode These bits define the transfer granularity for its conditioning by the trigger. If the channel x is enabled (GPDMA_CxCR.EN asserted) with TRIGPOL\[1:0\] = 00 or 11, these TRIGM\[1:0\] bits are ignored. Else, a GPDMA transfer is conditioned by at least one trigger hit: – If the peripheral is programmed as a source (DREQ = 0) of the LLI data transfer, each programmed burst read is conditioned. – If the peripheral is programmed as a destination (DREQ = 1) of the LLI data transfer, each programmed burst write is conditioned. The first memory burst read of a (possibly 2D/repeated) block, also named as the first ready FIFO-based source burst, is gated by the occurrence of both the hardware request and the first trigger hit. The GPDMA monitoring of a trigger for channel x is started when the channel is enabled/loaded with a new active trigger configuration: rising or falling edge on a selected trigger (TRIGPOL\[1:0\] = 01 or respectively TRIGPOL\[1:0\] = 10). The monitoring of this trigger is kept active during the triggered and uncompleted (data or link) transfer; and if a new trigger is detected then, this hit is internally memorized to grant the next transfer, as long as the defined rising or falling edge is not modified, and the TRIGSEL\[5:0\] is not modified, and the channel is enabled. Transferring a next LLI<sub>n+1</sub> that updates the GPDMA_CxTR2 with a new value for any of TRIGSEL\[5:0\] or TRIGPOL\[1:0\], resets the monitoring, trashing the memorized hit of the formerly defined LLI<sub>n </sub>trigger. After a first new trigger hit<sub>n+1</sub> is memorized, if another second trigger hit<sub>n+2</sub> is detected and if the hit<sub>n</sub> triggered transfer is still not completed, hit<sub>n+2 </sub>is lost and not memorized.memorized. A trigger overrun flag is reported (GPDMA_CxSR.TOF =1 ), and an interrupt is generated if enabled (GPDMA_CxCR.TOIE = 1). The channel is not automatically disabled by hardware due to a trigger overrun. Note: When the source block size is not a multiple of the source burst size and is a multiple of the source data width, then the last programmed source burst is not completed and is internally shorten to match the block size. In this case, if TRIGM\[1:0\] = 11 and (SWREQ =1 or (SWREQ = 0 and DREQ =0 )), the shortened burst transfer (by singles or/and by bursts of lower length) is conditioned once by the trigger. Note: When the programmed destination burst is internally shortened by singles or/and by bursts of lower length (versus FIFO size, versus block size, 1-Kbyte boundary address crossing): if the trigger is conditioning the programmed destination burst (if TRIGM\[1:0\] = 11 and SWREQ = 0 and DREQ = 1), this shortened destination burst transfer is conditioned once by the trigger.
    pub mod TRIGM {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (2 bits: 0b11 << 14)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// trigger event input selection These bits select the trigger event input of the GPDMA transfer (as per Section 14.3.7), with an active trigger event if TRIGPOL\[1:0\] ≠ 00.
    pub mod TRIGSEL {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (6 bits: 0x3f << 16)
        pub const mask: u32 = 0x3f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// trigger event polarity These bits define the polarity of the selected trigger event input defined by TRIGSEL\[5:0\].
    pub mod TRIGPOL {
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

    /// transfer complete event mode These bits define the transfer granularity for the transfer complete and half transfer complete events generation. Note: If the initial LLI<sub>0 </sub>data transfer is null/void (directly programmed by the internal register file with GPDMA_CxBR1.BNDT\[15:0\] = 0), then neither the complete transfer event nor the half transfer event is generated. Note: If the initial LLI<sub>0 </sub>data transfer is null/void (directly programmed by the internal register file with GPDMA_CxBR1.BNDT\[15:0\] = 0), then neither the complete transfer event nor the half transfer event is generated. Note: If the initial LLI<sub>0 </sub>data transfer is null/void (i.e. directly programmed by the internal register file with GPDMA_CxBR1.BNDT\[15:0\] =0 ), then the half transfer event is not generated, and the transfer complete event is generated when is completed the loading of the LLI<sub>1</sub>.
    pub mod TCEM {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (2 bits: 0b11 << 30)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// GPDMA channel 0 block register 1
pub mod C0BR1 {

    /// block number of data bytes to transfer from the source Block size transferred from the source. When the channel is enabled, this field becomes read-only and is decremented, indicating the remaining number of data items in the current source block to be transferred. BNDT\[15:0\] is programmed in number of bytes, maximum source block size is 64 Kbytes -1. Once the last data transfer is completed (BNDT\[15:0\] = 0): - if GPDMA_CxLLR.UB1 = 1, this field is updated by the LLI in the memory. - if GPDMA_CxLLR.UB1 = 0 and if there is at least one non null Uxx update bit, this field is internally restored to the programmed value. - if all GPDMA_CxLLR.Uxx = 0 and if GPDMA_CxLLR.LA\[15:0\] = 0, this field is internally restored to the programmed value (infinite/continuous last LLI). - if GPDMA_CxLLR = 0, this field is kept as zero following the last LLI data transfer. Note: A non-null source block size must be a multiple of the source data width (BNDT\[2:0\] versus GPDMA_CxTR1.SDW_LOG2\[1:0\]). Else a user setting error is reported and no transfer is issued. Note: When configured in packing mode (GPDMA_CxTR1.PAM\[1\] = 1 and destination data width different from source data width), a non-null source block size must be a multiple of the destination data width (BNDT\[2:0\] versus GPDMA_CxTR1.DDW_LOG2\[1:0\]). Else a user setting error is reported and no transfer is issued.
    pub mod BNDT {
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

/// GPDMA channel 0 source address register
pub mod C0SAR {

    /// source address This field is the pointer to the address from which the next data is read. During the channel activity, depending on the source addressing mode (GPDMA_CxTR1.SINC), this field is kept fixed or incremented by the data width (GPDMA_CxTR1.SDW_LOG2\[1:0\]) after each burst source data, reflecting the next address from which data is read. During the channel activity, this address is updated after each completed source burst, consequently to: the programmed source burst; either in fixed addressing mode or in contiguous-data incremented mode. If contiguously incremented (GPDMA_CxTR1.SINC = 1), then the additional address offset value is the programmed burst size, as defined by GPDMA_CxTR1.SBL_1\[5:0\] and GPDMA_CxTR1.SDW_LOG2\[21:0\] the additional source incremented/decremented offset value as programmed by GPDMA_CxBR1.SDEC and GPDMA_CxTR3.SAO\[12:0\]. once/if completed source block transfer, for a channel x with 2D addressing capability (x = 12 to 15). additional block repeat source incremented/decremented offset value as programmed by GPDMA_CxBR1.BRSDEC and GPDMA_CxBR2.BRSAO\[15:0\] In linked-list mode, after a LLI data transfer is completed, this register is automatically updated by GPDMA from the memory, provided the LLI is set with GPDMA_CxLLR.USA = 1. Note: A source address must be aligned with the programmed data width of a source burst (SA\[2:0\] versus GPDMA_CxTR1.SDW_LOG2\[1:0\]). Else, a user setting error is reported and no transfer is issued. Note: When the source block size is not a multiple of the source burst size and is a multiple of the source data width, the last programmed source burst is not completed and is internally shorten to match the block size. In this case, the additional GPDMA_CxTR3.SAO\[12:0\] is not applied.
    pub mod SA {
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

/// GPDMA channel 0 destination address register
pub mod C0DAR {

    /// destination address This field is the pointer to the address from which the next data is written. During the channel activity, depending on the destination addressing mode (GPDMA_CxTR1.DINC), this field is kept fixed or incremented by the data width (GPDMA_CxTR1.DDW_LOG2\[21:0\]) after each burst destination data, reflecting the next address from which data is written. During the channel activity, this address is updated after each completed destination burst, consequently to: the programmed destination burst; either in fixed addressing mode or in contiguous-data incremented mode. If contiguously incremented (GPDMA_CxTR1.DINC = 1), then the additional address offset value is the programmed burst size, as defined by GPDMA_CxTR1.DBL_1\[5:0\] and GPDMA_CxTR1.DDW_LOG2\[1:0\] the additional destination incremented/decremented offset value as programmed by GPDMA_CxBR1.DDEC and GPDMA_CxTR3.DAO\[12:0\]. once/if completed destination block transfer, for a channel x with 2D addressing capability (x = 12 to 15), the additional block repeat destination incremented/decremented offset value as programmed by GPDMA_CxBR1.BRDDEC and GPDMA_CxBR2.BRDAO\[15:0\] In linked-list mode, after a LLI data transfer is completed, this register is automatically updated by the GPDMA from the memory, provided the LLI is set with GPDMA_CxLLR.UDA = 1. Note: A destination address must be aligned with the programmed data width of a destination burst (DA\[2:0\] versus GPDMA_CxTR1.DDW_LOG2\[1:0\]). Else, a user setting error is reported and no transfer is issued.
    pub mod DA {
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

/// GPDMA channel 0 linked-list address register
pub mod C0LLR {

    /// pointer (16-bit low-significant address) to the next linked-list data structure If UT1 = UT2 = UB1 = USA = UDA = ULL = 0 and if LA\[15:20\] = 0, the current LLI is the last one. The channel transfer is completed without any update of the linked-list GPDMA register file. Else, this field is the pointer to the memory address offset from which the next linked-list data structure is automatically fetched from, once the data transfer is completed, in order to conditionally update the linked-list GPDMA internal register file (GPDMA_CxCTR1, GPDMA_CxTR2, GPDMA_CxBR1, GPDMA_CxSAR, GPDMA_CxDAR and GPDMA_CxLLR). Note: The user must program the pointer to be 32-bit aligned. The two low-significant bits are write ignored.
    pub mod LA {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (14 bits: 0x3fff << 2)
        pub const mask: u32 = 0x3fff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Update GPDMA_CxLLR register from memory This bit is used to control the update of GPDMA_CxLLR from the memory during the link transfer.
    pub mod ULL {
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

    /// Update GPDMA_CxDAR register from memory This bit is used to control the update of GPDMA_CxDAR from the memory during the link transfer.
    pub mod UDA {
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

    /// update GPDMA_CxSAR from memory This bit controls the update of GPDMA_CxSAR from the memory during the link transfer.
    pub mod USA {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (1 bit: 1 << 28)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Update GPDMA_CxBR1 from memory This bit controls the update of GPDMA_CxBR1 from the memory during the link transfer. If UB1 = 0 and if GPDMA_CxLLR ≠ 0, the linked-list is not completed. GPDMA_CxBR1.BNDT\[15:0\] is then restored to the programmed value after data transfer is completed and before the link transfer.
    pub mod UB1 {
        /// Offset (29 bits)
        pub const offset: u32 = 29;
        /// Mask (1 bit: 1 << 29)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Update GPDMA_CxTR2 from memory This bit controls the update of GPDMA_CxTR2 from the memory during the link transfer.
    pub mod UT2 {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (1 bit: 1 << 30)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Update GPDMA_CxTR1 from memory This bit controls the update of GPDMA_CxTR1 from the memory during the link transfer.
    pub mod UT1 {
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

/// GPDMA channel 1 linked-list base address register
pub mod C1LBAR {
    pub use super::C0LBAR::LBA;
}

/// GPDMA channel 1 flag clear register
pub mod C1FCR {
    pub use super::C0FCR::DTEF;
    pub use super::C0FCR::HTF;
    pub use super::C0FCR::SUSPF;
    pub use super::C0FCR::TCF;
    pub use super::C0FCR::TOF;
    pub use super::C0FCR::ULEF;
    pub use super::C0FCR::USEF;
}

/// GPDMA channel 1 status register
pub mod C1SR {
    pub use super::C0SR::DTEF;
    pub use super::C0SR::FIFOL;
    pub use super::C0SR::HTF;
    pub use super::C0SR::IDLEF;
    pub use super::C0SR::SUSPF;
    pub use super::C0SR::TCF;
    pub use super::C0SR::TOF;
    pub use super::C0SR::ULEF;
    pub use super::C0SR::USEF;
}

/// GPDMA channel 1 control register
pub mod C1CR {
    pub use super::C0CR::DTEIE;
    pub use super::C0CR::EN;
    pub use super::C0CR::HTIE;
    pub use super::C0CR::LAP;
    pub use super::C0CR::LSM;
    pub use super::C0CR::PRIO;
    pub use super::C0CR::RESET;
    pub use super::C0CR::SUSP;
    pub use super::C0CR::SUSPIE;
    pub use super::C0CR::TCIE;
    pub use super::C0CR::TOIE;
    pub use super::C0CR::ULEIE;
    pub use super::C0CR::USEIE;
}

/// GPDMA channel 1 transfer register 1
pub mod C1TR1 {
    pub use super::C0TR1::DAP;
    pub use super::C0TR1::DBL_1;
    pub use super::C0TR1::DBX;
    pub use super::C0TR1::DDW_LOG2;
    pub use super::C0TR1::DHX;
    pub use super::C0TR1::DINC;
    pub use super::C0TR1::PAM;
    pub use super::C0TR1::SAP;
    pub use super::C0TR1::SBL_1;
    pub use super::C0TR1::SBX;
    pub use super::C0TR1::SDW_LOG2;
    pub use super::C0TR1::SINC;
}

/// GPDMA channel 1 transfer register 2
pub mod C1TR2 {
    pub use super::C0TR2::BREQ;
    pub use super::C0TR2::DREQ;
    pub use super::C0TR2::PFREQ;
    pub use super::C0TR2::REQSEL;
    pub use super::C0TR2::SWREQ;
    pub use super::C0TR2::TCEM;
    pub use super::C0TR2::TRIGM;
    pub use super::C0TR2::TRIGPOL;
    pub use super::C0TR2::TRIGSEL;
}

/// GPDMA channel 1 block register 1
pub mod C1BR1 {
    pub use super::C0BR1::BNDT;
}

/// GPDMA channel 1 source address register
pub mod C1SAR {
    pub use super::C0SAR::SA;
}

/// GPDMA channel 1 destination address register
pub mod C1DAR {
    pub use super::C0DAR::DA;
}

/// GPDMA channel 1 linked-list address register
pub mod C1LLR {
    pub use super::C0LLR::LA;
    pub use super::C0LLR::UB1;
    pub use super::C0LLR::UDA;
    pub use super::C0LLR::ULL;
    pub use super::C0LLR::USA;
    pub use super::C0LLR::UT1;
    pub use super::C0LLR::UT2;
}

/// GPDMA channel 2 linked-list base address register
pub mod C2LBAR {
    pub use super::C0LBAR::LBA;
}

/// GPDMA channel 2 flag clear register
pub mod C2FCR {
    pub use super::C0FCR::DTEF;
    pub use super::C0FCR::HTF;
    pub use super::C0FCR::SUSPF;
    pub use super::C0FCR::TCF;
    pub use super::C0FCR::TOF;
    pub use super::C0FCR::ULEF;
    pub use super::C0FCR::USEF;
}

/// GPDMA channel 2 status register
pub mod C2SR {
    pub use super::C0SR::DTEF;
    pub use super::C0SR::FIFOL;
    pub use super::C0SR::HTF;
    pub use super::C0SR::IDLEF;
    pub use super::C0SR::SUSPF;
    pub use super::C0SR::TCF;
    pub use super::C0SR::TOF;
    pub use super::C0SR::ULEF;
    pub use super::C0SR::USEF;
}

/// GPDMA channel 2 control register
pub mod C2CR {
    pub use super::C0CR::DTEIE;
    pub use super::C0CR::EN;
    pub use super::C0CR::HTIE;
    pub use super::C0CR::LAP;
    pub use super::C0CR::LSM;
    pub use super::C0CR::PRIO;
    pub use super::C0CR::RESET;
    pub use super::C0CR::SUSP;
    pub use super::C0CR::SUSPIE;
    pub use super::C0CR::TCIE;
    pub use super::C0CR::TOIE;
    pub use super::C0CR::ULEIE;
    pub use super::C0CR::USEIE;
}

/// GPDMA channel 2 transfer register 1
pub mod C2TR1 {
    pub use super::C0TR1::DAP;
    pub use super::C0TR1::DBL_1;
    pub use super::C0TR1::DBX;
    pub use super::C0TR1::DDW_LOG2;
    pub use super::C0TR1::DHX;
    pub use super::C0TR1::DINC;
    pub use super::C0TR1::PAM;
    pub use super::C0TR1::SAP;
    pub use super::C0TR1::SBL_1;
    pub use super::C0TR1::SBX;
    pub use super::C0TR1::SDW_LOG2;
    pub use super::C0TR1::SINC;
}

/// GPDMA channel 2 transfer register 2
pub mod C2TR2 {
    pub use super::C0TR2::BREQ;
    pub use super::C0TR2::DREQ;
    pub use super::C0TR2::PFREQ;
    pub use super::C0TR2::REQSEL;
    pub use super::C0TR2::SWREQ;
    pub use super::C0TR2::TCEM;
    pub use super::C0TR2::TRIGM;
    pub use super::C0TR2::TRIGPOL;
    pub use super::C0TR2::TRIGSEL;
}

/// GPDMA channel 2 block register 1
pub mod C2BR1 {
    pub use super::C0BR1::BNDT;
}

/// GPDMA channel 2 source address register
pub mod C2SAR {
    pub use super::C0SAR::SA;
}

/// GPDMA channel 2 destination address register
pub mod C2DAR {
    pub use super::C0DAR::DA;
}

/// GPDMA channel 2 linked-list address register
pub mod C2LLR {
    pub use super::C0LLR::LA;
    pub use super::C0LLR::UB1;
    pub use super::C0LLR::UDA;
    pub use super::C0LLR::ULL;
    pub use super::C0LLR::USA;
    pub use super::C0LLR::UT1;
    pub use super::C0LLR::UT2;
}

/// GPDMA channel 3 linked-list base address register
pub mod C3LBAR {
    pub use super::C0LBAR::LBA;
}

/// GPDMA channel 3 flag clear register
pub mod C3FCR {
    pub use super::C0FCR::DTEF;
    pub use super::C0FCR::HTF;
    pub use super::C0FCR::SUSPF;
    pub use super::C0FCR::TCF;
    pub use super::C0FCR::TOF;
    pub use super::C0FCR::ULEF;
    pub use super::C0FCR::USEF;
}

/// GPDMA channel 3 status register
pub mod C3SR {
    pub use super::C0SR::DTEF;
    pub use super::C0SR::FIFOL;
    pub use super::C0SR::HTF;
    pub use super::C0SR::IDLEF;
    pub use super::C0SR::SUSPF;
    pub use super::C0SR::TCF;
    pub use super::C0SR::TOF;
    pub use super::C0SR::ULEF;
    pub use super::C0SR::USEF;
}

/// GPDMA channel 3 control register
pub mod C3CR {
    pub use super::C0CR::DTEIE;
    pub use super::C0CR::EN;
    pub use super::C0CR::HTIE;
    pub use super::C0CR::LAP;
    pub use super::C0CR::LSM;
    pub use super::C0CR::PRIO;
    pub use super::C0CR::RESET;
    pub use super::C0CR::SUSP;
    pub use super::C0CR::SUSPIE;
    pub use super::C0CR::TCIE;
    pub use super::C0CR::TOIE;
    pub use super::C0CR::ULEIE;
    pub use super::C0CR::USEIE;
}

/// GPDMA channel 3 transfer register 1
pub mod C3TR1 {
    pub use super::C0TR1::DAP;
    pub use super::C0TR1::DBL_1;
    pub use super::C0TR1::DBX;
    pub use super::C0TR1::DDW_LOG2;
    pub use super::C0TR1::DHX;
    pub use super::C0TR1::DINC;
    pub use super::C0TR1::PAM;
    pub use super::C0TR1::SAP;
    pub use super::C0TR1::SBL_1;
    pub use super::C0TR1::SBX;
    pub use super::C0TR1::SDW_LOG2;
    pub use super::C0TR1::SINC;
}

/// GPDMA channel 3 transfer register 2
pub mod C3TR2 {
    pub use super::C0TR2::BREQ;
    pub use super::C0TR2::DREQ;
    pub use super::C0TR2::PFREQ;
    pub use super::C0TR2::REQSEL;
    pub use super::C0TR2::SWREQ;
    pub use super::C0TR2::TCEM;
    pub use super::C0TR2::TRIGM;
    pub use super::C0TR2::TRIGPOL;
    pub use super::C0TR2::TRIGSEL;
}

/// GPDMA channel 3 block register 1
pub mod C3BR1 {
    pub use super::C0BR1::BNDT;
}

/// GPDMA channel 3 source address register
pub mod C3SAR {
    pub use super::C0SAR::SA;
}

/// GPDMA channel 3 destination address register
pub mod C3DAR {
    pub use super::C0DAR::DA;
}

/// GPDMA channel 3 linked-list address register
pub mod C3LLR {
    pub use super::C0LLR::LA;
    pub use super::C0LLR::UB1;
    pub use super::C0LLR::UDA;
    pub use super::C0LLR::ULL;
    pub use super::C0LLR::USA;
    pub use super::C0LLR::UT1;
    pub use super::C0LLR::UT2;
}

/// GPDMA channel 4 linked-list base address register
pub mod C4LBAR {
    pub use super::C0LBAR::LBA;
}

/// GPDMA channel 4 flag clear register
pub mod C4FCR {
    pub use super::C0FCR::DTEF;
    pub use super::C0FCR::HTF;
    pub use super::C0FCR::SUSPF;
    pub use super::C0FCR::TCF;
    pub use super::C0FCR::TOF;
    pub use super::C0FCR::ULEF;
    pub use super::C0FCR::USEF;
}

/// GPDMA channel 4 status register
pub mod C4SR {
    pub use super::C0SR::DTEF;
    pub use super::C0SR::FIFOL;
    pub use super::C0SR::HTF;
    pub use super::C0SR::IDLEF;
    pub use super::C0SR::SUSPF;
    pub use super::C0SR::TCF;
    pub use super::C0SR::TOF;
    pub use super::C0SR::ULEF;
    pub use super::C0SR::USEF;
}

/// GPDMA channel 4 control register
pub mod C4CR {
    pub use super::C0CR::DTEIE;
    pub use super::C0CR::EN;
    pub use super::C0CR::HTIE;
    pub use super::C0CR::LAP;
    pub use super::C0CR::LSM;
    pub use super::C0CR::PRIO;
    pub use super::C0CR::RESET;
    pub use super::C0CR::SUSP;
    pub use super::C0CR::SUSPIE;
    pub use super::C0CR::TCIE;
    pub use super::C0CR::TOIE;
    pub use super::C0CR::ULEIE;
    pub use super::C0CR::USEIE;
}

/// GPDMA channel 4 transfer register 1
pub mod C4TR1 {
    pub use super::C0TR1::DAP;
    pub use super::C0TR1::DBL_1;
    pub use super::C0TR1::DBX;
    pub use super::C0TR1::DDW_LOG2;
    pub use super::C0TR1::DHX;
    pub use super::C0TR1::DINC;
    pub use super::C0TR1::PAM;
    pub use super::C0TR1::SAP;
    pub use super::C0TR1::SBL_1;
    pub use super::C0TR1::SBX;
    pub use super::C0TR1::SDW_LOG2;
    pub use super::C0TR1::SINC;
}

/// GPDMA channel 4 transfer register 2
pub mod C4TR2 {
    pub use super::C0TR2::BREQ;
    pub use super::C0TR2::DREQ;
    pub use super::C0TR2::PFREQ;
    pub use super::C0TR2::REQSEL;
    pub use super::C0TR2::SWREQ;
    pub use super::C0TR2::TCEM;
    pub use super::C0TR2::TRIGM;
    pub use super::C0TR2::TRIGPOL;
    pub use super::C0TR2::TRIGSEL;
}

/// GPDMA channel 4 block register 1
pub mod C4BR1 {
    pub use super::C0BR1::BNDT;
}

/// GPDMA channel 4 source address register
pub mod C4SAR {
    pub use super::C0SAR::SA;
}

/// GPDMA channel 4 destination address register
pub mod C4DAR {
    pub use super::C0DAR::DA;
}

/// GPDMA channel 4 linked-list address register
pub mod C4LLR {
    pub use super::C0LLR::LA;
    pub use super::C0LLR::UB1;
    pub use super::C0LLR::UDA;
    pub use super::C0LLR::ULL;
    pub use super::C0LLR::USA;
    pub use super::C0LLR::UT1;
    pub use super::C0LLR::UT2;
}

/// GPDMA channel 5 linked-list base address register
pub mod C5LBAR {
    pub use super::C0LBAR::LBA;
}

/// GPDMA channel 5 flag clear register
pub mod C5FCR {
    pub use super::C0FCR::DTEF;
    pub use super::C0FCR::HTF;
    pub use super::C0FCR::SUSPF;
    pub use super::C0FCR::TCF;
    pub use super::C0FCR::TOF;
    pub use super::C0FCR::ULEF;
    pub use super::C0FCR::USEF;
}

/// GPDMA channel 5 status register
pub mod C5SR {
    pub use super::C0SR::DTEF;
    pub use super::C0SR::FIFOL;
    pub use super::C0SR::HTF;
    pub use super::C0SR::IDLEF;
    pub use super::C0SR::SUSPF;
    pub use super::C0SR::TCF;
    pub use super::C0SR::TOF;
    pub use super::C0SR::ULEF;
    pub use super::C0SR::USEF;
}

/// GPDMA channel 5 control register
pub mod C5CR {
    pub use super::C0CR::DTEIE;
    pub use super::C0CR::EN;
    pub use super::C0CR::HTIE;
    pub use super::C0CR::LAP;
    pub use super::C0CR::LSM;
    pub use super::C0CR::PRIO;
    pub use super::C0CR::RESET;
    pub use super::C0CR::SUSP;
    pub use super::C0CR::SUSPIE;
    pub use super::C0CR::TCIE;
    pub use super::C0CR::TOIE;
    pub use super::C0CR::ULEIE;
    pub use super::C0CR::USEIE;
}

/// GPDMA channel 5 transfer register 1
pub mod C5TR1 {
    pub use super::C0TR1::DAP;
    pub use super::C0TR1::DBL_1;
    pub use super::C0TR1::DBX;
    pub use super::C0TR1::DDW_LOG2;
    pub use super::C0TR1::DHX;
    pub use super::C0TR1::DINC;
    pub use super::C0TR1::PAM;
    pub use super::C0TR1::SAP;
    pub use super::C0TR1::SBL_1;
    pub use super::C0TR1::SBX;
    pub use super::C0TR1::SDW_LOG2;
    pub use super::C0TR1::SINC;
}

/// GPDMA channel 5 transfer register 2
pub mod C5TR2 {
    pub use super::C0TR2::BREQ;
    pub use super::C0TR2::DREQ;
    pub use super::C0TR2::PFREQ;
    pub use super::C0TR2::REQSEL;
    pub use super::C0TR2::SWREQ;
    pub use super::C0TR2::TCEM;
    pub use super::C0TR2::TRIGM;
    pub use super::C0TR2::TRIGPOL;
    pub use super::C0TR2::TRIGSEL;
}

/// GPDMA channel 5 block register 1
pub mod C5BR1 {
    pub use super::C0BR1::BNDT;
}

/// GPDMA channel 5 source address register
pub mod C5SAR {
    pub use super::C0SAR::SA;
}

/// GPDMA channel 5 destination address register
pub mod C5DAR {
    pub use super::C0DAR::DA;
}

/// GPDMA channel 5 linked-list address register
pub mod C5LLR {
    pub use super::C0LLR::LA;
    pub use super::C0LLR::UB1;
    pub use super::C0LLR::UDA;
    pub use super::C0LLR::ULL;
    pub use super::C0LLR::USA;
    pub use super::C0LLR::UT1;
    pub use super::C0LLR::UT2;
}

/// GPDMA channel 6 linked-list base address register
pub mod C6LBAR {
    pub use super::C0LBAR::LBA;
}

/// GPDMA channel 6 flag clear register
pub mod C6FCR {
    pub use super::C0FCR::DTEF;
    pub use super::C0FCR::HTF;
    pub use super::C0FCR::SUSPF;
    pub use super::C0FCR::TCF;
    pub use super::C0FCR::TOF;
    pub use super::C0FCR::ULEF;
    pub use super::C0FCR::USEF;
}

/// GPDMA channel 6 status register
pub mod C6SR {
    pub use super::C0SR::DTEF;
    pub use super::C0SR::FIFOL;
    pub use super::C0SR::HTF;
    pub use super::C0SR::IDLEF;
    pub use super::C0SR::SUSPF;
    pub use super::C0SR::TCF;
    pub use super::C0SR::TOF;
    pub use super::C0SR::ULEF;
    pub use super::C0SR::USEF;
}

/// GPDMA channel 6 control register
pub mod C6CR {
    pub use super::C0CR::DTEIE;
    pub use super::C0CR::EN;
    pub use super::C0CR::HTIE;
    pub use super::C0CR::LAP;
    pub use super::C0CR::LSM;
    pub use super::C0CR::PRIO;
    pub use super::C0CR::RESET;
    pub use super::C0CR::SUSP;
    pub use super::C0CR::SUSPIE;
    pub use super::C0CR::TCIE;
    pub use super::C0CR::TOIE;
    pub use super::C0CR::ULEIE;
    pub use super::C0CR::USEIE;
}

/// GPDMA channel 6 transfer register 1
pub mod C6TR1 {
    pub use super::C0TR1::DAP;
    pub use super::C0TR1::DBL_1;
    pub use super::C0TR1::DBX;
    pub use super::C0TR1::DDW_LOG2;
    pub use super::C0TR1::DHX;
    pub use super::C0TR1::DINC;
    pub use super::C0TR1::PAM;
    pub use super::C0TR1::SAP;
    pub use super::C0TR1::SBL_1;
    pub use super::C0TR1::SBX;
    pub use super::C0TR1::SDW_LOG2;
    pub use super::C0TR1::SINC;
}

/// GPDMA channel 6 transfer register 2
pub mod C6TR2 {
    pub use super::C0TR2::BREQ;
    pub use super::C0TR2::DREQ;
    pub use super::C0TR2::PFREQ;
    pub use super::C0TR2::REQSEL;
    pub use super::C0TR2::SWREQ;
    pub use super::C0TR2::TCEM;
    pub use super::C0TR2::TRIGM;
    pub use super::C0TR2::TRIGPOL;
    pub use super::C0TR2::TRIGSEL;
}

/// GPDMA channel 6 alternate block register 1
pub mod C6BR1 {

    /// block number of data bytes to transfer from the source Block size transferred from the source. When the channel is enabled, this field becomes read-only and is decremented, indicating the remaining number of data items in the current source block to be transferred. BNDT\[15:0\] is programmed in number of bytes, maximum source block size is 64 Kbytes -1. Once the last data transfer is completed (BNDT\[15:0\] = 0): - if GPDMA_CxLLR.UB1 = 1, this field is updated by the LLI in the memory. - if GPDMA_CxLLR.UB1 = 0 and if there is at least one not null Uxx update bit, this field is internally restored to the programmed value. - if all GPDMA_CxLLR.Uxx = 0 and if GPDMA_CxLLR.LA\[15:0\] ≠ 0, this field is internally restored to the programmed value (infinite/continuous last LLI). - if GPDMA_CxLLR = 0, this field is kept as zero following the last LLI data transfer. Note: A non-null source block size must be a multiple of the source data width (BNDT\[2:0\] versus GPDMA_CxTR1.SDW_LOG2\[1:0\]). Else a user setting error is reported and no transfer is issued. Note: When configured in packing mode (GPDMA_CxTR1.PAM\[1\]=1 and destination data width different from source data width), a non-null source block size must be a multiple of the destination data width (BNDT\[2:0\] versus GPDMA_CxTR1.DDW_LOG2\[1:0\]). Else a user setting error is reported and no transfer is issued.
    pub mod BNDT {
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

    /// Block repeat counter This field contains the number of repetitions of the current block (0 to 2047). When the channel is enabled, this field becomes read-only. After decrements, this field indicates the remaining number of blocks, excluding the current one. This counter is hardware decremented for each completed block transfer. Once the last block transfer is completed (BRC\[10:0\] = BNDT\[15:0\] = 0): If GPDMA_CxLLR.UB1 = 1, all GPDMA_CxBR1 fields are updated by the next LLI in the memory. If GPDMA_CxLLR.UB1 = 0 and if there is at least one not null Uxx update bit, this field is internally restored to the programmed value. if all GPDMA_CxLLR.Uxx = 0 and if GPDMA_CxLLR.LA\[15:0\] ≠ 0, this field is internally restored to the programmed value (infinite/continuous last LLI). if GPDMA_CxLLR = 0, this field is kept as zero following the last LLI and data transfer.
    pub mod BRC {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (11 bits: 0x7ff << 16)
        pub const mask: u32 = 0x7ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// source address decrement
    pub mod SDEC {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (1 bit: 1 << 28)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// destination address decrement
    pub mod DDEC {
        /// Offset (29 bits)
        pub const offset: u32 = 29;
        /// Mask (1 bit: 1 << 29)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Block repeat source address decrement Note: On top of this increment/decrement (depending on BRSDEC), GPDMA_CxSAR is in the same time also updated by the increment/decrement (depending on SDEC) of the GPDMA_CxTR3.SAO value, as it is done after any programmed burst transfer.
    pub mod BRSDEC {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (1 bit: 1 << 30)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Block repeat destination address decrement Note: On top of this increment/decrement (depending on BRDDEC), GPDMA_CxDAR is in the same time also updated by the increment/decrement (depending on DDEC) of the GPDMA_CxTR3.DAO value, as it is usually done at the end of each programmed burst transfer.
    pub mod BRDDEC {
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

/// GPDMA channel 6 source address register
pub mod C6SAR {
    pub use super::C0SAR::SA;
}

/// GPDMA channel 6 destination address register
pub mod C6DAR {
    pub use super::C0DAR::DA;
}

/// GPDMA channel 6 transfer register 3
pub mod C6TR3 {

    /// source address offset increment The source address, pointed by GPDMA_CxSAR, is incremented or decremented (depending on GPDMA_CxBR1.SDEC) by this offset SAO\[12:0\] for each programmed source burst. This offset is not including and is added to the programmed burst size when the completed burst is addressed in incremented mode (GPDMA_CxTR1.SINC = 1). Note: A source address offset must be aligned with the programmed data width of a source burst (SAO\[2:0\] versus GPDMA_CxTR1.SDW_LOG2\[1:0\]). Else a user setting error is reported and none transfer is issued. Note: When the source block size is not a multiple of the destination burst size and is a multiple of the source data width, then the last programmed source burst is not completed and is internally shorten to match the block size. In this case, the additional GPDMA_CxTR3.SAO\[12:0\] is not applied.
    pub mod SAO {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (13 bits: 0x1fff << 0)
        pub const mask: u32 = 0x1fff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// destination address offset increment The destination address, pointed by GPDMA_CxDAR, is incremented or decremented (depending on GPDMA_CxBR1.DDEC) by this offset DAO\[12:0\] for each programmed destination burst. This offset is not including and is added to the programmed burst size when the completed burst is addressed in incremented mode (GPDMA_CxTR1.DINC = 1). Note: A destination address offset must be aligned with the programmed data width of a destination burst (DAO\[2:0\] versus GPDMA_CxTR1.DDW_LOG2\[1:0\]). Else, a user setting error is reported and no transfer is issued.
    pub mod DAO {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (13 bits: 0x1fff << 16)
        pub const mask: u32 = 0x1fff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// GPDMA channel 6 block register 2
pub mod C6BR2 {

    /// Block repeated source address offset For a channel with 2D addressing capability, this field is used to update (by addition or subtraction depending on GPDMA_CxBR1.BRSDEC) the current source address (GPDMA_CxSAR) at the end of a block transfer. A block repeated source address offset must be aligned with the programmed data width of a source burst (BRSAO\[2:0\] versus GPDMA_CxTR1.SDW_LOG2\[1:0\]). Else a user setting error is reported and no transfer is issued. Note: BRSAO\[15:0\] must be set to 0 in peripheral flow-control mode (if GPDMA_CxTR2.PFREQ = 1).
    pub mod BRSAO {
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

    /// Block repeated destination address offset For a channel with 2D addressing capability, this field is used to update (by addition or subtraction depending on GPDMA_CxBR1.BRDDEC) the current destination address (GPDMA_CxDAR) at the end of a block transfer. A block repeated destination address offset must be aligned with the programmed data width of a destination burst (BRDAO\[2:0\] versus GPDMA_CxTR1.DDW_LOG2\[1:0\]). Else a user setting error is reported and no transfer is issued. Note: BRDAO\[15:0\] must be set to 0 in peripheral flow-control mode (if GPDMA_CxTR2.PFREQ = 1).
    pub mod BRDAO {
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

/// GPDMA channel 6 alternate linked-list address register
pub mod C6LLR {

    /// pointer (16-bit low-significant address) to the next linked-list data structure If UT1 = UT2 = UB1 = USA = UDA = ULL = 0 and if LA\[15:20\] = 0, the current LLI is the last one. The channel transfer is completed without any update of the linked-list GPDMA register file. Else, this field is the pointer to the memory address offset from which the next linked-list data structure is automatically fetched from, once the data transfer is completed, in order to conditionally update the linked-list GPDMA internal register file (GPDMA_CxCTR1, GPDMA_CxTR2, GPDMA_CxBR1, GPDMA_CxSAR, GPDMA_CxDAR and GPDMA_CxLLR). Note: The user must program the pointer to be 32-bit aligned. The two low-significant bits are write ignored.
    pub mod LA {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (14 bits: 0x3fff << 2)
        pub const mask: u32 = 0x3fff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Update GPDMA_CxLLR register from memory This bit is used to control the update of GPDMA_CxLLR from the memory during the link transfer.
    pub mod ULL {
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

    /// Update GPDMA_CxBR2 from memory This bit controls the update of GPDMA_CxBR2 from the memory during the link transfer.
    pub mod UB2 {
        /// Offset (25 bits)
        pub const offset: u32 = 25;
        /// Mask (1 bit: 1 << 25)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Update GPDMA_CxTR3 from memory This bit controls the update of GPDMA_CxTR3 from the memory during the link transfer.
    pub mod UT3 {
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

    /// Update GPDMA_CxDAR register from memory This bit is used to control the update of GPDMA_CxDAR from the memory during the link transfer.
    pub mod UDA {
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

    /// update GPDMA_CxSAR from memory This bit controls the update of GPDMA_CxSAR from the memory during the link transfer.
    pub mod USA {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (1 bit: 1 << 28)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Update GPDMA_CxBR1 from memory This bit controls the update of GPDMA_CxBR1 from the memory during the link transfer. If UB1 = 0 and if GPDMA_CxLLR ≠ 0, the linked-list is not completed. GPDMA_CxBR1.BNDT\[15:0\] is then restored to the programmed value after data transfer is completed and before the link transfer.
    pub mod UB1 {
        /// Offset (29 bits)
        pub const offset: u32 = 29;
        /// Mask (1 bit: 1 << 29)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Update GPDMA_CxTR2 from memory This bit controls the update of GPDMA_CxTR2 from the memory during the link transfer.
    pub mod UT2 {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (1 bit: 1 << 30)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Update GPDMA_CxTR1 from memory This bit controls the update of GPDMA_CxTR1 from the memory during the link transfer.
    pub mod UT1 {
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

/// GPDMA channel 7 linked-list base address register
pub mod C7LBAR {
    pub use super::C0LBAR::LBA;
}

/// GPDMA channel 7 flag clear register
pub mod C7FCR {
    pub use super::C0FCR::DTEF;
    pub use super::C0FCR::HTF;
    pub use super::C0FCR::SUSPF;
    pub use super::C0FCR::TCF;
    pub use super::C0FCR::TOF;
    pub use super::C0FCR::ULEF;
    pub use super::C0FCR::USEF;
}

/// GPDMA channel 7 status register
pub mod C7SR {
    pub use super::C0SR::DTEF;
    pub use super::C0SR::FIFOL;
    pub use super::C0SR::HTF;
    pub use super::C0SR::IDLEF;
    pub use super::C0SR::SUSPF;
    pub use super::C0SR::TCF;
    pub use super::C0SR::TOF;
    pub use super::C0SR::ULEF;
    pub use super::C0SR::USEF;
}

/// GPDMA channel 7 control register
pub mod C7CR {
    pub use super::C0CR::DTEIE;
    pub use super::C0CR::EN;
    pub use super::C0CR::HTIE;
    pub use super::C0CR::LAP;
    pub use super::C0CR::LSM;
    pub use super::C0CR::PRIO;
    pub use super::C0CR::RESET;
    pub use super::C0CR::SUSP;
    pub use super::C0CR::SUSPIE;
    pub use super::C0CR::TCIE;
    pub use super::C0CR::TOIE;
    pub use super::C0CR::ULEIE;
    pub use super::C0CR::USEIE;
}

/// GPDMA channel 7 transfer register 1
pub mod C7TR1 {
    pub use super::C0TR1::DAP;
    pub use super::C0TR1::DBL_1;
    pub use super::C0TR1::DBX;
    pub use super::C0TR1::DDW_LOG2;
    pub use super::C0TR1::DHX;
    pub use super::C0TR1::DINC;
    pub use super::C0TR1::PAM;
    pub use super::C0TR1::SAP;
    pub use super::C0TR1::SBL_1;
    pub use super::C0TR1::SBX;
    pub use super::C0TR1::SDW_LOG2;
    pub use super::C0TR1::SINC;
}

/// GPDMA channel 7 transfer register 2
pub mod C7TR2 {
    pub use super::C0TR2::BREQ;
    pub use super::C0TR2::DREQ;
    pub use super::C0TR2::PFREQ;
    pub use super::C0TR2::REQSEL;
    pub use super::C0TR2::SWREQ;
    pub use super::C0TR2::TCEM;
    pub use super::C0TR2::TRIGM;
    pub use super::C0TR2::TRIGPOL;
    pub use super::C0TR2::TRIGSEL;
}

/// GPDMA channel 7 alternate block register 1
pub mod C7BR1 {
    pub use super::C6BR1::BNDT;
    pub use super::C6BR1::BRC;
    pub use super::C6BR1::BRDDEC;
    pub use super::C6BR1::BRSDEC;
    pub use super::C6BR1::DDEC;
    pub use super::C6BR1::SDEC;
}

/// GPDMA channel 7 source address register
pub mod C7SAR {
    pub use super::C0SAR::SA;
}

/// GPDMA channel 7 destination address register
pub mod C7DAR {
    pub use super::C0DAR::DA;
}

/// GPDMA channel 7 transfer register 3
pub mod C7TR3 {
    pub use super::C6TR3::DAO;
    pub use super::C6TR3::SAO;
}

/// GPDMA channel 7 block register 2
pub mod C7BR2 {
    pub use super::C6BR2::BRDAO;
    pub use super::C6BR2::BRSAO;
}

/// GPDMA channel 7 alternate linked-list address register
pub mod C7LLR {
    pub use super::C6LLR::LA;
    pub use super::C6LLR::UB1;
    pub use super::C6LLR::UB2;
    pub use super::C6LLR::UDA;
    pub use super::C6LLR::ULL;
    pub use super::C6LLR::USA;
    pub use super::C6LLR::UT1;
    pub use super::C6LLR::UT2;
    pub use super::C6LLR::UT3;
}
#[repr(C)]
pub struct RegisterBlock {
    _reserved1: [u8; 4],

    /// GPDMA privileged configuration register
    pub PRIVCFGR: RWRegister<u32>,

    _reserved2: [u8; 4],

    /// GPDMA masked interrupt status register
    pub MISR: RORegister<u32>,

    _reserved3: [u8; 64],

    /// GPDMA channel 0 linked-list base address register
    pub C0LBAR: RWRegister<u32>,

    _reserved4: [u8; 8],

    /// GPDMA channel 0 flag clear register
    pub C0FCR: WORegister<u32>,

    /// GPDMA channel 0 status register
    pub C0SR: RORegister<u32>,

    /// GPDMA channel 0 control register
    pub C0CR: RWRegister<u32>,

    _reserved5: [u8; 40],

    /// GPDMA channel 0 transfer register 1
    pub C0TR1: RWRegister<u32>,

    /// GPDMA channel 0 transfer register 2
    pub C0TR2: RWRegister<u32>,

    /// GPDMA channel 0 block register 1
    pub C0BR1: RWRegister<u32>,

    /// GPDMA channel 0 source address register
    pub C0SAR: RWRegister<u32>,

    /// GPDMA channel 0 destination address register
    pub C0DAR: RWRegister<u32>,

    _reserved6: [u8; 40],

    /// GPDMA channel 0 linked-list address register
    pub C0LLR: RWRegister<u32>,

    /// GPDMA channel 1 linked-list base address register
    pub C1LBAR: RWRegister<u32>,

    _reserved7: [u8; 8],

    /// GPDMA channel 1 flag clear register
    pub C1FCR: WORegister<u32>,

    /// GPDMA channel 1 status register
    pub C1SR: RORegister<u32>,

    /// GPDMA channel 1 control register
    pub C1CR: RWRegister<u32>,

    _reserved8: [u8; 40],

    /// GPDMA channel 1 transfer register 1
    pub C1TR1: RWRegister<u32>,

    /// GPDMA channel 1 transfer register 2
    pub C1TR2: RWRegister<u32>,

    /// GPDMA channel 1 block register 1
    pub C1BR1: RWRegister<u32>,

    /// GPDMA channel 1 source address register
    pub C1SAR: RWRegister<u32>,

    /// GPDMA channel 1 destination address register
    pub C1DAR: RWRegister<u32>,

    _reserved9: [u8; 40],

    /// GPDMA channel 1 linked-list address register
    pub C1LLR: RWRegister<u32>,

    /// GPDMA channel 2 linked-list base address register
    pub C2LBAR: RWRegister<u32>,

    _reserved10: [u8; 8],

    /// GPDMA channel 2 flag clear register
    pub C2FCR: WORegister<u32>,

    /// GPDMA channel 2 status register
    pub C2SR: RORegister<u32>,

    /// GPDMA channel 2 control register
    pub C2CR: RWRegister<u32>,

    _reserved11: [u8; 40],

    /// GPDMA channel 2 transfer register 1
    pub C2TR1: RWRegister<u32>,

    /// GPDMA channel 2 transfer register 2
    pub C2TR2: RWRegister<u32>,

    /// GPDMA channel 2 block register 1
    pub C2BR1: RWRegister<u32>,

    /// GPDMA channel 2 source address register
    pub C2SAR: RWRegister<u32>,

    /// GPDMA channel 2 destination address register
    pub C2DAR: RWRegister<u32>,

    _reserved12: [u8; 40],

    /// GPDMA channel 2 linked-list address register
    pub C2LLR: RWRegister<u32>,

    /// GPDMA channel 3 linked-list base address register
    pub C3LBAR: RWRegister<u32>,

    _reserved13: [u8; 8],

    /// GPDMA channel 3 flag clear register
    pub C3FCR: WORegister<u32>,

    /// GPDMA channel 3 status register
    pub C3SR: RORegister<u32>,

    /// GPDMA channel 3 control register
    pub C3CR: RWRegister<u32>,

    _reserved14: [u8; 40],

    /// GPDMA channel 3 transfer register 1
    pub C3TR1: RWRegister<u32>,

    /// GPDMA channel 3 transfer register 2
    pub C3TR2: RWRegister<u32>,

    /// GPDMA channel 3 block register 1
    pub C3BR1: RWRegister<u32>,

    /// GPDMA channel 3 source address register
    pub C3SAR: RWRegister<u32>,

    /// GPDMA channel 3 destination address register
    pub C3DAR: RWRegister<u32>,

    _reserved15: [u8; 40],

    /// GPDMA channel 3 linked-list address register
    pub C3LLR: RWRegister<u32>,

    /// GPDMA channel 4 linked-list base address register
    pub C4LBAR: RWRegister<u32>,

    _reserved16: [u8; 8],

    /// GPDMA channel 4 flag clear register
    pub C4FCR: WORegister<u32>,

    /// GPDMA channel 4 status register
    pub C4SR: RORegister<u32>,

    /// GPDMA channel 4 control register
    pub C4CR: RWRegister<u32>,

    _reserved17: [u8; 40],

    /// GPDMA channel 4 transfer register 1
    pub C4TR1: RWRegister<u32>,

    /// GPDMA channel 4 transfer register 2
    pub C4TR2: RWRegister<u32>,

    /// GPDMA channel 4 block register 1
    pub C4BR1: RWRegister<u32>,

    /// GPDMA channel 4 source address register
    pub C4SAR: RWRegister<u32>,

    /// GPDMA channel 4 destination address register
    pub C4DAR: RWRegister<u32>,

    _reserved18: [u8; 40],

    /// GPDMA channel 4 linked-list address register
    pub C4LLR: RWRegister<u32>,

    /// GPDMA channel 5 linked-list base address register
    pub C5LBAR: RWRegister<u32>,

    _reserved19: [u8; 8],

    /// GPDMA channel 5 flag clear register
    pub C5FCR: WORegister<u32>,

    /// GPDMA channel 5 status register
    pub C5SR: RORegister<u32>,

    /// GPDMA channel 5 control register
    pub C5CR: RWRegister<u32>,

    _reserved20: [u8; 40],

    /// GPDMA channel 5 transfer register 1
    pub C5TR1: RWRegister<u32>,

    /// GPDMA channel 5 transfer register 2
    pub C5TR2: RWRegister<u32>,

    /// GPDMA channel 5 block register 1
    pub C5BR1: RWRegister<u32>,

    /// GPDMA channel 5 source address register
    pub C5SAR: RWRegister<u32>,

    /// GPDMA channel 5 destination address register
    pub C5DAR: RWRegister<u32>,

    _reserved21: [u8; 40],

    /// GPDMA channel 5 linked-list address register
    pub C5LLR: RWRegister<u32>,

    /// GPDMA channel 6 linked-list base address register
    pub C6LBAR: RWRegister<u32>,

    _reserved22: [u8; 8],

    /// GPDMA channel 6 flag clear register
    pub C6FCR: WORegister<u32>,

    /// GPDMA channel 6 status register
    pub C6SR: RORegister<u32>,

    /// GPDMA channel 6 control register
    pub C6CR: RWRegister<u32>,

    _reserved23: [u8; 40],

    /// GPDMA channel 6 transfer register 1
    pub C6TR1: RWRegister<u32>,

    /// GPDMA channel 6 transfer register 2
    pub C6TR2: RWRegister<u32>,

    /// GPDMA channel 6 alternate block register 1
    pub C6BR1: RWRegister<u32>,

    /// GPDMA channel 6 source address register
    pub C6SAR: RWRegister<u32>,

    /// GPDMA channel 6 destination address register
    pub C6DAR: RWRegister<u32>,

    /// GPDMA channel 6 transfer register 3
    pub C6TR3: RWRegister<u32>,

    /// GPDMA channel 6 block register 2
    pub C6BR2: RWRegister<u32>,

    _reserved24: [u8; 32],

    /// GPDMA channel 6 alternate linked-list address register
    pub C6LLR: RWRegister<u32>,

    /// GPDMA channel 7 linked-list base address register
    pub C7LBAR: RWRegister<u32>,

    _reserved25: [u8; 8],

    /// GPDMA channel 7 flag clear register
    pub C7FCR: WORegister<u32>,

    /// GPDMA channel 7 status register
    pub C7SR: RORegister<u32>,

    /// GPDMA channel 7 control register
    pub C7CR: RWRegister<u32>,

    _reserved26: [u8; 40],

    /// GPDMA channel 7 transfer register 1
    pub C7TR1: RWRegister<u32>,

    /// GPDMA channel 7 transfer register 2
    pub C7TR2: RWRegister<u32>,

    /// GPDMA channel 7 alternate block register 1
    pub C7BR1: RWRegister<u32>,

    /// GPDMA channel 7 source address register
    pub C7SAR: RWRegister<u32>,

    /// GPDMA channel 7 destination address register
    pub C7DAR: RWRegister<u32>,

    /// GPDMA channel 7 transfer register 3
    pub C7TR3: RWRegister<u32>,

    /// GPDMA channel 7 block register 2
    pub C7BR2: RWRegister<u32>,

    _reserved27: [u8; 32],

    /// GPDMA channel 7 alternate linked-list address register
    pub C7LLR: RWRegister<u32>,
}
pub struct ResetValues {
    pub PRIVCFGR: u32,
    pub MISR: u32,
    pub C0LBAR: u32,
    pub C0FCR: u32,
    pub C0SR: u32,
    pub C0CR: u32,
    pub C0TR1: u32,
    pub C0TR2: u32,
    pub C0BR1: u32,
    pub C0SAR: u32,
    pub C0DAR: u32,
    pub C0LLR: u32,
    pub C1LBAR: u32,
    pub C1FCR: u32,
    pub C1SR: u32,
    pub C1CR: u32,
    pub C1TR1: u32,
    pub C1TR2: u32,
    pub C1BR1: u32,
    pub C1SAR: u32,
    pub C1DAR: u32,
    pub C1LLR: u32,
    pub C2LBAR: u32,
    pub C2FCR: u32,
    pub C2SR: u32,
    pub C2CR: u32,
    pub C2TR1: u32,
    pub C2TR2: u32,
    pub C2BR1: u32,
    pub C2SAR: u32,
    pub C2DAR: u32,
    pub C2LLR: u32,
    pub C3LBAR: u32,
    pub C3FCR: u32,
    pub C3SR: u32,
    pub C3CR: u32,
    pub C3TR1: u32,
    pub C3TR2: u32,
    pub C3BR1: u32,
    pub C3SAR: u32,
    pub C3DAR: u32,
    pub C3LLR: u32,
    pub C4LBAR: u32,
    pub C4FCR: u32,
    pub C4SR: u32,
    pub C4CR: u32,
    pub C4TR1: u32,
    pub C4TR2: u32,
    pub C4BR1: u32,
    pub C4SAR: u32,
    pub C4DAR: u32,
    pub C4LLR: u32,
    pub C5LBAR: u32,
    pub C5FCR: u32,
    pub C5SR: u32,
    pub C5CR: u32,
    pub C5TR1: u32,
    pub C5TR2: u32,
    pub C5BR1: u32,
    pub C5SAR: u32,
    pub C5DAR: u32,
    pub C5LLR: u32,
    pub C6LBAR: u32,
    pub C6FCR: u32,
    pub C6SR: u32,
    pub C6CR: u32,
    pub C6TR1: u32,
    pub C6TR2: u32,
    pub C6BR1: u32,
    pub C6SAR: u32,
    pub C6DAR: u32,
    pub C6TR3: u32,
    pub C6BR2: u32,
    pub C6LLR: u32,
    pub C7LBAR: u32,
    pub C7FCR: u32,
    pub C7SR: u32,
    pub C7CR: u32,
    pub C7TR1: u32,
    pub C7TR2: u32,
    pub C7BR1: u32,
    pub C7SAR: u32,
    pub C7DAR: u32,
    pub C7TR3: u32,
    pub C7BR2: u32,
    pub C7LLR: u32,
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

/// Access functions for the GPDMA1 peripheral instance
pub mod GPDMA1 {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x40020000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in GPDMA1
    pub const reset: ResetValues = ResetValues {
        PRIVCFGR: 0x00000000,
        MISR: 0x00000000,
        C0LBAR: 0x00000000,
        C0FCR: 0x00000000,
        C0SR: 0x00000001,
        C0CR: 0x00000000,
        C0TR1: 0x00000000,
        C0TR2: 0x00000000,
        C0BR1: 0x00000000,
        C0SAR: 0x00000000,
        C0DAR: 0x00000000,
        C0LLR: 0x00000000,
        C1LBAR: 0x00000000,
        C1FCR: 0x00000000,
        C1SR: 0x00000001,
        C1CR: 0x00000000,
        C1TR1: 0x00000000,
        C1TR2: 0x00000000,
        C1BR1: 0x00000000,
        C1SAR: 0x00000000,
        C1DAR: 0x00000000,
        C1LLR: 0x00000000,
        C2LBAR: 0x00000000,
        C2FCR: 0x00000000,
        C2SR: 0x00000001,
        C2CR: 0x00000000,
        C2TR1: 0x00000000,
        C2TR2: 0x00000000,
        C2BR1: 0x00000000,
        C2SAR: 0x00000000,
        C2DAR: 0x00000000,
        C2LLR: 0x00000000,
        C3LBAR: 0x00000000,
        C3FCR: 0x00000000,
        C3SR: 0x00000001,
        C3CR: 0x00000000,
        C3TR1: 0x00000000,
        C3TR2: 0x00000000,
        C3BR1: 0x00000000,
        C3SAR: 0x00000000,
        C3DAR: 0x00000000,
        C3LLR: 0x00000000,
        C4LBAR: 0x00000000,
        C4FCR: 0x00000000,
        C4SR: 0x00000001,
        C4CR: 0x00000000,
        C4TR1: 0x00000000,
        C4TR2: 0x00000000,
        C4BR1: 0x00000000,
        C4SAR: 0x00000000,
        C4DAR: 0x00000000,
        C4LLR: 0x00000000,
        C5LBAR: 0x00000000,
        C5FCR: 0x00000000,
        C5SR: 0x00000001,
        C5CR: 0x00000000,
        C5TR1: 0x00000000,
        C5TR2: 0x00000000,
        C5BR1: 0x00000000,
        C5SAR: 0x00000000,
        C5DAR: 0x00000000,
        C5LLR: 0x00000000,
        C6LBAR: 0x00000000,
        C6FCR: 0x00000000,
        C6SR: 0x00000001,
        C6CR: 0x00000000,
        C6TR1: 0x00000000,
        C6TR2: 0x00000000,
        C6BR1: 0x00000000,
        C6SAR: 0x00000000,
        C6DAR: 0x00000000,
        C6TR3: 0x00000000,
        C6BR2: 0x00000000,
        C6LLR: 0x00000000,
        C7LBAR: 0x00000000,
        C7FCR: 0x00000000,
        C7SR: 0x00000001,
        C7CR: 0x00000000,
        C7TR1: 0x00000000,
        C7TR2: 0x00000000,
        C7BR1: 0x00000000,
        C7SAR: 0x00000000,
        C7DAR: 0x00000000,
        C7TR3: 0x00000000,
        C7BR2: 0x00000000,
        C7LLR: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut GPDMA1_TAKEN: bool = false;

    /// Safe access to GPDMA1
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
            if GPDMA1_TAKEN {
                None
            } else {
                GPDMA1_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to GPDMA1
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if GPDMA1_TAKEN && inst.addr == INSTANCE.addr {
                GPDMA1_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal GPDMA1
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        GPDMA1_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to GPDMA1
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const GPDMA1: *const RegisterBlock = 0x40020000 as *const _;

/// Access functions for the GPDMA2 peripheral instance
pub mod GPDMA2 {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x40021000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in GPDMA2
    pub const reset: ResetValues = ResetValues {
        PRIVCFGR: 0x00000000,
        MISR: 0x00000000,
        C0LBAR: 0x00000000,
        C0FCR: 0x00000000,
        C0SR: 0x00000001,
        C0CR: 0x00000000,
        C0TR1: 0x00000000,
        C0TR2: 0x00000000,
        C0BR1: 0x00000000,
        C0SAR: 0x00000000,
        C0DAR: 0x00000000,
        C0LLR: 0x00000000,
        C1LBAR: 0x00000000,
        C1FCR: 0x00000000,
        C1SR: 0x00000001,
        C1CR: 0x00000000,
        C1TR1: 0x00000000,
        C1TR2: 0x00000000,
        C1BR1: 0x00000000,
        C1SAR: 0x00000000,
        C1DAR: 0x00000000,
        C1LLR: 0x00000000,
        C2LBAR: 0x00000000,
        C2FCR: 0x00000000,
        C2SR: 0x00000001,
        C2CR: 0x00000000,
        C2TR1: 0x00000000,
        C2TR2: 0x00000000,
        C2BR1: 0x00000000,
        C2SAR: 0x00000000,
        C2DAR: 0x00000000,
        C2LLR: 0x00000000,
        C3LBAR: 0x00000000,
        C3FCR: 0x00000000,
        C3SR: 0x00000001,
        C3CR: 0x00000000,
        C3TR1: 0x00000000,
        C3TR2: 0x00000000,
        C3BR1: 0x00000000,
        C3SAR: 0x00000000,
        C3DAR: 0x00000000,
        C3LLR: 0x00000000,
        C4LBAR: 0x00000000,
        C4FCR: 0x00000000,
        C4SR: 0x00000001,
        C4CR: 0x00000000,
        C4TR1: 0x00000000,
        C4TR2: 0x00000000,
        C4BR1: 0x00000000,
        C4SAR: 0x00000000,
        C4DAR: 0x00000000,
        C4LLR: 0x00000000,
        C5LBAR: 0x00000000,
        C5FCR: 0x00000000,
        C5SR: 0x00000001,
        C5CR: 0x00000000,
        C5TR1: 0x00000000,
        C5TR2: 0x00000000,
        C5BR1: 0x00000000,
        C5SAR: 0x00000000,
        C5DAR: 0x00000000,
        C5LLR: 0x00000000,
        C6LBAR: 0x00000000,
        C6FCR: 0x00000000,
        C6SR: 0x00000001,
        C6CR: 0x00000000,
        C6TR1: 0x00000000,
        C6TR2: 0x00000000,
        C6BR1: 0x00000000,
        C6SAR: 0x00000000,
        C6DAR: 0x00000000,
        C6TR3: 0x00000000,
        C6BR2: 0x00000000,
        C6LLR: 0x00000000,
        C7LBAR: 0x00000000,
        C7FCR: 0x00000000,
        C7SR: 0x00000001,
        C7CR: 0x00000000,
        C7TR1: 0x00000000,
        C7TR2: 0x00000000,
        C7BR1: 0x00000000,
        C7SAR: 0x00000000,
        C7DAR: 0x00000000,
        C7TR3: 0x00000000,
        C7BR2: 0x00000000,
        C7LLR: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut GPDMA2_TAKEN: bool = false;

    /// Safe access to GPDMA2
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
            if GPDMA2_TAKEN {
                None
            } else {
                GPDMA2_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to GPDMA2
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if GPDMA2_TAKEN && inst.addr == INSTANCE.addr {
                GPDMA2_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal GPDMA2
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        GPDMA2_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to GPDMA2
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const GPDMA2: *const RegisterBlock = 0x40021000 as *const _;
