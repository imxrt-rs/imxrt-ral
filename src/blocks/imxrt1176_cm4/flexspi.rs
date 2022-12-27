#[doc = "FlexSPI"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "Module Control Register 0"]
    pub MCR0: crate::RWRegister<u32>,
    #[doc = "Module Control Register 1"]
    pub MCR1: crate::RWRegister<u32>,
    #[doc = "Module Control Register 2"]
    pub MCR2: crate::RWRegister<u32>,
    #[doc = "AHB Bus Control Register"]
    pub AHBCR: crate::RWRegister<u32>,
    #[doc = "Interrupt Enable Register"]
    pub INTEN: crate::RWRegister<u32>,
    #[doc = "Interrupt Register"]
    pub INTR: crate::RWRegister<u32>,
    #[doc = "LUT Key Register"]
    pub LUTKEY: crate::RWRegister<u32>,
    #[doc = "LUT Control Register"]
    pub LUTCR: crate::RWRegister<u32>,
    #[doc = "AHB RX Buffer 0 Control Register 0"]
    pub AHBRXBUF0CR0: crate::RWRegister<u32>,
    #[doc = "AHB RX Buffer 1 Control Register 0"]
    pub AHBRXBUF1CR0: crate::RWRegister<u32>,
    #[doc = "AHB RX Buffer 2 Control Register 0"]
    pub AHBRXBUF2CR0: crate::RWRegister<u32>,
    #[doc = "AHB RX Buffer 3 Control Register 0"]
    pub AHBRXBUF3CR0: crate::RWRegister<u32>,
    #[doc = "AHB RX Buffer 4 Control Register 0"]
    pub AHBRXBUF4CR0: crate::RWRegister<u32>,
    #[doc = "AHB RX Buffer 5 Control Register 0"]
    pub AHBRXBUF5CR0: crate::RWRegister<u32>,
    #[doc = "AHB RX Buffer 6 Control Register 0"]
    pub AHBRXBUF6CR0: crate::RWRegister<u32>,
    #[doc = "AHB RX Buffer 7 Control Register 0"]
    pub AHBRXBUF7CR0: crate::RWRegister<u32>,
    _reserved0: [u8; 0x20],
    #[doc = "Flash Control Register 0"]
    pub FLSHA1CR0: crate::RWRegister<u32>,
    #[doc = "Flash Control Register 0"]
    pub FLSHA2CR0: crate::RWRegister<u32>,
    #[doc = "Flash Control Register 0"]
    pub FLSHB1CR0: crate::RWRegister<u32>,
    #[doc = "Flash Control Register 0"]
    pub FLSHB2CR0: crate::RWRegister<u32>,
    #[doc = "Flash Control Register 1"]
    pub FLSHCR1: [crate::RWRegister<u32>; 4usize],
    #[doc = "Flash Control Register 2"]
    pub FLSHCR2: [crate::RWRegister<u32>; 4usize],
    _reserved1: [u8; 0x04],
    #[doc = "Flash Control Register 4"]
    pub FLSHCR4: crate::RWRegister<u32>,
    _reserved2: [u8; 0x08],
    #[doc = "IP Control Register 0"]
    pub IPCR0: crate::RWRegister<u32>,
    #[doc = "IP Control Register 1"]
    pub IPCR1: crate::RWRegister<u32>,
    _reserved3: [u8; 0x08],
    #[doc = "IP Command Register"]
    pub IPCMD: crate::RWRegister<u32>,
    _reserved4: [u8; 0x04],
    #[doc = "IP RX FIFO Control Register"]
    pub IPRXFCR: crate::RWRegister<u32>,
    #[doc = "IP TX FIFO Control Register"]
    pub IPTXFCR: crate::RWRegister<u32>,
    #[doc = "DLL Control Register 0"]
    pub DLLCR: [crate::RWRegister<u32>; 2usize],
    _reserved5: [u8; 0x08],
    #[doc = "Misc Control Register 4"]
    pub MISCCR4: crate::RORegister<u32>,
    #[doc = "Misc Control Register 5"]
    pub MISCCR5: crate::RORegister<u32>,
    #[doc = "Misc Control Register 6"]
    pub MISCCR6: crate::RORegister<u32>,
    #[doc = "Misc Control Register 7"]
    pub MISCCR7: crate::RORegister<u32>,
    #[doc = "Status Register 0"]
    pub STS0: crate::RORegister<u32>,
    #[doc = "Status Register 1"]
    pub STS1: crate::RORegister<u32>,
    #[doc = "Status Register 2"]
    pub STS2: crate::RORegister<u32>,
    #[doc = "AHB Suspend Status Register"]
    pub AHBSPNDSTS: crate::RORegister<u32>,
    #[doc = "IP RX FIFO Status Register"]
    pub IPRXFSTS: crate::RORegister<u32>,
    #[doc = "IP TX FIFO Status Register"]
    pub IPTXFSTS: crate::RORegister<u32>,
    _reserved6: [u8; 0x08],
    #[doc = "IP RX FIFO Data Register x"]
    pub RFDR: [crate::RORegister<u32>; 32usize],
    #[doc = "IP TX FIFO Data Register x"]
    pub TFDR: [crate::WORegister<u32>; 32usize],
    #[doc = "LUT x"]
    pub LUT: [crate::RWRegister<u32>; 64usize],
    _reserved7: [u8; 0x0100],
    #[doc = "AHB Master ID 0 Control Register"]
    pub HMSTR0CR: crate::RWRegister<u32>,
    #[doc = "AHB Master ID 1 Control Register"]
    pub HMSTR1CR: crate::RWRegister<u32>,
    #[doc = "AHB Master ID 2 Control Register"]
    pub HMSTR2CR: crate::RWRegister<u32>,
    #[doc = "AHB Master ID 3 Control Register"]
    pub HMSTR3CR: crate::RWRegister<u32>,
    #[doc = "AHB Master ID 4 Control Register"]
    pub HMSTR4CR: crate::RWRegister<u32>,
    #[doc = "AHB Master ID 5 Control Register"]
    pub HMSTR5CR: crate::RWRegister<u32>,
    #[doc = "AHB Master ID 6 Control Register"]
    pub HMSTR6CR: crate::RWRegister<u32>,
    #[doc = "AHB Master ID 7 Control Register"]
    pub HMSTR7CR: crate::RWRegister<u32>,
    #[doc = "HADDR REMAP START ADDR"]
    pub HADDRSTART: crate::RWRegister<u32>,
    #[doc = "HADDR REMAP END ADDR"]
    pub HADDREND: crate::RWRegister<u32>,
    #[doc = "HADDR REMAP OFFSET"]
    pub HADDROFFSET: crate::RWRegister<u32>,
    _reserved8: [u8; 0x04],
    #[doc = "IPS nonsecure region Start address of region 0"]
    pub IPSNSZSTART0: crate::RWRegister<u32>,
    #[doc = "IPS nonsecure region End address of region 0"]
    pub IPSNSZEND0: crate::RWRegister<u32>,
    #[doc = "IPS nonsecure region Start address of region 1"]
    pub IPSNSZSTART1: crate::RWRegister<u32>,
    #[doc = "IPS nonsecure region End address of region 1"]
    pub IPSNSZEND1: crate::RWRegister<u32>,
    #[doc = "RX BUF Start address of region 0"]
    pub AHBBUFREGIONSTART0: crate::RWRegister<u32>,
    #[doc = "RX BUF region End address of region 0"]
    pub AHBBUFREGIONEND0: crate::RWRegister<u32>,
    #[doc = "RX BUF Start address of region 1"]
    pub AHBBUFREGIONSTART1: crate::RWRegister<u32>,
    #[doc = "RX BUF region End address of region 1"]
    pub AHBBUFREGIONEND1: crate::RWRegister<u32>,
    #[doc = "RX BUF Start address of region 2"]
    pub AHBBUFREGIONSTART2: crate::RWRegister<u32>,
    #[doc = "RX BUF region End address of region 2"]
    pub AHBBUFREGIONEND2: crate::RWRegister<u32>,
    #[doc = "RX BUF Start address of region 3"]
    pub AHBBUFREGIONSTART3: crate::RWRegister<u32>,
    #[doc = "RX BUF region End address of region 3"]
    pub AHBBUFREGIONEND3: crate::RWRegister<u32>,
}
#[doc = "Module Control Register 0"]
pub mod MCR0 {
    #[doc = "Software Reset"]
    pub mod SWRESET {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Module Disable"]
    pub mod MDIS {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Sample Clock source selection for Flash Reading"]
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
    #[doc = "Enable AHB bus Read Access to IP RX FIFO."]
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
    #[doc = "Enable AHB bus Write Access to IP TX FIFO."]
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
    #[doc = "The serial root clock could be divided inside FlexSPI . Refer Clocks chapter for more details on clocking."]
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
    #[doc = "Half Speed Serial Flash access Enable."]
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
    #[doc = "Doze mode enable bit"]
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
    #[doc = "This bit is to support Flash Octal mode access by combining Port A and B Data pins (A_DATA\\[3:0\\] and B_DATA\\[3:0\\])."]
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
    #[doc = "This bit is used to force SCLK output free-running. For FPGA applications, external device may use SCLK as reference clock to its internal PLL. If SCLK free-running is enabled, data sampling with loopback clock from SCLK pad is not supported (MCR0\\[RXCLKSRC\\]=2)."]
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
    #[doc = "Time out wait cycle for IP command grant."]
    pub mod IPGRANTWAIT {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Timeout wait cycle for AHB command grant."]
    pub mod AHBGRANTWAIT {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Module Control Register 1"]
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
#[doc = "Module Control Register 2"]
pub mod MCR2 {
    #[doc = "This bit determines whether AHB RX Buffer and AHB TX Buffer will be cleaned automatically when FlexSPI returns STOP mode ACK. Software should set this bit if AHB RX Buffer or AHB TX Buffer will be powered off in STOP mode. Otherwise AHB read access after exiting STOP mode may hit AHB RX Buffer or AHB TX Buffer but their data entries are invalid."]
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
    #[doc = "All external devices are same devices (both in types and size) for A1/A2/B1/B2."]
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
    #[doc = "B_SCLK pad can be used as A_SCLK differential clock output (inverted clock to A_SCLK). In this case, port B flash access is not available. After changing the value of this field, MCR0\\[SWRESET\\] should be set."]
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
    #[doc = "Wait cycle (in AHB clock cycle) for idle state before suspended command sequence resumed."]
    pub mod RESUMEWAIT {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "AHB Bus Control Register"]
pub mod AHBCR {
    #[doc = "Parallel mode enabled for AHB triggered Command (both read and write) ."]
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
    #[doc = "Clear the status/pointers of AHB RX Buffer. Auto-cleared."]
    pub mod CLRAHBRXBUF {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enable AHB bus cachable read access support."]
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
    #[doc = "Enable AHB bus bufferable write access support. This field affects the last beat of AHB write access, refer for more details about AHB bufferable write."]
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
    #[doc = "AHB Read Prefetch Enable."]
    pub mod PREFETCHEN {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "AHB Read Address option bit. This option bit is intend to remove AHB burst start address alignment limitation."]
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
    #[doc = "AHB Read ECC Enable"]
    pub mod ECCEN {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "AHB read ECC check disabled"]
            pub const ECCEN_0: u32 = 0;
            #[doc = "AHB read ECC check enabled"]
            pub const ECCEN_1: u32 = 0x01;
        }
    }
    #[doc = "AHB transaction SPLIT"]
    pub mod SPLITEN {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "AHB Split disabled"]
            pub const SPLITEN_0: u32 = 0;
            #[doc = "AHB Split enabled"]
            pub const SPLITEN_1: u32 = 0x01;
        }
    }
    #[doc = "AHB SPLIT SIZE"]
    pub mod SPLIT_LIMIT {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "AHB Split Size=8bytes"]
            pub const SPLIT_LIMIT_0: u32 = 0;
            #[doc = "AHB Split Size=16bytes"]
            pub const SPLIT_LIMIT_1: u32 = 0x01;
            #[doc = "AHB Split Size=32bytes"]
            pub const SPLIT_LIMIT_2: u32 = 0x02;
            #[doc = "AHB Split Size=64bytes"]
            pub const SPLIT_LIMIT_3: u32 = 0x03;
        }
    }
    #[doc = "OTFAD KEY BLOC ECC Enable"]
    pub mod KEYECCEN {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "AHB KEY ECC check disabled"]
            pub const KEYECCEN_0: u32 = 0;
            #[doc = "AHB KEY ECC check enabled"]
            pub const KEYECCEN_1: u32 = 0x01;
        }
    }
    #[doc = "AHB ECC Single bit ERR CLR"]
    pub mod ECCSINGLEERRCLR {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "AHB ECC Multi bits ERR CLR"]
    pub mod ECCMULTIERRCLR {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "AHB Master ID Remapping enable"]
    pub mod HMSTRIDREMAP {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ECC Read data swap function"]
    pub mod ECCSWAPEN {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "rdata send to ecc check without swap."]
            pub const DISABLE: u32 = 0;
            #[doc = "rdata send to ecc ehck with swap."]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Decides all AHB read/write boundary. All access cross the boundary will be divided into smaller sub accesses."]
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
}
#[doc = "Interrupt Enable Register"]
pub mod INTEN {
    #[doc = "IP triggered Command Sequences Execution finished interrupt enable."]
    pub mod IPCMDDONEEN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "IP triggered Command Sequences Grant Timeout interrupt enable."]
    pub mod IPCMDGEEN {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "AHB triggered Command Sequences Grant Timeout interrupt enable."]
    pub mod AHBCMDGEEN {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "IP triggered Command Sequences Error Detected interrupt enable."]
    pub mod IPCMDERREN {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "AHB triggered Command Sequences Error Detected interrupt enable."]
    pub mod AHBCMDERREN {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "IP RX FIFO WaterMark available interrupt enable."]
    pub mod IPRXWAEN {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "IP TX FIFO WaterMark empty interrupt enable."]
    pub mod IPTXWEEN {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "SCLK is stopped during command sequence because Async RX FIFO full interrupt enable."]
    pub mod SCKSTOPBYRDEN {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "SCLK is stopped during command sequence because Async TX FIFO empty interrupt enable."]
    pub mod SCKSTOPBYWREN {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "AHB Bus error interrupt enable.Refer Interrupts chapter for more details."]
    pub mod AHBBUSERROREN {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Sequence execution timeout interrupt enable.Refer Interrupts chapter for more details."]
    pub mod SEQTIMEOUTEN {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "OTFAD key blob processing done interrupt enable.Refer Interrupts chapter for more details."]
    pub mod KEYDONEEN {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "OTFAD key blob processing error interrupt enable.Refer Interrupts chapter for more details."]
    pub mod KEYERROREN {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ECC multi bits error interrupt enable.Refer Interrupts chapter for more details."]
    pub mod ECCMULTIERREN {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ECC single bit error interrupt enable.Refer Interrupts chapter for more details."]
    pub mod ECCSINGLEERREN {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "IP command security violation interrupt enable."]
    pub mod IPCMDSECUREVIOEN {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Interrupt Register"]
pub mod INTR {
    #[doc = "IP triggered Command Sequences Execution finished interrupt. This interrupt is also generated when there is IPCMDGE or IPCMDERR interrupt generated."]
    pub mod IPCMDDONE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "IP triggered Command Sequences Grant Timeout interrupt."]
    pub mod IPCMDGE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "AHB triggered Command Sequences Grant Timeout interrupt."]
    pub mod AHBCMDGE {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "IP triggered Command Sequences Error Detected interrupt. When an error detected for IP command, this command will be ignored and not executed at all."]
    pub mod IPCMDERR {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "AHB triggered Command Sequences Error Detected interrupt. When an error detected for AHB command, this command will be ignored and not executed at all."]
    pub mod AHBCMDERR {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "IP RX FIFO watermark available interrupt."]
    pub mod IPRXWA {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "IP TX FIFO watermark empty interrupt."]
    pub mod IPTXWE {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "SCLK is stopped during command sequence because Async RX FIFO full interrupt."]
    pub mod SCKSTOPBYRD {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "SCLK is stopped during command sequence because Async TX FIFO empty interrupt."]
    pub mod SCKSTOPBYWR {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "AHB Bus timeout or AHB bus illegal access Flash during OTFAD key blob processing interrupt."]
    pub mod AHBBUSERROR {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Sequence execution timeout interrupt."]
    pub mod SEQTIMEOUT {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
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
    #[doc = "OTFAD key blob processing error interrupt."]
    pub mod KEYERROR {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ECC multi bits error interrupt."]
    pub mod ECCMULTIERR {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ECC single bit error interrupt."]
    pub mod ECCSINGLEERR {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "IP command security violation interrupt."]
    pub mod IPCMDSECUREVIO {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "LUT Key Register"]
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
#[doc = "LUT Control Register"]
pub mod LUTCR {
    #[doc = "Lock LUT"]
    pub mod LOCK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Unlock LUT"]
    pub mod UNLOCK {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "LUT protection"]
    pub mod PROTECT {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "AHB RX Buffer 0 Control Register 0"]
pub mod AHBRXBUF0CR0 {
    #[doc = "AHB RX Buffer Size in 64 bits."]
    pub mod BUFSZ {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This AHB RX Buffer is assigned according to AHB Master with ID (MSTR_ID)."]
    pub mod MSTRID {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This priority for AHB Master Read which this AHB RX Buffer is assigned. 7 is the highest priority, 0 the lowest."]
    pub mod PRIORITY {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "AHB RX Buffer address region funciton enable"]
    pub mod REGIONEN {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "AHB Read Prefetch Enable for current AHB RX Buffer corresponding Master."]
    pub mod PREFETCHEN {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "AHB RX Buffer 1 Control Register 0"]
pub mod AHBRXBUF1CR0 {
    #[doc = "AHB RX Buffer Size in 64 bits."]
    pub mod BUFSZ {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This AHB RX Buffer is assigned according to AHB Master with ID (MSTR_ID)."]
    pub mod MSTRID {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This priority for AHB Master Read which this AHB RX Buffer is assigned. 7 is the highest priority, 0 the lowest."]
    pub mod PRIORITY {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "AHB RX Buffer address region funciton enable"]
    pub mod REGIONEN {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "AHB Read Prefetch Enable for current AHB RX Buffer corresponding Master."]
    pub mod PREFETCHEN {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "AHB RX Buffer 2 Control Register 0"]
pub mod AHBRXBUF2CR0 {
    #[doc = "AHB RX Buffer Size in 64 bits."]
    pub mod BUFSZ {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This AHB RX Buffer is assigned according to AHB Master with ID (MSTR_ID)."]
    pub mod MSTRID {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This priority for AHB Master Read which this AHB RX Buffer is assigned. 7 is the highest priority, 0 the lowest."]
    pub mod PRIORITY {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "AHB RX Buffer address region funciton enable"]
    pub mod REGIONEN {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "AHB Read Prefetch Enable for current AHB RX Buffer corresponding Master."]
    pub mod PREFETCHEN {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "AHB RX Buffer 3 Control Register 0"]
pub mod AHBRXBUF3CR0 {
    #[doc = "AHB RX Buffer Size in 64 bits."]
    pub mod BUFSZ {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This AHB RX Buffer is assigned according to AHB Master with ID (MSTR_ID)."]
    pub mod MSTRID {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This priority for AHB Master Read which this AHB RX Buffer is assigned. 7 is the highest priority, 0 the lowest."]
    pub mod PRIORITY {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "AHB RX Buffer address region funciton enable"]
    pub mod REGIONEN {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "AHB Read Prefetch Enable for current AHB RX Buffer corresponding Master."]
    pub mod PREFETCHEN {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "AHB RX Buffer 4 Control Register 0"]
pub mod AHBRXBUF4CR0 {
    #[doc = "AHB RX Buffer Size in 64 bits."]
    pub mod BUFSZ {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This AHB RX Buffer is assigned according to AHB Master with ID (MSTR_ID)."]
    pub mod MSTRID {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This priority for AHB Master Read which this AHB RX Buffer is assigned. 7 is the highest priority, 0 the lowest."]
    pub mod PRIORITY {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "AHB RX Buffer address region funciton enable"]
    pub mod REGIONEN {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "AHB Read Prefetch Enable for current AHB RX Buffer corresponding Master."]
    pub mod PREFETCHEN {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "AHB RX Buffer 5 Control Register 0"]
pub mod AHBRXBUF5CR0 {
    #[doc = "AHB RX Buffer Size in 64 bits."]
    pub mod BUFSZ {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This AHB RX Buffer is assigned according to AHB Master with ID (MSTR_ID)."]
    pub mod MSTRID {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This priority for AHB Master Read which this AHB RX Buffer is assigned. 7 is the highest priority, 0 the lowest."]
    pub mod PRIORITY {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "AHB RX Buffer address region funciton enable"]
    pub mod REGIONEN {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "AHB Read Prefetch Enable for current AHB RX Buffer corresponding Master."]
    pub mod PREFETCHEN {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "AHB RX Buffer 6 Control Register 0"]
pub mod AHBRXBUF6CR0 {
    #[doc = "AHB RX Buffer Size in 64 bits."]
    pub mod BUFSZ {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This AHB RX Buffer is assigned according to AHB Master with ID (MSTR_ID)."]
    pub mod MSTRID {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This priority for AHB Master Read which this AHB RX Buffer is assigned. 7 is the highest priority, 0 the lowest."]
    pub mod PRIORITY {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "AHB RX Buffer address region funciton enable"]
    pub mod REGIONEN {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "AHB Read Prefetch Enable for current AHB RX Buffer corresponding Master."]
    pub mod PREFETCHEN {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "AHB RX Buffer 7 Control Register 0"]
pub mod AHBRXBUF7CR0 {
    #[doc = "AHB RX Buffer Size in 64 bits."]
    pub mod BUFSZ {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This AHB RX Buffer is assigned according to AHB Master with ID (MSTR_ID)."]
    pub mod MSTRID {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This priority for AHB Master Read which this AHB RX Buffer is assigned. 7 is the highest priority, 0 the lowest."]
    pub mod PRIORITY {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "AHB RX Buffer address region funciton enable"]
    pub mod REGIONEN {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "AHB Read Prefetch Enable for current AHB RX Buffer corresponding Master."]
    pub mod PREFETCHEN {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Flash Control Register 0"]
pub mod FLSHA1CR0 {
    #[doc = "Flash Size in KByte."]
    pub mod FLSHSZ {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x007f_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "AHB write access split function control."]
    pub mod SPLITWREN {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "AHB read access split function control."]
    pub mod SPLITRDEN {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Flash Control Register 0"]
pub mod FLSHA2CR0 {
    #[doc = "Flash Size in KByte."]
    pub mod FLSHSZ {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x007f_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "AHB write access split function control."]
    pub mod SPLITWREN {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "AHB read access split function control."]
    pub mod SPLITRDEN {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Flash Control Register 0"]
pub mod FLSHB1CR0 {
    #[doc = "Flash Size in KByte."]
    pub mod FLSHSZ {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x007f_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "AHB write access split function control."]
    pub mod SPLITWREN {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "AHB read access split function control."]
    pub mod SPLITRDEN {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Flash Control Register 0"]
pub mod FLSHB2CR0 {
    #[doc = "Flash Size in KByte."]
    pub mod FLSHSZ {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x007f_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "AHB write access split function control."]
    pub mod SPLITWREN {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "AHB read access split function control."]
    pub mod SPLITRDEN {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Flash Control Register 1"]
pub mod FLSHCR1 {
    #[doc = "Serial Flash CS setup time."]
    pub mod TCSS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Serial Flash CS Hold time."]
    pub mod TCSH {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Word Addressable."]
    pub mod WA {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Column Address Size."]
    pub mod CAS {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CS interval unit"]
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
    #[doc = "This field is used to set the minimum interval between flash device Chip selection deassertion and flash device Chip selection assertion. If external flash has a limitation on the interval between command sequences, this field should be set accordingly. If there is no limitation, set this field with value 0x0."]
    pub mod CSINTERVAL {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Flash Control Register 2"]
pub mod FLSHCR2 {
    #[doc = "Sequence Index for AHB Read triggered Command in LUT."]
    pub mod ARDSEQID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Sequence Number for AHB Read triggered Command in LUT."]
    pub mod ARDSEQNUM {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Sequence Index for AHB Write triggered Command."]
    pub mod AWRSEQID {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Sequence Number for AHB Write triggered Command."]
    pub mod AWRSEQNUM {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "For certain devices (such as FPGA), it need some time to write data into internal memory after the command sequences finished on FlexSPI interface"]
    pub mod AWRWAIT {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "AWRWAIT unit"]
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
    #[doc = "Clear the instruction pointer which is internally saved pointer by JMP_ON_CS. Refer Programmable Sequence Engine for details."]
    pub mod CLRINSTRPTR {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Flash Control Register 4"]
pub mod FLSHCR4 {
    #[doc = "Write mask option bit 1. This option bit could be used to remove AHB write burst start address alignment limitation."]
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
    #[doc = "Write mask option bit 2. When using AP memory, This option bit could be used to remove AHB write burst minimal length limitation. When using this bit, WMOPT1 should also be set."]
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
    #[doc = "Write mask enable bit for flash device on port A. When write mask function is needed for memory device on port A, this bit must be set."]
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
    #[doc = "Write mask enable bit for flash device on port B. When write mask function is needed for memory device on port B, this bit must be set."]
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
    #[doc = "Enable APMEM 16 bit write mask function, bit 9 for A1-B1 pair, bit 10 for A2-B2 pair."]
    pub mod PAR_WM {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Disable the address shift logic for lower density of 16 bit PSRAM."]
    pub mod PAR_ADDR_ADJ_DIS {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "IP Control Register 0"]
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
#[doc = "IP Control Register 1"]
pub mod IPCR1 {
    #[doc = "Flash Read/Program Data Size (in Bytes) for IP command."]
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
        pub const mask: u32 = 0x0f << offset;
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
    #[doc = "Parallel mode Enabled for IP command."]
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
#[doc = "IP Command Register"]
pub mod IPCMD {
    #[doc = "Setting this bit will trigger an IP Command."]
    pub mod TRG {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "IP RX FIFO Control Register"]
pub mod IPRXFCR {
    #[doc = "Clear all valid data entries in IP RX FIFO."]
    pub mod CLRIPRXF {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "IP RX FIFO reading by DMA enabled."]
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
    #[doc = "Watermark level is (RXWMRK+1)*64 Bits."]
    pub mod RXWMRK {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "IP TX FIFO Control Register"]
pub mod IPTXFCR {
    #[doc = "Clear all valid data entries in IP TX FIFO."]
    pub mod CLRIPTXF {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "IP TX FIFO filling by DMA enabled."]
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
    #[doc = "Watermark level is (TXWMRK+1)*64 Bits."]
    pub mod TXWMRK {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DLL Control Register 0"]
pub mod DLLCR {
    #[doc = "DLL calibration enable."]
    pub mod DLLEN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Software could force a reset on DLL by setting this field to 0x1. This will cause the DLL to lose lock and re-calibrate to detect an ref_clock half period phase shift. The reset action is edge triggered, so software need to clear this bit after set this bit (no delay limitation)."]
    pub mod DLLRESET {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The delay target for slave delay line is: ((SLVDLYTARGET+1) * 1/32 * clock cycle of reference clock (serial root clock). If serial root clock is >= 100 MHz, DLLEN set to 0x1, OVRDEN set to =0x0, then SLVDLYTARGET setting of 0xF is recommended."]
    pub mod SLVDLYTARGET {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Slave clock delay line delay cell number selection override enable."]
    pub mod OVRDEN {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Slave clock delay line delay cell number selection override value."]
    pub mod OVRDVAL {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Misc Control Register 4"]
pub mod MISCCR4 {
    #[doc = "AHB bus address that trigger the current ECC multi bits error interrupt."]
    pub mod AHBADDRESS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Misc Control Register 5"]
pub mod MISCCR5 {
    #[doc = "ECC single bit error correction indication."]
    pub mod ECCSINGLEERRORCORR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Misc Control Register 6"]
pub mod MISCCR6 {
    #[doc = "ECC single error information Valid"]
    pub mod VALID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ECC single error information Hit"]
    pub mod HIT {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ECC single error address"]
    pub mod ADDRESS {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x3fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Misc Control Register 7"]
pub mod MISCCR7 {
    #[doc = "ECC multi error information Valid"]
    pub mod VALID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ECC multi error information Hit"]
    pub mod HIT {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ECC multi error address"]
    pub mod ADDRESS {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x3fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Status Register 0"]
pub mod STS0 {
    #[doc = "This status bit indicates the state machine in SEQ_CTL is idle and there is command sequence executing on FlexSPI interface."]
    pub mod SEQIDLE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This status bit indicates the state machine in ARB_CTL is busy and there is command sequence granted by arbitrator and not finished yet on FlexSPI interface. When ARB_CTL state (ARBIDLE=0x1) is idle, there will be no transaction on FlexSPI interface also (SEQIDLE=0x1). So this bit should be polled to wait for FlexSPI controller become idle instead of SEQIDLE."]
    pub mod ARBIDLE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This status field indicates the trigger source of current command sequence granted by arbitrator. This field value is meaningless when ARB_CTL is not busy (STS0\\[ARBIDLE\\]=0x1)."]
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
#[doc = "Status Register 1"]
pub mod STS1 {
    #[doc = "Indicates the sequence index when an AHB command error is detected. This field will be cleared when INTR\\[AHBCMDERR\\] is write-1-clear(w1c)."]
    pub mod AHBCMDERRID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Indicates the Error Code when AHB command Error detected. This field will be cleared when INTR\\[AHBCMDERR\\] is write-1-clear(w1c)."]
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
    #[doc = "Indicates the sequence Index when IP command error detected. This field will be cleared when INTR\\[IPCMDERR\\] is write-1-clear(w1c)."]
    pub mod IPCMDERRID {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Indicates the Error Code when IP command Error detected. This field will be cleared when INTR\\[IPCMDERR\\] is write-1-clear(w1c)."]
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
#[doc = "Status Register 2"]
pub mod STS2 {
    #[doc = "Flash A sample clock slave delay line locked."]
    pub mod ASLVLOCK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Flash A sample clock reference delay line locked."]
    pub mod AREFLOCK {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Flash A sample clock slave delay line delay cell number selection ."]
    pub mod ASLVSEL {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Flash A sample clock reference delay line delay cell number selection."]
    pub mod AREFSEL {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Flash B sample clock slave delay line locked."]
    pub mod BSLVLOCK {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Flash B sample clock reference delay line locked."]
    pub mod BREFLOCK {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Flash B sample clock slave delay line delay cell number selection."]
    pub mod BSLVSEL {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Flash B sample clock reference delay line delay cell number selection."]
    pub mod BREFSEL {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "AHB Suspend Status Register"]
pub mod AHBSPNDSTS {
    #[doc = "Indicates if an AHB read prefetch command sequence has been suspended."]
    pub mod ACTIVE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "AHB RX BUF ID for suspended command sequence."]
    pub mod BUFID {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Left Data size for suspended command sequence (in byte)."]
    pub mod DATLFT {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "IP RX FIFO Status Register"]
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
#[doc = "IP TX FIFO Status Register"]
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
#[doc = "IP RX FIFO Data Register x"]
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
#[doc = "IP TX FIFO Data Register x"]
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
#[doc = "LUT x"]
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
#[doc = "AHB Master ID 0 Control Register"]
pub mod HMSTR0CR {
    #[doc = "Mask bits for AHB master ID."]
    pub mod MASK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Mask"]
            pub const DISABLE: u32 = 0;
            #[doc = "Unmask"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "This is expected Master ID."]
    pub mod MSTRID {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "AHB Master ID 1 Control Register"]
pub mod HMSTR1CR {
    #[doc = "Mask bits for AHB master ID."]
    pub mod MASK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Mask"]
            pub const DISABLE: u32 = 0;
            #[doc = "Unmask"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "This is expected Master ID."]
    pub mod MSTRID {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "AHB Master ID 2 Control Register"]
pub mod HMSTR2CR {
    #[doc = "Mask bits for AHB master ID."]
    pub mod MASK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Mask"]
            pub const DISABLE: u32 = 0;
            #[doc = "Unmask"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "This is expected Master ID."]
    pub mod MSTRID {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "AHB Master ID 3 Control Register"]
pub mod HMSTR3CR {
    #[doc = "Mask bits for AHB master ID."]
    pub mod MASK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Mask"]
            pub const DISABLE: u32 = 0;
            #[doc = "Unmask"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "This is expected Master ID."]
    pub mod MSTRID {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "AHB Master ID 4 Control Register"]
pub mod HMSTR4CR {
    #[doc = "Mask bits for AHB master ID."]
    pub mod MASK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Mask"]
            pub const DISABLE: u32 = 0;
            #[doc = "Unmask"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "This is expected Master ID."]
    pub mod MSTRID {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "AHB Master ID 5 Control Register"]
pub mod HMSTR5CR {
    #[doc = "Mask bits for AHB master ID."]
    pub mod MASK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Mask"]
            pub const DISABLE: u32 = 0;
            #[doc = "Unmask"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "This is expected Master ID."]
    pub mod MSTRID {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "AHB Master ID 6 Control Register"]
pub mod HMSTR6CR {
    #[doc = "Mask bits for AHB master ID."]
    pub mod MASK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Mask"]
            pub const DISABLE: u32 = 0;
            #[doc = "Unmask"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "This is expected Master ID."]
    pub mod MSTRID {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "AHB Master ID 7 Control Register"]
pub mod HMSTR7CR {
    #[doc = "Mask bits for AHB master ID."]
    pub mod MASK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Mask"]
            pub const DISABLE: u32 = 0;
            #[doc = "Unmask"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "This is expected Master ID."]
    pub mod MSTRID {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "HADDR REMAP START ADDR"]
pub mod HADDRSTART {
    #[doc = "AHB Bus address remap function enable"]
    pub mod REMAPEN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "HADDR REMAP Disabled"]
            pub const REMAPEN_0: u32 = 0;
            #[doc = "HADDR REMAP Enabled"]
            pub const REMAPEN_1: u32 = 0x01;
        }
    }
    #[doc = "OTFAD Keyblob is in ECC region and need to be remapped"]
    pub mod KBINECC {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "If key blob is in remap region, FlexSPI will fetch keyblob at base address + offset"]
            pub const KBINECC_0: u32 = 0;
            #[doc = "If key blob is in remap region, FlexSPI will fetch keyblob at base address + offset*2"]
            pub const KBINECC_1: u32 = 0x01;
        }
    }
    #[doc = "HADDR remap range's start addr, 4K aligned When ADDRSTART setting is same as ASFM_BASE, and OTFAD keyblob function is enabled, keyblob will also be remapped"]
    pub mod ADDRSTART {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x000f_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "HADDR REMAP END ADDR"]
pub mod HADDREND {
    #[doc = "HADDR remap range's end addr, 4K aligned"]
    pub mod ENDSTART {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x000f_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "HADDR REMAP OFFSET"]
pub mod HADDROFFSET {
    #[doc = "HADDR offset field, remapped address will be ADDR\\[31:12\\]=ADDR_original\\[31:12\\]+ADDROFFSET"]
    pub mod ADDROFFSET {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x000f_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "IPS nonsecure region Start address of region 0"]
pub mod IPSNSZSTART0 {
    #[doc = "Start address of region 0. Minimal 4K Bytes aligned. It is flash address."]
    pub mod START_ADDRESS {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x000f_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "IPS nonsecure region End address of region 0"]
pub mod IPSNSZEND0 {
    #[doc = "End address of region 0. Minimal 4K Bytes aligned. It is flash address."]
    pub mod END_ADDRESS {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x000f_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "IPS nonsecure region Start address of region 1"]
pub mod IPSNSZSTART1 {
    #[doc = "Start address of region 1. Minimal 4K Bytes aligned. It is flash address."]
    pub mod START_ADDRESS {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x000f_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "IPS nonsecure region End address of region 1"]
pub mod IPSNSZEND1 {
    #[doc = "End address of region 1. Minimal 4K Bytes aligned. It is flash address."]
    pub mod END_ADDRESS {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x000f_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "RX BUF Start address of region 0"]
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
#[doc = "RX BUF region End address of region 0"]
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
#[doc = "RX BUF Start address of region 1"]
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
#[doc = "RX BUF region End address of region 1"]
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
#[doc = "RX BUF Start address of region 2"]
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
#[doc = "RX BUF region End address of region 2"]
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
#[doc = "RX BUF Start address of region 3"]
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
#[doc = "RX BUF region End address of region 3"]
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
