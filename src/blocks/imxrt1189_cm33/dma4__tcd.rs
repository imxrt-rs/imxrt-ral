#[doc = "DMA TCD"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "Array of registers: CH_CSR, CH_ES, CH_INT, CH_PRI, CH_SBR, CH_MUX, TCD_SADDR, TCD_SOFF, TCD_ATTR, TCD_NBYTES_MLOFFNO, TCD_NBYTES_MLOFFYES, TCD_SLAST_SDA, TCD_DADDR, TCD_CITER_ELINKYES, TCD_CITER_ELINKNO, TCD_DOFF, TCD_DLAST_SGA, CH_MATTR, TCD_BITER_ELINKYES, TCD_BITER_ELINKNO, TCD_CSR"]
    pub TCD: [tcd::RegisterBlock; 64usize],
}
pub mod tcd {
    #[doc = "Array of registers: CH_CSR, CH_ES, CH_INT, CH_PRI, CH_SBR, CH_MUX, TCD_SADDR, TCD_SOFF, TCD_ATTR, TCD_NBYTES_MLOFFNO, TCD_NBYTES_MLOFFYES, TCD_SLAST_SDA, TCD_DADDR, TCD_CITER_ELINKYES, TCD_CITER_ELINKNO, TCD_DOFF, TCD_DLAST_SGA, CH_MATTR, TCD_BITER_ELINKYES, TCD_BITER_ELINKNO, TCD_CSR"]
    #[repr(C)]
    pub struct RegisterBlock {
        #[doc = "Channel Control and Status Register"]
        pub CH_CSR: crate::RWRegister<u32>,
        #[doc = "Channel Error Status Register"]
        pub CH_ES: crate::RWRegister<u32>,
        #[doc = "Channel Interrupt Status Register"]
        pub CH_INT: crate::RWRegister<u32>,
        #[doc = "Channel System Bus Register"]
        pub CH_SBR: crate::RWRegister<u32>,
        #[doc = "Channel Priority Register"]
        pub CH_PRI: crate::RWRegister<u32>,
        #[doc = "Channel Multiplexor Configuration"]
        pub CH_MUX: crate::RWRegister<u32>,
        #[doc = "Memory Attributes Register"]
        pub CH_MATTR: crate::RWRegister<u16>,
        _reserved0: [u8; 0x06],
        #[doc = "TCD Source Address Register"]
        pub TCD_SADDR: crate::RWRegister<u32>,
        #[doc = "TCD Signed Source Address Offset Register"]
        pub TCD_SOFF: crate::RWRegister<u16>,
        #[doc = "TCD Transfer Attributes Register"]
        pub TCD_ATTR: crate::RWRegister<u16>,
        #[doc = "TCD Transfer Size without Minor Loop Offsets Register"]
        pub TCD_NBYTES_MLOFFNO: crate::RWRegister<u32>,
        #[doc = "TCD Last Source Address Adjustment / Store DADDR Address Register"]
        pub TCD_SLAST_SDA: crate::RWRegister<u32>,
        #[doc = "TCD Destination Address Register"]
        pub TCD_DADDR: crate::RWRegister<u32>,
        #[doc = "TCD Signed Destination Address Offset Register"]
        pub TCD_DOFF: crate::RWRegister<u16>,
        #[doc = "TCD Current Major Loop Count (Minor Loop Channel Linking Disabled) Register"]
        pub TCD_CITER_ELINKNO: crate::RWRegister<u16>,
        #[doc = "TCD Last Destination Address Adjustment / Scatter Gather Address Register"]
        pub TCD_DLAST_SGA: crate::RWRegister<u32>,
        #[doc = "TCD Control and Status Register"]
        pub TCD_CSR: crate::RWRegister<u16>,
        #[doc = "TCD Beginning Major Loop Count (Minor Loop Channel Linking Disabled) Register"]
        pub TCD_BITER_ELINKNO: crate::RWRegister<u16>,
        _reserved1: [u8; 0x7fc0],
    }
    #[doc = "Channel Control and Status Register"]
    pub mod CH_CSR {
        #[doc = "Enable DMA Request"]
        pub mod ERQ {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0x01 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "The DMA hardware request signal for the corresponding channel is disabled."]
                pub const DISABLE: u32 = 0;
                #[doc = "The DMA hardware request signal for the corresponding channel is enabled."]
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
                #[doc = "Disable asynchronous DMA request for the channel."]
                pub const DISABLE: u32 = 0;
                #[doc = "Enable asynchronous DMA request for the channel."]
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
                #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
                pub const DISABLE: u32 = 0;
                #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
                pub const ENABLE: u32 = 0x01;
            }
        }
        #[doc = "Swap size"]
        pub mod SWAP {
            pub const offset: u32 = 12;
            pub const mask: u32 = 0x0f << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "disabled"]
                pub const DISABLE: u32 = 0;
                #[doc = "read with 8-bit swap"]
                pub const READ_SWAP8: u32 = 0x01;
                #[doc = "read with 16-bit swap"]
                pub const READ_SWAP16: u32 = 0x02;
                #[doc = "read with 32-bit swap"]
                pub const READ_SWAP32: u32 = 0x03;
                #[doc = "write with 8-bit swap"]
                pub const WRITE_SWAP8: u32 = 0x09;
                #[doc = "write with 16-bit swap"]
                pub const WRITE_SWAP16: u32 = 0x0a;
                #[doc = "write with 32-bit swap"]
                pub const WRITE_SWAP32: u32 = 0x0b;
            }
        }
        #[doc = "Sign Extension"]
        pub mod SIGNEXT {
            pub const offset: u32 = 16;
            pub const mask: u32 = 0x3f << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "disabled"]
                pub const DISABLE: u32 = 0;
                #[doc = "A non-zero value specifying the sign extend bit position"]
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
    #[doc = "Channel Error Status Register"]
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
                #[doc = "The last recorded error was a configuration error detected in the TCDn_NBYTES or TCDn_CITER fields. TCDn_NBYTES is not a multiple of TCDn_ATTR\\[SSIZE\\] and TCDn_ATTR\\[DSIZE\\], or TCDn_CITER\\[CITER\\] is equal to zero, or TCDn_CITER\\[ELINK\\] is not equal to TCDn_BITER\\[ELINK\\]"]
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
    #[doc = "Channel Interrupt Status Register"]
    pub mod CH_INT {
        #[doc = "Interrupt Request"]
        pub mod INT {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0x01 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "The interrupt request for corresponding channel is cleared"]
                pub const INACTIVE: u32 = 0;
                #[doc = "The interrupt request for corresponding channel is active"]
                pub const ACTIVE: u32 = 0x01;
            }
        }
    }
    #[doc = "Channel System Bus Register"]
    pub mod CH_SBR {
        #[doc = "Master ID"]
        pub mod MID {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0x0f << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Instruction/Data Access"]
        pub mod INSTR {
            pub const offset: u32 = 13;
            pub const mask: u32 = 0x01 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Data access for DMA transfers"]
                pub const DATA: u32 = 0;
                #[doc = "Instruction access for DMA transfers"]
                pub const INSTR: u32 = 0x01;
            }
        }
        #[doc = "Security Level"]
        pub mod SEC {
            pub const offset: u32 = 14;
            pub const mask: u32 = 0x01 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Nonsecure protection level for DMA transfers"]
                pub const NONSECURE: u32 = 0;
                #[doc = "Secure protection level for DMA transfers"]
                pub const SECURE: u32 = 0x01;
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
                pub const USER: u32 = 0;
                #[doc = "Privileged protection level for DMA transfers"]
                pub const PRIV: u32 = 0x01;
            }
        }
        #[doc = "Enable Master ID replication"]
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
    #[doc = "Channel Priority Register"]
    pub mod CH_PRI {
        #[doc = "Arbitration Priority Level"]
        pub mod APL {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0x07 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Disable Preempt Ability."]
        pub mod DPA {
            pub const offset: u32 = 30;
            pub const mask: u32 = 0x01 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "The channel can suspend a lower priority channel."]
                pub const DISABLE: u32 = 0;
                #[doc = "The channel cannot suspend any other channel, regardless of channel priority."]
                pub const ENABLE: u32 = 0x01;
            }
        }
        #[doc = "Enable Channel Preemption."]
        pub mod ECP {
            pub const offset: u32 = 31;
            pub const mask: u32 = 0x01 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "The channel cannot be suspended by a higher priority channel's service request."]
                pub const DISABLE: u32 = 0;
                #[doc = "The channel can be temporarily suspended by the service request of a higher priority channel."]
                pub const ENABLE: u32 = 0x01;
            }
        }
    }
    #[doc = "Channel Multiplexor Configuration"]
    pub mod CH_MUX {
        #[doc = "Service Request Source"]
        pub mod SRC {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0xff << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
    }
    #[doc = "Memory Attributes Register"]
    pub mod CH_MATTR {
        #[doc = "Read Cache Attributes"]
        pub mod RCACHE {
            pub const offset: u16 = 0;
            pub const mask: u16 = 0x0f << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Write Cache Attributes"]
        pub mod WCACHE {
            pub const offset: u16 = 4;
            pub const mask: u16 = 0x0f << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
    }
    #[doc = "TCD Source Address Register"]
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
    #[doc = "TCD Signed Source Address Offset Register"]
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
    #[doc = "TCD Transfer Attributes Register"]
    pub mod TCD_ATTR {
        #[doc = "Destination data transfer size"]
        pub mod DSIZE {
            pub const offset: u16 = 0;
            pub const mask: u16 = 0x07 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "8-bit"]
                pub const BIT8: u16 = 0;
                #[doc = "16-bit"]
                pub const BIT16: u16 = 0x01;
                #[doc = "32-bit"]
                pub const BIT32: u16 = 0x02;
                #[doc = "64-bit"]
                pub const BIT64: u16 = 0x03;
                #[doc = "16-byte"]
                pub const BYTE16: u16 = 0x04;
                #[doc = "32-byte"]
                pub const BYTE32: u16 = 0x05;
                #[doc = "64-byte"]
                pub const BYTE64: u16 = 0x06;
                #[doc = "128-byte"]
                pub const BYTE128: u16 = 0x07;
            }
        }
        #[doc = "Destination address modulo"]
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
                pub const BIT8: u16 = 0;
                #[doc = "16-bit"]
                pub const BIT16: u16 = 0x01;
                #[doc = "32-bit"]
                pub const BIT32: u16 = 0x02;
                #[doc = "64-bit"]
                pub const BIT64: u16 = 0x03;
                #[doc = "16-byte"]
                pub const BYTE16: u16 = 0x04;
                #[doc = "32-byte"]
                pub const BYTE32: u16 = 0x05;
                #[doc = "64-byte"]
                pub const BYTE64: u16 = 0x06;
                #[doc = "128-byte"]
                pub const BYTE128: u16 = 0x07;
            }
        }
        #[doc = "Source address modulo"]
        pub mod SMOD {
            pub const offset: u16 = 11;
            pub const mask: u16 = 0x1f << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Source address modulo feature is disabled"]
                pub const DISABLE: u16 = 0;
                #[doc = "Source address modulo feature is enabled for any non-zero value \\[1-31\\]"]
                pub const ENABLE: u16 = 0x01;
            }
        }
    }
    #[doc = "TCD Transfer Size without Minor Loop Offsets Register"]
    pub mod TCD_NBYTES_MLOFFNO {
        #[doc = "Number of Bytes to transfer per service request"]
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
                #[doc = "The minor loop offset is not applied to the DADDR"]
                pub const DISABLE: u32 = 0;
                #[doc = "The minor loop offset is applied to the DADDR"]
                pub const ENABLE: u32 = 0x01;
            }
        }
        #[doc = "Source Minor Loop Offset Enable"]
        pub mod SMLOE {
            pub const offset: u32 = 31;
            pub const mask: u32 = 0x01 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "The minor loop offset is not applied to the SADDR"]
                pub const DISABLE: u32 = 0;
                #[doc = "The minor loop offset is applied to the SADDR"]
                pub const ENABLE: u32 = 0x01;
            }
        }
    }
    #[doc = "TCD Last Source Address Adjustment / Store DADDR Address Register"]
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
    #[doc = "TCD Destination Address Register"]
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
    #[doc = "TCD Signed Destination Address Offset Register"]
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
    #[doc = "TCD Current Major Loop Count (Minor Loop Channel Linking Disabled) Register"]
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
                #[doc = "The channel-to-channel linking is disabled"]
                pub const DISABLE: u16 = 0;
                #[doc = "The channel-to-channel linking is enabled"]
                pub const ENABLE: u16 = 0x01;
            }
        }
    }
    #[doc = "TCD Last Destination Address Adjustment / Scatter Gather Address Register"]
    pub mod TCD_DLAST_SGA {
        #[doc = "Final Destination Address Adjustment / Scatter Gather Address"]
        pub mod DLAST_SGA {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0xffff_ffff << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
    }
    #[doc = "TCD Control and Status Register"]
    pub mod TCD_CSR {
        #[doc = "Channel Start"]
        pub mod START {
            pub const offset: u16 = 0;
            pub const mask: u16 = 0x01 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "The channel is not explicitly started."]
                pub const NO_START: u16 = 0;
                #[doc = "The channel is explicitly started via a software initiated service request."]
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
                #[doc = "The end-of-major loop interrupt is disabled."]
                pub const DISABLE: u16 = 0;
                #[doc = "The end-of-major loop interrupt is enabled."]
                pub const ENABLE: u16 = 0x01;
            }
        }
        #[doc = "Enable an interrupt when major counter is half complete."]
        pub mod INTHALF {
            pub const offset: u16 = 2;
            pub const mask: u16 = 0x01 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "The half-point interrupt is disabled."]
                pub const DISABLE: u16 = 0;
                #[doc = "The half-point interrupt is enabled."]
                pub const ENABLE: u16 = 0x01;
            }
        }
        #[doc = "Disable request"]
        pub mod DREQ {
            pub const offset: u16 = 3;
            pub const mask: u16 = 0x01 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "No operation"]
                pub const DISABLE: u16 = 0;
                #[doc = "Clear the ERQ bit upon major loop completion, thus disabling hardware service requests."]
                pub const ENABLE: u16 = 0x01;
            }
        }
        #[doc = "Enable Scatter/Gather processing"]
        pub mod ESG {
            pub const offset: u16 = 4;
            pub const mask: u16 = 0x01 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "The current channel's TCD is normal format."]
                pub const DISABLE: u16 = 0;
                #[doc = "The current channel's TCD specifies a scatter gather format. The DLASTSGA field provides a memory pointer to the next TCD to be loaded into this channel after the major loop completes its execution."]
                pub const ENABLE: u16 = 0x01;
            }
        }
        #[doc = "Enable channel-to-channel linking on major loop complete"]
        pub mod MAJORELINK {
            pub const offset: u16 = 5;
            pub const mask: u16 = 0x01 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "The channel-to-channel linking is disabled."]
                pub const DISABLE: u16 = 0;
                #[doc = "The channel-to-channel linking is enabled."]
                pub const ENABLE: u16 = 0x01;
            }
        }
        #[doc = "Enable end-of-packet processing"]
        pub mod EEOP {
            pub const offset: u16 = 6;
            pub const mask: u16 = 0x01 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "The end-of-packet operation is disabled."]
                pub const DISABLE: u16 = 0;
                #[doc = "The end-of-packet hardware input signal is enabled."]
                pub const ENABLE: u16 = 0x01;
            }
        }
        #[doc = "Enable store destination address"]
        pub mod ESDA {
            pub const offset: u16 = 7;
            pub const mask: u16 = 0x01 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "The store destination address to system memory operation is disabled."]
                pub const DISABLE: u16 = 0;
                #[doc = "The store destination address to system memory operation is enabled."]
                pub const ENABLE: u16 = 0x01;
            }
        }
        #[doc = "Major loop link channel number"]
        pub mod MAJORLINKCH {
            pub const offset: u16 = 8;
            pub const mask: u16 = 0x3f << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Transfer Mode Control"]
        pub mod TMC {
            pub const offset: u16 = 14;
            pub const mask: u16 = 0x03 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Read/Write"]
                pub const NORMAL: u16 = 0;
                #[doc = "Read Only"]
                pub const READ_ONLY: u16 = 0x01;
                #[doc = "Write Only"]
                pub const WRITE_ONLY: u16 = 0x02;
            }
        }
    }
    #[doc = "TCD Beginning Major Loop Count (Minor Loop Channel Linking Disabled) Register"]
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
                #[doc = "The channel-to-channel linking is disabled"]
                pub const DISABLE: u16 = 0;
                #[doc = "The channel-to-channel linking is enabled"]
                pub const ENABLE: u16 = 0x01;
            }
        }
    }
}
