#[doc = "Block Control WAKEUP Domain"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x04],
    #[doc = "IPG DEBUG mask bit"]
    pub IPG_DEBUG1: crate::RWRegister<u32>,
    #[doc = "IPG DEBUG mask bit"]
    pub IPG_DEBUG2: crate::RWRegister<u32>,
    #[doc = "IPG DEBUG mask bit"]
    pub IPG_DEBUG3: crate::RWRegister<u32>,
    _reserved1: [u8; 0x04],
    #[doc = "SSI master low power mode control"]
    pub SSI: crate::RWRegister<u32>,
    #[doc = "EtherCAT miscellaneous configuration"]
    pub ECAT_MISC_CFG: crate::RWRegister<u32>,
    #[doc = "DEXSC error response configuration"]
    pub DEXSC_ERR: crate::RWRegister<u32>,
    #[doc = "USBPHY miscellaneous control"]
    pub USBPHY_MISC_CTRL: crate::RWRegister<u32>,
    #[doc = "NETC Port miscellaneous configuration"]
    pub NETC_PORT_MISC_CFG: crate::RWRegister<u32>,
    #[doc = "M7 NMI interrupt clear register"]
    pub M7_NMI_CLR: crate::RWRegister<u32>,
    _reserved2: [u8; 0x04],
    #[doc = "Qtimer miscellaneous control register 1"]
    pub QTIMER_CTRL1: crate::RWRegister<u32>,
    #[doc = "Qtimer miscellaneous control register 2"]
    pub QTIMER_CTRL2: crate::RWRegister<u32>,
    #[doc = "SAI2 MCLK control register"]
    pub SAI2_MCLK_CTRL: crate::RWRegister<u32>,
    #[doc = "SAI3 MCLK control register"]
    pub SAI3_MCLK_CTRL: crate::RWRegister<u32>,
    #[doc = "SAI4 MCLK control register"]
    pub SAI4_MCLK_CTRL: crate::RWRegister<u32>,
    #[doc = "XBAR IO direction control register"]
    pub XBAR_DIR_CTRL1: crate::RWRegister<u32>,
    #[doc = "XBAR IO direction control register"]
    pub XBAR_DIR_CTRL2: crate::RWRegister<u32>,
    #[doc = "LPIT trigger input select register"]
    pub LPIT_TRIG_SEL: crate::RWRegister<u32>,
    #[doc = "AXI bus attribute configuration register"]
    pub AXI_ATTR_CFG: crate::RWRegister<u32>,
    #[doc = "SRAM Control Register 0"]
    pub SRAMCR0: crate::RWRegister<u32>,
    #[doc = "SRAM Control Register 1"]
    pub SRAMCR1: crate::RWRegister<u32>,
    _reserved3: [u8; 0x04],
    #[doc = "Slave stop mode configure register"]
    pub SLAVE_STOP_MODE_CFG: crate::RWRegister<u32>,
    _reserved4: [u8; 0x10],
    #[doc = "I3C2 async wakeup control register"]
    pub I3C2_ASYNC_WAKEUP_CTRL: crate::RWRegister<u32>,
    #[doc = "XBAR and AOI write protect register"]
    pub XBAR_AOI_WE: crate::RWRegister<u32>,
    #[doc = "XBAR trigger synchronizer control register1"]
    pub XBAR_TRIG_SYNC_CTRL1: crate::RWRegister<u32>,
    #[doc = "XBAR trigger synchronizer control register2"]
    pub XBAR_TRIG_SYNC_CTRL2: crate::RWRegister<u32>,
    _reserved5: [u8; 0x7c],
    #[doc = "NETC link configuration for port0"]
    pub NETC_LINK_CFG0: crate::RWRegister<u32>,
    #[doc = "NETC link configuration for port1"]
    pub NETC_LINK_CFG1: crate::RWRegister<u32>,
    #[doc = "NETC link configuration for port2"]
    pub NETC_LINK_CFG2: crate::RWRegister<u32>,
    #[doc = "NETC link configuration for port3"]
    pub NETC_LINK_CFG3: crate::RWRegister<u32>,
    #[doc = "NETC link configuration for port4"]
    pub NETC_LINK_CFG4: crate::RWRegister<u32>,
    #[doc = "NETC RevMII RGMII delay line configuration for port0"]
    pub NETC_REVMII_DLL0: crate::RWRegister<u32>,
    #[doc = "NETC RevMII RGMII delay line configuration for port1"]
    pub NETC_REVMII_DLL1: crate::RWRegister<u32>,
    #[doc = "NETC RevMII RGMII delay line configuration for port2"]
    pub NETC_REVMII_DLL2: crate::RWRegister<u32>,
    #[doc = "NETC RevMII RGMII delay line configuration for port3"]
    pub NETC_REVMII_DLL3: crate::RWRegister<u32>,
    #[doc = "NETC RevMII RGMII delay line configuration for port4"]
    pub NETC_REVMII_DLL4: crate::RWRegister<u32>,
    _reserved6: [u8; 0x08],
    #[doc = "Safety clock monitor control and status register"]
    pub SAFETY_CLK_MON_CS: crate::RWRegister<u32>,
    #[doc = "Safety clock monitor threshold register"]
    pub SAFETY_CLK_MON_TH: crate::RWRegister<u32>,
    _reserved7: [u8; 0x08],
    #[doc = "GPIO_EMC_B1 bank IO control"]
    pub EMC_B1_IO_CTRL: crate::RWRegister<u32>,
    #[doc = "GPIO_EMC_B2 bank IO control"]
    pub EMC_B2_IO_CTRL: crate::RWRegister<u32>,
    #[doc = "GPIO_SD_B1 bank IO control"]
    pub SD_B1_IO_CTRL: crate::RWRegister<u32>,
    #[doc = "GPIO_SD_B2 bank IO control"]
    pub SD_B2_IO_CTRL: crate::RWRegister<u32>,
    #[doc = "GPIO_B1 bank IO control"]
    pub GPIO_B1_IO_CTRL: crate::RWRegister<u32>,
    #[doc = "GPIO_B2 bank IO control"]
    pub GPIO_B2_IO_CTRL: crate::RWRegister<u32>,
    #[doc = "Miscellaneous control register of IO"]
    pub MISC_IO_CTRL: crate::RWRegister<u32>,
}
#[doc = "IPG DEBUG mask bit"]
pub mod IPG_DEBUG1 {
    #[doc = "CAN2 debug halted mode with M7"]
    pub mod M33_CAN2 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "CAN2 does not enter debug halted mode with CM33"]
            pub const MASK: u32 = 0;
            #[doc = "CAN2 enters debug halted mode when CM33 is debug halted"]
            pub const UNMASK: u32 = 0x01;
        }
    }
    #[doc = "EDMA4 debug halted mode with M33"]
    pub mod M33_EDMA4 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "EDMA4 does not enter debug halted mode with CM33"]
            pub const MASK: u32 = 0;
            #[doc = "EDMA4 enters debug halted mode when CM33 is debug halted"]
            pub const UNMASK: u32 = 0x01;
        }
    }
    #[doc = "FLEXIO1 debug halted mode with M33"]
    pub mod M33_FLEXIO1 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "FLEXIO1 does not enter debug halted mode with CM33"]
            pub const MASK: u32 = 0;
            #[doc = "FLEXIO1 enters debug halted mode when CM33 is debug halted"]
            pub const UNMASK: u32 = 0x01;
        }
    }
    #[doc = "FLEXIO2 debug halted mode with M33"]
    pub mod M33_FLEXIO2 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "FLEXIO2 does not enter debug halted mode with CM33"]
            pub const MASK: u32 = 0;
            #[doc = "FLEXIO2 enters debug halted mode when CM33 is debug halted"]
            pub const UNMASK: u32 = 0x01;
        }
    }
    #[doc = "LPI2C3 debug halted mode with M33"]
    pub mod M33_LPI2C3 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "LPI2C3 does not enter debug halted mode with CM33"]
            pub const MASK: u32 = 0;
            #[doc = "enters debug halted mode when CM33 is debug halted"]
            pub const UNMASK: u32 = 0x01;
        }
    }
    #[doc = "LPI2C4 debug halted mode with M33"]
    pub mod M33_LPI2C4 {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "LPI2C4 does not enter debug halted mode with CM33"]
            pub const MASK: u32 = 0;
            #[doc = "LPI2C4 enters debug halted mode when CM33 is debug halted"]
            pub const UNMASK: u32 = 0x01;
        }
    }
    #[doc = "LPIT2 debug halted mode with M33"]
    pub mod M33_LPIT2 {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "LPIT2 does not enter debug halted mode with CM33"]
            pub const MASK: u32 = 0;
            #[doc = "LPIT2 enters debug halted mode when CM33 is debug halted"]
            pub const UNMASK: u32 = 0x01;
        }
    }
    #[doc = "LPSPI3 debug halted mode with M33"]
    pub mod M33_LPSPI3 {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "LPSPI3 does not enter debug halted mode with CM33"]
            pub const MASK: u32 = 0;
            #[doc = "LPSPI3 enters debug halted mode when CM33 is debug halted"]
            pub const UNMASK: u32 = 0x01;
        }
    }
    #[doc = "LPSPI4 debug halted mode with M33"]
    pub mod M33_LPSPI4 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "LPSPI4 does not enter debug halted mode with CM33"]
            pub const MASK: u32 = 0;
            #[doc = "LPSPI4 enters debug halted mode when CM33 is debug halted"]
            pub const UNMASK: u32 = 0x01;
        }
    }
    #[doc = "LPTMR2 debug halted mode with M33"]
    pub mod M33_LPTMR2 {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "LPTMR2 does not enter debug halted mode with CM33"]
            pub const MASK: u32 = 0;
            #[doc = "LPTMR2 enters debug halted mode when CM33 is debug halted"]
            pub const UNMASK: u32 = 0x01;
        }
    }
    #[doc = "debug halted mode with M33"]
    pub mod M33_TPM3 {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "TPM3 does not enter debug halted mode with CM33"]
            pub const MASK: u32 = 0;
            #[doc = "TPM3 enters debug halted mode when CM33 is debug halted"]
            pub const UNMASK: u32 = 0x01;
        }
    }
    #[doc = "TPM3 debug halted mode with M33"]
    pub mod M33_TPM4 {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "does not enter debug halted mode with CM33"]
            pub const MASK: u32 = 0;
            #[doc = "enters debug halted mode when CM33 is debug halted"]
            pub const UNMASK: u32 = 0x01;
        }
    }
    #[doc = "TPM5 debug halted mode with M33"]
    pub mod M33_TPM5 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "TPM5 does not enter debug halted mode with CM33"]
            pub const MASK: u32 = 0;
            #[doc = "TPM5 enters debug halted mode when CM33 is debug halted"]
            pub const UNMASK: u32 = 0x01;
        }
    }
    #[doc = "TPM6 debug halted mode with M33"]
    pub mod M33_TPM6 {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "TPM6 does not enter debug halted mode with CM33"]
            pub const MASK: u32 = 0;
            #[doc = "TPM6 enters debug halted mode when CM33 is debug halted"]
            pub const UNMASK: u32 = 0x01;
        }
    }
    #[doc = "WDOG3 debug halted mode with M33"]
    pub mod M33_WDOG3 {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "WDOG3 does not enter debug halted mode with CM33"]
            pub const MASK: u32 = 0;
            #[doc = "WDOG3 enters debug halted mode when CM33 is debug halted"]
            pub const UNMASK: u32 = 0x01;
        }
    }
    #[doc = "WDOG4 debug halted mode with M33"]
    pub mod M33_WDOG4 {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "WDOG4 does not enter debug halted mode with CM33"]
            pub const MASK: u32 = 0;
            #[doc = "WDOG4 enters debug halted mode when CM33 is debug halted"]
            pub const UNMASK: u32 = 0x01;
        }
    }
    #[doc = "CAN2 debug halted mode with M7"]
    pub mod M7_CAN2 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "CAN2 does not enter debug halted mode with CM7"]
            pub const MASK: u32 = 0;
            #[doc = "CAN2 enters debug halted mode when CM7 is debug halted"]
            pub const UNMASK: u32 = 0x01;
        }
    }
    #[doc = "EDMA4 debug halted mode with M7"]
    pub mod M7_EDMA4 {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "EDMA4 does not enter debug halted mode with CM7"]
            pub const MASK: u32 = 0;
            #[doc = "EDMA4 enters debug halted mode when CM7 is debug halted"]
            pub const UNMASK: u32 = 0x01;
        }
    }
    #[doc = "FLEXIO1 debug halted mode with M7"]
    pub mod M7_FLEXIO1 {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "FLEXIO1 does not enter debug halted mode with CM7"]
            pub const MASK: u32 = 0;
            #[doc = "FLEXIO1 enters debug halted mode when CM7 is debug halted"]
            pub const UNMASK: u32 = 0x01;
        }
    }
    #[doc = "FLEXIO2 debug halted mode with M7"]
    pub mod M7_FLEXIO2 {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "FLEXIO2 does not enter debug halted mode with CM7"]
            pub const MASK: u32 = 0;
            #[doc = "FLEXIO2 enters debug halted mode when CM7 is debug halted"]
            pub const UNMASK: u32 = 0x01;
        }
    }
    #[doc = "LPI2C3 debug halted mode with M7"]
    pub mod M7_LPI2C3 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "LPI2C3 does not enter debug halted mode with CM7"]
            pub const MASK: u32 = 0;
            #[doc = "LPI2C3 enters debug halted mode when CM7 is debug halted"]
            pub const UNMASK: u32 = 0x01;
        }
    }
    #[doc = "LPI2C4 debug halted mode with M7"]
    pub mod M7_LPI2C4 {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "LPI2C4 does not enter debug halted mode with CM7"]
            pub const MASK: u32 = 0;
            #[doc = "LPI2C4 enters debug halted mode when CM7 is debug halted"]
            pub const UNMASK: u32 = 0x01;
        }
    }
    #[doc = "LPIT2 debug halted mode with M7"]
    pub mod M7_LPIT2 {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "LPIT2 does not enter debug halted mode with CM7"]
            pub const MASK: u32 = 0;
            #[doc = "LPIT2 enters debug halted mode when CM7 is debug halted"]
            pub const UNMASK: u32 = 0x01;
        }
    }
    #[doc = "WDOG3 debug halted mode with M7"]
    pub mod M7_LPSPI3 {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "WDOD3 does not enter debug halted mode with CM7"]
            pub const MASK: u32 = 0;
            #[doc = "WDOG3 enters debug halted mode when CM7 is debug halted"]
            pub const UNMASK: u32 = 0x01;
        }
    }
    #[doc = "LPSPI4 debug halted mode with M7"]
    pub mod M7_LPSPI4 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "LPSPI4 does not enter debug halted mode with CM7"]
            pub const MASK: u32 = 0;
            #[doc = "LPSPI4 enters debug halted mode when CM7 is debug halted"]
            pub const UNMASK: u32 = 0x01;
        }
    }
    #[doc = "LPTMR2 debug halted mode with M7"]
    pub mod M7_LPTMR2 {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "LPTMR2 does not enter debug halted mode with CM7"]
            pub const MASK: u32 = 0;
            #[doc = "LPTMR2 enters debug halted mode when CM7 is debug halted"]
            pub const UNMASK: u32 = 0x01;
        }
    }
    #[doc = "TPM3 debug halted mode with M7"]
    pub mod M7_TPM3 {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "TPM3 does not enter debug halted mode with CM7"]
            pub const MASK: u32 = 0;
            #[doc = "TPM3 enters debug halted mode when CM7 is debug halted"]
            pub const UNMASK: u32 = 0x01;
        }
    }
    #[doc = "TPM4 debug halted mode with M7"]
    pub mod M7_TPM4 {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "TPM4 does not enter debug halted mode with CM7"]
            pub const MASK: u32 = 0;
            #[doc = "TPM4 enters debug halted mode when CM7 is debug halted"]
            pub const UNMASK: u32 = 0x01;
        }
    }
    #[doc = "TPM5 debug halted mode with M7"]
    pub mod M7_TPM5 {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "TPM5 does not enter debug halted mode with CM7"]
            pub const MASK: u32 = 0;
            #[doc = "TPM5 enters debug halted mode when CM7 is debug halted"]
            pub const UNMASK: u32 = 0x01;
        }
    }
    #[doc = "TPM6 debug halted mode with M7"]
    pub mod M7_TPM6 {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "TPM5 does not enter debug halted mode with CM7"]
            pub const MASK: u32 = 0;
            #[doc = "TPM5 enters debug halted mode when CM7 is debug halted"]
            pub const UNMASK: u32 = 0x01;
        }
    }
    #[doc = "WDOG3 debug halted mode with M7"]
    pub mod M7_WDOG3 {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "WDOG4 debug halted mode with M7"]
    pub mod M7_WDOG4 {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "IPG DEBUG mask bit"]
