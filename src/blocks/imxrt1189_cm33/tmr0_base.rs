#[doc = "Timer"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "Module ID Register"]
    pub TMR_ID: crate::RORegister<u32>,
    _reserved0: [u8; 0x04],
    #[doc = "Timer Capability Register"]
    pub TMR_CAPR: crate::RORegister<u32>,
    _reserved1: [u8; 0x14],
    #[doc = "Timer free running time low register"]
    pub TMR_FRT_L: crate::RORegister<u32>,
    #[doc = "Timer free running time high register"]
    pub TMR_FRT_H: crate::RORegister<u32>,
    #[doc = "Timer synchronous time low register"]
    pub TMR_SRT_L: crate::RORegister<u32>,
    #[doc = "Timer synchronous time high register."]
    pub TMR_SRT_H: crate::RORegister<u32>,
    #[doc = "Default ns timer counter low register"]
    pub TMR_DEF_CNT_L: crate::RORegister<u32>,
    #[doc = "Default ns timer counter high register"]
    pub TMR_DEF_CNT_H: crate::RORegister<u32>,
    _reserved2: [u8; 0x48],
    #[doc = "Timer Control Register"]
    pub TMR_CTRL: crate::RWRegister<u32>,
    #[doc = "Timer Event Register"]
    pub TMR_TEVENT: crate::RWRegister<u32>,
    #[doc = "Timer event mask register"]
    pub TMR_TEMASK: crate::RWRegister<u32>,
    _reserved3: [u8; 0x08],
    #[doc = "Timer status register"]
    pub TMR_STAT: crate::RORegister<u32>,
    #[doc = "Timer counter low register"]
    pub TMR_CNT_L: crate::RWRegister<u32>,
    #[doc = "Timer counter high register"]
    pub TMR_CNT_H: crate::RWRegister<u32>,
    #[doc = "Timer addend register"]
    pub TMR_ADD: crate::RWRegister<u32>,
    #[doc = "Timer accumulator register"]
    pub TMR_ACC: crate::RORegister<u32>,
    #[doc = "Timer prescale register"]
    pub TMR_PRSC: crate::RWRegister<u32>,
    #[doc = "Extended timer control register"]
    pub TMR_ECTRL: crate::RWRegister<u32>,
    #[doc = "Timer offset low register"]
    pub TMROFF_L: crate::RWRegister<u32>,
    #[doc = "Timer offset high register"]
    pub TMROFF_H: crate::RWRegister<u32>,
    #[doc = "Alarm 1 time comparator low register"]
    pub TMR_ALARM1_L: crate::RWRegister<u32>,
    #[doc = "Alarm 1 time comparator high register"]
    pub TMR_ALARM1_H: crate::RWRegister<u32>,
    #[doc = "Alarm 2 time comparator low register"]
    pub TMR_ALARM2_L: crate::RWRegister<u32>,
    #[doc = "Alarm 2 time comparator high register"]
    pub TMR_ALARM2_H: crate::RWRegister<u32>,
    _reserved4: [u8; 0x04],
    #[doc = "Timer Alarm Control Register"]
    pub TMR_ALARM_CTRL: crate::RWRegister<u32>,
    #[doc = "Timer i fixed interval period register"]
    pub TMR_FIPER: [crate::RWRegister<u32>; 3usize],
    #[doc = "Timer FIPER Control Register"]
    pub TMR_FIPER_CTRL: crate::RWRegister<u32>,
    #[doc = "External trigger stamp register"]
    pub TMR_ETTS1_L: crate::RORegister<u32>,
    #[doc = "External trigger stamp register"]
    pub TMR_ETTS1_H: crate::RORegister<u32>,
    #[doc = "External trigger stamp register"]
    pub TMR_ETTS2_L: crate::RORegister<u32>,
    #[doc = "External trigger stamp register"]
    pub TMR_ETTS2_H: crate::RORegister<u32>,
    #[doc = "Timer current time low register"]
    pub TMR_CUR_TIME_L: crate::RORegister<u32>,
    #[doc = "Timer current time high register"]
    pub TMR_CUR_TIME_H: crate::RORegister<u32>,
    #[doc = "Timer parameter register"]
    pub TMR_PARAM: crate::RWRegister<u32>,
}
#[doc = "Module ID Register"]
pub mod TMR_ID {
    #[doc = "Minor revision"]
    pub mod REV_MN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Major revision"]
    pub mod REV_MJ {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Timer IP ID"]
    pub mod TMR_ID {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Timer Capability Register"]
pub mod TMR_CAPR {
    #[doc = "IEEE 1722 support 0 Not supported 1 Supported"]
    pub mod IEEE_1722 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enhanced 1588 nanosecond (ns) Timer Adjustment supported"]
    pub mod ECADJ {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Support 802.1AS-2020 by providing both a ns and free-running clock to the Ethernet MACs."]
    pub mod IEEE_8021AS_REV {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Number of MSI-x Vectors supported"]
    pub mod NUM_MSIX {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Timer free running time low register"]
pub mod TMR_FRT_L {
    #[doc = "Read-only copy of the free running time (lower 32b)"]
    pub mod TMR_FRT_L {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Timer free running time high register"]
pub mod TMR_FRT_H {
    #[doc = "Read-only copy of the free running time (upper 32b)"]
    pub mod TMR_FRT_H {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Timer synchronous time low register"]
pub mod TMR_SRT_L {
    #[doc = "Read-only copy of the synchronous time (lower 32b)"]
    pub mod TMR_SRT_L {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Timer synchronous time high register."]
pub mod TMR_SRT_H {
    #[doc = "Read-only copy of the synchronous time (upper 32b)"]
    pub mod TMR_SRT_H {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Default ns timer counter low register"]
pub mod TMR_DEF_CNT_L {
    #[doc = "Read-only copy of the Default ns time counter (lower 32b)"]
    pub mod TMR_DEF_CNT_L {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Default ns timer counter high register"]
pub mod TMR_DEF_CNT_H {
    #[doc = "Read-only copy of the Default ns time counter (upper 32b)"]
    pub mod TMR_DEF_CNT_H {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Timer Control Register"]
pub mod TMR_CTRL {
    #[doc = "1588 timer reference clock source select"]
    pub mod CK_SEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1588 timer enable"]
    pub mod TE {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "External oscillator input clock phase"]
    pub mod CIPH {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Generated clock (TMR_GCLK) output phase."]
    pub mod COPH {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "External trigger 1 edge polarity"]
    pub mod ETEP1 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "External trigger 2 edge polarity"]
    pub mod ETEP2 {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Mode bit to allow atomic writes to TCLK_PERIOD and TMR_ADD"]
    pub mod COMP_MODE {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1588 timer reference clock period"]
    pub mod TCLK_PERIOD {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x03ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Fiper2 pulse loop back mode enabled"]
    pub mod PP2L {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Fiper1 pulse loop back mode enabled"]
    pub mod PP1L {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "FIPER start indication"]
    pub mod FS {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "shadow Register disable"]
    pub mod SHADOW_DIS {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Alarm2 output polarity"]
    pub mod ALM2P {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Alarm1 output polarity"]
    pub mod ALM1P {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Timer Event Register"]
pub mod TMR_TEVENT {
    #[doc = "Periodic pulse event 3 enable"]
    pub mod PP3EN {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Periodic pulse event 2 enable"]
    pub mod PP2EN {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Periodic pulse event 1 enable"]
    pub mod PP1EN {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Timer ALM2 event enable"]
    pub mod ALM1EN {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Timer ALM1 event enable"]
    pub mod ALM2EN {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "External trigger 1 timestamp FIFO Threshold Level Hit"]
    pub mod ETS1_THREN {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "External trigger 2 timestamp FIFO Threshold Level Hit"]
    pub mod ETS2_THREN {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "External trigger 1 new timestamp sample event available"]
    pub mod ETS1EN {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "External trigger 2 new timestamp sample event available"]
    pub mod ETS2EN {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "External trigger 1 timestamp FIFO Overflow event occurred"]
    pub mod ETS1_OVEN {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "External trigger 2 timestamp FIFO Overflow event occurred"]
    pub mod ETS2_OVEN {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Timer event mask register"]
pub mod TMR_TEMASK {
    #[doc = "Periodic pulse event 3 enable"]
    pub mod PP3EN {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Periodic pulse event 2 enable"]
    pub mod PP2EN {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Periodic pulse event 1 enable"]
    pub mod PP1EN {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Timer ALM2 event enable"]
    pub mod ALM1EN {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Timer ALM1 event enable"]
    pub mod ALM2EN {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "External trigger 1 timestamp FIFO Threshold Level Hit event enable"]
    pub mod ETS1_THREN {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "External trigger 2 timestamp FIFO Threshold Level Hit event enable"]
    pub mod ETS2_THREN {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "External trigger 1 new timestamp sample event available interrupt enable"]
    pub mod ETS1EN {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "External trigger 2 new timestamp sample event available interrupt enable"]
    pub mod ETS2EN {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "External trigger 1 timestamp FIFO Overflow event interrupt enabled"]
    pub mod ETS1_OVEN {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "External trigger 2 timestamp FIFO Overflow event interrupt enabled"]
    pub mod ETS2_OVEN {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Timer status register"]
pub mod TMR_STAT {
    #[doc = "External trigger 1 Valid time-stamp 0 all valid external trigger time-stamps have been read 1 external trigger has an unread valid time-stamp value"]
    pub mod ETS1_VLD {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "External trigger 2 Valid time-stamp 0 all valid external trigger time-stamps have been read 1 external trigger has an unread valid time-stamp value"]
    pub mod ETS2_VLD {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Timer Reference Clock Detected"]
    pub mod RCD {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Reference Clock has not been detected as active. Registers in timer clock domain are not allowed to be accessed; reads return 0, writes are ignored."]
            pub const REF_CLK_NOT_ACTIVE: u32 = 0;
            #[doc = "Reference Clock has been detected as active. Registers in timer clock domain are allowed to be accessed."]
            pub const REF_CLK_ACTIVE: u32 = 0x01;
        }
    }
}
#[doc = "Timer counter low register"]
pub mod TMR_CNT_L {
    #[doc = "Timer counter register."]
    pub mod TMR_CNT_L {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Timer counter high register"]
pub mod TMR_CNT_H {
    #[doc = "Timer counter register."]
    pub mod TMR_CNT_H {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Timer addend register"]
pub mod TMR_ADD {
    #[doc = "Timer addend."]
    pub mod ADDEND {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Timer accumulator register"]
pub mod TMR_ACC {
    #[doc = "32-bit timer accumulator register"]
    pub mod TMR_ACC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Timer prescale register"]
pub mod TMR_PRSC {
    #[doc = "Output clock division prescale factor."]
    pub mod PRSC_OCK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Extended timer control register"]
pub mod TMR_ECTRL {
    #[doc = "External trigger FIFO threshold."]
    pub mod ETFF_THR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Timer offset low register"]
pub mod TMROFF_L {
    #[doc = "Offset value of the clock counter."]
    pub mod TMROFF_L {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Timer offset high register"]
pub mod TMROFF_H {
    #[doc = "Offset value of the clock counter."]
    pub mod TMROFF_H {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Alarm 1 time comparator low register"]
pub mod TMR_ALARM1_L {
    #[doc = "Alarm time comparator register."]
    pub mod ALARM_L {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Alarm 1 time comparator high register"]
pub mod TMR_ALARM1_H {
    #[doc = "Alarm time comparator register."]
    pub mod ALARM_H {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Alarm 2 time comparator low register"]
pub mod TMR_ALARM2_L {
    #[doc = "Alarm time comparator register."]
    pub mod ALARM_L {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Alarm 2 time comparator high register"]
pub mod TMR_ALARM2_H {
    #[doc = "Alarm time comparator register."]
    pub mod ALARM_H {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Timer Alarm Control Register"]
pub mod TMR_ALARM_CTRL {
    #[doc = "ALARM 1 pulse width selector"]
    pub mod ALARM1_PW {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Alarm1 pulse generation time"]
    pub mod PG1 {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ALARM 2 pulse width selector"]
    pub mod ALARM2_PW {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Alarm2 pulse generation time"]
    pub mod PG2 {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Timer i fixed interval period register"]
pub mod TMR_FIPER {
    #[doc = "Fixed Interval Pulse Period"]
    pub mod FIPER {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Timer FIPER Control Register"]
pub mod TMR_FIPER_CTRL {
    #[doc = "FIPER 1 pulse width selection"]
    pub mod FIPER1_PW {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "FIPER1 pulse generation select"]
    pub mod PG1 {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "FIPER1 disable"]
    pub mod FIPER1_DIS {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "FIPER 2 pulse width selection"]
    pub mod FIPER2_PW {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "FIPER2 pulse generation time"]
    pub mod PG2 {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "FIPER2 disable"]
    pub mod FIPER2_DIS {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "FIPER 3 pulse width selection"]
    pub mod FIPER3_PW {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "FIPER3 pulse generation time"]
    pub mod PG3 {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "FIPER3 disable"]
    pub mod FIPER3_DIS {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "External trigger stamp register"]
pub mod TMR_ETTS1_L {
    #[doc = "Time stamp field at the programmable edge of the external trigger"]
    pub mod ETTS_L {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "External trigger stamp register"]
pub mod TMR_ETTS1_H {
    #[doc = "Time stamp field at the programmable edge of the external trigger"]
    pub mod ETTS_H {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "External trigger stamp register"]
pub mod TMR_ETTS2_L {
    #[doc = "Time stamp field at the programmable edge of the external trigger"]
    pub mod ETTS_L {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "External trigger stamp register"]
pub mod TMR_ETTS2_H {
    #[doc = "Time stamp field at the programmable edge of the external trigger"]
    pub mod ETTS_H {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Timer current time low register"]
pub mod TMR_CUR_TIME_L {
    #[doc = "Read-only copy of current time (lower 32b)"]
    pub mod TMR_CUR_TIME_L {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Timer current time high register"]
pub mod TMR_CUR_TIME_H {
    #[doc = "Read-only copy of current time (upper 32b)"]
    pub mod TMR_CUR_TIME_H {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Timer parameter register"]
pub mod TMR_PARAM {
    #[doc = "Timer synchronization"]
    pub mod SYNC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "User specific parameter values"]
    pub mod PARAM_VAL {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x7fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
