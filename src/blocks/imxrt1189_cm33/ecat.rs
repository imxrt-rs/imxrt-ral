#[doc = "ETHERCAT"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "Type"]
    pub TYPE: crate::RORegister<u8>,
    #[doc = "Revision"]
    pub REVISION: crate::RORegister<u8>,
    #[doc = "Build"]
    pub BUILD: crate::RORegister<u16>,
    #[doc = "FMMUs supported"]
    pub FMMUS_SUPPORTED: crate::RORegister<u8>,
    #[doc = "SyncManagers supported"]
    pub SYNCMANAGERS_SUPPORTED: crate::RORegister<u8>,
    #[doc = "RAM Size"]
    pub RAM_SIZE: crate::RORegister<u8>,
    #[doc = "Port configuration"]
    pub PORT_DESCRIPTOR: crate::RORegister<u8>,
    #[doc = "Register ESC Features supported"]
    pub ESC_FEATURES_SUPPORTED: crate::RORegister<u16>,
    _reserved0: [u8; 0x06],
    #[doc = "Configured Station Address"]
    pub CONFIGURED_STATION_ADDRESS: crate::RWRegister<u16>,
    #[doc = "Configured Station Alias"]
    pub CONFIGURED_STATION_ALIAS_PDI: crate::RWRegister<u16>,
    _reserved1: [u8; 0x0c],
    #[doc = "Register Write Enable"]
    pub REGISTER_WRITE_ENABLE: crate::RWRegister<u8>,
    #[doc = "Register Write Protection"]
    pub REGISTER_WRITE_PROTECTION: crate::RWRegister<u8>,
    _reserved2: [u8; 0x0e],
    #[doc = "ESC Write Enable"]
    pub ESC_WRITE_ENABLE: crate::RWRegister<u8>,
    #[doc = "ESC Write Protection"]
    pub ESC_WRITE_PROTECTION: crate::RWRegister<u8>,
    _reserved3: [u8; 0x0e],
    #[doc = "ESC Reset ECAT WRITE"]
    pub ESC_RESET_ECAT_WRITE: crate::RWRegister<u8>,
    #[doc = "ESC Reset PDI WRITE"]
    pub ESC_RESET_PDI_WRITE_PDI: crate::RWRegister<u8>,
    _reserved4: [u8; 0xbe],
    #[doc = "ESC DL Control"]
    pub ESC_DL_CONTROL: crate::RWRegister<u32>,
    _reserved5: [u8; 0x04],
    #[doc = "Physical Read Write Offset"]
    pub PHYSICAL_READ_WRITE_OFFSET: crate::RWRegister<u16>,
    _reserved6: [u8; 0x06],
    #[doc = "ESC DL Status"]
    pub ESC_DL_STATUS: crate::RORegister<u16>,
    _reserved7: [u8; 0x0e],
    #[doc = "AL Control"]
    pub AL_CONTROL: crate::RWRegister<u16>,
    _reserved8: [u8; 0x0e],
    #[doc = "AL Status"]
    pub AL_STATUS_PDI: crate::RWRegister<u16>,
    _reserved9: [u8; 0x02],
    #[doc = "AL Status Code"]
    pub AL_STATUS_CODE_PDI: crate::RWRegister<u16>,
    _reserved10: [u8; 0x02],
    #[doc = "RUN LED Override"]
    pub RUN_LED_OVERRIDE: crate::RWRegister<u8>,
    #[doc = "ERR LED Override"]
    pub ERR_LED_OVERRIDE: crate::RWRegister<u8>,
    _reserved11: [u8; 0x06],
    #[doc = "PDI Control"]
    pub PDI_CONTROL: crate::RORegister<u8>,
    #[doc = "ESC Configuration"]
    pub ESC_CONFIGURATION: crate::RORegister<u8>,
    _reserved12: [u8; 0x0c],
    #[doc = "PDI Information"]
    pub PDI_INFORMATION: crate::RORegister<u16>,
    #[doc = "Register PDI On-chip bus configuration"]
    pub PDI_ON_CHIP_BUS_CONFIGURATION: crate::RORegister<u8>,
    #[doc = "PDI Configuration Sync Latch 1 and 0 PDI Configuration"]
    pub SYNC_LATCH_1_AND_0_PDI_CONFIGURATION: crate::RORegister<u8>,
    #[doc = "Register PDI On-chip bus extended configuration."]
    pub PDI_ON_CHIP_BUS_EXTENDED_CONFIGURATION: crate::RORegister<u16>,
    _reserved13: [u8; 0xac],
    #[doc = "ECAT Event Mask"]
    pub ECAT_EVENT_MASK: crate::RWRegister<u16>,
    _reserved14: [u8; 0x02],
    #[doc = "PDI AL Event Mask"]
    pub PDI_AL_EVENT_MASK_PDI: crate::RWRegister<u32>,
    _reserved15: [u8; 0x08],
    #[doc = "ECAT Event Request"]
    pub ECAT_EVENT_REQUEST: crate::RORegister<u16>,
    _reserved16: [u8; 0x0e],
    #[doc = "AL Event request"]
    pub AL_EVENT_REQUEST: crate::RORegister<u32>,
    _reserved17: [u8; 0xe8],
    #[doc = "ECAT Processing Unit Error Counter"]
    pub ECAT_PROCESSING_UNIT_ERROR_COUNTER: crate::RWRegister<u8>,
    #[doc = "PDI Error counter"]
    pub PDI_ERROR_COUNTER: crate::RWRegister<u8>,
    #[doc = "ASYNCHRONOUS_SYNCHRONOUS_MICROCONTROLLER_PDI_ERROR_CODE."]
    pub ASYNCHRONOUS_SYNCHRONOUS_MICROCONTROLLER: crate::RORegister<u8>,
    _reserved18: [u8; 0xf1],
    #[doc = "Watchdog Divider"]
    pub WATCHDOG_DIVIDER: crate::RWRegister<u16>,
    _reserved19: [u8; 0x0e],
    #[doc = "Register Watchdog Time PDI"]
    pub WATCHDOG_TIME_PDI: crate::RWRegister<u16>,
    _reserved20: [u8; 0x0e],
    #[doc = "Regsister Watchdog Time Process Data"]
    pub WATCHDOG_TIME_PROCESS_DATA: crate::RWRegister<u16>,
    _reserved21: [u8; 0x1e],
    #[doc = "Watchdog Status Process Data"]
    pub WATCHDOG_STATUS_PROCESS_DATA: crate::RORegister<u16>,
    #[doc = "Watchdog Counter Process Data"]
    pub WATCHDOG_COUNTER_PROCESS_DATA: crate::RWRegister<u8>,
    #[doc = "Watchdog Counter PDI"]
    pub WATCHDOG_COUNTER_PDI: crate::RWRegister<u8>,
    _reserved22: [u8; 0xbc],
    #[doc = "EEPROM Configuration"]
    pub EEPROM_CONFIGURATION: crate::RWRegister<u8>,
    #[doc = "EEPROM PDI Access State"]
    pub REGISTER_EEPROM_PDI_ACCESS_STATE_PDI: crate::RWRegister<u8>,
    #[doc = "Register EEPROM Control/Status"]
    pub EEPROM_CONTROL_STATUS: crate::RWRegister<u16>,
    #[doc = "EEPROM Address"]
    pub EEPROM_ADDRESS: crate::RWRegister<u32>,
    #[doc = "EEPROM Data"]
    pub EEPROM_DATA: crate::RWRegister<u64>,
    #[doc = "MII Management Control/Status"]
    pub MII_MANAGEMENT_CONTROL_OR_STATUS: crate::RWRegister<u16>,
    #[doc = "PHY Address"]
    pub PHY_ADDRESS: crate::RWRegister<u8>,
    #[doc = "PHY Register Address"]
    pub PHY_REGISTER_ADDRESS: crate::RWRegister<u8>,
    #[doc = "PHY Data"]
    pub PHY_DATA: crate::RWRegister<u16>,
    #[doc = "MII Management ECAT Access State"]
    pub MII_MANAGEMENT_ECAT_ACCESS_STATE: crate::RWRegister<u8>,
    #[doc = "MII Management PDI Access State"]
    pub MII_MANAGEMENT_PDI_ACCESS_STATE: crate::RWRegister<u8>,
    #[doc = "PHY Port"]
    pub PHY_PORT_STATUS: [crate::RWRegister<u8>; 2usize],
    _reserved23: [u8; 0x03e6],
    #[doc = "Distributed Clocks Receive Times"]
    pub RECEIVE_TIMES: crate::RWRegister<u32>,
    #[doc = "Distributed Clocks Receive Time Port 1"]
    pub RECEIVE_TIME_PORT_1: crate::RORegister<u32>,
    _reserved24: [u8; 0x08],
    #[doc = "Register System Time"]
    pub SYSTEM_TIME: crate::RWRegister<u64>,
    #[doc = "Distributed Clocks Register Receive Time ECAT Processing Unit"]
    pub RECEIVE_TIME_ECAT_PROCESSING_UNIT: crate::RORegister<u64>,
    #[doc = "Register System Time Offset"]
    pub SYSTEM_TIME_OFFSET: crate::RWRegister<u64>,
    #[doc = "Register System Time Delay"]
    pub SYSTEM_TIME_DELAY: crate::RWRegister<u32>,
    #[doc = "Register System Time Difference"]
    pub SYSTEM_TIME_DIFFERENCE: crate::RORegister<u32>,
    #[doc = "Register Speed Counter Start"]
    pub SPEED_COUNTER_START: crate::RWRegister<u16>,
    #[doc = "Register Speed Counter Diff"]
    pub SPEED_COUNTER_DIFF: crate::RORegister<u16>,
    #[doc = "Register System Time Difference Filter Depth"]
    pub SYSTEM_TIME_DIFFERENCE_FILTER_DEPTH: crate::RWRegister<u8>,
    #[doc = "Register Speed Counter Filter Depth"]
    pub SPEED_COUNTER_FILTER_DEPTH: crate::RWRegister<u8>,
    _reserved25: [u8; 0x4a],
    #[doc = "Register Cyclic Unit Control"]
    pub CYCLIC_UNIT_CONTROL: crate::RWRegister<u8>,
    #[doc = "Register Activation register"]
    pub UNIT_ACTIVATION_REGISTER: crate::RWRegister<u8>,
    #[doc = "Register Pulse Length of SyncSignals"]
    pub UNI_PULSE_LENGTH_OF_SYNCSIGNALS: crate::RORegister<u16>,
    #[doc = "Register Activation Status"]
    pub UNIT_ACTIVATION_STATUS: crate::RORegister<u8>,
    _reserved26: [u8; 0x09],
    #[doc = "Register SYNC0 Status"]
    pub UNIT_SYNC0_STATUS: crate::RORegister<u8>,
    #[doc = "Register SYNC1 Status"]
    pub UNIT_SYNC1_STATUS: crate::RORegister<u8>,
    #[doc = "Register Start Time Cyclic Operation"]
    pub UNIT_START_TIME_CYCLIC_OPERATION: crate::RWRegister<u64>,
    #[doc = "Register Next SYNC1 Pulse"]
    pub UNIT_NEXT_SYNC1_PULSE: crate::RORegister<u64>,
    #[doc = "Register SYNC0 Cycle Time"]
    pub UNIT_SYNC0_CYCLE_TIME: crate::RWRegister<u32>,
    #[doc = "Register SYNC1 Cycle Time"]
    pub UNIT_SYNC1_CYCLE_TIME: crate::RWRegister<u32>,
    #[doc = "Register Latch0 Control"]
    pub LATCH0_CONTROL: crate::RWRegister<u8>,
    #[doc = "Register Latch1 Control"]
    pub LATCH1_CONTROL: crate::RWRegister<u8>,
    _reserved27: [u8; 0x04],
    #[doc = "Register Latch0 Status"]
    pub LATCH0_STATUS: crate::RORegister<u8>,
    #[doc = "Register Latch1 Status"]
    pub LATCH1_STATUS: crate::RORegister<u8>,
    #[doc = "Register Latch0 Time Positive Edge"]
    pub LATCH0_TIME_POSITIVE_EDGE: crate::RORegister<u64>,
    #[doc = "Register Latch0 Time Negative Edge"]
    pub LATCH0_TIME_NEGATIVE_EDGE: crate::RORegister<u64>,
    #[doc = "Register Latch1 Time Positive Edge"]
    pub LATCH1_TIME_POSITIVE_EDGE: crate::RORegister<u64>,
    #[doc = "Register Latch1 Time Negative Edge"]
    pub LATCH1_TIME_NEGATIVE_EDGE: crate::RORegister<u64>,
    _reserved28: [u8; 0x20],
    #[doc = "Register EtherCAT Buffer Change Event Time"]
    pub ETHERCAT_BUFFER_CHANGE_EVENT_TIME: crate::RORegister<u32>,
    _reserved29: [u8; 0x04],
    #[doc = "Register PDI Buffer Start Event Time"]
    pub PDI_BUFFER_START_EVENT_TIME: crate::RORegister<u32>,
    #[doc = "Register PDI Buffer Change Event Time"]
    pub PDI_BUFFER_CHANGE_EVENT_TIME: crate::RORegister<u32>,
    _reserved30: [u8; 0x0400],
    #[doc = "Register Product ID IP Core"]
    pub PRODUCT_ID_IP_CORE: crate::RORegister<u64>,
    #[doc = "Register Vendor ID IP Core"]
    pub VENDOR_ID_IP_CORE: crate::RORegister<u64>,
    _reserved31: [u8; 0x0100],
    #[doc = "Register General Purpose Outputs"]
    pub GENERAL_PURPOSE_OUTPUTS: crate::RWRegister<u16>,
    _reserved32: [u8; 0x06],
    #[doc = "Register General Purpose Inputs"]
    pub GENERAL_PURPOSE_INPUTS: crate::RORegister<u16>,
}
#[doc = "Type"]
pub mod TYPE {
    #[doc = "Type of EtherCAT controller"]
    pub mod BF0 {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Revision"]
pub mod REVISION {
    #[doc = "Revision of EtherCAT controller."]
    pub mod BF0 {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Build"]
pub mod BUILD {
    #[doc = "Build of EtherCAT controller"]
    pub mod BF0 {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "FMMUs supported"]
pub mod FMMUS_SUPPORTED {
    #[doc = "Number of supported FMMU channels (or entities)"]
    pub mod BF0 {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SyncManagers supported"]
pub mod SYNCMANAGERS_SUPPORTED {
    #[doc = "Number of supported SyncManager channels (or entities)"]
    pub mod BF0 {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "RAM Size"]
pub mod RAM_SIZE {
    #[doc = "Process Data RAM size supported in KByte"]
    pub mod BF0 {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port configuration"]
pub mod PORT_DESCRIPTOR {
    #[doc = "Port 0"]
    pub mod BF0 {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Port 1"]
    pub mod BF2 {
        pub const offset: u8 = 2;
        pub const mask: u8 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Register ESC Features supported"]
pub mod ESC_FEATURES_SUPPORTED {
    #[doc = "FMMU Operation:"]
    pub mod BF0 {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Bit oriented"]
            pub const BF_VAL_0: u16 = 0;
            #[doc = "Byte oriented"]
            pub const BF_VAL_1: u16 = 0x01;
        }
    }
    #[doc = "Unused register access:"]
    pub mod BF1 {
        pub const offset: u16 = 1;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "allowed"]
            pub const BF_VAL_0: u16 = 0;
            #[doc = "not supported"]
            pub const BF_VAL_1: u16 = 0x01;
        }
    }
    #[doc = "Distributed Clocks:"]
    pub mod BF2 {
        pub const offset: u16 = 2;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not available"]
            pub const BF_VAL_0: u16 = 0;
            #[doc = "Available"]
            pub const BF_VAL_1: u16 = 0x01;
        }
    }
    #[doc = "Distributed Clocks (width):"]
    pub mod BF3 {
        pub const offset: u16 = 3;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "32 bit"]
            pub const BF_VAL_0: u16 = 0;
            #[doc = "64 bit"]
            pub const BF_VAL_1: u16 = 0x01;
        }
    }
    #[doc = "Enhanced Link Detection MII:"]
    pub mod BF6 {
        pub const offset: u16 = 6;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not available"]
            pub const BF_VAL_0: u16 = 0;
            #[doc = "Available"]
            pub const BF_VAL_1: u16 = 0x01;
        }
    }
    #[doc = "Separate Handling of FCS Errors:"]
    pub mod BF7 {
        pub const offset: u16 = 7;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not supported"]
            pub const BF_VAL_0: u16 = 0;
            #[doc = "Supported, frames with wrong FCS and additional nibble will be counted separately in Forwarded RX Error Counter"]
            pub const BF_VAL_1: u16 = 0x01;
        }
    }
    #[doc = "Enhanced DC SYNC Activation"]
    pub mod BF8 {
        pub const offset: u16 = 8;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not available"]
            pub const BF_VAL_0: u16 = 0;
            #[doc = "Available"]
            pub const BF_VAL_1: u16 = 0x01;
        }
    }
    #[doc = "EtherCAT LRW command support:"]
    pub mod BF9 {
        pub const offset: u16 = 9;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Supported"]
            pub const BF_VAL_0: u16 = 0;
            #[doc = "Not supported"]
            pub const BF_VAL_1: u16 = 0x01;
        }
    }
    #[doc = "EtherCAT read/write command support (BRW, APRW, FPRW):"]
    pub mod BF10 {
        pub const offset: u16 = 10;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Supported"]
            pub const BF_VAL_0: u16 = 0;
            #[doc = "Not supported"]
            pub const BF_VAL_1: u16 = 0x01;
        }
    }
    #[doc = "Fixed FMMU/SyncManager configuration"]
    pub mod BF11 {
        pub const offset: u16 = 11;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Variable configuration"]
            pub const BF_VAL_0: u16 = 0;
            #[doc = "Fixed configuration (refer to documentation of supporting ESCs)"]
            pub const BF_VAL_1: u16 = 0x01;
        }
    }
}
#[doc = "Configured Station Address"]
pub mod CONFIGURED_STATION_ADDRESS {
    #[doc = "Address used for node addressing (FPRD/FPWR/FPRW/FRMW commands)."]
    pub mod BF0 {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Configured Station Alias"]
pub mod CONFIGURED_STATION_ALIAS_PDI {
    #[doc = "Alias Address used for node addressing (FPRD/FPWR/FPRW/FRMW commands)."]
    pub mod BF0 {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Register Write Enable"]
pub mod REGISTER_WRITE_ENABLE {
    #[doc = "Register Write Enable."]
    pub mod BF0 {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Register Write Protection"]
pub mod REGISTER_WRITE_PROTECTION {
    #[doc = "Register write protection."]
    pub mod BF0 {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Protection disabled"]
            pub const BF_VAL_0: u8 = 0;
            #[doc = "Protection enabled"]
            pub const BF_VAL_1: u8 = 0x01;
        }
    }
}
#[doc = "ESC Write Enable"]
pub mod ESC_WRITE_ENABLE {
    #[doc = "ESC Write Enable"]
    pub mod BF0 {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ESC Write Protection"]
pub mod ESC_WRITE_PROTECTION {
    #[doc = "Write protect:"]
    pub mod BF0 {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Protection disabled"]
            pub const BF_VAL_0: u8 = 0;
            #[doc = "Protection enabled"]
            pub const BF_VAL_1: u8 = 0x01;
        }
    }
}
#[doc = "ESC Reset ECAT WRITE"]
pub mod ESC_RESET_ECAT_WRITE {
    #[doc = "A reset is asserted after writing 0x52 ('R'), 0x45 ('E') and 0x53 ('S') in this register with 3 consecutive frames."]
    pub mod BF0 {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ESC Reset PDI WRITE"]
pub mod ESC_RESET_PDI_WRITE_PDI {
    #[doc = "A reset is asserted after writing 0x52 ('R'), 0x45 ('E') and 0x53 ('S') in this register with 3 consecutive commands."]
    pub mod BF0 {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ESC DL Control"]
pub mod ESC_DL_CONTROL {
    #[doc = "Forwarding Rule"]
    pub mod BF0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "EtherCAT frames are processed, non-EtherCAT frames are forwarded without processing or modification."]
            pub const BF_VAL_0: u32 = 0;
            #[doc = "EtherCAT frames are processed, non-EtherCAT frames are destroyed."]
            pub const BF_VAL_1: u32 = 0x01;
        }
    }
    #[doc = "Temporary use of settings in 0x0100:0x0103\\[8:15\\]:"]
    pub mod BF1 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "permanent use"]
            pub const BF_VAL_0: u32 = 0;
            #[doc = "use for about 1 second, then revert to previous settings"]
            pub const BF_VAL_1: u32 = 0x01;
        }
    }
    #[doc = "Loop Port 0:"]
    pub mod BF8 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Auto"]
            pub const BF_VAL_0: u32 = 0;
            #[doc = "Auto Close"]
            pub const BF_VAL_1: u32 = 0x01;
            #[doc = "Open"]
            pub const BF_VAL_2: u32 = 0x02;
            #[doc = "Closed"]
            pub const BF_VAL_3: u32 = 0x03;
        }
    }
    #[doc = "Loop Port 1:"]
    pub mod BF10 {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Auto"]
            pub const BF_VAL_0: u32 = 0;
            #[doc = "Auto Close"]
            pub const BF_VAL_1: u32 = 0x01;
            #[doc = "Open"]
            pub const BF_VAL_2: u32 = 0x02;
            #[doc = "Closed"]
            pub const BF_VAL_3: u32 = 0x03;
        }
    }
    #[doc = "RX FIFO Size (ESC delays start of forwarding until FIFO is at least half full)."]
    pub mod BF16 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Station alias:"]
    pub mod BF24 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Ignore Station Alias"]
            pub const BF_VAL_0: u32 = 0;
            #[doc = "Alias can be used for all configured address command types (FPRD, FPWR, ...)"]
            pub const BF_VAL_1: u32 = 0x01;
        }
    }
}
#[doc = "Physical Read Write Offset"]
pub mod PHYSICAL_READ_WRITE_OFFSET {
    #[doc = "This register is used for ReadWrite commands in Device Addressing mode (FPRW, APRW, BRW)."]
    pub mod BF0 {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ESC DL Status"]
pub mod ESC_DL_STATUS {
    #[doc = "Register ESC DL Status"]
    pub mod BF0 {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "PDI operational/EEPROM loaded correctly:"]
            pub const BF_VAL_0: u16 = 0;
            #[doc = "EEPROM loaded correctly, PDI operational (access to Process Data RAM)"]
            pub const BF_VAL_1: u16 = 0x01;
        }
    }
    #[doc = "PDI Watchdog Status:"]
    pub mod BF1 {
        pub const offset: u16 = 1;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Watchdog expired"]
            pub const BF_VAL_0: u16 = 0;
            #[doc = "Watchdog reloaded"]
            pub const BF_VAL_1: u16 = 0x01;
        }
    }
    #[doc = "Enhanced Link detection:"]
    pub mod BF2 {
        pub const offset: u16 = 2;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Deactivated for all ports"]
            pub const BF_VAL_0: u16 = 0;
            #[doc = "Activated for at least one port"]
            pub const BF_VAL_1: u16 = 0x01;
        }
    }
    #[doc = "Physical link on Port 0:"]
    pub mod BF4 {
        pub const offset: u16 = 4;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No link"]
            pub const BF_VAL_0: u16 = 0;
            #[doc = "Link detected"]
            pub const BF_VAL_1: u16 = 0x01;
        }
    }
    #[doc = "Physical link on Port 1:"]
    pub mod BF5 {
        pub const offset: u16 = 5;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No link"]
            pub const BF_VAL_0: u16 = 0;
            #[doc = "Link detected"]
            pub const BF_VAL_1: u16 = 0x01;
        }
    }
    #[doc = "Loop Port 0:"]
    pub mod BF8 {
        pub const offset: u16 = 8;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Open"]
            pub const BF_VAL_0: u16 = 0;
            #[doc = "Closed"]
            pub const BF_VAL_1: u16 = 0x01;
        }
    }
    #[doc = "Communication on Port 0:"]
    pub mod BF9 {
        pub const offset: u16 = 9;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No stable communication"]
            pub const BF_VAL_0: u16 = 0;
            #[doc = "Communication established"]
            pub const BF_VAL_1: u16 = 0x01;
        }
    }
    #[doc = "Loop Port 1"]
    pub mod BF10 {
        pub const offset: u16 = 10;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Open"]
            pub const BF_VAL_0: u16 = 0;
            #[doc = "Closed"]
            pub const BF_VAL_1: u16 = 0x01;
        }
    }
    #[doc = "Communication on Port 1:"]
    pub mod BF11 {
        pub const offset: u16 = 11;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No stable communication"]
            pub const BF_VAL_0: u16 = 0;
            #[doc = "Communication established"]
            pub const BF_VAL_1: u16 = 0x01;
        }
    }
}
#[doc = "AL Control"]
pub mod AL_CONTROL {
    #[doc = "Initiate State Transition of the Device State Machine:"]
    pub mod BF0 {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Error Ind Ack"]
    pub mod BF4 {
        pub const offset: u16 = 4;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No Ack of Error Ind in AL status register"]
            pub const BF_VAL_0: u16 = 0;
            #[doc = "Ack of Error Ind in AL status register"]
            pub const BF_VAL_1: u16 = 0x01;
        }
    }
    #[doc = "Device Identification:"]
    pub mod BF5 {
        pub const offset: u16 = 5;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No request"]
            pub const BF_VAL_0: u16 = 0;
            #[doc = "Device Identification request"]
            pub const BF_VAL_1: u16 = 0x01;
        }
    }
}
#[doc = "AL Status"]
pub mod AL_STATUS_PDI {
    #[doc = "Actual State of the Device State Machine:"]
    pub mod BF0 {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Error Ind:"]
    pub mod BF4 {
        pub const offset: u16 = 4;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Device is in State as requested or Flag cleared by command"]
            pub const BF_VAL_0: u16 = 0;
            #[doc = "Device has not entered requested State or changed State as result of a local action"]
            pub const BF_VAL_1: u16 = 0x01;
        }
    }
    #[doc = "Device Identification:"]
    pub mod BF5 {
        pub const offset: u16 = 5;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Device Identification not valid"]
            pub const BF_VAL_0: u16 = 0;
            #[doc = "Device Identification loaded"]
            pub const BF_VAL_1: u16 = 0x01;
        }
    }
}
#[doc = "AL Status Code"]
pub mod AL_STATUS_CODE_PDI {
    #[doc = "AL Status Code"]
    pub mod BF0 {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "RUN LED Override"]
pub mod RUN_LED_OVERRIDE {
    #[doc = "LED code and AL Status"]
    pub mod BF0 {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enable Override:"]
    pub mod BF4 {
        pub const offset: u8 = 4;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Override disabled"]
            pub const BF_VAL_0: u8 = 0;
            #[doc = "Override enabled"]
            pub const BF_VAL_1: u8 = 0x01;
        }
    }
}
#[doc = "ERR LED Override"]
pub mod ERR_LED_OVERRIDE {
    #[doc = "LED code"]
    pub mod BF0 {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enable Override:"]
    pub mod BF4 {
        pub const offset: u8 = 4;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Override disabled"]
            pub const BF_VAL_0: u8 = 0;
            #[doc = "Override enabled"]
            pub const BF_VAL_1: u8 = 0x01;
        }
    }
}
#[doc = "PDI Control"]
pub mod PDI_CONTROL {
    #[doc = "Process data interface:"]
    pub mod BF0 {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interface deactivated (no PDI)"]
            pub const BF_VAL_0: u8 = 0;
            #[doc = "4 Digital Input"]
            pub const BF_VAL_1: u8 = 0x01;
            #[doc = "4 Digital Output"]
            pub const BF_VAL_2: u8 = 0x02;
            #[doc = "2 Digital Input and 2 Digital Output"]
            pub const BF_VAL_3: u8 = 0x03;
            #[doc = "Digital I/O"]
            pub const BF_VAL_4: u8 = 0x04;
            #[doc = "SPI Slave"]
            pub const BF_VAL_5: u8 = 0x05;
            #[doc = "Oversampling I/O"]
            pub const BF_VAL_6: u8 = 0x06;
            #[doc = "16 Bit asynchronous Microcontroller interface"]
            pub const BF_VAL_8: u8 = 0x08;
            #[doc = "8 Bit asynchronous Microcontroller interface"]
            pub const BF_VAL_9: u8 = 0x09;
            #[doc = "16 Bit synchronous Microcontroller interface"]
            pub const BF_VAL_10: u8 = 0x0a;
            #[doc = "8 Bit synchronous Microcontroller interface"]
            pub const BF_VAL_11: u8 = 0x0b;
            #[doc = "32 Digital Input and 0 Digital Output"]
            pub const BF_VAL_12: u8 = 0x10;
            #[doc = "24 Digital Input and 8 Digital Output"]
            pub const BF_VAL_13: u8 = 0x11;
            #[doc = "16 Digital Input and 16 Digital Output"]
            pub const BF_VAL_14: u8 = 0x12;
            #[doc = "8 Digital Input and 24 Digital Output"]
            pub const BF_VAL_15: u8 = 0x13;
            #[doc = "0 Digital Input and 32 Digital Output"]
            pub const BF_VAL_16: u8 = 0x14;
            #[doc = "On-chip bus."]
            pub const BF_VAL_17: u8 = 0x80;
        }
    }
}
#[doc = "ESC Configuration"]
pub mod ESC_CONFIGURATION {
    #[doc = "Device emulation (control of AL status):"]
    pub mod BF0 {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "AL status register has to be set by PDI"]
            pub const BF_VAL_0: u8 = 0;
            #[doc = "AL status register will be set to value written to AL control register"]
            pub const BF_VAL_1: u8 = 0x01;
        }
    }
    #[doc = "Enhanced Link detection all ports:"]
    pub mod BF1 {
        pub const offset: u8 = 1;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "disabled (if bits \\[7:4\\]=0)"]
            pub const BF_VAL_0: u8 = 0;
            #[doc = "enabled at all ports (overrides bits \\[7:4\\])"]
            pub const BF_VAL_1: u8 = 0x01;
        }
    }
    #[doc = "Distributed Clocks SYNC Out Unit:"]
    pub mod BF2 {
        pub const offset: u8 = 2;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "disabled (power saving)"]
            pub const BF_VAL_0: u8 = 0;
            #[doc = "enabled"]
            pub const BF_VAL_1: u8 = 0x01;
        }
    }
    #[doc = "Distributed Clocks Latch In Unit:"]
    pub mod BF3 {
        pub const offset: u8 = 3;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "disabled (power saving)"]
            pub const BF_VAL_0: u8 = 0;
            #[doc = "enabled"]
            pub const BF_VAL_1: u8 = 0x01;
        }
    }
    #[doc = "Enhanced Link port 0:"]
    pub mod BF4 {
        pub const offset: u8 = 4;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "disabled (if bit 1=0)"]
            pub const BF_VAL_0: u8 = 0;
            #[doc = "enabled"]
            pub const BF_VAL_1: u8 = 0x01;
        }
    }
    #[doc = "Enhanced Link port 1:"]
    pub mod BF5 {
        pub const offset: u8 = 5;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "disabled (if bit 1=0)"]
            pub const BF_VAL_0: u8 = 0;
            #[doc = "enabled"]
            pub const BF_VAL_1: u8 = 0x01;
        }
    }
}
#[doc = "PDI Information"]
pub mod PDI_INFORMATION {
    #[doc = "PDI function"]
    pub mod BF0 {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const BF_VAL_0: u16 = 0;
            #[doc = "Enabled"]
            pub const BF_VAL_1: u16 = 0x01;
        }
    }
    #[doc = "ESC configuration area loaded from EEPROM:"]
    pub mod BF1 {
        pub const offset: u16 = 1;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "not loaded"]
            pub const BF_VAL_0: u16 = 0;
            #[doc = "loaded"]
            pub const BF_VAL_1: u16 = 0x01;
        }
    }
    #[doc = "PDI active:"]
    pub mod BF2 {
        pub const offset: u16 = 2;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "PDI not active"]
            pub const BF_VAL_0: u16 = 0;
            #[doc = "PDI active"]
            pub const BF_VAL_1: u16 = 0x01;
        }
    }
    #[doc = "PDI configuration invalid:"]
    pub mod BF3 {
        pub const offset: u16 = 3;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "PDI configuration ok"]
            pub const BF_VAL_0: u16 = 0;
            #[doc = "PDI configuration invalid"]
            pub const BF_VAL_1: u16 = 0x01;
        }
    }
}
#[doc = "Register PDI On-chip bus configuration"]
pub mod PDI_ON_CHIP_BUS_CONFIGURATION {
    #[doc = "On-chip bus clock:"]
    pub mod BF0 {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "asynchronous"]
            pub const BF_VAL_0: u8 = 0;
            #[doc = "synchronous multiplication factor (N * 25 MHz)"]
            pub const BF_VAL_1_1: u8 = 0x01;
            #[doc = "synchronous multiplication factor (N * 25 MHz)"]
            pub const BF_VAL_1_2: u8 = 0x02;
            #[doc = "synchronous multiplication factor (N * 25 MHz)"]
            pub const BF_VAL_1_3: u8 = 0x03;
            #[doc = "synchronous multiplication factor (N * 25 MHz)"]
            pub const BF_VAL_1_4: u8 = 0x04;
            #[doc = "synchronous multiplication factor (N * 25 MHz)"]
            pub const BF_VAL_1_5: u8 = 0x05;
            #[doc = "synchronous multiplication factor (N * 25 MHz)"]
            pub const BF_VAL_1_6: u8 = 0x06;
            #[doc = "synchronous multiplication factor (N * 25 MHz)"]
            pub const BF_VAL_1_7: u8 = 0x07;
            #[doc = "synchronous multiplication factor (N * 25 MHz)"]
            pub const BF_VAL_1_8: u8 = 0x08;
            #[doc = "synchronous multiplication factor (N * 25 MHz)"]
            pub const BF_VAL_1_9: u8 = 0x09;
        }
    }
    #[doc = "On-chip bus"]
    pub mod BF5 {
        pub const offset: u8 = 5;
        pub const mask: u8 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Intel Avalon"]
            pub const BF_VAL_0: u8 = 0;
            #[doc = "AXI 010: Xilinx PLB v4.6"]
            pub const BF_VAL_1: u8 = 0x01;
            #[doc = "Xilinx OPB"]
            pub const BF_VAL_2: u8 = 0x04;
        }
    }
}
#[doc = "PDI Configuration Sync Latch 1 and 0 PDI Configuration"]
pub mod SYNC_LATCH_1_AND_0_PDI_CONFIGURATION {
    #[doc = "SYNC0 output driver/polarity:"]
    pub mod BF0 {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Push-Pull active low"]
            pub const BF_VAL_0: u8 = 0;
            #[doc = "Open Drain (active low)"]
            pub const BF_VAL_1: u8 = 0x01;
            #[doc = "Push-Pull active high"]
            pub const BF_VAL_2: u8 = 0x02;
            #[doc = "Open Source (active high)"]
            pub const BF_VAL_3: u8 = 0x03;
        }
    }
    #[doc = "SYNC0/LATCH0 configuration*:"]
    pub mod BF2 {
        pub const offset: u8 = 2;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "LATCH0 Input"]
            pub const BF_VAL_0: u8 = 0;
            #[doc = "SYNC0 Output"]
            pub const BF_VAL_1: u8 = 0x01;
        }
    }
    #[doc = "SYNC0 mapped to AL Event Request register 0x0220\\[2\\]:"]
    pub mod BF3 {
        pub const offset: u8 = 3;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const BF_VAL_0: u8 = 0;
            #[doc = "Enabled"]
            pub const BF_VAL_1: u8 = 0x01;
        }
    }
    #[doc = "SYNC1 output driver/polarity:"]
    pub mod BF4 {
        pub const offset: u8 = 4;
        pub const mask: u8 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Push-Pull active low"]
            pub const BF_VAL_0: u8 = 0;
            #[doc = "Open Drain (active low)"]
            pub const BF_VAL_1: u8 = 0x01;
            #[doc = "Push-Pull active high"]
            pub const BF_VAL_2: u8 = 0x02;
            #[doc = "Open Source (active high)"]
            pub const BF_VAL_3: u8 = 0x03;
        }
    }
    #[doc = "SYNC1/LATCH1 configuration*"]
    pub mod BF6 {
        pub const offset: u8 = 6;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "LATCH1 input"]
            pub const BF_VAL_0: u8 = 0;
            #[doc = "SYNC1 output"]
            pub const BF_VAL_1: u8 = 0x01;
        }
    }
    #[doc = "SYNC1 mapped to AL Event Request register 0x0220\\[3\\]:"]
    pub mod BF7 {
        pub const offset: u8 = 7;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const BF_VAL_0: u8 = 0;
            #[doc = "Enabled"]
            pub const BF_VAL_1: u8 = 0x01;
        }
    }
}
#[doc = "Register PDI On-chip bus extended configuration."]
pub mod PDI_ON_CHIP_BUS_EXTENDED_CONFIGURATION {
    #[doc = "Read prefetch size (in cycles of PDI width):"]
    pub mod BF0 {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "4 cycles"]
            pub const BF_VAL_0: u16 = 0;
            #[doc = "1 cycle (typical)"]
            pub const BF_VAL_1: u16 = 0x01;
            #[doc = "2 cycles"]
            pub const BF_VAL_2: u16 = 0x02;
        }
    }
    #[doc = "On-chip bus sub-type for AXI:"]
    pub mod BF8 {
        pub const offset: u16 = 8;
        pub const mask: u16 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "AXI3"]
            pub const BF_VAL_0: u16 = 0;
            #[doc = "AXI4"]
            pub const BF_VAL_1: u16 = 0x01;
            #[doc = "AXI4 LITE"]
            pub const BF_VAL_2: u16 = 0x02;
        }
    }
}
#[doc = "ECAT Event Mask"]
pub mod ECAT_EVENT_MASK {
    #[doc = "ECAT Event masking of the ECAT Event Request Events for mapping into ECAT event field of EtherCAT frames:"]
    pub mod BF0 {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Corresponding ECAT Event Request register bit is not mapped"]
            pub const BF_VAL_0: u16 = 0;
            #[doc = "Corresponding ECAT Event Request register bit is mapped"]
            pub const BF_VAL_1: u16 = 0x01;
        }
    }
}
#[doc = "PDI AL Event Mask"]
pub mod PDI_AL_EVENT_MASK_PDI {
    #[doc = "AL Event masking of the AL Event Request register Events for mapping to PDI IRQ signal:"]
    pub mod BF0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Corresponding AL Event Request register bit is not mapped"]
            pub const BF_VAL_0: u32 = 0;
            #[doc = "Corresponding AL Event Request register bit is mapped"]
            pub const BF_VAL_1: u32 = 0x01;
        }
    }
}
#[doc = "ECAT Event Request"]
pub mod ECAT_EVENT_REQUEST {
    #[doc = "DC Latch event:"]
    pub mod BF0 {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No change on DC Latch Inputs"]
            pub const BF_VAL_0: u16 = 0;
            #[doc = "At least one change on DC Latch Inputs"]
            pub const BF_VAL_1: u16 = 0x01;
        }
    }
    #[doc = "DL Status event:"]
    pub mod BF2 {
        pub const offset: u16 = 2;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No change in DL Status"]
            pub const BF_VAL_0: u16 = 0;
            #[doc = "DL Status change"]
            pub const BF_VAL_1: u16 = 0x01;
        }
    }
    #[doc = "AL Status event:"]
    pub mod BF3 {
        pub const offset: u16 = 3;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No change in AL Status"]
            pub const BF_VAL_0: u16 = 0;
            #[doc = "AL Status change"]
            pub const BF_VAL_1: u16 = 0x01;
        }
    }
    #[doc = "Mirrors values of each SyncManager Status:"]
    pub mod BF4 {
        pub const offset: u16 = 4;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No Sync Channel 0 event"]
            pub const BF_VAL_0: u16 = 0;
            #[doc = "Sync Channel 0 event pending"]
            pub const BF_VAL_1: u16 = 0x01;
        }
    }
    #[doc = "Mirrors values of each SyncManager Status:"]
    pub mod BF5 {
        pub const offset: u16 = 5;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No Sync Channel 1 event"]
            pub const BF_VAL_0: u16 = 0;
            #[doc = "Sync Channel 1 event pending"]
            pub const BF_VAL_1: u16 = 0x01;
        }
    }
    #[doc = "Mirrors values of each SyncManager Status:"]
    pub mod BF6 {
        pub const offset: u16 = 6;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No Sync Channel 2 event"]
            pub const BF_VAL_0: u16 = 0;
            #[doc = "Sync Channel 2 event pending"]
            pub const BF_VAL_1: u16 = 0x01;
        }
    }
    #[doc = "Mirrors values of each SyncManager Status:"]
    pub mod BF7 {
        pub const offset: u16 = 7;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No Sync Channel 3 event"]
            pub const BF_VAL_0: u16 = 0;
            #[doc = "Sync Channel 3 event pending"]
            pub const BF_VAL_1: u16 = 0x01;
        }
    }
    #[doc = "Mirrors values of each SyncManager Status:"]
    pub mod BF8 {
        pub const offset: u16 = 8;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No Sync Channel 4 event"]
            pub const BF_VAL_0: u16 = 0;
            #[doc = "Sync Channel 4 event pending"]
            pub const BF_VAL_1: u16 = 0x01;
        }
    }
    #[doc = "Mirrors values of each SyncManager Status:"]
    pub mod BF9 {
        pub const offset: u16 = 9;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No Sync Channel 5 event"]
            pub const BF_VAL_0: u16 = 0;
            #[doc = "Sync Channel 5 event pending"]
            pub const BF_VAL_1: u16 = 0x01;
        }
    }
    #[doc = "Mirrors values of each SyncManager Status:"]
    pub mod BF10 {
        pub const offset: u16 = 10;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No Sync Channel 6 event"]
            pub const BF_VAL_0: u16 = 0;
            #[doc = "Sync Channel 6 event pending"]
            pub const BF_VAL_1: u16 = 0x01;
        }
    }
    #[doc = "Mirrors values of each SyncManager Status:"]
    pub mod BF11 {
        pub const offset: u16 = 11;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No Sync Channel 7 event"]
            pub const BF_VAL_0: u16 = 0;
            #[doc = "Sync Channel 7 event pending"]
            pub const BF_VAL_1: u16 = 0x01;
        }
    }
}
#[doc = "AL Event request"]
pub mod AL_EVENT_REQUEST {
    #[doc = "AL Control event:"]
    pub mod BF0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No AL Control Register change"]
            pub const BF_VAL_0: u32 = 0;
            #[doc = "AL Control Register has been written3"]
            pub const BF_VAL_1: u32 = 0x01;
        }
    }
    #[doc = "DC Latch event:"]
    pub mod BF1 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No change on DC Latch Inputs"]
            pub const BF_VAL_0: u32 = 0;
            #[doc = "At least one change on DC Latch Inputs"]
            pub const BF_VAL_1: u32 = 0x01;
        }
    }
    #[doc = "State of DC SYNC0 (if register 0x0151\\[3\\]=1):"]
    pub mod BF2 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "State of DC SYNC1 (if register 0x0151\\[7\\]=1):"]
    pub mod BF3 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "SyncManager activation register (SyncManager register offset 0x6) changed:"]
    pub mod BF4 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No change in any SyncManager"]
            pub const BF_VAL_0: u32 = 0;
            #[doc = "At least one SyncManager changed"]
            pub const BF_VAL_1: u32 = 0x01;
        }
    }
    #[doc = "EEPROM Emulation:"]
    pub mod BF5 {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No command pending"]
            pub const BF_VAL_0: u32 = 0;
            #[doc = "EEPROM command pending"]
            pub const BF_VAL_1: u32 = 0x01;
        }
    }
    #[doc = "Watchdog Process Data:"]
    pub mod BF6 {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Has not expired"]
            pub const BF_VAL_0: u32 = 0;
            #[doc = "Has expired"]
            pub const BF_VAL_1: u32 = 0x01;
        }
    }
    #[doc = "SyncManager interrupts (SyncManager register offset 0x5, bit \\[0\\] or \\[1\\]):"]
    pub mod BF8 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No SyncManager 0 interrupt"]
            pub const BF_VAL_0: u32 = 0;
            #[doc = "SyncManager 0 interrupt pending"]
            pub const BF_VAL_1: u32 = 0x01;
        }
    }
    #[doc = "Bit field access for ECAT: r/-"]
    pub mod BF9 {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No SyncManager 1 interrupt"]
            pub const BF_VAL_0: u32 = 0;
            #[doc = "SyncManager 1 interrupt pending"]
            pub const BF_VAL_1: u32 = 0x01;
        }
    }
    #[doc = "Bit field access for ECAT: r/-"]
    pub mod BF10 {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No SyncManager2 interrupt"]
            pub const BF_VAL_0: u32 = 0;
            #[doc = "SyncManager 2 interrupt pending"]
            pub const BF_VAL_1: u32 = 0x01;
        }
    }
    #[doc = "Bit field access for ECAT: r/-"]
    pub mod BF11 {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No SyncManager 3 interrupt"]
            pub const BF_VAL_0: u32 = 0;
            #[doc = "SyncManager 3 interrupt pending"]
            pub const BF_VAL_1: u32 = 0x01;
        }
    }
    #[doc = "Bit field access for ECAT: r/-"]
    pub mod BF12 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No SyncManager4 interrupt"]
            pub const BF_VAL_0: u32 = 0;
            #[doc = "SyncManager 4 interrupt pending"]
            pub const BF_VAL_1: u32 = 0x01;
        }
    }
    #[doc = "Bit field access for ECAT: r/-"]
    pub mod BF13 {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No SyncManager 5 interrupt"]
            pub const BF_VAL_0: u32 = 0;
            #[doc = "SyncManager 5 interrupt pending"]
            pub const BF_VAL_1: u32 = 0x01;
        }
    }
    #[doc = "Bit field access for ECAT: r/-"]
    pub mod BF14 {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No SyncManager 6 interrupt"]
            pub const BF_VAL_0: u32 = 0;
            #[doc = "SyncManager6 interrupt pending"]
            pub const BF_VAL_1: u32 = 0x01;
        }
    }
    #[doc = "Bit field access for ECAT: r/-"]
    pub mod BF15 {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No SyncManager 7 interrupt"]
            pub const BF_VAL_0: u32 = 0;
            #[doc = "SyncManager 7 interrupt pending"]
            pub const BF_VAL_1: u32 = 0x01;
        }
    }
}
#[doc = "ECAT Processing Unit Error Counter"]
pub mod ECAT_PROCESSING_UNIT_ERROR_COUNTER {
    #[doc = "ECAT Processing Unit error counter (counting is stopped when 0xFF is reached). Counts errors of frames passing the Processing Unit."]
    pub mod BF0 {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "PDI Error counter"]
pub mod PDI_ERROR_COUNTER {
    #[doc = "PDI Error counter (counting is stopped when 0xFF is reached). Counts if a PDI access has an interface error."]
    pub mod BF0 {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ASYNCHRONOUS_SYNCHRONOUS_MICROCONTROLLER_PDI_ERROR_CODE."]
pub mod ASYNCHRONOUS_SYNCHRONOUS_MICROCONTROLLER {
    #[doc = "Busy violation during read access"]
    pub mod BF0 {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "no error"]
            pub const BF_VAL_0: u8 = 0;
            #[doc = "error detected"]
            pub const BF_VAL_1: u8 = 0x01;
        }
    }
    #[doc = "Busy violation during write access"]
    pub mod BF1 {
        pub const offset: u8 = 1;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "no error"]
            pub const BF_VAL_0: u8 = 0;
            #[doc = "error detected"]
            pub const BF_VAL_1: u8 = 0x01;
        }
    }
    #[doc = "Addressing error for a read access (odd address without BHE)"]
    pub mod BF2 {
        pub const offset: u8 = 2;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "no error"]
            pub const BF_VAL_0: u8 = 0;
            #[doc = "error detected"]
            pub const BF_VAL_1: u8 = 0x01;
        }
    }
    #[doc = "Addressing error for a write access (odd address without BHE)"]
    pub mod BF3 {
        pub const offset: u8 = 3;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "no error"]
            pub const BF_VAL_0: u8 = 0;
            #[doc = "error detected"]
            pub const BF_VAL_1: u8 = 0x01;
        }
    }
}
#[doc = "Watchdog Divider"]
pub mod WATCHDOG_DIVIDER {
    #[doc = "Watchdog divider: Number of 25 MHz tics (minus 2) that represent the basic watchdog increment. (Default value is 100us = 2498)"]
    pub mod BF0 {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Register Watchdog Time PDI"]
pub mod WATCHDOG_TIME_PDI {
    #[doc = "Watchdog Time PDI: number or basic watchdog increments (Default value with Watchdog divider 100us means 100ms Watchdog)"]
    pub mod BF0 {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Regsister Watchdog Time Process Data"]
pub mod WATCHDOG_TIME_PROCESS_DATA {
    #[doc = "Watchdog Time Process Data"]
    pub mod BF0 {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Watchdog Status Process Data"]
pub mod WATCHDOG_STATUS_PROCESS_DATA {
    #[doc = "Watchdog Status of Process Data (triggered by SyncManagers)"]
    pub mod BF0 {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Watchdog Process Data expired"]
            pub const BF_VAL_0: u16 = 0;
            #[doc = "Watchdog Process Data is active or disabled"]
            pub const BF_VAL_1: u16 = 0x01;
        }
    }
}
#[doc = "Watchdog Counter Process Data"]
pub mod WATCHDOG_COUNTER_PROCESS_DATA {
    #[doc = "Watchdog Counter Process Data (counting is stopped when 0xFF is reached). Counts if Process Data Watchdog expires."]
    pub mod BF0 {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Watchdog Counter PDI"]
pub mod WATCHDOG_COUNTER_PDI {
    #[doc = "Watchdog PDI counter (counting is stopped when 0xFF is reached)."]
    pub mod BF0 {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "EEPROM Configuration"]
pub mod EEPROM_CONFIGURATION {
    #[doc = "EEPROM control is offered to PDI"]
    pub mod BF0 {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "no"]
            pub const BF_VAL_0: u8 = 0;
            #[doc = "yes (PDI has EEPROM control)"]
            pub const BF_VAL_1: u8 = 0x01;
        }
    }
    #[doc = "Force ECAT access"]
    pub mod BF1 {
        pub const offset: u8 = 1;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Do not change Bit 0x0501\\[0\\]"]
            pub const BF_VAL_0: u8 = 0;
            #[doc = "Reset Bit 0x0501\\[0\\] to 0"]
            pub const BF_VAL_1: u8 = 0x01;
        }
    }
}
#[doc = "EEPROM PDI Access State"]
pub mod REGISTER_EEPROM_PDI_ACCESS_STATE_PDI {
    #[doc = "Access to EEPROM:"]
    pub mod BF0 {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "PDI releases EEPROM access"]
            pub const BF_VAL_0: u8 = 0;
            #[doc = "PDI takes EEPROM access (PDI has EEPROM control)"]
            pub const BF_VAL_1: u8 = 0x01;
        }
    }
}
#[doc = "Register EEPROM Control/Status"]
pub mod EEPROM_CONTROL_STATUS {
    #[doc = "ECAT write enable2"]
    pub mod BF0 {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Write requests are disabled"]
            pub const BF_VAL_0: u16 = 0;
            #[doc = "Write requests are enabled"]
            pub const BF_VAL_1: u16 = 0x01;
        }
    }
    #[doc = "EEPROM emulation:"]
    pub mod BF5 {
        pub const offset: u16 = 5;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Normal operation (I2C interface used)"]
            pub const BF_VAL_0: u16 = 0;
            #[doc = "PDI emulates EEPROM (I2C not used)"]
            pub const BF_VAL_1: u16 = 0x01;
        }
    }
    #[doc = "Supported number of EEPROM read bytes:"]
    pub mod BF6 {
        pub const offset: u16 = 6;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "4 Bytes"]
            pub const BF_VAL_0: u16 = 0;
            #[doc = "8 Bytes"]
            pub const BF_VAL_1: u16 = 0x01;
        }
    }
    #[doc = "Selected EEPROM Algorithm:"]
    pub mod BF7 {
        pub const offset: u16 = 7;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "1 address byte (1Kbit to 16Kbit EEPROMs)"]
            pub const BF_VAL_0: u16 = 0;
            #[doc = "2 address bytes (32Kbit to 4 Mbit EEPROMs)"]
            pub const BF_VAL_1: u16 = 0x01;
        }
    }
    #[doc = "Command register"]
    pub mod BF8 {
        pub const offset: u16 = 8;
        pub const mask: u16 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No command/EEPROM idle (clear error bits)"]
            pub const BF_VAL_0: u16 = 0;
            #[doc = "Read"]
            pub const BF_VAL_1: u16 = 0x01;
            #[doc = "Write"]
            pub const BF_VAL_2: u16 = 0x02;
            #[doc = "Reload"]
            pub const BF_VAL_3: u16 = 0x04;
        }
    }
    #[doc = "Checksum Error in ESC Configuration Area:"]
    pub mod BF11 {
        pub const offset: u16 = 11;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Checksum ok"]
            pub const BF_VAL_0: u16 = 0;
            #[doc = "Checksum error"]
            pub const BF_VAL_1: u16 = 0x01;
        }
    }
    #[doc = "EEPROM loading status:"]
    pub mod BF12 {
        pub const offset: u16 = 12;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "EEPROM loaded, device information ok"]
            pub const BF_VAL_0: u16 = 0;
            #[doc = "EEPROM not loaded, device information not available (EEPROM loading in progress or finished with a failure)"]
            pub const BF_VAL_1: u16 = 0x01;
        }
    }
    #[doc = "Error Acknowledge/Command3:"]
    pub mod BF13 {
        pub const offset: u16 = 13;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No error"]
            pub const BF_VAL_0: u16 = 0;
            #[doc = "Missing EEPROM acknowledge or invalid command"]
            pub const BF_VAL_1: u16 = 0x01;
        }
    }
    #[doc = "Error Write Enable3:"]
    pub mod BF14 {
        pub const offset: u16 = 14;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No error"]
            pub const BF_VAL_0: u16 = 0;
            #[doc = "Write Command without Write enable"]
            pub const BF_VAL_1: u16 = 0x01;
        }
    }
    #[doc = "Busy:"]
    pub mod BF15 {
        pub const offset: u16 = 15;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "EEPROM Interface is idle"]
            pub const BF_VAL_0: u16 = 0;
            #[doc = "EEPROM Interface is busy"]
            pub const BF_VAL_1: u16 = 0x01;
        }
    }
}
#[doc = "EEPROM Address"]
pub mod EEPROM_ADDRESS {
    #[doc = "EEPROM Address"]
    pub mod BF0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "EEPROM Data"]
