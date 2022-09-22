#[doc = "IOMUXC SNVS"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "SW_MUX_CTL_PAD_WAKEUP_DIG SW MUX Control Register"]
    pub SW_MUX_CTL_PAD_WAKEUP_DIG: crate::RWRegister<u32>,
    #[doc = "SW_MUX_CTL_PAD_PMIC_ON_REQ_DIG SW MUX Control Register"]
    pub SW_MUX_CTL_PAD_PMIC_ON_REQ_DIG: crate::RWRegister<u32>,
    #[doc = "SW_MUX_CTL_PAD_PMIC_STBY_REQ_DIG SW MUX Control Register"]
    pub SW_MUX_CTL_PAD_PMIC_STBY_REQ_DIG: crate::RWRegister<u32>,
    #[doc = "SW_MUX_CTL_PAD_GPIO_SNVS_00_DIG SW MUX Control Register"]
    pub SW_MUX_CTL_PAD_GPIO_SNVS_00_DIG: crate::RWRegister<u32>,
    #[doc = "SW_MUX_CTL_PAD_GPIO_SNVS_01_DIG SW MUX Control Register"]
    pub SW_MUX_CTL_PAD_GPIO_SNVS_01_DIG: crate::RWRegister<u32>,
    #[doc = "SW_MUX_CTL_PAD_GPIO_SNVS_02_DIG SW MUX Control Register"]
    pub SW_MUX_CTL_PAD_GPIO_SNVS_02_DIG: crate::RWRegister<u32>,
    #[doc = "SW_MUX_CTL_PAD_GPIO_SNVS_03_DIG SW MUX Control Register"]
    pub SW_MUX_CTL_PAD_GPIO_SNVS_03_DIG: crate::RWRegister<u32>,
    #[doc = "SW_MUX_CTL_PAD_GPIO_SNVS_04_DIG SW MUX Control Register"]
    pub SW_MUX_CTL_PAD_GPIO_SNVS_04_DIG: crate::RWRegister<u32>,
    #[doc = "SW_MUX_CTL_PAD_GPIO_SNVS_05_DIG SW MUX Control Register"]
    pub SW_MUX_CTL_PAD_GPIO_SNVS_05_DIG: crate::RWRegister<u32>,
    #[doc = "SW_MUX_CTL_PAD_GPIO_SNVS_06_DIG SW MUX Control Register"]
    pub SW_MUX_CTL_PAD_GPIO_SNVS_06_DIG: crate::RWRegister<u32>,
    #[doc = "SW_MUX_CTL_PAD_GPIO_SNVS_07_DIG SW MUX Control Register"]
    pub SW_MUX_CTL_PAD_GPIO_SNVS_07_DIG: crate::RWRegister<u32>,
    #[doc = "SW_MUX_CTL_PAD_GPIO_SNVS_08_DIG SW MUX Control Register"]
    pub SW_MUX_CTL_PAD_GPIO_SNVS_08_DIG: crate::RWRegister<u32>,
    #[doc = "SW_MUX_CTL_PAD_GPIO_SNVS_09_DIG SW MUX Control Register"]
    pub SW_MUX_CTL_PAD_GPIO_SNVS_09_DIG: crate::RWRegister<u32>,
    #[doc = "SW_PAD_CTL_PAD_TEST_MODE_DIG SW PAD Control Register"]
    pub SW_PAD_CTL_PAD_TEST_MODE_DIG: crate::RWRegister<u32>,
    #[doc = "SW_PAD_CTL_PAD_POR_B_DIG SW PAD Control Register"]
    pub SW_PAD_CTL_PAD_POR_B_DIG: crate::RWRegister<u32>,
    #[doc = "SW_PAD_CTL_PAD_ONOFF_DIG SW PAD Control Register"]
    pub SW_PAD_CTL_PAD_ONOFF_DIG: crate::RWRegister<u32>,
    #[doc = "SW_PAD_CTL_PAD_WAKEUP_DIG SW PAD Control Register"]
    pub SW_PAD_CTL_PAD_WAKEUP_DIG: crate::RWRegister<u32>,
    #[doc = "SW_PAD_CTL_PAD_PMIC_ON_REQ_DIG SW PAD Control Register"]
    pub SW_PAD_CTL_PAD_PMIC_ON_REQ_DIG: crate::RWRegister<u32>,
    #[doc = "SW_PAD_CTL_PAD_PMIC_STBY_REQ_DIG SW PAD Control Register"]
    pub SW_PAD_CTL_PAD_PMIC_STBY_REQ_DIG: crate::RWRegister<u32>,
    #[doc = "SW_PAD_CTL_PAD_GPIO_SNVS_00_DIG SW PAD Control Register"]
    pub SW_PAD_CTL_PAD_GPIO_SNVS_00_DIG: crate::RWRegister<u32>,
    #[doc = "SW_PAD_CTL_PAD_GPIO_SNVS_01_DIG SW PAD Control Register"]
    pub SW_PAD_CTL_PAD_GPIO_SNVS_01_DIG: crate::RWRegister<u32>,
    #[doc = "SW_PAD_CTL_PAD_GPIO_SNVS_02_DIG SW PAD Control Register"]
    pub SW_PAD_CTL_PAD_GPIO_SNVS_02_DIG: crate::RWRegister<u32>,
    #[doc = "SW_PAD_CTL_PAD_GPIO_SNVS_03_DIG SW PAD Control Register"]
    pub SW_PAD_CTL_PAD_GPIO_SNVS_03_DIG: crate::RWRegister<u32>,
    #[doc = "SW_PAD_CTL_PAD_GPIO_SNVS_04_DIG SW PAD Control Register"]
    pub SW_PAD_CTL_PAD_GPIO_SNVS_04_DIG: crate::RWRegister<u32>,
    #[doc = "SW_PAD_CTL_PAD_GPIO_SNVS_05_DIG SW PAD Control Register"]
    pub SW_PAD_CTL_PAD_GPIO_SNVS_05_DIG: crate::RWRegister<u32>,
    #[doc = "SW_PAD_CTL_PAD_GPIO_SNVS_06_DIG SW PAD Control Register"]
    pub SW_PAD_CTL_PAD_GPIO_SNVS_06_DIG: crate::RWRegister<u32>,
    #[doc = "SW_PAD_CTL_PAD_GPIO_SNVS_07_DIG SW PAD Control Register"]
    pub SW_PAD_CTL_PAD_GPIO_SNVS_07_DIG: crate::RWRegister<u32>,
    #[doc = "SW_PAD_CTL_PAD_GPIO_SNVS_08_DIG SW PAD Control Register"]
    pub SW_PAD_CTL_PAD_GPIO_SNVS_08_DIG: crate::RWRegister<u32>,
    #[doc = "SW_PAD_CTL_PAD_GPIO_SNVS_09_DIG SW PAD Control Register"]
    pub SW_PAD_CTL_PAD_GPIO_SNVS_09_DIG: crate::RWRegister<u32>,
}
#[doc = "SW_MUX_CTL_PAD_WAKEUP_DIG SW MUX Control Register"]
pub mod SW_MUX_CTL_PAD_WAKEUP_DIG {
    #[doc = "MUX Mode Select Field."]
    pub mod MUX_MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Select mux mode: ALT5 mux port: GPIO13_IO00 of instance: GPIO13"]
            pub const ALT5_GPIO13_IO0: u32 = 0x05;
            #[doc = "Select mux mode: ALT7 mux port: NMI_GLUE_NMI of instance: NMI_GLUE"]
            pub const ALT7_NMI_GLUE_NMI: u32 = 0x07;
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
            #[doc = "Force input path of pad WAKEUP_DIG"]
            pub const ENABLED: u32 = 0x01;
        }
    }
}
#[doc = "SW_MUX_CTL_PAD_PMIC_ON_REQ_DIG SW MUX Control Register"]
pub mod SW_MUX_CTL_PAD_PMIC_ON_REQ_DIG {
    #[doc = "MUX Mode Select Field."]
    pub mod MUX_MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Select mux mode: ALT0 mux port: SNVS_LP_PMIC_ON_REQ of instance: SNVS_LP"]
            pub const ALT0_SNVS_LP_PMIC_ON_REQ: u32 = 0;
            #[doc = "Select mux mode: ALT5 mux port: GPIO13_IO01 of instance: GPIO13"]
            pub const ALT5_GPIO13_IO1: u32 = 0x05;
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
            #[doc = "Force input path of pad PMIC_ON_REQ_DIG"]
            pub const ENABLED: u32 = 0x01;
        }
    }
}
#[doc = "SW_MUX_CTL_PAD_PMIC_STBY_REQ_DIG SW MUX Control Register"]
pub mod SW_MUX_CTL_PAD_PMIC_STBY_REQ_DIG {
    #[doc = "MUX Mode Select Field."]
    pub mod MUX_MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Select mux mode: ALT0 mux port: CCM_PMIC_VSTBY_REQ of instance: CCM"]
            pub const ALT0_CCM_PMIC_VSTBY_REQ: u32 = 0;
            #[doc = "Select mux mode: ALT5 mux port: GPIO13_IO02 of instance: GPIO13"]
            pub const ALT5_GPIO13_IO2: u32 = 0x05;
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
            #[doc = "Force input path of pad PMIC_STBY_REQ_DIG"]
            pub const ENABLED: u32 = 0x01;
        }
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_SNVS_00_DIG SW MUX Control Register"]
pub mod SW_MUX_CTL_PAD_GPIO_SNVS_00_DIG {
    #[doc = "MUX Mode Select Field."]
    pub mod MUX_MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Select mux mode: ALT0 mux port: SNVS_TAMPER0 of instance: SNVS_LP"]
            pub const ALT0_SNVS_TAMPER0: u32 = 0;
            #[doc = "Select mux mode: ALT5 mux port: GPIO13_IO03 of instance: GPIO13"]
            pub const ALT5_GPIO13_IO3: u32 = 0x05;
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
            #[doc = "Force input path of pad GPIO_SNVS_00_DIG"]
            pub const ENABLED: u32 = 0x01;
        }
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_SNVS_01_DIG SW MUX Control Register"]
pub mod SW_MUX_CTL_PAD_GPIO_SNVS_01_DIG {
    #[doc = "MUX Mode Select Field."]
    pub mod MUX_MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Select mux mode: ALT0 mux port: SNVS_TAMPER1 of instance: SNVS_LP"]
            pub const ALT0_SNVS_TAMPER1: u32 = 0;
            #[doc = "Select mux mode: ALT5 mux port: GPIO13_IO04 of instance: GPIO13"]
            pub const ALT5_GPIO13_IO4: u32 = 0x05;
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
            #[doc = "Force input path of pad GPIO_SNVS_01_DIG"]
            pub const ENABLED: u32 = 0x01;
        }
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_SNVS_02_DIG SW MUX Control Register"]
pub mod SW_MUX_CTL_PAD_GPIO_SNVS_02_DIG {
    #[doc = "MUX Mode Select Field."]
    pub mod MUX_MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Select mux mode: ALT0 mux port: SNVS_TAMPER2 of instance: SNVS_LP"]
            pub const ALT0_SNVS_TAMPER2: u32 = 0;
            #[doc = "Select mux mode: ALT5 mux port: GPIO13_IO05 of instance: GPIO13"]
            pub const ALT5_GPIO13_IO5: u32 = 0x05;
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
            #[doc = "Force input path of pad GPIO_SNVS_02_DIG"]
            pub const ENABLED: u32 = 0x01;
        }
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_SNVS_03_DIG SW MUX Control Register"]
pub mod SW_MUX_CTL_PAD_GPIO_SNVS_03_DIG {
    #[doc = "MUX Mode Select Field."]
    pub mod MUX_MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Select mux mode: ALT0 mux port: SNVS_TAMPER3 of instance: SNVS_LP"]
            pub const ALT0_SNVS_TAMPER3: u32 = 0;
            #[doc = "Select mux mode: ALT5 mux port: GPIO13_IO06 of instance: GPIO13"]
            pub const ALT5_GPIO13_IO6: u32 = 0x05;
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
            #[doc = "Force input path of pad GPIO_SNVS_03_DIG"]
            pub const ENABLED: u32 = 0x01;
        }
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_SNVS_04_DIG SW MUX Control Register"]
pub mod SW_MUX_CTL_PAD_GPIO_SNVS_04_DIG {
    #[doc = "MUX Mode Select Field."]
    pub mod MUX_MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Select mux mode: ALT0 mux port: SNVS_TAMPER4 of instance: SNVS_LP"]
            pub const ALT0_SNVS_TAMPER4: u32 = 0;
            #[doc = "Select mux mode: ALT5 mux port: GPIO13_IO07 of instance: GPIO13"]
            pub const ALT5_GPIO13_IO7: u32 = 0x05;
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
            #[doc = "Force input path of pad GPIO_SNVS_04_DIG"]
            pub const ENABLED: u32 = 0x01;
        }
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_SNVS_05_DIG SW MUX Control Register"]
pub mod SW_MUX_CTL_PAD_GPIO_SNVS_05_DIG {
    #[doc = "MUX Mode Select Field."]
    pub mod MUX_MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Select mux mode: ALT0 mux port: SNVS_TAMPER5 of instance: SNVS_LP"]
            pub const ALT0_SNVS_TAMPER5: u32 = 0;
            #[doc = "Select mux mode: ALT5 mux port: GPIO13_IO08 of instance: GPIO13"]
            pub const ALT5_GPIO13_IO8: u32 = 0x05;
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
            #[doc = "Force input path of pad GPIO_SNVS_05_DIG"]
            pub const ENABLED: u32 = 0x01;
        }
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_SNVS_06_DIG SW MUX Control Register"]
pub mod SW_MUX_CTL_PAD_GPIO_SNVS_06_DIG {
    #[doc = "MUX Mode Select Field."]
    pub mod MUX_MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Select mux mode: ALT0 mux port: SNVS_TAMPER6 of instance: SNVS_LP"]
            pub const ALT0_SNVS_TAMPER6: u32 = 0;
            #[doc = "Select mux mode: ALT5 mux port: GPIO13_IO09 of instance: GPIO13"]
            pub const ALT5_GPIO13_IO9: u32 = 0x05;
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
            #[doc = "Force input path of pad GPIO_SNVS_06_DIG"]
            pub const ENABLED: u32 = 0x01;
        }
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_SNVS_07_DIG SW MUX Control Register"]
pub mod SW_MUX_CTL_PAD_GPIO_SNVS_07_DIG {
    #[doc = "MUX Mode Select Field."]
    pub mod MUX_MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Select mux mode: ALT0 mux port: SNVS_TAMPER7 of instance: SNVS_LP"]
            pub const ALT0_SNVS_TAMPER7: u32 = 0;
            #[doc = "Select mux mode: ALT5 mux port: GPIO13_IO10 of instance: GPIO13"]
            pub const ALT5_GPIO13_IO10: u32 = 0x05;
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
            #[doc = "Force input path of pad GPIO_SNVS_07_DIG"]
            pub const ENABLED: u32 = 0x01;
        }
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_SNVS_08_DIG SW MUX Control Register"]
pub mod SW_MUX_CTL_PAD_GPIO_SNVS_08_DIG {
    #[doc = "MUX Mode Select Field."]
    pub mod MUX_MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Select mux mode: ALT0 mux port: SNVS_TAMPER8 of instance: SNVS_LP"]
            pub const ALT0_SNVS_TAMPER8: u32 = 0;
            #[doc = "Select mux mode: ALT5 mux port: GPIO13_IO11 of instance: GPIO13"]
            pub const ALT5_GPIO13_IO11: u32 = 0x05;
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
            #[doc = "Force input path of pad GPIO_SNVS_08_DIG"]
            pub const ENABLED: u32 = 0x01;
        }
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_SNVS_09_DIG SW MUX Control Register"]
pub mod SW_MUX_CTL_PAD_GPIO_SNVS_09_DIG {
    #[doc = "MUX Mode Select Field."]
    pub mod MUX_MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Select mux mode: ALT0 mux port: SNVS_TAMPER9 of instance: SNVS_LP"]
            pub const ALT0_SNVS_TAMPER9: u32 = 0;
            #[doc = "Select mux mode: ALT5 mux port: GPIO13_IO12 of instance: GPIO13"]
            pub const ALT5_GPIO13_IO12: u32 = 0x05;
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
            #[doc = "Force input path of pad GPIO_SNVS_09_DIG"]
            pub const ENABLED: u32 = 0x01;
        }
    }
}
#[doc = "SW_PAD_CTL_PAD_TEST_MODE_DIG SW PAD Control Register"]
pub mod SW_PAD_CTL_PAD_TEST_MODE_DIG {
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
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "normal driver"]
            pub const DSE_0_NORMAL_DRIVER: u32 = 0;
            #[doc = "high driver"]
            pub const DSE_1_HIGH_DRIVER: u32 = 0x01;
        }
    }
    #[doc = "Pull / Keep Select Field"]
    pub mod PUE {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pull Disable"]
            pub const PUE_0_DISABLE: u32 = 0;
            #[doc = "Pull Enable"]
            pub const PUE_1_PULL: u32 = 0x01;
        }
    }
    #[doc = "Pull Up / Down Config. Field"]
    pub mod PUS {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Weak pull down"]
            pub const PUS_0_WEAK_PULL_DOWN: u32 = 0;
            #[doc = "Weak pull up"]
            pub const PUS_1_WEAK_PULL_UP: u32 = 0x01;
        }
    }
    #[doc = "Domain write protection"]
    pub mod DWP {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Both cores are allowed"]
            pub const FORBID_NONE: u32 = 0;
            #[doc = "CM7 is forbidden"]
            pub const FORBID_CM7: u32 = 0x01;
            #[doc = "CM4 is forbidden"]
            pub const FORBID_CM4: u32 = 0x02;
            #[doc = "Both cores are forbidden"]
            pub const FORBID_BOTH: u32 = 0x03;
        }
    }
    #[doc = "Domain write protection lock"]
    pub mod DWP_LOCK {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Neither of DWP bits is locked"]
            pub const LOCK_NONE: u32 = 0;
            #[doc = "The lower DWP bit is locked"]
            pub const LOCK_LOW: u32 = 0x01;
            #[doc = "The higher DWP bit is locked"]
            pub const LOCK_HIGH: u32 = 0x02;
            #[doc = "Both DWP bits are locked"]
            pub const LOCK_BOTH: u32 = 0x03;
        }
    }
}
#[doc = "SW_PAD_CTL_PAD_POR_B_DIG SW PAD Control Register"]
pub mod SW_PAD_CTL_PAD_POR_B_DIG {
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
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "normal driver"]
            pub const DSE_0_NORMAL_DRIVER: u32 = 0;
            #[doc = "high driver"]
            pub const DSE_1_HIGH_DRIVER: u32 = 0x01;
        }
    }
    #[doc = "Pull / Keep Select Field"]
    pub mod PUE {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pull Disable"]
            pub const PUE_0_DISABLE: u32 = 0;
            #[doc = "Pull Enable"]
            pub const PUE_1_PULL: u32 = 0x01;
        }
    }
    #[doc = "Pull Up / Down Config. Field"]
    pub mod PUS {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Weak pull down"]
            pub const PUS_0_WEAK_PULL_DOWN: u32 = 0;
            #[doc = "Weak pull up"]
            pub const PUS_1_WEAK_PULL_UP: u32 = 0x01;
        }
    }
    #[doc = "Domain write protection"]
    pub mod DWP {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Both cores are allowed"]
            pub const FORBID_NONE: u32 = 0;
            #[doc = "CM7 is forbidden"]
            pub const FORBID_CM7: u32 = 0x01;
            #[doc = "CM4 is forbidden"]
            pub const FORBID_CM4: u32 = 0x02;
            #[doc = "Both cores are forbidden"]
            pub const FORBID_BOTH: u32 = 0x03;
        }
    }
    #[doc = "Domain write protection lock"]
    pub mod DWP_LOCK {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Neither of DWP bits is locked"]
            pub const LOCK_NONE: u32 = 0;
            #[doc = "The lower DWP bit is locked"]
            pub const LOCK_LOW: u32 = 0x01;
            #[doc = "The higher DWP bit is locked"]
            pub const LOCK_HIGH: u32 = 0x02;
            #[doc = "Both DWP bits are locked"]
            pub const LOCK_BOTH: u32 = 0x03;
        }
    }
}
#[doc = "SW_PAD_CTL_PAD_ONOFF_DIG SW PAD Control Register"]
pub mod SW_PAD_CTL_PAD_ONOFF_DIG {
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
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "normal driver"]
            pub const DSE_0_NORMAL_DRIVER: u32 = 0;
            #[doc = "high driver"]
            pub const DSE_1_HIGH_DRIVER: u32 = 0x01;
        }
    }
    #[doc = "Pull / Keep Select Field"]
    pub mod PUE {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pull Disable"]
            pub const PUE_0_DISABLE: u32 = 0;
            #[doc = "Pull Enable"]
            pub const PUE_1_PULL: u32 = 0x01;
        }
    }
    #[doc = "Pull Up / Down Config. Field"]
    pub mod PUS {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Weak pull down"]
            pub const PUS_0_WEAK_PULL_DOWN: u32 = 0;
            #[doc = "Weak pull up"]
            pub const PUS_1_WEAK_PULL_UP: u32 = 0x01;
        }
    }
    #[doc = "Domain write protection"]
    pub mod DWP {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Both cores are allowed"]
            pub const FORBID_NONE: u32 = 0;
            #[doc = "CM7 is forbidden"]
            pub const FORBID_CM7: u32 = 0x01;
            #[doc = "CM4 is forbidden"]
            pub const FORBID_CM4: u32 = 0x02;
            #[doc = "Both cores are forbidden"]
            pub const FORBID_BOTH: u32 = 0x03;
        }
    }
    #[doc = "Domain write protection lock"]
    pub mod DWP_LOCK {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Neither of DWP bits is locked"]
            pub const LOCK_NONE: u32 = 0;
            #[doc = "The lower DWP bit is locked"]
            pub const LOCK_LOW: u32 = 0x01;
            #[doc = "The higher DWP bit is locked"]
            pub const LOCK_HIGH: u32 = 0x02;
            #[doc = "Both DWP bits are locked"]
            pub const LOCK_BOTH: u32 = 0x03;
        }
    }
}
#[doc = "SW_PAD_CTL_PAD_WAKEUP_DIG SW PAD Control Register"]
pub mod SW_PAD_CTL_PAD_WAKEUP_DIG {
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
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "normal driver"]
            pub const DSE_0_NORMAL_DRIVER: u32 = 0;
            #[doc = "high driver"]
            pub const DSE_1_HIGH_DRIVER: u32 = 0x01;
        }
    }
    #[doc = "Pull / Keep Select Field"]
    pub mod PUE {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pull Disable"]
            pub const PUE_0_DISABLE: u32 = 0;
            #[doc = "Pull Enable"]
            pub const PUE_1_PULL: u32 = 0x01;
        }
    }
    #[doc = "Pull Up / Down Config. Field"]
    pub mod PUS {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Weak pull down"]
            pub const PUS_0_WEAK_PULL_DOWN: u32 = 0;
            #[doc = "Weak pull up"]
            pub const PUS_1_WEAK_PULL_UP: u32 = 0x01;
        }
    }
    #[doc = "Open Drain SNVS Field"]
    pub mod ODE_SNVS {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const ODE_SNVS_0_DISABLED: u32 = 0;
            #[doc = "Enabled"]
            pub const ODE_SNVS_1_ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Domain write protection"]
    pub mod DWP {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Both cores are allowed"]
            pub const FORBID_NONE: u32 = 0;
            #[doc = "CM7 is forbidden"]
            pub const FORBID_CM7: u32 = 0x01;
            #[doc = "CM4 is forbidden"]
            pub const FORBID_CM4: u32 = 0x02;
            #[doc = "Both cores are forbidden"]
            pub const FORBID_BOTH: u32 = 0x03;
        }
    }
    #[doc = "Domain write protection lock"]
    pub mod DWP_LOCK {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Neither of DWP bits is locked"]
            pub const LOCK_NONE: u32 = 0;
            #[doc = "The lower DWP bit is locked"]
            pub const LOCK_LOW: u32 = 0x01;
            #[doc = "The higher DWP bit is locked"]
            pub const LOCK_HIGH: u32 = 0x02;
            #[doc = "Both DWP bits are locked"]
            pub const LOCK_BOTH: u32 = 0x03;
        }
    }
}
#[doc = "SW_PAD_CTL_PAD_PMIC_ON_REQ_DIG SW PAD Control Register"]
pub mod SW_PAD_CTL_PAD_PMIC_ON_REQ_DIG {
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
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "normal driver"]
            pub const DSE_0_NORMAL_DRIVER: u32 = 0;
            #[doc = "high driver"]
            pub const DSE_1_HIGH_DRIVER: u32 = 0x01;
        }
    }
    #[doc = "Pull / Keep Select Field"]
    pub mod PUE {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pull Disable"]
            pub const PUE_0_DISABLE: u32 = 0;
            #[doc = "Pull Enable"]
            pub const PUE_1_PULL: u32 = 0x01;
        }
    }
    #[doc = "Pull Up / Down Config. Field"]
    pub mod PUS {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Weak pull down"]
            pub const PUS_0_WEAK_PULL_DOWN: u32 = 0;
            #[doc = "Weak pull up"]
            pub const PUS_1_WEAK_PULL_UP: u32 = 0x01;
        }
    }
    #[doc = "Open Drain SNVS Field"]
    pub mod ODE_SNVS {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const ODE_SNVS_0_DISABLED: u32 = 0;
            #[doc = "Enabled"]
            pub const ODE_SNVS_1_ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Domain write protection"]
    pub mod DWP {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Both cores are allowed"]
            pub const FORBID_NONE: u32 = 0;
            #[doc = "CM7 is forbidden"]
            pub const FORBID_CM7: u32 = 0x01;
            #[doc = "CM4 is forbidden"]
            pub const FORBID_CM4: u32 = 0x02;
            #[doc = "Both cores are forbidden"]
            pub const FORBID_BOTH: u32 = 0x03;
        }
    }
    #[doc = "Domain write protection lock"]
    pub mod DWP_LOCK {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Neither of DWP bits is locked"]
            pub const LOCK_NONE: u32 = 0;
            #[doc = "The lower DWP bit is locked"]
            pub const LOCK_LOW: u32 = 0x01;
            #[doc = "The higher DWP bit is locked"]
            pub const LOCK_HIGH: u32 = 0x02;
            #[doc = "Both DWP bits are locked"]
            pub const LOCK_BOTH: u32 = 0x03;
        }
    }
}
#[doc = "SW_PAD_CTL_PAD_PMIC_STBY_REQ_DIG SW PAD Control Register"]
pub mod SW_PAD_CTL_PAD_PMIC_STBY_REQ_DIG {
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
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "normal driver"]
            pub const DSE_0_NORMAL_DRIVER: u32 = 0;
            #[doc = "high driver"]
            pub const DSE_1_HIGH_DRIVER: u32 = 0x01;
        }
    }
    #[doc = "Pull / Keep Select Field"]
    pub mod PUE {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pull Disable"]
            pub const PUE_0_DISABLE: u32 = 0;
            #[doc = "Pull Enable"]
            pub const PUE_1_PULL: u32 = 0x01;
        }
    }
    #[doc = "Pull Up / Down Config. Field"]
    pub mod PUS {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Weak pull down"]
            pub const PUS_0_WEAK_PULL_DOWN: u32 = 0;
            #[doc = "Weak pull up"]
            pub const PUS_1_WEAK_PULL_UP: u32 = 0x01;
        }
    }
    #[doc = "Open Drain SNVS Field"]
    pub mod ODE_SNVS {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const ODE_SNVS_0_DISABLED: u32 = 0;
            #[doc = "Enabled"]
            pub const ODE_SNVS_1_ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Domain write protection"]
    pub mod DWP {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Both cores are allowed"]
            pub const FORBID_NONE: u32 = 0;
            #[doc = "CM7 is forbidden"]
            pub const FORBID_CM7: u32 = 0x01;
            #[doc = "CM4 is forbidden"]
            pub const FORBID_CM4: u32 = 0x02;
            #[doc = "Both cores are forbidden"]
            pub const FORBID_BOTH: u32 = 0x03;
        }
    }
    #[doc = "Domain write protection lock"]
    pub mod DWP_LOCK {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Neither of DWP bits is locked"]
            pub const LOCK_NONE: u32 = 0;
            #[doc = "The lower DWP bit is locked"]
            pub const LOCK_LOW: u32 = 0x01;
            #[doc = "The higher DWP bit is locked"]
            pub const LOCK_HIGH: u32 = 0x02;
            #[doc = "Both DWP bits are locked"]
            pub const LOCK_BOTH: u32 = 0x03;
        }
    }
}
#[doc = "SW_PAD_CTL_PAD_GPIO_SNVS_00_DIG SW PAD Control Register"]
pub mod SW_PAD_CTL_PAD_GPIO_SNVS_00_DIG {
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
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "normal driver"]
            pub const DSE_0_NORMAL_DRIVER: u32 = 0;
            #[doc = "high driver"]
            pub const DSE_1_HIGH_DRIVER: u32 = 0x01;
        }
    }
    #[doc = "Pull / Keep Select Field"]
    pub mod PUE {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pull Disable"]
            pub const PUE_0_DISABLE: u32 = 0;
            #[doc = "Pull Enable"]
            pub const PUE_1_PULL: u32 = 0x01;
        }
    }
    #[doc = "Pull Up / Down Config. Field"]
    pub mod PUS {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Weak pull down"]
            pub const PUS_0_WEAK_PULL_DOWN: u32 = 0;
            #[doc = "Weak pull up"]
            pub const PUS_1_WEAK_PULL_UP: u32 = 0x01;
        }
    }
    #[doc = "Open Drain SNVS Field"]
    pub mod ODE_SNVS {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const ODE_SNVS_0_DISABLED: u32 = 0;
            #[doc = "Enabled"]
            pub const ODE_SNVS_1_ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Domain write protection"]
    pub mod DWP {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Both cores are allowed"]
            pub const FORBID_NONE: u32 = 0;
            #[doc = "CM7 is forbidden"]
            pub const FORBID_CM7: u32 = 0x01;
            #[doc = "CM4 is forbidden"]
            pub const FORBID_CM4: u32 = 0x02;
            #[doc = "Both cores are forbidden"]
            pub const FORBID_BOTH: u32 = 0x03;
        }
    }
    #[doc = "Domain write protection lock"]
    pub mod DWP_LOCK {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Neither of DWP bits is locked"]
            pub const LOCK_NONE: u32 = 0;
            #[doc = "The lower DWP bit is locked"]
            pub const LOCK_LOW: u32 = 0x01;
            #[doc = "The higher DWP bit is locked"]
            pub const LOCK_HIGH: u32 = 0x02;
            #[doc = "Both DWP bits are locked"]
            pub const LOCK_BOTH: u32 = 0x03;
        }
    }
}
#[doc = "SW_PAD_CTL_PAD_GPIO_SNVS_01_DIG SW PAD Control Register"]
pub mod SW_PAD_CTL_PAD_GPIO_SNVS_01_DIG {
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
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "normal driver"]
            pub const DSE_0_NORMAL_DRIVER: u32 = 0;
            #[doc = "high driver"]
            pub const DSE_1_HIGH_DRIVER: u32 = 0x01;
        }
    }
    #[doc = "Pull / Keep Select Field"]
    pub mod PUE {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pull Disable"]
            pub const PUE_0_DISABLE: u32 = 0;
            #[doc = "Pull Enable"]
            pub const PUE_1_PULL: u32 = 0x01;
        }
    }
    #[doc = "Pull Up / Down Config. Field"]
    pub mod PUS {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Weak pull down"]
            pub const PUS_0_WEAK_PULL_DOWN: u32 = 0;
            #[doc = "Weak pull up"]
            pub const PUS_1_WEAK_PULL_UP: u32 = 0x01;
        }
    }
    #[doc = "Open Drain SNVS Field"]
    pub mod ODE_SNVS {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const ODE_SNVS_0_DISABLED: u32 = 0;
            #[doc = "Enabled"]
            pub const ODE_SNVS_1_ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Domain write protection"]
    pub mod DWP {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Both cores are allowed"]
            pub const FORBID_NONE: u32 = 0;
            #[doc = "CM7 is forbidden"]
            pub const FORBID_CM7: u32 = 0x01;
            #[doc = "CM4 is forbidden"]
            pub const FORBID_CM4: u32 = 0x02;
            #[doc = "Both cores are forbidden"]
            pub const FORBID_BOTH: u32 = 0x03;
        }
    }
    #[doc = "Domain write protection lock"]
    pub mod DWP_LOCK {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Neither of DWP bits is locked"]
            pub const LOCK_NONE: u32 = 0;
            #[doc = "The lower DWP bit is locked"]
            pub const LOCK_LOW: u32 = 0x01;
            #[doc = "The higher DWP bit is locked"]
            pub const LOCK_HIGH: u32 = 0x02;
            #[doc = "Both DWP bits are locked"]
            pub const LOCK_BOTH: u32 = 0x03;
        }
    }
}
#[doc = "SW_PAD_CTL_PAD_GPIO_SNVS_02_DIG SW PAD Control Register"]
pub mod SW_PAD_CTL_PAD_GPIO_SNVS_02_DIG {
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
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "normal driver"]
            pub const DSE_0_NORMAL_DRIVER: u32 = 0;
            #[doc = "high driver"]
            pub const DSE_1_HIGH_DRIVER: u32 = 0x01;
        }
    }
    #[doc = "Pull / Keep Select Field"]
    pub mod PUE {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pull Disable"]
            pub const PUE_0_DISABLE: u32 = 0;
            #[doc = "Pull Enable"]
            pub const PUE_1_PULL: u32 = 0x01;
        }
    }
    #[doc = "Pull Up / Down Config. Field"]
    pub mod PUS {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Weak pull down"]
            pub const PUS_0_WEAK_PULL_DOWN: u32 = 0;
            #[doc = "Weak pull up"]
            pub const PUS_1_WEAK_PULL_UP: u32 = 0x01;
        }
    }
    #[doc = "Open Drain SNVS Field"]
    pub mod ODE_SNVS {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const ODE_SNVS_0_DISABLED: u32 = 0;
            #[doc = "Enabled"]
            pub const ODE_SNVS_1_ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Domain write protection"]
    pub mod DWP {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Both cores are allowed"]
            pub const FORBID_NONE: u32 = 0;
            #[doc = "CM7 is forbidden"]
            pub const FORBID_CM7: u32 = 0x01;
            #[doc = "CM4 is forbidden"]
            pub const FORBID_CM4: u32 = 0x02;
            #[doc = "Both cores are forbidden"]
            pub const FORBID_BOTH: u32 = 0x03;
        }
    }
    #[doc = "Domain write protection lock"]
    pub mod DWP_LOCK {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Neither of DWP bits is locked"]
            pub const LOCK_NONE: u32 = 0;
            #[doc = "The lower DWP bit is locked"]
            pub const LOCK_LOW: u32 = 0x01;
            #[doc = "The higher DWP bit is locked"]
            pub const LOCK_HIGH: u32 = 0x02;
            #[doc = "Both DWP bits are locked"]
            pub const LOCK_BOTH: u32 = 0x03;
        }
    }
}
#[doc = "SW_PAD_CTL_PAD_GPIO_SNVS_03_DIG SW PAD Control Register"]
pub mod SW_PAD_CTL_PAD_GPIO_SNVS_03_DIG {
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
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "normal driver"]
            pub const DSE_0_NORMAL_DRIVER: u32 = 0;
            #[doc = "high driver"]
            pub const DSE_1_HIGH_DRIVER: u32 = 0x01;
        }
    }
    #[doc = "Pull / Keep Select Field"]
    pub mod PUE {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pull Disable"]
            pub const PUE_0_DISABLE: u32 = 0;
            #[doc = "Pull Enable"]
            pub const PUE_1_PULL: u32 = 0x01;
        }
    }
    #[doc = "Pull Up / Down Config. Field"]
    pub mod PUS {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Weak pull down"]
            pub const PUS_0_WEAK_PULL_DOWN: u32 = 0;
            #[doc = "Weak pull up"]
            pub const PUS_1_WEAK_PULL_UP: u32 = 0x01;
        }
    }
    #[doc = "Open Drain SNVS Field"]
    pub mod ODE_SNVS {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const ODE_SNVS_0_DISABLED: u32 = 0;
            #[doc = "Enabled"]
            pub const ODE_SNVS_1_ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Domain write protection"]
    pub mod DWP {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Both cores are allowed"]
            pub const FORBID_NONE: u32 = 0;
            #[doc = "CM7 is forbidden"]
            pub const FORBID_CM7: u32 = 0x01;
            #[doc = "CM4 is forbidden"]
            pub const FORBID_CM4: u32 = 0x02;
            #[doc = "Both cores are forbidden"]
            pub const FORBID_BOTH: u32 = 0x03;
        }
    }
    #[doc = "Domain write protection lock"]
    pub mod DWP_LOCK {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Neither of DWP bits is locked"]
            pub const LOCK_NONE: u32 = 0;
            #[doc = "The lower DWP bit is locked"]
            pub const LOCK_LOW: u32 = 0x01;
            #[doc = "The higher DWP bit is locked"]
            pub const LOCK_HIGH: u32 = 0x02;
            #[doc = "Both DWP bits are locked"]
            pub const LOCK_BOTH: u32 = 0x03;
        }
    }
}
#[doc = "SW_PAD_CTL_PAD_GPIO_SNVS_04_DIG SW PAD Control Register"]
pub mod SW_PAD_CTL_PAD_GPIO_SNVS_04_DIG {
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
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "normal driver"]
            pub const DSE_0_NORMAL_DRIVER: u32 = 0;
            #[doc = "high driver"]
            pub const DSE_1_HIGH_DRIVER: u32 = 0x01;
        }
    }
    #[doc = "Pull / Keep Select Field"]
    pub mod PUE {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pull Disable"]
            pub const PUE_0_DISABLE: u32 = 0;
            #[doc = "Pull Enable"]
            pub const PUE_1_PULL: u32 = 0x01;
        }
    }
    #[doc = "Pull Up / Down Config. Field"]
    pub mod PUS {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Weak pull down"]
            pub const PUS_0_WEAK_PULL_DOWN: u32 = 0;
            #[doc = "Weak pull up"]
            pub const PUS_1_WEAK_PULL_UP: u32 = 0x01;
        }
    }
    #[doc = "Open Drain SNVS Field"]
    pub mod ODE_SNVS {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const ODE_SNVS_0_DISABLED: u32 = 0;
            #[doc = "Enabled"]
            pub const ODE_SNVS_1_ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Domain write protection"]
    pub mod DWP {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Both cores are allowed"]
            pub const FORBID_NONE: u32 = 0;
            #[doc = "CM7 is forbidden"]
            pub const FORBID_CM7: u32 = 0x01;
            #[doc = "CM4 is forbidden"]
            pub const FORBID_CM4: u32 = 0x02;
            #[doc = "Both cores are forbidden"]
            pub const FORBID_BOTH: u32 = 0x03;
        }
    }
    #[doc = "Domain write protection lock"]
    pub mod DWP_LOCK {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Neither of DWP bits is locked"]
            pub const LOCK_NONE: u32 = 0;
            #[doc = "The lower DWP bit is locked"]
            pub const LOCK_LOW: u32 = 0x01;
            #[doc = "The higher DWP bit is locked"]
            pub const LOCK_HIGH: u32 = 0x02;
            #[doc = "Both DWP bits are locked"]
            pub const LOCK_BOTH: u32 = 0x03;
        }
    }
}
#[doc = "SW_PAD_CTL_PAD_GPIO_SNVS_05_DIG SW PAD Control Register"]
pub mod SW_PAD_CTL_PAD_GPIO_SNVS_05_DIG {
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
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "normal driver"]
            pub const DSE_0_NORMAL_DRIVER: u32 = 0;
            #[doc = "high driver"]
            pub const DSE_1_HIGH_DRIVER: u32 = 0x01;
        }
    }
    #[doc = "Pull / Keep Select Field"]
    pub mod PUE {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pull Disable"]
            pub const PUE_0_DISABLE: u32 = 0;
            #[doc = "Pull Enable"]
            pub const PUE_1_PULL: u32 = 0x01;
        }
    }
    #[doc = "Pull Up / Down Config. Field"]
    pub mod PUS {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Weak pull down"]
            pub const PUS_0_WEAK_PULL_DOWN: u32 = 0;
            #[doc = "Weak pull up"]
            pub const PUS_1_WEAK_PULL_UP: u32 = 0x01;
        }
    }
    #[doc = "Open Drain SNVS Field"]
    pub mod ODE_SNVS {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const ODE_SNVS_0_DISABLED: u32 = 0;
            #[doc = "Enabled"]
            pub const ODE_SNVS_1_ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Domain write protection"]
    pub mod DWP {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Both cores are allowed"]
            pub const FORBID_NONE: u32 = 0;
            #[doc = "CM7 is forbidden"]
            pub const FORBID_CM7: u32 = 0x01;
            #[doc = "CM4 is forbidden"]
            pub const FORBID_CM4: u32 = 0x02;
            #[doc = "Both cores are forbidden"]
            pub const FORBID_BOTH: u32 = 0x03;
        }
    }
    #[doc = "Domain write protection lock"]
    pub mod DWP_LOCK {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Neither of DWP bits is locked"]
            pub const LOCK_NONE: u32 = 0;
            #[doc = "The lower DWP bit is locked"]
            pub const LOCK_LOW: u32 = 0x01;
            #[doc = "The higher DWP bit is locked"]
            pub const LOCK_HIGH: u32 = 0x02;
            #[doc = "Both DWP bits are locked"]
            pub const LOCK_BOTH: u32 = 0x03;
        }
    }
}
#[doc = "SW_PAD_CTL_PAD_GPIO_SNVS_06_DIG SW PAD Control Register"]
pub mod SW_PAD_CTL_PAD_GPIO_SNVS_06_DIG {
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
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "normal driver"]
            pub const DSE_0_NORMAL_DRIVER: u32 = 0;
            #[doc = "high driver"]
            pub const DSE_1_HIGH_DRIVER: u32 = 0x01;
        }
    }
    #[doc = "Pull / Keep Select Field"]
    pub mod PUE {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pull Disable"]
            pub const PUE_0_DISABLE: u32 = 0;
            #[doc = "Pull Enable"]
            pub const PUE_1_PULL: u32 = 0x01;
        }
    }
    #[doc = "Pull Up / Down Config. Field"]
    pub mod PUS {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Weak pull down"]
            pub const PUS_0_WEAK_PULL_DOWN: u32 = 0;
            #[doc = "Weak pull up"]
            pub const PUS_1_WEAK_PULL_UP: u32 = 0x01;
        }
    }
    #[doc = "Open Drain SNVS Field"]
    pub mod ODE_SNVS {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const ODE_SNVS_0_DISABLED: u32 = 0;
            #[doc = "Enabled"]
            pub const ODE_SNVS_1_ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Domain write protection"]
    pub mod DWP {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Both cores are allowed"]
            pub const FORBID_NONE: u32 = 0;
            #[doc = "CM7 is forbidden"]
            pub const FORBID_CM7: u32 = 0x01;
            #[doc = "CM4 is forbidden"]
            pub const FORBID_CM4: u32 = 0x02;
            #[doc = "Both cores are forbidden"]
            pub const FORBID_BOTH: u32 = 0x03;
        }
    }
    #[doc = "Domain write protection lock"]
    pub mod DWP_LOCK {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Neither of DWP bits is locked"]
            pub const LOCK_NONE: u32 = 0;
            #[doc = "The lower DWP bit is locked"]
            pub const LOCK_LOW: u32 = 0x01;
            #[doc = "The higher DWP bit is locked"]
            pub const LOCK_HIGH: u32 = 0x02;
            #[doc = "Both DWP bits are locked"]
            pub const LOCK_BOTH: u32 = 0x03;
        }
    }
}
#[doc = "SW_PAD_CTL_PAD_GPIO_SNVS_07_DIG SW PAD Control Register"]
pub mod SW_PAD_CTL_PAD_GPIO_SNVS_07_DIG {
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
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "normal driver"]
            pub const DSE_0_NORMAL_DRIVER: u32 = 0;
            #[doc = "high driver"]
            pub const DSE_1_HIGH_DRIVER: u32 = 0x01;
        }
    }
    #[doc = "Pull / Keep Select Field"]
    pub mod PUE {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pull Disable"]
            pub const PUE_0_DISABLE: u32 = 0;
            #[doc = "Pull Enable"]
            pub const PUE_1_PULL: u32 = 0x01;
        }
    }
    #[doc = "Pull Up / Down Config. Field"]
    pub mod PUS {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Weak pull down"]
            pub const PUS_0_WEAK_PULL_DOWN: u32 = 0;
            #[doc = "Weak pull up"]
            pub const PUS_1_WEAK_PULL_UP: u32 = 0x01;
        }
    }
    #[doc = "Open Drain SNVS Field"]
    pub mod ODE_SNVS {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const ODE_SNVS_0_DISABLED: u32 = 0;
            #[doc = "Enabled"]
            pub const ODE_SNVS_1_ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Domain write protection"]
    pub mod DWP {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Both cores are allowed"]
            pub const FORBID_NONE: u32 = 0;
            #[doc = "CM7 is forbidden"]
            pub const FORBID_CM7: u32 = 0x01;
            #[doc = "CM4 is forbidden"]
            pub const FORBID_CM4: u32 = 0x02;
            #[doc = "Both cores are forbidden"]
            pub const FORBID_BOTH: u32 = 0x03;
        }
    }
    #[doc = "Domain write protection lock"]
    pub mod DWP_LOCK {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Neither of DWP bits is locked"]
            pub const LOCK_NONE: u32 = 0;
            #[doc = "The lower DWP bit is locked"]
            pub const LOCK_LOW: u32 = 0x01;
            #[doc = "The higher DWP bit is locked"]
            pub const LOCK_HIGH: u32 = 0x02;
            #[doc = "Both DWP bits are locked"]
            pub const LOCK_BOTH: u32 = 0x03;
        }
    }
}
#[doc = "SW_PAD_CTL_PAD_GPIO_SNVS_08_DIG SW PAD Control Register"]
pub mod SW_PAD_CTL_PAD_GPIO_SNVS_08_DIG {
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
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "normal driver"]
            pub const DSE_0_NORMAL_DRIVER: u32 = 0;
            #[doc = "high driver"]
            pub const DSE_1_HIGH_DRIVER: u32 = 0x01;
        }
    }
    #[doc = "Pull / Keep Select Field"]
    pub mod PUE {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pull Disable"]
            pub const PUE_0_DISABLE: u32 = 0;
            #[doc = "Pull Enable"]
            pub const PUE_1_PULL: u32 = 0x01;
        }
    }
    #[doc = "Pull Up / Down Config. Field"]
    pub mod PUS {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Weak pull down"]
            pub const PUS_0_WEAK_PULL_DOWN: u32 = 0;
            #[doc = "Weak pull up"]
            pub const PUS_1_WEAK_PULL_UP: u32 = 0x01;
        }
    }
    #[doc = "Open Drain SNVS Field"]
    pub mod ODE_SNVS {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const ODE_SNVS_0_DISABLED: u32 = 0;
            #[doc = "Enabled"]
            pub const ODE_SNVS_1_ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Domain write protection"]
    pub mod DWP {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Both cores are allowed"]
            pub const FORBID_NONE: u32 = 0;
            #[doc = "CM7 is forbidden"]
            pub const FORBID_CM7: u32 = 0x01;
            #[doc = "CM4 is forbidden"]
            pub const FORBID_CM4: u32 = 0x02;
            #[doc = "Both cores are forbidden"]
            pub const FORBID_BOTH: u32 = 0x03;
        }
    }
    #[doc = "Domain write protection lock"]
    pub mod DWP_LOCK {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Neither of DWP bits is locked"]
            pub const LOCK_NONE: u32 = 0;
            #[doc = "The lower DWP bit is locked"]
            pub const LOCK_LOW: u32 = 0x01;
            #[doc = "The higher DWP bit is locked"]
            pub const LOCK_HIGH: u32 = 0x02;
            #[doc = "Both DWP bits are locked"]
            pub const LOCK_BOTH: u32 = 0x03;
        }
    }
}
#[doc = "SW_PAD_CTL_PAD_GPIO_SNVS_09_DIG SW PAD Control Register"]
pub mod SW_PAD_CTL_PAD_GPIO_SNVS_09_DIG {
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
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "normal driver"]
            pub const DSE_0_NORMAL_DRIVER: u32 = 0;
            #[doc = "high driver"]
            pub const DSE_1_HIGH_DRIVER: u32 = 0x01;
        }
    }
    #[doc = "Pull / Keep Select Field"]
    pub mod PUE {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pull Disable"]
            pub const PUE_0_DISABLE: u32 = 0;
            #[doc = "Pull Enable"]
            pub const PUE_1_PULL: u32 = 0x01;
        }
    }
    #[doc = "Pull Up / Down Config. Field"]
    pub mod PUS {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Weak pull down"]
            pub const PUS_0_WEAK_PULL_DOWN: u32 = 0;
            #[doc = "Weak pull up"]
            pub const PUS_1_WEAK_PULL_UP: u32 = 0x01;
        }
    }
    #[doc = "Open Drain SNVS Field"]
    pub mod ODE_SNVS {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const ODE_SNVS_0_DISABLED: u32 = 0;
            #[doc = "Enabled"]
            pub const ODE_SNVS_1_ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Domain write protection"]
    pub mod DWP {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Both cores are allowed"]
            pub const FORBID_NONE: u32 = 0;
            #[doc = "CM7 is forbidden"]
            pub const FORBID_CM7: u32 = 0x01;
            #[doc = "CM4 is forbidden"]
            pub const FORBID_CM4: u32 = 0x02;
            #[doc = "Both cores are forbidden"]
            pub const FORBID_BOTH: u32 = 0x03;
        }
    }
    #[doc = "Domain write protection lock"]
    pub mod DWP_LOCK {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Neither of DWP bits is locked"]
            pub const LOCK_NONE: u32 = 0;
            #[doc = "The lower DWP bit is locked"]
            pub const LOCK_LOW: u32 = 0x01;
            #[doc = "The higher DWP bit is locked"]
            pub const LOCK_HIGH: u32 = 0x02;
            #[doc = "Both DWP bits are locked"]
            pub const LOCK_BOTH: u32 = 0x03;
        }
    }
}
