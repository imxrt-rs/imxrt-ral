#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! ADC_ETC
//!
//! Used by: imxrt1061, imxrt1062, imxrt1064

#[cfg(not(feature = "nosync"))]
use crate::imxrt106::peripherals::adc_etc::private;
#[cfg(not(feature = "nosync"))]
pub use crate::imxrt106::peripherals::adc_etc::{Instance, Valid};
pub use crate::imxrt106::peripherals::adc_etc::{RegisterBlock, ResetValues};
pub use crate::imxrt106::peripherals::adc_etc::{
    CTRL, DMA_CTRL, DONE0_1_IRQ, DONE2_ERR_IRQ, TRIG0_CHAIN_1_0, TRIG0_CHAIN_3_2, TRIG0_CHAIN_5_4,
    TRIG0_CHAIN_7_6, TRIG0_COUNTER, TRIG0_CTRL, TRIG0_RESULT_1_0, TRIG0_RESULT_3_2,
    TRIG0_RESULT_5_4, TRIG0_RESULT_7_6, TRIG1_CHAIN_1_0, TRIG1_CHAIN_3_2, TRIG1_CHAIN_5_4,
    TRIG1_CHAIN_7_6, TRIG1_COUNTER, TRIG1_CTRL, TRIG1_RESULT_1_0, TRIG1_RESULT_3_2,
    TRIG1_RESULT_5_4, TRIG1_RESULT_7_6, TRIG2_CHAIN_1_0, TRIG2_CHAIN_3_2, TRIG2_CHAIN_5_4,
    TRIG2_CHAIN_7_6, TRIG2_COUNTER, TRIG2_CTRL, TRIG2_RESULT_1_0, TRIG2_RESULT_3_2,
    TRIG2_RESULT_5_4, TRIG2_RESULT_7_6, TRIG3_CHAIN_1_0, TRIG3_CHAIN_3_2, TRIG3_CHAIN_5_4,
    TRIG3_CHAIN_7_6, TRIG3_COUNTER, TRIG3_CTRL, TRIG3_RESULT_1_0, TRIG3_RESULT_3_2,
    TRIG3_RESULT_5_4, TRIG3_RESULT_7_6, TRIG4_CHAIN_1_0, TRIG4_CHAIN_3_2, TRIG4_CHAIN_5_4,
    TRIG4_CHAIN_7_6, TRIG4_COUNTER, TRIG4_CTRL, TRIG4_RESULT_1_0, TRIG4_RESULT_3_2,
    TRIG4_RESULT_5_4, TRIG4_RESULT_7_6, TRIG5_CHAIN_1_0, TRIG5_CHAIN_3_2, TRIG5_CHAIN_5_4,
    TRIG5_CHAIN_7_6, TRIG5_COUNTER, TRIG5_CTRL, TRIG5_RESULT_1_0, TRIG5_RESULT_3_2,
    TRIG5_RESULT_5_4, TRIG5_RESULT_7_6, TRIG6_CHAIN_1_0, TRIG6_CHAIN_3_2, TRIG6_CHAIN_5_4,
    TRIG6_CHAIN_7_6, TRIG6_COUNTER, TRIG6_CTRL, TRIG6_RESULT_1_0, TRIG6_RESULT_3_2,
    TRIG6_RESULT_5_4, TRIG6_RESULT_7_6, TRIG7_CHAIN_1_0, TRIG7_CHAIN_3_2, TRIG7_CHAIN_5_4,
    TRIG7_CHAIN_7_6, TRIG7_COUNTER, TRIG7_CTRL, TRIG7_RESULT_1_0, TRIG7_RESULT_3_2,
    TRIG7_RESULT_5_4, TRIG7_RESULT_7_6,
};
#[cfg(not(feature = "nosync"))]
use core::sync::atomic::{AtomicBool, Ordering};

/// The ADC_ETC peripheral instance.
#[cfg(not(feature = "nosync"))]
pub type ADC_ETC = Instance<0>;

#[cfg(not(feature = "nosync"))]
impl private::Sealed for ADC_ETC {}
#[cfg(not(feature = "nosync"))]
impl Valid for ADC_ETC {}

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static ADC_ETC_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the ADC_ETC peripheral instance
#[cfg(not(feature = "nosync"))]
impl ADC_ETC {
    const INSTANCE: Self = Self {
        addr: 0x403b0000,
        #[cfg(not(feature = "doc"))]
        intrs: &[
            crate::interrupt::ADC_ETC_IRQ0,
            crate::interrupt::ADC_ETC_IRQ1,
            crate::interrupt::ADC_ETC_IRQ2,
            crate::interrupt::ADC_ETC_ERROR_IRQ,
        ],
        #[cfg(feature = "doc")]
        intrs: &[],
    };

