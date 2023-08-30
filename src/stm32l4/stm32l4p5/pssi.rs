#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Parallel synchronous slave interface

use crate::{RORegister, RWRegister, WORegister};
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// PSSI control register
pub mod CR {

    /// Data direction selection bit
    pub mod OUTEN {
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

            /// 0b0: Data is input synchronously with PSSI_PDCK
            pub const ReceiveMode: u32 = 0b0;

            /// 0b1: Data is output synchronously with PSSI_PDCK
            pub const TransmitMode: u32 = 0b1;
        }
    }

    /// DMA enable bit
    pub mod DMAEN {
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

            /// 0b0: DMA transfers are disabled. The user application can directly access the PSSI_DR register when DMA transfers are disabled.
            pub const Disabled: u32 = 0b0;

            /// 0b1: DMA transfers are enabled (default configuration). A DMA channel in the general-purpose DMA controller must be configured to perform transfers from/to PSSI_DR
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Data enable and ready configuration
    pub mod DERDYCFG {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (3 bits: 0b111 << 18)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: PSSI_DE and PSSI_RDY both disabled
            pub const Disabled: u32 = 0b000;

            /// 0b001: Only PSSI_RDY enabled
            pub const Rdy: u32 = 0b001;

            /// 0b010: Only PSSI_DE enabled
            pub const De: u32 = 0b010;

            /// 0b011: Both PSSI_RDY and PSSI_DE alternate functions enabled
            pub const RdyDeAlt: u32 = 0b011;

            /// 0b100: Both PSSI_RDY and PSSI_DE features enabled - bidirectional on PSSI_RDY pin
            pub const RdyDe: u32 = 0b100;

            /// 0b101: Only PSSI_RDY function enabled, but mapped to PSSI_DE pin
            pub const RdyRemapped: u32 = 0b101;

            /// 0b110: Only PSSI_DE function enabled, but mapped to PSSI_RDY pin
            pub const DeRemapped: u32 = 0b110;

            /// 0b111: Both PSSI_RDY and PSSI_DE features enabled - bidirectional on PSSI_DE pin
            pub const RdyDeBidi: u32 = 0b111;
        }
    }

    /// PSSI enable
    pub mod ENABLE {
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

            /// 0b0: PSSI disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: PSSI enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Extended data mode
    pub mod EDM {
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

            /// 0b00: Interface captures 8-bit data on every parallel data clock
            pub const BitWidth8: u32 = 0b00;

            /// 0b11: The interface captures 16-bit data on every parallel data clock
            pub const BitWidth16: u32 = 0b11;
        }
    }

    /// Ready (PSSI_RDY) polarity
    pub mod RDYPOL {
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

            /// 0b0: PSSI_RDY active low (0 indicates that the receiver is ready to receive)
            pub const ActiveLow: u32 = 0b0;

            /// 0b1: PSSI_RDY active high (1 indicates that the receiver is ready to receive)
            pub const ActiveHigh: u32 = 0b1;
        }
    }

    /// Data enable (PSSI_DE) polarity
    pub mod DEPOL {
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

            /// 0b0: PSSI_DE active low (0 indicates that data is valid)
            pub const ActiveLow: u32 = 0b0;

            /// 0b1: PSSI_DE active high (1 indicates that data is valid)
            pub const ActiveHigh: u32 = 0b1;
        }
    }

    /// Parallel data clock polarity
    pub mod CKPOL {
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

            /// 0b0: Falling edge active for inputs or rising edge active for outputs
            pub const FallingEdge: u32 = 0b0;

            /// 0b1: Rising edge active for inputs or falling edge active for outputs
            pub const RisingEdge: u32 = 0b1;
        }
    }
}

/// PSSI status register
pub mod SR {

    /// FIFO is ready to transfer one byte
    pub mod RTT1B {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: FIFO is not ready for a 1-byte transfer
            pub const NotReady: u32 = 0b0;

