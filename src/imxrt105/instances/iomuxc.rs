#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! IOMUXC
//!
//! Used by: imxrt1051, imxrt1052

#[cfg(not(feature = "nosync"))]
pub use crate::imxrt105::peripherals::iomuxc::Instance;
pub use crate::imxrt105::peripherals::iomuxc::{RegisterBlock, ResetValues};
pub use crate::imxrt105::peripherals::iomuxc::{
    ANATOP_USB_OTG1_ID_SELECT_INPUT, ANATOP_USB_OTG2_ID_SELECT_INPUT, CCM_PMIC_READY_SELECT_INPUT,
    CSI_DATA02_SELECT_INPUT, CSI_DATA03_SELECT_INPUT, CSI_DATA04_SELECT_INPUT,
    CSI_DATA05_SELECT_INPUT, CSI_DATA06_SELECT_INPUT, CSI_DATA07_SELECT_INPUT,
    CSI_DATA08_SELECT_INPUT, CSI_DATA09_SELECT_INPUT, CSI_HSYNC_SELECT_INPUT,
    CSI_PIXCLK_SELECT_INPUT, CSI_VSYNC_SELECT_INPUT, ENET0_RXDATA_SELECT_INPUT,
    ENET0_TIMER_SELECT_INPUT, ENET1_RXDATA_SELECT_INPUT, ENET_IPG_CLK_RMII_SELECT_INPUT,
    ENET_MDIO_SELECT_INPUT, ENET_RXEN_SELECT_INPUT, ENET_RXERR_SELECT_INPUT,
    ENET_TXCLK_SELECT_INPUT, FLEXCAN1_RX_SELECT_INPUT, FLEXCAN2_RX_SELECT_INPUT,
    FLEXPWM1_PWMA0_SELECT_INPUT, FLEXPWM1_PWMA1_SELECT_INPUT, FLEXPWM1_PWMA2_SELECT_INPUT,
    FLEXPWM1_PWMA3_SELECT_INPUT, FLEXPWM1_PWMB0_SELECT_INPUT, FLEXPWM1_PWMB1_SELECT_INPUT,
    FLEXPWM1_PWMB2_SELECT_INPUT, FLEXPWM1_PWMB3_SELECT_INPUT, FLEXPWM2_PWMA0_SELECT_INPUT,
    FLEXPWM2_PWMA1_SELECT_INPUT, FLEXPWM2_PWMA2_SELECT_INPUT, FLEXPWM2_PWMA3_SELECT_INPUT,
    FLEXPWM2_PWMB0_SELECT_INPUT, FLEXPWM2_PWMB1_SELECT_INPUT, FLEXPWM2_PWMB2_SELECT_INPUT,
    FLEXPWM2_PWMB3_SELECT_INPUT, FLEXPWM4_PWMA0_SELECT_INPUT, FLEXPWM4_PWMA1_SELECT_INPUT,
    FLEXPWM4_PWMA2_SELECT_INPUT, FLEXPWM4_PWMA3_SELECT_INPUT, FLEXSPIA_DATA0_SELECT_INPUT,
    FLEXSPIA_DATA1_SELECT_INPUT, FLEXSPIA_DATA2_SELECT_INPUT, FLEXSPIA_DATA3_SELECT_INPUT,
    FLEXSPIA_DQS_SELECT_INPUT, FLEXSPIA_SCK_SELECT_INPUT, FLEXSPIB_DATA0_SELECT_INPUT,
    FLEXSPIB_DATA1_SELECT_INPUT, FLEXSPIB_DATA2_SELECT_INPUT, FLEXSPIB_DATA3_SELECT_INPUT,
    LPI2C1_SCL_SELECT_INPUT, LPI2C1_SDA_SELECT_INPUT, LPI2C2_SCL_SELECT_INPUT,
    LPI2C2_SDA_SELECT_INPUT, LPI2C3_SCL_SELECT_INPUT, LPI2C3_SDA_SELECT_INPUT,
    LPI2C4_SCL_SELECT_INPUT, LPI2C4_SDA_SELECT_INPUT, LPSPI1_PCS0_SELECT_INPUT,
    LPSPI1_SCK_SELECT_INPUT, LPSPI1_SDI_SELECT_INPUT, LPSPI1_SDO_SELECT_INPUT,
    LPSPI2_PCS0_SELECT_INPUT, LPSPI2_SCK_SELECT_INPUT, LPSPI2_SDI_SELECT_INPUT,
    LPSPI2_SDO_SELECT_INPUT, LPSPI3_PCS0_SELECT_INPUT, LPSPI3_SCK_SELECT_INPUT,
    LPSPI3_SDI_SELECT_INPUT, LPSPI3_SDO_SELECT_INPUT, LPSPI4_PCS0_SELECT_INPUT,
    LPSPI4_SCK_SELECT_INPUT, LPSPI4_SDI_SELECT_INPUT, LPSPI4_SDO_SELECT_INPUT,
    LPUART2_RX_SELECT_INPUT, LPUART2_TX_SELECT_INPUT, LPUART3_CTS_B_SELECT_INPUT,
    LPUART3_RX_SELECT_INPUT, LPUART3_TX_SELECT_INPUT, LPUART4_RX_SELECT_INPUT,
    LPUART4_TX_SELECT_INPUT, LPUART5_RX_SELECT_INPUT, LPUART5_TX_SELECT_INPUT,
    LPUART6_RX_SELECT_INPUT, LPUART6_TX_SELECT_INPUT, LPUART7_RX_SELECT_INPUT,
    LPUART7_TX_SELECT_INPUT, LPUART8_RX_SELECT_INPUT, LPUART8_TX_SELECT_INPUT, NMI_SELECT_INPUT,
    QTIMER2_TIMER0_SELECT_INPUT, QTIMER2_TIMER1_SELECT_INPUT, QTIMER2_TIMER2_SELECT_INPUT,
    QTIMER2_TIMER3_SELECT_INPUT, QTIMER3_TIMER0_SELECT_INPUT, QTIMER3_TIMER1_SELECT_INPUT,
    QTIMER3_TIMER2_SELECT_INPUT, QTIMER3_TIMER3_SELECT_INPUT, SAI1_MCLK2_SELECT_INPUT,
    SAI1_RX_BCLK_SELECT_INPUT, SAI1_RX_DATA0_SELECT_INPUT, SAI1_RX_DATA1_SELECT_INPUT,
    SAI1_RX_DATA2_SELECT_INPUT, SAI1_RX_DATA3_SELECT_INPUT, SAI1_RX_SYNC_SELECT_INPUT,
    SAI1_TX_BCLK_SELECT_INPUT, SAI1_TX_SYNC_SELECT_INPUT, SAI2_MCLK2_SELECT_INPUT,
    SAI2_RX_BCLK_SELECT_INPUT, SAI2_RX_DATA0_SELECT_INPUT, SAI2_RX_SYNC_SELECT_INPUT,
    SAI2_TX_BCLK_SELECT_INPUT, SAI2_TX_SYNC_SELECT_INPUT, SPDIF_IN_SELECT_INPUT,
    SW_MUX_CTL_PAD_GPIO_AD_B0_00, SW_MUX_CTL_PAD_GPIO_AD_B0_01, SW_MUX_CTL_PAD_GPIO_AD_B0_02,
    SW_MUX_CTL_PAD_GPIO_AD_B0_03, SW_MUX_CTL_PAD_GPIO_AD_B0_04, SW_MUX_CTL_PAD_GPIO_AD_B0_05,
    SW_MUX_CTL_PAD_GPIO_AD_B0_06, SW_MUX_CTL_PAD_GPIO_AD_B0_07, SW_MUX_CTL_PAD_GPIO_AD_B0_08,
    SW_MUX_CTL_PAD_GPIO_AD_B0_09, SW_MUX_CTL_PAD_GPIO_AD_B0_10, SW_MUX_CTL_PAD_GPIO_AD_B0_11,
    SW_MUX_CTL_PAD_GPIO_AD_B0_12, SW_MUX_CTL_PAD_GPIO_AD_B0_13, SW_MUX_CTL_PAD_GPIO_AD_B0_14,
    SW_MUX_CTL_PAD_GPIO_AD_B0_15, SW_MUX_CTL_PAD_GPIO_AD_B1_00, SW_MUX_CTL_PAD_GPIO_AD_B1_01,
    SW_MUX_CTL_PAD_GPIO_AD_B1_02, SW_MUX_CTL_PAD_GPIO_AD_B1_03, SW_MUX_CTL_PAD_GPIO_AD_B1_04,
    SW_MUX_CTL_PAD_GPIO_AD_B1_05, SW_MUX_CTL_PAD_GPIO_AD_B1_06, SW_MUX_CTL_PAD_GPIO_AD_B1_07,
    SW_MUX_CTL_PAD_GPIO_AD_B1_08, SW_MUX_CTL_PAD_GPIO_AD_B1_09, SW_MUX_CTL_PAD_GPIO_AD_B1_10,
    SW_MUX_CTL_PAD_GPIO_AD_B1_11, SW_MUX_CTL_PAD_GPIO_AD_B1_12, SW_MUX_CTL_PAD_GPIO_AD_B1_13,
    SW_MUX_CTL_PAD_GPIO_AD_B1_14, SW_MUX_CTL_PAD_GPIO_AD_B1_15, SW_MUX_CTL_PAD_GPIO_B0_00,
    SW_MUX_CTL_PAD_GPIO_B0_01, SW_MUX_CTL_PAD_GPIO_B0_02, SW_MUX_CTL_PAD_GPIO_B0_03,
    SW_MUX_CTL_PAD_GPIO_B0_04, SW_MUX_CTL_PAD_GPIO_B0_05, SW_MUX_CTL_PAD_GPIO_B0_06,
    SW_MUX_CTL_PAD_GPIO_B0_07, SW_MUX_CTL_PAD_GPIO_B0_08, SW_MUX_CTL_PAD_GPIO_B0_09,
    SW_MUX_CTL_PAD_GPIO_B0_10, SW_MUX_CTL_PAD_GPIO_B0_11, SW_MUX_CTL_PAD_GPIO_B0_12,
    SW_MUX_CTL_PAD_GPIO_B0_13, SW_MUX_CTL_PAD_GPIO_B0_14, SW_MUX_CTL_PAD_GPIO_B0_15,
    SW_MUX_CTL_PAD_GPIO_B1_00, SW_MUX_CTL_PAD_GPIO_B1_01, SW_MUX_CTL_PAD_GPIO_B1_02,
    SW_MUX_CTL_PAD_GPIO_B1_03, SW_MUX_CTL_PAD_GPIO_B1_04, SW_MUX_CTL_PAD_GPIO_B1_05,
    SW_MUX_CTL_PAD_GPIO_B1_06, SW_MUX_CTL_PAD_GPIO_B1_07, SW_MUX_CTL_PAD_GPIO_B1_08,
    SW_MUX_CTL_PAD_GPIO_B1_09, SW_MUX_CTL_PAD_GPIO_B1_10, SW_MUX_CTL_PAD_GPIO_B1_11,
    SW_MUX_CTL_PAD_GPIO_B1_12, SW_MUX_CTL_PAD_GPIO_B1_13, SW_MUX_CTL_PAD_GPIO_B1_14,
    SW_MUX_CTL_PAD_GPIO_B1_15, SW_MUX_CTL_PAD_GPIO_EMC_00, SW_MUX_CTL_PAD_GPIO_EMC_01,
    SW_MUX_CTL_PAD_GPIO_EMC_02, SW_MUX_CTL_PAD_GPIO_EMC_03, SW_MUX_CTL_PAD_GPIO_EMC_04,
    SW_MUX_CTL_PAD_GPIO_EMC_05, SW_MUX_CTL_PAD_GPIO_EMC_06, SW_MUX_CTL_PAD_GPIO_EMC_07,
    SW_MUX_CTL_PAD_GPIO_EMC_08, SW_MUX_CTL_PAD_GPIO_EMC_09, SW_MUX_CTL_PAD_GPIO_EMC_10,
    SW_MUX_CTL_PAD_GPIO_EMC_11, SW_MUX_CTL_PAD_GPIO_EMC_12, SW_MUX_CTL_PAD_GPIO_EMC_13,
    SW_MUX_CTL_PAD_GPIO_EMC_14, SW_MUX_CTL_PAD_GPIO_EMC_15, SW_MUX_CTL_PAD_GPIO_EMC_16,
    SW_MUX_CTL_PAD_GPIO_EMC_17, SW_MUX_CTL_PAD_GPIO_EMC_18, SW_MUX_CTL_PAD_GPIO_EMC_19,
    SW_MUX_CTL_PAD_GPIO_EMC_20, SW_MUX_CTL_PAD_GPIO_EMC_21, SW_MUX_CTL_PAD_GPIO_EMC_22,
    SW_MUX_CTL_PAD_GPIO_EMC_23, SW_MUX_CTL_PAD_GPIO_EMC_24, SW_MUX_CTL_PAD_GPIO_EMC_25,
    SW_MUX_CTL_PAD_GPIO_EMC_26, SW_MUX_CTL_PAD_GPIO_EMC_27, SW_MUX_CTL_PAD_GPIO_EMC_28,
    SW_MUX_CTL_PAD_GPIO_EMC_29, SW_MUX_CTL_PAD_GPIO_EMC_30, SW_MUX_CTL_PAD_GPIO_EMC_31,
    SW_MUX_CTL_PAD_GPIO_EMC_32, SW_MUX_CTL_PAD_GPIO_EMC_33, SW_MUX_CTL_PAD_GPIO_EMC_34,
    SW_MUX_CTL_PAD_GPIO_EMC_35, SW_MUX_CTL_PAD_GPIO_EMC_36, SW_MUX_CTL_PAD_GPIO_EMC_37,
    SW_MUX_CTL_PAD_GPIO_EMC_38, SW_MUX_CTL_PAD_GPIO_EMC_39, SW_MUX_CTL_PAD_GPIO_EMC_40,
    SW_MUX_CTL_PAD_GPIO_EMC_41, SW_MUX_CTL_PAD_GPIO_SD_B0_00, SW_MUX_CTL_PAD_GPIO_SD_B0_01,
    SW_MUX_CTL_PAD_GPIO_SD_B0_02, SW_MUX_CTL_PAD_GPIO_SD_B0_03, SW_MUX_CTL_PAD_GPIO_SD_B0_04,
    SW_MUX_CTL_PAD_GPIO_SD_B0_05, SW_MUX_CTL_PAD_GPIO_SD_B1_00, SW_MUX_CTL_PAD_GPIO_SD_B1_01,
    SW_MUX_CTL_PAD_GPIO_SD_B1_02, SW_MUX_CTL_PAD_GPIO_SD_B1_03, SW_MUX_CTL_PAD_GPIO_SD_B1_04,
    SW_MUX_CTL_PAD_GPIO_SD_B1_05, SW_MUX_CTL_PAD_GPIO_SD_B1_06, SW_MUX_CTL_PAD_GPIO_SD_B1_07,
    SW_MUX_CTL_PAD_GPIO_SD_B1_08, SW_MUX_CTL_PAD_GPIO_SD_B1_09, SW_MUX_CTL_PAD_GPIO_SD_B1_10,
    SW_MUX_CTL_PAD_GPIO_SD_B1_11, SW_PAD_CTL_PAD_GPIO_AD_B0_00, SW_PAD_CTL_PAD_GPIO_AD_B0_01,
    SW_PAD_CTL_PAD_GPIO_AD_B0_02, SW_PAD_CTL_PAD_GPIO_AD_B0_03, SW_PAD_CTL_PAD_GPIO_AD_B0_04,
    SW_PAD_CTL_PAD_GPIO_AD_B0_05, SW_PAD_CTL_PAD_GPIO_AD_B0_06, SW_PAD_CTL_PAD_GPIO_AD_B0_07,
    SW_PAD_CTL_PAD_GPIO_AD_B0_08, SW_PAD_CTL_PAD_GPIO_AD_B0_09, SW_PAD_CTL_PAD_GPIO_AD_B0_10,
    SW_PAD_CTL_PAD_GPIO_AD_B0_11, SW_PAD_CTL_PAD_GPIO_AD_B0_12, SW_PAD_CTL_PAD_GPIO_AD_B0_13,
    SW_PAD_CTL_PAD_GPIO_AD_B0_14, SW_PAD_CTL_PAD_GPIO_AD_B0_15, SW_PAD_CTL_PAD_GPIO_AD_B1_00,
    SW_PAD_CTL_PAD_GPIO_AD_B1_01, SW_PAD_CTL_PAD_GPIO_AD_B1_02, SW_PAD_CTL_PAD_GPIO_AD_B1_03,
    SW_PAD_CTL_PAD_GPIO_AD_B1_04, SW_PAD_CTL_PAD_GPIO_AD_B1_05, SW_PAD_CTL_PAD_GPIO_AD_B1_06,
    SW_PAD_CTL_PAD_GPIO_AD_B1_07, SW_PAD_CTL_PAD_GPIO_AD_B1_08, SW_PAD_CTL_PAD_GPIO_AD_B1_09,
    SW_PAD_CTL_PAD_GPIO_AD_B1_10, SW_PAD_CTL_PAD_GPIO_AD_B1_11, SW_PAD_CTL_PAD_GPIO_AD_B1_12,
    SW_PAD_CTL_PAD_GPIO_AD_B1_13, SW_PAD_CTL_PAD_GPIO_AD_B1_14, SW_PAD_CTL_PAD_GPIO_AD_B1_15,
    SW_PAD_CTL_PAD_GPIO_B0_00, SW_PAD_CTL_PAD_GPIO_B0_01, SW_PAD_CTL_PAD_GPIO_B0_02,
    SW_PAD_CTL_PAD_GPIO_B0_03, SW_PAD_CTL_PAD_GPIO_B0_04, SW_PAD_CTL_PAD_GPIO_B0_05,
    SW_PAD_CTL_PAD_GPIO_B0_06, SW_PAD_CTL_PAD_GPIO_B0_07, SW_PAD_CTL_PAD_GPIO_B0_08,
    SW_PAD_CTL_PAD_GPIO_B0_09, SW_PAD_CTL_PAD_GPIO_B0_10, SW_PAD_CTL_PAD_GPIO_B0_11,
    SW_PAD_CTL_PAD_GPIO_B0_12, SW_PAD_CTL_PAD_GPIO_B0_13, SW_PAD_CTL_PAD_GPIO_B0_14,
    SW_PAD_CTL_PAD_GPIO_B0_15, SW_PAD_CTL_PAD_GPIO_B1_00, SW_PAD_CTL_PAD_GPIO_B1_01,
    SW_PAD_CTL_PAD_GPIO_B1_02, SW_PAD_CTL_PAD_GPIO_B1_03, SW_PAD_CTL_PAD_GPIO_B1_04,
    SW_PAD_CTL_PAD_GPIO_B1_05, SW_PAD_CTL_PAD_GPIO_B1_06, SW_PAD_CTL_PAD_GPIO_B1_07,
    SW_PAD_CTL_PAD_GPIO_B1_08, SW_PAD_CTL_PAD_GPIO_B1_09, SW_PAD_CTL_PAD_GPIO_B1_10,
    SW_PAD_CTL_PAD_GPIO_B1_11, SW_PAD_CTL_PAD_GPIO_B1_12, SW_PAD_CTL_PAD_GPIO_B1_13,
    SW_PAD_CTL_PAD_GPIO_B1_14, SW_PAD_CTL_PAD_GPIO_B1_15, SW_PAD_CTL_PAD_GPIO_EMC_00,
    SW_PAD_CTL_PAD_GPIO_EMC_01, SW_PAD_CTL_PAD_GPIO_EMC_02, SW_PAD_CTL_PAD_GPIO_EMC_03,
    SW_PAD_CTL_PAD_GPIO_EMC_04, SW_PAD_CTL_PAD_GPIO_EMC_05, SW_PAD_CTL_PAD_GPIO_EMC_06,
    SW_PAD_CTL_PAD_GPIO_EMC_07, SW_PAD_CTL_PAD_GPIO_EMC_08, SW_PAD_CTL_PAD_GPIO_EMC_09,
    SW_PAD_CTL_PAD_GPIO_EMC_10, SW_PAD_CTL_PAD_GPIO_EMC_11, SW_PAD_CTL_PAD_GPIO_EMC_12,
    SW_PAD_CTL_PAD_GPIO_EMC_13, SW_PAD_CTL_PAD_GPIO_EMC_14, SW_PAD_CTL_PAD_GPIO_EMC_15,
    SW_PAD_CTL_PAD_GPIO_EMC_16, SW_PAD_CTL_PAD_GPIO_EMC_17, SW_PAD_CTL_PAD_GPIO_EMC_18,
    SW_PAD_CTL_PAD_GPIO_EMC_19, SW_PAD_CTL_PAD_GPIO_EMC_20, SW_PAD_CTL_PAD_GPIO_EMC_21,
    SW_PAD_CTL_PAD_GPIO_EMC_22, SW_PAD_CTL_PAD_GPIO_EMC_23, SW_PAD_CTL_PAD_GPIO_EMC_24,
    SW_PAD_CTL_PAD_GPIO_EMC_25, SW_PAD_CTL_PAD_GPIO_EMC_26, SW_PAD_CTL_PAD_GPIO_EMC_27,
    SW_PAD_CTL_PAD_GPIO_EMC_28, SW_PAD_CTL_PAD_GPIO_EMC_29, SW_PAD_CTL_PAD_GPIO_EMC_30,
    SW_PAD_CTL_PAD_GPIO_EMC_31, SW_PAD_CTL_PAD_GPIO_EMC_32, SW_PAD_CTL_PAD_GPIO_EMC_33,
    SW_PAD_CTL_PAD_GPIO_EMC_34, SW_PAD_CTL_PAD_GPIO_EMC_35, SW_PAD_CTL_PAD_GPIO_EMC_36,
    SW_PAD_CTL_PAD_GPIO_EMC_37, SW_PAD_CTL_PAD_GPIO_EMC_38, SW_PAD_CTL_PAD_GPIO_EMC_39,
    SW_PAD_CTL_PAD_GPIO_EMC_40, SW_PAD_CTL_PAD_GPIO_EMC_41, SW_PAD_CTL_PAD_GPIO_SD_B0_00,
    SW_PAD_CTL_PAD_GPIO_SD_B0_01, SW_PAD_CTL_PAD_GPIO_SD_B0_02, SW_PAD_CTL_PAD_GPIO_SD_B0_03,
    SW_PAD_CTL_PAD_GPIO_SD_B0_04, SW_PAD_CTL_PAD_GPIO_SD_B0_05, SW_PAD_CTL_PAD_GPIO_SD_B1_00,
    SW_PAD_CTL_PAD_GPIO_SD_B1_01, SW_PAD_CTL_PAD_GPIO_SD_B1_02, SW_PAD_CTL_PAD_GPIO_SD_B1_03,
    SW_PAD_CTL_PAD_GPIO_SD_B1_04, SW_PAD_CTL_PAD_GPIO_SD_B1_05, SW_PAD_CTL_PAD_GPIO_SD_B1_06,
    SW_PAD_CTL_PAD_GPIO_SD_B1_07, SW_PAD_CTL_PAD_GPIO_SD_B1_08, SW_PAD_CTL_PAD_GPIO_SD_B1_09,
    SW_PAD_CTL_PAD_GPIO_SD_B1_10, SW_PAD_CTL_PAD_GPIO_SD_B1_11, USB_OTG1_OC_SELECT_INPUT,
    USB_OTG2_OC_SELECT_INPUT, USDHC1_CD_B_SELECT_INPUT, USDHC1_WP_SELECT_INPUT,
    USDHC2_CD_B_SELECT_INPUT, USDHC2_CLK_SELECT_INPUT, USDHC2_CMD_SELECT_INPUT,
    USDHC2_DATA0_SELECT_INPUT, USDHC2_DATA1_SELECT_INPUT, USDHC2_DATA2_SELECT_INPUT,
    USDHC2_DATA3_SELECT_INPUT, USDHC2_DATA4_SELECT_INPUT, USDHC2_DATA5_SELECT_INPUT,
    USDHC2_DATA6_SELECT_INPUT, USDHC2_DATA7_SELECT_INPUT, USDHC2_WP_SELECT_INPUT,
    XBAR1_IN02_SELECT_INPUT, XBAR1_IN03_SELECT_INPUT, XBAR1_IN04_SELECT_INPUT,
    XBAR1_IN05_SELECT_INPUT, XBAR1_IN06_SELECT_INPUT, XBAR1_IN07_SELECT_INPUT,
    XBAR1_IN08_SELECT_INPUT, XBAR1_IN09_SELECT_INPUT, XBAR1_IN14_SELECT_INPUT,
    XBAR1_IN15_SELECT_INPUT, XBAR1_IN16_SELECT_INPUT, XBAR1_IN17_SELECT_INPUT,
    XBAR1_IN18_SELECT_INPUT, XBAR1_IN19_SELECT_INPUT, XBAR1_IN20_SELECT_INPUT,
    XBAR1_IN21_SELECT_INPUT, XBAR1_IN22_SELECT_INPUT, XBAR1_IN23_SELECT_INPUT,
    XBAR1_IN24_SELECT_INPUT, XBAR1_IN25_SELECT_INPUT,
};

