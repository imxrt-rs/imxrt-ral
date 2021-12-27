//! An imxrt-ral example that blinks the Teensy 4's LED
//!
//! This example is equivalent to 'blink.rs', but it demonstrates
//! the "nosync" imxrt-ral feature. There's much more unsafe in this
//! example; note that `main` is unsafe.

#![no_std]
#![no_main]

use imxrt_ral as ral;
use teensy4_fcb as _;
use teensy4_panic as _;

const LED_OFFSET: u32 = 3;
const LED: u32 = 1 << LED_OFFSET;

/// Microseconds, given the clock selection and configuration
/// for the timer.
const PIT_PERIOD_US: u32 = 1_000_000;

#[cortex_m_rt::entry]
unsafe fn main() -> ! {
    // Set the GPIO pad to a GPIO function (ALT 5)
    ral::write_reg!(
        ral::iomuxc,
        ral::iomuxc::IOMUXC,
        SW_MUX_CTL_PAD_GPIO_B0_03,
        5
    );
    // Increase drive strength, but leave other fields at their current value...
    ral::modify_reg!(
        ral::iomuxc,
        ral::iomuxc::IOMUXC,
        SW_PAD_CTL_PAD_GPIO_B0_03,
        DSE: DSE_7_R0_7
    );

    // Set GPIO2[3] to an output
    ral::modify_reg!(ral::gpio, ral::gpio::GPIO2, GDIR, |gdir| gdir | LED);

    // Disable the PIT clock gate while we change the clock...
    ral::modify_reg!(ral::ccm, ral::ccm::CCM, CCGR1, CG6: 0b00);
    // Set the periodic clock divider, selection.
    // 24MHz crystal oscillator, divided by 24 == 1MHz PIT clock
    ral::modify_reg!(
        ral::ccm,
        ral::ccm::CCM,
        CSCMR1,
        PERCLK_PODF: DIVIDE_24,
        PERCLK_CLK_SEL: PERCLK_CLK_SEL_1 // Oscillator clock
    );
    // Re-enable PIT clock
    ral::modify_reg!(ral::ccm, ral::ccm::CCM, CCGR1, CG6: 0b11);

    // Disable the PIT, just in case it was used by the boot ROM
    ral::write_reg!(ral::pit, ral::pit::PIT, MCR, MDIS: MDIS_1);
    // Reset channel 0 control; we'll use channel 0 for our timer
    ral::write_reg!(ral::pit, ral::pit::PIT, TCTRL0, 0);
    // Set the counter value
    ral::write_reg!(ral::pit, ral::pit::PIT, LDVAL0, PIT_PERIOD_US);
    // Enable the PIT timer
    ral::modify_reg!(ral::pit, ral::pit::PIT, MCR, MDIS: MDIS_0);

    let mut on = false;
    loop {
        on = !on;
        if on {
            ral::write_reg!(ral::gpio, ral::gpio::GPIO2, DR_SET, LED);
        } else {
            ral::write_reg!(ral::gpio, ral::gpio::GPIO2, DR_CLEAR, LED);
        }

        // Start counting!
        ral::write_reg!(ral::pit, ral::pit::PIT, TCTRL0, TEN: 1);
        // Are we done?
        while ral::read_reg!(ral::pit, ral::pit::PIT, TFLG0, TIF == 0) {}
        // We're done; clear the flag
        ral::write_reg!(ral::pit, ral::pit::PIT, TFLG0, TIF: 1);
        // Turn off the timer
        ral::write_reg!(ral::pit, ral::pit::PIT, TCTRL0, TEN: 0);
    }
}