pub mod EEPROM_DATA {
    #[doc = "EEPROM Write data (data to be written to EEPROM) or EEPROM Read data (data read from EEPROM,. lower bytes)"]
    pub mod BF0 {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "EEPROM Read data (data read from EEPROM, higher bytes)"]
    pub mod BF16 {
        pub const offset: u64 = 16;
        pub const mask: u64 = 0xffff_ffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "MII Management Control/Status"]
pub mod MII_MANAGEMENT_CONTROL_OR_STATUS {
    #[doc = "Write enable*:"]
    pub mod BF0 {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Write disabled"]
            pub const BF_VAL_0: u16 = 0;
            #[doc = "Write enabled"]
            pub const BF_VAL_1: u16 = 0x01;
        }
    }
    #[doc = "Management Interface can be controlled by PDI (registers 0x0516-0x0517):"]
    pub mod BF1 {
        pub const offset: u16 = 1;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Only ECAT control"]
            pub const BF_VAL_0: u16 = 0;
            #[doc = "PDI control possible"]
            pub const BF_VAL_1: u16 = 0x01;
        }
    }
    #[doc = "MI link detection and configuration:"]
    pub mod BF2 {
        pub const offset: u16 = 2;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled for all ports"]
            pub const BF_VAL_0: u16 = 0;
            #[doc = "Enabled for at least one MII port, refer to PHY Port Status (0x0518 ff.) for details"]
            pub const BF_VAL_1: u16 = 0x01;
        }
    }
    #[doc = "PHY address of port 0"]
    pub mod BF3 {
        pub const offset: u16 = 3;
        pub const mask: u16 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Command register*:"]
    pub mod BF8 {
        pub const offset: u16 = 8;
        pub const mask: u16 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No command/MI idle (clear error bits)"]
            pub const BF_VAL_0: u16 = 0;
            #[doc = "Read"]
            pub const BF_VAL_1: u16 = 0x01;
            #[doc = "Write"]
            pub const BF_VAL_2: u16 = 0x02;
        }
    }
    #[doc = "Read error:"]
    pub mod BF13 {
        pub const offset: u16 = 13;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No read error"]
            pub const BF_VAL_0: u16 = 0;
            #[doc = "Read error occurred (PHY or register not available)"]
            pub const BF_VAL_1: u16 = 0x01;
        }
    }
    #[doc = "Command error:"]
    pub mod BF14 {
        pub const offset: u16 = 14;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Last Command was successful"]
            pub const BF_VAL_0: u16 = 0;
            #[doc = "Invalid command or write command without Write Enable"]
            pub const BF_VAL_1: u16 = 0x01;
        }
    }
    #[doc = "Busy:"]
    pub mod BF15 {
        pub const offset: u16 = 15;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MII Management Interface is idle"]
            pub const BF_VAL_0: u16 = 0;
            #[doc = "MII Management Interface is busy"]
            pub const BF_VAL_1: u16 = 0x01;
        }
    }
}
#[doc = "PHY Address"]
pub mod PHY_ADDRESS {
    #[doc = "PHY Address"]
    pub mod BF0 {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Show configured PHY address of port 0-3 in register 0x0510\\[7:3\\]. This is used if the PHY addresses are not consecutive."]
    pub mod BF7 {
        pub const offset: u8 = 7;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Register 0x0510\\[7:3\\] shows PHY address of port 0 (this is also the PHY address offset, if the PHY addresses are consecutive)"]
            pub const BF_VAL_0: u8 = 0;
            #[doc = "Register 0x0510\\[7:3\\] shows PHY address of port 0x0512\\[4:0\\] (valid values 0-3)"]
            pub const BF_VAL_1: u8 = 0x01;
        }
    }
}
#[doc = "PHY Register Address"]
pub mod PHY_REGISTER_ADDRESS {
    #[doc = "Address of PHY Register that shall be read/written"]
    pub mod BF0 {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "PHY Data"]
pub mod PHY_DATA {
    #[doc = "PHY Read/Write Data"]
    pub mod BF0 {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "MII Management ECAT Access State"]
pub mod MII_MANAGEMENT_ECAT_ACCESS_STATE {
    #[doc = "Access to MII management:"]
    pub mod BF0 {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ECAT enables PDI takeover of MII management interface"]
            pub const BF_VAL_0: u8 = 0;
            #[doc = "ECAT claims exclusive access to MII management interface"]
            pub const BF_VAL_1: u8 = 0x01;
        }
    }
}
#[doc = "MII Management PDI Access State"]
pub mod MII_MANAGEMENT_PDI_ACCESS_STATE {
    #[doc = "Access to MII management:"]
    pub mod BF0 {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ECAT has access to MII management"]
            pub const BF_VAL_0: u8 = 0;
            #[doc = "PDI has access to MII management"]
            pub const BF_VAL_1: u8 = 0x01;
        }
    }
    #[doc = "Force PDI Access State:"]
    pub mod BF1 {
        pub const offset: u8 = 1;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Do not change Bit 0x0517\\[0\\]"]
            pub const BF_VAL_0: u8 = 0;
            #[doc = "Reset Bit 0x0517\\[0\\] to 0"]
            pub const BF_VAL_1: u8 = 0x01;
        }
    }
}
#[doc = "PHY Port"]
pub mod PHY_PORT_STATUS {
    #[doc = "Physical link status (PHY status register 1.2):"]
    pub mod BF0 {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No physical link"]
            pub const BF_VAL_0: u8 = 0;
            #[doc = "Physical link detected"]
            pub const BF_VAL_1: u8 = 0x01;
        }
    }
    #[doc = "Link status (100 Mbit/s, Full Duplex, Auto negotiation):"]
    pub mod BF1 {
        pub const offset: u8 = 1;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No link"]
            pub const BF_VAL_0: u8 = 0;
            #[doc = "Link detected"]
            pub const BF_VAL_1: u8 = 0x01;
        }
    }
    #[doc = "Link status error:"]
    pub mod BF2 {
        pub const offset: u8 = 2;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No error"]
            pub const BF_VAL_0: u8 = 0;
            #[doc = "Link error, link inhibited"]
            pub const BF_VAL_1: u8 = 0x01;
        }
    }
    #[doc = "Read error:"]
    pub mod BF3 {
        pub const offset: u8 = 3;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No read error occurred"]
            pub const BF_VAL_0: u8 = 0;
            #[doc = "A read error has occurred"]
            pub const BF_VAL_1: u8 = 0x01;
        }
    }
    #[doc = "Link partner error:"]
    pub mod BF4 {
        pub const offset: u8 = 4;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No error detected"]
            pub const BF_VAL_0: u8 = 0;
            #[doc = "Link partner error"]
            pub const BF_VAL_1: u8 = 0x01;
        }
    }
    #[doc = "PHY configuration updated"]
    pub mod BF5 {
        pub const offset: u8 = 5;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No update"]
            pub const BF_VAL_0: u8 = 0;
            #[doc = "PHY configuration was updated"]
            pub const BF_VAL_1: u8 = 0x01;
        }
    }
}
#[doc = "Distributed Clocks Receive Times"]
pub mod RECEIVE_TIMES {
    #[doc = "Write"]
    pub mod BF0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Local time at the beginning of the last receive frame containing a write access to register 0x0900."]
    pub mod BF8 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Distributed Clocks Receive Time Port 1"]
pub mod RECEIVE_TIME_PORT_1 {
    #[doc = "Local time at the beginning of a frame (start first bit of preamble) received at port 1 containing a BWR or FPWR to register 0x0900."]
    pub mod BF0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Register System Time"]
pub mod SYSTEM_TIME {
    #[doc = "ECAT read access"]
    pub mod BF0ECAT {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Distributed Clocks Register Receive Time ECAT Processing Unit"]
pub mod RECEIVE_TIME_ECAT_PROCESSING_UNIT {
    #[doc = "Local time at the beginning of a frame (start first bit of preamble) received at the ECAT Processing Unit containing a write access to register 0x0900"]
    pub mod BF0 {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Register System Time Offset"]
pub mod SYSTEM_TIME_OFFSET {
    #[doc = "Difference between local time and System Time. Offset is added to the local time."]
    pub mod BF0 {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Register System Time Delay"]
pub mod SYSTEM_TIME_DELAY {
    #[doc = "Delay between Reference Clock and the eCAT"]
    pub mod BF0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Register System Time Difference"]
pub mod SYSTEM_TIME_DIFFERENCE {
    #[doc = "Mean difference between local copy of system Time and received System Time values"]
    pub mod BF0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x7fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Bit field access for ECAT: r/-"]
    pub mod BF31 {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Local copy of System Time less than received System Time"]
            pub const BF_VAL_0: u32 = 0;
            #[doc = "Local copy of System Time greater than or equal to received System Time"]
            pub const BF_VAL_3: u32 = 0x01;
        }
    }
}
#[doc = "Register Speed Counter Start"]
pub mod SPEED_COUNTER_START {
    #[doc = "Bandwidth for adjustment of local copy of system Time (larger values -> smaller bandwidth and smoother adjustment)"]
    pub mod BF0 {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0x7fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Register Speed Counter Diff"]
pub mod SPEED_COUNTER_DIFF {
    #[doc = "Representation of the deviation between local clock period and Reference Clock's clock period (representation: two's complement)"]
    pub mod BF0 {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Register System Time Difference Filter Depth"]
pub mod SYSTEM_TIME_DIFFERENCE_FILTER_DEPTH {
    #[doc = "Filter depth for averaging the received System Time deviation"]
    pub mod BF0 {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Register Speed Counter Filter Depth"]
pub mod SPEED_COUNTER_FILTER_DEPTH {
    #[doc = "Filter depth for averaging the clock period deviation"]
    pub mod BF0 {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Register Cyclic Unit Control"]
pub mod CYCLIC_UNIT_CONTROL {
    #[doc = "SYNC out unit control:"]
    pub mod BF0 {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ECAT-controlled"]
            pub const BF_VAL_0: u8 = 0;
            #[doc = "PDI-controlled"]
            pub const BF_VAL_1: u8 = 0x01;
        }
    }
    #[doc = "Latch In unit 0:"]
    pub mod BF4 {
        pub const offset: u8 = 4;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ECAT-controlled"]
            pub const BF_VAL_0: u8 = 0;
            #[doc = "PDI-controlled"]
            pub const BF_VAL_1: u8 = 0x01;
        }
    }
    #[doc = "Latch In unit 1:"]
    pub mod BF5 {
        pub const offset: u8 = 5;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ECAT-controlled"]
            pub const BF_VAL_0: u8 = 0;
            #[doc = "PDI-controlled"]
            pub const BF_VAL_1: u8 = 0x01;
        }
    }
}
#[doc = "Register Activation register"]
pub mod UNIT_ACTIVATION_REGISTER {
    #[doc = "Sync Out Unit activation:"]
    pub mod BF0 {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Deactivated"]
            pub const BF_VAL_0: u8 = 0;
            #[doc = "Activated"]
            pub const BF_VAL_1: u8 = 0x01;
        }
    }
    #[doc = "SYNC0 generation:"]
    pub mod BF1 {
        pub const offset: u8 = 1;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Deactivated"]
            pub const BF_VAL_0: u8 = 0;
            #[doc = "SYNC0 pulse is generated"]
            pub const BF_VAL_1: u8 = 0x01;
        }
    }
    #[doc = "SYNC1 generation:"]
    pub mod BF2 {
        pub const offset: u8 = 2;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Deactivated"]
            pub const BF_VAL_0: u8 = 0;
            #[doc = "SYNC1 pulse is generated"]
            pub const BF_VAL_1: u8 = 0x01;
        }
    }
    #[doc = "Auto-activation by writing Start Time Cyclic Operation (0x0990:0x0997):"]
    pub mod BF3 {
        pub const offset: u8 = 3;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const BF_VAL_0: u8 = 0;
            #[doc = "Auto-activation enabled. 0x0981\\[0\\] is set automatically after Start Time is written"]
            pub const BF_VAL_1: u8 = 0x01;
        }
    }
    #[doc = "Extension of Start Time Cyclic Operation (0x0990:0x0993):"]
    pub mod BF4 {
        pub const offset: u8 = 4;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No extension"]
            pub const BF_VAL_0: u8 = 0;
            #[doc = "Extend 32 bit written Start Time to 64 bit"]
            pub const BF_VAL_1: u8 = 0x01;
        }
    }
    #[doc = "Start Time plausibility check:"]
    pub mod BF5 {
        pub const offset: u8 = 5;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled. SyncSignal generation if Start Time is reached."]
            pub const BF_VAL_0: u8 = 0;
            #[doc = "Immediate SyncSignal generation if Start Time is outside near future (see 0x0981\\[6\\])"]
            pub const BF_VAL_1: u8 = 0x01;
        }
    }
    #[doc = "Near future configuration (approx.):"]
    pub mod BF6 {
        pub const offset: u8 = 6;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "1/2 DC width future (2^31ns or 2^63ns)"]
            pub const BF_VAL_0: u8 = 0;
            #[doc = "~2.1 sec. future (2^31ns)"]
            pub const BF_VAL_1: u8 = 0x01;
        }
    }
    #[doc = "SyncSignal debug pulse (Vasily bit):"]
    pub mod BF7 {
        pub const offset: u8 = 7;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Deactivated"]
            pub const BF_VAL_0: u8 = 0;
            #[doc = "Immediately generate one ping only on SYNC0-1 according to 0x0981\\[2:1\\] for debugging."]
            pub const BF_VAL_1: u8 = 0x01;
        }
    }
}
#[doc = "Register Pulse Length of SyncSignals"]
pub mod UNI_PULSE_LENGTH_OF_SYNCSIGNALS {
    #[doc = "Pulse length of SyncSignals (in Units of 10ns)"]
    pub mod BF0 {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Acknowledge mode: SyncSignal will be cleared by reading SYNC\\[1:0\\] Status register"]
            pub const BF_VAL_0: u16 = 0;
        }
    }
}
#[doc = "Register Activation Status"]
pub mod UNIT_ACTIVATION_STATUS {
    #[doc = "SYNC0 activation state:"]
    pub mod BF0 {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "First SYNC0 pulse is not pending"]
            pub const BF_VAL_0: u8 = 0;
            #[doc = "First SYNC0 pulse is pending"]
            pub const BF_VAL_1: u8 = 0x01;
        }
    }
    #[doc = "SYNC1 activation state:"]
    pub mod BF1 {
        pub const offset: u8 = 1;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "First SYNC1 pulse is not pending"]
            pub const BF_VAL_0: u8 = 0;
            #[doc = "First SYNC1 pulse is pending"]
            pub const BF_VAL_1: u8 = 0x01;
        }
    }
    #[doc = "Start Time Cyclic Operation (0x0990:0x0997) plausibility check result when Sync Out Unit was activated:"]
    pub mod BF2 {
        pub const offset: u8 = 2;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Start Time was within near future"]
            pub const BF_VAL_0: u8 = 0;
            #[doc = "Start Time was out of near future (0x0981\\[6\\])"]
            pub const BF_VAL_1: u8 = 0x01;
        }
    }
}
#[doc = "Register SYNC0 Status"]
pub mod UNIT_SYNC0_STATUS {
    #[doc = "SYNC0 state for Acknowledge mode."]
    pub mod BF0 {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Register SYNC1 Status"]
pub mod UNIT_SYNC1_STATUS {
    #[doc = "SYNC1 state for Acknowledge mode."]
    pub mod BF0 {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Register Start Time Cyclic Operation"]
pub mod UNIT_START_TIME_CYCLIC_OPERATION {
    #[doc = "Write: Start time (System time) of cyclic operation in ns"]
    pub mod BF0 {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Register Next SYNC1 Pulse"]
pub mod UNIT_NEXT_SYNC1_PULSE {
    #[doc = "System time of next SYNC1 pulse in ns"]
    pub mod BF0 {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Register SYNC0 Cycle Time"]
pub mod UNIT_SYNC0_CYCLE_TIME {
    #[doc = "Time between two consecutive SYNC0 pulses in ns."]
    pub mod BF0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Single shot mode, generate only one SYNC0 pulse."]
            pub const BF_VAL_0: u32 = 0;
        }
    }
}
#[doc = "Register SYNC1 Cycle Time"]
pub mod UNIT_SYNC1_CYCLE_TIME {
    #[doc = "Time between SYNC0 pulse and SYNC1 pulse in ns"]
    pub mod BF0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Register Latch0 Control"]
pub mod LATCH0_CONTROL {
    #[doc = "Latch0 positive edge:"]
    pub mod BF0 {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Continuous Latch active"]
            pub const BF_VAL_0: u8 = 0;
            #[doc = "Single event (only first event active)"]
            pub const BF_VAL_1: u8 = 0x01;
        }
    }
    #[doc = "Latch0 negative edge:"]
    pub mod BF1 {
        pub const offset: u8 = 1;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Continuous Latch active"]
            pub const BF_VAL_0: u8 = 0;
            #[doc = "Single event (only first event active)"]
            pub const BF_VAL_1: u8 = 0x01;
        }
    }
}
#[doc = "Register Latch1 Control"]
pub mod LATCH1_CONTROL {
    #[doc = "Latch1 positive edge:"]
    pub mod BF0 {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Continuous Latch active"]
            pub const BF_VAL_0: u8 = 0;
            #[doc = "Single event (only first event active)"]
            pub const BF_VAL_1: u8 = 0x01;
        }
    }
    #[doc = "Latch1 negative edge:"]
    pub mod BF1 {
        pub const offset: u8 = 1;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Continuous Latch active"]
            pub const BF_VAL_0: u8 = 0;
            #[doc = "Single event (only first event active)"]
            pub const BF_VAL_1: u8 = 0x01;
        }
    }
}
#[doc = "Register Latch0 Status"]
pub mod LATCH0_STATUS {
    #[doc = "Event Latch0 positive edge."]
    pub mod BF0 {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Positive edge not detected or continuous mode"]
            pub const BF_VAL_0: u8 = 0;
            #[doc = "Positive edge detected in single event mode only."]
            pub const BF_VAL_1: u8 = 0x01;
        }
    }
    #[doc = "Event Latch0 negative edge."]
    pub mod BF1 {
        pub const offset: u8 = 1;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Negative edge not detected or continuous mode"]
            pub const BF_VAL_0: u8 = 0;
            #[doc = "Negative edge detected in single event mode only"]
            pub const BF_VAL_1: u8 = 0x01;
        }
    }
    #[doc = "Latch0 pin state"]
    pub mod BF2 {
        pub const offset: u8 = 2;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Register Latch1 Status"]
pub mod LATCH1_STATUS {
    #[doc = "Event Latch1 positive edge."]
    pub mod BF0 {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Positive edge not detected or continuous mode"]
            pub const BF_VAL_0: u8 = 0;
            #[doc = "Positive edge detected in single event mode only."]
            pub const BF_VAL_1: u8 = 0x01;
        }
    }
    #[doc = "Event Latch1 negative edge."]
    pub mod BF1 {
        pub const offset: u8 = 1;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Negative edge not detected or continuous mode"]
            pub const BF_VAL_0: u8 = 0;
            #[doc = "Negative edge detected in single event mode only."]
            pub const BF_VAL_1: u8 = 0x01;
        }
    }
    #[doc = "Latch1 pin state"]
    pub mod BF2 {
        pub const offset: u8 = 2;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Register Latch0 Time Positive Edge"]
pub mod LATCH0_TIME_POSITIVE_EDGE {
    #[doc = "System time at the positive edge of the Latch0 signal."]
    pub mod BF0 {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Register Latch0 Time Negative Edge"]
pub mod LATCH0_TIME_NEGATIVE_EDGE {
    #[doc = "System time at the negative edge of the Latch0 signal."]
    pub mod BF0 {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Register Latch1 Time Positive Edge"]
pub mod LATCH1_TIME_POSITIVE_EDGE {
    #[doc = "System time at the positive edge of the Latch1 signal."]
    pub mod BF0 {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Register Latch1 Time Negative Edge"]
pub mod LATCH1_TIME_NEGATIVE_EDGE {
    #[doc = "System time at the negative edge of the Latch1 signal."]
    pub mod BF0 {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Register EtherCAT Buffer Change Event Time"]
pub mod ETHERCAT_BUFFER_CHANGE_EVENT_TIME {
    #[doc = "Local time at the beginning of the frame which causes at least one SyncManager to assert an ECAT event"]
    pub mod BF0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Register PDI Buffer Start Event Time"]
pub mod PDI_BUFFER_START_EVENT_TIME {
    #[doc = "Local time when at least one SyncManager asserts a PDI buffer start event"]
    pub mod BF0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Register PDI Buffer Change Event Time"]
pub mod PDI_BUFFER_CHANGE_EVENT_TIME {
    #[doc = "Local time when at least one SyncManager asserts a PDI buffer change event"]
    pub mod BF0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Register Product ID IP Core"]
pub mod PRODUCT_ID_IP_CORE {
    #[doc = "Product ID"]
    pub mod BF0 {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Register Vendor ID IP Core"]
pub mod VENDOR_ID_IP_CORE {
    #[doc = "Vendor ID"]
    pub mod BF0 {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Register General Purpose Outputs"]
pub mod GENERAL_PURPOSE_OUTPUTS {
    #[doc = "General Purpose Output Data"]
    pub mod BF0 {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Register General Purpose Inputs"]
pub mod GENERAL_PURPOSE_INPUTS {
    #[doc = "General Purpose Input Data"]
    pub mod BF0 {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
