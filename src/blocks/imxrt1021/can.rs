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
    _reserved3: [u8; 0x20],
    #[doc = "Message Buffer 0 CS Register"]
    pub CS0: crate::RWRegister<u32>,
    #[doc = "Message Buffer 0 ID Register"]
    pub ID0: crate::RWRegister<u32>,
    #[doc = "Message Buffer 0 WORD0 Register"]
    pub WORD00: crate::RWRegister<u32>,
    #[doc = "Message Buffer 0 WORD1 Register"]
    pub WORD10: crate::RWRegister<u32>,
    #[doc = "Message Buffer 1 CS Register"]
    pub CS1: crate::RWRegister<u32>,
    #[doc = "Message Buffer 1 ID Register"]
    pub ID1: crate::RWRegister<u32>,
    #[doc = "Message Buffer 1 WORD0 Register"]
    pub WORD01: crate::RWRegister<u32>,
    #[doc = "Message Buffer 1 WORD1 Register"]
    pub WORD11: crate::RWRegister<u32>,
    #[doc = "Message Buffer 2 CS Register"]
    pub CS2: crate::RWRegister<u32>,
    #[doc = "Message Buffer 2 ID Register"]
    pub ID2: crate::RWRegister<u32>,
    #[doc = "Message Buffer 2 WORD0 Register"]
    pub WORD02: crate::RWRegister<u32>,
    #[doc = "Message Buffer 2 WORD1 Register"]
    pub WORD12: crate::RWRegister<u32>,
    #[doc = "Message Buffer 3 CS Register"]
    pub CS3: crate::RWRegister<u32>,
    #[doc = "Message Buffer 3 ID Register"]
    pub ID3: crate::RWRegister<u32>,
    #[doc = "Message Buffer 3 WORD0 Register"]
    pub WORD03: crate::RWRegister<u32>,
    #[doc = "Message Buffer 3 WORD1 Register"]
    pub WORD13: crate::RWRegister<u32>,
    #[doc = "Message Buffer 4 CS Register"]
    pub CS4: crate::RWRegister<u32>,
    #[doc = "Message Buffer 4 ID Register"]
    pub ID4: crate::RWRegister<u32>,
    #[doc = "Message Buffer 4 WORD0 Register"]
    pub WORD04: crate::RWRegister<u32>,
    #[doc = "Message Buffer 4 WORD1 Register"]
    pub WORD14: crate::RWRegister<u32>,
    #[doc = "Message Buffer 5 CS Register"]
    pub CS5: crate::RWRegister<u32>,
    #[doc = "Message Buffer 5 ID Register"]
    pub ID5: crate::RWRegister<u32>,
    #[doc = "Message Buffer 5 WORD0 Register"]
    pub WORD05: crate::RWRegister<u32>,
    #[doc = "Message Buffer 5 WORD1 Register"]
    pub WORD15: crate::RWRegister<u32>,
    #[doc = "Message Buffer 6 CS Register"]
    pub CS6: crate::RWRegister<u32>,
    #[doc = "Message Buffer 6 ID Register"]
    pub ID6: crate::RWRegister<u32>,
    #[doc = "Message Buffer 6 WORD0 Register"]
    pub WORD06: crate::RWRegister<u32>,
    #[doc = "Message Buffer 6 WORD1 Register"]
    pub WORD16: crate::RWRegister<u32>,
    #[doc = "Message Buffer 7 CS Register"]
    pub CS7: crate::RWRegister<u32>,
    #[doc = "Message Buffer 7 ID Register"]
    pub ID7: crate::RWRegister<u32>,
    #[doc = "Message Buffer 7 WORD0 Register"]
    pub WORD07: crate::RWRegister<u32>,
    #[doc = "Message Buffer 7 WORD1 Register"]
    pub WORD17: crate::RWRegister<u32>,
    #[doc = "Message Buffer 8 CS Register"]
    pub CS8: crate::RWRegister<u32>,
    #[doc = "Message Buffer 8 ID Register"]
    pub ID8: crate::RWRegister<u32>,
    #[doc = "Message Buffer 8 WORD0 Register"]
    pub WORD08: crate::RWRegister<u32>,
    #[doc = "Message Buffer 8 WORD1 Register"]
    pub WORD18: crate::RWRegister<u32>,
    #[doc = "Message Buffer 9 CS Register"]
    pub CS9: crate::RWRegister<u32>,
    #[doc = "Message Buffer 9 ID Register"]
    pub ID9: crate::RWRegister<u32>,
    #[doc = "Message Buffer 9 WORD0 Register"]
    pub WORD09: crate::RWRegister<u32>,
    #[doc = "Message Buffer 9 WORD1 Register"]
    pub WORD19: crate::RWRegister<u32>,
    #[doc = "Message Buffer 10 CS Register"]
    pub CS10: crate::RWRegister<u32>,
    #[doc = "Message Buffer 10 ID Register"]
    pub ID10: crate::RWRegister<u32>,
    #[doc = "Message Buffer 10 WORD0 Register"]
    pub WORD010: crate::RWRegister<u32>,
    #[doc = "Message Buffer 10 WORD1 Register"]
    pub WORD110: crate::RWRegister<u32>,
    #[doc = "Message Buffer 11 CS Register"]
    pub CS11: crate::RWRegister<u32>,
    #[doc = "Message Buffer 11 ID Register"]
    pub ID11: crate::RWRegister<u32>,
    #[doc = "Message Buffer 11 WORD0 Register"]
    pub WORD011: crate::RWRegister<u32>,
    #[doc = "Message Buffer 11 WORD1 Register"]
    pub WORD111: crate::RWRegister<u32>,
    #[doc = "Message Buffer 12 CS Register"]
    pub CS12: crate::RWRegister<u32>,
    #[doc = "Message Buffer 12 ID Register"]
    pub ID12: crate::RWRegister<u32>,
    #[doc = "Message Buffer 12 WORD0 Register"]
    pub WORD012: crate::RWRegister<u32>,
    #[doc = "Message Buffer 12 WORD1 Register"]
    pub WORD112: crate::RWRegister<u32>,
    #[doc = "Message Buffer 13 CS Register"]
    pub CS13: crate::RWRegister<u32>,
    #[doc = "Message Buffer 13 ID Register"]
    pub ID13: crate::RWRegister<u32>,
    #[doc = "Message Buffer 13 WORD0 Register"]
    pub WORD013: crate::RWRegister<u32>,
    #[doc = "Message Buffer 13 WORD1 Register"]
    pub WORD113: crate::RWRegister<u32>,
    #[doc = "Message Buffer 14 CS Register"]
    pub CS14: crate::RWRegister<u32>,
    #[doc = "Message Buffer 14 ID Register"]
    pub ID14: crate::RWRegister<u32>,
    #[doc = "Message Buffer 14 WORD0 Register"]
    pub WORD014: crate::RWRegister<u32>,
    #[doc = "Message Buffer 14 WORD1 Register"]
    pub WORD114: crate::RWRegister<u32>,
    #[doc = "Message Buffer 15 CS Register"]
    pub CS15: crate::RWRegister<u32>,
    #[doc = "Message Buffer 15 ID Register"]
    pub ID15: crate::RWRegister<u32>,
    #[doc = "Message Buffer 15 WORD0 Register"]
    pub WORD015: crate::RWRegister<u32>,
    #[doc = "Message Buffer 15 WORD1 Register"]
    pub WORD115: crate::RWRegister<u32>,
    #[doc = "Message Buffer 16 CS Register"]
    pub CS16: crate::RWRegister<u32>,
    #[doc = "Message Buffer 16 ID Register"]
    pub ID16: crate::RWRegister<u32>,
    #[doc = "Message Buffer 16 WORD0 Register"]
    pub WORD016: crate::RWRegister<u32>,
    #[doc = "Message Buffer 16 WORD1 Register"]
    pub WORD116: crate::RWRegister<u32>,
    #[doc = "Message Buffer 17 CS Register"]
    pub CS17: crate::RWRegister<u32>,
    #[doc = "Message Buffer 17 ID Register"]
    pub ID17: crate::RWRegister<u32>,
    #[doc = "Message Buffer 17 WORD0 Register"]
    pub WORD017: crate::RWRegister<u32>,
    #[doc = "Message Buffer 17 WORD1 Register"]
    pub WORD117: crate::RWRegister<u32>,
    #[doc = "Message Buffer 18 CS Register"]
    pub CS18: crate::RWRegister<u32>,
    #[doc = "Message Buffer 18 ID Register"]
    pub ID18: crate::RWRegister<u32>,
    #[doc = "Message Buffer 18 WORD0 Register"]
    pub WORD018: crate::RWRegister<u32>,
    #[doc = "Message Buffer 18 WORD1 Register"]
    pub WORD118: crate::RWRegister<u32>,
    #[doc = "Message Buffer 19 CS Register"]
    pub CS19: crate::RWRegister<u32>,
    #[doc = "Message Buffer 19 ID Register"]
    pub ID19: crate::RWRegister<u32>,
    #[doc = "Message Buffer 19 WORD0 Register"]
    pub WORD019: crate::RWRegister<u32>,
    #[doc = "Message Buffer 19 WORD1 Register"]
    pub WORD119: crate::RWRegister<u32>,
    #[doc = "Message Buffer 20 CS Register"]
    pub CS20: crate::RWRegister<u32>,
    #[doc = "Message Buffer 20 ID Register"]
    pub ID20: crate::RWRegister<u32>,
    #[doc = "Message Buffer 20 WORD0 Register"]
    pub WORD020: crate::RWRegister<u32>,
    #[doc = "Message Buffer 20 WORD1 Register"]
    pub WORD120: crate::RWRegister<u32>,
    #[doc = "Message Buffer 21 CS Register"]
    pub CS21: crate::RWRegister<u32>,
    #[doc = "Message Buffer 21 ID Register"]
    pub ID21: crate::RWRegister<u32>,
    #[doc = "Message Buffer 21 WORD0 Register"]
    pub WORD021: crate::RWRegister<u32>,
    #[doc = "Message Buffer 21 WORD1 Register"]
    pub WORD121: crate::RWRegister<u32>,
    #[doc = "Message Buffer 22 CS Register"]
    pub CS22: crate::RWRegister<u32>,
    #[doc = "Message Buffer 22 ID Register"]
    pub ID22: crate::RWRegister<u32>,
    #[doc = "Message Buffer 22 WORD0 Register"]
    pub WORD022: crate::RWRegister<u32>,
    #[doc = "Message Buffer 22 WORD1 Register"]
    pub WORD122: crate::RWRegister<u32>,
    #[doc = "Message Buffer 23 CS Register"]
    pub CS23: crate::RWRegister<u32>,
    #[doc = "Message Buffer 23 ID Register"]
    pub ID23: crate::RWRegister<u32>,
    #[doc = "Message Buffer 23 WORD0 Register"]
    pub WORD023: crate::RWRegister<u32>,
    #[doc = "Message Buffer 23 WORD1 Register"]
    pub WORD123: crate::RWRegister<u32>,
    #[doc = "Message Buffer 24 CS Register"]
    pub CS24: crate::RWRegister<u32>,
    #[doc = "Message Buffer 24 ID Register"]
    pub ID24: crate::RWRegister<u32>,
    #[doc = "Message Buffer 24 WORD0 Register"]
    pub WORD024: crate::RWRegister<u32>,
    #[doc = "Message Buffer 24 WORD1 Register"]
    pub WORD124: crate::RWRegister<u32>,
    #[doc = "Message Buffer 25 CS Register"]
    pub CS25: crate::RWRegister<u32>,
    #[doc = "Message Buffer 25 ID Register"]
    pub ID25: crate::RWRegister<u32>,
    #[doc = "Message Buffer 25 WORD0 Register"]
    pub WORD025: crate::RWRegister<u32>,
    #[doc = "Message Buffer 25 WORD1 Register"]
    pub WORD125: crate::RWRegister<u32>,
    #[doc = "Message Buffer 26 CS Register"]
    pub CS26: crate::RWRegister<u32>,
    #[doc = "Message Buffer 26 ID Register"]
    pub ID26: crate::RWRegister<u32>,
    #[doc = "Message Buffer 26 WORD0 Register"]
    pub WORD026: crate::RWRegister<u32>,
    #[doc = "Message Buffer 26 WORD1 Register"]
    pub WORD126: crate::RWRegister<u32>,
    #[doc = "Message Buffer 27 CS Register"]
    pub CS27: crate::RWRegister<u32>,
    #[doc = "Message Buffer 27 ID Register"]
    pub ID27: crate::RWRegister<u32>,
    #[doc = "Message Buffer 27 WORD0 Register"]
    pub WORD027: crate::RWRegister<u32>,
    #[doc = "Message Buffer 27 WORD1 Register"]
    pub WORD127: crate::RWRegister<u32>,
    #[doc = "Message Buffer 28 CS Register"]
    pub CS28: crate::RWRegister<u32>,
    #[doc = "Message Buffer 28 ID Register"]
    pub ID28: crate::RWRegister<u32>,
    #[doc = "Message Buffer 28 WORD0 Register"]
    pub WORD028: crate::RWRegister<u32>,
    #[doc = "Message Buffer 28 WORD1 Register"]
    pub WORD128: crate::RWRegister<u32>,
    #[doc = "Message Buffer 29 CS Register"]
    pub CS29: crate::RWRegister<u32>,
    #[doc = "Message Buffer 29 ID Register"]
    pub ID29: crate::RWRegister<u32>,
    #[doc = "Message Buffer 29 WORD0 Register"]
    pub WORD029: crate::RWRegister<u32>,
    #[doc = "Message Buffer 29 WORD1 Register"]
    pub WORD129: crate::RWRegister<u32>,
    #[doc = "Message Buffer 30 CS Register"]
    pub CS30: crate::RWRegister<u32>,
    #[doc = "Message Buffer 30 ID Register"]
    pub ID30: crate::RWRegister<u32>,
    #[doc = "Message Buffer 30 WORD0 Register"]
    pub WORD030: crate::RWRegister<u32>,
    #[doc = "Message Buffer 30 WORD1 Register"]
    pub WORD130: crate::RWRegister<u32>,
    #[doc = "Message Buffer 31 CS Register"]
    pub CS31: crate::RWRegister<u32>,
    #[doc = "Message Buffer 31 ID Register"]
    pub ID31: crate::RWRegister<u32>,
    #[doc = "Message Buffer 31 WORD0 Register"]
    pub WORD031: crate::RWRegister<u32>,
    #[doc = "Message Buffer 31 WORD1 Register"]
    pub WORD131: crate::RWRegister<u32>,
    #[doc = "Message Buffer 32 CS Register"]
    pub CS32: crate::RWRegister<u32>,
    #[doc = "Message Buffer 32 ID Register"]
    pub ID32: crate::RWRegister<u32>,
    #[doc = "Message Buffer 32 WORD0 Register"]
    pub WORD032: crate::RWRegister<u32>,
    #[doc = "Message Buffer 32 WORD1 Register"]
    pub WORD132: crate::RWRegister<u32>,
    #[doc = "Message Buffer 33 CS Register"]
    pub CS33: crate::RWRegister<u32>,
    #[doc = "Message Buffer 33 ID Register"]
    pub ID33: crate::RWRegister<u32>,
    #[doc = "Message Buffer 33 WORD0 Register"]
    pub WORD033: crate::RWRegister<u32>,
    #[doc = "Message Buffer 33 WORD1 Register"]
    pub WORD133: crate::RWRegister<u32>,
    #[doc = "Message Buffer 34 CS Register"]
    pub CS34: crate::RWRegister<u32>,
    #[doc = "Message Buffer 34 ID Register"]
    pub ID34: crate::RWRegister<u32>,
    #[doc = "Message Buffer 34 WORD0 Register"]
    pub WORD034: crate::RWRegister<u32>,
    #[doc = "Message Buffer 34 WORD1 Register"]
    pub WORD134: crate::RWRegister<u32>,
    #[doc = "Message Buffer 35 CS Register"]
    pub CS35: crate::RWRegister<u32>,
    #[doc = "Message Buffer 35 ID Register"]
    pub ID35: crate::RWRegister<u32>,
    #[doc = "Message Buffer 35 WORD0 Register"]
    pub WORD035: crate::RWRegister<u32>,
    #[doc = "Message Buffer 35 WORD1 Register"]
    pub WORD135: crate::RWRegister<u32>,
    #[doc = "Message Buffer 36 CS Register"]
    pub CS36: crate::RWRegister<u32>,
    #[doc = "Message Buffer 36 ID Register"]
    pub ID36: crate::RWRegister<u32>,
    #[doc = "Message Buffer 36 WORD0 Register"]
    pub WORD036: crate::RWRegister<u32>,
    #[doc = "Message Buffer 36 WORD1 Register"]
    pub WORD136: crate::RWRegister<u32>,
    #[doc = "Message Buffer 37 CS Register"]
    pub CS37: crate::RWRegister<u32>,
    #[doc = "Message Buffer 37 ID Register"]
    pub ID37: crate::RWRegister<u32>,
    #[doc = "Message Buffer 37 WORD0 Register"]
    pub WORD037: crate::RWRegister<u32>,
    #[doc = "Message Buffer 37 WORD1 Register"]
    pub WORD137: crate::RWRegister<u32>,
    #[doc = "Message Buffer 38 CS Register"]
    pub CS38: crate::RWRegister<u32>,
    #[doc = "Message Buffer 38 ID Register"]
    pub ID38: crate::RWRegister<u32>,
    #[doc = "Message Buffer 38 WORD0 Register"]
    pub WORD038: crate::RWRegister<u32>,
    #[doc = "Message Buffer 38 WORD1 Register"]
    pub WORD138: crate::RWRegister<u32>,
    #[doc = "Message Buffer 39 CS Register"]
    pub CS39: crate::RWRegister<u32>,
    #[doc = "Message Buffer 39 ID Register"]
    pub ID39: crate::RWRegister<u32>,
    #[doc = "Message Buffer 39 WORD0 Register"]
    pub WORD039: crate::RWRegister<u32>,
    #[doc = "Message Buffer 39 WORD1 Register"]
    pub WORD139: crate::RWRegister<u32>,
    #[doc = "Message Buffer 40 CS Register"]
    pub CS40: crate::RWRegister<u32>,
    #[doc = "Message Buffer 40 ID Register"]
    pub ID40: crate::RWRegister<u32>,
    #[doc = "Message Buffer 40 WORD0 Register"]
    pub WORD040: crate::RWRegister<u32>,
    #[doc = "Message Buffer 40 WORD1 Register"]
    pub WORD140: crate::RWRegister<u32>,
    #[doc = "Message Buffer 41 CS Register"]
    pub CS41: crate::RWRegister<u32>,
    #[doc = "Message Buffer 41 ID Register"]
    pub ID41: crate::RWRegister<u32>,
    #[doc = "Message Buffer 41 WORD0 Register"]
    pub WORD041: crate::RWRegister<u32>,
    #[doc = "Message Buffer 41 WORD1 Register"]
    pub WORD141: crate::RWRegister<u32>,
    #[doc = "Message Buffer 42 CS Register"]
    pub CS42: crate::RWRegister<u32>,
    #[doc = "Message Buffer 42 ID Register"]
    pub ID42: crate::RWRegister<u32>,
    #[doc = "Message Buffer 42 WORD0 Register"]
    pub WORD042: crate::RWRegister<u32>,
    #[doc = "Message Buffer 42 WORD1 Register"]
    pub WORD142: crate::RWRegister<u32>,
    #[doc = "Message Buffer 43 CS Register"]
    pub CS43: crate::RWRegister<u32>,
    #[doc = "Message Buffer 43 ID Register"]
    pub ID43: crate::RWRegister<u32>,
    #[doc = "Message Buffer 43 WORD0 Register"]
    pub WORD043: crate::RWRegister<u32>,
    #[doc = "Message Buffer 43 WORD1 Register"]
    pub WORD143: crate::RWRegister<u32>,
    #[doc = "Message Buffer 44 CS Register"]
    pub CS44: crate::RWRegister<u32>,
    #[doc = "Message Buffer 44 ID Register"]
    pub ID44: crate::RWRegister<u32>,
    #[doc = "Message Buffer 44 WORD0 Register"]
    pub WORD044: crate::RWRegister<u32>,
    #[doc = "Message Buffer 44 WORD1 Register"]
    pub WORD144: crate::RWRegister<u32>,
    #[doc = "Message Buffer 45 CS Register"]
    pub CS45: crate::RWRegister<u32>,
    #[doc = "Message Buffer 45 ID Register"]
    pub ID45: crate::RWRegister<u32>,
    #[doc = "Message Buffer 45 WORD0 Register"]
    pub WORD045: crate::RWRegister<u32>,
    #[doc = "Message Buffer 45 WORD1 Register"]
    pub WORD145: crate::RWRegister<u32>,
    #[doc = "Message Buffer 46 CS Register"]
    pub CS46: crate::RWRegister<u32>,
    #[doc = "Message Buffer 46 ID Register"]
    pub ID46: crate::RWRegister<u32>,
    #[doc = "Message Buffer 46 WORD0 Register"]
    pub WORD046: crate::RWRegister<u32>,
    #[doc = "Message Buffer 46 WORD1 Register"]
    pub WORD146: crate::RWRegister<u32>,
    #[doc = "Message Buffer 47 CS Register"]
    pub CS47: crate::RWRegister<u32>,
    #[doc = "Message Buffer 47 ID Register"]
    pub ID47: crate::RWRegister<u32>,
    #[doc = "Message Buffer 47 WORD0 Register"]
    pub WORD047: crate::RWRegister<u32>,
    #[doc = "Message Buffer 47 WORD1 Register"]
    pub WORD147: crate::RWRegister<u32>,
    #[doc = "Message Buffer 48 CS Register"]
    pub CS48: crate::RWRegister<u32>,
    #[doc = "Message Buffer 48 ID Register"]
    pub ID48: crate::RWRegister<u32>,
    #[doc = "Message Buffer 48 WORD0 Register"]
    pub WORD048: crate::RWRegister<u32>,
    #[doc = "Message Buffer 48 WORD1 Register"]
    pub WORD148: crate::RWRegister<u32>,
    #[doc = "Message Buffer 49 CS Register"]
    pub CS49: crate::RWRegister<u32>,
    #[doc = "Message Buffer 49 ID Register"]
    pub ID49: crate::RWRegister<u32>,
    #[doc = "Message Buffer 49 WORD0 Register"]
    pub WORD049: crate::RWRegister<u32>,
    #[doc = "Message Buffer 49 WORD1 Register"]
    pub WORD149: crate::RWRegister<u32>,
    #[doc = "Message Buffer 50 CS Register"]
    pub CS50: crate::RWRegister<u32>,
    #[doc = "Message Buffer 50 ID Register"]
    pub ID50: crate::RWRegister<u32>,
    #[doc = "Message Buffer 50 WORD0 Register"]
    pub WORD050: crate::RWRegister<u32>,
    #[doc = "Message Buffer 50 WORD1 Register"]
    pub WORD150: crate::RWRegister<u32>,
    #[doc = "Message Buffer 51 CS Register"]
    pub CS51: crate::RWRegister<u32>,
    #[doc = "Message Buffer 51 ID Register"]
    pub ID51: crate::RWRegister<u32>,
    #[doc = "Message Buffer 51 WORD0 Register"]
    pub WORD051: crate::RWRegister<u32>,
    #[doc = "Message Buffer 51 WORD1 Register"]
    pub WORD151: crate::RWRegister<u32>,
    #[doc = "Message Buffer 52 CS Register"]
    pub CS52: crate::RWRegister<u32>,
    #[doc = "Message Buffer 52 ID Register"]
    pub ID52: crate::RWRegister<u32>,
    #[doc = "Message Buffer 52 WORD0 Register"]
    pub WORD052: crate::RWRegister<u32>,
    #[doc = "Message Buffer 52 WORD1 Register"]
    pub WORD152: crate::RWRegister<u32>,
    #[doc = "Message Buffer 53 CS Register"]
    pub CS53: crate::RWRegister<u32>,
    #[doc = "Message Buffer 53 ID Register"]
    pub ID53: crate::RWRegister<u32>,
    #[doc = "Message Buffer 53 WORD0 Register"]
    pub WORD053: crate::RWRegister<u32>,
    #[doc = "Message Buffer 53 WORD1 Register"]
    pub WORD153: crate::RWRegister<u32>,
    #[doc = "Message Buffer 54 CS Register"]
    pub CS54: crate::RWRegister<u32>,
    #[doc = "Message Buffer 54 ID Register"]
    pub ID54: crate::RWRegister<u32>,
    #[doc = "Message Buffer 54 WORD0 Register"]
    pub WORD054: crate::RWRegister<u32>,
    #[doc = "Message Buffer 54 WORD1 Register"]
    pub WORD154: crate::RWRegister<u32>,
    #[doc = "Message Buffer 55 CS Register"]
    pub CS55: crate::RWRegister<u32>,
    #[doc = "Message Buffer 55 ID Register"]
    pub ID55: crate::RWRegister<u32>,
    #[doc = "Message Buffer 55 WORD0 Register"]
    pub WORD055: crate::RWRegister<u32>,
    #[doc = "Message Buffer 55 WORD1 Register"]
    pub WORD155: crate::RWRegister<u32>,
    #[doc = "Message Buffer 56 CS Register"]
    pub CS56: crate::RWRegister<u32>,
    #[doc = "Message Buffer 56 ID Register"]
    pub ID56: crate::RWRegister<u32>,
    #[doc = "Message Buffer 56 WORD0 Register"]
    pub WORD056: crate::RWRegister<u32>,
    #[doc = "Message Buffer 56 WORD1 Register"]
    pub WORD156: crate::RWRegister<u32>,
    #[doc = "Message Buffer 57 CS Register"]
    pub CS57: crate::RWRegister<u32>,
    #[doc = "Message Buffer 57 ID Register"]
    pub ID57: crate::RWRegister<u32>,
    #[doc = "Message Buffer 57 WORD0 Register"]
    pub WORD057: crate::RWRegister<u32>,
    #[doc = "Message Buffer 57 WORD1 Register"]
    pub WORD157: crate::RWRegister<u32>,
    #[doc = "Message Buffer 58 CS Register"]
    pub CS58: crate::RWRegister<u32>,
    #[doc = "Message Buffer 58 ID Register"]
    pub ID58: crate::RWRegister<u32>,
    #[doc = "Message Buffer 58 WORD0 Register"]
    pub WORD058: crate::RWRegister<u32>,
    #[doc = "Message Buffer 58 WORD1 Register"]
    pub WORD158: crate::RWRegister<u32>,
    #[doc = "Message Buffer 59 CS Register"]
    pub CS59: crate::RWRegister<u32>,
    #[doc = "Message Buffer 59 ID Register"]
    pub ID59: crate::RWRegister<u32>,
    #[doc = "Message Buffer 59 WORD0 Register"]
    pub WORD059: crate::RWRegister<u32>,
    #[doc = "Message Buffer 59 WORD1 Register"]
    pub WORD159: crate::RWRegister<u32>,
    #[doc = "Message Buffer 60 CS Register"]
    pub CS60: crate::RWRegister<u32>,
    #[doc = "Message Buffer 60 ID Register"]
    pub ID60: crate::RWRegister<u32>,
    #[doc = "Message Buffer 60 WORD0 Register"]
    pub WORD060: crate::RWRegister<u32>,
    #[doc = "Message Buffer 60 WORD1 Register"]
    pub WORD160: crate::RWRegister<u32>,
    #[doc = "Message Buffer 61 CS Register"]
    pub CS61: crate::RWRegister<u32>,
    #[doc = "Message Buffer 61 ID Register"]
    pub ID61: crate::RWRegister<u32>,
    #[doc = "Message Buffer 61 WORD0 Register"]
    pub WORD061: crate::RWRegister<u32>,
    #[doc = "Message Buffer 61 WORD1 Register"]
    pub WORD161: crate::RWRegister<u32>,
    #[doc = "Message Buffer 62 CS Register"]
    pub CS62: crate::RWRegister<u32>,
    #[doc = "Message Buffer 62 ID Register"]
    pub ID62: crate::RWRegister<u32>,
    #[doc = "Message Buffer 62 WORD0 Register"]
    pub WORD062: crate::RWRegister<u32>,
    #[doc = "Message Buffer 62 WORD1 Register"]
    pub WORD162: crate::RWRegister<u32>,
    #[doc = "Message Buffer 63 CS Register"]
    pub CS63: crate::RWRegister<u32>,
    #[doc = "Message Buffer 63 ID Register"]
    pub ID63: crate::RWRegister<u32>,
    #[doc = "Message Buffer 63 WORD0 Register"]
    pub WORD063: crate::RWRegister<u32>,
    #[doc = "Message Buffer 63 WORD1 Register"]
    pub WORD163: crate::RWRegister<u32>,
    _reserved4: [u8; 0x0400],
    #[doc = "Rx Individual Mask Registers"]
    pub RXIMR: [crate::RWRegister<u32>; 64usize],
    _reserved5: [u8; 0x60],
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
    #[doc = "Reserved"]
    pub mod CODE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
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
#[doc = "Message Buffer 0 WORD0 Register"]
pub mod WORD00 {
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_3 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_2 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_1 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_0 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 0 WORD1 Register"]
pub mod WORD10 {
    #[doc = "Data byte 7 of Rx/Tx frame."]
    pub mod DATA_BYTE_7 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 6 of Rx/Tx frame."]
    pub mod DATA_BYTE_6 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 5 of Rx/Tx frame."]
    pub mod DATA_BYTE_5 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 4 of Rx/Tx frame."]
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
    #[doc = "Reserved"]
    pub mod CODE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
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
#[doc = "Message Buffer 1 WORD0 Register"]
pub mod WORD01 {
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_3 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_2 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_1 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_0 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 1 WORD1 Register"]
pub mod WORD11 {
    #[doc = "Data byte 7 of Rx/Tx frame."]
    pub mod DATA_BYTE_7 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 6 of Rx/Tx frame."]
    pub mod DATA_BYTE_6 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 5 of Rx/Tx frame."]
    pub mod DATA_BYTE_5 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 4 of Rx/Tx frame."]
    pub mod DATA_BYTE_4 {
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
    #[doc = "Reserved"]
    pub mod CODE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
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
#[doc = "Message Buffer 2 WORD0 Register"]
pub mod WORD02 {
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_3 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_2 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_1 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_0 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 2 WORD1 Register"]
pub mod WORD12 {
    #[doc = "Data byte 7 of Rx/Tx frame."]
    pub mod DATA_BYTE_7 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 6 of Rx/Tx frame."]
    pub mod DATA_BYTE_6 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 5 of Rx/Tx frame."]
    pub mod DATA_BYTE_5 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 4 of Rx/Tx frame."]
    pub mod DATA_BYTE_4 {
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
    #[doc = "Reserved"]
    pub mod CODE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
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
#[doc = "Message Buffer 3 WORD0 Register"]
pub mod WORD03 {
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_3 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_2 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_1 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_0 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 3 WORD1 Register"]
pub mod WORD13 {
    #[doc = "Data byte 7 of Rx/Tx frame."]
    pub mod DATA_BYTE_7 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 6 of Rx/Tx frame."]
    pub mod DATA_BYTE_6 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 5 of Rx/Tx frame."]
    pub mod DATA_BYTE_5 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 4 of Rx/Tx frame."]
    pub mod DATA_BYTE_4 {
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
    #[doc = "Reserved"]
    pub mod CODE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
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
#[doc = "Message Buffer 4 WORD0 Register"]
pub mod WORD04 {
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_3 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_2 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_1 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_0 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 4 WORD1 Register"]
pub mod WORD14 {
    #[doc = "Data byte 7 of Rx/Tx frame."]
    pub mod DATA_BYTE_7 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 6 of Rx/Tx frame."]
    pub mod DATA_BYTE_6 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 5 of Rx/Tx frame."]
    pub mod DATA_BYTE_5 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 4 of Rx/Tx frame."]
    pub mod DATA_BYTE_4 {
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
    #[doc = "Reserved"]
    pub mod CODE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
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
#[doc = "Message Buffer 5 WORD0 Register"]
pub mod WORD05 {
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_3 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_2 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_1 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_0 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 5 WORD1 Register"]
pub mod WORD15 {
    #[doc = "Data byte 7 of Rx/Tx frame."]
    pub mod DATA_BYTE_7 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 6 of Rx/Tx frame."]
    pub mod DATA_BYTE_6 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 5 of Rx/Tx frame."]
    pub mod DATA_BYTE_5 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 4 of Rx/Tx frame."]
    pub mod DATA_BYTE_4 {
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
    #[doc = "Reserved"]
    pub mod CODE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
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
#[doc = "Message Buffer 6 WORD0 Register"]
pub mod WORD06 {
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_3 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_2 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_1 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_0 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 6 WORD1 Register"]
pub mod WORD16 {
    #[doc = "Data byte 7 of Rx/Tx frame."]
    pub mod DATA_BYTE_7 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 6 of Rx/Tx frame."]
    pub mod DATA_BYTE_6 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 5 of Rx/Tx frame."]
    pub mod DATA_BYTE_5 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 4 of Rx/Tx frame."]
    pub mod DATA_BYTE_4 {
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
    #[doc = "Reserved"]
    pub mod CODE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
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
#[doc = "Message Buffer 7 WORD0 Register"]
pub mod WORD07 {
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_3 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_2 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_1 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_0 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 7 WORD1 Register"]
pub mod WORD17 {
    #[doc = "Data byte 7 of Rx/Tx frame."]
    pub mod DATA_BYTE_7 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 6 of Rx/Tx frame."]
    pub mod DATA_BYTE_6 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 5 of Rx/Tx frame."]
    pub mod DATA_BYTE_5 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 4 of Rx/Tx frame."]
    pub mod DATA_BYTE_4 {
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
    #[doc = "Reserved"]
    pub mod CODE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
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
#[doc = "Message Buffer 8 WORD0 Register"]
pub mod WORD08 {
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_3 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_2 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_1 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_0 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 8 WORD1 Register"]
pub mod WORD18 {
    #[doc = "Data byte 7 of Rx/Tx frame."]
    pub mod DATA_BYTE_7 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 6 of Rx/Tx frame."]
    pub mod DATA_BYTE_6 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 5 of Rx/Tx frame."]
    pub mod DATA_BYTE_5 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 4 of Rx/Tx frame."]
    pub mod DATA_BYTE_4 {
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
    #[doc = "Reserved"]
    pub mod CODE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
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
#[doc = "Message Buffer 9 WORD0 Register"]
pub mod WORD09 {
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_3 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_2 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_1 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_0 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 9 WORD1 Register"]
pub mod WORD19 {
    #[doc = "Data byte 7 of Rx/Tx frame."]
    pub mod DATA_BYTE_7 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 6 of Rx/Tx frame."]
    pub mod DATA_BYTE_6 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 5 of Rx/Tx frame."]
    pub mod DATA_BYTE_5 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 4 of Rx/Tx frame."]
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
    #[doc = "Reserved"]
    pub mod CODE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
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
#[doc = "Message Buffer 10 WORD0 Register"]
pub mod WORD010 {
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_3 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_2 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_1 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_0 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 10 WORD1 Register"]
pub mod WORD110 {
    #[doc = "Data byte 7 of Rx/Tx frame."]
    pub mod DATA_BYTE_7 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 6 of Rx/Tx frame."]
    pub mod DATA_BYTE_6 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 5 of Rx/Tx frame."]
    pub mod DATA_BYTE_5 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 4 of Rx/Tx frame."]
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
    #[doc = "Reserved"]
    pub mod CODE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
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
#[doc = "Message Buffer 11 WORD0 Register"]
pub mod WORD011 {
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_3 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_2 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_1 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_0 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 11 WORD1 Register"]
pub mod WORD111 {
    #[doc = "Data byte 7 of Rx/Tx frame."]
    pub mod DATA_BYTE_7 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 6 of Rx/Tx frame."]
    pub mod DATA_BYTE_6 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 5 of Rx/Tx frame."]
    pub mod DATA_BYTE_5 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 4 of Rx/Tx frame."]
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
    #[doc = "Reserved"]
    pub mod CODE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
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
#[doc = "Message Buffer 12 WORD0 Register"]
pub mod WORD012 {
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_3 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_2 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_1 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_0 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 12 WORD1 Register"]
pub mod WORD112 {
    #[doc = "Data byte 7 of Rx/Tx frame."]
    pub mod DATA_BYTE_7 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 6 of Rx/Tx frame."]
    pub mod DATA_BYTE_6 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 5 of Rx/Tx frame."]
    pub mod DATA_BYTE_5 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 4 of Rx/Tx frame."]
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
    #[doc = "Reserved"]
    pub mod CODE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
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
#[doc = "Message Buffer 13 WORD0 Register"]
pub mod WORD013 {
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_3 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_2 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_1 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_0 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 13 WORD1 Register"]
pub mod WORD113 {
    #[doc = "Data byte 7 of Rx/Tx frame."]
    pub mod DATA_BYTE_7 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 6 of Rx/Tx frame."]
    pub mod DATA_BYTE_6 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 5 of Rx/Tx frame."]
    pub mod DATA_BYTE_5 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 4 of Rx/Tx frame."]
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
    #[doc = "Reserved"]
    pub mod CODE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
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
#[doc = "Message Buffer 14 WORD0 Register"]
pub mod WORD014 {
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_3 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_2 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_1 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_0 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 14 WORD1 Register"]
pub mod WORD114 {
    #[doc = "Data byte 7 of Rx/Tx frame."]
    pub mod DATA_BYTE_7 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 6 of Rx/Tx frame."]
    pub mod DATA_BYTE_6 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 5 of Rx/Tx frame."]
    pub mod DATA_BYTE_5 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 4 of Rx/Tx frame."]
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
    #[doc = "Reserved"]
    pub mod CODE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
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
#[doc = "Message Buffer 15 WORD0 Register"]
pub mod WORD015 {
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_3 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_2 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_1 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_0 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 15 WORD1 Register"]
pub mod WORD115 {
    #[doc = "Data byte 7 of Rx/Tx frame."]
    pub mod DATA_BYTE_7 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 6 of Rx/Tx frame."]
    pub mod DATA_BYTE_6 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 5 of Rx/Tx frame."]
    pub mod DATA_BYTE_5 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 4 of Rx/Tx frame."]
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
    #[doc = "Reserved"]
    pub mod CODE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
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
#[doc = "Message Buffer 16 WORD0 Register"]
pub mod WORD016 {
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_3 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_2 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_1 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_0 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 16 WORD1 Register"]
pub mod WORD116 {
    #[doc = "Data byte 7 of Rx/Tx frame."]
    pub mod DATA_BYTE_7 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 6 of Rx/Tx frame."]
    pub mod DATA_BYTE_6 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 5 of Rx/Tx frame."]
    pub mod DATA_BYTE_5 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 4 of Rx/Tx frame."]
    pub mod DATA_BYTE_4 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
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
    #[doc = "Reserved"]
    pub mod CODE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
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
#[doc = "Message Buffer 17 WORD0 Register"]
pub mod WORD017 {
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_3 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_2 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_1 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_0 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 17 WORD1 Register"]
pub mod WORD117 {
    #[doc = "Data byte 7 of Rx/Tx frame."]
    pub mod DATA_BYTE_7 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 6 of Rx/Tx frame."]
    pub mod DATA_BYTE_6 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 5 of Rx/Tx frame."]
    pub mod DATA_BYTE_5 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 4 of Rx/Tx frame."]
    pub mod DATA_BYTE_4 {
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
    #[doc = "Reserved"]
    pub mod CODE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
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
#[doc = "Message Buffer 18 WORD0 Register"]
pub mod WORD018 {
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_3 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_2 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_1 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_0 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 18 WORD1 Register"]
pub mod WORD118 {
    #[doc = "Data byte 7 of Rx/Tx frame."]
    pub mod DATA_BYTE_7 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 6 of Rx/Tx frame."]
    pub mod DATA_BYTE_6 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 5 of Rx/Tx frame."]
    pub mod DATA_BYTE_5 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 4 of Rx/Tx frame."]
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
    #[doc = "Reserved"]
    pub mod CODE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
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
#[doc = "Message Buffer 19 WORD0 Register"]
pub mod WORD019 {
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_3 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_2 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_1 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_0 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 19 WORD1 Register"]
pub mod WORD119 {
    #[doc = "Data byte 7 of Rx/Tx frame."]
    pub mod DATA_BYTE_7 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 6 of Rx/Tx frame."]
    pub mod DATA_BYTE_6 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 5 of Rx/Tx frame."]
    pub mod DATA_BYTE_5 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 4 of Rx/Tx frame."]
    pub mod DATA_BYTE_4 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
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
    #[doc = "Reserved"]
    pub mod CODE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
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
#[doc = "Message Buffer 20 WORD0 Register"]
pub mod WORD020 {
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_3 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_2 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_1 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_0 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 20 WORD1 Register"]
pub mod WORD120 {
    #[doc = "Data byte 7 of Rx/Tx frame."]
    pub mod DATA_BYTE_7 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 6 of Rx/Tx frame."]
    pub mod DATA_BYTE_6 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 5 of Rx/Tx frame."]
    pub mod DATA_BYTE_5 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 4 of Rx/Tx frame."]
    pub mod DATA_BYTE_4 {
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
    #[doc = "Reserved"]
    pub mod CODE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
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
#[doc = "Message Buffer 21 WORD0 Register"]
pub mod WORD021 {
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_3 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_2 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_1 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_0 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 21 WORD1 Register"]
pub mod WORD121 {
    #[doc = "Data byte 7 of Rx/Tx frame."]
    pub mod DATA_BYTE_7 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 6 of Rx/Tx frame."]
    pub mod DATA_BYTE_6 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 5 of Rx/Tx frame."]
    pub mod DATA_BYTE_5 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 4 of Rx/Tx frame."]
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
    #[doc = "Reserved"]
    pub mod CODE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
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
#[doc = "Message Buffer 22 WORD0 Register"]
pub mod WORD022 {
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_3 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_2 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_1 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_0 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 22 WORD1 Register"]
pub mod WORD122 {
    #[doc = "Data byte 7 of Rx/Tx frame."]
    pub mod DATA_BYTE_7 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 6 of Rx/Tx frame."]
    pub mod DATA_BYTE_6 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 5 of Rx/Tx frame."]
    pub mod DATA_BYTE_5 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 4 of Rx/Tx frame."]
    pub mod DATA_BYTE_4 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
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
    #[doc = "Reserved"]
    pub mod CODE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
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
#[doc = "Message Buffer 23 WORD0 Register"]
pub mod WORD023 {
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_3 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_2 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_1 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_0 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 23 WORD1 Register"]
pub mod WORD123 {
    #[doc = "Data byte 7 of Rx/Tx frame."]
    pub mod DATA_BYTE_7 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 6 of Rx/Tx frame."]
    pub mod DATA_BYTE_6 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 5 of Rx/Tx frame."]
    pub mod DATA_BYTE_5 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 4 of Rx/Tx frame."]
    pub mod DATA_BYTE_4 {
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
    #[doc = "Reserved"]
    pub mod CODE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
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
#[doc = "Message Buffer 24 WORD0 Register"]
pub mod WORD024 {
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_3 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_2 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_1 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_0 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 24 WORD1 Register"]
pub mod WORD124 {
    #[doc = "Data byte 7 of Rx/Tx frame."]
    pub mod DATA_BYTE_7 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 6 of Rx/Tx frame."]
    pub mod DATA_BYTE_6 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 5 of Rx/Tx frame."]
    pub mod DATA_BYTE_5 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 4 of Rx/Tx frame."]
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
    #[doc = "Reserved"]
    pub mod CODE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
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
#[doc = "Message Buffer 25 WORD0 Register"]
pub mod WORD025 {
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_3 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_2 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_1 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_0 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 25 WORD1 Register"]
pub mod WORD125 {
    #[doc = "Data byte 7 of Rx/Tx frame."]
    pub mod DATA_BYTE_7 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 6 of Rx/Tx frame."]
    pub mod DATA_BYTE_6 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 5 of Rx/Tx frame."]
    pub mod DATA_BYTE_5 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 4 of Rx/Tx frame."]
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
    #[doc = "Reserved"]
    pub mod CODE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
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
#[doc = "Message Buffer 26 WORD0 Register"]
pub mod WORD026 {
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_3 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_2 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_1 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_0 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 26 WORD1 Register"]
pub mod WORD126 {
    #[doc = "Data byte 7 of Rx/Tx frame."]
    pub mod DATA_BYTE_7 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 6 of Rx/Tx frame."]
    pub mod DATA_BYTE_6 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 5 of Rx/Tx frame."]
    pub mod DATA_BYTE_5 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 4 of Rx/Tx frame."]
    pub mod DATA_BYTE_4 {
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
    #[doc = "Reserved"]
    pub mod CODE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
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
#[doc = "Message Buffer 27 WORD0 Register"]
pub mod WORD027 {
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_3 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_2 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_1 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_0 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 27 WORD1 Register"]
pub mod WORD127 {
    #[doc = "Data byte 7 of Rx/Tx frame."]
    pub mod DATA_BYTE_7 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 6 of Rx/Tx frame."]
    pub mod DATA_BYTE_6 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 5 of Rx/Tx frame."]
    pub mod DATA_BYTE_5 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 4 of Rx/Tx frame."]
    pub mod DATA_BYTE_4 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
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
    #[doc = "Reserved"]
    pub mod CODE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
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
#[doc = "Message Buffer 28 WORD0 Register"]
pub mod WORD028 {
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_3 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_2 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_1 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_0 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 28 WORD1 Register"]
pub mod WORD128 {
    #[doc = "Data byte 7 of Rx/Tx frame."]
    pub mod DATA_BYTE_7 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 6 of Rx/Tx frame."]
    pub mod DATA_BYTE_6 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 5 of Rx/Tx frame."]
    pub mod DATA_BYTE_5 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 4 of Rx/Tx frame."]
    pub mod DATA_BYTE_4 {
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
    #[doc = "Reserved"]
    pub mod CODE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
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
#[doc = "Message Buffer 29 WORD0 Register"]
pub mod WORD029 {
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_3 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_2 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_1 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_0 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 29 WORD1 Register"]
pub mod WORD129 {
    #[doc = "Data byte 7 of Rx/Tx frame."]
    pub mod DATA_BYTE_7 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 6 of Rx/Tx frame."]
    pub mod DATA_BYTE_6 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 5 of Rx/Tx frame."]
    pub mod DATA_BYTE_5 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 4 of Rx/Tx frame."]
    pub mod DATA_BYTE_4 {
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
    #[doc = "Reserved"]
    pub mod CODE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
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
#[doc = "Message Buffer 30 WORD0 Register"]
pub mod WORD030 {
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_3 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_2 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_1 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_0 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 30 WORD1 Register"]
pub mod WORD130 {
    #[doc = "Data byte 7 of Rx/Tx frame."]
    pub mod DATA_BYTE_7 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 6 of Rx/Tx frame."]
    pub mod DATA_BYTE_6 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 5 of Rx/Tx frame."]
    pub mod DATA_BYTE_5 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 4 of Rx/Tx frame."]
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
    #[doc = "Reserved"]
    pub mod CODE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
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
#[doc = "Message Buffer 31 WORD0 Register"]
pub mod WORD031 {
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_3 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_2 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_1 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_0 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 31 WORD1 Register"]
pub mod WORD131 {
    #[doc = "Data byte 7 of Rx/Tx frame."]
    pub mod DATA_BYTE_7 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 6 of Rx/Tx frame."]
    pub mod DATA_BYTE_6 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 5 of Rx/Tx frame."]
    pub mod DATA_BYTE_5 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 4 of Rx/Tx frame."]
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
    #[doc = "Reserved"]
    pub mod CODE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
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
#[doc = "Message Buffer 32 WORD0 Register"]
pub mod WORD032 {
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_3 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_2 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_1 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_0 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 32 WORD1 Register"]
pub mod WORD132 {
    #[doc = "Data byte 7 of Rx/Tx frame."]
    pub mod DATA_BYTE_7 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 6 of Rx/Tx frame."]
    pub mod DATA_BYTE_6 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 5 of Rx/Tx frame."]
    pub mod DATA_BYTE_5 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 4 of Rx/Tx frame."]
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
    #[doc = "Reserved"]
    pub mod CODE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
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
#[doc = "Message Buffer 33 WORD0 Register"]
pub mod WORD033 {
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_3 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_2 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_1 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_0 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 33 WORD1 Register"]
pub mod WORD133 {
    #[doc = "Data byte 7 of Rx/Tx frame."]
    pub mod DATA_BYTE_7 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 6 of Rx/Tx frame."]
    pub mod DATA_BYTE_6 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 5 of Rx/Tx frame."]
    pub mod DATA_BYTE_5 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 4 of Rx/Tx frame."]
    pub mod DATA_BYTE_4 {
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
    #[doc = "Reserved"]
    pub mod CODE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
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
#[doc = "Message Buffer 34 WORD0 Register"]
pub mod WORD034 {
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_3 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_2 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_1 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_0 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 34 WORD1 Register"]
pub mod WORD134 {
    #[doc = "Data byte 7 of Rx/Tx frame."]
    pub mod DATA_BYTE_7 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 6 of Rx/Tx frame."]
    pub mod DATA_BYTE_6 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 5 of Rx/Tx frame."]
    pub mod DATA_BYTE_5 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 4 of Rx/Tx frame."]
    pub mod DATA_BYTE_4 {
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
    #[doc = "Reserved"]
    pub mod CODE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
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
#[doc = "Message Buffer 35 WORD0 Register"]
pub mod WORD035 {
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_3 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_2 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_1 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_0 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 35 WORD1 Register"]
pub mod WORD135 {
    #[doc = "Data byte 7 of Rx/Tx frame."]
    pub mod DATA_BYTE_7 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 6 of Rx/Tx frame."]
    pub mod DATA_BYTE_6 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 5 of Rx/Tx frame."]
    pub mod DATA_BYTE_5 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 4 of Rx/Tx frame."]
    pub mod DATA_BYTE_4 {
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
    #[doc = "Reserved"]
    pub mod CODE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
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
#[doc = "Message Buffer 36 WORD0 Register"]
pub mod WORD036 {
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_3 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_2 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_1 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_0 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 36 WORD1 Register"]
pub mod WORD136 {
    #[doc = "Data byte 7 of Rx/Tx frame."]
    pub mod DATA_BYTE_7 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 6 of Rx/Tx frame."]
    pub mod DATA_BYTE_6 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 5 of Rx/Tx frame."]
    pub mod DATA_BYTE_5 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 4 of Rx/Tx frame."]
    pub mod DATA_BYTE_4 {
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
    #[doc = "Reserved"]
    pub mod CODE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
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
#[doc = "Message Buffer 37 WORD0 Register"]
pub mod WORD037 {
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_3 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_2 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_1 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_0 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 37 WORD1 Register"]
pub mod WORD137 {
    #[doc = "Data byte 7 of Rx/Tx frame."]
    pub mod DATA_BYTE_7 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 6 of Rx/Tx frame."]
    pub mod DATA_BYTE_6 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 5 of Rx/Tx frame."]
    pub mod DATA_BYTE_5 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 4 of Rx/Tx frame."]
    pub mod DATA_BYTE_4 {
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
    #[doc = "Reserved"]
    pub mod CODE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
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
#[doc = "Message Buffer 38 WORD0 Register"]
pub mod WORD038 {
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_3 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_2 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_1 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_0 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 38 WORD1 Register"]
pub mod WORD138 {
    #[doc = "Data byte 7 of Rx/Tx frame."]
    pub mod DATA_BYTE_7 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 6 of Rx/Tx frame."]
    pub mod DATA_BYTE_6 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 5 of Rx/Tx frame."]
    pub mod DATA_BYTE_5 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 4 of Rx/Tx frame."]
    pub mod DATA_BYTE_4 {
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
    #[doc = "Reserved"]
    pub mod CODE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
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
#[doc = "Message Buffer 39 WORD0 Register"]
pub mod WORD039 {
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_3 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_2 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_1 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_0 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 39 WORD1 Register"]
pub mod WORD139 {
    #[doc = "Data byte 7 of Rx/Tx frame."]
    pub mod DATA_BYTE_7 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 6 of Rx/Tx frame."]
    pub mod DATA_BYTE_6 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 5 of Rx/Tx frame."]
    pub mod DATA_BYTE_5 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 4 of Rx/Tx frame."]
    pub mod DATA_BYTE_4 {
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
    #[doc = "Reserved"]
    pub mod CODE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
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
#[doc = "Message Buffer 40 WORD0 Register"]
pub mod WORD040 {
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_3 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_2 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_1 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_0 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 40 WORD1 Register"]
pub mod WORD140 {
    #[doc = "Data byte 7 of Rx/Tx frame."]
    pub mod DATA_BYTE_7 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 6 of Rx/Tx frame."]
    pub mod DATA_BYTE_6 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 5 of Rx/Tx frame."]
    pub mod DATA_BYTE_5 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 4 of Rx/Tx frame."]
    pub mod DATA_BYTE_4 {
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
    #[doc = "Reserved"]
    pub mod CODE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
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
#[doc = "Message Buffer 41 WORD0 Register"]
pub mod WORD041 {
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_3 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_2 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_1 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_0 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 41 WORD1 Register"]
pub mod WORD141 {
    #[doc = "Data byte 7 of Rx/Tx frame."]
    pub mod DATA_BYTE_7 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 6 of Rx/Tx frame."]
    pub mod DATA_BYTE_6 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 5 of Rx/Tx frame."]
    pub mod DATA_BYTE_5 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 4 of Rx/Tx frame."]
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
    #[doc = "Reserved"]
    pub mod CODE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
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
#[doc = "Message Buffer 42 WORD0 Register"]
pub mod WORD042 {
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_3 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_2 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_1 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_0 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 42 WORD1 Register"]
pub mod WORD142 {
    #[doc = "Data byte 7 of Rx/Tx frame."]
    pub mod DATA_BYTE_7 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 6 of Rx/Tx frame."]
    pub mod DATA_BYTE_6 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 5 of Rx/Tx frame."]
    pub mod DATA_BYTE_5 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 4 of Rx/Tx frame."]
    pub mod DATA_BYTE_4 {
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
    #[doc = "Reserved"]
    pub mod CODE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
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
#[doc = "Message Buffer 43 WORD0 Register"]
pub mod WORD043 {
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_3 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_2 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_1 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_0 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 43 WORD1 Register"]
pub mod WORD143 {
    #[doc = "Data byte 7 of Rx/Tx frame."]
    pub mod DATA_BYTE_7 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 6 of Rx/Tx frame."]
    pub mod DATA_BYTE_6 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 5 of Rx/Tx frame."]
    pub mod DATA_BYTE_5 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 4 of Rx/Tx frame."]
    pub mod DATA_BYTE_4 {
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
    #[doc = "Reserved"]
    pub mod CODE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
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
#[doc = "Message Buffer 44 WORD0 Register"]
pub mod WORD044 {
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_3 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_2 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_1 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_0 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 44 WORD1 Register"]
pub mod WORD144 {
    #[doc = "Data byte 7 of Rx/Tx frame."]
    pub mod DATA_BYTE_7 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 6 of Rx/Tx frame."]
    pub mod DATA_BYTE_6 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 5 of Rx/Tx frame."]
    pub mod DATA_BYTE_5 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 4 of Rx/Tx frame."]
    pub mod DATA_BYTE_4 {
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
    #[doc = "Reserved"]
    pub mod CODE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
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
#[doc = "Message Buffer 45 WORD0 Register"]
pub mod WORD045 {
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_3 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_2 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_1 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_0 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 45 WORD1 Register"]
pub mod WORD145 {
    #[doc = "Data byte 7 of Rx/Tx frame."]
    pub mod DATA_BYTE_7 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 6 of Rx/Tx frame."]
    pub mod DATA_BYTE_6 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 5 of Rx/Tx frame."]
    pub mod DATA_BYTE_5 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 4 of Rx/Tx frame."]
    pub mod DATA_BYTE_4 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
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
    #[doc = "Reserved"]
    pub mod CODE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
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
#[doc = "Message Buffer 46 WORD0 Register"]
pub mod WORD046 {
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_3 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_2 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_1 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_0 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 46 WORD1 Register"]
pub mod WORD146 {
    #[doc = "Data byte 7 of Rx/Tx frame."]
    pub mod DATA_BYTE_7 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 6 of Rx/Tx frame."]
    pub mod DATA_BYTE_6 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 5 of Rx/Tx frame."]
    pub mod DATA_BYTE_5 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 4 of Rx/Tx frame."]
    pub mod DATA_BYTE_4 {
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
    #[doc = "Reserved"]
    pub mod CODE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
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
#[doc = "Message Buffer 47 WORD0 Register"]
pub mod WORD047 {
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_3 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_2 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_1 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_0 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 47 WORD1 Register"]
pub mod WORD147 {
    #[doc = "Data byte 7 of Rx/Tx frame."]
    pub mod DATA_BYTE_7 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 6 of Rx/Tx frame."]
    pub mod DATA_BYTE_6 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 5 of Rx/Tx frame."]
    pub mod DATA_BYTE_5 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 4 of Rx/Tx frame."]
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
    #[doc = "Reserved"]
    pub mod CODE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
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
#[doc = "Message Buffer 48 WORD0 Register"]
pub mod WORD048 {
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_3 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_2 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_1 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_0 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 48 WORD1 Register"]
pub mod WORD148 {
    #[doc = "Data byte 7 of Rx/Tx frame."]
    pub mod DATA_BYTE_7 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 6 of Rx/Tx frame."]
    pub mod DATA_BYTE_6 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 5 of Rx/Tx frame."]
    pub mod DATA_BYTE_5 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 4 of Rx/Tx frame."]
    pub mod DATA_BYTE_4 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
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
    #[doc = "Reserved"]
    pub mod CODE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
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
#[doc = "Message Buffer 49 WORD0 Register"]
pub mod WORD049 {
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_3 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_2 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_1 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_0 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 49 WORD1 Register"]
pub mod WORD149 {
    #[doc = "Data byte 7 of Rx/Tx frame."]
    pub mod DATA_BYTE_7 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 6 of Rx/Tx frame."]
    pub mod DATA_BYTE_6 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 5 of Rx/Tx frame."]
    pub mod DATA_BYTE_5 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 4 of Rx/Tx frame."]
    pub mod DATA_BYTE_4 {
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
    #[doc = "Reserved"]
    pub mod CODE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
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
#[doc = "Message Buffer 50 WORD0 Register"]
pub mod WORD050 {
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_3 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_2 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_1 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_0 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 50 WORD1 Register"]
pub mod WORD150 {
    #[doc = "Data byte 7 of Rx/Tx frame."]
    pub mod DATA_BYTE_7 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 6 of Rx/Tx frame."]
    pub mod DATA_BYTE_6 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 5 of Rx/Tx frame."]
    pub mod DATA_BYTE_5 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 4 of Rx/Tx frame."]
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
    #[doc = "Reserved"]
    pub mod CODE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
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
#[doc = "Message Buffer 51 WORD0 Register"]
pub mod WORD051 {
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_3 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_2 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_1 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_0 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 51 WORD1 Register"]
pub mod WORD151 {
    #[doc = "Data byte 7 of Rx/Tx frame."]
    pub mod DATA_BYTE_7 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 6 of Rx/Tx frame."]
    pub mod DATA_BYTE_6 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 5 of Rx/Tx frame."]
    pub mod DATA_BYTE_5 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 4 of Rx/Tx frame."]
    pub mod DATA_BYTE_4 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
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
    #[doc = "Reserved"]
    pub mod CODE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
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
#[doc = "Message Buffer 52 WORD0 Register"]
pub mod WORD052 {
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_3 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_2 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_1 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_0 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 52 WORD1 Register"]
pub mod WORD152 {
    #[doc = "Data byte 7 of Rx/Tx frame."]
    pub mod DATA_BYTE_7 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 6 of Rx/Tx frame."]
    pub mod DATA_BYTE_6 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 5 of Rx/Tx frame."]
    pub mod DATA_BYTE_5 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 4 of Rx/Tx frame."]
    pub mod DATA_BYTE_4 {
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
    #[doc = "Reserved"]
    pub mod CODE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
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
#[doc = "Message Buffer 53 WORD0 Register"]
pub mod WORD053 {
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_3 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_2 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_1 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_0 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 53 WORD1 Register"]
pub mod WORD153 {
    #[doc = "Data byte 7 of Rx/Tx frame."]
    pub mod DATA_BYTE_7 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 6 of Rx/Tx frame."]
    pub mod DATA_BYTE_6 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 5 of Rx/Tx frame."]
    pub mod DATA_BYTE_5 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 4 of Rx/Tx frame."]
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
    #[doc = "Reserved"]
    pub mod CODE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
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
#[doc = "Message Buffer 54 WORD0 Register"]
pub mod WORD054 {
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_3 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_2 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_1 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_0 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 54 WORD1 Register"]
pub mod WORD154 {
    #[doc = "Data byte 7 of Rx/Tx frame."]
    pub mod DATA_BYTE_7 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 6 of Rx/Tx frame."]
    pub mod DATA_BYTE_6 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 5 of Rx/Tx frame."]
    pub mod DATA_BYTE_5 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 4 of Rx/Tx frame."]
    pub mod DATA_BYTE_4 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
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
    #[doc = "Reserved"]
    pub mod CODE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
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
#[doc = "Message Buffer 55 WORD0 Register"]
pub mod WORD055 {
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_3 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_2 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_1 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_0 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 55 WORD1 Register"]
pub mod WORD155 {
    #[doc = "Data byte 7 of Rx/Tx frame."]
    pub mod DATA_BYTE_7 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 6 of Rx/Tx frame."]
    pub mod DATA_BYTE_6 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 5 of Rx/Tx frame."]
    pub mod DATA_BYTE_5 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 4 of Rx/Tx frame."]
    pub mod DATA_BYTE_4 {
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
    #[doc = "Reserved"]
    pub mod CODE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
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
#[doc = "Message Buffer 56 WORD0 Register"]
pub mod WORD056 {
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_3 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_2 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_1 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_0 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 56 WORD1 Register"]
pub mod WORD156 {
    #[doc = "Data byte 7 of Rx/Tx frame."]
    pub mod DATA_BYTE_7 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 6 of Rx/Tx frame."]
    pub mod DATA_BYTE_6 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 5 of Rx/Tx frame."]
    pub mod DATA_BYTE_5 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 4 of Rx/Tx frame."]
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
    #[doc = "Reserved"]
    pub mod CODE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
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
#[doc = "Message Buffer 57 WORD0 Register"]
pub mod WORD057 {
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_3 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_2 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_1 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_0 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 57 WORD1 Register"]
pub mod WORD157 {
    #[doc = "Data byte 7 of Rx/Tx frame."]
    pub mod DATA_BYTE_7 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 6 of Rx/Tx frame."]
    pub mod DATA_BYTE_6 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 5 of Rx/Tx frame."]
    pub mod DATA_BYTE_5 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 4 of Rx/Tx frame."]
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
    #[doc = "Reserved"]
    pub mod CODE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
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
#[doc = "Message Buffer 58 WORD0 Register"]
pub mod WORD058 {
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_3 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_2 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_1 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_0 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 58 WORD1 Register"]
pub mod WORD158 {
    #[doc = "Data byte 7 of Rx/Tx frame."]
    pub mod DATA_BYTE_7 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 6 of Rx/Tx frame."]
    pub mod DATA_BYTE_6 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 5 of Rx/Tx frame."]
    pub mod DATA_BYTE_5 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 4 of Rx/Tx frame."]
    pub mod DATA_BYTE_4 {
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
    #[doc = "Reserved"]
    pub mod CODE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
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
#[doc = "Message Buffer 59 WORD0 Register"]
pub mod WORD059 {
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_3 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_2 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_1 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_0 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 59 WORD1 Register"]
pub mod WORD159 {
    #[doc = "Data byte 7 of Rx/Tx frame."]
    pub mod DATA_BYTE_7 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 6 of Rx/Tx frame."]
    pub mod DATA_BYTE_6 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 5 of Rx/Tx frame."]
    pub mod DATA_BYTE_5 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 4 of Rx/Tx frame."]
    pub mod DATA_BYTE_4 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
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
    #[doc = "Reserved"]
    pub mod CODE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
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
#[doc = "Message Buffer 60 WORD0 Register"]
pub mod WORD060 {
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_3 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_2 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_1 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_0 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 60 WORD1 Register"]
pub mod WORD160 {
    #[doc = "Data byte 7 of Rx/Tx frame."]
    pub mod DATA_BYTE_7 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 6 of Rx/Tx frame."]
    pub mod DATA_BYTE_6 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 5 of Rx/Tx frame."]
    pub mod DATA_BYTE_5 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 4 of Rx/Tx frame."]
    pub mod DATA_BYTE_4 {
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
    #[doc = "Reserved"]
    pub mod CODE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
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
#[doc = "Message Buffer 61 WORD0 Register"]
pub mod WORD061 {
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_3 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_2 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_1 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_0 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 61 WORD1 Register"]
pub mod WORD161 {
    #[doc = "Data byte 7 of Rx/Tx frame."]
    pub mod DATA_BYTE_7 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 6 of Rx/Tx frame."]
    pub mod DATA_BYTE_6 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 5 of Rx/Tx frame."]
    pub mod DATA_BYTE_5 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 4 of Rx/Tx frame."]
    pub mod DATA_BYTE_4 {
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
    #[doc = "Reserved"]
    pub mod CODE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
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
#[doc = "Message Buffer 62 WORD0 Register"]
pub mod WORD062 {
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_3 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_2 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_1 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_0 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 62 WORD1 Register"]
pub mod WORD162 {
    #[doc = "Data byte 7 of Rx/Tx frame."]
    pub mod DATA_BYTE_7 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 6 of Rx/Tx frame."]
    pub mod DATA_BYTE_6 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 5 of Rx/Tx frame."]
    pub mod DATA_BYTE_5 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 4 of Rx/Tx frame."]
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
    #[doc = "Reserved"]
    pub mod CODE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
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
#[doc = "Message Buffer 63 WORD0 Register"]
pub mod WORD063 {
    #[doc = "Data byte 3 of Rx/Tx frame."]
    pub mod DATA_BYTE_3 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    pub mod DATA_BYTE_2 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    pub mod DATA_BYTE_1 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 0 of Rx/Tx frame."]
    pub mod DATA_BYTE_0 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Message Buffer 63 WORD1 Register"]
pub mod WORD163 {
    #[doc = "Data byte 7 of Rx/Tx frame."]
    pub mod DATA_BYTE_7 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 6 of Rx/Tx frame."]
    pub mod DATA_BYTE_6 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 5 of Rx/Tx frame."]
    pub mod DATA_BYTE_5 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data byte 4 of Rx/Tx frame."]
    pub mod DATA_BYTE_4 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
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
