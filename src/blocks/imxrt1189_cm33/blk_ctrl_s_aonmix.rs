#[doc = "Block Control Secure AONMIX"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "CM33_IRQ_MASK0"]
    pub CM33_IRQ_MASK0: crate::RWRegister<u32>,
    #[doc = "CM33 IRQ MASK1"]
    pub CM33_IRQ_MASK1: crate::RWRegister<u32>,
    #[doc = "CM33_IRQ_MASK2"]
    pub CM33_IRQ_MASK2: crate::RWRegister<u32>,
    #[doc = "CM33_IRQ_MASK3"]
    pub CM33_IRQ_MASK3: crate::RWRegister<u32>,
    #[doc = "CM33_IRQ_MASK4"]
    pub CM33_IRQ_MASK4: crate::RWRegister<u32>,
    #[doc = "CM33_IRQ_MASK5"]
    pub CM33_IRQ_MASK5: crate::RWRegister<u32>,
    #[doc = "CM33_IRQ_MASK6"]
    pub CM33_IRQ_MASK6: crate::RWRegister<u32>,
    #[doc = "CM33_IRQ_MASK7"]
    pub CM33_IRQ_MASK7: crate::RWRegister<u32>,
    #[doc = "CM7_IRQ_MASK0"]
    pub CM7_IRQ_MASK0: crate::RWRegister<u32>,
    #[doc = "CM7_IRQ_MASK1"]
    pub CM7_IRQ_MASK1: crate::RWRegister<u32>,
    #[doc = "CM7_IRQ_MASK2"]
    pub CM7_IRQ_MASK2: crate::RWRegister<u32>,
    #[doc = "CM7_IRQ_MASK3"]
    pub CM7_IRQ_MASK3: crate::RWRegister<u32>,
    #[doc = "CM7_IRQ_MASK4"]
    pub CM7_IRQ_MASK4: crate::RWRegister<u32>,
    #[doc = "CM7_IRQ_MASK5"]
    pub CM7_IRQ_MASK5: crate::RWRegister<u32>,
    #[doc = "CM7_IRQ_MASK6"]
    pub CM7_IRQ_MASK6: crate::RWRegister<u32>,
    #[doc = "CM7_IRQ_MASK7"]
    pub CM7_IRQ_MASK7: crate::RWRegister<u32>,
    _reserved0: [u8; 0x18],
    #[doc = "EdgeLock reset request mask"]
    pub EDGELOCK_RESET_REQ_MASK: crate::RWRegister<u32>,
    #[doc = "EdgeLock IRQ request mask"]
    pub EDGELOCK_IRQ_MASK: crate::RWRegister<u32>,
    #[doc = "M33 Configuration"]
    pub M33_CFG: crate::RWRegister<u32>,
    #[doc = "M33 INITSVTOR"]
    pub M33_INITSVTOR: crate::RWRegister<u32>,
    #[doc = "M33 INITNSVTOR"]
    pub M33_INITNSVTOR: crate::RWRegister<u32>,
    _reserved1: [u8; 0x14],
    #[doc = "M7 Configuration"]
    pub M7_CFG: crate::RWRegister<u32>,
    _reserved2: [u8; 0x0c],
    #[doc = "AXBS_AON_CTRL"]
    pub AXBS_AON_CTRL: crate::RWRegister<u32>,
    _reserved3: [u8; 0x6c],
    #[doc = "DAP Access Sticky Bit"]
    pub DAP_ACCESS_STKYBIT: crate::RWRegister<u32>,
    _reserved4: [u8; 0x0c],
    #[doc = "Low power handshake enable"]
    pub LP_HANDSHAKE: crate::RWRegister<u32>,
    #[doc = "EdgeLock halt status"]
    pub EDGELOCK_HALT_ST: crate::RWRegister<u32>,
    _reserved5: [u8; 0x08],
    #[doc = "ECC memory hardware initialization"]
    pub ECC_MEM_INIT: crate::RORegister<u32>,
    _reserved6: [u8; 0x24],
    #[doc = "IOMUXC domain configure"]
    pub IOMUXC_DOMAIN_CFG: crate::RWRegister<u32>,
    #[doc = "IOMUXC_AON domain configure"]
    pub IOMUXC_AON_DOMAIN_CFG: crate::RWRegister<u32>,
    _reserved7: [u8; 0x04],
    #[doc = "NMI control"]
    pub NMI_CTRL: crate::RWRegister<u32>,
    #[doc = "s401_ipi_noclk_ref1 clear control"]
    pub S401_NOCLK_CLEAR_CTRL: crate::RWRegister<u32>,
}
#[doc = "CM33_IRQ_MASK0"]
pub mod CM33_IRQ_MASK0 {
    #[doc = "CM33 IRQ MASK"]
    pub mod M {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Mask IRQ"]
            pub const ENABLE: u32 = 0;
            #[doc = "No Mask IRQ"]
            pub const DISABLE: u32 = 0x01;
        }
    }
}
#[doc = "CM33 IRQ MASK1"]
pub mod CM33_IRQ_MASK1 {
    #[doc = "CM33 IRQ MASK"]
    pub mod M {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Mask IRQ"]
            pub const ENABLE: u32 = 0;
            #[doc = "No Mask IRQ"]
            pub const DISABLE: u32 = 0x01;
        }
    }
}
#[doc = "CM33_IRQ_MASK2"]
pub mod CM33_IRQ_MASK2 {
    #[doc = "CM33 IRQ MASK"]
    pub mod M {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Mask IRQ"]
            pub const ENABLE: u32 = 0;
            #[doc = "No Mask IRQ"]
            pub const DISABLE: u32 = 0x01;
        }
    }
}
#[doc = "CM33_IRQ_MASK3"]
pub mod CM33_IRQ_MASK3 {
    #[doc = "CM33 IRQ MASK"]
    pub mod M {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Mask IRQ"]
            pub const ENABLE: u32 = 0;
            #[doc = "No Mask IRQ"]
            pub const DISABLE: u32 = 0x01;
        }
    }
}
#[doc = "CM33_IRQ_MASK4"]
pub mod CM33_IRQ_MASK4 {
    #[doc = "CM33 IRQ MASK"]
    pub mod M {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Mask IRQ"]
            pub const ENABLE: u32 = 0;
            #[doc = "No Mask IRQ"]
            pub const DISABLE: u32 = 0x01;
        }
    }
}
#[doc = "CM33_IRQ_MASK5"]
pub mod CM33_IRQ_MASK5 {
    #[doc = "CM33 IRQ MASK"]
    pub mod M {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Mask IRQ"]
            pub const ENABLE: u32 = 0;
            #[doc = "No Mask IRQ"]
            pub const DISABLE: u32 = 0x01;
        }
    }
}
#[doc = "CM33_IRQ_MASK6"]
pub mod CM33_IRQ_MASK6 {
    #[doc = "CM33 IRQ MASK"]
    pub mod M {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Mask IRQ"]
            pub const ENABLE: u32 = 0;
            #[doc = "No Mask IRQ"]
            pub const DISABLE: u32 = 0x01;
        }
    }
}
#[doc = "CM33_IRQ_MASK7"]
pub mod CM33_IRQ_MASK7 {
    #[doc = "CM33 IRQ MASK"]
    pub mod M {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Mask IRQ"]
            pub const ENABLE: u32 = 0;
            #[doc = "No Mask IRQ"]
            pub const DISABLE: u32 = 0x01;
        }
    }
}
#[doc = "CM7_IRQ_MASK0"]
pub mod CM7_IRQ_MASK0 {
    #[doc = "CM7 IRQ MASK"]
    pub mod M {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Mask IRQ"]
            pub const ENABLE: u32 = 0;
            #[doc = "No Mask IRQ"]
            pub const DISABLE: u32 = 0x01;
        }
    }
}
#[doc = "CM7_IRQ_MASK1"]
pub mod CM7_IRQ_MASK1 {
    #[doc = "CM7 IRQ MASK"]
    pub mod M {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Mask IRQ"]
            pub const ENABLE: u32 = 0;
            #[doc = "No Mask IRQ"]
            pub const DISABLE: u32 = 0x01;
        }
    }
}
#[doc = "CM7_IRQ_MASK2"]
pub mod CM7_IRQ_MASK2 {
    #[doc = "CM7 IRQ MASK"]
    pub mod M {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Mask IRQ"]
            pub const ENABLE: u32 = 0;
            #[doc = "No Mask IRQ"]
            pub const DISABLE: u32 = 0x01;
        }
    }
}
#[doc = "CM7_IRQ_MASK3"]
pub mod CM7_IRQ_MASK3 {
    #[doc = "CM7 IRQ MASK"]
    pub mod M {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Mask IRQ"]
            pub const ENABLE: u32 = 0;
            #[doc = "No Mask IRQ"]
            pub const DISABLE: u32 = 0x01;
        }
    }
}
#[doc = "CM7_IRQ_MASK4"]
pub mod CM7_IRQ_MASK4 {
    #[doc = "CM7 IRQ MASK"]
    pub mod M {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Mask IRQ"]
            pub const ENABLE: u32 = 0;
            #[doc = "No Mask IRQ"]
            pub const DISABLE: u32 = 0x01;
        }
    }
}
#[doc = "CM7_IRQ_MASK5"]
pub mod CM7_IRQ_MASK5 {
    #[doc = "CM7 IRQ MASK"]
    pub mod M {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Mask IRQ"]
            pub const ENABLE: u32 = 0;
            #[doc = "No Mask IRQ"]
            pub const DISABLE: u32 = 0x01;
        }
    }
}
#[doc = "CM7_IRQ_MASK6"]
pub mod CM7_IRQ_MASK6 {
    #[doc = "CM7 IRQ MASK"]
    pub mod M {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Mask IRQ"]
            pub const ENABLE: u32 = 0;
            #[doc = "No Mask IRQ"]
            pub const DISABLE: u32 = 0x01;
        }
    }
}
#[doc = "CM7_IRQ_MASK7"]
pub mod CM7_IRQ_MASK7 {
    #[doc = "CM7 IRQ MASK"]
    pub mod M {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Mask IRQ"]
            pub const ENABLE: u32 = 0;
            #[doc = "No Mask IRQ"]
            pub const DISABLE: u32 = 0x01;
        }
    }
}
#[doc = "EdgeLock reset request mask"]
pub mod EDGELOCK_RESET_REQ_MASK {
    #[doc = "EdgeLock Wdog reset mask"]
    pub mod WDG_RESET {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Unmask reset"]
            pub const DISABLE: u32 = 0;
            #[doc = "Mask reset"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "EdgeLock PUF reset mask"]
    pub mod PUF_RESET {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Unmask reset"]
            pub const DISABLE: u32 = 0;
            #[doc = "Mask reset"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "EdgeLock LMDA life cycle bricked reset mask"]
    pub mod LC_BRICKED {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Unmask reset"]
            pub const DISABLE: u32 = 0;
            #[doc = "Mask reset"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "EdgeLock system failure reset mask"]
    pub mod LMDA_SYS_FAIL {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Unmask reset"]
            pub const DISABLE: u32 = 0;
            #[doc = "Mask reset"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "EdgeLock 32k clock loss reset mask"]
    pub mod NOCLK_32K {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Unmask reset"]
            pub const DISABLE: u32 = 0;
            #[doc = "Mask reset"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "EdgeLock LMDA reset request mask"]
    pub mod LMDA_RESET_REQ {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Unmask reset"]
            pub const DISABLE: u32 = 0;
            #[doc = "Mask reset"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "EdgeLock LMDA reset request from 32k clock domain mask"]
    pub mod LMDA_32K_RESET_REQ {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Unmask reset"]
            pub const DISABLE: u32 = 0;
            #[doc = "Mask reset"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "EdgeLock CM33 root clock loss reset mask"]
    pub mod NOCLK_REF1 {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Unmask reset"]
            pub const DISABLE: u32 = 0;
            #[doc = "Mask reset"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "EdgeLock OSC 24Mhz clock loss reset mask"]
    pub mod NOCLK_REF2 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Unmask reset"]
            pub const DISABLE: u32 = 0;
            #[doc = "Mask reset"]
            pub const ENABLE: u32 = 0x01;
        }
    }
}
#[doc = "EdgeLock IRQ request mask"]
pub mod EDGELOCK_IRQ_MASK {
    #[doc = "EdgeLock Wdog reset interrupt mask"]
    pub mod WDG_RESET {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Unmask interrupt"]
            pub const DISABLE: u32 = 0;
            #[doc = "Mask interrupt"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "EdgeLock PUF reset interrupt mask"]
    pub mod PUF_RESET {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Unmask interrupt"]
            pub const DISABLE: u32 = 0;
            #[doc = "Mask interrupt"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "EdgeLock LMDA life cycle bricked interrupt mask"]
    pub mod LC_BRICKED {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Unmask interrupt"]
            pub const DISABLE: u32 = 0;
            #[doc = "Mask interrupt"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "EdgeLock system failure interrupt mask"]
    pub mod LMDA_SYS_FAIL {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Unmask interrupt"]
            pub const DISABLE: u32 = 0;
            #[doc = "Mask interrupt"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "EdgeLock 32k clock loss interrupt mask"]
    pub mod NOCLK_32K {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Unmask interrupt"]
            pub const DISABLE: u32 = 0;
            #[doc = "Mask interrupt"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "EdgeLock LMDA reset request interrupt mask"]
    pub mod LMDA_RESET_REQ {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Unmask interrupt"]
            pub const DISABLE: u32 = 0;
            #[doc = "Mask interrupt"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "EdgeLock LMDA reset request from 32k clock domain interrupt mask"]
    pub mod LMDA_32K_RESET_REQ {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Unmask interrupt"]
            pub const DISABLE: u32 = 0;
            #[doc = "Mask interrupt"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "EdgeLock cm33 root clock loss interrupt mask"]
    pub mod NOCLK_REF1 {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Unmask interrupt"]
            pub const DISABLE: u32 = 0;
            #[doc = "Mask interrupt"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "EdgeLock OSC 24Mhz clock loss interrupt mask"]
    pub mod NOCLK_REF2 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Unmask interrupt"]
            pub const DISABLE: u32 = 0;
            #[doc = "Mask interrupt"]
            pub const ENABLE: u32 = 0x01;
        }
    }
}
#[doc = "M33 Configuration"]
pub mod M33_CFG {
    #[doc = "M33 CPU WAIT"]
    pub mod WAIT {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "M33 TCM SIZE"]
    pub mod TCM_SIZE {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Regular TCM, 128KB Code TCM and 128KB Sys TCM"]
            pub const TCM_SIZE3: u32 = 0;
            #[doc = "Double Code TCM, 256KB Code TCM"]
            pub const TCM_SIZE2: u32 = 0x01;
            #[doc = "Double Sys TCM, 256KB Sys TCM"]
            pub const TCM_SIZE1: u32 = 0x02;
        }
    }
    #[doc = "Force CM33 core clock on in WAIT mode"]
    pub mod CORECLK_FORCE_ON {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "CM33 core clock is off in WAIT mode"]
            pub const OFF: u32 = 0;
            #[doc = "CM33 core clock is on in WAIT mode"]
            pub const ON: u32 = 0x01;
        }
    }
}
#[doc = "M33 INITSVTOR"]
pub mod M33_INITSVTOR {
    #[doc = "M33 INITSVTOR"]
    pub mod INITSVTOR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "M33 INITNSVTOR"]
pub mod M33_INITNSVTOR {
    #[doc = "M33 INITNSVTOR"]
    pub mod INITNSVTOR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "M7 Configuration"]
pub mod M7_CFG {
    #[doc = "M7 TCM SIZE"]
    pub mod TCM_SIZE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Regular TCM, 256KB ITCM and 256KB DTCM"]
            pub const REG_ITCM: u32 = 0;
            #[doc = "Double ITCM, 512KB ITCM"]
            pub const DOUBLE_ITCM: u32 = 0x01;
            #[doc = "Double DTCM, 512KB DTCM"]
            pub const DOUBLE_DTCM: u32 = 0x02;
            #[doc = "HALF ITCM, 128KB ITCM and 384KB DTCM"]
            pub const HALF_ITCM: u32 = 0x04;
            #[doc = "HALF DTCM, 384KB ITCM and 128KB DTCM"]
            pub const HALF_DTCM: u32 = 0x05;
        }
    }
    #[doc = "M7 CPUWAIT"]
    pub mod WAIT {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Force CM7 core clock on in WAIT mode"]
    pub mod CORECLK_FORCE_ON {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "CM7 core clock is off in WAIT mode"]
            pub const OFF: u32 = 0;
            #[doc = "CM7 core clock is on in WAIT mode"]
            pub const ON: u32 = 0x01;
        }
    }
    #[doc = "CM7 platform AHB clock enable"]
    pub mod HCLK_FORCE_ON {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "M7 INITVTOR\\[31:7\\]"]
    pub mod INITVTOR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "AXBS_AON_CTRL"]
pub mod AXBS_AON_CTRL {
    #[doc = "AXBS_AON Force Round Robin"]
    pub mod FORCE_ROUND_ROBIN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Enable force round robin(default)"]
            pub const ENABLE: u32 = 0;
            #[doc = "Disable force round robin"]
            pub const DISABLE: u32 = 0x01;
        }
    }
    #[doc = "M0 High Priority Control Bit"]
    pub mod M0_HIGH_PRIORITY {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Default Priority"]
            pub const ENABLE: u32 = 0;
            #[doc = "High Priority"]
            pub const DISABLE: u32 = 0x01;
        }
    }
    #[doc = "M1 High Priority Control Bit"]
    pub mod M1_HIGH_PRIORITY {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Default Priority"]
            pub const ENABLE: u32 = 0;
            #[doc = "High Priority"]
            pub const DISABLE: u32 = 0x01;
        }
    }
    #[doc = "M2 High Priority Control Bit"]
    pub mod M2_HIGH_PRIORITY {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Default Priority"]
            pub const ENABLE: u32 = 0;
            #[doc = "High Priority"]
            pub const DISABLE: u32 = 0x01;
        }
    }
    #[doc = "M3 High Priority Control Bit"]
    pub mod M3_HIGH_PRIORITY {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Default Priority"]
            pub const ENABLE: u32 = 0;
            #[doc = "High Priority"]
            pub const DISABLE: u32 = 0x01;
        }
    }
    #[doc = "M4 High Priority Control Bit"]
    pub mod M4_HIGH_PRIORITY {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Default Priority"]
            pub const ENABLE: u32 = 0;
            #[doc = "High Priority"]
            pub const DISABLE: u32 = 0x01;
        }
    }
    #[doc = "M5 High Priority Control Bit"]
    pub mod M5_HIGH_PRIORITY {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Default Priority"]
            pub const ENABLE: u32 = 0;
            #[doc = "High Priority"]
            pub const DISABLE: u32 = 0x01;
        }
    }
    #[doc = "M6 High Priority Control Bit"]
    pub mod M6_HIGH_PRIORITY {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Default Priority"]
            pub const ENABLE: u32 = 0;
            #[doc = "High Priority"]
            pub const DISABLE: u32 = 0x01;
        }
    }
}
#[doc = "DAP Access Sticky Bit"]
pub mod DAP_ACCESS_STKYBIT {
    #[doc = "DAP access grant bit controlled by Cortex-M33 ROM, once set \"1\" will kept \"1\" unless there is a reset."]
    pub mod DAP_CTR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "DAP access is not granted by ROM"]
            pub const DAP_NO: u32 = 0;
            #[doc = "DAP access is granted by ROM"]
            pub const DAP_YES: u32 = 0x01;
        }
    }
}
#[doc = "Low power handshake enable"]
pub mod LP_HANDSHAKE {
    #[doc = "CM33 reset handshake enable"]
    pub mod CM33_RESET_HS_EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Handshake is not enabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Handshake is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "CM7 reset handshake enable"]
    pub mod CM7_RESET_HS_EN {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Handshake is not enabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Handshake is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "CM7 suspend exit reset handshake enable"]
    pub mod CM7_SUSPEND_HS_EN {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Handshake is not enabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Handshake is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "AONMIX reset handshake enable"]
    pub mod AONMIX_RESET_HS_EN {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Handshake is not enabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Handshake is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Wakeupmix reset handshake enable"]
    pub mod WAKEUPMIX_RESET_HS_EN {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Handshake is not enabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Handshake is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Megamix reset handshake enable"]
    pub mod MEGAMIX_RESET_HS_EN {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Handshake is not enabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Handshake is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Megamix low power mode exit reset handshake enable"]
    pub mod MEGAMIX_LPM_HS_EN {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Handshake is not enabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Handshake is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "EDGELOCK clock off handshake enable"]
    pub mod EDGELOCK_CLK_OFF_HS_EN {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Handshake is not enabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Handshake is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "CM33 clock off handshake enable"]
    pub mod CM33_CLK_OFF_HS_EN {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Handshake is not enabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Handshake is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "CM7 clock off handshake enable"]
    pub mod CM7_CLK_OFF_HS_EN {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Handshake is not enabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Handshake is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "TRDC clock off handshake enable"]
    pub mod TRDC_CLK_OFF_HS_EN {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Handshake is not enabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Handshake is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "IEE clock off handshake enable"]
    pub mod IEE_CLK_OFF_HS_EN {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Handshake is not enabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Handshake is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "OTFAD1 clock off handshake enable"]
    pub mod OTFAD1_CLK_OFF_HS_EN {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Handshake is not enabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Handshake is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "OTFAD2 clock off handshake enable"]
    pub mod OTFAD2_CLK_OFF_HS_EN {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Handshake is not enabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Handshake is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "EDGELOCK clock on handshake enable"]
    pub mod EDGELOCK_CLK_ON_HS_EN {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Handshake is not enabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Handshake is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "CM33 clock on handshake enable"]
    pub mod CM33_CLK_ON_HS_EN {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Handshake is not enabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Handshake is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "CM7 clock on handshake enable"]
    pub mod CM7_CLK_ON_HS_EN {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Handshake is not enabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Handshake is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "TRDC clock on handshake enable"]
    pub mod TRDC_CLK_ON_HS_EN {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Handshake is not enabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Handshake is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "IEE clock on handshake enable"]
    pub mod IEE_CLK_ON_HS_EN {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Handshake is not enabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Handshake is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "OTFAD1 clock on handshake enable"]
    pub mod OTFAD1_CLK_ON_HS_EN {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Handshake is not enabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Handshake is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "OTFAD2 clock on handshake enable"]
    pub mod OTFAD2_CLK_ON_HS_EN {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Handshake is not enabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Handshake is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
}
#[doc = "EdgeLock halt status"]
pub mod EDGELOCK_HALT_ST {
    #[doc = "EdgeLock halt and clock status"]
    pub mod EDGELOCK_HALT_ACK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "EdgeLock is not fully halted and its clocks must be enabled"]
            pub const EDGELOCK_NOT_HALTED: u32 = 0;
            #[doc = "EdgeLock is fully halted indicating clocks may be removed"]
            pub const EDGELOCK_HALTED: u32 = 0x01;
        }
    }
    #[doc = "EdgeLock halt exit interrupt clear"]
    pub mod EDGELOCK_HALT_EXIT_IRQ_CLR {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Remove the clear signal. This bit is not self-clearing and need SW to clear."]
            pub const REMOVE_CLR: u32 = 0;
            #[doc = "Clear EdgeLock halt exit interrupt"]
            pub const INT_CLR: u32 = 0x01;
        }
    }
}
#[doc = "ECC memory hardware initialization"]
pub mod ECC_MEM_INIT {
    #[doc = "OCRAM1 initialization status"]
    pub mod OCRAM1_INIT_DONE {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "OCRAM1 memory is under initialization"]
            pub const OCRAM_INIT: u32 = 0;
            #[doc = "OCRAM1 memory initialization is complete"]
            pub const OCRAM_INIT_DONE: u32 = 0x01;
        }
    }
    #[doc = "OCRAM2 initialization status"]
    pub mod OCRAM2_INIT_DONE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "OCRAM2 memory is under initialization"]
            pub const OCRAM_INIT: u32 = 0;
            #[doc = "OCRAM2 memory initialization is complete"]
            pub const OCRAM_INIT_DONE: u32 = 0x01;
        }
    }
}
#[doc = "IOMUXC domain configure"]
pub mod IOMUXC_DOMAIN_CFG {
    #[doc = "Domain ID 0"]
    pub mod DID0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Domain ID 1"]
    pub mod DID1 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Domain ID 2"]
    pub mod DID2 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Domain ID 3"]
    pub mod DID3 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Lock bit"]
    pub mod LOCK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "IOMUXC_AON domain configure"]
pub mod IOMUXC_AON_DOMAIN_CFG {
    #[doc = "Domain ID 0"]
    pub mod DID0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Domain ID 1"]
    pub mod DID1 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Domain ID 2"]
    pub mod DID2 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Domain ID 3"]
    pub mod DID3 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Lock bit"]
    pub mod LOCK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "NMI control"]
pub mod NMI_CTRL {
    #[doc = "Mask CM7 NMI pin input"]
    pub mod M7_NMI_MASK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "NMI input from IO to CM7 is not blocked"]
            pub const DISABLE: u32 = 0;
            #[doc = "NMI input from IO to CM7 is blocked"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Mask CM33 NMI pin input"]
    pub mod M33_NMI_MASK {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "NMI input from IO to CM33 is not blocked"]
            pub const DISABLE: u32 = 0;
            #[doc = "NMI input from IO to CM33 is blocked"]
            pub const ENABLE: u32 = 0x01;
        }
    }
}
#[doc = "s401_ipi_noclk_ref1 clear control"]
pub mod S401_NOCLK_CLEAR_CTRL {
    #[doc = "clear the interrupt or reset source"]
    pub mod REF1_SLOW_CLEAR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
