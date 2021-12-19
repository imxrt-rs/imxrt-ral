// Copyright 2020 Tom Burdick
// See LICENSE-APACHE and LICENSE-MIT for license details.

//! This project provides a register access layer (RAL) for all
//! NXP i.mx rt microcontrollers.
//!
//! When built, you must specify a device feature, such as `imxrt1062`.
//! This will cause all modules in that device's module to be re-exported
//! from the top level, so that for example `imxrt_ral::gpio` will resolve to
//! `imxrt_ral::imxrt1062::gpio`.
//!
//! In the generated documentation, all devices are visible inside their family
//! modules, but when built for a specific device, only that devices' constants
//! will be available.
#![doc = include_str!("../usage.md")]
#![cfg_attr(target_arch = "arm", no_std)]
#![allow(clippy::all)]

mod register;

pub use crate::register::{modify_reg, read_reg, write_reg};
pub use crate::register::{RORegister, UnsafeRORegister};
pub use crate::register::{RWRegister, UnsafeRWRegister};
pub use crate::register::{UnsafeWORegister, WORegister};

#[cfg(feature = "doc")]
/// Interrupt sources
///
/// This enum is empty when generating documentation.
/// To see the specific interrupts for your chip, see
/// the `Interrupt` type in your chip-specific module, like
///
/// - [`imxrt101::imxrt1011::Interrupt`](imxrt101::imxrt1011::Interrupt)
/// - [`imxrt106::imxrt1062::Interrupt`](imxrt106::imxrt1062::Interrupt)
/// - etc
///
/// `Interrupt` resolves to those values when building the RAL for
/// your chip.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Interrupt {}

/// The constant for a peripheral with just one instance.
///
/// The CCM peripheral is an example of a "sole instance." On the other
/// hand, no LPUART peripheral will have this constant, since there are
/// multiple instances.
pub const SOLE_INSTANCE: u8 = 0;

mod private {
    pub trait Sealed {}
}

/// Implemented on all `Instance<N>` when `N` is a valid instance number.
pub trait Valid: private::Sealed {}
#[cfg(any(feature = "doc", feature = "imxrt1011", feature = "imxrt1015"))]
pub mod imxrt101;

#[cfg(feature = "imxrt1011")]
pub use imxrt101::imxrt1011::*;

#[cfg(feature = "imxrt1015")]
pub use imxrt101::imxrt1015::*;

#[cfg(any(feature = "doc", feature = "imxrt1021"))]
pub mod imxrt102;

#[cfg(feature = "imxrt1021")]
pub use imxrt102::imxrt1021::*;

#[cfg(any(feature = "doc", feature = "imxrt1051", feature = "imxrt1052"))]
pub mod imxrt105;

#[cfg(feature = "imxrt1051")]
pub use imxrt105::imxrt1051::*;

#[cfg(feature = "imxrt1052")]
pub use imxrt105::imxrt1052::*;

#[cfg(any(
    feature = "doc",
    feature = "imxrt1061",
    feature = "imxrt1062",
    feature = "imxrt1064"
))]
pub mod imxrt106;

#[cfg(feature = "imxrt1061")]
pub use imxrt106::imxrt1061::*;

#[cfg(feature = "imxrt1062")]
pub use imxrt106::imxrt1062::*;

#[cfg(feature = "imxrt1064")]
pub use imxrt106::imxrt1064::*;
