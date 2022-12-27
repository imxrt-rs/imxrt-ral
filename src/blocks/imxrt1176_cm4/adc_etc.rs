#[doc = "ADC_ETC"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "ADC_ETC Global Control Register"]
    pub CTRL: crate::RWRegister<u32>,
    #[doc = "ETC DONE0 and DONE1 IRQ State Register"]
    pub DONE0_1_IRQ: crate::RWRegister<u32>,
    #[doc = "ETC DONE_2, DONE_3 and DONE_ERR IRQ State Register"]
    pub DONE2_3_ERR_IRQ: crate::RWRegister<u32>,
    #[doc = "ETC DMA control Register"]
    pub DMA_CTRL: crate::RWRegister<u32>,
    #[doc = "ETC_TRIG Control Register"]
    pub TRIG0_CTRL: crate::RWRegister<u32>,
    #[doc = "ETC_TRIG Counter Register"]
    pub TRIG0_COUNTER: crate::RWRegister<u32>,
    #[doc = "ETC_TRIG Chain 0/1 Register"]
    pub TRIG0_CHAIN_1_0: crate::RWRegister<u32>,
    #[doc = "ETC_TRIG Chain 2/3 Register"]
    pub TRIG0_CHAIN_3_2: crate::RWRegister<u32>,
    #[doc = "ETC_TRIG Chain 4/5 Register"]
    pub TRIG0_CHAIN_5_4: crate::RWRegister<u32>,
    #[doc = "ETC_TRIG Chain 6/7 Register"]
    pub TRIG0_CHAIN_7_6: crate::RWRegister<u32>,
    #[doc = "ETC_TRIG Result Data 1/0 Register"]
    pub TRIG0_RESULT_1_0: crate::RORegister<u32>,
    #[doc = "ETC_TRIG Result Data 3/2 Register"]
    pub TRIG0_RESULT_3_2: crate::RORegister<u32>,
    #[doc = "ETC_TRIG Result Data 5/4 Register"]
    pub TRIG0_RESULT_5_4: crate::RORegister<u32>,
    #[doc = "ETC_TRIG Result Data 7/6 Register"]
    pub TRIG0_RESULT_7_6: crate::RORegister<u32>,
    #[doc = "ETC_TRIG Control Register"]
    pub TRIG1_CTRL: crate::RWRegister<u32>,
    #[doc = "ETC_TRIG Counter Register"]
    pub TRIG1_COUNTER: crate::RWRegister<u32>,
    #[doc = "ETC_TRIG Chain 0/1 Register"]
    pub TRIG1_CHAIN_1_0: crate::RWRegister<u32>,
    #[doc = "ETC_TRIG Chain 2/3 Register"]
    pub TRIG1_CHAIN_3_2: crate::RWRegister<u32>,
    #[doc = "ETC_TRIG Chain 4/5 Register"]
    pub TRIG1_CHAIN_5_4: crate::RWRegister<u32>,
    #[doc = "ETC_TRIG Chain 6/7 Register"]
    pub TRIG1_CHAIN_7_6: crate::RWRegister<u32>,
    #[doc = "ETC_TRIG Result Data 1/0 Register"]
    pub TRIG1_RESULT_1_0: crate::RORegister<u32>,
    #[doc = "ETC_TRIG Result Data 3/2 Register"]
    pub TRIG1_RESULT_3_2: crate::RORegister<u32>,
    #[doc = "ETC_TRIG Result Data 5/4 Register"]
    pub TRIG1_RESULT_5_4: crate::RORegister<u32>,
    #[doc = "ETC_TRIG Result Data 7/6 Register"]
    pub TRIG1_RESULT_7_6: crate::RORegister<u32>,
    #[doc = "ETC_TRIG Control Register"]
    pub TRIG2_CTRL: crate::RWRegister<u32>,
    #[doc = "ETC_TRIG Counter Register"]
    pub TRIG2_COUNTER: crate::RWRegister<u32>,
    #[doc = "ETC_TRIG Chain 0/1 Register"]
    pub TRIG2_CHAIN_1_0: crate::RWRegister<u32>,
    #[doc = "ETC_TRIG Chain 2/3 Register"]
    pub TRIG2_CHAIN_3_2: crate::RWRegister<u32>,
    #[doc = "ETC_TRIG Chain 4/5 Register"]
    pub TRIG2_CHAIN_5_4: crate::RWRegister<u32>,
    #[doc = "ETC_TRIG Chain 6/7 Register"]
    pub TRIG2_CHAIN_7_6: crate::RWRegister<u32>,
    #[doc = "ETC_TRIG Result Data 1/0 Register"]
    pub TRIG2_RESULT_1_0: crate::RORegister<u32>,
    #[doc = "ETC_TRIG Result Data 3/2 Register"]
    pub TRIG2_RESULT_3_2: crate::RORegister<u32>,
    #[doc = "ETC_TRIG Result Data 5/4 Register"]
    pub TRIG2_RESULT_5_4: crate::RORegister<u32>,
    #[doc = "ETC_TRIG Result Data 7/6 Register"]
    pub TRIG2_RESULT_7_6: crate::RORegister<u32>,
    #[doc = "ETC_TRIG Control Register"]
    pub TRIG3_CTRL: crate::RWRegister<u32>,
    #[doc = "ETC_TRIG Counter Register"]
    pub TRIG3_COUNTER: crate::RWRegister<u32>,
    #[doc = "ETC_TRIG Chain 0/1 Register"]
    pub TRIG3_CHAIN_1_0: crate::RWRegister<u32>,
    #[doc = "ETC_TRIG Chain 2/3 Register"]
    pub TRIG3_CHAIN_3_2: crate::RWRegister<u32>,
    #[doc = "ETC_TRIG Chain 4/5 Register"]
    pub TRIG3_CHAIN_5_4: crate::RWRegister<u32>,
    #[doc = "ETC_TRIG Chain 6/7 Register"]
    pub TRIG3_CHAIN_7_6: crate::RWRegister<u32>,
    #[doc = "ETC_TRIG Result Data 1/0 Register"]
    pub TRIG3_RESULT_1_0: crate::RORegister<u32>,
    #[doc = "ETC_TRIG Result Data 3/2 Register"]
    pub TRIG3_RESULT_3_2: crate::RORegister<u32>,
    #[doc = "ETC_TRIG Result Data 5/4 Register"]
    pub TRIG3_RESULT_5_4: crate::RORegister<u32>,
    #[doc = "ETC_TRIG Result Data 7/6 Register"]
    pub TRIG3_RESULT_7_6: crate::RORegister<u32>,
    #[doc = "ETC_TRIG Control Register"]
    pub TRIG4_CTRL: crate::RWRegister<u32>,
    #[doc = "ETC_TRIG Counter Register"]
    pub TRIG4_COUNTER: crate::RWRegister<u32>,
    #[doc = "ETC_TRIG Chain 0/1 Register"]
    pub TRIG4_CHAIN_1_0: crate::RWRegister<u32>,
    #[doc = "ETC_TRIG Chain 2/3 Register"]
    pub TRIG4_CHAIN_3_2: crate::RWRegister<u32>,
    #[doc = "ETC_TRIG Chain 4/5 Register"]
    pub TRIG4_CHAIN_5_4: crate::RWRegister<u32>,
    #[doc = "ETC_TRIG Chain 6/7 Register"]
    pub TRIG4_CHAIN_7_6: crate::RWRegister<u32>,
    #[doc = "ETC_TRIG Result Data 1/0 Register"]
    pub TRIG4_RESULT_1_0: crate::RORegister<u32>,
    #[doc = "ETC_TRIG Result Data 3/2 Register"]
    pub TRIG4_RESULT_3_2: crate::RORegister<u32>,
    #[doc = "ETC_TRIG Result Data 5/4 Register"]
    pub TRIG4_RESULT_5_4: crate::RORegister<u32>,
    #[doc = "ETC_TRIG Result Data 7/6 Register"]
    pub TRIG4_RESULT_7_6: crate::RORegister<u32>,
    #[doc = "ETC_TRIG Control Register"]
    pub TRIG5_CTRL: crate::RWRegister<u32>,
    #[doc = "ETC_TRIG Counter Register"]
    pub TRIG5_COUNTER: crate::RWRegister<u32>,
    #[doc = "ETC_TRIG Chain 0/1 Register"]
    pub TRIG5_CHAIN_1_0: crate::RWRegister<u32>,
    #[doc = "ETC_TRIG Chain 2/3 Register"]
    pub TRIG5_CHAIN_3_2: crate::RWRegister<u32>,
    #[doc = "ETC_TRIG Chain 4/5 Register"]
    pub TRIG5_CHAIN_5_4: crate::RWRegister<u32>,
    #[doc = "ETC_TRIG Chain 6/7 Register"]
    pub TRIG5_CHAIN_7_6: crate::RWRegister<u32>,
    #[doc = "ETC_TRIG Result Data 1/0 Register"]
    pub TRIG5_RESULT_1_0: crate::RORegister<u32>,
    #[doc = "ETC_TRIG Result Data 3/2 Register"]
    pub TRIG5_RESULT_3_2: crate::RORegister<u32>,
    #[doc = "ETC_TRIG Result Data 5/4 Register"]
    pub TRIG5_RESULT_5_4: crate::RORegister<u32>,
    #[doc = "ETC_TRIG Result Data 7/6 Register"]
    pub TRIG5_RESULT_7_6: crate::RORegister<u32>,
    #[doc = "ETC_TRIG Control Register"]
    pub TRIG6_CTRL: crate::RWRegister<u32>,
    #[doc = "ETC_TRIG Counter Register"]
    pub TRIG6_COUNTER: crate::RWRegister<u32>,
    #[doc = "ETC_TRIG Chain 0/1 Register"]
    pub TRIG6_CHAIN_1_0: crate::RWRegister<u32>,
    #[doc = "ETC_TRIG Chain 2/3 Register"]
    pub TRIG6_CHAIN_3_2: crate::RWRegister<u32>,
    #[doc = "ETC_TRIG Chain 4/5 Register"]
    pub TRIG6_CHAIN_5_4: crate::RWRegister<u32>,
    #[doc = "ETC_TRIG Chain 6/7 Register"]
    pub TRIG6_CHAIN_7_6: crate::RWRegister<u32>,
    #[doc = "ETC_TRIG Result Data 1/0 Register"]
    pub TRIG6_RESULT_1_0: crate::RORegister<u32>,
    #[doc = "ETC_TRIG Result Data 3/2 Register"]
    pub TRIG6_RESULT_3_2: crate::RORegister<u32>,
    #[doc = "ETC_TRIG Result Data 5/4 Register"]
    pub TRIG6_RESULT_5_4: crate::RORegister<u32>,
    #[doc = "ETC_TRIG Result Data 7/6 Register"]
    pub TRIG6_RESULT_7_6: crate::RORegister<u32>,
    #[doc = "ETC_TRIG Control Register"]
    pub TRIG7_CTRL: crate::RWRegister<u32>,
    #[doc = "ETC_TRIG Counter Register"]
    pub TRIG7_COUNTER: crate::RWRegister<u32>,
    #[doc = "ETC_TRIG Chain 0/1 Register"]
    pub TRIG7_CHAIN_1_0: crate::RWRegister<u32>,
    #[doc = "ETC_TRIG Chain 2/3 Register"]
    pub TRIG7_CHAIN_3_2: crate::RWRegister<u32>,
    #[doc = "ETC_TRIG Chain 4/5 Register"]
    pub TRIG7_CHAIN_5_4: crate::RWRegister<u32>,
    #[doc = "ETC_TRIG Chain 6/7 Register"]
    pub TRIG7_CHAIN_7_6: crate::RWRegister<u32>,
    #[doc = "ETC_TRIG Result Data 1/0 Register"]
    pub TRIG7_RESULT_1_0: crate::RORegister<u32>,
    #[doc = "ETC_TRIG Result Data 3/2 Register"]
    pub TRIG7_RESULT_3_2: crate::RORegister<u32>,
    #[doc = "ETC_TRIG Result Data 5/4 Register"]
    pub TRIG7_RESULT_5_4: crate::RORegister<u32>,
    #[doc = "ETC_TRIG Result Data 7/6 Register"]
    pub TRIG7_RESULT_7_6: crate::RORegister<u32>,
}
#[doc = "ADC_ETC Global Control Register"]
pub mod CTRL {
    #[doc = "TRIG enable register."]
    pub mod TRIG_ENABLE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "disable all 8 external XBAR triggers."]
            pub const TRIG_ENABLE_0: u32 = 0;
            #[doc = "enable external XBAR trigger0."]
            pub const TRIG_ENABLE_1: u32 = 0x01;
            #[doc = "enable external XBAR trigger1."]
            pub const TRIG_ENABLE_2: u32 = 0x02;
            #[doc = "enable external XBAR trigger0 and trigger1."]
            pub const TRIG_ENABLE_3: u32 = 0x03;
            #[doc = "enable all 8 external XBAR triggers."]
            pub const TRIG_ENABLE_255: u32 = 0xff;
        }
    }
    #[doc = "Pre-divider for trig delay and interval"]
    pub mod PRE_DIVIDER {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Select the trigger type of the DMA_REQ."]
    pub mod DMA_MODE_SEL {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Trig DMA_REQ with latched signal, REQ will be cleared when ACK and source request cleared."]
            pub const DMA_MODE_SEL_0: u32 = 0;
            #[doc = "Trig DMA_REQ with pulsed signal, REQ will be cleared by ACK only."]
            pub const DMA_MODE_SEL_1: u32 = 0x01;
        }
    }
    #[doc = "Software synchronous reset, active high."]
    pub mod SOFTRST {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ADC_ETC works normally."]
            pub const SOFTRST_0: u32 = 0;
            #[doc = "All registers inside ADC_ETC will be reset to the default value."]
            pub const SOFTRST_1: u32 = 0x01;
        }
    }
}
#[doc = "ETC DONE0 and DONE1 IRQ State Register"]
pub mod DONE0_1_IRQ {
    #[doc = "TRIG0 done0 interrupt detection."]
    pub mod TRIG0_DONE0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No TRIG0_DONE0 interrupt detected"]
            pub const TRIG0_DONE0_0: u32 = 0;
            #[doc = "TRIG0_DONE0 interrupt detected"]
            pub const TRIG0_DONE0_1: u32 = 0x01;
        }
    }
    #[doc = "TRIG1 done0 interrupt detection."]
    pub mod TRIG1_DONE0 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No TRIG1_DONE0 interrupt detected"]
            pub const TRIG1_DONE0_0: u32 = 0;
            #[doc = "TRIG1_DONE0 interrupt detected"]
            pub const TRIG1_DONE0_1: u32 = 0x01;
        }
    }
    #[doc = "TRIG2 done0 interrupt detection."]
    pub mod TRIG2_DONE0 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No TRIG2_DONE0 interrupt detected"]
            pub const TRIG2_DONE0_0: u32 = 0;
            #[doc = "TRIG2_DONE0 interrupt detected"]
            pub const TRIG2_DONE0_1: u32 = 0x01;
        }
    }
    #[doc = "TRIG3 done0 interrupt detection."]
    pub mod TRIG3_DONE0 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No TRIG3_DONE0 interrupt detected"]
            pub const TRIG3_DONE0_0: u32 = 0;
            #[doc = "TRIG3_DONE0 interrupt detected"]
            pub const TRIG3_DONE0_1: u32 = 0x01;
        }
    }
    #[doc = "TRIG4 done0 interrupt detection."]
    pub mod TRIG4_DONE0 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No TRIG4_DONE0 interrupt detected"]
            pub const TRIG4_DONE0_0: u32 = 0;
            #[doc = "TRIG4_DONE0 interrupt detected"]
            pub const TRIG4_DONE0_1: u32 = 0x01;
        }
    }
    #[doc = "TRIG5 done0 interrupt detection."]
    pub mod TRIG5_DONE0 {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No TRIG5_DONE0 interrupt detected"]
            pub const TRIG5_DONE0_0: u32 = 0;
            #[doc = "TRIG5_DONE0 interrupt detected"]
            pub const TRIG5_DONE0_1: u32 = 0x01;
        }
    }
    #[doc = "TRIG6 done0 interrupt detection."]
    pub mod TRIG6_DONE0 {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No TRIG6_DONE0 interrupt detected"]
            pub const TRIG6_DONE0_0: u32 = 0;
            #[doc = "TRIG6_DONE0 interrupt detected"]
            pub const TRIG6_DONE0_1: u32 = 0x01;
        }
    }
    #[doc = "TRIG7 done0 interrupt detection."]
    pub mod TRIG7_DONE0 {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No TRIG7_DONE0 interrupt detected"]
            pub const TRIG7_DONE0_0: u32 = 0;
            #[doc = "TRIG7_DONE0 interrupt detected"]
            pub const TRIG7_DONE0_1: u32 = 0x01;
        }
    }
    #[doc = "TRIG0 done1 interrupt detection."]
    pub mod TRIG0_DONE1 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No TRIG0_DONE1 interrupt detected"]
            pub const TRIG0_DONE1_0: u32 = 0;
            #[doc = "TRIG0_DONE1 interrupt detected"]
            pub const TRIG0_DONE1_1: u32 = 0x01;
        }
    }
    #[doc = "TRIG1 done1 interrupt detection."]
    pub mod TRIG1_DONE1 {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No TRIG1_DONE1 interrupt detected"]
            pub const TRIG1_DONE1_0: u32 = 0;
            #[doc = "TRIG1_DONE1 interrupt detected"]
            pub const TRIG1_DONE1_1: u32 = 0x01;
        }
    }
    #[doc = "TRIG2 done1 interrupt detection."]
    pub mod TRIG2_DONE1 {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No TRIG2_DONE1 interrupt detected"]
            pub const TRIG2_DONE1_0: u32 = 0;
            #[doc = "TRIG2_DONE1 interrupt detected"]
            pub const TRIG2_DONE1_1: u32 = 0x01;
        }
    }
    #[doc = "TRIG3 done1 interrupt detection."]
    pub mod TRIG3_DONE1 {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No TRIG3_DONE1 interrupt detected"]
            pub const TRIG3_DONE1_0: u32 = 0;
            #[doc = "TRIG3_DONE1 interrupt detected"]
            pub const TRIG3_DONE1_1: u32 = 0x01;
        }
    }
    #[doc = "TRIG4 done1 interrupt detection."]
    pub mod TRIG4_DONE1 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No TRIG4_DONE1 interrupt detected"]
            pub const TRIG4_DONE1_0: u32 = 0;
            #[doc = "TRIG4_DONE1 interrupt detected"]
            pub const TRIG4_DONE1_1: u32 = 0x01;
        }
    }
    #[doc = "TRIG5 done1 interrupt detection."]
    pub mod TRIG5_DONE1 {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No TRIG5_DONE1 interrupt detected"]
            pub const TRIG5_DONE1_0: u32 = 0;
            #[doc = "TRIG5_DONE1 interrupt detected"]
            pub const TRIG5_DONE1_1: u32 = 0x01;
        }
    }
    #[doc = "TRIG6 done1 interrupt detection."]
    pub mod TRIG6_DONE1 {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No TRIG6_DONE1 interrupt detected"]
            pub const TRIG6_DONE1_0: u32 = 0;
            #[doc = "TRIG6_DONE1 interrupt detected"]
            pub const TRIG6_DONE1_1: u32 = 0x01;
        }
    }
    #[doc = "TRIG7 done1 interrupt detection."]
    pub mod TRIG7_DONE1 {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No TRIG7_DONE1 interrupt detected"]
            pub const TRIG7_DONE1_0: u32 = 0;
            #[doc = "TRIG7_DONE1 interrupt detected"]
            pub const TRIG7_DONE1_1: u32 = 0x01;
        }
    }
}
#[doc = "ETC DONE_2, DONE_3 and DONE_ERR IRQ State Register"]
pub mod DONE2_3_ERR_IRQ {
    #[doc = "TRIG0 done2 interrupt detection."]
    pub mod TRIG0_DONE2 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No TRIG0_DONE2 interrupt detected"]
            pub const TRIG0_DONE2_0: u32 = 0;
            #[doc = "TRIG0_DONE2 interrupt detected"]
            pub const TRIG0_DONE2_1: u32 = 0x01;
        }
    }
    #[doc = "TRIG1 done2 interrupt detection."]
    pub mod TRIG1_DONE2 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No TRIG1_DONE2 interrupt detected"]
            pub const TRIG1_DONE2_0: u32 = 0;
            #[doc = "TRIG1_DONE2 interrupt detected"]
            pub const TRIG1_DONE2_1: u32 = 0x01;
        }
    }
    #[doc = "TRIG2 done2 interrupt detection."]
    pub mod TRIG2_DONE2 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No TRIG2_DONE2 interrupt detected"]
            pub const TRIG2_DONE2_0: u32 = 0;
            #[doc = "TRIG2_DONE2 interrupt detected"]
            pub const TRIG2_DONE2_1: u32 = 0x01;
        }
    }
    #[doc = "TRIG3 done2 interrupt detection."]
    pub mod TRIG3_DONE2 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No TRIG3_DONE2 interrupt detected"]
            pub const TRIG3_DONE2_0: u32 = 0;
            #[doc = "TRIG3_DONE2 interrupt detected"]
            pub const TRIG3_DONE2_1: u32 = 0x01;
        }
    }
    #[doc = "TRIG4 done2 interrupt detection."]
    pub mod TRIG4_DONE2 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No TRIG4_DONE2 interrupt detected"]
            pub const TRIG4_DONE2_0: u32 = 0;
            #[doc = "TRIG4_DONE2 interrupt detected"]
            pub const TRIG4_DONE2_1: u32 = 0x01;
        }
    }
    #[doc = "TRIG5 done2 interrupt detection."]
    pub mod TRIG5_DONE2 {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No TRIG5_DONE2 interrupt detected"]
            pub const TRIG5_DONE2_0: u32 = 0;
            #[doc = "TRIG5_DONE2 interrupt detected"]
            pub const TRIG5_DONE2_1: u32 = 0x01;
        }
    }
    #[doc = "TRIG6 done2 interrupt detection."]
    pub mod TRIG6_DONE2 {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No TRIG6_DONE2 interrupt detected"]
            pub const TRIG6_DONE2_0: u32 = 0;
            #[doc = "TRIG6_DONE2 interrupt detected"]
            pub const TRIG6_DONE2_1: u32 = 0x01;
        }
    }
    #[doc = "TRIG7 done2 interrupt detection."]
    pub mod TRIG7_DONE2 {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No TRIG7_DONE2 interrupt detected"]
            pub const TRIG7_DONE2_0: u32 = 0;
            #[doc = "TRIG7_DONE2 interrupt detected"]
            pub const TRIG7_DONE2_1: u32 = 0x01;
        }
    }
    #[doc = "TRIG0 done3 interrupt detection."]
    pub mod TRIG0_DONE3 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No TRIG0_DONE3 interrupt detected"]
            pub const TRIG0_DONE3_0: u32 = 0;
            #[doc = "TRIG0_DONE3 interrupt detected"]
            pub const TRIG0_DONE3_1: u32 = 0x01;
        }
    }
    #[doc = "TRIG1 done3 interrupt detection."]
    pub mod TRIG1_DONE3 {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No TRIG1_DONE3 interrupt detected"]
            pub const TRIG1_DONE3_0: u32 = 0;
            #[doc = "TRIG1_DONE3 interrupt detected"]
            pub const TRIG1_DONE3_1: u32 = 0x01;
        }
    }
    #[doc = "TRIG2 done3 interrupt detection."]
    pub mod TRIG2_DONE3 {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No TRIG2_DONE3 interrupt detected"]
            pub const TRIG2_DONE3_0: u32 = 0;
            #[doc = "TRIG2_DONE3 interrupt detected"]
            pub const TRIG2_DONE3_1: u32 = 0x01;
        }
    }
    #[doc = "TRIG3 done3 interrupt detection."]
    pub mod TRIG3_DONE3 {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No TRIG3_DONE3 interrupt detected"]
            pub const TRIG3_DONE3_0: u32 = 0;
            #[doc = "TRIG3_DONE3 interrupt detected"]
            pub const TRIG3_DONE3_1: u32 = 0x01;
        }
    }
    #[doc = "TRIG4 done3 interrupt detection."]
    pub mod TRIG4_DONE3 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No TRIG4_DONE3 interrupt detected"]
            pub const TRIG4_DONE3_0: u32 = 0;
            #[doc = "TRIG4_DONE3 interrupt detected"]
            pub const TRIG4_DONE3_1: u32 = 0x01;
        }
    }
    #[doc = "TRIG5 done3 interrupt detection."]
    pub mod TRIG5_DONE3 {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No TRIG5_DONE3 interrupt detected"]
            pub const TRIG5_DONE3_0: u32 = 0;
            #[doc = "TRIG5_DONE3 interrupt detected"]
            pub const TRIG5_DONE3_1: u32 = 0x01;
        }
    }
    #[doc = "TRIG6 done3 interrupt detection."]
    pub mod TRIG6_DONE3 {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No TRIG6_DONE3 interrupt detected"]
            pub const TRIG6_DONE3_0: u32 = 0;
            #[doc = "TRIG6_DONE3 interrupt detected"]
            pub const TRIG6_DONE3_1: u32 = 0x01;
        }
    }
    #[doc = "TRIG7 done3 interrupt detection."]
    pub mod TRIG7_DONE3 {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No TRIG7_DONE3 interrupt detected"]
            pub const TRIG7_DONE3_0: u32 = 0;
            #[doc = "TRIG7_DONE3 interrupt detected"]
            pub const TRIG7_DONE3_1: u32 = 0x01;
        }
    }
    #[doc = "TRIG0 error interrupt detection."]
    pub mod TRIG0_ERR {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No TRIG0_ERR interrupt detected"]
            pub const TRIG0_ERR_0: u32 = 0;
            #[doc = "TRIG0_ERR interrupt detected"]
            pub const TRIG0_ERR_1: u32 = 0x01;
        }
    }
    #[doc = "TRIG1 error interrupt detection."]
    pub mod TRIG1_ERR {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No TRIG1_ERR interrupt detected"]
            pub const TRIG1_ERR_0: u32 = 0;
            #[doc = "TRIG1_ERR interrupt detected"]
            pub const TRIG1_ERR_1: u32 = 0x01;
        }
    }
    #[doc = "TRIG2 error interrupt detection."]
    pub mod TRIG2_ERR {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No TRIG2_ERR interrupt detected"]
            pub const TRIG2_ERR_0: u32 = 0;
            #[doc = "TRIG2_ERR interrupt detected"]
            pub const TRIG2_ERR_1: u32 = 0x01;
        }
    }
    #[doc = "TRIG3 error interrupt detection."]
    pub mod TRIG3_ERR {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No TRIG3_ERR interrupt detected"]
            pub const TRIG3_ERR_0: u32 = 0;
            #[doc = "TRIG3_ERR interrupt detected"]
            pub const TRIG3_ERR_1: u32 = 0x01;
        }
    }
    #[doc = "TRIG4 error interrupt detection."]
    pub mod TRIG4_ERR {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No TRIG4_ERR interrupt detected"]
            pub const TRIG4_ERR_0: u32 = 0;
            #[doc = "TRIG4_ERR interrupt detected"]
            pub const TRIG4_ERR_1: u32 = 0x01;
        }
    }
    #[doc = "TRIG5 error interrupt detection."]
    pub mod TRIG5_ERR {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No TRIG5_ERR interrupt detected"]
            pub const TRIG5_ERR_0: u32 = 0;
            #[doc = "TRIG5_ERR interrupt detected"]
            pub const TRIG5_ERR_1: u32 = 0x01;
        }
    }
    #[doc = "TRIG6 error interrupt detection."]
    pub mod TRIG6_ERR {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No TRIG6_ERR interrupt detected"]
            pub const TRIG6_ERR_0: u32 = 0;
            #[doc = "TRIG6_ERR interrupt detected"]
            pub const TRIG6_ERR_1: u32 = 0x01;
        }
    }
    #[doc = "TRIG7 error interrupt detection."]
    pub mod TRIG7_ERR {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No TRIG7_ERR interrupt detected"]
            pub const TRIG7_ERR_0: u32 = 0;
            #[doc = "TRIG7_ERR interrupt detected"]
            pub const TRIG7_ERR_1: u32 = 0x01;
        }
    }
}
#[doc = "ETC DMA control Register"]
pub mod DMA_CTRL {
    #[doc = "Enable DMA request when TRIG0 done."]
    pub mod TRIG0_ENABLE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "TRIG0 DMA request disabled."]
            pub const TRIG0_ENABLE_0: u32 = 0;
            #[doc = "TRIG0 DMA request enabled."]
            pub const TRIG0_ENABLE_1: u32 = 0x01;
        }
    }
    #[doc = "Enable DMA request when TRIG1 done."]
    pub mod TRIG1_ENABLE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "TRIG1 DMA request disabled."]
            pub const TRIG1_ENABLE_0: u32 = 0;
            #[doc = "TRIG1 DMA request enabled."]
            pub const TRIG1_ENABLE_1: u32 = 0x01;
        }
    }
    #[doc = "Enable DMA request when TRIG2 done."]
    pub mod TRIG2_ENABLE {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "TRIG2 DMA request disabled."]
            pub const TRIG2_ENABLE_0: u32 = 0;
            #[doc = "TRIG2 DMA request enabled."]
            pub const TRIG2_ENABLE_1: u32 = 0x01;
        }
    }
    #[doc = "Enable DMA request when TRIG3 done."]
    pub mod TRIG3_ENABLE {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "TRIG3 DMA request disabled."]
            pub const TRIG3_ENABLE_0: u32 = 0;
            #[doc = "TRIG3 DMA request enabled."]
            pub const TRIG3_ENABLE_1: u32 = 0x01;
        }
    }
    #[doc = "Enable DMA request when TRIG4 done."]
    pub mod TRIG4_ENABLE {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "TRIG4 DMA request disabled."]
            pub const TRIG4_ENABLE_0: u32 = 0;
            #[doc = "TRIG4 DMA request enabled."]
            pub const TRIG4_ENABLE_1: u32 = 0x01;
        }
    }
    #[doc = "Enable DMA request when TRIG5 done."]
    pub mod TRIG5_ENABLE {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "TRIG5 DMA request disabled."]
            pub const TRIG5_ENABLE_0: u32 = 0;
            #[doc = "TRIG5 DMA request enabled."]
            pub const TRIG5_ENABLE_1: u32 = 0x01;
        }
    }
    #[doc = "Enable DMA request when TRIG6 done."]
    pub mod TRIG6_ENABLE {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "TRIG6 DMA request disabled."]
            pub const TRIG6_ENABLE_0: u32 = 0;
            #[doc = "TRIG6 DMA request enabled."]
            pub const TRIG6_ENABLE_1: u32 = 0x01;
        }
    }
    #[doc = "Enable DMA request when TRIG7 done."]
    pub mod TRIG7_ENABLE {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "TRIG7 DMA request disabled."]
            pub const TRIG7_ENABLE_0: u32 = 0;
            #[doc = "TRIG7 DMA request enabled."]
            pub const TRIG7_ENABLE_1: u32 = 0x01;
        }
    }
    #[doc = "Flag bit for DMA request"]
    pub mod TRIG0_REQ {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "TRIG0_REQ not detected."]
            pub const TRIG0_REQ_0: u32 = 0;
            #[doc = "TRIG0_REQ detected."]
            pub const TRIG0_REQ_1: u32 = 0x01;
        }
    }
    #[doc = "Flag bit for DMA request"]
    pub mod TRIG1_REQ {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "TRIG1_REQ not detected."]
            pub const TRIG1_REQ_0: u32 = 0;
            #[doc = "TRIG1_REQ detected."]
            pub const TRIG1_REQ_1: u32 = 0x01;
        }
    }
    #[doc = "Flag bit for DMA request"]
    pub mod TRIG2_REQ {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "TRIG2_REQ not detected."]
            pub const TRIG2_REQ_0: u32 = 0;
            #[doc = "TRIG2_REQ detected."]
            pub const TRIG2_REQ_1: u32 = 0x01;
        }
    }
    #[doc = "Flag bit for DMA request"]
    pub mod TRIG3_REQ {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "TRIG3_REQ not detected."]
            pub const TRIG3_REQ_0: u32 = 0;
            #[doc = "TRIG3_REQ detected."]
            pub const TRIG3_REQ_1: u32 = 0x01;
        }
    }
    #[doc = "Flag bit for DMA request"]
    pub mod TRIG4_REQ {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "TRIG4_REQ not detected."]
            pub const TRIG4_REQ_0: u32 = 0;
            #[doc = "TRIG4_REQ detected."]
            pub const TRIG4_REQ_1: u32 = 0x01;
        }
    }
    #[doc = "Flag bit for DMA request"]
    pub mod TRIG5_REQ {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "TRIG5_REQ not detected."]
            pub const TRIG5_REQ_0: u32 = 0;
            #[doc = "TRIG5_REQ detected."]
            pub const TRIG5_REQ_1: u32 = 0x01;
        }
    }
    #[doc = "Flag bit for DMA request"]
    pub mod TRIG6_REQ {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "TRIG6_REQ not detected."]
            pub const TRIG6_REQ_0: u32 = 0;
            #[doc = "TRIG6_REQ detected."]
            pub const TRIG6_REQ_1: u32 = 0x01;
        }
    }
    #[doc = "Flag bit for DMA request"]
    pub mod TRIG7_REQ {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "TRIG7_REQ not detected."]
            pub const TRIG7_REQ_0: u32 = 0;
            #[doc = "TRIG7_REQ detected."]
            pub const TRIG7_REQ_1: u32 = 0x01;
        }
    }
}
#[doc = "ETC_TRIG Control Register"]
pub mod TRIG0_CTRL {
    #[doc = "Software trigger. This field is self-clearing."]
    pub mod SW_TRIG {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No software trigger event generated."]
            pub const SW_TRIG_0: u32 = 0;
            #[doc = "Software trigger event generated."]
            pub const SW_TRIG_1: u32 = 0x01;
        }
    }
    #[doc = "Trigger mode selection."]
    pub mod TRIG_MODE {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Hardware trigger. The softerware trigger will be ignored."]
            pub const TRIG_MODE_0: u32 = 0;
            #[doc = "Software trigger. The hardware trigger will be ignored."]
            pub const TRIG_MODE_1: u32 = 0x01;
        }
    }
    #[doc = "The number of segments inside the trigger chain of TRIGa."]
    pub mod TRIG_CHAIN {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Trigger chain length is 1"]
            pub const TRIG_CHAIN_0: u32 = 0;
            #[doc = "Trigger chain length is 2"]
            pub const TRIG_CHAIN_1: u32 = 0x01;
            #[doc = "Trigger chain length is 3"]
            pub const TRIG_CHAIN_2: u32 = 0x02;
            #[doc = "Trigger chain length is 4"]
            pub const TRIG_CHAIN_3: u32 = 0x03;
            #[doc = "Trigger chain length is 5"]
            pub const TRIG_CHAIN_4: u32 = 0x04;
            #[doc = "Trigger chain length is 6"]
            pub const TRIG_CHAIN_5: u32 = 0x05;
            #[doc = "Trigger chain length is 7"]
            pub const TRIG_CHAIN_6: u32 = 0x06;
            #[doc = "Trigger chain length is 8"]
            pub const TRIG_CHAIN_7: u32 = 0x07;
        }
    }
    #[doc = "External trigger priority, 7 is highest priority, while 0 is lowest"]
    pub mod TRIG_PRIORITY {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Trigger synchronization mode selection"]
    pub mod SYNC_MODE {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Synchronization mode disabled, TRIGa and TRIG(a+4) are triggered independently."]
            pub const SYNC_MODE_0: u32 = 0;
            #[doc = "Synchronization mode enabled, TRIGa and TRIG(a+4) are triggered by TRIGa source synchronously."]
            pub const SYNC_MODE_1: u32 = 0x01;
        }
    }
    #[doc = "Segment x done detection"]
    pub mod CHAINX_DONE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "segment x done not detected."]
            pub const CHAINX_DONE_0: u32 = 0;
            #[doc = "segment x done detected."]
            pub const CHAINX_DONE_1: u32 = 0x01;
        }
    }
}
#[doc = "ETC_TRIG Counter Register"]
pub mod TRIG0_COUNTER {
    #[doc = "TRIGGER initial delay counter"]
    pub mod INIT_DELAY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TRIGGER sampling interval counter"]
    pub mod SAMPLE_INTERVAL {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ETC_TRIG Chain 0/1 Register"]
pub mod TRIG0_CHAIN_1_0 {
    #[doc = "ADC hardware trigger command selection"]
    pub mod CSEL0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not a valid selection from the command buffer. Trigger event is ignored."]
            pub const CSEL0_0: u32 = 0;
            #[doc = "ADC CMD1 selected."]
            pub const CSEL0_1: u32 = 0x01;
            #[doc = "ADC CMD2 selected."]
            pub const CSEL0_2: u32 = 0x02;
            #[doc = "ADC CMD3 selected."]
            pub const CSEL0_3: u32 = 0x03;
            #[doc = "ADC CMD4 selected."]
            pub const CSEL0_4: u32 = 0x04;
            #[doc = "ADC CMD5 selected."]
            pub const CSEL0_5: u32 = 0x05;
            #[doc = "ADC CMD6 selected."]
            pub const CSEL0_6: u32 = 0x06;
            #[doc = "ADC CMD7 selected."]
            pub const CSEL0_7: u32 = 0x07;
            #[doc = "ADC CMD8 selected."]
            pub const CSEL0_8: u32 = 0x08;
            #[doc = "ADC CMD9 selected."]
            pub const CSEL0_9: u32 = 0x09;
            #[doc = "ADC CMD10 selected."]
            pub const CSEL0_10: u32 = 0x0a;
            #[doc = "ADC CMD11 selected."]
            pub const CSEL0_11: u32 = 0x0b;
            #[doc = "ADC CMD12 selected."]
            pub const CSEL0_12: u32 = 0x0c;
            #[doc = "ADC CMD13 selected."]
            pub const CSEL0_13: u32 = 0x0d;
            #[doc = "ADC CMD14 selected."]
            pub const CSEL0_14: u32 = 0x0e;
            #[doc = "ADC CMD15 selected."]
            pub const CSEL0_15: u32 = 0x0f;
        }
    }
    #[doc = "Segment 0 HWTS ADC hardware trigger selection"]
    pub mod HWTS0 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "no trigger selected"]
            pub const HWTS0_0: u32 = 0;
            #[doc = "ADC TRIG0 selected"]
            pub const HWTS0_1: u32 = 0x01;
            #[doc = "ADC TRIG1 selected"]
            pub const HWTS0_2: u32 = 0x02;
            #[doc = "ADC TRIG2 selected"]
            pub const HWTS0_4: u32 = 0x04;
            #[doc = "ADC TRIG3 selected"]
            pub const HWTS0_8: u32 = 0x08;
            #[doc = "ADC TRIG4 selected"]
            pub const HWTS0_16: u32 = 0x10;
            #[doc = "ADC TRIG5 selected"]
            pub const HWTS0_32: u32 = 0x20;
            #[doc = "ADC TRIG6 selected"]
            pub const HWTS0_64: u32 = 0x40;
            #[doc = "ADC TRIG7 selected"]
            pub const HWTS0_128: u32 = 0x80;
        }
    }
    #[doc = "Segment 0 B2B"]
    pub mod B2B0 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable B2B. Wait until delay value defined by TRIG0_COUNTER\\[SAMPLE_INTERVAL\\] is reached"]
            pub const B2B0_0: u32 = 0;
            #[doc = "Enable B2B. When Segment 0 finished (ADC COCO) then automatically trigger next ADC conversion, no need to wait until interval delay reached."]
            pub const B2B0_1: u32 = 0x01;
        }
    }
    #[doc = "Segment 0 interrupt enable. (This bit field is meaningful only when IE0_EN is set)"]
    pub mod IE0 {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Generate interrupt on Done0 when segment 0 finish."]
            pub const IE0_0: u32 = 0;
            #[doc = "Generate interrupt on Done1 when segment 0 finish."]
            pub const IE0_1: u32 = 0x01;
            #[doc = "Generate interrupt on Done2 when segment 0 finish."]
            pub const IE0_2: u32 = 0x02;
            #[doc = "Generate interrupt on Done3 when segment 0 finish."]
            pub const IE0_3: u32 = 0x03;
        }
    }
    #[doc = "IRQ enable of segment 0."]
    pub mod IE0_EN {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt DONE disabled."]
            pub const IE0_EN_0: u32 = 0;
            #[doc = "Interrupt DONE enabled. When segment 0 finish, an interrupt will be generated on the specific port configured by the IE0."]
            pub const IE0_EN_1: u32 = 0x01;
        }
    }
    #[doc = "ADC hardware trigger command selection"]
    pub mod CSEL1 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not a valid selection from the command buffer. Trigger event is ignored."]
            pub const CSEL1_0: u32 = 0;
            #[doc = "ADC CMD1 selected."]
            pub const CSEL1_1: u32 = 0x01;
            #[doc = "ADC CMD2 selected."]
            pub const CSEL1_2: u32 = 0x02;
            #[doc = "ADC CMD3 selected."]
            pub const CSEL1_3: u32 = 0x03;
            #[doc = "ADC CMD4 selected."]
            pub const CSEL1_4: u32 = 0x04;
            #[doc = "ADC CMD5 selected."]
            pub const CSEL1_5: u32 = 0x05;
            #[doc = "ADC CMD6 selected."]
            pub const CSEL1_6: u32 = 0x06;
            #[doc = "ADC CMD7 selected."]
            pub const CSEL1_7: u32 = 0x07;
            #[doc = "ADC CMD8 selected."]
            pub const CSEL1_8: u32 = 0x08;
            #[doc = "ADC CMD9 selected."]
            pub const CSEL1_9: u32 = 0x09;
            #[doc = "ADC CMD10 selected."]
            pub const CSEL1_10: u32 = 0x0a;
            #[doc = "ADC CMD11 selected."]
            pub const CSEL1_11: u32 = 0x0b;
            #[doc = "ADC CMD12 selected."]
            pub const CSEL1_12: u32 = 0x0c;
            #[doc = "ADC CMD13 selected."]
            pub const CSEL1_13: u32 = 0x0d;
            #[doc = "ADC CMD14 selected."]
            pub const CSEL1_14: u32 = 0x0e;
            #[doc = "ADC CMD15 selected."]
            pub const CSEL1_15: u32 = 0x0f;
        }
    }
    #[doc = "Segment 1 HWTS ADC hardware trigger selection"]
    pub mod HWTS1 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "no trigger selected"]
            pub const HWTS1_0: u32 = 0;
            #[doc = "ADC TRIG0 selected"]
            pub const HWTS1_1: u32 = 0x01;
            #[doc = "ADC TRIG1 selected"]
            pub const HWTS1_2: u32 = 0x02;
            #[doc = "ADC TRIG2 selected"]
            pub const HWTS1_4: u32 = 0x04;
            #[doc = "ADC TRIG3 selected"]
            pub const HWTS1_8: u32 = 0x08;
            #[doc = "ADC TRIG4 selected"]
            pub const HWTS1_16: u32 = 0x10;
            #[doc = "ADC TRIG5 selected"]
            pub const HWTS1_32: u32 = 0x20;
            #[doc = "ADC TRIG6 selected"]
            pub const HWTS1_64: u32 = 0x40;
            #[doc = "ADC TRIG7 selected"]
            pub const HWTS1_128: u32 = 0x80;
        }
    }
    #[doc = "Segment 1 B2B"]
    pub mod B2B1 {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable B2B. Wait until delay value defined by TRIG1_COUNTER\\[SAMPLE_INTERVAL\\] is reached"]
            pub const B2B1_0: u32 = 0;
            #[doc = "Enable B2B. When Segment 0 finished (ADC COCO) then automatically trigger next ADC conversion, no need to wait until interval delay reached."]
            pub const B2B1_1: u32 = 0x01;
        }
    }
    #[doc = "Segment 1 interrupt enable. (This bit field is meaningful only when IE1_EN is set)"]
    pub mod IE1 {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Generate interrupt on Done0 when Segment 1 finish."]
            pub const IE1_0: u32 = 0;
            #[doc = "Generate interrupt on Done1 when Segment 1 finish."]
            pub const IE1_1: u32 = 0x01;
            #[doc = "Generate interrupt on Done2 when Segment 1 finish."]
            pub const IE1_2: u32 = 0x02;
            #[doc = "Generate interrupt on Done3 when Segment 1 finish."]
            pub const IE1_3: u32 = 0x03;
        }
    }
    #[doc = "IRQ enable of segment 1."]
    pub mod IE1_EN {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt DONE disabled."]
            pub const IE1_EN_0: u32 = 0;
            #[doc = "Interrupt DONE enabled. When segment 1 finish, an interrupt will be generated on the specific port configured by the IE1."]
            pub const IE1_EN_1: u32 = 0x01;
        }
    }
}
#[doc = "ETC_TRIG Chain 2/3 Register"]
pub mod TRIG0_CHAIN_3_2 {
    #[doc = "ADC hardware trigger command selection"]
    pub mod CSEL2 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not a valid selection from the command buffer. Trigger event is ignored."]
            pub const CSEL2_0: u32 = 0;
            #[doc = "ADC CMD1 selected."]
            pub const CSEL2_1: u32 = 0x01;
            #[doc = "ADC CMD2 selected."]
            pub const CSEL2_2: u32 = 0x02;
            #[doc = "ADC CMD3 selected."]
            pub const CSEL2_3: u32 = 0x03;
            #[doc = "ADC CMD4 selected."]
            pub const CSEL2_4: u32 = 0x04;
            #[doc = "ADC CMD5 selected."]
            pub const CSEL2_5: u32 = 0x05;
            #[doc = "ADC CMD6 selected."]
            pub const CSEL2_6: u32 = 0x06;
            #[doc = "ADC CMD7 selected."]
            pub const CSEL2_7: u32 = 0x07;
            #[doc = "ADC CMD8 selected."]
            pub const CSEL2_8: u32 = 0x08;
            #[doc = "ADC CMD9 selected."]
            pub const CSEL2_9: u32 = 0x09;
            #[doc = "ADC CMD10 selected."]
            pub const CSEL2_10: u32 = 0x0a;
            #[doc = "ADC CMD11 selected."]
            pub const CSEL2_11: u32 = 0x0b;
            #[doc = "ADC CMD12 selected."]
            pub const CSEL2_12: u32 = 0x0c;
            #[doc = "ADC CMD13 selected."]
            pub const CSEL2_13: u32 = 0x0d;
            #[doc = "ADC CMD14 selected."]
            pub const CSEL2_14: u32 = 0x0e;
            #[doc = "ADC CMD15 selected."]
            pub const CSEL2_15: u32 = 0x0f;
        }
    }
    #[doc = "Segment 2 HWTS ADC hardware trigger selection"]
    pub mod HWTS2 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "no trigger selected"]
            pub const HWTS2_0: u32 = 0;
            #[doc = "ADC TRIG0 selected"]
            pub const HWTS2_1: u32 = 0x01;
            #[doc = "ADC TRIG1 selected"]
            pub const HWTS2_2: u32 = 0x02;
            #[doc = "ADC TRIG2 selected"]
            pub const HWTS2_4: u32 = 0x04;
            #[doc = "ADC TRIG3 selected"]
            pub const HWTS2_8: u32 = 0x08;
            #[doc = "ADC TRIG4 selected"]
            pub const HWTS2_16: u32 = 0x10;
            #[doc = "ADC TRIG5 selected"]
            pub const HWTS2_32: u32 = 0x20;
            #[doc = "ADC TRIG6 selected"]
            pub const HWTS2_64: u32 = 0x40;
            #[doc = "ADC TRIG7 selected"]
            pub const HWTS2_128: u32 = 0x80;
        }
    }
    #[doc = "Segment 2 B2B"]
    pub mod B2B2 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable B2B. Wait until delay value defined by TRIG2_COUNTER\\[SAMPLE_INTERVAL\\] is reached"]
            pub const B2B2_0: u32 = 0;
            #[doc = "Enable B2B. When Segment 0 finished (ADC COCO) then automatically trigger next ADC conversion, no need to wait until interval delay reached."]
            pub const B2B2_1: u32 = 0x01;
        }
    }
    #[doc = "Segment 2 interrupt enable. (This bit field is meaningful only when IE2_EN is set)"]
    pub mod IE2 {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Generate interrupt on Done0 when segment 2 finish."]
            pub const IE2_0: u32 = 0;
            #[doc = "Generate interrupt on Done1 when segment 2 finish."]
            pub const IE2_1: u32 = 0x01;
            #[doc = "Generate interrupt on Done2 when segment 2 finish."]
            pub const IE2_2: u32 = 0x02;
            #[doc = "Generate interrupt on Done3 when segment 2 finish."]
            pub const IE2_3: u32 = 0x03;
        }
    }
    #[doc = "IRQ enable of segment 2."]
    pub mod IE2_EN {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt DONE disabled."]
            pub const IE2_EN_0: u32 = 0;
            #[doc = "Interrupt DONE enabled. When segment 2 finish, an interrupt will be generated on the specific port configured by the IE2."]
            pub const IE2_EN_1: u32 = 0x01;
        }
    }
    #[doc = "ADC hardware trigger command selection"]
    pub mod CSEL3 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not a valid selection from the command buffer. Trigger event is ignored."]
            pub const CSEL3_0: u32 = 0;
            #[doc = "ADC CMD1 selected."]
            pub const CSEL3_1: u32 = 0x01;
            #[doc = "ADC CMD2 selected."]
            pub const CSEL3_2: u32 = 0x02;
            #[doc = "ADC CMD3 selected."]
            pub const CSEL3_3: u32 = 0x03;
            #[doc = "ADC CMD4 selected."]
            pub const CSEL3_4: u32 = 0x04;
            #[doc = "ADC CMD5 selected."]
            pub const CSEL3_5: u32 = 0x05;
            #[doc = "ADC CMD6 selected."]
            pub const CSEL3_6: u32 = 0x06;
            #[doc = "ADC CMD7 selected."]
            pub const CSEL3_7: u32 = 0x07;
            #[doc = "ADC CMD8 selected."]
            pub const CSEL3_8: u32 = 0x08;
            #[doc = "ADC CMD9 selected."]
            pub const CSEL3_9: u32 = 0x09;
            #[doc = "ADC CMD10 selected."]
            pub const CSEL3_10: u32 = 0x0a;
            #[doc = "ADC CMD11 selected."]
            pub const CSEL3_11: u32 = 0x0b;
            #[doc = "ADC CMD12 selected."]
            pub const CSEL3_12: u32 = 0x0c;
            #[doc = "ADC CMD13 selected."]
            pub const CSEL3_13: u32 = 0x0d;
            #[doc = "ADC CMD14 selected."]
            pub const CSEL3_14: u32 = 0x0e;
            #[doc = "ADC CMD15 selected."]
            pub const CSEL3_15: u32 = 0x0f;
        }
    }
    #[doc = "Segment 3 HWTS ADC hardware trigger selection"]
    pub mod HWTS3 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "no trigger selected"]
            pub const HWTS3_0: u32 = 0;
            #[doc = "ADC TRIG0 selected"]
            pub const HWTS3_1: u32 = 0x01;
            #[doc = "ADC TRIG1 selected"]
            pub const HWTS3_2: u32 = 0x02;
            #[doc = "ADC TRIG2 selected"]
            pub const HWTS3_4: u32 = 0x04;
            #[doc = "ADC TRIG3 selected"]
            pub const HWTS3_8: u32 = 0x08;
            #[doc = "ADC TRIG4 selected"]
            pub const HWTS3_16: u32 = 0x10;
            #[doc = "ADC TRIG5 selected"]
            pub const HWTS3_32: u32 = 0x20;
            #[doc = "ADC TRIG6 selected"]
            pub const HWTS3_64: u32 = 0x40;
            #[doc = "ADC TRIG7 selected"]
            pub const HWTS3_128: u32 = 0x80;
        }
    }
    #[doc = "Segment 3 B2B"]
    pub mod B2B3 {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable B2B. Wait until delay value defined by TRIG3_COUNTER\\[SAMPLE_INTERVAL\\] is reached"]
            pub const B2B3_0: u32 = 0;
            #[doc = "Enable B2B. When Segment 0 finished (ADC COCO) then automatically trigger next ADC conversion, no need to wait until interval delay reached."]
            pub const B2B3_1: u32 = 0x01;
        }
    }
    #[doc = "Segment 3 interrupt enable. (This bit field is meaningful only when IE3_EN is set)"]
    pub mod IE3 {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Generate interrupt on Done0 when segment 3 finish."]
            pub const IE3_0: u32 = 0;
            #[doc = "Generate interrupt on Done1 when segment 3 finish."]
            pub const IE3_1: u32 = 0x01;
            #[doc = "Generate interrupt on Done2 when segment 3 finish."]
            pub const IE3_2: u32 = 0x02;
            #[doc = "Generate interrupt on Done3 when segment 3 finish."]
            pub const IE3_3: u32 = 0x03;
        }
    }
    #[doc = "IRQ enable of segment 3."]
    pub mod IE3_EN {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt DONE disabled."]
            pub const IE3_EN_0: u32 = 0;
            #[doc = "Interrupt DONE enabled. When segment 3 finish, an interrupt will be generated on the specific port configured by the IE3."]
            pub const IE3_EN_1: u32 = 0x01;
        }
    }
}
#[doc = "ETC_TRIG Chain 4/5 Register"]
pub mod TRIG0_CHAIN_5_4 {
    #[doc = "ADC hardware trigger command selection"]
    pub mod CSEL4 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not a valid selection from the command buffer. Trigger event is ignored."]
            pub const CSEL4_0: u32 = 0;
            #[doc = "ADC CMD1 selected."]
            pub const CSEL4_1: u32 = 0x01;
            #[doc = "ADC CMD2 selected."]
            pub const CSEL4_2: u32 = 0x02;
            #[doc = "ADC CMD3 selected."]
            pub const CSEL4_3: u32 = 0x03;
            #[doc = "ADC CMD4 selected."]
            pub const CSEL4_4: u32 = 0x04;
            #[doc = "ADC CMD5 selected."]
            pub const CSEL4_5: u32 = 0x05;
            #[doc = "ADC CMD6 selected."]
            pub const CSEL4_6: u32 = 0x06;
            #[doc = "ADC CMD7 selected."]
            pub const CSEL4_7: u32 = 0x07;
            #[doc = "ADC CMD8 selected."]
            pub const CSEL4_8: u32 = 0x08;
            #[doc = "ADC CMD9 selected."]
            pub const CSEL4_9: u32 = 0x09;
            #[doc = "ADC CMD10 selected."]
            pub const CSEL4_10: u32 = 0x0a;
            #[doc = "ADC CMD11 selected."]
            pub const CSEL4_11: u32 = 0x0b;
            #[doc = "ADC CMD12 selected."]
            pub const CSEL4_12: u32 = 0x0c;
            #[doc = "ADC CMD13 selected."]
            pub const CSEL4_13: u32 = 0x0d;
            #[doc = "ADC CMD14 selected."]
            pub const CSEL4_14: u32 = 0x0e;
            #[doc = "ADC CMD15 selected."]
            pub const CSEL4_15: u32 = 0x0f;
        }
    }
    #[doc = "Segment 4 HWTS ADC hardware trigger selection"]
    pub mod HWTS4 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "no trigger selected"]
            pub const HWTS4_0: u32 = 0;
            #[doc = "ADC TRIG0 selected"]
            pub const HWTS4_1: u32 = 0x01;
            #[doc = "ADC TRIG1 selected"]
            pub const HWTS4_2: u32 = 0x02;
            #[doc = "ADC TRIG2 selected"]
            pub const HWTS4_4: u32 = 0x04;
            #[doc = "ADC TRIG3 selected"]
            pub const HWTS4_8: u32 = 0x08;
            #[doc = "ADC TRIG4 selected"]
            pub const HWTS4_16: u32 = 0x10;
            #[doc = "ADC TRIG5 selected"]
            pub const HWTS4_32: u32 = 0x20;
            #[doc = "ADC TRIG6 selected"]
            pub const HWTS4_64: u32 = 0x40;
            #[doc = "ADC TRIG7 selected"]
            pub const HWTS4_128: u32 = 0x80;
        }
    }
    #[doc = "Segment 4 B2B"]
    pub mod B2B4 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable B2B. Wait until delay value defined by TRIG4_COUNTER\\[SAMPLE_INTERVAL\\] is reached"]
            pub const B2B4_0: u32 = 0;
            #[doc = "Enable B2B. When Segment 0 finished (ADC COCO) then automatically trigger next ADC conversion, no need to wait until interval delay reached."]
            pub const B2B4_1: u32 = 0x01;
        }
    }
    #[doc = "Segment 4 interrupt enable. (This bit field is meaningful only when IE4_EN is set)"]
    pub mod IE4 {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Generate interrupt on Done0 when segment 4 finish."]
            pub const IE4_0: u32 = 0;
            #[doc = "Generate interrupt on Done1 when segment 4 finish."]
            pub const IE4_1: u32 = 0x01;
            #[doc = "Generate interrupt on Done2 when segment 4 finish."]
            pub const IE4_2: u32 = 0x02;
            #[doc = "Generate interrupt on Done3 when segment 4 finish."]
            pub const IE4_3: u32 = 0x03;
        }
    }
    #[doc = "IRQ enable of segment 4."]
    pub mod IE4_EN {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt DONE disabled."]
            pub const IE4_EN_0: u32 = 0;
            #[doc = "Interrupt DONE enabled. When segment 4 finish, an interrupt will be generated on the specific port configured by the IE4."]
            pub const IE4_EN_1: u32 = 0x01;
        }
    }
    #[doc = "ADC hardware trigger command selection"]
    pub mod CSEL5 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not a valid selection from the command buffer. Trigger event is ignored."]
            pub const CSEL5_0: u32 = 0;
            #[doc = "ADC CMD1 selected."]
            pub const CSEL5_1: u32 = 0x01;
            #[doc = "ADC CMD2 selected."]
            pub const CSEL5_2: u32 = 0x02;
            #[doc = "ADC CMD3 selected."]
            pub const CSEL5_3: u32 = 0x03;
            #[doc = "ADC CMD4 selected."]
            pub const CSEL5_4: u32 = 0x04;
            #[doc = "ADC CMD5 selected."]
            pub const CSEL5_5: u32 = 0x05;
            #[doc = "ADC CMD6 selected."]
            pub const CSEL5_6: u32 = 0x06;
            #[doc = "ADC CMD7 selected."]
            pub const CSEL5_7: u32 = 0x07;
            #[doc = "ADC CMD8 selected."]
            pub const CSEL5_8: u32 = 0x08;
            #[doc = "ADC CMD9 selected."]
            pub const CSEL5_9: u32 = 0x09;
            #[doc = "ADC CMD10 selected."]
            pub const CSEL5_10: u32 = 0x0a;
            #[doc = "ADC CMD11 selected."]
            pub const CSEL5_11: u32 = 0x0b;
            #[doc = "ADC CMD12 selected."]
            pub const CSEL5_12: u32 = 0x0c;
            #[doc = "ADC CMD13 selected."]
            pub const CSEL5_13: u32 = 0x0d;
            #[doc = "ADC CMD14 selected."]
            pub const CSEL5_14: u32 = 0x0e;
            #[doc = "ADC CMD15 selected."]
            pub const CSEL5_15: u32 = 0x0f;
        }
    }
    #[doc = "Segment 5 HWTS ADC hardware trigger selection"]
    pub mod HWTS5 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "no trigger selected"]
            pub const HWTS5_0: u32 = 0;
            #[doc = "ADC TRIG0 selected"]
            pub const HWTS5_1: u32 = 0x01;
            #[doc = "ADC TRIG1 selected"]
            pub const HWTS5_2: u32 = 0x02;
            #[doc = "ADC TRIG2 selected"]
            pub const HWTS5_4: u32 = 0x04;
            #[doc = "ADC TRIG3 selected"]
            pub const HWTS5_8: u32 = 0x08;
            #[doc = "ADC TRIG4 selected"]
            pub const HWTS5_16: u32 = 0x10;
            #[doc = "ADC TRIG5 selected"]
            pub const HWTS5_32: u32 = 0x20;
            #[doc = "ADC TRIG6 selected"]
            pub const HWTS5_64: u32 = 0x40;
            #[doc = "ADC TRIG7 selected"]
            pub const HWTS5_128: u32 = 0x80;
        }
    }
    #[doc = "Segment 5 B2B"]
    pub mod B2B5 {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable B2B. Wait until delay value defined by TRIG5_COUNTER\\[SAMPLE_INTERVAL\\] is reached"]
            pub const B2B5_0: u32 = 0;
            #[doc = "Enable B2B. When Segment 0 finished (ADC COCO) then automatically trigger next ADC conversion, no need to wait until interval delay reached."]
            pub const B2B5_1: u32 = 0x01;
        }
    }
    #[doc = "Segment 5 interrupt enable. (This bit field is meaningful only when IE5_EN is set)"]
    pub mod IE5 {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Generate interrupt on Done0 when segment 5 finish."]
            pub const IE5_0: u32 = 0;
            #[doc = "Generate interrupt on Done1 when segment 5 finish."]
            pub const IE5_1: u32 = 0x01;
            #[doc = "Generate interrupt on Done2 when segment 5 finish."]
            pub const IE5_2: u32 = 0x02;
            #[doc = "Generate interrupt on Done3 when segment 5 finish."]
            pub const IE5_3: u32 = 0x03;
        }
    }
    #[doc = "IRQ enable of segment 5."]
    pub mod IE5_EN {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt DONE disabled."]
            pub const IE5_EN_0: u32 = 0;
            #[doc = "Interrupt DONE enabled. When segment 5 finish, an interrupt will be generated on the specific port configured by the IE5."]
            pub const IE5_EN_1: u32 = 0x01;
        }
    }
}
#[doc = "ETC_TRIG Chain 6/7 Register"]
pub mod TRIG0_CHAIN_7_6 {
    #[doc = "ADC hardware trigger command selection"]
    pub mod CSEL6 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not a valid selection from the command buffer. Trigger event is ignored."]
            pub const CSEL6_0: u32 = 0;
            #[doc = "ADC CMD1 selected."]
            pub const CSEL6_1: u32 = 0x01;
            #[doc = "ADC CMD2 selected."]
            pub const CSEL6_2: u32 = 0x02;
            #[doc = "ADC CMD3 selected."]
            pub const CSEL6_3: u32 = 0x03;
            #[doc = "ADC CMD4 selected."]
            pub const CSEL6_4: u32 = 0x04;
            #[doc = "ADC CMD5 selected."]
            pub const CSEL6_5: u32 = 0x05;
            #[doc = "ADC CMD6 selected."]
            pub const CSEL6_6: u32 = 0x06;
            #[doc = "ADC CMD7 selected."]
            pub const CSEL6_7: u32 = 0x07;
            #[doc = "ADC CMD8 selected."]
            pub const CSEL6_8: u32 = 0x08;
            #[doc = "ADC CMD9 selected."]
            pub const CSEL6_9: u32 = 0x09;
            #[doc = "ADC CMD10 selected."]
            pub const CSEL6_10: u32 = 0x0a;
            #[doc = "ADC CMD11 selected."]
            pub const CSEL6_11: u32 = 0x0b;
            #[doc = "ADC CMD12 selected."]
            pub const CSEL6_12: u32 = 0x0c;
            #[doc = "ADC CMD13 selected."]
            pub const CSEL6_13: u32 = 0x0d;
            #[doc = "ADC CMD14 selected."]
            pub const CSEL6_14: u32 = 0x0e;
            #[doc = "ADC CMD15 selected."]
            pub const CSEL6_15: u32 = 0x0f;
        }
    }
    #[doc = "Segment 6 HWTS ADC hardware trigger selection"]
    pub mod HWTS6 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "no trigger selected"]
            pub const HWTS6_0: u32 = 0;
            #[doc = "ADC TRIG0 selected"]
            pub const HWTS6_1: u32 = 0x01;
            #[doc = "ADC TRIG1 selected"]
            pub const HWTS6_2: u32 = 0x02;
            #[doc = "ADC TRIG2 selected"]
            pub const HWTS6_4: u32 = 0x04;
            #[doc = "ADC TRIG3 selected"]
            pub const HWTS6_8: u32 = 0x08;
            #[doc = "ADC TRIG4 selected"]
            pub const HWTS6_16: u32 = 0x10;
            #[doc = "ADC TRIG5 selected"]
            pub const HWTS6_32: u32 = 0x20;
            #[doc = "ADC TRIG6 selected"]
            pub const HWTS6_64: u32 = 0x40;
            #[doc = "ADC TRIG7 selected"]
            pub const HWTS6_128: u32 = 0x80;
        }
    }
    #[doc = "Segment 6 B2B"]
    pub mod B2B6 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable B2B. Wait until delay value defined by TRIG6_COUNTER\\[SAMPLE_INTERVAL\\] is reached"]
            pub const B2B6_0: u32 = 0;
            #[doc = "Enable B2B. When Segment 0 finished (ADC COCO) then automatically trigger next ADC conversion, no need to wait until interval delay reached."]
            pub const B2B6_1: u32 = 0x01;
        }
    }
    #[doc = "Segment 6 interrupt enable. (This bit field is meaningful only when IE6_EN is set)"]
    pub mod IE6 {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Generate interrupt on Done0 when segment 6 finish."]
            pub const IE6_0: u32 = 0;
            #[doc = "Generate interrupt on Done1 when segment 6 finish."]
            pub const IE6_1: u32 = 0x01;
            #[doc = "Generate interrupt on Done2 when segment 6 finish."]
            pub const IE6_2: u32 = 0x02;
            #[doc = "Generate interrupt on Done3 when segment 6 finish."]
            pub const IE6_3: u32 = 0x03;
        }
    }
    #[doc = "IRQ enable of segment 6."]
    pub mod IE6_EN {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt DONE disabled."]
            pub const IE6_EN_0: u32 = 0;
            #[doc = "Interrupt DONE enabled. When segment 6 finish, an interrupt will be generated on the specific port configured by the IE6."]
            pub const IE6_EN_1: u32 = 0x01;
        }
    }
    #[doc = "ADC hardware trigger command selection"]
    pub mod CSEL7 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not a valid selection from the command buffer. Trigger event is ignored."]
            pub const CSEL7_0: u32 = 0;
            #[doc = "ADC CMD1 selected."]
            pub const CSEL7_1: u32 = 0x01;
            #[doc = "ADC CMD2 selected."]
            pub const CSEL7_2: u32 = 0x02;
            #[doc = "ADC CMD3 selected."]
            pub const CSEL7_3: u32 = 0x03;
            #[doc = "ADC CMD4 selected."]
            pub const CSEL7_4: u32 = 0x04;
            #[doc = "ADC CMD5 selected."]
            pub const CSEL7_5: u32 = 0x05;
            #[doc = "ADC CMD6 selected."]
            pub const CSEL7_6: u32 = 0x06;
            #[doc = "ADC CMD7 selected."]
            pub const CSEL7_7: u32 = 0x07;
            #[doc = "ADC CMD8 selected."]
            pub const CSEL7_8: u32 = 0x08;
            #[doc = "ADC CMD9 selected."]
            pub const CSEL7_9: u32 = 0x09;
            #[doc = "ADC CMD10 selected."]
            pub const CSEL7_10: u32 = 0x0a;
            #[doc = "ADC CMD11 selected."]
            pub const CSEL7_11: u32 = 0x0b;
            #[doc = "ADC CMD12 selected."]
            pub const CSEL7_12: u32 = 0x0c;
            #[doc = "ADC CMD13 selected."]
            pub const CSEL7_13: u32 = 0x0d;
            #[doc = "ADC CMD14 selected."]
            pub const CSEL7_14: u32 = 0x0e;
            #[doc = "ADC CMD15 selected."]
            pub const CSEL7_15: u32 = 0x0f;
        }
    }
    #[doc = "Segment 7 HWTS ADC hardware trigger selection"]
    pub mod HWTS7 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "no trigger selected"]
            pub const HWTS7_0: u32 = 0;
            #[doc = "ADC TRIG0 selected"]
            pub const HWTS7_1: u32 = 0x01;
            #[doc = "ADC TRIG1 selected"]
            pub const HWTS7_2: u32 = 0x02;
            #[doc = "ADC TRIG2 selected"]
            pub const HWTS7_4: u32 = 0x04;
            #[doc = "ADC TRIG3 selected"]
            pub const HWTS7_8: u32 = 0x08;
            #[doc = "ADC TRIG4 selected"]
            pub const HWTS7_16: u32 = 0x10;
            #[doc = "ADC TRIG5 selected"]
            pub const HWTS7_32: u32 = 0x20;
            #[doc = "ADC TRIG6 selected"]
            pub const HWTS7_64: u32 = 0x40;
            #[doc = "ADC TRIG7 selected"]
            pub const HWTS7_128: u32 = 0x80;
        }
    }
    #[doc = "Segment 7 B2B"]
    pub mod B2B7 {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable B2B. Wait until delay value defined by TRIG7_COUNTER\\[SAMPLE_INTERVAL\\] is reached"]
            pub const B2B7_0: u32 = 0;
            #[doc = "Enable B2B. When Segment 0 finished (ADC COCO) then automatically trigger next ADC conversion, no need to wait until interval delay reached."]
            pub const B2B7_1: u32 = 0x01;
        }
    }
    #[doc = "Segment 7 interrupt enable. (This bit field is meaningful only when IE7_EN is set)"]
    pub mod IE7 {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Generate interrupt on Done0 when segment 7 finish."]
            pub const IE7_0: u32 = 0;
            #[doc = "Generate interrupt on Done1 when segment 7 finish."]
            pub const IE7_1: u32 = 0x01;
            #[doc = "Generate interrupt on Done2 when segment 7 finish."]
            pub const IE7_2: u32 = 0x02;
            #[doc = "Generate interrupt on Done3 when segment 7 finish."]
            pub const IE7_3: u32 = 0x03;
        }
    }
    #[doc = "IRQ enable of segment 7."]
    pub mod IE7_EN {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt DONE disabled."]
            pub const IE7_EN_0: u32 = 0;
            #[doc = "Interrupt DONE enabled. When segment 7 finish, an interrupt will be generated on the specific port configured by the IE7."]
            pub const IE7_EN_1: u32 = 0x01;
        }
    }
}
#[doc = "ETC_TRIG Result Data 1/0 Register"]
pub mod TRIG0_RESULT_1_0 {
    #[doc = "Result DATA0"]
    pub mod DATA0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Result DATA1"]
    pub mod DATA1 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ETC_TRIG Result Data 3/2 Register"]
pub mod TRIG0_RESULT_3_2 {
    #[doc = "Result DATA2"]
    pub mod DATA2 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Result DATA3"]
    pub mod DATA3 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ETC_TRIG Result Data 5/4 Register"]
pub mod TRIG0_RESULT_5_4 {
    #[doc = "Result DATA4"]
    pub mod DATA4 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Result DATA5"]
    pub mod DATA5 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ETC_TRIG Result Data 7/6 Register"]
pub mod TRIG0_RESULT_7_6 {
    #[doc = "Result DATA6"]
    pub mod DATA6 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Result DATA7"]
    pub mod DATA7 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ETC_TRIG Control Register"]
pub mod TRIG1_CTRL {
    #[doc = "Software trigger. This field is self-clearing."]
    pub mod SW_TRIG {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No software trigger event generated."]
            pub const SW_TRIG_0: u32 = 0;
            #[doc = "Software trigger event generated."]
            pub const SW_TRIG_1: u32 = 0x01;
        }
    }
    #[doc = "Trigger mode selection."]
    pub mod TRIG_MODE {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Hardware trigger. The softerware trigger will be ignored."]
            pub const TRIG_MODE_0: u32 = 0;
            #[doc = "Software trigger. The hardware trigger will be ignored."]
            pub const TRIG_MODE_1: u32 = 0x01;
        }
    }
    #[doc = "The number of segments inside the trigger chain of TRIGa."]
    pub mod TRIG_CHAIN {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Trigger chain length is 1"]
            pub const TRIG_CHAIN_0: u32 = 0;
            #[doc = "Trigger chain length is 2"]
            pub const TRIG_CHAIN_1: u32 = 0x01;
            #[doc = "Trigger chain length is 3"]
            pub const TRIG_CHAIN_2: u32 = 0x02;
            #[doc = "Trigger chain length is 4"]
            pub const TRIG_CHAIN_3: u32 = 0x03;
            #[doc = "Trigger chain length is 5"]
            pub const TRIG_CHAIN_4: u32 = 0x04;
            #[doc = "Trigger chain length is 6"]
            pub const TRIG_CHAIN_5: u32 = 0x05;
            #[doc = "Trigger chain length is 7"]
            pub const TRIG_CHAIN_6: u32 = 0x06;
            #[doc = "Trigger chain length is 8"]
            pub const TRIG_CHAIN_7: u32 = 0x07;
        }
    }
    #[doc = "External trigger priority, 7 is highest priority, while 0 is lowest"]
    pub mod TRIG_PRIORITY {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Trigger synchronization mode selection"]
    pub mod SYNC_MODE {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Synchronization mode disabled, TRIGa and TRIG(a+4) are triggered independently."]
            pub const SYNC_MODE_0: u32 = 0;
            #[doc = "Synchronization mode enabled, TRIGa and TRIG(a+4) are triggered by TRIGa source synchronously."]
            pub const SYNC_MODE_1: u32 = 0x01;
        }
    }
    #[doc = "Segment x done detection"]
    pub mod CHAINX_DONE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "segment x done not detected."]
            pub const CHAINX_DONE_0: u32 = 0;
            #[doc = "segment x done detected."]
            pub const CHAINX_DONE_1: u32 = 0x01;
        }
    }
}
#[doc = "ETC_TRIG Counter Register"]
pub mod TRIG1_COUNTER {
    #[doc = "TRIGGER initial delay counter"]
    pub mod INIT_DELAY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TRIGGER sampling interval counter"]
    pub mod SAMPLE_INTERVAL {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ETC_TRIG Chain 0/1 Register"]
pub mod TRIG1_CHAIN_1_0 {
    #[doc = "ADC hardware trigger command selection"]
    pub mod CSEL0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not a valid selection from the command buffer. Trigger event is ignored."]
            pub const CSEL0_0: u32 = 0;
            #[doc = "ADC CMD1 selected."]
            pub const CSEL0_1: u32 = 0x01;
            #[doc = "ADC CMD2 selected."]
            pub const CSEL0_2: u32 = 0x02;
            #[doc = "ADC CMD3 selected."]
            pub const CSEL0_3: u32 = 0x03;
            #[doc = "ADC CMD4 selected."]
            pub const CSEL0_4: u32 = 0x04;
            #[doc = "ADC CMD5 selected."]
            pub const CSEL0_5: u32 = 0x05;
            #[doc = "ADC CMD6 selected."]
            pub const CSEL0_6: u32 = 0x06;
            #[doc = "ADC CMD7 selected."]
            pub const CSEL0_7: u32 = 0x07;
            #[doc = "ADC CMD8 selected."]
            pub const CSEL0_8: u32 = 0x08;
            #[doc = "ADC CMD9 selected."]
            pub const CSEL0_9: u32 = 0x09;
            #[doc = "ADC CMD10 selected."]
            pub const CSEL0_10: u32 = 0x0a;
            #[doc = "ADC CMD11 selected."]
            pub const CSEL0_11: u32 = 0x0b;
            #[doc = "ADC CMD12 selected."]
            pub const CSEL0_12: u32 = 0x0c;
            #[doc = "ADC CMD13 selected."]
            pub const CSEL0_13: u32 = 0x0d;
            #[doc = "ADC CMD14 selected."]
            pub const CSEL0_14: u32 = 0x0e;
            #[doc = "ADC CMD15 selected."]
            pub const CSEL0_15: u32 = 0x0f;
        }
    }
    #[doc = "Segment 0 HWTS ADC hardware trigger selection"]
    pub mod HWTS0 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "no trigger selected"]
            pub const HWTS0_0: u32 = 0;
            #[doc = "ADC TRIG0 selected"]
            pub const HWTS0_1: u32 = 0x01;
            #[doc = "ADC TRIG1 selected"]
            pub const HWTS0_2: u32 = 0x02;
            #[doc = "ADC TRIG2 selected"]
            pub const HWTS0_4: u32 = 0x04;
            #[doc = "ADC TRIG3 selected"]
            pub const HWTS0_8: u32 = 0x08;
            #[doc = "ADC TRIG4 selected"]
            pub const HWTS0_16: u32 = 0x10;
            #[doc = "ADC TRIG5 selected"]
            pub const HWTS0_32: u32 = 0x20;
            #[doc = "ADC TRIG6 selected"]
            pub const HWTS0_64: u32 = 0x40;
            #[doc = "ADC TRIG7 selected"]
            pub const HWTS0_128: u32 = 0x80;
        }
    }
    #[doc = "Segment 0 B2B"]
    pub mod B2B0 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable B2B. Wait until delay value defined by TRIG0_COUNTER\\[SAMPLE_INTERVAL\\] is reached"]
            pub const B2B0_0: u32 = 0;
            #[doc = "Enable B2B. When Segment 0 finished (ADC COCO) then automatically trigger next ADC conversion, no need to wait until interval delay reached."]
            pub const B2B0_1: u32 = 0x01;
        }
    }
    #[doc = "Segment 0 interrupt enable. (This bit field is meaningful only when IE0_EN is set)"]
    pub mod IE0 {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Generate interrupt on Done0 when segment 0 finish."]
            pub const IE0_0: u32 = 0;
            #[doc = "Generate interrupt on Done1 when segment 0 finish."]
            pub const IE0_1: u32 = 0x01;
            #[doc = "Generate interrupt on Done2 when segment 0 finish."]
            pub const IE0_2: u32 = 0x02;
            #[doc = "Generate interrupt on Done3 when segment 0 finish."]
            pub const IE0_3: u32 = 0x03;
        }
    }
    #[doc = "IRQ enable of segment 0."]
    pub mod IE0_EN {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt DONE disabled."]
            pub const IE0_EN_0: u32 = 0;
            #[doc = "Interrupt DONE enabled. When segment 0 finish, an interrupt will be generated on the specific port configured by the IE0."]
            pub const IE0_EN_1: u32 = 0x01;
        }
    }
    #[doc = "ADC hardware trigger command selection"]
    pub mod CSEL1 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not a valid selection from the command buffer. Trigger event is ignored."]
            pub const CSEL1_0: u32 = 0;
            #[doc = "ADC CMD1 selected."]
            pub const CSEL1_1: u32 = 0x01;
            #[doc = "ADC CMD2 selected."]
            pub const CSEL1_2: u32 = 0x02;
            #[doc = "ADC CMD3 selected."]
            pub const CSEL1_3: u32 = 0x03;
            #[doc = "ADC CMD4 selected."]
            pub const CSEL1_4: u32 = 0x04;
            #[doc = "ADC CMD5 selected."]
            pub const CSEL1_5: u32 = 0x05;
            #[doc = "ADC CMD6 selected."]
            pub const CSEL1_6: u32 = 0x06;
            #[doc = "ADC CMD7 selected."]
            pub const CSEL1_7: u32 = 0x07;
            #[doc = "ADC CMD8 selected."]
            pub const CSEL1_8: u32 = 0x08;
            #[doc = "ADC CMD9 selected."]
            pub const CSEL1_9: u32 = 0x09;
            #[doc = "ADC CMD10 selected."]
            pub const CSEL1_10: u32 = 0x0a;
            #[doc = "ADC CMD11 selected."]
            pub const CSEL1_11: u32 = 0x0b;
            #[doc = "ADC CMD12 selected."]
            pub const CSEL1_12: u32 = 0x0c;
            #[doc = "ADC CMD13 selected."]
            pub const CSEL1_13: u32 = 0x0d;
            #[doc = "ADC CMD14 selected."]
            pub const CSEL1_14: u32 = 0x0e;
            #[doc = "ADC CMD15 selected."]
            pub const CSEL1_15: u32 = 0x0f;
        }
    }
    #[doc = "Segment 1 HWTS ADC hardware trigger selection"]
    pub mod HWTS1 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "no trigger selected"]
            pub const HWTS1_0: u32 = 0;
            #[doc = "ADC TRIG0 selected"]
            pub const HWTS1_1: u32 = 0x01;
            #[doc = "ADC TRIG1 selected"]
            pub const HWTS1_2: u32 = 0x02;
            #[doc = "ADC TRIG2 selected"]
            pub const HWTS1_4: u32 = 0x04;
            #[doc = "ADC TRIG3 selected"]
            pub const HWTS1_8: u32 = 0x08;
            #[doc = "ADC TRIG4 selected"]
            pub const HWTS1_16: u32 = 0x10;
            #[doc = "ADC TRIG5 selected"]
            pub const HWTS1_32: u32 = 0x20;
            #[doc = "ADC TRIG6 selected"]
            pub const HWTS1_64: u32 = 0x40;
            #[doc = "ADC TRIG7 selected"]
            pub const HWTS1_128: u32 = 0x80;
        }
    }
    #[doc = "Segment 1 B2B"]
    pub mod B2B1 {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable B2B. Wait until delay value defined by TRIG1_COUNTER\\[SAMPLE_INTERVAL\\] is reached"]
            pub const B2B1_0: u32 = 0;
            #[doc = "Enable B2B. When Segment 0 finished (ADC COCO) then automatically trigger next ADC conversion, no need to wait until interval delay reached."]
            pub const B2B1_1: u32 = 0x01;
        }
    }
    #[doc = "Segment 1 interrupt enable. (This bit field is meaningful only when IE1_EN is set)"]
    pub mod IE1 {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Generate interrupt on Done0 when Segment 1 finish."]
            pub const IE1_0: u32 = 0;
            #[doc = "Generate interrupt on Done1 when Segment 1 finish."]
            pub const IE1_1: u32 = 0x01;
            #[doc = "Generate interrupt on Done2 when Segment 1 finish."]
            pub const IE1_2: u32 = 0x02;
            #[doc = "Generate interrupt on Done3 when Segment 1 finish."]
            pub const IE1_3: u32 = 0x03;
        }
    }
    #[doc = "IRQ enable of segment 1."]
    pub mod IE1_EN {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt DONE disabled."]
            pub const IE1_EN_0: u32 = 0;
            #[doc = "Interrupt DONE enabled. When segment 1 finish, an interrupt will be generated on the specific port configured by the IE1."]
            pub const IE1_EN_1: u32 = 0x01;
        }
    }
}
#[doc = "ETC_TRIG Chain 2/3 Register"]
pub mod TRIG1_CHAIN_3_2 {
    #[doc = "ADC hardware trigger command selection"]
    pub mod CSEL2 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not a valid selection from the command buffer. Trigger event is ignored."]
            pub const CSEL2_0: u32 = 0;
            #[doc = "ADC CMD1 selected."]
            pub const CSEL2_1: u32 = 0x01;
            #[doc = "ADC CMD2 selected."]
            pub const CSEL2_2: u32 = 0x02;
            #[doc = "ADC CMD3 selected."]
            pub const CSEL2_3: u32 = 0x03;
            #[doc = "ADC CMD4 selected."]
            pub const CSEL2_4: u32 = 0x04;
            #[doc = "ADC CMD5 selected."]
            pub const CSEL2_5: u32 = 0x05;
            #[doc = "ADC CMD6 selected."]
            pub const CSEL2_6: u32 = 0x06;
            #[doc = "ADC CMD7 selected."]
            pub const CSEL2_7: u32 = 0x07;
            #[doc = "ADC CMD8 selected."]
            pub const CSEL2_8: u32 = 0x08;
            #[doc = "ADC CMD9 selected."]
            pub const CSEL2_9: u32 = 0x09;
            #[doc = "ADC CMD10 selected."]
            pub const CSEL2_10: u32 = 0x0a;
            #[doc = "ADC CMD11 selected."]
            pub const CSEL2_11: u32 = 0x0b;
            #[doc = "ADC CMD12 selected."]
            pub const CSEL2_12: u32 = 0x0c;
            #[doc = "ADC CMD13 selected."]
            pub const CSEL2_13: u32 = 0x0d;
            #[doc = "ADC CMD14 selected."]
            pub const CSEL2_14: u32 = 0x0e;
            #[doc = "ADC CMD15 selected."]
            pub const CSEL2_15: u32 = 0x0f;
        }
    }
    #[doc = "Segment 2 HWTS ADC hardware trigger selection"]
    pub mod HWTS2 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "no trigger selected"]
            pub const HWTS2_0: u32 = 0;
            #[doc = "ADC TRIG0 selected"]
            pub const HWTS2_1: u32 = 0x01;
            #[doc = "ADC TRIG1 selected"]
            pub const HWTS2_2: u32 = 0x02;
            #[doc = "ADC TRIG2 selected"]
            pub const HWTS2_4: u32 = 0x04;
            #[doc = "ADC TRIG3 selected"]
            pub const HWTS2_8: u32 = 0x08;
            #[doc = "ADC TRIG4 selected"]
            pub const HWTS2_16: u32 = 0x10;
            #[doc = "ADC TRIG5 selected"]
            pub const HWTS2_32: u32 = 0x20;
            #[doc = "ADC TRIG6 selected"]
            pub const HWTS2_64: u32 = 0x40;
            #[doc = "ADC TRIG7 selected"]
            pub const HWTS2_128: u32 = 0x80;
        }
    }
    #[doc = "Segment 2 B2B"]
    pub mod B2B2 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable B2B. Wait until delay value defined by TRIG2_COUNTER\\[SAMPLE_INTERVAL\\] is reached"]
            pub const B2B2_0: u32 = 0;
            #[doc = "Enable B2B. When Segment 0 finished (ADC COCO) then automatically trigger next ADC conversion, no need to wait until interval delay reached."]
            pub const B2B2_1: u32 = 0x01;
        }
    }
    #[doc = "Segment 2 interrupt enable. (This bit field is meaningful only when IE2_EN is set)"]
    pub mod IE2 {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Generate interrupt on Done0 when segment 2 finish."]
            pub const IE2_0: u32 = 0;
            #[doc = "Generate interrupt on Done1 when segment 2 finish."]
            pub const IE2_1: u32 = 0x01;
            #[doc = "Generate interrupt on Done2 when segment 2 finish."]
            pub const IE2_2: u32 = 0x02;
            #[doc = "Generate interrupt on Done3 when segment 2 finish."]
            pub const IE2_3: u32 = 0x03;
        }
    }
    #[doc = "IRQ enable of segment 2."]
    pub mod IE2_EN {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt DONE disabled."]
            pub const IE2_EN_0: u32 = 0;
            #[doc = "Interrupt DONE enabled. When segment 2 finish, an interrupt will be generated on the specific port configured by the IE2."]
            pub const IE2_EN_1: u32 = 0x01;
        }
    }
    #[doc = "ADC hardware trigger command selection"]
    pub mod CSEL3 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not a valid selection from the command buffer. Trigger event is ignored."]
            pub const CSEL3_0: u32 = 0;
            #[doc = "ADC CMD1 selected."]
            pub const CSEL3_1: u32 = 0x01;
            #[doc = "ADC CMD2 selected."]
            pub const CSEL3_2: u32 = 0x02;
            #[doc = "ADC CMD3 selected."]
            pub const CSEL3_3: u32 = 0x03;
            #[doc = "ADC CMD4 selected."]
            pub const CSEL3_4: u32 = 0x04;
            #[doc = "ADC CMD5 selected."]
            pub const CSEL3_5: u32 = 0x05;
            #[doc = "ADC CMD6 selected."]
            pub const CSEL3_6: u32 = 0x06;
            #[doc = "ADC CMD7 selected."]
            pub const CSEL3_7: u32 = 0x07;
            #[doc = "ADC CMD8 selected."]
            pub const CSEL3_8: u32 = 0x08;
            #[doc = "ADC CMD9 selected."]
            pub const CSEL3_9: u32 = 0x09;
            #[doc = "ADC CMD10 selected."]
            pub const CSEL3_10: u32 = 0x0a;
            #[doc = "ADC CMD11 selected."]
            pub const CSEL3_11: u32 = 0x0b;
            #[doc = "ADC CMD12 selected."]
            pub const CSEL3_12: u32 = 0x0c;
            #[doc = "ADC CMD13 selected."]
            pub const CSEL3_13: u32 = 0x0d;
            #[doc = "ADC CMD14 selected."]
            pub const CSEL3_14: u32 = 0x0e;
            #[doc = "ADC CMD15 selected."]
            pub const CSEL3_15: u32 = 0x0f;
        }
    }
    #[doc = "Segment 3 HWTS ADC hardware trigger selection"]
    pub mod HWTS3 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "no trigger selected"]
            pub const HWTS3_0: u32 = 0;
            #[doc = "ADC TRIG0 selected"]
            pub const HWTS3_1: u32 = 0x01;
            #[doc = "ADC TRIG1 selected"]
            pub const HWTS3_2: u32 = 0x02;
            #[doc = "ADC TRIG2 selected"]
            pub const HWTS3_4: u32 = 0x04;
            #[doc = "ADC TRIG3 selected"]
            pub const HWTS3_8: u32 = 0x08;
            #[doc = "ADC TRIG4 selected"]
            pub const HWTS3_16: u32 = 0x10;
            #[doc = "ADC TRIG5 selected"]
            pub const HWTS3_32: u32 = 0x20;
            #[doc = "ADC TRIG6 selected"]
            pub const HWTS3_64: u32 = 0x40;
            #[doc = "ADC TRIG7 selected"]
            pub const HWTS3_128: u32 = 0x80;
        }
    }
    #[doc = "Segment 3 B2B"]
    pub mod B2B3 {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable B2B. Wait until delay value defined by TRIG3_COUNTER\\[SAMPLE_INTERVAL\\] is reached"]
            pub const B2B3_0: u32 = 0;
            #[doc = "Enable B2B. When Segment 0 finished (ADC COCO) then automatically trigger next ADC conversion, no need to wait until interval delay reached."]
            pub const B2B3_1: u32 = 0x01;
        }
    }
    #[doc = "Segment 3 interrupt enable. (This bit field is meaningful only when IE3_EN is set)"]
    pub mod IE3 {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Generate interrupt on Done0 when segment 3 finish."]
            pub const IE3_0: u32 = 0;
            #[doc = "Generate interrupt on Done1 when segment 3 finish."]
            pub const IE3_1: u32 = 0x01;
            #[doc = "Generate interrupt on Done2 when segment 3 finish."]
            pub const IE3_2: u32 = 0x02;
            #[doc = "Generate interrupt on Done3 when segment 3 finish."]
            pub const IE3_3: u32 = 0x03;
        }
    }
    #[doc = "IRQ enable of segment 3."]
    pub mod IE3_EN {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt DONE disabled."]
            pub const IE3_EN_0: u32 = 0;
            #[doc = "Interrupt DONE enabled. When segment 3 finish, an interrupt will be generated on the specific port configured by the IE3."]
            pub const IE3_EN_1: u32 = 0x01;
        }
    }
}
#[doc = "ETC_TRIG Chain 4/5 Register"]
pub mod TRIG1_CHAIN_5_4 {
    #[doc = "ADC hardware trigger command selection"]
    pub mod CSEL4 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not a valid selection from the command buffer. Trigger event is ignored."]
            pub const CSEL4_0: u32 = 0;
            #[doc = "ADC CMD1 selected."]
            pub const CSEL4_1: u32 = 0x01;
            #[doc = "ADC CMD2 selected."]
            pub const CSEL4_2: u32 = 0x02;
            #[doc = "ADC CMD3 selected."]
            pub const CSEL4_3: u32 = 0x03;
            #[doc = "ADC CMD4 selected."]
            pub const CSEL4_4: u32 = 0x04;
            #[doc = "ADC CMD5 selected."]
            pub const CSEL4_5: u32 = 0x05;
            #[doc = "ADC CMD6 selected."]
            pub const CSEL4_6: u32 = 0x06;
            #[doc = "ADC CMD7 selected."]
            pub const CSEL4_7: u32 = 0x07;
            #[doc = "ADC CMD8 selected."]
            pub const CSEL4_8: u32 = 0x08;
            #[doc = "ADC CMD9 selected."]
            pub const CSEL4_9: u32 = 0x09;
            #[doc = "ADC CMD10 selected."]
            pub const CSEL4_10: u32 = 0x0a;
            #[doc = "ADC CMD11 selected."]
            pub const CSEL4_11: u32 = 0x0b;
            #[doc = "ADC CMD12 selected."]
            pub const CSEL4_12: u32 = 0x0c;
            #[doc = "ADC CMD13 selected."]
            pub const CSEL4_13: u32 = 0x0d;
            #[doc = "ADC CMD14 selected."]
            pub const CSEL4_14: u32 = 0x0e;
            #[doc = "ADC CMD15 selected."]
            pub const CSEL4_15: u32 = 0x0f;
        }
    }
    #[doc = "Segment 4 HWTS ADC hardware trigger selection"]
    pub mod HWTS4 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "no trigger selected"]
            pub const HWTS4_0: u32 = 0;
            #[doc = "ADC TRIG0 selected"]
            pub const HWTS4_1: u32 = 0x01;
            #[doc = "ADC TRIG1 selected"]
            pub const HWTS4_2: u32 = 0x02;
            #[doc = "ADC TRIG2 selected"]
            pub const HWTS4_4: u32 = 0x04;
            #[doc = "ADC TRIG3 selected"]
            pub const HWTS4_8: u32 = 0x08;
            #[doc = "ADC TRIG4 selected"]
            pub const HWTS4_16: u32 = 0x10;
            #[doc = "ADC TRIG5 selected"]
            pub const HWTS4_32: u32 = 0x20;
            #[doc = "ADC TRIG6 selected"]
            pub const HWTS4_64: u32 = 0x40;
            #[doc = "ADC TRIG7 selected"]
            pub const HWTS4_128: u32 = 0x80;
        }
    }
    #[doc = "Segment 4 B2B"]
    pub mod B2B4 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable B2B. Wait until delay value defined by TRIG4_COUNTER\\[SAMPLE_INTERVAL\\] is reached"]
            pub const B2B4_0: u32 = 0;
            #[doc = "Enable B2B. When Segment 0 finished (ADC COCO) then automatically trigger next ADC conversion, no need to wait until interval delay reached."]
            pub const B2B4_1: u32 = 0x01;
        }
    }
    #[doc = "Segment 4 interrupt enable. (This bit field is meaningful only when IE4_EN is set)"]
    pub mod IE4 {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Generate interrupt on Done0 when segment 4 finish."]
            pub const IE4_0: u32 = 0;
            #[doc = "Generate interrupt on Done1 when segment 4 finish."]
            pub const IE4_1: u32 = 0x01;
            #[doc = "Generate interrupt on Done2 when segment 4 finish."]
            pub const IE4_2: u32 = 0x02;
            #[doc = "Generate interrupt on Done3 when segment 4 finish."]
            pub const IE4_3: u32 = 0x03;
        }
    }
    #[doc = "IRQ enable of segment 4."]
    pub mod IE4_EN {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt DONE disabled."]
            pub const IE4_EN_0: u32 = 0;
            #[doc = "Interrupt DONE enabled. When segment 4 finish, an interrupt will be generated on the specific port configured by the IE4."]
            pub const IE4_EN_1: u32 = 0x01;
        }
    }
    #[doc = "ADC hardware trigger command selection"]
    pub mod CSEL5 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not a valid selection from the command buffer. Trigger event is ignored."]
            pub const CSEL5_0: u32 = 0;
            #[doc = "ADC CMD1 selected."]
            pub const CSEL5_1: u32 = 0x01;
            #[doc = "ADC CMD2 selected."]
            pub const CSEL5_2: u32 = 0x02;
            #[doc = "ADC CMD3 selected."]
            pub const CSEL5_3: u32 = 0x03;
            #[doc = "ADC CMD4 selected."]
            pub const CSEL5_4: u32 = 0x04;
            #[doc = "ADC CMD5 selected."]
            pub const CSEL5_5: u32 = 0x05;
            #[doc = "ADC CMD6 selected."]
            pub const CSEL5_6: u32 = 0x06;
            #[doc = "ADC CMD7 selected."]
            pub const CSEL5_7: u32 = 0x07;
            #[doc = "ADC CMD8 selected."]
            pub const CSEL5_8: u32 = 0x08;
            #[doc = "ADC CMD9 selected."]
            pub const CSEL5_9: u32 = 0x09;
            #[doc = "ADC CMD10 selected."]
            pub const CSEL5_10: u32 = 0x0a;
            #[doc = "ADC CMD11 selected."]
            pub const CSEL5_11: u32 = 0x0b;
            #[doc = "ADC CMD12 selected."]
            pub const CSEL5_12: u32 = 0x0c;
            #[doc = "ADC CMD13 selected."]
            pub const CSEL5_13: u32 = 0x0d;
            #[doc = "ADC CMD14 selected."]
            pub const CSEL5_14: u32 = 0x0e;
            #[doc = "ADC CMD15 selected."]
            pub const CSEL5_15: u32 = 0x0f;
        }
    }
    #[doc = "Segment 5 HWTS ADC hardware trigger selection"]
    pub mod HWTS5 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "no trigger selected"]
            pub const HWTS5_0: u32 = 0;
            #[doc = "ADC TRIG0 selected"]
            pub const HWTS5_1: u32 = 0x01;
            #[doc = "ADC TRIG1 selected"]
            pub const HWTS5_2: u32 = 0x02;
            #[doc = "ADC TRIG2 selected"]
            pub const HWTS5_4: u32 = 0x04;
            #[doc = "ADC TRIG3 selected"]
            pub const HWTS5_8: u32 = 0x08;
            #[doc = "ADC TRIG4 selected"]
            pub const HWTS5_16: u32 = 0x10;
            #[doc = "ADC TRIG5 selected"]
            pub const HWTS5_32: u32 = 0x20;
            #[doc = "ADC TRIG6 selected"]
            pub const HWTS5_64: u32 = 0x40;
            #[doc = "ADC TRIG7 selected"]
            pub const HWTS5_128: u32 = 0x80;
        }
    }
    #[doc = "Segment 5 B2B"]
    pub mod B2B5 {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable B2B. Wait until delay value defined by TRIG5_COUNTER\\[SAMPLE_INTERVAL\\] is reached"]
            pub const B2B5_0: u32 = 0;
            #[doc = "Enable B2B. When Segment 0 finished (ADC COCO) then automatically trigger next ADC conversion, no need to wait until interval delay reached."]
            pub const B2B5_1: u32 = 0x01;
        }
    }
    #[doc = "Segment 5 interrupt enable. (This bit field is meaningful only when IE5_EN is set)"]
    pub mod IE5 {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Generate interrupt on Done0 when segment 5 finish."]
            pub const IE5_0: u32 = 0;
            #[doc = "Generate interrupt on Done1 when segment 5 finish."]
            pub const IE5_1: u32 = 0x01;
            #[doc = "Generate interrupt on Done2 when segment 5 finish."]
            pub const IE5_2: u32 = 0x02;
            #[doc = "Generate interrupt on Done3 when segment 5 finish."]
            pub const IE5_3: u32 = 0x03;
        }
    }
    #[doc = "IRQ enable of segment 5."]
    pub mod IE5_EN {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt DONE disabled."]
            pub const IE5_EN_0: u32 = 0;
            #[doc = "Interrupt DONE enabled. When segment 5 finish, an interrupt will be generated on the specific port configured by the IE5."]
            pub const IE5_EN_1: u32 = 0x01;
        }
    }
}
#[doc = "ETC_TRIG Chain 6/7 Register"]
pub mod TRIG1_CHAIN_7_6 {
    #[doc = "ADC hardware trigger command selection"]
    pub mod CSEL6 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not a valid selection from the command buffer. Trigger event is ignored."]
            pub const CSEL6_0: u32 = 0;
            #[doc = "ADC CMD1 selected."]
            pub const CSEL6_1: u32 = 0x01;
            #[doc = "ADC CMD2 selected."]
            pub const CSEL6_2: u32 = 0x02;
            #[doc = "ADC CMD3 selected."]
            pub const CSEL6_3: u32 = 0x03;
            #[doc = "ADC CMD4 selected."]
            pub const CSEL6_4: u32 = 0x04;
            #[doc = "ADC CMD5 selected."]
            pub const CSEL6_5: u32 = 0x05;
            #[doc = "ADC CMD6 selected."]
            pub const CSEL6_6: u32 = 0x06;
            #[doc = "ADC CMD7 selected."]
            pub const CSEL6_7: u32 = 0x07;
            #[doc = "ADC CMD8 selected."]
            pub const CSEL6_8: u32 = 0x08;
            #[doc = "ADC CMD9 selected."]
            pub const CSEL6_9: u32 = 0x09;
            #[doc = "ADC CMD10 selected."]
            pub const CSEL6_10: u32 = 0x0a;
            #[doc = "ADC CMD11 selected."]
            pub const CSEL6_11: u32 = 0x0b;
            #[doc = "ADC CMD12 selected."]
            pub const CSEL6_12: u32 = 0x0c;
            #[doc = "ADC CMD13 selected."]
            pub const CSEL6_13: u32 = 0x0d;
            #[doc = "ADC CMD14 selected."]
            pub const CSEL6_14: u32 = 0x0e;
            #[doc = "ADC CMD15 selected."]
            pub const CSEL6_15: u32 = 0x0f;
        }
    }
    #[doc = "Segment 6 HWTS ADC hardware trigger selection"]
    pub mod HWTS6 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "no trigger selected"]
            pub const HWTS6_0: u32 = 0;
            #[doc = "ADC TRIG0 selected"]
            pub const HWTS6_1: u32 = 0x01;
            #[doc = "ADC TRIG1 selected"]
            pub const HWTS6_2: u32 = 0x02;
            #[doc = "ADC TRIG2 selected"]
            pub const HWTS6_4: u32 = 0x04;
            #[doc = "ADC TRIG3 selected"]
            pub const HWTS6_8: u32 = 0x08;
            #[doc = "ADC TRIG4 selected"]
            pub const HWTS6_16: u32 = 0x10;
            #[doc = "ADC TRIG5 selected"]
            pub const HWTS6_32: u32 = 0x20;
            #[doc = "ADC TRIG6 selected"]
            pub const HWTS6_64: u32 = 0x40;
            #[doc = "ADC TRIG7 selected"]
            pub const HWTS6_128: u32 = 0x80;
        }
    }
    #[doc = "Segment 6 B2B"]
    pub mod B2B6 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable B2B. Wait until delay value defined by TRIG6_COUNTER\\[SAMPLE_INTERVAL\\] is reached"]
            pub const B2B6_0: u32 = 0;
            #[doc = "Enable B2B. When Segment 0 finished (ADC COCO) then automatically trigger next ADC conversion, no need to wait until interval delay reached."]
            pub const B2B6_1: u32 = 0x01;
        }
    }
    #[doc = "Segment 6 interrupt enable. (This bit field is meaningful only when IE6_EN is set)"]
    pub mod IE6 {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Generate interrupt on Done0 when segment 6 finish."]
            pub const IE6_0: u32 = 0;
            #[doc = "Generate interrupt on Done1 when segment 6 finish."]
            pub const IE6_1: u32 = 0x01;
            #[doc = "Generate interrupt on Done2 when segment 6 finish."]
            pub const IE6_2: u32 = 0x02;
            #[doc = "Generate interrupt on Done3 when segment 6 finish."]
            pub const IE6_3: u32 = 0x03;
        }
    }
    #[doc = "IRQ enable of segment 6."]
    pub mod IE6_EN {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt DONE disabled."]
            pub const IE6_EN_0: u32 = 0;
            #[doc = "Interrupt DONE enabled. When segment 6 finish, an interrupt will be generated on the specific port configured by the IE6."]
            pub const IE6_EN_1: u32 = 0x01;
        }
    }
    #[doc = "ADC hardware trigger command selection"]
    pub mod CSEL7 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not a valid selection from the command buffer. Trigger event is ignored."]
            pub const CSEL7_0: u32 = 0;
            #[doc = "ADC CMD1 selected."]
            pub const CSEL7_1: u32 = 0x01;
            #[doc = "ADC CMD2 selected."]
            pub const CSEL7_2: u32 = 0x02;
            #[doc = "ADC CMD3 selected."]
            pub const CSEL7_3: u32 = 0x03;
            #[doc = "ADC CMD4 selected."]
            pub const CSEL7_4: u32 = 0x04;
            #[doc = "ADC CMD5 selected."]
            pub const CSEL7_5: u32 = 0x05;
            #[doc = "ADC CMD6 selected."]
            pub const CSEL7_6: u32 = 0x06;
            #[doc = "ADC CMD7 selected."]
            pub const CSEL7_7: u32 = 0x07;
            #[doc = "ADC CMD8 selected."]
            pub const CSEL7_8: u32 = 0x08;
            #[doc = "ADC CMD9 selected."]
            pub const CSEL7_9: u32 = 0x09;
            #[doc = "ADC CMD10 selected."]
            pub const CSEL7_10: u32 = 0x0a;
            #[doc = "ADC CMD11 selected."]
            pub const CSEL7_11: u32 = 0x0b;
            #[doc = "ADC CMD12 selected."]
            pub const CSEL7_12: u32 = 0x0c;
            #[doc = "ADC CMD13 selected."]
            pub const CSEL7_13: u32 = 0x0d;
            #[doc = "ADC CMD14 selected."]
            pub const CSEL7_14: u32 = 0x0e;
            #[doc = "ADC CMD15 selected."]
            pub const CSEL7_15: u32 = 0x0f;
        }
    }
    #[doc = "Segment 7 HWTS ADC hardware trigger selection"]
    pub mod HWTS7 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "no trigger selected"]
            pub const HWTS7_0: u32 = 0;
            #[doc = "ADC TRIG0 selected"]
            pub const HWTS7_1: u32 = 0x01;
            #[doc = "ADC TRIG1 selected"]
            pub const HWTS7_2: u32 = 0x02;
            #[doc = "ADC TRIG2 selected"]
            pub const HWTS7_4: u32 = 0x04;
            #[doc = "ADC TRIG3 selected"]
            pub const HWTS7_8: u32 = 0x08;
            #[doc = "ADC TRIG4 selected"]
            pub const HWTS7_16: u32 = 0x10;
            #[doc = "ADC TRIG5 selected"]
            pub const HWTS7_32: u32 = 0x20;
            #[doc = "ADC TRIG6 selected"]
            pub const HWTS7_64: u32 = 0x40;
            #[doc = "ADC TRIG7 selected"]
            pub const HWTS7_128: u32 = 0x80;
        }
    }
    #[doc = "Segment 7 B2B"]
    pub mod B2B7 {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable B2B. Wait until delay value defined by TRIG7_COUNTER\\[SAMPLE_INTERVAL\\] is reached"]
            pub const B2B7_0: u32 = 0;
            #[doc = "Enable B2B. When Segment 0 finished (ADC COCO) then automatically trigger next ADC conversion, no need to wait until interval delay reached."]
            pub const B2B7_1: u32 = 0x01;
        }
    }
    #[doc = "Segment 7 interrupt enable. (This bit field is meaningful only when IE7_EN is set)"]
    pub mod IE7 {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Generate interrupt on Done0 when segment 7 finish."]
            pub const IE7_0: u32 = 0;
            #[doc = "Generate interrupt on Done1 when segment 7 finish."]
            pub const IE7_1: u32 = 0x01;
            #[doc = "Generate interrupt on Done2 when segment 7 finish."]
            pub const IE7_2: u32 = 0x02;
            #[doc = "Generate interrupt on Done3 when segment 7 finish."]
            pub const IE7_3: u32 = 0x03;
        }
    }
    #[doc = "IRQ enable of segment 7."]
    pub mod IE7_EN {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt DONE disabled."]
            pub const IE7_EN_0: u32 = 0;
            #[doc = "Interrupt DONE enabled. When segment 7 finish, an interrupt will be generated on the specific port configured by the IE7."]
            pub const IE7_EN_1: u32 = 0x01;
        }
    }
}
#[doc = "ETC_TRIG Result Data 1/0 Register"]
pub mod TRIG1_RESULT_1_0 {
    #[doc = "Result DATA0"]
    pub mod DATA0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Result DATA1"]
    pub mod DATA1 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ETC_TRIG Result Data 3/2 Register"]
pub mod TRIG1_RESULT_3_2 {
    #[doc = "Result DATA2"]
    pub mod DATA2 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Result DATA3"]
    pub mod DATA3 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ETC_TRIG Result Data 5/4 Register"]
pub mod TRIG1_RESULT_5_4 {
    #[doc = "Result DATA4"]
    pub mod DATA4 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Result DATA5"]
    pub mod DATA5 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ETC_TRIG Result Data 7/6 Register"]
pub mod TRIG1_RESULT_7_6 {
    #[doc = "Result DATA6"]
    pub mod DATA6 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Result DATA7"]
    pub mod DATA7 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ETC_TRIG Control Register"]
pub mod TRIG2_CTRL {
    #[doc = "Software trigger. This field is self-clearing."]
    pub mod SW_TRIG {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No software trigger event generated."]
            pub const SW_TRIG_0: u32 = 0;
            #[doc = "Software trigger event generated."]
            pub const SW_TRIG_1: u32 = 0x01;
        }
    }
    #[doc = "Trigger mode selection."]
    pub mod TRIG_MODE {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Hardware trigger. The softerware trigger will be ignored."]
            pub const TRIG_MODE_0: u32 = 0;
            #[doc = "Software trigger. The hardware trigger will be ignored."]
            pub const TRIG_MODE_1: u32 = 0x01;
        }
    }
    #[doc = "The number of segments inside the trigger chain of TRIGa."]
    pub mod TRIG_CHAIN {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Trigger chain length is 1"]
            pub const TRIG_CHAIN_0: u32 = 0;
            #[doc = "Trigger chain length is 2"]
            pub const TRIG_CHAIN_1: u32 = 0x01;
            #[doc = "Trigger chain length is 3"]
            pub const TRIG_CHAIN_2: u32 = 0x02;
            #[doc = "Trigger chain length is 4"]
            pub const TRIG_CHAIN_3: u32 = 0x03;
            #[doc = "Trigger chain length is 5"]
            pub const TRIG_CHAIN_4: u32 = 0x04;
            #[doc = "Trigger chain length is 6"]
            pub const TRIG_CHAIN_5: u32 = 0x05;
            #[doc = "Trigger chain length is 7"]
            pub const TRIG_CHAIN_6: u32 = 0x06;
            #[doc = "Trigger chain length is 8"]
            pub const TRIG_CHAIN_7: u32 = 0x07;
        }
    }
    #[doc = "External trigger priority, 7 is highest priority, while 0 is lowest"]
    pub mod TRIG_PRIORITY {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Trigger synchronization mode selection"]
    pub mod SYNC_MODE {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Synchronization mode disabled, TRIGa and TRIG(a+4) are triggered independently."]
            pub const SYNC_MODE_0: u32 = 0;
            #[doc = "Synchronization mode enabled, TRIGa and TRIG(a+4) are triggered by TRIGa source synchronously."]
            pub const SYNC_MODE_1: u32 = 0x01;
        }
    }
    #[doc = "Segment x done detection"]
    pub mod CHAINX_DONE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "segment x done not detected."]
            pub const CHAINX_DONE_0: u32 = 0;
            #[doc = "segment x done detected."]
            pub const CHAINX_DONE_1: u32 = 0x01;
        }
    }
}
#[doc = "ETC_TRIG Counter Register"]
pub mod TRIG2_COUNTER {
    #[doc = "TRIGGER initial delay counter"]
    pub mod INIT_DELAY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TRIGGER sampling interval counter"]
    pub mod SAMPLE_INTERVAL {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ETC_TRIG Chain 0/1 Register"]
pub mod TRIG2_CHAIN_1_0 {
    #[doc = "ADC hardware trigger command selection"]
    pub mod CSEL0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not a valid selection from the command buffer. Trigger event is ignored."]
            pub const CSEL0_0: u32 = 0;
            #[doc = "ADC CMD1 selected."]
            pub const CSEL0_1: u32 = 0x01;
            #[doc = "ADC CMD2 selected."]
            pub const CSEL0_2: u32 = 0x02;
            #[doc = "ADC CMD3 selected."]
            pub const CSEL0_3: u32 = 0x03;
            #[doc = "ADC CMD4 selected."]
            pub const CSEL0_4: u32 = 0x04;
            #[doc = "ADC CMD5 selected."]
            pub const CSEL0_5: u32 = 0x05;
            #[doc = "ADC CMD6 selected."]
            pub const CSEL0_6: u32 = 0x06;
            #[doc = "ADC CMD7 selected."]
            pub const CSEL0_7: u32 = 0x07;
            #[doc = "ADC CMD8 selected."]
            pub const CSEL0_8: u32 = 0x08;
            #[doc = "ADC CMD9 selected."]
            pub const CSEL0_9: u32 = 0x09;
            #[doc = "ADC CMD10 selected."]
            pub const CSEL0_10: u32 = 0x0a;
            #[doc = "ADC CMD11 selected."]
            pub const CSEL0_11: u32 = 0x0b;
            #[doc = "ADC CMD12 selected."]
            pub const CSEL0_12: u32 = 0x0c;
            #[doc = "ADC CMD13 selected."]
            pub const CSEL0_13: u32 = 0x0d;
            #[doc = "ADC CMD14 selected."]
            pub const CSEL0_14: u32 = 0x0e;
            #[doc = "ADC CMD15 selected."]
            pub const CSEL0_15: u32 = 0x0f;
        }
    }
    #[doc = "Segment 0 HWTS ADC hardware trigger selection"]
    pub mod HWTS0 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "no trigger selected"]
            pub const HWTS0_0: u32 = 0;
            #[doc = "ADC TRIG0 selected"]
            pub const HWTS0_1: u32 = 0x01;
            #[doc = "ADC TRIG1 selected"]
            pub const HWTS0_2: u32 = 0x02;
            #[doc = "ADC TRIG2 selected"]
            pub const HWTS0_4: u32 = 0x04;
            #[doc = "ADC TRIG3 selected"]
            pub const HWTS0_8: u32 = 0x08;
            #[doc = "ADC TRIG4 selected"]
            pub const HWTS0_16: u32 = 0x10;
            #[doc = "ADC TRIG5 selected"]
            pub const HWTS0_32: u32 = 0x20;
            #[doc = "ADC TRIG6 selected"]
            pub const HWTS0_64: u32 = 0x40;
            #[doc = "ADC TRIG7 selected"]
            pub const HWTS0_128: u32 = 0x80;
        }
    }
    #[doc = "Segment 0 B2B"]
    pub mod B2B0 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable B2B. Wait until delay value defined by TRIG0_COUNTER\\[SAMPLE_INTERVAL\\] is reached"]
            pub const B2B0_0: u32 = 0;
            #[doc = "Enable B2B. When Segment 0 finished (ADC COCO) then automatically trigger next ADC conversion, no need to wait until interval delay reached."]
            pub const B2B0_1: u32 = 0x01;
        }
    }
    #[doc = "Segment 0 interrupt enable. (This bit field is meaningful only when IE0_EN is set)"]
    pub mod IE0 {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Generate interrupt on Done0 when segment 0 finish."]
            pub const IE0_0: u32 = 0;
            #[doc = "Generate interrupt on Done1 when segment 0 finish."]
            pub const IE0_1: u32 = 0x01;
            #[doc = "Generate interrupt on Done2 when segment 0 finish."]
            pub const IE0_2: u32 = 0x02;
            #[doc = "Generate interrupt on Done3 when segment 0 finish."]
            pub const IE0_3: u32 = 0x03;
        }
    }
    #[doc = "IRQ enable of segment 0."]
    pub mod IE0_EN {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt DONE disabled."]
            pub const IE0_EN_0: u32 = 0;
            #[doc = "Interrupt DONE enabled. When segment 0 finish, an interrupt will be generated on the specific port configured by the IE0."]
            pub const IE0_EN_1: u32 = 0x01;
        }
    }
    #[doc = "ADC hardware trigger command selection"]
    pub mod CSEL1 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not a valid selection from the command buffer. Trigger event is ignored."]
            pub const CSEL1_0: u32 = 0;
            #[doc = "ADC CMD1 selected."]
            pub const CSEL1_1: u32 = 0x01;
            #[doc = "ADC CMD2 selected."]
            pub const CSEL1_2: u32 = 0x02;
            #[doc = "ADC CMD3 selected."]
            pub const CSEL1_3: u32 = 0x03;
            #[doc = "ADC CMD4 selected."]
            pub const CSEL1_4: u32 = 0x04;
            #[doc = "ADC CMD5 selected."]
            pub const CSEL1_5: u32 = 0x05;
            #[doc = "ADC CMD6 selected."]
            pub const CSEL1_6: u32 = 0x06;
            #[doc = "ADC CMD7 selected."]
            pub const CSEL1_7: u32 = 0x07;
            #[doc = "ADC CMD8 selected."]
            pub const CSEL1_8: u32 = 0x08;
            #[doc = "ADC CMD9 selected."]
            pub const CSEL1_9: u32 = 0x09;
            #[doc = "ADC CMD10 selected."]
            pub const CSEL1_10: u32 = 0x0a;
            #[doc = "ADC CMD11 selected."]
            pub const CSEL1_11: u32 = 0x0b;
            #[doc = "ADC CMD12 selected."]
            pub const CSEL1_12: u32 = 0x0c;
            #[doc = "ADC CMD13 selected."]
            pub const CSEL1_13: u32 = 0x0d;
            #[doc = "ADC CMD14 selected."]
            pub const CSEL1_14: u32 = 0x0e;
            #[doc = "ADC CMD15 selected."]
            pub const CSEL1_15: u32 = 0x0f;
        }
    }
    #[doc = "Segment 1 HWTS ADC hardware trigger selection"]
    pub mod HWTS1 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "no trigger selected"]
            pub const HWTS1_0: u32 = 0;
            #[doc = "ADC TRIG0 selected"]
            pub const HWTS1_1: u32 = 0x01;
            #[doc = "ADC TRIG1 selected"]
            pub const HWTS1_2: u32 = 0x02;
            #[doc = "ADC TRIG2 selected"]
            pub const HWTS1_4: u32 = 0x04;
            #[doc = "ADC TRIG3 selected"]
            pub const HWTS1_8: u32 = 0x08;
            #[doc = "ADC TRIG4 selected"]
            pub const HWTS1_16: u32 = 0x10;
            #[doc = "ADC TRIG5 selected"]
            pub const HWTS1_32: u32 = 0x20;
            #[doc = "ADC TRIG6 selected"]
            pub const HWTS1_64: u32 = 0x40;
            #[doc = "ADC TRIG7 selected"]
            pub const HWTS1_128: u32 = 0x80;
        }
    }
    #[doc = "Segment 1 B2B"]
    pub mod B2B1 {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable B2B. Wait until delay value defined by TRIG1_COUNTER\\[SAMPLE_INTERVAL\\] is reached"]
            pub const B2B1_0: u32 = 0;
            #[doc = "Enable B2B. When Segment 0 finished (ADC COCO) then automatically trigger next ADC conversion, no need to wait until interval delay reached."]
            pub const B2B1_1: u32 = 0x01;
        }
    }
    #[doc = "Segment 1 interrupt enable. (This bit field is meaningful only when IE1_EN is set)"]
    pub mod IE1 {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Generate interrupt on Done0 when Segment 1 finish."]
            pub const IE1_0: u32 = 0;
            #[doc = "Generate interrupt on Done1 when Segment 1 finish."]
            pub const IE1_1: u32 = 0x01;
            #[doc = "Generate interrupt on Done2 when Segment 1 finish."]
            pub const IE1_2: u32 = 0x02;
            #[doc = "Generate interrupt on Done3 when Segment 1 finish."]
            pub const IE1_3: u32 = 0x03;
        }
    }
    #[doc = "IRQ enable of segment 1."]
    pub mod IE1_EN {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt DONE disabled."]
            pub const IE1_EN_0: u32 = 0;
            #[doc = "Interrupt DONE enabled. When segment 1 finish, an interrupt will be generated on the specific port configured by the IE1."]
            pub const IE1_EN_1: u32 = 0x01;
        }
    }
}
#[doc = "ETC_TRIG Chain 2/3 Register"]
pub mod TRIG2_CHAIN_3_2 {
    #[doc = "ADC hardware trigger command selection"]
    pub mod CSEL2 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not a valid selection from the command buffer. Trigger event is ignored."]
            pub const CSEL2_0: u32 = 0;
            #[doc = "ADC CMD1 selected."]
            pub const CSEL2_1: u32 = 0x01;
            #[doc = "ADC CMD2 selected."]
            pub const CSEL2_2: u32 = 0x02;
            #[doc = "ADC CMD3 selected."]
            pub const CSEL2_3: u32 = 0x03;
            #[doc = "ADC CMD4 selected."]
            pub const CSEL2_4: u32 = 0x04;
            #[doc = "ADC CMD5 selected."]
            pub const CSEL2_5: u32 = 0x05;
            #[doc = "ADC CMD6 selected."]
            pub const CSEL2_6: u32 = 0x06;
            #[doc = "ADC CMD7 selected."]
            pub const CSEL2_7: u32 = 0x07;
            #[doc = "ADC CMD8 selected."]
            pub const CSEL2_8: u32 = 0x08;
            #[doc = "ADC CMD9 selected."]
            pub const CSEL2_9: u32 = 0x09;
            #[doc = "ADC CMD10 selected."]
            pub const CSEL2_10: u32 = 0x0a;
            #[doc = "ADC CMD11 selected."]
            pub const CSEL2_11: u32 = 0x0b;
            #[doc = "ADC CMD12 selected."]
            pub const CSEL2_12: u32 = 0x0c;
            #[doc = "ADC CMD13 selected."]
            pub const CSEL2_13: u32 = 0x0d;
            #[doc = "ADC CMD14 selected."]
            pub const CSEL2_14: u32 = 0x0e;
            #[doc = "ADC CMD15 selected."]
            pub const CSEL2_15: u32 = 0x0f;
        }
    }
    #[doc = "Segment 2 HWTS ADC hardware trigger selection"]
    pub mod HWTS2 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "no trigger selected"]
            pub const HWTS2_0: u32 = 0;
            #[doc = "ADC TRIG0 selected"]
            pub const HWTS2_1: u32 = 0x01;
            #[doc = "ADC TRIG1 selected"]
            pub const HWTS2_2: u32 = 0x02;
            #[doc = "ADC TRIG2 selected"]
            pub const HWTS2_4: u32 = 0x04;
            #[doc = "ADC TRIG3 selected"]
            pub const HWTS2_8: u32 = 0x08;
            #[doc = "ADC TRIG4 selected"]
            pub const HWTS2_16: u32 = 0x10;
            #[doc = "ADC TRIG5 selected"]
            pub const HWTS2_32: u32 = 0x20;
            #[doc = "ADC TRIG6 selected"]
            pub const HWTS2_64: u32 = 0x40;
            #[doc = "ADC TRIG7 selected"]
            pub const HWTS2_128: u32 = 0x80;
        }
    }
    #[doc = "Segment 2 B2B"]
    pub mod B2B2 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable B2B. Wait until delay value defined by TRIG2_COUNTER\\[SAMPLE_INTERVAL\\] is reached"]
            pub const B2B2_0: u32 = 0;
            #[doc = "Enable B2B. When Segment 0 finished (ADC COCO) then automatically trigger next ADC conversion, no need to wait until interval delay reached."]
            pub const B2B2_1: u32 = 0x01;
        }
    }
    #[doc = "Segment 2 interrupt enable. (This bit field is meaningful only when IE2_EN is set)"]
    pub mod IE2 {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Generate interrupt on Done0 when segment 2 finish."]
            pub const IE2_0: u32 = 0;
            #[doc = "Generate interrupt on Done1 when segment 2 finish."]
            pub const IE2_1: u32 = 0x01;
            #[doc = "Generate interrupt on Done2 when segment 2 finish."]
            pub const IE2_2: u32 = 0x02;
            #[doc = "Generate interrupt on Done3 when segment 2 finish."]
            pub const IE2_3: u32 = 0x03;
        }
    }
    #[doc = "IRQ enable of segment 2."]
    pub mod IE2_EN {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt DONE disabled."]
            pub const IE2_EN_0: u32 = 0;
            #[doc = "Interrupt DONE enabled. When segment 2 finish, an interrupt will be generated on the specific port configured by the IE2."]
            pub const IE2_EN_1: u32 = 0x01;
        }
    }
    #[doc = "ADC hardware trigger command selection"]
    pub mod CSEL3 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not a valid selection from the command buffer. Trigger event is ignored."]
            pub const CSEL3_0: u32 = 0;
            #[doc = "ADC CMD1 selected."]
            pub const CSEL3_1: u32 = 0x01;
            #[doc = "ADC CMD2 selected."]
            pub const CSEL3_2: u32 = 0x02;
            #[doc = "ADC CMD3 selected."]
            pub const CSEL3_3: u32 = 0x03;
            #[doc = "ADC CMD4 selected."]
            pub const CSEL3_4: u32 = 0x04;
            #[doc = "ADC CMD5 selected."]
            pub const CSEL3_5: u32 = 0x05;
            #[doc = "ADC CMD6 selected."]
            pub const CSEL3_6: u32 = 0x06;
            #[doc = "ADC CMD7 selected."]
            pub const CSEL3_7: u32 = 0x07;
            #[doc = "ADC CMD8 selected."]
            pub const CSEL3_8: u32 = 0x08;
            #[doc = "ADC CMD9 selected."]
            pub const CSEL3_9: u32 = 0x09;
            #[doc = "ADC CMD10 selected."]
            pub const CSEL3_10: u32 = 0x0a;
            #[doc = "ADC CMD11 selected."]
            pub const CSEL3_11: u32 = 0x0b;
            #[doc = "ADC CMD12 selected."]
            pub const CSEL3_12: u32 = 0x0c;
            #[doc = "ADC CMD13 selected."]
            pub const CSEL3_13: u32 = 0x0d;
            #[doc = "ADC CMD14 selected."]
            pub const CSEL3_14: u32 = 0x0e;
            #[doc = "ADC CMD15 selected."]
            pub const CSEL3_15: u32 = 0x0f;
        }
    }
    #[doc = "Segment 3 HWTS ADC hardware trigger selection"]
    pub mod HWTS3 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "no trigger selected"]
            pub const HWTS3_0: u32 = 0;
            #[doc = "ADC TRIG0 selected"]
            pub const HWTS3_1: u32 = 0x01;
            #[doc = "ADC TRIG1 selected"]
            pub const HWTS3_2: u32 = 0x02;
            #[doc = "ADC TRIG2 selected"]
            pub const HWTS3_4: u32 = 0x04;
            #[doc = "ADC TRIG3 selected"]
            pub const HWTS3_8: u32 = 0x08;
            #[doc = "ADC TRIG4 selected"]
            pub const HWTS3_16: u32 = 0x10;
            #[doc = "ADC TRIG5 selected"]
            pub const HWTS3_32: u32 = 0x20;
            #[doc = "ADC TRIG6 selected"]
            pub const HWTS3_64: u32 = 0x40;
            #[doc = "ADC TRIG7 selected"]
            pub const HWTS3_128: u32 = 0x80;
        }
    }
    #[doc = "Segment 3 B2B"]
    pub mod B2B3 {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable B2B. Wait until delay value defined by TRIG3_COUNTER\\[SAMPLE_INTERVAL\\] is reached"]
            pub const B2B3_0: u32 = 0;
            #[doc = "Enable B2B. When Segment 0 finished (ADC COCO) then automatically trigger next ADC conversion, no need to wait until interval delay reached."]
            pub const B2B3_1: u32 = 0x01;
        }
    }
    #[doc = "Segment 3 interrupt enable. (This bit field is meaningful only when IE3_EN is set)"]
    pub mod IE3 {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Generate interrupt on Done0 when segment 3 finish."]
            pub const IE3_0: u32 = 0;
            #[doc = "Generate interrupt on Done1 when segment 3 finish."]
            pub const IE3_1: u32 = 0x01;
            #[doc = "Generate interrupt on Done2 when segment 3 finish."]
            pub const IE3_2: u32 = 0x02;
            #[doc = "Generate interrupt on Done3 when segment 3 finish."]
            pub const IE3_3: u32 = 0x03;
        }
    }
    #[doc = "IRQ enable of segment 3."]
    pub mod IE3_EN {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt DONE disabled."]
            pub const IE3_EN_0: u32 = 0;
            #[doc = "Interrupt DONE enabled. When segment 3 finish, an interrupt will be generated on the specific port configured by the IE3."]
            pub const IE3_EN_1: u32 = 0x01;
        }
    }
}
#[doc = "ETC_TRIG Chain 4/5 Register"]
pub mod TRIG2_CHAIN_5_4 {
    #[doc = "ADC hardware trigger command selection"]
    pub mod CSEL4 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not a valid selection from the command buffer. Trigger event is ignored."]
            pub const CSEL4_0: u32 = 0;
            #[doc = "ADC CMD1 selected."]
            pub const CSEL4_1: u32 = 0x01;
            #[doc = "ADC CMD2 selected."]
            pub const CSEL4_2: u32 = 0x02;
            #[doc = "ADC CMD3 selected."]
            pub const CSEL4_3: u32 = 0x03;
            #[doc = "ADC CMD4 selected."]
            pub const CSEL4_4: u32 = 0x04;
            #[doc = "ADC CMD5 selected."]
            pub const CSEL4_5: u32 = 0x05;
            #[doc = "ADC CMD6 selected."]
            pub const CSEL4_6: u32 = 0x06;
            #[doc = "ADC CMD7 selected."]
            pub const CSEL4_7: u32 = 0x07;
            #[doc = "ADC CMD8 selected."]
            pub const CSEL4_8: u32 = 0x08;
            #[doc = "ADC CMD9 selected."]
            pub const CSEL4_9: u32 = 0x09;
            #[doc = "ADC CMD10 selected."]
            pub const CSEL4_10: u32 = 0x0a;
            #[doc = "ADC CMD11 selected."]
            pub const CSEL4_11: u32 = 0x0b;
            #[doc = "ADC CMD12 selected."]
            pub const CSEL4_12: u32 = 0x0c;
            #[doc = "ADC CMD13 selected."]
            pub const CSEL4_13: u32 = 0x0d;
            #[doc = "ADC CMD14 selected."]
            pub const CSEL4_14: u32 = 0x0e;
            #[doc = "ADC CMD15 selected."]
            pub const CSEL4_15: u32 = 0x0f;
        }
    }
    #[doc = "Segment 4 HWTS ADC hardware trigger selection"]
    pub mod HWTS4 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "no trigger selected"]
            pub const HWTS4_0: u32 = 0;
            #[doc = "ADC TRIG0 selected"]
            pub const HWTS4_1: u32 = 0x01;
            #[doc = "ADC TRIG1 selected"]
            pub const HWTS4_2: u32 = 0x02;
            #[doc = "ADC TRIG2 selected"]
            pub const HWTS4_4: u32 = 0x04;
            #[doc = "ADC TRIG3 selected"]
            pub const HWTS4_8: u32 = 0x08;
            #[doc = "ADC TRIG4 selected"]
            pub const HWTS4_16: u32 = 0x10;
            #[doc = "ADC TRIG5 selected"]
            pub const HWTS4_32: u32 = 0x20;
            #[doc = "ADC TRIG6 selected"]
            pub const HWTS4_64: u32 = 0x40;
            #[doc = "ADC TRIG7 selected"]
            pub const HWTS4_128: u32 = 0x80;
        }
    }
    #[doc = "Segment 4 B2B"]
    pub mod B2B4 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable B2B. Wait until delay value defined by TRIG4_COUNTER\\[SAMPLE_INTERVAL\\] is reached"]
            pub const B2B4_0: u32 = 0;
            #[doc = "Enable B2B. When Segment 0 finished (ADC COCO) then automatically trigger next ADC conversion, no need to wait until interval delay reached."]
            pub const B2B4_1: u32 = 0x01;
        }
    }
    #[doc = "Segment 4 interrupt enable. (This bit field is meaningful only when IE4_EN is set)"]
    pub mod IE4 {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Generate interrupt on Done0 when segment 4 finish."]
            pub const IE4_0: u32 = 0;
            #[doc = "Generate interrupt on Done1 when segment 4 finish."]
            pub const IE4_1: u32 = 0x01;
            #[doc = "Generate interrupt on Done2 when segment 4 finish."]
            pub const IE4_2: u32 = 0x02;
            #[doc = "Generate interrupt on Done3 when segment 4 finish."]
            pub const IE4_3: u32 = 0x03;
        }
    }
    #[doc = "IRQ enable of segment 4."]
    pub mod IE4_EN {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt DONE disabled."]
            pub const IE4_EN_0: u32 = 0;
            #[doc = "Interrupt DONE enabled. When segment 4 finish, an interrupt will be generated on the specific port configured by the IE4."]
            pub const IE4_EN_1: u32 = 0x01;
        }
    }
    #[doc = "ADC hardware trigger command selection"]
    pub mod CSEL5 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not a valid selection from the command buffer. Trigger event is ignored."]
            pub const CSEL5_0: u32 = 0;
            #[doc = "ADC CMD1 selected."]
            pub const CSEL5_1: u32 = 0x01;
            #[doc = "ADC CMD2 selected."]
            pub const CSEL5_2: u32 = 0x02;
            #[doc = "ADC CMD3 selected."]
            pub const CSEL5_3: u32 = 0x03;
            #[doc = "ADC CMD4 selected."]
            pub const CSEL5_4: u32 = 0x04;
            #[doc = "ADC CMD5 selected."]
            pub const CSEL5_5: u32 = 0x05;
            #[doc = "ADC CMD6 selected."]
            pub const CSEL5_6: u32 = 0x06;
            #[doc = "ADC CMD7 selected."]
            pub const CSEL5_7: u32 = 0x07;
            #[doc = "ADC CMD8 selected."]
            pub const CSEL5_8: u32 = 0x08;
            #[doc = "ADC CMD9 selected."]
            pub const CSEL5_9: u32 = 0x09;
            #[doc = "ADC CMD10 selected."]
            pub const CSEL5_10: u32 = 0x0a;
            #[doc = "ADC CMD11 selected."]
            pub const CSEL5_11: u32 = 0x0b;
            #[doc = "ADC CMD12 selected."]
            pub const CSEL5_12: u32 = 0x0c;
            #[doc = "ADC CMD13 selected."]
            pub const CSEL5_13: u32 = 0x0d;
            #[doc = "ADC CMD14 selected."]
            pub const CSEL5_14: u32 = 0x0e;
            #[doc = "ADC CMD15 selected."]
            pub const CSEL5_15: u32 = 0x0f;
        }
    }
    #[doc = "Segment 5 HWTS ADC hardware trigger selection"]
    pub mod HWTS5 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "no trigger selected"]
            pub const HWTS5_0: u32 = 0;
            #[doc = "ADC TRIG0 selected"]
            pub const HWTS5_1: u32 = 0x01;
            #[doc = "ADC TRIG1 selected"]
            pub const HWTS5_2: u32 = 0x02;
            #[doc = "ADC TRIG2 selected"]
            pub const HWTS5_4: u32 = 0x04;
            #[doc = "ADC TRIG3 selected"]
            pub const HWTS5_8: u32 = 0x08;
            #[doc = "ADC TRIG4 selected"]
            pub const HWTS5_16: u32 = 0x10;
            #[doc = "ADC TRIG5 selected"]
            pub const HWTS5_32: u32 = 0x20;
            #[doc = "ADC TRIG6 selected"]
            pub const HWTS5_64: u32 = 0x40;
            #[doc = "ADC TRIG7 selected"]
            pub const HWTS5_128: u32 = 0x80;
        }
    }
    #[doc = "Segment 5 B2B"]
    pub mod B2B5 {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable B2B. Wait until delay value defined by TRIG5_COUNTER\\[SAMPLE_INTERVAL\\] is reached"]
            pub const B2B5_0: u32 = 0;
            #[doc = "Enable B2B. When Segment 0 finished (ADC COCO) then automatically trigger next ADC conversion, no need to wait until interval delay reached."]
            pub const B2B5_1: u32 = 0x01;
        }
    }
    #[doc = "Segment 5 interrupt enable. (This bit field is meaningful only when IE5_EN is set)"]
    pub mod IE5 {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Generate interrupt on Done0 when segment 5 finish."]
            pub const IE5_0: u32 = 0;
            #[doc = "Generate interrupt on Done1 when segment 5 finish."]
            pub const IE5_1: u32 = 0x01;
            #[doc = "Generate interrupt on Done2 when segment 5 finish."]
            pub const IE5_2: u32 = 0x02;
            #[doc = "Generate interrupt on Done3 when segment 5 finish."]
            pub const IE5_3: u32 = 0x03;
        }
    }
    #[doc = "IRQ enable of segment 5."]
    pub mod IE5_EN {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt DONE disabled."]
            pub const IE5_EN_0: u32 = 0;
            #[doc = "Interrupt DONE enabled. When segment 5 finish, an interrupt will be generated on the specific port configured by the IE5."]
            pub const IE5_EN_1: u32 = 0x01;
        }
    }
}
#[doc = "ETC_TRIG Chain 6/7 Register"]
pub mod TRIG2_CHAIN_7_6 {
    #[doc = "ADC hardware trigger command selection"]
    pub mod CSEL6 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not a valid selection from the command buffer. Trigger event is ignored."]
            pub const CSEL6_0: u32 = 0;
            #[doc = "ADC CMD1 selected."]
            pub const CSEL6_1: u32 = 0x01;
            #[doc = "ADC CMD2 selected."]
            pub const CSEL6_2: u32 = 0x02;
            #[doc = "ADC CMD3 selected."]
            pub const CSEL6_3: u32 = 0x03;
            #[doc = "ADC CMD4 selected."]
            pub const CSEL6_4: u32 = 0x04;
            #[doc = "ADC CMD5 selected."]
            pub const CSEL6_5: u32 = 0x05;
            #[doc = "ADC CMD6 selected."]
            pub const CSEL6_6: u32 = 0x06;
            #[doc = "ADC CMD7 selected."]
            pub const CSEL6_7: u32 = 0x07;
            #[doc = "ADC CMD8 selected."]
            pub const CSEL6_8: u32 = 0x08;
            #[doc = "ADC CMD9 selected."]
            pub const CSEL6_9: u32 = 0x09;
            #[doc = "ADC CMD10 selected."]
            pub const CSEL6_10: u32 = 0x0a;
            #[doc = "ADC CMD11 selected."]
            pub const CSEL6_11: u32 = 0x0b;
            #[doc = "ADC CMD12 selected."]
            pub const CSEL6_12: u32 = 0x0c;
            #[doc = "ADC CMD13 selected."]
            pub const CSEL6_13: u32 = 0x0d;
            #[doc = "ADC CMD14 selected."]
            pub const CSEL6_14: u32 = 0x0e;
            #[doc = "ADC CMD15 selected."]
            pub const CSEL6_15: u32 = 0x0f;
        }
    }
    #[doc = "Segment 6 HWTS ADC hardware trigger selection"]
    pub mod HWTS6 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "no trigger selected"]
            pub const HWTS6_0: u32 = 0;
            #[doc = "ADC TRIG0 selected"]
            pub const HWTS6_1: u32 = 0x01;
            #[doc = "ADC TRIG1 selected"]
            pub const HWTS6_2: u32 = 0x02;
            #[doc = "ADC TRIG2 selected"]
            pub const HWTS6_4: u32 = 0x04;
            #[doc = "ADC TRIG3 selected"]
            pub const HWTS6_8: u32 = 0x08;
            #[doc = "ADC TRIG4 selected"]
            pub const HWTS6_16: u32 = 0x10;
            #[doc = "ADC TRIG5 selected"]
            pub const HWTS6_32: u32 = 0x20;
            #[doc = "ADC TRIG6 selected"]
            pub const HWTS6_64: u32 = 0x40;
            #[doc = "ADC TRIG7 selected"]
            pub const HWTS6_128: u32 = 0x80;
        }
    }
    #[doc = "Segment 6 B2B"]
    pub mod B2B6 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable B2B. Wait until delay value defined by TRIG6_COUNTER\\[SAMPLE_INTERVAL\\] is reached"]
            pub const B2B6_0: u32 = 0;
            #[doc = "Enable B2B. When Segment 0 finished (ADC COCO) then automatically trigger next ADC conversion, no need to wait until interval delay reached."]
            pub const B2B6_1: u32 = 0x01;
        }
    }
    #[doc = "Segment 6 interrupt enable. (This bit field is meaningful only when IE6_EN is set)"]
    pub mod IE6 {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Generate interrupt on Done0 when segment 6 finish."]
            pub const IE6_0: u32 = 0;
            #[doc = "Generate interrupt on Done1 when segment 6 finish."]
            pub const IE6_1: u32 = 0x01;
            #[doc = "Generate interrupt on Done2 when segment 6 finish."]
            pub const IE6_2: u32 = 0x02;
            #[doc = "Generate interrupt on Done3 when segment 6 finish."]
            pub const IE6_3: u32 = 0x03;
        }
    }
    #[doc = "IRQ enable of segment 6."]
    pub mod IE6_EN {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt DONE disabled."]
            pub const IE6_EN_0: u32 = 0;
            #[doc = "Interrupt DONE enabled. When segment 6 finish, an interrupt will be generated on the specific port configured by the IE6."]
            pub const IE6_EN_1: u32 = 0x01;
        }
    }
    #[doc = "ADC hardware trigger command selection"]
    pub mod CSEL7 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not a valid selection from the command buffer. Trigger event is ignored."]
            pub const CSEL7_0: u32 = 0;
            #[doc = "ADC CMD1 selected."]
            pub const CSEL7_1: u32 = 0x01;
            #[doc = "ADC CMD2 selected."]
            pub const CSEL7_2: u32 = 0x02;
            #[doc = "ADC CMD3 selected."]
            pub const CSEL7_3: u32 = 0x03;
            #[doc = "ADC CMD4 selected."]
            pub const CSEL7_4: u32 = 0x04;
            #[doc = "ADC CMD5 selected."]
            pub const CSEL7_5: u32 = 0x05;
            #[doc = "ADC CMD6 selected."]
            pub const CSEL7_6: u32 = 0x06;
            #[doc = "ADC CMD7 selected."]
            pub const CSEL7_7: u32 = 0x07;
            #[doc = "ADC CMD8 selected."]
            pub const CSEL7_8: u32 = 0x08;
            #[doc = "ADC CMD9 selected."]
            pub const CSEL7_9: u32 = 0x09;
            #[doc = "ADC CMD10 selected."]
            pub const CSEL7_10: u32 = 0x0a;
            #[doc = "ADC CMD11 selected."]
            pub const CSEL7_11: u32 = 0x0b;
            #[doc = "ADC CMD12 selected."]
            pub const CSEL7_12: u32 = 0x0c;
            #[doc = "ADC CMD13 selected."]
            pub const CSEL7_13: u32 = 0x0d;
            #[doc = "ADC CMD14 selected."]
            pub const CSEL7_14: u32 = 0x0e;
            #[doc = "ADC CMD15 selected."]
            pub const CSEL7_15: u32 = 0x0f;
        }
    }
    #[doc = "Segment 7 HWTS ADC hardware trigger selection"]
    pub mod HWTS7 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "no trigger selected"]
            pub const HWTS7_0: u32 = 0;
            #[doc = "ADC TRIG0 selected"]
            pub const HWTS7_1: u32 = 0x01;
            #[doc = "ADC TRIG1 selected"]
            pub const HWTS7_2: u32 = 0x02;
            #[doc = "ADC TRIG2 selected"]
            pub const HWTS7_4: u32 = 0x04;
            #[doc = "ADC TRIG3 selected"]
            pub const HWTS7_8: u32 = 0x08;
            #[doc = "ADC TRIG4 selected"]
            pub const HWTS7_16: u32 = 0x10;
            #[doc = "ADC TRIG5 selected"]
            pub const HWTS7_32: u32 = 0x20;
            #[doc = "ADC TRIG6 selected"]
            pub const HWTS7_64: u32 = 0x40;
            #[doc = "ADC TRIG7 selected"]
            pub const HWTS7_128: u32 = 0x80;
        }
    }
    #[doc = "Segment 7 B2B"]
    pub mod B2B7 {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable B2B. Wait until delay value defined by TRIG7_COUNTER\\[SAMPLE_INTERVAL\\] is reached"]
            pub const B2B7_0: u32 = 0;
            #[doc = "Enable B2B. When Segment 0 finished (ADC COCO) then automatically trigger next ADC conversion, no need to wait until interval delay reached."]
            pub const B2B7_1: u32 = 0x01;
        }
    }
    #[doc = "Segment 7 interrupt enable. (This bit field is meaningful only when IE7_EN is set)"]
    pub mod IE7 {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Generate interrupt on Done0 when segment 7 finish."]
            pub const IE7_0: u32 = 0;
            #[doc = "Generate interrupt on Done1 when segment 7 finish."]
            pub const IE7_1: u32 = 0x01;
            #[doc = "Generate interrupt on Done2 when segment 7 finish."]
            pub const IE7_2: u32 = 0x02;
            #[doc = "Generate interrupt on Done3 when segment 7 finish."]
            pub const IE7_3: u32 = 0x03;
        }
    }
    #[doc = "IRQ enable of segment 7."]
    pub mod IE7_EN {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt DONE disabled."]
            pub const IE7_EN_0: u32 = 0;
            #[doc = "Interrupt DONE enabled. When segment 7 finish, an interrupt will be generated on the specific port configured by the IE7."]
            pub const IE7_EN_1: u32 = 0x01;
        }
    }
}
#[doc = "ETC_TRIG Result Data 1/0 Register"]
pub mod TRIG2_RESULT_1_0 {
    #[doc = "Result DATA0"]
    pub mod DATA0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Result DATA1"]
    pub mod DATA1 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ETC_TRIG Result Data 3/2 Register"]
pub mod TRIG2_RESULT_3_2 {
    #[doc = "Result DATA2"]
    pub mod DATA2 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Result DATA3"]
    pub mod DATA3 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ETC_TRIG Result Data 5/4 Register"]
pub mod TRIG2_RESULT_5_4 {
    #[doc = "Result DATA4"]
    pub mod DATA4 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Result DATA5"]
    pub mod DATA5 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ETC_TRIG Result Data 7/6 Register"]
pub mod TRIG2_RESULT_7_6 {
    #[doc = "Result DATA6"]
    pub mod DATA6 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Result DATA7"]
    pub mod DATA7 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ETC_TRIG Control Register"]
pub mod TRIG3_CTRL {
    #[doc = "Software trigger. This field is self-clearing."]
    pub mod SW_TRIG {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No software trigger event generated."]
            pub const SW_TRIG_0: u32 = 0;
            #[doc = "Software trigger event generated."]
            pub const SW_TRIG_1: u32 = 0x01;
        }
    }
    #[doc = "Trigger mode selection."]
    pub mod TRIG_MODE {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Hardware trigger. The softerware trigger will be ignored."]
            pub const TRIG_MODE_0: u32 = 0;
            #[doc = "Software trigger. The hardware trigger will be ignored."]
            pub const TRIG_MODE_1: u32 = 0x01;
        }
    }
    #[doc = "The number of segments inside the trigger chain of TRIGa."]
    pub mod TRIG_CHAIN {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Trigger chain length is 1"]
            pub const TRIG_CHAIN_0: u32 = 0;
            #[doc = "Trigger chain length is 2"]
            pub const TRIG_CHAIN_1: u32 = 0x01;
            #[doc = "Trigger chain length is 3"]
            pub const TRIG_CHAIN_2: u32 = 0x02;
            #[doc = "Trigger chain length is 4"]
            pub const TRIG_CHAIN_3: u32 = 0x03;
            #[doc = "Trigger chain length is 5"]
            pub const TRIG_CHAIN_4: u32 = 0x04;
            #[doc = "Trigger chain length is 6"]
            pub const TRIG_CHAIN_5: u32 = 0x05;
            #[doc = "Trigger chain length is 7"]
            pub const TRIG_CHAIN_6: u32 = 0x06;
            #[doc = "Trigger chain length is 8"]
            pub const TRIG_CHAIN_7: u32 = 0x07;
        }
    }
    #[doc = "External trigger priority, 7 is highest priority, while 0 is lowest"]
    pub mod TRIG_PRIORITY {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Trigger synchronization mode selection"]
    pub mod SYNC_MODE {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Synchronization mode disabled, TRIGa and TRIG(a+4) are triggered independently."]
            pub const SYNC_MODE_0: u32 = 0;
            #[doc = "Synchronization mode enabled, TRIGa and TRIG(a+4) are triggered by TRIGa source synchronously."]
            pub const SYNC_MODE_1: u32 = 0x01;
        }
    }
    #[doc = "Segment x done detection"]
    pub mod CHAINX_DONE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "segment x done not detected."]
            pub const CHAINX_DONE_0: u32 = 0;
            #[doc = "segment x done detected."]
            pub const CHAINX_DONE_1: u32 = 0x01;
        }
    }
}
#[doc = "ETC_TRIG Counter Register"]
pub mod TRIG3_COUNTER {
    #[doc = "TRIGGER initial delay counter"]
    pub mod INIT_DELAY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TRIGGER sampling interval counter"]
    pub mod SAMPLE_INTERVAL {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ETC_TRIG Chain 0/1 Register"]
pub mod TRIG3_CHAIN_1_0 {
    #[doc = "ADC hardware trigger command selection"]
    pub mod CSEL0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not a valid selection from the command buffer. Trigger event is ignored."]
            pub const CSEL0_0: u32 = 0;
            #[doc = "ADC CMD1 selected."]
            pub const CSEL0_1: u32 = 0x01;
            #[doc = "ADC CMD2 selected."]
            pub const CSEL0_2: u32 = 0x02;
            #[doc = "ADC CMD3 selected."]
            pub const CSEL0_3: u32 = 0x03;
            #[doc = "ADC CMD4 selected."]
            pub const CSEL0_4: u32 = 0x04;
            #[doc = "ADC CMD5 selected."]
            pub const CSEL0_5: u32 = 0x05;
            #[doc = "ADC CMD6 selected."]
            pub const CSEL0_6: u32 = 0x06;
            #[doc = "ADC CMD7 selected."]
            pub const CSEL0_7: u32 = 0x07;
            #[doc = "ADC CMD8 selected."]
            pub const CSEL0_8: u32 = 0x08;
            #[doc = "ADC CMD9 selected."]
            pub const CSEL0_9: u32 = 0x09;
            #[doc = "ADC CMD10 selected."]
            pub const CSEL0_10: u32 = 0x0a;
            #[doc = "ADC CMD11 selected."]
            pub const CSEL0_11: u32 = 0x0b;
            #[doc = "ADC CMD12 selected."]
            pub const CSEL0_12: u32 = 0x0c;
            #[doc = "ADC CMD13 selected."]
            pub const CSEL0_13: u32 = 0x0d;
            #[doc = "ADC CMD14 selected."]
            pub const CSEL0_14: u32 = 0x0e;
            #[doc = "ADC CMD15 selected."]
            pub const CSEL0_15: u32 = 0x0f;
        }
    }
    #[doc = "Segment 0 HWTS ADC hardware trigger selection"]
    pub mod HWTS0 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "no trigger selected"]
            pub const HWTS0_0: u32 = 0;
            #[doc = "ADC TRIG0 selected"]
            pub const HWTS0_1: u32 = 0x01;
            #[doc = "ADC TRIG1 selected"]
            pub const HWTS0_2: u32 = 0x02;
            #[doc = "ADC TRIG2 selected"]
            pub const HWTS0_4: u32 = 0x04;
            #[doc = "ADC TRIG3 selected"]
            pub const HWTS0_8: u32 = 0x08;
            #[doc = "ADC TRIG4 selected"]
            pub const HWTS0_16: u32 = 0x10;
            #[doc = "ADC TRIG5 selected"]
            pub const HWTS0_32: u32 = 0x20;
            #[doc = "ADC TRIG6 selected"]
            pub const HWTS0_64: u32 = 0x40;
            #[doc = "ADC TRIG7 selected"]
            pub const HWTS0_128: u32 = 0x80;
        }
    }
    #[doc = "Segment 0 B2B"]
    pub mod B2B0 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable B2B. Wait until delay value defined by TRIG0_COUNTER\\[SAMPLE_INTERVAL\\] is reached"]
            pub const B2B0_0: u32 = 0;
            #[doc = "Enable B2B. When Segment 0 finished (ADC COCO) then automatically trigger next ADC conversion, no need to wait until interval delay reached."]
            pub const B2B0_1: u32 = 0x01;
        }
    }
    #[doc = "Segment 0 interrupt enable. (This bit field is meaningful only when IE0_EN is set)"]
    pub mod IE0 {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Generate interrupt on Done0 when segment 0 finish."]
            pub const IE0_0: u32 = 0;
            #[doc = "Generate interrupt on Done1 when segment 0 finish."]
            pub const IE0_1: u32 = 0x01;
            #[doc = "Generate interrupt on Done2 when segment 0 finish."]
            pub const IE0_2: u32 = 0x02;
            #[doc = "Generate interrupt on Done3 when segment 0 finish."]
            pub const IE0_3: u32 = 0x03;
        }
    }
    #[doc = "IRQ enable of segment 0."]
    pub mod IE0_EN {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt DONE disabled."]
            pub const IE0_EN_0: u32 = 0;
            #[doc = "Interrupt DONE enabled. When segment 0 finish, an interrupt will be generated on the specific port configured by the IE0."]
            pub const IE0_EN_1: u32 = 0x01;
        }
    }
    #[doc = "ADC hardware trigger command selection"]
    pub mod CSEL1 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not a valid selection from the command buffer. Trigger event is ignored."]
            pub const CSEL1_0: u32 = 0;
            #[doc = "ADC CMD1 selected."]
            pub const CSEL1_1: u32 = 0x01;
            #[doc = "ADC CMD2 selected."]
            pub const CSEL1_2: u32 = 0x02;
            #[doc = "ADC CMD3 selected."]
            pub const CSEL1_3: u32 = 0x03;
            #[doc = "ADC CMD4 selected."]
            pub const CSEL1_4: u32 = 0x04;
            #[doc = "ADC CMD5 selected."]
            pub const CSEL1_5: u32 = 0x05;
            #[doc = "ADC CMD6 selected."]
            pub const CSEL1_6: u32 = 0x06;
            #[doc = "ADC CMD7 selected."]
            pub const CSEL1_7: u32 = 0x07;
            #[doc = "ADC CMD8 selected."]
            pub const CSEL1_8: u32 = 0x08;
            #[doc = "ADC CMD9 selected."]
            pub const CSEL1_9: u32 = 0x09;
            #[doc = "ADC CMD10 selected."]
            pub const CSEL1_10: u32 = 0x0a;
            #[doc = "ADC CMD11 selected."]
            pub const CSEL1_11: u32 = 0x0b;
            #[doc = "ADC CMD12 selected."]
            pub const CSEL1_12: u32 = 0x0c;
            #[doc = "ADC CMD13 selected."]
            pub const CSEL1_13: u32 = 0x0d;
            #[doc = "ADC CMD14 selected."]
            pub const CSEL1_14: u32 = 0x0e;
            #[doc = "ADC CMD15 selected."]
            pub const CSEL1_15: u32 = 0x0f;
        }
    }
    #[doc = "Segment 1 HWTS ADC hardware trigger selection"]
    pub mod HWTS1 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "no trigger selected"]
            pub const HWTS1_0: u32 = 0;
            #[doc = "ADC TRIG0 selected"]
            pub const HWTS1_1: u32 = 0x01;
            #[doc = "ADC TRIG1 selected"]
            pub const HWTS1_2: u32 = 0x02;
            #[doc = "ADC TRIG2 selected"]
            pub const HWTS1_4: u32 = 0x04;
            #[doc = "ADC TRIG3 selected"]
            pub const HWTS1_8: u32 = 0x08;
            #[doc = "ADC TRIG4 selected"]
            pub const HWTS1_16: u32 = 0x10;
            #[doc = "ADC TRIG5 selected"]
            pub const HWTS1_32: u32 = 0x20;
            #[doc = "ADC TRIG6 selected"]
            pub const HWTS1_64: u32 = 0x40;
            #[doc = "ADC TRIG7 selected"]
            pub const HWTS1_128: u32 = 0x80;
        }
    }
    #[doc = "Segment 1 B2B"]
    pub mod B2B1 {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable B2B. Wait until delay value defined by TRIG1_COUNTER\\[SAMPLE_INTERVAL\\] is reached"]
            pub const B2B1_0: u32 = 0;
            #[doc = "Enable B2B. When Segment 0 finished (ADC COCO) then automatically trigger next ADC conversion, no need to wait until interval delay reached."]
            pub const B2B1_1: u32 = 0x01;
        }
    }
    #[doc = "Segment 1 interrupt enable. (This bit field is meaningful only when IE1_EN is set)"]
    pub mod IE1 {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Generate interrupt on Done0 when Segment 1 finish."]
            pub const IE1_0: u32 = 0;
            #[doc = "Generate interrupt on Done1 when Segment 1 finish."]
            pub const IE1_1: u32 = 0x01;
            #[doc = "Generate interrupt on Done2 when Segment 1 finish."]
            pub const IE1_2: u32 = 0x02;
            #[doc = "Generate interrupt on Done3 when Segment 1 finish."]
            pub const IE1_3: u32 = 0x03;
        }
    }
    #[doc = "IRQ enable of segment 1."]
    pub mod IE1_EN {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt DONE disabled."]
            pub const IE1_EN_0: u32 = 0;
            #[doc = "Interrupt DONE enabled. When segment 1 finish, an interrupt will be generated on the specific port configured by the IE1."]
            pub const IE1_EN_1: u32 = 0x01;
        }
    }
}
#[doc = "ETC_TRIG Chain 2/3 Register"]
pub mod TRIG3_CHAIN_3_2 {
    #[doc = "ADC hardware trigger command selection"]
    pub mod CSEL2 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not a valid selection from the command buffer. Trigger event is ignored."]
            pub const CSEL2_0: u32 = 0;
            #[doc = "ADC CMD1 selected."]
            pub const CSEL2_1: u32 = 0x01;
            #[doc = "ADC CMD2 selected."]
            pub const CSEL2_2: u32 = 0x02;
            #[doc = "ADC CMD3 selected."]
            pub const CSEL2_3: u32 = 0x03;
            #[doc = "ADC CMD4 selected."]
            pub const CSEL2_4: u32 = 0x04;
            #[doc = "ADC CMD5 selected."]
            pub const CSEL2_5: u32 = 0x05;
            #[doc = "ADC CMD6 selected."]
            pub const CSEL2_6: u32 = 0x06;
            #[doc = "ADC CMD7 selected."]
            pub const CSEL2_7: u32 = 0x07;
            #[doc = "ADC CMD8 selected."]
            pub const CSEL2_8: u32 = 0x08;
            #[doc = "ADC CMD9 selected."]
            pub const CSEL2_9: u32 = 0x09;
            #[doc = "ADC CMD10 selected."]
            pub const CSEL2_10: u32 = 0x0a;
            #[doc = "ADC CMD11 selected."]
            pub const CSEL2_11: u32 = 0x0b;
            #[doc = "ADC CMD12 selected."]
            pub const CSEL2_12: u32 = 0x0c;
            #[doc = "ADC CMD13 selected."]
            pub const CSEL2_13: u32 = 0x0d;
            #[doc = "ADC CMD14 selected."]
            pub const CSEL2_14: u32 = 0x0e;
            #[doc = "ADC CMD15 selected."]
            pub const CSEL2_15: u32 = 0x0f;
        }
    }
    #[doc = "Segment 2 HWTS ADC hardware trigger selection"]
    pub mod HWTS2 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "no trigger selected"]
            pub const HWTS2_0: u32 = 0;
            #[doc = "ADC TRIG0 selected"]
            pub const HWTS2_1: u32 = 0x01;
            #[doc = "ADC TRIG1 selected"]
            pub const HWTS2_2: u32 = 0x02;
            #[doc = "ADC TRIG2 selected"]
            pub const HWTS2_4: u32 = 0x04;
            #[doc = "ADC TRIG3 selected"]
            pub const HWTS2_8: u32 = 0x08;
            #[doc = "ADC TRIG4 selected"]
            pub const HWTS2_16: u32 = 0x10;
            #[doc = "ADC TRIG5 selected"]
            pub const HWTS2_32: u32 = 0x20;
            #[doc = "ADC TRIG6 selected"]
            pub const HWTS2_64: u32 = 0x40;
            #[doc = "ADC TRIG7 selected"]
            pub const HWTS2_128: u32 = 0x80;
        }
    }
    #[doc = "Segment 2 B2B"]
    pub mod B2B2 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable B2B. Wait until delay value defined by TRIG2_COUNTER\\[SAMPLE_INTERVAL\\] is reached"]
            pub const B2B2_0: u32 = 0;
            #[doc = "Enable B2B. When Segment 0 finished (ADC COCO) then automatically trigger next ADC conversion, no need to wait until interval delay reached."]
            pub const B2B2_1: u32 = 0x01;
        }
    }
    #[doc = "Segment 2 interrupt enable. (This bit field is meaningful only when IE2_EN is set)"]
    pub mod IE2 {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Generate interrupt on Done0 when segment 2 finish."]
            pub const IE2_0: u32 = 0;
            #[doc = "Generate interrupt on Done1 when segment 2 finish."]
            pub const IE2_1: u32 = 0x01;
            #[doc = "Generate interrupt on Done2 when segment 2 finish."]
            pub const IE2_2: u32 = 0x02;
            #[doc = "Generate interrupt on Done3 when segment 2 finish."]
            pub const IE2_3: u32 = 0x03;
        }
    }
    #[doc = "IRQ enable of segment 2."]
    pub mod IE2_EN {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt DONE disabled."]
            pub const IE2_EN_0: u32 = 0;
            #[doc = "Interrupt DONE enabled. When segment 2 finish, an interrupt will be generated on the specific port configured by the IE2."]
            pub const IE2_EN_1: u32 = 0x01;
        }
    }
    #[doc = "ADC hardware trigger command selection"]
    pub mod CSEL3 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not a valid selection from the command buffer. Trigger event is ignored."]
            pub const CSEL3_0: u32 = 0;
            #[doc = "ADC CMD1 selected."]
            pub const CSEL3_1: u32 = 0x01;
            #[doc = "ADC CMD2 selected."]
            pub const CSEL3_2: u32 = 0x02;
            #[doc = "ADC CMD3 selected."]
            pub const CSEL3_3: u32 = 0x03;
            #[doc = "ADC CMD4 selected."]
            pub const CSEL3_4: u32 = 0x04;
            #[doc = "ADC CMD5 selected."]
            pub const CSEL3_5: u32 = 0x05;
            #[doc = "ADC CMD6 selected."]
            pub const CSEL3_6: u32 = 0x06;
            #[doc = "ADC CMD7 selected."]
            pub const CSEL3_7: u32 = 0x07;
            #[doc = "ADC CMD8 selected."]
            pub const CSEL3_8: u32 = 0x08;
            #[doc = "ADC CMD9 selected."]
            pub const CSEL3_9: u32 = 0x09;
            #[doc = "ADC CMD10 selected."]
            pub const CSEL3_10: u32 = 0x0a;
            #[doc = "ADC CMD11 selected."]
            pub const CSEL3_11: u32 = 0x0b;
            #[doc = "ADC CMD12 selected."]
            pub const CSEL3_12: u32 = 0x0c;
            #[doc = "ADC CMD13 selected."]
            pub const CSEL3_13: u32 = 0x0d;
            #[doc = "ADC CMD14 selected."]
            pub const CSEL3_14: u32 = 0x0e;
            #[doc = "ADC CMD15 selected."]
            pub const CSEL3_15: u32 = 0x0f;
        }
    }
    #[doc = "Segment 3 HWTS ADC hardware trigger selection"]
    pub mod HWTS3 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "no trigger selected"]
            pub const HWTS3_0: u32 = 0;
            #[doc = "ADC TRIG0 selected"]
            pub const HWTS3_1: u32 = 0x01;
            #[doc = "ADC TRIG1 selected"]
            pub const HWTS3_2: u32 = 0x02;
            #[doc = "ADC TRIG2 selected"]
            pub const HWTS3_4: u32 = 0x04;
            #[doc = "ADC TRIG3 selected"]
            pub const HWTS3_8: u32 = 0x08;
            #[doc = "ADC TRIG4 selected"]
            pub const HWTS3_16: u32 = 0x10;
            #[doc = "ADC TRIG5 selected"]
            pub const HWTS3_32: u32 = 0x20;
            #[doc = "ADC TRIG6 selected"]
            pub const HWTS3_64: u32 = 0x40;
            #[doc = "ADC TRIG7 selected"]
            pub const HWTS3_128: u32 = 0x80;
        }
    }
    #[doc = "Segment 3 B2B"]
    pub mod B2B3 {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable B2B. Wait until delay value defined by TRIG3_COUNTER\\[SAMPLE_INTERVAL\\] is reached"]
            pub const B2B3_0: u32 = 0;
            #[doc = "Enable B2B. When Segment 0 finished (ADC COCO) then automatically trigger next ADC conversion, no need to wait until interval delay reached."]
            pub const B2B3_1: u32 = 0x01;
        }
    }
    #[doc = "Segment 3 interrupt enable. (This bit field is meaningful only when IE3_EN is set)"]
    pub mod IE3 {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Generate interrupt on Done0 when segment 3 finish."]
            pub const IE3_0: u32 = 0;
            #[doc = "Generate interrupt on Done1 when segment 3 finish."]
            pub const IE3_1: u32 = 0x01;
            #[doc = "Generate interrupt on Done2 when segment 3 finish."]
            pub const IE3_2: u32 = 0x02;
            #[doc = "Generate interrupt on Done3 when segment 3 finish."]
            pub const IE3_3: u32 = 0x03;
        }
    }
    #[doc = "IRQ enable of segment 3."]
    pub mod IE3_EN {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt DONE disabled."]
            pub const IE3_EN_0: u32 = 0;
            #[doc = "Interrupt DONE enabled. When segment 3 finish, an interrupt will be generated on the specific port configured by the IE3."]
            pub const IE3_EN_1: u32 = 0x01;
        }
    }
}
#[doc = "ETC_TRIG Chain 4/5 Register"]
pub mod TRIG3_CHAIN_5_4 {
    #[doc = "ADC hardware trigger command selection"]
    pub mod CSEL4 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not a valid selection from the command buffer. Trigger event is ignored."]
            pub const CSEL4_0: u32 = 0;
            #[doc = "ADC CMD1 selected."]
            pub const CSEL4_1: u32 = 0x01;
            #[doc = "ADC CMD2 selected."]
            pub const CSEL4_2: u32 = 0x02;
            #[doc = "ADC CMD3 selected."]
            pub const CSEL4_3: u32 = 0x03;
            #[doc = "ADC CMD4 selected."]
            pub const CSEL4_4: u32 = 0x04;
            #[doc = "ADC CMD5 selected."]
            pub const CSEL4_5: u32 = 0x05;
            #[doc = "ADC CMD6 selected."]
            pub const CSEL4_6: u32 = 0x06;
            #[doc = "ADC CMD7 selected."]
            pub const CSEL4_7: u32 = 0x07;
            #[doc = "ADC CMD8 selected."]
            pub const CSEL4_8: u32 = 0x08;
            #[doc = "ADC CMD9 selected."]
            pub const CSEL4_9: u32 = 0x09;
            #[doc = "ADC CMD10 selected."]
            pub const CSEL4_10: u32 = 0x0a;
            #[doc = "ADC CMD11 selected."]
            pub const CSEL4_11: u32 = 0x0b;
            #[doc = "ADC CMD12 selected."]
            pub const CSEL4_12: u32 = 0x0c;
            #[doc = "ADC CMD13 selected."]
            pub const CSEL4_13: u32 = 0x0d;
            #[doc = "ADC CMD14 selected."]
            pub const CSEL4_14: u32 = 0x0e;
            #[doc = "ADC CMD15 selected."]
            pub const CSEL4_15: u32 = 0x0f;
        }
    }
    #[doc = "Segment 4 HWTS ADC hardware trigger selection"]
    pub mod HWTS4 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "no trigger selected"]
            pub const HWTS4_0: u32 = 0;
            #[doc = "ADC TRIG0 selected"]
            pub const HWTS4_1: u32 = 0x01;
            #[doc = "ADC TRIG1 selected"]
            pub const HWTS4_2: u32 = 0x02;
            #[doc = "ADC TRIG2 selected"]
            pub const HWTS4_4: u32 = 0x04;
            #[doc = "ADC TRIG3 selected"]
            pub const HWTS4_8: u32 = 0x08;
            #[doc = "ADC TRIG4 selected"]
            pub const HWTS4_16: u32 = 0x10;
            #[doc = "ADC TRIG5 selected"]
            pub const HWTS4_32: u32 = 0x20;
            #[doc = "ADC TRIG6 selected"]
            pub const HWTS4_64: u32 = 0x40;
            #[doc = "ADC TRIG7 selected"]
            pub const HWTS4_128: u32 = 0x80;
        }
    }
    #[doc = "Segment 4 B2B"]
    pub mod B2B4 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable B2B. Wait until delay value defined by TRIG4_COUNTER\\[SAMPLE_INTERVAL\\] is reached"]
            pub const B2B4_0: u32 = 0;
            #[doc = "Enable B2B. When Segment 0 finished (ADC COCO) then automatically trigger next ADC conversion, no need to wait until interval delay reached."]
            pub const B2B4_1: u32 = 0x01;
        }
    }
    #[doc = "Segment 4 interrupt enable. (This bit field is meaningful only when IE4_EN is set)"]
    pub mod IE4 {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Generate interrupt on Done0 when segment 4 finish."]
            pub const IE4_0: u32 = 0;
            #[doc = "Generate interrupt on Done1 when segment 4 finish."]
            pub const IE4_1: u32 = 0x01;
            #[doc = "Generate interrupt on Done2 when segment 4 finish."]
            pub const IE4_2: u32 = 0x02;
            #[doc = "Generate interrupt on Done3 when segment 4 finish."]
            pub const IE4_3: u32 = 0x03;
        }
    }
    #[doc = "IRQ enable of segment 4."]
    pub mod IE4_EN {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt DONE disabled."]
            pub const IE4_EN_0: u32 = 0;
            #[doc = "Interrupt DONE enabled. When segment 4 finish, an interrupt will be generated on the specific port configured by the IE4."]
            pub const IE4_EN_1: u32 = 0x01;
        }
    }
    #[doc = "ADC hardware trigger command selection"]
    pub mod CSEL5 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not a valid selection from the command buffer. Trigger event is ignored."]
            pub const CSEL5_0: u32 = 0;
            #[doc = "ADC CMD1 selected."]
            pub const CSEL5_1: u32 = 0x01;
            #[doc = "ADC CMD2 selected."]
            pub const CSEL5_2: u32 = 0x02;
            #[doc = "ADC CMD3 selected."]
            pub const CSEL5_3: u32 = 0x03;
            #[doc = "ADC CMD4 selected."]
            pub const CSEL5_4: u32 = 0x04;
            #[doc = "ADC CMD5 selected."]
            pub const CSEL5_5: u32 = 0x05;
            #[doc = "ADC CMD6 selected."]
            pub const CSEL5_6: u32 = 0x06;
            #[doc = "ADC CMD7 selected."]
            pub const CSEL5_7: u32 = 0x07;
            #[doc = "ADC CMD8 selected."]
            pub const CSEL5_8: u32 = 0x08;
            #[doc = "ADC CMD9 selected."]
            pub const CSEL5_9: u32 = 0x09;
            #[doc = "ADC CMD10 selected."]
            pub const CSEL5_10: u32 = 0x0a;
            #[doc = "ADC CMD11 selected."]
            pub const CSEL5_11: u32 = 0x0b;
            #[doc = "ADC CMD12 selected."]
            pub const CSEL5_12: u32 = 0x0c;
            #[doc = "ADC CMD13 selected."]
            pub const CSEL5_13: u32 = 0x0d;
            #[doc = "ADC CMD14 selected."]
            pub const CSEL5_14: u32 = 0x0e;
            #[doc = "ADC CMD15 selected."]
            pub const CSEL5_15: u32 = 0x0f;
        }
    }
    #[doc = "Segment 5 HWTS ADC hardware trigger selection"]
    pub mod HWTS5 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "no trigger selected"]
            pub const HWTS5_0: u32 = 0;
            #[doc = "ADC TRIG0 selected"]
            pub const HWTS5_1: u32 = 0x01;
            #[doc = "ADC TRIG1 selected"]
            pub const HWTS5_2: u32 = 0x02;
            #[doc = "ADC TRIG2 selected"]
            pub const HWTS5_4: u32 = 0x04;
            #[doc = "ADC TRIG3 selected"]
            pub const HWTS5_8: u32 = 0x08;
            #[doc = "ADC TRIG4 selected"]
            pub const HWTS5_16: u32 = 0x10;
            #[doc = "ADC TRIG5 selected"]
            pub const HWTS5_32: u32 = 0x20;
            #[doc = "ADC TRIG6 selected"]
            pub const HWTS5_64: u32 = 0x40;
            #[doc = "ADC TRIG7 selected"]
            pub const HWTS5_128: u32 = 0x80;
        }
    }
    #[doc = "Segment 5 B2B"]
    pub mod B2B5 {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable B2B. Wait until delay value defined by TRIG5_COUNTER\\[SAMPLE_INTERVAL\\] is reached"]
            pub const B2B5_0: u32 = 0;
            #[doc = "Enable B2B. When Segment 0 finished (ADC COCO) then automatically trigger next ADC conversion, no need to wait until interval delay reached."]
            pub const B2B5_1: u32 = 0x01;
        }
    }
    #[doc = "Segment 5 interrupt enable. (This bit field is meaningful only when IE5_EN is set)"]
    pub mod IE5 {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Generate interrupt on Done0 when segment 5 finish."]
            pub const IE5_0: u32 = 0;
            #[doc = "Generate interrupt on Done1 when segment 5 finish."]
            pub const IE5_1: u32 = 0x01;
            #[doc = "Generate interrupt on Done2 when segment 5 finish."]
            pub const IE5_2: u32 = 0x02;
            #[doc = "Generate interrupt on Done3 when segment 5 finish."]
            pub const IE5_3: u32 = 0x03;
        }
    }
    #[doc = "IRQ enable of segment 5."]
    pub mod IE5_EN {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt DONE disabled."]
            pub const IE5_EN_0: u32 = 0;
            #[doc = "Interrupt DONE enabled. When segment 5 finish, an interrupt will be generated on the specific port configured by the IE5."]
            pub const IE5_EN_1: u32 = 0x01;
        }
    }
}
#[doc = "ETC_TRIG Chain 6/7 Register"]
pub mod TRIG3_CHAIN_7_6 {
    #[doc = "ADC hardware trigger command selection"]
    pub mod CSEL6 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not a valid selection from the command buffer. Trigger event is ignored."]
            pub const CSEL6_0: u32 = 0;
            #[doc = "ADC CMD1 selected."]
            pub const CSEL6_1: u32 = 0x01;
            #[doc = "ADC CMD2 selected."]
            pub const CSEL6_2: u32 = 0x02;
            #[doc = "ADC CMD3 selected."]
            pub const CSEL6_3: u32 = 0x03;
            #[doc = "ADC CMD4 selected."]
            pub const CSEL6_4: u32 = 0x04;
            #[doc = "ADC CMD5 selected."]
            pub const CSEL6_5: u32 = 0x05;
            #[doc = "ADC CMD6 selected."]
            pub const CSEL6_6: u32 = 0x06;
            #[doc = "ADC CMD7 selected."]
            pub const CSEL6_7: u32 = 0x07;
            #[doc = "ADC CMD8 selected."]
            pub const CSEL6_8: u32 = 0x08;
            #[doc = "ADC CMD9 selected."]
            pub const CSEL6_9: u32 = 0x09;
            #[doc = "ADC CMD10 selected."]
            pub const CSEL6_10: u32 = 0x0a;
            #[doc = "ADC CMD11 selected."]
            pub const CSEL6_11: u32 = 0x0b;
            #[doc = "ADC CMD12 selected."]
            pub const CSEL6_12: u32 = 0x0c;
            #[doc = "ADC CMD13 selected."]
            pub const CSEL6_13: u32 = 0x0d;
            #[doc = "ADC CMD14 selected."]
            pub const CSEL6_14: u32 = 0x0e;
            #[doc = "ADC CMD15 selected."]
            pub const CSEL6_15: u32 = 0x0f;
        }
    }
    #[doc = "Segment 6 HWTS ADC hardware trigger selection"]
    pub mod HWTS6 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "no trigger selected"]
            pub const HWTS6_0: u32 = 0;
            #[doc = "ADC TRIG0 selected"]
            pub const HWTS6_1: u32 = 0x01;
            #[doc = "ADC TRIG1 selected"]
            pub const HWTS6_2: u32 = 0x02;
            #[doc = "ADC TRIG2 selected"]
            pub const HWTS6_4: u32 = 0x04;
            #[doc = "ADC TRIG3 selected"]
            pub const HWTS6_8: u32 = 0x08;
            #[doc = "ADC TRIG4 selected"]
            pub const HWTS6_16: u32 = 0x10;
            #[doc = "ADC TRIG5 selected"]
            pub const HWTS6_32: u32 = 0x20;
            #[doc = "ADC TRIG6 selected"]
            pub const HWTS6_64: u32 = 0x40;
            #[doc = "ADC TRIG7 selected"]
            pub const HWTS6_128: u32 = 0x80;
        }
    }
    #[doc = "Segment 6 B2B"]
    pub mod B2B6 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable B2B. Wait until delay value defined by TRIG6_COUNTER\\[SAMPLE_INTERVAL\\] is reached"]
            pub const B2B6_0: u32 = 0;
            #[doc = "Enable B2B. When Segment 0 finished (ADC COCO) then automatically trigger next ADC conversion, no need to wait until interval delay reached."]
            pub const B2B6_1: u32 = 0x01;
        }
    }
    #[doc = "Segment 6 interrupt enable. (This bit field is meaningful only when IE6_EN is set)"]
    pub mod IE6 {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Generate interrupt on Done0 when segment 6 finish."]
            pub const IE6_0: u32 = 0;
            #[doc = "Generate interrupt on Done1 when segment 6 finish."]
            pub const IE6_1: u32 = 0x01;
            #[doc = "Generate interrupt on Done2 when segment 6 finish."]
            pub const IE6_2: u32 = 0x02;
            #[doc = "Generate interrupt on Done3 when segment 6 finish."]
            pub const IE6_3: u32 = 0x03;
        }
    }
    #[doc = "IRQ enable of segment 6."]
    pub mod IE6_EN {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt DONE disabled."]
            pub const IE6_EN_0: u32 = 0;
            #[doc = "Interrupt DONE enabled. When segment 6 finish, an interrupt will be generated on the specific port configured by the IE6."]
            pub const IE6_EN_1: u32 = 0x01;
        }
    }
    #[doc = "ADC hardware trigger command selection"]
    pub mod CSEL7 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not a valid selection from the command buffer. Trigger event is ignored."]
            pub const CSEL7_0: u32 = 0;
            #[doc = "ADC CMD1 selected."]
            pub const CSEL7_1: u32 = 0x01;
            #[doc = "ADC CMD2 selected."]
            pub const CSEL7_2: u32 = 0x02;
            #[doc = "ADC CMD3 selected."]
            pub const CSEL7_3: u32 = 0x03;
            #[doc = "ADC CMD4 selected."]
            pub const CSEL7_4: u32 = 0x04;
            #[doc = "ADC CMD5 selected."]
            pub const CSEL7_5: u32 = 0x05;
            #[doc = "ADC CMD6 selected."]
            pub const CSEL7_6: u32 = 0x06;
            #[doc = "ADC CMD7 selected."]
            pub const CSEL7_7: u32 = 0x07;
            #[doc = "ADC CMD8 selected."]
            pub const CSEL7_8: u32 = 0x08;
            #[doc = "ADC CMD9 selected."]
            pub const CSEL7_9: u32 = 0x09;
            #[doc = "ADC CMD10 selected."]
            pub const CSEL7_10: u32 = 0x0a;
            #[doc = "ADC CMD11 selected."]
            pub const CSEL7_11: u32 = 0x0b;
            #[doc = "ADC CMD12 selected."]
            pub const CSEL7_12: u32 = 0x0c;
            #[doc = "ADC CMD13 selected."]
            pub const CSEL7_13: u32 = 0x0d;
            #[doc = "ADC CMD14 selected."]
            pub const CSEL7_14: u32 = 0x0e;
            #[doc = "ADC CMD15 selected."]
            pub const CSEL7_15: u32 = 0x0f;
        }
    }
    #[doc = "Segment 7 HWTS ADC hardware trigger selection"]
    pub mod HWTS7 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "no trigger selected"]
            pub const HWTS7_0: u32 = 0;
            #[doc = "ADC TRIG0 selected"]
            pub const HWTS7_1: u32 = 0x01;
            #[doc = "ADC TRIG1 selected"]
            pub const HWTS7_2: u32 = 0x02;
            #[doc = "ADC TRIG2 selected"]
            pub const HWTS7_4: u32 = 0x04;
            #[doc = "ADC TRIG3 selected"]
            pub const HWTS7_8: u32 = 0x08;
            #[doc = "ADC TRIG4 selected"]
            pub const HWTS7_16: u32 = 0x10;
            #[doc = "ADC TRIG5 selected"]
            pub const HWTS7_32: u32 = 0x20;
            #[doc = "ADC TRIG6 selected"]
            pub const HWTS7_64: u32 = 0x40;
            #[doc = "ADC TRIG7 selected"]
            pub const HWTS7_128: u32 = 0x80;
        }
    }
    #[doc = "Segment 7 B2B"]
    pub mod B2B7 {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable B2B. Wait until delay value defined by TRIG7_COUNTER\\[SAMPLE_INTERVAL\\] is reached"]
            pub const B2B7_0: u32 = 0;
            #[doc = "Enable B2B. When Segment 0 finished (ADC COCO) then automatically trigger next ADC conversion, no need to wait until interval delay reached."]
            pub const B2B7_1: u32 = 0x01;
        }
    }
    #[doc = "Segment 7 interrupt enable. (This bit field is meaningful only when IE7_EN is set)"]
    pub mod IE7 {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Generate interrupt on Done0 when segment 7 finish."]
            pub const IE7_0: u32 = 0;
            #[doc = "Generate interrupt on Done1 when segment 7 finish."]
            pub const IE7_1: u32 = 0x01;
            #[doc = "Generate interrupt on Done2 when segment 7 finish."]
            pub const IE7_2: u32 = 0x02;
            #[doc = "Generate interrupt on Done3 when segment 7 finish."]
            pub const IE7_3: u32 = 0x03;
        }
    }
    #[doc = "IRQ enable of segment 7."]
    pub mod IE7_EN {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt DONE disabled."]
            pub const IE7_EN_0: u32 = 0;
            #[doc = "Interrupt DONE enabled. When segment 7 finish, an interrupt will be generated on the specific port configured by the IE7."]
            pub const IE7_EN_1: u32 = 0x01;
        }
    }
}
#[doc = "ETC_TRIG Result Data 1/0 Register"]
pub mod TRIG3_RESULT_1_0 {
    #[doc = "Result DATA0"]
    pub mod DATA0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Result DATA1"]
    pub mod DATA1 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ETC_TRIG Result Data 3/2 Register"]
pub mod TRIG3_RESULT_3_2 {
    #[doc = "Result DATA2"]
    pub mod DATA2 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Result DATA3"]
    pub mod DATA3 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ETC_TRIG Result Data 5/4 Register"]
pub mod TRIG3_RESULT_5_4 {
    #[doc = "Result DATA4"]
    pub mod DATA4 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Result DATA5"]
    pub mod DATA5 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ETC_TRIG Result Data 7/6 Register"]
pub mod TRIG3_RESULT_7_6 {
    #[doc = "Result DATA6"]
    pub mod DATA6 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Result DATA7"]
    pub mod DATA7 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ETC_TRIG Control Register"]
pub mod TRIG4_CTRL {
    #[doc = "Software trigger. This field is self-clearing."]
    pub mod SW_TRIG {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No software trigger event generated."]
            pub const SW_TRIG_0: u32 = 0;
            #[doc = "Software trigger event generated."]
            pub const SW_TRIG_1: u32 = 0x01;
        }
    }
    #[doc = "Trigger mode selection."]
    pub mod TRIG_MODE {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Hardware trigger. The softerware trigger will be ignored."]
            pub const TRIG_MODE_0: u32 = 0;
            #[doc = "Software trigger. The hardware trigger will be ignored."]
            pub const TRIG_MODE_1: u32 = 0x01;
        }
    }
    #[doc = "The number of segments inside the trigger chain of TRIGa."]
    pub mod TRIG_CHAIN {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Trigger chain length is 1"]
            pub const TRIG_CHAIN_0: u32 = 0;
            #[doc = "Trigger chain length is 2"]
            pub const TRIG_CHAIN_1: u32 = 0x01;
            #[doc = "Trigger chain length is 3"]
            pub const TRIG_CHAIN_2: u32 = 0x02;
            #[doc = "Trigger chain length is 4"]
            pub const TRIG_CHAIN_3: u32 = 0x03;
            #[doc = "Trigger chain length is 5"]
            pub const TRIG_CHAIN_4: u32 = 0x04;
            #[doc = "Trigger chain length is 6"]
            pub const TRIG_CHAIN_5: u32 = 0x05;
            #[doc = "Trigger chain length is 7"]
            pub const TRIG_CHAIN_6: u32 = 0x06;
            #[doc = "Trigger chain length is 8"]
            pub const TRIG_CHAIN_7: u32 = 0x07;
        }
    }
    #[doc = "External trigger priority, 7 is highest priority, while 0 is lowest"]
    pub mod TRIG_PRIORITY {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Trigger synchronization mode selection"]
    pub mod SYNC_MODE {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Synchronization mode disabled, TRIGa and TRIG(a+4) are triggered independently."]
            pub const SYNC_MODE_0: u32 = 0;
            #[doc = "Synchronization mode enabled, TRIGa and TRIG(a+4) are triggered by TRIGa source synchronously."]
            pub const SYNC_MODE_1: u32 = 0x01;
        }
    }
    #[doc = "Segment x done detection"]
    pub mod CHAINX_DONE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "segment x done not detected."]
            pub const CHAINX_DONE_0: u32 = 0;
            #[doc = "segment x done detected."]
            pub const CHAINX_DONE_1: u32 = 0x01;
        }
    }
}
#[doc = "ETC_TRIG Counter Register"]
pub mod TRIG4_COUNTER {
    #[doc = "TRIGGER initial delay counter"]
    pub mod INIT_DELAY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TRIGGER sampling interval counter"]
    pub mod SAMPLE_INTERVAL {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ETC_TRIG Chain 0/1 Register"]
pub mod TRIG4_CHAIN_1_0 {
    #[doc = "ADC hardware trigger command selection"]
    pub mod CSEL0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not a valid selection from the command buffer. Trigger event is ignored."]
            pub const CSEL0_0: u32 = 0;
            #[doc = "ADC CMD1 selected."]
            pub const CSEL0_1: u32 = 0x01;
            #[doc = "ADC CMD2 selected."]
            pub const CSEL0_2: u32 = 0x02;
            #[doc = "ADC CMD3 selected."]
            pub const CSEL0_3: u32 = 0x03;
            #[doc = "ADC CMD4 selected."]
            pub const CSEL0_4: u32 = 0x04;
            #[doc = "ADC CMD5 selected."]
            pub const CSEL0_5: u32 = 0x05;
            #[doc = "ADC CMD6 selected."]
            pub const CSEL0_6: u32 = 0x06;
            #[doc = "ADC CMD7 selected."]
            pub const CSEL0_7: u32 = 0x07;
            #[doc = "ADC CMD8 selected."]
            pub const CSEL0_8: u32 = 0x08;
            #[doc = "ADC CMD9 selected."]
            pub const CSEL0_9: u32 = 0x09;
            #[doc = "ADC CMD10 selected."]
            pub const CSEL0_10: u32 = 0x0a;
            #[doc = "ADC CMD11 selected."]
            pub const CSEL0_11: u32 = 0x0b;
            #[doc = "ADC CMD12 selected."]
            pub const CSEL0_12: u32 = 0x0c;
            #[doc = "ADC CMD13 selected."]
            pub const CSEL0_13: u32 = 0x0d;
            #[doc = "ADC CMD14 selected."]
            pub const CSEL0_14: u32 = 0x0e;
            #[doc = "ADC CMD15 selected."]
            pub const CSEL0_15: u32 = 0x0f;
        }
    }
    #[doc = "Segment 0 HWTS ADC hardware trigger selection"]
    pub mod HWTS0 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "no trigger selected"]
            pub const HWTS0_0: u32 = 0;
            #[doc = "ADC TRIG0 selected"]
            pub const HWTS0_1: u32 = 0x01;
            #[doc = "ADC TRIG1 selected"]
            pub const HWTS0_2: u32 = 0x02;
            #[doc = "ADC TRIG2 selected"]
            pub const HWTS0_4: u32 = 0x04;
            #[doc = "ADC TRIG3 selected"]
            pub const HWTS0_8: u32 = 0x08;
            #[doc = "ADC TRIG4 selected"]
            pub const HWTS0_16: u32 = 0x10;
            #[doc = "ADC TRIG5 selected"]
            pub const HWTS0_32: u32 = 0x20;
            #[doc = "ADC TRIG6 selected"]
            pub const HWTS0_64: u32 = 0x40;
            #[doc = "ADC TRIG7 selected"]
            pub const HWTS0_128: u32 = 0x80;
        }
    }
    #[doc = "Segment 0 B2B"]
    pub mod B2B0 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable B2B. Wait until delay value defined by TRIG0_COUNTER\\[SAMPLE_INTERVAL\\] is reached"]
            pub const B2B0_0: u32 = 0;
            #[doc = "Enable B2B. When Segment 0 finished (ADC COCO) then automatically trigger next ADC conversion, no need to wait until interval delay reached."]
            pub const B2B0_1: u32 = 0x01;
        }
    }
    #[doc = "Segment 0 interrupt enable. (This bit field is meaningful only when IE0_EN is set)"]
    pub mod IE0 {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Generate interrupt on Done0 when segment 0 finish."]
            pub const IE0_0: u32 = 0;
            #[doc = "Generate interrupt on Done1 when segment 0 finish."]
            pub const IE0_1: u32 = 0x01;
            #[doc = "Generate interrupt on Done2 when segment 0 finish."]
            pub const IE0_2: u32 = 0x02;
            #[doc = "Generate interrupt on Done3 when segment 0 finish."]
            pub const IE0_3: u32 = 0x03;
        }
    }
    #[doc = "IRQ enable of segment 0."]
    pub mod IE0_EN {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt DONE disabled."]
            pub const IE0_EN_0: u32 = 0;
            #[doc = "Interrupt DONE enabled. When segment 0 finish, an interrupt will be generated on the specific port configured by the IE0."]
            pub const IE0_EN_1: u32 = 0x01;
        }
    }
    #[doc = "ADC hardware trigger command selection"]
    pub mod CSEL1 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not a valid selection from the command buffer. Trigger event is ignored."]
            pub const CSEL1_0: u32 = 0;
            #[doc = "ADC CMD1 selected."]
            pub const CSEL1_1: u32 = 0x01;
            #[doc = "ADC CMD2 selected."]
            pub const CSEL1_2: u32 = 0x02;
            #[doc = "ADC CMD3 selected."]
            pub const CSEL1_3: u32 = 0x03;
            #[doc = "ADC CMD4 selected."]
            pub const CSEL1_4: u32 = 0x04;
            #[doc = "ADC CMD5 selected."]
            pub const CSEL1_5: u32 = 0x05;
            #[doc = "ADC CMD6 selected."]
            pub const CSEL1_6: u32 = 0x06;
            #[doc = "ADC CMD7 selected."]
            pub const CSEL1_7: u32 = 0x07;
            #[doc = "ADC CMD8 selected."]
            pub const CSEL1_8: u32 = 0x08;
            #[doc = "ADC CMD9 selected."]
            pub const CSEL1_9: u32 = 0x09;
            #[doc = "ADC CMD10 selected."]
            pub const CSEL1_10: u32 = 0x0a;
            #[doc = "ADC CMD11 selected."]
            pub const CSEL1_11: u32 = 0x0b;
            #[doc = "ADC CMD12 selected."]
            pub const CSEL1_12: u32 = 0x0c;
            #[doc = "ADC CMD13 selected."]
            pub const CSEL1_13: u32 = 0x0d;
            #[doc = "ADC CMD14 selected."]
            pub const CSEL1_14: u32 = 0x0e;
            #[doc = "ADC CMD15 selected."]
            pub const CSEL1_15: u32 = 0x0f;
        }
    }
    #[doc = "Segment 1 HWTS ADC hardware trigger selection"]
    pub mod HWTS1 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "no trigger selected"]
            pub const HWTS1_0: u32 = 0;
            #[doc = "ADC TRIG0 selected"]
            pub const HWTS1_1: u32 = 0x01;
            #[doc = "ADC TRIG1 selected"]
            pub const HWTS1_2: u32 = 0x02;
            #[doc = "ADC TRIG2 selected"]
            pub const HWTS1_4: u32 = 0x04;
            #[doc = "ADC TRIG3 selected"]
            pub const HWTS1_8: u32 = 0x08;
            #[doc = "ADC TRIG4 selected"]
            pub const HWTS1_16: u32 = 0x10;
            #[doc = "ADC TRIG5 selected"]
            pub const HWTS1_32: u32 = 0x20;
            #[doc = "ADC TRIG6 selected"]
            pub const HWTS1_64: u32 = 0x40;
            #[doc = "ADC TRIG7 selected"]
            pub const HWTS1_128: u32 = 0x80;
        }
    }
    #[doc = "Segment 1 B2B"]
    pub mod B2B1 {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable B2B. Wait until delay value defined by TRIG1_COUNTER\\[SAMPLE_INTERVAL\\] is reached"]
            pub const B2B1_0: u32 = 0;
            #[doc = "Enable B2B. When Segment 0 finished (ADC COCO) then automatically trigger next ADC conversion, no need to wait until interval delay reached."]
            pub const B2B1_1: u32 = 0x01;
        }
    }
    #[doc = "Segment 1 interrupt enable. (This bit field is meaningful only when IE1_EN is set)"]
    pub mod IE1 {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Generate interrupt on Done0 when Segment 1 finish."]
            pub const IE1_0: u32 = 0;
            #[doc = "Generate interrupt on Done1 when Segment 1 finish."]
            pub const IE1_1: u32 = 0x01;
            #[doc = "Generate interrupt on Done2 when Segment 1 finish."]
            pub const IE1_2: u32 = 0x02;
            #[doc = "Generate interrupt on Done3 when Segment 1 finish."]
            pub const IE1_3: u32 = 0x03;
        }
    }
    #[doc = "IRQ enable of segment 1."]
    pub mod IE1_EN {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt DONE disabled."]
            pub const IE1_EN_0: u32 = 0;
            #[doc = "Interrupt DONE enabled. When segment 1 finish, an interrupt will be generated on the specific port configured by the IE1."]
            pub const IE1_EN_1: u32 = 0x01;
        }
    }
}
#[doc = "ETC_TRIG Chain 2/3 Register"]
pub mod TRIG4_CHAIN_3_2 {
    #[doc = "ADC hardware trigger command selection"]
    pub mod CSEL2 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not a valid selection from the command buffer. Trigger event is ignored."]
            pub const CSEL2_0: u32 = 0;
            #[doc = "ADC CMD1 selected."]
            pub const CSEL2_1: u32 = 0x01;
            #[doc = "ADC CMD2 selected."]
            pub const CSEL2_2: u32 = 0x02;
            #[doc = "ADC CMD3 selected."]
            pub const CSEL2_3: u32 = 0x03;
            #[doc = "ADC CMD4 selected."]
            pub const CSEL2_4: u32 = 0x04;
            #[doc = "ADC CMD5 selected."]
            pub const CSEL2_5: u32 = 0x05;
            #[doc = "ADC CMD6 selected."]
            pub const CSEL2_6: u32 = 0x06;
            #[doc = "ADC CMD7 selected."]
            pub const CSEL2_7: u32 = 0x07;
            #[doc = "ADC CMD8 selected."]
            pub const CSEL2_8: u32 = 0x08;
            #[doc = "ADC CMD9 selected."]
            pub const CSEL2_9: u32 = 0x09;
            #[doc = "ADC CMD10 selected."]
            pub const CSEL2_10: u32 = 0x0a;
            #[doc = "ADC CMD11 selected."]
            pub const CSEL2_11: u32 = 0x0b;
            #[doc = "ADC CMD12 selected."]
            pub const CSEL2_12: u32 = 0x0c;
            #[doc = "ADC CMD13 selected."]
            pub const CSEL2_13: u32 = 0x0d;
            #[doc = "ADC CMD14 selected."]
            pub const CSEL2_14: u32 = 0x0e;
            #[doc = "ADC CMD15 selected."]
            pub const CSEL2_15: u32 = 0x0f;
        }
    }
    #[doc = "Segment 2 HWTS ADC hardware trigger selection"]
    pub mod HWTS2 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "no trigger selected"]
            pub const HWTS2_0: u32 = 0;
            #[doc = "ADC TRIG0 selected"]
            pub const HWTS2_1: u32 = 0x01;
            #[doc = "ADC TRIG1 selected"]
            pub const HWTS2_2: u32 = 0x02;
            #[doc = "ADC TRIG2 selected"]
            pub const HWTS2_4: u32 = 0x04;
            #[doc = "ADC TRIG3 selected"]
            pub const HWTS2_8: u32 = 0x08;
            #[doc = "ADC TRIG4 selected"]
            pub const HWTS2_16: u32 = 0x10;
            #[doc = "ADC TRIG5 selected"]
            pub const HWTS2_32: u32 = 0x20;
            #[doc = "ADC TRIG6 selected"]
            pub const HWTS2_64: u32 = 0x40;
            #[doc = "ADC TRIG7 selected"]
            pub const HWTS2_128: u32 = 0x80;
        }
    }
    #[doc = "Segment 2 B2B"]
    pub mod B2B2 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable B2B. Wait until delay value defined by TRIG2_COUNTER\\[SAMPLE_INTERVAL\\] is reached"]
            pub const B2B2_0: u32 = 0;
            #[doc = "Enable B2B. When Segment 0 finished (ADC COCO) then automatically trigger next ADC conversion, no need to wait until interval delay reached."]
            pub const B2B2_1: u32 = 0x01;
        }
    }
    #[doc = "Segment 2 interrupt enable. (This bit field is meaningful only when IE2_EN is set)"]
    pub mod IE2 {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Generate interrupt on Done0 when segment 2 finish."]
            pub const IE2_0: u32 = 0;
            #[doc = "Generate interrupt on Done1 when segment 2 finish."]
            pub const IE2_1: u32 = 0x01;
            #[doc = "Generate interrupt on Done2 when segment 2 finish."]
            pub const IE2_2: u32 = 0x02;
            #[doc = "Generate interrupt on Done3 when segment 2 finish."]
            pub const IE2_3: u32 = 0x03;
        }
    }
    #[doc = "IRQ enable of segment 2."]
    pub mod IE2_EN {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt DONE disabled."]
            pub const IE2_EN_0: u32 = 0;
            #[doc = "Interrupt DONE enabled. When segment 2 finish, an interrupt will be generated on the specific port configured by the IE2."]
            pub const IE2_EN_1: u32 = 0x01;
        }
    }
    #[doc = "ADC hardware trigger command selection"]
    pub mod CSEL3 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not a valid selection from the command buffer. Trigger event is ignored."]
            pub const CSEL3_0: u32 = 0;
            #[doc = "ADC CMD1 selected."]
            pub const CSEL3_1: u32 = 0x01;
            #[doc = "ADC CMD2 selected."]
            pub const CSEL3_2: u32 = 0x02;
            #[doc = "ADC CMD3 selected."]
            pub const CSEL3_3: u32 = 0x03;
            #[doc = "ADC CMD4 selected."]
            pub const CSEL3_4: u32 = 0x04;
            #[doc = "ADC CMD5 selected."]
            pub const CSEL3_5: u32 = 0x05;
            #[doc = "ADC CMD6 selected."]
            pub const CSEL3_6: u32 = 0x06;
            #[doc = "ADC CMD7 selected."]
            pub const CSEL3_7: u32 = 0x07;
            #[doc = "ADC CMD8 selected."]
            pub const CSEL3_8: u32 = 0x08;
            #[doc = "ADC CMD9 selected."]
            pub const CSEL3_9: u32 = 0x09;
            #[doc = "ADC CMD10 selected."]
            pub const CSEL3_10: u32 = 0x0a;
            #[doc = "ADC CMD11 selected."]
            pub const CSEL3_11: u32 = 0x0b;
            #[doc = "ADC CMD12 selected."]
            pub const CSEL3_12: u32 = 0x0c;
            #[doc = "ADC CMD13 selected."]
            pub const CSEL3_13: u32 = 0x0d;
            #[doc = "ADC CMD14 selected."]
            pub const CSEL3_14: u32 = 0x0e;
            #[doc = "ADC CMD15 selected."]
            pub const CSEL3_15: u32 = 0x0f;
        }
    }
    #[doc = "Segment 3 HWTS ADC hardware trigger selection"]
    pub mod HWTS3 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "no trigger selected"]
            pub const HWTS3_0: u32 = 0;
            #[doc = "ADC TRIG0 selected"]
            pub const HWTS3_1: u32 = 0x01;
            #[doc = "ADC TRIG1 selected"]
            pub const HWTS3_2: u32 = 0x02;
            #[doc = "ADC TRIG2 selected"]
            pub const HWTS3_4: u32 = 0x04;
            #[doc = "ADC TRIG3 selected"]
            pub const HWTS3_8: u32 = 0x08;
            #[doc = "ADC TRIG4 selected"]
            pub const HWTS3_16: u32 = 0x10;
            #[doc = "ADC TRIG5 selected"]
            pub const HWTS3_32: u32 = 0x20;
            #[doc = "ADC TRIG6 selected"]
            pub const HWTS3_64: u32 = 0x40;
            #[doc = "ADC TRIG7 selected"]
            pub const HWTS3_128: u32 = 0x80;
        }
    }
    #[doc = "Segment 3 B2B"]
    pub mod B2B3 {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable B2B. Wait until delay value defined by TRIG3_COUNTER\\[SAMPLE_INTERVAL\\] is reached"]
            pub const B2B3_0: u32 = 0;
            #[doc = "Enable B2B. When Segment 0 finished (ADC COCO) then automatically trigger next ADC conversion, no need to wait until interval delay reached."]
            pub const B2B3_1: u32 = 0x01;
        }
    }
    #[doc = "Segment 3 interrupt enable. (This bit field is meaningful only when IE3_EN is set)"]
    pub mod IE3 {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Generate interrupt on Done0 when segment 3 finish."]
            pub const IE3_0: u32 = 0;
            #[doc = "Generate interrupt on Done1 when segment 3 finish."]
            pub const IE3_1: u32 = 0x01;
            #[doc = "Generate interrupt on Done2 when segment 3 finish."]
            pub const IE3_2: u32 = 0x02;
            #[doc = "Generate interrupt on Done3 when segment 3 finish."]
            pub const IE3_3: u32 = 0x03;
        }
    }
    #[doc = "IRQ enable of segment 3."]
    pub mod IE3_EN {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt DONE disabled."]
            pub const IE3_EN_0: u32 = 0;
            #[doc = "Interrupt DONE enabled. When segment 3 finish, an interrupt will be generated on the specific port configured by the IE3."]
            pub const IE3_EN_1: u32 = 0x01;
        }
    }
}
#[doc = "ETC_TRIG Chain 4/5 Register"]
pub mod TRIG4_CHAIN_5_4 {
    #[doc = "ADC hardware trigger command selection"]
    pub mod CSEL4 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not a valid selection from the command buffer. Trigger event is ignored."]
            pub const CSEL4_0: u32 = 0;
            #[doc = "ADC CMD1 selected."]
            pub const CSEL4_1: u32 = 0x01;
            #[doc = "ADC CMD2 selected."]
            pub const CSEL4_2: u32 = 0x02;
            #[doc = "ADC CMD3 selected."]
            pub const CSEL4_3: u32 = 0x03;
            #[doc = "ADC CMD4 selected."]
            pub const CSEL4_4: u32 = 0x04;
            #[doc = "ADC CMD5 selected."]
            pub const CSEL4_5: u32 = 0x05;
            #[doc = "ADC CMD6 selected."]
            pub const CSEL4_6: u32 = 0x06;
            #[doc = "ADC CMD7 selected."]
            pub const CSEL4_7: u32 = 0x07;
            #[doc = "ADC CMD8 selected."]
            pub const CSEL4_8: u32 = 0x08;
            #[doc = "ADC CMD9 selected."]
            pub const CSEL4_9: u32 = 0x09;
            #[doc = "ADC CMD10 selected."]
            pub const CSEL4_10: u32 = 0x0a;
            #[doc = "ADC CMD11 selected."]
            pub const CSEL4_11: u32 = 0x0b;
            #[doc = "ADC CMD12 selected."]
            pub const CSEL4_12: u32 = 0x0c;
            #[doc = "ADC CMD13 selected."]
            pub const CSEL4_13: u32 = 0x0d;
            #[doc = "ADC CMD14 selected."]
            pub const CSEL4_14: u32 = 0x0e;
            #[doc = "ADC CMD15 selected."]
            pub const CSEL4_15: u32 = 0x0f;
        }
    }
    #[doc = "Segment 4 HWTS ADC hardware trigger selection"]
    pub mod HWTS4 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "no trigger selected"]
            pub const HWTS4_0: u32 = 0;
            #[doc = "ADC TRIG0 selected"]
            pub const HWTS4_1: u32 = 0x01;
            #[doc = "ADC TRIG1 selected"]
            pub const HWTS4_2: u32 = 0x02;
            #[doc = "ADC TRIG2 selected"]
            pub const HWTS4_4: u32 = 0x04;
            #[doc = "ADC TRIG3 selected"]
            pub const HWTS4_8: u32 = 0x08;
            #[doc = "ADC TRIG4 selected"]
            pub const HWTS4_16: u32 = 0x10;
            #[doc = "ADC TRIG5 selected"]
            pub const HWTS4_32: u32 = 0x20;
            #[doc = "ADC TRIG6 selected"]
            pub const HWTS4_64: u32 = 0x40;
            #[doc = "ADC TRIG7 selected"]
            pub const HWTS4_128: u32 = 0x80;
        }
    }
    #[doc = "Segment 4 B2B"]
    pub mod B2B4 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable B2B. Wait until delay value defined by TRIG4_COUNTER\\[SAMPLE_INTERVAL\\] is reached"]
            pub const B2B4_0: u32 = 0;
            #[doc = "Enable B2B. When Segment 0 finished (ADC COCO) then automatically trigger next ADC conversion, no need to wait until interval delay reached."]
            pub const B2B4_1: u32 = 0x01;
        }
    }
    #[doc = "Segment 4 interrupt enable. (This bit field is meaningful only when IE4_EN is set)"]
    pub mod IE4 {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Generate interrupt on Done0 when segment 4 finish."]
            pub const IE4_0: u32 = 0;
            #[doc = "Generate interrupt on Done1 when segment 4 finish."]
            pub const IE4_1: u32 = 0x01;
            #[doc = "Generate interrupt on Done2 when segment 4 finish."]
            pub const IE4_2: u32 = 0x02;
            #[doc = "Generate interrupt on Done3 when segment 4 finish."]
            pub const IE4_3: u32 = 0x03;
        }
    }
    #[doc = "IRQ enable of segment 4."]
    pub mod IE4_EN {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt DONE disabled."]
            pub const IE4_EN_0: u32 = 0;
            #[doc = "Interrupt DONE enabled. When segment 4 finish, an interrupt will be generated on the specific port configured by the IE4."]
            pub const IE4_EN_1: u32 = 0x01;
        }
    }
    #[doc = "ADC hardware trigger command selection"]
    pub mod CSEL5 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not a valid selection from the command buffer. Trigger event is ignored."]
            pub const CSEL5_0: u32 = 0;
            #[doc = "ADC CMD1 selected."]
            pub const CSEL5_1: u32 = 0x01;
            #[doc = "ADC CMD2 selected."]
            pub const CSEL5_2: u32 = 0x02;
            #[doc = "ADC CMD3 selected."]
            pub const CSEL5_3: u32 = 0x03;
            #[doc = "ADC CMD4 selected."]
            pub const CSEL5_4: u32 = 0x04;
            #[doc = "ADC CMD5 selected."]
            pub const CSEL5_5: u32 = 0x05;
            #[doc = "ADC CMD6 selected."]
            pub const CSEL5_6: u32 = 0x06;
            #[doc = "ADC CMD7 selected."]
            pub const CSEL5_7: u32 = 0x07;
            #[doc = "ADC CMD8 selected."]
            pub const CSEL5_8: u32 = 0x08;
            #[doc = "ADC CMD9 selected."]
            pub const CSEL5_9: u32 = 0x09;
            #[doc = "ADC CMD10 selected."]
            pub const CSEL5_10: u32 = 0x0a;
            #[doc = "ADC CMD11 selected."]
            pub const CSEL5_11: u32 = 0x0b;
            #[doc = "ADC CMD12 selected."]
            pub const CSEL5_12: u32 = 0x0c;
            #[doc = "ADC CMD13 selected."]
            pub const CSEL5_13: u32 = 0x0d;
            #[doc = "ADC CMD14 selected."]
            pub const CSEL5_14: u32 = 0x0e;
            #[doc = "ADC CMD15 selected."]
            pub const CSEL5_15: u32 = 0x0f;
        }
    }
    #[doc = "Segment 5 HWTS ADC hardware trigger selection"]
    pub mod HWTS5 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "no trigger selected"]
            pub const HWTS5_0: u32 = 0;
            #[doc = "ADC TRIG0 selected"]
            pub const HWTS5_1: u32 = 0x01;
            #[doc = "ADC TRIG1 selected"]
            pub const HWTS5_2: u32 = 0x02;
            #[doc = "ADC TRIG2 selected"]
            pub const HWTS5_4: u32 = 0x04;
            #[doc = "ADC TRIG3 selected"]
            pub const HWTS5_8: u32 = 0x08;
            #[doc = "ADC TRIG4 selected"]
            pub const HWTS5_16: u32 = 0x10;
            #[doc = "ADC TRIG5 selected"]
            pub const HWTS5_32: u32 = 0x20;
            #[doc = "ADC TRIG6 selected"]
            pub const HWTS5_64: u32 = 0x40;
            #[doc = "ADC TRIG7 selected"]
            pub const HWTS5_128: u32 = 0x80;
        }
    }
    #[doc = "Segment 5 B2B"]
    pub mod B2B5 {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable B2B. Wait until delay value defined by TRIG5_COUNTER\\[SAMPLE_INTERVAL\\] is reached"]
            pub const B2B5_0: u32 = 0;
            #[doc = "Enable B2B. When Segment 0 finished (ADC COCO) then automatically trigger next ADC conversion, no need to wait until interval delay reached."]
            pub const B2B5_1: u32 = 0x01;
        }
    }
    #[doc = "Segment 5 interrupt enable. (This bit field is meaningful only when IE5_EN is set)"]
    pub mod IE5 {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Generate interrupt on Done0 when segment 5 finish."]
            pub const IE5_0: u32 = 0;
            #[doc = "Generate interrupt on Done1 when segment 5 finish."]
            pub const IE5_1: u32 = 0x01;
            #[doc = "Generate interrupt on Done2 when segment 5 finish."]
            pub const IE5_2: u32 = 0x02;
            #[doc = "Generate interrupt on Done3 when segment 5 finish."]
            pub const IE5_3: u32 = 0x03;
        }
    }
    #[doc = "IRQ enable of segment 5."]
    pub mod IE5_EN {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt DONE disabled."]
            pub const IE5_EN_0: u32 = 0;
            #[doc = "Interrupt DONE enabled. When segment 5 finish, an interrupt will be generated on the specific port configured by the IE5."]
            pub const IE5_EN_1: u32 = 0x01;
        }
    }
}
#[doc = "ETC_TRIG Chain 6/7 Register"]
pub mod TRIG4_CHAIN_7_6 {
    #[doc = "ADC hardware trigger command selection"]
    pub mod CSEL6 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not a valid selection from the command buffer. Trigger event is ignored."]
            pub const CSEL6_0: u32 = 0;
            #[doc = "ADC CMD1 selected."]
            pub const CSEL6_1: u32 = 0x01;
            #[doc = "ADC CMD2 selected."]
            pub const CSEL6_2: u32 = 0x02;
            #[doc = "ADC CMD3 selected."]
            pub const CSEL6_3: u32 = 0x03;
            #[doc = "ADC CMD4 selected."]
            pub const CSEL6_4: u32 = 0x04;
            #[doc = "ADC CMD5 selected."]
            pub const CSEL6_5: u32 = 0x05;
            #[doc = "ADC CMD6 selected."]
            pub const CSEL6_6: u32 = 0x06;
            #[doc = "ADC CMD7 selected."]
            pub const CSEL6_7: u32 = 0x07;
            #[doc = "ADC CMD8 selected."]
            pub const CSEL6_8: u32 = 0x08;
            #[doc = "ADC CMD9 selected."]
            pub const CSEL6_9: u32 = 0x09;
            #[doc = "ADC CMD10 selected."]
            pub const CSEL6_10: u32 = 0x0a;
            #[doc = "ADC CMD11 selected."]
            pub const CSEL6_11: u32 = 0x0b;
            #[doc = "ADC CMD12 selected."]
            pub const CSEL6_12: u32 = 0x0c;
            #[doc = "ADC CMD13 selected."]
            pub const CSEL6_13: u32 = 0x0d;
            #[doc = "ADC CMD14 selected."]
            pub const CSEL6_14: u32 = 0x0e;
            #[doc = "ADC CMD15 selected."]
            pub const CSEL6_15: u32 = 0x0f;
        }
    }
    #[doc = "Segment 6 HWTS ADC hardware trigger selection"]
    pub mod HWTS6 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "no trigger selected"]
            pub const HWTS6_0: u32 = 0;
            #[doc = "ADC TRIG0 selected"]
            pub const HWTS6_1: u32 = 0x01;
            #[doc = "ADC TRIG1 selected"]
            pub const HWTS6_2: u32 = 0x02;
            #[doc = "ADC TRIG2 selected"]
            pub const HWTS6_4: u32 = 0x04;
            #[doc = "ADC TRIG3 selected"]
            pub const HWTS6_8: u32 = 0x08;
            #[doc = "ADC TRIG4 selected"]
            pub const HWTS6_16: u32 = 0x10;
            #[doc = "ADC TRIG5 selected"]
            pub const HWTS6_32: u32 = 0x20;
            #[doc = "ADC TRIG6 selected"]
            pub const HWTS6_64: u32 = 0x40;
            #[doc = "ADC TRIG7 selected"]
            pub const HWTS6_128: u32 = 0x80;
        }
    }
    #[doc = "Segment 6 B2B"]
    pub mod B2B6 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable B2B. Wait until delay value defined by TRIG6_COUNTER\\[SAMPLE_INTERVAL\\] is reached"]
            pub const B2B6_0: u32 = 0;
            #[doc = "Enable B2B. When Segment 0 finished (ADC COCO) then automatically trigger next ADC conversion, no need to wait until interval delay reached."]
            pub const B2B6_1: u32 = 0x01;
        }
    }
    #[doc = "Segment 6 interrupt enable. (This bit field is meaningful only when IE6_EN is set)"]
    pub mod IE6 {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Generate interrupt on Done0 when segment 6 finish."]
            pub const IE6_0: u32 = 0;
            #[doc = "Generate interrupt on Done1 when segment 6 finish."]
            pub const IE6_1: u32 = 0x01;
            #[doc = "Generate interrupt on Done2 when segment 6 finish."]
            pub const IE6_2: u32 = 0x02;
            #[doc = "Generate interrupt on Done3 when segment 6 finish."]
            pub const IE6_3: u32 = 0x03;
        }
    }
    #[doc = "IRQ enable of segment 6."]
    pub mod IE6_EN {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt DONE disabled."]
            pub const IE6_EN_0: u32 = 0;
            #[doc = "Interrupt DONE enabled. When segment 6 finish, an interrupt will be generated on the specific port configured by the IE6."]
            pub const IE6_EN_1: u32 = 0x01;
        }
    }
    #[doc = "ADC hardware trigger command selection"]
    pub mod CSEL7 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not a valid selection from the command buffer. Trigger event is ignored."]
            pub const CSEL7_0: u32 = 0;
            #[doc = "ADC CMD1 selected."]
            pub const CSEL7_1: u32 = 0x01;
            #[doc = "ADC CMD2 selected."]
            pub const CSEL7_2: u32 = 0x02;
            #[doc = "ADC CMD3 selected."]
            pub const CSEL7_3: u32 = 0x03;
            #[doc = "ADC CMD4 selected."]
            pub const CSEL7_4: u32 = 0x04;
            #[doc = "ADC CMD5 selected."]
            pub const CSEL7_5: u32 = 0x05;
            #[doc = "ADC CMD6 selected."]
            pub const CSEL7_6: u32 = 0x06;
            #[doc = "ADC CMD7 selected."]
            pub const CSEL7_7: u32 = 0x07;
            #[doc = "ADC CMD8 selected."]
            pub const CSEL7_8: u32 = 0x08;
            #[doc = "ADC CMD9 selected."]
            pub const CSEL7_9: u32 = 0x09;
            #[doc = "ADC CMD10 selected."]
            pub const CSEL7_10: u32 = 0x0a;
            #[doc = "ADC CMD11 selected."]
            pub const CSEL7_11: u32 = 0x0b;
            #[doc = "ADC CMD12 selected."]
            pub const CSEL7_12: u32 = 0x0c;
            #[doc = "ADC CMD13 selected."]
            pub const CSEL7_13: u32 = 0x0d;
            #[doc = "ADC CMD14 selected."]
            pub const CSEL7_14: u32 = 0x0e;
            #[doc = "ADC CMD15 selected."]
            pub const CSEL7_15: u32 = 0x0f;
        }
    }
    #[doc = "Segment 7 HWTS ADC hardware trigger selection"]
    pub mod HWTS7 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "no trigger selected"]
            pub const HWTS7_0: u32 = 0;
            #[doc = "ADC TRIG0 selected"]
            pub const HWTS7_1: u32 = 0x01;
            #[doc = "ADC TRIG1 selected"]
            pub const HWTS7_2: u32 = 0x02;
            #[doc = "ADC TRIG2 selected"]
            pub const HWTS7_4: u32 = 0x04;
            #[doc = "ADC TRIG3 selected"]
            pub const HWTS7_8: u32 = 0x08;
            #[doc = "ADC TRIG4 selected"]
            pub const HWTS7_16: u32 = 0x10;
            #[doc = "ADC TRIG5 selected"]
            pub const HWTS7_32: u32 = 0x20;
            #[doc = "ADC TRIG6 selected"]
            pub const HWTS7_64: u32 = 0x40;
            #[doc = "ADC TRIG7 selected"]
            pub const HWTS7_128: u32 = 0x80;
        }
    }
    #[doc = "Segment 7 B2B"]
    pub mod B2B7 {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable B2B. Wait until delay value defined by TRIG7_COUNTER\\[SAMPLE_INTERVAL\\] is reached"]
            pub const B2B7_0: u32 = 0;
            #[doc = "Enable B2B. When Segment 0 finished (ADC COCO) then automatically trigger next ADC conversion, no need to wait until interval delay reached."]
            pub const B2B7_1: u32 = 0x01;
        }
    }
    #[doc = "Segment 7 interrupt enable. (This bit field is meaningful only when IE7_EN is set)"]
    pub mod IE7 {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Generate interrupt on Done0 when segment 7 finish."]
            pub const IE7_0: u32 = 0;
            #[doc = "Generate interrupt on Done1 when segment 7 finish."]
            pub const IE7_1: u32 = 0x01;
            #[doc = "Generate interrupt on Done2 when segment 7 finish."]
            pub const IE7_2: u32 = 0x02;
            #[doc = "Generate interrupt on Done3 when segment 7 finish."]
            pub const IE7_3: u32 = 0x03;
        }
    }
    #[doc = "IRQ enable of segment 7."]
    pub mod IE7_EN {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt DONE disabled."]
            pub const IE7_EN_0: u32 = 0;
            #[doc = "Interrupt DONE enabled. When segment 7 finish, an interrupt will be generated on the specific port configured by the IE7."]
            pub const IE7_EN_1: u32 = 0x01;
        }
    }
}
#[doc = "ETC_TRIG Result Data 1/0 Register"]
pub mod TRIG4_RESULT_1_0 {
    #[doc = "Result DATA0"]
    pub mod DATA0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Result DATA1"]
    pub mod DATA1 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ETC_TRIG Result Data 3/2 Register"]
pub mod TRIG4_RESULT_3_2 {
    #[doc = "Result DATA2"]
    pub mod DATA2 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Result DATA3"]
    pub mod DATA3 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ETC_TRIG Result Data 5/4 Register"]
pub mod TRIG4_RESULT_5_4 {
    #[doc = "Result DATA4"]
    pub mod DATA4 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Result DATA5"]
    pub mod DATA5 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ETC_TRIG Result Data 7/6 Register"]
pub mod TRIG4_RESULT_7_6 {
    #[doc = "Result DATA6"]
    pub mod DATA6 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Result DATA7"]
    pub mod DATA7 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ETC_TRIG Control Register"]
pub mod TRIG5_CTRL {
    #[doc = "Software trigger. This field is self-clearing."]
    pub mod SW_TRIG {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No software trigger event generated."]
            pub const SW_TRIG_0: u32 = 0;
            #[doc = "Software trigger event generated."]
            pub const SW_TRIG_1: u32 = 0x01;
        }
    }
    #[doc = "Trigger mode selection."]
    pub mod TRIG_MODE {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Hardware trigger. The softerware trigger will be ignored."]
            pub const TRIG_MODE_0: u32 = 0;
            #[doc = "Software trigger. The hardware trigger will be ignored."]
            pub const TRIG_MODE_1: u32 = 0x01;
        }
    }
    #[doc = "The number of segments inside the trigger chain of TRIGa."]
    pub mod TRIG_CHAIN {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Trigger chain length is 1"]
            pub const TRIG_CHAIN_0: u32 = 0;
            #[doc = "Trigger chain length is 2"]
            pub const TRIG_CHAIN_1: u32 = 0x01;
            #[doc = "Trigger chain length is 3"]
            pub const TRIG_CHAIN_2: u32 = 0x02;
            #[doc = "Trigger chain length is 4"]
            pub const TRIG_CHAIN_3: u32 = 0x03;
            #[doc = "Trigger chain length is 5"]
            pub const TRIG_CHAIN_4: u32 = 0x04;
            #[doc = "Trigger chain length is 6"]
            pub const TRIG_CHAIN_5: u32 = 0x05;
            #[doc = "Trigger chain length is 7"]
            pub const TRIG_CHAIN_6: u32 = 0x06;
            #[doc = "Trigger chain length is 8"]
            pub const TRIG_CHAIN_7: u32 = 0x07;
        }
    }
    #[doc = "External trigger priority, 7 is highest priority, while 0 is lowest"]
    pub mod TRIG_PRIORITY {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Trigger synchronization mode selection"]
    pub mod SYNC_MODE {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Synchronization mode disabled, TRIGa and TRIG(a+4) are triggered independently."]
            pub const SYNC_MODE_0: u32 = 0;
            #[doc = "Synchronization mode enabled, TRIGa and TRIG(a+4) are triggered by TRIGa source synchronously."]
            pub const SYNC_MODE_1: u32 = 0x01;
        }
    }
    #[doc = "Segment x done detection"]
    pub mod CHAINX_DONE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "segment x done not detected."]
            pub const CHAINX_DONE_0: u32 = 0;
            #[doc = "segment x done detected."]
            pub const CHAINX_DONE_1: u32 = 0x01;
        }
    }
}
#[doc = "ETC_TRIG Counter Register"]
pub mod TRIG5_COUNTER {
    #[doc = "TRIGGER initial delay counter"]
    pub mod INIT_DELAY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TRIGGER sampling interval counter"]
    pub mod SAMPLE_INTERVAL {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ETC_TRIG Chain 0/1 Register"]
pub mod TRIG5_CHAIN_1_0 {
    #[doc = "ADC hardware trigger command selection"]
    pub mod CSEL0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not a valid selection from the command buffer. Trigger event is ignored."]
            pub const CSEL0_0: u32 = 0;
            #[doc = "ADC CMD1 selected."]
            pub const CSEL0_1: u32 = 0x01;
            #[doc = "ADC CMD2 selected."]
            pub const CSEL0_2: u32 = 0x02;
            #[doc = "ADC CMD3 selected."]
            pub const CSEL0_3: u32 = 0x03;
            #[doc = "ADC CMD4 selected."]
            pub const CSEL0_4: u32 = 0x04;
            #[doc = "ADC CMD5 selected."]
            pub const CSEL0_5: u32 = 0x05;
            #[doc = "ADC CMD6 selected."]
            pub const CSEL0_6: u32 = 0x06;
            #[doc = "ADC CMD7 selected."]
            pub const CSEL0_7: u32 = 0x07;
            #[doc = "ADC CMD8 selected."]
            pub const CSEL0_8: u32 = 0x08;
            #[doc = "ADC CMD9 selected."]
            pub const CSEL0_9: u32 = 0x09;
            #[doc = "ADC CMD10 selected."]
            pub const CSEL0_10: u32 = 0x0a;
            #[doc = "ADC CMD11 selected."]
            pub const CSEL0_11: u32 = 0x0b;
            #[doc = "ADC CMD12 selected."]
            pub const CSEL0_12: u32 = 0x0c;
            #[doc = "ADC CMD13 selected."]
            pub const CSEL0_13: u32 = 0x0d;
            #[doc = "ADC CMD14 selected."]
            pub const CSEL0_14: u32 = 0x0e;
            #[doc = "ADC CMD15 selected."]
            pub const CSEL0_15: u32 = 0x0f;
        }
    }
    #[doc = "Segment 0 HWTS ADC hardware trigger selection"]
    pub mod HWTS0 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "no trigger selected"]
            pub const HWTS0_0: u32 = 0;
            #[doc = "ADC TRIG0 selected"]
            pub const HWTS0_1: u32 = 0x01;
            #[doc = "ADC TRIG1 selected"]
            pub const HWTS0_2: u32 = 0x02;
            #[doc = "ADC TRIG2 selected"]
            pub const HWTS0_4: u32 = 0x04;
            #[doc = "ADC TRIG3 selected"]
            pub const HWTS0_8: u32 = 0x08;
            #[doc = "ADC TRIG4 selected"]
            pub const HWTS0_16: u32 = 0x10;
            #[doc = "ADC TRIG5 selected"]
            pub const HWTS0_32: u32 = 0x20;
            #[doc = "ADC TRIG6 selected"]
            pub const HWTS0_64: u32 = 0x40;
            #[doc = "ADC TRIG7 selected"]
            pub const HWTS0_128: u32 = 0x80;
        }
    }
    #[doc = "Segment 0 B2B"]
    pub mod B2B0 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable B2B. Wait until delay value defined by TRIG0_COUNTER\\[SAMPLE_INTERVAL\\] is reached"]
            pub const B2B0_0: u32 = 0;
            #[doc = "Enable B2B. When Segment 0 finished (ADC COCO) then automatically trigger next ADC conversion, no need to wait until interval delay reached."]
            pub const B2B0_1: u32 = 0x01;
        }
    }
    #[doc = "Segment 0 interrupt enable. (This bit field is meaningful only when IE0_EN is set)"]
    pub mod IE0 {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Generate interrupt on Done0 when segment 0 finish."]
            pub const IE0_0: u32 = 0;
            #[doc = "Generate interrupt on Done1 when segment 0 finish."]
            pub const IE0_1: u32 = 0x01;
            #[doc = "Generate interrupt on Done2 when segment 0 finish."]
            pub const IE0_2: u32 = 0x02;
            #[doc = "Generate interrupt on Done3 when segment 0 finish."]
            pub const IE0_3: u32 = 0x03;
        }
    }
    #[doc = "IRQ enable of segment 0."]
    pub mod IE0_EN {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt DONE disabled."]
            pub const IE0_EN_0: u32 = 0;
            #[doc = "Interrupt DONE enabled. When segment 0 finish, an interrupt will be generated on the specific port configured by the IE0."]
            pub const IE0_EN_1: u32 = 0x01;
        }
    }
    #[doc = "ADC hardware trigger command selection"]
    pub mod CSEL1 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not a valid selection from the command buffer. Trigger event is ignored."]
            pub const CSEL1_0: u32 = 0;
            #[doc = "ADC CMD1 selected."]
            pub const CSEL1_1: u32 = 0x01;
            #[doc = "ADC CMD2 selected."]
            pub const CSEL1_2: u32 = 0x02;
            #[doc = "ADC CMD3 selected."]
            pub const CSEL1_3: u32 = 0x03;
            #[doc = "ADC CMD4 selected."]
            pub const CSEL1_4: u32 = 0x04;
            #[doc = "ADC CMD5 selected."]
            pub const CSEL1_5: u32 = 0x05;
            #[doc = "ADC CMD6 selected."]
            pub const CSEL1_6: u32 = 0x06;
            #[doc = "ADC CMD7 selected."]
            pub const CSEL1_7: u32 = 0x07;
            #[doc = "ADC CMD8 selected."]
            pub const CSEL1_8: u32 = 0x08;
            #[doc = "ADC CMD9 selected."]
            pub const CSEL1_9: u32 = 0x09;
            #[doc = "ADC CMD10 selected."]
            pub const CSEL1_10: u32 = 0x0a;
            #[doc = "ADC CMD11 selected."]
            pub const CSEL1_11: u32 = 0x0b;
            #[doc = "ADC CMD12 selected."]
            pub const CSEL1_12: u32 = 0x0c;
            #[doc = "ADC CMD13 selected."]
            pub const CSEL1_13: u32 = 0x0d;
            #[doc = "ADC CMD14 selected."]
            pub const CSEL1_14: u32 = 0x0e;
            #[doc = "ADC CMD15 selected."]
            pub const CSEL1_15: u32 = 0x0f;
        }
    }
    #[doc = "Segment 1 HWTS ADC hardware trigger selection"]
    pub mod HWTS1 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "no trigger selected"]
            pub const HWTS1_0: u32 = 0;
            #[doc = "ADC TRIG0 selected"]
            pub const HWTS1_1: u32 = 0x01;
            #[doc = "ADC TRIG1 selected"]
            pub const HWTS1_2: u32 = 0x02;
            #[doc = "ADC TRIG2 selected"]
            pub const HWTS1_4: u32 = 0x04;
            #[doc = "ADC TRIG3 selected"]
            pub const HWTS1_8: u32 = 0x08;
            #[doc = "ADC TRIG4 selected"]
            pub const HWTS1_16: u32 = 0x10;
            #[doc = "ADC TRIG5 selected"]
            pub const HWTS1_32: u32 = 0x20;
            #[doc = "ADC TRIG6 selected"]
            pub const HWTS1_64: u32 = 0x40;
            #[doc = "ADC TRIG7 selected"]
            pub const HWTS1_128: u32 = 0x80;
        }
    }
    #[doc = "Segment 1 B2B"]
    pub mod B2B1 {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable B2B. Wait until delay value defined by TRIG1_COUNTER\\[SAMPLE_INTERVAL\\] is reached"]
            pub const B2B1_0: u32 = 0;
            #[doc = "Enable B2B. When Segment 0 finished (ADC COCO) then automatically trigger next ADC conversion, no need to wait until interval delay reached."]
            pub const B2B1_1: u32 = 0x01;
        }
    }
    #[doc = "Segment 1 interrupt enable. (This bit field is meaningful only when IE1_EN is set)"]
    pub mod IE1 {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Generate interrupt on Done0 when Segment 1 finish."]
            pub const IE1_0: u32 = 0;
            #[doc = "Generate interrupt on Done1 when Segment 1 finish."]
            pub const IE1_1: u32 = 0x01;
            #[doc = "Generate interrupt on Done2 when Segment 1 finish."]
            pub const IE1_2: u32 = 0x02;
            #[doc = "Generate interrupt on Done3 when Segment 1 finish."]
            pub const IE1_3: u32 = 0x03;
        }
    }
    #[doc = "IRQ enable of segment 1."]
    pub mod IE1_EN {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt DONE disabled."]
            pub const IE1_EN_0: u32 = 0;
            #[doc = "Interrupt DONE enabled. When segment 1 finish, an interrupt will be generated on the specific port configured by the IE1."]
            pub const IE1_EN_1: u32 = 0x01;
        }
    }
}
#[doc = "ETC_TRIG Chain 2/3 Register"]
pub mod TRIG5_CHAIN_3_2 {
    #[doc = "ADC hardware trigger command selection"]
    pub mod CSEL2 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not a valid selection from the command buffer. Trigger event is ignored."]
            pub const CSEL2_0: u32 = 0;
            #[doc = "ADC CMD1 selected."]
            pub const CSEL2_1: u32 = 0x01;
            #[doc = "ADC CMD2 selected."]
            pub const CSEL2_2: u32 = 0x02;
            #[doc = "ADC CMD3 selected."]
            pub const CSEL2_3: u32 = 0x03;
            #[doc = "ADC CMD4 selected."]
            pub const CSEL2_4: u32 = 0x04;
            #[doc = "ADC CMD5 selected."]
            pub const CSEL2_5: u32 = 0x05;
            #[doc = "ADC CMD6 selected."]
            pub const CSEL2_6: u32 = 0x06;
            #[doc = "ADC CMD7 selected."]
            pub const CSEL2_7: u32 = 0x07;
            #[doc = "ADC CMD8 selected."]
            pub const CSEL2_8: u32 = 0x08;
            #[doc = "ADC CMD9 selected."]
            pub const CSEL2_9: u32 = 0x09;
            #[doc = "ADC CMD10 selected."]
            pub const CSEL2_10: u32 = 0x0a;
            #[doc = "ADC CMD11 selected."]
            pub const CSEL2_11: u32 = 0x0b;
            #[doc = "ADC CMD12 selected."]
            pub const CSEL2_12: u32 = 0x0c;
            #[doc = "ADC CMD13 selected."]
            pub const CSEL2_13: u32 = 0x0d;
            #[doc = "ADC CMD14 selected."]
            pub const CSEL2_14: u32 = 0x0e;
            #[doc = "ADC CMD15 selected."]
            pub const CSEL2_15: u32 = 0x0f;
        }
    }
    #[doc = "Segment 2 HWTS ADC hardware trigger selection"]
    pub mod HWTS2 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "no trigger selected"]
            pub const HWTS2_0: u32 = 0;
            #[doc = "ADC TRIG0 selected"]
            pub const HWTS2_1: u32 = 0x01;
            #[doc = "ADC TRIG1 selected"]
            pub const HWTS2_2: u32 = 0x02;
            #[doc = "ADC TRIG2 selected"]
            pub const HWTS2_4: u32 = 0x04;
            #[doc = "ADC TRIG3 selected"]
            pub const HWTS2_8: u32 = 0x08;
            #[doc = "ADC TRIG4 selected"]
            pub const HWTS2_16: u32 = 0x10;
            #[doc = "ADC TRIG5 selected"]
            pub const HWTS2_32: u32 = 0x20;
            #[doc = "ADC TRIG6 selected"]
            pub const HWTS2_64: u32 = 0x40;
            #[doc = "ADC TRIG7 selected"]
            pub const HWTS2_128: u32 = 0x80;
        }
    }
    #[doc = "Segment 2 B2B"]
    pub mod B2B2 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable B2B. Wait until delay value defined by TRIG2_COUNTER\\[SAMPLE_INTERVAL\\] is reached"]
            pub const B2B2_0: u32 = 0;
            #[doc = "Enable B2B. When Segment 0 finished (ADC COCO) then automatically trigger next ADC conversion, no need to wait until interval delay reached."]
            pub const B2B2_1: u32 = 0x01;
        }
    }
    #[doc = "Segment 2 interrupt enable. (This bit field is meaningful only when IE2_EN is set)"]
    pub mod IE2 {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Generate interrupt on Done0 when segment 2 finish."]
            pub const IE2_0: u32 = 0;
            #[doc = "Generate interrupt on Done1 when segment 2 finish."]
            pub const IE2_1: u32 = 0x01;
            #[doc = "Generate interrupt on Done2 when segment 2 finish."]
            pub const IE2_2: u32 = 0x02;
            #[doc = "Generate interrupt on Done3 when segment 2 finish."]
            pub const IE2_3: u32 = 0x03;
        }
    }
    #[doc = "IRQ enable of segment 2."]
    pub mod IE2_EN {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt DONE disabled."]
            pub const IE2_EN_0: u32 = 0;
            #[doc = "Interrupt DONE enabled. When segment 2 finish, an interrupt will be generated on the specific port configured by the IE2."]
            pub const IE2_EN_1: u32 = 0x01;
        }
    }
    #[doc = "ADC hardware trigger command selection"]
    pub mod CSEL3 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not a valid selection from the command buffer. Trigger event is ignored."]
            pub const CSEL3_0: u32 = 0;
            #[doc = "ADC CMD1 selected."]
            pub const CSEL3_1: u32 = 0x01;
            #[doc = "ADC CMD2 selected."]
            pub const CSEL3_2: u32 = 0x02;
            #[doc = "ADC CMD3 selected."]
            pub const CSEL3_3: u32 = 0x03;
            #[doc = "ADC CMD4 selected."]
            pub const CSEL3_4: u32 = 0x04;
            #[doc = "ADC CMD5 selected."]
            pub const CSEL3_5: u32 = 0x05;
            #[doc = "ADC CMD6 selected."]
            pub const CSEL3_6: u32 = 0x06;
            #[doc = "ADC CMD7 selected."]
            pub const CSEL3_7: u32 = 0x07;
            #[doc = "ADC CMD8 selected."]
            pub const CSEL3_8: u32 = 0x08;
            #[doc = "ADC CMD9 selected."]
            pub const CSEL3_9: u32 = 0x09;
            #[doc = "ADC CMD10 selected."]
            pub const CSEL3_10: u32 = 0x0a;
            #[doc = "ADC CMD11 selected."]
            pub const CSEL3_11: u32 = 0x0b;
            #[doc = "ADC CMD12 selected."]
            pub const CSEL3_12: u32 = 0x0c;
            #[doc = "ADC CMD13 selected."]
            pub const CSEL3_13: u32 = 0x0d;
            #[doc = "ADC CMD14 selected."]
            pub const CSEL3_14: u32 = 0x0e;
            #[doc = "ADC CMD15 selected."]
            pub const CSEL3_15: u32 = 0x0f;
        }
    }
    #[doc = "Segment 3 HWTS ADC hardware trigger selection"]
    pub mod HWTS3 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "no trigger selected"]
            pub const HWTS3_0: u32 = 0;
            #[doc = "ADC TRIG0 selected"]
            pub const HWTS3_1: u32 = 0x01;
            #[doc = "ADC TRIG1 selected"]
            pub const HWTS3_2: u32 = 0x02;
            #[doc = "ADC TRIG2 selected"]
            pub const HWTS3_4: u32 = 0x04;
            #[doc = "ADC TRIG3 selected"]
            pub const HWTS3_8: u32 = 0x08;
            #[doc = "ADC TRIG4 selected"]
            pub const HWTS3_16: u32 = 0x10;
            #[doc = "ADC TRIG5 selected"]
            pub const HWTS3_32: u32 = 0x20;
            #[doc = "ADC TRIG6 selected"]
            pub const HWTS3_64: u32 = 0x40;
            #[doc = "ADC TRIG7 selected"]
            pub const HWTS3_128: u32 = 0x80;
        }
    }
    #[doc = "Segment 3 B2B"]
    pub mod B2B3 {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable B2B. Wait until delay value defined by TRIG3_COUNTER\\[SAMPLE_INTERVAL\\] is reached"]
            pub const B2B3_0: u32 = 0;
            #[doc = "Enable B2B. When Segment 0 finished (ADC COCO) then automatically trigger next ADC conversion, no need to wait until interval delay reached."]
            pub const B2B3_1: u32 = 0x01;
        }
    }
    #[doc = "Segment 3 interrupt enable. (This bit field is meaningful only when IE3_EN is set)"]
    pub mod IE3 {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Generate interrupt on Done0 when segment 3 finish."]
            pub const IE3_0: u32 = 0;
            #[doc = "Generate interrupt on Done1 when segment 3 finish."]
            pub const IE3_1: u32 = 0x01;
            #[doc = "Generate interrupt on Done2 when segment 3 finish."]
            pub const IE3_2: u32 = 0x02;
            #[doc = "Generate interrupt on Done3 when segment 3 finish."]
            pub const IE3_3: u32 = 0x03;
        }
    }
    #[doc = "IRQ enable of segment 3."]
    pub mod IE3_EN {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt DONE disabled."]
            pub const IE3_EN_0: u32 = 0;
            #[doc = "Interrupt DONE enabled. When segment 3 finish, an interrupt will be generated on the specific port configured by the IE3."]
            pub const IE3_EN_1: u32 = 0x01;
        }
    }
}
#[doc = "ETC_TRIG Chain 4/5 Register"]
pub mod TRIG5_CHAIN_5_4 {
    #[doc = "ADC hardware trigger command selection"]
    pub mod CSEL4 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not a valid selection from the command buffer. Trigger event is ignored."]
            pub const CSEL4_0: u32 = 0;
            #[doc = "ADC CMD1 selected."]
            pub const CSEL4_1: u32 = 0x01;
            #[doc = "ADC CMD2 selected."]
            pub const CSEL4_2: u32 = 0x02;
            #[doc = "ADC CMD3 selected."]
            pub const CSEL4_3: u32 = 0x03;
            #[doc = "ADC CMD4 selected."]
            pub const CSEL4_4: u32 = 0x04;
            #[doc = "ADC CMD5 selected."]
            pub const CSEL4_5: u32 = 0x05;
            #[doc = "ADC CMD6 selected."]
            pub const CSEL4_6: u32 = 0x06;
            #[doc = "ADC CMD7 selected."]
            pub const CSEL4_7: u32 = 0x07;
            #[doc = "ADC CMD8 selected."]
            pub const CSEL4_8: u32 = 0x08;
            #[doc = "ADC CMD9 selected."]
            pub const CSEL4_9: u32 = 0x09;
            #[doc = "ADC CMD10 selected."]
            pub const CSEL4_10: u32 = 0x0a;
            #[doc = "ADC CMD11 selected."]
            pub const CSEL4_11: u32 = 0x0b;
            #[doc = "ADC CMD12 selected."]
            pub const CSEL4_12: u32 = 0x0c;
            #[doc = "ADC CMD13 selected."]
            pub const CSEL4_13: u32 = 0x0d;
            #[doc = "ADC CMD14 selected."]
            pub const CSEL4_14: u32 = 0x0e;
            #[doc = "ADC CMD15 selected."]
            pub const CSEL4_15: u32 = 0x0f;
        }
    }
    #[doc = "Segment 4 HWTS ADC hardware trigger selection"]
    pub mod HWTS4 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "no trigger selected"]
            pub const HWTS4_0: u32 = 0;
            #[doc = "ADC TRIG0 selected"]
            pub const HWTS4_1: u32 = 0x01;
            #[doc = "ADC TRIG1 selected"]
            pub const HWTS4_2: u32 = 0x02;
            #[doc = "ADC TRIG2 selected"]
            pub const HWTS4_4: u32 = 0x04;
            #[doc = "ADC TRIG3 selected"]
            pub const HWTS4_8: u32 = 0x08;
            #[doc = "ADC TRIG4 selected"]
            pub const HWTS4_16: u32 = 0x10;
            #[doc = "ADC TRIG5 selected"]
            pub const HWTS4_32: u32 = 0x20;
            #[doc = "ADC TRIG6 selected"]
            pub const HWTS4_64: u32 = 0x40;
            #[doc = "ADC TRIG7 selected"]
            pub const HWTS4_128: u32 = 0x80;
        }
    }
    #[doc = "Segment 4 B2B"]
    pub mod B2B4 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable B2B. Wait until delay value defined by TRIG4_COUNTER\\[SAMPLE_INTERVAL\\] is reached"]
            pub const B2B4_0: u32 = 0;
            #[doc = "Enable B2B. When Segment 0 finished (ADC COCO) then automatically trigger next ADC conversion, no need to wait until interval delay reached."]
            pub const B2B4_1: u32 = 0x01;
        }
    }
    #[doc = "Segment 4 interrupt enable. (This bit field is meaningful only when IE4_EN is set)"]
    pub mod IE4 {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Generate interrupt on Done0 when segment 4 finish."]
            pub const IE4_0: u32 = 0;
            #[doc = "Generate interrupt on Done1 when segment 4 finish."]
            pub const IE4_1: u32 = 0x01;
            #[doc = "Generate interrupt on Done2 when segment 4 finish."]
            pub const IE4_2: u32 = 0x02;
            #[doc = "Generate interrupt on Done3 when segment 4 finish."]
            pub const IE4_3: u32 = 0x03;
        }
    }
    #[doc = "IRQ enable of segment 4."]
    pub mod IE4_EN {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt DONE disabled."]
            pub const IE4_EN_0: u32 = 0;
            #[doc = "Interrupt DONE enabled. When segment 4 finish, an interrupt will be generated on the specific port configured by the IE4."]
            pub const IE4_EN_1: u32 = 0x01;
        }
    }
    #[doc = "ADC hardware trigger command selection"]
    pub mod CSEL5 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not a valid selection from the command buffer. Trigger event is ignored."]
            pub const CSEL5_0: u32 = 0;
            #[doc = "ADC CMD1 selected."]
            pub const CSEL5_1: u32 = 0x01;
            #[doc = "ADC CMD2 selected."]
            pub const CSEL5_2: u32 = 0x02;
            #[doc = "ADC CMD3 selected."]
            pub const CSEL5_3: u32 = 0x03;
            #[doc = "ADC CMD4 selected."]
            pub const CSEL5_4: u32 = 0x04;
            #[doc = "ADC CMD5 selected."]
            pub const CSEL5_5: u32 = 0x05;
            #[doc = "ADC CMD6 selected."]
            pub const CSEL5_6: u32 = 0x06;
            #[doc = "ADC CMD7 selected."]
            pub const CSEL5_7: u32 = 0x07;
            #[doc = "ADC CMD8 selected."]
            pub const CSEL5_8: u32 = 0x08;
            #[doc = "ADC CMD9 selected."]
            pub const CSEL5_9: u32 = 0x09;
            #[doc = "ADC CMD10 selected."]
            pub const CSEL5_10: u32 = 0x0a;
            #[doc = "ADC CMD11 selected."]
            pub const CSEL5_11: u32 = 0x0b;
            #[doc = "ADC CMD12 selected."]
            pub const CSEL5_12: u32 = 0x0c;
            #[doc = "ADC CMD13 selected."]
            pub const CSEL5_13: u32 = 0x0d;
            #[doc = "ADC CMD14 selected."]
            pub const CSEL5_14: u32 = 0x0e;
            #[doc = "ADC CMD15 selected."]
            pub const CSEL5_15: u32 = 0x0f;
        }
    }
    #[doc = "Segment 5 HWTS ADC hardware trigger selection"]
    pub mod HWTS5 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "no trigger selected"]
            pub const HWTS5_0: u32 = 0;
            #[doc = "ADC TRIG0 selected"]
            pub const HWTS5_1: u32 = 0x01;
            #[doc = "ADC TRIG1 selected"]
            pub const HWTS5_2: u32 = 0x02;
            #[doc = "ADC TRIG2 selected"]
            pub const HWTS5_4: u32 = 0x04;
            #[doc = "ADC TRIG3 selected"]
            pub const HWTS5_8: u32 = 0x08;
            #[doc = "ADC TRIG4 selected"]
            pub const HWTS5_16: u32 = 0x10;
            #[doc = "ADC TRIG5 selected"]
            pub const HWTS5_32: u32 = 0x20;
            #[doc = "ADC TRIG6 selected"]
            pub const HWTS5_64: u32 = 0x40;
            #[doc = "ADC TRIG7 selected"]
            pub const HWTS5_128: u32 = 0x80;
        }
    }
    #[doc = "Segment 5 B2B"]
    pub mod B2B5 {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable B2B. Wait until delay value defined by TRIG5_COUNTER\\[SAMPLE_INTERVAL\\] is reached"]
            pub const B2B5_0: u32 = 0;
            #[doc = "Enable B2B. When Segment 0 finished (ADC COCO) then automatically trigger next ADC conversion, no need to wait until interval delay reached."]
            pub const B2B5_1: u32 = 0x01;
        }
    }
    #[doc = "Segment 5 interrupt enable. (This bit field is meaningful only when IE5_EN is set)"]
    pub mod IE5 {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Generate interrupt on Done0 when segment 5 finish."]
            pub const IE5_0: u32 = 0;
            #[doc = "Generate interrupt on Done1 when segment 5 finish."]
            pub const IE5_1: u32 = 0x01;
            #[doc = "Generate interrupt on Done2 when segment 5 finish."]
            pub const IE5_2: u32 = 0x02;
            #[doc = "Generate interrupt on Done3 when segment 5 finish."]
            pub const IE5_3: u32 = 0x03;
        }
    }
    #[doc = "IRQ enable of segment 5."]
    pub mod IE5_EN {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt DONE disabled."]
            pub const IE5_EN_0: u32 = 0;
            #[doc = "Interrupt DONE enabled. When segment 5 finish, an interrupt will be generated on the specific port configured by the IE5."]
            pub const IE5_EN_1: u32 = 0x01;
        }
    }
}
#[doc = "ETC_TRIG Chain 6/7 Register"]
pub mod TRIG5_CHAIN_7_6 {
    #[doc = "ADC hardware trigger command selection"]
    pub mod CSEL6 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not a valid selection from the command buffer. Trigger event is ignored."]
            pub const CSEL6_0: u32 = 0;
            #[doc = "ADC CMD1 selected."]
            pub const CSEL6_1: u32 = 0x01;
            #[doc = "ADC CMD2 selected."]
            pub const CSEL6_2: u32 = 0x02;
            #[doc = "ADC CMD3 selected."]
            pub const CSEL6_3: u32 = 0x03;
            #[doc = "ADC CMD4 selected."]
            pub const CSEL6_4: u32 = 0x04;
            #[doc = "ADC CMD5 selected."]
            pub const CSEL6_5: u32 = 0x05;
            #[doc = "ADC CMD6 selected."]
            pub const CSEL6_6: u32 = 0x06;
            #[doc = "ADC CMD7 selected."]
            pub const CSEL6_7: u32 = 0x07;
            #[doc = "ADC CMD8 selected."]
            pub const CSEL6_8: u32 = 0x08;
            #[doc = "ADC CMD9 selected."]
            pub const CSEL6_9: u32 = 0x09;
            #[doc = "ADC CMD10 selected."]
            pub const CSEL6_10: u32 = 0x0a;
            #[doc = "ADC CMD11 selected."]
            pub const CSEL6_11: u32 = 0x0b;
            #[doc = "ADC CMD12 selected."]
            pub const CSEL6_12: u32 = 0x0c;
            #[doc = "ADC CMD13 selected."]
            pub const CSEL6_13: u32 = 0x0d;
            #[doc = "ADC CMD14 selected."]
            pub const CSEL6_14: u32 = 0x0e;
            #[doc = "ADC CMD15 selected."]
            pub const CSEL6_15: u32 = 0x0f;
        }
    }
    #[doc = "Segment 6 HWTS ADC hardware trigger selection"]
    pub mod HWTS6 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "no trigger selected"]
            pub const HWTS6_0: u32 = 0;
            #[doc = "ADC TRIG0 selected"]
            pub const HWTS6_1: u32 = 0x01;
            #[doc = "ADC TRIG1 selected"]
            pub const HWTS6_2: u32 = 0x02;
            #[doc = "ADC TRIG2 selected"]
            pub const HWTS6_4: u32 = 0x04;
            #[doc = "ADC TRIG3 selected"]
            pub const HWTS6_8: u32 = 0x08;
            #[doc = "ADC TRIG4 selected"]
            pub const HWTS6_16: u32 = 0x10;
            #[doc = "ADC TRIG5 selected"]
            pub const HWTS6_32: u32 = 0x20;
            #[doc = "ADC TRIG6 selected"]
            pub const HWTS6_64: u32 = 0x40;
            #[doc = "ADC TRIG7 selected"]
            pub const HWTS6_128: u32 = 0x80;
        }
    }
    #[doc = "Segment 6 B2B"]
    pub mod B2B6 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable B2B. Wait until delay value defined by TRIG6_COUNTER\\[SAMPLE_INTERVAL\\] is reached"]
            pub const B2B6_0: u32 = 0;
            #[doc = "Enable B2B. When Segment 0 finished (ADC COCO) then automatically trigger next ADC conversion, no need to wait until interval delay reached."]
            pub const B2B6_1: u32 = 0x01;
        }
    }
    #[doc = "Segment 6 interrupt enable. (This bit field is meaningful only when IE6_EN is set)"]
    pub mod IE6 {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Generate interrupt on Done0 when segment 6 finish."]
            pub const IE6_0: u32 = 0;
            #[doc = "Generate interrupt on Done1 when segment 6 finish."]
            pub const IE6_1: u32 = 0x01;
            #[doc = "Generate interrupt on Done2 when segment 6 finish."]
            pub const IE6_2: u32 = 0x02;
            #[doc = "Generate interrupt on Done3 when segment 6 finish."]
            pub const IE6_3: u32 = 0x03;
        }
    }
    #[doc = "IRQ enable of segment 6."]
    pub mod IE6_EN {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt DONE disabled."]
            pub const IE6_EN_0: u32 = 0;
            #[doc = "Interrupt DONE enabled. When segment 6 finish, an interrupt will be generated on the specific port configured by the IE6."]
            pub const IE6_EN_1: u32 = 0x01;
        }
    }
    #[doc = "ADC hardware trigger command selection"]
    pub mod CSEL7 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not a valid selection from the command buffer. Trigger event is ignored."]
            pub const CSEL7_0: u32 = 0;
            #[doc = "ADC CMD1 selected."]
            pub const CSEL7_1: u32 = 0x01;
            #[doc = "ADC CMD2 selected."]
            pub const CSEL7_2: u32 = 0x02;
            #[doc = "ADC CMD3 selected."]
            pub const CSEL7_3: u32 = 0x03;
            #[doc = "ADC CMD4 selected."]
            pub const CSEL7_4: u32 = 0x04;
            #[doc = "ADC CMD5 selected."]
            pub const CSEL7_5: u32 = 0x05;
            #[doc = "ADC CMD6 selected."]
            pub const CSEL7_6: u32 = 0x06;
            #[doc = "ADC CMD7 selected."]
            pub const CSEL7_7: u32 = 0x07;
            #[doc = "ADC CMD8 selected."]
            pub const CSEL7_8: u32 = 0x08;
            #[doc = "ADC CMD9 selected."]
            pub const CSEL7_9: u32 = 0x09;
            #[doc = "ADC CMD10 selected."]
            pub const CSEL7_10: u32 = 0x0a;
            #[doc = "ADC CMD11 selected."]
            pub const CSEL7_11: u32 = 0x0b;
            #[doc = "ADC CMD12 selected."]
            pub const CSEL7_12: u32 = 0x0c;
            #[doc = "ADC CMD13 selected."]
            pub const CSEL7_13: u32 = 0x0d;
            #[doc = "ADC CMD14 selected."]
            pub const CSEL7_14: u32 = 0x0e;
            #[doc = "ADC CMD15 selected."]
            pub const CSEL7_15: u32 = 0x0f;
        }
    }
    #[doc = "Segment 7 HWTS ADC hardware trigger selection"]
    pub mod HWTS7 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "no trigger selected"]
            pub const HWTS7_0: u32 = 0;
            #[doc = "ADC TRIG0 selected"]
            pub const HWTS7_1: u32 = 0x01;
            #[doc = "ADC TRIG1 selected"]
            pub const HWTS7_2: u32 = 0x02;
            #[doc = "ADC TRIG2 selected"]
            pub const HWTS7_4: u32 = 0x04;
            #[doc = "ADC TRIG3 selected"]
            pub const HWTS7_8: u32 = 0x08;
            #[doc = "ADC TRIG4 selected"]
            pub const HWTS7_16: u32 = 0x10;
            #[doc = "ADC TRIG5 selected"]
            pub const HWTS7_32: u32 = 0x20;
            #[doc = "ADC TRIG6 selected"]
            pub const HWTS7_64: u32 = 0x40;
            #[doc = "ADC TRIG7 selected"]
            pub const HWTS7_128: u32 = 0x80;
        }
    }
    #[doc = "Segment 7 B2B"]
    pub mod B2B7 {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable B2B. Wait until delay value defined by TRIG7_COUNTER\\[SAMPLE_INTERVAL\\] is reached"]
            pub const B2B7_0: u32 = 0;
            #[doc = "Enable B2B. When Segment 0 finished (ADC COCO) then automatically trigger next ADC conversion, no need to wait until interval delay reached."]
            pub const B2B7_1: u32 = 0x01;
        }
    }
    #[doc = "Segment 7 interrupt enable. (This bit field is meaningful only when IE7_EN is set)"]
    pub mod IE7 {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Generate interrupt on Done0 when segment 7 finish."]
            pub const IE7_0: u32 = 0;
            #[doc = "Generate interrupt on Done1 when segment 7 finish."]
            pub const IE7_1: u32 = 0x01;
            #[doc = "Generate interrupt on Done2 when segment 7 finish."]
            pub const IE7_2: u32 = 0x02;
            #[doc = "Generate interrupt on Done3 when segment 7 finish."]
            pub const IE7_3: u32 = 0x03;
        }
    }
    #[doc = "IRQ enable of segment 7."]
    pub mod IE7_EN {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt DONE disabled."]
            pub const IE7_EN_0: u32 = 0;
            #[doc = "Interrupt DONE enabled. When segment 7 finish, an interrupt will be generated on the specific port configured by the IE7."]
            pub const IE7_EN_1: u32 = 0x01;
        }
    }
}
#[doc = "ETC_TRIG Result Data 1/0 Register"]
pub mod TRIG5_RESULT_1_0 {
    #[doc = "Result DATA0"]
    pub mod DATA0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Result DATA1"]
    pub mod DATA1 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ETC_TRIG Result Data 3/2 Register"]
pub mod TRIG5_RESULT_3_2 {
    #[doc = "Result DATA2"]
    pub mod DATA2 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Result DATA3"]
    pub mod DATA3 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ETC_TRIG Result Data 5/4 Register"]
pub mod TRIG5_RESULT_5_4 {
    #[doc = "Result DATA4"]
    pub mod DATA4 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Result DATA5"]
    pub mod DATA5 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ETC_TRIG Result Data 7/6 Register"]
pub mod TRIG5_RESULT_7_6 {
    #[doc = "Result DATA6"]
    pub mod DATA6 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Result DATA7"]
    pub mod DATA7 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ETC_TRIG Control Register"]
pub mod TRIG6_CTRL {
    #[doc = "Software trigger. This field is self-clearing."]
    pub mod SW_TRIG {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No software trigger event generated."]
            pub const SW_TRIG_0: u32 = 0;
            #[doc = "Software trigger event generated."]
            pub const SW_TRIG_1: u32 = 0x01;
        }
    }
    #[doc = "Trigger mode selection."]
    pub mod TRIG_MODE {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Hardware trigger. The softerware trigger will be ignored."]
            pub const TRIG_MODE_0: u32 = 0;
            #[doc = "Software trigger. The hardware trigger will be ignored."]
            pub const TRIG_MODE_1: u32 = 0x01;
        }
    }
    #[doc = "The number of segments inside the trigger chain of TRIGa."]
    pub mod TRIG_CHAIN {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Trigger chain length is 1"]
            pub const TRIG_CHAIN_0: u32 = 0;
            #[doc = "Trigger chain length is 2"]
            pub const TRIG_CHAIN_1: u32 = 0x01;
            #[doc = "Trigger chain length is 3"]
            pub const TRIG_CHAIN_2: u32 = 0x02;
            #[doc = "Trigger chain length is 4"]
            pub const TRIG_CHAIN_3: u32 = 0x03;
            #[doc = "Trigger chain length is 5"]
            pub const TRIG_CHAIN_4: u32 = 0x04;
            #[doc = "Trigger chain length is 6"]
            pub const TRIG_CHAIN_5: u32 = 0x05;
            #[doc = "Trigger chain length is 7"]
            pub const TRIG_CHAIN_6: u32 = 0x06;
            #[doc = "Trigger chain length is 8"]
            pub const TRIG_CHAIN_7: u32 = 0x07;
        }
    }
    #[doc = "External trigger priority, 7 is highest priority, while 0 is lowest"]
    pub mod TRIG_PRIORITY {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Trigger synchronization mode selection"]
    pub mod SYNC_MODE {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Synchronization mode disabled, TRIGa and TRIG(a+4) are triggered independently."]
            pub const SYNC_MODE_0: u32 = 0;
            #[doc = "Synchronization mode enabled, TRIGa and TRIG(a+4) are triggered by TRIGa source synchronously."]
            pub const SYNC_MODE_1: u32 = 0x01;
        }
    }
    #[doc = "Segment x done detection"]
    pub mod CHAINX_DONE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "segment x done not detected."]
            pub const CHAINX_DONE_0: u32 = 0;
            #[doc = "segment x done detected."]
            pub const CHAINX_DONE_1: u32 = 0x01;
        }
    }
}
#[doc = "ETC_TRIG Counter Register"]
pub mod TRIG6_COUNTER {
    #[doc = "TRIGGER initial delay counter"]
    pub mod INIT_DELAY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TRIGGER sampling interval counter"]
    pub mod SAMPLE_INTERVAL {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ETC_TRIG Chain 0/1 Register"]
pub mod TRIG6_CHAIN_1_0 {
    #[doc = "ADC hardware trigger command selection"]
    pub mod CSEL0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not a valid selection from the command buffer. Trigger event is ignored."]
            pub const CSEL0_0: u32 = 0;
            #[doc = "ADC CMD1 selected."]
            pub const CSEL0_1: u32 = 0x01;
            #[doc = "ADC CMD2 selected."]
            pub const CSEL0_2: u32 = 0x02;
            #[doc = "ADC CMD3 selected."]
            pub const CSEL0_3: u32 = 0x03;
            #[doc = "ADC CMD4 selected."]
            pub const CSEL0_4: u32 = 0x04;
            #[doc = "ADC CMD5 selected."]
            pub const CSEL0_5: u32 = 0x05;
            #[doc = "ADC CMD6 selected."]
            pub const CSEL0_6: u32 = 0x06;
            #[doc = "ADC CMD7 selected."]
            pub const CSEL0_7: u32 = 0x07;
            #[doc = "ADC CMD8 selected."]
            pub const CSEL0_8: u32 = 0x08;
            #[doc = "ADC CMD9 selected."]
            pub const CSEL0_9: u32 = 0x09;
            #[doc = "ADC CMD10 selected."]
            pub const CSEL0_10: u32 = 0x0a;
            #[doc = "ADC CMD11 selected."]
            pub const CSEL0_11: u32 = 0x0b;
            #[doc = "ADC CMD12 selected."]
            pub const CSEL0_12: u32 = 0x0c;
            #[doc = "ADC CMD13 selected."]
            pub const CSEL0_13: u32 = 0x0d;
            #[doc = "ADC CMD14 selected."]
            pub const CSEL0_14: u32 = 0x0e;
            #[doc = "ADC CMD15 selected."]
            pub const CSEL0_15: u32 = 0x0f;
        }
    }
    #[doc = "Segment 0 HWTS ADC hardware trigger selection"]
    pub mod HWTS0 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "no trigger selected"]
            pub const HWTS0_0: u32 = 0;
            #[doc = "ADC TRIG0 selected"]
            pub const HWTS0_1: u32 = 0x01;
            #[doc = "ADC TRIG1 selected"]
            pub const HWTS0_2: u32 = 0x02;
            #[doc = "ADC TRIG2 selected"]
            pub const HWTS0_4: u32 = 0x04;
            #[doc = "ADC TRIG3 selected"]
            pub const HWTS0_8: u32 = 0x08;
            #[doc = "ADC TRIG4 selected"]
            pub const HWTS0_16: u32 = 0x10;
            #[doc = "ADC TRIG5 selected"]
            pub const HWTS0_32: u32 = 0x20;
            #[doc = "ADC TRIG6 selected"]
            pub const HWTS0_64: u32 = 0x40;
            #[doc = "ADC TRIG7 selected"]
            pub const HWTS0_128: u32 = 0x80;
        }
    }
    #[doc = "Segment 0 B2B"]
    pub mod B2B0 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable B2B. Wait until delay value defined by TRIG0_COUNTER\\[SAMPLE_INTERVAL\\] is reached"]
            pub const B2B0_0: u32 = 0;
            #[doc = "Enable B2B. When Segment 0 finished (ADC COCO) then automatically trigger next ADC conversion, no need to wait until interval delay reached."]
            pub const B2B0_1: u32 = 0x01;
        }
    }
    #[doc = "Segment 0 interrupt enable. (This bit field is meaningful only when IE0_EN is set)"]
    pub mod IE0 {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Generate interrupt on Done0 when segment 0 finish."]
            pub const IE0_0: u32 = 0;
            #[doc = "Generate interrupt on Done1 when segment 0 finish."]
            pub const IE0_1: u32 = 0x01;
            #[doc = "Generate interrupt on Done2 when segment 0 finish."]
            pub const IE0_2: u32 = 0x02;
            #[doc = "Generate interrupt on Done3 when segment 0 finish."]
            pub const IE0_3: u32 = 0x03;
        }
    }
    #[doc = "IRQ enable of segment 0."]
    pub mod IE0_EN {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt DONE disabled."]
            pub const IE0_EN_0: u32 = 0;
            #[doc = "Interrupt DONE enabled. When segment 0 finish, an interrupt will be generated on the specific port configured by the IE0."]
            pub const IE0_EN_1: u32 = 0x01;
        }
    }
    #[doc = "ADC hardware trigger command selection"]
    pub mod CSEL1 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not a valid selection from the command buffer. Trigger event is ignored."]
            pub const CSEL1_0: u32 = 0;
            #[doc = "ADC CMD1 selected."]
            pub const CSEL1_1: u32 = 0x01;
            #[doc = "ADC CMD2 selected."]
            pub const CSEL1_2: u32 = 0x02;
            #[doc = "ADC CMD3 selected."]
            pub const CSEL1_3: u32 = 0x03;
            #[doc = "ADC CMD4 selected."]
            pub const CSEL1_4: u32 = 0x04;
            #[doc = "ADC CMD5 selected."]
            pub const CSEL1_5: u32 = 0x05;
            #[doc = "ADC CMD6 selected."]
            pub const CSEL1_6: u32 = 0x06;
            #[doc = "ADC CMD7 selected."]
            pub const CSEL1_7: u32 = 0x07;
            #[doc = "ADC CMD8 selected."]
            pub const CSEL1_8: u32 = 0x08;
            #[doc = "ADC CMD9 selected."]
            pub const CSEL1_9: u32 = 0x09;
            #[doc = "ADC CMD10 selected."]
            pub const CSEL1_10: u32 = 0x0a;
            #[doc = "ADC CMD11 selected."]
            pub const CSEL1_11: u32 = 0x0b;
            #[doc = "ADC CMD12 selected."]
            pub const CSEL1_12: u32 = 0x0c;
            #[doc = "ADC CMD13 selected."]
            pub const CSEL1_13: u32 = 0x0d;
            #[doc = "ADC CMD14 selected."]
            pub const CSEL1_14: u32 = 0x0e;
            #[doc = "ADC CMD15 selected."]
            pub const CSEL1_15: u32 = 0x0f;
        }
    }
    #[doc = "Segment 1 HWTS ADC hardware trigger selection"]
    pub mod HWTS1 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "no trigger selected"]
            pub const HWTS1_0: u32 = 0;
            #[doc = "ADC TRIG0 selected"]
            pub const HWTS1_1: u32 = 0x01;
            #[doc = "ADC TRIG1 selected"]
            pub const HWTS1_2: u32 = 0x02;
            #[doc = "ADC TRIG2 selected"]
            pub const HWTS1_4: u32 = 0x04;
            #[doc = "ADC TRIG3 selected"]
            pub const HWTS1_8: u32 = 0x08;
            #[doc = "ADC TRIG4 selected"]
            pub const HWTS1_16: u32 = 0x10;
            #[doc = "ADC TRIG5 selected"]
            pub const HWTS1_32: u32 = 0x20;
            #[doc = "ADC TRIG6 selected"]
            pub const HWTS1_64: u32 = 0x40;
            #[doc = "ADC TRIG7 selected"]
            pub const HWTS1_128: u32 = 0x80;
        }
    }
    #[doc = "Segment 1 B2B"]
    pub mod B2B1 {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable B2B. Wait until delay value defined by TRIG1_COUNTER\\[SAMPLE_INTERVAL\\] is reached"]
            pub const B2B1_0: u32 = 0;
            #[doc = "Enable B2B. When Segment 0 finished (ADC COCO) then automatically trigger next ADC conversion, no need to wait until interval delay reached."]
            pub const B2B1_1: u32 = 0x01;
        }
    }
    #[doc = "Segment 1 interrupt enable. (This bit field is meaningful only when IE1_EN is set)"]
    pub mod IE1 {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Generate interrupt on Done0 when Segment 1 finish."]
            pub const IE1_0: u32 = 0;
            #[doc = "Generate interrupt on Done1 when Segment 1 finish."]
            pub const IE1_1: u32 = 0x01;
            #[doc = "Generate interrupt on Done2 when Segment 1 finish."]
            pub const IE1_2: u32 = 0x02;
            #[doc = "Generate interrupt on Done3 when Segment 1 finish."]
            pub const IE1_3: u32 = 0x03;
        }
    }
    #[doc = "IRQ enable of segment 1."]
    pub mod IE1_EN {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt DONE disabled."]
            pub const IE1_EN_0: u32 = 0;
            #[doc = "Interrupt DONE enabled. When segment 1 finish, an interrupt will be generated on the specific port configured by the IE1."]
            pub const IE1_EN_1: u32 = 0x01;
        }
    }
}
#[doc = "ETC_TRIG Chain 2/3 Register"]
pub mod TRIG6_CHAIN_3_2 {
    #[doc = "ADC hardware trigger command selection"]
    pub mod CSEL2 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not a valid selection from the command buffer. Trigger event is ignored."]
            pub const CSEL2_0: u32 = 0;
            #[doc = "ADC CMD1 selected."]
            pub const CSEL2_1: u32 = 0x01;
            #[doc = "ADC CMD2 selected."]
            pub const CSEL2_2: u32 = 0x02;
            #[doc = "ADC CMD3 selected."]
            pub const CSEL2_3: u32 = 0x03;
            #[doc = "ADC CMD4 selected."]
            pub const CSEL2_4: u32 = 0x04;
            #[doc = "ADC CMD5 selected."]
            pub const CSEL2_5: u32 = 0x05;
            #[doc = "ADC CMD6 selected."]
            pub const CSEL2_6: u32 = 0x06;
            #[doc = "ADC CMD7 selected."]
            pub const CSEL2_7: u32 = 0x07;
            #[doc = "ADC CMD8 selected."]
            pub const CSEL2_8: u32 = 0x08;
            #[doc = "ADC CMD9 selected."]
            pub const CSEL2_9: u32 = 0x09;
            #[doc = "ADC CMD10 selected."]
            pub const CSEL2_10: u32 = 0x0a;
            #[doc = "ADC CMD11 selected."]
            pub const CSEL2_11: u32 = 0x0b;
            #[doc = "ADC CMD12 selected."]
            pub const CSEL2_12: u32 = 0x0c;
            #[doc = "ADC CMD13 selected."]
            pub const CSEL2_13: u32 = 0x0d;
            #[doc = "ADC CMD14 selected."]
            pub const CSEL2_14: u32 = 0x0e;
            #[doc = "ADC CMD15 selected."]
            pub const CSEL2_15: u32 = 0x0f;
        }
    }
    #[doc = "Segment 2 HWTS ADC hardware trigger selection"]
    pub mod HWTS2 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "no trigger selected"]
            pub const HWTS2_0: u32 = 0;
            #[doc = "ADC TRIG0 selected"]
            pub const HWTS2_1: u32 = 0x01;
            #[doc = "ADC TRIG1 selected"]
            pub const HWTS2_2: u32 = 0x02;
            #[doc = "ADC TRIG2 selected"]
            pub const HWTS2_4: u32 = 0x04;
            #[doc = "ADC TRIG3 selected"]
            pub const HWTS2_8: u32 = 0x08;
            #[doc = "ADC TRIG4 selected"]
            pub const HWTS2_16: u32 = 0x10;
            #[doc = "ADC TRIG5 selected"]
            pub const HWTS2_32: u32 = 0x20;
            #[doc = "ADC TRIG6 selected"]
            pub const HWTS2_64: u32 = 0x40;
            #[doc = "ADC TRIG7 selected"]
            pub const HWTS2_128: u32 = 0x80;
        }
    }
    #[doc = "Segment 2 B2B"]
    pub mod B2B2 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable B2B. Wait until delay value defined by TRIG2_COUNTER\\[SAMPLE_INTERVAL\\] is reached"]
            pub const B2B2_0: u32 = 0;
            #[doc = "Enable B2B. When Segment 0 finished (ADC COCO) then automatically trigger next ADC conversion, no need to wait until interval delay reached."]
            pub const B2B2_1: u32 = 0x01;
        }
    }
    #[doc = "Segment 2 interrupt enable. (This bit field is meaningful only when IE2_EN is set)"]
    pub mod IE2 {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Generate interrupt on Done0 when segment 2 finish."]
            pub const IE2_0: u32 = 0;
            #[doc = "Generate interrupt on Done1 when segment 2 finish."]
            pub const IE2_1: u32 = 0x01;
            #[doc = "Generate interrupt on Done2 when segment 2 finish."]
            pub const IE2_2: u32 = 0x02;
            #[doc = "Generate interrupt on Done3 when segment 2 finish."]
            pub const IE2_3: u32 = 0x03;
        }
    }
    #[doc = "IRQ enable of segment 2."]
    pub mod IE2_EN {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt DONE disabled."]
            pub const IE2_EN_0: u32 = 0;
            #[doc = "Interrupt DONE enabled. When segment 2 finish, an interrupt will be generated on the specific port configured by the IE2."]
            pub const IE2_EN_1: u32 = 0x01;
        }
    }
    #[doc = "ADC hardware trigger command selection"]
    pub mod CSEL3 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not a valid selection from the command buffer. Trigger event is ignored."]
            pub const CSEL3_0: u32 = 0;
            #[doc = "ADC CMD1 selected."]
            pub const CSEL3_1: u32 = 0x01;
            #[doc = "ADC CMD2 selected."]
            pub const CSEL3_2: u32 = 0x02;
            #[doc = "ADC CMD3 selected."]
            pub const CSEL3_3: u32 = 0x03;
            #[doc = "ADC CMD4 selected."]
            pub const CSEL3_4: u32 = 0x04;
            #[doc = "ADC CMD5 selected."]
            pub const CSEL3_5: u32 = 0x05;
            #[doc = "ADC CMD6 selected."]
            pub const CSEL3_6: u32 = 0x06;
            #[doc = "ADC CMD7 selected."]
            pub const CSEL3_7: u32 = 0x07;
            #[doc = "ADC CMD8 selected."]
            pub const CSEL3_8: u32 = 0x08;
            #[doc = "ADC CMD9 selected."]
            pub const CSEL3_9: u32 = 0x09;
            #[doc = "ADC CMD10 selected."]
            pub const CSEL3_10: u32 = 0x0a;
            #[doc = "ADC CMD11 selected."]
            pub const CSEL3_11: u32 = 0x0b;
            #[doc = "ADC CMD12 selected."]
            pub const CSEL3_12: u32 = 0x0c;
            #[doc = "ADC CMD13 selected."]
            pub const CSEL3_13: u32 = 0x0d;
            #[doc = "ADC CMD14 selected."]
            pub const CSEL3_14: u32 = 0x0e;
            #[doc = "ADC CMD15 selected."]
            pub const CSEL3_15: u32 = 0x0f;
        }
    }
    #[doc = "Segment 3 HWTS ADC hardware trigger selection"]
    pub mod HWTS3 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "no trigger selected"]
            pub const HWTS3_0: u32 = 0;
            #[doc = "ADC TRIG0 selected"]
            pub const HWTS3_1: u32 = 0x01;
            #[doc = "ADC TRIG1 selected"]
            pub const HWTS3_2: u32 = 0x02;
            #[doc = "ADC TRIG2 selected"]
            pub const HWTS3_4: u32 = 0x04;
            #[doc = "ADC TRIG3 selected"]
            pub const HWTS3_8: u32 = 0x08;
            #[doc = "ADC TRIG4 selected"]
            pub const HWTS3_16: u32 = 0x10;
            #[doc = "ADC TRIG5 selected"]
            pub const HWTS3_32: u32 = 0x20;
            #[doc = "ADC TRIG6 selected"]
            pub const HWTS3_64: u32 = 0x40;
            #[doc = "ADC TRIG7 selected"]
            pub const HWTS3_128: u32 = 0x80;
        }
    }
    #[doc = "Segment 3 B2B"]
    pub mod B2B3 {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable B2B. Wait until delay value defined by TRIG3_COUNTER\\[SAMPLE_INTERVAL\\] is reached"]
            pub const B2B3_0: u32 = 0;
            #[doc = "Enable B2B. When Segment 0 finished (ADC COCO) then automatically trigger next ADC conversion, no need to wait until interval delay reached."]
            pub const B2B3_1: u32 = 0x01;
        }
    }
    #[doc = "Segment 3 interrupt enable. (This bit field is meaningful only when IE3_EN is set)"]
    pub mod IE3 {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Generate interrupt on Done0 when segment 3 finish."]
            pub const IE3_0: u32 = 0;
            #[doc = "Generate interrupt on Done1 when segment 3 finish."]
            pub const IE3_1: u32 = 0x01;
            #[doc = "Generate interrupt on Done2 when segment 3 finish."]
            pub const IE3_2: u32 = 0x02;
            #[doc = "Generate interrupt on Done3 when segment 3 finish."]
            pub const IE3_3: u32 = 0x03;
        }
    }
    #[doc = "IRQ enable of segment 3."]
    pub mod IE3_EN {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt DONE disabled."]
            pub const IE3_EN_0: u32 = 0;
            #[doc = "Interrupt DONE enabled. When segment 3 finish, an interrupt will be generated on the specific port configured by the IE3."]
            pub const IE3_EN_1: u32 = 0x01;
        }
    }
}
#[doc = "ETC_TRIG Chain 4/5 Register"]
pub mod TRIG6_CHAIN_5_4 {
    #[doc = "ADC hardware trigger command selection"]
    pub mod CSEL4 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not a valid selection from the command buffer. Trigger event is ignored."]
            pub const CSEL4_0: u32 = 0;
            #[doc = "ADC CMD1 selected."]
            pub const CSEL4_1: u32 = 0x01;
            #[doc = "ADC CMD2 selected."]
            pub const CSEL4_2: u32 = 0x02;
            #[doc = "ADC CMD3 selected."]
            pub const CSEL4_3: u32 = 0x03;
            #[doc = "ADC CMD4 selected."]
            pub const CSEL4_4: u32 = 0x04;
            #[doc = "ADC CMD5 selected."]
            pub const CSEL4_5: u32 = 0x05;
            #[doc = "ADC CMD6 selected."]
            pub const CSEL4_6: u32 = 0x06;
            #[doc = "ADC CMD7 selected."]
            pub const CSEL4_7: u32 = 0x07;
            #[doc = "ADC CMD8 selected."]
            pub const CSEL4_8: u32 = 0x08;
            #[doc = "ADC CMD9 selected."]
            pub const CSEL4_9: u32 = 0x09;
            #[doc = "ADC CMD10 selected."]
            pub const CSEL4_10: u32 = 0x0a;
            #[doc = "ADC CMD11 selected."]
            pub const CSEL4_11: u32 = 0x0b;
            #[doc = "ADC CMD12 selected."]
            pub const CSEL4_12: u32 = 0x0c;
            #[doc = "ADC CMD13 selected."]
            pub const CSEL4_13: u32 = 0x0d;
            #[doc = "ADC CMD14 selected."]
            pub const CSEL4_14: u32 = 0x0e;
            #[doc = "ADC CMD15 selected."]
            pub const CSEL4_15: u32 = 0x0f;
        }
    }
    #[doc = "Segment 4 HWTS ADC hardware trigger selection"]
    pub mod HWTS4 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "no trigger selected"]
            pub const HWTS4_0: u32 = 0;
            #[doc = "ADC TRIG0 selected"]
            pub const HWTS4_1: u32 = 0x01;
            #[doc = "ADC TRIG1 selected"]
            pub const HWTS4_2: u32 = 0x02;
            #[doc = "ADC TRIG2 selected"]
            pub const HWTS4_4: u32 = 0x04;
            #[doc = "ADC TRIG3 selected"]
            pub const HWTS4_8: u32 = 0x08;
            #[doc = "ADC TRIG4 selected"]
            pub const HWTS4_16: u32 = 0x10;
            #[doc = "ADC TRIG5 selected"]
            pub const HWTS4_32: u32 = 0x20;
            #[doc = "ADC TRIG6 selected"]
            pub const HWTS4_64: u32 = 0x40;
            #[doc = "ADC TRIG7 selected"]
            pub const HWTS4_128: u32 = 0x80;
        }
    }
    #[doc = "Segment 4 B2B"]
    pub mod B2B4 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable B2B. Wait until delay value defined by TRIG4_COUNTER\\[SAMPLE_INTERVAL\\] is reached"]
            pub const B2B4_0: u32 = 0;
            #[doc = "Enable B2B. When Segment 0 finished (ADC COCO) then automatically trigger next ADC conversion, no need to wait until interval delay reached."]
            pub const B2B4_1: u32 = 0x01;
        }
    }
    #[doc = "Segment 4 interrupt enable. (This bit field is meaningful only when IE4_EN is set)"]
    pub mod IE4 {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Generate interrupt on Done0 when segment 4 finish."]
            pub const IE4_0: u32 = 0;
            #[doc = "Generate interrupt on Done1 when segment 4 finish."]
            pub const IE4_1: u32 = 0x01;
            #[doc = "Generate interrupt on Done2 when segment 4 finish."]
            pub const IE4_2: u32 = 0x02;
            #[doc = "Generate interrupt on Done3 when segment 4 finish."]
            pub const IE4_3: u32 = 0x03;
        }
    }
    #[doc = "IRQ enable of segment 4."]
    pub mod IE4_EN {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt DONE disabled."]
            pub const IE4_EN_0: u32 = 0;
            #[doc = "Interrupt DONE enabled. When segment 4 finish, an interrupt will be generated on the specific port configured by the IE4."]
            pub const IE4_EN_1: u32 = 0x01;
        }
    }
    #[doc = "ADC hardware trigger command selection"]
    pub mod CSEL5 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not a valid selection from the command buffer. Trigger event is ignored."]
            pub const CSEL5_0: u32 = 0;
            #[doc = "ADC CMD1 selected."]
            pub const CSEL5_1: u32 = 0x01;
            #[doc = "ADC CMD2 selected."]
            pub const CSEL5_2: u32 = 0x02;
            #[doc = "ADC CMD3 selected."]
            pub const CSEL5_3: u32 = 0x03;
            #[doc = "ADC CMD4 selected."]
            pub const CSEL5_4: u32 = 0x04;
            #[doc = "ADC CMD5 selected."]
            pub const CSEL5_5: u32 = 0x05;
            #[doc = "ADC CMD6 selected."]
            pub const CSEL5_6: u32 = 0x06;
            #[doc = "ADC CMD7 selected."]
            pub const CSEL5_7: u32 = 0x07;
            #[doc = "ADC CMD8 selected."]
            pub const CSEL5_8: u32 = 0x08;
            #[doc = "ADC CMD9 selected."]
            pub const CSEL5_9: u32 = 0x09;
            #[doc = "ADC CMD10 selected."]
            pub const CSEL5_10: u32 = 0x0a;
            #[doc = "ADC CMD11 selected."]
            pub const CSEL5_11: u32 = 0x0b;
            #[doc = "ADC CMD12 selected."]
            pub const CSEL5_12: u32 = 0x0c;
            #[doc = "ADC CMD13 selected."]
            pub const CSEL5_13: u32 = 0x0d;
            #[doc = "ADC CMD14 selected."]
            pub const CSEL5_14: u32 = 0x0e;
            #[doc = "ADC CMD15 selected."]
            pub const CSEL5_15: u32 = 0x0f;
        }
    }
    #[doc = "Segment 5 HWTS ADC hardware trigger selection"]
    pub mod HWTS5 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "no trigger selected"]
            pub const HWTS5_0: u32 = 0;
            #[doc = "ADC TRIG0 selected"]
            pub const HWTS5_1: u32 = 0x01;
            #[doc = "ADC TRIG1 selected"]
            pub const HWTS5_2: u32 = 0x02;
            #[doc = "ADC TRIG2 selected"]
            pub const HWTS5_4: u32 = 0x04;
            #[doc = "ADC TRIG3 selected"]
            pub const HWTS5_8: u32 = 0x08;
            #[doc = "ADC TRIG4 selected"]
            pub const HWTS5_16: u32 = 0x10;
            #[doc = "ADC TRIG5 selected"]
            pub const HWTS5_32: u32 = 0x20;
            #[doc = "ADC TRIG6 selected"]
            pub const HWTS5_64: u32 = 0x40;
            #[doc = "ADC TRIG7 selected"]
            pub const HWTS5_128: u32 = 0x80;
        }
    }
    #[doc = "Segment 5 B2B"]
    pub mod B2B5 {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable B2B. Wait until delay value defined by TRIG5_COUNTER\\[SAMPLE_INTERVAL\\] is reached"]
            pub const B2B5_0: u32 = 0;
            #[doc = "Enable B2B. When Segment 0 finished (ADC COCO) then automatically trigger next ADC conversion, no need to wait until interval delay reached."]
            pub const B2B5_1: u32 = 0x01;
        }
    }
    #[doc = "Segment 5 interrupt enable. (This bit field is meaningful only when IE5_EN is set)"]
    pub mod IE5 {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Generate interrupt on Done0 when segment 5 finish."]
            pub const IE5_0: u32 = 0;
            #[doc = "Generate interrupt on Done1 when segment 5 finish."]
            pub const IE5_1: u32 = 0x01;
            #[doc = "Generate interrupt on Done2 when segment 5 finish."]
            pub const IE5_2: u32 = 0x02;
            #[doc = "Generate interrupt on Done3 when segment 5 finish."]
            pub const IE5_3: u32 = 0x03;
        }
    }
    #[doc = "IRQ enable of segment 5."]
    pub mod IE5_EN {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt DONE disabled."]
            pub const IE5_EN_0: u32 = 0;
            #[doc = "Interrupt DONE enabled. When segment 5 finish, an interrupt will be generated on the specific port configured by the IE5."]
            pub const IE5_EN_1: u32 = 0x01;
        }
    }
}
#[doc = "ETC_TRIG Chain 6/7 Register"]
pub mod TRIG6_CHAIN_7_6 {
    #[doc = "ADC hardware trigger command selection"]
    pub mod CSEL6 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not a valid selection from the command buffer. Trigger event is ignored."]
            pub const CSEL6_0: u32 = 0;
            #[doc = "ADC CMD1 selected."]
            pub const CSEL6_1: u32 = 0x01;
            #[doc = "ADC CMD2 selected."]
            pub const CSEL6_2: u32 = 0x02;
            #[doc = "ADC CMD3 selected."]
            pub const CSEL6_3: u32 = 0x03;
            #[doc = "ADC CMD4 selected."]
            pub const CSEL6_4: u32 = 0x04;
            #[doc = "ADC CMD5 selected."]
            pub const CSEL6_5: u32 = 0x05;
            #[doc = "ADC CMD6 selected."]
            pub const CSEL6_6: u32 = 0x06;
            #[doc = "ADC CMD7 selected."]
            pub const CSEL6_7: u32 = 0x07;
            #[doc = "ADC CMD8 selected."]
            pub const CSEL6_8: u32 = 0x08;
            #[doc = "ADC CMD9 selected."]
            pub const CSEL6_9: u32 = 0x09;
            #[doc = "ADC CMD10 selected."]
            pub const CSEL6_10: u32 = 0x0a;
            #[doc = "ADC CMD11 selected."]
            pub const CSEL6_11: u32 = 0x0b;
            #[doc = "ADC CMD12 selected."]
            pub const CSEL6_12: u32 = 0x0c;
            #[doc = "ADC CMD13 selected."]
            pub const CSEL6_13: u32 = 0x0d;
            #[doc = "ADC CMD14 selected."]
            pub const CSEL6_14: u32 = 0x0e;
            #[doc = "ADC CMD15 selected."]
            pub const CSEL6_15: u32 = 0x0f;
        }
    }
    #[doc = "Segment 6 HWTS ADC hardware trigger selection"]
    pub mod HWTS6 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "no trigger selected"]
            pub const HWTS6_0: u32 = 0;
            #[doc = "ADC TRIG0 selected"]
            pub const HWTS6_1: u32 = 0x01;
            #[doc = "ADC TRIG1 selected"]
            pub const HWTS6_2: u32 = 0x02;
            #[doc = "ADC TRIG2 selected"]
            pub const HWTS6_4: u32 = 0x04;
            #[doc = "ADC TRIG3 selected"]
            pub const HWTS6_8: u32 = 0x08;
            #[doc = "ADC TRIG4 selected"]
            pub const HWTS6_16: u32 = 0x10;
            #[doc = "ADC TRIG5 selected"]
            pub const HWTS6_32: u32 = 0x20;
            #[doc = "ADC TRIG6 selected"]
            pub const HWTS6_64: u32 = 0x40;
            #[doc = "ADC TRIG7 selected"]
            pub const HWTS6_128: u32 = 0x80;
        }
    }
    #[doc = "Segment 6 B2B"]
    pub mod B2B6 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable B2B. Wait until delay value defined by TRIG6_COUNTER\\[SAMPLE_INTERVAL\\] is reached"]
            pub const B2B6_0: u32 = 0;
            #[doc = "Enable B2B. When Segment 0 finished (ADC COCO) then automatically trigger next ADC conversion, no need to wait until interval delay reached."]
            pub const B2B6_1: u32 = 0x01;
        }
    }
    #[doc = "Segment 6 interrupt enable. (This bit field is meaningful only when IE6_EN is set)"]
    pub mod IE6 {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Generate interrupt on Done0 when segment 6 finish."]
            pub const IE6_0: u32 = 0;
            #[doc = "Generate interrupt on Done1 when segment 6 finish."]
            pub const IE6_1: u32 = 0x01;
            #[doc = "Generate interrupt on Done2 when segment 6 finish."]
            pub const IE6_2: u32 = 0x02;
            #[doc = "Generate interrupt on Done3 when segment 6 finish."]
            pub const IE6_3: u32 = 0x03;
        }
    }
    #[doc = "IRQ enable of segment 6."]
    pub mod IE6_EN {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt DONE disabled."]
            pub const IE6_EN_0: u32 = 0;
            #[doc = "Interrupt DONE enabled. When segment 6 finish, an interrupt will be generated on the specific port configured by the IE6."]
            pub const IE6_EN_1: u32 = 0x01;
        }
    }
    #[doc = "ADC hardware trigger command selection"]
    pub mod CSEL7 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not a valid selection from the command buffer. Trigger event is ignored."]
            pub const CSEL7_0: u32 = 0;
            #[doc = "ADC CMD1 selected."]
            pub const CSEL7_1: u32 = 0x01;
            #[doc = "ADC CMD2 selected."]
            pub const CSEL7_2: u32 = 0x02;
            #[doc = "ADC CMD3 selected."]
            pub const CSEL7_3: u32 = 0x03;
            #[doc = "ADC CMD4 selected."]
            pub const CSEL7_4: u32 = 0x04;
            #[doc = "ADC CMD5 selected."]
            pub const CSEL7_5: u32 = 0x05;
            #[doc = "ADC CMD6 selected."]
            pub const CSEL7_6: u32 = 0x06;
            #[doc = "ADC CMD7 selected."]
            pub const CSEL7_7: u32 = 0x07;
            #[doc = "ADC CMD8 selected."]
            pub const CSEL7_8: u32 = 0x08;
            #[doc = "ADC CMD9 selected."]
            pub const CSEL7_9: u32 = 0x09;
            #[doc = "ADC CMD10 selected."]
            pub const CSEL7_10: u32 = 0x0a;
            #[doc = "ADC CMD11 selected."]
            pub const CSEL7_11: u32 = 0x0b;
            #[doc = "ADC CMD12 selected."]
            pub const CSEL7_12: u32 = 0x0c;
            #[doc = "ADC CMD13 selected."]
            pub const CSEL7_13: u32 = 0x0d;
            #[doc = "ADC CMD14 selected."]
            pub const CSEL7_14: u32 = 0x0e;
            #[doc = "ADC CMD15 selected."]
            pub const CSEL7_15: u32 = 0x0f;
        }
    }
    #[doc = "Segment 7 HWTS ADC hardware trigger selection"]
    pub mod HWTS7 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "no trigger selected"]
            pub const HWTS7_0: u32 = 0;
            #[doc = "ADC TRIG0 selected"]
            pub const HWTS7_1: u32 = 0x01;
            #[doc = "ADC TRIG1 selected"]
            pub const HWTS7_2: u32 = 0x02;
            #[doc = "ADC TRIG2 selected"]
            pub const HWTS7_4: u32 = 0x04;
            #[doc = "ADC TRIG3 selected"]
            pub const HWTS7_8: u32 = 0x08;
            #[doc = "ADC TRIG4 selected"]
            pub const HWTS7_16: u32 = 0x10;
            #[doc = "ADC TRIG5 selected"]
            pub const HWTS7_32: u32 = 0x20;
            #[doc = "ADC TRIG6 selected"]
            pub const HWTS7_64: u32 = 0x40;
            #[doc = "ADC TRIG7 selected"]
            pub const HWTS7_128: u32 = 0x80;
        }
    }
    #[doc = "Segment 7 B2B"]
    pub mod B2B7 {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable B2B. Wait until delay value defined by TRIG7_COUNTER\\[SAMPLE_INTERVAL\\] is reached"]
            pub const B2B7_0: u32 = 0;
            #[doc = "Enable B2B. When Segment 0 finished (ADC COCO) then automatically trigger next ADC conversion, no need to wait until interval delay reached."]
            pub const B2B7_1: u32 = 0x01;
        }
    }
    #[doc = "Segment 7 interrupt enable. (This bit field is meaningful only when IE7_EN is set)"]
    pub mod IE7 {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Generate interrupt on Done0 when segment 7 finish."]
            pub const IE7_0: u32 = 0;
            #[doc = "Generate interrupt on Done1 when segment 7 finish."]
            pub const IE7_1: u32 = 0x01;
            #[doc = "Generate interrupt on Done2 when segment 7 finish."]
            pub const IE7_2: u32 = 0x02;
            #[doc = "Generate interrupt on Done3 when segment 7 finish."]
            pub const IE7_3: u32 = 0x03;
        }
    }
    #[doc = "IRQ enable of segment 7."]
    pub mod IE7_EN {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt DONE disabled."]
            pub const IE7_EN_0: u32 = 0;
            #[doc = "Interrupt DONE enabled. When segment 7 finish, an interrupt will be generated on the specific port configured by the IE7."]
            pub const IE7_EN_1: u32 = 0x01;
        }
    }
}
#[doc = "ETC_TRIG Result Data 1/0 Register"]
pub mod TRIG6_RESULT_1_0 {
    #[doc = "Result DATA0"]
    pub mod DATA0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Result DATA1"]
    pub mod DATA1 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ETC_TRIG Result Data 3/2 Register"]
pub mod TRIG6_RESULT_3_2 {
    #[doc = "Result DATA2"]
    pub mod DATA2 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Result DATA3"]
    pub mod DATA3 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ETC_TRIG Result Data 5/4 Register"]
pub mod TRIG6_RESULT_5_4 {
    #[doc = "Result DATA4"]
    pub mod DATA4 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Result DATA5"]
    pub mod DATA5 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ETC_TRIG Result Data 7/6 Register"]
pub mod TRIG6_RESULT_7_6 {
    #[doc = "Result DATA6"]
    pub mod DATA6 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Result DATA7"]
    pub mod DATA7 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ETC_TRIG Control Register"]
pub mod TRIG7_CTRL {
    #[doc = "Software trigger. This field is self-clearing."]
    pub mod SW_TRIG {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No software trigger event generated."]
            pub const SW_TRIG_0: u32 = 0;
            #[doc = "Software trigger event generated."]
            pub const SW_TRIG_1: u32 = 0x01;
        }
    }
    #[doc = "Trigger mode selection."]
    pub mod TRIG_MODE {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Hardware trigger. The softerware trigger will be ignored."]
            pub const TRIG_MODE_0: u32 = 0;
            #[doc = "Software trigger. The hardware trigger will be ignored."]
            pub const TRIG_MODE_1: u32 = 0x01;
        }
    }
    #[doc = "The number of segments inside the trigger chain of TRIGa."]
    pub mod TRIG_CHAIN {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Trigger chain length is 1"]
            pub const TRIG_CHAIN_0: u32 = 0;
            #[doc = "Trigger chain length is 2"]
            pub const TRIG_CHAIN_1: u32 = 0x01;
            #[doc = "Trigger chain length is 3"]
            pub const TRIG_CHAIN_2: u32 = 0x02;
            #[doc = "Trigger chain length is 4"]
            pub const TRIG_CHAIN_3: u32 = 0x03;
            #[doc = "Trigger chain length is 5"]
            pub const TRIG_CHAIN_4: u32 = 0x04;
            #[doc = "Trigger chain length is 6"]
            pub const TRIG_CHAIN_5: u32 = 0x05;
            #[doc = "Trigger chain length is 7"]
            pub const TRIG_CHAIN_6: u32 = 0x06;
            #[doc = "Trigger chain length is 8"]
            pub const TRIG_CHAIN_7: u32 = 0x07;
        }
    }
    #[doc = "External trigger priority, 7 is highest priority, while 0 is lowest"]
    pub mod TRIG_PRIORITY {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Trigger synchronization mode selection"]
    pub mod SYNC_MODE {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Synchronization mode disabled, TRIGa and TRIG(a+4) are triggered independently."]
            pub const SYNC_MODE_0: u32 = 0;
            #[doc = "Synchronization mode enabled, TRIGa and TRIG(a+4) are triggered by TRIGa source synchronously."]
            pub const SYNC_MODE_1: u32 = 0x01;
        }
    }
    #[doc = "Segment x done detection"]
    pub mod CHAINX_DONE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "segment x done not detected."]
            pub const CHAINX_DONE_0: u32 = 0;
            #[doc = "segment x done detected."]
            pub const CHAINX_DONE_1: u32 = 0x01;
        }
    }
}
#[doc = "ETC_TRIG Counter Register"]
pub mod TRIG7_COUNTER {
    #[doc = "TRIGGER initial delay counter"]
    pub mod INIT_DELAY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TRIGGER sampling interval counter"]
    pub mod SAMPLE_INTERVAL {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ETC_TRIG Chain 0/1 Register"]
pub mod TRIG7_CHAIN_1_0 {
    #[doc = "ADC hardware trigger command selection"]
    pub mod CSEL0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not a valid selection from the command buffer. Trigger event is ignored."]
            pub const CSEL0_0: u32 = 0;
            #[doc = "ADC CMD1 selected."]
            pub const CSEL0_1: u32 = 0x01;
            #[doc = "ADC CMD2 selected."]
            pub const CSEL0_2: u32 = 0x02;
            #[doc = "ADC CMD3 selected."]
            pub const CSEL0_3: u32 = 0x03;
            #[doc = "ADC CMD4 selected."]
            pub const CSEL0_4: u32 = 0x04;
            #[doc = "ADC CMD5 selected."]
            pub const CSEL0_5: u32 = 0x05;
            #[doc = "ADC CMD6 selected."]
            pub const CSEL0_6: u32 = 0x06;
            #[doc = "ADC CMD7 selected."]
            pub const CSEL0_7: u32 = 0x07;
            #[doc = "ADC CMD8 selected."]
            pub const CSEL0_8: u32 = 0x08;
            #[doc = "ADC CMD9 selected."]
            pub const CSEL0_9: u32 = 0x09;
            #[doc = "ADC CMD10 selected."]
            pub const CSEL0_10: u32 = 0x0a;
            #[doc = "ADC CMD11 selected."]
            pub const CSEL0_11: u32 = 0x0b;
            #[doc = "ADC CMD12 selected."]
            pub const CSEL0_12: u32 = 0x0c;
            #[doc = "ADC CMD13 selected."]
            pub const CSEL0_13: u32 = 0x0d;
            #[doc = "ADC CMD14 selected."]
            pub const CSEL0_14: u32 = 0x0e;
            #[doc = "ADC CMD15 selected."]
            pub const CSEL0_15: u32 = 0x0f;
        }
    }
    #[doc = "Segment 0 HWTS ADC hardware trigger selection"]
    pub mod HWTS0 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "no trigger selected"]
            pub const HWTS0_0: u32 = 0;
            #[doc = "ADC TRIG0 selected"]
            pub const HWTS0_1: u32 = 0x01;
            #[doc = "ADC TRIG1 selected"]
            pub const HWTS0_2: u32 = 0x02;
            #[doc = "ADC TRIG2 selected"]
            pub const HWTS0_4: u32 = 0x04;
            #[doc = "ADC TRIG3 selected"]
            pub const HWTS0_8: u32 = 0x08;
            #[doc = "ADC TRIG4 selected"]
            pub const HWTS0_16: u32 = 0x10;
            #[doc = "ADC TRIG5 selected"]
            pub const HWTS0_32: u32 = 0x20;
            #[doc = "ADC TRIG6 selected"]
            pub const HWTS0_64: u32 = 0x40;
            #[doc = "ADC TRIG7 selected"]
            pub const HWTS0_128: u32 = 0x80;
        }
    }
    #[doc = "Segment 0 B2B"]
    pub mod B2B0 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable B2B. Wait until delay value defined by TRIG0_COUNTER\\[SAMPLE_INTERVAL\\] is reached"]
            pub const B2B0_0: u32 = 0;
            #[doc = "Enable B2B. When Segment 0 finished (ADC COCO) then automatically trigger next ADC conversion, no need to wait until interval delay reached."]
            pub const B2B0_1: u32 = 0x01;
        }
    }
    #[doc = "Segment 0 interrupt enable. (This bit field is meaningful only when IE0_EN is set)"]
    pub mod IE0 {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Generate interrupt on Done0 when segment 0 finish."]
            pub const IE0_0: u32 = 0;
            #[doc = "Generate interrupt on Done1 when segment 0 finish."]
            pub const IE0_1: u32 = 0x01;
            #[doc = "Generate interrupt on Done2 when segment 0 finish."]
            pub const IE0_2: u32 = 0x02;
            #[doc = "Generate interrupt on Done3 when segment 0 finish."]
            pub const IE0_3: u32 = 0x03;
        }
    }
    #[doc = "IRQ enable of segment 0."]
    pub mod IE0_EN {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt DONE disabled."]
            pub const IE0_EN_0: u32 = 0;
            #[doc = "Interrupt DONE enabled. When segment 0 finish, an interrupt will be generated on the specific port configured by the IE0."]
            pub const IE0_EN_1: u32 = 0x01;
        }
    }
    #[doc = "ADC hardware trigger command selection"]
    pub mod CSEL1 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not a valid selection from the command buffer. Trigger event is ignored."]
            pub const CSEL1_0: u32 = 0;
            #[doc = "ADC CMD1 selected."]
            pub const CSEL1_1: u32 = 0x01;
            #[doc = "ADC CMD2 selected."]
            pub const CSEL1_2: u32 = 0x02;
            #[doc = "ADC CMD3 selected."]
            pub const CSEL1_3: u32 = 0x03;
            #[doc = "ADC CMD4 selected."]
            pub const CSEL1_4: u32 = 0x04;
            #[doc = "ADC CMD5 selected."]
            pub const CSEL1_5: u32 = 0x05;
            #[doc = "ADC CMD6 selected."]
            pub const CSEL1_6: u32 = 0x06;
            #[doc = "ADC CMD7 selected."]
            pub const CSEL1_7: u32 = 0x07;
            #[doc = "ADC CMD8 selected."]
            pub const CSEL1_8: u32 = 0x08;
            #[doc = "ADC CMD9 selected."]
            pub const CSEL1_9: u32 = 0x09;
            #[doc = "ADC CMD10 selected."]
            pub const CSEL1_10: u32 = 0x0a;
            #[doc = "ADC CMD11 selected."]
            pub const CSEL1_11: u32 = 0x0b;
            #[doc = "ADC CMD12 selected."]
            pub const CSEL1_12: u32 = 0x0c;
            #[doc = "ADC CMD13 selected."]
            pub const CSEL1_13: u32 = 0x0d;
            #[doc = "ADC CMD14 selected."]
            pub const CSEL1_14: u32 = 0x0e;
            #[doc = "ADC CMD15 selected."]
            pub const CSEL1_15: u32 = 0x0f;
        }
    }
    #[doc = "Segment 1 HWTS ADC hardware trigger selection"]
    pub mod HWTS1 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "no trigger selected"]
            pub const HWTS1_0: u32 = 0;
            #[doc = "ADC TRIG0 selected"]
            pub const HWTS1_1: u32 = 0x01;
            #[doc = "ADC TRIG1 selected"]
            pub const HWTS1_2: u32 = 0x02;
            #[doc = "ADC TRIG2 selected"]
            pub const HWTS1_4: u32 = 0x04;
            #[doc = "ADC TRIG3 selected"]
            pub const HWTS1_8: u32 = 0x08;
            #[doc = "ADC TRIG4 selected"]
            pub const HWTS1_16: u32 = 0x10;
            #[doc = "ADC TRIG5 selected"]
            pub const HWTS1_32: u32 = 0x20;
            #[doc = "ADC TRIG6 selected"]
            pub const HWTS1_64: u32 = 0x40;
            #[doc = "ADC TRIG7 selected"]
            pub const HWTS1_128: u32 = 0x80;
        }
    }
    #[doc = "Segment 1 B2B"]
    pub mod B2B1 {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable B2B. Wait until delay value defined by TRIG1_COUNTER\\[SAMPLE_INTERVAL\\] is reached"]
            pub const B2B1_0: u32 = 0;
            #[doc = "Enable B2B. When Segment 0 finished (ADC COCO) then automatically trigger next ADC conversion, no need to wait until interval delay reached."]
            pub const B2B1_1: u32 = 0x01;
        }
    }
    #[doc = "Segment 1 interrupt enable. (This bit field is meaningful only when IE1_EN is set)"]
    pub mod IE1 {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Generate interrupt on Done0 when Segment 1 finish."]
            pub const IE1_0: u32 = 0;
            #[doc = "Generate interrupt on Done1 when Segment 1 finish."]
            pub const IE1_1: u32 = 0x01;
            #[doc = "Generate interrupt on Done2 when Segment 1 finish."]
            pub const IE1_2: u32 = 0x02;
            #[doc = "Generate interrupt on Done3 when Segment 1 finish."]
            pub const IE1_3: u32 = 0x03;
        }
    }
    #[doc = "IRQ enable of segment 1."]
    pub mod IE1_EN {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt DONE disabled."]
            pub const IE1_EN_0: u32 = 0;
            #[doc = "Interrupt DONE enabled. When segment 1 finish, an interrupt will be generated on the specific port configured by the IE1."]
            pub const IE1_EN_1: u32 = 0x01;
        }
    }
}
#[doc = "ETC_TRIG Chain 2/3 Register"]
pub mod TRIG7_CHAIN_3_2 {
    #[doc = "ADC hardware trigger command selection"]
    pub mod CSEL2 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not a valid selection from the command buffer. Trigger event is ignored."]
            pub const CSEL2_0: u32 = 0;
            #[doc = "ADC CMD1 selected."]
            pub const CSEL2_1: u32 = 0x01;
            #[doc = "ADC CMD2 selected."]
            pub const CSEL2_2: u32 = 0x02;
            #[doc = "ADC CMD3 selected."]
            pub const CSEL2_3: u32 = 0x03;
            #[doc = "ADC CMD4 selected."]
            pub const CSEL2_4: u32 = 0x04;
            #[doc = "ADC CMD5 selected."]
            pub const CSEL2_5: u32 = 0x05;
            #[doc = "ADC CMD6 selected."]
            pub const CSEL2_6: u32 = 0x06;
            #[doc = "ADC CMD7 selected."]
            pub const CSEL2_7: u32 = 0x07;
            #[doc = "ADC CMD8 selected."]
            pub const CSEL2_8: u32 = 0x08;
            #[doc = "ADC CMD9 selected."]
            pub const CSEL2_9: u32 = 0x09;
            #[doc = "ADC CMD10 selected."]
            pub const CSEL2_10: u32 = 0x0a;
            #[doc = "ADC CMD11 selected."]
            pub const CSEL2_11: u32 = 0x0b;
            #[doc = "ADC CMD12 selected."]
            pub const CSEL2_12: u32 = 0x0c;
            #[doc = "ADC CMD13 selected."]
            pub const CSEL2_13: u32 = 0x0d;
            #[doc = "ADC CMD14 selected."]
            pub const CSEL2_14: u32 = 0x0e;
            #[doc = "ADC CMD15 selected."]
            pub const CSEL2_15: u32 = 0x0f;
        }
    }
    #[doc = "Segment 2 HWTS ADC hardware trigger selection"]
    pub mod HWTS2 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "no trigger selected"]
            pub const HWTS2_0: u32 = 0;
            #[doc = "ADC TRIG0 selected"]
            pub const HWTS2_1: u32 = 0x01;
            #[doc = "ADC TRIG1 selected"]
            pub const HWTS2_2: u32 = 0x02;
            #[doc = "ADC TRIG2 selected"]
            pub const HWTS2_4: u32 = 0x04;
            #[doc = "ADC TRIG3 selected"]
            pub const HWTS2_8: u32 = 0x08;
            #[doc = "ADC TRIG4 selected"]
            pub const HWTS2_16: u32 = 0x10;
            #[doc = "ADC TRIG5 selected"]
            pub const HWTS2_32: u32 = 0x20;
            #[doc = "ADC TRIG6 selected"]
            pub const HWTS2_64: u32 = 0x40;
            #[doc = "ADC TRIG7 selected"]
            pub const HWTS2_128: u32 = 0x80;
        }
    }
    #[doc = "Segment 2 B2B"]
    pub mod B2B2 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable B2B. Wait until delay value defined by TRIG2_COUNTER\\[SAMPLE_INTERVAL\\] is reached"]
            pub const B2B2_0: u32 = 0;
            #[doc = "Enable B2B. When Segment 0 finished (ADC COCO) then automatically trigger next ADC conversion, no need to wait until interval delay reached."]
            pub const B2B2_1: u32 = 0x01;
        }
    }
    #[doc = "Segment 2 interrupt enable. (This bit field is meaningful only when IE2_EN is set)"]
    pub mod IE2 {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Generate interrupt on Done0 when segment 2 finish."]
            pub const IE2_0: u32 = 0;
            #[doc = "Generate interrupt on Done1 when segment 2 finish."]
            pub const IE2_1: u32 = 0x01;
            #[doc = "Generate interrupt on Done2 when segment 2 finish."]
            pub const IE2_2: u32 = 0x02;
            #[doc = "Generate interrupt on Done3 when segment 2 finish."]
            pub const IE2_3: u32 = 0x03;
        }
    }
    #[doc = "IRQ enable of segment 2."]
    pub mod IE2_EN {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt DONE disabled."]
            pub const IE2_EN_0: u32 = 0;
            #[doc = "Interrupt DONE enabled. When segment 2 finish, an interrupt will be generated on the specific port configured by the IE2."]
            pub const IE2_EN_1: u32 = 0x01;
        }
    }
    #[doc = "ADC hardware trigger command selection"]
    pub mod CSEL3 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not a valid selection from the command buffer. Trigger event is ignored."]
            pub const CSEL3_0: u32 = 0;
            #[doc = "ADC CMD1 selected."]
            pub const CSEL3_1: u32 = 0x01;
            #[doc = "ADC CMD2 selected."]
            pub const CSEL3_2: u32 = 0x02;
            #[doc = "ADC CMD3 selected."]
            pub const CSEL3_3: u32 = 0x03;
            #[doc = "ADC CMD4 selected."]
            pub const CSEL3_4: u32 = 0x04;
            #[doc = "ADC CMD5 selected."]
            pub const CSEL3_5: u32 = 0x05;
            #[doc = "ADC CMD6 selected."]
            pub const CSEL3_6: u32 = 0x06;
            #[doc = "ADC CMD7 selected."]
            pub const CSEL3_7: u32 = 0x07;
            #[doc = "ADC CMD8 selected."]
            pub const CSEL3_8: u32 = 0x08;
            #[doc = "ADC CMD9 selected."]
            pub const CSEL3_9: u32 = 0x09;
            #[doc = "ADC CMD10 selected."]
            pub const CSEL3_10: u32 = 0x0a;
            #[doc = "ADC CMD11 selected."]
            pub const CSEL3_11: u32 = 0x0b;
            #[doc = "ADC CMD12 selected."]
            pub const CSEL3_12: u32 = 0x0c;
            #[doc = "ADC CMD13 selected."]
            pub const CSEL3_13: u32 = 0x0d;
            #[doc = "ADC CMD14 selected."]
            pub const CSEL3_14: u32 = 0x0e;
            #[doc = "ADC CMD15 selected."]
            pub const CSEL3_15: u32 = 0x0f;
        }
    }
    #[doc = "Segment 3 HWTS ADC hardware trigger selection"]
    pub mod HWTS3 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "no trigger selected"]
            pub const HWTS3_0: u32 = 0;
            #[doc = "ADC TRIG0 selected"]
            pub const HWTS3_1: u32 = 0x01;
            #[doc = "ADC TRIG1 selected"]
            pub const HWTS3_2: u32 = 0x02;
            #[doc = "ADC TRIG2 selected"]
            pub const HWTS3_4: u32 = 0x04;
            #[doc = "ADC TRIG3 selected"]
            pub const HWTS3_8: u32 = 0x08;
            #[doc = "ADC TRIG4 selected"]
            pub const HWTS3_16: u32 = 0x10;
            #[doc = "ADC TRIG5 selected"]
            pub const HWTS3_32: u32 = 0x20;
            #[doc = "ADC TRIG6 selected"]
            pub const HWTS3_64: u32 = 0x40;
            #[doc = "ADC TRIG7 selected"]
            pub const HWTS3_128: u32 = 0x80;
        }
    }
    #[doc = "Segment 3 B2B"]
    pub mod B2B3 {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable B2B. Wait until delay value defined by TRIG3_COUNTER\\[SAMPLE_INTERVAL\\] is reached"]
            pub const B2B3_0: u32 = 0;
            #[doc = "Enable B2B. When Segment 0 finished (ADC COCO) then automatically trigger next ADC conversion, no need to wait until interval delay reached."]
            pub const B2B3_1: u32 = 0x01;
        }
    }
    #[doc = "Segment 3 interrupt enable. (This bit field is meaningful only when IE3_EN is set)"]
    pub mod IE3 {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Generate interrupt on Done0 when segment 3 finish."]
            pub const IE3_0: u32 = 0;
            #[doc = "Generate interrupt on Done1 when segment 3 finish."]
            pub const IE3_1: u32 = 0x01;
            #[doc = "Generate interrupt on Done2 when segment 3 finish."]
            pub const IE3_2: u32 = 0x02;
            #[doc = "Generate interrupt on Done3 when segment 3 finish."]
            pub const IE3_3: u32 = 0x03;
        }
    }
    #[doc = "IRQ enable of segment 3."]
    pub mod IE3_EN {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt DONE disabled."]
            pub const IE3_EN_0: u32 = 0;
            #[doc = "Interrupt DONE enabled. When segment 3 finish, an interrupt will be generated on the specific port configured by the IE3."]
            pub const IE3_EN_1: u32 = 0x01;
        }
    }
}
#[doc = "ETC_TRIG Chain 4/5 Register"]
pub mod TRIG7_CHAIN_5_4 {
    #[doc = "ADC hardware trigger command selection"]
    pub mod CSEL4 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not a valid selection from the command buffer. Trigger event is ignored."]
            pub const CSEL4_0: u32 = 0;
            #[doc = "ADC CMD1 selected."]
            pub const CSEL4_1: u32 = 0x01;
            #[doc = "ADC CMD2 selected."]
            pub const CSEL4_2: u32 = 0x02;
            #[doc = "ADC CMD3 selected."]
            pub const CSEL4_3: u32 = 0x03;
            #[doc = "ADC CMD4 selected."]
            pub const CSEL4_4: u32 = 0x04;
            #[doc = "ADC CMD5 selected."]
            pub const CSEL4_5: u32 = 0x05;
            #[doc = "ADC CMD6 selected."]
            pub const CSEL4_6: u32 = 0x06;
            #[doc = "ADC CMD7 selected."]
            pub const CSEL4_7: u32 = 0x07;
            #[doc = "ADC CMD8 selected."]
            pub const CSEL4_8: u32 = 0x08;
            #[doc = "ADC CMD9 selected."]
            pub const CSEL4_9: u32 = 0x09;
            #[doc = "ADC CMD10 selected."]
            pub const CSEL4_10: u32 = 0x0a;
            #[doc = "ADC CMD11 selected."]
            pub const CSEL4_11: u32 = 0x0b;
            #[doc = "ADC CMD12 selected."]
            pub const CSEL4_12: u32 = 0x0c;
            #[doc = "ADC CMD13 selected."]
            pub const CSEL4_13: u32 = 0x0d;
            #[doc = "ADC CMD14 selected."]
            pub const CSEL4_14: u32 = 0x0e;
            #[doc = "ADC CMD15 selected."]
            pub const CSEL4_15: u32 = 0x0f;
        }
    }
    #[doc = "Segment 4 HWTS ADC hardware trigger selection"]
    pub mod HWTS4 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "no trigger selected"]
            pub const HWTS4_0: u32 = 0;
            #[doc = "ADC TRIG0 selected"]
            pub const HWTS4_1: u32 = 0x01;
            #[doc = "ADC TRIG1 selected"]
            pub const HWTS4_2: u32 = 0x02;
            #[doc = "ADC TRIG2 selected"]
            pub const HWTS4_4: u32 = 0x04;
            #[doc = "ADC TRIG3 selected"]
            pub const HWTS4_8: u32 = 0x08;
            #[doc = "ADC TRIG4 selected"]
            pub const HWTS4_16: u32 = 0x10;
            #[doc = "ADC TRIG5 selected"]
            pub const HWTS4_32: u32 = 0x20;
            #[doc = "ADC TRIG6 selected"]
            pub const HWTS4_64: u32 = 0x40;
            #[doc = "ADC TRIG7 selected"]
            pub const HWTS4_128: u32 = 0x80;
        }
    }
    #[doc = "Segment 4 B2B"]
    pub mod B2B4 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable B2B. Wait until delay value defined by TRIG4_COUNTER\\[SAMPLE_INTERVAL\\] is reached"]
            pub const B2B4_0: u32 = 0;
            #[doc = "Enable B2B. When Segment 0 finished (ADC COCO) then automatically trigger next ADC conversion, no need to wait until interval delay reached."]
            pub const B2B4_1: u32 = 0x01;
        }
    }
    #[doc = "Segment 4 interrupt enable. (This bit field is meaningful only when IE4_EN is set)"]
    pub mod IE4 {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Generate interrupt on Done0 when segment 4 finish."]
            pub const IE4_0: u32 = 0;
            #[doc = "Generate interrupt on Done1 when segment 4 finish."]
            pub const IE4_1: u32 = 0x01;
            #[doc = "Generate interrupt on Done2 when segment 4 finish."]
            pub const IE4_2: u32 = 0x02;
            #[doc = "Generate interrupt on Done3 when segment 4 finish."]
            pub const IE4_3: u32 = 0x03;
        }
    }
    #[doc = "IRQ enable of segment 4."]
    pub mod IE4_EN {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt DONE disabled."]
            pub const IE4_EN_0: u32 = 0;
            #[doc = "Interrupt DONE enabled. When segment 4 finish, an interrupt will be generated on the specific port configured by the IE4."]
            pub const IE4_EN_1: u32 = 0x01;
        }
    }
    #[doc = "ADC hardware trigger command selection"]
    pub mod CSEL5 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not a valid selection from the command buffer. Trigger event is ignored."]
            pub const CSEL5_0: u32 = 0;
            #[doc = "ADC CMD1 selected."]
            pub const CSEL5_1: u32 = 0x01;
            #[doc = "ADC CMD2 selected."]
            pub const CSEL5_2: u32 = 0x02;
            #[doc = "ADC CMD3 selected."]
            pub const CSEL5_3: u32 = 0x03;
            #[doc = "ADC CMD4 selected."]
            pub const CSEL5_4: u32 = 0x04;
            #[doc = "ADC CMD5 selected."]
            pub const CSEL5_5: u32 = 0x05;
            #[doc = "ADC CMD6 selected."]
            pub const CSEL5_6: u32 = 0x06;
            #[doc = "ADC CMD7 selected."]
            pub const CSEL5_7: u32 = 0x07;
            #[doc = "ADC CMD8 selected."]
            pub const CSEL5_8: u32 = 0x08;
            #[doc = "ADC CMD9 selected."]
            pub const CSEL5_9: u32 = 0x09;
            #[doc = "ADC CMD10 selected."]
            pub const CSEL5_10: u32 = 0x0a;
            #[doc = "ADC CMD11 selected."]
            pub const CSEL5_11: u32 = 0x0b;
            #[doc = "ADC CMD12 selected."]
            pub const CSEL5_12: u32 = 0x0c;
            #[doc = "ADC CMD13 selected."]
            pub const CSEL5_13: u32 = 0x0d;
            #[doc = "ADC CMD14 selected."]
            pub const CSEL5_14: u32 = 0x0e;
            #[doc = "ADC CMD15 selected."]
            pub const CSEL5_15: u32 = 0x0f;
        }
    }
    #[doc = "Segment 5 HWTS ADC hardware trigger selection"]
    pub mod HWTS5 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "no trigger selected"]
            pub const HWTS5_0: u32 = 0;
            #[doc = "ADC TRIG0 selected"]
            pub const HWTS5_1: u32 = 0x01;
            #[doc = "ADC TRIG1 selected"]
            pub const HWTS5_2: u32 = 0x02;
            #[doc = "ADC TRIG2 selected"]
            pub const HWTS5_4: u32 = 0x04;
            #[doc = "ADC TRIG3 selected"]
            pub const HWTS5_8: u32 = 0x08;
            #[doc = "ADC TRIG4 selected"]
            pub const HWTS5_16: u32 = 0x10;
            #[doc = "ADC TRIG5 selected"]
            pub const HWTS5_32: u32 = 0x20;
            #[doc = "ADC TRIG6 selected"]
            pub const HWTS5_64: u32 = 0x40;
            #[doc = "ADC TRIG7 selected"]
            pub const HWTS5_128: u32 = 0x80;
        }
    }
    #[doc = "Segment 5 B2B"]
    pub mod B2B5 {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable B2B. Wait until delay value defined by TRIG5_COUNTER\\[SAMPLE_INTERVAL\\] is reached"]
            pub const B2B5_0: u32 = 0;
            #[doc = "Enable B2B. When Segment 0 finished (ADC COCO) then automatically trigger next ADC conversion, no need to wait until interval delay reached."]
            pub const B2B5_1: u32 = 0x01;
        }
    }
    #[doc = "Segment 5 interrupt enable. (This bit field is meaningful only when IE5_EN is set)"]
    pub mod IE5 {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Generate interrupt on Done0 when segment 5 finish."]
            pub const IE5_0: u32 = 0;
            #[doc = "Generate interrupt on Done1 when segment 5 finish."]
            pub const IE5_1: u32 = 0x01;
            #[doc = "Generate interrupt on Done2 when segment 5 finish."]
            pub const IE5_2: u32 = 0x02;
            #[doc = "Generate interrupt on Done3 when segment 5 finish."]
            pub const IE5_3: u32 = 0x03;
        }
    }
    #[doc = "IRQ enable of segment 5."]
    pub mod IE5_EN {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt DONE disabled."]
            pub const IE5_EN_0: u32 = 0;
            #[doc = "Interrupt DONE enabled. When segment 5 finish, an interrupt will be generated on the specific port configured by the IE5."]
            pub const IE5_EN_1: u32 = 0x01;
        }
    }
}
#[doc = "ETC_TRIG Chain 6/7 Register"]
pub mod TRIG7_CHAIN_7_6 {
    #[doc = "ADC hardware trigger command selection"]
    pub mod CSEL6 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not a valid selection from the command buffer. Trigger event is ignored."]
            pub const CSEL6_0: u32 = 0;
            #[doc = "ADC CMD1 selected."]
            pub const CSEL6_1: u32 = 0x01;
            #[doc = "ADC CMD2 selected."]
            pub const CSEL6_2: u32 = 0x02;
            #[doc = "ADC CMD3 selected."]
            pub const CSEL6_3: u32 = 0x03;
            #[doc = "ADC CMD4 selected."]
            pub const CSEL6_4: u32 = 0x04;
            #[doc = "ADC CMD5 selected."]
            pub const CSEL6_5: u32 = 0x05;
            #[doc = "ADC CMD6 selected."]
            pub const CSEL6_6: u32 = 0x06;
            #[doc = "ADC CMD7 selected."]
            pub const CSEL6_7: u32 = 0x07;
            #[doc = "ADC CMD8 selected."]
            pub const CSEL6_8: u32 = 0x08;
            #[doc = "ADC CMD9 selected."]
            pub const CSEL6_9: u32 = 0x09;
            #[doc = "ADC CMD10 selected."]
            pub const CSEL6_10: u32 = 0x0a;
            #[doc = "ADC CMD11 selected."]
            pub const CSEL6_11: u32 = 0x0b;
            #[doc = "ADC CMD12 selected."]
            pub const CSEL6_12: u32 = 0x0c;
            #[doc = "ADC CMD13 selected."]
            pub const CSEL6_13: u32 = 0x0d;
            #[doc = "ADC CMD14 selected."]
            pub const CSEL6_14: u32 = 0x0e;
            #[doc = "ADC CMD15 selected."]
            pub const CSEL6_15: u32 = 0x0f;
        }
    }
    #[doc = "Segment 6 HWTS ADC hardware trigger selection"]
    pub mod HWTS6 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "no trigger selected"]
            pub const HWTS6_0: u32 = 0;
            #[doc = "ADC TRIG0 selected"]
            pub const HWTS6_1: u32 = 0x01;
            #[doc = "ADC TRIG1 selected"]
            pub const HWTS6_2: u32 = 0x02;
            #[doc = "ADC TRIG2 selected"]
            pub const HWTS6_4: u32 = 0x04;
            #[doc = "ADC TRIG3 selected"]
            pub const HWTS6_8: u32 = 0x08;
            #[doc = "ADC TRIG4 selected"]
            pub const HWTS6_16: u32 = 0x10;
            #[doc = "ADC TRIG5 selected"]
            pub const HWTS6_32: u32 = 0x20;
            #[doc = "ADC TRIG6 selected"]
            pub const HWTS6_64: u32 = 0x40;
            #[doc = "ADC TRIG7 selected"]
            pub const HWTS6_128: u32 = 0x80;
        }
    }
    #[doc = "Segment 6 B2B"]
    pub mod B2B6 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable B2B. Wait until delay value defined by TRIG6_COUNTER\\[SAMPLE_INTERVAL\\] is reached"]
            pub const B2B6_0: u32 = 0;
            #[doc = "Enable B2B. When Segment 0 finished (ADC COCO) then automatically trigger next ADC conversion, no need to wait until interval delay reached."]
            pub const B2B6_1: u32 = 0x01;
        }
    }
    #[doc = "Segment 6 interrupt enable. (This bit field is meaningful only when IE6_EN is set)"]
    pub mod IE6 {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Generate interrupt on Done0 when segment 6 finish."]
            pub const IE6_0: u32 = 0;
            #[doc = "Generate interrupt on Done1 when segment 6 finish."]
            pub const IE6_1: u32 = 0x01;
            #[doc = "Generate interrupt on Done2 when segment 6 finish."]
            pub const IE6_2: u32 = 0x02;
            #[doc = "Generate interrupt on Done3 when segment 6 finish."]
            pub const IE6_3: u32 = 0x03;
        }
    }
    #[doc = "IRQ enable of segment 6."]
    pub mod IE6_EN {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt DONE disabled."]
            pub const IE6_EN_0: u32 = 0;
            #[doc = "Interrupt DONE enabled. When segment 6 finish, an interrupt will be generated on the specific port configured by the IE6."]
            pub const IE6_EN_1: u32 = 0x01;
        }
    }
    #[doc = "ADC hardware trigger command selection"]
    pub mod CSEL7 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not a valid selection from the command buffer. Trigger event is ignored."]
            pub const CSEL7_0: u32 = 0;
            #[doc = "ADC CMD1 selected."]
            pub const CSEL7_1: u32 = 0x01;
            #[doc = "ADC CMD2 selected."]
            pub const CSEL7_2: u32 = 0x02;
            #[doc = "ADC CMD3 selected."]
            pub const CSEL7_3: u32 = 0x03;
            #[doc = "ADC CMD4 selected."]
            pub const CSEL7_4: u32 = 0x04;
            #[doc = "ADC CMD5 selected."]
            pub const CSEL7_5: u32 = 0x05;
            #[doc = "ADC CMD6 selected."]
            pub const CSEL7_6: u32 = 0x06;
            #[doc = "ADC CMD7 selected."]
            pub const CSEL7_7: u32 = 0x07;
            #[doc = "ADC CMD8 selected."]
            pub const CSEL7_8: u32 = 0x08;
            #[doc = "ADC CMD9 selected."]
            pub const CSEL7_9: u32 = 0x09;
            #[doc = "ADC CMD10 selected."]
            pub const CSEL7_10: u32 = 0x0a;
            #[doc = "ADC CMD11 selected."]
            pub const CSEL7_11: u32 = 0x0b;
            #[doc = "ADC CMD12 selected."]
            pub const CSEL7_12: u32 = 0x0c;
            #[doc = "ADC CMD13 selected."]
            pub const CSEL7_13: u32 = 0x0d;
            #[doc = "ADC CMD14 selected."]
            pub const CSEL7_14: u32 = 0x0e;
            #[doc = "ADC CMD15 selected."]
            pub const CSEL7_15: u32 = 0x0f;
        }
    }
    #[doc = "Segment 7 HWTS ADC hardware trigger selection"]
    pub mod HWTS7 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "no trigger selected"]
            pub const HWTS7_0: u32 = 0;
            #[doc = "ADC TRIG0 selected"]
            pub const HWTS7_1: u32 = 0x01;
            #[doc = "ADC TRIG1 selected"]
            pub const HWTS7_2: u32 = 0x02;
            #[doc = "ADC TRIG2 selected"]
            pub const HWTS7_4: u32 = 0x04;
            #[doc = "ADC TRIG3 selected"]
            pub const HWTS7_8: u32 = 0x08;
            #[doc = "ADC TRIG4 selected"]
            pub const HWTS7_16: u32 = 0x10;
            #[doc = "ADC TRIG5 selected"]
            pub const HWTS7_32: u32 = 0x20;
            #[doc = "ADC TRIG6 selected"]
            pub const HWTS7_64: u32 = 0x40;
            #[doc = "ADC TRIG7 selected"]
            pub const HWTS7_128: u32 = 0x80;
        }
    }
    #[doc = "Segment 7 B2B"]
    pub mod B2B7 {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable B2B. Wait until delay value defined by TRIG7_COUNTER\\[SAMPLE_INTERVAL\\] is reached"]
            pub const B2B7_0: u32 = 0;
            #[doc = "Enable B2B. When Segment 0 finished (ADC COCO) then automatically trigger next ADC conversion, no need to wait until interval delay reached."]
            pub const B2B7_1: u32 = 0x01;
        }
    }
    #[doc = "Segment 7 interrupt enable. (This bit field is meaningful only when IE7_EN is set)"]
    pub mod IE7 {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Generate interrupt on Done0 when segment 7 finish."]
            pub const IE7_0: u32 = 0;
            #[doc = "Generate interrupt on Done1 when segment 7 finish."]
            pub const IE7_1: u32 = 0x01;
            #[doc = "Generate interrupt on Done2 when segment 7 finish."]
            pub const IE7_2: u32 = 0x02;
            #[doc = "Generate interrupt on Done3 when segment 7 finish."]
            pub const IE7_3: u32 = 0x03;
        }
    }
    #[doc = "IRQ enable of segment 7."]
    pub mod IE7_EN {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt DONE disabled."]
            pub const IE7_EN_0: u32 = 0;
            #[doc = "Interrupt DONE enabled. When segment 7 finish, an interrupt will be generated on the specific port configured by the IE7."]
            pub const IE7_EN_1: u32 = 0x01;
        }
    }
}
#[doc = "ETC_TRIG Result Data 1/0 Register"]
pub mod TRIG7_RESULT_1_0 {
    #[doc = "Result DATA0"]
    pub mod DATA0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Result DATA1"]
    pub mod DATA1 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ETC_TRIG Result Data 3/2 Register"]
pub mod TRIG7_RESULT_3_2 {
    #[doc = "Result DATA2"]
    pub mod DATA2 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Result DATA3"]
    pub mod DATA3 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ETC_TRIG Result Data 5/4 Register"]
pub mod TRIG7_RESULT_5_4 {
    #[doc = "Result DATA4"]
    pub mod DATA4 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Result DATA5"]
    pub mod DATA5 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ETC_TRIG Result Data 7/6 Register"]
pub mod TRIG7_RESULT_7_6 {
    #[doc = "Result DATA6"]
    pub mod DATA6 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Result DATA7"]
    pub mod DATA7 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
