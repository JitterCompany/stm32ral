#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! TIM15 address block description
//!
//! Used by: stm32h562, stm32h563, stm32h573

use crate::RWRegister;
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// TIM15 control register 1
pub mod TIM15_CR1 {

    /// Counter enable Note: External clock and gated mode can work only if the CEN bit has been previously set by software. However trigger mode can set the CEN bit automatically by hardware.
    pub mod CEN {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Update disable This bit is set and cleared by software to enable/disable UEV event generation. Counter overflow/underflow Setting the UG bit Update generation through the slave mode controller Buffered registers are then loaded with their preload values.
    pub mod UDIS {
        /// Offset (1 bits)
        pub const offset: u16 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Update request source This bit is set and cleared by software to select the UEV event sources. Counter overflow/underflow Setting the UG bit Update generation through the slave mode controller
    pub mod URS {
        /// Offset (2 bits)
        pub const offset: u16 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// One-pulse mode
    pub mod OPM {
        /// Offset (3 bits)
        pub const offset: u16 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Auto-reload preload enable
    pub mod ARPE {
        /// Offset (7 bits)
        pub const offset: u16 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Clock division This bitfield indicates the division ratio between the timer clock (tim_ker_ck) frequency and the dead-time and sampling clock (tDTS) used by the dead-time generators and the digital filters (tim_tix)
    pub mod CKD {
        /// Offset (8 bits)
        pub const offset: u16 = 8;
        /// Mask (2 bits: 0b11 << 8)
        pub const mask: u16 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// UIF status bit remapping
    pub mod UIFREMAP {
        /// Offset (11 bits)
        pub const offset: u16 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Dithering enable Note: The DITHEN bit can only be modified when CEN bit is reset.
    pub mod DITHEN {
        /// Offset (12 bits)
        pub const offset: u16 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// TIM15 control register 2
pub mod TIM15_CR2 {

    /// Capture/compare preloaded control Note: This bit acts only on channels that have a complementary output.
    pub mod CCPC {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Capture/compare control update selection Note: This bit acts only on channels that have a complementary output.
    pub mod CCUS {
        /// Offset (2 bits)
        pub const offset: u16 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Capture/compare DMA selection
    pub mod CCDS {
        /// Offset (3 bits)
        pub const offset: u16 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Master mode selection These bits allow to select the information to be sent in master mode to slave timers for synchronization (tim_trgo). The combination is as follows:
    pub mod MMS {
        /// Offset (4 bits)
        pub const offset: u16 = 4;
        /// Mask (3 bits: 0b111 << 4)
        pub const mask: u16 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// tim_ti1 selection
    pub mod TI1S {
        /// Offset (7 bits)
        pub const offset: u16 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Output Idle state 1 (tim_oc1 output) Note: This bit can not be modified as long as LOCK level 1, 2 or 3 has been programmed (LOCK bits in TIM15_BKR register).
    pub mod OIS1 {
        /// Offset (8 bits)
        pub const offset: u16 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Output Idle state 1 (tim_oc1n output) Note: This bit can not be modified as long as LOCK level 1, 2 or 3 has been programmed (LOCK bits in TIM15_BKR register).
    pub mod OIS1N {
        /// Offset (9 bits)
        pub const offset: u16 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Output idle state 2 (tim_oc2 output) Note: This bit cannot be modified as long as LOCK level 1, 2 or 3 has been programmed (LOCK bits in the TIM15_BKR register).
    pub mod OIS2 {
        /// Offset (10 bits)
        pub const offset: u16 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// TIM15 slave mode control register
pub mod TIM15_SMCR {

    /// Slave mode selection When external signals are selected the active edge of the trigger signal (tim_trgi) is linked to the polarity selected on the external input (see Input Control register and Control Register description. Others: Reserved. Note: The gated mode must not be used if tim_ti1f_ed is selected as the trigger input (TS=’00100’). Indeed, tim_ti1f_ed outputs 1 pulse for each transition on tim_ti1f, whereas the gated mode checks the level of the trigger signal. Note: The clock of the slave peripherals (timer, ADC, ...) receiving the tim_trgo signal must be enabled prior to receive events from the master timer, and the clock frequency (prescaler) must not be changed on-the-fly while triggers are received from the master timer.
    pub mod SMS1 {
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

    /// Trigger selection This bit field selects the trigger input to be used to synchronize the counter. Others: Reserved See for more details on tim_itrx meaning for each timer. Note: These bits must be changed only when they are not used (e.g. when SMS=000) to avoid wrong edge detections at the transition.
    pub mod TS1 {
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

    /// Master/slave mode
    pub mod MSM {
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

