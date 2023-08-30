#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Improved inter-integrated circuit
//!
//! Used by: stm32h562, stm32h563, stm32h573

use crate::{RORegister, RWRegister, WORegister};
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// CR and CR_ALTERNATE
/// CR: I3C message control register
/// CR_ALTERNATE: I3C message control register alternate
pub mod CR {

    /// count of data to transfer during a read or write message, in bytes (whatever I3C is acting as controller/target) Linear encoding up to 64 Kbytes -1 ...
    pub mod DCNT {
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

    /// read / non-write message (when I3C is acting as controller) When I3C is acting as controller, this field is used if MTYPE\[3:0\]=0010 (private message) or MTYPE\[3:0\]=0011 (direct message) or MTYPE\[3:0\]=0100 (legacy I2C message), in order to emit the RnW bit on the I3C bus.
    pub mod RNW {
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

    /// 7-bit I3C dynamic / I2C static target address (when I3C is acting as controller) When I3C is acting as controller, this field is used if MTYPE\[3:0\]=0010 (private message) or MTYPE\[3:0\]=0011 (direct message) or MTYPE\[3:0\]=0100 (legacy I2C message)
    pub mod ADD {
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

    /// message type (whatever I3C is acting as controller/target) Bits\[26:0\] are ignored. After M2 error detection on an I3C SDR message, this is needed for SCL “stuck at” recovery. Bits\[26:0\] are ignored. If I3C_CFGR.EXITPTRN=1, an HDR exit pattern is emitted on the bus to generate an escalation fault. Bits\[23:17\] (ADD\[6:0\]) is the emitted 7-bit dynamic address. Bit\[16\] (RNW) is the emitted RnW bit. The transferred private message is: {S / S+7’h7E+RnW=0+Sr / Sr+*} + 7-bit DynAddr + RnW + (8-bit Data + T)* + Sr/P. After a S (START), depending on I3C_CFGR.NOARBH, the arbitrable header (7’h7E+RnW=0) is inserted or not. Sr+*: after a Sr (Repeated Start), the hardware automatically inserts (7’h7E+RnW=0) if needed, i.e. if it follows an I3C direct message without ending by a P (Stop). Bits\[23:17\] (ADD\[6:0\]) is the emitted 7-bit dynamic address. Bit\[16\] (RNW) is the emitted RnW bit. The transferred direct message is: Sr + 7-bit DynAddr + RnW + (8-bit Data + T)* + Sr/P Bits\[23:17\] (ADD\[6:0\]) is the emitted 7-bit static address. Bit\[16\] (RNW) is the emitted RnW bit. The transferred legacy I2C message is: {S / S+ 7’h7E+RnW=0 + Sr / Sr+*} + 7-bit StaAddr + RnW + (8-bit Data + T)* + Sr/P. After a S (START), depending on I3C_CFGR.NOARBH, the arbitrable header (7’h7E+RnW=0) is inserted or not. Sr+*: after a Sr (Repeated Start), the hardware automatically inserts (7’h7E+RnW=0) if needed, i.e. if it follows an I3C direct message without ending by a P (Stop). 1xxx: reserved (when I3C is acting as I3C controller, used when target) 0xxx: reserved {S +} 7’h02 addr + RnW=0 {S +} 7-bit I3C_DEVR0.DA\[6:0\] + RnW=0 after a bus available condition (the target first emits a START request), or once the controller drives a START. {S +} 7-bit I3C_DEVR0.DA\[6:0\] + RnW=1 (+Ack/Nack from controller) When acknowledged from controller, the next (optional, depending on I3C_BCR.BCR2) transmitted IBI payload data is defined by I3C_CR.DCNT\[15:0\] and must be consistently programmed vs the maximum IBI payload data size which is defined by I3C_IBIDR.IBIP\[2:0\]. Others: reserved
    pub mod MTYPE {
        /// Offset (27 bits)
        pub const offset: u32 = 27;
        /// Mask (4 bits: 0b1111 << 27)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// message end type (when the I3C is acting as controller)
    pub mod MEND {
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

    /// 8-bit CCC code (when I3C is acting as controller) If Bit\[23\]=CCC\[7\]=1, this is the 1st part of an I3C SDR direct CCC command. If Bit\[23\]=CCC\[7\]=0, this is an I3C SDR broadcast CCC command (including ENTDAA and ENTHDR0).
    pub mod CCC {
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

/// I3C configuration register
pub mod CFGR {

    /// I3C enable (whatever I3C is acting as controller/target) - Except registers, the peripheral is under reset (a.k.a. partial reset). - Before clearing EN, when I3C is acting as a controller, all the possible target requests must be disabled using DISEC CCC. - When I3C is acting as a target, software should not disable the I3C, unless a partial reset is needed. In this state, some register fields can not be modified (like CRINIT, HKSDAEN for the I3C_CFGR)
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

    /// initial controller/target role This bit can be modified only when I3C_CFGR.EN = 0. Once enabled by setting I3C_CFGR.EN = 1, I3C peripheral initially acts as an I3C target. I3C does not drive SCL line and does not enable SDA pull-up, until it eventually acquires the controller role. Once enabled by setting I3C_CFGR.EN = 1, I3C peripheral initially acts as a controller. It has the I3C controller role, so drives SCL line and enables SDA pull-up, until it eventually offers the controller role to an I3C secondary controller.
    pub mod CRINIT {
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

    /// no arbitrable header after a START (when I3C is acting as a controller) This bit can be modified only when there is no on-going frame. - The target address is emitted directly after a START in case of a legacy I2C message or an I3C SDR private read/write message. - This is a more performing option (when is useless the emission of the 0x7E arbitrable header), but this is to be used only when the controller is sure that the addressed target device can not emit concurrently an IBI or a controller-role request (to insure no misinterpretation and no potential conflict between the address emitted by the controller in open-drain mode and the same address a target device can emit after a START, for IBI or MR).
    pub mod NOARBH {
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

    /// HDR reset pattern enable (when I3C is acting as a controller) This bit can be modified only when there is no on-going frame.
    pub mod RSTPTRN {
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

    /// HDR Exit Pattern enable (when I3C is acting as a controller) This bit can be modified only when there is no on-going frame. This is used to send only the header to test ownership of the bus when there is a suspicion of problem after controller-role hand-off (new controller didn’t assert its controller-role by accessing the previous one in less than Activity State time). The HDR Exit Pattern is sent even if the message header {S/Sr + 0x7E addr + W } is ACKed.
    pub mod EXITPTRN {
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

    /// High-keeper enable on SDA line (when I3C is acting as a controller) This bit can be modified only when I3C_CFGR.EN=0.
    pub mod HKSDAEN {
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

    /// Hot Join request acknowledge (when I3C is acting as a controller) After the NACK, the message continues as initially programmed (the hot-joining target is aware of the NACK and surely emits another hot-join request later on). After the ACK, the message continues as initially programmed. The software is aware by the HJ interrupt (flag I3C_EVR.HJF is set) and initiates the ENTDAA sequence later on, potentially preventing others Hot Join requests with a Disable target events command (DISEC, with DISHJ=1). Independently of the HJACK configuration, further Hot Join request(s) are NACKed until the Hot Join flag, HJF, is cleared. However, a NACKed target can be assigned a dynamic address by the ENTDAA sequence initiated later on by the first HJ request, preventing this target to emit an HJ request again.
    pub mod HJACK {
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

    /// RX-FIFO DMA request enable (whatever I3C is acting as controller/target) - Software reads and pops a data byte/word from RX-FIFO i.e. reads I3C_RDR or I3C_RDWR register. - A next data byte/word is to be read by the software either via polling on the flag I3C_EVR.RXFNEF=1 or via interrupt notification (enabled by I3C_IER.RXFNEIE=1). - DMA reads and pops data byte(s)/word(s) from RX-FIFO i.e. reads I3C_RDR or I3C_RDWR register. - A next data byte/word is automatically read by the programmed hardware (i.e. via the asserted RX-FIFO DMA request from the I3C and the programmed DMA channel).
    pub mod RXDMAEN {
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

