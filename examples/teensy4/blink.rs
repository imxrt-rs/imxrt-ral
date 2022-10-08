//! An imxrt-ral example that blinks the Teensy 4's LED
//!
//! The example demonstrates
//!
//! - basic I/O multiplexing with the IOMUX controller
//! - GPIO pin configuration and I/O
//! - clock management with the CCM peripheral
//! - PIT timer configuration, spinloop

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
fn main() -> ! {
    let iomuxc = unsafe { ral::iomuxc::IOMUXC::instance() };
    // Set the GPIO pad to a GPIO function (ALT 5)
    ral::write_reg!(ral::iomuxc, iomuxc, SW_MUX_CTL_PAD_GPIO_B0_03, 5);
    // Increase drive strength, but leave other fields at their current value...
    ral::modify_reg!(
        ral::iomuxc,
        iomuxc,
        SW_PAD_CTL_PAD_GPIO_B0_03,
        DSE: DSE_7_R0_7
    );

    let gpio2 = unsafe { ral::gpio::GPIO2::instance() };
    // Set GPIO2[3] to an output
    ral::modify_reg!(ral::gpio, gpio2, GDIR, |gdir| gdir | LED);

    let ccm = unsafe { ral::ccm::CCM::instance() };
    // Disable the PIT clock gate while we change the clock...
    ral::modify_reg!(ral::ccm, ccm, CCGR1, CG6: 0b00);
    // Set the periodic clock divider, selection.
    // 24MHz crystal oscillator, divided by 24 == 1MHz PIT clock
    ral::modify_reg!(
        ral::ccm,
        ccm,
        CSCMR1,
        PERCLK_PODF: DIVIDE_24,
        PERCLK_CLK_SEL: PERCLK_CLK_SEL_1 // Oscillator clock
    );
    // Re-enable PIT clock
    ral::modify_reg!(ral::ccm, ccm, CCGR1, CG6: 0b11);

    let pit = unsafe { ral::pit::PIT::instance() };
    // Disable the PIT, just in case it was used by the boot ROM
    ral::write_reg!(ral::pit, pit, MCR, MDIS: MDIS_1);
    // Reset channel 0 control; we'll use channel 0 for our timer
    ral::write_reg!(ral::pit::timer, &pit.TIMER[0], TCTRL, 0);
    // Set the counter value
    ral::write_reg!(ral::pit::timer, &pit.TIMER[0], LDVAL, PIT_PERIOD_US);
    // Enable the PIT timer
    ral::modify_reg!(ral::pit, pit, MCR, MDIS: MDIS_0);

    let mut on = false;
    loop {
        on = !on;
        if on {
            ral::write_reg!(ral::gpio, gpio2, DR_SET, LED);
        } else {
            ral::write_reg!(ral::gpio, gpio2, DR_CLEAR, LED);
        }

        // Start counting!
        ral::write_reg!(ral::pit::timer, &pit.TIMER[0], TCTRL, TEN: 1);
        // Are we done?
        while ral::read_reg!(ral::pit::timer, &pit.TIMER[0], TFLG, TIF == 0) {}
        // We're done; clear the flag
        ral::write_reg!(ral::pit::timer, &pit.TIMER[0], TFLG, TIF: 1);
        // Turn off the timer
        ral::write_reg!(ral::pit::timer, &pit.TIMER[0], TCTRL, TEN: 0);
    }
}