    /// Slave mode selection When external signals are selected the active edge of the trigger signal (tim_trgi) is linked to the polarity selected on the external input (see Input Control register and Control Register description. Others: Reserved. Note: The gated mode must not be used if tim_ti1f_ed is selected as the trigger input (TS=’00100’). Indeed, tim_ti1f_ed outputs 1 pulse for each transition on tim_ti1f, whereas the gated mode checks the level of the trigger signal. Note: The clock of the slave peripherals (timer, ADC, ...) receiving the tim_trgo signal must be enabled prior to receive events from the master timer, and the clock frequency (prescaler) must not be changed on-the-fly while triggers are received from the master timer.
    pub mod SMS2 {
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

    /// Trigger selection This bit field selects the trigger input to be used to synchronize the counter. Others: Reserved See for more details on tim_itrx meaning for each timer. Note: These bits must be changed only when they are not used (e.g. when SMS=000) to avoid wrong edge detections at the transition.
    pub mod TS2 {
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
}

/// TIM15 DMA/interrupt enable register
pub mod TIM15_DIER {

    /// Update interrupt enable
    pub mod UIE {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Capture/Compare 1 interrupt enable
    pub mod CC1IE {
        /// Offset (1 bits)
        pub const offset: u16 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Capture/Compare 2 interrupt enable
    pub mod CC2IE {
        /// Offset (2 bits)
        pub const offset: u16 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// COM interrupt enable
    pub mod COMIE {
        /// Offset (5 bits)
        pub const offset: u16 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Trigger interrupt enable
    pub mod TIE {
        /// Offset (6 bits)
        pub const offset: u16 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Break interrupt enable
    pub mod BIE {
        /// Offset (7 bits)
        pub const offset: u16 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Update DMA request enable
    pub mod UDE {
        /// Offset (8 bits)
        pub const offset: u16 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Capture/Compare 1 DMA request enable
    pub mod CC1DE {
        /// Offset (9 bits)
        pub const offset: u16 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// COM DMA request enable
    pub mod COMDE {
        /// Offset (13 bits)
        pub const offset: u16 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Trigger DMA request enable
    pub mod TDE {
        /// Offset (14 bits)
        pub const offset: u16 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// TIM15 status register
pub mod TIM15_SR {

    /// Update interrupt flag This bit is set by hardware on an update event. It is cleared by software. At overflow regarding the repetition counter value (update if repetition counter = 0) and if the UDIS=0 in the TIM15_CR1 register. When CNT is reinitialized by software using the UG bit in TIM15_EGR register, if URS=0 and UDIS=0 in the TIM15_CR1 register. When CNT is reinitialized by a trigger event (refer to control register (TIM15_SMCR)), if URS=0 and UDIS=0 in the TIM15_CR1 register.
    pub mod UIF {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Capture/Compare 1 interrupt flag This flag is set by hardware. It is cleared by software (input capture or output compare mode) or by reading the TIMx_CCR1 register (input capture mode only). If channel CC1 is configured as output: this flag is set when the content of the counter TIMx_CNT matches the content of the TIMx_CCR1 register. When the content of TIMx_CCR1 is greater than the content of TIMx_ARR, the CC1IF bit goes high on the counter overflow (in up-counting and up/down-counting modes) or underflow (in downcounting mode). There are 3 possible options for flag setting in center-aligned mode, refer to the CMS bits in the TIMx_CR1 register for the full description. If channel CC1 is configured as input: this bit is set when counter value has been captured in TIMx_CCR1 register (an edge has been detected on IC1, as per the edge sensitivity defined with the CC1P and CC1NP bits setting, in TIMx_CCER).
    pub mod CC1IF {
        /// Offset (1 bits)
        pub const offset: u16 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Capture/Compare 2 interrupt flag refer to CC1IF description
    pub mod CC2IF {
        /// Offset (2 bits)
        pub const offset: u16 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// COM interrupt flag This flag is set by hardware on a COM event (once the capture/compare control bits –CCxE, CCxNE, OCxM– have been updated). It is cleared by software.
    pub mod COMIF {
        /// Offset (5 bits)
        pub const offset: u16 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Trigger interrupt flag This flag is set by hardware on the TRG trigger event (active edge detected on tim_trgi input when the slave mode controller is enabled in all modes but gated mode, both edges in case gated mode is selected). It is set when the counter starts or stops when gated mode is selected. It is cleared by software.
    pub mod TIF {
        /// Offset (6 bits)
        pub const offset: u16 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Break interrupt flag This flag is set by hardware as soon as the break input goes active. It can be cleared by software if the break input is not active.
    pub mod BIF {
        /// Offset (7 bits)
        pub const offset: u16 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Capture/Compare 1 overcapture flag This flag is set by hardware only when the corresponding channel is configured in input capture mode. It is cleared by software by writing it to ‘0’.
    pub mod CC1OF {
        /// Offset (9 bits)
        pub const offset: u16 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Capture/Compare 2 overcapture flag Refer to CC1OF description
    pub mod CC2OF {
        /// Offset (10 bits)
        pub const offset: u16 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// TIM15 event generation register
pub mod TIM15_EGR {

