#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Interrupt {
    #[doc = "0 - DMA0_DMA16"]
    DMA0_DMA16 = 0,
    #[doc = "1 - DMA1_DMA17"]
    DMA1_DMA17 = 1,
    #[doc = "2 - DMA2_DMA18"]
    DMA2_DMA18 = 2,
    #[doc = "3 - DMA3_DMA19"]
    DMA3_DMA19 = 3,
    #[doc = "4 - DMA4_DMA20"]
    DMA4_DMA20 = 4,
    #[doc = "5 - DMA5_DMA21"]
    DMA5_DMA21 = 5,
    #[doc = "6 - DMA6_DMA22"]
    DMA6_DMA22 = 6,
    #[doc = "7 - DMA7_DMA23"]
    DMA7_DMA23 = 7,
    #[doc = "8 - DMA8_DMA24"]
    DMA8_DMA24 = 8,
    #[doc = "9 - DMA9_DMA25"]
    DMA9_DMA25 = 9,
    #[doc = "10 - DMA10_DMA26"]
    DMA10_DMA26 = 10,
    #[doc = "11 - DMA11_DMA27"]
    DMA11_DMA27 = 11,
    #[doc = "12 - DMA12_DMA28"]
    DMA12_DMA28 = 12,
    #[doc = "13 - DMA13_DMA29"]
    DMA13_DMA29 = 13,
    #[doc = "14 - DMA14_DMA30"]
    DMA14_DMA30 = 14,
    #[doc = "15 - DMA15_DMA31"]
    DMA15_DMA31 = 15,
    #[doc = "16 - DMA_ERROR"]
    DMA_ERROR = 16,
    #[doc = "19 - CORE"]
    CORE = 19,
    #[doc = "20 - LPUART1"]
    LPUART1 = 20,
    #[doc = "21 - LPUART2"]
    LPUART2 = 21,
    #[doc = "22 - LPUART3"]
    LPUART3 = 22,
    #[doc = "23 - LPUART4"]
    LPUART4 = 23,
    #[doc = "24 - LPUART5"]
    LPUART5 = 24,
    #[doc = "25 - LPUART6"]
    LPUART6 = 25,
    #[doc = "26 - LPUART7"]
    LPUART7 = 26,
    #[doc = "27 - LPUART8"]
    LPUART8 = 27,
    #[doc = "28 - LPUART9"]
    LPUART9 = 28,
    #[doc = "29 - LPUART10"]
    LPUART10 = 29,
    #[doc = "30 - LPUART11"]
    LPUART11 = 30,
    #[doc = "31 - LPUART12"]
    LPUART12 = 31,
    #[doc = "32 - LPI2C1"]
    LPI2C1 = 32,
    #[doc = "33 - LPI2C2"]
    LPI2C2 = 33,
    #[doc = "34 - LPI2C3"]
    LPI2C3 = 34,
    #[doc = "35 - LPI2C4"]
    LPI2C4 = 35,
    #[doc = "36 - LPI2C5"]
    LPI2C5 = 36,
    #[doc = "37 - LPI2C6"]
    LPI2C6 = 37,
    #[doc = "38 - LPSPI1"]
    LPSPI1 = 38,
    #[doc = "39 - LPSPI2"]
    LPSPI2 = 39,
    #[doc = "40 - LPSPI3"]
    LPSPI3 = 40,
    #[doc = "41 - LPSPI4"]
    LPSPI4 = 41,
    #[doc = "42 - LPSPI5"]
    LPSPI5 = 42,
    #[doc = "43 - LPSPI6"]
    LPSPI6 = 43,
    #[doc = "44 - CAN1"]
    CAN1 = 44,
    #[doc = "45 - CAN1_ERROR"]
    CAN1_ERROR = 45,
    #[doc = "46 - CAN2"]
    CAN2 = 46,
    #[doc = "47 - CAN2_ERROR"]
    CAN2_ERROR = 47,
    #[doc = "48 - CAN3"]
    CAN3 = 48,
    #[doc = "49 - CAN3_ERROR"]
    CAN3_ERROR = 49,
    #[doc = "51 - KPP"]
    KPP = 51,
    #[doc = "53 - GPR_IRQ"]
    GPR_IRQ = 53,
    #[doc = "54 - ELCDIF"]
    ELCDIF = 54,
    #[doc = "55 - LCDIFV2"]
    LCDIFV2 = 55,
    #[doc = "56 - CSI"]
    CSI = 56,
    #[doc = "57 - PXP"]
    PXP = 57,
    #[doc = "58 - MIPI_CSI"]
    MIPI_CSI = 58,
    #[doc = "59 - MIPI_DSI"]
    MIPI_DSI = 59,
    #[doc = "61 - GPIO12_COMBINED_0_15"]
    GPIO12_COMBINED_0_15 = 61,
    #[doc = "62 - GPIO12_COMBINED_16_31"]
    GPIO12_COMBINED_16_31 = 62,
    #[doc = "63 - DAC"]
    DAC = 63,
    #[doc = "64 - KEY_MANAGER"]
    KEY_MANAGER = 64,
    #[doc = "65 - WDOG2"]
    WDOG2 = 65,
    #[doc = "66 - SNVS_HP_NON_TZ"]
    SNVS_HP_NON_TZ = 66,
    #[doc = "67 - SNVS_HP_TZ"]
    SNVS_HP_TZ = 67,
    #[doc = "68 - SNVS_PULSE_EVENT"]
    SNVS_PULSE_EVENT = 68,
    #[doc = "69 - CAAM_IRQ0"]
    CAAM_IRQ0 = 69,
    #[doc = "70 - CAAM_IRQ1"]
    CAAM_IRQ1 = 70,
    #[doc = "71 - CAAM_IRQ2"]
    CAAM_IRQ2 = 71,
    #[doc = "72 - CAAM_IRQ3"]
    CAAM_IRQ3 = 72,
    #[doc = "73 - CAAM_RECORVE_ERRPR"]
    CAAM_RECORVE_ERRPR = 73,
    #[doc = "74 - CAAM_RTIC"]
    CAAM_RTIC = 74,
    #[doc = "75 - CDOG"]
    CDOG = 75,
    #[doc = "76 - SAI1"]
    SAI1 = 76,
    #[doc = "77 - SAI2"]
    SAI2 = 77,
    #[doc = "78 - SAI3_RX"]
    SAI3_RX = 78,
    #[doc = "79 - SAI3_TX"]
    SAI3_TX = 79,
    #[doc = "80 - SAI4_RX"]
    SAI4_RX = 80,
    #[doc = "81 - SAI4_TX"]
    SAI4_TX = 81,
    #[doc = "82 - SPDIF"]
    SPDIF = 82,
    #[doc = "83 - TMPSNS_INT"]
    TMPSNS_INT = 83,
    #[doc = "84 - TMPSNS_LOW_HIGH"]
    TMPSNS_LOW_HIGH = 84,
    #[doc = "85 - TMPSNS_PANIC"]
    TMPSNS_PANIC = 85,
    #[doc = "86 - LPSR_LP8_BROWNOUT"]
    LPSR_LP8_BROWNOUT = 86,
    #[doc = "87 - LPSR_LP0_BROWNOUT"]
    LPSR_LP0_BROWNOUT = 87,
    #[doc = "88 - ADC1"]
    ADC1 = 88,
    #[doc = "89 - ADC2"]
    ADC2 = 89,
    #[doc = "90 - USBPHY1"]
    USBPHY1 = 90,
    #[doc = "91 - USBPHY2"]
    USBPHY2 = 91,
    #[doc = "92 - RDC"]
    RDC = 92,
    #[doc = "93 - GPIO13_COMBINED_0_31"]
    GPIO13_COMBINED_0_31 = 93,
    #[doc = "95 - DCIC1"]
    DCIC1 = 95,
    #[doc = "96 - DCIC2"]
    DCIC2 = 96,
    #[doc = "97 - ASRC"]
    ASRC = 97,
    #[doc = "98 - FLEXRAM_ECC"]
    FLEXRAM_ECC = 98,
    #[doc = "99 - GPIO7_8_9_10_11"]
    GPIO7_8_9_10_11 = 99,
    #[doc = "100 - GPIO1_COMBINED_0_15"]
    GPIO1_COMBINED_0_15 = 100,
    #[doc = "101 - GPIO1_COMBINED_16_31"]
    GPIO1_COMBINED_16_31 = 101,
    #[doc = "102 - GPIO2_COMBINED_0_15"]
    GPIO2_COMBINED_0_15 = 102,
    #[doc = "103 - GPIO2_COMBINED_16_31"]
    GPIO2_COMBINED_16_31 = 103,
    #[doc = "104 - GPIO3_COMBINED_0_15"]
    GPIO3_COMBINED_0_15 = 104,
    #[doc = "105 - GPIO3_COMBINED_16_31"]
    GPIO3_COMBINED_16_31 = 105,
    #[doc = "106 - GPIO4_COMBINED_0_15"]
    GPIO4_COMBINED_0_15 = 106,
    #[doc = "107 - GPIO4_COMBINED_16_31"]
    GPIO4_COMBINED_16_31 = 107,
    #[doc = "108 - GPIO5_COMBINED_0_15"]
    GPIO5_COMBINED_0_15 = 108,
    #[doc = "109 - GPIO5_COMBINED_16_31"]
    GPIO5_COMBINED_16_31 = 109,
    #[doc = "110 - FLEXIO1"]
    FLEXIO1 = 110,
    #[doc = "111 - FLEXIO2"]
    FLEXIO2 = 111,
    #[doc = "112 - WDOG1"]
    WDOG1 = 112,
    #[doc = "113 - RTWDOG4"]
    RTWDOG4 = 113,
    #[doc = "114 - EWM"]
    EWM = 114,
    #[doc = "115 - OCOTP_READ_FUSE_ERROR"]
    OCOTP_READ_FUSE_ERROR = 115,
    #[doc = "116 - OCOTP_READ_DONE_ERROR"]
    OCOTP_READ_DONE_ERROR = 116,
    #[doc = "117 - GPC"]
    GPC = 117,
    #[doc = "118 - MUB"]
    MUB = 118,
    #[doc = "119 - GPT1"]
    GPT1 = 119,
    #[doc = "120 - GPT2"]
    GPT2 = 120,
    #[doc = "121 - GPT3"]
    GPT3 = 121,
    #[doc = "122 - GPT4"]
    GPT4 = 122,
    #[doc = "123 - GPT5"]
    GPT5 = 123,
    #[doc = "124 - GPT6"]
    GPT6 = 124,
    #[doc = "125 - PWM1_0"]
    PWM1_0 = 125,
    #[doc = "126 - PWM1_1"]
    PWM1_1 = 126,
    #[doc = "127 - PWM1_2"]
    PWM1_2 = 127,
    #[doc = "128 - PWM1_3"]
    PWM1_3 = 128,
    #[doc = "129 - PWM1_FAULT"]
    PWM1_FAULT = 129,
    #[doc = "130 - FLEXSPI1"]
    FLEXSPI1 = 130,
    #[doc = "131 - FLEXSPI2"]
    FLEXSPI2 = 131,
    #[doc = "132 - SEMC"]
    SEMC = 132,
    #[doc = "133 - USDHC1"]
    USDHC1 = 133,
    #[doc = "134 - USDHC2"]
    USDHC2 = 134,
    #[doc = "135 - USB_OTG2"]
    USB_OTG2 = 135,
    #[doc = "136 - USB_OTG1"]
    USB_OTG1 = 136,
    #[doc = "137 - ENET"]
    ENET = 137,
    #[doc = "138 - ENET_1588_TIMER"]
    ENET_1588_TIMER = 138,
    #[doc = "139 - ENET_1G_MAC0_TX_RX_1"]
    ENET_1G_MAC0_TX_RX_1 = 139,
    #[doc = "140 - ENET_1G_MAC0_TX_RX_2"]
    ENET_1G_MAC0_TX_RX_2 = 140,
    #[doc = "141 - ENET_1G"]
    ENET_1G = 141,
    #[doc = "142 - ENET_1G_1588_TIMER"]
    ENET_1G_1588_TIMER = 142,
    #[doc = "143 - XBAR1_IRQ_0_1"]
    XBAR1_IRQ_0_1 = 143,
    #[doc = "144 - XBAR1_IRQ_2_3"]
    XBAR1_IRQ_2_3 = 144,
    #[doc = "145 - ADC_ETC_IRQ0"]
    ADC_ETC_IRQ0 = 145,
    #[doc = "146 - ADC_ETC_IRQ1"]
    ADC_ETC_IRQ1 = 146,
    #[doc = "147 - ADC_ETC_IRQ2"]
    ADC_ETC_IRQ2 = 147,
    #[doc = "148 - ADC_ETC_IRQ3"]
    ADC_ETC_IRQ3 = 148,
    #[doc = "149 - ADC_ETC_ERROR_IRQ"]
    ADC_ETC_ERROR_IRQ = 149,
    #[doc = "155 - PIT1"]
    PIT1 = 155,
    #[doc = "156 - PIT2"]
    PIT2 = 156,
    #[doc = "157 - ACMP1"]
    ACMP1 = 157,
    #[doc = "158 - ACMP2"]
    ACMP2 = 158,
    #[doc = "159 - ACMP3"]
    ACMP3 = 159,
    #[doc = "160 - ACMP4"]
    ACMP4 = 160,
    #[doc = "165 - ENC1"]
    ENC1 = 165,
    #[doc = "166 - ENC2"]
    ENC2 = 166,
    #[doc = "167 - ENC3"]
    ENC3 = 167,
    #[doc = "168 - ENC4"]
    ENC4 = 168,
    #[doc = "171 - TMR1"]
    TMR1 = 171,
    #[doc = "172 - TMR2"]
    TMR2 = 172,
    #[doc = "173 - TMR3"]
    TMR3 = 173,
    #[doc = "174 - TMR4"]
    TMR4 = 174,
    #[doc = "175 - SEMA4_CP0"]
    SEMA4_CP0 = 175,
    #[doc = "176 - SEMA4_CP1"]
    SEMA4_CP1 = 176,
    #[doc = "177 - PWM2_0"]
    PWM2_0 = 177,
    #[doc = "178 - PWM2_1"]
    PWM2_1 = 178,
    #[doc = "179 - PWM2_2"]
    PWM2_2 = 179,
    #[doc = "180 - PWM2_3"]
    PWM2_3 = 180,
    #[doc = "181 - PWM2_FAULT"]
    PWM2_FAULT = 181,
    #[doc = "182 - PWM3_0"]
    PWM3_0 = 182,
    #[doc = "183 - PWM3_1"]
    PWM3_1 = 183,
    #[doc = "184 - PWM3_2"]
    PWM3_2 = 184,
    #[doc = "185 - PWM3_3"]
    PWM3_3 = 185,
    #[doc = "186 - PWM3_FAULT"]
    PWM3_FAULT = 186,
    #[doc = "187 - PWM4_0"]
    PWM4_0 = 187,
    #[doc = "188 - PWM4_1"]
    PWM4_1 = 188,
    #[doc = "189 - PWM4_2"]
    PWM4_2 = 189,
    #[doc = "190 - PWM4_3"]
    PWM4_3 = 190,
    #[doc = "191 - PWM4_FAULT"]
    PWM4_FAULT = 191,
    #[doc = "200 - PDM_HWVAD_EVENT"]
    PDM_HWVAD_EVENT = 200,
    #[doc = "201 - PDM_HWVAD_ERROR"]
    PDM_HWVAD_ERROR = 201,
    #[doc = "202 - PDM_EVENT"]
    PDM_EVENT = 202,
    #[doc = "203 - PDM_ERROR"]
    PDM_ERROR = 203,
    #[doc = "204 - EMVSIM1"]
    EMVSIM1 = 204,
    #[doc = "205 - EMVSIM2"]
    EMVSIM2 = 205,
    #[doc = "206 - MECC1_INT"]
    MECC1_INT = 206,
    #[doc = "207 - MECC1_FATAL_INT"]
    MECC1_FATAL_INT = 207,
    #[doc = "208 - MECC2_INT"]
    MECC2_INT = 208,
    #[doc = "209 - MECC2_FATAL_INT"]
    MECC2_FATAL_INT = 209,
    #[doc = "210 - XECC_FLEXSPI1_INT"]
    XECC_FLEXSPI1_INT = 210,
    #[doc = "211 - XECC_FLEXSPI1_FATAL_INT"]
    XECC_FLEXSPI1_FATAL_INT = 211,
    #[doc = "212 - XECC_FLEXSPI2_INT"]
    XECC_FLEXSPI2_INT = 212,
    #[doc = "213 - XECC_FLEXSPI2_FATAL_INT"]
    XECC_FLEXSPI2_FATAL_INT = 213,
    #[doc = "214 - XECC_SEMC_INT"]
    XECC_SEMC_INT = 214,
    #[doc = "215 - XECC_SEMC_FATAL_INT"]
    XECC_SEMC_FATAL_INT = 215,
    #[doc = "216 - ENET_QOS"]
    ENET_QOS = 216,
    #[doc = "217 - ENET_QOS_PMT"]
    ENET_QOS_PMT = 217,
}
pub type interrupt = Interrupt;
unsafe impl cortex_m::interrupt::InterruptNumber for Interrupt {
    #[inline(always)]
    fn number(self) -> u16 {
        self as u16
    }
}
#[cfg(all(feature = "rt", target_os = "none"))]
mod _vectors {
    extern "C" {
        fn DMA0_DMA16();
        fn DMA1_DMA17();
        fn DMA2_DMA18();
        fn DMA3_DMA19();
        fn DMA4_DMA20();
        fn DMA5_DMA21();
        fn DMA6_DMA22();
        fn DMA7_DMA23();
        fn DMA8_DMA24();
        fn DMA9_DMA25();
        fn DMA10_DMA26();
        fn DMA11_DMA27();
        fn DMA12_DMA28();
        fn DMA13_DMA29();
        fn DMA14_DMA30();
        fn DMA15_DMA31();
        fn DMA_ERROR();
        fn CORE();
        fn LPUART1();
        fn LPUART2();
        fn LPUART3();
        fn LPUART4();
        fn LPUART5();
        fn LPUART6();
        fn LPUART7();
        fn LPUART8();
        fn LPUART9();
        fn LPUART10();
        fn LPUART11();
        fn LPUART12();
        fn LPI2C1();
        fn LPI2C2();
        fn LPI2C3();
        fn LPI2C4();
        fn LPI2C5();
        fn LPI2C6();
        fn LPSPI1();
        fn LPSPI2();
        fn LPSPI3();
        fn LPSPI4();
        fn LPSPI5();
        fn LPSPI6();
        fn CAN1();
        fn CAN1_ERROR();
        fn CAN2();
        fn CAN2_ERROR();
        fn CAN3();
        fn CAN3_ERROR();
        fn KPP();
        fn GPR_IRQ();
        fn ELCDIF();
        fn LCDIFV2();
        fn CSI();
        fn PXP();
        fn MIPI_CSI();
        fn MIPI_DSI();
        fn GPIO12_COMBINED_0_15();
        fn GPIO12_COMBINED_16_31();
        fn DAC();
        fn KEY_MANAGER();
        fn WDOG2();
        fn SNVS_HP_NON_TZ();
        fn SNVS_HP_TZ();
        fn SNVS_PULSE_EVENT();
        fn CAAM_IRQ0();
        fn CAAM_IRQ1();
        fn CAAM_IRQ2();
        fn CAAM_IRQ3();
        fn CAAM_RECORVE_ERRPR();
        fn CAAM_RTIC();
        fn CDOG();
        fn SAI1();
        fn SAI2();
        fn SAI3_RX();
        fn SAI3_TX();
        fn SAI4_RX();
        fn SAI4_TX();
        fn SPDIF();
        fn TMPSNS_INT();
        fn TMPSNS_LOW_HIGH();
        fn TMPSNS_PANIC();
        fn LPSR_LP8_BROWNOUT();
        fn LPSR_LP0_BROWNOUT();
        fn ADC1();
        fn ADC2();
        fn USBPHY1();
        fn USBPHY2();
        fn RDC();
        fn GPIO13_COMBINED_0_31();
        fn DCIC1();
        fn DCIC2();
        fn ASRC();
        fn FLEXRAM_ECC();
        fn GPIO7_8_9_10_11();
        fn GPIO1_COMBINED_0_15();
        fn GPIO1_COMBINED_16_31();
        fn GPIO2_COMBINED_0_15();
        fn GPIO2_COMBINED_16_31();
        fn GPIO3_COMBINED_0_15();
        fn GPIO3_COMBINED_16_31();
        fn GPIO4_COMBINED_0_15();
        fn GPIO4_COMBINED_16_31();
        fn GPIO5_COMBINED_0_15();
        fn GPIO5_COMBINED_16_31();
        fn FLEXIO1();
        fn FLEXIO2();
        fn WDOG1();
        fn RTWDOG4();
        fn EWM();
        fn OCOTP_READ_FUSE_ERROR();
        fn OCOTP_READ_DONE_ERROR();
        fn GPC();
        fn MUB();
        fn GPT1();
        fn GPT2();
        fn GPT3();
        fn GPT4();
        fn GPT5();
        fn GPT6();
        fn PWM1_0();
        fn PWM1_1();
        fn PWM1_2();
        fn PWM1_3();
        fn PWM1_FAULT();
        fn FLEXSPI1();
        fn FLEXSPI2();
        fn SEMC();
        fn USDHC1();
        fn USDHC2();
        fn USB_OTG2();
        fn USB_OTG1();
        fn ENET();
        fn ENET_1588_TIMER();
        fn ENET_1G_MAC0_TX_RX_1();
        fn ENET_1G_MAC0_TX_RX_2();
        fn ENET_1G();
        fn ENET_1G_1588_TIMER();
        fn XBAR1_IRQ_0_1();
        fn XBAR1_IRQ_2_3();
        fn ADC_ETC_IRQ0();
        fn ADC_ETC_IRQ1();
        fn ADC_ETC_IRQ2();
        fn ADC_ETC_IRQ3();
        fn ADC_ETC_ERROR_IRQ();
        fn PIT1();
        fn PIT2();
        fn ACMP1();
        fn ACMP2();
        fn ACMP3();
        fn ACMP4();
        fn ENC1();
        fn ENC2();
        fn ENC3();
        fn ENC4();
        fn TMR1();
        fn TMR2();
        fn TMR3();
        fn TMR4();
        fn SEMA4_CP0();
        fn SEMA4_CP1();
        fn PWM2_0();
        fn PWM2_1();
        fn PWM2_2();
        fn PWM2_3();
        fn PWM2_FAULT();
        fn PWM3_0();
        fn PWM3_1();
        fn PWM3_2();
        fn PWM3_3();
        fn PWM3_FAULT();
        fn PWM4_0();
        fn PWM4_1();
        fn PWM4_2();
        fn PWM4_3();
        fn PWM4_FAULT();
        fn PDM_HWVAD_EVENT();
        fn PDM_HWVAD_ERROR();
        fn PDM_EVENT();
        fn PDM_ERROR();
        fn EMVSIM1();
        fn EMVSIM2();
        fn MECC1_INT();
        fn MECC1_FATAL_INT();
        fn MECC2_INT();
        fn MECC2_FATAL_INT();
        fn XECC_FLEXSPI1_INT();
        fn XECC_FLEXSPI1_FATAL_INT();
        fn XECC_FLEXSPI2_INT();
        fn XECC_FLEXSPI2_FATAL_INT();
        fn XECC_SEMC_INT();
        fn XECC_SEMC_FATAL_INT();
        fn ENET_QOS();
        fn ENET_QOS_PMT();
    }
    pub union Vector {
        _handler: unsafe extern "C" fn(),
        _reserved: u32,
    }
    #[cfg_attr(target_os = "none", link_section = ".vector_table.interrupts")]
    #[no_mangle]
    pub static __INTERRUPTS: [Vector; 218] = [
        Vector {
            _handler: DMA0_DMA16,
        },
        Vector {
            _handler: DMA1_DMA17,
        },
        Vector {
            _handler: DMA2_DMA18,
        },
        Vector {
            _handler: DMA3_DMA19,
        },
        Vector {
            _handler: DMA4_DMA20,
        },
        Vector {
            _handler: DMA5_DMA21,
        },
        Vector {
            _handler: DMA6_DMA22,
        },
        Vector {
            _handler: DMA7_DMA23,
        },
        Vector {
            _handler: DMA8_DMA24,
        },
        Vector {
            _handler: DMA9_DMA25,
        },
        Vector {
            _handler: DMA10_DMA26,
        },
        Vector {
            _handler: DMA11_DMA27,
        },
        Vector {
            _handler: DMA12_DMA28,
        },
        Vector {
            _handler: DMA13_DMA29,
        },
        Vector {
            _handler: DMA14_DMA30,
        },
        Vector {
            _handler: DMA15_DMA31,
        },
        Vector {
            _handler: DMA_ERROR,
        },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _handler: CORE },
        Vector { _handler: LPUART1 },
        Vector { _handler: LPUART2 },
        Vector { _handler: LPUART3 },
        Vector { _handler: LPUART4 },
        Vector { _handler: LPUART5 },
        Vector { _handler: LPUART6 },
        Vector { _handler: LPUART7 },
        Vector { _handler: LPUART8 },
        Vector { _handler: LPUART9 },
        Vector { _handler: LPUART10 },
        Vector { _handler: LPUART11 },
        Vector { _handler: LPUART12 },
        Vector { _handler: LPI2C1 },
        Vector { _handler: LPI2C2 },
        Vector { _handler: LPI2C3 },
        Vector { _handler: LPI2C4 },
        Vector { _handler: LPI2C5 },
        Vector { _handler: LPI2C6 },
        Vector { _handler: LPSPI1 },
        Vector { _handler: LPSPI2 },
        Vector { _handler: LPSPI3 },
        Vector { _handler: LPSPI4 },
        Vector { _handler: LPSPI5 },
        Vector { _handler: LPSPI6 },
        Vector { _handler: CAN1 },
        Vector {
            _handler: CAN1_ERROR,
        },
        Vector { _handler: CAN2 },
        Vector {
            _handler: CAN2_ERROR,
        },
        Vector { _handler: CAN3 },
        Vector {
            _handler: CAN3_ERROR,
        },
        Vector { _reserved: 0 },
        Vector { _handler: KPP },
        Vector { _reserved: 0 },
        Vector { _handler: GPR_IRQ },
        Vector { _handler: ELCDIF },
        Vector { _handler: LCDIFV2 },
        Vector { _handler: CSI },
        Vector { _handler: PXP },
        Vector { _handler: MIPI_CSI },
        Vector { _handler: MIPI_DSI },
        Vector { _reserved: 0 },
        Vector {
            _handler: GPIO12_COMBINED_0_15,
        },
        Vector {
            _handler: GPIO12_COMBINED_16_31,
        },
        Vector { _handler: DAC },
        Vector {
            _handler: KEY_MANAGER,
        },
        Vector { _handler: WDOG2 },
        Vector {
            _handler: SNVS_HP_NON_TZ,
        },
        Vector {
            _handler: SNVS_HP_TZ,
        },
        Vector {
            _handler: SNVS_PULSE_EVENT,
        },
        Vector {
            _handler: CAAM_IRQ0,
        },
        Vector {
            _handler: CAAM_IRQ1,
        },
        Vector {
            _handler: CAAM_IRQ2,
        },
        Vector {
            _handler: CAAM_IRQ3,
        },
        Vector {
            _handler: CAAM_RECORVE_ERRPR,
        },
        Vector {
            _handler: CAAM_RTIC,
        },
        Vector { _handler: CDOG },
        Vector { _handler: SAI1 },
        Vector { _handler: SAI2 },
        Vector { _handler: SAI3_RX },
        Vector { _handler: SAI3_TX },
        Vector { _handler: SAI4_RX },
        Vector { _handler: SAI4_TX },
        Vector { _handler: SPDIF },
        Vector {
            _handler: TMPSNS_INT,
        },
        Vector {
            _handler: TMPSNS_LOW_HIGH,
        },
        Vector {
            _handler: TMPSNS_PANIC,
        },
        Vector {
            _handler: LPSR_LP8_BROWNOUT,
        },
        Vector {
            _handler: LPSR_LP0_BROWNOUT,
        },
        Vector { _handler: ADC1 },
        Vector { _handler: ADC2 },
        Vector { _handler: USBPHY1 },
        Vector { _handler: USBPHY2 },
        Vector { _handler: RDC },
        Vector {
            _handler: GPIO13_COMBINED_0_31,
        },
        Vector { _reserved: 0 },
        Vector { _handler: DCIC1 },
        Vector { _handler: DCIC2 },
        Vector { _handler: ASRC },
        Vector {
            _handler: FLEXRAM_ECC,
        },
        Vector {
            _handler: GPIO7_8_9_10_11,
        },
        Vector {
            _handler: GPIO1_COMBINED_0_15,
        },
        Vector {
            _handler: GPIO1_COMBINED_16_31,
        },
        Vector {
            _handler: GPIO2_COMBINED_0_15,
        },
        Vector {
            _handler: GPIO2_COMBINED_16_31,
        },
        Vector {
            _handler: GPIO3_COMBINED_0_15,
        },
        Vector {
            _handler: GPIO3_COMBINED_16_31,
        },
        Vector {
            _handler: GPIO4_COMBINED_0_15,
        },
        Vector {
            _handler: GPIO4_COMBINED_16_31,
        },
        Vector {
            _handler: GPIO5_COMBINED_0_15,
        },
        Vector {
            _handler: GPIO5_COMBINED_16_31,
        },
        Vector { _handler: FLEXIO1 },
        Vector { _handler: FLEXIO2 },
        Vector { _handler: WDOG1 },
        Vector { _handler: RTWDOG4 },
        Vector { _handler: EWM },
        Vector {
            _handler: OCOTP_READ_FUSE_ERROR,
        },
        Vector {
            _handler: OCOTP_READ_DONE_ERROR,
        },
        Vector { _handler: GPC },
        Vector { _handler: MUB },
        Vector { _handler: GPT1 },
        Vector { _handler: GPT2 },
        Vector { _handler: GPT3 },
        Vector { _handler: GPT4 },
        Vector { _handler: GPT5 },
        Vector { _handler: GPT6 },
        Vector { _handler: PWM1_0 },
        Vector { _handler: PWM1_1 },
        Vector { _handler: PWM1_2 },
        Vector { _handler: PWM1_3 },
        Vector {
            _handler: PWM1_FAULT,
        },
        Vector { _handler: FLEXSPI1 },
        Vector { _handler: FLEXSPI2 },
        Vector { _handler: SEMC },
        Vector { _handler: USDHC1 },
        Vector { _handler: USDHC2 },
        Vector { _handler: USB_OTG2 },
        Vector { _handler: USB_OTG1 },
        Vector { _handler: ENET },
        Vector {
            _handler: ENET_1588_TIMER,
        },
        Vector {
            _handler: ENET_1G_MAC0_TX_RX_1,
        },
        Vector {
            _handler: ENET_1G_MAC0_TX_RX_2,
        },
        Vector { _handler: ENET_1G },
        Vector {
            _handler: ENET_1G_1588_TIMER,
        },
        Vector {
            _handler: XBAR1_IRQ_0_1,
        },
        Vector {
            _handler: XBAR1_IRQ_2_3,
        },
        Vector {
            _handler: ADC_ETC_IRQ0,
        },
        Vector {
            _handler: ADC_ETC_IRQ1,
        },
        Vector {
            _handler: ADC_ETC_IRQ2,
        },
        Vector {
            _handler: ADC_ETC_IRQ3,
        },
        Vector {
            _handler: ADC_ETC_ERROR_IRQ,
        },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _handler: PIT1 },
        Vector { _handler: PIT2 },
        Vector { _handler: ACMP1 },
        Vector { _handler: ACMP2 },
        Vector { _handler: ACMP3 },
        Vector { _handler: ACMP4 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _handler: ENC1 },
        Vector { _handler: ENC2 },
        Vector { _handler: ENC3 },
        Vector { _handler: ENC4 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _handler: TMR1 },
        Vector { _handler: TMR2 },
        Vector { _handler: TMR3 },
        Vector { _handler: TMR4 },
        Vector {
            _handler: SEMA4_CP0,
        },
        Vector {
            _handler: SEMA4_CP1,
        },
        Vector { _handler: PWM2_0 },
        Vector { _handler: PWM2_1 },
        Vector { _handler: PWM2_2 },
        Vector { _handler: PWM2_3 },
        Vector {
            _handler: PWM2_FAULT,
        },
        Vector { _handler: PWM3_0 },
        Vector { _handler: PWM3_1 },
        Vector { _handler: PWM3_2 },
        Vector { _handler: PWM3_3 },
        Vector {
            _handler: PWM3_FAULT,
        },
        Vector { _handler: PWM4_0 },
        Vector { _handler: PWM4_1 },
        Vector { _handler: PWM4_2 },
        Vector { _handler: PWM4_3 },
        Vector {
            _handler: PWM4_FAULT,
        },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector {
            _handler: PDM_HWVAD_EVENT,
        },
        Vector {
            _handler: PDM_HWVAD_ERROR,
        },
        Vector {
            _handler: PDM_EVENT,
        },
        Vector {
            _handler: PDM_ERROR,
        },
        Vector { _handler: EMVSIM1 },
        Vector { _handler: EMVSIM2 },
        Vector {
            _handler: MECC1_INT,
        },
        Vector {
            _handler: MECC1_FATAL_INT,
        },
        Vector {
            _handler: MECC2_INT,
        },
        Vector {
            _handler: MECC2_FATAL_INT,
        },
        Vector {
            _handler: XECC_FLEXSPI1_INT,
        },
        Vector {
            _handler: XECC_FLEXSPI1_FATAL_INT,
        },
        Vector {
            _handler: XECC_FLEXSPI2_INT,
        },
        Vector {
            _handler: XECC_FLEXSPI2_FATAL_INT,
        },
        Vector {
            _handler: XECC_SEMC_INT,
        },
        Vector {
            _handler: XECC_SEMC_FATAL_INT,
        },
        Vector { _handler: ENET_QOS },
        Vector {
            _handler: ENET_QOS_PMT,
        },
    ];
}
#[path = "."]
pub mod adc_etc {
    #[doc = "ADC_ETC"]
    pub const ADC_ETC: *const RegisterBlock = 0x4004_8000 as *const RegisterBlock;
    #[path = "blocks/imxrt1176_cm4/adc_etc.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type ADC_ETC = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for ADC_ETC {}
    impl crate::Valid for ADC_ETC {}
    impl ADC_ETC {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(ADC_ETC)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, ADC_ETC).then_some(0)
    }
}
#[path = "."]
pub mod anadig_ldo_snvs {
    #[doc = "MX6RT_ANADIG_REGISTER"]
    pub const ANADIG_LDO_SNVS: *const RegisterBlock = 0x40c8_4000 as *const RegisterBlock;
    #[path = "blocks/imxrt1176_cm4/anadig_ldo_snvs.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type ANADIG_LDO_SNVS = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for ANADIG_LDO_SNVS {}
    impl crate::Valid for ANADIG_LDO_SNVS {}
    impl ANADIG_LDO_SNVS {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(ANADIG_LDO_SNVS)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, ANADIG_LDO_SNVS).then_some(0)
    }
}
#[path = "."]
pub mod anadig_ldo_snvs_dig {
    #[doc = "MX6RT_ANADIG_REGISTER"]
    pub const ANADIG_LDO_SNVS_DIG: *const RegisterBlock = 0x40c8_4000 as *const RegisterBlock;
    #[path = "blocks/imxrt1176_cm4/anadig_ldo_snvs_dig.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type ANADIG_LDO_SNVS_DIG = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for ANADIG_LDO_SNVS_DIG {}
    impl crate::Valid for ANADIG_LDO_SNVS_DIG {}
    impl ANADIG_LDO_SNVS_DIG {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(ANADIG_LDO_SNVS_DIG)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, ANADIG_LDO_SNVS_DIG).then_some(0)
    }
}
#[path = "."]
pub mod anadig_misc {
    #[doc = "MX6RT_ANADIG_REGISTER"]
    pub const ANADIG_MISC: *const RegisterBlock = 0x40c8_4000 as *const RegisterBlock;
    #[path = "blocks/imxrt1176_cm4/anadig_misc.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type ANADIG_MISC = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for ANADIG_MISC {}
    impl crate::Valid for ANADIG_MISC {}
    impl ANADIG_MISC {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(ANADIG_MISC)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, ANADIG_MISC).then_some(0)
    }
}
#[path = "."]
pub mod anadig_osc {
    #[doc = "MX6RT_ANADIG_REGISTER"]
    pub const ANADIG_OSC: *const RegisterBlock = 0x40c8_4000 as *const RegisterBlock;
    #[path = "blocks/imxrt1176_cm4/anadig_osc.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type ANADIG_OSC = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for ANADIG_OSC {}
    impl crate::Valid for ANADIG_OSC {}
    impl ANADIG_OSC {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(ANADIG_OSC)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, ANADIG_OSC).then_some(0)
    }
}
#[path = "."]
pub mod anadig_pll {
    #[doc = "MX6RT_ANADIG_REGISTER"]
    pub const ANADIG_PLL: *const RegisterBlock = 0x40c8_4000 as *const RegisterBlock;
    #[path = "blocks/imxrt1176_cm4/anadig_pll.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type ANADIG_PLL = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for ANADIG_PLL {}
    impl crate::Valid for ANADIG_PLL {}
    impl ANADIG_PLL {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(ANADIG_PLL)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, ANADIG_PLL).then_some(0)
    }
}
#[path = "."]
pub mod anadig_pmu {
    #[doc = "MX6RT_ANADIG_REGISTER"]
    pub const ANADIG_PMU: *const RegisterBlock = 0x40c8_4000 as *const RegisterBlock;
    #[path = "blocks/imxrt1176_cm4/anadig_pmu.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type ANADIG_PMU = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for ANADIG_PMU {}
    impl crate::Valid for ANADIG_PMU {}
    impl ANADIG_PMU {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(ANADIG_PMU)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, ANADIG_PMU).then_some(0)
    }
}
#[path = "."]
pub mod anadig_tempsensor {
    #[doc = "MX6RT_ANADIG_REGISTER"]
    pub const ANADIG_TEMPSENSOR: *const RegisterBlock = 0x40c8_4000 as *const RegisterBlock;
    #[path = "blocks/imxrt1176_cm4/anadig_tempsensor.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type ANADIG_TEMPSENSOR = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for ANADIG_TEMPSENSOR {}
    impl crate::Valid for ANADIG_TEMPSENSOR {}
    impl ANADIG_TEMPSENSOR {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(ANADIG_TEMPSENSOR)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, ANADIG_TEMPSENSOR).then_some(0)
    }
}
#[path = "."]
pub mod aoi {
    #[doc = "AOI"]
    pub const AOI1: *const RegisterBlock = 0x400b_8000 as *const RegisterBlock;
    #[doc = "AOI"]
    pub const AOI2: *const RegisterBlock = 0x400b_c000 as *const RegisterBlock;
    #[path = "blocks/imxrt1176_cm4/aoi.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type AOI1 = Instance<1>;
    impl crate::private::Sealed for AOI1 {}
    impl crate::Valid for AOI1 {}
    impl AOI1 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(AOI1)
        }
    }
    pub type AOI2 = Instance<2>;
    impl crate::private::Sealed for AOI2 {}
    impl crate::Valid for AOI2 {}
    impl AOI2 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(AOI2)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        [(AOI1, 1), (AOI2, 2)]
            .into_iter()
            .find(|(ptr, _)| core::ptr::eq(rb, *ptr))
            .map(|(_, inst)| inst)
    }
}
#[path = "."]
pub mod asrc {
    #[doc = "ASRC"]
    pub const ASRC: *const RegisterBlock = 0x4041_4000 as *const RegisterBlock;
    #[path = "blocks/imxrt1176_cm4/asrc.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type ASRC = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for ASRC {}
    impl crate::Valid for ASRC {}
    impl ASRC {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(ASRC)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, ASRC).then_some(0)
    }
}
#[path = "."]
pub mod can {
    #[doc = "CAN"]
    pub const CAN1: *const RegisterBlock = 0x400c_4000 as *const RegisterBlock;
    #[doc = "CAN"]
    pub const CAN2: *const RegisterBlock = 0x400c_8000 as *const RegisterBlock;
    #[doc = "CAN"]
    pub const CAN3: *const RegisterBlock = 0x40c3_c000 as *const RegisterBlock;
    #[path = "blocks/imxrt1176_cm4/can.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type CAN1 = Instance<1>;
    impl crate::private::Sealed for CAN1 {}
    impl crate::Valid for CAN1 {}
    impl CAN1 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(CAN1)
        }
    }
    pub type CAN2 = Instance<2>;
    impl crate::private::Sealed for CAN2 {}
    impl crate::Valid for CAN2 {}
    impl CAN2 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(CAN2)
        }
    }
    pub type CAN3 = Instance<3>;
    impl crate::private::Sealed for CAN3 {}
    impl crate::Valid for CAN3 {}
    impl CAN3 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(CAN3)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        [(CAN1, 1), (CAN2, 2), (CAN3, 3)]
            .into_iter()
            .find(|(ptr, _)| core::ptr::eq(rb, *ptr))
            .map(|(_, inst)| inst)
    }
}
#[path = "."]
pub mod can1_wrapper {
    #[doc = "FlexCAN wrapper"]
    pub const CAN_WRAPPER1: *const RegisterBlock = 0x400c_4000 as *const RegisterBlock;
    #[doc = "FlexCAN wrapper"]
    pub const CAN_WRAPPER2: *const RegisterBlock = 0x400c_8000 as *const RegisterBlock;
    #[doc = "FlexCAN wrapper"]
    pub const CAN_WRAPPER3: *const RegisterBlock = 0x40c3_c000 as *const RegisterBlock;
    #[path = "blocks/imxrt1176_cm4/can1_wrapper.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type CAN_WRAPPER1 = Instance<1>;
    impl crate::private::Sealed for CAN_WRAPPER1 {}
    impl crate::Valid for CAN_WRAPPER1 {}
    impl CAN_WRAPPER1 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(CAN_WRAPPER1)
        }
    }
    pub type CAN_WRAPPER2 = Instance<2>;
    impl crate::private::Sealed for CAN_WRAPPER2 {}
    impl crate::Valid for CAN_WRAPPER2 {}
    impl CAN_WRAPPER2 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(CAN_WRAPPER2)
        }
    }
    pub type CAN_WRAPPER3 = Instance<3>;
    impl crate::private::Sealed for CAN_WRAPPER3 {}
    impl crate::Valid for CAN_WRAPPER3 {}
    impl CAN_WRAPPER3 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(CAN_WRAPPER3)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        [(CAN_WRAPPER1, 1), (CAN_WRAPPER2, 2), (CAN_WRAPPER3, 3)]
            .into_iter()
            .find(|(ptr, _)| core::ptr::eq(rb, *ptr))
            .map(|(_, inst)| inst)
    }
}
#[path = "."]
pub mod ccm {
    #[doc = "CCM"]
    pub const CCM: *const RegisterBlock = 0x40cc_0000 as *const RegisterBlock;
    #[path = "blocks/imxrt1176_cm4/ccm.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type CCM = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for CCM {}
    impl crate::Valid for CCM {}
    impl CCM {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(CCM)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, CCM).then_some(0)
    }
}
#[path = "."]
pub mod ccm_obs {
    #[doc = "CCM_OBS"]
    pub const CCM_OBS: *const RegisterBlock = 0x4015_0000 as *const RegisterBlock;
    #[path = "blocks/imxrt1176_cm4/ccm_obs.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type CCM_OBS = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for CCM_OBS {}
    impl crate::Valid for CCM_OBS {}
    impl CCM_OBS {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(CCM_OBS)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, CCM_OBS).then_some(0)
    }
}
#[path = "."]
pub mod cdog {
    #[doc = "CDOG"]
    pub const CDOG: *const RegisterBlock = 0x4190_0000 as *const RegisterBlock;
    #[path = "blocks/imxrt1176_cm4/cdog.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type CDOG = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for CDOG {}
    impl crate::Valid for CDOG {}
    impl CDOG {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(CDOG)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, CDOG).then_some(0)
    }
}
#[path = "."]
pub mod cmp {
    #[doc = "CMP"]
    pub const CMP1: *const RegisterBlock = 0x401a_4000 as *const RegisterBlock;
    #[doc = "CMP"]
    pub const CMP2: *const RegisterBlock = 0x401a_8000 as *const RegisterBlock;
    #[doc = "CMP"]
    pub const CMP3: *const RegisterBlock = 0x401a_c000 as *const RegisterBlock;
    #[doc = "CMP"]
    pub const CMP4: *const RegisterBlock = 0x401b_0000 as *const RegisterBlock;
    #[path = "blocks/imxrt1176_cm4/cmp.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type CMP1 = Instance<1>;
    impl crate::private::Sealed for CMP1 {}
    impl crate::Valid for CMP1 {}
    impl CMP1 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(CMP1)
        }
    }
    pub type CMP2 = Instance<2>;
    impl crate::private::Sealed for CMP2 {}
    impl crate::Valid for CMP2 {}
    impl CMP2 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(CMP2)
        }
    }
    pub type CMP3 = Instance<3>;
    impl crate::private::Sealed for CMP3 {}
    impl crate::Valid for CMP3 {}
    impl CMP3 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(CMP3)
        }
    }
    pub type CMP4 = Instance<4>;
    impl crate::private::Sealed for CMP4 {}
    impl crate::Valid for CMP4 {}
    impl CMP4 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(CMP4)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        [(CMP1, 1), (CMP2, 2), (CMP3, 3), (CMP4, 4)]
            .into_iter()
            .find(|(ptr, _)| core::ptr::eq(rb, *ptr))
            .map(|(_, inst)| inst)
    }
}
#[path = "."]
pub mod csi {
    #[doc = "CSI"]
    pub const CSI: *const RegisterBlock = 0x4080_0000 as *const RegisterBlock;
    #[path = "blocks/imxrt1176_cm4/csi.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type CSI = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for CSI {}
    impl crate::Valid for CSI {}
    impl CSI {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(CSI)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, CSI).then_some(0)
    }
}
#[path = "."]
pub mod dac {
    #[doc = "DAC"]
    pub const DAC: *const RegisterBlock = 0x4006_4000 as *const RegisterBlock;
    #[path = "blocks/imxrt1176_cm4/dac.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type DAC = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for DAC {}
    impl crate::Valid for DAC {}
    impl DAC {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(DAC)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, DAC).then_some(0)
    }
}
#[path = "."]
pub mod dcdc {
    #[doc = "DCDC"]
    pub const DCDC: *const RegisterBlock = 0x40ca_8000 as *const RegisterBlock;
    #[path = "blocks/imxrt1176_cm4/dcdc.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type DCDC = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for DCDC {}
    impl crate::Valid for DCDC {}
    impl DCDC {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(DCDC)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, DCDC).then_some(0)
    }
}
#[path = "."]
pub mod dcic {
    #[doc = "DCIC"]
    pub const DCIC1: *const RegisterBlock = 0x4081_9000 as *const RegisterBlock;
    #[doc = "DCIC"]
    pub const DCIC2: *const RegisterBlock = 0x4081_a000 as *const RegisterBlock;
    #[path = "blocks/imxrt1176_cm4/dcic.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type DCIC1 = Instance<1>;
    impl crate::private::Sealed for DCIC1 {}
    impl crate::Valid for DCIC1 {}
    impl DCIC1 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(DCIC1)
        }
    }
    pub type DCIC2 = Instance<2>;
    impl crate::private::Sealed for DCIC2 {}
    impl crate::Valid for DCIC2 {}
    impl DCIC2 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(DCIC2)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        [(DCIC1, 1), (DCIC2, 2)]
            .into_iter()
            .find(|(ptr, _)| core::ptr::eq(rb, *ptr))
            .map(|(_, inst)| inst)
    }
}
#[path = "."]
pub mod dma {
    #[doc = "DMA"]
    pub const DMA: *const RegisterBlock = 0x40c1_4000 as *const RegisterBlock;
    #[path = "blocks/imxrt1176_cm4/dma.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type DMA = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for DMA {}
    impl crate::Valid for DMA {}
    impl DMA {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(DMA)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, DMA).then_some(0)
    }
}
#[path = "."]
pub mod dmamux {
    #[doc = "DMAMUX"]
    pub const DMAMUX: *const RegisterBlock = 0x40c1_8000 as *const RegisterBlock;
    #[path = "blocks/imxrt1176_cm4/dmamux.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type DMAMUX = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for DMAMUX {}
    impl crate::Valid for DMAMUX {}
    impl DMAMUX {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(DMAMUX)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, DMAMUX).then_some(0)
    }
}
#[path = "."]
pub mod dsi_host {
    #[doc = "DSI HOST"]
    pub const DSI_HOST: *const RegisterBlock = 0x4080_c000 as *const RegisterBlock;
    #[path = "blocks/imxrt1176_cm4/dsi_host.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type DSI_HOST = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for DSI_HOST {}
    impl crate::Valid for DSI_HOST {}
    impl DSI_HOST {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(DSI_HOST)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, DSI_HOST).then_some(0)
    }
}
#[path = "."]
pub mod dsi_host_apb_pkt_if {
    #[doc = "DSI HOST APB PKT Interface"]
    pub const DSI_HOST_APB_PKT_IF: *const RegisterBlock = 0x4080_c280 as *const RegisterBlock;
    #[path = "blocks/imxrt1176_cm4/dsi_host_apb_pkt_if.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type DSI_HOST_APB_PKT_IF = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for DSI_HOST_APB_PKT_IF {}
    impl crate::Valid for DSI_HOST_APB_PKT_IF {}
    impl DSI_HOST_APB_PKT_IF {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(DSI_HOST_APB_PKT_IF)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, DSI_HOST_APB_PKT_IF).then_some(0)
    }
}
#[path = "."]
pub mod dsi_host_dphy_intfc {
    #[doc = "DSI HOST DPHY INTFC"]
    pub const DSI_HOST_DPHY_INTFC: *const RegisterBlock = 0x4080_c300 as *const RegisterBlock;
    #[path = "blocks/imxrt1176_cm4/dsi_host_dphy_intfc.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type DSI_HOST_DPHY_INTFC = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for DSI_HOST_DPHY_INTFC {}
    impl crate::Valid for DSI_HOST_DPHY_INTFC {}
    impl DSI_HOST_DPHY_INTFC {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(DSI_HOST_DPHY_INTFC)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, DSI_HOST_DPHY_INTFC).then_some(0)
    }
}
#[path = "."]
pub mod dsi_host_dpi_intfc {
    #[doc = "DSI Host DPI Interface"]
    pub const DSI_HOST_DPI_INTFC: *const RegisterBlock = 0x4080_c200 as *const RegisterBlock;
    #[path = "blocks/imxrt1176_cm4/dsi_host_dpi_intfc.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type DSI_HOST_DPI_INTFC = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for DSI_HOST_DPI_INTFC {}
    impl crate::Valid for DSI_HOST_DPI_INTFC {}
    impl DSI_HOST_DPI_INTFC {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(DSI_HOST_DPI_INTFC)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, DSI_HOST_DPI_INTFC).then_some(0)
    }
}
#[path = "."]
pub mod emvsim {
    #[doc = "EMVSIM"]
    pub const EMVSIM1: *const RegisterBlock = 0x4015_4000 as *const RegisterBlock;
    #[doc = "EMVSIM"]
    pub const EMVSIM2: *const RegisterBlock = 0x4015_8000 as *const RegisterBlock;
    #[path = "blocks/imxrt1176_cm4/emvsim.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type EMVSIM1 = Instance<1>;
    impl crate::private::Sealed for EMVSIM1 {}
    impl crate::Valid for EMVSIM1 {}
    impl EMVSIM1 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(EMVSIM1)
        }
    }
    pub type EMVSIM2 = Instance<2>;
    impl crate::private::Sealed for EMVSIM2 {}
    impl crate::Valid for EMVSIM2 {}
    impl EMVSIM2 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(EMVSIM2)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        [(EMVSIM1, 1), (EMVSIM2, 2)]
            .into_iter()
            .find(|(ptr, _)| core::ptr::eq(rb, *ptr))
            .map(|(_, inst)| inst)
    }
}
#[path = "."]
pub mod enc {
    #[doc = "QDC"]
    pub const ENC1: *const RegisterBlock = 0x4017_4000 as *const RegisterBlock;
    #[doc = "QDC"]
    pub const ENC2: *const RegisterBlock = 0x4017_8000 as *const RegisterBlock;
    #[doc = "QDC"]
    pub const ENC3: *const RegisterBlock = 0x4017_c000 as *const RegisterBlock;
    #[doc = "QDC"]
    pub const ENC4: *const RegisterBlock = 0x4018_0000 as *const RegisterBlock;
    #[path = "blocks/imxrt1176_cm4/enc.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type ENC1 = Instance<1>;
    impl crate::private::Sealed for ENC1 {}
    impl crate::Valid for ENC1 {}
    impl ENC1 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(ENC1)
        }
    }
    pub type ENC2 = Instance<2>;
    impl crate::private::Sealed for ENC2 {}
    impl crate::Valid for ENC2 {}
    impl ENC2 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(ENC2)
        }
    }
    pub type ENC3 = Instance<3>;
    impl crate::private::Sealed for ENC3 {}
    impl crate::Valid for ENC3 {}
    impl ENC3 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(ENC3)
        }
    }
    pub type ENC4 = Instance<4>;
    impl crate::private::Sealed for ENC4 {}
    impl crate::Valid for ENC4 {}
    impl ENC4 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(ENC4)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        [(ENC1, 1), (ENC2, 2), (ENC3, 3), (ENC4, 4)]
            .into_iter()
            .find(|(ptr, _)| core::ptr::eq(rb, *ptr))
            .map(|(_, inst)| inst)
    }
}
#[path = "."]
pub mod enet {
    #[doc = "ENET"]
    pub const ENET: *const RegisterBlock = 0x4042_4000 as *const RegisterBlock;
    #[path = "blocks/imxrt1176_cm4/enet.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type ENET = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for ENET {}
    impl crate::Valid for ENET {}
    impl ENET {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(ENET)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, ENET).then_some(0)
    }
}
#[path = "."]
pub mod enet_1g {
    #[doc = "ENET"]
    pub const ENET_1G: *const RegisterBlock = 0x4042_0000 as *const RegisterBlock;
    #[path = "blocks/imxrt1176_cm4/enet_1g.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type ENET_1G = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for ENET_1G {}
    impl crate::Valid for ENET_1G {}
    impl ENET_1G {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(ENET_1G)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, ENET_1G).then_some(0)
    }
}
#[path = "."]
pub mod enet_qos {
    #[doc = "ENET_QOS"]
    pub const ENET_QOS: *const RegisterBlock = 0x4043_c000 as *const RegisterBlock;
    #[path = "blocks/imxrt1176_cm4/enet_qos.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type ENET_QOS = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for ENET_QOS {}
    impl crate::Valid for ENET_QOS {}
    impl ENET_QOS {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(ENET_QOS)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, ENET_QOS).then_some(0)
    }
}
#[path = "."]
pub mod ewm {
    #[doc = "EWM"]
    pub const EWM: *const RegisterBlock = 0x4002_c000 as *const RegisterBlock;
    #[path = "blocks/imxrt1176_cm4/ewm.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type EWM = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for EWM {}
    impl crate::Valid for EWM {}
    impl EWM {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(EWM)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, EWM).then_some(0)
    }
}
#[path = "."]
pub mod flexio {
    #[doc = "FLEXIO"]
    pub const FLEXIO1: *const RegisterBlock = 0x400a_c000 as *const RegisterBlock;
    #[doc = "FLEXIO"]
    pub const FLEXIO2: *const RegisterBlock = 0x400b_0000 as *const RegisterBlock;
    #[path = "blocks/imxrt1176_cm4/flexio.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type FLEXIO1 = Instance<1>;
    impl crate::private::Sealed for FLEXIO1 {}
    impl crate::Valid for FLEXIO1 {}
    impl FLEXIO1 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(FLEXIO1)
        }
    }
    pub type FLEXIO2 = Instance<2>;
    impl crate::private::Sealed for FLEXIO2 {}
    impl crate::Valid for FLEXIO2 {}
    impl FLEXIO2 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(FLEXIO2)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        [(FLEXIO1, 1), (FLEXIO2, 2)]
            .into_iter()
            .find(|(ptr, _)| core::ptr::eq(rb, *ptr))
            .map(|(_, inst)| inst)
    }
}
#[path = "."]
pub mod flexram {
    #[doc = "FLEXRAM"]
    pub const FLEXRAM: *const RegisterBlock = 0x4002_8000 as *const RegisterBlock;
    #[path = "blocks/imxrt1176_cm4/flexram.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type FLEXRAM = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for FLEXRAM {}
    impl crate::Valid for FLEXRAM {}
    impl FLEXRAM {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(FLEXRAM)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, FLEXRAM).then_some(0)
    }
}
#[path = "."]
pub mod flexspi {
    #[doc = "FlexSPI"]
    pub const FLEXSPI1: *const RegisterBlock = 0x400c_c000 as *const RegisterBlock;
    #[doc = "FlexSPI"]
    pub const FLEXSPI2: *const RegisterBlock = 0x400d_0000 as *const RegisterBlock;
    #[path = "blocks/imxrt1176_cm4/flexspi.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type FLEXSPI1 = Instance<1>;
    impl crate::private::Sealed for FLEXSPI1 {}
    impl crate::Valid for FLEXSPI1 {}
    impl FLEXSPI1 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(FLEXSPI1)
        }
    }
    pub type FLEXSPI2 = Instance<2>;
    impl crate::private::Sealed for FLEXSPI2 {}
    impl crate::Valid for FLEXSPI2 {}
    impl FLEXSPI2 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(FLEXSPI2)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        [(FLEXSPI1, 1), (FLEXSPI2, 2)]
            .into_iter()
            .find(|(ptr, _)| core::ptr::eq(rb, *ptr))
            .map(|(_, inst)| inst)
    }
}
#[path = "."]
pub mod gpc_cpu_mode_ctrl_ {
    #[doc = "GPC_CPU"]
    pub const GPC_CPU_MODE_CTRL_0: *const RegisterBlock = 0x40c0_0000 as *const RegisterBlock;
    #[doc = "GPC_CPU"]
    pub const GPC_CPU_MODE_CTRL_1: *const RegisterBlock = 0x40c0_0800 as *const RegisterBlock;
    #[path = "blocks/imxrt1176_cm4/gpc_cpu_mode_ctrl_.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type GPC_CPU_MODE_CTRL_0 = Instance<0>;
    impl crate::private::Sealed for GPC_CPU_MODE_CTRL_0 {}
    impl crate::Valid for GPC_CPU_MODE_CTRL_0 {}
    impl GPC_CPU_MODE_CTRL_0 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(GPC_CPU_MODE_CTRL_0)
        }
    }
    pub type GPC_CPU_MODE_CTRL_1 = Instance<1>;
    impl crate::private::Sealed for GPC_CPU_MODE_CTRL_1 {}
    impl crate::Valid for GPC_CPU_MODE_CTRL_1 {}
    impl GPC_CPU_MODE_CTRL_1 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(GPC_CPU_MODE_CTRL_1)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        [(GPC_CPU_MODE_CTRL_0, 0), (GPC_CPU_MODE_CTRL_1, 1)]
            .into_iter()
            .find(|(ptr, _)| core::ptr::eq(rb, *ptr))
            .map(|(_, inst)| inst)
    }
}
#[path = "."]
pub mod gpc_set_point_ctrl {
    #[doc = "GPC_SP"]
    pub const GPC_SET_POINT_CTRL: *const RegisterBlock = 0x40c0_2000 as *const RegisterBlock;
    #[path = "blocks/imxrt1176_cm4/gpc_set_point_ctrl.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type GPC_SET_POINT_CTRL = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for GPC_SET_POINT_CTRL {}
    impl crate::Valid for GPC_SET_POINT_CTRL {}
    impl GPC_SET_POINT_CTRL {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(GPC_SET_POINT_CTRL)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, GPC_SET_POINT_CTRL).then_some(0)
    }
}
#[path = "."]
pub mod gpc_stby_ctrl {
    #[doc = "GPC_STBY"]
    pub const GPC_STBY_CTRL: *const RegisterBlock = 0x40c0_2800 as *const RegisterBlock;
    #[path = "blocks/imxrt1176_cm4/gpc_stby_ctrl.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type GPC_STBY_CTRL = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for GPC_STBY_CTRL {}
    impl crate::Valid for GPC_STBY_CTRL {}
    impl GPC_STBY_CTRL {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(GPC_STBY_CTRL)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, GPC_STBY_CTRL).then_some(0)
    }
}
#[path = "."]
pub mod gpio {
    #[doc = "GPIO"]
    pub const GPIO2: *const RegisterBlock = 0x4013_0000 as *const RegisterBlock;
    #[doc = "GPIO"]
    pub const GPIO1: *const RegisterBlock = 0x4012_c000 as *const RegisterBlock;
    #[doc = "GPIO"]
    pub const GPIO3: *const RegisterBlock = 0x4013_4000 as *const RegisterBlock;
    #[doc = "GPIO"]
    pub const GPIO4: *const RegisterBlock = 0x4013_8000 as *const RegisterBlock;
    #[doc = "GPIO"]
    pub const GPIO5: *const RegisterBlock = 0x4013_c000 as *const RegisterBlock;
    #[doc = "GPIO"]
    pub const GPIO6: *const RegisterBlock = 0x4014_0000 as *const RegisterBlock;
    #[doc = "GPIO"]
    pub const GPIO7: *const RegisterBlock = 0x40c5_c000 as *const RegisterBlock;
    #[doc = "GPIO"]
    pub const GPIO8: *const RegisterBlock = 0x40c6_0000 as *const RegisterBlock;
    #[doc = "GPIO"]
    pub const GPIO9: *const RegisterBlock = 0x40c6_4000 as *const RegisterBlock;
    #[doc = "GPIO"]
    pub const GPIO10: *const RegisterBlock = 0x40c6_8000 as *const RegisterBlock;
    #[doc = "GPIO"]
    pub const GPIO11: *const RegisterBlock = 0x40c6_c000 as *const RegisterBlock;
    #[doc = "GPIO"]
    pub const GPIO12: *const RegisterBlock = 0x40c7_0000 as *const RegisterBlock;
    #[doc = "GPIO"]
    pub const GPIO13: *const RegisterBlock = 0x40ca_0000 as *const RegisterBlock;
    #[path = "blocks/imxrt1011/gpio.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type GPIO2 = Instance<2>;
    impl crate::private::Sealed for GPIO2 {}
    impl crate::Valid for GPIO2 {}
    impl GPIO2 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(GPIO2)
        }
    }
    pub type GPIO1 = Instance<1>;
    impl crate::private::Sealed for GPIO1 {}
    impl crate::Valid for GPIO1 {}
    impl GPIO1 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(GPIO1)
        }
    }
    pub type GPIO3 = Instance<3>;
    impl crate::private::Sealed for GPIO3 {}
    impl crate::Valid for GPIO3 {}
    impl GPIO3 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(GPIO3)
        }
    }
    pub type GPIO4 = Instance<4>;
    impl crate::private::Sealed for GPIO4 {}
    impl crate::Valid for GPIO4 {}
    impl GPIO4 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(GPIO4)
        }
    }
    pub type GPIO5 = Instance<5>;
    impl crate::private::Sealed for GPIO5 {}
    impl crate::Valid for GPIO5 {}
    impl GPIO5 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(GPIO5)
        }
    }
    pub type GPIO6 = Instance<6>;
    impl crate::private::Sealed for GPIO6 {}
    impl crate::Valid for GPIO6 {}
    impl GPIO6 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(GPIO6)
        }
    }
    pub type GPIO7 = Instance<7>;
    impl crate::private::Sealed for GPIO7 {}
    impl crate::Valid for GPIO7 {}
    impl GPIO7 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(GPIO7)
        }
    }
    pub type GPIO8 = Instance<8>;
    impl crate::private::Sealed for GPIO8 {}
    impl crate::Valid for GPIO8 {}
    impl GPIO8 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(GPIO8)
        }
    }
    pub type GPIO9 = Instance<9>;
    impl crate::private::Sealed for GPIO9 {}
    impl crate::Valid for GPIO9 {}
    impl GPIO9 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(GPIO9)
        }
    }
    pub type GPIO10 = Instance<10>;
    impl crate::private::Sealed for GPIO10 {}
    impl crate::Valid for GPIO10 {}
    impl GPIO10 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(GPIO10)
        }
    }
    pub type GPIO11 = Instance<11>;
    impl crate::private::Sealed for GPIO11 {}
    impl crate::Valid for GPIO11 {}
    impl GPIO11 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(GPIO11)
        }
    }
    pub type GPIO12 = Instance<12>;
    impl crate::private::Sealed for GPIO12 {}
    impl crate::Valid for GPIO12 {}
    impl GPIO12 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(GPIO12)
        }
    }
    pub type GPIO13 = Instance<13>;
    impl crate::private::Sealed for GPIO13 {}
    impl crate::Valid for GPIO13 {}
    impl GPIO13 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(GPIO13)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        [
            (GPIO2, 2),
            (GPIO1, 1),
            (GPIO3, 3),
            (GPIO4, 4),
            (GPIO5, 5),
            (GPIO6, 6),
            (GPIO7, 7),
            (GPIO8, 8),
            (GPIO9, 9),
            (GPIO10, 10),
            (GPIO11, 11),
            (GPIO12, 12),
            (GPIO13, 13),
        ]
        .into_iter()
        .find(|(ptr, _)| core::ptr::eq(rb, *ptr))
        .map(|(_, inst)| inst)
    }
}
#[path = "."]
pub mod gpt {
    #[doc = "GPT"]
    pub const GPT1: *const RegisterBlock = 0x400e_c000 as *const RegisterBlock;
    #[doc = "GPT"]
    pub const GPT2: *const RegisterBlock = 0x400f_0000 as *const RegisterBlock;
    #[doc = "GPT"]
    pub const GPT3: *const RegisterBlock = 0x400f_4000 as *const RegisterBlock;
    #[doc = "GPT"]
    pub const GPT4: *const RegisterBlock = 0x400f_8000 as *const RegisterBlock;
    #[doc = "GPT"]
    pub const GPT5: *const RegisterBlock = 0x400f_c000 as *const RegisterBlock;
    #[doc = "GPT"]
    pub const GPT6: *const RegisterBlock = 0x4010_0000 as *const RegisterBlock;
    #[path = "blocks/imxrt1011/gpt.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type GPT1 = Instance<1>;
    impl crate::private::Sealed for GPT1 {}
    impl crate::Valid for GPT1 {}
    impl GPT1 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(GPT1)
        }
    }
    pub type GPT2 = Instance<2>;
    impl crate::private::Sealed for GPT2 {}
    impl crate::Valid for GPT2 {}
    impl GPT2 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(GPT2)
        }
    }
    pub type GPT3 = Instance<3>;
    impl crate::private::Sealed for GPT3 {}
    impl crate::Valid for GPT3 {}
    impl GPT3 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(GPT3)
        }
    }
    pub type GPT4 = Instance<4>;
    impl crate::private::Sealed for GPT4 {}
    impl crate::Valid for GPT4 {}
    impl GPT4 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(GPT4)
        }
    }
    pub type GPT5 = Instance<5>;
    impl crate::private::Sealed for GPT5 {}
    impl crate::Valid for GPT5 {}
    impl GPT5 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(GPT5)
        }
    }
    pub type GPT6 = Instance<6>;
    impl crate::private::Sealed for GPT6 {}
    impl crate::Valid for GPT6 {}
    impl GPT6 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(GPT6)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        [
            (GPT1, 1),
            (GPT2, 2),
            (GPT3, 3),
            (GPT4, 4),
            (GPT5, 5),
            (GPT6, 6),
        ]
        .into_iter()
        .find(|(ptr, _)| core::ptr::eq(rb, *ptr))
        .map(|(_, inst)| inst)
    }
}
#[path = "."]
pub mod iee__iee_rt1170 {
    #[doc = "IEE"]
    pub const IEE__IEE_RT1170: *const RegisterBlock = 0x4006_c000 as *const RegisterBlock;
    #[path = "blocks/imxrt1176_cm4/iee__iee_rt1170.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type IEE__IEE_RT1170 = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for IEE__IEE_RT1170 {}
    impl crate::Valid for IEE__IEE_RT1170 {}
    impl IEE__IEE_RT1170 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(IEE__IEE_RT1170)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, IEE__IEE_RT1170).then_some(0)
    }
}
#[path = "."]
pub mod iee_apc {
    #[doc = "IEE_APC"]
    pub const IEE_APC: *const RegisterBlock = 0x4006_8000 as *const RegisterBlock;
    #[path = "blocks/imxrt1176_cm4/iee_apc.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type IEE_APC = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for IEE_APC {}
    impl crate::Valid for IEE_APC {}
    impl IEE_APC {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(IEE_APC)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, IEE_APC).then_some(0)
    }
}
#[path = "."]
pub mod iomuxc {
    #[doc = "IOMUXC"]
    pub const IOMUXC: *const RegisterBlock = 0x400e_8000 as *const RegisterBlock;
    #[path = "blocks/imxrt1176_cm4/iomuxc.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type IOMUXC = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for IOMUXC {}
    impl crate::Valid for IOMUXC {}
    impl IOMUXC {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(IOMUXC)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, IOMUXC).then_some(0)
    }
}
#[path = "."]
pub mod iomuxc_gpr {
    #[doc = "IOMUXC GPR"]
    pub const IOMUXC_GPR: *const RegisterBlock = 0x400e_4000 as *const RegisterBlock;
    #[path = "blocks/imxrt1176_cm4/iomuxc_gpr.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type IOMUXC_GPR = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for IOMUXC_GPR {}
    impl crate::Valid for IOMUXC_GPR {}
    impl IOMUXC_GPR {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(IOMUXC_GPR)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, IOMUXC_GPR).then_some(0)
    }
}
#[path = "."]
pub mod iomuxc_lpsr {
    #[doc = "IOMUXC LPSR"]
    pub const IOMUXC_LPSR: *const RegisterBlock = 0x40c0_8000 as *const RegisterBlock;
    #[path = "blocks/imxrt1176_cm4/iomuxc_lpsr.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type IOMUXC_LPSR = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for IOMUXC_LPSR {}
    impl crate::Valid for IOMUXC_LPSR {}
    impl IOMUXC_LPSR {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(IOMUXC_LPSR)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, IOMUXC_LPSR).then_some(0)
    }
}
#[path = "."]
pub mod iomuxc_lpsr_gpr {
    #[doc = "IOMUXC LPSR GPR"]
    pub const IOMUXC_LPSR_GPR: *const RegisterBlock = 0x40c0_c000 as *const RegisterBlock;
    #[path = "blocks/imxrt1176_cm4/iomuxc_lpsr_gpr.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type IOMUXC_LPSR_GPR = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for IOMUXC_LPSR_GPR {}
    impl crate::Valid for IOMUXC_LPSR_GPR {}
    impl IOMUXC_LPSR_GPR {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(IOMUXC_LPSR_GPR)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, IOMUXC_LPSR_GPR).then_some(0)
    }
}
#[path = "."]
pub mod iomuxc_snvs {
    #[doc = "IOMUXC SNVS"]
    pub const IOMUXC_SNVS: *const RegisterBlock = 0x40c9_4000 as *const RegisterBlock;
    #[path = "blocks/imxrt1176_cm4/iomuxc_snvs.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type IOMUXC_SNVS = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for IOMUXC_SNVS {}
    impl crate::Valid for IOMUXC_SNVS {}
    impl IOMUXC_SNVS {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(IOMUXC_SNVS)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, IOMUXC_SNVS).then_some(0)
    }
}
#[path = "."]
pub mod iomuxc_snvs_gpr {
    #[doc = "IOMUXC SNVS GPR"]
    pub const IOMUXC_SNVS_GPR: *const RegisterBlock = 0x40c9_8000 as *const RegisterBlock;
    #[path = "blocks/imxrt1176_cm4/iomuxc_snvs_gpr.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type IOMUXC_SNVS_GPR = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for IOMUXC_SNVS_GPR {}
    impl crate::Valid for IOMUXC_SNVS_GPR {}
    impl IOMUXC_SNVS_GPR {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(IOMUXC_SNVS_GPR)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, IOMUXC_SNVS_GPR).then_some(0)
    }
}
#[path = "."]
pub mod ips_domain {
    #[doc = "IPS Domain"]
    pub const IPS_DOMAIN: *const RegisterBlock = 0x40c8_7c00 as *const RegisterBlock;
    #[path = "blocks/imxrt1176_cm4/ips_domain.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type IPS_DOMAIN = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for IPS_DOMAIN {}
    impl crate::Valid for IPS_DOMAIN {}
    impl IPS_DOMAIN {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(IPS_DOMAIN)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, IPS_DOMAIN).then_some(0)
    }
}
#[path = "."]
pub mod key_manager {
    #[doc = "KEYMGR"]
    pub const KEY_MANAGER: *const RegisterBlock = 0x40c8_0000 as *const RegisterBlock;
    #[path = "blocks/imxrt1176_cm4/key_manager.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type KEY_MANAGER = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for KEY_MANAGER {}
    impl crate::Valid for KEY_MANAGER {}
    impl KEY_MANAGER {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(KEY_MANAGER)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, KEY_MANAGER).then_some(0)
    }
}
#[path = "."]
pub mod key_manager__puf {
    #[doc = "PUF"]
    pub const KEY_MANAGER__PUF: *const RegisterBlock = 0x40c8_2000 as *const RegisterBlock;
    #[path = "blocks/imxrt1176_cm4/key_manager__puf.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type KEY_MANAGER__PUF = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for KEY_MANAGER__PUF {}
    impl crate::Valid for KEY_MANAGER__PUF {}
    impl KEY_MANAGER__PUF {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(KEY_MANAGER__PUF)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, KEY_MANAGER__PUF).then_some(0)
    }
}
#[path = "."]
pub mod kpp {
    #[doc = "KPP"]
    pub const KPP: *const RegisterBlock = 0x400e_0000 as *const RegisterBlock;
    #[path = "blocks/imxrt1011/kpp.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type KPP = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for KPP {}
    impl crate::Valid for KPP {}
    impl KPP {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(KPP)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, KPP).then_some(0)
    }
}
#[path = "."]
pub mod lcdif {
    #[doc = "LCDIF Register Reference Index"]
    pub const LCDIF: *const RegisterBlock = 0x4080_4000 as *const RegisterBlock;
    #[path = "blocks/imxrt1176_cm4/lcdif.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type LCDIF = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for LCDIF {}
    impl crate::Valid for LCDIF {}
    impl LCDIF {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(LCDIF)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, LCDIF).then_some(0)
    }
}
#[path = "."]
pub mod lcdifv2 {
    #[doc = "LCDIF_V2"]
    pub const LCDIFV2: *const RegisterBlock = 0x4080_8000 as *const RegisterBlock;
    #[path = "blocks/imxrt1176_cm4/lcdifv2.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type LCDIFV2 = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for LCDIFV2 {}
    impl crate::Valid for LCDIFV2 {}
    impl LCDIFV2 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(LCDIFV2)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, LCDIFV2).then_some(0)
    }
}
#[path = "."]
pub mod lmem {
    #[doc = "LMEM"]
    pub const LMEM: *const RegisterBlock = 0xe008_2000 as *const RegisterBlock;
    #[path = "blocks/imxrt1176_cm4/lmem.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type LMEM = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for LMEM {}
    impl crate::Valid for LMEM {}
    impl LMEM {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(LMEM)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, LMEM).then_some(0)
    }
}
#[path = "."]
pub mod lpadc {
    #[doc = "LPADC"]
    pub const LPADC1: *const RegisterBlock = 0x4005_0000 as *const RegisterBlock;
    #[doc = "LPADC"]
    pub const LPADC2: *const RegisterBlock = 0x4005_4000 as *const RegisterBlock;
    #[path = "blocks/imxrt1176_cm4/lpadc.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type LPADC1 = Instance<1>;
    impl crate::private::Sealed for LPADC1 {}
    impl crate::Valid for LPADC1 {}
    impl LPADC1 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(LPADC1)
        }
    }
    pub type LPADC2 = Instance<2>;
    impl crate::private::Sealed for LPADC2 {}
    impl crate::Valid for LPADC2 {}
    impl LPADC2 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(LPADC2)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        [(LPADC1, 1), (LPADC2, 2)]
            .into_iter()
            .find(|(ptr, _)| core::ptr::eq(rb, *ptr))
            .map(|(_, inst)| inst)
    }
}
#[path = "."]
pub mod lpi2c {
    #[doc = "LPI2C"]
    pub const LPI2C1: *const RegisterBlock = 0x4010_4000 as *const RegisterBlock;
    #[doc = "LPI2C"]
    pub const LPI2C2: *const RegisterBlock = 0x4010_8000 as *const RegisterBlock;
    #[doc = "LPI2C"]
    pub const LPI2C3: *const RegisterBlock = 0x4010_c000 as *const RegisterBlock;
    #[doc = "LPI2C"]
    pub const LPI2C4: *const RegisterBlock = 0x4011_0000 as *const RegisterBlock;
    #[doc = "LPI2C"]
    pub const LPI2C5: *const RegisterBlock = 0x40c3_4000 as *const RegisterBlock;
    #[doc = "LPI2C"]
    pub const LPI2C6: *const RegisterBlock = 0x40c3_8000 as *const RegisterBlock;
    #[path = "blocks/imxrt1011/lpi2c.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type LPI2C1 = Instance<1>;
    impl crate::private::Sealed for LPI2C1 {}
    impl crate::Valid for LPI2C1 {}
    impl LPI2C1 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(LPI2C1)
        }
    }
    pub type LPI2C2 = Instance<2>;
    impl crate::private::Sealed for LPI2C2 {}
    impl crate::Valid for LPI2C2 {}
    impl LPI2C2 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(LPI2C2)
        }
    }
    pub type LPI2C3 = Instance<3>;
    impl crate::private::Sealed for LPI2C3 {}
    impl crate::Valid for LPI2C3 {}
    impl LPI2C3 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(LPI2C3)
        }
    }
    pub type LPI2C4 = Instance<4>;
    impl crate::private::Sealed for LPI2C4 {}
    impl crate::Valid for LPI2C4 {}
    impl LPI2C4 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(LPI2C4)
        }
    }
    pub type LPI2C5 = Instance<5>;
    impl crate::private::Sealed for LPI2C5 {}
    impl crate::Valid for LPI2C5 {}
    impl LPI2C5 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(LPI2C5)
        }
    }
    pub type LPI2C6 = Instance<6>;
    impl crate::private::Sealed for LPI2C6 {}
    impl crate::Valid for LPI2C6 {}
    impl LPI2C6 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(LPI2C6)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        [
            (LPI2C1, 1),
            (LPI2C2, 2),
            (LPI2C3, 3),
            (LPI2C4, 4),
            (LPI2C5, 5),
            (LPI2C6, 6),
        ]
        .into_iter()
        .find(|(ptr, _)| core::ptr::eq(rb, *ptr))
        .map(|(_, inst)| inst)
    }
}
#[path = "."]
pub mod lpspi {
    #[doc = "LPSPI"]
    pub const LPSPI1: *const RegisterBlock = 0x4011_4000 as *const RegisterBlock;
    #[doc = "LPSPI"]
    pub const LPSPI2: *const RegisterBlock = 0x4011_8000 as *const RegisterBlock;
    #[doc = "LPSPI"]
    pub const LPSPI3: *const RegisterBlock = 0x4011_c000 as *const RegisterBlock;
    #[doc = "LPSPI"]
    pub const LPSPI4: *const RegisterBlock = 0x4012_0000 as *const RegisterBlock;
    #[doc = "LPSPI"]
    pub const LPSPI5: *const RegisterBlock = 0x40c2_c000 as *const RegisterBlock;
    #[doc = "LPSPI"]
    pub const LPSPI6: *const RegisterBlock = 0x40c3_0000 as *const RegisterBlock;
    #[path = "blocks/imxrt1011/lpspi.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type LPSPI1 = Instance<1>;
    impl crate::private::Sealed for LPSPI1 {}
    impl crate::Valid for LPSPI1 {}
    impl LPSPI1 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(LPSPI1)
        }
    }
    pub type LPSPI2 = Instance<2>;
    impl crate::private::Sealed for LPSPI2 {}
    impl crate::Valid for LPSPI2 {}
    impl LPSPI2 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(LPSPI2)
        }
    }
    pub type LPSPI3 = Instance<3>;
    impl crate::private::Sealed for LPSPI3 {}
    impl crate::Valid for LPSPI3 {}
    impl LPSPI3 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(LPSPI3)
        }
    }
    pub type LPSPI4 = Instance<4>;
    impl crate::private::Sealed for LPSPI4 {}
    impl crate::Valid for LPSPI4 {}
    impl LPSPI4 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(LPSPI4)
        }
    }
    pub type LPSPI5 = Instance<5>;
    impl crate::private::Sealed for LPSPI5 {}
    impl crate::Valid for LPSPI5 {}
    impl LPSPI5 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(LPSPI5)
        }
    }
    pub type LPSPI6 = Instance<6>;
    impl crate::private::Sealed for LPSPI6 {}
    impl crate::Valid for LPSPI6 {}
    impl LPSPI6 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(LPSPI6)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        [
            (LPSPI1, 1),
            (LPSPI2, 2),
            (LPSPI3, 3),
            (LPSPI4, 4),
            (LPSPI5, 5),
            (LPSPI6, 6),
        ]
        .into_iter()
        .find(|(ptr, _)| core::ptr::eq(rb, *ptr))
        .map(|(_, inst)| inst)
    }
}
#[path = "."]
pub mod lpuart {
    #[doc = "LPUART"]
    pub const LPUART1: *const RegisterBlock = 0x4007_c000 as *const RegisterBlock;
    #[doc = "LPUART"]
    pub const LPUART2: *const RegisterBlock = 0x4008_0000 as *const RegisterBlock;
    #[doc = "LPUART"]
    pub const LPUART3: *const RegisterBlock = 0x4008_4000 as *const RegisterBlock;
    #[doc = "LPUART"]
    pub const LPUART4: *const RegisterBlock = 0x4008_8000 as *const RegisterBlock;
    #[doc = "LPUART"]
    pub const LPUART5: *const RegisterBlock = 0x4008_c000 as *const RegisterBlock;
    #[doc = "LPUART"]
    pub const LPUART6: *const RegisterBlock = 0x4009_0000 as *const RegisterBlock;
    #[doc = "LPUART"]
    pub const LPUART7: *const RegisterBlock = 0x4009_4000 as *const RegisterBlock;
    #[doc = "LPUART"]
    pub const LPUART8: *const RegisterBlock = 0x4009_8000 as *const RegisterBlock;
    #[doc = "LPUART"]
    pub const LPUART9: *const RegisterBlock = 0x4009_c000 as *const RegisterBlock;
    #[doc = "LPUART"]
    pub const LPUART10: *const RegisterBlock = 0x400a_0000 as *const RegisterBlock;
    #[doc = "LPUART"]
    pub const LPUART11: *const RegisterBlock = 0x40c2_4000 as *const RegisterBlock;
    #[doc = "LPUART"]
    pub const LPUART12: *const RegisterBlock = 0x40c2_8000 as *const RegisterBlock;
    #[path = "blocks/imxrt1011/lpuart.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type LPUART1 = Instance<1>;
    impl crate::private::Sealed for LPUART1 {}
    impl crate::Valid for LPUART1 {}
    impl LPUART1 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(LPUART1)
        }
    }
    pub type LPUART2 = Instance<2>;
    impl crate::private::Sealed for LPUART2 {}
    impl crate::Valid for LPUART2 {}
    impl LPUART2 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(LPUART2)
        }
    }
    pub type LPUART3 = Instance<3>;
    impl crate::private::Sealed for LPUART3 {}
    impl crate::Valid for LPUART3 {}
    impl LPUART3 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(LPUART3)
        }
    }
    pub type LPUART4 = Instance<4>;
    impl crate::private::Sealed for LPUART4 {}
    impl crate::Valid for LPUART4 {}
    impl LPUART4 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(LPUART4)
        }
    }
    pub type LPUART5 = Instance<5>;
    impl crate::private::Sealed for LPUART5 {}
    impl crate::Valid for LPUART5 {}
    impl LPUART5 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(LPUART5)
        }
    }
    pub type LPUART6 = Instance<6>;
    impl crate::private::Sealed for LPUART6 {}
    impl crate::Valid for LPUART6 {}
    impl LPUART6 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(LPUART6)
        }
    }
    pub type LPUART7 = Instance<7>;
    impl crate::private::Sealed for LPUART7 {}
    impl crate::Valid for LPUART7 {}
    impl LPUART7 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(LPUART7)
        }
    }
    pub type LPUART8 = Instance<8>;
    impl crate::private::Sealed for LPUART8 {}
    impl crate::Valid for LPUART8 {}
    impl LPUART8 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(LPUART8)
        }
    }
    pub type LPUART9 = Instance<9>;
    impl crate::private::Sealed for LPUART9 {}
    impl crate::Valid for LPUART9 {}
    impl LPUART9 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(LPUART9)
        }
    }
    pub type LPUART10 = Instance<10>;
    impl crate::private::Sealed for LPUART10 {}
    impl crate::Valid for LPUART10 {}
    impl LPUART10 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(LPUART10)
        }
    }
    pub type LPUART11 = Instance<11>;
    impl crate::private::Sealed for LPUART11 {}
    impl crate::Valid for LPUART11 {}
    impl LPUART11 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(LPUART11)
        }
    }
    pub type LPUART12 = Instance<12>;
    impl crate::private::Sealed for LPUART12 {}
    impl crate::Valid for LPUART12 {}
    impl LPUART12 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(LPUART12)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        [
            (LPUART1, 1),
            (LPUART2, 2),
            (LPUART3, 3),
            (LPUART4, 4),
            (LPUART5, 5),
            (LPUART6, 6),
            (LPUART7, 7),
            (LPUART8, 8),
            (LPUART9, 9),
            (LPUART10, 10),
            (LPUART11, 11),
            (LPUART12, 12),
        ]
        .into_iter()
        .find(|(ptr, _)| core::ptr::eq(rb, *ptr))
        .map(|(_, inst)| inst)
    }
}
#[path = "."]
pub mod mcm {
    #[doc = "Core Platform Miscellaneous Control Module"]
    pub const MCM: *const RegisterBlock = 0xe008_0000 as *const RegisterBlock;
    #[path = "blocks/imxrt1176_cm4/mcm.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type MCM = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for MCM {}
    impl crate::Valid for MCM {}
    impl MCM {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(MCM)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, MCM).then_some(0)
    }
}
#[path = "."]
pub mod mecc {
    #[doc = "MECC64"]
    pub const MECC1: *const RegisterBlock = 0x4001_4000 as *const RegisterBlock;
    #[doc = "MECC64"]
    pub const MECC2: *const RegisterBlock = 0x4001_8000 as *const RegisterBlock;
    #[path = "blocks/imxrt1176_cm4/mecc.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type MECC1 = Instance<1>;
    impl crate::private::Sealed for MECC1 {}
    impl crate::Valid for MECC1 {}
    impl MECC1 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(MECC1)
        }
    }
    pub type MECC2 = Instance<2>;
    impl crate::private::Sealed for MECC2 {}
    impl crate::Valid for MECC2 {}
    impl MECC2 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(MECC2)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        [(MECC1, 1), (MECC2, 2)]
            .into_iter()
            .find(|(ptr, _)| core::ptr::eq(rb, *ptr))
            .map(|(_, inst)| inst)
    }
}
#[path = "."]
pub mod mipi_csi2rx {
    #[doc = "no description available"]
    pub const MIPI_CSI2RX: *const RegisterBlock = 0x4081_0000 as *const RegisterBlock;
    #[path = "blocks/imxrt1176_cm4/mipi_csi2rx.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type MIPI_CSI2RX = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for MIPI_CSI2RX {}
    impl crate::Valid for MIPI_CSI2RX {}
    impl MIPI_CSI2RX {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(MIPI_CSI2RX)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, MIPI_CSI2RX).then_some(0)
    }
}
#[path = "."]
pub mod mmcau {
    #[doc = "CAU"]
    pub const MMCAU: *const RegisterBlock = 0xe008_1000 as *const RegisterBlock;
    #[path = "blocks/imxrt1176_cm4/mmcau.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type MMCAU = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for MMCAU {}
    impl crate::Valid for MMCAU {}
    impl MMCAU {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(MMCAU)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, MMCAU).then_some(0)
    }
}
#[path = "."]
pub mod mub {
    #[doc = "MUB"]
    pub const MUB: *const RegisterBlock = 0x40c4_c000 as *const RegisterBlock;
    #[path = "blocks/imxrt1176_cm4/mub.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type MUB = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for MUB {}
    impl crate::Valid for MUB {}
    impl MUB {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(MUB)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, MUB).then_some(0)
    }
}
#[path = "."]
pub mod ocotp {
    #[doc = "no description available"]
    pub const OCOTP: *const RegisterBlock = 0x40ca_c000 as *const RegisterBlock;
    #[path = "blocks/imxrt1176_cm4/ocotp.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type OCOTP = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for OCOTP {}
    impl crate::Valid for OCOTP {}
    impl OCOTP {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(OCOTP)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, OCOTP).then_some(0)
    }
}
#[path = "."]
pub mod osc_rc_400m {
    #[path = "blocks/imxrt1176_cm4/osc_rc_400m.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
}
#[path = "."]
pub mod otfad {
    #[doc = "OTFAD"]
    pub const OTFAD1: *const RegisterBlock = 0x400c_c000 as *const RegisterBlock;
    #[doc = "OTFAD"]
    pub const OTFAD2: *const RegisterBlock = 0x400d_0000 as *const RegisterBlock;
    #[path = "blocks/imxrt1176_cm4/otfad.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type OTFAD1 = Instance<1>;
    impl crate::private::Sealed for OTFAD1 {}
    impl crate::Valid for OTFAD1 {}
    impl OTFAD1 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(OTFAD1)
        }
    }
    pub type OTFAD2 = Instance<2>;
    impl crate::private::Sealed for OTFAD2 {}
    impl crate::Valid for OTFAD2 {}
    impl OTFAD2 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(OTFAD2)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        [(OTFAD1, 1), (OTFAD2, 2)]
            .into_iter()
            .find(|(ptr, _)| core::ptr::eq(rb, *ptr))
            .map(|(_, inst)| inst)
    }
}
#[path = "."]
pub mod pdm {
    #[doc = "PDM"]
    pub const PDM: *const RegisterBlock = 0x40c2_0000 as *const RegisterBlock;
    #[path = "blocks/imxrt1176_cm4/pdm.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type PDM = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for PDM {}
    impl crate::Valid for PDM {}
    impl PDM {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(PDM)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, PDM).then_some(0)
    }
}
#[path = "."]
pub mod pgmc_bpc {
    #[doc = "PGMC_BPC"]
    pub const PGMC_BPC0: *const RegisterBlock = 0x40c8_8000 as *const RegisterBlock;
    #[doc = "PGMC_BPC"]
    pub const PGMC_BPC1: *const RegisterBlock = 0x40c8_8200 as *const RegisterBlock;
    #[doc = "PGMC_BPC"]
    pub const PGMC_BPC2: *const RegisterBlock = 0x40c8_8400 as *const RegisterBlock;
    #[doc = "PGMC_BPC"]
    pub const PGMC_BPC3: *const RegisterBlock = 0x40c8_8600 as *const RegisterBlock;
    #[doc = "PGMC_BPC"]
    pub const PGMC_BPC4: *const RegisterBlock = 0x40c8_8800 as *const RegisterBlock;
    #[doc = "PGMC_BPC"]
    pub const PGMC_BPC5: *const RegisterBlock = 0x40c8_8a00 as *const RegisterBlock;
    #[doc = "PGMC_BPC"]
    pub const PGMC_BPC6: *const RegisterBlock = 0x40c8_8c00 as *const RegisterBlock;
    #[doc = "PGMC_BPC"]
    pub const PGMC_BPC7: *const RegisterBlock = 0x40c8_8e00 as *const RegisterBlock;
    #[path = "blocks/imxrt1176_cm4/pgmc_bpc.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type PGMC_BPC0 = Instance<0>;
    impl crate::private::Sealed for PGMC_BPC0 {}
    impl crate::Valid for PGMC_BPC0 {}
    impl PGMC_BPC0 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(PGMC_BPC0)
        }
    }
    pub type PGMC_BPC1 = Instance<1>;
    impl crate::private::Sealed for PGMC_BPC1 {}
    impl crate::Valid for PGMC_BPC1 {}
    impl PGMC_BPC1 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(PGMC_BPC1)
        }
    }
    pub type PGMC_BPC2 = Instance<2>;
    impl crate::private::Sealed for PGMC_BPC2 {}
    impl crate::Valid for PGMC_BPC2 {}
    impl PGMC_BPC2 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(PGMC_BPC2)
        }
    }
    pub type PGMC_BPC3 = Instance<3>;
    impl crate::private::Sealed for PGMC_BPC3 {}
    impl crate::Valid for PGMC_BPC3 {}
    impl PGMC_BPC3 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(PGMC_BPC3)
        }
    }
    pub type PGMC_BPC4 = Instance<4>;
    impl crate::private::Sealed for PGMC_BPC4 {}
    impl crate::Valid for PGMC_BPC4 {}
    impl PGMC_BPC4 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(PGMC_BPC4)
        }
    }
    pub type PGMC_BPC5 = Instance<5>;
    impl crate::private::Sealed for PGMC_BPC5 {}
    impl crate::Valid for PGMC_BPC5 {}
    impl PGMC_BPC5 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(PGMC_BPC5)
        }
    }
    pub type PGMC_BPC6 = Instance<6>;
    impl crate::private::Sealed for PGMC_BPC6 {}
    impl crate::Valid for PGMC_BPC6 {}
    impl PGMC_BPC6 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(PGMC_BPC6)
        }
    }
    pub type PGMC_BPC7 = Instance<7>;
    impl crate::private::Sealed for PGMC_BPC7 {}
    impl crate::Valid for PGMC_BPC7 {}
    impl PGMC_BPC7 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(PGMC_BPC7)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        [
            (PGMC_BPC0, 0),
            (PGMC_BPC1, 1),
            (PGMC_BPC2, 2),
            (PGMC_BPC3, 3),
            (PGMC_BPC4, 4),
            (PGMC_BPC5, 5),
            (PGMC_BPC6, 6),
            (PGMC_BPC7, 7),
        ]
        .into_iter()
        .find(|(ptr, _)| core::ptr::eq(rb, *ptr))
        .map(|(_, inst)| inst)
    }
}
#[path = "."]
pub mod pgmc_cpc {
    #[doc = "PGMC_CPC"]
    pub const PGMC_CPC0: *const RegisterBlock = 0x40c8_9000 as *const RegisterBlock;
    #[doc = "PGMC_CPC"]
    pub const PGMC_CPC1: *const RegisterBlock = 0x40c8_9400 as *const RegisterBlock;
    #[path = "blocks/imxrt1176_cm4/pgmc_cpc.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type PGMC_CPC0 = Instance<0>;
    impl crate::private::Sealed for PGMC_CPC0 {}
    impl crate::Valid for PGMC_CPC0 {}
    impl PGMC_CPC0 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(PGMC_CPC0)
        }
    }
    pub type PGMC_CPC1 = Instance<1>;
    impl crate::private::Sealed for PGMC_CPC1 {}
    impl crate::Valid for PGMC_CPC1 {}
    impl PGMC_CPC1 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(PGMC_CPC1)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        [(PGMC_CPC0, 0), (PGMC_CPC1, 1)]
            .into_iter()
            .find(|(ptr, _)| core::ptr::eq(rb, *ptr))
            .map(|(_, inst)| inst)
    }
}
#[path = "."]
pub mod pgmc_cpc0_mif {
    #[doc = "PGMC_MIF"]
    pub const PGMC_CPC_MIF00: *const RegisterBlock = 0x40c8_9100 as *const RegisterBlock;
    #[doc = "PGMC_MIF"]
    pub const PGMC_CPC_MIF01: *const RegisterBlock = 0x40c8_9200 as *const RegisterBlock;
    #[doc = "PGMC_MIF"]
    pub const PGMC_CPC_MIF10: *const RegisterBlock = 0x40c8_9500 as *const RegisterBlock;
    #[doc = "PGMC_MIF"]
    pub const PGMC_CPC_MIF11: *const RegisterBlock = 0x40c8_9600 as *const RegisterBlock;
    #[path = "blocks/imxrt1176_cm4/pgmc_cpc0_mif.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type PGMC_CPC_MIF00 = Instance<0>;
    impl crate::private::Sealed for PGMC_CPC_MIF00 {}
    impl crate::Valid for PGMC_CPC_MIF00 {}
    impl PGMC_CPC_MIF00 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(PGMC_CPC_MIF00)
        }
    }
    pub type PGMC_CPC_MIF01 = Instance<1>;
    impl crate::private::Sealed for PGMC_CPC_MIF01 {}
    impl crate::Valid for PGMC_CPC_MIF01 {}
    impl PGMC_CPC_MIF01 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(PGMC_CPC_MIF01)
        }
    }
    pub type PGMC_CPC_MIF10 = Instance<10>;
    impl crate::private::Sealed for PGMC_CPC_MIF10 {}
    impl crate::Valid for PGMC_CPC_MIF10 {}
    impl PGMC_CPC_MIF10 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(PGMC_CPC_MIF10)
        }
    }
    pub type PGMC_CPC_MIF11 = Instance<11>;
    impl crate::private::Sealed for PGMC_CPC_MIF11 {}
    impl crate::Valid for PGMC_CPC_MIF11 {}
    impl PGMC_CPC_MIF11 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(PGMC_CPC_MIF11)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        [
            (PGMC_CPC_MIF00, 0),
            (PGMC_CPC_MIF01, 1),
            (PGMC_CPC_MIF10, 10),
            (PGMC_CPC_MIF11, 11),
        ]
        .into_iter()
        .find(|(ptr, _)| core::ptr::eq(rb, *ptr))
        .map(|(_, inst)| inst)
    }
}
#[path = "."]
pub mod pgmc_ppc0 {
    #[doc = "PGMC_PPC"]
    pub const PGMC_PPC0: *const RegisterBlock = 0x40c8_b000 as *const RegisterBlock;
    #[path = "blocks/imxrt1176_cm4/pgmc_ppc0.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type PGMC_PPC0 = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for PGMC_PPC0 {}
    impl crate::Valid for PGMC_PPC0 {}
    impl PGMC_PPC0 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(PGMC_PPC0)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, PGMC_PPC0).then_some(0)
    }
}
#[path = "."]
pub mod phy_ldo {
    #[path = "blocks/imxrt1176_cm4/phy_ldo.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
}
#[path = "."]
pub mod pit {
    #[doc = "PIT"]
    pub const PIT1: *const RegisterBlock = 0x400d_8000 as *const RegisterBlock;
    #[doc = "PIT"]
    pub const PIT2: *const RegisterBlock = 0x40cb_0000 as *const RegisterBlock;
    #[path = "blocks/imxrt1011/pit.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type PIT1 = Instance<1>;
    impl crate::private::Sealed for PIT1 {}
    impl crate::Valid for PIT1 {}
    impl PIT1 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(PIT1)
        }
    }
    pub type PIT2 = Instance<2>;
    impl crate::private::Sealed for PIT2 {}
    impl crate::Valid for PIT2 {}
    impl PIT2 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(PIT2)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        [(PIT1, 1), (PIT2, 2)]
            .into_iter()
            .find(|(ptr, _)| core::ptr::eq(rb, *ptr))
            .map(|(_, inst)| inst)
    }
}
#[path = "."]
pub mod pwm {
    #[doc = "PWM"]
    pub const PWM1: *const RegisterBlock = 0x4018_c000 as *const RegisterBlock;
    #[doc = "PWM"]
    pub const PWM2: *const RegisterBlock = 0x4019_0000 as *const RegisterBlock;
    #[doc = "PWM"]
    pub const PWM3: *const RegisterBlock = 0x4019_4000 as *const RegisterBlock;
    #[doc = "PWM"]
    pub const PWM4: *const RegisterBlock = 0x4019_8000 as *const RegisterBlock;
    #[path = "blocks/imxrt1176_cm4/pwm.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type PWM1 = Instance<1>;
    impl crate::private::Sealed for PWM1 {}
    impl crate::Valid for PWM1 {}
    impl PWM1 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(PWM1)
        }
    }
    pub type PWM2 = Instance<2>;
    impl crate::private::Sealed for PWM2 {}
    impl crate::Valid for PWM2 {}
    impl PWM2 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(PWM2)
        }
    }
    pub type PWM3 = Instance<3>;
    impl crate::private::Sealed for PWM3 {}
    impl crate::Valid for PWM3 {}
    impl PWM3 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(PWM3)
        }
    }
    pub type PWM4 = Instance<4>;
    impl crate::private::Sealed for PWM4 {}
    impl crate::Valid for PWM4 {}
    impl PWM4 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(PWM4)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        [(PWM1, 1), (PWM2, 2), (PWM3, 3), (PWM4, 4)]
            .into_iter()
            .find(|(ptr, _)| core::ptr::eq(rb, *ptr))
            .map(|(_, inst)| inst)
    }
}
#[path = "."]
pub mod pxp {
    #[doc = "PXP v2.0 Register Reference Index"]
    pub const PXP: *const RegisterBlock = 0x4081_4000 as *const RegisterBlock;
    #[path = "blocks/imxrt1176_cm4/pxp.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type PXP = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for PXP {}
    impl crate::Valid for PXP {}
    impl PXP {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(PXP)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, PXP).then_some(0)
    }
}
#[path = "."]
pub mod rdc {
    #[doc = "RDC"]
    pub const RDC: *const RegisterBlock = 0x40c7_8000 as *const RegisterBlock;
    #[path = "blocks/imxrt1176_cm4/rdc.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type RDC = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for RDC {}
    impl crate::Valid for RDC {}
    impl RDC {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(RDC)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, RDC).then_some(0)
    }
}
#[path = "."]
pub mod rdc_semaphore {
    #[doc = "SEMA42"]
    pub const RDC_SEMAPHORE1: *const RegisterBlock = 0x40c4_4000 as *const RegisterBlock;
    #[doc = "SEMA42"]
    pub const RDC_SEMAPHORE2: *const RegisterBlock = 0x40cc_c000 as *const RegisterBlock;
    #[path = "blocks/imxrt1176_cm4/rdc_semaphore.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type RDC_SEMAPHORE1 = Instance<1>;
    impl crate::private::Sealed for RDC_SEMAPHORE1 {}
    impl crate::Valid for RDC_SEMAPHORE1 {}
    impl RDC_SEMAPHORE1 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(RDC_SEMAPHORE1)
        }
    }
    pub type RDC_SEMAPHORE2 = Instance<2>;
    impl crate::private::Sealed for RDC_SEMAPHORE2 {}
    impl crate::Valid for RDC_SEMAPHORE2 {}
    impl RDC_SEMAPHORE2 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(RDC_SEMAPHORE2)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        [(RDC_SEMAPHORE1, 1), (RDC_SEMAPHORE2, 2)]
            .into_iter()
            .find(|(ptr, _)| core::ptr::eq(rb, *ptr))
            .map(|(_, inst)| inst)
    }
}
#[path = "."]
pub mod rtwdog {
    #[doc = "WDOG"]
    pub const RTWDOG3: *const RegisterBlock = 0x4003_8000 as *const RegisterBlock;
    #[doc = "WDOG"]
    pub const RTWDOG4: *const RegisterBlock = 0x40c1_0000 as *const RegisterBlock;
    #[path = "blocks/imxrt1176_cm4/rtwdog.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type RTWDOG3 = Instance<3>;
    impl crate::private::Sealed for RTWDOG3 {}
    impl crate::Valid for RTWDOG3 {}
    impl RTWDOG3 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(RTWDOG3)
        }
    }
    pub type RTWDOG4 = Instance<4>;
    impl crate::private::Sealed for RTWDOG4 {}
    impl crate::Valid for RTWDOG4 {}
    impl RTWDOG4 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(RTWDOG4)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        [(RTWDOG3, 3), (RTWDOG4, 4)]
            .into_iter()
            .find(|(ptr, _)| core::ptr::eq(rb, *ptr))
            .map(|(_, inst)| inst)
    }
}
#[path = "."]
pub mod sai {
    #[doc = "SAI"]
    pub const SAI1: *const RegisterBlock = 0x4040_4000 as *const RegisterBlock;
    #[doc = "SAI"]
    pub const SAI2: *const RegisterBlock = 0x4040_8000 as *const RegisterBlock;
    #[doc = "SAI"]
    pub const SAI3: *const RegisterBlock = 0x4040_c000 as *const RegisterBlock;
    #[doc = "SAI"]
    pub const SAI4: *const RegisterBlock = 0x40c4_0000 as *const RegisterBlock;
    #[path = "blocks/imxrt1176_cm4/sai.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type SAI1 = Instance<1>;
    impl crate::private::Sealed for SAI1 {}
    impl crate::Valid for SAI1 {}
    impl SAI1 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(SAI1)
        }
    }
    pub type SAI2 = Instance<2>;
    impl crate::private::Sealed for SAI2 {}
    impl crate::Valid for SAI2 {}
    impl SAI2 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(SAI2)
        }
    }
    pub type SAI3 = Instance<3>;
    impl crate::private::Sealed for SAI3 {}
    impl crate::Valid for SAI3 {}
    impl SAI3 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(SAI3)
        }
    }
    pub type SAI4 = Instance<4>;
    impl crate::private::Sealed for SAI4 {}
    impl crate::Valid for SAI4 {}
    impl SAI4 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(SAI4)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        [(SAI1, 1), (SAI2, 2), (SAI3, 3), (SAI4, 4)]
            .into_iter()
            .find(|(ptr, _)| core::ptr::eq(rb, *ptr))
            .map(|(_, inst)| inst)
    }
}
#[path = "."]
pub mod sema4 {
    #[doc = "IPS_Semaphores"]
    pub const SEMA4: *const RegisterBlock = 0x40cc_8000 as *const RegisterBlock;
    #[path = "blocks/imxrt1176_cm4/sema4.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type SEMA4 = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for SEMA4 {}
    impl crate::Valid for SEMA4 {}
    impl SEMA4 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(SEMA4)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, SEMA4).then_some(0)
    }
}
#[path = "."]
pub mod semc {
    #[doc = "SEMC"]
    pub const SEMC: *const RegisterBlock = 0x400d_4000 as *const RegisterBlock;
    #[path = "blocks/imxrt1176_cm4/semc.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type SEMC = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for SEMC {}
    impl crate::Valid for SEMC {}
    impl SEMC {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(SEMC)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, SEMC).then_some(0)
    }
}
#[path = "."]
pub mod snvs {
    #[doc = "SNVS"]
    pub const SNVS: *const RegisterBlock = 0x40c9_0000 as *const RegisterBlock;
    #[path = "blocks/imxrt1176_cm4/snvs.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type SNVS = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for SNVS {}
    impl crate::Valid for SNVS {}
    impl SNVS {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(SNVS)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, SNVS).then_some(0)
    }
}
#[path = "."]
pub mod spdif {
    #[doc = "SPDIF"]
    pub const SPDIF: *const RegisterBlock = 0x4040_0000 as *const RegisterBlock;
    #[path = "blocks/imxrt1176_cm4/spdif.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type SPDIF = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for SPDIF {}
    impl crate::Valid for SPDIF {}
    impl SPDIF {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(SPDIF)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, SPDIF).then_some(0)
    }
}
#[path = "."]
pub mod sram {
    #[doc = "Secure RAM"]
    pub const SRAM: *const RegisterBlock = 0x40c9_c000 as *const RegisterBlock;
    #[path = "blocks/imxrt1176_cm4/sram.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type SRAM = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for SRAM {}
    impl crate::Valid for SRAM {}
    impl SRAM {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(SRAM)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, SRAM).then_some(0)
    }
}
#[path = "."]
pub mod src {
    #[doc = "SRC"]
    pub const SRC: *const RegisterBlock = 0x40c0_4000 as *const RegisterBlock;
    #[path = "blocks/imxrt1176_cm4/src.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type SRC = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for SRC {}
    impl crate::Valid for SRC {}
    impl SRC {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(SRC)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, SRC).then_some(0)
    }
}
#[path = "."]
pub mod ssarc_hp {
    #[doc = "SRAM Registers"]
    pub const SSARC_HP: *const RegisterBlock = 0x40cb_4000 as *const RegisterBlock;
    #[path = "blocks/imxrt1176_cm4/ssarc_hp.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type SSARC_HP = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for SSARC_HP {}
    impl crate::Valid for SSARC_HP {}
    impl SSARC_HP {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(SSARC_HP)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, SSARC_HP).then_some(0)
    }
}
#[path = "."]
pub mod ssarc_lp {
    #[doc = "SSARC Registers"]
    pub const SSARC_LP: *const RegisterBlock = 0x40cb_8000 as *const RegisterBlock;
    #[path = "blocks/imxrt1176_cm4/ssarc_lp.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type SSARC_LP = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for SSARC_LP {}
    impl crate::Valid for SSARC_LP {}
    impl SSARC_LP {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(SSARC_LP)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, SSARC_LP).then_some(0)
    }
}
#[path = "."]
pub mod tmpsns {
    #[path = "blocks/imxrt1176_cm4/tmpsns.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
}
#[path = "."]
pub mod tmr {
    #[doc = "TMR"]
    pub const TMR1: *const RegisterBlock = 0x4015_c000 as *const RegisterBlock;
    #[doc = "TMR"]
    pub const TMR2: *const RegisterBlock = 0x4016_0000 as *const RegisterBlock;
    #[doc = "TMR"]
    pub const TMR3: *const RegisterBlock = 0x4016_4000 as *const RegisterBlock;
    #[doc = "TMR"]
    pub const TMR4: *const RegisterBlock = 0x4016_8000 as *const RegisterBlock;
    #[path = "blocks/imxrt1176_cm4/tmr.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type TMR1 = Instance<1>;
    impl crate::private::Sealed for TMR1 {}
    impl crate::Valid for TMR1 {}
    impl TMR1 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(TMR1)
        }
    }
    pub type TMR2 = Instance<2>;
    impl crate::private::Sealed for TMR2 {}
    impl crate::Valid for TMR2 {}
    impl TMR2 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(TMR2)
        }
    }
    pub type TMR3 = Instance<3>;
    impl crate::private::Sealed for TMR3 {}
    impl crate::Valid for TMR3 {}
    impl TMR3 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(TMR3)
        }
    }
    pub type TMR4 = Instance<4>;
    impl crate::private::Sealed for TMR4 {}
    impl crate::Valid for TMR4 {}
    impl TMR4 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(TMR4)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        [(TMR1, 1), (TMR2, 2), (TMR3, 3), (TMR4, 4)]
            .into_iter()
            .find(|(ptr, _)| core::ptr::eq(rb, *ptr))
            .map(|(_, inst)| inst)
    }
}
#[path = "."]
pub mod usb {
    #[doc = "USB"]
    pub const USB1: *const RegisterBlock = 0x4043_0000 as *const RegisterBlock;
    pub const USB2: *const RegisterBlock = 0x4042_c000 as *const RegisterBlock;
    #[path = "blocks/imxrt1011/usb.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type USB1 = Instance<1>;
    impl crate::private::Sealed for USB1 {}
    impl crate::Valid for USB1 {}
    impl USB1 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(USB1)
        }
    }
    pub type USB2 = Instance<2>;
    impl crate::private::Sealed for USB2 {}
    impl crate::Valid for USB2 {}
    impl USB2 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(USB2)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        [(USB1, 1), (USB2, 2)]
            .into_iter()
            .find(|(ptr, _)| core::ptr::eq(rb, *ptr))
            .map(|(_, inst)| inst)
    }
}
#[path = "."]
pub mod usbhsdcd {
    #[doc = "USBDCD"]
    pub const USBHSDCD1: *const RegisterBlock = 0x4043_4800 as *const RegisterBlock;
    #[doc = "USBDCD"]
    pub const USBHSDCD2: *const RegisterBlock = 0x4043_8800 as *const RegisterBlock;
    #[path = "blocks/imxrt1176_cm4/usbhsdcd.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type USBHSDCD1 = Instance<1>;
    impl crate::private::Sealed for USBHSDCD1 {}
    impl crate::Valid for USBHSDCD1 {}
    impl USBHSDCD1 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(USBHSDCD1)
        }
    }
    pub type USBHSDCD2 = Instance<2>;
    impl crate::private::Sealed for USBHSDCD2 {}
    impl crate::Valid for USBHSDCD2 {}
    impl USBHSDCD2 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(USBHSDCD2)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        [(USBHSDCD1, 1), (USBHSDCD2, 2)]
            .into_iter()
            .find(|(ptr, _)| core::ptr::eq(rb, *ptr))
            .map(|(_, inst)| inst)
    }
}
#[path = "."]
pub mod usbnc {
    #[doc = "USBNC"]
    pub const USBNC1: *const RegisterBlock = 0x4043_0200 as *const RegisterBlock;
    pub const USBNC2: *const RegisterBlock = 0x4042_c200 as *const RegisterBlock;
    #[path = "blocks/imxrt1176_cm4/usbnc.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type USBNC1 = Instance<1>;
    impl crate::private::Sealed for USBNC1 {}
    impl crate::Valid for USBNC1 {}
    impl USBNC1 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(USBNC1)
        }
    }
    pub type USBNC2 = Instance<2>;
    impl crate::private::Sealed for USBNC2 {}
    impl crate::Valid for USBNC2 {}
    impl USBNC2 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(USBNC2)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        [(USBNC1, 1), (USBNC2, 2)]
            .into_iter()
            .find(|(ptr, _)| core::ptr::eq(rb, *ptr))
            .map(|(_, inst)| inst)
    }
}
#[path = "."]
pub mod usbphy {
    #[doc = "USBPHY"]
    pub const USBPHY1: *const RegisterBlock = 0x4043_4000 as *const RegisterBlock;
    #[doc = "USBPHY"]
    pub const USBPHY2: *const RegisterBlock = 0x4043_8000 as *const RegisterBlock;
    #[path = "blocks/imxrt1176_cm4/usbphy.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type USBPHY1 = Instance<1>;
    impl crate::private::Sealed for USBPHY1 {}
    impl crate::Valid for USBPHY1 {}
    impl USBPHY1 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(USBPHY1)
        }
    }
    pub type USBPHY2 = Instance<2>;
    impl crate::private::Sealed for USBPHY2 {}
    impl crate::Valid for USBPHY2 {}
    impl USBPHY2 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(USBPHY2)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        [(USBPHY1, 1), (USBPHY2, 2)]
            .into_iter()
            .find(|(ptr, _)| core::ptr::eq(rb, *ptr))
            .map(|(_, inst)| inst)
    }
}
#[path = "."]
pub mod usdhc {
    #[doc = "uSDHC"]
    pub const USDHC1: *const RegisterBlock = 0x4041_8000 as *const RegisterBlock;
    #[doc = "uSDHC"]
    pub const USDHC2: *const RegisterBlock = 0x4041_c000 as *const RegisterBlock;
    #[path = "blocks/imxrt1176_cm4/usdhc.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type USDHC1 = Instance<1>;
    impl crate::private::Sealed for USDHC1 {}
    impl crate::Valid for USDHC1 {}
    impl USDHC1 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(USDHC1)
        }
    }
    pub type USDHC2 = Instance<2>;
    impl crate::private::Sealed for USDHC2 {}
    impl crate::Valid for USDHC2 {}
    impl USDHC2 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(USDHC2)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        [(USDHC1, 1), (USDHC2, 2)]
            .into_iter()
            .find(|(ptr, _)| core::ptr::eq(rb, *ptr))
            .map(|(_, inst)| inst)
    }
}
#[path = "."]
pub mod video_mux {
    #[doc = "VIDEO_MUX"]
    pub const VIDEO_MUX: *const RegisterBlock = 0x4081_8000 as *const RegisterBlock;
    #[path = "blocks/imxrt1176_cm4/video_mux.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type VIDEO_MUX = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for VIDEO_MUX {}
    impl crate::Valid for VIDEO_MUX {}
    impl VIDEO_MUX {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(VIDEO_MUX)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, VIDEO_MUX).then_some(0)
    }
}
#[path = "."]
pub mod vmbandgap {
    #[path = "blocks/imxrt1176_cm4/vmbandgap.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
}
#[path = "."]
pub mod wdog {
    #[doc = "WDOG"]
    pub const WDOG1: *const RegisterBlock = 0x4003_0000 as *const RegisterBlock;
    #[doc = "WDOG"]
    pub const WDOG2: *const RegisterBlock = 0x4003_4000 as *const RegisterBlock;
    #[path = "blocks/imxrt1011/wdog.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type WDOG1 = Instance<1>;
    impl crate::private::Sealed for WDOG1 {}
    impl crate::Valid for WDOG1 {}
    impl WDOG1 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(WDOG1)
        }
    }
    pub type WDOG2 = Instance<2>;
    impl crate::private::Sealed for WDOG2 {}
    impl crate::Valid for WDOG2 {}
    impl WDOG2 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(WDOG2)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        [(WDOG1, 1), (WDOG2, 2)]
            .into_iter()
            .find(|(ptr, _)| core::ptr::eq(rb, *ptr))
            .map(|(_, inst)| inst)
    }
}
#[path = "."]
pub mod xbara1 {
    #[doc = "Crossbar Switch"]
    pub const XBARA1: *const RegisterBlock = 0x4003_c000 as *const RegisterBlock;
    #[path = "blocks/imxrt1176_cm4/xbara1.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type XBARA1 = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for XBARA1 {}
    impl crate::Valid for XBARA1 {}
    impl XBARA1 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(XBARA1)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, XBARA1).then_some(0)
    }
}
#[path = "."]
pub mod xbarb {
    #[doc = "Crossbar Switch"]
    pub const XBARB2: *const RegisterBlock = 0x4004_0000 as *const RegisterBlock;
    #[doc = "Crossbar Switch"]
    pub const XBARB3: *const RegisterBlock = 0x4004_4000 as *const RegisterBlock;
    #[path = "blocks/imxrt1176_cm4/xbarb.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type XBARB2 = Instance<2>;
    impl crate::private::Sealed for XBARB2 {}
    impl crate::Valid for XBARB2 {}
    impl XBARB2 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(XBARB2)
        }
    }
    pub type XBARB3 = Instance<3>;
    impl crate::private::Sealed for XBARB3 {}
    impl crate::Valid for XBARB3 {}
    impl XBARB3 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(XBARB3)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        [(XBARB2, 2), (XBARB3, 3)]
            .into_iter()
            .find(|(ptr, _)| core::ptr::eq(rb, *ptr))
            .map(|(_, inst)| inst)
    }
}
#[path = "."]
pub mod xecc_flexspi {
    #[doc = "XECC"]
    pub const XECC_FLEXSPI1: *const RegisterBlock = 0x4001_c000 as *const RegisterBlock;
    #[doc = "XECC"]
    pub const XECC_FLEXSPI2: *const RegisterBlock = 0x4002_0000 as *const RegisterBlock;
    #[doc = "XECC"]
    pub const XECC_SEMC0: *const RegisterBlock = 0x4002_4000 as *const RegisterBlock;
    #[path = "blocks/imxrt1176_cm4/xecc_flexspi.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type XECC_FLEXSPI1 = Instance<1>;
    impl crate::private::Sealed for XECC_FLEXSPI1 {}
    impl crate::Valid for XECC_FLEXSPI1 {}
    impl XECC_FLEXSPI1 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(XECC_FLEXSPI1)
        }
    }
    pub type XECC_FLEXSPI2 = Instance<2>;
    impl crate::private::Sealed for XECC_FLEXSPI2 {}
    impl crate::Valid for XECC_FLEXSPI2 {}
    impl XECC_FLEXSPI2 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(XECC_FLEXSPI2)
        }
    }
    pub type XECC_SEMC0 = Instance<0>;
    impl crate::private::Sealed for XECC_SEMC0 {}
    impl crate::Valid for XECC_SEMC0 {}
    impl XECC_SEMC0 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(XECC_SEMC0)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        [(XECC_FLEXSPI1, 1), (XECC_FLEXSPI2, 2), (XECC_SEMC0, 0)]
            .into_iter()
            .find(|(ptr, _)| core::ptr::eq(rb, *ptr))
            .map(|(_, inst)| inst)
    }
}
#[path = "."]
pub mod xrdc2_d {
    #[doc = "XRDC2"]
    pub const XRDC2_D0: *const RegisterBlock = 0x40ce_0000 as *const RegisterBlock;
    #[doc = "XRDC2"]
    pub const XRDC2_D1: *const RegisterBlock = 0x40cd_0000 as *const RegisterBlock;
    #[path = "blocks/imxrt1176_cm4/xrdc2_d.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type XRDC2_D0 = Instance<0>;
    impl crate::private::Sealed for XRDC2_D0 {}
    impl crate::Valid for XRDC2_D0 {}
    impl XRDC2_D0 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(XRDC2_D0)
        }
    }
    pub type XRDC2_D1 = Instance<1>;
    impl crate::private::Sealed for XRDC2_D1 {}
    impl crate::Valid for XRDC2_D1 {}
    impl XRDC2_D1 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(XRDC2_D1)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        [(XRDC2_D0, 0), (XRDC2_D1, 1)]
            .into_iter()
            .find(|(ptr, _)| core::ptr::eq(rb, *ptr))
            .map(|(_, inst)| inst)
    }
}
#[doc = r"Number available in the NVIC for configuring priority"]
pub const NVIC_PRIO_BITS: u8 = 4;
#[doc = r" Instances for all of this device's peripherals."]
#[doc = r""]
#[doc = r" Use this if you want a single way to acquire *all* instances"]
#[doc = r" for your device."]
pub struct Instances {
    pub ADC_ETC: adc_etc::ADC_ETC,
    pub ANADIG_LDO_SNVS: anadig_ldo_snvs::ANADIG_LDO_SNVS,
    pub ANADIG_LDO_SNVS_DIG: anadig_ldo_snvs_dig::ANADIG_LDO_SNVS_DIG,
    pub ANADIG_MISC: anadig_misc::ANADIG_MISC,
    pub ANADIG_OSC: anadig_osc::ANADIG_OSC,
    pub ANADIG_PLL: anadig_pll::ANADIG_PLL,
    pub ANADIG_PMU: anadig_pmu::ANADIG_PMU,
    pub ANADIG_TEMPSENSOR: anadig_tempsensor::ANADIG_TEMPSENSOR,
    pub AOI1: aoi::AOI1,
    pub AOI2: aoi::AOI2,
    pub ASRC: asrc::ASRC,
    pub CAN1: can::CAN1,
    pub CAN2: can::CAN2,
    pub CAN3: can::CAN3,
    pub CAN_WRAPPER1: can1_wrapper::CAN_WRAPPER1,
    pub CAN_WRAPPER2: can1_wrapper::CAN_WRAPPER2,
    pub CAN_WRAPPER3: can1_wrapper::CAN_WRAPPER3,
    pub CCM: ccm::CCM,
    pub CCM_OBS: ccm_obs::CCM_OBS,
    pub CDOG: cdog::CDOG,
    pub CMP1: cmp::CMP1,
    pub CMP2: cmp::CMP2,
    pub CMP3: cmp::CMP3,
    pub CMP4: cmp::CMP4,
    pub CSI: csi::CSI,
    pub DAC: dac::DAC,
    pub DCDC: dcdc::DCDC,
    pub DCIC1: dcic::DCIC1,
    pub DCIC2: dcic::DCIC2,
    pub DMA: dma::DMA,
    pub DMAMUX: dmamux::DMAMUX,
    pub DSI_HOST: dsi_host::DSI_HOST,
    pub DSI_HOST_APB_PKT_IF: dsi_host_apb_pkt_if::DSI_HOST_APB_PKT_IF,
    pub DSI_HOST_DPHY_INTFC: dsi_host_dphy_intfc::DSI_HOST_DPHY_INTFC,
    pub DSI_HOST_DPI_INTFC: dsi_host_dpi_intfc::DSI_HOST_DPI_INTFC,
    pub EMVSIM1: emvsim::EMVSIM1,
    pub EMVSIM2: emvsim::EMVSIM2,
    pub ENC1: enc::ENC1,
    pub ENC2: enc::ENC2,
    pub ENC3: enc::ENC3,
    pub ENC4: enc::ENC4,
    pub ENET: enet::ENET,
    pub ENET_1G: enet_1g::ENET_1G,
    pub ENET_QOS: enet_qos::ENET_QOS,
    pub EWM: ewm::EWM,
    pub FLEXIO1: flexio::FLEXIO1,
    pub FLEXIO2: flexio::FLEXIO2,
    pub FLEXRAM: flexram::FLEXRAM,
    pub FLEXSPI1: flexspi::FLEXSPI1,
    pub FLEXSPI2: flexspi::FLEXSPI2,
    pub GPC_CPU_MODE_CTRL_0: gpc_cpu_mode_ctrl_::GPC_CPU_MODE_CTRL_0,
    pub GPC_CPU_MODE_CTRL_1: gpc_cpu_mode_ctrl_::GPC_CPU_MODE_CTRL_1,
    pub GPC_SET_POINT_CTRL: gpc_set_point_ctrl::GPC_SET_POINT_CTRL,
    pub GPC_STBY_CTRL: gpc_stby_ctrl::GPC_STBY_CTRL,
    pub GPIO2: gpio::GPIO2,
    pub GPIO1: gpio::GPIO1,
    pub GPIO3: gpio::GPIO3,
    pub GPIO4: gpio::GPIO4,
    pub GPIO5: gpio::GPIO5,
    pub GPIO6: gpio::GPIO6,
    pub GPIO7: gpio::GPIO7,
    pub GPIO8: gpio::GPIO8,
    pub GPIO9: gpio::GPIO9,
    pub GPIO10: gpio::GPIO10,
    pub GPIO11: gpio::GPIO11,
    pub GPIO12: gpio::GPIO12,
    pub GPIO13: gpio::GPIO13,
    pub GPT1: gpt::GPT1,
    pub GPT2: gpt::GPT2,
    pub GPT3: gpt::GPT3,
    pub GPT4: gpt::GPT4,
    pub GPT5: gpt::GPT5,
    pub GPT6: gpt::GPT6,
    pub IEE__IEE_RT1170: iee__iee_rt1170::IEE__IEE_RT1170,
    pub IEE_APC: iee_apc::IEE_APC,
    pub IOMUXC: iomuxc::IOMUXC,
    pub IOMUXC_GPR: iomuxc_gpr::IOMUXC_GPR,
    pub IOMUXC_LPSR: iomuxc_lpsr::IOMUXC_LPSR,
    pub IOMUXC_LPSR_GPR: iomuxc_lpsr_gpr::IOMUXC_LPSR_GPR,
    pub IOMUXC_SNVS: iomuxc_snvs::IOMUXC_SNVS,
    pub IOMUXC_SNVS_GPR: iomuxc_snvs_gpr::IOMUXC_SNVS_GPR,
    pub IPS_DOMAIN: ips_domain::IPS_DOMAIN,
    pub KEY_MANAGER: key_manager::KEY_MANAGER,
    pub KEY_MANAGER__PUF: key_manager__puf::KEY_MANAGER__PUF,
    pub KPP: kpp::KPP,
    pub LCDIF: lcdif::LCDIF,
    pub LCDIFV2: lcdifv2::LCDIFV2,
    pub LMEM: lmem::LMEM,
    pub LPADC1: lpadc::LPADC1,
    pub LPADC2: lpadc::LPADC2,
    pub LPI2C1: lpi2c::LPI2C1,
    pub LPI2C2: lpi2c::LPI2C2,
    pub LPI2C3: lpi2c::LPI2C3,
    pub LPI2C4: lpi2c::LPI2C4,
    pub LPI2C5: lpi2c::LPI2C5,
    pub LPI2C6: lpi2c::LPI2C6,
    pub LPSPI1: lpspi::LPSPI1,
    pub LPSPI2: lpspi::LPSPI2,
    pub LPSPI3: lpspi::LPSPI3,
    pub LPSPI4: lpspi::LPSPI4,
    pub LPSPI5: lpspi::LPSPI5,
    pub LPSPI6: lpspi::LPSPI6,
    pub LPUART1: lpuart::LPUART1,
    pub LPUART2: lpuart::LPUART2,
    pub LPUART3: lpuart::LPUART3,
    pub LPUART4: lpuart::LPUART4,
    pub LPUART5: lpuart::LPUART5,
    pub LPUART6: lpuart::LPUART6,
    pub LPUART7: lpuart::LPUART7,
    pub LPUART8: lpuart::LPUART8,
    pub LPUART9: lpuart::LPUART9,
    pub LPUART10: lpuart::LPUART10,
    pub LPUART11: lpuart::LPUART11,
    pub LPUART12: lpuart::LPUART12,
    pub MCM: mcm::MCM,
    pub MECC1: mecc::MECC1,
    pub MECC2: mecc::MECC2,
    pub MIPI_CSI2RX: mipi_csi2rx::MIPI_CSI2RX,
    pub MMCAU: mmcau::MMCAU,
    pub MUB: mub::MUB,
    pub OCOTP: ocotp::OCOTP,
    pub OTFAD1: otfad::OTFAD1,
    pub OTFAD2: otfad::OTFAD2,
    pub PDM: pdm::PDM,
    pub PGMC_BPC0: pgmc_bpc::PGMC_BPC0,
    pub PGMC_BPC1: pgmc_bpc::PGMC_BPC1,
    pub PGMC_BPC2: pgmc_bpc::PGMC_BPC2,
    pub PGMC_BPC3: pgmc_bpc::PGMC_BPC3,
    pub PGMC_BPC4: pgmc_bpc::PGMC_BPC4,
    pub PGMC_BPC5: pgmc_bpc::PGMC_BPC5,
    pub PGMC_BPC6: pgmc_bpc::PGMC_BPC6,
    pub PGMC_BPC7: pgmc_bpc::PGMC_BPC7,
    pub PGMC_CPC0: pgmc_cpc::PGMC_CPC0,
    pub PGMC_CPC1: pgmc_cpc::PGMC_CPC1,
    pub PGMC_CPC_MIF00: pgmc_cpc0_mif::PGMC_CPC_MIF00,
    pub PGMC_CPC_MIF01: pgmc_cpc0_mif::PGMC_CPC_MIF01,
    pub PGMC_CPC_MIF10: pgmc_cpc0_mif::PGMC_CPC_MIF10,
    pub PGMC_CPC_MIF11: pgmc_cpc0_mif::PGMC_CPC_MIF11,
    pub PGMC_PPC0: pgmc_ppc0::PGMC_PPC0,
    pub PIT1: pit::PIT1,
    pub PIT2: pit::PIT2,
    pub PWM1: pwm::PWM1,
    pub PWM2: pwm::PWM2,
    pub PWM3: pwm::PWM3,
    pub PWM4: pwm::PWM4,
    pub PXP: pxp::PXP,
    pub RDC: rdc::RDC,
    pub RDC_SEMAPHORE1: rdc_semaphore::RDC_SEMAPHORE1,
    pub RDC_SEMAPHORE2: rdc_semaphore::RDC_SEMAPHORE2,
    pub RTWDOG3: rtwdog::RTWDOG3,
    pub RTWDOG4: rtwdog::RTWDOG4,
    pub SAI1: sai::SAI1,
    pub SAI2: sai::SAI2,
    pub SAI3: sai::SAI3,
    pub SAI4: sai::SAI4,
    pub SEMA4: sema4::SEMA4,
    pub SEMC: semc::SEMC,
    pub SNVS: snvs::SNVS,
    pub SPDIF: spdif::SPDIF,
    pub SRAM: sram::SRAM,
    pub SRC: src::SRC,
    pub SSARC_HP: ssarc_hp::SSARC_HP,
    pub SSARC_LP: ssarc_lp::SSARC_LP,
    pub TMR1: tmr::TMR1,
    pub TMR2: tmr::TMR2,
    pub TMR3: tmr::TMR3,
    pub TMR4: tmr::TMR4,
    pub USB1: usb::USB1,
    pub USB2: usb::USB2,
    pub USBHSDCD1: usbhsdcd::USBHSDCD1,
    pub USBHSDCD2: usbhsdcd::USBHSDCD2,
    pub USBNC1: usbnc::USBNC1,
    pub USBNC2: usbnc::USBNC2,
    pub USBPHY1: usbphy::USBPHY1,
    pub USBPHY2: usbphy::USBPHY2,
    pub USDHC1: usdhc::USDHC1,
    pub USDHC2: usdhc::USDHC2,
    pub VIDEO_MUX: video_mux::VIDEO_MUX,
    pub WDOG1: wdog::WDOG1,
    pub WDOG2: wdog::WDOG2,
    pub XBARA1: xbara1::XBARA1,
    pub XBARB2: xbarb::XBARB2,
    pub XBARB3: xbarb::XBARB3,
    pub XECC_FLEXSPI1: xecc_flexspi::XECC_FLEXSPI1,
    pub XECC_FLEXSPI2: xecc_flexspi::XECC_FLEXSPI2,
    pub XECC_SEMC0: xecc_flexspi::XECC_SEMC0,
    pub XRDC2_D0: xrdc2_d::XRDC2_D0,
    pub XRDC2_D1: xrdc2_d::XRDC2_D1,
}
impl Instances {
    #[doc = r" Acquire all peripheral instances."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Since this calls `instance()` to initialize each of its members,"]
    #[doc = r" the `instance()` safety contract applies. See [the `Instance` safety"]
    #[doc = r" documentation](crate::Instance) for more information."]
    #[inline]
    pub const unsafe fn instances() -> Self {
        Self {
            ADC_ETC: adc_etc::ADC_ETC::instance(),
            ANADIG_LDO_SNVS: anadig_ldo_snvs::ANADIG_LDO_SNVS::instance(),
            ANADIG_LDO_SNVS_DIG: anadig_ldo_snvs_dig::ANADIG_LDO_SNVS_DIG::instance(),
            ANADIG_MISC: anadig_misc::ANADIG_MISC::instance(),
            ANADIG_OSC: anadig_osc::ANADIG_OSC::instance(),
            ANADIG_PLL: anadig_pll::ANADIG_PLL::instance(),
            ANADIG_PMU: anadig_pmu::ANADIG_PMU::instance(),
            ANADIG_TEMPSENSOR: anadig_tempsensor::ANADIG_TEMPSENSOR::instance(),
            AOI1: aoi::AOI1::instance(),
            AOI2: aoi::AOI2::instance(),
            ASRC: asrc::ASRC::instance(),
            CAN1: can::CAN1::instance(),
            CAN2: can::CAN2::instance(),
            CAN3: can::CAN3::instance(),
            CAN_WRAPPER1: can1_wrapper::CAN_WRAPPER1::instance(),
            CAN_WRAPPER2: can1_wrapper::CAN_WRAPPER2::instance(),
            CAN_WRAPPER3: can1_wrapper::CAN_WRAPPER3::instance(),
            CCM: ccm::CCM::instance(),
            CCM_OBS: ccm_obs::CCM_OBS::instance(),
            CDOG: cdog::CDOG::instance(),
            CMP1: cmp::CMP1::instance(),
            CMP2: cmp::CMP2::instance(),
            CMP3: cmp::CMP3::instance(),
            CMP4: cmp::CMP4::instance(),
            CSI: csi::CSI::instance(),
            DAC: dac::DAC::instance(),
            DCDC: dcdc::DCDC::instance(),
            DCIC1: dcic::DCIC1::instance(),
            DCIC2: dcic::DCIC2::instance(),
            DMA: dma::DMA::instance(),
            DMAMUX: dmamux::DMAMUX::instance(),
            DSI_HOST: dsi_host::DSI_HOST::instance(),
            DSI_HOST_APB_PKT_IF: dsi_host_apb_pkt_if::DSI_HOST_APB_PKT_IF::instance(),
            DSI_HOST_DPHY_INTFC: dsi_host_dphy_intfc::DSI_HOST_DPHY_INTFC::instance(),
            DSI_HOST_DPI_INTFC: dsi_host_dpi_intfc::DSI_HOST_DPI_INTFC::instance(),
            EMVSIM1: emvsim::EMVSIM1::instance(),
            EMVSIM2: emvsim::EMVSIM2::instance(),
            ENC1: enc::ENC1::instance(),
            ENC2: enc::ENC2::instance(),
            ENC3: enc::ENC3::instance(),
            ENC4: enc::ENC4::instance(),
            ENET: enet::ENET::instance(),
            ENET_1G: enet_1g::ENET_1G::instance(),
            ENET_QOS: enet_qos::ENET_QOS::instance(),
            EWM: ewm::EWM::instance(),
            FLEXIO1: flexio::FLEXIO1::instance(),
            FLEXIO2: flexio::FLEXIO2::instance(),
            FLEXRAM: flexram::FLEXRAM::instance(),
            FLEXSPI1: flexspi::FLEXSPI1::instance(),
            FLEXSPI2: flexspi::FLEXSPI2::instance(),
            GPC_CPU_MODE_CTRL_0: gpc_cpu_mode_ctrl_::GPC_CPU_MODE_CTRL_0::instance(),
            GPC_CPU_MODE_CTRL_1: gpc_cpu_mode_ctrl_::GPC_CPU_MODE_CTRL_1::instance(),
            GPC_SET_POINT_CTRL: gpc_set_point_ctrl::GPC_SET_POINT_CTRL::instance(),
            GPC_STBY_CTRL: gpc_stby_ctrl::GPC_STBY_CTRL::instance(),
            GPIO2: gpio::GPIO2::instance(),
            GPIO1: gpio::GPIO1::instance(),
            GPIO3: gpio::GPIO3::instance(),
            GPIO4: gpio::GPIO4::instance(),
            GPIO5: gpio::GPIO5::instance(),
            GPIO6: gpio::GPIO6::instance(),
            GPIO7: gpio::GPIO7::instance(),
            GPIO8: gpio::GPIO8::instance(),
            GPIO9: gpio::GPIO9::instance(),
            GPIO10: gpio::GPIO10::instance(),
            GPIO11: gpio::GPIO11::instance(),
            GPIO12: gpio::GPIO12::instance(),
            GPIO13: gpio::GPIO13::instance(),
            GPT1: gpt::GPT1::instance(),
            GPT2: gpt::GPT2::instance(),
            GPT3: gpt::GPT3::instance(),
            GPT4: gpt::GPT4::instance(),
            GPT5: gpt::GPT5::instance(),
            GPT6: gpt::GPT6::instance(),
            IEE__IEE_RT1170: iee__iee_rt1170::IEE__IEE_RT1170::instance(),
            IEE_APC: iee_apc::IEE_APC::instance(),
            IOMUXC: iomuxc::IOMUXC::instance(),
            IOMUXC_GPR: iomuxc_gpr::IOMUXC_GPR::instance(),
            IOMUXC_LPSR: iomuxc_lpsr::IOMUXC_LPSR::instance(),
            IOMUXC_LPSR_GPR: iomuxc_lpsr_gpr::IOMUXC_LPSR_GPR::instance(),
            IOMUXC_SNVS: iomuxc_snvs::IOMUXC_SNVS::instance(),
            IOMUXC_SNVS_GPR: iomuxc_snvs_gpr::IOMUXC_SNVS_GPR::instance(),
            IPS_DOMAIN: ips_domain::IPS_DOMAIN::instance(),
            KEY_MANAGER: key_manager::KEY_MANAGER::instance(),
            KEY_MANAGER__PUF: key_manager__puf::KEY_MANAGER__PUF::instance(),
            KPP: kpp::KPP::instance(),
            LCDIF: lcdif::LCDIF::instance(),
            LCDIFV2: lcdifv2::LCDIFV2::instance(),
            LMEM: lmem::LMEM::instance(),
            LPADC1: lpadc::LPADC1::instance(),
            LPADC2: lpadc::LPADC2::instance(),
            LPI2C1: lpi2c::LPI2C1::instance(),
            LPI2C2: lpi2c::LPI2C2::instance(),
            LPI2C3: lpi2c::LPI2C3::instance(),
            LPI2C4: lpi2c::LPI2C4::instance(),
            LPI2C5: lpi2c::LPI2C5::instance(),
            LPI2C6: lpi2c::LPI2C6::instance(),
            LPSPI1: lpspi::LPSPI1::instance(),
            LPSPI2: lpspi::LPSPI2::instance(),
            LPSPI3: lpspi::LPSPI3::instance(),
            LPSPI4: lpspi::LPSPI4::instance(),
            LPSPI5: lpspi::LPSPI5::instance(),
            LPSPI6: lpspi::LPSPI6::instance(),
            LPUART1: lpuart::LPUART1::instance(),
            LPUART2: lpuart::LPUART2::instance(),
            LPUART3: lpuart::LPUART3::instance(),
            LPUART4: lpuart::LPUART4::instance(),
            LPUART5: lpuart::LPUART5::instance(),
            LPUART6: lpuart::LPUART6::instance(),
            LPUART7: lpuart::LPUART7::instance(),
            LPUART8: lpuart::LPUART8::instance(),
            LPUART9: lpuart::LPUART9::instance(),
            LPUART10: lpuart::LPUART10::instance(),
            LPUART11: lpuart::LPUART11::instance(),
            LPUART12: lpuart::LPUART12::instance(),
            MCM: mcm::MCM::instance(),
            MECC1: mecc::MECC1::instance(),
            MECC2: mecc::MECC2::instance(),
            MIPI_CSI2RX: mipi_csi2rx::MIPI_CSI2RX::instance(),
            MMCAU: mmcau::MMCAU::instance(),
            MUB: mub::MUB::instance(),
            OCOTP: ocotp::OCOTP::instance(),
            OTFAD1: otfad::OTFAD1::instance(),
            OTFAD2: otfad::OTFAD2::instance(),
            PDM: pdm::PDM::instance(),
            PGMC_BPC0: pgmc_bpc::PGMC_BPC0::instance(),
            PGMC_BPC1: pgmc_bpc::PGMC_BPC1::instance(),
            PGMC_BPC2: pgmc_bpc::PGMC_BPC2::instance(),
            PGMC_BPC3: pgmc_bpc::PGMC_BPC3::instance(),
            PGMC_BPC4: pgmc_bpc::PGMC_BPC4::instance(),
            PGMC_BPC5: pgmc_bpc::PGMC_BPC5::instance(),
            PGMC_BPC6: pgmc_bpc::PGMC_BPC6::instance(),
            PGMC_BPC7: pgmc_bpc::PGMC_BPC7::instance(),
            PGMC_CPC0: pgmc_cpc::PGMC_CPC0::instance(),
            PGMC_CPC1: pgmc_cpc::PGMC_CPC1::instance(),
            PGMC_CPC_MIF00: pgmc_cpc0_mif::PGMC_CPC_MIF00::instance(),
            PGMC_CPC_MIF01: pgmc_cpc0_mif::PGMC_CPC_MIF01::instance(),
            PGMC_CPC_MIF10: pgmc_cpc0_mif::PGMC_CPC_MIF10::instance(),
            PGMC_CPC_MIF11: pgmc_cpc0_mif::PGMC_CPC_MIF11::instance(),
            PGMC_PPC0: pgmc_ppc0::PGMC_PPC0::instance(),
            PIT1: pit::PIT1::instance(),
            PIT2: pit::PIT2::instance(),
            PWM1: pwm::PWM1::instance(),
            PWM2: pwm::PWM2::instance(),
            PWM3: pwm::PWM3::instance(),
            PWM4: pwm::PWM4::instance(),
            PXP: pxp::PXP::instance(),
            RDC: rdc::RDC::instance(),
            RDC_SEMAPHORE1: rdc_semaphore::RDC_SEMAPHORE1::instance(),
            RDC_SEMAPHORE2: rdc_semaphore::RDC_SEMAPHORE2::instance(),
            RTWDOG3: rtwdog::RTWDOG3::instance(),
            RTWDOG4: rtwdog::RTWDOG4::instance(),
            SAI1: sai::SAI1::instance(),
            SAI2: sai::SAI2::instance(),
            SAI3: sai::SAI3::instance(),
            SAI4: sai::SAI4::instance(),
            SEMA4: sema4::SEMA4::instance(),
            SEMC: semc::SEMC::instance(),
            SNVS: snvs::SNVS::instance(),
            SPDIF: spdif::SPDIF::instance(),
            SRAM: sram::SRAM::instance(),
            SRC: src::SRC::instance(),
            SSARC_HP: ssarc_hp::SSARC_HP::instance(),
            SSARC_LP: ssarc_lp::SSARC_LP::instance(),
            TMR1: tmr::TMR1::instance(),
            TMR2: tmr::TMR2::instance(),
            TMR3: tmr::TMR3::instance(),
            TMR4: tmr::TMR4::instance(),
            USB1: usb::USB1::instance(),
            USB2: usb::USB2::instance(),
            USBHSDCD1: usbhsdcd::USBHSDCD1::instance(),
            USBHSDCD2: usbhsdcd::USBHSDCD2::instance(),
            USBNC1: usbnc::USBNC1::instance(),
            USBNC2: usbnc::USBNC2::instance(),
            USBPHY1: usbphy::USBPHY1::instance(),
            USBPHY2: usbphy::USBPHY2::instance(),
            USDHC1: usdhc::USDHC1::instance(),
            USDHC2: usdhc::USDHC2::instance(),
            VIDEO_MUX: video_mux::VIDEO_MUX::instance(),
            WDOG1: wdog::WDOG1::instance(),
            WDOG2: wdog::WDOG2::instance(),
            XBARA1: xbara1::XBARA1::instance(),
            XBARB2: xbarb::XBARB2::instance(),
            XBARB3: xbarb::XBARB3::instance(),
            XECC_FLEXSPI1: xecc_flexspi::XECC_FLEXSPI1::instance(),
            XECC_FLEXSPI2: xecc_flexspi::XECC_FLEXSPI2::instance(),
            XECC_SEMC0: xecc_flexspi::XECC_SEMC0::instance(),
            XRDC2_D0: xrdc2_d::XRDC2_D0::instance(),
            XRDC2_D1: xrdc2_d::XRDC2_D1::instance(),
        }
    }
}