    /// Reset values for each field in ADC_ETC
    pub const reset: ResetValues = ResetValues {
        CTRL: 0xC0000000,
        DONE0_1_IRQ: 0x00000000,
        DONE2_ERR_IRQ: 0x00000000,
        DMA_CTRL: 0x00000000,
        TRIG0_CTRL: 0x00000000,
        TRIG0_COUNTER: 0x00000000,
        TRIG0_CHAIN_1_0: 0x00000000,
        TRIG0_CHAIN_3_2: 0x00000000,
        TRIG0_CHAIN_5_4: 0x00000000,
        TRIG0_CHAIN_7_6: 0x00000000,
        TRIG0_RESULT_1_0: 0x00000000,
        TRIG0_RESULT_3_2: 0x00000000,
        TRIG0_RESULT_5_4: 0x00000000,
        TRIG0_RESULT_7_6: 0x00000000,
        TRIG1_CTRL: 0x00000000,
        TRIG1_COUNTER: 0x00000000,
        TRIG1_CHAIN_1_0: 0x00000000,
        TRIG1_CHAIN_3_2: 0x00000000,
        TRIG1_CHAIN_5_4: 0x00000000,
        TRIG1_CHAIN_7_6: 0x00000000,
        TRIG1_RESULT_1_0: 0x00000000,
        TRIG1_RESULT_3_2: 0x00000000,
        TRIG1_RESULT_5_4: 0x00000000,
        TRIG1_RESULT_7_6: 0x00000000,
        TRIG2_CTRL: 0x00000000,
        TRIG2_COUNTER: 0x00000000,
        TRIG2_CHAIN_1_0: 0x00000000,
        TRIG2_CHAIN_3_2: 0x00000000,
        TRIG2_CHAIN_5_4: 0x00000000,
        TRIG2_CHAIN_7_6: 0x00000000,
        TRIG2_RESULT_1_0: 0x00000000,
        TRIG2_RESULT_3_2: 0x00000000,
        TRIG2_RESULT_5_4: 0x00000000,
        TRIG2_RESULT_7_6: 0x00000000,
        TRIG3_CTRL: 0x00000000,
        TRIG3_COUNTER: 0x00000000,
        TRIG3_CHAIN_1_0: 0x00000000,
        TRIG3_CHAIN_3_2: 0x00000000,
        TRIG3_CHAIN_5_4: 0x00000000,
        TRIG3_CHAIN_7_6: 0x00000000,
        TRIG3_RESULT_1_0: 0x00000000,
        TRIG3_RESULT_3_2: 0x00000000,
        TRIG3_RESULT_5_4: 0x00000000,
        TRIG3_RESULT_7_6: 0x00000000,
        TRIG4_CTRL: 0x00000000,
        TRIG4_COUNTER: 0x00000000,
        TRIG4_CHAIN_1_0: 0x00000000,
        TRIG4_CHAIN_3_2: 0x00000000,
        TRIG4_CHAIN_5_4: 0x00000000,
        TRIG4_CHAIN_7_6: 0x00000000,
        TRIG4_RESULT_1_0: 0x00000000,
        TRIG4_RESULT_3_2: 0x00000000,
        TRIG4_RESULT_5_4: 0x00000000,
        TRIG4_RESULT_7_6: 0x00000000,
        TRIG5_CTRL: 0x00000000,
        TRIG5_COUNTER: 0x00000000,
        TRIG5_CHAIN_1_0: 0x00000000,
        TRIG5_CHAIN_3_2: 0x00000000,
        TRIG5_CHAIN_5_4: 0x00000000,
        TRIG5_CHAIN_7_6: 0x00000000,
        TRIG5_RESULT_1_0: 0x00000000,
        TRIG5_RESULT_3_2: 0x00000000,
        TRIG5_RESULT_5_4: 0x00000000,
        TRIG5_RESULT_7_6: 0x00000000,
        TRIG6_CTRL: 0x00000000,
        TRIG6_COUNTER: 0x00000000,
        TRIG6_CHAIN_1_0: 0x00000000,
        TRIG6_CHAIN_3_2: 0x00000000,
        TRIG6_CHAIN_5_4: 0x00000000,
        TRIG6_CHAIN_7_6: 0x00000000,
        TRIG6_RESULT_1_0: 0x00000000,
        TRIG6_RESULT_3_2: 0x00000000,
        TRIG6_RESULT_5_4: 0x00000000,
        TRIG6_RESULT_7_6: 0x00000000,
        TRIG7_CTRL: 0x00000000,
        TRIG7_COUNTER: 0x00000000,
        TRIG7_CHAIN_1_0: 0x00000000,
        TRIG7_CHAIN_3_2: 0x00000000,
        TRIG7_CHAIN_5_4: 0x00000000,
        TRIG7_CHAIN_7_6: 0x00000000,
        TRIG7_RESULT_1_0: 0x00000000,
        TRIG7_RESULT_3_2: 0x00000000,
        TRIG7_RESULT_5_4: 0x00000000,
        TRIG7_RESULT_7_6: 0x00000000,
    };

    /// Safe access to ADC_ETC
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
    #[inline]
    pub fn take() -> Option<Self> {
        let taken = ADC_ETC_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to ADC_ETC
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = ADC_ETC_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal ADC_ETC
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        ADC_ETC_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }

    /// The interrupts associated with ADC_ETC
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 4] = [
        crate::interrupt::ADC_ETC_IRQ0,
        crate::interrupt::ADC_ETC_IRQ1,
        crate::interrupt::ADC_ETC_IRQ2,
        crate::interrupt::ADC_ETC_ERROR_IRQ,
    ];

    /// The interrupts associated with ADC_ETC
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to ADC_ETC
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const ADC_ETC: *const RegisterBlock = 0x403b0000 as *const _;
