#[doc = "FlexSPI"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "Module Control 0"]
    pub MCR0: crate::RWRegister<u32>,
    #[doc = "Module Control 1"]
    pub MCR1: crate::RWRegister<u32>,
    #[doc = "Module Control 2"]
    pub MCR2: crate::RWRegister<u32>,
    #[doc = "AHB Bus Control"]
    pub AHBCR: crate::RWRegister<u32>,
    #[doc = "Interrupt Enable"]
    pub INTEN: crate::RWRegister<u32>,
    #[doc = "Interrupt"]
    pub INTR: crate::RWRegister<u32>,
    #[doc = "LUT Key"]
    pub LUTKEY: crate::RWRegister<u32>,
    #[doc = "LUT Control"]
    pub LUTCR: crate::RWRegister<u32>,
    #[doc = "AHB Receive Buffer 0 Control 0"]
    pub AHBRXBUF0CR0: crate::RWRegister<u32>,
    #[doc = "AHB Receive Buffer 1 Control 0"]
    pub AHBRXBUF1CR0: crate::RWRegister<u32>,
    #[doc = "AHB Receive Buffer 2 Control 0"]
    pub AHBRXBUF2CR0: crate::RWRegister<u32>,
    #[doc = "AHB Receive Buffer 3 Control 0"]
    pub AHBRXBUF3CR0: crate::RWRegister<u32>,
    #[doc = "AHB Receive Buffer 4 Control 0"]
    pub AHBRXBUF4CR0: crate::RWRegister<u32>,
    #[doc = "AHB Receive Buffer 5 Control 0"]
    pub AHBRXBUF5CR0: crate::RWRegister<u32>,
    #[doc = "AHB Receive Buffer 6 Control 0"]
    pub AHBRXBUF6CR0: crate::RWRegister<u32>,
    #[doc = "AHB Receive Buffer 7 Control 0"]
    pub AHBRXBUF7CR0: crate::RWRegister<u32>,
    _reserved0: [u8; 0x20],
    #[doc = "Flash Control 0"]
    pub FLSHA1CR0: crate::RWRegister<u32>,
    #[doc = "Flash Control 0"]
    pub FLSHA2CR0: crate::RWRegister<u32>,
    #[doc = "Flash Control 0"]
    pub FLSHB1CR0: crate::RWRegister<u32>,
    #[doc = "Flash Control 0"]
    pub FLSHB2CR0: crate::RWRegister<u32>,
    #[doc = "Flash Control 1"]
    pub FLSHCR1: [crate::RWRegister<u32>; 4usize],
    #[doc = "Flash Control 2"]
    pub FLSHCR2: [crate::RWRegister<u32>; 4usize],
    _reserved1: [u8; 0x04],
    #[doc = "Flash Control 4"]
    pub FLSHCR4: crate::RWRegister<u32>,
    _reserved2: [u8; 0x08],
    #[doc = "IP Control 0"]
    pub IPCR0: crate::RWRegister<u32>,
    #[doc = "IP Control 1"]
    pub IPCR1: crate::RWRegister<u32>,
    _reserved3: [u8; 0x08],
    #[doc = "IP Command"]
    pub IPCMD: crate::RWRegister<u32>,
    _reserved4: [u8; 0x04],
    #[doc = "IP Receive FIFO Control"]
    pub IPRXFCR: crate::RWRegister<u32>,
    #[doc = "IP Transmit FIFO Control"]
    pub IPTXFCR: crate::RWRegister<u32>,
    #[doc = "DLL Control 0"]
    pub DLLCR: [crate::RWRegister<u32>; 2usize],
    _reserved5: [u8; 0x18],
    #[doc = "Status 0"]
    pub STS0: crate::RORegister<u32>,
    #[doc = "Status 1"]
    pub STS1: crate::RORegister<u32>,
    #[doc = "Status 2"]
    pub STS2: crate::RORegister<u32>,
    #[doc = "AHB Suspend Status"]
    pub AHBSPNDSTS: crate::RORegister<u32>,
    #[doc = "IP Receive FIFO Status"]
    pub IPRXFSTS: crate::RORegister<u32>,
    #[doc = "IP Transmit FIFO Status"]
    pub IPTXFSTS: crate::RORegister<u32>,
    _reserved6: [u8; 0x08],
    #[doc = "IP Receive FIFO Data x"]
    pub RFDR: [crate::RORegister<u32>; 32usize],
    #[doc = "IP TX FIFO Data x"]
    pub TFDR: [crate::WORegister<u32>; 32usize],
    #[doc = "Lookup Table x"]
    pub LUT: [crate::RWRegister<u32>; 128usize],
    _reserved7: [u8; 0x40],
    #[doc = "Receive Buffer Start Address of Region 0"]
    pub AHBBUFREGIONSTART0: crate::RWRegister<u32>,
    #[doc = "Receive Buffer Region 0 End Address"]
    pub AHBBUFREGIONEND0: crate::RWRegister<u32>,
    #[doc = "Receive Buffer Start Address of Region 1"]
    pub AHBBUFREGIONSTART1: crate::RWRegister<u32>,
    #[doc = "Receive Buffer Region 1 End Address"]
    pub AHBBUFREGIONEND1: crate::RWRegister<u32>,
    #[doc = "Receive Buffer Start Address of Region 2"]
    pub AHBBUFREGIONSTART2: crate::RWRegister<u32>,
    #[doc = "Receive Buffer Region 2 End Address"]
    pub AHBBUFREGIONEND2: crate::RWRegister<u32>,
    #[doc = "Receive Buffer Start Address of Region 3"]
    pub AHBBUFREGIONSTART3: crate::RWRegister<u32>,
    #[doc = "Receive Buffer Region 3 End Address"]
    pub AHBBUFREGIONEND3: crate::RWRegister<u32>,
}
#[doc = "Module Control 0"]
pub mod MCR0 {
    #[doc = "Software Reset"]
    pub mod SWRESET {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No impact"]
            pub const VAL0: u32 = 0;
            #[doc = "Software reset"]
            pub const VAL1: u32 = 0x01;
        }
    }
    #[doc = "Module Disable"]
    pub mod MDIS {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No impact"]
            pub const VAL0: u32 = 0;
            #[doc = "Module disable"]
            pub const VAL1: u32 = 0x01;
        }
    }
    #[doc = "Sample Clock Source for Flash Reading"]
    pub mod RXCLKSRC {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Dummy Read strobe generated by FlexSPI Controller and loopback internally."]
            pub const RXCLKSRC_0: u32 = 0;
            #[doc = "Dummy Read strobe generated by FlexSPI Controller and loopback from DQS pad."]
            pub const RXCLKSRC_1: u32 = 0x01;
            #[doc = "Flash provided Read strobe and input from DQS pad"]
            pub const RXCLKSRC_3: u32 = 0x03;
        }
    }
    #[doc = "AHB Read Access to IP Receive FIFO Enable"]
    pub mod ARDFEN {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "IP RX FIFO should be read by IP Bus. AHB Bus read access to IP RX FIFO memory space will get bus error response."]
            pub const ARDFEN_0: u32 = 0;
            #[doc = "IP RX FIFO should be read by AHB Bus. IP Bus read access to IP RX FIFO memory space will always return data zero but no bus error response."]
            pub const ARDFEN_1: u32 = 0x01;
        }
    }
    #[doc = "AHB Write Access to IP Transmit FIFO Enable"]
    pub mod ATDFEN {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "IP TX FIFO should be written by IP Bus. AHB Bus write access to IP TX FIFO memory space will get bus error response."]
            pub const ATDFEN_0: u32 = 0;
            #[doc = "IP TX FIFO should be written by AHB Bus. IP Bus write access to IP TX FIFO memory space will be ignored but no bus error response."]
            pub const ATDFEN_1: u32 = 0x01;
        }
    }
    #[doc = "Serial Root Clock Divider"]
    pub mod SERCLKDIV {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Divided by 1"]
            pub const SERCLKDIV_0: u32 = 0;
            #[doc = "Divided by 2"]
            pub const SERCLKDIV_1: u32 = 0x01;
            #[doc = "Divided by 3"]
            pub const SERCLKDIV_2: u32 = 0x02;
            #[doc = "Divided by 4"]
            pub const SERCLKDIV_3: u32 = 0x03;
            #[doc = "Divided by 5"]
            pub const SERCLKDIV_4: u32 = 0x04;
            #[doc = "Divided by 6"]
            pub const SERCLKDIV_5: u32 = 0x05;
            #[doc = "Divided by 7"]
            pub const SERCLKDIV_6: u32 = 0x06;
            #[doc = "Divided by 8"]
            pub const SERCLKDIV_7: u32 = 0x07;
        }
    }
    #[doc = "Half Speed Serial Flash Memory Access Enable"]
    pub mod HSEN {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable divide by 2 of serial flash clock for half speed commands."]
            pub const HSEN_0: u32 = 0;
            #[doc = "Enable divide by 2 of serial flash clock for half speed commands."]
            pub const HSEN_1: u32 = 0x01;
        }
    }
    #[doc = "Doze Mode Enable"]
    pub mod DOZEEN {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Doze mode support disabled. AHB clock and serial clock will not be gated off when there is doze mode request from system."]
            pub const DOZEEN_0: u32 = 0;
            #[doc = "Doze mode support enabled. AHB clock and serial clock will be gated off when there is doze mode request from system."]
            pub const DOZEEN_1: u32 = 0x01;
        }
    }
    #[doc = "Combination Mode Enable"]
    pub mod COMBINATIONEN {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable."]
            pub const COMBINATIONEN_0: u32 = 0;
            #[doc = "Enable."]
            pub const COMBINATIONEN_1: u32 = 0x01;
        }
    }
    #[doc = "SCLK Free-running Enable"]
    pub mod SCKFREERUNEN {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable."]
            pub const SCKFREERUNEN_0: u32 = 0;
            #[doc = "Enable."]
            pub const SCKFREERUNEN_1: u32 = 0x01;
        }
    }
    #[doc = "Timeout Wait Cycle for IP Command Grant"]
    pub mod IPGRANTWAIT {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Timeouts Wait Cycle for AHB command Grant"]
    pub mod AHBGRANTWAIT {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Module Control 1"]
pub mod MCR1 {
    #[doc = "AHB Read/Write access to Serial Flash Memory space will timeout if not data received from Flash or data not transmitted after AHBBUSWAIT * 1024 ahb clock cycles, AHB Bus will get an error response"]
    pub mod AHBBUSWAIT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Command Sequence Execution will timeout and abort after SEQWAIT * 1024 Serial Root Clock cycles"]
    pub mod SEQWAIT {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Module Control 2"]
pub mod MCR2 {
    #[doc = "Clear AHB Buffer"]
    pub mod CLRAHBBUFOPT {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "AHB RX/TX Buffer will not be cleaned automatically when FlexSPI return Stop mode ACK."]
            pub const CLRAHBBUFOPT_0: u32 = 0;
            #[doc = "AHB RX/TX Buffer will be cleaned automatically when FlexSPI return Stop mode ACK."]
            pub const CLRAHBBUFOPT_1: u32 = 0x01;
        }
    }
    #[doc = "Same Device Enable"]
    pub mod SAMEDEVICEEN {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "In Individual mode, FLSHA1CRx/FLSHA2CRx/FLSHB1CRx/FLSHB2CRx register setting will be applied to Flash A1/A2/B1/B2 separately. In Parallel mode, FLSHA1CRx register setting will be applied to Flash A1 and B1, FLSHA2CRx register setting will be applied to Flash A2 and B2. FLSHB1CRx/FLSHB2CRx register settings will be ignored."]
            pub const SAMEDEVICEEN_0: u32 = 0;
            #[doc = "FLSHA1CR0/FLSHA1CR1/FLSHA1CR2 register settings will be applied to Flash A1/A2/B1/B2. FLSHA2CRx/FLSHB1CRx/FLSHB2CRx will be ignored."]
            pub const SAMEDEVICEEN_1: u32 = 0x01;
        }
    }
    #[doc = "SCLK Port B Differential Output"]
    pub mod SCKBDIFFOPT {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "B_SCLK pad is used as port B SCLK clock output. Port B flash access is available."]
            pub const SCKBDIFFOPT_0: u32 = 0;
            #[doc = "B_SCLK pad is used as port A SCLK inverted clock output (Differential clock to A_SCLK). Port B flash access is not available."]
            pub const SCKBDIFFOPT_1: u32 = 0x01;
        }
    }
    #[doc = "Port B Receiver Clock Source"]
    pub mod RXCLKSRC_B {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Dummy read strobe that FlexSPI generates, looped back internally."]
            pub const VAL0: u32 = 0;
            #[doc = "Dummy read strobe that FlexSPI generates, looped back from DQS pad."]
            pub const VAL1: u32 = 0x01;
            #[doc = "SCLK output clock and looped back from SCLK padReserved"]
            pub const VAL2: u32 = 0x02;
            #[doc = "Flash-memory-provided read strobe and input from DQS pad"]
            pub const VAL3: u32 = 0x03;
        }
    }
    #[doc = "Sample Clock Source Different"]
    pub mod RX_CLK_SRC_DIFF {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Use MCR0\\[RXCLKSRC\\] for Port A and Port B. MCR2\\[RXCLKSRC_B\\] is ignored and MCR0\\[RXCLKSRC\\] selects the Sample Clock source for Flash Reading of both ports A and B."]
            pub const VALUE0: u32 = 0;
            #[doc = "Use MCR0\\[RXCLKSRC\\] for Port A, and MCR2\\[RXCLKSRC_B\\] for Port B. MCR0\\[RXCLKSRC\\] selects the Sample Clock source for Flash Reading of port A (A_SCLK) and MCR2\\[RXCLKSRC_B\\] selects the Sample Clock source for Flash Reading of port B (B_SCLK)."]
            pub const VALUE1: u32 = 0x01;
        }
    }
    #[doc = "Resume Wait Duration"]
    pub mod RESUMEWAIT {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "AHB Bus Control"]
