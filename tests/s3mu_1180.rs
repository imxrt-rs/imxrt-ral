#![cfg(any(feature = "imxrt1189_cm33", feature = "imxrt1189_cm7",))]

use core::mem::offset_of;
use imxrt_ral as ral;

#[test]
fn check_transmit_receive_register_offsets() {
    use ral::mu_apps_s3mua::RegisterBlock;
    assert_eq!(offset_of!(RegisterBlock, TR), 0x200);
    assert_eq!(offset_of!(RegisterBlock, RR), 0x280);
}
