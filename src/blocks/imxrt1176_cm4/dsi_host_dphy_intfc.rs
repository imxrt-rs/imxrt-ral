#[doc = "DSI HOST DPHY INTFC"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "PD_TX"]
    pub PD_TX: crate::RWRegister<u32>,
    #[doc = "M_PRG_HS_PREPARE"]
    pub M_PRG_HS_PREPARE: crate::RWRegister<u32>,
    #[doc = "MC_PRG_HS_PREPARE"]
    pub MC_PRG_HS_PREPARE: crate::RWRegister<u32>,
    #[doc = "M_PRG_HS_ZERO"]
    pub M_PRG_HS_ZERO: crate::RWRegister<u32>,
    #[doc = "MC_PRG_HS_ZERO"]
    pub MC_PRG_HS_ZERO: crate::RWRegister<u32>,
    #[doc = "M_PRG_HS_TRAIL"]
    pub M_PRG_HS_TRAIL: crate::RWRegister<u32>,
    #[doc = "MC_PRG_HS_TRAIL"]
    pub MC_PRG_HS_TRAIL: crate::RWRegister<u32>,
    #[doc = "PD_PLL"]
    pub PD_PLL: crate::RWRegister<u32>,
    #[doc = "TST"]
    pub TST: crate::RWRegister<u32>,
    #[doc = "CN"]
    pub CN: crate::RWRegister<u32>,
    #[doc = "CM"]
    pub CM: crate::RWRegister<u32>,
    #[doc = "CO"]
    pub CO: crate::RWRegister<u32>,
    #[doc = "LOCK"]
    pub LOCK: crate::RORegister<u32>,
    #[doc = "LOCK_BYP"]
    pub LOCK_BYP: crate::RWRegister<u32>,
    #[doc = "TX_RCAL"]
    pub TX_RCAL: crate::RWRegister<u32>,
    #[doc = "AUTO_PD_EN"]
    pub AUTO_PD_EN: crate::RWRegister<u32>,
    #[doc = "RXLPRP"]
    pub RXLPRP: crate::RWRegister<u32>,
    #[doc = "RXCDRP"]
    pub RXCDRP: crate::RWRegister<u32>,
}
#[doc = "PD_TX"]
pub mod PD_TX {
    #[doc = "Power Down input for D-PHY"]
    pub mod PD_TX {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Power Up"]
            pub const PD_TX_0: u32 = 0;
            #[doc = "Power Down"]
            pub const PD_TX_1: u32 = 0x01;
        }
    }
}
#[doc = "M_PRG_HS_PREPARE"]
pub mod M_PRG_HS_PREPARE {
    #[doc = "DPHY m_PRG_HS_PREPARE input"]
    pub mod M_PRG_HS_PREPARE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "MC_PRG_HS_PREPARE"]
pub mod MC_PRG_HS_PREPARE {
    #[doc = "DPHY mc_PRG_HS_PREPARE input"]
    pub mod MC_PRG_HS_PREPARE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "M_PRG_HS_ZERO"]
pub mod M_PRG_HS_ZERO {
    #[doc = "DPHY m_PRG_HS_ZERO input"]
    pub mod M_PRG_HS_ZERO {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "MC_PRG_HS_ZERO"]
pub mod MC_PRG_HS_ZERO {
    #[doc = "DPHY mc_PRG_HS_ZERO input"]
    pub mod MC_PRG_HS_ZERO {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "M_PRG_HS_TRAIL"]
pub mod M_PRG_HS_TRAIL {
    #[doc = "DPHY m_PRG_HS_TRAIL input"]
    pub mod M_PRG_HS_TRAIL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "MC_PRG_HS_TRAIL"]
pub mod MC_PRG_HS_TRAIL {
    #[doc = "DPHY mc_PRG_HS_TRAIL input"]
    pub mod MC_PRG_HS_TRAIL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "PD_PLL"]
pub mod PD_PLL {
    #[doc = "Power-down signal"]
    pub mod PD_PLL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Power up PLL"]
            pub const PD_PLL_0: u32 = 0;
            #[doc = "Power down PLL"]
            pub const PD_PLL_1: u32 = 0x01;
        }
    }
}
#[doc = "TST"]
pub mod TST {
    #[doc = "Test"]
    pub mod TST {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "CN"]
pub mod CN {
    #[doc = "Control N divider"]
    pub mod CN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "CM"]
pub mod CM {
    #[doc = "Control M divider"]
    pub mod CM {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "CO"]
pub mod CO {
    #[doc = "Control O divider"]
    pub mod CO {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Divide by 1"]
            pub const CO_0: u32 = 0;
            #[doc = "Divide by 2"]
            pub const CO_1: u32 = 0x01;
            #[doc = "Divide by 4"]
            pub const CO_2: u32 = 0x02;
            #[doc = "Divide by 8"]
            pub const CO_3: u32 = 0x03;
        }
    }
}
#[doc = "LOCK"]
pub mod LOCK {
    #[doc = "Lock Detect output"]
    pub mod LOCK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "PLL not locked"]
            pub const LOCK_0: u32 = 0;
            #[doc = "PLL has achieved frequency lock"]
            pub const LOCK_1: u32 = 0x01;
        }
    }
}
#[doc = "LOCK_BYP"]
pub mod LOCK_BYP {
    #[doc = "DPHY LOCK_BYP input"]
    pub mod LOCK_BYP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "PLL LOCK signal will gate TxByteClkHS clock"]
            pub const GATE: u32 = 0;
            #[doc = "PLL LOCK signal will not gate TxByteClkHS clock, CIL based counter will be used to gate the TxByteClkHS"]
            pub const NOGATE: u32 = 0x01;
        }
    }
}
#[doc = "TX_RCAL"]
pub mod TX_RCAL {
    #[doc = "On-chip termination control bits for manual calibration of HS-TX"]
    pub mod TX_RCAL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "20% higher than mid-range. Highest impedance setting"]
            pub const TX_RCAL_0: u32 = 0;
            #[doc = "Mid-range impedance setting (default)"]
            pub const TX_RCAL_1: u32 = 0x01;
            #[doc = "15% lower than mid-range"]
            pub const TX_RCAL_2: u32 = 0x02;
            #[doc = "25% lower than mid-range. Lowest impedance setting"]
            pub const TX_RCAL_3: u32 = 0x03;
        }
    }
}
#[doc = "AUTO_PD_EN"]
pub mod AUTO_PD_EN {
    #[doc = "DPHY AUTO_PD_EN input"]
    pub mod AUTO_PD_EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Inactive lanes are powered up and driving LP11"]
            pub const PWR_UP: u32 = 0;
            #[doc = "inactive lanes are powered down"]
            pub const PWR_DWN: u32 = 0x01;
        }
    }
}
#[doc = "RXLPRP"]
pub mod RXLPRP {
    #[doc = "DPHY RXLPRP input"]
    pub mod RXLPRP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "RXCDRP"]
pub mod RXCDRP {
    #[doc = "DPHY RXCDRP input"]
    pub mod RXCDRP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "344mV"]
            pub const RXCDRP_0: u32 = 0;
            #[doc = "325mV (Default)"]
            pub const RXCDRP_1: u32 = 0x01;
            #[doc = "307mV"]
            pub const RXCDRP_2: u32 = 0x02;
            #[doc = "Invalid"]
            pub const RXCDRP_3: u32 = 0x03;
        }
    }
}