            /// 0b1: FIFO is ready for a one byte (32-bit) transfer. In receive mode, this means that at least one valid data byte is in the FIFO. In transmit mode, this means that there is at least one byte free in the FIFO
            pub const Ready: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// FIFO is ready to transfer four bytes
    pub mod RTT4B {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: FIFO is not ready for a four-byte transfer
            pub const NotReady: u32 = 0b0;

            /// 0b1: FIFO is ready for a four-byte (32-bit) transfer. In receive mode, this means that at least four valid data bytes are in the FIFO. In transmit mode, this means that there are at least four bytes free in the FIFO
            pub const Ready: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// PSSI raw interrupt status register
pub mod RIS {

    /// Data buffer overrun/underrun raw interrupt status
    pub mod OVR_RIS {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: No overrun/underrun occurred
            pub const Cleared: u32 = 0b0;

            /// 0b1: An overrun/underrun occurred: overrun in receive mode, underrun in transmit mode. This bit is cleared by writing a 1 to the OVR_ISC bit in PSSI_ICR
            pub const Occurred: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// PSSI interrupt enable register
pub mod IER {

    /// Data buffer overrun/underrun interrupt enable
    pub mod OVR_IE {
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

            /// 0b0: No interrupt generation
            pub const Disabled: u32 = 0b0;

            /// 0b1: An interrupt is generated if either an overrun or an underrun error occurred
            pub const Enabled: u32 = 0b1;
        }
    }
}

/// PSSI masked interrupt status register
pub mod MIS {

    /// Data buffer overrun/underrun masked interrupt status
    pub mod OVR_MIS {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: No interrupt is generated when an overrun/underrun error occurs
            pub const Disabled: u32 = 0b0;

            /// 0b1: An interrupt is generated if there is either an overrun or an underrun error and the OVR_IE bit is set in PSSI_IER
            pub const Enabled: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// PSSI interrupt clear register
pub mod ICR {

    /// Data buffer overrun/underrun interrupt status clear
    pub mod OVR_ISC {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values
        pub mod W {

            /// 0b1: Writing this bit to 1 clears the OVR_RIS bit in PSSI_RIS
            pub const Clear: u32 = 0b1;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// PSSI data register
pub mod DR {

    ///
    pub mod Byte3 {
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

    ///
    pub mod Byte2 {
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

    ///
    pub mod Byte1 {
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

    ///
    pub mod Byte0 {
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
    /// PSSI control register
    pub CR: RWRegister<u32>,

    /// PSSI status register
    pub SR: RORegister<u32>,

    /// PSSI raw interrupt status register
    pub RIS: RORegister<u32>,

    /// PSSI interrupt enable register
    pub IER: RWRegister<u32>,

    /// PSSI masked interrupt status register
    pub MIS: RORegister<u32>,

    /// PSSI interrupt clear register
    pub ICR: WORegister<u32>,

    _reserved1: [u8; 16],

    /// PSSI data register
    pub DR: RWRegister<u32>,
}
pub struct ResetValues {
    pub CR: u32,
    pub SR: u32,
    pub RIS: u32,
    pub IER: u32,
    pub MIS: u32,
    pub ICR: u32,
    pub DR: u32,
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

/// Access functions for the PSSI peripheral instance
pub mod PSSI {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x50050400,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in PSSI
    pub const reset: ResetValues = ResetValues {
        CR: 0x40000000,
        SR: 0x00000000,
        RIS: 0x00000000,
        IER: 0x00000000,
        MIS: 0x00000000,
        ICR: 0x00000000,
        DR: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut PSSI_TAKEN: bool = false;

    /// Safe access to PSSI
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
            if PSSI_TAKEN {
                None
            } else {
                PSSI_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to PSSI
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if PSSI_TAKEN && inst.addr == INSTANCE.addr {
                PSSI_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal PSSI
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        PSSI_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to PSSI
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const PSSI: *const RegisterBlock = 0x50050400 as *const _;
