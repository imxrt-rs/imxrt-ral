#[doc = "FlexCAN wrapper"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x09e0],
    #[doc = "Glitch Filter Width Register"]
    pub GFWR: crate::RWRegister<u32>,
}
#[doc = "Glitch Filter Width Register"]
pub mod GFWR {
    #[doc = "Glitch Filter Width"]
    pub mod GFWR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
