#[doc = "CAN"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "Module Configuration"]
    pub MCR: crate::RWRegister<u32>,
    #[doc = "Control 1"]
    pub CTRL1: crate::RWRegister<u32>,
    #[doc = "Free-Running Timer"]
    pub TIMER: crate::RWRegister<u32>,
    _reserved0: [u8; 0x04],
    #[doc = "RX Message Buffers Global Mask"]
    pub RXMGMASK: crate::RWRegister<u32>,
    #[doc = "Receive 14 Mask"]
    pub RX14MASK: crate::RWRegister<u32>,
    #[doc = "Receive 15 Mask"]
    pub RX15MASK: crate::RWRegister<u32>,
    #[doc = "Error Counter"]
    pub ECR: crate::RWRegister<u32>,
    #[doc = "Error and Status 1"]
    pub ESR1: crate::RWRegister<u32>,
    #[doc = "Interrupt Masks 2"]
    pub IMASK2: crate::RWRegister<u32>,
    #[doc = "Interrupt Masks 1"]
    pub IMASK1: crate::RWRegister<u32>,
    #[doc = "Interrupt Flags 2"]
    pub IFLAG2: crate::RWRegister<u32>,
    #[doc = "Interrupt Flags 1"]
    pub IFLAG1: crate::RWRegister<u32>,
    #[doc = "Control 2"]
    pub CTRL2: crate::RWRegister<u32>,
    #[doc = "Error and Status 2"]
    pub ESR2: crate::RORegister<u32>,
    _reserved1: [u8; 0x08],
    #[doc = "Cyclic Redundancy Check"]
    pub CRCR: crate::RORegister<u32>,
    #[doc = "Legacy RX FIFO Global Mask"]
    pub RXFGMASK: crate::RWRegister<u32>,
    #[doc = "Legacy RX FIFO Information"]
    pub RXFIR: crate::RORegister<u32>,
    #[doc = "CAN Bit Timing"]
    pub CBT: crate::RWRegister<u32>,
    _reserved2: [u8; 0x18],
    #[doc = "Interrupt Masks 3"]
    pub IMASK3: crate::RWRegister<u32>,
    _reserved3: [u8; 0x04],
    #[doc = "Interrupt Flags 3"]
    pub IFLAG3: crate::RWRegister<u32>,
    _reserved4: [u8; 0x08],
    #[doc = "Message Buffer 0 CS Register"]
    pub CS0: crate::RWRegister<u32>,
    #[doc = "Message Buffer 0 ID Register"]
    pub ID0: crate::RWRegister<u32>,
    #[doc = "Message Buffer 0 WORD_16B Register"]
    pub MB0_16B_WORD0_L: crate::RWRegister<u32>,
    #[doc = "Message Buffer 0 WORD_16B Register"]
    pub MB0_16B_WORD1_L: crate::RWRegister<u32>,
    #[doc = "Message Buffer 1 CS Register"]
    pub CS1: crate::RWRegister<u32>,
    #[doc = "Message Buffer 1 ID Register"]
    pub ID1: crate::RWRegister<u32>,
    #[doc = "Message Buffer 0 WORD_32B Register"]
    pub MB0_32B_WORD4_L: crate::RWRegister<u32>,
    #[doc = "Message Buffer 0 WORD_32B Register"]
    pub MB0_32B_WORD5_L: crate::RWRegister<u32>,
    #[doc = "Message Buffer 2 CS Register"]
    pub CS2: crate::RWRegister<u32>,
    #[doc = "Message Buffer 2 ID Register"]
    pub ID2: crate::RWRegister<u32>,
    #[doc = "Message Buffer 0 WORD_64B Register"]
    pub MB0_64B_WORD8_L: crate::RWRegister<u32>,
    #[doc = "Message Buffer 0 WORD_64B Register"]
    pub MB0_64B_WORD9_L: crate::RWRegister<u32>,
    #[doc = "Message Buffer 3 CS Register"]
    pub CS3: crate::RWRegister<u32>,
    #[doc = "Message Buffer 3 ID Register"]
    pub ID3: crate::RWRegister<u32>,
    #[doc = "Message Buffer 0 WORD_64B Register"]
    pub MB0_64B_WORD12_L: crate::RWRegister<u32>,
    #[doc = "Message Buffer 0 WORD_64B Register"]
    pub MB0_64B_WORD13_L: crate::RWRegister<u32>,
    #[doc = "Message Buffer 4 CS Register"]
    pub CS4: crate::RWRegister<u32>,
    #[doc = "Message Buffer 4 ID Register"]
    pub ID4: crate::RWRegister<u32>,
    #[doc = "Message Buffer 1 WORD_32B Register"]
    pub MB1_32B_WORD6_L: crate::RWRegister<u32>,
    #[doc = "Message Buffer 1 WORD_32B Register"]
    pub MB1_32B_WORD7_L: crate::RWRegister<u32>,
    #[doc = "Message Buffer 5 CS Register"]
    pub CS5: crate::RWRegister<u32>,
    #[doc = "Message Buffer 5 ID Register"]
    pub ID5: crate::RWRegister<u32>,
    #[doc = "Message Buffer 1 WORD_64B Register"]
    pub MB1_64B_WORD2_L: crate::RWRegister<u32>,
    #[doc = "Message Buffer 1 WORD_64B Register"]
    pub MB1_64B_WORD3_L: crate::RWRegister<u32>,
    #[doc = "Message Buffer 6 CS Register"]
    pub CS6: crate::RWRegister<u32>,
    #[doc = "Message Buffer 6 ID Register"]
    pub ID6: crate::RWRegister<u32>,
    #[doc = "Message Buffer 1 WORD_64B Register"]
    pub MB1_64B_WORD6_L: crate::RWRegister<u32>,
    #[doc = "Message Buffer 1 WORD_64B Register"]
    pub MB1_64B_WORD7_L: crate::RWRegister<u32>,
    #[doc = "Message Buffer 7 CS Register"]
    pub CS7: crate::RWRegister<u32>,
    #[doc = "Message Buffer 7 ID Register"]
    pub ID7: crate::RWRegister<u32>,
    #[doc = "Message Buffer 1 WORD_64B Register"]
    pub MB1_64B_WORD10_L: crate::RWRegister<u32>,
    #[doc = "Message Buffer 1 WORD_64B Register"]
    pub MB1_64B_WORD11_L: crate::RWRegister<u32>,
    #[doc = "Message Buffer 8 CS Register"]
    pub CS8: crate::RWRegister<u32>,
    #[doc = "Message Buffer 8 ID Register"]
    pub ID8: crate::RWRegister<u32>,
    #[doc = "Message Buffer 1 WORD_64B Register"]
    pub MB1_64B_WORD14_L: crate::RWRegister<u32>,
    #[doc = "Message Buffer 1 WORD_64B Register"]
    pub MB1_64B_WORD15_L: crate::RWRegister<u32>,
    #[doc = "Message Buffer 9 CS Register"]
    pub CS9: crate::RWRegister<u32>,
    #[doc = "Message Buffer 9 ID Register"]
    pub ID9: crate::RWRegister<u32>,
    #[doc = "Message Buffer 2 WORD_64B Register"]
    pub MB2_64B_WORD0_L: crate::RWRegister<u32>,
    #[doc = "Message Buffer 2 WORD_64B Register"]
    pub MB2_64B_WORD1_L: crate::RWRegister<u32>,
    #[doc = "Message Buffer 10 CS Register"]
    pub CS10: crate::RWRegister<u32>,
    #[doc = "Message Buffer 10 ID Register"]
    pub ID10: crate::RWRegister<u32>,
    #[doc = "Message Buffer 10 WORD_8B Register"]
    pub MB10_8B_WORD0: crate::RWRegister<u32>,
    #[doc = "Message Buffer 10 WORD_8B Register"]
    pub MB10_8B_WORD1: crate::RWRegister<u32>,
    #[doc = "Message Buffer 11 CS Register"]
    pub CS11: crate::RWRegister<u32>,
    #[doc = "Message Buffer 11 ID Register"]
    pub ID11: crate::RWRegister<u32>,
    #[doc = "Message Buffer 11 WORD_8B Register"]
    pub MB11_8B_WORD0: crate::RWRegister<u32>,
    #[doc = "Message Buffer 11 WORD_8B Register"]
    pub MB11_8B_WORD1: crate::RWRegister<u32>,
    #[doc = "Message Buffer 12 CS Register"]
    pub CS12: crate::RWRegister<u32>,
    #[doc = "Message Buffer 12 ID Register"]
    pub ID12: crate::RWRegister<u32>,
    #[doc = "Message Buffer 12 WORD_8B Register"]
    pub MB12_8B_WORD0: crate::RWRegister<u32>,
    #[doc = "Message Buffer 12 WORD_8B Register"]
    pub MB12_8B_WORD1: crate::RWRegister<u32>,
    #[doc = "Message Buffer 13 CS Register"]
    pub CS13: crate::RWRegister<u32>,
    #[doc = "Message Buffer 13 ID Register"]
    pub ID13: crate::RWRegister<u32>,
    #[doc = "Message Buffer 13 WORD_8B Register"]
    pub MB13_8B_WORD0: crate::RWRegister<u32>,
    #[doc = "Message Buffer 13 WORD_8B Register"]
    pub MB13_8B_WORD1: crate::RWRegister<u32>,
    #[doc = "Message Buffer 14 CS Register"]
    pub CS14: crate::RWRegister<u32>,
    #[doc = "Message Buffer 14 ID Register"]
    pub ID14: crate::RWRegister<u32>,
    #[doc = "Message Buffer 14 WORD_8B Register"]
    pub MB14_8B_WORD0: crate::RWRegister<u32>,
    #[doc = "Message Buffer 14 WORD_8B Register"]
    pub MB14_8B_WORD1: crate::RWRegister<u32>,
    #[doc = "Message Buffer 15 CS Register"]
    pub CS15: crate::RWRegister<u32>,
    #[doc = "Message Buffer 15 ID Register"]
    pub ID15: crate::RWRegister<u32>,
    #[doc = "Message Buffer 10 WORD_16B Register"]
    pub MB10_16B_WORD0_L: crate::RWRegister<u32>,
    #[doc = "Message Buffer 10 WORD_16B Register"]
    pub MB10_16B_WORD1_L: crate::RWRegister<u32>,
    #[doc = "Message Buffer 16 CS Register"]
    pub CS16: crate::RWRegister<u32>,
    #[doc = "Message Buffer 16 ID Register"]
    pub ID16: crate::RWRegister<u32>,
    #[doc = "Message Buffer 11 CS Register"]
    pub MB11_16B_CS_L: crate::RWRegister<u32>,
    #[doc = "Message Buffer 11 ID Register"]
    pub MB11_16B_ID_L: crate::RWRegister<u32>,
    #[doc = "Message Buffer 17 CS Register"]
    pub CS17: crate::RWRegister<u32>,
    #[doc = "Message Buffer 17 ID Register"]
    pub ID17: crate::RWRegister<u32>,
    #[doc = "Message Buffer 11 WORD_16B Register"]
    pub MB11_16B_WORD2_L: crate::RWRegister<u32>,
    #[doc = "Message Buffer 11 WORD_16B Register"]
    pub MB11_16B_WORD3_L: crate::RWRegister<u32>,
    #[doc = "Message Buffer 18 CS Register"]
    pub CS18: crate::RWRegister<u32>,
    #[doc = "Message Buffer 18 ID Register"]
    pub ID18: crate::RWRegister<u32>,
    #[doc = "Message Buffer 12 WORD_16B Register"]
    pub MB12_16B_WORD0_L: crate::RWRegister<u32>,
    #[doc = "Message Buffer 12 WORD_16B Register"]
    pub MB12_16B_WORD1_L: crate::RWRegister<u32>,
    #[doc = "Message Buffer 19 CS Register"]
    pub CS19: crate::RWRegister<u32>,
    #[doc = "Message Buffer 19 ID Register"]
    pub ID19: crate::RWRegister<u32>,
    #[doc = "Message Buffer 13 CS Register"]
    pub MB13_16B_CS_L: crate::RWRegister<u32>,
    #[doc = "Message Buffer 13 ID Register"]
    pub MB13_16B_ID_L: crate::RWRegister<u32>,
    #[doc = "Message Buffer 20 CS Register"]
    pub CS20: crate::RWRegister<u32>,
    #[doc = "Message Buffer 20 ID Register"]
    pub ID20: crate::RWRegister<u32>,
    #[doc = "Message Buffer 13 WORD_16B Register"]
    pub MB13_16B_WORD2_L: crate::RWRegister<u32>,
    #[doc = "Message Buffer 13 WORD_16B Register"]
    pub MB13_16B_WORD3_L: crate::RWRegister<u32>,
    #[doc = "Message Buffer 21 CS Register"]
    pub CS21: crate::RWRegister<u32>,
    #[doc = "Message Buffer 21 ID Register"]
    pub ID21: crate::RWRegister<u32>,
    #[doc = "Message Buffer 14 WORD_16B Register"]
    pub MB14_16B_WORD0_L: crate::RWRegister<u32>,
    #[doc = "Message Buffer 14 WORD_16B Register"]
    pub MB14_16B_WORD1_L: crate::RWRegister<u32>,
    #[doc = "Message Buffer 22 CS Register"]
    pub CS22: crate::RWRegister<u32>,
    #[doc = "Message Buffer 22 ID Register"]
    pub ID22: crate::RWRegister<u32>,
    #[doc = "Message Buffer 15 CS Register"]
    pub MB15_16B_CS_L: crate::RWRegister<u32>,
    #[doc = "Message Buffer 15 ID Register"]
    pub MB15_16B_ID_L: crate::RWRegister<u32>,
    #[doc = "Message Buffer 23 CS Register"]
    pub CS23: crate::RWRegister<u32>,
    #[doc = "Message Buffer 23 ID Register"]
    pub ID23: crate::RWRegister<u32>,
    #[doc = "Message Buffer 15 WORD_16B Register"]
    pub MB15_16B_WORD2_L: crate::RWRegister<u32>,
    #[doc = "Message Buffer 15 WORD_16B Register"]
    pub MB15_16B_WORD3_L: crate::RWRegister<u32>,
    #[doc = "Message Buffer 24 CS Register"]
    pub CS24: crate::RWRegister<u32>,
    #[doc = "Message Buffer 24 ID Register"]
    pub ID24: crate::RWRegister<u32>,
    #[doc = "Message Buffer 16 WORD_16B Register"]
    pub MB16_16B_WORD0_L: crate::RWRegister<u32>,
    #[doc = "Message Buffer 16 WORD_16B Register"]
    pub MB16_16B_WORD1_L: crate::RWRegister<u32>,
    #[doc = "Message Buffer 25 CS Register"]
    pub CS25: crate::RWRegister<u32>,
    #[doc = "Message Buffer 25 ID Register"]
    pub ID25: crate::RWRegister<u32>,
    #[doc = "Message Buffer 10 WORD_32B Register"]
    pub MB10_32B_WORD0_L: crate::RWRegister<u32>,
    #[doc = "Message Buffer 10 WORD_32B Register"]
    pub MB10_32B_WORD1_L: crate::RWRegister<u32>,
    #[doc = "Message Buffer 26 CS Register"]
    pub CS26: crate::RWRegister<u32>,
    #[doc = "Message Buffer 26 ID Register"]
    pub ID26: crate::RWRegister<u32>,
    #[doc = "Message Buffer 10 WORD_32B Register"]
    pub MB10_32B_WORD4_L: crate::RWRegister<u32>,
    #[doc = "Message Buffer 10 WORD_32B Register"]
    pub MB10_32B_WORD5_L: crate::RWRegister<u32>,
    #[doc = "Message Buffer 27 CS Register"]
    pub CS27: crate::RWRegister<u32>,
    #[doc = "Message Buffer 27 ID Register"]
    pub ID27: crate::RWRegister<u32>,
    #[doc = "Message Buffer 11 CS Register"]
    pub MB11_32B_CS_L: crate::RWRegister<u32>,
    #[doc = "Message Buffer 11 ID Register"]
    pub MB11_32B_ID_L: crate::RWRegister<u32>,
    #[doc = "Message Buffer 28 CS Register"]
    pub CS28: crate::RWRegister<u32>,
    #[doc = "Message Buffer 28 ID Register"]
    pub ID28: crate::RWRegister<u32>,
    #[doc = "Message Buffer 11 WORD_32B Register"]
    pub MB11_32B_WORD2_L: crate::RWRegister<u32>,
    #[doc = "Message Buffer 11 WORD_32B Register"]
    pub MB11_32B_WORD3_L: crate::RWRegister<u32>,
    #[doc = "Message Buffer 29 CS Register"]
    pub CS29: crate::RWRegister<u32>,
    #[doc = "Message Buffer 29 ID Register"]
    pub ID29: crate::RWRegister<u32>,
    #[doc = "Message Buffer 11 WORD_32B Register"]
    pub MB11_32B_WORD6_L: crate::RWRegister<u32>,
    #[doc = "Message Buffer 11 WORD_32B Register"]
    pub MB11_32B_WORD7_L: crate::RWRegister<u32>,
    #[doc = "Message Buffer 30 CS Register"]
    pub CS30: crate::RWRegister<u32>,
    #[doc = "Message Buffer 30 ID Register"]
    pub ID30: crate::RWRegister<u32>,
    #[doc = "Message Buffer 20 WORD_16B Register"]
    pub MB20_16B_WORD0_L: crate::RWRegister<u32>,
    #[doc = "Message Buffer 20 WORD_16B Register"]
    pub MB20_16B_WORD1_L: crate::RWRegister<u32>,
    #[doc = "Message Buffer 31 CS Register"]
    pub CS31: crate::RWRegister<u32>,
    #[doc = "Message Buffer 31 ID Register"]
    pub ID31: crate::RWRegister<u32>,
    #[doc = "Message Buffer 31 WORD_8B Register"]
    pub MB31_8B_WORD0: crate::RWRegister<u32>,
    #[doc = "Message Buffer 31 WORD_8B Register"]
    pub MB31_8B_WORD1: crate::RWRegister<u32>,
    #[doc = "Message Buffer 32 CS Register"]
    pub CS32: crate::RWRegister<u32>,
    #[doc = "Message Buffer 32 ID Register"]
    pub ID32: crate::RWRegister<u32>,
    #[doc = "Message Buffer 0 WORD_16B Register"]
    pub MB0_16B_WORD0_M: crate::RWRegister<u32>,
    #[doc = "Message Buffer 0 WORD_16B Register"]
    pub MB0_16B_WORD1_M: crate::RWRegister<u32>,
    #[doc = "Message Buffer 33 CS Register"]
    pub CS33: crate::RWRegister<u32>,
    #[doc = "Message Buffer 33 ID Register"]
    pub ID33: crate::RWRegister<u32>,
    #[doc = "Message Buffer 0 WORD_32B Register"]
    pub MB0_32B_WORD4_M: crate::RWRegister<u32>,
    #[doc = "Message Buffer 0 WORD_32B Register"]
    pub MB0_32B_WORD5_M: crate::RWRegister<u32>,
    #[doc = "Message Buffer 34 CS Register"]
    pub CS34: crate::RWRegister<u32>,
    #[doc = "Message Buffer 34 ID Register"]
    pub ID34: crate::RWRegister<u32>,
    #[doc = "Message Buffer 0 WORD_64B Register"]
    pub MB0_64B_WORD8_M: crate::RWRegister<u32>,
    #[doc = "Message Buffer 0 WORD_64B Register"]
    pub MB0_64B_WORD9_M: crate::RWRegister<u32>,
    #[doc = "Message Buffer 35 CS Register"]
    pub CS35: crate::RWRegister<u32>,
    #[doc = "Message Buffer 35 ID Register"]
    pub ID35: crate::RWRegister<u32>,
    #[doc = "Message Buffer 0 WORD_64B Register"]
    pub MB0_64B_WORD12_M: crate::RWRegister<u32>,
    #[doc = "Message Buffer 0 WORD_64B Register"]
    pub MB0_64B_WORD13_M: crate::RWRegister<u32>,
    #[doc = "Message Buffer 36 CS Register"]
    pub CS36: crate::RWRegister<u32>,
    #[doc = "Message Buffer 36 ID Register"]
    pub ID36: crate::RWRegister<u32>,
    #[doc = "Message Buffer 1 WORD_32B Register"]
    pub MB1_32B_WORD6_M: crate::RWRegister<u32>,
    #[doc = "Message Buffer 1 WORD_32B Register"]
    pub MB1_32B_WORD7_M: crate::RWRegister<u32>,
    #[doc = "Message Buffer 37 CS Register"]
    pub CS37: crate::RWRegister<u32>,
    #[doc = "Message Buffer 37 ID Register"]
    pub ID37: crate::RWRegister<u32>,
    #[doc = "Message Buffer 1 WORD_64B Register"]
    pub MB1_64B_WORD2_M: crate::RWRegister<u32>,
    #[doc = "Message Buffer 1 WORD_64B Register"]
    pub MB1_64B_WORD3_M: crate::RWRegister<u32>,
    #[doc = "Message Buffer 38 CS Register"]
    pub CS38: crate::RWRegister<u32>,
    #[doc = "Message Buffer 38 ID Register"]
    pub ID38: crate::RWRegister<u32>,
    #[doc = "Message Buffer 1 WORD_64B Register"]
    pub MB1_64B_WORD6_M: crate::RWRegister<u32>,
    #[doc = "Message Buffer 1 WORD_64B Register"]
    pub MB1_64B_WORD7_M: crate::RWRegister<u32>,
    #[doc = "Message Buffer 39 CS Register"]
    pub CS39: crate::RWRegister<u32>,
    #[doc = "Message Buffer 39 ID Register"]
    pub ID39: crate::RWRegister<u32>,
    #[doc = "Message Buffer 1 WORD_64B Register"]
    pub MB1_64B_WORD10_M: crate::RWRegister<u32>,
    #[doc = "Message Buffer 1 WORD_64B Register"]
    pub MB1_64B_WORD11_M: crate::RWRegister<u32>,
    #[doc = "Message Buffer 40 CS Register"]
    pub CS40: crate::RWRegister<u32>,
    #[doc = "Message Buffer 40 ID Register"]
    pub ID40: crate::RWRegister<u32>,
    #[doc = "Message Buffer 1 WORD_64B Register"]
    pub MB1_64B_WORD14_M: crate::RWRegister<u32>,
    #[doc = "Message Buffer 1 WORD_64B Register"]
    pub MB1_64B_WORD15_M: crate::RWRegister<u32>,
    #[doc = "Message Buffer 41 CS Register"]
    pub CS41: crate::RWRegister<u32>,
    #[doc = "Message Buffer 41 ID Register"]
    pub ID41: crate::RWRegister<u32>,
    #[doc = "Message Buffer 2 WORD_64B Register"]
    pub MB2_64B_WORD0_M: crate::RWRegister<u32>,
    #[doc = "Message Buffer 2 WORD_64B Register"]
    pub MB2_64B_WORD1_M: crate::RWRegister<u32>,
    #[doc = "Message Buffer 42 CS Register"]
    pub CS42: crate::RWRegister<u32>,
    #[doc = "Message Buffer 42 ID Register"]
    pub ID42: crate::RWRegister<u32>,
    #[doc = "Message Buffer 2 WORD_64B Register"]
    pub MB2_64B_WORD4_M: crate::RWRegister<u32>,
    #[doc = "Message Buffer 2 WORD_64B Register"]
    pub MB2_64B_WORD5_M: crate::RWRegister<u32>,
    #[doc = "Message Buffer 43 CS Register"]
    pub CS43: crate::RWRegister<u32>,
    #[doc = "Message Buffer 43 ID Register"]
    pub ID43: crate::RWRegister<u32>,
    #[doc = "Message Buffer 2 WORD_64B Register"]
    pub MB2_64B_WORD8_M: crate::RWRegister<u32>,
    #[doc = "Message Buffer 2 WORD_64B Register"]
    pub MB2_64B_WORD9_M: crate::RWRegister<u32>,
    #[doc = "Message Buffer 44 CS Register"]
    pub CS44: crate::RWRegister<u32>,
    #[doc = "Message Buffer 44 ID Register"]
    pub ID44: crate::RWRegister<u32>,
    #[doc = "Message Buffer 2 WORD_64B Register"]
    pub MB2_64B_WORD12_M: crate::RWRegister<u32>,
    #[doc = "Message Buffer 2 WORD_64B Register"]
    pub MB2_64B_WORD13_M: crate::RWRegister<u32>,
    #[doc = "Message Buffer 45 CS Register"]
    pub CS45: crate::RWRegister<u32>,
    #[doc = "Message Buffer 45 ID Register"]
    pub ID45: crate::RWRegister<u32>,
    #[doc = "Message Buffer 3 CS Register"]
    pub MB3_64B_CS_M: crate::RWRegister<u32>,
    #[doc = "Message Buffer 3 ID Register"]
    pub MB3_64B_ID_M: crate::RWRegister<u32>,
    #[doc = "Message Buffer 46 CS Register"]
    pub CS46: crate::RWRegister<u32>,
    #[doc = "Message Buffer 46 ID Register"]
    pub ID46: crate::RWRegister<u32>,
    #[doc = "Message Buffer 3 WORD_64B Register"]
    pub MB3_64B_WORD2_M: crate::RWRegister<u32>,
    #[doc = "Message Buffer 3 WORD_64B Register"]
    pub MB3_64B_WORD3_M: crate::RWRegister<u32>,
    #[doc = "Message Buffer 47 CS Register"]
    pub CS47: crate::RWRegister<u32>,
    #[doc = "Message Buffer 47 ID Register"]
    pub ID47: crate::RWRegister<u32>,
    #[doc = "Message Buffer 10 WORD_16B Register"]
    pub MB10_16B_WORD0_M: crate::RWRegister<u32>,
    #[doc = "Message Buffer 10 WORD_16B Register"]
    pub MB10_16B_WORD1_M: crate::RWRegister<u32>,
    #[doc = "Message Buffer 48 CS Register"]
    pub CS48: crate::RWRegister<u32>,
    #[doc = "Message Buffer 48 ID Register"]
    pub ID48: crate::RWRegister<u32>,
    #[doc = "Message Buffer 11 CS Register"]
    pub MB11_16B_CS_M: crate::RWRegister<u32>,
    #[doc = "Message Buffer 11 ID Register"]
    pub MB11_16B_ID_M: crate::RWRegister<u32>,
    #[doc = "Message Buffer 49 CS Register"]
    pub CS49: crate::RWRegister<u32>,
    #[doc = "Message Buffer 49 ID Register"]
    pub ID49: crate::RWRegister<u32>,
    #[doc = "Message Buffer 11 WORD_16B Register"]
    pub MB11_16B_WORD2_M: crate::RWRegister<u32>,
    #[doc = "Message Buffer 11 WORD_16B Register"]
    pub MB11_16B_WORD3_M: crate::RWRegister<u32>,
    #[doc = "Message Buffer 50 CS Register"]
    pub CS50: crate::RWRegister<u32>,
    #[doc = "Message Buffer 50 ID Register"]
    pub ID50: crate::RWRegister<u32>,
    #[doc = "Message Buffer 12 WORD_16B Register"]
    pub MB12_16B_WORD0_M: crate::RWRegister<u32>,
    #[doc = "Message Buffer 12 WORD_16B Register"]
    pub MB12_16B_WORD1_M: crate::RWRegister<u32>,
    #[doc = "Message Buffer 51 CS Register"]
    pub CS51: crate::RWRegister<u32>,
    #[doc = "Message Buffer 51 ID Register"]
    pub ID51: crate::RWRegister<u32>,
    #[doc = "Message Buffer 13 CS Register"]
    pub MB13_16B_CS_M: crate::RWRegister<u32>,
    #[doc = "Message Buffer 13 ID Register"]
    pub MB13_16B_ID_M: crate::RWRegister<u32>,
    #[doc = "Message Buffer 52 CS Register"]
    pub CS52: crate::RWRegister<u32>,
    #[doc = "Message Buffer 52 ID Register"]
    pub ID52: crate::RWRegister<u32>,
    #[doc = "Message Buffer 13 WORD_16B Register"]
    pub MB13_16B_WORD2_M: crate::RWRegister<u32>,
    #[doc = "Message Buffer 13 WORD_16B Register"]
    pub MB13_16B_WORD3_M: crate::RWRegister<u32>,
    #[doc = "Message Buffer 53 CS Register"]
    pub CS53: crate::RWRegister<u32>,
    #[doc = "Message Buffer 53 ID Register"]
    pub ID53: crate::RWRegister<u32>,
    #[doc = "Message Buffer 14 WORD_16B Register"]
    pub MB14_16B_WORD0_M: crate::RWRegister<u32>,
    #[doc = "Message Buffer 14 WORD_16B Register"]
    pub MB14_16B_WORD1_M: crate::RWRegister<u32>,
    #[doc = "Message Buffer 54 CS Register"]
    pub CS54: crate::RWRegister<u32>,
    #[doc = "Message Buffer 54 ID Register"]
    pub ID54: crate::RWRegister<u32>,
    #[doc = "Message Buffer 15 CS Register"]
    pub MB15_16B_CS_M: crate::RWRegister<u32>,
    #[doc = "Message Buffer 15 ID Register"]
    pub MB15_16B_ID_M: crate::RWRegister<u32>,
    #[doc = "Message Buffer 55 CS Register"]
    pub CS55: crate::RWRegister<u32>,
    #[doc = "Message Buffer 55 ID Register"]
    pub ID55: crate::RWRegister<u32>,
    #[doc = "Message Buffer 15 WORD_16B Register"]
    pub MB15_16B_WORD2_M: crate::RWRegister<u32>,
    #[doc = "Message Buffer 15 WORD_16B Register"]
    pub MB15_16B_WORD3_M: crate::RWRegister<u32>,
    #[doc = "Message Buffer 56 CS Register"]
    pub CS56: crate::RWRegister<u32>,
    #[doc = "Message Buffer 56 ID Register"]
    pub ID56: crate::RWRegister<u32>,
    #[doc = "Message Buffer 16 WORD_16B Register"]
    pub MB16_16B_WORD0_M: crate::RWRegister<u32>,
    #[doc = "Message Buffer 16 WORD_16B Register"]
    pub MB16_16B_WORD1_M: crate::RWRegister<u32>,
    #[doc = "Message Buffer 57 CS Register"]
    pub CS57: crate::RWRegister<u32>,
    #[doc = "Message Buffer 57 ID Register"]
    pub ID57: crate::RWRegister<u32>,
    #[doc = "Message Buffer 10 WORD_32B Register"]
    pub MB10_32B_WORD0_M: crate::RWRegister<u32>,
    #[doc = "Message Buffer 10 WORD_32B Register"]
    pub MB10_32B_WORD1_M: crate::RWRegister<u32>,
    #[doc = "Message Buffer 58 CS Register"]
    pub CS58: crate::RWRegister<u32>,
    #[doc = "Message Buffer 58 ID Register"]
    pub ID58: crate::RWRegister<u32>,
    #[doc = "Message Buffer 10 WORD_32B Register"]
    pub MB10_32B_WORD4_M: crate::RWRegister<u32>,
    #[doc = "Message Buffer 10 WORD_32B Register"]
    pub MB10_32B_WORD5_M: crate::RWRegister<u32>,
    #[doc = "Message Buffer 59 CS Register"]
    pub CS59: crate::RWRegister<u32>,
    #[doc = "Message Buffer 59 ID Register"]
    pub ID59: crate::RWRegister<u32>,
    #[doc = "Message Buffer 11 CS Register"]
    pub MB11_32B_CS_M: crate::RWRegister<u32>,
    #[doc = "Message Buffer 11 ID Register"]
    pub MB11_32B_ID_M: crate::RWRegister<u32>,
    #[doc = "Message Buffer 60 CS Register"]
    pub CS60: crate::RWRegister<u32>,
    #[doc = "Message Buffer 60 ID Register"]
    pub ID60: crate::RWRegister<u32>,
    #[doc = "Message Buffer 11 WORD_32B Register"]
    pub MB11_32B_WORD2_M: crate::RWRegister<u32>,
    #[doc = "Message Buffer 11 WORD_32B Register"]
    pub MB11_32B_WORD3_M: crate::RWRegister<u32>,
    #[doc = "Message Buffer 61 CS Register"]
    pub CS61: crate::RWRegister<u32>,
    #[doc = "Message Buffer 61 ID Register"]
    pub ID61: crate::RWRegister<u32>,
    #[doc = "Message Buffer 11 WORD_32B Register"]
    pub MB11_32B_WORD6_M: crate::RWRegister<u32>,
    #[doc = "Message Buffer 11 WORD_32B Register"]
    pub MB11_32B_WORD7_M: crate::RWRegister<u32>,
    #[doc = "Message Buffer 62 CS Register"]
    pub CS62: crate::RWRegister<u32>,
    #[doc = "Message Buffer 62 ID Register"]
    pub ID62: crate::RWRegister<u32>,
    #[doc = "Message Buffer 20 WORD_16B Register"]
    pub MB20_16B_WORD0_M: crate::RWRegister<u32>,
    #[doc = "Message Buffer 20 WORD_16B Register"]
    pub MB20_16B_WORD1_M: crate::RWRegister<u32>,
    #[doc = "Message Buffer 63 CS Register"]
    pub CS63: crate::RWRegister<u32>,
    #[doc = "Message Buffer 63 ID Register"]
    pub ID63: crate::RWRegister<u32>,
    #[doc = "Message Buffer 63 WORD_8B Register"]
    pub MB63_8B_WORD0: crate::RWRegister<u32>,
    #[doc = "Message Buffer 63 WORD_8B Register"]
    pub MB63_8B_WORD1: crate::RWRegister<u32>,
    #[doc = "Message Buffer 64 CS Register"]
    pub CS64: crate::RWRegister<u32>,
    #[doc = "Message Buffer 64 ID Register"]
    pub ID64: crate::RWRegister<u32>,
    #[doc = "Message Buffer 0 WORD_16B Register"]
    pub MB0_16B_WORD0_H: crate::RWRegister<u32>,
    #[doc = "Message Buffer 0 WORD_16B Register"]
    pub MB0_16B_WORD1_H: crate::RWRegister<u32>,
    #[doc = "Message Buffer 65 CS Register"]
    pub CS65: crate::RWRegister<u32>,
    #[doc = "Message Buffer 65 ID Register"]
    pub ID65: crate::RWRegister<u32>,
    #[doc = "Message Buffer 0 WORD_32B Register"]
    pub MB0_32B_WORD4_H: crate::RWRegister<u32>,
    #[doc = "Message Buffer 0 WORD_32B Register"]
    pub MB0_32B_WORD5_H: crate::RWRegister<u32>,
    #[doc = "Message Buffer 66 CS Register"]
    pub CS66: crate::RWRegister<u32>,
    #[doc = "Message Buffer 66 ID Register"]
    pub ID66: crate::RWRegister<u32>,
    #[doc = "Message Buffer 0 WORD_64B Register"]
    pub MB0_64B_WORD8_H: crate::RWRegister<u32>,
    #[doc = "Message Buffer 0 WORD_64B Register"]
    pub MB0_64B_WORD9_H: crate::RWRegister<u32>,
    #[doc = "Message Buffer 67 CS Register"]
    pub CS67: crate::RWRegister<u32>,
    #[doc = "Message Buffer 67 ID Register"]
    pub ID67: crate::RWRegister<u32>,
    #[doc = "Message Buffer 0 WORD_64B Register"]
    pub MB0_64B_WORD12_H: crate::RWRegister<u32>,
    #[doc = "Message Buffer 0 WORD_64B Register"]
    pub MB0_64B_WORD13_H: crate::RWRegister<u32>,
    #[doc = "Message Buffer 68 CS Register"]
    pub CS68: crate::RWRegister<u32>,
    #[doc = "Message Buffer 68 ID Register"]
    pub ID68: crate::RWRegister<u32>,
    #[doc = "Message Buffer 1 WORD_32B Register"]
    pub MB1_32B_WORD6_H: crate::RWRegister<u32>,
    #[doc = "Message Buffer 1 WORD_32B Register"]
    pub MB1_32B_WORD7_H: crate::RWRegister<u32>,
    #[doc = "Message Buffer 69 CS Register"]
    pub CS69: crate::RWRegister<u32>,
    #[doc = "Message Buffer 69 ID Register"]
    pub ID69: crate::RWRegister<u32>,
    #[doc = "Message Buffer 1 WORD_64B Register"]
    pub MB1_64B_WORD2_H: crate::RWRegister<u32>,
    #[doc = "Message Buffer 1 WORD_64B Register"]
    pub MB1_64B_WORD3_H: crate::RWRegister<u32>,
    #[doc = "Message Buffer 70 CS Register"]
    pub CS70: crate::RWRegister<u32>,
    #[doc = "Message Buffer 70 ID Register"]
    pub ID70: crate::RWRegister<u32>,
    #[doc = "Message Buffer 1 WORD_64B Register"]
    pub MB1_64B_WORD6_H: crate::RWRegister<u32>,
    #[doc = "Message Buffer 1 WORD_64B Register"]
    pub MB1_64B_WORD7_H: crate::RWRegister<u32>,
    #[doc = "Message Buffer 71 CS Register"]
    pub CS71: crate::RWRegister<u32>,
    #[doc = "Message Buffer 71 ID Register"]
    pub ID71: crate::RWRegister<u32>,
    #[doc = "Message Buffer 1 WORD_64B Register"]
    pub MB1_64B_WORD10_H: crate::RWRegister<u32>,
    #[doc = "Message Buffer 1 WORD_64B Register"]
    pub MB1_64B_WORD11_H: crate::RWRegister<u32>,
    #[doc = "Message Buffer 72 CS Register"]
    pub CS72: crate::RWRegister<u32>,
    #[doc = "Message Buffer 72 ID Register"]
    pub ID72: crate::RWRegister<u32>,
    #[doc = "Message Buffer 1 WORD_64B Register"]
    pub MB1_64B_WORD14_H: crate::RWRegister<u32>,
    #[doc = "Message Buffer 1 WORD_64B Register"]
    pub MB1_64B_WORD15_H: crate::RWRegister<u32>,
    #[doc = "Message Buffer 73 CS Register"]
    pub CS73: crate::RWRegister<u32>,
    #[doc = "Message Buffer 73 ID Register"]
    pub ID73: crate::RWRegister<u32>,
    #[doc = "Message Buffer 2 WORD_64B Register"]
    pub MB2_64B_WORD0_H: crate::RWRegister<u32>,
    #[doc = "Message Buffer 2 WORD_64B Register"]
    pub MB2_64B_WORD1_H: crate::RWRegister<u32>,
    #[doc = "Message Buffer 74 CS Register"]
    pub CS74: crate::RWRegister<u32>,
    #[doc = "Message Buffer 74 ID Register"]
    pub ID74: crate::RWRegister<u32>,
    #[doc = "Message Buffer 2 WORD_64B Register"]
    pub MB2_64B_WORD4_H: crate::RWRegister<u32>,
    #[doc = "Message Buffer 2 WORD_64B Register"]
    pub MB2_64B_WORD5_H: crate::RWRegister<u32>,
    #[doc = "Message Buffer 75 CS Register"]
    pub CS75: crate::RWRegister<u32>,
    #[doc = "Message Buffer 75 ID Register"]
    pub ID75: crate::RWRegister<u32>,
    #[doc = "Message Buffer 2 WORD_64B Register"]
    pub MB2_64B_WORD8_H: crate::RWRegister<u32>,
    #[doc = "Message Buffer 2 WORD_64B Register"]
    pub MB2_64B_WORD9_H: crate::RWRegister<u32>,
    #[doc = "Message Buffer 76 CS Register"]
    pub CS76: crate::RWRegister<u32>,
    #[doc = "Message Buffer 76 ID Register"]
    pub ID76: crate::RWRegister<u32>,
    #[doc = "Message Buffer 2 WORD_64B Register"]
    pub MB2_64B_WORD12_H: crate::RWRegister<u32>,
    #[doc = "Message Buffer 2 WORD_64B Register"]
    pub MB2_64B_WORD13_H: crate::RWRegister<u32>,
    #[doc = "Message Buffer 77 CS Register"]
    pub CS77: crate::RWRegister<u32>,
    #[doc = "Message Buffer 77 ID Register"]
    pub ID77: crate::RWRegister<u32>,
    #[doc = "Message Buffer 3 CS Register"]
    pub MB3_64B_CS_H: crate::RWRegister<u32>,
    #[doc = "Message Buffer 3 ID Register"]
    pub MB3_64B_ID_H: crate::RWRegister<u32>,
    #[doc = "Message Buffer 78 CS Register"]
    pub CS78: crate::RWRegister<u32>,
    #[doc = "Message Buffer 78 ID Register"]
    pub ID78: crate::RWRegister<u32>,
    #[doc = "Message Buffer 3 WORD_64B Register"]
    pub MB3_64B_WORD2_H: crate::RWRegister<u32>,
    #[doc = "Message Buffer 3 WORD_64B Register"]
    pub MB3_64B_WORD3_H: crate::RWRegister<u32>,
    #[doc = "Message Buffer 79 CS Register"]
    pub CS79: crate::RWRegister<u32>,
    #[doc = "Message Buffer 79 ID Register"]
    pub ID79: crate::RWRegister<u32>,
    #[doc = "Message Buffer 10 WORD_16B Register"]
    pub MB10_16B_WORD0_H: crate::RWRegister<u32>,
    #[doc = "Message Buffer 10 WORD_16B Register"]
    pub MB10_16B_WORD1_H: crate::RWRegister<u32>,
    #[doc = "Message Buffer 80 CS Register"]
    pub CS80: crate::RWRegister<u32>,
    #[doc = "Message Buffer 80 ID Register"]
    pub ID80: crate::RWRegister<u32>,
    #[doc = "Message Buffer 11 CS Register"]
    pub MB11_16B_CS_H: crate::RWRegister<u32>,
    #[doc = "Message Buffer 11 ID Register"]
    pub MB11_16B_ID_H: crate::RWRegister<u32>,
    #[doc = "Message Buffer 81 CS Register"]
    pub CS81: crate::RWRegister<u32>,
    #[doc = "Message Buffer 81 ID Register"]
    pub ID81: crate::RWRegister<u32>,
    #[doc = "Message Buffer 11 WORD_16B Register"]
    pub MB11_16B_WORD2_H: crate::RWRegister<u32>,
    #[doc = "Message Buffer 11 WORD_16B Register"]
    pub MB11_16B_WORD3_H: crate::RWRegister<u32>,
    #[doc = "Message Buffer 82 CS Register"]
    pub CS82: crate::RWRegister<u32>,
    #[doc = "Message Buffer 82 ID Register"]
    pub ID82: crate::RWRegister<u32>,
    #[doc = "Message Buffer 12 WORD_16B Register"]
    pub MB12_16B_WORD0_H: crate::RWRegister<u32>,
    #[doc = "Message Buffer 12 WORD_16B Register"]
    pub MB12_16B_WORD1_H: crate::RWRegister<u32>,
    #[doc = "Message Buffer 83 CS Register"]
    pub CS83: crate::RWRegister<u32>,
    #[doc = "Message Buffer 83 ID Register"]
    pub ID83: crate::RWRegister<u32>,
    #[doc = "Message Buffer 13 CS Register"]
    pub MB13_16B_CS_H: crate::RWRegister<u32>,
    #[doc = "Message Buffer 13 ID Register"]
    pub MB13_16B_ID_H: crate::RWRegister<u32>,
    #[doc = "Message Buffer 84 CS Register"]
    pub CS84: crate::RWRegister<u32>,
    #[doc = "Message Buffer 84 ID Register"]
    pub ID84: crate::RWRegister<u32>,
    #[doc = "Message Buffer 13 WORD_16B Register"]
    pub MB13_16B_WORD2_H: crate::RWRegister<u32>,
    #[doc = "Message Buffer 13 WORD_16B Register"]
    pub MB13_16B_WORD3_H: crate::RWRegister<u32>,
    #[doc = "Message Buffer 85 CS Register"]
    pub CS85: crate::RWRegister<u32>,
    #[doc = "Message Buffer 85 ID Register"]
    pub ID85: crate::RWRegister<u32>,
    #[doc = "Message Buffer 14 WORD_16B Register"]
    pub MB14_16B_WORD0_H: crate::RWRegister<u32>,
    #[doc = "Message Buffer 14 WORD_16B Register"]
    pub MB14_16B_WORD1_H: crate::RWRegister<u32>,
    #[doc = "Message Buffer 86 CS Register"]
    pub CS86: crate::RWRegister<u32>,
    #[doc = "Message Buffer 86 ID Register"]
    pub ID86: crate::RWRegister<u32>,
    #[doc = "Message Buffer 15 CS Register"]
    pub MB15_16B_CS_H: crate::RWRegister<u32>,
    #[doc = "Message Buffer 15 ID Register"]
    pub MB15_16B_ID_H: crate::RWRegister<u32>,
    #[doc = "Message Buffer 87 CS Register"]
    pub CS87: crate::RWRegister<u32>,
    #[doc = "Message Buffer 87 ID Register"]
    pub ID87: crate::RWRegister<u32>,
    #[doc = "Message Buffer 15 WORD_16B Register"]
    pub MB15_16B_WORD2_H: crate::RWRegister<u32>,
    #[doc = "Message Buffer 15 WORD_16B Register"]
    pub MB15_16B_WORD3_H: crate::RWRegister<u32>,
    #[doc = "Message Buffer 88 CS Register"]
    pub CS88: crate::RWRegister<u32>,
    #[doc = "Message Buffer 88 ID Register"]
    pub ID88: crate::RWRegister<u32>,
    #[doc = "Message Buffer 16 WORD_16B Register"]
    pub MB16_16B_WORD0_H: crate::RWRegister<u32>,
    #[doc = "Message Buffer 16 WORD_16B Register"]
    pub MB16_16B_WORD1_H: crate::RWRegister<u32>,
    #[doc = "Message Buffer 89 CS Register"]
    pub CS89: crate::RWRegister<u32>,
    #[doc = "Message Buffer 89 ID Register"]
    pub ID89: crate::RWRegister<u32>,
    #[doc = "Message Buffer 10 WORD_32B Register"]
    pub MB10_32B_WORD0_H: crate::RWRegister<u32>,
    #[doc = "Message Buffer 10 WORD_32B Register"]
    pub MB10_32B_WORD1_H: crate::RWRegister<u32>,
    #[doc = "Message Buffer 90 CS Register"]
    pub CS90: crate::RWRegister<u32>,
    #[doc = "Message Buffer 90 ID Register"]
    pub ID90: crate::RWRegister<u32>,
    #[doc = "Message Buffer 10 WORD_32B Register"]
    pub MB10_32B_WORD4_H: crate::RWRegister<u32>,
    #[doc = "Message Buffer 10 WORD_32B Register"]
    pub MB10_32B_WORD5_H: crate::RWRegister<u32>,
    #[doc = "Message Buffer 91 CS Register"]
    pub CS91: crate::RWRegister<u32>,
    #[doc = "Message Buffer 91 ID Register"]
    pub ID91: crate::RWRegister<u32>,
    #[doc = "Message Buffer 11 CS Register"]
    pub MB11_32B_CS_H: crate::RWRegister<u32>,
    #[doc = "Message Buffer 11 ID Register"]
    pub MB11_32B_ID_H: crate::RWRegister<u32>,
    #[doc = "Message Buffer 92 CS Register"]
    pub CS92: crate::RWRegister<u32>,
    #[doc = "Message Buffer 92 ID Register"]
    pub ID92: crate::RWRegister<u32>,
    #[doc = "Message Buffer 11 WORD_32B Register"]
    pub MB11_32B_WORD2_H: crate::RWRegister<u32>,
    #[doc = "Message Buffer 11 WORD_32B Register"]
    pub MB11_32B_WORD3_H: crate::RWRegister<u32>,
    #[doc = "Message Buffer 93 CS Register"]
    pub CS93: crate::RWRegister<u32>,
    #[doc = "Message Buffer 93 ID Register"]
    pub ID93: crate::RWRegister<u32>,
    #[doc = "Message Buffer 11 WORD_32B Register"]
    pub MB11_32B_WORD6_H: crate::RWRegister<u32>,
    #[doc = "Message Buffer 11 WORD_32B Register"]
    pub MB11_32B_WORD7_H: crate::RWRegister<u32>,
    #[doc = "Message Buffer 94 CS Register"]
    pub CS94: crate::RWRegister<u32>,
    #[doc = "Message Buffer 94 ID Register"]
    pub ID94: crate::RWRegister<u32>,
    #[doc = "Message Buffer 20 WORD_16B Register"]
    pub MB20_16B_WORD0_H: crate::RWRegister<u32>,
    #[doc = "Message Buffer 20 WORD_16B Register"]
    pub MB20_16B_WORD1_H: crate::RWRegister<u32>,
    #[doc = "Message Buffer 95 CS Register"]
    pub CS95: crate::RWRegister<u32>,
    #[doc = "Message Buffer 95 ID Register"]
    pub ID95: crate::RWRegister<u32>,
    #[doc = "Message Buffer 95 WORD_8B Register"]
    pub MB95_8B_WORD0: crate::RWRegister<u32>,
    #[doc = "Message Buffer 95 WORD_8B Register"]
    pub MB95_8B_WORD1: crate::RWRegister<u32>,
    _reserved5: [u8; 0x0200],
    #[doc = "Receive Individual Mask"]
    pub RXIMR: [crate::RWRegister<u32>; 96usize],
    _reserved6: [u8; 0xe0],
    #[doc = "Memory Error Control"]
    pub MECR: crate::RWRegister<u32>,
    #[doc = "Error Injection Address"]
    pub ERRIAR: crate::RWRegister<u32>,
    #[doc = "Error Injection Data Pattern"]
    pub ERRIDPR: crate::RWRegister<u32>,
    #[doc = "Error Injection Parity Pattern"]
    pub ERRIPPR: crate::RWRegister<u32>,
    #[doc = "Error Report Address"]
    pub RERRAR: crate::RORegister<u32>,
    #[doc = "Error Report Data"]
    pub RERRDR: crate::RORegister<u32>,
    #[doc = "Error Report Syndrome"]
    pub RERRSYNR: crate::RORegister<u32>,
    #[doc = "Error Status"]
    pub ERRSR: crate::RWRegister<u32>,
    _reserved7: [u8; 0xf0],
    #[doc = "Enhanced CAN Bit Timing Prescalers"]
    pub EPRS: crate::RWRegister<u32>,
    #[doc = "Enhanced Nominal CAN Bit Timing"]
    pub ENCBT: crate::RWRegister<u32>,
    #[doc = "Enhanced Data Phase CAN Bit Timing"]
    pub EDCBT: crate::RWRegister<u32>,
    #[doc = "Enhanced Transceiver Delay Compensation"]
    pub ETDC: crate::RWRegister<u32>,
    #[doc = "CAN FD Control"]
    pub FDCTRL: crate::RWRegister<u32>,
    #[doc = "CAN FD Bit Timing"]
    pub FDCBT: crate::RWRegister<u32>,
    #[doc = "CAN FD CRC"]
    pub FDCRC: crate::RORegister<u32>,
    #[doc = "Enhanced RX FIFO Control"]
    pub ERFCR: crate::RWRegister<u32>,
    #[doc = "Enhanced RX FIFO Interrupt Enable"]
    pub ERFIER: crate::RWRegister<u32>,
    #[doc = "Enhanced RX FIFO Status"]
    pub ERFSR: crate::RWRegister<u32>,
    _reserved8: [u8; 0x18],
    #[doc = "High-Resolution Timestamp"]
    pub HR_TIME_STAMP: [crate::RWRegister<u32>; 96usize],
    _reserved9: [u8; 0x2250],
    #[doc = "Enhanced RX FIFO Filter Element"]
    pub ERFFEL: [crate::RWRegister<u32>; 128usize],
}
#[doc = "Module Configuration"]
pub mod MCR {
    #[doc = "Number Of The Last Message Buffer"]
    pub mod MAXMB {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x7f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ID Acceptance Mode"]
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
    #[doc = "CAN FD operation enable"]
    pub mod FDEN {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "CAN FD is disabled. FlexCAN is able to receive and transmit messages in CAN 2.0 format."]
            pub const CAN_FD_DISABLED: u32 = 0;
            #[doc = "CAN FD is enabled. FlexCAN is able to receive and transmit messages in both CAN FD and CAN 2.0 formats."]
            pub const CAN_FD_ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Abort Enable"]
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
    #[doc = "Local Priority Enable"]
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
    #[doc = "DMA Enable"]
    pub mod DMA {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "DMA feature for RX FIFO disabled."]
            pub const ID2: u32 = 0;
            #[doc = "DMA feature for RX FIFO enabled."]
            pub const ID4: u32 = 0x01;
        }
    }
    #[doc = "Individual Rx Masking And Queue Enable"]
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
    #[doc = "Self Reception Disable"]
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
    #[doc = "Doze Mode Enable"]
    pub mod DOZE {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "FlexCAN is not enabled to enter low-power mode when Doze mode is requested."]
            pub const LOW_POWER_DOZE_DISABLED: u32 = 0;
            #[doc = "FlexCAN is enabled to enter low-power mode when Doze mode is requested."]
            pub const LOW_POWER_DOZE_ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Wake Up Source"]
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
    #[doc = "Low-Power Mode Acknowledge"]
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
    #[doc = "Warning Interrupt Enable"]
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
    #[doc = "Self Wake Up"]
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
    #[doc = "Supervisor Mode"]
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
    #[doc = "Freeze Mode Acknowledge"]
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
    #[doc = "Soft Reset"]
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
    #[doc = "Wake Up Interrupt Mask"]
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
    #[doc = "FlexCAN Not Ready"]
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
    #[doc = "Halt FlexCAN"]
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
    #[doc = "Rx FIFO Enable"]
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
    #[doc = "Freeze Enable"]
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
    #[doc = "Module Disable"]
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
#[doc = "Control 1"]
pub mod CTRL1 {
    #[doc = "Propagation Segment"]
    pub mod PROPSEG {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Listen-Only Mode"]
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
    #[doc = "Lowest Buffer Transmitted First"]
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
    #[doc = "Timer Sync"]
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
    #[doc = "Bus Off Recovery"]
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
    #[doc = "CAN Bit Sampling"]
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
    #[doc = "Rx Warning Interrupt Mask"]
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
    #[doc = "Tx Warning Interrupt Mask"]
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
    #[doc = "Loop Back Mode"]
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
    #[doc = "CAN Engine Clock Source"]
    pub mod CLKSRC {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The CAN engine clock source is the oscillator clock. Under this condition, the oscillator clock frequency must be lower than the bus clock."]
            pub const OSCILLATOR_CLOCK: u32 = 0;
            #[doc = "The CAN engine clock source is the peripheral clock."]
            pub const PERIPHERAL_CLOCK: u32 = 0x01;
        }
    }
    #[doc = "Error Interrupt Mask"]
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
    #[doc = "Bus Off Interrupt Mask"]
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
    #[doc = "Phase Segment 2"]
    pub mod PSEG2 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Phase Segment 1"]
    pub mod PSEG1 {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Resync Jump Width"]
    pub mod RJW {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Prescaler Division Factor"]
    pub mod PRESDIV {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Free-Running Timer"]
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
#[doc = "RX Message Buffers Global Mask"]
pub mod RXMGMASK {
    #[doc = "Rx Mailboxes Global Mask Bits"]
    pub mod MG {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Receive 14 Mask"]
pub mod RX14MASK {
    #[doc = "Rx Buffer 14 Mask Bits"]
    pub mod RX14M {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Receive 15 Mask"]
pub mod RX15MASK {
    #[doc = "Rx Buffer 15 Mask Bits"]
    pub mod RX15M {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Error Counter"]
pub mod ECR {
    #[doc = "Transmit Error Counter"]
    pub mod TXERRCNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Receive Error Counter"]
    pub mod RXERRCNT {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Transmit Error Counter for fast bits"]
    pub mod TXERRCNT_FAST {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Receive Error Counter for fast bits"]
    pub mod RXERRCNT_FAST {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Error and Status 1"]
pub mod ESR1 {
    #[doc = "Wake-Up Interrupt"]
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
    #[doc = "Error Interrupt"]
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
    #[doc = "Bus Off Interrupt"]
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
    #[doc = "FlexCAN In Reception"]
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
    #[doc = "Fault Confinement State"]
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
    #[doc = "FlexCAN In Transmission"]
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
    #[doc = "IDLE"]
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
    #[doc = "Rx Error Warning"]
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
    #[doc = "TX Error Warning"]
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
    #[doc = "Stuffing Error"]
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
    #[doc = "Form Error"]
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
    #[doc = "Cyclic Redundancy Check Error"]
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
    #[doc = "Acknowledge Error"]
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
    #[doc = "Bit0 Error"]
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
    #[doc = "Bit1 Error"]
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
    #[doc = "Rx Warning Interrupt Flag"]
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
    #[doc = "Tx Warning Interrupt Flag"]
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
    #[doc = "CAN Synchronization Status"]
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
    #[doc = "Bus Off Done Interrupt"]
    pub mod BOFFDONEINT {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No such occurrence."]
            pub const BUS_OFF_NOT_DONE: u32 = 0;
            #[doc = "FlexCAN module has completed Bus Off process."]
            pub const BUS_OFF_DONE: u32 = 0x01;
        }
    }
    #[doc = "Error interrupt for errors detected in Data Phase of CAN FD frames with BRS bit set"]
    pub mod ERRINT_FAST {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No such occurrence."]
            pub const ERRORS_DATA_PHASE_NO: u32 = 0;
            #[doc = "Indicates setting of any error bit detected in the data phase of CAN FD frames with the BRS bit set."]
            pub const ERRORS_DATA_PHASE_YES: u32 = 0x01;
        }
    }
    #[doc = "Error Overrun"]
    pub mod ERROVR {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Overrun has not occurred."]
            pub const OVERRUN_NOT_OCCURRED: u32 = 0;
            #[doc = "Overrun has occurred."]
            pub const OVERRUN_OCCURRED: u32 = 0x01;
        }
    }
    #[doc = "Stuffing Error in the Data Phase of CAN FD frames with the BRS bit set"]
    pub mod STFERR_FAST {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No such occurrence."]
            pub const STUFFING_ERROR_NO: u32 = 0;
            #[doc = "A stuffing error occurred since last read of this register."]
            pub const STUFFING_ERROR_YES: u32 = 0x01;
        }
    }
    #[doc = "Form Error in the Data Phase of CAN FD frames with the BRS bit set"]
    pub mod FRMERR_FAST {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No such occurrence."]
            pub const FORM_ERROR_NO: u32 = 0;
            #[doc = "A form error occurred since last read of this register."]
            pub const FORM_ERROR_YES: u32 = 0x01;
        }
    }
    #[doc = "Cyclic Redundancy Check Error in the CRC field of CAN FD frames with the BRS bit set"]
    pub mod CRCERR_FAST {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No such occurrence."]
            pub const CRC_ERROR_NO: u32 = 0;
            #[doc = "A CRC error occurred since last read of this register."]
            pub const CRC_ERROR_YES: u32 = 0x01;
        }
    }
    #[doc = "Bit0 Error in the Data Phase of CAN FD frames with the BRS bit set"]
    pub mod BIT0ERR_FAST {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No such occurrence."]
            pub const BIT0_ERROR_NO: u32 = 0;
            #[doc = "At least one bit sent as dominant is received as recessive."]
            pub const BIT0_ERROR_YES: u32 = 0x01;
        }
    }
    #[doc = "Bit1 Error in the Data Phase of CAN FD frames with the BRS bit set"]
    pub mod BIT1ERR_FAST {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No such occurrence."]
            pub const BIT1_ERROR_NO: u32 = 0;
            #[doc = "At least one bit sent as recessive is received as dominant."]
            pub const BIT1_ERROR_YES: u32 = 0x01;
        }
    }
}
#[doc = "Interrupt Masks 2"]
pub mod IMASK2 {
    #[doc = "Buffer MBi Mask"]
    pub mod BUF63TO32M {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Interrupt Masks 1"]
pub mod IMASK1 {
    #[doc = "Buffer MBi Mask"]
    pub mod BUF31TO0M {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Interrupt Flags 2"]
pub mod IFLAG2 {
    #[doc = "Buffer MBi Interrupt"]
    pub mod BUF63TO32I {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Interrupt Flags 1"]
pub mod IFLAG1 {
    #[doc = "Buffer MB0 Interrupt Or Clear FIFO bit"]
    pub mod BUF0I {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The corresponding buffer has no occurrence of successfully completed transmission or reception when MCR\\[RFEN\\]=0."]
            pub const BUFFER_TX_RX_NOT_COMPLETE: u32 = 0;
            #[doc = "The corresponding buffer has successfully completed transmission or reception when MCR\\[RFEN\\]=0."]
            pub const BUFFER_TX_RX_COMPLETE: u32 = 0x01;
        }
    }
    #[doc = "Buffer MBi Interrupt Or Reserved"]
    pub mod BUF4TO1I {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Buffer MB5 Interrupt Or Frames available in Rx FIFO"]
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
    #[doc = "Buffer MB6 Interrupt Or Rx FIFO Warning"]
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
    #[doc = "Buffer MB7 Interrupt Or Rx FIFO Overflow"]
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
    #[doc = "Buffer MBi Interrupt"]
    pub mod BUF31TO8I {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Control 2"]
pub mod CTRL2 {
    #[doc = "Timestamp Capture Point"]
    pub mod TSTAMPCAP {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const TIME_STAMP_CAPTURE_DISABLED: u32 = 0;
            #[doc = "End of the CAN frame"]
            pub const TIME_STAMP_CAPTURE_FRAME_END_ENABLED: u32 = 0x01;
            #[doc = "Start of the CAN frame"]
            pub const TIME_STAMP_CAPTURE_FRAME_START_ENABLED: u32 = 0x02;
            #[doc = "Start of frame for classical CAN frames; res bit for CAN FD frames"]
            pub const TIME_STAMP_CAPTURE_2_TYPES_ENABLED: u32 = 0x03;
        }
    }
    #[doc = "Message Buffer Timestamp Base"]
    pub mod MBTSBASE {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "TIMER"]
            pub const BASE_TIMER: u32 = 0;
            #[doc = "Lower 16 bits of high-resolution timer"]
            pub const BASE_LOWER_16: u32 = 0x01;
            #[doc = "Upper 16 bits of high-resolution timer"]
            pub const BASE_UPPER_16: u32 = 0x02;
        }
    }
    #[doc = "Edge Filter Disable"]
    pub mod EDFLTDIS {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Edge filter is enabled"]
            pub const ENABLE: u32 = 0;
            #[doc = "Edge filter is disabled"]
            pub const DISABLE: u32 = 0x01;
        }
    }
    #[doc = "ISO CAN FD Enable"]
    pub mod ISOCANFDEN {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "FlexCAN operates using the non-ISO CAN FD protocol."]
            pub const NON_ISO: u32 = 0;
            #[doc = "FlexCAN operates using the ISO CAN FD protocol (ISO 11898-1)."]
            pub const ISO: u32 = 0x01;
        }
    }
    #[doc = "Bit Timing Expansion Enable"]
    pub mod BTE {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Protocol Exception Enable"]
    pub mod PREXCEN {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Protocol exception is disabled."]
            pub const DISABLE: u32 = 0;
            #[doc = "Protocol exception is enabled."]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Timer Source"]
    pub mod TIMER_SRC {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The free running timer is clocked by the CAN bit clock, which defines the baud rate on the CAN bus."]
            pub const CAN_BIT_CLOCK: u32 = 0;
            #[doc = "The free running timer is clocked by an external time tick. The period can be either adjusted to be equal to the baud rate on the CAN bus, or a different value as required. See the device-specific section for details about the external time tick."]
            pub const EXTERNAL_CLOCK: u32 = 0x01;
        }
    }
    #[doc = "Entire Frame Arbitration Field Comparison Enable for RX Message Buffers"]
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
    #[doc = "Remote Request Storing"]
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
    #[doc = "Message Buffers Reception Priority"]
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
    #[doc = "Transmission Arbitration Start Delay"]
    pub mod TASD {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Number of Legacy Receive FIFO Filters"]
    pub mod RFFN {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Write Access to Memory in Freeze Mode"]
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
    #[doc = "Error Correction Configuration Register Write Enable"]
    pub mod ECRWRE {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable update."]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable update."]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Bus Off Done Interrupt Mask"]
    pub mod BOFFDONEMSK {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Bus off done interrupt disabled."]
            pub const DISABLE: u32 = 0;
            #[doc = "Bus off done interrupt enabled."]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Error Interrupt Mask for Errors Detected in the Data Phase of Fast CAN FD Frames"]
    pub mod ERRMSK_FAST {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ERRINT_FAST error interrupt disabled."]
            pub const DISABLE: u32 = 0;
            #[doc = "ERRINT_FAST error interrupt enabled."]
            pub const ENABLE: u32 = 0x01;
        }
    }
}
#[doc = "Error and Status 2"]
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
#[doc = "Cyclic Redundancy Check"]
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
#[doc = "Legacy RX FIFO Global Mask"]
pub mod RXFGMASK {
    #[doc = "Rx FIFO Global Mask Bits"]
    pub mod FGM {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Legacy RX FIFO Information"]
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
#[doc = "CAN Bit Timing"]
pub mod CBT {
    #[doc = "Extended Phase Segment 2"]
    pub mod EPSEG2 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Extended Phase Segment 1"]
    pub mod EPSEG1 {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Extended Propagation Segment"]
    pub mod EPROPSEG {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Extended Resync Jump Width"]
    pub mod ERJW {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Extended Prescaler Division Factor"]
    pub mod EPRESDIV {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x03ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Bit Timing Format Enable"]
    pub mod BTF {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Extended bit time definitions disabled."]
            pub const DISABLE: u32 = 0;
            #[doc = "Extended bit time definitions enabled."]
            pub const ENABLE: u32 = 0x01;
        }
    }
}
#[doc = "Interrupt Masks 3"]
pub mod IMASK3 {
    #[doc = "Buffer MBi Mask"]
    pub mod BUF95TO64M {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Interrupt Flags 3"]
pub mod IFLAG3 {
    #[doc = "Buffer MBi Interrupt"]
    pub mod BUF95TO64 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 0 CS Register"]
pub mod CS0 {
    #[doc = "Free-Running Counter Time stamp. This 16-bit field is a copy of the Free-Running Timer, captured for Tx and Rx frames at the time when the beginning of the Identifier field appears on the CAN bus."]
    pub mod TIME_STAMP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Length of the data to be stored/transmitted."]
    pub mod DLC {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Remote Transmission Request. One/zero for remote/data frame."]
    pub mod RTR {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ID Extended. One/zero for extended/standard format frame."]
    pub mod IDE {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Substitute Remote Request. Contains a fixed recessive bit."]
    pub mod SRR {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Message Buffer Code. This 4-bit field can be accessed (read or write) by the CPU and by the FlexCAN module itself, as part of the message buffer matching and arbitration process."]
    pub mod CODE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Error State Indicator. This bit indicates if the transmitting node is error active or error passive."]
    pub mod ESI {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Bit Rate Switch. This bit defines whether the bit rate is switched inside a CAN FD format frame."]
    pub mod BRS {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Extended Data Length. This bit distinguishes between CAN format and CAN FD format frames. The EDL bit must not be set for Message Buffers configured to RANSWER with code field 0b1010."]
    pub mod EDL {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 0 ID Register"]
pub mod ID0 {
    #[doc = "Contains extended (LOW word) identifier of message buffer."]
    pub mod EXT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0003_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Contains standard/extended (HIGH word) identifier of message buffer."]
    pub mod STD {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Local priority. This 3-bit fieldis only used when LPRIO_EN bit is set in MCR and it only makes sense for Tx buffers. These bits are not transmitted. They are appended to the regular ID to define the transmission priority."]
    pub mod PRIO {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 0 WORD_16B Register"]
pub mod MB0_16B_WORD0_L {
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_3 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_2 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_1 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_0 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 0 WORD_16B Register"]
pub mod MB0_16B_WORD1_L {
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_7 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_6 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_5 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_4 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 1 CS Register"]
pub mod CS1 {
    #[doc = "Free-Running Counter Time stamp. This 16-bit field is a copy of the Free-Running Timer, captured for Tx and Rx frames at the time when the beginning of the Identifier field appears on the CAN bus."]
    pub mod TIME_STAMP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Length of the data to be stored/transmitted."]
    pub mod DLC {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Remote Transmission Request. One/zero for remote/data frame."]
    pub mod RTR {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ID Extended. One/zero for extended/standard format frame."]
    pub mod IDE {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Substitute Remote Request. Contains a fixed recessive bit."]
    pub mod SRR {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Message Buffer Code. This 4-bit field can be accessed (read or write) by the CPU and by the FlexCAN module itself, as part of the message buffer matching and arbitration process."]
    pub mod CODE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Error State Indicator. This bit indicates if the transmitting node is error active or error passive."]
    pub mod ESI {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Bit Rate Switch. This bit defines whether the bit rate is switched inside a CAN FD format frame."]
    pub mod BRS {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Extended Data Length. This bit distinguishes between CAN format and CAN FD format frames. The EDL bit must not be set for Message Buffers configured to RANSWER with code field 0b1010."]
    pub mod EDL {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 1 ID Register"]
pub mod ID1 {
    #[doc = "Contains extended (LOW word) identifier of message buffer."]
    pub mod EXT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0003_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Contains standard/extended (HIGH word) identifier of message buffer."]
    pub mod STD {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Local priority. This 3-bit fieldis only used when LPRIO_EN bit is set in MCR and it only makes sense for Tx buffers. These bits are not transmitted. They are appended to the regular ID to define the transmission priority."]
    pub mod PRIO {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 0 WORD_32B Register"]
pub mod MB0_32B_WORD4_L {
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_19 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_18 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_17 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_16 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 0 WORD_32B Register"]
pub mod MB0_32B_WORD5_L {
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_23 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_22 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_21 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_20 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 2 CS Register"]
pub mod CS2 {
    #[doc = "Free-Running Counter Time stamp. This 16-bit field is a copy of the Free-Running Timer, captured for Tx and Rx frames at the time when the beginning of the Identifier field appears on the CAN bus."]
    pub mod TIME_STAMP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Length of the data to be stored/transmitted."]
    pub mod DLC {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Remote Transmission Request. One/zero for remote/data frame."]
    pub mod RTR {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ID Extended. One/zero for extended/standard format frame."]
    pub mod IDE {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Substitute Remote Request. Contains a fixed recessive bit."]
    pub mod SRR {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Message Buffer Code. This 4-bit field can be accessed (read or write) by the CPU and by the FlexCAN module itself, as part of the message buffer matching and arbitration process."]
    pub mod CODE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Error State Indicator. This bit indicates if the transmitting node is error active or error passive."]
    pub mod ESI {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Bit Rate Switch. This bit defines whether the bit rate is switched inside a CAN FD format frame."]
    pub mod BRS {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Extended Data Length. This bit distinguishes between CAN format and CAN FD format frames. The EDL bit must not be set for Message Buffers configured to RANSWER with code field 0b1010."]
    pub mod EDL {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 2 ID Register"]
pub mod ID2 {
    #[doc = "Contains extended (LOW word) identifier of message buffer."]
    pub mod EXT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0003_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Contains standard/extended (HIGH word) identifier of message buffer."]
    pub mod STD {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Local priority. This 3-bit fieldis only used when LPRIO_EN bit is set in MCR and it only makes sense for Tx buffers. These bits are not transmitted. They are appended to the regular ID to define the transmission priority."]
    pub mod PRIO {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 0 WORD_64B Register"]
pub mod MB0_64B_WORD8_L {
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_35 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_34 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_33 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_32 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 0 WORD_64B Register"]
pub mod MB0_64B_WORD9_L {
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_39 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_38 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_37 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_36 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 3 CS Register"]
pub mod CS3 {
    #[doc = "Free-Running Counter Time stamp. This 16-bit field is a copy of the Free-Running Timer, captured for Tx and Rx frames at the time when the beginning of the Identifier field appears on the CAN bus."]
    pub mod TIME_STAMP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Length of the data to be stored/transmitted."]
    pub mod DLC {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Remote Transmission Request. One/zero for remote/data frame."]
    pub mod RTR {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ID Extended. One/zero for extended/standard format frame."]
    pub mod IDE {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Substitute Remote Request. Contains a fixed recessive bit."]
    pub mod SRR {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Message Buffer Code. This 4-bit field can be accessed (read or write) by the CPU and by the FlexCAN module itself, as part of the message buffer matching and arbitration process."]
    pub mod CODE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Error State Indicator. This bit indicates if the transmitting node is error active or error passive."]
    pub mod ESI {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Bit Rate Switch. This bit defines whether the bit rate is switched inside a CAN FD format frame."]
    pub mod BRS {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Extended Data Length. This bit distinguishes between CAN format and CAN FD format frames. The EDL bit must not be set for Message Buffers configured to RANSWER with code field 0b1010."]
    pub mod EDL {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 3 ID Register"]
pub mod ID3 {
    #[doc = "Contains extended (LOW word) identifier of message buffer."]
    pub mod EXT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0003_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Contains standard/extended (HIGH word) identifier of message buffer."]
    pub mod STD {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Local priority. This 3-bit fieldis only used when LPRIO_EN bit is set in MCR and it only makes sense for Tx buffers. These bits are not transmitted. They are appended to the regular ID to define the transmission priority."]
    pub mod PRIO {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 0 WORD_64B Register"]
pub mod MB0_64B_WORD12_L {
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_51 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_50 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_49 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_48 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 0 WORD_64B Register"]
pub mod MB0_64B_WORD13_L {
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_55 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_54 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_53 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_52 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 4 CS Register"]
pub mod CS4 {
    #[doc = "Free-Running Counter Time stamp. This 16-bit field is a copy of the Free-Running Timer, captured for Tx and Rx frames at the time when the beginning of the Identifier field appears on the CAN bus."]
    pub mod TIME_STAMP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Length of the data to be stored/transmitted."]
    pub mod DLC {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Remote Transmission Request. One/zero for remote/data frame."]
    pub mod RTR {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ID Extended. One/zero for extended/standard format frame."]
    pub mod IDE {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Substitute Remote Request. Contains a fixed recessive bit."]
    pub mod SRR {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Message Buffer Code. This 4-bit field can be accessed (read or write) by the CPU and by the FlexCAN module itself, as part of the message buffer matching and arbitration process."]
    pub mod CODE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Error State Indicator. This bit indicates if the transmitting node is error active or error passive."]
    pub mod ESI {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Bit Rate Switch. This bit defines whether the bit rate is switched inside a CAN FD format frame."]
    pub mod BRS {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Extended Data Length. This bit distinguishes between CAN format and CAN FD format frames. The EDL bit must not be set for Message Buffers configured to RANSWER with code field 0b1010."]
    pub mod EDL {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 4 ID Register"]
pub mod ID4 {
    #[doc = "Contains extended (LOW word) identifier of message buffer."]
    pub mod EXT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0003_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Contains standard/extended (HIGH word) identifier of message buffer."]
    pub mod STD {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Local priority. This 3-bit fieldis only used when LPRIO_EN bit is set in MCR and it only makes sense for Tx buffers. These bits are not transmitted. They are appended to the regular ID to define the transmission priority."]
    pub mod PRIO {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 1 WORD_32B Register"]
pub mod MB1_32B_WORD6_L {
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_27 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_26 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_25 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_24 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 1 WORD_32B Register"]
pub mod MB1_32B_WORD7_L {
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_31 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_30 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_29 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_28 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 5 CS Register"]
pub mod CS5 {
    #[doc = "Free-Running Counter Time stamp. This 16-bit field is a copy of the Free-Running Timer, captured for Tx and Rx frames at the time when the beginning of the Identifier field appears on the CAN bus."]
    pub mod TIME_STAMP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Length of the data to be stored/transmitted."]
    pub mod DLC {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Remote Transmission Request. One/zero for remote/data frame."]
    pub mod RTR {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ID Extended. One/zero for extended/standard format frame."]
    pub mod IDE {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Substitute Remote Request. Contains a fixed recessive bit."]
    pub mod SRR {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Message Buffer Code. This 4-bit field can be accessed (read or write) by the CPU and by the FlexCAN module itself, as part of the message buffer matching and arbitration process."]
    pub mod CODE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Error State Indicator. This bit indicates if the transmitting node is error active or error passive."]
    pub mod ESI {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Bit Rate Switch. This bit defines whether the bit rate is switched inside a CAN FD format frame."]
    pub mod BRS {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Extended Data Length. This bit distinguishes between CAN format and CAN FD format frames. The EDL bit must not be set for Message Buffers configured to RANSWER with code field 0b1010."]
    pub mod EDL {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 5 ID Register"]
pub mod ID5 {
    #[doc = "Contains extended (LOW word) identifier of message buffer."]
    pub mod EXT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0003_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Contains standard/extended (HIGH word) identifier of message buffer."]
    pub mod STD {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Local priority. This 3-bit fieldis only used when LPRIO_EN bit is set in MCR and it only makes sense for Tx buffers. These bits are not transmitted. They are appended to the regular ID to define the transmission priority."]
    pub mod PRIO {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 1 WORD_64B Register"]
pub mod MB1_64B_WORD2_L {
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_11 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_10 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_9 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_8 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 1 WORD_64B Register"]
pub mod MB1_64B_WORD3_L {
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_15 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_14 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_13 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_12 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 6 CS Register"]
pub mod CS6 {
    #[doc = "Free-Running Counter Time stamp. This 16-bit field is a copy of the Free-Running Timer, captured for Tx and Rx frames at the time when the beginning of the Identifier field appears on the CAN bus."]
    pub mod TIME_STAMP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Length of the data to be stored/transmitted."]
    pub mod DLC {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Remote Transmission Request. One/zero for remote/data frame."]
    pub mod RTR {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ID Extended. One/zero for extended/standard format frame."]
    pub mod IDE {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Substitute Remote Request. Contains a fixed recessive bit."]
    pub mod SRR {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Message Buffer Code. This 4-bit field can be accessed (read or write) by the CPU and by the FlexCAN module itself, as part of the message buffer matching and arbitration process."]
    pub mod CODE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Error State Indicator. This bit indicates if the transmitting node is error active or error passive."]
    pub mod ESI {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Bit Rate Switch. This bit defines whether the bit rate is switched inside a CAN FD format frame."]
    pub mod BRS {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Extended Data Length. This bit distinguishes between CAN format and CAN FD format frames. The EDL bit must not be set for Message Buffers configured to RANSWER with code field 0b1010."]
    pub mod EDL {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 6 ID Register"]
pub mod ID6 {
    #[doc = "Contains extended (LOW word) identifier of message buffer."]
    pub mod EXT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0003_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Contains standard/extended (HIGH word) identifier of message buffer."]
    pub mod STD {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Local priority. This 3-bit fieldis only used when LPRIO_EN bit is set in MCR and it only makes sense for Tx buffers. These bits are not transmitted. They are appended to the regular ID to define the transmission priority."]
    pub mod PRIO {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 1 WORD_64B Register"]
pub mod MB1_64B_WORD6_L {
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_27 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_26 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_25 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_24 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 1 WORD_64B Register"]
pub mod MB1_64B_WORD7_L {
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_31 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_30 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_29 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_28 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 7 CS Register"]
pub mod CS7 {
    #[doc = "Free-Running Counter Time stamp. This 16-bit field is a copy of the Free-Running Timer, captured for Tx and Rx frames at the time when the beginning of the Identifier field appears on the CAN bus."]
    pub mod TIME_STAMP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Length of the data to be stored/transmitted."]
    pub mod DLC {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Remote Transmission Request. One/zero for remote/data frame."]
    pub mod RTR {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ID Extended. One/zero for extended/standard format frame."]
    pub mod IDE {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Substitute Remote Request. Contains a fixed recessive bit."]
    pub mod SRR {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Message Buffer Code. This 4-bit field can be accessed (read or write) by the CPU and by the FlexCAN module itself, as part of the message buffer matching and arbitration process."]
    pub mod CODE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Error State Indicator. This bit indicates if the transmitting node is error active or error passive."]
    pub mod ESI {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Bit Rate Switch. This bit defines whether the bit rate is switched inside a CAN FD format frame."]
    pub mod BRS {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Extended Data Length. This bit distinguishes between CAN format and CAN FD format frames. The EDL bit must not be set for Message Buffers configured to RANSWER with code field 0b1010."]
    pub mod EDL {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 7 ID Register"]
pub mod ID7 {
    #[doc = "Contains extended (LOW word) identifier of message buffer."]
    pub mod EXT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0003_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Contains standard/extended (HIGH word) identifier of message buffer."]
    pub mod STD {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Local priority. This 3-bit fieldis only used when LPRIO_EN bit is set in MCR and it only makes sense for Tx buffers. These bits are not transmitted. They are appended to the regular ID to define the transmission priority."]
    pub mod PRIO {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 1 WORD_64B Register"]
pub mod MB1_64B_WORD10_L {
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_43 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_42 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_41 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_40 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 1 WORD_64B Register"]
pub mod MB1_64B_WORD11_L {
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_47 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_46 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_45 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_44 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 8 CS Register"]
pub mod CS8 {
    #[doc = "Free-Running Counter Time stamp. This 16-bit field is a copy of the Free-Running Timer, captured for Tx and Rx frames at the time when the beginning of the Identifier field appears on the CAN bus."]
    pub mod TIME_STAMP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Length of the data to be stored/transmitted."]
    pub mod DLC {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Remote Transmission Request. One/zero for remote/data frame."]
    pub mod RTR {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ID Extended. One/zero for extended/standard format frame."]
    pub mod IDE {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Substitute Remote Request. Contains a fixed recessive bit."]
    pub mod SRR {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Message Buffer Code. This 4-bit field can be accessed (read or write) by the CPU and by the FlexCAN module itself, as part of the message buffer matching and arbitration process."]
    pub mod CODE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Error State Indicator. This bit indicates if the transmitting node is error active or error passive."]
    pub mod ESI {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Bit Rate Switch. This bit defines whether the bit rate is switched inside a CAN FD format frame."]
    pub mod BRS {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Extended Data Length. This bit distinguishes between CAN format and CAN FD format frames. The EDL bit must not be set for Message Buffers configured to RANSWER with code field 0b1010."]
    pub mod EDL {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 8 ID Register"]
pub mod ID8 {
    #[doc = "Contains extended (LOW word) identifier of message buffer."]
    pub mod EXT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0003_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Contains standard/extended (HIGH word) identifier of message buffer."]
    pub mod STD {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Local priority. This 3-bit fieldis only used when LPRIO_EN bit is set in MCR and it only makes sense for Tx buffers. These bits are not transmitted. They are appended to the regular ID to define the transmission priority."]
    pub mod PRIO {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 1 WORD_64B Register"]
pub mod MB1_64B_WORD14_L {
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_59 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_58 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_57 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_56 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 1 WORD_64B Register"]
pub mod MB1_64B_WORD15_L {
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_63 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_62 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_61 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_60 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 9 CS Register"]
pub mod CS9 {
    #[doc = "Free-Running Counter Time stamp. This 16-bit field is a copy of the Free-Running Timer, captured for Tx and Rx frames at the time when the beginning of the Identifier field appears on the CAN bus."]
    pub mod TIME_STAMP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Length of the data to be stored/transmitted."]
    pub mod DLC {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Remote Transmission Request. One/zero for remote/data frame."]
    pub mod RTR {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ID Extended. One/zero for extended/standard format frame."]
    pub mod IDE {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Substitute Remote Request. Contains a fixed recessive bit."]
    pub mod SRR {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Message Buffer Code. This 4-bit field can be accessed (read or write) by the CPU and by the FlexCAN module itself, as part of the message buffer matching and arbitration process."]
    pub mod CODE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Error State Indicator. This bit indicates if the transmitting node is error active or error passive."]
    pub mod ESI {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Bit Rate Switch. This bit defines whether the bit rate is switched inside a CAN FD format frame."]
    pub mod BRS {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Extended Data Length. This bit distinguishes between CAN format and CAN FD format frames. The EDL bit must not be set for Message Buffers configured to RANSWER with code field 0b1010."]
    pub mod EDL {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 9 ID Register"]
pub mod ID9 {
    #[doc = "Contains extended (LOW word) identifier of message buffer."]
    pub mod EXT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0003_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Contains standard/extended (HIGH word) identifier of message buffer."]
    pub mod STD {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Local priority. This 3-bit fieldis only used when LPRIO_EN bit is set in MCR and it only makes sense for Tx buffers. These bits are not transmitted. They are appended to the regular ID to define the transmission priority."]
    pub mod PRIO {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 2 WORD_64B Register"]
pub mod MB2_64B_WORD0_L {
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_3 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_2 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_1 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_0 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 2 WORD_64B Register"]
pub mod MB2_64B_WORD1_L {
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_7 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_6 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_5 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_4 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 10 CS Register"]
pub mod CS10 {
    #[doc = "Free-Running Counter Time stamp. This 16-bit field is a copy of the Free-Running Timer, captured for Tx and Rx frames at the time when the beginning of the Identifier field appears on the CAN bus."]
    pub mod TIME_STAMP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Length of the data to be stored/transmitted."]
    pub mod DLC {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Remote Transmission Request. One/zero for remote/data frame."]
    pub mod RTR {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ID Extended. One/zero for extended/standard format frame."]
    pub mod IDE {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Substitute Remote Request. Contains a fixed recessive bit."]
    pub mod SRR {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Message Buffer Code. This 4-bit field can be accessed (read or write) by the CPU and by the FlexCAN module itself, as part of the message buffer matching and arbitration process."]
    pub mod CODE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Error State Indicator. This bit indicates if the transmitting node is error active or error passive."]
    pub mod ESI {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Bit Rate Switch. This bit defines whether the bit rate is switched inside a CAN FD format frame."]
    pub mod BRS {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Extended Data Length. This bit distinguishes between CAN format and CAN FD format frames. The EDL bit must not be set for Message Buffers configured to RANSWER with code field 0b1010."]
    pub mod EDL {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 10 ID Register"]
pub mod ID10 {
    #[doc = "Contains extended (LOW word) identifier of message buffer."]
    pub mod EXT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0003_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Contains standard/extended (HIGH word) identifier of message buffer."]
    pub mod STD {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Local priority. This 3-bit fieldis only used when LPRIO_EN bit is set in MCR and it only makes sense for Tx buffers. These bits are not transmitted. They are appended to the regular ID to define the transmission priority."]
    pub mod PRIO {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 10 WORD_8B Register"]
pub mod MB10_8B_WORD0 {
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_3 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_2 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_1 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_0 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 10 WORD_8B Register"]
pub mod MB10_8B_WORD1 {
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_7 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_6 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_5 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_4 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 11 CS Register"]
pub mod CS11 {
    #[doc = "Free-Running Counter Time stamp. This 16-bit field is a copy of the Free-Running Timer, captured for Tx and Rx frames at the time when the beginning of the Identifier field appears on the CAN bus."]
    pub mod TIME_STAMP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Length of the data to be stored/transmitted."]
    pub mod DLC {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Remote Transmission Request. One/zero for remote/data frame."]
    pub mod RTR {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ID Extended. One/zero for extended/standard format frame."]
    pub mod IDE {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Substitute Remote Request. Contains a fixed recessive bit."]
    pub mod SRR {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Message Buffer Code. This 4-bit field can be accessed (read or write) by the CPU and by the FlexCAN module itself, as part of the message buffer matching and arbitration process."]
    pub mod CODE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Error State Indicator. This bit indicates if the transmitting node is error active or error passive."]
    pub mod ESI {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Bit Rate Switch. This bit defines whether the bit rate is switched inside a CAN FD format frame."]
    pub mod BRS {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Extended Data Length. This bit distinguishes between CAN format and CAN FD format frames. The EDL bit must not be set for Message Buffers configured to RANSWER with code field 0b1010."]
    pub mod EDL {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 11 ID Register"]
pub mod ID11 {
    #[doc = "Contains extended (LOW word) identifier of message buffer."]
    pub mod EXT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0003_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Contains standard/extended (HIGH word) identifier of message buffer."]
    pub mod STD {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Local priority. This 3-bit fieldis only used when LPRIO_EN bit is set in MCR and it only makes sense for Tx buffers. These bits are not transmitted. They are appended to the regular ID to define the transmission priority."]
    pub mod PRIO {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 11 WORD_8B Register"]
pub mod MB11_8B_WORD0 {
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_3 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_2 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_1 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_0 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 11 WORD_8B Register"]
pub mod MB11_8B_WORD1 {
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_7 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_6 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_5 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_4 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 12 CS Register"]
pub mod CS12 {
    #[doc = "Free-Running Counter Time stamp. This 16-bit field is a copy of the Free-Running Timer, captured for Tx and Rx frames at the time when the beginning of the Identifier field appears on the CAN bus."]
    pub mod TIME_STAMP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Length of the data to be stored/transmitted."]
    pub mod DLC {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Remote Transmission Request. One/zero for remote/data frame."]
    pub mod RTR {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ID Extended. One/zero for extended/standard format frame."]
    pub mod IDE {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Substitute Remote Request. Contains a fixed recessive bit."]
    pub mod SRR {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Message Buffer Code. This 4-bit field can be accessed (read or write) by the CPU and by the FlexCAN module itself, as part of the message buffer matching and arbitration process."]
    pub mod CODE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Error State Indicator. This bit indicates if the transmitting node is error active or error passive."]
    pub mod ESI {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Bit Rate Switch. This bit defines whether the bit rate is switched inside a CAN FD format frame."]
    pub mod BRS {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Extended Data Length. This bit distinguishes between CAN format and CAN FD format frames. The EDL bit must not be set for Message Buffers configured to RANSWER with code field 0b1010."]
    pub mod EDL {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 12 ID Register"]
pub mod ID12 {
    #[doc = "Contains extended (LOW word) identifier of message buffer."]
    pub mod EXT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0003_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Contains standard/extended (HIGH word) identifier of message buffer."]
    pub mod STD {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Local priority. This 3-bit fieldis only used when LPRIO_EN bit is set in MCR and it only makes sense for Tx buffers. These bits are not transmitted. They are appended to the regular ID to define the transmission priority."]
    pub mod PRIO {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 12 WORD_8B Register"]
pub mod MB12_8B_WORD0 {
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_3 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_2 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_1 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_0 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 12 WORD_8B Register"]
pub mod MB12_8B_WORD1 {
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_7 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_6 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_5 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_4 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 13 CS Register"]
pub mod CS13 {
    #[doc = "Free-Running Counter Time stamp. This 16-bit field is a copy of the Free-Running Timer, captured for Tx and Rx frames at the time when the beginning of the Identifier field appears on the CAN bus."]
    pub mod TIME_STAMP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Length of the data to be stored/transmitted."]
    pub mod DLC {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Remote Transmission Request. One/zero for remote/data frame."]
    pub mod RTR {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ID Extended. One/zero for extended/standard format frame."]
    pub mod IDE {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Substitute Remote Request. Contains a fixed recessive bit."]
    pub mod SRR {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Message Buffer Code. This 4-bit field can be accessed (read or write) by the CPU and by the FlexCAN module itself, as part of the message buffer matching and arbitration process."]
    pub mod CODE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Error State Indicator. This bit indicates if the transmitting node is error active or error passive."]
    pub mod ESI {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Bit Rate Switch. This bit defines whether the bit rate is switched inside a CAN FD format frame."]
    pub mod BRS {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Extended Data Length. This bit distinguishes between CAN format and CAN FD format frames. The EDL bit must not be set for Message Buffers configured to RANSWER with code field 0b1010."]
    pub mod EDL {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 13 ID Register"]
pub mod ID13 {
    #[doc = "Contains extended (LOW word) identifier of message buffer."]
    pub mod EXT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0003_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Contains standard/extended (HIGH word) identifier of message buffer."]
    pub mod STD {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Local priority. This 3-bit fieldis only used when LPRIO_EN bit is set in MCR and it only makes sense for Tx buffers. These bits are not transmitted. They are appended to the regular ID to define the transmission priority."]
    pub mod PRIO {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 13 WORD_8B Register"]
pub mod MB13_8B_WORD0 {
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_3 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_2 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_1 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_0 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 13 WORD_8B Register"]
pub mod MB13_8B_WORD1 {
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_7 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_6 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_5 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_4 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 14 CS Register"]
pub mod CS14 {
    #[doc = "Free-Running Counter Time stamp. This 16-bit field is a copy of the Free-Running Timer, captured for Tx and Rx frames at the time when the beginning of the Identifier field appears on the CAN bus."]
    pub mod TIME_STAMP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Length of the data to be stored/transmitted."]
    pub mod DLC {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Remote Transmission Request. One/zero for remote/data frame."]
    pub mod RTR {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ID Extended. One/zero for extended/standard format frame."]
    pub mod IDE {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Substitute Remote Request. Contains a fixed recessive bit."]
    pub mod SRR {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Message Buffer Code. This 4-bit field can be accessed (read or write) by the CPU and by the FlexCAN module itself, as part of the message buffer matching and arbitration process."]
    pub mod CODE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Error State Indicator. This bit indicates if the transmitting node is error active or error passive."]
    pub mod ESI {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Bit Rate Switch. This bit defines whether the bit rate is switched inside a CAN FD format frame."]
    pub mod BRS {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Extended Data Length. This bit distinguishes between CAN format and CAN FD format frames. The EDL bit must not be set for Message Buffers configured to RANSWER with code field 0b1010."]
    pub mod EDL {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 14 ID Register"]
pub mod ID14 {
    #[doc = "Contains extended (LOW word) identifier of message buffer."]
    pub mod EXT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0003_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Contains standard/extended (HIGH word) identifier of message buffer."]
    pub mod STD {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Local priority. This 3-bit fieldis only used when LPRIO_EN bit is set in MCR and it only makes sense for Tx buffers. These bits are not transmitted. They are appended to the regular ID to define the transmission priority."]
    pub mod PRIO {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 14 WORD_8B Register"]
pub mod MB14_8B_WORD0 {
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_3 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_2 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_1 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_0 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 14 WORD_8B Register"]
pub mod MB14_8B_WORD1 {
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_7 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_6 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_5 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_4 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 15 CS Register"]
pub mod CS15 {
    #[doc = "Free-Running Counter Time stamp. This 16-bit field is a copy of the Free-Running Timer, captured for Tx and Rx frames at the time when the beginning of the Identifier field appears on the CAN bus."]
    pub mod TIME_STAMP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Length of the data to be stored/transmitted."]
    pub mod DLC {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Remote Transmission Request. One/zero for remote/data frame."]
    pub mod RTR {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ID Extended. One/zero for extended/standard format frame."]
    pub mod IDE {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Substitute Remote Request. Contains a fixed recessive bit."]
    pub mod SRR {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Message Buffer Code. This 4-bit field can be accessed (read or write) by the CPU and by the FlexCAN module itself, as part of the message buffer matching and arbitration process."]
    pub mod CODE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Error State Indicator. This bit indicates if the transmitting node is error active or error passive."]
    pub mod ESI {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Bit Rate Switch. This bit defines whether the bit rate is switched inside a CAN FD format frame."]
    pub mod BRS {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Extended Data Length. This bit distinguishes between CAN format and CAN FD format frames. The EDL bit must not be set for Message Buffers configured to RANSWER with code field 0b1010."]
    pub mod EDL {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 15 ID Register"]
pub mod ID15 {
    #[doc = "Contains extended (LOW word) identifier of message buffer."]
    pub mod EXT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0003_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Contains standard/extended (HIGH word) identifier of message buffer."]
    pub mod STD {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Local priority. This 3-bit fieldis only used when LPRIO_EN bit is set in MCR and it only makes sense for Tx buffers. These bits are not transmitted. They are appended to the regular ID to define the transmission priority."]
    pub mod PRIO {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 10 WORD_16B Register"]
pub mod MB10_16B_WORD0_L {
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_3 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_2 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_1 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_0 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 10 WORD_16B Register"]
pub mod MB10_16B_WORD1_L {
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_7 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_6 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_5 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_4 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 16 CS Register"]
pub mod CS16 {
    #[doc = "Free-Running Counter Time stamp. This 16-bit field is a copy of the Free-Running Timer, captured for Tx and Rx frames at the time when the beginning of the Identifier field appears on the CAN bus."]
    pub mod TIME_STAMP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Length of the data to be stored/transmitted."]
    pub mod DLC {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Remote Transmission Request. One/zero for remote/data frame."]
    pub mod RTR {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ID Extended. One/zero for extended/standard format frame."]
    pub mod IDE {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Substitute Remote Request. Contains a fixed recessive bit."]
    pub mod SRR {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Message Buffer Code. This 4-bit field can be accessed (read or write) by the CPU and by the FlexCAN module itself, as part of the message buffer matching and arbitration process."]
    pub mod CODE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Error State Indicator. This bit indicates if the transmitting node is error active or error passive."]
    pub mod ESI {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Bit Rate Switch. This bit defines whether the bit rate is switched inside a CAN FD format frame."]
    pub mod BRS {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Extended Data Length. This bit distinguishes between CAN format and CAN FD format frames. The EDL bit must not be set for Message Buffers configured to RANSWER with code field 0b1010."]
    pub mod EDL {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 16 ID Register"]
pub mod ID16 {
    #[doc = "Contains extended (LOW word) identifier of message buffer."]
    pub mod EXT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0003_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Contains standard/extended (HIGH word) identifier of message buffer."]
    pub mod STD {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Local priority. This 3-bit fieldis only used when LPRIO_EN bit is set in MCR and it only makes sense for Tx buffers. These bits are not transmitted. They are appended to the regular ID to define the transmission priority."]
    pub mod PRIO {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 11 CS Register"]
pub mod MB11_16B_CS_L {
    #[doc = "Free-Running Counter Time stamp. This 16-bit field is a copy of the Free-Running Timer, captured for Tx and Rx frames at the time when the beginning of the Identifier field appears on the CAN bus."]
    pub mod TIME_STAMP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Length of the data to be stored/transmitted."]
    pub mod DLC {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Remote Transmission Request. One/zero for remote/data frame."]
    pub mod RTR {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ID Extended. One/zero for extended/standard format frame."]
    pub mod IDE {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Substitute Remote Request. Contains a fixed recessive bit."]
    pub mod SRR {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Message Buffer Code. This 4-bit field can be accessed (read or write) by the CPU and by the FlexCAN module itself, as part of the message buffer matching and arbitration process."]
    pub mod CODE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Error State Indicator. This bit indicates if the transmitting node is error active or error passive."]
    pub mod ESI {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Bit Rate Switch. This bit defines whether the bit rate is switched inside a CAN FD format frame."]
    pub mod BRS {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Extended Data Length. This bit distinguishes between CAN format and CAN FD format frames. The EDL bit must not be set for Message Buffers configured to RANSWER with code field 0b1010."]
    pub mod EDL {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 11 ID Register"]
pub mod MB11_16B_ID_L {
    #[doc = "Contains extended (LOW word) identifier of message buffer."]
    pub mod EXT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0003_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Contains standard/extended (HIGH word) identifier of message buffer."]
    pub mod STD {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Local priority. This 3-bit fieldis only used when LPRIO_EN bit is set in MCR and it only makes sense for Tx buffers. These bits are not transmitted. They are appended to the regular ID to define the transmission priority."]
    pub mod PRIO {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 17 CS Register"]
pub mod CS17 {
    #[doc = "Free-Running Counter Time stamp. This 16-bit field is a copy of the Free-Running Timer, captured for Tx and Rx frames at the time when the beginning of the Identifier field appears on the CAN bus."]
    pub mod TIME_STAMP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Length of the data to be stored/transmitted."]
    pub mod DLC {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Remote Transmission Request. One/zero for remote/data frame."]
    pub mod RTR {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ID Extended. One/zero for extended/standard format frame."]
    pub mod IDE {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Substitute Remote Request. Contains a fixed recessive bit."]
    pub mod SRR {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Message Buffer Code. This 4-bit field can be accessed (read or write) by the CPU and by the FlexCAN module itself, as part of the message buffer matching and arbitration process."]
    pub mod CODE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Error State Indicator. This bit indicates if the transmitting node is error active or error passive."]
    pub mod ESI {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Bit Rate Switch. This bit defines whether the bit rate is switched inside a CAN FD format frame."]
    pub mod BRS {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Extended Data Length. This bit distinguishes between CAN format and CAN FD format frames. The EDL bit must not be set for Message Buffers configured to RANSWER with code field 0b1010."]
    pub mod EDL {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 17 ID Register"]
pub mod ID17 {
    #[doc = "Contains extended (LOW word) identifier of message buffer."]
    pub mod EXT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0003_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Contains standard/extended (HIGH word) identifier of message buffer."]
    pub mod STD {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Local priority. This 3-bit fieldis only used when LPRIO_EN bit is set in MCR and it only makes sense for Tx buffers. These bits are not transmitted. They are appended to the regular ID to define the transmission priority."]
    pub mod PRIO {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 11 WORD_16B Register"]
pub mod MB11_16B_WORD2_L {
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_11 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_10 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_9 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_8 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 11 WORD_16B Register"]
pub mod MB11_16B_WORD3_L {
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_15 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_14 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_13 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_12 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 18 CS Register"]
pub mod CS18 {
    #[doc = "Free-Running Counter Time stamp. This 16-bit field is a copy of the Free-Running Timer, captured for Tx and Rx frames at the time when the beginning of the Identifier field appears on the CAN bus."]
    pub mod TIME_STAMP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Length of the data to be stored/transmitted."]
    pub mod DLC {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Remote Transmission Request. One/zero for remote/data frame."]
    pub mod RTR {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ID Extended. One/zero for extended/standard format frame."]
    pub mod IDE {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Substitute Remote Request. Contains a fixed recessive bit."]
    pub mod SRR {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Message Buffer Code. This 4-bit field can be accessed (read or write) by the CPU and by the FlexCAN module itself, as part of the message buffer matching and arbitration process."]
    pub mod CODE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Error State Indicator. This bit indicates if the transmitting node is error active or error passive."]
    pub mod ESI {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Bit Rate Switch. This bit defines whether the bit rate is switched inside a CAN FD format frame."]
    pub mod BRS {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Extended Data Length. This bit distinguishes between CAN format and CAN FD format frames. The EDL bit must not be set for Message Buffers configured to RANSWER with code field 0b1010."]
    pub mod EDL {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 18 ID Register"]
pub mod ID18 {
    #[doc = "Contains extended (LOW word) identifier of message buffer."]
    pub mod EXT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0003_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Contains standard/extended (HIGH word) identifier of message buffer."]
    pub mod STD {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Local priority. This 3-bit fieldis only used when LPRIO_EN bit is set in MCR and it only makes sense for Tx buffers. These bits are not transmitted. They are appended to the regular ID to define the transmission priority."]
    pub mod PRIO {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 12 WORD_16B Register"]
pub mod MB12_16B_WORD0_L {
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_3 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_2 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_1 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_0 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 12 WORD_16B Register"]
pub mod MB12_16B_WORD1_L {
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_7 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_6 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_5 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_4 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 19 CS Register"]
pub mod CS19 {
    #[doc = "Free-Running Counter Time stamp. This 16-bit field is a copy of the Free-Running Timer, captured for Tx and Rx frames at the time when the beginning of the Identifier field appears on the CAN bus."]
    pub mod TIME_STAMP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Length of the data to be stored/transmitted."]
    pub mod DLC {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Remote Transmission Request. One/zero for remote/data frame."]
    pub mod RTR {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ID Extended. One/zero for extended/standard format frame."]
    pub mod IDE {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Substitute Remote Request. Contains a fixed recessive bit."]
    pub mod SRR {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Message Buffer Code. This 4-bit field can be accessed (read or write) by the CPU and by the FlexCAN module itself, as part of the message buffer matching and arbitration process."]
    pub mod CODE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Error State Indicator. This bit indicates if the transmitting node is error active or error passive."]
    pub mod ESI {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Bit Rate Switch. This bit defines whether the bit rate is switched inside a CAN FD format frame."]
    pub mod BRS {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Extended Data Length. This bit distinguishes between CAN format and CAN FD format frames. The EDL bit must not be set for Message Buffers configured to RANSWER with code field 0b1010."]
    pub mod EDL {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 19 ID Register"]
pub mod ID19 {
    #[doc = "Contains extended (LOW word) identifier of message buffer."]
    pub mod EXT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0003_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Contains standard/extended (HIGH word) identifier of message buffer."]
    pub mod STD {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Local priority. This 3-bit fieldis only used when LPRIO_EN bit is set in MCR and it only makes sense for Tx buffers. These bits are not transmitted. They are appended to the regular ID to define the transmission priority."]
    pub mod PRIO {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 13 CS Register"]
pub mod MB13_16B_CS_L {
    #[doc = "Free-Running Counter Time stamp. This 16-bit field is a copy of the Free-Running Timer, captured for Tx and Rx frames at the time when the beginning of the Identifier field appears on the CAN bus."]
    pub mod TIME_STAMP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Length of the data to be stored/transmitted."]
    pub mod DLC {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Remote Transmission Request. One/zero for remote/data frame."]
    pub mod RTR {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ID Extended. One/zero for extended/standard format frame."]
    pub mod IDE {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Substitute Remote Request. Contains a fixed recessive bit."]
    pub mod SRR {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Message Buffer Code. This 4-bit field can be accessed (read or write) by the CPU and by the FlexCAN module itself, as part of the message buffer matching and arbitration process."]
    pub mod CODE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Error State Indicator. This bit indicates if the transmitting node is error active or error passive."]
    pub mod ESI {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Bit Rate Switch. This bit defines whether the bit rate is switched inside a CAN FD format frame."]
    pub mod BRS {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Extended Data Length. This bit distinguishes between CAN format and CAN FD format frames. The EDL bit must not be set for Message Buffers configured to RANSWER with code field 0b1010."]
    pub mod EDL {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 13 ID Register"]
pub mod MB13_16B_ID_L {
    #[doc = "Contains extended (LOW word) identifier of message buffer."]
    pub mod EXT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0003_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Contains standard/extended (HIGH word) identifier of message buffer."]
    pub mod STD {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Local priority. This 3-bit fieldis only used when LPRIO_EN bit is set in MCR and it only makes sense for Tx buffers. These bits are not transmitted. They are appended to the regular ID to define the transmission priority."]
    pub mod PRIO {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 20 CS Register"]
pub mod CS20 {
    #[doc = "Free-Running Counter Time stamp. This 16-bit field is a copy of the Free-Running Timer, captured for Tx and Rx frames at the time when the beginning of the Identifier field appears on the CAN bus."]
    pub mod TIME_STAMP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Length of the data to be stored/transmitted."]
    pub mod DLC {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Remote Transmission Request. One/zero for remote/data frame."]
    pub mod RTR {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ID Extended. One/zero for extended/standard format frame."]
    pub mod IDE {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Substitute Remote Request. Contains a fixed recessive bit."]
    pub mod SRR {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Message Buffer Code. This 4-bit field can be accessed (read or write) by the CPU and by the FlexCAN module itself, as part of the message buffer matching and arbitration process."]
    pub mod CODE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Error State Indicator. This bit indicates if the transmitting node is error active or error passive."]
    pub mod ESI {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Bit Rate Switch. This bit defines whether the bit rate is switched inside a CAN FD format frame."]
    pub mod BRS {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Extended Data Length. This bit distinguishes between CAN format and CAN FD format frames. The EDL bit must not be set for Message Buffers configured to RANSWER with code field 0b1010."]
    pub mod EDL {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 20 ID Register"]
pub mod ID20 {
    #[doc = "Contains extended (LOW word) identifier of message buffer."]
    pub mod EXT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0003_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Contains standard/extended (HIGH word) identifier of message buffer."]
    pub mod STD {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Local priority. This 3-bit fieldis only used when LPRIO_EN bit is set in MCR and it only makes sense for Tx buffers. These bits are not transmitted. They are appended to the regular ID to define the transmission priority."]
    pub mod PRIO {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 13 WORD_16B Register"]
pub mod MB13_16B_WORD2_L {
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_11 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_10 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_9 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_8 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 13 WORD_16B Register"]
pub mod MB13_16B_WORD3_L {
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_15 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_14 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_13 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_12 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 21 CS Register"]
pub mod CS21 {
    #[doc = "Free-Running Counter Time stamp. This 16-bit field is a copy of the Free-Running Timer, captured for Tx and Rx frames at the time when the beginning of the Identifier field appears on the CAN bus."]
    pub mod TIME_STAMP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Length of the data to be stored/transmitted."]
    pub mod DLC {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Remote Transmission Request. One/zero for remote/data frame."]
    pub mod RTR {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ID Extended. One/zero for extended/standard format frame."]
    pub mod IDE {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Substitute Remote Request. Contains a fixed recessive bit."]
    pub mod SRR {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Message Buffer Code. This 4-bit field can be accessed (read or write) by the CPU and by the FlexCAN module itself, as part of the message buffer matching and arbitration process."]
    pub mod CODE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Error State Indicator. This bit indicates if the transmitting node is error active or error passive."]
    pub mod ESI {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Bit Rate Switch. This bit defines whether the bit rate is switched inside a CAN FD format frame."]
    pub mod BRS {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Extended Data Length. This bit distinguishes between CAN format and CAN FD format frames. The EDL bit must not be set for Message Buffers configured to RANSWER with code field 0b1010."]
    pub mod EDL {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 21 ID Register"]
pub mod ID21 {
    #[doc = "Contains extended (LOW word) identifier of message buffer."]
    pub mod EXT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0003_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Contains standard/extended (HIGH word) identifier of message buffer."]
    pub mod STD {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Local priority. This 3-bit fieldis only used when LPRIO_EN bit is set in MCR and it only makes sense for Tx buffers. These bits are not transmitted. They are appended to the regular ID to define the transmission priority."]
    pub mod PRIO {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 14 WORD_16B Register"]
pub mod MB14_16B_WORD0_L {
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_3 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_2 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_1 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_0 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 14 WORD_16B Register"]
pub mod MB14_16B_WORD1_L {
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_7 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_6 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_5 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_4 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 22 CS Register"]
pub mod CS22 {
    #[doc = "Free-Running Counter Time stamp. This 16-bit field is a copy of the Free-Running Timer, captured for Tx and Rx frames at the time when the beginning of the Identifier field appears on the CAN bus."]
    pub mod TIME_STAMP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Length of the data to be stored/transmitted."]
    pub mod DLC {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Remote Transmission Request. One/zero for remote/data frame."]
    pub mod RTR {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ID Extended. One/zero for extended/standard format frame."]
    pub mod IDE {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Substitute Remote Request. Contains a fixed recessive bit."]
    pub mod SRR {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Message Buffer Code. This 4-bit field can be accessed (read or write) by the CPU and by the FlexCAN module itself, as part of the message buffer matching and arbitration process."]
    pub mod CODE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Error State Indicator. This bit indicates if the transmitting node is error active or error passive."]
    pub mod ESI {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Bit Rate Switch. This bit defines whether the bit rate is switched inside a CAN FD format frame."]
    pub mod BRS {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Extended Data Length. This bit distinguishes between CAN format and CAN FD format frames. The EDL bit must not be set for Message Buffers configured to RANSWER with code field 0b1010."]
    pub mod EDL {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 22 ID Register"]
pub mod ID22 {
    #[doc = "Contains extended (LOW word) identifier of message buffer."]
    pub mod EXT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0003_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Contains standard/extended (HIGH word) identifier of message buffer."]
    pub mod STD {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Local priority. This 3-bit fieldis only used when LPRIO_EN bit is set in MCR and it only makes sense for Tx buffers. These bits are not transmitted. They are appended to the regular ID to define the transmission priority."]
    pub mod PRIO {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 15 CS Register"]
pub mod MB15_16B_CS_L {
    #[doc = "Free-Running Counter Time stamp. This 16-bit field is a copy of the Free-Running Timer, captured for Tx and Rx frames at the time when the beginning of the Identifier field appears on the CAN bus."]
    pub mod TIME_STAMP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Length of the data to be stored/transmitted."]
    pub mod DLC {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Remote Transmission Request. One/zero for remote/data frame."]
    pub mod RTR {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ID Extended. One/zero for extended/standard format frame."]
    pub mod IDE {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Substitute Remote Request. Contains a fixed recessive bit."]
    pub mod SRR {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Message Buffer Code. This 4-bit field can be accessed (read or write) by the CPU and by the FlexCAN module itself, as part of the message buffer matching and arbitration process."]
    pub mod CODE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Error State Indicator. This bit indicates if the transmitting node is error active or error passive."]
    pub mod ESI {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Bit Rate Switch. This bit defines whether the bit rate is switched inside a CAN FD format frame."]
    pub mod BRS {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Extended Data Length. This bit distinguishes between CAN format and CAN FD format frames. The EDL bit must not be set for Message Buffers configured to RANSWER with code field 0b1010."]
    pub mod EDL {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 15 ID Register"]
pub mod MB15_16B_ID_L {
    #[doc = "Contains extended (LOW word) identifier of message buffer."]
    pub mod EXT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0003_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Contains standard/extended (HIGH word) identifier of message buffer."]
    pub mod STD {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Local priority. This 3-bit fieldis only used when LPRIO_EN bit is set in MCR and it only makes sense for Tx buffers. These bits are not transmitted. They are appended to the regular ID to define the transmission priority."]
    pub mod PRIO {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 23 CS Register"]
pub mod CS23 {
    #[doc = "Free-Running Counter Time stamp. This 16-bit field is a copy of the Free-Running Timer, captured for Tx and Rx frames at the time when the beginning of the Identifier field appears on the CAN bus."]
    pub mod TIME_STAMP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Length of the data to be stored/transmitted."]
    pub mod DLC {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Remote Transmission Request. One/zero for remote/data frame."]
    pub mod RTR {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ID Extended. One/zero for extended/standard format frame."]
    pub mod IDE {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Substitute Remote Request. Contains a fixed recessive bit."]
    pub mod SRR {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Message Buffer Code. This 4-bit field can be accessed (read or write) by the CPU and by the FlexCAN module itself, as part of the message buffer matching and arbitration process."]
    pub mod CODE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Error State Indicator. This bit indicates if the transmitting node is error active or error passive."]
    pub mod ESI {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Bit Rate Switch. This bit defines whether the bit rate is switched inside a CAN FD format frame."]
    pub mod BRS {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Extended Data Length. This bit distinguishes between CAN format and CAN FD format frames. The EDL bit must not be set for Message Buffers configured to RANSWER with code field 0b1010."]
    pub mod EDL {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 23 ID Register"]
pub mod ID23 {
    #[doc = "Contains extended (LOW word) identifier of message buffer."]
    pub mod EXT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0003_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Contains standard/extended (HIGH word) identifier of message buffer."]
    pub mod STD {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Local priority. This 3-bit fieldis only used when LPRIO_EN bit is set in MCR and it only makes sense for Tx buffers. These bits are not transmitted. They are appended to the regular ID to define the transmission priority."]
    pub mod PRIO {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 15 WORD_16B Register"]
pub mod MB15_16B_WORD2_L {
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_11 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_10 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_9 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_8 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 15 WORD_16B Register"]
pub mod MB15_16B_WORD3_L {
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_15 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_14 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_13 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_12 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 24 CS Register"]
pub mod CS24 {
    #[doc = "Free-Running Counter Time stamp. This 16-bit field is a copy of the Free-Running Timer, captured for Tx and Rx frames at the time when the beginning of the Identifier field appears on the CAN bus."]
    pub mod TIME_STAMP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Length of the data to be stored/transmitted."]
    pub mod DLC {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Remote Transmission Request. One/zero for remote/data frame."]
    pub mod RTR {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ID Extended. One/zero for extended/standard format frame."]
    pub mod IDE {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Substitute Remote Request. Contains a fixed recessive bit."]
    pub mod SRR {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Message Buffer Code. This 4-bit field can be accessed (read or write) by the CPU and by the FlexCAN module itself, as part of the message buffer matching and arbitration process."]
    pub mod CODE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Error State Indicator. This bit indicates if the transmitting node is error active or error passive."]
    pub mod ESI {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Bit Rate Switch. This bit defines whether the bit rate is switched inside a CAN FD format frame."]
    pub mod BRS {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Extended Data Length. This bit distinguishes between CAN format and CAN FD format frames. The EDL bit must not be set for Message Buffers configured to RANSWER with code field 0b1010."]
    pub mod EDL {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 24 ID Register"]
pub mod ID24 {
    #[doc = "Contains extended (LOW word) identifier of message buffer."]
    pub mod EXT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0003_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Contains standard/extended (HIGH word) identifier of message buffer."]
    pub mod STD {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Local priority. This 3-bit fieldis only used when LPRIO_EN bit is set in MCR and it only makes sense for Tx buffers. These bits are not transmitted. They are appended to the regular ID to define the transmission priority."]
    pub mod PRIO {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 16 WORD_16B Register"]
pub mod MB16_16B_WORD0_L {
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_3 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_2 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_1 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_0 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 16 WORD_16B Register"]
pub mod MB16_16B_WORD1_L {
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_7 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_6 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_5 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_4 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 25 CS Register"]
pub mod CS25 {
    #[doc = "Free-Running Counter Time stamp. This 16-bit field is a copy of the Free-Running Timer, captured for Tx and Rx frames at the time when the beginning of the Identifier field appears on the CAN bus."]
    pub mod TIME_STAMP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Length of the data to be stored/transmitted."]
    pub mod DLC {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Remote Transmission Request. One/zero for remote/data frame."]
    pub mod RTR {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ID Extended. One/zero for extended/standard format frame."]
    pub mod IDE {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Substitute Remote Request. Contains a fixed recessive bit."]
    pub mod SRR {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Message Buffer Code. This 4-bit field can be accessed (read or write) by the CPU and by the FlexCAN module itself, as part of the message buffer matching and arbitration process."]
    pub mod CODE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Error State Indicator. This bit indicates if the transmitting node is error active or error passive."]
    pub mod ESI {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Bit Rate Switch. This bit defines whether the bit rate is switched inside a CAN FD format frame."]
    pub mod BRS {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Extended Data Length. This bit distinguishes between CAN format and CAN FD format frames. The EDL bit must not be set for Message Buffers configured to RANSWER with code field 0b1010."]
    pub mod EDL {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 25 ID Register"]
pub mod ID25 {
    #[doc = "Contains extended (LOW word) identifier of message buffer."]
    pub mod EXT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0003_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Contains standard/extended (HIGH word) identifier of message buffer."]
    pub mod STD {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Local priority. This 3-bit fieldis only used when LPRIO_EN bit is set in MCR and it only makes sense for Tx buffers. These bits are not transmitted. They are appended to the regular ID to define the transmission priority."]
    pub mod PRIO {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 10 WORD_32B Register"]
pub mod MB10_32B_WORD0_L {
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_3 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_2 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_1 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_0 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 10 WORD_32B Register"]
pub mod MB10_32B_WORD1_L {
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_7 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_6 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_5 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_4 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 26 CS Register"]
pub mod CS26 {
    #[doc = "Free-Running Counter Time stamp. This 16-bit field is a copy of the Free-Running Timer, captured for Tx and Rx frames at the time when the beginning of the Identifier field appears on the CAN bus."]
    pub mod TIME_STAMP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Length of the data to be stored/transmitted."]
    pub mod DLC {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Remote Transmission Request. One/zero for remote/data frame."]
    pub mod RTR {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ID Extended. One/zero for extended/standard format frame."]
    pub mod IDE {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Substitute Remote Request. Contains a fixed recessive bit."]
    pub mod SRR {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Message Buffer Code. This 4-bit field can be accessed (read or write) by the CPU and by the FlexCAN module itself, as part of the message buffer matching and arbitration process."]
    pub mod CODE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Error State Indicator. This bit indicates if the transmitting node is error active or error passive."]
    pub mod ESI {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Bit Rate Switch. This bit defines whether the bit rate is switched inside a CAN FD format frame."]
    pub mod BRS {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Extended Data Length. This bit distinguishes between CAN format and CAN FD format frames. The EDL bit must not be set for Message Buffers configured to RANSWER with code field 0b1010."]
    pub mod EDL {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 26 ID Register"]
pub mod ID26 {
    #[doc = "Contains extended (LOW word) identifier of message buffer."]
    pub mod EXT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0003_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Contains standard/extended (HIGH word) identifier of message buffer."]
    pub mod STD {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Local priority. This 3-bit fieldis only used when LPRIO_EN bit is set in MCR and it only makes sense for Tx buffers. These bits are not transmitted. They are appended to the regular ID to define the transmission priority."]
    pub mod PRIO {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 10 WORD_32B Register"]
pub mod MB10_32B_WORD4_L {
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_19 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_18 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_17 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_16 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 10 WORD_32B Register"]
pub mod MB10_32B_WORD5_L {
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_23 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_22 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_21 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_20 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 27 CS Register"]
pub mod CS27 {
    #[doc = "Free-Running Counter Time stamp. This 16-bit field is a copy of the Free-Running Timer, captured for Tx and Rx frames at the time when the beginning of the Identifier field appears on the CAN bus."]
    pub mod TIME_STAMP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Length of the data to be stored/transmitted."]
    pub mod DLC {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Remote Transmission Request. One/zero for remote/data frame."]
    pub mod RTR {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ID Extended. One/zero for extended/standard format frame."]
    pub mod IDE {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Substitute Remote Request. Contains a fixed recessive bit."]
    pub mod SRR {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Message Buffer Code. This 4-bit field can be accessed (read or write) by the CPU and by the FlexCAN module itself, as part of the message buffer matching and arbitration process."]
    pub mod CODE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Error State Indicator. This bit indicates if the transmitting node is error active or error passive."]
    pub mod ESI {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Bit Rate Switch. This bit defines whether the bit rate is switched inside a CAN FD format frame."]
    pub mod BRS {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Extended Data Length. This bit distinguishes between CAN format and CAN FD format frames. The EDL bit must not be set for Message Buffers configured to RANSWER with code field 0b1010."]
    pub mod EDL {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 27 ID Register"]
pub mod ID27 {
    #[doc = "Contains extended (LOW word) identifier of message buffer."]
    pub mod EXT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0003_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Contains standard/extended (HIGH word) identifier of message buffer."]
    pub mod STD {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Local priority. This 3-bit fieldis only used when LPRIO_EN bit is set in MCR and it only makes sense for Tx buffers. These bits are not transmitted. They are appended to the regular ID to define the transmission priority."]
    pub mod PRIO {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 11 CS Register"]
pub mod MB11_32B_CS_L {
    #[doc = "Free-Running Counter Time stamp. This 16-bit field is a copy of the Free-Running Timer, captured for Tx and Rx frames at the time when the beginning of the Identifier field appears on the CAN bus."]
    pub mod TIME_STAMP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Length of the data to be stored/transmitted."]
    pub mod DLC {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Remote Transmission Request. One/zero for remote/data frame."]
    pub mod RTR {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ID Extended. One/zero for extended/standard format frame."]
    pub mod IDE {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Substitute Remote Request. Contains a fixed recessive bit."]
    pub mod SRR {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Message Buffer Code. This 4-bit field can be accessed (read or write) by the CPU and by the FlexCAN module itself, as part of the message buffer matching and arbitration process."]
    pub mod CODE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Error State Indicator. This bit indicates if the transmitting node is error active or error passive."]
    pub mod ESI {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Bit Rate Switch. This bit defines whether the bit rate is switched inside a CAN FD format frame."]
    pub mod BRS {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Extended Data Length. This bit distinguishes between CAN format and CAN FD format frames. The EDL bit must not be set for Message Buffers configured to RANSWER with code field 0b1010."]
    pub mod EDL {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 11 ID Register"]
pub mod MB11_32B_ID_L {
    #[doc = "Contains extended (LOW word) identifier of message buffer."]
    pub mod EXT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0003_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Contains standard/extended (HIGH word) identifier of message buffer."]
    pub mod STD {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Local priority. This 3-bit fieldis only used when LPRIO_EN bit is set in MCR and it only makes sense for Tx buffers. These bits are not transmitted. They are appended to the regular ID to define the transmission priority."]
    pub mod PRIO {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 28 CS Register"]
pub mod CS28 {
    #[doc = "Free-Running Counter Time stamp. This 16-bit field is a copy of the Free-Running Timer, captured for Tx and Rx frames at the time when the beginning of the Identifier field appears on the CAN bus."]
    pub mod TIME_STAMP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Length of the data to be stored/transmitted."]
    pub mod DLC {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Remote Transmission Request. One/zero for remote/data frame."]
    pub mod RTR {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ID Extended. One/zero for extended/standard format frame."]
    pub mod IDE {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Substitute Remote Request. Contains a fixed recessive bit."]
    pub mod SRR {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Message Buffer Code. This 4-bit field can be accessed (read or write) by the CPU and by the FlexCAN module itself, as part of the message buffer matching and arbitration process."]
    pub mod CODE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Error State Indicator. This bit indicates if the transmitting node is error active or error passive."]
    pub mod ESI {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Bit Rate Switch. This bit defines whether the bit rate is switched inside a CAN FD format frame."]
    pub mod BRS {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Extended Data Length. This bit distinguishes between CAN format and CAN FD format frames. The EDL bit must not be set for Message Buffers configured to RANSWER with code field 0b1010."]
    pub mod EDL {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 28 ID Register"]
pub mod ID28 {
    #[doc = "Contains extended (LOW word) identifier of message buffer."]
    pub mod EXT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0003_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Contains standard/extended (HIGH word) identifier of message buffer."]
    pub mod STD {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Local priority. This 3-bit fieldis only used when LPRIO_EN bit is set in MCR and it only makes sense for Tx buffers. These bits are not transmitted. They are appended to the regular ID to define the transmission priority."]
    pub mod PRIO {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 11 WORD_32B Register"]
pub mod MB11_32B_WORD2_L {
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_11 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_10 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_9 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_8 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 11 WORD_32B Register"]
pub mod MB11_32B_WORD3_L {
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_15 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_14 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_13 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_12 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 29 CS Register"]
pub mod CS29 {
    #[doc = "Free-Running Counter Time stamp. This 16-bit field is a copy of the Free-Running Timer, captured for Tx and Rx frames at the time when the beginning of the Identifier field appears on the CAN bus."]
    pub mod TIME_STAMP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Length of the data to be stored/transmitted."]
    pub mod DLC {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Remote Transmission Request. One/zero for remote/data frame."]
    pub mod RTR {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ID Extended. One/zero for extended/standard format frame."]
    pub mod IDE {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Substitute Remote Request. Contains a fixed recessive bit."]
    pub mod SRR {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Message Buffer Code. This 4-bit field can be accessed (read or write) by the CPU and by the FlexCAN module itself, as part of the message buffer matching and arbitration process."]
    pub mod CODE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Error State Indicator. This bit indicates if the transmitting node is error active or error passive."]
    pub mod ESI {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Bit Rate Switch. This bit defines whether the bit rate is switched inside a CAN FD format frame."]
    pub mod BRS {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Extended Data Length. This bit distinguishes between CAN format and CAN FD format frames. The EDL bit must not be set for Message Buffers configured to RANSWER with code field 0b1010."]
    pub mod EDL {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 29 ID Register"]
pub mod ID29 {
    #[doc = "Contains extended (LOW word) identifier of message buffer."]
    pub mod EXT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0003_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Contains standard/extended (HIGH word) identifier of message buffer."]
    pub mod STD {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Local priority. This 3-bit fieldis only used when LPRIO_EN bit is set in MCR and it only makes sense for Tx buffers. These bits are not transmitted. They are appended to the regular ID to define the transmission priority."]
    pub mod PRIO {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 11 WORD_32B Register"]
pub mod MB11_32B_WORD6_L {
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_27 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_26 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_25 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_24 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 11 WORD_32B Register"]
pub mod MB11_32B_WORD7_L {
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_31 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_30 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_29 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_28 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 30 CS Register"]
pub mod CS30 {
    #[doc = "Free-Running Counter Time stamp. This 16-bit field is a copy of the Free-Running Timer, captured for Tx and Rx frames at the time when the beginning of the Identifier field appears on the CAN bus."]
    pub mod TIME_STAMP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Length of the data to be stored/transmitted."]
    pub mod DLC {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Remote Transmission Request. One/zero for remote/data frame."]
    pub mod RTR {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ID Extended. One/zero for extended/standard format frame."]
    pub mod IDE {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Substitute Remote Request. Contains a fixed recessive bit."]
    pub mod SRR {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Message Buffer Code. This 4-bit field can be accessed (read or write) by the CPU and by the FlexCAN module itself, as part of the message buffer matching and arbitration process."]
    pub mod CODE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Error State Indicator. This bit indicates if the transmitting node is error active or error passive."]
    pub mod ESI {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Bit Rate Switch. This bit defines whether the bit rate is switched inside a CAN FD format frame."]
    pub mod BRS {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Extended Data Length. This bit distinguishes between CAN format and CAN FD format frames. The EDL bit must not be set for Message Buffers configured to RANSWER with code field 0b1010."]
    pub mod EDL {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 30 ID Register"]
pub mod ID30 {
    #[doc = "Contains extended (LOW word) identifier of message buffer."]
    pub mod EXT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0003_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Contains standard/extended (HIGH word) identifier of message buffer."]
    pub mod STD {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Local priority. This 3-bit fieldis only used when LPRIO_EN bit is set in MCR and it only makes sense for Tx buffers. These bits are not transmitted. They are appended to the regular ID to define the transmission priority."]
    pub mod PRIO {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 20 WORD_16B Register"]
pub mod MB20_16B_WORD0_L {
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_3 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_2 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_1 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_0 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 20 WORD_16B Register"]
pub mod MB20_16B_WORD1_L {
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_7 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_6 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_5 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_4 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 31 CS Register"]
pub mod CS31 {
    #[doc = "Free-Running Counter Time stamp. This 16-bit field is a copy of the Free-Running Timer, captured for Tx and Rx frames at the time when the beginning of the Identifier field appears on the CAN bus."]
    pub mod TIME_STAMP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Length of the data to be stored/transmitted."]
    pub mod DLC {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Remote Transmission Request. One/zero for remote/data frame."]
    pub mod RTR {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ID Extended. One/zero for extended/standard format frame."]
    pub mod IDE {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Substitute Remote Request. Contains a fixed recessive bit."]
    pub mod SRR {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Message Buffer Code. This 4-bit field can be accessed (read or write) by the CPU and by the FlexCAN module itself, as part of the message buffer matching and arbitration process."]
    pub mod CODE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Error State Indicator. This bit indicates if the transmitting node is error active or error passive."]
    pub mod ESI {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Bit Rate Switch. This bit defines whether the bit rate is switched inside a CAN FD format frame."]
    pub mod BRS {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Extended Data Length. This bit distinguishes between CAN format and CAN FD format frames. The EDL bit must not be set for Message Buffers configured to RANSWER with code field 0b1010."]
    pub mod EDL {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 31 ID Register"]
pub mod ID31 {
    #[doc = "Contains extended (LOW word) identifier of message buffer."]
    pub mod EXT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0003_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Contains standard/extended (HIGH word) identifier of message buffer."]
    pub mod STD {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Local priority. This 3-bit fieldis only used when LPRIO_EN bit is set in MCR and it only makes sense for Tx buffers. These bits are not transmitted. They are appended to the regular ID to define the transmission priority."]
    pub mod PRIO {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 31 WORD_8B Register"]
pub mod MB31_8B_WORD0 {
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_3 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_2 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_1 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_0 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 31 WORD_8B Register"]
pub mod MB31_8B_WORD1 {
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_7 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_6 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_5 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_4 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 32 CS Register"]
pub mod CS32 {
    #[doc = "Free-Running Counter Time stamp. This 16-bit field is a copy of the Free-Running Timer, captured for Tx and Rx frames at the time when the beginning of the Identifier field appears on the CAN bus."]
    pub mod TIME_STAMP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Length of the data to be stored/transmitted."]
    pub mod DLC {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Remote Transmission Request. One/zero for remote/data frame."]
    pub mod RTR {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ID Extended. One/zero for extended/standard format frame."]
    pub mod IDE {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Substitute Remote Request. Contains a fixed recessive bit."]
    pub mod SRR {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Message Buffer Code. This 4-bit field can be accessed (read or write) by the CPU and by the FlexCAN module itself, as part of the message buffer matching and arbitration process."]
    pub mod CODE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Error State Indicator. This bit indicates if the transmitting node is error active or error passive."]
    pub mod ESI {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Bit Rate Switch. This bit defines whether the bit rate is switched inside a CAN FD format frame."]
    pub mod BRS {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Extended Data Length. This bit distinguishes between CAN format and CAN FD format frames. The EDL bit must not be set for Message Buffers configured to RANSWER with code field 0b1010."]
    pub mod EDL {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 32 ID Register"]
pub mod ID32 {
    #[doc = "Contains extended (LOW word) identifier of message buffer."]
    pub mod EXT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0003_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Contains standard/extended (HIGH word) identifier of message buffer."]
    pub mod STD {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Local priority. This 3-bit fieldis only used when LPRIO_EN bit is set in MCR and it only makes sense for Tx buffers. These bits are not transmitted. They are appended to the regular ID to define the transmission priority."]
    pub mod PRIO {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 0 WORD_16B Register"]
pub mod MB0_16B_WORD0_M {
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_3 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_2 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_1 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_0 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 0 WORD_16B Register"]
pub mod MB0_16B_WORD1_M {
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_7 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_6 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_5 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_4 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 33 CS Register"]
pub mod CS33 {
    #[doc = "Free-Running Counter Time stamp. This 16-bit field is a copy of the Free-Running Timer, captured for Tx and Rx frames at the time when the beginning of the Identifier field appears on the CAN bus."]
    pub mod TIME_STAMP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Length of the data to be stored/transmitted."]
    pub mod DLC {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Remote Transmission Request. One/zero for remote/data frame."]
    pub mod RTR {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ID Extended. One/zero for extended/standard format frame."]
    pub mod IDE {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Substitute Remote Request. Contains a fixed recessive bit."]
    pub mod SRR {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Message Buffer Code. This 4-bit field can be accessed (read or write) by the CPU and by the FlexCAN module itself, as part of the message buffer matching and arbitration process."]
    pub mod CODE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Error State Indicator. This bit indicates if the transmitting node is error active or error passive."]
    pub mod ESI {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Bit Rate Switch. This bit defines whether the bit rate is switched inside a CAN FD format frame."]
    pub mod BRS {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Extended Data Length. This bit distinguishes between CAN format and CAN FD format frames. The EDL bit must not be set for Message Buffers configured to RANSWER with code field 0b1010."]
    pub mod EDL {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 33 ID Register"]
pub mod ID33 {
    #[doc = "Contains extended (LOW word) identifier of message buffer."]
    pub mod EXT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0003_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Contains standard/extended (HIGH word) identifier of message buffer."]
    pub mod STD {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Local priority. This 3-bit fieldis only used when LPRIO_EN bit is set in MCR and it only makes sense for Tx buffers. These bits are not transmitted. They are appended to the regular ID to define the transmission priority."]
    pub mod PRIO {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 0 WORD_32B Register"]
pub mod MB0_32B_WORD4_M {
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_19 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_18 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_17 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_16 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 0 WORD_32B Register"]
pub mod MB0_32B_WORD5_M {
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_23 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_22 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_21 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_20 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 34 CS Register"]
pub mod CS34 {
    #[doc = "Free-Running Counter Time stamp. This 16-bit field is a copy of the Free-Running Timer, captured for Tx and Rx frames at the time when the beginning of the Identifier field appears on the CAN bus."]
    pub mod TIME_STAMP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Length of the data to be stored/transmitted."]
    pub mod DLC {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Remote Transmission Request. One/zero for remote/data frame."]
    pub mod RTR {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ID Extended. One/zero for extended/standard format frame."]
    pub mod IDE {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Substitute Remote Request. Contains a fixed recessive bit."]
    pub mod SRR {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Message Buffer Code. This 4-bit field can be accessed (read or write) by the CPU and by the FlexCAN module itself, as part of the message buffer matching and arbitration process."]
    pub mod CODE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Error State Indicator. This bit indicates if the transmitting node is error active or error passive."]
    pub mod ESI {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Bit Rate Switch. This bit defines whether the bit rate is switched inside a CAN FD format frame."]
    pub mod BRS {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Extended Data Length. This bit distinguishes between CAN format and CAN FD format frames. The EDL bit must not be set for Message Buffers configured to RANSWER with code field 0b1010."]
    pub mod EDL {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 34 ID Register"]
pub mod ID34 {
    #[doc = "Contains extended (LOW word) identifier of message buffer."]
    pub mod EXT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0003_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Contains standard/extended (HIGH word) identifier of message buffer."]
    pub mod STD {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Local priority. This 3-bit fieldis only used when LPRIO_EN bit is set in MCR and it only makes sense for Tx buffers. These bits are not transmitted. They are appended to the regular ID to define the transmission priority."]
    pub mod PRIO {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 0 WORD_64B Register"]
pub mod MB0_64B_WORD8_M {
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_35 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_34 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_33 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_32 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 0 WORD_64B Register"]
pub mod MB0_64B_WORD9_M {
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_39 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_38 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_37 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_36 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 35 CS Register"]
pub mod CS35 {
    #[doc = "Free-Running Counter Time stamp. This 16-bit field is a copy of the Free-Running Timer, captured for Tx and Rx frames at the time when the beginning of the Identifier field appears on the CAN bus."]
    pub mod TIME_STAMP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Length of the data to be stored/transmitted."]
    pub mod DLC {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Remote Transmission Request. One/zero for remote/data frame."]
    pub mod RTR {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ID Extended. One/zero for extended/standard format frame."]
    pub mod IDE {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Substitute Remote Request. Contains a fixed recessive bit."]
    pub mod SRR {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Message Buffer Code. This 4-bit field can be accessed (read or write) by the CPU and by the FlexCAN module itself, as part of the message buffer matching and arbitration process."]
    pub mod CODE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Error State Indicator. This bit indicates if the transmitting node is error active or error passive."]
    pub mod ESI {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Bit Rate Switch. This bit defines whether the bit rate is switched inside a CAN FD format frame."]
    pub mod BRS {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Extended Data Length. This bit distinguishes between CAN format and CAN FD format frames. The EDL bit must not be set for Message Buffers configured to RANSWER with code field 0b1010."]
    pub mod EDL {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 35 ID Register"]
pub mod ID35 {
    #[doc = "Contains extended (LOW word) identifier of message buffer."]
    pub mod EXT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0003_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Contains standard/extended (HIGH word) identifier of message buffer."]
    pub mod STD {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Local priority. This 3-bit fieldis only used when LPRIO_EN bit is set in MCR and it only makes sense for Tx buffers. These bits are not transmitted. They are appended to the regular ID to define the transmission priority."]
    pub mod PRIO {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 0 WORD_64B Register"]
pub mod MB0_64B_WORD12_M {
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_51 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_50 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_49 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_48 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 0 WORD_64B Register"]
pub mod MB0_64B_WORD13_M {
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_55 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_54 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_53 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_52 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 36 CS Register"]
pub mod CS36 {
    #[doc = "Free-Running Counter Time stamp. This 16-bit field is a copy of the Free-Running Timer, captured for Tx and Rx frames at the time when the beginning of the Identifier field appears on the CAN bus."]
    pub mod TIME_STAMP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Length of the data to be stored/transmitted."]
    pub mod DLC {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Remote Transmission Request. One/zero for remote/data frame."]
    pub mod RTR {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ID Extended. One/zero for extended/standard format frame."]
    pub mod IDE {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Substitute Remote Request. Contains a fixed recessive bit."]
    pub mod SRR {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Message Buffer Code. This 4-bit field can be accessed (read or write) by the CPU and by the FlexCAN module itself, as part of the message buffer matching and arbitration process."]
    pub mod CODE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Error State Indicator. This bit indicates if the transmitting node is error active or error passive."]
    pub mod ESI {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Bit Rate Switch. This bit defines whether the bit rate is switched inside a CAN FD format frame."]
    pub mod BRS {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Extended Data Length. This bit distinguishes between CAN format and CAN FD format frames. The EDL bit must not be set for Message Buffers configured to RANSWER with code field 0b1010."]
    pub mod EDL {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 36 ID Register"]
pub mod ID36 {
    #[doc = "Contains extended (LOW word) identifier of message buffer."]
    pub mod EXT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0003_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Contains standard/extended (HIGH word) identifier of message buffer."]
    pub mod STD {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Local priority. This 3-bit fieldis only used when LPRIO_EN bit is set in MCR and it only makes sense for Tx buffers. These bits are not transmitted. They are appended to the regular ID to define the transmission priority."]
    pub mod PRIO {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 1 WORD_32B Register"]
pub mod MB1_32B_WORD6_M {
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_27 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_26 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_25 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_24 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 1 WORD_32B Register"]
pub mod MB1_32B_WORD7_M {
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_31 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_30 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_29 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_28 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 37 CS Register"]
pub mod CS37 {
    #[doc = "Free-Running Counter Time stamp. This 16-bit field is a copy of the Free-Running Timer, captured for Tx and Rx frames at the time when the beginning of the Identifier field appears on the CAN bus."]
    pub mod TIME_STAMP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Length of the data to be stored/transmitted."]
    pub mod DLC {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Remote Transmission Request. One/zero for remote/data frame."]
    pub mod RTR {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ID Extended. One/zero for extended/standard format frame."]
    pub mod IDE {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Substitute Remote Request. Contains a fixed recessive bit."]
    pub mod SRR {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Message Buffer Code. This 4-bit field can be accessed (read or write) by the CPU and by the FlexCAN module itself, as part of the message buffer matching and arbitration process."]
    pub mod CODE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Error State Indicator. This bit indicates if the transmitting node is error active or error passive."]
    pub mod ESI {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Bit Rate Switch. This bit defines whether the bit rate is switched inside a CAN FD format frame."]
    pub mod BRS {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Extended Data Length. This bit distinguishes between CAN format and CAN FD format frames. The EDL bit must not be set for Message Buffers configured to RANSWER with code field 0b1010."]
    pub mod EDL {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 37 ID Register"]
pub mod ID37 {
    #[doc = "Contains extended (LOW word) identifier of message buffer."]
    pub mod EXT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0003_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Contains standard/extended (HIGH word) identifier of message buffer."]
    pub mod STD {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Local priority. This 3-bit fieldis only used when LPRIO_EN bit is set in MCR and it only makes sense for Tx buffers. These bits are not transmitted. They are appended to the regular ID to define the transmission priority."]
    pub mod PRIO {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 1 WORD_64B Register"]
pub mod MB1_64B_WORD2_M {
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_11 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_10 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_9 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_8 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 1 WORD_64B Register"]
pub mod MB1_64B_WORD3_M {
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_15 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_14 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_13 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_12 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 38 CS Register"]
pub mod CS38 {
    #[doc = "Free-Running Counter Time stamp. This 16-bit field is a copy of the Free-Running Timer, captured for Tx and Rx frames at the time when the beginning of the Identifier field appears on the CAN bus."]
    pub mod TIME_STAMP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Length of the data to be stored/transmitted."]
    pub mod DLC {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Remote Transmission Request. One/zero for remote/data frame."]
    pub mod RTR {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ID Extended. One/zero for extended/standard format frame."]
    pub mod IDE {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Substitute Remote Request. Contains a fixed recessive bit."]
    pub mod SRR {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Message Buffer Code. This 4-bit field can be accessed (read or write) by the CPU and by the FlexCAN module itself, as part of the message buffer matching and arbitration process."]
    pub mod CODE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Error State Indicator. This bit indicates if the transmitting node is error active or error passive."]
    pub mod ESI {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Bit Rate Switch. This bit defines whether the bit rate is switched inside a CAN FD format frame."]
    pub mod BRS {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Extended Data Length. This bit distinguishes between CAN format and CAN FD format frames. The EDL bit must not be set for Message Buffers configured to RANSWER with code field 0b1010."]
    pub mod EDL {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 38 ID Register"]
pub mod ID38 {
    #[doc = "Contains extended (LOW word) identifier of message buffer."]
    pub mod EXT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0003_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Contains standard/extended (HIGH word) identifier of message buffer."]
    pub mod STD {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Local priority. This 3-bit fieldis only used when LPRIO_EN bit is set in MCR and it only makes sense for Tx buffers. These bits are not transmitted. They are appended to the regular ID to define the transmission priority."]
    pub mod PRIO {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 1 WORD_64B Register"]
pub mod MB1_64B_WORD6_M {
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_27 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_26 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_25 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_24 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 1 WORD_64B Register"]
pub mod MB1_64B_WORD7_M {
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_31 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_30 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_29 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_28 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 39 CS Register"]
pub mod CS39 {
    #[doc = "Free-Running Counter Time stamp. This 16-bit field is a copy of the Free-Running Timer, captured for Tx and Rx frames at the time when the beginning of the Identifier field appears on the CAN bus."]
    pub mod TIME_STAMP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Length of the data to be stored/transmitted."]
    pub mod DLC {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Remote Transmission Request. One/zero for remote/data frame."]
    pub mod RTR {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ID Extended. One/zero for extended/standard format frame."]
    pub mod IDE {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Substitute Remote Request. Contains a fixed recessive bit."]
    pub mod SRR {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Message Buffer Code. This 4-bit field can be accessed (read or write) by the CPU and by the FlexCAN module itself, as part of the message buffer matching and arbitration process."]
    pub mod CODE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Error State Indicator. This bit indicates if the transmitting node is error active or error passive."]
    pub mod ESI {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Bit Rate Switch. This bit defines whether the bit rate is switched inside a CAN FD format frame."]
    pub mod BRS {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Extended Data Length. This bit distinguishes between CAN format and CAN FD format frames. The EDL bit must not be set for Message Buffers configured to RANSWER with code field 0b1010."]
    pub mod EDL {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 39 ID Register"]
pub mod ID39 {
    #[doc = "Contains extended (LOW word) identifier of message buffer."]
    pub mod EXT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0003_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Contains standard/extended (HIGH word) identifier of message buffer."]
    pub mod STD {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Local priority. This 3-bit fieldis only used when LPRIO_EN bit is set in MCR and it only makes sense for Tx buffers. These bits are not transmitted. They are appended to the regular ID to define the transmission priority."]
    pub mod PRIO {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 1 WORD_64B Register"]
pub mod MB1_64B_WORD10_M {
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_43 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_42 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_41 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_40 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 1 WORD_64B Register"]
pub mod MB1_64B_WORD11_M {
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_47 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_46 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_45 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_44 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 40 CS Register"]
pub mod CS40 {
    #[doc = "Free-Running Counter Time stamp. This 16-bit field is a copy of the Free-Running Timer, captured for Tx and Rx frames at the time when the beginning of the Identifier field appears on the CAN bus."]
    pub mod TIME_STAMP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Length of the data to be stored/transmitted."]
    pub mod DLC {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Remote Transmission Request. One/zero for remote/data frame."]
    pub mod RTR {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ID Extended. One/zero for extended/standard format frame."]
    pub mod IDE {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Substitute Remote Request. Contains a fixed recessive bit."]
    pub mod SRR {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Message Buffer Code. This 4-bit field can be accessed (read or write) by the CPU and by the FlexCAN module itself, as part of the message buffer matching and arbitration process."]
    pub mod CODE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Error State Indicator. This bit indicates if the transmitting node is error active or error passive."]
    pub mod ESI {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Bit Rate Switch. This bit defines whether the bit rate is switched inside a CAN FD format frame."]
    pub mod BRS {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Extended Data Length. This bit distinguishes between CAN format and CAN FD format frames. The EDL bit must not be set for Message Buffers configured to RANSWER with code field 0b1010."]
    pub mod EDL {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 40 ID Register"]
pub mod ID40 {
    #[doc = "Contains extended (LOW word) identifier of message buffer."]
    pub mod EXT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0003_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Contains standard/extended (HIGH word) identifier of message buffer."]
    pub mod STD {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Local priority. This 3-bit fieldis only used when LPRIO_EN bit is set in MCR and it only makes sense for Tx buffers. These bits are not transmitted. They are appended to the regular ID to define the transmission priority."]
    pub mod PRIO {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 1 WORD_64B Register"]
pub mod MB1_64B_WORD14_M {
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_59 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_58 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_57 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_56 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 1 WORD_64B Register"]
pub mod MB1_64B_WORD15_M {
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_63 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_62 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_61 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_60 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 41 CS Register"]
pub mod CS41 {
    #[doc = "Free-Running Counter Time stamp. This 16-bit field is a copy of the Free-Running Timer, captured for Tx and Rx frames at the time when the beginning of the Identifier field appears on the CAN bus."]
    pub mod TIME_STAMP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Length of the data to be stored/transmitted."]
    pub mod DLC {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Remote Transmission Request. One/zero for remote/data frame."]
    pub mod RTR {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ID Extended. One/zero for extended/standard format frame."]
    pub mod IDE {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Substitute Remote Request. Contains a fixed recessive bit."]
    pub mod SRR {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Message Buffer Code. This 4-bit field can be accessed (read or write) by the CPU and by the FlexCAN module itself, as part of the message buffer matching and arbitration process."]
    pub mod CODE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Error State Indicator. This bit indicates if the transmitting node is error active or error passive."]
    pub mod ESI {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Bit Rate Switch. This bit defines whether the bit rate is switched inside a CAN FD format frame."]
    pub mod BRS {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Extended Data Length. This bit distinguishes between CAN format and CAN FD format frames. The EDL bit must not be set for Message Buffers configured to RANSWER with code field 0b1010."]
    pub mod EDL {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 41 ID Register"]
pub mod ID41 {
    #[doc = "Contains extended (LOW word) identifier of message buffer."]
    pub mod EXT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0003_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Contains standard/extended (HIGH word) identifier of message buffer."]
    pub mod STD {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Local priority. This 3-bit fieldis only used when LPRIO_EN bit is set in MCR and it only makes sense for Tx buffers. These bits are not transmitted. They are appended to the regular ID to define the transmission priority."]
    pub mod PRIO {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 2 WORD_64B Register"]
pub mod MB2_64B_WORD0_M {
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_3 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_2 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_1 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_0 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 2 WORD_64B Register"]
pub mod MB2_64B_WORD1_M {
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_7 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_6 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_5 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_4 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 42 CS Register"]
pub mod CS42 {
    #[doc = "Free-Running Counter Time stamp. This 16-bit field is a copy of the Free-Running Timer, captured for Tx and Rx frames at the time when the beginning of the Identifier field appears on the CAN bus."]
    pub mod TIME_STAMP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Length of the data to be stored/transmitted."]
    pub mod DLC {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Remote Transmission Request. One/zero for remote/data frame."]
    pub mod RTR {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ID Extended. One/zero for extended/standard format frame."]
    pub mod IDE {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Substitute Remote Request. Contains a fixed recessive bit."]
    pub mod SRR {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Message Buffer Code. This 4-bit field can be accessed (read or write) by the CPU and by the FlexCAN module itself, as part of the message buffer matching and arbitration process."]
    pub mod CODE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Error State Indicator. This bit indicates if the transmitting node is error active or error passive."]
    pub mod ESI {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Bit Rate Switch. This bit defines whether the bit rate is switched inside a CAN FD format frame."]
    pub mod BRS {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Extended Data Length. This bit distinguishes between CAN format and CAN FD format frames. The EDL bit must not be set for Message Buffers configured to RANSWER with code field 0b1010."]
    pub mod EDL {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 42 ID Register"]
pub mod ID42 {
    #[doc = "Contains extended (LOW word) identifier of message buffer."]
    pub mod EXT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0003_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Contains standard/extended (HIGH word) identifier of message buffer."]
    pub mod STD {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Local priority. This 3-bit fieldis only used when LPRIO_EN bit is set in MCR and it only makes sense for Tx buffers. These bits are not transmitted. They are appended to the regular ID to define the transmission priority."]
    pub mod PRIO {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 2 WORD_64B Register"]
pub mod MB2_64B_WORD4_M {
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_19 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_18 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_17 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_16 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 2 WORD_64B Register"]
pub mod MB2_64B_WORD5_M {
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_23 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_22 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_21 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_20 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 43 CS Register"]
pub mod CS43 {
    #[doc = "Free-Running Counter Time stamp. This 16-bit field is a copy of the Free-Running Timer, captured for Tx and Rx frames at the time when the beginning of the Identifier field appears on the CAN bus."]
    pub mod TIME_STAMP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Length of the data to be stored/transmitted."]
    pub mod DLC {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Remote Transmission Request. One/zero for remote/data frame."]
    pub mod RTR {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ID Extended. One/zero for extended/standard format frame."]
    pub mod IDE {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Substitute Remote Request. Contains a fixed recessive bit."]
    pub mod SRR {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Message Buffer Code. This 4-bit field can be accessed (read or write) by the CPU and by the FlexCAN module itself, as part of the message buffer matching and arbitration process."]
    pub mod CODE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Error State Indicator. This bit indicates if the transmitting node is error active or error passive."]
    pub mod ESI {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Bit Rate Switch. This bit defines whether the bit rate is switched inside a CAN FD format frame."]
    pub mod BRS {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Extended Data Length. This bit distinguishes between CAN format and CAN FD format frames. The EDL bit must not be set for Message Buffers configured to RANSWER with code field 0b1010."]
    pub mod EDL {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 43 ID Register"]
pub mod ID43 {
    #[doc = "Contains extended (LOW word) identifier of message buffer."]
    pub mod EXT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0003_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Contains standard/extended (HIGH word) identifier of message buffer."]
    pub mod STD {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Local priority. This 3-bit fieldis only used when LPRIO_EN bit is set in MCR and it only makes sense for Tx buffers. These bits are not transmitted. They are appended to the regular ID to define the transmission priority."]
    pub mod PRIO {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 2 WORD_64B Register"]
pub mod MB2_64B_WORD8_M {
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_35 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_34 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_33 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_32 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 2 WORD_64B Register"]
pub mod MB2_64B_WORD9_M {
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_39 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_38 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_37 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_36 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 44 CS Register"]
pub mod CS44 {
    #[doc = "Free-Running Counter Time stamp. This 16-bit field is a copy of the Free-Running Timer, captured for Tx and Rx frames at the time when the beginning of the Identifier field appears on the CAN bus."]
    pub mod TIME_STAMP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Length of the data to be stored/transmitted."]
    pub mod DLC {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Remote Transmission Request. One/zero for remote/data frame."]
    pub mod RTR {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ID Extended. One/zero for extended/standard format frame."]
    pub mod IDE {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Substitute Remote Request. Contains a fixed recessive bit."]
    pub mod SRR {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Message Buffer Code. This 4-bit field can be accessed (read or write) by the CPU and by the FlexCAN module itself, as part of the message buffer matching and arbitration process."]
    pub mod CODE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Error State Indicator. This bit indicates if the transmitting node is error active or error passive."]
    pub mod ESI {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Bit Rate Switch. This bit defines whether the bit rate is switched inside a CAN FD format frame."]
    pub mod BRS {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Extended Data Length. This bit distinguishes between CAN format and CAN FD format frames. The EDL bit must not be set for Message Buffers configured to RANSWER with code field 0b1010."]
    pub mod EDL {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 44 ID Register"]
pub mod ID44 {
    #[doc = "Contains extended (LOW word) identifier of message buffer."]
    pub mod EXT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0003_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Contains standard/extended (HIGH word) identifier of message buffer."]
    pub mod STD {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Local priority. This 3-bit fieldis only used when LPRIO_EN bit is set in MCR and it only makes sense for Tx buffers. These bits are not transmitted. They are appended to the regular ID to define the transmission priority."]
    pub mod PRIO {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 2 WORD_64B Register"]
pub mod MB2_64B_WORD12_M {
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_51 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_50 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_49 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_48 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 2 WORD_64B Register"]
pub mod MB2_64B_WORD13_M {
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_55 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_54 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_53 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_52 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 45 CS Register"]
pub mod CS45 {
    #[doc = "Free-Running Counter Time stamp. This 16-bit field is a copy of the Free-Running Timer, captured for Tx and Rx frames at the time when the beginning of the Identifier field appears on the CAN bus."]
    pub mod TIME_STAMP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Length of the data to be stored/transmitted."]
    pub mod DLC {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Remote Transmission Request. One/zero for remote/data frame."]
    pub mod RTR {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ID Extended. One/zero for extended/standard format frame."]
    pub mod IDE {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Substitute Remote Request. Contains a fixed recessive bit."]
    pub mod SRR {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Message Buffer Code. This 4-bit field can be accessed (read or write) by the CPU and by the FlexCAN module itself, as part of the message buffer matching and arbitration process."]
    pub mod CODE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Error State Indicator. This bit indicates if the transmitting node is error active or error passive."]
    pub mod ESI {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Bit Rate Switch. This bit defines whether the bit rate is switched inside a CAN FD format frame."]
    pub mod BRS {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Extended Data Length. This bit distinguishes between CAN format and CAN FD format frames. The EDL bit must not be set for Message Buffers configured to RANSWER with code field 0b1010."]
    pub mod EDL {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 45 ID Register"]
pub mod ID45 {
    #[doc = "Contains extended (LOW word) identifier of message buffer."]
    pub mod EXT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0003_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Contains standard/extended (HIGH word) identifier of message buffer."]
    pub mod STD {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Local priority. This 3-bit fieldis only used when LPRIO_EN bit is set in MCR and it only makes sense for Tx buffers. These bits are not transmitted. They are appended to the regular ID to define the transmission priority."]
    pub mod PRIO {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 3 CS Register"]
pub mod MB3_64B_CS_M {
    #[doc = "Free-Running Counter Time stamp. This 16-bit field is a copy of the Free-Running Timer, captured for Tx and Rx frames at the time when the beginning of the Identifier field appears on the CAN bus."]
    pub mod TIME_STAMP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Length of the data to be stored/transmitted."]
    pub mod DLC {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Remote Transmission Request. One/zero for remote/data frame."]
    pub mod RTR {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ID Extended. One/zero for extended/standard format frame."]
    pub mod IDE {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Substitute Remote Request. Contains a fixed recessive bit."]
    pub mod SRR {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Message Buffer Code. This 4-bit field can be accessed (read or write) by the CPU and by the FlexCAN module itself, as part of the message buffer matching and arbitration process."]
    pub mod CODE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Error State Indicator. This bit indicates if the transmitting node is error active or error passive."]
    pub mod ESI {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Bit Rate Switch. This bit defines whether the bit rate is switched inside a CAN FD format frame."]
    pub mod BRS {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Extended Data Length. This bit distinguishes between CAN format and CAN FD format frames. The EDL bit must not be set for Message Buffers configured to RANSWER with code field 0b1010."]
    pub mod EDL {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 3 ID Register"]
pub mod MB3_64B_ID_M {
    #[doc = "Contains extended (LOW word) identifier of message buffer."]
    pub mod EXT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0003_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Contains standard/extended (HIGH word) identifier of message buffer."]
    pub mod STD {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Local priority. This 3-bit fieldis only used when LPRIO_EN bit is set in MCR and it only makes sense for Tx buffers. These bits are not transmitted. They are appended to the regular ID to define the transmission priority."]
    pub mod PRIO {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 46 CS Register"]
pub mod CS46 {
    #[doc = "Free-Running Counter Time stamp. This 16-bit field is a copy of the Free-Running Timer, captured for Tx and Rx frames at the time when the beginning of the Identifier field appears on the CAN bus."]
    pub mod TIME_STAMP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Length of the data to be stored/transmitted."]
    pub mod DLC {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Remote Transmission Request. One/zero for remote/data frame."]
    pub mod RTR {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ID Extended. One/zero for extended/standard format frame."]
    pub mod IDE {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Substitute Remote Request. Contains a fixed recessive bit."]
    pub mod SRR {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Message Buffer Code. This 4-bit field can be accessed (read or write) by the CPU and by the FlexCAN module itself, as part of the message buffer matching and arbitration process."]
    pub mod CODE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Error State Indicator. This bit indicates if the transmitting node is error active or error passive."]
    pub mod ESI {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Bit Rate Switch. This bit defines whether the bit rate is switched inside a CAN FD format frame."]
    pub mod BRS {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Extended Data Length. This bit distinguishes between CAN format and CAN FD format frames. The EDL bit must not be set for Message Buffers configured to RANSWER with code field 0b1010."]
    pub mod EDL {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 46 ID Register"]
pub mod ID46 {
    #[doc = "Contains extended (LOW word) identifier of message buffer."]
    pub mod EXT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0003_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Contains standard/extended (HIGH word) identifier of message buffer."]
    pub mod STD {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Local priority. This 3-bit fieldis only used when LPRIO_EN bit is set in MCR and it only makes sense for Tx buffers. These bits are not transmitted. They are appended to the regular ID to define the transmission priority."]
    pub mod PRIO {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 3 WORD_64B Register"]
pub mod MB3_64B_WORD2_M {
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_11 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_10 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_9 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_8 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 3 WORD_64B Register"]
pub mod MB3_64B_WORD3_M {
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_15 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_14 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_13 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_12 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 47 CS Register"]
pub mod CS47 {
    #[doc = "Free-Running Counter Time stamp. This 16-bit field is a copy of the Free-Running Timer, captured for Tx and Rx frames at the time when the beginning of the Identifier field appears on the CAN bus."]
    pub mod TIME_STAMP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Length of the data to be stored/transmitted."]
    pub mod DLC {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Remote Transmission Request. One/zero for remote/data frame."]
    pub mod RTR {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ID Extended. One/zero for extended/standard format frame."]
    pub mod IDE {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Substitute Remote Request. Contains a fixed recessive bit."]
    pub mod SRR {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Message Buffer Code. This 4-bit field can be accessed (read or write) by the CPU and by the FlexCAN module itself, as part of the message buffer matching and arbitration process."]
    pub mod CODE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Error State Indicator. This bit indicates if the transmitting node is error active or error passive."]
    pub mod ESI {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Bit Rate Switch. This bit defines whether the bit rate is switched inside a CAN FD format frame."]
    pub mod BRS {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Extended Data Length. This bit distinguishes between CAN format and CAN FD format frames. The EDL bit must not be set for Message Buffers configured to RANSWER with code field 0b1010."]
    pub mod EDL {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 47 ID Register"]
pub mod ID47 {
    #[doc = "Contains extended (LOW word) identifier of message buffer."]
    pub mod EXT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0003_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Contains standard/extended (HIGH word) identifier of message buffer."]
    pub mod STD {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Local priority. This 3-bit fieldis only used when LPRIO_EN bit is set in MCR and it only makes sense for Tx buffers. These bits are not transmitted. They are appended to the regular ID to define the transmission priority."]
    pub mod PRIO {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 10 WORD_16B Register"]
pub mod MB10_16B_WORD0_M {
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_3 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_2 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_1 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_0 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 10 WORD_16B Register"]
pub mod MB10_16B_WORD1_M {
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_7 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_6 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_5 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_4 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 48 CS Register"]
pub mod CS48 {
    #[doc = "Free-Running Counter Time stamp. This 16-bit field is a copy of the Free-Running Timer, captured for Tx and Rx frames at the time when the beginning of the Identifier field appears on the CAN bus."]
    pub mod TIME_STAMP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Length of the data to be stored/transmitted."]
    pub mod DLC {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Remote Transmission Request. One/zero for remote/data frame."]
    pub mod RTR {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ID Extended. One/zero for extended/standard format frame."]
    pub mod IDE {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Substitute Remote Request. Contains a fixed recessive bit."]
    pub mod SRR {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Message Buffer Code. This 4-bit field can be accessed (read or write) by the CPU and by the FlexCAN module itself, as part of the message buffer matching and arbitration process."]
    pub mod CODE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Error State Indicator. This bit indicates if the transmitting node is error active or error passive."]
    pub mod ESI {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Bit Rate Switch. This bit defines whether the bit rate is switched inside a CAN FD format frame."]
    pub mod BRS {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Extended Data Length. This bit distinguishes between CAN format and CAN FD format frames. The EDL bit must not be set for Message Buffers configured to RANSWER with code field 0b1010."]
    pub mod EDL {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 48 ID Register"]
pub mod ID48 {
    #[doc = "Contains extended (LOW word) identifier of message buffer."]
    pub mod EXT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0003_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Contains standard/extended (HIGH word) identifier of message buffer."]
    pub mod STD {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Local priority. This 3-bit fieldis only used when LPRIO_EN bit is set in MCR and it only makes sense for Tx buffers. These bits are not transmitted. They are appended to the regular ID to define the transmission priority."]
    pub mod PRIO {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 11 CS Register"]
pub mod MB11_16B_CS_M {
    #[doc = "Free-Running Counter Time stamp. This 16-bit field is a copy of the Free-Running Timer, captured for Tx and Rx frames at the time when the beginning of the Identifier field appears on the CAN bus."]
    pub mod TIME_STAMP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Length of the data to be stored/transmitted."]
    pub mod DLC {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Remote Transmission Request. One/zero for remote/data frame."]
    pub mod RTR {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ID Extended. One/zero for extended/standard format frame."]
    pub mod IDE {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Substitute Remote Request. Contains a fixed recessive bit."]
    pub mod SRR {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Message Buffer Code. This 4-bit field can be accessed (read or write) by the CPU and by the FlexCAN module itself, as part of the message buffer matching and arbitration process."]
    pub mod CODE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Error State Indicator. This bit indicates if the transmitting node is error active or error passive."]
    pub mod ESI {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Bit Rate Switch. This bit defines whether the bit rate is switched inside a CAN FD format frame."]
    pub mod BRS {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Extended Data Length. This bit distinguishes between CAN format and CAN FD format frames. The EDL bit must not be set for Message Buffers configured to RANSWER with code field 0b1010."]
    pub mod EDL {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 11 ID Register"]
pub mod MB11_16B_ID_M {
    #[doc = "Contains extended (LOW word) identifier of message buffer."]
    pub mod EXT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0003_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Contains standard/extended (HIGH word) identifier of message buffer."]
    pub mod STD {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Local priority. This 3-bit fieldis only used when LPRIO_EN bit is set in MCR and it only makes sense for Tx buffers. These bits are not transmitted. They are appended to the regular ID to define the transmission priority."]
    pub mod PRIO {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 49 CS Register"]
pub mod CS49 {
    #[doc = "Free-Running Counter Time stamp. This 16-bit field is a copy of the Free-Running Timer, captured for Tx and Rx frames at the time when the beginning of the Identifier field appears on the CAN bus."]
    pub mod TIME_STAMP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Length of the data to be stored/transmitted."]
    pub mod DLC {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Remote Transmission Request. One/zero for remote/data frame."]
    pub mod RTR {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ID Extended. One/zero for extended/standard format frame."]
    pub mod IDE {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Substitute Remote Request. Contains a fixed recessive bit."]
    pub mod SRR {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Message Buffer Code. This 4-bit field can be accessed (read or write) by the CPU and by the FlexCAN module itself, as part of the message buffer matching and arbitration process."]
    pub mod CODE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Error State Indicator. This bit indicates if the transmitting node is error active or error passive."]
    pub mod ESI {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Bit Rate Switch. This bit defines whether the bit rate is switched inside a CAN FD format frame."]
    pub mod BRS {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Extended Data Length. This bit distinguishes between CAN format and CAN FD format frames. The EDL bit must not be set for Message Buffers configured to RANSWER with code field 0b1010."]
    pub mod EDL {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 49 ID Register"]
pub mod ID49 {
    #[doc = "Contains extended (LOW word) identifier of message buffer."]
    pub mod EXT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0003_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Contains standard/extended (HIGH word) identifier of message buffer."]
    pub mod STD {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Local priority. This 3-bit fieldis only used when LPRIO_EN bit is set in MCR and it only makes sense for Tx buffers. These bits are not transmitted. They are appended to the regular ID to define the transmission priority."]
    pub mod PRIO {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 11 WORD_16B Register"]
pub mod MB11_16B_WORD2_M {
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_11 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_10 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_9 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_8 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 11 WORD_16B Register"]
pub mod MB11_16B_WORD3_M {
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_15 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_14 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_13 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_12 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 50 CS Register"]
pub mod CS50 {
    #[doc = "Free-Running Counter Time stamp. This 16-bit field is a copy of the Free-Running Timer, captured for Tx and Rx frames at the time when the beginning of the Identifier field appears on the CAN bus."]
    pub mod TIME_STAMP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Length of the data to be stored/transmitted."]
    pub mod DLC {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Remote Transmission Request. One/zero for remote/data frame."]
    pub mod RTR {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ID Extended. One/zero for extended/standard format frame."]
    pub mod IDE {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Substitute Remote Request. Contains a fixed recessive bit."]
    pub mod SRR {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Message Buffer Code. This 4-bit field can be accessed (read or write) by the CPU and by the FlexCAN module itself, as part of the message buffer matching and arbitration process."]
    pub mod CODE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Error State Indicator. This bit indicates if the transmitting node is error active or error passive."]
    pub mod ESI {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Bit Rate Switch. This bit defines whether the bit rate is switched inside a CAN FD format frame."]
    pub mod BRS {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Extended Data Length. This bit distinguishes between CAN format and CAN FD format frames. The EDL bit must not be set for Message Buffers configured to RANSWER with code field 0b1010."]
    pub mod EDL {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 50 ID Register"]
pub mod ID50 {
    #[doc = "Contains extended (LOW word) identifier of message buffer."]
    pub mod EXT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0003_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Contains standard/extended (HIGH word) identifier of message buffer."]
    pub mod STD {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Local priority. This 3-bit fieldis only used when LPRIO_EN bit is set in MCR and it only makes sense for Tx buffers. These bits are not transmitted. They are appended to the regular ID to define the transmission priority."]
    pub mod PRIO {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 12 WORD_16B Register"]
pub mod MB12_16B_WORD0_M {
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_3 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_2 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_1 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_0 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 12 WORD_16B Register"]
pub mod MB12_16B_WORD1_M {
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_7 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_6 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_5 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_4 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 51 CS Register"]
pub mod CS51 {
    #[doc = "Free-Running Counter Time stamp. This 16-bit field is a copy of the Free-Running Timer, captured for Tx and Rx frames at the time when the beginning of the Identifier field appears on the CAN bus."]
    pub mod TIME_STAMP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Length of the data to be stored/transmitted."]
    pub mod DLC {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Remote Transmission Request. One/zero for remote/data frame."]
    pub mod RTR {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ID Extended. One/zero for extended/standard format frame."]
    pub mod IDE {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Substitute Remote Request. Contains a fixed recessive bit."]
    pub mod SRR {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Message Buffer Code. This 4-bit field can be accessed (read or write) by the CPU and by the FlexCAN module itself, as part of the message buffer matching and arbitration process."]
    pub mod CODE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Error State Indicator. This bit indicates if the transmitting node is error active or error passive."]
    pub mod ESI {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Bit Rate Switch. This bit defines whether the bit rate is switched inside a CAN FD format frame."]
    pub mod BRS {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Extended Data Length. This bit distinguishes between CAN format and CAN FD format frames. The EDL bit must not be set for Message Buffers configured to RANSWER with code field 0b1010."]
    pub mod EDL {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 51 ID Register"]
pub mod ID51 {
    #[doc = "Contains extended (LOW word) identifier of message buffer."]
    pub mod EXT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0003_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Contains standard/extended (HIGH word) identifier of message buffer."]
    pub mod STD {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Local priority. This 3-bit fieldis only used when LPRIO_EN bit is set in MCR and it only makes sense for Tx buffers. These bits are not transmitted. They are appended to the regular ID to define the transmission priority."]
    pub mod PRIO {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 13 CS Register"]
pub mod MB13_16B_CS_M {
    #[doc = "Free-Running Counter Time stamp. This 16-bit field is a copy of the Free-Running Timer, captured for Tx and Rx frames at the time when the beginning of the Identifier field appears on the CAN bus."]
    pub mod TIME_STAMP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Length of the data to be stored/transmitted."]
    pub mod DLC {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Remote Transmission Request. One/zero for remote/data frame."]
    pub mod RTR {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ID Extended. One/zero for extended/standard format frame."]
    pub mod IDE {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Substitute Remote Request. Contains a fixed recessive bit."]
    pub mod SRR {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Message Buffer Code. This 4-bit field can be accessed (read or write) by the CPU and by the FlexCAN module itself, as part of the message buffer matching and arbitration process."]
    pub mod CODE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Error State Indicator. This bit indicates if the transmitting node is error active or error passive."]
    pub mod ESI {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Bit Rate Switch. This bit defines whether the bit rate is switched inside a CAN FD format frame."]
    pub mod BRS {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Extended Data Length. This bit distinguishes between CAN format and CAN FD format frames. The EDL bit must not be set for Message Buffers configured to RANSWER with code field 0b1010."]
    pub mod EDL {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 13 ID Register"]
pub mod MB13_16B_ID_M {
    #[doc = "Contains extended (LOW word) identifier of message buffer."]
    pub mod EXT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0003_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Contains standard/extended (HIGH word) identifier of message buffer."]
    pub mod STD {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Local priority. This 3-bit fieldis only used when LPRIO_EN bit is set in MCR and it only makes sense for Tx buffers. These bits are not transmitted. They are appended to the regular ID to define the transmission priority."]
    pub mod PRIO {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 52 CS Register"]
pub mod CS52 {
    #[doc = "Free-Running Counter Time stamp. This 16-bit field is a copy of the Free-Running Timer, captured for Tx and Rx frames at the time when the beginning of the Identifier field appears on the CAN bus."]
    pub mod TIME_STAMP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Length of the data to be stored/transmitted."]
    pub mod DLC {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Remote Transmission Request. One/zero for remote/data frame."]
    pub mod RTR {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ID Extended. One/zero for extended/standard format frame."]
    pub mod IDE {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Substitute Remote Request. Contains a fixed recessive bit."]
    pub mod SRR {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Message Buffer Code. This 4-bit field can be accessed (read or write) by the CPU and by the FlexCAN module itself, as part of the message buffer matching and arbitration process."]
    pub mod CODE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Error State Indicator. This bit indicates if the transmitting node is error active or error passive."]
    pub mod ESI {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Bit Rate Switch. This bit defines whether the bit rate is switched inside a CAN FD format frame."]
    pub mod BRS {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Extended Data Length. This bit distinguishes between CAN format and CAN FD format frames. The EDL bit must not be set for Message Buffers configured to RANSWER with code field 0b1010."]
    pub mod EDL {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 52 ID Register"]
pub mod ID52 {
    #[doc = "Contains extended (LOW word) identifier of message buffer."]
    pub mod EXT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0003_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Contains standard/extended (HIGH word) identifier of message buffer."]
    pub mod STD {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Local priority. This 3-bit fieldis only used when LPRIO_EN bit is set in MCR and it only makes sense for Tx buffers. These bits are not transmitted. They are appended to the regular ID to define the transmission priority."]
    pub mod PRIO {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 13 WORD_16B Register"]
pub mod MB13_16B_WORD2_M {
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_11 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_10 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_9 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_8 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 13 WORD_16B Register"]
pub mod MB13_16B_WORD3_M {
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_15 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_14 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_13 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_12 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 53 CS Register"]
pub mod CS53 {
    #[doc = "Free-Running Counter Time stamp. This 16-bit field is a copy of the Free-Running Timer, captured for Tx and Rx frames at the time when the beginning of the Identifier field appears on the CAN bus."]
    pub mod TIME_STAMP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Length of the data to be stored/transmitted."]
    pub mod DLC {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Remote Transmission Request. One/zero for remote/data frame."]
    pub mod RTR {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ID Extended. One/zero for extended/standard format frame."]
    pub mod IDE {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Substitute Remote Request. Contains a fixed recessive bit."]
    pub mod SRR {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Message Buffer Code. This 4-bit field can be accessed (read or write) by the CPU and by the FlexCAN module itself, as part of the message buffer matching and arbitration process."]
    pub mod CODE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Error State Indicator. This bit indicates if the transmitting node is error active or error passive."]
    pub mod ESI {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Bit Rate Switch. This bit defines whether the bit rate is switched inside a CAN FD format frame."]
    pub mod BRS {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Extended Data Length. This bit distinguishes between CAN format and CAN FD format frames. The EDL bit must not be set for Message Buffers configured to RANSWER with code field 0b1010."]
    pub mod EDL {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 53 ID Register"]
pub mod ID53 {
    #[doc = "Contains extended (LOW word) identifier of message buffer."]
    pub mod EXT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0003_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Contains standard/extended (HIGH word) identifier of message buffer."]
    pub mod STD {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Local priority. This 3-bit fieldis only used when LPRIO_EN bit is set in MCR and it only makes sense for Tx buffers. These bits are not transmitted. They are appended to the regular ID to define the transmission priority."]
    pub mod PRIO {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 14 WORD_16B Register"]
pub mod MB14_16B_WORD0_M {
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_3 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_2 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_1 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_0 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 14 WORD_16B Register"]
pub mod MB14_16B_WORD1_M {
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_7 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_6 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_5 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_4 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 54 CS Register"]
pub mod CS54 {
    #[doc = "Free-Running Counter Time stamp. This 16-bit field is a copy of the Free-Running Timer, captured for Tx and Rx frames at the time when the beginning of the Identifier field appears on the CAN bus."]
    pub mod TIME_STAMP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Length of the data to be stored/transmitted."]
    pub mod DLC {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Remote Transmission Request. One/zero for remote/data frame."]
    pub mod RTR {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ID Extended. One/zero for extended/standard format frame."]
    pub mod IDE {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Substitute Remote Request. Contains a fixed recessive bit."]
    pub mod SRR {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Message Buffer Code. This 4-bit field can be accessed (read or write) by the CPU and by the FlexCAN module itself, as part of the message buffer matching and arbitration process."]
    pub mod CODE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Error State Indicator. This bit indicates if the transmitting node is error active or error passive."]
    pub mod ESI {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Bit Rate Switch. This bit defines whether the bit rate is switched inside a CAN FD format frame."]
    pub mod BRS {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Extended Data Length. This bit distinguishes between CAN format and CAN FD format frames. The EDL bit must not be set for Message Buffers configured to RANSWER with code field 0b1010."]
    pub mod EDL {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 54 ID Register"]
pub mod ID54 {
    #[doc = "Contains extended (LOW word) identifier of message buffer."]
    pub mod EXT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0003_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Contains standard/extended (HIGH word) identifier of message buffer."]
    pub mod STD {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Local priority. This 3-bit fieldis only used when LPRIO_EN bit is set in MCR and it only makes sense for Tx buffers. These bits are not transmitted. They are appended to the regular ID to define the transmission priority."]
    pub mod PRIO {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 15 CS Register"]
pub mod MB15_16B_CS_M {
    #[doc = "Free-Running Counter Time stamp. This 16-bit field is a copy of the Free-Running Timer, captured for Tx and Rx frames at the time when the beginning of the Identifier field appears on the CAN bus."]
    pub mod TIME_STAMP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Length of the data to be stored/transmitted."]
    pub mod DLC {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Remote Transmission Request. One/zero for remote/data frame."]
    pub mod RTR {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ID Extended. One/zero for extended/standard format frame."]
    pub mod IDE {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Substitute Remote Request. Contains a fixed recessive bit."]
    pub mod SRR {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Message Buffer Code. This 4-bit field can be accessed (read or write) by the CPU and by the FlexCAN module itself, as part of the message buffer matching and arbitration process."]
    pub mod CODE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Error State Indicator. This bit indicates if the transmitting node is error active or error passive."]
    pub mod ESI {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Bit Rate Switch. This bit defines whether the bit rate is switched inside a CAN FD format frame."]
    pub mod BRS {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Extended Data Length. This bit distinguishes between CAN format and CAN FD format frames. The EDL bit must not be set for Message Buffers configured to RANSWER with code field 0b1010."]
    pub mod EDL {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 15 ID Register"]
pub mod MB15_16B_ID_M {
    #[doc = "Contains extended (LOW word) identifier of message buffer."]
    pub mod EXT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0003_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Contains standard/extended (HIGH word) identifier of message buffer."]
    pub mod STD {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Local priority. This 3-bit fieldis only used when LPRIO_EN bit is set in MCR and it only makes sense for Tx buffers. These bits are not transmitted. They are appended to the regular ID to define the transmission priority."]
    pub mod PRIO {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 55 CS Register"]
pub mod CS55 {
    #[doc = "Free-Running Counter Time stamp. This 16-bit field is a copy of the Free-Running Timer, captured for Tx and Rx frames at the time when the beginning of the Identifier field appears on the CAN bus."]
    pub mod TIME_STAMP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Length of the data to be stored/transmitted."]
    pub mod DLC {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Remote Transmission Request. One/zero for remote/data frame."]
    pub mod RTR {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ID Extended. One/zero for extended/standard format frame."]
    pub mod IDE {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Substitute Remote Request. Contains a fixed recessive bit."]
    pub mod SRR {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Message Buffer Code. This 4-bit field can be accessed (read or write) by the CPU and by the FlexCAN module itself, as part of the message buffer matching and arbitration process."]
    pub mod CODE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Error State Indicator. This bit indicates if the transmitting node is error active or error passive."]
    pub mod ESI {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Bit Rate Switch. This bit defines whether the bit rate is switched inside a CAN FD format frame."]
    pub mod BRS {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Extended Data Length. This bit distinguishes between CAN format and CAN FD format frames. The EDL bit must not be set for Message Buffers configured to RANSWER with code field 0b1010."]
    pub mod EDL {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 55 ID Register"]
pub mod ID55 {
    #[doc = "Contains extended (LOW word) identifier of message buffer."]
    pub mod EXT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0003_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Contains standard/extended (HIGH word) identifier of message buffer."]
    pub mod STD {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Local priority. This 3-bit fieldis only used when LPRIO_EN bit is set in MCR and it only makes sense for Tx buffers. These bits are not transmitted. They are appended to the regular ID to define the transmission priority."]
    pub mod PRIO {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 15 WORD_16B Register"]
pub mod MB15_16B_WORD2_M {
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_11 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_10 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_9 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_8 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 15 WORD_16B Register"]
pub mod MB15_16B_WORD3_M {
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_15 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_14 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_13 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_12 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 56 CS Register"]
pub mod CS56 {
    #[doc = "Free-Running Counter Time stamp. This 16-bit field is a copy of the Free-Running Timer, captured for Tx and Rx frames at the time when the beginning of the Identifier field appears on the CAN bus."]
    pub mod TIME_STAMP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Length of the data to be stored/transmitted."]
    pub mod DLC {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Remote Transmission Request. One/zero for remote/data frame."]
    pub mod RTR {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ID Extended. One/zero for extended/standard format frame."]
    pub mod IDE {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Substitute Remote Request. Contains a fixed recessive bit."]
    pub mod SRR {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Message Buffer Code. This 4-bit field can be accessed (read or write) by the CPU and by the FlexCAN module itself, as part of the message buffer matching and arbitration process."]
    pub mod CODE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Error State Indicator. This bit indicates if the transmitting node is error active or error passive."]
    pub mod ESI {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Bit Rate Switch. This bit defines whether the bit rate is switched inside a CAN FD format frame."]
    pub mod BRS {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Extended Data Length. This bit distinguishes between CAN format and CAN FD format frames. The EDL bit must not be set for Message Buffers configured to RANSWER with code field 0b1010."]
    pub mod EDL {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 56 ID Register"]
pub mod ID56 {
    #[doc = "Contains extended (LOW word) identifier of message buffer."]
    pub mod EXT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0003_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Contains standard/extended (HIGH word) identifier of message buffer."]
    pub mod STD {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Local priority. This 3-bit fieldis only used when LPRIO_EN bit is set in MCR and it only makes sense for Tx buffers. These bits are not transmitted. They are appended to the regular ID to define the transmission priority."]
    pub mod PRIO {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 16 WORD_16B Register"]
pub mod MB16_16B_WORD0_M {
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_3 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_2 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_1 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_0 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 16 WORD_16B Register"]
pub mod MB16_16B_WORD1_M {
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_7 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_6 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_5 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_4 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 57 CS Register"]
pub mod CS57 {
    #[doc = "Free-Running Counter Time stamp. This 16-bit field is a copy of the Free-Running Timer, captured for Tx and Rx frames at the time when the beginning of the Identifier field appears on the CAN bus."]
    pub mod TIME_STAMP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Length of the data to be stored/transmitted."]
    pub mod DLC {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Remote Transmission Request. One/zero for remote/data frame."]
    pub mod RTR {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ID Extended. One/zero for extended/standard format frame."]
    pub mod IDE {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Substitute Remote Request. Contains a fixed recessive bit."]
    pub mod SRR {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Message Buffer Code. This 4-bit field can be accessed (read or write) by the CPU and by the FlexCAN module itself, as part of the message buffer matching and arbitration process."]
    pub mod CODE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Error State Indicator. This bit indicates if the transmitting node is error active or error passive."]
    pub mod ESI {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Bit Rate Switch. This bit defines whether the bit rate is switched inside a CAN FD format frame."]
    pub mod BRS {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Extended Data Length. This bit distinguishes between CAN format and CAN FD format frames. The EDL bit must not be set for Message Buffers configured to RANSWER with code field 0b1010."]
    pub mod EDL {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 57 ID Register"]
pub mod ID57 {
    #[doc = "Contains extended (LOW word) identifier of message buffer."]
    pub mod EXT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0003_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Contains standard/extended (HIGH word) identifier of message buffer."]
    pub mod STD {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Local priority. This 3-bit fieldis only used when LPRIO_EN bit is set in MCR and it only makes sense for Tx buffers. These bits are not transmitted. They are appended to the regular ID to define the transmission priority."]
    pub mod PRIO {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 10 WORD_32B Register"]
pub mod MB10_32B_WORD0_M {
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_3 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_2 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_1 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_0 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 10 WORD_32B Register"]
pub mod MB10_32B_WORD1_M {
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_7 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_6 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_5 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_4 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 58 CS Register"]
pub mod CS58 {
    #[doc = "Free-Running Counter Time stamp. This 16-bit field is a copy of the Free-Running Timer, captured for Tx and Rx frames at the time when the beginning of the Identifier field appears on the CAN bus."]
    pub mod TIME_STAMP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Length of the data to be stored/transmitted."]
    pub mod DLC {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Remote Transmission Request. One/zero for remote/data frame."]
    pub mod RTR {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ID Extended. One/zero for extended/standard format frame."]
    pub mod IDE {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Substitute Remote Request. Contains a fixed recessive bit."]
    pub mod SRR {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Message Buffer Code. This 4-bit field can be accessed (read or write) by the CPU and by the FlexCAN module itself, as part of the message buffer matching and arbitration process."]
    pub mod CODE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Error State Indicator. This bit indicates if the transmitting node is error active or error passive."]
    pub mod ESI {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Bit Rate Switch. This bit defines whether the bit rate is switched inside a CAN FD format frame."]
    pub mod BRS {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Extended Data Length. This bit distinguishes between CAN format and CAN FD format frames. The EDL bit must not be set for Message Buffers configured to RANSWER with code field 0b1010."]
    pub mod EDL {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 58 ID Register"]
pub mod ID58 {
    #[doc = "Contains extended (LOW word) identifier of message buffer."]
    pub mod EXT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0003_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Contains standard/extended (HIGH word) identifier of message buffer."]
    pub mod STD {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Local priority. This 3-bit fieldis only used when LPRIO_EN bit is set in MCR and it only makes sense for Tx buffers. These bits are not transmitted. They are appended to the regular ID to define the transmission priority."]
    pub mod PRIO {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 10 WORD_32B Register"]
pub mod MB10_32B_WORD4_M {
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_19 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_18 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_17 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_16 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 10 WORD_32B Register"]
pub mod MB10_32B_WORD5_M {
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_23 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_22 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_21 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_20 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 59 CS Register"]
pub mod CS59 {
    #[doc = "Free-Running Counter Time stamp. This 16-bit field is a copy of the Free-Running Timer, captured for Tx and Rx frames at the time when the beginning of the Identifier field appears on the CAN bus."]
    pub mod TIME_STAMP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Length of the data to be stored/transmitted."]
    pub mod DLC {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Remote Transmission Request. One/zero for remote/data frame."]
    pub mod RTR {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ID Extended. One/zero for extended/standard format frame."]
    pub mod IDE {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Substitute Remote Request. Contains a fixed recessive bit."]
    pub mod SRR {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Message Buffer Code. This 4-bit field can be accessed (read or write) by the CPU and by the FlexCAN module itself, as part of the message buffer matching and arbitration process."]
    pub mod CODE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Error State Indicator. This bit indicates if the transmitting node is error active or error passive."]
    pub mod ESI {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Bit Rate Switch. This bit defines whether the bit rate is switched inside a CAN FD format frame."]
    pub mod BRS {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Extended Data Length. This bit distinguishes between CAN format and CAN FD format frames. The EDL bit must not be set for Message Buffers configured to RANSWER with code field 0b1010."]
    pub mod EDL {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 59 ID Register"]
pub mod ID59 {
    #[doc = "Contains extended (LOW word) identifier of message buffer."]
    pub mod EXT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0003_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Contains standard/extended (HIGH word) identifier of message buffer."]
    pub mod STD {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Local priority. This 3-bit fieldis only used when LPRIO_EN bit is set in MCR and it only makes sense for Tx buffers. These bits are not transmitted. They are appended to the regular ID to define the transmission priority."]
    pub mod PRIO {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 11 CS Register"]
pub mod MB11_32B_CS_M {
    #[doc = "Free-Running Counter Time stamp. This 16-bit field is a copy of the Free-Running Timer, captured for Tx and Rx frames at the time when the beginning of the Identifier field appears on the CAN bus."]
    pub mod TIME_STAMP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Length of the data to be stored/transmitted."]
    pub mod DLC {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Remote Transmission Request. One/zero for remote/data frame."]
    pub mod RTR {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ID Extended. One/zero for extended/standard format frame."]
    pub mod IDE {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Substitute Remote Request. Contains a fixed recessive bit."]
    pub mod SRR {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Message Buffer Code. This 4-bit field can be accessed (read or write) by the CPU and by the FlexCAN module itself, as part of the message buffer matching and arbitration process."]
    pub mod CODE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Error State Indicator. This bit indicates if the transmitting node is error active or error passive."]
    pub mod ESI {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Bit Rate Switch. This bit defines whether the bit rate is switched inside a CAN FD format frame."]
    pub mod BRS {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Extended Data Length. This bit distinguishes between CAN format and CAN FD format frames. The EDL bit must not be set for Message Buffers configured to RANSWER with code field 0b1010."]
    pub mod EDL {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 11 ID Register"]
pub mod MB11_32B_ID_M {
    #[doc = "Contains extended (LOW word) identifier of message buffer."]
    pub mod EXT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0003_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Contains standard/extended (HIGH word) identifier of message buffer."]
    pub mod STD {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Local priority. This 3-bit fieldis only used when LPRIO_EN bit is set in MCR and it only makes sense for Tx buffers. These bits are not transmitted. They are appended to the regular ID to define the transmission priority."]
    pub mod PRIO {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 60 CS Register"]
pub mod CS60 {
    #[doc = "Free-Running Counter Time stamp. This 16-bit field is a copy of the Free-Running Timer, captured for Tx and Rx frames at the time when the beginning of the Identifier field appears on the CAN bus."]
    pub mod TIME_STAMP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Length of the data to be stored/transmitted."]
    pub mod DLC {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Remote Transmission Request. One/zero for remote/data frame."]
    pub mod RTR {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ID Extended. One/zero for extended/standard format frame."]
    pub mod IDE {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Substitute Remote Request. Contains a fixed recessive bit."]
    pub mod SRR {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Message Buffer Code. This 4-bit field can be accessed (read or write) by the CPU and by the FlexCAN module itself, as part of the message buffer matching and arbitration process."]
    pub mod CODE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Error State Indicator. This bit indicates if the transmitting node is error active or error passive."]
    pub mod ESI {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Bit Rate Switch. This bit defines whether the bit rate is switched inside a CAN FD format frame."]
    pub mod BRS {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Extended Data Length. This bit distinguishes between CAN format and CAN FD format frames. The EDL bit must not be set for Message Buffers configured to RANSWER with code field 0b1010."]
    pub mod EDL {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 60 ID Register"]
pub mod ID60 {
    #[doc = "Contains extended (LOW word) identifier of message buffer."]
    pub mod EXT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0003_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Contains standard/extended (HIGH word) identifier of message buffer."]
    pub mod STD {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Local priority. This 3-bit fieldis only used when LPRIO_EN bit is set in MCR and it only makes sense for Tx buffers. These bits are not transmitted. They are appended to the regular ID to define the transmission priority."]
    pub mod PRIO {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 11 WORD_32B Register"]
pub mod MB11_32B_WORD2_M {
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_11 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_10 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_9 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_8 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 11 WORD_32B Register"]
pub mod MB11_32B_WORD3_M {
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_15 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_14 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_13 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_12 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 61 CS Register"]
pub mod CS61 {
    #[doc = "Free-Running Counter Time stamp. This 16-bit field is a copy of the Free-Running Timer, captured for Tx and Rx frames at the time when the beginning of the Identifier field appears on the CAN bus."]
    pub mod TIME_STAMP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Length of the data to be stored/transmitted."]
    pub mod DLC {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Remote Transmission Request. One/zero for remote/data frame."]
    pub mod RTR {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ID Extended. One/zero for extended/standard format frame."]
    pub mod IDE {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Substitute Remote Request. Contains a fixed recessive bit."]
    pub mod SRR {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Message Buffer Code. This 4-bit field can be accessed (read or write) by the CPU and by the FlexCAN module itself, as part of the message buffer matching and arbitration process."]
    pub mod CODE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Error State Indicator. This bit indicates if the transmitting node is error active or error passive."]
    pub mod ESI {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Bit Rate Switch. This bit defines whether the bit rate is switched inside a CAN FD format frame."]
    pub mod BRS {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Extended Data Length. This bit distinguishes between CAN format and CAN FD format frames. The EDL bit must not be set for Message Buffers configured to RANSWER with code field 0b1010."]
    pub mod EDL {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 61 ID Register"]
pub mod ID61 {
    #[doc = "Contains extended (LOW word) identifier of message buffer."]
    pub mod EXT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0003_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Contains standard/extended (HIGH word) identifier of message buffer."]
    pub mod STD {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Local priority. This 3-bit fieldis only used when LPRIO_EN bit is set in MCR and it only makes sense for Tx buffers. These bits are not transmitted. They are appended to the regular ID to define the transmission priority."]
    pub mod PRIO {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 11 WORD_32B Register"]
pub mod MB11_32B_WORD6_M {
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_27 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_26 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_25 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_24 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 11 WORD_32B Register"]
pub mod MB11_32B_WORD7_M {
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_31 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_30 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_29 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_28 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 62 CS Register"]
pub mod CS62 {
    #[doc = "Free-Running Counter Time stamp. This 16-bit field is a copy of the Free-Running Timer, captured for Tx and Rx frames at the time when the beginning of the Identifier field appears on the CAN bus."]
    pub mod TIME_STAMP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Length of the data to be stored/transmitted."]
    pub mod DLC {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Remote Transmission Request. One/zero for remote/data frame."]
    pub mod RTR {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ID Extended. One/zero for extended/standard format frame."]
    pub mod IDE {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Substitute Remote Request. Contains a fixed recessive bit."]
    pub mod SRR {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Message Buffer Code. This 4-bit field can be accessed (read or write) by the CPU and by the FlexCAN module itself, as part of the message buffer matching and arbitration process."]
    pub mod CODE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Error State Indicator. This bit indicates if the transmitting node is error active or error passive."]
    pub mod ESI {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Bit Rate Switch. This bit defines whether the bit rate is switched inside a CAN FD format frame."]
    pub mod BRS {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Extended Data Length. This bit distinguishes between CAN format and CAN FD format frames. The EDL bit must not be set for Message Buffers configured to RANSWER with code field 0b1010."]
    pub mod EDL {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 62 ID Register"]
pub mod ID62 {
    #[doc = "Contains extended (LOW word) identifier of message buffer."]
    pub mod EXT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0003_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Contains standard/extended (HIGH word) identifier of message buffer."]
    pub mod STD {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Local priority. This 3-bit fieldis only used when LPRIO_EN bit is set in MCR and it only makes sense for Tx buffers. These bits are not transmitted. They are appended to the regular ID to define the transmission priority."]
    pub mod PRIO {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 20 WORD_16B Register"]
pub mod MB20_16B_WORD0_M {
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_3 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_2 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_1 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_0 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 20 WORD_16B Register"]
pub mod MB20_16B_WORD1_M {
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_7 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_6 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_5 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_4 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 63 CS Register"]
pub mod CS63 {
    #[doc = "Free-Running Counter Time stamp. This 16-bit field is a copy of the Free-Running Timer, captured for Tx and Rx frames at the time when the beginning of the Identifier field appears on the CAN bus."]
    pub mod TIME_STAMP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Length of the data to be stored/transmitted."]
    pub mod DLC {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Remote Transmission Request. One/zero for remote/data frame."]
    pub mod RTR {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ID Extended. One/zero for extended/standard format frame."]
    pub mod IDE {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Substitute Remote Request. Contains a fixed recessive bit."]
    pub mod SRR {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Message Buffer Code. This 4-bit field can be accessed (read or write) by the CPU and by the FlexCAN module itself, as part of the message buffer matching and arbitration process."]
    pub mod CODE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Error State Indicator. This bit indicates if the transmitting node is error active or error passive."]
    pub mod ESI {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Bit Rate Switch. This bit defines whether the bit rate is switched inside a CAN FD format frame."]
    pub mod BRS {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Extended Data Length. This bit distinguishes between CAN format and CAN FD format frames. The EDL bit must not be set for Message Buffers configured to RANSWER with code field 0b1010."]
    pub mod EDL {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 63 ID Register"]
pub mod ID63 {
    #[doc = "Contains extended (LOW word) identifier of message buffer."]
    pub mod EXT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0003_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Contains standard/extended (HIGH word) identifier of message buffer."]
    pub mod STD {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Local priority. This 3-bit fieldis only used when LPRIO_EN bit is set in MCR and it only makes sense for Tx buffers. These bits are not transmitted. They are appended to the regular ID to define the transmission priority."]
    pub mod PRIO {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 63 WORD_8B Register"]
pub mod MB63_8B_WORD0 {
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_3 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_2 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_1 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_0 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 63 WORD_8B Register"]
pub mod MB63_8B_WORD1 {
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_7 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_6 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_5 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_4 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 64 CS Register"]
pub mod CS64 {
    #[doc = "Free-Running Counter Time stamp. This 16-bit field is a copy of the Free-Running Timer, captured for Tx and Rx frames at the time when the beginning of the Identifier field appears on the CAN bus."]
    pub mod TIME_STAMP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Length of the data to be stored/transmitted."]
    pub mod DLC {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Remote Transmission Request. One/zero for remote/data frame."]
    pub mod RTR {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ID Extended. One/zero for extended/standard format frame."]
    pub mod IDE {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Substitute Remote Request. Contains a fixed recessive bit."]
    pub mod SRR {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Message Buffer Code. This 4-bit field can be accessed (read or write) by the CPU and by the FlexCAN module itself, as part of the message buffer matching and arbitration process."]
    pub mod CODE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Error State Indicator. This bit indicates if the transmitting node is error active or error passive."]
    pub mod ESI {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Bit Rate Switch. This bit defines whether the bit rate is switched inside a CAN FD format frame."]
    pub mod BRS {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Extended Data Length. This bit distinguishes between CAN format and CAN FD format frames. The EDL bit must not be set for Message Buffers configured to RANSWER with code field 0b1010."]
    pub mod EDL {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 64 ID Register"]
pub mod ID64 {
    #[doc = "Contains extended (LOW word) identifier of message buffer."]
    pub mod EXT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0003_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Contains standard/extended (HIGH word) identifier of message buffer."]
    pub mod STD {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Local priority. This 3-bit fieldis only used when LPRIO_EN bit is set in MCR and it only makes sense for Tx buffers. These bits are not transmitted. They are appended to the regular ID to define the transmission priority."]
    pub mod PRIO {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 0 WORD_16B Register"]
pub mod MB0_16B_WORD0_H {
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_3 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_2 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_1 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_0 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 0 WORD_16B Register"]
pub mod MB0_16B_WORD1_H {
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_7 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_6 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_5 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_4 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 65 CS Register"]
pub mod CS65 {
    #[doc = "Free-Running Counter Time stamp. This 16-bit field is a copy of the Free-Running Timer, captured for Tx and Rx frames at the time when the beginning of the Identifier field appears on the CAN bus."]
    pub mod TIME_STAMP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Length of the data to be stored/transmitted."]
    pub mod DLC {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Remote Transmission Request. One/zero for remote/data frame."]
    pub mod RTR {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ID Extended. One/zero for extended/standard format frame."]
    pub mod IDE {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Substitute Remote Request. Contains a fixed recessive bit."]
    pub mod SRR {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Message Buffer Code. This 4-bit field can be accessed (read or write) by the CPU and by the FlexCAN module itself, as part of the message buffer matching and arbitration process."]
    pub mod CODE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Error State Indicator. This bit indicates if the transmitting node is error active or error passive."]
    pub mod ESI {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Bit Rate Switch. This bit defines whether the bit rate is switched inside a CAN FD format frame."]
    pub mod BRS {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Extended Data Length. This bit distinguishes between CAN format and CAN FD format frames. The EDL bit must not be set for Message Buffers configured to RANSWER with code field 0b1010."]
    pub mod EDL {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 65 ID Register"]
pub mod ID65 {
    #[doc = "Contains extended (LOW word) identifier of message buffer."]
    pub mod EXT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0003_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Contains standard/extended (HIGH word) identifier of message buffer."]
    pub mod STD {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Local priority. This 3-bit fieldis only used when LPRIO_EN bit is set in MCR and it only makes sense for Tx buffers. These bits are not transmitted. They are appended to the regular ID to define the transmission priority."]
    pub mod PRIO {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 0 WORD_32B Register"]
pub mod MB0_32B_WORD4_H {
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_19 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_18 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_17 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_16 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 0 WORD_32B Register"]
pub mod MB0_32B_WORD5_H {
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_23 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_22 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_21 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_20 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 66 CS Register"]
pub mod CS66 {
    #[doc = "Free-Running Counter Time stamp. This 16-bit field is a copy of the Free-Running Timer, captured for Tx and Rx frames at the time when the beginning of the Identifier field appears on the CAN bus."]
    pub mod TIME_STAMP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Length of the data to be stored/transmitted."]
    pub mod DLC {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Remote Transmission Request. One/zero for remote/data frame."]
    pub mod RTR {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ID Extended. One/zero for extended/standard format frame."]
    pub mod IDE {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Substitute Remote Request. Contains a fixed recessive bit."]
    pub mod SRR {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Message Buffer Code. This 4-bit field can be accessed (read or write) by the CPU and by the FlexCAN module itself, as part of the message buffer matching and arbitration process."]
    pub mod CODE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Error State Indicator. This bit indicates if the transmitting node is error active or error passive."]
    pub mod ESI {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Bit Rate Switch. This bit defines whether the bit rate is switched inside a CAN FD format frame."]
    pub mod BRS {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Extended Data Length. This bit distinguishes between CAN format and CAN FD format frames. The EDL bit must not be set for Message Buffers configured to RANSWER with code field 0b1010."]
    pub mod EDL {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 66 ID Register"]
pub mod ID66 {
    #[doc = "Contains extended (LOW word) identifier of message buffer."]
    pub mod EXT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0003_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Contains standard/extended (HIGH word) identifier of message buffer."]
    pub mod STD {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Local priority. This 3-bit fieldis only used when LPRIO_EN bit is set in MCR and it only makes sense for Tx buffers. These bits are not transmitted. They are appended to the regular ID to define the transmission priority."]
    pub mod PRIO {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 0 WORD_64B Register"]
pub mod MB0_64B_WORD8_H {
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_35 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_34 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_33 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_32 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 0 WORD_64B Register"]
pub mod MB0_64B_WORD9_H {
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_39 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_38 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_37 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_36 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 67 CS Register"]
pub mod CS67 {
    #[doc = "Free-Running Counter Time stamp. This 16-bit field is a copy of the Free-Running Timer, captured for Tx and Rx frames at the time when the beginning of the Identifier field appears on the CAN bus."]
    pub mod TIME_STAMP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Length of the data to be stored/transmitted."]
    pub mod DLC {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Remote Transmission Request. One/zero for remote/data frame."]
    pub mod RTR {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ID Extended. One/zero for extended/standard format frame."]
    pub mod IDE {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Substitute Remote Request. Contains a fixed recessive bit."]
    pub mod SRR {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Message Buffer Code. This 4-bit field can be accessed (read or write) by the CPU and by the FlexCAN module itself, as part of the message buffer matching and arbitration process."]
    pub mod CODE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Error State Indicator. This bit indicates if the transmitting node is error active or error passive."]
    pub mod ESI {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Bit Rate Switch. This bit defines whether the bit rate is switched inside a CAN FD format frame."]
    pub mod BRS {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Extended Data Length. This bit distinguishes between CAN format and CAN FD format frames. The EDL bit must not be set for Message Buffers configured to RANSWER with code field 0b1010."]
    pub mod EDL {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 67 ID Register"]
pub mod ID67 {
    #[doc = "Contains extended (LOW word) identifier of message buffer."]
    pub mod EXT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0003_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Contains standard/extended (HIGH word) identifier of message buffer."]
    pub mod STD {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Local priority. This 3-bit fieldis only used when LPRIO_EN bit is set in MCR and it only makes sense for Tx buffers. These bits are not transmitted. They are appended to the regular ID to define the transmission priority."]
    pub mod PRIO {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 0 WORD_64B Register"]
pub mod MB0_64B_WORD12_H {
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_51 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_50 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_49 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_48 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 0 WORD_64B Register"]
pub mod MB0_64B_WORD13_H {
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_55 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_54 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_53 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_52 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 68 CS Register"]
pub mod CS68 {
    #[doc = "Free-Running Counter Time stamp. This 16-bit field is a copy of the Free-Running Timer, captured for Tx and Rx frames at the time when the beginning of the Identifier field appears on the CAN bus."]
    pub mod TIME_STAMP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Length of the data to be stored/transmitted."]
    pub mod DLC {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Remote Transmission Request. One/zero for remote/data frame."]
    pub mod RTR {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ID Extended. One/zero for extended/standard format frame."]
    pub mod IDE {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Substitute Remote Request. Contains a fixed recessive bit."]
    pub mod SRR {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Message Buffer Code. This 4-bit field can be accessed (read or write) by the CPU and by the FlexCAN module itself, as part of the message buffer matching and arbitration process."]
    pub mod CODE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Error State Indicator. This bit indicates if the transmitting node is error active or error passive."]
    pub mod ESI {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Bit Rate Switch. This bit defines whether the bit rate is switched inside a CAN FD format frame."]
    pub mod BRS {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Extended Data Length. This bit distinguishes between CAN format and CAN FD format frames. The EDL bit must not be set for Message Buffers configured to RANSWER with code field 0b1010."]
    pub mod EDL {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 68 ID Register"]
pub mod ID68 {
    #[doc = "Contains extended (LOW word) identifier of message buffer."]
    pub mod EXT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0003_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Contains standard/extended (HIGH word) identifier of message buffer."]
    pub mod STD {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Local priority. This 3-bit fieldis only used when LPRIO_EN bit is set in MCR and it only makes sense for Tx buffers. These bits are not transmitted. They are appended to the regular ID to define the transmission priority."]
    pub mod PRIO {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 1 WORD_32B Register"]
pub mod MB1_32B_WORD6_H {
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_27 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_26 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_25 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_24 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 1 WORD_32B Register"]
pub mod MB1_32B_WORD7_H {
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_31 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_30 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_29 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_28 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 69 CS Register"]
pub mod CS69 {
    #[doc = "Free-Running Counter Time stamp. This 16-bit field is a copy of the Free-Running Timer, captured for Tx and Rx frames at the time when the beginning of the Identifier field appears on the CAN bus."]
    pub mod TIME_STAMP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Length of the data to be stored/transmitted."]
    pub mod DLC {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Remote Transmission Request. One/zero for remote/data frame."]
    pub mod RTR {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ID Extended. One/zero for extended/standard format frame."]
    pub mod IDE {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Substitute Remote Request. Contains a fixed recessive bit."]
    pub mod SRR {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Message Buffer Code. This 4-bit field can be accessed (read or write) by the CPU and by the FlexCAN module itself, as part of the message buffer matching and arbitration process."]
    pub mod CODE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Error State Indicator. This bit indicates if the transmitting node is error active or error passive."]
    pub mod ESI {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Bit Rate Switch. This bit defines whether the bit rate is switched inside a CAN FD format frame."]
    pub mod BRS {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Extended Data Length. This bit distinguishes between CAN format and CAN FD format frames. The EDL bit must not be set for Message Buffers configured to RANSWER with code field 0b1010."]
    pub mod EDL {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 69 ID Register"]
pub mod ID69 {
    #[doc = "Contains extended (LOW word) identifier of message buffer."]
    pub mod EXT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0003_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Contains standard/extended (HIGH word) identifier of message buffer."]
    pub mod STD {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Local priority. This 3-bit fieldis only used when LPRIO_EN bit is set in MCR and it only makes sense for Tx buffers. These bits are not transmitted. They are appended to the regular ID to define the transmission priority."]
    pub mod PRIO {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 1 WORD_64B Register"]
pub mod MB1_64B_WORD2_H {
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_11 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_10 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_9 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_8 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 1 WORD_64B Register"]
pub mod MB1_64B_WORD3_H {
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_15 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_14 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_13 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_12 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 70 CS Register"]
pub mod CS70 {
    #[doc = "Free-Running Counter Time stamp. This 16-bit field is a copy of the Free-Running Timer, captured for Tx and Rx frames at the time when the beginning of the Identifier field appears on the CAN bus."]
    pub mod TIME_STAMP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Length of the data to be stored/transmitted."]
    pub mod DLC {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Remote Transmission Request. One/zero for remote/data frame."]
    pub mod RTR {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ID Extended. One/zero for extended/standard format frame."]
    pub mod IDE {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Substitute Remote Request. Contains a fixed recessive bit."]
    pub mod SRR {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Message Buffer Code. This 4-bit field can be accessed (read or write) by the CPU and by the FlexCAN module itself, as part of the message buffer matching and arbitration process."]
    pub mod CODE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Error State Indicator. This bit indicates if the transmitting node is error active or error passive."]
    pub mod ESI {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Bit Rate Switch. This bit defines whether the bit rate is switched inside a CAN FD format frame."]
    pub mod BRS {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Extended Data Length. This bit distinguishes between CAN format and CAN FD format frames. The EDL bit must not be set for Message Buffers configured to RANSWER with code field 0b1010."]
    pub mod EDL {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 70 ID Register"]
pub mod ID70 {
    #[doc = "Contains extended (LOW word) identifier of message buffer."]
    pub mod EXT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0003_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Contains standard/extended (HIGH word) identifier of message buffer."]
    pub mod STD {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Local priority. This 3-bit fieldis only used when LPRIO_EN bit is set in MCR and it only makes sense for Tx buffers. These bits are not transmitted. They are appended to the regular ID to define the transmission priority."]
    pub mod PRIO {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 1 WORD_64B Register"]
pub mod MB1_64B_WORD6_H {
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_27 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_26 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_25 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_24 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 1 WORD_64B Register"]
pub mod MB1_64B_WORD7_H {
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_31 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_30 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_29 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_28 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 71 CS Register"]
pub mod CS71 {
    #[doc = "Free-Running Counter Time stamp. This 16-bit field is a copy of the Free-Running Timer, captured for Tx and Rx frames at the time when the beginning of the Identifier field appears on the CAN bus."]
    pub mod TIME_STAMP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Length of the data to be stored/transmitted."]
    pub mod DLC {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Remote Transmission Request. One/zero for remote/data frame."]
    pub mod RTR {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ID Extended. One/zero for extended/standard format frame."]
    pub mod IDE {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Substitute Remote Request. Contains a fixed recessive bit."]
    pub mod SRR {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Message Buffer Code. This 4-bit field can be accessed (read or write) by the CPU and by the FlexCAN module itself, as part of the message buffer matching and arbitration process."]
    pub mod CODE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Error State Indicator. This bit indicates if the transmitting node is error active or error passive."]
    pub mod ESI {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Bit Rate Switch. This bit defines whether the bit rate is switched inside a CAN FD format frame."]
    pub mod BRS {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Extended Data Length. This bit distinguishes between CAN format and CAN FD format frames. The EDL bit must not be set for Message Buffers configured to RANSWER with code field 0b1010."]
    pub mod EDL {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 71 ID Register"]
pub mod ID71 {
    #[doc = "Contains extended (LOW word) identifier of message buffer."]
    pub mod EXT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0003_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Contains standard/extended (HIGH word) identifier of message buffer."]
    pub mod STD {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Local priority. This 3-bit fieldis only used when LPRIO_EN bit is set in MCR and it only makes sense for Tx buffers. These bits are not transmitted. They are appended to the regular ID to define the transmission priority."]
    pub mod PRIO {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 1 WORD_64B Register"]
pub mod MB1_64B_WORD10_H {
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_43 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_42 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_41 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_40 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 1 WORD_64B Register"]
pub mod MB1_64B_WORD11_H {
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_47 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_46 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_45 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_44 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 72 CS Register"]
pub mod CS72 {
    #[doc = "Free-Running Counter Time stamp. This 16-bit field is a copy of the Free-Running Timer, captured for Tx and Rx frames at the time when the beginning of the Identifier field appears on the CAN bus."]
    pub mod TIME_STAMP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Length of the data to be stored/transmitted."]
    pub mod DLC {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Remote Transmission Request. One/zero for remote/data frame."]
    pub mod RTR {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ID Extended. One/zero for extended/standard format frame."]
    pub mod IDE {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Substitute Remote Request. Contains a fixed recessive bit."]
    pub mod SRR {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Message Buffer Code. This 4-bit field can be accessed (read or write) by the CPU and by the FlexCAN module itself, as part of the message buffer matching and arbitration process."]
    pub mod CODE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Error State Indicator. This bit indicates if the transmitting node is error active or error passive."]
    pub mod ESI {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Bit Rate Switch. This bit defines whether the bit rate is switched inside a CAN FD format frame."]
    pub mod BRS {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Extended Data Length. This bit distinguishes between CAN format and CAN FD format frames. The EDL bit must not be set for Message Buffers configured to RANSWER with code field 0b1010."]
    pub mod EDL {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 72 ID Register"]
pub mod ID72 {
    #[doc = "Contains extended (LOW word) identifier of message buffer."]
    pub mod EXT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0003_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Contains standard/extended (HIGH word) identifier of message buffer."]
    pub mod STD {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Local priority. This 3-bit fieldis only used when LPRIO_EN bit is set in MCR and it only makes sense for Tx buffers. These bits are not transmitted. They are appended to the regular ID to define the transmission priority."]
    pub mod PRIO {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 1 WORD_64B Register"]
pub mod MB1_64B_WORD14_H {
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_59 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_58 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_57 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_56 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 1 WORD_64B Register"]
pub mod MB1_64B_WORD15_H {
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_63 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_62 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_61 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_60 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 73 CS Register"]
pub mod CS73 {
    #[doc = "Free-Running Counter Time stamp. This 16-bit field is a copy of the Free-Running Timer, captured for Tx and Rx frames at the time when the beginning of the Identifier field appears on the CAN bus."]
    pub mod TIME_STAMP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Length of the data to be stored/transmitted."]
    pub mod DLC {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Remote Transmission Request. One/zero for remote/data frame."]
    pub mod RTR {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ID Extended. One/zero for extended/standard format frame."]
    pub mod IDE {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Substitute Remote Request. Contains a fixed recessive bit."]
    pub mod SRR {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Message Buffer Code. This 4-bit field can be accessed (read or write) by the CPU and by the FlexCAN module itself, as part of the message buffer matching and arbitration process."]
    pub mod CODE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Error State Indicator. This bit indicates if the transmitting node is error active or error passive."]
    pub mod ESI {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Bit Rate Switch. This bit defines whether the bit rate is switched inside a CAN FD format frame."]
    pub mod BRS {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Extended Data Length. This bit distinguishes between CAN format and CAN FD format frames. The EDL bit must not be set for Message Buffers configured to RANSWER with code field 0b1010."]
    pub mod EDL {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 73 ID Register"]
pub mod ID73 {
    #[doc = "Contains extended (LOW word) identifier of message buffer."]
    pub mod EXT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0003_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Contains standard/extended (HIGH word) identifier of message buffer."]
    pub mod STD {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Local priority. This 3-bit fieldis only used when LPRIO_EN bit is set in MCR and it only makes sense for Tx buffers. These bits are not transmitted. They are appended to the regular ID to define the transmission priority."]
    pub mod PRIO {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 2 WORD_64B Register"]
pub mod MB2_64B_WORD0_H {
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_3 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_2 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_1 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_0 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 2 WORD_64B Register"]
pub mod MB2_64B_WORD1_H {
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_7 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_6 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_5 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_4 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 74 CS Register"]
pub mod CS74 {
    #[doc = "Free-Running Counter Time stamp. This 16-bit field is a copy of the Free-Running Timer, captured for Tx and Rx frames at the time when the beginning of the Identifier field appears on the CAN bus."]
    pub mod TIME_STAMP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Length of the data to be stored/transmitted."]
    pub mod DLC {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Remote Transmission Request. One/zero for remote/data frame."]
    pub mod RTR {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ID Extended. One/zero for extended/standard format frame."]
    pub mod IDE {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Substitute Remote Request. Contains a fixed recessive bit."]
    pub mod SRR {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Message Buffer Code. This 4-bit field can be accessed (read or write) by the CPU and by the FlexCAN module itself, as part of the message buffer matching and arbitration process."]
    pub mod CODE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Error State Indicator. This bit indicates if the transmitting node is error active or error passive."]
    pub mod ESI {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Bit Rate Switch. This bit defines whether the bit rate is switched inside a CAN FD format frame."]
    pub mod BRS {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Extended Data Length. This bit distinguishes between CAN format and CAN FD format frames. The EDL bit must not be set for Message Buffers configured to RANSWER with code field 0b1010."]
    pub mod EDL {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 74 ID Register"]
pub mod ID74 {
    #[doc = "Contains extended (LOW word) identifier of message buffer."]
    pub mod EXT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0003_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Contains standard/extended (HIGH word) identifier of message buffer."]
    pub mod STD {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Local priority. This 3-bit fieldis only used when LPRIO_EN bit is set in MCR and it only makes sense for Tx buffers. These bits are not transmitted. They are appended to the regular ID to define the transmission priority."]
    pub mod PRIO {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 2 WORD_64B Register"]
pub mod MB2_64B_WORD4_H {
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_19 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_18 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_17 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_16 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 2 WORD_64B Register"]
pub mod MB2_64B_WORD5_H {
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_23 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_22 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_21 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_20 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 75 CS Register"]
pub mod CS75 {
    #[doc = "Free-Running Counter Time stamp. This 16-bit field is a copy of the Free-Running Timer, captured for Tx and Rx frames at the time when the beginning of the Identifier field appears on the CAN bus."]
    pub mod TIME_STAMP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Length of the data to be stored/transmitted."]
    pub mod DLC {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Remote Transmission Request. One/zero for remote/data frame."]
    pub mod RTR {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ID Extended. One/zero for extended/standard format frame."]
    pub mod IDE {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Substitute Remote Request. Contains a fixed recessive bit."]
    pub mod SRR {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Message Buffer Code. This 4-bit field can be accessed (read or write) by the CPU and by the FlexCAN module itself, as part of the message buffer matching and arbitration process."]
    pub mod CODE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Error State Indicator. This bit indicates if the transmitting node is error active or error passive."]
    pub mod ESI {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Bit Rate Switch. This bit defines whether the bit rate is switched inside a CAN FD format frame."]
    pub mod BRS {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Extended Data Length. This bit distinguishes between CAN format and CAN FD format frames. The EDL bit must not be set for Message Buffers configured to RANSWER with code field 0b1010."]
    pub mod EDL {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 75 ID Register"]
pub mod ID75 {
    #[doc = "Contains extended (LOW word) identifier of message buffer."]
    pub mod EXT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0003_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Contains standard/extended (HIGH word) identifier of message buffer."]
    pub mod STD {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Local priority. This 3-bit fieldis only used when LPRIO_EN bit is set in MCR and it only makes sense for Tx buffers. These bits are not transmitted. They are appended to the regular ID to define the transmission priority."]
    pub mod PRIO {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 2 WORD_64B Register"]
pub mod MB2_64B_WORD8_H {
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_35 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_34 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_33 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_32 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 2 WORD_64B Register"]
pub mod MB2_64B_WORD9_H {
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_39 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_38 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_37 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_36 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 76 CS Register"]
pub mod CS76 {
    #[doc = "Free-Running Counter Time stamp. This 16-bit field is a copy of the Free-Running Timer, captured for Tx and Rx frames at the time when the beginning of the Identifier field appears on the CAN bus."]
    pub mod TIME_STAMP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Length of the data to be stored/transmitted."]
    pub mod DLC {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Remote Transmission Request. One/zero for remote/data frame."]
    pub mod RTR {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ID Extended. One/zero for extended/standard format frame."]
    pub mod IDE {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Substitute Remote Request. Contains a fixed recessive bit."]
    pub mod SRR {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Message Buffer Code. This 4-bit field can be accessed (read or write) by the CPU and by the FlexCAN module itself, as part of the message buffer matching and arbitration process."]
    pub mod CODE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Error State Indicator. This bit indicates if the transmitting node is error active or error passive."]
    pub mod ESI {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Bit Rate Switch. This bit defines whether the bit rate is switched inside a CAN FD format frame."]
    pub mod BRS {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Extended Data Length. This bit distinguishes between CAN format and CAN FD format frames. The EDL bit must not be set for Message Buffers configured to RANSWER with code field 0b1010."]
    pub mod EDL {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 76 ID Register"]
pub mod ID76 {
    #[doc = "Contains extended (LOW word) identifier of message buffer."]
    pub mod EXT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0003_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Contains standard/extended (HIGH word) identifier of message buffer."]
    pub mod STD {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Local priority. This 3-bit fieldis only used when LPRIO_EN bit is set in MCR and it only makes sense for Tx buffers. These bits are not transmitted. They are appended to the regular ID to define the transmission priority."]
    pub mod PRIO {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 2 WORD_64B Register"]
pub mod MB2_64B_WORD12_H {
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_51 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_50 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_49 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_48 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 2 WORD_64B Register"]
pub mod MB2_64B_WORD13_H {
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_55 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_54 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_53 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_52 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 77 CS Register"]
pub mod CS77 {
    #[doc = "Free-Running Counter Time stamp. This 16-bit field is a copy of the Free-Running Timer, captured for Tx and Rx frames at the time when the beginning of the Identifier field appears on the CAN bus."]
    pub mod TIME_STAMP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Length of the data to be stored/transmitted."]
    pub mod DLC {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Remote Transmission Request. One/zero for remote/data frame."]
    pub mod RTR {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ID Extended. One/zero for extended/standard format frame."]
    pub mod IDE {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Substitute Remote Request. Contains a fixed recessive bit."]
    pub mod SRR {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Message Buffer Code. This 4-bit field can be accessed (read or write) by the CPU and by the FlexCAN module itself, as part of the message buffer matching and arbitration process."]
    pub mod CODE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Error State Indicator. This bit indicates if the transmitting node is error active or error passive."]
    pub mod ESI {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Bit Rate Switch. This bit defines whether the bit rate is switched inside a CAN FD format frame."]
    pub mod BRS {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Extended Data Length. This bit distinguishes between CAN format and CAN FD format frames. The EDL bit must not be set for Message Buffers configured to RANSWER with code field 0b1010."]
    pub mod EDL {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 77 ID Register"]
pub mod ID77 {
    #[doc = "Contains extended (LOW word) identifier of message buffer."]
    pub mod EXT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0003_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Contains standard/extended (HIGH word) identifier of message buffer."]
    pub mod STD {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Local priority. This 3-bit fieldis only used when LPRIO_EN bit is set in MCR and it only makes sense for Tx buffers. These bits are not transmitted. They are appended to the regular ID to define the transmission priority."]
    pub mod PRIO {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 3 CS Register"]
pub mod MB3_64B_CS_H {
    #[doc = "Free-Running Counter Time stamp. This 16-bit field is a copy of the Free-Running Timer, captured for Tx and Rx frames at the time when the beginning of the Identifier field appears on the CAN bus."]
    pub mod TIME_STAMP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Length of the data to be stored/transmitted."]
    pub mod DLC {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Remote Transmission Request. One/zero for remote/data frame."]
    pub mod RTR {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ID Extended. One/zero for extended/standard format frame."]
    pub mod IDE {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Substitute Remote Request. Contains a fixed recessive bit."]
    pub mod SRR {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Message Buffer Code. This 4-bit field can be accessed (read or write) by the CPU and by the FlexCAN module itself, as part of the message buffer matching and arbitration process."]
    pub mod CODE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Error State Indicator. This bit indicates if the transmitting node is error active or error passive."]
    pub mod ESI {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Bit Rate Switch. This bit defines whether the bit rate is switched inside a CAN FD format frame."]
    pub mod BRS {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Extended Data Length. This bit distinguishes between CAN format and CAN FD format frames. The EDL bit must not be set for Message Buffers configured to RANSWER with code field 0b1010."]
    pub mod EDL {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 3 ID Register"]
pub mod MB3_64B_ID_H {
    #[doc = "Contains extended (LOW word) identifier of message buffer."]
    pub mod EXT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0003_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Contains standard/extended (HIGH word) identifier of message buffer."]
    pub mod STD {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Local priority. This 3-bit fieldis only used when LPRIO_EN bit is set in MCR and it only makes sense for Tx buffers. These bits are not transmitted. They are appended to the regular ID to define the transmission priority."]
    pub mod PRIO {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 78 CS Register"]
pub mod CS78 {
    #[doc = "Free-Running Counter Time stamp. This 16-bit field is a copy of the Free-Running Timer, captured for Tx and Rx frames at the time when the beginning of the Identifier field appears on the CAN bus."]
    pub mod TIME_STAMP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Length of the data to be stored/transmitted."]
    pub mod DLC {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Remote Transmission Request. One/zero for remote/data frame."]
    pub mod RTR {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ID Extended. One/zero for extended/standard format frame."]
    pub mod IDE {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Substitute Remote Request. Contains a fixed recessive bit."]
    pub mod SRR {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Message Buffer Code. This 4-bit field can be accessed (read or write) by the CPU and by the FlexCAN module itself, as part of the message buffer matching and arbitration process."]
    pub mod CODE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Error State Indicator. This bit indicates if the transmitting node is error active or error passive."]
    pub mod ESI {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Bit Rate Switch. This bit defines whether the bit rate is switched inside a CAN FD format frame."]
    pub mod BRS {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Extended Data Length. This bit distinguishes between CAN format and CAN FD format frames. The EDL bit must not be set for Message Buffers configured to RANSWER with code field 0b1010."]
    pub mod EDL {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 78 ID Register"]
pub mod ID78 {
    #[doc = "Contains extended (LOW word) identifier of message buffer."]
    pub mod EXT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0003_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Contains standard/extended (HIGH word) identifier of message buffer."]
    pub mod STD {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Local priority. This 3-bit fieldis only used when LPRIO_EN bit is set in MCR and it only makes sense for Tx buffers. These bits are not transmitted. They are appended to the regular ID to define the transmission priority."]
    pub mod PRIO {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 3 WORD_64B Register"]
pub mod MB3_64B_WORD2_H {
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_11 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_10 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_9 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_8 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 3 WORD_64B Register"]
pub mod MB3_64B_WORD3_H {
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_15 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_14 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_13 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_12 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 79 CS Register"]
pub mod CS79 {
    #[doc = "Free-Running Counter Time stamp. This 16-bit field is a copy of the Free-Running Timer, captured for Tx and Rx frames at the time when the beginning of the Identifier field appears on the CAN bus."]
    pub mod TIME_STAMP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Length of the data to be stored/transmitted."]
    pub mod DLC {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Remote Transmission Request. One/zero for remote/data frame."]
    pub mod RTR {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ID Extended. One/zero for extended/standard format frame."]
    pub mod IDE {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Substitute Remote Request. Contains a fixed recessive bit."]
    pub mod SRR {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Message Buffer Code. This 4-bit field can be accessed (read or write) by the CPU and by the FlexCAN module itself, as part of the message buffer matching and arbitration process."]
    pub mod CODE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Error State Indicator. This bit indicates if the transmitting node is error active or error passive."]
    pub mod ESI {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Bit Rate Switch. This bit defines whether the bit rate is switched inside a CAN FD format frame."]
    pub mod BRS {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Extended Data Length. This bit distinguishes between CAN format and CAN FD format frames. The EDL bit must not be set for Message Buffers configured to RANSWER with code field 0b1010."]
    pub mod EDL {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 79 ID Register"]
pub mod ID79 {
    #[doc = "Contains extended (LOW word) identifier of message buffer."]
    pub mod EXT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0003_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Contains standard/extended (HIGH word) identifier of message buffer."]
    pub mod STD {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Local priority. This 3-bit fieldis only used when LPRIO_EN bit is set in MCR and it only makes sense for Tx buffers. These bits are not transmitted. They are appended to the regular ID to define the transmission priority."]
    pub mod PRIO {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 10 WORD_16B Register"]
pub mod MB10_16B_WORD0_H {
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_3 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_2 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_1 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_0 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 10 WORD_16B Register"]
pub mod MB10_16B_WORD1_H {
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_7 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_6 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_5 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_4 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 80 CS Register"]
pub mod CS80 {
    #[doc = "Free-Running Counter Time stamp. This 16-bit field is a copy of the Free-Running Timer, captured for Tx and Rx frames at the time when the beginning of the Identifier field appears on the CAN bus."]
    pub mod TIME_STAMP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Length of the data to be stored/transmitted."]
    pub mod DLC {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Remote Transmission Request. One/zero for remote/data frame."]
    pub mod RTR {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ID Extended. One/zero for extended/standard format frame."]
    pub mod IDE {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Substitute Remote Request. Contains a fixed recessive bit."]
    pub mod SRR {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Message Buffer Code. This 4-bit field can be accessed (read or write) by the CPU and by the FlexCAN module itself, as part of the message buffer matching and arbitration process."]
    pub mod CODE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Error State Indicator. This bit indicates if the transmitting node is error active or error passive."]
    pub mod ESI {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Bit Rate Switch. This bit defines whether the bit rate is switched inside a CAN FD format frame."]
    pub mod BRS {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Extended Data Length. This bit distinguishes between CAN format and CAN FD format frames. The EDL bit must not be set for Message Buffers configured to RANSWER with code field 0b1010."]
    pub mod EDL {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 80 ID Register"]
pub mod ID80 {
    #[doc = "Contains extended (LOW word) identifier of message buffer."]
    pub mod EXT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0003_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Contains standard/extended (HIGH word) identifier of message buffer."]
    pub mod STD {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Local priority. This 3-bit fieldis only used when LPRIO_EN bit is set in MCR and it only makes sense for Tx buffers. These bits are not transmitted. They are appended to the regular ID to define the transmission priority."]
    pub mod PRIO {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 11 CS Register"]
pub mod MB11_16B_CS_H {
    #[doc = "Free-Running Counter Time stamp. This 16-bit field is a copy of the Free-Running Timer, captured for Tx and Rx frames at the time when the beginning of the Identifier field appears on the CAN bus."]
    pub mod TIME_STAMP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Length of the data to be stored/transmitted."]
    pub mod DLC {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Remote Transmission Request. One/zero for remote/data frame."]
    pub mod RTR {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ID Extended. One/zero for extended/standard format frame."]
    pub mod IDE {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Substitute Remote Request. Contains a fixed recessive bit."]
    pub mod SRR {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Message Buffer Code. This 4-bit field can be accessed (read or write) by the CPU and by the FlexCAN module itself, as part of the message buffer matching and arbitration process."]
    pub mod CODE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Error State Indicator. This bit indicates if the transmitting node is error active or error passive."]
    pub mod ESI {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Bit Rate Switch. This bit defines whether the bit rate is switched inside a CAN FD format frame."]
    pub mod BRS {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Extended Data Length. This bit distinguishes between CAN format and CAN FD format frames. The EDL bit must not be set for Message Buffers configured to RANSWER with code field 0b1010."]
    pub mod EDL {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 11 ID Register"]
pub mod MB11_16B_ID_H {
    #[doc = "Contains extended (LOW word) identifier of message buffer."]
    pub mod EXT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0003_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Contains standard/extended (HIGH word) identifier of message buffer."]
    pub mod STD {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Local priority. This 3-bit fieldis only used when LPRIO_EN bit is set in MCR and it only makes sense for Tx buffers. These bits are not transmitted. They are appended to the regular ID to define the transmission priority."]
    pub mod PRIO {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 81 CS Register"]
pub mod CS81 {
    #[doc = "Free-Running Counter Time stamp. This 16-bit field is a copy of the Free-Running Timer, captured for Tx and Rx frames at the time when the beginning of the Identifier field appears on the CAN bus."]
    pub mod TIME_STAMP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Length of the data to be stored/transmitted."]
    pub mod DLC {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Remote Transmission Request. One/zero for remote/data frame."]
    pub mod RTR {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ID Extended. One/zero for extended/standard format frame."]
    pub mod IDE {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Substitute Remote Request. Contains a fixed recessive bit."]
    pub mod SRR {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Message Buffer Code. This 4-bit field can be accessed (read or write) by the CPU and by the FlexCAN module itself, as part of the message buffer matching and arbitration process."]
    pub mod CODE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Error State Indicator. This bit indicates if the transmitting node is error active or error passive."]
    pub mod ESI {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Bit Rate Switch. This bit defines whether the bit rate is switched inside a CAN FD format frame."]
    pub mod BRS {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Extended Data Length. This bit distinguishes between CAN format and CAN FD format frames. The EDL bit must not be set for Message Buffers configured to RANSWER with code field 0b1010."]
    pub mod EDL {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 81 ID Register"]
pub mod ID81 {
    #[doc = "Contains extended (LOW word) identifier of message buffer."]
    pub mod EXT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0003_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Contains standard/extended (HIGH word) identifier of message buffer."]
    pub mod STD {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Local priority. This 3-bit fieldis only used when LPRIO_EN bit is set in MCR and it only makes sense for Tx buffers. These bits are not transmitted. They are appended to the regular ID to define the transmission priority."]
    pub mod PRIO {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 11 WORD_16B Register"]
pub mod MB11_16B_WORD2_H {
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_11 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_10 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_9 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_8 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 11 WORD_16B Register"]
pub mod MB11_16B_WORD3_H {
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_15 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_14 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_13 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_12 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 82 CS Register"]
pub mod CS82 {
    #[doc = "Free-Running Counter Time stamp. This 16-bit field is a copy of the Free-Running Timer, captured for Tx and Rx frames at the time when the beginning of the Identifier field appears on the CAN bus."]
    pub mod TIME_STAMP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Length of the data to be stored/transmitted."]
    pub mod DLC {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Remote Transmission Request. One/zero for remote/data frame."]
    pub mod RTR {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ID Extended. One/zero for extended/standard format frame."]
    pub mod IDE {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Substitute Remote Request. Contains a fixed recessive bit."]
    pub mod SRR {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Message Buffer Code. This 4-bit field can be accessed (read or write) by the CPU and by the FlexCAN module itself, as part of the message buffer matching and arbitration process."]
    pub mod CODE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Error State Indicator. This bit indicates if the transmitting node is error active or error passive."]
    pub mod ESI {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Bit Rate Switch. This bit defines whether the bit rate is switched inside a CAN FD format frame."]
    pub mod BRS {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Extended Data Length. This bit distinguishes between CAN format and CAN FD format frames. The EDL bit must not be set for Message Buffers configured to RANSWER with code field 0b1010."]
    pub mod EDL {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 82 ID Register"]
pub mod ID82 {
    #[doc = "Contains extended (LOW word) identifier of message buffer."]
    pub mod EXT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0003_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Contains standard/extended (HIGH word) identifier of message buffer."]
    pub mod STD {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Local priority. This 3-bit fieldis only used when LPRIO_EN bit is set in MCR and it only makes sense for Tx buffers. These bits are not transmitted. They are appended to the regular ID to define the transmission priority."]
    pub mod PRIO {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 12 WORD_16B Register"]
pub mod MB12_16B_WORD0_H {
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_3 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_2 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_1 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_0 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 12 WORD_16B Register"]
pub mod MB12_16B_WORD1_H {
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_7 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_6 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_5 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_4 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 83 CS Register"]
pub mod CS83 {
    #[doc = "Free-Running Counter Time stamp. This 16-bit field is a copy of the Free-Running Timer, captured for Tx and Rx frames at the time when the beginning of the Identifier field appears on the CAN bus."]
    pub mod TIME_STAMP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Length of the data to be stored/transmitted."]
    pub mod DLC {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Remote Transmission Request. One/zero for remote/data frame."]
    pub mod RTR {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ID Extended. One/zero for extended/standard format frame."]
    pub mod IDE {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Substitute Remote Request. Contains a fixed recessive bit."]
    pub mod SRR {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Message Buffer Code. This 4-bit field can be accessed (read or write) by the CPU and by the FlexCAN module itself, as part of the message buffer matching and arbitration process."]
    pub mod CODE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Error State Indicator. This bit indicates if the transmitting node is error active or error passive."]
    pub mod ESI {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Bit Rate Switch. This bit defines whether the bit rate is switched inside a CAN FD format frame."]
    pub mod BRS {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Extended Data Length. This bit distinguishes between CAN format and CAN FD format frames. The EDL bit must not be set for Message Buffers configured to RANSWER with code field 0b1010."]
    pub mod EDL {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 83 ID Register"]
pub mod ID83 {
    #[doc = "Contains extended (LOW word) identifier of message buffer."]
    pub mod EXT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0003_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Contains standard/extended (HIGH word) identifier of message buffer."]
    pub mod STD {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Local priority. This 3-bit fieldis only used when LPRIO_EN bit is set in MCR and it only makes sense for Tx buffers. These bits are not transmitted. They are appended to the regular ID to define the transmission priority."]
    pub mod PRIO {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 13 CS Register"]
pub mod MB13_16B_CS_H {
    #[doc = "Free-Running Counter Time stamp. This 16-bit field is a copy of the Free-Running Timer, captured for Tx and Rx frames at the time when the beginning of the Identifier field appears on the CAN bus."]
    pub mod TIME_STAMP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Length of the data to be stored/transmitted."]
    pub mod DLC {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Remote Transmission Request. One/zero for remote/data frame."]
    pub mod RTR {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ID Extended. One/zero for extended/standard format frame."]
    pub mod IDE {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Substitute Remote Request. Contains a fixed recessive bit."]
    pub mod SRR {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Message Buffer Code. This 4-bit field can be accessed (read or write) by the CPU and by the FlexCAN module itself, as part of the message buffer matching and arbitration process."]
    pub mod CODE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Error State Indicator. This bit indicates if the transmitting node is error active or error passive."]
    pub mod ESI {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Bit Rate Switch. This bit defines whether the bit rate is switched inside a CAN FD format frame."]
    pub mod BRS {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Extended Data Length. This bit distinguishes between CAN format and CAN FD format frames. The EDL bit must not be set for Message Buffers configured to RANSWER with code field 0b1010."]
    pub mod EDL {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 13 ID Register"]
pub mod MB13_16B_ID_H {
    #[doc = "Contains extended (LOW word) identifier of message buffer."]
    pub mod EXT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0003_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Contains standard/extended (HIGH word) identifier of message buffer."]
    pub mod STD {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Local priority. This 3-bit fieldis only used when LPRIO_EN bit is set in MCR and it only makes sense for Tx buffers. These bits are not transmitted. They are appended to the regular ID to define the transmission priority."]
    pub mod PRIO {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 84 CS Register"]
pub mod CS84 {
    #[doc = "Free-Running Counter Time stamp. This 16-bit field is a copy of the Free-Running Timer, captured for Tx and Rx frames at the time when the beginning of the Identifier field appears on the CAN bus."]
    pub mod TIME_STAMP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Length of the data to be stored/transmitted."]
    pub mod DLC {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Remote Transmission Request. One/zero for remote/data frame."]
    pub mod RTR {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ID Extended. One/zero for extended/standard format frame."]
    pub mod IDE {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Substitute Remote Request. Contains a fixed recessive bit."]
    pub mod SRR {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Message Buffer Code. This 4-bit field can be accessed (read or write) by the CPU and by the FlexCAN module itself, as part of the message buffer matching and arbitration process."]
    pub mod CODE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Error State Indicator. This bit indicates if the transmitting node is error active or error passive."]
    pub mod ESI {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Bit Rate Switch. This bit defines whether the bit rate is switched inside a CAN FD format frame."]
    pub mod BRS {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Extended Data Length. This bit distinguishes between CAN format and CAN FD format frames. The EDL bit must not be set for Message Buffers configured to RANSWER with code field 0b1010."]
    pub mod EDL {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 84 ID Register"]
pub mod ID84 {
    #[doc = "Contains extended (LOW word) identifier of message buffer."]
    pub mod EXT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0003_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Contains standard/extended (HIGH word) identifier of message buffer."]
    pub mod STD {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Local priority. This 3-bit fieldis only used when LPRIO_EN bit is set in MCR and it only makes sense for Tx buffers. These bits are not transmitted. They are appended to the regular ID to define the transmission priority."]
    pub mod PRIO {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 13 WORD_16B Register"]
pub mod MB13_16B_WORD2_H {
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_11 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_10 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_9 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_8 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 13 WORD_16B Register"]
pub mod MB13_16B_WORD3_H {
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_15 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_14 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_13 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_12 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 85 CS Register"]
pub mod CS85 {
    #[doc = "Free-Running Counter Time stamp. This 16-bit field is a copy of the Free-Running Timer, captured for Tx and Rx frames at the time when the beginning of the Identifier field appears on the CAN bus."]
    pub mod TIME_STAMP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Length of the data to be stored/transmitted."]
    pub mod DLC {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Remote Transmission Request. One/zero for remote/data frame."]
    pub mod RTR {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ID Extended. One/zero for extended/standard format frame."]
    pub mod IDE {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Substitute Remote Request. Contains a fixed recessive bit."]
    pub mod SRR {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Message Buffer Code. This 4-bit field can be accessed (read or write) by the CPU and by the FlexCAN module itself, as part of the message buffer matching and arbitration process."]
    pub mod CODE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Error State Indicator. This bit indicates if the transmitting node is error active or error passive."]
    pub mod ESI {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Bit Rate Switch. This bit defines whether the bit rate is switched inside a CAN FD format frame."]
    pub mod BRS {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Extended Data Length. This bit distinguishes between CAN format and CAN FD format frames. The EDL bit must not be set for Message Buffers configured to RANSWER with code field 0b1010."]
    pub mod EDL {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 85 ID Register"]
pub mod ID85 {
    #[doc = "Contains extended (LOW word) identifier of message buffer."]
    pub mod EXT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0003_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Contains standard/extended (HIGH word) identifier of message buffer."]
    pub mod STD {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Local priority. This 3-bit fieldis only used when LPRIO_EN bit is set in MCR and it only makes sense for Tx buffers. These bits are not transmitted. They are appended to the regular ID to define the transmission priority."]
    pub mod PRIO {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 14 WORD_16B Register"]
pub mod MB14_16B_WORD0_H {
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_3 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_2 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_1 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_0 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 14 WORD_16B Register"]
pub mod MB14_16B_WORD1_H {
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_7 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_6 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_5 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_4 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 86 CS Register"]
pub mod CS86 {
    #[doc = "Free-Running Counter Time stamp. This 16-bit field is a copy of the Free-Running Timer, captured for Tx and Rx frames at the time when the beginning of the Identifier field appears on the CAN bus."]
    pub mod TIME_STAMP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Length of the data to be stored/transmitted."]
    pub mod DLC {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Remote Transmission Request. One/zero for remote/data frame."]
    pub mod RTR {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ID Extended. One/zero for extended/standard format frame."]
    pub mod IDE {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Substitute Remote Request. Contains a fixed recessive bit."]
    pub mod SRR {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Message Buffer Code. This 4-bit field can be accessed (read or write) by the CPU and by the FlexCAN module itself, as part of the message buffer matching and arbitration process."]
    pub mod CODE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Error State Indicator. This bit indicates if the transmitting node is error active or error passive."]
    pub mod ESI {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Bit Rate Switch. This bit defines whether the bit rate is switched inside a CAN FD format frame."]
    pub mod BRS {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Extended Data Length. This bit distinguishes between CAN format and CAN FD format frames. The EDL bit must not be set for Message Buffers configured to RANSWER with code field 0b1010."]
    pub mod EDL {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 86 ID Register"]
pub mod ID86 {
    #[doc = "Contains extended (LOW word) identifier of message buffer."]
    pub mod EXT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0003_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Contains standard/extended (HIGH word) identifier of message buffer."]
    pub mod STD {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Local priority. This 3-bit fieldis only used when LPRIO_EN bit is set in MCR and it only makes sense for Tx buffers. These bits are not transmitted. They are appended to the regular ID to define the transmission priority."]
    pub mod PRIO {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 15 CS Register"]
pub mod MB15_16B_CS_H {
    #[doc = "Free-Running Counter Time stamp. This 16-bit field is a copy of the Free-Running Timer, captured for Tx and Rx frames at the time when the beginning of the Identifier field appears on the CAN bus."]
    pub mod TIME_STAMP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Length of the data to be stored/transmitted."]
    pub mod DLC {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Remote Transmission Request. One/zero for remote/data frame."]
    pub mod RTR {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ID Extended. One/zero for extended/standard format frame."]
    pub mod IDE {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Substitute Remote Request. Contains a fixed recessive bit."]
    pub mod SRR {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Message Buffer Code. This 4-bit field can be accessed (read or write) by the CPU and by the FlexCAN module itself, as part of the message buffer matching and arbitration process."]
    pub mod CODE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Error State Indicator. This bit indicates if the transmitting node is error active or error passive."]
    pub mod ESI {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Bit Rate Switch. This bit defines whether the bit rate is switched inside a CAN FD format frame."]
    pub mod BRS {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Extended Data Length. This bit distinguishes between CAN format and CAN FD format frames. The EDL bit must not be set for Message Buffers configured to RANSWER with code field 0b1010."]
    pub mod EDL {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 15 ID Register"]
pub mod MB15_16B_ID_H {
    #[doc = "Contains extended (LOW word) identifier of message buffer."]
    pub mod EXT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0003_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Contains standard/extended (HIGH word) identifier of message buffer."]
    pub mod STD {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Local priority. This 3-bit fieldis only used when LPRIO_EN bit is set in MCR and it only makes sense for Tx buffers. These bits are not transmitted. They are appended to the regular ID to define the transmission priority."]
    pub mod PRIO {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 87 CS Register"]
pub mod CS87 {
    #[doc = "Free-Running Counter Time stamp. This 16-bit field is a copy of the Free-Running Timer, captured for Tx and Rx frames at the time when the beginning of the Identifier field appears on the CAN bus."]
    pub mod TIME_STAMP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Length of the data to be stored/transmitted."]
    pub mod DLC {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Remote Transmission Request. One/zero for remote/data frame."]
    pub mod RTR {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ID Extended. One/zero for extended/standard format frame."]
    pub mod IDE {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Substitute Remote Request. Contains a fixed recessive bit."]
    pub mod SRR {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Message Buffer Code. This 4-bit field can be accessed (read or write) by the CPU and by the FlexCAN module itself, as part of the message buffer matching and arbitration process."]
    pub mod CODE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Error State Indicator. This bit indicates if the transmitting node is error active or error passive."]
    pub mod ESI {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Bit Rate Switch. This bit defines whether the bit rate is switched inside a CAN FD format frame."]
    pub mod BRS {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Extended Data Length. This bit distinguishes between CAN format and CAN FD format frames. The EDL bit must not be set for Message Buffers configured to RANSWER with code field 0b1010."]
    pub mod EDL {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 87 ID Register"]
pub mod ID87 {
    #[doc = "Contains extended (LOW word) identifier of message buffer."]
    pub mod EXT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0003_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Contains standard/extended (HIGH word) identifier of message buffer."]
    pub mod STD {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Local priority. This 3-bit fieldis only used when LPRIO_EN bit is set in MCR and it only makes sense for Tx buffers. These bits are not transmitted. They are appended to the regular ID to define the transmission priority."]
    pub mod PRIO {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 15 WORD_16B Register"]
pub mod MB15_16B_WORD2_H {
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_11 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_10 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_9 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_8 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 15 WORD_16B Register"]
pub mod MB15_16B_WORD3_H {
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_15 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_14 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_13 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_12 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 88 CS Register"]
pub mod CS88 {
    #[doc = "Free-Running Counter Time stamp. This 16-bit field is a copy of the Free-Running Timer, captured for Tx and Rx frames at the time when the beginning of the Identifier field appears on the CAN bus."]
    pub mod TIME_STAMP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Length of the data to be stored/transmitted."]
    pub mod DLC {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Remote Transmission Request. One/zero for remote/data frame."]
    pub mod RTR {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ID Extended. One/zero for extended/standard format frame."]
    pub mod IDE {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Substitute Remote Request. Contains a fixed recessive bit."]
    pub mod SRR {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Message Buffer Code. This 4-bit field can be accessed (read or write) by the CPU and by the FlexCAN module itself, as part of the message buffer matching and arbitration process."]
    pub mod CODE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Error State Indicator. This bit indicates if the transmitting node is error active or error passive."]
    pub mod ESI {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Bit Rate Switch. This bit defines whether the bit rate is switched inside a CAN FD format frame."]
    pub mod BRS {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Extended Data Length. This bit distinguishes between CAN format and CAN FD format frames. The EDL bit must not be set for Message Buffers configured to RANSWER with code field 0b1010."]
    pub mod EDL {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 88 ID Register"]
pub mod ID88 {
    #[doc = "Contains extended (LOW word) identifier of message buffer."]
    pub mod EXT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0003_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Contains standard/extended (HIGH word) identifier of message buffer."]
    pub mod STD {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Local priority. This 3-bit fieldis only used when LPRIO_EN bit is set in MCR and it only makes sense for Tx buffers. These bits are not transmitted. They are appended to the regular ID to define the transmission priority."]
    pub mod PRIO {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 16 WORD_16B Register"]
pub mod MB16_16B_WORD0_H {
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_3 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_2 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_1 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_0 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 16 WORD_16B Register"]
pub mod MB16_16B_WORD1_H {
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_7 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_6 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_5 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_4 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 89 CS Register"]
pub mod CS89 {
    #[doc = "Free-Running Counter Time stamp. This 16-bit field is a copy of the Free-Running Timer, captured for Tx and Rx frames at the time when the beginning of the Identifier field appears on the CAN bus."]
    pub mod TIME_STAMP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Length of the data to be stored/transmitted."]
    pub mod DLC {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Remote Transmission Request. One/zero for remote/data frame."]
    pub mod RTR {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ID Extended. One/zero for extended/standard format frame."]
    pub mod IDE {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Substitute Remote Request. Contains a fixed recessive bit."]
    pub mod SRR {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Message Buffer Code. This 4-bit field can be accessed (read or write) by the CPU and by the FlexCAN module itself, as part of the message buffer matching and arbitration process."]
    pub mod CODE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Error State Indicator. This bit indicates if the transmitting node is error active or error passive."]
    pub mod ESI {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Bit Rate Switch. This bit defines whether the bit rate is switched inside a CAN FD format frame."]
    pub mod BRS {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Extended Data Length. This bit distinguishes between CAN format and CAN FD format frames. The EDL bit must not be set for Message Buffers configured to RANSWER with code field 0b1010."]
    pub mod EDL {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 89 ID Register"]
pub mod ID89 {
    #[doc = "Contains extended (LOW word) identifier of message buffer."]
    pub mod EXT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0003_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Contains standard/extended (HIGH word) identifier of message buffer."]
    pub mod STD {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Local priority. This 3-bit fieldis only used when LPRIO_EN bit is set in MCR and it only makes sense for Tx buffers. These bits are not transmitted. They are appended to the regular ID to define the transmission priority."]
    pub mod PRIO {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 10 WORD_32B Register"]
pub mod MB10_32B_WORD0_H {
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_3 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_2 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_1 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_0 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 10 WORD_32B Register"]
pub mod MB10_32B_WORD1_H {
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_7 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_6 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_5 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_4 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 90 CS Register"]
pub mod CS90 {
    #[doc = "Free-Running Counter Time stamp. This 16-bit field is a copy of the Free-Running Timer, captured for Tx and Rx frames at the time when the beginning of the Identifier field appears on the CAN bus."]
    pub mod TIME_STAMP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Length of the data to be stored/transmitted."]
    pub mod DLC {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Remote Transmission Request. One/zero for remote/data frame."]
    pub mod RTR {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ID Extended. One/zero for extended/standard format frame."]
    pub mod IDE {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Substitute Remote Request. Contains a fixed recessive bit."]
    pub mod SRR {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Message Buffer Code. This 4-bit field can be accessed (read or write) by the CPU and by the FlexCAN module itself, as part of the message buffer matching and arbitration process."]
    pub mod CODE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Error State Indicator. This bit indicates if the transmitting node is error active or error passive."]
    pub mod ESI {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Bit Rate Switch. This bit defines whether the bit rate is switched inside a CAN FD format frame."]
    pub mod BRS {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Extended Data Length. This bit distinguishes between CAN format and CAN FD format frames. The EDL bit must not be set for Message Buffers configured to RANSWER with code field 0b1010."]
    pub mod EDL {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 90 ID Register"]
pub mod ID90 {
    #[doc = "Contains extended (LOW word) identifier of message buffer."]
    pub mod EXT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0003_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Contains standard/extended (HIGH word) identifier of message buffer."]
    pub mod STD {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Local priority. This 3-bit fieldis only used when LPRIO_EN bit is set in MCR and it only makes sense for Tx buffers. These bits are not transmitted. They are appended to the regular ID to define the transmission priority."]
    pub mod PRIO {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 10 WORD_32B Register"]
pub mod MB10_32B_WORD4_H {
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_19 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_18 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_17 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_16 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 10 WORD_32B Register"]
pub mod MB10_32B_WORD5_H {
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_23 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_22 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_21 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_20 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 91 CS Register"]
pub mod CS91 {
    #[doc = "Free-Running Counter Time stamp. This 16-bit field is a copy of the Free-Running Timer, captured for Tx and Rx frames at the time when the beginning of the Identifier field appears on the CAN bus."]
    pub mod TIME_STAMP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Length of the data to be stored/transmitted."]
    pub mod DLC {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Remote Transmission Request. One/zero for remote/data frame."]
    pub mod RTR {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ID Extended. One/zero for extended/standard format frame."]
    pub mod IDE {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Substitute Remote Request. Contains a fixed recessive bit."]
    pub mod SRR {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Message Buffer Code. This 4-bit field can be accessed (read or write) by the CPU and by the FlexCAN module itself, as part of the message buffer matching and arbitration process."]
    pub mod CODE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Error State Indicator. This bit indicates if the transmitting node is error active or error passive."]
    pub mod ESI {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Bit Rate Switch. This bit defines whether the bit rate is switched inside a CAN FD format frame."]
    pub mod BRS {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Extended Data Length. This bit distinguishes between CAN format and CAN FD format frames. The EDL bit must not be set for Message Buffers configured to RANSWER with code field 0b1010."]
    pub mod EDL {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 91 ID Register"]
pub mod ID91 {
    #[doc = "Contains extended (LOW word) identifier of message buffer."]
    pub mod EXT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0003_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Contains standard/extended (HIGH word) identifier of message buffer."]
    pub mod STD {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Local priority. This 3-bit fieldis only used when LPRIO_EN bit is set in MCR and it only makes sense for Tx buffers. These bits are not transmitted. They are appended to the regular ID to define the transmission priority."]
    pub mod PRIO {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 11 CS Register"]
pub mod MB11_32B_CS_H {
    #[doc = "Free-Running Counter Time stamp. This 16-bit field is a copy of the Free-Running Timer, captured for Tx and Rx frames at the time when the beginning of the Identifier field appears on the CAN bus."]
    pub mod TIME_STAMP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Length of the data to be stored/transmitted."]
    pub mod DLC {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Remote Transmission Request. One/zero for remote/data frame."]
    pub mod RTR {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ID Extended. One/zero for extended/standard format frame."]
    pub mod IDE {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Substitute Remote Request. Contains a fixed recessive bit."]
    pub mod SRR {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Message Buffer Code. This 4-bit field can be accessed (read or write) by the CPU and by the FlexCAN module itself, as part of the message buffer matching and arbitration process."]
    pub mod CODE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Error State Indicator. This bit indicates if the transmitting node is error active or error passive."]
    pub mod ESI {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Bit Rate Switch. This bit defines whether the bit rate is switched inside a CAN FD format frame."]
    pub mod BRS {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Extended Data Length. This bit distinguishes between CAN format and CAN FD format frames. The EDL bit must not be set for Message Buffers configured to RANSWER with code field 0b1010."]
    pub mod EDL {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 11 ID Register"]
pub mod MB11_32B_ID_H {
    #[doc = "Contains extended (LOW word) identifier of message buffer."]
    pub mod EXT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0003_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Contains standard/extended (HIGH word) identifier of message buffer."]
    pub mod STD {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Local priority. This 3-bit fieldis only used when LPRIO_EN bit is set in MCR and it only makes sense for Tx buffers. These bits are not transmitted. They are appended to the regular ID to define the transmission priority."]
    pub mod PRIO {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 92 CS Register"]
pub mod CS92 {
    #[doc = "Free-Running Counter Time stamp. This 16-bit field is a copy of the Free-Running Timer, captured for Tx and Rx frames at the time when the beginning of the Identifier field appears on the CAN bus."]
    pub mod TIME_STAMP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Length of the data to be stored/transmitted."]
    pub mod DLC {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Remote Transmission Request. One/zero for remote/data frame."]
    pub mod RTR {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ID Extended. One/zero for extended/standard format frame."]
    pub mod IDE {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Substitute Remote Request. Contains a fixed recessive bit."]
    pub mod SRR {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Message Buffer Code. This 4-bit field can be accessed (read or write) by the CPU and by the FlexCAN module itself, as part of the message buffer matching and arbitration process."]
    pub mod CODE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Error State Indicator. This bit indicates if the transmitting node is error active or error passive."]
    pub mod ESI {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Bit Rate Switch. This bit defines whether the bit rate is switched inside a CAN FD format frame."]
    pub mod BRS {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Extended Data Length. This bit distinguishes between CAN format and CAN FD format frames. The EDL bit must not be set for Message Buffers configured to RANSWER with code field 0b1010."]
    pub mod EDL {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 92 ID Register"]
pub mod ID92 {
    #[doc = "Contains extended (LOW word) identifier of message buffer."]
    pub mod EXT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0003_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Contains standard/extended (HIGH word) identifier of message buffer."]
    pub mod STD {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Local priority. This 3-bit fieldis only used when LPRIO_EN bit is set in MCR and it only makes sense for Tx buffers. These bits are not transmitted. They are appended to the regular ID to define the transmission priority."]
    pub mod PRIO {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 11 WORD_32B Register"]
pub mod MB11_32B_WORD2_H {
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_11 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_10 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_9 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_8 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 11 WORD_32B Register"]
pub mod MB11_32B_WORD3_H {
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_15 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_14 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_13 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_12 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 93 CS Register"]
pub mod CS93 {
    #[doc = "Free-Running Counter Time stamp. This 16-bit field is a copy of the Free-Running Timer, captured for Tx and Rx frames at the time when the beginning of the Identifier field appears on the CAN bus."]
    pub mod TIME_STAMP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Length of the data to be stored/transmitted."]
    pub mod DLC {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Remote Transmission Request. One/zero for remote/data frame."]
    pub mod RTR {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ID Extended. One/zero for extended/standard format frame."]
    pub mod IDE {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Substitute Remote Request. Contains a fixed recessive bit."]
    pub mod SRR {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Message Buffer Code. This 4-bit field can be accessed (read or write) by the CPU and by the FlexCAN module itself, as part of the message buffer matching and arbitration process."]
    pub mod CODE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Error State Indicator. This bit indicates if the transmitting node is error active or error passive."]
    pub mod ESI {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Bit Rate Switch. This bit defines whether the bit rate is switched inside a CAN FD format frame."]
    pub mod BRS {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Extended Data Length. This bit distinguishes between CAN format and CAN FD format frames. The EDL bit must not be set for Message Buffers configured to RANSWER with code field 0b1010."]
    pub mod EDL {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 93 ID Register"]
pub mod ID93 {
    #[doc = "Contains extended (LOW word) identifier of message buffer."]
    pub mod EXT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0003_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Contains standard/extended (HIGH word) identifier of message buffer."]
    pub mod STD {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Local priority. This 3-bit fieldis only used when LPRIO_EN bit is set in MCR and it only makes sense for Tx buffers. These bits are not transmitted. They are appended to the regular ID to define the transmission priority."]
    pub mod PRIO {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 11 WORD_32B Register"]
pub mod MB11_32B_WORD6_H {
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_27 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_26 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_25 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_24 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 11 WORD_32B Register"]
pub mod MB11_32B_WORD7_H {
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_31 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_30 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_29 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_28 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 94 CS Register"]
pub mod CS94 {
    #[doc = "Free-Running Counter Time stamp. This 16-bit field is a copy of the Free-Running Timer, captured for Tx and Rx frames at the time when the beginning of the Identifier field appears on the CAN bus."]
    pub mod TIME_STAMP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Length of the data to be stored/transmitted."]
    pub mod DLC {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Remote Transmission Request. One/zero for remote/data frame."]
    pub mod RTR {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ID Extended. One/zero for extended/standard format frame."]
    pub mod IDE {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Substitute Remote Request. Contains a fixed recessive bit."]
    pub mod SRR {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Message Buffer Code. This 4-bit field can be accessed (read or write) by the CPU and by the FlexCAN module itself, as part of the message buffer matching and arbitration process."]
    pub mod CODE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Error State Indicator. This bit indicates if the transmitting node is error active or error passive."]
    pub mod ESI {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Bit Rate Switch. This bit defines whether the bit rate is switched inside a CAN FD format frame."]
    pub mod BRS {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Extended Data Length. This bit distinguishes between CAN format and CAN FD format frames. The EDL bit must not be set for Message Buffers configured to RANSWER with code field 0b1010."]
    pub mod EDL {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 94 ID Register"]
pub mod ID94 {
    #[doc = "Contains extended (LOW word) identifier of message buffer."]
    pub mod EXT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0003_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Contains standard/extended (HIGH word) identifier of message buffer."]
    pub mod STD {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Local priority. This 3-bit fieldis only used when LPRIO_EN bit is set in MCR and it only makes sense for Tx buffers. These bits are not transmitted. They are appended to the regular ID to define the transmission priority."]
    pub mod PRIO {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 20 WORD_16B Register"]
pub mod MB20_16B_WORD0_H {
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_3 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_2 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_1 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_0 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 20 WORD_16B Register"]
pub mod MB20_16B_WORD1_H {
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_7 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_6 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_5 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_4 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 95 CS Register"]
pub mod CS95 {
    #[doc = "Free-Running Counter Time stamp. This 16-bit field is a copy of the Free-Running Timer, captured for Tx and Rx frames at the time when the beginning of the Identifier field appears on the CAN bus."]
    pub mod TIME_STAMP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Length of the data to be stored/transmitted."]
    pub mod DLC {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Remote Transmission Request. One/zero for remote/data frame."]
    pub mod RTR {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ID Extended. One/zero for extended/standard format frame."]
    pub mod IDE {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Substitute Remote Request. Contains a fixed recessive bit."]
    pub mod SRR {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Message Buffer Code. This 4-bit field can be accessed (read or write) by the CPU and by the FlexCAN module itself, as part of the message buffer matching and arbitration process."]
    pub mod CODE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Error State Indicator. This bit indicates if the transmitting node is error active or error passive."]
    pub mod ESI {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Bit Rate Switch. This bit defines whether the bit rate is switched inside a CAN FD format frame."]
    pub mod BRS {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Extended Data Length. This bit distinguishes between CAN format and CAN FD format frames. The EDL bit must not be set for Message Buffers configured to RANSWER with code field 0b1010."]
    pub mod EDL {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 95 ID Register"]
pub mod ID95 {
    #[doc = "Contains extended (LOW word) identifier of message buffer."]
    pub mod EXT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0003_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Contains standard/extended (HIGH word) identifier of message buffer."]
    pub mod STD {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Local priority. This 3-bit fieldis only used when LPRIO_EN bit is set in MCR and it only makes sense for Tx buffers. These bits are not transmitted. They are appended to the regular ID to define the transmission priority."]
    pub mod PRIO {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 95 WORD_8B Register"]
pub mod MB95_8B_WORD0 {
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_3 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_2 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_1 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_0 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 95 WORD_8B Register"]
pub mod MB95_8B_WORD1 {
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_7 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_6 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_5 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_4 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Receive Individual Mask"]
pub mod RXIMR {
    #[doc = "Individual Mask Bits"]
    pub mod MI {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Memory Error Control"]
pub mod MECR {
    #[doc = "Non-Correctable Errors In FlexCAN Access Put Device In Freeze Mode"]
    pub mod NCEFAFRZ {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Keep normal operation."]
            pub const NORMAL: u32 = 0;
            #[doc = "Put FlexCAN in Freeze mode (see section \"Freeze mode\")."]
            pub const FREEZE: u32 = 0x01;
        }
    }
    #[doc = "Error Correction Disable"]
    pub mod ECCDIS {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Enable memory error correction."]
            pub const ENABLE: u32 = 0;
            #[doc = "Disable memory error correction."]
            pub const DISABLE: u32 = 0x01;
        }
    }
    #[doc = "Error Report Disable"]
    pub mod RERRDIS {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Enable updates of the error report registers."]
            pub const ENABLE: u32 = 0;
            #[doc = "Disable updates of the error report registers."]
            pub const DISABLE: u32 = 0x01;
        }
    }
    #[doc = "Extended Error Injection Enable"]
    pub mod EXTERRIE {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Error injection is applied only to the 32-bit word."]
            pub const INJECT_32_BIT: u32 = 0;
            #[doc = "Error injection is applied to the 64-bit word."]
            pub const INJECT_64_BIT: u32 = 0x01;
        }
    }
    #[doc = "FlexCAN Access Error Injection Enable"]
    pub mod FAERRIE {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Injection is disabled."]
            pub const DISABLE: u32 = 0;
            #[doc = "Injection is enabled."]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Host Access Error Injection Enable"]
    pub mod HAERRIE {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Injection is disabled."]
            pub const DISABLE: u32 = 0;
            #[doc = "Injection is enabled."]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Correctable Errors Interrupt Mask"]
    pub mod CEI_MSK {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt is disabled."]
            pub const DISABLE: u32 = 0;
            #[doc = "Interrupt is enabled."]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "FlexCAN Access With Non-Correctable Errors Interrupt Mask"]
    pub mod FANCEI_MSK {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt is disabled."]
            pub const DISABLE: u32 = 0;
            #[doc = "Interrupt is enabled."]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Host Access With Non-Correctable Errors Interrupt Mask"]
    pub mod HANCEI_MSK {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt is disabled."]
            pub const DISABLE: u32 = 0;
            #[doc = "Interrupt is enabled."]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Error Configuration Register Write Disable"]
    pub mod ECRWRDIS {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Write is enabled."]
            pub const ENABLE: u32 = 0;
            #[doc = "Write is disabled."]
            pub const DISABLE: u32 = 0x01;
        }
    }
}
#[doc = "Error Injection Address"]
pub mod ERRIAR {
    #[doc = "Error Injection Address Low"]
    pub mod INJADDR_L {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Error Injection Address High"]
    pub mod INJADDR_H {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Error Injection Data Pattern"]
pub mod ERRIDPR {
    #[doc = "Data flip pattern"]
    pub mod DFLIP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Error Injection Parity Pattern"]
pub mod ERRIPPR {
    #[doc = "Parity Flip Pattern For Byte 0 (Least Significant)"]
    pub mod PFLIP0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Parity Flip Pattern For Byte 1"]
    pub mod PFLIP1 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Parity Flip Pattern For Byte 2"]
    pub mod PFLIP2 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Parity Flip Pattern For Byte 3 (most significant)"]
    pub mod PFLIP3 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Error Report Address"]
pub mod RERRAR {
    #[doc = "Address Where Error Detected"]
    pub mod ERRADDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "SAID"]
    pub mod SAID {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Non-Correctable Error"]
    pub mod NCE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Reporting a correctable error"]
            pub const CORRECTABLE: u32 = 0;
            #[doc = "Reporting a non-correctable error"]
            pub const NON_CORRECTABLE: u32 = 0x01;
        }
    }
}
#[doc = "Error Report Data"]
pub mod RERRDR {
    #[doc = "Raw data word read from memory with error"]
    pub mod RDATA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Error Report Syndrome"]
pub mod RERRSYNR {
    #[doc = "Error Syndrome For Byte 0 (least significant)"]
    pub mod SYND0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Byte Enabled For Byte 0 (least significant)"]
    pub mod BE0 {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The byte was not read."]
            pub const NOT_READ: u32 = 0;
            #[doc = "The byte was read."]
            pub const READ: u32 = 0x01;
        }
    }
    #[doc = "Error Syndrome for Byte 1"]
    pub mod SYND1 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Byte Enabled For Byte 1"]
    pub mod BE1 {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The byte was not read."]
            pub const NOT_READ: u32 = 0;
            #[doc = "The byte was read."]
            pub const READ: u32 = 0x01;
        }
    }
    #[doc = "Error Syndrome For Byte 2"]
    pub mod SYND2 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Byte Enabled For Byte 2"]
    pub mod BE2 {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The byte was not read."]
            pub const NOT_READ: u32 = 0;
            #[doc = "The byte was read."]
            pub const READ: u32 = 0x01;
        }
    }
    #[doc = "Error Syndrome For Byte 3 (most significant)"]
    pub mod SYND3 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Byte Enabled For Byte 3 (most significant)"]
    pub mod BE3 {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The byte was not read."]
            pub const NOT_READ: u32 = 0;
            #[doc = "The byte was read."]
            pub const READ: u32 = 0x01;
        }
    }
}
#[doc = "Error Status"]
pub mod ERRSR {
    #[doc = "Correctable Error Interrupt Overrun Flag"]
    pub mod CEIOF {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No overrun on correctable errors"]
            pub const NO_OVERRUN: u32 = 0;
            #[doc = "Overrun on correctable errors"]
            pub const OVERRUN: u32 = 0x01;
        }
    }
    #[doc = "FlexCAN Access With Non-Correctable Error Interrupt Overrun Flag"]
    pub mod FANCEIOF {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No overrun on non-correctable errors in FlexCAN access"]
            pub const NO_OVERRUN: u32 = 0;
            #[doc = "Overrun on non-correctable errors in FlexCAN access"]
            pub const OVERRUN: u32 = 0x01;
        }
    }
    #[doc = "Host Access With Non-Correctable Error Interrupt Overrun Flag"]
    pub mod HANCEIOF {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No overrun on non-correctable errors in host access"]
            pub const NO_OVERRUN: u32 = 0;
            #[doc = "Overrun on non-correctable errors in host access"]
            pub const OVERRUN: u32 = 0x01;
        }
    }
    #[doc = "Correctable Error Interrupt Flag"]
    pub mod CEIF {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No correctable errors were detected so far."]
            pub const NO_ERRORS: u32 = 0;
            #[doc = "A correctable error was detected."]
            pub const ERRORS: u32 = 0x01;
        }
    }
    #[doc = "FlexCAN Access With Non-Correctable Error Interrupt Flag"]
    pub mod FANCEIF {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No non-correctable errors were detected in FlexCAN accesses so far."]
            pub const NOT_FOUND: u32 = 0;
            #[doc = "A non-correctable error was detected in a FlexCAN access."]
            pub const FOUND: u32 = 0x01;
        }
    }
    #[doc = "Host Access With Non-Correctable Error Interrupt Flag"]
    pub mod HANCEIF {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No non-correctable errors were detected in host accesses so far."]
            pub const NOT_FOUND: u32 = 0;
            #[doc = "A non-correctable error was detected in a host access."]
            pub const FOUND: u32 = 0x01;
        }
    }
}
#[doc = "Enhanced CAN Bit Timing Prescalers"]
pub mod EPRS {
    #[doc = "Extended Nominal Prescaler Division Factor"]
    pub mod ENPRESDIV {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Extended Data Phase Prescaler Division Factor"]
    pub mod EDPRESDIV {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x03ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Enhanced Nominal CAN Bit Timing"]
pub mod ENCBT {
    #[doc = "Nominal Time Segment 1"]
    pub mod NTSEG1 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Nominal Time Segment 2"]
    pub mod NTSEG2 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x7f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Nominal Resynchronization Jump Width"]
    pub mod NRJW {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x7f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Enhanced Data Phase CAN Bit Timing"]
pub mod EDCBT {
    #[doc = "Data Phase Segment 1"]
    pub mod DTSEG1 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data Phase Time Segment 2"]
    pub mod DTSEG2 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data Phase Resynchronization Jump Width"]
    pub mod DRJW {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Enhanced Transceiver Delay Compensation"]
pub mod ETDC {
    #[doc = "Enhanced Transceiver Delay Compensation Value"]
    pub mod ETDCVAL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Transceiver Delay Compensation Fail"]
    pub mod ETDCFAIL {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "In range"]
            pub const IN_RANGE: u32 = 0;
            #[doc = "Out of range"]
            pub const OUT_OF_RANGE: u32 = 0x01;
        }
    }
    #[doc = "Enhanced Transceiver Delay Compensation Offset"]
    pub mod ETDCOFF {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x7f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Transceiver Delay Measurement Disable"]
    pub mod TDMDIS {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Enable"]
            pub const ENABLE: u32 = 0;
            #[doc = "Disable"]
            pub const DISABLE: u32 = 0x01;
        }
    }
    #[doc = "Transceiver Delay Compensation Enable"]
    pub mod ETDCEN {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable"]
            pub const ENABLE: u32 = 0x01;
        }
    }
}
#[doc = "CAN FD Control"]
pub mod FDCTRL {
    #[doc = "Transceiver Delay Compensation Value"]
    pub mod TDCVAL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Transceiver Delay Compensation Offset"]
    pub mod TDCOFF {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Transceiver Delay Compensation Fail"]
    pub mod TDCFAIL {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Measured loop delay is in range."]
            pub const IN_RANGE: u32 = 0;
            #[doc = "Measured loop delay is out of range."]
            pub const OUT_OF_RANGE: u32 = 0x01;
        }
    }
    #[doc = "Transceiver Delay Compensation Enable"]
    pub mod TDCEN {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "TDC is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "TDC is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Message Buffer Data Size for Region 0"]
    pub mod MBDSR0 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Selects 8 bytes per message buffer."]
            pub const R0_8_BYTES: u32 = 0;
            #[doc = "Selects 16 bytes per message buffer."]
            pub const R0_16_BYTES: u32 = 0x01;
            #[doc = "Selects 32 bytes per message buffer."]
            pub const R0_32_BYTES: u32 = 0x02;
            #[doc = "Selects 64 bytes per message buffer."]
            pub const R0_64_BYTES: u32 = 0x03;
        }
    }
    #[doc = "Message Buffer Data Size for Region 1"]
    pub mod MBDSR1 {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Selects 8 bytes per message buffer."]
            pub const R1_8_BYTES: u32 = 0;
            #[doc = "Selects 16 bytes per message buffer."]
            pub const R1_16_BYTES: u32 = 0x01;
            #[doc = "Selects 32 bytes per message buffer."]
            pub const R1_32_BYTES: u32 = 0x02;
            #[doc = "Selects 64 bytes per message buffer."]
            pub const R1_64_BYTES: u32 = 0x03;
        }
    }
    #[doc = "Message Buffer Data Size for Region 2"]
    pub mod MBDSR2 {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "8 bytes"]
            pub const R2_8_BYTES: u32 = 0;
            #[doc = "16 bytes"]
            pub const R2_16_BYTES: u32 = 0x01;
            #[doc = "32 bytes"]
            pub const R2_32_BYTES: u32 = 0x02;
            #[doc = "64 bytes"]
            pub const R2_64_BYTES: u32 = 0x03;
        }
    }
    #[doc = "Bit Rate Switch Enable"]
    pub mod FDRATE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Transmit a frame in nominal rate. The BRS bit in the Tx MB has no effect."]
            pub const NOMINAL: u32 = 0;
            #[doc = "Transmit a frame with bit rate switching if the BRS bit in the Tx MB is recessive."]
            pub const BIT_RATE_SWITCHING: u32 = 0x01;
        }
    }
}
#[doc = "CAN FD Bit Timing"]
pub mod FDCBT {
    #[doc = "Fast Phase Segment 2"]
    pub mod FPSEG2 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Fast Phase Segment 1"]
    pub mod FPSEG1 {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Fast Propagation Segment"]
    pub mod FPROPSEG {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Fast Resync Jump Width"]
    pub mod FRJW {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Fast Prescaler Division Factor"]
    pub mod FPRESDIV {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x03ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "CAN FD CRC"]
pub mod FDCRC {
    #[doc = "Extended Transmitted CRC value"]
    pub mod FD_TXCRC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x001f_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CRC Mailbox Number for FD_TXCRC"]
    pub mod FD_MBCRC {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x7f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Enhanced RX FIFO Control"]
pub mod ERFCR {
    #[doc = "Enhanced RX FIFO Watermark"]
    pub mod ERFWM {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Number of Enhanced RX FIFO Filter Elements"]
    pub mod NFE {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Number of Extended ID Filter Elements"]
    pub mod NEXIF {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x7f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DMA Last Word"]
    pub mod DMALW {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enhanced RX FIFO enable"]
    pub mod ERFEN {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable"]
            pub const ENABLE: u32 = 0x01;
        }
    }
}
#[doc = "Enhanced RX FIFO Interrupt Enable"]
pub mod ERFIER {
    #[doc = "Enhanced RX FIFO Data Available Interrupt Enable"]
    pub mod ERFDAIE {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Enhanced RX FIFO Watermark Indication Interrupt Enable"]
    pub mod ERFWMIIE {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Enhanced RX FIFO Overflow Interrupt Enable"]
    pub mod ERFOVFIE {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Enhanced RX FIFO Underflow Interrupt Enable"]
    pub mod ERFUFWIE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable"]
            pub const ENABLE: u32 = 0x01;
        }
    }
}
#[doc = "Enhanced RX FIFO Status"]
pub mod ERFSR {
    #[doc = "Enhanced RX FIFO Elements"]
    pub mod ERFEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enhanced RX FIFO Full Flag"]
    pub mod ERFF {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not full"]
            pub const NOT_FULL: u32 = 0;
            #[doc = "Full"]
            pub const FULL: u32 = 0x01;
        }
    }
    #[doc = "Enhanced RX FIFO Empty Flag"]
    pub mod ERFE {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not empty"]
            pub const NOT_EMPTY: u32 = 0;
            #[doc = "Empty"]
            pub const EMPTY: u32 = 0x01;
        }
    }
    #[doc = "Enhanced RX FIFO Clear"]
    pub mod ERFCLR {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No effect"]
            pub const NO_EFFECT: u32 = 0;
            #[doc = "Clear enhanced RX FIFO content"]
            pub const CLEAR: u32 = 0x01;
        }
    }
    #[doc = "Enhanced RX FIFO Data Available Flag"]
    pub mod ERFDA {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No such occurrence"]
            pub const NO_MESSAGE_STORED: u32 = 0;
            #[doc = "At least one message stored in Enhanced RX FIFO"]
            pub const MESSAGE_STORED: u32 = 0x01;
        }
    }
    #[doc = "Enhanced RX FIFO Watermark Indication Flag"]
    pub mod ERFWMI {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No such occurrence"]
            pub const WATERMARK_NO: u32 = 0;
            #[doc = "Number of messages in FIFO is greater than the watermark"]
            pub const WATERMARK_YES: u32 = 0x01;
        }
    }
    #[doc = "Enhanced RX FIFO Overflow Flag"]
    pub mod ERFOVF {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No such occurrence"]
            pub const NO_OVERFLOW: u32 = 0;
            #[doc = "Overflow"]
            pub const OVERFLOW: u32 = 0x01;
        }
    }
    #[doc = "Enhanced RX FIFO Underflow Flag"]
    pub mod ERFUFW {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No such occurrence"]
            pub const NO_UNDERFLOW: u32 = 0;
            #[doc = "Underflow"]
            pub const UNDERFLOW: u32 = 0x01;
        }
    }
}
#[doc = "High-Resolution Timestamp"]
pub mod HR_TIME_STAMP {
    #[doc = "High-Resolution Timestamp"]
    pub mod TS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Enhanced RX FIFO Filter Element"]
pub mod ERFFEL {
    #[doc = "Filter Element Bits"]
    pub mod FEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
