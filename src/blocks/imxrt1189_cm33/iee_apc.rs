#[doc = "IEE_APC"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "End address of IEE region (n)"]
    pub REGION0_TOP_ADDR: crate::RWRegister<u32>,
    #[doc = "Start address of IEE region (n)"]
    pub REGION0_BOT_ADDR: crate::RWRegister<u32>,
    #[doc = "Region enable for region (n)"]
    pub REGION0_ENA: crate::RWRegister<u32>,
    #[doc = "Access control for IEE APC registers of region (n)"]
    pub REGION0_ACC_CTL: crate::RWRegister<u32>,
    #[doc = "End address of IEE region (n)"]
    pub REGION1_TOP_ADDR: crate::RWRegister<u32>,
    #[doc = "Start address of IEE region (n)"]
    pub REGION1_BOT_ADDR: crate::RWRegister<u32>,
    #[doc = "Region enable for region (n)"]
    pub REGION1_ENA: crate::RWRegister<u32>,
    #[doc = "Access control for IEE APC registers of region (n)"]
    pub REGION1_ACC_CTL: crate::RWRegister<u32>,
    #[doc = "End address of IEE region (n)"]
    pub REGION2_TOP_ADDR: crate::RWRegister<u32>,
    #[doc = "Start address of IEE region (n)"]
    pub REGION2_BOT_ADDR: crate::RWRegister<u32>,
    #[doc = "Region enable for region (n)"]
    pub REGION2_ENA: crate::RWRegister<u32>,
    #[doc = "Access control for IEE APC registers of region (n)"]
    pub REGION2_ACC_CTL: crate::RWRegister<u32>,
    #[doc = "End address of IEE region (n)"]
    pub REGION3_TOP_ADDR: crate::RWRegister<u32>,
    #[doc = "Start address of IEE region (n)"]
    pub REGION3_BOT_ADDR: crate::RWRegister<u32>,
    #[doc = "Region enable for region (n)"]
    pub REGION3_ENA: crate::RWRegister<u32>,
    #[doc = "Access control for IEE APC registers of region (n)"]
    pub REGION3_ACC_CTL: crate::RWRegister<u32>,
    #[doc = "End address of IEE region (n)"]
    pub REGION4_TOP_ADDR: crate::RWRegister<u32>,
    #[doc = "Start address of IEE region (n)"]
    pub REGION4_BOT_ADDR: crate::RWRegister<u32>,
    #[doc = "Region enable for region (n)"]
    pub REGION4_ENA: crate::RWRegister<u32>,
    #[doc = "Access control for IEE APC registers of region (n)"]
    pub REGION4_ACC_CTL: crate::RWRegister<u32>,
    #[doc = "End address of IEE region (n)"]
    pub REGION5_TOP_ADDR: crate::RWRegister<u32>,
    #[doc = "Start address of IEE region (n)"]
    pub REGION5_BOT_ADDR: crate::RWRegister<u32>,
    #[doc = "Region enable for region (n)"]
    pub REGION5_ENA: crate::RWRegister<u32>,
    #[doc = "Access control for IEE APC registers of region (n)"]
    pub REGION5_ACC_CTL: crate::RWRegister<u32>,
    #[doc = "End address of IEE region (n)"]
    pub REGION6_TOP_ADDR: crate::RWRegister<u32>,
    #[doc = "Start address of IEE region (n)"]
    pub REGION6_BOT_ADDR: crate::RWRegister<u32>,
    #[doc = "Region enable for region (n)"]
    pub REGION6_ENA: crate::RWRegister<u32>,
    #[doc = "Access control for IEE APC registers of region (n)"]
    pub REGION6_ACC_CTL: crate::RWRegister<u32>,
    #[doc = "End address of IEE region (n)"]
    pub REGION7_TOP_ADDR: crate::RWRegister<u32>,
    #[doc = "Start address of IEE region (n)"]
    pub REGION7_BOT_ADDR: crate::RWRegister<u32>,
    #[doc = "Region enable for region (n)"]
    pub REGION7_ENA: crate::RWRegister<u32>,
    #[doc = "Access control for IEE APC registers of region (n)"]
    pub REGION7_ACC_CTL: crate::RWRegister<u32>,
}
#[doc = "End address of IEE region (n)"]
pub mod REGION0_TOP_ADDR {
    #[doc = "End address\\[31:6\\] of IEE region. The lower 6 address bits of IEE region is always 0."]
    pub mod TOP_ADDR {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x03ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Start address of IEE region (n)"]
pub mod REGION0_BOT_ADDR {
    #[doc = "Start address\\[31:6\\] of IEE region. The lower 6 address bits of IEE region is always 0."]
    pub mod BOT_ADDR {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x03ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Region enable for region (n)"]
pub mod REGION0_ENA {
    #[doc = "Enable this region"]
    pub mod ENCRYPT_ENABLE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "This region is not enabled for IEE routing even hit"]
            pub const DISABLE: u32 = 0;
            #[doc = "This region is enabled for IEE routing once hit"]
            pub const ENABLE: u32 = 0x01;
        }
    }
}
#[doc = "Access control for IEE APC registers of region (n)"]
pub mod REGION0_ACC_CTL {
    #[doc = "Allowed domain ID"]
    pub mod ALLOW_DID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Lock bit for the lower half word"]
    pub mod LOCK_L {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Lower half word is not locked"]
            pub const UNLOCK: u32 = 0;
            #[doc = "Lower half word is locked"]
            pub const LOCK: u32 = 0x01;
        }
    }
    #[doc = "Allow nonsecure mode access"]
    pub mod ALLOW_NS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Only secure access to this region's registers is allowed"]
            pub const DISABLE: u32 = 0;
            #[doc = "Secure and nonsecure access to this region's registers is allowed"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Allow user mode access"]
    pub mod ALLOW_USER {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Only privilege access to this region's registers is allowed"]
            pub const DISABLE: u32 = 0;
            #[doc = "User and privilege access to this region's registers is allowed"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Lock bit for the higher half word"]
    pub mod LOCK_H {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Higher half word is not locked"]
            pub const UNLOCK: u32 = 0;
            #[doc = "Higher half word is locked"]
            pub const LOCK: u32 = 0x01;
        }
    }
}
#[doc = "End address of IEE region (n)"]
pub mod REGION1_TOP_ADDR {
    #[doc = "End address\\[31:6\\] of IEE region. The lower 6 address bits of IEE region is always 0."]
    pub mod TOP_ADDR {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x03ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Start address of IEE region (n)"]
pub mod REGION1_BOT_ADDR {
    #[doc = "Start address\\[31:6\\] of IEE region. The lower 6 address bits of IEE region is always 0."]
    pub mod BOT_ADDR {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x03ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Region enable for region (n)"]
pub mod REGION1_ENA {
    #[doc = "Enable this region"]
    pub mod ENCRYPT_ENABLE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "This region is not enabled for IEE routing even hit"]
            pub const DISABLE: u32 = 0;
            #[doc = "This region is enabled for IEE routing once hit"]
            pub const ENABLE: u32 = 0x01;
        }
    }
}
#[doc = "Access control for IEE APC registers of region (n)"]
pub mod REGION1_ACC_CTL {
    #[doc = "Allowed domain ID"]
    pub mod ALLOW_DID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Lock bit for the lower half word"]
    pub mod LOCK_L {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Lower half word is not locked"]
            pub const UNLOCK: u32 = 0;
            #[doc = "Lower half word is locked"]
            pub const LOCK: u32 = 0x01;
        }
    }
    #[doc = "Allow nonsecure mode access"]
    pub mod ALLOW_NS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Only secure access to this region's registers is allowed"]
            pub const DISABLE: u32 = 0;
            #[doc = "Secure and nonsecure access to this region's registers is allowed"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Allow user mode access"]
    pub mod ALLOW_USER {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Only privilege access to this region's registers is allowed"]
            pub const DISABLE: u32 = 0;
            #[doc = "User and privilege access to this region's registers is allowed"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Lock bit for the higher half word"]
    pub mod LOCK_H {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Higher half word is not locked"]
            pub const UNLOCK: u32 = 0;
            #[doc = "Higher half word is locked"]
            pub const LOCK: u32 = 0x01;
        }
    }
}
#[doc = "End address of IEE region (n)"]
pub mod REGION2_TOP_ADDR {
    #[doc = "End address\\[31:6\\] of IEE region. The lower 6 address bits of IEE region is always 0."]
    pub mod TOP_ADDR {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x03ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Start address of IEE region (n)"]
pub mod REGION2_BOT_ADDR {
    #[doc = "Start address\\[31:6\\] of IEE region. The lower 6 address bits of IEE region is always 0."]
    pub mod BOT_ADDR {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x03ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Region enable for region (n)"]
pub mod REGION2_ENA {
    #[doc = "Enable this region"]
    pub mod ENCRYPT_ENABLE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "This region is not enabled for IEE routing even hit"]
            pub const DISABLE: u32 = 0;
            #[doc = "This region is enabled for IEE routing once hit"]
            pub const ENABLE: u32 = 0x01;
        }
    }
}
#[doc = "Access control for IEE APC registers of region (n)"]
pub mod REGION2_ACC_CTL {
    #[doc = "Allowed domain ID"]
    pub mod ALLOW_DID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Lock bit for the lower half word"]
    pub mod LOCK_L {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Lower half word is not locked"]
            pub const UNLOCK: u32 = 0;
            #[doc = "Lower half word is locked"]
            pub const LOCK: u32 = 0x01;
        }
    }
    #[doc = "Allow nonsecure mode access"]
    pub mod ALLOW_NS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Only secure access to this region's registers is allowed"]
            pub const DISABLE: u32 = 0;
            #[doc = "Secure and nonsecure access to this region's registers is allowed"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Allow user mode access"]
    pub mod ALLOW_USER {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Only privilege access to this region's registers is allowed"]
            pub const DISABLE: u32 = 0;
            #[doc = "User and privilege access to this region's registers is allowed"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Lock bit for the higher half word"]
    pub mod LOCK_H {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Higher half word is not locked"]
            pub const UNLOCK: u32 = 0;
            #[doc = "Higher half word is locked"]
            pub const LOCK: u32 = 0x01;
        }
    }
}
#[doc = "End address of IEE region (n)"]
pub mod REGION3_TOP_ADDR {
    #[doc = "End address\\[31:6\\] of IEE region. The lower 6 address bits of IEE region is always 0."]
    pub mod TOP_ADDR {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x03ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Start address of IEE region (n)"]
pub mod REGION3_BOT_ADDR {
    #[doc = "Start address\\[31:6\\] of IEE region. The lower 6 address bits of IEE region is always 0."]
    pub mod BOT_ADDR {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x03ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Region enable for region (n)"]
pub mod REGION3_ENA {
    #[doc = "Enable this region"]
    pub mod ENCRYPT_ENABLE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "This region is not enabled for IEE routing even hit"]
            pub const DISABLE: u32 = 0;
            #[doc = "This region is enabled for IEE routing once hit"]
            pub const ENABLE: u32 = 0x01;
        }
    }
}
#[doc = "Access control for IEE APC registers of region (n)"]
pub mod REGION3_ACC_CTL {
    #[doc = "Allowed domain ID"]
    pub mod ALLOW_DID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Lock bit for the lower half word"]
    pub mod LOCK_L {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Lower half word is not locked"]
            pub const UNLOCK: u32 = 0;
            #[doc = "Lower half word is locked"]
            pub const LOCK: u32 = 0x01;
        }
    }
    #[doc = "Allow nonsecure mode access"]
    pub mod ALLOW_NS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Only secure access to this region's registers is allowed"]
            pub const DISABLE: u32 = 0;
            #[doc = "Secure and nonsecure access to this region's registers is allowed"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Allow user mode access"]
    pub mod ALLOW_USER {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Only privilege access to this region's registers is allowed"]
            pub const DISABLE: u32 = 0;
            #[doc = "User and privilege access to this region's registers is allowed"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Lock bit for the higher half word"]
    pub mod LOCK_H {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Higher half word is not locked"]
            pub const UNLOCK: u32 = 0;
            #[doc = "Higher half word is locked"]
            pub const LOCK: u32 = 0x01;
        }
    }
}
#[doc = "End address of IEE region (n)"]
pub mod REGION4_TOP_ADDR {
    #[doc = "End address\\[31:6\\] of IEE region. The lower 6 address bits of IEE region is always 0."]
    pub mod TOP_ADDR {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x03ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Start address of IEE region (n)"]
pub mod REGION4_BOT_ADDR {
    #[doc = "Start address\\[31:6\\] of IEE region. The lower 6 address bits of IEE region is always 0."]
    pub mod BOT_ADDR {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x03ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Region enable for region (n)"]
pub mod REGION4_ENA {
    #[doc = "Enable this region"]
    pub mod ENCRYPT_ENABLE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "This region is not enabled for IEE routing even hit"]
            pub const DISABLE: u32 = 0;
            #[doc = "This region is enabled for IEE routing once hit"]
            pub const ENABLE: u32 = 0x01;
        }
    }
}
#[doc = "Access control for IEE APC registers of region (n)"]
pub mod REGION4_ACC_CTL {
    #[doc = "Allowed domain ID"]
    pub mod ALLOW_DID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Lock bit for the lower half word"]
    pub mod LOCK_L {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Lower half word is not locked"]
            pub const UNLOCK: u32 = 0;
            #[doc = "Lower half word is locked"]
            pub const LOCK: u32 = 0x01;
        }
    }
    #[doc = "Allow nonsecure mode access"]
    pub mod ALLOW_NS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Only secure access to this region's registers is allowed"]
            pub const DISABLE: u32 = 0;
            #[doc = "Secure and nonsecure access to this region's registers is allowed"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Allow user mode access"]
    pub mod ALLOW_USER {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Only privilege access to this region's registers is allowed"]
            pub const DISABLE: u32 = 0;
            #[doc = "User and privilege access to this region's registers is allowed"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Lock bit for the higher half word"]
    pub mod LOCK_H {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Higher half word is not locked"]
            pub const UNLOCK: u32 = 0;
            #[doc = "Higher half word is locked"]
            pub const LOCK: u32 = 0x01;
        }
    }
}
#[doc = "End address of IEE region (n)"]
pub mod REGION5_TOP_ADDR {
    #[doc = "End address\\[31:6\\] of IEE region. The lower 6 address bits of IEE region is always 0."]
    pub mod TOP_ADDR {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x03ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Start address of IEE region (n)"]
pub mod REGION5_BOT_ADDR {
    #[doc = "Start address\\[31:6\\] of IEE region. The lower 6 address bits of IEE region is always 0."]
    pub mod BOT_ADDR {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x03ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Region enable for region (n)"]
pub mod REGION5_ENA {
    #[doc = "Enable this region"]
    pub mod ENCRYPT_ENABLE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "This region is not enabled for IEE routing even hit"]
            pub const DISABLE: u32 = 0;
            #[doc = "This region is enabled for IEE routing once hit"]
            pub const ENABLE: u32 = 0x01;
        }
    }
}
#[doc = "Access control for IEE APC registers of region (n)"]
pub mod REGION5_ACC_CTL {
    #[doc = "Allowed domain ID"]
    pub mod ALLOW_DID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Lock bit for the lower half word"]
    pub mod LOCK_L {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Lower half word is not locked"]
            pub const UNLOCK: u32 = 0;
            #[doc = "Lower half word is locked"]
            pub const LOCK: u32 = 0x01;
        }
    }
    #[doc = "Allow nonsecure mode access"]
    pub mod ALLOW_NS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Only secure access to this region's registers is allowed"]
            pub const DISABLE: u32 = 0;
            #[doc = "Secure and nonsecure access to this region's registers is allowed"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Allow user mode access"]
    pub mod ALLOW_USER {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Only privilege access to this region's registers is allowed"]
            pub const DISABLE: u32 = 0;
            #[doc = "User and privilege access to this region's registers is allowed"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Lock bit for the higher half word"]
    pub mod LOCK_H {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Higher half word is not locked"]
            pub const UNLOCK: u32 = 0;
            #[doc = "Higher half word is locked"]
            pub const LOCK: u32 = 0x01;
        }
    }
}
#[doc = "End address of IEE region (n)"]
pub mod REGION6_TOP_ADDR {
    #[doc = "End address\\[31:6\\] of IEE region. The lower 6 address bits of IEE region is always 0."]
    pub mod TOP_ADDR {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x03ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Start address of IEE region (n)"]
pub mod REGION6_BOT_ADDR {
    #[doc = "Start address\\[31:6\\] of IEE region. The lower 6 address bits of IEE region is always 0."]
    pub mod BOT_ADDR {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x03ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Region enable for region (n)"]
pub mod REGION6_ENA {
    #[doc = "Enable this region"]
    pub mod ENCRYPT_ENABLE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "This region is not enabled for IEE routing even hit"]
            pub const DISABLE: u32 = 0;
            #[doc = "This region is enabled for IEE routing once hit"]
            pub const ENABLE: u32 = 0x01;
        }
    }
}
#[doc = "Access control for IEE APC registers of region (n)"]
pub mod REGION6_ACC_CTL {
    #[doc = "Allowed domain ID"]
    pub mod ALLOW_DID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Lock bit for the lower half word"]
    pub mod LOCK_L {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Lower half word is not locked"]
            pub const UNLOCK: u32 = 0;
            #[doc = "Lower half word is locked"]
            pub const LOCK: u32 = 0x01;
        }
    }
    #[doc = "Allow nonsecure mode access"]
    pub mod ALLOW_NS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Only secure access to this region's registers is allowed"]
            pub const DISABLE: u32 = 0;
            #[doc = "Secure and nonsecure access to this region's registers is allowed"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Allow user mode access"]
    pub mod ALLOW_USER {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Only privilege access to this region's registers is allowed"]
            pub const DISABLE: u32 = 0;
            #[doc = "User and privilege access to this region's registers is allowed"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Lock bit for the higher half word"]
    pub mod LOCK_H {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Higher half word is not locked"]
            pub const UNLOCK: u32 = 0;
            #[doc = "Higher half word is locked"]
            pub const LOCK: u32 = 0x01;
        }
    }
}
#[doc = "End address of IEE region (n)"]
pub mod REGION7_TOP_ADDR {
    #[doc = "End address\\[31:6\\] of IEE region. The lower 6 address bits of IEE region is always 0."]
    pub mod TOP_ADDR {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x03ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Start address of IEE region (n)"]
pub mod REGION7_BOT_ADDR {
    #[doc = "Start address\\[31:6\\] of IEE region. The lower 6 address bits of IEE region is always 0."]
    pub mod BOT_ADDR {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x03ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Region enable for region (n)"]
pub mod REGION7_ENA {
    #[doc = "Enable this region"]
    pub mod ENCRYPT_ENABLE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "This region is not enabled for IEE routing even hit"]
            pub const DISABLE: u32 = 0;
            #[doc = "This region is enabled for IEE routing once hit"]
            pub const ENABLE: u32 = 0x01;
        }
    }
}
#[doc = "Access control for IEE APC registers of region (n)"]
pub mod REGION7_ACC_CTL {
    #[doc = "Allowed domain ID"]
    pub mod ALLOW_DID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Lock bit for the lower half word"]
    pub mod LOCK_L {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Lower half word is not locked"]
            pub const UNLOCK: u32 = 0;
            #[doc = "Lower half word is locked"]
            pub const LOCK: u32 = 0x01;
        }
    }
    #[doc = "Allow nonsecure mode access"]
    pub mod ALLOW_NS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Only secure access to this region's registers is allowed"]
            pub const DISABLE: u32 = 0;
            #[doc = "Secure and nonsecure access to this region's registers is allowed"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Allow user mode access"]
    pub mod ALLOW_USER {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Only privilege access to this region's registers is allowed"]
            pub const DISABLE: u32 = 0;
            #[doc = "User and privilege access to this region's registers is allowed"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Lock bit for the higher half word"]
    pub mod LOCK_H {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Higher half word is not locked"]
            pub const UNLOCK: u32 = 0;
            #[doc = "Higher half word is locked"]
            pub const LOCK: u32 = 0x01;
        }
    }
}
