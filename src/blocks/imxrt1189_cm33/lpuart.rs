#[doc = "LPUART"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "Version ID"]
    pub VERID: crate::RORegister<u32>,
    #[doc = "Parameter"]
    pub PARAM: crate::RORegister<u32>,
    #[doc = "Global"]
    pub GLOBAL: crate::RWRegister<u32>,
    #[doc = "Pin Configuration"]
    pub PINCFG: crate::RWRegister<u32>,
    #[doc = "Baud Rate"]
    pub BAUD: crate::RWRegister<u32>,
    #[doc = "Status"]
    pub STAT: crate::RWRegister<u32>,
    #[doc = "Control"]
    pub CTRL: crate::RWRegister<u32>,
    #[doc = "Data"]
    pub DATA: crate::RWRegister<u32>,
    #[doc = "Match Address"]
    pub MATCH: crate::RWRegister<u32>,
    #[doc = "MODEM IrDA"]
    pub MODIR: crate::RWRegister<u32>,
    #[doc = "FIFO"]
    pub FIFO: crate::RWRegister<u32>,
    #[doc = "Watermark"]
    pub WATER: crate::RWRegister<u32>,
    #[doc = "Data Read-Only"]
    pub DATARO: crate::RORegister<u32>,
    _reserved0: [u8; 0x0c],
    #[doc = "MODEM Control"]
    pub MCR: crate::RWRegister<u32>,
    #[doc = "MODEM Status"]
    pub MSR: crate::RWRegister<u32>,
    #[doc = "Receiver Extended Idle"]
    pub REIR: crate::RWRegister<u32>,
    #[doc = "Transmitter Extended Idle"]
    pub TEIR: crate::RWRegister<u32>,
    #[doc = "Half Duplex Control"]
    pub HDCR: crate::RWRegister<u32>,
    _reserved1: [u8; 0x04],
    #[doc = "Timeout Control"]
    pub TOCR: crate::RWRegister<u32>,
    #[doc = "Timeout Status"]
    pub TOSR: crate::RWRegister<u32>,
    #[doc = "Timeout N"]
    pub TIMEOUT: [crate::RWRegister<u32>; 4usize],
    _reserved2: [u8; 0x0190],
    #[doc = "Transmit Command Burst"]
    pub TCBR: [crate::WORegister<u32>; 128usize],
    #[doc = "Transmit Data Burst"]
    pub TDBR: [crate::WORegister<u32>; 256usize],
}
#[doc = "Version ID"]
pub mod VERID {
    #[doc = "Feature Identification Number"]
    pub mod FEATURE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Standard feature set"]
            pub const STANDARD: u32 = 0x01;
            #[doc = "Standard feature set with MODEM and IrDA support"]
            pub const MODEM: u32 = 0x03;
            #[doc = "Enhanced feature set with full MODEM, IrDA, and enhanced idle detection"]
            pub const MODEM_IDLE: u32 = 0x07;
        }
    }
    #[doc = "Minor Version Number"]
    pub mod MINOR {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Major Version Number"]
    pub mod MAJOR {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Parameter"]
pub mod PARAM {
    #[doc = "Transmit FIFO Size"]
    pub mod TXFIFO {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Receive FIFO Size"]
    pub mod RXFIFO {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Global"]
pub mod GLOBAL {
    #[doc = "Software Reset"]
    pub mod RST {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Module is not reset."]
            pub const RST_0: u32 = 0;
            #[doc = "Module is reset."]
            pub const RST_1: u32 = 0x01;
        }
    }
}
#[doc = "Pin Configuration"]
pub mod PINCFG {
    #[doc = "Trigger Select"]
    pub mod TRGSEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Input trigger is disabled."]
            pub const TRGSEL_0: u32 = 0;
            #[doc = "Input trigger is used instead of RXD pin input."]
            pub const TRGSEL_1: u32 = 0x01;
            #[doc = "Input trigger is used instead of CTS_B pin input."]
            pub const TRGSEL_2: u32 = 0x02;
            #[doc = "Input trigger is used to modulate the TXD pin output. The TXD pin output (after TXINV configuration) is ANDed with the input trigger."]
            pub const TRGSEL_3: u32 = 0x03;
        }
    }
}
#[doc = "Baud Rate"]
pub mod BAUD {
    #[doc = "Baud Rate Modulo Divisor."]
    pub mod SBR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Stop Bit Number Select"]
    pub mod SBNS {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "One stop bit."]
            pub const SBNS_0: u32 = 0;
            #[doc = "Two stop bits."]
            pub const SBNS_1: u32 = 0x01;
        }
    }
    #[doc = "RX Input Active Edge Interrupt Enable"]
    pub mod RXEDGIE {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Hardware interrupts from STAT\\[RXEDGIF\\] are disabled."]
            pub const RXEDGIE_0: u32 = 0;
            #[doc = "Hardware interrupt is requested when STAT\\[RXEDGIF\\] flag is 1."]
            pub const RXEDGIE_1: u32 = 0x01;
        }
    }
    #[doc = "LIN Break Detect Interrupt Enable"]
    pub mod LBKDIE {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Hardware interrupts from STAT\\[LBKDIF\\] flag are disabled (use polling)."]
            pub const LBKDIE_0: u32 = 0;
            #[doc = "Hardware interrupt requested when STAT\\[LBKDIF\\] flag is 1."]
            pub const LBKDIE_1: u32 = 0x01;
        }
    }
    #[doc = "Resynchronization Disable"]
    pub mod RESYNCDIS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Resynchronization during received data word is supported"]
            pub const RESYNCDIS_0: u32 = 0;
            #[doc = "Resynchronization during received data word is disabled"]
            pub const RESYNCDIS_1: u32 = 0x01;
        }
    }
    #[doc = "Both Edge Sampling"]
    pub mod BOTHEDGE {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Receiver samples input data using the rising edge of the baud rate clock."]
            pub const BOTHEDGE_0: u32 = 0;
            #[doc = "Receiver samples input data using the rising and falling edge of the baud rate clock."]
            pub const BOTHEDGE_1: u32 = 0x01;
        }
    }
    #[doc = "Match Configuration"]
    pub mod MATCFG {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Address Match Wakeup"]
            pub const MATCFG_0: u32 = 0;
            #[doc = "Idle Match Wakeup"]
            pub const MATCFG_1: u32 = 0x01;
            #[doc = "Match On and Match Off"]
            pub const MATCFG_2: u32 = 0x02;
            #[doc = "Enables RWU on Data Match and Match On/Off for transmitter CTS input"]
            pub const MATCFG_3: u32 = 0x03;
        }
    }
    #[doc = "Receiver Idle DMA Enable"]
    pub mod RIDMAE {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "DMA request disabled."]
            pub const RIDMAE_0: u32 = 0;
            #[doc = "DMA request enabled."]
            pub const RIDMAE_1: u32 = 0x01;
        }
    }
    #[doc = "Receiver Full DMA Enable"]
    pub mod RDMAE {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "DMA request disabled."]
            pub const RDMAE_0: u32 = 0;
            #[doc = "DMA request enabled."]
            pub const RDMAE_1: u32 = 0x01;
        }
    }
    #[doc = "Transmitter DMA Enable"]
    pub mod TDMAE {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "DMA request disabled."]
            pub const TDMAE_0: u32 = 0;
            #[doc = "DMA request enabled."]
            pub const TDMAE_1: u32 = 0x01;
        }
    }
    #[doc = "Oversampling Ratio"]
    pub mod OSR {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Writing 0 to this field will result in an oversampling ratio of 16"]
            pub const OSR_0: u32 = 0;
            #[doc = "Oversampling ratio of 4, requires BOTHEDGE to be set."]
            pub const OSR_3: u32 = 0x03;
            #[doc = "Oversampling ratio of 5, requires BOTHEDGE to be set."]
            pub const OSR_4: u32 = 0x04;
            #[doc = "Oversampling ratio of 6, requires BOTHEDGE to be set."]
            pub const OSR_5: u32 = 0x05;
            #[doc = "Oversampling ratio of 7, requires BOTHEDGE to be set."]
            pub const OSR_6: u32 = 0x06;
            #[doc = "Oversampling ratio of 8."]
            pub const OSR_7: u32 = 0x07;
            #[doc = "Oversampling ratio of 9."]
            pub const OSR_8: u32 = 0x08;
            #[doc = "Oversampling ratio of 10."]
            pub const OSR_9: u32 = 0x09;
            #[doc = "Oversampling ratio of 11."]
            pub const OSR_10: u32 = 0x0a;
            #[doc = "Oversampling ratio of 12."]
            pub const OSR_11: u32 = 0x0b;
            #[doc = "Oversampling ratio of 13."]
            pub const OSR_12: u32 = 0x0c;
            #[doc = "Oversampling ratio of 14."]
            pub const OSR_13: u32 = 0x0d;
            #[doc = "Oversampling ratio of 15."]
            pub const OSR_14: u32 = 0x0e;
            #[doc = "Oversampling ratio of 16."]
            pub const OSR_15: u32 = 0x0f;
            #[doc = "Oversampling ratio of 17."]
            pub const OSR_16: u32 = 0x10;
            #[doc = "Oversampling ratio of 18."]
            pub const OSR_17: u32 = 0x11;
            #[doc = "Oversampling ratio of 19."]
            pub const OSR_18: u32 = 0x12;
            #[doc = "Oversampling ratio of 20."]
            pub const OSR_19: u32 = 0x13;
            #[doc = "Oversampling ratio of 21."]
            pub const OSR_20: u32 = 0x14;
            #[doc = "Oversampling ratio of 22."]
            pub const OSR_21: u32 = 0x15;
            #[doc = "Oversampling ratio of 23."]
            pub const OSR_22: u32 = 0x16;
            #[doc = "Oversampling ratio of 24."]
            pub const OSR_23: u32 = 0x17;
            #[doc = "Oversampling ratio of 25."]
            pub const OSR_24: u32 = 0x18;
            #[doc = "Oversampling ratio of 26."]
            pub const OSR_25: u32 = 0x19;
            #[doc = "Oversampling ratio of 27."]
            pub const OSR_26: u32 = 0x1a;
            #[doc = "Oversampling ratio of 28."]
            pub const OSR_27: u32 = 0x1b;
            #[doc = "Oversampling ratio of 29."]
            pub const OSR_28: u32 = 0x1c;
            #[doc = "Oversampling ratio of 30."]
            pub const OSR_29: u32 = 0x1d;
            #[doc = "Oversampling ratio of 31."]
            pub const OSR_30: u32 = 0x1e;
            #[doc = "Oversampling ratio of 32."]
            pub const OSR_31: u32 = 0x1f;
        }
    }
    #[doc = "10-bit Mode select"]
    pub mod M10 {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Receiver and transmitter use 7-bit to 9-bit data characters."]
            pub const M10_0: u32 = 0;
            #[doc = "Receiver and transmitter use 10-bit data characters."]
            pub const M10_1: u32 = 0x01;
        }
    }
    #[doc = "Match Address Mode Enable 2"]
    pub mod MAEN2 {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Normal operation."]
            pub const MAEN2_0: u32 = 0;
            #[doc = "Enables automatic address matching or data matching mode for MATCH\\[MA2\\]."]
            pub const MAEN2_1: u32 = 0x01;
        }
    }
    #[doc = "Match Address Mode Enable 1"]
    pub mod MAEN1 {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Normal operation."]
            pub const MAEN1_0: u32 = 0;
            #[doc = "Enables automatic address matching or data matching mode for MATCH\\[MA1\\]."]
            pub const MAEN1_1: u32 = 0x01;
        }
    }
}
#[doc = "Status"]
pub mod STAT {
    #[doc = "LIN Break Flag Enable"]
    pub mod LBKFE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const DISABLED: u32 = 0;
            #[doc = "Enable"]
            pub const ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Address Mark Enable"]
    pub mod AME {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const DISABLED: u32 = 0;
            #[doc = "Enable"]
            pub const ENABLED: u32 = 0x01;
        }
    }
    #[doc = "MODEM Status Flag"]
    pub mod MSF {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Field is 0"]
            pub const NOFLAG: u32 = 0;
            #[doc = "Field is 1"]
            pub const FLAG: u32 = 0x01;
        }
    }
    #[doc = "Timeout Status Flag"]
    pub mod TSF {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Field is 0"]
            pub const NOFLAG: u32 = 0;
            #[doc = "Field is 1"]
            pub const FLAG: u32 = 0x01;
        }
    }
    #[doc = "Match 2 Flag"]
    pub mod MA2F {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {
            #[doc = "Received data is not equal to MA2"]
            pub const MA2F_0: u32 = 0;
            #[doc = "Received data is equal to MA2"]
            pub const MA2F_1: u32 = 0x01;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Match 1 Flag"]
    pub mod MA1F {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {
            #[doc = "Received data is not equal to MA1"]
            pub const MA1F_0: u32 = 0;
            #[doc = "Received data is equal to MA1"]
            pub const MA1F_1: u32 = 0x01;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Parity Error Flag"]
    pub mod PF {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {
            #[doc = "No parity error."]
            pub const PF_0: u32 = 0;
            #[doc = "Parity error."]
            pub const PF_1: u32 = 0x01;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Framing Error Flag"]
    pub mod FE {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {
            #[doc = "No framing error detected. This does not guarantee the framing is correct."]
            pub const FE_0: u32 = 0;
            #[doc = "Framing error."]
            pub const FE_1: u32 = 0x01;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Noise Flag"]
    pub mod NF {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {
            #[doc = "No noise detected."]
            pub const NF_0: u32 = 0;
            #[doc = "Noise detected in the received character in the DATA register."]
            pub const NF_1: u32 = 0x01;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Receiver Overrun Flag"]
    pub mod OR {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {
            #[doc = "No overrun."]
            pub const OR_0: u32 = 0;
            #[doc = "Receive overrun (new LPUART data lost)."]
            pub const OR_1: u32 = 0x01;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Idle Line Flag"]
    pub mod IDLE {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {
            #[doc = "No idle line detected."]
            pub const IDLE_0: u32 = 0;
            #[doc = "Idle line was detected."]
            pub const IDLE_1: u32 = 0x01;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Receive Data Register Full Flag"]
    pub mod RDRF {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Receive data buffer empty."]
            pub const RDRF_0: u32 = 0;
            #[doc = "Receive data buffer full."]
            pub const RDRF_1: u32 = 0x01;
        }
    }
    #[doc = "Transmission Complete Flag"]
    pub mod TC {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Transmitter active (sending data, a preamble, or a break)."]
            pub const TC_0: u32 = 0;
            #[doc = "Transmitter idle (transmission activity complete)."]
            pub const TC_1: u32 = 0x01;
        }
    }
    #[doc = "Transmit Data Register Empty Flag"]
    pub mod TDRE {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Transmit data buffer full."]
            pub const TDRE_0: u32 = 0;
            #[doc = "Transmit data buffer empty."]
            pub const TDRE_1: u32 = 0x01;
        }
    }
    #[doc = "Receiver Active Flag"]
    pub mod RAF {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "LPUART receiver idle waiting for a start bit."]
            pub const RAF_0: u32 = 0;
            #[doc = "LPUART receiver active (RXD input not idle)."]
            pub const RAF_1: u32 = 0x01;
        }
    }
    #[doc = "LIN Break Detection Enable"]
    pub mod LBKDE {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "LIN break detect is disabled, normal break character can be detected."]
            pub const LBKDE_0: u32 = 0;
            #[doc = "LIN break detect is enabled. LIN break character is detected at length of 11 bit times (if M = 0) or 12 (if M = 1) or 13 (M10 = 1)."]
            pub const LBKDE_1: u32 = 0x01;
        }
    }
    #[doc = "Break Character Generation Length"]
    pub mod BRK13 {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Break character is transmitted with length of 9 to 13 bit times."]
            pub const BRK13_0: u32 = 0;
            #[doc = "Break character is transmitted with length of 12 to 15 bit times."]
            pub const BRK13_1: u32 = 0x01;
        }
    }
    #[doc = "Receive Wake Up Idle Detect"]
    pub mod RWUID {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "During receive standby state (RWU = 1), the IDLE bit does not get set upon detection of an idle character. During address match wakeup, the IDLE bit does not set when an address does not match."]
            pub const RWUID_0: u32 = 0;
            #[doc = "During receive standby state (RWU = 1), the IDLE bit gets set upon detection of an idle character. During address match wakeup, the IDLE bit does set when an address does not match."]
            pub const RWUID_1: u32 = 0x01;
        }
    }
    #[doc = "Receive Data Inversion"]
    pub mod RXINV {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Receive data not inverted."]
            pub const RXINV_0: u32 = 0;
            #[doc = "Receive data inverted."]
            pub const RXINV_1: u32 = 0x01;
        }
    }
    #[doc = "MSB First"]
    pub mod MSBF {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "LSB (bit0) is the first bit that is transmitted following the start bit. Further, the first bit received after the start bit is identified as bit0."]
            pub const MSBF_0: u32 = 0;
            #[doc = "MSB (bit9, bit8, bit7 or bit6) is the first bit that is transmitted following the start bit depending on the setting of CTRL\\[M\\], CTRL\\[PE\\] and BAUD\\[M10\\]. Further, the first bit received after the start bit is identified as bit9, bit8, bit7 or bit6 depending on the setting of CTRL\\[M\\] and CTRL\\[PE\\]."]
            pub const MSBF_1: u32 = 0x01;
        }
    }
    #[doc = "RXD Pin Active Edge Interrupt Flag"]
    pub mod RXEDGIF {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {
            #[doc = "No active edge on the receive pin has occurred."]
            pub const RXEDGIF_0: u32 = 0;
            #[doc = "An active edge on the receive pin has occurred."]
            pub const RXEDGIF_1: u32 = 0x01;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "LIN Break Detect Interrupt Flag"]
    pub mod LBKDIF {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {
            #[doc = "No LIN break character has been detected."]
            pub const LBKDIF_0: u32 = 0;
            #[doc = "LIN break character has been detected."]
            pub const LBKDIF_1: u32 = 0x01;
        }
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Control"]
pub mod CTRL {
    #[doc = "Parity Type"]
    pub mod PT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Even parity."]
            pub const PT_0: u32 = 0;
            #[doc = "Odd parity."]
            pub const PT_1: u32 = 0x01;
        }
    }
    #[doc = "Parity Enable"]
    pub mod PE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No hardware parity generation or checking."]
            pub const PE_0: u32 = 0;
            #[doc = "Parity enabled."]
            pub const PE_1: u32 = 0x01;
        }
    }
    #[doc = "Idle Line Type Select"]
    pub mod ILT {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Idle character bit count starts after start bit."]
            pub const ILT_0: u32 = 0;
            #[doc = "Idle character bit count starts after stop bit."]
            pub const ILT_1: u32 = 0x01;
        }
    }
    #[doc = "Receiver Wakeup Method Select"]
    pub mod WAKE {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Configures RWU for idle-line wakeup."]
            pub const WAKE_0: u32 = 0;
            #[doc = "Configures RWU with address-mark wakeup."]
            pub const WAKE_1: u32 = 0x01;
        }
    }
    #[doc = "9-Bit or 8-Bit Mode Select"]
    pub mod M {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Receiver and transmitter use 8-bit data characters."]
            pub const M_0: u32 = 0;
            #[doc = "Receiver and transmitter use 9-bit data characters."]
            pub const M_1: u32 = 0x01;
        }
    }
    #[doc = "Receiver Source Select"]
    pub mod RSRC {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Provided LOOPS is set, RSRC is cleared, selects internal loop back mode and the LPUART does not use the RXD pin."]
            pub const RSRC_0: u32 = 0;
            #[doc = "Single-wire LPUART mode where the TXD pin is connected to the transmitter output and receiver input."]
            pub const RSRC_1: u32 = 0x01;
        }
    }
    #[doc = "Doze Enable"]
    pub mod DOZEEN {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "LPUART is enabled in Doze mode."]
            pub const DOZEEN_0: u32 = 0;
            #[doc = "LPUART is disabled in Doze mode."]
            pub const DOZEEN_1: u32 = 0x01;
        }
    }
    #[doc = "Loop Mode Select"]
    pub mod LOOPS {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Normal operation - RXD and TXD use separate pins."]
            pub const LOOPS_0: u32 = 0;
            #[doc = "Loop mode or single-wire mode where transmitter outputs are internally connected to receiver input (see RSRC bit)."]
            pub const LOOPS_1: u32 = 0x01;
        }
    }
    #[doc = "Idle Configuration"]
    pub mod IDLECFG {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "1 idle character"]
            pub const IDLECFG_0: u32 = 0;
            #[doc = "2 idle characters"]
            pub const IDLECFG_1: u32 = 0x01;
            #[doc = "4 idle characters"]
            pub const IDLECFG_2: u32 = 0x02;
            #[doc = "8 idle characters"]
            pub const IDLECFG_3: u32 = 0x03;
            #[doc = "16 idle characters"]
            pub const IDLECFG_4: u32 = 0x04;
            #[doc = "32 idle characters"]
            pub const IDLECFG_5: u32 = 0x05;
            #[doc = "64 idle characters"]
            pub const IDLECFG_6: u32 = 0x06;
            #[doc = "128 idle characters"]
            pub const IDLECFG_7: u32 = 0x07;
        }
    }
    #[doc = "7-Bit Mode Select"]
    pub mod M7 {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Receiver and transmitter use 8-bit to 10-bit data characters."]
            pub const M7_0: u32 = 0;
            #[doc = "Receiver and transmitter use 7-bit data characters."]
            pub const M7_1: u32 = 0x01;
        }
    }
    #[doc = "Match 2 Interrupt Enable"]
    pub mod MA2IE {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MA2F interrupt disabled"]
            pub const MA2IE_0: u32 = 0;
            #[doc = "MA2F interrupt enabled"]
            pub const MA2IE_1: u32 = 0x01;
        }
    }
    #[doc = "Match 1 Interrupt Enable"]
    pub mod MA1IE {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MA1F interrupt disabled"]
            pub const MA1IE_0: u32 = 0;
            #[doc = "MA1F interrupt enabled"]
            pub const MA1IE_1: u32 = 0x01;
        }
    }
    #[doc = "Send Break"]
    pub mod SBK {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Normal transmitter operation."]
            pub const SBK_0: u32 = 0;
            #[doc = "Queue break character(s) to be sent."]
            pub const SBK_1: u32 = 0x01;
        }
    }
    #[doc = "Receiver Wakeup Control"]
    pub mod RWU {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Normal receiver operation."]
            pub const RWU_0: u32 = 0;
            #[doc = "LPUART receiver in standby waiting for wakeup condition."]
            pub const RWU_1: u32 = 0x01;
        }
    }
    #[doc = "Receiver Enable"]
    pub mod RE {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Receiver disabled."]
            pub const RE_0: u32 = 0;
            #[doc = "Receiver enabled."]
            pub const RE_1: u32 = 0x01;
        }
    }
    #[doc = "Transmitter Enable"]
    pub mod TE {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Transmitter disabled."]
            pub const TE_0: u32 = 0;
            #[doc = "Transmitter enabled."]
            pub const TE_1: u32 = 0x01;
        }
    }
    #[doc = "Idle Line Interrupt Enable"]
    pub mod ILIE {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Hardware interrupts from IDLE disabled; use polling."]
            pub const ILIE_0: u32 = 0;
            #[doc = "Hardware interrupt requested when IDLE flag is 1."]
            pub const ILIE_1: u32 = 0x01;
        }
    }
    #[doc = "Receiver Interrupt Enable"]
    pub mod RIE {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Hardware interrupts from RDRF disabled; use polling."]
            pub const RIE_0: u32 = 0;
            #[doc = "Hardware interrupt requested when RDRF flag is 1."]
            pub const RIE_1: u32 = 0x01;
        }
    }
    #[doc = "Transmission Complete Interrupt Enable for"]
    pub mod TCIE {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Hardware interrupts from TC disabled; use polling."]
            pub const TCIE_0: u32 = 0;
            #[doc = "Hardware interrupt requested when TC flag is 1."]
            pub const TCIE_1: u32 = 0x01;
        }
    }
    #[doc = "Transmit Interrupt Enable"]
    pub mod TIE {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Hardware interrupts from TDRE disabled; use polling."]
            pub const TIE_0: u32 = 0;
            #[doc = "Hardware interrupt requested when TDRE flag is 1."]
            pub const TIE_1: u32 = 0x01;
        }
    }
    #[doc = "Parity Error Interrupt Enable"]
    pub mod PEIE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "PF interrupts disabled; use polling)."]
            pub const PEIE_0: u32 = 0;
            #[doc = "Hardware interrupt requested when PF is set."]
            pub const PEIE_1: u32 = 0x01;
        }
    }
    #[doc = "Framing Error Interrupt Enable"]
    pub mod FEIE {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "FE interrupts disabled; use polling."]
            pub const FEIE_0: u32 = 0;
            #[doc = "Hardware interrupt requested when FE is set."]
            pub const FEIE_1: u32 = 0x01;
        }
    }
    #[doc = "Noise Error Interrupt Enable"]
    pub mod NEIE {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "NF interrupts disabled; use polling."]
            pub const NEIE_0: u32 = 0;
            #[doc = "Hardware interrupt requested when NF is set."]
            pub const NEIE_1: u32 = 0x01;
        }
    }
    #[doc = "Overrun Interrupt Enable"]
    pub mod ORIE {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "OR interrupts disabled; use polling."]
            pub const ORIE_0: u32 = 0;
            #[doc = "Hardware interrupt requested when OR is set."]
            pub const ORIE_1: u32 = 0x01;
        }
    }
    #[doc = "Transmit Data Inversion"]
    pub mod TXINV {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Transmit data not inverted."]
            pub const TXINV_0: u32 = 0;
            #[doc = "Transmit data inverted."]
            pub const TXINV_1: u32 = 0x01;
        }
    }
    #[doc = "TXD Pin Direction in Single-Wire Mode"]
    pub mod TXDIR {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "TXD pin is an input in single-wire mode."]
            pub const TXDIR_0: u32 = 0;
            #[doc = "TXD pin is an output in single-wire mode."]
            pub const TXDIR_1: u32 = 0x01;
        }
    }
    #[doc = "Receive Bit 9 / Transmit Bit 8"]
    pub mod R9T8 {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Receive Bit 8 / Transmit Bit 9"]
    pub mod R8T9 {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Data"]
pub mod DATA {
    #[doc = "Read receive FIFO bit 0 or write transmit FIFO bit 0"]
    pub mod R0T0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Read receive FIFO bit 1 or write transmit FIFO bit 1"]
    pub mod R1T1 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Read receive FIFO bit 2 or write transmit FIFO bit 2"]
    pub mod R2T2 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Read receive FIFO bit 3 or write transmit FIFO bit 3"]
    pub mod R3T3 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Read receive FIFO bit 4 or write transmit FIFO bit 4"]
    pub mod R4T4 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Read receive FIFO bit 5 or write transmit FIFO bit 5"]
    pub mod R5T5 {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Read receive FIFO bit 6 or write transmit FIFO bit 6"]
    pub mod R6T6 {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Read receive FIFO bit 7 or write transmit FIFO bit 7"]
    pub mod R7T7 {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Read receive FIFO bit 8 or write transmit FIFO bit 8"]
    pub mod R8T8 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Read receive FIFO bit 9 or write transmit FIFO bit 9"]
    pub mod R9T9 {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "LIN Break"]
    pub mod LINBRK {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not detected"]
            pub const NO_BREAK: u32 = 0;
            #[doc = "Detected"]
            pub const BREAK: u32 = 0x01;
        }
    }
    #[doc = "Idle Line"]
    pub mod IDLINE {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Receiver was not idle before receiving this character."]
            pub const IDLINE_0: u32 = 0;
            #[doc = "Receiver was idle before receiving this character."]
            pub const IDLINE_1: u32 = 0x01;
        }
    }
    #[doc = "Receive Buffer Empty"]
    pub mod RXEMPT {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Receive buffer contains valid data."]
            pub const RXEMPT_0: u32 = 0;
            #[doc = "Receive buffer is empty, data returned on read is not valid."]
            pub const RXEMPT_1: u32 = 0x01;
        }
    }
    #[doc = "Frame Error Transmit Special Character"]
    pub mod FRETSC {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The dataword was received without a frame error on read, or transmit a normal character on write."]
            pub const FRETSC_0: u32 = 0;
            #[doc = "The dataword was received with a frame error, or transmit an idle or break character on transmit."]
            pub const FRETSC_1: u32 = 0x01;
        }
    }
    #[doc = "Parity Error"]
    pub mod PARITYE {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The dataword was received without a parity error."]
            pub const PARITYE_0: u32 = 0;
            #[doc = "The dataword was received with a parity error."]
            pub const PARITYE_1: u32 = 0x01;
        }
    }
    #[doc = "Noisy Data Received"]
    pub mod NOISY {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The dataword was received without noise."]
            pub const NOISY_0: u32 = 0;
            #[doc = "The data was received with noise."]
            pub const NOISY_1: u32 = 0x01;
        }
    }
}
#[doc = "Match Address"]
pub mod MATCH {
    #[doc = "Match Address 1"]
    pub mod MA1 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Match Address 2"]
    pub mod MA2 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x03ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "MODEM IrDA"]
