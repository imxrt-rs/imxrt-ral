#[doc = "no description available"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "OTP shadow register"]
    pub OTP_SHADOW_PARTA: [crate::RWRegister<u32>; 52usize],
    _reserved0: [u8; 0x0410],
    #[doc = "OTP shadow register"]
    pub OTP_SHADOW_PARTB: [crate::RWRegister<u32>; 200usize],
}
#[doc = "OTP shadow register"]
pub mod OTP_SHADOW_PARTA {
    #[doc = "OTP shadow register, fsb have read access of shadow 0-51 (offset should be 0*4-51*4)"]
    pub mod SHADOW {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "OTP shadow register"]
pub mod OTP_SHADOW_PARTB {
    #[doc = "OTP shadow register, fsb have read access of shadow 312-511 (offset should be 312*4-511*4)"]
    pub mod SHADOW {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