    /// Update generation This bit can be set by software, it is automatically cleared by hardware.
    pub mod UG {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Capture/Compare 1 generation This bit is set by software in order to generate an event, it is automatically cleared by hardware. 1 A capture/compare event is generated on channel 1: If channel CC1 is configured as output: CC1IF flag is set, Corresponding interrupt or DMA request is sent if enabled. If channel CC1 is configured as input: The current value of the counter is captured in TIM15_CCR1 register. The CC1IF flag is set, the corresponding interrupt or DMA request is sent if enabled. The CC1OF flag is set if the CC1IF flag was already high.
    pub mod CC1G {
        /// Offset (1 bits)
        pub const offset: u16 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Capture/Compare 2 generation Refer to CC1G description
    pub mod CC2G {
        /// Offset (2 bits)
        pub const offset: u16 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Capture/Compare control update generation This bit can be set by software, it is automatically cleared by hardware. Note: This bit acts only on channels that have a complementary output.
    pub mod COMG {
        /// Offset (5 bits)
        pub const offset: u16 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Trigger generation This bit is set by software in order to generate an event, it is automatically cleared by hardware.
    pub mod TG {
        /// Offset (6 bits)
        pub const offset: u16 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Break generation This bit is set by software in order to generate an event, it is automatically cleared by hardware.
    pub mod BG {
        /// Offset (7 bits)
        pub const offset: u16 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// TIM15_CCMR1_Input and TIM15_CCMR1_Output
/// TIM15_CCMR1_Input: TIM15 capture/compare mode register 1 \\[alternate\\]
/// TIM15_CCMR1_Output: TIM15 capture/compare mode register 1 \\[alternate\\]
pub mod TIM15_CCMR1 {

    /// Capture/Compare 1 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC1S bits are writable only when the channel is OFF (CC1E = ‘0’ in TIM15_CCER).
    pub mod CC1S {
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

    /// Input capture 1 prescaler This bit-field defines the ratio of the prescaler acting on CC1 input (tim_ic1). The prescaler is reset as soon as CC1E=’0’ (TIM15_CCER register).
    pub mod IC1PSC {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (2 bits: 0b11 << 2)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Input capture 1 filter This bit-field defines the frequency used to sample tim_ti1 input and the length of the digital filter applied to tim_ti1. The digital filter is made of an event counter in which N consecutive events are needed to validate a transition on the output:
    pub mod IC1F {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Capture/Compare 2 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC2S bits are writable only when the channel is OFF (CC2E = ‘0’ in TIM15_CCER).
    pub mod CC2S {
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

    /// Input capture 2 prescaler
    pub mod IC2PSC {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (2 bits: 0b11 << 10)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Input capture 2 filter
    pub mod IC2F {
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

    /// Output Compare 1 fast enable This bit decreases the latency between a trigger event and a transition on the timer output. It must be used in one-pulse mode (OPM bit set in TIMx_CR1 register), to have the output pulse starting as soon as possible after the starting trigger.
    pub mod OC1FE {
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

    /// Output Compare 1 preload enable Note: These bits can not be modified as long as LOCK level 3 has been programmed (LOCK bits in TIM15_BDTR register) and CC1S=’00’ (the channel is configured in output). The PWM mode can be used without validating the preload register only in one pulse mode (OPM bit set in TIM15_CR1 register). Else the behavior is not guaranteed.
    pub mod OC1PE {
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

    /// Output compare 1 mode These bits define the behavior of the output reference signal tim_oc1ref from which tim_oc1 and tim_oc1n are derived. tim_oc1ref is active high whereas tim_oc1 and tim_oc1n active level depends on CC1P and CC1NP bits. Note: These bits can not be modified as long as LOCK level 3 has been programmed (LOCK bits in TIM15_BDTR register) and CC1S=’00’ (the channel is configured in output). In PWM mode, the tim_ocxref level changes only when the result of the comparison changes or when the output compare mode switches from “frozen” mode to “PWM” mode. On channels that have a complementary output, this bit field is preloaded. If the CCPC bit is set in the TIM15_CR2 register then the OC1M active bits take the new value from the preloaded bits only when a COM event is generated.
    pub mod OC1M1 {
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

    /// Output compare 1 clear enable
    pub mod OC1CE {
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

    /// Output compare 2 fast enable
    pub mod OC2FE {
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

    /// Output compare 2 preload enable
    pub mod OC2PE {
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

    /// Output compare 2 mode
    pub mod OC2M1 {
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