    /// RX-FIFO flush (whatever I3C is acting as controller/target) This bit can only be written.
    pub mod RXFLUSH {
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

    /// RX-FIFO threshold (whatever I3C is acting as controller/target) This threshold defines, compared to the RX-FIFO level, when the I3C_EVR.RXFNEF flag is set (and consequently if RXDMAEN=1 when is asserted a DMA RX request). RXFNEF is set when 1 byte is to be read in RX-FIFO (i.e. in I3C_RDR). RXFNEF is set when 4 bytes are to be read in RX-FIFO (i.e. in I3C_RDWR).
    pub mod RXTHRES {
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

    /// TX-FIFO DMA request enable (whatever I3C is acting as controller/target) - Software writes and pushes a data byte/word into TX-FIFO i.e. writes I3C_TDR or I3C_TDWR register, to be transmitted over the I3C bus. - A next data byte/word is to be written by the software either via polling on the flag I3C_EVR.TXFNFF=1 or via interrupt notification (enabled by I3C_IER.TXFNFIE=1). - DMA writes and pushes data byte(s)/word(s) into TX-FIFO i.e. writes I3C_TDR or I3C_TDWR register. - A next data byte/word transfer is automatically pushed by the programmed hardware (i.e. via the asserted TX-FIFO DMA request from the I3C and the programmed DMA channel).
    pub mod TXDMAEN {
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

    /// TX-FIFO flush (whatever I3C is acting as controller/target) This bit can only be written. When the I3C is acting as target, this bit can be used to flush the TX-FIFO on a private read if the controller has early ended the read data (i.e. driven low the T bit) and there is/are remaining data in the TX-FIFO (i.e. I3C_SR.ABT=1 and I3C_SR.XDCNT\[15:0\] < I3C_TGTTDR.TGTTDCNT\[15:0\]).
    pub mod TXFLUSH {
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

    /// TX-FIFO threshold (whatever I3C is acting as controller/target) This threshold defines, compared to the TX-FIFO level, when the I3C_EVR.TXFNFF flag is set (and consequently if TXDMAEN=1 when is asserted a DMA TX request). TXFNFF is set when 1 byte is to be written in TX-FIFO (i.e. in I3C_TDR). TXFNFF is set when 4 bytes are to be written in TX-FIFO (i.e. in I3C_TDWR).
    pub mod TXTHRES {
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

    /// S-FIFO DMA request enable (when I3C is acting as controller) Condition: When RMODE=1 (FIFO is enabled for the status): - Software reads and pops a status word from S-FIFO i.e. reads I3C_SR register after a completed frame (I3C_EVR.FCF=1) or an error (I3C_EVR.ERRF=1). - A status word can be read by the software either via polling on these register flags or via interrupt notification (enabled by I3C_IER.FCIE=1 and I3C_IER.ERRIE=1). - DMA reads and pops status word(s) from S-FIFO i.e. reads I3C_SR register. - Status word(s) are automatically read by the programmed hardware (i.e. via the asserted S-FIFO DMA request from the I3C and the programmed DMA channel).
    pub mod SDMAEN {
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

    /// S-FIFO flush (when I3C is acting as controller) When I3C is acting as I3C controller, this bit can only be written (and is only used when I3C is acting as controller).
    pub mod SFLUSH {
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

    /// S-FIFO enable / status receive mode (when I3C is acting as controller) When I3C is acting as I3C controller, this bit is used for the enabling the FIFO for the status (S-FIFO) vs the received status from the target on the I3C bus. When I3C is acting as target, this bit must be cleared. - Status register (i.e. I3C_SR) is used without FIFO mechanism. - There is no SCL stretch if a new status register content is not read. - Status register must be read before being lost/overwritten. All message status must be read. There is SCL stretch when there is no more space in the S-FIFO.
    pub mod RMODE {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (1 bit: 1 << 18)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// transmit mode (when I3C is acting as controller) When I3C is acting as I3C controller, this bit is used for the C-FIFO and TX-FIFO management vs the emitted frame on the I3C bus. A frame transfer starts as soon as first control word is present in C-FIFO.
    pub mod TMODE {
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

    /// C-FIFO DMA request enable (when I3C is acting as controller) When I3C is acting as controller: - Software writes and pushes control word(s) into C-FIFO i.e. writes I3C_CR register, as needed for a given frame. - A next control word transfer can be written by software either via polling on the flag I3C_EVR.CFNFF=1 or via interrupt notification (enabled by I3C_IER.CFNFIE=1). - DMA writes and pushes control word(s) into C-FIFO i.e. writes I3C_CR register, as needed for a given frame. - A next control word transfer is automatically written by the programmed hardware (i.e. via the asserted C-FIFO DMA request from the I3C and the programmed DMA channel).
    pub mod CDMAEN {
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

    /// C-FIFO flush (when I3C is acting as controller) This bit can only be written.
    pub mod CFLUSH {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (1 bit: 1 << 21)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// frame transfer set (a.k.a. software trigger) (when I3C is acting as controller) This bit can only be written. When I3C is acting as I3C controller: Note: If this bit is not set, the other alternative for the software to initiate a frame transfer is to directly write the first control word register (i.e. I3C_CR) while C-FIFO is empty (i.e. I3C_EVR.CFEF=1). Then, if the first written control word is not tagged as a message end (i.e I3C_CR.MEND=0), it causes the hardware to assert the flag I3C_EVR.CFNFF (C-FIFO not full and a next control word is needed).
    pub mod TSFSET {
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

/// I3C receive data byte register
pub mod RDR {

    /// 8-bit received data on I3C bus.
    pub mod RDB0 {
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

/// I3C receive data word register
pub mod RDWR {

    /// 8-bit received data (earliest byte on I3C bus).
    pub mod RDB0 {
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

    /// 8-bit received data (next byte after RDB0 on I3C bus).
    pub mod RDB1 {
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

    /// 8-bit received data (next byte after RDB1 on I3C bus).
    pub mod RDB2 {
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

    /// 8-bit received data (latest byte on I3C bus).
    pub mod RDB3 {
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
}

/// I3C transmit data byte register
pub mod TDR {

    /// 8-bit data to transmit on I3C bus.
    pub mod TDB0 {
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

/// I3C transmit data word register
pub mod TDWR {

    /// 8-bit transmit data (earliest byte on I3C bus)
    pub mod TDB0 {
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

    /// 8-bit transmit data (next byte after TDB0\[7:0\] on I3C bus).
    pub mod TDB1 {
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

    /// 8-bit transmit data (next byte after TDB1\[7:0\] on I3C bus).
    pub mod TDB2 {
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

    /// 8-bit transmit data (latest byte on I3C bus).
    pub mod TDB3 {
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
}

/// I3C IBI payload data register
pub mod IBIDR {

    /// 8-bit IBI payload data (earliest byte on I3C bus, i.e. MDB\[7:0\] mandatory data byte).
    pub mod IBIDB0 {
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

    /// 8-bit IBI payload data (next byte on I3C bus after IBIDB0\[7:0\]).
    pub mod IBIDB1 {
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

    /// 8-bit IBI payload data (next byte on I3C bus after IBIDB1\[7:0\]).
    pub mod IBIDB2 {
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

    /// 8-bit IBI payload data (latest byte on I3C bus).
    pub mod IBIDB3 {
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
}

/// I3C target transmit configuration register
pub mod TGTTDR {

    /// transmit data counter, in bytes (when I3C is configured as target) This field must be written by software in the same access when is asserted PRELOAD, in order to define the number of bytes to preload and to transmit. This field is updated by hardware and reports, when read, the remaining number of bytes to be loaded into the TX-FIFO.
    pub mod TGTTDCNT {
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

