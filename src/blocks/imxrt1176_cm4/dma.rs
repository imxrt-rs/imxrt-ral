#[doc = "DMA"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "Control"]
    pub CR: crate::RWRegister<u32>,
    #[doc = "Error Status"]
    pub ES: crate::RORegister<u32>,
    _reserved0: [u8; 0x04],
    #[doc = "Enable Request"]
    pub ERQ: crate::RWRegister<u32>,
    _reserved1: [u8; 0x04],
    #[doc = "Enable Error Interrupt"]
    pub EEI: crate::RWRegister<u32>,
    #[doc = "Clear Enable Error Interrupt"]
    pub CEEI: crate::RWRegister<u8>,
    #[doc = "Set Enable Error Interrupt"]
    pub SEEI: crate::RWRegister<u8>,
    #[doc = "Clear Enable Request"]
    pub CERQ: crate::RWRegister<u8>,
    #[doc = "Set Enable Request"]
    pub SERQ: crate::RWRegister<u8>,
    #[doc = "Clear DONE Status Bit"]
    pub CDNE: crate::RWRegister<u8>,
    #[doc = "Set START Bit"]
    pub SSRT: crate::RWRegister<u8>,
    #[doc = "Clear Error"]
    pub CERR: crate::RWRegister<u8>,
    #[doc = "Clear Interrupt Request"]
    pub CINT: crate::RWRegister<u8>,
    _reserved2: [u8; 0x04],
    #[doc = "Interrupt Request"]
    pub INT: crate::RWRegister<u32>,
    _reserved3: [u8; 0x04],
    #[doc = "Error"]
    pub ERR: crate::RWRegister<u32>,
    _reserved4: [u8; 0x04],
    #[doc = "Hardware Request Status"]
    pub HRS: crate::RORegister<u32>,
    _reserved5: [u8; 0x0c],
    #[doc = "Enable Asynchronous Request in Stop"]
    pub EARS: crate::RWRegister<u32>,
    _reserved6: [u8; 0xb8],
    #[doc = "Channel Priority"]
    pub DCHPRI3: crate::RWRegister<u8>,
    #[doc = "Channel Priority"]
    pub DCHPRI2: crate::RWRegister<u8>,
    #[doc = "Channel Priority"]
    pub DCHPRI1: crate::RWRegister<u8>,
    #[doc = "Channel Priority"]
    pub DCHPRI0: crate::RWRegister<u8>,
    #[doc = "Channel Priority"]
    pub DCHPRI7: crate::RWRegister<u8>,
    #[doc = "Channel Priority"]
    pub DCHPRI6: crate::RWRegister<u8>,
    #[doc = "Channel Priority"]
    pub DCHPRI5: crate::RWRegister<u8>,
    #[doc = "Channel Priority"]
    pub DCHPRI4: crate::RWRegister<u8>,
    #[doc = "Channel Priority"]
    pub DCHPRI11: crate::RWRegister<u8>,
    #[doc = "Channel Priority"]
    pub DCHPRI10: crate::RWRegister<u8>,
    #[doc = "Channel Priority"]
    pub DCHPRI9: crate::RWRegister<u8>,
    #[doc = "Channel Priority"]
    pub DCHPRI8: crate::RWRegister<u8>,
    #[doc = "Channel Priority"]
    pub DCHPRI15: crate::RWRegister<u8>,
    #[doc = "Channel Priority"]
    pub DCHPRI14: crate::RWRegister<u8>,
    #[doc = "Channel Priority"]
    pub DCHPRI13: crate::RWRegister<u8>,
    #[doc = "Channel Priority"]
    pub DCHPRI12: crate::RWRegister<u8>,
    #[doc = "Channel Priority"]
    pub DCHPRI19: crate::RWRegister<u8>,
    #[doc = "Channel Priority"]
    pub DCHPRI18: crate::RWRegister<u8>,
    #[doc = "Channel Priority"]
    pub DCHPRI17: crate::RWRegister<u8>,
    #[doc = "Channel Priority"]
    pub DCHPRI16: crate::RWRegister<u8>,
    #[doc = "Channel Priority"]
    pub DCHPRI23: crate::RWRegister<u8>,
    #[doc = "Channel Priority"]
    pub DCHPRI22: crate::RWRegister<u8>,
    #[doc = "Channel Priority"]
    pub DCHPRI21: crate::RWRegister<u8>,
    #[doc = "Channel Priority"]
    pub DCHPRI20: crate::RWRegister<u8>,
    #[doc = "Channel Priority"]
    pub DCHPRI27: crate::RWRegister<u8>,
    #[doc = "Channel Priority"]
    pub DCHPRI26: crate::RWRegister<u8>,
    #[doc = "Channel Priority"]
    pub DCHPRI25: crate::RWRegister<u8>,
    #[doc = "Channel Priority"]
    pub DCHPRI24: crate::RWRegister<u8>,
    #[doc = "Channel Priority"]
    pub DCHPRI31: crate::RWRegister<u8>,
    #[doc = "Channel Priority"]
    pub DCHPRI30: crate::RWRegister<u8>,
    #[doc = "Channel Priority"]
    pub DCHPRI29: crate::RWRegister<u8>,
    #[doc = "Channel Priority"]
    pub DCHPRI28: crate::RWRegister<u8>,
    _reserved7: [u8; 0x0ee0],
    #[doc = "Cluster TCD%s, containing TCD*_SADDR, TCD*_SOFF, TCD*_ATTR, TCD*_NBYTES_MLNO, TCD*_NBYTES_MLOFFNO, TCD*_NBYTES_MLOFFYES, TCD*_SLAST, TCD*_DADDR, TCD*_DOFF, TCD*_CITER_ELINKNO, TCD*_CITER_ELINKYES, TCD*_DLASTSGA, TCD*_CSR, TCD*_BITER_ELINKNO, TCD*_BITER_ELINKYES"]
    pub TCD: [tcd::RegisterBlock; 32usize],
}
#[doc = "Control"]
pub mod CR {
    #[doc = "Enable Debug"]
    pub mod EDBG {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "When the chip is in Debug mode, the eDMA continues to operate."]
            pub const DISABLED: u32 = 0;
            #[doc = "When the chip is in debug mode, the DMA stalls the start of a new channel. Executing channels are allowed to complete."]
            pub const ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Enable Round Robin Channel Arbitration"]
    pub mod ERCA {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Fixed priority arbitration within each group"]
            pub const DISABLED: u32 = 0;
            #[doc = "Round robin arbitration within each group"]
            pub const ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Enable Round Robin Group Arbitration"]
    pub mod ERGA {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Fixed priority arbitration"]
            pub const DISABLED: u32 = 0;
            #[doc = "Round robin arbitration"]
            pub const ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Halt On Error"]
    pub mod HOE {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Normal operation"]
            pub const NORMAL_OPS: u32 = 0;
            #[doc = "Error causes HALT field to be automatically set to 1"]
            pub const HALT_ON_ERROR: u32 = 0x01;
        }
    }
    #[doc = "Halt eDMA Operations"]
    pub mod HALT {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Normal operation"]
            pub const NORMAL_OPS: u32 = 0;
            #[doc = "eDMA operations halted"]
            pub const HALT_DMA: u32 = 0x01;
        }
    }
    #[doc = "Continuous Link Mode"]
    pub mod CLM {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Continuous link mode is off"]
            pub const CLM_OFF: u32 = 0;
            #[doc = "Continuous link mode is on"]
            pub const CLM_ON: u32 = 0x01;
        }
    }
    #[doc = "Enable Minor Loop Mapping"]
    pub mod EMLM {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const DISABLED: u32 = 0;
            #[doc = "Enabled"]
            pub const ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Channel Group 0 Priority"]
    pub mod GRP0PRI {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Channel Group 1 Priority"]
    pub mod GRP1PRI {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Error Cancel Transfer"]
    pub mod ECX {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Normal operation"]
            pub const NORMAL_OPS: u32 = 0;
            #[doc = "Cancel the remaining data transfer"]
            pub const CANCEL: u32 = 0x01;
        }
    }
    #[doc = "Cancel Transfer"]
    pub mod CX {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Normal operation"]
            pub const NORMAL_OPS: u32 = 0;
            #[doc = "Cancel the remaining data transfer"]
            pub const CANCEL: u32 = 0x01;
        }
    }
    #[doc = "eDMA version number"]
    pub mod VERSION {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x7f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "eDMA Active Status"]
    pub mod ACTIVE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "eDMA is idle"]
            pub const IDLE: u32 = 0;
            #[doc = "eDMA is executing a channel"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
}
#[doc = "Error Status"]
pub mod ES {
    #[doc = "Destination Bus Error"]
    pub mod DBE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No destination bus error."]
            pub const NO_ERROR: u32 = 0;
            #[doc = "The most-recently recorded error was a bus error on a destination write."]
            pub const ERROR: u32 = 0x01;
        }
    }
    #[doc = "Source Bus Error"]
    pub mod SBE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No source bus error."]
            pub const NO_ERROR: u32 = 0;
            #[doc = "The most-recently recorded error was a bus error on a source read."]
            pub const ERROR: u32 = 0x01;
        }
    }
    #[doc = "Scatter/Gather Configuration Error"]
    pub mod SGE {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No scatter/gather configuration error."]
            pub const NO_ERROR: u32 = 0;
            #[doc = "The most-recently recorded error was a configuration error detected in the TCDn_DLASTSGA field."]
            pub const ERROR: u32 = 0x01;
        }
    }
    #[doc = "NBYTES/CITER Configuration Error"]
    pub mod NCE {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No NBYTES/CITER configuration error."]
            pub const NO_ERROR: u32 = 0;
            #[doc = "The most-recently recorded error was a configuration error detected in the TCDn_NBYTES or TCDn_CITER fields. TCDn_NBYTES is not a multiple of TCDn_ATTR\\[SSIZE\\] and TCDn_ATTR\\[DSIZE\\], or TCDn_CITER\\[CITER\\] = 0, or TCDn_CITER\\[ELINK\\] is not equal to TCDn_BITER\\[ELINK\\]."]
            pub const ERROR: u32 = 0x01;
        }
    }
    #[doc = "Destination Offset Error"]
    pub mod DOE {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No destination offset configuration error."]
            pub const NO_ERROR: u32 = 0;
            #[doc = "The most-recently recorded error was a configuration error detected in the TCDn_DOFF field. TCDn_DOFF is inconsistent with TCDn_ATTR\\[DSIZE\\]."]
            pub const ERROR: u32 = 0x01;
        }
    }
    #[doc = "Destination Address Error"]
    pub mod DAE {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No destination address configuration error."]
            pub const NO_ERROR: u32 = 0;
            #[doc = "The most-recently recorded error was a configuration error detected in the TCDn_DADDR field. TCDn_DADDR is inconsistent with TCDn_ATTR\\[DSIZE\\]."]
            pub const ERROR: u32 = 0x01;
        }
    }
    #[doc = "Source Offset Error"]
    pub mod SOE {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No source offset configuration error."]
            pub const NO_ERROR: u32 = 0;
            #[doc = "The most-recently recorded error was a configuration error detected in the TCDn_SOFF field. TCDn_SOFF is inconsistent with TCDn_ATTR\\[SSIZE\\]."]
            pub const ERROR: u32 = 0x01;
        }
    }
    #[doc = "Source Address Error"]
    pub mod SAE {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No source address configuration error."]
            pub const NO_ERROR: u32 = 0;
            #[doc = "The most-recently recorded error was a configuration error detected in the TCDn_SADDR field. TCDn_SADDR is inconsistent with TCDn_ATTR\\[SSIZE\\]."]
            pub const ERROR: u32 = 0x01;
        }
    }
    #[doc = "Error Channel Number or Canceled Channel Number"]
    pub mod ERRCHN {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Channel Priority Error"]
    pub mod CPE {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No channel priority error."]
            pub const NO_ERROR: u32 = 0;
            #[doc = "The most-recently recorded error was a configuration error in the channel priorities within a group. Channel priorities within a group are not unique."]
            pub const ERROR: u32 = 0x01;
        }
    }
    #[doc = "Group Priority Error"]
    pub mod GPE {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No group priority error."]
            pub const NO_ERROR: u32 = 0;
            #[doc = "The most-recently recorded error was a configuration error among the group priorities. All group priorities are not unique."]
            pub const ERROR: u32 = 0x01;
        }
    }
    #[doc = "Transfer Canceled"]
    pub mod ECX {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No canceled transfers"]
            pub const NO_CANCELS: u32 = 0;
            #[doc = "The most-recently recorded entry was a canceled transfer initiated by the error cancel transfer field"]
            pub const CANCELED: u32 = 0x01;
        }
    }
    #[doc = "Logical OR of all ERR status fields"]
    pub mod VLD {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No ERR fields are 1"]
            pub const NO_ERROR: u32 = 0;
            #[doc = "At least one ERR field has a value of 1, indicating a valid error exists that has not been cleared"]
            pub const ERROR: u32 = 0x01;
        }
    }
}
#[doc = "Enable Request"]
pub mod ERQ {
    #[doc = "Enable DMA Request 0"]
    pub mod ERQ0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The DMA request signal for channel 0 is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "The DMA request signal for channel 0 is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Enable DMA Request 1"]
    pub mod ERQ1 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The DMA request signal for channel 1 is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "The DMA request signal for channel 1 is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Enable DMA Request 2"]
    pub mod ERQ2 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The DMA request signal for channel 2 is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "The DMA request signal for channel 2 is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Enable DMA Request 3"]
    pub mod ERQ3 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The DMA request signal for channel 3 is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "The DMA request signal for channel 3 is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Enable DMA Request 4"]
    pub mod ERQ4 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The DMA request signal for channel 4 is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "The DMA request signal for channel 4 is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Enable DMA Request 5"]
    pub mod ERQ5 {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The DMA request signal for channel 5 is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "The DMA request signal for channel 5 is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Enable DMA Request 6"]
    pub mod ERQ6 {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The DMA request signal for channel 6 is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "The DMA request signal for channel 6 is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Enable DMA Request 7"]
    pub mod ERQ7 {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The DMA request signal for channel 7 is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "The DMA request signal for channel 7 is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Enable DMA Request 8"]
    pub mod ERQ8 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The DMA request signal for channel 8 is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "The DMA request signal for channel 8 is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Enable DMA Request 9"]
    pub mod ERQ9 {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The DMA request signal for channel 9 is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "The DMA request signal for channel 9 is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Enable DMA Request 10"]
    pub mod ERQ10 {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The DMA request signal for channel 10 is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "The DMA request signal for channel 10 is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Enable DMA Request 11"]
    pub mod ERQ11 {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The DMA request signal for channel 11 is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "The DMA request signal for channel 11 is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Enable DMA Request 12"]
    pub mod ERQ12 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The DMA request signal for channel 12 is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "The DMA request signal for channel 12 is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Enable DMA Request 13"]
    pub mod ERQ13 {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The DMA request signal for channel 13 is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "The DMA request signal for channel 13 is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Enable DMA Request 14"]
    pub mod ERQ14 {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The DMA request signal for channel 14 is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "The DMA request signal for channel 14 is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Enable DMA Request 15"]
    pub mod ERQ15 {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The DMA request signal for channel 15 is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "The DMA request signal for channel 15 is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Enable DMA Request 16"]
    pub mod ERQ16 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The DMA request signal for channel 16 is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "The DMA request signal for channel 16 is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Enable DMA Request 17"]
    pub mod ERQ17 {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The DMA request signal for channel 17 is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "The DMA request signal for channel 17 is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Enable DMA Request 18"]
    pub mod ERQ18 {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The DMA request signal for channel 18 is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "The DMA request signal for channel 18 is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Enable DMA Request 19"]
    pub mod ERQ19 {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The DMA request signal for channel 19 is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "The DMA request signal for channel 19 is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Enable DMA Request 20"]
    pub mod ERQ20 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The DMA request signal for channel 20 is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "The DMA request signal for channel 20 is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Enable DMA Request 21"]
    pub mod ERQ21 {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The DMA request signal for channel 21 is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "The DMA request signal for channel 21 is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Enable DMA Request 22"]
    pub mod ERQ22 {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The DMA request signal for channel 22 is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "The DMA request signal for channel 22 is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Enable DMA Request 23"]
    pub mod ERQ23 {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The DMA request signal for channel 23 is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "The DMA request signal for channel 23 is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Enable DMA Request 24"]
    pub mod ERQ24 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The DMA request signal for channel 24 is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "The DMA request signal for channel 24 is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Enable DMA Request 25"]
    pub mod ERQ25 {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The DMA request signal for channel 25 is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "The DMA request signal for channel 25 is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Enable DMA Request 26"]
    pub mod ERQ26 {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The DMA request signal for channel 26 is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "The DMA request signal for channel 26 is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Enable DMA Request 27"]
    pub mod ERQ27 {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The DMA request signal for channel 27 is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "The DMA request signal for channel 27 is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Enable DMA Request 28"]
    pub mod ERQ28 {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The DMA request signal for channel 28 is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "The DMA request signal for channel 28 is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Enable DMA Request 29"]
    pub mod ERQ29 {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The DMA request signal for channel 29 is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "The DMA request signal for channel 29 is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Enable DMA Request 30"]
    pub mod ERQ30 {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The DMA request signal for channel 30 is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "The DMA request signal for channel 30 is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Enable DMA Request 31"]
    pub mod ERQ31 {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The DMA request signal for channel 31 is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "The DMA request signal for channel 31 is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
}
#[doc = "Enable Error Interrupt"]
pub mod EEI {
    #[doc = "Enable Error Interrupt 0"]
    pub mod EEI0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "An error on channel 0 does not generate an error interrupt"]
            pub const NO_INTERRUPT: u32 = 0;
            #[doc = "An error on channel 0 generates an error interrupt request"]
            pub const INTERRUPT: u32 = 0x01;
        }
    }
    #[doc = "Enable Error Interrupt 1"]
    pub mod EEI1 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "An error on channel 1 does not generate an error interrupt"]
            pub const NO_INTERRUPT: u32 = 0;
            #[doc = "An error on channel 1 generates an error interrupt request"]
            pub const INTERRUPT: u32 = 0x01;
        }
    }
    #[doc = "Enable Error Interrupt 2"]
    pub mod EEI2 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "An error on channel 2 does not generate an error interrupt"]
            pub const NO_INTERRUPT: u32 = 0;
            #[doc = "An error on channel 2 generates an error interrupt request"]
            pub const INTERRUPT: u32 = 0x01;
        }
    }
    #[doc = "Enable Error Interrupt 3"]
    pub mod EEI3 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "An error on channel 3 does not generate an error interrupt"]
            pub const NO_INTERRUPT: u32 = 0;
            #[doc = "An error on channel 3 generates an error interrupt request"]
            pub const INTERRUPT: u32 = 0x01;
        }
    }
    #[doc = "Enable Error Interrupt 4"]
    pub mod EEI4 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "An error on channel 4 does not generate an error interrupt"]
            pub const NO_INTERRUPT: u32 = 0;
            #[doc = "An error on channel 4 generates an error interrupt request"]
            pub const INTERRUPT: u32 = 0x01;
        }
    }
    #[doc = "Enable Error Interrupt 5"]
    pub mod EEI5 {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "An error on channel 5 does not generate an error interrupt"]
            pub const NO_INTERRUPT: u32 = 0;
            #[doc = "An error on channel 5 generates an error interrupt request"]
            pub const INTERRUPT: u32 = 0x01;
        }
    }
    #[doc = "Enable Error Interrupt 6"]
    pub mod EEI6 {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "An error on channel 6 does not generate an error interrupt"]
            pub const NO_INTERRUPT: u32 = 0;
            #[doc = "An error on channel 6 generates an error interrupt request"]
            pub const INTERRUPT: u32 = 0x01;
        }
    }
    #[doc = "Enable Error Interrupt 7"]
    pub mod EEI7 {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "An error on channel 7 does not generate an error interrupt"]
            pub const NO_INTERRUPT: u32 = 0;
            #[doc = "An error on channel 7 generates an error interrupt request"]
            pub const INTERRUPT: u32 = 0x01;
        }
    }
    #[doc = "Enable Error Interrupt 8"]
    pub mod EEI8 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "An error on channel 8 does not generate an error interrupt"]
            pub const NO_INTERRUPT: u32 = 0;
            #[doc = "An error on channel 8 generates an error interrupt request"]
            pub const INTERRUPT: u32 = 0x01;
        }
    }
    #[doc = "Enable Error Interrupt 9"]
    pub mod EEI9 {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "An error on channel 9 does not generate an error interrupt"]
            pub const NO_INTERRUPT: u32 = 0;
            #[doc = "An error on channel 9 generates an error interrupt request"]
            pub const INTERRUPT: u32 = 0x01;
        }
    }
    #[doc = "Enable Error Interrupt 10"]
    pub mod EEI10 {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "An error on channel 10 does not generate an error interrupt"]
            pub const NO_INTERRUPT: u32 = 0;
            #[doc = "An error on channel 10 generates an error interrupt request"]
            pub const INTERRUPT: u32 = 0x01;
        }
    }
    #[doc = "Enable Error Interrupt 11"]
    pub mod EEI11 {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "An error on channel 11 does not generate an error interrupt"]
            pub const NO_INTERRUPT: u32 = 0;
            #[doc = "An error on channel 11 generates an error interrupt request"]
            pub const INTERRUPT: u32 = 0x01;
        }
    }
    #[doc = "Enable Error Interrupt 12"]
    pub mod EEI12 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "An error on channel 12 does not generate an error interrupt"]
            pub const NO_INTERRUPT: u32 = 0;
            #[doc = "An error on channel 12 generates an error interrupt request"]
            pub const INTERRUPT: u32 = 0x01;
        }
    }
    #[doc = "Enable Error Interrupt 13"]
    pub mod EEI13 {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "An error on channel 13 does not generate an error interrupt"]
            pub const NO_INTERRUPT: u32 = 0;
            #[doc = "An error on channel 13 generates an error interrupt request"]
            pub const INTERRUPT: u32 = 0x01;
        }
    }
    #[doc = "Enable Error Interrupt 14"]
    pub mod EEI14 {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "An error on channel 14 does not generate an error interrupt"]
            pub const NO_INTERRUPT: u32 = 0;
            #[doc = "An error on channel 14 generates an error interrupt request"]
            pub const INTERRUPT: u32 = 0x01;
        }
    }
    #[doc = "Enable Error Interrupt 15"]
    pub mod EEI15 {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "An error on channel 15 does not generate an error interrupt"]
            pub const NO_INTERRUPT: u32 = 0;
            #[doc = "An error on channel 15 generates an error interrupt request"]
            pub const INTERRUPT: u32 = 0x01;
        }
    }
    #[doc = "Enable Error Interrupt 16"]
    pub mod EEI16 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "An error on channel 16 does not generate an error interrupt"]
            pub const NO_INTERRUPT: u32 = 0;
            #[doc = "An error on channel 16 generates an error interrupt request"]
            pub const INTERRUPT: u32 = 0x01;
        }
    }
    #[doc = "Enable Error Interrupt 17"]
    pub mod EEI17 {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "An error on channel 17 does not generate an error interrupt"]
            pub const NO_INTERRUPT: u32 = 0;
            #[doc = "An error on channel 17 generates an error interrupt request"]
            pub const INTERRUPT: u32 = 0x01;
        }
    }
    #[doc = "Enable Error Interrupt 18"]
    pub mod EEI18 {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "An error on channel 18 does not generate an error interrupt"]
            pub const NO_INTERRUPT: u32 = 0;
            #[doc = "An error on channel 18 generates an error interrupt request"]
            pub const INTERRUPT: u32 = 0x01;
        }
    }
    #[doc = "Enable Error Interrupt 19"]
    pub mod EEI19 {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "An error on channel 19 does not generate an error interrupt"]
            pub const NO_INTERRUPT: u32 = 0;
            #[doc = "An error on channel 19 generates an error interrupt request"]
            pub const INTERRUPT: u32 = 0x01;
        }
    }
    #[doc = "Enable Error Interrupt 20"]
    pub mod EEI20 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "An error on channel 20 does not generate an error interrupt"]
            pub const NO_INTERRUPT: u32 = 0;
            #[doc = "An error on channel 20 generates an error interrupt request"]
            pub const INTERRUPT: u32 = 0x01;
        }
    }
    #[doc = "Enable Error Interrupt 21"]
    pub mod EEI21 {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "An error on channel 21 does not generate an error interrupt"]
            pub const NO_INTERRUPT: u32 = 0;
            #[doc = "An error on channel 21 generates an error interrupt request"]
            pub const INTERRUPT: u32 = 0x01;
        }
    }
    #[doc = "Enable Error Interrupt 22"]
    pub mod EEI22 {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "An error on channel 22 does not generate an error interrupt"]
            pub const NO_INTERRUPT: u32 = 0;
            #[doc = "An error on channel 22 generates an error interrupt request"]
            pub const INTERRUPT: u32 = 0x01;
        }
    }
    #[doc = "Enable Error Interrupt 23"]
    pub mod EEI23 {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "An error on channel 23 does not generate an error interrupt"]
            pub const NO_INTERRUPT: u32 = 0;
            #[doc = "An error on channel 23 generates an error interrupt request"]
            pub const INTERRUPT: u32 = 0x01;
        }
    }
    #[doc = "Enable Error Interrupt 24"]
    pub mod EEI24 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "An error on channel 24 does not generate an error interrupt"]
            pub const NO_INTERRUPT: u32 = 0;
            #[doc = "An error on channel 24 generates an error interrupt request"]
            pub const INTERRUPT: u32 = 0x01;
        }
    }
    #[doc = "Enable Error Interrupt 25"]
    pub mod EEI25 {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "An error on channel 25 does not generate an error interrupt"]
            pub const NO_INTERRUPT: u32 = 0;
            #[doc = "An error on channel 25 generates an error interrupt request"]
            pub const INTERRUPT: u32 = 0x01;
        }
    }
    #[doc = "Enable Error Interrupt 26"]
    pub mod EEI26 {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "An error on channel 26 does not generate an error interrupt"]
            pub const NO_INTERRUPT: u32 = 0;
            #[doc = "An error on channel 26 generates an error interrupt request"]
            pub const INTERRUPT: u32 = 0x01;
        }
    }
    #[doc = "Enable Error Interrupt 27"]
    pub mod EEI27 {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "An error on channel 27 does not generate an error interrupt"]
            pub const NO_INTERRUPT: u32 = 0;
            #[doc = "An error on channel 27 generates an error interrupt request"]
            pub const INTERRUPT: u32 = 0x01;
        }
    }
    #[doc = "Enable Error Interrupt 28"]
    pub mod EEI28 {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "An error on channel 28 does not generate an error interrupt"]
            pub const NO_INTERRUPT: u32 = 0;
            #[doc = "An error on channel 28 generates an error interrupt request"]
            pub const INTERRUPT: u32 = 0x01;
        }
    }
    #[doc = "Enable Error Interrupt 29"]
    pub mod EEI29 {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "An error on channel 29 does not generate an error interrupt"]
            pub const NO_INTERRUPT: u32 = 0;
            #[doc = "An error on channel 29 generates an error interrupt request"]
            pub const INTERRUPT: u32 = 0x01;
        }
    }
    #[doc = "Enable Error Interrupt 30"]
    pub mod EEI30 {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "An error on channel 30 does not generate an error interrupt"]
            pub const NO_INTERRUPT: u32 = 0;
            #[doc = "An error on channel 30 generates an error interrupt request"]
            pub const INTERRUPT: u32 = 0x01;
        }
    }
    #[doc = "Enable Error Interrupt 31"]
    pub mod EEI31 {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "An error on channel 31 does not generate an error interrupt"]
            pub const NO_INTERRUPT: u32 = 0;
            #[doc = "An error on channel 31 generates an error interrupt request"]
            pub const INTERRUPT: u32 = 0x01;
        }
    }
}
#[doc = "Clear Enable Error Interrupt"]
pub mod CEEI {
    #[doc = "Clear Enable Error Interrupt"]
    pub mod CEEI {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Clear All Enable Error Interrupts"]
    pub mod CAEE {
        pub const offset: u8 = 6;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Write 0 only to the EEI field specified in the CEEI field"]
            pub const CLEAR_EEI: u8 = 0;
            #[doc = "Write 0 to all fields in EEI"]
            pub const CLEAR_ALL: u8 = 0x01;
        }
    }
    #[doc = "No Op Enable"]
    pub mod NOP {
        pub const offset: u8 = 7;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Normal operation"]
            pub const NORMAL_OPS: u8 = 0;
            #[doc = "No operation, ignore the other fields in this register"]
            pub const NO_OPS: u8 = 0x01;
        }
    }
}
#[doc = "Set Enable Error Interrupt"]
pub mod SEEI {
    #[doc = "Set Enable Error Interrupt"]
    pub mod SEEI {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set All Enable Error Interrupts"]
    pub mod SAEE {
        pub const offset: u8 = 6;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Write 1 only to the EEI field specified in the SEEI field"]
            pub const SET_EEI: u8 = 0;
            #[doc = "Writes 1 to all fields in EEI"]
            pub const SET_ALL: u8 = 0x01;
        }
    }
    #[doc = "No Op Enable"]
    pub mod NOP {
        pub const offset: u8 = 7;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Normal operation"]
            pub const NORMAL_OPS: u8 = 0;
            #[doc = "No operation, ignore the other fields in this register"]
            pub const NO_OPS: u8 = 0x01;
        }
    }
}
#[doc = "Clear Enable Request"]
pub mod CERQ {
    #[doc = "Clear Enable Request"]
    pub mod CERQ {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Clear All Enable Requests"]
    pub mod CAER {
        pub const offset: u8 = 6;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Write 0 to only the ERQ field specified in the CERQ field"]
            pub const CLEAR_ERQ: u8 = 0;
            #[doc = "Write 0 to all fields in ERQ"]
            pub const CLEAR_ALL: u8 = 0x01;
        }
    }
    #[doc = "No Op Enable"]
    pub mod NOP {
        pub const offset: u8 = 7;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Normal operation"]
            pub const NORMAL_OPS: u8 = 0;
            #[doc = "No operation, ignore the other fields in this register"]
            pub const NO_OPS: u8 = 0x01;
        }
    }
}
#[doc = "Set Enable Request"]
pub mod SERQ {
    #[doc = "Set Enable Request"]
    pub mod SERQ {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set All Enable Requests"]
    pub mod SAER {
        pub const offset: u8 = 6;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Write 1 to only the ERQ field specified in the SERQ field"]
            pub const SET_ERQ: u8 = 0;
            #[doc = "Write 1 to all fields in ERQ"]
            pub const SET_ALL: u8 = 0x01;
        }
    }
    #[doc = "No Op Enable"]
    pub mod NOP {
        pub const offset: u8 = 7;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Normal operation"]
            pub const NORMAL_OPS: u8 = 0;
            #[doc = "No operation, ignore the other fields in this register"]
            pub const NO_OPS: u8 = 0x01;
        }
    }
}
#[doc = "Clear DONE Status Bit"]
pub mod CDNE {
    #[doc = "Clear DONE field"]
    pub mod CDNE {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Clears All DONE fields"]
    pub mod CADN {
        pub const offset: u8 = 6;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Writes 0 to only the TCDn_CSR\\[DONE\\] field specified in the CDNE field"]
            pub const CLEAR_DONE: u8 = 0;
            #[doc = "Writes 0 to all bits in TCDn_CSR\\[DONE\\]"]
            pub const CLEAR_ALL: u8 = 0x01;
        }
    }
    #[doc = "No Op Enable"]
    pub mod NOP {
        pub const offset: u8 = 7;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Normal operation"]
            pub const NORMAL_OPS: u8 = 0;
            #[doc = "No operation; all other fields in this register are ignored."]
            pub const NO_OPS: u8 = 0x01;
        }
    }
}
#[doc = "Set START Bit"]
pub mod SSRT {
    #[doc = "Set START field"]
    pub mod SSRT {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set All START fields (activates all channels)"]
    pub mod SAST {
        pub const offset: u8 = 6;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Write 1 to only the TCDn_CSR\\[START\\] field specified in the SSRT field"]
            pub const SET_START: u8 = 0;
            #[doc = "Write 1 to all bits in TCDn_CSR\\[START\\]"]
            pub const SET_ALL: u8 = 0x01;
        }
    }
    #[doc = "No Op Enable"]
    pub mod NOP {
        pub const offset: u8 = 7;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Normal operation"]
            pub const NORMAL_OPS: u8 = 0;
            #[doc = "No operation; all other fields in this register are ignored."]
            pub const NO_OPS: u8 = 0x01;
        }
    }
}
#[doc = "Clear Error"]
pub mod CERR {
    #[doc = "Clear Error Indicator"]
    pub mod CERR {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Clear All Error Indicators"]
    pub mod CAEI {
        pub const offset: u8 = 6;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Write 0 to only the ERR field specified in the CERR field"]
            pub const CLEAR_ERR: u8 = 0;
            #[doc = "Write 0 to all fields in ERR"]
            pub const CLEAR_ALL: u8 = 0x01;
        }
    }
    #[doc = "No Op Enable"]
    pub mod NOP {
        pub const offset: u8 = 7;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Normal operation"]
            pub const NORMAL_OPS: u8 = 0;
            #[doc = "No operation; all other fields in this register are ignored."]
            pub const NO_OPS: u8 = 0x01;
        }
    }
}
#[doc = "Clear Interrupt Request"]
pub mod CINT {
    #[doc = "Clear Interrupt Request"]
    pub mod CINT {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Clear All Interrupt Requests"]
    pub mod CAIR {
        pub const offset: u8 = 6;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Clear only the INT field specified in the CINT field"]
            pub const CLEAR_INT: u8 = 0;
            #[doc = "Clear all bits in INT"]
            pub const CLEAR_ALL: u8 = 0x01;
        }
    }
    #[doc = "No Op Enable"]
    pub mod NOP {
        pub const offset: u8 = 7;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Normal operation"]
            pub const NORMAL_OPS: u8 = 0;
            #[doc = "No operation; all other fields in this register are ignored."]
            pub const NO_OPS: u8 = 0x01;
        }
    }
}
#[doc = "Interrupt Request"]
pub mod INT {
    #[doc = "Interrupt Request 0"]
    pub mod INT0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The interrupt request for channel 0 is cleared"]
            pub const NOT_ACTIVE: u32 = 0;
            #[doc = "The interrupt request for channel 0 is active"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "Interrupt Request 1"]
    pub mod INT1 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The interrupt request for channel 1 is cleared"]
            pub const NOT_ACTIVE: u32 = 0;
            #[doc = "The interrupt request for channel 1 is active"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "Interrupt Request 2"]
    pub mod INT2 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The interrupt request for channel 2 is cleared"]
            pub const NOT_ACTIVE: u32 = 0;
            #[doc = "The interrupt request for channel 2 is active"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "Interrupt Request 3"]
    pub mod INT3 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The interrupt request for channel 3 is cleared"]
            pub const NOT_ACTIVE: u32 = 0;
            #[doc = "The interrupt request for channel 3 is active"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "Interrupt Request 4"]
    pub mod INT4 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The interrupt request for channel 4 is cleared"]
            pub const NOT_ACTIVE: u32 = 0;
            #[doc = "The interrupt request for channel 4 is active"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "Interrupt Request 5"]
    pub mod INT5 {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The interrupt request for channel 5 is cleared"]
            pub const NOT_ACTIVE: u32 = 0;
            #[doc = "The interrupt request for channel 5 is active"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "Interrupt Request 6"]
    pub mod INT6 {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The interrupt request for channel 6 is cleared"]
            pub const NOT_ACTIVE: u32 = 0;
            #[doc = "The interrupt request for channel 6 is active"]
            pub const CTIVE: u32 = 0x01;
        }
    }
    #[doc = "Interrupt Request 7"]
    pub mod INT7 {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The interrupt request for channel 7 is cleared"]
            pub const NOT_ACTIVE: u32 = 0;
            #[doc = "The interrupt request for channel 7 is active"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "Interrupt Request 8"]
    pub mod INT8 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The interrupt request for channel 8 is cleared"]
            pub const NOT_ACTIVE: u32 = 0;
            #[doc = "The interrupt request for channel 8 is active"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "Interrupt Request 9"]
    pub mod INT9 {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The interrupt request for channel 9 is cleared"]
            pub const NOT_ACTIVE: u32 = 0;
            #[doc = "The interrupt request for channel 9 is active"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "Interrupt Request 10"]
    pub mod INT10 {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The interrupt request for channel 10 is cleared"]
            pub const NOT_ACTIVE: u32 = 0;
            #[doc = "The interrupt request for channel 10 is active"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "Interrupt Request 11"]
    pub mod INT11 {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The interrupt request for channel 11 is cleared"]
            pub const NOT_ACTIVE: u32 = 0;
            #[doc = "The interrupt request for channel 11 is active"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "Interrupt Request 12"]
    pub mod INT12 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The interrupt request for channel 12 is cleared"]
            pub const NOT_ACTIVE: u32 = 0;
            #[doc = "The interrupt request for channel 12 is active"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "Interrupt Request 13"]
    pub mod INT13 {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The interrupt request for channel 13 is cleared"]
            pub const NOT_ACTIVE: u32 = 0;
            #[doc = "The interrupt request for channel 13 is active"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "Interrupt Request 14"]
    pub mod INT14 {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The interrupt request for channel 14 is cleared"]
            pub const NOT_ACTIVE: u32 = 0;
            #[doc = "The interrupt request for channel 14 is active"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "Interrupt Request 15"]
    pub mod INT15 {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The interrupt request for channel 15 is cleared"]
            pub const NOT_ACTIVE: u32 = 0;
            #[doc = "The interrupt request for channel 15 is active"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "Interrupt Request 16"]
    pub mod INT16 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The interrupt request for channel 16 is cleared"]
            pub const NOT_ACTIVE: u32 = 0;
            #[doc = "The interrupt request for channel 16 is active"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "Interrupt Request 17"]
    pub mod INT17 {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The interrupt request for channel 17 is cleared"]
            pub const NOT_ACTIVE: u32 = 0;
            #[doc = "The interrupt request for channel 17 is active"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "Interrupt Request 18"]
    pub mod INT18 {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The interrupt request for channel 18 is cleared"]
            pub const NOT_ACTIVE: u32 = 0;
            #[doc = "The interrupt request for channel 18 is active"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "Interrupt Request 19"]
    pub mod INT19 {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The interrupt request for channel 19 is cleared"]
            pub const NOT_ACTIVE: u32 = 0;
            #[doc = "The interrupt request for channel 19 is active"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "Interrupt Request 20"]
    pub mod INT20 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The interrupt request for channel 20 is cleared"]
            pub const NOT_ACTIVE: u32 = 0;
            #[doc = "The interrupt request for channel 20 is active"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "Interrupt Request 21"]
    pub mod INT21 {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The interrupt request for channel 21 is cleared"]
            pub const NOT_ACTIVE: u32 = 0;
            #[doc = "The interrupt request for channel 21 is active"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "Interrupt Request 22"]
    pub mod INT22 {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The interrupt request for channel 22 is cleared"]
            pub const NOT_ACTIVE: u32 = 0;
            #[doc = "The interrupt request for channel 22 is active"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "Interrupt Request 23"]
    pub mod INT23 {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The interrupt request for channel 23 is cleared"]
            pub const NOT_ACTIVE: u32 = 0;
            #[doc = "The interrupt request for channel 23 is active"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "Interrupt Request 24"]
    pub mod INT24 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The interrupt request for channel 24 is cleared"]
            pub const NOT_ACTIVE: u32 = 0;
            #[doc = "The interrupt request for channel 24 is active"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "Interrupt Request 25"]
    pub mod INT25 {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The interrupt request for channel 25 is cleared"]
            pub const NOT_ACTIVE: u32 = 0;
            #[doc = "The interrupt request for channel 25 is active"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "Interrupt Request 26"]
    pub mod INT26 {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The interrupt request for channel 26 is cleared"]
            pub const NOT_ACTIVE: u32 = 0;
            #[doc = "The interrupt request for channel 26 is active"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "Interrupt Request 27"]
    pub mod INT27 {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The interrupt request for channel 27 is cleared"]
            pub const NOT_ACTIVE: u32 = 0;
            #[doc = "The interrupt request for channel 27 is active"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "Interrupt Request 28"]
    pub mod INT28 {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The interrupt request for channel 28 is cleared"]
            pub const NOT_ACTIVE: u32 = 0;
            #[doc = "The interrupt request for channel 28 is active"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "Interrupt Request 29"]
    pub mod INT29 {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The interrupt request for channel 29 is cleared"]
            pub const NOT_ACTIVE: u32 = 0;
            #[doc = "The interrupt request for channel 29 is active"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "Interrupt Request 30"]
    pub mod INT30 {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The interrupt request for channel 30 is cleared"]
            pub const NOT_ACTIVE: u32 = 0;
            #[doc = "The interrupt request for channel 30 is active"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "Interrupt Request 31"]
    pub mod INT31 {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The interrupt request for channel 31 is cleared"]
            pub const NOT_ACTIVE: u32 = 0;
            #[doc = "The interrupt request for channel 31 is active"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
}
#[doc = "Error"]
pub mod ERR {
    #[doc = "Error In Channel 0"]
    pub mod ERR0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No error in this channel has occurred"]
            pub const NO_ERR: u32 = 0;
            #[doc = "An error in this channel has occurred"]
            pub const ERR: u32 = 0x01;
        }
    }
    #[doc = "Error In Channel 1"]
    pub mod ERR1 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No error in this channel has occurred"]
            pub const NO_ERR: u32 = 0;
            #[doc = "An error in this channel has occurred"]
            pub const ERR: u32 = 0x01;
        }
    }
    #[doc = "Error In Channel 2"]
    pub mod ERR2 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No error in this channel has occurred"]
            pub const NO_ERR: u32 = 0;
            #[doc = "An error in this channel has occurred"]
            pub const ERR: u32 = 0x01;
        }
    }
    #[doc = "Error In Channel 3"]
    pub mod ERR3 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No error in this channel has occurred"]
            pub const NO_ERR: u32 = 0;
            #[doc = "An error in this channel has occurred"]
            pub const ERR: u32 = 0x01;
        }
    }
    #[doc = "Error In Channel 4"]
    pub mod ERR4 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No error in this channel has occurred"]
            pub const NO_ERR: u32 = 0;
            #[doc = "An error in this channel has occurred"]
            pub const ERR: u32 = 0x01;
        }
    }
    #[doc = "Error In Channel 5"]
    pub mod ERR5 {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No error in this channel has occurred"]
            pub const NO_ERR: u32 = 0;
            #[doc = "An error in this channel has occurred"]
            pub const ERR: u32 = 0x01;
        }
    }
    #[doc = "Error In Channel 6"]
    pub mod ERR6 {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No error in this channel has occurred"]
            pub const NO_ERR: u32 = 0;
            #[doc = "An error in this channel has occurred"]
            pub const ERR: u32 = 0x01;
        }
    }
    #[doc = "Error In Channel 7"]
    pub mod ERR7 {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No error in this channel has occurred"]
            pub const NO_ERR: u32 = 0;
            #[doc = "An error in this channel has occurred"]
            pub const ERR: u32 = 0x01;
        }
    }
    #[doc = "Error In Channel 8"]
    pub mod ERR8 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No error in this channel has occurred"]
            pub const NO_ERR: u32 = 0;
            #[doc = "An error in this channel has occurred"]
            pub const ERR: u32 = 0x01;
        }
    }
    #[doc = "Error In Channel 9"]
    pub mod ERR9 {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No error in this channel has occurred"]
            pub const NO_ERR: u32 = 0;
            #[doc = "An error in this channel has occurred"]
            pub const ERR: u32 = 0x01;
        }
    }
    #[doc = "Error In Channel 10"]
    pub mod ERR10 {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No error in this channel has occurred"]
            pub const NO_ERR: u32 = 0;
            #[doc = "An error in this channel has occurred"]
            pub const ERR: u32 = 0x01;
        }
    }
    #[doc = "Error In Channel 11"]
    pub mod ERR11 {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No error in this channel has occurred"]
            pub const NO_ERR: u32 = 0;
            #[doc = "An error in this channel has occurred"]
            pub const ERR: u32 = 0x01;
        }
    }
    #[doc = "Error In Channel 12"]
    pub mod ERR12 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No error in this channel has occurred"]
            pub const NO_ERR: u32 = 0;
            #[doc = "An error in this channel has occurred"]
            pub const ERR: u32 = 0x01;
        }
    }
    #[doc = "Error In Channel 13"]
    pub mod ERR13 {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No error in this channel has occurred"]
            pub const NO_ERR: u32 = 0;
            #[doc = "An error in this channel has occurred"]
            pub const ERR: u32 = 0x01;
        }
    }
    #[doc = "Error In Channel 14"]
    pub mod ERR14 {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No error in this channel has occurred"]
            pub const NO_ERR: u32 = 0;
            #[doc = "An error in this channel has occurred"]
            pub const ERR: u32 = 0x01;
        }
    }
    #[doc = "Error In Channel 15"]
    pub mod ERR15 {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No error in this channel has occurred"]
            pub const NO_ERR: u32 = 0;
            #[doc = "An error in this channel has occurred"]
            pub const ERR: u32 = 0x01;
        }
    }
    #[doc = "Error In Channel 16"]
    pub mod ERR16 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No error in this channel has occurred"]
            pub const NO_ERR: u32 = 0;
            #[doc = "An error in this channel has occurred"]
            pub const ERR: u32 = 0x01;
        }
    }
    #[doc = "Error In Channel 17"]
    pub mod ERR17 {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No error in this channel has occurred"]
            pub const NO_ERR: u32 = 0;
            #[doc = "An error in this channel has occurred"]
            pub const ERR: u32 = 0x01;
        }
    }
    #[doc = "Error In Channel 18"]
    pub mod ERR18 {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No error in this channel has occurred"]
            pub const NO_ERR: u32 = 0;
            #[doc = "An error in this channel has occurred"]
            pub const ERR: u32 = 0x01;
        }
    }
    #[doc = "Error In Channel 19"]
    pub mod ERR19 {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No error in this channel has occurred"]
            pub const NO_ERR: u32 = 0;
            #[doc = "An error in this channel has occurred"]
            pub const ERR: u32 = 0x01;
        }
    }
    #[doc = "Error In Channel 20"]
    pub mod ERR20 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No error in this channel has occurred"]
            pub const NO_ERR: u32 = 0;
            #[doc = "An error in this channel has occurred"]
            pub const ERR: u32 = 0x01;
        }
    }
    #[doc = "Error In Channel 21"]
    pub mod ERR21 {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No error in this channel has occurred"]
            pub const NO_ERR: u32 = 0;
            #[doc = "An error in this channel has occurred"]
            pub const ERR: u32 = 0x01;
        }
    }
    #[doc = "Error In Channel 22"]
    pub mod ERR22 {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No error in this channel has occurred"]
            pub const NO_ERR: u32 = 0;
            #[doc = "An error in this channel has occurred"]
            pub const ERR: u32 = 0x01;
        }
    }
    #[doc = "Error In Channel 23"]
    pub mod ERR23 {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No error in this channel has occurred"]
            pub const NO_ERR: u32 = 0;
            #[doc = "An error in this channel has occurred"]
            pub const ERR: u32 = 0x01;
        }
    }
    #[doc = "Error In Channel 24"]
    pub mod ERR24 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No error in this channel has occurred"]
            pub const NO_ERR: u32 = 0;
            #[doc = "An error in this channel has occurred"]
            pub const ERR: u32 = 0x01;
        }
    }
    #[doc = "Error In Channel 25"]
    pub mod ERR25 {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No error in this channel has occurred"]
            pub const NO_ERR: u32 = 0;
            #[doc = "An error in this channel has occurred"]
            pub const ERR: u32 = 0x01;
        }
    }
    #[doc = "Error In Channel 26"]
    pub mod ERR26 {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No error in this channel has occurred"]
            pub const NO_ERR: u32 = 0;
            #[doc = "An error in this channel has occurred"]
            pub const ERR: u32 = 0x01;
        }
    }
    #[doc = "Error In Channel 27"]
    pub mod ERR27 {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No error in this channel has occurred"]
            pub const NO_ERR: u32 = 0;
            #[doc = "An error in this channel has occurred"]
            pub const ERR: u32 = 0x01;
        }
    }
    #[doc = "Error In Channel 28"]
    pub mod ERR28 {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No error in this channel has occurred"]
            pub const NO_ERR: u32 = 0;
            #[doc = "An error in this channel has occurred"]
            pub const ERR: u32 = 0x01;
        }
    }
    #[doc = "Error In Channel 29"]
    pub mod ERR29 {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No error in this channel has occurred"]
            pub const NO_ERR: u32 = 0;
            #[doc = "An error in this channel has occurred"]
            pub const ERR: u32 = 0x01;
        }
    }
    #[doc = "Error In Channel 30"]
    pub mod ERR30 {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No error in this channel has occurred"]
            pub const NO_ERR: u32 = 0;
            #[doc = "An error in this channel has occurred"]
            pub const ERR: u32 = 0x01;
        }
    }
    #[doc = "Error In Channel 31"]
    pub mod ERR31 {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No error in this channel has occurred"]
            pub const NO_ERR: u32 = 0;
            #[doc = "An error in this channel has occurred"]
            pub const ERR: u32 = 0x01;
        }
    }
}
#[doc = "Hardware Request Status"]
pub mod HRS {
    #[doc = "Hardware Request Status Channel 0"]
    pub mod HRS0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "A hardware service request for channel 0 is not present"]
            pub const NO_HWRQST: u32 = 0;
            #[doc = "A hardware service request for channel 0 is present"]
            pub const HWRQST: u32 = 0x01;
        }
    }
    #[doc = "Hardware Request Status Channel 1"]
    pub mod HRS1 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "A hardware service request for channel 1 is not present"]
            pub const NO_HWRQST: u32 = 0;
            #[doc = "A hardware service request for channel 1 is present"]
            pub const HWRQST: u32 = 0x01;
        }
    }
    #[doc = "Hardware Request Status Channel 2"]
    pub mod HRS2 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "A hardware service request for channel 2 is not present"]
            pub const NO_HWRQST: u32 = 0;
            #[doc = "A hardware service request for channel 2 is present"]
            pub const HWRQST: u32 = 0x01;
        }
    }
    #[doc = "Hardware Request Status Channel 3"]
    pub mod HRS3 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "A hardware service request for channel 3 is not present"]
            pub const NO_HWRQST: u32 = 0;
            #[doc = "A hardware service request for channel 3 is present"]
            pub const HWRQST: u32 = 0x01;
        }
    }
    #[doc = "Hardware Request Status Channel 4"]
    pub mod HRS4 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "A hardware service request for channel 4 is not present"]
            pub const NO_HWRQST: u32 = 0;
            #[doc = "A hardware service request for channel 4 is present"]
            pub const HWRQST: u32 = 0x01;
        }
    }
    #[doc = "Hardware Request Status Channel 5"]
    pub mod HRS5 {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "A hardware service request for channel 5 is not present"]
            pub const NO_HWRQST: u32 = 0;
            #[doc = "A hardware service request for channel 5 is present"]
            pub const HWRQST: u32 = 0x01;
        }
    }
    #[doc = "Hardware Request Status Channel 6"]
    pub mod HRS6 {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "A hardware service request for channel 6 is not present"]
            pub const NO_HWRQST: u32 = 0;
            #[doc = "A hardware service request for channel 6 is present"]
            pub const HWRQST: u32 = 0x01;
        }
    }
    #[doc = "Hardware Request Status Channel 7"]
    pub mod HRS7 {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "A hardware service request for channel 7 is not present"]
            pub const NO_HWRQST: u32 = 0;
            #[doc = "A hardware service request for channel 7 is present"]
            pub const HWRQST: u32 = 0x01;
        }
    }
    #[doc = "Hardware Request Status Channel 8"]
    pub mod HRS8 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "A hardware service request for channel 8 is not present"]
            pub const NO_HWRQST: u32 = 0;
            #[doc = "A hardware service request for channel 8 is present"]
            pub const HWRQST: u32 = 0x01;
        }
    }
    #[doc = "Hardware Request Status Channel 9"]
    pub mod HRS9 {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "A hardware service request for channel 9 is not present"]
            pub const NO_HWRQST: u32 = 0;
            #[doc = "A hardware service request for channel 9 is present"]
            pub const HWRQST: u32 = 0x01;
        }
    }
    #[doc = "Hardware Request Status Channel 10"]
    pub mod HRS10 {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "A hardware service request for channel 10 is not present"]
            pub const NO_HWRQST: u32 = 0;
            #[doc = "A hardware service request for channel 10 is present"]
            pub const HWRQST: u32 = 0x01;
        }
    }
    #[doc = "Hardware Request Status Channel 11"]
    pub mod HRS11 {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "A hardware service request for channel 11 is not present"]
            pub const NO_HWRQST: u32 = 0;
            #[doc = "A hardware service request for channel 11 is present"]
            pub const HWRQST: u32 = 0x01;
        }
    }
    #[doc = "Hardware Request Status Channel 12"]
    pub mod HRS12 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "A hardware service request for channel 12 is not present"]
            pub const NO_HWRQST: u32 = 0;
            #[doc = "A hardware service request for channel 12 is present"]
            pub const HWRQST: u32 = 0x01;
        }
    }
    #[doc = "Hardware Request Status Channel 13"]
    pub mod HRS13 {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "A hardware service request for channel 13 is not present"]
            pub const NO_HWRQST: u32 = 0;
            #[doc = "A hardware service request for channel 13 is present"]
            pub const HWRQST: u32 = 0x01;
        }
    }
    #[doc = "Hardware Request Status Channel 14"]
    pub mod HRS14 {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "A hardware service request for channel 14 is not present"]
            pub const NO_HWRQST: u32 = 0;
            #[doc = "A hardware service request for channel 14 is present"]
            pub const HWRQST: u32 = 0x01;
        }
    }
    #[doc = "Hardware Request Status Channel 15"]
    pub mod HRS15 {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "A hardware service request for channel 15 is not present"]
            pub const NO_HWRQST: u32 = 0;
            #[doc = "A hardware service request for channel 15 is present"]
            pub const HWRQST: u32 = 0x01;
        }
    }
    #[doc = "Hardware Request Status Channel 16"]
    pub mod HRS16 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "A hardware service request for channel 16 is not present"]
            pub const NO_HWRQST: u32 = 0;
            #[doc = "A hardware service request for channel 16 is present"]
            pub const HWRQST: u32 = 0x01;
        }
    }
    #[doc = "Hardware Request Status Channel 17"]
    pub mod HRS17 {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "A hardware service request for channel 17 is not present"]
            pub const NO_HWRQST: u32 = 0;
            #[doc = "A hardware service request for channel 17 is present"]
            pub const HWRQST: u32 = 0x01;
        }
    }
    #[doc = "Hardware Request Status Channel 18"]
    pub mod HRS18 {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "A hardware service request for channel 18 is not present"]
            pub const NO_HWRQST: u32 = 0;
            #[doc = "A hardware service request for channel 18 is present"]
            pub const HWRQST: u32 = 0x01;
        }
    }
    #[doc = "Hardware Request Status Channel 19"]
    pub mod HRS19 {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "A hardware service request for channel 19 is not present"]
            pub const NO_HWRQST: u32 = 0;
            #[doc = "A hardware service request for channel 19 is present"]
            pub const HWRQST: u32 = 0x01;
        }
    }
    #[doc = "Hardware Request Status Channel 20"]
    pub mod HRS20 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "A hardware service request for channel 20 is not present"]
            pub const NO_HWRQST: u32 = 0;
            #[doc = "A hardware service request for channel 20 is present"]
            pub const HWRQST: u32 = 0x01;
        }
    }
    #[doc = "Hardware Request Status Channel 21"]
    pub mod HRS21 {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "A hardware service request for channel 21 is not present"]
            pub const NO_HWRQST: u32 = 0;
            #[doc = "A hardware service request for channel 21 is present"]
            pub const HWRQST: u32 = 0x01;
        }
    }
    #[doc = "Hardware Request Status Channel 22"]
    pub mod HRS22 {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "A hardware service request for channel 22 is not present"]
            pub const NO_HWRQST: u32 = 0;
            #[doc = "A hardware service request for channel 22 is present"]
            pub const HWRQST: u32 = 0x01;
        }
    }
    #[doc = "Hardware Request Status Channel 23"]
    pub mod HRS23 {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "A hardware service request for channel 23 is not present"]
            pub const NO_HWRQST: u32 = 0;
            #[doc = "A hardware service request for channel 23 is present"]
            pub const HWRQST: u32 = 0x01;
        }
    }
    #[doc = "Hardware Request Status Channel 24"]
    pub mod HRS24 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "A hardware service request for channel 24 is not present"]
            pub const NO_HWRQST: u32 = 0;
            #[doc = "A hardware service request for channel 24 is present"]
            pub const HWRQST: u32 = 0x01;
        }
    }
    #[doc = "Hardware Request Status Channel 25"]
    pub mod HRS25 {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "A hardware service request for channel 25 is not present"]
            pub const NO_HWRQST: u32 = 0;
            #[doc = "A hardware service request for channel 25 is present"]
            pub const HWRQST: u32 = 0x01;
        }
    }
    #[doc = "Hardware Request Status Channel 26"]
    pub mod HRS26 {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "A hardware service request for channel 26 is not present"]
            pub const NO_HWRQST: u32 = 0;
            #[doc = "A hardware service request for channel 26 is present"]
            pub const HWRQST: u32 = 0x01;
        }
    }
    #[doc = "Hardware Request Status Channel 27"]
    pub mod HRS27 {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "A hardware service request for channel 27 is not present"]
            pub const NO_HWRQST: u32 = 0;
            #[doc = "A hardware service request for channel 27 is present"]
            pub const HWRQST: u32 = 0x01;
        }
    }
    #[doc = "Hardware Request Status Channel 28"]
    pub mod HRS28 {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "A hardware service request for channel 28 is not present"]
            pub const NO_HWRQST: u32 = 0;
            #[doc = "A hardware service request for channel 28 is present"]
            pub const HWRQST: u32 = 0x01;
        }
    }
    #[doc = "Hardware Request Status Channel 29"]
    pub mod HRS29 {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "A hardware service request for channel 29 is not preset"]
            pub const NO_HWRQST: u32 = 0;
            #[doc = "A hardware service request for channel 29 is present"]
            pub const HWRQST: u32 = 0x01;
        }
    }
    #[doc = "Hardware Request Status Channel 30"]
    pub mod HRS30 {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "A hardware service request for channel 30 is not present"]
            pub const NO_HWRQST: u32 = 0;
            #[doc = "A hardware service request for channel 30 is present"]
            pub const HWRQST: u32 = 0x01;
        }
    }
    #[doc = "Hardware Request Status Channel 31"]
    pub mod HRS31 {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "A hardware service request for channel 31 is not present"]
            pub const NO_HWRQST: u32 = 0;
            #[doc = "A hardware service request for channel 31 is present"]
            pub const HWRQST: u32 = 0x01;
        }
    }
}
#[doc = "Enable Asynchronous Request in Stop"]
pub mod EARS {
    #[doc = "Enable asynchronous DMA request in stop mode for channel 0."]
    pub mod EDREQ_0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable asynchronous DMA request for channel 0"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable asynchronous DMA request for channel 0"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 1."]
    pub mod EDREQ_1 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable asynchronous DMA request for channel 1"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable asynchronous DMA request for channel 1"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 2."]
    pub mod EDREQ_2 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable asynchronous DMA request for channel 2"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable asynchronous DMA request for channel 2"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 3."]
    pub mod EDREQ_3 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable asynchronous DMA request for channel 3"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable asynchronous DMA request for channel 3"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 4."]
    pub mod EDREQ_4 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable asynchronous DMA request for channel 4"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable asynchronous DMA request for channel 4"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 5."]
    pub mod EDREQ_5 {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable asynchronous DMA request for channel 5"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable asynchronous DMA request for channel 5"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 6."]
    pub mod EDREQ_6 {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable asynchronous DMA request for channel 6"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable asynchronous DMA request for channel 6"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 7."]
    pub mod EDREQ_7 {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable asynchronous DMA request for channel 7"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable asynchronous DMA request for channel 7"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 8."]
    pub mod EDREQ_8 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable asynchronous DMA request for channel 8"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable asynchronous DMA request for channel 8"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 9."]
    pub mod EDREQ_9 {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable asynchronous DMA request for channel 9"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable asynchronous DMA request for channel 9"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 10."]
    pub mod EDREQ_10 {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable asynchronous DMA request for channel 10"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable asynchronous DMA request for channel 10"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 11."]
    pub mod EDREQ_11 {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable asynchronous DMA request for channel 11"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable asynchronous DMA request for channel 11"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 12."]
    pub mod EDREQ_12 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable asynchronous DMA request for channel 12"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable asynchronous DMA request for channel 12"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 13."]
    pub mod EDREQ_13 {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable asynchronous DMA request for channel 13"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable asynchronous DMA request for channel 13"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 14."]
    pub mod EDREQ_14 {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable asynchronous DMA request for channel 14"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable asynchronous DMA request for channel 14"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 15."]
    pub mod EDREQ_15 {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable asynchronous DMA request for channel 15"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable asynchronous DMA request for channel 15"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 16."]
    pub mod EDREQ_16 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable asynchronous DMA request for channel 16"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable asynchronous DMA request for channel 16"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 17."]
    pub mod EDREQ_17 {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable asynchronous DMA request for channel 17"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable asynchronous DMA request for channel 17"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 18."]
    pub mod EDREQ_18 {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable asynchronous DMA request for channel 18"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable asynchronous DMA request for channel 18"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 19."]
    pub mod EDREQ_19 {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable asynchronous DMA request for channel 19"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable asynchronous DMA request for channel 19"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 20."]
    pub mod EDREQ_20 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable asynchronous DMA request for channel 20"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable asynchronous DMA request for channel 20"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 21."]
    pub mod EDREQ_21 {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable asynchronous DMA request for channel 21"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable asynchronous DMA request for channel 21"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 22."]
    pub mod EDREQ_22 {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable asynchronous DMA request for channel 22"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable asynchronous DMA request for channel 22"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 23."]
    pub mod EDREQ_23 {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable asynchronous DMA request for channel 23"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable asynchronous DMA request for channel 23"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 24."]
    pub mod EDREQ_24 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable asynchronous DMA request for channel 24"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable asynchronous DMA request for channel 24"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 25."]
    pub mod EDREQ_25 {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable asynchronous DMA request for channel 25"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable asynchronous DMA request for channel 25"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 26."]
    pub mod EDREQ_26 {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable asynchronous DMA request for channel 26"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable asynchronous DMA request for channel 26"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 27."]
    pub mod EDREQ_27 {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable asynchronous DMA request for channel 27"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable asynchronous DMA request for channel 27"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 28."]
    pub mod EDREQ_28 {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable asynchronous DMA request for channel 28"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable asynchronous DMA request for channel 28"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 29."]
    pub mod EDREQ_29 {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable asynchronous DMA request for channel 29"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable asynchronous DMA request for channel 29"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 30."]
    pub mod EDREQ_30 {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable asynchronous DMA request for channel 30"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable asynchronous DMA request for channel 30"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 31."]
    pub mod EDREQ_31 {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable asynchronous DMA request for channel 31"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable asynchronous DMA request for channel 31"]
            pub const ENABLE: u32 = 0x01;
        }
    }
}
#[doc = "Channel Priority"]
pub mod DCHPRI3 {
    #[doc = "Channel n Arbitration Priority"]
    pub mod CHPRI {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Channel n Current Group Priority"]
    pub mod GRPPRI {
        pub const offset: u8 = 4;
        pub const mask: u8 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Disable Preempt Ability. This field resets to 0."]
    pub mod DPA {
        pub const offset: u8 = 6;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Channel n can suspend a lower priority channel"]
            pub const ENABLED: u8 = 0;
            #[doc = "Channel n cannot suspend any channel, regardless of channel priority"]
            pub const DISABLED: u8 = 0x01;
        }
    }
    #[doc = "Enable Channel Preemption. This field resets to 0."]
    pub mod ECP {
        pub const offset: u8 = 7;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Channel n cannot be suspended by a higher priority channel's service request"]
            pub const DISABLED: u8 = 0;
            #[doc = "Channel n can be temporarily suspended by the service request of a higher priority channel"]
            pub const ENABLED: u8 = 0x01;
        }
    }
}
#[doc = "Channel Priority"]
pub mod DCHPRI2 {
    #[doc = "Channel n Arbitration Priority"]
    pub mod CHPRI {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Channel n Current Group Priority"]
    pub mod GRPPRI {
        pub const offset: u8 = 4;
        pub const mask: u8 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Disable Preempt Ability. This field resets to 0."]
    pub mod DPA {
        pub const offset: u8 = 6;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Channel n can suspend a lower priority channel"]
            pub const ENABLED: u8 = 0;
            #[doc = "Channel n cannot suspend any channel, regardless of channel priority"]
            pub const DISABLED: u8 = 0x01;
        }
    }
    #[doc = "Enable Channel Preemption. This field resets to 0."]
    pub mod ECP {
        pub const offset: u8 = 7;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Channel n cannot be suspended by a higher priority channel's service request"]
            pub const DISABLED: u8 = 0;
            #[doc = "Channel n can be temporarily suspended by the service request of a higher priority channel"]
            pub const ENABLED: u8 = 0x01;
        }
    }
}
#[doc = "Channel Priority"]
pub mod DCHPRI1 {
    #[doc = "Channel n Arbitration Priority"]
    pub mod CHPRI {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Channel n Current Group Priority"]
    pub mod GRPPRI {
        pub const offset: u8 = 4;
        pub const mask: u8 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Disable Preempt Ability. This field resets to 0."]
    pub mod DPA {
        pub const offset: u8 = 6;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Channel n can suspend a lower priority channel"]
            pub const ENABLED: u8 = 0;
            #[doc = "Channel n cannot suspend any channel, regardless of channel priority"]
            pub const DISABLED: u8 = 0x01;
        }
    }
    #[doc = "Enable Channel Preemption. This field resets to 0."]
    pub mod ECP {
        pub const offset: u8 = 7;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Channel n cannot be suspended by a higher priority channel's service request"]
            pub const DISABLED: u8 = 0;
            #[doc = "Channel n can be temporarily suspended by the service request of a higher priority channel"]
            pub const ENABLED: u8 = 0x01;
        }
    }
}
#[doc = "Channel Priority"]
pub mod DCHPRI0 {
    #[doc = "Channel n Arbitration Priority"]
    pub mod CHPRI {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Channel n Current Group Priority"]
    pub mod GRPPRI {
        pub const offset: u8 = 4;
        pub const mask: u8 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Disable Preempt Ability. This field resets to 0."]
    pub mod DPA {
        pub const offset: u8 = 6;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Channel n can suspend a lower priority channel"]
            pub const ENABLED: u8 = 0;
            #[doc = "Channel n cannot suspend any channel, regardless of channel priority"]
            pub const DISABLED: u8 = 0x01;
        }
    }
    #[doc = "Enable Channel Preemption. This field resets to 0."]
    pub mod ECP {
        pub const offset: u8 = 7;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Channel n cannot be suspended by a higher priority channel's service request"]
            pub const DISABLED: u8 = 0;
            #[doc = "Channel n can be temporarily suspended by the service request of a higher priority channel"]
            pub const ENABLED: u8 = 0x01;
        }
    }
}
#[doc = "Channel Priority"]
pub mod DCHPRI7 {
    #[doc = "Channel n Arbitration Priority"]
    pub mod CHPRI {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Channel n Current Group Priority"]
    pub mod GRPPRI {
        pub const offset: u8 = 4;
        pub const mask: u8 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Disable Preempt Ability. This field resets to 0."]
    pub mod DPA {
        pub const offset: u8 = 6;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Channel n can suspend a lower priority channel"]
            pub const ENABLED: u8 = 0;
            #[doc = "Channel n cannot suspend any channel, regardless of channel priority"]
            pub const DISABLED: u8 = 0x01;
        }
    }
    #[doc = "Enable Channel Preemption. This field resets to 0."]
    pub mod ECP {
        pub const offset: u8 = 7;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Channel n cannot be suspended by a higher priority channel's service request"]
            pub const DISABLED: u8 = 0;
            #[doc = "Channel n can be temporarily suspended by the service request of a higher priority channel"]
            pub const ENABLED: u8 = 0x01;
        }
    }
}
#[doc = "Channel Priority"]
pub mod DCHPRI6 {
    #[doc = "Channel n Arbitration Priority"]
    pub mod CHPRI {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Channel n Current Group Priority"]
    pub mod GRPPRI {
        pub const offset: u8 = 4;
        pub const mask: u8 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Disable Preempt Ability. This field resets to 0."]
    pub mod DPA {
        pub const offset: u8 = 6;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Channel n can suspend a lower priority channel"]
            pub const ENABLED: u8 = 0;
            #[doc = "Channel n cannot suspend any channel, regardless of channel priority"]
            pub const DISABLED: u8 = 0x01;
        }
    }
    #[doc = "Enable Channel Preemption. This field resets to 0."]
    pub mod ECP {
        pub const offset: u8 = 7;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Channel n cannot be suspended by a higher priority channel's service request"]
            pub const DISABLED: u8 = 0;
            #[doc = "Channel n can be temporarily suspended by the service request of a higher priority channel"]
            pub const ENABLED: u8 = 0x01;
        }
    }
}
#[doc = "Channel Priority"]
pub mod DCHPRI5 {
    #[doc = "Channel n Arbitration Priority"]
    pub mod CHPRI {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Channel n Current Group Priority"]
    pub mod GRPPRI {
        pub const offset: u8 = 4;
        pub const mask: u8 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Disable Preempt Ability. This field resets to 0."]
    pub mod DPA {
        pub const offset: u8 = 6;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Channel n can suspend a lower priority channel"]
            pub const ENABLED: u8 = 0;
            #[doc = "Channel n cannot suspend any channel, regardless of channel priority"]
            pub const DISABLED: u8 = 0x01;
        }
    }
    #[doc = "Enable Channel Preemption. This field resets to 0."]
    pub mod ECP {
        pub const offset: u8 = 7;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Channel n cannot be suspended by a higher priority channel's service request"]
            pub const DISABLED: u8 = 0;
            #[doc = "Channel n can be temporarily suspended by the service request of a higher priority channel"]
            pub const ENABLED: u8 = 0x01;
        }
    }
}
#[doc = "Channel Priority"]
pub mod DCHPRI4 {
    #[doc = "Channel n Arbitration Priority"]
    pub mod CHPRI {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Channel n Current Group Priority"]
    pub mod GRPPRI {
        pub const offset: u8 = 4;
        pub const mask: u8 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Disable Preempt Ability. This field resets to 0."]
    pub mod DPA {
        pub const offset: u8 = 6;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Channel n can suspend a lower priority channel"]
            pub const ENABLED: u8 = 0;
            #[doc = "Channel n cannot suspend any channel, regardless of channel priority"]
            pub const DISABLED: u8 = 0x01;
        }
    }
    #[doc = "Enable Channel Preemption. This field resets to 0."]
    pub mod ECP {
        pub const offset: u8 = 7;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Channel n cannot be suspended by a higher priority channel's service request"]
            pub const DISABLED: u8 = 0;
            #[doc = "Channel n can be temporarily suspended by the service request of a higher priority channel"]
            pub const ENABLED: u8 = 0x01;
        }
    }
}
#[doc = "Channel Priority"]
pub mod DCHPRI11 {
    #[doc = "Channel n Arbitration Priority"]
    pub mod CHPRI {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Channel n Current Group Priority"]
    pub mod GRPPRI {
        pub const offset: u8 = 4;
        pub const mask: u8 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Disable Preempt Ability. This field resets to 0."]
    pub mod DPA {
        pub const offset: u8 = 6;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Channel n can suspend a lower priority channel"]
            pub const ENABLED: u8 = 0;
            #[doc = "Channel n cannot suspend any channel, regardless of channel priority"]
            pub const DISABLED: u8 = 0x01;
        }
    }
    #[doc = "Enable Channel Preemption. This field resets to 0."]
    pub mod ECP {
        pub const offset: u8 = 7;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Channel n cannot be suspended by a higher priority channel's service request"]
            pub const DISABLED: u8 = 0;
            #[doc = "Channel n can be temporarily suspended by the service request of a higher priority channel"]
            pub const ENABLED: u8 = 0x01;
        }
    }
}
#[doc = "Channel Priority"]
pub mod DCHPRI10 {
    #[doc = "Channel n Arbitration Priority"]
    pub mod CHPRI {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Channel n Current Group Priority"]
    pub mod GRPPRI {
        pub const offset: u8 = 4;
        pub const mask: u8 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Disable Preempt Ability. This field resets to 0."]
    pub mod DPA {
        pub const offset: u8 = 6;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Channel n can suspend a lower priority channel"]
            pub const ENABLED: u8 = 0;
            #[doc = "Channel n cannot suspend any channel, regardless of channel priority"]
            pub const DISABLED: u8 = 0x01;
        }
    }
    #[doc = "Enable Channel Preemption. This field resets to 0."]
    pub mod ECP {
        pub const offset: u8 = 7;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Channel n cannot be suspended by a higher priority channel's service request"]
            pub const DISABLED: u8 = 0;
            #[doc = "Channel n can be temporarily suspended by the service request of a higher priority channel"]
            pub const ENABLED: u8 = 0x01;
        }
    }
}
#[doc = "Channel Priority"]
pub mod DCHPRI9 {
    #[doc = "Channel n Arbitration Priority"]
    pub mod CHPRI {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Channel n Current Group Priority"]
    pub mod GRPPRI {
        pub const offset: u8 = 4;
        pub const mask: u8 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Disable Preempt Ability. This field resets to 0."]
    pub mod DPA {
        pub const offset: u8 = 6;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Channel n can suspend a lower priority channel"]
            pub const ENABLED: u8 = 0;
            #[doc = "Channel n cannot suspend any channel, regardless of channel priority"]
            pub const DISABLED: u8 = 0x01;
        }
    }
    #[doc = "Enable Channel Preemption. This field resets to 0."]
    pub mod ECP {
        pub const offset: u8 = 7;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Channel n cannot be suspended by a higher priority channel's service request"]
            pub const DISABLED: u8 = 0;
            #[doc = "Channel n can be temporarily suspended by the service request of a higher priority channel"]
            pub const ENABLED: u8 = 0x01;
        }
    }
}
#[doc = "Channel Priority"]
pub mod DCHPRI8 {
    #[doc = "Channel n Arbitration Priority"]
    pub mod CHPRI {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Channel n Current Group Priority"]
    pub mod GRPPRI {
        pub const offset: u8 = 4;
        pub const mask: u8 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Disable Preempt Ability. This field resets to 0."]
    pub mod DPA {
        pub const offset: u8 = 6;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Channel n can suspend a lower priority channel"]
            pub const ENABLED: u8 = 0;
            #[doc = "Channel n cannot suspend any channel, regardless of channel priority"]
            pub const DISABLED: u8 = 0x01;
        }
    }
    #[doc = "Enable Channel Preemption. This field resets to 0."]
    pub mod ECP {
        pub const offset: u8 = 7;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Channel n cannot be suspended by a higher priority channel's service request"]
            pub const DISABLED: u8 = 0;
            #[doc = "Channel n can be temporarily suspended by the service request of a higher priority channel"]
            pub const ENABLED: u8 = 0x01;
        }
    }
}
#[doc = "Channel Priority"]
pub mod DCHPRI15 {
    #[doc = "Channel n Arbitration Priority"]
    pub mod CHPRI {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Channel n Current Group Priority"]
    pub mod GRPPRI {
        pub const offset: u8 = 4;
        pub const mask: u8 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Disable Preempt Ability. This field resets to 0."]
    pub mod DPA {
        pub const offset: u8 = 6;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Channel n can suspend a lower priority channel"]
            pub const ENABLED: u8 = 0;
            #[doc = "Channel n cannot suspend any channel, regardless of channel priority"]
            pub const DISABLED: u8 = 0x01;
        }
    }
    #[doc = "Enable Channel Preemption. This field resets to 0."]
    pub mod ECP {
        pub const offset: u8 = 7;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Channel n cannot be suspended by a higher priority channel's service request"]
            pub const DISABLED: u8 = 0;
            #[doc = "Channel n can be temporarily suspended by the service request of a higher priority channel"]
            pub const ENABLED: u8 = 0x01;
        }
    }
}
#[doc = "Channel Priority"]
pub mod DCHPRI14 {
    #[doc = "Channel n Arbitration Priority"]
    pub mod CHPRI {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Channel n Current Group Priority"]
    pub mod GRPPRI {
        pub const offset: u8 = 4;
        pub const mask: u8 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Disable Preempt Ability. This field resets to 0."]
    pub mod DPA {
        pub const offset: u8 = 6;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Channel n can suspend a lower priority channel"]
            pub const ENABLED: u8 = 0;
            #[doc = "Channel n cannot suspend any channel, regardless of channel priority"]
            pub const DISABLED: u8 = 0x01;
        }
    }
    #[doc = "Enable Channel Preemption. This field resets to 0."]
    pub mod ECP {
        pub const offset: u8 = 7;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Channel n cannot be suspended by a higher priority channel's service request"]
            pub const DISABLED: u8 = 0;
            #[doc = "Channel n can be temporarily suspended by the service request of a higher priority channel"]
            pub const ENABLED: u8 = 0x01;
        }
    }
}
#[doc = "Channel Priority"]
pub mod DCHPRI13 {
    #[doc = "Channel n Arbitration Priority"]
    pub mod CHPRI {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Channel n Current Group Priority"]
    pub mod GRPPRI {
        pub const offset: u8 = 4;
        pub const mask: u8 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Disable Preempt Ability. This field resets to 0."]
    pub mod DPA {
        pub const offset: u8 = 6;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Channel n can suspend a lower priority channel"]
            pub const ENABLED: u8 = 0;
            #[doc = "Channel n cannot suspend any channel, regardless of channel priority"]
            pub const DISABLED: u8 = 0x01;
        }
    }
    #[doc = "Enable Channel Preemption. This field resets to 0."]
    pub mod ECP {
        pub const offset: u8 = 7;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Channel n cannot be suspended by a higher priority channel's service request"]
            pub const DISABLED: u8 = 0;
            #[doc = "Channel n can be temporarily suspended by the service request of a higher priority channel"]
            pub const ENABLED: u8 = 0x01;
        }
    }
}
#[doc = "Channel Priority"]
pub mod DCHPRI12 {
    #[doc = "Channel n Arbitration Priority"]
    pub mod CHPRI {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Channel n Current Group Priority"]
    pub mod GRPPRI {
        pub const offset: u8 = 4;
        pub const mask: u8 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Disable Preempt Ability. This field resets to 0."]
    pub mod DPA {
        pub const offset: u8 = 6;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Channel n can suspend a lower priority channel"]
            pub const ENABLED: u8 = 0;
            #[doc = "Channel n cannot suspend any channel, regardless of channel priority"]
            pub const DISABLED: u8 = 0x01;
        }
    }
    #[doc = "Enable Channel Preemption. This field resets to 0."]
    pub mod ECP {
        pub const offset: u8 = 7;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Channel n cannot be suspended by a higher priority channel's service request"]
            pub const DISABLED: u8 = 0;
            #[doc = "Channel n can be temporarily suspended by the service request of a higher priority channel"]
            pub const ENABLED: u8 = 0x01;
        }
    }
}
#[doc = "Channel Priority"]
pub mod DCHPRI19 {
    #[doc = "Channel n Arbitration Priority"]
    pub mod CHPRI {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Channel n Current Group Priority"]
    pub mod GRPPRI {
        pub const offset: u8 = 4;
        pub const mask: u8 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Disable Preempt Ability. This field resets to 0."]
    pub mod DPA {
        pub const offset: u8 = 6;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Channel n can suspend a lower priority channel"]
            pub const ENABLED: u8 = 0;
            #[doc = "Channel n cannot suspend any channel, regardless of channel priority"]
            pub const DISABLED: u8 = 0x01;
        }
    }
    #[doc = "Enable Channel Preemption. This field resets to 0."]
    pub mod ECP {
        pub const offset: u8 = 7;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Channel n cannot be suspended by a higher priority channel's service request"]
            pub const DISABLED: u8 = 0;
            #[doc = "Channel n can be temporarily suspended by the service request of a higher priority channel"]
            pub const ENABLED: u8 = 0x01;
        }
    }
}
#[doc = "Channel Priority"]
pub mod DCHPRI18 {
    #[doc = "Channel n Arbitration Priority"]
    pub mod CHPRI {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Channel n Current Group Priority"]
    pub mod GRPPRI {
        pub const offset: u8 = 4;
        pub const mask: u8 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Disable Preempt Ability. This field resets to 0."]
    pub mod DPA {
        pub const offset: u8 = 6;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Channel n can suspend a lower priority channel"]
            pub const ENABLED: u8 = 0;
            #[doc = "Channel n cannot suspend any channel, regardless of channel priority"]
            pub const DISABLED: u8 = 0x01;
        }
    }
    #[doc = "Enable Channel Preemption. This field resets to 0."]
    pub mod ECP {
        pub const offset: u8 = 7;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Channel n cannot be suspended by a higher priority channel's service request"]
            pub const DISABLED: u8 = 0;
            #[doc = "Channel n can be temporarily suspended by the service request of a higher priority channel"]
            pub const ENABLED: u8 = 0x01;
        }
    }
}
#[doc = "Channel Priority"]
pub mod DCHPRI17 {
    #[doc = "Channel n Arbitration Priority"]
    pub mod CHPRI {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Channel n Current Group Priority"]
    pub mod GRPPRI {
        pub const offset: u8 = 4;
        pub const mask: u8 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Disable Preempt Ability. This field resets to 0."]
    pub mod DPA {
        pub const offset: u8 = 6;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Channel n can suspend a lower priority channel"]
            pub const ENABLED: u8 = 0;
            #[doc = "Channel n cannot suspend any channel, regardless of channel priority"]
            pub const DISABLED: u8 = 0x01;
        }
    }
    #[doc = "Enable Channel Preemption. This field resets to 0."]
    pub mod ECP {
        pub const offset: u8 = 7;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Channel n cannot be suspended by a higher priority channel's service request"]
            pub const DISABLED: u8 = 0;
            #[doc = "Channel n can be temporarily suspended by the service request of a higher priority channel"]
            pub const ENABLED: u8 = 0x01;
        }
    }
}
#[doc = "Channel Priority"]
pub mod DCHPRI16 {
    #[doc = "Channel n Arbitration Priority"]
    pub mod CHPRI {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Channel n Current Group Priority"]
    pub mod GRPPRI {
        pub const offset: u8 = 4;
        pub const mask: u8 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Disable Preempt Ability. This field resets to 0."]
    pub mod DPA {
        pub const offset: u8 = 6;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Channel n can suspend a lower priority channel"]
            pub const ENABLED: u8 = 0;
            #[doc = "Channel n cannot suspend any channel, regardless of channel priority"]
            pub const DISABLED: u8 = 0x01;
        }
    }
    #[doc = "Enable Channel Preemption. This field resets to 0."]
    pub mod ECP {
        pub const offset: u8 = 7;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Channel n cannot be suspended by a higher priority channel's service request"]
            pub const DISABLED: u8 = 0;
            #[doc = "Channel n can be temporarily suspended by the service request of a higher priority channel"]
            pub const ENABLED: u8 = 0x01;
        }
    }
}
#[doc = "Channel Priority"]
pub mod DCHPRI23 {
    #[doc = "Channel n Arbitration Priority"]
    pub mod CHPRI {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Channel n Current Group Priority"]
    pub mod GRPPRI {
        pub const offset: u8 = 4;
        pub const mask: u8 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Disable Preempt Ability. This field resets to 0."]
    pub mod DPA {
        pub const offset: u8 = 6;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Channel n can suspend a lower priority channel"]
            pub const ENABLED: u8 = 0;
            #[doc = "Channel n cannot suspend any channel, regardless of channel priority"]
            pub const DISABLED: u8 = 0x01;
        }
    }
    #[doc = "Enable Channel Preemption. This field resets to 0."]
    pub mod ECP {
        pub const offset: u8 = 7;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Channel n cannot be suspended by a higher priority channel's service request"]
            pub const DISABLED: u8 = 0;
            #[doc = "Channel n can be temporarily suspended by the service request of a higher priority channel"]
            pub const ENABLED: u8 = 0x01;
        }
    }
}
#[doc = "Channel Priority"]
pub mod DCHPRI22 {
    #[doc = "Channel n Arbitration Priority"]
    pub mod CHPRI {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Channel n Current Group Priority"]
    pub mod GRPPRI {
        pub const offset: u8 = 4;
        pub const mask: u8 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Disable Preempt Ability. This field resets to 0."]
    pub mod DPA {
        pub const offset: u8 = 6;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Channel n can suspend a lower priority channel"]
            pub const ENABLED: u8 = 0;
            #[doc = "Channel n cannot suspend any channel, regardless of channel priority"]
            pub const DISABLED: u8 = 0x01;
        }
    }
    #[doc = "Enable Channel Preemption. This field resets to 0."]
    pub mod ECP {
        pub const offset: u8 = 7;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Channel n cannot be suspended by a higher priority channel's service request"]
            pub const DISABLED: u8 = 0;
            #[doc = "Channel n can be temporarily suspended by the service request of a higher priority channel"]
            pub const ENABLED: u8 = 0x01;
        }
    }
}
#[doc = "Channel Priority"]
pub mod DCHPRI21 {
    #[doc = "Channel n Arbitration Priority"]
    pub mod CHPRI {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Channel n Current Group Priority"]
    pub mod GRPPRI {
        pub const offset: u8 = 4;
        pub const mask: u8 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Disable Preempt Ability. This field resets to 0."]
    pub mod DPA {
        pub const offset: u8 = 6;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Channel n can suspend a lower priority channel"]
            pub const ENABLED: u8 = 0;
            #[doc = "Channel n cannot suspend any channel, regardless of channel priority"]
            pub const DISABLED: u8 = 0x01;
        }
    }
    #[doc = "Enable Channel Preemption. This field resets to 0."]
    pub mod ECP {
        pub const offset: u8 = 7;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Channel n cannot be suspended by a higher priority channel's service request"]
            pub const DISABLED: u8 = 0;
            #[doc = "Channel n can be temporarily suspended by the service request of a higher priority channel"]
            pub const ENABLED: u8 = 0x01;
        }
    }
}
#[doc = "Channel Priority"]
pub mod DCHPRI20 {
    #[doc = "Channel n Arbitration Priority"]
    pub mod CHPRI {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Channel n Current Group Priority"]
    pub mod GRPPRI {
        pub const offset: u8 = 4;
        pub const mask: u8 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Disable Preempt Ability. This field resets to 0."]
    pub mod DPA {
        pub const offset: u8 = 6;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Channel n can suspend a lower priority channel"]
            pub const ENABLED: u8 = 0;
            #[doc = "Channel n cannot suspend any channel, regardless of channel priority"]
            pub const DISABLED: u8 = 0x01;
        }
    }
    #[doc = "Enable Channel Preemption. This field resets to 0."]
    pub mod ECP {
        pub const offset: u8 = 7;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Channel n cannot be suspended by a higher priority channel's service request"]
            pub const DISABLED: u8 = 0;
            #[doc = "Channel n can be temporarily suspended by the service request of a higher priority channel"]
            pub const ENABLED: u8 = 0x01;
        }
    }
}
#[doc = "Channel Priority"]
pub mod DCHPRI27 {
    #[doc = "Channel n Arbitration Priority"]
    pub mod CHPRI {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Channel n Current Group Priority"]
    pub mod GRPPRI {
        pub const offset: u8 = 4;
        pub const mask: u8 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Disable Preempt Ability. This field resets to 0."]
    pub mod DPA {
        pub const offset: u8 = 6;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Channel n can suspend a lower priority channel"]
            pub const ENABLED: u8 = 0;
            #[doc = "Channel n cannot suspend any channel, regardless of channel priority"]
            pub const DISABLED: u8 = 0x01;
        }
    }
    #[doc = "Enable Channel Preemption. This field resets to 0."]
    pub mod ECP {
        pub const offset: u8 = 7;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Channel n cannot be suspended by a higher priority channel's service request"]
            pub const DISABLED: u8 = 0;
            #[doc = "Channel n can be temporarily suspended by the service request of a higher priority channel"]
            pub const ENABLED: u8 = 0x01;
        }
    }
}
#[doc = "Channel Priority"]
pub mod DCHPRI26 {
    #[doc = "Channel n Arbitration Priority"]
    pub mod CHPRI {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Channel n Current Group Priority"]
    pub mod GRPPRI {
        pub const offset: u8 = 4;
        pub const mask: u8 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Disable Preempt Ability. This field resets to 0."]
    pub mod DPA {
        pub const offset: u8 = 6;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Channel n can suspend a lower priority channel"]
            pub const ENABLED: u8 = 0;
            #[doc = "Channel n cannot suspend any channel, regardless of channel priority"]
            pub const DISABLED: u8 = 0x01;
        }
    }
    #[doc = "Enable Channel Preemption. This field resets to 0."]
    pub mod ECP {
        pub const offset: u8 = 7;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Channel n cannot be suspended by a higher priority channel's service request"]
            pub const DISABLED: u8 = 0;
            #[doc = "Channel n can be temporarily suspended by the service request of a higher priority channel"]
            pub const ENABLED: u8 = 0x01;
        }
    }
}
#[doc = "Channel Priority"]
pub mod DCHPRI25 {
    #[doc = "Channel n Arbitration Priority"]
    pub mod CHPRI {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Channel n Current Group Priority"]
    pub mod GRPPRI {
        pub const offset: u8 = 4;
        pub const mask: u8 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Disable Preempt Ability. This field resets to 0."]
    pub mod DPA {
        pub const offset: u8 = 6;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Channel n can suspend a lower priority channel"]
            pub const ENABLED: u8 = 0;
            #[doc = "Channel n cannot suspend any channel, regardless of channel priority"]
            pub const DISABLED: u8 = 0x01;
        }
    }
    #[doc = "Enable Channel Preemption. This field resets to 0."]
    pub mod ECP {
        pub const offset: u8 = 7;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Channel n cannot be suspended by a higher priority channel's service request"]
            pub const DISABLED: u8 = 0;
            #[doc = "Channel n can be temporarily suspended by the service request of a higher priority channel"]
            pub const ENABLED: u8 = 0x01;
        }
    }
}
#[doc = "Channel Priority"]
pub mod DCHPRI24 {
    #[doc = "Channel n Arbitration Priority"]
    pub mod CHPRI {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Channel n Current Group Priority"]
    pub mod GRPPRI {
        pub const offset: u8 = 4;
        pub const mask: u8 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Disable Preempt Ability. This field resets to 0."]
    pub mod DPA {
        pub const offset: u8 = 6;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Channel n can suspend a lower priority channel"]
            pub const ENABLED: u8 = 0;
            #[doc = "Channel n cannot suspend any channel, regardless of channel priority"]
            pub const DISABLED: u8 = 0x01;
        }
    }
    #[doc = "Enable Channel Preemption. This field resets to 0."]
    pub mod ECP {
        pub const offset: u8 = 7;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Channel n cannot be suspended by a higher priority channel's service request"]
            pub const DISABLED: u8 = 0;
            #[doc = "Channel n can be temporarily suspended by the service request of a higher priority channel"]
            pub const ENABLED: u8 = 0x01;
        }
    }
}
#[doc = "Channel Priority"]
pub mod DCHPRI31 {
    #[doc = "Channel n Arbitration Priority"]
    pub mod CHPRI {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Channel n Current Group Priority"]
    pub mod GRPPRI {
        pub const offset: u8 = 4;
        pub const mask: u8 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Disable Preempt Ability. This field resets to 0."]
    pub mod DPA {
        pub const offset: u8 = 6;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Channel n can suspend a lower priority channel"]
            pub const ENABLED: u8 = 0;
            #[doc = "Channel n cannot suspend any channel, regardless of channel priority"]
            pub const DISABLED: u8 = 0x01;
        }
    }
    #[doc = "Enable Channel Preemption. This field resets to 0."]
    pub mod ECP {
        pub const offset: u8 = 7;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Channel n cannot be suspended by a higher priority channel's service request"]
            pub const DISABLED: u8 = 0;
            #[doc = "Channel n can be temporarily suspended by the service request of a higher priority channel"]
            pub const ENABLED: u8 = 0x01;
        }
    }
}
#[doc = "Channel Priority"]
pub mod DCHPRI30 {
    #[doc = "Channel n Arbitration Priority"]
    pub mod CHPRI {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Channel n Current Group Priority"]
    pub mod GRPPRI {
        pub const offset: u8 = 4;
        pub const mask: u8 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Disable Preempt Ability. This field resets to 0."]
    pub mod DPA {
        pub const offset: u8 = 6;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Channel n can suspend a lower priority channel"]
            pub const ENABLED: u8 = 0;
            #[doc = "Channel n cannot suspend any channel, regardless of channel priority"]
            pub const DISABLED: u8 = 0x01;
        }
    }
    #[doc = "Enable Channel Preemption. This field resets to 0."]
    pub mod ECP {
        pub const offset: u8 = 7;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Channel n cannot be suspended by a higher priority channel's service request"]
            pub const DISABLED: u8 = 0;
            #[doc = "Channel n can be temporarily suspended by the service request of a higher priority channel"]
            pub const ENABLED: u8 = 0x01;
        }
    }
}
#[doc = "Channel Priority"]
pub mod DCHPRI29 {
    #[doc = "Channel n Arbitration Priority"]
    pub mod CHPRI {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Channel n Current Group Priority"]
    pub mod GRPPRI {
        pub const offset: u8 = 4;
        pub const mask: u8 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Disable Preempt Ability. This field resets to 0."]
    pub mod DPA {
        pub const offset: u8 = 6;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Channel n can suspend a lower priority channel"]
            pub const ENABLED: u8 = 0;
            #[doc = "Channel n cannot suspend any channel, regardless of channel priority"]
            pub const DISABLED: u8 = 0x01;
        }
    }
    #[doc = "Enable Channel Preemption. This field resets to 0."]
    pub mod ECP {
        pub const offset: u8 = 7;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Channel n cannot be suspended by a higher priority channel's service request"]
            pub const DISABLED: u8 = 0;
            #[doc = "Channel n can be temporarily suspended by the service request of a higher priority channel"]
            pub const ENABLED: u8 = 0x01;
        }
    }
}
#[doc = "Channel Priority"]
pub mod DCHPRI28 {
    #[doc = "Channel n Arbitration Priority"]
    pub mod CHPRI {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Channel n Current Group Priority"]
    pub mod GRPPRI {
        pub const offset: u8 = 4;
        pub const mask: u8 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Disable Preempt Ability. This field resets to 0."]
    pub mod DPA {
        pub const offset: u8 = 6;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Channel n can suspend a lower priority channel"]
            pub const ENABLED: u8 = 0;
            #[doc = "Channel n cannot suspend any channel, regardless of channel priority"]
            pub const DISABLED: u8 = 0x01;
        }
    }
    #[doc = "Enable Channel Preemption. This field resets to 0."]
    pub mod ECP {
        pub const offset: u8 = 7;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Channel n cannot be suspended by a higher priority channel's service request"]
            pub const DISABLED: u8 = 0;
            #[doc = "Channel n can be temporarily suspended by the service request of a higher priority channel"]
            pub const ENABLED: u8 = 0x01;
        }
    }
}
pub mod tcd {
    #[doc = "Cluster TCD%s, containing TCD*_SADDR, TCD*_SOFF, TCD*_ATTR, TCD*_NBYTES_MLNO, TCD*_NBYTES_MLOFFNO, TCD*_NBYTES_MLOFFYES, TCD*_SLAST, TCD*_DADDR, TCD*_DOFF, TCD*_CITER_ELINKNO, TCD*_CITER_ELINKYES, TCD*_DLASTSGA, TCD*_CSR, TCD*_BITER_ELINKNO, TCD*_BITER_ELINKYES"]
    #[repr(C)]
    pub struct RegisterBlock {
        #[doc = "TCD Source Address"]
        pub TCD_SADDR: crate::RWRegister<u32>,
        #[doc = "TCD Signed Source Address Offset"]
        pub TCD_SOFF: crate::RWRegister<u16>,
        #[doc = "TCD Transfer Attributes"]
        pub TCD_ATTR: crate::RWRegister<u16>,
        #[doc = "TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
        pub TCD_NBYTES_MLNO: crate::RWRegister<u32>,
        #[doc = "TCD Last Source Address Adjustment"]
        pub TCD_SLAST: crate::RWRegister<u32>,
        #[doc = "TCD Destination Address"]
        pub TCD_DADDR: crate::RWRegister<u32>,
        #[doc = "TCD Signed Destination Address Offset"]
        pub TCD_DOFF: crate::RWRegister<u16>,
        #[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
        pub TCD_CITER_ELINKNO: crate::RWRegister<u16>,
        #[doc = "TCD Last Destination Address Adjustment/Scatter Gather Address"]
        pub TCD_DLASTSGA: crate::RWRegister<u32>,
        #[doc = "TCD Control and Status"]
        pub TCD_CSR: crate::RWRegister<u16>,
        #[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
        pub TCD_BITER_ELINKNO: crate::RWRegister<u16>,
    }
    #[doc = "TCD Source Address"]
    pub mod TCD_SADDR {
        #[doc = "Source Address"]
        pub mod SADDR {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0xffff_ffff << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
    }
    #[doc = "TCD Signed Source Address Offset"]
    pub mod TCD_SOFF {
        #[doc = "Source address signed offset"]
        pub mod SOFF {
            pub const offset: u16 = 0;
            pub const mask: u16 = 0xffff << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
    }
    #[doc = "TCD Transfer Attributes"]
    pub mod TCD_ATTR {
        #[doc = "Destination data transfer size"]
        pub mod DSIZE {
            pub const offset: u16 = 0;
            pub const mask: u16 = 0x07 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Destination Address Modulo"]
        pub mod DMOD {
            pub const offset: u16 = 3;
            pub const mask: u16 = 0x1f << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Source data transfer size"]
        pub mod SSIZE {
            pub const offset: u16 = 8;
            pub const mask: u16 = 0x07 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "8-bit"]
                pub const EIGHT: u16 = 0;
                #[doc = "16-bit"]
                pub const SIXTEEN_BIT: u16 = 0x01;
                #[doc = "32-bit"]
                pub const THIRTYTWO_BIT: u16 = 0x02;
                #[doc = "64-bit"]
                pub const SIXTYFOUR: u16 = 0x03;
                #[doc = "32-byte burst (4 beats of 64 bits)"]
                pub const THIRTYTWO_BYTE: u16 = 0x05;
            }
        }
        #[doc = "Source Address Modulo"]
        pub mod SMOD {
            pub const offset: u16 = 11;
            pub const mask: u16 = 0x1f << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Disabled"]
                pub const DISABLED: u16 = 0;
                #[doc = "ENABLED"]
                pub const ENABLED: u16 = 0x01;
            }
        }
    }
    #[doc = "TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    pub mod TCD_NBYTES_MLNO {
        #[doc = "Minor Byte Transfer Count"]
        pub mod NBYTES {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0xffff_ffff << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
    }
    #[doc = "TCD Last Source Address Adjustment"]
    pub mod TCD_SLAST {
        #[doc = "Last Source Address Adjustment"]
        pub mod SLAST {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0xffff_ffff << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
    }
    #[doc = "TCD Destination Address"]
    pub mod TCD_DADDR {
        #[doc = "Destination Address"]
        pub mod DADDR {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0xffff_ffff << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
    }
    #[doc = "TCD Signed Destination Address Offset"]
    pub mod TCD_DOFF {
        #[doc = "Destination Address Signed Offset"]
        pub mod DOFF {
            pub const offset: u16 = 0;
            pub const mask: u16 = 0xffff << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
    }
    #[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    pub mod TCD_CITER_ELINKNO {
        #[doc = "Current Major Iteration Count"]
        pub mod CITER {
            pub const offset: u16 = 0;
            pub const mask: u16 = 0x7fff << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Enable channel-to-channel linking on minor-loop complete"]
        pub mod ELINK {
            pub const offset: u16 = 15;
            pub const mask: u16 = 0x01 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Channel-to-channel linking is disabled"]
                pub const DISABLED: u16 = 0;
                #[doc = "Channel-to-channel linking is enabled"]
                pub const ENABLED: u16 = 0x01;
            }
        }
    }
    #[doc = "TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub mod TCD_DLASTSGA {
        #[doc = "Destination last address adjustment, or next memory address TCD for channel (scatter/gather)"]
        pub mod DLASTSGA {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0xffff_ffff << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
    }
    #[doc = "TCD Control and Status"]
    pub mod TCD_CSR {
        #[doc = "Channel Start"]
        pub mod START {
            pub const offset: u16 = 0;
            pub const mask: u16 = 0x01 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Channel is not explicitly started"]
                pub const NO_START: u16 = 0;
                #[doc = "Channel is explicitly started via a software initiated service request"]
                pub const START: u16 = 0x01;
            }
        }
        #[doc = "Enable an interrupt when major iteration count completes."]
        pub mod INTMAJOR {
            pub const offset: u16 = 1;
            pub const mask: u16 = 0x01 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "End of major loop interrupt is disabled"]
                pub const DISABLED: u16 = 0;
                #[doc = "End of major loop interrupt is enabled"]
                pub const ENABLED: u16 = 0x01;
            }
        }
        #[doc = "Enable an interrupt when major counter is half complete."]
        pub mod INTHALF {
            pub const offset: u16 = 2;
            pub const mask: u16 = 0x01 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Half-point interrupt is disabled"]
                pub const DISABLED: u16 = 0;
                #[doc = "Half-point interrupt is enabled"]
                pub const ENABLED: u16 = 0x01;
            }
        }
        #[doc = "Disable Request"]
        pub mod DREQ {
            pub const offset: u16 = 3;
            pub const mask: u16 = 0x01 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "The channel's ERQ field is not affected"]
                pub const NO_CLEAR: u16 = 0;
                #[doc = "The channel's ERQ field value changes to 0 when the major loop is complete"]
                pub const CLEAR: u16 = 0x01;
            }
        }
        #[doc = "Enable Scatter/Gather Processing"]
        pub mod ESG {
            pub const offset: u16 = 4;
            pub const mask: u16 = 0x01 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "The current channel's TCD is normal format"]
                pub const NORMAL: u16 = 0;
                #[doc = "The current channel's TCD specifies a scatter gather format"]
                pub const SCATTER: u16 = 0x01;
            }
        }
        #[doc = "Enable channel-to-channel linking on major loop complete"]
        pub mod MAJORELINK {
            pub const offset: u16 = 5;
            pub const mask: u16 = 0x01 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Channel-to-channel linking is disabled"]
                pub const DISABLED: u16 = 0;
                #[doc = "Channel-to-channel linking is enabled"]
                pub const ENABLED: u16 = 0x01;
            }
        }
        #[doc = "Channel Active"]
        pub mod ACTIVE {
            pub const offset: u16 = 6;
            pub const mask: u16 = 0x01 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Channel Done"]
        pub mod DONE {
            pub const offset: u16 = 7;
            pub const mask: u16 = 0x01 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Major Loop Link Channel Number"]
        pub mod MAJORLINKCH {
            pub const offset: u16 = 8;
            pub const mask: u16 = 0x1f << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Bandwidth Control"]
        pub mod BWC {
            pub const offset: u16 = 14;
            pub const mask: u16 = 0x03 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "No eDMA engine stalls"]
                pub const DISABLED: u16 = 0;
                #[doc = "eDMA engine stalls for 4 cycles after each R/W"]
                pub const STALL4: u16 = 0x02;
                #[doc = "eDMA engine stalls for 8 cycles after each R/W"]
                pub const STALL8: u16 = 0x03;
            }
        }
    }
    #[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    pub mod TCD_BITER_ELINKNO {
        #[doc = "Starting Major Iteration Count"]
        pub mod BITER {
            pub const offset: u16 = 0;
            pub const mask: u16 = 0x7fff << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Enables channel-to-channel linking on minor loop complete"]
        pub mod ELINK {
            pub const offset: u16 = 15;
            pub const mask: u16 = 0x01 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Channel-to-channel linking is disabled"]
                pub const DISABLED: u16 = 0;
                #[doc = "Channel-to-channel linking is enabled"]
                pub const ENABLED: u16 = 0x01;
            }
        }
    }
}
