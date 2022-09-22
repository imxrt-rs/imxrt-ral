use std::env;
use std::fs;
use std::path::PathBuf;
fn main() {
    if env::var_os("CARGO_FEATURE_RT").is_some() {
        let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
        println!("cargo:rustc-link-search={}", out.display());
        let device_file = if env::var_os("CARGO_FEATURE_IMXRT1011").is_some() {
            "src/imxrt1011.x"
        } else if env::var_os("CARGO_FEATURE_IMXRT1015").is_some() {
            "src/imxrt1015.x"
        } else if env::var_os("CARGO_FEATURE_IMXRT1021").is_some() {
            "src/imxrt1021.x"
        } else if env::var_os("CARGO_FEATURE_IMXRT1051").is_some() {
            "src/imxrt1051.x"
        } else if env::var_os("CARGO_FEATURE_IMXRT1052").is_some() {
            "src/imxrt1052.x"
        } else if env::var_os("CARGO_FEATURE_IMXRT1061").is_some() {
            "src/imxrt1061.x"
        } else if env::var_os("CARGO_FEATURE_IMXRT1062").is_some() {
            "src/imxrt1062.x"
        } else if env::var_os("CARGO_FEATURE_IMXRT1064").is_some() {
            "src/imxrt1064.x"
        } else {
            panic!("No device features selected");
        };
        fs::copy(device_file, out.join("device.x")).unwrap();
        println!("cargo:rerun-if-changed={}", device_file);
    }
    println!("cargo:rerun-if-changed=build.rs");
}
