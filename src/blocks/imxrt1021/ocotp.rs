#[doc = "no description available"]
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
    _reserved5: [u8; 0x20],
    #[doc = "OTP Controller Version Register"]
    pub VERSION: crate::RORegister<u32>,
    _reserved6: [u8; 0x6c],
    #[doc = "OTP Controller Timing Register 2"]
    pub TIMING2: crate::RWRegister<u32>,
    _reserved7: [u8; 0x02fc],
    #[doc = "Value of OTP Bank0 Word0 (Lock controls)"]
    pub LOCK: crate::RWRegister<u32>,
    _reserved8: [u8; 0x0c],
    #[doc = "Value of OTP Bank0 Word1 (Configuration and Manufacturing Info.)"]
    pub CFG0: crate::RWRegister<u32>,
    _reserved9: [u8; 0x0c],
    #[doc = "Value of OTP Bank0 Word2 (Configuration and Manufacturing Info.)"]
    pub CFG1: crate::RWRegister<u32>,
    _reserved10: [u8; 0x0c],
    #[doc = "Value of OTP Bank0 Word3 (Configuration and Manufacturing Info.)"]
    pub CFG2: crate::RWRegister<u32>,
    _reserved11: [u8; 0x0c],
    #[doc = "Value of OTP Bank0 Word4 (Configuration and Manufacturing Info.)"]
    pub CFG3: crate::RWRegister<u32>,
    _reserved12: [u8; 0x0c],
    #[doc = "Value of OTP Bank0 Word5 (Configuration and Manufacturing Info.)"]
    pub CFG4: crate::RWRegister<u32>,
    _reserved13: [u8; 0x0c],
    #[doc = "Value of OTP Bank0 Word6 (Configuration and Manufacturing Info.)"]
    pub CFG5: crate::RWRegister<u32>,
    _reserved14: [u8; 0x0c],
    #[doc = "Value of OTP Bank0 Word7 (Configuration and Manufacturing Info.)"]
    pub CFG6: crate::RWRegister<u32>,
    _reserved15: [u8; 0x0c],
    #[doc = "Value of OTP Bank1 Word0 (Memory Related Info.)"]
    pub MEM0: crate::RWRegister<u32>,
    _reserved16: [u8; 0x0c],
    #[doc = "Value of OTP Bank1 Word1 (Memory Related Info.)"]
    pub MEM1: crate::RWRegister<u32>,
    _reserved17: [u8; 0x0c],
    #[doc = "Value of OTP Bank1 Word2 (Memory Related Info.)"]
    pub MEM2: crate::RWRegister<u32>,
    _reserved18: [u8; 0x0c],
    #[doc = "Value of OTP Bank1 Word3 (Memory Related Info.)"]
    pub MEM3: crate::RWRegister<u32>,
    _reserved19: [u8; 0x0c],
    #[doc = "Value of OTP Bank1 Word4 (Memory Related Info.)"]
    pub MEM4: crate::RWRegister<u32>,
    _reserved20: [u8; 0x0c],
    #[doc = "Value of OTP Bank1 Word5 (Analog Info.)"]
    pub ANA0: crate::RWRegister<u32>,
    _reserved21: [u8; 0x0c],
    #[doc = "Value of OTP Bank1 Word6 (Analog Info.)"]
    pub ANA1: crate::RWRegister<u32>,
    _reserved22: [u8; 0x0c],
    #[doc = "Value of OTP Bank1 Word7 (Analog Info.)"]
    pub ANA2: crate::RWRegister<u32>,
    _reserved23: [u8; 0x8c],
    #[doc = "Shadow Register for OTP Bank3 Word0 (SRK Hash)"]
    pub SRK0: crate::RWRegister<u32>,
    _reserved24: [u8; 0x0c],
    #[doc = "Shadow Register for OTP Bank3 Word1 (SRK Hash)"]
    pub SRK1: crate::RWRegister<u32>,
    _reserved25: [u8; 0x0c],
    #[doc = "Shadow Register for OTP Bank3 Word2 (SRK Hash)"]
    pub SRK2: crate::RWRegister<u32>,
    _reserved26: [u8; 0x0c],
    #[doc = "Shadow Register for OTP Bank3 Word3 (SRK Hash)"]
    pub SRK3: crate::RWRegister<u32>,
    _reserved27: [u8; 0x0c],
    #[doc = "Shadow Register for OTP Bank3 Word4 (SRK Hash)"]
    pub SRK4: crate::RWRegister<u32>,
    _reserved28: [u8; 0x0c],
    #[doc = "Shadow Register for OTP Bank3 Word5 (SRK Hash)"]
    pub SRK5: crate::RWRegister<u32>,
    _reserved29: [u8; 0x0c],
    #[doc = "Shadow Register for OTP Bank3 Word6 (SRK Hash)"]
    pub SRK6: crate::RWRegister<u32>,
    _reserved30: [u8; 0x0c],
    #[doc = "Shadow Register for OTP Bank3 Word7 (SRK Hash)"]
    pub SRK7: crate::RWRegister<u32>,
    _reserved31: [u8; 0x0c],
    #[doc = "Value of OTP Bank4 Word0 (Secure JTAG Response Field)"]
    pub SJC_RESP0: crate::RWRegister<u32>,
    _reserved32: [u8; 0x0c],
    #[doc = "Value of OTP Bank4 Word1 (Secure JTAG Response Field)"]
    pub SJC_RESP1: crate::RWRegister<u32>,
    _reserved33: [u8; 0x0c],
    #[doc = "Value of OTP Bank4 Word2 (MAC Address)"]
    pub MAC0: crate::RWRegister<u32>,
    _reserved34: [u8; 0x0c],
    #[doc = "Value of OTP Bank4 Word3 (MAC Address)"]
    pub MAC1: crate::RWRegister<u32>,
    _reserved35: [u8; 0x0c],
    #[doc = "Value of OTP Bank4 Word4 (MAC Address)"]
    pub GP3: crate::RWRegister<u32>,
    _reserved36: [u8; 0x1c],
    #[doc = "Value of OTP Bank4 Word6 (General Purpose Customer Defined Info)"]
    pub GP1: crate::RWRegister<u32>,
    _reserved37: [u8; 0x0c],
    #[doc = "Value of OTP Bank4 Word7 (General Purpose Customer Defined Info)"]
    pub GP2: crate::RWRegister<u32>,
    _reserved38: [u8; 0x0c],
    #[doc = "Value of OTP Bank5 Word0 (SW GP1)"]
    pub SW_GP1: crate::RWRegister<u32>,
    _reserved39: [u8; 0x0c],
    #[doc = "Value of OTP Bank5 Word1 (SW GP2)"]
    pub SW_GP20: crate::RWRegister<u32>,
    _reserved40: [u8; 0x0c],
    #[doc = "Value of OTP Bank5 Word2 (SW GP2)"]
    pub SW_GP21: crate::RWRegister<u32>,
    _reserved41: [u8; 0x0c],
    #[doc = "Value of OTP Bank5 Word3 (SW GP2)"]
    pub SW_GP22: crate::RWRegister<u32>,
    _reserved42: [u8; 0x0c],
    #[doc = "Value of OTP Bank5 Word4 (SW GP2)"]
    pub SW_GP23: crate::RWRegister<u32>,
    _reserved43: [u8; 0x0c],
    #[doc = "Value of OTP Bank5 Word5 (Misc Conf)"]
    pub MISC_CONF0: crate::RWRegister<u32>,
    _reserved44: [u8; 0x0c],
    #[doc = "Value of OTP Bank5 Word6 (Misc Conf)"]
    pub MISC_CONF1: crate::RWRegister<u32>,
    _reserved45: [u8; 0x0c],
    #[doc = "Value of OTP Bank5 Word7 (SRK Revoke)"]
    pub SRK_REVOKE: crate::RWRegister<u32>,
}
#[doc = "OTP Controller Control Register"]
pub mod CTRL {
    #[doc = "OTP write and read access address register"]
    pub mod ADDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "OTP controller status bit"]
    pub mod BUSY {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set by the controller when an access to a locked region(OTP or shadow register) is requested"]
    pub mod ERROR {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set to force re-loading the shadow registers (HW/SW capability and LOCK)"]
    pub mod RELOAD_SHADOWS {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Write 0x3E77 to enable OTP write accesses"]
    pub mod WR_UNLOCK {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "OTP Controller Control Register"]
pub mod CTRL_SET {
    #[doc = "OTP write and read access address register"]
    pub mod ADDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "OTP controller status bit"]
    pub mod BUSY {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set by the controller when an access to a locked region(OTP or shadow register) is requested"]
    pub mod ERROR {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set to force re-loading the shadow registers (HW/SW capability and LOCK)"]
    pub mod RELOAD_SHADOWS {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Write 0x3E77 to enable OTP write accesses"]
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
    #[doc = "OTP write and read access address register"]
    pub mod ADDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "OTP controller status bit"]
    pub mod BUSY {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set by the controller when an access to a locked region(OTP or shadow register) is requested"]
    pub mod ERROR {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set to force re-loading the shadow registers (HW/SW capability and LOCK)"]
    pub mod RELOAD_SHADOWS {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Write 0x3E77 to enable OTP write accesses"]
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
    #[doc = "OTP write and read access address register"]
    pub mod ADDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "OTP controller status bit"]
    pub mod BUSY {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set by the controller when an access to a locked region(OTP or shadow register) is requested"]
    pub mod ERROR {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set to force re-loading the shadow registers (HW/SW capability and LOCK)"]
    pub mod RELOAD_SHADOWS {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Write 0x3E77 to enable OTP write accesses"]
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
    #[doc = "This count value specifies the strobe period in one time write OTP"]
    pub mod STROBE_PROG {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This count value specifies the time to add to all default timing parameters other than the Tpgm and Trd"]
    pub mod RELAX {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This count value specifies the strobe period in one time read OTP"]
    pub mod STROBE_READ {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This count value specifies time interval between auto read and write access in one time program"]
    pub mod WAIT {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "OTP Controller Write Data Register"]
pub mod DATA {
    #[doc = "Used to initiate a write to OTP"]
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
    #[doc = "Used to initiate a read to OTP"]
    pub mod READ_FUSE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "OTP Controller Read Data Register"]
pub mod READ_FUSE_DATA {
    #[doc = "The data read from OTP"]
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
    #[doc = "Shadow register read and OTP read lock for DTCP_KEY region"]
    pub mod BLOCK_DTCP_KEY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Shadow register write and OTP write lock for SRK_REVOKE region"]
    pub mod SRK_REVOKE_LOCK {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Shadow register write and OTP write lock for FIELD_RETURN region"]
    pub mod FIELD_RETURN_LOCK {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set by ARM during Boot after DTCP is initialized and before test mode entry, if ROM_PART_LOCK=1"]
    pub mod BLOCK_ROM_PART {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set by ARM during Boot after DTCP is initialized and before test mode entry"]
    pub mod JTAG_BLOCK_RELEASE {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Software Controllable Signals Register"]
pub mod SCS {
    #[doc = "HAB JTAG Debug Enable"]
    pub mod HAB_JDE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Unallocated read/write bits for implementation specific software use."]
    pub mod SPARE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x3fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "When set, all of the bits in this register are locked and can not be changed through SW programming"]
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
    #[doc = "HAB JTAG Debug Enable"]
    pub mod HAB_JDE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Unallocated read/write bits for implementation specific software use."]
    pub mod SPARE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x3fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "When set, all of the bits in this register are locked and can not be changed through SW programming"]
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
    #[doc = "HAB JTAG Debug Enable"]
    pub mod HAB_JDE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Unallocated read/write bits for implementation specific software use."]
    pub mod SPARE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x3fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "When set, all of the bits in this register are locked and can not be changed through SW programming"]
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
    #[doc = "HAB JTAG Debug Enable"]
    pub mod HAB_JDE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Unallocated read/write bits for implementation specific software use."]
    pub mod SPARE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x3fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "When set, all of the bits in this register are locked and can not be changed through SW programming"]
    pub mod LOCK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "OTP Controller Version Register"]
pub mod VERSION {
    #[doc = "Fixed read-only value reflecting the stepping of the RTL version."]
    pub mod STEP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Fixed read-only value reflecting the MINOR field of the RTL version."]
    pub mod MINOR {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Fixed read-only value reflecting the MAJOR field of the RTL version."]
    pub mod MAJOR {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "OTP Controller Timing Register 2"]
pub mod TIMING2 {
    #[doc = "This count value specifies the strobe period in one time write OTP"]
    pub mod RELAX_PROG {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This count value specifies the strobe period in one time read OTP"]
    pub mod RELAX_READ {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This count value specifies time interval between auto read and write access in one time program"]
    pub mod RELAX1 {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x7f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Value of OTP Bank0 Word0 (Lock controls)"]
pub mod LOCK {
    #[doc = "Status of shadow register and OTP write lock for tester region"]
    pub mod TESTER {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Status of shadow register and OTP write lock for boot_cfg region"]
    pub mod BOOT_CFG {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Status of shadow register and OTP write lock for mem_trim region"]
    pub mod MEM_TRIM {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Status of shadow register read and write, OTP read and write lock for sjc_resp region"]
    pub mod SJC_RESP {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Status of shadow register and OTP write lock for mac_addr region"]
    pub mod MAC_ADDR {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Status of shadow register and OTP write lock for gp1 region"]
    pub mod GP1 {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Status of shadow register and OTP write lock for gp2 region"]
    pub mod GP2 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Status of shadow register read and write, OTP read and write lock for otpmk region (MSB)"]
    pub mod OTPMK_MSB {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Status of shadow register and OTP write lock for sw_gp1 region"]
    pub mod SW_GP1 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Status of shadow register read and write, OTP read and write lock for otpmk region (LSB)"]
    pub mod OTPMK_LSB {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Status of shadow register and OTP write lock for analog region"]
    pub mod ANALOG {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Status of shadow register and OTP write lock for otpmk_crc region"]
    pub mod OTPMK_CRC {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Status of shadow register and OTP write lock for sw_gp2 region"]
    pub mod SW_GP2_LOCK {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Status of shadow register and OTP write lock for misc_conf region"]
    pub mod MISC_CONF {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Status of shadow register and OTP read lock for sw_gp2 region"]
    pub mod SW_GP2_RLOCK {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Status of shadow register and OTP write lock for gp3 region"]
    pub mod GP3 {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Value of OTP Bank0 Word1 (Configuration and Manufacturing Info.)"]
pub mod CFG0 {
    #[doc = "This register contains 32 bits of the Unique ID and SJC_CHALLENGE field"]
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
    #[doc = "This register contains 32 bits of the Unique ID and SJC_CHALLENGE field"]
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
    #[doc = "Reflects value of OTP Bank 0, word 3 (ADDR = 0x03)"]
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
    #[doc = "Reflects value of OTP Bank 0, word 4 (ADDR = 0x04)"]
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
    #[doc = "Reflects value of OTP Bank 0, word 5 (ADDR = 0x05)"]
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
    #[doc = "Reflects value of OTP Bank 0, word 6 (ADDR = 0x06)"]
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
    #[doc = "Reflects value of OTP Bank 0, word 7 (ADDR = 0x07)"]
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
    #[doc = "Reflects value of OTP bank 1, word 0 (ADDR = 0x08)"]
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
    #[doc = "Reflects value of OTP bank 1, word 1 (ADDR = 0x09)"]
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
    #[doc = "Reflects value of OTP bank 1, word 2 (ADDR = 0x0A)"]
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
    #[doc = "Reflects value of OTP bank 1, word 3 (ADDR = 0x0B)"]
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
    #[doc = "Reflects value of OTP bank 1, word 4 (ADDR = 0x0C)"]
    pub mod BITS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Value of OTP Bank1 Word5 (Analog Info.)"]
pub mod ANA0 {
    #[doc = "Reflects value of OTP bank 1, word 5 (ADDR = 0x0D)"]
    pub mod BITS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Value of OTP Bank1 Word6 (Analog Info.)"]
pub mod ANA1 {
    #[doc = "Reflects value of OTP bank 1, word 6 (ADDR = 0x0E)"]
    pub mod BITS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Value of OTP Bank1 Word7 (Analog Info.)"]
pub mod ANA2 {
    #[doc = "Reflects value of OTP bank 1, word 7 (ADDR = 0x0F)"]
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
    #[doc = "Shadow register for the hash of the Super Root Key word0 (Copy of OTP Bank 3, word 0 (ADDR = 0x1C))"]
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
    #[doc = "Shadow register for the hash of the Super Root Key word1 (Copy of OTP Bank 3, word 1 (ADDR = 0x1D))"]
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
    #[doc = "Shadow register for the hash of the Super Root Key word2 (Copy of OTP Bank 3, word 2 (ADDR = 0x1E))"]
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
    #[doc = "Shadow register for the hash of the Super Root Key word3 (Copy of OTP Bank 3, word 3 (ADDR = 0x1F))"]
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
    #[doc = "Shadow register for the hash of the Super Root Key word4 (Copy of OTP Bank 3, word 4 (ADDR = 0x20))"]
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
    #[doc = "Shadow register for the hash of the Super Root Key word5 (Copy of OTP Bank 3, word 5 (ADDR = 0x21))"]
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
    #[doc = "Shadow register for the hash of the Super Root Key word6 (Copy of OTP Bank 3, word 6 (ADDR = 0x22))"]
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
    #[doc = "Shadow register for the hash of the Super Root Key word7 (Copy of OTP Bank 3, word 7 (ADDR = 0x23))"]
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
    #[doc = "Shadow register for the SJC_RESP Key word0 (Copy of OTP Bank 4, word 0 (ADDR = 0x20))"]
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
    #[doc = "Shadow register for the SJC_RESP Key word1 (Copy of OTP Bank 4, word 1 (ADDR = 0x21))"]
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
    #[doc = "Reflects value of OTP Bank 4, word 2 (ADDR = 0x22)."]
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
    #[doc = "Reflects value of OTP Bank 4, word 3 (ADDR = 0x23)."]
    pub mod BITS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Value of OTP Bank4 Word4 (MAC Address)"]
pub mod GP3 {
    #[doc = "Reflects value of OTP Bank 4, word 4 (ADDR = 0x24)."]
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
    #[doc = "Reflects value of OTP Bank 4, word 6 (ADDR = 0x26)."]
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
    #[doc = "Reflects value of OTP Bank 4, word 7 (ADDR = 0x27)."]
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
    #[doc = "Reflects value of OTP Bank 5, word 0 (ADDR = 0x28)."]
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
    #[doc = "Reflects value of OTP Bank 5, word 1 (ADDR = 0x29)."]
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
    #[doc = "Reflects value of OTP Bank 5, word 2 (ADDR = 0x2a)."]
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
    #[doc = "Reflects value of OTP Bank 5, word 3 (ADDR = 0x2b)."]
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
    #[doc = "Reflects value of OTP Bank 5, word 4 (ADDR = 0x2c)."]
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
    #[doc = "Reflects value of OTP Bank 5, word 5 (ADDR = 0x2d)."]
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
    #[doc = "Reflects value of OTP Bank 5, word 6 (ADDR = 0x2e)."]
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
    #[doc = "Reflects value of OTP Bank 5, word 7 (ADDR = 0x2f)."]
    pub mod BITS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
