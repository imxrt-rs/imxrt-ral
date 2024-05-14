#[doc = "CCM"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "CCM Control Register"]
    pub CCR: crate::RWRegister<u32>,
    _reserved0: [u8; 0x04],
    #[doc = "CCM Status Register"]
    pub CSR: crate::RORegister<u32>,
    #[doc = "CCM Clock Switcher Register"]
    pub CCSR: crate::RWRegister<u32>,
    #[doc = "CCM Arm Clock Root Register"]
    pub CACRR: crate::RWRegister<u32>,
    #[doc = "CCM Bus Clock Divider Register"]
    pub CBCDR: crate::RWRegister<u32>,
    #[doc = "CCM Bus Clock Multiplexer Register"]
    pub CBCMR: crate::RWRegister<u32>,
    #[doc = "CCM Serial Clock Multiplexer Register 1"]
    pub CSCMR1: crate::RWRegister<u32>,
    #[doc = "CCM Serial Clock Multiplexer Register 2"]
    pub CSCMR2: crate::RWRegister<u32>,
    #[doc = "CCM Serial Clock Divider Register 1"]
    pub CSCDR1: crate::RWRegister<u32>,
    #[doc = "CCM Clock Divider Register"]
    pub CS1CDR: crate::RWRegister<u32>,
    #[doc = "CCM Clock Divider Register"]
    pub CS2CDR: crate::RWRegister<u32>,
    #[doc = "CCM D1 Clock Divider Register"]
    pub CDCDR: crate::RWRegister<u32>,
    _reserved1: [u8; 0x04],
    #[doc = "CCM Serial Clock Divider Register 2"]
    pub CSCDR2: crate::RWRegister<u32>,
    #[doc = "CCM Serial Clock Divider Register 3"]
    pub CSCDR3: crate::RWRegister<u32>,
    _reserved2: [u8; 0x08],
    #[doc = "CCM Divider Handshake In-Process Register"]
    pub CDHIPR: crate::RORegister<u32>,
    _reserved3: [u8; 0x08],
    #[doc = "CCM Low Power Control Register"]
    pub CLPCR: crate::RWRegister<u32>,
    #[doc = "CCM Interrupt Status Register"]
    pub CISR: crate::RWRegister<u32>,
    #[doc = "CCM Interrupt Mask Register"]
    pub CIMR: crate::RWRegister<u32>,
    #[doc = "CCM Clock Output Source Register"]
    pub CCOSR: crate::RWRegister<u32>,
    #[doc = "CCM General Purpose Register"]
    pub CGPR: crate::RWRegister<u32>,
    #[doc = "CCM Clock Gating Register 0"]
    pub CCGR0: crate::RWRegister<u32>,
    #[doc = "CCM Clock Gating Register 1"]
    pub CCGR1: crate::RWRegister<u32>,
    #[doc = "CCM Clock Gating Register 2"]
    pub CCGR2: crate::RWRegister<u32>,
    #[doc = "CCM Clock Gating Register 3"]
    pub CCGR3: crate::RWRegister<u32>,
    #[doc = "CCM Clock Gating Register 4"]
    pub CCGR4: crate::RWRegister<u32>,
    #[doc = "CCM Clock Gating Register 5"]
    pub CCGR5: crate::RWRegister<u32>,
    #[doc = "CCM Clock Gating Register 6"]
    pub CCGR6: crate::RWRegister<u32>,
    #[doc = "CCM Clock Gating Register 7"]
    pub CCGR7: crate::RWRegister<u32>,
    #[doc = "CCM Module Enable Overide Register"]
    pub CMEOR: crate::RWRegister<u32>,
}
#[doc = "CCM Control Register"]
pub mod CCR {
    #[doc = "Oscillator ready counter value. These bits define value of 32KHz counter, that serve as counter for oscillator lock time (count to n+1 ckil's). This is used for oscillator lock time. Current estimation is ~5ms. This counter will be used in ignition sequence and in wake from stop sequence if sbyos bit was defined, to notify that on chip oscillator output is ready for the dpll_ip to use and only then the gate in dpll_ip can be opened."]
    pub mod OSCNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "On chip oscillator enable bit - this bit value is reflected on the output cosc_en"]
    pub mod COSC_EN {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "disable on chip oscillator"]
            pub const COSC_EN_0: u32 = 0;
            #[doc = "enable on chip oscillator"]
            pub const COSC_EN_1: u32 = 0x01;
        }
    }
    #[doc = "Counter for analog_reg_bypass signal assertion after standby voltage request by PMIC_STBY_REQ"]
    pub mod REG_BYPASS_COUNT {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "no delay"]
            pub const REG_BYPASS_COUNT_0: u32 = 0;
            #[doc = "1 CKIL clock period delay"]
            pub const REG_BYPASS_COUNT_1: u32 = 0x01;
            #[doc = "63 CKIL clock periods delay"]
            pub const REG_BYPASS_COUNT_63: u32 = 0x3f;
        }
    }
    #[doc = "Enable for REG_BYPASS_COUNTER"]
    pub mod RBC_EN {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "REG_BYPASS_COUNTER disabled"]
            pub const RBC_EN_0: u32 = 0;
            #[doc = "REG_BYPASS_COUNTER enabled."]
            pub const RBC_EN_1: u32 = 0x01;
        }
    }
}
#[doc = "CCM Status Register"]
pub mod CSR {
    #[doc = "Status of the value of CCM_REF_EN_B output of ccm"]
    pub mod REF_EN_B {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "value of CCM_REF_EN_B is '0'"]
            pub const REF_EN_B_0: u32 = 0;
            #[doc = "value of CCM_REF_EN_B is '1'"]
            pub const REF_EN_B_1: u32 = 0x01;
        }
    }
    #[doc = "Status indication of CAMP2."]
    pub mod CAMP2_READY {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "CAMP2 is not ready."]
            pub const CAMP2_READY_0: u32 = 0;
            #[doc = "CAMP2 is ready."]
            pub const CAMP2_READY_1: u32 = 0x01;
        }
    }
    #[doc = "Status indication of on board oscillator"]
    pub mod COSC_READY {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "on board oscillator is not ready."]
            pub const COSC_READY_0: u32 = 0;
            #[doc = "on board oscillator is ready."]
            pub const COSC_READY_1: u32 = 0x01;
        }
    }
}
#[doc = "CCM Clock Switcher Register"]
pub mod CCSR {
    #[doc = "Selects source to generate pll3_sw_clk. This bit should only be used for testing purposes."]
    pub mod PLL3_SW_CLK_SEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "pll3_main_clk"]
            pub const PLL3_SW_CLK_SEL_0: u32 = 0;
            #[doc = "pll3 bypass clock"]
            pub const PLL3_SW_CLK_SEL_1: u32 = 0x01;
        }
    }
}
#[doc = "CCM Arm Clock Root Register"]
pub mod CACRR {
    #[doc = "Divider for ARM clock root"]
    pub mod ARM_PODF {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "divide by 1"]
            pub const ARM_PODF_0: u32 = 0;
            #[doc = "divide by 2"]
            pub const ARM_PODF_1: u32 = 0x01;
            #[doc = "divide by 3"]
            pub const ARM_PODF_2: u32 = 0x02;
            #[doc = "divide by 4"]
            pub const ARM_PODF_3: u32 = 0x03;
            #[doc = "divide by 5"]
            pub const ARM_PODF_4: u32 = 0x04;
            #[doc = "divide by 6"]
            pub const ARM_PODF_5: u32 = 0x05;
            #[doc = "divide by 7"]
            pub const ARM_PODF_6: u32 = 0x06;
            #[doc = "divide by 8"]
            pub const ARM_PODF_7: u32 = 0x07;
        }
    }
}
#[doc = "CCM Bus Clock Divider Register"]
pub mod CBCDR {
    #[doc = "SEMC clock source select"]
    pub mod SEMC_CLK_SEL {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Periph_clk output will be used as SEMC clock root"]
            pub const SEMC_CLK_SEL_0: u32 = 0;
            #[doc = "SEMC alternative clock will be used as SEMC clock root"]
            pub const SEMC_CLK_SEL_1: u32 = 0x01;
        }
    }
    #[doc = "SEMC alternative clock select"]
    pub mod SEMC_ALT_CLK_SEL {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "PLL2 PFD2 will be selected as alternative clock for SEMC root clock"]
            pub const SEMC_ALT_CLK_SEL_0: u32 = 0;
            #[doc = "PLL3 PFD1 will be selected as alternative clock for SEMC root clock"]
            pub const SEMC_ALT_CLK_SEL_1: u32 = 0x01;
        }
    }
    #[doc = "Divider for ipg podf."]
    pub mod IPG_PODF {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "divide by 1"]
            pub const IPG_PODF_0: u32 = 0;
            #[doc = "divide by 2"]
            pub const IPG_PODF_1: u32 = 0x01;
            #[doc = "divide by 3"]
            pub const IPG_PODF_2: u32 = 0x02;
            #[doc = "divide by 4"]
            pub const IPG_PODF_3: u32 = 0x03;
        }
    }
    #[doc = "Divider for AHB PODF"]
    pub mod AHB_PODF {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "divide by 1"]
            pub const AHB_PODF_0: u32 = 0;
            #[doc = "divide by 2"]
            pub const AHB_PODF_1: u32 = 0x01;
            #[doc = "divide by 3"]
            pub const AHB_PODF_2: u32 = 0x02;
            #[doc = "divide by 4"]
            pub const AHB_PODF_3: u32 = 0x03;
            #[doc = "divide by 5"]
            pub const AHB_PODF_4: u32 = 0x04;
            #[doc = "divide by 6"]
            pub const AHB_PODF_5: u32 = 0x05;
            #[doc = "divide by 7"]
            pub const AHB_PODF_6: u32 = 0x06;
            #[doc = "divide by 8"]
            pub const AHB_PODF_7: u32 = 0x07;
        }
    }
    #[doc = "Post divider for SEMC clock"]
    pub mod SEMC_PODF {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "divide by 1"]
            pub const SEMC_PODF_0: u32 = 0;
            #[doc = "divide by 2"]
            pub const SEMC_PODF_1: u32 = 0x01;
            #[doc = "divide by 3"]
            pub const SEMC_PODF_2: u32 = 0x02;
            #[doc = "divide by 4"]
            pub const SEMC_PODF_3: u32 = 0x03;
            #[doc = "divide by 5"]
            pub const SEMC_PODF_4: u32 = 0x04;
            #[doc = "divide by 6"]
            pub const SEMC_PODF_5: u32 = 0x05;
            #[doc = "divide by 7"]
            pub const SEMC_PODF_6: u32 = 0x06;
            #[doc = "divide by 8"]
            pub const SEMC_PODF_7: u32 = 0x07;
        }
    }
    #[doc = "Selector for peripheral main clock"]
    pub mod PERIPH_CLK_SEL {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "derive clock selected by CCM_CBCMR\\[CORE_CLK_PRE_SEL\\]"]
            pub const PERIPH_CLK_SEL_0: u32 = 0;
            #[doc = "derive clock selected by CCM_CBCMR\\[PERIPH_CLK2_SEL\\]"]
            pub const PERIPH_CLK_SEL_1: u32 = 0x01;
        }
    }
    #[doc = "Divider for periph_clk2_podf."]
    pub mod PERIPH_CLK2_PODF {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "divide by 1"]
            pub const PERIPH_CLK2_PODF_0: u32 = 0;
            #[doc = "divide by 2"]
            pub const PERIPH_CLK2_PODF_1: u32 = 0x01;
            #[doc = "divide by 3"]
            pub const PERIPH_CLK2_PODF_2: u32 = 0x02;
            #[doc = "divide by 4"]
            pub const PERIPH_CLK2_PODF_3: u32 = 0x03;
            #[doc = "divide by 5"]
            pub const PERIPH_CLK2_PODF_4: u32 = 0x04;
            #[doc = "divide by 6"]
            pub const PERIPH_CLK2_PODF_5: u32 = 0x05;
            #[doc = "divide by 7"]
            pub const PERIPH_CLK2_PODF_6: u32 = 0x06;
            #[doc = "divide by 8"]
            pub const PERIPH_CLK2_PODF_7: u32 = 0x07;
        }
    }
}
#[doc = "CCM Bus Clock Multiplexer Register"]
pub mod CBCMR {
    #[doc = "Selector for lpspi clock multiplexer"]
    pub mod LPSPI_CLK_SEL {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "derive clock from PLL3 PFD1 clk"]
            pub const LPSPI_CLK_SEL_0: u32 = 0;
            #[doc = "derive clock from PLL3 PFD0"]
            pub const LPSPI_CLK_SEL_1: u32 = 0x01;
            #[doc = "derive clock from PLL2"]
            pub const LPSPI_CLK_SEL_2: u32 = 0x02;
            #[doc = "derive clock from PLL2 PFD2"]
            pub const LPSPI_CLK_SEL_3: u32 = 0x03;
        }
    }
    #[doc = "Selector for flexspi2 clock multiplexer"]
    pub mod FLEXSPI2_CLK_SEL {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "derive clock from PLL2 PFD2"]
            pub const FLEXSPI2_CLK_SEL_0: u32 = 0;
            #[doc = "derive clock from PLL3 PFD0"]
            pub const FLEXSPI2_CLK_SEL_1: u32 = 0x01;
            #[doc = "derive clock from PLL3 PFD1"]
            pub const FLEXSPI2_CLK_SEL_2: u32 = 0x02;
            #[doc = "derive clock from PLL2 (pll2_main_clk)"]
            pub const FLEXSPI2_CLK_SEL_3: u32 = 0x03;
        }
    }
    #[doc = "Selector for peripheral clk2 clock multiplexer"]
    pub mod PERIPH_CLK2_SEL {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "derive clock from pll3_sw_clk"]
            pub const PERIPH_CLK2_SEL_0: u32 = 0;
            #[doc = "derive clock from osc_clk"]
            pub const PERIPH_CLK2_SEL_1: u32 = 0x01;
            #[doc = "derive clock from pll2_bypass_clk"]
            pub const PERIPH_CLK2_SEL_2: u32 = 0x02;
        }
    }
    #[doc = "Selector for Trace clock multiplexer"]
    pub mod TRACE_CLK_SEL {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "derive clock from PLL2"]
            pub const TRACE_CLK_SEL_0: u32 = 0;
            #[doc = "derive clock from PLL2 PFD2"]
            pub const TRACE_CLK_SEL_1: u32 = 0x01;
            #[doc = "derive clock from PLL2 PFD0"]
            pub const TRACE_CLK_SEL_2: u32 = 0x02;
            #[doc = "derive clock from PLL2 PFD1"]
            pub const TRACE_CLK_SEL_3: u32 = 0x03;
        }
    }
    #[doc = "Selector for pre_periph clock multiplexer"]
    pub mod PRE_PERIPH_CLK_SEL {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "derive clock from PLL2"]
            pub const PRE_PERIPH_CLK_SEL_0: u32 = 0;
            #[doc = "derive clock from PLL3 PFD3"]
            pub const PRE_PERIPH_CLK_SEL_1: u32 = 0x01;
            #[doc = "derive clock from PLL2 PFD3"]
            pub const PRE_PERIPH_CLK_SEL_2: u32 = 0x02;
            #[doc = "derive clock from PLL6"]
            pub const PRE_PERIPH_CLK_SEL_3: u32 = 0x03;
        }
    }
    #[doc = "Post-divider for LCDIF clock."]
    pub mod LCDIF_PODF {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "divide by 1"]
            pub const LCDIF_PODF_0: u32 = 0;
            #[doc = "divide by 2"]
            pub const LCDIF_PODF_1: u32 = 0x01;
            #[doc = "divide by 3"]
            pub const LCDIF_PODF_2: u32 = 0x02;
            #[doc = "divide by 4"]
            pub const LCDIF_PODF_3: u32 = 0x03;
            #[doc = "divide by 5"]
            pub const LCDIF_PODF_4: u32 = 0x04;
            #[doc = "divide by 6"]
            pub const LCDIF_PODF_5: u32 = 0x05;
            #[doc = "divide by 7"]
            pub const LCDIF_PODF_6: u32 = 0x06;
            #[doc = "divide by 8"]
            pub const LCDIF_PODF_7: u32 = 0x07;
        }
    }
    #[doc = "Divider for LPSPI. Divider should be updated when output clock is gated."]
    pub mod LPSPI_PODF {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "divide by 1"]
            pub const LPSPI_PODF_0: u32 = 0;
            #[doc = "divide by 2"]
            pub const LPSPI_PODF_1: u32 = 0x01;
            #[doc = "divide by 3"]
            pub const LPSPI_PODF_2: u32 = 0x02;
            #[doc = "divide by 4"]
            pub const LPSPI_PODF_3: u32 = 0x03;
            #[doc = "divide by 5"]
            pub const LPSPI_PODF_4: u32 = 0x04;
            #[doc = "divide by 6"]
            pub const LPSPI_PODF_5: u32 = 0x05;
            #[doc = "divide by 7"]
            pub const LPSPI_PODF_6: u32 = 0x06;
            #[doc = "divide by 8"]
            pub const LPSPI_PODF_7: u32 = 0x07;
        }
    }
    #[doc = "Divider for flexspi2 clock root."]
    pub mod FLEXSPI2_PODF {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "divide by 1"]
            pub const FLEXSPI2_PODF_0: u32 = 0;
            #[doc = "divide by 2"]
            pub const FLEXSPI2_PODF_1: u32 = 0x01;
            #[doc = "divide by 3"]
            pub const FLEXSPI2_PODF_2: u32 = 0x02;
            #[doc = "divide by 4"]
            pub const FLEXSPI2_PODF_3: u32 = 0x03;
            #[doc = "divide by 5"]
            pub const FLEXSPI2_PODF_4: u32 = 0x04;
            #[doc = "divide by 6"]
            pub const FLEXSPI2_PODF_5: u32 = 0x05;
            #[doc = "divide by 7"]
            pub const FLEXSPI2_PODF_6: u32 = 0x06;
            #[doc = "divide by 8"]
            pub const FLEXSPI2_PODF_7: u32 = 0x07;
        }
    }
}
#[doc = "CCM Serial Clock Multiplexer Register 1"]
pub mod CSCMR1 {
    #[doc = "Divider for perclk podf."]
    pub mod PERCLK_PODF {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Divide by 1"]
            pub const DIVIDE_1: u32 = 0;
            #[doc = "Divide by 2"]
            pub const DIVIDE_2: u32 = 0x01;
            #[doc = "Divide by 3"]
            pub const DIVIDE_3: u32 = 0x02;
            #[doc = "Divide by 4"]
            pub const DIVIDE_4: u32 = 0x03;
            #[doc = "Divide by 5"]
            pub const DIVIDE_5: u32 = 0x04;
            #[doc = "Divide by 6"]
            pub const DIVIDE_6: u32 = 0x05;
            #[doc = "Divide by 7"]
            pub const DIVIDE_7: u32 = 0x06;
            #[doc = "Divide by 8"]
            pub const DIVIDE_8: u32 = 0x07;
            #[doc = "Divide by 9"]
            pub const DIVIDE_9: u32 = 0x08;
            #[doc = "Divide by 10"]
            pub const DIVIDE_10: u32 = 0x09;
            #[doc = "Divide by 11"]
            pub const DIVIDE_11: u32 = 0x0a;
            #[doc = "Divide by 12"]
            pub const DIVIDE_12: u32 = 0x0b;
            #[doc = "Divide by 13"]
            pub const DIVIDE_13: u32 = 0x0c;
            #[doc = "Divide by 14"]
            pub const DIVIDE_14: u32 = 0x0d;
            #[doc = "Divide by 15"]
            pub const DIVIDE_15: u32 = 0x0e;
            #[doc = "Divide by 16"]
            pub const DIVIDE_16: u32 = 0x0f;
            #[doc = "Divide by 17"]
            pub const DIVIDE_17: u32 = 0x10;
            #[doc = "Divide by 18"]
            pub const DIVIDE_18: u32 = 0x11;
            #[doc = "Divide by 19"]
            pub const DIVIDE_19: u32 = 0x12;
            #[doc = "Divide by 20"]
            pub const DIVIDE_20: u32 = 0x13;
            #[doc = "Divide by 21"]
            pub const DIVIDE_21: u32 = 0x14;
            #[doc = "Divide by 22"]
            pub const DIVIDE_22: u32 = 0x15;
            #[doc = "Divide by 23"]
            pub const DIVIDE_23: u32 = 0x16;
            #[doc = "Divide by 24"]
            pub const DIVIDE_24: u32 = 0x17;
            #[doc = "Divide by 25"]
            pub const DIVIDE_25: u32 = 0x18;
            #[doc = "Divide by 26"]
            pub const DIVIDE_26: u32 = 0x19;
            #[doc = "Divide by 27"]
            pub const DIVIDE_27: u32 = 0x1a;
            #[doc = "Divide by 28"]
            pub const DIVIDE_28: u32 = 0x1b;
            #[doc = "Divide by 29"]
            pub const DIVIDE_29: u32 = 0x1c;
            #[doc = "Divide by 30"]
            pub const DIVIDE_30: u32 = 0x1d;
            #[doc = "Divide by 31"]
            pub const DIVIDE_31: u32 = 0x1e;
            #[doc = "Divide by 32"]
            pub const DIVIDE_32: u32 = 0x1f;
            #[doc = "Divide by 33"]
            pub const DIVIDE_33: u32 = 0x20;
            #[doc = "Divide by 34"]
            pub const DIVIDE_34: u32 = 0x21;
            #[doc = "Divide by 35"]
            pub const DIVIDE_35: u32 = 0x22;
            #[doc = "Divide by 36"]
            pub const DIVIDE_36: u32 = 0x23;
            #[doc = "Divide by 37"]
            pub const DIVIDE_37: u32 = 0x24;
            #[doc = "Divide by 38"]
            pub const DIVIDE_38: u32 = 0x25;
            #[doc = "Divide by 39"]
            pub const DIVIDE_39: u32 = 0x26;
            #[doc = "Divide by 40"]
            pub const DIVIDE_40: u32 = 0x27;
            #[doc = "Divide by 41"]
            pub const DIVIDE_41: u32 = 0x28;
            #[doc = "Divide by 42"]
            pub const DIVIDE_42: u32 = 0x29;
            #[doc = "Divide by 43"]
            pub const DIVIDE_43: u32 = 0x2a;
            #[doc = "Divide by 44"]
            pub const DIVIDE_44: u32 = 0x2b;
            #[doc = "Divide by 45"]
            pub const DIVIDE_45: u32 = 0x2c;
            #[doc = "Divide by 46"]
            pub const DIVIDE_46: u32 = 0x2d;
            #[doc = "Divide by 47"]
            pub const DIVIDE_47: u32 = 0x2e;
            #[doc = "Divide by 48"]
            pub const DIVIDE_48: u32 = 0x2f;
            #[doc = "Divide by 49"]
            pub const DIVIDE_49: u32 = 0x30;
            #[doc = "Divide by 50"]
            pub const DIVIDE_50: u32 = 0x31;
            #[doc = "Divide by 51"]
            pub const DIVIDE_51: u32 = 0x32;
            #[doc = "Divide by 52"]
            pub const DIVIDE_52: u32 = 0x33;
            #[doc = "Divide by 53"]
            pub const DIVIDE_53: u32 = 0x34;
            #[doc = "Divide by 54"]
            pub const DIVIDE_54: u32 = 0x35;
            #[doc = "Divide by 55"]
            pub const DIVIDE_55: u32 = 0x36;
            #[doc = "Divide by 56"]
            pub const DIVIDE_56: u32 = 0x37;
            #[doc = "Divide by 57"]
            pub const DIVIDE_57: u32 = 0x38;
            #[doc = "Divide by 58"]
            pub const DIVIDE_58: u32 = 0x39;
            #[doc = "Divide by 59"]
            pub const DIVIDE_59: u32 = 0x3a;
            #[doc = "Divide by 60"]
            pub const DIVIDE_60: u32 = 0x3b;
            #[doc = "Divide by 61"]
            pub const DIVIDE_61: u32 = 0x3c;
            #[doc = "Divide by 62"]
            pub const DIVIDE_62: u32 = 0x3d;
            #[doc = "Divide by 63"]
            pub const DIVIDE_63: u32 = 0x3e;
            #[doc = "Divide by 64"]
            pub const DIVIDE_64: u32 = 0x3f;
        }
    }
    #[doc = "Selector for the perclk clock multiplexor"]
    pub mod PERCLK_CLK_SEL {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "derive clock from ipg clk root"]
            pub const PERCLK_CLK_SEL_0: u32 = 0;
            #[doc = "derive clock from osc_clk"]
            pub const PERCLK_CLK_SEL_1: u32 = 0x01;
        }
    }
    #[doc = "Selector for sai1 clock multiplexer"]
    pub mod SAI1_CLK_SEL {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "derive clock from PLL3 PFD2"]
            pub const SAI1_CLK_SEL_0: u32 = 0;
            #[doc = "derive from pll3_sw_clk"]
            pub const SAI1_CLK_SEL_1: u32 = 0x01;
            #[doc = "derive clock from PLL4"]
            pub const SAI1_CLK_SEL_2: u32 = 0x02;
        }
    }
    #[doc = "Selector for sai2 clock multiplexer"]
    pub mod SAI2_CLK_SEL {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "derive clock from PLL3 PFD2"]
            pub const SAI2_CLK_SEL_0: u32 = 0;
            #[doc = "derive clock from PLL5"]
            pub const SAI2_CLK_SEL_1: u32 = 0x01;
            #[doc = "derive clock from PLL4"]
            pub const SAI2_CLK_SEL_2: u32 = 0x02;
        }
    }
    #[doc = "Selector for sai3 clock multiplexer"]
    pub mod SAI3_CLK_SEL {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "derive clock from PLL3 PFD2"]
            pub const SAI3_CLK_SEL_0: u32 = 0;
            #[doc = "derive from pll3_sw_clk"]
            pub const SAI3_CLK_SEL_1: u32 = 0x01;
            #[doc = "derive clock from PLL4"]
            pub const SAI3_CLK_SEL_2: u32 = 0x02;
        }
    }
    #[doc = "Selector for usdhc1 clock multiplexer"]
    pub mod USDHC1_CLK_SEL {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "derive clock from PLL2 PFD2"]
            pub const USDHC1_CLK_SEL_0: u32 = 0;
            #[doc = "derive clock from PLL2 PFD0"]
            pub const USDHC1_CLK_SEL_1: u32 = 0x01;
        }
    }
    #[doc = "Selector for usdhc2 clock multiplexer"]
    pub mod USDHC2_CLK_SEL {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "derive clock from PLL2 PFD2"]
            pub const USDHC2_CLK_SEL_0: u32 = 0;
            #[doc = "derive clock from PLL2 PFD0"]
            pub const USDHC2_CLK_SEL_1: u32 = 0x01;
        }
    }
    #[doc = "Divider for flexspi clock root."]
    pub mod FLEXSPI_PODF {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "divide by 1"]
            pub const FLEXSPI_PODF_0: u32 = 0;
            #[doc = "divide by 2"]
            pub const FLEXSPI_PODF_1: u32 = 0x01;
            #[doc = "divide by 3"]
            pub const FLEXSPI_PODF_2: u32 = 0x02;
            #[doc = "divide by 4"]
            pub const FLEXSPI_PODF_3: u32 = 0x03;
            #[doc = "divide by 5"]
            pub const FLEXSPI_PODF_4: u32 = 0x04;
            #[doc = "divide by 6"]
            pub const FLEXSPI_PODF_5: u32 = 0x05;
            #[doc = "divide by 7"]
            pub const FLEXSPI_PODF_6: u32 = 0x06;
            #[doc = "divide by 8"]
            pub const FLEXSPI_PODF_7: u32 = 0x07;
        }
    }
    #[doc = "Selector for flexspi clock multiplexer"]
    pub mod FLEXSPI_CLK_SEL {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "derive clock from PLL2"]
            pub const FLEXSPI_CLK_SEL_0: u32 = 0;
            #[doc = "derive clock from pll3_sw_clk"]
            pub const FLEXSPI_CLK_SEL_1: u32 = 0x01;
            #[doc = "derive clock from PLL2 PFD2"]
            pub const FLEXSPI_CLK_SEL_2: u32 = 0x02;
            #[doc = "derive clock from PLL3 PFD0"]
            pub const FLEXSPI_CLK_SEL_3: u32 = 0x03;
        }
    }
}
#[doc = "CCM Serial Clock Multiplexer Register 2"]
pub mod CSCMR2 {
    #[doc = "Divider for CAN clock podf."]
    pub mod CAN_CLK_PODF {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Divide by 1"]
            pub const DIVIDE_1: u32 = 0;
            #[doc = "Divide by 2"]
            pub const DIVIDE_2: u32 = 0x01;
            #[doc = "Divide by 3"]
            pub const DIVIDE_3: u32 = 0x02;
            #[doc = "Divide by 4"]
            pub const DIVIDE_4: u32 = 0x03;
            #[doc = "Divide by 5"]
            pub const DIVIDE_5: u32 = 0x04;
            #[doc = "Divide by 6"]
            pub const DIVIDE_6: u32 = 0x05;
            #[doc = "Divide by 7"]
            pub const DIVIDE_7: u32 = 0x06;
            #[doc = "Divide by 8"]
            pub const DIVIDE_8: u32 = 0x07;
            #[doc = "Divide by 9"]
            pub const DIVIDE_9: u32 = 0x08;
            #[doc = "Divide by 10"]
            pub const DIVIDE_10: u32 = 0x09;
            #[doc = "Divide by 11"]
            pub const DIVIDE_11: u32 = 0x0a;
            #[doc = "Divide by 12"]
            pub const DIVIDE_12: u32 = 0x0b;
            #[doc = "Divide by 13"]
            pub const DIVIDE_13: u32 = 0x0c;
            #[doc = "Divide by 14"]
            pub const DIVIDE_14: u32 = 0x0d;
            #[doc = "Divide by 15"]
            pub const DIVIDE_15: u32 = 0x0e;
            #[doc = "Divide by 16"]
            pub const DIVIDE_16: u32 = 0x0f;
            #[doc = "Divide by 17"]
            pub const DIVIDE_17: u32 = 0x10;
            #[doc = "Divide by 18"]
            pub const DIVIDE_18: u32 = 0x11;
            #[doc = "Divide by 19"]
            pub const DIVIDE_19: u32 = 0x12;
            #[doc = "Divide by 20"]
            pub const DIVIDE_20: u32 = 0x13;
            #[doc = "Divide by 21"]
            pub const DIVIDE_21: u32 = 0x14;
            #[doc = "Divide by 22"]
            pub const DIVIDE_22: u32 = 0x15;
            #[doc = "Divide by 23"]
            pub const DIVIDE_23: u32 = 0x16;
            #[doc = "Divide by 24"]
            pub const DIVIDE_24: u32 = 0x17;
            #[doc = "Divide by 25"]
            pub const DIVIDE_25: u32 = 0x18;
            #[doc = "Divide by 26"]
            pub const DIVIDE_26: u32 = 0x19;
            #[doc = "Divide by 27"]
            pub const DIVIDE_27: u32 = 0x1a;
            #[doc = "Divide by 28"]
            pub const DIVIDE_28: u32 = 0x1b;
            #[doc = "Divide by 29"]
            pub const DIVIDE_29: u32 = 0x1c;
            #[doc = "Divide by 30"]
            pub const DIVIDE_30: u32 = 0x1d;
            #[doc = "Divide by 31"]
            pub const DIVIDE_31: u32 = 0x1e;
            #[doc = "Divide by 32"]
            pub const DIVIDE_32: u32 = 0x1f;
            #[doc = "Divide by 33"]
            pub const DIVIDE_33: u32 = 0x20;
            #[doc = "Divide by 34"]
            pub const DIVIDE_34: u32 = 0x21;
            #[doc = "Divide by 35"]
            pub const DIVIDE_35: u32 = 0x22;
            #[doc = "Divide by 36"]
            pub const DIVIDE_36: u32 = 0x23;
            #[doc = "Divide by 37"]
            pub const DIVIDE_37: u32 = 0x24;
            #[doc = "Divide by 38"]
            pub const DIVIDE_38: u32 = 0x25;
            #[doc = "Divide by 39"]
            pub const DIVIDE_39: u32 = 0x26;
            #[doc = "Divide by 40"]
            pub const DIVIDE_40: u32 = 0x27;
            #[doc = "Divide by 41"]
            pub const DIVIDE_41: u32 = 0x28;
            #[doc = "Divide by 42"]
            pub const DIVIDE_42: u32 = 0x29;
            #[doc = "Divide by 43"]
            pub const DIVIDE_43: u32 = 0x2a;
            #[doc = "Divide by 44"]
            pub const DIVIDE_44: u32 = 0x2b;
            #[doc = "Divide by 45"]
            pub const DIVIDE_45: u32 = 0x2c;
            #[doc = "Divide by 46"]
            pub const DIVIDE_46: u32 = 0x2d;
            #[doc = "Divide by 47"]
            pub const DIVIDE_47: u32 = 0x2e;
            #[doc = "Divide by 48"]
            pub const DIVIDE_48: u32 = 0x2f;
            #[doc = "Divide by 49"]
            pub const DIVIDE_49: u32 = 0x30;
            #[doc = "Divide by 50"]
            pub const DIVIDE_50: u32 = 0x31;
            #[doc = "Divide by 51"]
            pub const DIVIDE_51: u32 = 0x32;
            #[doc = "Divide by 52"]
            pub const DIVIDE_52: u32 = 0x33;
            #[doc = "Divide by 53"]
            pub const DIVIDE_53: u32 = 0x34;
            #[doc = "Divide by 54"]
            pub const DIVIDE_54: u32 = 0x35;
            #[doc = "Divide by 55"]
            pub const DIVIDE_55: u32 = 0x36;
            #[doc = "Divide by 56"]
            pub const DIVIDE_56: u32 = 0x37;
            #[doc = "Divide by 57"]
            pub const DIVIDE_57: u32 = 0x38;
            #[doc = "Divide by 58"]
            pub const DIVIDE_58: u32 = 0x39;
            #[doc = "Divide by 59"]
            pub const DIVIDE_59: u32 = 0x3a;
            #[doc = "Divide by 60"]
            pub const DIVIDE_60: u32 = 0x3b;
            #[doc = "Divide by 61"]
            pub const DIVIDE_61: u32 = 0x3c;
            #[doc = "Divide by 62"]
            pub const DIVIDE_62: u32 = 0x3d;
            #[doc = "Divide by 63"]
            pub const DIVIDE_63: u32 = 0x3e;
            #[doc = "Divide by 64"]
            pub const DIVIDE_64: u32 = 0x3f;
        }
    }
    #[doc = "Selector for CAN clock multiplexer"]
    pub mod CAN_CLK_SEL {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "derive clock from pll3_sw_clk divided clock (60M)"]
            pub const CAN_CLK_SEL_0: u32 = 0;
            #[doc = "derive clock from osc_clk (24M)"]
            pub const CAN_CLK_SEL_1: u32 = 0x01;
            #[doc = "derive clock from pll3_sw_clk divided clock (80M)"]
            pub const CAN_CLK_SEL_2: u32 = 0x02;
            #[doc = "Disable FlexCAN clock"]
            pub const CAN_CLK_SEL_3: u32 = 0x03;
        }
    }
    #[doc = "Selector for flexio2 clock multiplexer"]
    pub mod FLEXIO2_CLK_SEL {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "derive clock from PLL4 divided clock"]
            pub const FLEXIO2_CLK_SEL_0: u32 = 0;
            #[doc = "derive clock from PLL3 PFD2 clock"]
            pub const FLEXIO2_CLK_SEL_1: u32 = 0x01;
            #[doc = "derive clock from PLL5 clock"]
            pub const FLEXIO2_CLK_SEL_2: u32 = 0x02;
            #[doc = "derive clock from pll3_sw_clk"]
            pub const FLEXIO2_CLK_SEL_3: u32 = 0x03;
        }
    }
}
#[doc = "CCM Serial Clock Divider Register 1"]
pub mod CSCDR1 {
    #[doc = "Divider for uart clock podf."]
    pub mod UART_CLK_PODF {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Divide by 1"]
            pub const DIVIDE_1: u32 = 0;
            #[doc = "Divide by 2"]
            pub const DIVIDE_2: u32 = 0x01;
            #[doc = "Divide by 3"]
            pub const DIVIDE_3: u32 = 0x02;
            #[doc = "Divide by 4"]
            pub const DIVIDE_4: u32 = 0x03;
            #[doc = "Divide by 5"]
            pub const DIVIDE_5: u32 = 0x04;
            #[doc = "Divide by 6"]
            pub const DIVIDE_6: u32 = 0x05;
            #[doc = "Divide by 7"]
            pub const DIVIDE_7: u32 = 0x06;
            #[doc = "Divide by 8"]
            pub const DIVIDE_8: u32 = 0x07;
            #[doc = "Divide by 9"]
            pub const DIVIDE_9: u32 = 0x08;
            #[doc = "Divide by 10"]
            pub const DIVIDE_10: u32 = 0x09;
            #[doc = "Divide by 11"]
            pub const DIVIDE_11: u32 = 0x0a;
            #[doc = "Divide by 12"]
            pub const DIVIDE_12: u32 = 0x0b;
            #[doc = "Divide by 13"]
            pub const DIVIDE_13: u32 = 0x0c;
            #[doc = "Divide by 14"]
            pub const DIVIDE_14: u32 = 0x0d;
            #[doc = "Divide by 15"]
            pub const DIVIDE_15: u32 = 0x0e;
            #[doc = "Divide by 16"]
            pub const DIVIDE_16: u32 = 0x0f;
            #[doc = "Divide by 17"]
            pub const DIVIDE_17: u32 = 0x10;
            #[doc = "Divide by 18"]
            pub const DIVIDE_18: u32 = 0x11;
            #[doc = "Divide by 19"]
            pub const DIVIDE_19: u32 = 0x12;
            #[doc = "Divide by 20"]
            pub const DIVIDE_20: u32 = 0x13;
            #[doc = "Divide by 21"]
            pub const DIVIDE_21: u32 = 0x14;
            #[doc = "Divide by 22"]
            pub const DIVIDE_22: u32 = 0x15;
            #[doc = "Divide by 23"]
            pub const DIVIDE_23: u32 = 0x16;
            #[doc = "Divide by 24"]
            pub const DIVIDE_24: u32 = 0x17;
            #[doc = "Divide by 25"]
            pub const DIVIDE_25: u32 = 0x18;
            #[doc = "Divide by 26"]
            pub const DIVIDE_26: u32 = 0x19;
            #[doc = "Divide by 27"]
            pub const DIVIDE_27: u32 = 0x1a;
            #[doc = "Divide by 28"]
            pub const DIVIDE_28: u32 = 0x1b;
            #[doc = "Divide by 29"]
            pub const DIVIDE_29: u32 = 0x1c;
            #[doc = "Divide by 30"]
            pub const DIVIDE_30: u32 = 0x1d;
            #[doc = "Divide by 31"]
            pub const DIVIDE_31: u32 = 0x1e;
            #[doc = "Divide by 32"]
            pub const DIVIDE_32: u32 = 0x1f;
            #[doc = "Divide by 33"]
            pub const DIVIDE_33: u32 = 0x20;
            #[doc = "Divide by 34"]
            pub const DIVIDE_34: u32 = 0x21;
            #[doc = "Divide by 35"]
            pub const DIVIDE_35: u32 = 0x22;
            #[doc = "Divide by 36"]
            pub const DIVIDE_36: u32 = 0x23;
            #[doc = "Divide by 37"]
            pub const DIVIDE_37: u32 = 0x24;
            #[doc = "Divide by 38"]
            pub const DIVIDE_38: u32 = 0x25;
            #[doc = "Divide by 39"]
            pub const DIVIDE_39: u32 = 0x26;
            #[doc = "Divide by 40"]
            pub const DIVIDE_40: u32 = 0x27;
            #[doc = "Divide by 41"]
            pub const DIVIDE_41: u32 = 0x28;
            #[doc = "Divide by 42"]
            pub const DIVIDE_42: u32 = 0x29;
            #[doc = "Divide by 43"]
            pub const DIVIDE_43: u32 = 0x2a;
            #[doc = "Divide by 44"]
            pub const DIVIDE_44: u32 = 0x2b;
            #[doc = "Divide by 45"]
            pub const DIVIDE_45: u32 = 0x2c;
            #[doc = "Divide by 46"]
            pub const DIVIDE_46: u32 = 0x2d;
            #[doc = "Divide by 47"]
            pub const DIVIDE_47: u32 = 0x2e;
            #[doc = "Divide by 48"]
            pub const DIVIDE_48: u32 = 0x2f;
            #[doc = "Divide by 49"]
            pub const DIVIDE_49: u32 = 0x30;
            #[doc = "Divide by 50"]
            pub const DIVIDE_50: u32 = 0x31;
            #[doc = "Divide by 51"]
            pub const DIVIDE_51: u32 = 0x32;
            #[doc = "Divide by 52"]
            pub const DIVIDE_52: u32 = 0x33;
            #[doc = "Divide by 53"]
            pub const DIVIDE_53: u32 = 0x34;
            #[doc = "Divide by 54"]
            pub const DIVIDE_54: u32 = 0x35;
            #[doc = "Divide by 55"]
            pub const DIVIDE_55: u32 = 0x36;
            #[doc = "Divide by 56"]
            pub const DIVIDE_56: u32 = 0x37;
            #[doc = "Divide by 57"]
            pub const DIVIDE_57: u32 = 0x38;
            #[doc = "Divide by 58"]
            pub const DIVIDE_58: u32 = 0x39;
            #[doc = "Divide by 59"]
            pub const DIVIDE_59: u32 = 0x3a;
            #[doc = "Divide by 60"]
            pub const DIVIDE_60: u32 = 0x3b;
            #[doc = "Divide by 61"]
            pub const DIVIDE_61: u32 = 0x3c;
            #[doc = "Divide by 62"]
            pub const DIVIDE_62: u32 = 0x3d;
            #[doc = "Divide by 63"]
            pub const DIVIDE_63: u32 = 0x3e;
            #[doc = "Divide by 64"]
            pub const DIVIDE_64: u32 = 0x3f;
        }
    }
    #[doc = "Selector for the UART clock multiplexor"]
    pub mod UART_CLK_SEL {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "derive clock from pll3_80m"]
            pub const UART_CLK_SEL_0: u32 = 0;
            #[doc = "derive clock from osc_clk"]
            pub const UART_CLK_SEL_1: u32 = 0x01;
        }
    }
    #[doc = "Divider for usdhc1 clock podf. Divider should be updated when output clock is gated."]
    pub mod USDHC1_PODF {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "divide by 1"]
            pub const USDHC1_PODF_0: u32 = 0;
            #[doc = "divide by 2"]
            pub const USDHC1_PODF_1: u32 = 0x01;
            #[doc = "divide by 3"]
            pub const USDHC1_PODF_2: u32 = 0x02;
            #[doc = "divide by 4"]
            pub const USDHC1_PODF_3: u32 = 0x03;
            #[doc = "divide by 5"]
            pub const USDHC1_PODF_4: u32 = 0x04;
            #[doc = "divide by 6"]
            pub const USDHC1_PODF_5: u32 = 0x05;
            #[doc = "divide by 7"]
            pub const USDHC1_PODF_6: u32 = 0x06;
            #[doc = "divide by 8"]
            pub const USDHC1_PODF_7: u32 = 0x07;
        }
    }
    #[doc = "Divider for usdhc2 clock. Divider should be updated when output clock is gated."]
    pub mod USDHC2_PODF {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "divide by 1"]
            pub const USDHC2_PODF_0: u32 = 0;
            #[doc = "divide by 2"]
            pub const USDHC2_PODF_1: u32 = 0x01;
            #[doc = "divide by 3"]
            pub const USDHC2_PODF_2: u32 = 0x02;
            #[doc = "divide by 4"]
            pub const USDHC2_PODF_3: u32 = 0x03;
            #[doc = "divide by 5"]
            pub const USDHC2_PODF_4: u32 = 0x04;
            #[doc = "divide by 6"]
            pub const USDHC2_PODF_5: u32 = 0x05;
            #[doc = "divide by 7"]
            pub const USDHC2_PODF_6: u32 = 0x06;
            #[doc = "divide by 8"]
            pub const USDHC2_PODF_7: u32 = 0x07;
        }
    }
    #[doc = "Divider for trace clock. Divider should be updated when output clock is gated."]
    pub mod TRACE_PODF {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "divide by 1"]
            pub const TRACE_PODF_0: u32 = 0;
            #[doc = "divide by 2"]
            pub const TRACE_PODF_1: u32 = 0x01;
            #[doc = "divide by 3"]
            pub const TRACE_PODF_2: u32 = 0x02;
            #[doc = "divide by 4"]
            pub const TRACE_PODF_3: u32 = 0x03;
        }
    }
}
#[doc = "CCM Clock Divider Register"]
pub mod CS1CDR {
    #[doc = "Divider for sai1 clock podf. The input clock to this divider should be lower than 300Mhz, the predivider can be used to achieve this."]
    pub mod SAI1_CLK_PODF {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Divide by 1"]
            pub const DIVIDE_1: u32 = 0;
            #[doc = "Divide by 2"]
            pub const DIVIDE_2: u32 = 0x01;
            #[doc = "Divide by 3"]
            pub const DIVIDE_3: u32 = 0x02;
            #[doc = "Divide by 4"]
            pub const DIVIDE_4: u32 = 0x03;
            #[doc = "Divide by 5"]
            pub const DIVIDE_5: u32 = 0x04;
            #[doc = "Divide by 6"]
            pub const DIVIDE_6: u32 = 0x05;
            #[doc = "Divide by 7"]
            pub const DIVIDE_7: u32 = 0x06;
            #[doc = "Divide by 8"]
            pub const DIVIDE_8: u32 = 0x07;
            #[doc = "Divide by 9"]
            pub const DIVIDE_9: u32 = 0x08;
            #[doc = "Divide by 10"]
            pub const DIVIDE_10: u32 = 0x09;
            #[doc = "Divide by 11"]
            pub const DIVIDE_11: u32 = 0x0a;
            #[doc = "Divide by 12"]
            pub const DIVIDE_12: u32 = 0x0b;
            #[doc = "Divide by 13"]
            pub const DIVIDE_13: u32 = 0x0c;
            #[doc = "Divide by 14"]
            pub const DIVIDE_14: u32 = 0x0d;
            #[doc = "Divide by 15"]
            pub const DIVIDE_15: u32 = 0x0e;
            #[doc = "Divide by 16"]
            pub const DIVIDE_16: u32 = 0x0f;
            #[doc = "Divide by 17"]
            pub const DIVIDE_17: u32 = 0x10;
            #[doc = "Divide by 18"]
            pub const DIVIDE_18: u32 = 0x11;
            #[doc = "Divide by 19"]
            pub const DIVIDE_19: u32 = 0x12;
            #[doc = "Divide by 20"]
            pub const DIVIDE_20: u32 = 0x13;
            #[doc = "Divide by 21"]
            pub const DIVIDE_21: u32 = 0x14;
            #[doc = "Divide by 22"]
            pub const DIVIDE_22: u32 = 0x15;
            #[doc = "Divide by 23"]
            pub const DIVIDE_23: u32 = 0x16;
            #[doc = "Divide by 24"]
            pub const DIVIDE_24: u32 = 0x17;
            #[doc = "Divide by 25"]
            pub const DIVIDE_25: u32 = 0x18;
            #[doc = "Divide by 26"]
            pub const DIVIDE_26: u32 = 0x19;
            #[doc = "Divide by 27"]
            pub const DIVIDE_27: u32 = 0x1a;
            #[doc = "Divide by 28"]
            pub const DIVIDE_28: u32 = 0x1b;
            #[doc = "Divide by 29"]
            pub const DIVIDE_29: u32 = 0x1c;
            #[doc = "Divide by 30"]
            pub const DIVIDE_30: u32 = 0x1d;
            #[doc = "Divide by 31"]
            pub const DIVIDE_31: u32 = 0x1e;
            #[doc = "Divide by 32"]
            pub const DIVIDE_32: u32 = 0x1f;
            #[doc = "Divide by 33"]
            pub const DIVIDE_33: u32 = 0x20;
            #[doc = "Divide by 34"]
            pub const DIVIDE_34: u32 = 0x21;
            #[doc = "Divide by 35"]
            pub const DIVIDE_35: u32 = 0x22;
            #[doc = "Divide by 36"]
            pub const DIVIDE_36: u32 = 0x23;
            #[doc = "Divide by 37"]
            pub const DIVIDE_37: u32 = 0x24;
            #[doc = "Divide by 38"]
            pub const DIVIDE_38: u32 = 0x25;
            #[doc = "Divide by 39"]
            pub const DIVIDE_39: u32 = 0x26;
            #[doc = "Divide by 40"]
            pub const DIVIDE_40: u32 = 0x27;
            #[doc = "Divide by 41"]
            pub const DIVIDE_41: u32 = 0x28;
            #[doc = "Divide by 42"]
            pub const DIVIDE_42: u32 = 0x29;
            #[doc = "Divide by 43"]
            pub const DIVIDE_43: u32 = 0x2a;
            #[doc = "Divide by 44"]
            pub const DIVIDE_44: u32 = 0x2b;
            #[doc = "Divide by 45"]
            pub const DIVIDE_45: u32 = 0x2c;
            #[doc = "Divide by 46"]
            pub const DIVIDE_46: u32 = 0x2d;
            #[doc = "Divide by 47"]
            pub const DIVIDE_47: u32 = 0x2e;
            #[doc = "Divide by 48"]
            pub const DIVIDE_48: u32 = 0x2f;
            #[doc = "Divide by 49"]
            pub const DIVIDE_49: u32 = 0x30;
            #[doc = "Divide by 50"]
            pub const DIVIDE_50: u32 = 0x31;
            #[doc = "Divide by 51"]
            pub const DIVIDE_51: u32 = 0x32;
            #[doc = "Divide by 52"]
            pub const DIVIDE_52: u32 = 0x33;
            #[doc = "Divide by 53"]
            pub const DIVIDE_53: u32 = 0x34;
            #[doc = "Divide by 54"]
            pub const DIVIDE_54: u32 = 0x35;
            #[doc = "Divide by 55"]
            pub const DIVIDE_55: u32 = 0x36;
            #[doc = "Divide by 56"]
            pub const DIVIDE_56: u32 = 0x37;
            #[doc = "Divide by 57"]
            pub const DIVIDE_57: u32 = 0x38;
            #[doc = "Divide by 58"]
            pub const DIVIDE_58: u32 = 0x39;
            #[doc = "Divide by 59"]
            pub const DIVIDE_59: u32 = 0x3a;
            #[doc = "Divide by 60"]
            pub const DIVIDE_60: u32 = 0x3b;
            #[doc = "Divide by 61"]
            pub const DIVIDE_61: u32 = 0x3c;
            #[doc = "Divide by 62"]
            pub const DIVIDE_62: u32 = 0x3d;
            #[doc = "Divide by 63"]
            pub const DIVIDE_63: u32 = 0x3e;
            #[doc = "Divide by 64"]
            pub const DIVIDE_64: u32 = 0x3f;
        }
    }
    #[doc = "Divider for sai1 clock pred."]
    pub mod SAI1_CLK_PRED {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "divide by 1"]
            pub const SAI1_CLK_PRED_0: u32 = 0;
            #[doc = "divide by 2"]
            pub const SAI1_CLK_PRED_1: u32 = 0x01;
            #[doc = "divide by 3"]
            pub const SAI1_CLK_PRED_2: u32 = 0x02;
            #[doc = "divide by 4"]
            pub const SAI1_CLK_PRED_3: u32 = 0x03;
            #[doc = "divide by 5"]
            pub const SAI1_CLK_PRED_4: u32 = 0x04;
            #[doc = "divide by 6"]
            pub const SAI1_CLK_PRED_5: u32 = 0x05;
            #[doc = "divide by 7"]
            pub const SAI1_CLK_PRED_6: u32 = 0x06;
            #[doc = "divide by 8"]
            pub const SAI1_CLK_PRED_7: u32 = 0x07;
        }
    }
    #[doc = "Divider for flexio2 clock."]
    pub mod FLEXIO2_CLK_PRED {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "divide by 1"]
            pub const FLEXIO2_CLK_PRED_0: u32 = 0;
            #[doc = "divide by 2"]
            pub const FLEXIO2_CLK_PRED_1: u32 = 0x01;
            #[doc = "divide by 3"]
            pub const FLEXIO2_CLK_PRED_2: u32 = 0x02;
            #[doc = "divide by 4"]
            pub const FLEXIO2_CLK_PRED_3: u32 = 0x03;
            #[doc = "divide by 5"]
            pub const FLEXIO2_CLK_PRED_4: u32 = 0x04;
            #[doc = "divide by 6"]
            pub const FLEXIO2_CLK_PRED_5: u32 = 0x05;
            #[doc = "divide by 7"]
            pub const FLEXIO2_CLK_PRED_6: u32 = 0x06;
            #[doc = "divide by 8"]
            pub const FLEXIO2_CLK_PRED_7: u32 = 0x07;
        }
    }
    #[doc = "Divider for sai3 clock podf. The input clock to this divider should be lower than 300Mhz, the predivider can be used to achieve this."]
    pub mod SAI3_CLK_PODF {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Divide by 1"]
            pub const DIVIDE_1: u32 = 0;
            #[doc = "Divide by 2"]
            pub const DIVIDE_2: u32 = 0x01;
            #[doc = "Divide by 3"]
            pub const DIVIDE_3: u32 = 0x02;
            #[doc = "Divide by 4"]
            pub const DIVIDE_4: u32 = 0x03;
            #[doc = "Divide by 5"]
            pub const DIVIDE_5: u32 = 0x04;
            #[doc = "Divide by 6"]
            pub const DIVIDE_6: u32 = 0x05;
            #[doc = "Divide by 7"]
            pub const DIVIDE_7: u32 = 0x06;
            #[doc = "Divide by 8"]
            pub const DIVIDE_8: u32 = 0x07;
            #[doc = "Divide by 9"]
            pub const DIVIDE_9: u32 = 0x08;
            #[doc = "Divide by 10"]
            pub const DIVIDE_10: u32 = 0x09;
            #[doc = "Divide by 11"]
            pub const DIVIDE_11: u32 = 0x0a;
            #[doc = "Divide by 12"]
            pub const DIVIDE_12: u32 = 0x0b;
            #[doc = "Divide by 13"]
            pub const DIVIDE_13: u32 = 0x0c;
            #[doc = "Divide by 14"]
            pub const DIVIDE_14: u32 = 0x0d;
            #[doc = "Divide by 15"]
            pub const DIVIDE_15: u32 = 0x0e;
            #[doc = "Divide by 16"]
            pub const DIVIDE_16: u32 = 0x0f;
            #[doc = "Divide by 17"]
            pub const DIVIDE_17: u32 = 0x10;
            #[doc = "Divide by 18"]
            pub const DIVIDE_18: u32 = 0x11;
            #[doc = "Divide by 19"]
            pub const DIVIDE_19: u32 = 0x12;
            #[doc = "Divide by 20"]
            pub const DIVIDE_20: u32 = 0x13;
            #[doc = "Divide by 21"]
            pub const DIVIDE_21: u32 = 0x14;
            #[doc = "Divide by 22"]
            pub const DIVIDE_22: u32 = 0x15;
            #[doc = "Divide by 23"]
            pub const DIVIDE_23: u32 = 0x16;
            #[doc = "Divide by 24"]
            pub const DIVIDE_24: u32 = 0x17;
            #[doc = "Divide by 25"]
            pub const DIVIDE_25: u32 = 0x18;
            #[doc = "Divide by 26"]
            pub const DIVIDE_26: u32 = 0x19;
            #[doc = "Divide by 27"]
            pub const DIVIDE_27: u32 = 0x1a;
            #[doc = "Divide by 28"]
            pub const DIVIDE_28: u32 = 0x1b;
            #[doc = "Divide by 29"]
            pub const DIVIDE_29: u32 = 0x1c;
            #[doc = "Divide by 30"]
            pub const DIVIDE_30: u32 = 0x1d;
            #[doc = "Divide by 31"]
            pub const DIVIDE_31: u32 = 0x1e;
            #[doc = "Divide by 32"]
            pub const DIVIDE_32: u32 = 0x1f;
            #[doc = "Divide by 33"]
            pub const DIVIDE_33: u32 = 0x20;
            #[doc = "Divide by 34"]
            pub const DIVIDE_34: u32 = 0x21;
            #[doc = "Divide by 35"]
            pub const DIVIDE_35: u32 = 0x22;
            #[doc = "Divide by 36"]
            pub const DIVIDE_36: u32 = 0x23;
            #[doc = "Divide by 37"]
            pub const DIVIDE_37: u32 = 0x24;
            #[doc = "Divide by 38"]
            pub const DIVIDE_38: u32 = 0x25;
            #[doc = "Divide by 39"]
            pub const DIVIDE_39: u32 = 0x26;
            #[doc = "Divide by 40"]
            pub const DIVIDE_40: u32 = 0x27;
            #[doc = "Divide by 41"]
            pub const DIVIDE_41: u32 = 0x28;
            #[doc = "Divide by 42"]
            pub const DIVIDE_42: u32 = 0x29;
            #[doc = "Divide by 43"]
            pub const DIVIDE_43: u32 = 0x2a;
            #[doc = "Divide by 44"]
            pub const DIVIDE_44: u32 = 0x2b;
            #[doc = "Divide by 45"]
            pub const DIVIDE_45: u32 = 0x2c;
            #[doc = "Divide by 46"]
            pub const DIVIDE_46: u32 = 0x2d;
            #[doc = "Divide by 47"]
            pub const DIVIDE_47: u32 = 0x2e;
            #[doc = "Divide by 48"]
            pub const DIVIDE_48: u32 = 0x2f;
            #[doc = "Divide by 49"]
            pub const DIVIDE_49: u32 = 0x30;
            #[doc = "Divide by 50"]
            pub const DIVIDE_50: u32 = 0x31;
            #[doc = "Divide by 51"]
            pub const DIVIDE_51: u32 = 0x32;
            #[doc = "Divide by 52"]
            pub const DIVIDE_52: u32 = 0x33;
            #[doc = "Divide by 53"]
            pub const DIVIDE_53: u32 = 0x34;
            #[doc = "Divide by 54"]
            pub const DIVIDE_54: u32 = 0x35;
            #[doc = "Divide by 55"]
            pub const DIVIDE_55: u32 = 0x36;
            #[doc = "Divide by 56"]
            pub const DIVIDE_56: u32 = 0x37;
            #[doc = "Divide by 57"]
            pub const DIVIDE_57: u32 = 0x38;
            #[doc = "Divide by 58"]
            pub const DIVIDE_58: u32 = 0x39;
            #[doc = "Divide by 59"]
            pub const DIVIDE_59: u32 = 0x3a;
            #[doc = "Divide by 60"]
            pub const DIVIDE_60: u32 = 0x3b;
            #[doc = "Divide by 61"]
            pub const DIVIDE_61: u32 = 0x3c;
            #[doc = "Divide by 62"]
            pub const DIVIDE_62: u32 = 0x3d;
            #[doc = "Divide by 63"]
            pub const DIVIDE_63: u32 = 0x3e;
            #[doc = "Divide by 64"]
            pub const DIVIDE_64: u32 = 0x3f;
        }
    }
    #[doc = "Divider for sai3 clock pred."]
    pub mod SAI3_CLK_PRED {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "divide by 1"]
            pub const SAI3_CLK_PRED_0: u32 = 0;
            #[doc = "divide by 2"]
            pub const SAI3_CLK_PRED_1: u32 = 0x01;
            #[doc = "divide by 3"]
            pub const SAI3_CLK_PRED_2: u32 = 0x02;
            #[doc = "divide by 4"]
            pub const SAI3_CLK_PRED_3: u32 = 0x03;
            #[doc = "divide by 5"]
            pub const SAI3_CLK_PRED_4: u32 = 0x04;
            #[doc = "divide by 6"]
            pub const SAI3_CLK_PRED_5: u32 = 0x05;
            #[doc = "divide by 7"]
            pub const SAI3_CLK_PRED_6: u32 = 0x06;
            #[doc = "divide by 8"]
            pub const SAI3_CLK_PRED_7: u32 = 0x07;
        }
    }
    #[doc = "Divider for flexio2 clock. Divider should be updated when output clock is gated."]
    pub mod FLEXIO2_CLK_PODF {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Divide by 1"]
            pub const DIVIDE_1: u32 = 0;
            #[doc = "Divide by 2"]
            pub const DIVIDE_2: u32 = 0x01;
            #[doc = "Divide by 3"]
            pub const DIVIDE_3: u32 = 0x02;
            #[doc = "Divide by 4"]
            pub const DIVIDE_4: u32 = 0x03;
            #[doc = "Divide by 5"]
            pub const DIVIDE_5: u32 = 0x04;
            #[doc = "Divide by 6"]
            pub const DIVIDE_6: u32 = 0x05;
            #[doc = "Divide by 7"]
            pub const DIVIDE_7: u32 = 0x06;
            #[doc = "Divide by 8"]
            pub const DIVIDE_8: u32 = 0x07;
        }
    }
}
#[doc = "CCM Clock Divider Register"]
pub mod CS2CDR {
    #[doc = "Divider for sai2 clock podf. The input clock to this divider should be lower than 300Mhz, the predivider can be used to achieve this."]
    pub mod SAI2_CLK_PODF {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Divide by 1"]
            pub const DIVIDE_1: u32 = 0;
            #[doc = "Divide by 2"]
            pub const DIVIDE_2: u32 = 0x01;
            #[doc = "Divide by 3"]
            pub const DIVIDE_3: u32 = 0x02;
            #[doc = "Divide by 4"]
            pub const DIVIDE_4: u32 = 0x03;
            #[doc = "Divide by 5"]
            pub const DIVIDE_5: u32 = 0x04;
            #[doc = "Divide by 6"]
            pub const DIVIDE_6: u32 = 0x05;
            #[doc = "Divide by 7"]
            pub const DIVIDE_7: u32 = 0x06;
            #[doc = "Divide by 8"]
            pub const DIVIDE_8: u32 = 0x07;
            #[doc = "Divide by 9"]
            pub const DIVIDE_9: u32 = 0x08;
            #[doc = "Divide by 10"]
            pub const DIVIDE_10: u32 = 0x09;
            #[doc = "Divide by 11"]
            pub const DIVIDE_11: u32 = 0x0a;
            #[doc = "Divide by 12"]
            pub const DIVIDE_12: u32 = 0x0b;
            #[doc = "Divide by 13"]
            pub const DIVIDE_13: u32 = 0x0c;
            #[doc = "Divide by 14"]
            pub const DIVIDE_14: u32 = 0x0d;
            #[doc = "Divide by 15"]
            pub const DIVIDE_15: u32 = 0x0e;
            #[doc = "Divide by 16"]
            pub const DIVIDE_16: u32 = 0x0f;
            #[doc = "Divide by 17"]
            pub const DIVIDE_17: u32 = 0x10;
            #[doc = "Divide by 18"]
            pub const DIVIDE_18: u32 = 0x11;
            #[doc = "Divide by 19"]
            pub const DIVIDE_19: u32 = 0x12;
            #[doc = "Divide by 20"]
            pub const DIVIDE_20: u32 = 0x13;
            #[doc = "Divide by 21"]
            pub const DIVIDE_21: u32 = 0x14;
            #[doc = "Divide by 22"]
            pub const DIVIDE_22: u32 = 0x15;
            #[doc = "Divide by 23"]
            pub const DIVIDE_23: u32 = 0x16;
            #[doc = "Divide by 24"]
            pub const DIVIDE_24: u32 = 0x17;
            #[doc = "Divide by 25"]
            pub const DIVIDE_25: u32 = 0x18;
            #[doc = "Divide by 26"]
            pub const DIVIDE_26: u32 = 0x19;
            #[doc = "Divide by 27"]
            pub const DIVIDE_27: u32 = 0x1a;
            #[doc = "Divide by 28"]
            pub const DIVIDE_28: u32 = 0x1b;
            #[doc = "Divide by 29"]
            pub const DIVIDE_29: u32 = 0x1c;
            #[doc = "Divide by 30"]
            pub const DIVIDE_30: u32 = 0x1d;
            #[doc = "Divide by 31"]
            pub const DIVIDE_31: u32 = 0x1e;
            #[doc = "Divide by 32"]
            pub const DIVIDE_32: u32 = 0x1f;
            #[doc = "Divide by 33"]
            pub const DIVIDE_33: u32 = 0x20;
            #[doc = "Divide by 34"]
            pub const DIVIDE_34: u32 = 0x21;
            #[doc = "Divide by 35"]
            pub const DIVIDE_35: u32 = 0x22;
            #[doc = "Divide by 36"]
            pub const DIVIDE_36: u32 = 0x23;
            #[doc = "Divide by 37"]
            pub const DIVIDE_37: u32 = 0x24;
            #[doc = "Divide by 38"]
            pub const DIVIDE_38: u32 = 0x25;
            #[doc = "Divide by 39"]
            pub const DIVIDE_39: u32 = 0x26;
            #[doc = "Divide by 40"]
            pub const DIVIDE_40: u32 = 0x27;
            #[doc = "Divide by 41"]
            pub const DIVIDE_41: u32 = 0x28;
            #[doc = "Divide by 42"]
            pub const DIVIDE_42: u32 = 0x29;
            #[doc = "Divide by 43"]
            pub const DIVIDE_43: u32 = 0x2a;
            #[doc = "Divide by 44"]
            pub const DIVIDE_44: u32 = 0x2b;
            #[doc = "Divide by 45"]
            pub const DIVIDE_45: u32 = 0x2c;
            #[doc = "Divide by 46"]
            pub const DIVIDE_46: u32 = 0x2d;
            #[doc = "Divide by 47"]
            pub const DIVIDE_47: u32 = 0x2e;
            #[doc = "Divide by 48"]
            pub const DIVIDE_48: u32 = 0x2f;
            #[doc = "Divide by 49"]
            pub const DIVIDE_49: u32 = 0x30;
            #[doc = "Divide by 50"]
            pub const DIVIDE_50: u32 = 0x31;
            #[doc = "Divide by 51"]
            pub const DIVIDE_51: u32 = 0x32;
            #[doc = "Divide by 52"]
            pub const DIVIDE_52: u32 = 0x33;
            #[doc = "Divide by 53"]
            pub const DIVIDE_53: u32 = 0x34;
            #[doc = "Divide by 54"]
            pub const DIVIDE_54: u32 = 0x35;
            #[doc = "Divide by 55"]
            pub const DIVIDE_55: u32 = 0x36;
            #[doc = "Divide by 56"]
            pub const DIVIDE_56: u32 = 0x37;
            #[doc = "Divide by 57"]
            pub const DIVIDE_57: u32 = 0x38;
            #[doc = "Divide by 58"]
            pub const DIVIDE_58: u32 = 0x39;
            #[doc = "Divide by 59"]
            pub const DIVIDE_59: u32 = 0x3a;
            #[doc = "Divide by 60"]
            pub const DIVIDE_60: u32 = 0x3b;
            #[doc = "Divide by 61"]
            pub const DIVIDE_61: u32 = 0x3c;
            #[doc = "Divide by 62"]
            pub const DIVIDE_62: u32 = 0x3d;
            #[doc = "Divide by 63"]
            pub const DIVIDE_63: u32 = 0x3e;
            #[doc = "Divide by 64"]
            pub const DIVIDE_64: u32 = 0x3f;
        }
    }
    #[doc = "Divider for sai2 clock pred.Divider should be updated when output clock is gated."]
    pub mod SAI2_CLK_PRED {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "divide by 1"]
            pub const SAI2_CLK_PRED_0: u32 = 0;
            #[doc = "divide by 2"]
            pub const SAI2_CLK_PRED_1: u32 = 0x01;
            #[doc = "divide by 3"]
            pub const SAI2_CLK_PRED_2: u32 = 0x02;
            #[doc = "divide by 4"]
            pub const SAI2_CLK_PRED_3: u32 = 0x03;
            #[doc = "divide by 5"]
            pub const SAI2_CLK_PRED_4: u32 = 0x04;
            #[doc = "divide by 6"]
            pub const SAI2_CLK_PRED_5: u32 = 0x05;
            #[doc = "divide by 7"]
            pub const SAI2_CLK_PRED_6: u32 = 0x06;
            #[doc = "divide by 8"]
            pub const SAI2_CLK_PRED_7: u32 = 0x07;
        }
    }
}
#[doc = "CCM D1 Clock Divider Register"]
pub mod CDCDR {
    #[doc = "Selector for flexio1 clock multiplexer"]
    pub mod FLEXIO1_CLK_SEL {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "derive clock from PLL4 divided clock"]
            pub const FLEXIO1_CLK_SEL_0: u32 = 0;
            #[doc = "derive clock from PLL3 PFD2 clock"]
            pub const FLEXIO1_CLK_SEL_1: u32 = 0x01;
            #[doc = "derive from PLL2"]
            pub const FLEXIO1_CLK_SEL_2: u32 = 0x02;
            #[doc = "derive clock from pll3_sw_clk"]
            pub const FLEXIO1_CLK_SEL_3: u32 = 0x03;
        }
    }
    #[doc = "Divider for flexio1 clock podf. Divider should be updated when output clock is gated."]
    pub mod FLEXIO1_CLK_PODF {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Divide by 1"]
            pub const DIVIDE_1: u32 = 0;
            #[doc = "Divide by 2"]
            pub const DIVIDE_2: u32 = 0x01;
            #[doc = "Divide by 3"]
            pub const DIVIDE_3: u32 = 0x02;
            #[doc = "Divide by 4"]
            pub const DIVIDE_4: u32 = 0x03;
            #[doc = "Divide by 5"]
            pub const DIVIDE_5: u32 = 0x04;
            #[doc = "Divide by 6"]
            pub const DIVIDE_6: u32 = 0x05;
            #[doc = "Divide by 7"]
            pub const DIVIDE_7: u32 = 0x06;
            #[doc = "Divide by 8"]
            pub const DIVIDE_8: u32 = 0x07;
        }
    }
    #[doc = "Divider for flexio1 clock pred. Divider should be updated when output clock is gated."]
    pub mod FLEXIO1_CLK_PRED {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "divide by 1"]
            pub const FLEXIO1_CLK_PRED_0: u32 = 0;
            #[doc = "divide by 2"]
            pub const FLEXIO1_CLK_PRED_1: u32 = 0x01;
            #[doc = "divide by 3"]
            pub const FLEXIO1_CLK_PRED_2: u32 = 0x02;
            #[doc = "divide by 4"]
            pub const FLEXIO1_CLK_PRED_3: u32 = 0x03;
            #[doc = "divide by 5"]
            pub const FLEXIO1_CLK_PRED_4: u32 = 0x04;
            #[doc = "divide by 6"]
            pub const FLEXIO1_CLK_PRED_5: u32 = 0x05;
            #[doc = "divide by 7"]
            pub const FLEXIO1_CLK_PRED_6: u32 = 0x06;
            #[doc = "divide by 8"]
            pub const FLEXIO1_CLK_PRED_7: u32 = 0x07;
        }
    }
    #[doc = "Selector for spdif0 clock multiplexer"]
    pub mod SPDIF0_CLK_SEL {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "derive clock from PLL4"]
            pub const SPDIF0_CLK_SEL_0: u32 = 0;
            #[doc = "derive clock from PLL3 PFD2"]
            pub const SPDIF0_CLK_SEL_1: u32 = 0x01;
            #[doc = "derive clock from PLL5"]
            pub const SPDIF0_CLK_SEL_2: u32 = 0x02;
            #[doc = "derive clock from pll3_sw_clk"]
            pub const SPDIF0_CLK_SEL_3: u32 = 0x03;
        }
    }
    #[doc = "Divider for spdif0 clock podf. Divider should be updated when output clock is gated."]
    pub mod SPDIF0_CLK_PODF {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Divide by 1"]
            pub const DIVIDE_1: u32 = 0;
            #[doc = "Divide by 2"]
            pub const DIVIDE_2: u32 = 0x01;
            #[doc = "Divide by 3"]
            pub const DIVIDE_3: u32 = 0x02;
            #[doc = "Divide by 4"]
            pub const DIVIDE_4: u32 = 0x03;
            #[doc = "Divide by 5"]
            pub const DIVIDE_5: u32 = 0x04;
            #[doc = "Divide by 6"]
            pub const DIVIDE_6: u32 = 0x05;
            #[doc = "Divide by 7"]
            pub const DIVIDE_7: u32 = 0x06;
            #[doc = "Divide by 8"]
            pub const DIVIDE_8: u32 = 0x07;
        }
    }
    #[doc = "Divider for spdif0 clock pred. Divider should be updated when output clock is gated."]
    pub mod SPDIF0_CLK_PRED {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Divide by 1"]
            pub const DIVIDE_1: u32 = 0;
            #[doc = "Divide by 2"]
            pub const DIVIDE_2: u32 = 0x01;
            #[doc = "Divide by 3"]
            pub const DIVIDE_3: u32 = 0x02;
            #[doc = "Divide by 4"]
            pub const DIVIDE_4: u32 = 0x03;
            #[doc = "Divide by 5"]
            pub const DIVIDE_5: u32 = 0x04;
            #[doc = "Divide by 6"]
            pub const DIVIDE_6: u32 = 0x05;
            #[doc = "Divide by 7"]
            pub const DIVIDE_7: u32 = 0x06;
            #[doc = "Divide by 8"]
            pub const DIVIDE_8: u32 = 0x07;
        }
    }
}
#[doc = "CCM Serial Clock Divider Register 2"]
pub mod CSCDR2 {
    #[doc = "Pre-divider for lcdif clock. Divider should be updated when output clock is gated."]
    pub mod LCDIF_PRED {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "divide by 1"]
            pub const LCDIF_PRED_0: u32 = 0;
            #[doc = "divide by 2"]
            pub const LCDIF_PRED_1: u32 = 0x01;
            #[doc = "divide by 3"]
            pub const LCDIF_PRED_2: u32 = 0x02;
            #[doc = "divide by 4"]
            pub const LCDIF_PRED_3: u32 = 0x03;
            #[doc = "divide by 5"]
            pub const LCDIF_PRED_4: u32 = 0x04;
            #[doc = "divide by 6"]
            pub const LCDIF_PRED_5: u32 = 0x05;
            #[doc = "divide by 7"]
            pub const LCDIF_PRED_6: u32 = 0x06;
            #[doc = "divide by 8"]
            pub const LCDIF_PRED_7: u32 = 0x07;
        }
    }
    #[doc = "Selector for lcdif root clock pre-multiplexer"]
    pub mod LCDIF_PRE_CLK_SEL {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "derive clock from PLL2"]
            pub const LCDIF_PRE_CLK_SEL_0: u32 = 0;
            #[doc = "derive clock from PLL3 PFD3"]
            pub const LCDIF_PRE_CLK_SEL_1: u32 = 0x01;
            #[doc = "derive clock from PLL5"]
            pub const LCDIF_PRE_CLK_SEL_2: u32 = 0x02;
            #[doc = "derive clock from PLL2 PFD0"]
            pub const LCDIF_PRE_CLK_SEL_3: u32 = 0x03;
            #[doc = "derive clock from PLL2 PFD1"]
            pub const LCDIF_PRE_CLK_SEL_4: u32 = 0x04;
            #[doc = "derive clock from PLL3 PFD1"]
            pub const LCDIF_PRE_CLK_SEL_5: u32 = 0x05;
        }
    }
    #[doc = "Selector for the LPI2C clock multiplexor"]
    pub mod LPI2C_CLK_SEL {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "derive clock from pll3_60m"]
            pub const LPI2C_CLK_SEL_0: u32 = 0;
            #[doc = "derive clock from osc_clk"]
            pub const LPI2C_CLK_SEL_1: u32 = 0x01;
        }
    }
    #[doc = "Divider for lpi2c clock podf. Divider should be updated when output clock is gated. The input clock to this divider should be lower than 300Mhz, the predivider can be used to achieve this."]
    pub mod LPI2C_CLK_PODF {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Divide by 1"]
            pub const DIVIDE_1: u32 = 0;
            #[doc = "Divide by 2"]
            pub const DIVIDE_2: u32 = 0x01;
            #[doc = "Divide by 3"]
            pub const DIVIDE_3: u32 = 0x02;
            #[doc = "Divide by 4"]
            pub const DIVIDE_4: u32 = 0x03;
            #[doc = "Divide by 5"]
            pub const DIVIDE_5: u32 = 0x04;
            #[doc = "Divide by 6"]
            pub const DIVIDE_6: u32 = 0x05;
            #[doc = "Divide by 7"]
            pub const DIVIDE_7: u32 = 0x06;
            #[doc = "Divide by 8"]
            pub const DIVIDE_8: u32 = 0x07;
            #[doc = "Divide by 9"]
            pub const DIVIDE_9: u32 = 0x08;
            #[doc = "Divide by 10"]
            pub const DIVIDE_10: u32 = 0x09;
            #[doc = "Divide by 11"]
            pub const DIVIDE_11: u32 = 0x0a;
            #[doc = "Divide by 12"]
            pub const DIVIDE_12: u32 = 0x0b;
            #[doc = "Divide by 13"]
            pub const DIVIDE_13: u32 = 0x0c;
            #[doc = "Divide by 14"]
            pub const DIVIDE_14: u32 = 0x0d;
            #[doc = "Divide by 15"]
            pub const DIVIDE_15: u32 = 0x0e;
            #[doc = "Divide by 16"]
            pub const DIVIDE_16: u32 = 0x0f;
            #[doc = "Divide by 17"]
            pub const DIVIDE_17: u32 = 0x10;
            #[doc = "Divide by 18"]
            pub const DIVIDE_18: u32 = 0x11;
            #[doc = "Divide by 19"]
            pub const DIVIDE_19: u32 = 0x12;
            #[doc = "Divide by 20"]
            pub const DIVIDE_20: u32 = 0x13;
            #[doc = "Divide by 21"]
            pub const DIVIDE_21: u32 = 0x14;
            #[doc = "Divide by 22"]
            pub const DIVIDE_22: u32 = 0x15;
            #[doc = "Divide by 23"]
            pub const DIVIDE_23: u32 = 0x16;
            #[doc = "Divide by 24"]
            pub const DIVIDE_24: u32 = 0x17;
            #[doc = "Divide by 25"]
            pub const DIVIDE_25: u32 = 0x18;
            #[doc = "Divide by 26"]
            pub const DIVIDE_26: u32 = 0x19;
            #[doc = "Divide by 27"]
            pub const DIVIDE_27: u32 = 0x1a;
            #[doc = "Divide by 28"]
            pub const DIVIDE_28: u32 = 0x1b;
            #[doc = "Divide by 29"]
            pub const DIVIDE_29: u32 = 0x1c;
            #[doc = "Divide by 30"]
            pub const DIVIDE_30: u32 = 0x1d;
            #[doc = "Divide by 31"]
            pub const DIVIDE_31: u32 = 0x1e;
            #[doc = "Divide by 32"]
            pub const DIVIDE_32: u32 = 0x1f;
            #[doc = "Divide by 33"]
            pub const DIVIDE_33: u32 = 0x20;
            #[doc = "Divide by 34"]
            pub const DIVIDE_34: u32 = 0x21;
            #[doc = "Divide by 35"]
            pub const DIVIDE_35: u32 = 0x22;
            #[doc = "Divide by 36"]
            pub const DIVIDE_36: u32 = 0x23;
            #[doc = "Divide by 37"]
            pub const DIVIDE_37: u32 = 0x24;
            #[doc = "Divide by 38"]
            pub const DIVIDE_38: u32 = 0x25;
            #[doc = "Divide by 39"]
            pub const DIVIDE_39: u32 = 0x26;
            #[doc = "Divide by 40"]
            pub const DIVIDE_40: u32 = 0x27;
            #[doc = "Divide by 41"]
            pub const DIVIDE_41: u32 = 0x28;
            #[doc = "Divide by 42"]
            pub const DIVIDE_42: u32 = 0x29;
            #[doc = "Divide by 43"]
            pub const DIVIDE_43: u32 = 0x2a;
            #[doc = "Divide by 44"]
            pub const DIVIDE_44: u32 = 0x2b;
            #[doc = "Divide by 45"]
            pub const DIVIDE_45: u32 = 0x2c;
            #[doc = "Divide by 46"]
            pub const DIVIDE_46: u32 = 0x2d;
            #[doc = "Divide by 47"]
            pub const DIVIDE_47: u32 = 0x2e;
            #[doc = "Divide by 48"]
            pub const DIVIDE_48: u32 = 0x2f;
            #[doc = "Divide by 49"]
            pub const DIVIDE_49: u32 = 0x30;
            #[doc = "Divide by 50"]
            pub const DIVIDE_50: u32 = 0x31;
            #[doc = "Divide by 51"]
            pub const DIVIDE_51: u32 = 0x32;
            #[doc = "Divide by 52"]
            pub const DIVIDE_52: u32 = 0x33;
            #[doc = "Divide by 53"]
            pub const DIVIDE_53: u32 = 0x34;
            #[doc = "Divide by 54"]
            pub const DIVIDE_54: u32 = 0x35;
            #[doc = "Divide by 55"]
            pub const DIVIDE_55: u32 = 0x36;
            #[doc = "Divide by 56"]
            pub const DIVIDE_56: u32 = 0x37;
            #[doc = "Divide by 57"]
            pub const DIVIDE_57: u32 = 0x38;
            #[doc = "Divide by 58"]
            pub const DIVIDE_58: u32 = 0x39;
            #[doc = "Divide by 59"]
            pub const DIVIDE_59: u32 = 0x3a;
            #[doc = "Divide by 60"]
            pub const DIVIDE_60: u32 = 0x3b;
            #[doc = "Divide by 61"]
            pub const DIVIDE_61: u32 = 0x3c;
            #[doc = "Divide by 62"]
            pub const DIVIDE_62: u32 = 0x3d;
            #[doc = "Divide by 63"]
            pub const DIVIDE_63: u32 = 0x3e;
            #[doc = "Divide by 64"]
            pub const DIVIDE_64: u32 = 0x3f;
        }
    }
}
#[doc = "CCM Serial Clock Divider Register 3"]
pub mod CSCDR3 {
    #[doc = "Selector for csi_mclk multiplexer"]
    pub mod CSI_CLK_SEL {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "derive clock from osc_clk (24M)"]
            pub const CSI_CLK_SEL_0: u32 = 0;
            #[doc = "derive clock from PLL2 PFD2"]
            pub const CSI_CLK_SEL_1: u32 = 0x01;
            #[doc = "derive clock from pll3_120M"]
            pub const CSI_CLK_SEL_2: u32 = 0x02;
            #[doc = "derive clock from PLL3 PFD1"]
            pub const CSI_CLK_SEL_3: u32 = 0x03;
        }
    }
    #[doc = "Post divider for csi_mclk. Divider should be updated when output clock is gated."]
    pub mod CSI_PODF {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "divide by 1"]
            pub const CSI_PODF_0: u32 = 0;
            #[doc = "divide by 2"]
            pub const CSI_PODF_1: u32 = 0x01;
            #[doc = "divide by 3"]
            pub const CSI_PODF_2: u32 = 0x02;
            #[doc = "divide by 4"]
            pub const CSI_PODF_3: u32 = 0x03;
            #[doc = "divide by 5"]
            pub const CSI_PODF_4: u32 = 0x04;
            #[doc = "divide by 6"]
            pub const CSI_PODF_5: u32 = 0x05;
            #[doc = "divide by 7"]
            pub const CSI_PODF_6: u32 = 0x06;
            #[doc = "divide by 8"]
            pub const CSI_PODF_7: u32 = 0x07;
        }
    }
}
#[doc = "CCM Divider Handshake In-Process Register"]
pub mod CDHIPR {
    #[doc = "Busy indicator for semc_podf."]
    pub mod SEMC_PODF_BUSY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "divider is not busy and its value represents the actual division."]
            pub const SEMC_PODF_BUSY_0: u32 = 0;
            #[doc = "divider is busy with handshake process with module. The value read in the divider represents the previous value of the division factor, and after the handshake the written value of the semc_podf will be applied."]
            pub const SEMC_PODF_BUSY_1: u32 = 0x01;
        }
    }
    #[doc = "Busy indicator for ahb_podf."]
    pub mod AHB_PODF_BUSY {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "divider is not busy and its value represents the actual division."]
            pub const AHB_PODF_BUSY_0: u32 = 0;
            #[doc = "divider is busy with handshake process with module. The value read in the divider represents the previous value of the division factor, and after the handshake the written value of the ahb_podf will be applied."]
            pub const AHB_PODF_BUSY_1: u32 = 0x01;
        }
    }
    #[doc = "Busy indicator for periph2_clk_sel mux control."]
    pub mod PERIPH2_CLK_SEL_BUSY {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "mux is not busy and its value represents the actual division."]
            pub const PERIPH2_CLK_SEL_BUSY_0: u32 = 0;
            #[doc = "mux is busy with handshake process with module. The value read in the periph2_clk_sel represents the previous value of select, and after the handshake periph2_clk_sel value will be applied."]
            pub const PERIPH2_CLK_SEL_BUSY_1: u32 = 0x01;
        }
    }
    #[doc = "Busy indicator for periph_clk_sel mux control."]
    pub mod PERIPH_CLK_SEL_BUSY {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "mux is not busy and its value represents the actual division."]
            pub const PERIPH_CLK_SEL_BUSY_0: u32 = 0;
            #[doc = "mux is busy with handshake process with module. The value read in the periph_clk_sel represents the previous value of select, and after the handshake periph_clk_sel value will be applied."]
            pub const PERIPH_CLK_SEL_BUSY_1: u32 = 0x01;
        }
    }
    #[doc = "Busy indicator for arm_podf."]
    pub mod ARM_PODF_BUSY {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "divider is not busy and its value represents the actual division."]
            pub const ARM_PODF_BUSY_0: u32 = 0;
            #[doc = "divider is busy with handshake process with module. The value read in the divider represents the previous value of the division factor, and after the handshake the written value of the arm_podf will be applied."]
            pub const ARM_PODF_BUSY_1: u32 = 0x01;
        }
    }
}
#[doc = "CCM Low Power Control Register"]
pub mod CLPCR {
    #[doc = "Setting the low power mode that system will enter on next assertion of dsm_request signal."]
    pub mod LPM {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Remain in run mode"]
            pub const LPM_0: u32 = 0;
            #[doc = "Transfer to wait mode"]
            pub const LPM_1: u32 = 0x01;
            #[doc = "Transfer to stop mode"]
            pub const LPM_2: u32 = 0x02;
        }
    }
    #[doc = "Define if ARM clocks (arm_clk, soc_mxclk, soc_pclk, soc_dbg_pclk, vl_wrck) will be disabled on wait mode"]
    pub mod ARM_CLK_DIS_ON_LPM {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ARM clock enabled on wait mode."]
            pub const ARM_CLK_DIS_ON_LPM_0: u32 = 0;
            #[doc = "ARM clock disabled on wait mode. ."]
            pub const ARM_CLK_DIS_ON_LPM_1: u32 = 0x01;
        }
    }
    #[doc = "Standby clock oscillator bit"]
    pub mod SBYOS {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "On-chip oscillator will not be powered down, after next entrance to STOP mode. (CCM_REF_EN_B will remain asserted - '0' and cosc_pwrdown will remain de asserted - '0')"]
            pub const SBYOS_0: u32 = 0;
            #[doc = "On-chip oscillator will be powered down, after next entrance to STOP mode. (CCM_REF_EN_B will be deasserted - '1' and cosc_pwrdown will be asserted - '1'). When returning from STOP mode, external oscillator will be enabled again, on-chip oscillator will return to oscillator mode, and after oscnt count, CCM will continue with the exit from the STOP mode process."]
            pub const SBYOS_1: u32 = 0x01;
        }
    }
    #[doc = "dis_ref_osc - in run mode, software can manually control closing of external reference oscillator clock, i"]
    pub mod DIS_REF_OSC {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "external high frequency oscillator will be enabled, i.e. CCM_REF_EN_B = '0'."]
            pub const DIS_REF_OSC_0: u32 = 0;
            #[doc = "external high frequency oscillator will be disabled, i.e. CCM_REF_EN_B = '1'"]
            pub const DIS_REF_OSC_1: u32 = 0x01;
        }
    }
    #[doc = "Voltage standby request bit"]
    pub mod VSTBY {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Voltage will not be changed to standby voltage after next entrance to STOP mode. ( PMIC_STBY_REQ will remain negated - '0')"]
            pub const VSTBY_0: u32 = 0;
            #[doc = "Voltage will be requested to change to standby voltage after next entrance to stop mode. ( PMIC_STBY_REQ will be asserted - '1')."]
            pub const VSTBY_1: u32 = 0x01;
        }
    }
    #[doc = "Standby counter definition"]
    pub mod STBY_COUNT {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "CCM will wait (1*pmic_delay_scaler)+1 ckil clock cycles"]
            pub const STBY_COUNT_0: u32 = 0;
            #[doc = "CCM will wait (3*pmic_delay_scaler)+1 ckil clock cycles"]
            pub const STBY_COUNT_1: u32 = 0x01;
            #[doc = "CCM will wait (7*pmic_delay_scaler)+1 ckil clock cycles"]
            pub const STBY_COUNT_2: u32 = 0x02;
            #[doc = "CCM will wait (15*pmic_delay_scaler)+1 ckil clock cycles"]
            pub const STBY_COUNT_3: u32 = 0x03;
        }
    }
    #[doc = "In run mode, software can manually control powering down of on chip oscillator, i"]
    pub mod COSC_PWRDOWN {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "On chip oscillator will not be powered down, i.e. cosc_pwrdown = '0'."]
            pub const COSC_PWRDOWN_0: u32 = 0;
            #[doc = "On chip oscillator will be powered down, i.e. cosc_pwrdown = '1'."]
            pub const COSC_PWRDOWN_1: u32 = 0x01;
        }
    }
    #[doc = "Bypass low power mode handshake. This bit should always be set to 1'b1 by software."]
    pub mod BYPASS_LPM_HS1 {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Bypass low power mode handshake. This bit should always be set to 1'b1 by software."]
    pub mod BYPASS_LPM_HS0 {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Mask WFI of core0 for entering low power mode Assertion of all bits\\[27:22\\] will generate low power mode request"]
    pub mod MASK_CORE0_WFI {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "WFI of core0 is not masked"]
            pub const MASK_CORE0_WFI_0: u32 = 0;
            #[doc = "WFI of core0 is masked"]
            pub const MASK_CORE0_WFI_1: u32 = 0x01;
        }
    }
    #[doc = "Mask SCU IDLE for entering low power mode Assertion of all bits\\[27:22\\] will generate low power mode request"]
    pub mod MASK_SCU_IDLE {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "SCU IDLE is not masked"]
            pub const MASK_SCU_IDLE_0: u32 = 0;
            #[doc = "SCU IDLE is masked"]
            pub const MASK_SCU_IDLE_1: u32 = 0x01;
        }
    }
    #[doc = "Mask L2CC IDLE for entering low power mode"]
    pub mod MASK_L2CC_IDLE {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "L2CC IDLE is not masked"]
            pub const MASK_L2CC_IDLE_0: u32 = 0;
            #[doc = "L2CC IDLE is masked"]
            pub const MASK_L2CC_IDLE_1: u32 = 0x01;
        }
    }
}
#[doc = "CCM Interrupt Status Register"]
pub mod CISR {
    #[doc = "CCM interrupt request 2 generated due to lock of all enabled and not bypaseed PLLs"]
    pub mod LRF_PLL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "interrupt is not generated due to lock ready of all enabled and not bypaseed PLLs"]
            pub const LRF_PLL_0: u32 = 0;
            #[doc = "interrupt generated due to lock ready of all enabled and not bypaseed PLLs"]
            pub const LRF_PLL_1: u32 = 0x01;
        }
    }
    #[doc = "CCM interrupt request 2 generated due to on board oscillator ready, i"]
    pub mod COSC_READY {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "interrupt is not generated due to on board oscillator ready"]
            pub const COSC_READY_0: u32 = 0;
            #[doc = "interrupt generated due to on board oscillator ready"]
            pub const COSC_READY_1: u32 = 0x01;
        }
    }
    #[doc = "CCM interrupt request 1 generated due to frequency change of semc_podf"]
    pub mod SEMC_PODF_LOADED {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "interrupt is not generated due to frequency change of semc_podf"]
            pub const SEMC_PODF_LOADED_0: u32 = 0;
            #[doc = "interrupt generated due to frequency change of semc_podf"]
            pub const SEMC_PODF_LOADED_1: u32 = 0x01;
        }
    }
    #[doc = "CCM interrupt request 1 generated due to frequency change of periph2_clk_sel"]
    pub mod PERIPH2_CLK_SEL_LOADED {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "interrupt is not generated due to frequency change of periph2_clk_sel"]
            pub const PERIPH2_CLK_SEL_LOADED_0: u32 = 0;
            #[doc = "interrupt generated due to frequency change of periph2_clk_sel"]
            pub const PERIPH2_CLK_SEL_LOADED_1: u32 = 0x01;
        }
    }
    #[doc = "CCM interrupt request 1 generated due to frequency change of ahb_podf"]
    pub mod AHB_PODF_LOADED {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "interrupt is not generated due to frequency change of ahb_podf"]
            pub const AHB_PODF_LOADED_0: u32 = 0;
            #[doc = "interrupt generated due to frequency change of ahb_podf"]
            pub const AHB_PODF_LOADED_1: u32 = 0x01;
        }
    }
    #[doc = "CCM interrupt request 1 generated due to update of periph_clk_sel."]
    pub mod PERIPH_CLK_SEL_LOADED {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "interrupt is not generated due to update of periph_clk_sel."]
            pub const PERIPH_CLK_SEL_LOADED_0: u32 = 0;
            #[doc = "interrupt generated due to update of periph_clk_sel."]
            pub const PERIPH_CLK_SEL_LOADED_1: u32 = 0x01;
        }
    }
    #[doc = "CCM interrupt request 1 generated due to frequency change of arm_podf"]
    pub mod ARM_PODF_LOADED {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "interrupt is not generated due to frequency change of arm_podf"]
            pub const ARM_PODF_LOADED_0: u32 = 0;
            #[doc = "interrupt generated due to frequency change of arm_podf"]
            pub const ARM_PODF_LOADED_1: u32 = 0x01;
        }
    }
}
#[doc = "CCM Interrupt Mask Register"]
pub mod CIMR {
    #[doc = "mask interrupt generation due to lrf of PLLs"]
    pub mod MASK_LRF_PLL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "don't mask interrupt due to lrf of PLLs - interrupt will be created"]
            pub const MASK_LRF_PLL_0: u32 = 0;
            #[doc = "mask interrupt due to lrf of PLLs"]
            pub const MASK_LRF_PLL_1: u32 = 0x01;
        }
    }
    #[doc = "mask interrupt generation due to on board oscillator ready"]
    pub mod MASK_COSC_READY {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "don't mask interrupt due to on board oscillator ready - interrupt will be created"]
            pub const MASK_COSC_READY_0: u32 = 0;
            #[doc = "mask interrupt due to on board oscillator ready"]
            pub const MASK_COSC_READY_1: u32 = 0x01;
        }
    }
    #[doc = "mask interrupt generation due to frequency change of semc_podf"]
    pub mod MASK_SEMC_PODF_LOADED {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "don't mask interrupt due to frequency change of semc_podf - interrupt will be created"]
            pub const MASK_SEMC_PODF_LOADED_0: u32 = 0;
            #[doc = "mask interrupt due to frequency change of semc_podf"]
            pub const MASK_SEMC_PODF_LOADED_1: u32 = 0x01;
        }
    }
    #[doc = "mask interrupt generation due to update of periph2_clk_sel."]
    pub mod MASK_PERIPH2_CLK_SEL_LOADED {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "don't mask interrupt due to update of periph2_clk_sel - interrupt will be created"]
            pub const MASK_PERIPH2_CLK_SEL_LOADED_0: u32 = 0;
            #[doc = "mask interrupt due to update of periph2_clk_sel"]
            pub const MASK_PERIPH2_CLK_SEL_LOADED_1: u32 = 0x01;
        }
    }
    #[doc = "mask interrupt generation due to frequency change of ahb_podf"]
    pub mod MASK_AHB_PODF_LOADED {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "don't mask interrupt due to frequency change of ahb_podf - interrupt will be created"]
            pub const MASK_AHB_PODF_LOADED_0: u32 = 0;
            #[doc = "mask interrupt due to frequency change of ahb_podf"]
            pub const MASK_AHB_PODF_LOADED_1: u32 = 0x01;
        }
    }
    #[doc = "mask interrupt generation due to update of periph_clk_sel."]
    pub mod MASK_PERIPH_CLK_SEL_LOADED {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "don't mask interrupt due to update of periph_clk_sel - interrupt will be created"]
            pub const MASK_PERIPH_CLK_SEL_LOADED_0: u32 = 0;
            #[doc = "mask interrupt due to update of periph_clk_sel"]
            pub const MASK_PERIPH_CLK_SEL_LOADED_1: u32 = 0x01;
        }
    }
    #[doc = "mask interrupt generation due to frequency change of arm_podf"]
    pub mod ARM_PODF_LOADED {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "don't mask interrupt due to frequency change of arm_podf - interrupt will be created"]
            pub const ARM_PODF_LOADED_0: u32 = 0;
            #[doc = "mask interrupt due to frequency change of arm_podf"]
            pub const ARM_PODF_LOADED_1: u32 = 0x01;
        }
    }
}
#[doc = "CCM Clock Output Source Register"]
pub mod CCOSR {
    #[doc = "Selection of the clock to be generated on CCM_CLKO1"]
    pub mod CLKO1_SEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "USB1 PLL clock (divided by 2)"]
            pub const CLKO1_SEL_0: u32 = 0;
            #[doc = "SYS PLL clock (divided by 2)"]
            pub const CLKO1_SEL_1: u32 = 0x01;
            #[doc = "VIDEO PLL clock (divided by 2)"]
            pub const CLKO1_SEL_3: u32 = 0x03;
            #[doc = "semc_clk_root"]
            pub const CLKO1_SEL_5: u32 = 0x05;
            #[doc = "lcdif_pix_clk_root"]
            pub const CLKO1_SEL_10: u32 = 0x0a;
            #[doc = "ahb_clk_root"]
            pub const CLKO1_SEL_11: u32 = 0x0b;
            #[doc = "ipg_clk_root"]
            pub const CLKO1_SEL_12: u32 = 0x0c;
            #[doc = "perclk_root"]
            pub const CLKO1_SEL_13: u32 = 0x0d;
            #[doc = "ckil_sync_clk_root"]
            pub const CLKO1_SEL_14: u32 = 0x0e;
            #[doc = "pll4_main_clk"]
            pub const CLKO1_SEL_15: u32 = 0x0f;
        }
    }
    #[doc = "Setting the divider of CCM_CLKO1"]
    pub mod CLKO1_DIV {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "divide by 1"]
            pub const CLKO1_DIV_0: u32 = 0;
            #[doc = "divide by 2"]
            pub const CLKO1_DIV_1: u32 = 0x01;
            #[doc = "divide by 3"]
            pub const CLKO1_DIV_2: u32 = 0x02;
            #[doc = "divide by 4"]
            pub const CLKO1_DIV_3: u32 = 0x03;
            #[doc = "divide by 5"]
            pub const CLKO1_DIV_4: u32 = 0x04;
            #[doc = "divide by 6"]
            pub const CLKO1_DIV_5: u32 = 0x05;
            #[doc = "divide by 7"]
            pub const CLKO1_DIV_6: u32 = 0x06;
            #[doc = "divide by 8"]
            pub const CLKO1_DIV_7: u32 = 0x07;
        }
    }
    #[doc = "Enable of CCM_CLKO1 clock"]
    pub mod CLKO1_EN {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "CCM_CLKO1 disabled."]
            pub const CLKO1_EN_0: u32 = 0;
            #[doc = "CCM_CLKO1 enabled."]
            pub const CLKO1_EN_1: u32 = 0x01;
        }
    }
    #[doc = "CCM_CLKO1 output to reflect CCM_CLKO1 or CCM_CLKO2 clocks"]
    pub mod CLK_OUT_SEL {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "CCM_CLKO1 output drives CCM_CLKO1 clock"]
            pub const CLK_OUT_SEL_0: u32 = 0;
            #[doc = "CCM_CLKO1 output drives CCM_CLKO2 clock"]
            pub const CLK_OUT_SEL_1: u32 = 0x01;
        }
    }
    #[doc = "Selection of the clock to be generated on CCM_CLKO2"]
    pub mod CLKO2_SEL {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "usdhc1_clk_root"]
            pub const CLKO2_SEL_3: u32 = 0x03;
            #[doc = "lpi2c_clk_root"]
            pub const CLKO2_SEL_6: u32 = 0x06;
            #[doc = "csi_clk_root"]
            pub const CLKO2_SEL_11: u32 = 0x0b;
            #[doc = "osc_clk"]
            pub const CLKO2_SEL_14: u32 = 0x0e;
            #[doc = "usdhc2_clk_root"]
            pub const CLKO2_SEL_17: u32 = 0x11;
            #[doc = "sai1_clk_root"]
            pub const CLKO2_SEL_18: u32 = 0x12;
            #[doc = "sai2_clk_root"]
            pub const CLKO2_SEL_19: u32 = 0x13;
            #[doc = "sai3_clk_root (shared with ADC1 and ADC2 alt_clk root)"]
            pub const CLKO2_SEL_20: u32 = 0x14;
            #[doc = "can_clk_root (FlexCAN, shared with CANFD)"]
            pub const CLKO2_SEL_23: u32 = 0x17;
            #[doc = "flexspi_clk_root"]
            pub const CLKO2_SEL_27: u32 = 0x1b;
            #[doc = "uart_clk_root"]
            pub const CLKO2_SEL_28: u32 = 0x1c;
            #[doc = "spdif0_clk_root"]
            pub const CLKO2_SEL_29: u32 = 0x1d;
        }
    }
    #[doc = "Setting the divider of CCM_CLKO2"]
    pub mod CLKO2_DIV {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "divide by 1"]
            pub const CLKO2_DIV_0: u32 = 0;
            #[doc = "divide by 2"]
            pub const CLKO2_DIV_1: u32 = 0x01;
            #[doc = "divide by 3"]
            pub const CLKO2_DIV_2: u32 = 0x02;
            #[doc = "divide by 4"]
            pub const CLKO2_DIV_3: u32 = 0x03;
            #[doc = "divide by 5"]
            pub const CLKO2_DIV_4: u32 = 0x04;
            #[doc = "divide by 6"]
            pub const CLKO2_DIV_5: u32 = 0x05;
            #[doc = "divide by 7"]
            pub const CLKO2_DIV_6: u32 = 0x06;
            #[doc = "divide by 8"]
            pub const CLKO2_DIV_7: u32 = 0x07;
        }
    }
    #[doc = "Enable of CCM_CLKO2 clock"]
    pub mod CLKO2_EN {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "CCM_CLKO2 disabled."]
            pub const CLKO2_EN_0: u32 = 0;
            #[doc = "CCM_CLKO2 enabled."]
            pub const CLKO2_EN_1: u32 = 0x01;
        }
    }
}
#[doc = "CCM General Purpose Register"]
pub mod CGPR {
    #[doc = "Defines clock dividion of clock for stby_count (pmic delay counter)"]
    pub mod PMIC_DELAY_SCALER {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "clock is not divided"]
            pub const PMIC_DELAY_SCALER_0: u32 = 0;
            #[doc = "clock is divided /8"]
            pub const PMIC_DELAY_SCALER_1: u32 = 0x01;
        }
    }
    #[doc = "Defines the value of the output signal cgpr_dout\\[4\\]. Gate of program supply for efuse programing"]
    pub mod EFUSE_PROG_SUPPLY_GATE {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "fuse programing supply voltage is gated off to the efuse module"]
            pub const EFUSE_PROG_SUPPLY_GATE_0: u32 = 0;
            #[doc = "allow fuse programing."]
            pub const EFUSE_PROG_SUPPLY_GATE_1: u32 = 0x01;
        }
    }
    #[doc = "System memory DS control"]
    pub mod SYS_MEM_DS_CTRL {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable memory DS mode always"]
            pub const SYS_MEM_DS_CTRL_0: u32 = 0;
            #[doc = "Enable memory (outside ARM platform) DS mode when system STOP and PLL are disabled"]
            pub const SYS_MEM_DS_CTRL_1: u32 = 0x01;
            #[doc = "enable memory (outside ARM platform) DS mode when system is in STOP mode"]
            pub const SYS_MEM_DS_CTRL_2: u32 = 0x02;
        }
    }
    #[doc = "Fast PLL enable."]
    pub mod FPL {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Engage PLL enable default way."]
            pub const FPL_0: u32 = 0;
            #[doc = "Engage PLL enable 3 CKIL clocks earlier at exiting low power mode (STOP). Should be used only if 24MHz OSC was active in low power mode."]
            pub const FPL_1: u32 = 0x01;
        }
    }
    #[doc = "Control for the Deep Sleep signal to the ARM Platform memories with additional control logic based on the ARM WFI signal"]
    pub mod INT_MEM_CLK_LPM {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable the clock to the ARM platform memories when entering Low Power Mode"]
            pub const INT_MEM_CLK_LPM_0: u32 = 0;
            #[doc = "Keep the clocks to the ARM platform memories enabled only if an interrupt is pending when entering Low Power Modes (WAIT and STOP without power gating)"]
            pub const INT_MEM_CLK_LPM_1: u32 = 0x01;
        }
    }
}
#[doc = "CCM Clock Gating Register 0"]
pub mod CCGR0 {
    #[doc = "aips_tz1 clocks (aips_tz1_clk_enable)"]
    pub mod CG0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "aips_tz2 clocks (aips_tz2_clk_enable)"]
    pub mod CG1 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "mqs clock ( mqs_hmclk_clock_enable)"]
    pub mod CG2 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "flexspi_exsc clock (flexspi_exsc_clk_enable)"]
    pub mod CG3 {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "sim_m or sim_main register access clock (sim_m_mainclk_r_enable)"]
    pub mod CG4 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "dcp clock (dcp_clk_enable)"]
    pub mod CG5 {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "lpuart3 clock (lpuart3_clk_enable)"]
    pub mod CG6 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "can1 clock (can1_clk_enable)"]
    pub mod CG7 {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "can1_serial clock (can1_serial_clk_enable)"]
    pub mod CG8 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "can2 clock (can2_clk_enable)"]
    pub mod CG9 {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "can2_serial clock (can2_serial_clk_enable)"]
    pub mod CG10 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "trace clock (trace_clk_enable)"]
    pub mod CG11 {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "gpt2 bus clocks (gpt2_bus_clk_enable)"]
    pub mod CG12 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "gpt2 serial clocks (gpt2_serial_clk_enable)"]
    pub mod CG13 {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "lpuart2 clock (lpuart2_clk_enable)"]
    pub mod CG14 {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "gpio2_clocks (gpio2_clk_enable)"]
    pub mod CG15 {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "CCM Clock Gating Register 1"]
