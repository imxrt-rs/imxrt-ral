#[doc = "MECC64"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "Error Interrupt Status Register"]
    pub ERR_STATUS: crate::RWRegister<u32>,
    #[doc = "Error Interrupt Status Enable Register"]
    pub ERR_STAT_EN: crate::RWRegister<u32>,
    #[doc = "Error Interrupt Enable Register"]
    pub ERR_SIG_EN: crate::RWRegister<u32>,
    #[doc = "Error Injection On LOW 32 bits Of OCRAM Bank0 Write Data"]
    pub ERR_DATA_INJ_LOW0: crate::RWRegister<u32>,
    #[doc = "Error Injection On HIGH 32 bits Of OCRAM Bank0 Write Data"]
    pub ERR_DATA_INJ_HIGH0: crate::RWRegister<u32>,
    #[doc = "Error Injection On 8 bits ECC code Of OCRAM Bank0 Write Data"]
    pub ERR_ECC_INJ0: crate::RWRegister<u32>,
    #[doc = "Error Injection On LOW 32 bits Of OCRAM Bank1 Write Data"]
    pub ERR_DATA_INJ_LOW1: crate::RWRegister<u32>,
    #[doc = "Error Injection On HIGH 32 bits Of OCRAM Bank1 Write Data"]
    pub ERR_DATA_INJ_HIGH1: crate::RWRegister<u32>,
    #[doc = "Error Injection On 8 bits ECC code Of OCRAM Bank1 Write Data"]
    pub ERR_ECC_INJ1: crate::RWRegister<u32>,
    #[doc = "Error Injection On LOW 32 bits Of OCRAM Bank2 Write Data"]
    pub ERR_DATA_INJ_LOW2: crate::RWRegister<u32>,
    #[doc = "Error Injection On HIGH 32 bits Of OCRAM Bank2 Write Data"]
    pub ERR_DATA_INJ_HIGH2: crate::RWRegister<u32>,
    #[doc = "Error Injection On 8 bits ECC code Of OCRAM Bank2 Write Data"]
    pub ERR_ECC_INJ2: crate::RWRegister<u32>,
    #[doc = "Error Injection On LOW 32 bits Of OCRAM Bank3 Write Data"]
    pub ERR_DATA_INJ_LOW3: crate::RWRegister<u32>,
    #[doc = "Error Injection On HIGH 32 bits Of OCRAM Bank3 Write Data"]
    pub ERR_DATA_INJ_HIGH3: crate::RWRegister<u32>,
    #[doc = "Error Injection On 8 bits ECC code Of OCRAM Bank3 Write Data"]
    pub ERR_ECC_INJ3: crate::RWRegister<u32>,
    #[doc = "Single Error Address And ECC code On OCRAM Bank0"]
    pub SINGLE_ERR_ADDR_ECC0: crate::RORegister<u32>,
    #[doc = "LOW 32 Bits Single Error Read Data On OCRAM Bank0"]
    pub SINGLE_ERR_DATA_LOW0: crate::RORegister<u32>,
    #[doc = "HIGH 32 Bits Single Error Read Data On OCRAM Bank0"]
    pub SINGLE_ERR_DATA_HIGH0: crate::RORegister<u32>,
    #[doc = "LOW Single Error Bit Position On OCRAM Bank0"]
    pub SINGLE_ERR_POS_LOW0: crate::RORegister<u32>,
    #[doc = "HIGH Single Error Bit Position On OCRAM Bank0"]
    pub SINGLE_ERR_POS_HIGH0: crate::RORegister<u32>,
    #[doc = "Single Error Address And ECC code On OCRAM Bank1"]
    pub SINGLE_ERR_ADDR_ECC1: crate::RORegister<u32>,
    #[doc = "LOW 32 Bits Single Error Read Data On OCRAM Bank1"]
    pub SINGLE_ERR_DATA_LOW1: crate::RORegister<u32>,
    #[doc = "HIGH 32 Bits Single Error Read Data On OCRAM Bank1"]
    pub SINGLE_ERR_DATA_HIGH1: crate::RORegister<u32>,
    #[doc = "LOW Single Error Bit Position On OCRAM Bank1"]
    pub SINGLE_ERR_POS_LOW1: crate::RORegister<u32>,
    #[doc = "HIGH Single Error Bit Position On OCRAM Bank1"]
    pub SINGLE_ERR_POS_HIGH1: crate::RORegister<u32>,
    #[doc = "Single Error Address And ECC code On OCRAM Bank2"]
    pub SINGLE_ERR_ADDR_ECC2: crate::RORegister<u32>,
    #[doc = "LOW 32 Bits Single Error Read Data On OCRAM Bank2"]
    pub SINGLE_ERR_DATA_LOW2: crate::RORegister<u32>,
    #[doc = "HIGH 32 Bits Single Error Read Data On OCRAM Bank2"]
    pub SINGLE_ERR_DATA_HIGH2: crate::RORegister<u32>,
    #[doc = "LOW Single Error Bit Position On OCRAM Bank2"]
    pub SINGLE_ERR_POS_LOW2: crate::RORegister<u32>,
    #[doc = "HIGH Single Error Bit Position On OCRAM Bank2"]
    pub SINGLE_ERR_POS_HIGH2: crate::RORegister<u32>,
    #[doc = "Single Error Address And ECC code On OCRAM Bank3"]
    pub SINGLE_ERR_ADDR_ECC3: crate::RORegister<u32>,
    #[doc = "LOW 32 Bits Single Error Read Data On OCRAM Bank3"]
    pub SINGLE_ERR_DATA_LOW3: crate::RORegister<u32>,
    #[doc = "HIGH 32 Bits Single Error Read Data On OCRAM Bank3"]
    pub SINGLE_ERR_DATA_HIGH3: crate::RORegister<u32>,
    #[doc = "LOW Single Error Bit Position On OCRAM Bank3"]
    pub SINGLE_ERR_POS_LOW3: crate::RORegister<u32>,
    #[doc = "HIGH Single Error Bit Position On OCRAM Bank3"]
    pub SINGLE_ERR_POS_HIGH3: crate::RORegister<u32>,
    #[doc = "Multiple Error Address And ECC code On OCRAM Bank0"]
    pub MULTI_ERR_ADDR_ECC0: crate::RORegister<u32>,
    #[doc = "LOW 32 Bits Multiple Error Read Data On OCRAM Bank0"]
    pub MULTI_ERR_DATA_LOW0: crate::RORegister<u32>,
    #[doc = "HIGH 32 Bits Multiple Error Read Data On OCRAM Bank0"]
    pub MULTI_ERR_DATA_HIGH0: crate::RORegister<u32>,
    #[doc = "Multiple Error Address And ECC code On OCRAM Bank1"]
    pub MULTI_ERR_ADDR_ECC1: crate::RORegister<u32>,
    #[doc = "LOW 32 Bits Multiple Error Read Data On OCRAM Bank1"]
    pub MULTI_ERR_DATA_LOW1: crate::RORegister<u32>,
    #[doc = "HIGH 32 Bits Multiple Error Read Data On OCRAM Bank1"]
    pub MULTI_ERR_DATA_HIGH1: crate::RORegister<u32>,
    #[doc = "Multiple Error Address And ECC code On OCRAM Bank2"]
    pub MULTI_ERR_ADDR_ECC2: crate::RORegister<u32>,
    #[doc = "LOW 32 Bits Multiple Error Read Data On OCRAM Bank2"]
    pub MULTI_ERR_DATA_LOW2: crate::RORegister<u32>,
    #[doc = "HIGH 32 Bits Multiple Error Read Data On OCRAM Bank2"]
    pub MULTI_ERR_DATA_HIGH2: crate::RORegister<u32>,
    #[doc = "Multiple Error Address And ECC code On OCRAM Bank3"]
    pub MULTI_ERR_ADDR_ECC3: crate::RORegister<u32>,
    #[doc = "LOW 32 Bits Multiple Error Read Data On OCRAM Bank3"]
    pub MULTI_ERR_DATA_LOW3: crate::RORegister<u32>,
    #[doc = "HIGH 32 Bits Multiple Error Read Data On OCRAM Bank3"]
    pub MULTI_ERR_DATA_HIGH3: crate::RORegister<u32>,
    _reserved0: [u8; 0x44],
    #[doc = "OCRAM Pipeline And ECC Enable"]
    pub PIPE_ECC_EN: crate::RWRegister<u32>,
    #[doc = "Pending Status"]
    pub PENDING_STAT: crate::RORegister<u32>,
}
#[doc = "Error Interrupt Status Register"]
pub mod ERR_STATUS {
    #[doc = "Single Bit Error On OCRAM Bank0"]
    pub mod SINGLE_ERR0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Single bit error does not happen on OCRAM bank0."]
            pub const SINGLE_ERR0_0: u32 = 0;
            #[doc = "Single bit error happens on OCRAM bank0."]
            pub const SINGLE_ERR0_1: u32 = 0x01;
        }
    }
    #[doc = "Single Bit Error On OCRAM Bank1"]
    pub mod SINGLE_ERR1 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Single bit error does not happen on OCRAM bank1."]
            pub const SINGLE_ERR1_0: u32 = 0;
            #[doc = "Single bit error happens on OCRAM bank1."]
            pub const SINGLE_ERR1_1: u32 = 0x01;
        }
    }
    #[doc = "Single Bit Error On OCRAM Bank2"]
    pub mod SINGLE_ERR2 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Single bit error does not happen on OCRAM bank2."]
            pub const SINGLE_ERR2_0: u32 = 0;
            #[doc = "Single bit error happens on OCRAM bank2."]
            pub const SINGLE_ERR2_1: u32 = 0x01;
        }
    }
    #[doc = "Single Bit Error On OCRAM Bank3"]
    pub mod SINGLE_ERR3 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Single bit error does not happen on OCRAM bank3."]
            pub const SINGLE_ERR3_0: u32 = 0;
            #[doc = "Single bit error happens on OCRAM bank3."]
            pub const SINGLE_ERR3_1: u32 = 0x01;
        }
    }
    #[doc = "Multiple Bits Error On OCRAM Bank0"]
    pub mod MULTI_ERR0 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Multiple bits error does not happen on OCRAM bank0."]
            pub const MULTI_ERR0_0: u32 = 0;
            #[doc = "Multiple bits error happens on OCRAM bank0."]
            pub const MULTI_ERR0_1: u32 = 0x01;
        }
    }
    #[doc = "Multiple Bits Error On OCRAM Bank1"]
    pub mod MULTI_ERR1 {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Multiple bits error does not happen on OCRAM bank1."]
            pub const MULTI_ERR1_0: u32 = 0;
            #[doc = "Multiple bits error happens on OCRAM bank1."]
            pub const MULTI_ERR1_1: u32 = 0x01;
        }
    }
    #[doc = "Multiple Bits Error On OCRAM Bank2"]
    pub mod MULTI_ERR2 {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Multiple bits error does not happen on OCRAM bank2."]
            pub const MULTI_ERR2_0: u32 = 0;
            #[doc = "Multiple bits error happens on OCRAM bank2."]
            pub const MULTI_ERR2_1: u32 = 0x01;
        }
    }
    #[doc = "Multiple Bits Error On OCRAM Bank3"]
    pub mod MULTI_ERR3 {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Multiple bits error does not happen on OCRAM bank3."]
            pub const MULTI_ERR3_0: u32 = 0;
            #[doc = "Multiple bits error happens on OCRAM bank3."]
            pub const MULTI_ERR3_1: u32 = 0x01;
        }
    }
    #[doc = "AXI Strobe Error On OCRAM Bank0"]
    pub mod STRB_ERR0 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "AXI strobe error does not happen on OCRAM bank0."]
            pub const STRB_ERR0_0: u32 = 0;
            #[doc = "AXI strobe error happens on OCRAM bank0."]
            pub const STRB_ERR0_1: u32 = 0x01;
        }
    }
    #[doc = "AXI Strobe Error On OCRAM Bank1"]
    pub mod STRB_ERR1 {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "AXI strobe error does not happen on OCRAM bank1."]
            pub const STRB_ERR1_0: u32 = 0;
            #[doc = "AXI strobe error happens on OCRAM bank1."]
            pub const STRB_ERR1_1: u32 = 0x01;
        }
    }
    #[doc = "AXI Strobe Error On OCRAM Bank2"]
    pub mod STRB_ERR2 {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "AXI strobe error does not happen on OCRAM bank2."]
            pub const STRB_ERR2_0: u32 = 0;
            #[doc = "AXI strobe error happens on OCRAM bank2."]
            pub const STRB_ERR2_1: u32 = 0x01;
        }
    }
    #[doc = "AXI Strobe Error On OCRAM Bank3"]
    pub mod STRB_ERR3 {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "AXI strobe error does not happen on OCRAM bank3."]
            pub const STRB_ERR3_0: u32 = 0;
            #[doc = "AXI strobe error happens on OCRAM bank3."]
            pub const STRB_ERR3_1: u32 = 0x01;
        }
    }
    #[doc = "OCRAM Access Error On Bank0"]
    pub mod ADDR_ERR0 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "OCRAM access error does not happen on OCRAM bank0."]
            pub const ADDR_ERR0_0: u32 = 0;
            #[doc = "OCRAM access error happens on OCRAM bank0."]
            pub const ADDR_ERR0_1: u32 = 0x01;
        }
    }
    #[doc = "OCRAM Access Error On Bank1"]
    pub mod ADDR_ERR1 {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "OCRAM access error does not happen on OCRAM bank1."]
            pub const ADDR_ERR1_0: u32 = 0;
            #[doc = "OCRAM access error happens on OCRAM bank1."]
            pub const ADDR_ERR1_1: u32 = 0x01;
        }
    }
    #[doc = "OCRAM Access Error On Bank2"]
    pub mod ADDR_ERR2 {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "OCRAM access error does not happen on OCRAM bank2."]
            pub const ADDR_ERR2_0: u32 = 0;
            #[doc = "OCRAM access error happens on OCRAM bank2."]
            pub const ADDR_ERR2_1: u32 = 0x01;
        }
    }
    #[doc = "OCRAM Access Error On Bank3"]
    pub mod ADDR_ERR3 {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "OCRAM access error does not happen on OCRAM bank3."]
            pub const ADDR_ERR3_0: u32 = 0;
            #[doc = "OCRAM access error happens on OCRAM bank3."]
            pub const ADDR_ERR3_1: u32 = 0x01;
        }
    }
}
#[doc = "Error Interrupt Status Enable Register"]
pub mod ERR_STAT_EN {
    #[doc = "Single Bit Error Status Enable On OCRAM Bank0"]
    pub mod SINGLE_ERR0_STAT_EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const SINGLE_ERR0_STAT_EN_0: u32 = 0;
            #[doc = "Enabled"]
            pub const SINGLE_ERR0_STAT_EN_1: u32 = 0x01;
        }
    }
    #[doc = "Single Bit Error Status Enable On OCRAM Bank1"]
    pub mod SINGLE_ERR1_STAT_EN {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const SINGLE_ERR1_STAT_EN_0: u32 = 0;
            #[doc = "Enabled"]
            pub const SINGLE_ERR1_STAT_EN_1: u32 = 0x01;
        }
    }
    #[doc = "Single Bit Error Status Enable On OCRAM Bank2"]
    pub mod SINGLE_ERR2_STAT_EN {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const SINGLE_ERR2_STAT_EN_0: u32 = 0;
            #[doc = "Enabled"]
            pub const SINGLE_ERR2_STAT_EN_1: u32 = 0x01;
        }
    }
    #[doc = "Single Bit Error Status Enable On OCRAM Bank3"]
    pub mod SINGLE_ERR3_STAT_EN {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const SINGLE_ERR3_STAT_EN_0: u32 = 0;
            #[doc = "Enabled"]
            pub const SINGLE_ERR3_STAT_EN_1: u32 = 0x01;
        }
    }
    #[doc = "Multiple Bits Error Status Enable On OCRAM Bank0"]
    pub mod MULTI_ERR0_STAT_EN {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const MULTI_ERR0_STAT_EN_0: u32 = 0;
            #[doc = "Enabled"]
            pub const MULTI_ERR0_STAT_EN_1: u32 = 0x01;
        }
    }
    #[doc = "Multiple Bits Error Status Enable On OCRAM Bank1"]
    pub mod MULTI_ERR1_STAT_EN {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const MULTI_ERR1_STAT_EN_0: u32 = 0;
            #[doc = "Enabled"]
            pub const MULTI_ERR1_STAT_EN_1: u32 = 0x01;
        }
    }
    #[doc = "Multiple Bits Error Status Enable On OCRAM Bank2"]
    pub mod MULTI_ERR2_STAT_EN {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const MULTI_ERR2_STAT_EN_0: u32 = 0;
            #[doc = "Enabled"]
            pub const MULTI_ERR2_STAT_EN_1: u32 = 0x01;
        }
    }
    #[doc = "Multiple Bits Error Status Enable On OCRAM Bank3"]
    pub mod MULTI_ERR3_STAT_EN {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const MULTI_ERR3_STAT_EN_0: u32 = 0;
            #[doc = "Enabled"]
            pub const MULTI_ERR3_STAT_EN_1: u32 = 0x01;
        }
    }
    #[doc = "AXI Strobe Error Status Enable On OCRAM Bank0"]
    pub mod STRB_ERR0_STAT_EN {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const STRB_ERR0_STAT_EN_0: u32 = 0;
            #[doc = "Enabled"]
            pub const STRB_ERR0_STAT_EN_1: u32 = 0x01;
        }
    }
    #[doc = "AXI Strobe Error Status Enable On OCRAM Bank1"]
    pub mod STRB_ERR1_STAT_EN {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const STRB_ERR1_STAT_EN_0: u32 = 0;
            #[doc = "Enabled"]
            pub const STRB_ERR1_STAT_EN_1: u32 = 0x01;
        }
    }
    #[doc = "AXI Strobe Error Status Enable On OCRAM Bank2"]
    pub mod STRB_ERR2_STAT_EN {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const STRB_ERR2_STAT_EN_0: u32 = 0;
            #[doc = "Enabled"]
            pub const STRB_ERR2_STAT_EN_1: u32 = 0x01;
        }
    }
    #[doc = "AXI Strobe Error Status Enable On OCRAM Bank3"]
    pub mod STRB_ERR3_STAT_EN {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const STRB_ERR3_STAT_EN_0: u32 = 0;
            #[doc = "Enabled"]
            pub const STRB_ERR3_STAT_EN_1: u32 = 0x01;
        }
    }
    #[doc = "OCRAM Access Error Status Enable On Bank0"]
    pub mod ADDR_ERR0_STAT_EN {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const ADDR_ERR0_STAT_EN_0: u32 = 0;
            #[doc = "Enabled"]
            pub const ADDR_ERR0_STAT_EN_1: u32 = 0x01;
        }
    }
    #[doc = "OCRAM Access Error Status Enable On Bank1"]
    pub mod ADDR_ERR1_STAT_EN {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const ADDR_ERR1_STAT_EN_0: u32 = 0;
            #[doc = "Enabled"]
            pub const ADDR_ERR1_STAT_EN_1: u32 = 0x01;
        }
    }
    #[doc = "OCRAM Access Error Status Enable On Bank2"]
    pub mod ADDR_ERR2_STAT_EN {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const ADDR_ERR2_STAT_EN_0: u32 = 0;
            #[doc = "Enabled"]
            pub const ADDR_ERR2_STAT_EN_1: u32 = 0x01;
        }
    }
    #[doc = "OCRAM Access Error Status Enable On Bank3"]
    pub mod ADDR_ERR3_STAT_EN {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const ADDR_ERR3_STAT_EN_0: u32 = 0;
            #[doc = "Enabled"]
            pub const ADDR_ERR3_STAT_EN_1: u32 = 0x01;
        }
    }
}
#[doc = "Error Interrupt Enable Register"]
pub mod ERR_SIG_EN {
    #[doc = "Single Bit Error Interrupt Enable On OCRAM Bank0"]
    pub mod SINGLE_ERR0_SIG_EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const SINGLE_ERR0_SIG_EN_0: u32 = 0;
            #[doc = "Enabled"]
            pub const SINGLE_ERR0_SIG_EN_1: u32 = 0x01;
        }
    }
    #[doc = "Single Bit Error Interrupt Enable On OCRAM Bank1"]
    pub mod SINGLE_ERR1_SIG_EN {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const SINGLE_ERR1_SIG_EN_0: u32 = 0;
            #[doc = "Enabled"]
            pub const SINGLE_ERR1_SIG_EN_1: u32 = 0x01;
        }
    }
    #[doc = "Single Bit Error Interrupt Enable On OCRAM Bank2"]
    pub mod SINGLE_ERR2_SIG_EN {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const SINGLE_ERR2_SIG_EN_0: u32 = 0;
            #[doc = "Enabled"]
            pub const SINGLE_ERR2_SIG_EN_1: u32 = 0x01;
        }
    }
    #[doc = "Single Bit Error Interrupt Enable On OCRAM Bank3"]
    pub mod SINGLE_ERR3_SIG_EN {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const SINGLE_ERR3_SIG_EN_0: u32 = 0;
            #[doc = "Enabled"]
            pub const SINGLE_ERR3_SIG_EN_1: u32 = 0x01;
        }
    }
    #[doc = "Multiple Bits Error Interrupt Enable On OCRAM Bank0"]
    pub mod MULTI_ERR0_SIG_EN {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const MULTI_ERR0_SIG_EN_0: u32 = 0;
            #[doc = "Enabled"]
            pub const MULTI_ERR0_SIG_EN_1: u32 = 0x01;
        }
    }
    #[doc = "Multiple Bits Error Interrupt Enable On OCRAM Bank1"]
    pub mod MULTI_ERR1_SIG_EN {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const MULTI_ERR1_SIG_EN_0: u32 = 0;
            #[doc = "Enabled"]
            pub const MULTI_ERR1_SIG_EN_1: u32 = 0x01;
        }
    }
    #[doc = "Multiple Bits Error Interrupt Enable On OCRAM Bank2"]
    pub mod MULTI_ERR2_SIG_EN {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const MULTI_ERR2_SIG_EN_0: u32 = 0;
            #[doc = "Enabled"]
            pub const MULTI_ERR2_SIG_EN_1: u32 = 0x01;
        }
    }
    #[doc = "Multiple Bits Error Interrupt Enable On OCRAM Bank3"]
    pub mod MULTI_ERR3_SIG_EN {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const MULTI_ERR3_SIG_EN_0: u32 = 0;
            #[doc = "Enabled"]
            pub const MULTI_ERR3_SIG_EN_1: u32 = 0x01;
        }
    }
    #[doc = "AXI Strobe Error Interrupt Enable On OCRAM Bank0"]
    pub mod STRB_ERR0_SIG_EN {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const STRB_ERR0_SIG_EN_0: u32 = 0;
            #[doc = "Enabled"]
            pub const STRB_ERR0_SIG_EN_1: u32 = 0x01;
        }
    }
    #[doc = "AXI Strobe Error Interrupt Enable On OCRAM Bank1"]
    pub mod STRB_ERR1_SIG_EN {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const STRB_ERR1_SIG_EN_0: u32 = 0;
            #[doc = "Enabled"]
            pub const STRB_ERR1_SIG_EN_1: u32 = 0x01;
        }
    }
    #[doc = "AXI Strobe Error Interrupt Enable On OCRAM Bank2"]
    pub mod STRB_ERR2_SIG_EN {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const STRB_ERR2_SIG_EN_0: u32 = 0;
            #[doc = "Enabled"]
            pub const STRB_ERR2_SIG_EN_1: u32 = 0x01;
        }
    }
    #[doc = "AXI Strobe Error Interrupt Enable On OCRAM Bank3"]
    pub mod STRB_ERR3_SIG_EN {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const STRB_ERR3_SIG_EN_0: u32 = 0;
            #[doc = "Enabled"]
            pub const STRB_ERR3_SIG_EN_1: u32 = 0x01;
        }
    }
    #[doc = "OCRAM Access Error Interrupt Enable On Bank0"]
    pub mod ADDR_ERR0_SIG_EN {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const ADDR_ERR0_SIG_EN_0: u32 = 0;
            #[doc = "Enabled"]
            pub const ADDR_ERR0_SIG_EN_1: u32 = 0x01;
        }
    }
    #[doc = "OCRAM Access Error Interrupt Enable On Bank1"]
    pub mod ADDR_ERR1_SIG_EN {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const ADDR_ERR1_SIG_EN_0: u32 = 0;
            #[doc = "Enabled"]
            pub const ADDR_ERR1_SIG_EN_1: u32 = 0x01;
        }
    }
    #[doc = "OCRAM Access Error Interrupt Enable On Bank2"]
    pub mod ADDR_ERR2_SIG_EN {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const ADDR_ERR2_SIG_EN_0: u32 = 0;
            #[doc = "Enabled"]
            pub const ADDR_ERR2_SIG_EN_1: u32 = 0x01;
        }
    }
    #[doc = "OCRAM Access Error Interrupt Enable On Bank3"]
    pub mod ADDR_ERR3_SIG_EN {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const ADDR_ERR3_SIG_EN_0: u32 = 0;
            #[doc = "Enabled"]
            pub const ADDR_ERR3_SIG_EN_1: u32 = 0x01;
        }
    }
}
#[doc = "Error Injection On LOW 32 bits Of OCRAM Bank0 Write Data"]
pub mod ERR_DATA_INJ_LOW0 {
    #[doc = "Error Injection On LOW 32 bits Of OCRAM Bank0 Write Data"]
    pub mod ERR_DATA_INJ {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Error Injection On HIGH 32 bits Of OCRAM Bank0 Write Data"]
pub mod ERR_DATA_INJ_HIGH0 {
    #[doc = "Error Injection On HIGH 32 bits Of OCRAM Bank0 Write Data"]
    pub mod ERR_DATA_INJ {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Error Injection On 8 bits ECC code Of OCRAM Bank0 Write Data"]
pub mod ERR_ECC_INJ0 {
    #[doc = "Error Injection On 8 bits ECC code Of OCRAM Bank0 Write Data"]
    pub mod ERR_ECC_INJ {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Error Injection On LOW 32 bits Of OCRAM Bank1 Write Data"]
pub mod ERR_DATA_INJ_LOW1 {
    #[doc = "Error Injection On LOW 32 bits Of OCRAM Bank1 Write Data"]
    pub mod ERR_DATA_INJ {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Error Injection On HIGH 32 bits Of OCRAM Bank1 Write Data"]
pub mod ERR_DATA_INJ_HIGH1 {
    #[doc = "Error Injection On HIGH 32 bits Of OCRAM Bank1 Write Data"]
    pub mod ERR_DATA_INJ {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Error Injection On 8 bits ECC code Of OCRAM Bank1 Write Data"]
pub mod ERR_ECC_INJ1 {
    #[doc = "Error Injection On 8 bits ECC code Of OCRAM Bank1 Write Data"]
    pub mod ERR_ECC_INJ {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Error Injection On LOW 32 bits Of OCRAM Bank2 Write Data"]
pub mod ERR_DATA_INJ_LOW2 {
    #[doc = "Error Injection On LOW 32 bits Of OCRAM Bank2 Write Data"]
    pub mod ERR_DATA_INJ {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Error Injection On HIGH 32 bits Of OCRAM Bank2 Write Data"]
pub mod ERR_DATA_INJ_HIGH2 {
    #[doc = "Error Injection On HIGH 32 bits Of OCRAM Bank2 Write Data"]
    pub mod ERR_DATA_INJ {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Error Injection On 8 bits ECC code Of OCRAM Bank2 Write Data"]
pub mod ERR_ECC_INJ2 {
    #[doc = "Error Injection On 8 bits ECC code Of OCRAM Bank2 Write Data"]
    pub mod ERR_ECC_INJ {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Error Injection On LOW 32 bits Of OCRAM Bank3 Write Data"]
pub mod ERR_DATA_INJ_LOW3 {
    #[doc = "Error Injection On LOW 32 bits Of OCRAM Bank3 Write Data"]
    pub mod ERR_DATA_INJ {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Error Injection On HIGH 32 bits Of OCRAM Bank3 Write Data"]
pub mod ERR_DATA_INJ_HIGH3 {
    #[doc = "Error Injection On HIGH 32 bits Of OCRAM Bank3 Write Data"]
    pub mod ERR_DATA_INJ {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Error Injection On 8 bits ECC code Of OCRAM Bank3 Write Data"]
pub mod ERR_ECC_INJ3 {
    #[doc = "Error Injection On 8 bits ECC code Of OCRAM Bank3 Write Data"]
    pub mod ERR_ECC_INJ {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Single Error Address And ECC code On OCRAM Bank0"]
pub mod SINGLE_ERR_ADDR_ECC0 {
    #[doc = "Single Error ECC code On OCRAM Bank0"]
    pub mod SINGLE_ERR_ECC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Single Error Address On OCRAM Bank0"]
    pub mod SINGLE_ERR_ADDR {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0007_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "LOW 32 Bits Single Error Read Data On OCRAM Bank0"]
pub mod SINGLE_ERR_DATA_LOW0 {
    #[doc = "LOW 32 Bits Single Error Read Data On OCRAM Bank0"]
    pub mod SINGLE_ERR_DATA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "HIGH 32 Bits Single Error Read Data On OCRAM Bank0"]
pub mod SINGLE_ERR_DATA_HIGH0 {
    #[doc = "HIGH 32 Bits Single Error Read Data On OCRAM Bank0"]
    pub mod SINGLE_ERR_DATA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "LOW Single Error Bit Position On OCRAM Bank0"]
pub mod SINGLE_ERR_POS_LOW0 {
    #[doc = "LOW Single Error Bit Position On OCRAM Bank0"]
    pub mod SINGLE_ERR_POS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "HIGH Single Error Bit Position On OCRAM Bank0"]
pub mod SINGLE_ERR_POS_HIGH0 {
    #[doc = "HIGH Single Error Bit Position On OCRAM Bank0"]
    pub mod SINGLE_ERR_POS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Single Error Address And ECC code On OCRAM Bank1"]
pub mod SINGLE_ERR_ADDR_ECC1 {
    #[doc = "Single Error ECC code On OCRAM Bank1"]
    pub mod SINGLE_ERR_ECC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Single Error Address On OCRAM Bank1"]
    pub mod SINGLE_ERR_ADDR {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0007_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "LOW 32 Bits Single Error Read Data On OCRAM Bank1"]
pub mod SINGLE_ERR_DATA_LOW1 {
    #[doc = "LOW 32 Bits Single Error Read Data On OCRAM Bank1"]
    pub mod SINGLE_ERR_DATA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "HIGH 32 Bits Single Error Read Data On OCRAM Bank1"]
pub mod SINGLE_ERR_DATA_HIGH1 {
    #[doc = "HIGH 32 Bits Single Error Read Data On OCRAM Bank1"]
    pub mod SINGLE_ERR_DATA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "LOW Single Error Bit Position On OCRAM Bank1"]
pub mod SINGLE_ERR_POS_LOW1 {
    #[doc = "LOW Single Error Bit Position On OCRAM Bank1"]
    pub mod SINGLE_ERR_POS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "HIGH Single Error Bit Position On OCRAM Bank1"]
pub mod SINGLE_ERR_POS_HIGH1 {
    #[doc = "HIGH Single Error Bit Position On OCRAM Bank1"]
    pub mod SINGLE_ERR_POS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Single Error Address And ECC code On OCRAM Bank2"]
pub mod SINGLE_ERR_ADDR_ECC2 {
    #[doc = "Single Error ECC code On OCRAM Bank2"]
    pub mod SINGLE_ERR_ECC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Single Error Address On OCRAM Bank2"]
    pub mod SINGLE_ERR_ADDR {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0007_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "LOW 32 Bits Single Error Read Data On OCRAM Bank2"]
pub mod SINGLE_ERR_DATA_LOW2 {
    #[doc = "LOW 32 Bits Single Error Read Data On OCRAM Bank2"]
    pub mod SINGLE_ERR_DATA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "HIGH 32 Bits Single Error Read Data On OCRAM Bank2"]
pub mod SINGLE_ERR_DATA_HIGH2 {
    #[doc = "HIGH 32 Bits Single Error Read Data On OCRAM Bank2"]
    pub mod SINGLE_ERR_DATA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "LOW Single Error Bit Position On OCRAM Bank2"]
pub mod SINGLE_ERR_POS_LOW2 {
    #[doc = "LOW Single Error Bit Position On OCRAM Bank2"]
    pub mod SINGLE_ERR_POS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "HIGH Single Error Bit Position On OCRAM Bank2"]
pub mod SINGLE_ERR_POS_HIGH2 {
    #[doc = "HIGH Single Error Bit Position On OCRAM Bank2"]
    pub mod SINGLE_ERR_POS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Single Error Address And ECC code On OCRAM Bank3"]
pub mod SINGLE_ERR_ADDR_ECC3 {
    #[doc = "Single Error ECC code On OCRAM Bank3"]
    pub mod SINGLE_ERR_ECC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Single Error Address On OCRAM Bank3"]
    pub mod SINGLE_ERR_ADDR {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0007_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "LOW 32 Bits Single Error Read Data On OCRAM Bank3"]
pub mod SINGLE_ERR_DATA_LOW3 {
    #[doc = "LOW 32 Bits Single Error Read Data On OCRAM Bank3"]
    pub mod SINGLE_ERR_DATA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "HIGH 32 Bits Single Error Read Data On OCRAM Bank3"]
pub mod SINGLE_ERR_DATA_HIGH3 {
    #[doc = "HIGH 32 Bits Single Error Read Data On OCRAM Bank3"]
    pub mod SINGLE_ERR_DATA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "LOW Single Error Bit Position On OCRAM Bank3"]
pub mod SINGLE_ERR_POS_LOW3 {
    #[doc = "LOW Single Error Bit Position On OCRAM Bank3"]
    pub mod SINGLE_ERR_POS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "HIGH Single Error Bit Position On OCRAM Bank3"]
pub mod SINGLE_ERR_POS_HIGH3 {
    #[doc = "HIGH Single Error Bit Position On OCRAM Bank3"]
    pub mod SINGLE_ERR_POS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Multiple Error Address And ECC code On OCRAM Bank0"]
pub mod MULTI_ERR_ADDR_ECC0 {
    #[doc = "Multiple Error ECC code On OCRAM Bank0"]
    pub mod MULTI_ERR_ECC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Multiple Error Address On OCRAM Bank0"]
    pub mod MULTI_ERR_ADDR {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0007_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "LOW 32 Bits Multiple Error Read Data On OCRAM Bank0"]
pub mod MULTI_ERR_DATA_LOW0 {
    #[doc = "LOW 32 Bits Multiple Error Read Data On OCRAM Bank0"]
    pub mod MULTI_ERR_DATA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "HIGH 32 Bits Multiple Error Read Data On OCRAM Bank0"]
pub mod MULTI_ERR_DATA_HIGH0 {
    #[doc = "HIGH 32 Bits Multiple Error Read Data On OCRAM Bank0"]
    pub mod MULTI_ERR_DATA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Multiple Error Address And ECC code On OCRAM Bank1"]
pub mod MULTI_ERR_ADDR_ECC1 {
    #[doc = "Multiple Error ECC code On OCRAM Bank1"]
    pub mod MULTI_ERR_ECC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Multiple Error Address On OCRAM Bank1"]
    pub mod MULTI_ERR_ADDR {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0007_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "LOW 32 Bits Multiple Error Read Data On OCRAM Bank1"]
pub mod MULTI_ERR_DATA_LOW1 {
    #[doc = "LOW 32 Bits Multiple Error Read Data On OCRAM Bank1"]
    pub mod MULTI_ERR_DATA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "HIGH 32 Bits Multiple Error Read Data On OCRAM Bank1"]
pub mod MULTI_ERR_DATA_HIGH1 {
    #[doc = "HIGH 32 Bits Multiple Error Read Data On OCRAM Bank1"]
    pub mod MULTI_ERR_DATA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Multiple Error Address And ECC code On OCRAM Bank2"]
pub mod MULTI_ERR_ADDR_ECC2 {
    #[doc = "Multiple Error ECC code On OCRAM Bank2"]
    pub mod MULTI_ERR_ECC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Multiple Error Address On OCRAM Bank2"]
    pub mod MULTI_ERR_ADDR {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0007_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "LOW 32 Bits Multiple Error Read Data On OCRAM Bank2"]
pub mod MULTI_ERR_DATA_LOW2 {
    #[doc = "LOW 32 Bits Multiple Error Read Data On OCRAM Bank2"]
    pub mod MULTI_ERR_DATA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "HIGH 32 Bits Multiple Error Read Data On OCRAM Bank2"]
pub mod MULTI_ERR_DATA_HIGH2 {
    #[doc = "HIGH 32 Bits Multiple Error Read Data On OCRAM Bank2"]
    pub mod MULTI_ERR_DATA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Multiple Error Address And ECC code On OCRAM Bank3"]
pub mod MULTI_ERR_ADDR_ECC3 {
    #[doc = "Multiple Error ECC code On OCRAM Bank3"]
    pub mod MULTI_ERR_ECC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Multiple Error Address On OCRAM Bank3"]
    pub mod MULTI_ERR_ADDR {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0007_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "LOW 32 Bits Multiple Error Read Data On OCRAM Bank3"]
pub mod MULTI_ERR_DATA_LOW3 {
    #[doc = "LOW 32 Bits Multiple Error Read Data On OCRAM Bank3"]
    pub mod MULTI_ERR_DATA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "HIGH 32 Bits Multiple Error Read Data On OCRAM Bank3"]
pub mod MULTI_ERR_DATA_HIGH3 {
    #[doc = "HIGH 32 Bits Multiple Error Read Data On OCRAM Bank3"]
    pub mod MULTI_ERR_DATA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "OCRAM Pipeline And ECC Enable"]
pub mod PIPE_ECC_EN {
    #[doc = "Read Data Wait Enable"]
    pub mod READ_DATA_WAIT_EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable."]
            pub const READ_DATA_WAIT_EN_0: u32 = 0;
            #[doc = "Enable."]
            pub const READ_DATA_WAIT_EN_1: u32 = 0x01;
        }
    }
    #[doc = "Read Address Pipeline Enable"]
    pub mod READ_ADDR_PIPE_EN {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable."]
            pub const READ_ADDR_PIPE_EN_0: u32 = 0;
            #[doc = "Enable."]
            pub const READ_ADDR_PIPE_EN_1: u32 = 0x01;
        }
    }
    #[doc = "Write Data Pipeline Enable"]
    pub mod WRITE_DATA_PIPE_EN {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable."]
            pub const WRITE_DATA_PIPE_EN_0: u32 = 0;
            #[doc = "Enable."]
            pub const WRITE_DATA_PIPE_EN_1: u32 = 0x01;
        }
    }
    #[doc = "Write Address Pipeline Enable"]
    pub mod WRITE_ADDR_PIPE_EN {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable."]
            pub const WRITE_ADDR_PIPE_EN_0: u32 = 0;
            #[doc = "Enable."]
            pub const WRITE_ADDR_PIPE_EN_1: u32 = 0x01;
        }
    }
    #[doc = "ECC Function Enable"]
    pub mod ECC_EN {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable."]
            pub const ECC_EN_0: u32 = 0;
            #[doc = "Enable."]
            pub const ECC_EN_1: u32 = 0x01;
        }
    }
}
#[doc = "Pending Status"]
pub mod PENDING_STAT {
    #[doc = "Read Data Wait Pending"]
    pub mod READ_DATA_WAIT_PENDING {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No update pending status for READ_DATA_WAIT_EN."]
            pub const READ_DATA_WAIT_PENDING_0: u32 = 0;
            #[doc = "When READ_DATA_WAIT_EN register bit is changed, this register bit will be set until the new setup becomes valid in the controller."]
            pub const READ_DATA_WAIT_PENDING_1: u32 = 0x01;
        }
    }
    #[doc = "Read Address Pipeline Pending"]
    pub mod READ_ADDR_PIPE_PENDING {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No update pending status for READ_ADDR_PIPE_EN."]
            pub const READ_ADDR_PIPE_PENDING_0: u32 = 0;
            #[doc = "When READ_ADDR_PIPE_EN register bit is changed, this register bit will be set until the new setup becomes valid in the controller."]
            pub const READ_ADDR_PIPE_PENDING_1: u32 = 0x01;
        }
    }
    #[doc = "Write Data Pipeline Pending"]
    pub mod WRITE_DATA_PIPE_PENDING {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No update pending status for WRITE_DATA_PIPE_EN."]
            pub const WRITE_DATA_PIPE_PENDING_0: u32 = 0;
            #[doc = "When WRITE_DATA_PIPE_EN register bit is changed, this register bit will be set until the new setup becomes valid in the controller."]
            pub const WRITE_DATA_PIPE_PENDING_1: u32 = 0x01;
        }
    }
    #[doc = "Write Address Pipeline Pending"]
    pub mod WRITE_ADDR_PIPE_PENDING {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No update pending status for WRITE_ADDR_PIPE_EN."]
            pub const WRITE_ADDR_PIPE_PENDING_0: u32 = 0;
            #[doc = "When WRITE_ADDR_PIPE_EN register bit is changed, this register bit will be set until the new setup becomes valid in the controller."]
            pub const WRITE_ADDR_PIPE_PENDING_1: u32 = 0x01;
        }
    }
}
