//! Ensures that the core imxrt-ral features, like
//!
//! - the read, write, modify, and reset macros,
//! - the register types
//!
//! are always avaliable, no matter the build.

/// Ensures that the imxrt-ral core API is available without
/// features (and with features). Failure manifests at compile
/// time.
mod import_test {
    #![allow(unused)]
    use imxrt_ral::{modify_reg, read_reg, write_reg};
    use imxrt_ral::{RORegister, RWRegister, WORegister};
}

use imxrt_ral as ral;

struct DummyRegisterBlock {
    register: ral::RWRegister<u32>,
}

mod register {
    #![allow(non_upper_case_globals, non_snake_case)] // Macro conventions...

    pub mod field_foo {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x7 << offset;
        pub mod RW {}
        pub mod W {}
        pub mod R {}
    }
}

fn dummy_register_block() -> DummyRegisterBlock {
    use core::mem::MaybeUninit;
    let register_block = MaybeUninit::zeroed();
    // Safety: 0 is a safe bitpattern
    unsafe { register_block.assume_init() }
}

#[test]
fn register_read() {
    let register_block = dummy_register_block();
    register_block.register.write(0b111 << 10);
    assert_eq!(
        0x7,
        ral::read_reg!(self, &register_block, register, field_foo)
    );
}

#[test]
fn register_write() {
    let register_block = dummy_register_block();
    ral::write_reg!(self, &register_block, register, field_foo: 5);
    assert_eq!(5 << 10, register_block.register.read());
}

#[test]
fn register_modify() {
    let register_block = dummy_register_block();
    register_block.register.write(1 << 10);
    ral::modify_reg!(self, &register_block, register, field_foo: 6);
    assert_eq!(6 << 10, register_block.register.read());
}
