#[doc = "USBC"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "Identification"]
    pub ID: crate::RORegister<u32>,
    #[doc = "Hardware General"]
    pub HWGENERAL: crate::RORegister<u32>,
    #[doc = "Host Hardware Parameters"]
    pub HWHOST: crate::RORegister<u32>,
    #[doc = "Device Hardware Parameters"]
    pub HWDEVICE: crate::RORegister<u32>,
    #[doc = "TX Buffer Hardware Parameters"]
    pub HWTXBUF: crate::RORegister<u32>,
    #[doc = "RX Buffer Hardware Parameters"]
    pub HWRXBUF: crate::RORegister<u32>,
    _reserved0: [u8; 0x68],
    #[doc = "General Purpose Timer #0 Load"]
    pub GPTIMER0LD: crate::RWRegister<u32>,
    #[doc = "General Purpose Timer #0 Controller"]
    pub GPTIMER0CTRL: crate::RWRegister<u32>,
    #[doc = "General Purpose Timer #1 Load"]
    pub GPTIMER1LD: crate::RWRegister<u32>,
    #[doc = "General Purpose Timer #1 Controller"]
    pub GPTIMER1CTRL: crate::RWRegister<u32>,
    #[doc = "System Bus Config"]
    pub SBUSCFG: crate::RWRegister<u32>,
    _reserved1: [u8; 0x6c],
    #[doc = "Capability Registers Length"]
    pub CAPLENGTH: crate::RORegister<u8>,
    _reserved2: [u8; 0x01],
    #[doc = "Host Controller Interface Version"]
    pub HCIVERSION: crate::RORegister<u16>,
    #[doc = "Host Controller Structural Parameters"]
    pub HCSPARAMS: crate::RORegister<u32>,
    #[doc = "Host Controller Capability Parameters"]
    pub HCCPARAMS: crate::RORegister<u32>,
    _reserved3: [u8; 0x14],
    #[doc = "Device Controller Interface Version"]
    pub DCIVERSION: crate::RORegister<u16>,
    _reserved4: [u8; 0x02],
    #[doc = "Device Controller Capability Parameters"]
    pub DCCPARAMS: crate::RORegister<u32>,
    _reserved5: [u8; 0x18],
    #[doc = "USB Command"]
    pub USBCMD: crate::RWRegister<u32>,
    #[doc = "USB Status"]
    pub USBSTS: crate::RWRegister<u32>,
    #[doc = "Interrupt Enable"]
    pub USBINTR: crate::RWRegister<u32>,
    #[doc = "USB Frame Index"]
    pub FRINDEX: crate::RWRegister<u32>,
    _reserved6: [u8; 0x04],
    #[doc = "Device Address"]
    pub DEVICEADDR: crate::RWRegister<u32>,
    #[doc = "Next Asynch. Address"]
    pub ASYNCLISTADDR: crate::RWRegister<u32>,
    _reserved7: [u8; 0x04],
    #[doc = "Programmable Burst Size"]
    pub BURSTSIZE: crate::RWRegister<u32>,
    #[doc = "TX FIFO Fill Tuning"]
    pub TXFILLTUNING: crate::RWRegister<u32>,
    _reserved8: [u8; 0x10],
    #[doc = "Endpoint NAK"]
    pub ENDPTNAK: crate::RWRegister<u32>,
    #[doc = "Endpoint NAK Enable"]
    pub ENDPTNAKEN: crate::RWRegister<u32>,
    #[doc = "Configure Flag"]
    pub CONFIGFLAG: crate::RORegister<u32>,
    #[doc = "Port Status & Control"]
    pub PORTSC1: crate::RWRegister<u32>,
    _reserved9: [u8; 0x1c],
    #[doc = "On-The-Go Status & control"]
    pub OTGSC: crate::RWRegister<u32>,
    #[doc = "USB Device Mode"]
    pub USBMODE: crate::RWRegister<u32>,
    #[doc = "Endpoint Setup Status"]
    pub ENDPTSETUPSTAT: crate::RWRegister<u32>,
    #[doc = "Endpoint Prime"]
    pub ENDPTPRIME: crate::RWRegister<u32>,
    #[doc = "Endpoint Flush"]
    pub ENDPTFLUSH: crate::RWRegister<u32>,
    #[doc = "Endpoint Status"]
    pub ENDPTSTAT: crate::RORegister<u32>,
    #[doc = "Endpoint Complete"]
    pub ENDPTCOMPLETE: crate::RWRegister<u32>,
    #[doc = "Endpoint Control0"]
    pub ENDPTCTRL0: crate::RWRegister<u32>,
    #[doc = "Endpoint Control 1"]
    pub ENDPTCTRL1: crate::RWRegister<u32>,
    #[doc = "Endpoint Control 2"]
    pub ENDPTCTRL2: crate::RWRegister<u32>,
    #[doc = "Endpoint Control 3"]
    pub ENDPTCTRL3: crate::RWRegister<u32>,
    #[doc = "Endpoint Control 4"]
    pub ENDPTCTRL4: crate::RWRegister<u32>,
    #[doc = "Endpoint Control 5"]
    pub ENDPTCTRL5: crate::RWRegister<u32>,
    #[doc = "Endpoint Control 6"]
    pub ENDPTCTRL6: crate::RWRegister<u32>,
    #[doc = "Endpoint Control 7"]
    pub ENDPTCTRL7: crate::RWRegister<u32>,
}
#[doc = "Identification"]
pub mod ID {
    #[doc = "ID"]
    pub mod ID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "NID"]
    pub mod NID {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "REVISION"]
    pub mod REVISION {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Hardware General"]
pub mod HWGENERAL {
    #[doc = "PHYW"]
    pub mod PHYW {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "8 bit wide data bus (Software non-programmable)"]
            pub const DATA_BUS_8: u32 = 0;
            #[doc = "16 bit wide data bus (Software non-programmable)"]
            pub const DATA_BUS_16: u32 = 0x01;
            #[doc = "Reset to 8 bit wide data bus (Software programmable)"]
            pub const SW_RST_8: u32 = 0x02;
            #[doc = "Reset to 16 bit wide data bus (Software programmable)"]
            pub const SW_RST_16: u32 = 0x03;
        }
    }
    #[doc = "PHYM"]
    pub mod PHYM {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "UTMI/UMTI+"]
            pub const UTMI: u32 = 0;
            #[doc = "ULPI DDR"]
            pub const ULPI_DDR: u32 = 0x01;
            #[doc = "ULPI"]
            pub const ULPI: u32 = 0x02;
            #[doc = "Serial Only"]
            pub const SERIAL: u32 = 0x03;
            #[doc = "Software programmable - reset to UTMI/UTMI+"]
            pub const SW_RST_UTMI: u32 = 0x04;
            #[doc = "Software programmable - reset to ULPI DDR"]
            pub const SW_RST_ULPI_DDR: u32 = 0x05;
            #[doc = "Software programmable - reset to ULPI"]
            pub const SW_RST_ULPI: u32 = 0x06;
            #[doc = "Software programmable - reset to Serial"]
            pub const SW_RST_SERIAL: u32 = 0x07;
        }
    }
    #[doc = "SM"]
    pub mod SM {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No Serial Engine, always use parallel signalling."]
            pub const SERIAL_ENGINE_NO: u32 = 0;
            #[doc = "Serial Engine present, always use serial signalling for FS/LS."]
            pub const SERIAL_ENGINE_EN: u32 = 0x01;
            #[doc = "Software programmable - Reset to use parallel signalling for FS/LS"]
            pub const SW_RST_PARALLEL: u32 = 0x02;
            #[doc = "Software programmable - Reset to use serial signalling for FS/LS"]
            pub const SW_RST_SERIAL_ENG: u32 = 0x03;
        }
    }
}
#[doc = "Host Hardware Parameters"]
pub mod HWHOST {
    #[doc = "HC"]
    pub mod HC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not supported"]
            pub const HOST_OP_DIS: u32 = 0;
            #[doc = "Supported"]
            pub const HOST_OP_EN: u32 = 0x01;
        }
    }
    #[doc = "NPORT"]
    pub mod NPORT {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Device Hardware Parameters"]
pub mod HWDEVICE {
    #[doc = "DC"]
    pub mod DC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not supported"]
            pub const DEVICE_OP_DIS: u32 = 0;
            #[doc = "Supported"]
            pub const DEVICE_OP_EN: u32 = 0x01;
        }
    }
    #[doc = "DEVEP"]
    pub mod DEVEP {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "TX Buffer Hardware Parameters"]
