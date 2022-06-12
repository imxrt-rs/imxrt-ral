//! Show that we compute reserved peripheral memory properly.

/// The 1011's PWM peripheral requires padding after a u16 register.
/// The next address is two byte aligned, but not four byte aligned,
/// so we should be careful about using a u32.
#[cfg(feature = "imxrt1011")]
#[test]
fn reserved_1011_pwm() {
    // See reference manual for addresses and offsets.
    const PWM_BASE_ADDRESS: u32 = 0x401C_C000;
    const SMCNT_OFFSETS: [u32; 4] = [0x000, 0x060, 0x0C0, 0x120];

    let pwm = imxrt_ral::pwm::PWM::take().unwrap();

    assert_eq!(
        core::ptr::addr_of!(pwm.SMCNT0) as u32,
        PWM_BASE_ADDRESS + SMCNT_OFFSETS[0]
    );
    assert_eq!(
        core::ptr::addr_of!(pwm.SMCNT1) as u32,
        PWM_BASE_ADDRESS + SMCNT_OFFSETS[1],
    );
    assert_eq!(
        core::ptr::addr_of!(pwm.SMCNT2) as u32,
        PWM_BASE_ADDRESS + SMCNT_OFFSETS[2],
    );
    assert_eq!(
        core::ptr::addr_of!(pwm.SMCNT3) as u32,
        PWM_BASE_ADDRESS + SMCNT_OFFSETS[3],
    );
}