    /// Output compare 2 clear enable
    pub mod OC2CE {
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

    /// Output compare 1 mode These bits define the behavior of the output reference signal tim_oc1ref from which tim_oc1 and tim_oc1n are derived. tim_oc1ref is active high whereas tim_oc1 and tim_oc1n active level depends on CC1P and CC1NP bits. Note: These bits can not be modified as long as LOCK level 3 has been programmed (LOCK bits in TIM15_BDTR register) and CC1S=’00’ (the channel is configured in output). In PWM mode, the tim_ocxref level changes only when the result of the comparison changes or when the output compare mode switches from “frozen” mode to “PWM” mode. On channels that have a complementary output, this bit field is preloaded. If the CCPC bit is set in the TIM15_CR2 register then the OC1M active bits take the new value from the preloaded bits only when a COM event is generated.
    pub mod OC1M2 {
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

    /// Output compare 2 mode
    pub mod OC2M2 {
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

/// TIM15 capture/compare enable register
pub mod TIM15_CCER {

    /// Capture/Compare 1 output enable When CC1 channel is configured as output, the OC1 level depends on MOE, OSSI, OSSR, OIS1, OIS1N and CC1NE bits, regardless of the CC1E bits state. Refer to for details.
    pub mod CC1E {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Capture/Compare 1 output polarity CC1 channel configured as output: When CC1 channel is configured as input, both CC1NP/CC1P bits select the active polarity of TI1FP1 and TI2FP1 for trigger or capture operations. CC1NP=0, CC1P=0: non-inverted/rising edge. The circuit is sensitive to TIxFP1 rising edge (capture or trigger operations in reset, external clock or trigger mode), TIxFP1 is not inverted (trigger operation in gated mode or encoder mode). CC1NP=0, CC1P=1: inverted/falling edge. The circuit is sensitive to TIxFP1 falling edge (capture or trigger operations in reset, external clock or trigger mode), TIxFP1 is inverted (trigger operation in gated mode or encoder mode). CC1NP=1, CC1P=1: non-inverted/both edges/ The circuit is sensitive to both TIxFP1 rising and falling edges (capture or trigger operations in reset, external clock or trigger mode), TIxFP1is not inverted (trigger operation in gated mode). This configuration must not be used in encoder mode. CC1NP=1, CC1P=0: this configuration is reserved, it must not be used. Note: This bit is not writable as soon as LOCK level 2 or 3 has been programmed (LOCK bits in TIM15_BDTR register). On channels that have a complementary output, this bit is preloaded. If the CCPC bit is set in the TIM15_CR2 register then the CC1P active bit takes the new value from the preloaded bit only when a Commutation event is generated.
    pub mod CC1P {
        /// Offset (1 bits)
        pub const offset: u16 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Capture/Compare 1 complementary output enable
    pub mod CC1NE {
        /// Offset (2 bits)
        pub const offset: u16 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Capture/Compare 1 complementary output polarity CC1 channel configured as output: CC1 channel configured as input: This bit is used in conjunction with CC1P to define the polarity of tim_ti1fp1 and tim_ti2fp1. Refer to CC1P description. Note: This bit is not writable as soon as LOCK level 2 or 3 has been programmed (LOCK bits in TIM15_BDTR register) and CC1S=”00” (the channel is configured in output). On channels that have a complementary output, this bit is preloaded. If the CCPC bit is set in the TIM15_CR2 register then the CC1NP active bit takes the new value from the preloaded bit only when a Commutation event is generated.
    pub mod CC1NP {
        /// Offset (3 bits)
        pub const offset: u16 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Capture/Compare 2 output enable Refer to CC1E description
    pub mod CC2E {
        /// Offset (4 bits)
        pub const offset: u16 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Capture/Compare 2 output polarity Refer to CC1P description
    pub mod CC2P {
        /// Offset (5 bits)
        pub const offset: u16 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Capture/Compare 2 complementary output polarity Refer to CC1NP description
    pub mod CC2NP {
        /// Offset (7 bits)
        pub const offset: u16 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// TIM15 counter
pub mod TIM15_CNT {

    /// Counter value Non-dithering mode (DITHEN = 0) The register holds the counter value. Dithering mode (DITHEN = 1) The register only holds the non-dithered part in CNT\[15:0\]. The fractional part is not available.
    pub mod CNT {
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

    /// UIF Copy This bit is a read-only copy of the UIF bit in the TIM15_ISR register.
    pub mod UIFCPY {
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

/// TIM15 prescaler
pub mod TIM15_PSC {

    /// Prescaler value The counter clock frequency (ftim_cnt_ck) is equal to ftim_psc_ck / (PSC\[15:0\] + 1). PSC contains the value to be loaded in the active prescaler register at each update event (including when the counter is cleared through UG bit of TIM15_EGR register or through trigger controller when configured in “reset mode”).
    pub mod PSC {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (16 bits: 0xffff << 0)
        pub const mask: u16 = 0xffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// TIM15 auto-reload register
pub mod TIM15_ARR {

