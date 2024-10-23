#[doc = "DMA MP"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "Management Page Control Register"]
    pub MP_CSR: crate::RWRegister<u32>,
    #[doc = "Management Page Error Status Register"]
    pub MP_ES: crate::RORegister<u32>,
    #[doc = "Management Page Interrupt Request Status Register - Low"]
    pub MP_INT_LOW: crate::RORegister<u32>,
    #[doc = "Management Page Interrupt Request Status Register- High"]
    pub MP_INT_HIGH: crate::RORegister<u32>,
    #[doc = "Management Page Hardware Request Status Register - Low"]
    pub MP_HRS_LOW: crate::RORegister<u32>,
    #[doc = "Management Page Hardware Request Status Register - High"]
    pub MP_HRS_HIGH: crate::RORegister<u32>,
    _reserved0: [u8; 0xe8],
    #[doc = "Channel Arbitration Group Register"]
    pub CH_GRPRI: [crate::RWRegister<u32>; 64usize],
}
#[doc = "Management Page Control Register"]
pub mod MP_CSR {
    #[doc = "Enable Debug"]
    pub mod EDBG {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Debug mode is disabled."]
            pub const DISABLE: u32 = 0;
            #[doc = "Debug mode is enabled."]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Enable Round Robin Channel Arbitration"]
    pub mod ERCA {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Round robin channel arbitration is disabled."]
            pub const DISABLE: u32 = 0;
            #[doc = "Round robin channel arbitration is enabled."]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Halt After Error"]
    pub mod HAE {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Normal operation"]
            pub const DISABLE: u32 = 0;
            #[doc = "Any error causes the HALT bit to set. Subsequently, all service requests are ignored until the HALT bit is cleared."]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Halt DMA Operations"]
    pub mod HALT {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Normal operation"]
            pub const DISABLE: u32 = 0;
            #[doc = "Stall the start of any new channels. Executing channels are allowed to complete. Channel execution resumes when this bit is cleared."]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Global Channel Linking Control"]
    pub mod GCLC {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Channel linking is disabled for all channels."]
            pub const DISABLED: u32 = 0;
            #[doc = "Channel linking is available and controlled by each channel's link settings."]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Global Master ID Replication Control"]
    pub mod GMRC {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Master ID replication is disabled for all channels."]
            pub const DISABLE: u32 = 0;
            #[doc = "Master ID replication is available and is controlled by each channel's CHn_SBR\\[EMI\\] setting."]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "eDMA version"]
    pub mod VER {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Active channel ID"]
    pub mod ACTIVE_ID {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DMA Active Status"]
    pub mod ACTIVE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "eDMA is idle."]
            pub const IDLE: u32 = 0;
            #[doc = "eDMA is executing a channel."]
            pub const ACTIVE: u32 = 0x01;
        }
    }
}
#[doc = "Management Page Error Status Register"]
pub mod MP_ES {
    #[doc = "Destination Bus Error"]
    pub mod DBE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No destination bus error"]
            pub const NO_ERROR: u32 = 0;
            #[doc = "The last recorded error was a bus error on a destination write"]
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
            #[doc = "No source bus error"]
            pub const NO_ERROR: u32 = 0;
            #[doc = "The last recorded error was a bus error on a source read"]
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
            #[doc = "No scatter/gather configuration error"]
            pub const NO_ERROR: u32 = 0;
            #[doc = "The last recorded error was a configuration error detected in the TCDn_DLASTSGA field. This field is checked at the beginning of a scatter/gather operation after major loop completion if TCDn_CSR\\[ESG\\] is enabled. TCDn_DLASTSGA is not on a 32 byte boundary."]
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
            #[doc = "No NBYTES/CITER configuration error"]
            pub const NO_ERROR: u32 = 0;
            #[doc = "The last recorded error was NBYTES equal to zero or a CITER not equal to BITER error"]
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
            #[doc = "No destination offset configuration error"]
            pub const NO_ERROR: u32 = 0;
            #[doc = "The last recorded error was a configuration error detected in the TCDn_DOFF field. TCDn_DOFF is inconsistent with TCDn_ATTR\\[DSIZE\\]."]
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
            #[doc = "No destination address configuration error"]
            pub const NO_ERROR: u32 = 0;
            #[doc = "The last recorded error was a configuration error detected in the TCDn_DADDR field. TCDn_DADDR is inconsistent with TCDn_ATTR\\[DSIZE\\]."]
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
            #[doc = "No source offset configuration error"]
            pub const NO_ERROR: u32 = 0;
            #[doc = "The last recorded error was a configuration error detected in the TCDn_SOFF field. TCDn_SOFF is inconsistent with TCDn_ATTR\\[SSIZE\\]."]
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
            #[doc = "The last recorded error was a configuration error detected in the TCDn_SADDR field. TCDn_SADDR is inconsistent with TCDn_ATTR\\[SSIZE\\]."]
            pub const ERROR: u32 = 0x01;
        }
    }
    #[doc = "Transfer Canceled"]
    pub mod ECX {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No canceled transfers"]
            pub const NO_ERROR: u32 = 0;
            #[doc = "The last recorded entry was a canceled transfer by the error cancel transfer input."]
            pub const ERROR: u32 = 0x01;
        }
    }
    #[doc = "Error Channel Number or Canceled Channel Number"]
    pub mod ERRCHN {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Valid"]
    pub mod VLD {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No ERR bits are set."]
            pub const NO_ERROR: u32 = 0;
            #[doc = "At least one ERR bit is set indicating a valid error exists that has not been cleared."]
            pub const ERROR: u32 = 0x01;
        }
    }
}
#[doc = "Management Page Interrupt Request Status Register - Low"]
pub mod MP_INT_LOW {
    #[doc = "Interrupt Request Status for channels 31 - 0"]
    pub mod INT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Management Page Interrupt Request Status Register- High"]
pub mod MP_INT_HIGH {
    #[doc = "Interrupt Request Status for channels 63-32"]
    pub mod INT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Management Page Hardware Request Status Register - Low"]
pub mod MP_HRS_LOW {
    #[doc = "Hardware Request Status for channels 31 - 0"]
    pub mod HRS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "A hardware service request for the channel is not present"]
            pub const IDLE: u32 = 0;
            #[doc = "A hardware service request for channel 0 is present"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
}
#[doc = "Management Page Hardware Request Status Register - High"]
pub mod MP_HRS_HIGH {
    #[doc = "Hardware Request Status for channels 63-32"]
    pub mod HRS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "A hardware service request for the channel is not present"]
            pub const IDLE: u32 = 0;
            #[doc = "A hardware service request for channel 0 is present"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
}
#[doc = "Channel Arbitration Group Register"]
pub mod CH_GRPRI {
    #[doc = "Arbitration group per channel."]
    pub mod GRPRI {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
