#[doc = "BBNSM"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "BBNSM Version ID Register"]
    pub BBNSM_VID: crate::RORegister<u32>,
    #[doc = "BBNSM Features Register"]
    pub BBNSM_FEATURES: crate::RORegister<u32>,
    #[doc = "BBNSM Control Register"]
    pub BBNSM_CTRL: crate::RWRegister<u32>,
    _reserved0: [u8; 0x04],
    #[doc = "BBNSM Interrupt Enable Register"]
    pub BBNSM_INT_EN: crate::RWRegister<u32>,
    #[doc = "BBNSM Events Register"]
    pub BBNSM_EVENTS: crate::RWRegister<u32>,
    _reserved1: [u8; 0x0c],
    #[doc = "BBNSM External Pad Control Register"]
    pub BBNSM_PAD_CTRL: crate::RWRegister<u32>,
    _reserved2: [u8; 0x18],
    #[doc = "BBNSM Real-Time Counter LS Register"]
    pub BBNSM_RTC_LS: crate::RWRegister<u32>,
    #[doc = "BBNSM Real-Time Counter MS Register"]
    pub BBNSM_RTC_MS: crate::RWRegister<u32>,
    _reserved3: [u8; 0x08],
    #[doc = "BBNSM Time Alarm Register"]
    pub BBNSM_TA: crate::RWRegister<u32>,
    _reserved4: [u8; 0x02ac],
    #[doc = "General Purpose Register Word word"]
    pub GPR: [crate::RWRegister<u32>; 8usize],
}
#[doc = "BBNSM Version ID Register"]
pub mod BBNSM_VID {
    #[doc = "BBNSM IP ID"]
    pub mod BBNSM_IPID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "BBNSM Revision"]
    pub mod BBNSM_REV {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "BBNSM Version ID"]
    pub mod BBNSM_VID {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "BBNSM Features Register"]
