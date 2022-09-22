#[doc = "Core Platform Miscellaneous Control Module"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "SoC-defined platform revision"]
    pub PLREV: crate::RORegister<u16>,
    #[doc = "Processor core type"]
    pub PCT: crate::RORegister<u16>,
    #[doc = "Memory configuration"]
    pub MEMCFG: crate::RORegister<u32>,
    #[doc = "Crossbar Switch (AXBS) Slave Configuration"]
    pub PLASC: crate::RORegister<u16>,
    #[doc = "Crossbar Switch (AXBS) Master Configuration"]
    pub PLAMC: crate::RORegister<u16>,
    #[doc = "Control Register"]
    pub CR: crate::RWRegister<u32>,
    #[doc = "Interrupt Status and Control Register"]
    pub ISCR: crate::RWRegister<u32>,
    _reserved0: [u8; 0x0c],
    #[doc = "Fault address register"]
    pub FADR: crate::RORegister<u32>,
    #[doc = "Fault attributes register"]
    pub FATR: crate::RORegister<u32>,
    #[doc = "Fault data register"]
    pub FDR: crate::RORegister<u32>,
    _reserved1: [u8; 0x03d4],
    #[doc = "Local Memory Descriptor Register"]
    pub LMDR: [crate::RWRegister<u32>; 4usize],
    _reserved2: [u8; 0x70],
    #[doc = "LMEM Parity & ECC Control Register"]
    pub LMPECR: crate::RWRegister<u32>,
    _reserved3: [u8; 0x04],
    #[doc = "LMEM Parity & ECC Interrupt Register"]
    pub LMPEIR: crate::RWRegister<u32>,
    _reserved4: [u8; 0x04],
    #[doc = "LMEM Fault Address Register"]
    pub LMFAR: crate::RORegister<u32>,
    #[doc = "LMEM Fault Attribute Register"]
    pub LMFATR: crate::RWRegister<u32>,
    _reserved5: [u8; 0x08],
    #[doc = "LMEM Fault Data High Register"]
    pub LMFDHR: crate::RORegister<u32>,
    #[doc = "LMEM Fault Data Low Register"]
    pub LMFDLR: crate::RORegister<u32>,
}
#[doc = "SoC-defined platform revision"]
pub mod PLREV {
    #[doc = "The PLREV\\[15:0\\] field is specified by an platform input signal to define a software-visible revision number."]
    pub mod PLREV {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Processor core type"]
pub mod PCT {
    #[doc = "This MCM design supports the ARM Cortex M4 core. The following value identifies this core complex."]
    pub mod PCT {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ARM Cortex M4"]
            pub const PCT_44096: u16 = 0xac40;
        }
    }
}
#[doc = "Memory configuration"]
pub mod MEMCFG {
    #[doc = "TCRAMU size"]
    pub mod TCRAMUSZ {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TCRAML size"]
    pub mod TCRAMLSZ {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Crossbar Switch (AXBS) Slave Configuration"]
pub mod PLASC {
    #[doc = "Each bit in the ASC field indicates whether there is a corresponding connection to the crossbar switch's slave input port."]
    pub mod ASC {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "A bus slave connection to AXBS input port n is absent"]
            pub const ASC_0: u16 = 0;
            #[doc = "A bus slave connection to AXBS input port n is present"]
            pub const ASC_1: u16 = 0x01;
        }
    }
}
#[doc = "Crossbar Switch (AXBS) Master Configuration"]
pub mod PLAMC {
    #[doc = "Each bit in the AMC field indicates whether there is a corresponding connection to the AXBS master input port."]
    pub mod AMC {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "A bus master connection to AXBS input port n is absent"]
            pub const AMC_0: u16 = 0;
            #[doc = "A bus master connection to AXBS input port n is present"]
            pub const AMC_1: u16 = 0x01;
        }
    }
}
#[doc = "Control Register"]
pub mod CR {
    #[doc = "Status bits"]
    pub mod STATUS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Crossbar round-robin arbitration enable"]
    pub mod CBRR {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Fixed-priority arbitration"]
            pub const CBRR_0: u32 = 0;
            #[doc = "Round-robin arbitration"]
            pub const CBRR_1: u32 = 0x01;
        }
    }
    #[doc = "System TCM arbitration priority"]
    pub mod STCMAP {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Round robin"]
            pub const STCMAP_0: u32 = 0;
            #[doc = "Special round robin (favors TCM backoor accesses over the processor)"]
            pub const STCMAP_1: u32 = 0x01;
            #[doc = "Fixed priority. Processor has highest, backdoor has lowest"]
            pub const STCMAP_2: u32 = 0x02;
            #[doc = "Fixed priority. Backdoor has highest, processor has lowest"]
            pub const STCMAP_3: u32 = 0x03;
        }
    }
    #[doc = "System TCM write protect"]
    pub mod STCMWP {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Code TCM arbitration priority"]
    pub mod CTCMAP {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Round robin"]
            pub const CTCMAP_0: u32 = 0;
            #[doc = "Special round robin (favors TCM backoor accesses over the processor)"]
            pub const CTCMAP_1: u32 = 0x01;
            #[doc = "Fixed priority. Processor has highest, backdoor has lowest"]
            pub const CTCMAP_2: u32 = 0x02;
            #[doc = "Fixed priority. Backdoor has highest, processor has lowest"]
            pub const CTCMAP_3: u32 = 0x03;
        }
    }
    #[doc = "Code TCM Write Protect"]
    pub mod CTCMWP {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Interrupt Status and Control Register"]
pub mod ISCR {
    #[doc = "Cache write buffer error status"]
    pub mod CWBER {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No error"]
            pub const CWBER_0: u32 = 0;
            #[doc = "Error occurred"]
            pub const CWBER_1: u32 = 0x01;
        }
    }
    #[doc = "FPU invalid operation interrupt status"]
    pub mod FIOC {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No interrupt"]
            pub const FIOC_0: u32 = 0;
            #[doc = "Interrupt occurred"]
            pub const FIOC_1: u32 = 0x01;
        }
    }
    #[doc = "FPU divide-by-zero interrupt status"]
    pub mod FDZC {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No interrupt"]
            pub const FDZC_0: u32 = 0;
            #[doc = "Interrupt occurred"]
            pub const FDZC_1: u32 = 0x01;
        }
    }
    #[doc = "FPU overflow interrupt status"]
    pub mod FOFC {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No interrupt"]
            pub const FOFC_0: u32 = 0;
            #[doc = "Interrupt occurred"]
            pub const FOFC_1: u32 = 0x01;
        }
    }
    #[doc = "FPU underflow interrupt status"]
    pub mod FUFC {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No interrupt"]
            pub const FUFC_0: u32 = 0;
            #[doc = "Interrupt occurred"]
            pub const FUFC_1: u32 = 0x01;
        }
    }
    #[doc = "FPU inexact interrupt status"]
    pub mod FIXC {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No interrupt"]
            pub const FIXC_0: u32 = 0;
            #[doc = "Interrupt occurred"]
            pub const FIXC_1: u32 = 0x01;
        }
    }
    #[doc = "FPU input denormal interrupt status"]
    pub mod FIDC {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No interrupt"]
            pub const FIDC_0: u32 = 0;
            #[doc = "Interrupt occurred"]
            pub const FIDC_1: u32 = 0x01;
        }
    }
    #[doc = "Cache write buffer error enable"]
    pub mod CWBEE {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable error interrupt"]
            pub const CWBEE_0: u32 = 0;
            #[doc = "Enable error interrupt"]
            pub const CWBEE_1: u32 = 0x01;
        }
    }
    #[doc = "FPU invalid operation interrupt enable"]
    pub mod FIOCE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable interrupt"]
            pub const FIOCE_0: u32 = 0;
            #[doc = "Enable interrupt"]
            pub const FIOCE_1: u32 = 0x01;
        }
    }
    #[doc = "FPU divide-by-zero interrupt enable"]
    pub mod FDZCE {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable interrupt"]
            pub const FDZCE_0: u32 = 0;
            #[doc = "Enable interrupt"]
            pub const FDZCE_1: u32 = 0x01;
        }
    }
    #[doc = "FPU overflow interrupt enable"]
    pub mod FOFCE {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable interrupt"]
            pub const FOFCE_0: u32 = 0;
            #[doc = "Enable interrupt"]
            pub const FOFCE_1: u32 = 0x01;
        }
    }
    #[doc = "FPU underflow interrupt enable"]
    pub mod FUFCE {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable interrupt"]
            pub const FUFCE_0: u32 = 0;
            #[doc = "Enable interrupt"]
            pub const FUFCE_1: u32 = 0x01;
        }
    }
    #[doc = "FPU inexact interrupt enable"]
    pub mod FIXCE {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable interrupt"]
            pub const FIXCE_0: u32 = 0;
            #[doc = "Enable interrupt"]
            pub const FIXCE_1: u32 = 0x01;
        }
    }
    #[doc = "FPU input denormal interrupt enable"]
    pub mod FIDCE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable interrupt"]
            pub const FIDCE_0: u32 = 0;
            #[doc = "Enable interrupt"]
            pub const FIDCE_1: u32 = 0x01;
        }
    }
}
#[doc = "Fault address register"]
pub mod FADR {
    #[doc = "Fault address"]
    pub mod ADDRESS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Fault attributes register"]
pub mod FATR {
    #[doc = "Bus error access type"]
    pub mod BEDA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Instruction"]
            pub const BEDA_0: u32 = 0;
            #[doc = "Data"]
            pub const BEDA_1: u32 = 0x01;
        }
    }
    #[doc = "Bus error privilege level"]
    pub mod BEMD {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "User mode"]
            pub const BEMD_0: u32 = 0;
            #[doc = "Supervisor/privileged mode"]
            pub const BEMD_1: u32 = 0x01;
        }
    }
    #[doc = "Bus error size"]
    pub mod BESZ {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "8-bit access"]
            pub const BESZ_0: u32 = 0;
            #[doc = "16-bit access"]
            pub const BESZ_1: u32 = 0x01;
            #[doc = "32-bit access"]
            pub const BESZ_2: u32 = 0x02;
        }
    }
    #[doc = "Bus error write"]
    pub mod BEWT {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Read access"]
            pub const BEWT_0: u32 = 0;
            #[doc = "Write access"]
            pub const BEWT_1: u32 = 0x01;
        }
    }
    #[doc = "Bus error master number"]
    pub mod BEMN {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Bus error overrun"]
    pub mod BEOVR {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No bus error overrun"]
            pub const BEOVR_0: u32 = 0;
            #[doc = "Bus error overrun occurred. The FADR and FDR registers and the other FATR bits are not updated to reflect this new bus error."]
            pub const BEOVR_1: u32 = 0x01;
        }
    }
}
#[doc = "Fault data register"]
pub mod FDR {
    #[doc = "Fault data"]
    pub mod DATA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Local Memory Descriptor Register"]
pub mod LMDR {
    #[doc = "Control Field 0"]
    pub mod CF0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Control Field 1 - for Cache Parity control functions"]
    pub mod CF1 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Memory Type"]
    pub mod MT {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "code TCM"]
            pub const MT_0: u32 = 0;
            #[doc = "system TCM"]
            pub const MT_1: u32 = 0x01;
            #[doc = "PC Cache"]
            pub const MT_2: u32 = 0x02;
            #[doc = "PS Cache"]
            pub const MT_3: u32 = 0x03;
        }
    }
    #[doc = "Read-Only"]
    pub mod RO {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Writes to the LMDRn\\[7:0\\] are allowed."]
            pub const RO_0: u32 = 0;
            #[doc = "Writes to the LMDRn\\[7:0\\] are ignored."]
            pub const RO_1: u32 = 0x01;
        }
    }
    #[doc = "LMEM Data Path Width. This read-only field defines the width of the local memory."]
    pub mod DPW {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "LMEMn 32-bits wide"]
            pub const DPW_2: u32 = 0x02;
            #[doc = "LMEMn 64-bits wide"]
            pub const DPW_3: u32 = 0x03;
        }
    }
    #[doc = "Level 1 Cache Ways"]
    pub mod WY {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No Cache"]
            pub const WY_0: u32 = 0;
            #[doc = "2-Way Set Associative"]
            pub const WY_2: u32 = 0x02;
            #[doc = "4-Way Set Associative"]
            pub const WY_4: u32 = 0x04;
        }
    }
    #[doc = "LMEM Size"]
    pub mod LMSZ {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "no LMEMn (0 KB)"]
            pub const LMSZ_0: u32 = 0;
            #[doc = "1 KB LMEMn"]
            pub const LMSZ_1: u32 = 0x01;
            #[doc = "2 KB LMEMn"]
            pub const LMSZ_2: u32 = 0x02;
            #[doc = "4 KB LMEMn"]
            pub const LMSZ_3: u32 = 0x03;
            #[doc = "8 KB LMEMn"]
            pub const LMSZ_4: u32 = 0x04;
            #[doc = "16 KB LMEMn"]
            pub const LMSZ_5: u32 = 0x05;
            #[doc = "32 KB LMEMn"]
            pub const LMSZ_6: u32 = 0x06;
            #[doc = "64 KB LMEMn"]
            pub const LMSZ_7: u32 = 0x07;
            #[doc = "128 KB LMEMn"]
            pub const LMSZ_8: u32 = 0x08;
            #[doc = "256 KB LMEMn"]
            pub const LMSZ_9: u32 = 0x09;
            #[doc = "512 KB LMEMn"]
            pub const LMSZ_10: u32 = 0x0a;
            #[doc = "1024 KB LMEMn"]
            pub const LMSZ_11: u32 = 0x0b;
            #[doc = "2048 KB LMEMn"]
            pub const LMSZ_12: u32 = 0x0c;
            #[doc = "4096 KB LMEMn"]
            pub const LMSZ_13: u32 = 0x0d;
            #[doc = "8192 KB LMEMn"]
            pub const LMSZ_14: u32 = 0x0e;
            #[doc = "16384 KB LMEMn"]
            pub const LMSZ_15: u32 = 0x0f;
        }
    }
    #[doc = "LMEM Size \"Hole\""]
    pub mod LMSZH {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "LMEMn is a power-of-2 capacity."]
            pub const LMSZH_0: u32 = 0;
            #[doc = "LMEMn is not a power-of-2, with a capacity is 0.75 * LMSZ."]
            pub const LMSZH_1: u32 = 0x01;
        }
    }
    #[doc = "Local memory Valid bit. This read-only field defines the validity (presence) of the local memory."]
    pub mod V {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "LMEMn is not present."]
            pub const V_0: u32 = 0;
            #[doc = "LMEMn is present."]
            pub const V_1: u32 = 0x01;
        }
    }
}
#[doc = "LMEM Parity & ECC Control Register"]
pub mod LMPECR {
    #[doc = "Enable RAM ECC Non-correctable Reporting"]
    pub mod ERNCR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "reporting enabled"]
            pub const ERNCR_0: u32 = 0;
            #[doc = "reporting disabled"]
            pub const ERNCR_1: u32 = 0x01;
        }
    }
    #[doc = "Enable RAM Non-correctable ECC Interrupt"]
    pub mod ERNCI {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt is disabled"]
            pub const ERNCI_0: u32 = 0;
            #[doc = "Interrupt is enabled"]
            pub const ERNCI_1: u32 = 0x01;
        }
    }
    #[doc = "Enable RAM ECC 1-bit Reporting"]
    pub mod ER1BR {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "reporting enabled"]
            pub const ER1BR_0: u32 = 0;
            #[doc = "reporting disabled"]
            pub const ER1BR_1: u32 = 0x01;
        }
    }
    #[doc = "Enable RAM ECC 1-bit Interrupt"]
    pub mod ER1BI {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt is disabled"]
            pub const ER1BI_0: u32 = 0;
            #[doc = "Interrupt is enabled"]
            pub const ER1BI_1: u32 = 0x01;
        }
    }
    #[doc = "Enable Cache Parity Reporting"]
    pub mod ECPR {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "reporting enabled"]
            pub const ECPR_0: u32 = 0;
            #[doc = "reporting disabled"]
            pub const ECPR_1: u32 = 0x01;
        }
    }
    #[doc = "Enable Cache Parity IRQ"]
    pub mod ECPI {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "enabled"]
            pub const ECPI_0: u32 = 0;
            #[doc = "disabled"]
            pub const ECPI_1: u32 = 0x01;
        }
    }
}
#[doc = "LMEM Parity & ECC Interrupt Register"]
pub mod LMPEIR {
    #[doc = "ENCn = ECC Non-correctable Error n"]
    pub mod ENC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "E1Bn = ECC 1-bit Error n"]
    pub mod E1B {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Parity Error"]
    pub mod PE {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Parity or ECC Error Location"]
    pub mod PEELOC {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Valid bit"]
    pub mod V {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "LMEM Fault Address Register"]
pub mod LMFAR {
    #[doc = "ECC Fault Address"]
    pub mod EFADD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "LMEM Fault Attribute Register"]
pub mod LMFATR {
    #[doc = "Parity/ECC Fault Protection FATR\\[3\\] is Cacheable: 0=Non-cacheable, 1=Cacheable FATR\\[2\\] is Bufferable: 0=Non-bufferable, 1=Bufferable FATR\\[1\\] is Mode: 0=User mode, 1=Supervisor mode FATR\\[0\\] is Type: 0=I-Fetch, 1=Data"]
    pub mod PEFPRT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Parity/ECC Fault Master Size 3'b000 = 8-bit access 3'b001 = 16-bit access 3'b010 = 32-bit access 3'b011 = 64-bit access 3'b1xx = Reserved"]
    pub mod PEFSIZE {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Parity/ECC Fault Write"]
    pub mod PEFW {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Parity/ECC Fault Master Number"]
    pub mod PEFMST {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ID of the word which has ECC error"]
    pub mod WORDID {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Overrun"]
    pub mod OVR {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "LMEM Fault Data High Register"]
pub mod LMFDHR {
    #[doc = "Parity or ECC Fault Data High"]
    pub mod PEFDH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "LMEM Fault Data Low Register"]
pub mod LMFDLR {
    #[doc = "Parity or ECC Fault Data Low"]
    pub mod PEFDL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
