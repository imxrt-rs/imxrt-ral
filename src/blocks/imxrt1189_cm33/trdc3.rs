#[doc = "TRDC"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "TRDC Register"]
    pub TRDC_CR: crate::RWRegister<u32>,
    _reserved0: [u8; 0xec],
    #[doc = "TRDC Hardware Configuration Register 0"]
    pub TRDC_HWCFG0: crate::RORegister<u32>,
    #[doc = "TRDC Hardware Configuration Register 1"]
    pub TRDC_HWCFG1: crate::RORegister<u32>,
    #[doc = "TRDC Hardware Configuration Register 2"]
    pub TRDC_HWCFG2: crate::RORegister<u32>,
    #[doc = "TRDC Hardware Configuration Register 3"]
    pub TRDC_HWCFG3: crate::RORegister<u32>,
    #[doc = "Domain Assignment Configuration Register"]
    pub DACFG0: crate::RORegister<u8>,
    #[doc = "Domain Assignment Configuration Register"]
    pub DACFG1: crate::RORegister<u8>,
    #[doc = "Domain Assignment Configuration Register"]
    pub DACFG2: crate::RORegister<u8>,
    #[doc = "Domain Assignment Configuration Register"]
    pub DACFG3: crate::RORegister<u8>,
    #[doc = "Domain Assignment Configuration Register"]
    pub DACFG4: crate::RORegister<u8>,
    _reserved1: [u8; 0xbb],
    #[doc = "TRDC IDAU Control Register"]
    pub TRDC_IDAU_CR: crate::RWRegister<u32>,
    _reserved2: [u8; 0x1c],
    #[doc = "TRDC FLW Control"]
    pub TRDC_FLW_CTL: crate::RWRegister<u32>,
    #[doc = "TRDC FLW Physical Base"]
    pub TRDC_FLW_PBASE: crate::RORegister<u32>,
    #[doc = "TRDC FLW Array Base"]
    pub TRDC_FLW_ABASE: crate::RWRegister<u32>,
    #[doc = "TRDC FLW Block Count"]
    pub TRDC_FLW_BCNT: crate::RWRegister<u32>,
    _reserved3: [u8; 0x0c],
    #[doc = "TRDC Fault Domain ID"]
    pub TRDC_FDID: crate::RWRegister<u32>,
    #[doc = "TRDC Domain Error Location Register"]
    pub TRDC_DERRLOC: [crate::RORegister<u32>; 16usize],
    _reserved4: [u8; 0x05c0],
    #[doc = "DAC Master Domain Assignment Register"]
    pub MDA_W0_0_DFMT1: crate::RWRegister<u32>,
    _reserved5: [u8; 0x1c],
    #[doc = "DAC Master Domain Assignment Register"]
    pub MDA_W0_1_DFMT1: crate::RWRegister<u32>,
    _reserved6: [u8; 0x1c],
    #[doc = "DAC Master Domain Assignment Register"]
    pub MDA_W0_2_DFMT1: crate::RWRegister<u32>,
    _reserved7: [u8; 0x1c],
    #[doc = "DAC Master Domain Assignment Register"]
    pub MDA_W0_3_DFMT1: crate::RWRegister<u32>,
    _reserved8: [u8; 0x1c],
    #[doc = "DAC Master Domain Assignment Register"]
    pub MDA_W0_4_DFMT1: crate::RWRegister<u32>,
}
#[doc = "TRDC Register"]
pub mod TRDC_CR {
    #[doc = "Global Valid for Domain Assignment Controllers"]
    pub mod GVLDM {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "TRDC DACs are disabled."]
            pub const DISABLED: u32 = 0;
            #[doc = "TRDC DACs are enabled."]
            pub const ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Hardware Revision Level"]
    pub mod HRL {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Global Valid for Memory Block Checkers"]
    pub mod GVLDB {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "TRDC MBCs are disabled."]
            pub const DISABLED: u32 = 0;
            #[doc = "TRDC MBCs are enabled."]
            pub const ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Global Valid for Memory Region Checkers"]
    pub mod GVLDR {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "TRDC MRCs are disabled."]
            pub const DISABLED: u32 = 0;
            #[doc = "TRDC MRCs are enabled."]
            pub const ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Lock Status"]
    pub mod LK1 {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The CR can be written by any secure privileged write."]
            pub const INVALID: u32 = 0;
            #[doc = "The CR is locked (read-only) until the next reset."]
            pub const VALID: u32 = 0x01;
        }
    }
}
#[doc = "TRDC Hardware Configuration Register 0"]
pub mod TRDC_HWCFG0 {
    #[doc = "Number of domains"]
    pub mod NDID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Number of bus masters"]
    pub mod NMSTR {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Number of MBCs"]
    pub mod NMBC {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Number of MRCs"]
    pub mod NMRC {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Module ID"]
    pub mod MID {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "TRDC Hardware Configuration Register 1"]
pub mod TRDC_HWCFG1 {
    #[doc = "Domain identifier number"]
    pub mod DID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "TRDC Hardware Configuration Register 2"]
pub mod TRDC_HWCFG2 {
    #[doc = "Process identifier present"]
    pub mod PIDPN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "TRDC Hardware Configuration Register 3"]
pub mod TRDC_HWCFG3 {
    #[doc = "Process identifier present"]
    pub mod PIDPN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Domain Assignment Configuration Register"]
pub mod DACFG0 {
    #[doc = "Number of master domain assignment registers for bus master m"]
    pub mod NMDAR {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Non-CPU Master"]
    pub mod NCM {
        pub const offset: u8 = 7;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Bus master is a processor."]
            pub const CPU: u8 = 0;
            #[doc = "Bus master is a non-processor."]
            pub const NON_CPU: u8 = 0x01;
        }
    }
}
#[doc = "Domain Assignment Configuration Register"]
pub mod DACFG1 {
    #[doc = "Number of master domain assignment registers for bus master m"]
    pub mod NMDAR {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Non-CPU Master"]
    pub mod NCM {
        pub const offset: u8 = 7;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Bus master is a processor."]
            pub const CPU: u8 = 0;
            #[doc = "Bus master is a non-processor."]
            pub const NON_CPU: u8 = 0x01;
        }
    }
}
#[doc = "Domain Assignment Configuration Register"]
pub mod DACFG2 {
    #[doc = "Number of master domain assignment registers for bus master m"]
    pub mod NMDAR {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Non-CPU Master"]
    pub mod NCM {
        pub const offset: u8 = 7;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Bus master is a processor."]
            pub const CPU: u8 = 0;
            #[doc = "Bus master is a non-processor."]
            pub const NON_CPU: u8 = 0x01;
        }
    }
}
#[doc = "Domain Assignment Configuration Register"]
pub mod DACFG3 {
    #[doc = "Number of master domain assignment registers for bus master m"]
    pub mod NMDAR {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Non-CPU Master"]
    pub mod NCM {
        pub const offset: u8 = 7;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Bus master is a processor."]
            pub const CPU: u8 = 0;
            #[doc = "Bus master is a non-processor."]
            pub const NON_CPU: u8 = 0x01;
        }
    }
}
#[doc = "Domain Assignment Configuration Register"]
pub mod DACFG4 {
    #[doc = "Number of master domain assignment registers for bus master m"]
    pub mod NMDAR {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Non-CPU Master"]
    pub mod NCM {
        pub const offset: u8 = 7;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Bus master is a processor."]
            pub const CPU: u8 = 0;
            #[doc = "Bus master is a non-processor."]
            pub const NON_CPU: u8 = 0x01;
        }
    }
}
#[doc = "TRDC IDAU Control Register"]
pub mod TRDC_IDAU_CR {
    #[doc = "Valid"]
    pub mod VLD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Configure Security Extension"]
    pub mod CFGSECEXT {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Armv8M Security Extension is disabled"]
            pub const DISABLED: u32 = 0;
            #[doc = "Armv8-M Security Extension is enabled"]
            pub const ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Secure Memory Protection Unit Disabled"]
    pub mod MPUSDIS {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure MPU is enabled"]
            pub const DISABLED: u32 = 0;
            #[doc = "Secure MPU is disabled"]
            pub const ENABLED: u32 = 0x01;
        }
    }
    #[doc = "NonSecure Memory Protection Unit Disabled"]
    pub mod MPUNSDIS {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Nonsecure MPU is enabled"]
            pub const DISABLED: u32 = 0;
            #[doc = "Nonsecure MPU is disabled"]
            pub const ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Security Attribution Unit Disable"]
    pub mod SAUDIS {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "SAU is enabled"]
            pub const DISABLED: u32 = 0;
            #[doc = "SAU is disabled"]
            pub const ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Lock Secure VTOR, Application interrupt and Reset Control Registers"]
    pub mod LKSVTAIRCR {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Unlock these registers"]
            pub const UNLOCK: u32 = 0;
            #[doc = "Disable writes to the VTOR_S, AIRCR\\[PRIS\\], and AIRCR\\[BFHFNMINS\\] registers"]
            pub const LOCK: u32 = 0x01;
        }
    }
    #[doc = "Lock Nonsecure Vector Table Offset Register"]
    pub mod LKNSVTOR {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Unlock this register"]
            pub const UNLOCK: u32 = 0;
            #[doc = "Disable writes to the VTOR_NS register"]
            pub const LOCK: u32 = 0x01;
        }
    }
    #[doc = "Lock Secure MPU"]
    pub mod LKSMPU {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Unlock these registers"]
            pub const UNLOCK: u32 = 0;
            #[doc = "Disable writes to the MPU_CTRL, MPU_RNR, MPU_RBAR, MPU_RLAR, MPU_RBAR_An and MPU_RLAR_An from software or from a debug agent connected to the processor in Secure state"]
            pub const LOCK: u32 = 0x01;
        }
    }
    #[doc = "Lock Nonsecure MPU"]
    pub mod LKNSMPU {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Unlock these registers"]
            pub const UNLOCK: u32 = 0;
            #[doc = "Disable writes to the MPU_CTRL_NS, MPU_RNR_NS, MPU_RBAR_NS, MPU_RLAR_NS, MPU_RBAR_A_NSn and MPU_RLAR_A_NSn from software or from a debug agent connected to the processor"]
            pub const LOCK: u32 = 0x01;
        }
    }
    #[doc = "Lock SAU"]
    pub mod LKSAU {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Unlock these registers"]
            pub const UNLOCK: u32 = 0;
            #[doc = "Disable writes to the SAU_CTRL, SAU_RNR, SAU_RBAR and SAU_RLAR registers from software or from a debug agent connected to the processor"]
            pub const LOCK: u32 = 0x01;
        }
    }
    #[doc = "Processor current security"]
    pub mod PCURRNS {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processor is in Secure state"]
            pub const SECURE: u32 = 0;
            #[doc = "Processor is in Nonsecure state"]
            pub const NONSECURE: u32 = 0x01;
        }
    }
}
#[doc = "TRDC FLW Control"]
pub mod TRDC_FLW_CTL {
    #[doc = "Lock bit"]
    pub mod LK {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "FLW registers may be modified."]
            pub const UNLOCKED: u32 = 0;
            #[doc = "FLW registers are locked until the next reset."]
            pub const LOCKED: u32 = 0x01;
        }
    }
    #[doc = "Valid bit"]
    pub mod V {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "FLW function is disabled."]
            pub const INVALID: u32 = 0;
            #[doc = "FLW function is enabled."]
            pub const VALID: u32 = 0x01;
        }
    }
}
#[doc = "TRDC FLW Physical Base"]
pub mod TRDC_FLW_PBASE {
    #[doc = "Physical base address"]
    pub mod PBASE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "TRDC FLW Array Base"]