    /// preload of the TX-FIFO (when I3C is configured as target) This bit must be written and asserted by software in the same access when is written and defined the number of bytes to preload into the TX-FIFO and to transmit. This bit is cleared by hardware when all the data bytes to transmit are loaded into the TX-FIFO.
    pub mod PRELOAD {
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
}

/// I3C status register
pub mod SR {

    /// data counter - When the I3C is acting as controller: number of targets detected on the bus - When the I3C is acting as target: number of transmitted bytes - Whatever the I3C is acting as controller or target: number of data bytes read from or transmitted on the I3C bus during the MID\[7:0\] message
    pub mod XDCNT {
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

    /// a private read message is completed/aborted prematurely by the target (when the I3C is acting as controller) When the I3C is acting as controller, this bit indicates if the private read data which is transmitted by the target early terminates (i.e. the target drives T bit low earlier vs what does expect the controller in terms of programmed number of read data bytes i.e. I3C_CR.DCNT\[15:0\]).
    pub mod ABT {
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

    /// message direction Whatever the I3C is acting as controller or target, this bit indicates the direction of the related message on the I3C bus Note: ENTDAA CCC is considered as a write command.
    pub mod DIR {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (1 bit: 1 << 18)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// message identifier/counter of a given frame (when the I3C is acting as controller) When the I3C is acting as controller, this field identifies the control word message (i.e. I3C_CR) to which the I3C_SR status register refers. First message of a frame is identified with MID\[7:0\]=0. This field is incremented (by hardware) on the completion of a new message control word (i.e. I3C_CR) over I3C bus. This field is reset for every new frame start.
    pub mod MID {
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
}

/// I3C status error register
pub mod SER {

    /// protocol error code/type controller detected an illegally formatted CCC controller detected that transmitted data on the bus is different from expected controller detected a not acknowledged broadcast address (7’hE) controller detected the new controller did not drive bus after controller-role hand-off target detected an invalid broadcast address 7’hE+W target detected a parity error on a CCC code via a parity check (vs T bit) target detected a parity error on a write data via a parity check (vs T bit) target detected a parity error on the assigned address during dynamic address arbitration via a parity check (vs PAR bit) target detected a 7’hE+R missing after Sr during dynamic address arbitration target detected an illegally formatted CCC target detected that transmitted data on the bus is different from expected others: reserved
    pub mod CODERR {
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

    /// protocol error
    pub mod PERR {
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

    /// SCL stall error (when the I3C is acting as target)
    pub mod STALL {
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

    /// RX-FIFO overrun or TX-FIFO underrun i) a TX-FIFO underrun: TX-FIFO is empty and a write data byte has to be transmitted ii) a RX-FIFO overrun: RX-FIFO is full and a new data byte is received
    pub mod DOVR {
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

    /// C-FIFO underrun or S-FIFO overrun (when the I3C is acting as controller) i) a C-FIFO underrun: control FIFO is empty and a restart has to be emitted ii) a S-FIFO overrun: S-FIFO is full and a new message ends
    pub mod COVR {
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

    /// address not acknowledged (when the I3C is configured as controller) i) a legacy I2C read/write transfer ii) a direct CCC write transfer iii) the second trial of a direct CCC read transfer iv) a private read/write transfer
    pub mod ANACK {
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

    /// data not acknowledged (when the I3C is acting as controller) i) a legacy I2C write transfer ii) the second trial when sending dynamic address during ENTDAA procedure
    pub mod DNACK {
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

    /// data error (when the I3C is acting as controller)
    pub mod DERR {
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
}

/// I3C received message register
pub mod RMR {

    /// IBI received payload data count (when the I3C is configured as controller) When the I3C is configured as controller, this field logs the number of data bytes effectively received in the I3C_IBIDR register.
    pub mod IBIRDCNT {
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

    /// received CCC code (when the I3C is configured as target) When the I3C is configured as target, this field logs the received CCC code.
    pub mod RCODE {
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

    /// received target address (when the I3C is configured as controller) When the I3C is configured as controller, this field logs the received dynamic address from the target during acknowledged IBI or controller-role request.
    pub mod RADD {
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

/// I3C event register
pub mod EVR {

    /// C-FIFO empty flag (whatever the I3C is acting as controller/target) This flag is asserted by hardware to indicate that the C-FIFO is empty when controller, and that the I3C_CR register contains no control word (i.e. none IBI/CR/HJ request) when target. This flag is de-asserted by hardware to indicate that the C-FIFO is not empty when controller, and that the I3C_CR register contains one control word (i.e. a pending IBI/CR/HJ request) when target. Note: When the I3C is acting as controller, if the C-FIFO and TX-FIFO preload is configured (i.e. I3C_CFGR.TMODE=1), the software must wait for TXFEF=1 and CFEF=1 before starting a new frame transfer.
    pub mod CFEF {
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

    /// TX-FIFO empty flag (whatever the I3C is acting as controller/target) This flag is asserted by hardware to indicate that the TX-FIFO is empty. This flag is de-asserted by hardware to indicate that the TX-FIFO is not empty. Note: When the I3C is acting as controller, if the C-FIFO and TX-FIFO preload is configured (i.e. I3C_CFGR.TMODE=1), the software must wait for TXFEF=1 and CFEF=1 before starting a new frame transfer.
    pub mod TXFEF {
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

    /// C-FIFO not full flag (when the I3C is acting as controller) When the I3C is acting as controller, this flag is asserted by hardware to indicate that a control word is to be written to the C-FIFO. This flag is de-asserted by hardware to indicate that a control word is not to be written to the C-FIFO. Note: The software must wait for CFNFF=1 (by polling or via the enabled interrupt) before writing to C-FIFO (i.e. writing to I3C_CR).
    pub mod CFNFF {
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

    /// S-FIFO not empty flag (when the I3C is acting as controller) When the I3C is acting as controller, if the S-FIFO is enabled (i.e. I3C_CFGR.RMODE=1), this flag is asserted by hardware to indicate that a status word is to be read from the S-FIFO. This flag is de-asserted by hardware to indicate that a status word is not to be read from the S-FIFO.
    pub mod SFNEF {
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

    /// TX-FIFO not full flag (whatever the I3C is acting as controller/target) This flag is asserted by hardware to indicate that a data byte/word is to be written to the TX-FIFO. This flag is de-asserted by hardware to indicate that a data byte/word is not to be written to the TX-FIFO. Note: The software must wait for TXFNFF=1 (by polling or via the enabled interrupt) before writing to TX-FIFO (i.e. writing to I3C_TDR or I3C_TDWR depending on I3C_CFGR.TXTHRES). Note: When the I3C is acting as target, if the software intends to use the TXFNFF flag for writing into I3C_TDR/I3C_TDWR, it must have configured and set the TX-FIFO preload (i.e. write I3C_TGTTDR.PRELOAD).
    pub mod TXFNFF {
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

    /// RX-FIFO not empty flag (whatever the I3C is acting as controller/target) This flag is asserted by hardware to indicate that a data byte is to be read from the RX-FIFO. This flag is de-asserted by hardware to indicate that a data byte is not to be read from the RX-FIFO. Note: The software must wait for RXFNEF=1 (by polling or via the enabled interrupt) before reading from RX-FIFO (i.e. writing to I3C_RDR or I3C_RDWR depending on I3C_CFGR.RXTHRES).
    pub mod RXFNEF {
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

    /// last written data byte/word flag (whatever the I3C is acting as controller/target) This flag is asserted by hardware to indicate that the last data byte/word (depending on I3C_CFGR.TXTHRES) of a message is to be written to the TX-FIFO. This flag is de-asserted by hardware when the last data byte/word of a message is written.
    pub mod TXLASTF {
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

