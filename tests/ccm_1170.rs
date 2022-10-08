//! Spot checks for the 1170's CCM, one of the most complex and
//! important peripherals we're generating.

#![cfg(any(feature = "imxrt1176_cm7", feature = "imxrt1176_cm4"))]

use imxrt_ral::ccm::RegisterBlock;

fn ccm() -> RegisterBlock {
    // Safety: zero bitpattern is fine
    unsafe { core::mem::MaybeUninit::zeroed().assume_init() }
}

/// Offsets come from reference manual.
#[test]
fn spot_check_offsets() {
    let ccm = &ccm();
    assert_eq!(
        core::ptr::addr_of!(ccm.CLOCK_ROOT[42].CLOCK_ROOT_CONFIG) as u32,
        (ccm as *const _ as u32) + (0x2C + 42 * 0x80),
    );
    assert_eq!(
        core::ptr::addr_of!(ccm.CLOCK_ROOT[21].CLOCK_ROOT_SETPOINT[13]) as u32,
        (ccm as *const _ as u32) + 0xAF4,
    );
    assert_eq!(
        core::ptr::addr_of!(ccm.CLOCK_ROOT[71].CLOCK_ROOT_CONTROL_SET) as u32,
        (ccm as *const _ as u32) + 0x2384,
    );
    assert_eq!(
        core::ptr::addr_of!(ccm.CLOCK_GROUP1_AUTHEN_CLR) as u32,
        (ccm as *const _ as u32) + 0x40B8,
    );
    assert_eq!(
        core::ptr::addr_of!(ccm.GPR_SHARED[6].GPR_SHARED) as u32,
        (ccm as *const _ as u32) + (0x4800 + 6 * 0x20),
    );
    assert_eq!(
        core::ptr::addr_of!(ccm.GPR_PRIVATE3_TOG) as u32,
        (ccm as *const _ as u32) + (0x4C0C + 3 * 0x20),
    );
    assert_eq!(
        core::ptr::addr_of!(ccm.OSCPLL[23].OSCPLL_STATUS1) as u32,
        (ccm as *const _ as u32) + (0x5014 + 23 * 0x20),
    );
    assert_eq!(
        core::ptr::addr_of!(ccm.LPCG3_SETPOINT) as u32,
        (ccm as *const _ as u32) + 0x6068,
    );
}
