#[doc = "IOMUXC GPR"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "GPR0 General Purpose Register"]
    pub GPR0: crate::RWRegister<u32>,
    #[doc = "GPR1 General Purpose Register"]
    pub GPR1: crate::RWRegister<u32>,
    #[doc = "GPR2 General Purpose Register"]
    pub GPR2: crate::RWRegister<u32>,
    #[doc = "GPR3 General Purpose Register"]
    pub GPR3: crate::RWRegister<u32>,
    #[doc = "GPR4 General Purpose Register"]
    pub GPR4: crate::RWRegister<u32>,
    #[doc = "GPR5 General Purpose Register"]
    pub GPR5: crate::RWRegister<u32>,
    #[doc = "GPR6 General Purpose Register"]
    pub GPR6: crate::RWRegister<u32>,
    #[doc = "GPR7 General Purpose Register"]
    pub GPR7: crate::RWRegister<u32>,
    #[doc = "GPR8 General Purpose Register"]
    pub GPR8: crate::RWRegister<u32>,
    #[doc = "GPR9 General Purpose Register"]
    pub GPR9: crate::RWRegister<u32>,
    #[doc = "GPR10 General Purpose Register"]
    pub GPR10: crate::RWRegister<u32>,
    #[doc = "GPR11 General Purpose Register"]
    pub GPR11: crate::RWRegister<u32>,
    #[doc = "GPR12 General Purpose Register"]
    pub GPR12: crate::RWRegister<u32>,
    #[doc = "GPR13 General Purpose Register"]
    pub GPR13: crate::RWRegister<u32>,
    #[doc = "GPR14 General Purpose Register"]
    pub GPR14: crate::RWRegister<u32>,
    #[doc = "GPR15 General Purpose Register"]
    pub GPR15: crate::RWRegister<u32>,
    #[doc = "GPR16 General Purpose Register"]
    pub GPR16: crate::RWRegister<u32>,
    #[doc = "GPR17 General Purpose Register"]
    pub GPR17: crate::RWRegister<u32>,
    #[doc = "GPR18 General Purpose Register"]
    pub GPR18: crate::RWRegister<u32>,
    _reserved0: [u8; 0x04],
    #[doc = "GPR20 General Purpose Register"]
    pub GPR20: crate::RWRegister<u32>,
    #[doc = "GPR21 General Purpose Register"]
    pub GPR21: crate::RWRegister<u32>,
    #[doc = "GPR22 General Purpose Register"]
    pub GPR22: crate::RWRegister<u32>,
    #[doc = "GPR23 General Purpose Register"]
    pub GPR23: crate::RWRegister<u32>,
    #[doc = "GPR24 General Purpose Register"]
    pub GPR24: crate::RWRegister<u32>,
    #[doc = "GPR25 General Purpose Register"]
    pub GPR25: crate::RWRegister<u32>,
    #[doc = "GPR26 General Purpose Register"]
    pub GPR26: crate::RWRegister<u32>,
    #[doc = "GPR27 General Purpose Register"]
    pub GPR27: crate::RWRegister<u32>,
    #[doc = "GPR28 General Purpose Register"]
    pub GPR28: crate::RWRegister<u32>,
    #[doc = "GPR29 General Purpose Register"]
    pub GPR29: crate::RWRegister<u32>,
    #[doc = "GPR30 General Purpose Register"]
    pub GPR30: crate::RWRegister<u32>,
    #[doc = "GPR31 General Purpose Register"]
    pub GPR31: crate::RWRegister<u32>,
    #[doc = "GPR32 General Purpose Register"]
    pub GPR32: crate::RWRegister<u32>,
    #[doc = "GPR33 General Purpose Register"]
    pub GPR33: crate::RWRegister<u32>,
    #[doc = "GPR34 General Purpose Register"]
    pub GPR34: crate::RWRegister<u32>,
    #[doc = "GPR35 General Purpose Register"]
    pub GPR35: crate::RWRegister<u32>,
    #[doc = "GPR36 General Purpose Register"]
    pub GPR36: crate::RWRegister<u32>,
    #[doc = "GPR37 General Purpose Register"]
    pub GPR37: crate::RWRegister<u32>,
    #[doc = "GPR38 General Purpose Register"]
    pub GPR38: crate::RWRegister<u32>,
    #[doc = "GPR39 General Purpose Register"]
    pub GPR39: crate::RWRegister<u32>,
    #[doc = "GPR40 General Purpose Register"]
    pub GPR40: crate::RWRegister<u32>,
    #[doc = "GPR41 General Purpose Register"]
    pub GPR41: crate::RWRegister<u32>,
    #[doc = "GPR42 General Purpose Register"]
    pub GPR42: crate::RWRegister<u32>,
    #[doc = "GPR43 General Purpose Register"]
    pub GPR43: crate::RWRegister<u32>,
    #[doc = "GPR44 General Purpose Register"]
    pub GPR44: crate::RWRegister<u32>,
    #[doc = "GPR45 General Purpose Register"]
    pub GPR45: crate::RWRegister<u32>,
    #[doc = "GPR46 General Purpose Register"]
    pub GPR46: crate::RWRegister<u32>,
    #[doc = "GPR47 General Purpose Register"]
    pub GPR47: crate::RWRegister<u32>,
    #[doc = "GPR48 General Purpose Register"]
    pub GPR48: crate::RWRegister<u32>,
    #[doc = "GPR49 General Purpose Register"]
    pub GPR49: crate::RWRegister<u32>,
    #[doc = "GPR50 General Purpose Register"]
    pub GPR50: crate::RWRegister<u32>,
    #[doc = "GPR51 General Purpose Register"]
    pub GPR51: crate::RWRegister<u32>,
    #[doc = "GPR52 General Purpose Register"]
    pub GPR52: crate::RWRegister<u32>,
    #[doc = "GPR53 General Purpose Register"]
    pub GPR53: crate::RWRegister<u32>,
    #[doc = "GPR54 General Purpose Register"]
    pub GPR54: crate::RWRegister<u32>,
    #[doc = "GPR55 General Purpose Register"]
    pub GPR55: crate::RWRegister<u32>,
    _reserved1: [u8; 0x0c],
    #[doc = "GPR59 General Purpose Register"]
    pub GPR59: crate::RWRegister<u32>,
    _reserved2: [u8; 0x08],
    #[doc = "GPR62 General Purpose Register"]
    pub GPR62: crate::RWRegister<u32>,
    #[doc = "GPR63 General Purpose Register"]
    pub GPR63: crate::RORegister<u32>,
    #[doc = "GPR64 General Purpose Register"]
    pub GPR64: crate::RWRegister<u32>,
    #[doc = "GPR65 General Purpose Register"]
    pub GPR65: crate::RWRegister<u32>,
    #[doc = "GPR66 General Purpose Register"]
    pub GPR66: crate::RWRegister<u32>,
    #[doc = "GPR67 General Purpose Register"]
    pub GPR67: crate::RWRegister<u32>,
    #[doc = "GPR68 General Purpose Register"]
    pub GPR68: crate::RWRegister<u32>,
    #[doc = "GPR69 General Purpose Register"]
    pub GPR69: crate::RWRegister<u32>,
    #[doc = "GPR70 General Purpose Register"]
    pub GPR70: crate::RWRegister<u32>,
    #[doc = "GPR71 General Purpose Register"]
    pub GPR71: crate::RWRegister<u32>,
    #[doc = "GPR72 General Purpose Register"]
    pub GPR72: crate::RWRegister<u32>,
    #[doc = "GPR73 General Purpose Register"]
    pub GPR73: crate::RWRegister<u32>,
    #[doc = "GPR74 General Purpose Register"]
    pub GPR74: crate::RWRegister<u32>,
    #[doc = "GPR75 General Purpose Register"]
    pub GPR75: crate::RORegister<u32>,
    #[doc = "GPR76 General Purpose Register"]
    pub GPR76: crate::RORegister<u32>,
}
#[doc = "GPR0 General Purpose Register"]
pub mod GPR0 {
    #[doc = "SAI1 MCLK1 source select"]
    pub mod SAI1_MCLK1_SEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "SAI1 MCLK2 source select"]
    pub mod SAI1_MCLK2_SEL {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "SAI1 MCLK3 source select"]
    pub mod SAI1_MCLK3_SEL {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "SAI1_MCLK signal direction control"]
    pub mod SAI1_MCLK_DIR {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Domain write protection"]
    pub mod DWP {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Both cores are allowed"]
            pub const FORBID_NONE: u32 = 0;
            #[doc = "CM7 is forbidden"]
            pub const FORBID_CM7: u32 = 0x01;
            #[doc = "CM4 is forbidden"]
            pub const FORBID_CM4: u32 = 0x02;
            #[doc = "Both cores are forbidden"]
            pub const FORBID_BOTH: u32 = 0x03;
        }
    }
    #[doc = "Domain write protection lock"]
    pub mod DWP_LOCK {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Neither of DWP bits is locked"]
            pub const LOCK_NONE: u32 = 0;
            #[doc = "The lower DWP bit is locked"]
            pub const LOCK_LOW: u32 = 0x01;
            #[doc = "The higher DWP bit is locked"]
            pub const LOCK_HIGH: u32 = 0x02;
            #[doc = "Both DWP bits are locked"]
            pub const LOCK_BOTH: u32 = 0x03;
        }
    }
}
#[doc = "GPR1 General Purpose Register"]
pub mod GPR1 {
    #[doc = "SAI2 MCLK3 source select"]
    pub mod SAI2_MCLK3_SEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "SAI2_MCLK signal direction control"]
    pub mod SAI2_MCLK_DIR {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Domain write protection"]
    pub mod DWP {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Both cores are allowed"]
            pub const FORBID_NONE: u32 = 0;
            #[doc = "CM7 is forbidden"]
            pub const FORBID_CM7: u32 = 0x01;
            #[doc = "CM4 is forbidden"]
            pub const FORBID_CM4: u32 = 0x02;
            #[doc = "Both cores are forbidden"]
            pub const FORBID_BOTH: u32 = 0x03;
        }
    }
    #[doc = "Domain write protection lock"]
    pub mod DWP_LOCK {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Neither of DWP bits is locked"]
            pub const LOCK_NONE: u32 = 0;
            #[doc = "The lower DWP bit is locked"]
            pub const LOCK_LOW: u32 = 0x01;
            #[doc = "The higher DWP bit is locked"]
            pub const LOCK_HIGH: u32 = 0x02;
            #[doc = "Both DWP bits are locked"]
            pub const LOCK_BOTH: u32 = 0x03;
        }
    }
}
#[doc = "GPR2 General Purpose Register"]
pub mod GPR2 {
    #[doc = "SAI3 MCLK3 source select"]
    pub mod SAI3_MCLK3_SEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "SAI3_MCLK signal direction control"]
    pub mod SAI3_MCLK_DIR {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "SAI4_MCLK signal direction control"]
    pub mod SAI4_MCLK_DIR {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Domain write protection"]
    pub mod DWP {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Both cores are allowed"]
            pub const FORBID_NONE: u32 = 0;
            #[doc = "CM7 is forbidden"]
            pub const FORBID_CM7: u32 = 0x01;
            #[doc = "CM4 is forbidden"]
            pub const FORBID_CM4: u32 = 0x02;
            #[doc = "Both cores are forbidden"]
            pub const FORBID_BOTH: u32 = 0x03;
        }
    }
    #[doc = "Domain write protection lock"]
    pub mod DWP_LOCK {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Neither of DWP bits is locked"]
            pub const LOCK_NONE: u32 = 0;
            #[doc = "The lower DWP bit is locked"]
            pub const LOCK_LOW: u32 = 0x01;
            #[doc = "The higher DWP bit is locked"]
            pub const LOCK_HIGH: u32 = 0x02;
            #[doc = "Both DWP bits are locked"]
            pub const LOCK_BOTH: u32 = 0x03;
        }
    }
}
#[doc = "GPR3 General Purpose Register"]
pub mod GPR3 {
    #[doc = "Divider ratio control for mclk from hmclk."]
    pub mod MQS_CLK_DIV {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "MQS software reset"]
    pub mod MQS_SW_RST {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "MQS enable"]
    pub mod MQS_EN {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Medium Quality Sound (MQS) Oversample"]
    pub mod MQS_OVERSAMPLE {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Domain write protection"]
    pub mod DWP {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Both cores are allowed"]
            pub const FORBID_NONE: u32 = 0;
            #[doc = "CM7 is forbidden"]
            pub const FORBID_CM7: u32 = 0x01;
            #[doc = "CM4 is forbidden"]
            pub const FORBID_CM4: u32 = 0x02;
            #[doc = "Both cores are forbidden"]
            pub const FORBID_BOTH: u32 = 0x03;
        }
    }
    #[doc = "Domain write protection lock"]
    pub mod DWP_LOCK {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Neither of DWP bits is locked"]
            pub const LOCK_NONE: u32 = 0;
            #[doc = "The lower DWP bit is locked"]
            pub const LOCK_LOW: u32 = 0x01;
            #[doc = "The higher DWP bit is locked"]
            pub const LOCK_HIGH: u32 = 0x02;
            #[doc = "Both DWP bits are locked"]
            pub const LOCK_BOTH: u32 = 0x03;
        }
    }
}
#[doc = "GPR4 General Purpose Register"]
pub mod GPR4 {
    #[doc = "ENET TX_CLK select"]
    pub mod ENET_TX_CLK_SEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ENET_REF_CLK direction control"]
    pub mod ENET_REF_CLK_DIR {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ENET master timer source select"]
    pub mod ENET_TIME_SEL {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ENET ENET_1588_EVENT0_IN source select"]
    pub mod ENET_EVENT0IN_SEL {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Domain write protection"]
    pub mod DWP {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Both cores are allowed"]
            pub const FORBID_NONE: u32 = 0;
            #[doc = "CM7 is forbidden"]
            pub const FORBID_CM7: u32 = 0x01;
            #[doc = "CM4 is forbidden"]
            pub const FORBID_CM4: u32 = 0x02;
            #[doc = "Both cores are forbidden"]
            pub const FORBID_BOTH: u32 = 0x03;
        }
    }
    #[doc = "Domain write protection lock"]
    pub mod DWP_LOCK {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Neither of DWP bits is locked"]
            pub const LOCK_NONE: u32 = 0;
            #[doc = "The lower DWP bit is locked"]
            pub const LOCK_LOW: u32 = 0x01;
            #[doc = "The higher DWP bit is locked"]
            pub const LOCK_HIGH: u32 = 0x02;
            #[doc = "Both DWP bits are locked"]
            pub const LOCK_BOTH: u32 = 0x03;
        }
    }
}
#[doc = "GPR5 General Purpose Register"]
pub mod GPR5 {
    #[doc = "ENET1G TX_CLK select"]
    pub mod ENET1G_TX_CLK_SEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ENET1G_REF_CLK direction control"]
    pub mod ENET1G_REF_CLK_DIR {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ENET1G RGMII TX clock output enable"]
    pub mod ENET1G_RGMII_EN {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ENET1G master timer source select"]
    pub mod ENET1G_TIME_SEL {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ENET1G ENET_1588_EVENT0_IN source select"]
    pub mod ENET1G_EVENT0IN_SEL {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Domain write protection"]
    pub mod DWP {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Both cores are allowed"]
            pub const FORBID_NONE: u32 = 0;
            #[doc = "CM7 is forbidden"]
            pub const FORBID_CM7: u32 = 0x01;
            #[doc = "CM4 is forbidden"]
            pub const FORBID_CM4: u32 = 0x02;
            #[doc = "Both cores are forbidden"]
            pub const FORBID_BOTH: u32 = 0x03;
        }
    }
    #[doc = "Domain write protection lock"]
    pub mod DWP_LOCK {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Neither of DWP bits is locked"]
            pub const LOCK_NONE: u32 = 0;
            #[doc = "The lower DWP bit is locked"]
            pub const LOCK_LOW: u32 = 0x01;
            #[doc = "The higher DWP bit is locked"]
            pub const LOCK_HIGH: u32 = 0x02;
            #[doc = "Both DWP bits are locked"]
            pub const LOCK_BOTH: u32 = 0x03;
        }
    }
}
#[doc = "GPR6 General Purpose Register"]
pub mod GPR6 {
    #[doc = "ENET_QOS_REF_CLK direction control"]
    pub mod ENET_QOS_REF_CLK_DIR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ENET_QOS RGMII TX clock output enable"]
    pub mod ENET_QOS_RGMII_EN {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ENET_QOS master timer source select"]
    pub mod ENET_QOS_TIME_SEL {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ENET_QOS PHY Interface Select"]
    pub mod ENET_QOS_INTF_SEL {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ENET_QOS clock generator enable"]
    pub mod ENET_QOS_CLKGEN_EN {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ENET_QOS ENET_1588_EVENT0_IN source select"]
    pub mod ENET_QOS_EVENT0IN_SEL {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Domain write protection"]
    pub mod DWP {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Both cores are allowed"]
            pub const FORBID_NONE: u32 = 0;
            #[doc = "CM7 is forbidden"]
            pub const FORBID_CM7: u32 = 0x01;
            #[doc = "CM4 is forbidden"]
            pub const FORBID_CM4: u32 = 0x02;
            #[doc = "Both cores are forbidden"]
            pub const FORBID_BOTH: u32 = 0x03;
        }
    }
    #[doc = "Domain write protection lock"]
    pub mod DWP_LOCK {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Neither of DWP bits is locked"]
            pub const LOCK_NONE: u32 = 0;
            #[doc = "The lower DWP bit is locked"]
            pub const LOCK_LOW: u32 = 0x01;
            #[doc = "The higher DWP bit is locked"]
            pub const LOCK_HIGH: u32 = 0x02;
            #[doc = "Both DWP bits are locked"]
            pub const LOCK_BOTH: u32 = 0x03;
        }
    }
}
#[doc = "GPR7 General Purpose Register"]
pub mod GPR7 {
    #[doc = "Global interrupt"]
    pub mod GINT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Domain write protection"]
    pub mod DWP {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Both cores are allowed"]
            pub const FORBID_NONE: u32 = 0;
            #[doc = "CM7 is forbidden"]
            pub const FORBID_CM7: u32 = 0x01;
            #[doc = "CM4 is forbidden"]
            pub const FORBID_CM4: u32 = 0x02;
            #[doc = "Both cores are forbidden"]
            pub const FORBID_BOTH: u32 = 0x03;
        }
    }
    #[doc = "Domain write protection lock"]
    pub mod DWP_LOCK {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Neither of DWP bits is locked"]
            pub const LOCK_NONE: u32 = 0;
            #[doc = "The lower DWP bit is locked"]
            pub const LOCK_LOW: u32 = 0x01;
            #[doc = "The higher DWP bit is locked"]
            pub const LOCK_HIGH: u32 = 0x02;
            #[doc = "Both DWP bits are locked"]
            pub const LOCK_BOTH: u32 = 0x03;
        }
    }
}
#[doc = "GPR8 General Purpose Register"]
pub mod GPR8 {
    #[doc = "WDOG1 timeout mask for WDOG_ANY"]
    pub mod WDOG1_MASK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Domain write protection"]
    pub mod DWP {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Both cores are allowed"]
            pub const FORBID_NONE: u32 = 0;
            #[doc = "CM7 is forbidden"]
            pub const FORBID_CM7: u32 = 0x01;
            #[doc = "CM4 is forbidden"]
            pub const FORBID_CM4: u32 = 0x02;
            #[doc = "Both cores are forbidden"]
            pub const FORBID_BOTH: u32 = 0x03;
        }
    }
    #[doc = "Domain write protection lock"]
    pub mod DWP_LOCK {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Neither of DWP bits is locked"]
            pub const LOCK_NONE: u32 = 0;
            #[doc = "The lower DWP bit is locked"]
            pub const LOCK_LOW: u32 = 0x01;
            #[doc = "The higher DWP bit is locked"]
            pub const LOCK_HIGH: u32 = 0x02;
            #[doc = "Both DWP bits are locked"]
            pub const LOCK_BOTH: u32 = 0x03;
        }
    }
}
#[doc = "GPR9 General Purpose Register"]
pub mod GPR9 {
    #[doc = "WDOG2 timeout mask for WDOG_ANY"]
    pub mod WDOG2_MASK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Domain write protection"]
    pub mod DWP {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Both cores are allowed"]
            pub const FORBID_NONE: u32 = 0;
            #[doc = "CM7 is forbidden"]
            pub const FORBID_CM7: u32 = 0x01;
            #[doc = "CM4 is forbidden"]
            pub const FORBID_CM4: u32 = 0x02;
            #[doc = "Both cores are forbidden"]
            pub const FORBID_BOTH: u32 = 0x03;
        }
    }
    #[doc = "Domain write protection lock"]
    pub mod DWP_LOCK {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Neither of DWP bits is locked"]
            pub const LOCK_NONE: u32 = 0;
            #[doc = "The lower DWP bit is locked"]
            pub const LOCK_LOW: u32 = 0x01;
            #[doc = "The higher DWP bit is locked"]
            pub const LOCK_HIGH: u32 = 0x02;
            #[doc = "Both DWP bits are locked"]
            pub const LOCK_BOTH: u32 = 0x03;
        }
    }
}
#[doc = "GPR10 General Purpose Register"]
pub mod GPR10 {
    #[doc = "Domain write protection"]
    pub mod DWP {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Both cores are allowed"]
            pub const FORBID_NONE: u32 = 0;
            #[doc = "CM7 is forbidden"]
            pub const FORBID_CM7: u32 = 0x01;
            #[doc = "CM4 is forbidden"]
            pub const FORBID_CM4: u32 = 0x02;
            #[doc = "Both cores are forbidden"]
            pub const FORBID_BOTH: u32 = 0x03;
        }
    }
    #[doc = "Domain write protection lock"]
    pub mod DWP_LOCK {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Neither of DWP bits is locked"]
            pub const LOCK_NONE: u32 = 0;
            #[doc = "The lower DWP bit is locked"]
            pub const LOCK_LOW: u32 = 0x01;
            #[doc = "The higher DWP bit is locked"]
            pub const LOCK_HIGH: u32 = 0x02;
            #[doc = "Both DWP bits are locked"]
            pub const LOCK_BOTH: u32 = 0x03;
        }
    }
}
#[doc = "GPR11 General Purpose Register"]
pub mod GPR11 {
    #[doc = "Domain write protection"]
    pub mod DWP {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Both cores are allowed"]
            pub const FORBID_NONE: u32 = 0;
            #[doc = "CM7 is forbidden"]
            pub const FORBID_CM7: u32 = 0x01;
            #[doc = "CM4 is forbidden"]
            pub const FORBID_CM4: u32 = 0x02;
            #[doc = "Both cores are forbidden"]
            pub const FORBID_BOTH: u32 = 0x03;
        }
    }
    #[doc = "Domain write protection lock"]
    pub mod DWP_LOCK {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Neither of DWP bits is locked"]
            pub const LOCK_NONE: u32 = 0;
            #[doc = "The lower DWP bit is locked"]
            pub const LOCK_LOW: u32 = 0x01;
            #[doc = "The higher DWP bit is locked"]
            pub const LOCK_HIGH: u32 = 0x02;
            #[doc = "Both DWP bits are locked"]
            pub const LOCK_BOTH: u32 = 0x03;
        }
    }
}
#[doc = "GPR12 General Purpose Register"]
pub mod GPR12 {
    #[doc = "QTIMER1 timer counter freeze"]
    pub mod QTIMER1_TMR_CNTS_FREEZE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "QTIMER1 TMR0 input select"]
    pub mod QTIMER1_TRM0_INPUT_SEL {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "QTIMER1 TMR1 input select"]
    pub mod QTIMER1_TRM1_INPUT_SEL {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "QTIMER1 TMR2 input select"]
    pub mod QTIMER1_TRM2_INPUT_SEL {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "QTIMER1 TMR3 input select"]
    pub mod QTIMER1_TRM3_INPUT_SEL {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Domain write protection"]
    pub mod DWP {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Both cores are allowed"]
            pub const FORBID_NONE: u32 = 0;
            #[doc = "CM7 is forbidden"]
            pub const FORBID_CM7: u32 = 0x01;
            #[doc = "CM4 is forbidden"]
            pub const FORBID_CM4: u32 = 0x02;
            #[doc = "Both cores are forbidden"]
            pub const FORBID_BOTH: u32 = 0x03;
        }
    }
    #[doc = "Domain write protection lock"]
    pub mod DWP_LOCK {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Neither of DWP bits is locked"]
            pub const LOCK_NONE: u32 = 0;
            #[doc = "The lower DWP bit is locked"]
            pub const LOCK_LOW: u32 = 0x01;
            #[doc = "The higher DWP bit is locked"]
            pub const LOCK_HIGH: u32 = 0x02;
            #[doc = "Both DWP bits are locked"]
            pub const LOCK_BOTH: u32 = 0x03;
        }
    }
}
#[doc = "GPR13 General Purpose Register"]
pub mod GPR13 {
    #[doc = "QTIMER2 timer counter freeze"]
    pub mod QTIMER2_TMR_CNTS_FREEZE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "QTIMER2 TMR0 input select"]
    pub mod QTIMER2_TRM0_INPUT_SEL {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "QTIMER2 TMR1 input select"]
    pub mod QTIMER2_TRM1_INPUT_SEL {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "QTIMER2 TMR2 input select"]
    pub mod QTIMER2_TRM2_INPUT_SEL {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "QTIMER2 TMR3 input select"]
    pub mod QTIMER2_TRM3_INPUT_SEL {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Domain write protection"]
    pub mod DWP {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Both cores are allowed"]
            pub const FORBID_NONE: u32 = 0;
            #[doc = "CM7 is forbidden"]
            pub const FORBID_CM7: u32 = 0x01;
            #[doc = "CM4 is forbidden"]
            pub const FORBID_CM4: u32 = 0x02;
            #[doc = "Both cores are forbidden"]
            pub const FORBID_BOTH: u32 = 0x03;
        }
    }
    #[doc = "Domain write protection lock"]
    pub mod DWP_LOCK {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Neither of DWP bits is locked"]
            pub const LOCK_NONE: u32 = 0;
            #[doc = "The lower DWP bit is locked"]
            pub const LOCK_LOW: u32 = 0x01;
            #[doc = "The higher DWP bit is locked"]
            pub const LOCK_HIGH: u32 = 0x02;
            #[doc = "Both DWP bits are locked"]
            pub const LOCK_BOTH: u32 = 0x03;
        }
    }
}
#[doc = "GPR14 General Purpose Register"]
pub mod GPR14 {
    #[doc = "QTIMER3 timer counter freeze"]
    pub mod QTIMER3_TMR_CNTS_FREEZE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "QTIMER3 TMR0 input select"]
    pub mod QTIMER3_TRM0_INPUT_SEL {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "QTIMER3 TMR1 input select"]
    pub mod QTIMER3_TRM1_INPUT_SEL {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "QTIMER3 TMR2 input select"]
    pub mod QTIMER3_TRM2_INPUT_SEL {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "QTIMER3 TMR3 input select"]
    pub mod QTIMER3_TRM3_INPUT_SEL {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Domain write protection"]
    pub mod DWP {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Both cores are allowed"]
            pub const FORBID_NONE: u32 = 0;
            #[doc = "CM7 is forbidden"]
            pub const FORBID_CM7: u32 = 0x01;
            #[doc = "CM4 is forbidden"]
            pub const FORBID_CM4: u32 = 0x02;
            #[doc = "Both cores are forbidden"]
            pub const FORBID_BOTH: u32 = 0x03;
        }
    }
    #[doc = "Domain write protection lock"]
    pub mod DWP_LOCK {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Neither of DWP bits is locked"]
            pub const LOCK_NONE: u32 = 0;
            #[doc = "The lower DWP bit is locked"]
            pub const LOCK_LOW: u32 = 0x01;
            #[doc = "The higher DWP bit is locked"]
            pub const LOCK_HIGH: u32 = 0x02;
            #[doc = "Both DWP bits are locked"]
            pub const LOCK_BOTH: u32 = 0x03;
        }
    }
}
#[doc = "GPR15 General Purpose Register"]
pub mod GPR15 {
    #[doc = "QTIMER4 timer counter freeze"]
    pub mod QTIMER4_TMR_CNTS_FREEZE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "QTIMER4 TMR0 input select"]
    pub mod QTIMER4_TRM0_INPUT_SEL {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "QTIMER4 TMR1 input select"]
    pub mod QTIMER4_TRM1_INPUT_SEL {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "QTIMER4 TMR2 input select"]
    pub mod QTIMER4_TRM2_INPUT_SEL {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "QTIMER4 TMR3 input select"]
    pub mod QTIMER4_TRM3_INPUT_SEL {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Domain write protection"]
    pub mod DWP {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Both cores are allowed"]
            pub const FORBID_NONE: u32 = 0;
            #[doc = "CM7 is forbidden"]
            pub const FORBID_CM7: u32 = 0x01;
            #[doc = "CM4 is forbidden"]
            pub const FORBID_CM4: u32 = 0x02;
            #[doc = "Both cores are forbidden"]
            pub const FORBID_BOTH: u32 = 0x03;
        }
    }
    #[doc = "Domain write protection lock"]
    pub mod DWP_LOCK {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Neither of DWP bits is locked"]
            pub const LOCK_NONE: u32 = 0;
            #[doc = "The lower DWP bit is locked"]
            pub const LOCK_LOW: u32 = 0x01;
            #[doc = "The higher DWP bit is locked"]
            pub const LOCK_HIGH: u32 = 0x02;
            #[doc = "Both DWP bits are locked"]
            pub const LOCK_BOTH: u32 = 0x03;
        }
    }
}
#[doc = "GPR16 General Purpose Register"]
pub mod GPR16 {
    #[doc = "FlexRAM bank config source select"]
    pub mod FLEXRAM_BANK_CFG_SEL {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CM7 platform AHB clock enable"]
    pub mod CM7_FORCE_HCLK_EN {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CM7 sleep request selection"]
    pub mod M7_GPC_SLEEP_SEL {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Domain write protection"]
    pub mod DWP {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Both cores are allowed"]
            pub const FORBID_NONE: u32 = 0;
            #[doc = "CM7 is forbidden"]
            pub const FORBID_CM7: u32 = 0x01;
            #[doc = "CM4 is forbidden"]
            pub const FORBID_CM4: u32 = 0x02;
            #[doc = "Both cores are forbidden"]
            pub const FORBID_BOTH: u32 = 0x03;
        }
    }
    #[doc = "Domain write protection lock"]
    pub mod DWP_LOCK {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Neither of DWP bits is locked"]
            pub const LOCK_NONE: u32 = 0;
            #[doc = "The lower DWP bit is locked"]
            pub const LOCK_LOW: u32 = 0x01;
            #[doc = "The higher DWP bit is locked"]
            pub const LOCK_HIGH: u32 = 0x02;
            #[doc = "Both DWP bits are locked"]
            pub const LOCK_BOTH: u32 = 0x03;
        }
    }
}
#[doc = "GPR17 General Purpose Register"]
pub mod GPR17 {
    #[doc = "FlexRAM bank config value"]
    pub mod FLEXRAM_BANK_CFG_LOW {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Domain write protection"]
    pub mod DWP {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Both cores are allowed"]
            pub const FORBID_NONE: u32 = 0;
            #[doc = "CM7 is forbidden"]
            pub const FORBID_CM7: u32 = 0x01;
            #[doc = "CM4 is forbidden"]
            pub const FORBID_CM4: u32 = 0x02;
            #[doc = "Both cores are forbidden"]
            pub const FORBID_BOTH: u32 = 0x03;
        }
    }
    #[doc = "Domain write protection lock"]
    pub mod DWP_LOCK {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Neither of DWP bits is locked"]
            pub const LOCK_NONE: u32 = 0;
            #[doc = "The lower DWP bit is locked"]
            pub const LOCK_LOW: u32 = 0x01;
            #[doc = "The higher DWP bit is locked"]
            pub const LOCK_HIGH: u32 = 0x02;
            #[doc = "Both DWP bits are locked"]
            pub const LOCK_BOTH: u32 = 0x03;
        }
    }
}
#[doc = "GPR18 General Purpose Register"]
pub mod GPR18 {
    #[doc = "FlexRAM bank config value"]
    pub mod FLEXRAM_BANK_CFG_HIGH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Domain write protection"]
    pub mod DWP {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Both cores are allowed"]
            pub const FORBID_NONE: u32 = 0;
            #[doc = "CM7 is forbidden"]
            pub const FORBID_CM7: u32 = 0x01;
            #[doc = "CM4 is forbidden"]
            pub const FORBID_CM4: u32 = 0x02;
            #[doc = "Both cores are forbidden"]
            pub const FORBID_BOTH: u32 = 0x03;
        }
    }
    #[doc = "Domain write protection lock"]
    pub mod DWP_LOCK {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Neither of DWP bits is locked"]
            pub const LOCK_NONE: u32 = 0;
            #[doc = "The lower DWP bit is locked"]
            pub const LOCK_LOW: u32 = 0x01;
            #[doc = "The higher DWP bit is locked"]
            pub const LOCK_HIGH: u32 = 0x02;
            #[doc = "Both DWP bits are locked"]
            pub const LOCK_BOTH: u32 = 0x03;
        }
    }
}
#[doc = "GPR20 General Purpose Register"]
pub mod GPR20 {
    #[doc = "IOMUXC XBAR_INOUT4 function direction select"]
    pub mod IOMUXC_XBAR_DIR_SEL_4 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "IOMUXC XBAR_INOUT5 function direction select"]
    pub mod IOMUXC_XBAR_DIR_SEL_5 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "IOMUXC XBAR_INOUT6 function direction select"]
    pub mod IOMUXC_XBAR_DIR_SEL_6 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "IOMUXC XBAR_INOUT7 function direction select"]
    pub mod IOMUXC_XBAR_DIR_SEL_7 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "IOMUXC XBAR_INOUT8 function direction select"]
    pub mod IOMUXC_XBAR_DIR_SEL_8 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "IOMUXC XBAR_INOUT9 function direction select"]
    pub mod IOMUXC_XBAR_DIR_SEL_9 {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "IOMUXC XBAR_INOUT10 function direction select"]
    pub mod IOMUXC_XBAR_DIR_SEL_10 {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "IOMUXC XBAR_INOUT11 function direction select"]
    pub mod IOMUXC_XBAR_DIR_SEL_11 {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "IOMUXC XBAR_INOUT12 function direction select"]
    pub mod IOMUXC_XBAR_DIR_SEL_12 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "IOMUXC XBAR_INOUT13 function direction select"]
    pub mod IOMUXC_XBAR_DIR_SEL_13 {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "IOMUXC XBAR_INOUT14 function direction select"]
    pub mod IOMUXC_XBAR_DIR_SEL_14 {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "IOMUXC XBAR_INOUT15 function direction select"]
    pub mod IOMUXC_XBAR_DIR_SEL_15 {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "IOMUXC XBAR_INOUT16 function direction select"]
    pub mod IOMUXC_XBAR_DIR_SEL_16 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "IOMUXC XBAR_INOUT17 function direction select"]
    pub mod IOMUXC_XBAR_DIR_SEL_17 {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "IOMUXC XBAR_INOUT18 function direction select"]
    pub mod IOMUXC_XBAR_DIR_SEL_18 {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "IOMUXC XBAR_INOUT19 function direction select"]
    pub mod IOMUXC_XBAR_DIR_SEL_19 {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "IOMUXC XBAR_INOUT20 function direction select"]
    pub mod IOMUXC_XBAR_DIR_SEL_20 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "IOMUXC XBAR_INOUT21 function direction select"]
    pub mod IOMUXC_XBAR_DIR_SEL_21 {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "IOMUXC XBAR_INOUT22 function direction select"]
    pub mod IOMUXC_XBAR_DIR_SEL_22 {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "IOMUXC XBAR_INOUT23 function direction select"]
    pub mod IOMUXC_XBAR_DIR_SEL_23 {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "IOMUXC XBAR_INOUT24 function direction select"]
    pub mod IOMUXC_XBAR_DIR_SEL_24 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "IOMUXC XBAR_INOUT25 function direction select"]
    pub mod IOMUXC_XBAR_DIR_SEL_25 {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "IOMUXC XBAR_INOUT26 function direction select"]
    pub mod IOMUXC_XBAR_DIR_SEL_26 {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "IOMUXC XBAR_INOUT27 function direction select"]
    pub mod IOMUXC_XBAR_DIR_SEL_27 {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "IOMUXC XBAR_INOUT28 function direction select"]
    pub mod IOMUXC_XBAR_DIR_SEL_28 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "IOMUXC XBAR_INOUT29 function direction select"]
    pub mod IOMUXC_XBAR_DIR_SEL_29 {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "IOMUXC XBAR_INOUT30 function direction select"]
    pub mod IOMUXC_XBAR_DIR_SEL_30 {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "IOMUXC XBAR_INOUT31 function direction select"]
    pub mod IOMUXC_XBAR_DIR_SEL_31 {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Domain write protection"]
    pub mod DWP {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Both cores are allowed"]
            pub const FORBID_NONE: u32 = 0;
            #[doc = "CM7 is forbidden"]
            pub const FORBID_CM7: u32 = 0x01;
            #[doc = "CM4 is forbidden"]
            pub const FORBID_CM4: u32 = 0x02;
            #[doc = "Both cores are forbidden"]
            pub const FORBID_BOTH: u32 = 0x03;
        }
    }
    #[doc = "Domain write protection lock"]
    pub mod DWP_LOCK {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Neither of DWP bits is locked"]
            pub const LOCK_NONE: u32 = 0;
            #[doc = "The lower DWP bit is locked"]
            pub const LOCK_LOW: u32 = 0x01;
            #[doc = "The higher DWP bit is locked"]
            pub const LOCK_HIGH: u32 = 0x02;
            #[doc = "Both DWP bits are locked"]
            pub const LOCK_BOTH: u32 = 0x03;
        }
    }
}
#[doc = "GPR21 General Purpose Register"]
pub mod GPR21 {
    #[doc = "IOMUXC XBAR_INOUT32 function direction select"]
    pub mod IOMUXC_XBAR_DIR_SEL_32 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "IOMUXC XBAR_INOUT33 function direction select"]
    pub mod IOMUXC_XBAR_DIR_SEL_33 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "IOMUXC XBAR_INOUT34 function direction select"]
    pub mod IOMUXC_XBAR_DIR_SEL_34 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "IOMUXC XBAR_INOUT35 function direction select"]
    pub mod IOMUXC_XBAR_DIR_SEL_35 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "IOMUXC XBAR_INOUT36 function direction select"]
    pub mod IOMUXC_XBAR_DIR_SEL_36 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "IOMUXC XBAR_INOUT37 function direction select"]
    pub mod IOMUXC_XBAR_DIR_SEL_37 {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "IOMUXC XBAR_INOUT38 function direction select"]
    pub mod IOMUXC_XBAR_DIR_SEL_38 {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "IOMUXC XBAR_INOUT39 function direction select"]
    pub mod IOMUXC_XBAR_DIR_SEL_39 {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "IOMUXC XBAR_INOUT40 function direction select"]
    pub mod IOMUXC_XBAR_DIR_SEL_40 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "IOMUXC XBAR_INOUT41 function direction select"]
    pub mod IOMUXC_XBAR_DIR_SEL_41 {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "IOMUXC XBAR_INOUT42 function direction select"]
    pub mod IOMUXC_XBAR_DIR_SEL_42 {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Domain write protection"]
    pub mod DWP {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Both cores are allowed"]
            pub const FORBID_NONE: u32 = 0;
            #[doc = "CM7 is forbidden"]
            pub const FORBID_CM7: u32 = 0x01;
            #[doc = "CM4 is forbidden"]
            pub const FORBID_CM4: u32 = 0x02;
            #[doc = "Both cores are forbidden"]
            pub const FORBID_BOTH: u32 = 0x03;
        }
    }
    #[doc = "Domain write protection lock"]
    pub mod DWP_LOCK {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Neither of DWP bits is locked"]
            pub const LOCK_NONE: u32 = 0;
            #[doc = "The lower DWP bit is locked"]
            pub const LOCK_LOW: u32 = 0x01;
            #[doc = "The higher DWP bit is locked"]
            pub const LOCK_HIGH: u32 = 0x02;
            #[doc = "Both DWP bits are locked"]
            pub const LOCK_BOTH: u32 = 0x03;
        }
    }
}
#[doc = "GPR22 General Purpose Register"]
pub mod GPR22 {
    #[doc = "GPT1 1 MHz clock source select"]
    pub mod REF_1M_CLK_GPT1 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Domain write protection"]
    pub mod DWP {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Both cores are allowed"]
            pub const FORBID_NONE: u32 = 0;
            #[doc = "CM7 is forbidden"]
            pub const FORBID_CM7: u32 = 0x01;
            #[doc = "CM4 is forbidden"]
            pub const FORBID_CM4: u32 = 0x02;
            #[doc = "Both cores are forbidden"]
            pub const FORBID_BOTH: u32 = 0x03;
        }
    }
    #[doc = "Domain write protection lock"]
    pub mod DWP_LOCK {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Neither of DWP bits is locked"]
            pub const LOCK_NONE: u32 = 0;
            #[doc = "The lower DWP bit is locked"]
            pub const LOCK_LOW: u32 = 0x01;
            #[doc = "The higher DWP bit is locked"]
            pub const LOCK_HIGH: u32 = 0x02;
            #[doc = "Both DWP bits are locked"]
            pub const LOCK_BOTH: u32 = 0x03;
        }
    }
}
#[doc = "GPR23 General Purpose Register"]
pub mod GPR23 {
    #[doc = "GPT2 1 MHz clock source select"]
    pub mod REF_1M_CLK_GPT2 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "GPT2 input capture channel 1 source select"]
    pub mod GPT2_CAPIN1_SEL {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "GPT2 input capture channel 2 source select"]
    pub mod GPT2_CAPIN2_SEL {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Domain write protection"]
    pub mod DWP {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Both cores are allowed"]
            pub const FORBID_NONE: u32 = 0;
            #[doc = "CM7 is forbidden"]
            pub const FORBID_CM7: u32 = 0x01;
            #[doc = "CM4 is forbidden"]
            pub const FORBID_CM4: u32 = 0x02;
            #[doc = "Both cores are forbidden"]
            pub const FORBID_BOTH: u32 = 0x03;
        }
    }
    #[doc = "Domain write protection lock"]
    pub mod DWP_LOCK {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Neither of DWP bits is locked"]
            pub const LOCK_NONE: u32 = 0;
            #[doc = "The lower DWP bit is locked"]
            pub const LOCK_LOW: u32 = 0x01;
            #[doc = "The higher DWP bit is locked"]
            pub const LOCK_HIGH: u32 = 0x02;
            #[doc = "Both DWP bits are locked"]
            pub const LOCK_BOTH: u32 = 0x03;
        }
    }
}
#[doc = "GPR24 General Purpose Register"]
pub mod GPR24 {
    #[doc = "GPT3 1 MHz clock source select"]
    pub mod REF_1M_CLK_GPT3 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "GPT3 input capture channel 1 source select"]
    pub mod GPT3_CAPIN1_SEL {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Domain write protection"]
    pub mod DWP {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Both cores are allowed"]
            pub const FORBID_NONE: u32 = 0;
            #[doc = "CM7 is forbidden"]
            pub const FORBID_CM7: u32 = 0x01;
            #[doc = "CM4 is forbidden"]
            pub const FORBID_CM4: u32 = 0x02;
            #[doc = "Both cores are forbidden"]
            pub const FORBID_BOTH: u32 = 0x03;
        }
    }
    #[doc = "Domain write protection lock"]
    pub mod DWP_LOCK {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Neither of DWP bits is locked"]
            pub const LOCK_NONE: u32 = 0;
            #[doc = "The lower DWP bit is locked"]
            pub const LOCK_LOW: u32 = 0x01;
            #[doc = "The higher DWP bit is locked"]
            pub const LOCK_HIGH: u32 = 0x02;
            #[doc = "Both DWP bits are locked"]
            pub const LOCK_BOTH: u32 = 0x03;
        }
    }
}
#[doc = "GPR25 General Purpose Register"]
pub mod GPR25 {
    #[doc = "GPT4 1 MHz clock source select"]
    pub mod REF_1M_CLK_GPT4 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Domain write protection"]
    pub mod DWP {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Both cores are allowed"]
            pub const FORBID_NONE: u32 = 0;
            #[doc = "CM7 is forbidden"]
            pub const FORBID_CM7: u32 = 0x01;
            #[doc = "CM4 is forbidden"]
            pub const FORBID_CM4: u32 = 0x02;
            #[doc = "Both cores are forbidden"]
            pub const FORBID_BOTH: u32 = 0x03;
        }
    }
    #[doc = "Domain write protection lock"]
    pub mod DWP_LOCK {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Neither of DWP bits is locked"]
            pub const LOCK_NONE: u32 = 0;
            #[doc = "The lower DWP bit is locked"]
            pub const LOCK_LOW: u32 = 0x01;
            #[doc = "The higher DWP bit is locked"]
            pub const LOCK_HIGH: u32 = 0x02;
            #[doc = "Both DWP bits are locked"]
            pub const LOCK_BOTH: u32 = 0x03;
        }
    }
}
#[doc = "GPR26 General Purpose Register"]
pub mod GPR26 {
    #[doc = "GPT5 1 MHz clock source select"]
    pub mod REF_1M_CLK_GPT5 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Domain write protection"]
    pub mod DWP {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Both cores are allowed"]
            pub const FORBID_NONE: u32 = 0;
            #[doc = "CM7 is forbidden"]
            pub const FORBID_CM7: u32 = 0x01;
            #[doc = "CM4 is forbidden"]
            pub const FORBID_CM4: u32 = 0x02;
            #[doc = "Both cores are forbidden"]
            pub const FORBID_BOTH: u32 = 0x03;
        }
    }
    #[doc = "Domain write protection lock"]
    pub mod DWP_LOCK {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Neither of DWP bits is locked"]
            pub const LOCK_NONE: u32 = 0;
            #[doc = "The lower DWP bit is locked"]
            pub const LOCK_LOW: u32 = 0x01;
            #[doc = "The higher DWP bit is locked"]
            pub const LOCK_HIGH: u32 = 0x02;
            #[doc = "Both DWP bits are locked"]
            pub const LOCK_BOTH: u32 = 0x03;
        }
    }
}
#[doc = "GPR27 General Purpose Register"]
pub mod GPR27 {
    #[doc = "GPT6 1 MHz clock source select"]
    pub mod REF_1M_CLK_GPT6 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Domain write protection"]
    pub mod DWP {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Both cores are allowed"]
            pub const FORBID_NONE: u32 = 0;
            #[doc = "CM7 is forbidden"]
            pub const FORBID_CM7: u32 = 0x01;
            #[doc = "CM4 is forbidden"]
            pub const FORBID_CM4: u32 = 0x02;
            #[doc = "Both cores are forbidden"]
            pub const FORBID_BOTH: u32 = 0x03;
        }
    }
    #[doc = "Domain write protection lock"]
    pub mod DWP_LOCK {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Neither of DWP bits is locked"]
            pub const LOCK_NONE: u32 = 0;
            #[doc = "The lower DWP bit is locked"]
            pub const LOCK_LOW: u32 = 0x01;
            #[doc = "The higher DWP bit is locked"]
            pub const LOCK_HIGH: u32 = 0x02;
            #[doc = "Both DWP bits are locked"]
            pub const LOCK_BOTH: u32 = 0x03;
        }
    }
}
#[doc = "GPR28 General Purpose Register"]
pub mod GPR28 {
    #[doc = "uSDHC block cacheable attribute value of AXI read transactions"]
    pub mod ARCACHE_USDHC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "uSDHC block cacheable attribute value of AXI write transactions"]
    pub mod AWCACHE_USDHC {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "no description available"]
    pub mod CACHE_ENET1G {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ENET block cacheable attribute value of AXI transactions"]
    pub mod CACHE_ENET {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "USB block cacheable attribute value of AXI transactions"]
    pub mod CACHE_USB {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Domain write protection"]
    pub mod DWP {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Both cores are allowed"]
            pub const FORBID_NONE: u32 = 0;
            #[doc = "CM7 is forbidden"]
            pub const FORBID_CM7: u32 = 0x01;
            #[doc = "CM4 is forbidden"]
            pub const FORBID_CM4: u32 = 0x02;
            #[doc = "Both cores are forbidden"]
            pub const FORBID_BOTH: u32 = 0x03;
        }
    }
    #[doc = "Domain write protection lock"]
    pub mod DWP_LOCK {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Neither of DWP bits is locked"]
            pub const LOCK_NONE: u32 = 0;
            #[doc = "The lower DWP bit is locked"]
            pub const LOCK_LOW: u32 = 0x01;
            #[doc = "The higher DWP bit is locked"]
            pub const LOCK_HIGH: u32 = 0x02;
            #[doc = "Both DWP bits are locked"]
            pub const LOCK_BOTH: u32 = 0x03;
        }
    }
}
#[doc = "GPR29 General Purpose Register"]
pub mod GPR29 {
    #[doc = "USBPHY1 register access clock enable"]
    pub mod USBPHY1_IPG_CLK_ACTIVE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Domain write protection"]
    pub mod DWP {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Both cores are allowed"]
            pub const FORBID_NONE: u32 = 0;
            #[doc = "CM7 is forbidden"]
            pub const FORBID_CM7: u32 = 0x01;
            #[doc = "CM4 is forbidden"]
            pub const FORBID_CM4: u32 = 0x02;
            #[doc = "Both cores are forbidden"]
            pub const FORBID_BOTH: u32 = 0x03;
        }
    }
    #[doc = "Domain write protection lock"]
    pub mod DWP_LOCK {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Neither of DWP bits is locked"]
            pub const LOCK_NONE: u32 = 0;
            #[doc = "The lower DWP bit is locked"]
            pub const LOCK_LOW: u32 = 0x01;
            #[doc = "The higher DWP bit is locked"]
            pub const LOCK_HIGH: u32 = 0x02;
            #[doc = "Both DWP bits are locked"]
            pub const LOCK_BOTH: u32 = 0x03;
        }
    }
}
#[doc = "GPR30 General Purpose Register"]
pub mod GPR30 {
    #[doc = "USBPHY2 register access clock enable"]
    pub mod USBPHY2_IPG_CLK_ACTIVE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Domain write protection"]
    pub mod DWP {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Both cores are allowed"]
            pub const FORBID_NONE: u32 = 0;
            #[doc = "CM7 is forbidden"]
            pub const FORBID_CM7: u32 = 0x01;
            #[doc = "CM4 is forbidden"]
            pub const FORBID_CM4: u32 = 0x02;
            #[doc = "Both cores are forbidden"]
            pub const FORBID_BOTH: u32 = 0x03;
        }
    }
    #[doc = "Domain write protection lock"]
    pub mod DWP_LOCK {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Neither of DWP bits is locked"]
            pub const LOCK_NONE: u32 = 0;
            #[doc = "The lower DWP bit is locked"]
            pub const LOCK_LOW: u32 = 0x01;
            #[doc = "The higher DWP bit is locked"]
            pub const LOCK_HIGH: u32 = 0x02;
            #[doc = "Both DWP bits are locked"]
            pub const LOCK_BOTH: u32 = 0x03;
        }
    }
}
#[doc = "GPR31 General Purpose Register"]
pub mod GPR31 {
    #[doc = "OCRAM M7 RMW wait enable"]
    pub mod RMW2_WAIT_BVALID_CPL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "OCRAM M7 clock gating enable"]
    pub mod OCRAM_M7_CLK_GATING {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Domain write protection"]
    pub mod DWP {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Both cores are allowed"]
            pub const FORBID_NONE: u32 = 0;
            #[doc = "CM7 is forbidden"]
            pub const FORBID_CM7: u32 = 0x01;
            #[doc = "CM4 is forbidden"]
            pub const FORBID_CM4: u32 = 0x02;
            #[doc = "Both cores are forbidden"]
            pub const FORBID_BOTH: u32 = 0x03;
        }
    }
    #[doc = "Domain write protection lock"]
    pub mod DWP_LOCK {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Neither of DWP bits is locked"]
            pub const LOCK_NONE: u32 = 0;
            #[doc = "The lower DWP bit is locked"]
            pub const LOCK_LOW: u32 = 0x01;
            #[doc = "The higher DWP bit is locked"]
            pub const LOCK_HIGH: u32 = 0x02;
            #[doc = "Both DWP bits are locked"]
            pub const LOCK_BOTH: u32 = 0x03;
        }
    }
}
#[doc = "GPR32 General Purpose Register"]
pub mod GPR32 {
    #[doc = "OCRAM1 RMW wait enable"]
    pub mod RMW1_WAIT_BVALID_CPL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Domain write protection"]
    pub mod DWP {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Both cores are allowed"]
            pub const FORBID_NONE: u32 = 0;
            #[doc = "CM7 is forbidden"]
            pub const FORBID_CM7: u32 = 0x01;
            #[doc = "CM4 is forbidden"]
            pub const FORBID_CM4: u32 = 0x02;
            #[doc = "Both cores are forbidden"]
            pub const FORBID_BOTH: u32 = 0x03;
        }
    }
    #[doc = "Domain write protection lock"]
    pub mod DWP_LOCK {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Neither of DWP bits is locked"]
            pub const LOCK_NONE: u32 = 0;
            #[doc = "The lower DWP bit is locked"]
            pub const LOCK_LOW: u32 = 0x01;
            #[doc = "The higher DWP bit is locked"]
            pub const LOCK_HIGH: u32 = 0x02;
            #[doc = "Both DWP bits are locked"]
            pub const LOCK_BOTH: u32 = 0x03;
        }
    }
}
#[doc = "GPR33 General Purpose Register"]
pub mod GPR33 {
    #[doc = "OCRAM2 RMW wait enable"]
    pub mod RMW2_WAIT_BVALID_CPL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Domain write protection"]
    pub mod DWP {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Both cores are allowed"]
            pub const FORBID_NONE: u32 = 0;
            #[doc = "CM7 is forbidden"]
            pub const FORBID_CM7: u32 = 0x01;
            #[doc = "CM4 is forbidden"]
            pub const FORBID_CM4: u32 = 0x02;
            #[doc = "Both cores are forbidden"]
            pub const FORBID_BOTH: u32 = 0x03;
        }
    }
    #[doc = "Domain write protection lock"]
    pub mod DWP_LOCK {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Neither of DWP bits is locked"]
            pub const LOCK_NONE: u32 = 0;
            #[doc = "The lower DWP bit is locked"]
            pub const LOCK_LOW: u32 = 0x01;
            #[doc = "The higher DWP bit is locked"]
            pub const LOCK_HIGH: u32 = 0x02;
            #[doc = "Both DWP bits are locked"]
            pub const LOCK_BOTH: u32 = 0x03;
        }
    }
}
#[doc = "GPR34 General Purpose Register"]
pub mod GPR34 {
    #[doc = "XECC_FLEXSPI1 RMW wait enable"]
    pub mod XECC_FLEXSPI1_WAIT_BVALID_CPL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "FlexSPI1 OTFAD enable"]
    pub mod FLEXSPI1_OTFAD_EN {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Domain write protection"]
    pub mod DWP {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Both cores are allowed"]
            pub const FORBID_NONE: u32 = 0;
            #[doc = "CM7 is forbidden"]
            pub const FORBID_CM7: u32 = 0x01;
            #[doc = "CM4 is forbidden"]
            pub const FORBID_CM4: u32 = 0x02;
            #[doc = "Both cores are forbidden"]
            pub const FORBID_BOTH: u32 = 0x03;
        }
    }
    #[doc = "Domain write protection lock"]
    pub mod DWP_LOCK {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Neither of DWP bits is locked"]
            pub const LOCK_NONE: u32 = 0;
            #[doc = "The lower DWP bit is locked"]
            pub const LOCK_LOW: u32 = 0x01;
            #[doc = "The higher DWP bit is locked"]
            pub const LOCK_HIGH: u32 = 0x02;
            #[doc = "Both DWP bits are locked"]
            pub const LOCK_BOTH: u32 = 0x03;
        }
    }
}
#[doc = "GPR35 General Purpose Register"]
pub mod GPR35 {
    #[doc = "XECC_FLEXSPI2 RMW wait enable"]
    pub mod XECC_FLEXSPI2_WAIT_BVALID_CPL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "FlexSPI2 OTFAD enable"]
    pub mod FLEXSPI2_OTFAD_EN {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Domain write protection"]
    pub mod DWP {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Both cores are allowed"]
            pub const FORBID_NONE: u32 = 0;
            #[doc = "CM7 is forbidden"]
            pub const FORBID_CM7: u32 = 0x01;
            #[doc = "CM4 is forbidden"]
            pub const FORBID_CM4: u32 = 0x02;
            #[doc = "Both cores are forbidden"]
            pub const FORBID_BOTH: u32 = 0x03;
        }
    }
    #[doc = "Domain write protection lock"]
    pub mod DWP_LOCK {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Neither of DWP bits is locked"]
            pub const LOCK_NONE: u32 = 0;
            #[doc = "The lower DWP bit is locked"]
            pub const LOCK_LOW: u32 = 0x01;
            #[doc = "The higher DWP bit is locked"]
            pub const LOCK_HIGH: u32 = 0x02;
            #[doc = "Both DWP bits are locked"]
            pub const LOCK_BOTH: u32 = 0x03;
        }
    }
}
#[doc = "GPR36 General Purpose Register"]
pub mod GPR36 {
    #[doc = "XECC_SEMC RMW wait enable"]
    pub mod XECC_SEMC_WAIT_BVALID_CPL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Domain write protection"]
    pub mod DWP {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Both cores are allowed"]
            pub const FORBID_NONE: u32 = 0;
            #[doc = "CM7 is forbidden"]
            pub const FORBID_CM7: u32 = 0x01;
            #[doc = "CM4 is forbidden"]
            pub const FORBID_CM4: u32 = 0x02;
            #[doc = "Both cores are forbidden"]
            pub const FORBID_BOTH: u32 = 0x03;
        }
    }
    #[doc = "Domain write protection lock"]
    pub mod DWP_LOCK {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Neither of DWP bits is locked"]
            pub const LOCK_NONE: u32 = 0;
            #[doc = "The lower DWP bit is locked"]
            pub const LOCK_LOW: u32 = 0x01;
            #[doc = "The higher DWP bit is locked"]
            pub const LOCK_HIGH: u32 = 0x02;
            #[doc = "Both DWP bits are locked"]
            pub const LOCK_BOTH: u32 = 0x03;
        }
    }
}
#[doc = "GPR37 General Purpose Register"]
pub mod GPR37 {
    #[doc = "ARM non-secure (non-invasive) debug enable"]
    pub mod NIDEN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ARM invasive debug enable"]
    pub mod DBG_EN {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Exclusive monitor response select of illegal command"]
    pub mod EXC_MON {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CM7 debug halt mask"]
    pub mod M7_DBG_ACK_MASK {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CM4 debug halt mask"]
    pub mod M4_DBG_ACK_MASK {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Domain write protection"]
    pub mod DWP {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Both cores are allowed"]
            pub const FORBID_NONE: u32 = 0;
            #[doc = "CM7 is forbidden"]
            pub const FORBID_CM7: u32 = 0x01;
            #[doc = "CM4 is forbidden"]
            pub const FORBID_CM4: u32 = 0x02;
            #[doc = "Both cores are forbidden"]
            pub const FORBID_BOTH: u32 = 0x03;
        }
    }
    #[doc = "Domain write protection lock"]
    pub mod DWP_LOCK {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Neither of DWP bits is locked"]
            pub const LOCK_NONE: u32 = 0;
            #[doc = "The lower DWP bit is locked"]
            pub const LOCK_LOW: u32 = 0x01;
            #[doc = "The higher DWP bit is locked"]
            pub const LOCK_HIGH: u32 = 0x02;
            #[doc = "Both DWP bits are locked"]
            pub const LOCK_BOTH: u32 = 0x03;
        }
    }
}
#[doc = "GPR38 General Purpose Register"]
pub mod GPR38 {
    #[doc = "Domain write protection"]
    pub mod DWP {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Both cores are allowed"]
            pub const FORBID_NONE: u32 = 0;
            #[doc = "CM7 is forbidden"]
            pub const FORBID_CM7: u32 = 0x01;
            #[doc = "CM4 is forbidden"]
            pub const FORBID_CM4: u32 = 0x02;
            #[doc = "Both cores are forbidden"]
            pub const FORBID_BOTH: u32 = 0x03;
        }
    }
    #[doc = "Domain write protection lock"]
    pub mod DWP_LOCK {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Neither of DWP bits is locked"]
            pub const LOCK_NONE: u32 = 0;
            #[doc = "The lower DWP bit is locked"]
            pub const LOCK_LOW: u32 = 0x01;
            #[doc = "The higher DWP bit is locked"]
            pub const LOCK_HIGH: u32 = 0x02;
            #[doc = "Both DWP bits are locked"]
            pub const LOCK_BOTH: u32 = 0x03;
        }
    }
}
#[doc = "GPR39 General Purpose Register"]
pub mod GPR39 {
    #[doc = "Domain write protection"]
    pub mod DWP {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Both cores are allowed"]
            pub const FORBID_NONE: u32 = 0;
            #[doc = "CM7 is forbidden"]
            pub const FORBID_CM7: u32 = 0x01;
            #[doc = "CM4 is forbidden"]
            pub const FORBID_CM4: u32 = 0x02;
            #[doc = "Both cores are forbidden"]
            pub const FORBID_BOTH: u32 = 0x03;
        }
    }
    #[doc = "Domain write protection lock"]
    pub mod DWP_LOCK {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Neither of DWP bits is locked"]
            pub const LOCK_NONE: u32 = 0;
            #[doc = "The lower DWP bit is locked"]
            pub const LOCK_LOW: u32 = 0x01;
            #[doc = "The higher DWP bit is locked"]
            pub const LOCK_HIGH: u32 = 0x02;
            #[doc = "Both DWP bits are locked"]
            pub const LOCK_BOTH: u32 = 0x03;
        }
    }
}
#[doc = "GPR40 General Purpose Register"]
pub mod GPR40 {
    #[doc = "GPIO2 and CM7_GPIO2 share same IO MUX function, GPIO_MUX2 selects one GPIO function."]
    pub mod GPIO_MUX2_GPIO_SEL_LOW {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Domain write protection"]
    pub mod DWP {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Both cores are allowed"]
            pub const FORBID_NONE: u32 = 0;
            #[doc = "CM7 is forbidden"]
            pub const FORBID_CM7: u32 = 0x01;
            #[doc = "CM4 is forbidden"]
            pub const FORBID_CM4: u32 = 0x02;
            #[doc = "Both cores are forbidden"]
            pub const FORBID_BOTH: u32 = 0x03;
        }
    }
    #[doc = "Domain write protection lock"]
    pub mod DWP_LOCK {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Neither of DWP bits is locked"]
            pub const LOCK_NONE: u32 = 0;
            #[doc = "The lower DWP bit is locked"]
            pub const LOCK_LOW: u32 = 0x01;
            #[doc = "The higher DWP bit is locked"]
            pub const LOCK_HIGH: u32 = 0x02;
            #[doc = "Both DWP bits are locked"]
            pub const LOCK_BOTH: u32 = 0x03;
        }
    }
}
#[doc = "GPR41 General Purpose Register"]
pub mod GPR41 {
    #[doc = "GPIO2 and CM7_GPIO2 share same IO MUX function, GPIO_MUX2 selects one GPIO function."]
    pub mod GPIO_MUX2_GPIO_SEL_HIGH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Domain write protection"]
    pub mod DWP {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Both cores are allowed"]
            pub const FORBID_NONE: u32 = 0;
            #[doc = "CM7 is forbidden"]
            pub const FORBID_CM7: u32 = 0x01;
            #[doc = "CM4 is forbidden"]
            pub const FORBID_CM4: u32 = 0x02;
            #[doc = "Both cores are forbidden"]
            pub const FORBID_BOTH: u32 = 0x03;
        }
    }
    #[doc = "Domain write protection lock"]
    pub mod DWP_LOCK {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Neither of DWP bits is locked"]
            pub const LOCK_NONE: u32 = 0;
            #[doc = "The lower DWP bit is locked"]
            pub const LOCK_LOW: u32 = 0x01;
            #[doc = "The higher DWP bit is locked"]
            pub const LOCK_HIGH: u32 = 0x02;
            #[doc = "Both DWP bits are locked"]
            pub const LOCK_BOTH: u32 = 0x03;
        }
    }
}
#[doc = "GPR42 General Purpose Register"]
pub mod GPR42 {
    #[doc = "GPIO3 and CM7_GPIO3 share same IO MUX function, GPIO_MUX3 selects one GPIO function."]
    pub mod GPIO_MUX3_GPIO_SEL_LOW {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Domain write protection"]
    pub mod DWP {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Both cores are allowed"]
            pub const FORBID_NONE: u32 = 0;
            #[doc = "CM7 is forbidden"]
            pub const FORBID_CM7: u32 = 0x01;
            #[doc = "CM4 is forbidden"]
            pub const FORBID_CM4: u32 = 0x02;
            #[doc = "Both cores are forbidden"]
            pub const FORBID_BOTH: u32 = 0x03;
        }
    }
    #[doc = "Domain write protection lock"]
    pub mod DWP_LOCK {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Neither of DWP bits is locked"]
            pub const LOCK_NONE: u32 = 0;
            #[doc = "The lower DWP bit is locked"]
            pub const LOCK_LOW: u32 = 0x01;
            #[doc = "The higher DWP bit is locked"]
            pub const LOCK_HIGH: u32 = 0x02;
            #[doc = "Both DWP bits are locked"]
            pub const LOCK_BOTH: u32 = 0x03;
        }
    }
}
#[doc = "GPR43 General Purpose Register"]
pub mod GPR43 {
    #[doc = "GPIO3 and CM7_GPIO3 share same IO MUX function, GPIO_MUX3 selects one GPIO function."]
    pub mod GPIO_MUX3_GPIO_SEL_HIGH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Domain write protection"]
    pub mod DWP {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Both cores are allowed"]
            pub const FORBID_NONE: u32 = 0;
            #[doc = "CM7 is forbidden"]
            pub const FORBID_CM7: u32 = 0x01;
            #[doc = "CM4 is forbidden"]
            pub const FORBID_CM4: u32 = 0x02;
            #[doc = "Both cores are forbidden"]
            pub const FORBID_BOTH: u32 = 0x03;
        }
    }
    #[doc = "Domain write protection lock"]
    pub mod DWP_LOCK {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Neither of DWP bits is locked"]
            pub const LOCK_NONE: u32 = 0;
            #[doc = "The lower DWP bit is locked"]
            pub const LOCK_LOW: u32 = 0x01;
            #[doc = "The higher DWP bit is locked"]
            pub const LOCK_HIGH: u32 = 0x02;
            #[doc = "Both DWP bits are locked"]
            pub const LOCK_BOTH: u32 = 0x03;
        }
    }
}
#[doc = "GPR44 General Purpose Register"]
pub mod GPR44 {
    #[doc = "Domain write protection"]
    pub mod DWP {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Both cores are allowed"]
            pub const FORBID_NONE: u32 = 0;
            #[doc = "CM7 is forbidden"]
            pub const FORBID_CM7: u32 = 0x01;
            #[doc = "CM4 is forbidden"]
            pub const FORBID_CM4: u32 = 0x02;
            #[doc = "Both cores are forbidden"]
            pub const FORBID_BOTH: u32 = 0x03;
        }
    }
    #[doc = "Domain write protection lock"]
    pub mod DWP_LOCK {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Neither of DWP bits is locked"]
            pub const LOCK_NONE: u32 = 0;
            #[doc = "The lower DWP bit is locked"]
            pub const LOCK_LOW: u32 = 0x01;
            #[doc = "The higher DWP bit is locked"]
            pub const LOCK_HIGH: u32 = 0x02;
            #[doc = "Both DWP bits are locked"]
            pub const LOCK_BOTH: u32 = 0x03;
        }
    }
}
#[doc = "GPR45 General Purpose Register"]
pub mod GPR45 {
    #[doc = "Domain write protection"]
    pub mod DWP {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Both cores are allowed"]
            pub const FORBID_NONE: u32 = 0;
            #[doc = "CM7 is forbidden"]
            pub const FORBID_CM7: u32 = 0x01;
            #[doc = "CM4 is forbidden"]
            pub const FORBID_CM4: u32 = 0x02;
            #[doc = "Both cores are forbidden"]
            pub const FORBID_BOTH: u32 = 0x03;
        }
    }
    #[doc = "Domain write protection lock"]
    pub mod DWP_LOCK {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Neither of DWP bits is locked"]
            pub const LOCK_NONE: u32 = 0;
            #[doc = "The lower DWP bit is locked"]
            pub const LOCK_LOW: u32 = 0x01;
            #[doc = "The higher DWP bit is locked"]
            pub const LOCK_HIGH: u32 = 0x02;
            #[doc = "Both DWP bits are locked"]
            pub const LOCK_BOTH: u32 = 0x03;
        }
    }
}
#[doc = "GPR46 General Purpose Register"]
pub mod GPR46 {
    #[doc = "Domain write protection"]
    pub mod DWP {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Both cores are allowed"]
            pub const FORBID_NONE: u32 = 0;
            #[doc = "CM7 is forbidden"]
            pub const FORBID_CM7: u32 = 0x01;
            #[doc = "CM4 is forbidden"]
            pub const FORBID_CM4: u32 = 0x02;
            #[doc = "Both cores are forbidden"]
            pub const FORBID_BOTH: u32 = 0x03;
        }
    }
    #[doc = "Domain write protection lock"]
    pub mod DWP_LOCK {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Neither of DWP bits is locked"]
            pub const LOCK_NONE: u32 = 0;
            #[doc = "The lower DWP bit is locked"]
            pub const LOCK_LOW: u32 = 0x01;
            #[doc = "The higher DWP bit is locked"]
            pub const LOCK_HIGH: u32 = 0x02;
            #[doc = "Both DWP bits are locked"]
            pub const LOCK_BOTH: u32 = 0x03;
        }
    }
}
#[doc = "GPR47 General Purpose Register"]
pub mod GPR47 {
    #[doc = "Domain write protection"]
    pub mod DWP {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Both cores are allowed"]
            pub const FORBID_NONE: u32 = 0;
            #[doc = "CM7 is forbidden"]
            pub const FORBID_CM7: u32 = 0x01;
            #[doc = "CM4 is forbidden"]
            pub const FORBID_CM4: u32 = 0x02;
            #[doc = "Both cores are forbidden"]
            pub const FORBID_BOTH: u32 = 0x03;
        }
    }
    #[doc = "Domain write protection lock"]
    pub mod DWP_LOCK {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Neither of DWP bits is locked"]
            pub const LOCK_NONE: u32 = 0;
            #[doc = "The lower DWP bit is locked"]
            pub const LOCK_LOW: u32 = 0x01;
            #[doc = "The higher DWP bit is locked"]
            pub const LOCK_HIGH: u32 = 0x02;
            #[doc = "Both DWP bits are locked"]
            pub const LOCK_BOTH: u32 = 0x03;
        }
    }
}
#[doc = "GPR48 General Purpose Register"]
pub mod GPR48 {
    #[doc = "Domain write protection"]
    pub mod DWP {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Both cores are allowed"]
            pub const FORBID_NONE: u32 = 0;
            #[doc = "CM7 is forbidden"]
            pub const FORBID_CM7: u32 = 0x01;
            #[doc = "CM4 is forbidden"]
            pub const FORBID_CM4: u32 = 0x02;
            #[doc = "Both cores are forbidden"]
            pub const FORBID_BOTH: u32 = 0x03;
        }
    }
    #[doc = "Domain write protection lock"]
    pub mod DWP_LOCK {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Neither of DWP bits is locked"]
            pub const LOCK_NONE: u32 = 0;
            #[doc = "The lower DWP bit is locked"]
            pub const LOCK_LOW: u32 = 0x01;
            #[doc = "The higher DWP bit is locked"]
            pub const LOCK_HIGH: u32 = 0x02;
            #[doc = "Both DWP bits are locked"]
            pub const LOCK_BOTH: u32 = 0x03;
        }
    }
}
#[doc = "GPR49 General Purpose Register"]
pub mod GPR49 {
    #[doc = "Domain write protection"]
    pub mod DWP {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Both cores are allowed"]
            pub const FORBID_NONE: u32 = 0;
            #[doc = "CM7 is forbidden"]
            pub const FORBID_CM7: u32 = 0x01;
            #[doc = "CM4 is forbidden"]
            pub const FORBID_CM4: u32 = 0x02;
            #[doc = "Both cores are forbidden"]
            pub const FORBID_BOTH: u32 = 0x03;
        }
    }
    #[doc = "Domain write protection lock"]
    pub mod DWP_LOCK {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Neither of DWP bits is locked"]
            pub const LOCK_NONE: u32 = 0;
            #[doc = "The lower DWP bit is locked"]
            pub const LOCK_LOW: u32 = 0x01;
            #[doc = "The higher DWP bit is locked"]
            pub const LOCK_HIGH: u32 = 0x02;
            #[doc = "Both DWP bits are locked"]
            pub const LOCK_BOTH: u32 = 0x03;
        }
    }
}
#[doc = "GPR50 General Purpose Register"]
pub mod GPR50 {
    #[doc = "CAAM manager processor identifier"]
    pub mod CAAM_IPS_MGR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Domain write protection"]
    pub mod DWP {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Both cores are allowed"]
            pub const FORBID_NONE: u32 = 0;
            #[doc = "CM7 is forbidden"]
            pub const FORBID_CM7: u32 = 0x01;
            #[doc = "CM4 is forbidden"]
            pub const FORBID_CM4: u32 = 0x02;
            #[doc = "Both cores are forbidden"]
            pub const FORBID_BOTH: u32 = 0x03;
        }
    }
    #[doc = "Domain write protection lock"]
    pub mod DWP_LOCK {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Neither of DWP bits is locked"]
            pub const LOCK_NONE: u32 = 0;
            #[doc = "The lower DWP bit is locked"]
            pub const LOCK_LOW: u32 = 0x01;
            #[doc = "The higher DWP bit is locked"]
            pub const LOCK_HIGH: u32 = 0x02;
            #[doc = "Both DWP bits are locked"]
            pub const LOCK_BOTH: u32 = 0x03;
        }
    }
}
#[doc = "GPR51 General Purpose Register"]
pub mod GPR51 {
    #[doc = "Clear CM7 NMI holding register"]
    pub mod M7_NMI_CLEAR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Domain write protection"]
    pub mod DWP {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Both cores are allowed"]
            pub const FORBID_NONE: u32 = 0;
            #[doc = "CM7 is forbidden"]
            pub const FORBID_CM7: u32 = 0x01;
            #[doc = "CM4 is forbidden"]
            pub const FORBID_CM4: u32 = 0x02;
            #[doc = "Both cores are forbidden"]
            pub const FORBID_BOTH: u32 = 0x03;
        }
    }
    #[doc = "Domain write protection lock"]
    pub mod DWP_LOCK {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Neither of DWP bits is locked"]
            pub const LOCK_NONE: u32 = 0;
            #[doc = "The lower DWP bit is locked"]
            pub const LOCK_LOW: u32 = 0x01;
            #[doc = "The higher DWP bit is locked"]
            pub const LOCK_HIGH: u32 = 0x02;
            #[doc = "Both DWP bits are locked"]
            pub const LOCK_BOTH: u32 = 0x03;
        }
    }
}
#[doc = "GPR52 General Purpose Register"]
pub mod GPR52 {
    #[doc = "Domain write protection"]
    pub mod DWP {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Both cores are allowed"]
            pub const FORBID_NONE: u32 = 0;
            #[doc = "CM7 is forbidden"]
            pub const FORBID_CM7: u32 = 0x01;
            #[doc = "CM4 is forbidden"]
            pub const FORBID_CM4: u32 = 0x02;
            #[doc = "Both cores are forbidden"]
            pub const FORBID_BOTH: u32 = 0x03;
        }
    }
    #[doc = "Domain write protection lock"]
    pub mod DWP_LOCK {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Neither of DWP bits is locked"]
            pub const LOCK_NONE: u32 = 0;
            #[doc = "The lower DWP bit is locked"]
            pub const LOCK_LOW: u32 = 0x01;
            #[doc = "The higher DWP bit is locked"]
            pub const LOCK_HIGH: u32 = 0x02;
            #[doc = "Both DWP bits are locked"]
            pub const LOCK_BOTH: u32 = 0x03;
        }
    }
}
#[doc = "GPR53 General Purpose Register"]
pub mod GPR53 {
    #[doc = "Domain write protection"]
    pub mod DWP {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Both cores are allowed"]
            pub const FORBID_NONE: u32 = 0;
            #[doc = "CM7 is forbidden"]
            pub const FORBID_CM7: u32 = 0x01;
            #[doc = "CM4 is forbidden"]
            pub const FORBID_CM4: u32 = 0x02;
            #[doc = "Both cores are forbidden"]
            pub const FORBID_BOTH: u32 = 0x03;
        }
    }
    #[doc = "Domain write protection lock"]
    pub mod DWP_LOCK {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Neither of DWP bits is locked"]
            pub const LOCK_NONE: u32 = 0;
            #[doc = "The lower DWP bit is locked"]
            pub const LOCK_LOW: u32 = 0x01;
            #[doc = "The higher DWP bit is locked"]
            pub const LOCK_HIGH: u32 = 0x02;
            #[doc = "Both DWP bits are locked"]
            pub const LOCK_BOTH: u32 = 0x03;
        }
    }
}
#[doc = "GPR54 General Purpose Register"]
pub mod GPR54 {
    #[doc = "Domain write protection"]
    pub mod DWP {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Both cores are allowed"]
            pub const FORBID_NONE: u32 = 0;
            #[doc = "CM7 is forbidden"]
            pub const FORBID_CM7: u32 = 0x01;
            #[doc = "CM4 is forbidden"]
            pub const FORBID_CM4: u32 = 0x02;
            #[doc = "Both cores are forbidden"]
            pub const FORBID_BOTH: u32 = 0x03;
        }
    }
    #[doc = "Domain write protection lock"]
    pub mod DWP_LOCK {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Neither of DWP bits is locked"]
            pub const LOCK_NONE: u32 = 0;
            #[doc = "The lower DWP bit is locked"]
            pub const LOCK_LOW: u32 = 0x01;
            #[doc = "The higher DWP bit is locked"]
            pub const LOCK_HIGH: u32 = 0x02;
            #[doc = "Both DWP bits are locked"]
            pub const LOCK_BOTH: u32 = 0x03;
        }
    }
}
#[doc = "GPR55 General Purpose Register"]
pub mod GPR55 {
    #[doc = "Domain write protection"]
    pub mod DWP {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Both cores are allowed"]
            pub const FORBID_NONE: u32 = 0;
            #[doc = "CM7 is forbidden"]
            pub const FORBID_CM7: u32 = 0x01;
            #[doc = "CM4 is forbidden"]
            pub const FORBID_CM4: u32 = 0x02;
            #[doc = "Both cores are forbidden"]
            pub const FORBID_BOTH: u32 = 0x03;
        }
    }
    #[doc = "Domain write protection lock"]
    pub mod DWP_LOCK {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Neither of DWP bits is locked"]
            pub const LOCK_NONE: u32 = 0;
            #[doc = "The lower DWP bit is locked"]
            pub const LOCK_LOW: u32 = 0x01;
            #[doc = "The higher DWP bit is locked"]
            pub const LOCK_HIGH: u32 = 0x02;
            #[doc = "Both DWP bits are locked"]
            pub const LOCK_BOTH: u32 = 0x03;
        }
    }
}
#[doc = "GPR59 General Purpose Register"]
pub mod GPR59 {
    #[doc = "Powers down inactive lanes reported by CSI2X_CFG_NUM_LANES."]
    pub mod MIPI_CSI_AUTO_PD_EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "MIPI CSI APB clock domain and User interface clock domain software reset bit"]
    pub mod MIPI_CSI_SOFT_RST_N {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Assert reset"]
            pub const ASSERT: u32 = 0;
            #[doc = "De-assert reset"]
            pub const DEAST: u32 = 0x01;
        }
    }
    #[doc = "Enables the slave clock lane feature to maintain HS reception state during continuous clock mode operation, despite line glitches."]
    pub mod MIPI_CSI_CONT_CLK_MODE {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "When high, enables received DDR clock on CLK_DRXHS"]
    pub mod MIPI_CSI_DDRCLK_EN {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Power Down input for MIPI CSI PHY."]
    pub mod MIPI_CSI_PD_RX {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Assert to enable MIPI CSI Receive Enable"]
    pub mod MIPI_CSI_RX_ENABLE {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "MIPI CSI PHY on-chip termination control bits"]
    pub mod MIPI_CSI_RX_RCAL {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Programming bits that adjust the threshold voltage of LP-CD, default setting 2'b01"]
    pub mod MIPI_CSI_RXCDRP {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "344mV"]
            pub const VAL0: u32 = 0;
            #[doc = "325mV (Default)"]
            pub const VAL01: u32 = 0x01;
            #[doc = "307mV"]
            pub const VAL10: u32 = 0x02;
            #[doc = "Invalid"]
            pub const VAL11: u32 = 0x03;
        }
    }
    #[doc = "Programming bits that adjust the threshold voltage of LP-RX, default setting 2'b01"]
    pub mod MIPI_CSI_RXLPRP {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Bits used to program T_HS_SETTLE."]
    pub mod MIPI_CSI_S_PRG_RXHS_SETTLE {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Domain write protection"]
    pub mod DWP {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Both cores are allowed"]
            pub const FORBID_NONE: u32 = 0;
            #[doc = "CM7 is forbidden"]
            pub const FORBID_CM7: u32 = 0x01;
            #[doc = "CM4 is forbidden"]
            pub const FORBID_CM4: u32 = 0x02;
            #[doc = "Both cores are forbidden"]
            pub const FORBID_BOTH: u32 = 0x03;
        }
    }
    #[doc = "Domain write protection lock"]
    pub mod DWP_LOCK {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Neither of DWP bits is locked"]
            pub const LOCK_NONE: u32 = 0;
            #[doc = "The lower DWP bit is locked"]
            pub const LOCK_LOW: u32 = 0x01;
            #[doc = "The higher DWP bit is locked"]
            pub const LOCK_HIGH: u32 = 0x02;
            #[doc = "Both DWP bits are locked"]
            pub const LOCK_BOTH: u32 = 0x03;
        }
    }
}
#[doc = "GPR62 General Purpose Register"]
pub mod GPR62 {
    #[doc = "MIPI DSI Clock Lane triming bits"]
    pub mod MIPI_DSI_CLK_TM {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "MIPI DSI Data Lane 0 triming bits"]
    pub mod MIPI_DSI_D0_TM {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "MIPI DSI Data Lane 1 triming bits"]
    pub mod MIPI_DSI_D1_TM {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "MIPI DSI PHY on-chip termination control bits"]
    pub mod MIPI_DSI_TX_RCAL {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DSI transmit ULPS mode enable"]
    pub mod MIPI_DSI_TX_ULPS_ENABLE {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "MIPI DSI APB clock domain software reset bit"]
    pub mod MIPI_DSI_PCLK_SOFT_RESET_N {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Assert reset"]
            pub const ASSERT: u32 = 0;
            #[doc = "De-assert reset"]
            pub const DEASSERT: u32 = 0x01;
        }
    }
    #[doc = "MIPI DSI Byte clock domain software reset bit"]
    pub mod MIPI_DSI_BYTE_SOFT_RESET_N {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Assert reset"]
            pub const ASSERT: u32 = 0;
            #[doc = "De-assert reset"]
            pub const DEASSERT: u32 = 0x01;
        }
    }
    #[doc = "MIPI DSI Pixel clock domain software reset bit"]
    pub mod MIPI_DSI_DPI_SOFT_RESET_N {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Assert reset"]
            pub const ASSERT: u32 = 0;
            #[doc = "De-assert reset"]
            pub const DEASSERT: u32 = 0x01;
        }
    }
    #[doc = "MIPI DSI Escape clock domain software reset bit"]
    pub mod MIPI_DSI_ESC_SOFT_RESET_N {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Assert reset"]
            pub const ASSERT: u32 = 0;
            #[doc = "De-assert reset"]
            pub const DEASSERT: u32 = 0x01;
        }
    }
    #[doc = "Domain write protection"]
    pub mod DWP {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Both cores are allowed"]
            pub const FORBID_NONE: u32 = 0;
            #[doc = "CM7 is forbidden"]
            pub const FORBID_CM7: u32 = 0x01;
            #[doc = "CM4 is forbidden"]
            pub const FORBID_CM4: u32 = 0x02;
            #[doc = "Both cores are forbidden"]
            pub const FORBID_BOTH: u32 = 0x03;
        }
    }
    #[doc = "Domain write protection lock"]
    pub mod DWP_LOCK {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Neither of DWP bits is locked"]
            pub const LOCK_NONE: u32 = 0;
            #[doc = "The lower DWP bit is locked"]
            pub const LOCK_LOW: u32 = 0x01;
            #[doc = "The higher DWP bit is locked"]
            pub const LOCK_HIGH: u32 = 0x02;
            #[doc = "Both DWP bits are locked"]
            pub const LOCK_BOTH: u32 = 0x03;
        }
    }
}
#[doc = "GPR63 General Purpose Register"]
pub mod GPR63 {
    #[doc = "DSI transmit ULPS mode active flag"]
    pub mod MIPI_DSI_TX_ULPS_ACTIVE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPR64 General Purpose Register"]
pub mod GPR64 {
    #[doc = "Compensation code freeze"]
    pub mod GPIO_DISP1_FREEZE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "COMPEN and COMPTQ control the operating modes of the compensation cell"]
    pub mod GPIO_DISP1_COMPTQ {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "COMPEN and COMPTQ control the operating modes of the compensation cell"]
    pub mod GPIO_DISP1_COMPEN {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Compensation code fast freeze"]
    pub mod GPIO_DISP1_FASTFRZ_EN {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "GPIO_DISP_B1 IO bank's 4-bit PMOS compensation codes from core"]
    pub mod GPIO_DISP1_RASRCP {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "GPIO_DISP_B1 IO bank's 4-bit NMOS compensation codes from core"]
    pub mod GPIO_DISP1_RASRCN {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "GPIO_DISP1_NASRC selection"]
    pub mod GPIO_DISP1_SELECT_NASRC {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "GPIO_DISP_B1 IO bank reference voltage generator cell sleep enable"]
    pub mod GPIO_DISP1_REFGEN_SLEEP {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "GPIO_DISP_B1 IO bank power supply mode latch enable"]
    pub mod GPIO_DISP1_SUPLYDET_LATCH {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "GPIO_DISP_B1 IO bank compensation OK flag"]
    pub mod GPIO_DISP1_COMPOK {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "GPIO_DISP_B1 IO bank compensation codes"]
    pub mod GPIO_DISP1_NASRC {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Domain write protection"]
    pub mod DWP {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Both cores are allowed"]
            pub const FORBID_NONE: u32 = 0;
            #[doc = "CM7 is forbidden"]
            pub const FORBID_CM7: u32 = 0x01;
            #[doc = "CM4 is forbidden"]
            pub const FORBID_CM4: u32 = 0x02;
            #[doc = "Both cores are forbidden"]
            pub const FORBID_BOTH: u32 = 0x03;
        }
    }
    #[doc = "Domain write protection lock"]
    pub mod DWP_LOCK {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Neither of DWP bits is locked"]
            pub const LOCK_NONE: u32 = 0;
            #[doc = "The lower DWP bit is locked"]
            pub const LOCK_LOW: u32 = 0x01;
            #[doc = "The higher DWP bit is locked"]
            pub const LOCK_HIGH: u32 = 0x02;
            #[doc = "Both DWP bits are locked"]
            pub const LOCK_BOTH: u32 = 0x03;
        }
    }
}
#[doc = "GPR65 General Purpose Register"]
pub mod GPR65 {
    #[doc = "Compensation code freeze"]
    pub mod GPIO_EMC1_FREEZE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "COMPEN and COMPTQ control the operating modes of the compensation cell"]
    pub mod GPIO_EMC1_COMPTQ {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "COMPEN and COMPTQ control the operating modes of the compensation cell"]
    pub mod GPIO_EMC1_COMPEN {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Compensation code fast freeze"]
    pub mod GPIO_EMC1_FASTFRZ_EN {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "GPIO_EMC_B1 IO bank's 4-bit PMOS compensation codes from core"]
    pub mod GPIO_EMC1_RASRCP {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "GPIO_EMC_B1 IO bank's 4-bit NMOS compensation codes from core"]
    pub mod GPIO_EMC1_RASRCN {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "GPIO_EMC1_NASRC selection"]
    pub mod GPIO_EMC1_SELECT_NASRC {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "GPIO_EMC_B1 IO bank reference voltage generator cell sleep enable"]
    pub mod GPIO_EMC1_REFGEN_SLEEP {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "GPIO_EMC_B1 IO bank power supply mode latch enable"]
    pub mod GPIO_EMC1_SUPLYDET_LATCH {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "GPIO_EMC_B1 IO bank compensation OK flag"]
    pub mod GPIO_EMC1_COMPOK {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "GPIO_EMC_B1 IO bank compensation codes"]
    pub mod GPIO_EMC1_NASRC {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Domain write protection"]
    pub mod DWP {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Both cores are allowed"]
            pub const FORBID_NONE: u32 = 0;
            #[doc = "CM7 is forbidden"]
            pub const FORBID_CM7: u32 = 0x01;
            #[doc = "CM4 is forbidden"]
            pub const FORBID_CM4: u32 = 0x02;
            #[doc = "Both cores are forbidden"]
            pub const FORBID_BOTH: u32 = 0x03;
        }
    }
    #[doc = "Domain write protection lock"]
    pub mod DWP_LOCK {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Neither of DWP bits is locked"]
            pub const LOCK_NONE: u32 = 0;
            #[doc = "The lower DWP bit is locked"]
            pub const LOCK_LOW: u32 = 0x01;
            #[doc = "The higher DWP bit is locked"]
            pub const LOCK_HIGH: u32 = 0x02;
            #[doc = "Both DWP bits are locked"]
            pub const LOCK_BOTH: u32 = 0x03;
        }
    }
}
#[doc = "GPR66 General Purpose Register"]
pub mod GPR66 {
    #[doc = "Compensation code freeze"]
    pub mod GPIO_EMC2_FREEZE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "COMPEN and COMPTQ control the operating modes of the compensation cell"]
    pub mod GPIO_EMC2_COMPTQ {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "COMPEN and COMPTQ control the operating modes of the compensation cell"]
    pub mod GPIO_EMC2_COMPEN {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Compensation code fast freeze"]
    pub mod GPIO_EMC2_FASTFRZ_EN {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "GPIO_EMC_B2 IO bank's 4-bit PMOS compensation codes from core"]
    pub mod GPIO_EMC2_RASRCP {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "GPIO_EMC_B2 IO bank's 4-bit NMOS compensation codes from core"]
    pub mod GPIO_EMC2_RASRCN {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "GPIO_EMC2_NASRC selection"]
    pub mod GPIO_EMC2_SELECT_NASRC {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "GPIO_EMC_B2 IO bank reference voltage generator cell sleep enable"]
    pub mod GPIO_EMC2_REFGEN_SLEEP {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "GPIO_EMC_B2 IO bank power supply mode latch enable"]
    pub mod GPIO_EMC2_SUPLYDET_LATCH {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "GPIO_EMC_B2 IO bank compensation OK flag"]
    pub mod GPIO_EMC2_COMPOK {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "GPIO_EMC_B2 IO bank compensation codes"]
    pub mod GPIO_EMC2_NASRC {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Domain write protection"]
    pub mod DWP {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Both cores are allowed"]
            pub const FORBID_NONE: u32 = 0;
            #[doc = "CM7 is forbidden"]
            pub const FORBID_CM7: u32 = 0x01;
            #[doc = "CM4 is forbidden"]
            pub const FORBID_CM4: u32 = 0x02;
            #[doc = "Both cores are forbidden"]
            pub const FORBID_BOTH: u32 = 0x03;
        }
    }
    #[doc = "Domain write protection lock"]
    pub mod DWP_LOCK {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Neither of DWP bits is locked"]
            pub const LOCK_NONE: u32 = 0;
            #[doc = "The lower DWP bit is locked"]
            pub const LOCK_LOW: u32 = 0x01;
            #[doc = "The higher DWP bit is locked"]
            pub const LOCK_HIGH: u32 = 0x02;
            #[doc = "Both DWP bits are locked"]
            pub const LOCK_BOTH: u32 = 0x03;
        }
    }
}
#[doc = "GPR67 General Purpose Register"]
pub mod GPR67 {
    #[doc = "Compensation code freeze"]
    pub mod GPIO_SD1_FREEZE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "COMPEN and COMPTQ control the operating modes of the compensation cell"]
    pub mod GPIO_SD1_COMPTQ {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "COMPEN and COMPTQ control the operating modes of the compensation cell"]
    pub mod GPIO_SD1_COMPEN {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Compensation code fast freeze"]
    pub mod GPIO_SD1_FASTFRZ_EN {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "GPIO_SD_B1 IO bank's 4-bit PMOS compensation codes from core"]
    pub mod GPIO_SD1_RASRCP {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "GPIO_SD_B1 IO bank's 4-bit NMOS compensation codes from core"]
    pub mod GPIO_SD1_RASRCN {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "GPIO_SD1_NASRC selection"]
    pub mod GPIO_SD1_SELECT_NASRC {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "GPIO_SD_B1 IO bank reference voltage generator cell sleep enable"]
    pub mod GPIO_SD1_REFGEN_SLEEP {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "GPIO_SD_B1 IO bank power supply mode latch enable"]
    pub mod GPIO_SD1_SUPLYDET_LATCH {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "GPIO_SD_B1 IO bank compensation OK flag"]
    pub mod GPIO_SD1_COMPOK {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "GPIO_SD_B1 IO bank compensation codes"]
    pub mod GPIO_SD1_NASRC {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Domain write protection"]
    pub mod DWP {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Both cores are allowed"]
            pub const FORBID_NONE: u32 = 0;
            #[doc = "CM7 is forbidden"]
            pub const FORBID_CM7: u32 = 0x01;
            #[doc = "CM4 is forbidden"]
            pub const FORBID_CM4: u32 = 0x02;
            #[doc = "Both cores are forbidden"]
            pub const FORBID_BOTH: u32 = 0x03;
        }
    }
    #[doc = "Domain write protection lock"]
    pub mod DWP_LOCK {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Neither of DWP bits is locked"]
            pub const LOCK_NONE: u32 = 0;
            #[doc = "The lower DWP bit is locked"]
            pub const LOCK_LOW: u32 = 0x01;
            #[doc = "The higher DWP bit is locked"]
            pub const LOCK_HIGH: u32 = 0x02;
            #[doc = "Both DWP bits are locked"]
            pub const LOCK_BOTH: u32 = 0x03;
        }
    }
}
#[doc = "GPR68 General Purpose Register"]
pub mod GPR68 {
    #[doc = "Compensation code freeze"]
    pub mod GPIO_SD2_FREEZE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "COMPEN and COMPTQ control the operating modes of the compensation cell"]
    pub mod GPIO_SD2_COMPTQ {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "COMPEN and COMPTQ control the operating modes of the compensation cell"]
    pub mod GPIO_SD2_COMPEN {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Compensation code fast freeze"]
    pub mod GPIO_SD2_FASTFRZ_EN {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "GPIO_SD_B2 IO bank's 4-bit PMOS compensation codes from core"]
    pub mod GPIO_SD2_RASRCP {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "GPIO_SD_B2 IO bank's 4-bit NMOS compensation codes from core"]
    pub mod GPIO_SD2_RASRCN {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "GPIO_SD2_NASRC selection"]
    pub mod GPIO_SD2_SELECT_NASRC {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "GPIO_SD_B2 IO bank reference voltage generator cell sleep enable"]
    pub mod GPIO_SD2_REFGEN_SLEEP {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "GPIO_SD_B2 IO bank power supply mode latch enable"]
    pub mod GPIO_SD2_SUPLYDET_LATCH {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "GPIO_SD_B2 IO bank compensation OK flag"]
    pub mod GPIO_SD2_COMPOK {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "GPIO_SD_B2 IO bank compensation codes"]
    pub mod GPIO_SD2_NASRC {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Domain write protection"]
    pub mod DWP {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Both cores are allowed"]
            pub const FORBID_NONE: u32 = 0;
            #[doc = "CM7 is forbidden"]
            pub const FORBID_CM7: u32 = 0x01;
            #[doc = "CM4 is forbidden"]
            pub const FORBID_CM4: u32 = 0x02;
            #[doc = "Both cores are forbidden"]
            pub const FORBID_BOTH: u32 = 0x03;
        }
    }
    #[doc = "Domain write protection lock"]
    pub mod DWP_LOCK {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Neither of DWP bits is locked"]
            pub const LOCK_NONE: u32 = 0;
            #[doc = "The lower DWP bit is locked"]
            pub const LOCK_LOW: u32 = 0x01;
            #[doc = "The higher DWP bit is locked"]
            pub const LOCK_HIGH: u32 = 0x02;
            #[doc = "Both DWP bits are locked"]
            pub const LOCK_BOTH: u32 = 0x03;
        }
    }
}
#[doc = "GPR69 General Purpose Register"]
pub mod GPR69 {
    #[doc = "GPIO_DISP_B2 IO bank supply voltage range selection"]
    pub mod GPIO_DISP2_HIGH_RANGE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "GPIO_DISP_B2 IO bank supply voltage range selection"]
    pub mod GPIO_DISP2_LOW_RANGE {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "GPIO_AD IO bank supply voltage range selection for GPIO_AD_00 to GPIO_AD_17"]
    pub mod GPIO_AD0_HIGH_RANGE {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "GPIO_AD IO bank supply voltage range selection for GPIO_AD_00 to GPIO_AD_17"]
    pub mod GPIO_AD0_LOW_RANGE {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "GPIO_LPSR IO bank supply voltage range selection for GPIO_AD_18 to GPIO_AD_35"]
    pub mod GPIO_AD1_HIGH_RANGE {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "GPIO_LPSR IO bank supply voltage range selection for GPIO_AD_18 to GPIO_AD_35"]
    pub mod GPIO_AD1_LOW_RANGE {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "GPIO_DISP_B1 IO bank supply voltage detector sleep mode enable"]
    pub mod SUPLYDET_DISP1_SLEEP {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "GPIO_EMC_B1 IO bank supply voltage detector sleep mode enable"]
    pub mod SUPLYDET_EMC1_SLEEP {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "GPIO_EMC_B2 IO bank supply voltage detector sleep mode enable"]
    pub mod SUPLYDET_EMC2_SLEEP {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "GPIO_SD_B1 IO bank supply voltage detector sleep mode enable"]
    pub mod SUPLYDET_SD1_SLEEP {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "GPIO_SD_B2 IO bank supply voltage detector sleep mode enable"]
    pub mod SUPLYDET_SD2_SLEEP {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Domain write protection"]
    pub mod DWP {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Both cores are allowed"]
            pub const FORBID_NONE: u32 = 0;
            #[doc = "CM7 is forbidden"]
            pub const FORBID_CM7: u32 = 0x01;
            #[doc = "CM4 is forbidden"]
            pub const FORBID_CM4: u32 = 0x02;
            #[doc = "Both cores are forbidden"]
            pub const FORBID_BOTH: u32 = 0x03;
        }
    }
    #[doc = "Domain write protection lock"]
    pub mod DWP_LOCK {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Neither of DWP bits is locked"]
            pub const LOCK_NONE: u32 = 0;
            #[doc = "The lower DWP bit is locked"]
            pub const LOCK_LOW: u32 = 0x01;
            #[doc = "The higher DWP bit is locked"]
            pub const LOCK_HIGH: u32 = 0x02;
            #[doc = "Both DWP bits are locked"]
            pub const LOCK_BOTH: u32 = 0x03;
        }
    }
}
#[doc = "GPR70 General Purpose Register"]
pub mod GPR70 {
    #[doc = "ADC1 doze mode"]
    pub mod ADC1_IPG_DOZE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ADC1 stop request"]
    pub mod ADC1_STOP_REQ {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ADC1 stop mode selection, cannot change when ADC1_STOP_REQ is asserted."]
    pub mod ADC1_IPG_STOP_MODE {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "This module is functional in Stop Mode"]
            pub const FUNC: u32 = 0;
            #[doc = "This module is not functional in Stop Mode and the corresponding x_STOP_REQ field is set to '1'."]
            pub const NONFUNC: u32 = 0x01;
        }
    }
    #[doc = "ADC2 doze mode"]
    pub mod ADC2_IPG_DOZE {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ADC2 stop request"]
    pub mod ADC2_STOP_REQ {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ADC2 stop mode selection, cannot change when ADC2_STOP_REQ is asserted."]
    pub mod ADC2_IPG_STOP_MODE {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "This module is functional in Stop Mode"]
            pub const FUNC: u32 = 0;
            #[doc = "This module is not functional in Stop Mode and the corresponding x_STOP_REQ field is set to '1'."]
            pub const NONFUNC: u32 = 0x01;
        }
    }
    #[doc = "CAN3 doze mode"]
    pub mod CAAM_IPG_DOZE {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CAAM stop request"]
    pub mod CAAM_STOP_REQ {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CAN1 doze mode"]
    pub mod CAN1_IPG_DOZE {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CAN1 stop request"]
    pub mod CAN1_STOP_REQ {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CAN2 doze mode"]
    pub mod CAN2_IPG_DOZE {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CAN2 stop request"]
    pub mod CAN2_STOP_REQ {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CAN3 doze mode"]
    pub mod CAN3_IPG_DOZE {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CAN3 stop request"]
    pub mod CAN3_STOP_REQ {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "EDMA stop request"]
    pub mod EDMA_STOP_REQ {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "EDMA_LPSR stop request"]
    pub mod EDMA_LPSR_STOP_REQ {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ENET doze mode"]
    pub mod ENET_IPG_DOZE {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ENET stop request"]
    pub mod ENET_STOP_REQ {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ENET1G doze mode"]
    pub mod ENET1G_IPG_DOZE {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ENET1G stop request"]
    pub mod ENET1G_STOP_REQ {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "FLEXIO2 doze mode"]
    pub mod FLEXIO1_IPG_DOZE {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "FLEXIO2 doze mode"]
    pub mod FLEXIO2_IPG_DOZE {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "FLEXSPI1 doze mode"]
    pub mod FLEXSPI1_IPG_DOZE {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "FLEXSPI1 stop request"]
    pub mod FLEXSPI1_STOP_REQ {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "FLEXSPI2 doze mode"]
    pub mod FLEXSPI2_IPG_DOZE {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "FLEXSPI2 stop request"]
    pub mod FLEXSPI2_STOP_REQ {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Domain write protection"]
    pub mod DWP {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Both cores are allowed"]
            pub const FORBID_NONE: u32 = 0;
            #[doc = "CM7 is forbidden"]
            pub const FORBID_CM7: u32 = 0x01;
            #[doc = "CM4 is forbidden"]
            pub const FORBID_CM4: u32 = 0x02;
            #[doc = "Both cores are forbidden"]
            pub const FORBID_BOTH: u32 = 0x03;
        }
    }
    #[doc = "Domain write protection lock"]
    pub mod DWP_LOCK {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Neither of DWP bits is locked"]
            pub const LOCK_NONE: u32 = 0;
            #[doc = "The lower DWP bit is locked"]
            pub const LOCK_LOW: u32 = 0x01;
            #[doc = "The higher DWP bit is locked"]
            pub const LOCK_HIGH: u32 = 0x02;
            #[doc = "Both DWP bits are locked"]
            pub const LOCK_BOTH: u32 = 0x03;
        }
    }
}
#[doc = "GPR71 General Purpose Register"]
pub mod GPR71 {
    #[doc = "GPT1 doze mode"]
    pub mod GPT1_IPG_DOZE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "GPT2 doze mode"]
    pub mod GPT2_IPG_DOZE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "GPT3 doze mode"]
    pub mod GPT3_IPG_DOZE {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "GPT4 doze mode"]
    pub mod GPT4_IPG_DOZE {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "GPT5 doze mode"]
    pub mod GPT5_IPG_DOZE {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "GPT6 doze mode"]
    pub mod GPT6_IPG_DOZE {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "LPI2C1 doze mode"]
    pub mod LPI2C1_IPG_DOZE {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "LPI2C1 stop request"]
    pub mod LPI2C1_STOP_REQ {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "LPI2C1 stop mode selection, cannot change when LPI2C1_STOP_REQ is asserted."]
    pub mod LPI2C1_IPG_STOP_MODE {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "the module is functional in Stop mode"]
            pub const LPI2C1_IPG_STOP_MODE_0: u32 = 0;
            #[doc = "the module is NOT functional in Stop mode, when this bit is equal to 1 and ipg_stop is asserted"]
            pub const LPI2C1_IPG_STOP_MODE_1: u32 = 0x01;
        }
    }
    #[doc = "LPI2C2 doze mode"]
    pub mod LPI2C2_IPG_DOZE {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "LPI2C2 stop request"]
    pub mod LPI2C2_STOP_REQ {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "LPI2C2 stop mode selection, cannot change when LPI2C2_STOP_REQ is asserted."]
    pub mod LPI2C2_IPG_STOP_MODE {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "the module is functional in Stop mode"]
            pub const LPI2C2_IPG_STOP_MODE_0: u32 = 0;
            #[doc = "the module is NOT functional in Stop mode, when this bit is equal to 1 and ipg_stop is asserted"]
            pub const LPI2C2_IPG_STOP_MODE_1: u32 = 0x01;
        }
    }
    #[doc = "LPI2C3 doze mode"]
    pub mod LPI2C3_IPG_DOZE {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "LPI2C3 stop request"]
    pub mod LPI2C3_STOP_REQ {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "LPI2C3 stop mode selection, cannot change when LPI2C3_STOP_REQ is asserted."]
    pub mod LPI2C3_IPG_STOP_MODE {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "the module is functional in Stop mode"]
            pub const LPI2C3_IPG_STOP_MODE_0: u32 = 0;
            #[doc = "the module is NOT functional in Stop mode, when this bit is equal to 1 and ipg_stop is asserted"]
            pub const LPI2C3_IPG_STOP_MODE_1: u32 = 0x01;
        }
    }
    #[doc = "LPI2C4 doze mode"]
    pub mod LPI2C4_IPG_DOZE {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "LPI2C4 stop request"]
    pub mod LPI2C4_STOP_REQ {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "LPI2C4 stop mode selection, cannot change when LPI2C4_STOP_REQ is asserted."]
    pub mod LPI2C4_IPG_STOP_MODE {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "the module is functional in Stop mode"]
            pub const LPI2C4_IPG_STOP_MODE_0: u32 = 0;
            #[doc = "the module is NOT functional in Stop mode, when this bit is equal to 1 and ipg_stop is asserted"]
            pub const LPI2C4_IPG_STOP_MODE_1: u32 = 0x01;
        }
    }
    #[doc = "LPI2C5 doze mode"]
    pub mod LPI2C5_IPG_DOZE {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "LPI2C5 stop request"]
    pub mod LPI2C5_STOP_REQ {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "LPI2C5 stop mode selection, cannot change when LPI2C5_STOP_REQ is asserted."]
    pub mod LPI2C5_IPG_STOP_MODE {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "This module is functional in Stop Mode"]
            pub const FUNC: u32 = 0;
            #[doc = "This module is not functional in Stop Mode and the corresponding x_STOP_REQ field is set to '1'."]
            pub const NONFUNC: u32 = 0x01;
        }
    }
    #[doc = "LPI2C6 doze mode"]
    pub mod LPI2C6_IPG_DOZE {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "LPI2C6 stop request"]
    pub mod LPI2C6_STOP_REQ {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "LPI2C6 stop mode selection, cannot change when LPI2C6_STOP_REQ is asserted."]
    pub mod LPI2C6_IPG_STOP_MODE {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "This module is functional in Stop Mode"]
            pub const FUNC: u32 = 0;
            #[doc = "This module is not functional in Stop Mode and the corresponding x_STOP_REQ field is set to '1'."]
            pub const NONFUNC: u32 = 0x01;
        }
    }
    #[doc = "LPSPI1 doze mode"]
    pub mod LPSPI1_IPG_DOZE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "LPSPI1 stop request"]
    pub mod LPSPI1_STOP_REQ {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "LPSPI1 stop mode selection, cannot change when LPSPI1_STOP_REQ is asserted."]
    pub mod LPSPI1_IPG_STOP_MODE {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "the module is functional in Stop mode"]
            pub const LPSPI1_IPG_STOP_MODE_0: u32 = 0;
            #[doc = "the module is NOT functional in Stop mode, when this bit is equal to 1 and ipg_stop is asserted"]
            pub const LPSPI1_IPG_STOP_MODE_1: u32 = 0x01;
        }
    }
    #[doc = "Domain write protection"]
    pub mod DWP {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Both cores are allowed"]
            pub const FORBID_NONE: u32 = 0;
            #[doc = "CM7 is forbidden"]
            pub const FORBID_CM7: u32 = 0x01;
            #[doc = "CM4 is forbidden"]
            pub const FORBID_CM4: u32 = 0x02;
            #[doc = "Both cores are forbidden"]
            pub const FORBID_BOTH: u32 = 0x03;
        }
    }
    #[doc = "Domain write protection lock"]
    pub mod DWP_LOCK {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Neither of DWP bits is locked"]
            pub const LOCK_NONE: u32 = 0;
            #[doc = "The lower DWP bit is locked"]
            pub const LOCK_LOW: u32 = 0x01;
            #[doc = "The higher DWP bit is locked"]
            pub const LOCK_HIGH: u32 = 0x02;
            #[doc = "Both DWP bits are locked"]
            pub const LOCK_BOTH: u32 = 0x03;
        }
    }
}
#[doc = "GPR72 General Purpose Register"]
pub mod GPR72 {
    #[doc = "LPSPI2 doze mode"]
    pub mod LPSPI2_IPG_DOZE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "LPSPI2 stop request"]
    pub mod LPSPI2_STOP_REQ {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "LPSPI2 stop mode selection, cannot change when LPSPI2_STOP_REQ is asserted."]
    pub mod LPSPI2_IPG_STOP_MODE {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "the module is functional in Stop mode"]
            pub const LPSPI2_IPG_STOP_MODE_0: u32 = 0;
            #[doc = "the module is NOT functional in Stop mode, when this bit is equal to 1 and ipg_stop is asserted"]
            pub const LPSPI2_IPG_STOP_MODE_1: u32 = 0x01;
        }
    }
    #[doc = "LPSPI3 doze mode"]
    pub mod LPSPI3_IPG_DOZE {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "LPSPI3 stop request"]
    pub mod LPSPI3_STOP_REQ {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "LPSPI3 stop mode selection, cannot change when LPSPI3_STOP_REQ is asserted."]
    pub mod LPSPI3_IPG_STOP_MODE {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "the module is functional in Stop mode"]
            pub const LPSPI3_IPG_STOP_MODE_0: u32 = 0;
            #[doc = "the module is NOT functional in Stop mode, when this bit is equal to 1 and ipg_stop is asserted"]
            pub const LPSPI3_IPG_STOP_MODE_1: u32 = 0x01;
        }
    }
    #[doc = "LPSPI4 doze mode"]
    pub mod LPSPI4_IPG_DOZE {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "LPSPI4 stop request"]
    pub mod LPSPI4_STOP_REQ {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "LPSPI4 stop mode selection, cannot change when LPSPI4_STOP_REQ is asserted."]
    pub mod LPSPI4_IPG_STOP_MODE {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "the module is functional in Stop mode"]
            pub const LPSPI4_IPG_STOP_MODE_0: u32 = 0;
            #[doc = "the module is NOT functional in Stop mode, when this bit is equal to 1 and ipg_stop is asserted"]
            pub const LPSPI4_IPG_STOP_MODE_1: u32 = 0x01;
        }
    }
    #[doc = "LPSPI5 doze mode"]
    pub mod LPSPI5_IPG_DOZE {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "LPSPI5 stop request"]
    pub mod LPSPI5_STOP_REQ {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "LPSPI5 stop mode selection, cannot change when LPSPI5_STOP_REQ is asserted."]
    pub mod LPSPI5_IPG_STOP_MODE {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "This module is functional in Stop Mode"]
            pub const FUNC: u32 = 0;
            #[doc = "This module is not functional in Stop Mode and the corresponding x_STOP_REQ field is set to '1'."]
            pub const NONFUNC: u32 = 0x01;
        }
    }
    #[doc = "LPSPI6 doze mode"]
    pub mod LPSPI6_IPG_DOZE {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "LPSPI6 stop request"]
    pub mod LPSPI6_STOP_REQ {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "LPSPI6 stop mode selection, cannot change when LPSPI6_STOP_REQ is asserted."]
    pub mod LPSPI6_IPG_STOP_MODE {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "This module is functional in Stop Mode"]
            pub const FUNC: u32 = 0;
            #[doc = "This module is not functional in Stop Mode and the corresponding x_STOP_REQ field is set to '1'."]
            pub const NONFUNC: u32 = 0x01;
        }
    }
    #[doc = "LPUART1 doze mode"]
    pub mod LPUART1_IPG_DOZE {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "LPUART1 stop request"]
    pub mod LPUART1_STOP_REQ {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "LPUART1 stop mode selection, cannot change when LPUART1_STOP_REQ is asserted."]
    pub mod LPUART1_IPG_STOP_MODE {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "the module is functional in Stop mode"]
            pub const LPUART1_IPG_STOP_MODE_0: u32 = 0;
            #[doc = "the module is NOT functional in Stop mode, when this bit is equal to 1 and ipg_stop is asserted"]
            pub const LPUART1_IPG_STOP_MODE_1: u32 = 0x01;
        }
    }
    #[doc = "LPUART2 doze mode"]
    pub mod LPUART2_IPG_DOZE {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "LPUART2 stop request"]
    pub mod LPUART2_STOP_REQ {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "LPUART2 stop mode selection, cannot change when LPUART2_STOP_REQ is asserted."]
    pub mod LPUART2_IPG_STOP_MODE {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "the module is functional in Stop mode"]
            pub const LPUART2_IPG_STOP_MODE_0: u32 = 0;
            #[doc = "the module is NOT functional in Stop mode, when this bit is equal to 1 and ipg_stop is asserted"]
            pub const LPUART2_IPG_STOP_MODE_1: u32 = 0x01;
        }
    }
    #[doc = "LPUART3 doze mode"]
    pub mod LPUART3_IPG_DOZE {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "LPUART3 stop request"]
    pub mod LPUART3_STOP_REQ {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "LPUART3 stop mode selection, cannot change when LPUART3_STOP_REQ is asserted."]
    pub mod LPUART3_IPG_STOP_MODE {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "the module is functional in Stop mode"]
            pub const LPUART3_IPG_STOP_MODE_0: u32 = 0;
            #[doc = "the module is NOT functional in Stop mode, when this bit is equal to 1 and ipg_stop is asserted"]
            pub const LPUART3_IPG_STOP_MODE_1: u32 = 0x01;
        }
    }
    #[doc = "LPUART4 doze mode"]
    pub mod LPUART4_IPG_DOZE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "LPUART4 stop request"]
    pub mod LPUART4_STOP_REQ {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "LPUART4 stop mode selection, cannot change when LPUART4_STOP_REQ is asserted."]
    pub mod LPUART4_IPG_STOP_MODE {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "the module is functional in Stop mode"]
            pub const LPUART4_IPG_STOP_MODE_0: u32 = 0;
            #[doc = "the module is NOT functional in Stop mode, when this bit is equal to 1 and ipg_stop is asserted"]
            pub const LPUART4_IPG_STOP_MODE_1: u32 = 0x01;
        }
    }
    #[doc = "Domain write protection"]
    pub mod DWP {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Both cores are allowed"]
            pub const FORBID_NONE: u32 = 0;
            #[doc = "CM7 is forbidden"]
            pub const FORBID_CM7: u32 = 0x01;
            #[doc = "CM4 is forbidden"]
            pub const FORBID_CM4: u32 = 0x02;
            #[doc = "Both cores are forbidden"]
            pub const FORBID_BOTH: u32 = 0x03;
        }
    }
    #[doc = "Domain write protection lock"]
    pub mod DWP_LOCK {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Neither of DWP bits is locked"]
            pub const LOCK_NONE: u32 = 0;
            #[doc = "The lower DWP bit is locked"]
            pub const LOCK_LOW: u32 = 0x01;
            #[doc = "The higher DWP bit is locked"]
            pub const LOCK_HIGH: u32 = 0x02;
            #[doc = "Both DWP bits are locked"]
            pub const LOCK_BOTH: u32 = 0x03;
        }
    }
}
#[doc = "GPR73 General Purpose Register"]
pub mod GPR73 {
    #[doc = "LPUART5 doze mode"]
    pub mod LPUART5_IPG_DOZE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "LPUART5 stop request"]
    pub mod LPUART5_STOP_REQ {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "LPUART5 stop mode selection, cannot change when LPUART5_STOP_REQ is asserted."]
    pub mod LPUART5_IPG_STOP_MODE {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "the module is functional in Stop mode"]
            pub const LPUART5_IPG_STOP_MODE_0: u32 = 0;
            #[doc = "the module is NOT functional in Stop mode, when this bit is equal to 1 and ipg_stop is asserted"]
            pub const LPUART5_IPG_STOP_MODE_1: u32 = 0x01;
        }
    }
    #[doc = "LPUART6 doze mode"]
    pub mod LPUART6_IPG_DOZE {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "LPUART6 stop request"]
    pub mod LPUART6_STOP_REQ {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "LPUART6 stop mode selection, cannot change when LPUART6_STOP_REQ is asserted."]
    pub mod LPUART6_IPG_STOP_MODE {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "the module is functional in Stop mode"]
            pub const LPUART6_IPG_STOP_MODE_0: u32 = 0;
            #[doc = "the module is NOT functional in Stop mode, when this bit is equal to 1 and ipg_stop is asserted"]
            pub const LPUART6_IPG_STOP_MODE_1: u32 = 0x01;
        }
    }
    #[doc = "LPUART7 doze mode"]
    pub mod LPUART7_IPG_DOZE {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "LPUART7 stop request"]
    pub mod LPUART7_STOP_REQ {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "LPUART7 stop mode selection, cannot change when LPUART7_STOP_REQ is asserted."]
    pub mod LPUART7_IPG_STOP_MODE {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "the module is functional in Stop mode"]
            pub const LPUART7_IPG_STOP_MODE_0: u32 = 0;
            #[doc = "the module is NOT functional in Stop mode, when this bit is equal to 1 and ipg_stop is asserted"]
            pub const LPUART7_IPG_STOP_MODE_1: u32 = 0x01;
        }
    }
    #[doc = "LPUART8 doze mode"]
    pub mod LPUART8_IPG_DOZE {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "LPUART8 stop request"]
    pub mod LPUART8_STOP_REQ {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "LPUART8 stop mode selection, cannot change when LPUART8_STOP_REQ is asserted."]
    pub mod LPUART8_IPG_STOP_MODE {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "the module is functional in Stop mode"]
            pub const LPUART8_IPG_STOP_MODE_0: u32 = 0;
            #[doc = "the module is NOT functional in Stop mode, when this bit is equal to 1 and ipg_stop is asserted"]
            pub const LPUART8_IPG_STOP_MODE_1: u32 = 0x01;
        }
    }
    #[doc = "LPUART9 doze mode"]
    pub mod LPUART9_IPG_DOZE {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "LPUART9 stop request"]
    pub mod LPUART9_STOP_REQ {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "LPUART9 stop mode selection, cannot change when LPUART9_STOP_REQ is asserted."]
    pub mod LPUART9_IPG_STOP_MODE {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "This module is functional in Stop Mode"]
            pub const FUNC: u32 = 0;
            #[doc = "This module is not functional in Stop Mode and the corresponding x_STOP_REQ field is set to '1'."]
            pub const NONFUNC: u32 = 0x01;
        }
    }
    #[doc = "LPUART10 doze mode"]
    pub mod LPUART10_IPG_DOZE {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "LPUART10 stop request"]
    pub mod LPUART10_STOP_REQ {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "LPUART10 stop mode selection, cannot change when LPUART10_STOP_REQ is asserted."]
    pub mod LPUART10_IPG_STOP_MODE {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "This module is functional in Stop Mode"]
            pub const FUNC: u32 = 0;
            #[doc = "This module is not functional in Stop Mode and the corresponding x_STOP_REQ field is set to '1'."]
            pub const NONFUNC: u32 = 0x01;
        }
    }
    #[doc = "LPUART11 doze mode"]
    pub mod LPUART11_IPG_DOZE {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "LPUART11 stop request"]
    pub mod LPUART11_STOP_REQ {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "LPUART11 stop mode selection, cannot change when LPUART11_STOP_REQ is asserted."]
    pub mod LPUART11_IPG_STOP_MODE {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "This module is functional in Stop Mode"]
            pub const FUNC: u32 = 0;
            #[doc = "This module is not functional in Stop Mode and the corresponding x_STOP_REQ field is set to '1'."]
            pub const NONFUNC: u32 = 0x01;
        }
    }
    #[doc = "LPUART12 doze mode"]
    pub mod LPUART12_IPG_DOZE {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "LPUART12 stop request"]
    pub mod LPUART12_STOP_REQ {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "LPUART12 stop mode selection, cannot change when LPUART12_STOP_REQ is asserted."]
    pub mod LPUART12_IPG_STOP_MODE {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "This module is functional in Stop Mode"]
            pub const FUNC: u32 = 0;
            #[doc = "This module is not functional in Stop Mode and the corresponding x_STOP_REQ field is set to '1'."]
            pub const NONFUNC: u32 = 0x01;
        }
    }
    #[doc = "MIC doze mode"]
    pub mod MIC_IPG_DOZE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "MIC stop request"]
    pub mod MIC_STOP_REQ {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "MIC stop mode selection, cannot change when MIC_STOP_REQ is asserted."]
    pub mod MIC_IPG_STOP_MODE {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "This module is functional in Stop Mode"]
            pub const FUNC: u32 = 0;
            #[doc = "This module is not functional in Stop Mode and the corresponding x_STOP_REQ field is set to '1'."]
            pub const NONFUNC: u32 = 0x01;
        }
    }
    #[doc = "Domain write protection"]
    pub mod DWP {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Both cores are allowed"]
            pub const FORBID_NONE: u32 = 0;
            #[doc = "CM7 is forbidden"]
            pub const FORBID_CM7: u32 = 0x01;
            #[doc = "CM4 is forbidden"]
            pub const FORBID_CM4: u32 = 0x02;
            #[doc = "Both cores are forbidden"]
            pub const FORBID_BOTH: u32 = 0x03;
        }
    }
    #[doc = "Domain write protection lock"]
    pub mod DWP_LOCK {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Neither of DWP bits is locked"]
            pub const LOCK_NONE: u32 = 0;
            #[doc = "The lower DWP bit is locked"]
            pub const LOCK_LOW: u32 = 0x01;
            #[doc = "The higher DWP bit is locked"]
            pub const LOCK_HIGH: u32 = 0x02;
            #[doc = "Both DWP bits are locked"]
            pub const LOCK_BOTH: u32 = 0x03;
        }
    }
}
#[doc = "GPR74 General Purpose Register"]
pub mod GPR74 {
    #[doc = "PIT1 stop request"]
    pub mod PIT1_STOP_REQ {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "PIT2 stop request"]
    pub mod PIT2_STOP_REQ {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "SEMC stop request"]
    pub mod SEMC_STOP_REQ {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "SIM1 doze mode"]
    pub mod SIM1_IPG_DOZE {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "SIM2 doze mode"]
    pub mod SIM2_IPG_DOZE {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "SNVS_HP doze mode"]
    pub mod SNVS_HP_IPG_DOZE {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "SNVS_HP stop request"]
    pub mod SNVS_HP_STOP_REQ {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "WDOG1 doze mode"]
    pub mod WDOG1_IPG_DOZE {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "WDOG2 doze mode"]
    pub mod WDOG2_IPG_DOZE {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "SAI1 stop request"]
    pub mod SAI1_STOP_REQ {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "SAI2 stop request"]
    pub mod SAI2_STOP_REQ {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "SAI3 stop request"]
    pub mod SAI3_STOP_REQ {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "SAI4 stop request"]
    pub mod SAI4_STOP_REQ {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "FLEXIO1 bus clock domain stop request"]
    pub mod FLEXIO1_STOP_REQ_BUS {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "FLEXIO1 peripheral clock domain stop request"]
    pub mod FLEXIO1_STOP_REQ_PER {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "FLEXIO2 bus clock domain stop request"]
    pub mod FLEXIO2_STOP_REQ_BUS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "FLEXIO2 peripheral clock domain stop request"]
    pub mod FLEXIO2_STOP_REQ_PER {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Domain write protection"]
    pub mod DWP {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Both cores are allowed"]
            pub const FORBID_NONE: u32 = 0;
            #[doc = "CM7 is forbidden"]
            pub const FORBID_CM7: u32 = 0x01;
            #[doc = "CM4 is forbidden"]
            pub const FORBID_CM4: u32 = 0x02;
            #[doc = "Both cores are forbidden"]
            pub const FORBID_BOTH: u32 = 0x03;
        }
    }
    #[doc = "Domain write protection lock"]
    pub mod DWP_LOCK {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Neither of DWP bits is locked"]
            pub const LOCK_NONE: u32 = 0;
            #[doc = "The lower DWP bit is locked"]
            pub const LOCK_LOW: u32 = 0x01;
            #[doc = "The higher DWP bit is locked"]
            pub const LOCK_HIGH: u32 = 0x02;
            #[doc = "Both DWP bits are locked"]
            pub const LOCK_BOTH: u32 = 0x03;
        }
    }
}
#[doc = "GPR75 General Purpose Register"]
pub mod GPR75 {
    #[doc = "ADC1 stop acknowledge"]
    pub mod ADC1_STOP_ACK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ADC2 stop acknowledge"]
    pub mod ADC2_STOP_ACK {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CAAM stop acknowledge"]
    pub mod CAAM_STOP_ACK {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CAN1 stop acknowledge"]
    pub mod CAN1_STOP_ACK {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CAN2 stop acknowledge"]
    pub mod CAN2_STOP_ACK {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CAN3 stop acknowledge"]
    pub mod CAN3_STOP_ACK {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "EDMA stop acknowledge"]
    pub mod EDMA_STOP_ACK {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "EDMA_LPSR stop acknowledge"]
    pub mod EDMA_LPSR_STOP_ACK {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ENET stop acknowledge"]
    pub mod ENET_STOP_ACK {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ENET1G stop acknowledge"]
    pub mod ENET1G_STOP_ACK {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "FLEXSPI1 stop acknowledge"]
    pub mod FLEXSPI1_STOP_ACK {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "FLEXSPI2 stop acknowledge"]
    pub mod FLEXSPI2_STOP_ACK {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "LPI2C1 stop acknowledge"]
    pub mod LPI2C1_STOP_ACK {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "LPI2C2 stop acknowledge"]
    pub mod LPI2C2_STOP_ACK {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "LPI2C3 stop acknowledge"]
    pub mod LPI2C3_STOP_ACK {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "LPI2C4 stop acknowledge"]
    pub mod LPI2C4_STOP_ACK {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "LPI2C5 stop acknowledge"]
    pub mod LPI2C5_STOP_ACK {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "LPI2C6 stop acknowledge"]
    pub mod LPI2C6_STOP_ACK {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "LPSPI1 stop acknowledge"]
    pub mod LPSPI1_STOP_ACK {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "LPSPI2 stop acknowledge"]
    pub mod LPSPI2_STOP_ACK {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "LPSPI3 stop acknowledge"]
    pub mod LPSPI3_STOP_ACK {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "LPSPI4 stop acknowledge"]
    pub mod LPSPI4_STOP_ACK {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "LPSPI5 stop acknowledge"]
    pub mod LPSPI5_STOP_ACK {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "LPSPI6 stop acknowledge"]
    pub mod LPSPI6_STOP_ACK {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "LPUART1 stop acknowledge"]
    pub mod LPUART1_STOP_ACK {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "LPUART2 stop acknowledge"]
    pub mod LPUART2_STOP_ACK {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "LPUART3 stop acknowledge"]
    pub mod LPUART3_STOP_ACK {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "LPUART4 stop acknowledge"]
    pub mod LPUART4_STOP_ACK {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "LPUART5 stop acknowledge"]
    pub mod LPUART5_STOP_ACK {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "LPUART6 stop acknowledge"]
    pub mod LPUART6_STOP_ACK {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "LPUART7 stop acknowledge"]
    pub mod LPUART7_STOP_ACK {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "LPUART8 stop acknowledge"]
    pub mod LPUART8_STOP_ACK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPR76 General Purpose Register"]
pub mod GPR76 {
    #[doc = "LPUART9 stop acknowledge"]
    pub mod LPUART9_STOP_ACK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "LPUART10 stop acknowledge"]
    pub mod LPUART10_STOP_ACK {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "LPUART11 stop acknowledge"]
    pub mod LPUART11_STOP_ACK {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "LPUART12 stop acknowledge"]
    pub mod LPUART12_STOP_ACK {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "MIC stop acknowledge"]
    pub mod MIC_STOP_ACK {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "PIT1 stop acknowledge"]
    pub mod PIT1_STOP_ACK {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "PIT2 stop acknowledge"]
    pub mod PIT2_STOP_ACK {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "SEMC stop acknowledge"]
    pub mod SEMC_STOP_ACK {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "SNVS_HP stop acknowledge"]
    pub mod SNVS_HP_STOP_ACK {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "SAI1 stop acknowledge"]
    pub mod SAI1_STOP_ACK {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "SAI2 stop acknowledge"]
    pub mod SAI2_STOP_ACK {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "SAI3 stop acknowledge"]
    pub mod SAI3_STOP_ACK {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "SAI4 stop acknowledge"]
    pub mod SAI4_STOP_ACK {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "FLEXIO1 stop acknowledge of bus clock domain"]
    pub mod FLEXIO1_STOP_ACK_BUS {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "FLEXIO1 stop acknowledge of peripheral clock domain"]
    pub mod FLEXIO1_STOP_ACK_PER {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "FLEXIO2 stop acknowledge of bus clock domain"]
    pub mod FLEXIO2_STOP_ACK_BUS {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "FLEXIO2 stop acknowledge of peripheral clock domain"]
    pub mod FLEXIO2_STOP_ACK_PER {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
