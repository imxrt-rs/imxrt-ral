#[doc = "DMA"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "Control Register"]
    pub CR: crate::RWRegister<u32>,
    #[doc = "Error Status Register"]
    pub ES: crate::RORegister<u32>,
    _reserved0: [u8; 0x04],
    #[doc = "Enable Request Register"]
    pub ERQ: crate::RWRegister<u32>,
    _reserved1: [u8; 0x04],
    #[doc = "Enable Error Interrupt Register"]
    pub EEI: crate::RWRegister<u32>,
    #[doc = "Clear Enable Error Interrupt Register"]
    pub CEEI: crate::RWRegister<u8>,
    #[doc = "Set Enable Error Interrupt Register"]
    pub SEEI: crate::RWRegister<u8>,
    #[doc = "Clear Enable Request Register"]
    pub CERQ: crate::RWRegister<u8>,
    #[doc = "Set Enable Request Register"]
    pub SERQ: crate::RWRegister<u8>,
    #[doc = "Clear DONE Status Bit Register"]
    pub CDNE: crate::RWRegister<u8>,
    #[doc = "Set START Bit Register"]
    pub SSRT: crate::RWRegister<u8>,
    #[doc = "Clear Error Register"]
    pub CERR: crate::RWRegister<u8>,
    #[doc = "Clear Interrupt Request Register"]
    pub CINT: crate::RWRegister<u8>,
    _reserved2: [u8; 0x04],
    #[doc = "Interrupt Request Register"]
    pub INT: crate::RWRegister<u32>,
    _reserved3: [u8; 0x04],
    #[doc = "Error Register"]
    pub ERR: crate::RWRegister<u32>,
    _reserved4: [u8; 0x04],
    #[doc = "Hardware Request Status Register"]
    pub HRS: crate::RORegister<u32>,
    _reserved5: [u8; 0x0c],
    #[doc = "Enable Asynchronous Request in Stop Register"]
    pub EARS: crate::RWRegister<u32>,
    _reserved6: [u8; 0xb8],
    #[doc = "Channel n Priority Register"]
    pub DCHPRI3: crate::RWRegister<u8>,
    #[doc = "Channel n Priority Register"]
    pub DCHPRI2: crate::RWRegister<u8>,
    #[doc = "Channel n Priority Register"]
    pub DCHPRI1: crate::RWRegister<u8>,
    #[doc = "Channel n Priority Register"]
    pub DCHPRI0: crate::RWRegister<u8>,
    #[doc = "Channel n Priority Register"]
    pub DCHPRI7: crate::RWRegister<u8>,
    #[doc = "Channel n Priority Register"]
    pub DCHPRI6: crate::RWRegister<u8>,
    #[doc = "Channel n Priority Register"]
    pub DCHPRI5: crate::RWRegister<u8>,
    #[doc = "Channel n Priority Register"]
    pub DCHPRI4: crate::RWRegister<u8>,
    #[doc = "Channel n Priority Register"]
    pub DCHPRI11: crate::RWRegister<u8>,
    #[doc = "Channel n Priority Register"]
    pub DCHPRI10: crate::RWRegister<u8>,
    #[doc = "Channel n Priority Register"]
    pub DCHPRI9: crate::RWRegister<u8>,
    #[doc = "Channel n Priority Register"]
    pub DCHPRI8: crate::RWRegister<u8>,
    #[doc = "Channel n Priority Register"]
    pub DCHPRI15: crate::RWRegister<u8>,
    #[doc = "Channel n Priority Register"]
    pub DCHPRI14: crate::RWRegister<u8>,
    #[doc = "Channel n Priority Register"]
    pub DCHPRI13: crate::RWRegister<u8>,
    #[doc = "Channel n Priority Register"]
    pub DCHPRI12: crate::RWRegister<u8>,
    #[doc = "Channel n Priority Register"]
    pub DCHPRI19: crate::RWRegister<u8>,
    #[doc = "Channel n Priority Register"]
    pub DCHPRI18: crate::RWRegister<u8>,
    #[doc = "Channel n Priority Register"]
    pub DCHPRI17: crate::RWRegister<u8>,
    #[doc = "Channel n Priority Register"]
    pub DCHPRI16: crate::RWRegister<u8>,
    #[doc = "Channel n Priority Register"]
    pub DCHPRI23: crate::RWRegister<u8>,
    #[doc = "Channel n Priority Register"]
    pub DCHPRI22: crate::RWRegister<u8>,
    #[doc = "Channel n Priority Register"]
    pub DCHPRI21: crate::RWRegister<u8>,
    #[doc = "Channel n Priority Register"]
    pub DCHPRI20: crate::RWRegister<u8>,
    #[doc = "Channel n Priority Register"]
    pub DCHPRI27: crate::RWRegister<u8>,
    #[doc = "Channel n Priority Register"]
    pub DCHPRI26: crate::RWRegister<u8>,
    #[doc = "Channel n Priority Register"]
    pub DCHPRI25: crate::RWRegister<u8>,
    #[doc = "Channel n Priority Register"]
    pub DCHPRI24: crate::RWRegister<u8>,
    #[doc = "Channel n Priority Register"]
    pub DCHPRI31: crate::RWRegister<u8>,
    #[doc = "Channel n Priority Register"]
    pub DCHPRI30: crate::RWRegister<u8>,
    #[doc = "Channel n Priority Register"]
    pub DCHPRI29: crate::RWRegister<u8>,
    #[doc = "Channel n Priority Register"]
    pub DCHPRI28: crate::RWRegister<u8>,
    _reserved7: [u8; 0x0ee0],
    #[doc = "Cluster TCD%s, containing TCD*_SADDR, TCD*_SOFF, TCD*_ATTR, TCD*_NBYTES_MLNO, TCD*_NBYTES_MLOFFNO, TCD*_NBYTES_MLOFFYES, TCD*_SLAST, TCD*_DADDR, TCD*_DOFF, TCD*_CITER_ELINKNO, TCD*_CITER_ELINKYES, TCD*_DLASTSGA, TCD*_CSR, TCD*_BITER_ELINKNO, TCD*_BITER_ELINKYES"]
    pub TCD: [tcd::RegisterBlock; 32usize],
}
#[doc = "Control Register"]
pub mod CR {
    #[doc = "Enable Debug"]
    pub mod EDBG {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "When in debug mode, the DMA continues to operate."]
            pub const EDBG_0: u32 = 0;
            #[doc = "When in debug mode, the DMA stalls the start of a new channel. Executing channels are allowed to complete. Channel execution resumes when the system exits debug mode or the EDBG bit is cleared."]
            pub const EDBG_1: u32 = 0x01;
        }
    }
    #[doc = "Enable Round Robin Channel Arbitration"]
    pub mod ERCA {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Fixed priority arbitration is used for channel selection ."]
            pub const ERCA_0: u32 = 0;
            #[doc = "Round robin arbitration is used for channel selection ."]
            pub const ERCA_1: u32 = 0x01;
        }
    }
    #[doc = "Enable Round Robin Group Arbitration"]
    pub mod ERGA {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Fixed priority arbitration is used for selection among the groups."]
            pub const ERGA_0: u32 = 0;
            #[doc = "Round robin arbitration is used for selection among the groups."]
            pub const ERGA_1: u32 = 0x01;
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
            pub const HOE_0: u32 = 0;
            #[doc = "Any error causes the HALT bit to set. Subsequently, all service requests are ignored until the HALT bit is cleared."]
            pub const HOE_1: u32 = 0x01;
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
            pub const HALT_0: u32 = 0;
            #[doc = "Stall the start of any new channels. Executing channels are allowed to complete. Channel execution resumes when this bit is cleared."]
            pub const HALT_1: u32 = 0x01;
        }
    }
    #[doc = "Continuous Link Mode"]
    pub mod CLM {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "A minor loop channel link made to itself goes through channel arbitration before being activated again."]
            pub const CLM_0: u32 = 0;
            #[doc = "A minor loop channel link made to itself does not go through channel arbitration before being activated again. Upon minor loop completion, the channel activates again if that channel has a minor loop channel link enabled and the link channel is itself. This effectively applies the minor loop offsets and restarts the next minor loop."]
            pub const CLM_1: u32 = 0x01;
        }
    }
    #[doc = "Enable Minor Loop Mapping"]
    pub mod EMLM {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled. TCDn.word2 is defined as a 32-bit NBYTES field."]
            pub const EMLM_0: u32 = 0;
            #[doc = "Enabled. TCDn.word2 is redefined to include individual enable fields, an offset field, and the NBYTES field. The individual enable fields allow the minor loop offset to be applied to the source address, the destination address, or both. The NBYTES field is reduced when either offset is enabled."]
            pub const EMLM_1: u32 = 0x01;
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
            pub const ECX_0: u32 = 0;
            #[doc = "Cancel the remaining data transfer in the same fashion as the CX bit. Stop the executing channel and force the minor loop to finish. The cancel takes effect after the last write of the current read/write sequence. The ECX bit clears itself after the cancel is honored. In addition to cancelling the transfer, ECX treats the cancel as an error condition, thus updating the Error Status register (DMAx_ES) and generating an optional error interrupt."]
            pub const ECX_1: u32 = 0x01;
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
            pub const CX_0: u32 = 0;
            #[doc = "Cancel the remaining data transfer. Stop the executing channel and force the minor loop to finish. The cancel takes effect after the last write of the current read/write sequence. The CX bit clears itself after the cancel has been honored. This cancel retires the channel normally as if the minor loop was completed."]
            pub const CX_1: u32 = 0x01;
        }
    }
    #[doc = "DMA Active Status"]
    pub mod ACTIVE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "eDMA is idle."]
            pub const ACTIVE_0: u32 = 0;
            #[doc = "eDMA is executing a channel."]
            pub const ACTIVE_1: u32 = 0x01;
        }
    }
}
#[doc = "Error Status Register"]
pub mod ES {
    #[doc = "Destination Bus Error"]
    pub mod DBE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No destination bus error"]
            pub const DBE_0: u32 = 0;
            #[doc = "The last recorded error was a bus error on a destination write"]
            pub const DBE_1: u32 = 0x01;
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
            pub const SBE_0: u32 = 0;
            #[doc = "The last recorded error was a bus error on a source read"]
            pub const SBE_1: u32 = 0x01;
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
            pub const SGE_0: u32 = 0;
            #[doc = "The last recorded error was a configuration error detected in the TCDn_DLASTSGA field. This field is checked at the beginning of a scatter/gather operation after major loop completion if TCDn_CSR\\[ESG\\] is enabled. TCDn_DLASTSGA is not on a 32 byte boundary."]
            pub const SGE_1: u32 = 0x01;
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
            pub const NCE_0: u32 = 0;
            #[doc = "The last recorded error was a configuration error detected in the TCDn_NBYTES or TCDn_CITER fields. TCDn_NBYTES is not a multiple of TCDn_ATTR\\[SSIZE\\] and TCDn_ATTR\\[DSIZE\\], or TCDn_CITER\\[CITER\\] is equal to zero, or TCDn_CITER\\[ELINK\\] is not equal to TCDn_BITER\\[ELINK\\]"]
            pub const NCE_1: u32 = 0x01;
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
            pub const DOE_0: u32 = 0;
            #[doc = "The last recorded error was a configuration error detected in the TCDn_DOFF field. TCDn_DOFF is inconsistent with TCDn_ATTR\\[DSIZE\\]."]
            pub const DOE_1: u32 = 0x01;
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
            pub const DAE_0: u32 = 0;
            #[doc = "The last recorded error was a configuration error detected in the TCDn_DADDR field. TCDn_DADDR is inconsistent with TCDn_ATTR\\[DSIZE\\]."]
            pub const DAE_1: u32 = 0x01;
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
            pub const SOE_0: u32 = 0;
            #[doc = "The last recorded error was a configuration error detected in the TCDn_SOFF field. TCDn_SOFF is inconsistent with TCDn_ATTR\\[SSIZE\\]."]
            pub const SOE_1: u32 = 0x01;
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
            pub const SAE_0: u32 = 0;
            #[doc = "The last recorded error was a configuration error detected in the TCDn_SADDR field. TCDn_SADDR is inconsistent with TCDn_ATTR\\[SSIZE\\]."]
            pub const SAE_1: u32 = 0x01;
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
            #[doc = "No channel priority error"]
            pub const CPE_0: u32 = 0;
            #[doc = "The last recorded error was a configuration error in the channel priorities . Channel priorities are not unique."]
            pub const CPE_1: u32 = 0x01;
        }
    }
    #[doc = "Group Priority Error"]
    pub mod GPE {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No group priority error"]
            pub const GPE_0: u32 = 0;
            #[doc = "The last recorded error was a configuration error among the group priorities. All group priorities are not unique."]
            pub const GPE_1: u32 = 0x01;
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
            pub const ECX_0: u32 = 0;
            #[doc = "The last recorded entry was a canceled transfer by the error cancel transfer input"]
            pub const ECX_1: u32 = 0x01;
        }
    }
    #[doc = "VLD"]
    pub mod VLD {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No ERR bits are set."]
            pub const VLD_0: u32 = 0;
            #[doc = "At least one ERR bit is set indicating a valid error exists that has not been cleared."]
            pub const VLD_1: u32 = 0x01;
        }
    }
}
#[doc = "Enable Request Register"]
pub mod ERQ {
    #[doc = "Enable DMA Request 0"]
    pub mod ERQ0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The DMA request signal for the corresponding channel is disabled"]
            pub const ERQ0_0: u32 = 0;
            #[doc = "The DMA request signal for the corresponding channel is enabled"]
            pub const ERQ0_1: u32 = 0x01;
        }
    }
    #[doc = "Enable DMA Request 1"]
    pub mod ERQ1 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The DMA request signal for the corresponding channel is disabled"]
            pub const ERQ1_0: u32 = 0;
            #[doc = "The DMA request signal for the corresponding channel is enabled"]
            pub const ERQ1_1: u32 = 0x01;
        }
    }
    #[doc = "Enable DMA Request 2"]
    pub mod ERQ2 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The DMA request signal for the corresponding channel is disabled"]
            pub const ERQ2_0: u32 = 0;
            #[doc = "The DMA request signal for the corresponding channel is enabled"]
            pub const ERQ2_1: u32 = 0x01;
        }
    }
    #[doc = "Enable DMA Request 3"]
    pub mod ERQ3 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The DMA request signal for the corresponding channel is disabled"]
            pub const ERQ3_0: u32 = 0;
            #[doc = "The DMA request signal for the corresponding channel is enabled"]
            pub const ERQ3_1: u32 = 0x01;
        }
    }
    #[doc = "Enable DMA Request 4"]
    pub mod ERQ4 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The DMA request signal for the corresponding channel is disabled"]
            pub const ERQ4_0: u32 = 0;
            #[doc = "The DMA request signal for the corresponding channel is enabled"]
            pub const ERQ4_1: u32 = 0x01;
        }
    }
    #[doc = "Enable DMA Request 5"]
    pub mod ERQ5 {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The DMA request signal for the corresponding channel is disabled"]
            pub const ERQ5_0: u32 = 0;
            #[doc = "The DMA request signal for the corresponding channel is enabled"]
            pub const ERQ5_1: u32 = 0x01;
        }
    }
    #[doc = "Enable DMA Request 6"]
    pub mod ERQ6 {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The DMA request signal for the corresponding channel is disabled"]
            pub const ERQ6_0: u32 = 0;
            #[doc = "The DMA request signal for the corresponding channel is enabled"]
            pub const ERQ6_1: u32 = 0x01;
        }
    }
    #[doc = "Enable DMA Request 7"]
    pub mod ERQ7 {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The DMA request signal for the corresponding channel is disabled"]
            pub const ERQ7_0: u32 = 0;
            #[doc = "The DMA request signal for the corresponding channel is enabled"]
            pub const ERQ7_1: u32 = 0x01;
        }
    }
    #[doc = "Enable DMA Request 8"]
    pub mod ERQ8 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The DMA request signal for the corresponding channel is disabled"]
            pub const ERQ8_0: u32 = 0;
            #[doc = "The DMA request signal for the corresponding channel is enabled"]
            pub const ERQ8_1: u32 = 0x01;
        }
    }
    #[doc = "Enable DMA Request 9"]
    pub mod ERQ9 {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The DMA request signal for the corresponding channel is disabled"]
            pub const ERQ9_0: u32 = 0;
            #[doc = "The DMA request signal for the corresponding channel is enabled"]
            pub const ERQ9_1: u32 = 0x01;
        }
    }
    #[doc = "Enable DMA Request 10"]
    pub mod ERQ10 {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The DMA request signal for the corresponding channel is disabled"]
            pub const ERQ10_0: u32 = 0;
            #[doc = "The DMA request signal for the corresponding channel is enabled"]
            pub const ERQ10_1: u32 = 0x01;
        }
    }
    #[doc = "Enable DMA Request 11"]
    pub mod ERQ11 {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The DMA request signal for the corresponding channel is disabled"]
            pub const ERQ11_0: u32 = 0;
            #[doc = "The DMA request signal for the corresponding channel is enabled"]
            pub const ERQ11_1: u32 = 0x01;
        }
    }
    #[doc = "Enable DMA Request 12"]
    pub mod ERQ12 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The DMA request signal for the corresponding channel is disabled"]
            pub const ERQ12_0: u32 = 0;
            #[doc = "The DMA request signal for the corresponding channel is enabled"]
            pub const ERQ12_1: u32 = 0x01;
        }
    }
    #[doc = "Enable DMA Request 13"]
    pub mod ERQ13 {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The DMA request signal for the corresponding channel is disabled"]
            pub const ERQ13_0: u32 = 0;
            #[doc = "The DMA request signal for the corresponding channel is enabled"]
            pub const ERQ13_1: u32 = 0x01;
        }
    }
    #[doc = "Enable DMA Request 14"]
    pub mod ERQ14 {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The DMA request signal for the corresponding channel is disabled"]
            pub const ERQ14_0: u32 = 0;
            #[doc = "The DMA request signal for the corresponding channel is enabled"]
            pub const ERQ14_1: u32 = 0x01;
        }
    }
    #[doc = "Enable DMA Request 15"]
    pub mod ERQ15 {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The DMA request signal for the corresponding channel is disabled"]
            pub const ERQ15_0: u32 = 0;
            #[doc = "The DMA request signal for the corresponding channel is enabled"]
            pub const ERQ15_1: u32 = 0x01;
        }
    }
    #[doc = "Enable DMA Request 16"]
    pub mod ERQ16 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The DMA request signal for the corresponding channel is disabled"]
            pub const ERQ16_0: u32 = 0;
            #[doc = "The DMA request signal for the corresponding channel is enabled"]
            pub const ERQ16_1: u32 = 0x01;
        }
    }
    #[doc = "Enable DMA Request 17"]
    pub mod ERQ17 {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The DMA request signal for the corresponding channel is disabled"]
            pub const ERQ17_0: u32 = 0;
            #[doc = "The DMA request signal for the corresponding channel is enabled"]
            pub const ERQ17_1: u32 = 0x01;
        }
    }
    #[doc = "Enable DMA Request 18"]
    pub mod ERQ18 {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The DMA request signal for the corresponding channel is disabled"]
            pub const ERQ18_0: u32 = 0;
            #[doc = "The DMA request signal for the corresponding channel is enabled"]
            pub const ERQ18_1: u32 = 0x01;
        }
    }
    #[doc = "Enable DMA Request 19"]
    pub mod ERQ19 {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The DMA request signal for the corresponding channel is disabled"]
            pub const ERQ19_0: u32 = 0;
            #[doc = "The DMA request signal for the corresponding channel is enabled"]
            pub const ERQ19_1: u32 = 0x01;
        }
    }
    #[doc = "Enable DMA Request 20"]
    pub mod ERQ20 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The DMA request signal for the corresponding channel is disabled"]
            pub const ERQ20_0: u32 = 0;
            #[doc = "The DMA request signal for the corresponding channel is enabled"]
            pub const ERQ20_1: u32 = 0x01;
        }
    }
    #[doc = "Enable DMA Request 21"]
    pub mod ERQ21 {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The DMA request signal for the corresponding channel is disabled"]
            pub const ERQ21_0: u32 = 0;
            #[doc = "The DMA request signal for the corresponding channel is enabled"]
            pub const ERQ21_1: u32 = 0x01;
        }
    }
    #[doc = "Enable DMA Request 22"]
    pub mod ERQ22 {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The DMA request signal for the corresponding channel is disabled"]
            pub const ERQ22_0: u32 = 0;
            #[doc = "The DMA request signal for the corresponding channel is enabled"]
            pub const ERQ22_1: u32 = 0x01;
        }
    }
    #[doc = "Enable DMA Request 23"]
    pub mod ERQ23 {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The DMA request signal for the corresponding channel is disabled"]
            pub const ERQ23_0: u32 = 0;
            #[doc = "The DMA request signal for the corresponding channel is enabled"]
            pub const ERQ23_1: u32 = 0x01;
        }
    }
    #[doc = "Enable DMA Request 24"]
    pub mod ERQ24 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The DMA request signal for the corresponding channel is disabled"]
            pub const ERQ24_0: u32 = 0;
            #[doc = "The DMA request signal for the corresponding channel is enabled"]
            pub const ERQ24_1: u32 = 0x01;
        }
    }
    #[doc = "Enable DMA Request 25"]
    pub mod ERQ25 {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The DMA request signal for the corresponding channel is disabled"]
            pub const ERQ25_0: u32 = 0;
            #[doc = "The DMA request signal for the corresponding channel is enabled"]
            pub const ERQ25_1: u32 = 0x01;
        }
    }
    #[doc = "Enable DMA Request 26"]
    pub mod ERQ26 {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The DMA request signal for the corresponding channel is disabled"]
            pub const ERQ26_0: u32 = 0;
            #[doc = "The DMA request signal for the corresponding channel is enabled"]
            pub const ERQ26_1: u32 = 0x01;
        }
    }
    #[doc = "Enable DMA Request 27"]
    pub mod ERQ27 {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The DMA request signal for the corresponding channel is disabled"]
            pub const ERQ27_0: u32 = 0;
            #[doc = "The DMA request signal for the corresponding channel is enabled"]
            pub const ERQ27_1: u32 = 0x01;
        }
    }
    #[doc = "Enable DMA Request 28"]
    pub mod ERQ28 {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The DMA request signal for the corresponding channel is disabled"]
            pub const ERQ28_0: u32 = 0;
            #[doc = "The DMA request signal for the corresponding channel is enabled"]
            pub const ERQ28_1: u32 = 0x01;
        }
    }
    #[doc = "Enable DMA Request 29"]
    pub mod ERQ29 {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The DMA request signal for the corresponding channel is disabled"]
            pub const ERQ29_0: u32 = 0;
            #[doc = "The DMA request signal for the corresponding channel is enabled"]
            pub const ERQ29_1: u32 = 0x01;
        }
    }
    #[doc = "Enable DMA Request 30"]
    pub mod ERQ30 {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The DMA request signal for the corresponding channel is disabled"]
            pub const ERQ30_0: u32 = 0;
            #[doc = "The DMA request signal for the corresponding channel is enabled"]
            pub const ERQ30_1: u32 = 0x01;
        }
    }
    #[doc = "Enable DMA Request 31"]
    pub mod ERQ31 {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The DMA request signal for the corresponding channel is disabled"]
            pub const ERQ31_0: u32 = 0;
            #[doc = "The DMA request signal for the corresponding channel is enabled"]
            pub const ERQ31_1: u32 = 0x01;
        }
    }
}
#[doc = "Enable Error Interrupt Register"]
pub mod EEI {
    #[doc = "Enable Error Interrupt 0"]
    pub mod EEI0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
            pub const EEI0_0: u32 = 0;
            #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
            pub const EEI0_1: u32 = 0x01;
        }
    }
    #[doc = "Enable Error Interrupt 1"]
    pub mod EEI1 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
            pub const EEI1_0: u32 = 0;
            #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
            pub const EEI1_1: u32 = 0x01;
        }
    }
    #[doc = "Enable Error Interrupt 2"]
    pub mod EEI2 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
            pub const EEI2_0: u32 = 0;
            #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
            pub const EEI2_1: u32 = 0x01;
        }
    }
    #[doc = "Enable Error Interrupt 3"]
    pub mod EEI3 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
            pub const EEI3_0: u32 = 0;
            #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
            pub const EEI3_1: u32 = 0x01;
        }
    }
    #[doc = "Enable Error Interrupt 4"]
    pub mod EEI4 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
            pub const EEI4_0: u32 = 0;
            #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
            pub const EEI4_1: u32 = 0x01;
        }
    }
    #[doc = "Enable Error Interrupt 5"]
    pub mod EEI5 {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
            pub const EEI5_0: u32 = 0;
            #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
            pub const EEI5_1: u32 = 0x01;
        }
    }
    #[doc = "Enable Error Interrupt 6"]
    pub mod EEI6 {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
            pub const EEI6_0: u32 = 0;
            #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
            pub const EEI6_1: u32 = 0x01;
        }
    }
    #[doc = "Enable Error Interrupt 7"]
    pub mod EEI7 {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
            pub const EEI7_0: u32 = 0;
            #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
            pub const EEI7_1: u32 = 0x01;
        }
    }
    #[doc = "Enable Error Interrupt 8"]
    pub mod EEI8 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
            pub const EEI8_0: u32 = 0;
            #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
            pub const EEI8_1: u32 = 0x01;
        }
    }
    #[doc = "Enable Error Interrupt 9"]
    pub mod EEI9 {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
            pub const EEI9_0: u32 = 0;
            #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
            pub const EEI9_1: u32 = 0x01;
        }
    }
    #[doc = "Enable Error Interrupt 10"]
    pub mod EEI10 {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
            pub const EEI10_0: u32 = 0;
            #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
            pub const EEI10_1: u32 = 0x01;
        }
    }
    #[doc = "Enable Error Interrupt 11"]
    pub mod EEI11 {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
            pub const EEI11_0: u32 = 0;
            #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
            pub const EEI11_1: u32 = 0x01;
        }
    }
    #[doc = "Enable Error Interrupt 12"]
    pub mod EEI12 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
            pub const EEI12_0: u32 = 0;
            #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
            pub const EEI12_1: u32 = 0x01;
        }
    }
    #[doc = "Enable Error Interrupt 13"]
    pub mod EEI13 {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
            pub const EEI13_0: u32 = 0;
            #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
            pub const EEI13_1: u32 = 0x01;
        }
    }
    #[doc = "Enable Error Interrupt 14"]
    pub mod EEI14 {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
            pub const EEI14_0: u32 = 0;
            #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
            pub const EEI14_1: u32 = 0x01;
        }
    }
    #[doc = "Enable Error Interrupt 15"]
    pub mod EEI15 {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
            pub const EEI15_0: u32 = 0;
            #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
            pub const EEI15_1: u32 = 0x01;
        }
    }
    #[doc = "Enable Error Interrupt 16"]
    pub mod EEI16 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
            pub const EEI16_0: u32 = 0;
            #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
            pub const EEI16_1: u32 = 0x01;
        }
    }
    #[doc = "Enable Error Interrupt 17"]
    pub mod EEI17 {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
            pub const EEI17_0: u32 = 0;
            #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
            pub const EEI17_1: u32 = 0x01;
        }
    }
    #[doc = "Enable Error Interrupt 18"]
    pub mod EEI18 {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
            pub const EEI18_0: u32 = 0;
            #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
            pub const EEI18_1: u32 = 0x01;
        }
    }
    #[doc = "Enable Error Interrupt 19"]
    pub mod EEI19 {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
            pub const EEI19_0: u32 = 0;
            #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
            pub const EEI19_1: u32 = 0x01;
        }
    }
    #[doc = "Enable Error Interrupt 20"]
    pub mod EEI20 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
            pub const EEI20_0: u32 = 0;
            #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
            pub const EEI20_1: u32 = 0x01;
        }
    }
    #[doc = "Enable Error Interrupt 21"]
    pub mod EEI21 {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
            pub const EEI21_0: u32 = 0;
            #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
            pub const EEI21_1: u32 = 0x01;
        }
    }
    #[doc = "Enable Error Interrupt 22"]
    pub mod EEI22 {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
            pub const EEI22_0: u32 = 0;
            #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
            pub const EEI22_1: u32 = 0x01;
        }
    }
    #[doc = "Enable Error Interrupt 23"]
    pub mod EEI23 {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
            pub const EEI23_0: u32 = 0;
            #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
            pub const EEI23_1: u32 = 0x01;
        }
    }
    #[doc = "Enable Error Interrupt 24"]
    pub mod EEI24 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
            pub const EEI24_0: u32 = 0;
            #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
            pub const EEI24_1: u32 = 0x01;
        }
    }
    #[doc = "Enable Error Interrupt 25"]
    pub mod EEI25 {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
            pub const EEI25_0: u32 = 0;
            #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
            pub const EEI25_1: u32 = 0x01;
        }
    }
    #[doc = "Enable Error Interrupt 26"]
    pub mod EEI26 {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
            pub const EEI26_0: u32 = 0;
            #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
            pub const EEI26_1: u32 = 0x01;
        }
    }
    #[doc = "Enable Error Interrupt 27"]
    pub mod EEI27 {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
            pub const EEI27_0: u32 = 0;
            #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
            pub const EEI27_1: u32 = 0x01;
        }
    }
    #[doc = "Enable Error Interrupt 28"]
    pub mod EEI28 {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
            pub const EEI28_0: u32 = 0;
            #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
            pub const EEI28_1: u32 = 0x01;
        }
    }
    #[doc = "Enable Error Interrupt 29"]
    pub mod EEI29 {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
            pub const EEI29_0: u32 = 0;
            #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
            pub const EEI29_1: u32 = 0x01;
        }
    }
    #[doc = "Enable Error Interrupt 30"]
    pub mod EEI30 {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
            pub const EEI30_0: u32 = 0;
            #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
            pub const EEI30_1: u32 = 0x01;
        }
    }
    #[doc = "Enable Error Interrupt 31"]
    pub mod EEI31 {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
            pub const EEI31_0: u32 = 0;
            #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
            pub const EEI31_1: u32 = 0x01;
        }
    }
}
#[doc = "Clear Enable Error Interrupt Register"]
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
            #[doc = "Clear only the EEI bit specified in the CEEI field"]
            pub const CAEE_0: u8 = 0;
            #[doc = "Clear all bits in EEI"]
            pub const CAEE_1: u8 = 0x01;
        }
    }
    #[doc = "No Op enable"]
    pub mod NOP {
        pub const offset: u8 = 7;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Normal operation"]
            pub const NOP_0: u8 = 0;
            #[doc = "No operation, ignore the other bits in this register"]
            pub const NOP_1: u8 = 0x01;
        }
    }
}
#[doc = "Set Enable Error Interrupt Register"]
pub mod SEEI {
    #[doc = "Set Enable Error Interrupt"]
    pub mod SEEI {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Sets All Enable Error Interrupts"]
    pub mod SAEE {
        pub const offset: u8 = 6;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Set only the EEI bit specified in the SEEI field."]
            pub const SAEE_0: u8 = 0;
            #[doc = "Sets all bits in EEI"]
            pub const SAEE_1: u8 = 0x01;
        }
    }
    #[doc = "No Op enable"]
    pub mod NOP {
        pub const offset: u8 = 7;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Normal operation"]
            pub const NOP_0: u8 = 0;
            #[doc = "No operation, ignore the other bits in this register"]
            pub const NOP_1: u8 = 0x01;
        }
    }
}
#[doc = "Clear Enable Request Register"]
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
            #[doc = "Clear only the ERQ bit specified in the CERQ field"]
            pub const CAER_0: u8 = 0;
            #[doc = "Clear all bits in ERQ"]
            pub const CAER_1: u8 = 0x01;
        }
    }
    #[doc = "No Op enable"]
    pub mod NOP {
        pub const offset: u8 = 7;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Normal operation"]
            pub const NOP_0: u8 = 0;
            #[doc = "No operation, ignore the other bits in this register"]
            pub const NOP_1: u8 = 0x01;
        }
    }
}
#[doc = "Set Enable Request Register"]
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
            #[doc = "Set only the ERQ bit specified in the SERQ field"]
            pub const SAER_0: u8 = 0;
            #[doc = "Set all bits in ERQ"]
            pub const SAER_1: u8 = 0x01;
        }
    }
    #[doc = "No Op enable"]
    pub mod NOP {
        pub const offset: u8 = 7;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Normal operation"]
            pub const NOP_0: u8 = 0;
            #[doc = "No operation, ignore the other bits in this register"]
            pub const NOP_1: u8 = 0x01;
        }
    }
}
#[doc = "Clear DONE Status Bit Register"]
pub mod CDNE {
    #[doc = "Clear DONE Bit"]
    pub mod CDNE {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Clears All DONE Bits"]
    pub mod CADN {
        pub const offset: u8 = 6;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Clears only the TCDn_CSR\\[DONE\\] bit specified in the CDNE field"]
            pub const CADN_0: u8 = 0;
            #[doc = "Clears all bits in TCDn_CSR\\[DONE\\]"]
            pub const CADN_1: u8 = 0x01;
        }
    }
    #[doc = "No Op enable"]
    pub mod NOP {
        pub const offset: u8 = 7;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Normal operation"]
            pub const NOP_0: u8 = 0;
            #[doc = "No operation, ignore the other bits in this register"]
            pub const NOP_1: u8 = 0x01;
        }
    }
}
#[doc = "Set START Bit Register"]
pub mod SSRT {
    #[doc = "Set START Bit"]
    pub mod SSRT {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set All START Bits (activates all channels)"]
    pub mod SAST {
        pub const offset: u8 = 6;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Set only the TCDn_CSR\\[START\\] bit specified in the SSRT field"]
            pub const SAST_0: u8 = 0;
            #[doc = "Set all bits in TCDn_CSR\\[START\\]"]
            pub const SAST_1: u8 = 0x01;
        }
    }
    #[doc = "No Op enable"]
    pub mod NOP {
        pub const offset: u8 = 7;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Normal operation"]
            pub const NOP_0: u8 = 0;
            #[doc = "No operation, ignore the other bits in this register"]
            pub const NOP_1: u8 = 0x01;
        }
    }
}
#[doc = "Clear Error Register"]
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
            #[doc = "Clear only the ERR bit specified in the CERR field"]
            pub const CAEI_0: u8 = 0;
            #[doc = "Clear all bits in ERR"]
            pub const CAEI_1: u8 = 0x01;
        }
    }
    #[doc = "No Op enable"]
    pub mod NOP {
        pub const offset: u8 = 7;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Normal operation"]
            pub const NOP_0: u8 = 0;
            #[doc = "No operation, ignore the other bits in this register"]
            pub const NOP_1: u8 = 0x01;
        }
    }
}
#[doc = "Clear Interrupt Request Register"]
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
            #[doc = "Clear only the INT bit specified in the CINT field"]
            pub const CAIR_0: u8 = 0;
            #[doc = "Clear all bits in INT"]
            pub const CAIR_1: u8 = 0x01;
        }
    }
    #[doc = "No Op enable"]
    pub mod NOP {
        pub const offset: u8 = 7;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Normal operation"]
            pub const NOP_0: u8 = 0;
            #[doc = "No operation, ignore the other bits in this register"]
            pub const NOP_1: u8 = 0x01;
        }
    }
}
#[doc = "Interrupt Request Register"]
pub mod INT {
    #[doc = "Interrupt Request 0"]
    pub mod INT0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The interrupt request for corresponding channel is cleared"]
            pub const INT0_0: u32 = 0;
            #[doc = "The interrupt request for corresponding channel is active"]
            pub const INT0_1: u32 = 0x01;
        }
    }
    #[doc = "Interrupt Request 1"]
    pub mod INT1 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The interrupt request for corresponding channel is cleared"]
            pub const INT1_0: u32 = 0;
            #[doc = "The interrupt request for corresponding channel is active"]
            pub const INT1_1: u32 = 0x01;
        }
    }
    #[doc = "Interrupt Request 2"]
    pub mod INT2 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The interrupt request for corresponding channel is cleared"]
            pub const INT2_0: u32 = 0;
            #[doc = "The interrupt request for corresponding channel is active"]
            pub const INT2_1: u32 = 0x01;
        }
    }
    #[doc = "Interrupt Request 3"]
    pub mod INT3 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The interrupt request for corresponding channel is cleared"]
            pub const INT3_0: u32 = 0;
            #[doc = "The interrupt request for corresponding channel is active"]
            pub const INT3_1: u32 = 0x01;
        }
    }
    #[doc = "Interrupt Request 4"]
    pub mod INT4 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The interrupt request for corresponding channel is cleared"]
            pub const INT4_0: u32 = 0;
            #[doc = "The interrupt request for corresponding channel is active"]
            pub const INT4_1: u32 = 0x01;
        }
    }
    #[doc = "Interrupt Request 5"]
    pub mod INT5 {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The interrupt request for corresponding channel is cleared"]
            pub const INT5_0: u32 = 0;
            #[doc = "The interrupt request for corresponding channel is active"]
            pub const INT5_1: u32 = 0x01;
        }
    }
    #[doc = "Interrupt Request 6"]
    pub mod INT6 {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The interrupt request for corresponding channel is cleared"]
            pub const INT6_0: u32 = 0;
            #[doc = "The interrupt request for corresponding channel is active"]
            pub const INT6_1: u32 = 0x01;
        }
    }
    #[doc = "Interrupt Request 7"]
    pub mod INT7 {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The interrupt request for corresponding channel is cleared"]
            pub const INT7_0: u32 = 0;
            #[doc = "The interrupt request for corresponding channel is active"]
            pub const INT7_1: u32 = 0x01;
        }
    }
    #[doc = "Interrupt Request 8"]
    pub mod INT8 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The interrupt request for corresponding channel is cleared"]
            pub const INT8_0: u32 = 0;
            #[doc = "The interrupt request for corresponding channel is active"]
            pub const INT8_1: u32 = 0x01;
        }
    }
    #[doc = "Interrupt Request 9"]
    pub mod INT9 {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The interrupt request for corresponding channel is cleared"]
            pub const INT9_0: u32 = 0;
            #[doc = "The interrupt request for corresponding channel is active"]
            pub const INT9_1: u32 = 0x01;
        }
    }
    #[doc = "Interrupt Request 10"]
    pub mod INT10 {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The interrupt request for corresponding channel is cleared"]
            pub const INT10_0: u32 = 0;
            #[doc = "The interrupt request for corresponding channel is active"]
            pub const INT10_1: u32 = 0x01;
        }
    }
    #[doc = "Interrupt Request 11"]
    pub mod INT11 {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The interrupt request for corresponding channel is cleared"]
            pub const INT11_0: u32 = 0;
            #[doc = "The interrupt request for corresponding channel is active"]
            pub const INT11_1: u32 = 0x01;
        }
    }
    #[doc = "Interrupt Request 12"]
    pub mod INT12 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The interrupt request for corresponding channel is cleared"]
            pub const INT12_0: u32 = 0;
            #[doc = "The interrupt request for corresponding channel is active"]
            pub const INT12_1: u32 = 0x01;
        }
    }
    #[doc = "Interrupt Request 13"]
    pub mod INT13 {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The interrupt request for corresponding channel is cleared"]
            pub const INT13_0: u32 = 0;
            #[doc = "The interrupt request for corresponding channel is active"]
            pub const INT13_1: u32 = 0x01;
        }
    }
    #[doc = "Interrupt Request 14"]
    pub mod INT14 {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The interrupt request for corresponding channel is cleared"]
            pub const INT14_0: u32 = 0;
            #[doc = "The interrupt request for corresponding channel is active"]
            pub const INT14_1: u32 = 0x01;
        }
    }
    #[doc = "Interrupt Request 15"]
    pub mod INT15 {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The interrupt request for corresponding channel is cleared"]
            pub const INT15_0: u32 = 0;
            #[doc = "The interrupt request for corresponding channel is active"]
            pub const INT15_1: u32 = 0x01;
        }
    }
    #[doc = "Interrupt Request 16"]
    pub mod INT16 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The interrupt request for corresponding channel is cleared"]
            pub const INT16_0: u32 = 0;
            #[doc = "The interrupt request for corresponding channel is active"]
            pub const INT16_1: u32 = 0x01;
        }
    }
    #[doc = "Interrupt Request 17"]
    pub mod INT17 {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The interrupt request for corresponding channel is cleared"]
            pub const INT17_0: u32 = 0;
            #[doc = "The interrupt request for corresponding channel is active"]
            pub const INT17_1: u32 = 0x01;
        }
    }
    #[doc = "Interrupt Request 18"]
    pub mod INT18 {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The interrupt request for corresponding channel is cleared"]
            pub const INT18_0: u32 = 0;
            #[doc = "The interrupt request for corresponding channel is active"]
            pub const INT18_1: u32 = 0x01;
        }
    }
    #[doc = "Interrupt Request 19"]
    pub mod INT19 {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The interrupt request for corresponding channel is cleared"]
            pub const INT19_0: u32 = 0;
            #[doc = "The interrupt request for corresponding channel is active"]
            pub const INT19_1: u32 = 0x01;
        }
    }
    #[doc = "Interrupt Request 20"]
    pub mod INT20 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The interrupt request for corresponding channel is cleared"]
            pub const INT20_0: u32 = 0;
            #[doc = "The interrupt request for corresponding channel is active"]
            pub const INT20_1: u32 = 0x01;
        }
    }
    #[doc = "Interrupt Request 21"]
    pub mod INT21 {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The interrupt request for corresponding channel is cleared"]
            pub const INT21_0: u32 = 0;
            #[doc = "The interrupt request for corresponding channel is active"]
            pub const INT21_1: u32 = 0x01;
        }
    }
    #[doc = "Interrupt Request 22"]
    pub mod INT22 {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The interrupt request for corresponding channel is cleared"]
            pub const INT22_0: u32 = 0;
            #[doc = "The interrupt request for corresponding channel is active"]
            pub const INT22_1: u32 = 0x01;
        }
    }
    #[doc = "Interrupt Request 23"]
    pub mod INT23 {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The interrupt request for corresponding channel is cleared"]
            pub const INT23_0: u32 = 0;
            #[doc = "The interrupt request for corresponding channel is active"]
            pub const INT23_1: u32 = 0x01;
        }
    }
    #[doc = "Interrupt Request 24"]
    pub mod INT24 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The interrupt request for corresponding channel is cleared"]
            pub const INT24_0: u32 = 0;
            #[doc = "The interrupt request for corresponding channel is active"]
            pub const INT24_1: u32 = 0x01;
        }
    }
    #[doc = "Interrupt Request 25"]
    pub mod INT25 {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The interrupt request for corresponding channel is cleared"]
            pub const INT25_0: u32 = 0;
            #[doc = "The interrupt request for corresponding channel is active"]
            pub const INT25_1: u32 = 0x01;
        }
    }
    #[doc = "Interrupt Request 26"]
    pub mod INT26 {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The interrupt request for corresponding channel is cleared"]
            pub const INT26_0: u32 = 0;
            #[doc = "The interrupt request for corresponding channel is active"]
            pub const INT26_1: u32 = 0x01;
        }
    }
    #[doc = "Interrupt Request 27"]
    pub mod INT27 {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The interrupt request for corresponding channel is cleared"]
            pub const INT27_0: u32 = 0;
            #[doc = "The interrupt request for corresponding channel is active"]
            pub const INT27_1: u32 = 0x01;
        }
    }
    #[doc = "Interrupt Request 28"]
    pub mod INT28 {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The interrupt request for corresponding channel is cleared"]
            pub const INT28_0: u32 = 0;
            #[doc = "The interrupt request for corresponding channel is active"]
            pub const INT28_1: u32 = 0x01;
        }
    }
    #[doc = "Interrupt Request 29"]
    pub mod INT29 {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The interrupt request for corresponding channel is cleared"]
            pub const INT29_0: u32 = 0;
            #[doc = "The interrupt request for corresponding channel is active"]
            pub const INT29_1: u32 = 0x01;
        }
    }
    #[doc = "Interrupt Request 30"]
    pub mod INT30 {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The interrupt request for corresponding channel is cleared"]
            pub const INT30_0: u32 = 0;
            #[doc = "The interrupt request for corresponding channel is active"]
            pub const INT30_1: u32 = 0x01;
        }
    }
    #[doc = "Interrupt Request 31"]
    pub mod INT31 {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The interrupt request for corresponding channel is cleared"]
            pub const INT31_0: u32 = 0;
            #[doc = "The interrupt request for corresponding channel is active"]
            pub const INT31_1: u32 = 0x01;
        }
    }
}
#[doc = "Error Register"]
pub mod ERR {
    #[doc = "Error In Channel 0"]
    pub mod ERR0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "An error in this channel has not occurred"]
            pub const ERR0_0: u32 = 0;
            #[doc = "An error in this channel has occurred"]
            pub const ERR0_1: u32 = 0x01;
        }
    }
    #[doc = "Error In Channel 1"]
    pub mod ERR1 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "An error in this channel has not occurred"]
            pub const ERR1_0: u32 = 0;
            #[doc = "An error in this channel has occurred"]
            pub const ERR1_1: u32 = 0x01;
        }
    }
    #[doc = "Error In Channel 2"]
    pub mod ERR2 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "An error in this channel has not occurred"]
            pub const ERR2_0: u32 = 0;
            #[doc = "An error in this channel has occurred"]
            pub const ERR2_1: u32 = 0x01;
        }
    }
    #[doc = "Error In Channel 3"]
    pub mod ERR3 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "An error in this channel has not occurred"]
            pub const ERR3_0: u32 = 0;
            #[doc = "An error in this channel has occurred"]
            pub const ERR3_1: u32 = 0x01;
        }
    }
    #[doc = "Error In Channel 4"]
    pub mod ERR4 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "An error in this channel has not occurred"]
            pub const ERR4_0: u32 = 0;
            #[doc = "An error in this channel has occurred"]
            pub const ERR4_1: u32 = 0x01;
        }
    }
    #[doc = "Error In Channel 5"]
    pub mod ERR5 {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "An error in this channel has not occurred"]
            pub const ERR5_0: u32 = 0;
            #[doc = "An error in this channel has occurred"]
            pub const ERR5_1: u32 = 0x01;
        }
    }
    #[doc = "Error In Channel 6"]
    pub mod ERR6 {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "An error in this channel has not occurred"]
            pub const ERR6_0: u32 = 0;
            #[doc = "An error in this channel has occurred"]
            pub const ERR6_1: u32 = 0x01;
        }
    }
    #[doc = "Error In Channel 7"]
    pub mod ERR7 {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "An error in this channel has not occurred"]
            pub const ERR7_0: u32 = 0;
            #[doc = "An error in this channel has occurred"]
            pub const ERR7_1: u32 = 0x01;
        }
    }
    #[doc = "Error In Channel 8"]
    pub mod ERR8 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "An error in this channel has not occurred"]
            pub const ERR8_0: u32 = 0;
            #[doc = "An error in this channel has occurred"]
            pub const ERR8_1: u32 = 0x01;
        }
    }
    #[doc = "Error In Channel 9"]
    pub mod ERR9 {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "An error in this channel has not occurred"]
            pub const ERR9_0: u32 = 0;
            #[doc = "An error in this channel has occurred"]
            pub const ERR9_1: u32 = 0x01;
        }
    }
    #[doc = "Error In Channel 10"]
    pub mod ERR10 {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "An error in this channel has not occurred"]
            pub const ERR10_0: u32 = 0;
            #[doc = "An error in this channel has occurred"]
            pub const ERR10_1: u32 = 0x01;
        }
    }
    #[doc = "Error In Channel 11"]
    pub mod ERR11 {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "An error in this channel has not occurred"]
            pub const ERR11_0: u32 = 0;
            #[doc = "An error in this channel has occurred"]
            pub const ERR11_1: u32 = 0x01;
        }
    }
    #[doc = "Error In Channel 12"]
    pub mod ERR12 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "An error in this channel has not occurred"]
            pub const ERR12_0: u32 = 0;
            #[doc = "An error in this channel has occurred"]
            pub const ERR12_1: u32 = 0x01;
        }
    }
    #[doc = "Error In Channel 13"]
    pub mod ERR13 {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "An error in this channel has not occurred"]
            pub const ERR13_0: u32 = 0;
            #[doc = "An error in this channel has occurred"]
            pub const ERR13_1: u32 = 0x01;
        }
    }
    #[doc = "Error In Channel 14"]
    pub mod ERR14 {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "An error in this channel has not occurred"]
            pub const ERR14_0: u32 = 0;
            #[doc = "An error in this channel has occurred"]
            pub const ERR14_1: u32 = 0x01;
        }
    }
    #[doc = "Error In Channel 15"]
    pub mod ERR15 {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "An error in this channel has not occurred"]
            pub const ERR15_0: u32 = 0;
            #[doc = "An error in this channel has occurred"]
            pub const ERR15_1: u32 = 0x01;
        }
    }
    #[doc = "Error In Channel 16"]
    pub mod ERR16 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "An error in this channel has not occurred"]
            pub const ERR16_0: u32 = 0;
            #[doc = "An error in this channel has occurred"]
            pub const ERR16_1: u32 = 0x01;
        }
    }
    #[doc = "Error In Channel 17"]
    pub mod ERR17 {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "An error in this channel has not occurred"]
            pub const ERR17_0: u32 = 0;
            #[doc = "An error in this channel has occurred"]
            pub const ERR17_1: u32 = 0x01;
        }
    }
    #[doc = "Error In Channel 18"]
    pub mod ERR18 {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "An error in this channel has not occurred"]
            pub const ERR18_0: u32 = 0;
            #[doc = "An error in this channel has occurred"]
            pub const ERR18_1: u32 = 0x01;
        }
    }
    #[doc = "Error In Channel 19"]
    pub mod ERR19 {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "An error in this channel has not occurred"]
            pub const ERR19_0: u32 = 0;
            #[doc = "An error in this channel has occurred"]
            pub const ERR19_1: u32 = 0x01;
        }
    }
    #[doc = "Error In Channel 20"]
    pub mod ERR20 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "An error in this channel has not occurred"]
            pub const ERR20_0: u32 = 0;
            #[doc = "An error in this channel has occurred"]
            pub const ERR20_1: u32 = 0x01;
        }
    }
    #[doc = "Error In Channel 21"]
    pub mod ERR21 {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "An error in this channel has not occurred"]
            pub const ERR21_0: u32 = 0;
            #[doc = "An error in this channel has occurred"]
            pub const ERR21_1: u32 = 0x01;
        }
    }
    #[doc = "Error In Channel 22"]
    pub mod ERR22 {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "An error in this channel has not occurred"]
            pub const ERR22_0: u32 = 0;
            #[doc = "An error in this channel has occurred"]
            pub const ERR22_1: u32 = 0x01;
        }
    }
    #[doc = "Error In Channel 23"]
    pub mod ERR23 {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "An error in this channel has not occurred"]
            pub const ERR23_0: u32 = 0;
            #[doc = "An error in this channel has occurred"]
            pub const ERR23_1: u32 = 0x01;
        }
    }
    #[doc = "Error In Channel 24"]
    pub mod ERR24 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "An error in this channel has not occurred"]
            pub const ERR24_0: u32 = 0;
            #[doc = "An error in this channel has occurred"]
            pub const ERR24_1: u32 = 0x01;
        }
    }
    #[doc = "Error In Channel 25"]
    pub mod ERR25 {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "An error in this channel has not occurred"]
            pub const ERR25_0: u32 = 0;
            #[doc = "An error in this channel has occurred"]
            pub const ERR25_1: u32 = 0x01;
        }
    }
    #[doc = "Error In Channel 26"]
    pub mod ERR26 {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "An error in this channel has not occurred"]
            pub const ERR26_0: u32 = 0;
            #[doc = "An error in this channel has occurred"]
            pub const ERR26_1: u32 = 0x01;
        }
    }
    #[doc = "Error In Channel 27"]
    pub mod ERR27 {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "An error in this channel has not occurred"]
            pub const ERR27_0: u32 = 0;
            #[doc = "An error in this channel has occurred"]
            pub const ERR27_1: u32 = 0x01;
        }
    }
    #[doc = "Error In Channel 28"]
    pub mod ERR28 {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "An error in this channel has not occurred"]
            pub const ERR28_0: u32 = 0;
            #[doc = "An error in this channel has occurred"]
            pub const ERR28_1: u32 = 0x01;
        }
    }
    #[doc = "Error In Channel 29"]
    pub mod ERR29 {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "An error in this channel has not occurred"]
            pub const ERR29_0: u32 = 0;
            #[doc = "An error in this channel has occurred"]
            pub const ERR29_1: u32 = 0x01;
        }
    }
    #[doc = "Error In Channel 30"]
    pub mod ERR30 {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "An error in this channel has not occurred"]
            pub const ERR30_0: u32 = 0;
            #[doc = "An error in this channel has occurred"]
            pub const ERR30_1: u32 = 0x01;
        }
    }
    #[doc = "Error In Channel 31"]
    pub mod ERR31 {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "An error in this channel has not occurred"]
            pub const ERR31_0: u32 = 0;
            #[doc = "An error in this channel has occurred"]
            pub const ERR31_1: u32 = 0x01;
        }
    }
}
#[doc = "Hardware Request Status Register"]
pub mod HRS {
    #[doc = "Hardware Request Status Channel 0"]
    pub mod HRS0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "A hardware service request for channel 0 is not present"]
            pub const HRS0_0: u32 = 0;
            #[doc = "A hardware service request for channel 0 is present"]
            pub const HRS0_1: u32 = 0x01;
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
            pub const HRS1_0: u32 = 0;
            #[doc = "A hardware service request for channel 1 is present"]
            pub const HRS1_1: u32 = 0x01;
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
            pub const HRS2_0: u32 = 0;
            #[doc = "A hardware service request for channel 2 is present"]
            pub const HRS2_1: u32 = 0x01;
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
            pub const HRS3_0: u32 = 0;
            #[doc = "A hardware service request for channel 3 is present"]
            pub const HRS3_1: u32 = 0x01;
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
            pub const HRS4_0: u32 = 0;
            #[doc = "A hardware service request for channel 4 is present"]
            pub const HRS4_1: u32 = 0x01;
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
            pub const HRS5_0: u32 = 0;
            #[doc = "A hardware service request for channel 5 is present"]
            pub const HRS5_1: u32 = 0x01;
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
            pub const HRS6_0: u32 = 0;
            #[doc = "A hardware service request for channel 6 is present"]
            pub const HRS6_1: u32 = 0x01;
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
            pub const HRS7_0: u32 = 0;
            #[doc = "A hardware service request for channel 7 is present"]
            pub const HRS7_1: u32 = 0x01;
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
            pub const HRS8_0: u32 = 0;
            #[doc = "A hardware service request for channel 8 is present"]
            pub const HRS8_1: u32 = 0x01;
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
            pub const HRS9_0: u32 = 0;
            #[doc = "A hardware service request for channel 9 is present"]
            pub const HRS9_1: u32 = 0x01;
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
            pub const HRS10_0: u32 = 0;
            #[doc = "A hardware service request for channel 10 is present"]
            pub const HRS10_1: u32 = 0x01;
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
            pub const HRS11_0: u32 = 0;
            #[doc = "A hardware service request for channel 11 is present"]
            pub const HRS11_1: u32 = 0x01;
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
            pub const HRS12_0: u32 = 0;
            #[doc = "A hardware service request for channel 12 is present"]
            pub const HRS12_1: u32 = 0x01;
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
            pub const HRS13_0: u32 = 0;
            #[doc = "A hardware service request for channel 13 is present"]
            pub const HRS13_1: u32 = 0x01;
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
            pub const HRS14_0: u32 = 0;
            #[doc = "A hardware service request for channel 14 is present"]
            pub const HRS14_1: u32 = 0x01;
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
            pub const HRS15_0: u32 = 0;
            #[doc = "A hardware service request for channel 15 is present"]
            pub const HRS15_1: u32 = 0x01;
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
            pub const HRS16_0: u32 = 0;
            #[doc = "A hardware service request for channel 16 is present"]
            pub const HRS16_1: u32 = 0x01;
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
            pub const HRS17_0: u32 = 0;
            #[doc = "A hardware service request for channel 17 is present"]
            pub const HRS17_1: u32 = 0x01;
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
            pub const HRS18_0: u32 = 0;
            #[doc = "A hardware service request for channel 18 is present"]
            pub const HRS18_1: u32 = 0x01;
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
            pub const HRS19_0: u32 = 0;
            #[doc = "A hardware service request for channel 19 is present"]
            pub const HRS19_1: u32 = 0x01;
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
            pub const HRS20_0: u32 = 0;
            #[doc = "A hardware service request for channel 20 is present"]
            pub const HRS20_1: u32 = 0x01;
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
            pub const HRS21_0: u32 = 0;
            #[doc = "A hardware service request for channel 21 is present"]
            pub const HRS21_1: u32 = 0x01;
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
            pub const HRS22_0: u32 = 0;
            #[doc = "A hardware service request for channel 22 is present"]
            pub const HRS22_1: u32 = 0x01;
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
            pub const HRS23_0: u32 = 0;
            #[doc = "A hardware service request for channel 23 is present"]
            pub const HRS23_1: u32 = 0x01;
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
            pub const HRS24_0: u32 = 0;
            #[doc = "A hardware service request for channel 24 is present"]
            pub const HRS24_1: u32 = 0x01;
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
            pub const HRS25_0: u32 = 0;
            #[doc = "A hardware service request for channel 25 is present"]
            pub const HRS25_1: u32 = 0x01;
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
            pub const HRS26_0: u32 = 0;
            #[doc = "A hardware service request for channel 26 is present"]
            pub const HRS26_1: u32 = 0x01;
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
            pub const HRS27_0: u32 = 0;
            #[doc = "A hardware service request for channel 27 is present"]
            pub const HRS27_1: u32 = 0x01;
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
            pub const HRS28_0: u32 = 0;
            #[doc = "A hardware service request for channel 28 is present"]
            pub const HRS28_1: u32 = 0x01;
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
            pub const HRS29_0: u32 = 0;
            #[doc = "A hardware service request for channel 29 is present"]
            pub const HRS29_1: u32 = 0x01;
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
            pub const HRS30_0: u32 = 0;
            #[doc = "A hardware service request for channel 30 is present"]
            pub const HRS30_1: u32 = 0x01;
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
            pub const HRS31_0: u32 = 0;
            #[doc = "A hardware service request for channel 31 is present"]
            pub const HRS31_1: u32 = 0x01;
        }
    }
}
#[doc = "Enable Asynchronous Request in Stop Register"]
pub mod EARS {
    #[doc = "Enable asynchronous DMA request in stop mode for channel 0."]
    pub mod EDREQ_0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable asynchronous DMA request for channel 0."]
            pub const EDREQ_0_0: u32 = 0;
            #[doc = "Enable asynchronous DMA request for channel 0."]
            pub const EDREQ_0_1: u32 = 0x01;
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
            pub const EDREQ_1_0: u32 = 0;
            #[doc = "Enable asynchronous DMA request for channel 1."]
            pub const EDREQ_1_1: u32 = 0x01;
        }
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 2."]
    pub mod EDREQ_2 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable asynchronous DMA request for channel 2."]
            pub const EDREQ_2_0: u32 = 0;
            #[doc = "Enable asynchronous DMA request for channel 2."]
            pub const EDREQ_2_1: u32 = 0x01;
        }
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 3."]
    pub mod EDREQ_3 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable asynchronous DMA request for channel 3."]
            pub const EDREQ_3_0: u32 = 0;
            #[doc = "Enable asynchronous DMA request for channel 3."]
            pub const EDREQ_3_1: u32 = 0x01;
        }
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 4"]
    pub mod EDREQ_4 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable asynchronous DMA request for channel 4."]
            pub const EDREQ_4_0: u32 = 0;
            #[doc = "Enable asynchronous DMA request for channel 4."]
            pub const EDREQ_4_1: u32 = 0x01;
        }
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 5"]
    pub mod EDREQ_5 {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable asynchronous DMA request for channel 5."]
            pub const EDREQ_5_0: u32 = 0;
            #[doc = "Enable asynchronous DMA request for channel 5."]
            pub const EDREQ_5_1: u32 = 0x01;
        }
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 6"]
    pub mod EDREQ_6 {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable asynchronous DMA request for channel 6."]
            pub const EDREQ_6_0: u32 = 0;
            #[doc = "Enable asynchronous DMA request for channel 6."]
            pub const EDREQ_6_1: u32 = 0x01;
        }
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 7"]
    pub mod EDREQ_7 {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable asynchronous DMA request for channel 7."]
            pub const EDREQ_7_0: u32 = 0;
            #[doc = "Enable asynchronous DMA request for channel 7."]
            pub const EDREQ_7_1: u32 = 0x01;
        }
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 8"]
    pub mod EDREQ_8 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable asynchronous DMA request for channel 8."]
            pub const EDREQ_8_0: u32 = 0;
            #[doc = "Enable asynchronous DMA request for channel 8."]
            pub const EDREQ_8_1: u32 = 0x01;
        }
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 9"]
    pub mod EDREQ_9 {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable asynchronous DMA request for channel 9."]
            pub const EDREQ_9_0: u32 = 0;
            #[doc = "Enable asynchronous DMA request for channel 9."]
            pub const EDREQ_9_1: u32 = 0x01;
        }
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 10"]
    pub mod EDREQ_10 {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable asynchronous DMA request for channel 10."]
            pub const EDREQ_10_0: u32 = 0;
            #[doc = "Enable asynchronous DMA request for channel 10."]
            pub const EDREQ_10_1: u32 = 0x01;
        }
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 11"]
    pub mod EDREQ_11 {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable asynchronous DMA request for channel 11."]
            pub const EDREQ_11_0: u32 = 0;
            #[doc = "Enable asynchronous DMA request for channel 11."]
            pub const EDREQ_11_1: u32 = 0x01;
        }
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 12"]
    pub mod EDREQ_12 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable asynchronous DMA request for channel 12."]
            pub const EDREQ_12_0: u32 = 0;
            #[doc = "Enable asynchronous DMA request for channel 12."]
            pub const EDREQ_12_1: u32 = 0x01;
        }
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 13"]
    pub mod EDREQ_13 {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable asynchronous DMA request for channel 13."]
            pub const EDREQ_13_0: u32 = 0;
            #[doc = "Enable asynchronous DMA request for channel 13."]
            pub const EDREQ_13_1: u32 = 0x01;
        }
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 14"]
    pub mod EDREQ_14 {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable asynchronous DMA request for channel 14."]
            pub const EDREQ_14_0: u32 = 0;
            #[doc = "Enable asynchronous DMA request for channel 14."]
            pub const EDREQ_14_1: u32 = 0x01;
        }
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 15"]
    pub mod EDREQ_15 {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable asynchronous DMA request for channel 15."]
            pub const EDREQ_15_0: u32 = 0;
            #[doc = "Enable asynchronous DMA request for channel 15."]
            pub const EDREQ_15_1: u32 = 0x01;
        }
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 16"]
    pub mod EDREQ_16 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable asynchronous DMA request for channel 16"]
            pub const EDREQ_16_0: u32 = 0;
            #[doc = "Enable asynchronous DMA request for channel 16"]
            pub const EDREQ_16_1: u32 = 0x01;
        }
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 17"]
    pub mod EDREQ_17 {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable asynchronous DMA request for channel 17"]
            pub const EDREQ_17_0: u32 = 0;
            #[doc = "Enable asynchronous DMA request for channel 17"]
            pub const EDREQ_17_1: u32 = 0x01;
        }
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 18"]
    pub mod EDREQ_18 {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable asynchronous DMA request for channel 18"]
            pub const EDREQ_18_0: u32 = 0;
            #[doc = "Enable asynchronous DMA request for channel 18"]
            pub const EDREQ_18_1: u32 = 0x01;
        }
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 19"]
    pub mod EDREQ_19 {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable asynchronous DMA request for channel 19"]
            pub const EDREQ_19_0: u32 = 0;
            #[doc = "Enable asynchronous DMA request for channel 19"]
            pub const EDREQ_19_1: u32 = 0x01;
        }
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 20"]
    pub mod EDREQ_20 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable asynchronous DMA request for channel 20"]
            pub const EDREQ_20_0: u32 = 0;
            #[doc = "Enable asynchronous DMA request for channel 20"]
            pub const EDREQ_20_1: u32 = 0x01;
        }
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 21"]
    pub mod EDREQ_21 {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable asynchronous DMA request for channel 21"]
            pub const EDREQ_21_0: u32 = 0;
            #[doc = "Enable asynchronous DMA request for channel 21"]
            pub const EDREQ_21_1: u32 = 0x01;
        }
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 22"]
    pub mod EDREQ_22 {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable asynchronous DMA request for channel 22"]
            pub const EDREQ_22_0: u32 = 0;
            #[doc = "Enable asynchronous DMA request for channel 22"]
            pub const EDREQ_22_1: u32 = 0x01;
        }
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 23"]
    pub mod EDREQ_23 {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable asynchronous DMA request for channel 23"]
            pub const EDREQ_23_0: u32 = 0;
            #[doc = "Enable asynchronous DMA request for channel 23"]
            pub const EDREQ_23_1: u32 = 0x01;
        }
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 24"]
    pub mod EDREQ_24 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable asynchronous DMA request for channel 24"]
            pub const EDREQ_24_0: u32 = 0;
            #[doc = "Enable asynchronous DMA request for channel 24"]
            pub const EDREQ_24_1: u32 = 0x01;
        }
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 25"]
    pub mod EDREQ_25 {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable asynchronous DMA request for channel 25"]
            pub const EDREQ_25_0: u32 = 0;
            #[doc = "Enable asynchronous DMA request for channel 25"]
            pub const EDREQ_25_1: u32 = 0x01;
        }
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 26"]
    pub mod EDREQ_26 {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable asynchronous DMA request for channel 26"]
            pub const EDREQ_26_0: u32 = 0;
            #[doc = "Enable asynchronous DMA request for channel 26"]
            pub const EDREQ_26_1: u32 = 0x01;
        }
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 27"]
    pub mod EDREQ_27 {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable asynchronous DMA request for channel 27"]
            pub const EDREQ_27_0: u32 = 0;
            #[doc = "Enable asynchronous DMA request for channel 27"]
            pub const EDREQ_27_1: u32 = 0x01;
        }
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 28"]
    pub mod EDREQ_28 {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable asynchronous DMA request for channel 28"]
            pub const EDREQ_28_0: u32 = 0;
            #[doc = "Enable asynchronous DMA request for channel 28"]
            pub const EDREQ_28_1: u32 = 0x01;
        }
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 29"]
    pub mod EDREQ_29 {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable asynchronous DMA request for channel 29"]
            pub const EDREQ_29_0: u32 = 0;
            #[doc = "Enable asynchronous DMA request for channel 29"]
            pub const EDREQ_29_1: u32 = 0x01;
        }
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 30"]
    pub mod EDREQ_30 {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable asynchronous DMA request for channel 30"]
            pub const EDREQ_30_0: u32 = 0;
            #[doc = "Enable asynchronous DMA request for channel 30"]
            pub const EDREQ_30_1: u32 = 0x01;
        }
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 31"]
    pub mod EDREQ_31 {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable asynchronous DMA request for channel 31"]
            pub const EDREQ_31_0: u32 = 0;
            #[doc = "Enable asynchronous DMA request for channel 31"]
            pub const EDREQ_31_1: u32 = 0x01;
        }
    }
}
#[doc = "Channel n Priority Register"]
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
            #[doc = "Channel n can suspend a lower priority channel."]
            pub const DPA_0: u8 = 0;
            #[doc = "Channel n cannot suspend any channel, regardless of channel priority."]
            pub const DPA_1: u8 = 0x01;
        }
    }
    #[doc = "Enable Channel Preemption. This field resets to 0."]
    pub mod ECP {
        pub const offset: u8 = 7;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Channel n cannot be suspended by a higher priority channel's service request."]
            pub const ECP_0: u8 = 0;
            #[doc = "Channel n can be temporarily suspended by the service request of a higher priority channel."]
            pub const ECP_1: u8 = 0x01;
        }
    }
}
#[doc = "Channel n Priority Register"]
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
            #[doc = "Channel n can suspend a lower priority channel."]
            pub const DPA_0: u8 = 0;
            #[doc = "Channel n cannot suspend any channel, regardless of channel priority."]
            pub const DPA_1: u8 = 0x01;
        }
    }
    #[doc = "Enable Channel Preemption. This field resets to 0."]
    pub mod ECP {
        pub const offset: u8 = 7;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Channel n cannot be suspended by a higher priority channel's service request."]
            pub const ECP_0: u8 = 0;
            #[doc = "Channel n can be temporarily suspended by the service request of a higher priority channel."]
            pub const ECP_1: u8 = 0x01;
        }
    }
}
#[doc = "Channel n Priority Register"]
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
            #[doc = "Channel n can suspend a lower priority channel."]
            pub const DPA_0: u8 = 0;
            #[doc = "Channel n cannot suspend any channel, regardless of channel priority."]
            pub const DPA_1: u8 = 0x01;
        }
    }
    #[doc = "Enable Channel Preemption. This field resets to 0."]
    pub mod ECP {
        pub const offset: u8 = 7;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Channel n cannot be suspended by a higher priority channel's service request."]
            pub const ECP_0: u8 = 0;
            #[doc = "Channel n can be temporarily suspended by the service request of a higher priority channel."]
            pub const ECP_1: u8 = 0x01;
        }
    }
}
#[doc = "Channel n Priority Register"]
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
            #[doc = "Channel n can suspend a lower priority channel."]
            pub const DPA_0: u8 = 0;
            #[doc = "Channel n cannot suspend any channel, regardless of channel priority."]
            pub const DPA_1: u8 = 0x01;
        }
    }
    #[doc = "Enable Channel Preemption. This field resets to 0."]
    pub mod ECP {
        pub const offset: u8 = 7;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Channel n cannot be suspended by a higher priority channel's service request."]
            pub const ECP_0: u8 = 0;
            #[doc = "Channel n can be temporarily suspended by the service request of a higher priority channel."]
            pub const ECP_1: u8 = 0x01;
        }
    }
}
#[doc = "Channel n Priority Register"]
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
            #[doc = "Channel n can suspend a lower priority channel."]
            pub const DPA_0: u8 = 0;
            #[doc = "Channel n cannot suspend any channel, regardless of channel priority."]
            pub const DPA_1: u8 = 0x01;
        }
    }
    #[doc = "Enable Channel Preemption. This field resets to 0."]
    pub mod ECP {
        pub const offset: u8 = 7;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Channel n cannot be suspended by a higher priority channel's service request."]
            pub const ECP_0: u8 = 0;
            #[doc = "Channel n can be temporarily suspended by the service request of a higher priority channel."]
            pub const ECP_1: u8 = 0x01;
        }
    }
}
#[doc = "Channel n Priority Register"]
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
            #[doc = "Channel n can suspend a lower priority channel."]
            pub const DPA_0: u8 = 0;
            #[doc = "Channel n cannot suspend any channel, regardless of channel priority."]
            pub const DPA_1: u8 = 0x01;
        }
    }
    #[doc = "Enable Channel Preemption. This field resets to 0."]
    pub mod ECP {
        pub const offset: u8 = 7;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Channel n cannot be suspended by a higher priority channel's service request."]
            pub const ECP_0: u8 = 0;
            #[doc = "Channel n can be temporarily suspended by the service request of a higher priority channel."]
            pub const ECP_1: u8 = 0x01;
        }
    }
}
#[doc = "Channel n Priority Register"]
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
            #[doc = "Channel n can suspend a lower priority channel."]
            pub const DPA_0: u8 = 0;
            #[doc = "Channel n cannot suspend any channel, regardless of channel priority."]
            pub const DPA_1: u8 = 0x01;
        }
    }
    #[doc = "Enable Channel Preemption. This field resets to 0."]
    pub mod ECP {
        pub const offset: u8 = 7;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Channel n cannot be suspended by a higher priority channel's service request."]
            pub const ECP_0: u8 = 0;
            #[doc = "Channel n can be temporarily suspended by the service request of a higher priority channel."]
            pub const ECP_1: u8 = 0x01;
        }
    }
}
#[doc = "Channel n Priority Register"]
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
            #[doc = "Channel n can suspend a lower priority channel."]
            pub const DPA_0: u8 = 0;
            #[doc = "Channel n cannot suspend any channel, regardless of channel priority."]
            pub const DPA_1: u8 = 0x01;
        }
    }
    #[doc = "Enable Channel Preemption. This field resets to 0."]
    pub mod ECP {
        pub const offset: u8 = 7;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Channel n cannot be suspended by a higher priority channel's service request."]
            pub const ECP_0: u8 = 0;
            #[doc = "Channel n can be temporarily suspended by the service request of a higher priority channel."]
            pub const ECP_1: u8 = 0x01;
        }
    }
}
#[doc = "Channel n Priority Register"]
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
            #[doc = "Channel n can suspend a lower priority channel."]
            pub const DPA_0: u8 = 0;
            #[doc = "Channel n cannot suspend any channel, regardless of channel priority."]
            pub const DPA_1: u8 = 0x01;
        }
    }
    #[doc = "Enable Channel Preemption. This field resets to 0."]
    pub mod ECP {
        pub const offset: u8 = 7;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Channel n cannot be suspended by a higher priority channel's service request."]
            pub const ECP_0: u8 = 0;
            #[doc = "Channel n can be temporarily suspended by the service request of a higher priority channel."]
            pub const ECP_1: u8 = 0x01;
        }
    }
}
#[doc = "Channel n Priority Register"]
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
            #[doc = "Channel n can suspend a lower priority channel."]
            pub const DPA_0: u8 = 0;
            #[doc = "Channel n cannot suspend any channel, regardless of channel priority."]
            pub const DPA_1: u8 = 0x01;
        }
    }
    #[doc = "Enable Channel Preemption. This field resets to 0."]
    pub mod ECP {
        pub const offset: u8 = 7;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Channel n cannot be suspended by a higher priority channel's service request."]
            pub const ECP_0: u8 = 0;
            #[doc = "Channel n can be temporarily suspended by the service request of a higher priority channel."]
            pub const ECP_1: u8 = 0x01;
        }
    }
}
#[doc = "Channel n Priority Register"]
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
            #[doc = "Channel n can suspend a lower priority channel."]
            pub const DPA_0: u8 = 0;
            #[doc = "Channel n cannot suspend any channel, regardless of channel priority."]
            pub const DPA_1: u8 = 0x01;
        }
    }
    #[doc = "Enable Channel Preemption. This field resets to 0."]
    pub mod ECP {
        pub const offset: u8 = 7;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Channel n cannot be suspended by a higher priority channel's service request."]
            pub const ECP_0: u8 = 0;
            #[doc = "Channel n can be temporarily suspended by the service request of a higher priority channel."]
            pub const ECP_1: u8 = 0x01;
        }
    }
}
#[doc = "Channel n Priority Register"]
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
            #[doc = "Channel n can suspend a lower priority channel."]
            pub const DPA_0: u8 = 0;
            #[doc = "Channel n cannot suspend any channel, regardless of channel priority."]
            pub const DPA_1: u8 = 0x01;
        }
    }
    #[doc = "Enable Channel Preemption. This field resets to 0."]
    pub mod ECP {
        pub const offset: u8 = 7;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Channel n cannot be suspended by a higher priority channel's service request."]
            pub const ECP_0: u8 = 0;
            #[doc = "Channel n can be temporarily suspended by the service request of a higher priority channel."]
            pub const ECP_1: u8 = 0x01;
        }
    }
}
#[doc = "Channel n Priority Register"]
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
            #[doc = "Channel n can suspend a lower priority channel."]
            pub const DPA_0: u8 = 0;
            #[doc = "Channel n cannot suspend any channel, regardless of channel priority."]
            pub const DPA_1: u8 = 0x01;
        }
    }
    #[doc = "Enable Channel Preemption. This field resets to 0."]
    pub mod ECP {
        pub const offset: u8 = 7;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Channel n cannot be suspended by a higher priority channel's service request."]
            pub const ECP_0: u8 = 0;
            #[doc = "Channel n can be temporarily suspended by the service request of a higher priority channel."]
            pub const ECP_1: u8 = 0x01;
        }
    }
}
#[doc = "Channel n Priority Register"]
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
            #[doc = "Channel n can suspend a lower priority channel."]
            pub const DPA_0: u8 = 0;
            #[doc = "Channel n cannot suspend any channel, regardless of channel priority."]
            pub const DPA_1: u8 = 0x01;
        }
    }
    #[doc = "Enable Channel Preemption. This field resets to 0."]
    pub mod ECP {
        pub const offset: u8 = 7;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Channel n cannot be suspended by a higher priority channel's service request."]
            pub const ECP_0: u8 = 0;
            #[doc = "Channel n can be temporarily suspended by the service request of a higher priority channel."]
            pub const ECP_1: u8 = 0x01;
        }
    }
}
#[doc = "Channel n Priority Register"]
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
            #[doc = "Channel n can suspend a lower priority channel."]
            pub const DPA_0: u8 = 0;
            #[doc = "Channel n cannot suspend any channel, regardless of channel priority."]
            pub const DPA_1: u8 = 0x01;
        }
    }
    #[doc = "Enable Channel Preemption. This field resets to 0."]
    pub mod ECP {
        pub const offset: u8 = 7;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Channel n cannot be suspended by a higher priority channel's service request."]
            pub const ECP_0: u8 = 0;
            #[doc = "Channel n can be temporarily suspended by the service request of a higher priority channel."]
            pub const ECP_1: u8 = 0x01;
        }
    }
}
#[doc = "Channel n Priority Register"]
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
            #[doc = "Channel n can suspend a lower priority channel."]
            pub const DPA_0: u8 = 0;
            #[doc = "Channel n cannot suspend any channel, regardless of channel priority."]
            pub const DPA_1: u8 = 0x01;
        }
    }
    #[doc = "Enable Channel Preemption. This field resets to 0."]
    pub mod ECP {
        pub const offset: u8 = 7;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Channel n cannot be suspended by a higher priority channel's service request."]
            pub const ECP_0: u8 = 0;
            #[doc = "Channel n can be temporarily suspended by the service request of a higher priority channel."]
            pub const ECP_1: u8 = 0x01;
        }
    }
}
#[doc = "Channel n Priority Register"]
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
            #[doc = "Channel n can suspend a lower priority channel."]
            pub const DPA_0: u8 = 0;
            #[doc = "Channel n cannot suspend any channel, regardless of channel priority."]
            pub const DPA_1: u8 = 0x01;
        }
    }
    #[doc = "Enable Channel Preemption. This field resets to 0."]
    pub mod ECP {
        pub const offset: u8 = 7;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Channel n cannot be suspended by a higher priority channel's service request."]
            pub const ECP_0: u8 = 0;
            #[doc = "Channel n can be temporarily suspended by the service request of a higher priority channel."]
            pub const ECP_1: u8 = 0x01;
        }
    }
}
#[doc = "Channel n Priority Register"]
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
            #[doc = "Channel n can suspend a lower priority channel."]
            pub const DPA_0: u8 = 0;
            #[doc = "Channel n cannot suspend any channel, regardless of channel priority."]
            pub const DPA_1: u8 = 0x01;
        }
    }
    #[doc = "Enable Channel Preemption. This field resets to 0."]
    pub mod ECP {
        pub const offset: u8 = 7;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Channel n cannot be suspended by a higher priority channel's service request."]
            pub const ECP_0: u8 = 0;
            #[doc = "Channel n can be temporarily suspended by the service request of a higher priority channel."]
            pub const ECP_1: u8 = 0x01;
        }
    }
}
#[doc = "Channel n Priority Register"]
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
            #[doc = "Channel n can suspend a lower priority channel."]
            pub const DPA_0: u8 = 0;
            #[doc = "Channel n cannot suspend any channel, regardless of channel priority."]
            pub const DPA_1: u8 = 0x01;
        }
    }
    #[doc = "Enable Channel Preemption. This field resets to 0."]
    pub mod ECP {
        pub const offset: u8 = 7;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Channel n cannot be suspended by a higher priority channel's service request."]
            pub const ECP_0: u8 = 0;
            #[doc = "Channel n can be temporarily suspended by the service request of a higher priority channel."]
            pub const ECP_1: u8 = 0x01;
        }
    }
}
#[doc = "Channel n Priority Register"]
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
            #[doc = "Channel n can suspend a lower priority channel."]
            pub const DPA_0: u8 = 0;
            #[doc = "Channel n cannot suspend any channel, regardless of channel priority."]
            pub const DPA_1: u8 = 0x01;
        }
    }
    #[doc = "Enable Channel Preemption. This field resets to 0."]
    pub mod ECP {
        pub const offset: u8 = 7;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Channel n cannot be suspended by a higher priority channel's service request."]
            pub const ECP_0: u8 = 0;
            #[doc = "Channel n can be temporarily suspended by the service request of a higher priority channel."]
            pub const ECP_1: u8 = 0x01;
        }
    }
}
#[doc = "Channel n Priority Register"]
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
            #[doc = "Channel n can suspend a lower priority channel."]
            pub const DPA_0: u8 = 0;
            #[doc = "Channel n cannot suspend any channel, regardless of channel priority."]
            pub const DPA_1: u8 = 0x01;
        }
    }
    #[doc = "Enable Channel Preemption. This field resets to 0."]
    pub mod ECP {
        pub const offset: u8 = 7;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Channel n cannot be suspended by a higher priority channel's service request."]
            pub const ECP_0: u8 = 0;
            #[doc = "Channel n can be temporarily suspended by the service request of a higher priority channel."]
            pub const ECP_1: u8 = 0x01;
        }
    }
}
#[doc = "Channel n Priority Register"]
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
            #[doc = "Channel n can suspend a lower priority channel."]
            pub const DPA_0: u8 = 0;
            #[doc = "Channel n cannot suspend any channel, regardless of channel priority."]
            pub const DPA_1: u8 = 0x01;
        }
    }
    #[doc = "Enable Channel Preemption. This field resets to 0."]
    pub mod ECP {
        pub const offset: u8 = 7;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Channel n cannot be suspended by a higher priority channel's service request."]
            pub const ECP_0: u8 = 0;
            #[doc = "Channel n can be temporarily suspended by the service request of a higher priority channel."]
            pub const ECP_1: u8 = 0x01;
        }
    }
}
#[doc = "Channel n Priority Register"]
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
            #[doc = "Channel n can suspend a lower priority channel."]
            pub const DPA_0: u8 = 0;
            #[doc = "Channel n cannot suspend any channel, regardless of channel priority."]
            pub const DPA_1: u8 = 0x01;
        }
    }
    #[doc = "Enable Channel Preemption. This field resets to 0."]
    pub mod ECP {
        pub const offset: u8 = 7;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Channel n cannot be suspended by a higher priority channel's service request."]
            pub const ECP_0: u8 = 0;
            #[doc = "Channel n can be temporarily suspended by the service request of a higher priority channel."]
            pub const ECP_1: u8 = 0x01;
        }
    }
}
#[doc = "Channel n Priority Register"]
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
            #[doc = "Channel n can suspend a lower priority channel."]
            pub const DPA_0: u8 = 0;
            #[doc = "Channel n cannot suspend any channel, regardless of channel priority."]
            pub const DPA_1: u8 = 0x01;
        }
    }
    #[doc = "Enable Channel Preemption. This field resets to 0."]
    pub mod ECP {
        pub const offset: u8 = 7;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Channel n cannot be suspended by a higher priority channel's service request."]
            pub const ECP_0: u8 = 0;
            #[doc = "Channel n can be temporarily suspended by the service request of a higher priority channel."]
            pub const ECP_1: u8 = 0x01;
        }
    }
}
#[doc = "Channel n Priority Register"]
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
            #[doc = "Channel n can suspend a lower priority channel."]
            pub const DPA_0: u8 = 0;
            #[doc = "Channel n cannot suspend any channel, regardless of channel priority."]
            pub const DPA_1: u8 = 0x01;
        }
    }
    #[doc = "Enable Channel Preemption. This field resets to 0."]
    pub mod ECP {
        pub const offset: u8 = 7;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Channel n cannot be suspended by a higher priority channel's service request."]
            pub const ECP_0: u8 = 0;
            #[doc = "Channel n can be temporarily suspended by the service request of a higher priority channel."]
            pub const ECP_1: u8 = 0x01;
        }
    }
}
#[doc = "Channel n Priority Register"]
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
            #[doc = "Channel n can suspend a lower priority channel."]
            pub const DPA_0: u8 = 0;
            #[doc = "Channel n cannot suspend any channel, regardless of channel priority."]
            pub const DPA_1: u8 = 0x01;
        }
    }
    #[doc = "Enable Channel Preemption. This field resets to 0."]
    pub mod ECP {
        pub const offset: u8 = 7;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Channel n cannot be suspended by a higher priority channel's service request."]
            pub const ECP_0: u8 = 0;
            #[doc = "Channel n can be temporarily suspended by the service request of a higher priority channel."]
            pub const ECP_1: u8 = 0x01;
        }
    }
}
#[doc = "Channel n Priority Register"]
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
            #[doc = "Channel n can suspend a lower priority channel."]
            pub const DPA_0: u8 = 0;
            #[doc = "Channel n cannot suspend any channel, regardless of channel priority."]
            pub const DPA_1: u8 = 0x01;
        }
    }
    #[doc = "Enable Channel Preemption. This field resets to 0."]
    pub mod ECP {
        pub const offset: u8 = 7;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Channel n cannot be suspended by a higher priority channel's service request."]
            pub const ECP_0: u8 = 0;
            #[doc = "Channel n can be temporarily suspended by the service request of a higher priority channel."]
            pub const ECP_1: u8 = 0x01;
        }
    }
}
#[doc = "Channel n Priority Register"]
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
            #[doc = "Channel n can suspend a lower priority channel."]
            pub const DPA_0: u8 = 0;
            #[doc = "Channel n cannot suspend any channel, regardless of channel priority."]
            pub const DPA_1: u8 = 0x01;
        }
    }
    #[doc = "Enable Channel Preemption. This field resets to 0."]
    pub mod ECP {
        pub const offset: u8 = 7;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Channel n cannot be suspended by a higher priority channel's service request."]
            pub const ECP_0: u8 = 0;
            #[doc = "Channel n can be temporarily suspended by the service request of a higher priority channel."]
            pub const ECP_1: u8 = 0x01;
        }
    }
}
#[doc = "Channel n Priority Register"]
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
            #[doc = "Channel n can suspend a lower priority channel."]
            pub const DPA_0: u8 = 0;
            #[doc = "Channel n cannot suspend any channel, regardless of channel priority."]
            pub const DPA_1: u8 = 0x01;
        }
    }
    #[doc = "Enable Channel Preemption. This field resets to 0."]
    pub mod ECP {
        pub const offset: u8 = 7;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Channel n cannot be suspended by a higher priority channel's service request."]
            pub const ECP_0: u8 = 0;
            #[doc = "Channel n can be temporarily suspended by the service request of a higher priority channel."]
            pub const ECP_1: u8 = 0x01;
        }
    }
}
#[doc = "Channel n Priority Register"]
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
            #[doc = "Channel n can suspend a lower priority channel."]
            pub const DPA_0: u8 = 0;
            #[doc = "Channel n cannot suspend any channel, regardless of channel priority."]
            pub const DPA_1: u8 = 0x01;
        }
    }
    #[doc = "Enable Channel Preemption. This field resets to 0."]
    pub mod ECP {
        pub const offset: u8 = 7;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Channel n cannot be suspended by a higher priority channel's service request."]
            pub const ECP_0: u8 = 0;
            #[doc = "Channel n can be temporarily suspended by the service request of a higher priority channel."]
            pub const ECP_1: u8 = 0x01;
        }
    }
}
#[doc = "Channel n Priority Register"]
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
            #[doc = "Channel n can suspend a lower priority channel."]
            pub const DPA_0: u8 = 0;
            #[doc = "Channel n cannot suspend any channel, regardless of channel priority."]
            pub const DPA_1: u8 = 0x01;
        }
    }
    #[doc = "Enable Channel Preemption. This field resets to 0."]
    pub mod ECP {
        pub const offset: u8 = 7;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Channel n cannot be suspended by a higher priority channel's service request."]
            pub const ECP_0: u8 = 0;
            #[doc = "Channel n can be temporarily suspended by the service request of a higher priority channel."]
            pub const ECP_1: u8 = 0x01;
        }
    }
}
#[doc = "Channel n Priority Register"]
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
            #[doc = "Channel n can suspend a lower priority channel."]
            pub const DPA_0: u8 = 0;
            #[doc = "Channel n cannot suspend any channel, regardless of channel priority."]
            pub const DPA_1: u8 = 0x01;
        }
    }
    #[doc = "Enable Channel Preemption. This field resets to 0."]
    pub mod ECP {
        pub const offset: u8 = 7;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Channel n cannot be suspended by a higher priority channel's service request."]
            pub const ECP_0: u8 = 0;
            #[doc = "Channel n can be temporarily suspended by the service request of a higher priority channel."]
            pub const ECP_1: u8 = 0x01;
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
                pub const SSIZE_0: u16 = 0;
                #[doc = "16-bit"]
                pub const SSIZE_1: u16 = 0x01;
                #[doc = "32-bit"]
                pub const SSIZE_2: u16 = 0x02;
                #[doc = "64-bit"]
                pub const SSIZE_3: u16 = 0x03;
                #[doc = "32-byte burst (4 beats of 64 bits)"]
                pub const SSIZE_5: u16 = 0x05;
            }
        }
        #[doc = "Source Address Modulo"]
        pub mod SMOD {
            pub const offset: u16 = 11;
            pub const mask: u16 = 0x1f << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Source address modulo feature is disabled"]
                pub const SMOD_0: u16 = 0;
                #[doc = "This value defines a specific address range specified to be the value after SADDR + SOFF calculation is performed on the original register value. Setting this field provides the ability to implement a circular data queue easily. For data queues requiring power-of-2 size bytes, the queue should start at a 0-modulo-size address and the SMOD field should be set to the appropriate value for the queue, freezing the desired number of upper address bits. The value programmed into this field specifies the number of lower address bits allowed to change. For a circular queue application, the SOFF is typically set to the transfer size to implement post-increment addressing with the SMOD function constraining the addresses to a 0-modulo-size range."]
                pub const SMOD_1: u16 = 0x01;
                #[doc = "This value defines a specific address range specified to be the value after SADDR + SOFF calculation is performed on the original register value. Setting this field provides the ability to implement a circular data queue easily. For data queues requiring power-of-2 size bytes, the queue should start at a 0-modulo-size address and the SMOD field should be set to the appropriate value for the queue, freezing the desired number of upper address bits. The value programmed into this field specifies the number of lower address bits allowed to change. For a circular queue application, the SOFF is typically set to the transfer size to implement post-increment addressing with the SMOD function constraining the addresses to a 0-modulo-size range."]
                pub const SMOD_2: u16 = 0x02;
                #[doc = "This value defines a specific address range specified to be the value after SADDR + SOFF calculation is performed on the original register value. Setting this field provides the ability to implement a circular data queue easily. For data queues requiring power-of-2 size bytes, the queue should start at a 0-modulo-size address and the SMOD field should be set to the appropriate value for the queue, freezing the desired number of upper address bits. The value programmed into this field specifies the number of lower address bits allowed to change. For a circular queue application, the SOFF is typically set to the transfer size to implement post-increment addressing with the SMOD function constraining the addresses to a 0-modulo-size range."]
                pub const SMOD_3: u16 = 0x03;
                #[doc = "This value defines a specific address range specified to be the value after SADDR + SOFF calculation is performed on the original register value. Setting this field provides the ability to implement a circular data queue easily. For data queues requiring power-of-2 size bytes, the queue should start at a 0-modulo-size address and the SMOD field should be set to the appropriate value for the queue, freezing the desired number of upper address bits. The value programmed into this field specifies the number of lower address bits allowed to change. For a circular queue application, the SOFF is typically set to the transfer size to implement post-increment addressing with the SMOD function constraining the addresses to a 0-modulo-size range."]
                pub const SMOD_4: u16 = 0x04;
                #[doc = "This value defines a specific address range specified to be the value after SADDR + SOFF calculation is performed on the original register value. Setting this field provides the ability to implement a circular data queue easily. For data queues requiring power-of-2 size bytes, the queue should start at a 0-modulo-size address and the SMOD field should be set to the appropriate value for the queue, freezing the desired number of upper address bits. The value programmed into this field specifies the number of lower address bits allowed to change. For a circular queue application, the SOFF is typically set to the transfer size to implement post-increment addressing with the SMOD function constraining the addresses to a 0-modulo-size range."]
                pub const SMOD_5: u16 = 0x05;
                #[doc = "This value defines a specific address range specified to be the value after SADDR + SOFF calculation is performed on the original register value. Setting this field provides the ability to implement a circular data queue easily. For data queues requiring power-of-2 size bytes, the queue should start at a 0-modulo-size address and the SMOD field should be set to the appropriate value for the queue, freezing the desired number of upper address bits. The value programmed into this field specifies the number of lower address bits allowed to change. For a circular queue application, the SOFF is typically set to the transfer size to implement post-increment addressing with the SMOD function constraining the addresses to a 0-modulo-size range."]
                pub const SMOD_6: u16 = 0x06;
                #[doc = "This value defines a specific address range specified to be the value after SADDR + SOFF calculation is performed on the original register value. Setting this field provides the ability to implement a circular data queue easily. For data queues requiring power-of-2 size bytes, the queue should start at a 0-modulo-size address and the SMOD field should be set to the appropriate value for the queue, freezing the desired number of upper address bits. The value programmed into this field specifies the number of lower address bits allowed to change. For a circular queue application, the SOFF is typically set to the transfer size to implement post-increment addressing with the SMOD function constraining the addresses to a 0-modulo-size range."]
                pub const SMOD_7: u16 = 0x07;
                #[doc = "This value defines a specific address range specified to be the value after SADDR + SOFF calculation is performed on the original register value. Setting this field provides the ability to implement a circular data queue easily. For data queues requiring power-of-2 size bytes, the queue should start at a 0-modulo-size address and the SMOD field should be set to the appropriate value for the queue, freezing the desired number of upper address bits. The value programmed into this field specifies the number of lower address bits allowed to change. For a circular queue application, the SOFF is typically set to the transfer size to implement post-increment addressing with the SMOD function constraining the addresses to a 0-modulo-size range."]
                pub const SMOD_8: u16 = 0x08;
                #[doc = "This value defines a specific address range specified to be the value after SADDR + SOFF calculation is performed on the original register value. Setting this field provides the ability to implement a circular data queue easily. For data queues requiring power-of-2 size bytes, the queue should start at a 0-modulo-size address and the SMOD field should be set to the appropriate value for the queue, freezing the desired number of upper address bits. The value programmed into this field specifies the number of lower address bits allowed to change. For a circular queue application, the SOFF is typically set to the transfer size to implement post-increment addressing with the SMOD function constraining the addresses to a 0-modulo-size range."]
                pub const SMOD_9: u16 = 0x09;
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
                #[doc = "The channel-to-channel linking is disabled"]
                pub const ELINK_0: u16 = 0;
                #[doc = "The channel-to-channel linking is enabled"]
                pub const ELINK_1: u16 = 0x01;
            }
        }
    }
    #[doc = "TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub mod TCD_DLASTSGA {
        #[doc = "DLASTSGA"]
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
                #[doc = "The channel is not explicitly started."]
                pub const START_0: u16 = 0;
                #[doc = "The channel is explicitly started via a software initiated service request."]
                pub const START_1: u16 = 0x01;
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
                pub const INTMAJOR_0: u16 = 0;
                #[doc = "The end-of-major loop interrupt is enabled."]
                pub const INTMAJOR_1: u16 = 0x01;
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
                pub const INTHALF_0: u16 = 0;
                #[doc = "The half-point interrupt is enabled."]
                pub const INTHALF_1: u16 = 0x01;
            }
        }
        #[doc = "Disable Request"]
        pub mod DREQ {
            pub const offset: u16 = 3;
            pub const mask: u16 = 0x01 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "The channel's ERQ bit is not affected."]
                pub const DREQ_0: u16 = 0;
                #[doc = "The channel's ERQ bit is cleared when the major loop is complete."]
                pub const DREQ_1: u16 = 0x01;
            }
        }
        #[doc = "Enable Scatter/Gather Processing"]
        pub mod ESG {
            pub const offset: u16 = 4;
            pub const mask: u16 = 0x01 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "The current channel's TCD is normal format."]
                pub const ESG_0: u16 = 0;
                #[doc = "The current channel's TCD specifies a scatter gather format. The DLASTSGA field provides a memory pointer to the next TCD to be loaded into this channel after the major loop completes its execution."]
                pub const ESG_1: u16 = 0x01;
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
                pub const MAJORELINK_0: u16 = 0;
                #[doc = "The channel-to-channel linking is enabled."]
                pub const MAJORELINK_1: u16 = 0x01;
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
                #[doc = "No eDMA engine stalls."]
                pub const BWC_0: u16 = 0;
                #[doc = "eDMA engine stalls for 4 cycles after each R/W."]
                pub const BWC_2: u16 = 0x02;
                #[doc = "eDMA engine stalls for 8 cycles after each R/W."]
                pub const BWC_3: u16 = 0x03;
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
                #[doc = "The channel-to-channel linking is disabled"]
                pub const ELINK_0: u16 = 0;
                #[doc = "The channel-to-channel linking is enabled"]
                pub const ELINK_1: u16 = 0x01;
            }
        }
    }
}
