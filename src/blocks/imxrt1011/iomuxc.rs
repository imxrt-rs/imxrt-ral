#[doc = "IOMUXC"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x10],
    #[doc = "SW_MUX_CTL_PAD_GPIO_AD_14 SW MUX Control Register"]
    pub SW_MUX_CTL_PAD_GPIO_AD_14: crate::RWRegister<u32>,
    #[doc = "SW_MUX_CTL_PAD_GPIO_AD_13 SW MUX Control Register"]
    pub SW_MUX_CTL_PAD_GPIO_AD_13: crate::RWRegister<u32>,
    #[doc = "SW_MUX_CTL_PAD_GPIO_AD_12 SW MUX Control Register"]
    pub SW_MUX_CTL_PAD_GPIO_AD_12: crate::RWRegister<u32>,
    #[doc = "SW_MUX_CTL_PAD_GPIO_AD_11 SW MUX Control Register"]
    pub SW_MUX_CTL_PAD_GPIO_AD_11: crate::RWRegister<u32>,
    #[doc = "SW_MUX_CTL_PAD_GPIO_AD_10 SW MUX Control Register"]
    pub SW_MUX_CTL_PAD_GPIO_AD_10: crate::RWRegister<u32>,
    #[doc = "SW_MUX_CTL_PAD_GPIO_AD_09 SW MUX Control Register"]
    pub SW_MUX_CTL_PAD_GPIO_AD_09: crate::RWRegister<u32>,
    #[doc = "SW_MUX_CTL_PAD_GPIO_AD_08 SW MUX Control Register"]
    pub SW_MUX_CTL_PAD_GPIO_AD_08: crate::RWRegister<u32>,
    #[doc = "SW_MUX_CTL_PAD_GPIO_AD_07 SW MUX Control Register"]
    pub SW_MUX_CTL_PAD_GPIO_AD_07: crate::RWRegister<u32>,
    #[doc = "SW_MUX_CTL_PAD_GPIO_AD_06 SW MUX Control Register"]
    pub SW_MUX_CTL_PAD_GPIO_AD_06: crate::RWRegister<u32>,
    #[doc = "SW_MUX_CTL_PAD_GPIO_AD_05 SW MUX Control Register"]
    pub SW_MUX_CTL_PAD_GPIO_AD_05: crate::RWRegister<u32>,
    #[doc = "SW_MUX_CTL_PAD_GPIO_AD_04 SW MUX Control Register"]
    pub SW_MUX_CTL_PAD_GPIO_AD_04: crate::RWRegister<u32>,
    #[doc = "SW_MUX_CTL_PAD_GPIO_AD_03 SW MUX Control Register"]
    pub SW_MUX_CTL_PAD_GPIO_AD_03: crate::RWRegister<u32>,
    #[doc = "SW_MUX_CTL_PAD_GPIO_AD_02 SW MUX Control Register"]
    pub SW_MUX_CTL_PAD_GPIO_AD_02: crate::RWRegister<u32>,
    #[doc = "SW_MUX_CTL_PAD_GPIO_AD_01 SW MUX Control Register"]
    pub SW_MUX_CTL_PAD_GPIO_AD_01: crate::RWRegister<u32>,
    #[doc = "SW_MUX_CTL_PAD_GPIO_AD_00 SW MUX Control Register"]
    pub SW_MUX_CTL_PAD_GPIO_AD_00: crate::RWRegister<u32>,
    #[doc = "SW_MUX_CTL_PAD_GPIO_SD_14 SW MUX Control Register"]
    pub SW_MUX_CTL_PAD_GPIO_SD_14: crate::RWRegister<u32>,
    #[doc = "SW_MUX_CTL_PAD_GPIO_SD_13 SW MUX Control Register"]
    pub SW_MUX_CTL_PAD_GPIO_SD_13: crate::RWRegister<u32>,
    #[doc = "SW_MUX_CTL_PAD_GPIO_SD_12 SW MUX Control Register"]
    pub SW_MUX_CTL_PAD_GPIO_SD_12: crate::RWRegister<u32>,
    #[doc = "SW_MUX_CTL_PAD_GPIO_SD_11 SW MUX Control Register"]
    pub SW_MUX_CTL_PAD_GPIO_SD_11: crate::RWRegister<u32>,
    #[doc = "SW_MUX_CTL_PAD_GPIO_SD_10 SW MUX Control Register"]
    pub SW_MUX_CTL_PAD_GPIO_SD_10: crate::RWRegister<u32>,
    #[doc = "SW_MUX_CTL_PAD_GPIO_SD_09 SW MUX Control Register"]
    pub SW_MUX_CTL_PAD_GPIO_SD_09: crate::RWRegister<u32>,
    #[doc = "SW_MUX_CTL_PAD_GPIO_SD_08 SW MUX Control Register"]
    pub SW_MUX_CTL_PAD_GPIO_SD_08: crate::RWRegister<u32>,
    #[doc = "SW_MUX_CTL_PAD_GPIO_SD_07 SW MUX Control Register"]
    pub SW_MUX_CTL_PAD_GPIO_SD_07: crate::RWRegister<u32>,
    #[doc = "SW_MUX_CTL_PAD_GPIO_SD_06 SW MUX Control Register"]
    pub SW_MUX_CTL_PAD_GPIO_SD_06: crate::RWRegister<u32>,
    #[doc = "SW_MUX_CTL_PAD_GPIO_SD_05 SW MUX Control Register"]
    pub SW_MUX_CTL_PAD_GPIO_SD_05: crate::RWRegister<u32>,
    #[doc = "SW_MUX_CTL_PAD_GPIO_SD_04 SW MUX Control Register"]
    pub SW_MUX_CTL_PAD_GPIO_SD_04: crate::RWRegister<u32>,
    #[doc = "SW_MUX_CTL_PAD_GPIO_SD_03 SW MUX Control Register"]
    pub SW_MUX_CTL_PAD_GPIO_SD_03: crate::RWRegister<u32>,
    #[doc = "SW_MUX_CTL_PAD_GPIO_SD_02 SW MUX Control Register"]
    pub SW_MUX_CTL_PAD_GPIO_SD_02: crate::RWRegister<u32>,
    #[doc = "SW_MUX_CTL_PAD_GPIO_SD_01 SW MUX Control Register"]
    pub SW_MUX_CTL_PAD_GPIO_SD_01: crate::RWRegister<u32>,
    #[doc = "SW_MUX_CTL_PAD_GPIO_SD_00 SW MUX Control Register"]
    pub SW_MUX_CTL_PAD_GPIO_SD_00: crate::RWRegister<u32>,
    #[doc = "SW_MUX_CTL_PAD_GPIO_13 SW MUX Control Register"]
    pub SW_MUX_CTL_PAD_GPIO_13: crate::RWRegister<u32>,
    #[doc = "SW_MUX_CTL_PAD_GPIO_12 SW MUX Control Register"]
    pub SW_MUX_CTL_PAD_GPIO_12: crate::RWRegister<u32>,
    #[doc = "SW_MUX_CTL_PAD_GPIO_11 SW MUX Control Register"]
    pub SW_MUX_CTL_PAD_GPIO_11: crate::RWRegister<u32>,
    #[doc = "SW_MUX_CTL_PAD_GPIO_10 SW MUX Control Register"]
    pub SW_MUX_CTL_PAD_GPIO_10: crate::RWRegister<u32>,
    #[doc = "SW_MUX_CTL_PAD_GPIO_09 SW MUX Control Register"]
    pub SW_MUX_CTL_PAD_GPIO_09: crate::RWRegister<u32>,
    #[doc = "SW_MUX_CTL_PAD_GPIO_08 SW MUX Control Register"]
    pub SW_MUX_CTL_PAD_GPIO_08: crate::RWRegister<u32>,
    #[doc = "SW_MUX_CTL_PAD_GPIO_07 SW MUX Control Register"]
    pub SW_MUX_CTL_PAD_GPIO_07: crate::RWRegister<u32>,
    #[doc = "SW_MUX_CTL_PAD_GPIO_06 SW MUX Control Register"]
    pub SW_MUX_CTL_PAD_GPIO_06: crate::RWRegister<u32>,
    #[doc = "SW_MUX_CTL_PAD_GPIO_05 SW MUX Control Register"]
    pub SW_MUX_CTL_PAD_GPIO_05: crate::RWRegister<u32>,
    #[doc = "SW_MUX_CTL_PAD_GPIO_04 SW MUX Control Register"]
    pub SW_MUX_CTL_PAD_GPIO_04: crate::RWRegister<u32>,
    #[doc = "SW_MUX_CTL_PAD_GPIO_03 SW MUX Control Register"]
    pub SW_MUX_CTL_PAD_GPIO_03: crate::RWRegister<u32>,
    #[doc = "SW_MUX_CTL_PAD_GPIO_02 SW MUX Control Register"]
    pub SW_MUX_CTL_PAD_GPIO_02: crate::RWRegister<u32>,
    #[doc = "SW_MUX_CTL_PAD_GPIO_01 SW MUX Control Register"]
    pub SW_MUX_CTL_PAD_GPIO_01: crate::RWRegister<u32>,
    #[doc = "SW_MUX_CTL_PAD_GPIO_00 SW MUX Control Register"]
    pub SW_MUX_CTL_PAD_GPIO_00: crate::RWRegister<u32>,
    #[doc = "SW_PAD_CTL_PAD_GPIO_AD_14 SW PAD Control Register"]
    pub SW_PAD_CTL_PAD_GPIO_AD_14: crate::RWRegister<u32>,
    #[doc = "SW_PAD_CTL_PAD_GPIO_AD_13 SW PAD Control Register"]
    pub SW_PAD_CTL_PAD_GPIO_AD_13: crate::RWRegister<u32>,
    #[doc = "SW_PAD_CTL_PAD_GPIO_AD_12 SW PAD Control Register"]
    pub SW_PAD_CTL_PAD_GPIO_AD_12: crate::RWRegister<u32>,
    #[doc = "SW_PAD_CTL_PAD_GPIO_AD_11 SW PAD Control Register"]
    pub SW_PAD_CTL_PAD_GPIO_AD_11: crate::RWRegister<u32>,
    #[doc = "SW_PAD_CTL_PAD_GPIO_AD_10 SW PAD Control Register"]
    pub SW_PAD_CTL_PAD_GPIO_AD_10: crate::RWRegister<u32>,
    #[doc = "SW_PAD_CTL_PAD_GPIO_AD_09 SW PAD Control Register"]
    pub SW_PAD_CTL_PAD_GPIO_AD_09: crate::RWRegister<u32>,
    #[doc = "SW_PAD_CTL_PAD_GPIO_AD_08 SW PAD Control Register"]
    pub SW_PAD_CTL_PAD_GPIO_AD_08: crate::RWRegister<u32>,
    #[doc = "SW_PAD_CTL_PAD_GPIO_AD_07 SW PAD Control Register"]
    pub SW_PAD_CTL_PAD_GPIO_AD_07: crate::RWRegister<u32>,
    #[doc = "SW_PAD_CTL_PAD_GPIO_AD_06 SW PAD Control Register"]
    pub SW_PAD_CTL_PAD_GPIO_AD_06: crate::RWRegister<u32>,
    #[doc = "SW_PAD_CTL_PAD_GPIO_AD_05 SW PAD Control Register"]
    pub SW_PAD_CTL_PAD_GPIO_AD_05: crate::RWRegister<u32>,
    #[doc = "SW_PAD_CTL_PAD_GPIO_AD_04 SW PAD Control Register"]
    pub SW_PAD_CTL_PAD_GPIO_AD_04: crate::RWRegister<u32>,
    #[doc = "SW_PAD_CTL_PAD_GPIO_AD_03 SW PAD Control Register"]
    pub SW_PAD_CTL_PAD_GPIO_AD_03: crate::RWRegister<u32>,
    #[doc = "SW_PAD_CTL_PAD_GPIO_AD_02 SW PAD Control Register"]
    pub SW_PAD_CTL_PAD_GPIO_AD_02: crate::RWRegister<u32>,
    #[doc = "SW_PAD_CTL_PAD_GPIO_AD_01 SW PAD Control Register"]
    pub SW_PAD_CTL_PAD_GPIO_AD_01: crate::RWRegister<u32>,
    #[doc = "SW_PAD_CTL_PAD_GPIO_AD_00 SW PAD Control Register"]
    pub SW_PAD_CTL_PAD_GPIO_AD_00: crate::RWRegister<u32>,
    #[doc = "SW_PAD_CTL_PAD_GPIO_SD_14 SW PAD Control Register"]
    pub SW_PAD_CTL_PAD_GPIO_SD_14: crate::RWRegister<u32>,
    #[doc = "SW_PAD_CTL_PAD_GPIO_SD_13 SW PAD Control Register"]
    pub SW_PAD_CTL_PAD_GPIO_SD_13: crate::RWRegister<u32>,
    #[doc = "SW_PAD_CTL_PAD_GPIO_SD_12 SW PAD Control Register"]
    pub SW_PAD_CTL_PAD_GPIO_SD_12: crate::RWRegister<u32>,
    #[doc = "SW_PAD_CTL_PAD_GPIO_SD_11 SW PAD Control Register"]
    pub SW_PAD_CTL_PAD_GPIO_SD_11: crate::RWRegister<u32>,
    #[doc = "SW_PAD_CTL_PAD_GPIO_SD_10 SW PAD Control Register"]
    pub SW_PAD_CTL_PAD_GPIO_SD_10: crate::RWRegister<u32>,
    #[doc = "SW_PAD_CTL_PAD_GPIO_SD_09 SW PAD Control Register"]
    pub SW_PAD_CTL_PAD_GPIO_SD_09: crate::RWRegister<u32>,
    #[doc = "SW_PAD_CTL_PAD_GPIO_SD_08 SW PAD Control Register"]
    pub SW_PAD_CTL_PAD_GPIO_SD_08: crate::RWRegister<u32>,
    #[doc = "SW_PAD_CTL_PAD_GPIO_SD_07 SW PAD Control Register"]
    pub SW_PAD_CTL_PAD_GPIO_SD_07: crate::RWRegister<u32>,
    #[doc = "SW_PAD_CTL_PAD_GPIO_SD_06 SW PAD Control Register"]
    pub SW_PAD_CTL_PAD_GPIO_SD_06: crate::RWRegister<u32>,
    #[doc = "SW_PAD_CTL_PAD_GPIO_SD_05 SW PAD Control Register"]
    pub SW_PAD_CTL_PAD_GPIO_SD_05: crate::RWRegister<u32>,
    #[doc = "SW_PAD_CTL_PAD_GPIO_SD_04 SW PAD Control Register"]
    pub SW_PAD_CTL_PAD_GPIO_SD_04: crate::RWRegister<u32>,
    #[doc = "SW_PAD_CTL_PAD_GPIO_SD_03 SW PAD Control Register"]
    pub SW_PAD_CTL_PAD_GPIO_SD_03: crate::RWRegister<u32>,
    #[doc = "SW_PAD_CTL_PAD_GPIO_SD_02 SW PAD Control Register"]
    pub SW_PAD_CTL_PAD_GPIO_SD_02: crate::RWRegister<u32>,
    #[doc = "SW_PAD_CTL_PAD_GPIO_SD_01 SW PAD Control Register"]
    pub SW_PAD_CTL_PAD_GPIO_SD_01: crate::RWRegister<u32>,
    #[doc = "SW_PAD_CTL_PAD_GPIO_SD_00 SW PAD Control Register"]
    pub SW_PAD_CTL_PAD_GPIO_SD_00: crate::RWRegister<u32>,
    #[doc = "SW_PAD_CTL_PAD_GPIO_13 SW PAD Control Register"]
    pub SW_PAD_CTL_PAD_GPIO_13: crate::RWRegister<u32>,
    #[doc = "SW_PAD_CTL_PAD_GPIO_12 SW PAD Control Register"]
    pub SW_PAD_CTL_PAD_GPIO_12: crate::RWRegister<u32>,
    #[doc = "SW_PAD_CTL_PAD_GPIO_11 SW PAD Control Register"]
    pub SW_PAD_CTL_PAD_GPIO_11: crate::RWRegister<u32>,
    #[doc = "SW_PAD_CTL_PAD_GPIO_10 SW PAD Control Register"]
    pub SW_PAD_CTL_PAD_GPIO_10: crate::RWRegister<u32>,
    #[doc = "SW_PAD_CTL_PAD_GPIO_09 SW PAD Control Register"]
    pub SW_PAD_CTL_PAD_GPIO_09: crate::RWRegister<u32>,
    #[doc = "SW_PAD_CTL_PAD_GPIO_08 SW PAD Control Register"]
    pub SW_PAD_CTL_PAD_GPIO_08: crate::RWRegister<u32>,
    #[doc = "SW_PAD_CTL_PAD_GPIO_07 SW PAD Control Register"]
    pub SW_PAD_CTL_PAD_GPIO_07: crate::RWRegister<u32>,
    #[doc = "SW_PAD_CTL_PAD_GPIO_06 SW PAD Control Register"]
    pub SW_PAD_CTL_PAD_GPIO_06: crate::RWRegister<u32>,
    #[doc = "SW_PAD_CTL_PAD_GPIO_05 SW PAD Control Register"]
    pub SW_PAD_CTL_PAD_GPIO_05: crate::RWRegister<u32>,
    #[doc = "SW_PAD_CTL_PAD_GPIO_04 SW PAD Control Register"]
    pub SW_PAD_CTL_PAD_GPIO_04: crate::RWRegister<u32>,
    #[doc = "SW_PAD_CTL_PAD_GPIO_03 SW PAD Control Register"]
    pub SW_PAD_CTL_PAD_GPIO_03: crate::RWRegister<u32>,
    #[doc = "SW_PAD_CTL_PAD_GPIO_02 SW PAD Control Register"]
    pub SW_PAD_CTL_PAD_GPIO_02: crate::RWRegister<u32>,
    #[doc = "SW_PAD_CTL_PAD_GPIO_01 SW PAD Control Register"]
    pub SW_PAD_CTL_PAD_GPIO_01: crate::RWRegister<u32>,
    #[doc = "SW_PAD_CTL_PAD_GPIO_00 SW PAD Control Register"]
    pub SW_PAD_CTL_PAD_GPIO_00: crate::RWRegister<u32>,
    #[doc = "USB_OTG_ID_SELECT_INPUT DAISY Register"]
    pub USB_OTG_ID_SELECT_INPUT: crate::RWRegister<u32>,
    #[doc = "FLEXPWM1_PWMA_SELECT_INPUT_0 DAISY Register"]
    pub FLEXPWM1_PWMA_SELECT_INPUT_0: crate::RWRegister<u32>,
    #[doc = "FLEXPWM1_PWMA_SELECT_INPUT_1 DAISY Register"]
    pub FLEXPWM1_PWMA_SELECT_INPUT_1: crate::RWRegister<u32>,
    #[doc = "FLEXPWM1_PWMA_SELECT_INPUT_2 DAISY Register"]
    pub FLEXPWM1_PWMA_SELECT_INPUT_2: crate::RWRegister<u32>,
    #[doc = "FLEXPWM1_PWMA_SELECT_INPUT_3 DAISY Register"]
    pub FLEXPWM1_PWMA_SELECT_INPUT_3: crate::RWRegister<u32>,
    #[doc = "FLEXPWM1_PWMB_SELECT_INPUT_0 DAISY Register"]
    pub FLEXPWM1_PWMB_SELECT_INPUT_0: crate::RWRegister<u32>,
    #[doc = "FLEXPWM1_PWMB_SELECT_INPUT_1 DAISY Register"]
    pub FLEXPWM1_PWMB_SELECT_INPUT_1: crate::RWRegister<u32>,
    #[doc = "FLEXPWM1_PWMB_SELECT_INPUT_2 DAISY Register"]
    pub FLEXPWM1_PWMB_SELECT_INPUT_2: crate::RWRegister<u32>,
    #[doc = "FLEXPWM1_PWMB_SELECT_INPUT_3 DAISY Register"]
    pub FLEXPWM1_PWMB_SELECT_INPUT_3: crate::RWRegister<u32>,
    #[doc = "FLEXSPI_DQS_FA_SELECT_INPUT DAISY Register"]
    pub FLEXSPI_DQS_FA_SELECT_INPUT: crate::RWRegister<u32>,
    #[doc = "FLEXSPI_DQS_FB_SELECT_INPUT DAISY Register"]
    pub FLEXSPI_DQS_FB_SELECT_INPUT: crate::RWRegister<u32>,
    #[doc = "KPP_COL_SELECT_INPUT_0 DAISY Register"]
    pub KPP_COL_SELECT_INPUT_0: crate::RWRegister<u32>,
    #[doc = "KPP_COL_SELECT_INPUT_1 DAISY Register"]
    pub KPP_COL_SELECT_INPUT_1: crate::RWRegister<u32>,
    #[doc = "KPP_COL_SELECT_INPUT_2 DAISY Register"]
    pub KPP_COL_SELECT_INPUT_2: crate::RWRegister<u32>,
    #[doc = "KPP_COL_SELECT_INPUT_3 DAISY Register"]
    pub KPP_COL_SELECT_INPUT_3: crate::RWRegister<u32>,
    #[doc = "KPP_ROW_SELECT_INPUT_0 DAISY Register"]
    pub KPP_ROW_SELECT_INPUT_0: crate::RWRegister<u32>,
    #[doc = "KPP_ROW_SELECT_INPUT_1 DAISY Register"]
    pub KPP_ROW_SELECT_INPUT_1: crate::RWRegister<u32>,
    #[doc = "KPP_ROW_SELECT_INPUT_2 DAISY Register"]
    pub KPP_ROW_SELECT_INPUT_2: crate::RWRegister<u32>,
    #[doc = "KPP_ROW_SELECT_INPUT_3 DAISY Register"]
    pub KPP_ROW_SELECT_INPUT_3: crate::RWRegister<u32>,
    #[doc = "LPI2C1_HREQ_SELECT_INPUT DAISY Register"]
    pub LPI2C1_HREQ_SELECT_INPUT: crate::RWRegister<u32>,
    #[doc = "LPI2C1_SCL_SELECT_INPUT DAISY Register"]
    pub LPI2C1_SCL_SELECT_INPUT: crate::RWRegister<u32>,
    #[doc = "LPI2C1_SDA_SELECT_INPUT DAISY Register"]
    pub LPI2C1_SDA_SELECT_INPUT: crate::RWRegister<u32>,
    #[doc = "LPI2C2_SCL_SELECT_INPUT DAISY Register"]
    pub LPI2C2_SCL_SELECT_INPUT: crate::RWRegister<u32>,
    #[doc = "LPI2C2_SDA_SELECT_INPUT DAISY Register"]
    pub LPI2C2_SDA_SELECT_INPUT: crate::RWRegister<u32>,
    #[doc = "LPSPI1_PCS_SELECT_INPUT_0 DAISY Register"]
    pub LPSPI1_PCS_SELECT_INPUT_0: crate::RWRegister<u32>,
    #[doc = "LPSPI1_SCK_SELECT_INPUT DAISY Register"]
    pub LPSPI1_SCK_SELECT_INPUT: crate::RWRegister<u32>,
    #[doc = "LPSPI1_SDI_SELECT_INPUT DAISY Register"]
    pub LPSPI1_SDI_SELECT_INPUT: crate::RWRegister<u32>,
    #[doc = "LPSPI1_SDO_SELECT_INPUT DAISY Register"]
    pub LPSPI1_SDO_SELECT_INPUT: crate::RWRegister<u32>,
    #[doc = "LPSPI2_PCS_SELECT_INPUT_0 DAISY Register"]
    pub LPSPI2_PCS_SELECT_INPUT_0: crate::RWRegister<u32>,
    #[doc = "LPSPI2_SCK_SELECT_INPUT DAISY Register"]
    pub LPSPI2_SCK_SELECT_INPUT: crate::RWRegister<u32>,
    #[doc = "LPSPI2_SDI_SELECT_INPUT DAISY Register"]
    pub LPSPI2_SDI_SELECT_INPUT: crate::RWRegister<u32>,
    #[doc = "LPSPI2_SDO_SELECT_INPUT DAISY Register"]
    pub LPSPI2_SDO_SELECT_INPUT: crate::RWRegister<u32>,
    #[doc = "LPUART1_RXD_SELECT_INPUT DAISY Register"]
    pub LPUART1_RXD_SELECT_INPUT: crate::RWRegister<u32>,
    #[doc = "LPUART1_TXD_SELECT_INPUT DAISY Register"]
    pub LPUART1_TXD_SELECT_INPUT: crate::RWRegister<u32>,
    #[doc = "LPUART2_RXD_SELECT_INPUT DAISY Register"]
    pub LPUART2_RXD_SELECT_INPUT: crate::RWRegister<u32>,
    #[doc = "LPUART2_TXD_SELECT_INPUT DAISY Register"]
    pub LPUART2_TXD_SELECT_INPUT: crate::RWRegister<u32>,
    #[doc = "LPUART3_RXD_SELECT_INPUT DAISY Register"]
    pub LPUART3_RXD_SELECT_INPUT: crate::RWRegister<u32>,
    #[doc = "LPUART3_TXD_SELECT_INPUT DAISY Register"]
    pub LPUART3_TXD_SELECT_INPUT: crate::RWRegister<u32>,
    #[doc = "LPUART4_RXD_SELECT_INPUT DAISY Register"]
    pub LPUART4_RXD_SELECT_INPUT: crate::RWRegister<u32>,
    #[doc = "LPUART4_TXD_SELECT_INPUT DAISY Register"]
    pub LPUART4_TXD_SELECT_INPUT: crate::RWRegister<u32>,
    #[doc = "NMI_GLUE_NMI_SELECT_INPUT DAISY Register"]
    pub NMI_GLUE_NMI_SELECT_INPUT: crate::RWRegister<u32>,
    #[doc = "SPDIF_IN1_SELECT_INPUT DAISY Register"]
    pub SPDIF_IN1_SELECT_INPUT: crate::RWRegister<u32>,
    #[doc = "SPDIF_TX_CLK2_SELECT_INPUT DAISY Register"]
    pub SPDIF_TX_CLK2_SELECT_INPUT: crate::RWRegister<u32>,
    #[doc = "USB_OTG_OC_SELECT_INPUT DAISY Register"]
    pub USB_OTG_OC_SELECT_INPUT: crate::RWRegister<u32>,
    #[doc = "XEV_GLUE_RXEV_SELECT_INPUT DAISY Register"]
    pub XEV_GLUE_RXEV_SELECT_INPUT: crate::RWRegister<u32>,
}
#[doc = "SW_MUX_CTL_PAD_GPIO_AD_14 SW MUX Control Register"]
pub mod SW_MUX_CTL_PAD_GPIO_AD_14 {
    #[doc = "MUX Mode Select Field."]
    pub mod MUX_MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Select mux mode: ALT0 mux port: LPI2C1_SCL of instance: LPI2C1"]
            pub const ALT0: u32 = 0;
            #[doc = "Select mux mode: ALT1 mux port: LPUART3_CTS_B of instance: LPUART3"]
            pub const ALT1: u32 = 0x01;
            #[doc = "Select mux mode: ALT2 mux port: KPP_COL00 of instance: KPP"]
            pub const ALT2: u32 = 0x02;
            #[doc = "Select mux mode: ALT3 mux port: LPUART4_CTS_B of instance: LPUART4"]
            pub const ALT3: u32 = 0x03;
            #[doc = "Select mux mode: ALT4 mux port: FLEXIO1_IO26 of instance: FLEXIO1"]
            pub const ALT4: u32 = 0x04;
            #[doc = "Select mux mode: ALT5 mux port: GPIOMUX_IO28 of instance: GPIOMUX"]
            pub const ALT5: u32 = 0x05;
            #[doc = "Select mux mode: ALT6 mux port: REF_CLK_24M of instance: XTAL OSC"]
            pub const ALT6: u32 = 0x06;
            #[doc = "Select mux mode: ALT7 mux port: XBAR1_INOUT02 of instance: XBAR1"]
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
            #[doc = "Force input path of pad GPIO_AD_14"]
            pub const ENABLED: u32 = 0x01;
        }
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_AD_13 SW MUX Control Register"]
pub mod SW_MUX_CTL_PAD_GPIO_AD_13 {
    #[doc = "MUX Mode Select Field."]
    pub mod MUX_MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Select mux mode: ALT0 mux port: LPI2C1_SDA of instance: LPI2C1"]
            pub const ALT0: u32 = 0;
            #[doc = "Select mux mode: ALT1 mux port: LPUART3_RTS_B of instance: LPUART3"]
            pub const ALT1: u32 = 0x01;
            #[doc = "Select mux mode: ALT2 mux port: KPP_ROW00 of instance: KPP"]
            pub const ALT2: u32 = 0x02;
            #[doc = "Select mux mode: ALT3 mux port: LPUART4_RTS_B of instance: LPUART4"]
            pub const ALT3: u32 = 0x03;
            #[doc = "Select mux mode: ALT4 mux port: FLEXIO1_IO25 of instance: FLEXIO1"]
            pub const ALT4: u32 = 0x04;
            #[doc = "Select mux mode: ALT5 mux port: GPIOMUX_IO27 of instance: GPIOMUX"]
            pub const ALT5: u32 = 0x05;
            #[doc = "Select mux mode: ALT6 mux port: NMI_GLUE_NMI of instance: NMI_GLUE"]
            pub const ALT6: u32 = 0x06;
            #[doc = "Select mux mode: ALT7 mux port: JTAG_TMS of instance: JTAG"]
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
            #[doc = "Force input path of pad GPIO_AD_13"]
            pub const ENABLED: u32 = 0x01;
        }
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_AD_12 SW MUX Control Register"]
pub mod SW_MUX_CTL_PAD_GPIO_AD_12 {
    #[doc = "MUX Mode Select Field."]
    pub mod MUX_MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Select mux mode: ALT0 mux port: LPSPI2_SCK of instance: LPSPI2"]
            pub const ALT0: u32 = 0;
            #[doc = "Select mux mode: ALT1 mux port: FLEXPWM1_PWM0_X of instance: FLEXPWM1"]
            pub const ALT1: u32 = 0x01;
            #[doc = "Select mux mode: ALT2 mux port: KPP_COL01 of instance: KPP"]
            pub const ALT2: u32 = 0x02;
            #[doc = "Select mux mode: ALT3 mux port: PIT_TRIGGER01 of instance: PIT"]
            pub const ALT3: u32 = 0x03;
            #[doc = "Select mux mode: ALT4 mux port: FLEXIO1_IO24 of instance: FLEXIO1"]
            pub const ALT4: u32 = 0x04;
            #[doc = "Select mux mode: ALT5 mux port: GPIOMUX_IO26 of instance: GPIOMUX"]
            pub const ALT5: u32 = 0x05;
            #[doc = "Select mux mode: ALT6 mux port: USB_OTG1_PWR of instance: USB"]
            pub const ALT6: u32 = 0x06;
            #[doc = "Select mux mode: ALT7 mux port: JTAG_TCK of instance: JTAG"]
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
            #[doc = "Force input path of pad GPIO_AD_12"]
            pub const ENABLED: u32 = 0x01;
        }
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_AD_11 SW MUX Control Register"]
pub mod SW_MUX_CTL_PAD_GPIO_AD_11 {
    #[doc = "MUX Mode Select Field."]
    pub mod MUX_MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Select mux mode: ALT0 mux port: LPSPI2_PCS0 of instance: LPSPI2"]
            pub const ALT0: u32 = 0;
            #[doc = "Select mux mode: ALT1 mux port: FLEXPWM1_PWM1_X of instance: FLEXPWM1"]
            pub const ALT1: u32 = 0x01;
            #[doc = "Select mux mode: ALT2 mux port: KPP_ROW01 of instance: KPP"]
            pub const ALT2: u32 = 0x02;
            #[doc = "Select mux mode: ALT3 mux port: PIT_TRIGGER02 of instance: PIT"]
            pub const ALT3: u32 = 0x03;
            #[doc = "Select mux mode: ALT4 mux port: FLEXIO1_IO23 of instance: FLEXIO1"]
            pub const ALT4: u32 = 0x04;
            #[doc = "Select mux mode: ALT5 mux port: GPIOMUX_IO25 of instance: GPIOMUX"]
            pub const ALT5: u32 = 0x05;
            #[doc = "Select mux mode: ALT6 mux port: WDOG1_B of instance: WDOG1"]
            pub const ALT6: u32 = 0x06;
            #[doc = "Select mux mode: ALT7 mux port: JTAG_MOD of instance: JTAG"]
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
            #[doc = "Force input path of pad GPIO_AD_11"]
            pub const ENABLED: u32 = 0x01;
        }
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_AD_10 SW MUX Control Register"]
pub mod SW_MUX_CTL_PAD_GPIO_AD_10 {
    #[doc = "MUX Mode Select Field."]
    pub mod MUX_MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Select mux mode: ALT0 mux port: LPSPI2_SDO of instance: LPSPI2"]
            pub const ALT0: u32 = 0;
            #[doc = "Select mux mode: ALT1 mux port: FLEXPWM1_PWM2_X of instance: FLEXPWM1"]
            pub const ALT1: u32 = 0x01;
            #[doc = "Select mux mode: ALT2 mux port: KPP_COL02 of instance: KPP"]
            pub const ALT2: u32 = 0x02;
            #[doc = "Select mux mode: ALT3 mux port: PIT_TRIGGER03 of instance: PIT"]
            pub const ALT3: u32 = 0x03;
            #[doc = "Select mux mode: ALT4 mux port: FLEXIO1_IO22 of instance: FLEXIO1"]
            pub const ALT4: u32 = 0x04;
            #[doc = "Select mux mode: ALT5 mux port: GPIOMUX_IO24 of instance: GPIOMUX"]
            pub const ALT5: u32 = 0x05;
            #[doc = "Select mux mode: ALT6 mux port: OTG1_ID of instance: anatop"]
            pub const ALT6: u32 = 0x06;
            #[doc = "Select mux mode: ALT7 mux port: JTAG_TDI of instance: JTAG"]
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
            #[doc = "Force input path of pad GPIO_AD_10"]
            pub const ENABLED: u32 = 0x01;
        }
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_AD_09 SW MUX Control Register"]
pub mod SW_MUX_CTL_PAD_GPIO_AD_09 {
    #[doc = "MUX Mode Select Field."]
    pub mod MUX_MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Select mux mode: ALT0 mux port: LPSPI2_SDI of instance: LPSPI2"]
            pub const ALT0: u32 = 0;
            #[doc = "Select mux mode: ALT1 mux port: FLEXPWM1_PWM3_X of instance: FLEXPWM1"]
            pub const ALT1: u32 = 0x01;
            #[doc = "Select mux mode: ALT2 mux port: KPP_ROW02 of instance: KPP"]
            pub const ALT2: u32 = 0x02;
            #[doc = "Select mux mode: ALT3 mux port: ARM_TRACE_SWO of instance: cm7_mxrt"]
            pub const ALT3: u32 = 0x03;
            #[doc = "Select mux mode: ALT4 mux port: FLEXIO1_IO21 of instance: FLEXIO1"]
            pub const ALT4: u32 = 0x04;
            #[doc = "Select mux mode: ALT5 mux port: GPIOMUX_IO23 of instance: GPIOMUX"]
            pub const ALT5: u32 = 0x05;
            #[doc = "Select mux mode: ALT6 mux port: REF_32K_OUT of instance: anatop"]
            pub const ALT6: u32 = 0x06;
            #[doc = "Select mux mode: ALT7 mux port: JTAG_TDO of instance: JTAG"]
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
            #[doc = "Force input path of pad GPIO_AD_09"]
            pub const ENABLED: u32 = 0x01;
        }
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_AD_08 SW MUX Control Register"]
pub mod SW_MUX_CTL_PAD_GPIO_AD_08 {
    #[doc = "MUX Mode Select Field."]
    pub mod MUX_MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Select mux mode: ALT0 mux port: LPI2C2_SCL of instance: LPI2C2"]
            pub const ALT0: u32 = 0;
            #[doc = "Select mux mode: ALT1 mux port: LPUART3_TXD of instance: LPUART3"]
            pub const ALT1: u32 = 0x01;
            #[doc = "Select mux mode: ALT2 mux port: ARM_CM7_TXEV of instance: cm7_mxrt"]
            pub const ALT2: u32 = 0x02;
            #[doc = "Select mux mode: ALT3 mux port: LPUART2_CTS_B of instance: LPUART2"]
            pub const ALT3: u32 = 0x03;
            #[doc = "Select mux mode: ALT4 mux port: GPT2_COMPARE3 of instance: GPT2"]
            pub const ALT4: u32 = 0x04;
            #[doc = "Select mux mode: ALT5 mux port: GPIOMUX_IO22 of instance: GPIOMUX"]
            pub const ALT5: u32 = 0x05;
            #[doc = "Select mux mode: ALT6 mux port: EWM_OUT_B of instance: EWM"]
            pub const ALT6: u32 = 0x06;
            #[doc = "Select mux mode: ALT7 mux port: JTAG_TRSTB of instance: JTAG"]
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
            #[doc = "Force input path of pad GPIO_AD_08"]
            pub const ENABLED: u32 = 0x01;
        }
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_AD_07 SW MUX Control Register"]
pub mod SW_MUX_CTL_PAD_GPIO_AD_07 {
    #[doc = "MUX Mode Select Field."]
    pub mod MUX_MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Select mux mode: ALT0 mux port: LPI2C2_SDA of instance: LPI2C2"]
            pub const ALT0: u32 = 0;
            #[doc = "Select mux mode: ALT1 mux port: LPUART3_RXD of instance: LPUART3"]
            pub const ALT1: u32 = 0x01;
            #[doc = "Select mux mode: ALT2 mux port: ARM_CM7_RXEV of instance: cm7_mxrt"]
            pub const ALT2: u32 = 0x02;
            #[doc = "Select mux mode: ALT3 mux port: LPUART2_RTS_B of instance: LPUART2"]
            pub const ALT3: u32 = 0x03;
            #[doc = "Select mux mode: ALT4 mux port: GPT2_CAPTURE2 of instance: GPT2"]
            pub const ALT4: u32 = 0x04;
            #[doc = "Select mux mode: ALT5 mux port: GPIOMUX_IO21 of instance: GPIOMUX"]
            pub const ALT5: u32 = 0x05;
            #[doc = "Select mux mode: ALT6 mux port: OCOTP_FUSE_LATCHED of instance: OCOTP"]
            pub const ALT6: u32 = 0x06;
            #[doc = "Select mux mode: ALT7 mux port: XBAR1_INOUT03 of instance: XBAR1"]
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
            #[doc = "Force input path of pad GPIO_AD_07"]
            pub const ENABLED: u32 = 0x01;
        }
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_AD_06 SW MUX Control Register"]
pub mod SW_MUX_CTL_PAD_GPIO_AD_06 {
    #[doc = "MUX Mode Select Field."]
    pub mod MUX_MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Select mux mode: ALT0 mux port: LPSPI1_SCK of instance: LPSPI1"]
            pub const ALT0: u32 = 0;
            #[doc = "Select mux mode: ALT1 mux port: PIT_TRIGGER00 of instance: PIT"]
            pub const ALT1: u32 = 0x01;
            #[doc = "Select mux mode: ALT2 mux port: FLEXPWM1_PWM3_A of instance: FLEXPWM1"]
            pub const ALT2: u32 = 0x02;
            #[doc = "Select mux mode: ALT3 mux port: KPP_COL01 of instance: KPP"]
            pub const ALT3: u32 = 0x03;
            #[doc = "Select mux mode: ALT4 mux port: GPT2_COMPARE2 of instance: GPT2"]
            pub const ALT4: u32 = 0x04;
            #[doc = "Select mux mode: ALT5 mux port: GPIOMUX_IO20 of instance: GPIOMUX"]
            pub const ALT5: u32 = 0x05;
            #[doc = "Select mux mode: ALT6 mux port: LPI2C1_HREQ of instance: LPI2C1"]
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
            #[doc = "Force input path of pad GPIO_AD_06"]
            pub const ENABLED: u32 = 0x01;
        }
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_AD_05 SW MUX Control Register"]
pub mod SW_MUX_CTL_PAD_GPIO_AD_05 {
    #[doc = "MUX Mode Select Field."]
    pub mod MUX_MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Select mux mode: ALT0 mux port: LPSPI1_PCS0 of instance: LPSPI1"]
            pub const ALT0: u32 = 0;
            #[doc = "Select mux mode: ALT1 mux port: PIT_TRIGGER01 of instance: PIT"]
            pub const ALT1: u32 = 0x01;
            #[doc = "Select mux mode: ALT2 mux port: FLEXPWM1_PWM3_B of instance: FLEXPWM1"]
            pub const ALT2: u32 = 0x02;
            #[doc = "Select mux mode: ALT3 mux port: KPP_ROW01 of instance: KPP"]
            pub const ALT3: u32 = 0x03;
            #[doc = "Select mux mode: ALT4 mux port: GPT2_CAPTURE1 of instance: GPT2"]
            pub const ALT4: u32 = 0x04;
            #[doc = "Select mux mode: ALT5 mux port: GPIOMUX_IO19 of instance: GPIOMUX"]
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
            #[doc = "Force input path of pad GPIO_AD_05"]
            pub const ENABLED: u32 = 0x01;
        }
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_AD_04 SW MUX Control Register"]
pub mod SW_MUX_CTL_PAD_GPIO_AD_04 {
    #[doc = "MUX Mode Select Field."]
    pub mod MUX_MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Select mux mode: ALT0 mux port: LPSPI1_SDO of instance: LPSPI1"]
            pub const ALT0: u32 = 0;
            #[doc = "Select mux mode: ALT1 mux port: PIT_TRIGGER02 of instance: PIT"]
            pub const ALT1: u32 = 0x01;
            #[doc = "Select mux mode: ALT2 mux port: FLEXPWM1_PWM2_A of instance: FLEXPWM1"]
            pub const ALT2: u32 = 0x02;
            #[doc = "Select mux mode: ALT3 mux port: KPP_COL02 of instance: KPP"]
            pub const ALT3: u32 = 0x03;
            #[doc = "Select mux mode: ALT4 mux port: GPT2_COMPARE1 of instance: GPT2"]
            pub const ALT4: u32 = 0x04;
            #[doc = "Select mux mode: ALT5 mux port: GPIOMUX_IO18 of instance: GPIOMUX"]
            pub const ALT5: u32 = 0x05;
            #[doc = "Select mux mode: ALT6 mux port: SNVS_HP_VIO_5_CTL of instance: snvs_hp"]
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
            #[doc = "Force input path of pad GPIO_AD_04"]
            pub const ENABLED: u32 = 0x01;
        }
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_AD_03 SW MUX Control Register"]
pub mod SW_MUX_CTL_PAD_GPIO_AD_03 {
    #[doc = "MUX Mode Select Field."]
    pub mod MUX_MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Select mux mode: ALT0 mux port: LPSPI1_SDI of instance: LPSPI1"]
            pub const ALT0: u32 = 0;
            #[doc = "Select mux mode: ALT1 mux port: PIT_TRIGGER03 of instance: PIT"]
            pub const ALT1: u32 = 0x01;
            #[doc = "Select mux mode: ALT2 mux port: FLEXPWM1_PWM2_B of instance: FLEXPWM1"]
            pub const ALT2: u32 = 0x02;
            #[doc = "Select mux mode: ALT3 mux port: KPP_ROW02 of instance: KPP"]
            pub const ALT3: u32 = 0x03;
            #[doc = "Select mux mode: ALT4 mux port: GPT2_CLK of instance: GPT2"]
            pub const ALT4: u32 = 0x04;
            #[doc = "Select mux mode: ALT5 mux port: GPIOMUX_IO17 of instance: GPIOMUX"]
            pub const ALT5: u32 = 0x05;
            #[doc = "Select mux mode: ALT6 mux port: SNVS_HP_VIO_5_B of instance: snvs_hp"]
            pub const ALT6: u32 = 0x06;
            #[doc = "Select mux mode: ALT7 mux port: JTAG_DE_B of instance: JTAG"]
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
            #[doc = "Force input path of pad GPIO_AD_03"]
            pub const ENABLED: u32 = 0x01;
        }
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_AD_02 SW MUX Control Register"]
pub mod SW_MUX_CTL_PAD_GPIO_AD_02 {
    #[doc = "MUX Mode Select Field."]
    pub mod MUX_MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Select mux mode: ALT0 mux port: LPUART4_TXD of instance: LPUART4"]
            pub const ALT0: u32 = 0;
            #[doc = "Select mux mode: ALT1 mux port: LPSPI1_PCS1 of instance: LPSPI1"]
            pub const ALT1: u32 = 0x01;
            #[doc = "Select mux mode: ALT2 mux port: WDOG2_B of instance: WDOG2"]
            pub const ALT2: u32 = 0x02;
            #[doc = "Select mux mode: ALT3 mux port: LPI2C2_SCL of instance: LPI2C2"]
            pub const ALT3: u32 = 0x03;
            #[doc = "Select mux mode: ALT4 mux port: MQS_RIGHT of instance: MQS"]
            pub const ALT4: u32 = 0x04;
            #[doc = "Select mux mode: ALT5 mux port: GPIOMUX_IO16 of instance: GPIOMUX"]
            pub const ALT5: u32 = 0x05;
            #[doc = "Select mux mode: ALT7 mux port: ARM_CM7_TRACE_CLK of instance: cm7_mxrt"]
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
            #[doc = "Force input path of pad GPIO_AD_02"]
            pub const ENABLED: u32 = 0x01;
        }
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_AD_01 SW MUX Control Register"]
pub mod SW_MUX_CTL_PAD_GPIO_AD_01 {
    #[doc = "MUX Mode Select Field."]
    pub mod MUX_MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Select mux mode: ALT0 mux port: LPUART4_RXD of instance: LPUART4"]
            pub const ALT0: u32 = 0;
            #[doc = "Select mux mode: ALT1 mux port: LPSPI2_PCS1 of instance: LPSPI2"]
            pub const ALT1: u32 = 0x01;
            #[doc = "Select mux mode: ALT2 mux port: WDOG1_ANY of instance: WDOG1"]
            pub const ALT2: u32 = 0x02;
            #[doc = "Select mux mode: ALT3 mux port: LPI2C2_SDA of instance: LPI2C2"]
            pub const ALT3: u32 = 0x03;
            #[doc = "Select mux mode: ALT4 mux port: MQS_LEFT of instance: MQS"]
            pub const ALT4: u32 = 0x04;
            #[doc = "Select mux mode: ALT5 mux port: GPIOMUX_IO15 of instance: GPIOMUX"]
            pub const ALT5: u32 = 0x05;
            #[doc = "Select mux mode: ALT6 mux port: USB_OTG1_OC of instance: USB"]
            pub const ALT6: u32 = 0x06;
            #[doc = "Select mux mode: ALT7 mux port: ARM_CM7_TRACE_SWO of instance: cm7_mxrt"]
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
            #[doc = "Force input path of pad GPIO_AD_01"]
            pub const ENABLED: u32 = 0x01;
        }
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_AD_00 SW MUX Control Register"]
pub mod SW_MUX_CTL_PAD_GPIO_AD_00 {
    #[doc = "MUX Mode Select Field."]
    pub mod MUX_MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Select mux mode: ALT0 mux port: LPUART2_TXD of instance: LPUART2"]
            pub const ALT0: u32 = 0;
            #[doc = "Select mux mode: ALT1 mux port: LPSPI1_PCS2 of instance: LPSPI1"]
            pub const ALT1: u32 = 0x01;
            #[doc = "Select mux mode: ALT2 mux port: KPP_COL03 of instance: KPP"]
            pub const ALT2: u32 = 0x02;
            #[doc = "Select mux mode: ALT3 mux port: USB_OTG1_PWR of instance: USB"]
            pub const ALT3: u32 = 0x03;
            #[doc = "Select mux mode: ALT4 mux port: FLEXIO1_IO20 of instance: FLEXIO1"]
            pub const ALT4: u32 = 0x04;
            #[doc = "Select mux mode: ALT5 mux port: GPIOMUX_IO14 of instance: GPIOMUX"]
            pub const ALT5: u32 = 0x05;
            #[doc = "Select mux mode: ALT6 mux port: NMI_GLUE_NMI of instance: NMI_GLUE"]
            pub const ALT6: u32 = 0x06;
            #[doc = "Select mux mode: ALT7 mux port: ARM_CM7_TRACE00 of instance: cm7_mxrt"]
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
            #[doc = "Force input path of pad GPIO_AD_00"]
            pub const ENABLED: u32 = 0x01;
        }
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_SD_14 SW MUX Control Register"]
pub mod SW_MUX_CTL_PAD_GPIO_SD_14 {
    #[doc = "MUX Mode Select Field."]
    pub mod MUX_MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Select mux mode: ALT0 mux port: FLEXSPI_A_DQS of instance: FLEXSPI"]
            pub const ALT0: u32 = 0;
            #[doc = "Select mux mode: ALT1 mux port: FLEXSPI_B_DQS of instance: FLEXSPI"]
            pub const ALT1: u32 = 0x01;
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
            #[doc = "Force input path of pad GPIO_SD_14"]
            pub const ENABLED: u32 = 0x01;
        }
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_SD_13 SW MUX Control Register"]
pub mod SW_MUX_CTL_PAD_GPIO_SD_13 {
    #[doc = "MUX Mode Select Field."]
    pub mod MUX_MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Select mux mode: ALT0 mux port: FLEXSPI_B_SCLK of instance: FLEXSPI"]
            pub const ALT0: u32 = 0;
            #[doc = "Select mux mode: ALT1 mux port: SAI3_RX_BCLK of instance: SAI3"]
            pub const ALT1: u32 = 0x01;
            #[doc = "Select mux mode: ALT2 mux port: ARM_CM7_TXEV of instance: cm7_mxrt"]
            pub const ALT2: u32 = 0x02;
            #[doc = "Select mux mode: ALT3 mux port: CCM_PMIC_RDY of instance: CCM"]
            pub const ALT3: u32 = 0x03;
            #[doc = "Select mux mode: ALT4 mux port: FLEXIO1_IO19 of instance: FLEXIO1"]
            pub const ALT4: u32 = 0x04;
            #[doc = "Select mux mode: ALT5 mux port: GPIO2_IO13 of instance: GPIO2"]
            pub const ALT5: u32 = 0x05;
            #[doc = "Select mux mode: ALT6 mux port: SRC_BT_CFG03 of instance: SRC"]
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
            #[doc = "Force input path of pad GPIO_SD_13"]
            pub const ENABLED: u32 = 0x01;
        }
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_SD_12 SW MUX Control Register"]
pub mod SW_MUX_CTL_PAD_GPIO_SD_12 {
    #[doc = "MUX Mode Select Field."]
    pub mod MUX_MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Select mux mode: ALT0 mux port: FLEXSPI_A_DQS of instance: FLEXSPI"]
            pub const ALT0: u32 = 0;
            #[doc = "Select mux mode: ALT1 mux port: LPSPI2_PCS0 of instance: LPSPI2"]
            pub const ALT1: u32 = 0x01;
            #[doc = "Select mux mode: ALT2 mux port: LPUART1_TXD of instance: LPUART1"]
            pub const ALT2: u32 = 0x02;
            #[doc = "Select mux mode: ALT4 mux port: FLEXIO1_IO18 of instance: FLEXIO1"]
            pub const ALT4: u32 = 0x04;
            #[doc = "Select mux mode: ALT5 mux port: GPIO2_IO12 of instance: GPIO2"]
            pub const ALT5: u32 = 0x05;
            #[doc = "Select mux mode: ALT6 mux port: WDOG2_RST_B_DEB of instance: WDOG2"]
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
            #[doc = "Force input path of pad GPIO_SD_12"]
            pub const ENABLED: u32 = 0x01;
        }
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_SD_11 SW MUX Control Register"]
pub mod SW_MUX_CTL_PAD_GPIO_SD_11 {
    #[doc = "MUX Mode Select Field."]
    pub mod MUX_MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Select mux mode: ALT0 mux port: FLEXSPI_A_DATA3 of instance: FLEXSPI"]
            pub const ALT0: u32 = 0;
            #[doc = "Select mux mode: ALT1 mux port: LPSPI2_SCK of instance: LPSPI2"]
            pub const ALT1: u32 = 0x01;
            #[doc = "Select mux mode: ALT2 mux port: LPUART1_RXD of instance: LPUART1"]
            pub const ALT2: u32 = 0x02;
            #[doc = "Select mux mode: ALT4 mux port: FLEXIO1_IO17 of instance: FLEXIO1"]
            pub const ALT4: u32 = 0x04;
            #[doc = "Select mux mode: ALT5 mux port: GPIO2_IO11 of instance: GPIO2"]
            pub const ALT5: u32 = 0x05;
            #[doc = "Select mux mode: ALT6 mux port: WDOG1_RST_B_DEB of instance: WDOG1"]
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
            #[doc = "Force input path of pad GPIO_SD_11"]
            pub const ENABLED: u32 = 0x01;
        }
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_SD_10 SW MUX Control Register"]
pub mod SW_MUX_CTL_PAD_GPIO_SD_10 {
    #[doc = "MUX Mode Select Field."]
    pub mod MUX_MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Select mux mode: ALT0 mux port: FLEXSPI_A_SCLK of instance: FLEXSPI"]
            pub const ALT0: u32 = 0;
            #[doc = "Select mux mode: ALT1 mux port: LPSPI2_SDO of instance: LPSPI2"]
            pub const ALT1: u32 = 0x01;
            #[doc = "Select mux mode: ALT2 mux port: LPUART2_TXD of instance: LPUART2"]
            pub const ALT2: u32 = 0x02;
            #[doc = "Select mux mode: ALT4 mux port: FLEXIO1_IO16 of instance: FLEXIO1"]
            pub const ALT4: u32 = 0x04;
            #[doc = "Select mux mode: ALT5 mux port: GPIO2_IO10 of instance: GPIO2"]
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
            #[doc = "Force input path of pad GPIO_SD_10"]
            pub const ENABLED: u32 = 0x01;
        }
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_SD_09 SW MUX Control Register"]
pub mod SW_MUX_CTL_PAD_GPIO_SD_09 {
    #[doc = "MUX Mode Select Field."]
    pub mod MUX_MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Select mux mode: ALT0 mux port: FLEXSPI_A_DATA0 of instance: FLEXSPI"]
            pub const ALT0: u32 = 0;
            #[doc = "Select mux mode: ALT1 mux port: LPSPI2_SDI of instance: LPSPI2"]
            pub const ALT1: u32 = 0x01;
            #[doc = "Select mux mode: ALT2 mux port: LPUART2_RXD of instance: LPUART2"]
            pub const ALT2: u32 = 0x02;
            #[doc = "Select mux mode: ALT4 mux port: FLEXIO1_IO15 of instance: FLEXIO1"]
            pub const ALT4: u32 = 0x04;
            #[doc = "Select mux mode: ALT5 mux port: GPIO2_IO09 of instance: GPIO2"]
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
            #[doc = "Force input path of pad GPIO_SD_09"]
            pub const ENABLED: u32 = 0x01;
        }
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_SD_08 SW MUX Control Register"]
pub mod SW_MUX_CTL_PAD_GPIO_SD_08 {
    #[doc = "MUX Mode Select Field."]
    pub mod MUX_MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Select mux mode: ALT0 mux port: FLEXSPI_A_DATA2 of instance: FLEXSPI"]
            pub const ALT0: u32 = 0;
            #[doc = "Select mux mode: ALT1 mux port: LPI2C2_SCL of instance: LPI2C2"]
            pub const ALT1: u32 = 0x01;
            #[doc = "Select mux mode: ALT2 mux port: LPSPI1_SCK of instance: LPSPI1"]
            pub const ALT2: u32 = 0x02;
            #[doc = "Select mux mode: ALT4 mux port: FLEXIO1_IO14 of instance: FLEXIO1"]
            pub const ALT4: u32 = 0x04;
            #[doc = "Select mux mode: ALT5 mux port: GPIO2_IO08 of instance: GPIO2"]
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
            #[doc = "Force input path of pad GPIO_SD_08"]
            pub const ENABLED: u32 = 0x01;
        }
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_SD_07 SW MUX Control Register"]
pub mod SW_MUX_CTL_PAD_GPIO_SD_07 {
    #[doc = "MUX Mode Select Field."]
    pub mod MUX_MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Select mux mode: ALT0 mux port: FLEXSPI_A_DATA1 of instance: FLEXSPI"]
            pub const ALT0: u32 = 0;
            #[doc = "Select mux mode: ALT1 mux port: LPI2C2_SDA of instance: LPI2C2"]
            pub const ALT1: u32 = 0x01;
            #[doc = "Select mux mode: ALT2 mux port: LPSPI1_PCS0 of instance: LPSPI1"]
            pub const ALT2: u32 = 0x02;
            #[doc = "Select mux mode: ALT4 mux port: FLEXIO1_IO13 of instance: FLEXIO1"]
            pub const ALT4: u32 = 0x04;
            #[doc = "Select mux mode: ALT5 mux port: GPIO2_IO07 of instance: GPIO2"]
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
            #[doc = "Force input path of pad GPIO_SD_07"]
            pub const ENABLED: u32 = 0x01;
        }
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_SD_06 SW MUX Control Register"]
pub mod SW_MUX_CTL_PAD_GPIO_SD_06 {
    #[doc = "MUX Mode Select Field."]
    pub mod MUX_MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Select mux mode: ALT0 mux port: FLEXSPI_A_SS0_B of instance: FLEXSPI"]
            pub const ALT0: u32 = 0;
            #[doc = "Select mux mode: ALT1 mux port: LPI2C1_SCL of instance: LPI2C1"]
            pub const ALT1: u32 = 0x01;
            #[doc = "Select mux mode: ALT2 mux port: LPSPI1_SDO of instance: LPSPI1"]
            pub const ALT2: u32 = 0x02;
            #[doc = "Select mux mode: ALT4 mux port: FLEXIO1_IO12 of instance: FLEXIO1"]
            pub const ALT4: u32 = 0x04;
            #[doc = "Select mux mode: ALT5 mux port: GPIO2_IO06 of instance: GPIO2"]
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
            #[doc = "Force input path of pad GPIO_SD_06"]
            pub const ENABLED: u32 = 0x01;
        }
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_SD_05 SW MUX Control Register"]
pub mod SW_MUX_CTL_PAD_GPIO_SD_05 {
    #[doc = "MUX Mode Select Field."]
    pub mod MUX_MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Select mux mode: ALT0 mux port: FLEXSPI_A_SS1_B of instance: FLEXSPI"]
            pub const ALT0: u32 = 0;
            #[doc = "Select mux mode: ALT1 mux port: LPI2C1_SDA of instance: LPI2C1"]
            pub const ALT1: u32 = 0x01;
            #[doc = "Select mux mode: ALT2 mux port: LPSPI1_SDI of instance: LPSPI1"]
            pub const ALT2: u32 = 0x02;
            #[doc = "Select mux mode: ALT4 mux port: FLEXIO1_IO11 of instance: FLEXIO1"]
            pub const ALT4: u32 = 0x04;
            #[doc = "Select mux mode: ALT5 mux port: GPIO2_IO05 of instance: GPIO2"]
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
            #[doc = "Force input path of pad GPIO_SD_05"]
            pub const ENABLED: u32 = 0x01;
        }
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_SD_04 SW MUX Control Register"]
pub mod SW_MUX_CTL_PAD_GPIO_SD_04 {
    #[doc = "MUX Mode Select Field."]
    pub mod MUX_MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Select mux mode: ALT0 mux port: FLEXSPI_B_DATA03 of instance: FLEXSPI"]
            pub const ALT0: u32 = 0;
            #[doc = "Select mux mode: ALT1 mux port: SAI3_RX_SYNC of instance: SAI3"]
            pub const ALT1: u32 = 0x01;
            #[doc = "Select mux mode: ALT2 mux port: FLEXPWM1_PWM1_A of instance: FLEXPWM1"]
            pub const ALT2: u32 = 0x02;
            #[doc = "Select mux mode: ALT3 mux port: CCM_WAIT of instance: CCM"]
            pub const ALT3: u32 = 0x03;
            #[doc = "Select mux mode: ALT4 mux port: FLEXIO1_IO10 of instance: FLEXIO1"]
            pub const ALT4: u32 = 0x04;
            #[doc = "Select mux mode: ALT5 mux port: GPIO2_IO04 of instance: GPIO2"]
            pub const ALT5: u32 = 0x05;
            #[doc = "Select mux mode: ALT6 mux port: SRC_BOOT_MODE00 of instance: SRC"]
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
            #[doc = "Force input path of pad GPIO_SD_04"]
            pub const ENABLED: u32 = 0x01;
        }
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_SD_03 SW MUX Control Register"]
pub mod SW_MUX_CTL_PAD_GPIO_SD_03 {
    #[doc = "MUX Mode Select Field."]
    pub mod MUX_MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Select mux mode: ALT0 mux port: FLEXSPI_B_DATA00 of instance: FLEXSPI"]
            pub const ALT0: u32 = 0;
            #[doc = "Select mux mode: ALT1 mux port: SAI3_RX_DATA of instance: SAI3"]
            pub const ALT1: u32 = 0x01;
            #[doc = "Select mux mode: ALT2 mux port: FLEXPWM1_PWM1_B of instance: FLEXPWM1"]
            pub const ALT2: u32 = 0x02;
            #[doc = "Select mux mode: ALT3 mux port: CCM_REF_EN_B of instance: CCM"]
            pub const ALT3: u32 = 0x03;
            #[doc = "Select mux mode: ALT4 mux port: FLEXIO1_IO09 of instance: FLEXIO1"]
            pub const ALT4: u32 = 0x04;
            #[doc = "Select mux mode: ALT5 mux port: GPIO2_IO03 of instance: GPIO2"]
            pub const ALT5: u32 = 0x05;
            #[doc = "Select mux mode: ALT6 mux port: SRC_BOOT_MODE01 of instance: SRC"]
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
            #[doc = "Force input path of pad GPIO_SD_03"]
            pub const ENABLED: u32 = 0x01;
        }
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_SD_02 SW MUX Control Register"]
pub mod SW_MUX_CTL_PAD_GPIO_SD_02 {
    #[doc = "MUX Mode Select Field."]
    pub mod MUX_MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Select mux mode: ALT0 mux port: FLEXSPI_B_DATA02 of instance: FLEXSPI"]
            pub const ALT0: u32 = 0;
            #[doc = "Select mux mode: ALT1 mux port: SAI3_TX_DATA of instance: SAI3"]
            pub const ALT1: u32 = 0x01;
            #[doc = "Select mux mode: ALT2 mux port: FLEXPWM1_PWM0_A of instance: FLEXPWM1"]
            pub const ALT2: u32 = 0x02;
            #[doc = "Select mux mode: ALT3 mux port: CCM_CLKO1 of instance: CCM"]
            pub const ALT3: u32 = 0x03;
            #[doc = "Select mux mode: ALT4 mux port: FLEXIO1_IO08 of instance: FLEXIO1"]
            pub const ALT4: u32 = 0x04;
            #[doc = "Select mux mode: ALT5 mux port: GPIO2_IO02 of instance: GPIO2"]
            pub const ALT5: u32 = 0x05;
            #[doc = "Select mux mode: ALT6 mux port: SRC_BT_CFG00 of instance: SRC"]
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
            #[doc = "Force input path of pad GPIO_SD_02"]
            pub const ENABLED: u32 = 0x01;
        }
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_SD_01 SW MUX Control Register"]
pub mod SW_MUX_CTL_PAD_GPIO_SD_01 {
    #[doc = "MUX Mode Select Field."]
    pub mod MUX_MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Select mux mode: ALT0 mux port: FLEXSPI_B_DATA01 of instance: FLEXSPI"]
            pub const ALT0: u32 = 0;
            #[doc = "Select mux mode: ALT1 mux port: SAI3_TX_BCLK of instance: SAI3"]
            pub const ALT1: u32 = 0x01;
            #[doc = "Select mux mode: ALT2 mux port: FLEXPWM1_PWM0_B of instance: FLEXPWM1"]
            pub const ALT2: u32 = 0x02;
            #[doc = "Select mux mode: ALT3 mux port: CCM_CLKO2 of instance: CCM"]
            pub const ALT3: u32 = 0x03;
            #[doc = "Select mux mode: ALT4 mux port: FLEXIO1_IO07 of instance: FLEXIO1"]
            pub const ALT4: u32 = 0x04;
            #[doc = "Select mux mode: ALT5 mux port: GPIO2_IO01 of instance: GPIO2"]
            pub const ALT5: u32 = 0x05;
            #[doc = "Select mux mode: ALT6 mux port: SRC_BT_CFG01 of instance: SRC"]
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
            #[doc = "Force input path of pad GPIO_SD_01"]
            pub const ENABLED: u32 = 0x01;
        }
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_SD_00 SW MUX Control Register"]
pub mod SW_MUX_CTL_PAD_GPIO_SD_00 {
    #[doc = "MUX Mode Select Field."]
    pub mod MUX_MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Select mux mode: ALT0 mux port: FLEXSPI_B_SS0_B of instance: FLEXSPI"]
            pub const ALT0: u32 = 0;
            #[doc = "Select mux mode: ALT1 mux port: SAI3_TX_SYNC of instance: SAI3"]
            pub const ALT1: u32 = 0x01;
            #[doc = "Select mux mode: ALT2 mux port: ARM_CM7_RXEV of instance: cm7_mxrt"]
            pub const ALT2: u32 = 0x02;
            #[doc = "Select mux mode: ALT3 mux port: CCM_STOP of instance: CCM"]
            pub const ALT3: u32 = 0x03;
            #[doc = "Select mux mode: ALT4 mux port: FLEXIO1_IO06 of instance: FLEXIO1"]
            pub const ALT4: u32 = 0x04;
            #[doc = "Select mux mode: ALT5 mux port: GPIO2_IO00 of instance: GPIO2"]
            pub const ALT5: u32 = 0x05;
            #[doc = "Select mux mode: ALT6 mux port: SRC_BT_CFG02 of instance: SRC"]
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
            #[doc = "Force input path of pad GPIO_SD_00"]
            pub const ENABLED: u32 = 0x01;
        }
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_13 SW MUX Control Register"]
pub mod SW_MUX_CTL_PAD_GPIO_13 {
    #[doc = "MUX Mode Select Field."]
    pub mod MUX_MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Select mux mode: ALT0 mux port: LPUART2_RXD of instance: LPUART2"]
            pub const ALT0: u32 = 0;
            #[doc = "Select mux mode: ALT1 mux port: LPSPI2_PCS2 of instance: LPSPI2"]
            pub const ALT1: u32 = 0x01;
            #[doc = "Select mux mode: ALT2 mux port: KPP_ROW03 of instance: KPP"]
            pub const ALT2: u32 = 0x02;
            #[doc = "Select mux mode: ALT3 mux port: OTG1_ID of instance: anatop"]
            pub const ALT3: u32 = 0x03;
            #[doc = "Select mux mode: ALT4 mux port: FLEXIO1_IO05 of instance: FLEXIO1"]
            pub const ALT4: u32 = 0x04;
            #[doc = "Select mux mode: ALT5 mux port: GPIOMUX_IO13 of instance: GPIOMUX"]
            pub const ALT5: u32 = 0x05;
            #[doc = "Select mux mode: ALT6 mux port: SPDIF_LOCK of instance: SPDIF"]
            pub const ALT6: u32 = 0x06;
            #[doc = "Select mux mode: ALT7 mux port: ARM_CM7_TRACE01 of instance: cm7_mxrt"]
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
            #[doc = "Force input path of pad GPIO_13"]
            pub const ENABLED: u32 = 0x01;
        }
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_12 SW MUX Control Register"]
pub mod SW_MUX_CTL_PAD_GPIO_12 {
    #[doc = "MUX Mode Select Field."]
    pub mod MUX_MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Select mux mode: ALT0 mux port: LPUART3_TXD of instance: LPUART3"]
            pub const ALT0: u32 = 0;
            #[doc = "Select mux mode: ALT1 mux port: LPI2C1_SCL of instance: LPI2C1"]
            pub const ALT1: u32 = 0x01;
            #[doc = "Select mux mode: ALT2 mux port: KPP_COL00 of instance: KPP"]
            pub const ALT2: u32 = 0x02;
            #[doc = "Select mux mode: ALT3 mux port: USB_OTG1_OC of instance: USB"]
            pub const ALT3: u32 = 0x03;
            #[doc = "Select mux mode: ALT4 mux port: FLEXIO1_IO04 of instance: FLEXIO1"]
            pub const ALT4: u32 = 0x04;
            #[doc = "Select mux mode: ALT5 mux port: GPIOMUX_IO12 of instance: GPIOMUX"]
            pub const ALT5: u32 = 0x05;
            #[doc = "Select mux mode: ALT6 mux port: SPDIF_EXT_CLK of instance: SPDIF"]
            pub const ALT6: u32 = 0x06;
            #[doc = "Select mux mode: ALT7 mux port: ARM_CM7_TRACE02 of instance: cm7_mxrt"]
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
            #[doc = "Force input path of pad GPIO_12"]
            pub const ENABLED: u32 = 0x01;
        }
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_11 SW MUX Control Register"]
pub mod SW_MUX_CTL_PAD_GPIO_11 {
    #[doc = "MUX Mode Select Field."]
    pub mod MUX_MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Select mux mode: ALT0 mux port: LPUART3_RXD of instance: LPUART3"]
            pub const ALT0: u32 = 0;
            #[doc = "Select mux mode: ALT1 mux port: LPI2C1_SDA of instance: LPI2C1"]
            pub const ALT1: u32 = 0x01;
            #[doc = "Select mux mode: ALT2 mux port: KPP_ROW00 of instance: KPP"]
            pub const ALT2: u32 = 0x02;
            #[doc = "Select mux mode: ALT3 mux port: FLEXSPI_B_SS1_B of instance: FLEXSPI"]
            pub const ALT3: u32 = 0x03;
            #[doc = "Select mux mode: ALT4 mux port: FLEXIO1_IO03 of instance: FLEXIO1"]
            pub const ALT4: u32 = 0x04;
            #[doc = "Select mux mode: ALT5 mux port: GPIOMUX_IO11 of instance: GPIOMUX"]
            pub const ALT5: u32 = 0x05;
            #[doc = "Select mux mode: ALT6 mux port: SPDIF_OUT of instance: SPDIF"]
            pub const ALT6: u32 = 0x06;
            #[doc = "Select mux mode: ALT7 mux port: ARM_CM7_TRACE03 of instance: cm7_mxrt"]
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
            #[doc = "Force input path of pad GPIO_11"]
            pub const ENABLED: u32 = 0x01;
        }
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_10 SW MUX Control Register"]
pub mod SW_MUX_CTL_PAD_GPIO_10 {
    #[doc = "MUX Mode Select Field."]
    pub mod MUX_MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Select mux mode: ALT0 mux port: LPUART1_TXD of instance: LPUART1"]
            pub const ALT0: u32 = 0;
            #[doc = "Select mux mode: ALT1 mux port: LPI2C1_HREQ of instance: LPI2C1"]
            pub const ALT1: u32 = 0x01;
            #[doc = "Select mux mode: ALT2 mux port: EWM_OUT_B of instance: EWM"]
            pub const ALT2: u32 = 0x02;
            #[doc = "Select mux mode: ALT3 mux port: LPI2C2_SCL of instance: LPI2C2"]
            pub const ALT3: u32 = 0x03;
            #[doc = "Select mux mode: ALT4 mux port: FLEXIO1_IO02 of instance: FLEXIO1"]
            pub const ALT4: u32 = 0x04;
            #[doc = "Select mux mode: ALT5 mux port: GPIOMUX_IO10 of instance: GPIOMUX"]
            pub const ALT5: u32 = 0x05;
            #[doc = "Select mux mode: ALT6 mux port: SPDIF_IN of instance: SPDIF"]
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
            #[doc = "Force input path of pad GPIO_10"]
            pub const ENABLED: u32 = 0x01;
        }
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_09 SW MUX Control Register"]
pub mod SW_MUX_CTL_PAD_GPIO_09 {
    #[doc = "MUX Mode Select Field."]
    pub mod MUX_MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Select mux mode: ALT0 mux port: LPUART1_RXD of instance: LPUART1"]
            pub const ALT0: u32 = 0;
            #[doc = "Select mux mode: ALT1 mux port: WDOG1_B of instance: WDOG1"]
            pub const ALT1: u32 = 0x01;
            #[doc = "Select mux mode: ALT2 mux port: FLEXSPI_A_SS1_B of instance: FLEXSPI"]
            pub const ALT2: u32 = 0x02;
            #[doc = "Select mux mode: ALT3 mux port: LPI2C2_SDA of instance: LPI2C2"]
            pub const ALT3: u32 = 0x03;
            #[doc = "Select mux mode: ALT4 mux port: FLEXIO1_IO01 of instance: FLEXIO1"]
            pub const ALT4: u32 = 0x04;
            #[doc = "Select mux mode: ALT5 mux port: GPIOMUX_IO09 of instance: GPIOMUX"]
            pub const ALT5: u32 = 0x05;
            #[doc = "Select mux mode: ALT6 mux port: SPDIF_SR_CLK of instance: SPDIF"]
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
            #[doc = "Force input path of pad GPIO_09"]
            pub const ENABLED: u32 = 0x01;
        }
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_08 SW MUX Control Register"]
pub mod SW_MUX_CTL_PAD_GPIO_08 {
    #[doc = "MUX Mode Select Field."]
    pub mod MUX_MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Select mux mode: ALT0 mux port: SAI1_MCLK of instance: SAI1"]
            pub const ALT0: u32 = 0;
            #[doc = "Select mux mode: ALT1 mux port: GPT1_CLK of instance: GPT1"]
            pub const ALT1: u32 = 0x01;
            #[doc = "Select mux mode: ALT2 mux port: FLEXPWM1_PWM3_A of instance: FLEXPWM1"]
            pub const ALT2: u32 = 0x02;
            #[doc = "Select mux mode: ALT3 mux port: LPUART3_TXD of instance: LPUART3"]
            pub const ALT3: u32 = 0x03;
            #[doc = "Select mux mode: ALT4 mux port: FLEXIO1_IO00 of instance: FLEXIO1"]
            pub const ALT4: u32 = 0x04;
            #[doc = "Select mux mode: ALT5 mux port: GPIOMUX_IO08 of instance: GPIOMUX"]
            pub const ALT5: u32 = 0x05;
            #[doc = "Select mux mode: ALT6 mux port: LPUART1_CTS_B of instance: LPUART1"]
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
            #[doc = "Force input path of pad GPIO_08"]
            pub const ENABLED: u32 = 0x01;
        }
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_07 SW MUX Control Register"]
pub mod SW_MUX_CTL_PAD_GPIO_07 {
    #[doc = "MUX Mode Select Field."]
    pub mod MUX_MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Select mux mode: ALT0 mux port: SAI1_TX_SYNC of instance: SAI1"]
            pub const ALT0: u32 = 0;
            #[doc = "Select mux mode: ALT1 mux port: GPT1_COMPARE1 of instance: GPT1"]
            pub const ALT1: u32 = 0x01;
            #[doc = "Select mux mode: ALT2 mux port: FLEXPWM1_PWM3_B of instance: FLEXPWM1"]
            pub const ALT2: u32 = 0x02;
            #[doc = "Select mux mode: ALT3 mux port: LPUART3_RXD of instance: LPUART3"]
            pub const ALT3: u32 = 0x03;
            #[doc = "Select mux mode: ALT4 mux port: SPDIF_LOCK of instance: SPDIF"]
            pub const ALT4: u32 = 0x04;
            #[doc = "Select mux mode: ALT5 mux port: GPIOMUX_IO07 of instance: GPIOMUX"]
            pub const ALT5: u32 = 0x05;
            #[doc = "Select mux mode: ALT6 mux port: LPUART1_RTS_B of instance: LPUART1"]
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
            #[doc = "Force input path of pad GPIO_07"]
            pub const ENABLED: u32 = 0x01;
        }
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_06 SW MUX Control Register"]
pub mod SW_MUX_CTL_PAD_GPIO_06 {
    #[doc = "MUX Mode Select Field."]
    pub mod MUX_MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Select mux mode: ALT0 mux port: SAI1_TX_BCLK of instance: SAI1"]
            pub const ALT0: u32 = 0;
            #[doc = "Select mux mode: ALT1 mux port: GPT1_CAPTURE1 of instance: GPT1"]
            pub const ALT1: u32 = 0x01;
            #[doc = "Select mux mode: ALT2 mux port: FLEXPWM1_PWM2_A of instance: FLEXPWM1"]
            pub const ALT2: u32 = 0x02;
            #[doc = "Select mux mode: ALT3 mux port: LPUART4_TXD of instance: LPUART4"]
            pub const ALT3: u32 = 0x03;
            #[doc = "Select mux mode: ALT4 mux port: SPDIF_EXT_CLK of instance: SPDIF"]
            pub const ALT4: u32 = 0x04;
            #[doc = "Select mux mode: ALT5 mux port: GPIOMUX_IO06 of instance: GPIOMUX"]
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
            #[doc = "Force input path of pad GPIO_06"]
            pub const ENABLED: u32 = 0x01;
        }
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_05 SW MUX Control Register"]
pub mod SW_MUX_CTL_PAD_GPIO_05 {
    #[doc = "MUX Mode Select Field."]
    pub mod MUX_MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Select mux mode: ALT0 mux port: SAI1_TX_DATA01 of instance: SAI1"]
            pub const ALT0: u32 = 0;
            #[doc = "Select mux mode: ALT1 mux port: GPT1_COMPARE2 of instance: GPT1"]
            pub const ALT1: u32 = 0x01;
            #[doc = "Select mux mode: ALT2 mux port: FLEXPWM1_PWM2_B of instance: FLEXPWM1"]
            pub const ALT2: u32 = 0x02;
            #[doc = "Select mux mode: ALT3 mux port: LPUART4_RXD of instance: LPUART4"]
            pub const ALT3: u32 = 0x03;
            #[doc = "Select mux mode: ALT4 mux port: SPDIF_OUT of instance: SPDIF"]
            pub const ALT4: u32 = 0x04;
            #[doc = "Select mux mode: ALT5 mux port: GPIOMUX_IO05 of instance: GPIOMUX"]
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
            #[doc = "Force input path of pad GPIO_05"]
            pub const ENABLED: u32 = 0x01;
        }
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_04 SW MUX Control Register"]
pub mod SW_MUX_CTL_PAD_GPIO_04 {
    #[doc = "MUX Mode Select Field."]
    pub mod MUX_MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Select mux mode: ALT0 mux port: SAI1_TX_DATA00 of instance: SAI1"]
            pub const ALT0: u32 = 0;
            #[doc = "Select mux mode: ALT1 mux port: GPT1_CAPTURE2 of instance: GPT1"]
            pub const ALT1: u32 = 0x01;
            #[doc = "Select mux mode: ALT2 mux port: FLEXPWM1_PWM1_A of instance: FLEXPWM1"]
            pub const ALT2: u32 = 0x02;
            #[doc = "Select mux mode: ALT4 mux port: SPDIF_IN of instance: SPDIF"]
            pub const ALT4: u32 = 0x04;
            #[doc = "Select mux mode: ALT5 mux port: GPIOMUX_IO04 of instance: GPIOMUX"]
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
            #[doc = "Force input path of pad GPIO_04"]
            pub const ENABLED: u32 = 0x01;
        }
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_03 SW MUX Control Register"]
pub mod SW_MUX_CTL_PAD_GPIO_03 {
    #[doc = "MUX Mode Select Field."]
    pub mod MUX_MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Select mux mode: ALT0 mux port: SAI1_RX_DATA00 of instance: SAI1"]
            pub const ALT0: u32 = 0;
            #[doc = "Select mux mode: ALT1 mux port: GPT1_COMPARE3 of instance: GPT1"]
            pub const ALT1: u32 = 0x01;
            #[doc = "Select mux mode: ALT2 mux port: FLEXPWM1_PWM1_B of instance: FLEXPWM1"]
            pub const ALT2: u32 = 0x02;
            #[doc = "Select mux mode: ALT4 mux port: SPDIF_SR_CLK of instance: SPDIF"]
            pub const ALT4: u32 = 0x04;
            #[doc = "Select mux mode: ALT5 mux port: GPIOMUX_IO03 of instance: GPIOMUX"]
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
            #[doc = "Force input path of pad GPIO_03"]
            pub const ENABLED: u32 = 0x01;
        }
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_02 SW MUX Control Register"]
pub mod SW_MUX_CTL_PAD_GPIO_02 {
    #[doc = "MUX Mode Select Field."]
    pub mod MUX_MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Select mux mode: ALT0 mux port: SAI1_RX_SYNC of instance: SAI1"]
            pub const ALT0: u32 = 0;
            #[doc = "Select mux mode: ALT1 mux port: WDOG2_B of instance: WDOG2"]
            pub const ALT1: u32 = 0x01;
            #[doc = "Select mux mode: ALT2 mux port: FLEXPWM1_PWM0_A of instance: FLEXPWM1"]
            pub const ALT2: u32 = 0x02;
            #[doc = "Select mux mode: ALT3 mux port: LPI2C1_SCL of instance: LPI2C1"]
            pub const ALT3: u32 = 0x03;
            #[doc = "Select mux mode: ALT4 mux port: KPP_COL03 of instance: KPP"]
            pub const ALT4: u32 = 0x04;
            #[doc = "Select mux mode: ALT5 mux port: GPIOMUX_IO02 of instance: GPIOMUX"]
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
            #[doc = "Force input path of pad GPIO_02"]
            pub const ENABLED: u32 = 0x01;
        }
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_01 SW MUX Control Register"]
pub mod SW_MUX_CTL_PAD_GPIO_01 {
    #[doc = "MUX Mode Select Field."]
    pub mod MUX_MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Select mux mode: ALT0 mux port: SAI1_RX_BCLK of instance: SAI1"]
            pub const ALT0: u32 = 0;
            #[doc = "Select mux mode: ALT1 mux port: WDOG1_ANY of instance: WDOG1"]
            pub const ALT1: u32 = 0x01;
            #[doc = "Select mux mode: ALT2 mux port: FLEXPWM1_PWM0_B of instance: FLEXPWM1"]
            pub const ALT2: u32 = 0x02;
            #[doc = "Select mux mode: ALT3 mux port: LPI2C1_SDA of instance: LPI2C1"]
            pub const ALT3: u32 = 0x03;
            #[doc = "Select mux mode: ALT4 mux port: KPP_ROW03 of instance: KPP"]
            pub const ALT4: u32 = 0x04;
            #[doc = "Select mux mode: ALT5 mux port: GPIOMUX_IO01 of instance: GPIOMUX"]
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
            #[doc = "Force input path of pad GPIO_01"]
            pub const ENABLED: u32 = 0x01;
        }
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_00 SW MUX Control Register"]
pub mod SW_MUX_CTL_PAD_GPIO_00 {
    #[doc = "MUX Mode Select Field."]
    pub mod MUX_MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Select mux mode: ALT0 mux port: FLEXSPI_B_DQS of instance: FLEXSPI"]
            pub const ALT0: u32 = 0;
            #[doc = "Select mux mode: ALT1 mux port: SAI3_MCLK of instance: SAI3"]
            pub const ALT1: u32 = 0x01;
            #[doc = "Select mux mode: ALT2 mux port: LPSPI2_PCS3 of instance: LPSPI2"]
            pub const ALT2: u32 = 0x02;
            #[doc = "Select mux mode: ALT3 mux port: LPSPI1_PCS3 of instance: LPSPI1"]
            pub const ALT3: u32 = 0x03;
            #[doc = "Select mux mode: ALT4 mux port: PIT_TRIGGER00 of instance: PIT"]
            pub const ALT4: u32 = 0x04;
            #[doc = "Select mux mode: ALT5 mux port: GPIOMUX_IO00 of instance: GPIOMUX"]
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
            #[doc = "Force input path of pad GPIO_00"]
            pub const ENABLED: u32 = 0x01;
        }
    }
}
#[doc = "SW_PAD_CTL_PAD_GPIO_AD_14 SW PAD Control Register"]
pub mod SW_PAD_CTL_PAD_GPIO_AD_14 {
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
            #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V, 240 Ohm for DDR)"]
            pub const DSE_1_R0_150_OHM___3_3V__260_OHM_1_8V__240_OHM_FOR_DDR_: u32 = 0x01;
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
            pub const SPEED_0_LOW_50MHZ: u32 = 0;
            #[doc = "medium(100MHz)"]
            pub const SPEED_1_MEDIUM_100MHZ: u32 = 0x01;
            #[doc = "fast(150MHz)"]
            pub const SPEED_2_FAST_150MHZ: u32 = 0x02;
            #[doc = "max(200MHz)"]
            pub const SPEED_3_MAX_200MHZ: u32 = 0x03;
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
#[doc = "SW_PAD_CTL_PAD_GPIO_AD_13 SW PAD Control Register"]
pub mod SW_PAD_CTL_PAD_GPIO_AD_13 {
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
            #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V, 240 Ohm for DDR)"]
            pub const DSE_1_R0_150_OHM___3_3V__260_OHM_1_8V__240_OHM_FOR_DDR_: u32 = 0x01;
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
            pub const SPEED_0_LOW_50MHZ: u32 = 0;
            #[doc = "medium(100MHz)"]
            pub const SPEED_1_MEDIUM_100MHZ: u32 = 0x01;
            #[doc = "fast(150MHz)"]
            pub const SPEED_2_FAST_150MHZ: u32 = 0x02;
            #[doc = "max(200MHz)"]
            pub const SPEED_3_MAX_200MHZ: u32 = 0x03;
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
#[doc = "SW_PAD_CTL_PAD_GPIO_AD_12 SW PAD Control Register"]
pub mod SW_PAD_CTL_PAD_GPIO_AD_12 {
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
            #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V, 240 Ohm for DDR)"]
            pub const DSE_1_R0_150_OHM___3_3V__260_OHM_1_8V__240_OHM_FOR_DDR_: u32 = 0x01;
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
            pub const SPEED_0_LOW_50MHZ: u32 = 0;
            #[doc = "medium(100MHz)"]
            pub const SPEED_1_MEDIUM_100MHZ: u32 = 0x01;
            #[doc = "fast(150MHz)"]
            pub const SPEED_2_FAST_150MHZ: u32 = 0x02;
            #[doc = "max(200MHz)"]
            pub const SPEED_3_MAX_200MHZ: u32 = 0x03;
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
#[doc = "SW_PAD_CTL_PAD_GPIO_AD_11 SW PAD Control Register"]
pub mod SW_PAD_CTL_PAD_GPIO_AD_11 {
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
            #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V, 240 Ohm for DDR)"]
            pub const DSE_1_R0_150_OHM___3_3V__260_OHM_1_8V__240_OHM_FOR_DDR_: u32 = 0x01;
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
            pub const SPEED_0_LOW_50MHZ: u32 = 0;
            #[doc = "medium(100MHz)"]
            pub const SPEED_1_MEDIUM_100MHZ: u32 = 0x01;
            #[doc = "fast(150MHz)"]
            pub const SPEED_2_FAST_150MHZ: u32 = 0x02;
            #[doc = "max(200MHz)"]
            pub const SPEED_3_MAX_200MHZ: u32 = 0x03;
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
#[doc = "SW_PAD_CTL_PAD_GPIO_AD_10 SW PAD Control Register"]
pub mod SW_PAD_CTL_PAD_GPIO_AD_10 {
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
            #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V, 240 Ohm for DDR)"]
            pub const DSE_1_R0_150_OHM___3_3V__260_OHM_1_8V__240_OHM_FOR_DDR_: u32 = 0x01;
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
            pub const SPEED_0_LOW_50MHZ: u32 = 0;
            #[doc = "medium(100MHz)"]
            pub const SPEED_1_MEDIUM_100MHZ: u32 = 0x01;
            #[doc = "fast(150MHz)"]
            pub const SPEED_2_FAST_150MHZ: u32 = 0x02;
            #[doc = "max(200MHz)"]
            pub const SPEED_3_MAX_200MHZ: u32 = 0x03;
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
#[doc = "SW_PAD_CTL_PAD_GPIO_AD_09 SW PAD Control Register"]
pub mod SW_PAD_CTL_PAD_GPIO_AD_09 {
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
            #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V, 240 Ohm for DDR)"]
            pub const DSE_1_R0_150_OHM___3_3V__260_OHM_1_8V__240_OHM_FOR_DDR_: u32 = 0x01;
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
            pub const SPEED_0_LOW_50MHZ: u32 = 0;
            #[doc = "medium(100MHz)"]
            pub const SPEED_1_MEDIUM_100MHZ: u32 = 0x01;
            #[doc = "fast(150MHz)"]
            pub const SPEED_2_FAST_150MHZ: u32 = 0x02;
            #[doc = "max(200MHz)"]
            pub const SPEED_3_MAX_200MHZ: u32 = 0x03;
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
#[doc = "SW_PAD_CTL_PAD_GPIO_AD_08 SW PAD Control Register"]
pub mod SW_PAD_CTL_PAD_GPIO_AD_08 {
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
            #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V, 240 Ohm for DDR)"]
            pub const DSE_1_R0_150_OHM___3_3V__260_OHM_1_8V__240_OHM_FOR_DDR_: u32 = 0x01;
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
            pub const SPEED_0_LOW_50MHZ: u32 = 0;
            #[doc = "medium(100MHz)"]
            pub const SPEED_1_MEDIUM_100MHZ: u32 = 0x01;
            #[doc = "fast(150MHz)"]
            pub const SPEED_2_FAST_150MHZ: u32 = 0x02;
            #[doc = "max(200MHz)"]
            pub const SPEED_3_MAX_200MHZ: u32 = 0x03;
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
#[doc = "SW_PAD_CTL_PAD_GPIO_AD_07 SW PAD Control Register"]
pub mod SW_PAD_CTL_PAD_GPIO_AD_07 {
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
            #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V, 240 Ohm for DDR)"]
            pub const DSE_1_R0_150_OHM___3_3V__260_OHM_1_8V__240_OHM_FOR_DDR_: u32 = 0x01;
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
            pub const SPEED_0_LOW_50MHZ: u32 = 0;
            #[doc = "medium(100MHz)"]
            pub const SPEED_1_MEDIUM_100MHZ: u32 = 0x01;
            #[doc = "fast(150MHz)"]
            pub const SPEED_2_FAST_150MHZ: u32 = 0x02;
            #[doc = "max(200MHz)"]
            pub const SPEED_3_MAX_200MHZ: u32 = 0x03;
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
#[doc = "SW_PAD_CTL_PAD_GPIO_AD_06 SW PAD Control Register"]
pub mod SW_PAD_CTL_PAD_GPIO_AD_06 {
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
            #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V, 240 Ohm for DDR)"]
            pub const DSE_1_R0_150_OHM___3_3V__260_OHM_1_8V__240_OHM_FOR_DDR_: u32 = 0x01;
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
            pub const SPEED_0_LOW_50MHZ: u32 = 0;
            #[doc = "medium(100MHz)"]
            pub const SPEED_1_MEDIUM_100MHZ: u32 = 0x01;
            #[doc = "fast(150MHz)"]
            pub const SPEED_2_FAST_150MHZ: u32 = 0x02;
            #[doc = "max(200MHz)"]
            pub const SPEED_3_MAX_200MHZ: u32 = 0x03;
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
#[doc = "SW_PAD_CTL_PAD_GPIO_AD_05 SW PAD Control Register"]
pub mod SW_PAD_CTL_PAD_GPIO_AD_05 {
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
            #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V, 240 Ohm for DDR)"]
            pub const DSE_1_R0_150_OHM___3_3V__260_OHM_1_8V__240_OHM_FOR_DDR_: u32 = 0x01;
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
            pub const SPEED_0_LOW_50MHZ: u32 = 0;
            #[doc = "medium(100MHz)"]
            pub const SPEED_1_MEDIUM_100MHZ: u32 = 0x01;
            #[doc = "fast(150MHz)"]
            pub const SPEED_2_FAST_150MHZ: u32 = 0x02;
            #[doc = "max(200MHz)"]
            pub const SPEED_3_MAX_200MHZ: u32 = 0x03;
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
#[doc = "SW_PAD_CTL_PAD_GPIO_AD_04 SW PAD Control Register"]
pub mod SW_PAD_CTL_PAD_GPIO_AD_04 {
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
            #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V, 240 Ohm for DDR)"]
            pub const DSE_1_R0_150_OHM___3_3V__260_OHM_1_8V__240_OHM_FOR_DDR_: u32 = 0x01;
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
            pub const SPEED_0_LOW_50MHZ: u32 = 0;
            #[doc = "medium(100MHz)"]
            pub const SPEED_1_MEDIUM_100MHZ: u32 = 0x01;
            #[doc = "fast(150MHz)"]
            pub const SPEED_2_FAST_150MHZ: u32 = 0x02;
            #[doc = "max(200MHz)"]
            pub const SPEED_3_MAX_200MHZ: u32 = 0x03;
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
#[doc = "SW_PAD_CTL_PAD_GPIO_AD_03 SW PAD Control Register"]
pub mod SW_PAD_CTL_PAD_GPIO_AD_03 {
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
            #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V, 240 Ohm for DDR)"]
            pub const DSE_1_R0_150_OHM___3_3V__260_OHM_1_8V__240_OHM_FOR_DDR_: u32 = 0x01;
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
            pub const SPEED_0_LOW_50MHZ: u32 = 0;
            #[doc = "medium(100MHz)"]
            pub const SPEED_1_MEDIUM_100MHZ: u32 = 0x01;
            #[doc = "fast(150MHz)"]
            pub const SPEED_2_FAST_150MHZ: u32 = 0x02;
            #[doc = "max(200MHz)"]
            pub const SPEED_3_MAX_200MHZ: u32 = 0x03;
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
#[doc = "SW_PAD_CTL_PAD_GPIO_AD_02 SW PAD Control Register"]
pub mod SW_PAD_CTL_PAD_GPIO_AD_02 {
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
            #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V, 240 Ohm for DDR)"]
            pub const DSE_1_R0_150_OHM___3_3V__260_OHM_1_8V__240_OHM_FOR_DDR_: u32 = 0x01;
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
            pub const SPEED_0_LOW_50MHZ: u32 = 0;
            #[doc = "medium(100MHz)"]
            pub const SPEED_1_MEDIUM_100MHZ: u32 = 0x01;
            #[doc = "fast(150MHz)"]
            pub const SPEED_2_FAST_150MHZ: u32 = 0x02;
            #[doc = "max(200MHz)"]
            pub const SPEED_3_MAX_200MHZ: u32 = 0x03;
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
#[doc = "SW_PAD_CTL_PAD_GPIO_AD_01 SW PAD Control Register"]
pub mod SW_PAD_CTL_PAD_GPIO_AD_01 {
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
            #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V, 240 Ohm for DDR)"]
            pub const DSE_1_R0_150_OHM___3_3V__260_OHM_1_8V__240_OHM_FOR_DDR_: u32 = 0x01;
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
            pub const SPEED_0_LOW_50MHZ: u32 = 0;
            #[doc = "medium(100MHz)"]
            pub const SPEED_1_MEDIUM_100MHZ: u32 = 0x01;
            #[doc = "fast(150MHz)"]
            pub const SPEED_2_FAST_150MHZ: u32 = 0x02;
            #[doc = "max(200MHz)"]
            pub const SPEED_3_MAX_200MHZ: u32 = 0x03;
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
#[doc = "SW_PAD_CTL_PAD_GPIO_AD_00 SW PAD Control Register"]
pub mod SW_PAD_CTL_PAD_GPIO_AD_00 {
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
            #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V, 240 Ohm for DDR)"]
            pub const DSE_1_R0_150_OHM___3_3V__260_OHM_1_8V__240_OHM_FOR_DDR_: u32 = 0x01;
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
            pub const SPEED_0_LOW_50MHZ: u32 = 0;
            #[doc = "medium(100MHz)"]
            pub const SPEED_1_MEDIUM_100MHZ: u32 = 0x01;
            #[doc = "fast(150MHz)"]
            pub const SPEED_2_FAST_150MHZ: u32 = 0x02;
            #[doc = "max(200MHz)"]
            pub const SPEED_3_MAX_200MHZ: u32 = 0x03;
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
#[doc = "SW_PAD_CTL_PAD_GPIO_SD_14 SW PAD Control Register"]
pub mod SW_PAD_CTL_PAD_GPIO_SD_14 {
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
            #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V, 240 Ohm for DDR)"]
            pub const DSE_1_R0_150_OHM___3_3V__260_OHM_1_8V__240_OHM_FOR_DDR_: u32 = 0x01;
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
            pub const SPEED_0_LOW_50MHZ: u32 = 0;
            #[doc = "medium(100MHz)"]
            pub const SPEED_1_MEDIUM_100MHZ: u32 = 0x01;
            #[doc = "fast(150MHz)"]
            pub const SPEED_2_FAST_150MHZ: u32 = 0x02;
            #[doc = "max(200MHz)"]
            pub const SPEED_3_MAX_200MHZ: u32 = 0x03;
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
#[doc = "SW_PAD_CTL_PAD_GPIO_SD_13 SW PAD Control Register"]
pub mod SW_PAD_CTL_PAD_GPIO_SD_13 {
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
            #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V, 240 Ohm for DDR)"]
            pub const DSE_1_R0_150_OHM___3_3V__260_OHM_1_8V__240_OHM_FOR_DDR_: u32 = 0x01;
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
            pub const SPEED_0_LOW_50MHZ: u32 = 0;
            #[doc = "medium(100MHz)"]
            pub const SPEED_1_MEDIUM_100MHZ: u32 = 0x01;
            #[doc = "fast(150MHz)"]
            pub const SPEED_2_FAST_150MHZ: u32 = 0x02;
            #[doc = "max(200MHz)"]
            pub const SPEED_3_MAX_200MHZ: u32 = 0x03;
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
#[doc = "SW_PAD_CTL_PAD_GPIO_SD_12 SW PAD Control Register"]
pub mod SW_PAD_CTL_PAD_GPIO_SD_12 {
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
            #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V, 240 Ohm for DDR)"]
            pub const DSE_1_R0_150_OHM___3_3V__260_OHM_1_8V__240_OHM_FOR_DDR_: u32 = 0x01;
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
            pub const SPEED_0_LOW_50MHZ: u32 = 0;
            #[doc = "medium(100MHz)"]
            pub const SPEED_1_MEDIUM_100MHZ: u32 = 0x01;
            #[doc = "fast(150MHz)"]
            pub const SPEED_2_FAST_150MHZ: u32 = 0x02;
            #[doc = "max(200MHz)"]
            pub const SPEED_3_MAX_200MHZ: u32 = 0x03;
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
#[doc = "SW_PAD_CTL_PAD_GPIO_SD_11 SW PAD Control Register"]
pub mod SW_PAD_CTL_PAD_GPIO_SD_11 {
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
            #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V, 240 Ohm for DDR)"]
            pub const DSE_1_R0_150_OHM___3_3V__260_OHM_1_8V__240_OHM_FOR_DDR_: u32 = 0x01;
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
            pub const SPEED_0_LOW_50MHZ: u32 = 0;
            #[doc = "medium(100MHz)"]
            pub const SPEED_1_MEDIUM_100MHZ: u32 = 0x01;
            #[doc = "fast(150MHz)"]
            pub const SPEED_2_FAST_150MHZ: u32 = 0x02;
            #[doc = "max(200MHz)"]
            pub const SPEED_3_MAX_200MHZ: u32 = 0x03;
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
#[doc = "SW_PAD_CTL_PAD_GPIO_SD_10 SW PAD Control Register"]
pub mod SW_PAD_CTL_PAD_GPIO_SD_10 {
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
            #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V, 240 Ohm for DDR)"]
            pub const DSE_1_R0_150_OHM___3_3V__260_OHM_1_8V__240_OHM_FOR_DDR_: u32 = 0x01;
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
            pub const SPEED_0_LOW_50MHZ: u32 = 0;
            #[doc = "medium(100MHz)"]
            pub const SPEED_1_MEDIUM_100MHZ: u32 = 0x01;
            #[doc = "fast(150MHz)"]
            pub const SPEED_2_FAST_150MHZ: u32 = 0x02;
            #[doc = "max(200MHz)"]
            pub const SPEED_3_MAX_200MHZ: u32 = 0x03;
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
#[doc = "SW_PAD_CTL_PAD_GPIO_SD_09 SW PAD Control Register"]
pub mod SW_PAD_CTL_PAD_GPIO_SD_09 {
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
            #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V, 240 Ohm for DDR)"]
            pub const DSE_1_R0_150_OHM___3_3V__260_OHM_1_8V__240_OHM_FOR_DDR_: u32 = 0x01;
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
            pub const SPEED_0_LOW_50MHZ: u32 = 0;
            #[doc = "medium(100MHz)"]
            pub const SPEED_1_MEDIUM_100MHZ: u32 = 0x01;
            #[doc = "fast(150MHz)"]
            pub const SPEED_2_FAST_150MHZ: u32 = 0x02;
            #[doc = "max(200MHz)"]
            pub const SPEED_3_MAX_200MHZ: u32 = 0x03;
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
#[doc = "SW_PAD_CTL_PAD_GPIO_SD_08 SW PAD Control Register"]
pub mod SW_PAD_CTL_PAD_GPIO_SD_08 {
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
            #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V, 240 Ohm for DDR)"]
            pub const DSE_1_R0_150_OHM___3_3V__260_OHM_1_8V__240_OHM_FOR_DDR_: u32 = 0x01;
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
            pub const SPEED_0_LOW_50MHZ: u32 = 0;
            #[doc = "medium(100MHz)"]
            pub const SPEED_1_MEDIUM_100MHZ: u32 = 0x01;
            #[doc = "fast(150MHz)"]
            pub const SPEED_2_FAST_150MHZ: u32 = 0x02;
            #[doc = "max(200MHz)"]
            pub const SPEED_3_MAX_200MHZ: u32 = 0x03;
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
#[doc = "SW_PAD_CTL_PAD_GPIO_SD_07 SW PAD Control Register"]
pub mod SW_PAD_CTL_PAD_GPIO_SD_07 {
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
            #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V, 240 Ohm for DDR)"]
            pub const DSE_1_R0_150_OHM___3_3V__260_OHM_1_8V__240_OHM_FOR_DDR_: u32 = 0x01;
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
            pub const SPEED_0_LOW_50MHZ: u32 = 0;
            #[doc = "medium(100MHz)"]
            pub const SPEED_1_MEDIUM_100MHZ: u32 = 0x01;
            #[doc = "fast(150MHz)"]
            pub const SPEED_2_FAST_150MHZ: u32 = 0x02;
            #[doc = "max(200MHz)"]
            pub const SPEED_3_MAX_200MHZ: u32 = 0x03;
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
#[doc = "SW_PAD_CTL_PAD_GPIO_SD_06 SW PAD Control Register"]
pub mod SW_PAD_CTL_PAD_GPIO_SD_06 {
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
            #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V, 240 Ohm for DDR)"]
            pub const DSE_1_R0_150_OHM___3_3V__260_OHM_1_8V__240_OHM_FOR_DDR_: u32 = 0x01;
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
            pub const SPEED_0_LOW_50MHZ: u32 = 0;
            #[doc = "medium(100MHz)"]
            pub const SPEED_1_MEDIUM_100MHZ: u32 = 0x01;
            #[doc = "fast(150MHz)"]
            pub const SPEED_2_FAST_150MHZ: u32 = 0x02;
            #[doc = "max(200MHz)"]
            pub const SPEED_3_MAX_200MHZ: u32 = 0x03;
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
#[doc = "SW_PAD_CTL_PAD_GPIO_SD_05 SW PAD Control Register"]
pub mod SW_PAD_CTL_PAD_GPIO_SD_05 {
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
            #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V, 240 Ohm for DDR)"]
            pub const DSE_1_R0_150_OHM___3_3V__260_OHM_1_8V__240_OHM_FOR_DDR_: u32 = 0x01;
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
            pub const SPEED_0_LOW_50MHZ: u32 = 0;
            #[doc = "medium(100MHz)"]
            pub const SPEED_1_MEDIUM_100MHZ: u32 = 0x01;
            #[doc = "fast(150MHz)"]
            pub const SPEED_2_FAST_150MHZ: u32 = 0x02;
            #[doc = "max(200MHz)"]
            pub const SPEED_3_MAX_200MHZ: u32 = 0x03;
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
#[doc = "SW_PAD_CTL_PAD_GPIO_SD_04 SW PAD Control Register"]
pub mod SW_PAD_CTL_PAD_GPIO_SD_04 {
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
            #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V, 240 Ohm for DDR)"]
            pub const DSE_1_R0_150_OHM___3_3V__260_OHM_1_8V__240_OHM_FOR_DDR_: u32 = 0x01;
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
            pub const SPEED_0_LOW_50MHZ: u32 = 0;
            #[doc = "medium(100MHz)"]
            pub const SPEED_1_MEDIUM_100MHZ: u32 = 0x01;
            #[doc = "fast(150MHz)"]
            pub const SPEED_2_FAST_150MHZ: u32 = 0x02;
            #[doc = "max(200MHz)"]
            pub const SPEED_3_MAX_200MHZ: u32 = 0x03;
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
#[doc = "SW_PAD_CTL_PAD_GPIO_SD_03 SW PAD Control Register"]
pub mod SW_PAD_CTL_PAD_GPIO_SD_03 {
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
            #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V, 240 Ohm for DDR)"]
            pub const DSE_1_R0_150_OHM___3_3V__260_OHM_1_8V__240_OHM_FOR_DDR_: u32 = 0x01;
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
            pub const SPEED_0_LOW_50MHZ: u32 = 0;
            #[doc = "medium(100MHz)"]
            pub const SPEED_1_MEDIUM_100MHZ: u32 = 0x01;
            #[doc = "fast(150MHz)"]
            pub const SPEED_2_FAST_150MHZ: u32 = 0x02;
            #[doc = "max(200MHz)"]
            pub const SPEED_3_MAX_200MHZ: u32 = 0x03;
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
#[doc = "SW_PAD_CTL_PAD_GPIO_SD_02 SW PAD Control Register"]
pub mod SW_PAD_CTL_PAD_GPIO_SD_02 {
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
            #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V, 240 Ohm for DDR)"]
            pub const DSE_1_R0_150_OHM___3_3V__260_OHM_1_8V__240_OHM_FOR_DDR_: u32 = 0x01;
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
            pub const SPEED_0_LOW_50MHZ: u32 = 0;
            #[doc = "medium(100MHz)"]
            pub const SPEED_1_MEDIUM_100MHZ: u32 = 0x01;
            #[doc = "fast(150MHz)"]
            pub const SPEED_2_FAST_150MHZ: u32 = 0x02;
            #[doc = "max(200MHz)"]
            pub const SPEED_3_MAX_200MHZ: u32 = 0x03;
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
#[doc = "SW_PAD_CTL_PAD_GPIO_SD_01 SW PAD Control Register"]
pub mod SW_PAD_CTL_PAD_GPIO_SD_01 {
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
            #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V, 240 Ohm for DDR)"]
            pub const DSE_1_R0_150_OHM___3_3V__260_OHM_1_8V__240_OHM_FOR_DDR_: u32 = 0x01;
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
            pub const SPEED_0_LOW_50MHZ: u32 = 0;
            #[doc = "medium(100MHz)"]
            pub const SPEED_1_MEDIUM_100MHZ: u32 = 0x01;
            #[doc = "fast(150MHz)"]
            pub const SPEED_2_FAST_150MHZ: u32 = 0x02;
            #[doc = "max(200MHz)"]
            pub const SPEED_3_MAX_200MHZ: u32 = 0x03;
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
#[doc = "SW_PAD_CTL_PAD_GPIO_SD_00 SW PAD Control Register"]
pub mod SW_PAD_CTL_PAD_GPIO_SD_00 {
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
            #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V, 240 Ohm for DDR)"]
            pub const DSE_1_R0_150_OHM___3_3V__260_OHM_1_8V__240_OHM_FOR_DDR_: u32 = 0x01;
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
            pub const SPEED_0_LOW_50MHZ: u32 = 0;
            #[doc = "medium(100MHz)"]
            pub const SPEED_1_MEDIUM_100MHZ: u32 = 0x01;
            #[doc = "fast(150MHz)"]
            pub const SPEED_2_FAST_150MHZ: u32 = 0x02;
            #[doc = "max(200MHz)"]
            pub const SPEED_3_MAX_200MHZ: u32 = 0x03;
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
#[doc = "SW_PAD_CTL_PAD_GPIO_13 SW PAD Control Register"]
pub mod SW_PAD_CTL_PAD_GPIO_13 {
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
            #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V, 240 Ohm for DDR)"]
            pub const DSE_1_R0_150_OHM___3_3V__260_OHM_1_8V__240_OHM_FOR_DDR_: u32 = 0x01;
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
            pub const SPEED_0_LOW_50MHZ: u32 = 0;
            #[doc = "medium(100MHz)"]
            pub const SPEED_1_MEDIUM_100MHZ: u32 = 0x01;
            #[doc = "fast(150MHz)"]
            pub const SPEED_2_FAST_150MHZ: u32 = 0x02;
            #[doc = "max(200MHz)"]
            pub const SPEED_3_MAX_200MHZ: u32 = 0x03;
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
#[doc = "SW_PAD_CTL_PAD_GPIO_12 SW PAD Control Register"]
pub mod SW_PAD_CTL_PAD_GPIO_12 {
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
            #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V, 240 Ohm for DDR)"]
            pub const DSE_1_R0_150_OHM___3_3V__260_OHM_1_8V__240_OHM_FOR_DDR_: u32 = 0x01;
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
            pub const SPEED_0_LOW_50MHZ: u32 = 0;
            #[doc = "medium(100MHz)"]
            pub const SPEED_1_MEDIUM_100MHZ: u32 = 0x01;
            #[doc = "fast(150MHz)"]
            pub const SPEED_2_FAST_150MHZ: u32 = 0x02;
            #[doc = "max(200MHz)"]
            pub const SPEED_3_MAX_200MHZ: u32 = 0x03;
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
#[doc = "SW_PAD_CTL_PAD_GPIO_11 SW PAD Control Register"]
pub mod SW_PAD_CTL_PAD_GPIO_11 {
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
            #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V, 240 Ohm for DDR)"]
            pub const DSE_1_R0_150_OHM___3_3V__260_OHM_1_8V__240_OHM_FOR_DDR_: u32 = 0x01;
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
            pub const SPEED_0_LOW_50MHZ: u32 = 0;
            #[doc = "medium(100MHz)"]
            pub const SPEED_1_MEDIUM_100MHZ: u32 = 0x01;
            #[doc = "fast(150MHz)"]
            pub const SPEED_2_FAST_150MHZ: u32 = 0x02;
            #[doc = "max(200MHz)"]
            pub const SPEED_3_MAX_200MHZ: u32 = 0x03;
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
#[doc = "SW_PAD_CTL_PAD_GPIO_10 SW PAD Control Register"]
pub mod SW_PAD_CTL_PAD_GPIO_10 {
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
            #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V, 240 Ohm for DDR)"]
            pub const DSE_1_R0_150_OHM___3_3V__260_OHM_1_8V__240_OHM_FOR_DDR_: u32 = 0x01;
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
            pub const SPEED_0_LOW_50MHZ: u32 = 0;
            #[doc = "medium(100MHz)"]
            pub const SPEED_1_MEDIUM_100MHZ: u32 = 0x01;
            #[doc = "fast(150MHz)"]
            pub const SPEED_2_FAST_150MHZ: u32 = 0x02;
            #[doc = "max(200MHz)"]
            pub const SPEED_3_MAX_200MHZ: u32 = 0x03;
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
#[doc = "SW_PAD_CTL_PAD_GPIO_09 SW PAD Control Register"]
pub mod SW_PAD_CTL_PAD_GPIO_09 {
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
            #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V, 240 Ohm for DDR)"]
            pub const DSE_1_R0_150_OHM___3_3V__260_OHM_1_8V__240_OHM_FOR_DDR_: u32 = 0x01;
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
            pub const SPEED_0_LOW_50MHZ: u32 = 0;
            #[doc = "medium(100MHz)"]
            pub const SPEED_1_MEDIUM_100MHZ: u32 = 0x01;
            #[doc = "fast(150MHz)"]
            pub const SPEED_2_FAST_150MHZ: u32 = 0x02;
            #[doc = "max(200MHz)"]
            pub const SPEED_3_MAX_200MHZ: u32 = 0x03;
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
#[doc = "SW_PAD_CTL_PAD_GPIO_08 SW PAD Control Register"]
pub mod SW_PAD_CTL_PAD_GPIO_08 {
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
            #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V, 240 Ohm for DDR)"]
            pub const DSE_1_R0_150_OHM___3_3V__260_OHM_1_8V__240_OHM_FOR_DDR_: u32 = 0x01;
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
            pub const SPEED_0_LOW_50MHZ: u32 = 0;
            #[doc = "medium(100MHz)"]
            pub const SPEED_1_MEDIUM_100MHZ: u32 = 0x01;
            #[doc = "fast(150MHz)"]
            pub const SPEED_2_FAST_150MHZ: u32 = 0x02;
            #[doc = "max(200MHz)"]
            pub const SPEED_3_MAX_200MHZ: u32 = 0x03;
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
#[doc = "SW_PAD_CTL_PAD_GPIO_07 SW PAD Control Register"]
pub mod SW_PAD_CTL_PAD_GPIO_07 {
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
            #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V, 240 Ohm for DDR)"]
            pub const DSE_1_R0_150_OHM___3_3V__260_OHM_1_8V__240_OHM_FOR_DDR_: u32 = 0x01;
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
            pub const SPEED_0_LOW_50MHZ: u32 = 0;
            #[doc = "medium(100MHz)"]
            pub const SPEED_1_MEDIUM_100MHZ: u32 = 0x01;
            #[doc = "fast(150MHz)"]
            pub const SPEED_2_FAST_150MHZ: u32 = 0x02;
            #[doc = "max(200MHz)"]
            pub const SPEED_3_MAX_200MHZ: u32 = 0x03;
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
#[doc = "SW_PAD_CTL_PAD_GPIO_06 SW PAD Control Register"]
pub mod SW_PAD_CTL_PAD_GPIO_06 {
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
            #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V, 240 Ohm for DDR)"]
            pub const DSE_1_R0_150_OHM___3_3V__260_OHM_1_8V__240_OHM_FOR_DDR_: u32 = 0x01;
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
            pub const SPEED_0_LOW_50MHZ: u32 = 0;
            #[doc = "medium(100MHz)"]
            pub const SPEED_1_MEDIUM_100MHZ: u32 = 0x01;
            #[doc = "fast(150MHz)"]
            pub const SPEED_2_FAST_150MHZ: u32 = 0x02;
            #[doc = "max(200MHz)"]
            pub const SPEED_3_MAX_200MHZ: u32 = 0x03;
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
#[doc = "SW_PAD_CTL_PAD_GPIO_05 SW PAD Control Register"]
pub mod SW_PAD_CTL_PAD_GPIO_05 {
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
            #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V, 240 Ohm for DDR)"]
            pub const DSE_1_R0_150_OHM___3_3V__260_OHM_1_8V__240_OHM_FOR_DDR_: u32 = 0x01;
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
            pub const SPEED_0_LOW_50MHZ: u32 = 0;
            #[doc = "medium(100MHz)"]
            pub const SPEED_1_MEDIUM_100MHZ: u32 = 0x01;
            #[doc = "fast(150MHz)"]
            pub const SPEED_2_FAST_150MHZ: u32 = 0x02;
            #[doc = "max(200MHz)"]
            pub const SPEED_3_MAX_200MHZ: u32 = 0x03;
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
#[doc = "SW_PAD_CTL_PAD_GPIO_04 SW PAD Control Register"]
pub mod SW_PAD_CTL_PAD_GPIO_04 {
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
            #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V, 240 Ohm for DDR)"]
            pub const DSE_1_R0_150_OHM___3_3V__260_OHM_1_8V__240_OHM_FOR_DDR_: u32 = 0x01;
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
            pub const SPEED_0_LOW_50MHZ: u32 = 0;
            #[doc = "medium(100MHz)"]
            pub const SPEED_1_MEDIUM_100MHZ: u32 = 0x01;
            #[doc = "fast(150MHz)"]
            pub const SPEED_2_FAST_150MHZ: u32 = 0x02;
            #[doc = "max(200MHz)"]
            pub const SPEED_3_MAX_200MHZ: u32 = 0x03;
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
#[doc = "SW_PAD_CTL_PAD_GPIO_03 SW PAD Control Register"]
pub mod SW_PAD_CTL_PAD_GPIO_03 {
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
            #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V, 240 Ohm for DDR)"]
            pub const DSE_1_R0_150_OHM___3_3V__260_OHM_1_8V__240_OHM_FOR_DDR_: u32 = 0x01;
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
            pub const SPEED_0_LOW_50MHZ: u32 = 0;
            #[doc = "medium(100MHz)"]
            pub const SPEED_1_MEDIUM_100MHZ: u32 = 0x01;
            #[doc = "fast(150MHz)"]
            pub const SPEED_2_FAST_150MHZ: u32 = 0x02;
            #[doc = "max(200MHz)"]
            pub const SPEED_3_MAX_200MHZ: u32 = 0x03;
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
#[doc = "SW_PAD_CTL_PAD_GPIO_02 SW PAD Control Register"]
pub mod SW_PAD_CTL_PAD_GPIO_02 {
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
            #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V, 240 Ohm for DDR)"]
            pub const DSE_1_R0_150_OHM___3_3V__260_OHM_1_8V__240_OHM_FOR_DDR_: u32 = 0x01;
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
            pub const SPEED_0_LOW_50MHZ: u32 = 0;
            #[doc = "medium(100MHz)"]
            pub const SPEED_1_MEDIUM_100MHZ: u32 = 0x01;
            #[doc = "fast(150MHz)"]
            pub const SPEED_2_FAST_150MHZ: u32 = 0x02;
            #[doc = "max(200MHz)"]
            pub const SPEED_3_MAX_200MHZ: u32 = 0x03;
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
#[doc = "SW_PAD_CTL_PAD_GPIO_01 SW PAD Control Register"]
pub mod SW_PAD_CTL_PAD_GPIO_01 {
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
            #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V, 240 Ohm for DDR)"]
            pub const DSE_1_R0_150_OHM___3_3V__260_OHM_1_8V__240_OHM_FOR_DDR_: u32 = 0x01;
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
            pub const SPEED_0_LOW_50MHZ: u32 = 0;
            #[doc = "medium(100MHz)"]
            pub const SPEED_1_MEDIUM_100MHZ: u32 = 0x01;
            #[doc = "fast(150MHz)"]
            pub const SPEED_2_FAST_150MHZ: u32 = 0x02;
            #[doc = "max(200MHz)"]
            pub const SPEED_3_MAX_200MHZ: u32 = 0x03;
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
#[doc = "SW_PAD_CTL_PAD_GPIO_00 SW PAD Control Register"]
pub mod SW_PAD_CTL_PAD_GPIO_00 {
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
            #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V, 240 Ohm for DDR)"]
            pub const DSE_1_R0_150_OHM___3_3V__260_OHM_1_8V__240_OHM_FOR_DDR_: u32 = 0x01;
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
            pub const SPEED_0_LOW_50MHZ: u32 = 0;
            #[doc = "medium(100MHz)"]
            pub const SPEED_1_MEDIUM_100MHZ: u32 = 0x01;
            #[doc = "fast(150MHz)"]
            pub const SPEED_2_FAST_150MHZ: u32 = 0x02;
            #[doc = "max(200MHz)"]
            pub const SPEED_3_MAX_200MHZ: u32 = 0x03;
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
#[doc = "USB_OTG_ID_SELECT_INPUT DAISY Register"]
pub mod USB_OTG_ID_SELECT_INPUT {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    pub mod DAISY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Selecting Pad: GPIO_AD_10 for Mode: ALT6"]
            pub const GPIO_AD_10_ALT6: u32 = 0;
            #[doc = "Selecting Pad: GPIO_13 for Mode: ALT3"]
            pub const GPIO_13_ALT3: u32 = 0x01;
        }
    }
}
#[doc = "FLEXPWM1_PWMA_SELECT_INPUT_0 DAISY Register"]
pub mod FLEXPWM1_PWMA_SELECT_INPUT_0 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    pub mod DAISY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Selecting Pad: GPIO_SD_02 for Mode: ALT2"]
            pub const GPIO_SD_02_ALT2: u32 = 0;
            #[doc = "Selecting Pad: GPIO_02 for Mode: ALT2"]
            pub const GPIO_02_ALT2: u32 = 0x01;
        }
    }
}
#[doc = "FLEXPWM1_PWMA_SELECT_INPUT_1 DAISY Register"]
pub mod FLEXPWM1_PWMA_SELECT_INPUT_1 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    pub mod DAISY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Selecting Pad: GPIO_SD_04 for Mode: ALT2"]
            pub const GPIO_SD_04_ALT2: u32 = 0;
            #[doc = "Selecting Pad: GPIO_04 for Mode: ALT2"]
            pub const GPIO_04_ALT2: u32 = 0x01;
        }
    }
}
#[doc = "FLEXPWM1_PWMA_SELECT_INPUT_2 DAISY Register"]
pub mod FLEXPWM1_PWMA_SELECT_INPUT_2 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    pub mod DAISY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Selecting Pad: GPIO_AD_04 for Mode: ALT2"]
            pub const GPIO_AD_04_ALT2: u32 = 0;
            #[doc = "Selecting Pad: GPIO_06 for Mode: ALT2"]
            pub const GPIO_06_ALT2: u32 = 0x01;
        }
    }
}
#[doc = "FLEXPWM1_PWMA_SELECT_INPUT_3 DAISY Register"]
pub mod FLEXPWM1_PWMA_SELECT_INPUT_3 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    pub mod DAISY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Selecting Pad: GPIO_AD_06 for Mode: ALT2"]
            pub const GPIO_AD_06_ALT2: u32 = 0;
            #[doc = "Selecting Pad: GPIO_08 for Mode: ALT2"]
            pub const GPIO_08_ALT2: u32 = 0x01;
        }
    }
}
#[doc = "FLEXPWM1_PWMB_SELECT_INPUT_0 DAISY Register"]
pub mod FLEXPWM1_PWMB_SELECT_INPUT_0 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    pub mod DAISY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Selecting Pad: GPIO_SD_01 for Mode: ALT2"]
            pub const GPIO_SD_01_ALT2: u32 = 0;
            #[doc = "Selecting Pad: GPIO_01 for Mode: ALT2"]
            pub const GPIO_01_ALT2: u32 = 0x01;
        }
    }
}
#[doc = "FLEXPWM1_PWMB_SELECT_INPUT_1 DAISY Register"]
pub mod FLEXPWM1_PWMB_SELECT_INPUT_1 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    pub mod DAISY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Selecting Pad: GPIO_SD_03 for Mode: ALT2"]
            pub const GPIO_SD_03_ALT2: u32 = 0;
            #[doc = "Selecting Pad: GPIO_03 for Mode: ALT2"]
            pub const GPIO_03_ALT2: u32 = 0x01;
        }
    }
}
#[doc = "FLEXPWM1_PWMB_SELECT_INPUT_2 DAISY Register"]
pub mod FLEXPWM1_PWMB_SELECT_INPUT_2 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    pub mod DAISY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Selecting Pad: GPIO_AD_03 for Mode: ALT2"]
            pub const GPIO_AD_03_ALT2: u32 = 0;
            #[doc = "Selecting Pad: GPIO_05 for Mode: ALT2"]
            pub const GPIO_05_ALT2: u32 = 0x01;
        }
    }
}
#[doc = "FLEXPWM1_PWMB_SELECT_INPUT_3 DAISY Register"]
pub mod FLEXPWM1_PWMB_SELECT_INPUT_3 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    pub mod DAISY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Selecting Pad: GPIO_AD_05 for Mode: ALT2"]
            pub const GPIO_AD_05_ALT2: u32 = 0;
            #[doc = "Selecting Pad: GPIO_07 for Mode: ALT2"]
            pub const GPIO_07_ALT2: u32 = 0x01;
        }
    }
}
#[doc = "FLEXSPI_DQS_FA_SELECT_INPUT DAISY Register"]
pub mod FLEXSPI_DQS_FA_SELECT_INPUT {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    pub mod DAISY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Selecting Pad: GPIO_SD_14 for Mode: ALT0"]
            pub const GPIO_SD_14_ALT0: u32 = 0;
            #[doc = "Selecting Pad: GPIO_SD_12 for Mode: ALT0"]
            pub const GPIO_SD_12_ALT0: u32 = 0x01;
        }
    }
}
#[doc = "FLEXSPI_DQS_FB_SELECT_INPUT DAISY Register"]
pub mod FLEXSPI_DQS_FB_SELECT_INPUT {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    pub mod DAISY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Selecting Pad: GPIO_SD_14 for Mode: ALT1"]
            pub const GPIO_SD_14_ALT1: u32 = 0;
            #[doc = "Selecting Pad: GPIO_00 for Mode: ALT0"]
            pub const GPIO_00_ALT0: u32 = 0x01;
        }
    }
}
#[doc = "KPP_COL_SELECT_INPUT_0 DAISY Register"]
pub mod KPP_COL_SELECT_INPUT_0 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    pub mod DAISY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Selecting Pad: GPIO_AD_14 for Mode: ALT2"]
            pub const GPIO_AD_14_ALT2: u32 = 0;
            #[doc = "Selecting Pad: GPIO_12 for Mode: ALT2"]
            pub const GPIO_12_ALT2: u32 = 0x01;
        }
    }
}
#[doc = "KPP_COL_SELECT_INPUT_1 DAISY Register"]
pub mod KPP_COL_SELECT_INPUT_1 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    pub mod DAISY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Selecting Pad: GPIO_AD_12 for Mode: ALT2"]
            pub const GPIO_AD_12_ALT2: u32 = 0;
            #[doc = "Selecting Pad: GPIO_AD_06 for Mode: ALT3"]
            pub const GPIO_AD_06_ALT3: u32 = 0x01;
        }
    }
}
#[doc = "KPP_COL_SELECT_INPUT_2 DAISY Register"]
pub mod KPP_COL_SELECT_INPUT_2 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    pub mod DAISY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Selecting Pad: GPIO_AD_10 for Mode: ALT2"]
            pub const GPIO_AD_10_ALT2: u32 = 0;
            #[doc = "Selecting Pad: GPIO_AD_04 for Mode: ALT3"]
            pub const GPIO_AD_04_ALT3: u32 = 0x01;
        }
    }
}
#[doc = "KPP_COL_SELECT_INPUT_3 DAISY Register"]
pub mod KPP_COL_SELECT_INPUT_3 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    pub mod DAISY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Selecting Pad: GPIO_AD_00 for Mode: ALT2"]
            pub const GPIO_AD_00_ALT2: u32 = 0;
            #[doc = "Selecting Pad: GPIO_02 for Mode: ALT4"]
            pub const GPIO_02_ALT4: u32 = 0x01;
        }
    }
}
#[doc = "KPP_ROW_SELECT_INPUT_0 DAISY Register"]
pub mod KPP_ROW_SELECT_INPUT_0 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    pub mod DAISY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Selecting Pad: GPIO_AD_13 for Mode: ALT2"]
            pub const GPIO_AD_13_ALT2: u32 = 0;
            #[doc = "Selecting Pad: GPIO_11 for Mode: ALT2"]
            pub const GPIO_11_ALT2: u32 = 0x01;
        }
    }
}
#[doc = "KPP_ROW_SELECT_INPUT_1 DAISY Register"]
pub mod KPP_ROW_SELECT_INPUT_1 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    pub mod DAISY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Selecting Pad: GPIO_AD_11 for Mode: ALT2"]
            pub const GPIO_AD_11_ALT2: u32 = 0;
            #[doc = "Selecting Pad: GPIO_AD_05 for Mode: ALT3"]
            pub const GPIO_AD_05_ALT3: u32 = 0x01;
        }
    }
}
#[doc = "KPP_ROW_SELECT_INPUT_2 DAISY Register"]
pub mod KPP_ROW_SELECT_INPUT_2 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    pub mod DAISY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Selecting Pad: GPIO_AD_09 for Mode: ALT2"]
            pub const GPIO_AD_09_ALT2: u32 = 0;
            #[doc = "Selecting Pad: GPIO_AD_03 for Mode: ALT3"]
            pub const GPIO_AD_03_ALT3: u32 = 0x01;
        }
    }
}
#[doc = "KPP_ROW_SELECT_INPUT_3 DAISY Register"]
pub mod KPP_ROW_SELECT_INPUT_3 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    pub mod DAISY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Selecting Pad: GPIO_13 for Mode: ALT2"]
            pub const GPIO_13_ALT2: u32 = 0;
            #[doc = "Selecting Pad: GPIO_01 for Mode: ALT4"]
            pub const GPIO_01_ALT4: u32 = 0x01;
        }
    }
}
#[doc = "LPI2C1_HREQ_SELECT_INPUT DAISY Register"]
pub mod LPI2C1_HREQ_SELECT_INPUT {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    pub mod DAISY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Selecting Pad: GPIO_AD_06 for Mode: ALT6"]
            pub const GPIO_AD_06_ALT6: u32 = 0;
            #[doc = "Selecting Pad: GPIO_10 for Mode: ALT1"]
            pub const GPIO_10_ALT1: u32 = 0x01;
        }
    }
}
#[doc = "LPI2C1_SCL_SELECT_INPUT DAISY Register"]
pub mod LPI2C1_SCL_SELECT_INPUT {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    pub mod DAISY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Selecting Pad: GPIO_AD_14 for Mode: ALT0"]
            pub const GPIO_AD_14_ALT0: u32 = 0;
            #[doc = "Selecting Pad: GPIO_SD_06 for Mode: ALT1"]
            pub const GPIO_SD_06_ALT1: u32 = 0x01;
            #[doc = "Selecting Pad: GPIO_12 for Mode: ALT1"]
            pub const GPIO_12_ALT1: u32 = 0x02;
            #[doc = "Selecting Pad: GPIO_02 for Mode: ALT3"]
            pub const GPIO_02_ALT3: u32 = 0x03;
        }
    }
}
#[doc = "LPI2C1_SDA_SELECT_INPUT DAISY Register"]
pub mod LPI2C1_SDA_SELECT_INPUT {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    pub mod DAISY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Selecting Pad: GPIO_AD_13 for Mode: ALT0"]
            pub const GPIO_AD_13_ALT0: u32 = 0;
            #[doc = "Selecting Pad: GPIO_SD_05 for Mode: ALT1"]
            pub const GPIO_SD_05_ALT1: u32 = 0x01;
            #[doc = "Selecting Pad: GPIO_11 for Mode: ALT1"]
            pub const GPIO_11_ALT1: u32 = 0x02;
            #[doc = "Selecting Pad: GPIO_01 for Mode: ALT3"]
            pub const GPIO_01_ALT3: u32 = 0x03;
        }
    }
}
#[doc = "LPI2C2_SCL_SELECT_INPUT DAISY Register"]
pub mod LPI2C2_SCL_SELECT_INPUT {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    pub mod DAISY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Selecting Pad: GPIO_AD_08 for Mode: ALT0"]
            pub const GPIO_AD_08_ALT0: u32 = 0;
            #[doc = "Selecting Pad: GPIO_AD_02 for Mode: ALT3"]
            pub const GPIO_AD_02_ALT3: u32 = 0x01;
            #[doc = "Selecting Pad: GPIO_SD_08 for Mode: ALT1"]
            pub const GPIO_SD_08_ALT1: u32 = 0x02;
            #[doc = "Selecting Pad: GPIO_10 for Mode: ALT3"]
            pub const GPIO_10_ALT3: u32 = 0x03;
        }
    }
}
#[doc = "LPI2C2_SDA_SELECT_INPUT DAISY Register"]
pub mod LPI2C2_SDA_SELECT_INPUT {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    pub mod DAISY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Selecting Pad: GPIO_AD_07 for Mode: ALT0"]
            pub const GPIO_AD_07_ALT0: u32 = 0;
            #[doc = "Selecting Pad: GPIO_AD_01 for Mode: ALT3"]
            pub const GPIO_AD_01_ALT3: u32 = 0x01;
            #[doc = "Selecting Pad: GPIO_SD_07 for Mode: ALT1"]
            pub const GPIO_SD_07_ALT1: u32 = 0x02;
            #[doc = "Selecting Pad: GPIO_09 for Mode: ALT3"]
            pub const GPIO_09_ALT3: u32 = 0x03;
        }
    }
}
#[doc = "LPSPI1_PCS_SELECT_INPUT_0 DAISY Register"]
pub mod LPSPI1_PCS_SELECT_INPUT_0 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    pub mod DAISY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Selecting Pad: GPIO_AD_05 for Mode: ALT0"]
            pub const GPIO_AD_05_ALT0: u32 = 0;
            #[doc = "Selecting Pad: GPIO_SD_07 for Mode: ALT2"]
            pub const GPIO_SD_07_ALT2: u32 = 0x01;
        }
    }
}
#[doc = "LPSPI1_SCK_SELECT_INPUT DAISY Register"]
pub mod LPSPI1_SCK_SELECT_INPUT {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    pub mod DAISY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Selecting Pad: GPIO_AD_06 for Mode: ALT0"]
            pub const GPIO_AD_06_ALT0: u32 = 0;
            #[doc = "Selecting Pad: GPIO_SD_08 for Mode: ALT2"]
            pub const GPIO_SD_08_ALT2: u32 = 0x01;
        }
    }
}
#[doc = "LPSPI1_SDI_SELECT_INPUT DAISY Register"]
pub mod LPSPI1_SDI_SELECT_INPUT {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    pub mod DAISY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Selecting Pad: GPIO_AD_03 for Mode: ALT0"]
            pub const GPIO_AD_03_ALT0: u32 = 0;
            #[doc = "Selecting Pad: GPIO_SD_05 for Mode: ALT2"]
            pub const GPIO_SD_05_ALT2: u32 = 0x01;
        }
    }
}
#[doc = "LPSPI1_SDO_SELECT_INPUT DAISY Register"]
pub mod LPSPI1_SDO_SELECT_INPUT {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    pub mod DAISY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Selecting Pad: GPIO_AD_04 for Mode: ALT0"]
            pub const GPIO_AD_04_ALT0: u32 = 0;
            #[doc = "Selecting Pad: GPIO_SD_06 for Mode: ALT2"]
            pub const GPIO_SD_06_ALT2: u32 = 0x01;
        }
    }
}
#[doc = "LPSPI2_PCS_SELECT_INPUT_0 DAISY Register"]
pub mod LPSPI2_PCS_SELECT_INPUT_0 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    pub mod DAISY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Selecting Pad: GPIO_AD_11 for Mode: ALT0"]
            pub const GPIO_AD_11_ALT0: u32 = 0;
            #[doc = "Selecting Pad: GPIO_SD_12 for Mode: ALT1"]
            pub const GPIO_SD_12_ALT1: u32 = 0x01;
        }
    }
}
#[doc = "LPSPI2_SCK_SELECT_INPUT DAISY Register"]
pub mod LPSPI2_SCK_SELECT_INPUT {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    pub mod DAISY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Selecting Pad: GPIO_AD_12 for Mode: ALT0"]
            pub const GPIO_AD_12_ALT0: u32 = 0;
            #[doc = "Selecting Pad: GPIO_SD_11 for Mode: ALT1"]
            pub const GPIO_SD_11_ALT1: u32 = 0x01;
        }
    }
}
#[doc = "LPSPI2_SDI_SELECT_INPUT DAISY Register"]
pub mod LPSPI2_SDI_SELECT_INPUT {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    pub mod DAISY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Selecting Pad: GPIO_AD_09 for Mode: ALT0"]
            pub const GPIO_AD_09_ALT0: u32 = 0;
            #[doc = "Selecting Pad: GPIO_SD_09 for Mode: ALT1"]
            pub const GPIO_SD_09_ALT1: u32 = 0x01;
        }
    }
}
#[doc = "LPSPI2_SDO_SELECT_INPUT DAISY Register"]
pub mod LPSPI2_SDO_SELECT_INPUT {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    pub mod DAISY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Selecting Pad: GPIO_AD_10 for Mode: ALT0"]
            pub const GPIO_AD_10_ALT0: u32 = 0;
            #[doc = "Selecting Pad: GPIO_SD_10 for Mode: ALT1"]
            pub const GPIO_SD_10_ALT1: u32 = 0x01;
        }
    }
}
#[doc = "LPUART1_RXD_SELECT_INPUT DAISY Register"]
pub mod LPUART1_RXD_SELECT_INPUT {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    pub mod DAISY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Selecting Pad: GPIO_SD_11 for Mode: ALT2"]
            pub const GPIO_SD_11_ALT2: u32 = 0;
            #[doc = "Selecting Pad: GPIO_09 for Mode: ALT0"]
            pub const GPIO_09_ALT0: u32 = 0x01;
        }
    }
}
#[doc = "LPUART1_TXD_SELECT_INPUT DAISY Register"]
pub mod LPUART1_TXD_SELECT_INPUT {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    pub mod DAISY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Selecting Pad: GPIO_SD_12 for Mode: ALT2"]
            pub const GPIO_SD_12_ALT2: u32 = 0;
            #[doc = "Selecting Pad: GPIO_10 for Mode: ALT0"]
            pub const GPIO_10_ALT0: u32 = 0x01;
        }
    }
}
#[doc = "LPUART2_RXD_SELECT_INPUT DAISY Register"]
pub mod LPUART2_RXD_SELECT_INPUT {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    pub mod DAISY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Selecting Pad: GPIO_SD_09 for Mode: ALT2"]
            pub const GPIO_SD_09_ALT2: u32 = 0;
            #[doc = "Selecting Pad: GPIO_13 for Mode: ALT0"]
            pub const GPIO_13_ALT0: u32 = 0x01;
        }
    }
}
#[doc = "LPUART2_TXD_SELECT_INPUT DAISY Register"]
pub mod LPUART2_TXD_SELECT_INPUT {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    pub mod DAISY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Selecting Pad: GPIO_AD_00 for Mode: ALT0"]
            pub const GPIO_AD_00_ALT0: u32 = 0;
            #[doc = "Selecting Pad: GPIO_SD_10 for Mode: ALT2"]
            pub const GPIO_SD_10_ALT2: u32 = 0x01;
        }
    }
}
#[doc = "LPUART3_RXD_SELECT_INPUT DAISY Register"]
pub mod LPUART3_RXD_SELECT_INPUT {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    pub mod DAISY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Selecting Pad: GPIO_AD_07 for Mode: ALT1"]
            pub const GPIO_AD_07_ALT1: u32 = 0;
            #[doc = "Selecting Pad: GPIO_11 for Mode: ALT0"]
            pub const GPIO_11_ALT0: u32 = 0x01;
            #[doc = "Selecting Pad: GPIO_07 for Mode: ALT3"]
            pub const GPIO_07_ALT3: u32 = 0x02;
        }
    }
}
#[doc = "LPUART3_TXD_SELECT_INPUT DAISY Register"]
pub mod LPUART3_TXD_SELECT_INPUT {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    pub mod DAISY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Selecting Pad: GPIO_AD_08 for Mode: ALT1"]
            pub const GPIO_AD_08_ALT1: u32 = 0;
            #[doc = "Selecting Pad: GPIO_12 for Mode: ALT0"]
            pub const GPIO_12_ALT0: u32 = 0x01;
            #[doc = "Selecting Pad: GPIO_08 for Mode: ALT3"]
            pub const GPIO_08_ALT3: u32 = 0x02;
        }
    }
}
#[doc = "LPUART4_RXD_SELECT_INPUT DAISY Register"]
pub mod LPUART4_RXD_SELECT_INPUT {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    pub mod DAISY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Selecting Pad: GPIO_AD_01 for Mode: ALT0"]
            pub const GPIO_AD_01_ALT0: u32 = 0;
            #[doc = "Selecting Pad: GPIO_05 for Mode: ALT3"]
            pub const GPIO_05_ALT3: u32 = 0x01;
        }
    }
}
#[doc = "LPUART4_TXD_SELECT_INPUT DAISY Register"]
pub mod LPUART4_TXD_SELECT_INPUT {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    pub mod DAISY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Selecting Pad: GPIO_AD_02 for Mode: ALT0"]
            pub const GPIO_AD_02_ALT0: u32 = 0;
            #[doc = "Selecting Pad: GPIO_06 for Mode: ALT3"]
            pub const GPIO_06_ALT3: u32 = 0x01;
        }
    }
}
#[doc = "NMI_GLUE_NMI_SELECT_INPUT DAISY Register"]
pub mod NMI_GLUE_NMI_SELECT_INPUT {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    pub mod DAISY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Selecting Pad: GPIO_AD_13 for Mode: ALT6"]
            pub const GPIO_AD_13_ALT6: u32 = 0;
            #[doc = "Selecting Pad: GPIO_AD_00 for Mode: ALT6"]
            pub const GPIO_AD_00_ALT6: u32 = 0x01;
        }
    }
}
#[doc = "SPDIF_IN1_SELECT_INPUT DAISY Register"]
pub mod SPDIF_IN1_SELECT_INPUT {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    pub mod DAISY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Selecting Pad: GPIO_10 for Mode: ALT6"]
            pub const GPIO_10_ALT6: u32 = 0;
            #[doc = "Selecting Pad: GPIO_04 for Mode: ALT4"]
            pub const GPIO_04_ALT4: u32 = 0x01;
        }
    }
}
#[doc = "SPDIF_TX_CLK2_SELECT_INPUT DAISY Register"]
pub mod SPDIF_TX_CLK2_SELECT_INPUT {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    pub mod DAISY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Selecting Pad: GPIO_12 for Mode: ALT6"]
            pub const GPIO_12_ALT6: u32 = 0;
            #[doc = "Selecting Pad: GPIO_06 for Mode: ALT4"]
            pub const GPIO_06_ALT4: u32 = 0x01;
        }
    }
}
#[doc = "USB_OTG_OC_SELECT_INPUT DAISY Register"]
pub mod USB_OTG_OC_SELECT_INPUT {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    pub mod DAISY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Selecting Pad: GPIO_AD_01 for Mode: ALT6"]
            pub const GPIO_AD_01_ALT6: u32 = 0;
            #[doc = "Selecting Pad: GPIO_12 for Mode: ALT3"]
            pub const GPIO_12_ALT3: u32 = 0x01;
        }
    }
}
#[doc = "XEV_GLUE_RXEV_SELECT_INPUT DAISY Register"]
pub mod XEV_GLUE_RXEV_SELECT_INPUT {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    pub mod DAISY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Selecting Pad: GPIO_AD_07 for Mode: ALT2"]
            pub const GPIO_AD_07_ALT2: u32 = 0;
            #[doc = "Selecting Pad: GPIO_SD_00 for Mode: ALT2"]
            pub const GPIO_SD_00_ALT2: u32 = 0x01;
        }
    }
}
