#[doc = "CSU registers"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "Config security level register"]
    pub CSL: [crate::RWRegister<u32>; 32usize],
    _reserved0: [u8; 0x0180],
    #[doc = "HP0 register"]
    pub HP0: crate::RWRegister<u32>,
    _reserved1: [u8; 0x14],
    #[doc = "Secure access register"]
    pub SA: crate::RWRegister<u32>,
    _reserved2: [u8; 0x013c],
    #[doc = "HPCONTROL0 register"]
    pub HPCONTROL0: crate::RWRegister<u32>,
}
#[doc = "Config security level register"]
pub mod CSL {
    #[doc = "Secure user read access control for the second slave"]
    pub mod SUR_S2 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The secure user read access is disabled for the second slave."]
            pub const SUR_S2_0: u32 = 0;
            #[doc = "The secure user read access is enabled for the second slave."]
            pub const SUR_S2_1: u32 = 0x01;
        }
    }
    #[doc = "Secure supervisor read access control for the second slave"]
    pub mod SSR_S2 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The secure supervisor read access is disabled for the second slave."]
            pub const SSR_S2_0: u32 = 0;
            #[doc = "The secure supervisor read access is enabled for the second slave."]
            pub const SSR_S2_1: u32 = 0x01;
        }
    }
    #[doc = "Non-secure user read access control for the second slave"]
    pub mod NUR_S2 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The non-secure user read access is disabled for the second slave."]
            pub const NUR_S2_0: u32 = 0;
            #[doc = "The non-secure user read access is enabled for the second slave."]
            pub const NUR_S2_1: u32 = 0x01;
        }
    }
    #[doc = "Non-secure supervisor read access control for the second slave"]
    pub mod NSR_S2 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The non-secure supervisor read access is disabled for the second slave."]
            pub const NSR_S2_0: u32 = 0;
            #[doc = "The non-secure supervisor read access is enabled for the second slave."]
            pub const NSR_S2_1: u32 = 0x01;
        }
    }
    #[doc = "Secure user write access control for the second slave"]
    pub mod SUW_S2 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The secure user write access is disabled for the second slave."]
            pub const SUW_S2_0: u32 = 0;
            #[doc = "The secure user write access is enabled for the second slave."]
            pub const SUW_S2_1: u32 = 0x01;
        }
    }
    #[doc = "Secure supervisor write access control for the second slave"]
    pub mod SSW_S2 {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The secure supervisor write access is disabled for the second slave."]
            pub const SSW_S2_0: u32 = 0;
            #[doc = "The secure supervisor write access is enabled for the second slave."]
            pub const SSW_S2_1: u32 = 0x01;
        }
    }
    #[doc = "Non-secure user write access control for the second slave"]
    pub mod NUW_S2 {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The non-secure user write access is disabled for the second slave."]
            pub const NUW_S2_0: u32 = 0;
            #[doc = "The non-secure user write access is enabled for the second slave."]
            pub const NUW_S2_1: u32 = 0x01;
        }
    }
    #[doc = "Non-secure supervisor write access control for the second slave"]
    pub mod NSW_S2 {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The non-secure supervisor write access is disabled for the second slave."]
            pub const NSW_S2_0: u32 = 0;
            #[doc = "The non-secure supervisor write access is enabled for the second slave."]
            pub const NSW_S2_1: u32 = 0x01;
        }
    }
    #[doc = "The lock bit corresponding to the second slave. It is written by the secure software."]
    pub mod LOCK_S2 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not locked. Bits 7-0 can be written by the software."]
            pub const LOCK_S2_0: u32 = 0;
            #[doc = "Bits 7-0 are locked and cannot be written by the software"]
            pub const LOCK_S2_1: u32 = 0x01;
        }
    }
    #[doc = "Secure user read access control for the first slave"]
    pub mod SUR_S1 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The secure user read access is disabled for the first slave."]
            pub const SUR_S1_0: u32 = 0;
            #[doc = "The secure user read access is enabled for the first slave."]
            pub const SUR_S1_1: u32 = 0x01;
        }
    }
    #[doc = "Secure supervisor read access control for the first slave"]
    pub mod SSR_S1 {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The secure supervisor read access is disabled for the first slave."]
            pub const SSR_S1_0: u32 = 0;
            #[doc = "The secure supervisor read access is enabled for the first slave."]
            pub const SSR_S1_1: u32 = 0x01;
        }
    }
    #[doc = "Non-secure user read access control for the first slave"]
    pub mod NUR_S1 {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The non-secure user read access is disabled for the first slave."]
            pub const NUR_S1_0: u32 = 0;
            #[doc = "The non-secure user read access is enabled for the first slave."]
            pub const NUR_S1_1: u32 = 0x01;
        }
    }
    #[doc = "Non-secure supervisor read access control for the first slave"]
    pub mod NSR_S1 {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The non-secure supervisor read access is disabled for the first slave."]
            pub const NSR_S1_0: u32 = 0;
            #[doc = "The non-secure supervisor read access is enabled for the first slave."]
            pub const NSR_S1_1: u32 = 0x01;
        }
    }
    #[doc = "Secure user write access control for the first slave"]
    pub mod SUW_S1 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The secure user write access is disabled for the first slave."]
            pub const SUW_S1_0: u32 = 0;
            #[doc = "The secure user write access is enabled for the first slave."]
            pub const SUW_S1_1: u32 = 0x01;
        }
    }
    #[doc = "Secure supervisor write access control for the first slave"]
    pub mod SSW_S1 {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The secure supervisor write access is disabled for the first slave."]
            pub const SSW_S1_0: u32 = 0;
            #[doc = "The secure supervisor write access is enabled for the first slave."]
            pub const SSW_S1_1: u32 = 0x01;
        }
    }
    #[doc = "Non-secure user write access control for the first slave"]
    pub mod NUW_S1 {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The non-secure user write access is disabled for the first slave."]
            pub const NUW_S1_0: u32 = 0;
            #[doc = "The non-secure user write access is enabled for the first slave."]
            pub const NUW_S1_1: u32 = 0x01;
        }
    }
    #[doc = "Non-secure supervisor write access control for the first slave"]
    pub mod NSW_S1 {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The non-secure supervisor write access is disabled for the first slave."]
            pub const NSW_S1_0: u32 = 0;
            #[doc = "The non-secure supervisor write access is enabled for the first slave"]
            pub const NSW_S1_1: u32 = 0x01;
        }
    }
    #[doc = "The lock bit corresponding to the first slave. It is written by the secure software."]
    pub mod LOCK_S1 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not locked. The bits 16-23 can be written by the software."]
            pub const LOCK_S1_0: u32 = 0;
            #[doc = "The bits 16-23 are locked and can't be written by the software."]
            pub const LOCK_S1_1: u32 = 0x01;
        }
    }
}
#[doc = "HP0 register"]
pub mod HP0 {
    #[doc = "Determines whether the register value of the corresponding HP field is passed as the hprot\\[1\\] of the eDMA"]
    pub mod HP_DMA {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The hprot1 input signal value is routed to the csu_hprot1 output for the corresponding master."]
            pub const HP_DMA_0: u32 = 0;
            #[doc = "The HP register bit is routed to the csu_hprot1 output for the corresponding master."]
            pub const HP_DMA_1: u32 = 0x01;
        }
    }
    #[doc = "Lock bit set by the TZ software for the eDMA"]
    pub mod L_DMA {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No lock-the adjacent (next lower) bit can be written by the software."]
            pub const L_DMA_0: u32 = 0;
            #[doc = "Lock-the adjacent (next lower) bit can't be written by the software."]
            pub const L_DMA_1: u32 = 0x01;
        }
    }
    #[doc = "Determines whether the register value of the corresponding HP field is passed as the hprot\\[1\\] of the LCDIF"]
    pub mod HP_LCDIF {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The hprot1 input signal value is routed to the csu_hprot1 output for the corresponding master."]
            pub const HP_LCDIF_0: u32 = 0;
            #[doc = "The HP register bit is routed to the csu_hprot1 output for the corresponding master."]
            pub const HP_LCDIF_1: u32 = 0x01;
        }
    }
    #[doc = "Lock bit set by the TZ software for the LCDIF"]
    pub mod L_LCDIF {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No lock-the adjacent (next lower) bit can be written by the software."]
            pub const L_LCDIF_0: u32 = 0;
            #[doc = "Lock-the adjacent (next lower) bit can't be written by the software."]
            pub const L_LCDIF_1: u32 = 0x01;
        }
    }
    #[doc = "Determines whether the register value of the corresponding HP field is passed as the hprot\\[1\\] of the CSI"]
    pub mod HP_CSI {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The hprot1 input signal value is routed to the csu_hprot1 output for the corresponding master."]
            pub const HP_CSI_0: u32 = 0;
            #[doc = "The HP register bit is routed to the csu_hprot1 output for the corresponding master."]
            pub const HP_CSI_1: u32 = 0x01;
        }
    }
    #[doc = "Lock bit set by the TZ software for the CSI"]
    pub mod L_CSI {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No lock-the adjacent (next lower) bit can be written by the software."]
            pub const L_CSI_0: u32 = 0;
            #[doc = "Lock-the adjacent (next lower) bit can't be written by the software."]
            pub const L_CSI_1: u32 = 0x01;
        }
    }
    #[doc = "Determines whether the register value of the corresponding HP field is passed as the hprot\\[1\\] of the PXP"]
    pub mod HP_PXP {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The hprot1 input signal value is routed to the csu_hprot1 output for the corresponding master."]
            pub const HP_PXP_0: u32 = 0;
            #[doc = "The HP register bit is routed to the csu_hprot1 output for the corresponding master."]
            pub const HP_PXP_1: u32 = 0x01;
        }
    }
    #[doc = "Lock bit set by the TZ software for the PXP"]
    pub mod L_PXP {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No lock-the adjacent (next lower) bit can be written by the software."]
            pub const L_PXP_0: u32 = 0;
            #[doc = "Lock-the adjacent (next lower) bit can't be written by the software."]
            pub const L_PXP_1: u32 = 0x01;
        }
    }
    #[doc = "Determines whether the register value of the corresponding HP field is passed as the hprot\\[1\\] of the DCP"]
    pub mod HP_DCP {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The hprot1 input signal value is routed to the csu_hprot1 output for the corresponding master."]
            pub const HP_DCP_0: u32 = 0;
            #[doc = "The HP register bit is routed to the csu_hprot1 output for the corresponding master."]
            pub const HP_DCP_1: u32 = 0x01;
        }
    }
    #[doc = "Lock bit set by the TZ software for the DCP"]
    pub mod L_DCP {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No lock-the adjacent (next lower) bit can be written by the software."]
            pub const L_DCP_0: u32 = 0;
            #[doc = "Lock-the adjacent (next lower) bit cannot be written by the software."]
            pub const L_DCP_1: u32 = 0x01;
        }
    }
    #[doc = "Determines whether the register value of the corresponding HP field is passed as the hprot\\[1\\] of the ENET"]
    pub mod HP_ENET {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The hprot1 input signal value is routed to the csu_hprot1 output for the corresponding master."]
            pub const HP_ENET_0: u32 = 0;
            #[doc = "The HP register bit is routed to the csu_hprot1 output for the corresponding master."]
            pub const HP_ENET_1: u32 = 0x01;
        }
    }
    #[doc = "Lock bit set by the TZ software for the ENET"]
    pub mod L_ENET {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No lock-the adjacent (next lower) bit can be written by the software."]
            pub const L_ENET_0: u32 = 0;
            #[doc = "Lock-the adjacent (next lower) bit can't be written by the software."]
            pub const L_ENET_1: u32 = 0x01;
        }
    }
    #[doc = "Determines whether the register value of the corresponding HP field is passed as the hprot\\[1\\] of the USDHC1"]
    pub mod HP_USDHC1 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The hprot1 input signal value is routed to the csu_hprot1 output for the corresponding master."]
            pub const HP_USDHC1_0: u32 = 0;
            #[doc = "The HP register bit is routed to the csu_hprot1 output for the corresponding master."]
            pub const HP_USDHC1_1: u32 = 0x01;
        }
    }
    #[doc = "Lock bit set by the TZ software for the USDHC1"]
    pub mod L_USDHC1 {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No lock-the adjacent (next lower) bit can be written by the software."]
            pub const L_USDHC1_0: u32 = 0;
            #[doc = "Lock-the adjacent (next lower) bit can't be written by the software."]
            pub const L_USDHC1_1: u32 = 0x01;
        }
    }
    #[doc = "Determines whether the register value of the corresponding HP field is passed as the hprot\\[1\\] of the USDHC2"]
    pub mod HP_USDHC2 {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The hprot1 input signal value is routed to the csu_hprot1 output for the corresponding master."]
            pub const HP_USDHC2_0: u32 = 0;
            #[doc = "The HP register bit is routed to the csu_hprot1 output for the corresponding master."]
            pub const HP_USDHC2_1: u32 = 0x01;
        }
    }
    #[doc = "Lock bit set by the TZ software for the USDHC2"]
    pub mod L_USDHC2 {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No lock-the adjacent (next lower) bit can be written by the software."]
            pub const L_USDHC2_0: u32 = 0;
            #[doc = "Lock-the adjacent (next lower) bit can't be written by the software."]
            pub const L_USDHC2_1: u32 = 0x01;
        }
    }
    #[doc = "Determines whether the register value of the corresponding HP field is passed as the hprot\\[1\\] of the TPSMP"]
    pub mod HP_TPSMP {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The hprot1 input signal value is routed to the csu_hprot1 output for the corresponding master."]
            pub const HP_TPSMP_0: u32 = 0;
            #[doc = "The HP register bit is routed to the csu_hprot1 output for the corresponding master."]
            pub const HP_TPSMP_1: u32 = 0x01;
        }
    }
    #[doc = "Lock bit set by the TZ software for the TPSMP"]
    pub mod L_TPSMP {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No lock-the adjacent (next lower) bit can be written by the software."]
            pub const L_TPSMP_0: u32 = 0;
            #[doc = "Lock-the adjacent (next lower) bit can't be written by the software."]
            pub const L_TPSMP_1: u32 = 0x01;
        }
    }
    #[doc = "Determines whether the register value of the corresponding HP field is passed as the hprot\\[1\\] of the USB"]
    pub mod HP_USB {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The hprot1 input signal value is routed to the csu_hprot1 output for the corresponding master."]
            pub const HP_USB_0: u32 = 0;
            #[doc = "The HP register bit is routed to the csu_hprot1 output for the corresponding master."]
            pub const HP_USB_1: u32 = 0x01;
        }
    }
    #[doc = "Lock bit set by the TZ software for the USB"]
    pub mod L_USB {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No lock-the adjacent (next lower) bit can be written by the software."]
            pub const L_USB_0: u32 = 0;
            #[doc = "Lock-the adjacent (next lower) bit can't be written by the software."]
            pub const L_USB_1: u32 = 0x01;
        }
    }
}
#[doc = "Secure access register"]
pub mod SA {
    #[doc = "Non-secure access policy indicator bit"]
    pub mod NSA_DMA {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure access for the corresponding type-1 master"]
            pub const NSA_DMA_0: u32 = 0;
            #[doc = "Non-secure access for the corresponding type-1 master"]
            pub const NSA_DMA_1: u32 = 0x01;
        }
    }
    #[doc = "Lock bit set by the TZ software for the eDMA"]
    pub mod L_DMA {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No lock-the adjacent (next lower) bit can be written by the software."]
            pub const L_DMA_0: u32 = 0;
            #[doc = "Lock-the adjacent (next lower) bit can't be written by the software."]
            pub const L_DMA_1: u32 = 0x01;
        }
    }
    #[doc = "Non-secure access policy indicator bit"]
    pub mod NSA_LCDIF {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure access for the corresponding type-1 master"]
            pub const NSA_LCDIF_0: u32 = 0;
            #[doc = "Non-secure access for the corresponding type-1 master"]
            pub const NSA_LCDIF_1: u32 = 0x01;
        }
    }
    #[doc = "Lock bit set by the TZ software for the LCDIF"]
    pub mod L_LCDIF {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No lock-the adjacent (next lower) bit can be written by the software."]
            pub const L_LCDIF_0: u32 = 0;
            #[doc = "Lock-the adjacent (next lower) bit can't be written by the software."]
            pub const L_LCDIF_1: u32 = 0x01;
        }
    }
    #[doc = "Non-secure access policy indicator bit"]
    pub mod NSA_CSI {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure access for the corresponding type-1 master"]
            pub const NSA_CSI_0: u32 = 0;
            #[doc = "Non-secure access for the corresponding type-1 master"]
            pub const NSA_CSI_1: u32 = 0x01;
        }
    }
    #[doc = "Lock bit set by the TZ software for the CSI"]
    pub mod L_CSI {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No lock-the adjacent (next lower) bit can be written by the software."]
            pub const L_CSI_0: u32 = 0;
            #[doc = "Lock-the adjacent (next lower) bit can't be written by the software."]
            pub const L_CSI_1: u32 = 0x01;
        }
    }
    #[doc = "Non-Secure Access Policy indicator bit"]
    pub mod NSA_PXP {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure access for the corresponding type-1 master"]
            pub const NSA_PXP_0: u32 = 0;
            #[doc = "Non-secure access for the corresponding type-1 master"]
            pub const NSA_PXP_1: u32 = 0x01;
        }
    }
    #[doc = "Lock bit set by the TZ software for the PXP"]
    pub mod L_PXP {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No lock-the adjacent (next lower) bit can be written by the software."]
            pub const L_PXP_0: u32 = 0;
            #[doc = "Lock-the adjacent (next lower) bit can't be written by the software."]
            pub const L_PXP_1: u32 = 0x01;
        }
    }
    #[doc = "Non-secure access policy indicator bit"]
    pub mod NSA_DCP {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure access for the corresponding type-1 master"]
            pub const NSA_DCP_0: u32 = 0;
            #[doc = "Non-secure access for the corresponding type-1 master"]
            pub const NSA_DCP_1: u32 = 0x01;
        }
    }
    #[doc = "Lock bit set by the TZ software for the DCP"]
    pub mod L_DCP {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No lock-the adjacent (next lower) bit can be written by the software."]
            pub const L_DCP_0: u32 = 0;
            #[doc = "Lock-the adjacent (next lower) bit can't be written by the software."]
            pub const L_DCP_1: u32 = 0x01;
        }
    }
    #[doc = "Non-secure access policy indicator bit"]
    pub mod NSA_ENET {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure access for the corresponding type-1 master"]
            pub const NSA_ENET_0: u32 = 0;
            #[doc = "Non-secure access for the corresponding type-1 master"]
            pub const NSA_ENET_1: u32 = 0x01;
        }
    }
    #[doc = "Lock bit set by the TZ software for the ENET1 and ENET2"]
    pub mod L_ENET {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No lock-the adjacent (next lower) bit can be written by the software."]
            pub const L_ENET_0: u32 = 0;
            #[doc = "Lock-the adjacent (next lower) bit can't be written by the software."]
            pub const L_ENET_1: u32 = 0x01;
        }
    }
    #[doc = "Non-secure access policy indicator bit"]
    pub mod NSA_USDHC1 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure access for the corresponding type-1 master"]
            pub const NSA_USDHC1_0: u32 = 0;
            #[doc = "Non-secure access for the corresponding type-1 master"]
            pub const NSA_USDHC1_1: u32 = 0x01;
        }
    }
    #[doc = "Lock bit set by the TZ software for the USDHC1"]
    pub mod L_USDHC1 {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No lock-the adjacent (next lower) bit can be written by the software."]
            pub const L_USDHC1_0: u32 = 0;
            #[doc = "Lock-the adjacent (next lower) bit can't be written by the software."]
            pub const L_USDHC1_1: u32 = 0x01;
        }
    }
    #[doc = "Non-secure access policy indicator bit"]
    pub mod NSA_USDHC2 {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure access for the corresponding type-1 master"]
            pub const NSA_USDHC2_0: u32 = 0;
            #[doc = "Non-secure access for the corresponding type-1 master"]
            pub const NSA_USDHC2_1: u32 = 0x01;
        }
    }
    #[doc = "Lock bit set by the TZ software for the USDHC2"]
    pub mod L_USDHC2 {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No lock-the adjacent (next lower) bit can be written by the software."]
            pub const L_USDHC2_0: u32 = 0;
            #[doc = "Lock-the adjacent (next lower) bit can't be written by the software."]
            pub const L_USDHC2_1: u32 = 0x01;
        }
    }
    #[doc = "Non-secure access policy indicator bit"]
    pub mod NSA_TPSMP {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure access for the corresponding type-1 master"]
            pub const NSA_TPSMP_0: u32 = 0;
            #[doc = "Non-secure access for the corresponding type-1 master"]
            pub const NSA_TPSMP_1: u32 = 0x01;
        }
    }
    #[doc = "Lock bit set by the TZ software for the TPSMP"]
    pub mod L_TPSMP {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No lock-the adjacent (next lower) bit can be written by the software."]
            pub const L_TPSMP_0: u32 = 0;
            #[doc = "Lock-the adjacent (next lower) bit can't be written by the software."]
            pub const L_TPSMP_1: u32 = 0x01;
        }
    }
    #[doc = "Non-secure access policy indicator bit"]
    pub mod NSA_USB {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure access for the corresponding type-1 master"]
            pub const NSA_USB_0: u32 = 0;
            #[doc = "Non-secure access for the corresponding type-1 master"]
            pub const NSA_USB_1: u32 = 0x01;
        }
    }
    #[doc = "Lock bit set by the TZ software for the USB"]
    pub mod L_USB {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No lock-the adjacent (next lower) bit can be written by the software."]
            pub const L_USB_0: u32 = 0;
            #[doc = "Lock-the adjacent (next lower) bit can't be written by the software."]
            pub const L_USB_1: u32 = 0x01;
        }
    }
}
#[doc = "HPCONTROL0 register"]
pub mod HPCONTROL0 {
    #[doc = "Indicates the privilege/user mode for the eDMA"]
    pub mod HPC_DMA {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "User mode for the corresponding master"]
            pub const HPC_DMA_0: u32 = 0;
            #[doc = "Supervisor mode for the corresponding master"]
            pub const HPC_DMA_1: u32 = 0x01;
        }
    }
    #[doc = "Lock bit set by the TZ software for the eDMA"]
    pub mod L_DMA {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No lock-the adjacent (next lower) bit can be written by the software."]
            pub const L_DMA_0: u32 = 0;
            #[doc = "Lock-the adjacent (next lower) bit can't be written by the software."]
            pub const L_DMA_1: u32 = 0x01;
        }
    }
    #[doc = "Indicates the privilege/user mode for the LCDIF"]
    pub mod HPC_LCDIF {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "User mode for the corresponding master"]
            pub const HPC_LCDIF_0: u32 = 0;
            #[doc = "Supervisor mode for the corresponding master"]
            pub const HPC_LCDIF_1: u32 = 0x01;
        }
    }
    #[doc = "Lock bit set by the TZ software for the LCDIF"]
    pub mod L_LCDIF {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No lock-the adjacent (next lower) bit can be written by the software."]
            pub const L_LCDIF_0: u32 = 0;
            #[doc = "Lock-the adjacent (next lower) bit can't be written by the software."]
            pub const L_LCDIF_1: u32 = 0x01;
        }
    }
    #[doc = "Indicates the privilege/user mode for the CSI"]
    pub mod HPC_CSI {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "User mode for the corresponding master"]
            pub const HPC_CSI_0: u32 = 0;
            #[doc = "Supervisor mode for the corresponding master"]
            pub const HPC_CSI_1: u32 = 0x01;
        }
    }
    #[doc = "Lock bit set by the TZ software for the CSI"]
    pub mod L_CSI {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No lock-the adjacent (next lower) bit can be written by the software."]
            pub const L_CSI_0: u32 = 0;
            #[doc = "Lock-the adjacent (next lower) bit can't be written by the software."]
            pub const L_CSI_1: u32 = 0x01;
        }
    }
    #[doc = "Indicates the privilege/user mode for the PXP"]
    pub mod HPC_PXP {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "User mode for the corresponding master"]
            pub const HPC_PXP_0: u32 = 0;
            #[doc = "Supervisor mode for the corresponding master"]
            pub const HPC_PXP_1: u32 = 0x01;
        }
    }
    #[doc = "Lock bit set by the TZ software for the PXP"]
    pub mod L_PXP {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No lock-the adjacent (next lower) bit can be written by the software."]
            pub const L_PXP_0: u32 = 0;
            #[doc = "Lock-the adjacent (next lower) bit can't be written by the software."]
            pub const L_PXP_1: u32 = 0x01;
        }
    }
    #[doc = "Indicates the privilege/user mode for the DCP"]
    pub mod HPC_DCP {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "User mode for the corresponding master"]
            pub const HPC_DCP_0: u32 = 0;
            #[doc = "Supervisor mode for the corresponding master"]
            pub const HPC_DCP_1: u32 = 0x01;
        }
    }
    #[doc = "Lock bit set by the TZ software for the DCP"]
    pub mod L_DCP {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No lock-the adjacent (next lower) bit can be written by the software."]
            pub const L_DCP_0: u32 = 0;
            #[doc = "Lock-the adjacent (next lower) bit can't be written by the software."]
            pub const L_DCP_1: u32 = 0x01;
        }
    }
    #[doc = "Indicates the privilege/user mode for the ENET"]
    pub mod HPC_ENET {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "User mode for the corresponding master"]
            pub const HPC_ENET_0: u32 = 0;
            #[doc = "Supervisor mode for the corresponding master"]
            pub const HPC_ENET_1: u32 = 0x01;
        }
    }
    #[doc = "Lock bit set by the TZ software for the ENET"]
    pub mod L_ENET {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No lock-the adjacent (next lower) bit can be written by the software."]
            pub const L_ENET_0: u32 = 0;
            #[doc = "Lock-the adjacent (next lower) bit can't be written by the software."]
            pub const L_ENET_1: u32 = 0x01;
        }
    }
    #[doc = "Indicates the privilege/user mode for the USDHC1"]
    pub mod HPC_USDHC1 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "User mode for the corresponding master"]
            pub const HPC_USDHC1_0: u32 = 0;
            #[doc = "Supervisor mode for the corresponding master"]
            pub const HPC_USDHC1_1: u32 = 0x01;
        }
    }
    #[doc = "Lock bit set by the TZ software for the USDHC1"]
    pub mod L_USDHC1 {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No lock-the adjacent (next lower) bit can be written by the software."]
            pub const L_USDHC1_0: u32 = 0;
            #[doc = "Lock-the adjacent (next lower) bit can't be written by the software."]
            pub const L_USDHC1_1: u32 = 0x01;
        }
    }
    #[doc = "Indicates the privilege/user mode for the USDHC2"]
    pub mod HPC_USDHC2 {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "User mode for the corresponding master"]
            pub const HPC_USDHC2_0: u32 = 0;
            #[doc = "Supervisor mode for the corresponding master"]
            pub const HPC_USDHC2_1: u32 = 0x01;
        }
    }
    #[doc = "Lock bit set by the TZ software for the USDHC2."]
    pub mod L_USDHC2 {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No lock-the adjacent (next lower) bit can be written by the software."]
            pub const L_USDHC2_0: u32 = 0;
            #[doc = "Lock-the adjacent (next lower) bit can't be written by the software."]
            pub const L_USDHC2_1: u32 = 0x01;
        }
    }
    #[doc = "Indicates the privilege/user mode for the TPSMP"]
    pub mod HPC_TPSMP {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "User mode for the corresponding master"]
            pub const HPC_TPSMP_0: u32 = 0;
            #[doc = "Supervisor mode for the corresponding master"]
            pub const HPC_TPSMP_1: u32 = 0x01;
        }
    }
    #[doc = "Lock bit set by the TZ software for the TPSMP."]
    pub mod L_TPSMP {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No lock-the adjacent (next lower) bit can be written by the software."]
            pub const L_TPSMP_0: u32 = 0;
            #[doc = "Lock-the adjacent (next lower) bit can't be written by the software."]
            pub const L_TPSMP_1: u32 = 0x01;
        }
    }
    #[doc = "Indicates the privilege/user mode for the USB"]
    pub mod HPC_USB {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "User mode for the corresponding master"]
            pub const HPC_USB_0: u32 = 0;
            #[doc = "Supervisor mode for the corresponding master"]
            pub const HPC_USB_1: u32 = 0x01;
        }
    }
    #[doc = "Lock bit set by the TZ software for the USB."]
    pub mod L_USB {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No lock-the adjacent (next lower) bit can be written by the software."]
            pub const L_USB_0: u32 = 0;
            #[doc = "Lock-the adjacent (next lower) bit can't be written by the software."]
            pub const L_USB_1: u32 = 0x01;
        }
    }
}