pub mod IPG_DEBUG2 {
    #[doc = "WDOG5 debug halted mode with M7"]
    pub mod M33_WDOG5 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "WDOG5 does not enter debug halted mode with CM33"]
            pub const MASK: u32 = 0;
            #[doc = "WDOG5 enters debug halted mode when CM33 is debug halted"]
            pub const UNMASK: u32 = 0x01;
        }
    }
    #[doc = "LPTMR3 debug halted mode with M33"]
    pub mod M33_LPTMR3 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "LPTMR3 does not enter debug halted mode with CM33"]
            pub const MASK: u32 = 0;
            #[doc = "LPTMR3 enters debug halted mode when CM33 is debug halted"]
            pub const UNMASK: u32 = 0x01;
        }
    }
    #[doc = "LPSPI5 debug halted mode with M33"]
    pub mod M33_LPSPI5 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "LPSPI5 does not enter debug halted mode with CM33"]
            pub const MASK: u32 = 0;
            #[doc = "LPSPI5 enters debug halted mode when CM33 is debug halted"]
            pub const UNMASK: u32 = 0x01;
        }
    }
    #[doc = "LPSPI6 debug halted mode with M33"]
    pub mod M33_LPSPI6 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "LPSPI6 does not enter debug halted mode with CM33"]
            pub const MASK: u32 = 0;
            #[doc = "LPSPI6 enters debug halted mode when CM33 is debug halted"]
            pub const UNMASK: u32 = 0x01;
        }
    }
    #[doc = "LPIT3 debug halted mode with M33"]
    pub mod M33_LPIT3 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "LPIT3 does not enter debug halted mode with CM33"]
            pub const MASK: u32 = 0;
            #[doc = "LPIT3 enters debug halted mode when CM33 is debug halted"]
            pub const UNMASK: u32 = 0x01;
        }
    }
    #[doc = "LPI2C5 debug halted mode with M33"]
    pub mod M33_LPI2C5 {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "LPI2C5 does not enter debug halted mode with CM33"]
            pub const MASK: u32 = 0;
            #[doc = "LPI2C5 enters debug halted mode when CM33 is debug halted"]
            pub const UNMASK: u32 = 0x01;
        }
    }
    #[doc = "LPI2C6 debug halted mode with M33"]
    pub mod M33_LPI2C6 {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "LPI2C6 does not enter debug halted mode with CM33"]
            pub const MASK: u32 = 0;
            #[doc = "LPI2C6 enters debug halted mode when CM33 is debug halted"]
            pub const UNMASK: u32 = 0x01;
        }
    }
    #[doc = "GPT2 debug halted mode with M33"]
    pub mod M33_GPT2 {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "GPT2 does not enter debug halted mode with CM33"]
            pub const MASK: u32 = 0;
            #[doc = "GPT2 enters debug halted mode when CM33 is debug halted"]
            pub const UNMASK: u32 = 0x01;
        }
    }
    #[doc = "FLEXPWM1 debug halted mode with M33"]
    pub mod M33_FLEXPWM1 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "FLEXPWM1 does not enter debug halted mode with CM33"]
            pub const MASK: u32 = 0;
            #[doc = "FLEXPWM1 enters debug halted mode when CM33 is debug halted"]
            pub const UNMASK: u32 = 0x01;
        }
    }
    #[doc = "FLEXPWM2 debug halted mode with M33"]
    pub mod M33_FLEXPWM2 {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "FLEXPWM2 does not enter debug halted mode with CM33"]
            pub const MASK: u32 = 0;
            #[doc = "FLEXPWM2 enters debug halted mode when CM33 is debug halted"]
            pub const UNMASK: u32 = 0x01;
        }
    }
    #[doc = "FLEXPWM3 debug halted mode with M33"]
    pub mod M33_FLEXPWM3 {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "FLEXPWM3 does not enter debug halted mode with CM33"]
            pub const MASK: u32 = 0;
            #[doc = "FLEXPWM3 enters debug halted mode when CM33 is debug halted"]
            pub const UNMASK: u32 = 0x01;
        }
    }
    #[doc = "FLEXPWM4 debug halted mode with M33"]
    pub mod M33_FLEXPWM4 {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "FLEXPWM4 does not enter debug halted mode with CM33"]
            pub const MASK: u32 = 0;
            #[doc = "FLEXPWM4 enters debug halted mode when CM33 is debug halted"]
            pub const UNMASK: u32 = 0x01;
        }
    }
    #[doc = "MIC debug halted mode with M33"]
    pub mod M33_MIC {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MIC does not enter debug halted mode with CM33"]
            pub const MASK: u32 = 0;
            #[doc = "MIC enters debug halted mode when CM33 is debug halted"]
            pub const UNMASK: u32 = 0x01;
        }
    }
    #[doc = "SAI2 debug halted mode with M33"]
    pub mod M33_SAI2 {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "SAI2 does not enter debug halted mode with CM33"]
            pub const MASK: u32 = 0;
            #[doc = "SAI2 enters debug halted mode when CM33 is debug halted"]
            pub const UNMASK: u32 = 0x01;
        }
    }
    #[doc = "SAI3 debug halted mode with M33"]
    pub mod M33_SAI3 {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "SAI3 does not enter debug halted mode with CM33"]
            pub const MASK: u32 = 0;
            #[doc = "SAI3 enters debug halted mode when CM33 is debug halted"]
            pub const UNMASK: u32 = 0x01;
        }
    }
    #[doc = "SAI4 debug halted mode with M33"]
    pub mod M33_SAI4 {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "SAI4 does not enter debug halted mode with CM33"]
            pub const MASK: u32 = 0;
            #[doc = "SAI4 enters debug halted mode when CM33 is debug halted"]
            pub const UNMASK: u32 = 0x01;
        }
    }
    #[doc = "WDOG5 debug halted mode with M7"]
    pub mod M7_WDOG5 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "WDOG5 does not enter debug halted mode with CM7"]
            pub const MASK: u32 = 0;
            #[doc = "WDOG5 enters debug halted mode when CM7 is debug halted"]
            pub const UNMASK: u32 = 0x01;
        }
    }
    #[doc = "LPTMR3 debug halted mode with M7"]
    pub mod M7_LPTMR3 {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "LPTMR3 does not enter debug halted mode with CM7"]
            pub const MASK: u32 = 0;
            #[doc = "LPTMR3 enters debug halted mode when CM7 is debug halted"]
            pub const UNMASK: u32 = 0x01;
        }
    }
    #[doc = "LPTMR3 debug halted mode with M7"]
    pub mod M7_LPSPI5 {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "LPTMR3 does not enter debug halted mode with CM7"]
            pub const MASK: u32 = 0;
            #[doc = "enters debug halted mode when CM7 is debug halted"]
            pub const UNMASK: u32 = 0x01;
        }
    }
    #[doc = "LPSPI6 debug halted mode with M7"]
    pub mod M7_LPSPI6 {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "LPSPI6 does not enter debug halted mode with CM7"]
            pub const MASK: u32 = 0;
            #[doc = "LPSPI6 enters debug halted mode when CM7 is debug halted"]
            pub const UNMASK: u32 = 0x01;
        }
    }
    #[doc = "LPIT3 debug halted mode with M7"]
    pub mod M7_LPIT3 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "LPIT3 does not enter debug halted mode with CM7"]
            pub const MASK: u32 = 0;
            #[doc = "LPIT3 enters debug halted mode when CM7 is debug halted"]
            pub const UNMASK: u32 = 0x01;
        }
    }
    #[doc = "LPI2C5 debug halted mode with M7"]
    pub mod M7_LPI2C5 {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "LPI2C5 does not enter debug halted mode with CM7"]
            pub const MASK: u32 = 0;
            #[doc = "LPI2C5 enters debug halted mode when CM7 is debug halted"]
            pub const UNMASK: u32 = 0x01;
        }
    }
    #[doc = "LPI2C6\" debug halted mode with M7"]
    pub mod M7_LPI2C6 {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "LPI2C6\" does not enter debug halted mode with CM7"]
            pub const MASK: u32 = 0;
            #[doc = "LPI2C6\" enters debug halted mode when CM7 is debug halted"]
            pub const UNMASK: u32 = 0x01;
        }
    }
    #[doc = "GPT2 debug halted mode with M7"]
    pub mod M7_GPT2 {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "GPT2 does not enter debug halted mode with CM7"]
            pub const MASK: u32 = 0;
            #[doc = "GPT2 enters debug halted mode when CM7 is debug halted"]
            pub const UNMASK: u32 = 0x01;
        }
    }
    #[doc = "FLEXPWM1 debug halted mode with M7"]
    pub mod M7_FLEXPWM1 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "FLEXPWM1 does not enter debug halted mode with CM7"]
            pub const MASK: u32 = 0;
            #[doc = "FLEXPWM1 enters debug halted mode when CM7 is debug halted"]
            pub const UNMASK: u32 = 0x01;
        }
    }
    #[doc = "FLEXPWM2 debug halted mode with M7"]
    pub mod M7_FLEXPWM2 {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "FLEXPWM2 does not enter debug halted mode with CM7"]
            pub const MASK: u32 = 0;
            #[doc = "FLEXPWM2 enters debug halted mode when CM7 is debug halted"]
            pub const UNMASK: u32 = 0x01;
        }
    }
    #[doc = "FLEXPWM3 debug halted mode with M7"]
    pub mod M7_FLEXPWM3 {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "FLEXPWM3 does not enter debug halted mode with CM7"]
            pub const MASK: u32 = 0;
            #[doc = "FLEXPWM3 enters debug halted mode when CM7 is debug halted"]
            pub const UNMASK: u32 = 0x01;
        }
    }
    #[doc = "FLEXPWM4 debug halted mode with M7"]
    pub mod M7_FLEXPWM4 {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "FLEXPWM4 does not enter debug halted mode with CM7"]
            pub const MASK: u32 = 0;
            #[doc = "FLEXPWM4 enters debug halted mode when CM7 is debug halted"]
            pub const UNMASK: u32 = 0x01;
        }
    }
    #[doc = "MIC debug halted mode with M7"]
    pub mod M7_MIC {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MIC does not enter debug halted mode with CM7"]
            pub const MASK: u32 = 0;
            #[doc = "MIC enters debug halted mode when CM7 is debug halted"]
            pub const UNMASK: u32 = 0x01;
        }
    }
    #[doc = "SAI2 debug halted mode with M7"]
    pub mod M7_SAI2 {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "SAI2 does not enter debug halted mode with CM7"]
            pub const MASK: u32 = 0;
            #[doc = "SAI2 enters debug halted mode when CM7 is debug halted"]
            pub const UNMASK: u32 = 0x01;
        }
    }
    #[doc = "SAI3 debug halted mode with M7"]
    pub mod M7_SAI3 {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "SAI3 does not enter debug halted mode with CM7"]
            pub const MASK: u32 = 0;
            #[doc = "SAI3 enters debug halted mode when CM7 is debug halted"]
            pub const UNMASK: u32 = 0x01;
        }
    }
    #[doc = "SAI4 debug halted mode with M7"]
    pub mod M7_SAI4 {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "SAI4 does not enter debug halted mode with CM7"]
            pub const MASK: u32 = 0;
            #[doc = "SAI4 enters debug halted mode when CM7 is debug halted"]
            pub const UNMASK: u32 = 0x01;
        }
    }
}
#[doc = "IPG DEBUG mask bit"]
pub mod IPG_DEBUG3 {
    #[doc = "I3C2 debug halted mode with M33"]
    pub mod M33_SINC1 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "I3C2 does not enter debug halted mode with CM33"]
            pub const MASK: u32 = 0;
            #[doc = "I3C2 enters debug halted mode when CM33 is debug halted"]
            pub const UNMASK: u32 = 0x01;
        }
    }
    #[doc = "SINC2 debug halted mode with M33"]
    pub mod M33_SINC2 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "SINC2 does not enter debug halted mode with CM33"]
            pub const MASK: u32 = 0;
            #[doc = "SINC2 enters debug halted mode when CM33 is debug halted"]
            pub const UNMASK: u32 = 0x01;
        }
    }
    #[doc = "SINC3 debug halted mode with M33"]
    pub mod M33_SINC3 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "SINC3 does not enter debug halted mode with CM33"]
            pub const MASK: u32 = 0;
            #[doc = "SINC3 enters debug halted mode when CM33 is debug halted"]
            pub const UNMASK: u32 = 0x01;
        }
    }
    #[doc = "QTIMER1 debug halted mode with M33"]
    pub mod M33_QTIMER1 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "QTIMER1 does not enter debug halted mode with CM33"]
            pub const MASK: u32 = 0;
            #[doc = "QTIMER1 enters debug halted mode when CM33 is debug halted"]
            pub const UNMASK: u32 = 0x01;
        }
    }
    #[doc = "QTIMER2 debug halted mode with M33"]
    pub mod M33_QTIMER2 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "QTIMER2 does not enter debug halted mode with CM33"]
            pub const MASK: u32 = 0;
            #[doc = "QTIMER2 enters debug halted mode when CM33 is debug halted"]
            pub const UNMASK: u32 = 0x01;
        }
    }
    #[doc = "QTIMER3 debug halted mode with M33"]
    pub mod M33_QTIMER3 {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "QTIMER3 does not enter debug halted mode with CM33"]
            pub const MASK: u32 = 0;
            #[doc = "QTIMER3 enters debug halted mode when CM33 is debug halted"]
            pub const UNMASK: u32 = 0x01;
        }
    }
    #[doc = "QTIMER4 debug halted mode with M33"]
    pub mod M33_QTIMER4 {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "QTIMER4 does not enter debug halted mode with CM33"]
            pub const MASK: u32 = 0;
            #[doc = "QTIMER4 enters debug halted mode when CM33 is debug halted"]
            pub const UNMASK: u32 = 0x01;
        }
    }
    #[doc = "QTIMER5 debug halted mode with M33"]
    pub mod M33_QTIMER5 {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "QTIMER5 does not enter debug halted mode with CM33"]
            pub const MASK: u32 = 0;
            #[doc = "QTIMER5 enters debug halted mode when CM33 is debug halted"]
            pub const UNMASK: u32 = 0x01;
        }
    }
    #[doc = "QTIMER6 debug halted mode with M33"]
    pub mod M33_QTIMER6 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "QTIMER6 does not enter debug halted mode with CM33"]
            pub const MASK: u32 = 0;
            #[doc = "QTIMER6 enters debug halted mode when CM33 is debug halted"]
            pub const UNMASK: u32 = 0x01;
        }
    }
    #[doc = "QTIMER7 debug halted mode with M33"]
    pub mod M33_QTIMER7 {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "QTIMER7 does not enter debug halted mode with CM33"]
            pub const MASK: u32 = 0;
            #[doc = "QTIMER7 enters debug halted mode when CM33 is debug halted"]
            pub const UNMASK: u32 = 0x01;
        }
    }
    #[doc = "QTIMER8 debug halted mode with M33"]
    pub mod M33_QTIMER8 {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "QTIMER8 does not enter debug halted mode with CM33"]
            pub const MASK: u32 = 0;
            #[doc = "QTIMER8 enters debug halted mode when CM33 is debug halted"]
            pub const UNMASK: u32 = 0x01;
        }
    }
    #[doc = "I3C2 debug halted mode with M33"]
    pub mod M33_I3C2 {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "I3C2 does not enter debug halted mode with CM33"]
            pub const MASK: u32 = 0;
            #[doc = "I3C2 enters debug halted mode when CM33 is debug halted"]
            pub const UNMASK: u32 = 0x01;
        }
    }
    #[doc = "SINC1 debug halted mode with M7"]
    pub mod M7_SINC1 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "SINC1 does not enter debug halted mode with CM7"]
            pub const MASK: u32 = 0;
            #[doc = "SINC1 enters debug halted mode when CM7 is debug halted"]
            pub const UNMASK: u32 = 0x01;
        }
    }
    #[doc = "SINC2 debug halted mode with M7"]
    pub mod M7_SINC2 {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "SINC2 does not enter debug halted mode with CM7"]
            pub const MASK: u32 = 0;
            #[doc = "SINC2 enters debug halted mode when CM7 is debug halted"]
            pub const UNMASK: u32 = 0x01;
        }
    }
    #[doc = "SINC3 debug halted mode with M7"]
    pub mod M7_SINC3 {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "SINC3 does not enter debug halted mode with CM7"]
            pub const MASK: u32 = 0;
            #[doc = "SINC3 enters debug halted mode when CM7 is debug halted"]
            pub const UNMASK: u32 = 0x01;
        }
    }
    #[doc = "QTIMER1 debug halted mode with M7"]
    pub mod M7_QTIMER1 {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "QTIMER1 does not enter debug halted mode with CM7"]
            pub const MASK: u32 = 0;
            #[doc = "QTIMER1 enters debug halted mode when CM7 is debug halted"]
            pub const UNMASK: u32 = 0x01;
        }
    }
    #[doc = "QTIMER2 debug halted mode with M7"]
    pub mod M7_QTIMER2 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "QTIMER2 does not enter debug halted mode with CM7"]
            pub const MASK: u32 = 0;
            #[doc = "QTIMER2 enters debug halted mode when CM7 is debug halted"]
            pub const UNMASK: u32 = 0x01;
        }
    }
    #[doc = "QTIMER3 debug halted mode with M7"]
    pub mod M7_QTIMER3 {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "QTIMER3 does not enter debug halted mode with CM7"]
            pub const MASK: u32 = 0;
            #[doc = "QTIMER3 enters debug halted mode when CM7 is debug halted"]
            pub const UNMASK: u32 = 0x01;
        }
    }
    #[doc = "QTIMER4 debug halted mode with M7"]
    pub mod M7_QTIMER4 {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "QTIMER4 does not enter debug halted mode with CM7"]
            pub const MASK: u32 = 0;
            #[doc = "QTIMER4 enters debug halted mode when CM7 is debug halted"]
            pub const UNMASK: u32 = 0x01;
        }
    }
    #[doc = "QTIMER5 debug halted mode with M7"]
    pub mod M7_QTIMER5 {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "QTIMER5 does not enter debug halted mode with CM7"]
            pub const MASK: u32 = 0;
            #[doc = "QTIMER5 enters debug halted mode when CM7 is debug halted"]
            pub const UNMASK: u32 = 0x01;
        }
    }
    #[doc = "debug halted mode with M7"]
    pub mod M7_QTIMER6 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "does not enter debug halted mode with CM7"]
            pub const MASK: u32 = 0;
            #[doc = "enters debug halted mode when CM7 is debug halted"]
            pub const UNMASK: u32 = 0x01;
        }
    }
    #[doc = "QTIMER7 debug halted mode with M7"]
    pub mod M7_QTIMER7 {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "QTIMER7 does not enter debug halted mode with CM7"]
            pub const MASK: u32 = 0;
            #[doc = "QTIMER7 enters debug halted mode when CM7 is debug halted"]
            pub const UNMASK: u32 = 0x01;
        }
    }
    #[doc = "QTIMER8 debug halted mode with M7"]
    pub mod M7_QTIMER8 {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "QTIMER8 does not enter debug halted mode with CM7"]
            pub const MASK: u32 = 0;
            #[doc = "QTIMER8 enters debug halted mode when CM7 is debug halted"]
            pub const UNMASK: u32 = 0x01;
        }
    }
    #[doc = "debug halted mode with M7"]
    pub mod M7_I3C2 {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "I3C2 does not enter debug halted mode with CM7"]
            pub const MASK: u32 = 0;
            #[doc = "I3C2 enters debug halted mode when CM7 is debug halted"]
            pub const UNMASK: u32 = 0x01;
        }
    }
}
#[doc = "SSI master low power mode control"]
pub mod SSI {
    #[doc = "WAKEUP Domain to M7 SSI master idle"]
    pub mod SSI_IDLE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "WAKEUP Domain to M7 SSI master is not idle"]
            pub const NOT_IDLE: u32 = 0;
            #[doc = "WAKEUP Domain to M7 SSI master is idle"]
            pub const IDLE: u32 = 0x01;
        }
    }
    #[doc = "WAKEUP Domain to M7 SSI master blackhole mode"]
    pub mod BLKHOLE_MODE_B {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "WAKEUP Domain to M7 SSI master will exit blackhole mode"]
            pub const EXIT: u32 = 0;
            #[doc = "WAKEUP Domain to M7 SSI master will enter blackhole mode"]
            pub const ENTER: u32 = 0x01;
        }
    }
    #[doc = "WAKEUP Domain to M7 SSI master pause mode"]
    pub mod PAUSE_MODE {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "WAKEUP Domain to M7 SSI master will enter pause mode"]
            pub const EXIT: u32 = 0;
            #[doc = "WAKEUP Domain to M7 SSI master will exit pause mode"]
            pub const ENTER: u32 = 0x01;
        }
    }
}
#[doc = "EtherCAT miscellaneous configuration"]
pub mod ECAT_MISC_CFG {
    #[doc = "RMII mode selection for EtherCAT port 0"]
    pub mod RMII_SEL0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "EtherCAT port0 is in MII mode"]
            pub const MII: u32 = 0;
            #[doc = "EtherCAT port0 is in RMII mode"]
            pub const RMII: u32 = 0x01;
        }
    }
    #[doc = "RMII mode selection for EtherCAT port 1"]
    pub mod RMII_SEL1 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "EtherCAT port1 is in MII mode"]
            pub const MII: u32 = 0;
            #[doc = "EtherCAT port1 is in RMII mode"]
            pub const RMII: u32 = 0x01;
        }
    }
    #[doc = "Global enable of EtherCAT"]
    pub mod GLB_EN {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "EtherCAT is off"]
            pub const OFF: u32 = 0;
            #[doc = "EtherCAT is on"]
            pub const ON: u32 = 0x01;
        }
    }
    #[doc = "Global reset of EtherCAT"]
    pub mod GLB_RST {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "EtherCAT is out of reset"]
            pub const OUT_RESET: u32 = 0;
            #[doc = "EtherCAT is held in reset"]
            pub const IN_RESET: u32 = 0x01;
        }
    }
    #[doc = "RMII Port0 REF_CLK direction control"]
    pub mod RMII_REF_CLK_DIR0 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "RMII REF_CLK is input"]
            pub const INPUT: u32 = 0;
            #[doc = "RMII REF_CLK is output driven by ECAT_CLK_ROOT/2"]
            pub const OUTPUT: u32 = 0x01;
        }
    }
    #[doc = "RMII Port1 REF_CLK direction control"]
    pub mod RMII_REF_CLK_DIR1 {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "RMII REF_CLK is input"]
            pub const INPUT: u32 = 0;
            #[doc = "RMII REF_CLK is output driven by ECAT_CLK_ROOT/2"]
            pub const OUTPUT: u32 = 0x01;
        }
    }
    #[doc = "EtherCAT PHY_OFFSET"]
    pub mod PHY_OFFSET {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "EtherCAT PHY_OFFSET_VEC"]
    pub mod PHY_OFFSET_VEC {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "EtherCAT EEPROM SIZE OPTION"]
    pub mod EEPROM_SIZE_OPTION {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DEXSC error response configuration"]
pub mod DEXSC_ERR {
    #[doc = "Exclusive error response enable"]
    pub mod EXC_ERR_RESP_EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "OKAY response"]
            pub const OKAY_RESPONSE: u32 = 0;
            #[doc = "SLVError response"]
            pub const SLVERROR_RESPONSE: u32 = 0x01;
        }
    }
    #[doc = "Lock bit of EXC_ERR_RESP_EN"]
    pub mod LOCK_EXC_ERR_RESP_EN {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "USBPHY miscellaneous control"]
