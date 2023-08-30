#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Ethernet media access control
//!
//! Used by: stm32h563, stm32h573

#[cfg(not(feature = "nosync"))]
pub use crate::stm32h5::peripherals::eth::Instance;
pub use crate::stm32h5::peripherals::eth::{RegisterBlock, ResetValues};
pub use crate::stm32h5::peripherals::eth::{
    DMACCARXBR, DMACCARXDR, DMACCATXBR, DMACCATXDR, DMACCR, DMACIER, DMACMFCR, DMACRXCR,
    DMACRXDLAR, DMACRXDTPR, DMACRXIWTR, DMACRXRLR, DMACSR, DMACTXCR, DMACTXDLAR, DMACTXDTPR,
    DMACTXRLR, DMADSR, DMAISR, DMAMR, DMASBMR, MAC1USTCR, MACA0HR, MACA0LR, MACA1HR, MACA1LR,
    MACA2HR, MACA2LR, MACA3HR, MACA3LR, MACACR, MACARPAR, MACATSNR, MACATSSR, MACCR, MACCSRSWCR,
    MACDR, MACECR, MACHT0R, MACHT1R, MACHWF0R, MACHWF1R, MACHWF2R, MACHWF3R, MACIER, MACISR,
    MACIVIR, MACL3A00R, MACL3A01R, MACL3A10R, MACL3A11R, MACL3A20R, MACL3A21R, MACL3A30R,
    MACL3A31R, MACL3L4C0R, MACL3L4C1R, MACL4A0R, MACL4A1R, MACLCSR, MACLETR, MACLMIR, MACLTCR,
    MACMDIOAR, MACMDIODR, MACPCSR, MACPFR, MACPOCR, MACPPSCR, MACPPSIR, MACPPSTTNR, MACPPSTTSR,
    MACPPSWR, MACQTXFCR, MACRWKPFR, MACRXFCR, MACRXTXSR, MACSPI0R, MACSPI1R, MACSPI2R, MACSSIR,
    MACSTNR, MACSTNUR, MACSTSR, MACSTSUR, MACTSAR, MACTSCR, MACTSEACR, MACTSECNR, MACTSIACR,
    MACTSICNR, MACTSSR, MACTXTSSNR, MACTXTSSSR, MACVHTR, MACVIR, MACVR, MACVTR, MACWTR,
    MMC_CONTROL, MMC_RX_INTERRUPT, MMC_RX_INTERRUPT_MASK, MMC_TX_INTERRUPT, MMC_TX_INTERRUPT_MASK,
    MTLISR, MTLOMR, MTLQICSR, MTLRXQDR, MTLRXQMPOCR, MTLRXQOMR, MTLTXQDR, MTLTXQOMR, MTLTXQUR,
    RX_ALIGNMENT_ERROR_PACKETS, RX_CRC_ERROR_PACKETS, RX_LPI_TRAN_CNTR, RX_LPI_USEC_CNTR,
    RX_UNICAST_PACKETS_GOOD, TX_LPI_TRAN_CNTR, TX_LPI_USEC_CNTR,
    TX_MULTIPLE_COLLISION_GOOD_PACKETS, TX_PACKET_COUNT_GOOD, TX_SINGLE_COLLISION_GOOD_PACKETS,
};

