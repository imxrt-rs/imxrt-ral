use imxrt_rt::{Family, RuntimeBuilder};

fn main() {
    RuntimeBuilder::from_flexspi(Family::Imxrt1060, 1984 * 1024)
        .build()
        .unwrap();
}
