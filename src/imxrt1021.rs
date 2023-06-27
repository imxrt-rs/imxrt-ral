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
    #[doc = "28 - LPI2C1"]
    LPI2C1 = 28,
    #[doc = "29 - LPI2C2"]
    LPI2C2 = 29,
    #[doc = "30 - LPI2C3"]
    LPI2C3 = 30,
    #[doc = "31 - LPI2C4"]
    LPI2C4 = 31,
    #[doc = "32 - LPSPI1"]
    LPSPI1 = 32,
    #[doc = "33 - LPSPI2"]
    LPSPI2 = 33,
    #[doc = "34 - LPSPI3"]
    LPSPI3 = 34,
    #[doc = "35 - LPSPI4"]
    LPSPI4 = 35,
    #[doc = "36 - CAN1"]
    CAN1 = 36,
    #[doc = "37 - CAN2"]
    CAN2 = 37,
    #[doc = "38 - FLEXRAM"]
    FLEXRAM = 38,
    #[doc = "39 - KPP"]
    KPP = 39,
    #[doc = "41 - GPR (aka \"GPC\") interrupt request"]
    GPR_IRQ = 41,
    #[doc = "45 - WDOG2"]
    WDOG2 = 45,
    #[doc = "46 - SNVS_HP_WRAPPER"]
    SNVS_HP_WRAPPER = 46,
    #[doc = "47 - SNVS_HP_WRAPPER_TZ"]
    SNVS_HP_WRAPPER_TZ = 47,
    #[doc = "48 - SNVS_LP_WRAPPER"]
    SNVS_LP_WRAPPER = 48,
    #[doc = "49 - CSU"]
    CSU = 49,
    #[doc = "50 - DCP"]
    DCP = 50,
    #[doc = "51 - DCP_VMI"]
    DCP_VMI = 51,
    #[doc = "53 - TRNG"]
    TRNG = 53,
    #[doc = "55 - BEE"]
    BEE = 55,
    #[doc = "56 - SAI1"]
    SAI1 = 56,
    #[doc = "57 - SAI2"]
    SAI2 = 57,
    #[doc = "58 - SAI3_RX"]
    SAI3_RX = 58,
    #[doc = "59 - SAI3_TX"]
    SAI3_TX = 59,
    #[doc = "60 - SPDIF"]
    SPDIF = 60,
    #[doc = "61 - PMU"]
    PMU = 61,
    #[doc = "63 - TEMP_LOW_HIGH"]
    TEMP_LOW_HIGH = 63,
    #[doc = "64 - TEMP_PANIC"]
    TEMP_PANIC = 64,
    #[doc = "65 - USB_PHY"]
    USB_PHY = 65,
    #[doc = "67 - ADC1"]
    ADC1 = 67,
    #[doc = "68 - ADC2"]
    ADC2 = 68,
    #[doc = "69 - DCDC"]
    DCDC = 69,
    #[doc = "72 - GPIO1_INT0"]
    GPIO1_INT0 = 72,
    #[doc = "73 - GPIO1_INT1"]
    GPIO1_INT1 = 73,
    #[doc = "74 - GPIO1_INT2"]
    GPIO1_INT2 = 74,
    #[doc = "75 - GPIO1_INT3"]
    GPIO1_INT3 = 75,
    #[doc = "76 - GPIO1_INT4"]
    GPIO1_INT4 = 76,
    #[doc = "77 - GPIO1_INT5"]
    GPIO1_INT5 = 77,
    #[doc = "78 - GPIO1_INT6"]
    GPIO1_INT6 = 78,
    #[doc = "79 - GPIO1_INT7"]
    GPIO1_INT7 = 79,
    #[doc = "80 - GPIO1_COMBINED_0_15"]
    GPIO1_COMBINED_0_15 = 80,
    #[doc = "81 - GPIO1_COMBINED_16_31"]
    GPIO1_COMBINED_16_31 = 81,
    #[doc = "82 - GPIO2_COMBINED_0_15"]
    GPIO2_COMBINED_0_15 = 82,
    #[doc = "83 - GPIO2_COMBINED_16_31"]
    GPIO2_COMBINED_16_31 = 83,
    #[doc = "84 - GPIO3_COMBINED_0_15"]
    GPIO3_COMBINED_0_15 = 84,
    #[doc = "85 - GPIO3_COMBINED_16_31"]
    GPIO3_COMBINED_16_31 = 85,
    #[doc = "88 - GPIO5_COMBINED_0_15"]
    GPIO5_COMBINED_0_15 = 88,
    #[doc = "89 - GPIO5_COMBINED_16_31"]
    GPIO5_COMBINED_16_31 = 89,
    #[doc = "90 - FLEXIO1"]
    FLEXIO1 = 90,
    #[doc = "92 - WDOG1"]
    WDOG1 = 92,
    #[doc = "93 - RTWDOG"]
    RTWDOG = 93,
    #[doc = "94 - EWM"]
    EWM = 94,
    #[doc = "95 - CCM_1"]
    CCM_1 = 95,
    #[doc = "96 - CCM_2"]
    CCM_2 = 96,
    #[doc = "97 - GPC"]
    GPC = 97,
    #[doc = "98 - SRC"]
    SRC = 98,
    #[doc = "100 - GPT1"]
    GPT1 = 100,
    #[doc = "101 - GPT2"]
    GPT2 = 101,
    #[doc = "102 - PWM1_0"]
    PWM1_0 = 102,
    #[doc = "103 - PWM1_1"]
    PWM1_1 = 103,
    #[doc = "104 - PWM1_2"]
    PWM1_2 = 104,
    #[doc = "105 - PWM1_3"]
    PWM1_3 = 105,
    #[doc = "106 - PWM1_FAULT"]
    PWM1_FAULT = 106,
    #[doc = "108 - FLEXSPI"]
    FLEXSPI = 108,
    #[doc = "109 - SEMC"]
    SEMC = 109,
    #[doc = "110 - USDHC1"]
    USDHC1 = 110,
    #[doc = "111 - USDHC2"]
    USDHC2 = 111,
    #[doc = "113 - USB_OTG1"]
    USB_OTG1 = 113,
    #[doc = "114 - ENET"]
    ENET = 114,
    #[doc = "115 - ENET_1588_TIMER"]
    ENET_1588_TIMER = 115,
    #[doc = "116 - XBAR1_IRQ_0_1"]
    XBAR1_IRQ_0_1 = 116,
    #[doc = "117 - XBAR1_IRQ_2_3"]
    XBAR1_IRQ_2_3 = 117,
    #[doc = "118 - ADC_ETC_IRQ0"]
    ADC_ETC_IRQ0 = 118,
    #[doc = "119 - ADC_ETC_IRQ1"]
    ADC_ETC_IRQ1 = 119,
    #[doc = "120 - ADC_ETC_IRQ2"]
    ADC_ETC_IRQ2 = 120,
    #[doc = "121 - ADC_ETC_ERROR_IRQ"]
    ADC_ETC_ERROR_IRQ = 121,
    #[doc = "122 - PIT"]
    PIT = 122,
    #[doc = "123 - ACMP1"]
    ACMP1 = 123,
    #[doc = "124 - ACMP2"]
    ACMP2 = 124,
    #[doc = "125 - ACMP3"]
    ACMP3 = 125,
    #[doc = "126 - ACMP4"]
    ACMP4 = 126,
    #[doc = "129 - ENC1"]
    ENC1 = 129,
    #[doc = "130 - ENC2"]
    ENC2 = 130,
    #[doc = "133 - TMR1"]
    TMR1 = 133,
    #[doc = "134 - TMR2"]
    TMR2 = 134,
    #[doc = "137 - PWM2_0"]
    PWM2_0 = 137,
    #[doc = "138 - PWM2_1"]
    PWM2_1 = 138,
    #[doc = "139 - PWM2_2"]
    PWM2_2 = 139,
    #[doc = "140 - PWM2_3"]
    PWM2_3 = 140,
    #[doc = "141 - PWM2_FAULT"]
    PWM2_FAULT = 141,
}
pub type interrupt = Interrupt;
unsafe impl cortex_m::interrupt::InterruptNumber for Interrupt {
    #[inline(always)]
    fn number(self) -> u16 {
        self as u16
    }
}
#[cfg(feature = "rt")]
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
        fn LPUART1();
        fn LPUART2();
        fn LPUART3();
        fn LPUART4();
        fn LPUART5();
        fn LPUART6();
        fn LPUART7();
        fn LPUART8();
        fn LPI2C1();
        fn LPI2C2();
        fn LPI2C3();
        fn LPI2C4();
        fn LPSPI1();
        fn LPSPI2();
        fn LPSPI3();
        fn LPSPI4();
        fn CAN1();
        fn CAN2();
        fn FLEXRAM();
        fn KPP();
        fn GPR_IRQ();
        fn WDOG2();
        fn SNVS_HP_WRAPPER();
        fn SNVS_HP_WRAPPER_TZ();
        fn SNVS_LP_WRAPPER();
        fn CSU();
        fn DCP();
        fn DCP_VMI();
        fn TRNG();
        fn BEE();
        fn SAI1();
        fn SAI2();
        fn SAI3_RX();
        fn SAI3_TX();
        fn SPDIF();
        fn PMU();
        fn TEMP_LOW_HIGH();
        fn TEMP_PANIC();
        fn USB_PHY();
        fn ADC1();
        fn ADC2();
        fn DCDC();
        fn GPIO1_INT0();
        fn GPIO1_INT1();
        fn GPIO1_INT2();
        fn GPIO1_INT3();
        fn GPIO1_INT4();
        fn GPIO1_INT5();
        fn GPIO1_INT6();
        fn GPIO1_INT7();
        fn GPIO1_COMBINED_0_15();
        fn GPIO1_COMBINED_16_31();
        fn GPIO2_COMBINED_0_15();
        fn GPIO2_COMBINED_16_31();
        fn GPIO3_COMBINED_0_15();
        fn GPIO3_COMBINED_16_31();
        fn GPIO5_COMBINED_0_15();
        fn GPIO5_COMBINED_16_31();
        fn FLEXIO1();
        fn WDOG1();
        fn RTWDOG();
        fn EWM();
        fn CCM_1();
        fn CCM_2();
        fn GPC();
        fn SRC();
        fn GPT1();
        fn GPT2();
        fn PWM1_0();
        fn PWM1_1();
        fn PWM1_2();
        fn PWM1_3();
        fn PWM1_FAULT();
        fn FLEXSPI();
        fn SEMC();
        fn USDHC1();
        fn USDHC2();
        fn USB_OTG1();
        fn ENET();
        fn ENET_1588_TIMER();
        fn XBAR1_IRQ_0_1();
        fn XBAR1_IRQ_2_3();
        fn ADC_ETC_IRQ0();
        fn ADC_ETC_IRQ1();
        fn ADC_ETC_IRQ2();
        fn ADC_ETC_ERROR_IRQ();
        fn PIT();
        fn ACMP1();
        fn ACMP2();
        fn ACMP3();
        fn ACMP4();
        fn ENC1();
        fn ENC2();
        fn TMR1();
        fn TMR2();
        fn PWM2_0();
        fn PWM2_1();
        fn PWM2_2();
        fn PWM2_3();
        fn PWM2_FAULT();
    }
    pub union Vector {
        _handler: unsafe extern "C" fn(),
        _reserved: u32,
    }
    #[cfg_attr(target_os = "none", link_section = ".vector_table.interrupts")]
    #[no_mangle]
    pub static __INTERRUPTS: [Vector; 142] = [
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
        Vector { _reserved: 0 },
        Vector { _handler: LPUART1 },
        Vector { _handler: LPUART2 },
        Vector { _handler: LPUART3 },
        Vector { _handler: LPUART4 },
        Vector { _handler: LPUART5 },
        Vector { _handler: LPUART6 },
        Vector { _handler: LPUART7 },
        Vector { _handler: LPUART8 },
        Vector { _handler: LPI2C1 },
        Vector { _handler: LPI2C2 },
        Vector { _handler: LPI2C3 },
        Vector { _handler: LPI2C4 },
        Vector { _handler: LPSPI1 },
        Vector { _handler: LPSPI2 },
        Vector { _handler: LPSPI3 },
        Vector { _handler: LPSPI4 },
        Vector { _handler: CAN1 },
        Vector { _handler: CAN2 },
        Vector { _handler: FLEXRAM },
        Vector { _handler: KPP },
        Vector { _reserved: 0 },
        Vector { _handler: GPR_IRQ },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _handler: WDOG2 },
        Vector {
            _handler: SNVS_HP_WRAPPER,
        },
        Vector {
            _handler: SNVS_HP_WRAPPER_TZ,
        },
        Vector {
            _handler: SNVS_LP_WRAPPER,
        },
        Vector { _handler: CSU },
        Vector { _handler: DCP },
        Vector { _handler: DCP_VMI },
        Vector { _reserved: 0 },
        Vector { _handler: TRNG },
        Vector { _reserved: 0 },
        Vector { _handler: BEE },
        Vector { _handler: SAI1 },
        Vector { _handler: SAI2 },
        Vector { _handler: SAI3_RX },
        Vector { _handler: SAI3_TX },
        Vector { _handler: SPDIF },
        Vector { _handler: PMU },
        Vector { _reserved: 0 },
        Vector {
            _handler: TEMP_LOW_HIGH,
        },
        Vector {
            _handler: TEMP_PANIC,
        },
        Vector { _handler: USB_PHY },
        Vector { _reserved: 0 },
        Vector { _handler: ADC1 },
        Vector { _handler: ADC2 },
        Vector { _handler: DCDC },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector {
            _handler: GPIO1_INT0,
        },
        Vector {
            _handler: GPIO1_INT1,
        },
        Vector {
            _handler: GPIO1_INT2,
        },
        Vector {
            _handler: GPIO1_INT3,
        },
        Vector {
            _handler: GPIO1_INT4,
        },
        Vector {
            _handler: GPIO1_INT5,
        },
        Vector {
            _handler: GPIO1_INT6,
        },
        Vector {
            _handler: GPIO1_INT7,
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
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector {
            _handler: GPIO5_COMBINED_0_15,
        },
        Vector {
            _handler: GPIO5_COMBINED_16_31,
        },
        Vector { _handler: FLEXIO1 },
        Vector { _reserved: 0 },
        Vector { _handler: WDOG1 },
        Vector { _handler: RTWDOG },
        Vector { _handler: EWM },
        Vector { _handler: CCM_1 },
        Vector { _handler: CCM_2 },
        Vector { _handler: GPC },
        Vector { _handler: SRC },
        Vector { _reserved: 0 },
        Vector { _handler: GPT1 },
        Vector { _handler: GPT2 },
        Vector { _handler: PWM1_0 },
        Vector { _handler: PWM1_1 },
        Vector { _handler: PWM1_2 },
        Vector { _handler: PWM1_3 },
        Vector {
            _handler: PWM1_FAULT,
        },
        Vector { _reserved: 0 },
        Vector { _handler: FLEXSPI },
        Vector { _handler: SEMC },
        Vector { _handler: USDHC1 },
        Vector { _handler: USDHC2 },
        Vector { _reserved: 0 },
        Vector { _handler: USB_OTG1 },
        Vector { _handler: ENET },
        Vector {
            _handler: ENET_1588_TIMER,
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
            _handler: ADC_ETC_ERROR_IRQ,
        },
        Vector { _handler: PIT },
        Vector { _handler: ACMP1 },
        Vector { _handler: ACMP2 },
        Vector { _handler: ACMP3 },
        Vector { _handler: ACMP4 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _handler: ENC1 },
        Vector { _handler: ENC2 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _handler: TMR1 },
        Vector { _handler: TMR2 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _handler: PWM2_0 },
        Vector { _handler: PWM2_1 },
        Vector { _handler: PWM2_2 },
        Vector { _handler: PWM2_3 },
        Vector {
            _handler: PWM2_FAULT,
        },
    ];
}
#[path = "."]
pub mod adc {
    #[doc = "Analog-to-Digital Converter"]
    pub const ADC1: *const RegisterBlock = 0x400c_4000 as *const RegisterBlock;
    #[doc = "Analog-to-Digital Converter"]
    pub const ADC2: *const RegisterBlock = 0x400c_8000 as *const RegisterBlock;
    #[path = "blocks/imxrt1021/adc.rs"]
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
pub mod adc_etc {
    #[doc = "ADC_ETC"]
    pub const ADC_ETC: *const RegisterBlock = 0x403b_0000 as *const RegisterBlock;
    #[path = "blocks/imxrt1021/adc_etc.rs"]
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
pub mod aipstz {
    #[doc = "AIPSTZ Control Registers"]
    pub const AIPSTZ1: *const RegisterBlock = 0x4007_c000 as *const RegisterBlock;
    #[doc = "AIPSTZ Control Registers"]
    pub const AIPSTZ2: *const RegisterBlock = 0x4017_c000 as *const RegisterBlock;
    #[doc = "AIPSTZ Control Registers"]
    pub const AIPSTZ3: *const RegisterBlock = 0x4027_c000 as *const RegisterBlock;
    #[doc = "AIPSTZ Control Registers"]
    pub const AIPSTZ4: *const RegisterBlock = 0x4037_c000 as *const RegisterBlock;
    #[path = "blocks/imxrt1011/aipstz.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type AIPSTZ1 = Instance<1>;
    impl crate::private::Sealed for AIPSTZ1 {}
    impl crate::Valid for AIPSTZ1 {}
    impl AIPSTZ1 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(AIPSTZ1)
        }
    }
    pub type AIPSTZ2 = Instance<2>;
    impl crate::private::Sealed for AIPSTZ2 {}
    impl crate::Valid for AIPSTZ2 {}
    impl AIPSTZ2 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(AIPSTZ2)
        }
    }
    pub type AIPSTZ3 = Instance<3>;
    impl crate::private::Sealed for AIPSTZ3 {}
    impl crate::Valid for AIPSTZ3 {}
    impl AIPSTZ3 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(AIPSTZ3)
        }
    }
    pub type AIPSTZ4 = Instance<4>;
    impl crate::private::Sealed for AIPSTZ4 {}
    impl crate::Valid for AIPSTZ4 {}
    impl AIPSTZ4 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(AIPSTZ4)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        [(AIPSTZ1, 1), (AIPSTZ2, 2), (AIPSTZ3, 3), (AIPSTZ4, 4)]
            .into_iter()
            .find(|(ptr, _)| core::ptr::eq(rb, *ptr))
            .map(|(_, inst)| inst)
    }
}
#[path = "."]
pub mod aoi {
    #[doc = "AND/OR/INVERT module"]
    pub const AOI: *const RegisterBlock = 0x403b_4000 as *const RegisterBlock;
    #[path = "blocks/imxrt1011/aoi.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type AOI = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for AOI {}
    impl crate::Valid for AOI {}
    impl AOI {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(AOI)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, AOI).then_some(0)
    }
}
#[path = "."]
pub mod bee {
    #[doc = "Bus Encryption Engine"]
    pub const BEE: *const RegisterBlock = 0x403e_c000 as *const RegisterBlock;
    #[path = "blocks/imxrt1015/bee.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type BEE = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for BEE {}
    impl crate::Valid for BEE {}
    impl BEE {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(BEE)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, BEE).then_some(0)
    }
}
#[path = "."]
pub mod can {
    #[doc = "FLEXCAN"]
    pub const CAN1: *const RegisterBlock = 0x401d_0000 as *const RegisterBlock;
    #[doc = "FLEXCAN"]
    pub const CAN2: *const RegisterBlock = 0x401d_4000 as *const RegisterBlock;
    #[path = "blocks/imxrt1021/can.rs"]
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
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        [(CAN1, 1), (CAN2, 2)]
            .into_iter()
            .find(|(ptr, _)| core::ptr::eq(rb, *ptr))
            .map(|(_, inst)| inst)
    }
}
#[path = "."]
pub mod ccm {
    #[doc = "CCM"]
    pub const CCM: *const RegisterBlock = 0x400f_c000 as *const RegisterBlock;
    #[path = "blocks/imxrt1021/ccm.rs"]
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
pub mod ccm_analog {
    #[doc = "CCM_ANALOG"]
    pub const CCM_ANALOG: *const RegisterBlock = 0x400d_8000 as *const RegisterBlock;
    #[path = "blocks/imxrt1021/ccm_analog.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type CCM_ANALOG = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for CCM_ANALOG {}
    impl crate::Valid for CCM_ANALOG {}
    impl CCM_ANALOG {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(CCM_ANALOG)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, CCM_ANALOG).then_some(0)
    }
}
#[path = "."]
pub mod cmp {
    #[doc = "High-Speed Comparator (CMP), Voltage Reference (VREF) Digital-to-Analog Converter (DAC), and Analog Mux (ANMUX)"]
    pub const CMP1: *const RegisterBlock = 0x4009_4000 as *const RegisterBlock;
    #[doc = "High-Speed Comparator (CMP), Voltage Reference (VREF) Digital-to-Analog Converter (DAC), and Analog Mux (ANMUX)"]
    pub const CMP2: *const RegisterBlock = 0x4009_4008 as *const RegisterBlock;
    #[doc = "High-Speed Comparator (CMP), Voltage Reference (VREF) Digital-to-Analog Converter (DAC), and Analog Mux (ANMUX)"]
    pub const CMP3: *const RegisterBlock = 0x4009_4010 as *const RegisterBlock;
    #[doc = "High-Speed Comparator (CMP), Voltage Reference (VREF) Digital-to-Analog Converter (DAC), and Analog Mux (ANMUX)"]
    pub const CMP4: *const RegisterBlock = 0x4009_4018 as *const RegisterBlock;
    #[path = "blocks/imxrt1021/cmp.rs"]
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
pub mod csu {
    #[doc = "CSU registers"]
    pub const CSU: *const RegisterBlock = 0x400d_c000 as *const RegisterBlock;
    #[path = "blocks/imxrt1011/csu.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type CSU = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for CSU {}
    impl crate::Valid for CSU {}
    impl CSU {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(CSU)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, CSU).then_some(0)
    }
}
#[path = "."]
pub mod dcdc {
    #[doc = "DCDC"]
    pub const DCDC: *const RegisterBlock = 0x4008_0000 as *const RegisterBlock;
    #[path = "blocks/imxrt1011/dcdc.rs"]
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
pub mod dcp {
    #[doc = "DCP register reference index"]
    pub const DCP: *const RegisterBlock = 0x402f_c000 as *const RegisterBlock;
    #[path = "blocks/imxrt1011/dcp.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type DCP = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for DCP {}
    impl crate::Valid for DCP {}
    impl DCP {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(DCP)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, DCP).then_some(0)
    }
}
#[path = "."]
pub mod dma {
    #[doc = "DMA"]
    pub const DMA: *const RegisterBlock = 0x400e_8000 as *const RegisterBlock;
    #[path = "blocks/imxrt1015/dma.rs"]
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
    pub const DMAMUX: *const RegisterBlock = 0x400e_c000 as *const RegisterBlock;
    #[path = "blocks/imxrt1015/dmamux.rs"]
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
pub mod enc {
    #[doc = "Quadrature Decoder"]
    pub const ENC1: *const RegisterBlock = 0x403c_8000 as *const RegisterBlock;
    #[doc = "Quadrature Decoder"]
    pub const ENC2: *const RegisterBlock = 0x403c_c000 as *const RegisterBlock;
    #[path = "blocks/imxrt1021/enc.rs"]
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
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        [(ENC1, 1), (ENC2, 2)]
            .into_iter()
            .find(|(ptr, _)| core::ptr::eq(rb, *ptr))
            .map(|(_, inst)| inst)
    }
}
#[path = "."]
pub mod enet {
    #[doc = "Ethernet MAC-NET Core"]
    pub const ENET: *const RegisterBlock = 0x402d_8000 as *const RegisterBlock;
    #[path = "blocks/imxrt1021/enet.rs"]
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
pub mod ewm {
    #[doc = "EWM"]
    pub const EWM: *const RegisterBlock = 0x400b_4000 as *const RegisterBlock;
    #[path = "blocks/imxrt1011/ewm.rs"]
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
    pub const FLEXIO: *const RegisterBlock = 0x401a_c000 as *const RegisterBlock;
    #[path = "blocks/imxrt1011/flexio.rs"]
    pub(super) mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type FLEXIO = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for FLEXIO {}
    impl crate::Valid for FLEXIO {}
    impl FLEXIO {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(FLEXIO)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, FLEXIO).then_some(0)
    }
}
#[deprecated(since = "0.5.1", note = "Use 'flexio'")]
pub mod flexio1 {
    pub use super::flexio::{blocks::*, number, Instance, FLEXIO as FLEXIO1};
}
#[path = "."]
pub mod flexram {
    #[doc = "FLEXRAM"]
    pub const FLEXRAM: *const RegisterBlock = 0x400b_0000 as *const RegisterBlock;
    #[path = "blocks/imxrt1015/flexram.rs"]
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
    pub const FLEXSPI: *const RegisterBlock = 0x402a_8000 as *const RegisterBlock;
    #[path = "blocks/imxrt1015/flexspi.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type FLEXSPI = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for FLEXSPI {}
    impl crate::Valid for FLEXSPI {}
    impl FLEXSPI {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(FLEXSPI)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, FLEXSPI).then_some(0)
    }
}
#[path = "."]
pub mod gpc {
    #[doc = "GPC"]
    pub const GPC: *const RegisterBlock = 0x400f_4000 as *const RegisterBlock;
    #[path = "blocks/imxrt1011/gpc.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type GPC = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for GPC {}
    impl crate::Valid for GPC {}
    impl GPC {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(GPC)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, GPC).then_some(0)
    }
}
#[path = "."]
pub mod gpio {
    #[doc = "GPIO"]
    pub const GPIO1: *const RegisterBlock = 0x401b_8000 as *const RegisterBlock;
    #[doc = "GPIO"]
    pub const GPIO5: *const RegisterBlock = 0x400c_0000 as *const RegisterBlock;
    #[doc = "GPIO"]
    pub const GPIO2: *const RegisterBlock = 0x401b_c000 as *const RegisterBlock;
    #[doc = "GPIO"]
    pub const GPIO3: *const RegisterBlock = 0x401c_0000 as *const RegisterBlock;
    #[path = "blocks/imxrt1011/gpio.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
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
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        [(GPIO1, 1), (GPIO5, 5), (GPIO2, 2), (GPIO3, 3)]
            .into_iter()
            .find(|(ptr, _)| core::ptr::eq(rb, *ptr))
            .map(|(_, inst)| inst)
    }
}
#[path = "."]
pub mod gpt {
    #[doc = "GPT"]
    pub const GPT1: *const RegisterBlock = 0x401e_c000 as *const RegisterBlock;
    #[doc = "GPT"]
    pub const GPT2: *const RegisterBlock = 0x401f_0000 as *const RegisterBlock;
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
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        [(GPT1, 1), (GPT2, 2)]
            .into_iter()
            .find(|(ptr, _)| core::ptr::eq(rb, *ptr))
            .map(|(_, inst)| inst)
    }
}
#[path = "."]
pub mod iomuxc {
    #[doc = "IOMUXC"]
    pub const IOMUXC: *const RegisterBlock = 0x401f_8000 as *const RegisterBlock;
    #[path = "blocks/imxrt1021/iomuxc.rs"]
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
    #[doc = "IOMUXC_GPR"]
    pub const IOMUXC_GPR: *const RegisterBlock = 0x400a_c000 as *const RegisterBlock;
    #[path = "blocks/imxrt1021/iomuxc_gpr.rs"]
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
pub mod iomuxc_snvs {
    #[doc = "IOMUXC_SNVS"]
    pub const IOMUXC_SNVS: *const RegisterBlock = 0x400a_8000 as *const RegisterBlock;
    #[path = "blocks/imxrt1021/iomuxc_snvs.rs"]
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
    #[doc = "IOMUXC"]
    pub const IOMUXC_SNVS_GPR: *const RegisterBlock = 0x400a_4000 as *const RegisterBlock;
    #[path = "blocks/imxrt1011/iomuxc_snvs_gpr.rs"]
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
pub mod kpp {
    #[doc = "KPP Registers"]
    pub const KPP: *const RegisterBlock = 0x401f_c000 as *const RegisterBlock;
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
    #[doc = "LPI2C"]
    pub const LPI2C1: *const RegisterBlock = 0x403f_0000 as *const RegisterBlock;
    #[doc = "LPI2C"]
    pub const LPI2C2: *const RegisterBlock = 0x403f_4000 as *const RegisterBlock;
    #[doc = "LPI2C"]
    pub const LPI2C3: *const RegisterBlock = 0x403f_8000 as *const RegisterBlock;
    #[doc = "LPI2C"]
    pub const LPI2C4: *const RegisterBlock = 0x403f_c000 as *const RegisterBlock;
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
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        [(LPI2C1, 1), (LPI2C2, 2), (LPI2C3, 3), (LPI2C4, 4)]
            .into_iter()
            .find(|(ptr, _)| core::ptr::eq(rb, *ptr))
            .map(|(_, inst)| inst)
    }
}
#[path = "."]
pub mod lpspi {
    #[doc = "LPSPI"]
    pub const LPSPI1: *const RegisterBlock = 0x4039_4000 as *const RegisterBlock;
    #[doc = "LPSPI"]
    pub const LPSPI2: *const RegisterBlock = 0x4039_8000 as *const RegisterBlock;
    #[doc = "LPSPI"]
    pub const LPSPI3: *const RegisterBlock = 0x4039_c000 as *const RegisterBlock;
    #[doc = "LPSPI"]
    pub const LPSPI4: *const RegisterBlock = 0x403a_0000 as *const RegisterBlock;
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
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        [(LPSPI1, 1), (LPSPI2, 2), (LPSPI3, 3), (LPSPI4, 4)]
            .into_iter()
            .find(|(ptr, _)| core::ptr::eq(rb, *ptr))
            .map(|(_, inst)| inst)
    }
}
#[path = "."]
pub mod lpuart {
    #[doc = "LPUART"]
    pub const LPUART1: *const RegisterBlock = 0x4018_4000 as *const RegisterBlock;
    #[doc = "LPUART"]
    pub const LPUART2: *const RegisterBlock = 0x4018_8000 as *const RegisterBlock;
    #[doc = "LPUART"]
    pub const LPUART3: *const RegisterBlock = 0x4018_c000 as *const RegisterBlock;
    #[doc = "LPUART"]
    pub const LPUART4: *const RegisterBlock = 0x4019_0000 as *const RegisterBlock;
    #[doc = "LPUART"]
    pub const LPUART5: *const RegisterBlock = 0x4019_4000 as *const RegisterBlock;
    #[doc = "LPUART"]
    pub const LPUART6: *const RegisterBlock = 0x4019_8000 as *const RegisterBlock;
    #[doc = "LPUART"]
    pub const LPUART7: *const RegisterBlock = 0x4019_c000 as *const RegisterBlock;
    #[doc = "LPUART"]
    pub const LPUART8: *const RegisterBlock = 0x401a_0000 as *const RegisterBlock;
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
        ]
        .into_iter()
        .find(|(ptr, _)| core::ptr::eq(rb, *ptr))
        .map(|(_, inst)| inst)
    }
}
#[path = "."]
pub mod ocotp {
    #[doc = "no description available"]
    pub const OCOTP: *const RegisterBlock = 0x401f_4000 as *const RegisterBlock;
    #[path = "blocks/imxrt1021/ocotp.rs"]
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
pub mod pgc {
    #[doc = "PGC"]
    pub const PGC: *const RegisterBlock = 0x400f_4000 as *const RegisterBlock;
    #[path = "blocks/imxrt1011/pgc.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type PGC = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for PGC {}
    impl crate::Valid for PGC {}
    impl PGC {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(PGC)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, PGC).then_some(0)
    }
}
#[path = "."]
pub mod pit {
    #[doc = "PIT"]
    pub const PIT: *const RegisterBlock = 0x4008_4000 as *const RegisterBlock;
    #[path = "blocks/imxrt1011/pit.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type PIT = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for PIT {}
    impl crate::Valid for PIT {}
    impl PIT {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(PIT)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, PIT).then_some(0)
    }
}
#[path = "."]
pub mod pmu {
    #[doc = "PMU"]
    pub const PMU: *const RegisterBlock = 0x400d_8000 as *const RegisterBlock;
    #[path = "blocks/imxrt1021/pmu.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type PMU = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for PMU {}
    impl crate::Valid for PMU {}
    impl PMU {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(PMU)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, PMU).then_some(0)
    }
}
#[path = "."]
pub mod pwm {
    #[doc = "PWM"]
    pub const PWM1: *const RegisterBlock = 0x403d_c000 as *const RegisterBlock;
    #[doc = "PWM"]
    pub const PWM2: *const RegisterBlock = 0x403e_0000 as *const RegisterBlock;
    #[path = "blocks/imxrt1021/pwm.rs"]
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
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        [(PWM1, 1), (PWM2, 2)]
            .into_iter()
            .find(|(ptr, _)| core::ptr::eq(rb, *ptr))
            .map(|(_, inst)| inst)
    }
}
#[path = "."]
pub mod romc {
    #[doc = "ROMC"]
    pub const ROMC: *const RegisterBlock = 0x4018_0000 as *const RegisterBlock;
    #[path = "blocks/imxrt1011/romc.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type ROMC = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for ROMC {}
    impl crate::Valid for ROMC {}
    impl ROMC {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(ROMC)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, ROMC).then_some(0)
    }
}
#[path = "."]
pub mod rtwdog {
    #[doc = "WDOG"]
    pub const RTWDOG: *const RegisterBlock = 0x400b_c000 as *const RegisterBlock;
    #[path = "blocks/imxrt1011/rtwdog.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type RTWDOG = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for RTWDOG {}
    impl crate::Valid for RTWDOG {}
    impl RTWDOG {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(RTWDOG)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, RTWDOG).then_some(0)
    }
}
#[path = "."]
pub mod sai {
    #[doc = "I2S"]
    pub const SAI1: *const RegisterBlock = 0x4038_4000 as *const RegisterBlock;
    #[doc = "I2S"]
    pub const SAI2: *const RegisterBlock = 0x4038_8000 as *const RegisterBlock;
    #[doc = "I2S"]
    pub const SAI3: *const RegisterBlock = 0x4038_c000 as *const RegisterBlock;
    #[path = "blocks/imxrt1015/sai.rs"]
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
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        [(SAI1, 1), (SAI2, 2), (SAI3, 3)]
            .into_iter()
            .find(|(ptr, _)| core::ptr::eq(rb, *ptr))
            .map(|(_, inst)| inst)
    }
}
#[path = "."]
pub mod semc {
    #[doc = "SEMC"]
    pub const SEMC: *const RegisterBlock = 0x402f_0000 as *const RegisterBlock;
    #[path = "blocks/imxrt1021/semc.rs"]
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
    pub const SNVS: *const RegisterBlock = 0x400d_4000 as *const RegisterBlock;
    #[path = "blocks/imxrt1011/snvs.rs"]
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
    pub const SPDIF: *const RegisterBlock = 0x4038_0000 as *const RegisterBlock;
    #[path = "blocks/imxrt1021/spdif.rs"]
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
pub mod src {
    #[doc = "SRC"]
    pub const SRC: *const RegisterBlock = 0x400f_8000 as *const RegisterBlock;
    #[path = "blocks/imxrt1015/src.rs"]
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
pub mod tempmon {
    #[doc = "Temperature Monitor"]
    pub const TEMPMON: *const RegisterBlock = 0x400d_8000 as *const RegisterBlock;
    #[path = "blocks/imxrt1011/tempmon.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type TEMPMON = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for TEMPMON {}
    impl crate::Valid for TEMPMON {}
    impl TEMPMON {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(TEMPMON)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, TEMPMON).then_some(0)
    }
}
#[path = "."]
pub mod tmr {
    #[doc = "Quad Timer"]
    pub const TMR1: *const RegisterBlock = 0x401d_c000 as *const RegisterBlock;
    #[doc = "Quad Timer"]
    pub const TMR2: *const RegisterBlock = 0x401e_0000 as *const RegisterBlock;
    #[path = "blocks/imxrt1021/tmr.rs"]
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
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        [(TMR1, 1), (TMR2, 2)]
            .into_iter()
            .find(|(ptr, _)| core::ptr::eq(rb, *ptr))
            .map(|(_, inst)| inst)
    }
}
#[path = "."]
pub mod trng {
    #[doc = "TRNG"]
    pub const TRNG: *const RegisterBlock = 0x400c_c000 as *const RegisterBlock;
    #[path = "blocks/imxrt1011/trng.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type TRNG = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for TRNG {}
    impl crate::Valid for TRNG {}
    impl TRNG {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(TRNG)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, TRNG).then_some(0)
    }
}
#[path = "."]
pub mod usb {
    #[doc = "USB"]
    pub const USB: *const RegisterBlock = 0x402e_0000 as *const RegisterBlock;
    #[path = "blocks/imxrt1011/usb.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type USB = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for USB {}
    impl crate::Valid for USB {}
    impl USB {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(USB)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, USB).then_some(0)
    }
}
#[path = "."]
pub mod usb_analog {
    #[doc = "USB Analog"]
    pub const USB_ANALOG: *const RegisterBlock = 0x400d_8000 as *const RegisterBlock;
    #[path = "blocks/imxrt1015/usb_analog.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type USB_ANALOG = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for USB_ANALOG {}
    impl crate::Valid for USB_ANALOG {}
    impl USB_ANALOG {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(USB_ANALOG)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, USB_ANALOG).then_some(0)
    }
}
#[path = "."]
pub mod usbnc {
    #[doc = "USB"]
    pub const USBNC: *const RegisterBlock = 0x402e_0000 as *const RegisterBlock;
    #[path = "blocks/imxrt1011/usbnc.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type USBNC = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for USBNC {}
    impl crate::Valid for USBNC {}
    impl USBNC {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(USBNC)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, USBNC).then_some(0)
    }
}
#[path = "."]
pub mod usbphy {
    #[doc = "USBPHY Register Reference Index"]
    pub const USBPHY: *const RegisterBlock = 0x400d_9000 as *const RegisterBlock;
    #[path = "blocks/imxrt1011/usbphy.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type USBPHY = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for USBPHY {}
    impl crate::Valid for USBPHY {}
    impl USBPHY {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(USBPHY)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, USBPHY).then_some(0)
    }
}
#[path = "."]
pub mod usdhc {
    #[doc = "uSDHC"]
    pub const USDHC1: *const RegisterBlock = 0x402c_0000 as *const RegisterBlock;
    #[doc = "uSDHC"]
    pub const USDHC2: *const RegisterBlock = 0x402c_4000 as *const RegisterBlock;
    #[path = "blocks/imxrt1021/usdhc.rs"]
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
pub mod wdog {
    #[doc = "WDOG"]
    pub const WDOG1: *const RegisterBlock = 0x400b_8000 as *const RegisterBlock;
    #[doc = "WDOG"]
    pub const WDOG2: *const RegisterBlock = 0x400d_0000 as *const RegisterBlock;
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
pub mod xbara {
    #[doc = "Crossbar Switch"]
    pub const XBARA: *const RegisterBlock = 0x403b_c000 as *const RegisterBlock;
    #[path = "blocks/imxrt1011/xbara.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type XBARA = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for XBARA {}
    impl crate::Valid for XBARA {}
    impl XBARA {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(XBARA)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, XBARA).then_some(0)
    }
}
#[path = "."]
pub mod xbarb {
    #[doc = "Crossbar Switch"]
    pub const XBARB: *const RegisterBlock = 0x403c_0000 as *const RegisterBlock;
    #[path = "blocks/imxrt1015/xbarb.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type XBARB = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for XBARB {}
    impl crate::Valid for XBARB {}
    impl XBARB {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(XBARB)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, XBARB).then_some(0)
    }
}
#[path = "."]
pub mod xtalosc24m {
    #[doc = "XTALOSC24M"]
    pub const XTALOSC24M: *const RegisterBlock = 0x400d_8000 as *const RegisterBlock;
    #[path = "blocks/imxrt1011/xtalosc24m.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type XTALOSC24M = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for XTALOSC24M {}
    impl crate::Valid for XTALOSC24M {}
    impl XTALOSC24M {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(XTALOSC24M)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, XTALOSC24M).then_some(0)
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
    pub ADC_ETC: adc_etc::ADC_ETC,
    pub AIPSTZ1: aipstz::AIPSTZ1,
    pub AIPSTZ2: aipstz::AIPSTZ2,
    pub AIPSTZ3: aipstz::AIPSTZ3,
    pub AIPSTZ4: aipstz::AIPSTZ4,
    pub AOI: aoi::AOI,
    pub BEE: bee::BEE,
    pub CAN1: can::CAN1,
    pub CAN2: can::CAN2,
    pub CCM: ccm::CCM,
    pub CCM_ANALOG: ccm_analog::CCM_ANALOG,
    pub CMP1: cmp::CMP1,
    pub CMP2: cmp::CMP2,
    pub CMP3: cmp::CMP3,
    pub CMP4: cmp::CMP4,
    pub CSU: csu::CSU,
    pub DCDC: dcdc::DCDC,
    pub DCP: dcp::DCP,
    pub DMA: dma::DMA,
    pub DMAMUX: dmamux::DMAMUX,
    pub ENC1: enc::ENC1,
    pub ENC2: enc::ENC2,
    pub ENET: enet::ENET,
    pub EWM: ewm::EWM,
    pub FLEXIO1: flexio1::FLEXIO1,
    pub FLEXRAM: flexram::FLEXRAM,
    pub FLEXSPI: flexspi::FLEXSPI,
    pub GPC: gpc::GPC,
    pub GPIO1: gpio::GPIO1,
    pub GPIO5: gpio::GPIO5,
    pub GPIO2: gpio::GPIO2,
    pub GPIO3: gpio::GPIO3,
    pub GPT1: gpt::GPT1,
    pub GPT2: gpt::GPT2,
    pub IOMUXC: iomuxc::IOMUXC,
    pub IOMUXC_GPR: iomuxc_gpr::IOMUXC_GPR,
    pub IOMUXC_SNVS: iomuxc_snvs::IOMUXC_SNVS,
    pub IOMUXC_SNVS_GPR: iomuxc_snvs_gpr::IOMUXC_SNVS_GPR,
    pub KPP: kpp::KPP,
    pub LPI2C1: lpi2c::LPI2C1,
    pub LPI2C2: lpi2c::LPI2C2,
    pub LPI2C3: lpi2c::LPI2C3,
    pub LPI2C4: lpi2c::LPI2C4,
    pub LPSPI1: lpspi::LPSPI1,
    pub LPSPI2: lpspi::LPSPI2,
    pub LPSPI3: lpspi::LPSPI3,
    pub LPSPI4: lpspi::LPSPI4,
    pub LPUART1: lpuart::LPUART1,
    pub LPUART2: lpuart::LPUART2,
    pub LPUART3: lpuart::LPUART3,
    pub LPUART4: lpuart::LPUART4,
    pub LPUART5: lpuart::LPUART5,
    pub LPUART6: lpuart::LPUART6,
    pub LPUART7: lpuart::LPUART7,
    pub LPUART8: lpuart::LPUART8,
    pub OCOTP: ocotp::OCOTP,
    pub PGC: pgc::PGC,
    pub PIT: pit::PIT,
    pub PMU: pmu::PMU,
    pub PWM1: pwm::PWM1,
    pub PWM2: pwm::PWM2,
    pub ROMC: romc::ROMC,
    pub RTWDOG: rtwdog::RTWDOG,
    pub SAI1: sai::SAI1,
    pub SAI2: sai::SAI2,
    pub SAI3: sai::SAI3,
    pub SEMC: semc::SEMC,
    pub SNVS: snvs::SNVS,
    pub SPDIF: spdif::SPDIF,
    pub SRC: src::SRC,
    pub TEMPMON: tempmon::TEMPMON,
    pub TMR1: tmr::TMR1,
    pub TMR2: tmr::TMR2,
    pub TRNG: trng::TRNG,
    pub USB: usb::USB,
    pub USB_ANALOG: usb_analog::USB_ANALOG,
    pub USBNC: usbnc::USBNC,
    pub USBPHY: usbphy::USBPHY,
    pub USDHC1: usdhc::USDHC1,
    pub USDHC2: usdhc::USDHC2,
    pub WDOG1: wdog::WDOG1,
    pub WDOG2: wdog::WDOG2,
    pub XBARA: xbara::XBARA,
    pub XBARB: xbarb::XBARB,
    pub XTALOSC24M: xtalosc24m::XTALOSC24M,
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
            ADC_ETC: adc_etc::ADC_ETC::instance(),
            AIPSTZ1: aipstz::AIPSTZ1::instance(),
            AIPSTZ2: aipstz::AIPSTZ2::instance(),
            AIPSTZ3: aipstz::AIPSTZ3::instance(),
            AIPSTZ4: aipstz::AIPSTZ4::instance(),
            AOI: aoi::AOI::instance(),
            BEE: bee::BEE::instance(),
            CAN1: can::CAN1::instance(),
            CAN2: can::CAN2::instance(),
            CCM: ccm::CCM::instance(),
            CCM_ANALOG: ccm_analog::CCM_ANALOG::instance(),
            CMP1: cmp::CMP1::instance(),
            CMP2: cmp::CMP2::instance(),
            CMP3: cmp::CMP3::instance(),
            CMP4: cmp::CMP4::instance(),
            CSU: csu::CSU::instance(),
            DCDC: dcdc::DCDC::instance(),
            DCP: dcp::DCP::instance(),
            DMA: dma::DMA::instance(),
            DMAMUX: dmamux::DMAMUX::instance(),
            ENC1: enc::ENC1::instance(),
            ENC2: enc::ENC2::instance(),
            ENET: enet::ENET::instance(),
            EWM: ewm::EWM::instance(),
            FLEXIO1: flexio1::FLEXIO1::instance(),
            FLEXRAM: flexram::FLEXRAM::instance(),
            FLEXSPI: flexspi::FLEXSPI::instance(),
            GPC: gpc::GPC::instance(),
            GPIO1: gpio::GPIO1::instance(),
            GPIO5: gpio::GPIO5::instance(),
            GPIO2: gpio::GPIO2::instance(),
            GPIO3: gpio::GPIO3::instance(),
            GPT1: gpt::GPT1::instance(),
            GPT2: gpt::GPT2::instance(),
            IOMUXC: iomuxc::IOMUXC::instance(),
            IOMUXC_GPR: iomuxc_gpr::IOMUXC_GPR::instance(),
            IOMUXC_SNVS: iomuxc_snvs::IOMUXC_SNVS::instance(),
            IOMUXC_SNVS_GPR: iomuxc_snvs_gpr::IOMUXC_SNVS_GPR::instance(),
            KPP: kpp::KPP::instance(),
            LPI2C1: lpi2c::LPI2C1::instance(),
            LPI2C2: lpi2c::LPI2C2::instance(),
            LPI2C3: lpi2c::LPI2C3::instance(),
            LPI2C4: lpi2c::LPI2C4::instance(),
            LPSPI1: lpspi::LPSPI1::instance(),
            LPSPI2: lpspi::LPSPI2::instance(),
            LPSPI3: lpspi::LPSPI3::instance(),
            LPSPI4: lpspi::LPSPI4::instance(),
            LPUART1: lpuart::LPUART1::instance(),
            LPUART2: lpuart::LPUART2::instance(),
            LPUART3: lpuart::LPUART3::instance(),
            LPUART4: lpuart::LPUART4::instance(),
            LPUART5: lpuart::LPUART5::instance(),
            LPUART6: lpuart::LPUART6::instance(),
            LPUART7: lpuart::LPUART7::instance(),
            LPUART8: lpuart::LPUART8::instance(),
            OCOTP: ocotp::OCOTP::instance(),
            PGC: pgc::PGC::instance(),
            PIT: pit::PIT::instance(),
            PMU: pmu::PMU::instance(),
            PWM1: pwm::PWM1::instance(),
            PWM2: pwm::PWM2::instance(),
            ROMC: romc::ROMC::instance(),
            RTWDOG: rtwdog::RTWDOG::instance(),
            SAI1: sai::SAI1::instance(),
            SAI2: sai::SAI2::instance(),
            SAI3: sai::SAI3::instance(),
            SEMC: semc::SEMC::instance(),
            SNVS: snvs::SNVS::instance(),
            SPDIF: spdif::SPDIF::instance(),
            SRC: src::SRC::instance(),
            TEMPMON: tempmon::TEMPMON::instance(),
            TMR1: tmr::TMR1::instance(),
            TMR2: tmr::TMR2::instance(),
            TRNG: trng::TRNG::instance(),
            USB: usb::USB::instance(),
            USB_ANALOG: usb_analog::USB_ANALOG::instance(),
            USBNC: usbnc::USBNC::instance(),
            USBPHY: usbphy::USBPHY::instance(),
            USDHC1: usdhc::USDHC1::instance(),
            USDHC2: usdhc::USDHC2::instance(),
            WDOG1: wdog::WDOG1::instance(),
            WDOG2: wdog::WDOG2::instance(),
            XBARA: xbara::XBARA::instance(),
            XBARB: xbarb::XBARB::instance(),
            XTALOSC24M: xtalosc24m::XTALOSC24M::instance(),
        }
    }
}