/// Access functions for the ETH peripheral instance
pub mod ETH {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x40028000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in ETH
    pub const reset: ResetValues = ResetValues {
        MACCR: 0x00000000,
        MACECR: 0x00000000,
        MACPFR: 0x00000000,
        MACWTR: 0x00000000,
        MACHT0R: 0x00000000,
        MACHT1R: 0x00000000,
        MACVTR: 0x00000000,
        MACVHTR: 0x00000000,
        MACVIR: 0x00000000,
        MACIVIR: 0x00000000,
        MACQTXFCR: 0x00000000,
        MACRXFCR: 0x00000000,
        MACISR: 0x00000000,
        MACIER: 0x00000000,
        MACRXTXSR: 0x00000000,
        MACPCSR: 0x00000000,
        MACRWKPFR: 0x00000000,
        MACLCSR: 0x00000000,
        MACLTCR: 0x03E80000,
        MACLETR: 0x00000000,
        MAC1USTCR: 0x00000000,
        MACVR: 0x00003242,
        MACDR: 0x00000000,
        MACHWF0R: 0x0A0D73F7,
        MACHWF1R: 0x11041904,
        MACHWF2R: 0x41000000,
        MACHWF3R: 0x00000020,
        MACMDIOAR: 0x00000000,
        MACMDIODR: 0x00000000,
        MACARPAR: 0x00000000,
        MACCSRSWCR: 0x00000000,
        MACA0HR: 0x8000FFFF,
        MACA0LR: 0xFFFFFFFF,
        MACA1HR: 0x0000FFFF,
        MACA1LR: 0xFFFFFFFF,
        MACA2HR: 0x0000FFFF,
        MACA2LR: 0xFFFFFFFF,
        MACA3HR: 0x0000FFFF,
        MACA3LR: 0xFFFFFFFF,
        MMC_CONTROL: 0x00000000,
        MMC_RX_INTERRUPT: 0x00000000,
        MMC_TX_INTERRUPT: 0x00000000,
        MMC_RX_INTERRUPT_MASK: 0x00000000,
        MMC_TX_INTERRUPT_MASK: 0x00000000,
        TX_SINGLE_COLLISION_GOOD_PACKETS: 0x00000000,
        TX_MULTIPLE_COLLISION_GOOD_PACKETS: 0x00000000,
        TX_PACKET_COUNT_GOOD: 0x00000000,
        RX_CRC_ERROR_PACKETS: 0x00000000,
        RX_ALIGNMENT_ERROR_PACKETS: 0x00000000,
        RX_UNICAST_PACKETS_GOOD: 0x00000000,
        TX_LPI_USEC_CNTR: 0x00000000,
        TX_LPI_TRAN_CNTR: 0x00000000,
        RX_LPI_USEC_CNTR: 0x00000000,
        RX_LPI_TRAN_CNTR: 0x00000000,
        MACL3L4C0R: 0x00000000,
        MACL4A0R: 0x00000000,
        MACL3A00R: 0x00000000,
        MACL3A10R: 0x00000000,
        MACL3A20R: 0x00000000,
        MACL3A30R: 0x00000000,
        MACL3L4C1R: 0x00000000,
        MACL4A1R: 0x00000000,
        MACL3A01R: 0x00000000,
        MACL3A11R: 0x00000000,
        MACL3A21R: 0x00000000,
        MACL3A31R: 0x00000000,
        MACTSCR: 0x00002000,
        MACSSIR: 0x00000000,
        MACSTSR: 0x00000000,
        MACSTNR: 0x00000000,
        MACSTSUR: 0x00000000,
        MACSTNUR: 0x00000000,
        MACTSAR: 0x00000000,
        MACTSSR: 0x00000000,
        MACTXTSSNR: 0x00000000,
        MACTXTSSSR: 0x00000000,
        MACACR: 0x00000000,
        MACATSNR: 0x00000000,
        MACATSSR: 0x00000000,
        MACTSIACR: 0x00000000,
        MACTSEACR: 0x00000000,
        MACTSICNR: 0x00000000,
        MACTSECNR: 0x00000000,
        MACPPSCR: 0x00000000,
        MACPPSTTSR: 0x00000000,
        MACPPSTTNR: 0x00000000,
        MACPPSIR: 0x00000000,
        MACPPSWR: 0x00000000,
        MACPOCR: 0x00000000,
        MACSPI0R: 0x00000000,
        MACSPI1R: 0x00000000,
        MACSPI2R: 0x00000000,
        MACLMIR: 0x00000000,
        MTLOMR: 0x00000000,
        MTLISR: 0x00000000,
        MTLTXQOMR: 0x00070008,
        MTLTXQUR: 0x00000000,
        MTLTXQDR: 0x00000000,
        MTLQICSR: 0x00000000,
        MTLRXQOMR: 0x00700000,
        MTLRXQMPOCR: 0x00000000,
        MTLRXQDR: 0x00000000,
        DMAMR: 0x00000000,
        DMASBMR: 0x00000000,
        DMAISR: 0x00000000,
        DMADSR: 0x00000000,
        DMACCR: 0x00000000,
        DMACTXCR: 0x00000000,
        DMACRXCR: 0x00000000,
        DMACTXDLAR: 0x00000000,
        DMACRXDLAR: 0x00000000,
        DMACTXDTPR: 0x00000000,
        DMACRXDTPR: 0x00000000,
        DMACTXRLR: 0x00000000,
        DMACRXRLR: 0x00000000,
        DMACIER: 0x00000000,
        DMACRXIWTR: 0x00000000,
        DMACCATXDR: 0x00000000,
        DMACCARXDR: 0x00000000,
        DMACCATXBR: 0x00000000,
        DMACCARXBR: 0x00000000,
        DMACSR: 0x00000000,
        DMACMFCR: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut ETH_TAKEN: bool = false;

    /// Safe access to ETH
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
            if ETH_TAKEN {
                None
            } else {
                ETH_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to ETH
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if ETH_TAKEN && inst.addr == INSTANCE.addr {
                ETH_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal ETH
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        ETH_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to ETH
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const ETH: *const RegisterBlock = 0x40028000 as *const _;

/// Access functions for the SEC_ETH peripheral instance
pub mod SEC_ETH {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x50028000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in SEC_ETH
    pub const reset: ResetValues = ResetValues {
        MACCR: 0x00000000,
        MACECR: 0x00000000,
        MACPFR: 0x00000000,
        MACWTR: 0x00000000,
        MACHT0R: 0x00000000,
        MACHT1R: 0x00000000,
        MACVTR: 0x00000000,
        MACVHTR: 0x00000000,
        MACVIR: 0x00000000,
        MACIVIR: 0x00000000,
        MACQTXFCR: 0x00000000,
        MACRXFCR: 0x00000000,
        MACISR: 0x00000000,
        MACIER: 0x00000000,
        MACRXTXSR: 0x00000000,
        MACPCSR: 0x00000000,
        MACRWKPFR: 0x00000000,
        MACLCSR: 0x00000000,
        MACLTCR: 0x03E80000,
        MACLETR: 0x00000000,
        MAC1USTCR: 0x00000000,
        MACVR: 0x00003242,
        MACDR: 0x00000000,
        MACHWF0R: 0x0A0D73F7,
        MACHWF1R: 0x11041904,
        MACHWF2R: 0x41000000,
        MACHWF3R: 0x00000020,
        MACMDIOAR: 0x00000000,
        MACMDIODR: 0x00000000,
        MACARPAR: 0x00000000,
        MACCSRSWCR: 0x00000000,
        MACA0HR: 0x8000FFFF,
        MACA0LR: 0xFFFFFFFF,
        MACA1HR: 0x0000FFFF,
        MACA1LR: 0xFFFFFFFF,
        MACA2HR: 0x0000FFFF,
        MACA2LR: 0xFFFFFFFF,
        MACA3HR: 0x0000FFFF,
        MACA3LR: 0xFFFFFFFF,
        MMC_CONTROL: 0x00000000,
        MMC_RX_INTERRUPT: 0x00000000,
        MMC_TX_INTERRUPT: 0x00000000,
        MMC_RX_INTERRUPT_MASK: 0x00000000,
        MMC_TX_INTERRUPT_MASK: 0x00000000,
        TX_SINGLE_COLLISION_GOOD_PACKETS: 0x00000000,
        TX_MULTIPLE_COLLISION_GOOD_PACKETS: 0x00000000,
        TX_PACKET_COUNT_GOOD: 0x00000000,
        RX_CRC_ERROR_PACKETS: 0x00000000,
        RX_ALIGNMENT_ERROR_PACKETS: 0x00000000,
        RX_UNICAST_PACKETS_GOOD: 0x00000000,
        TX_LPI_USEC_CNTR: 0x00000000,
        TX_LPI_TRAN_CNTR: 0x00000000,
        RX_LPI_USEC_CNTR: 0x00000000,
        RX_LPI_TRAN_CNTR: 0x00000000,
        MACL3L4C0R: 0x00000000,
        MACL4A0R: 0x00000000,
        MACL3A00R: 0x00000000,
        MACL3A10R: 0x00000000,
        MACL3A20R: 0x00000000,
        MACL3A30R: 0x00000000,
        MACL3L4C1R: 0x00000000,
        MACL4A1R: 0x00000000,
        MACL3A01R: 0x00000000,
        MACL3A11R: 0x00000000,
        MACL3A21R: 0x00000000,
        MACL3A31R: 0x00000000,
        MACTSCR: 0x00002000,
        MACSSIR: 0x00000000,
        MACSTSR: 0x00000000,
        MACSTNR: 0x00000000,
        MACSTSUR: 0x00000000,
        MACSTNUR: 0x00000000,
        MACTSAR: 0x00000000,
        MACTSSR: 0x00000000,
        MACTXTSSNR: 0x00000000,
        MACTXTSSSR: 0x00000000,
        MACACR: 0x00000000,
        MACATSNR: 0x00000000,
        MACATSSR: 0x00000000,
        MACTSIACR: 0x00000000,
        MACTSEACR: 0x00000000,
        MACTSICNR: 0x00000000,
        MACTSECNR: 0x00000000,
        MACPPSCR: 0x00000000,
        MACPPSTTSR: 0x00000000,
        MACPPSTTNR: 0x00000000,
        MACPPSIR: 0x00000000,
        MACPPSWR: 0x00000000,
        MACPOCR: 0x00000000,
        MACSPI0R: 0x00000000,
        MACSPI1R: 0x00000000,
        MACSPI2R: 0x00000000,
        MACLMIR: 0x00000000,
        MTLOMR: 0x00000000,
        MTLISR: 0x00000000,
        MTLTXQOMR: 0x00070008,
        MTLTXQUR: 0x00000000,
        MTLTXQDR: 0x00000000,
        MTLQICSR: 0x00000000,
        MTLRXQOMR: 0x00700000,
        MTLRXQMPOCR: 0x00000000,
        MTLRXQDR: 0x00000000,
        DMAMR: 0x00000000,
        DMASBMR: 0x00000000,
        DMAISR: 0x00000000,
        DMADSR: 0x00000000,
        DMACCR: 0x00000000,
        DMACTXCR: 0x00000000,
        DMACRXCR: 0x00000000,
        DMACTXDLAR: 0x00000000,
        DMACRXDLAR: 0x00000000,
        DMACTXDTPR: 0x00000000,
        DMACRXDTPR: 0x00000000,
        DMACTXRLR: 0x00000000,
        DMACRXRLR: 0x00000000,
        DMACIER: 0x00000000,
        DMACRXIWTR: 0x00000000,
        DMACCATXDR: 0x00000000,
        DMACCARXDR: 0x00000000,
        DMACCATXBR: 0x00000000,
        DMACCARXBR: 0x00000000,
        DMACSR: 0x00000000,
        DMACMFCR: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut SEC_ETH_TAKEN: bool = false;

    /// Safe access to SEC_ETH
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
            if SEC_ETH_TAKEN {
                None
            } else {
                SEC_ETH_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to SEC_ETH
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if SEC_ETH_TAKEN && inst.addr == INSTANCE.addr {
                SEC_ETH_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal SEC_ETH
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        SEC_ETH_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to SEC_ETH
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const SEC_ETH: *const RegisterBlock = 0x50028000 as *const _;
