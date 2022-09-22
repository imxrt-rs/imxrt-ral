#[doc = "MX6RT_ANADIG_REGISTER"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x10],
    #[doc = "48MHz RCOSC Control Register"]
    pub OSC_48M_CTRL: crate::RWRegister<u32>,
    _reserved1: [u8; 0x0c],
    #[doc = "24MHz OSC Control Register"]
    pub OSC_24M_CTRL: crate::RWRegister<u32>,
    _reserved2: [u8; 0x1c],
    #[doc = "400MHz RCOSC Control0 Register"]
    pub OSC_400M_CTRL0: crate::RWRegister<u32>,
    _reserved3: [u8; 0x0c],
    #[doc = "400MHz RCOSC Control1 Register"]
    pub OSC_400M_CTRL1: crate::RWRegister<u32>,
    _reserved4: [u8; 0x0c],
    #[doc = "400MHz RCOSC Control2 Register"]
    pub OSC_400M_CTRL2: crate::RWRegister<u32>,
    _reserved5: [u8; 0x5c],
    #[doc = "16MHz RCOSC Control Register"]
    pub OSC_16M_CTRL: crate::RWRegister<u32>,
}
#[doc = "48MHz RCOSC Control Register"]
pub mod OSC_48M_CTRL {
    #[doc = "48MHz RCOSC Enable"]
    pub mod TEN {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Power down"]
            pub const PD: u32 = 0;
            #[doc = "Power up"]
            pub const PU: u32 = 0x01;
        }
    }
    #[doc = "RCOSC_48M_DIV2 Enable"]
    pub mod RC_48M_DIV2_EN {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const PD: u32 = 0;
            #[doc = "Enable"]
            pub const PU: u32 = 0x01;
        }
    }
    #[doc = "RCOSC_48M_DIV2 Control Mode"]
    pub mod RC_48M_DIV2_CONTROL_MODE {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Software mode (default)"]
            pub const SW: u32 = 0;
            #[doc = "GPC mode (Setpoint)"]
            pub const GPC: u32 = 0x01;
        }
    }
    #[doc = "48MHz RCOSC Control Mode"]
    pub mod RC_48M_CONTROL_MODE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Software mode (default)"]
            pub const SW: u32 = 0;
            #[doc = "GPC mode (Setpoint)"]
            pub const GPC: u32 = 0x01;
        }
    }
}
#[doc = "24MHz OSC Control Register"]
pub mod OSC_24M_CTRL {
    #[doc = "24MHz OSC Bypass Clock"]
    pub mod BYPASS_CLK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "24MHz OSC Bypass Enable"]
    pub mod BYPASS_EN {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "24MHz OSC Low-Power Mode Enable"]
    pub mod LP_EN {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "High Gain mode (HP)"]
            pub const HP: u32 = 0;
            #[doc = "Low-power mode (LP)"]
            pub const LP: u32 = 0x01;
        }
    }
    #[doc = "24MHz OSC Comparator Mode"]
    pub mod OSC_COMP_MODE {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Single-ended mode (default)"]
            pub const SINGLE: u32 = 0;
            #[doc = "Differential mode (test mode)"]
            pub const DIFF: u32 = 0x01;
        }
    }
    #[doc = "24MHz OSC Enable"]
    pub mod OSC_EN {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "24MHz OSC Gate Control"]
    pub mod OSC_24M_GATE {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not Gated"]
            pub const NG: u32 = 0;
            #[doc = "Gated"]
            pub const GATE: u32 = 0x01;
        }
    }
    #[doc = "24MHz OSC Stable"]
    pub mod OSC_24M_STABLE {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not Stable"]
            pub const NS: u32 = 0;
            #[doc = "Stable"]
            pub const STABLE: u32 = 0x01;
        }
    }
    #[doc = "24MHz OSC Control Mode"]
    pub mod OSC_24M_CONTROL_MODE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Software mode (default)"]
            pub const SW: u32 = 0;
            #[doc = "GPC mode (Setpoint)"]
            pub const GPC: u32 = 0x01;
        }
    }
}
#[doc = "400MHz RCOSC Control0 Register"]
pub mod OSC_400M_CTRL0 {
    #[doc = "400MHz OSC AI BUSY"]
    pub mod OSC400M_AI_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "400MHz RCOSC Control1 Register"]
pub mod OSC_400M_CTRL1 {
    #[doc = "Power down control for 400MHz RCOSC"]
    pub mod PWD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No Power down"]
            pub const PD: u32 = 0;
            #[doc = "Power down"]
            pub const PU: u32 = 0x01;
        }
    }
    #[doc = "Clock gate control for 400MHz RCOSC"]
    pub mod CLKGATE_400MEG {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not Gated"]
            pub const NG: u32 = 0;
            #[doc = "Gated"]
            pub const GATE: u32 = 0x01;
        }
    }
    #[doc = "400MHz RCOSC Control mode"]
    pub mod RC_400M_CONTROL_MODE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Software mode (default)"]
            pub const SW: u32 = 0;
            #[doc = "GPC mode (Setpoint)"]
            pub const GPC: u32 = 0x01;
        }
    }
}
#[doc = "400MHz RCOSC Control2 Register"]
pub mod OSC_400M_CTRL2 {
    #[doc = "Clock enable"]
    pub mod ENABLE_CLK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Clock is disabled before entering GPC mode"]
            pub const DISABLE: u32 = 0;
            #[doc = "Clock is enabled before entering GPC mode"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Bypass tuning logic"]
    pub mod TUNE_BYP {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Use the output of tuning logic to run the oscillator"]
            pub const RUN: u32 = 0;
            #[doc = "Bypass the tuning logic and use the programmed OSC_TUNE_VAL to run the oscillator"]
            pub const BYPASS: u32 = 0x01;
        }
    }
    #[doc = "Oscillator Tune Value"]
    pub mod OSC_TUNE_VAL {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "16MHz RCOSC Control Register"]
pub mod OSC_16M_CTRL {
    #[doc = "Enable Clock Output"]
    pub mod EN_IRC4M16M {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Power Save Enable"]
    pub mod EN_POWER_SAVE {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Source select"]
    pub mod SOURCE_SEL_16M {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "16MHz Oscillator"]
            pub const MHZ_16: u32 = 0;
            #[doc = "24MHz Oscillator"]
            pub const MHZ_24: u32 = 0x01;
        }
    }
    #[doc = "Control Mode for 16MHz Oscillator"]
    pub mod RC_16M_CONTROL_MODE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Software mode (default)"]
            pub const SW: u32 = 0;
            #[doc = "GPC mode (Setpoint)"]
            pub const GPC: u32 = 0x01;
        }
    }
}
