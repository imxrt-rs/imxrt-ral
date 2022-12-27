#[doc = "USB"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "Identification register"]
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
    #[doc = "USB Command Register"]
    pub USBCMD: crate::RWRegister<u32>,
    #[doc = "USB Status Register"]
    pub USBSTS: crate::RWRegister<u32>,
    #[doc = "Interrupt Enable Register"]
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
    #[doc = "Configure Flag Register"]
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
    pub ENDPTSTAT: crate::RWRegister<u32>,
    #[doc = "Endpoint Complete"]
    pub ENDPTCOMPLETE: crate::RWRegister<u32>,
    #[doc = "Endpoint Control0"]
    pub ENDPTCTRL0: crate::RWRegister<u32>,
    #[doc = "Endpoint Control"]
    pub ENDPTCTRL: [crate::RWRegister<u32>; 7usize],
}
#[doc = "Identification register"]
pub mod ID {
    #[doc = "Configuration number"]
    pub mod ID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Complement version of ID"]
    pub mod NID {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Revision number of the controller core."]
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
    #[doc = "Data width of the transciever connected to the controller core. PHYW bit reset value is"]
    pub mod PHYW {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "8 bit wide data bus Software non-programmable"]
            pub const PHYW_0: u32 = 0;
            #[doc = "16 bit wide data bus Software non-programmable"]
            pub const PHYW_1: u32 = 0x01;
            #[doc = "Reset to 8 bit wide data bus Software programmable"]
            pub const PHYW_2: u32 = 0x02;
            #[doc = "Reset to 16 bit wide data bus Software programmable"]
            pub const PHYW_3: u32 = 0x03;
        }
    }
    #[doc = "Transciever type"]
    pub mod PHYM {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "UTMI/UMTI+"]
            pub const PHYM_0: u32 = 0;
            #[doc = "ULPI DDR"]
            pub const PHYM_1: u32 = 0x01;
            #[doc = "ULPI"]
            pub const PHYM_2: u32 = 0x02;
            #[doc = "Serial Only"]
            pub const PHYM_3: u32 = 0x03;
            #[doc = "Software programmable - reset to UTMI/UTMI+"]
            pub const PHYM_4: u32 = 0x04;
            #[doc = "Software programmable - reset to ULPI DDR"]
            pub const PHYM_5: u32 = 0x05;
            #[doc = "Software programmable - reset to ULPI"]
            pub const PHYM_6: u32 = 0x06;
            #[doc = "Software programmable - reset to Serial"]
            pub const PHYM_7: u32 = 0x07;
        }
    }
    #[doc = "Serial interface mode capability"]
    pub mod SM {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No Serial Engine, always use parallel signalling."]
            pub const SM_0: u32 = 0;
            #[doc = "Serial Engine present, always use serial signalling for FS/LS."]
            pub const SM_1: u32 = 0x01;
            #[doc = "Software programmable - Reset to use parallel signalling for FS/LS"]
            pub const SM_2: u32 = 0x02;
            #[doc = "Software programmable - Reset to use serial signalling for FS/LS"]
            pub const SM_3: u32 = 0x03;
        }
    }
}
#[doc = "Host Hardware Parameters"]
pub mod HWHOST {
    #[doc = "Host Capable. Indicating whether host operation mode is supported or not."]
    pub mod HC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not supported"]
            pub const HC_0: u32 = 0;
            #[doc = "Supported"]
            pub const HC_1: u32 = 0x01;
        }
    }
    #[doc = "The Nmber of downstream ports supported by the host controller is (NPORT+1)"]
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
    #[doc = "Device Capable. Indicating whether device operation mode is supported or not."]
    pub mod DC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not supported"]
            pub const DC_0: u32 = 0;
            #[doc = "Supported"]
            pub const DC_1: u32 = 0x01;
        }
    }
    #[doc = "Device Endpoint Number"]
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
    #[doc = "Default burst size for memory to TX buffer transfer"]
    pub mod TXBURST {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TX FIFO Buffer size is: (2^TXCHANADD) * 4 Bytes"]
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
    #[doc = "Default burst size for memory to RX buffer transfer"]
    pub mod RXBURST {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Buffer total size for all receive endpoints is (2^RXADD)"]
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
    #[doc = "General Purpose Timer Load Value These bit fields are loaded to GPTCNT bits when GPTRST bit is set '1b'"]
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
    #[doc = "General Purpose Timer Counter. This field is the count value of the countdown timer."]
    pub mod GPTCNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "General Purpose Timer Mode In one shot mode, the timer will count down to zero, generate an interrupt, and stop until the counter is reset by software; In repeat mode, the timer will count down to zero, generate an interrupt and automatically reload the counter value from GPTLD bits to start again"]
    pub mod GPTMODE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "One Shot Mode"]
            pub const GPTMODE_0: u32 = 0;
            #[doc = "Repeat Mode"]
            pub const GPTMODE_1: u32 = 0x01;
        }
    }
    #[doc = "General Purpose Timer Reset"]
    pub mod GPTRST {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No action"]
            pub const GPTRST_0: u32 = 0;
            #[doc = "Load counter value from GPTLD bits in n_GPTIMER0LD"]
            pub const GPTRST_1: u32 = 0x01;
        }
    }
    #[doc = "General Purpose Timer Run GPTCNT bits are not effected when setting or clearing this bit."]
    pub mod GPTRUN {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Stop counting"]
            pub const GPTRUN_0: u32 = 0;
            #[doc = "Run"]
            pub const GPTRUN_1: u32 = 0x01;
        }
    }
}
#[doc = "General Purpose Timer #1 Load"]
pub mod GPTIMER1LD {
    #[doc = "General Purpose Timer Load Value These bit fields are loaded to GPTCNT bits when GPTRST bit is set '1b'"]
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
    #[doc = "General Purpose Timer Counter. This field is the count value of the countdown timer."]
    pub mod GPTCNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "General Purpose Timer Mode In one shot mode, the timer will count down to zero, generate an interrupt, and stop until the counter is reset by software"]
    pub mod GPTMODE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "One Shot Mode"]
            pub const GPTMODE_0: u32 = 0;
            #[doc = "Repeat Mode"]
            pub const GPTMODE_1: u32 = 0x01;
        }
    }
    #[doc = "General Purpose Timer Reset"]
    pub mod GPTRST {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No action"]
            pub const GPTRST_0: u32 = 0;
            #[doc = "Load counter value from GPTLD bits in USB_n_GPTIMER0LD"]
            pub const GPTRST_1: u32 = 0x01;
        }
    }
    #[doc = "General Purpose Timer Run GPTCNT bits are not effected when setting or clearing this bit."]
    pub mod GPTRUN {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Stop counting"]
            pub const GPTRUN_0: u32 = 0;
            #[doc = "Run"]
            pub const GPTRUN_1: u32 = 0x01;
        }
    }
}
#[doc = "System Bus Config"]
pub mod SBUSCFG {
    #[doc = "AHB master interface Burst configuration These bits control AHB master transfer type sequence (or priority)"]
    pub mod AHBBRST {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Incremental burst of unspecified length only"]
            pub const AHBBRST_0: u32 = 0;
            #[doc = "INCR4 burst, then single transfer"]
            pub const AHBBRST_1: u32 = 0x01;
            #[doc = "INCR8 burst, INCR4 burst, then single transfer"]
            pub const AHBBRST_2: u32 = 0x02;
            #[doc = "INCR16 burst, INCR8 burst, INCR4 burst, then single transfer"]
            pub const AHBBRST_3: u32 = 0x03;
            #[doc = "INCR4 burst, then incremental burst of unspecified length"]
            pub const AHBBRST_5: u32 = 0x05;
            #[doc = "INCR8 burst, INCR4 burst, then incremental burst of unspecified length"]
            pub const AHBBRST_6: u32 = 0x06;
            #[doc = "INCR16 burst, INCR8 burst, INCR4 burst, then incremental burst of unspecified length"]
            pub const AHBBRST_7: u32 = 0x07;
        }
    }
}
#[doc = "Capability Registers Length"]
pub mod CAPLENGTH {
    #[doc = "These bits are used as an offset to add to register base to find the beginning of the Operational Register"]
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
    #[doc = "Host Controller Interface Version Number Default value is '10h', which means EHCI rev1.0."]
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
    #[doc = "Number of downstream ports"]
    pub mod N_PORTS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Port Power Control This field indicates whether the host controller implementation includes port power control"]
    pub mod PPC {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Number of Ports per Companion Controller This field indicates the number of ports supported per internal Companion Controller"]
    pub mod N_PCC {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Number of Companion Controller (N_CC)"]
    pub mod N_CC {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "There is no internal Companion Controller and port-ownership hand-off is not supported."]
            pub const N_CC_0: u32 = 0;
            #[doc = "There are internal companion controller(s) and port-ownership hand-offs is supported."]
            pub const N_CC_1: u32 = 0x01;
        }
    }
    #[doc = "Port Indicators (P INDICATOR) This bit indicates whether the ports support port indicator control"]
    pub mod PI {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Number of Ports per Transaction Translator (N_PTT)"]
    pub mod N_PTT {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Number of Transaction Translators (N_TT)"]
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
    #[doc = "64-bit Addressing Capability This bit is set '0b' in all controller core, no 64-bit addressing capability is supported"]
    pub mod ADC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Programmable Frame List Flag If this bit is set to zero, then the system software must use a frame list length of 1024 elements with this host controller"]
    pub mod PFL {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Asynchronous Schedule Park Capability If this bit is set to a one, then the host controller supports the park feature for high-speed queue heads in the Asynchronous Schedule"]
    pub mod ASP {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Isochronous Scheduling Threshold"]
    pub mod IST {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "EHCI Extended Capabilities Pointer"]
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
    #[doc = "Device Controller Interface Version Number Default value is '01h', which means rev0.1."]
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
    #[doc = "Device Endpoint Number This field indicates the number of endpoints built into the device controller"]
    pub mod DEN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Device Capable When this bit is 1, this controller is capable of operating as a USB 2.0 device."]
    pub mod DC {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Host Capable When this bit is 1, this controller is capable of operating as an EHCI compatible USB 2"]
    pub mod HC {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "USB Command Register"]
pub mod USBCMD {
    #[doc = "Run/Stop (RS) - Read/Write"]
    pub mod RS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Controller Reset (RESET) - Read/Write"]
    pub mod RST {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "See description at bit 15"]
    pub mod FS_1 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Periodic Schedule Enable- Read/Write"]
    pub mod PSE {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Do not process the Periodic Schedule"]
            pub const PSE_0: u32 = 0;
            #[doc = "Use the PERIODICLISTBASE register to access the Periodic Schedule."]
            pub const PSE_1: u32 = 0x01;
        }
    }
    #[doc = "Asynchronous Schedule Enable - Read/Write"]
    pub mod ASE {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Do not process the Asynchronous Schedule."]
            pub const ASE_0: u32 = 0;
            #[doc = "Use the ASYNCLISTADDR register to access the Asynchronous Schedule."]
            pub const ASE_1: u32 = 0x01;
        }
    }
    #[doc = "Interrupt on Async Advance Doorbell - Read/Write"]
    pub mod IAA {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Asynchronous Schedule Park Mode Count - Read/Write"]
    pub mod ASP {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Asynchronous Schedule Park Mode Enable - Read/Write"]
    pub mod ASPE {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Setup TripWire - Read/Write"]
    pub mod SUTW {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Add dTD TripWire - Read/Write"]
    pub mod ATDTW {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Frame List Size - (Read/Write or Read Only)"]
    pub mod FS_2 {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Interrupt Threshold Control -Read/Write"]
    pub mod ITC {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Immediate (no threshold)"]
            pub const ITC_0: u32 = 0;
            #[doc = "1 micro-frame"]
            pub const ITC_1: u32 = 0x01;
            #[doc = "2 micro-frames"]
            pub const ITC_2: u32 = 0x02;
            #[doc = "4 micro-frames"]
            pub const ITC_4: u32 = 0x04;
            #[doc = "8 micro-frames"]
            pub const ITC_8: u32 = 0x08;
            #[doc = "16 micro-frames"]
            pub const ITC_16: u32 = 0x10;
            #[doc = "32 micro-frames"]
            pub const ITC_32: u32 = 0x20;
            #[doc = "64 micro-frames"]
            pub const ITC_64: u32 = 0x40;
        }
    }
}
#[doc = "USB Status Register"]
pub mod USBSTS {
    #[doc = "USB Interrupt (USBINT) - R/WC"]
    pub mod UI {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "USB Error Interrupt (USBERRINT) - R/WC"]
    pub mod UEI {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Port Change Detect - R/WC"]
    pub mod PCI {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Frame List Rollover - R/WC"]
    pub mod FRI {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "System Error- R/WC"]
    pub mod SEI {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Interrupt on Async Advance - R/WC"]
    pub mod AAI {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "USB Reset Received - R/WC"]
    pub mod URI {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "SOF Received - R/WC"]
    pub mod SRI {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DCSuspend - R/WC"]
    pub mod SLI {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ULPI Interrupt - R/WC"]
    pub mod ULPII {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "HCHaIted - Read Only"]
    pub mod HCH {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Reclamation - Read Only"]
    pub mod RCL {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Periodic Schedule Status - Read Only"]
    pub mod PS {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Asynchronous Schedule Status - Read Only"]
    pub mod AS {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "NAK Interrupt Bit--RO"]
    pub mod NAKI {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "General Purpose Timer Interrupt 0(GPTINT0)--R/WC"]
    pub mod TI0 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "General Purpose Timer Interrupt 1(GPTINT1)--R/WC"]
    pub mod TI1 {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Interrupt Enable Register"]
pub mod USBINTR {
    #[doc = "USB Interrupt Enable When this bit is one and the UI bit in n_USBSTS register is a one the controller will issue an interrupt"]
    pub mod UE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "USB Error Interrupt Enable When this bit is one and the UEI bit in n_USBSTS register is a one the controller will issue an interrupt"]
    pub mod UEE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Port Change Detect Interrupt Enable When this bit is one and the PCI bit in n_USBSTS register is a one the controller will issue an interrupt"]
    pub mod PCE {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Frame List Rollover Interrupt Enable When this bit is one and the FRI bit in n_USBSTS register is a one the controller will issue an interrupt"]
    pub mod FRE {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "System Error Interrupt Enable When this bit is one and the SEI bit in n_USBSTS register is a one the controller will issue an interrupt"]
    pub mod SEE {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Async Advance Interrupt Enable When this bit is one and the AAI bit in n_USBSTS register is a one the controller will issue an interrupt"]
    pub mod AAE {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "USB Reset Interrupt Enable When this bit is one and the URI bit in n_USBSTS register is a one the controller will issue an interrupt"]
    pub mod URE {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "SOF Received Interrupt Enable When this bit is one and the SRI bit in n_USBSTS register is a one the controller will issue an interrupt"]
    pub mod SRE {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Sleep Interrupt Enable When this bit is one and the SLI bit in n_n_USBSTS register is a one the controller will issue an interrupt"]
    pub mod SLE {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ULPI Interrupt Enable When this bit is one and the UPLII bit in n_USBSTS register is a one the controller will issue an interrupt"]
    pub mod ULPIE {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "NAK Interrupt Enable When this bit is one and the NAKI bit in n_USBSTS register is a one the controller will issue an interrupt"]
    pub mod NAKE {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "USB Host Asynchronous Interrupt Enable When this bit is one, and the UAI bit in the n_USBSTS register is one, host controller will issue an interrupt at the next interrupt threshold"]
    pub mod UAIE {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "USB Host Periodic Interrupt Enable When this bit is one, and the UPI bit in the n_USBSTS register is one, host controller will issue an interrupt at the next interrupt threshold"]
    pub mod UPIE {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "General Purpose Timer #0 Interrupt Enable When this bit is one and the TI0 bit in n_USBSTS register is a one the controller will issue an interrupt"]
    pub mod TIE0 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "General Purpose Timer #1 Interrupt Enable When this bit is one and the TI1 bit in n_USBSTS register is a one the controller will issue an interrupt"]
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
    #[doc = "Frame Index"]
    pub mod FRINDEX {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "(1024) 12"]
            pub const FRINDEX_0: u32 = 0;
            #[doc = "(512) 11"]
            pub const FRINDEX_1: u32 = 0x01;
            #[doc = "(256) 10"]
            pub const FRINDEX_2: u32 = 0x02;
            #[doc = "(128) 9"]
            pub const FRINDEX_3: u32 = 0x03;
            #[doc = "(64) 8"]
            pub const FRINDEX_4: u32 = 0x04;
            #[doc = "(32) 7"]
            pub const FRINDEX_5: u32 = 0x05;
            #[doc = "(16) 6"]
            pub const FRINDEX_6: u32 = 0x06;
            #[doc = "(8) 5"]
            pub const FRINDEX_7: u32 = 0x07;
        }
    }
}
#[doc = "Device Address"]
pub mod DEVICEADDR {
    #[doc = "Device Address Advance"]
    pub mod USBADRA {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Device Address. These bits correspond to the USB device address"]
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
    #[doc = "Link Pointer Low (LPL)"]
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
    #[doc = "Programmable RX Burst Size"]
    pub mod RXPBURST {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Programmable TX Burst Size"]
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
    #[doc = "Scheduler Overhead"]
    pub mod TXSCHOH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Scheduler Health Counter"]
    pub mod TXSCHHEALTH {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "FIFO Burst Threshold"]
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
    #[doc = "RX Endpoint NAK - R/WC"]
    pub mod EPRN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TX Endpoint NAK - R/WC"]
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
    #[doc = "RX Endpoint NAK Enable - R/W"]
    pub mod EPRNE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TX Endpoint NAK Enable - R/W"]
    pub mod EPTNE {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Configure Flag Register"]
pub mod CONFIGFLAG {
    #[doc = "Configure Flag Host software sets this bit as the last action in its process of configuring the Host Controller"]
    pub mod CF {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Port routing control logic default-routes each port to an implementation dependent classic host controller."]
            pub const CF_0: u32 = 0;
            #[doc = "Port routing control logic default-routes all ports to this host controller."]
            pub const CF_1: u32 = 0x01;
        }
    }
}
#[doc = "Port Status & Control"]
pub mod PORTSC1 {
    #[doc = "Current Connect Status-Read Only"]
    pub mod CCS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Connect Status Change-R/WC"]
    pub mod CSC {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Port Enabled/Disabled-Read/Write"]
    pub mod PE {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Port Enable/Disable Change-R/WC"]
    pub mod PEC {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Over-current Active-Read Only"]
    pub mod OCA {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "This port does not have an over-current condition."]
            pub const OCA_0: u32 = 0;
            #[doc = "This port currently has an over-current condition"]
            pub const OCA_1: u32 = 0x01;
        }
    }
    #[doc = "Over-current Change-R/WC"]
    pub mod OCC {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Force Port Resume -Read/Write"]
    pub mod FPR {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Suspend - Read/Write or Read Only"]
    pub mod SUSP {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Port Reset - Read/Write or Read Only"]
    pub mod PR {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "High-Speed Port - Read Only"]
    pub mod HSP {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Line Status-Read Only"]
    pub mod LS {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "SE0"]
            pub const LS_0: u32 = 0;
            #[doc = "K-state"]
            pub const LS_1: u32 = 0x01;
            #[doc = "J-state"]
            pub const LS_2: u32 = 0x02;
            #[doc = "Undefined"]
            pub const LS_3: u32 = 0x03;
        }
    }
    #[doc = "Port Power (PP)-Read/Write or Read Only"]
    pub mod PP {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Port Owner-Read/Write"]
    pub mod PO {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Port Indicator Control - Read/Write"]
    pub mod PIC {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Port indicators are off"]
            pub const PIC_0: u32 = 0;
            #[doc = "Amber"]
            pub const PIC_1: u32 = 0x01;
            #[doc = "Green"]
            pub const PIC_2: u32 = 0x02;
            #[doc = "Undefined"]
            pub const PIC_3: u32 = 0x03;
        }
    }
    #[doc = "Port Test Control - Read/Write"]
    pub mod PTC {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "TEST_MODE_DISABLE"]
            pub const PTC_0: u32 = 0;
            #[doc = "J_STATE"]
            pub const PTC_1: u32 = 0x01;
            #[doc = "K_STATE"]
            pub const PTC_2: u32 = 0x02;
            #[doc = "SE0 (host) / NAK (device)"]
            pub const PTC_3: u32 = 0x03;
            #[doc = "Packet"]
            pub const PTC_4: u32 = 0x04;
            #[doc = "FORCE_ENABLE_HS"]
            pub const PTC_5: u32 = 0x05;
            #[doc = "FORCE_ENABLE_FS"]
            pub const PTC_6: u32 = 0x06;
            #[doc = "FORCE_ENABLE_LS"]
            pub const PTC_7: u32 = 0x07;
        }
    }
    #[doc = "Wake on Connect Enable (WKCNNT_E) - Read/Write"]
    pub mod WKCN {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Wake on Disconnect Enable (WKDSCNNT_E) - Read/Write"]
    pub mod WKDC {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Wake on Over-current Enable (WKOC_E) - Read/Write"]
    pub mod WKOC {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "PHY Low Power Suspend - Clock Disable (PLPSCD) - Read/Write"]
    pub mod PHCD {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Enable PHY clock"]
            pub const PHCD_0: u32 = 0;
            #[doc = "Disable PHY clock"]
            pub const PHCD_1: u32 = 0x01;
        }
    }
    #[doc = "Port Force Full Speed Connect - Read/Write"]
    pub mod PFSC {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Normal operation"]
            pub const PFSC_0: u32 = 0;
            #[doc = "Forced to full speed"]
            pub const PFSC_1: u32 = 0x01;
        }
    }
    #[doc = "See description at bits 31-30"]
    pub mod PTS_2 {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Port Speed - Read Only. This register field indicates the speed at which the port is operating."]
    pub mod PSPD {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Full Speed"]
            pub const PSPD_0: u32 = 0;
            #[doc = "Low Speed"]
            pub const PSPD_1: u32 = 0x01;
            #[doc = "High Speed"]
            pub const PSPD_2: u32 = 0x02;
            #[doc = "Undefined"]
            pub const PSPD_3: u32 = 0x03;
        }
    }
    #[doc = "Parallel Transceiver Width This bit has no effect if serial interface engine is used"]
    pub mod PTW {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Select the 8-bit UTMI interface \\[60MHz\\]"]
            pub const PTW_0: u32 = 0;
            #[doc = "Select the 16-bit UTMI interface \\[30MHz\\]"]
            pub const PTW_1: u32 = 0x01;
        }
    }
    #[doc = "Serial Transceiver Select 1 Serial Interface Engine is selected 0 Parallel Interface signals is selected Serial Interface Engine can be used in combination with UTMI+/ULPI physical interface to provide FS/LS signaling instead of the parallel interface signals"]
    pub mod STS {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "All USB port interface modes are listed in this field description, but not all are supported"]
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
    #[doc = "VBUS_Discharge - Read/Write. Setting this bit causes VBus to discharge through a resistor."]
    pub mod VD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "VBUS Charge - Read/Write"]
    pub mod VC {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "OTG Termination - Read/Write"]
    pub mod OT {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data Pulsing - Read/Write"]
    pub mod DP {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ID Pullup - Read/Write This bit provide control over the ID pull-up resistor; 0 = off, 1 = on \\[default\\]"]
    pub mod IDPU {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "USB ID - Read Only. 0 = A device, 1 = B device"]
    pub mod ID {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "A VBus Valid - Read Only. Indicates VBus is above the A VBus valid threshold."]
    pub mod AVV {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "A Session Valid - Read Only. Indicates VBus is above the A session valid threshold."]
    pub mod ASV {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "B Session Valid - Read Only. Indicates VBus is above the B session valid threshold."]
    pub mod BSV {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "B Session End - Read Only. Indicates VBus is below the B session end threshold."]
    pub mod BSE {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1 millisecond timer toggle - Read Only. This bit toggles once per millisecond."]
    pub mod TOG_1MS {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data Bus Pulsing Status - Read Only"]
    pub mod DPS {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "USB ID Interrupt Status - Read/Write"]
    pub mod IDIS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "A VBus Valid Interrupt Status - Read/Write to Clear"]
    pub mod AVVIS {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "A Session Valid Interrupt Status - Read/Write to Clear"]
    pub mod ASVIS {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "B Session Valid Interrupt Status - Read/Write to Clear"]
    pub mod BSVIS {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "B Session End Interrupt Status - Read/Write to Clear"]
    pub mod BSEIS {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1 millisecond timer Interrupt Status - Read/Write to Clear"]
    pub mod STATUS_1MS {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data Pulse Interrupt Status - Read/Write to Clear"]
    pub mod DPIS {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "USB ID Interrupt Enable - Read/Write. Setting this bit enables the USB ID interrupt."]
    pub mod IDIE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "A VBus Valid Interrupt Enable - Read/Write. Setting this bit enables the A VBus valid interrupt."]
    pub mod AVVIE {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "A Session Valid Interrupt Enable - Read/Write"]
    pub mod ASVIE {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "B Session Valid Interrupt Enable - Read/Write"]
    pub mod BSVIE {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "B Session End Interrupt Enable - Read/Write. Setting this bit enables the B session end interrupt."]
    pub mod BSEIE {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1 millisecond timer Interrupt Enable - Read/Write"]
    pub mod EN_1MS {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data Pulse Interrupt Enable"]
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
    #[doc = "Controller Mode - R/WO"]
    pub mod CM {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Idle \\[Default for combination host/device\\]"]
            pub const CM_0: u32 = 0;
            #[doc = "Device Controller \\[Default for device only controller\\]"]
            pub const CM_2: u32 = 0x02;
            #[doc = "Host Controller \\[Default for host only controller\\]"]
            pub const CM_3: u32 = 0x03;
        }
    }
    #[doc = "Endian Select - Read/Write"]
    pub mod ES {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Little Endian \\[Default\\]"]
            pub const ES_0: u32 = 0;
            #[doc = "Big Endian"]
            pub const ES_1: u32 = 0x01;
        }
    }
    #[doc = "Setup Lockout Mode"]
    pub mod SLOM {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Setup Lockouts On (default);"]
            pub const SLOM_0: u32 = 0;
            #[doc = "Setup Lockouts Off (DCD requires use of Setup Data Buffer Tripwire in USBCMDUSB Command Register ."]
            pub const SLOM_1: u32 = 0x01;
        }
    }
    #[doc = "Stream Disable Mode"]
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
    #[doc = "Setup Endpoint Status"]
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
    #[doc = "Prime Endpoint Receive Buffer - R/WS"]
    pub mod PERB {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Prime Endpoint Transmit Buffer - R/WS"]
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
    #[doc = "Flush Endpoint Receive Buffer - R/WS"]
    pub mod FERB {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Flush Endpoint Transmit Buffer - R/WS"]
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
    #[doc = "Endpoint Receive Buffer Ready -- Read Only"]
    pub mod ERBR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Endpoint Transmit Buffer Ready -- Read Only"]
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
    #[doc = "Endpoint Receive Complete Event - RW/C"]
    pub mod ERCE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Endpoint Transmit Complete Event - R/WC"]
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
    #[doc = "RX Endpoint Stall - Read/Write 0 End Point OK"]
    pub mod RXS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "RX Endpoint Type - Read/Write 00 Control Endpoint0 is fixed as a Control End Point."]
    pub mod RXT {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "RX Endpoint Enable 1 Enabled Endpoint0 is always enabled."]
    pub mod RXE {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TX Endpoint Stall - Read/Write 0 End Point OK \\[Default\\] 1 End Point Stalled Software can write a one to this bit to force the endpoint to return a STALL handshake to the Host"]
    pub mod TXS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TX Endpoint Type - Read/Write 00 - Control Endpoint0 is fixed as a Control End Point."]
    pub mod TXT {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TX Endpoint Enable 1 Enabled Endpoint0 is always enabled."]
    pub mod TXE {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Endpoint Control"]
pub mod ENDPTCTRL {
    #[doc = "RX Endpoint Stall - Read/Write 0 End Point OK"]
    pub mod RXS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "RX Endpoint Data Sink - Read/Write 0 Dual Port Memory Buffer/DMA Engine \\[Default\\] Should always be written as zero"]
    pub mod RXD {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "RX Endpoint Type - Read/Write 00 Control 01 Isochronous 10 Bulk 11 Interrupt"]
    pub mod RXT {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "RX Data Toggle Inhibit 0 Disabled \\[Default\\] 1 Enabled This bit is only used for test and should always be written as zero"]
    pub mod RXI {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "RX Data Toggle Reset (WS) Write 1 - Reset PID Sequence Whenever a configuration event is received for this Endpoint, software must write a one to this bit in order to synchronize the data PID's between the host and device"]
    pub mod RXR {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "RX Endpoint Enable 0 Disabled \\[Default\\] 1 Enabled An Endpoint should be enabled only after it has been configured"]
    pub mod RXE {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TX Endpoint Stall - Read/Write 0 End Point OK 1 End Point Stalled This bit will be cleared automatically upon receipt of a SETUP request if this Endpoint is configured as a Control Endpoint and this bit will continue to be cleared by hardware until the associated ENDPTSETUPSTAT bit is cleared"]
    pub mod TXS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TX Endpoint Data Source - Read/Write 0 Dual Port Memory Buffer/DMA Engine \\[DEFAULT\\] Should always be written as 0"]
    pub mod TXD {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TX Endpoint Type - Read/Write 00 Control 01 Isochronous 10 Bulk 11 Interrupt"]
    pub mod TXT {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TX Data Toggle Inhibit 0 PID Sequencing Enabled"]
    pub mod TXI {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TX Data Toggle Reset (WS) Write 1 - Reset PID Sequence Whenever a configuration event is received for this Endpoint, software must write a one to this bit in order to synchronize the data PID's between the Host and device"]
    pub mod TXR {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TX Endpoint Enable 0 Disabled \\[Default\\] 1 Enabled An Endpoint should be enabled only after it has been configured"]
    pub mod TXE {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