pub mod USBPHY_MISC_CTRL {
    #[doc = "USBPHY1 register access clock enable"]
    pub mod USBPHY1_IPG_CLK_ACTIVE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "USBPHY2 register access clock enable"]
    pub mod USBPHY2_IPG_CLK_ACTIVE {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Clear USBPHY1 wakeup interrupt holding register"]
    pub mod USBPHY1_WAKEUP_IRQ_CLEAR {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Clear USBPHY2 wakeup interrupt holding register"]
    pub mod USBPHY2_WAKEUP_IRQ_CLEAR {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "NETC Port miscellaneous configuration"]
pub mod NETC_PORT_MISC_CFG {
    #[doc = "Port0 RMII Reference clock direction control"]
    pub mod PORT0_RMII_REF_CLK_DIR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Port1 RMII Reference clock direction control"]
    pub mod PORT1_RMII_REF_CLK_DIR {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Port1 RMII Reference clock is input"]
            pub const INPUT: u32 = 0;
            #[doc = "Port1 RMII Reference clock is output"]
            pub const OUTPUT: u32 = 0x01;
        }
    }
    #[doc = "Port2 RMII Reference clock direction control"]
    pub mod PORT2_RMII_REF_CLK_DIR {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Port2 RMII Reference clock is input"]
            pub const INPUT: u32 = 0;
            #[doc = "Port2 RMII Reference clock is output"]
            pub const OUTPUT: u32 = 0x01;
        }
    }
    #[doc = "Port3 RMII Reference clock direction control"]
    pub mod PORT3_RMII_REF_CLK_DIR {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Port3 RMII Reference clock is input"]
            pub const INPUT: u32 = 0;
            #[doc = "Port3 RMII Reference clock is output"]
            pub const OUTPUT: u32 = 0x01;
        }
    }
    #[doc = "Port4 RMII Reference clock direction control"]
    pub mod PORT4_RMII_REF_CLK_DIR {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Port4 RMII Reference clock is input"]
            pub const INPUT: u32 = 0;
            #[doc = "Port4 RMII Reference clock is output"]
            pub const OUTPUT: u32 = 0x01;
        }
    }
    #[doc = "Default value for IERB NETCRR\\[LOCK\\] bit. Determines write accessibility of IERB registers after power-on-reset"]
    pub mod CFG_IERB_LOCK {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Unlocked after power-on-reset. Normal read/write access to all IERB registers"]
            pub const UNLOCK: u32 = 0;
            #[doc = "Locked after power-on-reset. Write access inhibited to all IERB registers, except NETCRR"]
            pub const LOCK: u32 = 0x01;
        }
    }
    #[doc = "1588 timer external clock selection"]
    pub mod TMR_EXT_CLK_SEL {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "CCM tmr_1588_clk_root is selected"]
            pub const CCM_TIMER: u32 = 0;
            #[doc = "External pin is selected"]
            pub const EXT_PIN: u32 = 0x01;
        }
    }
    #[doc = "1588 timer trigger1 input selection"]
    pub mod TMR_TRIG1_SEL {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Input from IOMUX"]
            pub const IOMUX: u32 = 0;
            #[doc = "Input from XBAR"]
            pub const XBAR: u32 = 0x01;
        }
    }
    #[doc = "1588 timer trigger2 input selection"]
    pub mod TMR_TRIG2_SEL {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Input from IOMUX"]
            pub const IOMUX: u32 = 0;
            #[doc = "Input from XBAR"]
            pub const XBAR: u32 = 0x01;
        }
    }
}
#[doc = "M7 NMI interrupt clear register"]
pub mod M7_NMI_CLR {
    #[doc = "Clear CM7 NMI holding register"]
    pub mod M7_NMI_CLEAR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Qtimer miscellaneous control register 1"]
pub mod QTIMER_CTRL1 {
    #[doc = "QTIMER1 timer counter freeze"]
    pub mod QTIMER1_TMR_CNTS_FREEZE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Timer counter works normally"]
            pub const NORMAL: u32 = 0;
            #[doc = "Reset counter and output flags"]
            pub const FLAGS: u32 = 0x01;
        }
    }
    #[doc = "QTIMER1 TMR0 input select"]
    pub mod QTIMER1_TMR0_INPUT_SEL {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Input from IOMUX"]
            pub const IOMUX: u32 = 0;
            #[doc = "Input from XBAR"]
            pub const XBAR: u32 = 0x01;
        }
    }
    #[doc = "QTIMER1 TMR1 input select"]
    pub mod QTIMER1_TMR1_INPUT_SEL {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Input from IOMUX"]
            pub const IOMUX: u32 = 0;
            #[doc = "Input from XBAR"]
            pub const XBAR: u32 = 0x01;
        }
    }
    #[doc = "QTIMER1 TMR2 input select"]
    pub mod QTIMER1_TMR2_INPUT_SEL {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Input from IOMUX"]
            pub const IOMUX: u32 = 0;
            #[doc = "Input from XBAR"]
            pub const XBAR: u32 = 0x01;
        }
    }
    #[doc = "QTIMER1 TMR3 input select"]
    pub mod QTIMER1_TMR3_INPUT_SEL {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Input from IOMUX"]
            pub const IOMUX: u32 = 0;
            #[doc = "Input from XBAR"]
            pub const XBAR: u32 = 0x01;
        }
    }
    #[doc = "QTIMER2 timer counter freeze"]
    pub mod QTIMER2_TMR_CNTS_FREEZE {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Timer counter works normally"]
            pub const NORMAL: u32 = 0;
            #[doc = "Reset counter and output flags"]
            pub const FLAGS: u32 = 0x01;
        }
    }
    #[doc = "QTIMER2 TMR0 input select"]
    pub mod QTIMER2_TMR0_INPUT_SEL {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Input from IOMUX"]
            pub const IOMUX: u32 = 0;
            #[doc = "Input from XBAR"]
            pub const XBAR: u32 = 0x01;
        }
    }
    #[doc = "QTIMER2 TMR1 input select"]
    pub mod QTIMER2_TMR1_INPUT_SEL {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Input from IOMUX"]
            pub const IOMUX: u32 = 0;
            #[doc = "Input from XBAR"]
            pub const XBAR: u32 = 0x01;
        }
    }
    #[doc = "QTIMER2 TMR2 input select"]
    pub mod QTIMER2_TMR2_INPUT_SEL {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Input from IOMUX"]
            pub const IOMUX: u32 = 0;
            #[doc = "Input from XBAR"]
            pub const XBAR: u32 = 0x01;
        }
    }
    #[doc = "QTIMER2 TMR3 input select"]
    pub mod QTIMER2_TMR3_INPUT_SEL {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Input from IOMUX"]
            pub const IOMUX: u32 = 0;
            #[doc = "Input from XBAR"]
            pub const XBAR: u32 = 0x01;
        }
    }
    #[doc = "QTIMER3 timer counter freeze"]
    pub mod QTIMER3_TMR_CNTS_FREEZE {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Timer counter works normally"]
            pub const NORMAL: u32 = 0;
            #[doc = "Reset counter and ouput flags"]
            pub const FLAGS: u32 = 0x01;
        }
    }
    #[doc = "QTIMER3 TMR0 input select"]
    pub mod QTIMER3_TMR0_INPUT_SEL {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Input from IOMUX"]
            pub const IOMUX: u32 = 0;
            #[doc = "Input from XBAR"]
            pub const XBAR: u32 = 0x01;
        }
    }
    #[doc = "QTIMER3 TMR1 input select"]
    pub mod QTIMER3_TMR1_INPUT_SEL {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Input from IOMUX"]
            pub const IOMUX: u32 = 0;
            #[doc = "Input from XBAR"]
            pub const XBAR: u32 = 0x01;
        }
    }
    #[doc = "QTIMER3 TMR2 input select"]
    pub mod QTIMER3_TMR2_INPUT_SEL {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Input from IOMUX"]
            pub const IOMUX: u32 = 0;
            #[doc = "Input from XBAR"]
            pub const XBAR: u32 = 0x01;
        }
    }
    #[doc = "QTIMER3 TMR3 input select"]
    pub mod QTIMER3_TMR3_INPUT_SEL {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Input from IOMUX"]
            pub const IOMUX: u32 = 0;
            #[doc = "Input from XBAR"]
            pub const XBAR: u32 = 0x01;
        }
    }
    #[doc = "QTIMER4 timer counter freeze"]
    pub mod QTIMER4_TMR_CNTS_FREEZE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Timer counter works normally"]
            pub const NORMAL: u32 = 0;
            #[doc = "Reset counter and output flags"]
            pub const FLAGS: u32 = 0x01;
        }
    }
    #[doc = "QTIMER4 TMR0 input select"]
    pub mod QTIMER4_TMR0_INPUT_SEL {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Input from IOMUX"]
            pub const IOMUX: u32 = 0;
            #[doc = "Input from XBAR"]
            pub const XBAR: u32 = 0x01;
        }
    }
    #[doc = "QTIMER4 TMR1 input select"]
    pub mod QTIMER4_TMR1_INPUT_SEL {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Input from IOMUX"]
            pub const IOMUX: u32 = 0;
            #[doc = "Input from XBAR"]
            pub const XBAR: u32 = 0x01;
        }
    }
    #[doc = "QTIMER4 TMR2 input select"]
    pub mod QTIMER4_TMR2_INPUT_SEL {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Input from IOMUX"]
            pub const IOMUX: u32 = 0;
            #[doc = "Input from XBAR"]
            pub const XBAR: u32 = 0x01;
        }
    }
    #[doc = "QTIMER4 TMR3 input select"]
    pub mod QTIMER4_TMR3_INPUT_SEL {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Input from IOMUX"]
            pub const IOMUX: u32 = 0;
            #[doc = "Input from XBAR"]
            pub const XBAR: u32 = 0x01;
        }
    }
}
#[doc = "Qtimer miscellaneous control register 2"]
pub mod QTIMER_CTRL2 {
    #[doc = "QTIMER5 timer counter freeze"]
    pub mod QTIMER5_TMR_CNTS_FREEZE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Timer counter works normally"]
            pub const NORMAL_COUNTER: u32 = 0;
            #[doc = "Reset counter and output flags"]
            pub const RESET_COUNTER: u32 = 0x01;
        }
    }
    #[doc = "QTIMER5 TMR0 input select"]
    pub mod QTIMER5_TMR0_INPUT_SEL {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Input from IOMUX"]
            pub const IOMUX: u32 = 0;
            #[doc = "Input from XBAR"]
            pub const XBAR: u32 = 0x01;
        }
    }
    #[doc = "QTIMER5 TMR1 input select"]
    pub mod QTIMER5_TMR1_INPUT_SEL {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Input from IOMUX"]
            pub const IOMUX: u32 = 0;
            #[doc = "Input from XBAR"]
            pub const XBAR: u32 = 0x01;
        }
    }
    #[doc = "QTIMER5 TMR2 input select"]
    pub mod QTIMER5_TMR2_INPUT_SEL {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Input from IOMUX"]
            pub const IOMUX: u32 = 0;
            #[doc = "Input from XBAR"]
            pub const XBAR: u32 = 0x01;
        }
    }
    #[doc = "QTIMER5 TMR3 input select"]
    pub mod QTIMER5_TMR3_INPUT_SEL {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Input from IOMUX"]
            pub const IOMUX: u32 = 0;
            #[doc = "Input from XBAR"]
            pub const XBAR: u32 = 0x01;
        }
    }
    #[doc = "QTIMER6 timer counter freeze"]
    pub mod QTIMER6_TMR_CNTS_FREEZE {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "QTIMER6 TMR0 input select"]
    pub mod QTIMER6_TMR0_INPUT_SEL {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Input from IOMUX"]
            pub const IOMUX: u32 = 0;
            #[doc = "Input from XBAR"]
            pub const XBAR: u32 = 0x01;
        }
    }
    #[doc = "QTIMER6 TMR1 input select"]
    pub mod QTIMER6_TMR1_INPUT_SEL {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Input from IOMUX"]
            pub const IOMUX: u32 = 0;
            #[doc = "Input from XBAR"]
            pub const XBAR: u32 = 0x01;
        }
    }
    #[doc = "QTIMER6 TMR2 input select"]
    pub mod QTIMER6_TMR2_INPUT_SEL {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Input from IOMUX"]
            pub const IOMUX: u32 = 0;
            #[doc = "Input from XBAR"]
            pub const XBAR: u32 = 0x01;
        }
    }
    #[doc = "QTIMER6 TMR3 input select"]
    pub mod QTIMER6_TMR3_INPUT_SEL {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Input from IOMUX"]
            pub const IOMUX: u32 = 0;
            #[doc = "Input from XBAR"]
            pub const XBAR: u32 = 0x01;
        }
    }
    #[doc = "QTIMER7 timer counter freeze"]
    pub mod QTIMER7_TMR_CNTS_FREEZE {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Timer counter works normally"]
            pub const NORMAL_COUNTER: u32 = 0;
            #[doc = "Reset counter and output flags"]
            pub const RESET_COUNTER: u32 = 0x01;
        }
    }
    #[doc = "QTIMER7 TMR0 input select"]
    pub mod QTIMER7_TMR0_INPUT_SEL {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Input from IOMUX"]
            pub const IOMUX: u32 = 0;
            #[doc = "Input from XBAR"]
            pub const XBAR: u32 = 0x01;
        }
    }
    #[doc = "QTIMER7 TMR1 input select"]
    pub mod QTIMER7_TMR1_INPUT_SEL {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Input from IOMUX"]
            pub const IOMUX: u32 = 0;
            #[doc = "Input from XBAR"]
            pub const XBAR: u32 = 0x01;
        }
    }
    #[doc = "QTIMER7 TMR2 input select"]
    pub mod QTIMER7_TMR2_INPUT_SEL {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Input from IOMUX"]
            pub const IOMUX: u32 = 0;
            #[doc = "Input from XBAR"]
            pub const XBAR: u32 = 0x01;
        }
    }
    #[doc = "QTIMER7 TMR3 input select"]
    pub mod QTIMER7_TMR3_INPUT_SEL {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Input from IOMUX"]
            pub const IOMUX: u32 = 0;
            #[doc = "Input from XBAR"]
            pub const XBAR: u32 = 0x01;
        }
    }
    #[doc = "QTIMER8 timer counter freeze"]
    pub mod QTIMER8_TMR_CNTS_FREEZE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Timer counter works normally"]
            pub const NORMAL: u32 = 0;
            #[doc = "Reset counter and output flags"]
            pub const FLAGS: u32 = 0x01;
        }
    }
    #[doc = "QTIMER8 TMR0 input select"]
    pub mod QTIMER8_TMR0_INPUT_SEL {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Input from IOMUX"]
            pub const IOMUX: u32 = 0;
            #[doc = "Input from XBAR"]
            pub const XBAR: u32 = 0x01;
        }
    }
    #[doc = "QTIMER8 TMR1 input select"]
    pub mod QTIMER8_TMR1_INPUT_SEL {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Input from IOMUX"]
            pub const IOMUX: u32 = 0;
            #[doc = "Input from XBAR"]
            pub const XBAR: u32 = 0x01;
        }
    }
    #[doc = "QTIMER8 TMR2 input select"]
    pub mod QTIMER8_TMR2_INPUT_SEL {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Input from IOMUX"]
            pub const IOMUX: u32 = 0;
            #[doc = "Input from XBAR"]
            pub const XBAR: u32 = 0x01;
        }
    }
    #[doc = "QTIMER8 TMR3 input select"]
    pub mod QTIMER8_TMR3_INPUT_SEL {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Input from IOMUX"]
            pub const IOMUX: u32 = 0;
            #[doc = "Input from XBAR"]
            pub const XBAR: u32 = 0x01;
        }
    }
}
#[doc = "SAI2 MCLK control register"]
pub mod SAI2_MCLK_CTRL {
    #[doc = "SAI2 MCLK3 source select"]
    pub mod SAI2_MCLK3_SEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "SPDIF_CLK_ROOT"]
            pub const CLK_ROOT: u32 = 0;
            #[doc = "spdif_tx_clk2"]
            pub const TX_CLK2: u32 = 0x01;
            #[doc = "spdif_srclk"]
            pub const SRCLK: u32 = 0x02;
            #[doc = "spdif_outclock"]
            pub const OUTCLK: u32 = 0x03;
        }
    }
    #[doc = "SAI2_MCLK IO direction control. IOMUX need select SAI2 MCLK function."]
    pub mod SAI2_MCLK_DIR {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "SAI2_MCLK is input signal"]
            pub const INPUT: u32 = 0;
            #[doc = "SAI2_MCLK is output signal"]
            pub const OUTPUT: u32 = 0x01;
        }
    }
}
#[doc = "SAI3 MCLK control register"]
pub mod SAI3_MCLK_CTRL {
    #[doc = "SAI3 MCLK3 source select"]
    pub mod SAI3_MCLK3_SEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "SPDIF_CLK_ROOT"]
            pub const CLK_ROOT: u32 = 0;
            #[doc = "spdif_tx_clk2"]
            pub const TX_CLK2: u32 = 0x01;
            #[doc = "spdif_srclk"]
            pub const SRCLK: u32 = 0x02;
            #[doc = "spdif_outclock"]
            pub const OUTCLK: u32 = 0x03;
        }
    }
    #[doc = "SAI3_MCLK IO direction control. IOMUX need select SAI3 MCLK function."]
    pub mod SAI3_MCLK_DIR {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "SAI3_MCLK is input signal"]
            pub const MCLK_INPUT: u32 = 0;
            #[doc = "SAI3_MCLK is output signal"]
            pub const MCLK_OUTPUT: u32 = 0x01;
        }
    }
}
#[doc = "SAI4 MCLK control register"]
pub mod SAI4_MCLK_CTRL {
    #[doc = "SAI4 MCLK1 source select"]
    pub mod SAI4_MCLK1_SEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "SAI4_CLK_ROOT"]
            pub const SAI4_CLK_ROOT: u32 = 0;
            #[doc = "SAI2_CLK_ROOT"]
            pub const SAI2_CLK_ROOT: u32 = 0x01;
            #[doc = "SAI3_CLK_ROOT"]
            pub const SAI3_CLK_ROOT: u32 = 0x03;
            #[doc = "SAI4 MCLK IO pin"]
            pub const SAI4_MCLK_IO: u32 = 0x04;
            #[doc = "SAI2 MCLK IO pin"]
            pub const SAI2_MCLK_IO: u32 = 0x05;
            #[doc = "SAI3 MCLK IO pin"]
            pub const SAI3_MCLK_IO: u32 = 0x06;
        }
    }
    #[doc = "SAI4 MCLK2 source select"]
    pub mod SAI4_MCLK2_SEL {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "SAI4_CLK_ROOT"]
            pub const SAI4_CLK_ROOT: u32 = 0;
            #[doc = "SAI2_CLK_ROOT"]
            pub const SAI2_CLK_ROOT: u32 = 0x01;
            #[doc = "SAI3_CLK_ROOT"]
            pub const SAI3_CLK_ROOT: u32 = 0x03;
            #[doc = "SAI4 MCLK IO pin"]
            pub const SAI4_MCLK_IO: u32 = 0x04;
            #[doc = "SAI2 MCLK IO pin"]
            pub const SAI2_MCLK_IO: u32 = 0x05;
            #[doc = "SAI3 MCLK IO pin"]
            pub const SAI3_MCLK_IO: u32 = 0x06;
        }
    }
    #[doc = "SAI4 MCLK3 source select"]
    pub mod SAI4_MCLK3_SEL {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "SPDIF_CLK_ROOT"]
            pub const CLK_ROOT: u32 = 0;
            #[doc = "spdif_tx_clk2"]
            pub const TX_CLK2: u32 = 0x01;
            #[doc = "spdif_srclk"]
            pub const SRCLK: u32 = 0x02;
            #[doc = "spdif_outclock"]
            pub const OUTCLK: u32 = 0x03;
        }
    }
    #[doc = "SAI4_MCLK IO direction control. IOMUX need select SAI4 MCLK function."]
    pub mod SAI4_MCLK_DIR {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "XBAR IO direction control register"]
