#[doc = "OCOTP"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "OTP Controller Control Register"]
    pub CTRL: crate::RWRegister<u32>,
    #[doc = "OTP Controller Control Register"]
    pub CTRL_SET: crate::RWRegister<u32>,
    #[doc = "OTP Controller Control Register"]
    pub CTRL_CLR: crate::RWRegister<u32>,
    #[doc = "OTP Controller Control Register"]
    pub CTRL_TOG: crate::RWRegister<u32>,
    #[doc = "OTP Controller Timing Register"]
    pub TIMING: crate::RWRegister<u32>,
    _reserved0: [u8; 0x0c],
    #[doc = "OTP Controller Write Data Register"]
    pub DATA: crate::RWRegister<u32>,
    _reserved1: [u8; 0x0c],
    #[doc = "OTP Controller Write Data Register"]
    pub READ_CTRL: crate::RWRegister<u32>,
    _reserved2: [u8; 0x0c],
    #[doc = "OTP Controller Read Data Register"]
    pub READ_FUSE_DATA: crate::RWRegister<u32>,
    _reserved3: [u8; 0x0c],
    #[doc = "Sticky bit Register"]
    pub SW_STICKY: crate::RWRegister<u32>,
    _reserved4: [u8; 0x0c],
    #[doc = "Software Controllable Signals Register"]
    pub SCS: crate::RWRegister<u32>,
    #[doc = "Software Controllable Signals Register"]
    pub SCS_SET: crate::RWRegister<u32>,
    #[doc = "Software Controllable Signals Register"]
    pub SCS_CLR: crate::RWRegister<u32>,
    #[doc = "Software Controllable Signals Register"]
    pub SCS_TOG: crate::RWRegister<u32>,
    #[doc = "OTP Controller CRC test address"]
    pub CRC_ADDR: crate::RWRegister<u32>,
    _reserved5: [u8; 0x0c],
    #[doc = "OTP Controller CRC Value Register"]
    pub CRC_VALUE: crate::RWRegister<u32>,
    _reserved6: [u8; 0x0c],
    #[doc = "OTP Controller Version Register"]
    pub VERSION: crate::RORegister<u32>,
    _reserved7: [u8; 0x6c],
    #[doc = "OTP Controller Timing Register"]
    pub TIMING2: crate::RWRegister<u32>,
    _reserved8: [u8; 0x02fc],
    #[doc = "Value of OTP Bank0 Word0 (Lock controls)"]
    pub LOCK: crate::RWRegister<u32>,
    _reserved9: [u8; 0x0c],
    #[doc = "Value of OTP Bank0 Word1 (Configuration and Manufacturing Info.)"]
    pub CFG0: crate::RWRegister<u32>,
    _reserved10: [u8; 0x0c],
    #[doc = "Value of OTP Bank0 Word2 (Configuration and Manufacturing Info.)"]
    pub CFG1: crate::RWRegister<u32>,
    _reserved11: [u8; 0x0c],
    #[doc = "Value of OTP Bank0 Word3 (Configuration and Manufacturing Info.)"]
    pub CFG2: crate::RWRegister<u32>,
    _reserved12: [u8; 0x0c],
    #[doc = "Value of OTP Bank0 Word4 (Configuration and Manufacturing Info.)"]
    pub CFG3: crate::RWRegister<u32>,
    _reserved13: [u8; 0x0c],
    #[doc = "Value of OTP Bank0 Word5 (Configuration and Manufacturing Info.)"]
    pub CFG4: crate::RWRegister<u32>,
    _reserved14: [u8; 0x0c],
    #[doc = "Value of OTP Bank0 Word6 (Configuration and Manufacturing Info.)"]
    pub CFG5: crate::RWRegister<u32>,
    _reserved15: [u8; 0x0c],
    #[doc = "Value of OTP Bank0 Word7 (Configuration and Manufacturing Info.)"]
    pub CFG6: crate::RWRegister<u32>,
    _reserved16: [u8; 0x0c],
    #[doc = "Value of OTP Bank1 Word0 (Memory Related Info.)"]
    pub MEM0: crate::RWRegister<u32>,
    _reserved17: [u8; 0x0c],
    #[doc = "Value of OTP Bank1 Word1 (Memory Related Info.)"]
    pub MEM1: crate::RWRegister<u32>,
    _reserved18: [u8; 0x0c],
    #[doc = "Value of OTP Bank1 Word2 (Memory Related Info.)"]
    pub MEM2: crate::RWRegister<u32>,
    _reserved19: [u8; 0x0c],
    #[doc = "Value of OTP Bank1 Word3 (Memory Related Info.)"]
    pub MEM3: crate::RWRegister<u32>,
    _reserved20: [u8; 0x0c],
    #[doc = "Value of OTP Bank1 Word4 (Memory Related Info.)"]
    pub MEM4: crate::RWRegister<u32>,
    _reserved21: [u8; 0x0c],
    #[doc = "Value of OTP Bank1 Word5 (Memory Related Info.)"]
    pub ANA0: crate::RWRegister<u32>,
    _reserved22: [u8; 0x0c],
    #[doc = "Value of OTP Bank1 Word6 (General Purpose Customer Defined Info.)"]
    pub ANA1: crate::RWRegister<u32>,
    _reserved23: [u8; 0x0c],
    #[doc = "Value of OTP Bank1 Word7 (General Purpose Customer Defined Info.)"]
    pub ANA2: crate::RWRegister<u32>,
    _reserved24: [u8; 0x0c],
    #[doc = "Value of OTP Bank2 Word0 (OTPMK Key)"]
    pub OTPMK0: crate::RWRegister<u32>,
    _reserved25: [u8; 0x0c],
    #[doc = "Value of OTP Bank2 Word1 (OTPMK Key)"]
    pub OTPMK1: crate::RWRegister<u32>,
    _reserved26: [u8; 0x0c],
    #[doc = "Value of OTP Bank2 Word2 (OTPMK Key)"]
    pub OTPMK2: crate::RWRegister<u32>,
    _reserved27: [u8; 0x0c],
    #[doc = "Value of OTP Bank2 Word3 (OTPMK Key)"]
    pub OTPMK3: crate::RWRegister<u32>,
    _reserved28: [u8; 0x0c],
    #[doc = "Value of OTP Bank2 Word4 (OTPMK Key)"]
    pub OTPMK4: crate::RWRegister<u32>,
    _reserved29: [u8; 0x0c],
    #[doc = "Value of OTP Bank2 Word5 (OTPMK Key)"]
    pub OTPMK5: crate::RWRegister<u32>,
    _reserved30: [u8; 0x0c],
    #[doc = "Value of OTP Bank2 Word6 (OTPMK Key)"]
    pub OTPMK6: crate::RWRegister<u32>,
    _reserved31: [u8; 0x0c],
    #[doc = "Value of OTP Bank2 Word7 (OTPMK Key)"]
    pub OTPMK7: crate::RWRegister<u32>,
    _reserved32: [u8; 0x0c],
    #[doc = "Shadow Register for OTP Bank3 Word0 (SRK Hash)"]
    pub SRK0: crate::RWRegister<u32>,
    _reserved33: [u8; 0x0c],
    #[doc = "Shadow Register for OTP Bank3 Word1 (SRK Hash)"]
    pub SRK1: crate::RWRegister<u32>,
    _reserved34: [u8; 0x0c],
    #[doc = "Shadow Register for OTP Bank3 Word2 (SRK Hash)"]
    pub SRK2: crate::RWRegister<u32>,
    _reserved35: [u8; 0x0c],
    #[doc = "Shadow Register for OTP Bank3 Word3 (SRK Hash)"]
    pub SRK3: crate::RWRegister<u32>,
    _reserved36: [u8; 0x0c],
    #[doc = "Shadow Register for OTP Bank3 Word4 (SRK Hash)"]
    pub SRK4: crate::RWRegister<u32>,
    _reserved37: [u8; 0x0c],
    #[doc = "Shadow Register for OTP Bank3 Word5 (SRK Hash)"]
    pub SRK5: crate::RWRegister<u32>,
    _reserved38: [u8; 0x0c],
    #[doc = "Shadow Register for OTP Bank3 Word6 (SRK Hash)"]
    pub SRK6: crate::RWRegister<u32>,
    _reserved39: [u8; 0x0c],
    #[doc = "Shadow Register for OTP Bank3 Word7 (SRK Hash)"]
    pub SRK7: crate::RWRegister<u32>,
    _reserved40: [u8; 0x0c],
    #[doc = "Value of OTP Bank4 Word0 (Secure JTAG Response Field)"]
    pub SJC_RESP0: crate::RWRegister<u32>,
    _reserved41: [u8; 0x0c],
    #[doc = "Value of OTP Bank4 Word1 (Secure JTAG Response Field)"]
    pub SJC_RESP1: crate::RWRegister<u32>,
    _reserved42: [u8; 0x0c],
    #[doc = "Value of OTP Bank4 Word2 (MAC Address)"]
    pub MAC0: crate::RWRegister<u32>,
    _reserved43: [u8; 0x0c],
    #[doc = "Value of OTP Bank4 Word3 (MAC Address)"]
    pub MAC1: crate::RWRegister<u32>,
    _reserved44: [u8; 0x0c],
    #[doc = "Value of OTP Bank4 Word4 (MAC2 Address)"]
    pub MAC2: crate::RWRegister<u32>,
    _reserved45: [u8; 0x0c],
    #[doc = "Value of OTP Bank4 Word5 (CRC Key)"]
    pub OTPMK_CRC32: crate::RWRegister<u32>,
    _reserved46: [u8; 0x0c],
    #[doc = "Value of OTP Bank4 Word6 (General Purpose Customer Defined Info)"]
    pub GP1: crate::RWRegister<u32>,
    _reserved47: [u8; 0x0c],
    #[doc = "Value of OTP Bank4 Word7 (General Purpose Customer Defined Info)"]
    pub GP2: crate::RWRegister<u32>,
    _reserved48: [u8; 0x0c],
    #[doc = "Value of OTP Bank5 Word0 (SW GP1)"]
    pub SW_GP1: crate::RWRegister<u32>,
    _reserved49: [u8; 0x0c],
    #[doc = "Value of OTP Bank5 Word1 (SW GP2)"]
    pub SW_GP20: crate::RWRegister<u32>,
    _reserved50: [u8; 0x0c],
    #[doc = "Value of OTP Bank5 Word2 (SW GP2)"]
    pub SW_GP21: crate::RWRegister<u32>,
    _reserved51: [u8; 0x0c],
    #[doc = "Value of OTP Bank5 Word3 (SW GP2)"]
    pub SW_GP22: crate::RWRegister<u32>,
    _reserved52: [u8; 0x0c],
    #[doc = "Value of OTP Bank5 Word4 (SW GP2)"]
    pub SW_GP23: crate::RWRegister<u32>,
    _reserved53: [u8; 0x0c],
    #[doc = "Value of OTP Bank5 Word5 (Misc Conf)"]
    pub MISC_CONF0: crate::RWRegister<u32>,
    _reserved54: [u8; 0x0c],
    #[doc = "Value of OTP Bank5 Word6 (Misc Conf)"]
    pub MISC_CONF1: crate::RWRegister<u32>,
    _reserved55: [u8; 0x0c],
    #[doc = "Value of OTP Bank5 Word7 (SRK Revoke)"]
    pub SRK_REVOKE: crate::RWRegister<u32>,
    _reserved56: [u8; 0x010c],
    #[doc = "Value of OTP Bank6 Word0 (ROM Patch)"]
    pub ROM_PATCH0: crate::RWRegister<u32>,
    _reserved57: [u8; 0x0c],
    #[doc = "Value of OTP Bank6 Word1 (ROM Patch)"]
    pub ROM_PATCH1: crate::RWRegister<u32>,
    _reserved58: [u8; 0x0c],
    #[doc = "Value of OTP Bank6 Word2 (ROM Patch)"]
    pub ROM_PATCH2: crate::RWRegister<u32>,
    _reserved59: [u8; 0x0c],
    #[doc = "Value of OTP Bank6 Word3 (ROM Patch)"]
    pub ROM_PATCH3: crate::RWRegister<u32>,
    _reserved60: [u8; 0x0c],
    #[doc = "Value of OTP Bank6 Word4 (ROM Patch)"]
    pub ROM_PATCH4: crate::RWRegister<u32>,
    _reserved61: [u8; 0x0c],
    #[doc = "Value of OTP Bank6 Word5 (ROM Patch)"]
    pub ROM_PATCH5: crate::RWRegister<u32>,
    _reserved62: [u8; 0x0c],
    #[doc = "Value of OTP Bank6 Word6 (ROM Patch)"]
    pub ROM_PATCH6: crate::RWRegister<u32>,
    _reserved63: [u8; 0x0c],
    #[doc = "Value of OTP Bank6 Word7 (ROM Patch)"]
    pub ROM_PATCH7: crate::RWRegister<u32>,
    _reserved64: [u8; 0x0c],
    #[doc = "Value of OTP Bank7 Word0 (GP3)"]
    pub GP30: crate::RWRegister<u32>,
    _reserved65: [u8; 0x0c],
    #[doc = "Value of OTP Bank7 Word1 (GP3)"]
    pub GP31: crate::RWRegister<u32>,
    _reserved66: [u8; 0x0c],
    #[doc = "Value of OTP Bank7 Word2 (GP3)"]
    pub GP32: crate::RWRegister<u32>,
    _reserved67: [u8; 0x0c],
    #[doc = "Value of OTP Bank7 Word3 (GP3)"]
    pub GP33: crate::RWRegister<u32>,
    _reserved68: [u8; 0x0c],
    #[doc = "Value of OTP Bank7 Word4 (GP4)"]
    pub GP40: crate::RWRegister<u32>,
    _reserved69: [u8; 0x0c],
    #[doc = "Value of OTP Bank7 Word5 (GP4)"]
    pub GP41: crate::RWRegister<u32>,
    _reserved70: [u8; 0x0c],
    #[doc = "Value of OTP Bank7 Word6 (GP4)"]
    pub GP42: crate::RWRegister<u32>,
    _reserved71: [u8; 0x0c],
    #[doc = "Value of OTP Bank7 Word7 (GP4)"]
    pub GP43: crate::RWRegister<u32>,
}
#[doc = "OTP Controller Control Register"]
pub mod CTRL {
    #[doc = "ADDR"]
    pub mod ADDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "RSVD0"]
    pub mod RSVD0 {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "BUSY"]
    pub mod BUSY {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ERROR"]
    pub mod ERROR {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "RELOAD_SHADOWS"]
    pub mod RELOAD_SHADOWS {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CRC_TEST"]
    pub mod CRC_TEST {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CRC_FAIL"]
    pub mod CRC_FAIL {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "RSVD1"]
    pub mod RSVD1 {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "WR_UNLOCK"]
    pub mod WR_UNLOCK {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Key needed to unlock HW_OCOTP_DATA register."]
            pub const KEY: u32 = 0x3e77;
        }
    }
}
#[doc = "OTP Controller Control Register"]
pub mod CTRL_SET {
    #[doc = "ADDR"]
    pub mod ADDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "RSVD0"]
    pub mod RSVD0 {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "BUSY"]
    pub mod BUSY {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ERROR"]
    pub mod ERROR {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "RELOAD_SHADOWS"]
    pub mod RELOAD_SHADOWS {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CRC_TEST"]
    pub mod CRC_TEST {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CRC_FAIL"]
    pub mod CRC_FAIL {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "RSVD1"]
    pub mod RSVD1 {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "WR_UNLOCK"]
    pub mod WR_UNLOCK {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "OTP Controller Control Register"]
pub mod CTRL_CLR {
    #[doc = "ADDR"]
    pub mod ADDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "RSVD0"]
    pub mod RSVD0 {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "BUSY"]
    pub mod BUSY {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ERROR"]
    pub mod ERROR {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "RELOAD_SHADOWS"]
    pub mod RELOAD_SHADOWS {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CRC_TEST"]
    pub mod CRC_TEST {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CRC_FAIL"]
    pub mod CRC_FAIL {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "RSVD1"]
    pub mod RSVD1 {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "WR_UNLOCK"]
    pub mod WR_UNLOCK {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "OTP Controller Control Register"]
pub mod CTRL_TOG {
    #[doc = "ADDR"]
    pub mod ADDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "RSVD0"]
    pub mod RSVD0 {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "BUSY"]
    pub mod BUSY {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ERROR"]
    pub mod ERROR {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "RELOAD_SHADOWS"]
    pub mod RELOAD_SHADOWS {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CRC_TEST"]
    pub mod CRC_TEST {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CRC_FAIL"]
    pub mod CRC_FAIL {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "RSVD1"]
    pub mod RSVD1 {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "WR_UNLOCK"]
    pub mod WR_UNLOCK {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "OTP Controller Timing Register"]
pub mod TIMING {
    #[doc = "STROBE_PROG"]
    pub mod STROBE_PROG {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "RELAX"]
    pub mod RELAX {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "STROBE_READ"]
    pub mod STROBE_READ {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "WAIT"]
    pub mod WAIT {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "RSRVD0"]
    pub mod RSRVD0 {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "OTP Controller Write Data Register"]
pub mod DATA {
    #[doc = "DATA"]
    pub mod DATA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "OTP Controller Write Data Register"]
pub mod READ_CTRL {
    #[doc = "READ_FUSE"]
    pub mod READ_FUSE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "RSVD0"]
    pub mod RSVD0 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x7fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "OTP Controller Read Data Register"]
pub mod READ_FUSE_DATA {
    #[doc = "DATA"]
    pub mod DATA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Sticky bit Register"]
pub mod SW_STICKY {
    #[doc = "BLOCK_DTCP_KEY"]
    pub mod BLOCK_DTCP_KEY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "SRK_REVOKE_LOCK"]
    pub mod SRK_REVOKE_LOCK {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "FIELD_RETURN_LOCK"]
    pub mod FIELD_RETURN_LOCK {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "BLOCK_ROM_PART"]
    pub mod BLOCK_ROM_PART {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "JTAG_BLOCK_RELEASE"]
    pub mod JTAG_BLOCK_RELEASE {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "RSVD0"]
    pub mod RSVD0 {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x07ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Software Controllable Signals Register"]
pub mod SCS {
    #[doc = "HAB_JDE"]
    pub mod HAB_JDE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "SPARE"]
    pub mod SPARE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x3fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "LOCK"]
    pub mod LOCK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Software Controllable Signals Register"]
pub mod SCS_SET {
    #[doc = "HAB_JDE"]
    pub mod HAB_JDE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "SPARE"]
    pub mod SPARE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x3fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "LOCK"]
    pub mod LOCK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Software Controllable Signals Register"]
pub mod SCS_CLR {
    #[doc = "HAB_JDE"]
    pub mod HAB_JDE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "SPARE"]
    pub mod SPARE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x3fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "LOCK"]
    pub mod LOCK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Software Controllable Signals Register"]
pub mod SCS_TOG {
    #[doc = "HAB_JDE"]
    pub mod HAB_JDE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "SPARE"]
    pub mod SPARE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x3fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "LOCK"]
    pub mod LOCK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "OTP Controller CRC test address"]
pub mod CRC_ADDR {
    #[doc = "DATA_START_ADDR"]
    pub mod DATA_START_ADDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DATA_END_ADDR"]
    pub mod DATA_END_ADDR {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CRC_ADDR"]
    pub mod CRC_ADDR {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "OTPMK_CRC"]
    pub mod OTPMK_CRC {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "RSVD0"]
    pub mod RSVD0 {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x7f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "OTP Controller CRC Value Register"]
pub mod CRC_VALUE {
    #[doc = "DATA"]
    pub mod DATA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "OTP Controller Version Register"]
pub mod VERSION {
    #[doc = "STEP"]
    pub mod STEP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "MINOR"]
    pub mod MINOR {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "MAJOR"]
    pub mod MAJOR {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "OTP Controller Timing Register"]
pub mod TIMING2 {
    #[doc = "RELAX_PROG"]
    pub mod RELAX_PROG {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "RSRVD0"]
    pub mod RSRVD0 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "RELAX_READ"]
    pub mod RELAX_READ {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "RSRVD0"]
    pub mod RSRVD1 {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x03ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Value of OTP Bank0 Word0 (Lock controls)"]
pub mod LOCK {
    #[doc = "TESTER"]
    pub mod TESTER {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "BOOT_CFG"]
    pub mod BOOT_CFG {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "MEM_TRIM"]
    pub mod MEM_TRIM {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "SJC_RESP"]
    pub mod SJC_RESP {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "GP4_RLOCK"]
    pub mod GP4_RLOCK {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "MAC_ADDR"]
    pub mod MAC_ADDR {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "GP1"]
    pub mod GP1 {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "GP2"]
    pub mod GP2 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ROM_PATCH"]
    pub mod ROM_PATCH {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "SW_GP1"]
    pub mod SW_GP1 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "OTPMK"]
    pub mod OTPMK {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ANALOG"]
    pub mod ANALOG {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "OTPMK_CRC"]
    pub mod OTPMK_CRC {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "SW_GP2_LOCK"]
    pub mod SW_GP2_LOCK {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "MISC_CONF"]
    pub mod MISC_CONF {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "SW_GP2_RLOCK"]
    pub mod SW_GP2_RLOCK {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "GP4"]
    pub mod GP4 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "GP3"]
    pub mod GP3 {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "FIELD_RETURN"]
    pub mod FIELD_RETURN {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Value of OTP Bank0 Word1 (Configuration and Manufacturing Info.)"]
pub mod CFG0 {
    #[doc = "BITS"]
    pub mod BITS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Value of OTP Bank0 Word2 (Configuration and Manufacturing Info.)"]
pub mod CFG1 {
    #[doc = "BITS"]
    pub mod BITS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Value of OTP Bank0 Word3 (Configuration and Manufacturing Info.)"]
pub mod CFG2 {
    #[doc = "BITS"]
    pub mod BITS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Value of OTP Bank0 Word4 (Configuration and Manufacturing Info.)"]
pub mod CFG3 {
    #[doc = "BITS"]
    pub mod BITS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Value of OTP Bank0 Word5 (Configuration and Manufacturing Info.)"]
pub mod CFG4 {
    #[doc = "BITS"]
    pub mod BITS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Value of OTP Bank0 Word6 (Configuration and Manufacturing Info.)"]
pub mod CFG5 {
    #[doc = "BITS"]
    pub mod BITS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Value of OTP Bank0 Word7 (Configuration and Manufacturing Info.)"]
pub mod CFG6 {
    #[doc = "BITS"]
    pub mod BITS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Value of OTP Bank1 Word0 (Memory Related Info.)"]
pub mod MEM0 {
    #[doc = "BITS"]
    pub mod BITS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Value of OTP Bank1 Word1 (Memory Related Info.)"]
pub mod MEM1 {
    #[doc = "BITS"]
    pub mod BITS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Value of OTP Bank1 Word2 (Memory Related Info.)"]
pub mod MEM2 {
    #[doc = "BITS"]
    pub mod BITS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Value of OTP Bank1 Word3 (Memory Related Info.)"]
pub mod MEM3 {
    #[doc = "BITS"]
    pub mod BITS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Value of OTP Bank1 Word4 (Memory Related Info.)"]
pub mod MEM4 {
    #[doc = "BITS"]
    pub mod BITS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Value of OTP Bank1 Word5 (Memory Related Info.)"]
pub mod ANA0 {
    #[doc = "BITS"]
    pub mod BITS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Value of OTP Bank1 Word6 (General Purpose Customer Defined Info.)"]
pub mod ANA1 {
    #[doc = "BITS"]
    pub mod BITS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Value of OTP Bank1 Word7 (General Purpose Customer Defined Info.)"]
pub mod ANA2 {
    #[doc = "BITS"]
    pub mod BITS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Value of OTP Bank2 Word0 (OTPMK Key)"]
pub mod OTPMK0 {
    #[doc = "BITS"]
    pub mod BITS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Value of OTP Bank2 Word1 (OTPMK Key)"]
pub mod OTPMK1 {
    #[doc = "BITS"]
    pub mod BITS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Value of OTP Bank2 Word2 (OTPMK Key)"]
pub mod OTPMK2 {
    #[doc = "BITS"]
    pub mod BITS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Value of OTP Bank2 Word3 (OTPMK Key)"]
pub mod OTPMK3 {
    #[doc = "BITS"]
    pub mod BITS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Value of OTP Bank2 Word4 (OTPMK Key)"]
pub mod OTPMK4 {
    #[doc = "BITS"]
    pub mod BITS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Value of OTP Bank2 Word5 (OTPMK Key)"]
pub mod OTPMK5 {
    #[doc = "BITS"]
    pub mod BITS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Value of OTP Bank2 Word6 (OTPMK Key)"]
pub mod OTPMK6 {
    #[doc = "BITS"]
    pub mod BITS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Value of OTP Bank2 Word7 (OTPMK Key)"]
pub mod OTPMK7 {
    #[doc = "BITS"]
    pub mod BITS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Shadow Register for OTP Bank3 Word0 (SRK Hash)"]
pub mod SRK0 {
    #[doc = "BITS"]
    pub mod BITS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Shadow Register for OTP Bank3 Word1 (SRK Hash)"]
pub mod SRK1 {
    #[doc = "BITS"]
    pub mod BITS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Shadow Register for OTP Bank3 Word2 (SRK Hash)"]
pub mod SRK2 {
    #[doc = "BITS"]
    pub mod BITS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Shadow Register for OTP Bank3 Word3 (SRK Hash)"]
pub mod SRK3 {
    #[doc = "BITS"]
    pub mod BITS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Shadow Register for OTP Bank3 Word4 (SRK Hash)"]
pub mod SRK4 {
    #[doc = "BITS"]
    pub mod BITS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Shadow Register for OTP Bank3 Word5 (SRK Hash)"]
pub mod SRK5 {
    #[doc = "BITS"]
    pub mod BITS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Shadow Register for OTP Bank3 Word6 (SRK Hash)"]
pub mod SRK6 {
    #[doc = "BITS"]
    pub mod BITS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Shadow Register for OTP Bank3 Word7 (SRK Hash)"]
pub mod SRK7 {
    #[doc = "BITS"]
    pub mod BITS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Value of OTP Bank4 Word0 (Secure JTAG Response Field)"]
pub mod SJC_RESP0 {
    #[doc = "BITS"]
    pub mod BITS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Value of OTP Bank4 Word1 (Secure JTAG Response Field)"]
pub mod SJC_RESP1 {
    #[doc = "BITS"]
    pub mod BITS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Value of OTP Bank4 Word2 (MAC Address)"]
pub mod MAC0 {
    #[doc = "BITS"]
    pub mod BITS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Value of OTP Bank4 Word3 (MAC Address)"]
pub mod MAC1 {
    #[doc = "BITS"]
    pub mod BITS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Value of OTP Bank4 Word4 (MAC2 Address)"]
pub mod MAC2 {
    #[doc = "BITS"]
    pub mod BITS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Value of OTP Bank4 Word5 (CRC Key)"]
pub mod OTPMK_CRC32 {
    #[doc = "BITS"]
    pub mod BITS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Value of OTP Bank4 Word6 (General Purpose Customer Defined Info)"]
pub mod GP1 {
    #[doc = "BITS"]
    pub mod BITS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Value of OTP Bank4 Word7 (General Purpose Customer Defined Info)"]
pub mod GP2 {
    #[doc = "BITS"]
    pub mod BITS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Value of OTP Bank5 Word0 (SW GP1)"]
pub mod SW_GP1 {
    #[doc = "BITS"]
    pub mod BITS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Value of OTP Bank5 Word1 (SW GP2)"]
pub mod SW_GP20 {
    #[doc = "BITS"]
    pub mod BITS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Value of OTP Bank5 Word2 (SW GP2)"]
pub mod SW_GP21 {
    #[doc = "BITS"]
    pub mod BITS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Value of OTP Bank5 Word3 (SW GP2)"]
pub mod SW_GP22 {
    #[doc = "BITS"]
    pub mod BITS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Value of OTP Bank5 Word4 (SW GP2)"]
pub mod SW_GP23 {
    #[doc = "BITS"]
    pub mod BITS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Value of OTP Bank5 Word5 (Misc Conf)"]
pub mod MISC_CONF0 {
    #[doc = "BITS"]
    pub mod BITS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Value of OTP Bank5 Word6 (Misc Conf)"]
pub mod MISC_CONF1 {
    #[doc = "BITS"]
    pub mod BITS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Value of OTP Bank5 Word7 (SRK Revoke)"]
pub mod SRK_REVOKE {
    #[doc = "BITS"]
    pub mod BITS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Value of OTP Bank6 Word0 (ROM Patch)"]
pub mod ROM_PATCH0 {
    #[doc = "BITS"]
    pub mod BITS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Value of OTP Bank6 Word1 (ROM Patch)"]
pub mod ROM_PATCH1 {
    #[doc = "BITS"]
    pub mod BITS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Value of OTP Bank6 Word2 (ROM Patch)"]
pub mod ROM_PATCH2 {
    #[doc = "BITS"]
    pub mod BITS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Value of OTP Bank6 Word3 (ROM Patch)"]
pub mod ROM_PATCH3 {
    #[doc = "BITS"]
    pub mod BITS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Value of OTP Bank6 Word4 (ROM Patch)"]
pub mod ROM_PATCH4 {
    #[doc = "BITS"]
    pub mod BITS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Value of OTP Bank6 Word5 (ROM Patch)"]
pub mod ROM_PATCH5 {
    #[doc = "BITS"]
    pub mod BITS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Value of OTP Bank6 Word6 (ROM Patch)"]
pub mod ROM_PATCH6 {
    #[doc = "BITS"]
    pub mod BITS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Value of OTP Bank6 Word7 (ROM Patch)"]
pub mod ROM_PATCH7 {
    #[doc = "BITS"]
    pub mod BITS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Value of OTP Bank7 Word0 (GP3)"]
pub mod GP30 {
    #[doc = "BITS"]
    pub mod BITS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Value of OTP Bank7 Word1 (GP3)"]
pub mod GP31 {
    #[doc = "BITS"]
    pub mod BITS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Value of OTP Bank7 Word2 (GP3)"]
pub mod GP32 {
    #[doc = "BITS"]
    pub mod BITS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Value of OTP Bank7 Word3 (GP3)"]
pub mod GP33 {
    #[doc = "BITS"]
    pub mod BITS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Value of OTP Bank7 Word4 (GP4)"]
pub mod GP40 {
    #[doc = "BITS"]
    pub mod BITS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Value of OTP Bank7 Word5 (GP4)"]
pub mod GP41 {
    #[doc = "BITS"]
    pub mod BITS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Value of OTP Bank7 Word6 (GP4)"]
pub mod GP42 {
    #[doc = "BITS"]
    pub mod BITS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Value of OTP Bank7 Word7 (GP4)"]
pub mod GP43 {
    #[doc = "BITS"]
    pub mod BITS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