pub mod MODIR {
    #[doc = "Transmitter CTS Enable"]
    pub mod TXCTSE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "CTS has no effect on the transmitter."]
            pub const TXCTSE_0: u32 = 0;
            #[doc = "Enables clear-to-send operation. The transmitter checks the state of CTS each time it is ready to send a character. If CTS is asserted, the character is sent. If CTS is deasserted, the signal TXD remains in the mark state and transmission is delayed until CTS is asserted. Changes in CTS as a character is being sent do not affect its transmission."]
            pub const TXCTSE_1: u32 = 0x01;
        }
    }
    #[doc = "Transmitter RTS Enable"]
    pub mod TXRTSE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The transmitter has no effect on RTS."]
            pub const TXRTSE_0: u32 = 0;
            #[doc = "When a character is placed into an empty transmitter data buffer , RTS asserts one bit time before the start bit is transmitted. RTS deasserts one bit time after all characters in the transmitter data buffer and shift register are completely sent, including the last stop bit."]
            pub const TXRTSE_1: u32 = 0x01;
        }
    }
    #[doc = "Transmitter RTS Polarity"]
    pub mod TXRTSPOL {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Transmitter RTS is active low."]
            pub const TXRTSPOL_0: u32 = 0;
            #[doc = "Transmitter RTS is active high."]
            pub const TXRTSPOL_1: u32 = 0x01;
        }
    }
    #[doc = "Receiver RTS Enable"]
    pub mod RXRTSE {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The receiver has no effect on RTS."]
            pub const RXRTSE_0: u32 = 0;
            #[doc = "RTS is deasserted if the receiver data register is full or a start bit has been detected that would cause the receiver data register to become full. RTS is asserted if the receiver data register is not full and has not detected a start bit that would cause the receiver data register to become full."]
            pub const RXRTSE_1: u32 = 0x01;
        }
    }
    #[doc = "Transmit CTS Configuration"]
    pub mod TXCTSC {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "CTS input is sampled at the start of each character."]
            pub const TXCTSC_0: u32 = 0;
            #[doc = "CTS input is sampled when the transmitter is idle."]
            pub const TXCTSC_1: u32 = 0x01;
        }
    }
    #[doc = "Transmit CTS Source"]
    pub mod TXCTSSRC {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "CTS input is the CTS_B pin."]
            pub const TXCTSSRC_0: u32 = 0;
            #[doc = "CTS input is the inverted Receiver Match result."]
            pub const TXCTSSRC_1: u32 = 0x01;
        }
    }
    #[doc = "Receive RTS Configuration"]
    pub mod RTSWATER {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Transmitter Narrow Pulse"]
    pub mod TNP {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "1/OSR."]
            pub const TNP_0: u32 = 0;
            #[doc = "2/OSR."]
            pub const TNP_1: u32 = 0x01;
            #[doc = "3/OSR."]
            pub const TNP_2: u32 = 0x02;
            #[doc = "4/OSR."]
            pub const TNP_3: u32 = 0x03;
        }
    }
    #[doc = "IR Enable"]
    pub mod IREN {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "IR disabled."]
            pub const IREN_0: u32 = 0;
            #[doc = "IR enabled."]
            pub const IREN_1: u32 = 0x01;
        }
    }
}
#[doc = "FIFO"]
pub mod FIFO {
    #[doc = "Receive FIFO Buffer Depth"]
    pub mod RXFIFOSIZE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Receive FIFO/Buffer depth = 1 dataword."]
            pub const RXFIFOSIZE_0: u32 = 0;
            #[doc = "Receive FIFO/Buffer depth = 4 datawords."]
            pub const RXFIFOSIZE_1: u32 = 0x01;
            #[doc = "Receive FIFO/Buffer depth = 8 datawords."]
            pub const RXFIFOSIZE_2: u32 = 0x02;
            #[doc = "Receive FIFO/Buffer depth = 16 datawords."]
            pub const RXFIFOSIZE_3: u32 = 0x03;
            #[doc = "Receive FIFO/Buffer depth = 32 datawords."]
            pub const RXFIFOSIZE_4: u32 = 0x04;
            #[doc = "Receive FIFO/Buffer depth = 64 datawords."]
            pub const RXFIFOSIZE_5: u32 = 0x05;
            #[doc = "Receive FIFO/Buffer depth = 128 datawords."]
            pub const RXFIFOSIZE_6: u32 = 0x06;
            #[doc = "Receive FIFO/Buffer depth = 256 datawords."]
            pub const RXFIFOSIZE_7: u32 = 0x07;
        }
    }
    #[doc = "Receive FIFO Enable"]
    pub mod RXFE {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Receive FIFO is not enabled. Buffer is depth 1."]
            pub const RXFE_0: u32 = 0;
            #[doc = "Receive FIFO is enabled. Buffer is depth indicted by RXFIFOSIZE."]
            pub const RXFE_1: u32 = 0x01;
        }
    }
    #[doc = "Transmit FIFO Buffer Depth"]
    pub mod TXFIFOSIZE {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Transmit FIFO/Buffer depth = 1 dataword."]
            pub const TXFIFOSIZE_0: u32 = 0;
            #[doc = "Transmit FIFO/Buffer depth = 4 datawords."]
            pub const TXFIFOSIZE_1: u32 = 0x01;
            #[doc = "Transmit FIFO/Buffer depth = 8 datawords."]
            pub const TXFIFOSIZE_2: u32 = 0x02;
            #[doc = "Transmit FIFO/Buffer depth = 16 datawords."]
            pub const TXFIFOSIZE_3: u32 = 0x03;
            #[doc = "Transmit FIFO/Buffer depth = 32 datawords."]
            pub const TXFIFOSIZE_4: u32 = 0x04;
            #[doc = "Transmit FIFO/Buffer depth = 64 datawords."]
            pub const TXFIFOSIZE_5: u32 = 0x05;
            #[doc = "Transmit FIFO/Buffer depth = 128 datawords."]
            pub const TXFIFOSIZE_6: u32 = 0x06;
            #[doc = "Transmit FIFO/Buffer depth = 256 datawords"]
            pub const TXFIFOSIZE_7: u32 = 0x07;
        }
    }
    #[doc = "Transmit FIFO Enable"]
    pub mod TXFE {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Transmit FIFO is not enabled. Buffer is depth 1."]
            pub const TXFE_0: u32 = 0;
            #[doc = "Transmit FIFO is enabled. Buffer is depth indicated by TXFIFOSIZE."]
            pub const TXFE_1: u32 = 0x01;
        }
    }
    #[doc = "Receive FIFO Underflow Interrupt Enable"]
    pub mod RXUFE {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "RXUF flag does not generate an interrupt to the host."]
            pub const RXUFE_0: u32 = 0;
            #[doc = "RXUF flag generates an interrupt to the host."]
            pub const RXUFE_1: u32 = 0x01;
        }
    }
    #[doc = "Transmit FIFO Overflow Interrupt Enable"]
    pub mod TXOFE {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "TXOF flag does not generate an interrupt to the host."]
            pub const TXOFE_0: u32 = 0;
            #[doc = "TXOF flag generates an interrupt to the host."]
            pub const TXOFE_1: u32 = 0x01;
        }
    }
    #[doc = "Receiver Idle Empty Enable"]
    pub mod RXIDEN {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable RDRF assertion due to partially filled FIFO when receiver is idle."]
            pub const RXIDEN_0: u32 = 0;
            #[doc = "Enable RDRF assertion due to partially filled FIFO when receiver is idle for 1 character."]
            pub const RXIDEN_1: u32 = 0x01;
            #[doc = "Enable RDRF assertion due to partially filled FIFO when receiver is idle for 2 characters."]
            pub const RXIDEN_2: u32 = 0x02;
            #[doc = "Enable RDRF assertion due to partially filled FIFO when receiver is idle for 4 characters."]
            pub const RXIDEN_3: u32 = 0x03;
            #[doc = "Enable RDRF assertion due to partially filled FIFO when receiver is idle for 8 characters."]
            pub const RXIDEN_4: u32 = 0x04;
            #[doc = "Enable RDRF assertion due to partially filled FIFO when receiver is idle for 16 characters."]
            pub const RXIDEN_5: u32 = 0x05;
            #[doc = "Enable RDRF assertion due to partially filled FIFO when receiver is idle for 32 characters."]
            pub const RXIDEN_6: u32 = 0x06;
            #[doc = "Enable RDRF assertion due to partially filled FIFO when receiver is idle for 64 characters."]
            pub const RXIDEN_7: u32 = 0x07;
        }
    }
    #[doc = "Receive FIFO Flush"]
    pub mod RXFLUSH {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No flush operation occurs."]
            pub const RXFLUSH_0: u32 = 0;
            #[doc = "All data in the receive FIFO/buffer is cleared out."]
            pub const RXFLUSH_1: u32 = 0x01;
        }
    }
    #[doc = "Transmit FIFO Flush"]
    pub mod TXFLUSH {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No flush operation occurs."]
            pub const TXFLUSH_0: u32 = 0;
            #[doc = "All data in the transmit FIFO/Buffer is cleared out."]
            pub const TXFLUSH_1: u32 = 0x01;
        }
    }
    #[doc = "Receiver FIFO Underflow Flag"]
    pub mod RXUF {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {
            #[doc = "No receive buffer underflow has occurred since the last time the flag was cleared."]
            pub const RXUF_0: u32 = 0;
            #[doc = "At least one receive buffer underflow has occurred since the last time the flag was cleared."]
            pub const RXUF_1: u32 = 0x01;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Transmitter FIFO Overflow Flag"]
    pub mod TXOF {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {
            #[doc = "No transmit buffer overflow has occurred since the last time the flag was cleared."]
            pub const TXOF_0: u32 = 0;
            #[doc = "At least one transmit buffer overflow has occurred since the last time the flag was cleared."]
            pub const TXOF_1: u32 = 0x01;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Receive FIFO Or Buffer Empty"]
    pub mod RXEMPT {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Receive buffer is not empty."]
            pub const RXEMPT_0: u32 = 0;
            #[doc = "Receive buffer is empty."]
            pub const RXEMPT_1: u32 = 0x01;
        }
    }
    #[doc = "Transmit FIFO Or Buffer Empty"]
    pub mod TXEMPT {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Transmit buffer is not empty."]
            pub const TXEMPT_0: u32 = 0;
            #[doc = "Transmit buffer is empty."]
            pub const TXEMPT_1: u32 = 0x01;
        }
    }
}
#[doc = "Watermark"]
pub mod WATER {
    #[doc = "Transmit Watermark"]
    pub mod TXWATER {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Transmit Counter"]
    pub mod TXCOUNT {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Receive Watermark"]
    pub mod RXWATER {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Receive Counter"]
    pub mod RXCOUNT {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Data Read-Only"]
pub mod DATARO {
    #[doc = "Receive Data"]
    pub mod DATA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "MODEM Control"]
pub mod MCR {
    #[doc = "Clear To Send"]
    pub mod CTS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable interrupt"]
            pub const DISABLED: u32 = 0;
            #[doc = "Enable interrupt"]
            pub const ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Data Set Ready"]
    pub mod DSR {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable interrupt"]
            pub const DISABLED: u32 = 0;
            #[doc = "Enable interrupt"]
            pub const ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Ring Indicator"]
    pub mod RIN {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable interrupt"]
            pub const DISABLED: u32 = 0;
            #[doc = "Enable interrupt"]
            pub const ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Data Carrier Detect"]
    pub mod DCD {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable interrupt"]
            pub const DISABLED: u32 = 0;
            #[doc = "Enable interrupt"]
            pub const ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Data Terminal Ready"]
    pub mod DTR {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Logic one"]
            pub const LOGIC_ONE: u32 = 0;
            #[doc = "Logic zero"]
            pub const LOGIC_ZERO: u32 = 0x01;
        }
    }
    #[doc = "Request To Send"]
    pub mod RTS {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Logic one"]
            pub const LOGIC_ONE: u32 = 0;
            #[doc = "Logic zero"]
            pub const LOGIC_ZERO: u32 = 0x01;
        }
    }
}
#[doc = "MODEM Status"]
pub mod MSR {
    #[doc = "Delta Clear To Send"]
    pub mod DCTS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {
            #[doc = "Did not change state"]
            pub const NO_CHANGE: u32 = 0;
            #[doc = "Changed state"]
            pub const CHANGE: u32 = 0x01;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Delta Data Set Ready"]
    pub mod DDSR {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {
            #[doc = "Did not change state"]
            pub const NO_CHANGE: u32 = 0;
            #[doc = "Changed state"]
            pub const CHANGE: u32 = 0x01;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Delta Ring Indicator"]
    pub mod DRI {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {
            #[doc = "Did not change state"]
            pub const NO_CHANGE: u32 = 0;
            #[doc = "Changed state"]
            pub const CHANGE: u32 = 0x01;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Delta Data Carrier Detect"]
    pub mod DDCD {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {
            #[doc = "Did not change state"]
            pub const NO_CHANGE: u32 = 0;
            #[doc = "Changed state"]
            pub const CHANGE: u32 = 0x01;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Clear To Send"]
    pub mod CTS {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Logic one"]
            pub const LOGIC_ONE: u32 = 0;
            #[doc = "Logic zero"]
            pub const LOGIC_ZERO: u32 = 0x01;
        }
    }
    #[doc = "Data Set Ready"]
    pub mod DSR {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Logic one"]
            pub const LOGIC_ONE: u32 = 0;
            #[doc = "Logic zero"]
            pub const LOGIC_ZERO: u32 = 0x01;
        }
    }
    #[doc = "Ring Indicator"]
    pub mod RIN {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Logic one"]
            pub const LOGIC_ONE: u32 = 0;
            #[doc = "Logic zero"]
            pub const LOGIC_ZERO: u32 = 0x01;
        }
    }
    #[doc = "Data Carrier Detect"]
    pub mod DCD {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Logic one"]
            pub const LOGIC_ONE: u32 = 0;
            #[doc = "Logic zero"]
            pub const LOGIC_ZERO: u32 = 0x01;
        }
    }
}
#[doc = "Receiver Extended Idle"]
pub mod REIR {
    #[doc = "Idle Time"]
    pub mod IDTIME {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Transmitter Extended Idle"]
pub mod TEIR {
    #[doc = "Idle Time"]
    pub mod IDTIME {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Half Duplex Control"]
pub mod HDCR {
    #[doc = "Transmit Stall"]
    pub mod TXSTALL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No effect"]
            pub const NO_EFFECT: u32 = 0;
            #[doc = "Does not become busy"]
            pub const RX_ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "Receive Select"]
    pub mod RXSEL {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "RXD"]
            pub const PIN_RXD: u32 = 0;
            #[doc = "TXD"]
            pub const PIN_TXD: u32 = 0x01;
        }
    }
    #[doc = "Receive FIFO Write Mask"]
    pub mod RXWRMSK {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Do not mask"]
            pub const NO_EFFECT: u32 = 0;
            #[doc = "Mask"]
            pub const TX_RTS: u32 = 0x01;
        }
    }
    #[doc = "Receive Mask"]
    pub mod RXMSK {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Do not mask"]
            pub const NO_EFFECT: u32 = 0;
            #[doc = "Mask"]
            pub const TX_RTS: u32 = 0x01;
        }
    }
    #[doc = "RTS Extended"]
    pub mod RTSEXT {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Timeout Control"]
pub mod TOCR {
    #[doc = "Timeout Enable"]
    pub mod TOEN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Timeout Interrupt Enable"]
    pub mod TOIE {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Timeout Status"]
pub mod TOSR {
    #[doc = "Timeout Zero"]
    pub mod TOZ {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Timeout Flag"]
    pub mod TOF {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {
            #[doc = "Not occurred"]
            pub const NOT_OCCURRED: u32 = 0;
            #[doc = "Occurred"]
            pub const OCCURRED: u32 = 0x01;
        }
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Timeout N"]
pub mod TIMEOUT {
    #[doc = "Timeout Value"]
    pub mod TIMEOUT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Idle Configuration"]
    pub mod CFG {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Becomes 1 after timeout characters are received"]
            pub const CNT_CHAR: u32 = 0;
            #[doc = "Becomes 1 when idle for timeout bit clocks"]
            pub const CNT_IDLE: u32 = 0x01;
            #[doc = "Becomes 1 when idle for timeout bit clocks following the next character"]
            pub const CNT_BUSY_IDLE: u32 = 0x02;
            #[doc = "Becomes 1 when idle for at least timeout bit clocks, but a new character is detected before the extended idle timeout is reached"]
            pub const CNT_CHAR_IDLE: u32 = 0x03;
        }
    }
}
#[doc = "Transmit Command Burst"]
pub mod TCBR {
    #[doc = "Data"]
    pub mod DATA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Transmit Data Burst"]
pub mod TDBR {
    #[doc = "Data0"]
    pub mod DATA0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data1"]
    pub mod DATA1 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data2"]
    pub mod DATA2 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data3"]
    pub mod DATA3 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