pub mod XBAR_DIR_CTRL1 {
    #[doc = "IOMUXC XBAR_INOUT4 function direction select"]
    pub mod IOMUXC_XBAR_DIR_SEL_4 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "XBAR_INOUT as input"]
            pub const IOMUX: u32 = 0;
            #[doc = "XBAR_INOUT as output"]
            pub const XBAR: u32 = 0x01;
        }
    }
    #[doc = "IOMUXC XBAR_INOUT5 function direction select"]
    pub mod IOMUXC_XBAR_DIR_SEL_5 {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "XBAR_INOUT as input"]
            pub const IOMUX: u32 = 0;
            #[doc = "XBAR_INOUT as output"]
            pub const XBAR: u32 = 0x01;
        }
    }
    #[doc = "IOMUXC XBAR_INOUT6 function direction select"]
    pub mod IOMUXC_XBAR_DIR_SEL_6 {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "XBAR_INOUT as input"]
            pub const IOMUX: u32 = 0;
            #[doc = "XBAR_INOUT as output"]
            pub const XBAR: u32 = 0x01;
        }
    }
    #[doc = "IOMUXC XBAR_INOUT7 function direction select"]
    pub mod IOMUXC_XBAR_DIR_SEL_7 {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "XBAR_INOUT as input"]
            pub const IOMUX: u32 = 0;
            #[doc = "XBAR_INOUT as output"]
            pub const XBAR: u32 = 0x01;
        }
    }
    #[doc = "IOMUXC XBAR_INOUT8 function direction select"]
    pub mod IOMUXC_XBAR_DIR_SEL_8 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "XBAR_INOUT as input"]
            pub const IOMUX: u32 = 0;
            #[doc = "XBAR_INOUT as output"]
            pub const XBAR: u32 = 0x01;
        }
    }
    #[doc = "IOMUXC XBAR_INOUT9 function direction select"]
    pub mod IOMUXC_XBAR_DIR_SEL_9 {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "XBAR_INOUT as input"]
            pub const IOMUX: u32 = 0;
            #[doc = "XBAR_INOUT as output"]
            pub const XBAR: u32 = 0x01;
        }
    }
    #[doc = "IOMUXC XBAR_INOUT10 function direction select"]
    pub mod IOMUXC_XBAR_DIR_SEL_10 {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "XBAR_INOUT as input"]
            pub const IOMUX: u32 = 0;
            #[doc = "XBAR_INOUT as output"]
            pub const XBAR: u32 = 0x01;
        }
    }
    #[doc = "IOMUXC XBAR_INOUT11 function direction select"]
    pub mod IOMUXC_XBAR_DIR_SEL_11 {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "XBAR_INOUT as input"]
            pub const IOMUX: u32 = 0;
            #[doc = "XBAR_INOUT as output"]
            pub const XBAR: u32 = 0x01;
        }
    }
    #[doc = "IOMUXC XBAR_INOUT12 function direction select"]
    pub mod IOMUXC_XBAR_DIR_SEL_12 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "XBAR_INOUT as input"]
            pub const IOMUX: u32 = 0;
            #[doc = "XBAR_INOUT as output"]
            pub const XBAR: u32 = 0x01;
        }
    }
    #[doc = "IOMUXC XBAR_INOUT13 function direction select"]
    pub mod IOMUXC_XBAR_DIR_SEL_13 {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "XBAR_INOUT as input"]
            pub const IOMUX: u32 = 0;
            #[doc = "XBAR_INOUT as output"]
            pub const XBAR: u32 = 0x01;
        }
    }
    #[doc = "IOMUXC XBAR_INOUT14 function direction select"]
    pub mod IOMUXC_XBAR_DIR_SEL_14 {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "XBAR_INOUT as input"]
            pub const IOMUX: u32 = 0;
            #[doc = "XBAR_INOUT as output"]
            pub const XBAR: u32 = 0x01;
        }
    }
    #[doc = "IOMUXC XBAR_INOUT15 function direction select"]
    pub mod IOMUXC_XBAR_DIR_SEL_15 {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "XBAR_INOUT as input"]
            pub const IOMUX: u32 = 0;
            #[doc = "XBAR_INOUT as output"]
            pub const XBAR: u32 = 0x01;
        }
    }
    #[doc = "IOMUXC XBAR_INOUT16 function direction select"]
    pub mod IOMUXC_XBAR_DIR_SEL_16 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "XBAR_INOUT as input"]
            pub const IOMUX: u32 = 0;
            #[doc = "XBAR_INOUT as output"]
            pub const XBAR: u32 = 0x01;
        }
    }
    #[doc = "IOMUXC XBAR_INOUT17 function direction select"]
    pub mod IOMUXC_XBAR_DIR_SEL_17 {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "XBAR_INOUT as input"]
            pub const IOMUX: u32 = 0;
            #[doc = "XBAR_INOUT as output"]
            pub const XBAR: u32 = 0x01;
        }
    }
    #[doc = "IOMUXC XBAR_INOUT18 function direction select"]
    pub mod IOMUXC_XBAR_DIR_SEL_18 {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "XBAR_INOUT as input"]
            pub const IOMUX: u32 = 0;
            #[doc = "XBAR_INOUT as output"]
            pub const XBAR: u32 = 0x01;
        }
    }
    #[doc = "IOMUXC XBAR_INOUT19 function direction select"]
    pub mod IOMUXC_XBAR_DIR_SEL_19 {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "XBAR_INOUT as input"]
            pub const IOMUX: u32 = 0;
            #[doc = "XBAR_INOUT as output"]
            pub const XBAR: u32 = 0x01;
        }
    }
    #[doc = "IOMUXC XBAR_INOUT20 function direction select"]
    pub mod IOMUXC_XBAR_DIR_SEL_20 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "XBAR_INOUT as input"]
            pub const IOMUX: u32 = 0;
            #[doc = "XBAR_INOUT as output"]
            pub const XBAR: u32 = 0x01;
        }
    }
    #[doc = "IOMUXC XBAR_INOUT21 function direction select"]
    pub mod IOMUXC_XBAR_DIR_SEL_21 {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "XBAR_INOUT as input"]
            pub const IOMUX: u32 = 0;
            #[doc = "XBAR_INOUT as output"]
            pub const XBAR: u32 = 0x01;
        }
    }
    #[doc = "IOMUXC XBAR_INOUT22 function direction select"]
    pub mod IOMUXC_XBAR_DIR_SEL_22 {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "XBAR_INOUT as input"]
            pub const IOMUX: u32 = 0;
            #[doc = "XBAR_INOUT as output"]
            pub const XBAR: u32 = 0x01;
        }
    }
    #[doc = "IOMUXC XBAR_INOUT23 function direction select"]
    pub mod IOMUXC_XBAR_DIR_SEL_23 {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "XBAR_INOUT as input"]
            pub const IOMUX: u32 = 0;
            #[doc = "XBAR_INOUT as output"]
            pub const XBAR: u32 = 0x01;
        }
    }
    #[doc = "IOMUXC XBAR_INOUT24 function direction select"]
    pub mod IOMUXC_XBAR_DIR_SEL_24 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "XBAR_INOUT as input"]
            pub const IOMUX: u32 = 0;
            #[doc = "XBAR_INOUT as output"]
            pub const XBAR: u32 = 0x01;
        }
    }
    #[doc = "IOMUXC XBAR_INOUT25 function direction select"]
    pub mod IOMUXC_XBAR_DIR_SEL_25 {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "XBAR_INOUT as input"]
            pub const IOMUX: u32 = 0;
            #[doc = "XBAR_INOUT as output"]
            pub const XBAR: u32 = 0x01;
        }
    }
    #[doc = "IOMUXC XBAR_INOUT26 function direction select"]
    pub mod IOMUXC_XBAR_DIR_SEL_26 {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "XBAR_INOUT as input"]
            pub const IOMUX: u32 = 0;
            #[doc = "XBAR_INOUT as output"]
            pub const XBAR: u32 = 0x01;
        }
    }
    #[doc = "IOMUXC XBAR_INOUT27 function direction select"]
    pub mod IOMUXC_XBAR_DIR_SEL_27 {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "XBAR_INOUT as input"]
            pub const IOMUX: u32 = 0;
            #[doc = "XBAR_INOUT as output"]
            pub const XBAR: u32 = 0x01;
        }
    }
    #[doc = "IOMUXC XBAR_INOUT28 function direction select"]
    pub mod IOMUXC_XBAR_DIR_SEL_28 {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "XBAR_INOUT as input"]
            pub const IOMUX: u32 = 0;
            #[doc = "XBAR_INOUT as output"]
            pub const XBAR: u32 = 0x01;
        }
    }
    #[doc = "IOMUXC XBAR_INOUT29 function direction select"]
    pub mod IOMUXC_XBAR_DIR_SEL_29 {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "XBAR_INOUT as input"]
            pub const IOMUX: u32 = 0;
            #[doc = "XBAR_INOUT as output"]
            pub const XBAR: u32 = 0x01;
        }
    }
    #[doc = "IOMUXC XBAR_INOUT30 function direction select"]
    pub mod IOMUXC_XBAR_DIR_SEL_30 {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "XBAR_INOUT as input"]
            pub const IOMUX: u32 = 0;
            #[doc = "XBAR_INOUT as output"]
            pub const XBAR: u32 = 0x01;
        }
    }
    #[doc = "IOMUXC XBAR_INOUT31 function direction select"]
    pub mod IOMUXC_XBAR_DIR_SEL_31 {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "XBAR_INOUT as input"]
            pub const IOMUX: u32 = 0;
            #[doc = "XBAR_INOUT as output"]
            pub const XBAR: u32 = 0x01;
        }
    }
}
#[doc = "XBAR IO direction control register"]
pub mod XBAR_DIR_CTRL2 {
    #[doc = "IOMUXC XBAR_INOUT32 function direction select"]
    pub mod IOMUXC_XBAR_DIR_SEL_32 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "XBAR_INOUT as input"]
            pub const IOMUX: u32 = 0;
            #[doc = "XBAR_INOUT as output"]
            pub const XBAR: u32 = 0x01;
        }
    }
    #[doc = "IOMUXC XBAR_INOUT33 function direction select"]
    pub mod IOMUXC_XBAR_DIR_SEL_33 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "XBAR_INOUT as input"]
            pub const IOMUX: u32 = 0;
            #[doc = "XBAR_INOUT as output"]
            pub const XBAR: u32 = 0x01;
        }
    }
    #[doc = "IOMUXC XBAR_INOUT34 function direction select"]
    pub mod IOMUXC_XBAR_DIR_SEL_34 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "XBAR_INOUT as input"]
            pub const IOMUX: u32 = 0;
            #[doc = "XBAR_INOUT as output"]
            pub const XBAR: u32 = 0x01;
        }
    }
    #[doc = "IOMUXC XBAR_INOUT35 function direction select"]
    pub mod IOMUXC_XBAR_DIR_SEL_35 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "XBAR_INOUT as input"]
            pub const IOMUX: u32 = 0;
            #[doc = "XBAR_INOUT as output"]
            pub const XBAR: u32 = 0x01;
        }
    }
    #[doc = "IOMUXC XBAR_INOUT36 function direction select"]
    pub mod IOMUXC_XBAR_DIR_SEL_36 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "XBAR_INOUT as input"]
            pub const IOMUX: u32 = 0;
            #[doc = "XBAR_INOUT as output"]
            pub const XBAR: u32 = 0x01;
        }
    }
    #[doc = "IOMUXC XBAR_INOUT37 function direction select"]
    pub mod IOMUXC_XBAR_DIR_SEL_37 {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "XBAR_INOUT as input"]
            pub const IOMUX: u32 = 0;
            #[doc = "XBAR_INOUT as output"]
            pub const XBAR: u32 = 0x01;
        }
    }
}
#[doc = "LPIT trigger input select register"]
pub mod LPIT_TRIG_SEL {
    #[doc = "LPIT1 TRIG0 input select"]
    pub mod LPIT1_TRIG0_INPUT_SEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Input from IOMUX"]
            pub const IOMUX: u32 = 0;
            #[doc = "Input from XBAR"]
            pub const XBAR: u32 = 0x01;
        }
    }
    #[doc = "LPIT1 TRIG1 input select"]
    pub mod LPIT1_TRIG1_INPUT_SEL {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Input from IOMUX"]
            pub const IOMUX: u32 = 0;
            #[doc = "Input from XBAR"]
            pub const XBAR: u32 = 0x01;
        }
    }
    #[doc = "LPIT1 TRIG2 input select"]
    pub mod LPIT1_TRIG2_INPUT_SEL {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Input from IOMUX"]
            pub const IOMUX: u32 = 0;
            #[doc = "Input from XBAR"]
            pub const XBAR: u32 = 0x01;
        }
    }
    #[doc = "LPIT1 TRIG3 input select"]
    pub mod LPIT1_TRIG3_INPUT_SEL {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Input from IOMUX"]
            pub const IOMUX: u32 = 0;
            #[doc = "Input from XBAR"]
            pub const XBAR: u32 = 0x01;
        }
    }
    #[doc = "LPIT2 TRIG0 input select"]
    pub mod LPIT2_TRIG0_INPUT_SEL {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Input from IOMUX"]
            pub const IOMUX: u32 = 0;
            #[doc = "Input from XBAR"]
            pub const XBAR: u32 = 0x01;
        }
    }
    #[doc = "LPIT2 TRIG1 input select"]
    pub mod LPIT2_TRIG1_INPUT_SEL {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Input from IOMUX"]
            pub const IOMUX: u32 = 0;
            #[doc = "Input from XBAR"]
            pub const XBAR: u32 = 0x01;
        }
    }
    #[doc = "LPIT2 TRIG2 input select"]
    pub mod LPIT2_TRIG2_INPUT_SEL {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Input from IOMUX"]
            pub const IOMUX: u32 = 0;
            #[doc = "Input from XBAR"]
            pub const XBAR: u32 = 0x01;
        }
    }
    #[doc = "LPIT2 TRIG3 input select"]
    pub mod LPIT2_TRIG3_INPUT_SEL {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Input from IOMUX"]
            pub const IOMUX: u32 = 0;
            #[doc = "Input from XBAR"]
            pub const XBAR: u32 = 0x01;
        }
    }
    #[doc = "LPIT3 TRIG0 input select"]
    pub mod LPIT3_TRIG0_INPUT_SEL {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Input from IOMUX"]
            pub const IOMUX: u32 = 0;
            #[doc = "Input from XBAR"]
            pub const XBAR: u32 = 0x01;
        }
    }
    #[doc = "LPIT3 TRIG1 input select"]
    pub mod LPIT3_TRIG1_INPUT_SEL {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Input from IOMUX"]
            pub const IOMUX: u32 = 0;
            #[doc = "Input from XBAR"]
            pub const XBAR: u32 = 0x01;
        }
    }
    #[doc = "LPIT3 TRIG2 input select"]
    pub mod LPIT3_TRIG2_INPUT_SEL {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Input from IOMUX"]
            pub const IOMUX: u32 = 0;
            #[doc = "Input from XBAR"]
            pub const XBAR: u32 = 0x01;
        }
    }
    #[doc = "LPIT3 TRIG3 input select"]
    pub mod LPIT3_TRIG3_INPUT_SEL {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Input from IOMUX"]
            pub const IOMUX: u32 = 0;
            #[doc = "Input from XBAR"]
            pub const XBAR: u32 = 0x01;
        }
    }
}
#[doc = "AXI bus attribute configuration register"]
pub mod AXI_ATTR_CFG {
    #[doc = "uSDHC1 block cacheable attribute value of AXI read transactions"]
    pub mod ARCACHE_USDHC1 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Cacheable attribute is off for read transactions"]
            pub const OFF: u32 = 0;
            #[doc = "Cacheable attribute is on for read transactions"]
            pub const ON: u32 = 0x01;
        }
    }
    #[doc = "uSDHC1 block cacheable attribute value of AXI write transactions"]
    pub mod AWCACHE_USDHC1 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Cacheable attribute is off for write transactions"]
            pub const OFF: u32 = 0;
            #[doc = "Cacheable attribute is on for write transactions"]
            pub const ON: u32 = 0x01;
        }
    }
    #[doc = "uSDHC2 block cacheable attribute value of AXI read transactions"]
    pub mod ARCACHE_USDHC2 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Cacheable attribute is off for read transactions"]
            pub const OFF: u32 = 0;
            #[doc = "Cacheable attribute is on for read transactions"]
            pub const ON: u32 = 0x01;
        }
    }
    #[doc = "uSDHC2 block cacheable attribute value of AXI write transactions"]
    pub mod AWCACHE_USDHC2 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Cacheable attribute is off for write transactions"]
            pub const OFF: u32 = 0;
            #[doc = "Cacheable attribute is on for write transactions"]
            pub const ON: u32 = 0x01;
        }
    }
    #[doc = "USB block cacheable attribute value of AXI read transactions"]
    pub mod ARCACHE_USB {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Cacheable attribute is off for read transactions"]
            pub const OFF: u32 = 0;
            #[doc = "Cacheable attribute is on for read transactions"]
            pub const ON: u32 = 0x01;
        }
    }
    #[doc = "USB block cacheable attribute value of AXI write transactions"]
    pub mod AWCACHE_USB {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Cacheable attribute is off for write transactions"]
            pub const OFF: u32 = 0;
            #[doc = "Cacheable attribute is on for write transactions"]
            pub const ON: u32 = 0x01;
        }
    }
}
#[doc = "SRAM Control Register 0"]
pub mod SRAMCR0 {
    #[doc = "AHB Bus Timeout Wait Cycle"]
    pub mod BTO {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "AHB Bus Timeout Enable"]
    pub mod BTOEN {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "AHB bus timeout counter is not enabled."]
            pub const DISABLE: u32 = 0;
            #[doc = "AHB bus timeout counter is enabled."]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Port Size"]
    pub mod PS {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "8bit"]
            pub const PS8: u32 = 0;
            #[doc = "16bit"]
            pub const PS16: u32 = 0x01;
        }
    }
    #[doc = "Address Mode"]
    pub mod AM {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Address/Data MUX mode (ADMUX)"]
            pub const AM0: u32 = 0;
            #[doc = "Address/Data non-MUX mode (Non-ADMUX)"]
            pub const AM1: u32 = 0x01;
        }
    }
    #[doc = "ADV# polarity"]
    pub mod ADVP {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ADV# is active low."]
            pub const ADVP0: u32 = 0;
            #[doc = "ADV# is active high."]
            pub const ADVP1: u32 = 0x01;
        }
    }
    #[doc = "CE setup time"]
    pub mod CES {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CE hold time"]
    pub mod CEH {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Address setup time"]
    pub mod AS {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Address hold time"]
    pub mod AH {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Turnaround time"]
    pub mod TA {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SRAM Control Register 1"]
pub mod SRAMCR1 {
    #[doc = "WE low time"]
    pub mod WEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "WE high time"]
    pub mod WEH {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "RE low time"]
    pub mod REL {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "RE high time"]
    pub mod REH {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Prescaler timer"]
    pub mod PRE {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Time granularity is 1 clock cycle."]
            pub const PRE0: u32 = 0;
            #[doc = "Time granularity is 2 clock cycles."]
            pub const PRE1: u32 = 0x01;
            #[doc = "Time granularity is 3 clock cycles."]
            pub const PRE2: u32 = 0x02;
            #[doc = "Time granularity is 4 clock cycles."]
            pub const PRE3: u32 = 0x03;
        }
    }
}
#[doc = "Slave stop mode configure register"]
pub mod SLAVE_STOP_MODE_CFG {
    #[doc = "ADC1 stop mode selection."]
    pub mod ADC1_IPG_STOP_MODE {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "This module is functional in Stop Mode"]
            pub const FUNC: u32 = 0;
            #[doc = "This module is not functional in Stop Mode"]
            pub const NONFUNC: u32 = 0x01;
        }
    }
    #[doc = "ADC2 stop mode selection."]
    pub mod ADC2_IPG_STOP_MODE {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "This module is functional in Stop Mode"]
            pub const FUNC: u32 = 0;
            #[doc = "This module is not functional in Stop Mode"]
            pub const NONFUNC: u32 = 0x01;
        }
    }
}
#[doc = "I3C2 async wakeup control register"]
pub mod I3C2_ASYNC_WAKEUP_CTRL {
    #[doc = "Async wakeup interrupt clear"]
    pub mod IRQ_CLR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Async wakeup interrupt status"]
    pub mod IRQ_STATUS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt not asserted"]
            pub const ENABLE: u32 = 0;
            #[doc = "Interrupt asserted"]
            pub const DISABLE: u32 = 0x01;
        }
    }
    #[doc = "Master mode async wakeup interrupt enable"]
    pub mod IRQ_EN {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Interrupt enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
}
#[doc = "XBAR and AOI write protect register"]
pub mod XBAR_AOI_WE {
    #[doc = "Write Enable to XBAR and AOI"]
    pub mod WE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Write is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Write is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
}
#[doc = "XBAR trigger synchronizer control register1"]
pub mod XBAR_TRIG_SYNC_CTRL1 {
    #[doc = "Trigger out polarity select"]
    pub mod POL_SEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Same as trigger in"]
            pub const SAME: u32 = 0;
            #[doc = "Invert trigger in"]
            pub const INV: u32 = 0x01;
        }
    }
    #[doc = "Asynchronous trigger in enable"]
    pub mod ASYNC_EN {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Trigger in is synchronous"]
            pub const SYNC: u32 = 0;
            #[doc = "Trigger in is asynchronous"]
            pub const ASYNC: u32 = 0x01;
        }
    }
    #[doc = "Trigger out synchronizer enable"]
    pub mod SYNC_ENABLE {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Channel is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Channel is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
}
#[doc = "XBAR trigger synchronizer control register2"]
pub mod XBAR_TRIG_SYNC_CTRL2 {
    #[doc = "Pulse width control register of channel0"]
    pub mod PULSE_WIDTH0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Pulse width control register of channel1"]
    pub mod PULSE_WIDTH1 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Pulse width control register of channel2"]
    pub mod PULSE_WIDTH2 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Pulse width control register of channel3"]
    pub mod PULSE_WIDTH3 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Pulse width control register of channel4"]
    pub mod PULSE_WIDTH4 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Pulse width control register of channel5"]
    pub mod PULSE_WIDTH5 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Pulse width control register of channel6"]
    pub mod PULSE_WIDTH6 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Pulse width control register of channel7"]
    pub mod PULSE_WIDTH7 {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "NETC link configuration for port0"]
pub mod NETC_LINK_CFG0 {
    #[doc = "MII protocol selection"]
    pub mod MII_PROT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MII"]
            pub const MII: u32 = 0;
            #[doc = "RMII"]
            pub const RMII: u32 = 0x01;
            #[doc = "RGMII"]
            pub const RGMII: u32 = 0x02;
        }
    }
    #[doc = "IO variant selection"]
    pub mod IO_VAR {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "None"]
            pub const NONE: u32 = 0;
        }
    }
}
#[doc = "NETC link configuration for port1"]
pub mod NETC_LINK_CFG1 {
    #[doc = "MII protocol selection"]
    pub mod MII_PROT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MII"]
            pub const MII: u32 = 0;
            #[doc = "RMII"]
            pub const RMII: u32 = 0x01;
            #[doc = "RGMII"]
            pub const RGMII: u32 = 0x02;
        }
    }
    #[doc = "IO variant selection"]
    pub mod IO_VAR {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "None"]
            pub const NONE: u32 = 0;
        }
    }
}
#[doc = "NETC link configuration for port2"]
pub mod NETC_LINK_CFG2 {
    #[doc = "MII protocol selection"]
    pub mod MII_PROT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MII"]
            pub const MII: u32 = 0;
            #[doc = "RMII"]
            pub const RMII: u32 = 0x01;
            #[doc = "RGMII"]
            pub const RGMII: u32 = 0x02;
        }
    }
    #[doc = "IO variant selection"]
    pub mod IO_VAR {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "None"]
            pub const DISABLE: u32 = 0;
            #[doc = "no description available"]
            pub const RES_1: u32 = 0x01;
            #[doc = "no description available"]
            pub const RES_2: u32 = 0x02;
            #[doc = "no description available"]
            pub const RES_3: u32 = 0x03;
            #[doc = "no description available"]
            pub const RES_4: u32 = 0x04;
            #[doc = "no description available"]
            pub const RES_5: u32 = 0x05;
            #[doc = "no description available"]
            pub const RES_6: u32 = 0x06;
            #[doc = "no description available"]
            pub const RES_7: u32 = 0x07;
            #[doc = "no description available"]
            pub const RES_8: u32 = 0x08;
            #[doc = "no description available"]
            pub const RES_9: u32 = 0x09;
        }
    }
    #[doc = "When REVMII=1 and MII_PROT=MII, this bit configures RevMII rates, otherwise this field has no meaning."]
    pub mod REVMII_RATE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MII interface is operating at 100Mbps"]
            pub const MII_100: u32 = 0;
            #[doc = "MII interface is operating at 10Mbps"]
            pub const MII_10: u32 = 0x01;
        }
    }
    #[doc = "RevMII selection"]
    pub mod REVMII {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "NETC link configuration for port3"]
