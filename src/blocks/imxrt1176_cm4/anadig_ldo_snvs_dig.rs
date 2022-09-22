#[doc = "MX6RT_ANADIG_REGISTER"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0540],
    #[doc = "PMU_LDO_SNVS_DIG_REGISTER"]
    pub PMU_LDO_SNVS_DIG: crate::RWRegister<u32>,
}
#[doc = "PMU_LDO_SNVS_DIG_REGISTER"]
pub mod PMU_LDO_SNVS_DIG {
    #[doc = "REG_LP_EN"]
    pub mod REG_LP_EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "test_override"]
    pub mod TEST_OVERRIDE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "REG_EN"]
    pub mod REG_EN {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