pub mod CCGR1 {
    #[doc = "lpspi1 clocks (lpspi1_clk_enable)"]
    pub mod CG0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "lpspi2 clocks (lpspi2_clk_enable)"]
    pub mod CG1 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "lpspi3 clocks (lpspi3_clk_enable)"]
    pub mod CG2 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "lpspi4 clocks (lpspi4_clk_enable)"]
    pub mod CG3 {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "adc2 clock (adc2_clk_enable)"]
    pub mod CG4 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "enet clock (enet_clk_enable)"]
    pub mod CG5 {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "pit clocks (pit_clk_enable)"]
    pub mod CG6 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "aoi2 clocks (aoi2_clk_enable)"]
    pub mod CG7 {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "adc1 clock (adc1_clk_enable)"]
    pub mod CG8 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "semc_exsc clock (semc_exsc_clk_enable)"]
    pub mod CG9 {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "gpt1 bus clock (gpt_clk_enable)"]
    pub mod CG10 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "gpt1 serial clock (gpt_serial_clk_enable)"]
    pub mod CG11 {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "lpuart4 clock (lpuart4_clk_enable)"]
    pub mod CG12 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "gpio1 clock (gpio1_clk_enable)"]
    pub mod CG13 {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "csu clock (csu_clk_enable)"]
    pub mod CG14 {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "gpio5 clock (gpio5_clk_enable)"]
    pub mod CG15 {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "CCM Clock Gating Register 2"]
