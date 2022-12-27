#[doc = "IEE_APC"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "End address of IEE region (n)"]
    pub REGION0_TOP_ADDR: crate::RWRegister<u32>,
    #[doc = "Start address of IEE region (n)"]
    pub REGION0_BOT_ADDR: crate::RWRegister<u32>,
    #[doc = "Region control of core domain 0 for region (n)"]
    pub REGION0_RDC_D0: crate::RWRegister<u32>,
    #[doc = "Region control of core domain 1 for region (n)"]
    pub REGION0_RDC_D1: crate::RWRegister<u32>,
    #[doc = "End address of IEE region (n)"]
    pub REGION1_TOP_ADDR: crate::RWRegister<u32>,
    #[doc = "Start address of IEE region (n)"]
    pub REGION1_BOT_ADDR: crate::RWRegister<u32>,
    #[doc = "Region control of core domain 0 for region (n)"]
    pub REGION1_RDC_D0: crate::RWRegister<u32>,
    #[doc = "Region control of core domain 1 for region (n)"]
    pub REGION1_RDC_D1: crate::RWRegister<u32>,
    #[doc = "End address of IEE region (n)"]
    pub REGION2_TOP_ADDR: crate::RWRegister<u32>,
    #[doc = "Start address of IEE region (n)"]
    pub REGION2_BOT_ADDR: crate::RWRegister<u32>,
    #[doc = "Region control of core domain 0 for region (n)"]
    pub REGION2_RDC_D0: crate::RWRegister<u32>,
    #[doc = "Region control of core domain 1 for region (n)"]
    pub REGION2_RDC_D1: crate::RWRegister<u32>,
    #[doc = "End address of IEE region (n)"]
    pub REGION3_TOP_ADDR: crate::RWRegister<u32>,
    #[doc = "Start address of IEE region (n)"]
    pub REGION3_BOT_ADDR: crate::RWRegister<u32>,
    #[doc = "Region control of core domain 0 for region (n)"]
    pub REGION3_RDC_D0: crate::RWRegister<u32>,
    #[doc = "Region control of core domain 1 for region (n)"]
    pub REGION3_RDC_D1: crate::RWRegister<u32>,
    #[doc = "End address of IEE region (n)"]
    pub REGION4_TOP_ADDR: crate::RWRegister<u32>,
    #[doc = "Start address of IEE region (n)"]
    pub REGION4_BOT_ADDR: crate::RWRegister<u32>,
    #[doc = "Region control of core domain 0 for region (n)"]
    pub REGION4_RDC_D0: crate::RWRegister<u32>,
    #[doc = "Region control of core domain 1 for region (n)"]
    pub REGION4_RDC_D1: crate::RWRegister<u32>,
    #[doc = "End address of IEE region (n)"]
    pub REGION5_TOP_ADDR: crate::RWRegister<u32>,
    #[doc = "Start address of IEE region (n)"]
    pub REGION5_BOT_ADDR: crate::RWRegister<u32>,
    #[doc = "Region control of core domain 0 for region (n)"]
    pub REGION5_RDC_D0: crate::RWRegister<u32>,
    #[doc = "Region control of core domain 1 for region (n)"]
    pub REGION5_RDC_D1: crate::RWRegister<u32>,
    #[doc = "End address of IEE region (n)"]
    pub REGION6_TOP_ADDR: crate::RWRegister<u32>,
    #[doc = "Start address of IEE region (n)"]
    pub REGION6_BOT_ADDR: crate::RWRegister<u32>,
    #[doc = "Region control of core domain 0 for region (n)"]
    pub REGION6_RDC_D0: crate::RWRegister<u32>,
    #[doc = "Region control of core domain 1 for region (n)"]
    pub REGION6_RDC_D1: crate::RWRegister<u32>,
    #[doc = "End address of IEE region (n)"]
    pub REGION7_TOP_ADDR: crate::RWRegister<u32>,
    #[doc = "Start address of IEE region (n)"]
    pub REGION7_BOT_ADDR: crate::RWRegister<u32>,
    #[doc = "Region control of core domain 0 for region (n)"]
    pub REGION7_RDC_D0: crate::RWRegister<u32>,
    #[doc = "Region control of core domain 1 for region (n)"]
    pub REGION7_RDC_D1: crate::RWRegister<u32>,
}
#[doc = "End address of IEE region (n)"]
pub mod REGION0_TOP_ADDR {
    #[doc = "End address of IEE region"]
    pub mod TOP_ADDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Start address of IEE region (n)"]
pub mod REGION0_BOT_ADDR {
    #[doc = "Start address of IEE region"]
    pub mod BOT_ADDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Region control of core domain 0 for region (n)"]
pub mod REGION0_RDC_D0 {
    #[doc = "Write disable of core domain 1"]
    pub mod RDC_D0_WRITE_DIS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Write to TOP_ADDR and BOT_ADDR of this region enabled"]
            pub const ENABLE: u32 = 0;
            #[doc = "Write to TOP_ADDR and BOT_ADDR of this region disabled"]
            pub const DISABLE: u32 = 0x01;
        }
    }
    #[doc = "Lock bit for bit 0"]
    pub mod RDC_D0_LOCK {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Bit 0 is unlocked"]
            pub const UNLOCK: u32 = 0;
            #[doc = "Bit 0 is locked"]
            pub const LOCK: u32 = 0x01;
        }
    }
}
#[doc = "Region control of core domain 1 for region (n)"]
pub mod REGION0_RDC_D1 {
    #[doc = "Write disable of core domain 1"]
    pub mod RDC_D1_WRITE_DIS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Write to TOP_ADDR and BOT_ADDR of this region enabled"]
            pub const ENABLE: u32 = 0;
            #[doc = "Write to TOP_ADDR and BOT_ADDR of this region disabled"]
            pub const DISABLE: u32 = 0x01;
        }
    }
    #[doc = "Lock bit for bit 0"]
    pub mod RDC_D1_LOCK {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Bit 0 is unlocked"]
            pub const UNLOCK: u32 = 0;
            #[doc = "Bit 0 is locked"]
            pub const LOCK: u32 = 0x01;
        }
    }
}
#[doc = "End address of IEE region (n)"]
pub mod REGION1_TOP_ADDR {
    #[doc = "End address of IEE region"]
    pub mod TOP_ADDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Start address of IEE region (n)"]
pub mod REGION1_BOT_ADDR {
    #[doc = "Start address of IEE region"]
    pub mod BOT_ADDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Region control of core domain 0 for region (n)"]
pub mod REGION1_RDC_D0 {
    #[doc = "Write disable of core domain 1"]
    pub mod RDC_D0_WRITE_DIS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Write to TOP_ADDR and BOT_ADDR of this region enabled"]
            pub const ENABLE: u32 = 0;
            #[doc = "Write to TOP_ADDR and BOT_ADDR of this region disabled"]
            pub const DISABLE: u32 = 0x01;
        }
    }
    #[doc = "Lock bit for bit 0"]
    pub mod RDC_D0_LOCK {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Bit 0 is unlocked"]
            pub const UNLOCK: u32 = 0;
            #[doc = "Bit 0 is locked"]
            pub const LOCK: u32 = 0x01;
        }
    }
}
#[doc = "Region control of core domain 1 for region (n)"]
pub mod REGION1_RDC_D1 {
    #[doc = "Write disable of core domain 1"]
    pub mod RDC_D1_WRITE_DIS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Write to TOP_ADDR and BOT_ADDR of this region enabled"]
            pub const ENABLE: u32 = 0;
            #[doc = "Write to TOP_ADDR and BOT_ADDR of this region disabled"]
            pub const DISABLE: u32 = 0x01;
        }
    }
    #[doc = "Lock bit for bit 0"]
    pub mod RDC_D1_LOCK {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Bit 0 is unlocked"]
            pub const UNLOCK: u32 = 0;
            #[doc = "Bit 0 is locked"]
            pub const LOCK: u32 = 0x01;
        }
    }
}
#[doc = "End address of IEE region (n)"]
pub mod REGION2_TOP_ADDR {
    #[doc = "End address of IEE region"]
    pub mod TOP_ADDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Start address of IEE region (n)"]
pub mod REGION2_BOT_ADDR {
    #[doc = "Start address of IEE region"]
    pub mod BOT_ADDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Region control of core domain 0 for region (n)"]
pub mod REGION2_RDC_D0 {
    #[doc = "Write disable of core domain 1"]
    pub mod RDC_D0_WRITE_DIS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Write to TOP_ADDR and BOT_ADDR of this region enabled"]
            pub const ENABLE: u32 = 0;
            #[doc = "Write to TOP_ADDR and BOT_ADDR of this region disabled"]
            pub const DISABLE: u32 = 0x01;
        }
    }
    #[doc = "Lock bit for bit 0"]
    pub mod RDC_D0_LOCK {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Bit 0 is unlocked"]
            pub const UNLOCK: u32 = 0;
            #[doc = "Bit 0 is locked"]
            pub const LOCK: u32 = 0x01;
        }
    }
}
#[doc = "Region control of core domain 1 for region (n)"]
pub mod REGION2_RDC_D1 {
    #[doc = "Write disable of core domain 1"]
    pub mod RDC_D1_WRITE_DIS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Write to TOP_ADDR and BOT_ADDR of this region enabled"]
            pub const ENABLE: u32 = 0;
            #[doc = "Write to TOP_ADDR and BOT_ADDR of this region disabled"]
            pub const DISABLE: u32 = 0x01;
        }
    }
    #[doc = "Lock bit for bit 0"]
    pub mod RDC_D1_LOCK {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Bit 0 is unlocked"]
            pub const UNLOCK: u32 = 0;
            #[doc = "Bit 0 is locked"]
            pub const LOCK: u32 = 0x01;
        }
    }
}
#[doc = "End address of IEE region (n)"]
pub mod REGION3_TOP_ADDR {
    #[doc = "End address of IEE region"]
    pub mod TOP_ADDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Start address of IEE region (n)"]
pub mod REGION3_BOT_ADDR {
    #[doc = "Start address of IEE region"]
    pub mod BOT_ADDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Region control of core domain 0 for region (n)"]
pub mod REGION3_RDC_D0 {
    #[doc = "Write disable of core domain 1"]
    pub mod RDC_D0_WRITE_DIS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Write to TOP_ADDR and BOT_ADDR of this region enabled"]
            pub const ENABLE: u32 = 0;
            #[doc = "Write to TOP_ADDR and BOT_ADDR of this region disabled"]
            pub const DISABLE: u32 = 0x01;
        }
    }
    #[doc = "Lock bit for bit 0"]
    pub mod RDC_D0_LOCK {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Bit 0 is unlocked"]
            pub const UNLOCK: u32 = 0;
            #[doc = "Bit 0 is locked"]
            pub const LOCK: u32 = 0x01;
        }
    }
}
#[doc = "Region control of core domain 1 for region (n)"]
pub mod REGION3_RDC_D1 {
    #[doc = "Write disable of core domain 1"]
    pub mod RDC_D1_WRITE_DIS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Write to TOP_ADDR and BOT_ADDR of this region enabled"]
            pub const ENABLE: u32 = 0;
            #[doc = "Write to TOP_ADDR and BOT_ADDR of this region disabled"]
            pub const DISABLE: u32 = 0x01;
        }
    }
    #[doc = "Lock bit for bit 0"]
    pub mod RDC_D1_LOCK {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Bit 0 is unlocked"]
            pub const UNLOCK: u32 = 0;
            #[doc = "Bit 0 is locked"]
            pub const LOCK: u32 = 0x01;
        }
    }
}
#[doc = "End address of IEE region (n)"]
pub mod REGION4_TOP_ADDR {
    #[doc = "End address of IEE region"]
    pub mod TOP_ADDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Start address of IEE region (n)"]
pub mod REGION4_BOT_ADDR {
    #[doc = "Start address of IEE region"]
    pub mod BOT_ADDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Region control of core domain 0 for region (n)"]
pub mod REGION4_RDC_D0 {
    #[doc = "Write disable of core domain 1"]
    pub mod RDC_D0_WRITE_DIS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Write to TOP_ADDR and BOT_ADDR of this region enabled"]
            pub const ENABLE: u32 = 0;
            #[doc = "Write to TOP_ADDR and BOT_ADDR of this region disabled"]
            pub const DISABLE: u32 = 0x01;
        }
    }
    #[doc = "Lock bit for bit 0"]
    pub mod RDC_D0_LOCK {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Bit 0 is unlocked"]
            pub const UNLOCK: u32 = 0;
            #[doc = "Bit 0 is locked"]
            pub const LOCK: u32 = 0x01;
        }
    }
}
#[doc = "Region control of core domain 1 for region (n)"]
pub mod REGION4_RDC_D1 {
    #[doc = "Write disable of core domain 1"]
    pub mod RDC_D1_WRITE_DIS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Write to TOP_ADDR and BOT_ADDR of this region enabled"]
            pub const ENABLE: u32 = 0;
            #[doc = "Write to TOP_ADDR and BOT_ADDR of this region disabled"]
            pub const DISABLE: u32 = 0x01;
        }
    }
    #[doc = "Lock bit for bit 0"]
    pub mod RDC_D1_LOCK {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Bit 0 is unlocked"]
            pub const UNLOCK: u32 = 0;
            #[doc = "Bit 0 is locked"]
            pub const LOCK: u32 = 0x01;
        }
    }
}
#[doc = "End address of IEE region (n)"]
pub mod REGION5_TOP_ADDR {
    #[doc = "End address of IEE region"]
    pub mod TOP_ADDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Start address of IEE region (n)"]
pub mod REGION5_BOT_ADDR {
    #[doc = "Start address of IEE region"]
    pub mod BOT_ADDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Region control of core domain 0 for region (n)"]
pub mod REGION5_RDC_D0 {
    #[doc = "Write disable of core domain 1"]
    pub mod RDC_D0_WRITE_DIS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Write to TOP_ADDR and BOT_ADDR of this region enabled"]
            pub const ENABLE: u32 = 0;
            #[doc = "Write to TOP_ADDR and BOT_ADDR of this region disabled"]
            pub const DISABLE: u32 = 0x01;
        }
    }
    #[doc = "Lock bit for bit 0"]
    pub mod RDC_D0_LOCK {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Bit 0 is unlocked"]
            pub const UNLOCK: u32 = 0;
            #[doc = "Bit 0 is locked"]
            pub const LOCK: u32 = 0x01;
        }
    }
}
#[doc = "Region control of core domain 1 for region (n)"]
pub mod REGION5_RDC_D1 {
    #[doc = "Write disable of core domain 1"]
    pub mod RDC_D1_WRITE_DIS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Write to TOP_ADDR and BOT_ADDR of this region enabled"]
            pub const ENABLE: u32 = 0;
            #[doc = "Write to TOP_ADDR and BOT_ADDR of this region disabled"]
            pub const DISABLE: u32 = 0x01;
        }
    }
    #[doc = "Lock bit for bit 0"]
    pub mod RDC_D1_LOCK {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Bit 0 is unlocked"]
            pub const UNLOCK: u32 = 0;
            #[doc = "Bit 0 is locked"]
            pub const LOCK: u32 = 0x01;
        }
    }
}
#[doc = "End address of IEE region (n)"]
pub mod REGION6_TOP_ADDR {
    #[doc = "End address of IEE region"]
    pub mod TOP_ADDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Start address of IEE region (n)"]
pub mod REGION6_BOT_ADDR {
    #[doc = "Start address of IEE region"]
    pub mod BOT_ADDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Region control of core domain 0 for region (n)"]
pub mod REGION6_RDC_D0 {
    #[doc = "Write disable of core domain 1"]
    pub mod RDC_D0_WRITE_DIS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Write to TOP_ADDR and BOT_ADDR of this region enabled"]
            pub const ENABLE: u32 = 0;
            #[doc = "Write to TOP_ADDR and BOT_ADDR of this region disabled"]
            pub const DISABLE: u32 = 0x01;
        }
    }
    #[doc = "Lock bit for bit 0"]
    pub mod RDC_D0_LOCK {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Bit 0 is unlocked"]
            pub const UNLOCK: u32 = 0;
            #[doc = "Bit 0 is locked"]
            pub const LOCK: u32 = 0x01;
        }
    }
}
#[doc = "Region control of core domain 1 for region (n)"]
pub mod REGION6_RDC_D1 {
    #[doc = "Write disable of core domain 1"]
    pub mod RDC_D1_WRITE_DIS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Write to TOP_ADDR and BOT_ADDR of this region enabled"]
            pub const ENABLE: u32 = 0;
            #[doc = "Write to TOP_ADDR and BOT_ADDR of this region disabled"]
            pub const DISABLE: u32 = 0x01;
        }
    }
    #[doc = "Lock bit for bit 0"]
    pub mod RDC_D1_LOCK {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Bit 0 is unlocked"]
            pub const UNLOCK: u32 = 0;
            #[doc = "Bit 0 is locked"]
            pub const LOCK: u32 = 0x01;
        }
    }
}
#[doc = "End address of IEE region (n)"]
pub mod REGION7_TOP_ADDR {
    #[doc = "End address of IEE region"]
    pub mod TOP_ADDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Start address of IEE region (n)"]
pub mod REGION7_BOT_ADDR {
    #[doc = "Start address of IEE region"]
    pub mod BOT_ADDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Region control of core domain 0 for region (n)"]
pub mod REGION7_RDC_D0 {
    #[doc = "Write disable of core domain 1"]
    pub mod RDC_D0_WRITE_DIS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Write to TOP_ADDR and BOT_ADDR of this region enabled"]
            pub const ENABLE: u32 = 0;
            #[doc = "Write to TOP_ADDR and BOT_ADDR of this region disabled"]
            pub const DISABLE: u32 = 0x01;
        }
    }
    #[doc = "Lock bit for bit 0"]
    pub mod RDC_D0_LOCK {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Bit 0 is unlocked"]
            pub const UNLOCK: u32 = 0;
            #[doc = "Bit 0 is locked"]
            pub const LOCK: u32 = 0x01;
        }
    }
}
#[doc = "Region control of core domain 1 for region (n)"]
pub mod REGION7_RDC_D1 {
    #[doc = "Write disable of core domain 1"]
    pub mod RDC_D1_WRITE_DIS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Write to TOP_ADDR and BOT_ADDR of this region enabled"]
            pub const ENABLE: u32 = 0;
            #[doc = "Write to TOP_ADDR and BOT_ADDR of this region disabled"]
            pub const DISABLE: u32 = 0x01;
        }
    }
    #[doc = "Lock bit for bit 0"]
    pub mod RDC_D1_LOCK {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Bit 0 is unlocked"]
            pub const UNLOCK: u32 = 0;
            #[doc = "Bit 0 is locked"]
            pub const LOCK: u32 = 0x01;
        }
    }
}