/// Access functions for the IOMUXC peripheral instance
pub mod IOMUXC {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x401f8000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in IOMUXC
    pub const reset: ResetValues = ResetValues {
        SW_MUX_CTL_PAD_GPIO_EMC_00: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_EMC_01: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_EMC_02: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_EMC_03: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_EMC_04: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_EMC_05: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_EMC_06: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_EMC_07: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_EMC_08: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_EMC_09: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_EMC_10: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_EMC_11: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_EMC_12: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_EMC_13: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_EMC_14: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_EMC_15: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_EMC_16: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_EMC_17: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_EMC_18: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_EMC_19: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_EMC_20: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_EMC_21: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_EMC_22: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_EMC_23: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_EMC_24: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_EMC_25: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_EMC_26: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_EMC_27: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_EMC_28: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_EMC_29: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_EMC_30: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_EMC_31: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_EMC_32: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_EMC_33: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_EMC_34: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_EMC_35: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_EMC_36: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_EMC_37: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_EMC_38: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_EMC_39: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_EMC_40: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_EMC_41: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_AD_B0_00: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_AD_B0_01: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_AD_B0_02: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_AD_B0_03: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_AD_B0_04: 0x00000000,
        SW_MUX_CTL_PAD_GPIO_AD_B0_05: 0x00000000,
        SW_MUX_CTL_PAD_GPIO_AD_B0_06: 0x00000000,
        SW_MUX_CTL_PAD_GPIO_AD_B0_07: 0x00000000,
        SW_MUX_CTL_PAD_GPIO_AD_B0_08: 0x00000000,
        SW_MUX_CTL_PAD_GPIO_AD_B0_09: 0x00000000,
        SW_MUX_CTL_PAD_GPIO_AD_B0_10: 0x00000000,
        SW_MUX_CTL_PAD_GPIO_AD_B0_11: 0x00000000,
        SW_MUX_CTL_PAD_GPIO_AD_B0_12: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_AD_B0_13: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_AD_B0_14: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_AD_B0_15: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_AD_B1_00: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_AD_B1_01: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_AD_B1_02: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_AD_B1_03: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_AD_B1_04: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_AD_B1_05: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_AD_B1_06: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_AD_B1_07: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_AD_B1_08: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_AD_B1_09: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_AD_B1_10: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_AD_B1_11: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_AD_B1_12: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_AD_B1_13: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_AD_B1_14: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_AD_B1_15: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_B0_00: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_B0_01: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_B0_02: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_B0_03: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_B0_04: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_B0_05: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_B0_06: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_B0_07: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_B0_08: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_B0_09: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_B0_10: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_B0_11: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_B0_12: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_B0_13: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_B0_14: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_B0_15: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_B1_00: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_B1_01: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_B1_02: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_B1_03: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_B1_04: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_B1_05: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_B1_06: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_B1_07: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_B1_08: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_B1_09: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_B1_10: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_B1_11: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_B1_12: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_B1_13: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_B1_14: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_B1_15: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_SD_B0_00: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_SD_B0_01: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_SD_B0_02: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_SD_B0_03: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_SD_B0_04: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_SD_B0_05: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_SD_B1_00: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_SD_B1_01: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_SD_B1_02: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_SD_B1_03: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_SD_B1_04: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_SD_B1_05: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_SD_B1_06: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_SD_B1_07: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_SD_B1_08: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_SD_B1_09: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_SD_B1_10: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_SD_B1_11: 0x00000005,
        SW_PAD_CTL_PAD_GPIO_EMC_00: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_EMC_01: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_EMC_02: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_EMC_03: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_EMC_04: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_EMC_05: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_EMC_06: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_EMC_07: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_EMC_08: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_EMC_09: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_EMC_10: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_EMC_11: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_EMC_12: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_EMC_13: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_EMC_14: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_EMC_15: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_EMC_16: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_EMC_17: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_EMC_18: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_EMC_19: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_EMC_20: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_EMC_21: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_EMC_22: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_EMC_23: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_EMC_24: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_EMC_25: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_EMC_26: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_EMC_27: 0x000030B0,
        SW_PAD_CTL_PAD_GPIO_EMC_28: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_EMC_29: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_EMC_30: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_EMC_31: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_EMC_32: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_EMC_33: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_EMC_34: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_EMC_35: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_EMC_36: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_EMC_37: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_EMC_38: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_EMC_39: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_EMC_40: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_EMC_41: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_AD_B0_00: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_AD_B0_01: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_AD_B0_02: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_AD_B0_03: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_AD_B0_04: 0x000030B0,
        SW_PAD_CTL_PAD_GPIO_AD_B0_05: 0x000030B0,
        SW_PAD_CTL_PAD_GPIO_AD_B0_06: 0x000070A0,
        SW_PAD_CTL_PAD_GPIO_AD_B0_07: 0x000070A0,
        SW_PAD_CTL_PAD_GPIO_AD_B0_08: 0x0000B0A0,
        SW_PAD_CTL_PAD_GPIO_AD_B0_09: 0x000070A0,
        SW_PAD_CTL_PAD_GPIO_AD_B0_10: 0x000090B1,
        SW_PAD_CTL_PAD_GPIO_AD_B0_11: 0x000070A0,
        SW_PAD_CTL_PAD_GPIO_AD_B0_12: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_AD_B0_13: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_AD_B0_14: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_AD_B0_15: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_AD_B1_00: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_AD_B1_01: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_AD_B1_02: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_AD_B1_03: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_AD_B1_04: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_AD_B1_05: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_AD_B1_06: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_AD_B1_07: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_AD_B1_08: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_AD_B1_09: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_AD_B1_10: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_AD_B1_11: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_AD_B1_12: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_AD_B1_13: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_AD_B1_14: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_AD_B1_15: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_B0_00: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_B0_01: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_B0_02: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_B0_03: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_B0_04: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_B0_05: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_B0_06: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_B0_07: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_B0_08: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_B0_09: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_B0_10: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_B0_11: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_B0_12: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_B0_13: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_B0_14: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_B0_15: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_B1_00: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_B1_01: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_B1_02: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_B1_03: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_B1_04: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_B1_05: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_B1_06: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_B1_07: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_B1_08: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_B1_09: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_B1_10: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_B1_11: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_B1_12: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_B1_13: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_B1_14: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_B1_15: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_SD_B0_00: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_SD_B0_01: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_SD_B0_02: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_SD_B0_03: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_SD_B0_04: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_SD_B0_05: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_SD_B1_00: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_SD_B1_01: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_SD_B1_02: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_SD_B1_03: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_SD_B1_04: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_SD_B1_05: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_SD_B1_06: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_SD_B1_07: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_SD_B1_08: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_SD_B1_09: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_SD_B1_10: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_SD_B1_11: 0x000010B0,
        ANATOP_USB_OTG1_ID_SELECT_INPUT: 0x00000000,
        ANATOP_USB_OTG2_ID_SELECT_INPUT: 0x00000000,
        CCM_PMIC_READY_SELECT_INPUT: 0x00000000,
        CSI_DATA02_SELECT_INPUT: 0x00000000,
        CSI_DATA03_SELECT_INPUT: 0x00000000,
        CSI_DATA04_SELECT_INPUT: 0x00000000,
        CSI_DATA05_SELECT_INPUT: 0x00000000,
        CSI_DATA06_SELECT_INPUT: 0x00000000,
        CSI_DATA07_SELECT_INPUT: 0x00000000,
        CSI_DATA08_SELECT_INPUT: 0x00000000,
        CSI_DATA09_SELECT_INPUT: 0x00000000,
        CSI_HSYNC_SELECT_INPUT: 0x00000000,
        CSI_PIXCLK_SELECT_INPUT: 0x00000000,
        CSI_VSYNC_SELECT_INPUT: 0x00000000,
        ENET_IPG_CLK_RMII_SELECT_INPUT: 0x00000000,
        ENET_MDIO_SELECT_INPUT: 0x00000000,
        ENET0_RXDATA_SELECT_INPUT: 0x00000000,
        ENET1_RXDATA_SELECT_INPUT: 0x00000000,
        ENET_RXEN_SELECT_INPUT: 0x00000000,
        ENET_RXERR_SELECT_INPUT: 0x00000000,
        ENET0_TIMER_SELECT_INPUT: 0x00000000,
        ENET_TXCLK_SELECT_INPUT: 0x00000000,
        FLEXCAN1_RX_SELECT_INPUT: 0x00000000,
        FLEXCAN2_RX_SELECT_INPUT: 0x00000000,
        FLEXPWM1_PWMA3_SELECT_INPUT: 0x00000000,
        FLEXPWM1_PWMA0_SELECT_INPUT: 0x00000000,
        FLEXPWM1_PWMA1_SELECT_INPUT: 0x00000000,
        FLEXPWM1_PWMA2_SELECT_INPUT: 0x00000000,
        FLEXPWM1_PWMB3_SELECT_INPUT: 0x00000000,
        FLEXPWM1_PWMB0_SELECT_INPUT: 0x00000000,
        FLEXPWM1_PWMB1_SELECT_INPUT: 0x00000000,
        FLEXPWM1_PWMB2_SELECT_INPUT: 0x00000000,
        FLEXPWM2_PWMA3_SELECT_INPUT: 0x00000000,
        FLEXPWM2_PWMA0_SELECT_INPUT: 0x00000000,
        FLEXPWM2_PWMA1_SELECT_INPUT: 0x00000000,
        FLEXPWM2_PWMA2_SELECT_INPUT: 0x00000000,
        FLEXPWM2_PWMB3_SELECT_INPUT: 0x00000000,
        FLEXPWM2_PWMB0_SELECT_INPUT: 0x00000000,
        FLEXPWM2_PWMB1_SELECT_INPUT: 0x00000000,
        FLEXPWM2_PWMB2_SELECT_INPUT: 0x00000000,
        FLEXPWM4_PWMA0_SELECT_INPUT: 0x00000000,
        FLEXPWM4_PWMA1_SELECT_INPUT: 0x00000000,
        FLEXPWM4_PWMA2_SELECT_INPUT: 0x00000000,
        FLEXPWM4_PWMA3_SELECT_INPUT: 0x00000000,
        FLEXSPIA_DQS_SELECT_INPUT: 0x00000000,
        FLEXSPIA_DATA0_SELECT_INPUT: 0x00000000,
        FLEXSPIA_DATA1_SELECT_INPUT: 0x00000000,
        FLEXSPIA_DATA2_SELECT_INPUT: 0x00000000,
        FLEXSPIA_DATA3_SELECT_INPUT: 0x00000000,
        FLEXSPIB_DATA0_SELECT_INPUT: 0x00000000,
        FLEXSPIB_DATA1_SELECT_INPUT: 0x00000000,
        FLEXSPIB_DATA2_SELECT_INPUT: 0x00000000,
        FLEXSPIB_DATA3_SELECT_INPUT: 0x00000000,
        FLEXSPIA_SCK_SELECT_INPUT: 0x00000000,
        LPI2C1_SCL_SELECT_INPUT: 0x00000000,
        LPI2C1_SDA_SELECT_INPUT: 0x00000000,
        LPI2C2_SCL_SELECT_INPUT: 0x00000000,
        LPI2C2_SDA_SELECT_INPUT: 0x00000000,
        LPI2C3_SCL_SELECT_INPUT: 0x00000000,
        LPI2C3_SDA_SELECT_INPUT: 0x00000000,
        LPI2C4_SCL_SELECT_INPUT: 0x00000000,
        LPI2C4_SDA_SELECT_INPUT: 0x00000000,
        LPSPI1_PCS0_SELECT_INPUT: 0x00000000,
        LPSPI1_SCK_SELECT_INPUT: 0x00000000,
        LPSPI1_SDI_SELECT_INPUT: 0x00000000,
        LPSPI1_SDO_SELECT_INPUT: 0x00000000,
        LPSPI2_PCS0_SELECT_INPUT: 0x00000000,
        LPSPI2_SCK_SELECT_INPUT: 0x00000000,
        LPSPI2_SDI_SELECT_INPUT: 0x00000000,
        LPSPI2_SDO_SELECT_INPUT: 0x00000000,
        LPSPI3_PCS0_SELECT_INPUT: 0x00000000,
        LPSPI3_SCK_SELECT_INPUT: 0x00000000,
        LPSPI3_SDI_SELECT_INPUT: 0x00000000,
        LPSPI3_SDO_SELECT_INPUT: 0x00000000,
        LPSPI4_PCS0_SELECT_INPUT: 0x00000000,
        LPSPI4_SCK_SELECT_INPUT: 0x00000000,
        LPSPI4_SDI_SELECT_INPUT: 0x00000000,
        LPSPI4_SDO_SELECT_INPUT: 0x00000000,
        LPUART2_RX_SELECT_INPUT: 0x00000000,
        LPUART2_TX_SELECT_INPUT: 0x00000000,
        LPUART3_CTS_B_SELECT_INPUT: 0x00000000,
        LPUART3_RX_SELECT_INPUT: 0x00000000,
        LPUART3_TX_SELECT_INPUT: 0x00000000,
        LPUART4_RX_SELECT_INPUT: 0x00000000,
        LPUART4_TX_SELECT_INPUT: 0x00000000,
        LPUART5_RX_SELECT_INPUT: 0x00000000,
        LPUART5_TX_SELECT_INPUT: 0x00000000,
        LPUART6_RX_SELECT_INPUT: 0x00000000,
        LPUART6_TX_SELECT_INPUT: 0x00000000,
        LPUART7_RX_SELECT_INPUT: 0x00000000,
        LPUART7_TX_SELECT_INPUT: 0x00000000,
        LPUART8_RX_SELECT_INPUT: 0x00000000,
        LPUART8_TX_SELECT_INPUT: 0x00000000,
        NMI_SELECT_INPUT: 0x00000000,
        QTIMER2_TIMER0_SELECT_INPUT: 0x00000000,
        QTIMER2_TIMER1_SELECT_INPUT: 0x00000000,
        QTIMER2_TIMER2_SELECT_INPUT: 0x00000000,
        QTIMER2_TIMER3_SELECT_INPUT: 0x00000000,
        QTIMER3_TIMER0_SELECT_INPUT: 0x00000000,
        QTIMER3_TIMER1_SELECT_INPUT: 0x00000000,
        QTIMER3_TIMER2_SELECT_INPUT: 0x00000000,
        QTIMER3_TIMER3_SELECT_INPUT: 0x00000000,
        SAI1_MCLK2_SELECT_INPUT: 0x00000000,
        SAI1_RX_BCLK_SELECT_INPUT: 0x00000000,
        SAI1_RX_DATA0_SELECT_INPUT: 0x00000000,
        SAI1_RX_DATA1_SELECT_INPUT: 0x00000000,
        SAI1_RX_DATA2_SELECT_INPUT: 0x00000000,
        SAI1_RX_DATA3_SELECT_INPUT: 0x00000000,
        SAI1_RX_SYNC_SELECT_INPUT: 0x00000000,
        SAI1_TX_BCLK_SELECT_INPUT: 0x00000000,
        SAI1_TX_SYNC_SELECT_INPUT: 0x00000000,
        SAI2_MCLK2_SELECT_INPUT: 0x00000000,
        SAI2_RX_BCLK_SELECT_INPUT: 0x00000000,
        SAI2_RX_DATA0_SELECT_INPUT: 0x00000000,
        SAI2_RX_SYNC_SELECT_INPUT: 0x00000000,
        SAI2_TX_BCLK_SELECT_INPUT: 0x00000000,
        SAI2_TX_SYNC_SELECT_INPUT: 0x00000000,
        SPDIF_IN_SELECT_INPUT: 0x00000000,
        USB_OTG2_OC_SELECT_INPUT: 0x00000000,
        USB_OTG1_OC_SELECT_INPUT: 0x00000000,
        USDHC1_CD_B_SELECT_INPUT: 0x00000000,
        USDHC1_WP_SELECT_INPUT: 0x00000000,
        USDHC2_CLK_SELECT_INPUT: 0x00000000,
        USDHC2_CD_B_SELECT_INPUT: 0x00000000,
        USDHC2_CMD_SELECT_INPUT: 0x00000000,
        USDHC2_DATA0_SELECT_INPUT: 0x00000000,
        USDHC2_DATA1_SELECT_INPUT: 0x00000000,
        USDHC2_DATA2_SELECT_INPUT: 0x00000000,
        USDHC2_DATA3_SELECT_INPUT: 0x00000000,
        USDHC2_DATA4_SELECT_INPUT: 0x00000000,
        USDHC2_DATA5_SELECT_INPUT: 0x00000000,
        USDHC2_DATA6_SELECT_INPUT: 0x00000000,
        USDHC2_DATA7_SELECT_INPUT: 0x00000000,
        USDHC2_WP_SELECT_INPUT: 0x00000000,
        XBAR1_IN02_SELECT_INPUT: 0x00000000,
        XBAR1_IN03_SELECT_INPUT: 0x00000000,
        XBAR1_IN04_SELECT_INPUT: 0x00000000,
        XBAR1_IN05_SELECT_INPUT: 0x00000000,
        XBAR1_IN06_SELECT_INPUT: 0x00000000,
        XBAR1_IN07_SELECT_INPUT: 0x00000000,
        XBAR1_IN08_SELECT_INPUT: 0x00000000,
        XBAR1_IN09_SELECT_INPUT: 0x00000000,
        XBAR1_IN17_SELECT_INPUT: 0x00000000,
        XBAR1_IN18_SELECT_INPUT: 0x00000000,
        XBAR1_IN20_SELECT_INPUT: 0x00000000,
        XBAR1_IN22_SELECT_INPUT: 0x00000000,
        XBAR1_IN23_SELECT_INPUT: 0x00000000,
        XBAR1_IN24_SELECT_INPUT: 0x00000000,
        XBAR1_IN14_SELECT_INPUT: 0x00000000,
        XBAR1_IN15_SELECT_INPUT: 0x00000000,
        XBAR1_IN16_SELECT_INPUT: 0x00000000,
        XBAR1_IN25_SELECT_INPUT: 0x00000000,
        XBAR1_IN19_SELECT_INPUT: 0x00000000,
        XBAR1_IN21_SELECT_INPUT: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut IOMUXC_TAKEN: bool = false;

    /// Safe access to IOMUXC
    ///
    /// This function returns `Some(Instance)` if this instance is not
    /// currently taken, and `None` if it is. This ensures that if you
    /// do get `Some(Instance)`, you are ensured unique access to
    /// the peripheral and there cannot be data races (unless other
    /// code uses `unsafe`, of course). You can then pass the
    /// `Instance` around to other functions as required. When you're
    /// done with it, you can call `release(instance)` to return it.
    ///
    /// `Instance` itself dereferences to a `RegisterBlock`, which
    /// provides access to the peripheral's registers.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn take() -> Option<Instance> {
        external_cortex_m::interrupt::free(|_| unsafe {
            if IOMUXC_TAKEN {
                None
            } else {
                IOMUXC_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to IOMUXC
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if IOMUXC_TAKEN && inst.addr == INSTANCE.addr {
                IOMUXC_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal IOMUXC
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        IOMUXC_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to IOMUXC
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const IOMUXC: *const RegisterBlock = 0x401f8000 as *const _;