pub mod CCGR2 {
    #[doc = "ocram_exsc clock (ocram_exsc_clk_enable)"]
    pub mod CG0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "csi clock (csi_clk_enable)"]
    pub mod CG1 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "iomuxc_snvs clock (iomuxc_snvs_clk_enable)"]
    pub mod CG2 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "lpi2c1 clock (lpi2c1_clk_enable)"]
    pub mod CG3 {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "lpi2c2 clock (lpi2c2_clk_enable)"]
    pub mod CG4 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "lpi2c3 clock (lpi2c3_clk_enable)"]
    pub mod CG5 {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "OCOTP_CTRL clock (iim_clk_enable)"]
    pub mod CG6 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "xbar3 clock (xbar3_clk_enable)"]
    pub mod CG7 {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ipmux1 clock (ipmux1_clk_enable)"]
    pub mod CG8 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ipmux2 clock (ipmux2_clk_enable)"]
    pub mod CG9 {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ipmux3 clock (ipmux3_clk_enable)"]
    pub mod CG10 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "xbar1 clock (xbar1_clk_enable)"]
    pub mod CG11 {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "xbar2 clock (xbar2_clk_enable)"]
    pub mod CG12 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "gpio3 clock (gpio3_clk_enable)"]
    pub mod CG13 {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "lcd clocks (lcd_clk_enable)"]
    pub mod CG14 {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "pxp clocks (pxp_clk_enable)"]
    pub mod CG15 {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "CCM Clock Gating Register 3"]
