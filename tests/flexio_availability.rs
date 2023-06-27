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
#![allow(unused_imports, dead_code)]

#[test]
fn namespace_exists() {
    use imxrt_ral::flexio::{number, Instance, RegisterBlock};

    #[cfg(any(feature = "imxrt1011", feature = "imxrt1015", feature = "imxrt1021",))]
    use imxrt_ral::flexio::FLEXIO as INST;

    #[cfg(any(
        feature = "imxrt1051",
        feature = "imxrt1052",
        feature = "imxrt1061",
        feature = "imxrt1062",
        feature = "imxrt1064",
    ))]
    use imxrt_ral::flexio::FLEXIO1 as INST;

    type TypeAliasWorks = INST;
    const PTR_WORKS: *const RegisterBlock = INST;

    // These chips have custom, backwards-compatible items for FlexIO1.
    #[cfg(any(feature = "imxrt1011", feature = "imxrt1015", feature = "imxrt1021",))]
    {
        use imxrt_ral::flexio1::{number, Instance, RegisterBlock, FLEXIO1};
        type TypeAliasWorks = FLEXIO1;
        const PTR_WORKS: *const RegisterBlock = FLEXIO1;
    }
}
