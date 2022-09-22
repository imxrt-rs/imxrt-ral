#[doc = "SEMC"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "Module Control Register"]
    pub MCR: crate::RWRegister<u32>,
    #[doc = "IO Mux Control Register"]
    pub IOCR: crate::RWRegister<u32>,
    #[doc = "Master Bus (AXI) Control Register 0"]
    pub BMCR0: crate::RWRegister<u32>,
    #[doc = "Master Bus (AXI) Control Register 1"]
    pub BMCR1: crate::RWRegister<u32>,
    #[doc = "Base Register 0 (For SDRAM CS0 device)"]
    pub BR0: crate::RWRegister<u32>,
    #[doc = "Base Register 1 (For SDRAM CS1 device)"]
    pub BR1: crate::RWRegister<u32>,
    #[doc = "Base Register 2 (For SDRAM CS2 device)"]
    pub BR2: crate::RWRegister<u32>,
    #[doc = "Base Register 3 (For SDRAM CS3 device)"]
    pub BR3: crate::RWRegister<u32>,
    #[doc = "Base Register 4 (For NAND device)"]
    pub BR4: crate::RWRegister<u32>,
    #[doc = "Base Register 5 (For NOR device)"]
    pub BR5: crate::RWRegister<u32>,
    #[doc = "Base Register 6 (For PSRAM device)"]
    pub BR6: crate::RWRegister<u32>,
    #[doc = "Base Register 7 (For DBI-B (MIPI Display Bus Interface Type B) device)"]
    pub BR7: crate::RWRegister<u32>,
    #[doc = "Base Register 8 (For NAND device)"]
    pub BR8: crate::RWRegister<u32>,
    #[doc = "DLL Control Register"]
    pub DLLCR: crate::RWRegister<u32>,
    #[doc = "Interrupt Enable Register"]
    pub INTEN: crate::RWRegister<u32>,
    #[doc = "Interrupt Enable Register"]
    pub INTR: crate::RWRegister<u32>,
    #[doc = "SDRAM control register 0"]
    pub SDRAMCR0: crate::RWRegister<u32>,
    #[doc = "SDRAM control register 1"]
    pub SDRAMCR1: crate::RWRegister<u32>,
    #[doc = "SDRAM control register 2"]
    pub SDRAMCR2: crate::RWRegister<u32>,
    #[doc = "SDRAM control register 3"]
    pub SDRAMCR3: crate::RWRegister<u32>,
    #[doc = "NAND control register 0"]
    pub NANDCR0: crate::RWRegister<u32>,
    #[doc = "NAND control register 1"]
    pub NANDCR1: crate::RWRegister<u32>,
    #[doc = "NAND control register 2"]
    pub NANDCR2: crate::RWRegister<u32>,
    #[doc = "NAND control register 3"]
    pub NANDCR3: crate::RWRegister<u32>,
    #[doc = "NOR control register 0"]
    pub NORCR0: crate::RWRegister<u32>,
    #[doc = "NOR control register 1"]
    pub NORCR1: crate::RWRegister<u32>,
    #[doc = "NOR control register 2"]
    pub NORCR2: crate::RWRegister<u32>,
    #[doc = "NOR control register 3"]
    pub NORCR3: crate::RWRegister<u32>,
    #[doc = "SRAM control register 0"]
    pub SRAMCR0: crate::RWRegister<u32>,
    #[doc = "SRAM control register 1"]
    pub SRAMCR1: crate::RWRegister<u32>,
    #[doc = "SRAM control register 2"]
    pub SRAMCR2: crate::RWRegister<u32>,
    #[doc = "SRAM control register 3"]
    pub SRAMCR3: crate::RWRegister<u32>,
    #[doc = "DBI-B control register 0"]
    pub DBICR0: crate::RWRegister<u32>,
    #[doc = "DBI-B control register 1"]
    pub DBICR1: crate::RWRegister<u32>,
    _reserved0: [u8; 0x08],
    #[doc = "IP Command control register 0"]
    pub IPCR0: crate::RWRegister<u32>,
    #[doc = "IP Command control register 1"]
    pub IPCR1: crate::RWRegister<u32>,
    #[doc = "IP Command control register 2"]
    pub IPCR2: crate::RWRegister<u32>,
    #[doc = "IP Command register"]
    pub IPCMD: crate::RWRegister<u32>,
    #[doc = "TX DATA register (for IP Command)"]
    pub IPTXDAT: crate::RWRegister<u32>,
    _reserved1: [u8; 0x0c],
    #[doc = "RX DATA register (for IP Command)"]
    pub IPRXDAT: crate::RORegister<u32>,
    _reserved2: [u8; 0x0c],
    #[doc = "Status register 0"]
    pub STS0: crate::RORegister<u32>,
    #[doc = "Status register 1"]
    pub STS1: crate::RORegister<u32>,
    #[doc = "Status register 2"]
    pub STS2: crate::RORegister<u32>,
    #[doc = "Status register 3"]
    pub STS3: crate::RORegister<u32>,
    #[doc = "Status register 4"]
    pub STS4: crate::RORegister<u32>,
    #[doc = "Status register 5"]
    pub STS5: crate::RORegister<u32>,
    #[doc = "Status register 6"]
    pub STS6: crate::RORegister<u32>,
    #[doc = "Status register 7"]
    pub STS7: crate::RORegister<u32>,
    #[doc = "Status register 8"]
    pub STS8: crate::RORegister<u32>,
    #[doc = "Status register 9"]
    pub STS9: crate::RORegister<u32>,
    #[doc = "Status register 10"]
    pub STS10: crate::RORegister<u32>,
    #[doc = "Status register 11"]
    pub STS11: crate::RORegister<u32>,
    #[doc = "Status register 12"]
    pub STS12: crate::RORegister<u32>,
    #[doc = "Status register 13"]
    pub STS13: crate::RORegister<u32>,
    #[doc = "Status register 14"]
    pub STS14: crate::RORegister<u32>,
    #[doc = "Status register 15"]
    pub STS15: crate::RORegister<u32>,
}
#[doc = "Module Control Register"]
pub mod MCR {
    #[doc = "Software Reset"]
    pub mod SWRST {
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
        pub mod RW {
            #[doc = "Module enabled"]
            pub const MDIS_0: u32 = 0;
            #[doc = "Master disabled."]
            pub const MDIS_1: u32 = 0x01;
        }
    }
    #[doc = "DQS (read strobe) mode"]
    pub mod DQSMD {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Dummy read strobe loopbacked internally"]
            pub const DQSMD_0: u32 = 0;
            #[doc = "Dummy read strobe loopbacked from DQS pad"]
            pub const DQSMD_1: u32 = 0x01;
        }
    }
    #[doc = "WAIT/RDY# polarity for NOR/PSRAM"]
    pub mod WPOL0 {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Low active"]
            pub const WPOL0_0: u32 = 0;
            #[doc = "High active"]
            pub const WPOL0_1: u32 = 0x01;
        }
    }
    #[doc = "WAIT/RDY# polarity for NAND"]
    pub mod WPOL1 {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Low active"]
            pub const WPOL1_0: u32 = 0;
            #[doc = "High active"]
            pub const WPOL1_1: u32 = 0x01;
        }
    }
    #[doc = "Select DQS source when DQSMD and DLLSEL both set."]
    pub mod DQSSEL {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "SDRAM/NOR/SRAM read clock source is from DQS pad in synchronous mode."]
            pub const DQSSEL_0: u32 = 0;
            #[doc = "SDRAM/NOR/SRAM read clock source is from DLL delay chain in synchronous mode."]
            pub const DQSSEL_1: u32 = 0x01;
        }
    }
    #[doc = "Select DLL delay chain clock input."]
    pub mod DLLSEL {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "DLL delay chain clock input is from NAND device's DQS pad. For NAND synchronous mode only."]
            pub const DLLSEL_0: u32 = 0;
            #[doc = "DLL delay chain clock input is from internal clock. For SDRAM, NOR and SRAM synchronous mode only."]
            pub const DLLSEL_1: u32 = 0x01;
        }
    }
    #[doc = "Command Execution timeout cycles"]
    pub mod CTO {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Bus timeout cycles"]
    pub mod BTO {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "255*1"]
            pub const BTO_0: u32 = 0;
            #[doc = "255*2 - 255*2^30"]
            pub const BTO_1: u32 = 0x01;
            #[doc = "255*2 - 255*2^30"]
            pub const BTO_2: u32 = 0x02;
            #[doc = "255*2 - 255*2^30"]
            pub const BTO_3: u32 = 0x03;
            #[doc = "255*2 - 255*2^30"]
            pub const BTO_4: u32 = 0x04;
            #[doc = "255*2 - 255*2^30"]
            pub const BTO_5: u32 = 0x05;
            #[doc = "255*2 - 255*2^30"]
            pub const BTO_6: u32 = 0x06;
            #[doc = "255*2 - 255*2^30"]
            pub const BTO_7: u32 = 0x07;
            #[doc = "255*2 - 255*2^30"]
            pub const BTO_8: u32 = 0x08;
            #[doc = "255*2 - 255*2^30"]
            pub const BTO_9: u32 = 0x09;
            #[doc = "255*2^31"]
            pub const BTO_31: u32 = 0x1f;
        }
    }
}
#[doc = "IO Mux Control Register"]
pub mod IOCR {
    #[doc = "SEMC_A8 output selection"]
    pub mod MUX_A8 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "SDRAM Address bit (A8)"]
            pub const MUX_A8_0: u32 = 0;
            #[doc = "NAND CE#"]
            pub const MUX_A8_1: u32 = 0x01;
            #[doc = "NOR CE#"]
            pub const MUX_A8_2: u32 = 0x02;
            #[doc = "PSRAM CE#"]
            pub const MUX_A8_3: u32 = 0x03;
            #[doc = "DBI CSX"]
            pub const MUX_A8_4: u32 = 0x04;
            #[doc = "SDRAM Address bit (A8)"]
            pub const MUX_A8_5: u32 = 0x05;
            #[doc = "SDRAM Address bit (A8)"]
            pub const MUX_A8_6: u32 = 0x06;
            #[doc = "SDRAM Address bit (A8)"]
            pub const MUX_A8_7: u32 = 0x07;
        }
    }
    #[doc = "SEMC_CSX0 output selection"]
    pub mod MUX_CSX0 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "NOR/PSRAM Address bit 24 (A24)"]
            pub const MUX_CSX0_0: u32 = 0;
            #[doc = "SDRAM CS1"]
            pub const MUX_CSX0_1: u32 = 0x01;
            #[doc = "SDRAM CS2"]
            pub const MUX_CSX0_2: u32 = 0x02;
            #[doc = "SDRAM CS3"]
            pub const MUX_CSX0_3: u32 = 0x03;
            #[doc = "NAND CE#"]
            pub const MUX_CSX0_4: u32 = 0x04;
            #[doc = "NOR CE#"]
            pub const MUX_CSX0_5: u32 = 0x05;
            #[doc = "PSRAM CE#"]
            pub const MUX_CSX0_6: u32 = 0x06;
            #[doc = "DBI CSX"]
            pub const MUX_CSX0_7: u32 = 0x07;
        }
    }
    #[doc = "SEMC_CSX1 output selection"]
    pub mod MUX_CSX1 {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "NOR/PSRAM Address bit 25 (A25)"]
            pub const MUX_CSX1_0: u32 = 0;
            #[doc = "SDRAM CS1"]
            pub const MUX_CSX1_1: u32 = 0x01;
            #[doc = "SDRAM CS2"]
            pub const MUX_CSX1_2: u32 = 0x02;
            #[doc = "SDRAM CS3"]
            pub const MUX_CSX1_3: u32 = 0x03;
            #[doc = "NAND CE#"]
            pub const MUX_CSX1_4: u32 = 0x04;
            #[doc = "NOR CE#"]
            pub const MUX_CSX1_5: u32 = 0x05;
            #[doc = "PSRAM CE#"]
            pub const MUX_CSX1_6: u32 = 0x06;
            #[doc = "DBI CSX"]
            pub const MUX_CSX1_7: u32 = 0x07;
        }
    }
    #[doc = "SEMC_CSX2 output selection"]
    pub mod MUX_CSX2 {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "NOR/PSRAM Address bit 26 (A26)"]
            pub const MUX_CSX2_0: u32 = 0;
            #[doc = "SDRAM CS1"]
            pub const MUX_CSX2_1: u32 = 0x01;
            #[doc = "SDRAM CS2"]
            pub const MUX_CSX2_2: u32 = 0x02;
            #[doc = "SDRAM CS3"]
            pub const MUX_CSX2_3: u32 = 0x03;
            #[doc = "NAND CE#"]
            pub const MUX_CSX2_4: u32 = 0x04;
            #[doc = "NOR CE#"]
            pub const MUX_CSX2_5: u32 = 0x05;
            #[doc = "PSRAM CE#"]
            pub const MUX_CSX2_6: u32 = 0x06;
            #[doc = "DBI CSX"]
            pub const MUX_CSX2_7: u32 = 0x07;
        }
    }
    #[doc = "SEMC_CSX3 output selection"]
    pub mod MUX_CSX3 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "NOR/PSRAM Address bit 27 (A27)"]
            pub const MUX_CSX3_0: u32 = 0;
            #[doc = "SDRAM CS1"]
            pub const MUX_CSX3_1: u32 = 0x01;
            #[doc = "SDRAM CS2"]
            pub const MUX_CSX3_2: u32 = 0x02;
            #[doc = "SDRAM CS3"]
            pub const MUX_CSX3_3: u32 = 0x03;
            #[doc = "NAND CE#"]
            pub const MUX_CSX3_4: u32 = 0x04;
            #[doc = "NOR CE#"]
            pub const MUX_CSX3_5: u32 = 0x05;
            #[doc = "PSRAM CE#"]
            pub const MUX_CSX3_6: u32 = 0x06;
            #[doc = "DBI CSX"]
            pub const MUX_CSX3_7: u32 = 0x07;
        }
    }
    #[doc = "SEMC_RDY function selection"]
    pub mod MUX_RDY {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "NAND Ready/Wait# input"]
            pub const MUX_RDY_0: u32 = 0;
            #[doc = "SDRAM CS1"]
            pub const MUX_RDY_1: u32 = 0x01;
            #[doc = "SDRAM CS2"]
            pub const MUX_RDY_2: u32 = 0x02;
            #[doc = "SDRAM CS3"]
            pub const MUX_RDY_3: u32 = 0x03;
            #[doc = "NOR CE#"]
            pub const MUX_RDY_4: u32 = 0x04;
            #[doc = "PSRAM CE#"]
            pub const MUX_RDY_5: u32 = 0x05;
            #[doc = "DBI CSX"]
            pub const MUX_RDY_6: u32 = 0x06;
            #[doc = "NOR/PSRAM Address bit 27"]
            pub const MUX_RDY_7: u32 = 0x07;
        }
    }
    #[doc = "SEMC_CLKX0 function selection"]
    pub mod MUX_CLKX0 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "NOR clock"]
            pub const MUX_CLKX0_0: u32 = 0;
            #[doc = "SRAM clock"]
            pub const MUX_CLKX0_1: u32 = 0x01;
        }
    }
    #[doc = "SEMC_CLKX1 function selection"]
    pub mod MUX_CLKX1 {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "NOR clock"]
            pub const MUX_CLKX1_0: u32 = 0;
            #[doc = "SRAM clock"]
            pub const MUX_CLKX1_1: u32 = 0x01;
        }
    }
}
#[doc = "Master Bus (AXI) Control Register 0"]
pub mod BMCR0 {
    #[doc = "Weight of QoS"]
    pub mod WQOS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Weight of Aging"]
    pub mod WAGE {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Weight of Slave Hit (no read/write switch)"]
    pub mod WSH {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Weight of Slave Hit (Read/Write switch)"]
    pub mod WRWS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Master Bus (AXI) Control Register 1"]
pub mod BMCR1 {
    #[doc = "Weight of QoS"]
    pub mod WQOS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Weight of Aging"]
    pub mod WAGE {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Weight of Page Hit"]
    pub mod WPH {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Weight of Read/Write switch"]
    pub mod WRWS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Weight of Bank Rotation"]
    pub mod WBR {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Base Register 0 (For SDRAM CS0 device)"]
pub mod BR0 {
    #[doc = "Valid"]
    pub mod VLD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Memory size"]
    pub mod MS {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "4KB"]
            pub const MS_0: u32 = 0;
            #[doc = "8KB"]
            pub const MS_1: u32 = 0x01;
            #[doc = "16KB"]
            pub const MS_2: u32 = 0x02;
            #[doc = "32KB"]
            pub const MS_3: u32 = 0x03;
            #[doc = "64KB"]
            pub const MS_4: u32 = 0x04;
            #[doc = "128KB"]
            pub const MS_5: u32 = 0x05;
            #[doc = "256KB"]
            pub const MS_6: u32 = 0x06;
            #[doc = "512KB"]
            pub const MS_7: u32 = 0x07;
            #[doc = "1MB"]
            pub const MS_8: u32 = 0x08;
            #[doc = "2MB"]
            pub const MS_9: u32 = 0x09;
            #[doc = "4MB"]
            pub const MS_10: u32 = 0x0a;
            #[doc = "8MB"]
            pub const MS_11: u32 = 0x0b;
            #[doc = "16MB"]
            pub const MS_12: u32 = 0x0c;
            #[doc = "32MB"]
            pub const MS_13: u32 = 0x0d;
            #[doc = "64MB"]
            pub const MS_14: u32 = 0x0e;
            #[doc = "128MB"]
            pub const MS_15: u32 = 0x0f;
            #[doc = "256MB"]
            pub const MS_16: u32 = 0x10;
            #[doc = "512MB"]
            pub const MS_17: u32 = 0x11;
            #[doc = "1GB"]
            pub const MS_18: u32 = 0x12;
            #[doc = "2GB"]
            pub const MS_19: u32 = 0x13;
            #[doc = "4GB"]
            pub const MS_20: u32 = 0x14;
            #[doc = "4GB"]
            pub const MS_21: u32 = 0x15;
            #[doc = "4GB"]
            pub const MS_22: u32 = 0x16;
            #[doc = "4GB"]
            pub const MS_23: u32 = 0x17;
            #[doc = "4GB"]
            pub const MS_24: u32 = 0x18;
            #[doc = "4GB"]
            pub const MS_25: u32 = 0x19;
            #[doc = "4GB"]
            pub const MS_26: u32 = 0x1a;
            #[doc = "4GB"]
            pub const MS_27: u32 = 0x1b;
            #[doc = "4GB"]
            pub const MS_28: u32 = 0x1c;
            #[doc = "4GB"]
            pub const MS_29: u32 = 0x1d;
            #[doc = "4GB"]
            pub const MS_30: u32 = 0x1e;
            #[doc = "4GB"]
            pub const MS_31: u32 = 0x1f;
        }
    }
    #[doc = "Base Address"]
    pub mod BA {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x000f_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Base Register 1 (For SDRAM CS1 device)"]
pub mod BR1 {
    #[doc = "Valid"]
    pub mod VLD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Memory size"]
    pub mod MS {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "4KB"]
            pub const MS_0: u32 = 0;
            #[doc = "8KB"]
            pub const MS_1: u32 = 0x01;
            #[doc = "16KB"]
            pub const MS_2: u32 = 0x02;
            #[doc = "32KB"]
            pub const MS_3: u32 = 0x03;
            #[doc = "64KB"]
            pub const MS_4: u32 = 0x04;
            #[doc = "128KB"]
            pub const MS_5: u32 = 0x05;
            #[doc = "256KB"]
            pub const MS_6: u32 = 0x06;
            #[doc = "512KB"]
            pub const MS_7: u32 = 0x07;
            #[doc = "1MB"]
            pub const MS_8: u32 = 0x08;
            #[doc = "2MB"]
            pub const MS_9: u32 = 0x09;
            #[doc = "4MB"]
            pub const MS_10: u32 = 0x0a;
            #[doc = "8MB"]
            pub const MS_11: u32 = 0x0b;
            #[doc = "16MB"]
            pub const MS_12: u32 = 0x0c;
            #[doc = "32MB"]
            pub const MS_13: u32 = 0x0d;
            #[doc = "64MB"]
            pub const MS_14: u32 = 0x0e;
            #[doc = "128MB"]
            pub const MS_15: u32 = 0x0f;
            #[doc = "256MB"]
            pub const MS_16: u32 = 0x10;
            #[doc = "512MB"]
            pub const MS_17: u32 = 0x11;
            #[doc = "1GB"]
            pub const MS_18: u32 = 0x12;
            #[doc = "2GB"]
            pub const MS_19: u32 = 0x13;
            #[doc = "4GB"]
            pub const MS_20: u32 = 0x14;
            #[doc = "4GB"]
            pub const MS_21: u32 = 0x15;
            #[doc = "4GB"]
            pub const MS_22: u32 = 0x16;
            #[doc = "4GB"]
            pub const MS_23: u32 = 0x17;
            #[doc = "4GB"]
            pub const MS_24: u32 = 0x18;
            #[doc = "4GB"]
            pub const MS_25: u32 = 0x19;
            #[doc = "4GB"]
            pub const MS_26: u32 = 0x1a;
            #[doc = "4GB"]
            pub const MS_27: u32 = 0x1b;
            #[doc = "4GB"]
            pub const MS_28: u32 = 0x1c;
            #[doc = "4GB"]
            pub const MS_29: u32 = 0x1d;
            #[doc = "4GB"]
            pub const MS_30: u32 = 0x1e;
            #[doc = "4GB"]
            pub const MS_31: u32 = 0x1f;
        }
    }
    #[doc = "Base Address"]
    pub mod BA {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x000f_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Base Register 2 (For SDRAM CS2 device)"]
pub mod BR2 {
    #[doc = "Valid"]
    pub mod VLD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Memory size"]
    pub mod MS {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "4KB"]
            pub const MS_0: u32 = 0;
            #[doc = "8KB"]
            pub const MS_1: u32 = 0x01;
            #[doc = "16KB"]
            pub const MS_2: u32 = 0x02;
            #[doc = "32KB"]
            pub const MS_3: u32 = 0x03;
            #[doc = "64KB"]
            pub const MS_4: u32 = 0x04;
            #[doc = "128KB"]
            pub const MS_5: u32 = 0x05;
            #[doc = "256KB"]
            pub const MS_6: u32 = 0x06;
            #[doc = "512KB"]
            pub const MS_7: u32 = 0x07;
            #[doc = "1MB"]
            pub const MS_8: u32 = 0x08;
            #[doc = "2MB"]
            pub const MS_9: u32 = 0x09;
            #[doc = "4MB"]
            pub const MS_10: u32 = 0x0a;
            #[doc = "8MB"]
            pub const MS_11: u32 = 0x0b;
            #[doc = "16MB"]
            pub const MS_12: u32 = 0x0c;
            #[doc = "32MB"]
            pub const MS_13: u32 = 0x0d;
            #[doc = "64MB"]
            pub const MS_14: u32 = 0x0e;
            #[doc = "128MB"]
            pub const MS_15: u32 = 0x0f;
            #[doc = "256MB"]
            pub const MS_16: u32 = 0x10;
            #[doc = "512MB"]
            pub const MS_17: u32 = 0x11;
            #[doc = "1GB"]
            pub const MS_18: u32 = 0x12;
            #[doc = "2GB"]
            pub const MS_19: u32 = 0x13;
            #[doc = "4GB"]
            pub const MS_20: u32 = 0x14;
            #[doc = "4GB"]
            pub const MS_21: u32 = 0x15;
            #[doc = "4GB"]
            pub const MS_22: u32 = 0x16;
            #[doc = "4GB"]
            pub const MS_23: u32 = 0x17;
            #[doc = "4GB"]
            pub const MS_24: u32 = 0x18;
            #[doc = "4GB"]
            pub const MS_25: u32 = 0x19;
            #[doc = "4GB"]
            pub const MS_26: u32 = 0x1a;
            #[doc = "4GB"]
            pub const MS_27: u32 = 0x1b;
            #[doc = "4GB"]
            pub const MS_28: u32 = 0x1c;
            #[doc = "4GB"]
            pub const MS_29: u32 = 0x1d;
            #[doc = "4GB"]
            pub const MS_30: u32 = 0x1e;
            #[doc = "4GB"]
            pub const MS_31: u32 = 0x1f;
        }
    }
    #[doc = "Base Address"]
    pub mod BA {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x000f_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Base Register 3 (For SDRAM CS3 device)"]
pub mod BR3 {
    #[doc = "Valid"]
    pub mod VLD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Memory size"]
    pub mod MS {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "4KB"]
            pub const MS_0: u32 = 0;
            #[doc = "8KB"]
            pub const MS_1: u32 = 0x01;
            #[doc = "16KB"]
            pub const MS_2: u32 = 0x02;
            #[doc = "32KB"]
            pub const MS_3: u32 = 0x03;
            #[doc = "64KB"]
            pub const MS_4: u32 = 0x04;
            #[doc = "128KB"]
            pub const MS_5: u32 = 0x05;
            #[doc = "256KB"]
            pub const MS_6: u32 = 0x06;
            #[doc = "512KB"]
            pub const MS_7: u32 = 0x07;
            #[doc = "1MB"]
            pub const MS_8: u32 = 0x08;
            #[doc = "2MB"]
            pub const MS_9: u32 = 0x09;
            #[doc = "4MB"]
            pub const MS_10: u32 = 0x0a;
            #[doc = "8MB"]
            pub const MS_11: u32 = 0x0b;
            #[doc = "16MB"]
            pub const MS_12: u32 = 0x0c;
            #[doc = "32MB"]
            pub const MS_13: u32 = 0x0d;
            #[doc = "64MB"]
            pub const MS_14: u32 = 0x0e;
            #[doc = "128MB"]
            pub const MS_15: u32 = 0x0f;
            #[doc = "256MB"]
            pub const MS_16: u32 = 0x10;
            #[doc = "512MB"]
            pub const MS_17: u32 = 0x11;
            #[doc = "1GB"]
            pub const MS_18: u32 = 0x12;
            #[doc = "2GB"]
            pub const MS_19: u32 = 0x13;
            #[doc = "4GB"]
            pub const MS_20: u32 = 0x14;
            #[doc = "4GB"]
            pub const MS_21: u32 = 0x15;
            #[doc = "4GB"]
            pub const MS_22: u32 = 0x16;
            #[doc = "4GB"]
            pub const MS_23: u32 = 0x17;
            #[doc = "4GB"]
            pub const MS_24: u32 = 0x18;
            #[doc = "4GB"]
            pub const MS_25: u32 = 0x19;
            #[doc = "4GB"]
            pub const MS_26: u32 = 0x1a;
            #[doc = "4GB"]
            pub const MS_27: u32 = 0x1b;
            #[doc = "4GB"]
            pub const MS_28: u32 = 0x1c;
            #[doc = "4GB"]
            pub const MS_29: u32 = 0x1d;
            #[doc = "4GB"]
            pub const MS_30: u32 = 0x1e;
            #[doc = "4GB"]
            pub const MS_31: u32 = 0x1f;
        }
    }
    #[doc = "Base Address"]
    pub mod BA {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x000f_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Base Register 4 (For NAND device)"]
pub mod BR4 {
    #[doc = "Valid"]
    pub mod VLD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Memory size"]
    pub mod MS {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "4KB"]
            pub const MS_0: u32 = 0;
            #[doc = "8KB"]
            pub const MS_1: u32 = 0x01;
            #[doc = "16KB"]
            pub const MS_2: u32 = 0x02;
            #[doc = "32KB"]
            pub const MS_3: u32 = 0x03;
            #[doc = "64KB"]
            pub const MS_4: u32 = 0x04;
            #[doc = "128KB"]
            pub const MS_5: u32 = 0x05;
            #[doc = "256KB"]
            pub const MS_6: u32 = 0x06;
            #[doc = "512KB"]
            pub const MS_7: u32 = 0x07;
            #[doc = "1MB"]
            pub const MS_8: u32 = 0x08;
            #[doc = "2MB"]
            pub const MS_9: u32 = 0x09;
            #[doc = "4MB"]
            pub const MS_10: u32 = 0x0a;
            #[doc = "8MB"]
            pub const MS_11: u32 = 0x0b;
            #[doc = "16MB"]
            pub const MS_12: u32 = 0x0c;
            #[doc = "32MB"]
            pub const MS_13: u32 = 0x0d;
            #[doc = "64MB"]
            pub const MS_14: u32 = 0x0e;
            #[doc = "128MB"]
            pub const MS_15: u32 = 0x0f;
            #[doc = "256MB"]
            pub const MS_16: u32 = 0x10;
            #[doc = "512MB"]
            pub const MS_17: u32 = 0x11;
            #[doc = "1GB"]
            pub const MS_18: u32 = 0x12;
            #[doc = "2GB"]
            pub const MS_19: u32 = 0x13;
            #[doc = "4GB"]
            pub const MS_20: u32 = 0x14;
            #[doc = "4GB"]
            pub const MS_21: u32 = 0x15;
            #[doc = "4GB"]
            pub const MS_22: u32 = 0x16;
            #[doc = "4GB"]
            pub const MS_23: u32 = 0x17;
            #[doc = "4GB"]
            pub const MS_24: u32 = 0x18;
            #[doc = "4GB"]
            pub const MS_25: u32 = 0x19;
            #[doc = "4GB"]
            pub const MS_26: u32 = 0x1a;
            #[doc = "4GB"]
            pub const MS_27: u32 = 0x1b;
            #[doc = "4GB"]
            pub const MS_28: u32 = 0x1c;
            #[doc = "4GB"]
            pub const MS_29: u32 = 0x1d;
            #[doc = "4GB"]
            pub const MS_30: u32 = 0x1e;
            #[doc = "4GB"]
            pub const MS_31: u32 = 0x1f;
        }
    }
    #[doc = "Base Address"]
    pub mod BA {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x000f_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Base Register 5 (For NOR device)"]
pub mod BR5 {
    #[doc = "Valid"]
    pub mod VLD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Memory size"]
    pub mod MS {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "4KB"]
            pub const MS_0: u32 = 0;
            #[doc = "8KB"]
            pub const MS_1: u32 = 0x01;
            #[doc = "16KB"]
            pub const MS_2: u32 = 0x02;
            #[doc = "32KB"]
            pub const MS_3: u32 = 0x03;
            #[doc = "64KB"]
            pub const MS_4: u32 = 0x04;
            #[doc = "128KB"]
            pub const MS_5: u32 = 0x05;
            #[doc = "256KB"]
            pub const MS_6: u32 = 0x06;
            #[doc = "512KB"]
            pub const MS_7: u32 = 0x07;
            #[doc = "1MB"]
            pub const MS_8: u32 = 0x08;
            #[doc = "2MB"]
            pub const MS_9: u32 = 0x09;
            #[doc = "4MB"]
            pub const MS_10: u32 = 0x0a;
            #[doc = "8MB"]
            pub const MS_11: u32 = 0x0b;
            #[doc = "16MB"]
            pub const MS_12: u32 = 0x0c;
            #[doc = "32MB"]
            pub const MS_13: u32 = 0x0d;
            #[doc = "64MB"]
            pub const MS_14: u32 = 0x0e;
            #[doc = "128MB"]
            pub const MS_15: u32 = 0x0f;
            #[doc = "256MB"]
            pub const MS_16: u32 = 0x10;
            #[doc = "512MB"]
            pub const MS_17: u32 = 0x11;
            #[doc = "1GB"]
            pub const MS_18: u32 = 0x12;
            #[doc = "2GB"]
            pub const MS_19: u32 = 0x13;
            #[doc = "4GB"]
            pub const MS_20: u32 = 0x14;
            #[doc = "4GB"]
            pub const MS_21: u32 = 0x15;
            #[doc = "4GB"]
            pub const MS_22: u32 = 0x16;
            #[doc = "4GB"]
            pub const MS_23: u32 = 0x17;
            #[doc = "4GB"]
            pub const MS_24: u32 = 0x18;
            #[doc = "4GB"]
            pub const MS_25: u32 = 0x19;
            #[doc = "4GB"]
            pub const MS_26: u32 = 0x1a;
            #[doc = "4GB"]
            pub const MS_27: u32 = 0x1b;
            #[doc = "4GB"]
            pub const MS_28: u32 = 0x1c;
            #[doc = "4GB"]
            pub const MS_29: u32 = 0x1d;
            #[doc = "4GB"]
            pub const MS_30: u32 = 0x1e;
            #[doc = "4GB"]
            pub const MS_31: u32 = 0x1f;
        }
    }
    #[doc = "Base Address"]
    pub mod BA {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x000f_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Base Register 6 (For PSRAM device)"]
pub mod BR6 {
    #[doc = "Valid"]
    pub mod VLD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Memory size"]
    pub mod MS {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "4KB"]
            pub const MS_0: u32 = 0;
            #[doc = "8KB"]
            pub const MS_1: u32 = 0x01;
            #[doc = "16KB"]
            pub const MS_2: u32 = 0x02;
            #[doc = "32KB"]
            pub const MS_3: u32 = 0x03;
            #[doc = "64KB"]
            pub const MS_4: u32 = 0x04;
            #[doc = "128KB"]
            pub const MS_5: u32 = 0x05;
            #[doc = "256KB"]
            pub const MS_6: u32 = 0x06;
            #[doc = "512KB"]
            pub const MS_7: u32 = 0x07;
            #[doc = "1MB"]
            pub const MS_8: u32 = 0x08;
            #[doc = "2MB"]
            pub const MS_9: u32 = 0x09;
            #[doc = "4MB"]
            pub const MS_10: u32 = 0x0a;
            #[doc = "8MB"]
            pub const MS_11: u32 = 0x0b;
            #[doc = "16MB"]
            pub const MS_12: u32 = 0x0c;
            #[doc = "32MB"]
            pub const MS_13: u32 = 0x0d;
            #[doc = "64MB"]
            pub const MS_14: u32 = 0x0e;
            #[doc = "128MB"]
            pub const MS_15: u32 = 0x0f;
            #[doc = "256MB"]
            pub const MS_16: u32 = 0x10;
            #[doc = "512MB"]
            pub const MS_17: u32 = 0x11;
            #[doc = "1GB"]
            pub const MS_18: u32 = 0x12;
            #[doc = "2GB"]
            pub const MS_19: u32 = 0x13;
            #[doc = "4GB"]
            pub const MS_20: u32 = 0x14;
            #[doc = "4GB"]
            pub const MS_21: u32 = 0x15;
            #[doc = "4GB"]
            pub const MS_22: u32 = 0x16;
            #[doc = "4GB"]
            pub const MS_23: u32 = 0x17;
            #[doc = "4GB"]
            pub const MS_24: u32 = 0x18;
            #[doc = "4GB"]
            pub const MS_25: u32 = 0x19;
            #[doc = "4GB"]
            pub const MS_26: u32 = 0x1a;
            #[doc = "4GB"]
            pub const MS_27: u32 = 0x1b;
            #[doc = "4GB"]
            pub const MS_28: u32 = 0x1c;
            #[doc = "4GB"]
            pub const MS_29: u32 = 0x1d;
            #[doc = "4GB"]
            pub const MS_30: u32 = 0x1e;
            #[doc = "4GB"]
            pub const MS_31: u32 = 0x1f;
        }
    }
    #[doc = "Base Address"]
    pub mod BA {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x000f_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Base Register 7 (For DBI-B (MIPI Display Bus Interface Type B) device)"]
pub mod BR7 {
    #[doc = "Valid"]
    pub mod VLD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Memory size"]
    pub mod MS {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "4KB"]
            pub const MS_0: u32 = 0;
            #[doc = "8KB"]
            pub const MS_1: u32 = 0x01;
            #[doc = "16KB"]
            pub const MS_2: u32 = 0x02;
            #[doc = "32KB"]
            pub const MS_3: u32 = 0x03;
            #[doc = "64KB"]
            pub const MS_4: u32 = 0x04;
            #[doc = "128KB"]
            pub const MS_5: u32 = 0x05;
            #[doc = "256KB"]
            pub const MS_6: u32 = 0x06;
            #[doc = "512KB"]
            pub const MS_7: u32 = 0x07;
            #[doc = "1MB"]
            pub const MS_8: u32 = 0x08;
            #[doc = "2MB"]
            pub const MS_9: u32 = 0x09;
            #[doc = "4MB"]
            pub const MS_10: u32 = 0x0a;
            #[doc = "8MB"]
            pub const MS_11: u32 = 0x0b;
            #[doc = "16MB"]
            pub const MS_12: u32 = 0x0c;
            #[doc = "32MB"]
            pub const MS_13: u32 = 0x0d;
            #[doc = "64MB"]
            pub const MS_14: u32 = 0x0e;
            #[doc = "128MB"]
            pub const MS_15: u32 = 0x0f;
            #[doc = "256MB"]
            pub const MS_16: u32 = 0x10;
            #[doc = "512MB"]
            pub const MS_17: u32 = 0x11;
            #[doc = "1GB"]
            pub const MS_18: u32 = 0x12;
            #[doc = "2GB"]
            pub const MS_19: u32 = 0x13;
            #[doc = "4GB"]
            pub const MS_20: u32 = 0x14;
            #[doc = "4GB"]
            pub const MS_21: u32 = 0x15;
            #[doc = "4GB"]
            pub const MS_22: u32 = 0x16;
            #[doc = "4GB"]
            pub const MS_23: u32 = 0x17;
            #[doc = "4GB"]
            pub const MS_24: u32 = 0x18;
            #[doc = "4GB"]
            pub const MS_25: u32 = 0x19;
            #[doc = "4GB"]
            pub const MS_26: u32 = 0x1a;
            #[doc = "4GB"]
            pub const MS_27: u32 = 0x1b;
            #[doc = "4GB"]
            pub const MS_28: u32 = 0x1c;
            #[doc = "4GB"]
            pub const MS_29: u32 = 0x1d;
            #[doc = "4GB"]
            pub const MS_30: u32 = 0x1e;
            #[doc = "4GB"]
            pub const MS_31: u32 = 0x1f;
        }
    }
    #[doc = "Base Address"]
    pub mod BA {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x000f_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Base Register 8 (For NAND device)"]
pub mod BR8 {
    #[doc = "Valid"]
    pub mod VLD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Memory size"]
    pub mod MS {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "4KB"]
            pub const MS_0: u32 = 0;
            #[doc = "8KB"]
            pub const MS_1: u32 = 0x01;
            #[doc = "16KB"]
            pub const MS_2: u32 = 0x02;
            #[doc = "32KB"]
            pub const MS_3: u32 = 0x03;
            #[doc = "64KB"]
            pub const MS_4: u32 = 0x04;
            #[doc = "128KB"]
            pub const MS_5: u32 = 0x05;
            #[doc = "256KB"]
            pub const MS_6: u32 = 0x06;
            #[doc = "512KB"]
            pub const MS_7: u32 = 0x07;
            #[doc = "1MB"]
            pub const MS_8: u32 = 0x08;
            #[doc = "2MB"]
            pub const MS_9: u32 = 0x09;
            #[doc = "4MB"]
            pub const MS_10: u32 = 0x0a;
            #[doc = "8MB"]
            pub const MS_11: u32 = 0x0b;
            #[doc = "16MB"]
            pub const MS_12: u32 = 0x0c;
            #[doc = "32MB"]
            pub const MS_13: u32 = 0x0d;
            #[doc = "64MB"]
            pub const MS_14: u32 = 0x0e;
            #[doc = "128MB"]
            pub const MS_15: u32 = 0x0f;
            #[doc = "256MB"]
            pub const MS_16: u32 = 0x10;
            #[doc = "512MB"]
            pub const MS_17: u32 = 0x11;
            #[doc = "1GB"]
            pub const MS_18: u32 = 0x12;
            #[doc = "2GB"]
            pub const MS_19: u32 = 0x13;
            #[doc = "4GB"]
            pub const MS_20: u32 = 0x14;
            #[doc = "4GB"]
            pub const MS_21: u32 = 0x15;
            #[doc = "4GB"]
            pub const MS_22: u32 = 0x16;
            #[doc = "4GB"]
            pub const MS_23: u32 = 0x17;
            #[doc = "4GB"]
            pub const MS_24: u32 = 0x18;
            #[doc = "4GB"]
            pub const MS_25: u32 = 0x19;
            #[doc = "4GB"]
            pub const MS_26: u32 = 0x1a;
            #[doc = "4GB"]
            pub const MS_27: u32 = 0x1b;
            #[doc = "4GB"]
            pub const MS_28: u32 = 0x1c;
            #[doc = "4GB"]
            pub const MS_29: u32 = 0x1d;
            #[doc = "4GB"]
            pub const MS_30: u32 = 0x1e;
            #[doc = "4GB"]
            pub const MS_31: u32 = 0x1f;
        }
    }
    #[doc = "Base Address"]
    pub mod BA {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x000f_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DLL Control Register"]
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
    #[doc = "The delay target for slave delay line is: ((SLVDLYTARGET+1) * 1/32 * clock cycle of reference clock (ipgclock)."]
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
#[doc = "Interrupt Enable Register"]
pub mod INTEN {
    #[doc = "IP command done interrupt enable"]
    pub mod IPCMDDONEEN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "IP command error interrupt enable"]
    pub mod IPCMDERREN {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "AXI command error interrupt enable"]
    pub mod AXICMDERREN {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "AXI bus error interrupt enable"]
    pub mod AXIBUSERREN {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bit enable/disable the NDPAGEEND interrupt generation."]
    pub mod NDPAGEENDEN {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const NDPAGEENDEN_0: u32 = 0;
            #[doc = "Enable"]
            pub const NDPAGEENDEN_1: u32 = 0x01;
        }
    }
    #[doc = "This bit enable/disable the NDNOPEND interrupt generation."]
    pub mod NDNOPENDEN {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const NDNOPENDEN_0: u32 = 0;
            #[doc = "Enable"]
            pub const NDNOPENDEN_1: u32 = 0x01;
        }
    }
}
#[doc = "Interrupt Enable Register"]
pub mod INTR {
    #[doc = "IP command normal done interrupt"]
    pub mod IPCMDDONE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "IP command error done interrupt"]
    pub mod IPCMDERR {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "AXI command error interrupt"]
    pub mod AXICMDERR {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "AXI bus error interrupt"]
    pub mod AXIBUSERR {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This interrupt is generated when the last address of one page in NAND device is written by AXI command"]
    pub mod NDPAGEEND {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This interrupt is generated when all pending AXI write command to NAND is finished on NAND interface."]
    pub mod NDNOPEND {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SDRAM control register 0"]
pub mod SDRAMCR0 {
    #[doc = "Port Size"]
    pub mod PS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "8bit"]
            pub const PS_0: u32 = 0;
            #[doc = "16bit"]
            pub const PS_1: u32 = 0x01;
        }
    }
    #[doc = "Burst Length"]
    pub mod BL {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "1"]
            pub const BL_0: u32 = 0;
            #[doc = "2"]
            pub const BL_1: u32 = 0x01;
            #[doc = "4"]
            pub const BL_2: u32 = 0x02;
            #[doc = "8"]
            pub const BL_3: u32 = 0x03;
            #[doc = "8"]
            pub const BL_4: u32 = 0x04;
            #[doc = "8"]
            pub const BL_5: u32 = 0x05;
            #[doc = "8"]
            pub const BL_6: u32 = 0x06;
            #[doc = "8"]
            pub const BL_7: u32 = 0x07;
        }
    }
    #[doc = "Column 8 selection bit"]
    pub mod COL8 {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Column address bit number is decided by COL field."]
            pub const COL8_0: u32 = 0;
            #[doc = "Column address bit number is 8. COL field is ignored."]
            pub const COL8_1: u32 = 0x01;
        }
    }
    #[doc = "Column address bit number"]
    pub mod COL {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "12 bit"]
            pub const COL_0: u32 = 0;
            #[doc = "11 bit"]
            pub const COL_1: u32 = 0x01;
            #[doc = "10 bit"]
            pub const COL_2: u32 = 0x02;
            #[doc = "9 bit"]
            pub const COL_3: u32 = 0x03;
        }
    }
    #[doc = "CAS Latency"]
    pub mod CL {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "1"]
            pub const CL_0: u32 = 0;
            #[doc = "1"]
            pub const CL_1: u32 = 0x01;
            #[doc = "2"]
            pub const CL_2: u32 = 0x02;
            #[doc = "3"]
            pub const CL_3: u32 = 0x03;
        }
    }
    #[doc = "2 Bank selection bit"]
    pub mod BANK2 {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "SDRAM device has 4 banks."]
            pub const BANK2_0: u32 = 0;
            #[doc = "SDRAM device has 2 banks."]
            pub const BANK2_1: u32 = 0x01;
        }
    }
}
#[doc = "SDRAM control register 1"]
pub mod SDRAMCR1 {
    #[doc = "PRECHARGE to ACT/Refresh wait time"]
    pub mod PRE2ACT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ACT to Read/Write wait time"]
    pub mod ACT2RW {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Refresh recovery time"]
    pub mod RFRC {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Write recovery time"]
    pub mod WRC {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CKE OFF minimum time"]
    pub mod CKEOFF {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ACT to Precharge minimum time"]
    pub mod ACT2PRE {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SDRAM control register 2"]
pub mod SDRAMCR2 {
    #[doc = "Self Refresh Recovery time"]
    pub mod SRRC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Refresh to Refresh wait time"]
    pub mod REF2REF {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ACT to ACT wait time"]
    pub mod ACT2ACT {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "SDRAM Idle timeout"]
    pub mod ITO {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "IDLE timeout period is 256*Prescale period."]
            pub const ITO_0: u32 = 0;
            #[doc = "IDLE timeout period is ITO*Prescale period."]
            pub const ITO_1: u32 = 0x01;
            #[doc = "IDLE timeout period is ITO*Prescale period."]
            pub const ITO_2: u32 = 0x02;
            #[doc = "IDLE timeout period is ITO*Prescale period."]
            pub const ITO_3: u32 = 0x03;
            #[doc = "IDLE timeout period is ITO*Prescale period."]
            pub const ITO_4: u32 = 0x04;
            #[doc = "IDLE timeout period is ITO*Prescale period."]
            pub const ITO_5: u32 = 0x05;
            #[doc = "IDLE timeout period is ITO*Prescale period."]
            pub const ITO_6: u32 = 0x06;
            #[doc = "IDLE timeout period is ITO*Prescale period."]
            pub const ITO_7: u32 = 0x07;
            #[doc = "IDLE timeout period is ITO*Prescale period."]
            pub const ITO_8: u32 = 0x08;
            #[doc = "IDLE timeout period is ITO*Prescale period."]
            pub const ITO_9: u32 = 0x09;
        }
    }
}
#[doc = "SDRAM control register 3"]
pub mod SDRAMCR3 {
    #[doc = "Refresh enable"]
    pub mod REN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Refresh burst length"]
    pub mod REBL {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "1"]
            pub const REBL_0: u32 = 0;
            #[doc = "2"]
            pub const REBL_1: u32 = 0x01;
            #[doc = "3"]
            pub const REBL_2: u32 = 0x02;
            #[doc = "4"]
            pub const REBL_3: u32 = 0x03;
            #[doc = "5"]
            pub const REBL_4: u32 = 0x04;
            #[doc = "6"]
            pub const REBL_5: u32 = 0x05;
            #[doc = "7"]
            pub const REBL_6: u32 = 0x06;
            #[doc = "8"]
            pub const REBL_7: u32 = 0x07;
        }
    }
    #[doc = "Prescaler timer period"]
    pub mod PRESCALE {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "256*16 cycle"]
            pub const PRESCALE_0: u32 = 0;
            #[doc = "PRESCALE*16 cycle"]
            pub const PRESCALE_1: u32 = 0x01;
            #[doc = "PRESCALE*16 cycle"]
            pub const PRESCALE_2: u32 = 0x02;
            #[doc = "PRESCALE*16 cycle"]
            pub const PRESCALE_3: u32 = 0x03;
            #[doc = "PRESCALE*16 cycle"]
            pub const PRESCALE_4: u32 = 0x04;
            #[doc = "PRESCALE*16 cycle"]
            pub const PRESCALE_5: u32 = 0x05;
            #[doc = "PRESCALE*16 cycle"]
            pub const PRESCALE_6: u32 = 0x06;
            #[doc = "PRESCALE*16 cycle"]
            pub const PRESCALE_7: u32 = 0x07;
            #[doc = "PRESCALE*16 cycle"]
            pub const PRESCALE_8: u32 = 0x08;
            #[doc = "PRESCALE*16 cycle"]
            pub const PRESCALE_9: u32 = 0x09;
        }
    }
    #[doc = "Refresh timer period"]
    pub mod RT {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "256*Prescaler period"]
            pub const RT_0: u32 = 0;
            #[doc = "RT*Prescaler period"]
            pub const RT_1: u32 = 0x01;
            #[doc = "RT*Prescaler period"]
            pub const RT_2: u32 = 0x02;
            #[doc = "RT*Prescaler period"]
            pub const RT_3: u32 = 0x03;
            #[doc = "RT*Prescaler period"]
            pub const RT_4: u32 = 0x04;
            #[doc = "RT*Prescaler period"]
            pub const RT_5: u32 = 0x05;
            #[doc = "RT*Prescaler period"]
            pub const RT_6: u32 = 0x06;
            #[doc = "RT*Prescaler period"]
            pub const RT_7: u32 = 0x07;
            #[doc = "RT*Prescaler period"]
            pub const RT_8: u32 = 0x08;
            #[doc = "RT*Prescaler period"]
            pub const RT_9: u32 = 0x09;
        }
    }
    #[doc = "Refresh urgent threshold"]
    pub mod UT {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "256*Prescaler period"]
            pub const UT_0: u32 = 0;
            #[doc = "UT*Prescaler period"]
            pub const UT_1: u32 = 0x01;
            #[doc = "UT*Prescaler period"]
            pub const UT_2: u32 = 0x02;
            #[doc = "UT*Prescaler period"]
            pub const UT_3: u32 = 0x03;
            #[doc = "UT*Prescaler period"]
            pub const UT_4: u32 = 0x04;
            #[doc = "UT*Prescaler period"]
            pub const UT_5: u32 = 0x05;
            #[doc = "UT*Prescaler period"]
            pub const UT_6: u32 = 0x06;
            #[doc = "UT*Prescaler period"]
            pub const UT_7: u32 = 0x07;
            #[doc = "UT*Prescaler period"]
            pub const UT_8: u32 = 0x08;
            #[doc = "UT*Prescaler period"]
            pub const UT_9: u32 = 0x09;
        }
    }
}
#[doc = "NAND control register 0"]
pub mod NANDCR0 {
    #[doc = "Port Size"]
    pub mod PS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "8bit"]
            pub const PS_0: u32 = 0;
            #[doc = "16bit"]
            pub const PS_1: u32 = 0x01;
        }
    }
    #[doc = "Select NAND controller mode."]
    pub mod SYNCEN {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Asynchronous mode is enabled."]
            pub const SYNCEN_0: u32 = 0;
            #[doc = "Synchronous mode is enabled."]
            pub const SYNCEN_1: u32 = 0x01;
        }
    }
    #[doc = "Burst Length"]
    pub mod BL {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "1"]
            pub const BL_0: u32 = 0;
            #[doc = "2"]
            pub const BL_1: u32 = 0x01;
            #[doc = "4"]
            pub const BL_2: u32 = 0x02;
            #[doc = "8"]
            pub const BL_3: u32 = 0x03;
            #[doc = "16"]
            pub const BL_4: u32 = 0x04;
            #[doc = "32"]
            pub const BL_5: u32 = 0x05;
            #[doc = "64"]
            pub const BL_6: u32 = 0x06;
            #[doc = "64"]
            pub const BL_7: u32 = 0x07;
        }
    }
    #[doc = "EDO mode enabled"]
    pub mod EDO {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "EDO mode disabled"]
            pub const EDO_0: u32 = 0;
            #[doc = "EDO mode enabled"]
            pub const EDO_1: u32 = 0x01;
        }
    }
    #[doc = "Column address bit number"]
    pub mod COL {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "16"]
            pub const COL_0: u32 = 0;
            #[doc = "15"]
            pub const COL_1: u32 = 0x01;
            #[doc = "14"]
            pub const COL_2: u32 = 0x02;
            #[doc = "13"]
            pub const COL_3: u32 = 0x03;
            #[doc = "12"]
            pub const COL_4: u32 = 0x04;
            #[doc = "11"]
            pub const COL_5: u32 = 0x05;
            #[doc = "10"]
            pub const COL_6: u32 = 0x06;
            #[doc = "9"]
            pub const COL_7: u32 = 0x07;
        }
    }
}
#[doc = "NAND control register 1"]
pub mod NANDCR1 {
    #[doc = "CE setup time"]
    pub mod CES {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CE hold time"]
    pub mod CEH {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "WE# LOW time"]
    pub mod WEL {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "WE# HIGH time"]
    pub mod WEH {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "RE# LOW time"]
    pub mod REL {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "RE# HIGH time"]
    pub mod REH {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Turnaround time"]
    pub mod TA {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CE# interval time"]
    pub mod CEITV {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "NAND control register 2"]
pub mod NANDCR2 {
    #[doc = "WE# HIGH to RE# LOW wait time"]
    pub mod TWHR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "RE# HIGH to WE# LOW wait time"]
    pub mod TRHW {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ALE to WRITE Data start wait time"]
    pub mod TADL {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Ready to RE# LOW min wait time"]
    pub mod TRR {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "WE# HIGH to busy wait time"]
    pub mod TWB {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "NAND control register 3"]
pub mod NANDCR3 {
    #[doc = "NAND option bit 1"]
    pub mod NDOPT1 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "NAND option bit 2"]
    pub mod NDOPT2 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "NAND option bit 3"]
    pub mod NDOPT3 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "NAND CLE Option"]
    pub mod CLE {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Read Data Setup cycle time."]
    pub mod RDS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Read Data Hold cycle time."]
    pub mod RDH {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Write Data Setup cycle time."]
    pub mod WDS {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Write Data Hold cycle time."]
    pub mod WDH {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "NOR control register 0"]
pub mod NORCR0 {
    #[doc = "Port Size"]
    pub mod PS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "8bit"]
            pub const PS_0: u32 = 0;
            #[doc = "16bit"]
            pub const PS_1: u32 = 0x01;
        }
    }
    #[doc = "Select NOR controller mode."]
    pub mod SYNCEN {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Asynchronous mode is enabled."]
            pub const SYNCEN_0: u32 = 0;
            #[doc = "Synchronous mode is enabled."]
            pub const SYNCEN_1: u32 = 0x01;
        }
    }
    #[doc = "Burst Length"]
    pub mod BL {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "1"]
            pub const BL_0: u32 = 0;
            #[doc = "2"]
            pub const BL_1: u32 = 0x01;
            #[doc = "4"]
            pub const BL_2: u32 = 0x02;
            #[doc = "8"]
            pub const BL_3: u32 = 0x03;
            #[doc = "16"]
            pub const BL_4: u32 = 0x04;
            #[doc = "32"]
            pub const BL_5: u32 = 0x05;
            #[doc = "64"]
            pub const BL_6: u32 = 0x06;
            #[doc = "64"]
            pub const BL_7: u32 = 0x07;
        }
    }
    #[doc = "Address Mode"]
    pub mod AM {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Address/Data MUX mode"]
            pub const AM_0: u32 = 0;
            #[doc = "Advanced Address/Data MUX mode"]
            pub const AM_1: u32 = 0x01;
            #[doc = "Address/Data non-MUX mode"]
            pub const AM_2: u32 = 0x02;
            #[doc = "Address/Data non-MUX mode"]
            pub const AM_3: u32 = 0x03;
        }
    }
    #[doc = "ADV# polarity"]
    pub mod ADVP {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ADV# is Low Active. In ASYNC mode, device sample address with ADV# rise edge; In SYNC mode, device sample address when ADV# is LOW."]
            pub const ADVP_0: u32 = 0;
            #[doc = "ADV# is High Active. In ASYNC mode, device sample address with ADV# fall edge; In SYNC mode, device sample address when ADV# is HIGH."]
            pub const ADVP_1: u32 = 0x01;
        }
    }
    #[doc = "ADV# level control during address hold state"]
    pub mod ADVH {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ADV# is high during address hold state."]
            pub const ADVH_0: u32 = 0;
            #[doc = "ADV# is low during address hold state."]
            pub const ADVH_1: u32 = 0x01;
        }
    }
    #[doc = "Column Address bit width"]
    pub mod COL {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "12 Bits"]
            pub const COL_0: u32 = 0;
            #[doc = "11 Bits"]
            pub const COL_1: u32 = 0x01;
            #[doc = "10 Bits"]
            pub const COL_2: u32 = 0x02;
            #[doc = "9 Bits"]
            pub const COL_3: u32 = 0x03;
            #[doc = "8 Bits"]
            pub const COL_4: u32 = 0x04;
            #[doc = "7 Bits"]
            pub const COL_5: u32 = 0x05;
            #[doc = "6 Bits"]
            pub const COL_6: u32 = 0x06;
            #[doc = "5 Bits"]
            pub const COL_7: u32 = 0x07;
            #[doc = "4 Bits"]
            pub const COL_8: u32 = 0x08;
            #[doc = "3 Bits"]
            pub const COL_9: u32 = 0x09;
            #[doc = "2 Bits"]
            pub const COL_10: u32 = 0x0a;
            #[doc = "12 Bits"]
            pub const COL_11: u32 = 0x0b;
            #[doc = "12 Bits"]
            pub const COL_12: u32 = 0x0c;
            #[doc = "12 Bits"]
            pub const COL_13: u32 = 0x0d;
            #[doc = "12 Bits"]
            pub const COL_14: u32 = 0x0e;
            #[doc = "12 Bits"]
            pub const COL_15: u32 = 0x0f;
        }
    }
}
#[doc = "NOR control register 1"]
pub mod NORCR1 {
    #[doc = "CE setup time cycle"]
    pub mod CES {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CE hold min time (CEH+1) cycle"]
    pub mod CEH {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Address setup time"]
    pub mod AS {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Address hold time"]
    pub mod AH {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "WE LOW time (WEL+1) cycle"]
    pub mod WEL {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "WE HIGH time (WEH+1) cycle"]
    pub mod WEH {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "RE LOW time (REL+1) cycle"]
    pub mod REL {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "RE HIGH time (REH+1) cycle"]
    pub mod REH {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "NOR control register 2"]
pub mod NORCR2 {
    #[doc = "Turnaround time cycle"]
    pub mod TA {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Address to write data hold time cycle"]
    pub mod AWDH {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Latency count"]
    pub mod LC {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Read cycle time"]
    pub mod RD {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CE# interval min time"]
    pub mod CEITV {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Read cycle hold time"]
    pub mod RDH {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "NOR control register 3"]
pub mod NORCR3 {
    #[doc = "Address setup time for synchronous read"]
    pub mod ASSR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Address hold time for synchronous read"]
    pub mod AHSR {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SRAM control register 0"]
pub mod SRAMCR0 {
    #[doc = "Port Size"]
    pub mod PS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "8bit"]
            pub const PS_0: u32 = 0;
            #[doc = "16bit"]
            pub const PS_1: u32 = 0x01;
        }
    }
    #[doc = "Select SRAM controller mode."]
    pub mod SYNCEN {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Asynchronous mode is enabled."]
            pub const SYNCEN_0: u32 = 0;
            #[doc = "Synchronous mode is enabled."]
            pub const SYNCEN_1: u32 = 0x01;
        }
    }
    #[doc = "Burst Length"]
    pub mod BL {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "1"]
            pub const BL_0: u32 = 0;
            #[doc = "2"]
            pub const BL_1: u32 = 0x01;
            #[doc = "4"]
            pub const BL_2: u32 = 0x02;
            #[doc = "8"]
            pub const BL_3: u32 = 0x03;
            #[doc = "16"]
            pub const BL_4: u32 = 0x04;
            #[doc = "32"]
            pub const BL_5: u32 = 0x05;
            #[doc = "64"]
            pub const BL_6: u32 = 0x06;
            #[doc = "64"]
            pub const BL_7: u32 = 0x07;
        }
    }
    #[doc = "Address Mode"]
    pub mod AM {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Address/Data MUX mode"]
            pub const AM_0: u32 = 0;
            #[doc = "Advanced Address/Data MUX mode"]
            pub const AM_1: u32 = 0x01;
            #[doc = "Address/Data non-MUX mode"]
            pub const AM_2: u32 = 0x02;
            #[doc = "Address/Data non-MUX mode"]
            pub const AM_3: u32 = 0x03;
        }
    }
    #[doc = "ADV# polarity"]
    pub mod ADVP {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ADV# is Low Active. In ASYNC mode, device sample address with ADV# rise edge; In SYNC mode, device sample address when ADV# is LOW."]
            pub const ADVP_0: u32 = 0;
            #[doc = "ADV# is High Active. In ASYNC mode, device sample address with ADV# fall edge; In SYNC mode, device sample address when ADV# is HIGH."]
            pub const ADVP_1: u32 = 0x01;
        }
    }
    #[doc = "ADV# level control during address hold state"]
    pub mod ADVH {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ADV# is high during address hold state."]
            pub const ADVH_0: u32 = 0;
            #[doc = "ADV# is low during address hold state."]
            pub const ADVH_1: u32 = 0x01;
        }
    }
    #[doc = "Column Address bit width"]
    pub mod COL {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "12 Bits"]
            pub const COL_0: u32 = 0;
            #[doc = "11 Bits"]
            pub const COL_1: u32 = 0x01;
            #[doc = "10 Bits"]
            pub const COL_2: u32 = 0x02;
            #[doc = "9 Bits"]
            pub const COL_3: u32 = 0x03;
            #[doc = "8 Bits"]
            pub const COL_4: u32 = 0x04;
            #[doc = "7 Bits"]
            pub const COL_5: u32 = 0x05;
            #[doc = "6 Bits"]
            pub const COL_6: u32 = 0x06;
            #[doc = "5 Bits"]
            pub const COL_7: u32 = 0x07;
            #[doc = "4 Bits"]
            pub const COL_8: u32 = 0x08;
            #[doc = "3 Bits"]
            pub const COL_9: u32 = 0x09;
            #[doc = "2 Bits"]
            pub const COL_10: u32 = 0x0a;
            #[doc = "12 Bits"]
            pub const COL_11: u32 = 0x0b;
            #[doc = "12 Bits"]
            pub const COL_12: u32 = 0x0c;
            #[doc = "12 Bits"]
            pub const COL_13: u32 = 0x0d;
            #[doc = "12 Bits"]
            pub const COL_14: u32 = 0x0e;
            #[doc = "12 Bits"]
            pub const COL_15: u32 = 0x0f;
        }
    }
}
#[doc = "SRAM control register 1"]
pub mod SRAMCR1 {
    #[doc = "CE setup time cycle"]
    pub mod CES {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CE hold min time (CEH+1) cycle"]
    pub mod CEH {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Address setup time"]
    pub mod AS {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Address hold time"]
    pub mod AH {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "WE LOW time (WEL+1) cycle"]
    pub mod WEL {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "WE HIGH time (WEH+1) cycle"]
    pub mod WEH {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "RE LOW time (REL+1) cycle"]
    pub mod REL {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "RE HIGH time (REH+1) cycle"]
    pub mod REH {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SRAM control register 2"]
pub mod SRAMCR2 {
    #[doc = "Write Data setup time (WDS+1) cycle"]
    pub mod WDS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Write Data hold time WDH cycle"]
    pub mod WDH {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Turnaround time cycle"]
    pub mod TA {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Address to write data hold time cycle"]
    pub mod AWDH {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Latency count"]
    pub mod LC {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Read cycle time"]
    pub mod RD {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CE# interval min time"]
    pub mod CEITV {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Read cycle hold time"]
    pub mod RDH {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DBI-B control register 0"]
pub mod DBICR0 {
    #[doc = "Port Size"]
    pub mod PS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "8bit"]
            pub const PS_0: u32 = 0;
            #[doc = "16bit"]
            pub const PS_1: u32 = 0x01;
        }
    }
    #[doc = "Burst Length"]
    pub mod BL {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "1"]
            pub const BL_0: u32 = 0;
            #[doc = "2"]
            pub const BL_1: u32 = 0x01;
            #[doc = "4"]
            pub const BL_2: u32 = 0x02;
            #[doc = "8"]
            pub const BL_3: u32 = 0x03;
            #[doc = "16"]
            pub const BL_4: u32 = 0x04;
            #[doc = "32"]
            pub const BL_5: u32 = 0x05;
            #[doc = "64"]
            pub const BL_6: u32 = 0x06;
            #[doc = "64"]
            pub const BL_7: u32 = 0x07;
        }
    }
    #[doc = "Column Address bit width"]
    pub mod COL {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "12 Bits"]
            pub const COL_0: u32 = 0;
            #[doc = "11 Bits"]
            pub const COL_1: u32 = 0x01;
            #[doc = "10 Bits"]
            pub const COL_2: u32 = 0x02;
            #[doc = "9 Bits"]
            pub const COL_3: u32 = 0x03;
            #[doc = "8 Bits"]
            pub const COL_4: u32 = 0x04;
            #[doc = "7 Bits"]
            pub const COL_5: u32 = 0x05;
            #[doc = "6 Bits"]
            pub const COL_6: u32 = 0x06;
            #[doc = "5 Bits"]
            pub const COL_7: u32 = 0x07;
            #[doc = "4 Bits"]
            pub const COL_8: u32 = 0x08;
            #[doc = "3 Bits"]
            pub const COL_9: u32 = 0x09;
            #[doc = "2 Bits"]
            pub const COL_10: u32 = 0x0a;
            #[doc = "12 Bits"]
            pub const COL_11: u32 = 0x0b;
            #[doc = "12 Bits"]
            pub const COL_12: u32 = 0x0c;
            #[doc = "12 Bits"]
            pub const COL_13: u32 = 0x0d;
            #[doc = "12 Bits"]
            pub const COL_14: u32 = 0x0e;
            #[doc = "12 Bits"]
            pub const COL_15: u32 = 0x0f;
        }
    }
}
#[doc = "DBI-B control register 1"]
pub mod DBICR1 {
    #[doc = "CSX Setup Time"]
    pub mod CES {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CSX Hold Time"]
    pub mod CEH {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "WRX Low Time"]
    pub mod WEL {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "WRX High Time"]
    pub mod WEH {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "RDX Low Time"]
    pub mod REL {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "RDX High Time"]
    pub mod REH {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CSX interval min time"]
    pub mod CEITV {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "IP Command control register 0"]
pub mod IPCR0 {
    #[doc = "Slave address"]
    pub mod SA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "IP Command control register 1"]
pub mod IPCR1 {
    #[doc = "Data Size in Byte"]
    pub mod DATSZ {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "4"]
            pub const DATSZ_0: u32 = 0;
            #[doc = "1"]
            pub const DATSZ_1: u32 = 0x01;
            #[doc = "2"]
            pub const DATSZ_2: u32 = 0x02;
            #[doc = "3"]
            pub const DATSZ_3: u32 = 0x03;
            #[doc = "4"]
            pub const DATSZ_4: u32 = 0x04;
            #[doc = "4"]
            pub const DATSZ_5: u32 = 0x05;
            #[doc = "4"]
            pub const DATSZ_6: u32 = 0x06;
            #[doc = "4"]
            pub const DATSZ_7: u32 = 0x07;
        }
    }
    #[doc = "NAND Extended Address"]
    pub mod NAND_EXT_ADDR {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "IP Command control register 2"]
pub mod IPCR2 {
    #[doc = "Byte Mask for Byte 0 (IPTXD bit 7:0)"]
    pub mod BM0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Byte Unmasked"]
            pub const BM0_0: u32 = 0;
            #[doc = "Byte Masked"]
            pub const BM0_1: u32 = 0x01;
        }
    }
    #[doc = "Byte Mask for Byte 1 (IPTXD bit 15:8)"]
    pub mod BM1 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Byte Unmasked"]
            pub const BM1_0: u32 = 0;
            #[doc = "Byte Masked"]
            pub const BM1_1: u32 = 0x01;
        }
    }
    #[doc = "Byte Mask for Byte 2 (IPTXD bit 23:16)"]
    pub mod BM2 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Byte Unmasked"]
            pub const BM2_0: u32 = 0;
            #[doc = "Byte Masked"]
            pub const BM2_1: u32 = 0x01;
        }
    }
    #[doc = "Byte Mask for Byte 3 (IPTXD bit 31:24)"]
    pub mod BM3 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Byte Unmasked"]
            pub const BM3_0: u32 = 0;
            #[doc = "Byte Masked"]
            pub const BM3_1: u32 = 0x01;
        }
    }
}
#[doc = "IP Command register"]
pub mod IPCMD {
    #[doc = "SDRAM Commands: 0x8: READ 0x9: WRITE 0xA: MODESET 0xB: ACTIVE 0xC: AUTO REFRESH 0xD: SELF REFRESH 0xE: PRECHARGE 0xF: PRECHARGE ALL Others: RSVD SELF REFRESH will be sent to all SDRAM devices because they shared same SEMC_CLK pin"]
    pub mod CMD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This field should be written with 0xA55A when trigging an IP command."]
    pub mod KEY {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "TX DATA register (for IP Command)"]
pub mod IPTXDAT {
    #[doc = "no description available"]
    pub mod DAT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "RX DATA register (for IP Command)"]
pub mod IPRXDAT {
    #[doc = "no description available"]
    pub mod DAT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Status register 0"]
pub mod STS0 {
    #[doc = "Indicating whether SEMC is in IDLE state."]
    pub mod IDLE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Indicating NAND device Ready/WAIT# pin level."]
    pub mod NARDY {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "NAND device is not ready"]
            pub const NARDY_0: u32 = 0;
            #[doc = "NAND device is ready"]
            pub const NARDY_1: u32 = 0x01;
        }
    }
}
#[doc = "Status register 2"]
pub mod STS2 {
    #[doc = "This field indicating whether there is pending AXI command (write) to NAND device."]
    pub mod NDWRPEND {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No pending"]
            pub const NDWRPEND_0: u32 = 0;
            #[doc = "Pending"]
            pub const NDWRPEND_1: u32 = 0x01;
        }
    }
}
#[doc = "Status register 12"]
pub mod STS12 {
    #[doc = "This field indicating the last write address (AXI command) to NAND device (without base address in SEMC_BR4)."]
    pub mod NDADDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Status register 13"]
pub mod STS13 {
    #[doc = "Sample clock slave delay line locked."]
    pub mod SLVLOCK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Sample clock reference delay line locked."]
    pub mod REFLOCK {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Sample clock slave delay line delay cell number selection ."]
    pub mod SLVSEL {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Sample clock reference delay line delay cell number selection."]
    pub mod REFSEL {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