pub mod CCGR3 {
    #[doc = "flexio2 clocks (flexio2_clk_enable)"]
    pub mod CG0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "lpuart5 clock (lpuart5_clk_enable)"]
    pub mod CG1 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "semc clocks (semc_clk_enable)"]
    pub mod CG2 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "lpuart6 clock (lpuart6_clk_enable)"]
    pub mod CG3 {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "aoi1 clock (aoi1_clk_enable)"]
    pub mod CG4 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "lcdif pix clock (lcdif_pix_clk_enable)"]
    pub mod CG5 {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "gpio4 clock (gpio4_clk_enable)"]
    pub mod CG6 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ewm clocks (ewm_clk_enable)"]
    pub mod CG7 {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "wdog1 clock (wdog1_clk_enable)"]
    pub mod CG8 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "flexram clock (flexram_clk_enable)"]
    pub mod CG9 {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "acmp1 clocks (acmp1_clk_enable)"]
    pub mod CG10 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "acmp2 clocks (acmp2_clk_enable)"]
    pub mod CG11 {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "acmp3 clocks (acmp3_clk_enable)"]
    pub mod CG12 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "acmp4 clocks (acmp4_clk_enable)"]
    pub mod CG13 {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The OCRAM clock cannot be turned off when the CM cache is running on this device."]
    pub mod CG14 {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "iomuxc_snvs_gpr clock (iomuxc_snvs_gpr_clk_enable)"]
    pub mod CG15 {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "CCM Clock Gating Register 4"]
pub mod CCGR4 {
    #[doc = "sim_m7 register access clock (sim_m7_mainclk_r_enable)"]
    pub mod CG0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "iomuxc clock (iomuxc_clk_enable)"]
    pub mod CG1 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "iomuxc gpr clock (iomuxc_gpr_clk_enable)"]
    pub mod CG2 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "bee clock(bee_clk_enable)"]
    pub mod CG3 {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "sim_m7 clock (sim_m7_clk_enable)"]
    pub mod CG4 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "tsc_dig clock (tsc_clk_enable)"]
    pub mod CG5 {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "sim_m clocks (sim_m_clk_enable)"]
    pub mod CG6 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "sim_ems clocks (sim_ems_clk_enable)"]
    pub mod CG7 {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "pwm1 clocks (pwm1_clk_enable)"]
    pub mod CG8 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "pwm2 clocks (pwm2_clk_enable)"]
    pub mod CG9 {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "pwm3 clocks (pwm3_clk_enable)"]
    pub mod CG10 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "pwm4 clocks (pwm4_clk_enable)"]
    pub mod CG11 {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "enc1 clocks (enc1_clk_enable)"]
    pub mod CG12 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "enc2 clocks (enc2_clk_enable)"]
    pub mod CG13 {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "enc3 clocks (enc3_clk_enable)"]
    pub mod CG14 {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "enc4 clocks (enc4_clk_enable)"]
    pub mod CG15 {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "CCM Clock Gating Register 5"]
