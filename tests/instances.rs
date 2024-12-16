#![cfg(any(
    feature = "imxrt1011",
    feature = "imxrt1015",
    feature = "imxrt1021",
    feature = "imxrt1051",
    feature = "imxrt1052",
    feature = "imxrt1061",
    feature = "imxrt1062",
    feature = "imxrt1064",
    feature = "imxrt1176_cm7",
    feature = "imxrt1176_cm4",
))]

#[test]
fn instances() {
    let _instances = unsafe { imxrt_ral::Instances::instances() };
}
