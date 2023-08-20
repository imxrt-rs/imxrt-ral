#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Interrupt {
    #[doc = "0 - DMA0"]
    DMA0 = 0,
    #[doc = "1 - DMA1"]
    DMA1 = 1,
    #[doc = "2 - DMA2"]
    DMA2 = 2,
    #[doc = "3 - DMA3"]
    DMA3 = 3,
    #[doc = "4 - DMA4"]
    DMA4 = 4,
    #[doc = "5 - DMA5"]
    DMA5 = 5,
    #[doc = "6 - DMA6"]
    DMA6 = 6,
    #[doc = "7 - DMA7"]
    DMA7 = 7,
    #[doc = "8 - DMA8"]
    DMA8 = 8,
    #[doc = "9 - DMA9"]
    DMA9 = 9,
    #[doc = "10 - DMA10"]
    DMA10 = 10,
    #[doc = "11 - DMA11"]
    DMA11 = 11,
    #[doc = "12 - DMA12"]
    DMA12 = 12,
    #[doc = "13 - DMA13"]
    DMA13 = 13,
    #[doc = "14 - DMA14"]
    DMA14 = 14,
    #[doc = "15 - DMA15"]
    DMA15 = 15,
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
    #[doc = "24 - PIT"]
    PIT = 24,
    #[doc = "25 - USB_OTG1"]
    USB_OTG1 = 25,
    #[doc = "26 - FLEXSPI"]
    FLEXSPI = 26,
    #[doc = "27 - FLEXRAM"]
    FLEXRAM = 27,
    #[doc = "28 - LPI2C1"]
    LPI2C1 = 28,
    #[doc = "29 - LPI2C2"]
    LPI2C2 = 29,
    #[doc = "30 - GPT1"]
    GPT1 = 30,
    #[doc = "31 - GPT2"]
    GPT2 = 31,
    #[doc = "32 - LPSPI1"]
    LPSPI1 = 32,
    #[doc = "33 - LPSPI2"]
    LPSPI2 = 33,
    #[doc = "34 - PWM1_0"]
    PWM1_0 = 34,
    #[doc = "35 - PWM1_1"]
    PWM1_1 = 35,
    #[doc = "36 - PWM1_2"]
    PWM1_2 = 36,
    #[doc = "37 - PWM1_3"]
    PWM1_3 = 37,
    #[doc = "38 - PWM1_FAULT"]
    PWM1_FAULT = 38,
    #[doc = "39 - KPP"]
    KPP = 39,
    #[doc = "40 - SRC"]
    SRC = 40,
    #[doc = "41 - GPR (aka \"GPC\") interrupt request"]
    GPR_IRQ = 41,
    #[doc = "42 - CCM_1"]
    CCM_1 = 42,
    #[doc = "43 - CCM_2"]
    CCM_2 = 43,
    #[doc = "44 - EWM"]
    EWM = 44,
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
    #[doc = "56 - SAI1"]
    SAI1 = 56,
    #[doc = "57 - RTWDOG"]
    RTWDOG = 57,
    #[doc = "58 - SAI3_RX"]
    SAI3_RX = 58,
    #[doc = "59 - SAI3_TX"]
    SAI3_TX = 59,
    #[doc = "60 - SPDIF"]
    SPDIF = 60,
    #[doc = "61 - PMU"]
    PMU = 61,
    #[doc = "62 - XBAR1_IRQ_0_1_2_3"]
    XBAR1_IRQ_0_1_2_3 = 62,
    #[doc = "63 - TEMP_LOW_HIGH"]
    TEMP_LOW_HIGH = 63,
    #[doc = "64 - TEMP_PANIC"]
    TEMP_PANIC = 64,
    #[doc = "65 - USB_PHY"]
    USB_PHY = 65,
    #[doc = "66 - GPC"]
    GPC = 66,
    #[doc = "67 - ADC1"]
    ADC1 = 67,
    #[doc = "68 - FLEXIO1"]
    FLEXIO1 = 68,
    #[doc = "69 - DCDC"]
    DCDC = 69,
    #[doc = "70 - GPIO1_COMBINED_0_15"]
    GPIO1_COMBINED_0_15 = 70,
    #[doc = "71 - GPIO1_COMBINED_16_31"]
    GPIO1_COMBINED_16_31 = 71,
    #[doc = "72 - GPIO2_COMBINED_0_15"]
    GPIO2_COMBINED_0_15 = 72,
    #[doc = "73 - GPIO5_COMBINED_0_15"]
    GPIO5_COMBINED_0_15 = 73,
    #[doc = "74 - WDOG1"]
    WDOG1 = 74,
    #[doc = "75 - ADC_ETC_IRQ0"]
    ADC_ETC_IRQ0 = 75,
    #[doc = "76 - ADC_ETC_IRQ1"]
    ADC_ETC_IRQ1 = 76,
    #[doc = "77 - ADC_ETC_IRQ2"]
    ADC_ETC_IRQ2 = 77,
    #[doc = "78 - ADC_ETC_IRQ3"]
    ADC_ETC_IRQ3 = 78,
    #[doc = "79 - ADC_ETC_ERROR_IRQ"]
    ADC_ETC_ERROR_IRQ = 79,
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
        fn DMA0();
        fn DMA1();
        fn DMA2();
        fn DMA3();
        fn DMA4();
        fn DMA5();
        fn DMA6();
        fn DMA7();
        fn DMA8();
        fn DMA9();
        fn DMA10();
        fn DMA11();
        fn DMA12();
        fn DMA13();
        fn DMA14();
        fn DMA15();
        fn DMA_ERROR();
        fn LPUART1();
        fn LPUART2();
        fn LPUART3();
        fn LPUART4();
        fn PIT();
        fn USB_OTG1();
        fn FLEXSPI();
        fn FLEXRAM();
        fn LPI2C1();
        fn LPI2C2();
        fn GPT1();
        fn GPT2();
        fn LPSPI1();
        fn LPSPI2();
        fn PWM1_0();
        fn PWM1_1();
        fn PWM1_2();
        fn PWM1_3();
        fn PWM1_FAULT();
        fn KPP();
        fn SRC();
        fn GPR_IRQ();
        fn CCM_1();
        fn CCM_2();
        fn EWM();
        fn WDOG2();
        fn SNVS_HP_WRAPPER();
        fn SNVS_HP_WRAPPER_TZ();
        fn SNVS_LP_WRAPPER();
        fn CSU();
        fn DCP();
        fn DCP_VMI();
        fn TRNG();
        fn SAI1();
        fn RTWDOG();
        fn SAI3_RX();
        fn SAI3_TX();
        fn SPDIF();
        fn PMU();
        fn XBAR1_IRQ_0_1_2_3();
        fn TEMP_LOW_HIGH();
        fn TEMP_PANIC();
        fn USB_PHY();
        fn GPC();
        fn ADC1();
        fn FLEXIO1();
        fn DCDC();
        fn GPIO1_COMBINED_0_15();
        fn GPIO1_COMBINED_16_31();
        fn GPIO2_COMBINED_0_15();
        fn GPIO5_COMBINED_0_15();
        fn WDOG1();
        fn ADC_ETC_IRQ0();
        fn ADC_ETC_IRQ1();
        fn ADC_ETC_IRQ2();
        fn ADC_ETC_IRQ3();
        fn ADC_ETC_ERROR_IRQ();
    }
    pub union Vector {
        _handler: unsafe extern "C" fn(),
        _reserved: u32,
    }
    #[cfg_attr(target_os = "none", link_section = ".vector_table.interrupts")]
    #[no_mangle]
    pub static __INTERRUPTS: [Vector; 80] = [
        Vector { _handler: DMA0 },
        Vector { _handler: DMA1 },
        Vector { _handler: DMA2 },
        Vector { _handler: DMA3 },
        Vector { _handler: DMA4 },
        Vector { _handler: DMA5 },
        Vector { _handler: DMA6 },
        Vector { _handler: DMA7 },
        Vector { _handler: DMA8 },
        Vector { _handler: DMA9 },
        Vector { _handler: DMA10 },
        Vector { _handler: DMA11 },
        Vector { _handler: DMA12 },
        Vector { _handler: DMA13 },
        Vector { _handler: DMA14 },
        Vector { _handler: DMA15 },
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
        Vector { _handler: PIT },
        Vector { _handler: USB_OTG1 },
        Vector { _handler: FLEXSPI },
        Vector { _handler: FLEXRAM },
        Vector { _handler: LPI2C1 },
        Vector { _handler: LPI2C2 },
        Vector { _handler: GPT1 },
        Vector { _handler: GPT2 },
        Vector { _handler: LPSPI1 },
        Vector { _handler: LPSPI2 },
        Vector { _handler: PWM1_0 },
        Vector { _handler: PWM1_1 },
        Vector { _handler: PWM1_2 },
        Vector { _handler: PWM1_3 },
        Vector {
            _handler: PWM1_FAULT,
        },
        Vector { _handler: KPP },
        Vector { _handler: SRC },
        Vector { _handler: GPR_IRQ },
        Vector { _handler: CCM_1 },
        Vector { _handler: CCM_2 },
        Vector { _handler: EWM },
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
        Vector { _reserved: 0 },
        Vector { _handler: SAI1 },
        Vector { _handler: RTWDOG },
        Vector { _handler: SAI3_RX },
        Vector { _handler: SAI3_TX },
        Vector { _handler: SPDIF },
        Vector { _handler: PMU },
        Vector {
            _handler: XBAR1_IRQ_0_1_2_3,
        },
        Vector {
            _handler: TEMP_LOW_HIGH,
        },
        Vector {
            _handler: TEMP_PANIC,
        },
        Vector { _handler: USB_PHY },
        Vector { _handler: GPC },
        Vector { _handler: ADC1 },
        Vector { _handler: FLEXIO1 },
        Vector { _handler: DCDC },
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
            _handler: GPIO5_COMBINED_0_15,
        },
        Vector { _handler: WDOG1 },
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
    ];
}
#[path = "."]
pub mod adc {
    #[doc = "Analog-to-Digital Converter"]
    pub const ADC: *const RegisterBlock = 0x400c_4000 as *const RegisterBlock;
    #[path = "blocks/imxrt1011/adc.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type ADC = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for ADC {}
    impl crate::Valid for ADC {}
    impl ADC {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(ADC)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, ADC).then_some(0)
    }
}
#[path = "."]
pub mod adc_etc {
    #[doc = "ADC_ETC"]
    pub const ADC_ETC: *const RegisterBlock = 0x4008_8000 as *const RegisterBlock;
    #[path = "blocks/imxrt1011/adc_etc.rs"]
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
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        [(AIPSTZ1, 1), (AIPSTZ2, 2)]
            .into_iter()
            .find(|(ptr, _)| core::ptr::eq(rb, *ptr))
            .map(|(_, inst)| inst)
    }
}
#[path = "."]
pub mod aoi {
    #[doc = "AND/OR/INVERT module"]
    pub const AOI: *const RegisterBlock = 0x4009_4000 as *const RegisterBlock;
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
pub mod ccm {
    #[doc = "CCM"]
    pub const CCM: *const RegisterBlock = 0x400f_c000 as *const RegisterBlock;
    #[path = "blocks/imxrt1011/ccm.rs"]
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
    #[path = "blocks/imxrt1011/ccm_analog.rs"]
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
    pub const DCP: *const RegisterBlock = 0x400f_0000 as *const RegisterBlock;
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
    #[path = "blocks/imxrt1011/dma.rs"]
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
    #[path = "blocks/imxrt1011/dmamux.rs"]
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
    #[path = "blocks/imxrt1011/flexram.rs"]
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
    pub const FLEXSPI: *const RegisterBlock = 0x400a_0000 as *const RegisterBlock;
    #[path = "blocks/imxrt1011/flexspi.rs"]
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
    pub const GPIO2: *const RegisterBlock = 0x4200_0000 as *const RegisterBlock;
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
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        [(GPIO1, 1), (GPIO5, 5), (GPIO2, 2)]
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
    #[path = "blocks/imxrt1011/iomuxc.rs"]
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
    #[path = "blocks/imxrt1011/iomuxc_gpr.rs"]
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
    #[path = "blocks/imxrt1011/iomuxc_snvs.rs"]
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
    pub const LPI2C1: *const RegisterBlock = 0x401a_4000 as *const RegisterBlock;
    #[doc = "LPI2C"]
    pub const LPI2C2: *const RegisterBlock = 0x401a_8000 as *const RegisterBlock;
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
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        [(LPI2C1, 1), (LPI2C2, 2)]
            .into_iter()
            .find(|(ptr, _)| core::ptr::eq(rb, *ptr))
            .map(|(_, inst)| inst)
    }
}
#[path = "."]
pub mod lpspi {
    #[doc = "LPSPI"]
    pub const LPSPI1: *const RegisterBlock = 0x4019_4000 as *const RegisterBlock;
    #[doc = "LPSPI"]
    pub const LPSPI2: *const RegisterBlock = 0x4019_8000 as *const RegisterBlock;
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
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        [(LPSPI1, 1), (LPSPI2, 2)]
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
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        [(LPUART1, 1), (LPUART2, 2), (LPUART3, 3), (LPUART4, 4)]
            .into_iter()
            .find(|(ptr, _)| core::ptr::eq(rb, *ptr))
            .map(|(_, inst)| inst)
    }
}
#[path = "."]
pub mod ocotp {
    #[doc = "no description available"]
    pub const OCOTP: *const RegisterBlock = 0x401f_4000 as *const RegisterBlock;
    #[path = "blocks/imxrt1011/ocotp.rs"]
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
pub mod otfad {
    #[doc = "OTFAD"]
    pub const OTFAD: *const RegisterBlock = 0x400a_0000 as *const RegisterBlock;
    #[path = "blocks/imxrt1011/otfad.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type OTFAD = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for OTFAD {}
    impl crate::Valid for OTFAD {}
    impl OTFAD {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(OTFAD)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, OTFAD).then_some(0)
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
    #[path = "blocks/imxrt1011/pmu.rs"]
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
    pub const PWM: *const RegisterBlock = 0x401c_c000 as *const RegisterBlock;
    #[path = "blocks/imxrt1011/pwm.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type PWM = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for PWM {}
    impl crate::Valid for PWM {}
    impl PWM {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(PWM)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, PWM).then_some(0)
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
    pub const SAI1: *const RegisterBlock = 0x401e_0000 as *const RegisterBlock;
    #[doc = "I2S"]
    pub const SAI3: *const RegisterBlock = 0x401e_8000 as *const RegisterBlock;
    #[path = "blocks/imxrt1011/sai.rs"]
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
        [(SAI1, 1), (SAI3, 3)]
            .into_iter()
            .find(|(ptr, _)| core::ptr::eq(rb, *ptr))
            .map(|(_, inst)| inst)
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
    pub const SPDIF: *const RegisterBlock = 0x401d_c000 as *const RegisterBlock;
    #[path = "blocks/imxrt1011/spdif.rs"]
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
    #[path = "blocks/imxrt1011/src.rs"]
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
    pub const USB: *const RegisterBlock = 0x400e_4000 as *const RegisterBlock;
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
    #[path = "blocks/imxrt1011/usb_analog.rs"]
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
    pub const USBNC: *const RegisterBlock = 0x400e_4000 as *const RegisterBlock;
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
    pub const XBARA: *const RegisterBlock = 0x4009_8000 as *const RegisterBlock;
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
    pub ADC: adc::ADC,
    pub ADC_ETC: adc_etc::ADC_ETC,
    pub AIPSTZ1: aipstz::AIPSTZ1,
    pub AIPSTZ2: aipstz::AIPSTZ2,
    pub AOI: aoi::AOI,
    pub CCM: ccm::CCM,
    pub CCM_ANALOG: ccm_analog::CCM_ANALOG,
    pub CSU: csu::CSU,
    pub DCDC: dcdc::DCDC,
    pub DCP: dcp::DCP,
    pub DMA: dma::DMA,
    pub DMAMUX: dmamux::DMAMUX,
    pub EWM: ewm::EWM,
    pub FLEXIO1: flexio1::FLEXIO1,
    pub FLEXRAM: flexram::FLEXRAM,
    pub FLEXSPI: flexspi::FLEXSPI,
    pub GPC: gpc::GPC,
    pub GPIO1: gpio::GPIO1,
    pub GPIO5: gpio::GPIO5,
    pub GPIO2: gpio::GPIO2,
    pub GPT1: gpt::GPT1,
    pub GPT2: gpt::GPT2,
    pub IOMUXC: iomuxc::IOMUXC,
    pub IOMUXC_GPR: iomuxc_gpr::IOMUXC_GPR,
    pub IOMUXC_SNVS: iomuxc_snvs::IOMUXC_SNVS,
    pub IOMUXC_SNVS_GPR: iomuxc_snvs_gpr::IOMUXC_SNVS_GPR,
    pub KPP: kpp::KPP,
    pub LPI2C1: lpi2c::LPI2C1,
    pub LPI2C2: lpi2c::LPI2C2,
    pub LPSPI1: lpspi::LPSPI1,
    pub LPSPI2: lpspi::LPSPI2,
    pub LPUART1: lpuart::LPUART1,
    pub LPUART2: lpuart::LPUART2,
    pub LPUART3: lpuart::LPUART3,
    pub LPUART4: lpuart::LPUART4,
    pub OCOTP: ocotp::OCOTP,
    pub OTFAD: otfad::OTFAD,
    pub PGC: pgc::PGC,
    pub PIT: pit::PIT,
    pub PMU: pmu::PMU,
    pub PWM: pwm::PWM,
    pub ROMC: romc::ROMC,
    pub RTWDOG: rtwdog::RTWDOG,
    pub SAI1: sai::SAI1,
    pub SAI3: sai::SAI3,
    pub SNVS: snvs::SNVS,
    pub SPDIF: spdif::SPDIF,
    pub SRC: src::SRC,
    pub TEMPMON: tempmon::TEMPMON,
    pub TRNG: trng::TRNG,
    pub USB: usb::USB,
    pub USB_ANALOG: usb_analog::USB_ANALOG,
    pub USBNC: usbnc::USBNC,
    pub USBPHY: usbphy::USBPHY,
    pub WDOG1: wdog::WDOG1,
    pub WDOG2: wdog::WDOG2,
    pub XBARA: xbara::XBARA,
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
            ADC: adc::ADC::instance(),
            ADC_ETC: adc_etc::ADC_ETC::instance(),
            AIPSTZ1: aipstz::AIPSTZ1::instance(),
            AIPSTZ2: aipstz::AIPSTZ2::instance(),
            AOI: aoi::AOI::instance(),
            CCM: ccm::CCM::instance(),
            CCM_ANALOG: ccm_analog::CCM_ANALOG::instance(),
            CSU: csu::CSU::instance(),
            DCDC: dcdc::DCDC::instance(),
            DCP: dcp::DCP::instance(),
            DMA: dma::DMA::instance(),
            DMAMUX: dmamux::DMAMUX::instance(),
            EWM: ewm::EWM::instance(),
            FLEXIO1: flexio1::FLEXIO1::instance(),
            FLEXRAM: flexram::FLEXRAM::instance(),
            FLEXSPI: flexspi::FLEXSPI::instance(),
            GPC: gpc::GPC::instance(),
            GPIO1: gpio::GPIO1::instance(),
            GPIO5: gpio::GPIO5::instance(),
            GPIO2: gpio::GPIO2::instance(),
            GPT1: gpt::GPT1::instance(),
            GPT2: gpt::GPT2::instance(),
            IOMUXC: iomuxc::IOMUXC::instance(),
            IOMUXC_GPR: iomuxc_gpr::IOMUXC_GPR::instance(),
            IOMUXC_SNVS: iomuxc_snvs::IOMUXC_SNVS::instance(),
            IOMUXC_SNVS_GPR: iomuxc_snvs_gpr::IOMUXC_SNVS_GPR::instance(),
            KPP: kpp::KPP::instance(),
            LPI2C1: lpi2c::LPI2C1::instance(),
            LPI2C2: lpi2c::LPI2C2::instance(),
            LPSPI1: lpspi::LPSPI1::instance(),
            LPSPI2: lpspi::LPSPI2::instance(),
            LPUART1: lpuart::LPUART1::instance(),
            LPUART2: lpuart::LPUART2::instance(),
            LPUART3: lpuart::LPUART3::instance(),
            LPUART4: lpuart::LPUART4::instance(),
            OCOTP: ocotp::OCOTP::instance(),
            OTFAD: otfad::OTFAD::instance(),
            PGC: pgc::PGC::instance(),
            PIT: pit::PIT::instance(),
            PMU: pmu::PMU::instance(),
            PWM: pwm::PWM::instance(),
            ROMC: romc::ROMC::instance(),
            RTWDOG: rtwdog::RTWDOG::instance(),
            SAI1: sai::SAI1::instance(),
            SAI3: sai::SAI3::instance(),
            SNVS: snvs::SNVS::instance(),
            SPDIF: spdif::SPDIF::instance(),
            SRC: src::SRC::instance(),
            TEMPMON: tempmon::TEMPMON::instance(),
            TRNG: trng::TRNG::instance(),
            USB: usb::USB::instance(),
            USB_ANALOG: usb_analog::USB_ANALOG::instance(),
            USBNC: usbnc::USBNC::instance(),
            USBPHY: usbphy::USBPHY::instance(),
            WDOG1: wdog::WDOG1::instance(),
            WDOG2: wdog::WDOG2::instance(),
            XBARA: xbara::XBARA::instance(),
            XTALOSC24M: xtalosc24m::XTALOSC24M::instance(),
        }
    }
}
