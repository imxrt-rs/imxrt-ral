#[doc = "MSGINTR"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "Message Signaled Interrupt Index Register 0"]
    pub MSIIR0: crate::WORegister<u32>,
    #[doc = "Message Signaled Interrupt Register 0"]
    pub MSIR0: crate::RORegister<u32>,
    #[doc = "Message Signaled Interrupt Index Register 1"]
    pub MSIIR1: crate::WORegister<u32>,
    #[doc = "Message Signaled Interrupt Register 1"]
    pub MSIR1: crate::RORegister<u32>,
    #[doc = "Message Signaled Interrupt Index Register 2"]
    pub MSIIR2: crate::WORegister<u32>,
    #[doc = "Message Signaled Interrupt Register 2"]
    pub MSIR2: crate::RORegister<u32>,
}
#[doc = "Message Signaled Interrupt Index Register 0"]
pub mod MSIIR0 {
    #[doc = "Interrupt Bit Select"]
    pub mod IBS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Signaled Interrupt Register 0"]
pub mod MSIR0 {
    #[doc = "Message sharer n has a pending interrupt."]
    pub mod SHN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Signaled Interrupt Index Register 1"]
pub mod MSIIR1 {
    #[doc = "Interrupt Bit Select"]
    pub mod IBS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Signaled Interrupt Register 1"]
pub mod MSIR1 {
    #[doc = "Message sharer n has a pending interrupt."]
    pub mod SHN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Signaled Interrupt Index Register 2"]
pub mod MSIIR2 {
    #[doc = "Interrupt Bit Select"]
    pub mod IBS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Signaled Interrupt Register 2"]
pub mod MSIR2 {
    #[doc = "Message sharer n has a pending interrupt."]
    pub mod SHN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