pub mod HWTXBUF {
    #[doc = "TXBURST"]
    pub mod TXBURST {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TXCHANADD"]
    pub mod TXCHANADD {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "RX Buffer Hardware Parameters"]
pub mod HWRXBUF {
    #[doc = "RXBURST"]
    pub mod RXBURST {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "RXADD"]
    pub mod RXADD {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "General Purpose Timer #0 Load"]
pub mod GPTIMER0LD {
    #[doc = "GPTLD"]
    pub mod GPTLD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "General Purpose Timer #0 Controller"]
pub mod GPTIMER0CTRL {
    #[doc = "GPTCNT"]
    pub mod GPTCNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "GPTMODE"]
    pub mod GPTMODE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "One Shot Mode"]
            pub const ONE_SHOT: u32 = 0;
            #[doc = "Repeat Mode"]
            pub const REPEAT: u32 = 0x01;
        }
    }
    #[doc = "GPTRST"]
    pub mod GPTRST {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No action"]
            pub const NO_ACTION: u32 = 0;
            #[doc = "Load counter value from GPTLD bits in n_GPTIMER0LD"]
            pub const LOAD_CNTR: u32 = 0x01;
        }
    }
    #[doc = "GPTRUN"]
    pub mod GPTRUN {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Stop counting"]
            pub const STOP_CNTR: u32 = 0;
            #[doc = "Run"]
            pub const RUN: u32 = 0x01;
        }
    }
}
#[doc = "General Purpose Timer #1 Load"]
pub mod GPTIMER1LD {
    #[doc = "GPTLD"]
    pub mod GPTLD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "General Purpose Timer #1 Controller"]
