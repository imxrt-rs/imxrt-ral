#[doc = "no description available"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "Control Register 0"]
    pub CTRL0: crate::RWRegister<u32>,
    #[doc = "Control Register 0"]
    pub CTRL0_SET: crate::RWRegister<u32>,
    #[doc = "Control Register 0"]
    pub CTRL0_CLR: crate::RWRegister<u32>,
    #[doc = "Control Register 0"]
    pub CTRL0_TOG: crate::RWRegister<u32>,
    #[doc = "Control Register 1"]
    pub CTRL1: crate::RWRegister<u32>,
    #[doc = "Control Register 1"]
    pub CTRL1_SET: crate::RWRegister<u32>,
    #[doc = "Control Register 1"]
    pub CTRL1_CLR: crate::RWRegister<u32>,
    #[doc = "Control Register 1"]
    pub CTRL1_TOG: crate::RWRegister<u32>,
    #[doc = "Control Register 2"]
    pub CTRL2: crate::RWRegister<u32>,
    #[doc = "Control Register 2"]
    pub CTRL2_SET: crate::RWRegister<u32>,
    #[doc = "Control Register 2"]
    pub CTRL2_CLR: crate::RWRegister<u32>,
    #[doc = "Control Register 2"]
    pub CTRL2_TOG: crate::RWRegister<u32>,
    #[doc = "Control Register 3"]
    pub CTRL3: crate::RWRegister<u32>,
    #[doc = "Control Register 3"]
    pub CTRL3_SET: crate::RWRegister<u32>,
    #[doc = "Control Register 3"]
    pub CTRL3_CLR: crate::RWRegister<u32>,
    #[doc = "Control Register 3"]
    pub CTRL3_TOG: crate::RWRegister<u32>,
    _reserved0: [u8; 0x10],
    #[doc = "Status Register 0"]
    pub STAT0: crate::RORegister<u32>,
    #[doc = "Status Register 0"]
    pub STAT0_SET: crate::RORegister<u32>,
    #[doc = "Status Register 0"]
    pub STAT0_CLR: crate::RORegister<u32>,
    #[doc = "Status Register 0"]
    pub STAT0_TOG: crate::RORegister<u32>,
    #[doc = "Status Register 1"]
    pub STAT1: crate::RORegister<u32>,
    #[doc = "Status Register 1"]
    pub STAT1_SET: crate::RORegister<u32>,
    #[doc = "Status Register 1"]
    pub STAT1_CLR: crate::RORegister<u32>,
    #[doc = "Status Register 1"]
    pub STAT1_TOG: crate::RORegister<u32>,
    #[doc = "Status Register 2"]
    pub STAT2: crate::RORegister<u32>,
    #[doc = "Status Register 2"]
    pub STAT2_SET: crate::RORegister<u32>,
    #[doc = "Status Register 2"]
    pub STAT2_CLR: crate::RORegister<u32>,
    #[doc = "Status Register 2"]
    pub STAT2_TOG: crate::RORegister<u32>,
}
#[doc = "Control Register 0"]
pub mod CTRL0 {
    #[doc = "Divide value for ref_clk to generate slow_clk (used inside this IP)"]
    pub mod REF_CLK_DIV {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Control Register 0"]
pub mod CTRL0_SET {
    #[doc = "Divide value for ref_clk to generate slow_clk (used inside this IP)"]
    pub mod REF_CLK_DIV {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Control Register 0"]
pub mod CTRL0_CLR {
    #[doc = "Divide value for ref_clk to generate slow_clk (used inside this IP)"]
    pub mod REF_CLK_DIV {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Control Register 0"]
pub mod CTRL0_TOG {
    #[doc = "Divide value for ref_clk to generate slow_clk (used inside this IP)"]
    pub mod REF_CLK_DIV {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Control Register 1"]
pub mod CTRL1 {
    #[doc = "Negative hysteresis value for the tuned clock"]
    pub mod HYST_MINUS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Positive hysteresis value for the tuned clock"]
    pub mod HYST_PLUS {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Target count for the fast clock"]
    pub mod TARGET_COUNT {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Control Register 1"]
pub mod CTRL1_SET {
    #[doc = "Negative hysteresis value for the tuned clock"]
    pub mod HYST_MINUS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Positive hysteresis value for the tuned clock"]
    pub mod HYST_PLUS {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Target count for the fast clock"]
    pub mod TARGET_COUNT {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Control Register 1"]
pub mod CTRL1_CLR {
    #[doc = "Negative hysteresis value for the tuned clock"]
    pub mod HYST_MINUS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Positive hysteresis value for the tuned clock"]
    pub mod HYST_PLUS {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Target count for the fast clock"]
    pub mod TARGET_COUNT {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Control Register 1"]
pub mod CTRL1_TOG {
    #[doc = "Negative hysteresis value for the tuned clock"]
    pub mod HYST_MINUS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Positive hysteresis value for the tuned clock"]
    pub mod HYST_PLUS {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Target count for the fast clock"]
    pub mod TARGET_COUNT {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Control Register 2"]
pub mod CTRL2 {
    #[doc = "Bypass the tuning logic"]
    pub mod TUNE_BYP {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Use the output of tuning logic to run the oscillator"]
            pub const TUNE_BYP_0: u32 = 0;
            #[doc = "Bypass the tuning logic and use the programmed OSC_TUNE_VAL to run the oscillator"]
            pub const TUNE_BYP_1: u32 = 0x01;
        }
    }
    #[doc = "Freeze/Unfreeze the tuning value"]
    pub mod TUNE_EN {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Freezes the tuning at the current tuned value. Oscillator runs at the frozen tuning value"]
            pub const TUNE_EN_0: u32 = 0;
            #[doc = "Unfreezes and continues the tuning operation"]
            pub const TUNE_EN_1: u32 = 0x01;
        }
    }
    #[doc = "Start/Stop tuning"]
    pub mod TUNE_START {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Stop tuning and reset the tuning logic. Oscillator runs using programmed OSC_TUNE_VAL"]
            pub const TUNE_START_0: u32 = 0;
            #[doc = "Start tuning"]
            pub const TUNE_START_1: u32 = 0x01;
        }
    }
    #[doc = "Program the oscillator frequency"]
    pub mod OSC_TUNE_VAL {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Control Register 2"]
pub mod CTRL2_SET {
    #[doc = "Bypass the tuning logic"]
    pub mod TUNE_BYP {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Freeze/Unfreeze the tuning value"]
    pub mod TUNE_EN {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Start/Stop tuning"]
    pub mod TUNE_START {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Program the oscillator frequency"]
    pub mod OSC_TUNE_VAL {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Control Register 2"]
pub mod CTRL2_CLR {
    #[doc = "Bypass the tuning logic"]
    pub mod TUNE_BYP {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Freeze/Unfreeze the tuning value"]
    pub mod TUNE_EN {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Start/Stop tuning"]
    pub mod TUNE_START {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Program the oscillator frequency"]
    pub mod OSC_TUNE_VAL {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Control Register 2"]
pub mod CTRL2_TOG {
    #[doc = "Bypass the tuning logic"]
    pub mod TUNE_BYP {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Freeze/Unfreeze the tuning value"]
    pub mod TUNE_EN {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Start/Stop tuning"]
    pub mod TUNE_START {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Program the oscillator frequency"]
    pub mod OSC_TUNE_VAL {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Control Register 3"]
pub mod CTRL3 {
    #[doc = "Clear the error flag CLK1M_ERR"]
    pub mod CLR_ERR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No effect"]
            pub const CLR_ERR_0: u32 = 0;
            #[doc = "Clears the error flag CLK1M_ERR in status register STAT0"]
            pub const CLR_ERR_1: u32 = 0x01;
        }
    }
    #[doc = "Enable 1MHz output Clock"]
    pub mod EN_1M_CLK {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Enable the output (clk_1m_out)"]
            pub const EN_1M_CLK_0: u32 = 0;
            #[doc = "Disable the output (clk_1m_out)"]
            pub const EN_1M_CLK_1: u32 = 0x01;
        }
    }
    #[doc = "Select free/locked 1MHz output"]
    pub mod MUX_1M_CLK {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Select free-running 1MHz to be put out on clk_1m_out"]
            pub const MUX_1M_CLK_0: u32 = 0;
            #[doc = "Select locked 1MHz to be put out on clk_1m_out"]
            pub const MUX_1M_CLK_1: u32 = 0x01;
        }
    }
    #[doc = "Count for the locked clk_1m_out"]
    pub mod COUNT_1M_CLK {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Control Register 3"]
pub mod CTRL3_SET {
    #[doc = "Clear the error flag CLK1M_ERR"]
    pub mod CLR_ERR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enable 1MHz output Clock"]
    pub mod EN_1M_CLK {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Select free/locked 1MHz output"]
    pub mod MUX_1M_CLK {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Count for the locked clk_1m_out"]
    pub mod COUNT_1M_CLK {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Control Register 3"]
pub mod CTRL3_CLR {
    #[doc = "Clear the error flag CLK1M_ERR"]
    pub mod CLR_ERR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enable 1MHz output Clock"]
    pub mod EN_1M_CLK {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Select free/locked 1MHz output"]
    pub mod MUX_1M_CLK {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Count for the locked clk_1m_out"]
    pub mod COUNT_1M_CLK {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Control Register 3"]
pub mod CTRL3_TOG {
    #[doc = "Clear the error flag CLK1M_ERR"]
    pub mod CLR_ERR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enable 1MHz output Clock"]
    pub mod EN_1M_CLK {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Select free/locked 1MHz output"]
    pub mod MUX_1M_CLK {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Count for the locked clk_1m_out"]
    pub mod COUNT_1M_CLK {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Status Register 0"]
pub mod STAT0 {
    #[doc = "Error flag for clk_1m_locked"]
    pub mod CLK1M_ERR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No effect"]
            pub const CLK1M_ERR_0: u32 = 0;
            #[doc = "The count value has been reached within one divided ref_clk period"]
            pub const CLK1M_ERR_1: u32 = 0x01;
        }
    }
}
#[doc = "Status Register 0"]
pub mod STAT0_SET {
    #[doc = "Error flag for clk_1m_locked"]
    pub mod CLK1M_ERR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Status Register 0"]
pub mod STAT0_CLR {
    #[doc = "Error flag for clk_1m_locked"]
    pub mod CLK1M_ERR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Status Register 0"]
pub mod STAT0_TOG {
    #[doc = "Error flag for clk_1m_locked"]
    pub mod CLK1M_ERR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Status Register 1"]
pub mod STAT1 {
    #[doc = "Current count for the fast clock"]
    pub mod CURR_COUNT_VAL {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Status Register 1"]
pub mod STAT1_SET {
    #[doc = "Current count for the fast clock"]
    pub mod CURR_COUNT_VAL {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Status Register 1"]
pub mod STAT1_CLR {
    #[doc = "Current count for the fast clock"]
    pub mod CURR_COUNT_VAL {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Status Register 1"]
pub mod STAT1_TOG {
    #[doc = "Current count for the fast clock"]
    pub mod CURR_COUNT_VAL {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Status Register 2"]
pub mod STAT2 {
    #[doc = "Current tuning value used by oscillator"]
    pub mod CURR_OSC_TUNE_VAL {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Status Register 2"]
pub mod STAT2_SET {
    #[doc = "Current tuning value used by oscillator"]
    pub mod CURR_OSC_TUNE_VAL {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Status Register 2"]
pub mod STAT2_CLR {
    #[doc = "Current tuning value used by oscillator"]
    pub mod CURR_OSC_TUNE_VAL {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Status Register 2"]
pub mod STAT2_TOG {
    #[doc = "Current tuning value used by oscillator"]
    pub mod CURR_OSC_TUNE_VAL {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
