//! It's safe to run RTIC on the Teensy 4 for these examples.

#![no_std]

pub use imxrt_ral::{interrupt, Interrupt, NVIC_PRIO_BITS};

pub struct Peripherals(pub imxrt_ral::Instances);
impl Peripherals {
    #[inline]
    pub const unsafe fn steal() -> Self {
        Self(imxrt_ral::Instances::instances())
    }
}
