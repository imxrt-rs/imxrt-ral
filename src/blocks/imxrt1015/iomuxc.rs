#[doc = "IOMUXC"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x24],
    #[doc = "SW_MUX_CTL_PAD_GPIO_EMC_04 SW MUX Control Register"]
    pub SW_MUX_CTL_PAD_GPIO_EMC_04: crate::RWRegister<u32>,
    #[doc = "SW_MUX_CTL_PAD_GPIO_EMC_05 SW MUX Control Register"]
    pub SW_MUX_CTL_PAD_GPIO_EMC_05: crate::RWRegister<u32>,
    #[doc = "SW_MUX_CTL_PAD_GPIO_EMC_06 SW MUX Control Register"]
    pub SW_MUX_CTL_PAD_GPIO_EMC_06: crate::RWRegister<u32>,
    #[doc = "SW_MUX_CTL_PAD_GPIO_EMC_07 SW MUX Control Register"]
    pub SW_MUX_CTL_PAD_GPIO_EMC_07: crate::RWRegister<u32>,
    #[doc = "SW_MUX_CTL_PAD_GPIO_EMC_08 SW MUX Control Register"]
    pub SW_MUX_CTL_PAD_GPIO_EMC_08: crate::RWRegister<u32>,
    #[doc = "SW_MUX_CTL_PAD_GPIO_EMC_09 SW MUX Control Register"]
    pub SW_MUX_CTL_PAD_GPIO_EMC_09: crate::RWRegister<u32>,
    _reserved1: [u8; 0x18],
    #[doc = "SW_MUX_CTL_PAD_GPIO_EMC_16 SW MUX Control Register"]
    pub SW_MUX_CTL_PAD_GPIO_EMC_16: crate::RWRegister<u32>,
    #[doc = "SW_MUX_CTL_PAD_GPIO_EMC_17 SW MUX Control Register"]
    pub SW_MUX_CTL_PAD_GPIO_EMC_17: crate::RWRegister<u32>,
    #[doc = "SW_MUX_CTL_PAD_GPIO_EMC_18 SW MUX Control Register"]
    pub SW_MUX_CTL_PAD_GPIO_EMC_18: crate::RWRegister<u32>,
    #[doc = "SW_MUX_CTL_PAD_GPIO_EMC_19 SW MUX Control Register"]
    pub SW_MUX_CTL_PAD_GPIO_EMC_19: crate::RWRegister<u32>,
    #[doc = "SW_MUX_CTL_PAD_GPIO_EMC_20 SW MUX Control Register"]
    pub SW_MUX_CTL_PAD_GPIO_EMC_20: crate::RWRegister<u32>,
    #[doc = "SW_MUX_CTL_PAD_GPIO_EMC_21 SW MUX Control Register"]
    pub SW_MUX_CTL_PAD_GPIO_EMC_21: crate::RWRegister<u32>,
    #[doc = "SW_MUX_CTL_PAD_GPIO_EMC_22 SW MUX Control Register"]
    pub SW_MUX_CTL_PAD_GPIO_EMC_22: crate::RWRegister<u32>,
    #[doc = "SW_MUX_CTL_PAD_GPIO_EMC_23 SW MUX Control Register"]
    pub SW_MUX_CTL_PAD_GPIO_EMC_23: crate::RWRegister<u32>,
    #[doc = "SW_MUX_CTL_PAD_GPIO_EMC_24 SW MUX Control Register"]
    pub SW_MUX_CTL_PAD_GPIO_EMC_24: crate::RWRegister<u32>,
    #[doc = "SW_MUX_CTL_PAD_GPIO_EMC_25 SW MUX Control Register"]
    pub SW_MUX_CTL_PAD_GPIO_EMC_25: crate::RWRegister<u32>,
    #[doc = "SW_MUX_CTL_PAD_GPIO_EMC_26 SW MUX Control Register"]
    pub SW_MUX_CTL_PAD_GPIO_EMC_26: crate::RWRegister<u32>,
    #[doc = "SW_MUX_CTL_PAD_GPIO_EMC_27 SW MUX Control Register"]
    pub SW_MUX_CTL_PAD_GPIO_EMC_27: crate::RWRegister<u32>,
    _reserved2: [u8; 0x10],
    #[doc = "SW_MUX_CTL_PAD_GPIO_EMC_32 SW MUX Control Register"]
    pub SW_MUX_CTL_PAD_GPIO_EMC_32: crate::RWRegister<u32>,
    #[doc = "SW_MUX_CTL_PAD_GPIO_EMC_33 SW MUX Control Register"]
    pub SW_MUX_CTL_PAD_GPIO_EMC_33: crate::RWRegister<u32>,
    #[doc = "SW_MUX_CTL_PAD_GPIO_EMC_34 SW MUX Control Register"]
    pub SW_MUX_CTL_PAD_GPIO_EMC_34: crate::RWRegister<u32>,
    #[doc = "SW_MUX_CTL_PAD_GPIO_EMC_35 SW MUX Control Register"]
    pub SW_MUX_CTL_PAD_GPIO_EMC_35: crate::RWRegister<u32>,
    _reserved3: [u8; 0x18],
    #[doc = "SW_MUX_CTL_PAD_GPIO_AD_B0_00 SW MUX Control Register"]
    pub SW_MUX_CTL_PAD_GPIO_AD_B0_00: crate::RWRegister<u32>,
    #[doc = "SW_MUX_CTL_PAD_GPIO_AD_B0_01 SW MUX Control Register"]
    pub SW_MUX_CTL_PAD_GPIO_AD_B0_01: crate::RWRegister<u32>,
    #[doc = "SW_MUX_CTL_PAD_GPIO_AD_B0_02 SW MUX Control Register"]
    pub SW_MUX_CTL_PAD_GPIO_AD_B0_02: crate::RWRegister<u32>,
    #[doc = "SW_MUX_CTL_PAD_GPIO_AD_B0_03 SW MUX Control Register"]
    pub SW_MUX_CTL_PAD_GPIO_AD_B0_03: crate::RWRegister<u32>,
    #[doc = "SW_MUX_CTL_PAD_GPIO_AD_B0_04 SW MUX Control Register"]
    pub SW_MUX_CTL_PAD_GPIO_AD_B0_04: crate::RWRegister<u32>,
    #[doc = "SW_MUX_CTL_PAD_GPIO_AD_B0_05 SW MUX Control Register"]
    pub SW_MUX_CTL_PAD_GPIO_AD_B0_05: crate::RWRegister<u32>,
    #[doc = "SW_MUX_CTL_PAD_GPIO_AD_B0_06 SW MUX Control Register"]
    pub SW_MUX_CTL_PAD_GPIO_AD_B0_06: crate::RWRegister<u32>,
    #[doc = "SW_MUX_CTL_PAD_GPIO_AD_B0_07 SW MUX Control Register"]
    pub SW_MUX_CTL_PAD_GPIO_AD_B0_07: crate::RWRegister<u32>,
    #[doc = "SW_MUX_CTL_PAD_GPIO_AD_B0_08 SW MUX Control Register"]
    pub SW_MUX_CTL_PAD_GPIO_AD_B0_08: crate::RWRegister<u32>,
    #[doc = "SW_MUX_CTL_PAD_GPIO_AD_B0_09 SW MUX Control Register"]
    pub SW_MUX_CTL_PAD_GPIO_AD_B0_09: crate::RWRegister<u32>,
    #[doc = "SW_MUX_CTL_PAD_GPIO_AD_B0_10 SW MUX Control Register"]
    pub SW_MUX_CTL_PAD_GPIO_AD_B0_10: crate::RWRegister<u32>,
    #[doc = "SW_MUX_CTL_PAD_GPIO_AD_B0_11 SW MUX Control Register"]
    pub SW_MUX_CTL_PAD_GPIO_AD_B0_11: crate::RWRegister<u32>,
    #[doc = "SW_MUX_CTL_PAD_GPIO_AD_B0_12 SW MUX Control Register"]
    pub SW_MUX_CTL_PAD_GPIO_AD_B0_12: crate::RWRegister<u32>,
    #[doc = "SW_MUX_CTL_PAD_GPIO_AD_B0_13 SW MUX Control Register"]
    pub SW_MUX_CTL_PAD_GPIO_AD_B0_13: crate::RWRegister<u32>,
    #[doc = "SW_MUX_CTL_PAD_GPIO_AD_B0_14 SW MUX Control Register"]
    pub SW_MUX_CTL_PAD_GPIO_AD_B0_14: crate::RWRegister<u32>,
    #[doc = "SW_MUX_CTL_PAD_GPIO_AD_B0_15 SW MUX Control Register"]
    pub SW_MUX_CTL_PAD_GPIO_AD_B0_15: crate::RWRegister<u32>,
    _reserved4: [u8; 0x28],
    #[doc = "SW_MUX_CTL_PAD_GPIO_AD_B1_10 SW MUX Control Register"]
    pub SW_MUX_CTL_PAD_GPIO_AD_B1_10: crate::RWRegister<u32>,
    #[doc = "SW_MUX_CTL_PAD_GPIO_AD_B1_11 SW MUX Control Register"]
    pub SW_MUX_CTL_PAD_GPIO_AD_B1_11: crate::RWRegister<u32>,
    #[doc = "SW_MUX_CTL_PAD_GPIO_AD_B1_12 SW MUX Control Register"]
    pub SW_MUX_CTL_PAD_GPIO_AD_B1_12: crate::RWRegister<u32>,
    #[doc = "SW_MUX_CTL_PAD_GPIO_AD_B1_13 SW MUX Control Register"]
    pub SW_MUX_CTL_PAD_GPIO_AD_B1_13: crate::RWRegister<u32>,
    #[doc = "SW_MUX_CTL_PAD_GPIO_AD_B1_14 SW MUX Control Register"]
    pub SW_MUX_CTL_PAD_GPIO_AD_B1_14: crate::RWRegister<u32>,
    #[doc = "SW_MUX_CTL_PAD_GPIO_AD_B1_15 SW MUX Control Register"]
    pub SW_MUX_CTL_PAD_GPIO_AD_B1_15: crate::RWRegister<u32>,
    _reserved5: [u8; 0x1c],
    #[doc = "SW_MUX_CTL_PAD_GPIO_SD_B1_00 SW MUX Control Register"]
    pub SW_MUX_CTL_PAD_GPIO_SD_B1_00: crate::RWRegister<u32>,
    #[doc = "SW_MUX_CTL_PAD_GPIO_SD_B1_01 SW MUX Control Register"]
    pub SW_MUX_CTL_PAD_GPIO_SD_B1_01: crate::RWRegister<u32>,
    #[doc = "SW_MUX_CTL_PAD_GPIO_SD_B1_02 SW MUX Control Register"]
    pub SW_MUX_CTL_PAD_GPIO_SD_B1_02: crate::RWRegister<u32>,
    #[doc = "SW_MUX_CTL_PAD_GPIO_SD_B1_03 SW MUX Control Register"]
    pub SW_MUX_CTL_PAD_GPIO_SD_B1_03: crate::RWRegister<u32>,
    #[doc = "SW_MUX_CTL_PAD_GPIO_SD_B1_04 SW MUX Control Register"]
    pub SW_MUX_CTL_PAD_GPIO_SD_B1_04: crate::RWRegister<u32>,
    #[doc = "SW_MUX_CTL_PAD_GPIO_SD_B1_05 SW MUX Control Register"]
    pub SW_MUX_CTL_PAD_GPIO_SD_B1_05: crate::RWRegister<u32>,
    #[doc = "SW_MUX_CTL_PAD_GPIO_SD_B1_06 SW MUX Control Register"]
    pub SW_MUX_CTL_PAD_GPIO_SD_B1_06: crate::RWRegister<u32>,
    #[doc = "SW_MUX_CTL_PAD_GPIO_SD_B1_07 SW MUX Control Register"]
    pub SW_MUX_CTL_PAD_GPIO_SD_B1_07: crate::RWRegister<u32>,
    #[doc = "SW_MUX_CTL_PAD_GPIO_SD_B1_08 SW MUX Control Register"]
    pub SW_MUX_CTL_PAD_GPIO_SD_B1_08: crate::RWRegister<u32>,
    #[doc = "SW_MUX_CTL_PAD_GPIO_SD_B1_09 SW MUX Control Register"]
    pub SW_MUX_CTL_PAD_GPIO_SD_B1_09: crate::RWRegister<u32>,
    #[doc = "SW_MUX_CTL_PAD_GPIO_SD_B1_10 SW MUX Control Register"]
    pub SW_MUX_CTL_PAD_GPIO_SD_B1_10: crate::RWRegister<u32>,
    #[doc = "SW_MUX_CTL_PAD_GPIO_SD_B1_11 SW MUX Control Register"]
    pub SW_MUX_CTL_PAD_GPIO_SD_B1_11: crate::RWRegister<u32>,
    _reserved6: [u8; 0x10],
    #[doc = "SW_PAD_CTL_PAD_GPIO_EMC_04 SW PAD Control Register"]
    pub SW_PAD_CTL_PAD_GPIO_EMC_04: crate::RWRegister<u32>,
    #[doc = "SW_PAD_CTL_PAD_GPIO_EMC_05 SW PAD Control Register"]
    pub SW_PAD_CTL_PAD_GPIO_EMC_05: crate::RWRegister<u32>,
    #[doc = "SW_PAD_CTL_PAD_GPIO_EMC_06 SW PAD Control Register"]
    pub SW_PAD_CTL_PAD_GPIO_EMC_06: crate::RWRegister<u32>,
    #[doc = "SW_PAD_CTL_PAD_GPIO_EMC_07 SW PAD Control Register"]
    pub SW_PAD_CTL_PAD_GPIO_EMC_07: crate::RWRegister<u32>,
    #[doc = "SW_PAD_CTL_PAD_GPIO_EMC_08 SW PAD Control Register"]
    pub SW_PAD_CTL_PAD_GPIO_EMC_08: crate::RWRegister<u32>,
    #[doc = "SW_PAD_CTL_PAD_GPIO_EMC_09 SW PAD Control Register"]
    pub SW_PAD_CTL_PAD_GPIO_EMC_09: crate::RWRegister<u32>,
    _reserved7: [u8; 0x18],
    #[doc = "SW_PAD_CTL_PAD_GPIO_EMC_16 SW PAD Control Register"]
    pub SW_PAD_CTL_PAD_GPIO_EMC_16: crate::RWRegister<u32>,
    #[doc = "SW_PAD_CTL_PAD_GPIO_EMC_17 SW PAD Control Register"]
    pub SW_PAD_CTL_PAD_GPIO_EMC_17: crate::RWRegister<u32>,
    #[doc = "SW_PAD_CTL_PAD_GPIO_EMC_18 SW PAD Control Register"]
    pub SW_PAD_CTL_PAD_GPIO_EMC_18: crate::RWRegister<u32>,
    #[doc = "SW_PAD_CTL_PAD_GPIO_EMC_19 SW PAD Control Register"]
    pub SW_PAD_CTL_PAD_GPIO_EMC_19: crate::RWRegister<u32>,
    #[doc = "SW_PAD_CTL_PAD_GPIO_EMC_20 SW PAD Control Register"]
    pub SW_PAD_CTL_PAD_GPIO_EMC_20: crate::RWRegister<u32>,
    #[doc = "SW_PAD_CTL_PAD_GPIO_EMC_21 SW PAD Control Register"]
    pub SW_PAD_CTL_PAD_GPIO_EMC_21: crate::RWRegister<u32>,
    #[doc = "SW_PAD_CTL_PAD_GPIO_EMC_22 SW PAD Control Register"]
    pub SW_PAD_CTL_PAD_GPIO_EMC_22: crate::RWRegister<u32>,
    #[doc = "SW_PAD_CTL_PAD_GPIO_EMC_23 SW PAD Control Register"]
    pub SW_PAD_CTL_PAD_GPIO_EMC_23: crate::RWRegister<u32>,
    #[doc = "SW_PAD_CTL_PAD_GPIO_EMC_24 SW PAD Control Register"]
    pub SW_PAD_CTL_PAD_GPIO_EMC_24: crate::RWRegister<u32>,
    #[doc = "SW_PAD_CTL_PAD_GPIO_EMC_25 SW PAD Control Register"]
    pub SW_PAD_CTL_PAD_GPIO_EMC_25: crate::RWRegister<u32>,
    #[doc = "SW_PAD_CTL_PAD_GPIO_EMC_26 SW PAD Control Register"]
    pub SW_PAD_CTL_PAD_GPIO_EMC_26: crate::RWRegister<u32>,
    #[doc = "SW_PAD_CTL_PAD_GPIO_EMC_27 SW PAD Control Register"]
    pub SW_PAD_CTL_PAD_GPIO_EMC_27: crate::RWRegister<u32>,
    _reserved8: [u8; 0x10],
    #[doc = "SW_PAD_CTL_PAD_GPIO_EMC_32 SW PAD Control Register"]
    pub SW_PAD_CTL_PAD_GPIO_EMC_32: crate::RWRegister<u32>,
    #[doc = "SW_PAD_CTL_PAD_GPIO_EMC_33 SW PAD Control Register"]
    pub SW_PAD_CTL_PAD_GPIO_EMC_33: crate::RWRegister<u32>,
    #[doc = "SW_PAD_CTL_PAD_GPIO_EMC_34 SW PAD Control Register"]
    pub SW_PAD_CTL_PAD_GPIO_EMC_34: crate::RWRegister<u32>,
    #[doc = "SW_PAD_CTL_PAD_GPIO_EMC_35 SW PAD Control Register"]
    pub SW_PAD_CTL_PAD_GPIO_EMC_35: crate::RWRegister<u32>,
    _reserved9: [u8; 0x18],
    #[doc = "SW_PAD_CTL_PAD_GPIO_AD_B0_00 SW PAD Control Register"]
    pub SW_PAD_CTL_PAD_GPIO_AD_B0_00: crate::RWRegister<u32>,
    #[doc = "SW_PAD_CTL_PAD_GPIO_AD_B0_01 SW PAD Control Register"]
    pub SW_PAD_CTL_PAD_GPIO_AD_B0_01: crate::RWRegister<u32>,
    #[doc = "SW_PAD_CTL_PAD_GPIO_AD_B0_02 SW PAD Control Register"]
    pub SW_PAD_CTL_PAD_GPIO_AD_B0_02: crate::RWRegister<u32>,
    #[doc = "SW_PAD_CTL_PAD_GPIO_AD_B0_03 SW PAD Control Register"]
    pub SW_PAD_CTL_PAD_GPIO_AD_B0_03: crate::RWRegister<u32>,
    #[doc = "SW_PAD_CTL_PAD_GPIO_AD_B0_04 SW PAD Control Register"]
    pub SW_PAD_CTL_PAD_GPIO_AD_B0_04: crate::RWRegister<u32>,
    #[doc = "SW_PAD_CTL_PAD_GPIO_AD_B0_05 SW PAD Control Register"]
    pub SW_PAD_CTL_PAD_GPIO_AD_B0_05: crate::RWRegister<u32>,
    #[doc = "SW_PAD_CTL_PAD_GPIO_AD_B0_06 SW PAD Control Register"]
    pub SW_PAD_CTL_PAD_GPIO_AD_B0_06: crate::RWRegister<u32>,
    #[doc = "SW_PAD_CTL_PAD_GPIO_AD_B0_07 SW PAD Control Register"]
    pub SW_PAD_CTL_PAD_GPIO_AD_B0_07: crate::RWRegister<u32>,
    #[doc = "SW_PAD_CTL_PAD_GPIO_AD_B0_08 SW PAD Control Register"]
    pub SW_PAD_CTL_PAD_GPIO_AD_B0_08: crate::RWRegister<u32>,
    #[doc = "SW_PAD_CTL_PAD_GPIO_AD_B0_09 SW PAD Control Register"]
    pub SW_PAD_CTL_PAD_GPIO_AD_B0_09: crate::RWRegister<u32>,
    #[doc = "SW_PAD_CTL_PAD_GPIO_AD_B0_10 SW PAD Control Register"]
    pub SW_PAD_CTL_PAD_GPIO_AD_B0_10: crate::RWRegister<u32>,
    #[doc = "SW_PAD_CTL_PAD_GPIO_AD_B0_11 SW PAD Control Register"]
    pub SW_PAD_CTL_PAD_GPIO_AD_B0_11: crate::RWRegister<u32>,
    #[doc = "SW_PAD_CTL_PAD_GPIO_AD_B0_12 SW PAD Control Register"]
    pub SW_PAD_CTL_PAD_GPIO_AD_B0_12: crate::RWRegister<u32>,
    #[doc = "SW_PAD_CTL_PAD_GPIO_AD_B0_13 SW PAD Control Register"]
    pub SW_PAD_CTL_PAD_GPIO_AD_B0_13: crate::RWRegister<u32>,
    #[doc = "SW_PAD_CTL_PAD_GPIO_AD_B0_14 SW PAD Control Register"]
    pub SW_PAD_CTL_PAD_GPIO_AD_B0_14: crate::RWRegister<u32>,
    #[doc = "SW_PAD_CTL_PAD_GPIO_AD_B0_15 SW PAD Control Register"]
    pub SW_PAD_CTL_PAD_GPIO_AD_B0_15: crate::RWRegister<u32>,
    _reserved10: [u8; 0x28],
    #[doc = "SW_PAD_CTL_PAD_GPIO_AD_B1_10 SW PAD Control Register"]
    pub SW_PAD_CTL_PAD_GPIO_AD_B1_10: crate::RWRegister<u32>,
    #[doc = "SW_PAD_CTL_PAD_GPIO_AD_B1_11 SW PAD Control Register"]
    pub SW_PAD_CTL_PAD_GPIO_AD_B1_11: crate::RWRegister<u32>,
    #[doc = "SW_PAD_CTL_PAD_GPIO_AD_B1_12 SW PAD Control Register"]
    pub SW_PAD_CTL_PAD_GPIO_AD_B1_12: crate::RWRegister<u32>,
    #[doc = "SW_PAD_CTL_PAD_GPIO_AD_B1_13 SW PAD Control Register"]
    pub SW_PAD_CTL_PAD_GPIO_AD_B1_13: crate::RWRegister<u32>,
    #[doc = "SW_PAD_CTL_PAD_GPIO_AD_B1_14 SW PAD Control Register"]
    pub SW_PAD_CTL_PAD_GPIO_AD_B1_14: crate::RWRegister<u32>,
    #[doc = "SW_PAD_CTL_PAD_GPIO_AD_B1_15 SW PAD Control Register"]
    pub SW_PAD_CTL_PAD_GPIO_AD_B1_15: crate::RWRegister<u32>,
    _reserved11: [u8; 0x1c],
    #[doc = "SW_PAD_CTL_PAD_GPIO_SD_B1_00 SW PAD Control Register"]
    pub SW_PAD_CTL_PAD_GPIO_SD_B1_00: crate::RWRegister<u32>,
    #[doc = "SW_PAD_CTL_PAD_GPIO_SD_B1_01 SW PAD Control Register"]
    pub SW_PAD_CTL_PAD_GPIO_SD_B1_01: crate::RWRegister<u32>,
    #[doc = "SW_PAD_CTL_PAD_GPIO_SD_B1_02 SW PAD Control Register"]
    pub SW_PAD_CTL_PAD_GPIO_SD_B1_02: crate::RWRegister<u32>,
    #[doc = "SW_PAD_CTL_PAD_GPIO_SD_B1_03 SW PAD Control Register"]
    pub SW_PAD_CTL_PAD_GPIO_SD_B1_03: crate::RWRegister<u32>,
    #[doc = "SW_PAD_CTL_PAD_GPIO_SD_B1_04 SW PAD Control Register"]
    pub SW_PAD_CTL_PAD_GPIO_SD_B1_04: crate::RWRegister<u32>,
    #[doc = "SW_PAD_CTL_PAD_GPIO_SD_B1_05 SW PAD Control Register"]
    pub SW_PAD_CTL_PAD_GPIO_SD_B1_05: crate::RWRegister<u32>,
    #[doc = "SW_PAD_CTL_PAD_GPIO_SD_B1_06 SW PAD Control Register"]
    pub SW_PAD_CTL_PAD_GPIO_SD_B1_06: crate::RWRegister<u32>,
    #[doc = "SW_PAD_CTL_PAD_GPIO_SD_B1_07 SW PAD Control Register"]
    pub SW_PAD_CTL_PAD_GPIO_SD_B1_07: crate::RWRegister<u32>,
    #[doc = "SW_PAD_CTL_PAD_GPIO_SD_B1_08 SW PAD Control Register"]
    pub SW_PAD_CTL_PAD_GPIO_SD_B1_08: crate::RWRegister<u32>,
    #[doc = "SW_PAD_CTL_PAD_GPIO_SD_B1_09 SW PAD Control Register"]
    pub SW_PAD_CTL_PAD_GPIO_SD_B1_09: crate::RWRegister<u32>,
    #[doc = "SW_PAD_CTL_PAD_GPIO_SD_B1_10 SW PAD Control Register"]
    pub SW_PAD_CTL_PAD_GPIO_SD_B1_10: crate::RWRegister<u32>,
    #[doc = "SW_PAD_CTL_PAD_GPIO_SD_B1_11 SW PAD Control Register"]
    pub SW_PAD_CTL_PAD_GPIO_SD_B1_11: crate::RWRegister<u32>,
    #[doc = "ANATOP_USB_OTG_ID_SELECT_INPUT DAISY Register"]
    pub ANATOP_USB_OTG_ID_SELECT_INPUT: crate::RWRegister<u32>,
    #[doc = "CCM_PMIC_VFUNCIONAL_READY_SELECT_INPUT DAISY Register"]
    pub CCM_PMIC_VFUNCIONAL_READY_SELECT_INPUT: crate::RWRegister<u32>,
    _reserved12: [u8; 0x24],
    #[doc = "FLEXPWM1_IPP_IND_PWMA_SELECT_INPUT_0 DAISY Register"]
    pub FLEXPWM1_IPP_IND_PWMA_SELECT_INPUT_0: crate::RWRegister<u32>,
    #[doc = "FLEXPWM1_IPP_IND_PWMA_SELECT_INPUT_1 DAISY Register"]
    pub FLEXPWM1_IPP_IND_PWMA_SELECT_INPUT_1: crate::RWRegister<u32>,
    #[doc = "FLEXPWM1_IPP_IND_PWMA_SELECT_INPUT_2 DAISY Register"]
    pub FLEXPWM1_IPP_IND_PWMA_SELECT_INPUT_2: crate::RWRegister<u32>,
    #[doc = "FLEXPWM1_IPP_IND_PWMA_SELECT_INPUT_3 DAISY Register"]
    pub FLEXPWM1_IPP_IND_PWMA_SELECT_INPUT_3: crate::RWRegister<u32>,
    #[doc = "FLEXPWM1_IPP_IND_PWMB_SELECT_INPUT_0 DAISY Register"]
    pub FLEXPWM1_IPP_IND_PWMB_SELECT_INPUT_0: crate::RWRegister<u32>,
    #[doc = "FLEXPWM1_IPP_IND_PWMB_SELECT_INPUT_1 DAISY Register"]
    pub FLEXPWM1_IPP_IND_PWMB_SELECT_INPUT_1: crate::RWRegister<u32>,
    #[doc = "FLEXPWM1_IPP_IND_PWMB_SELECT_INPUT_2 DAISY Register"]
    pub FLEXPWM1_IPP_IND_PWMB_SELECT_INPUT_2: crate::RWRegister<u32>,
    #[doc = "FLEXPWM1_IPP_IND_PWMB_SELECT_INPUT_3 DAISY Register"]
    pub FLEXPWM1_IPP_IND_PWMB_SELECT_INPUT_3: crate::RWRegister<u32>,
    _reserved13: [u8; 0x20],
    #[doc = "FLEXSPI_IPP_IND_IO_FA_BIT0_SELECT_INPUT DAISY Register"]
    pub FLEXSPI_IPP_IND_IO_FA_BIT0_SELECT_INPUT: crate::RWRegister<u32>,
    #[doc = "FLEXSPI_IPP_IND_IO_FA_BIT1_SELECT_INPUT DAISY Register"]
    pub FLEXSPI_IPP_IND_IO_FA_BIT1_SELECT_INPUT: crate::RWRegister<u32>,
    #[doc = "FLEXSPI_IPP_IND_IO_FA_BIT2_SELECT_INPUT DAISY Register"]
    pub FLEXSPI_IPP_IND_IO_FA_BIT2_SELECT_INPUT: crate::RWRegister<u32>,
    #[doc = "FLEXSPI_IPP_IND_IO_FA_BIT3_SELECT_INPUT DAISY Register"]
    pub FLEXSPI_IPP_IND_IO_FA_BIT3_SELECT_INPUT: crate::RWRegister<u32>,
    #[doc = "FLEXSPI_IPP_IND_SCK_FA_SELECT_INPUT DAISY Register"]
    pub FLEXSPI_IPP_IND_SCK_FA_SELECT_INPUT: crate::RWRegister<u32>,
    #[doc = "LPI2C1_IPP_IND_LPI2C_SCL_SELECT_INPUT DAISY Register"]
    pub LPI2C1_IPP_IND_LPI2C_SCL_SELECT_INPUT: crate::RWRegister<u32>,
    #[doc = "LPI2C1_IPP_IND_LPI2C_SDA_SELECT_INPUT DAISY Register"]
    pub LPI2C1_IPP_IND_LPI2C_SDA_SELECT_INPUT: crate::RWRegister<u32>,
    #[doc = "LPI2C2_IPP_IND_LPI2C_SCL_SELECT_INPUT DAISY Register"]
    pub LPI2C2_IPP_IND_LPI2C_SCL_SELECT_INPUT: crate::RWRegister<u32>,
    #[doc = "LPI2C2_IPP_IND_LPI2C_SDA_SELECT_INPUT DAISY Register"]
    pub LPI2C2_IPP_IND_LPI2C_SDA_SELECT_INPUT: crate::RWRegister<u32>,
    _reserved14: [u8; 0x10],
    #[doc = "LPSPI1_IPP_IND_LPSPI_PCS_SELECT_INPUT_0 DAISY Register"]
    pub LPSPI1_IPP_IND_LPSPI_PCS_SELECT_INPUT_0: crate::RWRegister<u32>,
    #[doc = "LPSPI1_IPP_IND_LPSPI_SCK_SELECT_INPUT DAISY Register"]
    pub LPSPI1_IPP_IND_LPSPI_SCK_SELECT_INPUT: crate::RWRegister<u32>,
    #[doc = "LPSPI1_IPP_IND_LPSPI_SDI_SELECT_INPUT DAISY Register"]
    pub LPSPI1_IPP_IND_LPSPI_SDI_SELECT_INPUT: crate::RWRegister<u32>,
    #[doc = "LPSPI1_IPP_IND_LPSPI_SDO_SELECT_INPUT DAISY Register"]
    pub LPSPI1_IPP_IND_LPSPI_SDO_SELECT_INPUT: crate::RWRegister<u32>,
    #[doc = "LPSPI2_IPP_IND_LPSPI_PCS_SELECT_INPUT_0 DAISY Register"]
    pub LPSPI2_IPP_IND_LPSPI_PCS_SELECT_INPUT_0: crate::RWRegister<u32>,
    #[doc = "LPSPI2_IPP_IND_LPSPI_SCK_SELECT_INPUT DAISY Register"]
    pub LPSPI2_IPP_IND_LPSPI_SCK_SELECT_INPUT: crate::RWRegister<u32>,
    #[doc = "LPSPI2_IPP_IND_LPSPI_SDI_SELECT_INPUT DAISY Register"]
    pub LPSPI2_IPP_IND_LPSPI_SDI_SELECT_INPUT: crate::RWRegister<u32>,
    #[doc = "LPSPI2_IPP_IND_LPSPI_SDO_SELECT_INPUT DAISY Register"]
    pub LPSPI2_IPP_IND_LPSPI_SDO_SELECT_INPUT: crate::RWRegister<u32>,
    _reserved15: [u8; 0x10],
    #[doc = "LPUART2_IPP_IND_LPUART_CTS_B_SELECT_INPUT DAISY Register"]
    pub LPUART2_IPP_IND_LPUART_CTS_B_SELECT_INPUT: crate::RWRegister<u32>,
    #[doc = "LPUART2_IPP_IND_LPUART_RXD_SELECT_INPUT DAISY Register"]
    pub LPUART2_IPP_IND_LPUART_RXD_SELECT_INPUT: crate::RWRegister<u32>,
    #[doc = "LPUART2_IPP_IND_LPUART_TXD_SELECT_INPUT DAISY Register"]
    pub LPUART2_IPP_IND_LPUART_TXD_SELECT_INPUT: crate::RWRegister<u32>,
    #[doc = "LPUART3_IPP_IND_LPUART_RXD_SELECT_INPUT DAISY Register"]
    pub LPUART3_IPP_IND_LPUART_RXD_SELECT_INPUT: crate::RWRegister<u32>,
    #[doc = "LPUART3_IPP_IND_LPUART_TXD_SELECT_INPUT DAISY Register"]
    pub LPUART3_IPP_IND_LPUART_TXD_SELECT_INPUT: crate::RWRegister<u32>,
    #[doc = "LPUART4_IPP_IND_LPUART_CTS_B_SELECT_INPUT DAISY Register"]
    pub LPUART4_IPP_IND_LPUART_CTS_B_SELECT_INPUT: crate::RWRegister<u32>,
    #[doc = "LPUART4_IPP_IND_LPUART_RXD_SELECT_INPUT DAISY Register"]
    pub LPUART4_IPP_IND_LPUART_RXD_SELECT_INPUT: crate::RWRegister<u32>,
    #[doc = "LPUART4_IPP_IND_LPUART_TXD_SELECT_INPUT DAISY Register"]
    pub LPUART4_IPP_IND_LPUART_TXD_SELECT_INPUT: crate::RWRegister<u32>,
    _reserved16: [u8; 0x20],
    #[doc = "NMI_GLUE_IPP_IND_NMI_SELECT_INPUT DAISY Register"]
    pub NMI_GLUE_IPP_IND_NMI_SELECT_INPUT: crate::RWRegister<u32>,
    #[doc = "QTIMER1_TMR0_INPUT_SELECT_INPUT DAISY Register"]
    pub QTIMER1_TMR0_INPUT_SELECT_INPUT: crate::RWRegister<u32>,
    #[doc = "QTIMER1_TMR1_INPUT_SELECT_INPUT DAISY Register"]
    pub QTIMER1_TMR1_INPUT_SELECT_INPUT: crate::RWRegister<u32>,
    #[doc = "QTIMER1_TMR2_INPUT_SELECT_INPUT DAISY Register"]
    pub QTIMER1_TMR2_INPUT_SELECT_INPUT: crate::RWRegister<u32>,
    #[doc = "QTIMER1_TMR3_INPUT_SELECT_INPUT DAISY Register"]
    pub QTIMER1_TMR3_INPUT_SELECT_INPUT: crate::RWRegister<u32>,
    _reserved17: [u8; 0x10],
    #[doc = "SAI1_IPG_CLK_SAI_MCLK_SELECT_INPUT_2 DAISY Register"]
    pub SAI1_IPG_CLK_SAI_MCLK_SELECT_INPUT_2: crate::RWRegister<u32>,
    #[doc = "SAI1_IPP_IND_SAI_RXBCLK_SELECT_INPUT DAISY Register"]
    pub SAI1_IPP_IND_SAI_RXBCLK_SELECT_INPUT: crate::RWRegister<u32>,
    #[doc = "SAI1_IPP_IND_SAI_RXDATA_SELECT_INPUT_0 DAISY Register"]
    pub SAI1_IPP_IND_SAI_RXDATA_SELECT_INPUT_0: crate::RWRegister<u32>,
    #[doc = "SAI1_IPP_IND_SAI_RXDATA_SELECT_INPUT_1 DAISY Register"]
    pub SAI1_IPP_IND_SAI_RXDATA_SELECT_INPUT_1: crate::RWRegister<u32>,
    #[doc = "SAI1_IPP_IND_SAI_RXDATA_SELECT_INPUT_2 DAISY Register"]
    pub SAI1_IPP_IND_SAI_RXDATA_SELECT_INPUT_2: crate::RWRegister<u32>,
    #[doc = "SAI1_IPP_IND_SAI_RXDATA_SELECT_INPUT_3 DAISY Register"]
    pub SAI1_IPP_IND_SAI_RXDATA_SELECT_INPUT_3: crate::RWRegister<u32>,
    #[doc = "SAI1_IPP_IND_SAI_RXSYNC_SELECT_INPUT DAISY Register"]
    pub SAI1_IPP_IND_SAI_RXSYNC_SELECT_INPUT: crate::RWRegister<u32>,
    #[doc = "SAI1_IPP_IND_SAI_TXBCLK_SELECT_INPUT DAISY Register"]
    pub SAI1_IPP_IND_SAI_TXBCLK_SELECT_INPUT: crate::RWRegister<u32>,
    #[doc = "SAI1_IPP_IND_SAI_TXSYNC_SELECT_INPUT DAISY Register"]
    pub SAI1_IPP_IND_SAI_TXSYNC_SELECT_INPUT: crate::RWRegister<u32>,
    #[doc = "SAI2_IPG_CLK_SAI_MCLK_SELECT_INPUT_2 DAISY Register"]
    pub SAI2_IPG_CLK_SAI_MCLK_SELECT_INPUT_2: crate::RWRegister<u32>,
    #[doc = "SAI2_IPP_IND_SAI_RXBCLK_SELECT_INPUT DAISY Register"]
    pub SAI2_IPP_IND_SAI_RXBCLK_SELECT_INPUT: crate::RWRegister<u32>,
    #[doc = "SAI2_IPP_IND_SAI_RXDATA_SELECT_INPUT_0 DAISY Register"]
    pub SAI2_IPP_IND_SAI_RXDATA_SELECT_INPUT_0: crate::RWRegister<u32>,
    #[doc = "SAI2_IPP_IND_SAI_RXSYNC_SELECT_INPUT DAISY Register"]
    pub SAI2_IPP_IND_SAI_RXSYNC_SELECT_INPUT: crate::RWRegister<u32>,
    #[doc = "SAI2_IPP_IND_SAI_TXBCLK_SELECT_INPUT DAISY Register"]
    pub SAI2_IPP_IND_SAI_TXBCLK_SELECT_INPUT: crate::RWRegister<u32>,
    #[doc = "SAI2_IPP_IND_SAI_TXSYNC_SELECT_INPUT DAISY Register"]
    pub SAI2_IPP_IND_SAI_TXSYNC_SELECT_INPUT: crate::RWRegister<u32>,
    #[doc = "SAI3_IPG_CLK_SAI_MCLK_SELECT_INPUT_2 DAISY Register"]
    pub SAI3_IPG_CLK_SAI_MCLK_SELECT_INPUT_2: crate::RWRegister<u32>,
    #[doc = "SAI3_IPP_IND_SAI_RXBCLK_SELECT_INPUT DAISY Register"]
    pub SAI3_IPP_IND_SAI_RXBCLK_SELECT_INPUT: crate::RWRegister<u32>,
    #[doc = "SAI3_IPP_IND_SAI_RXDATA_SELECT_INPUT_0 DAISY Register"]
    pub SAI3_IPP_IND_SAI_RXDATA_SELECT_INPUT_0: crate::RWRegister<u32>,
    #[doc = "SAI3_IPP_IND_SAI_RXSYNC_SELECT_INPUT DAISY Register"]
    pub SAI3_IPP_IND_SAI_RXSYNC_SELECT_INPUT: crate::RWRegister<u32>,
    #[doc = "SAI3_IPP_IND_SAI_TXBCLK_SELECT_INPUT DAISY Register"]
    pub SAI3_IPP_IND_SAI_TXBCLK_SELECT_INPUT: crate::RWRegister<u32>,
    #[doc = "SAI3_IPP_IND_SAI_TXSYNC_SELECT_INPUT DAISY Register"]
    pub SAI3_IPP_IND_SAI_TXSYNC_SELECT_INPUT: crate::RWRegister<u32>,
    _reserved18: [u8; 0x04],
    #[doc = "SPDIF_SPDIF_IN1_SELECT_INPUT DAISY Register"]
    pub SPDIF_SPDIF_IN1_SELECT_INPUT: crate::RWRegister<u32>,
    #[doc = "USB_IPP_IND_OTG_OC_SELECT_INPUT DAISY Register"]
    pub USB_IPP_IND_OTG_OC_SELECT_INPUT: crate::RWRegister<u32>,
    _reserved19: [u8; 0x10],
    #[doc = "XBAR1_XBAR_IN_SELECT_INPUT_14 DAISY Register"]
    pub XBAR1_XBAR_IN_SELECT_INPUT_14: crate::RWRegister<u32>,
    #[doc = "XBAR1_XBAR_IN_SELECT_INPUT_15 DAISY Register"]
    pub XBAR1_XBAR_IN_SELECT_INPUT_15: crate::RWRegister<u32>,
    #[doc = "XBAR1_XBAR_IN_SELECT_INPUT_16 DAISY Register"]
    pub XBAR1_XBAR_IN_SELECT_INPUT_16: crate::RWRegister<u32>,
    #[doc = "XBAR1_XBAR_IN_SELECT_INPUT_17 DAISY Register"]
    pub XBAR1_XBAR_IN_SELECT_INPUT_17: crate::RWRegister<u32>,
    #[doc = "XBAR1_XBAR_IN_SELECT_INPUT_10 DAISY Register"]
    pub XBAR1_XBAR_IN_SELECT_INPUT_10: crate::RWRegister<u32>,
    #[doc = "XBAR1_XBAR_IN_SELECT_INPUT_12 DAISY Register"]
    pub XBAR1_XBAR_IN_SELECT_INPUT_12: crate::RWRegister<u32>,
    #[doc = "XBAR1_XBAR_IN_SELECT_INPUT_13 DAISY Register"]
    pub XBAR1_XBAR_IN_SELECT_INPUT_13: crate::RWRegister<u32>,
    #[doc = "XBAR1_XBAR_IN_SELECT_INPUT_18 DAISY Register"]
    pub XBAR1_XBAR_IN_SELECT_INPUT_18: crate::RWRegister<u32>,
    #[doc = "XBAR1_XBAR_IN_SELECT_INPUT_19 DAISY Register"]
    pub XBAR1_XBAR_IN_SELECT_INPUT_19: crate::RWRegister<u32>,
}
#[doc = "SW_MUX_CTL_PAD_GPIO_EMC_04 SW MUX Control Register"]
pub mod SW_MUX_CTL_PAD_GPIO_EMC_04 {
    #[doc = "MUX Mode Select Field."]
    pub mod MUX_MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Select mux mode: ALT1 mux port: XBAR1_XBAR_INOUT04 of instance: xbar1"]
            pub const ALT1: u32 = 0x01;
            #[doc = "Select mux mode: ALT2 mux port: SPDIF_OUT of instance: spdif"]
            pub const ALT2: u32 = 0x02;
            #[doc = "Select mux mode: ALT3 mux port: SAI2_TX_BCLK of instance: sai2"]
            pub const ALT3: u32 = 0x03;
            #[doc = "Select mux mode: ALT4 mux port: FLEXIO1_FLEXIO16 of instance: flexio1"]
            pub const ALT4: u32 = 0x04;
            #[doc = "Select mux mode: ALT5 mux port: GPIO2_IO04 of instance: gpio2"]
            pub const ALT5: u32 = 0x05;
            #[doc = "Select mux mode: ALT7 mux port: SJC_JTAG_ACT of instance: sjc"]
            pub const ALT7: u32 = 0x07;
        }
    }
    #[doc = "Software Input On Field."]
    pub mod SION {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Input Path is determined by functionality"]
            pub const DISABLED: u32 = 0;
            #[doc = "Force input path of pad GPIO_EMC_04"]
            pub const ENABLED: u32 = 0x01;
        }
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_EMC_05 SW MUX Control Register"]
pub mod SW_MUX_CTL_PAD_GPIO_EMC_05 {
    #[doc = "MUX Mode Select Field."]
    pub mod MUX_MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Select mux mode: ALT1 mux port: XBAR1_XBAR_INOUT05 of instance: xbar1"]
            pub const ALT1: u32 = 0x01;
            #[doc = "Select mux mode: ALT2 mux port: SPDIF_IN of instance: spdif"]
            pub const ALT2: u32 = 0x02;
            #[doc = "Select mux mode: ALT3 mux port: SAI2_TX_SYNC of instance: sai2"]
            pub const ALT3: u32 = 0x03;
            #[doc = "Select mux mode: ALT4 mux port: FLEXIO1_FLEXIO17 of instance: flexio1"]
            pub const ALT4: u32 = 0x04;
            #[doc = "Select mux mode: ALT5 mux port: GPIO2_IO05 of instance: gpio2"]
            pub const ALT5: u32 = 0x05;
            #[doc = "Select mux mode: ALT7 mux port: SJC_DE_B of instance: sjc"]
            pub const ALT7: u32 = 0x07;
        }
    }
    #[doc = "Software Input On Field."]
    pub mod SION {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Input Path is determined by functionality"]
            pub const DISABLED: u32 = 0;
            #[doc = "Force input path of pad GPIO_EMC_05"]
            pub const ENABLED: u32 = 0x01;
        }
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_EMC_06 SW MUX Control Register"]
pub mod SW_MUX_CTL_PAD_GPIO_EMC_06 {
    #[doc = "MUX Mode Select Field."]
    pub mod MUX_MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Select mux mode: ALT1 mux port: XBAR1_XBAR_INOUT06 of instance: xbar1"]
            pub const ALT1: u32 = 0x01;
            #[doc = "Select mux mode: ALT2 mux port: LPUART3_TX of instance: lpuart3"]
            pub const ALT2: u32 = 0x02;
            #[doc = "Select mux mode: ALT3 mux port: SAI2_TX_DATA of instance: sai2"]
            pub const ALT3: u32 = 0x03;
            #[doc = "Select mux mode: ALT4 mux port: FLEXIO1_FLEXIO18 of instance: flexio1"]
            pub const ALT4: u32 = 0x04;
            #[doc = "Select mux mode: ALT5 mux port: GPIO2_IO06 of instance: gpio2"]
            pub const ALT5: u32 = 0x05;
        }
    }
    #[doc = "Software Input On Field."]
    pub mod SION {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Input Path is determined by functionality"]
            pub const DISABLED: u32 = 0;
            #[doc = "Force input path of pad GPIO_EMC_06"]
            pub const ENABLED: u32 = 0x01;
        }
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_EMC_07 SW MUX Control Register"]
pub mod SW_MUX_CTL_PAD_GPIO_EMC_07 {
    #[doc = "MUX Mode Select Field."]
    pub mod MUX_MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Select mux mode: ALT1 mux port: XBAR1_XBAR_INOUT07 of instance: xbar1"]
            pub const ALT1: u32 = 0x01;
            #[doc = "Select mux mode: ALT2 mux port: LPUART3_RX of instance: lpuart3"]
            pub const ALT2: u32 = 0x02;
            #[doc = "Select mux mode: ALT3 mux port: SAI2_RX_SYNC of instance: sai2"]
            pub const ALT3: u32 = 0x03;
            #[doc = "Select mux mode: ALT4 mux port: FLEXIO1_FLEXIO19 of instance: flexio1"]
            pub const ALT4: u32 = 0x04;
            #[doc = "Select mux mode: ALT5 mux port: GPIO2_IO07 of instance: gpio2"]
            pub const ALT5: u32 = 0x05;
        }
    }
    #[doc = "Software Input On Field."]
    pub mod SION {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Input Path is determined by functionality"]
            pub const DISABLED: u32 = 0;
            #[doc = "Force input path of pad GPIO_EMC_07"]
            pub const ENABLED: u32 = 0x01;
        }
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_EMC_08 SW MUX Control Register"]
pub mod SW_MUX_CTL_PAD_GPIO_EMC_08 {
    #[doc = "MUX Mode Select Field."]
    pub mod MUX_MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Select mux mode: ALT1 mux port: XBAR1_XBAR_INOUT08 of instance: xbar1"]
            pub const ALT1: u32 = 0x01;
            #[doc = "Select mux mode: ALT3 mux port: SAI2_RX_DATA of instance: sai2"]
            pub const ALT3: u32 = 0x03;
            #[doc = "Select mux mode: ALT4 mux port: FLEXIO1_FLEXIO20 of instance: flexio1"]
            pub const ALT4: u32 = 0x04;
            #[doc = "Select mux mode: ALT5 mux port: GPIO2_IO08 of instance: gpio2"]
            pub const ALT5: u32 = 0x05;
        }
    }
    #[doc = "Software Input On Field."]
    pub mod SION {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Input Path is determined by functionality"]
            pub const DISABLED: u32 = 0;
            #[doc = "Force input path of pad GPIO_EMC_08"]
            pub const ENABLED: u32 = 0x01;
        }
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_EMC_09 SW MUX Control Register"]
pub mod SW_MUX_CTL_PAD_GPIO_EMC_09 {
    #[doc = "MUX Mode Select Field."]
    pub mod MUX_MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Select mux mode: ALT1 mux port: XBAR1_XBAR_INOUT09 of instance: xbar1"]
            pub const ALT1: u32 = 0x01;
            #[doc = "Select mux mode: ALT3 mux port: SAI2_RX_BCLK of instance: sai2"]
            pub const ALT3: u32 = 0x03;
            #[doc = "Select mux mode: ALT4 mux port: FLEXIO1_FLEXIO21 of instance: flexio1"]
            pub const ALT4: u32 = 0x04;
            #[doc = "Select mux mode: ALT5 mux port: GPIO2_IO09 of instance: gpio2"]
            pub const ALT5: u32 = 0x05;
        }
    }
    #[doc = "Software Input On Field."]
    pub mod SION {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Input Path is determined by functionality"]
            pub const DISABLED: u32 = 0;
            #[doc = "Force input path of pad GPIO_EMC_09"]
            pub const ENABLED: u32 = 0x01;
        }
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_EMC_16 SW MUX Control Register"]
pub mod SW_MUX_CTL_PAD_GPIO_EMC_16 {
    #[doc = "MUX Mode Select Field."]
    pub mod MUX_MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Select mux mode: ALT2 mux port: MQS_RIGHT of instance: mqs"]
            pub const ALT2: u32 = 0x02;
            #[doc = "Select mux mode: ALT3 mux port: SAI2_MCLK of instance: sai2"]
            pub const ALT3: u32 = 0x03;
            #[doc = "Select mux mode: ALT5 mux port: GPIO2_IO16 of instance: gpio2"]
            pub const ALT5: u32 = 0x05;
            #[doc = "Select mux mode: ALT6 mux port: SRC_BOOT_MODE00 of instance: src"]
            pub const ALT6: u32 = 0x06;
        }
    }
    #[doc = "Software Input On Field."]
    pub mod SION {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Input Path is determined by functionality"]
            pub const DISABLED: u32 = 0;
            #[doc = "Force input path of pad GPIO_EMC_16"]
            pub const ENABLED: u32 = 0x01;
        }
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_EMC_17 SW MUX Control Register"]
pub mod SW_MUX_CTL_PAD_GPIO_EMC_17 {
    #[doc = "MUX Mode Select Field."]
    pub mod MUX_MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Select mux mode: ALT2 mux port: MQS_LEFT of instance: mqs"]
            pub const ALT2: u32 = 0x02;
            #[doc = "Select mux mode: ALT3 mux port: SAI3_MCLK of instance: sai3"]
            pub const ALT3: u32 = 0x03;
            #[doc = "Select mux mode: ALT5 mux port: GPIO2_IO17 of instance: gpio2"]
            pub const ALT5: u32 = 0x05;
            #[doc = "Select mux mode: ALT6 mux port: SRC_BOOT_MODE01 of instance: src"]
            pub const ALT6: u32 = 0x06;
        }
    }
    #[doc = "Software Input On Field."]
    pub mod SION {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Input Path is determined by functionality"]
            pub const DISABLED: u32 = 0;
            #[doc = "Force input path of pad GPIO_EMC_17"]
            pub const ENABLED: u32 = 0x01;
        }
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_EMC_18 SW MUX Control Register"]
pub mod SW_MUX_CTL_PAD_GPIO_EMC_18 {
    #[doc = "MUX Mode Select Field."]
    pub mod MUX_MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Select mux mode: ALT1 mux port: XBAR1_XBAR_INOUT16 of instance: xbar1"]
            pub const ALT1: u32 = 0x01;
            #[doc = "Select mux mode: ALT2 mux port: LPI2C2_SDA of instance: lpi2c2"]
            pub const ALT2: u32 = 0x02;
            #[doc = "Select mux mode: ALT3 mux port: SAI1_RX_SYNC of instance: sai1"]
            pub const ALT3: u32 = 0x03;
            #[doc = "Select mux mode: ALT4 mux port: FLEXIO1_FLEXIO22 of instance: flexio1"]
            pub const ALT4: u32 = 0x04;
            #[doc = "Select mux mode: ALT5 mux port: GPIO2_IO18 of instance: gpio2"]
            pub const ALT5: u32 = 0x05;
            #[doc = "Select mux mode: ALT6 mux port: SRC_BT_CFG00 of instance: src"]
            pub const ALT6: u32 = 0x06;
        }
    }
    #[doc = "Software Input On Field."]
    pub mod SION {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Input Path is determined by functionality"]
            pub const DISABLED: u32 = 0;
            #[doc = "Force input path of pad GPIO_EMC_18"]
            pub const ENABLED: u32 = 0x01;
        }
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_EMC_19 SW MUX Control Register"]
pub mod SW_MUX_CTL_PAD_GPIO_EMC_19 {
    #[doc = "MUX Mode Select Field."]
    pub mod MUX_MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Select mux mode: ALT1 mux port: XBAR1_XBAR_INOUT17 of instance: xbar1"]
            pub const ALT1: u32 = 0x01;
            #[doc = "Select mux mode: ALT2 mux port: LPI2C2_SCL of instance: lpi2c2"]
            pub const ALT2: u32 = 0x02;
            #[doc = "Select mux mode: ALT3 mux port: SAI1_RX_BCLK of instance: sai1"]
            pub const ALT3: u32 = 0x03;
            #[doc = "Select mux mode: ALT4 mux port: FLEXIO1_FLEXIO23 of instance: flexio1"]
            pub const ALT4: u32 = 0x04;
            #[doc = "Select mux mode: ALT5 mux port: GPIO2_IO19 of instance: gpio2"]
            pub const ALT5: u32 = 0x05;
            #[doc = "Select mux mode: ALT6 mux port: SRC_BT_CFG01 of instance: src"]
            pub const ALT6: u32 = 0x06;
        }
    }
    #[doc = "Software Input On Field."]
    pub mod SION {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Input Path is determined by functionality"]
            pub const DISABLED: u32 = 0;
            #[doc = "Force input path of pad GPIO_EMC_19"]
            pub const ENABLED: u32 = 0x01;
        }
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_EMC_20 SW MUX Control Register"]
pub mod SW_MUX_CTL_PAD_GPIO_EMC_20 {
    #[doc = "MUX Mode Select Field."]
    pub mod MUX_MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Select mux mode: ALT1 mux port: FLEXPWM1_PWMA03 of instance: flexpwm1"]
            pub const ALT1: u32 = 0x01;
            #[doc = "Select mux mode: ALT2 mux port: LPUART2_CTS_B of instance: lpuart2"]
            pub const ALT2: u32 = 0x02;
            #[doc = "Select mux mode: ALT3 mux port: SAI1_MCLK of instance: sai1"]
            pub const ALT3: u32 = 0x03;
            #[doc = "Select mux mode: ALT4 mux port: FLEXIO1_FLEXIO24 of instance: flexio1"]
            pub const ALT4: u32 = 0x04;
            #[doc = "Select mux mode: ALT5 mux port: GPIO2_IO20 of instance: gpio2"]
            pub const ALT5: u32 = 0x05;
            #[doc = "Select mux mode: ALT6 mux port: SRC_BT_CFG02 of instance: src"]
            pub const ALT6: u32 = 0x06;
        }
    }
    #[doc = "Software Input On Field."]
    pub mod SION {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Input Path is determined by functionality"]
            pub const DISABLED: u32 = 0;
            #[doc = "Force input path of pad GPIO_EMC_20"]
            pub const ENABLED: u32 = 0x01;
        }
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_EMC_21 SW MUX Control Register"]
pub mod SW_MUX_CTL_PAD_GPIO_EMC_21 {
    #[doc = "MUX Mode Select Field."]
    pub mod MUX_MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Select mux mode: ALT1 mux port: FLEXPWM1_PWMB03 of instance: flexpwm1"]
            pub const ALT1: u32 = 0x01;
            #[doc = "Select mux mode: ALT2 mux port: LPUART2_RTS_B of instance: lpuart2"]
            pub const ALT2: u32 = 0x02;
            #[doc = "Select mux mode: ALT3 mux port: SAI1_RX_DATA00 of instance: sai1"]
            pub const ALT3: u32 = 0x03;
            #[doc = "Select mux mode: ALT4 mux port: FLEXIO1_FLEXIO25 of instance: flexio1"]
            pub const ALT4: u32 = 0x04;
            #[doc = "Select mux mode: ALT5 mux port: GPIO2_IO21 of instance: gpio2"]
            pub const ALT5: u32 = 0x05;
            #[doc = "Select mux mode: ALT6 mux port: SRC_BT_CFG03 of instance: src"]
            pub const ALT6: u32 = 0x06;
        }
    }
    #[doc = "Software Input On Field."]
    pub mod SION {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Input Path is determined by functionality"]
            pub const DISABLED: u32 = 0;
            #[doc = "Force input path of pad GPIO_EMC_21"]
            pub const ENABLED: u32 = 0x01;
        }
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_EMC_22 SW MUX Control Register"]
pub mod SW_MUX_CTL_PAD_GPIO_EMC_22 {
    #[doc = "MUX Mode Select Field."]
    pub mod MUX_MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Select mux mode: ALT1 mux port: FLEXPWM1_PWMA02 of instance: flexpwm1"]
            pub const ALT1: u32 = 0x01;
            #[doc = "Select mux mode: ALT2 mux port: LPUART2_TX of instance: lpuart2"]
            pub const ALT2: u32 = 0x02;
            #[doc = "Select mux mode: ALT3 mux port: SAI1_TX_DATA03 of instance: sai1"]
            pub const ALT3: u32 = 0x03;
            #[doc = "Select mux mode: ALT4 mux port: FLEXIO1_FLEXIO26 of instance: flexio1"]
            pub const ALT4: u32 = 0x04;
            #[doc = "Select mux mode: ALT5 mux port: GPIO2_IO22 of instance: gpio2"]
            pub const ALT5: u32 = 0x05;
            #[doc = "Select mux mode: ALT6 mux port: SRC_BT_CFG04 of instance: src"]
            pub const ALT6: u32 = 0x06;
        }
    }
    #[doc = "Software Input On Field."]
    pub mod SION {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Input Path is determined by functionality"]
            pub const DISABLED: u32 = 0;
            #[doc = "Force input path of pad GPIO_EMC_22"]
            pub const ENABLED: u32 = 0x01;
        }
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_EMC_23 SW MUX Control Register"]
pub mod SW_MUX_CTL_PAD_GPIO_EMC_23 {
    #[doc = "MUX Mode Select Field."]
    pub mod MUX_MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Select mux mode: ALT1 mux port: FLEXPWM1_PWMB02 of instance: flexpwm1"]
            pub const ALT1: u32 = 0x01;
            #[doc = "Select mux mode: ALT2 mux port: LPUART2_RX of instance: lpuart2"]
            pub const ALT2: u32 = 0x02;
            #[doc = "Select mux mode: ALT3 mux port: SAI1_TX_DATA02 of instance: sai1"]
            pub const ALT3: u32 = 0x03;
            #[doc = "Select mux mode: ALT4 mux port: FLEXIO1_FLEXIO27 of instance: flexio1"]
            pub const ALT4: u32 = 0x04;
            #[doc = "Select mux mode: ALT5 mux port: GPIO2_IO23 of instance: gpio2"]
            pub const ALT5: u32 = 0x05;
            #[doc = "Select mux mode: ALT6 mux port: SRC_BT_CFG05 of instance: src"]
            pub const ALT6: u32 = 0x06;
        }
    }
    #[doc = "Software Input On Field."]
    pub mod SION {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Input Path is determined by functionality"]
            pub const DISABLED: u32 = 0;
            #[doc = "Force input path of pad GPIO_EMC_23"]
            pub const ENABLED: u32 = 0x01;
        }
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_EMC_24 SW MUX Control Register"]
pub mod SW_MUX_CTL_PAD_GPIO_EMC_24 {
    #[doc = "MUX Mode Select Field."]
    pub mod MUX_MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Select mux mode: ALT1 mux port: FLEXPWM1_PWMA01 of instance: flexpwm1"]
            pub const ALT1: u32 = 0x01;
            #[doc = "Select mux mode: ALT3 mux port: SAI1_TX_DATA01 of instance: sai1"]
            pub const ALT3: u32 = 0x03;
            #[doc = "Select mux mode: ALT4 mux port: FLEXIO1_FLEXIO28 of instance: flexio1"]
            pub const ALT4: u32 = 0x04;
            #[doc = "Select mux mode: ALT5 mux port: GPIO2_IO24 of instance: gpio2"]
            pub const ALT5: u32 = 0x05;
            #[doc = "Select mux mode: ALT6 mux port: SRC_BT_CFG06 of instance: src"]
            pub const ALT6: u32 = 0x06;
        }
    }
    #[doc = "Software Input On Field."]
    pub mod SION {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Input Path is determined by functionality"]
            pub const DISABLED: u32 = 0;
            #[doc = "Force input path of pad GPIO_EMC_24"]
            pub const ENABLED: u32 = 0x01;
        }
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_EMC_25 SW MUX Control Register"]
pub mod SW_MUX_CTL_PAD_GPIO_EMC_25 {
    #[doc = "MUX Mode Select Field."]
    pub mod MUX_MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Select mux mode: ALT1 mux port: FLEXPWM1_PWMB01 of instance: flexpwm1"]
            pub const ALT1: u32 = 0x01;
            #[doc = "Select mux mode: ALT3 mux port: SAI1_TX_DATA00 of instance: sai1"]
            pub const ALT3: u32 = 0x03;
            #[doc = "Select mux mode: ALT4 mux port: FLEXIO1_FLEXIO29 of instance: flexio1"]
            pub const ALT4: u32 = 0x04;
            #[doc = "Select mux mode: ALT5 mux port: GPIO2_IO25 of instance: gpio2"]
            pub const ALT5: u32 = 0x05;
            #[doc = "Select mux mode: ALT6 mux port: SRC_BT_CFG07 of instance: src"]
            pub const ALT6: u32 = 0x06;
        }
    }
    #[doc = "Software Input On Field."]
    pub mod SION {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Input Path is determined by functionality"]
            pub const DISABLED: u32 = 0;
            #[doc = "Force input path of pad GPIO_EMC_25"]
            pub const ENABLED: u32 = 0x01;
        }
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_EMC_26 SW MUX Control Register"]
pub mod SW_MUX_CTL_PAD_GPIO_EMC_26 {
    #[doc = "MUX Mode Select Field."]
    pub mod MUX_MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Select mux mode: ALT1 mux port: FLEXPWM1_PWMA00 of instance: flexpwm1"]
            pub const ALT1: u32 = 0x01;
            #[doc = "Select mux mode: ALT3 mux port: SAI1_TX_BCLK of instance: sai1"]
            pub const ALT3: u32 = 0x03;
            #[doc = "Select mux mode: ALT4 mux port: FLEXIO1_FLEXIO30 of instance: flexio1"]
            pub const ALT4: u32 = 0x04;
            #[doc = "Select mux mode: ALT5 mux port: GPIO2_IO26 of instance: gpio2"]
            pub const ALT5: u32 = 0x05;
            #[doc = "Select mux mode: ALT6 mux port: SRC_BT_CFG08 of instance: src"]
            pub const ALT6: u32 = 0x06;
        }
    }
    #[doc = "Software Input On Field."]
    pub mod SION {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Input Path is determined by functionality"]
            pub const DISABLED: u32 = 0;
            #[doc = "Force input path of pad GPIO_EMC_26"]
            pub const ENABLED: u32 = 0x01;
        }
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_EMC_27 SW MUX Control Register"]
pub mod SW_MUX_CTL_PAD_GPIO_EMC_27 {
    #[doc = "MUX Mode Select Field."]
    pub mod MUX_MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Select mux mode: ALT1 mux port: FLEXPWM1_PWMB00 of instance: flexpwm1"]
            pub const ALT1: u32 = 0x01;
            #[doc = "Select mux mode: ALT3 mux port: SAI1_TX_SYNC of instance: sai1"]
            pub const ALT3: u32 = 0x03;
            #[doc = "Select mux mode: ALT4 mux port: FLEXIO1_FLEXIO31 of instance: flexio1"]
            pub const ALT4: u32 = 0x04;
            #[doc = "Select mux mode: ALT5 mux port: GPIO2_IO27 of instance: gpio2"]
            pub const ALT5: u32 = 0x05;
            #[doc = "Select mux mode: ALT6 mux port: SRC_BT_CFG09 of instance: src"]
            pub const ALT6: u32 = 0x06;
        }
    }
    #[doc = "Software Input On Field."]
    pub mod SION {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Input Path is determined by functionality"]
            pub const DISABLED: u32 = 0;
            #[doc = "Force input path of pad GPIO_EMC_27"]
            pub const ENABLED: u32 = 0x01;
        }
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_EMC_32 SW MUX Control Register"]
pub mod SW_MUX_CTL_PAD_GPIO_EMC_32 {
    #[doc = "MUX Mode Select Field."]
    pub mod MUX_MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Select mux mode: ALT1 mux port: QTIMER1_TIMER0 of instance: qtimer1"]
            pub const ALT1: u32 = 0x01;
            #[doc = "Select mux mode: ALT2 mux port: LPUART4_TX of instance: lpuart4"]
            pub const ALT2: u32 = 0x02;
            #[doc = "Select mux mode: ALT3 mux port: SAI3_TX_DATA of instance: sai3"]
            pub const ALT3: u32 = 0x03;
            #[doc = "Select mux mode: ALT5 mux port: GPIO3_IO00 of instance: gpio3"]
            pub const ALT5: u32 = 0x05;
            #[doc = "Select mux mode: ALT7 mux port: REF_24M_OUT of instance: anatop"]
            pub const ALT7: u32 = 0x07;
        }
    }
    #[doc = "Software Input On Field."]
    pub mod SION {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Input Path is determined by functionality"]
            pub const DISABLED: u32 = 0;
            #[doc = "Force input path of pad GPIO_EMC_32"]
            pub const ENABLED: u32 = 0x01;
        }
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_EMC_33 SW MUX Control Register"]
pub mod SW_MUX_CTL_PAD_GPIO_EMC_33 {
    #[doc = "MUX Mode Select Field."]
    pub mod MUX_MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Select mux mode: ALT1 mux port: QTIMER1_TIMER1 of instance: qtimer1"]
            pub const ALT1: u32 = 0x01;
            #[doc = "Select mux mode: ALT2 mux port: LPUART4_RX of instance: lpuart4"]
            pub const ALT2: u32 = 0x02;
            #[doc = "Select mux mode: ALT3 mux port: SAI3_TX_BCLK of instance: sai3"]
            pub const ALT3: u32 = 0x03;
            #[doc = "Select mux mode: ALT5 mux port: GPIO3_IO01 of instance: gpio3"]
            pub const ALT5: u32 = 0x05;
        }
    }
    #[doc = "Software Input On Field."]
    pub mod SION {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Input Path is determined by functionality"]
            pub const DISABLED: u32 = 0;
            #[doc = "Force input path of pad GPIO_EMC_33"]
            pub const ENABLED: u32 = 0x01;
        }
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_EMC_34 SW MUX Control Register"]
pub mod SW_MUX_CTL_PAD_GPIO_EMC_34 {
    #[doc = "MUX Mode Select Field."]
    pub mod MUX_MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Select mux mode: ALT1 mux port: QTIMER1_TIMER2 of instance: qtimer1"]
            pub const ALT1: u32 = 0x01;
            #[doc = "Select mux mode: ALT3 mux port: SAI3_TX_SYNC of instance: sai3"]
            pub const ALT3: u32 = 0x03;
            #[doc = "Select mux mode: ALT5 mux port: GPIO3_IO02 of instance: gpio3"]
            pub const ALT5: u32 = 0x05;
        }
    }
    #[doc = "Software Input On Field."]
    pub mod SION {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Input Path is determined by functionality"]
            pub const DISABLED: u32 = 0;
            #[doc = "Force input path of pad GPIO_EMC_34"]
            pub const ENABLED: u32 = 0x01;
        }
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_EMC_35 SW MUX Control Register"]
pub mod SW_MUX_CTL_PAD_GPIO_EMC_35 {
    #[doc = "MUX Mode Select Field."]
    pub mod MUX_MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Select mux mode: ALT1 mux port: QTIMER1_TIMER3 of instance: qtimer1"]
            pub const ALT1: u32 = 0x01;
            #[doc = "Select mux mode: ALT5 mux port: GPIO3_IO03 of instance: gpio3"]
            pub const ALT5: u32 = 0x05;
        }
    }
    #[doc = "Software Input On Field."]
    pub mod SION {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Input Path is determined by functionality"]
            pub const DISABLED: u32 = 0;
            #[doc = "Force input path of pad GPIO_EMC_35"]
            pub const ENABLED: u32 = 0x01;
        }
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_AD_B0_00 SW MUX Control Register"]
pub mod SW_MUX_CTL_PAD_GPIO_AD_B0_00 {
    #[doc = "MUX Mode Select Field."]
    pub mod MUX_MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Select mux mode: ALT0 mux port: JTAG_MUX_TMS of instance: jtag_mux"]
            pub const ALT0: u32 = 0;
            #[doc = "Select mux mode: ALT5 mux port: GPIO1_IO00 of instance: gpio1"]
            pub const ALT5: u32 = 0x05;
            #[doc = "Select mux mode: ALT7 mux port: GPT1_COMPARE1 of instance: gpt1"]
            pub const ALT7: u32 = 0x07;
        }
    }
    #[doc = "Software Input On Field."]
    pub mod SION {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Input Path is determined by functionality"]
            pub const DISABLED: u32 = 0;
            #[doc = "Force input path of pad GPIO_AD_B0_00"]
            pub const ENABLED: u32 = 0x01;
        }
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_AD_B0_01 SW MUX Control Register"]
pub mod SW_MUX_CTL_PAD_GPIO_AD_B0_01 {
    #[doc = "MUX Mode Select Field."]
    pub mod MUX_MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Select mux mode: ALT0 mux port: JTAG_MUX_TCK of instance: jtag_mux"]
            pub const ALT0: u32 = 0;
            #[doc = "Select mux mode: ALT5 mux port: GPIO1_IO01 of instance: gpio1"]
            pub const ALT5: u32 = 0x05;
            #[doc = "Select mux mode: ALT7 mux port: GPT1_CAPTURE2 of instance: gpt1"]
            pub const ALT7: u32 = 0x07;
        }
    }
    #[doc = "Software Input On Field."]
    pub mod SION {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Input Path is determined by functionality"]
            pub const DISABLED: u32 = 0;
            #[doc = "Force input path of pad GPIO_AD_B0_01"]
            pub const ENABLED: u32 = 0x01;
        }
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_AD_B0_02 SW MUX Control Register"]
pub mod SW_MUX_CTL_PAD_GPIO_AD_B0_02 {
    #[doc = "MUX Mode Select Field."]
    pub mod MUX_MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Select mux mode: ALT0 mux port: JTAG_MUX_MOD of instance: jtag_mux"]
            pub const ALT0: u32 = 0;
            #[doc = "Select mux mode: ALT5 mux port: GPIO1_IO02 of instance: gpio1"]
            pub const ALT5: u32 = 0x05;
            #[doc = "Select mux mode: ALT7 mux port: GPT1_CAPTURE1 of instance: gpt1"]
            pub const ALT7: u32 = 0x07;
        }
    }
    #[doc = "Software Input On Field."]
    pub mod SION {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Input Path is determined by functionality"]
            pub const DISABLED: u32 = 0;
            #[doc = "Force input path of pad GPIO_AD_B0_02"]
            pub const ENABLED: u32 = 0x01;
        }
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_AD_B0_03 SW MUX Control Register"]
pub mod SW_MUX_CTL_PAD_GPIO_AD_B0_03 {
    #[doc = "MUX Mode Select Field."]
    pub mod MUX_MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Select mux mode: ALT0 mux port: JTAG_MUX_TDI of instance: jtag_mux"]
            pub const ALT0: u32 = 0;
            #[doc = "Select mux mode: ALT2 mux port: WDOG1_WDOG_B of instance: wdog1"]
            pub const ALT2: u32 = 0x02;
            #[doc = "Select mux mode: ALT3 mux port: SAI1_MCLK of instance: sai1"]
            pub const ALT3: u32 = 0x03;
            #[doc = "Select mux mode: ALT5 mux port: GPIO1_IO03 of instance: gpio1"]
            pub const ALT5: u32 = 0x05;
            #[doc = "Select mux mode: ALT6 mux port: USB_OTG1_OC of instance: usb"]
            pub const ALT6: u32 = 0x06;
            #[doc = "Select mux mode: ALT7 mux port: CCM_PMIC_RDY of instance: ccm"]
            pub const ALT7: u32 = 0x07;
        }
    }
    #[doc = "Software Input On Field."]
    pub mod SION {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Input Path is determined by functionality"]
            pub const DISABLED: u32 = 0;
            #[doc = "Force input path of pad GPIO_AD_B0_03"]
            pub const ENABLED: u32 = 0x01;
        }
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_AD_B0_04 SW MUX Control Register"]
pub mod SW_MUX_CTL_PAD_GPIO_AD_B0_04 {
    #[doc = "MUX Mode Select Field."]
    pub mod MUX_MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Select mux mode: ALT0 mux port: JTAG_MUX_TDO of instance: jtag_mux"]
            pub const ALT0: u32 = 0;
            #[doc = "Select mux mode: ALT5 mux port: GPIO1_IO04 of instance: gpio1"]
            pub const ALT5: u32 = 0x05;
            #[doc = "Select mux mode: ALT6 mux port: USB_OTG1_PWR of instance: usb"]
            pub const ALT6: u32 = 0x06;
            #[doc = "Select mux mode: ALT7 mux port: EWM_EWM_OUT_B of instance: ewm"]
            pub const ALT7: u32 = 0x07;
        }
    }
    #[doc = "Software Input On Field."]
    pub mod SION {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Input Path is determined by functionality"]
            pub const DISABLED: u32 = 0;
            #[doc = "Force input path of pad GPIO_AD_B0_04"]
            pub const ENABLED: u32 = 0x01;
        }
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_AD_B0_05 SW MUX Control Register"]
pub mod SW_MUX_CTL_PAD_GPIO_AD_B0_05 {
    #[doc = "MUX Mode Select Field."]
    pub mod MUX_MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Select mux mode: ALT0 mux port: JTAG_MUX_TRSTB of instance: jtag_mux"]
            pub const ALT0: u32 = 0;
            #[doc = "Select mux mode: ALT5 mux port: GPIO1_IO05 of instance: gpio1"]
            pub const ALT5: u32 = 0x05;
            #[doc = "Select mux mode: ALT6 mux port: USB_OTG1_ID of instance: anatop"]
            pub const ALT6: u32 = 0x06;
            #[doc = "Select mux mode: ALT7 mux port: NMI_GLUE_NMI of instance: nmi_glue"]
            pub const ALT7: u32 = 0x07;
        }
    }
    #[doc = "Software Input On Field."]
    pub mod SION {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Input Path is determined by functionality"]
            pub const DISABLED: u32 = 0;
            #[doc = "Force input path of pad GPIO_AD_B0_05"]
            pub const ENABLED: u32 = 0x01;
        }
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_AD_B0_06 SW MUX Control Register"]
pub mod SW_MUX_CTL_PAD_GPIO_AD_B0_06 {
    #[doc = "MUX Mode Select Field."]
    pub mod MUX_MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Select mux mode: ALT0 mux port: PIT_TRIGGER00 of instance: pit"]
            pub const ALT0: u32 = 0;
            #[doc = "Select mux mode: ALT1 mux port: MQS_RIGHT of instance: mqs"]
            pub const ALT1: u32 = 0x01;
            #[doc = "Select mux mode: ALT2 mux port: LPUART1_TX of instance: lpuart1"]
            pub const ALT2: u32 = 0x02;
            #[doc = "Select mux mode: ALT5 mux port: GPIO1_IO06 of instance: gpio1"]
            pub const ALT5: u32 = 0x05;
            #[doc = "Select mux mode: ALT6 mux port: REF_32K_OUT of instance: anatop"]
            pub const ALT6: u32 = 0x06;
        }
    }
    #[doc = "Software Input On Field."]
    pub mod SION {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Input Path is determined by functionality"]
            pub const DISABLED: u32 = 0;
            #[doc = "Force input path of pad GPIO_AD_B0_06"]
            pub const ENABLED: u32 = 0x01;
        }
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_AD_B0_07 SW MUX Control Register"]
pub mod SW_MUX_CTL_PAD_GPIO_AD_B0_07 {
    #[doc = "MUX Mode Select Field."]
    pub mod MUX_MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Select mux mode: ALT0 mux port: PIT_TRIGGER01 of instance: pit"]
            pub const ALT0: u32 = 0;
            #[doc = "Select mux mode: ALT1 mux port: MQS_LEFT of instance: mqs"]
            pub const ALT1: u32 = 0x01;
            #[doc = "Select mux mode: ALT2 mux port: LPUART1_RX of instance: lpuart1"]
            pub const ALT2: u32 = 0x02;
            #[doc = "Select mux mode: ALT5 mux port: GPIO1_IO07 of instance: gpio1"]
            pub const ALT5: u32 = 0x05;
            #[doc = "Select mux mode: ALT6 mux port: REF_24M_OUT of instance: anatop"]
            pub const ALT6: u32 = 0x06;
        }
    }
    #[doc = "Software Input On Field."]
    pub mod SION {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Input Path is determined by functionality"]
            pub const DISABLED: u32 = 0;
            #[doc = "Force input path of pad GPIO_AD_B0_07"]
            pub const ENABLED: u32 = 0x01;
        }
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_AD_B0_08 SW MUX Control Register"]
pub mod SW_MUX_CTL_PAD_GPIO_AD_B0_08 {
    #[doc = "MUX Mode Select Field."]
    pub mod MUX_MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Select mux mode: ALT2 mux port: LPUART1_CTS_B of instance: lpuart1"]
            pub const ALT2: u32 = 0x02;
            #[doc = "Select mux mode: ALT3 mux port: KPP_COL00 of instance: kpp"]
            pub const ALT3: u32 = 0x03;
            #[doc = "Select mux mode: ALT5 mux port: GPIO1_IO08 of instance: gpio1"]
            pub const ALT5: u32 = 0x05;
            #[doc = "Select mux mode: ALT6 mux port: ARM_CM7_TXEV of instance: cm7_mxrt"]
            pub const ALT6: u32 = 0x06;
        }
    }
    #[doc = "Software Input On Field."]
    pub mod SION {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Input Path is determined by functionality"]
            pub const DISABLED: u32 = 0;
            #[doc = "Force input path of pad GPIO_AD_B0_08"]
            pub const ENABLED: u32 = 0x01;
        }
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_AD_B0_09 SW MUX Control Register"]
pub mod SW_MUX_CTL_PAD_GPIO_AD_B0_09 {
    #[doc = "MUX Mode Select Field."]
    pub mod MUX_MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Select mux mode: ALT2 mux port: LPUART1_RTS_B of instance: lpuart1"]
            pub const ALT2: u32 = 0x02;
            #[doc = "Select mux mode: ALT3 mux port: KPP_ROW00 of instance: kpp"]
            pub const ALT3: u32 = 0x03;
            #[doc = "Select mux mode: ALT4 mux port: CSU_CSU_INT_DEB of instance: csu"]
            pub const ALT4: u32 = 0x04;
            #[doc = "Select mux mode: ALT5 mux port: GPIO1_IO09 of instance: gpio1"]
            pub const ALT5: u32 = 0x05;
            #[doc = "Select mux mode: ALT6 mux port: ARM_CM7_RXEV of instance: cm7_mxrt"]
            pub const ALT6: u32 = 0x06;
        }
    }
    #[doc = "Software Input On Field."]
    pub mod SION {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Input Path is determined by functionality"]
            pub const DISABLED: u32 = 0;
            #[doc = "Force input path of pad GPIO_AD_B0_09"]
            pub const ENABLED: u32 = 0x01;
        }
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_AD_B0_10 SW MUX Control Register"]
pub mod SW_MUX_CTL_PAD_GPIO_AD_B0_10 {
    #[doc = "MUX Mode Select Field."]
    pub mod MUX_MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Select mux mode: ALT1 mux port: LPSPI1_SCK of instance: lpspi1"]
            pub const ALT1: u32 = 0x01;
            #[doc = "Select mux mode: ALT3 mux port: KPP_COL01 of instance: kpp"]
            pub const ALT3: u32 = 0x03;
            #[doc = "Select mux mode: ALT5 mux port: GPIO1_IO10 of instance: gpio1"]
            pub const ALT5: u32 = 0x05;
        }
    }
    #[doc = "Software Input On Field."]
    pub mod SION {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Input Path is determined by functionality"]
            pub const DISABLED: u32 = 0;
            #[doc = "Force input path of pad GPIO_AD_B0_10"]
            pub const ENABLED: u32 = 0x01;
        }
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_AD_B0_11 SW MUX Control Register"]
pub mod SW_MUX_CTL_PAD_GPIO_AD_B0_11 {
    #[doc = "MUX Mode Select Field."]
    pub mod MUX_MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Select mux mode: ALT1 mux port: LPSPI1_PCS0 of instance: lpspi1"]
            pub const ALT1: u32 = 0x01;
            #[doc = "Select mux mode: ALT3 mux port: KPP_ROW01 of instance: kpp"]
            pub const ALT3: u32 = 0x03;
            #[doc = "Select mux mode: ALT5 mux port: GPIO1_IO11 of instance: gpio1"]
            pub const ALT5: u32 = 0x05;
        }
    }
    #[doc = "Software Input On Field."]
    pub mod SION {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Input Path is determined by functionality"]
            pub const DISABLED: u32 = 0;
            #[doc = "Force input path of pad GPIO_AD_B0_11"]
            pub const ENABLED: u32 = 0x01;
        }
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_AD_B0_12 SW MUX Control Register"]
pub mod SW_MUX_CTL_PAD_GPIO_AD_B0_12 {
    #[doc = "MUX Mode Select Field."]
    pub mod MUX_MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Select mux mode: ALT1 mux port: LPSPI1_SDO of instance: lpspi1"]
            pub const ALT1: u32 = 0x01;
            #[doc = "Select mux mode: ALT2 mux port: LPUART3_CTS_B of instance: lpuart3"]
            pub const ALT2: u32 = 0x02;
            #[doc = "Select mux mode: ALT3 mux port: KPP_COL02 of instance: kpp"]
            pub const ALT3: u32 = 0x03;
            #[doc = "Select mux mode: ALT5 mux port: GPIO1_IO12 of instance: gpio1"]
            pub const ALT5: u32 = 0x05;
            #[doc = "Select mux mode: ALT7 mux port: SNVS_HP_VIO_5_CTL of instance: snvs_hp"]
            pub const ALT7: u32 = 0x07;
        }
    }
    #[doc = "Software Input On Field."]
    pub mod SION {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Input Path is determined by functionality"]
            pub const DISABLED: u32 = 0;
            #[doc = "Force input path of pad GPIO_AD_B0_12"]
            pub const ENABLED: u32 = 0x01;
        }
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_AD_B0_13 SW MUX Control Register"]
pub mod SW_MUX_CTL_PAD_GPIO_AD_B0_13 {
    #[doc = "MUX Mode Select Field."]
    pub mod MUX_MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Select mux mode: ALT1 mux port: LPSPI1_SDI of instance: lpspi1"]
            pub const ALT1: u32 = 0x01;
            #[doc = "Select mux mode: ALT2 mux port: LPUART3_RTS_B of instance: lpuart3"]
            pub const ALT2: u32 = 0x02;
            #[doc = "Select mux mode: ALT3 mux port: KPP_ROW02 of instance: kpp"]
            pub const ALT3: u32 = 0x03;
            #[doc = "Select mux mode: ALT5 mux port: GPIO1_IO13 of instance: gpio1"]
            pub const ALT5: u32 = 0x05;
            #[doc = "Select mux mode: ALT7 mux port: SNVS_HP_VIO_5_B of instance: snvs_hp"]
            pub const ALT7: u32 = 0x07;
        }
    }
    #[doc = "Software Input On Field."]
    pub mod SION {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Input Path is determined by functionality"]
            pub const DISABLED: u32 = 0;
            #[doc = "Force input path of pad GPIO_AD_B0_13"]
            pub const ENABLED: u32 = 0x01;
        }
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_AD_B0_14 SW MUX Control Register"]
pub mod SW_MUX_CTL_PAD_GPIO_AD_B0_14 {
    #[doc = "MUX Mode Select Field."]
    pub mod MUX_MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Select mux mode: ALT2 mux port: LPUART3_TX of instance: lpuart3"]
            pub const ALT2: u32 = 0x02;
            #[doc = "Select mux mode: ALT3 mux port: KPP_COL03 of instance: kpp"]
            pub const ALT3: u32 = 0x03;
            #[doc = "Select mux mode: ALT5 mux port: GPIO1_IO14 of instance: gpio1"]
            pub const ALT5: u32 = 0x05;
            #[doc = "Select mux mode: ALT7 mux port: WDOG1_WDOG_ANY of instance: wdog1"]
            pub const ALT7: u32 = 0x07;
        }
    }
    #[doc = "Software Input On Field."]
    pub mod SION {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Input Path is determined by functionality"]
            pub const DISABLED: u32 = 0;
            #[doc = "Force input path of pad GPIO_AD_B0_14"]
            pub const ENABLED: u32 = 0x01;
        }
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_AD_B0_15 SW MUX Control Register"]
pub mod SW_MUX_CTL_PAD_GPIO_AD_B0_15 {
    #[doc = "MUX Mode Select Field."]
    pub mod MUX_MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Select mux mode: ALT2 mux port: LPUART3_RX of instance: lpuart3"]
            pub const ALT2: u32 = 0x02;
            #[doc = "Select mux mode: ALT3 mux port: KPP_ROW03 of instance: kpp"]
            pub const ALT3: u32 = 0x03;
            #[doc = "Select mux mode: ALT5 mux port: GPIO1_IO15 of instance: gpio1"]
            pub const ALT5: u32 = 0x05;
        }
    }
    #[doc = "Software Input On Field."]
    pub mod SION {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Input Path is determined by functionality"]
            pub const DISABLED: u32 = 0;
            #[doc = "Force input path of pad GPIO_AD_B0_15"]
            pub const ENABLED: u32 = 0x01;
        }
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_AD_B1_10 SW MUX Control Register"]
pub mod SW_MUX_CTL_PAD_GPIO_AD_B1_10 {
    #[doc = "MUX Mode Select Field."]
    pub mod MUX_MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Select mux mode: ALT0 mux port: USB_OTG1_PWR of instance: usb"]
            pub const ALT0: u32 = 0;
            #[doc = "Select mux mode: ALT1 mux port: FLEXPWM1_PWMA02 of instance: flexpwm1"]
            pub const ALT1: u32 = 0x01;
            #[doc = "Select mux mode: ALT2 mux port: LPUART4_TX of instance: lpuart4"]
            pub const ALT2: u32 = 0x02;
            #[doc = "Select mux mode: ALT4 mux port: FLEXIO1_FLEXIO05 of instance: flexio1"]
            pub const ALT4: u32 = 0x04;
            #[doc = "Select mux mode: ALT5 mux port: GPIO1_IO26 of instance: gpio1"]
            pub const ALT5: u32 = 0x05;
            #[doc = "Select mux mode: ALT6 mux port: GPT2_CAPTURE1 of instance: gpt2"]
            pub const ALT6: u32 = 0x06;
        }
    }
    #[doc = "Software Input On Field."]
    pub mod SION {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Input Path is determined by functionality"]
            pub const DISABLED: u32 = 0;
            #[doc = "Force input path of pad GPIO_AD_B1_10"]
            pub const ENABLED: u32 = 0x01;
        }
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_AD_B1_11 SW MUX Control Register"]
pub mod SW_MUX_CTL_PAD_GPIO_AD_B1_11 {
    #[doc = "MUX Mode Select Field."]
    pub mod MUX_MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Select mux mode: ALT0 mux port: USB_OTG1_ID of instance: anatop"]
            pub const ALT0: u32 = 0;
            #[doc = "Select mux mode: ALT1 mux port: FLEXPWM1_PWMB02 of instance: flexpwm1"]
            pub const ALT1: u32 = 0x01;
            #[doc = "Select mux mode: ALT2 mux port: LPUART4_RX of instance: lpuart4"]
            pub const ALT2: u32 = 0x02;
            #[doc = "Select mux mode: ALT4 mux port: FLEXIO1_FLEXIO04 of instance: flexio1"]
            pub const ALT4: u32 = 0x04;
            #[doc = "Select mux mode: ALT5 mux port: GPIO1_IO27 of instance: gpio1"]
            pub const ALT5: u32 = 0x05;
            #[doc = "Select mux mode: ALT6 mux port: GPT2_COMPARE1 of instance: gpt2"]
            pub const ALT6: u32 = 0x06;
        }
    }
    #[doc = "Software Input On Field."]
    pub mod SION {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Input Path is determined by functionality"]
            pub const DISABLED: u32 = 0;
            #[doc = "Force input path of pad GPIO_AD_B1_11"]
            pub const ENABLED: u32 = 0x01;
        }
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_AD_B1_12 SW MUX Control Register"]
pub mod SW_MUX_CTL_PAD_GPIO_AD_B1_12 {
    #[doc = "MUX Mode Select Field."]
    pub mod MUX_MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Select mux mode: ALT0 mux port: USB_OTG1_OC of instance: usb"]
            pub const ALT0: u32 = 0;
            #[doc = "Select mux mode: ALT1 mux port: ACMP_OUT00 of instance: acmp"]
            pub const ALT1: u32 = 0x01;
            #[doc = "Select mux mode: ALT4 mux port: FLEXIO1_FLEXIO03 of instance: flexio1"]
            pub const ALT4: u32 = 0x04;
            #[doc = "Select mux mode: ALT5 mux port: GPIO1_IO28 of instance: gpio1"]
            pub const ALT5: u32 = 0x05;
            #[doc = "Select mux mode: ALT6 mux port: FLEXPWM1_PWMA03 of instance: flexpwm1"]
            pub const ALT6: u32 = 0x06;
        }
    }
    #[doc = "Software Input On Field."]
    pub mod SION {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Input Path is determined by functionality"]
            pub const DISABLED: u32 = 0;
            #[doc = "Force input path of pad GPIO_AD_B1_12"]
            pub const ENABLED: u32 = 0x01;
        }
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_AD_B1_13 SW MUX Control Register"]
pub mod SW_MUX_CTL_PAD_GPIO_AD_B1_13 {
    #[doc = "MUX Mode Select Field."]
    pub mod MUX_MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Select mux mode: ALT0 mux port: LPI2C1_HREQ of instance: lpi2c1"]
            pub const ALT0: u32 = 0;
            #[doc = "Select mux mode: ALT1 mux port: ACMP_OUT01 of instance: acmp"]
            pub const ALT1: u32 = 0x01;
            #[doc = "Select mux mode: ALT4 mux port: FLEXIO1_FLEXIO02 of instance: flexio1"]
            pub const ALT4: u32 = 0x04;
            #[doc = "Select mux mode: ALT5 mux port: GPIO1_IO29 of instance: gpio1"]
            pub const ALT5: u32 = 0x05;
            #[doc = "Select mux mode: ALT6 mux port: FLEXPWM1_PWMB03 of instance: flexpwm1"]
            pub const ALT6: u32 = 0x06;
        }
    }
    #[doc = "Software Input On Field."]
    pub mod SION {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Input Path is determined by functionality"]
            pub const DISABLED: u32 = 0;
            #[doc = "Force input path of pad GPIO_AD_B1_13"]
            pub const ENABLED: u32 = 0x01;
        }
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_AD_B1_14 SW MUX Control Register"]
pub mod SW_MUX_CTL_PAD_GPIO_AD_B1_14 {
    #[doc = "MUX Mode Select Field."]
    pub mod MUX_MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Select mux mode: ALT0 mux port: LPI2C1_SCL of instance: lpi2c1"]
            pub const ALT0: u32 = 0;
            #[doc = "Select mux mode: ALT1 mux port: ACMP_OUT02 of instance: acmp"]
            pub const ALT1: u32 = 0x01;
            #[doc = "Select mux mode: ALT4 mux port: FLEXIO1_FLEXIO01 of instance: flexio1"]
            pub const ALT4: u32 = 0x04;
            #[doc = "Select mux mode: ALT5 mux port: GPIO1_IO30 of instance: gpio1"]
            pub const ALT5: u32 = 0x05;
        }
    }
    #[doc = "Software Input On Field."]
    pub mod SION {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Input Path is determined by functionality"]
            pub const DISABLED: u32 = 0;
            #[doc = "Force input path of pad GPIO_AD_B1_14"]
            pub const ENABLED: u32 = 0x01;
        }
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_AD_B1_15 SW MUX Control Register"]
pub mod SW_MUX_CTL_PAD_GPIO_AD_B1_15 {
    #[doc = "MUX Mode Select Field."]
    pub mod MUX_MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Select mux mode: ALT0 mux port: LPI2C1_SDA of instance: lpi2c1"]
            pub const ALT0: u32 = 0;
            #[doc = "Select mux mode: ALT1 mux port: ACMP_OUT03 of instance: acmp"]
            pub const ALT1: u32 = 0x01;
            #[doc = "Select mux mode: ALT4 mux port: FLEXIO1_FLEXIO00 of instance: flexio1"]
            pub const ALT4: u32 = 0x04;
            #[doc = "Select mux mode: ALT5 mux port: GPIO1_IO31 of instance: gpio1"]
            pub const ALT5: u32 = 0x05;
            #[doc = "Select mux mode: ALT6 mux port: CCM_DI0_EXT_CLK of instance: ccm"]
            pub const ALT6: u32 = 0x06;
        }
    }
    #[doc = "Software Input On Field."]
    pub mod SION {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Input Path is determined by functionality"]
            pub const DISABLED: u32 = 0;
            #[doc = "Force input path of pad GPIO_AD_B1_15"]
            pub const ENABLED: u32 = 0x01;
        }
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_SD_B1_00 SW MUX Control Register"]
pub mod SW_MUX_CTL_PAD_GPIO_SD_B1_00 {
    #[doc = "MUX Mode Select Field."]
    pub mod MUX_MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Select mux mode: ALT1 mux port: FLEXSPI_B_DATA03 of instance: flexspi"]
            pub const ALT1: u32 = 0x01;
            #[doc = "Select mux mode: ALT3 mux port: XBAR1_XBAR_INOUT10 of instance: xbar1"]
            pub const ALT3: u32 = 0x03;
            #[doc = "Select mux mode: ALT5 mux port: GPIO3_IO20 of instance: gpio3"]
            pub const ALT5: u32 = 0x05;
        }
    }
    #[doc = "Software Input On Field."]
    pub mod SION {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Input Path is determined by functionality"]
            pub const DISABLED: u32 = 0;
            #[doc = "Force input path of pad GPIO_SD_B1_00"]
            pub const ENABLED: u32 = 0x01;
        }
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_SD_B1_01 SW MUX Control Register"]
pub mod SW_MUX_CTL_PAD_GPIO_SD_B1_01 {
    #[doc = "MUX Mode Select Field."]
    pub mod MUX_MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Select mux mode: ALT1 mux port: FLEXSPI_B_SCLK of instance: flexspi"]
            pub const ALT1: u32 = 0x01;
            #[doc = "Select mux mode: ALT3 mux port: FLEXSPI_A_SS1_B of instance: flexspi"]
            pub const ALT3: u32 = 0x03;
            #[doc = "Select mux mode: ALT5 mux port: GPIO3_IO21 of instance: gpio3"]
            pub const ALT5: u32 = 0x05;
        }
    }
    #[doc = "Software Input On Field."]
    pub mod SION {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Input Path is determined by functionality"]
            pub const DISABLED: u32 = 0;
            #[doc = "Force input path of pad GPIO_SD_B1_01"]
            pub const ENABLED: u32 = 0x01;
        }
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_SD_B1_02 SW MUX Control Register"]
pub mod SW_MUX_CTL_PAD_GPIO_SD_B1_02 {
    #[doc = "MUX Mode Select Field."]
    pub mod MUX_MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Select mux mode: ALT1 mux port: FLEXSPI_B_DATA00 of instance: flexspi"]
            pub const ALT1: u32 = 0x01;
            #[doc = "Select mux mode: ALT5 mux port: GPIO3_IO22 of instance: gpio3"]
            pub const ALT5: u32 = 0x05;
            #[doc = "Select mux mode: ALT6 mux port: CCM_CLKO1 of instance: ccm"]
            pub const ALT6: u32 = 0x06;
        }
    }
    #[doc = "Software Input On Field."]
    pub mod SION {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Input Path is determined by functionality"]
            pub const DISABLED: u32 = 0;
            #[doc = "Force input path of pad GPIO_SD_B1_02"]
            pub const ENABLED: u32 = 0x01;
        }
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_SD_B1_03 SW MUX Control Register"]
pub mod SW_MUX_CTL_PAD_GPIO_SD_B1_03 {
    #[doc = "MUX Mode Select Field."]
    pub mod MUX_MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Select mux mode: ALT1 mux port: FLEXSPI_B_DATA02 of instance: flexspi"]
            pub const ALT1: u32 = 0x01;
            #[doc = "Select mux mode: ALT5 mux port: GPIO3_IO23 of instance: gpio3"]
            pub const ALT5: u32 = 0x05;
            #[doc = "Select mux mode: ALT6 mux port: CCM_CLKO2 of instance: ccm"]
            pub const ALT6: u32 = 0x06;
        }
    }
    #[doc = "Software Input On Field."]
    pub mod SION {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Input Path is determined by functionality"]
            pub const DISABLED: u32 = 0;
            #[doc = "Force input path of pad GPIO_SD_B1_03"]
            pub const ENABLED: u32 = 0x01;
        }
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_SD_B1_04 SW MUX Control Register"]
pub mod SW_MUX_CTL_PAD_GPIO_SD_B1_04 {
    #[doc = "MUX Mode Select Field."]
    pub mod MUX_MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Select mux mode: ALT1 mux port: FLEXSPI_B_DATA01 of instance: flexspi"]
            pub const ALT1: u32 = 0x01;
            #[doc = "Select mux mode: ALT4 mux port: EWM_EWM_OUT_B of instance: ewm"]
            pub const ALT4: u32 = 0x04;
            #[doc = "Select mux mode: ALT5 mux port: GPIO3_IO24 of instance: gpio3"]
            pub const ALT5: u32 = 0x05;
            #[doc = "Select mux mode: ALT6 mux port: CCM_WAIT of instance: ccm"]
            pub const ALT6: u32 = 0x06;
        }
    }
    #[doc = "Software Input On Field."]
    pub mod SION {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Input Path is determined by functionality"]
            pub const DISABLED: u32 = 0;
            #[doc = "Force input path of pad GPIO_SD_B1_04"]
            pub const ENABLED: u32 = 0x01;
        }
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_SD_B1_05 SW MUX Control Register"]
pub mod SW_MUX_CTL_PAD_GPIO_SD_B1_05 {
    #[doc = "MUX Mode Select Field."]
    pub mod MUX_MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Select mux mode: ALT1 mux port: FLEXSPI_A_DQS of instance: flexspi"]
            pub const ALT1: u32 = 0x01;
            #[doc = "Select mux mode: ALT3 mux port: SAI3_MCLK of instance: sai3"]
            pub const ALT3: u32 = 0x03;
            #[doc = "Select mux mode: ALT4 mux port: FLEXSPI_B_SS0_B of instance: flexspi"]
            pub const ALT4: u32 = 0x04;
            #[doc = "Select mux mode: ALT5 mux port: GPIO3_IO25 of instance: gpio3"]
            pub const ALT5: u32 = 0x05;
            #[doc = "Select mux mode: ALT6 mux port: CCM_PMIC_RDY of instance: ccm"]
            pub const ALT6: u32 = 0x06;
        }
    }
    #[doc = "Software Input On Field."]
    pub mod SION {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Input Path is determined by functionality"]
            pub const DISABLED: u32 = 0;
            #[doc = "Force input path of pad GPIO_SD_B1_05"]
            pub const ENABLED: u32 = 0x01;
        }
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_SD_B1_06 SW MUX Control Register"]
pub mod SW_MUX_CTL_PAD_GPIO_SD_B1_06 {
    #[doc = "MUX Mode Select Field."]
    pub mod MUX_MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Select mux mode: ALT1 mux port: FLEXSPI_A_DATA03 of instance: flexspi"]
            pub const ALT1: u32 = 0x01;
            #[doc = "Select mux mode: ALT3 mux port: SAI3_TX_BCLK of instance: sai3"]
            pub const ALT3: u32 = 0x03;
            #[doc = "Select mux mode: ALT4 mux port: LPSPI2_PCS0 of instance: lpspi2"]
            pub const ALT4: u32 = 0x04;
            #[doc = "Select mux mode: ALT5 mux port: GPIO3_IO26 of instance: gpio3"]
            pub const ALT5: u32 = 0x05;
            #[doc = "Select mux mode: ALT6 mux port: CCM_STOP of instance: ccm"]
            pub const ALT6: u32 = 0x06;
        }
    }
    #[doc = "Software Input On Field."]
    pub mod SION {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Input Path is determined by functionality"]
            pub const DISABLED: u32 = 0;
            #[doc = "Force input path of pad GPIO_SD_B1_06"]
            pub const ENABLED: u32 = 0x01;
        }
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_SD_B1_07 SW MUX Control Register"]
pub mod SW_MUX_CTL_PAD_GPIO_SD_B1_07 {
    #[doc = "MUX Mode Select Field."]
    pub mod MUX_MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Select mux mode: ALT1 mux port: FLEXSPI_A_SCLK of instance: flexspi"]
            pub const ALT1: u32 = 0x01;
            #[doc = "Select mux mode: ALT3 mux port: SAI3_TX_SYNC of instance: sai3"]
            pub const ALT3: u32 = 0x03;
            #[doc = "Select mux mode: ALT4 mux port: LPSPI2_SCK of instance: lpspi2"]
            pub const ALT4: u32 = 0x04;
            #[doc = "Select mux mode: ALT5 mux port: GPIO3_IO27 of instance: gpio3"]
            pub const ALT5: u32 = 0x05;
        }
    }
    #[doc = "Software Input On Field."]
    pub mod SION {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Input Path is determined by functionality"]
            pub const DISABLED: u32 = 0;
            #[doc = "Force input path of pad GPIO_SD_B1_07"]
            pub const ENABLED: u32 = 0x01;
        }
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_SD_B1_08 SW MUX Control Register"]
pub mod SW_MUX_CTL_PAD_GPIO_SD_B1_08 {
    #[doc = "MUX Mode Select Field."]
    pub mod MUX_MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Select mux mode: ALT1 mux port: FLEXSPI_A_DATA00 of instance: flexspi"]
            pub const ALT1: u32 = 0x01;
            #[doc = "Select mux mode: ALT3 mux port: SAI3_TX_DATA of instance: sai3"]
            pub const ALT3: u32 = 0x03;
            #[doc = "Select mux mode: ALT4 mux port: LPSPI2_SDO of instance: lpspi2"]
            pub const ALT4: u32 = 0x04;
            #[doc = "Select mux mode: ALT5 mux port: GPIO3_IO28 of instance: gpio3"]
            pub const ALT5: u32 = 0x05;
        }
    }
    #[doc = "Software Input On Field."]
    pub mod SION {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Input Path is determined by functionality"]
            pub const DISABLED: u32 = 0;
            #[doc = "Force input path of pad GPIO_SD_B1_08"]
            pub const ENABLED: u32 = 0x01;
        }
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_SD_B1_09 SW MUX Control Register"]
pub mod SW_MUX_CTL_PAD_GPIO_SD_B1_09 {
    #[doc = "MUX Mode Select Field."]
    pub mod MUX_MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Select mux mode: ALT1 mux port: FLEXSPI_A_DATA02 of instance: flexspi"]
            pub const ALT1: u32 = 0x01;
            #[doc = "Select mux mode: ALT3 mux port: SAI3_RX_BCLK of instance: sai3"]
            pub const ALT3: u32 = 0x03;
            #[doc = "Select mux mode: ALT4 mux port: LPSPI2_SDI of instance: lpspi2"]
            pub const ALT4: u32 = 0x04;
            #[doc = "Select mux mode: ALT5 mux port: GPIO3_IO29 of instance: gpio3"]
            pub const ALT5: u32 = 0x05;
            #[doc = "Select mux mode: ALT6 mux port: CCM_REF_EN_B of instance: ccm"]
            pub const ALT6: u32 = 0x06;
        }
    }
    #[doc = "Software Input On Field."]
    pub mod SION {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Input Path is determined by functionality"]
            pub const DISABLED: u32 = 0;
            #[doc = "Force input path of pad GPIO_SD_B1_09"]
            pub const ENABLED: u32 = 0x01;
        }
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_SD_B1_10 SW MUX Control Register"]
pub mod SW_MUX_CTL_PAD_GPIO_SD_B1_10 {
    #[doc = "MUX Mode Select Field."]
    pub mod MUX_MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Select mux mode: ALT1 mux port: FLEXSPI_A_DATA01 of instance: flexspi"]
            pub const ALT1: u32 = 0x01;
            #[doc = "Select mux mode: ALT3 mux port: SAI3_RX_SYNC of instance: sai3"]
            pub const ALT3: u32 = 0x03;
            #[doc = "Select mux mode: ALT4 mux port: LPSPI2_PCS2 of instance: lpspi2"]
            pub const ALT4: u32 = 0x04;
            #[doc = "Select mux mode: ALT5 mux port: GPIO3_IO30 of instance: gpio3"]
            pub const ALT5: u32 = 0x05;
        }
    }
    #[doc = "Software Input On Field."]
    pub mod SION {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Input Path is determined by functionality"]
            pub const DISABLED: u32 = 0;
            #[doc = "Force input path of pad GPIO_SD_B1_10"]
            pub const ENABLED: u32 = 0x01;
        }
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_SD_B1_11 SW MUX Control Register"]
pub mod SW_MUX_CTL_PAD_GPIO_SD_B1_11 {
    #[doc = "MUX Mode Select Field."]
    pub mod MUX_MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Select mux mode: ALT1 mux port: FLEXSPI_A_SS0_B of instance: flexspi"]
            pub const ALT1: u32 = 0x01;
            #[doc = "Select mux mode: ALT3 mux port: SAI3_RX_DATA of instance: sai3"]
            pub const ALT3: u32 = 0x03;
            #[doc = "Select mux mode: ALT4 mux port: LPSPI2_PCS3 of instance: lpspi2"]
            pub const ALT4: u32 = 0x04;
            #[doc = "Select mux mode: ALT5 mux port: GPIO3_IO31 of instance: gpio3"]
            pub const ALT5: u32 = 0x05;
        }
    }
    #[doc = "Software Input On Field."]
    pub mod SION {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Input Path is determined by functionality"]
            pub const DISABLED: u32 = 0;
            #[doc = "Force input path of pad GPIO_SD_B1_11"]
            pub const ENABLED: u32 = 0x01;
        }
    }
}
#[doc = "SW_PAD_CTL_PAD_GPIO_EMC_04 SW PAD Control Register"]
pub mod SW_PAD_CTL_PAD_GPIO_EMC_04 {
    #[doc = "Slew Rate Field"]
    pub mod SRE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Slow Slew Rate"]
            pub const SRE_0_SLOW_SLEW_RATE: u32 = 0;
            #[doc = "Fast Slew Rate"]
            pub const SRE_1_FAST_SLEW_RATE: u32 = 0x01;
        }
    }
    #[doc = "Drive Strength Field"]
    pub mod DSE {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "output driver disabled;"]
            pub const DSE_0_OUTPUT_DRIVER_DISABLED_: u32 = 0;
            #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V)"]
            pub const DSE_1_R0_150_OHM___3_3V__260_OHM_1_8V: u32 = 0x01;
            #[doc = "R0/2"]
            pub const DSE_2_R0_2: u32 = 0x02;
            #[doc = "R0/3"]
            pub const DSE_3_R0_3: u32 = 0x03;
            #[doc = "R0/4"]
            pub const DSE_4_R0_4: u32 = 0x04;
            #[doc = "R0/5"]
            pub const DSE_5_R0_5: u32 = 0x05;
            #[doc = "R0/6"]
            pub const DSE_6_R0_6: u32 = 0x06;
            #[doc = "R0/7"]
            pub const DSE_7_R0_7: u32 = 0x07;
        }
    }
    #[doc = "Speed Field"]
    pub mod SPEED {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "low(50MHz)"]
            pub const SPEED_0_LOW_50MHZ_: u32 = 0;
            #[doc = "medium(100MHz)"]
            pub const SPEED_1_MEDIUM_100MHZ_: u32 = 0x01;
            #[doc = "medium(100MHz)"]
            pub const SPEED_2_MEDIUM_100MHZ_: u32 = 0x02;
            #[doc = "max(200MHz)"]
            pub const SPEED_3_MAX_200MHZ_: u32 = 0x03;
        }
    }
    #[doc = "Open Drain Enable Field"]
    pub mod ODE {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Open Drain Disabled"]
            pub const ODE_0_OPEN_DRAIN_DISABLED: u32 = 0;
            #[doc = "Open Drain Enabled"]
            pub const ODE_1_OPEN_DRAIN_ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Pull / Keep Enable Field"]
    pub mod PKE {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pull/Keeper Disabled"]
            pub const PKE_0_PULL_KEEPER_DISABLED: u32 = 0;
            #[doc = "Pull/Keeper Enabled"]
            pub const PKE_1_PULL_KEEPER_ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Pull / Keep Select Field"]
    pub mod PUE {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Keeper"]
            pub const PUE_0_KEEPER: u32 = 0;
            #[doc = "Pull"]
            pub const PUE_1_PULL: u32 = 0x01;
        }
    }
    #[doc = "Pull Up / Down Config. Field"]
    pub mod PUS {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "100K Ohm Pull Down"]
            pub const PUS_0_100K_OHM_PULL_DOWN: u32 = 0;
            #[doc = "47K Ohm Pull Up"]
            pub const PUS_1_47K_OHM_PULL_UP: u32 = 0x01;
            #[doc = "100K Ohm Pull Up"]
            pub const PUS_2_100K_OHM_PULL_UP: u32 = 0x02;
            #[doc = "22K Ohm Pull Up"]
            pub const PUS_3_22K_OHM_PULL_UP: u32 = 0x03;
        }
    }
    #[doc = "Hyst. Enable Field"]
    pub mod HYS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Hysteresis Disabled"]
            pub const HYS_0_HYSTERESIS_DISABLED: u32 = 0;
            #[doc = "Hysteresis Enabled"]
            pub const HYS_1_HYSTERESIS_ENABLED: u32 = 0x01;
        }
    }
}
#[doc = "SW_PAD_CTL_PAD_GPIO_EMC_05 SW PAD Control Register"]
pub mod SW_PAD_CTL_PAD_GPIO_EMC_05 {
    #[doc = "Slew Rate Field"]
    pub mod SRE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Slow Slew Rate"]
            pub const SRE_0_SLOW_SLEW_RATE: u32 = 0;
            #[doc = "Fast Slew Rate"]
            pub const SRE_1_FAST_SLEW_RATE: u32 = 0x01;
        }
    }
    #[doc = "Drive Strength Field"]
    pub mod DSE {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "output driver disabled;"]
            pub const DSE_0_OUTPUT_DRIVER_DISABLED_: u32 = 0;
            #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V)"]
            pub const DSE_1_R0_150_OHM___3_3V__260_OHM_1_8V: u32 = 0x01;
            #[doc = "R0/2"]
            pub const DSE_2_R0_2: u32 = 0x02;
            #[doc = "R0/3"]
            pub const DSE_3_R0_3: u32 = 0x03;
            #[doc = "R0/4"]
            pub const DSE_4_R0_4: u32 = 0x04;
            #[doc = "R0/5"]
            pub const DSE_5_R0_5: u32 = 0x05;
            #[doc = "R0/6"]
            pub const DSE_6_R0_6: u32 = 0x06;
            #[doc = "R0/7"]
            pub const DSE_7_R0_7: u32 = 0x07;
        }
    }
    #[doc = "Speed Field"]
    pub mod SPEED {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "low(50MHz)"]
            pub const SPEED_0_LOW_50MHZ_: u32 = 0;
            #[doc = "medium(100MHz)"]
            pub const SPEED_1_MEDIUM_100MHZ_: u32 = 0x01;
            #[doc = "medium(100MHz)"]
            pub const SPEED_2_MEDIUM_100MHZ_: u32 = 0x02;
            #[doc = "max(200MHz)"]
            pub const SPEED_3_MAX_200MHZ_: u32 = 0x03;
        }
    }
    #[doc = "Open Drain Enable Field"]
    pub mod ODE {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Open Drain Disabled"]
            pub const ODE_0_OPEN_DRAIN_DISABLED: u32 = 0;
            #[doc = "Open Drain Enabled"]
            pub const ODE_1_OPEN_DRAIN_ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Pull / Keep Enable Field"]
    pub mod PKE {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pull/Keeper Disabled"]
            pub const PKE_0_PULL_KEEPER_DISABLED: u32 = 0;
            #[doc = "Pull/Keeper Enabled"]
            pub const PKE_1_PULL_KEEPER_ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Pull / Keep Select Field"]
    pub mod PUE {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Keeper"]
            pub const PUE_0_KEEPER: u32 = 0;
            #[doc = "Pull"]
            pub const PUE_1_PULL: u32 = 0x01;
        }
    }
    #[doc = "Pull Up / Down Config. Field"]
    pub mod PUS {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "100K Ohm Pull Down"]
            pub const PUS_0_100K_OHM_PULL_DOWN: u32 = 0;
            #[doc = "47K Ohm Pull Up"]
            pub const PUS_1_47K_OHM_PULL_UP: u32 = 0x01;
            #[doc = "100K Ohm Pull Up"]
            pub const PUS_2_100K_OHM_PULL_UP: u32 = 0x02;
            #[doc = "22K Ohm Pull Up"]
            pub const PUS_3_22K_OHM_PULL_UP: u32 = 0x03;
        }
    }
    #[doc = "Hyst. Enable Field"]
    pub mod HYS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Hysteresis Disabled"]
            pub const HYS_0_HYSTERESIS_DISABLED: u32 = 0;
            #[doc = "Hysteresis Enabled"]
            pub const HYS_1_HYSTERESIS_ENABLED: u32 = 0x01;
        }
    }
}
#[doc = "SW_PAD_CTL_PAD_GPIO_EMC_06 SW PAD Control Register"]
pub mod SW_PAD_CTL_PAD_GPIO_EMC_06 {
    #[doc = "Slew Rate Field"]
    pub mod SRE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Slow Slew Rate"]
            pub const SRE_0_SLOW_SLEW_RATE: u32 = 0;
            #[doc = "Fast Slew Rate"]
            pub const SRE_1_FAST_SLEW_RATE: u32 = 0x01;
        }
    }
    #[doc = "Drive Strength Field"]
    pub mod DSE {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "output driver disabled;"]
            pub const DSE_0_OUTPUT_DRIVER_DISABLED_: u32 = 0;
            #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V)"]
            pub const DSE_1_R0_150_OHM___3_3V__260_OHM_1_8V: u32 = 0x01;
            #[doc = "R0/2"]
            pub const DSE_2_R0_2: u32 = 0x02;
            #[doc = "R0/3"]
            pub const DSE_3_R0_3: u32 = 0x03;
            #[doc = "R0/4"]
            pub const DSE_4_R0_4: u32 = 0x04;
            #[doc = "R0/5"]
            pub const DSE_5_R0_5: u32 = 0x05;
            #[doc = "R0/6"]
            pub const DSE_6_R0_6: u32 = 0x06;
            #[doc = "R0/7"]
            pub const DSE_7_R0_7: u32 = 0x07;
        }
    }
    #[doc = "Speed Field"]
    pub mod SPEED {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "low(50MHz)"]
            pub const SPEED_0_LOW_50MHZ_: u32 = 0;
            #[doc = "medium(100MHz)"]
            pub const SPEED_1_MEDIUM_100MHZ_: u32 = 0x01;
            #[doc = "medium(100MHz)"]
            pub const SPEED_2_MEDIUM_100MHZ_: u32 = 0x02;
            #[doc = "max(200MHz)"]
            pub const SPEED_3_MAX_200MHZ_: u32 = 0x03;
        }
    }
    #[doc = "Open Drain Enable Field"]
    pub mod ODE {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Open Drain Disabled"]
            pub const ODE_0_OPEN_DRAIN_DISABLED: u32 = 0;
            #[doc = "Open Drain Enabled"]
            pub const ODE_1_OPEN_DRAIN_ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Pull / Keep Enable Field"]
    pub mod PKE {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pull/Keeper Disabled"]
            pub const PKE_0_PULL_KEEPER_DISABLED: u32 = 0;
            #[doc = "Pull/Keeper Enabled"]
            pub const PKE_1_PULL_KEEPER_ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Pull / Keep Select Field"]
    pub mod PUE {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Keeper"]
            pub const PUE_0_KEEPER: u32 = 0;
            #[doc = "Pull"]
            pub const PUE_1_PULL: u32 = 0x01;
        }
    }
    #[doc = "Pull Up / Down Config. Field"]
    pub mod PUS {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "100K Ohm Pull Down"]
            pub const PUS_0_100K_OHM_PULL_DOWN: u32 = 0;
            #[doc = "47K Ohm Pull Up"]
            pub const PUS_1_47K_OHM_PULL_UP: u32 = 0x01;
            #[doc = "100K Ohm Pull Up"]
            pub const PUS_2_100K_OHM_PULL_UP: u32 = 0x02;
            #[doc = "22K Ohm Pull Up"]
            pub const PUS_3_22K_OHM_PULL_UP: u32 = 0x03;
        }
    }
    #[doc = "Hyst. Enable Field"]
    pub mod HYS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Hysteresis Disabled"]
            pub const HYS_0_HYSTERESIS_DISABLED: u32 = 0;
            #[doc = "Hysteresis Enabled"]
            pub const HYS_1_HYSTERESIS_ENABLED: u32 = 0x01;
        }
    }
}
#[doc = "SW_PAD_CTL_PAD_GPIO_EMC_07 SW PAD Control Register"]
pub mod SW_PAD_CTL_PAD_GPIO_EMC_07 {
    #[doc = "Slew Rate Field"]
    pub mod SRE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Slow Slew Rate"]
            pub const SRE_0_SLOW_SLEW_RATE: u32 = 0;
            #[doc = "Fast Slew Rate"]
            pub const SRE_1_FAST_SLEW_RATE: u32 = 0x01;
        }
    }
    #[doc = "Drive Strength Field"]
    pub mod DSE {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "output driver disabled;"]
            pub const DSE_0_OUTPUT_DRIVER_DISABLED_: u32 = 0;
            #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V)"]
            pub const DSE_1_R0_150_OHM___3_3V__260_OHM_1_8V: u32 = 0x01;
            #[doc = "R0/2"]
            pub const DSE_2_R0_2: u32 = 0x02;
            #[doc = "R0/3"]
            pub const DSE_3_R0_3: u32 = 0x03;
            #[doc = "R0/4"]
            pub const DSE_4_R0_4: u32 = 0x04;
            #[doc = "R0/5"]
            pub const DSE_5_R0_5: u32 = 0x05;
            #[doc = "R0/6"]
            pub const DSE_6_R0_6: u32 = 0x06;
            #[doc = "R0/7"]
            pub const DSE_7_R0_7: u32 = 0x07;
        }
    }
    #[doc = "Speed Field"]
    pub mod SPEED {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "low(50MHz)"]
            pub const SPEED_0_LOW_50MHZ_: u32 = 0;
            #[doc = "medium(100MHz)"]
            pub const SPEED_1_MEDIUM_100MHZ_: u32 = 0x01;
            #[doc = "medium(100MHz)"]
            pub const SPEED_2_MEDIUM_100MHZ_: u32 = 0x02;
            #[doc = "max(200MHz)"]
            pub const SPEED_3_MAX_200MHZ_: u32 = 0x03;
        }
    }
    #[doc = "Open Drain Enable Field"]
    pub mod ODE {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Open Drain Disabled"]
            pub const ODE_0_OPEN_DRAIN_DISABLED: u32 = 0;
            #[doc = "Open Drain Enabled"]
            pub const ODE_1_OPEN_DRAIN_ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Pull / Keep Enable Field"]
    pub mod PKE {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pull/Keeper Disabled"]
            pub const PKE_0_PULL_KEEPER_DISABLED: u32 = 0;
            #[doc = "Pull/Keeper Enabled"]
            pub const PKE_1_PULL_KEEPER_ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Pull / Keep Select Field"]
    pub mod PUE {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Keeper"]
            pub const PUE_0_KEEPER: u32 = 0;
            #[doc = "Pull"]
            pub const PUE_1_PULL: u32 = 0x01;
        }
    }
    #[doc = "Pull Up / Down Config. Field"]
    pub mod PUS {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "100K Ohm Pull Down"]
            pub const PUS_0_100K_OHM_PULL_DOWN: u32 = 0;
            #[doc = "47K Ohm Pull Up"]
            pub const PUS_1_47K_OHM_PULL_UP: u32 = 0x01;
            #[doc = "100K Ohm Pull Up"]
            pub const PUS_2_100K_OHM_PULL_UP: u32 = 0x02;
            #[doc = "22K Ohm Pull Up"]
            pub const PUS_3_22K_OHM_PULL_UP: u32 = 0x03;
        }
    }
    #[doc = "Hyst. Enable Field"]
    pub mod HYS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Hysteresis Disabled"]
            pub const HYS_0_HYSTERESIS_DISABLED: u32 = 0;
            #[doc = "Hysteresis Enabled"]
            pub const HYS_1_HYSTERESIS_ENABLED: u32 = 0x01;
        }
    }
}
#[doc = "SW_PAD_CTL_PAD_GPIO_EMC_08 SW PAD Control Register"]
pub mod SW_PAD_CTL_PAD_GPIO_EMC_08 {
    #[doc = "Slew Rate Field"]
    pub mod SRE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Slow Slew Rate"]
            pub const SRE_0_SLOW_SLEW_RATE: u32 = 0;
            #[doc = "Fast Slew Rate"]
            pub const SRE_1_FAST_SLEW_RATE: u32 = 0x01;
        }
    }
    #[doc = "Drive Strength Field"]
    pub mod DSE {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "output driver disabled;"]
            pub const DSE_0_OUTPUT_DRIVER_DISABLED_: u32 = 0;
            #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V)"]
            pub const DSE_1_R0_150_OHM___3_3V__260_OHM_1_8V: u32 = 0x01;
            #[doc = "R0/2"]
            pub const DSE_2_R0_2: u32 = 0x02;
            #[doc = "R0/3"]
            pub const DSE_3_R0_3: u32 = 0x03;
            #[doc = "R0/4"]
            pub const DSE_4_R0_4: u32 = 0x04;
            #[doc = "R0/5"]
            pub const DSE_5_R0_5: u32 = 0x05;
            #[doc = "R0/6"]
            pub const DSE_6_R0_6: u32 = 0x06;
            #[doc = "R0/7"]
            pub const DSE_7_R0_7: u32 = 0x07;
        }
    }
    #[doc = "Speed Field"]
    pub mod SPEED {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "low(50MHz)"]
            pub const SPEED_0_LOW_50MHZ_: u32 = 0;
            #[doc = "medium(100MHz)"]
            pub const SPEED_1_MEDIUM_100MHZ_: u32 = 0x01;
            #[doc = "medium(100MHz)"]
            pub const SPEED_2_MEDIUM_100MHZ_: u32 = 0x02;
            #[doc = "max(200MHz)"]
            pub const SPEED_3_MAX_200MHZ_: u32 = 0x03;
        }
    }
    #[doc = "Open Drain Enable Field"]
    pub mod ODE {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Open Drain Disabled"]
            pub const ODE_0_OPEN_DRAIN_DISABLED: u32 = 0;
            #[doc = "Open Drain Enabled"]
            pub const ODE_1_OPEN_DRAIN_ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Pull / Keep Enable Field"]
    pub mod PKE {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pull/Keeper Disabled"]
            pub const PKE_0_PULL_KEEPER_DISABLED: u32 = 0;
            #[doc = "Pull/Keeper Enabled"]
            pub const PKE_1_PULL_KEEPER_ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Pull / Keep Select Field"]
    pub mod PUE {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Keeper"]
            pub const PUE_0_KEEPER: u32 = 0;
            #[doc = "Pull"]
            pub const PUE_1_PULL: u32 = 0x01;
        }
    }
    #[doc = "Pull Up / Down Config. Field"]
    pub mod PUS {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "100K Ohm Pull Down"]
            pub const PUS_0_100K_OHM_PULL_DOWN: u32 = 0;
            #[doc = "47K Ohm Pull Up"]
            pub const PUS_1_47K_OHM_PULL_UP: u32 = 0x01;
            #[doc = "100K Ohm Pull Up"]
            pub const PUS_2_100K_OHM_PULL_UP: u32 = 0x02;
            #[doc = "22K Ohm Pull Up"]
            pub const PUS_3_22K_OHM_PULL_UP: u32 = 0x03;
        }
    }
    #[doc = "Hyst. Enable Field"]
    pub mod HYS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Hysteresis Disabled"]
            pub const HYS_0_HYSTERESIS_DISABLED: u32 = 0;
            #[doc = "Hysteresis Enabled"]
            pub const HYS_1_HYSTERESIS_ENABLED: u32 = 0x01;
        }
    }
}
#[doc = "SW_PAD_CTL_PAD_GPIO_EMC_09 SW PAD Control Register"]
pub mod SW_PAD_CTL_PAD_GPIO_EMC_09 {
    #[doc = "Slew Rate Field"]
    pub mod SRE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Slow Slew Rate"]
            pub const SRE_0_SLOW_SLEW_RATE: u32 = 0;
            #[doc = "Fast Slew Rate"]
            pub const SRE_1_FAST_SLEW_RATE: u32 = 0x01;
        }
    }
    #[doc = "Drive Strength Field"]
    pub mod DSE {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "output driver disabled;"]
            pub const DSE_0_OUTPUT_DRIVER_DISABLED_: u32 = 0;
            #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V)"]
            pub const DSE_1_R0_150_OHM___3_3V__260_OHM_1_8V: u32 = 0x01;
            #[doc = "R0/2"]
            pub const DSE_2_R0_2: u32 = 0x02;
            #[doc = "R0/3"]
            pub const DSE_3_R0_3: u32 = 0x03;
            #[doc = "R0/4"]
            pub const DSE_4_R0_4: u32 = 0x04;
            #[doc = "R0/5"]
            pub const DSE_5_R0_5: u32 = 0x05;
            #[doc = "R0/6"]
            pub const DSE_6_R0_6: u32 = 0x06;
            #[doc = "R0/7"]
            pub const DSE_7_R0_7: u32 = 0x07;
        }
    }
    #[doc = "Speed Field"]
    pub mod SPEED {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "low(50MHz)"]
            pub const SPEED_0_LOW_50MHZ_: u32 = 0;
            #[doc = "medium(100MHz)"]
            pub const SPEED_1_MEDIUM_100MHZ_: u32 = 0x01;
            #[doc = "medium(100MHz)"]
            pub const SPEED_2_MEDIUM_100MHZ_: u32 = 0x02;
            #[doc = "max(200MHz)"]
            pub const SPEED_3_MAX_200MHZ_: u32 = 0x03;
        }
    }
    #[doc = "Open Drain Enable Field"]
    pub mod ODE {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Open Drain Disabled"]
            pub const ODE_0_OPEN_DRAIN_DISABLED: u32 = 0;
            #[doc = "Open Drain Enabled"]
            pub const ODE_1_OPEN_DRAIN_ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Pull / Keep Enable Field"]
    pub mod PKE {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pull/Keeper Disabled"]
            pub const PKE_0_PULL_KEEPER_DISABLED: u32 = 0;
            #[doc = "Pull/Keeper Enabled"]
            pub const PKE_1_PULL_KEEPER_ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Pull / Keep Select Field"]
    pub mod PUE {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Keeper"]
            pub const PUE_0_KEEPER: u32 = 0;
            #[doc = "Pull"]
            pub const PUE_1_PULL: u32 = 0x01;
        }
    }
    #[doc = "Pull Up / Down Config. Field"]
    pub mod PUS {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "100K Ohm Pull Down"]
            pub const PUS_0_100K_OHM_PULL_DOWN: u32 = 0;
            #[doc = "47K Ohm Pull Up"]
            pub const PUS_1_47K_OHM_PULL_UP: u32 = 0x01;
            #[doc = "100K Ohm Pull Up"]
            pub const PUS_2_100K_OHM_PULL_UP: u32 = 0x02;
            #[doc = "22K Ohm Pull Up"]
            pub const PUS_3_22K_OHM_PULL_UP: u32 = 0x03;
        }
    }
    #[doc = "Hyst. Enable Field"]
    pub mod HYS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Hysteresis Disabled"]
            pub const HYS_0_HYSTERESIS_DISABLED: u32 = 0;
            #[doc = "Hysteresis Enabled"]
            pub const HYS_1_HYSTERESIS_ENABLED: u32 = 0x01;
        }
    }
}
#[doc = "SW_PAD_CTL_PAD_GPIO_EMC_16 SW PAD Control Register"]
pub mod SW_PAD_CTL_PAD_GPIO_EMC_16 {
    #[doc = "Slew Rate Field"]
    pub mod SRE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Slow Slew Rate"]
            pub const SRE_0_SLOW_SLEW_RATE: u32 = 0;
            #[doc = "Fast Slew Rate"]
            pub const SRE_1_FAST_SLEW_RATE: u32 = 0x01;
        }
    }
    #[doc = "Drive Strength Field"]
    pub mod DSE {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "output driver disabled;"]
            pub const DSE_0_OUTPUT_DRIVER_DISABLED_: u32 = 0;
            #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V)"]
            pub const DSE_1_R0_150_OHM___3_3V__260_OHM_1_8V: u32 = 0x01;
            #[doc = "R0/2"]
            pub const DSE_2_R0_2: u32 = 0x02;
            #[doc = "R0/3"]
            pub const DSE_3_R0_3: u32 = 0x03;
            #[doc = "R0/4"]
            pub const DSE_4_R0_4: u32 = 0x04;
            #[doc = "R0/5"]
            pub const DSE_5_R0_5: u32 = 0x05;
            #[doc = "R0/6"]
            pub const DSE_6_R0_6: u32 = 0x06;
            #[doc = "R0/7"]
            pub const DSE_7_R0_7: u32 = 0x07;
        }
    }
    #[doc = "Speed Field"]
    pub mod SPEED {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "low(50MHz)"]
            pub const SPEED_0_LOW_50MHZ_: u32 = 0;
            #[doc = "medium(100MHz)"]
            pub const SPEED_1_MEDIUM_100MHZ_: u32 = 0x01;
            #[doc = "medium(100MHz)"]
            pub const SPEED_2_MEDIUM_100MHZ_: u32 = 0x02;
            #[doc = "max(200MHz)"]
            pub const SPEED_3_MAX_200MHZ_: u32 = 0x03;
        }
    }
    #[doc = "Open Drain Enable Field"]
    pub mod ODE {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Open Drain Disabled"]
            pub const ODE_0_OPEN_DRAIN_DISABLED: u32 = 0;
            #[doc = "Open Drain Enabled"]
            pub const ODE_1_OPEN_DRAIN_ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Pull / Keep Enable Field"]
    pub mod PKE {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pull/Keeper Disabled"]
            pub const PKE_0_PULL_KEEPER_DISABLED: u32 = 0;
            #[doc = "Pull/Keeper Enabled"]
            pub const PKE_1_PULL_KEEPER_ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Pull / Keep Select Field"]
    pub mod PUE {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Keeper"]
            pub const PUE_0_KEEPER: u32 = 0;
            #[doc = "Pull"]
            pub const PUE_1_PULL: u32 = 0x01;
        }
    }
    #[doc = "Pull Up / Down Config. Field"]
    pub mod PUS {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "100K Ohm Pull Down"]
            pub const PUS_0_100K_OHM_PULL_DOWN: u32 = 0;
            #[doc = "47K Ohm Pull Up"]
            pub const PUS_1_47K_OHM_PULL_UP: u32 = 0x01;
            #[doc = "100K Ohm Pull Up"]
            pub const PUS_2_100K_OHM_PULL_UP: u32 = 0x02;
            #[doc = "22K Ohm Pull Up"]
            pub const PUS_3_22K_OHM_PULL_UP: u32 = 0x03;
        }
    }
    #[doc = "Hyst. Enable Field"]
    pub mod HYS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Hysteresis Disabled"]
            pub const HYS_0_HYSTERESIS_DISABLED: u32 = 0;
            #[doc = "Hysteresis Enabled"]
            pub const HYS_1_HYSTERESIS_ENABLED: u32 = 0x01;
        }
    }
}
#[doc = "SW_PAD_CTL_PAD_GPIO_EMC_17 SW PAD Control Register"]
pub mod SW_PAD_CTL_PAD_GPIO_EMC_17 {
    #[doc = "Slew Rate Field"]
    pub mod SRE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Slow Slew Rate"]
            pub const SRE_0_SLOW_SLEW_RATE: u32 = 0;
            #[doc = "Fast Slew Rate"]
            pub const SRE_1_FAST_SLEW_RATE: u32 = 0x01;
        }
    }
    #[doc = "Drive Strength Field"]
    pub mod DSE {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "output driver disabled;"]
            pub const DSE_0_OUTPUT_DRIVER_DISABLED_: u32 = 0;
            #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V)"]
            pub const DSE_1_R0_150_OHM___3_3V__260_OHM_1_8V: u32 = 0x01;
            #[doc = "R0/2"]
            pub const DSE_2_R0_2: u32 = 0x02;
            #[doc = "R0/3"]
            pub const DSE_3_R0_3: u32 = 0x03;
            #[doc = "R0/4"]
            pub const DSE_4_R0_4: u32 = 0x04;
            #[doc = "R0/5"]
            pub const DSE_5_R0_5: u32 = 0x05;
            #[doc = "R0/6"]
            pub const DSE_6_R0_6: u32 = 0x06;
            #[doc = "R0/7"]
            pub const DSE_7_R0_7: u32 = 0x07;
        }
    }
    #[doc = "Speed Field"]
    pub mod SPEED {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "low(50MHz)"]
            pub const SPEED_0_LOW_50MHZ_: u32 = 0;
            #[doc = "medium(100MHz)"]
            pub const SPEED_1_MEDIUM_100MHZ_: u32 = 0x01;
            #[doc = "medium(100MHz)"]
            pub const SPEED_2_MEDIUM_100MHZ_: u32 = 0x02;
            #[doc = "max(200MHz)"]
            pub const SPEED_3_MAX_200MHZ_: u32 = 0x03;
        }
    }
    #[doc = "Open Drain Enable Field"]
    pub mod ODE {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Open Drain Disabled"]
            pub const ODE_0_OPEN_DRAIN_DISABLED: u32 = 0;
            #[doc = "Open Drain Enabled"]
            pub const ODE_1_OPEN_DRAIN_ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Pull / Keep Enable Field"]
    pub mod PKE {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pull/Keeper Disabled"]
            pub const PKE_0_PULL_KEEPER_DISABLED: u32 = 0;
            #[doc = "Pull/Keeper Enabled"]
            pub const PKE_1_PULL_KEEPER_ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Pull / Keep Select Field"]
    pub mod PUE {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Keeper"]
            pub const PUE_0_KEEPER: u32 = 0;
            #[doc = "Pull"]
            pub const PUE_1_PULL: u32 = 0x01;
        }
    }
    #[doc = "Pull Up / Down Config. Field"]
    pub mod PUS {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "100K Ohm Pull Down"]
            pub const PUS_0_100K_OHM_PULL_DOWN: u32 = 0;
            #[doc = "47K Ohm Pull Up"]
            pub const PUS_1_47K_OHM_PULL_UP: u32 = 0x01;
            #[doc = "100K Ohm Pull Up"]
            pub const PUS_2_100K_OHM_PULL_UP: u32 = 0x02;
            #[doc = "22K Ohm Pull Up"]
            pub const PUS_3_22K_OHM_PULL_UP: u32 = 0x03;
        }
    }
    #[doc = "Hyst. Enable Field"]
    pub mod HYS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Hysteresis Disabled"]
            pub const HYS_0_HYSTERESIS_DISABLED: u32 = 0;
            #[doc = "Hysteresis Enabled"]
            pub const HYS_1_HYSTERESIS_ENABLED: u32 = 0x01;
        }
    }
}
#[doc = "SW_PAD_CTL_PAD_GPIO_EMC_18 SW PAD Control Register"]
pub mod SW_PAD_CTL_PAD_GPIO_EMC_18 {
    #[doc = "Slew Rate Field"]
    pub mod SRE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Slow Slew Rate"]
            pub const SRE_0_SLOW_SLEW_RATE: u32 = 0;
            #[doc = "Fast Slew Rate"]
            pub const SRE_1_FAST_SLEW_RATE: u32 = 0x01;
        }
    }
    #[doc = "Drive Strength Field"]
    pub mod DSE {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "output driver disabled;"]
            pub const DSE_0_OUTPUT_DRIVER_DISABLED_: u32 = 0;
            #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V)"]
            pub const DSE_1_R0_150_OHM___3_3V__260_OHM_1_8V: u32 = 0x01;
            #[doc = "R0/2"]
            pub const DSE_2_R0_2: u32 = 0x02;
            #[doc = "R0/3"]
            pub const DSE_3_R0_3: u32 = 0x03;
            #[doc = "R0/4"]
            pub const DSE_4_R0_4: u32 = 0x04;
            #[doc = "R0/5"]
            pub const DSE_5_R0_5: u32 = 0x05;
            #[doc = "R0/6"]
            pub const DSE_6_R0_6: u32 = 0x06;
            #[doc = "R0/7"]
            pub const DSE_7_R0_7: u32 = 0x07;
        }
    }
    #[doc = "Speed Field"]
    pub mod SPEED {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "low(50MHz)"]
            pub const SPEED_0_LOW_50MHZ_: u32 = 0;
            #[doc = "medium(100MHz)"]
            pub const SPEED_1_MEDIUM_100MHZ_: u32 = 0x01;
            #[doc = "medium(100MHz)"]
            pub const SPEED_2_MEDIUM_100MHZ_: u32 = 0x02;
            #[doc = "max(200MHz)"]
            pub const SPEED_3_MAX_200MHZ_: u32 = 0x03;
        }
    }
    #[doc = "Open Drain Enable Field"]
    pub mod ODE {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Open Drain Disabled"]
            pub const ODE_0_OPEN_DRAIN_DISABLED: u32 = 0;
            #[doc = "Open Drain Enabled"]
            pub const ODE_1_OPEN_DRAIN_ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Pull / Keep Enable Field"]
    pub mod PKE {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pull/Keeper Disabled"]
            pub const PKE_0_PULL_KEEPER_DISABLED: u32 = 0;
            #[doc = "Pull/Keeper Enabled"]
            pub const PKE_1_PULL_KEEPER_ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Pull / Keep Select Field"]
    pub mod PUE {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Keeper"]
            pub const PUE_0_KEEPER: u32 = 0;
            #[doc = "Pull"]
            pub const PUE_1_PULL: u32 = 0x01;
        }
    }
    #[doc = "Pull Up / Down Config. Field"]
    pub mod PUS {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "100K Ohm Pull Down"]
            pub const PUS_0_100K_OHM_PULL_DOWN: u32 = 0;
            #[doc = "47K Ohm Pull Up"]
            pub const PUS_1_47K_OHM_PULL_UP: u32 = 0x01;
            #[doc = "100K Ohm Pull Up"]
            pub const PUS_2_100K_OHM_PULL_UP: u32 = 0x02;
            #[doc = "22K Ohm Pull Up"]
            pub const PUS_3_22K_OHM_PULL_UP: u32 = 0x03;
        }
    }
    #[doc = "Hyst. Enable Field"]
    pub mod HYS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Hysteresis Disabled"]
            pub const HYS_0_HYSTERESIS_DISABLED: u32 = 0;
            #[doc = "Hysteresis Enabled"]
            pub const HYS_1_HYSTERESIS_ENABLED: u32 = 0x01;
        }
    }
}
#[doc = "SW_PAD_CTL_PAD_GPIO_EMC_19 SW PAD Control Register"]
pub mod SW_PAD_CTL_PAD_GPIO_EMC_19 {
    #[doc = "Slew Rate Field"]
    pub mod SRE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Slow Slew Rate"]
            pub const SRE_0_SLOW_SLEW_RATE: u32 = 0;
            #[doc = "Fast Slew Rate"]
            pub const SRE_1_FAST_SLEW_RATE: u32 = 0x01;
        }
    }
    #[doc = "Drive Strength Field"]
    pub mod DSE {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "output driver disabled;"]
            pub const DSE_0_OUTPUT_DRIVER_DISABLED_: u32 = 0;
            #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V)"]
            pub const DSE_1_R0_150_OHM___3_3V__260_OHM_1_8V: u32 = 0x01;
            #[doc = "R0/2"]
            pub const DSE_2_R0_2: u32 = 0x02;
            #[doc = "R0/3"]
            pub const DSE_3_R0_3: u32 = 0x03;
            #[doc = "R0/4"]
            pub const DSE_4_R0_4: u32 = 0x04;
            #[doc = "R0/5"]
            pub const DSE_5_R0_5: u32 = 0x05;
            #[doc = "R0/6"]
            pub const DSE_6_R0_6: u32 = 0x06;
            #[doc = "R0/7"]
            pub const DSE_7_R0_7: u32 = 0x07;
        }
    }
    #[doc = "Speed Field"]
    pub mod SPEED {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "low(50MHz)"]
            pub const SPEED_0_LOW_50MHZ_: u32 = 0;
            #[doc = "medium(100MHz)"]
            pub const SPEED_1_MEDIUM_100MHZ_: u32 = 0x01;
            #[doc = "medium(100MHz)"]
            pub const SPEED_2_MEDIUM_100MHZ_: u32 = 0x02;
            #[doc = "max(200MHz)"]
            pub const SPEED_3_MAX_200MHZ_: u32 = 0x03;
        }
    }
    #[doc = "Open Drain Enable Field"]
    pub mod ODE {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Open Drain Disabled"]
            pub const ODE_0_OPEN_DRAIN_DISABLED: u32 = 0;
            #[doc = "Open Drain Enabled"]
            pub const ODE_1_OPEN_DRAIN_ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Pull / Keep Enable Field"]
    pub mod PKE {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pull/Keeper Disabled"]
            pub const PKE_0_PULL_KEEPER_DISABLED: u32 = 0;
            #[doc = "Pull/Keeper Enabled"]
            pub const PKE_1_PULL_KEEPER_ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Pull / Keep Select Field"]
    pub mod PUE {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Keeper"]
            pub const PUE_0_KEEPER: u32 = 0;
            #[doc = "Pull"]
            pub const PUE_1_PULL: u32 = 0x01;
        }
    }
    #[doc = "Pull Up / Down Config. Field"]
    pub mod PUS {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "100K Ohm Pull Down"]
            pub const PUS_0_100K_OHM_PULL_DOWN: u32 = 0;
            #[doc = "47K Ohm Pull Up"]
            pub const PUS_1_47K_OHM_PULL_UP: u32 = 0x01;
            #[doc = "100K Ohm Pull Up"]
            pub const PUS_2_100K_OHM_PULL_UP: u32 = 0x02;
            #[doc = "22K Ohm Pull Up"]
            pub const PUS_3_22K_OHM_PULL_UP: u32 = 0x03;
        }
    }
    #[doc = "Hyst. Enable Field"]
    pub mod HYS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Hysteresis Disabled"]
            pub const HYS_0_HYSTERESIS_DISABLED: u32 = 0;
            #[doc = "Hysteresis Enabled"]
            pub const HYS_1_HYSTERESIS_ENABLED: u32 = 0x01;
        }
    }
}
#[doc = "SW_PAD_CTL_PAD_GPIO_EMC_20 SW PAD Control Register"]
pub mod SW_PAD_CTL_PAD_GPIO_EMC_20 {
    #[doc = "Slew Rate Field"]
    pub mod SRE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Slow Slew Rate"]
            pub const SRE_0_SLOW_SLEW_RATE: u32 = 0;
            #[doc = "Fast Slew Rate"]
            pub const SRE_1_FAST_SLEW_RATE: u32 = 0x01;
        }
    }
    #[doc = "Drive Strength Field"]
    pub mod DSE {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "output driver disabled;"]
            pub const DSE_0_OUTPUT_DRIVER_DISABLED_: u32 = 0;
            #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V)"]
            pub const DSE_1_R0_150_OHM___3_3V__260_OHM_1_8V: u32 = 0x01;
            #[doc = "R0/2"]
            pub const DSE_2_R0_2: u32 = 0x02;
            #[doc = "R0/3"]
            pub const DSE_3_R0_3: u32 = 0x03;
            #[doc = "R0/4"]
            pub const DSE_4_R0_4: u32 = 0x04;
            #[doc = "R0/5"]
            pub const DSE_5_R0_5: u32 = 0x05;
            #[doc = "R0/6"]
            pub const DSE_6_R0_6: u32 = 0x06;
            #[doc = "R0/7"]
            pub const DSE_7_R0_7: u32 = 0x07;
        }
    }
    #[doc = "Speed Field"]
    pub mod SPEED {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "low(50MHz)"]
            pub const SPEED_0_LOW_50MHZ_: u32 = 0;
            #[doc = "medium(100MHz)"]
            pub const SPEED_1_MEDIUM_100MHZ_: u32 = 0x01;
            #[doc = "medium(100MHz)"]
            pub const SPEED_2_MEDIUM_100MHZ_: u32 = 0x02;
            #[doc = "max(200MHz)"]
            pub const SPEED_3_MAX_200MHZ_: u32 = 0x03;
        }
    }
    #[doc = "Open Drain Enable Field"]
    pub mod ODE {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Open Drain Disabled"]
            pub const ODE_0_OPEN_DRAIN_DISABLED: u32 = 0;
            #[doc = "Open Drain Enabled"]
            pub const ODE_1_OPEN_DRAIN_ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Pull / Keep Enable Field"]
    pub mod PKE {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pull/Keeper Disabled"]
            pub const PKE_0_PULL_KEEPER_DISABLED: u32 = 0;
            #[doc = "Pull/Keeper Enabled"]
            pub const PKE_1_PULL_KEEPER_ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Pull / Keep Select Field"]
    pub mod PUE {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Keeper"]
            pub const PUE_0_KEEPER: u32 = 0;
            #[doc = "Pull"]
            pub const PUE_1_PULL: u32 = 0x01;
        }
    }
    #[doc = "Pull Up / Down Config. Field"]
    pub mod PUS {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "100K Ohm Pull Down"]
            pub const PUS_0_100K_OHM_PULL_DOWN: u32 = 0;
            #[doc = "47K Ohm Pull Up"]
            pub const PUS_1_47K_OHM_PULL_UP: u32 = 0x01;
            #[doc = "100K Ohm Pull Up"]
            pub const PUS_2_100K_OHM_PULL_UP: u32 = 0x02;
            #[doc = "22K Ohm Pull Up"]
            pub const PUS_3_22K_OHM_PULL_UP: u32 = 0x03;
        }
    }
    #[doc = "Hyst. Enable Field"]
    pub mod HYS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Hysteresis Disabled"]
            pub const HYS_0_HYSTERESIS_DISABLED: u32 = 0;
            #[doc = "Hysteresis Enabled"]
            pub const HYS_1_HYSTERESIS_ENABLED: u32 = 0x01;
        }
    }
}
#[doc = "SW_PAD_CTL_PAD_GPIO_EMC_21 SW PAD Control Register"]
pub mod SW_PAD_CTL_PAD_GPIO_EMC_21 {
    #[doc = "Slew Rate Field"]
    pub mod SRE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Slow Slew Rate"]
            pub const SRE_0_SLOW_SLEW_RATE: u32 = 0;
            #[doc = "Fast Slew Rate"]
            pub const SRE_1_FAST_SLEW_RATE: u32 = 0x01;
        }
    }
    #[doc = "Drive Strength Field"]
    pub mod DSE {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "output driver disabled;"]
            pub const DSE_0_OUTPUT_DRIVER_DISABLED_: u32 = 0;
            #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V)"]
            pub const DSE_1_R0_150_OHM___3_3V__260_OHM_1_8V: u32 = 0x01;
            #[doc = "R0/2"]
            pub const DSE_2_R0_2: u32 = 0x02;
            #[doc = "R0/3"]
            pub const DSE_3_R0_3: u32 = 0x03;
            #[doc = "R0/4"]
            pub const DSE_4_R0_4: u32 = 0x04;
            #[doc = "R0/5"]
            pub const DSE_5_R0_5: u32 = 0x05;
            #[doc = "R0/6"]
            pub const DSE_6_R0_6: u32 = 0x06;
            #[doc = "R0/7"]
            pub const DSE_7_R0_7: u32 = 0x07;
        }
    }
    #[doc = "Speed Field"]
    pub mod SPEED {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "low(50MHz)"]
            pub const SPEED_0_LOW_50MHZ_: u32 = 0;
            #[doc = "medium(100MHz)"]
            pub const SPEED_1_MEDIUM_100MHZ_: u32 = 0x01;
            #[doc = "medium(100MHz)"]
            pub const SPEED_2_MEDIUM_100MHZ_: u32 = 0x02;
            #[doc = "max(200MHz)"]
            pub const SPEED_3_MAX_200MHZ_: u32 = 0x03;
        }
    }
    #[doc = "Open Drain Enable Field"]
    pub mod ODE {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Open Drain Disabled"]
            pub const ODE_0_OPEN_DRAIN_DISABLED: u32 = 0;
            #[doc = "Open Drain Enabled"]
            pub const ODE_1_OPEN_DRAIN_ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Pull / Keep Enable Field"]
    pub mod PKE {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pull/Keeper Disabled"]
            pub const PKE_0_PULL_KEEPER_DISABLED: u32 = 0;
            #[doc = "Pull/Keeper Enabled"]
            pub const PKE_1_PULL_KEEPER_ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Pull / Keep Select Field"]
    pub mod PUE {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Keeper"]
            pub const PUE_0_KEEPER: u32 = 0;
            #[doc = "Pull"]
            pub const PUE_1_PULL: u32 = 0x01;
        }
    }
    #[doc = "Pull Up / Down Config. Field"]
    pub mod PUS {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "100K Ohm Pull Down"]
            pub const PUS_0_100K_OHM_PULL_DOWN: u32 = 0;
            #[doc = "47K Ohm Pull Up"]
            pub const PUS_1_47K_OHM_PULL_UP: u32 = 0x01;
            #[doc = "100K Ohm Pull Up"]
            pub const PUS_2_100K_OHM_PULL_UP: u32 = 0x02;
            #[doc = "22K Ohm Pull Up"]
            pub const PUS_3_22K_OHM_PULL_UP: u32 = 0x03;
        }
    }
    #[doc = "Hyst. Enable Field"]
    pub mod HYS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Hysteresis Disabled"]
            pub const HYS_0_HYSTERESIS_DISABLED: u32 = 0;
            #[doc = "Hysteresis Enabled"]
            pub const HYS_1_HYSTERESIS_ENABLED: u32 = 0x01;
        }
    }
}
#[doc = "SW_PAD_CTL_PAD_GPIO_EMC_22 SW PAD Control Register"]
pub mod SW_PAD_CTL_PAD_GPIO_EMC_22 {
    #[doc = "Slew Rate Field"]
    pub mod SRE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Slow Slew Rate"]
            pub const SRE_0_SLOW_SLEW_RATE: u32 = 0;
            #[doc = "Fast Slew Rate"]
            pub const SRE_1_FAST_SLEW_RATE: u32 = 0x01;
        }
    }
    #[doc = "Drive Strength Field"]
    pub mod DSE {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "output driver disabled;"]
            pub const DSE_0_OUTPUT_DRIVER_DISABLED_: u32 = 0;
            #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V)"]
            pub const DSE_1_R0_150_OHM___3_3V__260_OHM_1_8V: u32 = 0x01;
            #[doc = "R0/2"]
            pub const DSE_2_R0_2: u32 = 0x02;
            #[doc = "R0/3"]
            pub const DSE_3_R0_3: u32 = 0x03;
            #[doc = "R0/4"]
            pub const DSE_4_R0_4: u32 = 0x04;
            #[doc = "R0/5"]
            pub const DSE_5_R0_5: u32 = 0x05;
            #[doc = "R0/6"]
            pub const DSE_6_R0_6: u32 = 0x06;
            #[doc = "R0/7"]
            pub const DSE_7_R0_7: u32 = 0x07;
        }
    }
    #[doc = "Speed Field"]
    pub mod SPEED {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "low(50MHz)"]
            pub const SPEED_0_LOW_50MHZ_: u32 = 0;
            #[doc = "medium(100MHz)"]
            pub const SPEED_1_MEDIUM_100MHZ_: u32 = 0x01;
            #[doc = "medium(100MHz)"]
            pub const SPEED_2_MEDIUM_100MHZ_: u32 = 0x02;
            #[doc = "max(200MHz)"]
            pub const SPEED_3_MAX_200MHZ_: u32 = 0x03;
        }
    }
    #[doc = "Open Drain Enable Field"]
    pub mod ODE {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Open Drain Disabled"]
            pub const ODE_0_OPEN_DRAIN_DISABLED: u32 = 0;
            #[doc = "Open Drain Enabled"]
            pub const ODE_1_OPEN_DRAIN_ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Pull / Keep Enable Field"]
    pub mod PKE {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pull/Keeper Disabled"]
            pub const PKE_0_PULL_KEEPER_DISABLED: u32 = 0;
            #[doc = "Pull/Keeper Enabled"]
            pub const PKE_1_PULL_KEEPER_ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Pull / Keep Select Field"]
    pub mod PUE {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Keeper"]
            pub const PUE_0_KEEPER: u32 = 0;
            #[doc = "Pull"]
            pub const PUE_1_PULL: u32 = 0x01;
        }
    }
    #[doc = "Pull Up / Down Config. Field"]
    pub mod PUS {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "100K Ohm Pull Down"]
            pub const PUS_0_100K_OHM_PULL_DOWN: u32 = 0;
            #[doc = "47K Ohm Pull Up"]
            pub const PUS_1_47K_OHM_PULL_UP: u32 = 0x01;
            #[doc = "100K Ohm Pull Up"]
            pub const PUS_2_100K_OHM_PULL_UP: u32 = 0x02;
            #[doc = "22K Ohm Pull Up"]
            pub const PUS_3_22K_OHM_PULL_UP: u32 = 0x03;
        }
    }
    #[doc = "Hyst. Enable Field"]
    pub mod HYS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Hysteresis Disabled"]
            pub const HYS_0_HYSTERESIS_DISABLED: u32 = 0;
            #[doc = "Hysteresis Enabled"]
            pub const HYS_1_HYSTERESIS_ENABLED: u32 = 0x01;
        }
    }
}
#[doc = "SW_PAD_CTL_PAD_GPIO_EMC_23 SW PAD Control Register"]
pub mod SW_PAD_CTL_PAD_GPIO_EMC_23 {
    #[doc = "Slew Rate Field"]
    pub mod SRE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Slow Slew Rate"]
            pub const SRE_0_SLOW_SLEW_RATE: u32 = 0;
            #[doc = "Fast Slew Rate"]
            pub const SRE_1_FAST_SLEW_RATE: u32 = 0x01;
        }
    }
    #[doc = "Drive Strength Field"]
    pub mod DSE {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "output driver disabled;"]
            pub const DSE_0_OUTPUT_DRIVER_DISABLED_: u32 = 0;
            #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V)"]
            pub const DSE_1_R0_150_OHM___3_3V__260_OHM_1_8V: u32 = 0x01;
            #[doc = "R0/2"]
            pub const DSE_2_R0_2: u32 = 0x02;
            #[doc = "R0/3"]
            pub const DSE_3_R0_3: u32 = 0x03;
            #[doc = "R0/4"]
            pub const DSE_4_R0_4: u32 = 0x04;
            #[doc = "R0/5"]
            pub const DSE_5_R0_5: u32 = 0x05;
            #[doc = "R0/6"]
            pub const DSE_6_R0_6: u32 = 0x06;
            #[doc = "R0/7"]
            pub const DSE_7_R0_7: u32 = 0x07;
        }
    }
    #[doc = "Speed Field"]
    pub mod SPEED {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "low(50MHz)"]
            pub const SPEED_0_LOW_50MHZ_: u32 = 0;
            #[doc = "medium(100MHz)"]
            pub const SPEED_1_MEDIUM_100MHZ_: u32 = 0x01;
            #[doc = "medium(100MHz)"]
            pub const SPEED_2_MEDIUM_100MHZ_: u32 = 0x02;
            #[doc = "max(200MHz)"]
            pub const SPEED_3_MAX_200MHZ_: u32 = 0x03;
        }
    }
    #[doc = "Open Drain Enable Field"]
    pub mod ODE {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Open Drain Disabled"]
            pub const ODE_0_OPEN_DRAIN_DISABLED: u32 = 0;
            #[doc = "Open Drain Enabled"]
            pub const ODE_1_OPEN_DRAIN_ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Pull / Keep Enable Field"]
    pub mod PKE {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pull/Keeper Disabled"]
            pub const PKE_0_PULL_KEEPER_DISABLED: u32 = 0;
            #[doc = "Pull/Keeper Enabled"]
            pub const PKE_1_PULL_KEEPER_ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Pull / Keep Select Field"]
    pub mod PUE {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Keeper"]
            pub const PUE_0_KEEPER: u32 = 0;
            #[doc = "Pull"]
            pub const PUE_1_PULL: u32 = 0x01;
        }
    }
    #[doc = "Pull Up / Down Config. Field"]
    pub mod PUS {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "100K Ohm Pull Down"]
            pub const PUS_0_100K_OHM_PULL_DOWN: u32 = 0;
            #[doc = "47K Ohm Pull Up"]
            pub const PUS_1_47K_OHM_PULL_UP: u32 = 0x01;
            #[doc = "100K Ohm Pull Up"]
            pub const PUS_2_100K_OHM_PULL_UP: u32 = 0x02;
            #[doc = "22K Ohm Pull Up"]
            pub const PUS_3_22K_OHM_PULL_UP: u32 = 0x03;
        }
    }
    #[doc = "Hyst. Enable Field"]
    pub mod HYS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Hysteresis Disabled"]
            pub const HYS_0_HYSTERESIS_DISABLED: u32 = 0;
            #[doc = "Hysteresis Enabled"]
            pub const HYS_1_HYSTERESIS_ENABLED: u32 = 0x01;
        }
    }
}
#[doc = "SW_PAD_CTL_PAD_GPIO_EMC_24 SW PAD Control Register"]
pub mod SW_PAD_CTL_PAD_GPIO_EMC_24 {
    #[doc = "Slew Rate Field"]
    pub mod SRE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Slow Slew Rate"]
            pub const SRE_0_SLOW_SLEW_RATE: u32 = 0;
            #[doc = "Fast Slew Rate"]
            pub const SRE_1_FAST_SLEW_RATE: u32 = 0x01;
        }
    }
    #[doc = "Drive Strength Field"]
    pub mod DSE {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "output driver disabled;"]
            pub const DSE_0_OUTPUT_DRIVER_DISABLED_: u32 = 0;
            #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V)"]
            pub const DSE_1_R0_150_OHM___3_3V__260_OHM_1_8V: u32 = 0x01;
            #[doc = "R0/2"]
            pub const DSE_2_R0_2: u32 = 0x02;
            #[doc = "R0/3"]
            pub const DSE_3_R0_3: u32 = 0x03;
            #[doc = "R0/4"]
            pub const DSE_4_R0_4: u32 = 0x04;
            #[doc = "R0/5"]
            pub const DSE_5_R0_5: u32 = 0x05;
            #[doc = "R0/6"]
            pub const DSE_6_R0_6: u32 = 0x06;
            #[doc = "R0/7"]
            pub const DSE_7_R0_7: u32 = 0x07;
        }
    }
    #[doc = "Speed Field"]
    pub mod SPEED {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "low(50MHz)"]
            pub const SPEED_0_LOW_50MHZ_: u32 = 0;
            #[doc = "medium(100MHz)"]
            pub const SPEED_1_MEDIUM_100MHZ_: u32 = 0x01;
            #[doc = "medium(100MHz)"]
            pub const SPEED_2_MEDIUM_100MHZ_: u32 = 0x02;
            #[doc = "max(200MHz)"]
            pub const SPEED_3_MAX_200MHZ_: u32 = 0x03;
        }
    }
    #[doc = "Open Drain Enable Field"]
    pub mod ODE {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Open Drain Disabled"]
            pub const ODE_0_OPEN_DRAIN_DISABLED: u32 = 0;
            #[doc = "Open Drain Enabled"]
            pub const ODE_1_OPEN_DRAIN_ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Pull / Keep Enable Field"]
    pub mod PKE {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pull/Keeper Disabled"]
            pub const PKE_0_PULL_KEEPER_DISABLED: u32 = 0;
            #[doc = "Pull/Keeper Enabled"]
            pub const PKE_1_PULL_KEEPER_ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Pull / Keep Select Field"]
    pub mod PUE {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Keeper"]
            pub const PUE_0_KEEPER: u32 = 0;
            #[doc = "Pull"]
            pub const PUE_1_PULL: u32 = 0x01;
        }
    }
    #[doc = "Pull Up / Down Config. Field"]
    pub mod PUS {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "100K Ohm Pull Down"]
            pub const PUS_0_100K_OHM_PULL_DOWN: u32 = 0;
            #[doc = "47K Ohm Pull Up"]
            pub const PUS_1_47K_OHM_PULL_UP: u32 = 0x01;
            #[doc = "100K Ohm Pull Up"]
            pub const PUS_2_100K_OHM_PULL_UP: u32 = 0x02;
            #[doc = "22K Ohm Pull Up"]
            pub const PUS_3_22K_OHM_PULL_UP: u32 = 0x03;
        }
    }
    #[doc = "Hyst. Enable Field"]
    pub mod HYS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Hysteresis Disabled"]
            pub const HYS_0_HYSTERESIS_DISABLED: u32 = 0;
            #[doc = "Hysteresis Enabled"]
            pub const HYS_1_HYSTERESIS_ENABLED: u32 = 0x01;
        }
    }
}
#[doc = "SW_PAD_CTL_PAD_GPIO_EMC_25 SW PAD Control Register"]
pub mod SW_PAD_CTL_PAD_GPIO_EMC_25 {
    #[doc = "Slew Rate Field"]
    pub mod SRE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Slow Slew Rate"]
            pub const SRE_0_SLOW_SLEW_RATE: u32 = 0;
            #[doc = "Fast Slew Rate"]
            pub const SRE_1_FAST_SLEW_RATE: u32 = 0x01;
        }
    }
    #[doc = "Drive Strength Field"]
    pub mod DSE {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "output driver disabled;"]
            pub const DSE_0_OUTPUT_DRIVER_DISABLED_: u32 = 0;
            #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V)"]
            pub const DSE_1_R0_150_OHM___3_3V__260_OHM_1_8V: u32 = 0x01;
            #[doc = "R0/2"]
            pub const DSE_2_R0_2: u32 = 0x02;
            #[doc = "R0/3"]
            pub const DSE_3_R0_3: u32 = 0x03;
            #[doc = "R0/4"]
            pub const DSE_4_R0_4: u32 = 0x04;
            #[doc = "R0/5"]
            pub const DSE_5_R0_5: u32 = 0x05;
            #[doc = "R0/6"]
            pub const DSE_6_R0_6: u32 = 0x06;
            #[doc = "R0/7"]
            pub const DSE_7_R0_7: u32 = 0x07;
        }
    }
    #[doc = "Speed Field"]
    pub mod SPEED {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "low(50MHz)"]
            pub const SPEED_0_LOW_50MHZ_: u32 = 0;
            #[doc = "medium(100MHz)"]
            pub const SPEED_1_MEDIUM_100MHZ_: u32 = 0x01;
            #[doc = "medium(100MHz)"]
            pub const SPEED_2_MEDIUM_100MHZ_: u32 = 0x02;
            #[doc = "max(200MHz)"]
            pub const SPEED_3_MAX_200MHZ_: u32 = 0x03;
        }
    }
    #[doc = "Open Drain Enable Field"]
    pub mod ODE {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Open Drain Disabled"]
            pub const ODE_0_OPEN_DRAIN_DISABLED: u32 = 0;
            #[doc = "Open Drain Enabled"]
            pub const ODE_1_OPEN_DRAIN_ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Pull / Keep Enable Field"]
    pub mod PKE {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pull/Keeper Disabled"]
            pub const PKE_0_PULL_KEEPER_DISABLED: u32 = 0;
            #[doc = "Pull/Keeper Enabled"]
            pub const PKE_1_PULL_KEEPER_ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Pull / Keep Select Field"]
    pub mod PUE {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Keeper"]
            pub const PUE_0_KEEPER: u32 = 0;
            #[doc = "Pull"]
            pub const PUE_1_PULL: u32 = 0x01;
        }
    }
    #[doc = "Pull Up / Down Config. Field"]
    pub mod PUS {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "100K Ohm Pull Down"]
            pub const PUS_0_100K_OHM_PULL_DOWN: u32 = 0;
            #[doc = "47K Ohm Pull Up"]
            pub const PUS_1_47K_OHM_PULL_UP: u32 = 0x01;
            #[doc = "100K Ohm Pull Up"]
            pub const PUS_2_100K_OHM_PULL_UP: u32 = 0x02;
            #[doc = "22K Ohm Pull Up"]
            pub const PUS_3_22K_OHM_PULL_UP: u32 = 0x03;
        }
    }
    #[doc = "Hyst. Enable Field"]
    pub mod HYS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Hysteresis Disabled"]
            pub const HYS_0_HYSTERESIS_DISABLED: u32 = 0;
            #[doc = "Hysteresis Enabled"]
            pub const HYS_1_HYSTERESIS_ENABLED: u32 = 0x01;
        }
    }
}
#[doc = "SW_PAD_CTL_PAD_GPIO_EMC_26 SW PAD Control Register"]
pub mod SW_PAD_CTL_PAD_GPIO_EMC_26 {
    #[doc = "Slew Rate Field"]
    pub mod SRE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Slow Slew Rate"]
            pub const SRE_0_SLOW_SLEW_RATE: u32 = 0;
            #[doc = "Fast Slew Rate"]
            pub const SRE_1_FAST_SLEW_RATE: u32 = 0x01;
        }
    }
    #[doc = "Drive Strength Field"]
    pub mod DSE {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "output driver disabled;"]
            pub const DSE_0_OUTPUT_DRIVER_DISABLED_: u32 = 0;
            #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V)"]
            pub const DSE_1_R0_150_OHM___3_3V__260_OHM_1_8V: u32 = 0x01;
            #[doc = "R0/2"]
            pub const DSE_2_R0_2: u32 = 0x02;
            #[doc = "R0/3"]
            pub const DSE_3_R0_3: u32 = 0x03;
            #[doc = "R0/4"]
            pub const DSE_4_R0_4: u32 = 0x04;
            #[doc = "R0/5"]
            pub const DSE_5_R0_5: u32 = 0x05;
            #[doc = "R0/6"]
            pub const DSE_6_R0_6: u32 = 0x06;
            #[doc = "R0/7"]
            pub const DSE_7_R0_7: u32 = 0x07;
        }
    }
    #[doc = "Speed Field"]
    pub mod SPEED {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "low(50MHz)"]
            pub const SPEED_0_LOW_50MHZ_: u32 = 0;
            #[doc = "medium(100MHz)"]
            pub const SPEED_1_MEDIUM_100MHZ_: u32 = 0x01;
            #[doc = "medium(100MHz)"]
            pub const SPEED_2_MEDIUM_100MHZ_: u32 = 0x02;
            #[doc = "max(200MHz)"]
            pub const SPEED_3_MAX_200MHZ_: u32 = 0x03;
        }
    }
    #[doc = "Open Drain Enable Field"]
    pub mod ODE {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Open Drain Disabled"]
            pub const ODE_0_OPEN_DRAIN_DISABLED: u32 = 0;
            #[doc = "Open Drain Enabled"]
            pub const ODE_1_OPEN_DRAIN_ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Pull / Keep Enable Field"]
    pub mod PKE {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pull/Keeper Disabled"]
            pub const PKE_0_PULL_KEEPER_DISABLED: u32 = 0;
            #[doc = "Pull/Keeper Enabled"]
            pub const PKE_1_PULL_KEEPER_ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Pull / Keep Select Field"]
    pub mod PUE {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Keeper"]
            pub const PUE_0_KEEPER: u32 = 0;
            #[doc = "Pull"]
            pub const PUE_1_PULL: u32 = 0x01;
        }
    }
    #[doc = "Pull Up / Down Config. Field"]
    pub mod PUS {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "100K Ohm Pull Down"]
            pub const PUS_0_100K_OHM_PULL_DOWN: u32 = 0;
            #[doc = "47K Ohm Pull Up"]
            pub const PUS_1_47K_OHM_PULL_UP: u32 = 0x01;
            #[doc = "100K Ohm Pull Up"]
            pub const PUS_2_100K_OHM_PULL_UP: u32 = 0x02;
            #[doc = "22K Ohm Pull Up"]
            pub const PUS_3_22K_OHM_PULL_UP: u32 = 0x03;
        }
    }
    #[doc = "Hyst. Enable Field"]
    pub mod HYS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Hysteresis Disabled"]
            pub const HYS_0_HYSTERESIS_DISABLED: u32 = 0;
            #[doc = "Hysteresis Enabled"]
            pub const HYS_1_HYSTERESIS_ENABLED: u32 = 0x01;
        }
    }
}
#[doc = "SW_PAD_CTL_PAD_GPIO_EMC_27 SW PAD Control Register"]
pub mod SW_PAD_CTL_PAD_GPIO_EMC_27 {
    #[doc = "Slew Rate Field"]
    pub mod SRE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Slow Slew Rate"]
            pub const SRE_0_SLOW_SLEW_RATE: u32 = 0;
            #[doc = "Fast Slew Rate"]
            pub const SRE_1_FAST_SLEW_RATE: u32 = 0x01;
        }
    }
    #[doc = "Drive Strength Field"]
    pub mod DSE {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "output driver disabled;"]
            pub const DSE_0_OUTPUT_DRIVER_DISABLED_: u32 = 0;
            #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V)"]
            pub const DSE_1_R0_150_OHM___3_3V__260_OHM_1_8V: u32 = 0x01;
            #[doc = "R0/2"]
            pub const DSE_2_R0_2: u32 = 0x02;
            #[doc = "R0/3"]
            pub const DSE_3_R0_3: u32 = 0x03;
            #[doc = "R0/4"]
            pub const DSE_4_R0_4: u32 = 0x04;
            #[doc = "R0/5"]
            pub const DSE_5_R0_5: u32 = 0x05;
            #[doc = "R0/6"]
            pub const DSE_6_R0_6: u32 = 0x06;
            #[doc = "R0/7"]
            pub const DSE_7_R0_7: u32 = 0x07;
        }
    }
    #[doc = "Speed Field"]
    pub mod SPEED {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "low(50MHz)"]
            pub const SPEED_0_LOW_50MHZ_: u32 = 0;
            #[doc = "medium(100MHz)"]
            pub const SPEED_1_MEDIUM_100MHZ_: u32 = 0x01;
            #[doc = "medium(100MHz)"]
            pub const SPEED_2_MEDIUM_100MHZ_: u32 = 0x02;
            #[doc = "max(200MHz)"]
            pub const SPEED_3_MAX_200MHZ_: u32 = 0x03;
        }
    }
    #[doc = "Open Drain Enable Field"]
    pub mod ODE {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Open Drain Disabled"]
            pub const ODE_0_OPEN_DRAIN_DISABLED: u32 = 0;
            #[doc = "Open Drain Enabled"]
            pub const ODE_1_OPEN_DRAIN_ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Pull / Keep Enable Field"]
    pub mod PKE {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pull/Keeper Disabled"]
            pub const PKE_0_PULL_KEEPER_DISABLED: u32 = 0;
            #[doc = "Pull/Keeper Enabled"]
            pub const PKE_1_PULL_KEEPER_ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Pull / Keep Select Field"]
    pub mod PUE {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Keeper"]
            pub const PUE_0_KEEPER: u32 = 0;
            #[doc = "Pull"]
            pub const PUE_1_PULL: u32 = 0x01;
        }
    }
    #[doc = "Pull Up / Down Config. Field"]
    pub mod PUS {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "100K Ohm Pull Down"]
            pub const PUS_0_100K_OHM_PULL_DOWN: u32 = 0;
            #[doc = "47K Ohm Pull Up"]
            pub const PUS_1_47K_OHM_PULL_UP: u32 = 0x01;
            #[doc = "100K Ohm Pull Up"]
            pub const PUS_2_100K_OHM_PULL_UP: u32 = 0x02;
            #[doc = "22K Ohm Pull Up"]
            pub const PUS_3_22K_OHM_PULL_UP: u32 = 0x03;
        }
    }
    #[doc = "Hyst. Enable Field"]
    pub mod HYS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Hysteresis Disabled"]
            pub const HYS_0_HYSTERESIS_DISABLED: u32 = 0;
            #[doc = "Hysteresis Enabled"]
            pub const HYS_1_HYSTERESIS_ENABLED: u32 = 0x01;
        }
    }
}
#[doc = "SW_PAD_CTL_PAD_GPIO_EMC_32 SW PAD Control Register"]
pub mod SW_PAD_CTL_PAD_GPIO_EMC_32 {
    #[doc = "Slew Rate Field"]
    pub mod SRE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Slow Slew Rate"]
            pub const SRE_0_SLOW_SLEW_RATE: u32 = 0;
            #[doc = "Fast Slew Rate"]
            pub const SRE_1_FAST_SLEW_RATE: u32 = 0x01;
        }
    }
    #[doc = "Drive Strength Field"]
    pub mod DSE {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "output driver disabled;"]
            pub const DSE_0_OUTPUT_DRIVER_DISABLED_: u32 = 0;
            #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V)"]
            pub const DSE_1_R0_150_OHM___3_3V__260_OHM_1_8V: u32 = 0x01;
            #[doc = "R0/2"]
            pub const DSE_2_R0_2: u32 = 0x02;
            #[doc = "R0/3"]
            pub const DSE_3_R0_3: u32 = 0x03;
            #[doc = "R0/4"]
            pub const DSE_4_R0_4: u32 = 0x04;
            #[doc = "R0/5"]
            pub const DSE_5_R0_5: u32 = 0x05;
            #[doc = "R0/6"]
            pub const DSE_6_R0_6: u32 = 0x06;
            #[doc = "R0/7"]
            pub const DSE_7_R0_7: u32 = 0x07;
        }
    }
    #[doc = "Speed Field"]
    pub mod SPEED {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "low(50MHz)"]
            pub const SPEED_0_LOW_50MHZ_: u32 = 0;
            #[doc = "medium(100MHz)"]
            pub const SPEED_1_MEDIUM_100MHZ_: u32 = 0x01;
            #[doc = "medium(100MHz)"]
            pub const SPEED_2_MEDIUM_100MHZ_: u32 = 0x02;
            #[doc = "max(200MHz)"]
            pub const SPEED_3_MAX_200MHZ_: u32 = 0x03;
        }
    }
    #[doc = "Open Drain Enable Field"]
    pub mod ODE {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Open Drain Disabled"]
            pub const ODE_0_OPEN_DRAIN_DISABLED: u32 = 0;
            #[doc = "Open Drain Enabled"]
            pub const ODE_1_OPEN_DRAIN_ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Pull / Keep Enable Field"]
    pub mod PKE {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pull/Keeper Disabled"]
            pub const PKE_0_PULL_KEEPER_DISABLED: u32 = 0;
            #[doc = "Pull/Keeper Enabled"]
            pub const PKE_1_PULL_KEEPER_ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Pull / Keep Select Field"]
    pub mod PUE {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Keeper"]
            pub const PUE_0_KEEPER: u32 = 0;
            #[doc = "Pull"]
            pub const PUE_1_PULL: u32 = 0x01;
        }
    }
    #[doc = "Pull Up / Down Config. Field"]
    pub mod PUS {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "100K Ohm Pull Down"]
            pub const PUS_0_100K_OHM_PULL_DOWN: u32 = 0;
            #[doc = "47K Ohm Pull Up"]
            pub const PUS_1_47K_OHM_PULL_UP: u32 = 0x01;
            #[doc = "100K Ohm Pull Up"]
            pub const PUS_2_100K_OHM_PULL_UP: u32 = 0x02;
            #[doc = "22K Ohm Pull Up"]
            pub const PUS_3_22K_OHM_PULL_UP: u32 = 0x03;
        }
    }
    #[doc = "Hyst. Enable Field"]
    pub mod HYS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Hysteresis Disabled"]
            pub const HYS_0_HYSTERESIS_DISABLED: u32 = 0;
            #[doc = "Hysteresis Enabled"]
            pub const HYS_1_HYSTERESIS_ENABLED: u32 = 0x01;
        }
    }
}
#[doc = "SW_PAD_CTL_PAD_GPIO_EMC_33 SW PAD Control Register"]
pub mod SW_PAD_CTL_PAD_GPIO_EMC_33 {
    #[doc = "Slew Rate Field"]
    pub mod SRE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Slow Slew Rate"]
            pub const SRE_0_SLOW_SLEW_RATE: u32 = 0;
            #[doc = "Fast Slew Rate"]
            pub const SRE_1_FAST_SLEW_RATE: u32 = 0x01;
        }
    }
    #[doc = "Drive Strength Field"]
    pub mod DSE {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "output driver disabled;"]
            pub const DSE_0_OUTPUT_DRIVER_DISABLED_: u32 = 0;
            #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V)"]
            pub const DSE_1_R0_150_OHM___3_3V__260_OHM_1_8V: u32 = 0x01;
            #[doc = "R0/2"]
            pub const DSE_2_R0_2: u32 = 0x02;
            #[doc = "R0/3"]
            pub const DSE_3_R0_3: u32 = 0x03;
            #[doc = "R0/4"]
            pub const DSE_4_R0_4: u32 = 0x04;
            #[doc = "R0/5"]
            pub const DSE_5_R0_5: u32 = 0x05;
            #[doc = "R0/6"]
            pub const DSE_6_R0_6: u32 = 0x06;
            #[doc = "R0/7"]
            pub const DSE_7_R0_7: u32 = 0x07;
        }
    }
    #[doc = "Speed Field"]
    pub mod SPEED {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "low(50MHz)"]
            pub const SPEED_0_LOW_50MHZ_: u32 = 0;
            #[doc = "medium(100MHz)"]
            pub const SPEED_1_MEDIUM_100MHZ_: u32 = 0x01;
            #[doc = "medium(100MHz)"]
            pub const SPEED_2_MEDIUM_100MHZ_: u32 = 0x02;
            #[doc = "max(200MHz)"]
            pub const SPEED_3_MAX_200MHZ_: u32 = 0x03;
        }
    }
    #[doc = "Open Drain Enable Field"]
    pub mod ODE {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Open Drain Disabled"]
            pub const ODE_0_OPEN_DRAIN_DISABLED: u32 = 0;
            #[doc = "Open Drain Enabled"]
            pub const ODE_1_OPEN_DRAIN_ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Pull / Keep Enable Field"]
    pub mod PKE {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pull/Keeper Disabled"]
            pub const PKE_0_PULL_KEEPER_DISABLED: u32 = 0;
            #[doc = "Pull/Keeper Enabled"]
            pub const PKE_1_PULL_KEEPER_ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Pull / Keep Select Field"]
    pub mod PUE {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Keeper"]
            pub const PUE_0_KEEPER: u32 = 0;
            #[doc = "Pull"]
            pub const PUE_1_PULL: u32 = 0x01;
        }
    }
    #[doc = "Pull Up / Down Config. Field"]
    pub mod PUS {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "100K Ohm Pull Down"]
            pub const PUS_0_100K_OHM_PULL_DOWN: u32 = 0;
            #[doc = "47K Ohm Pull Up"]
            pub const PUS_1_47K_OHM_PULL_UP: u32 = 0x01;
            #[doc = "100K Ohm Pull Up"]
            pub const PUS_2_100K_OHM_PULL_UP: u32 = 0x02;
            #[doc = "22K Ohm Pull Up"]
            pub const PUS_3_22K_OHM_PULL_UP: u32 = 0x03;
        }
    }
    #[doc = "Hyst. Enable Field"]
    pub mod HYS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Hysteresis Disabled"]
            pub const HYS_0_HYSTERESIS_DISABLED: u32 = 0;
            #[doc = "Hysteresis Enabled"]
            pub const HYS_1_HYSTERESIS_ENABLED: u32 = 0x01;
        }
    }
}
#[doc = "SW_PAD_CTL_PAD_GPIO_EMC_34 SW PAD Control Register"]
pub mod SW_PAD_CTL_PAD_GPIO_EMC_34 {
    #[doc = "Slew Rate Field"]
    pub mod SRE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Slow Slew Rate"]
            pub const SRE_0_SLOW_SLEW_RATE: u32 = 0;
            #[doc = "Fast Slew Rate"]
            pub const SRE_1_FAST_SLEW_RATE: u32 = 0x01;
        }
    }
    #[doc = "Drive Strength Field"]
    pub mod DSE {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "output driver disabled;"]
            pub const DSE_0_OUTPUT_DRIVER_DISABLED_: u32 = 0;
            #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V)"]
            pub const DSE_1_R0_150_OHM___3_3V__260_OHM_1_8V: u32 = 0x01;
            #[doc = "R0/2"]
            pub const DSE_2_R0_2: u32 = 0x02;
            #[doc = "R0/3"]
            pub const DSE_3_R0_3: u32 = 0x03;
            #[doc = "R0/4"]
            pub const DSE_4_R0_4: u32 = 0x04;
            #[doc = "R0/5"]
            pub const DSE_5_R0_5: u32 = 0x05;
            #[doc = "R0/6"]
            pub const DSE_6_R0_6: u32 = 0x06;
            #[doc = "R0/7"]
            pub const DSE_7_R0_7: u32 = 0x07;
        }
    }
    #[doc = "Speed Field"]
    pub mod SPEED {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "low(50MHz)"]
            pub const SPEED_0_LOW_50MHZ_: u32 = 0;
            #[doc = "medium(100MHz)"]
            pub const SPEED_1_MEDIUM_100MHZ_: u32 = 0x01;
            #[doc = "medium(100MHz)"]
            pub const SPEED_2_MEDIUM_100MHZ_: u32 = 0x02;
            #[doc = "max(200MHz)"]
            pub const SPEED_3_MAX_200MHZ_: u32 = 0x03;
        }
    }
    #[doc = "Open Drain Enable Field"]
    pub mod ODE {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Open Drain Disabled"]
            pub const ODE_0_OPEN_DRAIN_DISABLED: u32 = 0;
            #[doc = "Open Drain Enabled"]
            pub const ODE_1_OPEN_DRAIN_ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Pull / Keep Enable Field"]
    pub mod PKE {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pull/Keeper Disabled"]
            pub const PKE_0_PULL_KEEPER_DISABLED: u32 = 0;
            #[doc = "Pull/Keeper Enabled"]
            pub const PKE_1_PULL_KEEPER_ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Pull / Keep Select Field"]
    pub mod PUE {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Keeper"]
            pub const PUE_0_KEEPER: u32 = 0;
            #[doc = "Pull"]
            pub const PUE_1_PULL: u32 = 0x01;
        }
    }
    #[doc = "Pull Up / Down Config. Field"]
    pub mod PUS {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "100K Ohm Pull Down"]
            pub const PUS_0_100K_OHM_PULL_DOWN: u32 = 0;
            #[doc = "47K Ohm Pull Up"]
            pub const PUS_1_47K_OHM_PULL_UP: u32 = 0x01;
            #[doc = "100K Ohm Pull Up"]
            pub const PUS_2_100K_OHM_PULL_UP: u32 = 0x02;
            #[doc = "22K Ohm Pull Up"]
            pub const PUS_3_22K_OHM_PULL_UP: u32 = 0x03;
        }
    }
    #[doc = "Hyst. Enable Field"]
    pub mod HYS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Hysteresis Disabled"]
            pub const HYS_0_HYSTERESIS_DISABLED: u32 = 0;
            #[doc = "Hysteresis Enabled"]
            pub const HYS_1_HYSTERESIS_ENABLED: u32 = 0x01;
        }
    }
}
#[doc = "SW_PAD_CTL_PAD_GPIO_EMC_35 SW PAD Control Register"]
pub mod SW_PAD_CTL_PAD_GPIO_EMC_35 {
    #[doc = "Slew Rate Field"]
    pub mod SRE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Slow Slew Rate"]
            pub const SRE_0_SLOW_SLEW_RATE: u32 = 0;
            #[doc = "Fast Slew Rate"]
            pub const SRE_1_FAST_SLEW_RATE: u32 = 0x01;
        }
    }
    #[doc = "Drive Strength Field"]
    pub mod DSE {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "output driver disabled;"]
            pub const DSE_0_OUTPUT_DRIVER_DISABLED_: u32 = 0;
            #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V)"]
            pub const DSE_1_R0_150_OHM___3_3V__260_OHM_1_8V: u32 = 0x01;
            #[doc = "R0/2"]
            pub const DSE_2_R0_2: u32 = 0x02;
            #[doc = "R0/3"]
            pub const DSE_3_R0_3: u32 = 0x03;
            #[doc = "R0/4"]
            pub const DSE_4_R0_4: u32 = 0x04;
            #[doc = "R0/5"]
            pub const DSE_5_R0_5: u32 = 0x05;
            #[doc = "R0/6"]
            pub const DSE_6_R0_6: u32 = 0x06;
            #[doc = "R0/7"]
            pub const DSE_7_R0_7: u32 = 0x07;
        }
    }
    #[doc = "Speed Field"]
    pub mod SPEED {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "low(50MHz)"]
            pub const SPEED_0_LOW_50MHZ_: u32 = 0;
            #[doc = "medium(100MHz)"]
            pub const SPEED_1_MEDIUM_100MHZ_: u32 = 0x01;
            #[doc = "medium(100MHz)"]
            pub const SPEED_2_MEDIUM_100MHZ_: u32 = 0x02;
            #[doc = "max(200MHz)"]
            pub const SPEED_3_MAX_200MHZ_: u32 = 0x03;
        }
    }
    #[doc = "Open Drain Enable Field"]
    pub mod ODE {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Open Drain Disabled"]
            pub const ODE_0_OPEN_DRAIN_DISABLED: u32 = 0;
            #[doc = "Open Drain Enabled"]
            pub const ODE_1_OPEN_DRAIN_ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Pull / Keep Enable Field"]
    pub mod PKE {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pull/Keeper Disabled"]
            pub const PKE_0_PULL_KEEPER_DISABLED: u32 = 0;
            #[doc = "Pull/Keeper Enabled"]
            pub const PKE_1_PULL_KEEPER_ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Pull / Keep Select Field"]
    pub mod PUE {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Keeper"]
            pub const PUE_0_KEEPER: u32 = 0;
            #[doc = "Pull"]
            pub const PUE_1_PULL: u32 = 0x01;
        }
    }
    #[doc = "Pull Up / Down Config. Field"]
    pub mod PUS {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "100K Ohm Pull Down"]
            pub const PUS_0_100K_OHM_PULL_DOWN: u32 = 0;
            #[doc = "47K Ohm Pull Up"]
            pub const PUS_1_47K_OHM_PULL_UP: u32 = 0x01;
            #[doc = "100K Ohm Pull Up"]
            pub const PUS_2_100K_OHM_PULL_UP: u32 = 0x02;
            #[doc = "22K Ohm Pull Up"]
            pub const PUS_3_22K_OHM_PULL_UP: u32 = 0x03;
        }
    }
    #[doc = "Hyst. Enable Field"]
    pub mod HYS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Hysteresis Disabled"]
            pub const HYS_0_HYSTERESIS_DISABLED: u32 = 0;
            #[doc = "Hysteresis Enabled"]
            pub const HYS_1_HYSTERESIS_ENABLED: u32 = 0x01;
        }
    }
}
#[doc = "SW_PAD_CTL_PAD_GPIO_AD_B0_00 SW PAD Control Register"]
pub mod SW_PAD_CTL_PAD_GPIO_AD_B0_00 {
    #[doc = "Slew Rate Field"]
    pub mod SRE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Slow Slew Rate"]
            pub const SRE_0_SLOW_SLEW_RATE: u32 = 0;
            #[doc = "Fast Slew Rate"]
            pub const SRE_1_FAST_SLEW_RATE: u32 = 0x01;
        }
    }
    #[doc = "Drive Strength Field"]
    pub mod DSE {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "output driver disabled;"]
            pub const DSE_0_OUTPUT_DRIVER_DISABLED_: u32 = 0;
            #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V)"]
            pub const DSE_1_R0_150_OHM___3_3V__260_OHM_1_8V: u32 = 0x01;
            #[doc = "R0/2"]
            pub const DSE_2_R0_2: u32 = 0x02;
            #[doc = "R0/3"]
            pub const DSE_3_R0_3: u32 = 0x03;
            #[doc = "R0/4"]
            pub const DSE_4_R0_4: u32 = 0x04;
            #[doc = "R0/5"]
            pub const DSE_5_R0_5: u32 = 0x05;
            #[doc = "R0/6"]
            pub const DSE_6_R0_6: u32 = 0x06;
            #[doc = "R0/7"]
            pub const DSE_7_R0_7: u32 = 0x07;
        }
    }
    #[doc = "Speed Field"]
    pub mod SPEED {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "low(50MHz)"]
            pub const SPEED_0_LOW_50MHZ_: u32 = 0;
            #[doc = "medium(100MHz)"]
            pub const SPEED_1_MEDIUM_100MHZ_: u32 = 0x01;
            #[doc = "medium(100MHz)"]
            pub const SPEED_2_MEDIUM_100MHZ_: u32 = 0x02;
            #[doc = "max(200MHz)"]
            pub const SPEED_3_MAX_200MHZ_: u32 = 0x03;
        }
    }
    #[doc = "Open Drain Enable Field"]
    pub mod ODE {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Open Drain Disabled"]
            pub const ODE_0_OPEN_DRAIN_DISABLED: u32 = 0;
            #[doc = "Open Drain Enabled"]
            pub const ODE_1_OPEN_DRAIN_ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Pull / Keep Enable Field"]
    pub mod PKE {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pull/Keeper Disabled"]
            pub const PKE_0_PULL_KEEPER_DISABLED: u32 = 0;
            #[doc = "Pull/Keeper Enabled"]
            pub const PKE_1_PULL_KEEPER_ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Pull / Keep Select Field"]
    pub mod PUE {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Keeper"]
            pub const PUE_0_KEEPER: u32 = 0;
            #[doc = "Pull"]
            pub const PUE_1_PULL: u32 = 0x01;
        }
    }
    #[doc = "Pull Up / Down Config. Field"]
    pub mod PUS {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "100K Ohm Pull Down"]
            pub const PUS_0_100K_OHM_PULL_DOWN: u32 = 0;
            #[doc = "47K Ohm Pull Up"]
            pub const PUS_1_47K_OHM_PULL_UP: u32 = 0x01;
            #[doc = "100K Ohm Pull Up"]
            pub const PUS_2_100K_OHM_PULL_UP: u32 = 0x02;
            #[doc = "22K Ohm Pull Up"]
            pub const PUS_3_22K_OHM_PULL_UP: u32 = 0x03;
        }
    }
    #[doc = "Hyst. Enable Field"]
    pub mod HYS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Hysteresis Disabled"]
            pub const HYS_0_HYSTERESIS_DISABLED: u32 = 0;
            #[doc = "Hysteresis Enabled"]
            pub const HYS_1_HYSTERESIS_ENABLED: u32 = 0x01;
        }
    }
}
#[doc = "SW_PAD_CTL_PAD_GPIO_AD_B0_01 SW PAD Control Register"]
pub mod SW_PAD_CTL_PAD_GPIO_AD_B0_01 {
    #[doc = "Slew Rate Field"]
    pub mod SRE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Slow Slew Rate"]
            pub const SRE_0_SLOW_SLEW_RATE: u32 = 0;
            #[doc = "Fast Slew Rate"]
            pub const SRE_1_FAST_SLEW_RATE: u32 = 0x01;
        }
    }
    #[doc = "Drive Strength Field"]
    pub mod DSE {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "output driver disabled;"]
            pub const DSE_0_OUTPUT_DRIVER_DISABLED_: u32 = 0;
            #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V)"]
            pub const DSE_1_R0_150_OHM___3_3V__260_OHM_1_8V: u32 = 0x01;
            #[doc = "R0/2"]
            pub const DSE_2_R0_2: u32 = 0x02;
            #[doc = "R0/3"]
            pub const DSE_3_R0_3: u32 = 0x03;
            #[doc = "R0/4"]
            pub const DSE_4_R0_4: u32 = 0x04;
            #[doc = "R0/5"]
            pub const DSE_5_R0_5: u32 = 0x05;
            #[doc = "R0/6"]
            pub const DSE_6_R0_6: u32 = 0x06;
            #[doc = "R0/7"]
            pub const DSE_7_R0_7: u32 = 0x07;
        }
    }
    #[doc = "Speed Field"]
    pub mod SPEED {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "low(50MHz)"]
            pub const SPEED_0_LOW_50MHZ_: u32 = 0;
            #[doc = "medium(100MHz)"]
            pub const SPEED_1_MEDIUM_100MHZ_: u32 = 0x01;
            #[doc = "medium(100MHz)"]
            pub const SPEED_2_MEDIUM_100MHZ_: u32 = 0x02;
            #[doc = "max(200MHz)"]
            pub const SPEED_3_MAX_200MHZ_: u32 = 0x03;
        }
    }
    #[doc = "Open Drain Enable Field"]
    pub mod ODE {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Open Drain Disabled"]
            pub const ODE_0_OPEN_DRAIN_DISABLED: u32 = 0;
            #[doc = "Open Drain Enabled"]
            pub const ODE_1_OPEN_DRAIN_ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Pull / Keep Enable Field"]
    pub mod PKE {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pull/Keeper Disabled"]
            pub const PKE_0_PULL_KEEPER_DISABLED: u32 = 0;
            #[doc = "Pull/Keeper Enabled"]
            pub const PKE_1_PULL_KEEPER_ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Pull / Keep Select Field"]
    pub mod PUE {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Keeper"]
            pub const PUE_0_KEEPER: u32 = 0;
            #[doc = "Pull"]
            pub const PUE_1_PULL: u32 = 0x01;
        }
    }
    #[doc = "Pull Up / Down Config. Field"]
    pub mod PUS {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "100K Ohm Pull Down"]
            pub const PUS_0_100K_OHM_PULL_DOWN: u32 = 0;
            #[doc = "47K Ohm Pull Up"]
            pub const PUS_1_47K_OHM_PULL_UP: u32 = 0x01;
            #[doc = "100K Ohm Pull Up"]
            pub const PUS_2_100K_OHM_PULL_UP: u32 = 0x02;
            #[doc = "22K Ohm Pull Up"]
            pub const PUS_3_22K_OHM_PULL_UP: u32 = 0x03;
        }
    }
    #[doc = "Hyst. Enable Field"]
    pub mod HYS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Hysteresis Disabled"]
            pub const HYS_0_HYSTERESIS_DISABLED: u32 = 0;
            #[doc = "Hysteresis Enabled"]
            pub const HYS_1_HYSTERESIS_ENABLED: u32 = 0x01;
        }
    }
}
#[doc = "SW_PAD_CTL_PAD_GPIO_AD_B0_02 SW PAD Control Register"]
pub mod SW_PAD_CTL_PAD_GPIO_AD_B0_02 {
    #[doc = "Slew Rate Field"]
    pub mod SRE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Slow Slew Rate"]
            pub const SRE_0_SLOW_SLEW_RATE: u32 = 0;
            #[doc = "Fast Slew Rate"]
            pub const SRE_1_FAST_SLEW_RATE: u32 = 0x01;
        }
    }
    #[doc = "Drive Strength Field"]
    pub mod DSE {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "output driver disabled;"]
            pub const DSE_0_OUTPUT_DRIVER_DISABLED_: u32 = 0;
            #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V)"]
            pub const DSE_1_R0_150_OHM___3_3V__260_OHM_1_8V: u32 = 0x01;
            #[doc = "R0/2"]
            pub const DSE_2_R0_2: u32 = 0x02;
            #[doc = "R0/3"]
            pub const DSE_3_R0_3: u32 = 0x03;
            #[doc = "R0/4"]
            pub const DSE_4_R0_4: u32 = 0x04;
            #[doc = "R0/5"]
            pub const DSE_5_R0_5: u32 = 0x05;
            #[doc = "R0/6"]
            pub const DSE_6_R0_6: u32 = 0x06;
            #[doc = "R0/7"]
            pub const DSE_7_R0_7: u32 = 0x07;
        }
    }
    #[doc = "Speed Field"]
    pub mod SPEED {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "low(50MHz)"]
            pub const SPEED_0_LOW_50MHZ_: u32 = 0;
            #[doc = "medium(100MHz)"]
            pub const SPEED_1_MEDIUM_100MHZ_: u32 = 0x01;
            #[doc = "medium(100MHz)"]
            pub const SPEED_2_MEDIUM_100MHZ_: u32 = 0x02;
            #[doc = "max(200MHz)"]
            pub const SPEED_3_MAX_200MHZ_: u32 = 0x03;
        }
    }
    #[doc = "Open Drain Enable Field"]
    pub mod ODE {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Open Drain Disabled"]
            pub const ODE_0_OPEN_DRAIN_DISABLED: u32 = 0;
            #[doc = "Open Drain Enabled"]
            pub const ODE_1_OPEN_DRAIN_ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Pull / Keep Enable Field"]
    pub mod PKE {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pull/Keeper Disabled"]
            pub const PKE_0_PULL_KEEPER_DISABLED: u32 = 0;
            #[doc = "Pull/Keeper Enabled"]
            pub const PKE_1_PULL_KEEPER_ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Pull / Keep Select Field"]
    pub mod PUE {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Keeper"]
            pub const PUE_0_KEEPER: u32 = 0;
            #[doc = "Pull"]
            pub const PUE_1_PULL: u32 = 0x01;
        }
    }
    #[doc = "Pull Up / Down Config. Field"]
    pub mod PUS {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "100K Ohm Pull Down"]
            pub const PUS_0_100K_OHM_PULL_DOWN: u32 = 0;
            #[doc = "47K Ohm Pull Up"]
            pub const PUS_1_47K_OHM_PULL_UP: u32 = 0x01;
            #[doc = "100K Ohm Pull Up"]
            pub const PUS_2_100K_OHM_PULL_UP: u32 = 0x02;
            #[doc = "22K Ohm Pull Up"]
            pub const PUS_3_22K_OHM_PULL_UP: u32 = 0x03;
        }
    }
    #[doc = "Hyst. Enable Field"]
    pub mod HYS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Hysteresis Disabled"]
            pub const HYS_0_HYSTERESIS_DISABLED: u32 = 0;
            #[doc = "Hysteresis Enabled"]
            pub const HYS_1_HYSTERESIS_ENABLED: u32 = 0x01;
        }
    }
}
#[doc = "SW_PAD_CTL_PAD_GPIO_AD_B0_03 SW PAD Control Register"]
pub mod SW_PAD_CTL_PAD_GPIO_AD_B0_03 {
    #[doc = "Slew Rate Field"]
    pub mod SRE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Slow Slew Rate"]
            pub const SRE_0_SLOW_SLEW_RATE: u32 = 0;
            #[doc = "Fast Slew Rate"]
            pub const SRE_1_FAST_SLEW_RATE: u32 = 0x01;
        }
    }
    #[doc = "Drive Strength Field"]
    pub mod DSE {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "output driver disabled;"]
            pub const DSE_0_OUTPUT_DRIVER_DISABLED_: u32 = 0;
            #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V)"]
            pub const DSE_1_R0_150_OHM___3_3V__260_OHM_1_8V: u32 = 0x01;
            #[doc = "R0/2"]
            pub const DSE_2_R0_2: u32 = 0x02;
            #[doc = "R0/3"]
            pub const DSE_3_R0_3: u32 = 0x03;
            #[doc = "R0/4"]
            pub const DSE_4_R0_4: u32 = 0x04;
            #[doc = "R0/5"]
            pub const DSE_5_R0_5: u32 = 0x05;
            #[doc = "R0/6"]
            pub const DSE_6_R0_6: u32 = 0x06;
            #[doc = "R0/7"]
            pub const DSE_7_R0_7: u32 = 0x07;
        }
    }
    #[doc = "Speed Field"]
    pub mod SPEED {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "low(50MHz)"]
            pub const SPEED_0_LOW_50MHZ_: u32 = 0;
            #[doc = "medium(100MHz)"]
            pub const SPEED_1_MEDIUM_100MHZ_: u32 = 0x01;
            #[doc = "medium(100MHz)"]
            pub const SPEED_2_MEDIUM_100MHZ_: u32 = 0x02;
            #[doc = "max(200MHz)"]
            pub const SPEED_3_MAX_200MHZ_: u32 = 0x03;
        }
    }
    #[doc = "Open Drain Enable Field"]
    pub mod ODE {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Open Drain Disabled"]
            pub const ODE_0_OPEN_DRAIN_DISABLED: u32 = 0;
            #[doc = "Open Drain Enabled"]
            pub const ODE_1_OPEN_DRAIN_ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Pull / Keep Enable Field"]
    pub mod PKE {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pull/Keeper Disabled"]
            pub const PKE_0_PULL_KEEPER_DISABLED: u32 = 0;
            #[doc = "Pull/Keeper Enabled"]
            pub const PKE_1_PULL_KEEPER_ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Pull / Keep Select Field"]
    pub mod PUE {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Keeper"]
            pub const PUE_0_KEEPER: u32 = 0;
            #[doc = "Pull"]
            pub const PUE_1_PULL: u32 = 0x01;
        }
    }
    #[doc = "Pull Up / Down Config. Field"]
    pub mod PUS {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "100K Ohm Pull Down"]
            pub const PUS_0_100K_OHM_PULL_DOWN: u32 = 0;
            #[doc = "47K Ohm Pull Up"]
            pub const PUS_1_47K_OHM_PULL_UP: u32 = 0x01;
            #[doc = "100K Ohm Pull Up"]
            pub const PUS_2_100K_OHM_PULL_UP: u32 = 0x02;
            #[doc = "22K Ohm Pull Up"]
            pub const PUS_3_22K_OHM_PULL_UP: u32 = 0x03;
        }
    }
    #[doc = "Hyst. Enable Field"]
    pub mod HYS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Hysteresis Disabled"]
            pub const HYS_0_HYSTERESIS_DISABLED: u32 = 0;
            #[doc = "Hysteresis Enabled"]
            pub const HYS_1_HYSTERESIS_ENABLED: u32 = 0x01;
        }
    }
}
#[doc = "SW_PAD_CTL_PAD_GPIO_AD_B0_04 SW PAD Control Register"]
pub mod SW_PAD_CTL_PAD_GPIO_AD_B0_04 {
    #[doc = "Slew Rate Field"]
    pub mod SRE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Slow Slew Rate"]
            pub const SRE_0_SLOW_SLEW_RATE: u32 = 0;
            #[doc = "Fast Slew Rate"]
            pub const SRE_1_FAST_SLEW_RATE: u32 = 0x01;
        }
    }
    #[doc = "Drive Strength Field"]
    pub mod DSE {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "output driver disabled;"]
            pub const DSE_0_OUTPUT_DRIVER_DISABLED_: u32 = 0;
            #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V)"]
            pub const DSE_1_R0_150_OHM___3_3V__260_OHM_1_8V: u32 = 0x01;
            #[doc = "R0/2"]
            pub const DSE_2_R0_2: u32 = 0x02;
            #[doc = "R0/3"]
            pub const DSE_3_R0_3: u32 = 0x03;
            #[doc = "R0/4"]
            pub const DSE_4_R0_4: u32 = 0x04;
            #[doc = "R0/5"]
            pub const DSE_5_R0_5: u32 = 0x05;
            #[doc = "R0/6"]
            pub const DSE_6_R0_6: u32 = 0x06;
            #[doc = "R0/7"]
            pub const DSE_7_R0_7: u32 = 0x07;
        }
    }
    #[doc = "Speed Field"]
    pub mod SPEED {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "low(50MHz)"]
            pub const SPEED_0_LOW_50MHZ_: u32 = 0;
            #[doc = "medium(100MHz)"]
            pub const SPEED_1_MEDIUM_100MHZ_: u32 = 0x01;
            #[doc = "medium(100MHz)"]
            pub const SPEED_2_MEDIUM_100MHZ_: u32 = 0x02;
            #[doc = "max(200MHz)"]
            pub const SPEED_3_MAX_200MHZ_: u32 = 0x03;
        }
    }
    #[doc = "Open Drain Enable Field"]
    pub mod ODE {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Open Drain Disabled"]
            pub const ODE_0_OPEN_DRAIN_DISABLED: u32 = 0;
            #[doc = "Open Drain Enabled"]
            pub const ODE_1_OPEN_DRAIN_ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Pull / Keep Enable Field"]
    pub mod PKE {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pull/Keeper Disabled"]
            pub const PKE_0_PULL_KEEPER_DISABLED: u32 = 0;
            #[doc = "Pull/Keeper Enabled"]
            pub const PKE_1_PULL_KEEPER_ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Pull / Keep Select Field"]
    pub mod PUE {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Keeper"]
            pub const PUE_0_KEEPER: u32 = 0;
            #[doc = "Pull"]
            pub const PUE_1_PULL: u32 = 0x01;
        }
    }
    #[doc = "Pull Up / Down Config. Field"]
    pub mod PUS {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "100K Ohm Pull Down"]
            pub const PUS_0_100K_OHM_PULL_DOWN: u32 = 0;
            #[doc = "47K Ohm Pull Up"]
            pub const PUS_1_47K_OHM_PULL_UP: u32 = 0x01;
            #[doc = "100K Ohm Pull Up"]
            pub const PUS_2_100K_OHM_PULL_UP: u32 = 0x02;
            #[doc = "22K Ohm Pull Up"]
            pub const PUS_3_22K_OHM_PULL_UP: u32 = 0x03;
        }
    }
    #[doc = "Hyst. Enable Field"]
    pub mod HYS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Hysteresis Disabled"]
            pub const HYS_0_HYSTERESIS_DISABLED: u32 = 0;
            #[doc = "Hysteresis Enabled"]
            pub const HYS_1_HYSTERESIS_ENABLED: u32 = 0x01;
        }
    }
}
#[doc = "SW_PAD_CTL_PAD_GPIO_AD_B0_05 SW PAD Control Register"]
pub mod SW_PAD_CTL_PAD_GPIO_AD_B0_05 {
    #[doc = "Slew Rate Field"]
    pub mod SRE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Slow Slew Rate"]
            pub const SRE_0_SLOW_SLEW_RATE: u32 = 0;
            #[doc = "Fast Slew Rate"]
            pub const SRE_1_FAST_SLEW_RATE: u32 = 0x01;
        }
    }
    #[doc = "Drive Strength Field"]
    pub mod DSE {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "output driver disabled;"]
            pub const DSE_0_OUTPUT_DRIVER_DISABLED_: u32 = 0;
            #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V)"]
            pub const DSE_1_R0_150_OHM___3_3V__260_OHM_1_8V: u32 = 0x01;
            #[doc = "R0/2"]
            pub const DSE_2_R0_2: u32 = 0x02;
            #[doc = "R0/3"]
            pub const DSE_3_R0_3: u32 = 0x03;
            #[doc = "R0/4"]
            pub const DSE_4_R0_4: u32 = 0x04;
            #[doc = "R0/5"]
            pub const DSE_5_R0_5: u32 = 0x05;
            #[doc = "R0/6"]
            pub const DSE_6_R0_6: u32 = 0x06;
            #[doc = "R0/7"]
            pub const DSE_7_R0_7: u32 = 0x07;
        }
    }
    #[doc = "Speed Field"]
    pub mod SPEED {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "low(50MHz)"]
            pub const SPEED_0_LOW_50MHZ_: u32 = 0;
            #[doc = "medium(100MHz)"]
            pub const SPEED_1_MEDIUM_100MHZ_: u32 = 0x01;
            #[doc = "medium(100MHz)"]
            pub const SPEED_2_MEDIUM_100MHZ_: u32 = 0x02;
            #[doc = "max(200MHz)"]
            pub const SPEED_3_MAX_200MHZ_: u32 = 0x03;
        }
    }
    #[doc = "Open Drain Enable Field"]
    pub mod ODE {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Open Drain Disabled"]
            pub const ODE_0_OPEN_DRAIN_DISABLED: u32 = 0;
            #[doc = "Open Drain Enabled"]
            pub const ODE_1_OPEN_DRAIN_ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Pull / Keep Enable Field"]
    pub mod PKE {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pull/Keeper Disabled"]
            pub const PKE_0_PULL_KEEPER_DISABLED: u32 = 0;
            #[doc = "Pull/Keeper Enabled"]
            pub const PKE_1_PULL_KEEPER_ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Pull / Keep Select Field"]
    pub mod PUE {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Keeper"]
            pub const PUE_0_KEEPER: u32 = 0;
            #[doc = "Pull"]
            pub const PUE_1_PULL: u32 = 0x01;
        }
    }
    #[doc = "Pull Up / Down Config. Field"]
    pub mod PUS {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "100K Ohm Pull Down"]
            pub const PUS_0_100K_OHM_PULL_DOWN: u32 = 0;
            #[doc = "47K Ohm Pull Up"]
            pub const PUS_1_47K_OHM_PULL_UP: u32 = 0x01;
            #[doc = "100K Ohm Pull Up"]
            pub const PUS_2_100K_OHM_PULL_UP: u32 = 0x02;
            #[doc = "22K Ohm Pull Up"]
            pub const PUS_3_22K_OHM_PULL_UP: u32 = 0x03;
        }
    }
    #[doc = "Hyst. Enable Field"]
    pub mod HYS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Hysteresis Disabled"]
            pub const HYS_0_HYSTERESIS_DISABLED: u32 = 0;
            #[doc = "Hysteresis Enabled"]
            pub const HYS_1_HYSTERESIS_ENABLED: u32 = 0x01;
        }
    }
}
#[doc = "SW_PAD_CTL_PAD_GPIO_AD_B0_06 SW PAD Control Register"]
pub mod SW_PAD_CTL_PAD_GPIO_AD_B0_06 {
    #[doc = "Slew Rate Field"]
    pub mod SRE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Slow Slew Rate"]
            pub const SRE_0_SLOW_SLEW_RATE: u32 = 0;
            #[doc = "Fast Slew Rate"]
            pub const SRE_1_FAST_SLEW_RATE: u32 = 0x01;
        }
    }
    #[doc = "Drive Strength Field"]
    pub mod DSE {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "output driver disabled;"]
            pub const DSE_0_OUTPUT_DRIVER_DISABLED_: u32 = 0;
            #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V)"]
            pub const DSE_1_R0_150_OHM___3_3V__260_OHM_1_8V: u32 = 0x01;
            #[doc = "R0/2"]
            pub const DSE_2_R0_2: u32 = 0x02;
            #[doc = "R0/3"]
            pub const DSE_3_R0_3: u32 = 0x03;
            #[doc = "R0/4"]
            pub const DSE_4_R0_4: u32 = 0x04;
            #[doc = "R0/5"]
            pub const DSE_5_R0_5: u32 = 0x05;
            #[doc = "R0/6"]
            pub const DSE_6_R0_6: u32 = 0x06;
            #[doc = "R0/7"]
            pub const DSE_7_R0_7: u32 = 0x07;
        }
    }
    #[doc = "Speed Field"]
    pub mod SPEED {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "low(50MHz)"]
            pub const SPEED_0_LOW_50MHZ_: u32 = 0;
            #[doc = "medium(100MHz)"]
            pub const SPEED_1_MEDIUM_100MHZ_: u32 = 0x01;
            #[doc = "medium(100MHz)"]
            pub const SPEED_2_MEDIUM_100MHZ_: u32 = 0x02;
            #[doc = "max(200MHz)"]
            pub const SPEED_3_MAX_200MHZ_: u32 = 0x03;
        }
    }
    #[doc = "Open Drain Enable Field"]
    pub mod ODE {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Open Drain Disabled"]
            pub const ODE_0_OPEN_DRAIN_DISABLED: u32 = 0;
            #[doc = "Open Drain Enabled"]
            pub const ODE_1_OPEN_DRAIN_ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Pull / Keep Enable Field"]
    pub mod PKE {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pull/Keeper Disabled"]
            pub const PKE_0_PULL_KEEPER_DISABLED: u32 = 0;
            #[doc = "Pull/Keeper Enabled"]
            pub const PKE_1_PULL_KEEPER_ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Pull / Keep Select Field"]
    pub mod PUE {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Keeper"]
            pub const PUE_0_KEEPER: u32 = 0;
            #[doc = "Pull"]
            pub const PUE_1_PULL: u32 = 0x01;
        }
    }
    #[doc = "Pull Up / Down Config. Field"]
    pub mod PUS {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "100K Ohm Pull Down"]
            pub const PUS_0_100K_OHM_PULL_DOWN: u32 = 0;
            #[doc = "47K Ohm Pull Up"]
            pub const PUS_1_47K_OHM_PULL_UP: u32 = 0x01;
            #[doc = "100K Ohm Pull Up"]
            pub const PUS_2_100K_OHM_PULL_UP: u32 = 0x02;
            #[doc = "22K Ohm Pull Up"]
            pub const PUS_3_22K_OHM_PULL_UP: u32 = 0x03;
        }
    }
    #[doc = "Hyst. Enable Field"]
    pub mod HYS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Hysteresis Disabled"]
            pub const HYS_0_HYSTERESIS_DISABLED: u32 = 0;
            #[doc = "Hysteresis Enabled"]
            pub const HYS_1_HYSTERESIS_ENABLED: u32 = 0x01;
        }
    }
}
#[doc = "SW_PAD_CTL_PAD_GPIO_AD_B0_07 SW PAD Control Register"]
pub mod SW_PAD_CTL_PAD_GPIO_AD_B0_07 {
    #[doc = "Slew Rate Field"]
    pub mod SRE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Slow Slew Rate"]
            pub const SRE_0_SLOW_SLEW_RATE: u32 = 0;
            #[doc = "Fast Slew Rate"]
            pub const SRE_1_FAST_SLEW_RATE: u32 = 0x01;
        }
    }
    #[doc = "Drive Strength Field"]
    pub mod DSE {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "output driver disabled;"]
            pub const DSE_0_OUTPUT_DRIVER_DISABLED_: u32 = 0;
            #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V)"]
            pub const DSE_1_R0_150_OHM___3_3V__260_OHM_1_8V: u32 = 0x01;
            #[doc = "R0/2"]
            pub const DSE_2_R0_2: u32 = 0x02;
            #[doc = "R0/3"]
            pub const DSE_3_R0_3: u32 = 0x03;
            #[doc = "R0/4"]
            pub const DSE_4_R0_4: u32 = 0x04;
            #[doc = "R0/5"]
            pub const DSE_5_R0_5: u32 = 0x05;
            #[doc = "R0/6"]
            pub const DSE_6_R0_6: u32 = 0x06;
            #[doc = "R0/7"]
            pub const DSE_7_R0_7: u32 = 0x07;
        }
    }
    #[doc = "Speed Field"]
    pub mod SPEED {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "low(50MHz)"]
            pub const SPEED_0_LOW_50MHZ_: u32 = 0;
            #[doc = "medium(100MHz)"]
            pub const SPEED_1_MEDIUM_100MHZ_: u32 = 0x01;
            #[doc = "medium(100MHz)"]
            pub const SPEED_2_MEDIUM_100MHZ_: u32 = 0x02;
            #[doc = "max(200MHz)"]
            pub const SPEED_3_MAX_200MHZ_: u32 = 0x03;
        }
    }
    #[doc = "Open Drain Enable Field"]
    pub mod ODE {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Open Drain Disabled"]
            pub const ODE_0_OPEN_DRAIN_DISABLED: u32 = 0;
            #[doc = "Open Drain Enabled"]
            pub const ODE_1_OPEN_DRAIN_ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Pull / Keep Enable Field"]
    pub mod PKE {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pull/Keeper Disabled"]
            pub const PKE_0_PULL_KEEPER_DISABLED: u32 = 0;
            #[doc = "Pull/Keeper Enabled"]
            pub const PKE_1_PULL_KEEPER_ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Pull / Keep Select Field"]
    pub mod PUE {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Keeper"]
            pub const PUE_0_KEEPER: u32 = 0;
            #[doc = "Pull"]
            pub const PUE_1_PULL: u32 = 0x01;
        }
    }
    #[doc = "Pull Up / Down Config. Field"]
    pub mod PUS {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "100K Ohm Pull Down"]
            pub const PUS_0_100K_OHM_PULL_DOWN: u32 = 0;
            #[doc = "47K Ohm Pull Up"]
            pub const PUS_1_47K_OHM_PULL_UP: u32 = 0x01;
            #[doc = "100K Ohm Pull Up"]
            pub const PUS_2_100K_OHM_PULL_UP: u32 = 0x02;
            #[doc = "22K Ohm Pull Up"]
            pub const PUS_3_22K_OHM_PULL_UP: u32 = 0x03;
        }
    }
    #[doc = "Hyst. Enable Field"]
    pub mod HYS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Hysteresis Disabled"]
            pub const HYS_0_HYSTERESIS_DISABLED: u32 = 0;
            #[doc = "Hysteresis Enabled"]
            pub const HYS_1_HYSTERESIS_ENABLED: u32 = 0x01;
        }
    }
}
#[doc = "SW_PAD_CTL_PAD_GPIO_AD_B0_08 SW PAD Control Register"]
pub mod SW_PAD_CTL_PAD_GPIO_AD_B0_08 {
    #[doc = "Slew Rate Field"]
    pub mod SRE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Slow Slew Rate"]
            pub const SRE_0_SLOW_SLEW_RATE: u32 = 0;
            #[doc = "Fast Slew Rate"]
            pub const SRE_1_FAST_SLEW_RATE: u32 = 0x01;
        }
    }
    #[doc = "Drive Strength Field"]
    pub mod DSE {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "output driver disabled;"]
            pub const DSE_0_OUTPUT_DRIVER_DISABLED_: u32 = 0;
            #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V)"]
            pub const DSE_1_R0_150_OHM___3_3V__260_OHM_1_8V: u32 = 0x01;
            #[doc = "R0/2"]
            pub const DSE_2_R0_2: u32 = 0x02;
            #[doc = "R0/3"]
            pub const DSE_3_R0_3: u32 = 0x03;
            #[doc = "R0/4"]
            pub const DSE_4_R0_4: u32 = 0x04;
            #[doc = "R0/5"]
            pub const DSE_5_R0_5: u32 = 0x05;
            #[doc = "R0/6"]
            pub const DSE_6_R0_6: u32 = 0x06;
            #[doc = "R0/7"]
            pub const DSE_7_R0_7: u32 = 0x07;
        }
    }
    #[doc = "Speed Field"]
    pub mod SPEED {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "low(50MHz)"]
            pub const SPEED_0_LOW_50MHZ_: u32 = 0;
            #[doc = "medium(100MHz)"]
            pub const SPEED_1_MEDIUM_100MHZ_: u32 = 0x01;
            #[doc = "medium(100MHz)"]
            pub const SPEED_2_MEDIUM_100MHZ_: u32 = 0x02;
            #[doc = "max(200MHz)"]
            pub const SPEED_3_MAX_200MHZ_: u32 = 0x03;
        }
    }
    #[doc = "Open Drain Enable Field"]
    pub mod ODE {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Open Drain Disabled"]
            pub const ODE_0_OPEN_DRAIN_DISABLED: u32 = 0;
            #[doc = "Open Drain Enabled"]
            pub const ODE_1_OPEN_DRAIN_ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Pull / Keep Enable Field"]
    pub mod PKE {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pull/Keeper Disabled"]
            pub const PKE_0_PULL_KEEPER_DISABLED: u32 = 0;
            #[doc = "Pull/Keeper Enabled"]
            pub const PKE_1_PULL_KEEPER_ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Pull / Keep Select Field"]
    pub mod PUE {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Keeper"]
            pub const PUE_0_KEEPER: u32 = 0;
            #[doc = "Pull"]
            pub const PUE_1_PULL: u32 = 0x01;
        }
    }
    #[doc = "Pull Up / Down Config. Field"]
    pub mod PUS {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "100K Ohm Pull Down"]
            pub const PUS_0_100K_OHM_PULL_DOWN: u32 = 0;
            #[doc = "47K Ohm Pull Up"]
            pub const PUS_1_47K_OHM_PULL_UP: u32 = 0x01;
            #[doc = "100K Ohm Pull Up"]
            pub const PUS_2_100K_OHM_PULL_UP: u32 = 0x02;
            #[doc = "22K Ohm Pull Up"]
            pub const PUS_3_22K_OHM_PULL_UP: u32 = 0x03;
        }
    }
    #[doc = "Hyst. Enable Field"]
    pub mod HYS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Hysteresis Disabled"]
            pub const HYS_0_HYSTERESIS_DISABLED: u32 = 0;
            #[doc = "Hysteresis Enabled"]
            pub const HYS_1_HYSTERESIS_ENABLED: u32 = 0x01;
        }
    }
}
#[doc = "SW_PAD_CTL_PAD_GPIO_AD_B0_09 SW PAD Control Register"]
pub mod SW_PAD_CTL_PAD_GPIO_AD_B0_09 {
    #[doc = "Slew Rate Field"]
    pub mod SRE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Slow Slew Rate"]
            pub const SRE_0_SLOW_SLEW_RATE: u32 = 0;
            #[doc = "Fast Slew Rate"]
            pub const SRE_1_FAST_SLEW_RATE: u32 = 0x01;
        }
    }
    #[doc = "Drive Strength Field"]
    pub mod DSE {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "output driver disabled;"]
            pub const DSE_0_OUTPUT_DRIVER_DISABLED_: u32 = 0;
            #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V)"]
            pub const DSE_1_R0_150_OHM___3_3V__260_OHM_1_8V: u32 = 0x01;
            #[doc = "R0/2"]
            pub const DSE_2_R0_2: u32 = 0x02;
            #[doc = "R0/3"]
            pub const DSE_3_R0_3: u32 = 0x03;
            #[doc = "R0/4"]
            pub const DSE_4_R0_4: u32 = 0x04;
            #[doc = "R0/5"]
            pub const DSE_5_R0_5: u32 = 0x05;
            #[doc = "R0/6"]
            pub const DSE_6_R0_6: u32 = 0x06;
            #[doc = "R0/7"]
            pub const DSE_7_R0_7: u32 = 0x07;
        }
    }
    #[doc = "Speed Field"]
    pub mod SPEED {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "low(50MHz)"]
            pub const SPEED_0_LOW_50MHZ_: u32 = 0;
            #[doc = "medium(100MHz)"]
            pub const SPEED_1_MEDIUM_100MHZ_: u32 = 0x01;
            #[doc = "medium(100MHz)"]
            pub const SPEED_2_MEDIUM_100MHZ_: u32 = 0x02;
            #[doc = "max(200MHz)"]
            pub const SPEED_3_MAX_200MHZ_: u32 = 0x03;
        }
    }
    #[doc = "Open Drain Enable Field"]
    pub mod ODE {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Open Drain Disabled"]
            pub const ODE_0_OPEN_DRAIN_DISABLED: u32 = 0;
            #[doc = "Open Drain Enabled"]
            pub const ODE_1_OPEN_DRAIN_ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Pull / Keep Enable Field"]
    pub mod PKE {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pull/Keeper Disabled"]
            pub const PKE_0_PULL_KEEPER_DISABLED: u32 = 0;
            #[doc = "Pull/Keeper Enabled"]
            pub const PKE_1_PULL_KEEPER_ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Pull / Keep Select Field"]
    pub mod PUE {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Keeper"]
            pub const PUE_0_KEEPER: u32 = 0;
            #[doc = "Pull"]
            pub const PUE_1_PULL: u32 = 0x01;
        }
    }
    #[doc = "Pull Up / Down Config. Field"]
    pub mod PUS {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "100K Ohm Pull Down"]
            pub const PUS_0_100K_OHM_PULL_DOWN: u32 = 0;
            #[doc = "47K Ohm Pull Up"]
            pub const PUS_1_47K_OHM_PULL_UP: u32 = 0x01;
            #[doc = "100K Ohm Pull Up"]
            pub const PUS_2_100K_OHM_PULL_UP: u32 = 0x02;
            #[doc = "22K Ohm Pull Up"]
            pub const PUS_3_22K_OHM_PULL_UP: u32 = 0x03;
        }
    }
    #[doc = "Hyst. Enable Field"]
    pub mod HYS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Hysteresis Disabled"]
            pub const HYS_0_HYSTERESIS_DISABLED: u32 = 0;
            #[doc = "Hysteresis Enabled"]
            pub const HYS_1_HYSTERESIS_ENABLED: u32 = 0x01;
        }
    }
}
#[doc = "SW_PAD_CTL_PAD_GPIO_AD_B0_10 SW PAD Control Register"]
pub mod SW_PAD_CTL_PAD_GPIO_AD_B0_10 {
    #[doc = "Slew Rate Field"]
    pub mod SRE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Slow Slew Rate"]
            pub const SRE_0_SLOW_SLEW_RATE: u32 = 0;
            #[doc = "Fast Slew Rate"]
            pub const SRE_1_FAST_SLEW_RATE: u32 = 0x01;
        }
    }
    #[doc = "Drive Strength Field"]
    pub mod DSE {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "output driver disabled;"]
            pub const DSE_0_OUTPUT_DRIVER_DISABLED_: u32 = 0;
            #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V)"]
            pub const DSE_1_R0_150_OHM___3_3V__260_OHM_1_8V: u32 = 0x01;
            #[doc = "R0/2"]
            pub const DSE_2_R0_2: u32 = 0x02;
            #[doc = "R0/3"]
            pub const DSE_3_R0_3: u32 = 0x03;
            #[doc = "R0/4"]
            pub const DSE_4_R0_4: u32 = 0x04;
            #[doc = "R0/5"]
            pub const DSE_5_R0_5: u32 = 0x05;
            #[doc = "R0/6"]
            pub const DSE_6_R0_6: u32 = 0x06;
            #[doc = "R0/7"]
            pub const DSE_7_R0_7: u32 = 0x07;
        }
    }
    #[doc = "Speed Field"]
    pub mod SPEED {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "low(50MHz)"]
            pub const SPEED_0_LOW_50MHZ_: u32 = 0;
            #[doc = "medium(100MHz)"]
            pub const SPEED_1_MEDIUM_100MHZ_: u32 = 0x01;
            #[doc = "medium(100MHz)"]
            pub const SPEED_2_MEDIUM_100MHZ_: u32 = 0x02;
            #[doc = "max(200MHz)"]
            pub const SPEED_3_MAX_200MHZ_: u32 = 0x03;
        }
    }
    #[doc = "Open Drain Enable Field"]
    pub mod ODE {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Open Drain Disabled"]
            pub const ODE_0_OPEN_DRAIN_DISABLED: u32 = 0;
            #[doc = "Open Drain Enabled"]
            pub const ODE_1_OPEN_DRAIN_ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Pull / Keep Enable Field"]
    pub mod PKE {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pull/Keeper Disabled"]
            pub const PKE_0_PULL_KEEPER_DISABLED: u32 = 0;
            #[doc = "Pull/Keeper Enabled"]
            pub const PKE_1_PULL_KEEPER_ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Pull / Keep Select Field"]
    pub mod PUE {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Keeper"]
            pub const PUE_0_KEEPER: u32 = 0;
            #[doc = "Pull"]
            pub const PUE_1_PULL: u32 = 0x01;
        }
    }
    #[doc = "Pull Up / Down Config. Field"]
    pub mod PUS {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "100K Ohm Pull Down"]
            pub const PUS_0_100K_OHM_PULL_DOWN: u32 = 0;
            #[doc = "47K Ohm Pull Up"]
            pub const PUS_1_47K_OHM_PULL_UP: u32 = 0x01;
            #[doc = "100K Ohm Pull Up"]
            pub const PUS_2_100K_OHM_PULL_UP: u32 = 0x02;
            #[doc = "22K Ohm Pull Up"]
            pub const PUS_3_22K_OHM_PULL_UP: u32 = 0x03;
        }
    }
    #[doc = "Hyst. Enable Field"]
    pub mod HYS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Hysteresis Disabled"]
            pub const HYS_0_HYSTERESIS_DISABLED: u32 = 0;
            #[doc = "Hysteresis Enabled"]
            pub const HYS_1_HYSTERESIS_ENABLED: u32 = 0x01;
        }
    }
}
#[doc = "SW_PAD_CTL_PAD_GPIO_AD_B0_11 SW PAD Control Register"]
pub mod SW_PAD_CTL_PAD_GPIO_AD_B0_11 {
    #[doc = "Slew Rate Field"]
    pub mod SRE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Slow Slew Rate"]
            pub const SRE_0_SLOW_SLEW_RATE: u32 = 0;
            #[doc = "Fast Slew Rate"]
            pub const SRE_1_FAST_SLEW_RATE: u32 = 0x01;
        }
    }
    #[doc = "Drive Strength Field"]
    pub mod DSE {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "output driver disabled;"]
            pub const DSE_0_OUTPUT_DRIVER_DISABLED_: u32 = 0;
            #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V)"]
            pub const DSE_1_R0_150_OHM___3_3V__260_OHM_1_8V: u32 = 0x01;
            #[doc = "R0/2"]
            pub const DSE_2_R0_2: u32 = 0x02;
            #[doc = "R0/3"]
            pub const DSE_3_R0_3: u32 = 0x03;
            #[doc = "R0/4"]
            pub const DSE_4_R0_4: u32 = 0x04;
            #[doc = "R0/5"]
            pub const DSE_5_R0_5: u32 = 0x05;
            #[doc = "R0/6"]
            pub const DSE_6_R0_6: u32 = 0x06;
            #[doc = "R0/7"]
            pub const DSE_7_R0_7: u32 = 0x07;
        }
    }
    #[doc = "Speed Field"]
    pub mod SPEED {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "low(50MHz)"]
            pub const SPEED_0_LOW_50MHZ_: u32 = 0;
            #[doc = "medium(100MHz)"]
            pub const SPEED_1_MEDIUM_100MHZ_: u32 = 0x01;
            #[doc = "medium(100MHz)"]
            pub const SPEED_2_MEDIUM_100MHZ_: u32 = 0x02;
            #[doc = "max(200MHz)"]
            pub const SPEED_3_MAX_200MHZ_: u32 = 0x03;
        }
    }
    #[doc = "Open Drain Enable Field"]
    pub mod ODE {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Open Drain Disabled"]
            pub const ODE_0_OPEN_DRAIN_DISABLED: u32 = 0;
            #[doc = "Open Drain Enabled"]
            pub const ODE_1_OPEN_DRAIN_ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Pull / Keep Enable Field"]
    pub mod PKE {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pull/Keeper Disabled"]
            pub const PKE_0_PULL_KEEPER_DISABLED: u32 = 0;
            #[doc = "Pull/Keeper Enabled"]
            pub const PKE_1_PULL_KEEPER_ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Pull / Keep Select Field"]
    pub mod PUE {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Keeper"]
            pub const PUE_0_KEEPER: u32 = 0;
            #[doc = "Pull"]
            pub const PUE_1_PULL: u32 = 0x01;
        }
    }
    #[doc = "Pull Up / Down Config. Field"]
    pub mod PUS {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "100K Ohm Pull Down"]
            pub const PUS_0_100K_OHM_PULL_DOWN: u32 = 0;
            #[doc = "47K Ohm Pull Up"]
            pub const PUS_1_47K_OHM_PULL_UP: u32 = 0x01;
            #[doc = "100K Ohm Pull Up"]
            pub const PUS_2_100K_OHM_PULL_UP: u32 = 0x02;
            #[doc = "22K Ohm Pull Up"]
            pub const PUS_3_22K_OHM_PULL_UP: u32 = 0x03;
        }
    }
    #[doc = "Hyst. Enable Field"]
    pub mod HYS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Hysteresis Disabled"]
            pub const HYS_0_HYSTERESIS_DISABLED: u32 = 0;
            #[doc = "Hysteresis Enabled"]
            pub const HYS_1_HYSTERESIS_ENABLED: u32 = 0x01;
        }
    }
}
#[doc = "SW_PAD_CTL_PAD_GPIO_AD_B0_12 SW PAD Control Register"]
pub mod SW_PAD_CTL_PAD_GPIO_AD_B0_12 {
    #[doc = "Slew Rate Field"]
    pub mod SRE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Slow Slew Rate"]
            pub const SRE_0_SLOW_SLEW_RATE: u32 = 0;
            #[doc = "Fast Slew Rate"]
            pub const SRE_1_FAST_SLEW_RATE: u32 = 0x01;
        }
    }
    #[doc = "Drive Strength Field"]
    pub mod DSE {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "output driver disabled;"]
            pub const DSE_0_OUTPUT_DRIVER_DISABLED_: u32 = 0;
            #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V)"]
            pub const DSE_1_R0_150_OHM___3_3V__260_OHM_1_8V: u32 = 0x01;
            #[doc = "R0/2"]
            pub const DSE_2_R0_2: u32 = 0x02;
            #[doc = "R0/3"]
            pub const DSE_3_R0_3: u32 = 0x03;
            #[doc = "R0/4"]
            pub const DSE_4_R0_4: u32 = 0x04;
            #[doc = "R0/5"]
            pub const DSE_5_R0_5: u32 = 0x05;
            #[doc = "R0/6"]
            pub const DSE_6_R0_6: u32 = 0x06;
            #[doc = "R0/7"]
            pub const DSE_7_R0_7: u32 = 0x07;
        }
    }
    #[doc = "Speed Field"]
    pub mod SPEED {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "low(50MHz)"]
            pub const SPEED_0_LOW_50MHZ_: u32 = 0;
            #[doc = "medium(100MHz)"]
            pub const SPEED_1_MEDIUM_100MHZ_: u32 = 0x01;
            #[doc = "medium(100MHz)"]
            pub const SPEED_2_MEDIUM_100MHZ_: u32 = 0x02;
            #[doc = "max(200MHz)"]
            pub const SPEED_3_MAX_200MHZ_: u32 = 0x03;
        }
    }
    #[doc = "Open Drain Enable Field"]
    pub mod ODE {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Open Drain Disabled"]
            pub const ODE_0_OPEN_DRAIN_DISABLED: u32 = 0;
            #[doc = "Open Drain Enabled"]
            pub const ODE_1_OPEN_DRAIN_ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Pull / Keep Enable Field"]
    pub mod PKE {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pull/Keeper Disabled"]
            pub const PKE_0_PULL_KEEPER_DISABLED: u32 = 0;
            #[doc = "Pull/Keeper Enabled"]
            pub const PKE_1_PULL_KEEPER_ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Pull / Keep Select Field"]
    pub mod PUE {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Keeper"]
            pub const PUE_0_KEEPER: u32 = 0;
            #[doc = "Pull"]
            pub const PUE_1_PULL: u32 = 0x01;
        }
    }
    #[doc = "Pull Up / Down Config. Field"]
    pub mod PUS {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "100K Ohm Pull Down"]
            pub const PUS_0_100K_OHM_PULL_DOWN: u32 = 0;
            #[doc = "47K Ohm Pull Up"]
            pub const PUS_1_47K_OHM_PULL_UP: u32 = 0x01;
            #[doc = "100K Ohm Pull Up"]
            pub const PUS_2_100K_OHM_PULL_UP: u32 = 0x02;
            #[doc = "22K Ohm Pull Up"]
            pub const PUS_3_22K_OHM_PULL_UP: u32 = 0x03;
        }
    }
    #[doc = "Hyst. Enable Field"]
    pub mod HYS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Hysteresis Disabled"]
            pub const HYS_0_HYSTERESIS_DISABLED: u32 = 0;
            #[doc = "Hysteresis Enabled"]
            pub const HYS_1_HYSTERESIS_ENABLED: u32 = 0x01;
        }
    }
}
#[doc = "SW_PAD_CTL_PAD_GPIO_AD_B0_13 SW PAD Control Register"]
pub mod SW_PAD_CTL_PAD_GPIO_AD_B0_13 {
    #[doc = "Slew Rate Field"]
    pub mod SRE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Slow Slew Rate"]
            pub const SRE_0_SLOW_SLEW_RATE: u32 = 0;
            #[doc = "Fast Slew Rate"]
            pub const SRE_1_FAST_SLEW_RATE: u32 = 0x01;
        }
    }
    #[doc = "Drive Strength Field"]
    pub mod DSE {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "output driver disabled;"]
            pub const DSE_0_OUTPUT_DRIVER_DISABLED_: u32 = 0;
            #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V)"]
            pub const DSE_1_R0_150_OHM___3_3V__260_OHM_1_8V: u32 = 0x01;
            #[doc = "R0/2"]
            pub const DSE_2_R0_2: u32 = 0x02;
            #[doc = "R0/3"]
            pub const DSE_3_R0_3: u32 = 0x03;
            #[doc = "R0/4"]
            pub const DSE_4_R0_4: u32 = 0x04;
            #[doc = "R0/5"]
            pub const DSE_5_R0_5: u32 = 0x05;
            #[doc = "R0/6"]
            pub const DSE_6_R0_6: u32 = 0x06;
            #[doc = "R0/7"]
            pub const DSE_7_R0_7: u32 = 0x07;
        }
    }
    #[doc = "Speed Field"]
    pub mod SPEED {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "low(50MHz)"]
            pub const SPEED_0_LOW_50MHZ_: u32 = 0;
            #[doc = "medium(100MHz)"]
            pub const SPEED_1_MEDIUM_100MHZ_: u32 = 0x01;
            #[doc = "medium(100MHz)"]
            pub const SPEED_2_MEDIUM_100MHZ_: u32 = 0x02;
            #[doc = "max(200MHz)"]
            pub const SPEED_3_MAX_200MHZ_: u32 = 0x03;
        }
    }
    #[doc = "Open Drain Enable Field"]
    pub mod ODE {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Open Drain Disabled"]
            pub const ODE_0_OPEN_DRAIN_DISABLED: u32 = 0;
            #[doc = "Open Drain Enabled"]
            pub const ODE_1_OPEN_DRAIN_ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Pull / Keep Enable Field"]
    pub mod PKE {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pull/Keeper Disabled"]
            pub const PKE_0_PULL_KEEPER_DISABLED: u32 = 0;
            #[doc = "Pull/Keeper Enabled"]
            pub const PKE_1_PULL_KEEPER_ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Pull / Keep Select Field"]
    pub mod PUE {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Keeper"]
            pub const PUE_0_KEEPER: u32 = 0;
            #[doc = "Pull"]
            pub const PUE_1_PULL: u32 = 0x01;
        }
    }
    #[doc = "Pull Up / Down Config. Field"]
    pub mod PUS {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "100K Ohm Pull Down"]
            pub const PUS_0_100K_OHM_PULL_DOWN: u32 = 0;
            #[doc = "47K Ohm Pull Up"]
            pub const PUS_1_47K_OHM_PULL_UP: u32 = 0x01;
            #[doc = "100K Ohm Pull Up"]
            pub const PUS_2_100K_OHM_PULL_UP: u32 = 0x02;
            #[doc = "22K Ohm Pull Up"]
            pub const PUS_3_22K_OHM_PULL_UP: u32 = 0x03;
        }
    }
    #[doc = "Hyst. Enable Field"]
    pub mod HYS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Hysteresis Disabled"]
            pub const HYS_0_HYSTERESIS_DISABLED: u32 = 0;
            #[doc = "Hysteresis Enabled"]
            pub const HYS_1_HYSTERESIS_ENABLED: u32 = 0x01;
        }
    }
}
#[doc = "SW_PAD_CTL_PAD_GPIO_AD_B0_14 SW PAD Control Register"]
pub mod SW_PAD_CTL_PAD_GPIO_AD_B0_14 {
    #[doc = "Slew Rate Field"]
    pub mod SRE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Slow Slew Rate"]
            pub const SRE_0_SLOW_SLEW_RATE: u32 = 0;
            #[doc = "Fast Slew Rate"]
            pub const SRE_1_FAST_SLEW_RATE: u32 = 0x01;
        }
    }
    #[doc = "Drive Strength Field"]
    pub mod DSE {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "output driver disabled;"]
            pub const DSE_0_OUTPUT_DRIVER_DISABLED_: u32 = 0;
            #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V)"]
            pub const DSE_1_R0_150_OHM___3_3V__260_OHM_1_8V: u32 = 0x01;
            #[doc = "R0/2"]
            pub const DSE_2_R0_2: u32 = 0x02;
            #[doc = "R0/3"]
            pub const DSE_3_R0_3: u32 = 0x03;
            #[doc = "R0/4"]
            pub const DSE_4_R0_4: u32 = 0x04;
            #[doc = "R0/5"]
            pub const DSE_5_R0_5: u32 = 0x05;
            #[doc = "R0/6"]
            pub const DSE_6_R0_6: u32 = 0x06;
            #[doc = "R0/7"]
            pub const DSE_7_R0_7: u32 = 0x07;
        }
    }
    #[doc = "Speed Field"]
    pub mod SPEED {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "low(50MHz)"]
            pub const SPEED_0_LOW_50MHZ_: u32 = 0;
            #[doc = "medium(100MHz)"]
            pub const SPEED_1_MEDIUM_100MHZ_: u32 = 0x01;
            #[doc = "medium(100MHz)"]
            pub const SPEED_2_MEDIUM_100MHZ_: u32 = 0x02;
            #[doc = "max(200MHz)"]
            pub const SPEED_3_MAX_200MHZ_: u32 = 0x03;
        }
    }
    #[doc = "Open Drain Enable Field"]
    pub mod ODE {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Open Drain Disabled"]
            pub const ODE_0_OPEN_DRAIN_DISABLED: u32 = 0;
            #[doc = "Open Drain Enabled"]
            pub const ODE_1_OPEN_DRAIN_ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Pull / Keep Enable Field"]
    pub mod PKE {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pull/Keeper Disabled"]
            pub const PKE_0_PULL_KEEPER_DISABLED: u32 = 0;
            #[doc = "Pull/Keeper Enabled"]
            pub const PKE_1_PULL_KEEPER_ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Pull / Keep Select Field"]
    pub mod PUE {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Keeper"]
            pub const PUE_0_KEEPER: u32 = 0;
            #[doc = "Pull"]
            pub const PUE_1_PULL: u32 = 0x01;
        }
    }
    #[doc = "Pull Up / Down Config. Field"]
    pub mod PUS {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "100K Ohm Pull Down"]
            pub const PUS_0_100K_OHM_PULL_DOWN: u32 = 0;
            #[doc = "47K Ohm Pull Up"]
            pub const PUS_1_47K_OHM_PULL_UP: u32 = 0x01;
            #[doc = "100K Ohm Pull Up"]
            pub const PUS_2_100K_OHM_PULL_UP: u32 = 0x02;
            #[doc = "22K Ohm Pull Up"]
            pub const PUS_3_22K_OHM_PULL_UP: u32 = 0x03;
        }
    }
    #[doc = "Hyst. Enable Field"]
    pub mod HYS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Hysteresis Disabled"]
            pub const HYS_0_HYSTERESIS_DISABLED: u32 = 0;
            #[doc = "Hysteresis Enabled"]
            pub const HYS_1_HYSTERESIS_ENABLED: u32 = 0x01;
        }
    }
}
#[doc = "SW_PAD_CTL_PAD_GPIO_AD_B0_15 SW PAD Control Register"]
pub mod SW_PAD_CTL_PAD_GPIO_AD_B0_15 {
    #[doc = "Slew Rate Field"]
    pub mod SRE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Slow Slew Rate"]
            pub const SRE_0_SLOW_SLEW_RATE: u32 = 0;
            #[doc = "Fast Slew Rate"]
            pub const SRE_1_FAST_SLEW_RATE: u32 = 0x01;
        }
    }
    #[doc = "Drive Strength Field"]
    pub mod DSE {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "output driver disabled;"]
            pub const DSE_0_OUTPUT_DRIVER_DISABLED_: u32 = 0;
            #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V)"]
            pub const DSE_1_R0_150_OHM___3_3V__260_OHM_1_8V: u32 = 0x01;
            #[doc = "R0/2"]
            pub const DSE_2_R0_2: u32 = 0x02;
            #[doc = "R0/3"]
            pub const DSE_3_R0_3: u32 = 0x03;
            #[doc = "R0/4"]
            pub const DSE_4_R0_4: u32 = 0x04;
            #[doc = "R0/5"]
            pub const DSE_5_R0_5: u32 = 0x05;
            #[doc = "R0/6"]
            pub const DSE_6_R0_6: u32 = 0x06;
            #[doc = "R0/7"]
            pub const DSE_7_R0_7: u32 = 0x07;
        }
    }
    #[doc = "Speed Field"]
    pub mod SPEED {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "low(50MHz)"]
            pub const SPEED_0_LOW_50MHZ_: u32 = 0;
            #[doc = "medium(100MHz)"]
            pub const SPEED_1_MEDIUM_100MHZ_: u32 = 0x01;
            #[doc = "medium(100MHz)"]
            pub const SPEED_2_MEDIUM_100MHZ_: u32 = 0x02;
            #[doc = "max(200MHz)"]
            pub const SPEED_3_MAX_200MHZ_: u32 = 0x03;
        }
    }
    #[doc = "Open Drain Enable Field"]
    pub mod ODE {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Open Drain Disabled"]
            pub const ODE_0_OPEN_DRAIN_DISABLED: u32 = 0;
            #[doc = "Open Drain Enabled"]
            pub const ODE_1_OPEN_DRAIN_ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Pull / Keep Enable Field"]
    pub mod PKE {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pull/Keeper Disabled"]
            pub const PKE_0_PULL_KEEPER_DISABLED: u32 = 0;
            #[doc = "Pull/Keeper Enabled"]
            pub const PKE_1_PULL_KEEPER_ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Pull / Keep Select Field"]
    pub mod PUE {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Keeper"]
            pub const PUE_0_KEEPER: u32 = 0;
            #[doc = "Pull"]
            pub const PUE_1_PULL: u32 = 0x01;
        }
    }
    #[doc = "Pull Up / Down Config. Field"]
    pub mod PUS {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "100K Ohm Pull Down"]
            pub const PUS_0_100K_OHM_PULL_DOWN: u32 = 0;
            #[doc = "47K Ohm Pull Up"]
            pub const PUS_1_47K_OHM_PULL_UP: u32 = 0x01;
            #[doc = "100K Ohm Pull Up"]
            pub const PUS_2_100K_OHM_PULL_UP: u32 = 0x02;
            #[doc = "22K Ohm Pull Up"]
            pub const PUS_3_22K_OHM_PULL_UP: u32 = 0x03;
        }
    }
    #[doc = "Hyst. Enable Field"]
    pub mod HYS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Hysteresis Disabled"]
            pub const HYS_0_HYSTERESIS_DISABLED: u32 = 0;
            #[doc = "Hysteresis Enabled"]
            pub const HYS_1_HYSTERESIS_ENABLED: u32 = 0x01;
        }
    }
}
#[doc = "SW_PAD_CTL_PAD_GPIO_AD_B1_10 SW PAD Control Register"]
pub mod SW_PAD_CTL_PAD_GPIO_AD_B1_10 {
    #[doc = "Slew Rate Field"]
    pub mod SRE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Slow Slew Rate"]
            pub const SRE_0_SLOW_SLEW_RATE: u32 = 0;
            #[doc = "Fast Slew Rate"]
            pub const SRE_1_FAST_SLEW_RATE: u32 = 0x01;
        }
    }
    #[doc = "Drive Strength Field"]
    pub mod DSE {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "output driver disabled;"]
            pub const DSE_0_OUTPUT_DRIVER_DISABLED_: u32 = 0;
            #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V)"]
            pub const DSE_1_R0_150_OHM___3_3V__260_OHM_1_8V: u32 = 0x01;
            #[doc = "R0/2"]
            pub const DSE_2_R0_2: u32 = 0x02;
            #[doc = "R0/3"]
            pub const DSE_3_R0_3: u32 = 0x03;
            #[doc = "R0/4"]
            pub const DSE_4_R0_4: u32 = 0x04;
            #[doc = "R0/5"]
            pub const DSE_5_R0_5: u32 = 0x05;
            #[doc = "R0/6"]
            pub const DSE_6_R0_6: u32 = 0x06;
            #[doc = "R0/7"]
            pub const DSE_7_R0_7: u32 = 0x07;
        }
    }
    #[doc = "Speed Field"]
    pub mod SPEED {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "low(50MHz)"]
            pub const SPEED_0_LOW_50MHZ_: u32 = 0;
            #[doc = "medium(100MHz)"]
            pub const SPEED_1_MEDIUM_100MHZ_: u32 = 0x01;
            #[doc = "medium(100MHz)"]
            pub const SPEED_2_MEDIUM_100MHZ_: u32 = 0x02;
            #[doc = "max(200MHz)"]
            pub const SPEED_3_MAX_200MHZ_: u32 = 0x03;
        }
    }
    #[doc = "Open Drain Enable Field"]
    pub mod ODE {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Open Drain Disabled"]
            pub const ODE_0_OPEN_DRAIN_DISABLED: u32 = 0;
            #[doc = "Open Drain Enabled"]
            pub const ODE_1_OPEN_DRAIN_ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Pull / Keep Enable Field"]
    pub mod PKE {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pull/Keeper Disabled"]
            pub const PKE_0_PULL_KEEPER_DISABLED: u32 = 0;
            #[doc = "Pull/Keeper Enabled"]
            pub const PKE_1_PULL_KEEPER_ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Pull / Keep Select Field"]
    pub mod PUE {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Keeper"]
            pub const PUE_0_KEEPER: u32 = 0;
            #[doc = "Pull"]
            pub const PUE_1_PULL: u32 = 0x01;
        }
    }
    #[doc = "Pull Up / Down Config. Field"]
    pub mod PUS {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "100K Ohm Pull Down"]
            pub const PUS_0_100K_OHM_PULL_DOWN: u32 = 0;
            #[doc = "47K Ohm Pull Up"]
            pub const PUS_1_47K_OHM_PULL_UP: u32 = 0x01;
            #[doc = "100K Ohm Pull Up"]
            pub const PUS_2_100K_OHM_PULL_UP: u32 = 0x02;
            #[doc = "22K Ohm Pull Up"]
            pub const PUS_3_22K_OHM_PULL_UP: u32 = 0x03;
        }
    }
    #[doc = "Hyst. Enable Field"]
    pub mod HYS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Hysteresis Disabled"]
            pub const HYS_0_HYSTERESIS_DISABLED: u32 = 0;
            #[doc = "Hysteresis Enabled"]
            pub const HYS_1_HYSTERESIS_ENABLED: u32 = 0x01;
        }
    }
}
#[doc = "SW_PAD_CTL_PAD_GPIO_AD_B1_11 SW PAD Control Register"]
pub mod SW_PAD_CTL_PAD_GPIO_AD_B1_11 {
    #[doc = "Slew Rate Field"]
    pub mod SRE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Slow Slew Rate"]
            pub const SRE_0_SLOW_SLEW_RATE: u32 = 0;
            #[doc = "Fast Slew Rate"]
            pub const SRE_1_FAST_SLEW_RATE: u32 = 0x01;
        }
    }
    #[doc = "Drive Strength Field"]
    pub mod DSE {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "output driver disabled;"]
            pub const DSE_0_OUTPUT_DRIVER_DISABLED_: u32 = 0;
            #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V)"]
            pub const DSE_1_R0_150_OHM___3_3V__260_OHM_1_8V: u32 = 0x01;
            #[doc = "R0/2"]
            pub const DSE_2_R0_2: u32 = 0x02;
            #[doc = "R0/3"]
            pub const DSE_3_R0_3: u32 = 0x03;
            #[doc = "R0/4"]
            pub const DSE_4_R0_4: u32 = 0x04;
            #[doc = "R0/5"]
            pub const DSE_5_R0_5: u32 = 0x05;
            #[doc = "R0/6"]
            pub const DSE_6_R0_6: u32 = 0x06;
            #[doc = "R0/7"]
            pub const DSE_7_R0_7: u32 = 0x07;
        }
    }
    #[doc = "Speed Field"]
    pub mod SPEED {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "low(50MHz)"]
            pub const SPEED_0_LOW_50MHZ_: u32 = 0;
            #[doc = "medium(100MHz)"]
            pub const SPEED_1_MEDIUM_100MHZ_: u32 = 0x01;
            #[doc = "medium(100MHz)"]
            pub const SPEED_2_MEDIUM_100MHZ_: u32 = 0x02;
            #[doc = "max(200MHz)"]
            pub const SPEED_3_MAX_200MHZ_: u32 = 0x03;
        }
    }
    #[doc = "Open Drain Enable Field"]
    pub mod ODE {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Open Drain Disabled"]
            pub const ODE_0_OPEN_DRAIN_DISABLED: u32 = 0;
            #[doc = "Open Drain Enabled"]
            pub const ODE_1_OPEN_DRAIN_ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Pull / Keep Enable Field"]
    pub mod PKE {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pull/Keeper Disabled"]
            pub const PKE_0_PULL_KEEPER_DISABLED: u32 = 0;
            #[doc = "Pull/Keeper Enabled"]
            pub const PKE_1_PULL_KEEPER_ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Pull / Keep Select Field"]
    pub mod PUE {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Keeper"]
            pub const PUE_0_KEEPER: u32 = 0;
            #[doc = "Pull"]
            pub const PUE_1_PULL: u32 = 0x01;
        }
    }
    #[doc = "Pull Up / Down Config. Field"]
    pub mod PUS {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "100K Ohm Pull Down"]
            pub const PUS_0_100K_OHM_PULL_DOWN: u32 = 0;
            #[doc = "47K Ohm Pull Up"]
            pub const PUS_1_47K_OHM_PULL_UP: u32 = 0x01;
            #[doc = "100K Ohm Pull Up"]
            pub const PUS_2_100K_OHM_PULL_UP: u32 = 0x02;
            #[doc = "22K Ohm Pull Up"]
            pub const PUS_3_22K_OHM_PULL_UP: u32 = 0x03;
        }
    }
    #[doc = "Hyst. Enable Field"]
    pub mod HYS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Hysteresis Disabled"]
            pub const HYS_0_HYSTERESIS_DISABLED: u32 = 0;
            #[doc = "Hysteresis Enabled"]
            pub const HYS_1_HYSTERESIS_ENABLED: u32 = 0x01;
        }
    }
}
#[doc = "SW_PAD_CTL_PAD_GPIO_AD_B1_12 SW PAD Control Register"]
pub mod SW_PAD_CTL_PAD_GPIO_AD_B1_12 {
    #[doc = "Slew Rate Field"]
    pub mod SRE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Slow Slew Rate"]
            pub const SRE_0_SLOW_SLEW_RATE: u32 = 0;
            #[doc = "Fast Slew Rate"]
            pub const SRE_1_FAST_SLEW_RATE: u32 = 0x01;
        }
    }
    #[doc = "Drive Strength Field"]
    pub mod DSE {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "output driver disabled;"]
            pub const DSE_0_OUTPUT_DRIVER_DISABLED_: u32 = 0;
            #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V)"]
            pub const DSE_1_R0_150_OHM___3_3V__260_OHM_1_8V: u32 = 0x01;
            #[doc = "R0/2"]
            pub const DSE_2_R0_2: u32 = 0x02;
            #[doc = "R0/3"]
            pub const DSE_3_R0_3: u32 = 0x03;
            #[doc = "R0/4"]
            pub const DSE_4_R0_4: u32 = 0x04;
            #[doc = "R0/5"]
            pub const DSE_5_R0_5: u32 = 0x05;
            #[doc = "R0/6"]
            pub const DSE_6_R0_6: u32 = 0x06;
            #[doc = "R0/7"]
            pub const DSE_7_R0_7: u32 = 0x07;
        }
    }
    #[doc = "Speed Field"]
    pub mod SPEED {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "low(50MHz)"]
            pub const SPEED_0_LOW_50MHZ_: u32 = 0;
            #[doc = "medium(100MHz)"]
            pub const SPEED_1_MEDIUM_100MHZ_: u32 = 0x01;
            #[doc = "medium(100MHz)"]
            pub const SPEED_2_MEDIUM_100MHZ_: u32 = 0x02;
            #[doc = "max(200MHz)"]
            pub const SPEED_3_MAX_200MHZ_: u32 = 0x03;
        }
    }
    #[doc = "Open Drain Enable Field"]
    pub mod ODE {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Open Drain Disabled"]
            pub const ODE_0_OPEN_DRAIN_DISABLED: u32 = 0;
            #[doc = "Open Drain Enabled"]
            pub const ODE_1_OPEN_DRAIN_ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Pull / Keep Enable Field"]
    pub mod PKE {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pull/Keeper Disabled"]
            pub const PKE_0_PULL_KEEPER_DISABLED: u32 = 0;
            #[doc = "Pull/Keeper Enabled"]
            pub const PKE_1_PULL_KEEPER_ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Pull / Keep Select Field"]
    pub mod PUE {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Keeper"]
            pub const PUE_0_KEEPER: u32 = 0;
            #[doc = "Pull"]
            pub const PUE_1_PULL: u32 = 0x01;
        }
    }
    #[doc = "Pull Up / Down Config. Field"]
    pub mod PUS {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "100K Ohm Pull Down"]
            pub const PUS_0_100K_OHM_PULL_DOWN: u32 = 0;
            #[doc = "47K Ohm Pull Up"]
            pub const PUS_1_47K_OHM_PULL_UP: u32 = 0x01;
            #[doc = "100K Ohm Pull Up"]
            pub const PUS_2_100K_OHM_PULL_UP: u32 = 0x02;
            #[doc = "22K Ohm Pull Up"]
            pub const PUS_3_22K_OHM_PULL_UP: u32 = 0x03;
        }
    }
    #[doc = "Hyst. Enable Field"]
    pub mod HYS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Hysteresis Disabled"]
            pub const HYS_0_HYSTERESIS_DISABLED: u32 = 0;
            #[doc = "Hysteresis Enabled"]
            pub const HYS_1_HYSTERESIS_ENABLED: u32 = 0x01;
        }
    }
}
#[doc = "SW_PAD_CTL_PAD_GPIO_AD_B1_13 SW PAD Control Register"]
pub mod SW_PAD_CTL_PAD_GPIO_AD_B1_13 {
    #[doc = "Slew Rate Field"]
    pub mod SRE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Slow Slew Rate"]
            pub const SRE_0_SLOW_SLEW_RATE: u32 = 0;
            #[doc = "Fast Slew Rate"]
            pub const SRE_1_FAST_SLEW_RATE: u32 = 0x01;
        }
    }
    #[doc = "Drive Strength Field"]
    pub mod DSE {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "output driver disabled;"]
            pub const DSE_0_OUTPUT_DRIVER_DISABLED_: u32 = 0;
            #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V)"]
            pub const DSE_1_R0_150_OHM___3_3V__260_OHM_1_8V: u32 = 0x01;
            #[doc = "R0/2"]
            pub const DSE_2_R0_2: u32 = 0x02;
            #[doc = "R0/3"]
            pub const DSE_3_R0_3: u32 = 0x03;
            #[doc = "R0/4"]
            pub const DSE_4_R0_4: u32 = 0x04;
            #[doc = "R0/5"]
            pub const DSE_5_R0_5: u32 = 0x05;
            #[doc = "R0/6"]
            pub const DSE_6_R0_6: u32 = 0x06;
            #[doc = "R0/7"]
            pub const DSE_7_R0_7: u32 = 0x07;
        }
    }
    #[doc = "Speed Field"]
    pub mod SPEED {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "low(50MHz)"]
            pub const SPEED_0_LOW_50MHZ_: u32 = 0;
            #[doc = "medium(100MHz)"]
            pub const SPEED_1_MEDIUM_100MHZ_: u32 = 0x01;
            #[doc = "medium(100MHz)"]
            pub const SPEED_2_MEDIUM_100MHZ_: u32 = 0x02;
            #[doc = "max(200MHz)"]
            pub const SPEED_3_MAX_200MHZ_: u32 = 0x03;
        }
    }
    #[doc = "Open Drain Enable Field"]
    pub mod ODE {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Open Drain Disabled"]
            pub const ODE_0_OPEN_DRAIN_DISABLED: u32 = 0;
            #[doc = "Open Drain Enabled"]
            pub const ODE_1_OPEN_DRAIN_ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Pull / Keep Enable Field"]
    pub mod PKE {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pull/Keeper Disabled"]
            pub const PKE_0_PULL_KEEPER_DISABLED: u32 = 0;
            #[doc = "Pull/Keeper Enabled"]
            pub const PKE_1_PULL_KEEPER_ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Pull / Keep Select Field"]
    pub mod PUE {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Keeper"]
            pub const PUE_0_KEEPER: u32 = 0;
            #[doc = "Pull"]
            pub const PUE_1_PULL: u32 = 0x01;
        }
    }
    #[doc = "Pull Up / Down Config. Field"]
    pub mod PUS {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "100K Ohm Pull Down"]
            pub const PUS_0_100K_OHM_PULL_DOWN: u32 = 0;
            #[doc = "47K Ohm Pull Up"]
            pub const PUS_1_47K_OHM_PULL_UP: u32 = 0x01;
            #[doc = "100K Ohm Pull Up"]
            pub const PUS_2_100K_OHM_PULL_UP: u32 = 0x02;
            #[doc = "22K Ohm Pull Up"]
            pub const PUS_3_22K_OHM_PULL_UP: u32 = 0x03;
        }
    }
    #[doc = "Hyst. Enable Field"]
    pub mod HYS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Hysteresis Disabled"]
            pub const HYS_0_HYSTERESIS_DISABLED: u32 = 0;
            #[doc = "Hysteresis Enabled"]
            pub const HYS_1_HYSTERESIS_ENABLED: u32 = 0x01;
        }
    }
}
#[doc = "SW_PAD_CTL_PAD_GPIO_AD_B1_14 SW PAD Control Register"]
pub mod SW_PAD_CTL_PAD_GPIO_AD_B1_14 {
    #[doc = "Slew Rate Field"]
    pub mod SRE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Slow Slew Rate"]
            pub const SRE_0_SLOW_SLEW_RATE: u32 = 0;
            #[doc = "Fast Slew Rate"]
            pub const SRE_1_FAST_SLEW_RATE: u32 = 0x01;
        }
    }
    #[doc = "Drive Strength Field"]
    pub mod DSE {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "output driver disabled;"]
            pub const DSE_0_OUTPUT_DRIVER_DISABLED_: u32 = 0;
            #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V)"]
            pub const DSE_1_R0_150_OHM___3_3V__260_OHM_1_8V: u32 = 0x01;
            #[doc = "R0/2"]
            pub const DSE_2_R0_2: u32 = 0x02;
            #[doc = "R0/3"]
            pub const DSE_3_R0_3: u32 = 0x03;
            #[doc = "R0/4"]
            pub const DSE_4_R0_4: u32 = 0x04;
            #[doc = "R0/5"]
            pub const DSE_5_R0_5: u32 = 0x05;
            #[doc = "R0/6"]
            pub const DSE_6_R0_6: u32 = 0x06;
            #[doc = "R0/7"]
            pub const DSE_7_R0_7: u32 = 0x07;
        }
    }
    #[doc = "Speed Field"]
    pub mod SPEED {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "low(50MHz)"]
            pub const SPEED_0_LOW_50MHZ_: u32 = 0;
            #[doc = "medium(100MHz)"]
            pub const SPEED_1_MEDIUM_100MHZ_: u32 = 0x01;
            #[doc = "medium(100MHz)"]
            pub const SPEED_2_MEDIUM_100MHZ_: u32 = 0x02;
            #[doc = "max(200MHz)"]
            pub const SPEED_3_MAX_200MHZ_: u32 = 0x03;
        }
    }
    #[doc = "Open Drain Enable Field"]
    pub mod ODE {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Open Drain Disabled"]
            pub const ODE_0_OPEN_DRAIN_DISABLED: u32 = 0;
            #[doc = "Open Drain Enabled"]
            pub const ODE_1_OPEN_DRAIN_ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Pull / Keep Enable Field"]
    pub mod PKE {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pull/Keeper Disabled"]
            pub const PKE_0_PULL_KEEPER_DISABLED: u32 = 0;
            #[doc = "Pull/Keeper Enabled"]
            pub const PKE_1_PULL_KEEPER_ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Pull / Keep Select Field"]
    pub mod PUE {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Keeper"]
            pub const PUE_0_KEEPER: u32 = 0;
            #[doc = "Pull"]
            pub const PUE_1_PULL: u32 = 0x01;
        }
    }
    #[doc = "Pull Up / Down Config. Field"]
    pub mod PUS {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "100K Ohm Pull Down"]
            pub const PUS_0_100K_OHM_PULL_DOWN: u32 = 0;
            #[doc = "47K Ohm Pull Up"]
            pub const PUS_1_47K_OHM_PULL_UP: u32 = 0x01;
            #[doc = "100K Ohm Pull Up"]
            pub const PUS_2_100K_OHM_PULL_UP: u32 = 0x02;
            #[doc = "22K Ohm Pull Up"]
            pub const PUS_3_22K_OHM_PULL_UP: u32 = 0x03;
        }
    }
    #[doc = "Hyst. Enable Field"]
    pub mod HYS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Hysteresis Disabled"]
            pub const HYS_0_HYSTERESIS_DISABLED: u32 = 0;
            #[doc = "Hysteresis Enabled"]
            pub const HYS_1_HYSTERESIS_ENABLED: u32 = 0x01;
        }
    }
}
#[doc = "SW_PAD_CTL_PAD_GPIO_AD_B1_15 SW PAD Control Register"]
pub mod SW_PAD_CTL_PAD_GPIO_AD_B1_15 {
    #[doc = "Slew Rate Field"]
    pub mod SRE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Slow Slew Rate"]
            pub const SRE_0_SLOW_SLEW_RATE: u32 = 0;
            #[doc = "Fast Slew Rate"]
            pub const SRE_1_FAST_SLEW_RATE: u32 = 0x01;
        }
    }
    #[doc = "Drive Strength Field"]
    pub mod DSE {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "output driver disabled;"]
            pub const DSE_0_OUTPUT_DRIVER_DISABLED_: u32 = 0;
            #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V)"]
            pub const DSE_1_R0_150_OHM___3_3V__260_OHM_1_8V: u32 = 0x01;
            #[doc = "R0/2"]
            pub const DSE_2_R0_2: u32 = 0x02;
            #[doc = "R0/3"]
            pub const DSE_3_R0_3: u32 = 0x03;
            #[doc = "R0/4"]
            pub const DSE_4_R0_4: u32 = 0x04;
            #[doc = "R0/5"]
            pub const DSE_5_R0_5: u32 = 0x05;
            #[doc = "R0/6"]
            pub const DSE_6_R0_6: u32 = 0x06;
            #[doc = "R0/7"]
            pub const DSE_7_R0_7: u32 = 0x07;
        }
    }
    #[doc = "Speed Field"]
    pub mod SPEED {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "low(50MHz)"]
            pub const SPEED_0_LOW_50MHZ_: u32 = 0;
            #[doc = "medium(100MHz)"]
            pub const SPEED_1_MEDIUM_100MHZ_: u32 = 0x01;
            #[doc = "medium(100MHz)"]
            pub const SPEED_2_MEDIUM_100MHZ_: u32 = 0x02;
            #[doc = "max(200MHz)"]
            pub const SPEED_3_MAX_200MHZ_: u32 = 0x03;
        }
    }
    #[doc = "Open Drain Enable Field"]
    pub mod ODE {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Open Drain Disabled"]
            pub const ODE_0_OPEN_DRAIN_DISABLED: u32 = 0;
            #[doc = "Open Drain Enabled"]
            pub const ODE_1_OPEN_DRAIN_ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Pull / Keep Enable Field"]
    pub mod PKE {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pull/Keeper Disabled"]
            pub const PKE_0_PULL_KEEPER_DISABLED: u32 = 0;
            #[doc = "Pull/Keeper Enabled"]
            pub const PKE_1_PULL_KEEPER_ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Pull / Keep Select Field"]
    pub mod PUE {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Keeper"]
            pub const PUE_0_KEEPER: u32 = 0;
            #[doc = "Pull"]
            pub const PUE_1_PULL: u32 = 0x01;
        }
    }
    #[doc = "Pull Up / Down Config. Field"]
    pub mod PUS {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "100K Ohm Pull Down"]
            pub const PUS_0_100K_OHM_PULL_DOWN: u32 = 0;
            #[doc = "47K Ohm Pull Up"]
            pub const PUS_1_47K_OHM_PULL_UP: u32 = 0x01;
            #[doc = "100K Ohm Pull Up"]
            pub const PUS_2_100K_OHM_PULL_UP: u32 = 0x02;
            #[doc = "22K Ohm Pull Up"]
            pub const PUS_3_22K_OHM_PULL_UP: u32 = 0x03;
        }
    }
    #[doc = "Hyst. Enable Field"]
    pub mod HYS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Hysteresis Disabled"]
            pub const HYS_0_HYSTERESIS_DISABLED: u32 = 0;
            #[doc = "Hysteresis Enabled"]
            pub const HYS_1_HYSTERESIS_ENABLED: u32 = 0x01;
        }
    }
}
#[doc = "SW_PAD_CTL_PAD_GPIO_SD_B1_00 SW PAD Control Register"]
pub mod SW_PAD_CTL_PAD_GPIO_SD_B1_00 {
    #[doc = "Slew Rate Field"]
    pub mod SRE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Slow Slew Rate"]
            pub const SRE_0_SLOW_SLEW_RATE: u32 = 0;
            #[doc = "Fast Slew Rate"]
            pub const SRE_1_FAST_SLEW_RATE: u32 = 0x01;
        }
    }
    #[doc = "Drive Strength Field"]
    pub mod DSE {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "output driver disabled;"]
            pub const DSE_0_OUTPUT_DRIVER_DISABLED_: u32 = 0;
            #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V)"]
            pub const DSE_1_R0_150_OHM___3_3V__260_OHM_1_8V: u32 = 0x01;
            #[doc = "R0/2"]
            pub const DSE_2_R0_2: u32 = 0x02;
            #[doc = "R0/3"]
            pub const DSE_3_R0_3: u32 = 0x03;
            #[doc = "R0/4"]
            pub const DSE_4_R0_4: u32 = 0x04;
            #[doc = "R0/5"]
            pub const DSE_5_R0_5: u32 = 0x05;
            #[doc = "R0/6"]
            pub const DSE_6_R0_6: u32 = 0x06;
            #[doc = "R0/7"]
            pub const DSE_7_R0_7: u32 = 0x07;
        }
    }
    #[doc = "Speed Field"]
    pub mod SPEED {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "low(50MHz)"]
            pub const SPEED_0_LOW_50MHZ_: u32 = 0;
            #[doc = "medium(100MHz)"]
            pub const SPEED_1_MEDIUM_100MHZ_: u32 = 0x01;
            #[doc = "medium(100MHz)"]
            pub const SPEED_2_MEDIUM_100MHZ_: u32 = 0x02;
            #[doc = "max(200MHz)"]
            pub const SPEED_3_MAX_200MHZ_: u32 = 0x03;
        }
    }
    #[doc = "Open Drain Enable Field"]
    pub mod ODE {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Open Drain Disabled"]
            pub const ODE_0_OPEN_DRAIN_DISABLED: u32 = 0;
            #[doc = "Open Drain Enabled"]
            pub const ODE_1_OPEN_DRAIN_ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Pull / Keep Enable Field"]
    pub mod PKE {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pull/Keeper Disabled"]
            pub const PKE_0_PULL_KEEPER_DISABLED: u32 = 0;
            #[doc = "Pull/Keeper Enabled"]
            pub const PKE_1_PULL_KEEPER_ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Pull / Keep Select Field"]
    pub mod PUE {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Keeper"]
            pub const PUE_0_KEEPER: u32 = 0;
            #[doc = "Pull"]
            pub const PUE_1_PULL: u32 = 0x01;
        }
    }
    #[doc = "Pull Up / Down Config. Field"]
    pub mod PUS {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "100K Ohm Pull Down"]
            pub const PUS_0_100K_OHM_PULL_DOWN: u32 = 0;
            #[doc = "47K Ohm Pull Up"]
            pub const PUS_1_47K_OHM_PULL_UP: u32 = 0x01;
            #[doc = "100K Ohm Pull Up"]
            pub const PUS_2_100K_OHM_PULL_UP: u32 = 0x02;
            #[doc = "22K Ohm Pull Up"]
            pub const PUS_3_22K_OHM_PULL_UP: u32 = 0x03;
        }
    }
    #[doc = "Hyst. Enable Field"]
    pub mod HYS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Hysteresis Disabled"]
            pub const HYS_0_HYSTERESIS_DISABLED: u32 = 0;
            #[doc = "Hysteresis Enabled"]
            pub const HYS_1_HYSTERESIS_ENABLED: u32 = 0x01;
        }
    }
}
#[doc = "SW_PAD_CTL_PAD_GPIO_SD_B1_01 SW PAD Control Register"]
pub mod SW_PAD_CTL_PAD_GPIO_SD_B1_01 {
    #[doc = "Slew Rate Field"]
    pub mod SRE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Slow Slew Rate"]
            pub const SRE_0_SLOW_SLEW_RATE: u32 = 0;
            #[doc = "Fast Slew Rate"]
            pub const SRE_1_FAST_SLEW_RATE: u32 = 0x01;
        }
    }
    #[doc = "Drive Strength Field"]
    pub mod DSE {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "output driver disabled;"]
            pub const DSE_0_OUTPUT_DRIVER_DISABLED_: u32 = 0;
            #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V)"]
            pub const DSE_1_R0_150_OHM___3_3V__260_OHM_1_8V: u32 = 0x01;
            #[doc = "R0/2"]
            pub const DSE_2_R0_2: u32 = 0x02;
            #[doc = "R0/3"]
            pub const DSE_3_R0_3: u32 = 0x03;
            #[doc = "R0/4"]
            pub const DSE_4_R0_4: u32 = 0x04;
            #[doc = "R0/5"]
            pub const DSE_5_R0_5: u32 = 0x05;
            #[doc = "R0/6"]
            pub const DSE_6_R0_6: u32 = 0x06;
            #[doc = "R0/7"]
            pub const DSE_7_R0_7: u32 = 0x07;
        }
    }
    #[doc = "Speed Field"]
    pub mod SPEED {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "low(50MHz)"]
            pub const SPEED_0_LOW_50MHZ_: u32 = 0;
            #[doc = "medium(100MHz)"]
            pub const SPEED_1_MEDIUM_100MHZ_: u32 = 0x01;
            #[doc = "medium(100MHz)"]
            pub const SPEED_2_MEDIUM_100MHZ_: u32 = 0x02;
            #[doc = "max(200MHz)"]
            pub const SPEED_3_MAX_200MHZ_: u32 = 0x03;
        }
    }
    #[doc = "Open Drain Enable Field"]
    pub mod ODE {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Open Drain Disabled"]
            pub const ODE_0_OPEN_DRAIN_DISABLED: u32 = 0;
            #[doc = "Open Drain Enabled"]
            pub const ODE_1_OPEN_DRAIN_ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Pull / Keep Enable Field"]
    pub mod PKE {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pull/Keeper Disabled"]
            pub const PKE_0_PULL_KEEPER_DISABLED: u32 = 0;
            #[doc = "Pull/Keeper Enabled"]
            pub const PKE_1_PULL_KEEPER_ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Pull / Keep Select Field"]
    pub mod PUE {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Keeper"]
            pub const PUE_0_KEEPER: u32 = 0;
            #[doc = "Pull"]
            pub const PUE_1_PULL: u32 = 0x01;
        }
    }
    #[doc = "Pull Up / Down Config. Field"]
    pub mod PUS {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "100K Ohm Pull Down"]
            pub const PUS_0_100K_OHM_PULL_DOWN: u32 = 0;
            #[doc = "47K Ohm Pull Up"]
            pub const PUS_1_47K_OHM_PULL_UP: u32 = 0x01;
            #[doc = "100K Ohm Pull Up"]
            pub const PUS_2_100K_OHM_PULL_UP: u32 = 0x02;
            #[doc = "22K Ohm Pull Up"]
            pub const PUS_3_22K_OHM_PULL_UP: u32 = 0x03;
        }
    }
    #[doc = "Hyst. Enable Field"]
    pub mod HYS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Hysteresis Disabled"]
            pub const HYS_0_HYSTERESIS_DISABLED: u32 = 0;
            #[doc = "Hysteresis Enabled"]
            pub const HYS_1_HYSTERESIS_ENABLED: u32 = 0x01;
        }
    }
}
#[doc = "SW_PAD_CTL_PAD_GPIO_SD_B1_02 SW PAD Control Register"]
pub mod SW_PAD_CTL_PAD_GPIO_SD_B1_02 {
    #[doc = "Slew Rate Field"]
    pub mod SRE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Slow Slew Rate"]
            pub const SRE_0_SLOW_SLEW_RATE: u32 = 0;
            #[doc = "Fast Slew Rate"]
            pub const SRE_1_FAST_SLEW_RATE: u32 = 0x01;
        }
    }
    #[doc = "Drive Strength Field"]
    pub mod DSE {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "output driver disabled;"]
            pub const DSE_0_OUTPUT_DRIVER_DISABLED_: u32 = 0;
            #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V)"]
            pub const DSE_1_R0_150_OHM___3_3V__260_OHM_1_8V: u32 = 0x01;
            #[doc = "R0/2"]
            pub const DSE_2_R0_2: u32 = 0x02;
            #[doc = "R0/3"]
            pub const DSE_3_R0_3: u32 = 0x03;
            #[doc = "R0/4"]
            pub const DSE_4_R0_4: u32 = 0x04;
            #[doc = "R0/5"]
            pub const DSE_5_R0_5: u32 = 0x05;
            #[doc = "R0/6"]
            pub const DSE_6_R0_6: u32 = 0x06;
            #[doc = "R0/7"]
            pub const DSE_7_R0_7: u32 = 0x07;
        }
    }
    #[doc = "Speed Field"]
    pub mod SPEED {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "low(50MHz)"]
            pub const SPEED_0_LOW_50MHZ_: u32 = 0;
            #[doc = "medium(100MHz)"]
            pub const SPEED_1_MEDIUM_100MHZ_: u32 = 0x01;
            #[doc = "medium(100MHz)"]
            pub const SPEED_2_MEDIUM_100MHZ_: u32 = 0x02;
            #[doc = "max(200MHz)"]
            pub const SPEED_3_MAX_200MHZ_: u32 = 0x03;
        }
    }
    #[doc = "Open Drain Enable Field"]
    pub mod ODE {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Open Drain Disabled"]
            pub const ODE_0_OPEN_DRAIN_DISABLED: u32 = 0;
            #[doc = "Open Drain Enabled"]
            pub const ODE_1_OPEN_DRAIN_ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Pull / Keep Enable Field"]
    pub mod PKE {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pull/Keeper Disabled"]
            pub const PKE_0_PULL_KEEPER_DISABLED: u32 = 0;
            #[doc = "Pull/Keeper Enabled"]
            pub const PKE_1_PULL_KEEPER_ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Pull / Keep Select Field"]
    pub mod PUE {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Keeper"]
            pub const PUE_0_KEEPER: u32 = 0;
            #[doc = "Pull"]
            pub const PUE_1_PULL: u32 = 0x01;
        }
    }
    #[doc = "Pull Up / Down Config. Field"]
    pub mod PUS {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "100K Ohm Pull Down"]
            pub const PUS_0_100K_OHM_PULL_DOWN: u32 = 0;
            #[doc = "47K Ohm Pull Up"]
            pub const PUS_1_47K_OHM_PULL_UP: u32 = 0x01;
            #[doc = "100K Ohm Pull Up"]
            pub const PUS_2_100K_OHM_PULL_UP: u32 = 0x02;
            #[doc = "22K Ohm Pull Up"]
            pub const PUS_3_22K_OHM_PULL_UP: u32 = 0x03;
        }
    }
    #[doc = "Hyst. Enable Field"]
    pub mod HYS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Hysteresis Disabled"]
            pub const HYS_0_HYSTERESIS_DISABLED: u32 = 0;
            #[doc = "Hysteresis Enabled"]
            pub const HYS_1_HYSTERESIS_ENABLED: u32 = 0x01;
        }
    }
}
#[doc = "SW_PAD_CTL_PAD_GPIO_SD_B1_03 SW PAD Control Register"]
pub mod SW_PAD_CTL_PAD_GPIO_SD_B1_03 {
    #[doc = "Slew Rate Field"]
    pub mod SRE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Slow Slew Rate"]
            pub const SRE_0_SLOW_SLEW_RATE: u32 = 0;
            #[doc = "Fast Slew Rate"]
            pub const SRE_1_FAST_SLEW_RATE: u32 = 0x01;
        }
    }
    #[doc = "Drive Strength Field"]
    pub mod DSE {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "output driver disabled;"]
            pub const DSE_0_OUTPUT_DRIVER_DISABLED_: u32 = 0;
            #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V)"]
            pub const DSE_1_R0_150_OHM___3_3V__260_OHM_1_8V: u32 = 0x01;
            #[doc = "R0/2"]
            pub const DSE_2_R0_2: u32 = 0x02;
            #[doc = "R0/3"]
            pub const DSE_3_R0_3: u32 = 0x03;
            #[doc = "R0/4"]
            pub const DSE_4_R0_4: u32 = 0x04;
            #[doc = "R0/5"]
            pub const DSE_5_R0_5: u32 = 0x05;
            #[doc = "R0/6"]
            pub const DSE_6_R0_6: u32 = 0x06;
            #[doc = "R0/7"]
            pub const DSE_7_R0_7: u32 = 0x07;
        }
    }
    #[doc = "Speed Field"]
    pub mod SPEED {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "low(50MHz)"]
            pub const SPEED_0_LOW_50MHZ_: u32 = 0;
            #[doc = "medium(100MHz)"]
            pub const SPEED_1_MEDIUM_100MHZ_: u32 = 0x01;
            #[doc = "medium(100MHz)"]
            pub const SPEED_2_MEDIUM_100MHZ_: u32 = 0x02;
            #[doc = "max(200MHz)"]
            pub const SPEED_3_MAX_200MHZ_: u32 = 0x03;
        }
    }
    #[doc = "Open Drain Enable Field"]
    pub mod ODE {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Open Drain Disabled"]
            pub const ODE_0_OPEN_DRAIN_DISABLED: u32 = 0;
            #[doc = "Open Drain Enabled"]
            pub const ODE_1_OPEN_DRAIN_ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Pull / Keep Enable Field"]
    pub mod PKE {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pull/Keeper Disabled"]
            pub const PKE_0_PULL_KEEPER_DISABLED: u32 = 0;
            #[doc = "Pull/Keeper Enabled"]
            pub const PKE_1_PULL_KEEPER_ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Pull / Keep Select Field"]
    pub mod PUE {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Keeper"]
            pub const PUE_0_KEEPER: u32 = 0;
            #[doc = "Pull"]
            pub const PUE_1_PULL: u32 = 0x01;
        }
    }
    #[doc = "Pull Up / Down Config. Field"]
    pub mod PUS {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "100K Ohm Pull Down"]
            pub const PUS_0_100K_OHM_PULL_DOWN: u32 = 0;
            #[doc = "47K Ohm Pull Up"]
            pub const PUS_1_47K_OHM_PULL_UP: u32 = 0x01;
            #[doc = "100K Ohm Pull Up"]
            pub const PUS_2_100K_OHM_PULL_UP: u32 = 0x02;
            #[doc = "22K Ohm Pull Up"]
            pub const PUS_3_22K_OHM_PULL_UP: u32 = 0x03;
        }
    }
    #[doc = "Hyst. Enable Field"]
    pub mod HYS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Hysteresis Disabled"]
            pub const HYS_0_HYSTERESIS_DISABLED: u32 = 0;
            #[doc = "Hysteresis Enabled"]
            pub const HYS_1_HYSTERESIS_ENABLED: u32 = 0x01;
        }
    }
}
#[doc = "SW_PAD_CTL_PAD_GPIO_SD_B1_04 SW PAD Control Register"]
pub mod SW_PAD_CTL_PAD_GPIO_SD_B1_04 {
    #[doc = "Slew Rate Field"]
    pub mod SRE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Slow Slew Rate"]
            pub const SRE_0_SLOW_SLEW_RATE: u32 = 0;
            #[doc = "Fast Slew Rate"]
            pub const SRE_1_FAST_SLEW_RATE: u32 = 0x01;
        }
    }
    #[doc = "Drive Strength Field"]
    pub mod DSE {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "output driver disabled;"]
            pub const DSE_0_OUTPUT_DRIVER_DISABLED_: u32 = 0;
            #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V)"]
            pub const DSE_1_R0_150_OHM___3_3V__260_OHM_1_8V: u32 = 0x01;
            #[doc = "R0/2"]
            pub const DSE_2_R0_2: u32 = 0x02;
            #[doc = "R0/3"]
            pub const DSE_3_R0_3: u32 = 0x03;
            #[doc = "R0/4"]
            pub const DSE_4_R0_4: u32 = 0x04;
            #[doc = "R0/5"]
            pub const DSE_5_R0_5: u32 = 0x05;
            #[doc = "R0/6"]
            pub const DSE_6_R0_6: u32 = 0x06;
            #[doc = "R0/7"]
            pub const DSE_7_R0_7: u32 = 0x07;
        }
    }
    #[doc = "Speed Field"]
    pub mod SPEED {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "low(50MHz)"]
            pub const SPEED_0_LOW_50MHZ_: u32 = 0;
            #[doc = "medium(100MHz)"]
            pub const SPEED_1_MEDIUM_100MHZ_: u32 = 0x01;
            #[doc = "medium(100MHz)"]
            pub const SPEED_2_MEDIUM_100MHZ_: u32 = 0x02;
            #[doc = "max(200MHz)"]
            pub const SPEED_3_MAX_200MHZ_: u32 = 0x03;
        }
    }
    #[doc = "Open Drain Enable Field"]
    pub mod ODE {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Open Drain Disabled"]
            pub const ODE_0_OPEN_DRAIN_DISABLED: u32 = 0;
            #[doc = "Open Drain Enabled"]
            pub const ODE_1_OPEN_DRAIN_ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Pull / Keep Enable Field"]
    pub mod PKE {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pull/Keeper Disabled"]
            pub const PKE_0_PULL_KEEPER_DISABLED: u32 = 0;
            #[doc = "Pull/Keeper Enabled"]
            pub const PKE_1_PULL_KEEPER_ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Pull / Keep Select Field"]
    pub mod PUE {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Keeper"]
            pub const PUE_0_KEEPER: u32 = 0;
            #[doc = "Pull"]
            pub const PUE_1_PULL: u32 = 0x01;
        }
    }
    #[doc = "Pull Up / Down Config. Field"]
    pub mod PUS {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "100K Ohm Pull Down"]
            pub const PUS_0_100K_OHM_PULL_DOWN: u32 = 0;
            #[doc = "47K Ohm Pull Up"]
            pub const PUS_1_47K_OHM_PULL_UP: u32 = 0x01;
            #[doc = "100K Ohm Pull Up"]
            pub const PUS_2_100K_OHM_PULL_UP: u32 = 0x02;
            #[doc = "22K Ohm Pull Up"]
            pub const PUS_3_22K_OHM_PULL_UP: u32 = 0x03;
        }
    }
    #[doc = "Hyst. Enable Field"]
    pub mod HYS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Hysteresis Disabled"]
            pub const HYS_0_HYSTERESIS_DISABLED: u32 = 0;
            #[doc = "Hysteresis Enabled"]
            pub const HYS_1_HYSTERESIS_ENABLED: u32 = 0x01;
        }
    }
}
#[doc = "SW_PAD_CTL_PAD_GPIO_SD_B1_05 SW PAD Control Register"]
pub mod SW_PAD_CTL_PAD_GPIO_SD_B1_05 {
    #[doc = "Slew Rate Field"]
    pub mod SRE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Slow Slew Rate"]
            pub const SRE_0_SLOW_SLEW_RATE: u32 = 0;
            #[doc = "Fast Slew Rate"]
            pub const SRE_1_FAST_SLEW_RATE: u32 = 0x01;
        }
    }
    #[doc = "Drive Strength Field"]
    pub mod DSE {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "output driver disabled;"]
            pub const DSE_0_OUTPUT_DRIVER_DISABLED_: u32 = 0;
            #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V)"]
            pub const DSE_1_R0_150_OHM___3_3V__260_OHM_1_8V: u32 = 0x01;
            #[doc = "R0/2"]
            pub const DSE_2_R0_2: u32 = 0x02;
            #[doc = "R0/3"]
            pub const DSE_3_R0_3: u32 = 0x03;
            #[doc = "R0/4"]
            pub const DSE_4_R0_4: u32 = 0x04;
            #[doc = "R0/5"]
            pub const DSE_5_R0_5: u32 = 0x05;
            #[doc = "R0/6"]
            pub const DSE_6_R0_6: u32 = 0x06;
            #[doc = "R0/7"]
            pub const DSE_7_R0_7: u32 = 0x07;
        }
    }
    #[doc = "Speed Field"]
    pub mod SPEED {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "low(50MHz)"]
            pub const SPEED_0_LOW_50MHZ_: u32 = 0;
            #[doc = "medium(100MHz)"]
            pub const SPEED_1_MEDIUM_100MHZ_: u32 = 0x01;
            #[doc = "medium(100MHz)"]
            pub const SPEED_2_MEDIUM_100MHZ_: u32 = 0x02;
            #[doc = "max(200MHz)"]
            pub const SPEED_3_MAX_200MHZ_: u32 = 0x03;
        }
    }
    #[doc = "Open Drain Enable Field"]
    pub mod ODE {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Open Drain Disabled"]
            pub const ODE_0_OPEN_DRAIN_DISABLED: u32 = 0;
            #[doc = "Open Drain Enabled"]
            pub const ODE_1_OPEN_DRAIN_ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Pull / Keep Enable Field"]
    pub mod PKE {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pull/Keeper Disabled"]
            pub const PKE_0_PULL_KEEPER_DISABLED: u32 = 0;
            #[doc = "Pull/Keeper Enabled"]
            pub const PKE_1_PULL_KEEPER_ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Pull / Keep Select Field"]
    pub mod PUE {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Keeper"]
            pub const PUE_0_KEEPER: u32 = 0;
            #[doc = "Pull"]
            pub const PUE_1_PULL: u32 = 0x01;
        }
    }
    #[doc = "Pull Up / Down Config. Field"]
    pub mod PUS {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "100K Ohm Pull Down"]
            pub const PUS_0_100K_OHM_PULL_DOWN: u32 = 0;
            #[doc = "47K Ohm Pull Up"]
            pub const PUS_1_47K_OHM_PULL_UP: u32 = 0x01;
            #[doc = "100K Ohm Pull Up"]
            pub const PUS_2_100K_OHM_PULL_UP: u32 = 0x02;
            #[doc = "22K Ohm Pull Up"]
            pub const PUS_3_22K_OHM_PULL_UP: u32 = 0x03;
        }
    }
    #[doc = "Hyst. Enable Field"]
    pub mod HYS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Hysteresis Disabled"]
            pub const HYS_0_HYSTERESIS_DISABLED: u32 = 0;
            #[doc = "Hysteresis Enabled"]
            pub const HYS_1_HYSTERESIS_ENABLED: u32 = 0x01;
        }
    }
}
#[doc = "SW_PAD_CTL_PAD_GPIO_SD_B1_06 SW PAD Control Register"]
pub mod SW_PAD_CTL_PAD_GPIO_SD_B1_06 {
    #[doc = "Slew Rate Field"]
    pub mod SRE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Slow Slew Rate"]
            pub const SRE_0_SLOW_SLEW_RATE: u32 = 0;
            #[doc = "Fast Slew Rate"]
            pub const SRE_1_FAST_SLEW_RATE: u32 = 0x01;
        }
    }
    #[doc = "Drive Strength Field"]
    pub mod DSE {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "output driver disabled;"]
            pub const DSE_0_OUTPUT_DRIVER_DISABLED_: u32 = 0;
            #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V)"]
            pub const DSE_1_R0_150_OHM___3_3V__260_OHM_1_8V: u32 = 0x01;
            #[doc = "R0/2"]
            pub const DSE_2_R0_2: u32 = 0x02;
            #[doc = "R0/3"]
            pub const DSE_3_R0_3: u32 = 0x03;
            #[doc = "R0/4"]
            pub const DSE_4_R0_4: u32 = 0x04;
            #[doc = "R0/5"]
            pub const DSE_5_R0_5: u32 = 0x05;
            #[doc = "R0/6"]
            pub const DSE_6_R0_6: u32 = 0x06;
            #[doc = "R0/7"]
            pub const DSE_7_R0_7: u32 = 0x07;
        }
    }
    #[doc = "Speed Field"]
    pub mod SPEED {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "low(50MHz)"]
            pub const SPEED_0_LOW_50MHZ_: u32 = 0;
            #[doc = "medium(100MHz)"]
            pub const SPEED_1_MEDIUM_100MHZ_: u32 = 0x01;
            #[doc = "medium(100MHz)"]
            pub const SPEED_2_MEDIUM_100MHZ_: u32 = 0x02;
            #[doc = "max(200MHz)"]
            pub const SPEED_3_MAX_200MHZ_: u32 = 0x03;
        }
    }
    #[doc = "Open Drain Enable Field"]
    pub mod ODE {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Open Drain Disabled"]
            pub const ODE_0_OPEN_DRAIN_DISABLED: u32 = 0;
            #[doc = "Open Drain Enabled"]
            pub const ODE_1_OPEN_DRAIN_ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Pull / Keep Enable Field"]
    pub mod PKE {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pull/Keeper Disabled"]
            pub const PKE_0_PULL_KEEPER_DISABLED: u32 = 0;
            #[doc = "Pull/Keeper Enabled"]
            pub const PKE_1_PULL_KEEPER_ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Pull / Keep Select Field"]
    pub mod PUE {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Keeper"]
            pub const PUE_0_KEEPER: u32 = 0;
            #[doc = "Pull"]
            pub const PUE_1_PULL: u32 = 0x01;
        }
    }
    #[doc = "Pull Up / Down Config. Field"]
    pub mod PUS {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "100K Ohm Pull Down"]
            pub const PUS_0_100K_OHM_PULL_DOWN: u32 = 0;
            #[doc = "47K Ohm Pull Up"]
            pub const PUS_1_47K_OHM_PULL_UP: u32 = 0x01;
            #[doc = "100K Ohm Pull Up"]
            pub const PUS_2_100K_OHM_PULL_UP: u32 = 0x02;
            #[doc = "22K Ohm Pull Up"]
            pub const PUS_3_22K_OHM_PULL_UP: u32 = 0x03;
        }
    }
    #[doc = "Hyst. Enable Field"]
    pub mod HYS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Hysteresis Disabled"]
            pub const HYS_0_HYSTERESIS_DISABLED: u32 = 0;
            #[doc = "Hysteresis Enabled"]
            pub const HYS_1_HYSTERESIS_ENABLED: u32 = 0x01;
        }
    }
}
#[doc = "SW_PAD_CTL_PAD_GPIO_SD_B1_07 SW PAD Control Register"]
pub mod SW_PAD_CTL_PAD_GPIO_SD_B1_07 {
    #[doc = "Slew Rate Field"]
    pub mod SRE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Slow Slew Rate"]
            pub const SRE_0_SLOW_SLEW_RATE: u32 = 0;
            #[doc = "Fast Slew Rate"]
            pub const SRE_1_FAST_SLEW_RATE: u32 = 0x01;
        }
    }
    #[doc = "Drive Strength Field"]
    pub mod DSE {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "output driver disabled;"]
            pub const DSE_0_OUTPUT_DRIVER_DISABLED_: u32 = 0;
            #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V)"]
            pub const DSE_1_R0_150_OHM___3_3V__260_OHM_1_8V: u32 = 0x01;
            #[doc = "R0/2"]
            pub const DSE_2_R0_2: u32 = 0x02;
            #[doc = "R0/3"]
            pub const DSE_3_R0_3: u32 = 0x03;
            #[doc = "R0/4"]
            pub const DSE_4_R0_4: u32 = 0x04;
            #[doc = "R0/5"]
            pub const DSE_5_R0_5: u32 = 0x05;
            #[doc = "R0/6"]
            pub const DSE_6_R0_6: u32 = 0x06;
            #[doc = "R0/7"]
            pub const DSE_7_R0_7: u32 = 0x07;
        }
    }
    #[doc = "Speed Field"]
    pub mod SPEED {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "low(50MHz)"]
            pub const SPEED_0_LOW_50MHZ_: u32 = 0;
            #[doc = "medium(100MHz)"]
            pub const SPEED_1_MEDIUM_100MHZ_: u32 = 0x01;
            #[doc = "medium(100MHz)"]
            pub const SPEED_2_MEDIUM_100MHZ_: u32 = 0x02;
            #[doc = "max(200MHz)"]
            pub const SPEED_3_MAX_200MHZ_: u32 = 0x03;
        }
    }
    #[doc = "Open Drain Enable Field"]
    pub mod ODE {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Open Drain Disabled"]
            pub const ODE_0_OPEN_DRAIN_DISABLED: u32 = 0;
            #[doc = "Open Drain Enabled"]
            pub const ODE_1_OPEN_DRAIN_ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Pull / Keep Enable Field"]
    pub mod PKE {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pull/Keeper Disabled"]
            pub const PKE_0_PULL_KEEPER_DISABLED: u32 = 0;
            #[doc = "Pull/Keeper Enabled"]
            pub const PKE_1_PULL_KEEPER_ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Pull / Keep Select Field"]
    pub mod PUE {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Keeper"]
            pub const PUE_0_KEEPER: u32 = 0;
            #[doc = "Pull"]
            pub const PUE_1_PULL: u32 = 0x01;
        }
    }
    #[doc = "Pull Up / Down Config. Field"]
    pub mod PUS {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "100K Ohm Pull Down"]
            pub const PUS_0_100K_OHM_PULL_DOWN: u32 = 0;
            #[doc = "47K Ohm Pull Up"]
            pub const PUS_1_47K_OHM_PULL_UP: u32 = 0x01;
            #[doc = "100K Ohm Pull Up"]
            pub const PUS_2_100K_OHM_PULL_UP: u32 = 0x02;
            #[doc = "22K Ohm Pull Up"]
            pub const PUS_3_22K_OHM_PULL_UP: u32 = 0x03;
        }
    }
    #[doc = "Hyst. Enable Field"]
    pub mod HYS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Hysteresis Disabled"]
            pub const HYS_0_HYSTERESIS_DISABLED: u32 = 0;
            #[doc = "Hysteresis Enabled"]
            pub const HYS_1_HYSTERESIS_ENABLED: u32 = 0x01;
        }
    }
}
#[doc = "SW_PAD_CTL_PAD_GPIO_SD_B1_08 SW PAD Control Register"]
pub mod SW_PAD_CTL_PAD_GPIO_SD_B1_08 {
    #[doc = "Slew Rate Field"]
    pub mod SRE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Slow Slew Rate"]
            pub const SRE_0_SLOW_SLEW_RATE: u32 = 0;
            #[doc = "Fast Slew Rate"]
            pub const SRE_1_FAST_SLEW_RATE: u32 = 0x01;
        }
    }
    #[doc = "Drive Strength Field"]
    pub mod DSE {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "output driver disabled;"]
            pub const DSE_0_OUTPUT_DRIVER_DISABLED_: u32 = 0;
            #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V)"]
            pub const DSE_1_R0_150_OHM___3_3V__260_OHM_1_8V: u32 = 0x01;
            #[doc = "R0/2"]
            pub const DSE_2_R0_2: u32 = 0x02;
            #[doc = "R0/3"]
            pub const DSE_3_R0_3: u32 = 0x03;
            #[doc = "R0/4"]
            pub const DSE_4_R0_4: u32 = 0x04;
            #[doc = "R0/5"]
            pub const DSE_5_R0_5: u32 = 0x05;
            #[doc = "R0/6"]
            pub const DSE_6_R0_6: u32 = 0x06;
            #[doc = "R0/7"]
            pub const DSE_7_R0_7: u32 = 0x07;
        }
    }
    #[doc = "Speed Field"]
    pub mod SPEED {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "low(50MHz)"]
            pub const SPEED_0_LOW_50MHZ_: u32 = 0;
            #[doc = "medium(100MHz)"]
            pub const SPEED_1_MEDIUM_100MHZ_: u32 = 0x01;
            #[doc = "medium(100MHz)"]
            pub const SPEED_2_MEDIUM_100MHZ_: u32 = 0x02;
            #[doc = "max(200MHz)"]
            pub const SPEED_3_MAX_200MHZ_: u32 = 0x03;
        }
    }
    #[doc = "Open Drain Enable Field"]
    pub mod ODE {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Open Drain Disabled"]
            pub const ODE_0_OPEN_DRAIN_DISABLED: u32 = 0;
            #[doc = "Open Drain Enabled"]
            pub const ODE_1_OPEN_DRAIN_ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Pull / Keep Enable Field"]
    pub mod PKE {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pull/Keeper Disabled"]
            pub const PKE_0_PULL_KEEPER_DISABLED: u32 = 0;
            #[doc = "Pull/Keeper Enabled"]
            pub const PKE_1_PULL_KEEPER_ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Pull / Keep Select Field"]
    pub mod PUE {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Keeper"]
            pub const PUE_0_KEEPER: u32 = 0;
            #[doc = "Pull"]
            pub const PUE_1_PULL: u32 = 0x01;
        }
    }
    #[doc = "Pull Up / Down Config. Field"]
    pub mod PUS {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "100K Ohm Pull Down"]
            pub const PUS_0_100K_OHM_PULL_DOWN: u32 = 0;
            #[doc = "47K Ohm Pull Up"]
            pub const PUS_1_47K_OHM_PULL_UP: u32 = 0x01;
            #[doc = "100K Ohm Pull Up"]
            pub const PUS_2_100K_OHM_PULL_UP: u32 = 0x02;
            #[doc = "22K Ohm Pull Up"]
            pub const PUS_3_22K_OHM_PULL_UP: u32 = 0x03;
        }
    }
    #[doc = "Hyst. Enable Field"]
    pub mod HYS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Hysteresis Disabled"]
            pub const HYS_0_HYSTERESIS_DISABLED: u32 = 0;
            #[doc = "Hysteresis Enabled"]
            pub const HYS_1_HYSTERESIS_ENABLED: u32 = 0x01;
        }
    }
}
#[doc = "SW_PAD_CTL_PAD_GPIO_SD_B1_09 SW PAD Control Register"]
pub mod SW_PAD_CTL_PAD_GPIO_SD_B1_09 {
    #[doc = "Slew Rate Field"]
    pub mod SRE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Slow Slew Rate"]
            pub const SRE_0_SLOW_SLEW_RATE: u32 = 0;
            #[doc = "Fast Slew Rate"]
            pub const SRE_1_FAST_SLEW_RATE: u32 = 0x01;
        }
    }
    #[doc = "Drive Strength Field"]
    pub mod DSE {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "output driver disabled;"]
            pub const DSE_0_OUTPUT_DRIVER_DISABLED_: u32 = 0;
            #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V)"]
            pub const DSE_1_R0_150_OHM___3_3V__260_OHM_1_8V: u32 = 0x01;
            #[doc = "R0/2"]
            pub const DSE_2_R0_2: u32 = 0x02;
            #[doc = "R0/3"]
            pub const DSE_3_R0_3: u32 = 0x03;
            #[doc = "R0/4"]
            pub const DSE_4_R0_4: u32 = 0x04;
            #[doc = "R0/5"]
            pub const DSE_5_R0_5: u32 = 0x05;
            #[doc = "R0/6"]
            pub const DSE_6_R0_6: u32 = 0x06;
            #[doc = "R0/7"]
            pub const DSE_7_R0_7: u32 = 0x07;
        }
    }
    #[doc = "Speed Field"]
    pub mod SPEED {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "low(50MHz)"]
            pub const SPEED_0_LOW_50MHZ_: u32 = 0;
            #[doc = "medium(100MHz)"]
            pub const SPEED_1_MEDIUM_100MHZ_: u32 = 0x01;
            #[doc = "medium(100MHz)"]
            pub const SPEED_2_MEDIUM_100MHZ_: u32 = 0x02;
            #[doc = "max(200MHz)"]
            pub const SPEED_3_MAX_200MHZ_: u32 = 0x03;
        }
    }
    #[doc = "Open Drain Enable Field"]
    pub mod ODE {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Open Drain Disabled"]
            pub const ODE_0_OPEN_DRAIN_DISABLED: u32 = 0;
            #[doc = "Open Drain Enabled"]
            pub const ODE_1_OPEN_DRAIN_ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Pull / Keep Enable Field"]
    pub mod PKE {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pull/Keeper Disabled"]
            pub const PKE_0_PULL_KEEPER_DISABLED: u32 = 0;
            #[doc = "Pull/Keeper Enabled"]
            pub const PKE_1_PULL_KEEPER_ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Pull / Keep Select Field"]
    pub mod PUE {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Keeper"]
            pub const PUE_0_KEEPER: u32 = 0;
            #[doc = "Pull"]
            pub const PUE_1_PULL: u32 = 0x01;
        }
    }
    #[doc = "Pull Up / Down Config. Field"]
    pub mod PUS {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "100K Ohm Pull Down"]
            pub const PUS_0_100K_OHM_PULL_DOWN: u32 = 0;
            #[doc = "47K Ohm Pull Up"]
            pub const PUS_1_47K_OHM_PULL_UP: u32 = 0x01;
            #[doc = "100K Ohm Pull Up"]
            pub const PUS_2_100K_OHM_PULL_UP: u32 = 0x02;
            #[doc = "22K Ohm Pull Up"]
            pub const PUS_3_22K_OHM_PULL_UP: u32 = 0x03;
        }
    }
    #[doc = "Hyst. Enable Field"]
    pub mod HYS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Hysteresis Disabled"]
            pub const HYS_0_HYSTERESIS_DISABLED: u32 = 0;
            #[doc = "Hysteresis Enabled"]
            pub const HYS_1_HYSTERESIS_ENABLED: u32 = 0x01;
        }
    }
}
#[doc = "SW_PAD_CTL_PAD_GPIO_SD_B1_10 SW PAD Control Register"]
pub mod SW_PAD_CTL_PAD_GPIO_SD_B1_10 {
    #[doc = "Slew Rate Field"]
    pub mod SRE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Slow Slew Rate"]
            pub const SRE_0_SLOW_SLEW_RATE: u32 = 0;
            #[doc = "Fast Slew Rate"]
            pub const SRE_1_FAST_SLEW_RATE: u32 = 0x01;
        }
    }
    #[doc = "Drive Strength Field"]
    pub mod DSE {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "output driver disabled;"]
            pub const DSE_0_OUTPUT_DRIVER_DISABLED_: u32 = 0;
            #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V)"]
            pub const DSE_1_R0_150_OHM___3_3V__260_OHM_1_8V: u32 = 0x01;
            #[doc = "R0/2"]
            pub const DSE_2_R0_2: u32 = 0x02;
            #[doc = "R0/3"]
            pub const DSE_3_R0_3: u32 = 0x03;
            #[doc = "R0/4"]
            pub const DSE_4_R0_4: u32 = 0x04;
            #[doc = "R0/5"]
            pub const DSE_5_R0_5: u32 = 0x05;
            #[doc = "R0/6"]
            pub const DSE_6_R0_6: u32 = 0x06;
            #[doc = "R0/7"]
            pub const DSE_7_R0_7: u32 = 0x07;
        }
    }
    #[doc = "Speed Field"]
    pub mod SPEED {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "low(50MHz)"]
            pub const SPEED_0_LOW_50MHZ_: u32 = 0;
            #[doc = "medium(100MHz)"]
            pub const SPEED_1_MEDIUM_100MHZ_: u32 = 0x01;
            #[doc = "medium(100MHz)"]
            pub const SPEED_2_MEDIUM_100MHZ_: u32 = 0x02;
            #[doc = "max(200MHz)"]
            pub const SPEED_3_MAX_200MHZ_: u32 = 0x03;
        }
    }
    #[doc = "Open Drain Enable Field"]
    pub mod ODE {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Open Drain Disabled"]
            pub const ODE_0_OPEN_DRAIN_DISABLED: u32 = 0;
            #[doc = "Open Drain Enabled"]
            pub const ODE_1_OPEN_DRAIN_ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Pull / Keep Enable Field"]
    pub mod PKE {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pull/Keeper Disabled"]
            pub const PKE_0_PULL_KEEPER_DISABLED: u32 = 0;
            #[doc = "Pull/Keeper Enabled"]
            pub const PKE_1_PULL_KEEPER_ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Pull / Keep Select Field"]
    pub mod PUE {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Keeper"]
            pub const PUE_0_KEEPER: u32 = 0;
            #[doc = "Pull"]
            pub const PUE_1_PULL: u32 = 0x01;
        }
    }
    #[doc = "Pull Up / Down Config. Field"]
    pub mod PUS {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "100K Ohm Pull Down"]
            pub const PUS_0_100K_OHM_PULL_DOWN: u32 = 0;
            #[doc = "47K Ohm Pull Up"]
            pub const PUS_1_47K_OHM_PULL_UP: u32 = 0x01;
            #[doc = "100K Ohm Pull Up"]
            pub const PUS_2_100K_OHM_PULL_UP: u32 = 0x02;
            #[doc = "22K Ohm Pull Up"]
            pub const PUS_3_22K_OHM_PULL_UP: u32 = 0x03;
        }
    }
    #[doc = "Hyst. Enable Field"]
    pub mod HYS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Hysteresis Disabled"]
            pub const HYS_0_HYSTERESIS_DISABLED: u32 = 0;
            #[doc = "Hysteresis Enabled"]
            pub const HYS_1_HYSTERESIS_ENABLED: u32 = 0x01;
        }
    }
}
#[doc = "SW_PAD_CTL_PAD_GPIO_SD_B1_11 SW PAD Control Register"]
pub mod SW_PAD_CTL_PAD_GPIO_SD_B1_11 {
    #[doc = "Slew Rate Field"]
    pub mod SRE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Slow Slew Rate"]
            pub const SRE_0_SLOW_SLEW_RATE: u32 = 0;
            #[doc = "Fast Slew Rate"]
            pub const SRE_1_FAST_SLEW_RATE: u32 = 0x01;
        }
    }
    #[doc = "Drive Strength Field"]
    pub mod DSE {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "output driver disabled;"]
            pub const DSE_0_OUTPUT_DRIVER_DISABLED_: u32 = 0;
            #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V)"]
            pub const DSE_1_R0_150_OHM___3_3V__260_OHM_1_8V: u32 = 0x01;
            #[doc = "R0/2"]
            pub const DSE_2_R0_2: u32 = 0x02;
            #[doc = "R0/3"]
            pub const DSE_3_R0_3: u32 = 0x03;
            #[doc = "R0/4"]
            pub const DSE_4_R0_4: u32 = 0x04;
            #[doc = "R0/5"]
            pub const DSE_5_R0_5: u32 = 0x05;
            #[doc = "R0/6"]
            pub const DSE_6_R0_6: u32 = 0x06;
            #[doc = "R0/7"]
            pub const DSE_7_R0_7: u32 = 0x07;
        }
    }
    #[doc = "Speed Field"]
    pub mod SPEED {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "low(50MHz)"]
            pub const SPEED_0_LOW_50MHZ_: u32 = 0;
            #[doc = "medium(100MHz)"]
            pub const SPEED_1_MEDIUM_100MHZ_: u32 = 0x01;
            #[doc = "medium(100MHz)"]
            pub const SPEED_2_MEDIUM_100MHZ_: u32 = 0x02;
            #[doc = "max(200MHz)"]
            pub const SPEED_3_MAX_200MHZ_: u32 = 0x03;
        }
    }
    #[doc = "Open Drain Enable Field"]
    pub mod ODE {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Open Drain Disabled"]
            pub const ODE_0_OPEN_DRAIN_DISABLED: u32 = 0;
            #[doc = "Open Drain Enabled"]
            pub const ODE_1_OPEN_DRAIN_ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Pull / Keep Enable Field"]
    pub mod PKE {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pull/Keeper Disabled"]
            pub const PKE_0_PULL_KEEPER_DISABLED: u32 = 0;
            #[doc = "Pull/Keeper Enabled"]
            pub const PKE_1_PULL_KEEPER_ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Pull / Keep Select Field"]
    pub mod PUE {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Keeper"]
            pub const PUE_0_KEEPER: u32 = 0;
            #[doc = "Pull"]
            pub const PUE_1_PULL: u32 = 0x01;
        }
    }
    #[doc = "Pull Up / Down Config. Field"]
    pub mod PUS {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "100K Ohm Pull Down"]
            pub const PUS_0_100K_OHM_PULL_DOWN: u32 = 0;
            #[doc = "47K Ohm Pull Up"]
            pub const PUS_1_47K_OHM_PULL_UP: u32 = 0x01;
            #[doc = "100K Ohm Pull Up"]
            pub const PUS_2_100K_OHM_PULL_UP: u32 = 0x02;
            #[doc = "22K Ohm Pull Up"]
            pub const PUS_3_22K_OHM_PULL_UP: u32 = 0x03;
        }
    }
    #[doc = "Hyst. Enable Field"]
    pub mod HYS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Hysteresis Disabled"]
            pub const HYS_0_HYSTERESIS_DISABLED: u32 = 0;
            #[doc = "Hysteresis Enabled"]
            pub const HYS_1_HYSTERESIS_ENABLED: u32 = 0x01;
        }
    }
}
#[doc = "ANATOP_USB_OTG_ID_SELECT_INPUT DAISY Register"]
pub mod ANATOP_USB_OTG_ID_SELECT_INPUT {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    pub mod DAISY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Selecting Pad: GPIO_AD_B0_05 for Mode: ALT6"]
            pub const GPIO_AD_B0_05_ALT6: u32 = 0;
            #[doc = "Selecting Pad: GPIO_AD_B1_11 for Mode: ALT0"]
            pub const GPIO_AD_B1_11_ALT0: u32 = 0x01;
        }
    }
}
#[doc = "CCM_PMIC_VFUNCIONAL_READY_SELECT_INPUT DAISY Register"]
pub mod CCM_PMIC_VFUNCIONAL_READY_SELECT_INPUT {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    pub mod DAISY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Selecting Pad: GPIO_SD_B1_05 for Mode: ALT6"]
            pub const GPIO_SD_B1_05_ALT6: u32 = 0x01;
            #[doc = "Selecting Pad: GPIO_AD_B0_03 for Mode: ALT7"]
            pub const GPIO_AD_B0_03_ALT7: u32 = 0x02;
        }
    }
}
#[doc = "FLEXPWM1_IPP_IND_PWMA_SELECT_INPUT_0 DAISY Register"]
pub mod FLEXPWM1_IPP_IND_PWMA_SELECT_INPUT_0 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    pub mod DAISY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Selecting Pad: GPIO_EMC_26 for Mode: ALT1"]
            pub const GPIO_EMC_26_ALT1: u32 = 0x01;
        }
    }
}
#[doc = "FLEXPWM1_IPP_IND_PWMA_SELECT_INPUT_1 DAISY Register"]
pub mod FLEXPWM1_IPP_IND_PWMA_SELECT_INPUT_1 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    pub mod DAISY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Selecting Pad: GPIO_EMC_24 for Mode: ALT1"]
            pub const GPIO_EMC_24_ALT1: u32 = 0x01;
        }
    }
}
#[doc = "FLEXPWM1_IPP_IND_PWMA_SELECT_INPUT_2 DAISY Register"]
pub mod FLEXPWM1_IPP_IND_PWMA_SELECT_INPUT_2 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    pub mod DAISY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Selecting Pad: GPIO_AD_B1_10 for Mode: ALT1"]
            pub const GPIO_AD_B1_10_ALT1: u32 = 0;
            #[doc = "Selecting Pad: GPIO_EMC_22 for Mode: ALT1"]
            pub const GPIO_EMC_22_ALT1: u32 = 0x01;
        }
    }
}
#[doc = "FLEXPWM1_IPP_IND_PWMA_SELECT_INPUT_3 DAISY Register"]
pub mod FLEXPWM1_IPP_IND_PWMA_SELECT_INPUT_3 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    pub mod DAISY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Selecting Pad: GPIO_AD_B1_12 for Mode: ALT6"]
            pub const GPIO_AD_B1_12_ALT6: u32 = 0;
            #[doc = "Selecting Pad: GPIO_EMC_20 for Mode: ALT1"]
            pub const GPIO_EMC_20_ALT1: u32 = 0x01;
        }
    }
}
#[doc = "FLEXPWM1_IPP_IND_PWMB_SELECT_INPUT_0 DAISY Register"]
pub mod FLEXPWM1_IPP_IND_PWMB_SELECT_INPUT_0 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    pub mod DAISY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Selecting Pad: GPIO_EMC_27 for Mode: ALT1"]
            pub const GPIO_EMC_27_ALT1: u32 = 0x01;
        }
    }
}
#[doc = "FLEXPWM1_IPP_IND_PWMB_SELECT_INPUT_1 DAISY Register"]
pub mod FLEXPWM1_IPP_IND_PWMB_SELECT_INPUT_1 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    pub mod DAISY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Selecting Pad: GPIO_EMC_25 for Mode: ALT1"]
            pub const GPIO_EMC_25_ALT1: u32 = 0x01;
        }
    }
}
#[doc = "FLEXPWM1_IPP_IND_PWMB_SELECT_INPUT_2 DAISY Register"]
pub mod FLEXPWM1_IPP_IND_PWMB_SELECT_INPUT_2 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    pub mod DAISY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Selecting Pad: GPIO_AD_B1_11 for Mode: ALT1"]
            pub const GPIO_AD_B1_11_ALT1: u32 = 0;
            #[doc = "Selecting Pad: GPIO_EMC_23 for Mode: ALT1"]
            pub const GPIO_EMC_23_ALT1: u32 = 0x01;
        }
    }
}
#[doc = "FLEXPWM1_IPP_IND_PWMB_SELECT_INPUT_3 DAISY Register"]
pub mod FLEXPWM1_IPP_IND_PWMB_SELECT_INPUT_3 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    pub mod DAISY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Selecting Pad: GPIO_AD_B1_13 for Mode: ALT6"]
            pub const GPIO_AD_B1_13_ALT6: u32 = 0;
            #[doc = "Selecting Pad: GPIO_EMC_21 for Mode: ALT1"]
            pub const GPIO_EMC_21_ALT1: u32 = 0x01;
        }
    }
}
#[doc = "FLEXSPI_IPP_IND_IO_FA_BIT0_SELECT_INPUT DAISY Register"]
pub mod FLEXSPI_IPP_IND_IO_FA_BIT0_SELECT_INPUT {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    pub mod DAISY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Selecting Pad: GPIO_SD_B1_08 for Mode: ALT1"]
            pub const GPIO_SD_B1_08_ALT1: u32 = 0;
        }
    }
}
#[doc = "FLEXSPI_IPP_IND_IO_FA_BIT1_SELECT_INPUT DAISY Register"]
pub mod FLEXSPI_IPP_IND_IO_FA_BIT1_SELECT_INPUT {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    pub mod DAISY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Selecting Pad: GPIO_SD_B1_10 for Mode: ALT1"]
            pub const GPIO_SD_B1_10_ALT1: u32 = 0;
        }
    }
}
#[doc = "FLEXSPI_IPP_IND_IO_FA_BIT2_SELECT_INPUT DAISY Register"]
pub mod FLEXSPI_IPP_IND_IO_FA_BIT2_SELECT_INPUT {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    pub mod DAISY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Selecting Pad: GPIO_SD_B1_09 for Mode: ALT1"]
            pub const GPIO_SD_B1_09_ALT1: u32 = 0;
        }
    }
}
#[doc = "FLEXSPI_IPP_IND_IO_FA_BIT3_SELECT_INPUT DAISY Register"]
pub mod FLEXSPI_IPP_IND_IO_FA_BIT3_SELECT_INPUT {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    pub mod DAISY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Selecting Pad: GPIO_SD_B1_06 for Mode: ALT1"]
            pub const GPIO_SD_B1_06_ALT1: u32 = 0;
        }
    }
}
#[doc = "FLEXSPI_IPP_IND_SCK_FA_SELECT_INPUT DAISY Register"]
pub mod FLEXSPI_IPP_IND_SCK_FA_SELECT_INPUT {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    pub mod DAISY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Selecting Pad: GPIO_SD_B1_07 for Mode: ALT1"]
            pub const GPIO_SD_B1_07_ALT1: u32 = 0;
        }
    }
}
#[doc = "LPI2C1_IPP_IND_LPI2C_SCL_SELECT_INPUT DAISY Register"]
pub mod LPI2C1_IPP_IND_LPI2C_SCL_SELECT_INPUT {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    pub mod DAISY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Selecting Pad: GPIO_AD_B1_14 for Mode: ALT0"]
            pub const GPIO_AD_B1_14_ALT0: u32 = 0x01;
        }
    }
}
#[doc = "LPI2C1_IPP_IND_LPI2C_SDA_SELECT_INPUT DAISY Register"]
pub mod LPI2C1_IPP_IND_LPI2C_SDA_SELECT_INPUT {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    pub mod DAISY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Selecting Pad: GPIO_AD_B1_15 for Mode: ALT0"]
            pub const GPIO_AD_B1_15_ALT0: u32 = 0x01;
        }
    }
}
#[doc = "LPI2C2_IPP_IND_LPI2C_SCL_SELECT_INPUT DAISY Register"]
pub mod LPI2C2_IPP_IND_LPI2C_SCL_SELECT_INPUT {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    pub mod DAISY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Selecting Pad: GPIO_EMC_19 for Mode: ALT2"]
            pub const GPIO_EMC_19_ALT2: u32 = 0x01;
        }
    }
}
#[doc = "LPI2C2_IPP_IND_LPI2C_SDA_SELECT_INPUT DAISY Register"]
pub mod LPI2C2_IPP_IND_LPI2C_SDA_SELECT_INPUT {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    pub mod DAISY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Selecting Pad: GPIO_EMC_18 for Mode: ALT2"]
            pub const GPIO_EMC_18_ALT2: u32 = 0x01;
        }
    }
}
#[doc = "LPSPI1_IPP_IND_LPSPI_PCS_SELECT_INPUT_0 DAISY Register"]
pub mod LPSPI1_IPP_IND_LPSPI_PCS_SELECT_INPUT_0 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    pub mod DAISY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Selecting Pad: GPIO_AD_B0_11 for Mode: ALT1"]
            pub const GPIO_AD_B0_11_ALT1: u32 = 0x01;
        }
    }
}
#[doc = "LPSPI1_IPP_IND_LPSPI_SCK_SELECT_INPUT DAISY Register"]
pub mod LPSPI1_IPP_IND_LPSPI_SCK_SELECT_INPUT {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    pub mod DAISY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Selecting Pad: GPIO_AD_B0_10 for Mode: ALT1"]
            pub const GPIO_AD_B0_10_ALT1: u32 = 0x01;
        }
    }
}
#[doc = "LPSPI1_IPP_IND_LPSPI_SDI_SELECT_INPUT DAISY Register"]
pub mod LPSPI1_IPP_IND_LPSPI_SDI_SELECT_INPUT {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    pub mod DAISY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Selecting Pad: GPIO_AD_B0_13 for Mode: ALT1"]
            pub const GPIO_AD_B0_13_ALT1: u32 = 0x01;
        }
    }
}
#[doc = "LPSPI1_IPP_IND_LPSPI_SDO_SELECT_INPUT DAISY Register"]
pub mod LPSPI1_IPP_IND_LPSPI_SDO_SELECT_INPUT {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    pub mod DAISY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Selecting Pad: GPIO_AD_B0_12 for Mode: ALT1"]
            pub const GPIO_AD_B0_12_ALT1: u32 = 0x01;
        }
    }
}
#[doc = "LPSPI2_IPP_IND_LPSPI_PCS_SELECT_INPUT_0 DAISY Register"]
pub mod LPSPI2_IPP_IND_LPSPI_PCS_SELECT_INPUT_0 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    pub mod DAISY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Selecting Pad: GPIO_SD_B1_06 for Mode: ALT4"]
            pub const GPIO_SD_B1_06_ALT4: u32 = 0x02;
        }
    }
}
#[doc = "LPSPI2_IPP_IND_LPSPI_SCK_SELECT_INPUT DAISY Register"]
pub mod LPSPI2_IPP_IND_LPSPI_SCK_SELECT_INPUT {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    pub mod DAISY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Selecting Pad: GPIO_SD_B1_07 for Mode: ALT4"]
            pub const GPIO_SD_B1_07_ALT4: u32 = 0x02;
        }
    }
}
#[doc = "LPSPI2_IPP_IND_LPSPI_SDI_SELECT_INPUT DAISY Register"]
pub mod LPSPI2_IPP_IND_LPSPI_SDI_SELECT_INPUT {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    pub mod DAISY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Selecting Pad: GPIO_SD_B1_09 for Mode: ALT4"]
            pub const GPIO_SD_B1_09_ALT4: u32 = 0x02;
        }
    }
}
#[doc = "LPSPI2_IPP_IND_LPSPI_SDO_SELECT_INPUT DAISY Register"]
pub mod LPSPI2_IPP_IND_LPSPI_SDO_SELECT_INPUT {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    pub mod DAISY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Selecting Pad: GPIO_SD_B1_08 for Mode: ALT4"]
            pub const GPIO_SD_B1_08_ALT4: u32 = 0x02;
        }
    }
}
#[doc = "LPUART2_IPP_IND_LPUART_CTS_B_SELECT_INPUT DAISY Register"]
pub mod LPUART2_IPP_IND_LPUART_CTS_B_SELECT_INPUT {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    pub mod DAISY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Selecting Pad: GPIO_EMC_20 for Mode: ALT2"]
            pub const GPIO_EMC_20_ALT2: u32 = 0x01;
        }
    }
}
#[doc = "LPUART2_IPP_IND_LPUART_RXD_SELECT_INPUT DAISY Register"]
pub mod LPUART2_IPP_IND_LPUART_RXD_SELECT_INPUT {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    pub mod DAISY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Selecting Pad: GPIO_EMC_23 for Mode: ALT2"]
            pub const GPIO_EMC_23_ALT2: u32 = 0x01;
        }
    }
}
#[doc = "LPUART2_IPP_IND_LPUART_TXD_SELECT_INPUT DAISY Register"]
pub mod LPUART2_IPP_IND_LPUART_TXD_SELECT_INPUT {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    pub mod DAISY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Selecting Pad: GPIO_EMC_22 for Mode: ALT2"]
            pub const GPIO_EMC_22_ALT2: u32 = 0x01;
        }
    }
}
#[doc = "LPUART3_IPP_IND_LPUART_RXD_SELECT_INPUT DAISY Register"]
pub mod LPUART3_IPP_IND_LPUART_RXD_SELECT_INPUT {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    pub mod DAISY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Selecting Pad: GPIO_EMC_07 for Mode: ALT2"]
            pub const GPIO_EMC_07_ALT2: u32 = 0;
            #[doc = "Selecting Pad: GPIO_AD_B0_15 for Mode: ALT2"]
            pub const GPIO_AD_B0_15_ALT2: u32 = 0x01;
        }
    }
}
#[doc = "LPUART3_IPP_IND_LPUART_TXD_SELECT_INPUT DAISY Register"]
pub mod LPUART3_IPP_IND_LPUART_TXD_SELECT_INPUT {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    pub mod DAISY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Selecting Pad: GPIO_EMC_06 for Mode: ALT2"]
            pub const GPIO_EMC_06_ALT2: u32 = 0;
            #[doc = "Selecting Pad: GPIO_AD_B0_14 for Mode: ALT2"]
            pub const GPIO_AD_B0_14_ALT2: u32 = 0x01;
        }
    }
}
#[doc = "LPUART4_IPP_IND_LPUART_CTS_B_SELECT_INPUT DAISY Register"]
pub mod LPUART4_IPP_IND_LPUART_CTS_B_SELECT_INPUT {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    pub mod DAISY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "LPUART4_IPP_IND_LPUART_RXD_SELECT_INPUT DAISY Register"]
pub mod LPUART4_IPP_IND_LPUART_RXD_SELECT_INPUT {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    pub mod DAISY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Selecting Pad: GPIO_AD_B1_11 for Mode: ALT2"]
            pub const GPIO_AD_B1_11_ALT2: u32 = 0x01;
            #[doc = "Selecting Pad: GPIO_EMC_33 for Mode: ALT2"]
            pub const GPIO_EMC_33_ALT2: u32 = 0x02;
        }
    }
}
#[doc = "LPUART4_IPP_IND_LPUART_TXD_SELECT_INPUT DAISY Register"]
pub mod LPUART4_IPP_IND_LPUART_TXD_SELECT_INPUT {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    pub mod DAISY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Selecting Pad: GPIO_AD_B1_10 for Mode: ALT2"]
            pub const GPIO_AD_B1_10_ALT2: u32 = 0x01;
            #[doc = "Selecting Pad: GPIO_EMC_32 for Mode: ALT2"]
            pub const GPIO_EMC_32_ALT2: u32 = 0x02;
        }
    }
}
#[doc = "NMI_GLUE_IPP_IND_NMI_SELECT_INPUT DAISY Register"]
pub mod NMI_GLUE_IPP_IND_NMI_SELECT_INPUT {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    pub mod DAISY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Selecting Pad: GPIO_AD_B0_05 for Mode: ALT7"]
            pub const GPIO_AD_B0_05_ALT7: u32 = 0;
            #[doc = "Selecting Pad: WAKEUP for Mode: ALT7"]
            pub const WAKEUP_ALT7: u32 = 0x01;
        }
    }
}
#[doc = "QTIMER1_TMR0_INPUT_SELECT_INPUT DAISY Register"]
pub mod QTIMER1_TMR0_INPUT_SELECT_INPUT {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    pub mod DAISY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Selecting Pad: GPIO_EMC_32 for Mode: ALT1"]
            pub const GPIO_EMC_32_ALT1: u32 = 0x01;
        }
    }
}
#[doc = "QTIMER1_TMR1_INPUT_SELECT_INPUT DAISY Register"]
pub mod QTIMER1_TMR1_INPUT_SELECT_INPUT {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    pub mod DAISY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Selecting Pad: GPIO_EMC_33 for Mode: ALT1"]
            pub const GPIO_EMC_33_ALT1: u32 = 0x01;
        }
    }
}
#[doc = "QTIMER1_TMR2_INPUT_SELECT_INPUT DAISY Register"]
pub mod QTIMER1_TMR2_INPUT_SELECT_INPUT {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    pub mod DAISY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Selecting Pad: GPIO_EMC_34 for Mode: ALT1"]
            pub const GPIO_EMC_34_ALT1: u32 = 0x01;
        }
    }
}
#[doc = "QTIMER1_TMR3_INPUT_SELECT_INPUT DAISY Register"]
pub mod QTIMER1_TMR3_INPUT_SELECT_INPUT {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    pub mod DAISY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Selecting Pad: GPIO_EMC_35 for Mode: ALT1"]
            pub const GPIO_EMC_35_ALT1: u32 = 0x01;
        }
    }
}
#[doc = "SAI1_IPG_CLK_SAI_MCLK_SELECT_INPUT_2 DAISY Register"]
pub mod SAI1_IPG_CLK_SAI_MCLK_SELECT_INPUT_2 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    pub mod DAISY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Selecting Pad: GPIO_AD_B0_03 for Mode: ALT3"]
            pub const GPIO_AD_B0_03_ALT3: u32 = 0x01;
            #[doc = "Selecting Pad: GPIO_EMC_20 for Mode: ALT3"]
            pub const GPIO_EMC_20_ALT3: u32 = 0x03;
        }
    }
}
#[doc = "SAI1_IPP_IND_SAI_RXBCLK_SELECT_INPUT DAISY Register"]
pub mod SAI1_IPP_IND_SAI_RXBCLK_SELECT_INPUT {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    pub mod DAISY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Selecting Pad: GPIO_EMC_19 for Mode: ALT3"]
            pub const GPIO_EMC_19_ALT3: u32 = 0x02;
        }
    }
}
#[doc = "SAI1_IPP_IND_SAI_RXDATA_SELECT_INPUT_0 DAISY Register"]
pub mod SAI1_IPP_IND_SAI_RXDATA_SELECT_INPUT_0 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    pub mod DAISY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Selecting Pad: GPIO_EMC_21 for Mode: ALT3"]
            pub const GPIO_EMC_21_ALT3: u32 = 0x02;
        }
    }
}
#[doc = "SAI1_IPP_IND_SAI_RXDATA_SELECT_INPUT_1 DAISY Register"]
pub mod SAI1_IPP_IND_SAI_RXDATA_SELECT_INPUT_1 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    pub mod DAISY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Selecting Pad: GPIO_EMC_22 for Mode: ALT3"]
            pub const GPIO_EMC_22_ALT3: u32 = 0x01;
        }
    }
}
#[doc = "SAI1_IPP_IND_SAI_RXDATA_SELECT_INPUT_2 DAISY Register"]
pub mod SAI1_IPP_IND_SAI_RXDATA_SELECT_INPUT_2 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    pub mod DAISY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Selecting Pad: GPIO_EMC_23 for Mode: ALT3"]
            pub const GPIO_EMC_23_ALT3: u32 = 0x01;
        }
    }
}
#[doc = "SAI1_IPP_IND_SAI_RXDATA_SELECT_INPUT_3 DAISY Register"]
pub mod SAI1_IPP_IND_SAI_RXDATA_SELECT_INPUT_3 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    pub mod DAISY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Selecting Pad: GPIO_EMC_24 for Mode: ALT3"]
            pub const GPIO_EMC_24_ALT3: u32 = 0x01;
        }
    }
}
#[doc = "SAI1_IPP_IND_SAI_RXSYNC_SELECT_INPUT DAISY Register"]
pub mod SAI1_IPP_IND_SAI_RXSYNC_SELECT_INPUT {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    pub mod DAISY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Selecting Pad: GPIO_EMC_18 for Mode: ALT3"]
            pub const GPIO_EMC_18_ALT3: u32 = 0x02;
        }
    }
}
#[doc = "SAI1_IPP_IND_SAI_TXBCLK_SELECT_INPUT DAISY Register"]
pub mod SAI1_IPP_IND_SAI_TXBCLK_SELECT_INPUT {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    pub mod DAISY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Selecting Pad: GPIO_EMC_26 for Mode: ALT3"]
            pub const GPIO_EMC_26_ALT3: u32 = 0x02;
        }
    }
}
#[doc = "SAI1_IPP_IND_SAI_TXSYNC_SELECT_INPUT DAISY Register"]
pub mod SAI1_IPP_IND_SAI_TXSYNC_SELECT_INPUT {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    pub mod DAISY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Selecting Pad: GPIO_EMC_27 for Mode: ALT3"]
            pub const GPIO_EMC_27_ALT3: u32 = 0x02;
        }
    }
}
#[doc = "SAI2_IPG_CLK_SAI_MCLK_SELECT_INPUT_2 DAISY Register"]
pub mod SAI2_IPG_CLK_SAI_MCLK_SELECT_INPUT_2 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    pub mod DAISY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Selecting Pad: GPIO_EMC_16 for Mode: ALT3"]
            pub const GPIO_EMC_16_ALT3: u32 = 0x01;
        }
    }
}
#[doc = "SAI2_IPP_IND_SAI_RXBCLK_SELECT_INPUT DAISY Register"]
pub mod SAI2_IPP_IND_SAI_RXBCLK_SELECT_INPUT {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    pub mod DAISY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Selecting Pad: GPIO_EMC_09 for Mode: ALT3"]
            pub const GPIO_EMC_09_ALT3: u32 = 0x01;
        }
    }
}
#[doc = "SAI2_IPP_IND_SAI_RXDATA_SELECT_INPUT_0 DAISY Register"]
pub mod SAI2_IPP_IND_SAI_RXDATA_SELECT_INPUT_0 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    pub mod DAISY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Selecting Pad: GPIO_EMC_08 for Mode: ALT3"]
            pub const GPIO_EMC_08_ALT3: u32 = 0x01;
        }
    }
}
#[doc = "SAI2_IPP_IND_SAI_RXSYNC_SELECT_INPUT DAISY Register"]
pub mod SAI2_IPP_IND_SAI_RXSYNC_SELECT_INPUT {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    pub mod DAISY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Selecting Pad: GPIO_EMC_07 for Mode: ALT3"]
            pub const GPIO_EMC_07_ALT3: u32 = 0x01;
        }
    }
}
#[doc = "SAI2_IPP_IND_SAI_TXBCLK_SELECT_INPUT DAISY Register"]
pub mod SAI2_IPP_IND_SAI_TXBCLK_SELECT_INPUT {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    pub mod DAISY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Selecting Pad: GPIO_EMC_04 for Mode: ALT3"]
            pub const GPIO_EMC_04_ALT3: u32 = 0x01;
        }
    }
}
#[doc = "SAI2_IPP_IND_SAI_TXSYNC_SELECT_INPUT DAISY Register"]
pub mod SAI2_IPP_IND_SAI_TXSYNC_SELECT_INPUT {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    pub mod DAISY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Selecting Pad: GPIO_EMC_05 for Mode: ALT3"]
            pub const GPIO_EMC_05_ALT3: u32 = 0x01;
        }
    }
}
#[doc = "SAI3_IPG_CLK_SAI_MCLK_SELECT_INPUT_2 DAISY Register"]
pub mod SAI3_IPG_CLK_SAI_MCLK_SELECT_INPUT_2 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    pub mod DAISY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Selecting Pad: GPIO_SD_B1_05 for Mode: ALT3"]
            pub const GPIO_SD_B1_05_ALT3: u32 = 0;
            #[doc = "Selecting Pad: GPIO_EMC_17 for Mode: ALT3"]
            pub const GPIO_EMC_17_ALT3: u32 = 0x01;
        }
    }
}
#[doc = "SAI3_IPP_IND_SAI_RXBCLK_SELECT_INPUT DAISY Register"]
pub mod SAI3_IPP_IND_SAI_RXBCLK_SELECT_INPUT {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    pub mod DAISY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Selecting Pad: GPIO_SD_B1_09 for Mode: ALT3"]
            pub const GPIO_SD_B1_09_ALT3: u32 = 0;
        }
    }
}
#[doc = "SAI3_IPP_IND_SAI_RXDATA_SELECT_INPUT_0 DAISY Register"]
pub mod SAI3_IPP_IND_SAI_RXDATA_SELECT_INPUT_0 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    pub mod DAISY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Selecting Pad: GPIO_SD_B1_11 for Mode: ALT3"]
            pub const GPIO_SD_B1_11_ALT3: u32 = 0;
        }
    }
}
#[doc = "SAI3_IPP_IND_SAI_RXSYNC_SELECT_INPUT DAISY Register"]
pub mod SAI3_IPP_IND_SAI_RXSYNC_SELECT_INPUT {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    pub mod DAISY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Selecting Pad: GPIO_SD_B1_10 for Mode: ALT3"]
            pub const GPIO_SD_B1_10_ALT3: u32 = 0;
        }
    }
}
#[doc = "SAI3_IPP_IND_SAI_TXBCLK_SELECT_INPUT DAISY Register"]
pub mod SAI3_IPP_IND_SAI_TXBCLK_SELECT_INPUT {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    pub mod DAISY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Selecting Pad: GPIO_SD_B1_06 for Mode: ALT3"]
            pub const GPIO_SD_B1_06_ALT3: u32 = 0;
            #[doc = "Selecting Pad: GPIO_EMC_33 for Mode: ALT3"]
            pub const GPIO_EMC_33_ALT3: u32 = 0x01;
        }
    }
}
#[doc = "SAI3_IPP_IND_SAI_TXSYNC_SELECT_INPUT DAISY Register"]
pub mod SAI3_IPP_IND_SAI_TXSYNC_SELECT_INPUT {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    pub mod DAISY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Selecting Pad: GPIO_SD_B1_07 for Mode: ALT3"]
            pub const GPIO_SD_B1_07_ALT3: u32 = 0;
            #[doc = "Selecting Pad: GPIO_EMC_34 for Mode: ALT3"]
            pub const GPIO_EMC_34_ALT3: u32 = 0x01;
        }
    }
}
#[doc = "SPDIF_SPDIF_IN1_SELECT_INPUT DAISY Register"]
pub mod SPDIF_SPDIF_IN1_SELECT_INPUT {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    pub mod DAISY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Selecting Pad: GPIO_EMC_05 for Mode: ALT2"]
            pub const GPIO_EMC_05_ALT2: u32 = 0;
        }
    }
}
#[doc = "USB_IPP_IND_OTG_OC_SELECT_INPUT DAISY Register"]
pub mod USB_IPP_IND_OTG_OC_SELECT_INPUT {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    pub mod DAISY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Selecting Pad: GPIO_AD_B0_03 for Mode: ALT6"]
            pub const GPIO_AD_B0_03_ALT6: u32 = 0;
            #[doc = "Selecting Pad: GPIO_AD_B1_12 for Mode: ALT0"]
            pub const GPIO_AD_B1_12_ALT0: u32 = 0x01;
        }
    }
}
#[doc = "XBAR1_XBAR_IN_SELECT_INPUT_14 DAISY Register"]
pub mod XBAR1_XBAR_IN_SELECT_INPUT_14 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    pub mod DAISY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "XBAR1_XBAR_IN_SELECT_INPUT_15 DAISY Register"]
pub mod XBAR1_XBAR_IN_SELECT_INPUT_15 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    pub mod DAISY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "XBAR1_XBAR_IN_SELECT_INPUT_16 DAISY Register"]
pub mod XBAR1_XBAR_IN_SELECT_INPUT_16 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    pub mod DAISY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Selecting Pad: GPIO_EMC_18 for Mode: ALT1"]
            pub const GPIO_EMC_18_ALT1: u32 = 0x01;
        }
    }
}
#[doc = "XBAR1_XBAR_IN_SELECT_INPUT_17 DAISY Register"]
pub mod XBAR1_XBAR_IN_SELECT_INPUT_17 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    pub mod DAISY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Selecting Pad: GPIO_EMC_19 for Mode: ALT1"]
            pub const GPIO_EMC_19_ALT1: u32 = 0x01;
        }
    }
}
#[doc = "XBAR1_XBAR_IN_SELECT_INPUT_10 DAISY Register"]
pub mod XBAR1_XBAR_IN_SELECT_INPUT_10 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    pub mod DAISY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Selecting Pad: GPIO_SD_B1_00 for Mode: ALT3"]
            pub const GPIO_SD_B1_00_ALT3: u32 = 0x01;
        }
    }
}
#[doc = "XBAR1_XBAR_IN_SELECT_INPUT_12 DAISY Register"]
pub mod XBAR1_XBAR_IN_SELECT_INPUT_12 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    pub mod DAISY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "XBAR1_XBAR_IN_SELECT_INPUT_13 DAISY Register"]
pub mod XBAR1_XBAR_IN_SELECT_INPUT_13 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    pub mod DAISY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "XBAR1_XBAR_IN_SELECT_INPUT_18 DAISY Register"]
pub mod XBAR1_XBAR_IN_SELECT_INPUT_18 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    pub mod DAISY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "XBAR1_XBAR_IN_SELECT_INPUT_19 DAISY Register"]
pub mod XBAR1_XBAR_IN_SELECT_INPUT_19 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    pub mod DAISY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
