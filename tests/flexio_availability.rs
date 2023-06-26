//! Ensure that the `imxrt_ral::flexio` module is present.
//!
//! Rationale: see https://github.com/imxrt-rs/imxrt-ral/issues/37#issuecomment-1593734998

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

#[test]
fn namespace_exists() {
    use imxrt_ral::flexio::Instance;

    // Do something with the namespace.
    // It's only to check whether the namespace exists.
    let _dummy = Instance::<0>::new;
}