    /// Auto-reload value ARR is the value to be loaded in the actual auto-reload register. Refer to the for more details about ARR update and behavior. The counter is blocked while the auto-reload value is null. Non-dithering mode (DITHEN = 0) The register holds the auto-reload value in ARR\[15:0\]. The ARR\[19:16\] bits are reset. Dithering mode (DITHEN = 1) The register holds the integer part in ARR\[19:4\]. The ARR\[3:0\] bitfield contains the dithered part.
    pub mod ARR {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (20 bits: 0xfffff << 0)
        pub const mask: u32 = 0xfffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// TIM15 repetition counter register
pub mod TIM15_RCR {

    /// Repetition counter reload value This bitfield defines the update rate of the compare registers (i.e. periodic transfers from preload to active registers) when preload registers are enable. It also defines the update interrupt generation rate, if this interrupt is enable. When the repetition down-counter reaches zero, an update event is generated and it restarts counting from REP value. As the reptition counter is reloaded with REP value only at the repetition update event UEV, any write to the TIM15_RCR register is not taken in account until the next repetition update event. It means in PWM mode (REP+1) corresponds to the number of PWM periods in edge-aligned mode: the number of PWM periods in edge-aligned mode the number of half PWM period in center-aligned mode
    pub mod REP {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (8 bits: 0xff << 0)
        pub const mask: u16 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// TIM15 capture/compare register 1
pub mod TIM15_CCR1 {

    /// Capture/compare 1 value If channel CC1 is configured as output: CCR1 is the value to be loaded in the actual capture/compare 1 register (preload value). It is loaded permanently if the preload feature is not selected in the TIM15_CCMR1 register (bit OC1PE). Else the preload value is copied in the active capture/compare 1 register when an update event occurs. The active capture/compare register contains the value to be compared to the counter TIM15_CNT and signaled on tim_oc1 output. Non-dithering mode (DITHEN = 0) The register holds the compare value in CCR1\[15:0\]. The CCR1\[19:16\] bits are reset. Dithering mode (DITHEN = 1) The register holds the integer part in CCR1\[19:4\]. The CCR1\[3:0\] bitfield contains the dithered part. If channel CC1 is configured as input: CR1 is the counter value transferred by the last input capture 1 event (tim_ic1). The TIMx_CCR1 register is read-only and cannot be programmed. Non-dithering mode (DITHEN = 0) The register holds the capture value in CCR1\[15:0\]. The CCR1\[19:16\] bits are reset. Dithering mode (DITHEN = 1) The register holds the capture in CCR1\[19:4\]. The CCR1\[3:0\] bits are reset.
    pub mod CCR1 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (20 bits: 0xfffff << 0)
        pub const mask: u32 = 0xfffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// TIM15 capture/compare register 2
pub mod TIM15_CCR2 {

    /// Capture/compare 2 value If channel CC2 is configured as output: CCR2 is the value to be loaded in the actual capture/compare 2 register (preload value). It is loaded permanently if the preload feature is not selected in the TIM15_CCMR2 register (bit OC2PE). Else the preload value is copied in the active capture/compare 2 register when an update event occurs. The active capture/compare register contains the value to be compared to the counter TIM15_CNT and signalled on tim_oc2 output. Non-dithering mode (DITHEN = 0) The register holds the compare value in CCR2\[15:0\]. The CCR2\[19:16\] bits are reset. Dithering mode (DITHEN = 1) The register holds the integer part in CCR2\[19:4\]. The CCR2\[3:0\] bitfield contains the dithered part. If channel CC2 is configured as input: CCR2 is the counter value transferred by the last input capture 1 event (tim_ic2). The TIMx_CCR2 register is read-only and cannot be programmed. Non-dithering mode (DITHEN = 0) The register holds the capture value in CCR2\[15:0\]. The CCR2\[19:16\] bits are reset. Dithering mode (DITHEN = 1) The register holds the capture in CCR2\[19:4\]. The CCR2\[3:0\] bits are reset.
    pub mod CCR2 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (20 bits: 0xfffff << 0)
        pub const mask: u32 = 0xfffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// TIM15 break and dead-time register
pub mod TIM15_BDTR {

