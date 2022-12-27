#[doc = "MX6RT_ANADIG_REGISTER"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0400],
    #[doc = "Tempsensor Register"]
    pub TEMPSENSOR: crate::RWRegister<u32>,
    _reserved1: [u8; 0x2c],
    #[doc = "TEMPSNS_OTP_TRIM_VALUE_REGISTER"]
    pub TEMPSNS_OTP_TRIM_VALUE: crate::RORegister<u32>,
}
#[doc = "Tempsensor Register"]
pub mod TEMPSENSOR {
    #[doc = "AI toggle"]
    pub mod TEMPSNS_AI_TOGGLE {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "AI Busy monitor"]
    pub mod TEMPSNS_AI_BUSY {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "TEMPSNS_OTP_TRIM_VALUE_REGISTER"]
pub mod TEMPSNS_OTP_TRIM_VALUE {
    #[doc = "Temperature Value at 25C"]
    pub mod TEMPSNS_TEMP_VAL {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
