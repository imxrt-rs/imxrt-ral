#[doc = "GPIO"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "GPIO data register"]
    pub DR: crate::RWRegister<u32>,
    #[doc = "GPIO direction register"]
    pub GDIR: crate::RWRegister<u32>,
    #[doc = "GPIO pad status register"]
    pub PSR: crate::RORegister<u32>,
    #[doc = "GPIO interrupt configuration register1"]
    pub ICR1: crate::RWRegister<u32>,
    #[doc = "GPIO interrupt configuration register2"]
    pub ICR2: crate::RWRegister<u32>,
    #[doc = "GPIO interrupt mask register"]
    pub IMR: crate::RWRegister<u32>,
    #[doc = "GPIO interrupt status register"]
    pub ISR: crate::RWRegister<u32>,
    #[doc = "GPIO edge select register"]
    pub EDGE_SEL: crate::RWRegister<u32>,
    _reserved0: [u8; 0x64],
    #[doc = "GPIO data register SET"]
    pub DR_SET: crate::WORegister<u32>,
    #[doc = "GPIO data register CLEAR"]
    pub DR_CLEAR: crate::WORegister<u32>,
    #[doc = "GPIO data register TOGGLE"]
    pub DR_TOGGLE: crate::WORegister<u32>,
}
#[doc = "GPIO data register"]
pub mod DR {
    #[doc = "DR"]
    pub mod DR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO direction register"]
pub mod GDIR {
    #[doc = "GDIR"]
    pub mod GDIR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO pad status register"]
pub mod PSR {
    #[doc = "PSR"]
    pub mod PSR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO interrupt configuration register1"]
pub mod ICR1 {
    #[doc = "ICR0"]
    pub mod ICR0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt n is low-level sensitive."]
            pub const LOW_LEVEL: u32 = 0;
            #[doc = "Interrupt n is high-level sensitive."]
            pub const HIGH_LEVEL: u32 = 0x01;
            #[doc = "Interrupt n is rising-edge sensitive."]
            pub const RISING_EDGE: u32 = 0x02;
            #[doc = "Interrupt n is falling-edge sensitive."]
            pub const FALLING_EDGE: u32 = 0x03;
        }
    }
    #[doc = "ICR1"]
    pub mod ICR1 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt n is low-level sensitive."]
            pub const LOW_LEVEL: u32 = 0;
            #[doc = "Interrupt n is high-level sensitive."]
            pub const HIGH_LEVEL: u32 = 0x01;
            #[doc = "Interrupt n is rising-edge sensitive."]
            pub const RISING_EDGE: u32 = 0x02;
            #[doc = "Interrupt n is falling-edge sensitive."]
            pub const FALLING_EDGE: u32 = 0x03;
        }
    }
    #[doc = "ICR2"]
    pub mod ICR2 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt n is low-level sensitive."]
            pub const LOW_LEVEL: u32 = 0;
            #[doc = "Interrupt n is high-level sensitive."]
            pub const HIGH_LEVEL: u32 = 0x01;
            #[doc = "Interrupt n is rising-edge sensitive."]
            pub const RISING_EDGE: u32 = 0x02;
            #[doc = "Interrupt n is falling-edge sensitive."]
            pub const FALLING_EDGE: u32 = 0x03;
        }
    }
    #[doc = "ICR3"]
    pub mod ICR3 {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt n is low-level sensitive."]
            pub const LOW_LEVEL: u32 = 0;
            #[doc = "Interrupt n is high-level sensitive."]
            pub const HIGH_LEVEL: u32 = 0x01;
            #[doc = "Interrupt n is rising-edge sensitive."]
            pub const RISING_EDGE: u32 = 0x02;
            #[doc = "Interrupt n is falling-edge sensitive."]
            pub const FALLING_EDGE: u32 = 0x03;
        }
    }
    #[doc = "ICR4"]
    pub mod ICR4 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt n is low-level sensitive."]
            pub const LOW_LEVEL: u32 = 0;
            #[doc = "Interrupt n is high-level sensitive."]
            pub const HIGH_LEVEL: u32 = 0x01;
            #[doc = "Interrupt n is rising-edge sensitive."]
            pub const RISING_EDGE: u32 = 0x02;
            #[doc = "Interrupt n is falling-edge sensitive."]
            pub const FALLING_EDGE: u32 = 0x03;
        }
    }
    #[doc = "ICR5"]
    pub mod ICR5 {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt n is low-level sensitive."]
            pub const LOW_LEVEL: u32 = 0;
            #[doc = "Interrupt n is high-level sensitive."]
            pub const HIGH_LEVEL: u32 = 0x01;
            #[doc = "Interrupt n is rising-edge sensitive."]
            pub const RISING_EDGE: u32 = 0x02;
            #[doc = "Interrupt n is falling-edge sensitive."]
            pub const FALLING_EDGE: u32 = 0x03;
        }
    }
    #[doc = "ICR6"]
    pub mod ICR6 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt n is low-level sensitive."]
            pub const LOW_LEVEL: u32 = 0;
            #[doc = "Interrupt n is high-level sensitive."]
            pub const HIGH_LEVEL: u32 = 0x01;
            #[doc = "Interrupt n is rising-edge sensitive."]
            pub const RISING_EDGE: u32 = 0x02;
            #[doc = "Interrupt n is falling-edge sensitive."]
            pub const FALLING_EDGE: u32 = 0x03;
        }
    }
    #[doc = "ICR7"]
    pub mod ICR7 {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt n is low-level sensitive."]
            pub const LOW_LEVEL: u32 = 0;
            #[doc = "Interrupt n is high-level sensitive."]
            pub const HIGH_LEVEL: u32 = 0x01;
            #[doc = "Interrupt n is rising-edge sensitive."]
            pub const RISING_EDGE: u32 = 0x02;
            #[doc = "Interrupt n is falling-edge sensitive."]
            pub const FALLING_EDGE: u32 = 0x03;
        }
    }
    #[doc = "ICR8"]
    pub mod ICR8 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt n is low-level sensitive."]
            pub const LOW_LEVEL: u32 = 0;
            #[doc = "Interrupt n is high-level sensitive."]
            pub const HIGH_LEVEL: u32 = 0x01;
            #[doc = "Interrupt n is rising-edge sensitive."]
            pub const RISING_EDGE: u32 = 0x02;
            #[doc = "Interrupt n is falling-edge sensitive."]
            pub const FALLING_EDGE: u32 = 0x03;
        }
    }
    #[doc = "ICR9"]
    pub mod ICR9 {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt n is low-level sensitive."]
            pub const LOW_LEVEL: u32 = 0;
            #[doc = "Interrupt n is high-level sensitive."]
            pub const HIGH_LEVEL: u32 = 0x01;
            #[doc = "Interrupt n is rising-edge sensitive."]
            pub const RISING_EDGE: u32 = 0x02;
            #[doc = "Interrupt n is falling-edge sensitive."]
            pub const FALLING_EDGE: u32 = 0x03;
        }
    }
    #[doc = "ICR10"]
    pub mod ICR10 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt n is low-level sensitive."]
            pub const LOW_LEVEL: u32 = 0;
            #[doc = "Interrupt n is high-level sensitive."]
            pub const HIGH_LEVEL: u32 = 0x01;
            #[doc = "Interrupt n is rising-edge sensitive."]
            pub const RISING_EDGE: u32 = 0x02;
            #[doc = "Interrupt n is falling-edge sensitive."]
            pub const FALLING_EDGE: u32 = 0x03;
        }
    }
    #[doc = "ICR11"]
    pub mod ICR11 {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt n is low-level sensitive."]
            pub const LOW_LEVEL: u32 = 0;
            #[doc = "Interrupt n is high-level sensitive."]
            pub const HIGH_LEVEL: u32 = 0x01;
            #[doc = "Interrupt n is rising-edge sensitive."]
            pub const RISING_EDGE: u32 = 0x02;
            #[doc = "Interrupt n is falling-edge sensitive."]
            pub const FALLING_EDGE: u32 = 0x03;
        }
    }
    #[doc = "ICR12"]
    pub mod ICR12 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt n is low-level sensitive."]
            pub const LOW_LEVEL: u32 = 0;
            #[doc = "Interrupt n is high-level sensitive."]
            pub const HIGH_LEVEL: u32 = 0x01;
            #[doc = "Interrupt n is rising-edge sensitive."]
            pub const RISING_EDGE: u32 = 0x02;
            #[doc = "Interrupt n is falling-edge sensitive."]
            pub const FALLING_EDGE: u32 = 0x03;
        }
    }
    #[doc = "ICR13"]
    pub mod ICR13 {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt n is low-level sensitive."]
            pub const LOW_LEVEL: u32 = 0;
            #[doc = "Interrupt n is high-level sensitive."]
            pub const HIGH_LEVEL: u32 = 0x01;
            #[doc = "Interrupt n is rising-edge sensitive."]
            pub const RISING_EDGE: u32 = 0x02;
            #[doc = "Interrupt n is falling-edge sensitive."]
            pub const FALLING_EDGE: u32 = 0x03;
        }
    }
    #[doc = "ICR14"]
    pub mod ICR14 {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt n is low-level sensitive."]
            pub const LOW_LEVEL: u32 = 0;
            #[doc = "Interrupt n is high-level sensitive."]
            pub const HIGH_LEVEL: u32 = 0x01;
            #[doc = "Interrupt n is rising-edge sensitive."]
            pub const RISING_EDGE: u32 = 0x02;
            #[doc = "Interrupt n is falling-edge sensitive."]
            pub const FALLING_EDGE: u32 = 0x03;
        }
    }
    #[doc = "ICR15"]
    pub mod ICR15 {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt n is low-level sensitive."]
            pub const LOW_LEVEL: u32 = 0;
            #[doc = "Interrupt n is high-level sensitive."]
            pub const HIGH_LEVEL: u32 = 0x01;
            #[doc = "Interrupt n is rising-edge sensitive."]
            pub const RISING_EDGE: u32 = 0x02;
            #[doc = "Interrupt n is falling-edge sensitive."]
            pub const FALLING_EDGE: u32 = 0x03;
        }
    }
}
#[doc = "GPIO interrupt configuration register2"]
pub mod ICR2 {
    #[doc = "ICR16"]
    pub mod ICR16 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt n is low-level sensitive."]
            pub const LOW_LEVEL: u32 = 0;
            #[doc = "Interrupt n is high-level sensitive."]
            pub const HIGH_LEVEL: u32 = 0x01;
            #[doc = "Interrupt n is rising-edge sensitive."]
            pub const RISING_EDGE: u32 = 0x02;
            #[doc = "Interrupt n is falling-edge sensitive."]
            pub const FALLING_EDGE: u32 = 0x03;
        }
    }
    #[doc = "ICR17"]
    pub mod ICR17 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt n is low-level sensitive."]
            pub const LOW_LEVEL: u32 = 0;
            #[doc = "Interrupt n is high-level sensitive."]
            pub const HIGH_LEVEL: u32 = 0x01;
            #[doc = "Interrupt n is rising-edge sensitive."]
            pub const RISING_EDGE: u32 = 0x02;
            #[doc = "Interrupt n is falling-edge sensitive."]
            pub const FALLING_EDGE: u32 = 0x03;
        }
    }
    #[doc = "ICR18"]
    pub mod ICR18 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt n is low-level sensitive."]
            pub const LOW_LEVEL: u32 = 0;
            #[doc = "Interrupt n is high-level sensitive."]
            pub const HIGH_LEVEL: u32 = 0x01;
            #[doc = "Interrupt n is rising-edge sensitive."]
            pub const RISING_EDGE: u32 = 0x02;
            #[doc = "Interrupt n is falling-edge sensitive."]
            pub const FALLING_EDGE: u32 = 0x03;
        }
    }
    #[doc = "ICR19"]
    pub mod ICR19 {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt n is low-level sensitive."]
            pub const LOW_LEVEL: u32 = 0;
            #[doc = "Interrupt n is high-level sensitive."]
            pub const HIGH_LEVEL: u32 = 0x01;
            #[doc = "Interrupt n is rising-edge sensitive."]
            pub const RISING_EDGE: u32 = 0x02;
            #[doc = "Interrupt n is falling-edge sensitive."]
            pub const FALLING_EDGE: u32 = 0x03;
        }
    }
    #[doc = "ICR20"]
    pub mod ICR20 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt n is low-level sensitive."]
            pub const LOW_LEVEL: u32 = 0;
            #[doc = "Interrupt n is high-level sensitive."]
            pub const HIGH_LEVEL: u32 = 0x01;
            #[doc = "Interrupt n is rising-edge sensitive."]
            pub const RISING_EDGE: u32 = 0x02;
            #[doc = "Interrupt n is falling-edge sensitive."]
            pub const FALLING_EDGE: u32 = 0x03;
        }
    }
    #[doc = "ICR21"]
    pub mod ICR21 {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt n is low-level sensitive."]
            pub const LOW_LEVEL: u32 = 0;
            #[doc = "Interrupt n is high-level sensitive."]
            pub const HIGH_LEVEL: u32 = 0x01;
            #[doc = "Interrupt n is rising-edge sensitive."]
            pub const RISING_EDGE: u32 = 0x02;
            #[doc = "Interrupt n is falling-edge sensitive."]
            pub const FALLING_EDGE: u32 = 0x03;
        }
    }
    #[doc = "ICR22"]
    pub mod ICR22 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt n is low-level sensitive."]
            pub const LOW_LEVEL: u32 = 0;
            #[doc = "Interrupt n is high-level sensitive."]
            pub const HIGH_LEVEL: u32 = 0x01;
            #[doc = "Interrupt n is rising-edge sensitive."]
            pub const RISING_EDGE: u32 = 0x02;
            #[doc = "Interrupt n is falling-edge sensitive."]
            pub const FALLING_EDGE: u32 = 0x03;
        }
    }
    #[doc = "ICR23"]
    pub mod ICR23 {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt n is low-level sensitive."]
            pub const LOW_LEVEL: u32 = 0;
            #[doc = "Interrupt n is high-level sensitive."]
            pub const HIGH_LEVEL: u32 = 0x01;
            #[doc = "Interrupt n is rising-edge sensitive."]
            pub const RISING_EDGE: u32 = 0x02;
            #[doc = "Interrupt n is falling-edge sensitive."]
            pub const FALLING_EDGE: u32 = 0x03;
        }
    }
    #[doc = "ICR24"]
    pub mod ICR24 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt n is low-level sensitive."]
            pub const LOW_LEVEL: u32 = 0;
            #[doc = "Interrupt n is high-level sensitive."]
            pub const HIGH_LEVEL: u32 = 0x01;
            #[doc = "Interrupt n is rising-edge sensitive."]
            pub const RISING_EDGE: u32 = 0x02;
            #[doc = "Interrupt n is falling-edge sensitive."]
            pub const FALLING_EDGE: u32 = 0x03;
        }
    }
    #[doc = "ICR25"]
    pub mod ICR25 {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt n is low-level sensitive."]
            pub const LOW_LEVEL: u32 = 0;
            #[doc = "Interrupt n is high-level sensitive."]
            pub const HIGH_LEVEL: u32 = 0x01;
            #[doc = "Interrupt n is rising-edge sensitive."]
            pub const RISING_EDGE: u32 = 0x02;
            #[doc = "Interrupt n is falling-edge sensitive."]
            pub const FALLING_EDGE: u32 = 0x03;
        }
    }
    #[doc = "ICR26"]
    pub mod ICR26 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt n is low-level sensitive."]
            pub const LOW_LEVEL: u32 = 0;
            #[doc = "Interrupt n is high-level sensitive."]
            pub const HIGH_LEVEL: u32 = 0x01;
            #[doc = "Interrupt n is rising-edge sensitive."]
            pub const RISING_EDGE: u32 = 0x02;
            #[doc = "Interrupt n is falling-edge sensitive."]
            pub const FALLING_EDGE: u32 = 0x03;
        }
    }
    #[doc = "ICR27"]
    pub mod ICR27 {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt n is low-level sensitive."]
            pub const LOW_LEVEL: u32 = 0;
            #[doc = "Interrupt n is high-level sensitive."]
            pub const HIGH_LEVEL: u32 = 0x01;
            #[doc = "Interrupt n is rising-edge sensitive."]
            pub const RISING_EDGE: u32 = 0x02;
            #[doc = "Interrupt n is falling-edge sensitive."]
            pub const FALLING_EDGE: u32 = 0x03;
        }
    }
    #[doc = "ICR28"]
    pub mod ICR28 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt n is low-level sensitive."]
            pub const LOW_LEVEL: u32 = 0;
            #[doc = "Interrupt n is high-level sensitive."]
            pub const HIGH_LEVEL: u32 = 0x01;
            #[doc = "Interrupt n is rising-edge sensitive."]
            pub const RISING_EDGE: u32 = 0x02;
            #[doc = "Interrupt n is falling-edge sensitive."]
            pub const FALLING_EDGE: u32 = 0x03;
        }
    }
    #[doc = "ICR29"]
    pub mod ICR29 {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt n is low-level sensitive."]
            pub const LOW_LEVEL: u32 = 0;
            #[doc = "Interrupt n is high-level sensitive."]
            pub const HIGH_LEVEL: u32 = 0x01;
            #[doc = "Interrupt n is rising-edge sensitive."]
            pub const RISING_EDGE: u32 = 0x02;
            #[doc = "Interrupt n is falling-edge sensitive."]
            pub const FALLING_EDGE: u32 = 0x03;
        }
    }
    #[doc = "ICR30"]
    pub mod ICR30 {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt n is low-level sensitive."]
            pub const LOW_LEVEL: u32 = 0;
            #[doc = "Interrupt n is high-level sensitive."]
            pub const HIGH_LEVEL: u32 = 0x01;
            #[doc = "Interrupt n is rising-edge sensitive."]
            pub const RISING_EDGE: u32 = 0x02;
            #[doc = "Interrupt n is falling-edge sensitive."]
            pub const FALLING_EDGE: u32 = 0x03;
        }
    }
    #[doc = "ICR31"]
    pub mod ICR31 {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt n is low-level sensitive."]
            pub const LOW_LEVEL: u32 = 0;
            #[doc = "Interrupt n is high-level sensitive."]
            pub const HIGH_LEVEL: u32 = 0x01;
            #[doc = "Interrupt n is rising-edge sensitive."]
            pub const RISING_EDGE: u32 = 0x02;
            #[doc = "Interrupt n is falling-edge sensitive."]
            pub const FALLING_EDGE: u32 = 0x03;
        }
    }
}
#[doc = "GPIO interrupt mask register"]
pub mod IMR {
    #[doc = "IMR"]
    pub mod IMR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO interrupt status register"]
pub mod ISR {
    #[doc = "ISR"]
    pub mod ISR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO edge select register"]
pub mod EDGE_SEL {
    #[doc = "GPIO_EDGE_SEL"]
    pub mod GPIO_EDGE_SEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO data register SET"]
pub mod DR_SET {
    #[doc = "DR_SET"]
    pub mod DR_SET {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO data register CLEAR"]
pub mod DR_CLEAR {
    #[doc = "DR_CLEAR"]
    pub mod DR_CLEAR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO data register TOGGLE"]
pub mod DR_TOGGLE {
    #[doc = "DR_TOGGLE"]
    pub mod DR_TOGGLE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