    /// Dead-time generator setup This bit-field defines the duration of the dead-time inserted between the complementary outputs. DT correspond to this duration. DTG\[7:5\]=0xx => DT=DTG\[7:0\]x tdtg with tdtg=tDTS DTG\[7:5\]=10x => DT=(64+DTG\[5:0\])xtdtg with Tdtg=2xtDTS DTG\[7:5\]=110 => DT=(32+DTG\[4:0\])xtdtg with Tdtg=8xtDTS DTG\[7:5\]=111 => DT=(32+DTG\[4:0\])xtdtg with Tdtg=16xtDTS Example if TDTS=125ns (8MHz), dead-time possible values are: 0 to 15875 ns by 125 ns steps, 16 �s to 31750 ns by 250 ns steps, 32 �s to 63 �s by 1 �s steps, 64 �s to 126 �s by 2 �s steps Note: This bit-field can not be modified as long as LOCK level 1, 2 or 3 has been programmed (LOCK bits in TIM15_BDTR register).
    pub mod DTG {
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

    /// Lock configuration These bits offer a write protection against software errors. Note: The LOCK bits can be written only once after the reset. Once the TIM15_BDTR register has been written, their content is frozen until the next reset.
    pub mod LOCK {
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

    /// Off-state selection for Idle mode This bit is used when MOE=0 on channels configured as outputs. See tim_ocx/tim_ocxn enable description for more details (capture/compare enable register (TIM15_CCER) on page 1985). Note: This bit can not be modified as soon as the LOCK level 2 has been programmed (LOCK bits in TIM15_BDTR register).
    pub mod OSSI {
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

    /// Off-state selection for Run mode This bit is used when MOE=1 on channels that have a complementary output which are configured as outputs. OSSR is not implemented if no complementary output is implemented in the timer. See tim_ocx/tim_ocxn enable description for more details (capture/compare enable register (TIM15_CCER) on page 1985). Note: This bit can not be modified as soon as the LOCK level 2 has been programmed (LOCK bits in TIM15_BDTR register).
    pub mod OSSR {
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

    /// Break enable 1; Break inputs (tim_brk and tim_sys_brk clock failure event) enabled This bit cannot be modified when LOCK level 1 has been programmed (LOCK bits in TIM15_BDTR register). Note: Any write operation to this bit takes a delay of 1 APB clock cycle to become effective.
    pub mod BKE {
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

    /// Break polarity Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIM15_BDTR register). Any write operation to this bit takes a delay of 1 APB clock cycle to become effective.
    pub mod BKP {
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

    /// Automatic output enable Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIM15_BDTR register).
    pub mod AOE {
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

    /// Main output enable This bit is cleared asynchronously by hardware as soon as the tim_brk input is active. It is set by software or automatically depending on the AOE bit. It is acting only on the channels which are configured in output. See tim_ocx/tim_ocxn enable description for more details (capture/compare enable register (TIM15_CCER) on page 1985).
    pub mod MOE {
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

    /// Break filter This bit-field defines the frequency used to sample the tim_brk input signal and the length of the digital filter applied to tim_brk. The digital filter is made of an event counter in which N events are needed to validate a transition on the output: Note: This bit cannot be modified when LOCK level 1 has been programmed (LOCK bits in TIM15_BDTR register).
    pub mod BKF {
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

    /// Break disarm This bit is cleared by hardware when no break source is active. The BKDSRM bit must be set by software to release the bidirectional output control (open-drain output in Hi-Z state) and then be polled until it is reset by hardware, indicating that the fault condition has disappeared. Note: Any write operation to this bit takes a delay of 1 APB clock cycle to become effective.
    pub mod BKDSRM {
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

    /// Break bidirectional In the bidirectional mode (BKBID bit set to 1), the break input is configured both in input mode and in open drain output mode. Any active break event asserts a low logic level on the Break input to indicate an internal break event to external devices. Note: This bit cannot be modified as long as LOCK level 1 has been programmed (LOCK bits in TIM15_BDTR register). Note: Any write operation to this bit takes a delay of 1 APB clock cycle to become effective.
    pub mod BKBID {
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

/// TIM15 timer deadtime register 2
pub mod TIM15_DTR2 {

    /// Dead-time falling edge generator setup This bit-field defines the duration of the dead-time inserted between the complementary outputs, on the falling edge. DTGF\[7:5\]=0xx => DTF=DTGF\[7:0\]x tdtg with tdtg=tDTS. DTGF\[7:5\]=10x => DTF=(64+DTGF\[5:0\])xtdtg with Tdtg=2xtDTS. DTGF\[7:5\]=110 => DTF=(32+DTGF\[4:0\])xtdtg with Tdtg=8xtDTS. DTGF\[7:5\]=111 => DTF=(32+DTGF\[4:0\])xtdtg with Tdtg=16xtDTS. Example if TDTS=125ns (8MHz), dead-time possible values are: 0 to 15875 ns by 125 ns steps, 16 us to 31750 ns by 250 ns steps, 32 us to 63us by 1 us steps, 64 us to 126 us by 2 us steps Note: This bit-field can not be modified as long as LOCK level 1, 2 or 3 has been programmed (LOCK bits in TIM15_BDTR register).
    pub mod DTGF {
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

