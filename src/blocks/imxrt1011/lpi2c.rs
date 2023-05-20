#[doc = "LPI2C"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "Version ID Register"]
    pub VERID: crate::RORegister<u32>,
    #[doc = "Parameter Register"]
    pub PARAM: crate::RORegister<u32>,
    _reserved0: [u8; 0x08],
    #[doc = "Master Control Register"]
    pub MCR: crate::RWRegister<u32>,
    #[doc = "Master Status Register"]
    pub MSR: crate::RWRegister<u32>,
    #[doc = "Master Interrupt Enable Register"]
    pub MIER: crate::RWRegister<u32>,
    #[doc = "Master DMA Enable Register"]
    pub MDER: crate::RWRegister<u32>,
    #[doc = "Master Configuration Register 0"]
    pub MCFGR0: crate::RWRegister<u32>,
    #[doc = "Master Configuration Register 1"]
    pub MCFGR1: crate::RWRegister<u32>,
    #[doc = "Master Configuration Register 2"]
    pub MCFGR2: crate::RWRegister<u32>,
    #[doc = "Master Configuration Register 3"]
    pub MCFGR3: crate::RWRegister<u32>,
    _reserved1: [u8; 0x10],
    #[doc = "Master Data Match Register"]
    pub MDMR: crate::RWRegister<u32>,
    _reserved2: [u8; 0x04],
    #[doc = "Master Clock Configuration Register 0"]
    pub MCCR0: crate::RWRegister<u32>,
    _reserved3: [u8; 0x04],
    #[doc = "Master Clock Configuration Register 1"]
    pub MCCR1: crate::RWRegister<u32>,
    _reserved4: [u8; 0x04],
    #[doc = "Master FIFO Control Register"]
    pub MFCR: crate::RWRegister<u32>,
    #[doc = "Master FIFO Status Register"]
    pub MFSR: crate::RORegister<u32>,
    #[doc = "Master Transmit Data Register"]
    pub MTDR: crate::WORegister<u32>,
    _reserved5: [u8; 0x0c],
    #[doc = "Master Receive Data Register"]
    pub MRDR: crate::RORegister<u32>,
    _reserved6: [u8; 0x9c],
    #[doc = "Slave Control Register"]
    pub SCR: crate::RWRegister<u32>,
    #[doc = "Slave Status Register"]
    pub SSR: crate::RWRegister<u32>,
    #[doc = "Slave Interrupt Enable Register"]
    pub SIER: crate::RWRegister<u32>,
    #[doc = "Slave DMA Enable Register"]
    pub SDER: crate::RWRegister<u32>,
    _reserved7: [u8; 0x04],
    #[doc = "Slave Configuration Register 1"]
    pub SCFGR1: crate::RWRegister<u32>,
    #[doc = "Slave Configuration Register 2"]
    pub SCFGR2: crate::RWRegister<u32>,
    _reserved8: [u8; 0x14],
    #[doc = "Slave Address Match Register"]
    pub SAMR: crate::RWRegister<u32>,
    _reserved9: [u8; 0x0c],
    #[doc = "Slave Address Status Register"]
    pub SASR: crate::RORegister<u32>,
    #[doc = "Slave Transmit ACK Register"]
    pub STAR: crate::RWRegister<u32>,
    _reserved10: [u8; 0x08],
    #[doc = "Slave Transmit Data Register"]
    pub STDR: crate::WORegister<u32>,
    _reserved11: [u8; 0x0c],
    #[doc = "Slave Receive Data Register"]
    pub SRDR: crate::RORegister<u32>,
}
#[doc = "Version ID Register"]
pub mod VERID {
    #[doc = "Feature Specification Number"]
    pub mod FEATURE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Master only, with standard feature set"]
            pub const FEATURE_2: u32 = 0x02;
            #[doc = "Master and slave, with standard feature set"]
            pub const FEATURE_3: u32 = 0x03;
        }
    }
    #[doc = "Minor Version Number"]
    pub mod MINOR {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Major Version Number"]
    pub mod MAJOR {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Parameter Register"]
pub mod PARAM {
    #[doc = "Master Transmit FIFO Size"]
    pub mod MTXFIFO {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Master Receive FIFO Size"]
    pub mod MRXFIFO {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Master Control Register"]
pub mod MCR {
    #[doc = "Master Enable"]
    pub mod MEN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Master logic is disabled"]
            pub const MEN_0: u32 = 0;
            #[doc = "Master logic is enabled"]
            pub const MEN_1: u32 = 0x01;
        }
    }
    #[doc = "Software Reset"]
    pub mod RST {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Master logic is not reset"]
            pub const RST_0: u32 = 0;
            #[doc = "Master logic is reset"]
            pub const RST_1: u32 = 0x01;
        }
    }
    #[doc = "Doze mode enable"]
    pub mod DOZEN {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Master is enabled in Doze mode"]
            pub const DOZEN_0: u32 = 0;
            #[doc = "Master is disabled in Doze mode"]
            pub const DOZEN_1: u32 = 0x01;
        }
    }
    #[doc = "Debug Enable"]
    pub mod DBGEN {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Master is disabled in debug mode"]
            pub const DBGEN_0: u32 = 0;
            #[doc = "Master is enabled in debug mode"]
            pub const DBGEN_1: u32 = 0x01;
        }
    }
    #[doc = "Reset Transmit FIFO"]
    pub mod RTF {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No effect"]
            pub const RTF_0: u32 = 0;
            #[doc = "Transmit FIFO is reset"]
            pub const RTF_1: u32 = 0x01;
        }
    }
    #[doc = "Reset Receive FIFO"]
    pub mod RRF {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No effect"]
            pub const RRF_0: u32 = 0;
            #[doc = "Receive FIFO is reset"]
            pub const RRF_1: u32 = 0x01;
        }
    }
}
#[doc = "Master Status Register"]
pub mod MSR {
    #[doc = "Transmit Data Flag"]
    pub mod TDF {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Transmit data is not requested"]
            pub const TDF_0: u32 = 0;
            #[doc = "Transmit data is requested"]
            pub const TDF_1: u32 = 0x01;
        }
    }
    #[doc = "Receive Data Flag"]
    pub mod RDF {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Receive Data is not ready"]
            pub const RDF_0: u32 = 0;
            #[doc = "Receive data is ready"]
            pub const RDF_1: u32 = 0x01;
        }
    }
    #[doc = "End Packet Flag"]
    pub mod EPF {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Master has not generated a STOP or Repeated START condition"]
            pub const EPF_0: u32 = 0;
            #[doc = "Master has generated a STOP or Repeated START condition"]
            pub const EPF_1: u32 = 0x01;
        }
    }
    #[doc = "STOP Detect Flag"]
    pub mod SDF {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Master has not generated a STOP condition"]
            pub const SDF_0: u32 = 0;
            #[doc = "Master has generated a STOP condition"]
            pub const SDF_1: u32 = 0x01;
        }
    }
    #[doc = "NACK Detect Flag"]
    pub mod NDF {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Unexpected NACK was not detected"]
            pub const NDF_0: u32 = 0;
            #[doc = "Unexpected NACK was detected"]
            pub const NDF_1: u32 = 0x01;
        }
    }
    #[doc = "Arbitration Lost Flag"]
    pub mod ALF {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Master has not lost arbitration"]
            pub const ALF_0: u32 = 0;
            #[doc = "Master has lost arbitration"]
            pub const ALF_1: u32 = 0x01;
        }
    }
    #[doc = "FIFO Error Flag"]
    pub mod FEF {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No error"]
            pub const FEF_0: u32 = 0;
            #[doc = "Master sending or receiving data without a START condition"]
            pub const FEF_1: u32 = 0x01;
        }
    }
    #[doc = "Pin Low Timeout Flag"]
    pub mod PLTF {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pin low timeout has not occurred or is disabled"]
            pub const PLTF_0: u32 = 0;
            #[doc = "Pin low timeout has occurred"]
            pub const PLTF_1: u32 = 0x01;
        }
    }
    #[doc = "Data Match Flag"]
    pub mod DMF {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Have not received matching data"]
            pub const DMF_0: u32 = 0;
            #[doc = "Have received matching data"]
            pub const DMF_1: u32 = 0x01;
        }
    }
    #[doc = "Master Busy Flag"]
    pub mod MBF {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "I2C Master is idle"]
            pub const MBF_0: u32 = 0;
            #[doc = "I2C Master is busy"]
            pub const MBF_1: u32 = 0x01;
        }
    }
    #[doc = "Bus Busy Flag"]
    pub mod BBF {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "I2C Bus is idle"]
            pub const BBF_0: u32 = 0;
            #[doc = "I2C Bus is busy"]
            pub const BBF_1: u32 = 0x01;
        }
    }
}
#[doc = "Master Interrupt Enable Register"]
pub mod MIER {
    #[doc = "Transmit Data Interrupt Enable"]
    pub mod TDIE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const TDIE_0: u32 = 0;
            #[doc = "Enabled"]
            pub const TDIE_1: u32 = 0x01;
        }
    }
    #[doc = "Receive Data Interrupt Enable"]
    pub mod RDIE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const RDIE_0: u32 = 0;
            #[doc = "Enabled"]
            pub const RDIE_1: u32 = 0x01;
        }
    }
    #[doc = "End Packet Interrupt Enable"]
    pub mod EPIE {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const EPIE_0: u32 = 0;
            #[doc = "Enabled"]
            pub const EPIE_1: u32 = 0x01;
        }
    }
    #[doc = "STOP Detect Interrupt Enable"]
    pub mod SDIE {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const SDIE_0: u32 = 0;
            #[doc = "Enabled"]
            pub const SDIE_1: u32 = 0x01;
        }
    }
    #[doc = "NACK Detect Interrupt Enable"]
    pub mod NDIE {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const NDIE_0: u32 = 0;
            #[doc = "Enabled"]
            pub const NDIE_1: u32 = 0x01;
        }
    }
    #[doc = "Arbitration Lost Interrupt Enable"]
    pub mod ALIE {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const ALIE_0: u32 = 0;
            #[doc = "Enabled"]
            pub const ALIE_1: u32 = 0x01;
        }
    }
    #[doc = "FIFO Error Interrupt Enable"]
    pub mod FEIE {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Enabled"]
            pub const FEIE_0: u32 = 0;
            #[doc = "Disabled"]
            pub const FEIE_1: u32 = 0x01;
        }
    }
    #[doc = "Pin Low Timeout Interrupt Enable"]
    pub mod PLTIE {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const PLTIE_0: u32 = 0;
            #[doc = "Enabled"]
            pub const PLTIE_1: u32 = 0x01;
        }
    }
    #[doc = "Data Match Interrupt Enable"]
    pub mod DMIE {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const DMIE_0: u32 = 0;
            #[doc = "Enabled"]
            pub const DMIE_1: u32 = 0x01;
        }
    }
}
#[doc = "Master DMA Enable Register"]
pub mod MDER {
    #[doc = "Transmit Data DMA Enable"]
    pub mod TDDE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "DMA request is disabled"]
            pub const TDDE_0: u32 = 0;
            #[doc = "DMA request is enabled"]
            pub const TDDE_1: u32 = 0x01;
        }
    }
    #[doc = "Receive Data DMA Enable"]
    pub mod RDDE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "DMA request is disabled"]
            pub const RDDE_0: u32 = 0;
            #[doc = "DMA request is enabled"]
            pub const RDDE_1: u32 = 0x01;
        }
    }
}
#[doc = "Master Configuration Register 0"]
pub mod MCFGR0 {
    #[doc = "Host Request Enable"]
    pub mod HREN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Host request input is disabled"]
            pub const HREN_0: u32 = 0;
            #[doc = "Host request input is enabled"]
            pub const HREN_1: u32 = 0x01;
        }
    }
    #[doc = "Host Request Polarity"]
    pub mod HRPOL {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Active low"]
            pub const HRPOL_0: u32 = 0;
            #[doc = "Active high"]
            pub const HRPOL_1: u32 = 0x01;
        }
    }
    #[doc = "Host Request Select"]
    pub mod HRSEL {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Host request input is pin HREQ"]
            pub const HRSEL_0: u32 = 0;
            #[doc = "Host request input is input trigger"]
            pub const HRSEL_1: u32 = 0x01;
        }
    }
    #[doc = "Circular FIFO Enable"]
    pub mod CIRFIFO {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Circular FIFO is disabled"]
            pub const CIRFIFO_0: u32 = 0;
            #[doc = "Circular FIFO is enabled"]
            pub const CIRFIFO_1: u32 = 0x01;
        }
    }
    #[doc = "Receive Data Match Only"]
    pub mod RDMO {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Received data is stored in the receive FIFO"]
            pub const RDMO_0: u32 = 0;
            #[doc = "Received data is discarded unless the the Data Match Flag (MSR\\[DMF\\]) is set"]
            pub const RDMO_1: u32 = 0x01;
        }
    }
}
#[doc = "Master Configuration Register 1"]
pub mod MCFGR1 {
    #[doc = "Prescaler"]
    pub mod PRESCALE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Divide by 1"]
            pub const PRESCALE_0: u32 = 0;
            #[doc = "Divide by 2"]
            pub const PRESCALE_1: u32 = 0x01;
            #[doc = "Divide by 4"]
            pub const PRESCALE_2: u32 = 0x02;
            #[doc = "Divide by 8"]
            pub const PRESCALE_3: u32 = 0x03;
            #[doc = "Divide by 16"]
            pub const PRESCALE_4: u32 = 0x04;
            #[doc = "Divide by 32"]
            pub const PRESCALE_5: u32 = 0x05;
            #[doc = "Divide by 64"]
            pub const PRESCALE_6: u32 = 0x06;
            #[doc = "Divide by 128"]
            pub const PRESCALE_7: u32 = 0x07;
        }
    }
    #[doc = "Automatic STOP Generation"]
    pub mod AUTOSTOP {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No effect"]
            pub const AUTOSTOP_0: u32 = 0;
            #[doc = "STOP condition is automatically generated whenever the transmit FIFO is empty and the LPI2C master is busy"]
            pub const AUTOSTOP_1: u32 = 0x01;
        }
    }
    #[doc = "IGNACK"]
    pub mod IGNACK {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "LPI2C Master will receive ACK and NACK normally"]
            pub const IGNACK_0: u32 = 0;
            #[doc = "LPI2C Master will treat a received NACK as if it (NACK) was an ACK"]
            pub const IGNACK_1: u32 = 0x01;
        }
    }
    #[doc = "Timeout Configuration"]
    pub mod TIMECFG {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pin Low Timeout Flag will set if SCL is low for longer than the configured timeout"]
            pub const TIMECFG_0: u32 = 0;
            #[doc = "Pin Low Timeout Flag will set if either SCL or SDA is low for longer than the configured timeout"]
            pub const TIMECFG_1: u32 = 0x01;
        }
    }
    #[doc = "Match Configuration"]
    pub mod MATCFG {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Match is disabled"]
            pub const MATCFG_0: u32 = 0;
            #[doc = "Match is enabled (1st data word equals MATCH0 OR MATCH1)"]
            pub const MATCFG_2: u32 = 0x02;
            #[doc = "Match is enabled (any data word equals MATCH0 OR MATCH1)"]
            pub const MATCFG_3: u32 = 0x03;
            #[doc = "Match is enabled (1st data word equals MATCH0 AND 2nd data word equals MATCH1)"]
            pub const MATCFG_4: u32 = 0x04;
            #[doc = "Match is enabled (any data word equals MATCH0 AND next data word equals MATCH1)"]
            pub const MATCFG_5: u32 = 0x05;
            #[doc = "Match is enabled (1st data word AND MATCH1 equals MATCH0 AND MATCH1)"]
            pub const MATCFG_6: u32 = 0x06;
            #[doc = "Match is enabled (any data word AND MATCH1 equals MATCH0 AND MATCH1)"]
            pub const MATCFG_7: u32 = 0x07;
        }
    }
    #[doc = "Pin Configuration"]
    pub mod PINCFG {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "2-pin open drain mode"]
            pub const PINCFG_0: u32 = 0;
            #[doc = "2-pin output only mode (ultra-fast mode)"]
            pub const PINCFG_1: u32 = 0x01;
            #[doc = "2-pin push-pull mode"]
            pub const PINCFG_2: u32 = 0x02;
            #[doc = "4-pin push-pull mode"]
            pub const PINCFG_3: u32 = 0x03;
            #[doc = "2-pin open drain mode with separate LPI2C slave"]
            pub const PINCFG_4: u32 = 0x04;
            #[doc = "2-pin output only mode (ultra-fast mode) with separate LPI2C slave"]
            pub const PINCFG_5: u32 = 0x05;
            #[doc = "2-pin push-pull mode with separate LPI2C slave"]
            pub const PINCFG_6: u32 = 0x06;
            #[doc = "4-pin push-pull mode (inverted outputs)"]
            pub const PINCFG_7: u32 = 0x07;
        }
    }
}
#[doc = "Master Configuration Register 2"]
pub mod MCFGR2 {
    #[doc = "Bus Idle Timeout"]
    pub mod BUSIDLE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Glitch Filter SCL"]
    pub mod FILTSCL {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Glitch Filter SDA"]
    pub mod FILTSDA {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Master Configuration Register 3"]
pub mod MCFGR3 {
    #[doc = "Pin Low Timeout"]
    pub mod PINLOW {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Master Data Match Register"]
pub mod MDMR {
    #[doc = "Match 0 Value"]
    pub mod MATCH0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Match 1 Value"]
    pub mod MATCH1 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Master Clock Configuration Register 0"]
pub mod MCCR0 {
    #[doc = "Clock Low Period"]
    pub mod CLKLO {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Clock High Period"]
    pub mod CLKHI {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Setup Hold Delay"]
    pub mod SETHOLD {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data Valid Delay"]
    pub mod DATAVD {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Master Clock Configuration Register 1"]
pub mod MCCR1 {
    #[doc = "Clock Low Period"]
    pub mod CLKLO {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Clock High Period"]
    pub mod CLKHI {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Setup Hold Delay"]
    pub mod SETHOLD {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data Valid Delay"]
    pub mod DATAVD {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Master FIFO Control Register"]
pub mod MFCR {
    #[doc = "Transmit FIFO Watermark"]
    pub mod TXWATER {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Receive FIFO Watermark"]
    pub mod RXWATER {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Master FIFO Status Register"]
pub mod MFSR {
    #[doc = "Transmit FIFO Count"]
    pub mod TXCOUNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Receive FIFO Count"]
    pub mod RXCOUNT {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Master Transmit Data Register"]
pub mod MTDR {
    #[doc = "Transmit Data"]
    pub mod DATA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Command Data"]
    pub mod CMD {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Transmit DATA\\[7:0\\]"]
            pub const CMD_0: u32 = 0;
            #[doc = "Receive (DATA\\[7:0\\] + 1) bytes"]
            pub const CMD_1: u32 = 0x01;
            #[doc = "Generate STOP condition"]
            pub const CMD_2: u32 = 0x02;
            #[doc = "Receive and discard (DATA\\[7:0\\] + 1) bytes"]
            pub const CMD_3: u32 = 0x03;
            #[doc = "Generate (repeated) START and transmit address in DATA\\[7:0\\]"]
            pub const CMD_4: u32 = 0x04;
            #[doc = "Generate (repeated) START and transmit address in DATA\\[7:0\\]. This transfer expects a NACK to be returned."]
            pub const CMD_5: u32 = 0x05;
            #[doc = "Generate (repeated) START and transmit address in DATA\\[7:0\\] using high speed mode"]
            pub const CMD_6: u32 = 0x06;
            #[doc = "Generate (repeated) START and transmit address in DATA\\[7:0\\] using high speed mode. This transfer expects a NACK to be returned."]
            pub const CMD_7: u32 = 0x07;
        }
    }
}
#[doc = "Master Receive Data Register"]
pub mod MRDR {
    #[doc = "Receive Data"]
    pub mod DATA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "RX Empty"]
    pub mod RXEMPTY {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Receive FIFO is not empty"]
            pub const RXEMPTY_0: u32 = 0;
            #[doc = "Receive FIFO is empty"]
            pub const RXEMPTY_1: u32 = 0x01;
        }
    }
}
#[doc = "Slave Control Register"]
pub mod SCR {
    #[doc = "Slave Enable"]
    pub mod SEN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "I2C Slave mode is disabled"]
            pub const SEN_0: u32 = 0;
            #[doc = "I2C Slave mode is enabled"]
            pub const SEN_1: u32 = 0x01;
        }
    }
    #[doc = "Software Reset"]
    pub mod RST {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Slave mode logic is not reset"]
            pub const RST_0: u32 = 0;
            #[doc = "Slave mode logic is reset"]
            pub const RST_1: u32 = 0x01;
        }
    }
    #[doc = "Filter Enable"]
    pub mod FILTEN {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable digital filter and output delay counter for slave mode"]
            pub const FILTEN_0: u32 = 0;
            #[doc = "Enable digital filter and output delay counter for slave mode"]
            pub const FILTEN_1: u32 = 0x01;
        }
    }
    #[doc = "Filter Doze Enable"]
    pub mod FILTDZ {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Filter remains enabled in Doze mode"]
            pub const FILTDZ_0: u32 = 0;
            #[doc = "Filter is disabled in Doze mode"]
            pub const FILTDZ_1: u32 = 0x01;
        }
    }
    #[doc = "Reset Transmit FIFO"]
    pub mod RTF {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No effect"]
            pub const RTF_0: u32 = 0;
            #[doc = "Transmit Data Register is now empty"]
            pub const RTF_1: u32 = 0x01;
        }
    }
    #[doc = "Reset Receive FIFO"]
    pub mod RRF {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No effect"]
            pub const RRF_0: u32 = 0;
            #[doc = "Receive Data Register is now empty"]
            pub const RRF_1: u32 = 0x01;
        }
    }
}
#[doc = "Slave Status Register"]
pub mod SSR {
    #[doc = "Transmit Data Flag"]
    pub mod TDF {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Transmit data not requested"]
            pub const TDF_0: u32 = 0;
            #[doc = "Transmit data is requested"]
            pub const TDF_1: u32 = 0x01;
        }
    }
    #[doc = "Receive Data Flag"]
    pub mod RDF {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Receive data is not ready"]
            pub const RDF_0: u32 = 0;
            #[doc = "Receive data is ready"]
            pub const RDF_1: u32 = 0x01;
        }
    }
    #[doc = "Address Valid Flag"]
    pub mod AVF {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Address Status Register is not valid"]
            pub const AVF_0: u32 = 0;
            #[doc = "Address Status Register is valid"]
            pub const AVF_1: u32 = 0x01;
        }
    }
    #[doc = "Transmit ACK Flag"]
    pub mod TAF {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Transmit ACK/NACK is not required"]
            pub const TAF_0: u32 = 0;
            #[doc = "Transmit ACK/NACK is required"]
            pub const TAF_1: u32 = 0x01;
        }
    }
    #[doc = "Repeated Start Flag"]
    pub mod RSF {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Slave has not detected a Repeated START condition"]
            pub const RSF_0: u32 = 0;
            #[doc = "Slave has detected a Repeated START condition"]
            pub const RSF_1: u32 = 0x01;
        }
    }
    #[doc = "STOP Detect Flag"]
    pub mod SDF {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Slave has not detected a STOP condition"]
            pub const SDF_0: u32 = 0;
            #[doc = "Slave has detected a STOP condition"]
            pub const SDF_1: u32 = 0x01;
        }
    }
    #[doc = "Bit Error Flag"]
    pub mod BEF {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Slave has not detected a bit error"]
            pub const BEF_0: u32 = 0;
            #[doc = "Slave has detected a bit error"]
            pub const BEF_1: u32 = 0x01;
        }
    }
    #[doc = "FIFO Error Flag"]
    pub mod FEF {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "FIFO underflow or overflow was not detected"]
            pub const FEF_0: u32 = 0;
            #[doc = "FIFO underflow or overflow was detected"]
            pub const FEF_1: u32 = 0x01;
        }
    }
    #[doc = "Address Match 0 Flag"]
    pub mod AM0F {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Have not received an ADDR0 matching address"]
            pub const AM0F_0: u32 = 0;
            #[doc = "Have received an ADDR0 matching address"]
            pub const AM0F_1: u32 = 0x01;
        }
    }
    #[doc = "Address Match 1 Flag"]
    pub mod AM1F {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Have not received an ADDR1 or ADDR0/ADDR1 range matching address"]
            pub const AM1F_0: u32 = 0;
            #[doc = "Have received an ADDR1 or ADDR0/ADDR1 range matching address"]
            pub const AM1F_1: u32 = 0x01;
        }
    }
    #[doc = "General Call Flag"]
    pub mod GCF {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Slave has not detected the General Call Address or the General Call Address is disabled"]
            pub const GCF_0: u32 = 0;
            #[doc = "Slave has detected the General Call Address"]
            pub const GCF_1: u32 = 0x01;
        }
    }
    #[doc = "SMBus Alert Response Flag"]
    pub mod SARF {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "SMBus Alert Response is disabled or not detected"]
            pub const SARF_0: u32 = 0;
            #[doc = "SMBus Alert Response is enabled and detected"]
            pub const SARF_1: u32 = 0x01;
        }
    }
    #[doc = "Slave Busy Flag"]
    pub mod SBF {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "I2C Slave is idle"]
            pub const SBF_0: u32 = 0;
            #[doc = "I2C Slave is busy"]
            pub const SBF_1: u32 = 0x01;
        }
    }
    #[doc = "Bus Busy Flag"]
    pub mod BBF {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "I2C Bus is idle"]
            pub const BBF_0: u32 = 0;
            #[doc = "I2C Bus is busy"]
            pub const BBF_1: u32 = 0x01;
        }
    }
}
#[doc = "Slave Interrupt Enable Register"]
pub mod SIER {
    #[doc = "Transmit Data Interrupt Enable"]
    pub mod TDIE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const TDIE_0: u32 = 0;
            #[doc = "Enabled"]
            pub const TDIE_1: u32 = 0x01;
        }
    }
    #[doc = "Receive Data Interrupt Enable"]
    pub mod RDIE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const RDIE_0: u32 = 0;
            #[doc = "Enabled"]
            pub const RDIE_1: u32 = 0x01;
        }
    }
    #[doc = "Address Valid Interrupt Enable"]
    pub mod AVIE {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const AVIE_0: u32 = 0;
            #[doc = "Enabled"]
            pub const AVIE_1: u32 = 0x01;
        }
    }
    #[doc = "Transmit ACK Interrupt Enable"]
    pub mod TAIE {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const TAIE_0: u32 = 0;
            #[doc = "Enabled"]
            pub const TAIE_1: u32 = 0x01;
        }
    }
    #[doc = "Repeated Start Interrupt Enable"]
    pub mod RSIE {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const RSIE_0: u32 = 0;
            #[doc = "Enabled"]
            pub const RSIE_1: u32 = 0x01;
        }
    }
    #[doc = "STOP Detect Interrupt Enable"]
    pub mod SDIE {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const SDIE_0: u32 = 0;
            #[doc = "Enabled"]
            pub const SDIE_1: u32 = 0x01;
        }
    }
    #[doc = "Bit Error Interrupt Enable"]
    pub mod BEIE {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const BEIE_0: u32 = 0;
            #[doc = "Enabled"]
            pub const BEIE_1: u32 = 0x01;
        }
    }
    #[doc = "FIFO Error Interrupt Enable"]
    pub mod FEIE {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const FEIE_0: u32 = 0;
            #[doc = "Enabled"]
            pub const FEIE_1: u32 = 0x01;
        }
    }
    #[doc = "Address Match 0 Interrupt Enable"]
    pub mod AM0IE {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Enabled"]
            pub const AM0IE_0: u32 = 0;
            #[doc = "Disabled"]
            pub const AM0IE_1: u32 = 0x01;
        }
    }
    #[doc = "Address Match 1 Interrupt Enable"]
    #[deprecated(since = "0.5.1", note = "Use AM1IE")]
    pub mod AM1F {
        pub use super::AM1IE::*;
    }
    #[doc = "Address Match 1 Interrupt Enable"]
    pub mod AM1IE {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const AM1F_0: u32 = 0;
            #[doc = "Enabled"]
            pub const AM1F_1: u32 = 0x01;
        }
    }
    #[doc = "General Call Interrupt Enable"]
    pub mod GCIE {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const GCIE_0: u32 = 0;
            #[doc = "Enabled"]
            pub const GCIE_1: u32 = 0x01;
        }
    }
    #[doc = "SMBus Alert Response Interrupt Enable"]
    pub mod SARIE {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const SARIE_0: u32 = 0;
            #[doc = "Enabled"]
            pub const SARIE_1: u32 = 0x01;
        }
    }
}
#[doc = "Slave DMA Enable Register"]
pub mod SDER {
    #[doc = "Transmit Data DMA Enable"]
    pub mod TDDE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "DMA request is disabled"]
            pub const TDDE_0: u32 = 0;
            #[doc = "DMA request is enabled"]
            pub const TDDE_1: u32 = 0x01;
        }
    }
    #[doc = "Receive Data DMA Enable"]
    pub mod RDDE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "DMA request is disabled"]
            pub const RDDE_0: u32 = 0;
            #[doc = "DMA request is enabled"]
            pub const RDDE_1: u32 = 0x01;
        }
    }
    #[doc = "Address Valid DMA Enable"]
    pub mod AVDE {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "DMA request is disabled"]
            pub const AVDE_0: u32 = 0;
            #[doc = "DMA request is enabled"]
            pub const AVDE_1: u32 = 0x01;
        }
    }
}
#[doc = "Slave Configuration Register 1"]
pub mod SCFGR1 {
    #[doc = "Address SCL Stall"]
    pub mod ADRSTALL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Clock stretching is disabled"]
            pub const ADRSTALL_0: u32 = 0;
            #[doc = "Clock stretching is enabled"]
            pub const ADRSTALL_1: u32 = 0x01;
        }
    }
    #[doc = "RX SCL Stall"]
    pub mod RXSTALL {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Clock stretching is disabled"]
            pub const RXSTALL_0: u32 = 0;
            #[doc = "Clock stretching is enabled"]
            pub const RXSTALL_1: u32 = 0x01;
        }
    }
    #[doc = "TX Data SCL Stall"]
    pub mod TXDSTALL {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Clock stretching is disabled"]
            pub const TXDSTALL_0: u32 = 0;
            #[doc = "Clock stretching is enabled"]
            pub const TXDSTALL_1: u32 = 0x01;
        }
    }
    #[doc = "ACK SCL Stall"]
    pub mod ACKSTALL {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Clock stretching is disabled"]
            pub const ACKSTALL_0: u32 = 0;
            #[doc = "Clock stretching is enabled"]
            pub const ACKSTALL_1: u32 = 0x01;
        }
    }
    #[doc = "General Call Enable"]
    pub mod GCEN {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "General Call address is disabled"]
            pub const GCEN_0: u32 = 0;
            #[doc = "General Call address is enabled"]
            pub const GCEN_1: u32 = 0x01;
        }
    }
    #[doc = "SMBus Alert Enable"]
    pub mod SAEN {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables match on SMBus Alert"]
            pub const SAEN_0: u32 = 0;
            #[doc = "Enables match on SMBus Alert"]
            pub const SAEN_1: u32 = 0x01;
        }
    }
    #[doc = "Transmit Flag Configuration"]
    pub mod TXCFG {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Transmit Data Flag will only assert during a slave-transmit transfer when the Transmit Data register is empty"]
            pub const TXCFG_0: u32 = 0;
            #[doc = "Transmit Data Flag will assert whenever the Transmit Data register is empty"]
            pub const TXCFG_1: u32 = 0x01;
        }
    }
    #[doc = "Receive Data Configuration"]
    pub mod RXCFG {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Reading the Receive Data register will return received data and clear the Receive Data flag (MSR\\[RDF\\])."]
            pub const RXCFG_0: u32 = 0;
            #[doc = "Reading the Receive Data register when the Address Valid flag (SSR\\[AVF\\])is set, will return the Address Status register and clear the Address Valid flag. Reading the Receive Data register when the Address Valid flag is clear, will return received data and clear the Receive Data flag (MSR\\[RDF\\])."]
            pub const RXCFG_1: u32 = 0x01;
        }
    }
    #[doc = "Ignore NACK"]
    pub mod IGNACK {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Slave will end transfer when NACK is detected"]
            pub const IGNACK_0: u32 = 0;
            #[doc = "Slave will not end transfer when NACK detected"]
            pub const IGNACK_1: u32 = 0x01;
        }
    }
    #[doc = "High Speed Mode Enable"]
    pub mod HSMEN {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables detection of HS-mode master code"]
            pub const HSMEN_0: u32 = 0;
            #[doc = "Enables detection of HS-mode master code"]
            pub const HSMEN_1: u32 = 0x01;
        }
    }
    #[doc = "Address Configuration"]
    pub mod ADDRCFG {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Address match 0 (7-bit)"]
            pub const ADDRCFG_0: u32 = 0;
            #[doc = "Address match 0 (10-bit)"]
            pub const ADDRCFG_1: u32 = 0x01;
            #[doc = "Address match 0 (7-bit) or Address match 1 (7-bit)"]
            pub const ADDRCFG_2: u32 = 0x02;
            #[doc = "Address match 0 (10-bit) or Address match 1 (10-bit)"]
            pub const ADDRCFG_3: u32 = 0x03;
            #[doc = "Address match 0 (7-bit) or Address match 1 (10-bit)"]
            pub const ADDRCFG_4: u32 = 0x04;
            #[doc = "Address match 0 (10-bit) or Address match 1 (7-bit)"]
            pub const ADDRCFG_5: u32 = 0x05;
            #[doc = "From Address match 0 (7-bit) to Address match 1 (7-bit)"]
            pub const ADDRCFG_6: u32 = 0x06;
            #[doc = "From Address match 0 (10-bit) to Address match 1 (10-bit)"]
            pub const ADDRCFG_7: u32 = 0x07;
        }
    }
}
#[doc = "Slave Configuration Register 2"]
pub mod SCFGR2 {
    #[doc = "Clock Hold Time"]
    pub mod CLKHOLD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data Valid Delay"]
    pub mod DATAVD {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Glitch Filter SCL"]
    pub mod FILTSCL {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Glitch Filter SDA"]
    pub mod FILTSDA {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Slave Address Match Register"]
pub mod SAMR {
    #[doc = "Address 0 Value"]
    pub mod ADDR0 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x03ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Address 1 Value"]
    pub mod ADDR1 {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x03ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Slave Address Status Register"]
pub mod SASR {
    #[doc = "Received Address"]
    pub mod RADDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Address Not Valid"]
    pub mod ANV {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Received Address (RADDR) is valid"]
            pub const ANV_0: u32 = 0;
            #[doc = "Received Address (RADDR) is not valid"]
            pub const ANV_1: u32 = 0x01;
        }
    }
}
#[doc = "Slave Transmit ACK Register"]
pub mod STAR {
    #[doc = "Transmit NACK"]
    pub mod TXNACK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Write a Transmit ACK for each received word"]
            pub const TXNACK_0: u32 = 0;
            #[doc = "Write a Transmit NACK for each received word"]
            pub const TXNACK_1: u32 = 0x01;
        }
    }
}
#[doc = "Slave Transmit Data Register"]
pub mod STDR {
    #[doc = "Transmit Data"]
    pub mod DATA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Slave Receive Data Register"]
pub mod SRDR {
    #[doc = "Receive Data"]
    pub mod DATA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "RX Empty"]
    pub mod RXEMPTY {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The Receive Data Register is not empty"]
            pub const RXEMPTY_0: u32 = 0;
            #[doc = "The Receive Data Register is empty"]
            pub const RXEMPTY_1: u32 = 0x01;
        }
    }
    #[doc = "Start Of Frame"]
    pub mod SOF {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Indicates this is not the first data word since a (repeated) START or STOP condition"]
            pub const SOF_0: u32 = 0;
            #[doc = "Indicates this is the first data word since a (repeated) START or STOP condition"]
            pub const SOF_1: u32 = 0x01;
        }
    }
}