    /// last read data byte/word flag (whatever the I3C is acting as controller/target) This flag is asserted by hardware to indicate that the last data byte/word (depending on I3C_CFGR.RXTHRES) of a message is to be read from the RX-FIFO. This flag is de-asserted by hardware when the last data byte/word of a message is read.
    pub mod RXLASTF {
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

    /// frame complete flag (whatever the I3C is acting as controller/target) When the I3C is acting as controller, this flag is asserted by hardware to indicate that a frame has been (normally) completed on the I3C bus, i.e when a stop is issued. When the I3C is acting as target, this flag is asserted by hardware to indicate that a message addressed to/by this target has been (normally) completed on the I3C bus, i.e when a next stop or repeated start is then issued by the controller. This flag is cleared when software writes 1 into corresponding I3C_CEVR.CFCF bit.
    pub mod FCF {
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

    /// target-initiated read end flag (when the I3C is acting as controller) When the I3C is acting as controller, this flag is asserted by hardware to indicate that the target has prematurely ended a read transfer. Then, software should read I3C_SR to get more information on the prematurely read transfer. This flag is cleared when software writes 1 into corresponding I3C_CEVR.CRXTGTENDF bit.
    pub mod RXTGTENDF {
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

    /// flag (whatever the I3C is acting as controller/target) This flag is asserted by hardware to indicate that an error occurred.Then, software should read I3C_SER to get the error type. This flag is cleared when software writes 1 into corresponding I3C_CEVR.CERRF bit.
    pub mod ERRF {
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

    /// IBI flag (when the I3C is acting as controller) When the I3C is acting as controller, this flag is asserted by hardware to indicate that an IBI request has been received. This flag is cleared when software writes 1 into corresponding I3C_CEVR.CIBIF bit.
    pub mod IBIF {
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

    /// IBI end flag (when the I3C is acting as target) When the I3C is acting as target, this flag is asserted by hardware to indicate that a IBI transfer has been received and completed (IBI acknowledged and IBI data bytes read by controller if any). This flag is cleared when software writes 1 into corresponding I3C_CEVR.CIBIENDF bit.
    pub mod IBIENDF {
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

    /// controller-role request flag (when the I3C is acting as controller) When the I3C is acting as controller, this flag is asserted by hardware to indicate that a controller-role request has been acknowledged and completed (by hardware). The software should then issue a GETACCCR CCC (get accept controller role) for the controller-role hand-off procedure. This flag is cleared when software writes 1 into corresponding I3C_CEVR.CCRF bit.
    pub mod CRF {
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

    /// controller-role update flag (when the I3C is acting as target) When the I3C is acting as target, this flag is asserted by hardware to indicate that it has now gained the controller role after the completed controller-role hand-off procedure. This flag is cleared when software writes 1 into corresponding I3C_CEVR.CCRUPDF bit.
    pub mod CRUPDF {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (1 bit: 1 << 18)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// hot-join flag (when the I3C is acting as controller) When the I3C is acting as controller, this flag is asserted by hardware to indicate that an hot join request has been received. This flag is cleared when software writes 1 into corresponding I3C_CEVR.CHJF bit.
    pub mod HJF {
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

    /// wakeup/missed start flag (when the I3C is acting as target) When the I3C is acting as target, this flag is asserted by hardware to indicate that a start has been detected (i.e. a SDA falling edge followed by a SCL falling edge) but on the next SCL falling edge, the I3C kernel clock is (still) gated. Thus an I3C bus transaction may have been lost by the target. The corresponding interrupt may be used to wakeup the device from a low power mode (Sleep or Stop mode). This flag is cleared when software writes 1 into corresponding I3C_CEVR.CWKPF bit.
    pub mod WKPF {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (1 bit: 1 << 21)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// get flag (when the I3C is acting as target) When the I3C is acting as target, this flag is asserted by hardware to indicate that any direct CCC of get type (GET*** CCC) has been received. This flag is cleared when software writes 1 into corresponding I3C_CEVR.CGETF bit.
    pub mod GETF {
        /// Offset (22 bits)
        pub const offset: u32 = 22;
        /// Mask (1 bit: 1 << 22)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// get status flag (when the I3C is acting as target) When the I3C is acting as target, this flag is asserted by hardware to indicate that a direct GETSTATUS CCC (get status) has been received. This flag is cleared when software writes 1 into corresponding I3C_CEVR.CSTAF bit.
    pub mod STAF {
        /// Offset (23 bits)
        pub const offset: u32 = 23;
        /// Mask (1 bit: 1 << 23)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// dynamic address update flag (when the I3C is acting as target) When the I3C is acting as target, this flag is asserted by hardware to indicate that a dynamic address update has been received via any of the broadcast ENTDAA, RSTDAA and direct SETNEWDA CCC. Then, software should read I3C_DEVR0.DA\[6:0\] to get the maximum write length value. This flag is cleared when software writes 1 into corresponding I3C_CEVR.CDAUPDF bit.
    pub mod DAUPDF {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (1 bit: 1 << 24)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// maximum write length update flag (when the I3C is acting as target) When the I3C is acting as target, this flag is asserted by hardware to indicate that a direct SETMWL CCC (set max write length) has been received. Then, software should read I3C_MAXWLR.MWL\[15:0\] to get the maximum write length value. This flag is cleared when software writes 1 into corresponding I3C_CEVR.CMWLUPDF bit.
    pub mod MWLUPDF {
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

    /// maximum read length update flag (when the I3C is acting as target) When the I3C is acting as target, this flag is asserted by hardware to indicate that a direct SETMRL CCC (set max read length) has been received. Then, software should read I3C_MAXRLR.MRL\[15:0\] to get the maximum read length value. This flag is cleared when software writes 1 into corresponding I3C_CEVR.CMRLUPDF bit.
    pub mod MRLUPDF {
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

    /// reset pattern flag (when the I3C is acting as target) When the I3C is acting as target, this flag is asserted by hardware to indicate that a reset pattern has been detected (i.e. 14 SDA transitions while SCL is low, followed by repeated start, then stop). Then, software should read I3C_DEVR0.RSTACT\[1:0\] and I3C_DEVR0.RSTVAL, to know what reset level is required. If RSTVAL=1: when the RSTF is asserted (and/or the corresponding interrupt if enabled), I3C_DEVR0.RSTACT\[1:0\] dictates the reset action to be performed by the software if any. If RSTVAL=0: when the RSTF is asserted (and/or the corresponding interrupt if enabled), the software should issue an I3C reset after a first detected reset pattern, and a system reset on the second one. The corresponding interrupt may be used to wakeup the device from a low power mode (Sleep or Stop mode). This flag is cleared when software writes 1 into corresponding I3C_CEVR.CRSTF bit.
    pub mod RSTF {
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

    /// activity state update flag (when the I3C is acting as target) When the I3C is acting as target, this flag is asserted by hardware to indicate that the direct or broadcast ENTASx CCC (with x=0...3) has been received. Then, software should read I3C_DEVR0.AS\[1:0\]. This flag is cleared when software writes 1 into corresponding I3C_CEVR.CASUPDF bit.
    pub mod ASUPDF {
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

    /// interrupt/controller-role/hot-join update flag (when the I3C is acting as target) When the I3C is acting as target, this flag is asserted by hardware to indicate that the direct or broadcast ENEC/DISEC CCC (enable/disable target events) has been received, where a target event is either an interrupt/IBI request, a controller-role request, or an hot-join request. Then, software should read respectively I3C_DEVR0.IBIEN, I3C_DEVR0.CREN or I3C_DEVR0.HJEN. This flag is cleared when software writes 1 into corresponding I3C_CEVR.CINTUPDF bit.
    pub mod INTUPDF {
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

