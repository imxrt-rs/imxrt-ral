#[doc = "Secure RAM"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x3000],
    #[doc = "Control Register"]
    pub CTRL: crate::RWRegister<u32>,
}
#[doc = "Control Register"]
pub mod CTRL {
    #[doc = "RAM Read Enable (with lock)"]
    pub mod RAM_RD_EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable read access"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable read access"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "RAM Write Enable (with lock)"]
    pub mod RAM_WR_EN {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable write access"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable write access"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Power Enable (with lock)"]
    pub mod PWR_EN {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Tamper Block Enable (with lock)"]
    pub mod TAMPER_BLOCK_EN {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Allow R/W access to secure RAM when tamper is detected"]
            pub const ACCESS: u32 = 0;
            #[doc = "Block R/W access to secure RAM when tamper is detected"]
            pub const BLOCK: u32 = 0x01;
        }
    }
    #[doc = "Turn off power on tamper event (with lock)"]
    pub mod TAMPER_PWR_OFF_EN {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable the turn off function when tamper is detected"]
            pub const OFF: u32 = 0;
            #[doc = "Turn off power for all secure RAM banks when tamper is detected"]
            pub const ON: u32 = 0x01;
        }
    }
    #[doc = "Lock bits"]
    pub mod LOCK_BIT {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