pub mod AHBCR {
    #[doc = "AHB Parallel Mode Enable"]
    pub mod APAREN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Flash will be accessed in Individual mode."]
            pub const APAREN_0: u32 = 0;
            #[doc = "Flash will be accessed in Parallel mode."]
            pub const APAREN_1: u32 = 0x01;
        }
    }
    #[doc = "Clear AHB Transmit Buffer"]
    pub mod CLRAHBTXBUF {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No impact."]
            pub const VAL0: u32 = 0;
            #[doc = "Enable clear operation."]
            pub const VAL1: u32 = 0x01;
        }
    }
    #[doc = "Cacheable Read Access Enable"]
    pub mod CACHABLEEN {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled. When there is AHB bus cachable read access, FlexSPI will not check whether it hit AHB TX Buffer."]
            pub const CACHABLEEN_0: u32 = 0;
            #[doc = "Enabled. When there is AHB bus cachable read access, FlexSPI will check whether it hit AHB TX Buffer first."]
            pub const CACHABLEEN_1: u32 = 0x01;
        }
    }
    #[doc = "Bufferable Write Access Enable"]
    pub mod BUFFERABLEEN {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled. For all AHB write access (no matter bufferable or non-bufferable ), FlexSPI will return AHB Bus ready after all data is transmitted to External device and AHB command finished."]
            pub const BUFFERABLEEN_0: u32 = 0;
            #[doc = "Enabled. For AHB bufferable write access, FlexSPI will return AHB Bus ready when the AHB command is granted by arbitrator and will not wait for AHB command finished."]
            pub const BUFFERABLEEN_1: u32 = 0x01;
        }
    }
    #[doc = "AHB Read Prefetch Enable"]
    pub mod PREFETCHEN {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const VALUE0: u32 = 0;
            #[doc = "Enable"]
            pub const VALUE1: u32 = 0x01;
        }
    }
    #[doc = "AHB Read Address Option"]
    pub mod READADDROPT {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "There is AHB read burst start address alignment limitation when flash is accessed in parallel mode or flash is wordaddressable."]
            pub const READADDROPT_0: u32 = 0;
            #[doc = "There is no AHB read burst start address alignment limitation. FlexSPI will fetch more data than AHB burst required to meet the alignment requirement."]
            pub const READADDROPT_1: u32 = 0x01;
        }
    }
    #[doc = "AHB Read Resume Disable"]
    pub mod RESUMEDISABLE {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Suspended AHB read prefetch resumes when AHB is IDLE."]
            pub const VAL0: u32 = 0;
            #[doc = "Suspended AHB read prefetch does not resume once aborted."]
            pub const VAL1: u32 = 0x01;
        }
    }
    #[doc = "AHB Read Size Alignment"]
    pub mod READSZALIGN {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "AHB read size will be decided by other register setting like PREFETCH_EN,OTFAD_EN..."]
            pub const READSZALIGN_0: u32 = 0;
            #[doc = "AHB read size to up size to 8 bytes aligned, no prefetching"]
            pub const READSZALIGN_1: u32 = 0x01;
        }
    }
    #[doc = "AHB Boundary Alignment"]
    pub mod ALIGNMENT {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No limit"]
            pub const BIT0: u32 = 0;
            #[doc = "1 KBytes"]
            pub const BIT1: u32 = 0x01;
            #[doc = "512 Bytes"]
            pub const BIT2: u32 = 0x02;
            #[doc = "256 Bytes"]
            pub const BIT3: u32 = 0x03;
        }
    }
    #[doc = "AHB Memory-Mapped Flash Base Address"]
    pub mod AFLASHBASE {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Interrupt Enable"]
pub mod INTEN {
    #[doc = "IP-Triggered Command Sequences Execution Finished Interrupt Enable"]
    pub mod IPCMDDONEEN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable interrupt or no impact"]
            pub const VALUE0: u32 = 0;
            #[doc = "Enable interrupt"]
            pub const VALUE1: u32 = 0x01;
        }
    }
    #[doc = "IP-Triggered Command Sequences Grant Timeout Interrupt Enable"]
    pub mod IPCMDGEEN {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable interrupt or no impact"]
            pub const VALUE0: u32 = 0;
            #[doc = "Enable interrupt"]
            pub const VALUE1: u32 = 0x01;
        }
    }
    #[doc = "AHB-Triggered Command Sequences Grant Timeout Interrupt Enable."]
    pub mod AHBCMDGEEN {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable interrupt or no impact"]
            pub const VALUE0: u32 = 0;
            #[doc = "Enable interrupt"]
            pub const VALUE1: u32 = 0x01;
        }
    }
    #[doc = "IP-Triggered Command Sequences Error Detected Interrupt Enable"]
    pub mod IPCMDERREN {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable interrupt or no impact"]
            pub const VALUE0: u32 = 0;
            #[doc = "Enable interrupt"]
            pub const VALUE1: u32 = 0x01;
        }
    }
    #[doc = "AHB-Triggered Command Sequences Error Detected Interrupt Enable"]
    pub mod AHBCMDERREN {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable interrupt or no impact"]
            pub const VALUE0: u32 = 0;
            #[doc = "Enable interrupt"]
            pub const VALUE1: u32 = 0x01;
        }
    }
    #[doc = "IP Receive FIFO Watermark Available Interrupt Enable"]
    pub mod IPRXWAEN {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable interrupt or no impact"]
            pub const VALUE0: u32 = 0;
            #[doc = "Enable interrupt"]
            pub const VALUE1: u32 = 0x01;
        }
    }
    #[doc = "IP Transmit FIFO Watermark Empty Interrupt Enable"]
    pub mod IPTXWEEN {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable interrupt or no impact"]
            pub const VALUE0: u32 = 0;
            #[doc = "Enable interrupt"]
            pub const VALUE1: u32 = 0x01;
        }
    }
    #[doc = "SCLK Stopped By Read Interrupt Enable"]
    pub mod SCKSTOPBYRDEN {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable interrupt or no impact"]
            pub const VALUE0: u32 = 0;
            #[doc = "Enable interrupt"]
            pub const VALUE1: u32 = 0x01;
        }
    }
    #[doc = "SCLK Stopped By Write Interrupt Enable"]
    pub mod SCKSTOPBYWREN {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable interrupt or no impact"]
            pub const VALUE0: u32 = 0;
            #[doc = "Enable interrupt"]
            pub const VALUE1: u32 = 0x01;
        }
    }
    #[doc = "AHB Bus Error Interrupt Enable"]
    pub mod AHBBUSERROREN {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable interrupt or no impact"]
            pub const VALUE0: u32 = 0;
            #[doc = "Enable interrupt"]
            pub const VALUE1: u32 = 0x01;
        }
    }
    #[doc = "Sequence execution Timeout Interrupt Enable"]
    pub mod SEQTIMEOUTEN {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable interrupt or no impact"]
            pub const VALUE0: u32 = 0;
            #[doc = "Enable interrupt"]
            pub const VALUE1: u32 = 0x01;
        }
    }
    #[doc = "OTFAD Key Blob Processing Done Interrupt Enable"]
    pub mod KEYDONEEN {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable interrupt or no impact"]
            pub const VALUE0: u32 = 0;
            #[doc = "Enable interrupt"]
            pub const VALUE1: u32 = 0x01;
        }
    }
    #[doc = "OTFAD Key Blob Processing Error Interrupt Enable"]
    pub mod KEYERROREN {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable interrupt or no impact"]
            pub const VALUE0: u32 = 0;
            #[doc = "Enable interrupt"]
            pub const VALUE1: u32 = 0x01;
        }
    }
}
#[doc = "Interrupt"]
pub mod INTR {
    #[doc = "IP-Triggered Command Sequences Execution Finished"]
    pub mod IPCMDDONE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {
            #[doc = "Interrupt condition has not occurred"]
            pub const NO_INTERRUPT: u32 = 0;
            #[doc = "Interrupt condition has occurred"]
            pub const INTERRUPT: u32 = 0x01;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "IP-Triggered Command Sequences Grant Timeout"]
    pub mod IPCMDGE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {
            #[doc = "Interrupt condition has not occurred"]
            pub const NO_INTERRUPT: u32 = 0;
            #[doc = "Interrupt condition has occurred"]
            pub const INTERRUPT: u32 = 0x01;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "AHB-Triggered Command Sequences Grant Timeout"]
    pub mod AHBCMDGE {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {
            #[doc = "Interrupt condition has not occurred"]
            pub const NO_INTERRUPT: u32 = 0;
            #[doc = "Interrupt condition has occurred"]
            pub const INTERRUPT: u32 = 0x01;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "IP-Triggered Command Sequences Error"]
    pub mod IPCMDERR {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {
            #[doc = "Interrupt condition has not occurred"]
            pub const NO_INTERRUPT: u32 = 0;
            #[doc = "Interrupt condition has occurred"]
            pub const INTERRUPT: u32 = 0x01;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "AHB-Triggered Command Sequences Error"]
    pub mod AHBCMDERR {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {
            #[doc = "Interrupt condition has not occurred"]
            pub const NO_INTERRUPT: u32 = 0;
            #[doc = "Interrupt condition has occurred"]
            pub const INTERRUPT: u32 = 0x01;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "IP Receive FIFO Watermark Available"]
    pub mod IPRXWA {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {
            #[doc = "Interrupt condition has not occurred"]
            pub const NO_INTERRUPT: u32 = 0;
            #[doc = "Interrupt condition has occurred"]
            pub const INTERRUPT: u32 = 0x01;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "IP Transmit FIFO Watermark Empty"]
    pub mod IPTXWE {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {
            #[doc = "Interrupt condition has not occurred"]
            pub const NO_INTERRUPT: u32 = 0;
            #[doc = "Interrupt condition has occurred"]
            pub const INTERRUPT: u32 = 0x01;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "SCLK Stopped Due To Full Receive FIFO"]
    pub mod SCKSTOPBYRD {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {
            #[doc = "Interrupt condition has not occurred"]
            pub const NO_INTERRUPT: u32 = 0;
            #[doc = "Interrupt condition has occurred"]
            pub const INTERRUPT: u32 = 0x01;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "SCLK Stopped Due To Empty Transmit FIFO"]
    pub mod SCKSTOPBYWR {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {
            #[doc = "Interrupt condition has not occurred"]
            pub const NO_INTERRUPT: u32 = 0;
            #[doc = "Interrupt condition has occurred"]
            pub const INTERRUPT: u32 = 0x01;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "AHB Bus Error"]
    pub mod AHBBUSERROR {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {
            #[doc = "Interrupt condition has not occurred"]
            pub const NO_INTERRUPT: u32 = 0;
            #[doc = "Interrupt condition has occurred"]
            pub const INTERRUPT: u32 = 0x01;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Sequence Execution Timeout"]
    pub mod SEQTIMEOUT {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {
            #[doc = "Interrupt condition has not occurred"]
            pub const NO_INTERRUPT: u32 = 0;
            #[doc = "Interrupt condition has occurred"]
            pub const INTERRUPT: u32 = 0x01;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "OTFAD key blob processing done interrupt."]
    pub mod KEYDONE {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "OTFAD Key Blob Processing Error"]
    pub mod KEYERROR {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {
            #[doc = "Interrupt condition has not occurred"]
            pub const NO_INTERRUPT: u32 = 0;
            #[doc = "Interrupt condition has occurred"]
            pub const INTERRUPT: u32 = 0x01;
        }
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "LUT Key"]
pub mod LUTKEY {
    #[doc = "The Key to lock or unlock LUT."]
    pub mod KEY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "LUT Control"]
pub mod LUTCR {
    #[doc = "Lock LUT"]
    pub mod LOCK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "LUT is unlocked (LUTCR\\[UNLOCK\\] must be 1)"]
            pub const VALUE0: u32 = 0;
            #[doc = "LUT is locked and cannot be written"]
            pub const VALUE1: u32 = 0x01;
        }
    }
    #[doc = "Unlock LUT"]
    pub mod UNLOCK {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "LUT is locked (LUTCR\\[LOCK\\] must be 1)"]
            pub const VALUE0: u32 = 0;
            #[doc = "LUT is unlocked and can be written"]
            pub const VALUE1: u32 = 0x01;
        }
    }
}
#[doc = "AHB Receive Buffer 0 Control 0"]
pub mod AHBRXBUF0CR0 {
    #[doc = "AHB Receive Buffer Size"]
    pub mod BUFSZ {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "AHB Controller ID"]
    pub mod MSTRID {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "AHB Controller Read Priority"]
    pub mod PRIORITY {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "AHB Receive Buffer Address Region Enable"]
    pub mod REGIONEN {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled. The buffer hit is based on the value of MSTRID only."]
            pub const VALUE0: u32 = 0;
            #[doc = "Enabled. The buffer hit is based on the value of MSTRID and the address within AHBBUFREGIONSTARTn and AHBREGIONENDn."]
            pub const VALUE1: u32 = 0x01;
        }
    }
    #[doc = "AHB Read Prefetch Enable"]
    pub mod PREFETCHEN {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const VALUE0: u32 = 0;
            #[doc = "Enabled when is enabled."]
            pub const VALUE1: u32 = 0x01;
        }
    }
}
#[doc = "AHB Receive Buffer 1 Control 0"]
pub mod AHBRXBUF1CR0 {
    #[doc = "AHB Receive Buffer Size"]
    pub mod BUFSZ {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "AHB Controller ID"]
    pub mod MSTRID {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "AHB Controller Read Priority"]
    pub mod PRIORITY {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "AHB Receive Buffer Address Region Enable"]
    pub mod REGIONEN {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled. The buffer hit is based on the value of MSTRID only."]
            pub const VALUE0: u32 = 0;
            #[doc = "Enabled. The buffer hit is based on the value of MSTRID and the address within AHBBUFREGIONSTARTn and AHBREGIONENDn."]
            pub const VALUE1: u32 = 0x01;
        }
    }
    #[doc = "AHB Read Prefetch Enable"]
    pub mod PREFETCHEN {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const VALUE0: u32 = 0;
            #[doc = "Enabled when is enabled."]
            pub const VALUE1: u32 = 0x01;
        }
    }
}
#[doc = "AHB Receive Buffer 2 Control 0"]
pub mod AHBRXBUF2CR0 {
    #[doc = "AHB Receive Buffer Size"]
    pub mod BUFSZ {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "AHB Controller ID"]
    pub mod MSTRID {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "AHB Controller Read Priority"]
    pub mod PRIORITY {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "AHB Receive Buffer Address Region Enable"]
    pub mod REGIONEN {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled. The buffer hit is based on the value of MSTRID only."]
            pub const VALUE0: u32 = 0;
            #[doc = "Enabled. The buffer hit is based on the value of MSTRID and the address within AHBBUFREGIONSTARTn and AHBREGIONENDn."]
            pub const VALUE1: u32 = 0x01;
        }
    }
    #[doc = "AHB Read Prefetch Enable"]
    pub mod PREFETCHEN {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const VALUE0: u32 = 0;
            #[doc = "Enabled when is enabled."]
            pub const VALUE1: u32 = 0x01;
        }
    }
}
#[doc = "AHB Receive Buffer 3 Control 0"]
pub mod AHBRXBUF3CR0 {
    #[doc = "AHB Receive Buffer Size"]
    pub mod BUFSZ {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "AHB Controller ID"]
    pub mod MSTRID {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "AHB Controller Read Priority"]
    pub mod PRIORITY {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "AHB Receive Buffer Address Region Enable"]
    pub mod REGIONEN {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled. The buffer hit is based on the value of MSTRID only."]
            pub const VALUE0: u32 = 0;
            #[doc = "Enabled. The buffer hit is based on the value of MSTRID and the address within AHBBUFREGIONSTARTn and AHBREGIONENDn."]
            pub const VALUE1: u32 = 0x01;
        }
    }
    #[doc = "AHB Read Prefetch Enable"]
    pub mod PREFETCHEN {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const VALUE0: u32 = 0;
            #[doc = "Enabled when is enabled."]
            pub const VALUE1: u32 = 0x01;
        }
    }
}
#[doc = "AHB Receive Buffer 4 Control 0"]
pub mod AHBRXBUF4CR0 {
    #[doc = "AHB Receive Buffer Size"]
    pub mod BUFSZ {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "AHB Controller ID"]
    pub mod MSTRID {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "AHB Controller Read Priority"]
    pub mod PRIORITY {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "AHB Receive Buffer Address Region Enable"]
    pub mod REGIONEN {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled. The buffer hit is based on the value of MSTRID only."]
            pub const VALUE0: u32 = 0;
            #[doc = "Enabled. The buffer hit is based on the value of MSTRID and the address within AHBBUFREGIONSTARTn and AHBREGIONENDn."]
            pub const VALUE1: u32 = 0x01;
        }
    }
    #[doc = "AHB Read Prefetch Enable"]
    pub mod PREFETCHEN {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const VALUE0: u32 = 0;
            #[doc = "Enabled when is enabled."]
            pub const VALUE1: u32 = 0x01;
        }
    }
}
#[doc = "AHB Receive Buffer 5 Control 0"]
pub mod AHBRXBUF5CR0 {
    #[doc = "AHB Receive Buffer Size"]
    pub mod BUFSZ {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "AHB Controller ID"]
    pub mod MSTRID {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "AHB Controller Read Priority"]
    pub mod PRIORITY {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "AHB Receive Buffer Address Region Enable"]
    pub mod REGIONEN {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled. The buffer hit is based on the value of MSTRID only."]
            pub const VALUE0: u32 = 0;
            #[doc = "Enabled. The buffer hit is based on the value of MSTRID and the address within AHBBUFREGIONSTARTn and AHBREGIONENDn."]
            pub const VALUE1: u32 = 0x01;
        }
    }
    #[doc = "AHB Read Prefetch Enable"]
    pub mod PREFETCHEN {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const VALUE0: u32 = 0;
            #[doc = "Enabled when is enabled."]
            pub const VALUE1: u32 = 0x01;
        }
    }
}
#[doc = "AHB Receive Buffer 6 Control 0"]
pub mod AHBRXBUF6CR0 {
    #[doc = "AHB Receive Buffer Size"]
    pub mod BUFSZ {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "AHB Controller ID"]
    pub mod MSTRID {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "AHB Controller Read Priority"]
    pub mod PRIORITY {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "AHB Receive Buffer Address Region Enable"]
    pub mod REGIONEN {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled. The buffer hit is based on the value of MSTRID only."]
            pub const VALUE0: u32 = 0;
            #[doc = "Enabled. The buffer hit is based on the value of MSTRID and the address within AHBBUFREGIONSTARTn and AHBREGIONENDn."]
            pub const VALUE1: u32 = 0x01;
        }
    }
    #[doc = "AHB Read Prefetch Enable"]
    pub mod PREFETCHEN {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const VALUE0: u32 = 0;
            #[doc = "Enabled when is enabled."]
            pub const VALUE1: u32 = 0x01;
        }
    }
}
#[doc = "AHB Receive Buffer 7 Control 0"]
pub mod AHBRXBUF7CR0 {
    #[doc = "AHB Receive Buffer Size"]
    pub mod BUFSZ {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "AHB Controller ID"]
    pub mod MSTRID {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "AHB Controller Read Priority"]
    pub mod PRIORITY {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "AHB Receive Buffer Address Region Enable"]
    pub mod REGIONEN {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled. The buffer hit is based on the value of MSTRID only."]
            pub const VALUE0: u32 = 0;
            #[doc = "Enabled. The buffer hit is based on the value of MSTRID and the address within AHBBUFREGIONSTARTn and AHBREGIONENDn."]
            pub const VALUE1: u32 = 0x01;
        }
    }
    #[doc = "AHB Read Prefetch Enable"]
    pub mod PREFETCHEN {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const VALUE0: u32 = 0;
            #[doc = "Enabled when is enabled."]
            pub const VALUE1: u32 = 0x01;
        }
    }
}
#[doc = "Flash Control 0"]
pub mod FLSHA1CR0 {
    #[doc = "Flash Size in KB"]
    pub mod FLSHSZ {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x007f_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "AHB Address Shift Function control"]
    pub mod ADDRSHIFT {
        pub const offset: u32 = 29;
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
}
#[doc = "Flash Control 0"]
pub mod FLSHA2CR0 {
    #[doc = "Flash Size in KB"]
    pub mod FLSHSZ {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x007f_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "AHB Address Shift Function control"]
    pub mod ADDRSHIFT {
        pub const offset: u32 = 29;
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
}
#[doc = "Flash Control 0"]
pub mod FLSHB1CR0 {
    #[doc = "Flash Size in KB"]
    pub mod FLSHSZ {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x007f_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "AHB Address Shift Function control"]
    pub mod ADDRSHIFT {
        pub const offset: u32 = 29;
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
}
#[doc = "Flash Control 0"]
pub mod FLSHB2CR0 {
    #[doc = "Flash Size in KB"]
    pub mod FLSHSZ {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x007f_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "AHB Address Shift Function control"]
    pub mod ADDRSHIFT {
        pub const offset: u32 = 29;
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
}
#[doc = "Flash Control 1"]
pub mod FLSHCR1 {
    #[doc = "Serial Flash CS Setup Time"]
    pub mod TCSS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Serial Flash CS Hold Time"]
    pub mod TCSH {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Word-Addressable"]
    pub mod WA {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Byte-addressable"]
            pub const VALUE0: u32 = 0;
            #[doc = "Word-addressable"]
            pub const VALUE1: u32 = 0x01;
        }
    }
    #[doc = "Column Address Size"]
    pub mod CAS {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Chip Select Interval Unit"]
    pub mod CSINTERVALUNIT {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The CS interval unit is 1 serial clock cycle"]
            pub const CSINTERVALUNIT_0: u32 = 0;
            #[doc = "The CS interval unit is 256 serial clock cycle"]
            pub const CSINTERVALUNIT_1: u32 = 0x01;
        }
    }
    #[doc = "Chip Select Interval"]
    pub mod CSINTERVAL {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Flash Control 2"]
pub mod FLSHCR2 {
    #[doc = "Sequence Index for AHB Read-Triggered Command in LUT"]
    pub mod ARDSEQID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Sequence Number for AHB Read-Triggered Command"]
    pub mod ARDSEQNUM {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Sequence Index for AHB Write-Triggered Command"]
    pub mod AWRSEQID {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Sequence Number for AHB Write-Triggered Command"]
    pub mod AWRSEQNUM {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "AHB Write Wait"]
    pub mod AWRWAIT {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "AWRWAIT Unit"]
    pub mod AWRWAITUNIT {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The AWRWAIT unit is 2 ahb clock cycle"]
            pub const AWRWAITUNIT_0: u32 = 0;
            #[doc = "The AWRWAIT unit is 8 ahb clock cycle"]
            pub const AWRWAITUNIT_1: u32 = 0x01;
            #[doc = "The AWRWAIT unit is 32 ahb clock cycle"]
            pub const AWRWAITUNIT_2: u32 = 0x02;
            #[doc = "The AWRWAIT unit is 128 ahb clock cycle"]
            pub const AWRWAITUNIT_3: u32 = 0x03;
            #[doc = "The AWRWAIT unit is 512 ahb clock cycle"]
            pub const AWRWAITUNIT_4: u32 = 0x04;
            #[doc = "The AWRWAIT unit is 2048 ahb clock cycle"]
            pub const AWRWAITUNIT_5: u32 = 0x05;
            #[doc = "The AWRWAIT unit is 8192 ahb clock cycle"]
            pub const AWRWAITUNIT_6: u32 = 0x06;
            #[doc = "The AWRWAIT unit is 32768 ahb clock cycle"]
            pub const AWRWAITUNIT_7: u32 = 0x07;
        }
    }
    #[doc = "Clear Instruction Pointer"]
    pub mod CLRINSTRPTR {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Flash Control 4"]
pub mod FLSHCR4 {
    #[doc = "Write Mask Option 1"]
    pub mod WMOPT1 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "DQS pin will be used as Write Mask when writing to external device. There is no limitation on AHB write burst start address alignment when flash is accessed in individual mode."]
            pub const WMOPT1_0: u32 = 0;
            #[doc = "DQS pin will not be used as Write Mask when writing to external device. There is limitation on AHB write burst start address alignment when flash is accessed in individual mode."]
            pub const WMOPT1_1: u32 = 0x01;
        }
    }
    #[doc = "Write Mask Option 2"]
    pub mod WMOPT2 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "DQS pin will be used as Write Mask when writing to external device. There is no limitation on AHB write burst length when flash is accessed in individual mode."]
            pub const WMOPT2_0: u32 = 0;
            #[doc = "DQS pin will not be used as Write Mask when writing to external device. There is limitation on AHB write burst length when flash is accessed in individual mode, the minimal write burst length should be 4."]
            pub const WMOPT2_1: u32 = 0x01;
        }
    }
    #[doc = "Write Mask Enable for Port A"]
    pub mod WMENA {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Write mask is disabled, DQS(RWDS) pin will be un-driven when writing to external device."]
            pub const WMENA_0: u32 = 0;
            #[doc = "Write mask is enabled, DQS(RWDS) pin will be driven by FlexSPI as write mask output when writing to external device."]
            pub const WMENA_1: u32 = 0x01;
        }
    }
    #[doc = "Write Mask Enable for Port B"]
    pub mod WMENB {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Write mask is disabled, DQS(RWDS) pin will be un-driven when writing to external device."]
            pub const WMENB_0: u32 = 0;
            #[doc = "Write mask is enabled, DQS(RWDS) pin will be driven by FlexSPI as write mask output when writing to external device."]
            pub const WMENB_1: u32 = 0x01;
        }
    }
}
#[doc = "IP Control 0"]
pub mod IPCR0 {
    #[doc = "Serial Flash Address for IP command."]
    pub mod SFAR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "IP Control 1"]
pub mod IPCR1 {
    #[doc = "Flash Read/Program Data Size (in bytes) for IP command."]
    pub mod IDATSZ {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Sequence Index in LUT for IP command."]
    pub mod ISEQID {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Sequence Number for IP command: ISEQNUM+1."]
    pub mod ISEQNUM {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Parallel Mode Enable for IP Commands"]
    pub mod IPAREN {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Flash will be accessed in Individual mode."]
            pub const IPAREN_0: u32 = 0;
            #[doc = "Flash will be accessed in Parallel mode."]
            pub const IPAREN_1: u32 = 0x01;
        }
    }
}
#[doc = "IP Command"]
pub mod IPCMD {
    #[doc = "Command Trigger"]
    pub mod TRG {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No action"]
            pub const VALUE0: u32 = 0;
            #[doc = "Start the IP command that the IPCR0 and IPCR1 registers define."]
            pub const VALUE1: u32 = 0x01;
        }
    }
}
#[doc = "IP Receive FIFO Control"]
pub mod IPRXFCR {
    #[doc = "Clear IP Receive FIFO"]
    pub mod CLRIPRXF {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No function"]
            pub const VALUE0: u32 = 0;
            #[doc = "A clock cycle pulse clears all valid data entries in IP receive FIFO."]
            pub const VALUE1: u32 = 0x01;
        }
    }
    #[doc = "IP Receive FIFO Reading by DMA Enable"]
    pub mod RXDMAEN {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "IP RX FIFO would be read by processor."]
            pub const RXDMAEN_0: u32 = 0;
            #[doc = "IP RX FIFO would be read by DMA."]
            pub const RXDMAEN_1: u32 = 0x01;
        }
    }
    #[doc = "IP Receive FIFO Watermark Level"]
    pub mod RXWMRK {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "IP Transmit FIFO Control"]
