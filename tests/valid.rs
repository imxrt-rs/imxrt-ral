//! Ensure that Valid is always implemented, even in `nosync` builds.

#![cfg(any(
    feature = "imxrt1011",
    feature = "imxrt1015",
    feature = "imxrt1021",
    feature = "imxrt1051",
    feature = "imxrt1052",
    feature = "imxrt1061",
    feature = "imxrt1062",
    feature = "imxrt1064",
))]

fn is_valid<const LPUART_N: u8>() -> bool
where
    imxrt_ral::lpuart::Instance<LPUART_N>: imxrt_ral::lpuart::Valid,
{
    true
}

#[test]
fn test_valid() {
    assert!(is_valid::<1>());
    assert!(is_valid::<2>());
    assert!(is_valid::<3>());
    assert!(is_valid::<4>());
}