    /// Deadtime asymmetric enable Note: This bit can not be modified as long as LOCK level 1, 2 or 3 has been programmed (LOCK bits in TIM15_BDTR register).
    pub mod DTAE {
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

    /// Deadtime preload enable Note: This bit can not be modified as long as LOCK level 1, 2 or 3 has been programmed (LOCK bits in TIM15_BDTR register).
    pub mod DTPE {
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
}

/// TIM15 input selection register
pub mod TIM15_TISEL {

    /// selects tim_ti1_in\[0..15\] input ... Refer to for interconnects list.
    pub mod TI1SEL {
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

    /// selects tim_ti2_in\[0..15\] input ... Refer to for interconnects list.
    pub mod TI2SEL {
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
}

/// TIM15 alternate function register 1
pub mod TIM15_AF1 {

    /// TIMx_BKIN input enable This bit enables the TIMx_BKIN alternate function input for the timer’s tim_brk input. TIMx_BKIN input is ‘ORed’ with the other tim_brk sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIM15_BDTR register).
    pub mod BKINE {
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

    /// tim_brk_cmp1 enable This bit enables the tim_brk_cmp1 for the timer’s tim_brk input. tim_brk_cmp1 output is ‘ORed’ with the other tim_brk sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIM15_BDTR register).
    pub mod BKCMP1E {
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

    /// tim_brk_cmp2 enable This bit enables the tim_brk_cmp2 for the timer’s tim_brk input. tim_brk_cmp2 output is ‘ORed’ with the other tim_brk sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIM15_BDTR register).
    pub mod BKCMP2E {
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

    /// tim_brk_cmp3 enable This bit enables the tim_brk_cmp3 for the timer’s tim_brk input. tim_brk_cmp3 output is ‘ORed’ with the other tim_brk sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIM15_BDTR register).
    pub mod BKCMP3E {
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

    /// tim_brk_cmp4 enable This bit enables the tim_brk_cmp4 for the timer’s tim_brk input. tim_brk_cmp4 output is ‘ORed’ with the other tim_brk sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIM15_BDTR register).
    pub mod BKCMP4E {
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

    /// tim_brk_cmp5 enable This bit enables the tim_brk_cmp5 for the timer’s tim_brk input. tim_brk_cmp5 output is ‘ORed’ with the other tim_brk sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIM15_BDTR register).
    pub mod BKCMP5E {
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

    /// tim_brk_cmp6 enable This bit enables the tim_brk_cmp6 for the timer’s tim_brk input. tim_brk_cmp6 output is ‘ORed’ with the other tim_brk sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIM15_BDTR register).
    pub mod BKCMP6E {
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

    /// tim_brk_cmp7 enable This bit enables the tim_brk_cmp7 for the timer’s tim_brk input. COMP7 output is ‘ORed’ with the other tim_brk sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIM15_BDTR register).
    pub mod BKCMP7E {
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

    /// tim_brk_cmp8 enable This bit enables the tim_brk_cmp8 for the timer’s tim_brk input. mdf_brkx output is ‘ORed’ with the other tim_brk sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIM15_BDTR register).
    pub mod BKCMP8E {
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

    /// TIMx_BKIN input polarity This bit selects the TIMx_BKIN alternate function input sensitivity. It must be programmed together with the BKP polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIM15_BDTR register).
    pub mod BKINP {
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

    /// tim_brk_cmp1 input polarity This bit selects the tim_brk_cmp1 input sensitivity. It must be programmed together with the BKP polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIM15_BDTR register).
    pub mod BKCMP1P {
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

    /// tim_brk_cmp2 input polarity This bit selects the tim_brk_cmp2 input sensitivity. It must be programmed together with the BKP polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIM15_BDTR register).
    pub mod BKCMP2P {
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

    /// tim_brk_cmp3 input polarity This bit selects the tim_brk_cmp3 input sensitivity. It must be programmed together with the BKP polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIM15_BDTR register).
    pub mod BKCMP3P {
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

    /// tim_brk_cmp4 input polarity This bit selects the tim_brk_cmp4 input sensitivity. It must be programmed together with the BKP polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIM15_BDTR register).
    pub mod BKCMP4P {
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
}

/// TIM15 alternate function register 2
pub mod TIM15_AF2 {

    /// ocref_clr source selection These bits select the ocref_clr input source. Refer to for product specific implementation. Note: These bits can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIM15_BDTR register).
    pub mod OCRSEL {
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

/// TIM15 DMA control register
pub mod TIM15_DCR {

    /// DMA base address This 5-bit field defines the base-address for DMA transfers (when read/write access are done through the TIM15_DMAR address). DBA is defined as an offset starting from the address of the TIM15_CR1 register. Example: ...
    pub mod DBA {
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

