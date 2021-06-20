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

#![cfg_attr(target_arch = "arm", no_std)]
#![allow(clippy::all)]

mod register;
pub mod usage;

pub use crate::register::{RORegister, UnsafeRORegister};
pub use crate::register::{RWRegister, UnsafeRWRegister};
pub use crate::register::{UnsafeWORegister, WORegister};
#[cfg(any(
    feature = "doc",
    feature = "imxrt1011",
    feature = "imxrt1015",
    feature = "imxrt1021",
    feature = "imxrt1051",
    feature = "imxrt1052",
    feature = "imxrt1061",
    feature = "imxrt1062",
    feature = "imxrt1064"
))]
pub mod consts {
    //! Type-level constants used throughout the API

    pub use typenum::{Unsigned, U1, U2, U3, U4, U5, U6, U7, U8, U9};
}
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
