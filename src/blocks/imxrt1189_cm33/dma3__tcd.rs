#[doc = "DMA TCD"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "Array of registers: CH_CSR, CH_ES, CH_INT, CH_PRI, CH_SBR, CH_MUX, TCD_SADDR, TCD_SOFF, TCD_ATTR, TCD_NBYTES_MLOFFNO, TCD_NBYTES_MLOFFYES, TCD_SLAST_SDA, TCD_DADDR, TCD_CITER_ELINKYES, TCD_CITER_ELINKNO, TCD_DOFF, TCD_DLAST_SGA, TCD_BITER_ELINKYES, TCD_BITER_ELINKNO, TCD_CSR"]
    pub TCD: [tcd::RegisterBlock; 32usize],
}
pub mod tcd {
    #[doc = "Array of registers: CH_CSR, CH_ES, CH_INT, CH_PRI, CH_SBR, CH_MUX, TCD_SADDR, TCD_SOFF, TCD_ATTR, TCD_NBYTES_MLOFFNO, TCD_NBYTES_MLOFFYES, TCD_SLAST_SDA, TCD_DADDR, TCD_CITER_ELINKYES, TCD_CITER_ELINKNO, TCD_DOFF, TCD_DLAST_SGA, TCD_BITER_ELINKYES, TCD_BITER_ELINKNO, TCD_CSR"]
    #[repr(C)]
    pub struct RegisterBlock {
        #[doc = "Channel Control and Status"]
        pub CH_CSR: crate::RWRegister<u32>,
        #[doc = "Channel Error Status"]
        pub CH_ES: crate::RWRegister<u32>,
        #[doc = "Channel Interrupt Status"]
        pub CH_INT: crate::RWRegister<u32>,
        #[doc = "Channel System Bus"]
        pub CH_SBR: crate::RWRegister<u32>,
        #[doc = "Channel Priority"]
        pub CH_PRI: crate::RWRegister<u32>,
        #[doc = "Channel Multiplexor Configuration"]
        pub CH_MUX: crate::RWRegister<u32>,
        _reserved0: [u8; 0x08],
        #[doc = "TCD Source Address"]
        pub TCD_SADDR: crate::RWRegister<u32>,
        #[doc = "TCD Signed Source Address Offset"]
        pub TCD_SOFF: crate::RWRegister<u16>,
        #[doc = "TCD Transfer Attributes"]
        pub TCD_ATTR: crate::RWRegister<u16>,
        #[doc = "TCD Transfer Size Without Minor Loop Offsets"]
        pub TCD_NBYTES_MLOFFNO: crate::RWRegister<u32>,
        #[doc = "TCD Last Source Address Adjustment / Store DADDR Address"]
        pub TCD_SLAST_SDA: crate::RWRegister<u32>,
        #[doc = "TCD Destination Address"]
        pub TCD_DADDR: crate::RWRegister<u32>,
        #[doc = "TCD Signed Destination Address Offset"]
        pub TCD_DOFF: crate::RWRegister<u16>,
        #[doc = "TCD Current Major Loop Count (Minor Loop Channel Linking Disabled)"]
        pub TCD_CITER_ELINKNO: crate::RWRegister<u16>,
        #[doc = "TCD Last Destination Address Adjustment / Scatter Gather Address"]
        pub TCD_DLAST_SGA: crate::RWRegister<u32>,
        #[doc = "TCD Control and Status"]
        pub TCD_CSR: crate::RWRegister<u16>,
        #[doc = "TCD Beginning Major Loop Count (Minor Loop Channel Linking Disabled)"]
        pub TCD_BITER_ELINKNO: crate::RWRegister<u16>,
        _reserved1: [u8; 0xffc0],
    }
    #[doc = "Channel Control and Status"]
    pub mod CH_CSR {
        #[doc = "Enable DMA Request"]
        pub mod ERQ {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0x01 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "DMA hardware request signal for corresponding channel disabled"]
                pub const DISABLE: u32 = 0;
                #[doc = "DMA hardware request signal for corresponding channel enabled"]
                pub const ENABLE: u32 = 0x01;
            }
        }
        #[doc = "Enable Asynchronous DMA Request"]
        pub mod EARQ {
            pub const offset: u32 = 1;
            pub const mask: u32 = 0x01 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Disable asynchronous DMA request for the channel"]
                pub const DISABLE: u32 = 0;
                #[doc = "Enable asynchronous DMA request for the channel"]
                pub const ENABLE: u32 = 0x01;
            }
        }
        #[doc = "Enable Error Interrupt"]
        pub mod EEI {
            pub const offset: u32 = 2;
            pub const mask: u32 = 0x01 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Error signal for corresponding channel does not generate error interrupt"]
                pub const NO_ERROR: u32 = 0;
                #[doc = "Assertion of error signal for corresponding channel generates error interrupt request"]
                pub const ERROR: u32 = 0x01;
            }
        }
        #[doc = "Enable Buffered Writes"]
        pub mod EBW {
            pub const offset: u32 = 3;
            pub const mask: u32 = 0x01 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Buffered writes on system bus disabled"]
                pub const DISABLE: u32 = 0;
                #[doc = "Buffered writes on system bus enabled"]
                pub const ENABLE: u32 = 0x01;
            }
        }
        #[doc = "Channel Done"]
        pub mod DONE {
            pub const offset: u32 = 30;
            pub const mask: u32 = 0x01 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Channel Active"]
        pub mod ACTIVE {
            pub const offset: u32 = 31;
            pub const mask: u32 = 0x01 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
    }
    #[doc = "Channel Error Status"]
    pub mod CH_ES {
        #[doc = "Destination Bus Error"]
        pub mod DBE {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0x01 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "No destination bus error"]
                pub const NO_ERROR: u32 = 0;
                #[doc = "Last recorded error was bus error on destination write"]
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
                #[doc = "Last recorded error was bus error on source read"]
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
                #[doc = "Last recorded error was a configuration error detected in the TCDn_DLAST_SGA field"]
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
                #[doc = "Last recorded error was a configuration error detected in the TCDn_NBYTES or TCDn_CITER fields"]
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
                #[doc = "Last recorded error was a configuration error detected in the TCDn_DOFF field"]
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
                #[doc = "Last recorded error was a configuration error detected in the TCDn_DADDR field"]
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
                #[doc = "Last recorded error was a configuration error detected in the TCDn_SOFF field"]
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
                #[doc = "No source address configuration error"]
                pub const NO_ERROR: u32 = 0;
                #[doc = "Last recorded error was a configuration error detected in the TCDn_SADDR field"]
                pub const ERROR: u32 = 0x01;
            }
        }
        #[doc = "Error In Channel"]
        pub mod ERR {
            pub const offset: u32 = 31;
            pub const mask: u32 = 0x01 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "An error in this channel has not occurred"]
                pub const NO_ERROR: u32 = 0;
                #[doc = "An error in this channel has occurred"]
                pub const ERROR: u32 = 0x01;
            }
        }
    }
    #[doc = "Channel Interrupt Status"]
    pub mod CH_INT {
        #[doc = "Interrupt Request"]
        pub mod INT {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0x01 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Interrupt request for corresponding channel cleared"]
                pub const INTERRUPT_CLEARED: u32 = 0;
                #[doc = "Interrupt request for corresponding channel active"]
                pub const INTERRUPT_ACTIVE: u32 = 0x01;
            }
        }
    }
    #[doc = "Channel System Bus"]
    pub mod CH_SBR {
        #[doc = "Master ID"]
        pub mod MID {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0x0f << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Security Level"]
        pub mod SEC {
            pub const offset: u32 = 14;
            pub const mask: u32 = 0x01 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Nonsecure protection level for DMA transfers"]
                pub const NONSECURE_PROTECTION: u32 = 0;
                #[doc = "Secure protection level for DMA transfers"]
                pub const SECURE_PROTECTION: u32 = 0x01;
            }
        }
        #[doc = "Privileged Access Level"]
        pub mod PAL {
            pub const offset: u32 = 15;
            pub const mask: u32 = 0x01 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "User protection level for DMA transfers"]
                pub const USER_PROTECTION: u32 = 0;
                #[doc = "Privileged protection level for DMA transfers"]
                pub const PRIVILEGED_PROTECTION: u32 = 0x01;
            }
        }
        #[doc = "Enable Master ID Replication"]
        pub mod EMI {
            pub const offset: u32 = 16;
            pub const mask: u32 = 0x01 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Master ID replication is disabled"]
                pub const DISABLE: u32 = 0;
                #[doc = "Master ID replication is enabled"]
                pub const ENABLE: u32 = 0x01;
            }
        }
        #[doc = "Attribute Output"]
        pub mod ATTR {
            pub const offset: u32 = 17;
            pub const mask: u32 = 0x3f << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
    }
    #[doc = "Channel Priority"]
    pub mod CH_PRI {
        #[doc = "Arbitration Priority Level"]
        pub mod APL {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0x07 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Disable Preempt Ability"]
        pub mod DPA {
            pub const offset: u32 = 30;
            pub const mask: u32 = 0x01 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Channel can suspend a lower-priority channel"]
                pub const SUSPEND: u32 = 0;
                #[doc = "Channel cannot suspend any other channel, regardless of channel priority"]
                pub const CANNOT_SUSPEND: u32 = 0x01;
            }
        }
        #[doc = "Enable Channel Preemption"]
        pub mod ECP {
            pub const offset: u32 = 31;
            pub const mask: u32 = 0x01 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Channel cannot be suspended by a higher-priority channel's service request"]
                pub const CANNOT_SUSPEND: u32 = 0;
                #[doc = "Channel can be temporarily suspended by a higher-priority channel's service request"]
                pub const SUSPEND: u32 = 0x01;
            }
        }
    }
    #[doc = "Channel Multiplexor Configuration"]
    pub mod CH_MUX {
        #[doc = "Service Request Source"]
        pub mod SRC {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0x3f << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
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
        #[doc = "Source Address Signed Offset"]
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
        #[doc = "Destination Data Transfer Size"]
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
        #[doc = "Source Data Transfer Size"]
        pub mod SSIZE {
            pub const offset: u16 = 8;
            pub const mask: u16 = 0x07 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "8-bit"]
                pub const EIGHT_BIT: u16 = 0;
                #[doc = "16-bit"]
                pub const SIXTEEN_BIT: u16 = 0x01;
                #[doc = "32-bit"]
                pub const THIRTYTWO_BIT: u16 = 0x02;
                #[doc = "64-bit"]
                pub const SIXTYFOUR_BIT: u16 = 0x03;
                #[doc = "16-byte"]
                pub const SIXTEEN_BYTE: u16 = 0x04;
                #[doc = "32-byte"]
                pub const THIRTYTWO_BYTE: u16 = 0x05;
                #[doc = "64-byte"]
                pub const SIXTYFOUR_BYTE: u16 = 0x06;
            }
        }
        #[doc = "Source Address Modulo"]
        pub mod SMOD {
            pub const offset: u16 = 11;
            pub const mask: u16 = 0x1f << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Source address modulo feature disabled"]
                pub const DISABLE: u16 = 0;
                #[doc = "Source address modulo feature enabled for any non-zero value \\[1-31\\]"]
                pub const ENABLE: u16 = 0x01;
            }
        }
    }
    #[doc = "TCD Transfer Size Without Minor Loop Offsets"]
    pub mod TCD_NBYTES_MLOFFNO {
        #[doc = "Number of Bytes To Transfer Per Service Request"]
        pub mod NBYTES {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0x3fff_ffff << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Destination Minor Loop Offset Enable"]
        pub mod DMLOE {
            pub const offset: u32 = 30;
            pub const mask: u32 = 0x01 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Minor loop offset not applied to DADDR"]
                pub const OFFSET_NOT_APPLIED: u32 = 0;
                #[doc = "Minor loop offset applied to DADDR"]
                pub const OFFSET_APPLIED: u32 = 0x01;
            }
        }
        #[doc = "Source Minor Loop Offset Enable"]
        pub mod SMLOE {
            pub const offset: u32 = 31;
            pub const mask: u32 = 0x01 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Minor loop offset not applied to SADDR"]
                pub const OFFSET_NOT_APPLIED: u32 = 0;
                #[doc = "Minor loop offset applied to SADDR"]
                pub const OFFSET_APPLIED: u32 = 0x01;
            }
        }
    }
    #[doc = "TCD Last Source Address Adjustment / Store DADDR Address"]
    pub mod TCD_SLAST_SDA {
        #[doc = "Last Source Address Adjustment / Store DADDR Address"]
        pub mod SLAST_SDA {
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
    #[doc = "TCD Current Major Loop Count (Minor Loop Channel Linking Disabled)"]
    pub mod TCD_CITER_ELINKNO {
        #[doc = "Current Major Iteration Count"]
        pub mod CITER {
            pub const offset: u16 = 0;
            pub const mask: u16 = 0x7fff << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Enable Link"]
        pub mod ELINK {
            pub const offset: u16 = 15;
            pub const mask: u16 = 0x01 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Channel-to-channel linking disabled"]
                pub const DISABLE: u16 = 0;
                #[doc = "Channel-to-channel linking enabled"]
                pub const ENABLE: u16 = 0x01;
            }
        }
    }
    #[doc = "TCD Last Destination Address Adjustment / Scatter Gather Address"]
    pub mod TCD_DLAST_SGA {
        #[doc = "Last Destination Address Adjustment / Scatter Gather Address"]
        pub mod DLAST_SGA {
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
                #[doc = "Channel not explicitly started"]
                pub const CHANNEL_NOT_STARTED: u16 = 0;
                #[doc = "Channel explicitly started via a software-initiated service request"]
                pub const CHANNEL_STARTED: u16 = 0x01;
            }
        }
        #[doc = "Enable Interrupt If Major count complete"]
        pub mod INTMAJOR {
            pub const offset: u16 = 1;
            pub const mask: u16 = 0x01 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "End-of-major loop interrupt disabled"]
                pub const DISABLE: u16 = 0;
                #[doc = "End-of-major loop interrupt enabled"]
                pub const ENABLE: u16 = 0x01;
            }
        }
        #[doc = "Enable Interrupt If Major Counter Half-complete"]
        pub mod INTHALF {
            pub const offset: u16 = 2;
            pub const mask: u16 = 0x01 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Halfway point interrupt disabled"]
                pub const DISABLE: u16 = 0;
                #[doc = "Halfway point interrupt enabled"]
                pub const ENABLE: u16 = 0x01;
            }
        }
        #[doc = "Disable Request"]
        pub mod DREQ {
            pub const offset: u16 = 3;
            pub const mask: u16 = 0x01 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "No operation"]
                pub const CHANNEL_NOT_AFFECTED: u16 = 0;
                #[doc = "Clear the ERQ field to 0 upon major loop completion, thus disabling hardware service requests"]
                pub const ERQ_FIELD_CLEAR: u16 = 0x01;
            }
        }
        #[doc = "Enable Scatter/Gather Processing"]
        pub mod ESG {
            pub const offset: u16 = 4;
            pub const mask: u16 = 0x01 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Current channel's TCD is normal format"]
                pub const NORMAL_FORMAT: u16 = 0;
                #[doc = "Current channel's TCD specifies scatter/gather format."]
                pub const SCATTER_GATHER_FORMAT: u16 = 0x01;
            }
        }
        #[doc = "Enable Link When Major Loop Complete"]
        pub mod MAJORELINK {
            pub const offset: u16 = 5;
            pub const mask: u16 = 0x01 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Channel-to-channel linking disabled"]
                pub const DISABLE: u16 = 0;
                #[doc = "Channel-to-channel linking enabled"]
                pub const ENABLE: u16 = 0x01;
            }
        }
        #[doc = "Enable End-Of-Packet Processing"]
        pub mod EEOP {
            pub const offset: u16 = 6;
            pub const mask: u16 = 0x01 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "End-of-packet operation disabled"]
                pub const DISABLE: u16 = 0;
                #[doc = "End-of-packet hardware input signal enabled"]
                pub const ENABLE: u16 = 0x01;
            }
        }
        #[doc = "Enable Store Destination Address"]
        pub mod ESDA {
            pub const offset: u16 = 7;
            pub const mask: u16 = 0x01 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Ability to store destination address to system memory disabled"]
                pub const DISABLE: u16 = 0;
                #[doc = "Ability to store destination address to system memory enabled"]
                pub const ENABLE: u16 = 0x01;
            }
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
                pub const NO_STALL: u16 = 0;
                #[doc = "eDMA engine stalls for 4 cycles after each R/W"]
                pub const ENGINE_STALLS_FOUR: u16 = 0x02;
                #[doc = "eDMA engine stalls for 8 cycles after each R/W"]
                pub const ENGINE_STALLS_EIGHT: u16 = 0x03;
            }
        }
    }
    #[doc = "TCD Beginning Major Loop Count (Minor Loop Channel Linking Disabled)"]
    pub mod TCD_BITER_ELINKNO {
        #[doc = "Starting Major Iteration Count"]
        pub mod BITER {
            pub const offset: u16 = 0;
            pub const mask: u16 = 0x7fff << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Enables Link"]
        pub mod ELINK {
            pub const offset: u16 = 15;
            pub const mask: u16 = 0x01 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Channel-to-channel linking disabled"]
                pub const DISABLE: u16 = 0;
                #[doc = "Channel-to-channel linking enabled"]
                pub const ENABLE: u16 = 0x01;
            }
        }
    }
}
