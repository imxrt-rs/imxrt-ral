#[doc = "IOMUXC_SNVS"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "SW_MUX_CTL_PAD_PMIC_ON_REQ SW MUX Control Register"]
    pub SW_MUX_CTL_PAD_PMIC_ON_REQ: crate::RWRegister<u32>,
    #[doc = "SW_PAD_CTL_PAD_TEST_MODE SW PAD Control Register"]
    pub SW_PAD_CTL_PAD_TEST_MODE: crate::RWRegister<u32>,
    #[doc = "SW_PAD_CTL_PAD_POR_B SW PAD Control Register"]
    pub SW_PAD_CTL_PAD_POR_B: crate::RWRegister<u32>,
    #[doc = "SW_PAD_CTL_PAD_ONOFF SW PAD Control Register"]
    pub SW_PAD_CTL_PAD_ONOFF: crate::RWRegister<u32>,
    #[doc = "SW_PAD_CTL_PAD_PMIC_ON_REQ SW PAD Control Register"]
    pub SW_PAD_CTL_PAD_PMIC_ON_REQ: crate::RWRegister<u32>,
}
#[doc = "SW_MUX_CTL_PAD_PMIC_ON_REQ SW MUX Control Register"]
pub mod SW_MUX_CTL_PAD_PMIC_ON_REQ {
    #[doc = "MUX Mode Select Field."]
    pub mod MUX_MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Select mux mode: ALT0 mux port: SNVS_LP_PMIC_ON_REQ of instance: snvs_lp"]
            pub const ALT0: u32 = 0;
            #[doc = "Select mux mode: ALT5 mux port: GPIO5_IO00 of instance: gpio5"]
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
            #[doc = "Force input path of pad PMIC_ON_REQ"]
            pub const ENABLED: u32 = 0x01;
        }
    }
}
#[doc = "SW_PAD_CTL_PAD_TEST_MODE SW PAD Control Register"]
pub mod SW_PAD_CTL_PAD_TEST_MODE {
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
            #[doc = "medium(100MHz)"]
            pub const SPEED: u32 = 0x02;
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
#[doc = "SW_PAD_CTL_PAD_POR_B SW PAD Control Register"]
pub mod SW_PAD_CTL_PAD_POR_B {
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
            #[doc = "medium(100MHz)"]
            pub const SPEED: u32 = 0x02;
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
#[doc = "SW_PAD_CTL_PAD_ONOFF SW PAD Control Register"]
pub mod SW_PAD_CTL_PAD_ONOFF {
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
            #[doc = "medium(100MHz)"]
            pub const SPEED: u32 = 0x02;
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
#[doc = "SW_PAD_CTL_PAD_PMIC_ON_REQ SW PAD Control Register"]
pub mod SW_PAD_CTL_PAD_PMIC_ON_REQ {
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
            #[doc = "medium(100MHz)"]
            pub const SPEED: u32 = 0x02;
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
