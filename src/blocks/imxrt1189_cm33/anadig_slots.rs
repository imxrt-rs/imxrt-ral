#[doc = "IPS Domain"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x4c00],
    #[doc = "Slot Control Register"]
    pub SLOT_CTRL: [crate::RWRegister<u32>; 35usize],
}
#[doc = "Slot Control Register"]
pub mod SLOT_CTRL {
    #[doc = "Domain ID of the slot to be locked"]
    pub mod LOCKED_DOMAIN_ID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Lock domain ID of this slot"]
    pub mod DOMAIN_LOCK {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Do not lock the domain ID"]
            pub const UNLOCK: u32 = 0;
            #[doc = "Lock the domain ID"]
            pub const LOCK: u32 = 0x01;
        }
    }
    #[doc = "Allow non-secure write access to this domain control register or domain register"]
    pub mod ALLOW_NONSECURE {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Do not allow non-secure write access"]
            pub const PREVENT: u32 = 0;
            #[doc = "Allow non-secure write access"]
            pub const ALLOW: u32 = 0x01;
        }
    }
    #[doc = "Allow user write access to this domain control register or domain register"]
    pub mod ALLOW_USER {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Do not allow user write access"]
            pub const PREVENT: u32 = 0;
            #[doc = "Allow user write access"]
            pub const ALLOW: u32 = 0x01;
        }
    }
    #[doc = "Lock control of this slot"]
    pub mod LOCK_CONTROL {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Do not lock the control register of this slot"]
            pub const UNLOCK: u32 = 0;
            #[doc = "Lock the control register of this slot"]
            pub const LOCK: u32 = 0x01;
        }
    }
}