    /// DEFTGTS flag (when the I3C is acting as target) When the I3C is acting as target (and is typically controller capable), this flag is asserted by hardware to indicate that the broadcast DEFTGTS CCC (define list of targets) has been received. Then, software may store the received data for when getting the controller role. This flag is cleared when software writes 1 into corresponding I3C_CEVR.CDEFF bit.
    pub mod DEFF {
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

    /// group addressing flag (when the I3C is acting as target) When the I3C is acting as target (and is typically controller capable), this flag is asserted by hardware to indicate that the broadcast DEFGRPA CCC (define list of group addresses) has been received. Then, software may store the received data for when getting the controller role. This flag is cleared when software writes 1 into corresponding I3C_CEVR.CGRPF bit.
    pub mod GRPF {
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

/// I3C interrupt enable register
pub mod IER {

    /// C-FIFO not full interrupt enable (whatever the I3C is acting as controller/target)
    pub mod CFNFIE {
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

    /// S-FIFO not empty interrupt enable (whatever the I3C is acting as controller/target)
    pub mod SFNEIE {
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

    /// TX-FIFO not full interrupt enable (whatever the I3C is acting as controller/target)
    pub mod TXFNFIE {
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

    /// RX-FIFO not empty interrupt enable (whatever the I3C is acting as controller/target)
    pub mod RXFNEIE {
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

    /// frame complete interrupt enable (whatever the I3C is acting as controller/target)
    pub mod FCIE {
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

    /// target-initiated read end interrupt enable (when the I3C is acting as controller)
    pub mod RXTGTENDIE {
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

    /// error interrupt enable (whatever the I3C is acting as controller/target)
    pub mod ERRIE {
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

    /// IBI request interrupt enable (when the I3C is acting as controller)
    pub mod IBIIE {
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

    /// IBI end interrupt enable (when the I3C is acting as target)
    pub mod IBIENDIE {
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

    /// controller-role request interrupt enable (when the I3C is acting as controller)
    pub mod CRIE {
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

    /// controller-role update interrupt enable (when the I3C is acting as target)
    pub mod CRUPDIE {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (1 bit: 1 << 18)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// hot-join interrupt enable (when the I3C is acting as controller)
    pub mod HJIE {
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

    /// wakeup interrupt enable (when the I3C is acting as target)
    pub mod WKPIE {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (1 bit: 1 << 21)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// GETxxx CCC interrupt enable (when the I3C is acting as target)
    pub mod GETIE {
        /// Offset (22 bits)
        pub const offset: u32 = 22;
        /// Mask (1 bit: 1 << 22)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// GETSTATUS CCC interrupt enable (when the I3C is acting as target)
    pub mod STAIE {
        /// Offset (23 bits)
        pub const offset: u32 = 23;
        /// Mask (1 bit: 1 << 23)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// ENTDAA/RSTDAA/SETNEWDA CCC interrupt enable (when the I3C is acting as target)
    pub mod DAUPDIE {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (1 bit: 1 << 24)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// SETMWL CCC interrupt enable (when the I3C is acting as target)
    pub mod MWLUPDIE {
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

    /// SETMRL CCC interrupt enable (when the I3C is acting as target)
    pub mod MRLUPDIE {
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

    /// reset pattern interrupt enable (when the I3C is acting as target)
    pub mod RSTIE {
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

    /// ENTASx CCC interrupt enable (when the I3C is acting as target)
    pub mod ASUPDIE {
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

    /// ENEC/DISEC CCC interrupt enable (when the I3C is acting as target)
    pub mod INTUPDIE {
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

    /// DEFTGTS CCC interrupt enable (when the I3C is acting as target)
    pub mod DEFIE {
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

    /// DEFGRPA CCC interrupt enable (when the I3C is acting as target)
    pub mod GRPIE {
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

/// I3C clear event register
pub mod CEVR {

    /// clear frame complete flag (whatever the I3C is acting as controller/target)
    pub mod CFCF {
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

    /// clear target-initiated read end flag (when the I3C is acting as controller)
    pub mod CRXTGTENDF {
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

    /// clear error flag (whatever the I3C is acting as controller/target)
    pub mod CERRF {
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

    /// clear IBI request flag (when the I3C is acting as controller)
    pub mod CIBIF {
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

    /// clear IBI end flag (when the I3C is acting as target)
    pub mod CIBIENDF {
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

    /// clear controller-role request flag (when the I3C is acting as controller)
    pub mod CCRF {
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

    /// clear controller-role update flag (when the I3C is acting as target)
    pub mod CCRUPDF {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (1 bit: 1 << 18)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// clear hot-join flag (when the I3C is acting as controller)
    pub mod CHJF {
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

    /// clear wakeup flag (when the I3C is acting as target)
    pub mod CWKPF {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (1 bit: 1 << 21)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// clear GETxxx CCC flag (when the I3C is acting as target)
    pub mod CGETF {
        /// Offset (22 bits)
        pub const offset: u32 = 22;
        /// Mask (1 bit: 1 << 22)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// clear GETSTATUS CCC flag (when the I3C is acting as target)
    pub mod CSTAF {
        /// Offset (23 bits)
        pub const offset: u32 = 23;
        /// Mask (1 bit: 1 << 23)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// clear ENTDAA/RSTDAA/SETNEWDA CCC flag (when the I3C is acting as target)
    pub mod CDAUPDF {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (1 bit: 1 << 24)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// clear SETMWL CCC flag (when the I3C is acting as target)
    pub mod CMWLUPDF {
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

    /// clear SETMRL CCC flag (when the I3C is acting as target)
    pub mod CMRLUPDF {
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

    /// clear reset pattern flag (when the I3C is acting as target)
    pub mod CRSTF {
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

    /// clear ENTASx CCC flag (when the I3C is acting as target)
    pub mod CASUPDF {
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

    /// clear ENEC/DISEC CCC flag (when the I3C is acting as target)
    pub mod CINTUPDF {
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

    /// clear DEFTGTS CCC flag (when the I3C is acting as target)
    pub mod CDEFF {
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

    /// clear DEFGRPA CCC flag (when the I3C is acting as target)
    pub mod CGRPF {
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

/// I3C own device characteristics register
pub mod DEVR0 {

    /// dynamic address is valid (when the I3C is acting as target) When the I3C is acting as controller, this field can be written by software, for validating its own dynamic address, for example before a controller-role hand-off. When the I3C is acting as target, this field is asserted by hardware on the acknowledge of the broadcast ENTDAA CCC or the direct SETNEWDA CCC, and this field is cleared by hardware on the acknowledge of the broadcast RSTDAA CCC.
    pub mod DAVAL {
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

    /// 7-bit dynamic address When the I3C is acting as controller, this field can be written by software, for defining its own dynamic address. When the I3C is acting as target, this field is updated by hardware on the reception of either the broadcast ENTDAA CCC or the direct SETNEWDA CCC.
    pub mod DA {
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

    /// IBI request enable (when the I3C is acting as target) This field is initially written by software when I3C_CFGR.EN=0, and is updated by hardware on the reception of DISEC CCC with DISINT=1 (i.e. cleared) and the reception of ENEC CCC with ENINT=1 (i.e. set).
    pub mod IBIEN {
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

    /// controller-role request enable (when the I3C is acting as target) This field is initially written by software when I3C_CFGR.EN=0, and is updated by hardware on the reception of DISEC CCC with DISCR=1 (i.e. cleared) and the reception of ENEC CCC with ENCR=1 (i.e. set).
    pub mod CREN {
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

    /// hot-join request enable (when the I3C is acting as target) This field is initially written by software when I3C_CFGR.EN=0, and is updated by hardware on the reception of DISEC CCC with DISHJ=1 (i.e. cleared) and the reception of ENEC CCC with ENHJ=1 (i.e. set).
    pub mod HJEN {
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

