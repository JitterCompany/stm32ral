//! Parent module for all STM32H5 devices.

/// Peripherals shared by multiple devices
pub mod peripherals;

/// Peripheral instances shared by multiple devices
pub(crate) mod instances;

#[cfg(any(feature="stm32h503", feature="doc"))]
pub mod stm32h503;

#[cfg(any(feature="stm32h562", feature="doc"))]
pub mod stm32h562;

#[cfg(any(feature="stm32h563", feature="doc"))]
pub mod stm32h563;

#[cfg(any(feature="stm32h573", feature="doc"))]
pub mod stm32h573;

