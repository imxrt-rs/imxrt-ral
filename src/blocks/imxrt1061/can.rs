#[doc = "FLEXCAN"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "Module Configuration Register"]
    pub MCR: crate::RWRegister<u32>,
    #[doc = "Control 1 Register"]
    pub CTRL1: crate::RWRegister<u32>,
    #[doc = "Free Running Timer Register"]
    pub TIMER: crate::RWRegister<u32>,
    _reserved0: [u8; 0x04],
    #[doc = "Rx Mailboxes Global Mask Register"]
    pub RXMGMASK: crate::RWRegister<u32>,
    #[doc = "Rx Buffer 14 Mask Register"]
    pub RX14MASK: crate::RWRegister<u32>,
    #[doc = "Rx Buffer 15 Mask Register"]
    pub RX15MASK: crate::RWRegister<u32>,
    #[doc = "Error Counter Register"]
    pub ECR: crate::RWRegister<u32>,
    #[doc = "Error and Status 1 Register"]
    pub ESR1: crate::RWRegister<u32>,
    #[doc = "Interrupt Masks 2 Register"]
    pub IMASK2: crate::RWRegister<u32>,
    #[doc = "Interrupt Masks 1 Register"]
    pub IMASK1: crate::RWRegister<u32>,
    #[doc = "Interrupt Flags 2 Register"]
    pub IFLAG2: crate::RWRegister<u32>,
    #[doc = "Interrupt Flags 1 Register"]
    pub IFLAG1: crate::RWRegister<u32>,
    #[doc = "Control 2 Register"]
    pub CTRL2: crate::RWRegister<u32>,
    #[doc = "Error and Status 2 Register"]
    pub ESR2: crate::RORegister<u32>,
    _reserved1: [u8; 0x08],
    #[doc = "CRC Register"]
    pub CRCR: crate::RORegister<u32>,
    #[doc = "Rx FIFO Global Mask Register"]
    pub RXFGMASK: crate::RWRegister<u32>,
    #[doc = "Rx FIFO Information Register"]
    pub RXFIR: crate::RORegister<u32>,
    _reserved2: [u8; 0x08],
    #[doc = "Debug 1 register"]
    pub DBG1: crate::RORegister<u32>,
    #[doc = "Debug 2 register"]
    pub DBG2: crate::RORegister<u32>,
    _reserved3: [u8; 0x0820],
    #[doc = "Rx Individual Mask Registers"]
    pub RXIMR: [crate::RWRegister<u32>; 64usize],
    _reserved4: [u8; 0x60],
    #[doc = "Glitch Filter Width Registers"]
    pub GFWR: crate::RWRegister<u32>,
}
#[doc = "Module Configuration Register"]
pub mod MCR {
    #[doc = "This 7-bit field defines the number of the last Message Buffers that will take part in the matching and arbitration processes"]
    pub mod MAXMB {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x7f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This 2-bit field identifies the format of the elements of the Rx FIFO filter table, as shown below"]
    pub mod IDAM {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Format A One full ID (standard or extended) per ID filter Table element."]
            pub const IDAM_0: u32 = 0;
            #[doc = "Format B Two full standard IDs or two partial 14-bit extended IDs per ID filter Table element."]
            pub const IDAM_1: u32 = 0x01;
            #[doc = "Format C Four partial 8-bit IDs (standard or extended) per ID filter Table element."]
            pub const IDAM_2: u32 = 0x02;
            #[doc = "Format D All frames rejected."]
            pub const IDAM_3: u32 = 0x03;
        }
    }
    #[doc = "This bit is supplied for backwards compatibility reasons"]
    pub mod AEN {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Abort disabled"]
            pub const AEN_0: u32 = 0;
            #[doc = "Abort enabled"]
            pub const AEN_1: u32 = 0x01;
        }
    }
    #[doc = "This bit is provided for backwards compatibility reasons"]
    pub mod LPRIOEN {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Local Priority disabled"]
            pub const LPRIOEN_0: u32 = 0;
            #[doc = "Local Priority enabled"]
            pub const LPRIOEN_1: u32 = 0x01;
        }
    }
    #[doc = "This bit indicates whether Rx matching process will be based either on individual masking and queue or on masking scheme with RXMGMASK, RX14MASK and RX15MASK, RXFGMASK"]
    pub mod IRMQ {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Individual Rx masking and queue feature are disabled.For backward compatibility, the reading of C/S word locks the MB even if it is EMPTY."]
            pub const IRMQ_0: u32 = 0;
            #[doc = "Individual Rx masking and queue feature are enabled."]
            pub const IRMQ_1: u32 = 0x01;
        }
    }
    #[doc = "This bit defines whether FlexCAN is allowed to receive frames transmitted by itself"]
    pub mod SRXDIS {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Self reception enabled"]
            pub const SRXDIS_0: u32 = 0;
            #[doc = "Self reception disabled"]
            pub const SRXDIS_1: u32 = 0x01;
        }
    }
    #[doc = "This bit defines whether the integrated low-pass filter is applied to protect the FLEXCAN_RX input from spurious wake up"]
    pub mod WAKSRC {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "FLEXCAN uses the unfiltered FLEXCAN_RX input to detect recessive to dominant edges on the CAN bus."]
            pub const WAKSRC_0: u32 = 0;
            #[doc = "FLEXCAN uses the filtered FLEXCAN_RX input to detect recessive to dominant edges on the CAN bus"]
            pub const WAKSRC_1: u32 = 0x01;
        }
    }
    #[doc = "This read-only bit indicates that FLEXCAN is either in Disable Mode or Stop Mode"]
    pub mod LPMACK {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "FLEXCAN not in any of the low power modes"]
            pub const LPMACK_0: u32 = 0;
            #[doc = "FLEXCAN is either in Disable Mode, or Stop mode"]
            pub const LPMACK_1: u32 = 0x01;
        }
    }
    #[doc = "When asserted, this bit enables the generation of the TWRN_INT and RWRN_INT flags in the Error and Status Register"]
    pub mod WRNEN {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "TWRN_INT and RWRN_INT bits are zero, independent of the values in the error counters."]
            pub const WRNEN_0: u32 = 0;
            #[doc = "TWRN_INT and RWRN_INT bits are set when the respective error counter transition from <96 to >= 96."]
            pub const WRNEN_1: u32 = 0x01;
        }
    }
    #[doc = "This bit enables the Self Wake Up feature when FLEXCAN is in Stop Mode"]
    pub mod SLFWAK {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "FLEXCAN Self Wake Up feature is disabled"]
            pub const SLFWAK_0: u32 = 0;
            #[doc = "FLEXCAN Self Wake Up feature is enabled"]
            pub const SLFWAK_1: u32 = 0x01;
        }
    }
    #[doc = "This bit configures some of the FLEXCAN registers to be either in Supervisor or User Mode"]
    pub mod SUPV {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "FlexCAN is in User Mode. Affected registers allow both Supervisor and Unrestricted accesses"]
            pub const SUPV_0: u32 = 0;
            #[doc = "FlexCAN is in Supervisor Mode. Affected registers allow only Supervisor access. Unrestricted access behaves as though the access was done to an unimplemented register location"]
            pub const SUPV_1: u32 = 0x01;
        }
    }
    #[doc = "This read-only bit indicates that FLEXCAN is in Freeze Mode and its prescaler is stopped"]
    pub mod FRZACK {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "FLEXCAN not in Freeze Mode, prescaler running"]
            pub const FRZACK_0: u32 = 0;
            #[doc = "FLEXCAN in Freeze Mode, prescaler stopped"]
            pub const FRZACK_1: u32 = 0x01;
        }
    }
    #[doc = "When this bit is asserted, FlexCAN resets its internal state machines and some of the memory mapped registers"]
    pub mod SOFTRST {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No reset request"]
            pub const SOFTRST_0: u32 = 0;
            #[doc = "Reset the registers"]
            pub const SOFTRST_1: u32 = 0x01;
        }
    }
    #[doc = "This bit enables the Wake Up Interrupt generation."]
    pub mod WAKMSK {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Wake Up Interrupt is disabled"]
            pub const WAKMSK_0: u32 = 0;
            #[doc = "Wake Up Interrupt is enabled"]
            pub const WAKMSK_1: u32 = 0x01;
        }
    }
    #[doc = "This read-only bit indicates that FLEXCAN is either in Disable Mode, Stop Mode or Freeze Mode"]
    pub mod NOTRDY {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "FLEXCAN module is either in Normal Mode, Listen-Only Mode or Loop-Back Mode"]
            pub const NOTRDY_0: u32 = 0;
            #[doc = "FLEXCAN module is either in Disable Mode, Stop Mode or Freeze Mode"]
            pub const NOTRDY_1: u32 = 0x01;
        }
    }
    #[doc = "Assertion of this bit puts the FLEXCAN module into Freeze Mode"]
    pub mod HALT {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No Freeze Mode request."]
            pub const HALT_0: u32 = 0;
            #[doc = "Enters Freeze Mode if the FRZ bit is asserted."]
            pub const HALT_1: u32 = 0x01;
        }
    }
    #[doc = "This bit controls whether the Rx FIFO feature is enabled or not"]
    pub mod RFEN {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "FIFO not enabled"]
            pub const RFEN_0: u32 = 0;
            #[doc = "FIFO enabled"]
            pub const RFEN_1: u32 = 0x01;
        }
    }
    #[doc = "The FRZ bit specifies the FLEXCAN behavior when the HALT bit in the MCR Register is set or when Debug Mode is requested at Arm level"]
    pub mod FRZ {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not enabled to enter Freeze Mode"]
            pub const FRZ_0: u32 = 0;
            #[doc = "Enabled to enter Freeze Mode"]
            pub const FRZ_1: u32 = 0x01;
        }
    }
    #[doc = "This bit controls whether FLEXCAN is enabled or not"]
    pub mod MDIS {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Enable the FLEXCAN module"]
            pub const MDIS_0: u32 = 0;
            #[doc = "Disable the FLEXCAN module"]
            pub const MDIS_1: u32 = 0x01;
        }
    }
}
#[doc = "Control 1 Register"]
pub mod CTRL1 {
    #[doc = "This 3-bit field defines the length of the Propagation Segment in the bit time"]
    pub mod PROPSEG {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bit configures FLEXCAN to operate in Listen Only Mode"]
    pub mod LOM {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Listen Only Mode is deactivated"]
            pub const LOM_0: u32 = 0;
            #[doc = "FLEXCAN module operates in Listen Only Mode"]
            pub const LOM_1: u32 = 0x01;
        }
    }
    #[doc = "This bit defines the ordering mechanism for Message Buffer transmission"]
    pub mod LBUF {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Buffer with highest priority is transmitted first"]
            pub const LBUF_0: u32 = 0;
            #[doc = "Lowest number buffer is transmitted first"]
            pub const LBUF_1: u32 = 0x01;
        }
    }
    #[doc = "This bit enables a mechanism that resets the free-running timer each time a message is received in Message Buffer 0"]
    pub mod TSYN {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Timer Sync feature disabled"]
            pub const TSYN_0: u32 = 0;
            #[doc = "Timer Sync feature enabled"]
            pub const TSYN_1: u32 = 0x01;
        }
    }
    #[doc = "This bit defines how FLEXCAN recovers from Bus Off state"]
    pub mod BOFFREC {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Automatic recovering from Bus Off state enabled, according to CAN Spec 2.0 part B"]
            pub const BOFFREC_0: u32 = 0;
            #[doc = "Automatic recovering from Bus Off state disabled"]
            pub const BOFFREC_1: u32 = 0x01;
        }
    }
    #[doc = "This bit defines the sampling mode of CAN bits at the FLEXCAN_RX"]
    pub mod SMP {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Just one sample is used to determine the bit value"]
            pub const SMP_0: u32 = 0;
            #[doc = "Three samples are used to determine the value of the received bit: the regular one (sample point) and 2 preceding samples, a majority rule is used"]
            pub const SMP_1: u32 = 0x01;
        }
    }
    #[doc = "This bit provides a mask for the Rx Warning Interrupt associated with the RWRN_INT flag in the Error and Status Register"]
    pub mod RWRNMSK {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Rx Warning Interrupt disabled"]
            pub const RWRNMSK_0: u32 = 0;
            #[doc = "Rx Warning Interrupt enabled"]
            pub const RWRNMSK_1: u32 = 0x01;
        }
    }
    #[doc = "This bit provides a mask for the Tx Warning Interrupt associated with the TWRN_INT flag in the Error and Status Register"]
    pub mod TWRNMSK {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Tx Warning Interrupt disabled"]
            pub const TWRNMSK_0: u32 = 0;
            #[doc = "Tx Warning Interrupt enabled"]
            pub const TWRNMSK_1: u32 = 0x01;
        }
    }
    #[doc = "This bit configures FlexCAN to operate in Loop-Back Mode"]
    pub mod LPB {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Loop Back disabled"]
            pub const LPB_0: u32 = 0;
            #[doc = "Loop Back enabled"]
            pub const LPB_1: u32 = 0x01;
        }
    }
    #[doc = "This bit provides a mask for the Error Interrupt."]
    pub mod ERRMSK {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Error interrupt disabled"]
            pub const ERRMSK_0: u32 = 0;
            #[doc = "Error interrupt enabled"]
            pub const ERRMSK_1: u32 = 0x01;
        }
    }
    #[doc = "This bit provides a mask for the Bus Off Interrupt."]
    pub mod BOFFMSK {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Bus Off interrupt disabled"]
            pub const BOFFMSK_0: u32 = 0;
            #[doc = "Bus Off interrupt enabled"]
            pub const BOFFMSK_1: u32 = 0x01;
        }
    }
    #[doc = "This 3-bit field defines the length of Phase Buffer Segment 2 in the bit time"]
    pub mod PSEG2 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This 3-bit field defines the length of Phase Buffer Segment 1 in the bit time"]
    pub mod PSEG1 {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This 2-bit field defines the maximum number of time quanta One time quantum is equal to the Sclock period"]
    pub mod RJW {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This 8-bit field defines the ratio between the PE clock frequency and the Serial Clock (Sclock) frequency"]
    pub mod PRESDIV {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Free Running Timer Register"]
pub mod TIMER {
    #[doc = "TIMER"]
    pub mod TIMER {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Rx Mailboxes Global Mask Register"]
pub mod RXMGMASK {
    #[doc = "These bits mask the Mailbox filter bits as shown in the figure above"]
    pub mod MG {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "the corresponding bit in the filter is \"don't care\""]
            pub const MG_0: u32 = 0;
            #[doc = "The corresponding bit in the filter is checked against the one received"]
            pub const MG_1: u32 = 0x01;
        }
    }
}
#[doc = "Rx Buffer 14 Mask Register"]
pub mod RX14MASK {
    #[doc = "These bits mask Mailbox 14 filter bits in the same fashion as RXMGMASK masks other Mailboxes filters (see RXMGMASKRx Mailboxes Global Mask Register )"]
    pub mod RX14M {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "the corresponding bit in the filter is \"don't care\""]
            pub const RX14M_0: u32 = 0;
            #[doc = "The corresponding bit in the filter is checked"]
            pub const RX14M_1: u32 = 0x01;
        }
    }
}
#[doc = "Rx Buffer 15 Mask Register"]
pub mod RX15MASK {
    #[doc = "These bits mask Mailbox 15 filter bits in the same fashion as RXMGMASK masks other Mailboxes filters (see RXMGMASKRx Mailboxes Global Mask Register )"]
    pub mod RX15M {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "the corresponding bit in the filter is \"don't care\""]
            pub const RX15M_0: u32 = 0;
            #[doc = "The corresponding bit in the filter is checked"]
            pub const RX15M_1: u32 = 0x01;
        }
    }
}
#[doc = "Error Counter Register"]
pub mod ECR {
    #[doc = "Tx_Err_Counter"]
    pub mod TX_ERR_COUNTER {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Rx_Err_Counter"]
    pub mod RX_ERR_COUNTER {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Error and Status 1 Register"]
pub mod ESR1 {
    #[doc = "When FLEXCAN is Stop Mode and a recessive to dominant transition is detected on the CAN bus and if the WAK_MSK bit in the MCR Register is set, an interrupt is generated to the Arm"]
    pub mod WAKINT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No such occurrence"]
            pub const WAKINT_0: u32 = 0;
            #[doc = "Indicates a recessive to dominant transition received on the CAN bus when the FLEXCAN module is in Stop Mode"]
            pub const WAKINT_1: u32 = 0x01;
        }
    }
    #[doc = "This bit indicates that at least one of the Error Bits (bits 15-10) is set"]
    pub mod ERRINT {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No such occurrence"]
            pub const ERRINT_0: u32 = 0;
            #[doc = "Indicates setting of any Error Bit in the Error and Status Register"]
            pub const ERRINT_1: u32 = 0x01;
        }
    }
    #[doc = "This bit is set when FLEXCAN enters 'Bus Off' state"]
    pub mod BOFFINT {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No such occurrence"]
            pub const BOFFINT_0: u32 = 0;
            #[doc = "FLEXCAN module entered 'Bus Off' state"]
            pub const BOFFINT_1: u32 = 0x01;
        }
    }
    #[doc = "This bit indicates if FlexCAN is receiving a message. Refer to ."]
    pub mod RX {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "FLEXCAN is receiving a message"]
            pub const RX_0: u32 = 0;
            #[doc = "FLEXCAN is transmitting a message"]
            pub const RX_1: u32 = 0x01;
        }
    }
    #[doc = "If the LOM bit in the Control Register is asserted, after some delay that depends on the CAN bit timing the FLT_CONF field will indicate \"Error Passive\""]
    pub mod FLTCONF {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Error Active"]
            pub const FLTCONF_0: u32 = 0;
            #[doc = "Error Passive"]
            pub const FLTCONF_1: u32 = 0x01;
            #[doc = "Bus off"]
            pub const FLTCONF_2: u32 = 0x02;
        }
    }
    #[doc = "This bit indicates if FLEXCAN is transmitting a message.Refer to ."]
    pub mod TX {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "FLEXCAN is receiving a message"]
            pub const TX_0: u32 = 0;
            #[doc = "FLEXCAN is transmitting a message"]
            pub const TX_1: u32 = 0x01;
        }
    }
    #[doc = "This bit indicates when CAN bus is in IDLE state.Refer to ."]
    pub mod IDLE {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No such occurrence"]
            pub const IDLE_0: u32 = 0;
            #[doc = "CAN bus is now IDLE"]
            pub const IDLE_1: u32 = 0x01;
        }
    }
    #[doc = "This bit indicates when repetitive errors are occurring during message reception."]
    pub mod RXWRN {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No such occurrence"]
            pub const RXWRN_0: u32 = 0;
            #[doc = "Rx_Err_Counter >= 96"]
            pub const RXWRN_1: u32 = 0x01;
        }
    }
    #[doc = "This bit indicates when repetitive errors are occurring during message transmission."]
    pub mod TXWRN {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No such occurrence"]
            pub const TXWRN_0: u32 = 0;
            #[doc = "TX_Err_Counter >= 96"]
            pub const TXWRN_1: u32 = 0x01;
        }
    }
    #[doc = "This bit indicates that a Stuffing Error has been detected."]
    pub mod STFERR {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No such occurrence."]
            pub const STFERR_0: u32 = 0;
            #[doc = "A Stuffing Error occurred since last read of this register."]
            pub const STFERR_1: u32 = 0x01;
        }
    }
    #[doc = "This bit indicates that a Form Error has been detected by the receiver node, i"]
    pub mod FRMERR {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No such occurrence"]
            pub const FRMERR_0: u32 = 0;
            #[doc = "A Form Error occurred since last read of this register"]
            pub const FRMERR_1: u32 = 0x01;
        }
    }
    #[doc = "This bit indicates that a CRC Error has been detected by the receiver node, i"]
    pub mod CRCERR {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No such occurrence"]
            pub const CRCERR_0: u32 = 0;
            #[doc = "A CRC error occurred since last read of this register."]
            pub const CRCERR_1: u32 = 0x01;
        }
    }
    #[doc = "This bit indicates that an Acknowledge Error has been detected by the transmitter node, i"]
    pub mod ACKERR {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No such occurrence"]
            pub const ACKERR_0: u32 = 0;
            #[doc = "An ACK error occurred since last read of this register"]
            pub const ACKERR_1: u32 = 0x01;
        }
    }
    #[doc = "This bit indicates when an inconsistency occurs between the transmitted and the received bit in a message"]
    pub mod BIT0ERR {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No such occurrence"]
            pub const BIT0ERR_0: u32 = 0;
            #[doc = "At least one bit sent as dominant is received as recessive"]
            pub const BIT0ERR_1: u32 = 0x01;
        }
    }
    #[doc = "This bit indicates when an inconsistency occurs between the transmitted and the received bit in a message"]
    pub mod BIT1ERR {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No such occurrence"]
            pub const BIT1ERR_0: u32 = 0;
            #[doc = "At least one bit sent as recessive is received as dominant"]
            pub const BIT1ERR_1: u32 = 0x01;
        }
    }
    #[doc = "If the WRN_EN bit in MCR is asserted, the RWRN_INT bit is set when the RX_WRN flag transition from '0' to '1', meaning that the Rx error counters reached 96"]
    pub mod RWRNINT {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No such occurrence"]
            pub const RWRNINT_0: u32 = 0;
            #[doc = "The Rx error counter transition from < 96 to >= 96"]
            pub const RWRNINT_1: u32 = 0x01;
        }
    }
    #[doc = "If the WRN_EN bit in MCR is asserted, the TWRN_INT bit is set when the TX_WRN flag transition from '0' to '1', meaning that the Tx error counter reached 96"]
    pub mod TWRNINT {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No such occurrence"]
            pub const TWRNINT_0: u32 = 0;
            #[doc = "The Tx error counter transition from < 96 to >= 96"]
            pub const TWRNINT_1: u32 = 0x01;
        }
    }
    #[doc = "This read-only flag indicates whether the FlexCAN is synchronized to the CAN bus and able to participate in the communication process"]
    pub mod SYNCH {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "FlexCAN is not synchronized to the CAN bus"]
            pub const SYNCH_0: u32 = 0;
            #[doc = "FlexCAN is synchronized to the CAN bus"]
            pub const SYNCH_1: u32 = 0x01;
        }
    }
}
#[doc = "Interrupt Masks 2 Register"]
pub mod IMASK2 {
    #[doc = "Each bit enables or disables the respective FLEXCAN Message Buffer (MB32 to MB63) Interrupt"]
    pub mod BUFHM {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The corresponding buffer Interrupt is disabled"]
            pub const BUFHM_0: u32 = 0;
            #[doc = "The corresponding buffer Interrupt is enabled"]
            pub const BUFHM_1: u32 = 0x01;
        }
    }
}
#[doc = "Interrupt Masks 1 Register"]
pub mod IMASK1 {
    #[doc = "Each bit enables or disables the respective FLEXCAN Message Buffer (MB0 to MB31) Interrupt"]
    pub mod BUFLM {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The corresponding buffer Interrupt is disabled"]
            pub const BUFLM_0: u32 = 0;
            #[doc = "The corresponding buffer Interrupt is enabled"]
            pub const BUFLM_1: u32 = 0x01;
        }
    }
}
#[doc = "Interrupt Flags 2 Register"]
pub mod IFLAG2 {
    #[doc = "Each bit flags the respective FLEXCAN Message Buffer (MB32 to MB63) interrupt."]
    pub mod BUFHI {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No such occurrence"]
            pub const BUFHI_0: u32 = 0;
            #[doc = "The corresponding buffer has successfully completed transmission or reception"]
            pub const BUFHI_1: u32 = 0x01;
        }
    }
}
#[doc = "Interrupt Flags 1 Register"]
pub mod IFLAG1 {
    #[doc = "If the Rx FIFO is not enabled, these bits flag the interrupts for MB0 to MB4"]
    pub mod BUF4TO0I {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No such occurrence"]
            pub const BUF4TO0I_0: u32 = 0;
            #[doc = "Corresponding MB completed transmission/reception"]
            pub const BUF4TO0I_1: u32 = 0x01;
        }
    }
    #[doc = "If the Rx FIFO is not enabled, this bit flags the interrupt for MB5"]
    pub mod BUF5I {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No such occurrence"]
            pub const BUF5I_0: u32 = 0;
            #[doc = "MB5 completed transmission/reception or frames available in the FIFO"]
            pub const BUF5I_1: u32 = 0x01;
        }
    }
    #[doc = "If the Rx FIFO is not enabled, this bit flags the interrupt for MB6"]
    pub mod BUF6I {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No such occurrence"]
            pub const BUF6I_0: u32 = 0;
            #[doc = "MB6 completed transmission/reception or FIFO almost full"]
            pub const BUF6I_1: u32 = 0x01;
        }
    }
    #[doc = "If the Rx FIFO is not enabled, this bit flags the interrupt for MB7"]
    pub mod BUF7I {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No such occurrence"]
            pub const BUF7I_0: u32 = 0;
            #[doc = "MB7 completed transmission/reception or FIFO overflow"]
            pub const BUF7I_1: u32 = 0x01;
        }
    }
    #[doc = "Each bit flags the respective FLEXCAN Message Buffer (MB8 to MB31) interrupt."]
    pub mod BUF31TO8I {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No such occurrence"]
            pub const BUF31TO8I_0: u32 = 0;
            #[doc = "The corresponding MB has successfully completed transmission or reception"]
            pub const BUF31TO8I_1: u32 = 0x01;
        }
    }
}
#[doc = "Control 2 Register"]
pub mod CTRL2 {
    #[doc = "This bit controls the comparison of IDE and RTR bits within Rx Mailboxes filters with their corresponding bits in the incoming frame by the matching process"]
    pub mod EACEN {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Rx Mailbox filter's IDE bit is always compared and RTR is never compared despite mask bits."]
            pub const EACEN_0: u32 = 0;
            #[doc = "Enables the comparison of both Rx Mailbox filter's IDE and RTR bit with their corresponding bits within the incoming frame. Mask bits do apply."]
            pub const EACEN_1: u32 = 0x01;
        }
    }
    #[doc = "If this bit is asserted Remote Request Frame is submitted to a matching process and stored in the corresponding Message Buffer in the same fashion of a Data Frame"]
    pub mod RRS {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Remote Response Frame is generated"]
            pub const RRS_0: u32 = 0;
            #[doc = "Remote Request Frame is stored"]
            pub const RRS_1: u32 = 0x01;
        }
    }
    #[doc = "If this bit is set the matching process starts from the Mailboxes and if no match occurs the matching continues on the Rx FIFO"]
    pub mod MRP {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Matching starts from Rx FIFO and continues on Mailboxes"]
            pub const MRP_0: u32 = 0;
            #[doc = "Matching starts from Mailboxes and continues on Rx FIFO"]
            pub const MRP_1: u32 = 0x01;
        }
    }
    #[doc = "This 5-bit field indicates how many CAN bits the Tx arbitration process start point can be delayed from the first bit of CRC field on CAN bus"]
    pub mod TASD {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This 4-bit field defines the number of Rx FIFO filters according to"]
    pub mod RFFN {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enable unrestricted write access to FlexCAN memory in Freeze mode"]
    pub mod WRMFRZ {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Keep the write access restricted in some regions of FlexCAN memory"]
            pub const WRMFRZ_0: u32 = 0;
            #[doc = "Enable unrestricted write access to FlexCAN memory"]
            pub const WRMFRZ_1: u32 = 0x01;
        }
    }
}
#[doc = "Error and Status 2 Register"]
pub mod ESR2 {
    #[doc = "If ESR2\\[VPS\\] is asserted, this bit indicates whether there is any inactive Mailbox (CODE field is either 0b1000 or 0b0000)"]
    pub mod IMB {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "If ESR2\\[VPS\\] is asserted, the ESR2\\[LPTM\\] is not an inactive Mailbox."]
            pub const IMB_0: u32 = 0;
            #[doc = "If ESR2\\[VPS\\] is asserted, there is at least one inactive Mailbox. LPTM content is the number of the first one."]
            pub const IMB_1: u32 = 0x01;
        }
    }
    #[doc = "This bit indicates whether IMB and LPTM contents are currently valid or not"]
    pub mod VPS {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Contents of IMB and LPTM are invalid"]
            pub const VPS_0: u32 = 0;
            #[doc = "Contents of IMB and LPTM are valid"]
            pub const VPS_1: u32 = 0x01;
        }
    }
    #[doc = "If ESR2\\[VPS\\] is asserted, his 7-bit field indicates the lowest number inactive Mailbox (refer to IMB bit description)"]
    pub mod LPTM {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x7f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "CRC Register"]
