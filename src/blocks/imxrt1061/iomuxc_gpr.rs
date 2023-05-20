#[doc = "IOMUXC_GPR"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "GPR0 General Purpose Register"]
    pub GPR0: crate::RORegister<u32>,
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
    pub GPR9: crate::RORegister<u32>,
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
    pub GPR15: crate::RORegister<u32>,
    #[doc = "GPR16 General Purpose Register"]
    pub GPR16: crate::RWRegister<u32>,
    #[doc = "GPR17 General Purpose Register"]
    pub GPR17: crate::RWRegister<u32>,
    #[doc = "GPR18 General Purpose Register"]
    pub GPR18: crate::RWRegister<u32>,
    #[doc = "GPR19 General Purpose Register"]
    pub GPR19: crate::RWRegister<u32>,
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
}
#[doc = "GPR1 General Purpose Register"]
pub mod GPR1 {
    #[doc = "SAI1 MCLK1 source select"]
    pub mod SAI1_MCLK1_SEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ccm.ssi1_clk_root"]
            pub const SAI1_MCLK1_SEL_0: u32 = 0;
            #[doc = "ccm.ssi2_clk_root"]
            pub const SAI1_MCLK1_SEL_1: u32 = 0x01;
            #[doc = "ccm.ssi3_clk_root"]
            pub const SAI1_MCLK1_SEL_2: u32 = 0x02;
            #[doc = "iomux.sai1_ipg_clk_sai_mclk"]
            pub const SAI1_MCLK1_SEL_3: u32 = 0x03;
            #[doc = "iomux.sai2_ipg_clk_sai_mclk"]
            pub const SAI1_MCLK1_SEL_4: u32 = 0x04;
            #[doc = "iomux.sai3_ipg_clk_sai_mclk"]
            pub const SAI1_MCLK1_SEL_5: u32 = 0x05;
        }
    }
    #[doc = "SAI1 MCLK2 source select"]
    pub mod SAI1_MCLK2_SEL {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ccm.ssi1_clk_root"]
            pub const SAI1_MCLK2_SEL_0: u32 = 0;
            #[doc = "ccm.ssi2_clk_root"]
            pub const SAI1_MCLK2_SEL_1: u32 = 0x01;
            #[doc = "ccm.ssi3_clk_root"]
            pub const SAI1_MCLK2_SEL_2: u32 = 0x02;
            #[doc = "iomux.sai1_ipg_clk_sai_mclk"]
            pub const SAI1_MCLK2_SEL_3: u32 = 0x03;
            #[doc = "iomux.sai2_ipg_clk_sai_mclk"]
            pub const SAI1_MCLK2_SEL_4: u32 = 0x04;
            #[doc = "iomux.sai3_ipg_clk_sai_mclk"]
            pub const SAI1_MCLK2_SEL_5: u32 = 0x05;
        }
    }
    #[doc = "SAI1 MCLK3 source select"]
    pub mod SAI1_MCLK3_SEL {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ccm.spdif0_clk_root"]
            pub const SAI1_MCLK3_SEL_0: u32 = 0;
            #[doc = "SPDIF_EXT_CLK"]
            pub const SAI1_MCLK3_SEL_1: u32 = 0x01;
            #[doc = "spdif.spdif_srclk"]
            pub const SAI1_MCLK3_SEL_2: u32 = 0x02;
            #[doc = "spdif.spdif_outclock"]
            pub const SAI1_MCLK3_SEL_3: u32 = 0x03;
        }
    }
    #[doc = "SAI2 MCLK3 source select"]
    pub mod SAI2_MCLK3_SEL {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ccm.spdif0_clk_root"]
            pub const SAI2_MCLK3_SEL_0: u32 = 0;
            #[doc = "SPDIF_EXT_CLK"]
            pub const SAI2_MCLK3_SEL_1: u32 = 0x01;
            #[doc = "spdif.spdif_srclk"]
            pub const SAI2_MCLK3_SEL_2: u32 = 0x02;
            #[doc = "spdif.spdif_outclock"]
            pub const SAI2_MCLK3_SEL_3: u32 = 0x03;
        }
    }
    #[doc = "SAI3 MCLK3 source select"]
    pub mod SAI3_MCLK3_SEL {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ccm.spdif0_clk_root"]
            pub const SAI3_MCLK3_SEL_0: u32 = 0;
            #[doc = "SPDIF_EXT_CLK"]
            pub const SAI3_MCLK3_SEL_1: u32 = 0x01;
            #[doc = "spdif.spdif_srclk"]
            pub const SAI3_MCLK3_SEL_2: u32 = 0x02;
            #[doc = "spdif.spdif_outclock"]
            pub const SAI3_MCLK3_SEL_3: u32 = 0x03;
        }
    }
    #[doc = "Global interrupt \"0\" bit (connected to ARM M7 IRQ#0 and GPC)"]
    pub mod GINT {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Global interrupt request is not asserted."]
            pub const GINT_0: u32 = 0;
            #[doc = "Global interrupt request is asserted."]
            pub const GINT_1: u32 = 0x01;
        }
    }
    #[doc = "ENET1 reference clock mode select."]
    pub mod ENET1_CLK_SEL {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ENET1 TX reference clock driven by ref_enetpll."]
            pub const ENET1_CLK_SEL_0: u32 = 0;
            #[doc = "Gets ENET1 TX reference clock from the ENET1_TX_CLK pin. In this use case, an external OSC provides the clock for both the external PHY and the internal controller."]
            pub const ENET1_CLK_SEL_1: u32 = 0x01;
        }
    }
    #[doc = "ENET2 reference clock mode select."]
    pub mod ENET2_CLK_SEL {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ENET2 TX reference clock driven by ref_enetpll. This clock is also output to pins via the IOMUX. ENET2_REF_CLK function."]
            pub const ENET2_CLK_SEL_0: u32 = 0;
            #[doc = "Gets ENET2 TX reference clock from the ENET2_TX_CLK pin. In this use case, an external OSC provides the clock for both the external PHY and the internal controller."]
            pub const ENET2_CLK_SEL_1: u32 = 0x01;
        }
    }
    #[doc = "USB Exposure mode"]
    pub mod USB_EXP_MODE {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Exposure mode is disabled."]
            pub const USB_EXP_MODE_0: u32 = 0;
            #[doc = "Exposure mode is enabled."]
            pub const USB_EXP_MODE_1: u32 = 0x01;
        }
    }
    #[doc = "ENET1_TX_CLK data direction control"]
    pub mod ENET1_TX_CLK_DIR {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ENET1_TX_CLK output driver is disabled"]
            pub const ENET1_TX_CLK_DIR_0: u32 = 0;
            #[doc = "ENET1_TX_CLK output driver is enabled"]
            pub const ENET1_TX_CLK_DIR_1: u32 = 0x01;
        }
    }
    #[doc = "ENET2_TX_CLK data direction control"]
    pub mod ENET2_TX_CLK_DIR {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ENET2_TX_CLK output driver is disabled"]
            pub const ENET2_TX_CLK_DIR_0: u32 = 0;
            #[doc = "ENET2_TX_CLK output driver is enabled"]
            pub const ENET2_TX_CLK_DIR_1: u32 = 0x01;
        }
    }
    #[doc = "sai1.MCLK signal direction control"]
    pub mod SAI1_MCLK_DIR {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "sai1.MCLK is input signal"]
            pub const SAI1_MCLK_DIR_0: u32 = 0;
            #[doc = "sai1.MCLK is output signal"]
            pub const SAI1_MCLK_DIR_1: u32 = 0x01;
        }
    }
    #[doc = "sai2.MCLK signal direction control"]
    pub mod SAI2_MCLK_DIR {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "sai2.MCLK is input signal"]
            pub const SAI2_MCLK_DIR_0: u32 = 0;
            #[doc = "sai2.MCLK is output signal"]
            pub const SAI2_MCLK_DIR_1: u32 = 0x01;
        }
    }
    #[doc = "sai3.MCLK signal direction control"]
    pub mod SAI3_MCLK_DIR {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "sai3.MCLK is input signal"]
            pub const SAI3_MCLK_DIR_0: u32 = 0;
            #[doc = "sai3.MCLK is output signal"]
            pub const SAI3_MCLK_DIR_1: u32 = 0x01;
        }
    }
    #[doc = "Exclusive monitor response select of illegal command"]
    pub mod EXC_MON {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "OKAY response"]
            pub const EXC_MON_0: u32 = 0;
            #[doc = "SLVError response"]
            pub const EXC_MON_1: u32 = 0x01;
        }
    }
    #[doc = "ENET and ENET2 ipg_clk_s clock gating enable"]
    pub mod ENET_IPG_CLK_S_EN {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ipg_clk_s is gated when there is no IPS access"]
            pub const ENET_IPG_CLK_S_EN_0: u32 = 0;
            #[doc = "ipg_clk_s is always on"]
            pub const ENET_IPG_CLK_S_EN_1: u32 = 0x01;
        }
    }
    #[doc = "ARM CM7 platform AHB clock enable"]
    pub mod CM7_FORCE_HCLK_EN {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "AHB clock is not running (gated) when CM7 is sleeping and TCM is not accessible."]
            pub const CM7_FORCE_HCLK_EN_0: u32 = 0;
            #[doc = "AHB clock is running (enabled) when CM7 is sleeping and TCM is accessible."]
            pub const CM7_FORCE_HCLK_EN_1: u32 = 0x01;
        }
    }
}
#[doc = "GPR2 General Purpose Register"]
pub mod GPR2 {
    #[doc = "AXBS_L AHBXL master has higher priority.Do not set both DMA and AHBXL to high priority."]
    pub mod AXBS_L_AHBXL_HIGH_PRIORITY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "AXBS_L AHBXL master does not have high priority"]
            pub const AXBS_L_AHBXL_HIGH_PRIORITY_0: u32 = 0;
            #[doc = "AXBS_P AHBXL master has high priority"]
            pub const AXBS_L_AHBXL_HIGH_PRIORITY_1: u32 = 0x01;
        }
    }
    #[doc = "AXBS_L DMA master has higher priority.Do not set both DMA and AHBXL to high priority."]
    pub mod AXBS_L_DMA_HIGH_PRIORITY {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "AXBS_L DMA master does not have high priority"]
            pub const AXBS_L_DMA_HIGH_PRIORITY_0: u32 = 0;
            #[doc = "AXBS_L DMA master has high priority"]
            pub const AXBS_L_DMA_HIGH_PRIORITY_1: u32 = 0x01;
        }
    }
    #[doc = "Force Round Robin in AXBS_L"]
    pub mod AXBS_L_FORCE_ROUND_ROBIN {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "AXBS_L masters are not arbitored in round robin, depending on DMA and AHBXL master priority settings."]
            pub const AXBS_L_FORCE_ROUND_ROBIN_0: u32 = 0;
            #[doc = "AXBS_L masters are arbitored in round robin"]
            pub const AXBS_L_FORCE_ROUND_ROBIN_1: u32 = 0x01;
        }
    }
    #[doc = "AXBS_P M0 master has higher priority.Do not set both M1 and M0 to high priority."]
    pub mod AXBS_P_M0_HIGH_PRIORITY {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "AXBS_P M0 master doesn't have high priority"]
            pub const AXBS_P_M0_HIGH_PRIORITY_0: u32 = 0;
            #[doc = "AXBS_P M0 master has high priority"]
            pub const AXBS_P_M0_HIGH_PRIORITY_1: u32 = 0x01;
        }
    }
    #[doc = "AXBS_P M1 master has higher priority.Do not set both M1 and M0 to high priority."]
    pub mod AXBS_P_M1_HIGH_PRIORITY {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "AXBS_P M1 master does not have high priority"]
            pub const AXBS_P_M1_HIGH_PRIORITY_0: u32 = 0;
            #[doc = "AXBS_P M1 master has high priority"]
            pub const AXBS_P_M1_HIGH_PRIORITY_1: u32 = 0x01;
        }
    }
    #[doc = "Force Round Robin in AXBS_P. This bit can override master M0 M1 high priority configuration."]
    pub mod AXBS_P_FORCE_ROUND_ROBIN {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "AXBS_P masters are not arbitored in round robin, depending on M0/M1 master priority settings."]
            pub const AXBS_P_FORCE_ROUND_ROBIN_0: u32 = 0;
            #[doc = "AXBS_P masters are arbitored in round robin"]
            pub const AXBS_P_FORCE_ROUND_ROBIN_1: u32 = 0x01;
        }
    }
    #[doc = "Disable CANFD filter"]
    pub mod CANFD_FILTER_BYPASS {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "enable CANFD filter"]
            pub const CANFD_FILTER_BYPASS_0: u32 = 0;
            #[doc = "disable CANFD filter"]
            pub const CANFD_FILTER_BYPASS_1: u32 = 0x01;
        }
    }
    #[doc = "enable power saving features on L2 memory"]
    pub mod L2_MEM_EN_POWERSAVING {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Enters power saving mode only when chip is in SUSPEND mode"]
            pub const L2_MEM_EN_POWERSAVING_0: u32 = 0;
            #[doc = "Controlled by L2_MEM_DEEPSLEEP bitfield"]
            pub const L2_MEM_EN_POWERSAVING_1: u32 = 0x01;
        }
    }
    #[doc = "Automatically gate off RAM clock when RAM is not accessed."]
    pub mod RAM_AUTO_CLK_GATING_EN {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "disable automatically gate off RAM clock"]
            pub const RAM_AUTO_CLK_GATING_EN_0: u32 = 0;
            #[doc = "enable automatically gate off RAM clock"]
            pub const RAM_AUTO_CLK_GATING_EN_1: u32 = 0x01;
        }
    }
    #[doc = "control how memory enter Deep Sleep mode (shutdown periphery power, but maintain memory contents, outputs of memory are pulled low)"]
    pub mod L2_MEM_DEEPSLEEP {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No force sleep control supported, memory deep sleep mode only entered when whole system in stop mode (OCRAM in normal mode)"]
            pub const L2_MEM_DEEPSLEEP_0: u32 = 0;
            #[doc = "Force memory into deep sleep mode (OCRAM in power saving mode)"]
            pub const L2_MEM_DEEPSLEEP_1: u32 = 0x01;
        }
    }
    #[doc = "Divider ratio control for mclk from hmclk. mclk frequency = 1/(n+1) * hmclk frequency."]
    pub mod MQS_CLK_DIV {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "mclk frequency = 1/1 * hmclk frequency"]
            pub const DIVIDE_1: u32 = 0;
            #[doc = "mclk frequency = 1/2 * hmclk frequency"]
            pub const DIVIDE_2: u32 = 0x01;
            #[doc = "mclk frequency = 1/3 * hmclk frequency"]
            pub const DIVIDE_3: u32 = 0x02;
            #[doc = "mclk frequency = 1/4 * hmclk frequency"]
            pub const DIVIDE_4: u32 = 0x03;
            #[doc = "mclk frequency = 1/5 * hmclk frequency"]
            pub const DIVIDE_5: u32 = 0x04;
            #[doc = "mclk frequency = 1/6 * hmclk frequency"]
            pub const DIVIDE_6: u32 = 0x05;
            #[doc = "mclk frequency = 1/7 * hmclk frequency"]
            pub const DIVIDE_7: u32 = 0x06;
            #[doc = "mclk frequency = 1/8 * hmclk frequency"]
            pub const DIVIDE_8: u32 = 0x07;
            #[doc = "mclk frequency = 1/9 * hmclk frequency"]
            pub const DIVIDE_9: u32 = 0x08;
            #[doc = "mclk frequency = 1/10 * hmclk frequency"]
            pub const DIVIDE_10: u32 = 0x09;
            #[doc = "mclk frequency = 1/11 * hmclk frequency"]
            pub const DIVIDE_11: u32 = 0x0a;
            #[doc = "mclk frequency = 1/12 * hmclk frequency"]
            pub const DIVIDE_12: u32 = 0x0b;
            #[doc = "mclk frequency = 1/13 * hmclk frequency"]
            pub const DIVIDE_13: u32 = 0x0c;
            #[doc = "mclk frequency = 1/14 * hmclk frequency"]
            pub const DIVIDE_14: u32 = 0x0d;
            #[doc = "mclk frequency = 1/15 * hmclk frequency"]
            pub const DIVIDE_15: u32 = 0x0e;
            #[doc = "mclk frequency = 1/16 * hmclk frequency"]
            pub const DIVIDE_16: u32 = 0x0f;
            #[doc = "mclk frequency = 1/17 * hmclk frequency"]
            pub const DIVIDE_17: u32 = 0x10;
            #[doc = "mclk frequency = 1/18 * hmclk frequency"]
            pub const DIVIDE_18: u32 = 0x11;
            #[doc = "mclk frequency = 1/19 * hmclk frequency"]
            pub const DIVIDE_19: u32 = 0x12;
            #[doc = "mclk frequency = 1/20 * hmclk frequency"]
            pub const DIVIDE_20: u32 = 0x13;
            #[doc = "mclk frequency = 1/21 * hmclk frequency"]
            pub const DIVIDE_21: u32 = 0x14;
            #[doc = "mclk frequency = 1/22 * hmclk frequency"]
            pub const DIVIDE_22: u32 = 0x15;
            #[doc = "mclk frequency = 1/23 * hmclk frequency"]
            pub const DIVIDE_23: u32 = 0x16;
            #[doc = "mclk frequency = 1/24 * hmclk frequency"]
            pub const DIVIDE_24: u32 = 0x17;
            #[doc = "mclk frequency = 1/25 * hmclk frequency"]
            pub const DIVIDE_25: u32 = 0x18;
            #[doc = "mclk frequency = 1/26 * hmclk frequency"]
            pub const DIVIDE_26: u32 = 0x19;
            #[doc = "mclk frequency = 1/27 * hmclk frequency"]
            pub const DIVIDE_27: u32 = 0x1a;
            #[doc = "mclk frequency = 1/28 * hmclk frequency"]
            pub const DIVIDE_28: u32 = 0x1b;
            #[doc = "mclk frequency = 1/29 * hmclk frequency"]
            pub const DIVIDE_29: u32 = 0x1c;
            #[doc = "mclk frequency = 1/30 * hmclk frequency"]
            pub const DIVIDE_30: u32 = 0x1d;
            #[doc = "mclk frequency = 1/31 * hmclk frequency"]
            pub const DIVIDE_31: u32 = 0x1e;
            #[doc = "mclk frequency = 1/32 * hmclk frequency"]
            pub const DIVIDE_32: u32 = 0x1f;
            #[doc = "mclk frequency = 1/33 * hmclk frequency"]
            pub const DIVIDE_33: u32 = 0x20;
            #[doc = "mclk frequency = 1/34 * hmclk frequency"]
            pub const DIVIDE_34: u32 = 0x21;
            #[doc = "mclk frequency = 1/35 * hmclk frequency"]
            pub const DIVIDE_35: u32 = 0x22;
            #[doc = "mclk frequency = 1/36 * hmclk frequency"]
            pub const DIVIDE_36: u32 = 0x23;
            #[doc = "mclk frequency = 1/37 * hmclk frequency"]
            pub const DIVIDE_37: u32 = 0x24;
            #[doc = "mclk frequency = 1/38 * hmclk frequency"]
            pub const DIVIDE_38: u32 = 0x25;
            #[doc = "mclk frequency = 1/39 * hmclk frequency"]
            pub const DIVIDE_39: u32 = 0x26;
            #[doc = "mclk frequency = 1/40 * hmclk frequency"]
            pub const DIVIDE_40: u32 = 0x27;
            #[doc = "mclk frequency = 1/41 * hmclk frequency"]
            pub const DIVIDE_41: u32 = 0x28;
            #[doc = "mclk frequency = 1/42 * hmclk frequency"]
            pub const DIVIDE_42: u32 = 0x29;
            #[doc = "mclk frequency = 1/43 * hmclk frequency"]
            pub const DIVIDE_43: u32 = 0x2a;
            #[doc = "mclk frequency = 1/44 * hmclk frequency"]
            pub const DIVIDE_44: u32 = 0x2b;
            #[doc = "mclk frequency = 1/45 * hmclk frequency"]
            pub const DIVIDE_45: u32 = 0x2c;
            #[doc = "mclk frequency = 1/46 * hmclk frequency"]
            pub const DIVIDE_46: u32 = 0x2d;
            #[doc = "mclk frequency = 1/47 * hmclk frequency"]
            pub const DIVIDE_47: u32 = 0x2e;
            #[doc = "mclk frequency = 1/48 * hmclk frequency"]
            pub const DIVIDE_48: u32 = 0x2f;
            #[doc = "mclk frequency = 1/49 * hmclk frequency"]
            pub const DIVIDE_49: u32 = 0x30;
            #[doc = "mclk frequency = 1/50 * hmclk frequency"]
            pub const DIVIDE_50: u32 = 0x31;
            #[doc = "mclk frequency = 1/51 * hmclk frequency"]
            pub const DIVIDE_51: u32 = 0x32;
            #[doc = "mclk frequency = 1/52 * hmclk frequency"]
            pub const DIVIDE_52: u32 = 0x33;
            #[doc = "mclk frequency = 1/53 * hmclk frequency"]
            pub const DIVIDE_53: u32 = 0x34;
            #[doc = "mclk frequency = 1/54 * hmclk frequency"]
            pub const DIVIDE_54: u32 = 0x35;
            #[doc = "mclk frequency = 1/55 * hmclk frequency"]
            pub const DIVIDE_55: u32 = 0x36;
            #[doc = "mclk frequency = 1/56 * hmclk frequency"]
            pub const DIVIDE_56: u32 = 0x37;
            #[doc = "mclk frequency = 1/57 * hmclk frequency"]
            pub const DIVIDE_57: u32 = 0x38;
            #[doc = "mclk frequency = 1/58 * hmclk frequency"]
            pub const DIVIDE_58: u32 = 0x39;
            #[doc = "mclk frequency = 1/59 * hmclk frequency"]
            pub const DIVIDE_59: u32 = 0x3a;
            #[doc = "mclk frequency = 1/60 * hmclk frequency"]
            pub const DIVIDE_60: u32 = 0x3b;
            #[doc = "mclk frequency = 1/61 * hmclk frequency"]
            pub const DIVIDE_61: u32 = 0x3c;
            #[doc = "mclk frequency = 1/62 * hmclk frequency"]
            pub const DIVIDE_62: u32 = 0x3d;
            #[doc = "mclk frequency = 1/63 * hmclk frequency"]
            pub const DIVIDE_63: u32 = 0x3e;
            #[doc = "mclk frequency = 1/64 * hmclk frequency"]
            pub const DIVIDE_64: u32 = 0x3f;
            #[doc = "mclk frequency = 1/65 * hmclk frequency"]
            pub const DIVIDE_65: u32 = 0x40;
            #[doc = "mclk frequency = 1/66 * hmclk frequency"]
            pub const DIVIDE_66: u32 = 0x41;
            #[doc = "mclk frequency = 1/67 * hmclk frequency"]
            pub const DIVIDE_67: u32 = 0x42;
            #[doc = "mclk frequency = 1/68 * hmclk frequency"]
            pub const DIVIDE_68: u32 = 0x43;
            #[doc = "mclk frequency = 1/69 * hmclk frequency"]
            pub const DIVIDE_69: u32 = 0x44;
            #[doc = "mclk frequency = 1/70 * hmclk frequency"]
            pub const DIVIDE_70: u32 = 0x45;
            #[doc = "mclk frequency = 1/71 * hmclk frequency"]
            pub const DIVIDE_71: u32 = 0x46;
            #[doc = "mclk frequency = 1/72 * hmclk frequency"]
            pub const DIVIDE_72: u32 = 0x47;
            #[doc = "mclk frequency = 1/73 * hmclk frequency"]
            pub const DIVIDE_73: u32 = 0x48;
            #[doc = "mclk frequency = 1/74 * hmclk frequency"]
            pub const DIVIDE_74: u32 = 0x49;
            #[doc = "mclk frequency = 1/75 * hmclk frequency"]
            pub const DIVIDE_75: u32 = 0x4a;
            #[doc = "mclk frequency = 1/76 * hmclk frequency"]
            pub const DIVIDE_76: u32 = 0x4b;
            #[doc = "mclk frequency = 1/77 * hmclk frequency"]
            pub const DIVIDE_77: u32 = 0x4c;
            #[doc = "mclk frequency = 1/78 * hmclk frequency"]
            pub const DIVIDE_78: u32 = 0x4d;
            #[doc = "mclk frequency = 1/79 * hmclk frequency"]
            pub const DIVIDE_79: u32 = 0x4e;
            #[doc = "mclk frequency = 1/80 * hmclk frequency"]
            pub const DIVIDE_80: u32 = 0x4f;
            #[doc = "mclk frequency = 1/81 * hmclk frequency"]
            pub const DIVIDE_81: u32 = 0x50;
            #[doc = "mclk frequency = 1/82 * hmclk frequency"]
            pub const DIVIDE_82: u32 = 0x51;
            #[doc = "mclk frequency = 1/83 * hmclk frequency"]
            pub const DIVIDE_83: u32 = 0x52;
            #[doc = "mclk frequency = 1/84 * hmclk frequency"]
            pub const DIVIDE_84: u32 = 0x53;
            #[doc = "mclk frequency = 1/85 * hmclk frequency"]
            pub const DIVIDE_85: u32 = 0x54;
            #[doc = "mclk frequency = 1/86 * hmclk frequency"]
            pub const DIVIDE_86: u32 = 0x55;
            #[doc = "mclk frequency = 1/87 * hmclk frequency"]
            pub const DIVIDE_87: u32 = 0x56;
            #[doc = "mclk frequency = 1/88 * hmclk frequency"]
            pub const DIVIDE_88: u32 = 0x57;
            #[doc = "mclk frequency = 1/89 * hmclk frequency"]
            pub const DIVIDE_89: u32 = 0x58;
            #[doc = "mclk frequency = 1/90 * hmclk frequency"]
            pub const DIVIDE_90: u32 = 0x59;
            #[doc = "mclk frequency = 1/91 * hmclk frequency"]
            pub const DIVIDE_91: u32 = 0x5a;
            #[doc = "mclk frequency = 1/92 * hmclk frequency"]
            pub const DIVIDE_92: u32 = 0x5b;
            #[doc = "mclk frequency = 1/93 * hmclk frequency"]
            pub const DIVIDE_93: u32 = 0x5c;
            #[doc = "mclk frequency = 1/94 * hmclk frequency"]
            pub const DIVIDE_94: u32 = 0x5d;
            #[doc = "mclk frequency = 1/95 * hmclk frequency"]
            pub const DIVIDE_95: u32 = 0x5e;
            #[doc = "mclk frequency = 1/96 * hmclk frequency"]
            pub const DIVIDE_96: u32 = 0x5f;
            #[doc = "mclk frequency = 1/97 * hmclk frequency"]
            pub const DIVIDE_97: u32 = 0x60;
            #[doc = "mclk frequency = 1/98 * hmclk frequency"]
            pub const DIVIDE_98: u32 = 0x61;
            #[doc = "mclk frequency = 1/99 * hmclk frequency"]
            pub const DIVIDE_99: u32 = 0x62;
            #[doc = "mclk frequency = 1/100 * hmclk frequency"]
            pub const DIVIDE_100: u32 = 0x63;
            #[doc = "mclk frequency = 1/101 * hmclk frequency"]
            pub const DIVIDE_101: u32 = 0x64;
            #[doc = "mclk frequency = 1/102 * hmclk frequency"]
            pub const DIVIDE_102: u32 = 0x65;
            #[doc = "mclk frequency = 1/103 * hmclk frequency"]
            pub const DIVIDE_103: u32 = 0x66;
            #[doc = "mclk frequency = 1/104 * hmclk frequency"]
            pub const DIVIDE_104: u32 = 0x67;
            #[doc = "mclk frequency = 1/105 * hmclk frequency"]
            pub const DIVIDE_105: u32 = 0x68;
            #[doc = "mclk frequency = 1/106 * hmclk frequency"]
            pub const DIVIDE_106: u32 = 0x69;
            #[doc = "mclk frequency = 1/107 * hmclk frequency"]
            pub const DIVIDE_107: u32 = 0x6a;
            #[doc = "mclk frequency = 1/108 * hmclk frequency"]
            pub const DIVIDE_108: u32 = 0x6b;
            #[doc = "mclk frequency = 1/109 * hmclk frequency"]
            pub const DIVIDE_109: u32 = 0x6c;
            #[doc = "mclk frequency = 1/110 * hmclk frequency"]
            pub const DIVIDE_110: u32 = 0x6d;
            #[doc = "mclk frequency = 1/111 * hmclk frequency"]
            pub const DIVIDE_111: u32 = 0x6e;
            #[doc = "mclk frequency = 1/112 * hmclk frequency"]
            pub const DIVIDE_112: u32 = 0x6f;
            #[doc = "mclk frequency = 1/113 * hmclk frequency"]
            pub const DIVIDE_113: u32 = 0x70;
            #[doc = "mclk frequency = 1/114 * hmclk frequency"]
            pub const DIVIDE_114: u32 = 0x71;
            #[doc = "mclk frequency = 1/115 * hmclk frequency"]
            pub const DIVIDE_115: u32 = 0x72;
            #[doc = "mclk frequency = 1/116 * hmclk frequency"]
            pub const DIVIDE_116: u32 = 0x73;
            #[doc = "mclk frequency = 1/117 * hmclk frequency"]
            pub const DIVIDE_117: u32 = 0x74;
            #[doc = "mclk frequency = 1/118 * hmclk frequency"]
            pub const DIVIDE_118: u32 = 0x75;
            #[doc = "mclk frequency = 1/119 * hmclk frequency"]
            pub const DIVIDE_119: u32 = 0x76;
            #[doc = "mclk frequency = 1/120 * hmclk frequency"]
            pub const DIVIDE_120: u32 = 0x77;
            #[doc = "mclk frequency = 1/121 * hmclk frequency"]
            pub const DIVIDE_121: u32 = 0x78;
            #[doc = "mclk frequency = 1/122 * hmclk frequency"]
            pub const DIVIDE_122: u32 = 0x79;
            #[doc = "mclk frequency = 1/123 * hmclk frequency"]
            pub const DIVIDE_123: u32 = 0x7a;
            #[doc = "mclk frequency = 1/124 * hmclk frequency"]
            pub const DIVIDE_124: u32 = 0x7b;
            #[doc = "mclk frequency = 1/125 * hmclk frequency"]
            pub const DIVIDE_125: u32 = 0x7c;
            #[doc = "mclk frequency = 1/126 * hmclk frequency"]
            pub const DIVIDE_126: u32 = 0x7d;
            #[doc = "mclk frequency = 1/127 * hmclk frequency"]
            pub const DIVIDE_127: u32 = 0x7e;
            #[doc = "mclk frequency = 1/128 * hmclk frequency"]
            pub const DIVIDE_128: u32 = 0x7f;
            #[doc = "mclk frequency = 1/129 * hmclk frequency"]
            pub const DIVIDE_129: u32 = 0x80;
            #[doc = "mclk frequency = 1/130 * hmclk frequency"]
            pub const DIVIDE_130: u32 = 0x81;
            #[doc = "mclk frequency = 1/131 * hmclk frequency"]
            pub const DIVIDE_131: u32 = 0x82;
            #[doc = "mclk frequency = 1/132 * hmclk frequency"]
            pub const DIVIDE_132: u32 = 0x83;
            #[doc = "mclk frequency = 1/133 * hmclk frequency"]
            pub const DIVIDE_133: u32 = 0x84;
            #[doc = "mclk frequency = 1/134 * hmclk frequency"]
            pub const DIVIDE_134: u32 = 0x85;
            #[doc = "mclk frequency = 1/135 * hmclk frequency"]
            pub const DIVIDE_135: u32 = 0x86;
            #[doc = "mclk frequency = 1/136 * hmclk frequency"]
            pub const DIVIDE_136: u32 = 0x87;
            #[doc = "mclk frequency = 1/137 * hmclk frequency"]
            pub const DIVIDE_137: u32 = 0x88;
            #[doc = "mclk frequency = 1/138 * hmclk frequency"]
            pub const DIVIDE_138: u32 = 0x89;
            #[doc = "mclk frequency = 1/139 * hmclk frequency"]
            pub const DIVIDE_139: u32 = 0x8a;
            #[doc = "mclk frequency = 1/140 * hmclk frequency"]
            pub const DIVIDE_140: u32 = 0x8b;
            #[doc = "mclk frequency = 1/141 * hmclk frequency"]
            pub const DIVIDE_141: u32 = 0x8c;
            #[doc = "mclk frequency = 1/142 * hmclk frequency"]
            pub const DIVIDE_142: u32 = 0x8d;
            #[doc = "mclk frequency = 1/143 * hmclk frequency"]
            pub const DIVIDE_143: u32 = 0x8e;
            #[doc = "mclk frequency = 1/144 * hmclk frequency"]
            pub const DIVIDE_144: u32 = 0x8f;
            #[doc = "mclk frequency = 1/145 * hmclk frequency"]
            pub const DIVIDE_145: u32 = 0x90;
            #[doc = "mclk frequency = 1/146 * hmclk frequency"]
            pub const DIVIDE_146: u32 = 0x91;
            #[doc = "mclk frequency = 1/147 * hmclk frequency"]
            pub const DIVIDE_147: u32 = 0x92;
            #[doc = "mclk frequency = 1/148 * hmclk frequency"]
            pub const DIVIDE_148: u32 = 0x93;
            #[doc = "mclk frequency = 1/149 * hmclk frequency"]
            pub const DIVIDE_149: u32 = 0x94;
            #[doc = "mclk frequency = 1/150 * hmclk frequency"]
            pub const DIVIDE_150: u32 = 0x95;
            #[doc = "mclk frequency = 1/151 * hmclk frequency"]
            pub const DIVIDE_151: u32 = 0x96;
            #[doc = "mclk frequency = 1/152 * hmclk frequency"]
            pub const DIVIDE_152: u32 = 0x97;
            #[doc = "mclk frequency = 1/153 * hmclk frequency"]
            pub const DIVIDE_153: u32 = 0x98;
            #[doc = "mclk frequency = 1/154 * hmclk frequency"]
            pub const DIVIDE_154: u32 = 0x99;
            #[doc = "mclk frequency = 1/155 * hmclk frequency"]
            pub const DIVIDE_155: u32 = 0x9a;
            #[doc = "mclk frequency = 1/156 * hmclk frequency"]
            pub const DIVIDE_156: u32 = 0x9b;
            #[doc = "mclk frequency = 1/157 * hmclk frequency"]
            pub const DIVIDE_157: u32 = 0x9c;
            #[doc = "mclk frequency = 1/158 * hmclk frequency"]
            pub const DIVIDE_158: u32 = 0x9d;
            #[doc = "mclk frequency = 1/159 * hmclk frequency"]
            pub const DIVIDE_159: u32 = 0x9e;
            #[doc = "mclk frequency = 1/160 * hmclk frequency"]
            pub const DIVIDE_160: u32 = 0x9f;
            #[doc = "mclk frequency = 1/161 * hmclk frequency"]
            pub const DIVIDE_161: u32 = 0xa0;
            #[doc = "mclk frequency = 1/162 * hmclk frequency"]
            pub const DIVIDE_162: u32 = 0xa1;
            #[doc = "mclk frequency = 1/163 * hmclk frequency"]
            pub const DIVIDE_163: u32 = 0xa2;
            #[doc = "mclk frequency = 1/164 * hmclk frequency"]
            pub const DIVIDE_164: u32 = 0xa3;
            #[doc = "mclk frequency = 1/165 * hmclk frequency"]
            pub const DIVIDE_165: u32 = 0xa4;
            #[doc = "mclk frequency = 1/166 * hmclk frequency"]
            pub const DIVIDE_166: u32 = 0xa5;
            #[doc = "mclk frequency = 1/167 * hmclk frequency"]
            pub const DIVIDE_167: u32 = 0xa6;
            #[doc = "mclk frequency = 1/168 * hmclk frequency"]
            pub const DIVIDE_168: u32 = 0xa7;
            #[doc = "mclk frequency = 1/169 * hmclk frequency"]
            pub const DIVIDE_169: u32 = 0xa8;
            #[doc = "mclk frequency = 1/170 * hmclk frequency"]
            pub const DIVIDE_170: u32 = 0xa9;
            #[doc = "mclk frequency = 1/171 * hmclk frequency"]
            pub const DIVIDE_171: u32 = 0xaa;
            #[doc = "mclk frequency = 1/172 * hmclk frequency"]
            pub const DIVIDE_172: u32 = 0xab;
            #[doc = "mclk frequency = 1/173 * hmclk frequency"]
            pub const DIVIDE_173: u32 = 0xac;
            #[doc = "mclk frequency = 1/174 * hmclk frequency"]
            pub const DIVIDE_174: u32 = 0xad;
            #[doc = "mclk frequency = 1/175 * hmclk frequency"]
            pub const DIVIDE_175: u32 = 0xae;
            #[doc = "mclk frequency = 1/176 * hmclk frequency"]
            pub const DIVIDE_176: u32 = 0xaf;
            #[doc = "mclk frequency = 1/177 * hmclk frequency"]
            pub const DIVIDE_177: u32 = 0xb0;
            #[doc = "mclk frequency = 1/178 * hmclk frequency"]
            pub const DIVIDE_178: u32 = 0xb1;
            #[doc = "mclk frequency = 1/179 * hmclk frequency"]
            pub const DIVIDE_179: u32 = 0xb2;
            #[doc = "mclk frequency = 1/180 * hmclk frequency"]
            pub const DIVIDE_180: u32 = 0xb3;
            #[doc = "mclk frequency = 1/181 * hmclk frequency"]
            pub const DIVIDE_181: u32 = 0xb4;
            #[doc = "mclk frequency = 1/182 * hmclk frequency"]
            pub const DIVIDE_182: u32 = 0xb5;
            #[doc = "mclk frequency = 1/183 * hmclk frequency"]
            pub const DIVIDE_183: u32 = 0xb6;
            #[doc = "mclk frequency = 1/184 * hmclk frequency"]
            pub const DIVIDE_184: u32 = 0xb7;
            #[doc = "mclk frequency = 1/185 * hmclk frequency"]
            pub const DIVIDE_185: u32 = 0xb8;
            #[doc = "mclk frequency = 1/186 * hmclk frequency"]
            pub const DIVIDE_186: u32 = 0xb9;
            #[doc = "mclk frequency = 1/187 * hmclk frequency"]
            pub const DIVIDE_187: u32 = 0xba;
            #[doc = "mclk frequency = 1/188 * hmclk frequency"]
            pub const DIVIDE_188: u32 = 0xbb;
            #[doc = "mclk frequency = 1/189 * hmclk frequency"]
            pub const DIVIDE_189: u32 = 0xbc;
            #[doc = "mclk frequency = 1/190 * hmclk frequency"]
            pub const DIVIDE_190: u32 = 0xbd;
            #[doc = "mclk frequency = 1/191 * hmclk frequency"]
            pub const DIVIDE_191: u32 = 0xbe;
            #[doc = "mclk frequency = 1/192 * hmclk frequency"]
            pub const DIVIDE_192: u32 = 0xbf;
            #[doc = "mclk frequency = 1/193 * hmclk frequency"]
            pub const DIVIDE_193: u32 = 0xc0;
            #[doc = "mclk frequency = 1/194 * hmclk frequency"]
            pub const DIVIDE_194: u32 = 0xc1;
            #[doc = "mclk frequency = 1/195 * hmclk frequency"]
            pub const DIVIDE_195: u32 = 0xc2;
            #[doc = "mclk frequency = 1/196 * hmclk frequency"]
            pub const DIVIDE_196: u32 = 0xc3;
            #[doc = "mclk frequency = 1/197 * hmclk frequency"]
            pub const DIVIDE_197: u32 = 0xc4;
            #[doc = "mclk frequency = 1/198 * hmclk frequency"]
            pub const DIVIDE_198: u32 = 0xc5;
            #[doc = "mclk frequency = 1/199 * hmclk frequency"]
            pub const DIVIDE_199: u32 = 0xc6;
            #[doc = "mclk frequency = 1/200 * hmclk frequency"]
            pub const DIVIDE_200: u32 = 0xc7;
            #[doc = "mclk frequency = 1/201 * hmclk frequency"]
            pub const DIVIDE_201: u32 = 0xc8;
            #[doc = "mclk frequency = 1/202 * hmclk frequency"]
            pub const DIVIDE_202: u32 = 0xc9;
            #[doc = "mclk frequency = 1/203 * hmclk frequency"]
            pub const DIVIDE_203: u32 = 0xca;
            #[doc = "mclk frequency = 1/204 * hmclk frequency"]
            pub const DIVIDE_204: u32 = 0xcb;
            #[doc = "mclk frequency = 1/205 * hmclk frequency"]
            pub const DIVIDE_205: u32 = 0xcc;
            #[doc = "mclk frequency = 1/206 * hmclk frequency"]
            pub const DIVIDE_206: u32 = 0xcd;
            #[doc = "mclk frequency = 1/207 * hmclk frequency"]
            pub const DIVIDE_207: u32 = 0xce;
            #[doc = "mclk frequency = 1/208 * hmclk frequency"]
            pub const DIVIDE_208: u32 = 0xcf;
            #[doc = "mclk frequency = 1/209 * hmclk frequency"]
            pub const DIVIDE_209: u32 = 0xd0;
            #[doc = "mclk frequency = 1/210 * hmclk frequency"]
            pub const DIVIDE_210: u32 = 0xd1;
            #[doc = "mclk frequency = 1/211 * hmclk frequency"]
            pub const DIVIDE_211: u32 = 0xd2;
            #[doc = "mclk frequency = 1/212 * hmclk frequency"]
            pub const DIVIDE_212: u32 = 0xd3;
            #[doc = "mclk frequency = 1/213 * hmclk frequency"]
            pub const DIVIDE_213: u32 = 0xd4;
            #[doc = "mclk frequency = 1/214 * hmclk frequency"]
            pub const DIVIDE_214: u32 = 0xd5;
            #[doc = "mclk frequency = 1/215 * hmclk frequency"]
            pub const DIVIDE_215: u32 = 0xd6;
            #[doc = "mclk frequency = 1/216 * hmclk frequency"]
            pub const DIVIDE_216: u32 = 0xd7;
            #[doc = "mclk frequency = 1/217 * hmclk frequency"]
            pub const DIVIDE_217: u32 = 0xd8;
            #[doc = "mclk frequency = 1/218 * hmclk frequency"]
            pub const DIVIDE_218: u32 = 0xd9;
            #[doc = "mclk frequency = 1/219 * hmclk frequency"]
            pub const DIVIDE_219: u32 = 0xda;
            #[doc = "mclk frequency = 1/220 * hmclk frequency"]
            pub const DIVIDE_220: u32 = 0xdb;
            #[doc = "mclk frequency = 1/221 * hmclk frequency"]
            pub const DIVIDE_221: u32 = 0xdc;
            #[doc = "mclk frequency = 1/222 * hmclk frequency"]
            pub const DIVIDE_222: u32 = 0xdd;
            #[doc = "mclk frequency = 1/223 * hmclk frequency"]
            pub const DIVIDE_223: u32 = 0xde;
            #[doc = "mclk frequency = 1/224 * hmclk frequency"]
            pub const DIVIDE_224: u32 = 0xdf;
            #[doc = "mclk frequency = 1/225 * hmclk frequency"]
            pub const DIVIDE_225: u32 = 0xe0;
            #[doc = "mclk frequency = 1/226 * hmclk frequency"]
            pub const DIVIDE_226: u32 = 0xe1;
            #[doc = "mclk frequency = 1/227 * hmclk frequency"]
            pub const DIVIDE_227: u32 = 0xe2;
            #[doc = "mclk frequency = 1/228 * hmclk frequency"]
            pub const DIVIDE_228: u32 = 0xe3;
            #[doc = "mclk frequency = 1/229 * hmclk frequency"]
            pub const DIVIDE_229: u32 = 0xe4;
            #[doc = "mclk frequency = 1/230 * hmclk frequency"]
            pub const DIVIDE_230: u32 = 0xe5;
            #[doc = "mclk frequency = 1/231 * hmclk frequency"]
            pub const DIVIDE_231: u32 = 0xe6;
            #[doc = "mclk frequency = 1/232 * hmclk frequency"]
            pub const DIVIDE_232: u32 = 0xe7;
            #[doc = "mclk frequency = 1/233 * hmclk frequency"]
            pub const DIVIDE_233: u32 = 0xe8;
            #[doc = "mclk frequency = 1/234 * hmclk frequency"]
            pub const DIVIDE_234: u32 = 0xe9;
            #[doc = "mclk frequency = 1/235 * hmclk frequency"]
            pub const DIVIDE_235: u32 = 0xea;
            #[doc = "mclk frequency = 1/236 * hmclk frequency"]
            pub const DIVIDE_236: u32 = 0xeb;
            #[doc = "mclk frequency = 1/237 * hmclk frequency"]
            pub const DIVIDE_237: u32 = 0xec;
            #[doc = "mclk frequency = 1/238 * hmclk frequency"]
            pub const DIVIDE_238: u32 = 0xed;
            #[doc = "mclk frequency = 1/239 * hmclk frequency"]
            pub const DIVIDE_239: u32 = 0xee;
            #[doc = "mclk frequency = 1/240 * hmclk frequency"]
            pub const DIVIDE_240: u32 = 0xef;
            #[doc = "mclk frequency = 1/241 * hmclk frequency"]
            pub const DIVIDE_241: u32 = 0xf0;
            #[doc = "mclk frequency = 1/242 * hmclk frequency"]
            pub const DIVIDE_242: u32 = 0xf1;
            #[doc = "mclk frequency = 1/243 * hmclk frequency"]
            pub const DIVIDE_243: u32 = 0xf2;
            #[doc = "mclk frequency = 1/244 * hmclk frequency"]
            pub const DIVIDE_244: u32 = 0xf3;
            #[doc = "mclk frequency = 1/245 * hmclk frequency"]
            pub const DIVIDE_245: u32 = 0xf4;
            #[doc = "mclk frequency = 1/246 * hmclk frequency"]
            pub const DIVIDE_246: u32 = 0xf5;
            #[doc = "mclk frequency = 1/247 * hmclk frequency"]
            pub const DIVIDE_247: u32 = 0xf6;
            #[doc = "mclk frequency = 1/248 * hmclk frequency"]
            pub const DIVIDE_248: u32 = 0xf7;
            #[doc = "mclk frequency = 1/249 * hmclk frequency"]
            pub const DIVIDE_249: u32 = 0xf8;
            #[doc = "mclk frequency = 1/250 * hmclk frequency"]
            pub const DIVIDE_250: u32 = 0xf9;
            #[doc = "mclk frequency = 1/251 * hmclk frequency"]
            pub const DIVIDE_251: u32 = 0xfa;
            #[doc = "mclk frequency = 1/252 * hmclk frequency"]
            pub const DIVIDE_252: u32 = 0xfb;
            #[doc = "mclk frequency = 1/253 * hmclk frequency"]
            pub const DIVIDE_253: u32 = 0xfc;
            #[doc = "mclk frequency = 1/254 * hmclk frequency"]
            pub const DIVIDE_254: u32 = 0xfd;
            #[doc = "mclk frequency = 1/255 * hmclk frequency"]
            pub const DIVIDE_255: u32 = 0xfe;
            #[doc = "mclk frequency = 1/256 * hmclk frequency"]
            pub const DIVIDE_256: u32 = 0xff;
        }
    }
    #[doc = "MQS software reset"]
    pub mod MQS_SW_RST {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Exit software reset for MQS"]
            pub const MQS_SW_RST_0: u32 = 0;
            #[doc = "Enable software reset for MQS"]
            pub const MQS_SW_RST_1: u32 = 0x01;
        }
    }
    #[doc = "MQS enable."]
    pub mod MQS_EN {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable MQS"]
            pub const MQS_EN_0: u32 = 0;
            #[doc = "Enable MQS"]
            pub const MQS_EN_1: u32 = 0x01;
        }
    }
    #[doc = "Used to control the PWM oversampling rate compared with mclk."]
    pub mod MQS_OVERSAMPLE {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "32"]
            pub const MQS_OVERSAMPLE_0: u32 = 0;
            #[doc = "64"]
            pub const MQS_OVERSAMPLE_1: u32 = 0x01;
        }
    }
    #[doc = "QTIMER1 timer counter freeze"]
    pub mod QTIMER1_TMR_CNTS_FREEZE {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Timer counter works normally"]
            pub const QTIMER1_TMR_CNTS_FREEZE_0: u32 = 0;
            #[doc = "Reset counter and ouput flags"]
            pub const QTIMER1_TMR_CNTS_FREEZE_1: u32 = 0x01;
        }
    }
    #[doc = "QTIMER2 timer counter freeze"]
    pub mod QTIMER2_TMR_CNTS_FREEZE {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "timer counter work normally"]
            pub const QTIMER2_TMR_CNTS_FREEZE_0: u32 = 0;
            #[doc = "reset counter and ouput flags"]
            pub const QTIMER2_TMR_CNTS_FREEZE_1: u32 = 0x01;
        }
    }
    #[doc = "QTIMER3 timer counter freeze"]
    pub mod QTIMER3_TMR_CNTS_FREEZE {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "timer counter work normally"]
            pub const QTIMER3_TMR_CNTS_FREEZE_0: u32 = 0;
            #[doc = "reset counter and ouput flags"]
            pub const QTIMER3_TMR_CNTS_FREEZE_1: u32 = 0x01;
        }
    }
    #[doc = "QTIMER4 timer counter freeze"]
    pub mod QTIMER4_TMR_CNTS_FREEZE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "timer counter work normally"]
            pub const QTIMER4_TMR_CNTS_FREEZE_0: u32 = 0;
            #[doc = "reset counter and ouput flags"]
            pub const QTIMER4_TMR_CNTS_FREEZE_1: u32 = 0x01;
        }
    }
}
#[doc = "GPR3 General Purpose Register"]
pub mod GPR3 {
    #[doc = "OCRAM_CTL\\[3\\] - write address pipeline control bit"]
    pub mod OCRAM_CTL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Select 128-bit dcp key from 256-bit key from snvs/ocotp"]
    pub mod DCP_KEY_SEL {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Select \\[127:0\\] from SNVS Master Key as DCP key"]
            pub const DCP_KEY_SEL_0: u32 = 0;
            #[doc = "Select \\[255:128\\] from SNVS Master Key as DCP key"]
            pub const DCP_KEY_SEL_1: u32 = 0x01;
        }
    }
    #[doc = "OCRAM2_CTL\\[3\\] - write address pipeline control bit"]
    pub mod OCRAM2_CTL {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Request to halt axbs_l"]
    pub mod AXBS_L_HALT_REQ {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "axbs_l normal run"]
            pub const AXBS_L_HALT_REQ_0: u32 = 0;
            #[doc = "request to halt axbs_l"]
            pub const AXBS_L_HALT_REQ_1: u32 = 0x01;
        }
    }
    #[doc = "This field shows the OCRAM pipeline settings status, controlled by OCRAM_CTL bits respectively"]
    pub mod OCRAM_STATUS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This field shows the OCRAM2 pipeline settings status, controlled by OCRAM2_CTL bits respectively"]
    pub mod OCRAM2_STATUS {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bit shows the status of axbs_l"]
    pub mod AXBS_L_HALTED {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "axbs_l is not halted"]
            pub const AXBS_L_HALTED_0: u32 = 0;
            #[doc = "axbs_l is in halted status"]
            pub const AXBS_L_HALTED_1: u32 = 0x01;
        }
    }
}
#[doc = "GPR4 General Purpose Register"]
pub mod GPR4 {
    #[doc = "EDMA stop request."]
    pub mod EDMA_STOP_REQ {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "stop request off"]
            pub const EDMA_STOP_REQ_0: u32 = 0;
            #[doc = "stop request on"]
            pub const EDMA_STOP_REQ_1: u32 = 0x01;
        }
    }
    #[doc = "CAN1 stop request."]
    pub mod CAN1_STOP_REQ {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "stop request off"]
            pub const CAN1_STOP_REQ_0: u32 = 0;
            #[doc = "stop request on"]
            pub const CAN1_STOP_REQ_1: u32 = 0x01;
        }
    }
    #[doc = "CAN2 stop request."]
    pub mod CAN2_STOP_REQ {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "stop request off"]
            pub const CAN2_STOP_REQ_0: u32 = 0;
            #[doc = "stop request on"]
            pub const CAN2_STOP_REQ_1: u32 = 0x01;
        }
    }
    #[doc = "TRNG stop request."]
    pub mod TRNG_STOP_REQ {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "stop request off"]
            pub const TRNG_STOP_REQ_0: u32 = 0;
            #[doc = "stop request on"]
            pub const TRNG_STOP_REQ_1: u32 = 0x01;
        }
    }
    #[doc = "ENET stop request."]
    pub mod ENET_STOP_REQ {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "stop request off"]
            pub const ENET_STOP_REQ_0: u32 = 0;
            #[doc = "stop request on"]
            pub const ENET_STOP_REQ_1: u32 = 0x01;
        }
    }
    #[doc = "SAI1 stop request."]
    pub mod SAI1_STOP_REQ {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "stop request off"]
            pub const SAI1_STOP_REQ_0: u32 = 0;
            #[doc = "stop request on"]
            pub const SAI1_STOP_REQ_1: u32 = 0x01;
        }
    }
    #[doc = "SAI2 stop request."]
    pub mod SAI2_STOP_REQ {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "stop request off"]
            pub const SAI2_STOP_REQ_0: u32 = 0;
            #[doc = "stop request on"]
            pub const SAI2_STOP_REQ_1: u32 = 0x01;
        }
    }
    #[doc = "SAI3 stop request."]
    pub mod SAI3_STOP_REQ {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "stop request off"]
            pub const SAI3_STOP_REQ_0: u32 = 0;
            #[doc = "stop request on"]
            pub const SAI3_STOP_REQ_1: u32 = 0x01;
        }
    }
    #[doc = "ENET2 stop request."]
    pub mod ENET2_STOP_REQ {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "stop request off"]
            pub const ENET2_STOP_REQ_0: u32 = 0;
            #[doc = "stop request on"]
            pub const ENET2_STOP_REQ_1: u32 = 0x01;
        }
    }
    #[doc = "SEMC stop request."]
    pub mod SEMC_STOP_REQ {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "stop request off"]
            pub const SEMC_STOP_REQ_0: u32 = 0;
            #[doc = "stop request on"]
            pub const SEMC_STOP_REQ_1: u32 = 0x01;
        }
    }
    #[doc = "PIT stop request."]
    pub mod PIT_STOP_REQ {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "stop request off"]
            pub const PIT_STOP_REQ_0: u32 = 0;
            #[doc = "stop request on"]
            pub const PIT_STOP_REQ_1: u32 = 0x01;
        }
    }
    #[doc = "FlexSPI stop request."]
    pub mod FLEXSPI_STOP_REQ {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "stop request off"]
            pub const FLEXSPI_STOP_REQ_0: u32 = 0;
            #[doc = "stop request on"]
            pub const FLEXSPI_STOP_REQ_1: u32 = 0x01;
        }
    }
    #[doc = "FlexIO1 stop request."]
    pub mod FLEXIO1_STOP_REQ {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "stop request off"]
            pub const FLEXIO1_STOP_REQ_0: u32 = 0;
            #[doc = "stop request on"]
            pub const FLEXIO1_STOP_REQ_1: u32 = 0x01;
        }
    }
    #[doc = "FlexIO2 stop request."]
    pub mod FLEXIO2_STOP_REQ {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "stop request off"]
            pub const FLEXIO2_STOP_REQ_0: u32 = 0;
            #[doc = "stop request on"]
            pub const FLEXIO2_STOP_REQ_1: u32 = 0x01;
        }
    }
    #[doc = "On-platform flexio3 stop request."]
    pub mod FLEXIO3_STOP_REQ {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "stop request off"]
            pub const FLEXIO3_STOP_REQ_0: u32 = 0;
            #[doc = "stop request on"]
            pub const FLEXIO3_STOP_REQ_1: u32 = 0x01;
        }
    }
    #[doc = "FlexSPI2 stop request."]
    pub mod FLEXSPI2_STOP_REQ {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "stop request off"]
            pub const FLEXSPI2_STOP_REQ_0: u32 = 0;
            #[doc = "stop request on"]
            pub const FLEXSPI2_STOP_REQ_1: u32 = 0x01;
        }
    }
    #[doc = "EDMA stop acknowledge. This is a status (read-only) bit"]
    pub mod EDMA_STOP_ACK {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "EDMA stop acknowledge is not asserted"]
            pub const EDMA_STOP_ACK_0: u32 = 0;
            #[doc = "EDMA stop acknowledge is asserted (EDMA is in STOP mode)."]
            pub const EDMA_STOP_ACK_1: u32 = 0x01;
        }
    }
    #[doc = "CAN1 stop acknowledge."]
    pub mod CAN1_STOP_ACK {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "CAN1 stop acknowledge is not asserted"]
            pub const CAN1_STOP_ACK_0: u32 = 0;
            #[doc = "CAN1 stop acknowledge is asserted"]
            pub const CAN1_STOP_ACK_1: u32 = 0x01;
        }
    }
    #[doc = "CAN2 stop acknowledge."]
    pub mod CAN2_STOP_ACK {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "CAN2 stop acknowledge is not asserted"]
            pub const CAN2_STOP_ACK_0: u32 = 0;
            #[doc = "CAN2 stop acknowledge is asserted"]
            pub const CAN2_STOP_ACK_1: u32 = 0x01;
        }
    }
    #[doc = "TRNG stop acknowledge"]
    pub mod TRNG_STOP_ACK {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "TRNG stop acknowledge is not asserted"]
            pub const TRNG_STOP_ACK_0: u32 = 0;
            #[doc = "TRNG stop acknowledge is asserted"]
            pub const TRNG_STOP_ACK_1: u32 = 0x01;
        }
    }
    #[doc = "ENET stop acknowledge."]
    pub mod ENET_STOP_ACK {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ENET stop acknowledge is not asserted"]
            pub const ENET_STOP_ACK_0: u32 = 0;
            #[doc = "ENET stop acknowledge is asserted"]
            pub const ENET_STOP_ACK_1: u32 = 0x01;
        }
    }
    #[doc = "SAI1 stop acknowledge"]
    pub mod SAI1_STOP_ACK {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "SAI1 stop acknowledge is not asserted"]
            pub const SAI1_STOP_ACK_0: u32 = 0;
            #[doc = "SAI1 stop acknowledge is asserted"]
            pub const SAI1_STOP_ACK_1: u32 = 0x01;
        }
    }
    #[doc = "SAI2 stop acknowledge"]
    pub mod SAI2_STOP_ACK {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "SAI2 stop acknowledge is not asserted"]
            pub const SAI2_STOP_ACK_0: u32 = 0;
            #[doc = "SAI2 stop acknowledge is asserted"]
            pub const SAI2_STOP_ACK_1: u32 = 0x01;
        }
    }
    #[doc = "SAI3 stop acknowledge"]
    pub mod SAI3_STOP_ACK {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "SAI3 stop acknowledge is not asserted"]
            pub const SAI3_STOP_ACK_0: u32 = 0;
            #[doc = "SAI3 stop acknowledge is asserted"]
            pub const SAI3_STOP_ACK_1: u32 = 0x01;
        }
    }
    #[doc = "ENET2 stop acknowledge."]
    pub mod ENET2_STOP_ACK {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ENET2 stop acknowledge is not asserted"]
            pub const ENET2_STOP_ACK_0: u32 = 0;
            #[doc = "ENET2 stop acknowledge is asserted"]
            pub const ENET2_STOP_ACK_1: u32 = 0x01;
        }
    }
    #[doc = "SEMC stop acknowledge"]
    pub mod SEMC_STOP_ACK {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "SEMC stop acknowledge is not asserted"]
            pub const SEMC_STOP_ACK_0: u32 = 0;
            #[doc = "SEMC stop acknowledge is asserted"]
            pub const SEMC_STOP_ACK_1: u32 = 0x01;
        }
    }
    #[doc = "PIT stop acknowledge"]
    pub mod PIT_STOP_ACK {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "PIT stop acknowledge is not asserted"]
            pub const PIT_STOP_ACK_0: u32 = 0;
            #[doc = "PIT stop acknowledge is asserted"]
            pub const PIT_STOP_ACK_1: u32 = 0x01;
        }
    }
    #[doc = "FLEXSPI stop acknowledge"]
    pub mod FLEXSPI_STOP_ACK {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "FLEXSPI stop acknowledge is not asserted"]
            pub const FLEXSPI_STOP_ACK_0: u32 = 0;
            #[doc = "FLEXSPI stop acknowledge is asserted"]
            pub const FLEXSPI_STOP_ACK_1: u32 = 0x01;
        }
    }
    #[doc = "FLEXIO1 stop acknowledge"]
    pub mod FLEXIO1_STOP_ACK {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "FLEXIO1 stop acknowledge is not asserted"]
            pub const FLEXIO1_STOP_ACK_0: u32 = 0;
            #[doc = "FLEXIO1 stop acknowledge is asserted"]
            pub const FLEXIO1_STOP_ACK_1: u32 = 0x01;
        }
    }
    #[doc = "FLEXIO2 stop acknowledge"]
    pub mod FLEXIO2_STOP_ACK {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "FLEXIO2 stop acknowledge is not asserted"]
            pub const FLEXIO2_STOP_ACK_0: u32 = 0;
            #[doc = "FLEXIO2 stop acknowledge is asserted (FLEXIO2 is in STOP mode)"]
            pub const FLEXIO2_STOP_ACK_1: u32 = 0x01;
        }
    }
    #[doc = "On-platform FLEXIO3 stop acknowledge"]
    pub mod FLEXIO3_STOP_ACK {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "FLEXIO3 stop acknowledge is not asserted"]
            pub const FLEXIO3_STOP_ACK_0: u32 = 0;
            #[doc = "FLEXIO3 stop acknowledge is asserted"]
            pub const FLEXIO3_STOP_ACK_1: u32 = 0x01;
        }
    }
    #[doc = "FLEXSPI2 stop acknowledge"]
    pub mod FLEXSPI2_STOP_ACK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "FLEXSPI2 stop acknowledge is not asserted"]
            pub const FLEXSPI2_STOP_ACK_0: u32 = 0;
            #[doc = "FLEXSPI2 stop acknowledge is asserted"]
            pub const FLEXSPI2_STOP_ACK_1: u32 = 0x01;
        }
    }
}
#[doc = "GPR5 General Purpose Register"]
pub mod GPR5 {
    #[doc = "WDOG1 Timeout Mask"]
    pub mod WDOG1_MASK {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "WDOG1 Timeout behaves normally"]
            pub const WDOG1_MASK_0: u32 = 0;
            #[doc = "WDOG1 Timeout is masked"]
            pub const WDOG1_MASK_1: u32 = 0x01;
        }
    }
    #[doc = "WDOG2 Timeout Mask"]
    pub mod WDOG2_MASK {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "WDOG2 Timeout behaves normally"]
            pub const WDOG2_MASK_0: u32 = 0;
            #[doc = "WDOG2 Timeout is masked"]
            pub const WDOG2_MASK_1: u32 = 0x01;
        }
    }
    #[doc = "GPT2 input capture channel 1 source select"]
    pub mod GPT2_CAPIN1_SEL {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "source from GPT2_CAPTURE1"]
            pub const GPT2_CAPIN1_SEL_0: u32 = 0;
            #[doc = "source from ENET_1588_EVENT3_OUT (chnnal 3 of IEEE 1588 timer)"]
            pub const GPT2_CAPIN1_SEL_1: u32 = 0x01;
        }
    }
    #[doc = "GPT2 input capture channel 2 source select"]
    pub mod GPT2_CAPIN2_SEL {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "source from GPT2_CAPTURE2"]
            pub const GPT2_CAPIN2_SEL_0: u32 = 0;
            #[doc = "source from ENET2_1588_EVENT3_OUT (chnnal 3 of IEEE 1588 timer)"]
            pub const GPT2_CAPIN2_SEL_1: u32 = 0x01;
        }
    }
    #[doc = "ENET input timer event3 source select"]
    pub mod ENET_EVENT3IN_SEL {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "event3 source input from ENET_1588_EVENT3_IN"]
            pub const ENET_EVENT3IN_SEL_0: u32 = 0;
            #[doc = "event3 source input from GPT2.GPT_COMPARE1"]
            pub const ENET_EVENT3IN_SEL_1: u32 = 0x01;
        }
    }
    #[doc = "ENET2 input timer event3 source select"]
    pub mod ENET2_EVENT3IN_SEL {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "event3 source input from ENET2_1588_EVENT3_IN"]
            pub const ENET2_EVENT3IN_SEL_0: u32 = 0;
            #[doc = "event3 source input from GPT2.GPT_COMPARE2"]
            pub const ENET2_EVENT3IN_SEL_1: u32 = 0x01;
        }
    }
    #[doc = "GPT1 1 MHz clock source select"]
    pub mod VREF_1M_CLK_GPT1 {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "GPT1 ipg_clk_highfreq driven by IPG_PERCLK"]
            pub const VREF_1M_CLK_GPT1_0: u32 = 0;
            #[doc = "GPT1 ipg_clk_highfreq driven by anatop 1 MHz clock"]
            pub const VREF_1M_CLK_GPT1_1: u32 = 0x01;
        }
    }
    #[doc = "GPT2 1 MHz clock source select"]
    pub mod VREF_1M_CLK_GPT2 {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "GPT2 ipg_clk_highfreq driven by IPG_PERCLK"]
            pub const VREF_1M_CLK_GPT2_0: u32 = 0;
            #[doc = "GPT2 ipg_clk_highfreq driven by anatop 1 MHz clock"]
            pub const VREF_1M_CLK_GPT2_1: u32 = 0x01;
        }
    }
}
#[doc = "GPR6 General Purpose Register"]
pub mod GPR6 {
    #[doc = "QTIMER1 TMR0 input select"]
    pub mod QTIMER1_TRM0_INPUT_SEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "input from IOMUX"]
            pub const QTIMER1_TRM0_INPUT_SEL_0: u32 = 0;
            #[doc = "input from XBAR"]
            pub const QTIMER1_TRM0_INPUT_SEL_1: u32 = 0x01;
        }
    }
    #[doc = "QTIMER1 TMR1 input select"]
    pub mod QTIMER1_TRM1_INPUT_SEL {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "input from IOMUX"]
            pub const QTIMER1_TRM1_INPUT_SEL_0: u32 = 0;
            #[doc = "input from XBAR"]
            pub const QTIMER1_TRM1_INPUT_SEL_1: u32 = 0x01;
        }
    }
    #[doc = "QTIMER1 TMR2 input select"]
    pub mod QTIMER1_TRM2_INPUT_SEL {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "input from IOMUX"]
            pub const QTIMER1_TRM2_INPUT_SEL_0: u32 = 0;
            #[doc = "input from XBAR"]
            pub const QTIMER1_TRM2_INPUT_SEL_1: u32 = 0x01;
        }
    }
    #[doc = "QTIMER1 TMR3 input select"]
    pub mod QTIMER1_TRM3_INPUT_SEL {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "input from IOMUX"]
            pub const QTIMER1_TRM3_INPUT_SEL_0: u32 = 0;
            #[doc = "input from XBAR"]
            pub const QTIMER1_TRM3_INPUT_SEL_1: u32 = 0x01;
        }
    }
    #[doc = "QTIMER2 TMR0 input select"]
    pub mod QTIMER2_TRM0_INPUT_SEL {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "input from IOMUX"]
            pub const QTIMER2_TRM0_INPUT_SEL_0: u32 = 0;
            #[doc = "input from XBAR"]
            pub const QTIMER2_TRM0_INPUT_SEL_1: u32 = 0x01;
        }
    }
    #[doc = "QTIMER2 TMR1 input select"]
    pub mod QTIMER2_TRM1_INPUT_SEL {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "input from IOMUX"]
            pub const QTIMER2_TRM1_INPUT_SEL_0: u32 = 0;
            #[doc = "input from XBAR"]
            pub const QTIMER2_TRM1_INPUT_SEL_1: u32 = 0x01;
        }
    }
    #[doc = "QTIMER2 TMR2 input select"]
    pub mod QTIMER2_TRM2_INPUT_SEL {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "input from IOMUX"]
            pub const QTIMER2_TRM2_INPUT_SEL_0: u32 = 0;
            #[doc = "input from XBAR"]
            pub const QTIMER2_TRM2_INPUT_SEL_1: u32 = 0x01;
        }
    }
    #[doc = "QTIMER2 TMR3 input select"]
    pub mod QTIMER2_TRM3_INPUT_SEL {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "input from IOMUX"]
            pub const QTIMER2_TRM3_INPUT_SEL_0: u32 = 0;
            #[doc = "input from XBAR"]
            pub const QTIMER2_TRM3_INPUT_SEL_1: u32 = 0x01;
        }
    }
    #[doc = "QTIMER3 TMR0 input select"]
    pub mod QTIMER3_TRM0_INPUT_SEL {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "input from IOMUX"]
            pub const QTIMER3_TRM0_INPUT_SEL_0: u32 = 0;
            #[doc = "input from XBAR"]
            pub const QTIMER3_TRM0_INPUT_SEL_1: u32 = 0x01;
        }
    }
    #[doc = "QTIMER3 TMR1 input select"]
    pub mod QTIMER3_TRM1_INPUT_SEL {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "input from IOMUX"]
            pub const QTIMER3_TRM1_INPUT_SEL_0: u32 = 0;
            #[doc = "input from XBAR"]
            pub const QTIMER3_TRM1_INPUT_SEL_1: u32 = 0x01;
        }
    }
    #[doc = "QTIMER3 TMR2 input select"]
    pub mod QTIMER3_TRM2_INPUT_SEL {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "input from IOMUX"]
            pub const QTIMER3_TRM2_INPUT_SEL_0: u32 = 0;
            #[doc = "input from XBAR"]
            pub const QTIMER3_TRM2_INPUT_SEL_1: u32 = 0x01;
        }
    }
    #[doc = "QTIMER3 TMR3 input select"]
    pub mod QTIMER3_TRM3_INPUT_SEL {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "input from IOMUX"]
            pub const QTIMER3_TRM3_INPUT_SEL_0: u32 = 0;
            #[doc = "input from XBAR"]
            pub const QTIMER3_TRM3_INPUT_SEL_1: u32 = 0x01;
        }
    }
    #[doc = "QTIMER4 TMR0 input select"]
    pub mod QTIMER4_TRM0_INPUT_SEL {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "input from IOMUX"]
            pub const QTIMER4_TRM0_INPUT_SEL_0: u32 = 0;
            #[doc = "input from XBAR"]
            pub const QTIMER4_TRM0_INPUT_SEL_1: u32 = 0x01;
        }
    }
    #[doc = "QTIMER4 TMR1 input select"]
    pub mod QTIMER4_TRM1_INPUT_SEL {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "input from IOMUX"]
            pub const QTIMER4_TRM1_INPUT_SEL_0: u32 = 0;
            #[doc = "input from XBAR"]
            pub const QTIMER4_TRM1_INPUT_SEL_1: u32 = 0x01;
        }
    }
    #[doc = "QTIMER4 TMR2 input select"]
    pub mod QTIMER4_TRM2_INPUT_SEL {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "input from IOMUX"]
            pub const QTIMER4_TRM2_INPUT_SEL_0: u32 = 0;
            #[doc = "input from XBAR"]
            pub const QTIMER4_TRM2_INPUT_SEL_1: u32 = 0x01;
        }
    }
    #[doc = "QTIMER4 TMR3 input select"]
    pub mod QTIMER4_TRM3_INPUT_SEL {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "input from IOMUX"]
            pub const QTIMER4_TRM3_INPUT_SEL_0: u32 = 0;
            #[doc = "input from XBAR"]
            pub const QTIMER4_TRM3_INPUT_SEL_1: u32 = 0x01;
        }
    }
    #[doc = "IOMUXC XBAR_INOUT4 function direction select"]
    pub mod IOMUXC_XBAR_DIR_SEL_4 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "XBAR_INOUT as input"]
            pub const IOMUXC_XBAR_DIR_SEL_4_0: u32 = 0;
            #[doc = "XBAR_INOUT as output"]
            pub const IOMUXC_XBAR_DIR_SEL_4_1: u32 = 0x01;
        }
    }
    #[doc = "IOMUXC XBAR_INOUT5 function direction select"]
    pub mod IOMUXC_XBAR_DIR_SEL_5 {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "XBAR_INOUT as input"]
            pub const IOMUXC_XBAR_DIR_SEL_5_0: u32 = 0;
            #[doc = "XBAR_INOUT as output"]
            pub const IOMUXC_XBAR_DIR_SEL_5_1: u32 = 0x01;
        }
    }
    #[doc = "IOMUXC XBAR_INOUT6 function direction select"]
    pub mod IOMUXC_XBAR_DIR_SEL_6 {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "XBAR_INOUT as input"]
            pub const IOMUXC_XBAR_DIR_SEL_6_0: u32 = 0;
            #[doc = "XBAR_INOUT as output"]
            pub const IOMUXC_XBAR_DIR_SEL_6_1: u32 = 0x01;
        }
    }
    #[doc = "IOMUXC XBAR_INOUT7 function direction select"]
    pub mod IOMUXC_XBAR_DIR_SEL_7 {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "XBAR_INOUT as input"]
            pub const IOMUXC_XBAR_DIR_SEL_7_0: u32 = 0;
            #[doc = "XBAR_INOUT as output"]
            pub const IOMUXC_XBAR_DIR_SEL_7_1: u32 = 0x01;
        }
    }
    #[doc = "IOMUXC XBAR_INOUT8 function direction select"]
    pub mod IOMUXC_XBAR_DIR_SEL_8 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "XBAR_INOUT as input"]
            pub const IOMUXC_XBAR_DIR_SEL_8_0: u32 = 0;
            #[doc = "XBAR_INOUT as output"]
            pub const IOMUXC_XBAR_DIR_SEL_8_1: u32 = 0x01;
        }
    }
    #[doc = "IOMUXC XBAR_INOUT9 function direction select"]
    pub mod IOMUXC_XBAR_DIR_SEL_9 {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "XBAR_INOUT as input"]
            pub const IOMUXC_XBAR_DIR_SEL_9_0: u32 = 0;
            #[doc = "XBAR_INOUT as output"]
            pub const IOMUXC_XBAR_DIR_SEL_9_1: u32 = 0x01;
        }
    }
    #[doc = "IOMUXC XBAR_INOUT10 function direction select"]
    pub mod IOMUXC_XBAR_DIR_SEL_10 {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "XBAR_INOUT as input"]
            pub const IOMUXC_XBAR_DIR_SEL_10_0: u32 = 0;
            #[doc = "XBAR_INOUT as output"]
            pub const IOMUXC_XBAR_DIR_SEL_10_1: u32 = 0x01;
        }
    }
    #[doc = "IOMUXC XBAR_INOUT11 function direction select"]
    pub mod IOMUXC_XBAR_DIR_SEL_11 {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "XBAR_INOUT as input"]
            pub const IOMUXC_XBAR_DIR_SEL_11_0: u32 = 0;
            #[doc = "XBAR_INOUT as output"]
            pub const IOMUXC_XBAR_DIR_SEL_11_1: u32 = 0x01;
        }
    }
    #[doc = "IOMUXC XBAR_INOUT12 function direction select"]
    pub mod IOMUXC_XBAR_DIR_SEL_12 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "XBAR_INOUT as input"]
            pub const IOMUXC_XBAR_DIR_SEL_12_0: u32 = 0;
            #[doc = "XBAR_INOUT as output"]
            pub const IOMUXC_XBAR_DIR_SEL_12_1: u32 = 0x01;
        }
    }
    #[doc = "IOMUXC XBAR_INOUT13 function direction select"]
    pub mod IOMUXC_XBAR_DIR_SEL_13 {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "XBAR_INOUT as input"]
            pub const IOMUXC_XBAR_DIR_SEL_13_0: u32 = 0;
            #[doc = "XBAR_INOUT as output"]
            pub const IOMUXC_XBAR_DIR_SEL_13_1: u32 = 0x01;
        }
    }
    #[doc = "IOMUXC XBAR_INOUT14 function direction select"]
    pub mod IOMUXC_XBAR_DIR_SEL_14 {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "XBAR_INOUT as input"]
            pub const IOMUXC_XBAR_DIR_SEL_14_0: u32 = 0;
            #[doc = "XBAR_INOUT as output"]
            pub const IOMUXC_XBAR_DIR_SEL_14_1: u32 = 0x01;
        }
    }
    #[doc = "IOMUXC XBAR_INOUT15 function direction select"]
    pub mod IOMUXC_XBAR_DIR_SEL_15 {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "XBAR_INOUT as input"]
            pub const IOMUXC_XBAR_DIR_SEL_15_0: u32 = 0;
            #[doc = "XBAR_INOUT as output"]
            pub const IOMUXC_XBAR_DIR_SEL_15_1: u32 = 0x01;
        }
    }
    #[doc = "IOMUXC XBAR_INOUT16 function direction select"]
    pub mod IOMUXC_XBAR_DIR_SEL_16 {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "XBAR_INOUT as input"]
            pub const IOMUXC_XBAR_DIR_SEL_16_0: u32 = 0;
            #[doc = "XBAR_INOUT as output"]
            pub const IOMUXC_XBAR_DIR_SEL_16_1: u32 = 0x01;
        }
    }
    #[doc = "IOMUXC XBAR_INOUT17 function direction select"]
    pub mod IOMUXC_XBAR_DIR_SEL_17 {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "XBAR_INOUT as input"]
            pub const IOMUXC_XBAR_DIR_SEL_17_0: u32 = 0;
            #[doc = "XBAR_INOUT as output"]
            pub const IOMUXC_XBAR_DIR_SEL_17_1: u32 = 0x01;
        }
    }
    #[doc = "IOMUXC XBAR_INOUT18 function direction select"]
    pub mod IOMUXC_XBAR_DIR_SEL_18 {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "XBAR_INOUT as input"]
            pub const IOMUXC_XBAR_DIR_SEL_18_0: u32 = 0;
            #[doc = "XBAR_INOUT as output"]
            pub const IOMUXC_XBAR_DIR_SEL_18_1: u32 = 0x01;
        }
    }
    #[doc = "IOMUXC XBAR_INOUT19 function direction select"]
    pub mod IOMUXC_XBAR_DIR_SEL_19 {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "XBAR_INOUT as input"]
            pub const IOMUXC_XBAR_DIR_SEL_19_0: u32 = 0;
            #[doc = "XBAR_INOUT as output"]
            pub const IOMUXC_XBAR_DIR_SEL_19_1: u32 = 0x01;
        }
    }
}
#[doc = "GPR7 General Purpose Register"]
pub mod GPR7 {
    #[doc = "LPI2C1 stop request"]
    pub mod LPI2C1_STOP_REQ {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "stop request off"]
            pub const LPI2C1_STOP_REQ_0: u32 = 0;
            #[doc = "stop request on"]
            pub const LPI2C1_STOP_REQ_1: u32 = 0x01;
        }
    }
    #[doc = "LPI2C2 stop request"]
    pub mod LPI2C2_STOP_REQ {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "stop request off"]
            pub const LPI2C2_STOP_REQ_0: u32 = 0;
            #[doc = "stop request on"]
            pub const LPI2C2_STOP_REQ_1: u32 = 0x01;
        }
    }
    #[doc = "LPI2C3 stop request"]
    pub mod LPI2C3_STOP_REQ {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "stop request off"]
            pub const LPI2C3_STOP_REQ_0: u32 = 0;
            #[doc = "stop request on"]
            pub const LPI2C3_STOP_REQ_1: u32 = 0x01;
        }
    }
    #[doc = "LPI2C4 stop request"]
    pub mod LPI2C4_STOP_REQ {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "stop request off"]
            pub const LPI2C4_STOP_REQ_0: u32 = 0;
            #[doc = "stop request on"]
            pub const LPI2C4_STOP_REQ_1: u32 = 0x01;
        }
    }
    #[doc = "LPSPI1 stop request"]
    pub mod LPSPI1_STOP_REQ {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "stop request off"]
            pub const LPSPI1_STOP_REQ_0: u32 = 0;
            #[doc = "stop request on"]
            pub const LPSPI1_STOP_REQ_1: u32 = 0x01;
        }
    }
    #[doc = "LPSPI2 stop request"]
    pub mod LPSPI2_STOP_REQ {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "stop request off"]
            pub const LPSPI2_STOP_REQ_0: u32 = 0;
            #[doc = "stop request on"]
            pub const LPSPI2_STOP_REQ_1: u32 = 0x01;
        }
    }
    #[doc = "LPSPI3 stop request"]
    pub mod LPSPI3_STOP_REQ {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "stop request off"]
            pub const LPSPI3_STOP_REQ_0: u32 = 0;
            #[doc = "stop request on"]
            pub const LPSPI3_STOP_REQ_1: u32 = 0x01;
        }
    }
    #[doc = "LPSPI4 stop request"]
    pub mod LPSPI4_STOP_REQ {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "stop request off"]
            pub const LPSPI4_STOP_REQ_0: u32 = 0;
            #[doc = "stop request on"]
            pub const LPSPI4_STOP_REQ_1: u32 = 0x01;
        }
    }
    #[doc = "LPUART1 stop request"]
    pub mod LPUART1_STOP_REQ {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "stop request off"]
            pub const LPUART1_STOP_REQ_0: u32 = 0;
            #[doc = "stop request on"]
            pub const LPUART1_STOP_REQ_1: u32 = 0x01;
        }
    }
    #[doc = "LPUART1 stop request"]
    pub mod LPUART2_STOP_REQ {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "stop request off"]
            pub const LPUART2_STOP_REQ_0: u32 = 0;
            #[doc = "stop request on"]
            pub const LPUART2_STOP_REQ_1: u32 = 0x01;
        }
    }
    #[doc = "LPUART3 stop request"]
    pub mod LPUART3_STOP_REQ {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "stop request off"]
            pub const LPUART3_STOP_REQ_0: u32 = 0;
            #[doc = "stop request on"]
            pub const LPUART3_STOP_REQ_1: u32 = 0x01;
        }
    }
    #[doc = "LPUART4 stop request"]
    pub mod LPUART4_STOP_REQ {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "stop request off"]
            pub const LPUART4_STOP_REQ_0: u32 = 0;
            #[doc = "stop request on"]
            pub const LPUART4_STOP_REQ_1: u32 = 0x01;
        }
    }
    #[doc = "LPUART5 stop request"]
    pub mod LPUART5_STOP_REQ {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "stop request off"]
            pub const LPUART5_STOP_REQ_0: u32 = 0;
            #[doc = "stop request on"]
            pub const LPUART5_STOP_REQ_1: u32 = 0x01;
        }
    }
    #[doc = "LPUART6 stop request"]
    pub mod LPUART6_STOP_REQ {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "stop request off"]
            pub const LPUART6_STOP_REQ_0: u32 = 0;
            #[doc = "stop request on"]
            pub const LPUART6_STOP_REQ_1: u32 = 0x01;
        }
    }
    #[doc = "LPUART7 stop request"]
    pub mod LPUART7_STOP_REQ {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "stop request off"]
            pub const LPUART7_STOP_REQ_0: u32 = 0;
            #[doc = "stop request on"]
            pub const LPUART7_STOP_REQ_1: u32 = 0x01;
        }
    }
    #[doc = "LPUART8 stop request"]
    pub mod LPUART8_STOP_REQ {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "stop request off"]
            pub const LPUART8_STOP_REQ_0: u32 = 0;
            #[doc = "stop request on"]
            pub const LPUART8_STOP_REQ_1: u32 = 0x01;
        }
    }
    #[doc = "LPI2C1 stop acknowledge"]
    pub mod LPI2C1_STOP_ACK {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "stop acknowledge is not asserted"]
            pub const LPI2C1_STOP_ACK_0: u32 = 0;
            #[doc = "stop acknowledge is asserted (the module is in Stop mode)"]
            pub const LPI2C1_STOP_ACK_1: u32 = 0x01;
        }
    }
    #[doc = "LPI2C2 stop acknowledge"]
    pub mod LPI2C2_STOP_ACK {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "stop acknowledge is not asserted"]
            pub const LPI2C2_STOP_ACK_0: u32 = 0;
            #[doc = "stop acknowledge is asserted"]
            pub const LPI2C2_STOP_ACK_1: u32 = 0x01;
        }
    }
    #[doc = "LPI2C3 stop acknowledge"]
    pub mod LPI2C3_STOP_ACK {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "stop acknowledge is not asserted"]
            pub const LPI2C3_STOP_ACK_0: u32 = 0;
            #[doc = "stop acknowledge is asserted"]
            pub const LPI2C3_STOP_ACK_1: u32 = 0x01;
        }
    }
    #[doc = "LPI2C4 stop acknowledge"]
    pub mod LPI2C4_STOP_ACK {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "stop acknowledge is not asserted"]
            pub const LPI2C4_STOP_ACK_0: u32 = 0;
            #[doc = "stop acknowledge is asserted"]
            pub const LPI2C4_STOP_ACK_1: u32 = 0x01;
        }
    }
    #[doc = "LPSPI1 stop acknowledge"]
    pub mod LPSPI1_STOP_ACK {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "stop acknowledge is not asserted"]
            pub const LPSPI1_STOP_ACK_0: u32 = 0;
            #[doc = "stop acknowledge is asserted"]
            pub const LPSPI1_STOP_ACK_1: u32 = 0x01;
        }
    }
    #[doc = "LPSPI2 stop acknowledge"]
    pub mod LPSPI2_STOP_ACK {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "stop acknowledge is not asserted"]
            pub const LPSPI2_STOP_ACK_0: u32 = 0;
            #[doc = "stop acknowledge is asserted"]
            pub const LPSPI2_STOP_ACK_1: u32 = 0x01;
        }
    }
    #[doc = "LPSPI3 stop acknowledge"]
    pub mod LPSPI3_STOP_ACK {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "stop acknowledge is not asserted"]
            pub const LPSPI3_STOP_ACK_0: u32 = 0;
            #[doc = "stop acknowledge is asserted"]
            pub const LPSPI3_STOP_ACK_1: u32 = 0x01;
        }
    }
    #[doc = "LPSPI4 stop acknowledge"]
    pub mod LPSPI4_STOP_ACK {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "stop acknowledge is not asserted"]
            pub const LPSPI4_STOP_ACK_0: u32 = 0;
            #[doc = "stop acknowledge is asserted"]
            pub const LPSPI4_STOP_ACK_1: u32 = 0x01;
        }
    }
    #[doc = "LPUART1 stop acknowledge"]
    pub mod LPUART1_STOP_ACK {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "stop acknowledge is not asserted"]
            pub const LPUART1_STOP_ACK_0: u32 = 0;
            #[doc = "stop acknowledge is asserted"]
            pub const LPUART1_STOP_ACK_1: u32 = 0x01;
        }
    }
    #[doc = "LPUART1 stop acknowledge"]
    pub mod LPUART2_STOP_ACK {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "stop acknowledge is not asserted"]
            pub const LPUART2_STOP_ACK_0: u32 = 0;
            #[doc = "stop acknowledge is asserted"]
            pub const LPUART2_STOP_ACK_1: u32 = 0x01;
        }
    }
    #[doc = "LPUART3 stop acknowledge"]
    pub mod LPUART3_STOP_ACK {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "stop acknowledge is not asserted"]
            pub const LPUART3_STOP_ACK_0: u32 = 0;
            #[doc = "stop acknowledge is asserted"]
            pub const LPUART3_STOP_ACK_1: u32 = 0x01;
        }
    }
    #[doc = "LPUART4 stop acknowledge"]
    pub mod LPUART4_STOP_ACK {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "stop acknowledge is not asserted"]
            pub const LPUART4_STOP_ACK_0: u32 = 0;
            #[doc = "stop acknowledge is asserted"]
            pub const LPUART4_STOP_ACK_1: u32 = 0x01;
        }
    }
    #[doc = "LPUART5 stop acknowledge"]
    pub mod LPUART5_STOP_ACK {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "stop acknowledge is not asserted"]
            pub const LPUART5_STOP_ACK_0: u32 = 0;
            #[doc = "stop acknowledge is asserted"]
            pub const LPUART5_STOP_ACK_1: u32 = 0x01;
        }
    }
    #[doc = "LPUART6 stop acknowledge"]
    pub mod LPUART6_STOP_ACK {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "stop acknowledge is not asserted"]
            pub const LPUART6_STOP_ACK_0: u32 = 0;
            #[doc = "stop acknowledge is asserted"]
            pub const LPUART6_STOP_ACK_1: u32 = 0x01;
        }
    }
    #[doc = "LPUART7 stop acknowledge"]
    pub mod LPUART7_STOP_ACK {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "stop acknowledge is not asserted"]
            pub const LPUART7_STOP_ACK_0: u32 = 0;
            #[doc = "stop acknowledge is asserted"]
            pub const LPUART7_STOP_ACK_1: u32 = 0x01;
        }
    }
    #[doc = "LPUART8 stop acknowledge"]
    pub mod LPUART8_STOP_ACK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "stop acknowledge is not asserted"]
            pub const LPUART8_STOP_ACK_0: u32 = 0;
            #[doc = "stop acknowledge is asserted (the module is in Stop mode)"]
            pub const LPUART8_STOP_ACK_1: u32 = 0x01;
        }
    }
}
#[doc = "GPR8 General Purpose Register"]
pub mod GPR8 {
    #[doc = "LPI2C1 stop mode selection, cannot change when ipg_stop is asserted."]
    pub mod LPI2C1_IPG_STOP_MODE {
        pub const offset: u32 = 0;
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
    #[doc = "LPI2C1 ipg_doze mode"]
    pub mod LPI2C1_IPG_DOZE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "not in doze mode"]
            pub const LPI2C1_IPG_DOZE_0: u32 = 0;
            #[doc = "in doze mode"]
            pub const LPI2C1_IPG_DOZE_1: u32 = 0x01;
        }
    }
    #[doc = "LPI2C2 stop mode selection, cannot change when ipg_stop is asserted."]
    pub mod LPI2C2_IPG_STOP_MODE {
        pub const offset: u32 = 2;
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
    #[doc = "LPI2C2 ipg_doze mode"]
    pub mod LPI2C2_IPG_DOZE {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "not in doze mode"]
            pub const LPI2C2_IPG_DOZE_0: u32 = 0;
            #[doc = "in doze mode"]
            pub const LPI2C2_IPG_DOZE_1: u32 = 0x01;
        }
    }
    #[doc = "LPI2C3 stop mode selection, cannot change when ipg_stop is asserted."]
    pub mod LPI2C3_IPG_STOP_MODE {
        pub const offset: u32 = 4;
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
    #[doc = "LPI2C3 ipg_doze mode"]
    pub mod LPI2C3_IPG_DOZE {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "not in doze mode"]
            pub const LPI2C3_IPG_DOZE_0: u32 = 0;
            #[doc = "in doze mode"]
            pub const LPI2C3_IPG_DOZE_1: u32 = 0x01;
        }
    }
    #[doc = "LPI2C4 stop mode selection, cannot change when ipg_stop is asserted."]
    pub mod LPI2C4_IPG_STOP_MODE {
        pub const offset: u32 = 6;
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
    #[doc = "LPI2C4 ipg_doze mode"]
    pub mod LPI2C4_IPG_DOZE {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "not in doze mode"]
            pub const LPI2C4_IPG_DOZE_0: u32 = 0;
            #[doc = "in doze mode"]
            pub const LPI2C4_IPG_DOZE_1: u32 = 0x01;
        }
    }
    #[doc = "LPSPI1 stop mode selection, cannot change when ipg_stop is asserted."]
    pub mod LPSPI1_IPG_STOP_MODE {
        pub const offset: u32 = 8;
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
    #[doc = "LPSPI1 ipg_doze mode"]
    pub mod LPSPI1_IPG_DOZE {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "not in doze mode"]
            pub const LPSPI1_IPG_DOZE_0: u32 = 0;
            #[doc = "in doze mode"]
            pub const LPSPI1_IPG_DOZE_1: u32 = 0x01;
        }
    }
    #[doc = "LPSPI2 stop mode selection, cannot change when ipg_stop is asserted."]
    pub mod LPSPI2_IPG_STOP_MODE {
        pub const offset: u32 = 10;
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
    #[doc = "LPSPI2 ipg_doze mode"]
    pub mod LPSPI2_IPG_DOZE {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "not in doze mode"]
            pub const LPSPI2_IPG_DOZE_0: u32 = 0;
            #[doc = "in doze mode"]
            pub const LPSPI2_IPG_DOZE_1: u32 = 0x01;
        }
    }
    #[doc = "LPSPI3 stop mode selection, cannot change when ipg_stop is asserted."]
    pub mod LPSPI3_IPG_STOP_MODE {
        pub const offset: u32 = 12;
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
    #[doc = "LPSPI3 ipg_doze mode"]
    pub mod LPSPI3_IPG_DOZE {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "not in doze mode"]
            pub const LPSPI3_IPG_DOZE_0: u32 = 0;
            #[doc = "in doze mode"]
            pub const LPSPI3_IPG_DOZE_1: u32 = 0x01;
        }
    }
    #[doc = "LPSPI4 stop mode selection, cannot change when ipg_stop is asserted."]
    pub mod LPSPI4_IPG_STOP_MODE {
        pub const offset: u32 = 14;
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
    #[doc = "LPSPI4 ipg_doze mode"]
    pub mod LPSPI4_IPG_DOZE {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "not in doze mode"]
            pub const LPSPI4_IPG_DOZE_0: u32 = 0;
            #[doc = "in doze mode"]
            pub const LPSPI4_IPG_DOZE_1: u32 = 0x01;
        }
    }
    #[doc = "LPUART1 stop mode selection, cannot change when ipg_stop is asserted."]
    pub mod LPUART1_IPG_STOP_MODE {
        pub const offset: u32 = 16;
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
    #[doc = "LPUART1 ipg_doze mode"]
    pub mod LPUART1_IPG_DOZE {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "not in doze mode"]
            pub const LPUART1_IPG_DOZE_0: u32 = 0;
            #[doc = "in doze mode"]
            pub const LPUART1_IPG_DOZE_1: u32 = 0x01;
        }
    }
    #[doc = "LPUART2 stop mode selection, cannot change when ipg_stop is asserted."]
    pub mod LPUART2_IPG_STOP_MODE {
        pub const offset: u32 = 18;
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
    #[doc = "LPUART2 ipg_doze mode"]
    pub mod LPUART2_IPG_DOZE {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "not in doze mode"]
            pub const LPUART2_IPG_DOZE_0: u32 = 0;
            #[doc = "in doze mode"]
            pub const LPUART2_IPG_DOZE_1: u32 = 0x01;
        }
    }
    #[doc = "LPUART3 stop mode selection, cannot change when ipg_stop is asserted."]
    pub mod LPUART3_IPG_STOP_MODE {
        pub const offset: u32 = 20;
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
    #[doc = "LPUART3 ipg_doze mode"]
    pub mod LPUART3_IPG_DOZE {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "not in doze mode"]
            pub const LPUART3_IPG_DOZE_0: u32 = 0;
            #[doc = "in doze mode"]
            pub const LPUART3_IPG_DOZE_1: u32 = 0x01;
        }
    }
    #[doc = "LPUART4 stop mode selection, cannot change when ipg_stop is asserted."]
    pub mod LPUART4_IPG_STOP_MODE {
        pub const offset: u32 = 22;
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
    #[doc = "LPUART4 ipg_doze mode"]
    pub mod LPUART4_IPG_DOZE {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "not in doze mode"]
            pub const LPUART4_IPG_DOZE_0: u32 = 0;
            #[doc = "in doze mode"]
            pub const LPUART4_IPG_DOZE_1: u32 = 0x01;
        }
    }
    #[doc = "LPUART5 stop mode selection, cannot change when ipg_stop is asserted."]
    pub mod LPUART5_IPG_STOP_MODE {
        pub const offset: u32 = 24;
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
    #[doc = "LPUART5 ipg_doze mode"]
    pub mod LPUART5_IPG_DOZE {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "not in doze mode"]
            pub const LPUART5_IPG_DOZE_0: u32 = 0;
            #[doc = "in doze mode"]
            pub const LPUART5_IPG_DOZE_1: u32 = 0x01;
        }
    }
    #[doc = "LPUART6 stop mode selection, cannot change when ipg_stop is asserted."]
    pub mod LPUART6_IPG_STOP_MODE {
        pub const offset: u32 = 26;
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
    #[doc = "LPUART6 ipg_doze mode"]
    pub mod LPUART6_IPG_DOZE {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "not in doze mode"]
            pub const LPUART6_IPG_DOZE_0: u32 = 0;
            #[doc = "in doze mode"]
            pub const LPUART6_IPG_DOZE_1: u32 = 0x01;
        }
    }
    #[doc = "LPUART7 stop mode selection, cannot change when ipg_stop is asserted."]
    pub mod LPUART7_IPG_STOP_MODE {
        pub const offset: u32 = 28;
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
    #[doc = "LPUART7 ipg_doze mode"]
    pub mod LPUART7_IPG_DOZE {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "not in doze mode"]
            pub const LPUART7_IPG_DOZE_0: u32 = 0;
            #[doc = "in doze mode"]
            pub const LPUART7_IPG_DOZE_1: u32 = 0x01;
        }
    }
    #[doc = "LPUART8 stop mode selection, cannot change when ipg_stop is asserted."]
    pub mod LPUART8_IPG_STOP_MODE {
        pub const offset: u32 = 30;
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
    #[doc = "LPUART8 ipg_doze mode"]
    pub mod LPUART8_IPG_DOZE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "not in doze mode"]
            pub const LPUART8_IPG_DOZE_0: u32 = 0;
            #[doc = "in doze mode"]
            pub const LPUART8_IPG_DOZE_1: u32 = 0x01;
        }
    }
}
#[doc = "GPR10 General Purpose Register"]
pub mod GPR10 {
    #[doc = "ARM non-secure (non-invasive) debug enable"]
    pub mod NIDEN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Debug turned off."]
            pub const NIDEN_0: u32 = 0;
            #[doc = "Debug enabled (default)."]
            pub const NIDEN_1: u32 = 0x01;
        }
    }
    #[doc = "ARM invasive debug enable"]
    pub mod DBG_EN {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Debug turned off."]
            pub const DBG_EN_0: u32 = 0;
            #[doc = "Debug enabled (default)."]
            pub const DBG_EN_1: u32 = 0x01;
        }
    }
    #[doc = "Security error response enable for all security gaskets (on both AHB and AXI buses)"]
    pub mod SEC_ERR_RESP {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "OKEY response"]
            pub const SEC_ERR_RESP_0: u32 = 0;
            #[doc = "SLVError (default)"]
            pub const SEC_ERR_RESP_1: u32 = 0x01;
        }
    }
    #[doc = "DCP Key selection bit."]
    pub mod DCPKEY_OCOTP_OR_KEYMUX {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Select key from SNVS Master Key."]
            pub const DCPKEY_OCOTP_OR_KEYMUX_0: u32 = 0;
            #[doc = "Select key from OCOTP (SW_GP2)."]
            pub const DCPKEY_OCOTP_OR_KEYMUX_1: u32 = 0x01;
        }
    }
    #[doc = "OCRAM TrustZone (TZ) enable."]
    pub mod OCRAM_TZ_EN {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The TrustZone feature is disabled. Entire OCRAM space is available for all access types (secure/non-secure/user/supervisor)."]
            pub const OCRAM_TZ_EN_0: u32 = 0;
            #[doc = "The TrustZone feature is enabled. Access to address in the range specified by \\[ENDADDR:STARTADDR\\] follows the execution mode access policy described in CSU chapter."]
            pub const OCRAM_TZ_EN_1: u32 = 0x01;
        }
    }
    #[doc = "OCRAM TrustZone (TZ) start address"]
    pub mod OCRAM_TZ_ADDR {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x7f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Lock NIDEN field for changes"]
    pub mod LOCK_NIDEN {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Field is not locked"]
            pub const LOCK_NIDEN_0: u32 = 0;
            #[doc = "Field is locked (read access only)"]
            pub const LOCK_NIDEN_1: u32 = 0x01;
        }
    }
    #[doc = "Lock DBG_EN field for changes"]
    pub mod LOCK_DBG_EN {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Field is not locked"]
            pub const LOCK_DBG_EN_0: u32 = 0;
            #[doc = "Field is locked (read access only)"]
            pub const LOCK_DBG_EN_1: u32 = 0x01;
        }
    }
    #[doc = "Lock SEC_ERR_RESP field for changes"]
    pub mod LOCK_SEC_ERR_RESP {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Field is not locked"]
            pub const LOCK_SEC_ERR_RESP_0: u32 = 0;
            #[doc = "Field is locked (read access only)"]
            pub const LOCK_SEC_ERR_RESP_1: u32 = 0x01;
        }
    }
    #[doc = "Lock DCP Key OCOTP/Key MUX selection bit"]
    pub mod LOCK_DCPKEY_OCOTP_OR_KEYMUX {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Field is not locked"]
            pub const LOCK_DCPKEY_OCOTP_OR_KEYMUX_0: u32 = 0;
            #[doc = "Field is locked (read access only)"]
            pub const LOCK_DCPKEY_OCOTP_OR_KEYMUX_1: u32 = 0x01;
        }
    }
    #[doc = "Lock OCRAM_TZ_EN field for changes"]
    pub mod LOCK_OCRAM_TZ_EN {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Field is not locked"]
            pub const LOCK_OCRAM_TZ_EN_0: u32 = 0;
            #[doc = "Field is locked (read access only)"]
            pub const LOCK_OCRAM_TZ_EN_1: u32 = 0x01;
        }
    }
    #[doc = "Lock OCRAM_TZ_ADDR field for changes"]
    pub mod LOCK_OCRAM_TZ_ADDR {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x7f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Field is not locked"]
            pub const LOCK_OCRAM_TZ_ADDR_0: u32 = 0;
            #[doc = "Field is locked (read access only)"]
            pub const LOCK_OCRAM_TZ_ADDR_1: u32 = 0x01;
        }
    }
}
#[doc = "GPR11 General Purpose Register"]
pub mod GPR11 {
    #[doc = "Access control of memory region-0"]
    pub mod M7_APC_AC_R0_CTRL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No access protection"]
            pub const M7_APC_AC_R0_CTRL_0: u32 = 0;
            #[doc = "M7 debug protection enabled"]
            pub const M7_APC_AC_R0_CTRL_1: u32 = 0x01;
            #[doc = "FlexSPI access protection"]
            pub const M7_APC_AC_R0_CTRL_2: u32 = 0x02;
            #[doc = "Both M7 debug and FlexSPI access are protected"]
            pub const M7_APC_AC_R0_CTRL_3: u32 = 0x03;
        }
    }
    #[doc = "Access control of memory region-1"]
    pub mod M7_APC_AC_R1_CTRL {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No access protection"]
            pub const M7_APC_AC_R1_CTRL_0: u32 = 0;
            #[doc = "M7 debug protection enabled"]
            pub const M7_APC_AC_R1_CTRL_1: u32 = 0x01;
            #[doc = "FlexSPI access protection"]
            pub const M7_APC_AC_R1_CTRL_2: u32 = 0x02;
            #[doc = "Both M7 debug and FlexSPI access are protected"]
            pub const M7_APC_AC_R1_CTRL_3: u32 = 0x03;
        }
    }
    #[doc = "Access control of memory region-2"]
    pub mod M7_APC_AC_R2_CTRL {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No access protection"]
            pub const M7_APC_AC_R2_CTRL_0: u32 = 0;
            #[doc = "M7 debug protection enabled"]
            pub const M7_APC_AC_R2_CTRL_1: u32 = 0x01;
            #[doc = "FlexSPI access protection"]
            pub const M7_APC_AC_R2_CTRL_2: u32 = 0x02;
            #[doc = "Both M7 debug and FlexSPI access are protected"]
            pub const M7_APC_AC_R2_CTRL_3: u32 = 0x03;
        }
    }
    #[doc = "Access control of memory region-3"]
    pub mod M7_APC_AC_R3_CTRL {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No access protection"]
            pub const M7_APC_AC_R3_CTRL_0: u32 = 0;
            #[doc = "M7 debug protection enabled"]
            pub const M7_APC_AC_R3_CTRL_1: u32 = 0x01;
            #[doc = "FlexSPI access protection"]
            pub const M7_APC_AC_R3_CTRL_2: u32 = 0x02;
            #[doc = "Both M7 debug and FlexSPI access are protected"]
            pub const M7_APC_AC_R3_CTRL_3: u32 = 0x03;
        }
    }
    #[doc = "BEE data decryption of memory region-n (n = 3 to 0)"]
    pub mod BEE_DE_RX_EN {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPR12 General Purpose Register"]
pub mod GPR12 {
    #[doc = "FlexIO1 stop mode selection. Cannot change when ipg_stop is asserted."]
    pub mod FLEXIO1_IPG_STOP_MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "FlexIO1 is functional in Stop mode."]
            pub const FLEXIO1_IPG_STOP_MODE_0: u32 = 0;
            #[doc = "When this bit is equal to 1'b1 and ipg_stop is asserted, FlexIO1 is not functional in Stop mode."]
            pub const FLEXIO1_IPG_STOP_MODE_1: u32 = 0x01;
        }
    }
    #[doc = "FLEXIO1 ipg_doze mode"]
    pub mod FLEXIO1_IPG_DOZE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "FLEXIO1 is not in doze mode"]
            pub const FLEXIO1_IPG_DOZE_0: u32 = 0;
            #[doc = "FLEXIO1 is in doze mode"]
            pub const FLEXIO1_IPG_DOZE_1: u32 = 0x01;
        }
    }
    #[doc = "FlexIO2 stop mode selection. Cannot change when ipg_stop is asserted."]
    pub mod FLEXIO2_IPG_STOP_MODE {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "FlexIO2 is functional in Stop mode."]
            pub const FLEXIO2_IPG_STOP_MODE_0: u32 = 0;
            #[doc = "When this bit is equal to 1'b1 and ipg_stop is asserted, FlexIO2 is not functional in Stop mode."]
            pub const FLEXIO2_IPG_STOP_MODE_1: u32 = 0x01;
        }
    }
    #[doc = "FLEXIO2 ipg_doze mode"]
    pub mod FLEXIO2_IPG_DOZE {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "FLEXIO2 is not in doze mode"]
            pub const FLEXIO2_IPG_DOZE_0: u32 = 0;
            #[doc = "FLEXIO2 is in doze mode"]
            pub const FLEXIO2_IPG_DOZE_1: u32 = 0x01;
        }
    }
    #[doc = "ACMP stop mode selection. Cannot change when ipg_stop is asserted."]
    pub mod ACMP_IPG_STOP_MODE {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ACMP is functional in Stop mode."]
            pub const ACMP_IPG_STOP_MODE_0: u32 = 0;
            #[doc = "When this bit is equal to 1'b1 and ipg_stop is asserted, ACMP is not functional in Stop mode."]
            pub const ACMP_IPG_STOP_MODE_1: u32 = 0x01;
        }
    }
    #[doc = "FlexIO3 stop mode selection. Cannot change when ipg_stop is asserted."]
    pub mod FLEXIO3_IPG_STOP_MODE {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "FlexIO3 is functional in Stop mode."]
            pub const FLEXIO3_IPG_STOP_MODE_0: u32 = 0;
            #[doc = "When this bit is equal to 1'b1 and ipg_stop is asserted, FlexIO3 is not functional in Stop mode."]
            pub const FLEXIO3_IPG_STOP_MODE_1: u32 = 0x01;
        }
    }
    #[doc = "FLEXIO3 ipg_doze mode"]
    pub mod FLEXIO3_IPG_DOZE {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "FLEXIO3 is not in doze mode"]
            pub const FLEXIO3_IPG_DOZE_0: u32 = 0;
            #[doc = "FLEXIO3 is in doze mode"]
            pub const FLEXIO3_IPG_DOZE_1: u32 = 0x01;
        }
    }
}
#[doc = "GPR13 General Purpose Register"]
pub mod GPR13 {
    #[doc = "uSDHC block cacheable attribute value of AXI read transactions"]
    pub mod ARCACHE_USDHC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Cacheable attribute is off for read transactions."]
            pub const ARCACHE_USDHC_0: u32 = 0;
            #[doc = "Cacheable attribute is on for read transactions."]
            pub const ARCACHE_USDHC_1: u32 = 0x01;
        }
    }
    #[doc = "uSDHC block cacheable attribute value of AXI write transactions"]
    pub mod AWCACHE_USDHC {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Cacheable attribute is off for write transactions."]
            pub const AWCACHE_USDHC_0: u32 = 0;
            #[doc = "Cacheable attribute is on for write transactions."]
            pub const AWCACHE_USDHC_1: u32 = 0x01;
        }
    }
    #[doc = "CANFD stop request."]
    pub mod CANFD_STOP_REQ {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "stop request off"]
            pub const CANFD_STOP_REQ_0: u32 = 0;
            #[doc = "stop request on"]
            pub const CANFD_STOP_REQ_1: u32 = 0x01;
        }
    }
    #[doc = "ENET block cacheable attribute value of AXI transactions"]
    pub mod CACHE_ENET {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Cacheable attribute is off for read/write transactions."]
            pub const CACHE_ENET_0: u32 = 0;
            #[doc = "Cacheable attribute is on for read/write transactions."]
            pub const CACHE_ENET_1: u32 = 0x01;
        }
    }
    #[doc = "USB block cacheable attribute value of AXI transactions"]
    pub mod CACHE_USB {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Cacheable attribute is off for read/write transactions."]
            pub const CACHE_USB_0: u32 = 0;
            #[doc = "Cacheable attribute is on for read/write transactions."]
            pub const CACHE_USB_1: u32 = 0x01;
        }
    }
    #[doc = "CANFD stop acknowledge."]
    pub mod CANFD_STOP_ACK {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "CANFD stop acknowledge is not asserted"]
            pub const CANFD_STOP_ACK_0: u32 = 0;
            #[doc = "CANFD stop acknowledge is asserted"]
            pub const CANFD_STOP_ACK_1: u32 = 0x01;
        }
    }
}
#[doc = "GPR14 General Purpose Register"]
pub mod GPR14 {
    #[doc = "reduces ACMP1 internal bias current by 30%"]
    pub mod ACMP1_CMP_IGEN_TRIM_DN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "no reduce"]
            pub const ACMP1_CMP_IGEN_TRIM_DN_0: u32 = 0;
            #[doc = "reduces"]
            pub const ACMP1_CMP_IGEN_TRIM_DN_1: u32 = 0x01;
        }
    }
    #[doc = "reduces ACMP2 internal bias current by 30%"]
    pub mod ACMP2_CMP_IGEN_TRIM_DN {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "no reduce"]
            pub const ACMP2_CMP_IGEN_TRIM_DN_0: u32 = 0;
            #[doc = "reduces"]
            pub const ACMP2_CMP_IGEN_TRIM_DN_1: u32 = 0x01;
        }
    }
    #[doc = "reduces ACMP3 internal bias current by 30%"]
    pub mod ACMP3_CMP_IGEN_TRIM_DN {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "no reduce"]
            pub const ACMP3_CMP_IGEN_TRIM_DN_0: u32 = 0;
            #[doc = "reduces"]
            pub const ACMP3_CMP_IGEN_TRIM_DN_1: u32 = 0x01;
        }
    }
    #[doc = "reduces ACMP4 internal bias current by 30%"]
    pub mod ACMP4_CMP_IGEN_TRIM_DN {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "no reduce"]
            pub const ACMP4_CMP_IGEN_TRIM_DN_0: u32 = 0;
            #[doc = "reduces"]
            pub const ACMP4_CMP_IGEN_TRIM_DN_1: u32 = 0x01;
        }
    }
    #[doc = "increases ACMP1 internal bias current by 30%"]
    pub mod ACMP1_CMP_IGEN_TRIM_UP {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "no increase"]
            pub const ACMP1_CMP_IGEN_TRIM_UP_0: u32 = 0;
            #[doc = "increases"]
            pub const ACMP1_CMP_IGEN_TRIM_UP_1: u32 = 0x01;
        }
    }
    #[doc = "increases ACMP2 internal bias current by 30%"]
    pub mod ACMP2_CMP_IGEN_TRIM_UP {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "no increase"]
            pub const ACMP2_CMP_IGEN_TRIM_UP_0: u32 = 0;
            #[doc = "increases"]
            pub const ACMP2_CMP_IGEN_TRIM_UP_1: u32 = 0x01;
        }
    }
    #[doc = "increases ACMP3 internal bias current by 30%"]
    pub mod ACMP3_CMP_IGEN_TRIM_UP {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "no increase"]
            pub const ACMP3_CMP_IGEN_TRIM_UP_0: u32 = 0;
            #[doc = "increases"]
            pub const ACMP3_CMP_IGEN_TRIM_UP_1: u32 = 0x01;
        }
    }
    #[doc = "increases ACMP4 internal bias current by 30%"]
    pub mod ACMP4_CMP_IGEN_TRIM_UP {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "no increase"]
            pub const ACMP4_CMP_IGEN_TRIM_UP_0: u32 = 0;
            #[doc = "increases"]
            pub const ACMP4_CMP_IGEN_TRIM_UP_1: u32 = 0x01;
        }
    }
    #[doc = "ACMP1 sample_lv source select"]
    pub mod ACMP1_SAMPLE_SYNC_EN {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "select XBAR output"]
            pub const ACMP1_SAMPLE_SYNC_EN_0: u32 = 0;
            #[doc = "select synced sample_lv"]
            pub const ACMP1_SAMPLE_SYNC_EN_1: u32 = 0x01;
        }
    }
    #[doc = "ACMP2 sample_lv source select"]
    pub mod ACMP2_SAMPLE_SYNC_EN {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "select XBAR output"]
            pub const ACMP2_SAMPLE_SYNC_EN_0: u32 = 0;
            #[doc = "select synced sample_lv"]
            pub const ACMP2_SAMPLE_SYNC_EN_1: u32 = 0x01;
        }
    }
    #[doc = "ACMP3 sample_lv source select"]
    pub mod ACMP3_SAMPLE_SYNC_EN {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "select XBAR output"]
            pub const ACMP3_SAMPLE_SYNC_EN_0: u32 = 0;
            #[doc = "select synced sample_lv"]
            pub const ACMP3_SAMPLE_SYNC_EN_1: u32 = 0x01;
        }
    }
    #[doc = "ACMP4 sample_lv source select"]
    pub mod ACMP4_SAMPLE_SYNC_EN {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "select XBAR output"]
            pub const ACMP4_SAMPLE_SYNC_EN_0: u32 = 0;
            #[doc = "select synced sample_lv"]
            pub const ACMP4_SAMPLE_SYNC_EN_1: u32 = 0x01;
        }
    }
    #[doc = "ITCM total size configuration"]
    pub mod CM7_CFGITCMSZ {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "0 KB (No ITCM)"]
            pub const CM7_CFGITCMSZ_0: u32 = 0;
            #[doc = "4 KB"]
            pub const CM7_CFGITCMSZ_3: u32 = 0x03;
            #[doc = "8 KB"]
            pub const CM7_CFGITCMSZ_4: u32 = 0x04;
            #[doc = "16 KB"]
            pub const CM7_CFGITCMSZ_5: u32 = 0x05;
            #[doc = "32 KB"]
            pub const CM7_CFGITCMSZ_6: u32 = 0x06;
            #[doc = "64 KB"]
            pub const CM7_CFGITCMSZ_7: u32 = 0x07;
            #[doc = "128 KB"]
            pub const CM7_CFGITCMSZ_8: u32 = 0x08;
            #[doc = "256 KB"]
            pub const CM7_CFGITCMSZ_9: u32 = 0x09;
            #[doc = "512 KB"]
            pub const CM7_CFGITCMSZ_10: u32 = 0x0a;
        }
    }
    #[doc = "DTCM total size configuration"]
    pub mod CM7_CFGDTCMSZ {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "0 KB (No DTCM)"]
            pub const CM7_CFGDTCMSZ_0: u32 = 0;
            #[doc = "4 KB"]
            pub const CM7_CFGDTCMSZ_3: u32 = 0x03;
            #[doc = "8 KB"]
            pub const CM7_CFGDTCMSZ_4: u32 = 0x04;
            #[doc = "16 KB"]
            pub const CM7_CFGDTCMSZ_5: u32 = 0x05;
            #[doc = "32 KB"]
            pub const CM7_CFGDTCMSZ_6: u32 = 0x06;
            #[doc = "64 KB"]
            pub const CM7_CFGDTCMSZ_7: u32 = 0x07;
            #[doc = "128 KB"]
            pub const CM7_CFGDTCMSZ_8: u32 = 0x08;
            #[doc = "256 KB"]
            pub const CM7_CFGDTCMSZ_9: u32 = 0x09;
            #[doc = "512 KB"]
            pub const CM7_CFGDTCMSZ_10: u32 = 0x0a;
        }
    }
}
#[doc = "GPR16 General Purpose Register"]
pub mod GPR16 {
    #[doc = "ITCM enable initialization out of reset"]
    pub mod INIT_ITCM_EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ITCM is disabled"]
            pub const INIT_ITCM_EN_0: u32 = 0;
            #[doc = "ITCM is enabled"]
            pub const INIT_ITCM_EN_1: u32 = 0x01;
        }
    }
    #[doc = "DTCM enable initialization out of reset"]
    pub mod INIT_DTCM_EN {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "DTCM is disabled"]
            pub const INIT_DTCM_EN_0: u32 = 0;
            #[doc = "DTCM is enabled"]
            pub const INIT_DTCM_EN_1: u32 = 0x01;
        }
    }
    #[doc = "FlexRAM bank config source select"]
    pub mod FLEXRAM_BANK_CFG_SEL {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "use fuse value to config"]
            pub const FLEXRAM_BANK_CFG_SEL_0: u32 = 0;
            #[doc = "use FLEXRAM_BANK_CFG to config"]
            pub const FLEXRAM_BANK_CFG_SEL_1: u32 = 0x01;
        }
    }
}
#[doc = "GPR17 General Purpose Register"]
pub mod GPR17 {
    #[doc = "FlexRAM bank config value"]
    pub mod FLEXRAM_BANK_CFG {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPR18 General Purpose Register"]
pub mod GPR18 {
    #[doc = "lock M7_APC_AC_R0_BOT field for changes"]
    pub mod LOCK_M7_APC_AC_R0_BOT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Register field \\[31:1\\] is not locked"]
            pub const LOCK_M7_APC_AC_R0_BOT_0: u32 = 0;
            #[doc = "Register field \\[31:1\\] is locked (read access only)"]
            pub const LOCK_M7_APC_AC_R0_BOT_1: u32 = 0x01;
        }
    }
    #[doc = "APC end address of memory region-0"]
    pub mod M7_APC_AC_R0_BOT {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x1fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPR19 General Purpose Register"]
pub mod GPR19 {
    #[doc = "lock M7_APC_AC_R0_TOP field for changes"]
    pub mod LOCK_M7_APC_AC_R0_TOP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Register field \\[31:1\\] is not locked"]
            pub const LOCK_M7_APC_AC_R0_TOP_0: u32 = 0;
            #[doc = "Register field \\[31:1\\] is locked (read access only)"]
            pub const LOCK_M7_APC_AC_R0_TOP_1: u32 = 0x01;
        }
    }
    #[doc = "APC start address of memory region-0"]
    pub mod M7_APC_AC_R0_TOP {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x1fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPR20 General Purpose Register"]
pub mod GPR20 {
    #[doc = "lock M7_APC_AC_R1_BOT field for changes"]
    pub mod LOCK_M7_APC_AC_R1_BOT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Register field \\[31:1\\] is not locked"]
            pub const LOCK_M7_APC_AC_R1_BOT_0: u32 = 0;
            #[doc = "Register field \\[31:1\\] is locked (read access only)"]
            pub const LOCK_M7_APC_AC_R1_BOT_1: u32 = 0x01;
        }
    }
    #[doc = "APC end address of memory region-1"]
    pub mod M7_APC_AC_R1_BOT {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x1fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPR21 General Purpose Register"]
pub mod GPR21 {
    #[doc = "lock M7_APC_AC_R1_TOP field for changes"]
    pub mod LOCK_M7_APC_AC_R1_TOP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Register field \\[31:1\\] is not locked"]
            pub const LOCK_M7_APC_AC_R1_TOP_0: u32 = 0;
            #[doc = "Register field \\[31:1\\] is locked (read access only)"]
            pub const LOCK_M7_APC_AC_R1_TOP_1: u32 = 0x01;
        }
    }
    #[doc = "APC start address of memory region-1"]
    pub mod M7_APC_AC_R1_TOP {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x1fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPR22 General Purpose Register"]
pub mod GPR22 {
    #[doc = "lock M7_APC_AC_R2_BOT field for changes"]
    pub mod LOCK_M7_APC_AC_R2_BOT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Register field \\[31:1\\] is not locked"]
            pub const LOCK_M7_APC_AC_R2_BOT_0: u32 = 0;
            #[doc = "Register field \\[31:1\\] is locked (read access only)"]
            pub const LOCK_M7_APC_AC_R2_BOT_1: u32 = 0x01;
        }
    }
    #[doc = "APC end address of memory region-2"]
    pub mod M7_APC_AC_R2_BOT {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x1fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPR23 General Purpose Register"]
pub mod GPR23 {
    #[doc = "lock M7_APC_AC_R2_TOP field for changes"]
    pub mod LOCK_M7_APC_AC_R2_TOP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Register field \\[31:1\\] is not locked"]
            pub const LOCK_M7_APC_AC_R2_TOP_0: u32 = 0;
            #[doc = "Register field \\[31:1\\] is locked (read access only)"]
            pub const LOCK_M7_APC_AC_R2_TOP_1: u32 = 0x01;
        }
    }
    #[doc = "APC start address of memory region-2"]
    pub mod M7_APC_AC_R2_TOP {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x1fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPR24 General Purpose Register"]
pub mod GPR24 {
    #[doc = "lock M7_APC_AC_R3_BOT field for changes"]
    pub mod LOCK_M7_APC_AC_R3_BOT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Register field \\[31:1\\] is not locked"]
            pub const LOCK_M7_APC_AC_R3_BOT_0: u32 = 0;
            #[doc = "Register field \\[31:1\\] is locked (read access only)"]
            pub const LOCK_M7_APC_AC_R3_BOT_1: u32 = 0x01;
        }
    }
    #[doc = "APC end address of memory region-3"]
    pub mod M7_APC_AC_R3_BOT {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x1fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPR25 General Purpose Register"]
pub mod GPR25 {
    #[doc = "lock M7_APC_AC_R3_TOP field for changes"]
    pub mod LOCK_M7_APC_AC_R3_TOP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Register field \\[31:1\\] is not locked"]
            pub const LOCK_M7_APC_AC_R3_TOP_0: u32 = 0;
            #[doc = "Register field \\[31:1\\] is locked (read access only)"]
            pub const LOCK_M7_APC_AC_R3_TOP_1: u32 = 0x01;
        }
    }
    #[doc = "APC start address of memory region-3"]
    pub mod M7_APC_AC_R3_TOP {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x1fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPR26 General Purpose Register"]
pub mod GPR26 {
    #[doc = "GPIO1 and GPIO6 share same IO MUX function, GPIO_MUX1 selects one GPIO function."]
    pub mod GPIO_MUX1_GPIO_SEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPR27 General Purpose Register"]
pub mod GPR27 {
    #[doc = "GPIO2 and GPIO7 share same IO MUX function, GPIO_MUX2 selects one GPIO function."]
    pub mod GPIO_MUX2_GPIO_SEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPR28 General Purpose Register"]
pub mod GPR28 {
    #[doc = "GPIO3 and GPIO8 share same IO MUX function, GPIO_MUX3 selects one GPIO function."]
    pub mod GPIO_MUX3_GPIO_SEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPR29 General Purpose Register"]
pub mod GPR29 {
    #[doc = "GPIO4 and GPIO9 share same IO MUX function, GPIO_MUX4 selects one GPIO function."]
    pub mod GPIO_MUX4_GPIO_SEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPR30 General Purpose Register"]
pub mod GPR30 {
    #[doc = "Start address of flexspi1 and flexspi2"]
    pub mod FLEXSPI_REMAP_ADDR_START {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x000f_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPR31 General Purpose Register"]
pub mod GPR31 {
    #[doc = "End address of flexspi1 and flexspi2"]
    pub mod FLEXSPI_REMAP_ADDR_END {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x000f_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPR32 General Purpose Register"]
pub mod GPR32 {
    #[doc = "Offset address of flexspi1 and flexspi2"]
    pub mod FLEXSPI_REMAP_ADDR_OFFSET {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x000f_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPR33 General Purpose Register"]
pub mod GPR33 {
    #[doc = "OCRAM2 TrustZone (TZ) enable."]
    pub mod OCRAM2_TZ_EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The TrustZone feature is disabled. Entire OCRAM2 space is available for all access types (secure/non-secure/user/supervisor)."]
            pub const OCRAM2_TZ_EN_0: u32 = 0;
            #[doc = "The TrustZone feature is enabled. Access to address in the range specified by \\[ENDADDR:STARTADDR\\] follows the execution mode access policy described in CSU chapter."]
            pub const OCRAM2_TZ_EN_1: u32 = 0x01;
        }
    }
    #[doc = "OCRAM2 TrustZone (TZ) start address"]
    pub mod OCRAM2_TZ_ADDR {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x7f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Lock OCRAM2_TZ_EN field for changes"]
    pub mod LOCK_OCRAM2_TZ_EN {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Field is not locked"]
            pub const LOCK_OCRAM2_TZ_EN_0: u32 = 0;
            #[doc = "Field is locked (read access only)"]
            pub const LOCK_OCRAM2_TZ_EN_1: u32 = 0x01;
        }
    }
    #[doc = "Lock OCRAM2_TZ_ADDR field for changes"]
    pub mod LOCK_OCRAM2_TZ_ADDR {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x7f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Field is not locked"]
            pub const LOCK_OCRAM2_TZ_ADDR_0: u32 = 0;
            #[doc = "Field is locked (read access only)"]
            pub const LOCK_OCRAM2_TZ_ADDR_1: u32 = 0x01;
        }
    }
}
#[doc = "GPR34 General Purpose Register"]
pub mod GPR34 {
    #[doc = "Boot Pin select in SIP_TEST_MUX"]
    pub mod SIP_TEST_MUX_BOOT_PIN_SEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enable SIP_TEST_MUX"]
    pub mod SIP_TEST_MUX_QSPI_SIP_EN {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "SIP_TEST_MUX is disabled"]
            pub const SIP_TEST_MUX_QSPI_SIP_EN_0: u32 = 0;
            #[doc = "SIP_TEST_MUX is enabled"]
            pub const SIP_TEST_MUX_QSPI_SIP_EN_1: u32 = 0x01;
        }
    }
}
