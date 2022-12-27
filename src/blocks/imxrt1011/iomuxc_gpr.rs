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
            #[doc = "ccm.ssi3_clk_root"]
            pub const SAI1_MCLK1_SEL_2: u32 = 0x02;
            #[doc = "iomux.sai1_ipg_clk_sai_mclk"]
            pub const SAI1_MCLK1_SEL_3: u32 = 0x03;
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
            #[doc = "ccm.ssi3_clk_root"]
            pub const SAI1_MCLK2_SEL_2: u32 = 0x02;
            #[doc = "iomux.sai1_ipg_clk_sai_mclk"]
            pub const SAI1_MCLK2_SEL_3: u32 = 0x03;
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
    #[doc = "Global Interrupt"]
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
    #[doc = "Enable power saving features on L2 memory"]
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
    #[doc = "This bit controls how memory (OCRAM) enters Deep Sleep mode (shutdown periphery power, but maintain memory contents, outputs of memory are pulled low"]
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
    #[doc = "Divider ratio control for mclk from hmclk"]
    pub mod MQS_CLK_DIV {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "mclk frequency = hmclk frequency"]
            pub const MQS_CLK_DIV_0: u32 = 0;
            #[doc = "mclk frequency = 1/2 * hmclk frequency"]
            pub const MQS_CLK_DIV_1: u32 = 0x01;
            #[doc = "mclk frequency = 1/3 * hmclk frequency"]
            pub const MQS_CLK_DIV_2: u32 = 0x02;
            #[doc = "mclk frequency = 1/256 * hmclk frequency"]
            pub const MQS_CLK_DIV_255: u32 = 0xff;
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
    #[doc = "Medium Quality Sound (MQS) Oversample"]
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
}
#[doc = "GPR3 General Purpose Register"]
pub mod GPR3 {
    #[doc = "Select 128-bit DCP key from 256-bit key from SNVS Master Key"]
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
    #[doc = "IOMUXC XBAR_INOUT2 function direction select"]
    pub mod IOMUXC_XBAR_DIR_SEL_2 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "XBAR_INOUT as input"]
            pub const IOMUXC_XBAR_DIR_SEL_2_0: u32 = 0;
            #[doc = "XBAR_INOUT as output"]
            pub const IOMUXC_XBAR_DIR_SEL_2_1: u32 = 0x01;
        }
    }
    #[doc = "IOMUXC XBAR_INOUT3 function direction select"]
    pub mod IOMUXC_XBAR_DIR_SEL_3 {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "XBAR_INOUT as input"]
            pub const IOMUXC_XBAR_DIR_SEL_3_0: u32 = 0;
            #[doc = "XBAR_INOUT as output"]
            pub const IOMUXC_XBAR_DIR_SEL_3_1: u32 = 0x01;
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
        pub const mask: u32 = 0x1f << offset;
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
        pub const mask: u32 = 0x1f << offset;
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
        }
    }
    #[doc = "Lock M7_APC_AC_R0_CTRL field for changes"]
    pub mod LOCK_M7_APC_AC_R0_CTRL {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Lock M7_APC_AC_R1_CTRL field for changes"]
    pub mod LOCK_M7_APC_AC_R1_CTRL {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Lock M7_APC_AC_R2_CTRL field for changes"]
    pub mod LOCK_M7_APC_AC_R2_CTRL {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Lock M7_APC_AC_R3_CTRL field for changes"]
    pub mod LOCK_M7_APC_AC_R3_CTRL {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x03 << offset;
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
}
#[doc = "GPR13 General Purpose Register"]
pub mod GPR13 {
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
}
#[doc = "GPR14 General Purpose Register"]
pub mod GPR14 {
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
    #[doc = "Lock CM7_INIT_VTOR field for changes"]
    pub mod LOCK_VTOR {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "CM7_INIT_VTOR field is not locked."]
            pub const LOCK_VTOR_0: u32 = 0;
            #[doc = "CM7_INIT_VTOR field is locked (read access only)."]
            pub const LOCK_VTOR_1: u32 = 0x01;
        }
    }
    #[doc = "Vector table offset register out of reset"]
    pub mod CM7_INIT_VTOR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPR17 General Purpose Register"]
pub mod GPR17 {
    #[doc = "FlexRAM bank config value"]
    pub mod FLEXRAM_BANK_CFG {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
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
    #[doc = "Select GPIO1 or GPIO2"]
    pub mod GPIO_SEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPR27 General Purpose Register"]
pub mod GPR27 {
    #[doc = "Start address of flexspi1"]
    pub mod FLEXSPI_REMAP_ADDR_START {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x000f_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPR28 General Purpose Register"]
pub mod GPR28 {
    #[doc = "End address of flexspi1"]
    pub mod FLEXSPI_REMAP_ADDR_END {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x000f_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPR29 General Purpose Register"]
pub mod GPR29 {
    #[doc = "Offset address of flexspi1"]
    pub mod FLEXSPI_REMAP_ADDR_OFFSET {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x000f_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