pub mod GPTIMER1CTRL {
    #[doc = "GPTCNT"]
    pub mod GPTCNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "GPTMODE"]
    pub mod GPTMODE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "One Shot Mode"]
            pub const ONE_SHOT: u32 = 0;
            #[doc = "Repeat Mode"]
            pub const REPEAT: u32 = 0x01;
        }
    }
    #[doc = "GPTRST"]
    pub mod GPTRST {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No action"]
            pub const NO_ACTION: u32 = 0;
            #[doc = "Load counter value from GPTLD bits in USB_n_GPTIMER0LD"]
            pub const LOAD_CNTR: u32 = 0x01;
        }
    }
    #[doc = "GPTRUN"]
    pub mod GPTRUN {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Stop counting"]
            pub const STOP_CNTR: u32 = 0;
            #[doc = "Run"]
            pub const RUN: u32 = 0x01;
        }
    }
}
#[doc = "System Bus Config"]
pub mod SBUSCFG {
    #[doc = "AHBBRST"]
    pub mod AHBBRST {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Incremental burst of unspecified length only"]
            pub const INCR_BURST: u32 = 0;
            #[doc = "INCR4 burst, then single transfer"]
            pub const INCR4_BURST: u32 = 0x01;
            #[doc = "INCR8 burst, INCR4 burst, then single transfer"]
            pub const INCR8_BURST: u32 = 0x02;
            #[doc = "INCR16 burst, INCR8 burst, INCR4 burst, then single transfer"]
            pub const INCR16_BURST: u32 = 0x03;
            #[doc = "INCR4 burst, then incremental burst of unspecified length"]
            pub const INCR4_UNSPEC: u32 = 0x05;
            #[doc = "INCR8 burst, INCR4 burst, then incremental burst of unspecified length"]
            pub const INCR8_4_UNSPEC: u32 = 0x06;
            #[doc = "INCR16 burst, INCR8 burst, INCR4 burst, then incremental burst of unspecified length"]
            pub const INCR16_8_4_UNSPEC: u32 = 0x07;
        }
    }
}
#[doc = "Capability Registers Length"]
pub mod CAPLENGTH {
    #[doc = "CAPLENGTH"]
    pub mod CAPLENGTH {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Host Controller Interface Version"]
pub mod HCIVERSION {
    #[doc = "HCIVERSION"]
    pub mod HCIVERSION {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Host Controller Structural Parameters"]
pub mod HCSPARAMS {
    #[doc = "N_PORTS"]
    pub mod N_PORTS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "PPC"]
    pub mod PPC {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "N_PCC"]
    pub mod N_PCC {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "N_CC"]
    pub mod N_CC {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "There is no internal Companion Controller and port-ownership hand-off is not supported."]
            pub const NO_COMP_CONTROLLER: u32 = 0;
            #[doc = "There are internal companion controller(s) and port-ownership hand-offs is supported."]
            pub const COMP_CONTROLLER: u32 = 0x01;
        }
    }
    #[doc = "PI"]
    pub mod PI {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "N_PTT"]
    pub mod N_PTT {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "N_TT"]
    pub mod N_TT {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Host Controller Capability Parameters"]
pub mod HCCPARAMS {
    #[doc = "ADC"]
    pub mod ADC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "PFL"]
    pub mod PFL {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ASP"]
    pub mod ASP {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "IST"]
    pub mod IST {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "EECP"]
    pub mod EECP {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Device Controller Interface Version"]
pub mod DCIVERSION {
    #[doc = "DCIVERSION"]
    pub mod DCIVERSION {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Device Controller Capability Parameters"]
pub mod DCCPARAMS {
    #[doc = "DEN"]
    pub mod DEN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DC"]
    pub mod DC {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "HC"]
    pub mod HC {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "USB Command"]
pub mod USBCMD {
    #[doc = "RS"]
    pub mod RS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "RST"]
    pub mod RST {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "FS_1"]
    pub mod FS_1 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "PSE"]
    pub mod PSE {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Do not process the Periodic Schedule"]
            pub const DONT_PROCESS_PT: u32 = 0;
            #[doc = "Use the PERIODICLISTBASE register to access the Periodic Schedule."]
            pub const PROCESS_PT_PERIODICLISTBASE: u32 = 0x01;
        }
    }
    #[doc = "ASE"]
    pub mod ASE {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Do not process the Asynchronous Schedule."]
            pub const DONT_PROCESS_ASYNC: u32 = 0;
            #[doc = "Use the ASYNCLISTADDR register to access the Asynchronous Schedule."]
            pub const ACCESS_ASYNC: u32 = 0x01;
        }
    }
    #[doc = "IAA"]
    pub mod IAA {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ASP"]
    pub mod ASP {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ASPE"]
    pub mod ASPE {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "SUTW"]
    pub mod SUTW {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ATDTW"]
    pub mod ATDTW {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "FS_2"]
    pub mod FS_2 {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ITC"]
    pub mod ITC {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Immediate (no threshold)"]
            pub const IMMEDIATE: u32 = 0;
            #[doc = "1 micro-frame"]
            pub const MICROFRAME_1: u32 = 0x01;
            #[doc = "2 micro-frames"]
            pub const MICROFRAME_2: u32 = 0x02;
            #[doc = "4 micro-frames"]
            pub const MICROFRAME_4: u32 = 0x04;
            #[doc = "8 micro-frames"]
            pub const MICROFRAME_8: u32 = 0x08;
            #[doc = "16 micro-frames"]
            pub const MICROFRAME_16: u32 = 0x10;
            #[doc = "32 micro-frames"]
            pub const MICROFRAME_32: u32 = 0x20;
            #[doc = "64 micro-frames"]
            pub const MICROFRAME_64: u32 = 0x40;
        }
    }
}
#[doc = "USB Status"]
pub mod USBSTS {
    #[doc = "UI"]
    pub mod UI {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "UEI"]
    pub mod UEI {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "PCI"]
    pub mod PCI {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "FRI"]
    pub mod FRI {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "SEI"]
    pub mod SEI {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "AAI"]
    pub mod AAI {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "URI"]
    pub mod URI {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "SRI"]
    pub mod SRI {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "SLI"]
    pub mod SLI {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ULPII"]
    pub mod ULPII {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "HCH"]
    pub mod HCH {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "RCL"]
    pub mod RCL {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "PS"]
    pub mod PS {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "AS"]
    pub mod AS {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "NAKI"]
    pub mod NAKI {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TI0"]
    pub mod TI0 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TI1"]
    pub mod TI1 {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Interrupt Enable"]
pub mod USBINTR {
    #[doc = "UE"]
    pub mod UE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "UEE"]
    pub mod UEE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "PCE"]
    pub mod PCE {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "FRE"]
    pub mod FRE {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "SEE"]
    pub mod SEE {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "AAE"]
    pub mod AAE {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "URE"]
    pub mod URE {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "SRE"]
    pub mod SRE {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "SLE"]
    pub mod SLE {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ULPIE"]
    pub mod ULPIE {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "NAKE"]
    pub mod NAKE {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "UAIE"]
    pub mod UAIE {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "UPIE"]
    pub mod UPIE {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TIE0"]
    pub mod TIE0 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TIE1"]
    pub mod TIE1 {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "USB Frame Index"]
pub mod FRINDEX {
    #[doc = "FRINDEX"]
    pub mod FRINDEX {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "(1024) 12"]
            pub const FRINDEX_1024: u32 = 0;
            #[doc = "(512) 11"]
            pub const FRINDEX_512: u32 = 0x01;
            #[doc = "(256) 10"]
            pub const FRINDEX_256: u32 = 0x02;
            #[doc = "(128) 9"]
            pub const FRINDEX_128: u32 = 0x03;
            #[doc = "(64) 8"]
            pub const FRINDEX_64: u32 = 0x04;
            #[doc = "(32) 7"]
            pub const FRINDEX_32: u32 = 0x05;
            #[doc = "(16) 6"]
            pub const FRINDEX_16: u32 = 0x06;
            #[doc = "(8) 5"]
            pub const FRINDEX_8: u32 = 0x07;
        }
    }
}
#[doc = "Device Address"]
pub mod DEVICEADDR {
    #[doc = "USBADRA"]
    pub mod USBADRA {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "USBADR"]
    pub mod USBADR {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x7f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Next Asynch. Address"]
pub mod ASYNCLISTADDR {
    #[doc = "ASYBASE"]
    pub mod ASYBASE {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x07ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Programmable Burst Size"]
pub mod BURSTSIZE {
    #[doc = "RXPBURST"]
    pub mod RXPBURST {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TXPBURST"]
    pub mod TXPBURST {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "TX FIFO Fill Tuning"]
pub mod TXFILLTUNING {
    #[doc = "TXSCHOH"]
    pub mod TXSCHOH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TXSCHHEALTH"]
    pub mod TXSCHHEALTH {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TXFIFOTHRES"]
    pub mod TXFIFOTHRES {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Endpoint NAK"]
pub mod ENDPTNAK {
    #[doc = "EPRN"]
    pub mod EPRN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "EPTN"]
    pub mod EPTN {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Endpoint NAK Enable"]
pub mod ENDPTNAKEN {
    #[doc = "EPRNE"]
    pub mod EPRNE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "EPTNE"]
    pub mod EPTNE {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Configure Flag"]
pub mod CONFIGFLAG {
    #[doc = "CF"]
    pub mod CF {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Port routing control logic default-routes each port to an implementation dependent classic host controller."]
            pub const PORT_ROUTING_CLASSIC_HOST: u32 = 0;
            #[doc = "Port routing control logic default-routes all ports to this host controller."]
            pub const PORT_ROUTING_HOST: u32 = 0x01;
        }
    }
}
#[doc = "Port Status & Control"]
pub mod PORTSC1 {
    #[doc = "CCS"]
    pub mod CCS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CSC"]
    pub mod CSC {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "PE"]
    pub mod PE {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "PEC"]
    pub mod PEC {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "OCA"]
    pub mod OCA {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "This port does not have an over-current condition."]
            pub const NO_OVERCURRENT: u32 = 0;
            #[doc = "This port currently has an over-current condition"]
            pub const OVERCURRENT: u32 = 0x01;
        }
    }
    #[doc = "OCC"]
    pub mod OCC {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "FPR"]
    pub mod FPR {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "SUSP"]
    pub mod SUSP {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "PR"]
    pub mod PR {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "HSP"]
    pub mod HSP {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "LS"]
    pub mod LS {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "SE0"]
            pub const SE0: u32 = 0;
            #[doc = "K-state"]
            pub const K_STATE: u32 = 0x01;
            #[doc = "J-state"]
            pub const J_STATE: u32 = 0x02;
            #[doc = "Undefined"]
            pub const UNDEFINED: u32 = 0x03;
        }
    }
    #[doc = "PP"]
    pub mod PP {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "PO"]
    pub mod PO {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "PIC"]
    pub mod PIC {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Port indicators are off"]
            pub const PORT_INDICATOR_OFF: u32 = 0;
            #[doc = "Amber"]
            pub const PORT_IND_AMBER: u32 = 0x01;
            #[doc = "Green"]
            pub const PORT_IND_GREEN: u32 = 0x02;
            #[doc = "Undefined"]
            pub const UNDEFINED: u32 = 0x03;
        }
    }
    #[doc = "PTC"]
    pub mod PTC {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "TEST_MODE_DISABLE"]
            pub const TST_MODE_DIS: u32 = 0;
            #[doc = "J_STATE"]
            pub const J_STATE: u32 = 0x01;
            #[doc = "K_STATE"]
            pub const K_STATE: u32 = 0x02;
            #[doc = "SE0 (host) / NAK (device)"]
            pub const SE0: u32 = 0x03;
            #[doc = "Packet"]
            pub const PCKT: u32 = 0x04;
            #[doc = "FORCE_ENABLE_HS"]
            pub const HS: u32 = 0x05;
            #[doc = "FORCE_ENABLE_FS"]
            pub const FS: u32 = 0x06;
            #[doc = "FORCE_ENABLE_LS"]
            pub const LS: u32 = 0x07;
        }
    }
    #[doc = "WKCN"]
    pub mod WKCN {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "WKDC"]
    pub mod WKDC {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "WKOC"]
    pub mod WKOC {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "PHCD"]
    pub mod PHCD {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Enable PHY clock"]
            pub const PHY_CLK_EN: u32 = 0;
            #[doc = "Disable PHY clock"]
            pub const PHY_CLK_DIS: u32 = 0x01;
        }
    }
    #[doc = "PFSC"]
    pub mod PFSC {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Normal operation"]
            pub const NORMAL: u32 = 0;
            #[doc = "Forced to full speed"]
            pub const FULL_SPEED: u32 = 0x01;
        }
    }
    #[doc = "PTS_2"]
    pub mod PTS_2 {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "PSPD"]
    pub mod PSPD {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Full Speed"]
            pub const FS: u32 = 0;
            #[doc = "Low Speed"]
            pub const LS: u32 = 0x01;
            #[doc = "High Speed"]
            pub const HS: u32 = 0x02;
            #[doc = "Undefined"]
            pub const UNDEFINED: u32 = 0x03;
        }
    }
    #[doc = "PTW"]
    pub mod PTW {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Select the 8-bit UTMI interface \\[60MHz\\]"]
            pub const UTMI_8: u32 = 0;
            #[doc = "Select the 16-bit UTMI interface \\[30MHz\\]"]
            pub const UTMI_16: u32 = 0x01;
        }
    }
    #[doc = "STS"]
    pub mod STS {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "PTS_1"]
    pub mod PTS_1 {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "On-The-Go Status & control"]
pub mod OTGSC {
    #[doc = "VD"]
    pub mod VD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "VC"]
    pub mod VC {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "OT"]
    pub mod OT {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DP"]
    pub mod DP {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "IDPU"]
    pub mod IDPU {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ID"]
    pub mod ID {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "AVV"]
    pub mod AVV {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ASV"]
    pub mod ASV {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "BSV"]
    pub mod BSV {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "BSE"]
    pub mod BSE {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TOG_1MS"]
    pub mod TOG_1MS {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DPS"]
    pub mod DPS {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "IDIS"]
    pub mod IDIS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "AVVIS"]
    pub mod AVVIS {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ASVIS"]
    pub mod ASVIS {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "BSVIS"]
    pub mod BSVIS {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "BSEIS"]
    pub mod BSEIS {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "STATUS_1MS"]
    pub mod STATUS_1MS {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DPIS"]
    pub mod DPIS {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "IDIE"]
    pub mod IDIE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "AVVIE"]
    pub mod AVVIE {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ASVIE"]
    pub mod ASVIE {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "BSVIE"]
    pub mod BSVIE {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "BSEIE"]
    pub mod BSEIE {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "EN_1MS"]
    pub mod EN_1MS {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DPIE"]
    pub mod DPIE {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "USB Device Mode"]
pub mod USBMODE {
    #[doc = "CM"]
    pub mod CM {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Idle \\[Default for combination host/device\\]"]
            pub const IDL: u32 = 0;
            #[doc = "Device Controller \\[Default for device only controller\\]"]
            pub const DEVICE_CONTR: u32 = 0x02;
            #[doc = "Host Controller \\[Default for host only controller\\]"]
            pub const HOST_CONTR: u32 = 0x03;
        }
    }
    #[doc = "ES"]
    pub mod ES {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Little Endian \\[Default\\]"]
            pub const LITTLE_ENDIAN: u32 = 0;
            #[doc = "Big Endian"]
            pub const BIG_ENDIAN: u32 = 0x01;
        }
    }
    #[doc = "SLOM"]
    pub mod SLOM {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Setup Lockouts On (default);"]
            pub const LOCKOUT_ON: u32 = 0;
            #[doc = "Setup Lockouts Off"]
            pub const LOCKOUT_OFF: u32 = 0x01;
        }
    }
    #[doc = "SDIS"]
    pub mod SDIS {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Endpoint Setup Status"]
pub mod ENDPTSETUPSTAT {
    #[doc = "ENDPTSETUPSTAT"]
    pub mod ENDPTSETUPSTAT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Endpoint Prime"]
pub mod ENDPTPRIME {
    #[doc = "PERB"]
    pub mod PERB {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "PETB"]
    pub mod PETB {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Endpoint Flush"]
pub mod ENDPTFLUSH {
    #[doc = "FERB"]
    pub mod FERB {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "FETB"]
    pub mod FETB {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Endpoint Status"]
pub mod ENDPTSTAT {
    #[doc = "ERBR"]
    pub mod ERBR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ETBR"]
    pub mod ETBR {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Endpoint Complete"]
pub mod ENDPTCOMPLETE {
    #[doc = "ERCE"]
    pub mod ERCE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ETCE"]
    pub mod ETCE {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Endpoint Control0"]
pub mod ENDPTCTRL0 {
    #[doc = "RXS"]
    pub mod RXS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "RXT"]
    pub mod RXT {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "RXE"]
    pub mod RXE {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TXS"]
    pub mod TXS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TXT"]
    pub mod TXT {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TXE"]
    pub mod TXE {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Endpoint Control 1"]
pub mod ENDPTCTRL1 {
    #[doc = "RXS"]
    pub mod RXS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "RXD"]
    pub mod RXD {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "RXT"]
    pub mod RXT {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "RXI"]
    pub mod RXI {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "RXR"]
    pub mod RXR {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "RXE"]
    pub mod RXE {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TXS"]
    pub mod TXS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TXD"]
    pub mod TXD {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TXT"]
    pub mod TXT {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TXI"]
    pub mod TXI {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TXR"]
    pub mod TXR {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TXE"]
    pub mod TXE {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Endpoint Control 2"]
pub mod ENDPTCTRL2 {
    #[doc = "RXS"]
    pub mod RXS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "RXD"]
    pub mod RXD {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "RXT"]
    pub mod RXT {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "RXI"]
    pub mod RXI {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "RXR"]
    pub mod RXR {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "RXE"]
    pub mod RXE {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TXS"]
    pub mod TXS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TXD"]
    pub mod TXD {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TXT"]
    pub mod TXT {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TXI"]
    pub mod TXI {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TXR"]
    pub mod TXR {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TXE"]
    pub mod TXE {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Endpoint Control 3"]
pub mod ENDPTCTRL3 {
    #[doc = "RXS"]
    pub mod RXS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "RXD"]
    pub mod RXD {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "RXT"]
    pub mod RXT {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "RXI"]
    pub mod RXI {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "RXR"]
    pub mod RXR {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "RXE"]
    pub mod RXE {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TXS"]
    pub mod TXS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TXD"]
    pub mod TXD {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TXT"]
    pub mod TXT {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TXI"]
    pub mod TXI {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TXR"]
    pub mod TXR {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TXE"]
    pub mod TXE {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Endpoint Control 4"]
pub mod ENDPTCTRL4 {
    #[doc = "RXS"]
    pub mod RXS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "RXD"]
    pub mod RXD {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "RXT"]
    pub mod RXT {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "RXI"]
    pub mod RXI {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "RXR"]
    pub mod RXR {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "RXE"]
    pub mod RXE {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TXS"]
    pub mod TXS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TXD"]
    pub mod TXD {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TXT"]
    pub mod TXT {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TXI"]
    pub mod TXI {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TXR"]
    pub mod TXR {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TXE"]
    pub mod TXE {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Endpoint Control 5"]
pub mod ENDPTCTRL5 {
    #[doc = "RXS"]
    pub mod RXS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "RXD"]
    pub mod RXD {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "RXT"]
    pub mod RXT {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "RXI"]
    pub mod RXI {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "RXR"]
    pub mod RXR {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "RXE"]
    pub mod RXE {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TXS"]
    pub mod TXS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TXD"]
    pub mod TXD {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TXT"]
    pub mod TXT {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TXI"]
    pub mod TXI {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TXR"]
    pub mod TXR {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TXE"]
    pub mod TXE {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Endpoint Control 6"]
pub mod ENDPTCTRL6 {
    #[doc = "RXS"]
    pub mod RXS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "RXD"]
    pub mod RXD {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "RXT"]
    pub mod RXT {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "RXI"]
    pub mod RXI {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "RXR"]
    pub mod RXR {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "RXE"]
    pub mod RXE {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TXS"]
    pub mod TXS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TXD"]
    pub mod TXD {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TXT"]
    pub mod TXT {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TXI"]
    pub mod TXI {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TXR"]
    pub mod TXR {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TXE"]
    pub mod TXE {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Endpoint Control 7"]
pub mod ENDPTCTRL7 {
    #[doc = "RXS"]
    pub mod RXS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "RXD"]
    pub mod RXD {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "RXT"]
    pub mod RXT {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "RXI"]
    pub mod RXI {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "RXR"]
    pub mod RXR {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "RXE"]
    pub mod RXE {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TXS"]
    pub mod TXS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TXD"]
    pub mod TXD {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TXT"]
    pub mod TXT {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TXI"]
    pub mod TXI {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TXR"]
    pub mod TXR {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TXE"]
    pub mod TXE {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