pub mod NETC_LINK_CFG3 {
    #[doc = "MII protocol selection"]
    pub mod MII_PROT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "IO variant selection"]
    pub mod IO_VAR {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "When REVMII=1 and MII_PROT=MII, this bit configures RevMII rates, otherwise this field has no meaning."]
    pub mod REVMII_RATE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "RevMII selection"]
    pub mod REVMII {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "NETC link configuration for port4"]
pub mod NETC_LINK_CFG4 {
    #[doc = "MII protocol selection"]
    pub mod MII_PROT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "IO variant selection"]
    pub mod IO_VAR {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "When REVMII=1 and MII_PROT=MII, this bit configures RevMII rates, otherwise this field has no meaning."]
    pub mod REVMII_RATE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "RevMII selection"]
    pub mod REVMII {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "RevMII not selected"]
            pub const DISABLE: u32 = 0;
            #[doc = "RevMII selected"]
            pub const ENABLE: u32 = 0x01;
        }
    }
}
#[doc = "NETC RevMII RGMII delay line configuration for port0"]
pub mod NETC_REVMII_DLL0 {
    #[doc = "Delay target of slave delay line"]
    pub mod DLY_TARGET {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Reference delay line lock flag"]
    pub mod REF_LOCK {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Reference delay line is not locked"]
            pub const DISABLE: u32 = 0;
            #[doc = "Reference delay line is locked"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Slave delay line lock flag"]
    pub mod SLV_LOCK {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Slave delay line is not locked"]
            pub const NOT_LOCKED: u32 = 0;
            #[doc = "Slave delay line is locked"]
            pub const LOCKED: u32 = 0x01;
        }
    }
}
#[doc = "NETC RevMII RGMII delay line configuration for port1"]
pub mod NETC_REVMII_DLL1 {
    #[doc = "Delay target of slave delay line"]
    pub mod DLY_TARGET {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Reference delay line lock flag"]
    pub mod REF_LOCK {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Reference delay line is not locked"]
            pub const DISABLE: u32 = 0;
            #[doc = "Reference delay line is locked"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Slave delay line lock flag"]
    pub mod SLV_LOCK {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Slave delay line is not locked"]
            pub const DISABLE: u32 = 0;
            #[doc = "Slave delay line is locked"]
            pub const ENABLE: u32 = 0x01;
        }
    }
}
#[doc = "NETC RevMII RGMII delay line configuration for port2"]
pub mod NETC_REVMII_DLL2 {
    #[doc = "Delay target of slave delay line"]
    pub mod DLY_TARGET {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Reference delay line lock flag"]
    pub mod REF_LOCK {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Reference delay line is not locked"]
            pub const DISABLE: u32 = 0;
            #[doc = "Reference delay line is locked"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Slave delay line lock flag"]
    pub mod SLV_LOCK {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Slave delay line is not locked"]
            pub const DISABLE: u32 = 0;
            #[doc = "Slave delay line is locked"]
            pub const ENABLE: u32 = 0x01;
        }
    }
}
#[doc = "NETC RevMII RGMII delay line configuration for port3"]
pub mod NETC_REVMII_DLL3 {
    #[doc = "Delay target of slave delay line"]
    pub mod DLY_TARGET {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Reference delay line lock flag"]
    pub mod REF_LOCK {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Reference delay line is not locked"]
            pub const DISABLE: u32 = 0;
            #[doc = "Reference delay line is locked"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Slave delay line lock flag"]
    pub mod SLV_LOCK {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Slave delay line is not locked"]
            pub const DISABLE: u32 = 0;
            #[doc = "Slave delay line is locked"]
            pub const ENABLE: u32 = 0x01;
        }
    }
}
#[doc = "NETC RevMII RGMII delay line configuration for port4"]
pub mod NETC_REVMII_DLL4 {
    #[doc = "Delay target of slave delay line"]
    pub mod DLY_TARGET {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Reference delay line lock flag"]
    pub mod REF_LOCK {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Reference delay line is not locked"]
            pub const DISABLE: u32 = 0;
            #[doc = "Reference delay line is locked"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Slave delay line lock flag"]
    pub mod SLV_LOCK {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Slave delay line is not locked"]
            pub const DISABLE: u32 = 0;
            #[doc = "Slave delay line is locked"]
            pub const ENABLE: u32 = 0x01;
        }
    }
}
#[doc = "Safety clock monitor control and status register"]
pub mod SAFETY_CLK_MON_CS {
    #[doc = "Monitor enable bit"]
    pub mod MON_EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The monitor is off"]
            pub const DISABLE: u32 = 0;
            #[doc = "The monitor is on"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Interrupt enable"]
    pub mod IRQ_EN {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Clock failure will not assert interrupt"]
            pub const DISABLE: u32 = 0;
            #[doc = "Clock failure will assert interrupt"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Reset out enable"]
    pub mod FAST_RST_EN {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Clock failure will not assert EWM_OUT"]
            pub const DISABLE: u32 = 0;
            #[doc = "Clock failure will assert EWM_OUT_b immediately regardless EWM state"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Status clear"]
    pub mod STAT_CLR {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No effect to clock failure status bit"]
            pub const DISABLE: u32 = 0;
            #[doc = "Clear clock failure status bit"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "XBAR_OUT220 clock failure status"]
    pub mod STAT {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No failure detected by the monitor"]
            pub const DISABLE: u32 = 0;
            #[doc = "Clock failure has been detected by the monitor"]
            pub const ENABLE: u32 = 0x01;
        }
    }
}
#[doc = "Safety clock monitor threshold register"]
pub mod SAFETY_CLK_MON_TH {
    #[doc = "Threshold low value"]
    pub mod TH_LOW {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Threshold high value"]
    pub mod TH_HIGH {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO_EMC_B1 bank IO control"]
pub mod EMC_B1_IO_CTRL {
    #[doc = "Compensation code freeze"]
    pub mod GPIO_EMC1_FREEZE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "COMPEN and COMPTQ control the operating modes of the compensation cell"]
    pub mod GPIO_EMC1_COMPTQ {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "COMPEN and COMPTQ control the operating modes of the compensation cell"]
    pub mod GPIO_EMC1_COMPEN {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Compensation code fast freeze enable"]
    pub mod GPIO_EMC1_FASTFRZ_EN {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "GPIO_EMC_B1 IO bank's 4-bit PMOS compensation codes from core"]
    pub mod GPIO_EMC1_RASRCP {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "GPIO_EMC_B1 IO bank's 4-bit NMOS compensation codes from core"]
    pub mod GPIO_EMC1_RASRCN {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "GPIO_EMC1_NASRC selection"]
    pub mod GPIO_EMC1_SELECT_NASRC {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "GPIO_EMC_B1 IO bank reference voltage generator cell sleep enable"]
    pub mod GPIO_EMC1_REFGEN_SLEEP {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "GPIO_EMC_B1 IO bank power supply mode latch enable"]
    pub mod GPIO_EMC1_SUPLYDET_LATCH {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Compensation code fast-freeze"]
    pub mod GPIO_EMC1_FASTFRZ {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "GPIO_EMC_B1 IO bank compensation OK flag"]
    pub mod GPIO_EMC1_COMPOK {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "GPIO_EMC_B1 IO bank compensation codes"]
    pub mod GPIO_EMC1_NASRC {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO_EMC_B2 bank IO control"]
pub mod EMC_B2_IO_CTRL {
    #[doc = "Compensation code freeze"]
    pub mod GPIO_EMC2_FREEZE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "COMPEN and COMPTQ control the operating modes of the compensation cell"]
    pub mod GPIO_EMC2_COMPTQ {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "COMPEN and COMPTQ control the operating modes of the compensation cell"]
    pub mod GPIO_EMC2_COMPEN {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Compensation code fast freeze enable"]
    pub mod GPIO_EMC2_FASTFRZ_EN {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "GPIO_EMC_B2 IO bank's 4-bit PMOS compensation codes from core"]
    pub mod GPIO_EMC2_RASRCP {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "GPIO_EMC_B2 IO bank's 4-bit NMOS compensation codes from core"]
    pub mod GPIO_EMC2_RASRCN {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "GPIO_EMC2_NASRC selection"]
    pub mod GPIO_EMC2_SELECT_NASRC {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Show the 4-bit PMOS compensation codes in GPIO_EMC2_NASRC field"]
            pub const PMOS: u32 = 0;
            #[doc = "Show the 4-bit NMOS compensation codes in GPIO_EMC2_NASRC field"]
            pub const NMOS: u32 = 0x01;
        }
    }
    #[doc = "GPIO_EMC_B2 IO bank reference voltage generator cell sleep enable"]
    pub mod GPIO_EMC2_REFGEN_SLEEP {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "GPIO_EMC_B2 IO bank power supply mode latch enable"]
    pub mod GPIO_EMC2_SUPLYDET_LATCH {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Compensation code fast-freeze"]
    pub mod GPIO_EMC2_FASTFRZ {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "GPIO_EMC_B2 IO bank compensation OK flag"]
    pub mod GPIO_EMC2_COMPOK {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "GPIO_EMC_B2 IO bank compensation codes"]
    pub mod GPIO_EMC2_NASRC {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO_SD_B1 bank IO control"]
pub mod SD_B1_IO_CTRL {
    #[doc = "Compensation code freeze"]
    pub mod GPIO_SD1_FREEZE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "COMPEN and COMPTQ control the operating modes of the compensation cell"]
    pub mod GPIO_SD1_COMPTQ {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "COMPEN and COMPTQ control the operating modes of the compensation cell"]
    pub mod GPIO_SD1_COMPEN {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Compensation code fast freeze enable"]
    pub mod GPIO_SD1_FASTFRZ_EN {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "GPIO_SD_B1 IO bank's 4-bit PMOS compensation codes from core"]
    pub mod GPIO_SD1_RASRCP {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "GPIO_SD_B1 IO bank's 4-bit NMOS compensation codes from core"]
    pub mod GPIO_SD1_RASRCN {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "GPIO_SD1_NASRC selection"]
    pub mod GPIO_SD1_SELECT_NASRC {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "GPIO_SD_B1 IO bank reference voltage generator cell sleep enable"]
    pub mod GPIO_SD1_REFGEN_SLEEP {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "GPIO_SD_B1 IO bank power supply mode latch enable"]
    pub mod GPIO_SD1_SUPLYDET_LATCH {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Compensation code fast-freeze"]
    pub mod GPIO_SD1_FASTFRZ {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "GPIO_SD_B1 IO bank compensation OK flag"]
    pub mod GPIO_SD1_COMPOK {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "GPIO_SD_B1 IO bank compensation codes"]
    pub mod GPIO_SD1_NASRC {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO_SD_B2 bank IO control"]
pub mod SD_B2_IO_CTRL {
    #[doc = "Compensation code freeze"]
    pub mod GPIO_SD2_FREEZE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "COMPEN and COMPTQ control the operating modes of the compensation cell"]
    pub mod GPIO_SD2_COMPTQ {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "COMPEN and COMPTQ control the operating modes of the compensation cell"]
    pub mod GPIO_SD2_COMPEN {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Compensation code fast freeze enable"]
    pub mod GPIO_SD2_FASTFRZ_EN {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "GPIO_SD_B2 IO bank's 4-bit PMOS compensation codes from core"]
    pub mod GPIO_SD2_RASRCP {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "GPIO_SD_B2 IO bank's 4-bit NMOS compensation codes from core"]
    pub mod GPIO_SD2_RASRCN {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "GPIO_SD2_NASRC selection"]
    pub mod GPIO_SD2_SELECT_NASRC {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "GPIO_SD_B2 IO bank reference voltage generator cell sleep enable"]
    pub mod GPIO_SD2_REFGEN_SLEEP {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "GPIO_SD_B2 IO bank power supply mode latch enable"]
    pub mod GPIO_SD2_SUPLYDET_LATCH {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Compensation code fast-freeze"]
    pub mod GPIO_SD2_FASTFRZ {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "GPIO_SD_B2 IO bank compensation OK flag"]
    pub mod GPIO_SD2_COMPOK {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "GPIO_SD_B2 IO bank compensation codes"]
    pub mod GPIO_SD2_NASRC {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO_B1 bank IO control"]
pub mod GPIO_B1_IO_CTRL {
    #[doc = "Compensation code freeze"]
    pub mod GPIO_B1_FREEZE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "COMPEN and COMPTQ control the operating modes of the compensation cell"]
    pub mod GPIO_B1_COMPTQ {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "COMPEN and COMPTQ control the operating modes of the compensation cell"]
    pub mod GPIO_B1_COMPEN {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Compensation code fast freeze enable"]
    pub mod GPIO_B1_FASTFRZ_EN {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "GPIO_B1 IO bank's 4-bit PMOS compensation codes from core"]
    pub mod GPIO_B1_RASRCP {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "GPIO_B1 IO bank's 4-bit NMOS compensation codes from core"]
    pub mod GPIO_B1_RASRCN {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "GPIO_B1_NASRC selection"]
    pub mod GPIO_B1_SELECT_NASRC {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Show the 4-bit PMOS compensation codes in GPIO_B1_NASRC field"]
            pub const PMOS: u32 = 0;
            #[doc = "Show the 4-bit NMOS compensation codes in GPIO_B1_NASRC field"]
            pub const NMOS: u32 = 0x01;
        }
    }
    #[doc = "GPIO_B1 IO bank reference voltage generator cell sleep enable"]
    pub mod GPIO_B1_REFGEN_SLEEP {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "GPIO_B1 IO bank power supply mode latch enable"]
    pub mod GPIO_B1_SUPLYDET_LATCH {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Compensation code fast-freeze"]
    pub mod GPIO_B1_FASTFRZ {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "GPIO_B1 IO bank compensation OK flag"]
    pub mod GPIO_B1_COMPOK {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "GPIO_B1 IO bank compensation codes"]
    pub mod GPIO_B1_NASRC {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO_B2 bank IO control"]
pub mod GPIO_B2_IO_CTRL {
    #[doc = "Compensation code freeze"]
    pub mod GPIO_B2_FREEZE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "COMPEN and COMPTQ control the operating modes of the compensation cell"]
    pub mod GPIO_B2_COMPTQ {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "COMPEN and COMPTQ control the operating modes of the compensation cell"]
    pub mod GPIO_B2_COMPEN {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Compensation code fast freeze enable"]
    pub mod GPIO_B2_FASTFRZ_EN {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "GPIO_B2 IO bank's 4-bit PMOS compensation codes from core"]
    pub mod GPIO_B2_RASRCP {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "GPIO_B2 IO bank's 4-bit NMOS compensation codes from core"]
    pub mod GPIO_B2_RASRCN {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "GPIO_B2_NASRC selection"]
    pub mod GPIO_B2_SELECT_NASRC {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "GPIO_B2 IO bank reference voltage generator cell sleep enable"]
    pub mod GPIO_B2_REFGEN_SLEEP {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "GPIO_B2 IO bank power supply mode latch enable"]
    pub mod GPIO_B2_SUPLYDET_LATCH {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Compensation code fast-freeze"]
    pub mod GPIO_B2_FASTFRZ {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "GPIO_B2 IO bank compensation OK flag"]
    pub mod GPIO_B2_COMPOK {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "GPIO_B2 IO bank compensation codes"]
    pub mod GPIO_B2_NASRC {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Miscellaneous control register of IO"]
pub mod MISC_IO_CTRL {
    #[doc = "Disable I3C on-chip strong pull for I3C2"]
    pub mod I3C_ON_CHIP_STRONG_PULL_DIS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "On-chip strong pull is enabled"]
            pub const ENABLE: u32 = 0;
            #[doc = "On-chip strong pull is disabled"]
            pub const DISABLE: u32 = 0x01;
        }
    }
    #[doc = "GPIO_AD IO bank supply voltage range selection for GPIO_AD_00 to GPIO_AD_17"]
    pub mod GPIO_AD_HIGH_RANGE {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "GPIO_AD IO bank supply voltage range selection for GPIO_AD_00 to GPIO_AD_17"]
    pub mod GPIO_AD_LOW_RANGE {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "GPIO_EMC_B1 IO bank supply voltage detector sleep mode enable"]
    pub mod SUPLYDET_EMC1_SLEEP {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "GPIO_EMC_B2 IO bank supply voltage detector sleep mode enable"]
    pub mod SUPLYDET_EMC2_SLEEP {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "GPIO_SD_B1 IO bank supply voltage detector sleep mode enable"]
    pub mod SUPLYDET_SD1_SLEEP {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "GPIO_SD_B2 IO bank supply voltage detector sleep mode enable"]
    pub mod SUPLYDET_SD2_SLEEP {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "GPIO_GPIO_B1 IO bank supply voltage detector sleep mode enable"]
    pub mod SUPLYDET_GPIO_B1_SLEEP {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "GPIO_GPIO_B1 IO bank supply voltage detector sleep mode enable"]
    pub mod SUPLYDET_GPIO_B2_SLEEP {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ECAT_LINK_ACT\\[0\\] polarity control"]
    pub mod ECAT_LINK_ACT0_POL {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ECAT_LINK_ACT\\[1\\] polarity control"]
    pub mod ECAT_LINK_ACT1_POL {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
