#[doc = "ADC_ETC"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "ADC_ETC Global Control Register"]
    pub CTRL: crate::RWRegister<u32>,
    #[doc = "ETC DONE0 and DONE1 IRQ State Register"]
    pub DONE0_1_IRQ: crate::RWRegister<u32>,
    #[doc = "ETC DONE_2 and DONE_ERR IRQ State Register"]
    pub DONE2_ERR_IRQ: crate::RWRegister<u32>,
    #[doc = "ETC DMA control Register"]
    pub DMA_CTRL: crate::RWRegister<u32>,
    #[doc = "ETC_TRIG0 Control Register"]
    pub TRIG0_CTRL: crate::RWRegister<u32>,
    #[doc = "ETC_TRIG0 Counter Register"]
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
    #[doc = "ETC_TRIG1 Control Register"]
    pub TRIG1_CTRL: crate::RWRegister<u32>,
    #[doc = "ETC_TRIG1 Counter Register"]
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
    #[doc = "ETC_TRIG2 Control Register"]
    pub TRIG2_CTRL: crate::RWRegister<u32>,
    #[doc = "ETC_TRIG2 Counter Register"]
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
    #[doc = "ETC_TRIG3 Control Register"]
    pub TRIG3_CTRL: crate::RWRegister<u32>,
    #[doc = "ETC_TRIG3 Counter Register"]
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
}
#[doc = "ADC_ETC Global Control Register"]
pub mod CTRL {
    #[doc = "TRIG enable register"]
    pub mod TRIG_ENABLE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TSC0 TRIG enable register. 1'b1: enable external TSC0 trigger. 1'b0: disable external TSC0 trigger."]
    pub mod EXT0_TRIG_ENABLE {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "External TSC0 trigger priority, 7 is Highest, 0 is lowest ."]
    pub mod EXT0_TRIG_PRIORITY {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TSC1 TRIG enable register. 1'b1: enable external TSC1 trigger. 1'b0: disable external TSC1 trigger."]
    pub mod EXT1_TRIG_ENABLE {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "External TSC1 trigger priority, 7 is Highest, 0 is lowest ."]
    pub mod EXT1_TRIG_PRIORITY {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Pre-divider for trig delay and interval ."]
    pub mod PRE_DIVIDER {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1'b0: Trig DMA_REQ with latched signal, REQ will be cleared when ACK and source request cleared"]
    pub mod DMA_MODE_SEL {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1'b1: TSC is bypassed to ADC2. 1'b0: TSC not bypassed. To use ADC2, this bit should be cleared."]
    pub mod TSC_BYPASS {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Software reset, high active. When write 1 ,all logical will be reset."]
    pub mod SOFTRST {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ETC DONE0 and DONE1 IRQ State Register"]
pub mod DONE0_1_IRQ {
    #[doc = "TRIG0 done0 interrupt detection"]
    pub mod TRIG0_DONE0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TRIG1 done0 interrupt detection"]
    pub mod TRIG1_DONE0 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TRIG2 done0 interrupt detection"]
    pub mod TRIG2_DONE0 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TRIG3 done0 interrupt detection"]
    pub mod TRIG3_DONE0 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TRIG4 done0 interrupt detection"]
    pub mod TRIG4_DONE0 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TRIG5 done0 interrupt detection"]
    pub mod TRIG5_DONE0 {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TRIG6 done0 interrupt detection"]
    pub mod TRIG6_DONE0 {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TRIG7 done0 interrupt detection"]
    pub mod TRIG7_DONE0 {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TRIG0 done1 interrupt detection"]
    pub mod TRIG0_DONE1 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TRIG1 done1 interrupt detection"]
    pub mod TRIG1_DONE1 {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TRIG2 done1 interrupt detection"]
    pub mod TRIG2_DONE1 {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TRIG3 done1 interrupt detection"]
    pub mod TRIG3_DONE1 {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TRIG4 done1 interrupt detection"]
    pub mod TRIG4_DONE1 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TRIG5 done1 interrupt detection"]
    pub mod TRIG5_DONE1 {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TRIG6 done1 interrupt detection"]
    pub mod TRIG6_DONE1 {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TRIG7 done1 interrupt detection"]
    pub mod TRIG7_DONE1 {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ETC DONE_2 and DONE_ERR IRQ State Register"]
pub mod DONE2_ERR_IRQ {
    #[doc = "TRIG0 done2 interrupt detection"]
    pub mod TRIG0_DONE2 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TRIG1 done2 interrupt detection"]
    pub mod TRIG1_DONE2 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TRIG2 done2 interrupt detection"]
    pub mod TRIG2_DONE2 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TRIG3 done2 interrupt detection"]
    pub mod TRIG3_DONE2 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TRIG4 done2 interrupt detection"]
    pub mod TRIG4_DONE2 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TRIG5 done2 interrupt detection"]
    pub mod TRIG5_DONE2 {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TRIG6 done2 interrupt detection"]
    pub mod TRIG6_DONE2 {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TRIG7 done2 interrupt detection"]
    pub mod TRIG7_DONE2 {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TRIG0 error interrupt detection"]
    pub mod TRIG0_ERR {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TRIG1 error interrupt detection"]
    pub mod TRIG1_ERR {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TRIG2 error interrupt detection"]
    pub mod TRIG2_ERR {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TRIG3 error interrupt detection"]
    pub mod TRIG3_ERR {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TRIG4 error interrupt detection"]
    pub mod TRIG4_ERR {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TRIG5 error interrupt detection"]
    pub mod TRIG5_ERR {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TRIG6 error interrupt detection"]
    pub mod TRIG6_ERR {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TRIG7 error interrupt detection"]
    pub mod TRIG7_ERR {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ETC DMA control Register"]
pub mod DMA_CTRL {
    #[doc = "When TRIG0 done enable DMA request"]
    pub mod TRIG0_ENABLE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "When TRIG1 done enable DMA request"]
    pub mod TRIG1_ENABLE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "When TRIG2 done enable DMA request"]
    pub mod TRIG2_ENABLE {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "When TRIG3 done enable DMA request"]
    pub mod TRIG3_ENABLE {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "When TRIG4 done enable DMA request"]
    pub mod TRIG4_ENABLE {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "When TRIG5 done enable DMA request"]
    pub mod TRIG5_ENABLE {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "When TRIG6 done enable DMA request"]
    pub mod TRIG6_ENABLE {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "When TRIG7 done enable DMA request"]
    pub mod TRIG7_ENABLE {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "When TRIG0 done DMA request detection"]
    pub mod TRIG0_REQ {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "When TRIG1 done DMA request detection"]
    pub mod TRIG1_REQ {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "When TRIG2 done DMA request detection"]
    pub mod TRIG2_REQ {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "When TRIG3 done DMA request detection"]
    pub mod TRIG3_REQ {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "When TRIG4 done DMA request detection"]
    pub mod TRIG4_REQ {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "When TRIG5 done DMA request detection"]
    pub mod TRIG5_REQ {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "When TRIG6 done DMA request detection"]
    pub mod TRIG6_REQ {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "When TRIG7 done DMA request detection"]
    pub mod TRIG7_REQ {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ETC_TRIG0 Control Register"]
pub mod TRIG0_CTRL {
    #[doc = "Software write 1 as the TRIGGER. This register is self-clearing."]
    pub mod SW_TRIG {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TRIG mode register. 1'b0: hardware trigger. 1'b1: software trigger."]
    pub mod TRIG_MODE {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TRIG chain length to the ADC. 0: Trig length is 1; ... 7: Trig length is 8;"]
    pub mod TRIG_CHAIN {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "External trigger priority, 7 is highest, 0 is lowest ."]
    pub mod TRIG_PRIORITY {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TRIG mode control . 1'b0: Disable sync mode; 1'b1: Enable sync mode"]
    pub mod SYNC_MODE {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ETC_TRIG0 Counter Register"]
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
    #[doc = "CHAIN0 CSEL ADC channel selection"]
    pub mod CSEL0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CHAIN0 HWTS ADC hardware trigger selection. For more information, see the ADC chapter."]
    pub mod HWTS0 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CHAIN0 B2B 1'b0: Disable B2B, wait until interval is reached 1'b1: Enable B2B, back to back ADC trigger"]
    pub mod B2B0 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CHAIN0 IE 2'b00: No interrupt when finished 2'b01: Finished Interrupt on Done0 2'b10: Finished Interrupt on Done1 2'b11: Finished Interrupt on Done2"]
    pub mod IE0 {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CHAIN1 CSEL ADC channel selection"]
    pub mod CSEL1 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CHAIN1 HWTS ADC hardware trigger selection. For more information, see the ADC chapter."]
    pub mod HWTS1 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CHAIN1 B2B 1'b0: Disable B2B, wait until interval is reached 1'b1: Enable B2B, back to back ADC trigger"]
    pub mod B2B1 {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CHAIN1 IE 2'b00: No interrupt when finished 2'b01: Finished Interrupt on Done0 2'b10: Finished Interrupt on Done1 2'b11: Finished Interrupt on Done2"]
    pub mod IE1 {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ETC_TRIG Chain 2/3 Register"]
pub mod TRIG0_CHAIN_3_2 {
    #[doc = "CHAIN2 CSEL"]
    pub mod CSEL2 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CHAIN2 HWTS"]
    pub mod HWTS2 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CHAIN2 B2B"]
    pub mod B2B2 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CHAIN2 IE"]
    pub mod IE2 {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CHAIN3 CSEL"]
    pub mod CSEL3 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CHAIN3 HWTS"]
    pub mod HWTS3 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CHAIN3 B2B"]
    pub mod B2B3 {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CHAIN3 IE"]
    pub mod IE3 {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ETC_TRIG Chain 4/5 Register"]
pub mod TRIG0_CHAIN_5_4 {
    #[doc = "CHAIN4 CSEL"]
    pub mod CSEL4 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CHAIN4 HWTS"]
    pub mod HWTS4 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CHAIN4 B2B"]
    pub mod B2B4 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CHAIN4 IE"]
    pub mod IE4 {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CHAIN5 CSEL"]
    pub mod CSEL5 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CHAIN5 HWTS"]
    pub mod HWTS5 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CHAIN5 B2B"]
    pub mod B2B5 {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CHAIN5 IE"]
    pub mod IE5 {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ETC_TRIG Chain 6/7 Register"]
pub mod TRIG0_CHAIN_7_6 {
    #[doc = "CHAIN6 CSEL"]
    pub mod CSEL6 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CHAIN6 HWTS"]
    pub mod HWTS6 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CHAIN6 B2B"]
    pub mod B2B6 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CHAIN6 IE"]
    pub mod IE6 {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CHAIN7 CSEL"]
    pub mod CSEL7 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CHAIN7 HWTS"]
    pub mod HWTS7 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CHAIN7 B2B"]
    pub mod B2B7 {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CHAIN7 IE"]
    pub mod IE7 {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
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
#[doc = "ETC_TRIG1 Control Register"]
pub mod TRIG1_CTRL {
    #[doc = "Software write 1 as the TRIGGER. This register is self-clearing."]
    pub mod SW_TRIG {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TRIG mode register. 1'b0: hardware trigger. 1'b1: software trigger."]
    pub mod TRIG_MODE {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TRIG chain length to the ADC. 0: Trig length is 1; ... 7: Trig length is 8;"]
    pub mod TRIG_CHAIN {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "External trigger priority, 7 is highest, 0 is lowest ."]
    pub mod TRIG_PRIORITY {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TRIG mode control . 1'b0: Disable sync mode; 1'b1: Enable sync mode"]
    pub mod SYNC_MODE {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ETC_TRIG1 Counter Register"]
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
    #[doc = "CHAIN0 CSEL ADC channel selection"]
    pub mod CSEL0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CHAIN0 HWTS ADC hardware trigger selection"]
    pub mod HWTS0 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CHAIN0 B2B 1'b0: Disable B2B, wait until interval is reached 1'b1: Enable B2B, back to back ADC trigger"]
    pub mod B2B0 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CHAIN0 IE 2'b00: No interrupt when finished 2'b01: Finished Interrupt on Done0 2'b10: Finished Interrupt on Done1 2'b11: Finished Interrupt on Done2"]
    pub mod IE0 {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CHAIN1 CSEL ADC channel selection"]
    pub mod CSEL1 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CHAIN1 HWTS ADC hardware trigger selection"]
    pub mod HWTS1 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CHAIN1 B2B 1'b0: Disable B2B, wait until interval is reached 1'b1: Enable B2B, back to back ADC trigger"]
    pub mod B2B1 {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CHAIN1 IE 2'b00: No interrupt when finished 2'b01: Finished Interrupt on Done0 2'b10: Finished Interrupt on Done1 2'b11: Finished Interrupt on Done2"]
    pub mod IE1 {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ETC_TRIG Chain 2/3 Register"]
pub mod TRIG1_CHAIN_3_2 {
    #[doc = "CHAIN2 CSEL"]
    pub mod CSEL2 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CHAIN2 HWTS"]
    pub mod HWTS2 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CHAIN2 B2B"]
    pub mod B2B2 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CHAIN2 IE"]
    pub mod IE2 {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CHAIN3 CSEL"]
    pub mod CSEL3 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CHAIN3 HWTS"]
    pub mod HWTS3 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CHAIN3 B2B"]
    pub mod B2B3 {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CHAIN3 IE"]
    pub mod IE3 {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ETC_TRIG Chain 4/5 Register"]
pub mod TRIG1_CHAIN_5_4 {
    #[doc = "CHAIN4 CSEL"]
    pub mod CSEL4 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CHAIN4 HWTS"]
    pub mod HWTS4 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CHAIN4 B2B"]
    pub mod B2B4 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CHAIN4 IE"]
    pub mod IE4 {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CHAIN5 CSEL"]
    pub mod CSEL5 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CHAIN5 HWTS"]
    pub mod HWTS5 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CHAIN5 B2B"]
    pub mod B2B5 {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CHAIN5 IE"]
    pub mod IE5 {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ETC_TRIG Chain 6/7 Register"]
pub mod TRIG1_CHAIN_7_6 {
    #[doc = "CHAIN6 CSEL"]
    pub mod CSEL6 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CHAIN6 HWTS"]
    pub mod HWTS6 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CHAIN6 B2B"]
    pub mod B2B6 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CHAIN6 IE"]
    pub mod IE6 {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CHAIN7 CSEL"]
    pub mod CSEL7 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CHAIN7 HWTS"]
    pub mod HWTS7 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CHAIN7 B2B"]
    pub mod B2B7 {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CHAIN7 IE"]
    pub mod IE7 {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
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
#[doc = "ETC_TRIG2 Control Register"]
pub mod TRIG2_CTRL {
    #[doc = "Software write 1 as the TRIGGER. This register is self-clearing."]
    pub mod SW_TRIG {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TRIG mode register. 1'b0: hardware trigger. 1'b1: software trigger."]
    pub mod TRIG_MODE {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TRIG chain length to the ADC. 0: Trig length is 1; ... 7: Trig length is 8;"]
    pub mod TRIG_CHAIN {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "External trigger priority, 7 is highest, 0 is lowest ."]
    pub mod TRIG_PRIORITY {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TRIG mode control . 1'b0: Disable sync mode; 1'b1: Enable sync mode"]
    pub mod SYNC_MODE {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ETC_TRIG2 Counter Register"]
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
    #[doc = "CHAIN0 CSEL ADC channel selection"]
    pub mod CSEL0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CHAIN0 HWTS ADC hardware trigger selection"]
    pub mod HWTS0 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CHAIN0 B2B 1'b0: Disable B2B, wait until interval is reached 1'b1: Enable B2B, back to back ADC trigger"]
    pub mod B2B0 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CHAIN0 IE 2'b00: No interrupt when finished 2'b01: Finished Interrupt on Done0 2'b10: Finished Interrupt on Done1 2'b11: Finished Interrupt on Done2"]
    pub mod IE0 {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CHAIN1 CSEL ADC channel selection"]
    pub mod CSEL1 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CHAIN1 HWTS ADC hardware trigger selection"]
    pub mod HWTS1 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CHAIN1 B2B 1'b0: Disable B2B, wait until interval is reached 1'b1: Enable B2B, back to back ADC trigger"]
    pub mod B2B1 {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CHAIN1 IE 2'b00: No interrupt when finished 2'b01: Finished Interrupt on Done0 2'b10: Finished Interrupt on Done1 2'b11: Finished Interrupt on Done2"]
    pub mod IE1 {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ETC_TRIG Chain 2/3 Register"]
pub mod TRIG2_CHAIN_3_2 {
    #[doc = "CHAIN2 CSEL"]
    pub mod CSEL2 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CHAIN2 HWTS"]
    pub mod HWTS2 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CHAIN2 B2B"]
    pub mod B2B2 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CHAIN2 IE"]
    pub mod IE2 {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CHAIN3 CSEL"]
    pub mod CSEL3 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CHAIN3 HWTS"]
    pub mod HWTS3 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CHAIN3 B2B"]
    pub mod B2B3 {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CHAIN3 IE"]
    pub mod IE3 {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ETC_TRIG Chain 4/5 Register"]
pub mod TRIG2_CHAIN_5_4 {
    #[doc = "CHAIN4 CSEL"]
    pub mod CSEL4 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CHAIN4 HWTS"]
    pub mod HWTS4 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CHAIN4 B2B"]
    pub mod B2B4 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CHAIN4 IE"]
    pub mod IE4 {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CHAIN5 CSEL"]
    pub mod CSEL5 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CHAIN5 HWTS"]
    pub mod HWTS5 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CHAIN5 B2B"]
    pub mod B2B5 {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CHAIN5 IE"]
    pub mod IE5 {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ETC_TRIG Chain 6/7 Register"]
pub mod TRIG2_CHAIN_7_6 {
    #[doc = "CHAIN6 CSEL"]
    pub mod CSEL6 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CHAIN6 HWTS"]
    pub mod HWTS6 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CHAIN6 B2B"]
    pub mod B2B6 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CHAIN6 IE"]
    pub mod IE6 {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CHAIN7 CSEL"]
    pub mod CSEL7 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CHAIN7 HWTS"]
    pub mod HWTS7 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CHAIN7 B2B"]
    pub mod B2B7 {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CHAIN7 IE"]
    pub mod IE7 {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
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
#[doc = "ETC_TRIG3 Control Register"]
pub mod TRIG3_CTRL {
    #[doc = "Software write 1 as the TRIGGER. This register is self-clearing."]
    pub mod SW_TRIG {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TRIG mode register. 1'b0: hardware trigger. 1'b1: software trigger."]
    pub mod TRIG_MODE {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TRIG chain length to the ADC. 0: Trig length is 1; ... 7: Trig length is 8;"]
    pub mod TRIG_CHAIN {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "External trigger priority, 7 is highest, 0 is lowest ."]
    pub mod TRIG_PRIORITY {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TRIG mode control . 1'b0: Disable sync mode; 1'b1: Enable sync mode"]
    pub mod SYNC_MODE {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ETC_TRIG3 Counter Register"]
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
    #[doc = "CHAIN0 CSEL ADC channel selection"]
    pub mod CSEL0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CHAIN0 HWTS ADC hardware trigger selection"]
    pub mod HWTS0 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CHAIN0 B2B 1'b0: Disable B2B, wait until interval is reached 1'b1: Enable B2B, back to back ADC trigger"]
    pub mod B2B0 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CHAIN0 IE 2'b00: No interrupt when finished 2'b01: Finished Interrupt on Done0 2'b10: Finished Interrupt on Done1 2'b11: Finished Interrupt on Done2"]
    pub mod IE0 {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CHAIN1 CSEL ADC channel selection"]
    pub mod CSEL1 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CHAIN1 HWTS ADC hardware trigger selection"]
    pub mod HWTS1 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CHAIN1 B2B 1'b0: Disable B2B, wait until interval is reached 1'b1: Enable B2B, back to back ADC trigger"]
    pub mod B2B1 {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CHAIN1 IE 2'b00: No interrupt when finished 2'b01: Finished Interrupt on Done0 2'b10: Finished Interrupt on Done1 2'b11: Finished Interrupt on Done2"]
    pub mod IE1 {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ETC_TRIG Chain 2/3 Register"]
pub mod TRIG3_CHAIN_3_2 {
    #[doc = "CHAIN2 CSEL"]
    pub mod CSEL2 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CHAIN2 HWTS"]
    pub mod HWTS2 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CHAIN2 B2B"]
    pub mod B2B2 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CHAIN2 IE"]
    pub mod IE2 {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CHAIN3 CSEL"]
    pub mod CSEL3 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CHAIN3 HWTS"]
    pub mod HWTS3 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CHAIN3 B2B"]
    pub mod B2B3 {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CHAIN3 IE"]
    pub mod IE3 {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ETC_TRIG Chain 4/5 Register"]
pub mod TRIG3_CHAIN_5_4 {
    #[doc = "CHAIN4 CSEL"]
    pub mod CSEL4 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CHAIN4 HWTS"]
    pub mod HWTS4 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CHAIN4 B2B"]
    pub mod B2B4 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CHAIN4 IE"]
    pub mod IE4 {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CHAIN5 CSEL"]
    pub mod CSEL5 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CHAIN5 HWTS"]
    pub mod HWTS5 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CHAIN5 B2B"]
    pub mod B2B5 {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CHAIN5 IE"]
    pub mod IE5 {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ETC_TRIG Chain 6/7 Register"]
pub mod TRIG3_CHAIN_7_6 {
    #[doc = "CHAIN6 CSEL"]
    pub mod CSEL6 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CHAIN6 HWTS"]
    pub mod HWTS6 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CHAIN6 B2B"]
    pub mod B2B6 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CHAIN6 IE"]
    pub mod IE6 {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CHAIN7 CSEL"]
    pub mod CSEL7 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CHAIN7 HWTS"]
    pub mod HWTS7 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CHAIN7 B2B"]
    pub mod B2B7 {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CHAIN7 IE"]
    pub mod IE7 {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
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
