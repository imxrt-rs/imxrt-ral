#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Interrupt {
    #[doc = "0 - TMR1"]
    TMR1 = 0,
    #[doc = "4 - TMR5"]
    TMR5 = 4,
    #[doc = "5 - TMR6"]
    TMR6 = 5,
    #[doc = "6 - TMR7"]
    TMR7 = 6,
    #[doc = "7 - TMR8"]
    TMR8 = 7,
    #[doc = "8 - CAN1"]
    CAN1 = 8,
    #[doc = "9 - CAN1_ERROR"]
    CAN1_ERROR = 9,
    #[doc = "10 - GPIO1_0"]
    GPIO1_0 = 10,
    #[doc = "11 - GPIO1_1"]
    GPIO1_1 = 11,
    #[doc = "12 - I3C1"]
    I3C1 = 12,
    #[doc = "13 - LPI2C1"]
    LPI2C1 = 13,
    #[doc = "14 - LPI2C2"]
    LPI2C2 = 14,
    #[doc = "15 - LPIT1"]
    LPIT1 = 15,
    #[doc = "16 - LPSPI1"]
    LPSPI1 = 16,
    #[doc = "17 - LPSPI2"]
    LPSPI2 = 17,
    #[doc = "18 - LPTMR1"]
    LPTMR1 = 18,
    #[doc = "19 - LPUART1"]
    LPUART1 = 19,
    #[doc = "20 - LPUART2"]
    LPUART2 = 20,
    #[doc = "21 - MU1"]
    MU1 = 21,
    #[doc = "22 - MU2"]
    MU2 = 22,
    #[doc = "23 - PWM1_FAULT"]
    PWM1_FAULT = 23,
    #[doc = "24 - PWM1_0"]
    PWM1_0 = 24,
    #[doc = "25 - PWM1_1"]
    PWM1_1 = 25,
    #[doc = "26 - PWM1_2"]
    PWM1_2 = 26,
    #[doc = "27 - PWM1_3"]
    PWM1_3 = 27,
    #[doc = "36 - TPM1"]
    TPM1 = 36,
    #[doc = "37 - TPM2"]
    TPM2 = 37,
    #[doc = "38 - RTWDOG1"]
    RTWDOG1 = 38,
    #[doc = "39 - RTWDOG2"]
    RTWDOG2 = 39,
    #[doc = "40 - TRDC_MGR_AON"]
    TRDC_MGR_AON = 40,
    #[doc = "41 - PDM_HWVAD_EVENT"]
    PDM_HWVAD_EVENT = 41,
    #[doc = "42 - PDM_HWVAD_ERROR"]
    PDM_HWVAD_ERROR = 42,
    #[doc = "43 - PDM_EVENT"]
    PDM_EVENT = 43,
    #[doc = "44 - PDM_ERROR"]
    PDM_ERROR = 44,
    #[doc = "45 - SAI1"]
    SAI1 = 45,
    #[doc = "51 - CAN2"]
    CAN2 = 51,
    #[doc = "52 - CAN2_ERROR"]
    CAN2_ERROR = 52,
    #[doc = "53 - FLEXIO1"]
    FLEXIO1 = 53,
    #[doc = "54 - FLEXIO2"]
    FLEXIO2 = 54,
    #[doc = "55 - FLEXSPI1"]
    FLEXSPI1 = 55,
    #[doc = "56 - FLEXSPI2"]
    FLEXSPI2 = 56,
    #[doc = "57 - GPIO2_0"]
    GPIO2_0 = 57,
    #[doc = "58 - GPIO2_1"]
    GPIO2_1 = 58,
    #[doc = "59 - GPIO3_0"]
    GPIO3_0 = 59,
    #[doc = "60 - GPIO3_1"]
    GPIO3_1 = 60,
    #[doc = "61 - I3C2"]
    I3C2 = 61,
    #[doc = "62 - LPI2C3"]
    LPI2C3 = 62,
    #[doc = "63 - LPI2C4"]
    LPI2C4 = 63,
    #[doc = "64 - LPIT2"]
    LPIT2 = 64,
    #[doc = "65 - LPSPI3"]
    LPSPI3 = 65,
    #[doc = "66 - LPSPI4"]
    LPSPI4 = 66,
    #[doc = "67 - LPTMR2"]
    LPTMR2 = 67,
    #[doc = "68 - LPUART3"]
    LPUART3 = 68,
    #[doc = "69 - LPUART4"]
    LPUART4 = 69,
    #[doc = "70 - LPUART5"]
    LPUART5 = 70,
    #[doc = "71 - LPUART6"]
    LPUART6 = 71,
    #[doc = "73 - BBNSM"]
    BBNSM = 73,
    #[doc = "75 - TPM3"]
    TPM3 = 75,
    #[doc = "76 - TPM4"]
    TPM4 = 76,
    #[doc = "77 - TPM5"]
    TPM5 = 77,
    #[doc = "78 - TPM6"]
    TPM6 = 78,
    #[doc = "79 - RTWDOG3"]
    RTWDOG3 = 79,
    #[doc = "80 - RTWDOG4"]
    RTWDOG4 = 80,
    #[doc = "81 - RTWDOG5"]
    RTWDOG5 = 81,
    #[doc = "82 - TRDC_MGR_WKUP"]
    TRDC_MGR_WKUP = 82,
    #[doc = "86 - USDHC1"]
    USDHC1 = 86,
    #[doc = "87 - USDHC2"]
    USDHC2 = 87,
    #[doc = "88 - TRDC_MGR_MEGA"]
    TRDC_MGR_MEGA = 88,
    #[doc = "93 - ADC1"]
    ADC1 = 93,
    #[doc = "94 - DMA_ERROR"]
    DMA_ERROR = 94,
    #[doc = "95 - DMA3_CH0"]
    DMA3_CH0 = 95,
    #[doc = "96 - DMA3_CH1"]
    DMA3_CH1 = 96,
    #[doc = "97 - DMA3_CH2"]
    DMA3_CH2 = 97,
    #[doc = "98 - DMA3_CH3"]
    DMA3_CH3 = 98,
    #[doc = "99 - DMA3_CH4"]
    DMA3_CH4 = 99,
    #[doc = "100 - DMA3_CH5"]
    DMA3_CH5 = 100,
    #[doc = "101 - DMA3_CH6"]
    DMA3_CH6 = 101,
    #[doc = "102 - DMA3_CH7"]
    DMA3_CH7 = 102,
    #[doc = "103 - DMA3_CH8"]
    DMA3_CH8 = 103,
    #[doc = "104 - DMA3_CH9"]
    DMA3_CH9 = 104,
    #[doc = "105 - DMA3_CH10"]
    DMA3_CH10 = 105,
    #[doc = "106 - DMA3_CH11"]
    DMA3_CH11 = 106,
    #[doc = "107 - DMA3_CH12"]
    DMA3_CH12 = 107,
    #[doc = "108 - DMA3_CH13"]
    DMA3_CH13 = 108,
    #[doc = "109 - DMA3_CH14"]
    DMA3_CH14 = 109,
    #[doc = "110 - DMA3_CH15"]
    DMA3_CH15 = 110,
    #[doc = "111 - DMA3_CH16"]
    DMA3_CH16 = 111,
    #[doc = "112 - DMA3_CH17"]
    DMA3_CH17 = 112,
    #[doc = "113 - DMA3_CH18"]
    DMA3_CH18 = 113,
    #[doc = "114 - DMA3_CH19"]
    DMA3_CH19 = 114,
    #[doc = "115 - DMA3_CH20"]
    DMA3_CH20 = 115,
    #[doc = "116 - DMA3_CH21"]
    DMA3_CH21 = 116,
    #[doc = "117 - DMA3_CH22"]
    DMA3_CH22 = 117,
    #[doc = "118 - DMA3_CH23"]
    DMA3_CH23 = 118,
    #[doc = "119 - DMA3_CH24"]
    DMA3_CH24 = 119,
    #[doc = "120 - DMA3_CH25"]
    DMA3_CH25 = 120,
    #[doc = "121 - DMA3_CH26"]
    DMA3_CH26 = 121,
    #[doc = "122 - DMA3_CH27"]
    DMA3_CH27 = 122,
    #[doc = "123 - DMA3_CH28"]
    DMA3_CH28 = 123,
    #[doc = "124 - DMA3_CH29"]
    DMA3_CH29 = 124,
    #[doc = "125 - DMA3_CH30"]
    DMA3_CH30 = 125,
    #[doc = "126 - DMA3_CH31"]
    DMA3_CH31 = 126,
    #[doc = "127 - DMA4_ERROR"]
    DMA4_ERROR = 127,
    #[doc = "128 - DMA4_CH0_CH1_CH32_CH33"]
    DMA4_CH0_CH1_CH32_CH33 = 128,
    #[doc = "129 - DMA4_CH2_CH3_CH34_CH35"]
    DMA4_CH2_CH3_CH34_CH35 = 129,
    #[doc = "130 - DMA4_CH4_CH5_CH36_CH37"]
    DMA4_CH4_CH5_CH36_CH37 = 130,
    #[doc = "131 - DMA4_CH6_CH7_CH38_CH39"]
    DMA4_CH6_CH7_CH38_CH39 = 131,
    #[doc = "132 - DMA4_CH8_CH9_CH40_CH41"]
    DMA4_CH8_CH9_CH40_CH41 = 132,
    #[doc = "133 - DMA4_CH10_CH11_CH42_CH43"]
    DMA4_CH10_CH11_CH42_CH43 = 133,
    #[doc = "134 - DMA4_CH12_CH13_CH44_CH45"]
    DMA4_CH12_CH13_CH44_CH45 = 134,
    #[doc = "135 - DMA4_CH14_CH15_CH46_CH47"]
    DMA4_CH14_CH15_CH46_CH47 = 135,
    #[doc = "136 - DMA4_CH16_CH17_CH48_CH49"]
    DMA4_CH16_CH17_CH48_CH49 = 136,
    #[doc = "137 - DMA4_CH18_CH19_CH50_CH51"]
    DMA4_CH18_CH19_CH50_CH51 = 137,
    #[doc = "138 - DMA4_CH20_CH21_CH52_CH53"]
    DMA4_CH20_CH21_CH52_CH53 = 138,
    #[doc = "139 - DMA4_CH22_CH23_CH54_CH55"]
    DMA4_CH22_CH23_CH54_CH55 = 139,
    #[doc = "140 - DMA4_CH24_CH25_CH56_CH57"]
    DMA4_CH24_CH25_CH56_CH57 = 140,
    #[doc = "141 - DMA4_CH26_CH27_CH58_CH59"]
    DMA4_CH26_CH27_CH58_CH59 = 141,
    #[doc = "142 - DMA4_CH28_CH29_CH60_CH61"]
    DMA4_CH28_CH29_CH60_CH61 = 142,
    #[doc = "143 - DMA4_CH30_CH31_CH62_CH63"]
    DMA4_CH30_CH31_CH62_CH63 = 143,
    #[doc = "146 - SINC3_CH0_CH1_CH2_CH3"]
    SINC3_CH0_CH1_CH2_CH3 = 146,
    #[doc = "147 - EWM"]
    EWM = 147,
    #[doc = "148 - SEMC"]
    SEMC = 148,
    #[doc = "149 - LPIT3"]
    LPIT3 = 149,
    #[doc = "150 - LPTMR3"]
    LPTMR3 = 150,
    #[doc = "151 - TMR4"]
    TMR4 = 151,
    #[doc = "152 - LPI2C5"]
    LPI2C5 = 152,
    #[doc = "153 - LPI2C6"]
    LPI2C6 = 153,
    #[doc = "154 - SAI4"]
    SAI4 = 154,
    #[doc = "155 - SPDIF"]
    SPDIF = 155,
    #[doc = "156 - LPUART9"]
    LPUART9 = 156,
    #[doc = "157 - LPUART10"]
    LPUART10 = 157,
    #[doc = "158 - LPUART11"]
    LPUART11 = 158,
    #[doc = "159 - LPUART12"]
    LPUART12 = 159,
    #[doc = "164 - TMR3"]
    TMR3 = 164,
    #[doc = "170 - PWM2_FAULT"]
    PWM2_FAULT = 170,
    #[doc = "171 - PWM2_0"]
    PWM2_0 = 171,
    #[doc = "172 - PWM2_1"]
    PWM2_1 = 172,
    #[doc = "173 - PWM2_2"]
    PWM2_2 = 173,
    #[doc = "174 - PWM2_3"]
    PWM2_3 = 174,
    #[doc = "175 - PWM3_FAULT"]
    PWM3_FAULT = 175,
    #[doc = "176 - PWM3_0"]
    PWM3_0 = 176,
    #[doc = "177 - PWM3_1"]
    PWM3_1 = 177,
    #[doc = "178 - PWM3_2"]
    PWM3_2 = 178,
    #[doc = "179 - PWM3_3"]
    PWM3_3 = 179,
    #[doc = "180 - PWM4_FAULT"]
    PWM4_FAULT = 180,
    #[doc = "181 - PWM4_0"]
    PWM4_0 = 181,
    #[doc = "182 - PWM4_1"]
    PWM4_1 = 182,
    #[doc = "183 - PWM4_2"]
    PWM4_2 = 183,
    #[doc = "184 - PWM4_3"]
    PWM4_3 = 184,
    #[doc = "185 - EQDC1"]
    EQDC1 = 185,
    #[doc = "186 - EQDC2"]
    EQDC2 = 186,
    #[doc = "187 - EQDC3"]
    EQDC3 = 187,
    #[doc = "188 - EQDC4"]
    EQDC4 = 188,
    #[doc = "189 - ADC2"]
    ADC2 = 189,
    #[doc = "190 - DCDC"]
    DCDC = 190,
    #[doc = "191 - CAN3"]
    CAN3 = 191,
    #[doc = "192 - CAN3_ERROR"]
    CAN3_ERROR = 192,
    #[doc = "193 - DAC"]
    DAC = 193,
    #[doc = "194 - LPSPI5"]
    LPSPI5 = 194,
    #[doc = "195 - LPSPI6"]
    LPSPI6 = 195,
    #[doc = "196 - LPUART7"]
    LPUART7 = 196,
    #[doc = "197 - LPUART8"]
    LPUART8 = 197,
    #[doc = "198 - SAI2"]
    SAI2 = 198,
    #[doc = "199 - SAI3"]
    SAI3 = 199,
    #[doc = "200 - ACMP1"]
    ACMP1 = 200,
    #[doc = "201 - ACMP2"]
    ACMP2 = 201,
    #[doc = "202 - ACMP3"]
    ACMP3 = 202,
    #[doc = "203 - ACMP4"]
    ACMP4 = 203,
    #[doc = "209 - GPT1"]
    GPT1 = 209,
    #[doc = "210 - GPT2"]
    GPT2 = 210,
    #[doc = "211 - KPP"]
    KPP = 211,
    #[doc = "212 - USBPHY1"]
    USBPHY1 = 212,
    #[doc = "213 - USBPHY2"]
    USBPHY2 = 213,
    #[doc = "214 - USB_OTG2"]
    USB_OTG2 = 214,
    #[doc = "215 - USB_OTG1"]
    USB_OTG1 = 215,
    #[doc = "224 - SINC1_CH0"]
    SINC1_CH0 = 224,
    #[doc = "225 - SINC1_CH1"]
    SINC1_CH1 = 225,
    #[doc = "226 - SINC1_CH2"]
    SINC1_CH2 = 226,
    #[doc = "227 - SINC1_CH3"]
    SINC1_CH3 = 227,
    #[doc = "228 - SINC2_CH0"]
    SINC2_CH0 = 228,
    #[doc = "229 - SINC2_CH1"]
    SINC2_CH1 = 229,
    #[doc = "230 - SINC2_CH2"]
    SINC2_CH2 = 230,
    #[doc = "231 - SINC2_CH3"]
    SINC2_CH3 = 231,
    #[doc = "232 - GPIO4"]
    GPIO4 = 232,
    #[doc = "233 - TMR2"]
    TMR2 = 233,
    #[doc = "234 - GPIO5"]
    GPIO5 = 234,
    #[doc = "235 - ASRC"]
    ASRC = 235,
    #[doc = "236 - GPIO6"]
    GPIO6 = 236,
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
        fn TMR1();
        fn TMR5();
        fn TMR6();
        fn TMR7();
        fn TMR8();
        fn CAN1();
        fn CAN1_ERROR();
        fn GPIO1_0();
        fn GPIO1_1();
        fn I3C1();
        fn LPI2C1();
        fn LPI2C2();
        fn LPIT1();
        fn LPSPI1();
        fn LPSPI2();
        fn LPTMR1();
        fn LPUART1();
        fn LPUART2();
        fn MU1();
        fn MU2();
        fn PWM1_FAULT();
        fn PWM1_0();
        fn PWM1_1();
        fn PWM1_2();
        fn PWM1_3();
        fn TPM1();
        fn TPM2();
        fn RTWDOG1();
        fn RTWDOG2();
        fn TRDC_MGR_AON();
        fn PDM_HWVAD_EVENT();
        fn PDM_HWVAD_ERROR();
        fn PDM_EVENT();
        fn PDM_ERROR();
        fn SAI1();
        fn CAN2();
        fn CAN2_ERROR();
        fn FLEXIO1();
        fn FLEXIO2();
        fn FLEXSPI1();
        fn FLEXSPI2();
        fn GPIO2_0();
        fn GPIO2_1();
        fn GPIO3_0();
        fn GPIO3_1();
        fn I3C2();
        fn LPI2C3();
        fn LPI2C4();
        fn LPIT2();
        fn LPSPI3();
        fn LPSPI4();
        fn LPTMR2();
        fn LPUART3();
        fn LPUART4();
        fn LPUART5();
        fn LPUART6();
        fn BBNSM();
        fn TPM3();
        fn TPM4();
        fn TPM5();
        fn TPM6();
        fn RTWDOG3();
        fn RTWDOG4();
        fn RTWDOG5();
        fn TRDC_MGR_WKUP();
        fn USDHC1();
        fn USDHC2();
        fn TRDC_MGR_MEGA();
        fn ADC1();
        fn DMA_ERROR();
        fn DMA3_CH0();
        fn DMA3_CH1();
        fn DMA3_CH2();
        fn DMA3_CH3();
        fn DMA3_CH4();
        fn DMA3_CH5();
        fn DMA3_CH6();
        fn DMA3_CH7();
        fn DMA3_CH8();
        fn DMA3_CH9();
        fn DMA3_CH10();
        fn DMA3_CH11();
        fn DMA3_CH12();
        fn DMA3_CH13();
        fn DMA3_CH14();
        fn DMA3_CH15();
        fn DMA3_CH16();
        fn DMA3_CH17();
        fn DMA3_CH18();
        fn DMA3_CH19();
        fn DMA3_CH20();
        fn DMA3_CH21();
        fn DMA3_CH22();
        fn DMA3_CH23();
        fn DMA3_CH24();
        fn DMA3_CH25();
        fn DMA3_CH26();
        fn DMA3_CH27();
        fn DMA3_CH28();
        fn DMA3_CH29();
        fn DMA3_CH30();
        fn DMA3_CH31();
        fn DMA4_ERROR();
        fn DMA4_CH0_CH1_CH32_CH33();
        fn DMA4_CH2_CH3_CH34_CH35();
        fn DMA4_CH4_CH5_CH36_CH37();
        fn DMA4_CH6_CH7_CH38_CH39();
        fn DMA4_CH8_CH9_CH40_CH41();
        fn DMA4_CH10_CH11_CH42_CH43();
        fn DMA4_CH12_CH13_CH44_CH45();
        fn DMA4_CH14_CH15_CH46_CH47();
        fn DMA4_CH16_CH17_CH48_CH49();
        fn DMA4_CH18_CH19_CH50_CH51();
        fn DMA4_CH20_CH21_CH52_CH53();
        fn DMA4_CH22_CH23_CH54_CH55();
        fn DMA4_CH24_CH25_CH56_CH57();
        fn DMA4_CH26_CH27_CH58_CH59();
        fn DMA4_CH28_CH29_CH60_CH61();
        fn DMA4_CH30_CH31_CH62_CH63();
        fn SINC3_CH0_CH1_CH2_CH3();
        fn EWM();
        fn SEMC();
        fn LPIT3();
        fn LPTMR3();
        fn TMR4();
        fn LPI2C5();
        fn LPI2C6();
        fn SAI4();
        fn SPDIF();
        fn LPUART9();
        fn LPUART10();
        fn LPUART11();
        fn LPUART12();
        fn TMR3();
        fn PWM2_FAULT();
        fn PWM2_0();
        fn PWM2_1();
        fn PWM2_2();
        fn PWM2_3();
        fn PWM3_FAULT();
        fn PWM3_0();
        fn PWM3_1();
        fn PWM3_2();
        fn PWM3_3();
        fn PWM4_FAULT();
        fn PWM4_0();
        fn PWM4_1();
        fn PWM4_2();
        fn PWM4_3();
        fn EQDC1();
        fn EQDC2();
        fn EQDC3();
        fn EQDC4();
        fn ADC2();
        fn DCDC();
        fn CAN3();
        fn CAN3_ERROR();
        fn DAC();
        fn LPSPI5();
        fn LPSPI6();
        fn LPUART7();
        fn LPUART8();
        fn SAI2();
        fn SAI3();
        fn ACMP1();
        fn ACMP2();
        fn ACMP3();
        fn ACMP4();
        fn GPT1();
        fn GPT2();
        fn KPP();
        fn USBPHY1();
        fn USBPHY2();
        fn USB_OTG2();
        fn USB_OTG1();
        fn SINC1_CH0();
        fn SINC1_CH1();
        fn SINC1_CH2();
        fn SINC1_CH3();
        fn SINC2_CH0();
        fn SINC2_CH1();
        fn SINC2_CH2();
        fn SINC2_CH3();
        fn GPIO4();
        fn TMR2();
        fn GPIO5();
        fn ASRC();
        fn GPIO6();
    }
    pub union Vector {
        _handler: unsafe extern "C" fn(),
        _reserved: u32,
    }
    #[cfg_attr(target_os = "none", link_section = ".vector_table.interrupts")]
    #[no_mangle]
    pub static __INTERRUPTS: [Vector; 237] = [
        Vector { _handler: TMR1 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _handler: TMR5 },
        Vector { _handler: TMR6 },
        Vector { _handler: TMR7 },
        Vector { _handler: TMR8 },
        Vector { _handler: CAN1 },
        Vector {
            _handler: CAN1_ERROR,
        },
        Vector { _handler: GPIO1_0 },
        Vector { _handler: GPIO1_1 },
        Vector { _handler: I3C1 },
        Vector { _handler: LPI2C1 },
        Vector { _handler: LPI2C2 },
        Vector { _handler: LPIT1 },
        Vector { _handler: LPSPI1 },
        Vector { _handler: LPSPI2 },
        Vector { _handler: LPTMR1 },
        Vector { _handler: LPUART1 },
        Vector { _handler: LPUART2 },
        Vector { _handler: MU1 },
        Vector { _handler: MU2 },
        Vector {
            _handler: PWM1_FAULT,
        },
        Vector { _handler: PWM1_0 },
        Vector { _handler: PWM1_1 },
        Vector { _handler: PWM1_2 },
        Vector { _handler: PWM1_3 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _handler: TPM1 },
        Vector { _handler: TPM2 },
        Vector { _handler: RTWDOG1 },
        Vector { _handler: RTWDOG2 },
        Vector {
            _handler: TRDC_MGR_AON,
        },
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
        Vector { _handler: SAI1 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _handler: CAN2 },
        Vector {
            _handler: CAN2_ERROR,
        },
        Vector { _handler: FLEXIO1 },
        Vector { _handler: FLEXIO2 },
        Vector { _handler: FLEXSPI1 },
        Vector { _handler: FLEXSPI2 },
        Vector { _handler: GPIO2_0 },
        Vector { _handler: GPIO2_1 },
        Vector { _handler: GPIO3_0 },
        Vector { _handler: GPIO3_1 },
        Vector { _handler: I3C2 },
        Vector { _handler: LPI2C3 },
        Vector { _handler: LPI2C4 },
        Vector { _handler: LPIT2 },
        Vector { _handler: LPSPI3 },
        Vector { _handler: LPSPI4 },
        Vector { _handler: LPTMR2 },
        Vector { _handler: LPUART3 },
        Vector { _handler: LPUART4 },
        Vector { _handler: LPUART5 },
        Vector { _handler: LPUART6 },
        Vector { _reserved: 0 },
        Vector { _handler: BBNSM },
        Vector { _reserved: 0 },
        Vector { _handler: TPM3 },
        Vector { _handler: TPM4 },
        Vector { _handler: TPM5 },
        Vector { _handler: TPM6 },
        Vector { _handler: RTWDOG3 },
        Vector { _handler: RTWDOG4 },
        Vector { _handler: RTWDOG5 },
        Vector {
            _handler: TRDC_MGR_WKUP,
        },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _handler: USDHC1 },
        Vector { _handler: USDHC2 },
        Vector {
            _handler: TRDC_MGR_MEGA,
        },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _handler: ADC1 },
        Vector {
            _handler: DMA_ERROR,
        },
        Vector { _handler: DMA3_CH0 },
        Vector { _handler: DMA3_CH1 },
        Vector { _handler: DMA3_CH2 },
        Vector { _handler: DMA3_CH3 },
        Vector { _handler: DMA3_CH4 },
        Vector { _handler: DMA3_CH5 },
        Vector { _handler: DMA3_CH6 },
        Vector { _handler: DMA3_CH7 },
        Vector { _handler: DMA3_CH8 },
        Vector { _handler: DMA3_CH9 },
        Vector {
            _handler: DMA3_CH10,
        },
        Vector {
            _handler: DMA3_CH11,
        },
        Vector {
            _handler: DMA3_CH12,
        },
        Vector {
            _handler: DMA3_CH13,
        },
        Vector {
            _handler: DMA3_CH14,
        },
        Vector {
            _handler: DMA3_CH15,
        },
        Vector {
            _handler: DMA3_CH16,
        },
        Vector {
            _handler: DMA3_CH17,
        },
        Vector {
            _handler: DMA3_CH18,
        },
        Vector {
            _handler: DMA3_CH19,
        },
        Vector {
            _handler: DMA3_CH20,
        },
        Vector {
            _handler: DMA3_CH21,
        },
        Vector {
            _handler: DMA3_CH22,
        },
        Vector {
            _handler: DMA3_CH23,
        },
        Vector {
            _handler: DMA3_CH24,
        },
        Vector {
            _handler: DMA3_CH25,
        },
        Vector {
            _handler: DMA3_CH26,
        },
        Vector {
            _handler: DMA3_CH27,
        },
        Vector {
            _handler: DMA3_CH28,
        },
        Vector {
            _handler: DMA3_CH29,
        },
        Vector {
            _handler: DMA3_CH30,
        },
        Vector {
            _handler: DMA3_CH31,
        },
        Vector {
            _handler: DMA4_ERROR,
        },
        Vector {
            _handler: DMA4_CH0_CH1_CH32_CH33,
        },
        Vector {
            _handler: DMA4_CH2_CH3_CH34_CH35,
        },
        Vector {
            _handler: DMA4_CH4_CH5_CH36_CH37,
        },
        Vector {
            _handler: DMA4_CH6_CH7_CH38_CH39,
        },
        Vector {
            _handler: DMA4_CH8_CH9_CH40_CH41,
        },
        Vector {
            _handler: DMA4_CH10_CH11_CH42_CH43,
        },
        Vector {
            _handler: DMA4_CH12_CH13_CH44_CH45,
        },
        Vector {
            _handler: DMA4_CH14_CH15_CH46_CH47,
        },
        Vector {
            _handler: DMA4_CH16_CH17_CH48_CH49,
        },
        Vector {
            _handler: DMA4_CH18_CH19_CH50_CH51,
        },
        Vector {
            _handler: DMA4_CH20_CH21_CH52_CH53,
        },
        Vector {
            _handler: DMA4_CH22_CH23_CH54_CH55,
        },
        Vector {
            _handler: DMA4_CH24_CH25_CH56_CH57,
        },
        Vector {
            _handler: DMA4_CH26_CH27_CH58_CH59,
        },
        Vector {
            _handler: DMA4_CH28_CH29_CH60_CH61,
        },
        Vector {
            _handler: DMA4_CH30_CH31_CH62_CH63,
        },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector {
            _handler: SINC3_CH0_CH1_CH2_CH3,
        },
        Vector { _handler: EWM },
        Vector { _handler: SEMC },
        Vector { _handler: LPIT3 },
        Vector { _handler: LPTMR3 },
        Vector { _handler: TMR4 },
        Vector { _handler: LPI2C5 },
        Vector { _handler: LPI2C6 },
        Vector { _handler: SAI4 },
        Vector { _handler: SPDIF },
        Vector { _handler: LPUART9 },
        Vector { _handler: LPUART10 },
        Vector { _handler: LPUART11 },
        Vector { _handler: LPUART12 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _handler: TMR3 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector {
            _handler: PWM2_FAULT,
        },
        Vector { _handler: PWM2_0 },
        Vector { _handler: PWM2_1 },
        Vector { _handler: PWM2_2 },
        Vector { _handler: PWM2_3 },
        Vector {
            _handler: PWM3_FAULT,
        },
        Vector { _handler: PWM3_0 },
        Vector { _handler: PWM3_1 },
        Vector { _handler: PWM3_2 },
        Vector { _handler: PWM3_3 },
        Vector {
            _handler: PWM4_FAULT,
        },
        Vector { _handler: PWM4_0 },
        Vector { _handler: PWM4_1 },
        Vector { _handler: PWM4_2 },
        Vector { _handler: PWM4_3 },
        Vector { _handler: EQDC1 },
        Vector { _handler: EQDC2 },
        Vector { _handler: EQDC3 },
        Vector { _handler: EQDC4 },
        Vector { _handler: ADC2 },
        Vector { _handler: DCDC },
        Vector { _handler: CAN3 },
        Vector {
            _handler: CAN3_ERROR,
        },
        Vector { _handler: DAC },
        Vector { _handler: LPSPI5 },
        Vector { _handler: LPSPI6 },
        Vector { _handler: LPUART7 },
        Vector { _handler: LPUART8 },
        Vector { _handler: SAI2 },
        Vector { _handler: SAI3 },
        Vector { _handler: ACMP1 },
        Vector { _handler: ACMP2 },
        Vector { _handler: ACMP3 },
        Vector { _handler: ACMP4 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _handler: GPT1 },
        Vector { _handler: GPT2 },
        Vector { _handler: KPP },
        Vector { _handler: USBPHY1 },
        Vector { _handler: USBPHY2 },
        Vector { _handler: USB_OTG2 },
        Vector { _handler: USB_OTG1 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector {
            _handler: SINC1_CH0,
        },
        Vector {
            _handler: SINC1_CH1,
        },
        Vector {
            _handler: SINC1_CH2,
        },
        Vector {
            _handler: SINC1_CH3,
        },
        Vector {
            _handler: SINC2_CH0,
        },
        Vector {
            _handler: SINC2_CH1,
        },
        Vector {
            _handler: SINC2_CH2,
        },
        Vector {
            _handler: SINC2_CH3,
        },
        Vector { _handler: GPIO4 },
        Vector { _handler: TMR2 },
        Vector { _handler: GPIO5 },
        Vector { _handler: ASRC },
        Vector { _handler: GPIO6 },
    ];
}
#[path = "."]
pub mod adc {
    #[doc = "ADC"]
    pub const ADC1: *const RegisterBlock = 0x4260_0000 as *const RegisterBlock;
    #[doc = "ADC"]
    pub const ADC2: *const RegisterBlock = 0x42e0_0000 as *const RegisterBlock;
    #[path = "blocks/imxrt1189_cm33/adc.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type ADC1 = Instance<1>;
    impl crate::private::Sealed for ADC1 {}
    impl crate::Valid for ADC1 {}
    impl ADC1 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(ADC1)
        }
    }
    pub type ADC2 = Instance<2>;
    impl crate::private::Sealed for ADC2 {}
    impl crate::Valid for ADC2 {}
    impl ADC2 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(ADC2)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        [(ADC1, 1), (ADC2, 2)]
            .into_iter()
            .find(|(ptr, _)| core::ptr::eq(rb, *ptr))
            .map(|(_, inst)| inst)
    }
}
#[path = "."]
pub mod anadig_ldo_bbsm {
    #[doc = "RT1180_ANADIG_REGISTER"]
    pub const ANADIG_LDO_BBSM: *const RegisterBlock = 0x4448_0000 as *const RegisterBlock;
    #[path = "blocks/imxrt1189_cm33/anadig_ldo_bbsm.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type ANADIG_LDO_BBSM = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for ANADIG_LDO_BBSM {}
    impl crate::Valid for ANADIG_LDO_BBSM {}
    impl ANADIG_LDO_BBSM {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(ANADIG_LDO_BBSM)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, ANADIG_LDO_BBSM).then_some(0)
    }
}
#[path = "."]
pub mod anadig_misc {
    #[doc = "RT1180_ANADIG_REGISTER"]
    pub const ANADIG_MISC: *const RegisterBlock = 0x4448_0000 as *const RegisterBlock;
    #[path = "blocks/imxrt1189_cm33/anadig_misc.rs"]
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
    #[doc = "RT1180_ANADIG_REGISTER"]
    pub const ANADIG_OSC: *const RegisterBlock = 0x4448_0000 as *const RegisterBlock;
    #[path = "blocks/imxrt1189_cm33/anadig_osc.rs"]
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
    #[doc = "RT1180_ANADIG_REGISTER"]
    pub const ANADIG_PLL: *const RegisterBlock = 0x4448_0000 as *const RegisterBlock;
    #[path = "blocks/imxrt1189_cm33/anadig_pll.rs"]
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
    #[doc = "RT1180_ANADIG_REGISTER"]
    pub const ANADIG_PMU: *const RegisterBlock = 0x4448_0000 as *const RegisterBlock;
    #[path = "blocks/imxrt1189_cm33/anadig_pmu.rs"]
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
pub mod anadig_slots {
    #[doc = "IPS Domain"]
    pub const ANADIG_SLOTS: *const RegisterBlock = 0x4448_0000 as *const RegisterBlock;
    #[path = "blocks/imxrt1189_cm33/anadig_slots.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type ANADIG_SLOTS = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for ANADIG_SLOTS {}
    impl crate::Valid for ANADIG_SLOTS {}
    impl ANADIG_SLOTS {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(ANADIG_SLOTS)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, ANADIG_SLOTS).then_some(0)
    }
}
#[path = "."]
pub mod anadig_tempsensor {
    #[doc = "RT1180_ANADIG_REGISTER"]
    pub const ANADIG_TEMPSENSOR: *const RegisterBlock = 0x4448_0000 as *const RegisterBlock;
    #[path = "blocks/imxrt1189_cm33/anadig_tempsensor.rs"]
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
    pub const AOI1: *const RegisterBlock = 0x4278_0000 as *const RegisterBlock;
    #[doc = "AOI"]
    pub const AOI2: *const RegisterBlock = 0x4279_0000 as *const RegisterBlock;
    #[doc = "AOI"]
    pub const AOI3: *const RegisterBlock = 0x427e_0000 as *const RegisterBlock;
    #[doc = "AOI"]
    pub const AOI4: *const RegisterBlock = 0x427f_0000 as *const RegisterBlock;
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
    pub type AOI3 = Instance<3>;
    impl crate::private::Sealed for AOI3 {}
    impl crate::Valid for AOI3 {}
    impl AOI3 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(AOI3)
        }
    }
    pub type AOI4 = Instance<4>;
    impl crate::private::Sealed for AOI4 {}
    impl crate::Valid for AOI4 {}
    impl AOI4 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(AOI4)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        [(AOI1, 1), (AOI2, 2), (AOI3, 3), (AOI4, 4)]
            .into_iter()
            .find(|(ptr, _)| core::ptr::eq(rb, *ptr))
            .map(|(_, inst)| inst)
    }
}
#[path = "."]
pub mod asrc {
    #[doc = "ASRC"]
    pub const ASRC: *const RegisterBlock = 0x429a_0000 as *const RegisterBlock;
    #[path = "blocks/imxrt1189_cm33/asrc.rs"]
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
pub mod axbs {
    #[doc = "AXBS"]
    pub const AXBS: *const RegisterBlock = 0x4451_0000 as *const RegisterBlock;
    #[path = "blocks/imxrt1189_cm33/axbs.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type AXBS = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for AXBS {}
    impl crate::Valid for AXBS {}
    impl AXBS {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(AXBS)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, AXBS).then_some(0)
    }
}
#[path = "."]
pub mod bbnsm {
    #[doc = "BBNSM"]
    pub const BBNSM: *const RegisterBlock = 0x4444_0000 as *const RegisterBlock;
    #[path = "blocks/imxrt1189_cm33/bbnsm.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type BBNSM = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for BBNSM {}
    impl crate::Valid for BBNSM {}
    impl BBNSM {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(BBNSM)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, BBNSM).then_some(0)
    }
}
#[path = "."]
pub mod blk_ctrl_bbsmmix {
    #[doc = "blk_ctrl_bbsmmix"]
    pub const BLK_CTRL_BBSMMIX: *const RegisterBlock = 0x4441_0000 as *const RegisterBlock;
    #[path = "blocks/imxrt1189_cm33/blk_ctrl_bbsmmix.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type BLK_CTRL_BBSMMIX = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for BLK_CTRL_BBSMMIX {}
    impl crate::Valid for BLK_CTRL_BBSMMIX {}
    impl BLK_CTRL_BBSMMIX {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(BLK_CTRL_BBSMMIX)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, BLK_CTRL_BBSMMIX).then_some(0)
    }
}
#[path = "."]
pub mod blk_ctrl_ns_aonmix {
    #[doc = "Block Control Non-Secure AON Domain"]
    pub const BLK_CTRL_NS_AONMIX: *const RegisterBlock = 0x4421_0000 as *const RegisterBlock;
    #[path = "blocks/imxrt1189_cm33/blk_ctrl_ns_aonmix.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type BLK_CTRL_NS_AONMIX = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for BLK_CTRL_NS_AONMIX {}
    impl crate::Valid for BLK_CTRL_NS_AONMIX {}
    impl BLK_CTRL_NS_AONMIX {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(BLK_CTRL_NS_AONMIX)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, BLK_CTRL_NS_AONMIX).then_some(0)
    }
}
#[path = "."]
pub mod blk_ctrl_s_aonmix {
    #[doc = "Block Control Secure AONMIX"]
    pub const BLK_CTRL_S_AONMIX: *const RegisterBlock = 0x444f_0000 as *const RegisterBlock;
    #[path = "blocks/imxrt1189_cm33/blk_ctrl_s_aonmix.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type BLK_CTRL_S_AONMIX = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for BLK_CTRL_S_AONMIX {}
    impl crate::Valid for BLK_CTRL_S_AONMIX {}
    impl BLK_CTRL_S_AONMIX {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(BLK_CTRL_S_AONMIX)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, BLK_CTRL_S_AONMIX).then_some(0)
    }
}
#[path = "."]
pub mod blk_ctrl_wakeupmix {
    #[doc = "Block Control WAKEUP Domain"]
    pub const BLK_CTRL_WAKEUPMIX: *const RegisterBlock = 0x4242_0000 as *const RegisterBlock;
    #[path = "blocks/imxrt1189_cm33/blk_ctrl_wakeupmix.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type BLK_CTRL_WAKEUPMIX = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for BLK_CTRL_WAKEUPMIX {}
    impl crate::Valid for BLK_CTRL_WAKEUPMIX {}
    impl BLK_CTRL_WAKEUPMIX {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(BLK_CTRL_WAKEUPMIX)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, BLK_CTRL_WAKEUPMIX).then_some(0)
    }
}
#[path = "."]
pub mod can {
    #[doc = "CAN"]
    pub const CAN2: *const RegisterBlock = 0x425b_0000 as *const RegisterBlock;
    #[doc = "CAN"]
    pub const CAN1: *const RegisterBlock = 0x443a_0000 as *const RegisterBlock;
    #[doc = "CAN"]
    pub const CAN3: *const RegisterBlock = 0x445b_0000 as *const RegisterBlock;
    #[path = "blocks/imxrt1189_cm33/can.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
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
        [(CAN2, 2), (CAN1, 1), (CAN3, 3)]
            .into_iter()
            .find(|(ptr, _)| core::ptr::eq(rb, *ptr))
            .map(|(_, inst)| inst)
    }
}
#[path = "."]
pub mod ccm {
    #[doc = "CCM"]
    pub const CCM: *const RegisterBlock = 0x4445_0000 as *const RegisterBlock;
    #[path = "blocks/imxrt1189_cm33/ccm.rs"]
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
pub mod cm7platform_tcm {
    #[doc = "MEM Type I with PSW"]
    pub const CM7PLATFORM_TCM: *const RegisterBlock = 0x4446_4c00 as *const RegisterBlock;
    #[path = "blocks/imxrt1189_cm33/cm7platform_tcm.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type CM7PLATFORM_TCM = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for CM7PLATFORM_TCM {}
    impl crate::Valid for CM7PLATFORM_TCM {}
    impl CM7PLATFORM_TCM {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(CM7PLATFORM_TCM)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, CM7PLATFORM_TCM).then_some(0)
    }
}
#[path = "."]
pub mod cmp {
    #[doc = "ACMP"]
    pub const CMP1: *const RegisterBlock = 0x42dc_0000 as *const RegisterBlock;
    #[doc = "ACMP"]
    pub const CMP2: *const RegisterBlock = 0x42dd_0000 as *const RegisterBlock;
    #[doc = "ACMP"]
    pub const CMP3: *const RegisterBlock = 0x42de_0000 as *const RegisterBlock;
    #[doc = "ACMP"]
    pub const CMP4: *const RegisterBlock = 0x42df_0000 as *const RegisterBlock;
    #[path = "blocks/imxrt1189_cm33/cmp.rs"]
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
pub mod cp_cm33_imx9rtc__cm33_cache_ecc_mcm {
    #[doc = "CM33_CACHE_ECC_MCM"]
    pub const CP_CM33_IMX9RTC__CM33_CACHE_ECC_MCM: *const RegisterBlock =
        0x4440_1000 as *const RegisterBlock;
    #[path = "blocks/imxrt1189_cm33/cp_cm33_imx9rtc__cm33_cache_ecc_mcm.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type CP_CM33_IMX9RTC__CM33_CACHE_ECC_MCM = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for CP_CM33_IMX9RTC__CM33_CACHE_ECC_MCM {}
    impl crate::Valid for CP_CM33_IMX9RTC__CM33_CACHE_ECC_MCM {}
    impl CP_CM33_IMX9RTC__CM33_CACHE_ECC_MCM {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(CP_CM33_IMX9RTC__CM33_CACHE_ECC_MCM)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, CP_CM33_IMX9RTC__CM33_CACHE_ECC_MCM).then_some(0)
    }
}
#[path = "."]
pub mod cp_cm33_imx9rtc__cm33_tcm_mcm {
    #[doc = "CM33_TCM_MCM"]
    pub const CP_CM33_IMX9RTC__CM33_TCM_MCM: *const RegisterBlock =
        0x4442_0000 as *const RegisterBlock;
    #[path = "blocks/imxrt1189_cm33/cp_cm33_imx9rtc__cm33_tcm_mcm.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type CP_CM33_IMX9RTC__CM33_TCM_MCM = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for CP_CM33_IMX9RTC__CM33_TCM_MCM {}
    impl crate::Valid for CP_CM33_IMX9RTC__CM33_TCM_MCM {}
    impl CP_CM33_IMX9RTC__CM33_TCM_MCM {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(CP_CM33_IMX9RTC__CM33_TCM_MCM)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, CP_CM33_IMX9RTC__CM33_TCM_MCM).then_some(0)
    }
}
#[path = "."]
pub mod dac {
    #[doc = "DAC"]
    pub const DAC: *const RegisterBlock = 0x42e2_0000 as *const RegisterBlock;
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
    pub const DCDC: *const RegisterBlock = 0x4452_0000 as *const RegisterBlock;
    #[path = "blocks/imxrt1189_cm33/dcdc.rs"]
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
pub mod dma {
    #[doc = "DMA MP"]
    pub const DMA: *const RegisterBlock = 0x4200_0000 as *const RegisterBlock;
    #[path = "blocks/imxrt1189_cm33/dma.rs"]
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
pub mod dma3__tcd {
    #[doc = "DMA TCD"]
    pub const DMA3__TCD: *const RegisterBlock = 0x4401_0000 as *const RegisterBlock;
    #[path = "blocks/imxrt1189_cm33/dma3__tcd.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type DMA3__TCD = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for DMA3__TCD {}
    impl crate::Valid for DMA3__TCD {}
    impl DMA3__TCD {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(DMA3__TCD)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, DMA3__TCD).then_some(0)
    }
}
#[path = "."]
pub mod dma4__tcd {
    #[doc = "DMA TCD"]
    pub const DMA4__TCD: *const RegisterBlock = 0x4201_0000 as *const RegisterBlock;
    #[path = "blocks/imxrt1189_cm33/dma4__tcd.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type DMA4__TCD = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for DMA4__TCD {}
    impl crate::Valid for DMA4__TCD {}
    impl DMA4__TCD {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(DMA4__TCD)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, DMA4__TCD).then_some(0)
    }
}
#[path = "."]
pub mod ecat {
    #[doc = "ETHERCAT"]
    pub const ECAT: *const RegisterBlock = 0x42a8_0000 as *const RegisterBlock;
    #[path = "blocks/imxrt1189_cm33/ecat.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type ECAT = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for ECAT {}
    impl crate::Valid for ECAT {}
    impl ECAT {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(ECAT)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, ECAT).then_some(0)
    }
}
#[path = "."]
pub mod eim {
    #[doc = "EIM"]
    pub const EIM: *const RegisterBlock = 0x4b86_0000 as *const RegisterBlock;
    #[path = "blocks/imxrt1189_cm33/eim.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type EIM = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for EIM {}
    impl crate::Valid for EIM {}
    impl EIM {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(EIM)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, EIM).then_some(0)
    }
}
#[path = "."]
pub mod emdio_base {
    #[doc = "NETC EMDIO base function"]
    pub const EMDIO_BASE: *const RegisterBlock = 0x60ba_0000 as *const RegisterBlock;
    #[path = "blocks/imxrt1189_cm33/emdio_base.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type EMDIO_BASE = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for EMDIO_BASE {}
    impl crate::Valid for EMDIO_BASE {}
    impl EMDIO_BASE {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(EMDIO_BASE)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, EMDIO_BASE).then_some(0)
    }
}
#[path = "."]
pub mod emdio_global {
    #[doc = "NETC global"]
    pub const EMDIO_GLOBAL: *const RegisterBlock = 0x60bb_0000 as *const RegisterBlock;
    #[path = "blocks/imxrt1189_cm33/emdio_global.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type EMDIO_GLOBAL = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for EMDIO_GLOBAL {}
    impl crate::Valid for EMDIO_GLOBAL {}
    impl EMDIO_GLOBAL {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(EMDIO_GLOBAL)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, EMDIO_GLOBAL).then_some(0)
    }
}
#[path = "."]
pub mod enetc0_base {
    #[doc = "ENETC base"]
    pub const ENETC0_BASE: *const RegisterBlock = 0x60b1_0000 as *const RegisterBlock;
    #[path = "blocks/imxrt1189_cm33/enetc0_base.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type ENETC0_BASE = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for ENETC0_BASE {}
    impl crate::Valid for ENETC0_BASE {}
    impl ENETC0_BASE {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(ENETC0_BASE)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, ENETC0_BASE).then_some(0)
    }
}
#[path = "."]
pub mod enetc0_common {
    #[doc = "Switch and ENETC common base"]
    pub const ENETC0_COMMON: *const RegisterBlock = 0x60b1_0000 as *const RegisterBlock;
    #[path = "blocks/imxrt1189_cm33/enetc0_common.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type ENETC0_COMMON = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for ENETC0_COMMON {}
    impl crate::Valid for ENETC0_COMMON {}
    impl ENETC0_COMMON {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(ENETC0_COMMON)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, ENETC0_COMMON).then_some(0)
    }
}
#[path = "."]
pub mod enetc0_eth_mac_port {
    #[doc = "Ethernet MAC port"]
    pub const ENETC0_ETH_MAC_PORT: *const RegisterBlock = 0x60b1_5000 as *const RegisterBlock;
    #[path = "blocks/imxrt1189_cm33/enetc0_eth_mac_port.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type ENETC0_ETH_MAC_PORT = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for ENETC0_ETH_MAC_PORT {}
    impl crate::Valid for ENETC0_ETH_MAC_PORT {}
    impl ENETC0_ETH_MAC_PORT {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(ENETC0_ETH_MAC_PORT)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, ENETC0_ETH_MAC_PORT).then_some(0)
    }
}
#[path = "."]
pub mod enetc0_port {
    #[doc = "Port"]
    pub const ENETC0_PORT: *const RegisterBlock = 0x60b1_4000 as *const RegisterBlock;
    #[path = "blocks/imxrt1189_cm33/enetc0_port.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type ENETC0_PORT = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for ENETC0_PORT {}
    impl crate::Valid for ENETC0_PORT {}
    impl ENETC0_PORT {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(ENETC0_PORT)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, ENETC0_PORT).then_some(0)
    }
}
#[path = "."]
pub mod enetc0_si0 {
    #[doc = "ENETC Station Interface"]
    pub const ENETC0_SI0: *const RegisterBlock = 0x60b0_0000 as *const RegisterBlock;
    #[path = "blocks/imxrt1189_cm33/enetc0_si0.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type ENETC0_SI0 = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for ENETC0_SI0 {}
    impl crate::Valid for ENETC0_SI0 {}
    impl ENETC0_SI0 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(ENETC0_SI0)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, ENETC0_SI0).then_some(0)
    }
}
#[path = "."]
pub mod enetc1_base {
    #[doc = "ENETC base"]
    pub const ENETC1_BASE: *const RegisterBlock = 0x60b5_0000 as *const RegisterBlock;
    #[path = "blocks/imxrt1189_cm33/enetc1_base.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type ENETC1_BASE = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for ENETC1_BASE {}
    impl crate::Valid for ENETC1_BASE {}
    impl ENETC1_BASE {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(ENETC1_BASE)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, ENETC1_BASE).then_some(0)
    }
}
#[path = "."]
pub mod enetc1_common {
    #[doc = "Switch and ENETC common base"]
    pub const ENETC1_COMMON: *const RegisterBlock = 0x60b5_0000 as *const RegisterBlock;
    #[path = "blocks/imxrt1189_cm33/enetc1_common.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type ENETC1_COMMON = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for ENETC1_COMMON {}
    impl crate::Valid for ENETC1_COMMON {}
    impl ENETC1_COMMON {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(ENETC1_COMMON)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, ENETC1_COMMON).then_some(0)
    }
}
#[path = "."]
pub mod enetc1_port {
    #[doc = "Port"]
    pub const ENETC1_PORT: *const RegisterBlock = 0x60b5_4000 as *const RegisterBlock;
    #[path = "blocks/imxrt1189_cm33/enetc1_port.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type ENETC1_PORT = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for ENETC1_PORT {}
    impl crate::Valid for ENETC1_PORT {}
    impl ENETC1_PORT {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(ENETC1_PORT)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, ENETC1_PORT).then_some(0)
    }
}
#[path = "."]
pub mod enetc1_pseudo_mac_port {
    #[doc = "Pseudo MAC port"]
    pub const ENETC1_PSEUDO_MAC_PORT: *const RegisterBlock = 0x60b5_5000 as *const RegisterBlock;
    #[path = "blocks/imxrt1189_cm33/enetc1_pseudo_mac_port.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type ENETC1_PSEUDO_MAC_PORT = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for ENETC1_PSEUDO_MAC_PORT {}
    impl crate::Valid for ENETC1_PSEUDO_MAC_PORT {}
    impl ENETC1_PSEUDO_MAC_PORT {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(ENETC1_PSEUDO_MAC_PORT)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, ENETC1_PSEUDO_MAC_PORT).then_some(0)
    }
}
#[path = "."]
pub mod enetc1_si0 {
    #[doc = "ENETC Station Interface"]
    pub const ENETC1_SI0: *const RegisterBlock = 0x60b4_0000 as *const RegisterBlock;
    #[path = "blocks/imxrt1189_cm33/enetc1_si0.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type ENETC1_SI0 = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for ENETC1_SI0 {}
    impl crate::Valid for ENETC1_SI0 {}
    impl ENETC1_SI0 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(ENETC1_SI0)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, ENETC1_SI0).then_some(0)
    }
}
#[path = "."]
pub mod enetc1_si1 {
    #[doc = "ENETC Station Interface"]
    pub const ENETC1_SI1: *const RegisterBlock = 0x60c1_0000 as *const RegisterBlock;
    #[path = "blocks/imxrt1189_cm33/enetc1_si1.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type ENETC1_SI1 = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for ENETC1_SI1 {}
    impl crate::Valid for ENETC1_SI1 {}
    impl ENETC1_SI1 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(ENETC1_SI1)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, ENETC1_SI1).then_some(0)
    }
}
#[path = "."]
pub mod eqdc {
    #[doc = "Quadrature_Decoder"]
    pub const EQDC1: *const RegisterBlock = 0x4271_0000 as *const RegisterBlock;
    #[doc = "Quadrature_Decoder"]
    pub const EQDC2: *const RegisterBlock = 0x4272_0000 as *const RegisterBlock;
    #[doc = "Quadrature_Decoder"]
    pub const EQDC3: *const RegisterBlock = 0x4273_0000 as *const RegisterBlock;
    #[doc = "Quadrature_Decoder"]
    pub const EQDC4: *const RegisterBlock = 0x4274_0000 as *const RegisterBlock;
    #[path = "blocks/imxrt1189_cm33/eqdc.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type EQDC1 = Instance<1>;
    impl crate::private::Sealed for EQDC1 {}
    impl crate::Valid for EQDC1 {}
    impl EQDC1 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(EQDC1)
        }
    }
    pub type EQDC2 = Instance<2>;
    impl crate::private::Sealed for EQDC2 {}
    impl crate::Valid for EQDC2 {}
    impl EQDC2 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(EQDC2)
        }
    }
    pub type EQDC3 = Instance<3>;
    impl crate::private::Sealed for EQDC3 {}
    impl crate::Valid for EQDC3 {}
    impl EQDC3 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(EQDC3)
        }
    }
    pub type EQDC4 = Instance<4>;
    impl crate::private::Sealed for EQDC4 {}
    impl crate::Valid for EQDC4 {}
    impl EQDC4 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(EQDC4)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        [(EQDC1, 1), (EQDC2, 2), (EQDC3, 3), (EQDC4, 4)]
            .into_iter()
            .find(|(ptr, _)| core::ptr::eq(rb, *ptr))
            .map(|(_, inst)| inst)
    }
}
#[path = "."]
pub mod erm {
    #[doc = "ERM"]
    pub const ERM: *const RegisterBlock = 0x4b86_4000 as *const RegisterBlock;
    #[path = "blocks/imxrt1189_cm33/erm.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type ERM = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for ERM {}
    impl crate::Valid for ERM {}
    impl ERM {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(ERM)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, ERM).then_some(0)
    }
}
#[path = "."]
pub mod ewm {
    #[doc = "EWM"]
    pub const EWM: *const RegisterBlock = 0x427b_0000 as *const RegisterBlock;
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
    pub const FLEXIO1: *const RegisterBlock = 0x425c_0000 as *const RegisterBlock;
    #[doc = "FLEXIO"]
    pub const FLEXIO2: *const RegisterBlock = 0x425d_0000 as *const RegisterBlock;
    #[path = "blocks/imxrt1189_cm33/flexio.rs"]
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
pub mod flexspi {
    #[doc = "FlexSPI"]
    pub const FLEXSPI1: *const RegisterBlock = 0x425e_0000 as *const RegisterBlock;
    #[doc = "FlexSPI"]
    pub const FLEXSPI2: *const RegisterBlock = 0x445e_0000 as *const RegisterBlock;
    #[path = "blocks/imxrt1189_cm33/flexspi.rs"]
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
pub mod flexspi_slv {
    #[doc = "FlexSPI_FLR"]
    pub const FLEXSPI_SLV: *const RegisterBlock = 0x4290_0000 as *const RegisterBlock;
    #[path = "blocks/imxrt1189_cm33/flexspi_slv.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type FLEXSPI_SLV = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for FLEXSPI_SLV {}
    impl crate::Valid for FLEXSPI_SLV {}
    impl FLEXSPI_SLV {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(FLEXSPI_SLV)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, FLEXSPI_SLV).then_some(0)
    }
}
#[path = "."]
pub mod gpc_cpu_ctrl {
    #[doc = "no description available"]
    pub const GPC_CPU_CTRL: *const RegisterBlock = 0x4447_0000 as *const RegisterBlock;
    #[path = "blocks/imxrt1189_cm33/gpc_cpu_ctrl.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type GPC_CPU_CTRL = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for GPC_CPU_CTRL {}
    impl crate::Valid for GPC_CPU_CTRL {}
    impl GPC_CPU_CTRL {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(GPC_CPU_CTRL)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, GPC_CPU_CTRL).then_some(0)
    }
}
#[path = "."]
pub mod gpc_global {
    #[doc = "no description available"]
    pub const GPC_GLOBAL: *const RegisterBlock = 0x4447_2000 as *const RegisterBlock;
    #[path = "blocks/imxrt1189_cm33/gpc_global.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type GPC_GLOBAL = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for GPC_GLOBAL {}
    impl crate::Valid for GPC_GLOBAL {}
    impl GPC_GLOBAL {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(GPC_GLOBAL)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, GPC_GLOBAL).then_some(0)
    }
}
#[path = "."]
pub mod gpc_sys_sleep_ctrl {
    #[doc = "no description available"]
    pub const GPC_SYS_SLEEP_CTRL: *const RegisterBlock = 0x4447_2800 as *const RegisterBlock;
    #[path = "blocks/imxrt1189_cm33/gpc_sys_sleep_ctrl.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type GPC_SYS_SLEEP_CTRL = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for GPC_SYS_SLEEP_CTRL {}
    impl crate::Valid for GPC_SYS_SLEEP_CTRL {}
    impl GPC_SYS_SLEEP_CTRL {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(GPC_SYS_SLEEP_CTRL)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, GPC_SYS_SLEEP_CTRL).then_some(0)
    }
}
#[path = "."]
pub mod gpt {
    pub const GPT2: *const RegisterBlock = 0x42ec_0000 as *const RegisterBlock;
    #[doc = "GPT"]
    pub const GPT1: *const RegisterBlock = 0x446c_0000 as *const RegisterBlock;
    #[path = "blocks/imxrt1011/gpt.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
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
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        [(GPT2, 2), (GPT1, 1)]
            .into_iter()
            .find(|(ptr, _)| core::ptr::eq(rb, *ptr))
            .map(|(_, inst)| inst)
    }
}
#[path = "."]
pub mod i3c {
    #[doc = "I3C"]
    pub const I3C2: *const RegisterBlock = 0x4252_0000 as *const RegisterBlock;
    #[doc = "I3C"]
    pub const I3C1: *const RegisterBlock = 0x4433_0000 as *const RegisterBlock;
    #[path = "blocks/imxrt1189_cm33/i3c.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type I3C2 = Instance<2>;
    impl crate::private::Sealed for I3C2 {}
    impl crate::Valid for I3C2 {}
    impl I3C2 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(I3C2)
        }
    }
    pub type I3C1 = Instance<1>;
    impl crate::private::Sealed for I3C1 {}
    impl crate::Valid for I3C1 {}
    impl I3C1 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(I3C1)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        [(I3C2, 2), (I3C1, 1)]
            .into_iter()
            .find(|(ptr, _)| core::ptr::eq(rb, *ptr))
            .map(|(_, inst)| inst)
    }
}
#[path = "."]
pub mod iee {
    #[doc = "IEE"]
    pub const IEE: *const RegisterBlock = 0x42e4_0000 as *const RegisterBlock;
    #[path = "blocks/imxrt1189_cm33/iee.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type IEE = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for IEE {}
    impl crate::Valid for IEE {}
    impl IEE {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(IEE)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, IEE).then_some(0)
    }
}
#[path = "."]
pub mod iee_apc {
    #[doc = "IEE_APC"]
    pub const IEE_APC: *const RegisterBlock = 0x42e4_4000 as *const RegisterBlock;
    #[path = "blocks/imxrt1189_cm33/iee_apc.rs"]
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
pub mod ierc_f0_pci_hdr_type0 {
    #[doc = "PCI Express ECAM Event Collector config"]
    pub const IERC_F0_PCI_HDR_TYPE0: *const RegisterBlock = 0x600f_8000 as *const RegisterBlock;
    #[path = "blocks/imxrt1189_cm33/ierc_f0_pci_hdr_type0.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type IERC_F0_PCI_HDR_TYPE0 = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for IERC_F0_PCI_HDR_TYPE0 {}
    impl crate::Valid for IERC_F0_PCI_HDR_TYPE0 {}
    impl IERC_F0_PCI_HDR_TYPE0 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(IERC_F0_PCI_HDR_TYPE0)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, IERC_F0_PCI_HDR_TYPE0).then_some(0)
    }
}
#[path = "."]
pub mod ierc_ierb {
    #[doc = "Event Collector Integrated Endpoint Register Block"]
    pub const IERC_IERB: *const RegisterBlock = 0x6081_0000 as *const RegisterBlock;
    #[path = "blocks/imxrt1189_cm33/ierc_ierb.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type IERC_IERB = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for IERC_IERB {}
    impl crate::Valid for IERC_IERB {}
    impl IERC_IERB {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(IERC_IERB)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, IERC_IERB).then_some(0)
    }
}
#[path = "."]
pub mod iomuxc {
    #[doc = "IOMUXC"]
    pub const IOMUXC: *const RegisterBlock = 0x42a1_0000 as *const RegisterBlock;
    #[path = "blocks/imxrt1189_cm33/iomuxc.rs"]
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
pub mod iomuxc_aon {
    #[doc = "IOMUXC_AON"]
    pub const IOMUXC_AON: *const RegisterBlock = 0x443c_0000 as *const RegisterBlock;
    #[path = "blocks/imxrt1189_cm33/iomuxc_aon.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type IOMUXC_AON = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for IOMUXC_AON {}
    impl crate::Valid for IOMUXC_AON {}
    impl IOMUXC_AON {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(IOMUXC_AON)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, IOMUXC_AON).then_some(0)
    }
}
#[path = "."]
pub mod kpp {
    #[doc = "KPP"]
    pub const KPP: *const RegisterBlock = 0x42a0_0000 as *const RegisterBlock;
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
pub mod lpi2c {
    pub const LPI2C3: *const RegisterBlock = 0x4253_0000 as *const RegisterBlock;
    #[doc = "Low-Power Inter-Integrated Circuit"]
    pub const LPI2C4: *const RegisterBlock = 0x4254_0000 as *const RegisterBlock;
    #[doc = "Low-Power Inter-Integrated Circuit"]
    pub const LPI2C5: *const RegisterBlock = 0x42d3_0000 as *const RegisterBlock;
    #[doc = "Low-Power Inter-Integrated Circuit"]
    pub const LPI2C6: *const RegisterBlock = 0x42d4_0000 as *const RegisterBlock;
    #[doc = "Low-Power Inter-Integrated Circuit"]
    pub const LPI2C1: *const RegisterBlock = 0x4434_0000 as *const RegisterBlock;
    #[doc = "Low-Power Inter-Integrated Circuit"]
    pub const LPI2C2: *const RegisterBlock = 0x4435_0000 as *const RegisterBlock;
    #[path = "blocks/imxrt1011/lpi2c.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
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
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        [
            (LPI2C3, 3),
            (LPI2C4, 4),
            (LPI2C5, 5),
            (LPI2C6, 6),
            (LPI2C1, 1),
            (LPI2C2, 2),
        ]
        .into_iter()
        .find(|(ptr, _)| core::ptr::eq(rb, *ptr))
        .map(|(_, inst)| inst)
    }
}
#[path = "."]
pub mod lpit {
    #[doc = "LPIT"]
    pub const LPIT2: *const RegisterBlock = 0x424c_0000 as *const RegisterBlock;
    #[doc = "LPIT"]
    pub const LPIT3: *const RegisterBlock = 0x42cc_0000 as *const RegisterBlock;
    #[doc = "LPIT"]
    pub const LPIT1: *const RegisterBlock = 0x442f_0000 as *const RegisterBlock;
    #[path = "blocks/imxrt1189_cm33/lpit.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type LPIT2 = Instance<2>;
    impl crate::private::Sealed for LPIT2 {}
    impl crate::Valid for LPIT2 {}
    impl LPIT2 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(LPIT2)
        }
    }
    pub type LPIT3 = Instance<3>;
    impl crate::private::Sealed for LPIT3 {}
    impl crate::Valid for LPIT3 {}
    impl LPIT3 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(LPIT3)
        }
    }
    pub type LPIT1 = Instance<1>;
    impl crate::private::Sealed for LPIT1 {}
    impl crate::Valid for LPIT1 {}
    impl LPIT1 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(LPIT1)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        [(LPIT2, 2), (LPIT3, 3), (LPIT1, 1)]
            .into_iter()
            .find(|(ptr, _)| core::ptr::eq(rb, *ptr))
            .map(|(_, inst)| inst)
    }
}
#[path = "."]
pub mod lpspi {
    pub const LPSPI3: *const RegisterBlock = 0x4255_0000 as *const RegisterBlock;
    #[doc = "Low-Power Serial Peripheral Interface"]
    pub const LPSPI4: *const RegisterBlock = 0x4256_0000 as *const RegisterBlock;
    #[doc = "Low-Power Serial Peripheral Interface"]
    pub const LPSPI5: *const RegisterBlock = 0x42d5_0000 as *const RegisterBlock;
    #[doc = "Low-Power Serial Peripheral Interface"]
    pub const LPSPI6: *const RegisterBlock = 0x42d6_0000 as *const RegisterBlock;
    #[doc = "Low-Power Serial Peripheral Interface"]
    pub const LPSPI1: *const RegisterBlock = 0x4436_0000 as *const RegisterBlock;
    #[doc = "Low-Power Serial Peripheral Interface"]
    pub const LPSPI2: *const RegisterBlock = 0x4437_0000 as *const RegisterBlock;
    #[path = "blocks/imxrt1011/lpspi.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
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
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        [
            (LPSPI3, 3),
            (LPSPI4, 4),
            (LPSPI5, 5),
            (LPSPI6, 6),
            (LPSPI1, 1),
            (LPSPI2, 2),
        ]
        .into_iter()
        .find(|(ptr, _)| core::ptr::eq(rb, *ptr))
        .map(|(_, inst)| inst)
    }
}
#[path = "."]
pub mod lptmr {
    #[doc = "LPTMR"]
    pub const LPTMR2: *const RegisterBlock = 0x424d_0000 as *const RegisterBlock;
    #[doc = "LPTMR"]
    pub const LPTMR3: *const RegisterBlock = 0x42cd_0000 as *const RegisterBlock;
    #[doc = "LPTMR"]
    pub const LPTMR1: *const RegisterBlock = 0x4430_0000 as *const RegisterBlock;
    #[path = "blocks/imxrt1189_cm33/lptmr.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type LPTMR2 = Instance<2>;
    impl crate::private::Sealed for LPTMR2 {}
    impl crate::Valid for LPTMR2 {}
    impl LPTMR2 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(LPTMR2)
        }
    }
    pub type LPTMR3 = Instance<3>;
    impl crate::private::Sealed for LPTMR3 {}
    impl crate::Valid for LPTMR3 {}
    impl LPTMR3 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(LPTMR3)
        }
    }
    pub type LPTMR1 = Instance<1>;
    impl crate::private::Sealed for LPTMR1 {}
    impl crate::Valid for LPTMR1 {}
    impl LPTMR1 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(LPTMR1)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        [(LPTMR2, 2), (LPTMR3, 3), (LPTMR1, 1)]
            .into_iter()
            .find(|(ptr, _)| core::ptr::eq(rb, *ptr))
            .map(|(_, inst)| inst)
    }
}
#[path = "."]
pub mod lpuart {
    pub const LPUART3: *const RegisterBlock = 0x4257_0000 as *const RegisterBlock;
    #[doc = "LPUART"]
    pub const LPUART4: *const RegisterBlock = 0x4258_0000 as *const RegisterBlock;
    #[doc = "LPUART"]
    pub const LPUART5: *const RegisterBlock = 0x4259_0000 as *const RegisterBlock;
    #[doc = "LPUART"]
    pub const LPUART6: *const RegisterBlock = 0x425a_0000 as *const RegisterBlock;
    #[doc = "LPUART"]
    pub const LPUART9: *const RegisterBlock = 0x42d7_0000 as *const RegisterBlock;
    #[doc = "LPUART"]
    pub const LPUART10: *const RegisterBlock = 0x42d8_0000 as *const RegisterBlock;
    #[doc = "LPUART"]
    pub const LPUART11: *const RegisterBlock = 0x42d9_0000 as *const RegisterBlock;
    #[doc = "LPUART"]
    pub const LPUART8: *const RegisterBlock = 0x42da_0000 as *const RegisterBlock;
    #[doc = "LPUART"]
    pub const LPUART1: *const RegisterBlock = 0x4438_0000 as *const RegisterBlock;
    #[doc = "LPUART"]
    pub const LPUART2: *const RegisterBlock = 0x4439_0000 as *const RegisterBlock;
    #[doc = "LPUART"]
    pub const LPUART7: *const RegisterBlock = 0x4457_0000 as *const RegisterBlock;
    #[doc = "LPUART"]
    pub const LPUART12: *const RegisterBlock = 0x4458_0000 as *const RegisterBlock;
    #[path = "blocks/imxrt1011/lpuart.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
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
            (LPUART3, 3),
            (LPUART4, 4),
            (LPUART5, 5),
            (LPUART6, 6),
            (LPUART9, 9),
            (LPUART10, 10),
            (LPUART11, 11),
            (LPUART8, 8),
            (LPUART1, 1),
            (LPUART2, 2),
            (LPUART7, 7),
            (LPUART12, 12),
        ]
        .into_iter()
        .find(|(ptr, _)| core::ptr::eq(rb, *ptr))
        .map(|(_, inst)| inst)
    }
}
#[path = "."]
pub mod m7_mcm {
    #[doc = "CM7_MCM"]
    pub const M7_MCM: *const RegisterBlock = 0xe008_0000 as *const RegisterBlock;
    #[path = "blocks/imxrt1189_cm7/m7_mcm.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type M7_MCM = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for M7_MCM {}
    impl crate::Valid for M7_MCM {}
    impl M7_MCM {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(M7_MCM)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, M7_MCM).then_some(0)
    }
}
#[path = "."]
pub mod mecc {
    #[doc = "MECC64"]
    pub const MECC1: *const RegisterBlock = 0x4292_0000 as *const RegisterBlock;
    #[doc = "MECC64"]
    pub const MECC2: *const RegisterBlock = 0x4293_0000 as *const RegisterBlock;
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
pub mod msgintr {
    #[doc = "MSGINTR"]
    pub const MSGINTR1: *const RegisterBlock = 0x428a_0000 as *const RegisterBlock;
    #[doc = "MSGINTR"]
    pub const MSGINTR2: *const RegisterBlock = 0x428b_0000 as *const RegisterBlock;
    #[doc = "MSGINTR"]
    pub const MSGINTR3: *const RegisterBlock = 0x428c_0000 as *const RegisterBlock;
    #[doc = "MSGINTR"]
    pub const MSGINTR4: *const RegisterBlock = 0x428d_0000 as *const RegisterBlock;
    #[doc = "MSGINTR"]
    pub const MSGINTR5: *const RegisterBlock = 0x428e_0000 as *const RegisterBlock;
    #[doc = "MSGINTR"]
    pub const MSGINTR6: *const RegisterBlock = 0x428f_0000 as *const RegisterBlock;
    #[path = "blocks/imxrt1189_cm33/msgintr.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type MSGINTR1 = Instance<1>;
    impl crate::private::Sealed for MSGINTR1 {}
    impl crate::Valid for MSGINTR1 {}
    impl MSGINTR1 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(MSGINTR1)
        }
    }
    pub type MSGINTR2 = Instance<2>;
    impl crate::private::Sealed for MSGINTR2 {}
    impl crate::Valid for MSGINTR2 {}
    impl MSGINTR2 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(MSGINTR2)
        }
    }
    pub type MSGINTR3 = Instance<3>;
    impl crate::private::Sealed for MSGINTR3 {}
    impl crate::Valid for MSGINTR3 {}
    impl MSGINTR3 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(MSGINTR3)
        }
    }
    pub type MSGINTR4 = Instance<4>;
    impl crate::private::Sealed for MSGINTR4 {}
    impl crate::Valid for MSGINTR4 {}
    impl MSGINTR4 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(MSGINTR4)
        }
    }
    pub type MSGINTR5 = Instance<5>;
    impl crate::private::Sealed for MSGINTR5 {}
    impl crate::Valid for MSGINTR5 {}
    impl MSGINTR5 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(MSGINTR5)
        }
    }
    pub type MSGINTR6 = Instance<6>;
    impl crate::private::Sealed for MSGINTR6 {}
    impl crate::Valid for MSGINTR6 {}
    impl MSGINTR6 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(MSGINTR6)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        [
            (MSGINTR1, 1),
            (MSGINTR2, 2),
            (MSGINTR3, 3),
            (MSGINTR4, 4),
            (MSGINTR5, 5),
            (MSGINTR6, 6),
        ]
        .into_iter()
        .find(|(ptr, _)| core::ptr::eq(rb, *ptr))
        .map(|(_, inst)| inst)
    }
}
#[path = "."]
pub mod netc_f0_pci_hdr_type0 {
    #[doc = "NETC PCI Express ECAM PF config"]
    pub const NETC_F0_PCI_HDR_TYPE0: *const RegisterBlock = 0x6000_0000 as *const RegisterBlock;
    #[path = "blocks/imxrt1189_cm33/netc_f0_pci_hdr_type0.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type NETC_F0_PCI_HDR_TYPE0 = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for NETC_F0_PCI_HDR_TYPE0 {}
    impl crate::Valid for NETC_F0_PCI_HDR_TYPE0 {}
    impl NETC_F0_PCI_HDR_TYPE0 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(NETC_F0_PCI_HDR_TYPE0)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, NETC_F0_PCI_HDR_TYPE0).then_some(0)
    }
}
#[path = "."]
pub mod netc_f1_pci_hdr_type0 {
    #[doc = "NETC PCI Express ECAM PF config"]
    pub const NETC_F1_PCI_HDR_TYPE0: *const RegisterBlock = 0x6000_1000 as *const RegisterBlock;
    #[path = "blocks/imxrt1189_cm33/netc_f1_pci_hdr_type0.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type NETC_F1_PCI_HDR_TYPE0 = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for NETC_F1_PCI_HDR_TYPE0 {}
    impl crate::Valid for NETC_F1_PCI_HDR_TYPE0 {}
    impl NETC_F1_PCI_HDR_TYPE0 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(NETC_F1_PCI_HDR_TYPE0)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, NETC_F1_PCI_HDR_TYPE0).then_some(0)
    }
}
#[path = "."]
pub mod netc_f2_pci_hdr_type0 {
    #[doc = "NETC PCI Express ECAM PF config"]
    pub const NETC_F2_PCI_HDR_TYPE0: *const RegisterBlock = 0x6000_2000 as *const RegisterBlock;
    #[path = "blocks/imxrt1189_cm33/netc_f2_pci_hdr_type0.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type NETC_F2_PCI_HDR_TYPE0 = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for NETC_F2_PCI_HDR_TYPE0 {}
    impl crate::Valid for NETC_F2_PCI_HDR_TYPE0 {}
    impl NETC_F2_PCI_HDR_TYPE0 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(NETC_F2_PCI_HDR_TYPE0)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, NETC_F2_PCI_HDR_TYPE0).then_some(0)
    }
}
#[path = "."]
pub mod netc_f3_pci_hdr_type0 {
    #[doc = "NETC PCI Express ECAM PF config"]
    pub const NETC_F3_PCI_HDR_TYPE0: *const RegisterBlock = 0x6000_3000 as *const RegisterBlock;
    #[path = "blocks/imxrt1189_cm33/netc_f3_pci_hdr_type0.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type NETC_F3_PCI_HDR_TYPE0 = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for NETC_F3_PCI_HDR_TYPE0 {}
    impl crate::Valid for NETC_F3_PCI_HDR_TYPE0 {}
    impl NETC_F3_PCI_HDR_TYPE0 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(NETC_F3_PCI_HDR_TYPE0)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, NETC_F3_PCI_HDR_TYPE0).then_some(0)
    }
}
#[path = "."]
pub mod netc_f4_pci_hdr_type0 {
    #[doc = "NETC PCI Express ECAM PF config"]
    pub const NETC_F4_PCI_HDR_TYPE0: *const RegisterBlock = 0x6000_4000 as *const RegisterBlock;
    #[path = "blocks/imxrt1189_cm33/netc_f4_pci_hdr_type0.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type NETC_F4_PCI_HDR_TYPE0 = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for NETC_F4_PCI_HDR_TYPE0 {}
    impl crate::Valid for NETC_F4_PCI_HDR_TYPE0 {}
    impl NETC_F4_PCI_HDR_TYPE0 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(NETC_F4_PCI_HDR_TYPE0)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, NETC_F4_PCI_HDR_TYPE0).then_some(0)
    }
}
#[path = "."]
pub mod netc_ierb {
    #[doc = "NETC Integrated Endpoint Register Block"]
    pub const NETC_IERB: *const RegisterBlock = 0x6080_0000 as *const RegisterBlock;
    #[path = "blocks/imxrt1189_cm33/netc_ierb.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type NETC_IERB = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for NETC_IERB {}
    impl crate::Valid for NETC_IERB {}
    impl NETC_IERB {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(NETC_IERB)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, NETC_IERB).then_some(0)
    }
}
#[path = "."]
pub mod netc_priv {
    #[doc = "NETC privileged"]
    pub const NETC_PRIV: *const RegisterBlock = 0x6090_0000 as *const RegisterBlock;
    #[path = "blocks/imxrt1189_cm33/netc_priv.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type NETC_PRIV = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for NETC_PRIV {}
    impl crate::Valid for NETC_PRIV {}
    impl NETC_PRIV {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(NETC_PRIV)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, NETC_PRIV).then_some(0)
    }
}
#[path = "."]
pub mod netc_vf1_pci_hdr_type0 {
    #[doc = "NETC PCI Express ECAM VF config"]
    pub const NETC_VF1_PCI_HDR_TYPE0: *const RegisterBlock = 0x6010_0000 as *const RegisterBlock;
    #[path = "blocks/imxrt1189_cm33/netc_vf1_pci_hdr_type0.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type NETC_VF1_PCI_HDR_TYPE0 = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for NETC_VF1_PCI_HDR_TYPE0 {}
    impl crate::Valid for NETC_VF1_PCI_HDR_TYPE0 {}
    impl NETC_VF1_PCI_HDR_TYPE0 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(NETC_VF1_PCI_HDR_TYPE0)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, NETC_VF1_PCI_HDR_TYPE0).then_some(0)
    }
}
#[path = "."]
pub mod ocotp_fsb {
    #[doc = "no description available"]
    pub const OCOTP_FSB: *const RegisterBlock = 0x4751_8000 as *const RegisterBlock;
    #[path = "blocks/imxrt1189_cm33/ocotp_fsb.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type OCOTP_FSB = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for OCOTP_FSB {}
    impl crate::Valid for OCOTP_FSB {}
    impl OCOTP_FSB {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(OCOTP_FSB)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, OCOTP_FSB).then_some(0)
    }
}
#[path = "."]
pub mod osc_rc_400m {
    #[doc = "no description available"]
    pub const OSC_RC_400M: *const RegisterBlock = 0x4448_4380 as *const RegisterBlock;
    #[path = "blocks/imxrt1189_cm33/osc_rc_400m.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type OSC_RC_400M = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for OSC_RC_400M {}
    impl crate::Valid for OSC_RC_400M {}
    impl OSC_RC_400M {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(OSC_RC_400M)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, OSC_RC_400M).then_some(0)
    }
}
#[path = "."]
pub mod otfad {
    #[doc = "OTFAD"]
    pub const OTFAD1: *const RegisterBlock = 0x425e_0000 as *const RegisterBlock;
    #[doc = "OTFAD"]
    pub const OTFAD2: *const RegisterBlock = 0x445e_0000 as *const RegisterBlock;
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
    #[doc = "MICFIL"]
    pub const PDM: *const RegisterBlock = 0x42be_0000 as *const RegisterBlock;
    #[path = "blocks/imxrt1189_cm33/pdm.rs"]
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
pub mod phy_ldo {
    #[doc = "no description available"]
    pub const PHY_LDO: *const RegisterBlock = 0x4448_4680 as *const RegisterBlock;
    #[path = "blocks/imxrt1189_cm33/phy_ldo.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type PHY_LDO = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for PHY_LDO {}
    impl crate::Valid for PHY_LDO {}
    impl PHY_LDO {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(PHY_LDO)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, PHY_LDO).then_some(0)
    }
}
#[path = "."]
pub mod pwm {
    #[doc = "PWM"]
    pub const PWM1: *const RegisterBlock = 0x4265_0000 as *const RegisterBlock;
    #[doc = "PWM"]
    pub const PWM2: *const RegisterBlock = 0x4266_0000 as *const RegisterBlock;
    #[doc = "PWM"]
    pub const PWM3: *const RegisterBlock = 0x4267_0000 as *const RegisterBlock;
    #[doc = "PWM"]
    pub const PWM4: *const RegisterBlock = 0x4268_0000 as *const RegisterBlock;
    #[path = "blocks/imxrt1189_cm33/pwm.rs"]
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
pub mod rgpio {
    #[doc = "GPIO"]
    pub const RGPIO2: *const RegisterBlock = 0x4381_0000 as *const RegisterBlock;
    #[doc = "GPIO"]
    pub const RGPIO3: *const RegisterBlock = 0x4382_0000 as *const RegisterBlock;
    #[doc = "GPIO"]
    pub const RGPIO4: *const RegisterBlock = 0x4383_0000 as *const RegisterBlock;
    #[doc = "GPIO"]
    pub const RGPIO5: *const RegisterBlock = 0x4384_0000 as *const RegisterBlock;
    #[doc = "GPIO"]
    pub const RGPIO6: *const RegisterBlock = 0x4385_0000 as *const RegisterBlock;
    #[doc = "GPIO"]
    pub const RGPIO1: *const RegisterBlock = 0x4740_0000 as *const RegisterBlock;
    #[path = "blocks/imxrt1189_cm33/rgpio.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type RGPIO2 = Instance<2>;
    impl crate::private::Sealed for RGPIO2 {}
    impl crate::Valid for RGPIO2 {}
    impl RGPIO2 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(RGPIO2)
        }
    }
    pub type RGPIO3 = Instance<3>;
    impl crate::private::Sealed for RGPIO3 {}
    impl crate::Valid for RGPIO3 {}
    impl RGPIO3 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(RGPIO3)
        }
    }
    pub type RGPIO4 = Instance<4>;
    impl crate::private::Sealed for RGPIO4 {}
    impl crate::Valid for RGPIO4 {}
    impl RGPIO4 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(RGPIO4)
        }
    }
    pub type RGPIO5 = Instance<5>;
    impl crate::private::Sealed for RGPIO5 {}
    impl crate::Valid for RGPIO5 {}
    impl RGPIO5 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(RGPIO5)
        }
    }
    pub type RGPIO6 = Instance<6>;
    impl crate::private::Sealed for RGPIO6 {}
    impl crate::Valid for RGPIO6 {}
    impl RGPIO6 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(RGPIO6)
        }
    }
    pub type RGPIO1 = Instance<1>;
    impl crate::private::Sealed for RGPIO1 {}
    impl crate::Valid for RGPIO1 {}
    impl RGPIO1 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(RGPIO1)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        [
            (RGPIO2, 2),
            (RGPIO3, 3),
            (RGPIO4, 4),
            (RGPIO5, 5),
            (RGPIO6, 6),
            (RGPIO1, 1),
        ]
        .into_iter()
        .find(|(ptr, _)| core::ptr::eq(rb, *ptr))
        .map(|(_, inst)| inst)
    }
}
#[path = "."]
pub mod rtwdog {
    #[doc = "WDOG"]
    pub const RTWDOG3: *const RegisterBlock = 0x4249_0000 as *const RegisterBlock;
    #[doc = "WDOG"]
    pub const RTWDOG4: *const RegisterBlock = 0x424a_0000 as *const RegisterBlock;
    #[doc = "WDOG"]
    pub const RTWDOG5: *const RegisterBlock = 0x424b_0000 as *const RegisterBlock;
    #[doc = "WDOG"]
    pub const RTWDOG1: *const RegisterBlock = 0x442d_0000 as *const RegisterBlock;
    #[doc = "WDOG"]
    pub const RTWDOG2: *const RegisterBlock = 0x442e_0000 as *const RegisterBlock;
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
    pub type RTWDOG5 = Instance<5>;
    impl crate::private::Sealed for RTWDOG5 {}
    impl crate::Valid for RTWDOG5 {}
    impl RTWDOG5 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(RTWDOG5)
        }
    }
    pub type RTWDOG1 = Instance<1>;
    impl crate::private::Sealed for RTWDOG1 {}
    impl crate::Valid for RTWDOG1 {}
    impl RTWDOG1 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(RTWDOG1)
        }
    }
    pub type RTWDOG2 = Instance<2>;
    impl crate::private::Sealed for RTWDOG2 {}
    impl crate::Valid for RTWDOG2 {}
    impl RTWDOG2 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(RTWDOG2)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        [
            (RTWDOG3, 3),
            (RTWDOG4, 4),
            (RTWDOG5, 5),
            (RTWDOG1, 1),
            (RTWDOG2, 2),
        ]
        .into_iter()
        .find(|(ptr, _)| core::ptr::eq(rb, *ptr))
        .map(|(_, inst)| inst)
    }
}
#[path = "."]
pub mod sai {
    #[doc = "SAI"]
    pub const SAI2: *const RegisterBlock = 0x42bb_0000 as *const RegisterBlock;
    #[doc = "SAI"]
    pub const SAI3: *const RegisterBlock = 0x42bc_0000 as *const RegisterBlock;
    #[path = "blocks/imxrt1189_cm33/sai.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
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
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        [(SAI2, 2), (SAI3, 3)]
            .into_iter()
            .find(|(ptr, _)| core::ptr::eq(rb, *ptr))
            .map(|(_, inst)| inst)
    }
}
#[path = "."]
pub mod sai1 {
    #[doc = "SAI"]
    pub const SAI1: *const RegisterBlock = 0x443b_0000 as *const RegisterBlock;
    #[path = "blocks/imxrt1189_cm33/sai1.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type SAI1 = Instance<{ crate::SOLE_INSTANCE }>;
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
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, SAI1).then_some(0)
    }
}
#[path = "."]
pub mod sai4 {
    #[doc = "SAI"]
    pub const SAI4: *const RegisterBlock = 0x42bd_0000 as *const RegisterBlock;
    #[path = "blocks/imxrt1189_cm33/sai4.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type SAI4 = Instance<{ crate::SOLE_INSTANCE }>;
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
        core::ptr::eq(rb, SAI4).then_some(0)
    }
}
#[path = "."]
pub mod sema {
    #[doc = "SEMA42"]
    pub const SEMA2: *const RegisterBlock = 0x4245_0000 as *const RegisterBlock;
    #[doc = "SEMA42"]
    pub const SEMA1: *const RegisterBlock = 0x4426_0000 as *const RegisterBlock;
    #[path = "blocks/imxrt1189_cm33/sema.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type SEMA2 = Instance<2>;
    impl crate::private::Sealed for SEMA2 {}
    impl crate::Valid for SEMA2 {}
    impl SEMA2 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(SEMA2)
        }
    }
    pub type SEMA1 = Instance<1>;
    impl crate::private::Sealed for SEMA1 {}
    impl crate::Valid for SEMA1 {}
    impl SEMA1 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(SEMA1)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        [(SEMA2, 2), (SEMA1, 1)]
            .into_iter()
            .find(|(ptr, _)| core::ptr::eq(rb, *ptr))
            .map(|(_, inst)| inst)
    }
}
#[path = "."]
pub mod semc {
    #[doc = "SEMC"]
    pub const SEMC: *const RegisterBlock = 0x4291_0000 as *const RegisterBlock;
    #[path = "blocks/imxrt1189_cm33/semc.rs"]
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
pub mod sinc {
    #[doc = "SINC"]
    pub const SINC1: *const RegisterBlock = 0x42bf_0000 as *const RegisterBlock;
    #[doc = "SINC"]
    pub const SINC2: *const RegisterBlock = 0x42c0_0000 as *const RegisterBlock;
    #[doc = "SINC"]
    pub const SINC3: *const RegisterBlock = 0x42c1_0000 as *const RegisterBlock;
    #[path = "blocks/imxrt1189_cm33/sinc.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type SINC1 = Instance<1>;
    impl crate::private::Sealed for SINC1 {}
    impl crate::Valid for SINC1 {}
    impl SINC1 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(SINC1)
        }
    }
    pub type SINC2 = Instance<2>;
    impl crate::private::Sealed for SINC2 {}
    impl crate::Valid for SINC2 {}
    impl SINC2 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(SINC2)
        }
    }
    pub type SINC3 = Instance<3>;
    impl crate::private::Sealed for SINC3 {}
    impl crate::Valid for SINC3 {}
    impl SINC3 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(SINC3)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        [(SINC1, 1), (SINC2, 2), (SINC3, 3)]
            .into_iter()
            .find(|(ptr, _)| core::ptr::eq(rb, *ptr))
            .map(|(_, inst)| inst)
    }
}
#[path = "."]
pub mod spdif {
    #[doc = "SPDIF"]
    pub const SPDIF: *const RegisterBlock = 0x42ba_0000 as *const RegisterBlock;
    #[path = "blocks/imxrt1189_cm33/spdif.rs"]
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
pub mod src_general_reg {
    #[doc = "SRC General"]
    pub const SRC_GENERAL_REG: *const RegisterBlock = 0x4446_0000 as *const RegisterBlock;
    #[path = "blocks/imxrt1189_cm33/src_general_reg.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type SRC_GENERAL_REG = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for SRC_GENERAL_REG {}
    impl crate::Valid for SRC_GENERAL_REG {}
    impl SRC_GENERAL_REG {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(SRC_GENERAL_REG)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, SRC_GENERAL_REG).then_some(0)
    }
}
#[path = "."]
pub mod sw0_base {
    #[doc = "Switch base"]
    pub const SW0_BASE: *const RegisterBlock = 0x60a0_0000 as *const RegisterBlock;
    #[path = "blocks/imxrt1189_cm33/sw0_base.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type SW0_BASE = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for SW0_BASE {}
    impl crate::Valid for SW0_BASE {}
    impl SW0_BASE {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(SW0_BASE)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, SW0_BASE).then_some(0)
    }
}
#[path = "."]
pub mod sw0_common {
    #[doc = "Switch and ENETC common base"]
    pub const SW0_COMMON: *const RegisterBlock = 0x60a0_0000 as *const RegisterBlock;
    #[path = "blocks/imxrt1189_cm33/sw0_common.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type SW0_COMMON = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for SW0_COMMON {}
    impl crate::Valid for SW0_COMMON {}
    impl SW0_COMMON {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(SW0_COMMON)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, SW0_COMMON).then_some(0)
    }
}
#[path = "."]
pub mod sw0_eth_mac_port0 {
    #[doc = "Ethernet MAC port"]
    pub const SW0_ETH_MAC_PORT0: *const RegisterBlock = 0x60a0_5000 as *const RegisterBlock;
    #[path = "blocks/imxrt1189_cm33/sw0_eth_mac_port0.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type SW0_ETH_MAC_PORT0 = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for SW0_ETH_MAC_PORT0 {}
    impl crate::Valid for SW0_ETH_MAC_PORT0 {}
    impl SW0_ETH_MAC_PORT0 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(SW0_ETH_MAC_PORT0)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, SW0_ETH_MAC_PORT0).then_some(0)
    }
}
#[path = "."]
pub mod sw0_eth_mac_port1 {
    #[doc = "Ethernet MAC port"]
    pub const SW0_ETH_MAC_PORT1: *const RegisterBlock = 0x60a0_9000 as *const RegisterBlock;
    #[path = "blocks/imxrt1189_cm33/sw0_eth_mac_port1.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type SW0_ETH_MAC_PORT1 = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for SW0_ETH_MAC_PORT1 {}
    impl crate::Valid for SW0_ETH_MAC_PORT1 {}
    impl SW0_ETH_MAC_PORT1 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(SW0_ETH_MAC_PORT1)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, SW0_ETH_MAC_PORT1).then_some(0)
    }
}
#[path = "."]
pub mod sw0_eth_mac_port2 {
    #[doc = "Ethernet MAC port"]
    pub const SW0_ETH_MAC_PORT2: *const RegisterBlock = 0x60a0_d000 as *const RegisterBlock;
    #[path = "blocks/imxrt1189_cm33/sw0_eth_mac_port2.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type SW0_ETH_MAC_PORT2 = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for SW0_ETH_MAC_PORT2 {}
    impl crate::Valid for SW0_ETH_MAC_PORT2 {}
    impl SW0_ETH_MAC_PORT2 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(SW0_ETH_MAC_PORT2)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, SW0_ETH_MAC_PORT2).then_some(0)
    }
}
#[path = "."]
pub mod sw0_eth_mac_port3 {
    #[doc = "Ethernet MAC port"]
    pub const SW0_ETH_MAC_PORT3: *const RegisterBlock = 0x60a1_1000 as *const RegisterBlock;
    #[path = "blocks/imxrt1189_cm33/sw0_eth_mac_port3.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type SW0_ETH_MAC_PORT3 = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for SW0_ETH_MAC_PORT3 {}
    impl crate::Valid for SW0_ETH_MAC_PORT3 {}
    impl SW0_ETH_MAC_PORT3 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(SW0_ETH_MAC_PORT3)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, SW0_ETH_MAC_PORT3).then_some(0)
    }
}
#[path = "."]
pub mod sw0_port {
    #[doc = "Port"]
    pub const SW0_PORT0: *const RegisterBlock = 0x60a0_4000 as *const RegisterBlock;
    #[doc = "Port"]
    pub const SW0_PORT1: *const RegisterBlock = 0x60a0_8000 as *const RegisterBlock;
    #[doc = "Port"]
    pub const SW0_PORT2: *const RegisterBlock = 0x60a0_c000 as *const RegisterBlock;
    #[doc = "Port"]
    pub const SW0_PORT3: *const RegisterBlock = 0x60a1_0000 as *const RegisterBlock;
    #[path = "blocks/imxrt1189_cm33/sw0_port.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type SW0_PORT0 = Instance<0>;
    impl crate::private::Sealed for SW0_PORT0 {}
    impl crate::Valid for SW0_PORT0 {}
    impl SW0_PORT0 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(SW0_PORT0)
        }
    }
    pub type SW0_PORT1 = Instance<1>;
    impl crate::private::Sealed for SW0_PORT1 {}
    impl crate::Valid for SW0_PORT1 {}
    impl SW0_PORT1 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(SW0_PORT1)
        }
    }
    pub type SW0_PORT2 = Instance<2>;
    impl crate::private::Sealed for SW0_PORT2 {}
    impl crate::Valid for SW0_PORT2 {}
    impl SW0_PORT2 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(SW0_PORT2)
        }
    }
    pub type SW0_PORT3 = Instance<3>;
    impl crate::private::Sealed for SW0_PORT3 {}
    impl crate::Valid for SW0_PORT3 {}
    impl SW0_PORT3 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(SW0_PORT3)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        [
            (SW0_PORT0, 0),
            (SW0_PORT1, 1),
            (SW0_PORT2, 2),
            (SW0_PORT3, 3),
        ]
        .into_iter()
        .find(|(ptr, _)| core::ptr::eq(rb, *ptr))
        .map(|(_, inst)| inst)
    }
}
#[path = "."]
pub mod sw0_port4 {
    #[doc = "Port"]
    pub const SW0_PORT4: *const RegisterBlock = 0x60a1_4000 as *const RegisterBlock;
    #[path = "blocks/imxrt1189_cm33/sw0_port4.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type SW0_PORT4 = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for SW0_PORT4 {}
    impl crate::Valid for SW0_PORT4 {}
    impl SW0_PORT4 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(SW0_PORT4)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, SW0_PORT4).then_some(0)
    }
}
#[path = "."]
pub mod sw0_pseudo_mac_port4 {
    #[doc = "Pseudo MAC port"]
    pub const SW0_PSEUDO_MAC_PORT4: *const RegisterBlock = 0x60a1_5000 as *const RegisterBlock;
    #[path = "blocks/imxrt1189_cm33/sw0_pseudo_mac_port4.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type SW0_PSEUDO_MAC_PORT4 = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for SW0_PSEUDO_MAC_PORT4 {}
    impl crate::Valid for SW0_PSEUDO_MAC_PORT4 {}
    impl SW0_PSEUDO_MAC_PORT4 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(SW0_PSEUDO_MAC_PORT4)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, SW0_PSEUDO_MAC_PORT4).then_some(0)
    }
}
#[path = "."]
pub mod sys_ctr_compare {
    #[doc = "SYS_CTR_COMPARE"]
    pub const SYS_CTR_COMPARE: *const RegisterBlock = 0x442a_0000 as *const RegisterBlock;
    #[path = "blocks/imxrt1189_cm33/sys_ctr_compare.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type SYS_CTR_COMPARE = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for SYS_CTR_COMPARE {}
    impl crate::Valid for SYS_CTR_COMPARE {}
    impl SYS_CTR_COMPARE {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(SYS_CTR_COMPARE)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, SYS_CTR_COMPARE).then_some(0)
    }
}
#[path = "."]
pub mod sys_ctr_control {
    #[doc = "SYS_CTR_CONTROL"]
    pub const SYS_CTR_CONTROL: *const RegisterBlock = 0x4429_0000 as *const RegisterBlock;
    #[path = "blocks/imxrt1189_cm33/sys_ctr_control.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type SYS_CTR_CONTROL = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for SYS_CTR_CONTROL {}
    impl crate::Valid for SYS_CTR_CONTROL {}
    impl SYS_CTR_CONTROL {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(SYS_CTR_CONTROL)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, SYS_CTR_CONTROL).then_some(0)
    }
}
#[path = "."]
pub mod sys_ctr_read {
    #[doc = "SYS_CTR_READ"]
    pub const SYS_CTR_READ: *const RegisterBlock = 0x442b_0000 as *const RegisterBlock;
    #[path = "blocks/imxrt1189_cm33/sys_ctr_read.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type SYS_CTR_READ = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for SYS_CTR_READ {}
    impl crate::Valid for SYS_CTR_READ {}
    impl SYS_CTR_READ {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(SYS_CTR_READ)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, SYS_CTR_READ).then_some(0)
    }
}
#[path = "."]
pub mod sys_tick1 {
    #[doc = "M7 Systick module"]
    pub const SYSTICK1: *const RegisterBlock = 0xe000_e010 as *const RegisterBlock;
    #[path = "blocks/imxrt1189_cm7/sys_tick1.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type SYSTICK1 = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for SYSTICK1 {}
    impl crate::Valid for SYSTICK1 {}
    impl SYSTICK1 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(SYSTICK1)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, SYSTICK1).then_some(0)
    }
}
#[path = "."]
pub mod tmpsns {
    #[doc = "TMPSNS"]
    pub const TMPSNS: *const RegisterBlock = 0x4448_4580 as *const RegisterBlock;
    #[path = "blocks/imxrt1189_cm33/tmpsns.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type TMPSNS = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for TMPSNS {}
    impl crate::Valid for TMPSNS {}
    impl TMPSNS {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(TMPSNS)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, TMPSNS).then_some(0)
    }
}
#[path = "."]
pub mod tmr {
    #[doc = "TMR"]
    pub const TMR1: *const RegisterBlock = 0x4269_0000 as *const RegisterBlock;
    #[doc = "TMR"]
    pub const TMR2: *const RegisterBlock = 0x426a_0000 as *const RegisterBlock;
    #[doc = "TMR"]
    pub const TMR3: *const RegisterBlock = 0x426b_0000 as *const RegisterBlock;
    #[doc = "TMR"]
    pub const TMR4: *const RegisterBlock = 0x426c_0000 as *const RegisterBlock;
    #[doc = "TMR"]
    pub const TMR5: *const RegisterBlock = 0x426d_0000 as *const RegisterBlock;
    #[doc = "TMR"]
    pub const TMR6: *const RegisterBlock = 0x426e_0000 as *const RegisterBlock;
    #[doc = "TMR"]
    pub const TMR7: *const RegisterBlock = 0x426f_0000 as *const RegisterBlock;
    #[doc = "TMR"]
    pub const TMR8: *const RegisterBlock = 0x4270_0000 as *const RegisterBlock;
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
    pub type TMR5 = Instance<5>;
    impl crate::private::Sealed for TMR5 {}
    impl crate::Valid for TMR5 {}
    impl TMR5 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(TMR5)
        }
    }
    pub type TMR6 = Instance<6>;
    impl crate::private::Sealed for TMR6 {}
    impl crate::Valid for TMR6 {}
    impl TMR6 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(TMR6)
        }
    }
    pub type TMR7 = Instance<7>;
    impl crate::private::Sealed for TMR7 {}
    impl crate::Valid for TMR7 {}
    impl TMR7 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(TMR7)
        }
    }
    pub type TMR8 = Instance<8>;
    impl crate::private::Sealed for TMR8 {}
    impl crate::Valid for TMR8 {}
    impl TMR8 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(TMR8)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        [
            (TMR1, 1),
            (TMR2, 2),
            (TMR3, 3),
            (TMR4, 4),
            (TMR5, 5),
            (TMR6, 6),
            (TMR7, 7),
            (TMR8, 8),
        ]
        .into_iter()
        .find(|(ptr, _)| core::ptr::eq(rb, *ptr))
        .map(|(_, inst)| inst)
    }
}
#[path = "."]
pub mod tmr0_base {
    #[doc = "Timer"]
    pub const TMR0_BASE: *const RegisterBlock = 0x60b8_0000 as *const RegisterBlock;
    #[path = "blocks/imxrt1189_cm33/tmr0_base.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type TMR0_BASE = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for TMR0_BASE {}
    impl crate::Valid for TMR0_BASE {}
    impl TMR0_BASE {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(TMR0_BASE)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, TMR0_BASE).then_some(0)
    }
}
#[path = "."]
pub mod tmr0_global {
    #[doc = "NETC global"]
    pub const TMR0_GLOBAL: *const RegisterBlock = 0x60b9_0000 as *const RegisterBlock;
    #[path = "blocks/imxrt1189_cm33/tmr0_global.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type TMR0_GLOBAL = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for TMR0_GLOBAL {}
    impl crate::Valid for TMR0_GLOBAL {}
    impl TMR0_GLOBAL {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(TMR0_GLOBAL)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, TMR0_GLOBAL).then_some(0)
    }
}
#[path = "."]
pub mod tpm {
    #[doc = "TPM"]
    pub const TPM3: *const RegisterBlock = 0x424e_0000 as *const RegisterBlock;
    #[doc = "TPM"]
    pub const TPM4: *const RegisterBlock = 0x424f_0000 as *const RegisterBlock;
    #[doc = "TPM"]
    pub const TPM5: *const RegisterBlock = 0x4250_0000 as *const RegisterBlock;
    #[doc = "TPM"]
    pub const TPM6: *const RegisterBlock = 0x4251_0000 as *const RegisterBlock;
    #[doc = "TPM"]
    pub const TPM1: *const RegisterBlock = 0x4431_0000 as *const RegisterBlock;
    #[doc = "TPM"]
    pub const TPM2: *const RegisterBlock = 0x4432_0000 as *const RegisterBlock;
    #[path = "blocks/imxrt1189_cm33/tpm.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type TPM3 = Instance<3>;
    impl crate::private::Sealed for TPM3 {}
    impl crate::Valid for TPM3 {}
    impl TPM3 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(TPM3)
        }
    }
    pub type TPM4 = Instance<4>;
    impl crate::private::Sealed for TPM4 {}
    impl crate::Valid for TPM4 {}
    impl TPM4 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(TPM4)
        }
    }
    pub type TPM5 = Instance<5>;
    impl crate::private::Sealed for TPM5 {}
    impl crate::Valid for TPM5 {}
    impl TPM5 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(TPM5)
        }
    }
    pub type TPM6 = Instance<6>;
    impl crate::private::Sealed for TPM6 {}
    impl crate::Valid for TPM6 {}
    impl TPM6 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(TPM6)
        }
    }
    pub type TPM1 = Instance<1>;
    impl crate::private::Sealed for TPM1 {}
    impl crate::Valid for TPM1 {}
    impl TPM1 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(TPM1)
        }
    }
    pub type TPM2 = Instance<2>;
    impl crate::private::Sealed for TPM2 {}
    impl crate::Valid for TPM2 {}
    impl TPM2 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(TPM2)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        [
            (TPM3, 3),
            (TPM4, 4),
            (TPM5, 5),
            (TPM6, 6),
            (TPM1, 1),
            (TPM2, 2),
        ]
        .into_iter()
        .find(|(ptr, _)| core::ptr::eq(rb, *ptr))
        .map(|(_, inst)| inst)
    }
}
#[path = "."]
pub mod trdc1 {
    #[doc = "TRDC"]
    pub const TRDC1: *const RegisterBlock = 0x4427_0000 as *const RegisterBlock;
    #[path = "blocks/imxrt1189_cm33/trdc1.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type TRDC1 = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for TRDC1 {}
    impl crate::Valid for TRDC1 {}
    impl TRDC1 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(TRDC1)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, TRDC1).then_some(0)
    }
}
#[path = "."]
pub mod trdc2 {
    #[doc = "TRDC"]
    pub const TRDC2: *const RegisterBlock = 0x4246_0000 as *const RegisterBlock;
    #[path = "blocks/imxrt1189_cm33/trdc2.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type TRDC2 = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for TRDC2 {}
    impl crate::Valid for TRDC2 {}
    impl TRDC2 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(TRDC2)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, TRDC2).then_some(0)
    }
}
#[path = "."]
pub mod trdc3 {
    #[doc = "TRDC"]
    pub const TRDC3: *const RegisterBlock = 0x4281_0000 as *const RegisterBlock;
    #[path = "blocks/imxrt1189_cm33/trdc3.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type TRDC3 = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for TRDC3 {}
    impl crate::Valid for TRDC3 {}
    impl TRDC3 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(TRDC3)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, TRDC3).then_some(0)
    }
}
#[path = "."]
pub mod usb {
    #[doc = "USBC"]
    pub const USB1: *const RegisterBlock = 0x42c8_0000 as *const RegisterBlock;
    pub const USB2: *const RegisterBlock = 0x42c9_0000 as *const RegisterBlock;
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
    pub const USBHSDCD1: *const RegisterBlock = 0x42ca_0800 as *const RegisterBlock;
    #[doc = "USBDCD"]
    pub const USBHSDCD2: *const RegisterBlock = 0x42cb_0800 as *const RegisterBlock;
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
    pub const USBNC1: *const RegisterBlock = 0x42c8_0200 as *const RegisterBlock;
    pub const USBNC2: *const RegisterBlock = 0x42c9_0200 as *const RegisterBlock;
    #[path = "blocks/imxrt1189_cm33/usbnc.rs"]
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
    pub const USBPHY1: *const RegisterBlock = 0x42ca_0000 as *const RegisterBlock;
    #[doc = "USBPHY"]
    pub const USBPHY2: *const RegisterBlock = 0x42cb_0000 as *const RegisterBlock;
    #[path = "blocks/imxrt1189_cm33/usbphy.rs"]
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
    pub const USDHC1: *const RegisterBlock = 0x4285_0000 as *const RegisterBlock;
    #[doc = "uSDHC"]
    pub const USDHC2: *const RegisterBlock = 0x4286_0000 as *const RegisterBlock;
    #[path = "blocks/imxrt1189_cm33/usdhc.rs"]
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
pub mod vmbandgap {
    #[doc = "no description available"]
    pub const VMBANDGAP: *const RegisterBlock = 0x4448_4780 as *const RegisterBlock;
    #[path = "blocks/imxrt1176_cm4/vmbandgap.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type VMBANDGAP = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for VMBANDGAP {}
    impl crate::Valid for VMBANDGAP {}
    impl VMBANDGAP {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(VMBANDGAP)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, VMBANDGAP).then_some(0)
    }
}
#[path = "."]
pub mod vref {
    #[doc = "VREF"]
    pub const VREF: *const RegisterBlock = 0x42e3_0000 as *const RegisterBlock;
    #[path = "blocks/imxrt1189_cm33/vref.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type VREF = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for VREF {}
    impl crate::Valid for VREF {}
    impl VREF {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(VREF)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, VREF).then_some(0)
    }
}
#[path = "."]
pub mod xbar {
    #[doc = "XBAR"]
    pub const XBAR2: *const RegisterBlock = 0x4276_0000 as *const RegisterBlock;
    #[doc = "XBAR"]
    pub const XBAR3: *const RegisterBlock = 0x4277_0000 as *const RegisterBlock;
    #[path = "blocks/imxrt1189_cm33/xbar.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type XBAR2 = Instance<2>;
    impl crate::private::Sealed for XBAR2 {}
    impl crate::Valid for XBAR2 {}
    impl XBAR2 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(XBAR2)
        }
    }
    pub type XBAR3 = Instance<3>;
    impl crate::private::Sealed for XBAR3 {}
    impl crate::Valid for XBAR3 {}
    impl XBAR3 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(XBAR3)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        [(XBAR2, 2), (XBAR3, 3)]
            .into_iter()
            .find(|(ptr, _)| core::ptr::eq(rb, *ptr))
            .map(|(_, inst)| inst)
    }
}
#[path = "."]
pub mod xbar1 {
    #[doc = "XBAR"]
    pub const XBAR1: *const RegisterBlock = 0x4275_0000 as *const RegisterBlock;
    #[path = "blocks/imxrt1189_cm33/xbar1.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type XBAR1 = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for XBAR1 {}
    impl crate::Valid for XBAR1 {}
    impl XBAR1 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(XBAR1)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, XBAR1).then_some(0)
    }
}
#[doc = r"Number available in the NVIC for configuring priority"]
pub const NVIC_PRIO_BITS: u8 = 4;
#[doc = r" Instances for all of this device's peripherals."]
#[doc = r""]
#[doc = r" Use this if you want a single way to acquire *all* instances"]
#[doc = r" for your device."]
pub struct Instances {
    pub ADC1: adc::ADC1,
    pub ADC2: adc::ADC2,
    pub ANADIG_LDO_BBSM: anadig_ldo_bbsm::ANADIG_LDO_BBSM,
    pub ANADIG_MISC: anadig_misc::ANADIG_MISC,
    pub ANADIG_OSC: anadig_osc::ANADIG_OSC,
    pub ANADIG_PLL: anadig_pll::ANADIG_PLL,
    pub ANADIG_PMU: anadig_pmu::ANADIG_PMU,
    pub ANADIG_SLOTS: anadig_slots::ANADIG_SLOTS,
    pub ANADIG_TEMPSENSOR: anadig_tempsensor::ANADIG_TEMPSENSOR,
    pub AOI1: aoi::AOI1,
    pub AOI2: aoi::AOI2,
    pub AOI3: aoi::AOI3,
    pub AOI4: aoi::AOI4,
    pub ASRC: asrc::ASRC,
    pub AXBS: axbs::AXBS,
    pub BBNSM: bbnsm::BBNSM,
    pub BLK_CTRL_BBSMMIX: blk_ctrl_bbsmmix::BLK_CTRL_BBSMMIX,
    pub BLK_CTRL_NS_AONMIX: blk_ctrl_ns_aonmix::BLK_CTRL_NS_AONMIX,
    pub BLK_CTRL_S_AONMIX: blk_ctrl_s_aonmix::BLK_CTRL_S_AONMIX,
    pub BLK_CTRL_WAKEUPMIX: blk_ctrl_wakeupmix::BLK_CTRL_WAKEUPMIX,
    pub CAN2: can::CAN2,
    pub CAN1: can::CAN1,
    pub CAN3: can::CAN3,
    pub CCM: ccm::CCM,
    pub CM7PLATFORM_TCM: cm7platform_tcm::CM7PLATFORM_TCM,
    pub CMP1: cmp::CMP1,
    pub CMP2: cmp::CMP2,
    pub CMP3: cmp::CMP3,
    pub CMP4: cmp::CMP4,
    pub CP_CM33_IMX9RTC__CM33_CACHE_ECC_MCM:
        cp_cm33_imx9rtc__cm33_cache_ecc_mcm::CP_CM33_IMX9RTC__CM33_CACHE_ECC_MCM,
    pub CP_CM33_IMX9RTC__CM33_TCM_MCM: cp_cm33_imx9rtc__cm33_tcm_mcm::CP_CM33_IMX9RTC__CM33_TCM_MCM,
    pub DAC: dac::DAC,
    pub DCDC: dcdc::DCDC,
    pub DMA: dma::DMA,
    pub DMA3__TCD: dma3__tcd::DMA3__TCD,
    pub DMA4__TCD: dma4__tcd::DMA4__TCD,
    pub ECAT: ecat::ECAT,
    pub EIM: eim::EIM,
    pub EMDIO_BASE: emdio_base::EMDIO_BASE,
    pub EMDIO_GLOBAL: emdio_global::EMDIO_GLOBAL,
    pub ENETC0_BASE: enetc0_base::ENETC0_BASE,
    pub ENETC0_COMMON: enetc0_common::ENETC0_COMMON,
    pub ENETC0_ETH_MAC_PORT: enetc0_eth_mac_port::ENETC0_ETH_MAC_PORT,
    pub ENETC0_PORT: enetc0_port::ENETC0_PORT,
    pub ENETC0_SI0: enetc0_si0::ENETC0_SI0,
    pub ENETC1_BASE: enetc1_base::ENETC1_BASE,
    pub ENETC1_COMMON: enetc1_common::ENETC1_COMMON,
    pub ENETC1_PORT: enetc1_port::ENETC1_PORT,
    pub ENETC1_PSEUDO_MAC_PORT: enetc1_pseudo_mac_port::ENETC1_PSEUDO_MAC_PORT,
    pub ENETC1_SI0: enetc1_si0::ENETC1_SI0,
    pub ENETC1_SI1: enetc1_si1::ENETC1_SI1,
    pub EQDC1: eqdc::EQDC1,
    pub EQDC2: eqdc::EQDC2,
    pub EQDC3: eqdc::EQDC3,
    pub EQDC4: eqdc::EQDC4,
    pub ERM: erm::ERM,
    pub EWM: ewm::EWM,
    pub FLEXIO1: flexio::FLEXIO1,
    pub FLEXIO2: flexio::FLEXIO2,
    pub FLEXSPI1: flexspi::FLEXSPI1,
    pub FLEXSPI2: flexspi::FLEXSPI2,
    pub FLEXSPI_SLV: flexspi_slv::FLEXSPI_SLV,
    pub GPC_CPU_CTRL: gpc_cpu_ctrl::GPC_CPU_CTRL,
    pub GPC_GLOBAL: gpc_global::GPC_GLOBAL,
    pub GPC_SYS_SLEEP_CTRL: gpc_sys_sleep_ctrl::GPC_SYS_SLEEP_CTRL,
    pub GPT2: gpt::GPT2,
    pub GPT1: gpt::GPT1,
    pub I3C2: i3c::I3C2,
    pub I3C1: i3c::I3C1,
    pub IEE: iee::IEE,
    pub IEE_APC: iee_apc::IEE_APC,
    pub IERC_F0_PCI_HDR_TYPE0: ierc_f0_pci_hdr_type0::IERC_F0_PCI_HDR_TYPE0,
    pub IERC_IERB: ierc_ierb::IERC_IERB,
    pub IOMUXC: iomuxc::IOMUXC,
    pub IOMUXC_AON: iomuxc_aon::IOMUXC_AON,
    pub KPP: kpp::KPP,
    pub LPI2C3: lpi2c::LPI2C3,
    pub LPI2C4: lpi2c::LPI2C4,
    pub LPI2C5: lpi2c::LPI2C5,
    pub LPI2C6: lpi2c::LPI2C6,
    pub LPI2C1: lpi2c::LPI2C1,
    pub LPI2C2: lpi2c::LPI2C2,
    pub LPIT2: lpit::LPIT2,
    pub LPIT3: lpit::LPIT3,
    pub LPIT1: lpit::LPIT1,
    pub LPSPI3: lpspi::LPSPI3,
    pub LPSPI4: lpspi::LPSPI4,
    pub LPSPI5: lpspi::LPSPI5,
    pub LPSPI6: lpspi::LPSPI6,
    pub LPSPI1: lpspi::LPSPI1,
    pub LPSPI2: lpspi::LPSPI2,
    pub LPTMR2: lptmr::LPTMR2,
    pub LPTMR3: lptmr::LPTMR3,
    pub LPTMR1: lptmr::LPTMR1,
    pub LPUART3: lpuart::LPUART3,
    pub LPUART4: lpuart::LPUART4,
    pub LPUART5: lpuart::LPUART5,
    pub LPUART6: lpuart::LPUART6,
    pub LPUART9: lpuart::LPUART9,
    pub LPUART10: lpuart::LPUART10,
    pub LPUART11: lpuart::LPUART11,
    pub LPUART8: lpuart::LPUART8,
    pub LPUART1: lpuart::LPUART1,
    pub LPUART2: lpuart::LPUART2,
    pub LPUART7: lpuart::LPUART7,
    pub LPUART12: lpuart::LPUART12,
    pub M7_MCM: m7_mcm::M7_MCM,
    pub MECC1: mecc::MECC1,
    pub MECC2: mecc::MECC2,
    pub MSGINTR1: msgintr::MSGINTR1,
    pub MSGINTR2: msgintr::MSGINTR2,
    pub MSGINTR3: msgintr::MSGINTR3,
    pub MSGINTR4: msgintr::MSGINTR4,
    pub MSGINTR5: msgintr::MSGINTR5,
    pub MSGINTR6: msgintr::MSGINTR6,
    pub NETC_F0_PCI_HDR_TYPE0: netc_f0_pci_hdr_type0::NETC_F0_PCI_HDR_TYPE0,
    pub NETC_F1_PCI_HDR_TYPE0: netc_f1_pci_hdr_type0::NETC_F1_PCI_HDR_TYPE0,
    pub NETC_F2_PCI_HDR_TYPE0: netc_f2_pci_hdr_type0::NETC_F2_PCI_HDR_TYPE0,
    pub NETC_F3_PCI_HDR_TYPE0: netc_f3_pci_hdr_type0::NETC_F3_PCI_HDR_TYPE0,
    pub NETC_F4_PCI_HDR_TYPE0: netc_f4_pci_hdr_type0::NETC_F4_PCI_HDR_TYPE0,
    pub NETC_IERB: netc_ierb::NETC_IERB,
    pub NETC_PRIV: netc_priv::NETC_PRIV,
    pub NETC_VF1_PCI_HDR_TYPE0: netc_vf1_pci_hdr_type0::NETC_VF1_PCI_HDR_TYPE0,
    pub OCOTP_FSB: ocotp_fsb::OCOTP_FSB,
    pub OSC_RC_400M: osc_rc_400m::OSC_RC_400M,
    pub OTFAD1: otfad::OTFAD1,
    pub OTFAD2: otfad::OTFAD2,
    pub PDM: pdm::PDM,
    pub PHY_LDO: phy_ldo::PHY_LDO,
    pub PWM1: pwm::PWM1,
    pub PWM2: pwm::PWM2,
    pub PWM3: pwm::PWM3,
    pub PWM4: pwm::PWM4,
    pub RGPIO2: rgpio::RGPIO2,
    pub RGPIO3: rgpio::RGPIO3,
    pub RGPIO4: rgpio::RGPIO4,
    pub RGPIO5: rgpio::RGPIO5,
    pub RGPIO6: rgpio::RGPIO6,
    pub RGPIO1: rgpio::RGPIO1,
    pub RTWDOG3: rtwdog::RTWDOG3,
    pub RTWDOG4: rtwdog::RTWDOG4,
    pub RTWDOG5: rtwdog::RTWDOG5,
    pub RTWDOG1: rtwdog::RTWDOG1,
    pub RTWDOG2: rtwdog::RTWDOG2,
    pub SAI2: sai::SAI2,
    pub SAI3: sai::SAI3,
    pub SAI1: sai1::SAI1,
    pub SAI4: sai4::SAI4,
    pub SEMA2: sema::SEMA2,
    pub SEMA1: sema::SEMA1,
    pub SEMC: semc::SEMC,
    pub SINC1: sinc::SINC1,
    pub SINC2: sinc::SINC2,
    pub SINC3: sinc::SINC3,
    pub SPDIF: spdif::SPDIF,
    pub SRC_GENERAL_REG: src_general_reg::SRC_GENERAL_REG,
    pub SW0_BASE: sw0_base::SW0_BASE,
    pub SW0_COMMON: sw0_common::SW0_COMMON,
    pub SW0_ETH_MAC_PORT0: sw0_eth_mac_port0::SW0_ETH_MAC_PORT0,
    pub SW0_ETH_MAC_PORT1: sw0_eth_mac_port1::SW0_ETH_MAC_PORT1,
    pub SW0_ETH_MAC_PORT2: sw0_eth_mac_port2::SW0_ETH_MAC_PORT2,
    pub SW0_ETH_MAC_PORT3: sw0_eth_mac_port3::SW0_ETH_MAC_PORT3,
    pub SW0_PORT0: sw0_port::SW0_PORT0,
    pub SW0_PORT1: sw0_port::SW0_PORT1,
    pub SW0_PORT2: sw0_port::SW0_PORT2,
    pub SW0_PORT3: sw0_port::SW0_PORT3,
    pub SW0_PORT4: sw0_port4::SW0_PORT4,
    pub SW0_PSEUDO_MAC_PORT4: sw0_pseudo_mac_port4::SW0_PSEUDO_MAC_PORT4,
    pub SYS_CTR_COMPARE: sys_ctr_compare::SYS_CTR_COMPARE,
    pub SYS_CTR_CONTROL: sys_ctr_control::SYS_CTR_CONTROL,
    pub SYS_CTR_READ: sys_ctr_read::SYS_CTR_READ,
    pub SYSTICK1: sys_tick1::SYSTICK1,
    pub TMPSNS: tmpsns::TMPSNS,
    pub TMR1: tmr::TMR1,
    pub TMR2: tmr::TMR2,
    pub TMR3: tmr::TMR3,
    pub TMR4: tmr::TMR4,
    pub TMR5: tmr::TMR5,
    pub TMR6: tmr::TMR6,
    pub TMR7: tmr::TMR7,
    pub TMR8: tmr::TMR8,
    pub TMR0_BASE: tmr0_base::TMR0_BASE,
    pub TMR0_GLOBAL: tmr0_global::TMR0_GLOBAL,
    pub TPM3: tpm::TPM3,
    pub TPM4: tpm::TPM4,
    pub TPM5: tpm::TPM5,
    pub TPM6: tpm::TPM6,
    pub TPM1: tpm::TPM1,
    pub TPM2: tpm::TPM2,
    pub TRDC1: trdc1::TRDC1,
    pub TRDC2: trdc2::TRDC2,
    pub TRDC3: trdc3::TRDC3,
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
    pub VMBANDGAP: vmbandgap::VMBANDGAP,
    pub VREF: vref::VREF,
    pub XBAR2: xbar::XBAR2,
    pub XBAR3: xbar::XBAR3,
    pub XBAR1: xbar1::XBAR1,
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
            ADC1: adc::ADC1::instance(),
            ADC2: adc::ADC2::instance(),
            ANADIG_LDO_BBSM: anadig_ldo_bbsm::ANADIG_LDO_BBSM::instance(),
            ANADIG_MISC: anadig_misc::ANADIG_MISC::instance(),
            ANADIG_OSC: anadig_osc::ANADIG_OSC::instance(),
            ANADIG_PLL: anadig_pll::ANADIG_PLL::instance(),
            ANADIG_PMU: anadig_pmu::ANADIG_PMU::instance(),
            ANADIG_SLOTS: anadig_slots::ANADIG_SLOTS::instance(),
            ANADIG_TEMPSENSOR: anadig_tempsensor::ANADIG_TEMPSENSOR::instance(),
            AOI1: aoi::AOI1::instance(),
            AOI2: aoi::AOI2::instance(),
            AOI3: aoi::AOI3::instance(),
            AOI4: aoi::AOI4::instance(),
            ASRC: asrc::ASRC::instance(),
            AXBS: axbs::AXBS::instance(),
            BBNSM: bbnsm::BBNSM::instance(),
            BLK_CTRL_BBSMMIX: blk_ctrl_bbsmmix::BLK_CTRL_BBSMMIX::instance(),
            BLK_CTRL_NS_AONMIX: blk_ctrl_ns_aonmix::BLK_CTRL_NS_AONMIX::instance(),
            BLK_CTRL_S_AONMIX: blk_ctrl_s_aonmix::BLK_CTRL_S_AONMIX::instance(),
            BLK_CTRL_WAKEUPMIX: blk_ctrl_wakeupmix::BLK_CTRL_WAKEUPMIX::instance(),
            CAN2: can::CAN2::instance(),
            CAN1: can::CAN1::instance(),
            CAN3: can::CAN3::instance(),
            CCM: ccm::CCM::instance(),
            CM7PLATFORM_TCM: cm7platform_tcm::CM7PLATFORM_TCM::instance(),
            CMP1: cmp::CMP1::instance(),
            CMP2: cmp::CMP2::instance(),
            CMP3: cmp::CMP3::instance(),
            CMP4: cmp::CMP4::instance(),
            CP_CM33_IMX9RTC__CM33_CACHE_ECC_MCM:
                cp_cm33_imx9rtc__cm33_cache_ecc_mcm::CP_CM33_IMX9RTC__CM33_CACHE_ECC_MCM::instance(),
            CP_CM33_IMX9RTC__CM33_TCM_MCM:
                cp_cm33_imx9rtc__cm33_tcm_mcm::CP_CM33_IMX9RTC__CM33_TCM_MCM::instance(),
            DAC: dac::DAC::instance(),
            DCDC: dcdc::DCDC::instance(),
            DMA: dma::DMA::instance(),
            DMA3__TCD: dma3__tcd::DMA3__TCD::instance(),
            DMA4__TCD: dma4__tcd::DMA4__TCD::instance(),
            ECAT: ecat::ECAT::instance(),
            EIM: eim::EIM::instance(),
            EMDIO_BASE: emdio_base::EMDIO_BASE::instance(),
            EMDIO_GLOBAL: emdio_global::EMDIO_GLOBAL::instance(),
            ENETC0_BASE: enetc0_base::ENETC0_BASE::instance(),
            ENETC0_COMMON: enetc0_common::ENETC0_COMMON::instance(),
            ENETC0_ETH_MAC_PORT: enetc0_eth_mac_port::ENETC0_ETH_MAC_PORT::instance(),
            ENETC0_PORT: enetc0_port::ENETC0_PORT::instance(),
            ENETC0_SI0: enetc0_si0::ENETC0_SI0::instance(),
            ENETC1_BASE: enetc1_base::ENETC1_BASE::instance(),
            ENETC1_COMMON: enetc1_common::ENETC1_COMMON::instance(),
            ENETC1_PORT: enetc1_port::ENETC1_PORT::instance(),
            ENETC1_PSEUDO_MAC_PORT: enetc1_pseudo_mac_port::ENETC1_PSEUDO_MAC_PORT::instance(),
            ENETC1_SI0: enetc1_si0::ENETC1_SI0::instance(),
            ENETC1_SI1: enetc1_si1::ENETC1_SI1::instance(),
            EQDC1: eqdc::EQDC1::instance(),
            EQDC2: eqdc::EQDC2::instance(),
            EQDC3: eqdc::EQDC3::instance(),
            EQDC4: eqdc::EQDC4::instance(),
            ERM: erm::ERM::instance(),
            EWM: ewm::EWM::instance(),
            FLEXIO1: flexio::FLEXIO1::instance(),
            FLEXIO2: flexio::FLEXIO2::instance(),
            FLEXSPI1: flexspi::FLEXSPI1::instance(),
            FLEXSPI2: flexspi::FLEXSPI2::instance(),
            FLEXSPI_SLV: flexspi_slv::FLEXSPI_SLV::instance(),
            GPC_CPU_CTRL: gpc_cpu_ctrl::GPC_CPU_CTRL::instance(),
            GPC_GLOBAL: gpc_global::GPC_GLOBAL::instance(),
            GPC_SYS_SLEEP_CTRL: gpc_sys_sleep_ctrl::GPC_SYS_SLEEP_CTRL::instance(),
            GPT2: gpt::GPT2::instance(),
            GPT1: gpt::GPT1::instance(),
            I3C2: i3c::I3C2::instance(),
            I3C1: i3c::I3C1::instance(),
            IEE: iee::IEE::instance(),
            IEE_APC: iee_apc::IEE_APC::instance(),
            IERC_F0_PCI_HDR_TYPE0: ierc_f0_pci_hdr_type0::IERC_F0_PCI_HDR_TYPE0::instance(),
            IERC_IERB: ierc_ierb::IERC_IERB::instance(),
            IOMUXC: iomuxc::IOMUXC::instance(),
            IOMUXC_AON: iomuxc_aon::IOMUXC_AON::instance(),
            KPP: kpp::KPP::instance(),
            LPI2C3: lpi2c::LPI2C3::instance(),
            LPI2C4: lpi2c::LPI2C4::instance(),
            LPI2C5: lpi2c::LPI2C5::instance(),
            LPI2C6: lpi2c::LPI2C6::instance(),
            LPI2C1: lpi2c::LPI2C1::instance(),
            LPI2C2: lpi2c::LPI2C2::instance(),
            LPIT2: lpit::LPIT2::instance(),
            LPIT3: lpit::LPIT3::instance(),
            LPIT1: lpit::LPIT1::instance(),
            LPSPI3: lpspi::LPSPI3::instance(),
            LPSPI4: lpspi::LPSPI4::instance(),
            LPSPI5: lpspi::LPSPI5::instance(),
            LPSPI6: lpspi::LPSPI6::instance(),
            LPSPI1: lpspi::LPSPI1::instance(),
            LPSPI2: lpspi::LPSPI2::instance(),
            LPTMR2: lptmr::LPTMR2::instance(),
            LPTMR3: lptmr::LPTMR3::instance(),
            LPTMR1: lptmr::LPTMR1::instance(),
            LPUART3: lpuart::LPUART3::instance(),
            LPUART4: lpuart::LPUART4::instance(),
            LPUART5: lpuart::LPUART5::instance(),
            LPUART6: lpuart::LPUART6::instance(),
            LPUART9: lpuart::LPUART9::instance(),
            LPUART10: lpuart::LPUART10::instance(),
            LPUART11: lpuart::LPUART11::instance(),
            LPUART8: lpuart::LPUART8::instance(),
            LPUART1: lpuart::LPUART1::instance(),
            LPUART2: lpuart::LPUART2::instance(),
            LPUART7: lpuart::LPUART7::instance(),
            LPUART12: lpuart::LPUART12::instance(),
            M7_MCM: m7_mcm::M7_MCM::instance(),
            MECC1: mecc::MECC1::instance(),
            MECC2: mecc::MECC2::instance(),
            MSGINTR1: msgintr::MSGINTR1::instance(),
            MSGINTR2: msgintr::MSGINTR2::instance(),
            MSGINTR3: msgintr::MSGINTR3::instance(),
            MSGINTR4: msgintr::MSGINTR4::instance(),
            MSGINTR5: msgintr::MSGINTR5::instance(),
            MSGINTR6: msgintr::MSGINTR6::instance(),
            NETC_F0_PCI_HDR_TYPE0: netc_f0_pci_hdr_type0::NETC_F0_PCI_HDR_TYPE0::instance(),
            NETC_F1_PCI_HDR_TYPE0: netc_f1_pci_hdr_type0::NETC_F1_PCI_HDR_TYPE0::instance(),
            NETC_F2_PCI_HDR_TYPE0: netc_f2_pci_hdr_type0::NETC_F2_PCI_HDR_TYPE0::instance(),
            NETC_F3_PCI_HDR_TYPE0: netc_f3_pci_hdr_type0::NETC_F3_PCI_HDR_TYPE0::instance(),
            NETC_F4_PCI_HDR_TYPE0: netc_f4_pci_hdr_type0::NETC_F4_PCI_HDR_TYPE0::instance(),
            NETC_IERB: netc_ierb::NETC_IERB::instance(),
            NETC_PRIV: netc_priv::NETC_PRIV::instance(),
            NETC_VF1_PCI_HDR_TYPE0: netc_vf1_pci_hdr_type0::NETC_VF1_PCI_HDR_TYPE0::instance(),
            OCOTP_FSB: ocotp_fsb::OCOTP_FSB::instance(),
            OSC_RC_400M: osc_rc_400m::OSC_RC_400M::instance(),
            OTFAD1: otfad::OTFAD1::instance(),
            OTFAD2: otfad::OTFAD2::instance(),
            PDM: pdm::PDM::instance(),
            PHY_LDO: phy_ldo::PHY_LDO::instance(),
            PWM1: pwm::PWM1::instance(),
            PWM2: pwm::PWM2::instance(),
            PWM3: pwm::PWM3::instance(),
            PWM4: pwm::PWM4::instance(),
            RGPIO2: rgpio::RGPIO2::instance(),
            RGPIO3: rgpio::RGPIO3::instance(),
            RGPIO4: rgpio::RGPIO4::instance(),
            RGPIO5: rgpio::RGPIO5::instance(),
            RGPIO6: rgpio::RGPIO6::instance(),
            RGPIO1: rgpio::RGPIO1::instance(),
            RTWDOG3: rtwdog::RTWDOG3::instance(),
            RTWDOG4: rtwdog::RTWDOG4::instance(),
            RTWDOG5: rtwdog::RTWDOG5::instance(),
            RTWDOG1: rtwdog::RTWDOG1::instance(),
            RTWDOG2: rtwdog::RTWDOG2::instance(),
            SAI2: sai::SAI2::instance(),
            SAI3: sai::SAI3::instance(),
            SAI1: sai1::SAI1::instance(),
            SAI4: sai4::SAI4::instance(),
            SEMA2: sema::SEMA2::instance(),
            SEMA1: sema::SEMA1::instance(),
            SEMC: semc::SEMC::instance(),
            SINC1: sinc::SINC1::instance(),
            SINC2: sinc::SINC2::instance(),
            SINC3: sinc::SINC3::instance(),
            SPDIF: spdif::SPDIF::instance(),
            SRC_GENERAL_REG: src_general_reg::SRC_GENERAL_REG::instance(),
            SW0_BASE: sw0_base::SW0_BASE::instance(),
            SW0_COMMON: sw0_common::SW0_COMMON::instance(),
            SW0_ETH_MAC_PORT0: sw0_eth_mac_port0::SW0_ETH_MAC_PORT0::instance(),
            SW0_ETH_MAC_PORT1: sw0_eth_mac_port1::SW0_ETH_MAC_PORT1::instance(),
            SW0_ETH_MAC_PORT2: sw0_eth_mac_port2::SW0_ETH_MAC_PORT2::instance(),
            SW0_ETH_MAC_PORT3: sw0_eth_mac_port3::SW0_ETH_MAC_PORT3::instance(),
            SW0_PORT0: sw0_port::SW0_PORT0::instance(),
            SW0_PORT1: sw0_port::SW0_PORT1::instance(),
            SW0_PORT2: sw0_port::SW0_PORT2::instance(),
            SW0_PORT3: sw0_port::SW0_PORT3::instance(),
            SW0_PORT4: sw0_port4::SW0_PORT4::instance(),
            SW0_PSEUDO_MAC_PORT4: sw0_pseudo_mac_port4::SW0_PSEUDO_MAC_PORT4::instance(),
            SYS_CTR_COMPARE: sys_ctr_compare::SYS_CTR_COMPARE::instance(),
            SYS_CTR_CONTROL: sys_ctr_control::SYS_CTR_CONTROL::instance(),
            SYS_CTR_READ: sys_ctr_read::SYS_CTR_READ::instance(),
            SYSTICK1: sys_tick1::SYSTICK1::instance(),
            TMPSNS: tmpsns::TMPSNS::instance(),
            TMR1: tmr::TMR1::instance(),
            TMR2: tmr::TMR2::instance(),
            TMR3: tmr::TMR3::instance(),
            TMR4: tmr::TMR4::instance(),
            TMR5: tmr::TMR5::instance(),
            TMR6: tmr::TMR6::instance(),
            TMR7: tmr::TMR7::instance(),
            TMR8: tmr::TMR8::instance(),
            TMR0_BASE: tmr0_base::TMR0_BASE::instance(),
            TMR0_GLOBAL: tmr0_global::TMR0_GLOBAL::instance(),
            TPM3: tpm::TPM3::instance(),
            TPM4: tpm::TPM4::instance(),
            TPM5: tpm::TPM5::instance(),
            TPM6: tpm::TPM6::instance(),
            TPM1: tpm::TPM1::instance(),
            TPM2: tpm::TPM2::instance(),
            TRDC1: trdc1::TRDC1::instance(),
            TRDC2: trdc2::TRDC2::instance(),
            TRDC3: trdc3::TRDC3::instance(),
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
            VMBANDGAP: vmbandgap::VMBANDGAP::instance(),
            VREF: vref::VREF::instance(),
            XBAR2: xbar::XBAR2::instance(),
            XBAR3: xbar::XBAR3::instance(),
            XBAR1: xbar1::XBAR1::instance(),
        }
    }
}
