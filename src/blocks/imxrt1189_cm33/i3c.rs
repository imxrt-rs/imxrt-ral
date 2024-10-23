#[doc = "I3C"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "Controller Configuration"]
    pub MCONFIG: crate::RWRegister<u32>,
    #[doc = "Target Configuration"]
    pub SCONFIG: crate::RWRegister<u32>,
    #[doc = "Target Status"]
    pub SSTATUS: crate::RWRegister<u32>,
    #[doc = "Target Control"]
    pub SCTRL: crate::RWRegister<u32>,
    #[doc = "Target Interrupt Set"]
    pub SINTSET: crate::RWRegister<u32>,
    #[doc = "Target Interrupt Clear"]
    pub SINTCLR: crate::RWRegister<u32>,
    #[doc = "Target Interrupt Mask"]
    pub SINTMASKED: crate::RORegister<u32>,
    #[doc = "Target Errors and Warnings"]
    pub SERRWARN: crate::RWRegister<u32>,
    #[doc = "Target DMA Control"]
    pub SDMACTRL: crate::RWRegister<u32>,
    _reserved0: [u8; 0x08],
    #[doc = "Target Data Control"]
    pub SDATACTRL: crate::RWRegister<u32>,
    #[doc = "Target Write Data Byte"]
    pub SWDATAB: crate::WORegister<u32>,
    #[doc = "Target Write Data Byte End"]
    pub SWDATABE: crate::WORegister<u32>,
    #[doc = "Target Write Data Halfword"]
    pub SWDATAH: crate::WORegister<u32>,
    #[doc = "Target Write Data Halfword End"]
    pub SWDATAHE: crate::WORegister<u32>,
    #[doc = "Target Read Data Byte"]
    pub SRDATAB: crate::RORegister<u32>,
    _reserved1: [u8; 0x04],
    #[doc = "Target Read Data Halfword"]
    pub SRDATAH: crate::RORegister<u32>,
    _reserved2: [u8; 0x08],
    #[doc = "Target Write Data Byte"]
    pub SWDATAB1: crate::WORegister<u32>,
    _reserved3: [u8; 0x04],
    #[doc = "Target Capabilities 2"]
    pub SCAPABILITIES2: crate::RORegister<u32>,
    #[doc = "Target Capabilities"]
    pub SCAPABILITIES: crate::RORegister<u32>,
    _reserved4: [u8; 0x04],
    #[doc = "Target Maximum Limits"]
    pub SMAXLIMITS: crate::RWRegister<u32>,
    #[doc = "Target ID Part Number"]
    pub SIDPARTNO: crate::RWRegister<u32>,
    #[doc = "Target ID Extension"]
    pub SIDEXT: crate::RWRegister<u32>,
    #[doc = "Target Vendor ID"]
    pub SVENDORID: crate::RWRegister<u32>,
    #[doc = "Target Time Control Clock"]
    pub STCCLOCK: crate::RWRegister<u32>,
    #[doc = "Target Message Map Address"]
    pub SMSGMAPADDR: crate::RORegister<u32>,
    _reserved5: [u8; 0x04],
    #[doc = "Controller Control"]
    pub MCTRL: crate::RWRegister<u32>,
    #[doc = "Controller Status"]
    pub MSTATUS: crate::RWRegister<u32>,
    #[doc = "Controller In-band Interrupt Registry and Rules"]
    pub MIBIRULES: crate::RWRegister<u32>,
    #[doc = "Controller Interrupt Set"]
    pub MINTSET: crate::RWRegister<u32>,
    #[doc = "Controller Interrupt Clear"]
    pub MINTCLR: crate::RWRegister<u32>,
    #[doc = "Controller Interrupt Mask"]
    pub MINTMASKED: crate::RORegister<u32>,
    #[doc = "Controller Errors and Warnings"]
    pub MERRWARN: crate::RWRegister<u32>,
    #[doc = "Controller DMA Control"]
    pub MDMACTRL: crate::RWRegister<u32>,
    _reserved6: [u8; 0x08],
    #[doc = "Controller Data Control"]
    pub MDATACTRL: crate::RWRegister<u32>,
    #[doc = "Controller Write Data Byte"]
    pub MWDATAB: crate::WORegister<u32>,
    #[doc = "Controller Write Data Byte End"]
    pub MWDATABE: crate::WORegister<u32>,
    #[doc = "Controller Write Data Halfword"]
    pub MWDATAH: crate::WORegister<u32>,
    #[doc = "Controller Write Data Halfword End"]
    pub MWDATAHE: crate::WORegister<u32>,
    #[doc = "Controller Read Data Byte"]
    pub MRDATAB: crate::RORegister<u32>,
    _reserved7: [u8; 0x04],
    #[doc = "Controller Read Data Halfword"]
    pub MRDATAH: crate::RORegister<u32>,
    #[doc = "Controller Write Byte Data 1 (to Bus)"]
    pub MWDATAB1: crate::WORegister<u32>,
    #[doc = "Controller Write Message Control in SDR mode"]
    pub MWMSG_SDR_CONTROL: crate::WORegister<u32>,
    #[doc = "Controller Read Message in SDR mode"]
    pub MRMSG_SDR: crate::RORegister<u32>,
    #[doc = "Controller Write Message in DDR mode: First Control Word"]
    pub MWMSG_DDR_CONTROL: crate::WORegister<u32>,
    #[doc = "Controller Read Message in DDR mode"]
    pub MRMSG_DDR: crate::RORegister<u32>,
    _reserved8: [u8; 0x04],
    #[doc = "Controller Dynamic Address"]
    pub MDYNADDR: crate::RWRegister<u32>,
    _reserved9: [u8; 0x18],
    #[doc = "Timing Rules for Target Reset Recovery"]
    pub SRSTACTTIME: crate::RWRegister<u32>,
    _reserved10: [u8; 0x08],
    #[doc = "CCC Mask for Unhandled CCCs"]
    pub SCCCMASK: crate::RWRegister<u32>,
    #[doc = "Target Errors and Warnings Mask"]
    pub SERRWARNMASK: crate::RWRegister<u32>,
    _reserved11: [u8; 0x08],
    #[doc = "Map Feature Control 0"]
    pub SMAPCTRL0: crate::RORegister<u32>,
    #[doc = "Map Feature Control 1"]
    pub SMAPCTRL1: crate::RWRegister<u32>,
    _reserved12: [u8; 0x1c],
    #[doc = "Extended IBI Data 1"]
    pub IBIEXT1: crate::RWRegister<u32>,
    #[doc = "Extended IBI Data 2"]
    pub IBIEXT2: crate::RWRegister<u32>,
}
#[doc = "Controller Configuration"]
pub mod MCONFIG {
    #[doc = "Controller Enable"]
    pub mod MSTENA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "CONTROLLER_OFF"]
            pub const MASTER_OFF: u32 = 0;
            #[doc = "CONTROLLER_ON"]
            pub const MASTER_ON: u32 = 0x01;
            #[doc = "CONTROLLER_CAPABLE"]
            pub const MASTER_CAPABLE: u32 = 0x02;
        }
    }
    #[doc = "Disable Timeout"]
    pub mod DISTO {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Enabled"]
            pub const ENABLE: u32 = 0;
            #[doc = "Disabled, if configured"]
            pub const DISABLE: u32 = 0x01;
        }
    }
    #[doc = "High-Keeper"]
    pub mod HKEEP {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "None"]
            pub const NONE: u32 = 0;
            #[doc = "WIRED_IN"]
            pub const WIRED_IN: u32 = 0x01;
            #[doc = "PASSIVE_SDA"]
            pub const PASSIVE_SDA: u32 = 0x02;
            #[doc = "PASSIVE_ON_SDA_SCL"]
            pub const PASSIVE_ON_SDA_SCL: u32 = 0x03;
        }
    }
    #[doc = "Open Drain Stop"]
    pub mod ODSTOP {
        pub const offset: u32 = 6;
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
    #[doc = "Push-Pull Baud Rate"]
    pub mod PPBAUD {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Push-Pull Low"]
    pub mod PPLOW {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Open Drain Baud Rate"]
    pub mod ODBAUD {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Open Drain High Push-Pull"]
    pub mod ODHPP {
        pub const offset: u32 = 24;
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
    #[doc = "Skew"]
    pub mod SKEW {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "I2C Baud Rate"]
    pub mod I2CBAUD {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Target Configuration"]
pub mod SCONFIG {
    #[doc = "Target Enable"]
    pub mod SLVENA {
        pub const offset: u32 = 0;
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
    #[doc = "Not Acknowledge"]
    pub mod NACK {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Always disable NACK mode"]
            pub const DISABLE: u32 = 0;
            #[doc = "Always enable NACK mode (works normally)"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Match Start or Stop"]
    pub mod MATCHSS {
        pub const offset: u32 = 2;
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
    #[doc = "Ignore TE0 or TE1 Errors"]
    pub mod S0IGNORE {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Do not ignore TE0 or TE1 errors"]
            pub const DISABLE: u32 = 0;
            #[doc = "Ignore TE0 or TE1 errors"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Double Data Rate OK"]
    pub mod DDROK {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Do not allow HDR-DDR messaging"]
            pub const DISABLE: u32 = 0;
            #[doc = "Allow HDR-DDR messaging"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Offline"]
    pub mod OFFLINE {
        pub const offset: u32 = 9;
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
    #[doc = "Static Address"]
    pub mod SADDR {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x7f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Target Status"]
pub mod SSTATUS {
    #[doc = "Status not Stop"]
    pub mod STNOTSTOP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "In STOP condition"]
            pub const STOPPED: u32 = 0;
            #[doc = "Busy"]
            pub const BUSY: u32 = 0x01;
        }
    }
    #[doc = "Status Message"]
    pub mod STMSG {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Idle"]
            pub const IDLE: u32 = 0;
            #[doc = "Busy"]
            pub const BUSY: u32 = 0x01;
        }
    }
    #[doc = "Status Common Command Code Handler"]
    pub mod STCCCH {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No CCC message handled"]
            pub const IDLE: u32 = 0;
            #[doc = "Handled automatically"]
            pub const BUSY: u32 = 0x01;
        }
    }
    #[doc = "Status Request Read"]
    pub mod STREQRD {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not an SDR read"]
            pub const IDLE: u32 = 0;
            #[doc = "SDR read from this target or an IBI is being pushed out"]
            pub const BUSY: u32 = 0x01;
        }
    }
    #[doc = "Status Request Write"]
    pub mod STREQWR {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not an SDR write"]
            pub const IDLE: u32 = 0;
            #[doc = "SDR write data from the controller, but not in ENTDAA mode"]
            pub const BUSY: u32 = 0x01;
        }
    }
    #[doc = "Status Dynamic Address Assignment"]
    pub mod STDAA {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not in ENTDAA mode"]
            pub const NOT_IN_ENTDAA: u32 = 0;
            #[doc = "In ENTDAA mode"]
            pub const IN_ENTDAA: u32 = 0x01;
        }
    }
    #[doc = "Status High Data Rate"]
    pub mod STHDR {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "I3C bus not in HDR-DDR mode"]
            pub const NOT_IN_HDR_DDR: u32 = 0;
            #[doc = "I3C bus in HDR-DDR mode"]
            pub const IN_HDR_DDR: u32 = 0x01;
        }
    }
    #[doc = "Start Flag"]
    pub mod START {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {
            #[doc = "Not detected"]
            pub const START_NOT_DETECTED: u32 = 0;
            #[doc = "Detected"]
            pub const START_DETECTED: u32 = 0x01;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Matched Flag"]
    pub mod MATCHED {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {
            #[doc = "Header not matched"]
            pub const NOT_MATCHED: u32 = 0;
            #[doc = "Header matched"]
            pub const MATCHED: u32 = 0x01;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Stop Flag"]
    pub mod STOP {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {
            #[doc = "No Stopped state detected"]
            pub const NO_STOP_DETECTED: u32 = 0;
            #[doc = "Stopped state detected"]
            pub const STOP_DETECTED: u32 = 0x01;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Received Message Pending"]
    pub mod RX_PEND {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No received message pending"]
            pub const NO_MSG_PENDING: u32 = 0;
            #[doc = "Received message pending"]
            pub const MSG_PENDING: u32 = 0x01;
        }
    }
    #[doc = "Transmit Buffer Not Full"]
    pub mod TXNOTFULL {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Transmit buffer full"]
            pub const FULL: u32 = 0;
            #[doc = "Transmit buffer not full"]
            pub const NOT_FULL: u32 = 0x01;
        }
    }
    #[doc = "Dynamic Address Change Flag"]
    pub mod DACHG {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {
            #[doc = "No DA change detected"]
            pub const NO_CHANGE_DETECTED: u32 = 0;
            #[doc = "DA change detected"]
            pub const CHANGE_DETECTED: u32 = 0x01;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Common Command Code Flag"]
    pub mod CCC {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {
            #[doc = "CCC not received"]
            pub const NO_CCC_RECEIVED: u32 = 0;
            #[doc = "CCC received"]
            pub const CCC_RECEIVED: u32 = 0x01;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Error Warning"]
    pub mod ERRWARN {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "High Data Rate Command Match Flag"]
    pub mod HDRMATCH {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {
            #[doc = "Did not match"]
            pub const NO_MATCH: u32 = 0;
            #[doc = "Matched the I3C dynamic address"]
            pub const MATCH: u32 = 0x01;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Common Command Code Handled Flag"]
    pub mod CHANDLED {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {
            #[doc = "CCC handling not in progress"]
            pub const NOT_HANDLED: u32 = 0;
            #[doc = "CCC handling in progress"]
            pub const HANDLED: u32 = 0x01;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Event Flag"]
    pub mod EVENT {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {
            #[doc = "No event occurred"]
            pub const NO_EVENT: u32 = 0;
            #[doc = "IBI, CR, or HJ occurred"]
            pub const EVENT: u32 = 0x01;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Target Reset Flag"]
    pub mod SLVRST {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Event Details"]
    pub mod EVDET {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "NONE (no event or no pending event)"]
            pub const NONE: u32 = 0;
            #[doc = "NO_REQUEST (request is not sent yet; either there is no START condition yet, or is waiting for Bus-Available or Bus-Idle (HJ))"]
            pub const NO_REQUEST: u32 = 0x01;
            #[doc = "NACKed (not acknowledged, request sent and rejected); I3C tries again"]
            pub const NACKED: u32 = 0x02;
            #[doc = "ACKed (acknowledged; request sent and accepted), so done (unless the time control data is still being sent)"]
            pub const ACKED: u32 = 0x03;
        }
    }
    #[doc = "In-Band Interrupts Disable"]
    pub mod IBIDIS {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Enabled"]
            pub const INTERRUPTS_ENABLED: u32 = 0;
            #[doc = "Disabled"]
            pub const INTERRUPTS_DISABLED: u32 = 0x01;
        }
    }
    #[doc = "Controller Requests Disable"]
    pub mod MRDIS {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Enabled"]
            pub const MR_ENABLED: u32 = 0;
            #[doc = "Disabled"]
            pub const MR_DISABLED: u32 = 0x01;
        }
    }
    #[doc = "Hot-Join Disabled"]
    pub mod HJDIS {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Enabled"]
            pub const MR_ENABLED: u32 = 0;
            #[doc = "Disabled"]
            pub const MR_DISABLED: u32 = 0x01;
        }
    }
    #[doc = "Activity State from Common Command Codes (CCC)"]
    pub mod ACTSTATE {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "NO_LATENCY (normal bus operations)"]
            pub const NO_LATENCY: u32 = 0;
            #[doc = "LATENCY_1MS (1 ms of latency)"]
            pub const LATENCY_1MS: u32 = 0x01;
            #[doc = "LATENCY_100MS (100 ms of latency)"]
            pub const LATENCY_100MS: u32 = 0x02;
            #[doc = "LATENCY_10S (10 seconds of latency)"]
            pub const LATENCY_10S: u32 = 0x03;
        }
    }
    #[doc = "Time Control"]
    pub mod TIMECTRL {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "NO_TIME_CONTROL (no time control is enabled)"]
            pub const NO_TIME_CONTROL: u32 = 0;
            #[doc = "SYNC_MODE (Synchronous mode is enabled)"]
            pub const SYNC: u32 = 0x01;
            #[doc = "ASYNC_MODE (Asynchronous standard mode (0 or 1) is enabled)"]
            pub const ASYNC_MODE: u32 = 0x02;
            #[doc = "BOTHSYNCASYNC (both Synchronous and Asynchronous modes are enabled)"]
            pub const BOTHSYNCASYNC: u32 = 0x03;
        }
    }
}
#[doc = "Target Control"]
pub mod SCTRL {
    #[doc = "Event"]
    pub mod EVENT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "NORMAL_MODE"]
            pub const NORMAL_MODE: u32 = 0;
            #[doc = "IBI"]
            pub const IBI: u32 = 0x01;
            #[doc = "CONTROLLER_REQUEST"]
            pub const MASTER_REQUEST: u32 = 0x02;
            #[doc = "HOT_JOIN_REQUEST"]
            pub const HOT_JOIN_REQUEST: u32 = 0x03;
        }
    }
    #[doc = "Extended Data"]
    pub mod EXTDATA {
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
    #[doc = "Map Index"]
    pub mod MAPIDX {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "In-Band Interrupt Data"]
    pub mod IBIDATA {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Pending Interrupt"]
    pub mod PENDINT {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Activity State of Target"]
    pub mod ACTSTATE {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Vendor Information"]
    pub mod VENDINFO {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Target Interrupt Set"]
pub mod SINTSET {
    #[doc = "Start Interrupt Enable"]
    pub mod START {
        pub const offset: u32 = 8;
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
    #[doc = "Match Interrupt Enable"]
    pub mod MATCHED {
        pub const offset: u32 = 9;
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
    #[doc = "Stop Interrupt Enable"]
    pub mod STOP {
        pub const offset: u32 = 10;
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
    #[doc = "Receive Interrupt Enable"]
    pub mod RXPEND {
        pub const offset: u32 = 11;
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
    #[doc = "Transmit Interrupt Enable"]
    pub mod TXSEND {
        pub const offset: u32 = 12;
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
    #[doc = "Dynamic Address Change Interrupt Enable"]
    pub mod DACHG {
        pub const offset: u32 = 13;
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
    #[doc = "Common Command Code (CCC) Interrupt Enable"]
    pub mod CCC {
        pub const offset: u32 = 14;
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
    #[doc = "Error or Warning Interrupt Enable"]
    pub mod ERRWARN {
        pub const offset: u32 = 15;
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
    #[doc = "Double Data Rate Interrupt Enable"]
    pub mod DDRMATCHED {
        pub const offset: u32 = 16;
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
    #[doc = "Common Command Code (CCC) Interrupt Enable"]
    pub mod CHANDLED {
        pub const offset: u32 = 17;
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
    #[doc = "Event Interrupt Enable"]
    pub mod EVENT {
        pub const offset: u32 = 18;
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
    #[doc = "Target Reset"]
    pub mod SLVRST {
        pub const offset: u32 = 19;
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
}
#[doc = "Target Interrupt Clear"]
pub mod SINTCLR {
    #[doc = "START Interrupt Enable Clear Flag"]
    pub mod START {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Matched Interrupt Enable Clear Flag"]
    pub mod MATCHED {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "STOP Interrupt Enable Clear Flag"]
    pub mod STOP {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "RXPEND Interrupt Enable Clear Flag"]
    pub mod RXPEND {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TXSEND Interrupt Enable Clear Flag"]
    pub mod TXSEND {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DACHG Interrupt Enable Clear Flag"]
    pub mod DACHG {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CCC Interrupt Enable Clear Flag"]
    pub mod CCC {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ERRWARN Interrupt Enable Clear Flag"]
    pub mod ERRWARN {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DDRMATCHED Interrupt Enable Clear Flag"]
    pub mod DDRMATCHED {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CHANDLED Interrupt Enable Clear Flag"]
    pub mod CHANDLED {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "EVENT Interrupt Enable Clear Flag"]
    pub mod EVENT {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Target Reset Flag (SLVRST Interrupt Enable Clear)"]
    pub mod SLVRST {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Target Interrupt Mask"]
pub mod SINTMASKED {
    #[doc = "START Interrupt Mask"]
    pub mod START {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "MATCHED Interrupt Mask"]
    pub mod MATCHED {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "STOP Interrupt Mask"]
    pub mod STOP {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "RXPEND Interrupt Mask"]
    pub mod RXPEND {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TXSEND Interrupt Mask"]
    pub mod TXSEND {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DACHG Interrupt Mask"]
    pub mod DACHG {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CCC Interrupt Mask"]
    pub mod CCC {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ERRWARN Interrupt Mask"]
    pub mod ERRWARN {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DDRMATCHED Interrupt Mask"]
    pub mod DDRMATCHED {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CHANDLED Interrupt Mask"]
    pub mod CHANDLED {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "EVENT Interrupt Mask"]
    pub mod EVENT {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Target Reset Interrupt Mask"]
    pub mod SLVRST {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Target Errors and Warnings"]
pub mod SERRWARN {
    #[doc = "Overrun Error Flag"]
    pub mod ORUN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {
            #[doc = "No overrun error"]
            pub const NO_ERROR: u32 = 0;
            #[doc = "Overrun error"]
            pub const ERROR: u32 = 0x01;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Underrun Error Flag"]
    pub mod URUN {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {
            #[doc = "No underrun error"]
            pub const NO_ERROR: u32 = 0;
            #[doc = "Underrun error"]
            pub const ERROR: u32 = 0x01;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Underrun and Not Acknowledged (NACKed) Error Flag"]
    pub mod URUNNACK {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {
            #[doc = "No underrun; not acknowledged error"]
            pub const NO_ERROR: u32 = 0;
            #[doc = "Underrun; not acknowledged error"]
            pub const ERROR: u32 = 0x01;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Terminated Error Flag"]
    pub mod TERM {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {
            #[doc = "No terminated error"]
            pub const NO_ERROR: u32 = 0;
            #[doc = "Terminated error"]
            pub const ERROR: u32 = 0x01;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Invalid Start Error Flag"]
    pub mod INVSTART {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {
            #[doc = "No invalid start error"]
            pub const NO_ERROR: u32 = 0;
            #[doc = "Invalid start error"]
            pub const ERROR: u32 = 0x01;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "SDR Parity Error Flag"]
    pub mod SPAR {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {
            #[doc = "No SDR parity error"]
            pub const NO_ERROR: u32 = 0;
            #[doc = "SDR parity error"]
            pub const ERROR: u32 = 0x01;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "HDR Parity Error Flag"]
    pub mod HPAR {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {
            #[doc = "No HDR parity error"]
            pub const NO_ERROR: u32 = 0;
            #[doc = "HDR parity error"]
            pub const ERROR: u32 = 0x01;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "HDR-DDR CRC Error Flag"]
    pub mod HCRC {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {
            #[doc = "No HDR-DDR CRC error occurred"]
            pub const NO_ERROR: u32 = 0;
            #[doc = "HDR-DDR CRC error occurred"]
            pub const ERROR: u32 = 0x01;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TE0 or TE1 Error Flag"]
    pub mod S0S1 {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {
            #[doc = "No TE0 or TE1 error occurred"]
            pub const NO_ERROR: u32 = 0;
            #[doc = "TE0 or TE1 error occurred"]
            pub const ERROR: u32 = 0x01;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Over-Read Error Flag"]
    pub mod OREAD {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {
            #[doc = "No over-read error"]
            pub const NO_ERROR: u32 = 0;
            #[doc = "Over-read error"]
            pub const ERROR: u32 = 0x01;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Over-Write Error Flag"]
    pub mod OWRITE {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {
            #[doc = "No overwrite error"]
            pub const NO_ERROR: u32 = 0;
            #[doc = "Overwrite error"]
            pub const ERROR: u32 = 0x01;
        }
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Target DMA Control"]
pub mod SDMACTRL {
    #[doc = "DMA Read (From-Bus) Trigger"]
    pub mod DMAFB {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "DMA not used"]
            pub const NOT_USED: u32 = 0;
            #[doc = "DMA enabled for one frame"]
            pub const ENABLE_ONE_FRAME: u32 = 0x01;
            #[doc = "DMA enabled until turned off"]
            pub const ENABLE: u32 = 0x02;
        }
    }
    #[doc = "DMA Write (To-Bus) Trigger"]
    pub mod DMATB {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "DMA not used"]
            pub const NOT_USED: u32 = 0;
            #[doc = "DMA enabled for one frame"]
            pub const ENABLE_ONE_FRAME: u32 = 0x01;
            #[doc = "DMA enabled until turned off"]
            pub const ENABLE: u32 = 0x02;
        }
    }
    #[doc = "Width of DMA Operations"]
    pub mod DMAWIDTH {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Byte"]
            pub const BYTE_0: u32 = 0;
            #[doc = "Byte"]
            pub const BYTE_1: u32 = 0x01;
            #[doc = "Halfword (16 bits) (this value ensures that two bytes are available in the FIFO)"]
            pub const HALF_WORD: u32 = 0x02;
        }
    }
}
#[doc = "Target Data Control"]
pub mod SDATACTRL {
    #[doc = "Flush To-Bus Buffer or FIFO"]
    pub mod FLUSHTB {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Flush From-Bus Buffer or FIFO"]
    pub mod FLUSHFB {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Unlock"]
    pub mod UNLOCK {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Cannot be changed"]
            pub const DISABLED: u32 = 0;
            #[doc = "Can be changed"]
            pub const ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Transmit Trigger Level"]
    pub mod TXTRIG {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Trigger when empty"]
            pub const TRIGGREMPTY: u32 = 0;
            #[doc = "Trigger when 1/4 full or less"]
            pub const TRIGGRONEFOURTH: u32 = 0x01;
            #[doc = "Trigger when 1/2 full or less"]
            pub const TRIGGRONEHALF: u32 = 0x02;
            #[doc = "Default (trigger when 1 less than full or less)"]
            pub const TRIGGRONELESS: u32 = 0x03;
        }
    }
    #[doc = "Receive Trigger Level"]
    pub mod RXTRIG {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Trigger when not empty"]
            pub const TRIGGRNOTEMPTY: u32 = 0;
            #[doc = "Trigger when 1/4 or more full"]
            pub const TRIGGRONEFOURTH: u32 = 0x01;
            #[doc = "Trigger when 1/2 or more full"]
            pub const TRIGGRONEHALF: u32 = 0x02;
            #[doc = "Trigger when 3/4 or more full"]
            pub const TRIGGRTHREEFOURTHS: u32 = 0x03;
        }
    }
    #[doc = "Count of Bytes in Transmit"]
    pub mod TXCOUNT {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Count of Bytes in Receive"]
    pub mod RXCOUNT {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Transmit is Full"]
    pub mod TXFULL {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not full"]
            pub const TXISNOTFULL: u32 = 0;
            #[doc = "Full"]
            pub const TXISFULL: u32 = 0x01;
        }
    }
    #[doc = "Receive is Empty"]
    pub mod RXEMPTY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not empty"]
            pub const RXISNOTEMPTY: u32 = 0;
            #[doc = "Empty"]
            pub const RXISEMPTY: u32 = 0x01;
        }
    }
}
#[doc = "Target Write Data Byte"]
pub mod SWDATAB {
    #[doc = "Data"]
    pub mod DATA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "End"]
    pub mod END {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not the end"]
            pub const NOT_END: u32 = 0;
            #[doc = "End"]
            pub const END: u32 = 0x01;
        }
    }
    #[doc = "End Also"]
    pub mod END_ALSO {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not the end"]
            pub const NOT_END: u32 = 0;
            #[doc = "End"]
            pub const END: u32 = 0x01;
        }
    }
}
#[doc = "Target Write Data Byte End"]
pub mod SWDATABE {
    #[doc = "Data"]
    pub mod DATA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Target Write Data Halfword"]
pub mod SWDATAH {
    #[doc = "Data 0"]
    pub mod DATA0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data 1"]
    pub mod DATA1 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "End of Message"]
    pub mod END {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not the end"]
            pub const NOT_END: u32 = 0;
            #[doc = "End"]
            pub const END: u32 = 0x01;
        }
    }
}
#[doc = "Target Write Data Halfword End"]
pub mod SWDATAHE {
    #[doc = "Data 0"]
    pub mod DATA0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data 1"]
    pub mod DATA1 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Target Read Data Byte"]
pub mod SRDATAB {
    #[doc = "Data 0"]
    pub mod DATA0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Target Read Data Halfword"]
pub mod SRDATAH {
    #[doc = "Low Byte"]
    pub mod LSB {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "High Byte"]
    pub mod MSB {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Target Write Data Byte"]
pub mod SWDATAB1 {
    #[doc = "Data"]
    pub mod DATA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Target Capabilities 2"]
pub mod SCAPABILITIES2 {
    #[doc = "Map Count"]
    pub mod MAPCNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "I2C 10-bit Address"]
    pub mod I2C10B {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not supported"]
            pub const DISABLE: u32 = 0;
            #[doc = "Supported"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "I2C Device ID"]
    pub mod I2CDEVID {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not supported"]
            pub const DISABLE: u32 = 0;
            #[doc = "Supported"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "In-Band Interrupt EXTDATA"]
    pub mod IBIEXT {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not supported"]
            pub const DISABLE: u32 = 0;
            #[doc = "Supported"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "In-Band Interrupt Extended Register"]
    pub mod IBIXREG {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not supported"]
            pub const DISABLE: u32 = 0;
            #[doc = "Supported"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Version 1.1"]
    pub mod V1_1 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not supported"]
            pub const DISABLE: u32 = 0;
            #[doc = "Supported"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Target Reset"]
    pub mod SLVRST {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not supported"]
            pub const DISABLE: u32 = 0;
            #[doc = "Supported"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Group"]
    pub mod GROUP {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "v1.1 group addressing not supported"]
            pub const NOTSUPPORTED: u32 = 0;
            #[doc = "One group supported"]
            pub const ONE: u32 = 0x01;
            #[doc = "Two groups supported"]
            pub const TWO: u32 = 0x02;
            #[doc = "Three groups supported"]
            pub const THREE: u32 = 0x03;
        }
    }
    #[doc = "SETAASA"]
    pub mod AASA {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "SETAASA not supported"]
            pub const NOTSUPPORTED: u32 = 0;
            #[doc = "SETAASA supported"]
            pub const SUPPORTED: u32 = 0x01;
        }
    }
    #[doc = "Target-Target(s)-Tunnel Subscriber Capable"]
    pub mod SSTSUB {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not subscriber capable"]
            pub const NOTSUPPORTED: u32 = 0;
            #[doc = "Subscriber capable"]
            pub const SUPPORTED: u32 = 0x01;
        }
    }
    #[doc = "Target-Target(s)-Tunnel Write Capable"]
    pub mod SSTWR {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not write capable"]
            pub const NOTSUPPORTED: u32 = 0;
            #[doc = "Write capable"]
            pub const SUPPORTED: u32 = 0x01;
        }
    }
}
#[doc = "Target Capabilities"]
pub mod SCAPABILITIES {
    #[doc = "ID 48b Handler"]
    pub mod IDENA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Application"]
            pub const APPLICATION: u32 = 0;
            #[doc = "Hardware"]
            pub const HW: u32 = 0x01;
            #[doc = "Hardware, but the I3C module instance handles ID 48b"]
            pub const HW_BUT: u32 = 0x02;
            #[doc = "A part number register (PARTNO)"]
            pub const PARTNO: u32 = 0x03;
        }
    }
    #[doc = "ID Register"]
    pub mod IDREG {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "All ID register features disabled"]
            pub const ALL_DISABLED: u32 = 0;
            #[doc = "ID Instance is a register; used if there is no PARTNO register"]
            pub const ID_INSTANCE: u32 = 0x01;
        }
    }
    #[doc = "High Data Rate Support"]
    pub mod HDRSUPP {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No HDR modes supported"]
            pub const NO_HDR: u32 = 0;
            #[doc = "DDR mode supported"]
            pub const DDR: u32 = 0x01;
        }
    }
    #[doc = "Controller"]
    pub mod MASTER {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not supported"]
            pub const MASTERNOTSUPPORTED: u32 = 0;
            #[doc = "Supported"]
            pub const MASTERSUPPORTED: u32 = 0x01;
        }
    }
    #[doc = "Static Address"]
    pub mod SADDR {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No static address"]
            pub const NO_STATIC: u32 = 0;
            #[doc = "Static address is fixed in hardware"]
            pub const STATIC: u32 = 0x01;
            #[doc = "Hardware controls the static address dynamically (for example, from the pin strap)"]
            pub const HW_CONTROL: u32 = 0x02;
            #[doc = "SCONFIG register supplies the static address"]
            pub const CONFIG: u32 = 0x03;
        }
    }
    #[doc = "Common Command Codes Handling"]
    pub mod CCCHANDLE {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "All handling features disabled"]
            pub const ALL_DISABLED: u32 = 0;
            #[doc = "The I3C module manages events, activities, status, HDR, and if enabled for it, ID and static-address-related items"]
            pub const BLOCK_HANDLE: u32 = 0x01;
        }
    }
    #[doc = "In-Band Interrupts, Controller Requests, Hot-Join Events"]
    pub mod IBI_MR_HJ {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Application cannot generate IBI, CR, or HJ"]
            pub const ALL_DISABLED: u32 = 0;
            #[doc = "Application can generate an IBI"]
            pub const IBI: u32 = 0x01;
        }
    }
    #[doc = "Time Control"]
    pub mod TIMECTRL {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No time control supported"]
            pub const NO_TIME_CONTROL_TYPE: u32 = 0;
            #[doc = "At least one time-control type supported"]
            pub const ATLEAST1_TIME_CONTROL: u32 = 0x01;
        }
    }
    #[doc = "External FIFO"]
    pub mod EXTFIFO {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No external FIFO available"]
            pub const NO_EXT_FIFO: u32 = 0;
            #[doc = "Standard available or free external FIFO"]
            pub const STD_EXT_FIFO: u32 = 0x01;
            #[doc = "Request track external FIFO"]
            pub const REQUEST_EXT_FIFO: u32 = 0x02;
        }
    }
    #[doc = "FIFO Transmit"]
    pub mod FIFOTX {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Two"]
            pub const FIFO_2BYTE: u32 = 0;
            #[doc = "Four"]
            pub const FIFO_4BYTE: u32 = 0x01;
            #[doc = "Eight"]
            pub const FIFO_8BYTE: u32 = 0x02;
            #[doc = "16 or larger"]
            pub const FIFO_16BYTE: u32 = 0x03;
        }
    }
    #[doc = "FIFO Receive"]
    pub mod FIFORX {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Two or three"]
            pub const FIFO_2BYTE: u32 = 0;
            #[doc = "Four"]
            pub const FIFO_4BYTE: u32 = 0x01;
            #[doc = "Eight"]
            pub const FIFO_8BYTE: u32 = 0x02;
            #[doc = "16 or larger"]
            pub const FIFO_16BYTE: u32 = 0x03;
        }
    }
    #[doc = "Interrupts"]
    pub mod INT {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not supported"]
            pub const INTERRUPTSNO: u32 = 0;
            #[doc = "Supported"]
            pub const INTERRUPTSYES: u32 = 0x01;
        }
    }
    #[doc = "Direct Memory Access"]
    pub mod DMA {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not supported"]
            pub const DMANO: u32 = 0;
            #[doc = "Supported"]
            pub const DMAYES: u32 = 0x01;
        }
    }
}
#[doc = "Target Maximum Limits"]
pub mod SMAXLIMITS {
    #[doc = "Maximum Read Length"]
    pub mod MAXRD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Maximum Write Length"]
    pub mod MAXWR {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Target ID Part Number"]
pub mod SIDPARTNO {
    #[doc = "Part Number"]
    pub mod PARTNO {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Target ID Extension"]
pub mod SIDEXT {
    #[doc = "Device Characteristic Register"]
    pub mod DCR {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Bus Characteristics Register"]
    pub mod BCR {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Target Vendor ID"]
pub mod SVENDORID {
    #[doc = "Vendor ID"]
    pub mod VID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x7fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Target Time Control Clock"]
pub mod STCCLOCK {
    #[doc = "Clock Accuracy"]
    pub mod ACCURACY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Clock Frequency"]
    pub mod FREQ {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Target Message Map Address"]
pub mod SMSGMAPADDR {
    #[doc = "Matched Address Index"]
    pub mod MAPLAST {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Last Static Address Matched"]
    pub mod LASTSTATIC {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "I3C dynamic address"]
            pub const I3C: u32 = 0;
            #[doc = "I2C static address"]
            pub const I2C: u32 = 0x01;
        }
    }
    #[doc = "Matched Previous Address Index 1"]
    pub mod MAPLASTM1 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Matched Previous Index 2"]
    pub mod MAPLASTM2 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Controller Control"]
pub mod MCTRL {
    #[doc = "Request"]
    pub mod REQUEST {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "NONE"]
            pub const NONE: u32 = 0;
            #[doc = "EMITSTARTADDR"]
            pub const EMITSTARTADDR: u32 = 0x01;
            #[doc = "EMITSTOP"]
            pub const EMITSTOP: u32 = 0x02;
            #[doc = "IBIACKNACK"]
            pub const IBIACKNACK: u32 = 0x03;
            #[doc = "PROCESSDAA"]
            pub const PROCESSDAA: u32 = 0x04;
            #[doc = "Force Exit and Target Reset"]
            pub const FORCEEXIT: u32 = 0x06;
            #[doc = "AUTOIBI"]
            pub const AUTOIBI: u32 = 0x07;
        }
    }
    #[doc = "Bus Type with EmitStartAddr"]
    pub mod TYPE {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "I3C"]
            pub const I3C: u32 = 0;
            #[doc = "I2C"]
            pub const I2C: u32 = 0x01;
            #[doc = "DDR"]
            pub const DDR: u32 = 0x02;
        }
    }
    #[doc = "In-Band Interrupt Response"]
    pub mod IBIRESP {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ACK (acknowledge)"]
            pub const ACK: u32 = 0;
            #[doc = "NACK (reject)"]
            pub const NACK: u32 = 0x01;
            #[doc = "Acknowledge with mandatory byte"]
            pub const ACK_WITH_MANDATORY: u32 = 0x02;
            #[doc = "Manual"]
            pub const MANUAL: u32 = 0x03;
        }
    }
    #[doc = "Direction"]
    pub mod DIR {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Write"]
            pub const DIRWRITE: u32 = 0;
            #[doc = "Read"]
            pub const DIRREAD: u32 = 0x01;
        }
    }
    #[doc = "Address"]
    pub mod ADDR {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x7f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Read Terminate Counter"]
    pub mod RDTERM {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Controller Status"]
pub mod MSTATUS {
    #[doc = "State of the Controller"]
    pub mod STATE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "IDLE (bus has stopped)"]
            pub const IDLE: u32 = 0;
            #[doc = "SLVREQ (target request)"]
            pub const SLVREQ: u32 = 0x01;
            #[doc = "MSGSDR"]
            pub const MSGSDR: u32 = 0x02;
            #[doc = "NORMACT"]
            pub const NORMACT: u32 = 0x03;
            #[doc = "MSGDDR"]
            pub const DDR: u32 = 0x04;
            #[doc = "DAA"]
            pub const DAA: u32 = 0x05;
            #[doc = "IBIACK"]
            pub const IBIACK: u32 = 0x06;
            #[doc = "IBIRCV"]
            pub const IBIRCV: u32 = 0x07;
        }
    }
    #[doc = "Between"]
    pub mod BETWEEN {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Inactive (for other cases)"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Active"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "Not Acknowledged"]
    pub mod NACKED {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not NACKed"]
            pub const NOT_NACKED: u32 = 0;
            #[doc = "NACKed (not acknowledged)"]
            pub const NACKED: u32 = 0x01;
        }
    }
    #[doc = "In-Band Interrupt (IBI) Type"]
    pub mod IBITYPE {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "NONE (no IBI: this status occurs when MSTATUS\\[IBIWON\\] becomes 0)"]
            pub const NONE: u32 = 0;
            #[doc = "IBI"]
            pub const IBI: u32 = 0x01;
            #[doc = "CR"]
            pub const MR: u32 = 0x02;
            #[doc = "HJ"]
            pub const HJ: u32 = 0x03;
        }
    }
    #[doc = "Target Start Flag"]
    pub mod SLVSTART {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {
            #[doc = "Target not requesting START"]
            pub const NOT_START: u32 = 0;
            #[doc = "Target requesting START"]
            pub const START: u32 = 0x01;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Controller Control Done Flag"]
    pub mod MCTRLDONE {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {
            #[doc = "Not done"]
            pub const NOT_DONE: u32 = 0;
            #[doc = "Done"]
            pub const DONE: u32 = 0x01;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Complete Flag"]
    pub mod COMPLETE {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {
            #[doc = "Not complete"]
            pub const NOT_COMPLETE: u32 = 0;
            #[doc = "Complete"]
            pub const COMPLETE: u32 = 0x01;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "RXPEND"]
    pub mod RXPEND {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No receive message pending"]
            pub const IDLE: u32 = 0;
            #[doc = "Receive message pending"]
            pub const PENDING: u32 = 0x01;
        }
    }
    #[doc = "TX Buffer or FIFO Not Full"]
    pub mod TXNOTFULL {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Receive buffer or FIFO full"]
            pub const FULL: u32 = 0;
            #[doc = "Receive buffer or FIFO not full"]
            pub const NOTFULL: u32 = 0x01;
        }
    }
    #[doc = "In-Band Interrupt (IBI) Won Flag"]
    pub mod IBIWON {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {
            #[doc = "No IBI arbitration won"]
            pub const NOT_WON: u32 = 0;
            #[doc = "IBI arbitration won"]
            pub const WON: u32 = 0x01;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Error or Warning"]
    pub mod ERRWARN {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No error or warning"]
            pub const NO_ERROR: u32 = 0;
            #[doc = "Error or warning"]
            pub const ERROR: u32 = 0x01;
        }
    }
    #[doc = "Module is now Controller Flag"]
    pub mod NOWMASTER {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {
            #[doc = "Not a controller"]
            pub const NOT_MASTER: u32 = 0;
            #[doc = "Controller"]
            pub const MASTER: u32 = 0x01;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "IBI Address"]
    pub mod IBIADDR {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x7f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Controller In-band Interrupt Registry and Rules"]
pub mod MIBIRULES {
    #[doc = "ADDR0"]
    pub mod ADDR0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ADDR1"]
    pub mod ADDR1 {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ADDR2"]
    pub mod ADDR2 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ADDR3"]
    pub mod ADDR3 {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ADDR4"]
    pub mod ADDR4 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Most Significant Address Bit is 0"]
    pub mod MSB0 {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MSB is not 0"]
            pub const DISABLE: u32 = 0;
            #[doc = "MSB is 0"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "No IBI byte"]
    pub mod NOBYTE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "With mandatory IBI byte"]
            pub const IBIBYTE: u32 = 0;
            #[doc = "Without mandatory IBI byte"]
            pub const NO_IBIBYTE: u32 = 0x01;
        }
    }
}
#[doc = "Controller Interrupt Set"]
pub mod MINTSET {
    #[doc = "Target Start Interrupt Enable"]
    pub mod SLVSTART {
        pub const offset: u32 = 8;
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
    #[doc = "Controller Control Done Interrupt Enable"]
    pub mod MCTRLDONE {
        pub const offset: u32 = 9;
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
    #[doc = "Completed Message Interrupt Enable"]
    pub mod COMPLETE {
        pub const offset: u32 = 10;
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
    #[doc = "Receive Pending Interrupt Enable"]
    pub mod RXPEND {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Transmit Buffer/FIFO Not Full Interrupt Enable"]
    pub mod TXNOTFULL {
        pub const offset: u32 = 12;
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
    #[doc = "IBI Won Interrupt Enable"]
    pub mod IBIWON {
        pub const offset: u32 = 13;
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
    #[doc = "Error or Warning (ERRWARN) Interrupt Enable"]
    pub mod ERRWARN {
        pub const offset: u32 = 15;
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
    #[doc = "Now Controller Interrupt Enable"]
    pub mod NOWMASTER {
        pub const offset: u32 = 19;
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
}
#[doc = "Controller Interrupt Clear"]
pub mod MINTCLR {
    #[doc = "SLVSTART Interrupt Enable Clear Flag"]
    pub mod SLVSTART {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {
            #[doc = "No effect"]
            pub const NONE: u32 = 0;
            #[doc = "Interrupt enable cleared"]
            pub const CLEAR: u32 = 0x01;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "MCTRLDONE Interrupt Enable Clear Flag"]
    pub mod MCTRLDONE {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {
            #[doc = "No effect"]
            pub const NONE: u32 = 0;
            #[doc = "Interrupt enable cleared"]
            pub const CLEAR: u32 = 0x01;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "COMPLETE Interrupt Enable Clear Flag"]
    pub mod COMPLETE {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {
            #[doc = "No effect"]
            pub const NONE: u32 = 0;
            #[doc = "Interrupt enable cleared"]
            pub const CLEAR: u32 = 0x01;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "RXPEND Interrupt Enable Clear Flag"]
    pub mod RXPEND {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {
            #[doc = "No effect"]
            pub const NONE: u32 = 0;
            #[doc = "Interrupt enable cleared"]
            pub const CLEAR: u32 = 0x01;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TXNOTFULL Interrupt Enable Clear Flag"]
    pub mod TXNOTFULL {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {
            #[doc = "No effect"]
            pub const NONE: u32 = 0;
            #[doc = "Interrupt enable cleared"]
            pub const CLEAR: u32 = 0x01;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "IBIWON Interrupt Enable Clear Flag"]
    pub mod IBIWON {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {
            #[doc = "No effect"]
            pub const NONE: u32 = 0;
            #[doc = "Interrupt enable cleared"]
            pub const CLEAR: u32 = 0x01;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ERRWARN Interrupt Enable Clear Flag"]
    pub mod ERRWARN {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {
            #[doc = "No effect"]
            pub const NONE: u32 = 0;
            #[doc = "Interrupt enable cleared"]
            pub const CLEAR: u32 = 0x01;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "NOWCONTROLLER Interrupt Enable Clear Flag"]
    pub mod NOWMASTER {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {
            #[doc = "No effect"]
            pub const NONE: u32 = 0;
            #[doc = "Interrupt enable cleared"]
            pub const CLEAR: u32 = 0x01;
        }
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Controller Interrupt Mask"]
pub mod MINTMASKED {
    #[doc = "SLVSTART Interrupt Mask"]
    pub mod SLVSTART {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const NOT_ENABLED: u32 = 0;
            #[doc = "Enabled"]
            pub const ENABLED: u32 = 0x01;
        }
    }
    #[doc = "MCTRLDONE Interrupt Mask"]
    pub mod MCTRLDONE {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const NOT_ENABLED: u32 = 0;
            #[doc = "Enabled"]
            pub const ENABLED: u32 = 0x01;
        }
    }
    #[doc = "COMPLETE Interrupt Mask"]
    pub mod COMPLETE {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const NOT_ENABLED: u32 = 0;
            #[doc = "Enabled"]
            pub const ENABLED: u32 = 0x01;
        }
    }
    #[doc = "RXPEND Interrupt Mask"]
    pub mod RXPEND {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TXNOTFULL Interrupt Mask"]
    pub mod TXNOTFULL {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const NOT_ENABLED: u32 = 0;
            #[doc = "Enabled"]
            pub const ENABLED: u32 = 0x01;
        }
    }
    #[doc = "IBIWON Interrupt Mask"]
    pub mod IBIWON {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const NOT_ENABLED: u32 = 0;
            #[doc = "Enabled"]
            pub const ENABLED: u32 = 0x01;
        }
    }
    #[doc = "ERRWARN Interrupt Mask"]
    pub mod ERRWARN {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const NOT_ENABLED: u32 = 0;
            #[doc = "Enabled"]
            pub const ENABLED: u32 = 0x01;
        }
    }
    #[doc = "NOWCONTROLLER Interrupt Mask"]
    pub mod NOWMASTER {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const NOT_ENABLED: u32 = 0;
            #[doc = "Enabled"]
            pub const ENABLED: u32 = 0x01;
        }
    }
}
#[doc = "Controller Errors and Warnings"]
pub mod MERRWARN {
    #[doc = "Not Acknowledge Error Flag"]
    pub mod NACK {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {
            #[doc = "No error"]
            pub const NO_ERROR: u32 = 0;
            #[doc = "Error"]
            pub const ERROR: u32 = 0x01;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Write Abort Error Flag"]
    pub mod WRABT {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {
            #[doc = "No error"]
            pub const NO_ERROR: u32 = 0;
            #[doc = "Error"]
            pub const ERROR: u32 = 0x01;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "High Data Rate Parity Flag"]
    pub mod HPAR {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {
            #[doc = "No error"]
            pub const NO_ERROR: u32 = 0;
            #[doc = "Error"]
            pub const ERROR: u32 = 0x01;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "High Data Rate CRC Error Flag"]
    pub mod HCRC {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {
            #[doc = "No error"]
            pub const NO_ERROR: u32 = 0;
            #[doc = "Error"]
            pub const ERROR: u32 = 0x01;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Overread Error Flag"]
    pub mod OREAD {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {
            #[doc = "No error"]
            pub const NO_ERROR: u32 = 0;
            #[doc = "Error"]
            pub const ERROR: u32 = 0x01;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Overwrite Error Flag"]
    pub mod OWRITE {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {
            #[doc = "No error"]
            pub const NO_ERROR: u32 = 0;
            #[doc = "Error"]
            pub const ERROR: u32 = 0x01;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Message Error Flag"]
    pub mod MSGERR {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {
            #[doc = "No error"]
            pub const NO_ERROR: u32 = 0;
            #[doc = "Error"]
            pub const ERROR: u32 = 0x01;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Invalid Request Error Flag"]
    pub mod INVREQ {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {
            #[doc = "No error"]
            pub const NO_ERROR: u32 = 0;
            #[doc = "Error"]
            pub const ERROR: u32 = 0x01;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Timeout Error Flag"]
    pub mod TIMEOUT {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {
            #[doc = "No error"]
            pub const NO_ERROR: u32 = 0;
            #[doc = "Error"]
            pub const ERROR: u32 = 0x01;
        }
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Controller DMA Control"]
pub mod MDMACTRL {
    #[doc = "DMA from Bus"]
    pub mod DMAFB {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "DMA not used"]
            pub const NOT_USED: u32 = 0;
            #[doc = "Enable DMA for one frame"]
            pub const ENABLE_ONE_FRAME: u32 = 0x01;
            #[doc = "Enable DMA until DMA is turned off"]
            pub const ENABLE: u32 = 0x02;
        }
    }
    #[doc = "DMA to Bus"]
    pub mod DMATB {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "DMA not used"]
            pub const NOT_USED: u32 = 0;
            #[doc = "Enable DMA for one frame (ended by DMA or terminated)"]
            pub const ENABLE_ONE_FRAME: u32 = 0x01;
            #[doc = "Enable DMA until DMA is turned off"]
            pub const ENABLE: u32 = 0x02;
        }
    }
    #[doc = "DMA Width"]
    pub mod DMAWIDTH {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Byte"]
            pub const BYTE_0: u32 = 0;
            #[doc = "Byte"]
            pub const BYTE_1: u32 = 0x01;
            #[doc = "Halfword (16 bits)"]
            pub const HALF_WORD: u32 = 0x02;
        }
    }
}
#[doc = "Controller Data Control"]
pub mod MDATACTRL {
    #[doc = "Flush To-Bus Buffer or FIFO"]
    pub mod FLUSHTB {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No action"]
            pub const NO_ACTION: u32 = 0;
            #[doc = "Flush the buffer"]
            pub const FLUSH: u32 = 0x01;
        }
    }
    #[doc = "Flush From-Bus Buffer or FIFO"]
    pub mod FLUSHFB {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No action"]
            pub const NO_ACTION: u32 = 0;
            #[doc = "Flush the buffer"]
            pub const FLUSH: u32 = 0x01;
        }
    }
    #[doc = "Unlock"]
    pub mod UNLOCK {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Locked"]
            pub const DISABLED: u32 = 0;
            #[doc = "Unlocked"]
            pub const ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Transmit Trigger Level"]
    pub mod TXTRIG {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Trigger when empty"]
            pub const EMPTY: u32 = 0;
            #[doc = "Trigger when 1/4 full or less"]
            pub const QUARTER_OR_LESS: u32 = 0x01;
            #[doc = "Trigger when 1/2 full or less"]
            pub const HALF_OR_LESS: u32 = 0x02;
            #[doc = "Trigger when 1 less than full or less (default)"]
            pub const FULL_OR_LESS: u32 = 0x03;
        }
    }
    #[doc = "Receive Trigger Level"]
    pub mod RXTRIG {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Trigger when not empty"]
            pub const NOT_EMPTY: u32 = 0;
            #[doc = "Trigger when 1/4 full or more"]
            pub const QUARTER_OR_MORE: u32 = 0x01;
            #[doc = "Trigger when 1/2 full or more"]
            pub const HALF_OR_MORE: u32 = 0x02;
            #[doc = "Trigger when 3/4 full or more"]
            pub const THREE_QUARTER_OR_MORE: u32 = 0x03;
        }
    }
    #[doc = "Transmit Byte Count"]
    pub mod TXCOUNT {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Receive Byte Count"]
    pub mod RXCOUNT {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Transmit is Full"]
    pub mod TXFULL {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not full"]
            pub const NOT_FULL: u32 = 0;
            #[doc = "Full"]
            pub const FULL: u32 = 0x01;
        }
    }
    #[doc = "Receive is Empty"]
    pub mod RXEMPTY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not empty"]
            pub const NOT_EMPTY: u32 = 0;
            #[doc = "Empty"]
            pub const EMPTY: u32 = 0x01;
        }
    }
}
#[doc = "Controller Write Data Byte"]
pub mod MWDATAB {
    #[doc = "Data Byte"]
    pub mod VALUE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "End of Message"]
    pub mod END {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not the end"]
            pub const NOT_END: u32 = 0;
            #[doc = "End"]
            pub const END: u32 = 0x01;
        }
    }
    #[doc = "End of Message ALSO"]
    pub mod END_ALSO {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not the end"]
            pub const NOT_END: u32 = 0;
            #[doc = "End"]
            pub const END: u32 = 0x01;
        }
    }
}
#[doc = "Controller Write Data Byte End"]
pub mod MWDATABE {
    #[doc = "Data"]
    pub mod VALUE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Controller Write Data Halfword"]
pub mod MWDATAH {
    #[doc = "Data Byte 0"]
    pub mod DATA0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data Byte 1"]
    pub mod DATA1 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "End of Message"]
    pub mod END {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not the end"]
            pub const NOT_END: u32 = 0;
            #[doc = "End"]
            pub const END: u32 = 0x01;
        }
    }
}
#[doc = "Controller Write Data Halfword End"]
pub mod MWDATAHE {
    #[doc = "Data Byte 0"]
    pub mod DATA0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data Byte 1"]
    pub mod DATA1 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Controller Read Data Byte"]
pub mod MRDATAB {
    #[doc = "Value"]
    pub mod VALUE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Controller Read Data Halfword"]
pub mod MRDATAH {
    #[doc = "Low Byte"]
    pub mod LSB {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "High Byte"]
    pub mod MSB {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Controller Write Byte Data 1 (to Bus)"]
pub mod MWDATAB1 {
    #[doc = "Value"]
    pub mod VALUE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Controller Write Message Control in SDR mode"]
pub mod MWMSG_SDR_CONTROL {
    #[doc = "Direction"]
    pub mod DIR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Write"]
            pub const WRITE: u32 = 0;
            #[doc = "Read"]
            pub const READ: u32 = 0x01;
        }
    }
    #[doc = "Address"]
    pub mod ADDR {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x7f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "End of SDR Message"]
    pub mod END {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not the end"]
            pub const NOT_END: u32 = 0;
            #[doc = "End"]
            pub const END: u32 = 0x01;
        }
    }
    #[doc = "I2C"]
    pub mod I2C {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "I3C message"]
            pub const I3CMESSAGE: u32 = 0;
            #[doc = "I2C message"]
            pub const I2CMESSAGE: u32 = 0x01;
        }
    }
    #[doc = "Length"]
    pub mod LEN {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Controller Read Message in SDR mode"]
pub mod MRMSG_SDR {
    #[doc = "Data"]
    pub mod DATA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Controller Write Message in DDR mode: First Control Word"]
pub mod MWMSG_DDR_CONTROL {
    #[doc = "Address Command"]
    pub mod ADDRCMD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Controller Read Message in DDR mode"]
pub mod MRMSG_DDR {
    #[doc = "Data"]
    pub mod DATA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Controller Dynamic Address"]
pub mod MDYNADDR {
    #[doc = "Dynamic Address Valid"]
    pub mod DAVALID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No valid DA assigned"]
            pub const NO_VALID: u32 = 0;
            #[doc = "Valid DA assigned"]
            pub const VALID: u32 = 0x01;
        }
    }
    #[doc = "Dynamic Address"]
    pub mod DADDR {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x7f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Timing Rules for Target Reset Recovery"]
pub mod SRSTACTTIME {
    #[doc = "Time to Recover from the I3C Peripheral"]
    pub mod PERRSTTIM {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Time to Recover from Chip Reset"]
    pub mod SYSRSTTIM {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "CCC Mask for Unhandled CCCs"]
pub mod SCCCMASK {
    #[doc = "Base"]
    pub mod BASE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Suppressed"]
            pub const DISABLE: u32 = 0;
            #[doc = "Passed to application"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "BASEBX"]
    pub mod BASEBX {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Suppressed"]
            pub const DISABLE: u32 = 0;
            #[doc = "Passed to application"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "BASEDX"]
    pub mod BASEDX {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Suppressed"]
            pub const DISABLE: u32 = 0;
            #[doc = "Passed to application"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "MEXTB"]
    pub mod MEXTB {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Suppressed"]
            pub const DISABLE: u32 = 0;
            #[doc = "Passed to application"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "MEXTD"]
    pub mod MEXTD {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Suppressed"]
            pub const DISABLE: u32 = 0;
            #[doc = "Passed to application"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "VENDB"]
    pub mod VENDB {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Suppressed"]
            pub const DISABLE: u32 = 0;
            #[doc = "Passed to application"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "VENDD"]
    pub mod VENDD {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Suppressed"]
            pub const DISABLE: u32 = 0;
            #[doc = "Passed to application"]
            pub const ENABLE: u32 = 0x01;
        }
    }
}
#[doc = "Target Errors and Warnings Mask"]
pub mod SERRWARNMASK {
    #[doc = "ORUN Mask"]
    pub mod ORUN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Deny"]
            pub const DENY: u32 = 0;
            #[doc = "Allow"]
            pub const ALLOW: u32 = 0x01;
        }
    }
    #[doc = "URUN Mask"]
    pub mod URUN {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Deny"]
            pub const DENY: u32 = 0;
            #[doc = "Allow"]
            pub const ALLOW: u32 = 0x01;
        }
    }
    #[doc = "URUNNACK Mask"]
    pub mod URUNNACK {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Deny"]
            pub const DENY: u32 = 0;
            #[doc = "Allow"]
            pub const ALLOW: u32 = 0x01;
        }
    }
    #[doc = "TERM Mask"]
    pub mod TERM {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Deny"]
            pub const DENY: u32 = 0;
            #[doc = "Allow"]
            pub const ALLOW: u32 = 0x01;
        }
    }
    #[doc = "INVSTART Mask"]
    pub mod INVSTART {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Deny"]
            pub const DENY: u32 = 0;
            #[doc = "Allow"]
            pub const ALLOW: u32 = 0x01;
        }
    }
    #[doc = "SPAR Mask"]
    pub mod SPAR {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Deny"]
            pub const DENY: u32 = 0;
            #[doc = "Allow"]
            pub const ALLOW: u32 = 0x01;
        }
    }
    #[doc = "HPAR Mask"]
    pub mod HPAR {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Deny"]
            pub const DENY: u32 = 0;
            #[doc = "Allow"]
            pub const ALLOW: u32 = 0x01;
        }
    }
    #[doc = "HCRC Mask"]
    pub mod HCRC {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Deny"]
            pub const DENY: u32 = 0;
            #[doc = "Allow"]
            pub const ALLOW: u32 = 0x01;
        }
    }
    #[doc = "S0S1 Mask"]
    pub mod S0S1 {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Deny"]
            pub const DENY: u32 = 0;
            #[doc = "Allow"]
            pub const ALLOW: u32 = 0x01;
        }
    }
}
#[doc = "Map Feature Control 0"]
pub mod SMAPCTRL0 {
    #[doc = "Enable Primary Dynamic Address"]
    pub mod ENA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Dynamic Address"]
    pub mod DA {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x7f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Cause"]
    pub mod CAUSE {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No information (this value occurs when not configured to write DA)"]
            pub const NONE: u32 = 0;
            #[doc = "Set using ENTDAA"]
            pub const ENTDAA: u32 = 0x01;
            #[doc = "Set using SETDASA, SETAASA, or SETNEWDA"]
            pub const SETDASA: u32 = 0x02;
            #[doc = "Cleared using RSTDAA"]
            pub const RSTDAA: u32 = 0x03;
            #[doc = "Auto MAP change happened last"]
            pub const AUTOMAP: u32 = 0x04;
        }
    }
}
#[doc = "Map Feature Control 1"]
pub mod SMAPCTRL1 {
    #[doc = "Enable"]
    pub mod ENA {
        pub const offset: u32 = 0;
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
    #[doc = "Address"]
    pub mod ADDR {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x7f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "MAP Static Address"]
    pub mod MAPSA {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "I3C dynamic address"]
            pub const DISABLE: u32 = 0;
            #[doc = "Static address (I2C style)"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Static Address 10-Bit Extension"]
    pub mod SA10B {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Not Acknowledged"]
    pub mod NACK {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Do not always NACK messages"]
            pub const NOT_ALWAYS_NACK: u32 = 0;
            #[doc = "Always NACK messages"]
            pub const ALWAYS_NACK: u32 = 0x01;
        }
    }
}
#[doc = "Extended IBI Data 1"]
pub mod IBIEXT1 {
    #[doc = "Count"]
    pub mod CNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Maximum"]
    pub mod MAX {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Extra Byte 1"]
    pub mod EXT1 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Extra Byte 2"]
    pub mod EXT2 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Extra Byte 3"]
    pub mod EXT3 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Extended IBI Data 2"]
pub mod IBIEXT2 {
    #[doc = "Extra Byte 4"]
    pub mod EXT4 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Extra Byte 5"]
    pub mod EXT5 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Extra Byte 6"]
    pub mod EXT6 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Extra Byte 7"]
    pub mod EXT7 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