    /// DMA burst length This 5-bit field defines the length of DMA transfers (the timer recognizes a burst transfer when a read or a write access is done to the TIM15_DMAR address). ...
    pub mod DBL {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (5 bits: 0b11111 << 8)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// DMA burst source selection This bitfield defines the interrupt source that triggers the DMA burst transfers (the timer recognizes a burst transfer when a read or a write access is done to the TIMx_DMAR address). Other: reserved
    pub mod DBSS {
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
}

/// TIM15 DMA address for full transfer
pub mod TIM15_DMAR {

    /// DMA register for burst accesses A read or write operation to the DMAR register accesses the register located at the address (TIM15_CR1 address) + (DBA + DMA index) x 4 where TIM15_CR1 address is the address of the control register 1, DBA is the DMA base address configured in TIM15_DCR register, DMA index is automatically controlled by the DMA transfer, and ranges from 0 to DBL (DBL configured in TIM15_DCR).
    pub mod DMAB {
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
    /// TIM15 control register 1
    pub TIM15_CR1: RWRegister<u16>,

    _reserved1: [u8; 2],

    /// TIM15 control register 2
    pub TIM15_CR2: RWRegister<u16>,

    _reserved2: [u8; 2],

    /// TIM15 slave mode control register
    pub TIM15_SMCR: RWRegister<u32>,

    /// TIM15 DMA/interrupt enable register
    pub TIM15_DIER: RWRegister<u16>,

    _reserved3: [u8; 2],

    /// TIM15 status register
    pub TIM15_SR: RWRegister<u16>,

    _reserved4: [u8; 2],

    /// TIM15 event generation register
    pub TIM15_EGR: RWRegister<u16>,

    _reserved5: [u8; 2],

    /// TIM15_CCMR1_Input and TIM15_CCMR1_Output
    /// TIM15_CCMR1_Input: TIM15 capture/compare mode register 1 \\[alternate\\]
    /// TIM15_CCMR1_Output: TIM15 capture/compare mode register 1 \\[alternate\\]
    pub TIM15_CCMR1: RWRegister<u32>,

    _reserved6: [u8; 4],

    /// TIM15 capture/compare enable register
    pub TIM15_CCER: RWRegister<u16>,

    _reserved7: [u8; 2],

    /// TIM15 counter
    pub TIM15_CNT: RWRegister<u32>,

    /// TIM15 prescaler
    pub TIM15_PSC: RWRegister<u16>,

    _reserved8: [u8; 2],

    /// TIM15 auto-reload register
    pub TIM15_ARR: RWRegister<u32>,

    /// TIM15 repetition counter register
    pub TIM15_RCR: RWRegister<u16>,

    _reserved9: [u8; 2],

    /// TIM15 capture/compare register 1
    pub TIM15_CCR1: RWRegister<u32>,

    /// TIM15 capture/compare register 2
    pub TIM15_CCR2: RWRegister<u32>,

    _reserved10: [u8; 8],

    /// TIM15 break and dead-time register
    pub TIM15_BDTR: RWRegister<u32>,

    _reserved11: [u8; 12],

    /// TIM15 timer deadtime register 2
    pub TIM15_DTR2: RWRegister<u32>,

    _reserved12: [u8; 4],

    /// TIM15 input selection register
    pub TIM15_TISEL: RWRegister<u32>,

    /// TIM15 alternate function register 1
    pub TIM15_AF1: RWRegister<u32>,

    /// TIM15 alternate function register 2
    pub TIM15_AF2: RWRegister<u32>,

    _reserved13: [u8; 884],

    /// TIM15 DMA control register
    pub TIM15_DCR: RWRegister<u32>,

    /// TIM15 DMA address for full transfer
    pub TIM15_DMAR: RWRegister<u32>,
}
pub struct ResetValues {
    pub TIM15_CR1: u16,
    pub TIM15_CR2: u16,
    pub TIM15_SMCR: u32,
    pub TIM15_DIER: u16,
    pub TIM15_SR: u16,
    pub TIM15_EGR: u16,
    pub TIM15_CCMR1: u32,
    pub TIM15_CCER: u16,
    pub TIM15_CNT: u32,
    pub TIM15_PSC: u16,
    pub TIM15_ARR: u32,
    pub TIM15_RCR: u16,
    pub TIM15_CCR1: u32,
    pub TIM15_CCR2: u32,
    pub TIM15_BDTR: u32,
    pub TIM15_DTR2: u32,
    pub TIM15_TISEL: u32,
    pub TIM15_AF1: u32,
    pub TIM15_AF2: u32,
    pub TIM15_DCR: u32,
    pub TIM15_DMAR: u32,
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