pub mod CRCR {
    #[doc = "This field indicates the CRC value of the last message transmitted"]
    pub mod TXCRC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x7fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This field indicates the number of the Mailbox corresponding to the value in TXCRC field."]
    pub mod MBCRC {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x7f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Rx FIFO Global Mask Register"]
pub mod RXFGMASK {
    #[doc = "These bits mask the ID Filter Table elements bits in a perfect alignment"]
    pub mod FGM {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The corresponding bit in the filter is \"don't care\""]
            pub const FGM_0: u32 = 0;
            #[doc = "The corresponding bit in the filter is checked"]
            pub const FGM_1: u32 = 0x01;
        }
    }
}
#[doc = "Rx FIFO Information Register"]
pub mod RXFIR {
    #[doc = "This 9-bit field indicates which Identifier Acceptance Filter (see Rx FIFO Structure) was hit by the received message that is in the output of the Rx FIFO"]
    pub mod IDHIT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Debug 1 register"]
pub mod DBG1 {
    #[doc = "CAN Finite State Machine"]
    pub mod CFSM {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CAN Bit Number"]
    pub mod CBN {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Debug 2 register"]
pub mod DBG2 {
    #[doc = "Rx Matching Pointer"]
    pub mod RMP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x7f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Matching Process in Progress"]
    pub mod MPP {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No matching process ongoing."]
            pub const MPP_0: u32 = 0;
            #[doc = "Matching process is in progress."]
            pub const MPP_1: u32 = 0x01;
        }
    }
    #[doc = "Tx Arbitration Pointer"]
    pub mod TAP {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x7f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Arbitration Process in Progress"]
    pub mod APP {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No matching process ongoing."]
            pub const APP_0: u32 = 0;
            #[doc = "Matching process is in progress."]
            pub const APP_1: u32 = 0x01;
        }
    }
}
#[doc = "Rx Individual Mask Registers"]
pub mod RXIMR {
    #[doc = "These bits mask both Mailbox filter and Rx FIFO ID Filter Table element in distinct ways"]
    pub mod MI {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "the corresponding bit in the filter is \"don't care\""]
            pub const MI_0: u32 = 0;
            #[doc = "The corresponding bit in the filter is checked"]
            pub const MI_1: u32 = 0x01;
        }
    }
}
#[doc = "Glitch Filter Width Registers"]
pub mod GFWR {
    #[doc = "It determines the Glitch Filter Width"]
    pub mod GFWR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