pub mod BBNSM_FEATURES {
    #[doc = "GPR Register Array Size"]
    pub mod GPR_SZ {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "This version of BBNSM does not implement a general-purpose register array."]
            pub const NO_GPR: u32 = 0;
        }
    }
}
#[doc = "BBNSM Control Register"]
pub mod BBNSM_CTRL {
    #[doc = "Real-Time Counter Enable"]
    pub mod RTC_EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable the real-time counter."]
            pub const DISABLED: u32 = 0x01;
            #[doc = "Enable the real-time counter."]
            pub const ENABLED: u32 = 0x02;
        }
    }
    #[doc = "Time Alarm Enable"]
    pub mod TA_EN {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable the time alarm."]
            pub const DISABLED: u32 = 0x01;
            #[doc = "Enable the time alarm. A time alarm event occurs if the value in the real-time counter register is equal to the value in the time alarm register."]
            pub const ENABLED: u32 = 0x02;
        }
    }
    #[doc = "Calibration Enable"]
    pub mod CAL_EN {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "RTC Time calibration is disabled."]
            pub const DISABLED: u32 = 0;
            #[doc = "RTC Time calibration is enabled."]
            pub const ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Calibration Value"]
    pub mod CAL_VAL {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "+0 counts per each 32768 ticks of the counter clock."]
            pub const ADD_0_PER_32768_TICKS: u32 = 0;
            #[doc = "+1 counts per each 32768 ticks of the counter clock."]
            pub const ADD_1_PER_32768_TICKS: u32 = 0x01;
            #[doc = "+2 counts per each 32768 ticks of the counter clock."]
            pub const ADD_2_PER_32768_TICKS: u32 = 0x02;
            #[doc = "+15 counts per each 32768 ticks of the counter clock."]
            pub const ADD_15_PER_32768_TICKS: u32 = 0x0f;
            #[doc = "-16 counts per each 32768 ticks of the counter clock."]
            pub const SUB_16_PER_32768_TICKS: u32 = 0x10;
            #[doc = "-15 counts per each 32768 ticks of the counter clock."]
            pub const SUB_15_PER_32768_TICKS: u32 = 0x11;
            #[doc = "-2 counts per each 32768 ticks of the counter clock."]
            pub const SUB_2_PER_32768_TICKS: u32 = 0x1e;
            #[doc = "-1 counts per each 32768 ticks of the counter clock."]
            pub const SUB_1_PER_32768_TICKS: u32 = 0x1f;
        }
    }
    #[doc = "Button Press Timeout"]
    pub mod BTN_TIMEOUT {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "5 seconds."]
            pub const SEC_5: u32 = 0;
            #[doc = "10 seconds."]
            pub const SEC_10: u32 = 0x01;
            #[doc = "15 seconds."]
            pub const SEC_15: u32 = 0x02;
            #[doc = "Timeout disabled. Long button presses will not request a power down."]
            pub const DISABLE: u32 = 0x03;
        }
    }
    #[doc = "Debounce Time"]
    pub mod DEBOUNCE {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "50 milliseconds."]
            pub const MSEC_50: u32 = 0;
            #[doc = "100 milliseconds."]
            pub const MSEC_100: u32 = 0x01;
            #[doc = "500 milliseconds."]
            pub const MSEC_500: u32 = 0x02;
            #[doc = "0 milliseconds."]
            pub const MSEC_0: u32 = 0x03;
        }
    }
    #[doc = "Turn-On Time"]
    pub mod TURN_ON_TIME {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "500 milliseconds."]
            pub const MSEC_50: u32 = 0;
            #[doc = "50 milliseconds."]
            pub const MSEC_100: u32 = 0x01;
            #[doc = "100 milliseconds."]
            pub const MSEC_500: u32 = 0x02;
            #[doc = "0 milliseconds."]
            pub const MSEC_0: u32 = 0x03;
        }
    }
    #[doc = "PMIC On Request Enable"]
    pub mod PK_EN {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "PMIC On Request is disabled."]
            pub const DISABLED: u32 = 0;
            #[doc = "PMIC On Request is enabled."]
            pub const ENABLED: u32 = 0x01;
        }
    }
    #[doc = "PMIC On Request Override"]
    pub mod PK_OVR {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "PMIC On Request Override is disabled."]
            pub const DISABLED: u32 = 0;
            #[doc = "PMIC On Request Override is enabled."]
            pub const ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Dumb PMIC Enable"]
    pub mod DP_EN {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Smart PMIC is enabled."]
            pub const DISABLED: u32 = 0;
            #[doc = "Dumb PMIC is enabled."]
            pub const ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Turn Off System Power"]
    pub mod TOSP {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Leave system power on."]
            pub const DISABLED: u32 = 0;
            #[doc = "Turn off system power when Dumb PMIC is enabled."]
            pub const ENABLED: u32 = 0x01;
        }
    }
}
#[doc = "BBNSM Interrupt Enable Register"]
pub mod BBNSM_INT_EN {
    #[doc = "Real-Time Counter Rollover Interrupt Enable"]
    pub mod RTC_INT_EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Do not issue an interrupt when RTC has rolled over. The interrupt is cleared when this value is written."]
            pub const DISABLED: u32 = 0x01;
            #[doc = "Issue an interrupt when RTC has rolled over."]
            pub const ENABLED: u32 = 0x02;
        }
    }
    #[doc = "Time Alarm Interrupt Enable"]
    pub mod TA_INT_EN {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Do not issue an interrupt when RTC has reached alarm time. The interrupt is cleared when this value is written."]
            pub const DISABLED: u32 = 0x01;
            #[doc = "Issue an interrupt when RTC has reached alarm time."]
            pub const ENABLED: u32 = 0x02;
        }
    }
}
#[doc = "BBNSM Events Register"]
pub mod BBNSM_EVENTS {
    #[doc = "Real-Time Counter Rollover Event"]
    pub mod RTC_ROLL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The real-time counter has not rolled over."]
            pub const NO_EVENT: u32 = 0x01;
            #[doc = "The real-time counter has rolled over."]
            pub const EVENT_ASSERTED: u32 = 0x02;
        }
    }
    #[doc = "Time Alarm Event"]
    pub mod TA {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The real-time counter has not reached the alarm time."]
            pub const NO_EVENT: u32 = 0x01;
            #[doc = "The real-time counter has reached the alarm time."]
            pub const EVENT_ASSERTED: u32 = 0x02;
        }
    }
    #[doc = "Emergency Off Event"]
    pub mod EMG_OFF {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "An emergency power off has not been requested."]
            pub const NO_EVENT: u32 = 0;
            #[doc = "An emergency power off has been requested."]
            pub const EVENT_ASSERTED: u32 = 0x01;
        }
    }
    #[doc = "Set Power Off Event"]
    pub mod PWR_OFF {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The power off interrupt has not been requested."]
            pub const NO_EVENT: u32 = 0;
            #[doc = "The power off interrupt has been requested."]
            pub const EVENT_ASSERTED: u32 = 0x01;
        }
    }
    #[doc = "Set Power On Event"]
    pub mod PWR_ON {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The power on interrupt has not been requested."]
            pub const NO_EVENT: u32 = 0;
            #[doc = "The power on interrupt has been requested."]
            pub const EVENT_ASSERTED: u32 = 0x01;
        }
    }
}
#[doc = "BBNSM External Pad Control Register"]
pub mod BBNSM_PAD_CTRL {
    #[doc = "Control I/O Pads"]
    pub mod PAD_CTRL0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Deasserts bit n in bbnsm_pad_ctrl\\[n\\]"]
            pub const ZERO: u32 = 0;
            #[doc = "Assert bit n in bbnsm_pad_ctrl\\[n\\]"]
            pub const ONE: u32 = 0x01;
        }
    }
    #[doc = "Control I/O Pads"]
    pub mod PAD_CTRL1 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Deasserts bit n in bbnsm_pad_ctrl\\[n\\]"]
            pub const ZERO: u32 = 0;
            #[doc = "Assert bit n in bbnsm_pad_ctrl\\[n\\]"]
            pub const ONE: u32 = 0x01;
        }
    }
    #[doc = "Control I/O Pads"]
    pub mod PAD_CTRL2 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Deasserts bit n in bbnsm_pad_ctrl\\[n\\]"]
            pub const ZERO: u32 = 0;
            #[doc = "Assert bit n in bbnsm_pad_ctrl\\[n\\]"]
            pub const ONE: u32 = 0x01;
        }
    }
    #[doc = "Control I/O Pads"]
    pub mod PAD_CTRL3 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Deasserts bit n in bbnsm_pad_ctrl\\[n\\]"]
            pub const ZERO: u32 = 0;
            #[doc = "Assert bit n in bbnsm_pad_ctrl\\[n\\]"]
            pub const ONE: u32 = 0x01;
        }
    }
    #[doc = "Control I/O Pads"]
    pub mod PAD_CTRL4 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Deasserts bit n in bbnsm_pad_ctrl\\[n\\]"]
            pub const ZERO: u32 = 0;
            #[doc = "Assert bit n in bbnsm_pad_ctrl\\[n\\]"]
            pub const ONE: u32 = 0x01;
        }
    }
    #[doc = "Control I/O Pads"]
    pub mod PAD_CTRL5 {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Deasserts bit n in bbnsm_pad_ctrl\\[n\\]"]
            pub const ZERO: u32 = 0;
            #[doc = "Assert bit n in bbnsm_pad_ctrl\\[n\\]"]
            pub const ONE: u32 = 0x01;
        }
    }
    #[doc = "Control I/O Pads"]
    pub mod PAD_CTRL6 {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Deasserts bit n in bbnsm_pad_ctrl\\[n\\]"]
            pub const ZERO: u32 = 0;
            #[doc = "Assert bit n in bbnsm_pad_ctrl\\[n\\]"]
            pub const ONE: u32 = 0x01;
        }
    }
    #[doc = "Control I/O Pads"]
    pub mod PAD_CTRL7 {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Deasserts bit n in bbnsm_pad_ctrl\\[n\\]"]
            pub const ZERO: u32 = 0;
            #[doc = "Assert bit n in bbnsm_pad_ctrl\\[n\\]"]
            pub const ONE: u32 = 0x01;
        }
    }
    #[doc = "Control I/O Pads"]
    pub mod PAD_CTRL8 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Deasserts bit n in bbnsm_pad_ctrl\\[n\\]"]
            pub const ZERO: u32 = 0;
            #[doc = "Assert bit n in bbnsm_pad_ctrl\\[n\\]"]
            pub const ONE: u32 = 0x01;
        }
    }
    #[doc = "Control I/O Pads"]
    pub mod PAD_CTRL9 {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Deasserts bit n in bbnsm_pad_ctrl\\[n\\]"]
            pub const ZERO: u32 = 0;
            #[doc = "Assert bit n in bbnsm_pad_ctrl\\[n\\]"]
            pub const ONE: u32 = 0x01;
        }
    }
    #[doc = "Control I/O Pads"]
    pub mod PAD_CTRL10 {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Deasserts bit n in bbnsm_pad_ctrl\\[n\\]"]
            pub const ZERO: u32 = 0;
            #[doc = "Assert bit n in bbnsm_pad_ctrl\\[n\\]"]
            pub const ONE: u32 = 0x01;
        }
    }
    #[doc = "Control I/O Pads"]
    pub mod PAD_CTRL11 {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Deasserts bit n in bbnsm_pad_ctrl\\[n\\]"]
            pub const ZERO: u32 = 0;
            #[doc = "Assert bit n in bbnsm_pad_ctrl\\[n\\]"]
            pub const ONE: u32 = 0x01;
        }
    }
    #[doc = "Control I/O Pads"]
    pub mod PAD_CTRL12 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Deasserts bit n in bbnsm_pad_ctrl\\[n\\]"]
            pub const ZERO: u32 = 0;
            #[doc = "Assert bit n in bbnsm_pad_ctrl\\[n\\]"]
            pub const ONE: u32 = 0x01;
        }
    }
    #[doc = "Control I/O Pads"]
    pub mod PAD_CTRL13 {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Deasserts bit n in bbnsm_pad_ctrl\\[n\\]"]
            pub const ZERO: u32 = 0;
            #[doc = "Assert bit n in bbnsm_pad_ctrl\\[n\\]"]
            pub const ONE: u32 = 0x01;
        }
    }
    #[doc = "Control I/O Pads"]
    pub mod PAD_CTRL14 {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Deasserts bit n in bbnsm_pad_ctrl\\[n\\]"]
            pub const ZERO: u32 = 0;
            #[doc = "Assert bit n in bbnsm_pad_ctrl\\[n\\]"]
            pub const ONE: u32 = 0x01;
        }
    }
    #[doc = "Control I/O Pads"]
    pub mod PAD_CTRL15 {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Deasserts bit n in bbnsm_pad_ctrl\\[n\\]"]
            pub const ZERO: u32 = 0;
            #[doc = "Assert bit n in bbnsm_pad_ctrl\\[n\\]"]
            pub const ONE: u32 = 0x01;
        }
    }
}
#[doc = "BBNSM Real-Time Counter LS Register"]
pub mod BBNSM_RTC_LS {
    #[doc = "Real-time Counter"]
    pub mod RTC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "BBNSM Real-Time Counter MS Register"]
pub mod BBNSM_RTC_MS {
    #[doc = "Real-Time Counter"]
    pub mod RTC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x7fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "BBNSM Time Alarm Register"]
pub mod BBNSM_TA {
    #[doc = "Time Alarm Value"]
    pub mod TA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "General Purpose Register Word word"]
pub mod GPR {
    #[doc = "32 bits of the GPR."]
    pub mod GPR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