pub mod CCGR5 {
    #[doc = "rom clock (rom_clk_enable)"]
    pub mod CG0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "flexio1 clock (flexio1_clk_enable)"]
    pub mod CG1 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "wdog3 clock (wdog3_clk_enable)"]
    pub mod CG2 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "dma clock (dma_clk_enable)"]
    pub mod CG3 {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "kpp clock (kpp_clk_enable)"]
    pub mod CG4 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "wdog2 clock (wdog2_clk_enable)"]
    pub mod CG5 {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "aipstz4 clocks (aips_tz4_clk_enable)"]
    pub mod CG6 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "spdif clock (spdif_clk_enable)"]
    pub mod CG7 {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "sim_main clock (sim_main_clk_enable)"]
    pub mod CG8 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "sai1 clock (sai1_clk_enable)"]
    pub mod CG9 {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "sai2 clock (sai2_clk_enable)"]
    pub mod CG10 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "sai3 clock (sai3_clk_enable)"]
    pub mod CG11 {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "lpuart1 clock (lpuart1_clk_enable)"]
    pub mod CG12 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "lpuart7 clock (lpuart7_clk_enable)"]
    pub mod CG13 {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "snvs_hp clock (snvs_hp_clk_enable)"]
    pub mod CG14 {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "snvs_lp clock (snvs_lp_clk_enable)"]
    pub mod CG15 {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "CCM Clock Gating Register 6"]
