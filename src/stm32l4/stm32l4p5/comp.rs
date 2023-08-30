#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Comparator

use crate::RWRegister;
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// Comparator 1 control and status register
pub mod COMP1_CSR {

    /// Comparator 1 enable bit
    pub mod EN {
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

            /// 0b0: Comparator x switched OFF
            pub const Disabled: u32 = 0b0;

            /// 0b1: Comparator x switched ON
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Power Mode of the comparator 1
    pub mod PWRMODE {
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

            /// 0b00: High speed
            pub const High: u32 = 0b00;

            /// 0b01: Medium speed
            pub const Medium: u32 = 0b01;

            /// 0b11: Ultra low power
            pub const Low: u32 = 0b11;
        }
    }

    /// Comparator 1 Input Minus connection configuration bit
    pub mod INMSEL {
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

    /// Comparator1 input plus selection bit
    pub mod INPSEL {
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

            /// 0b0: external IO - PC5
            pub const External: u32 = 0b0;

            /// 0b1: PB2
            pub const PB2: u32 = 0b1;
        }
    }

    /// Comparator 1 polarity selection bit
    pub mod POLARITY {
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

            /// 0b0: Comparator x output value not inverted
            pub const Normal: u32 = 0b0;

            /// 0b1: Comparator x output value inverted
            pub const Inverted: u32 = 0b1;
        }
    }

    /// Comparator 1 hysteresis selection bits
    pub mod HYST {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (2 bits: 0b11 << 16)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: No hysteresis
            pub const None: u32 = 0b00;

            /// 0b01: Low hysteresis
            pub const Low: u32 = 0b01;

            /// 0b10: Medium hysteresis
            pub const Medium: u32 = 0b10;

            /// 0b11: High hysteresis
            pub const High: u32 = 0b11;
        }
    }

    /// Comparator 1 blanking source selection bits
    pub mod BLANKING {
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

            /// 0b000: No blanking
            pub const Disabled: u32 = 0b000;

            /// 0b001: TIM1 OC5 selected as blanking source
            pub const TIM1OC5: u32 = 0b001;

            /// 0b010: TIM2 OC3 selected as blanking source
            pub const TIM2OC3: u32 = 0b010;
        }
    }

    /// Scaler bridge enable
    pub mod BRGEN {
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

            /// 0b0: Scaler resistor bridge disabled (if BRGEN bit of COMP2_CSR register is also reset)
            pub const Disabled: u32 = 0b0;

            /// 0b1: Scaler resistor bridge enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Voltage scaler enable bit
    pub mod SCALEN {
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

            /// 0b0: Bandgap scaler disabled (if SCALEN bit of COMP2_CSR register is also reset)
            pub const Disabled: u32 = 0b0;

            /// 0b1: Bandgap scaler enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Comparator 1 output status bit
    pub mod VALUE {
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

    /// COMP1_CSR register lock bit
    pub mod LOCK {
        /// Offset (31 bits)
        pub const offset: u32 = 31;
        /// Mask (1 bit: 1 << 31)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values
        pub mod W {

            /// 0b0: COMPx_CSR\[31:0\] for comparator x are read/write
            pub const Unlocked: u32 = 0b0;

            /// 0b1: COMPx_CSR\[31:0\] for comparator x are read-only
            pub const Locked: u32 = 0b1;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Comparator 2 control and status register
pub mod COMP2_CSR {

    /// Comparator 2 enable bit
    pub mod EN {
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

            /// 0b0: Comparator x switched OFF
            pub const Disabled: u32 = 0b0;

            /// 0b1: Comparator x switched ON
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Power Mode of the comparator 2
    pub mod PWRMODE {
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

            /// 0b00: High speed
            pub const High: u32 = 0b00;

            /// 0b01: Medium speed
            pub const Medium: u32 = 0b01;

            /// 0b11: Ultra low power
            pub const Low: u32 = 0b11;
        }
    }

    /// Comparator 2 Input Minus connection configuration bit
    pub mod INMSEL {
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

    /// Comparator 2 Input Plus connection configuration bit
    pub mod INPSEL {
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

            /// 0b0: PB4
            pub const PB4: u32 = 0b0;

            /// 0b1: PB6
            pub const PB6: u32 = 0b1;
        }
    }

    /// Windows mode selection bit
    pub mod WINMODE {
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

            /// 0b0: Input plus of Comparator 2 is not connected to Comparator 1
            pub const Disabled: u32 = 0b0;

            /// 0b1: Input plus of Comparator 2 is connected with input plus of Comparator 1
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Comparator 2 polarity selection bit
    pub mod POLARITY {
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

            /// 0b0: Comparator x output value not inverted
            pub const Normal: u32 = 0b0;

            /// 0b1: Comparator x output value inverted
            pub const Inverted: u32 = 0b1;
        }
    }

    /// Comparator 2 hysteresis selection bits
    pub mod HYST {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (2 bits: 0b11 << 16)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: No hysteresis
            pub const None: u32 = 0b00;

            /// 0b01: Low hysteresis
            pub const Low: u32 = 0b01;

            /// 0b10: Medium hysteresis
            pub const Medium: u32 = 0b10;

            /// 0b11: High hysteresis
            pub const High: u32 = 0b11;
        }
    }

    /// Comparator 2 blanking source selection bits
    pub mod BLANKING {
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

            /// 0b000: No blanking
            pub const Disabled: u32 = 0b000;

            /// 0b100: TIM15 OC1 selected as blanking source
            pub const TIM15OC1: u32 = 0b100;
        }
    }

    /// Scaler bridge enable
    pub mod BRGEN {
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

            /// 0b0: Scaler resistor bridge disabled (if BRGEN bit of COMP2_CSR register is also reset)
            pub const Disabled: u32 = 0b0;

            /// 0b1: Scaler resistor bridge enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Voltage scaler enable bit
    pub mod SCALEN {
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

            /// 0b0: Bandgap scaler disabled (if SCALEN bit of COMP2_CSR register is also reset)
            pub const Disabled: u32 = 0b0;

            /// 0b1: Bandgap scaler enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Comparator 2 output status bit
    pub mod VALUE {
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

    /// COMP2_CSR register lock bit
    pub mod LOCK {
        /// Offset (31 bits)
        pub const offset: u32 = 31;
        /// Mask (1 bit: 1 << 31)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values
        pub mod W {

            /// 0b0: COMPx_CSR\[31:0\] for comparator x are read/write
            pub const Unlocked: u32 = 0b0;

            /// 0b1: COMPx_CSR\[31:0\] for comparator x are read-only
            pub const Locked: u32 = 0b1;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }
}
#[repr(C)]
pub struct RegisterBlock {
    /// Comparator 1 control and status register
    pub COMP1_CSR: RWRegister<u32>,

    /// Comparator 2 control and status register
    pub COMP2_CSR: RWRegister<u32>,
}
pub struct ResetValues {
    pub COMP1_CSR: u32,
    pub COMP2_CSR: u32,
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

/// Access functions for the COMP peripheral instance
pub mod COMP {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x40010200,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in COMP
    pub const reset: ResetValues = ResetValues {
        COMP1_CSR: 0x00000000,
        COMP2_CSR: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut COMP_TAKEN: bool = false;

    /// Safe access to COMP
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
            if COMP_TAKEN {
                None
            } else {
                COMP_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to COMP
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if COMP_TAKEN && inst.addr == INSTANCE.addr {
                COMP_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal COMP
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        COMP_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to COMP
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const COMP: *const RegisterBlock = 0x40010200 as *const _;
