#[doc = "SYS_CTR_READ"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x08],
    #[doc = "Counter Count Value Low"]
    pub CNTCV0: crate::RORegister<u32>,
    #[doc = "Counter Count Value High"]
    pub CNTCV1: crate::RORegister<u32>,
    _reserved1: [u8; 0x0fc0],
    #[doc = "Counter ID"]
    pub CNTID0: crate::RORegister<u32>,
}
#[doc = "Counter Count Value Low"]
pub mod CNTCV0 {
    #[doc = "Counter Count Value Bits \\[31:0\\]"]
    pub mod CNTCV0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Counter Count Value High"]
pub mod CNTCV1 {
    #[doc = "Counter Count Value Bits \\[55:32\\]"]
    pub mod CNTCV1 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Counter ID"]
pub mod CNTID0 {
    #[doc = "Counter Identification"]
    pub mod CNTID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
