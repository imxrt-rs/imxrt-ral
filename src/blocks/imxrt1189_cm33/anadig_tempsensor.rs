#[doc = "RT1180_ANADIG_REGISTER"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x4530],
    #[doc = "TEMPSNS_OTP_TRIM_VALUE_REGISTER"]
    pub TEMPSNS_OTP_TRIM_VALUE: crate::RORegister<u32>,
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