pub mod CCGR6 {
    #[doc = "usboh3 clock (usboh3_clk_enable)"]
    pub mod CG0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "usdhc1 clocks (usdhc1_clk_enable)"]
    pub mod CG1 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "usdhc2 clocks (usdhc2_clk_enable)"]
    pub mod CG2 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "dcdc clocks (dcdc_clk_enable)"]
    pub mod CG3 {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ipmux4 clock (ipmux4_clk_enable)"]
    pub mod CG4 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "flexspi clocks (flexspi_clk_enable) sim_ems_clk_enable must also be cleared, when flexspi_clk_enable is cleared"]
    pub mod CG5 {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "trng clock (trng_clk_enable)"]
    pub mod CG6 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "lpuart8 clocks (lpuart8_clk_enable)"]
    pub mod CG7 {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "timer4 clocks (timer4_clk_enable)"]
    pub mod CG8 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "aips_tz3 clock (aips_tz3_clk_enable)"]
    pub mod CG9 {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "sim_axbs_p_clk_enable"]
    pub mod CG10 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "anadig clocks (anadig_clk_enable)"]
    pub mod CG11 {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "lpi2c4 serial clock (lpi2c4_serial_clk_enable)"]
    pub mod CG12 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "timer1 clocks (timer1_clk_enable)"]
    pub mod CG13 {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "timer2 clocks (timer2_clk_enable)"]
    pub mod CG14 {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "timer3 clocks (timer3_clk_enable)"]
    pub mod CG15 {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "CCM Clock Gating Register 7"]
