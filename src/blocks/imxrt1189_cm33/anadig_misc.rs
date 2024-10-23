#[doc = "RT1180_ANADIG_REGISTER"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x4800],
    #[doc = "Chip Silicon Version Register"]
    pub MISC_DIFPROG: crate::RORegister<u32>,
}
#[doc = "Chip Silicon Version Register"]
pub mod MISC_DIFPROG {
    #[doc = "Chip ID"]
    pub mod CHIPID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