pub mod TRDC_FLW_ABASE {
    #[doc = "Array base address low"]
    pub mod ABASE_L {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x7f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Array base address high"]
    pub mod ABASE_H {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x03ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "TRDC FLW Block Count"]
pub mod TRDC_FLW_BCNT {
    #[doc = "Block Count"]
    pub mod BCNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x7fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "TRDC Fault Domain ID"]
pub mod TRDC_FDID {
    #[doc = "Domain ID of Faulted Access"]
    pub mod FDID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "TRDC Domain Error Location Register"]
pub mod TRDC_DERRLOC {
    #[doc = "MBC instance"]
    pub mod MBCINST {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "MRC instance"]
    pub mod MRCINST {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DAC Master Domain Assignment Register"]
pub mod MDA_W0_0_DFMT1 {
    #[doc = "Domain identifier"]
    pub mod DID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Privileged attribute"]
    pub mod PA {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Force the bus attribute for this master to user."]
            pub const ZERO: u32 = 0;
            #[doc = "Force the bus attribute for this master to privileged."]
            pub const ONE: u32 = 0x01;
            #[doc = "Use the bus master's privileged/user attribute directly."]
            pub const TWO: u32 = 0x02;
            #[doc = "Use the bus master's privileged/user attribute directly."]
            pub const THREE: u32 = 0x03;
        }
    }
    #[doc = "Secure attribute"]
    pub mod SA {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Force the bus attribute for this master to secure."]
            pub const ZERO: u32 = 0;
            #[doc = "Force the bus attribute for this master to nonsecure."]
            pub const ONE: u32 = 0x01;
            #[doc = "Use the bus master's secure/nonsecure attribute directly."]
            pub const TWO: u32 = 0x02;
            #[doc = "Use the bus master's secure/nonsecure attribute directly."]
            pub const THREE: u32 = 0x03;
        }
    }
    #[doc = "DID Bypass"]
    pub mod DIDB {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Use MDAn\\[3:0\\] as the domain identifier."]
            pub const REG: u32 = 0;
            #[doc = "Use the DID input as the domain identifier."]
            pub const INPUT: u32 = 0x01;
        }
    }
    #[doc = "Domain format"]
    pub mod DFMT {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processor-core domain assignment"]
            pub const CPU: u32 = 0;
            #[doc = "Non-processor domain assignment"]
            pub const NONCPU: u32 = 0x01;
        }
    }
    #[doc = "1-bit Lock"]
    pub mod LK1 {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Register can be written by any secure privileged write."]
            pub const UNLOCK: u32 = 0;
            #[doc = "Register is locked (read-only) until the next reset."]
            pub const LOCKED: u32 = 0x01;
        }
    }
    #[doc = "Valid"]
    pub mod VLD {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The Wr domain assignment is invalid."]
            pub const INVALID: u32 = 0;
            #[doc = "The Wr domain assignment is valid."]
            pub const VALID: u32 = 0x01;
        }
    }
}
#[doc = "DAC Master Domain Assignment Register"]
pub mod MDA_W0_1_DFMT1 {
    #[doc = "Domain identifier"]
    pub mod DID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Privileged attribute"]
    pub mod PA {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Force the bus attribute for this master to user."]
            pub const ZERO: u32 = 0;
            #[doc = "Force the bus attribute for this master to privileged."]
            pub const ONE: u32 = 0x01;
            #[doc = "Use the bus master's privileged/user attribute directly."]
            pub const TWO: u32 = 0x02;
            #[doc = "Use the bus master's privileged/user attribute directly."]
            pub const THREE: u32 = 0x03;
        }
    }
    #[doc = "Secure attribute"]
    pub mod SA {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Force the bus attribute for this master to secure."]
            pub const ZERO: u32 = 0;
            #[doc = "Force the bus attribute for this master to nonsecure."]
            pub const ONE: u32 = 0x01;
            #[doc = "Use the bus master's secure/nonsecure attribute directly."]
            pub const TWO: u32 = 0x02;
            #[doc = "Use the bus master's secure/nonsecure attribute directly."]
            pub const THREE: u32 = 0x03;
        }
    }
    #[doc = "DID Bypass"]
    pub mod DIDB {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Use MDAn\\[3:0\\] as the domain identifier."]
            pub const REG: u32 = 0;
            #[doc = "Use the DID input as the domain identifier."]
            pub const INPUT: u32 = 0x01;
        }
    }
    #[doc = "Domain format"]
    pub mod DFMT {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processor-core domain assignment"]
            pub const CPU: u32 = 0;
            #[doc = "Non-processor domain assignment"]
            pub const NONCPU: u32 = 0x01;
        }
    }
    #[doc = "1-bit Lock"]
    pub mod LK1 {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Register can be written by any secure privileged write."]
            pub const UNLOCK: u32 = 0;
            #[doc = "Register is locked (read-only) until the next reset."]
            pub const LOCKED: u32 = 0x01;
        }
    }
    #[doc = "Valid"]
    pub mod VLD {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The Wr domain assignment is invalid."]
            pub const INVALID: u32 = 0;
            #[doc = "The Wr domain assignment is valid."]
            pub const VALID: u32 = 0x01;
        }
    }
}
#[doc = "DAC Master Domain Assignment Register"]
pub mod MDA_W0_2_DFMT1 {
    #[doc = "Domain identifier"]
    pub mod DID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Privileged attribute"]
    pub mod PA {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Force the bus attribute for this master to user."]
            pub const ZERO: u32 = 0;
            #[doc = "Force the bus attribute for this master to privileged."]
            pub const ONE: u32 = 0x01;
            #[doc = "Use the bus master's privileged/user attribute directly."]
            pub const TWO: u32 = 0x02;
            #[doc = "Use the bus master's privileged/user attribute directly."]
            pub const THREE: u32 = 0x03;
        }
    }
    #[doc = "Secure attribute"]
    pub mod SA {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Force the bus attribute for this master to secure."]
            pub const ZERO: u32 = 0;
            #[doc = "Force the bus attribute for this master to nonsecure."]
            pub const ONE: u32 = 0x01;
            #[doc = "Use the bus master's secure/nonsecure attribute directly."]
            pub const TWO: u32 = 0x02;
            #[doc = "Use the bus master's secure/nonsecure attribute directly."]
            pub const THREE: u32 = 0x03;
        }
    }
    #[doc = "DID Bypass"]
    pub mod DIDB {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Use MDAn\\[3:0\\] as the domain identifier."]
            pub const REG: u32 = 0;
            #[doc = "Use the DID input as the domain identifier."]
            pub const INPUT: u32 = 0x01;
        }
    }
    #[doc = "Domain format"]
    pub mod DFMT {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processor-core domain assignment"]
            pub const CPU: u32 = 0;
            #[doc = "Non-processor domain assignment"]
            pub const NONCPU: u32 = 0x01;
        }
    }
    #[doc = "1-bit Lock"]
    pub mod LK1 {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Register can be written by any secure privileged write."]
            pub const UNLOCK: u32 = 0;
            #[doc = "Register is locked (read-only) until the next reset."]
            pub const LOCKED: u32 = 0x01;
        }
    }
    #[doc = "Valid"]
    pub mod VLD {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The Wr domain assignment is invalid."]
            pub const INVALID: u32 = 0;
            #[doc = "The Wr domain assignment is valid."]
            pub const VALID: u32 = 0x01;
        }
    }
}
#[doc = "DAC Master Domain Assignment Register"]
pub mod MDA_W0_3_DFMT1 {
    #[doc = "Domain identifier"]
    pub mod DID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Privileged attribute"]
    pub mod PA {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Force the bus attribute for this master to user."]
            pub const ZERO: u32 = 0;
            #[doc = "Force the bus attribute for this master to privileged."]
            pub const ONE: u32 = 0x01;
            #[doc = "Use the bus master's privileged/user attribute directly."]
            pub const TWO: u32 = 0x02;
            #[doc = "Use the bus master's privileged/user attribute directly."]
            pub const THREE: u32 = 0x03;
        }
    }
    #[doc = "Secure attribute"]
    pub mod SA {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Force the bus attribute for this master to secure."]
            pub const ZERO: u32 = 0;
            #[doc = "Force the bus attribute for this master to nonsecure."]
            pub const ONE: u32 = 0x01;
            #[doc = "Use the bus master's secure/nonsecure attribute directly."]
            pub const TWO: u32 = 0x02;
            #[doc = "Use the bus master's secure/nonsecure attribute directly."]
            pub const THREE: u32 = 0x03;
        }
    }
    #[doc = "DID Bypass"]
    pub mod DIDB {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Use MDAn\\[3:0\\] as the domain identifier."]
            pub const REG: u32 = 0;
            #[doc = "Use the DID input as the domain identifier."]
            pub const INPUT: u32 = 0x01;
        }
    }
    #[doc = "Domain format"]
    pub mod DFMT {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processor-core domain assignment"]
            pub const CPU: u32 = 0;
            #[doc = "Non-processor domain assignment"]
            pub const NONCPU: u32 = 0x01;
        }
    }
    #[doc = "1-bit Lock"]
    pub mod LK1 {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Register can be written by any secure privileged write."]
            pub const UNLOCK: u32 = 0;
            #[doc = "Register is locked (read-only) until the next reset."]
            pub const LOCKED: u32 = 0x01;
        }
    }
    #[doc = "Valid"]
    pub mod VLD {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The Wr domain assignment is invalid."]
            pub const INVALID: u32 = 0;
            #[doc = "The Wr domain assignment is valid."]
            pub const VALID: u32 = 0x01;
        }
    }
}
#[doc = "DAC Master Domain Assignment Register"]
pub mod MDA_W0_4_DFMT1 {
    #[doc = "Domain identifier"]
    pub mod DID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Privileged attribute"]
    pub mod PA {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Force the bus attribute for this master to user."]
            pub const ZERO: u32 = 0;
            #[doc = "Force the bus attribute for this master to privileged."]
            pub const ONE: u32 = 0x01;
            #[doc = "Use the bus master's privileged/user attribute directly."]
            pub const TWO: u32 = 0x02;
            #[doc = "Use the bus master's privileged/user attribute directly."]
            pub const THREE: u32 = 0x03;
        }
    }
    #[doc = "Secure attribute"]
    pub mod SA {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Force the bus attribute for this master to secure."]
            pub const ZERO: u32 = 0;
            #[doc = "Force the bus attribute for this master to nonsecure."]
            pub const ONE: u32 = 0x01;
            #[doc = "Use the bus master's secure/nonsecure attribute directly."]
            pub const TWO: u32 = 0x02;
            #[doc = "Use the bus master's secure/nonsecure attribute directly."]
            pub const THREE: u32 = 0x03;
        }
    }
    #[doc = "DID Bypass"]
    pub mod DIDB {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Use MDAn\\[3:0\\] as the domain identifier."]
            pub const REG: u32 = 0;
            #[doc = "Use the DID input as the domain identifier."]
            pub const INPUT: u32 = 0x01;
        }
    }
    #[doc = "Domain format"]
    pub mod DFMT {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processor-core domain assignment"]
            pub const CPU: u32 = 0;
            #[doc = "Non-processor domain assignment"]
            pub const NONCPU: u32 = 0x01;
        }
    }
    #[doc = "1-bit Lock"]
    pub mod LK1 {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Register can be written by any secure privileged write."]
            pub const UNLOCK: u32 = 0;
            #[doc = "Register is locked (read-only) until the next reset."]
            pub const LOCKED: u32 = 0x01;
        }
    }
    #[doc = "Valid"]
    pub mod VLD {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The Wr domain assignment is invalid."]
            pub const INVALID: u32 = 0;
            #[doc = "The Wr domain assignment is valid."]
            pub const VALID: u32 = 0x01;
        }
    }
}