    /// activity state (when the I3C is acting as target) This read field is updated by hardware on the reception of a ENTASx CCC (enter activity state, with x=0-3):
    pub mod AS {
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

    /// reset action/level on received reset pattern (when the I3C is acting as target) This read field is used by hardware on the reception of a direct read RSTACT CCC in order to return the corresponding data byte on the I3C bus. This read field is updated by hardware on the reception of a broadcast or direct write RSTACT CCC (target reset action). Only the defining bytes 0x00, 0x01 and 0x02 are mapped, and RSTACT\[1:0\] = Defining Byte\[1:0\]. a) partially reset the I3C peripheral, by a write and clear of the enable bit of the i3C configuration register (i.e. write I3C_CFGR.EN=0). This reset the I3C bus interface and the I3C kernel sub-parts, without modifying the content of the I3C APB registers (excepted the I3C_CFGR.EN bit). b) reset fully the I3C peripheral including all its registers via a write and set to the I3C reset control bit of the RCC (Reset and Clock Controller) register. a system reset. This has the same impact as a pin reset (i.e. NRST=0) (refer to RCC functional description - Reset part): – the software writes and set the AICR.SYSRESETREQ register control bit, when the device is controlled by a CortexTM-M. – the software writes and set the RCC_GRSTCSETR.SYSRST=1, when the device is controlled by a CortexTM-A.
    pub mod RSTACT {
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

    /// reset action is valid (when the I3C is acting as target) This read bit is asserted by hardware to indicate that the RTSACT\[1:0\] field has been updated on the reception of a broadcast or direct write RSTACT CCC (target reset action) and is valid. This field is cleared by hardware when the target receives a frame start. If RSTVAL=1: when the RSTF is asserted (and/or the corresponding interrupt if enabled), I3C_DEVR0.RSTACT\[1:0\] dictates the reset action to be performed by the software if any. If RSTVAL=0: when the RSTF is asserted (and/or the corresponding interrupt if enabled), the software should issue an I3C reset after a first detected reset pattern, and a system reset on the second one.
    pub mod RSTVAL {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (1 bit: 1 << 24)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// I3C device 1 characteristics register
pub mod DEVR1 {

    /// assigned I3C dynamic address to target x (when the I3C is acting as controller) When the I3C is acting as controller, this field should be written by software to store the 7-bit dynamic address that the controller sends via a broadcast ENTDAA or a direct SETNEWDA CCC which has been acknowledged by the target x. Writing to this field has no impact when the read field I3C_DEVRx.DIS=1.
    pub mod DA {
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

    /// IBI request acknowledge (when the I3C is acting as controller) When the I3C is acting as controller, this bit is written by software to define the acknowledge policy to be applied on the I3C bus on the reception of a IBI request from target x: - After the NACK, the message continues as initially programmed (the target is aware of the NACK and can emit another IBI request later on) - The field DIS is asserted by hardware to protect DA\[6:0\] from being modified by software meanwhile the hardware can store internally the current DA\[6:0\] into the kernel clock domain. - After the ACK, the controller logs the IBI payload data, if any, depending on I3C_DEVRx.IBIDEN. - The software is notified by the IBI flag (i.e. I3C_EVR.IBIF=1) and/or the corresponding interrupt if enabled; - Independently from IBIACK configuration for this or other devices, further IBI request(s) are NACKed until IBI request flag (i.e. I3C_EVR.IBIF) and controller-role request flag (i.e. I3C_EVR.CRF) are both cleared.
    pub mod IBIACK {
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

    /// controller-role request acknowledge (when the I3C is acting as controller) When the I3C is acting as controller, this bit is written by software to define the acknowledge policy to be applied on the I3C bus on the reception of a controller-role request from target x: After the NACK, the message continues as initially programmed (the target is aware of the NACK and can emit another controller-role request later on) - The field DIS is asserted by hardware to protect DA\[6:0\] from being modified by software meanwhile the hardware can store internally the current DA\[6:0\] into the kernel clock domain. - After the ACK, the message continues as initially programmed. The software is notified by the controller-role request flag (i.e. I3C_EVR.CRF=1) and/or the corresponding interrupt if enabled; For effectively granting the controller-role to the requesting secondary controller, software should issue a GETACCCR (formerly known as GETACCMST), followed by a STOP. - Independently of CRACK configuration for this or other devices, further controller-role request(s) are NACKed until controller-role request flag (i.e. I3C_EVR.CRF) and IBI flag (i.e. I3C_EVR.IBIF) are both cleared.
    pub mod CRACK {
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

    /// IBI data enable (when the I3C is acting as controller) When the I3C is acting as controller, this bit should be written by software to store the BCR\[2\] bit as received from the target x during broadcast ENTDAA or direct GETBCR CCC via the received I3C_RDR. Writing to this field has no impact when the read field I3C_DEVRx.DIS=1.
    pub mod IBIDEN {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (1 bit: 1 << 18)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// suspend/stop I3C transfer on received IBI (when the I3C is acting as controller) When the I3C is acting as controller, this bit is used to receive an IBI from target x with pending read notification feature (i.e. with received MDB\[7:5\]=3’b101). If this bit is set, when an IBI is received (i.e. I3C_EVR.IBIF=1), a Stop is emitted on the I3C bus and the C-FIFO is automatically flushed by hardware; to avoid a next private read communication issue if a previous private read message to the target x was stored in the C-FIFO.
    pub mod SUSP {
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

    /// DA\[6:0\] write disabled (when the I3C is acting as controller) When the I3C is acting as controller, once that software set IBIACK=1 or CRACK=1, this read bit is set by hardware (i.e. DIS=1) to lock the configured DA\[6:0\] and IBIDEN values. Then, to be able to next modify DA\[6:0\] or IBIDEN, the software must wait for this field DIS to be de-asserted by hardware (i.e. polling on DIS=0) before modifying these two assigned values to the target x. Indeed, the target may be requesting an IBI or a controller-role meanwhile the controller intends to modify DA\[6:0\] or IBIDEN.
    pub mod DIS {
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

/// I3C device 2 characteristics register
pub mod DEVR2 {
    pub use super::DEVR1::CRACK;
    pub use super::DEVR1::DA;
    pub use super::DEVR1::DIS;
    pub use super::DEVR1::IBIACK;
    pub use super::DEVR1::IBIDEN;
    pub use super::DEVR1::SUSP;
}

/// I3C device 3 characteristics register
pub mod DEVR3 {
    pub use super::DEVR1::CRACK;
    pub use super::DEVR1::DA;
    pub use super::DEVR1::DIS;
    pub use super::DEVR1::IBIACK;
    pub use super::DEVR1::IBIDEN;
    pub use super::DEVR1::SUSP;
}

/// I3C device 4 characteristics register
pub mod DEVR4 {
    pub use super::DEVR1::CRACK;
    pub use super::DEVR1::DA;
    pub use super::DEVR1::DIS;
    pub use super::DEVR1::IBIACK;
    pub use super::DEVR1::IBIDEN;
    pub use super::DEVR1::SUSP;
}

/// I3C maximum read length register
pub mod MAXRLR {

    /// maximum data read length (when I3C is acting as target) This field is initially written by software when I3C_CFGR.EN=0 and updated by hardware on the reception of SETMRL command (with potentially also updated IBIP\[2:0\]). Software is notified of a MRL update by the I3C_EVR.MRLUPF and the corresponding interrupt if enabled. This field is used by hardware to return the value on the I3C bus when the target receives a GETMRL CCC.
    pub mod MRL {
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

    /// IBI payload data size, in bytes (when I3C is acting as target) This field is initially written by software when I3C_CFGR.EN=0 to set the number of data bytes to be sent to the controller after an IBI request has been acknowledged.This field may be updated by hardware on the reception of SETMRL command (which potentially also updated IBIP\[2:0\]). Software is notified of a MRL update by the I3C_EVR.MRLUPF and the corresponding interrupt if enabled. others: same as 100
    pub mod IBIP {
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
}

/// I3C maximum write length register
pub mod MAXWLR {