pub mod CCGR7 {
    #[doc = "enet2_clk_enable"]
    pub mod CG0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "flexspi2_clk_enable"]
    pub mod CG1 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "axbs_l_clk_enable"]
    pub mod CG2 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "can3_clk_enable"]
    pub mod CG3 {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "can3_serial_clk_enable"]
    pub mod CG4 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "aips_lite_clk_enable"]
    pub mod CG5 {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "flexio3_clk_enable"]
    pub mod CG6 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "CCM Module Enable Overide Register"]
pub mod CMEOR {
    #[doc = "Overide clock enable signal from GPT - clock will not be gated based on GPT's signal 'ipg_enable_clk'"]
    pub mod MOD_EN_OV_GPT {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "don't override module enable signal"]
            pub const MOD_EN_OV_GPT_0: u32 = 0;
            #[doc = "override module enable signal"]
            pub const MOD_EN_OV_GPT_1: u32 = 0x01;
        }
    }
    #[doc = "Overide clock enable signal from PIT - clock will not be gated based on PIT's signal 'ipg_enable_clk'"]
    pub mod MOD_EN_OV_PIT {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "don't override module enable signal"]
            pub const MOD_EN_OV_PIT_0: u32 = 0;
            #[doc = "override module enable signal"]
            pub const MOD_EN_OV_PIT_1: u32 = 0x01;
        }
    }
    #[doc = "overide clock enable signal from USDHC."]
    pub mod MOD_EN_USDHC {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "don't override module enable signal"]
            pub const MOD_EN_USDHC_0: u32 = 0;
            #[doc = "override module enable signal"]
            pub const MOD_EN_USDHC_1: u32 = 0x01;
        }
    }
    #[doc = "Overide clock enable signal from TRNG"]
    pub mod MOD_EN_OV_TRNG {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "don't override module enable signal"]
            pub const MOD_EN_OV_TRNG_0: u32 = 0;
            #[doc = "override module enable signal"]
            pub const MOD_EN_OV_TRNG_1: u32 = 0x01;
        }
    }
    #[doc = "Overide clock enable signal from FlexCAN3(CANFD) - clock will not be gated based on CAN's signal 'enable_clk_cpi'"]
    pub mod MOD_EN_OV_CANFD_CPI {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "don't override module enable signal"]
            pub const MOD_EN_OV_CANFD_CPI_0: u32 = 0;
            #[doc = "override module enable signal"]
            pub const MOD_EN_OV_CANFD_CPI_1: u32 = 0x01;
        }
    }
    #[doc = "Overide clock enable signal from CAN2 - clock will not be gated based on CAN's signal 'enable_clk_cpi'"]
    pub mod MOD_EN_OV_CAN2_CPI {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "don't override module enable signal"]
            pub const MOD_EN_OV_CAN2_CPI_0: u32 = 0;
            #[doc = "override module enable signal"]
            pub const MOD_EN_OV_CAN2_CPI_1: u32 = 0x01;
        }
    }
    #[doc = "Overide clock enable signal from CAN1 - clock will not be gated based on CAN's signal 'enable_clk_cpi'"]
    pub mod MOD_EN_OV_CAN1_CPI {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "don't overide module enable signal"]
            pub const MOD_EN_OV_CAN1_CPI_0: u32 = 0;
            #[doc = "overide module enable signal"]
            pub const MOD_EN_OV_CAN1_CPI_1: u32 = 0x01;
        }
    }
}