pub mod IPTXFCR {
    #[doc = "Clear IP Transmit FIFO"]
    pub mod CLRIPTXF {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No function"]
            pub const VALUE0: u32 = 0;
            #[doc = "A clock cycle pulse clears all valid data entries in the IP transmit FIFO."]
            pub const VALUE1: u32 = 0x01;
        }
    }
    #[doc = "Transmit FIFO DMA Enable"]
    pub mod TXDMAEN {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "IP TX FIFO would be filled by processor."]
            pub const TXDMAEN_0: u32 = 0;
            #[doc = "IP TX FIFO would be filled by DMA."]
            pub const TXDMAEN_1: u32 = 0x01;
        }
    }
    #[doc = "Transmit Watermark Level"]
    pub mod TXWMRK {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x7f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DLL Control 0"]
pub mod DLLCR {
    #[doc = "DLL Calibration Enable"]
    pub mod DLLEN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const VALUE0: u32 = 0;
            #[doc = "Enable"]
            pub const VALUE1: u32 = 0x01;
        }
    }
    #[doc = "DLL reset"]
    pub mod DLLRESET {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No function"]
            pub const VALUE0: u32 = 0;
            #[doc = "Force DLL reset."]
            pub const VALUE1: u32 = 0x01;
        }
    }
    #[doc = "Target Delay Line"]
    pub mod SLVDLYTARGET {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Target Clock Delay Line Override Value Enable"]
    pub mod OVRDEN {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const VALUE0: u32 = 0;
            #[doc = "Enable"]
            pub const VALUE1: u32 = 0x01;
        }
    }
    #[doc = "Target Clock Delay Line Override Value"]
    pub mod OVRDVAL {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Reference Clock Delay Line Phase Adjust Gap. REFPHASEGAP setting of 2h is recommended if DLLEN is set."]
    pub mod REFPHASEGAP {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Status 0"]
pub mod STS0 {
    #[doc = "SEQ_CTL State Machine Idle"]
    pub mod SEQIDLE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not idle"]
            pub const VALUE0: u32 = 0;
            #[doc = "Idle"]
            pub const VALUE1: u32 = 0x01;
        }
    }
    #[doc = "ARB_CTL State Machine Idle"]
    pub mod ARBIDLE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not idle"]
            pub const VALUE0: u32 = 0;
            #[doc = "Idle"]
            pub const VALUE1: u32 = 0x01;
        }
    }
    #[doc = "ARB Command Source"]
    pub mod ARBCMDSRC {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Triggered by AHB read command (triggered by AHB read)."]
            pub const ARBCMDSRC_0: u32 = 0;
            #[doc = "Triggered by AHB write command (triggered by AHB Write)."]
            pub const ARBCMDSRC_1: u32 = 0x01;
            #[doc = "Triggered by IP command (triggered by setting register bit IPCMD.TRG)."]
            pub const ARBCMDSRC_2: u32 = 0x02;
            #[doc = "Triggered by suspended command (resumed)."]
            pub const ARBCMDSRC_3: u32 = 0x03;
        }
    }
}
#[doc = "Status 1"]
pub mod STS1 {
    #[doc = "AHB Command Error ID"]
    pub mod AHBCMDERRID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "AHB Command Error Code"]
    pub mod AHBCMDERRCODE {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No error."]
            pub const AHBCMDERRCODE_0: u32 = 0;
            #[doc = "AHB Write command with JMP_ON_CS instruction used in the sequence."]
            pub const AHBCMDERRCODE_2: u32 = 0x02;
            #[doc = "There is unknown instruction opcode in the sequence."]
            pub const AHBCMDERRCODE_3: u32 = 0x03;
            #[doc = "Instruction DUMMY_SDR/DUMMY_RWDS_SDR used in DDR sequence."]
            pub const AHBCMDERRCODE_4: u32 = 0x04;
            #[doc = "Instruction DUMMY_DDR/DUMMY_RWDS_DDR used in SDR sequence."]
            pub const AHBCMDERRCODE_5: u32 = 0x05;
            #[doc = "Sequence execution timeout."]
            pub const AHBCMDERRCODE_14: u32 = 0x0e;
        }
    }
    #[doc = "IP Command Error ID"]
    pub mod IPCMDERRID {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "IP Command Error Code"]
    pub mod IPCMDERRCODE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No error."]
            pub const IPCMDERRCODE_0: u32 = 0;
            #[doc = "IP command with JMP_ON_CS instruction used in the sequence."]
            pub const IPCMDERRCODE_2: u32 = 0x02;
            #[doc = "There is unknown instruction opcode in the sequence."]
            pub const IPCMDERRCODE_3: u32 = 0x03;
            #[doc = "Instruction DUMMY_SDR/DUMMY_RWDS_SDR used in DDR sequence."]
            pub const IPCMDERRCODE_4: u32 = 0x04;
            #[doc = "Instruction DUMMY_DDR/DUMMY_RWDS_DDR used in SDR sequence."]
            pub const IPCMDERRCODE_5: u32 = 0x05;
            #[doc = "Flash access start address exceed the whole flash address range (A1/A2/B1/B2)."]
            pub const IPCMDERRCODE_6: u32 = 0x06;
            #[doc = "Sequence execution timeout."]
            pub const IPCMDERRCODE_14: u32 = 0x0e;
            #[doc = "Flash boundary crossed."]
            pub const IPCMDERRCODE_15: u32 = 0x0f;
        }
    }
}
#[doc = "Status 2"]
pub mod STS2 {
    #[doc = "Flash A Sample Target Delay Line Locked"]
    pub mod ASLVLOCK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not locked"]
            pub const VAL0: u32 = 0;
            #[doc = "Locked"]
            pub const VAL1: u32 = 0x01;
        }
    }
    #[doc = "Flash A Sample Clock Reference Delay Line Locked"]
    pub mod AREFLOCK {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not locked"]
            pub const VAL0: u32 = 0;
            #[doc = "Locked"]
            pub const VAL1: u32 = 0x01;
        }
    }
    #[doc = "Flash A Sample Clock Target Delay Line Delay Cell Number"]
    pub mod ASLVSEL {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Flash A Sample Clock Reference Delay Line Delay Cell Number"]
    pub mod AREFSEL {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Flash B Sample Target Reference Delay Line Locked"]
    pub mod BSLVLOCK {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not locked"]
            pub const VAL0: u32 = 0;
            #[doc = "Locked"]
            pub const VAL1: u32 = 0x01;
        }
    }
    #[doc = "Flash B Sample Clock Reference Delay Line Locked"]
    pub mod BREFLOCK {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not locked"]
            pub const VAL0: u32 = 0;
            #[doc = "Locked"]
            pub const VAL1: u32 = 0x01;
        }
    }
    #[doc = "Flash B Sample Clock Target Delay Line Delay Cell Number"]
    pub mod BSLVSEL {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Flash B Sample Clock Reference Delay Line Delay Cell Number"]
    pub mod BREFSEL {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "AHB Suspend Status"]
pub mod AHBSPNDSTS {
    #[doc = "Active AHB Read Prefetch Suspended"]
    pub mod ACTIVE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No suspended AHB read prefetch command."]
            pub const VAL0: u32 = 0;
            #[doc = "An AHB read prefetch command sequence has been suspended."]
            pub const VAL1: u32 = 0x01;
        }
    }
    #[doc = "AHB Receive Buffer ID for Suspended Command Sequence"]
    pub mod BUFID {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data Left"]
    pub mod DATLFT {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "IP Receive FIFO Status"]
pub mod IPRXFSTS {
    #[doc = "Fill level of IP RX FIFO."]
    pub mod FILL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Total Read Data Counter: RDCNTR * 64 Bits."]
    pub mod RDCNTR {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "IP Transmit FIFO Status"]
pub mod IPTXFSTS {
    #[doc = "Fill level of IP TX FIFO."]
    pub mod FILL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Total Write Data Counter: WRCNTR * 64 Bits."]
    pub mod WRCNTR {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "IP Receive FIFO Data x"]
pub mod RFDR {
    #[doc = "RX Data"]
    pub mod RXDATA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "IP TX FIFO Data x"]
pub mod TFDR {
    #[doc = "TX Data"]
    pub mod TXDATA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Lookup Table x"]
pub mod LUT {
    #[doc = "OPERAND0"]
    pub mod OPERAND0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "NUM_PADS0"]
    pub mod NUM_PADS0 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "OPCODE"]
    pub mod OPCODE0 {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "OPERAND1"]
    pub mod OPERAND1 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "NUM_PADS1"]
    pub mod NUM_PADS1 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "OPCODE1"]
    pub mod OPCODE1 {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Receive Buffer Start Address of Region 0"]
pub mod AHBBUFREGIONSTART0 {
    #[doc = "Start address of region 0. Minimal 4K Bytes aligned. It is system address."]
    pub mod START_ADDRESS {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x000f_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Receive Buffer Region 0 End Address"]
pub mod AHBBUFREGIONEND0 {
    #[doc = "End address of region 0. Minimal 4K Bytes aligned. It is system address."]
    pub mod END_ADDRESS {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x000f_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Receive Buffer Start Address of Region 1"]
pub mod AHBBUFREGIONSTART1 {
    #[doc = "Start address of region 1. Minimal 4K Bytes aligned. It is system address."]
    pub mod START_ADDRESS {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x000f_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Receive Buffer Region 1 End Address"]
pub mod AHBBUFREGIONEND1 {
    #[doc = "End address of region 1. Minimal 4K Bytes aligned. It is system address."]
    pub mod END_ADDRESS {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x000f_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Receive Buffer Start Address of Region 2"]
pub mod AHBBUFREGIONSTART2 {
    #[doc = "Start address of region 2. Minimal 4K Bytes aligned. It is system address."]
    pub mod START_ADDRESS {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x000f_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Receive Buffer Region 2 End Address"]
pub mod AHBBUFREGIONEND2 {
    #[doc = "End address of region 2. Minimal 4K Bytes aligned. It is system address."]
    pub mod END_ADDRESS {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x000f_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Receive Buffer Start Address of Region 3"]
pub mod AHBBUFREGIONSTART3 {
    #[doc = "Start address of region 3. Minimal 4K Bytes aligned. It is system address."]
    pub mod START_ADDRESS {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x000f_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Receive Buffer Region 3 End Address"]
pub mod AHBBUFREGIONEND3 {
    #[doc = "End address of region 3. Minimal 4K Bytes aligned. It is system address."]
    pub mod END_ADDRESS {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x000f_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