    /// maximum data write length (when I3C is acting as target) This field is initially written by software when I3C_CFGR.EN=0 and updated by hardware on the reception of SETMWL command. Software is notified of a MWL update by the I3C_EVR.MWLUPF and the corresponding interrupt if enabled. This field is used by hardware to return the value on the I3C bus when the target receives a GETMWL CCC.
    pub mod MWL {
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

/// I3C timing register 0
pub mod TIMINGR0 {

    /// SCL low duration in I3C push-pull phases, in number of kernel clocks cycles: tSCLL_PP = (SCLL_PP + 1) x tI3CCLK SCLL_PP is used to generate tLOW (I3C) timing.
    pub mod SCLL_PP {
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

    /// SCL high duration, used for I3C messages (both in push-pull and open-drain phases), in number of kernel clocks cycles: tSCLH_I3C = (SCLH_I3C + 1) x tI3CCLK SCLH_I3C is used to generate both tHIGH (I3C) and tHIGH_MIXED timings.
    pub mod SCLH_I3C {
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

    /// SCL low duration in open-drain phases, used for legacy I2C commands and for I3C open-drain phases (address header phase following a START, not a Repeated START), in number of kernel clocks cycles: tSCLL_OD = (SCLL_OD + 1) x tI3CCLK SCLL_OD is used to generate both tLOW (I2C) and tLOW_OD timings (max. of the two).
    pub mod SCLL_OD {
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

    /// SCL high duration, used for legacy I2C commands, in number of kernel clocks cycles: tSCLH_I2C = (SCLH_I2C + 1) x tI3CCLK SCLH_I2C is used to generate tHIGH (I2C) timing.
    pub mod SCLH_I2C {
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
}

/// I3C timing register 1
pub mod TIMINGR1 {

    /// number of kernel clock cycles, that is used whatever I3C is acting as controller or target, to set the following MIPI I3C timings, like bus available condition time: When the I3C is acting as target: for bus available condition time: it must wait for (bus available condition) time to be elapsed after a stop and before issuing a start request for an IBI or a controller-role request (i.e. bus free condition is sustained for at least tAVAL). refer to MIPI timing tAVAL = 1 �s. This timing is defined by: tAVAL = (AVAL\[7:0\] + 2) x tI3CCLK for bus idle condition time: it must wait for (bus idle condition) time to be elapsed after that both SDA and SCL are continuously high and stable before issuing a hot-join event. Refer to MIPI v1.1 timing tIDLE = 200 �s . This timing is defined by: tIDLE = (AVAL\[7:0\] + 2) x 200 x tI3CCLK When the I3C is acting as controller, it can not stall the clock beyond a maximum stall time (i.e. stall the SCL clock low), as follows: on first bit of assigned address during dynamic address assignment: it can not stall the clock beyond the MIPI timing tSTALLDAA = 15 ms. This timing is defined by: tSTALLDAA = (AVAL\[7:0\] + 1) x 15000 x tI3CCLK on ACK/NACK phase of I3C/I2C transfer, on parity bit of write data transfer, on transition bit of I3C read transfer: it can not stall the clock beyond the MIPI timing tSTALL = 100 �s. This timing is defined by: tSTALL = (AVAL\[7:0\] + 1) x 100 x tI3CCLK Whatever the I3C is acting as controller or as (controller-capable) target, during a controller-role hand-off procedure: The new controller must wait for a time (refer to MIPI timing tNEWCRLock) before pulling SDA low (i.e. issuing a start). And the active controller must wait for the same time while monitoring new controller and before testing the new controller by pulling SDA low. This time to wait is dependent on the defined I3C_TIMINGR1.ANSCR\[1:0\], as follows: If ASNCR\[1:0\]=00: tNEWCRLock = (AVAL\[7:0\] + 1) x tI3CCLK If ASNCR\[1:0\]=01: tNEWCRLock = (AVAL\[7:0\] + 1) x 100 x tI3CCLK If ASNCR\[1:0\]=10: tNEWCRLock = (AVAL\[7:0\] + 1) x 2000 x tI3CCLK If ASNCR\[1:0\]=11: tNEWCRLock = (AVAL\[7:0\] + 1) x 50000 x tI3CCLK
    pub mod AVAL {
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

    /// activity state of the new controller (when I3C is acting as - active- controller) This field indicates the time to wait before being accessed as new target, refer to the other field AVAL\[7:0\]. This field can be modified only when the I3C is acting as controller.
    pub mod ASNCR {
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

    /// number of kernel clocks cycles that is used to set some MIPI timings like bus free condition time (when the I3C is acting as controller) When the I3C is acting as controller: for I3C start timing: it must wait for (bus free condition) time to be elapsed after a stop and before a start, refer to MIPI timings (I3C) tCAS and (I2C) tBUF. These timings are defined by: tBUF= tCAS = \[ (FREE\[6:0\] + 1) x 2 - (0,5 + SDA_HD)\] x tI3CCLK Note: for pure I3C bus: tCASmin= 38,4 ns. Note: for pure I3C bus: tCASmax=1�s, 100�s, 2ms, 50ms for respectively ENTAS0,1,2, and 3. Note: for mixed bus with I2C fm+ device: tBUFmin = 0,5 �s. Note: for mixed bus with I2C fm device: tBUFmin = 1,3 �s. for I3C repeated start timing: it must wait for time to be elapsed after a repeated start (i.e. SDA is de-asserted) and before driving SCL low, refer to. MIPI timing tCASr. This timing is defined by: tCASr = \[ (FREE\[6:0\] + 1) x 2 - (0,5 + SDA_HD)\] x tI3CCLK for I3C stop timing: it must wait for time to be elapsed after that the SCL clock is driven high and before the stop condition (i.e. SDA is asserted). This timing is defined by: tCBP = (FREE\[6:0\] + 1) x tI3CCLK for I3C repeated start timing (T-bit when controller ends read with repeated start followed by stop): it must wait for time to be elapsed after that the SCL clock is driven high and before the repeated start condition (i.e. SDA is de-asserted). This timing is defined by: tCBSr = (FREE\[6:0\] + 1) x tI3CCLK
    pub mod FREE {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (7 bits: 0x7f << 16)
        pub const mask: u32 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// SDA hold time (when the I3C is acting as controller), in number of kernel clocks cycles (refer to MIPI timing SDA hold time in push-pull tHD_PP):
    pub mod SDA_HD {
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
}

/// I3C timing register 2
pub mod TIMINGR2 {

    /// Controller clock stall on T-bit phase of Data enable The SCL is stalled during STALL x tSCLL_PP in the T-bit phase (before 9th bit). This allows the target to prepare data to be sent.
    pub mod STALLT {
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

    /// controller clock stall on PAR phase of Data enable The SCL is stalled during STALL x tSCLL_PP in the T-bit phase (before 9th bit). This allows the target to read received data.
    pub mod STALLD {
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

    /// controller clock stall on PAR phase of CCC enable The SCL is stalled during STALL x tSCLL_PP in the T-bit phase of common command code (before 9th bit). This allows the target to decode the command.
    pub mod STALLC {
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

    /// controller clock stall enable on ACK phase The SCL is stalled (during tSCLL_STALLas defined by STALL) in the address ACK/NACK phase (before 9th bit). This allows the target to prepare data or the controller to respond to target interrupt.
    pub mod STALLA {
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

    /// controller clock stall time, in number of kernel clock cycles tSCLL_STALL = STALL x tI3CCLK
    pub mod STALL {
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

/// I3C bus characteristics register
pub mod BCR {

    /// max data speed limitation
    pub mod BCR0 {
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

    /// in-band interrupt (IBI) payload
    pub mod BCR2 {
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

