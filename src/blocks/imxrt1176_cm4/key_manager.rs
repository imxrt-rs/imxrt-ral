#[doc = "KEYMGR"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "CSR Master Key Control Register"]
    pub MASTER_KEY_CTRL: crate::RWRegister<u32>,
    _reserved0: [u8; 0x0c],
    #[doc = "CSR OTFAD-1 Key Control"]
    pub OTFAD1_KEY_CTRL: crate::RWRegister<u32>,
    _reserved1: [u8; 0x04],
    #[doc = "CSR OTFAD-2 Key Control"]
    pub OTFAD2_KEY_CTRL: crate::RWRegister<u32>,
    _reserved2: [u8; 0x04],
    #[doc = "CSR IEE Key Control"]
    pub IEE_KEY_CTRL: crate::RWRegister<u32>,
    _reserved3: [u8; 0x0c],
    #[doc = "CSR PUF Key Control"]
    pub PUF_KEY_CTRL: crate::RWRegister<u32>,
    _reserved4: [u8; 0x03cc],
    #[doc = "Slot 0 Control"]
    pub SLOT0_CTRL: crate::RWRegister<u32>,
    #[doc = "Slot1 Control"]
    pub SLOT1_CTRL: crate::RWRegister<u32>,
    #[doc = "Slot2 Control"]
    pub SLOT2_CTRL: crate::RWRegister<u32>,
    #[doc = "Slot3 Control"]
    pub SLOT3_CTRL: crate::RWRegister<u32>,
    #[doc = "Slot 4 Control"]
    pub SLOT4_CTRL: crate::RWRegister<u32>,
}
#[doc = "CSR Master Key Control Register"]
pub mod MASTER_KEY_CTRL {
    #[doc = "Key select for SNVS OTPMK. Default value comes from FUSE_MASTER_KEY_SEL."]
    pub mod SELECT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "select key from UDF"]
            pub const SELECT_FROM_UDF: u32 = 0;
            #[doc = "If LOCK = 1, select key from PUF, otherwise select key from fuse (bypass the fuse OTPMK to SNVS)"]
            pub const SELECT_FROM_PUF: u32 = 0x01;
        }
    }
    #[doc = "lock this register, prevent from writing. Default value comes from FUSE_MASTER_KEY_SEL_LOCK."]
    pub mod LOCK {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "not locked"]
            pub const UNLOCK: u32 = 0;
            #[doc = "locked"]
            pub const LOCK: u32 = 0x01;
        }
    }
}
#[doc = "CSR OTFAD-1 Key Control"]
pub mod OTFAD1_KEY_CTRL {
    #[doc = "key select for OTFAD-1. Default value comes from FUSE_OTFAD1_KEY_SEL."]
    pub mod SELECT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Select key from OCOTP USER_KEY5"]
            pub const SELECT_FROM_USER_KEY5: u32 = 0;
            #[doc = "If PUF_KEY_CTRL\\[LOCK\\] is 1, select key from PUF, otherwise select key from OCOTP USER_KEY5"]
            pub const SELECT_FROM_PUF: u32 = 0x01;
        }
    }
    #[doc = "lock this register, prevent from writing. Default value comes from FUSE_OTFAD1_KEY_SEL_LOCK."]
    pub mod LOCK {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "not locked"]
            pub const UNLOCK: u32 = 0;
            #[doc = "locked"]
            pub const LOCK: u32 = 0x01;
        }
    }
}
#[doc = "CSR OTFAD-2 Key Control"]
pub mod OTFAD2_KEY_CTRL {
    #[doc = "key select for OTFAD-2. Default value comes from FUSE_OTFAD1_KEY_SEL."]
    pub mod SELECT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "select key from OCOTP USER_KEY5"]
            pub const SELECT_FROM_USER_KEY5: u32 = 0;
            #[doc = "If PUF_KEY_CTRL\\[LOCK\\] is 1, select key from PUF, otherwise select key from OCOTP USER_KEY5"]
            pub const SELECT_FROM_PUF: u32 = 0x01;
        }
    }
    #[doc = "lock this register, prevent from writing. Default value comes from FUSE_OTFAD2_KEY_SEL_LOCK."]
    pub mod LOCK {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "not locked"]
            pub const UNLOCK: u32 = 0;
            #[doc = "locked"]
            pub const LOCK: u32 = 0x01;
        }
    }
}
#[doc = "CSR IEE Key Control"]
pub mod IEE_KEY_CTRL {
    #[doc = "Restart load key signal for IEE"]
    pub mod RELOAD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Do nothing"]
            pub const IDLE: u32 = 0;
            #[doc = "Restart IEE key load flow"]
            pub const RESTART: u32 = 0x01;
        }
    }
}
#[doc = "CSR PUF Key Control"]
pub mod PUF_KEY_CTRL {
    #[doc = "Lock signal for key select"]
    pub mod LOCK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Do not lock the key select"]
            pub const UNLOCK: u32 = 0;
            #[doc = "Lock the key select to select key from PUF, otherwise bypass key from OCOPT and do not lock. Once it has been set to 1, it cannot be reset manually. It will be set to 0 when the IEE key reload operation is done."]
            pub const LOCK: u32 = 0x01;
        }
    }
}
#[doc = "Slot 0 Control"]
pub mod SLOT0_CTRL {
    #[doc = "Whitelist"]
    pub mod WHITE_LIST {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Lock whitelist"]
    pub mod LOCK_LIST {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Whitelist is not locked"]
            pub const UNLOCK: u32 = 0;
            #[doc = "Whitelist is locked"]
            pub const LOCK: u32 = 0x01;
        }
    }
    #[doc = "Allow non-secure write access to this register and the slot it controls"]
    pub mod TZ_NS {
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
    #[doc = "Allow user write access to this register and the slot it controls"]
    pub mod TZ_USER {
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
#[doc = "Slot1 Control"]
pub mod SLOT1_CTRL {
    #[doc = "Whitelist"]
    pub mod WHITE_LIST {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Lock whitelist"]
    pub mod LOCK_LIST {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Whitelist is not locked"]
            pub const UNLOCK: u32 = 0;
            #[doc = "Whitelist is locked"]
            pub const LOCK: u32 = 0x01;
        }
    }
    #[doc = "Allow non-secure write access to this register and the slot it controls"]
    pub mod TZ_NS {
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
    #[doc = "Allow user write access to this register and the slot it controls"]
    pub mod TZ_USER {
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
#[doc = "Slot2 Control"]
pub mod SLOT2_CTRL {
    #[doc = "Whitelist"]
    pub mod WHITE_LIST {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Lock whitelist"]
    pub mod LOCK_LIST {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Whitelist is not locked"]
            pub const UNLOCK: u32 = 0;
            #[doc = "Whitelist is locked"]
            pub const LOCK: u32 = 0x01;
        }
    }
    #[doc = "Allow non-secure write access to this register and the slot it controls"]
    pub mod TZ_NS {
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
    #[doc = "Allow user write access to this register and the slot it controls"]
    pub mod TZ_USER {
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
#[doc = "Slot3 Control"]
pub mod SLOT3_CTRL {
    #[doc = "Whitelist"]
    pub mod WHITE_LIST {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Lock whitelist"]
    pub mod LOCK_LIST {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Whitelist is not locked"]
            pub const UNLOCK: u32 = 0;
            #[doc = "Whitelist is locked"]
            pub const LOCK: u32 = 0x01;
        }
    }
    #[doc = "Allow non-secure write access to this register and the slot it controls"]
    pub mod TZ_NS {
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
    #[doc = "Allow user write access to this register and the slot it controls"]
    pub mod TZ_USER {
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
#[doc = "Slot 4 Control"]
pub mod SLOT4_CTRL {
    #[doc = "Whitelist"]
    pub mod WHITE_LIST {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Lock whitelist"]
    pub mod LOCK_LIST {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Whitelist is not locked"]
            pub const UNLOCK: u32 = 0;
            #[doc = "Whitelist is locked"]
            pub const LOCK: u32 = 0x01;
        }
    }
    #[doc = "Allow non-secure write access to this register and the slot it controls"]
    pub mod TZ_NS {
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
    #[doc = "Allow user write access to this register and the slot it controls"]
    pub mod TZ_USER {
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