    /// controller capable
    pub mod BCR6 {
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
}

/// I3C device characteristics register
pub mod DCR {

    /// device characteristics ID others: ID to describe the type of the I3C sensor/device Note: The latest MIPI DCR ID assignments are available at: https://www.mipi.org/MIPI_I3C_device_characteristics_register
    pub mod DCR {
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

/// I3C get capability register
pub mod GETCAPR {

    /// IBI MDB support for pending read notification This bit is written by software during bus initialization (i.e. I3C_CFGR.EN=0) and indicates the support (or not) of the pending read notification via the IBI MDB\[7:0\] value. This bit is used to return the GETCAP3 byte in response to the GETCAPS CCC format 1.
    pub mod CAPPEND {
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

/// I3C controller-role capability register
pub mod CRCAPR {

    /// delayed controller-role hand-off This bit is written by software during bus initialization (i.e. I3C_CFGR.EN=0) and indicates if this target I3C may need additional time to process a controller-role hand-off requested by the current controller. This bit is used to return the CRCAP2 byte in response to the GETCAPS CCC format 2.
    pub mod CAPDHOFF {
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

    /// group management support (when acting as controller) This bit is written by software during bus initialization (i.e. I3C_CFGR.EN=0) and indicates if the I3C is able to support group management when it acts as a controller (after controller-role hand-off) via emitted DEFGRPA, RSTGRPA, and SETGRPA CCC. This bit is used to return the CRCAP1 byte in response to the GETCAPS CCC format 2.
    pub mod CAPGRP {
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

/// I3C get capability register
pub mod GETMXDSR {

    /// controller hand-off activity state This bit is written by software during bus initialization (i.e. I3C_CFGR.EN=0) and indicates in which initial activity state the (other) current controller should expect the I3C bus after a controller-role hand-off to this controller-capable I3C, when returning the defining byte CRHDLY (0x91) to a GETMXDS CCC. This 2-bit field is used to return the CRHDLY1 byte in response to the GETCAPS CCC format 3, in order to state which is the activity state of this I3C when becoming controller after a controller-role hand-off, and consequently the time the former controller should wait before testing this I3C to be confirmed its ownership.
    pub mod HOFFAS {
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

    /// GETMXDS CCC format
    pub mod FMT {
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

    /// programmed byte of the 3-byte MaxRdTurn (maximum read turnaround byte) This bit is written by software during bus initialization (i.e. I3C_CFGR.EN=0) and writes the value of the selected byte (via the FMT\[1:0\] field) of the 3-byte MaxRdTurn which is returned in response to the GETMXDS CCC format 2 to encode the maximum read turnaround time.
    pub mod RDTURN {
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

    /// clock-to-data turnaround time (tSCO) This bit is written by software during bus initialization (i.e. I3C_CFGR.EN=0) and is used to specify the clock-to-data turnaround time tSCO (vs the value of 12 ns). This bit is used by the hardware in response to the GETMXDS CCC to return the encoded clock-to-data turnaround time via the returned MaxRd\[5:3\] bits.
    pub mod TSCO {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (1 bit: 1 << 24)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// I3C extended provisioned ID register
pub mod EPIDR {

    /// 4-bit MIPI Instance ID This field is written by software to set and identify individually each instance of this I3C IP with a specific number on a single I3C bus. This field represents the bits\[15:12\] of the 48-bit provisioned ID. Note: The bits\[11:0\] of the provisioned ID may be 0.
    pub mod MIPIID {
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

    /// provisioned ID type selector This field is set as 0 i.e. vendor fixed value. This field represents the bit\[32\] of the 48-bit provisioned ID. Note: The bits\[31:16\] of the provisioned ID may be 0.
    pub mod IDTSEL {
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

    /// 15-bit MIPI manufacturer ID This read field is the 15-bit STMicroelectronics MIPI ID i.e. 0x0104. This field represents the bits\[47:33\] of the 48-bit provisioned ID.
    pub mod MIPIMID {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (15 bits: 0x7fff << 17)
        pub const mask: u32 = 0x7fff << offset;
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
    /// CR and CR_ALTERNATE
    /// CR: I3C message control register
    /// CR_ALTERNATE: I3C message control register alternate
    pub CR: RWRegister<u32>,

    /// I3C configuration register
    pub CFGR: RWRegister<u32>,

    _reserved1: [u8; 8],

    /// I3C receive data byte register
    pub RDR: RORegister<u32>,

    /// I3C receive data word register
    pub RDWR: RORegister<u32>,

    /// I3C transmit data byte register
    pub TDR: WORegister<u32>,

    /// I3C transmit data word register
    pub TDWR: WORegister<u32>,

    /// I3C IBI payload data register
    pub IBIDR: RWRegister<u32>,

    /// I3C target transmit configuration register
    pub TGTTDR: RWRegister<u32>,

    _reserved2: [u8; 8],

    /// I3C status register
    pub SR: RORegister<u32>,

    /// I3C status error register
    pub SER: RORegister<u32>,

    _reserved3: [u8; 8],

    /// I3C received message register
    pub RMR: RORegister<u32>,

    _reserved4: [u8; 12],

    /// I3C event register
    pub EVR: RORegister<u32>,

    /// I3C interrupt enable register
    pub IER: RORegister<u32>,

    /// I3C clear event register
    pub CEVR: WORegister<u32>,

    _reserved5: [u8; 4],

    /// I3C own device characteristics register
    pub DEVR0: RWRegister<u32>,

    /// I3C device 1 characteristics register
    pub DEVR1: RWRegister<u32>,

    /// I3C device 2 characteristics register
    pub DEVR2: RWRegister<u32>,

    /// I3C device 3 characteristics register
    pub DEVR3: RWRegister<u32>,

    /// I3C device 4 characteristics register
    pub DEVR4: RWRegister<u32>,

    _reserved6: [u8; 28],

    /// I3C maximum read length register
    pub MAXRLR: RWRegister<u32>,

    /// I3C maximum write length register
    pub MAXWLR: RWRegister<u32>,

    _reserved7: [u8; 8],

    /// I3C timing register 0
    pub TIMINGR0: RWRegister<u32>,

    /// I3C timing register 1
    pub TIMINGR1: RWRegister<u32>,

    /// I3C timing register 2
    pub TIMINGR2: RWRegister<u32>,

    _reserved8: [u8; 20],

    /// I3C bus characteristics register
    pub BCR: RWRegister<u32>,

    /// I3C device characteristics register
    pub DCR: RWRegister<u32>,

    /// I3C get capability register
    pub GETCAPR: RWRegister<u32>,

    /// I3C controller-role capability register
    pub CRCAPR: RWRegister<u32>,

    /// I3C get capability register
    pub GETMXDSR: RWRegister<u32>,

    /// I3C extended provisioned ID register
    pub EPIDR: RWRegister<u32>,
}
pub struct ResetValues {
    pub CR: u32,
    pub CFGR: u32,
    pub RDR: u32,
    pub RDWR: u32,
    pub TDR: u32,
    pub TDWR: u32,
    pub IBIDR: u32,
    pub TGTTDR: u32,
    pub SR: u32,
    pub SER: u32,
    pub RMR: u32,
    pub EVR: u32,
    pub IER: u32,
    pub CEVR: u32,
    pub DEVR0: u32,
    pub DEVR1: u32,
    pub DEVR2: u32,
    pub DEVR3: u32,
    pub DEVR4: u32,
    pub MAXRLR: u32,
    pub MAXWLR: u32,
    pub TIMINGR0: u32,
    pub TIMINGR1: u32,
    pub TIMINGR2: u32,
    pub BCR: u32,
    pub DCR: u32,
    pub GETCAPR: u32,
    pub CRCAPR: u32,
    pub GETMXDSR: u32,
    pub EPIDR: u32,
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
