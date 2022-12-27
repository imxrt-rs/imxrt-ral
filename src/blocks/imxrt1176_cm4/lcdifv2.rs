#[doc = "LCDIF_V2"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "LCDIFv2 display control Register"]
    pub CTRL: crate::RWRegister<u32>,
    #[doc = "LCDIFv2 display control Register"]
    pub CTRL_SET: crate::RWRegister<u32>,
    #[doc = "LCDIFv2 display control Register"]
    pub CTRL_CLR: crate::RWRegister<u32>,
    #[doc = "LCDIFv2 display control Register"]
    pub CTRL_TOG: crate::RWRegister<u32>,
    #[doc = "Display Parameter Register"]
    pub DISP_PARA: crate::RWRegister<u32>,
    #[doc = "Display Size Register"]
    pub DISP_SIZE: crate::RWRegister<u32>,
    #[doc = "Horizontal Sync Parameter Register"]
    pub HSYN_PARA: crate::RWRegister<u32>,
    #[doc = "Vertical Sync Parameter Register"]
    pub VSYN_PARA: crate::RWRegister<u32>,
    #[doc = "Interrupt Status Register for domain 0"]
    pub INT_STATUS_D0: crate::RWRegister<u32>,
    #[doc = "Interrupt Enable Register for domain 0"]
    pub INT_ENABLE_D0: crate::RWRegister<u32>,
    _reserved0: [u8; 0x08],
    #[doc = "Interrupt Status Register for domain 1"]
    pub INT_STATUS_D1: crate::RWRegister<u32>,
    #[doc = "Interrupt Enable Register for domain 1"]
    pub INT_ENABLE_D1: crate::RWRegister<u32>,
    _reserved1: [u8; 0x08],
    #[doc = "Parallel Data Interface Parameter Register"]
    pub PDI_PARA: crate::RWRegister<u32>,
    _reserved2: [u8; 0x01bc],
    #[doc = "Control Descriptor Layer 1 Register"]
    pub CTRLDESCL0_1: crate::RWRegister<u32>,
    #[doc = "Control Descriptor Layer 2 Register"]
    pub CTRLDESCL0_2: crate::RWRegister<u32>,
    #[doc = "Control Descriptor Layer 3 Register"]
    pub CTRLDESCL0_3: crate::RWRegister<u32>,
    #[doc = "Control Descriptor Layer 4 Register"]
    pub CTRLDESCL0_4: crate::RWRegister<u32>,
    #[doc = "Control Descriptor Layer 5 Register"]
    pub CTRLDESCL0_5: crate::RWRegister<u32>,
    #[doc = "Control Descriptor Layer 6 Register"]
    pub CTRLDESCL0_6: crate::RWRegister<u32>,
    #[doc = "Color Space Conversion Coefficient Register 0"]
    pub CSC0_COEF0: crate::RWRegister<u32>,
    #[doc = "Color Space Conversion Coefficient Register 1"]
    pub CSC0_COEF1: crate::RWRegister<u32>,
    #[doc = "Color Space Conversion Coefficient Register 2"]
    pub CSC0_COEF2: crate::RWRegister<u32>,
    _reserved3: [u8; 0x1c],
    #[doc = "Control Descriptor Layer 1 Register"]
    pub CTRLDESCL1_1: crate::RWRegister<u32>,
    #[doc = "Control Descriptor Layer 2 Register"]
    pub CTRLDESCL1_2: crate::RWRegister<u32>,
    #[doc = "Control Descriptor Layer 3 Register"]
    pub CTRLDESCL1_3: crate::RWRegister<u32>,
    #[doc = "Control Descriptor Layer 4 Register"]
    pub CTRLDESCL1_4: crate::RWRegister<u32>,
    #[doc = "Control Descriptor Layer 5 Register"]
    pub CTRLDESCL1_5: crate::RWRegister<u32>,
    #[doc = "Control Descriptor Layer 6 Register"]
    pub CTRLDESCL1_6: crate::RWRegister<u32>,
    #[doc = "Color Space Conversion Coefficient Register 0"]
    pub CSC1_COEF0: crate::RWRegister<u32>,
    #[doc = "Color Space Conversion Coefficient Register 1"]
    pub CSC1_COEF1: crate::RWRegister<u32>,
    #[doc = "Color Space Conversion Coefficient Register 2"]
    pub CSC1_COEF2: crate::RWRegister<u32>,
    _reserved4: [u8; 0x1c],
    #[doc = "Control Descriptor Layer 1 Register"]
    pub CTRLDESCL2_1: crate::RWRegister<u32>,
    #[doc = "Control Descriptor Layer 2 Register"]
    pub CTRLDESCL2_2: crate::RWRegister<u32>,
    #[doc = "Control Descriptor Layer 3 Register"]
    pub CTRLDESCL2_3: crate::RWRegister<u32>,
    #[doc = "Control Descriptor Layer 4 Register"]
    pub CTRLDESCL2_4: crate::RWRegister<u32>,
    #[doc = "Control Descriptor Layer 5 Register"]
    pub CTRLDESCL2_5: crate::RWRegister<u32>,
    #[doc = "Control Descriptor Layer 6 Register"]
    pub CTRLDESCL2_6: crate::RWRegister<u32>,
    _reserved5: [u8; 0x28],
    #[doc = "Control Descriptor Layer 1 Register"]
    pub CTRLDESCL3_1: crate::RWRegister<u32>,
    #[doc = "Control Descriptor Layer 2 Register"]
    pub CTRLDESCL3_2: crate::RWRegister<u32>,
    #[doc = "Control Descriptor Layer 3 Register"]
    pub CTRLDESCL3_3: crate::RWRegister<u32>,
    #[doc = "Control Descriptor Layer 4 Register"]
    pub CTRLDESCL3_4: crate::RWRegister<u32>,
    #[doc = "Control Descriptor Layer 5 Register"]
    pub CTRLDESCL3_5: crate::RWRegister<u32>,
    #[doc = "Control Descriptor Layer 6 Register"]
    pub CTRLDESCL3_6: crate::RWRegister<u32>,
    _reserved6: [u8; 0x28],
    #[doc = "Control Descriptor Layer 1 Register"]
    pub CTRLDESCL4_1: crate::RWRegister<u32>,
    #[doc = "Control Descriptor Layer 2 Register"]
    pub CTRLDESCL4_2: crate::RWRegister<u32>,
    #[doc = "Control Descriptor Layer 3 Register"]
    pub CTRLDESCL4_3: crate::RWRegister<u32>,
    #[doc = "Control Descriptor Layer 4 Register"]
    pub CTRLDESCL4_4: crate::RWRegister<u32>,
    #[doc = "Control Descriptor Layer 5 Register"]
    pub CTRLDESCL4_5: crate::RWRegister<u32>,
    #[doc = "Control Descriptor Layer 6 Register"]
    pub CTRLDESCL4_6: crate::RWRegister<u32>,
    _reserved7: [u8; 0x28],
    #[doc = "Control Descriptor Layer 1 Register"]
    pub CTRLDESCL5_1: crate::RWRegister<u32>,
    #[doc = "Control Descriptor Layer 2 Register"]
    pub CTRLDESCL5_2: crate::RWRegister<u32>,
    #[doc = "Control Descriptor Layer 3 Register"]
    pub CTRLDESCL5_3: crate::RWRegister<u32>,
    #[doc = "Control Descriptor Layer 4 Register"]
    pub CTRLDESCL5_4: crate::RWRegister<u32>,
    #[doc = "Control Descriptor Layer 5 Register"]
    pub CTRLDESCL5_5: crate::RWRegister<u32>,
    #[doc = "Control Descriptor Layer 6 Register"]
    pub CTRLDESCL5_6: crate::RWRegister<u32>,
    _reserved8: [u8; 0x28],
    #[doc = "Control Descriptor Layer 1 Register"]
    pub CTRLDESCL6_1: crate::RWRegister<u32>,
    #[doc = "Control Descriptor Layer 2 Register"]
    pub CTRLDESCL6_2: crate::RWRegister<u32>,
    #[doc = "Control Descriptor Layer 3 Register"]
    pub CTRLDESCL6_3: crate::RWRegister<u32>,
    #[doc = "Control Descriptor Layer 4 Register"]
    pub CTRLDESCL6_4: crate::RWRegister<u32>,
    #[doc = "Control Descriptor Layer 5 Register"]
    pub CTRLDESCL6_5: crate::RWRegister<u32>,
    #[doc = "Control Descriptor Layer 6 Register"]
    pub CTRLDESCL6_6: crate::RWRegister<u32>,
    _reserved9: [u8; 0x28],
    #[doc = "Control Descriptor Layer 1 Register"]
    pub CTRLDESCL7_1: crate::RWRegister<u32>,
    #[doc = "Control Descriptor Layer 2 Register"]
    pub CTRLDESCL7_2: crate::RWRegister<u32>,
    #[doc = "Control Descriptor Layer 3 Register"]
    pub CTRLDESCL7_3: crate::RWRegister<u32>,
    #[doc = "Control Descriptor Layer 4 Register"]
    pub CTRLDESCL7_4: crate::RWRegister<u32>,
    #[doc = "Control Descriptor Layer 5 Register"]
    pub CTRLDESCL7_5: crate::RWRegister<u32>,
    #[doc = "Control Descriptor Layer 6 Register"]
    pub CTRLDESCL7_6: crate::RWRegister<u32>,
    _reserved10: [u8; 0x28],
    #[doc = "LCDIFv2 CLUT load Register"]
    pub CLUT_LOAD: crate::RWRegister<u32>,
}
#[doc = "LCDIFv2 display control Register"]
pub mod CTRL {
    #[doc = "Invert Horizontal synchronization signal"]
    pub mod INV_HS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "HSYNC signal not inverted (active HIGH)"]
            pub const INV_HS_0: u32 = 0;
            #[doc = "Invert HSYNC signal (active LOW)"]
            pub const INV_HS_1: u32 = 0x01;
        }
    }
    #[doc = "Invert Vertical synchronization signal"]
    pub mod INV_VS {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "VSYNC signal not inverted (active HIGH)"]
            pub const INV_VS_0: u32 = 0;
            #[doc = "Invert VSYNC signal (active LOW)"]
            pub const INV_VS_1: u32 = 0x01;
        }
    }
    #[doc = "Invert Data Enable polarity"]
    pub mod INV_DE {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Data enable is active high"]
            pub const INV_DE_0: u32 = 0;
            #[doc = "Data enable is active low"]
            pub const INV_DE_1: u32 = 0x01;
        }
    }
    #[doc = "Polarity change of Pixel Clock"]
    pub mod INV_PXCK {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Display samples data on the falling edge"]
            pub const INV_PXCK_0: u32 = 0;
            #[doc = "Display samples data on the rising edge"]
            pub const INV_PXCK_1: u32 = 0x01;
        }
    }
    #[doc = "Indicates if value at the output (pixel data output) needs to be negated"]
    pub mod NEG {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Output is to remain same"]
            pub const NEG_0: u32 = 0;
            #[doc = "Output to be negated"]
            pub const NEG_1: u32 = 0x01;
        }
    }
    #[doc = "Software Reset"]
    pub mod SW_RESET {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No action"]
            pub const SW_RESET_0: u32 = 0;
            #[doc = "All LCDIFv2 internal registers are forced into their reset state. User registers are not affected"]
            pub const SW_RESET_1: u32 = 0x01;
        }
    }
}
#[doc = "LCDIFv2 display control Register"]
pub mod CTRL_SET {
    #[doc = "Invert Horizontal synchronization signal"]
    pub mod INV_HS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Invert Vertical synchronization signal"]
    pub mod INV_VS {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Invert Data Enable polarity"]
    pub mod INV_DE {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Polarity change of Pixel Clock"]
    pub mod INV_PXCK {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Indicates if value at the output (pixel data output) needs to be negated"]
    pub mod NEG {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Software Reset"]
    pub mod SW_RESET {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "LCDIFv2 display control Register"]
pub mod CTRL_CLR {
    #[doc = "Invert Horizontal synchronization signal"]
    pub mod INV_HS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Invert Vertical synchronization signal"]
    pub mod INV_VS {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Invert Data Enable polarity"]
    pub mod INV_DE {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Polarity change of Pixel Clock"]
    pub mod INV_PXCK {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Indicates if value at the output (pixel data output) needs to be negated"]
    pub mod NEG {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Software Reset"]
    pub mod SW_RESET {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "LCDIFv2 display control Register"]
pub mod CTRL_TOG {
    #[doc = "Invert Horizontal synchronization signal"]
    pub mod INV_HS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Invert Vertical synchronization signal"]
    pub mod INV_VS {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Invert Data Enable polarity"]
    pub mod INV_DE {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Polarity change of Pixel Clock"]
    pub mod INV_PXCK {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Indicates if value at the output (pixel data output) needs to be negated"]
    pub mod NEG {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Software Reset"]
    pub mod SW_RESET {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Display Parameter Register"]
pub mod DISP_PARA {
    #[doc = "Blue component of the default color displayed in the sectors where no layer is active"]
    pub mod BGND_B {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Green component of the default color displayed in the sectors where no layer is active"]
    pub mod BGND_G {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Red component of the default color displayed in the sectors where no layer is active"]
    pub mod BGND_R {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "LCDIFv2 operating mode"]
    pub mod DISP_MODE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Normal mode. Panel content controlled by layer configuration"]
            pub const DISP_MODE_0: u32 = 0;
            #[doc = "Test Mode1(BGND Color Display)"]
            pub const DISP_MODE_1: u32 = 0x01;
            #[doc = "Test Mode2(Column Color Bar)"]
            pub const DISP_MODE_2: u32 = 0x02;
            #[doc = "Test Mode3(Row Color Bar)"]
            pub const DISP_MODE_3: u32 = 0x03;
        }
    }
    #[doc = "LCDIFv2 line output order"]
    pub mod LINE_PATTERN {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "RGB"]
            pub const LINE_PATTERN_0: u32 = 0;
            #[doc = "RBG"]
            pub const LINE_PATTERN_1: u32 = 0x01;
            #[doc = "GBR"]
            pub const LINE_PATTERN_2: u32 = 0x02;
            #[doc = "GRB"]
            pub const LINE_PATTERN_3: u32 = 0x03;
            #[doc = "BRG"]
            pub const LINE_PATTERN_4: u32 = 0x04;
            #[doc = "BGR"]
            pub const LINE_PATTERN_5: u32 = 0x05;
        }
    }
    #[doc = "Display panel On/Off mode"]
    pub mod DISP_ON {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Display Off"]
            pub const DISP_ON_0: u32 = 0;
            #[doc = "Display On"]
            pub const DISP_ON_1: u32 = 0x01;
        }
    }
}
#[doc = "Display Size Register"]
pub mod DISP_SIZE {
    #[doc = "Sets the display size horizontal resolution in pixels"]
    pub mod DELTA_X {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Sets the display size vertical resolution in pixels"]
    pub mod DELTA_Y {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Horizontal Sync Parameter Register"]
pub mod HSYN_PARA {
    #[doc = "HSYNC front-porch pulse width (in pixel clock cycles). Pulse width has a minimum value of 1"]
    pub mod FP_H {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "HSYNC active pulse width (in pixel clock cycles). Pulse width has a minimum value of 1"]
    pub mod PW_H {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "HSYNC back-porch pulse width (in pixel clock cycles). Pulse width has a minimum value of 1"]
    pub mod BP_H {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Vertical Sync Parameter Register"]
pub mod VSYN_PARA {
    #[doc = "VSYNC front-porch pulse width (in horizontal line cycles). Pulse width has a minimum value of 1"]
    pub mod FP_V {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "VSYNC active pulse width (in horizontal line cycles). Pulse width has a minimum value of 1"]
    pub mod PW_V {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "VSYNC back-porch pulse width (in horizontal line cycles). Pulse width has a minimum value of 1"]
    pub mod BP_V {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Interrupt Status Register for domain 0"]
pub mod INT_STATUS_D0 {
    #[doc = "Interrupt flag to indicate that the vertical synchronization phase(The beginning of a frame)"]
    pub mod VSYNC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "VSYNC has not started"]
            pub const VSYNC_0: u32 = 0;
            #[doc = "VSYNC has started"]
            pub const VSYNC_1: u32 = 0x01;
        }
    }
    #[doc = "Interrupt flag to indicate the output buffer underrun condition"]
    pub mod UNDERRUN {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Output buffer not underrun"]
            pub const UNDERRUN_0: u32 = 0;
            #[doc = "Output buffer underrun"]
            pub const UNDERRUN_1: u32 = 0x01;
        }
    }
    #[doc = "Interrupt flag to indicate vertical blanking period"]
    pub mod VS_BLANK {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Vertical blanking period has not started"]
            pub const VS_BLANK_0: u32 = 0;
            #[doc = "Vertical blanking period has started"]
            pub const VS_BLANK_1: u32 = 0x01;
        }
    }
    #[doc = "Interrupt flag to indicate that which PLANE has Read Error on the AXI interface"]
    pub mod DMA_ERR {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Interrupt flag to indicate that which PLANE has fetched the last pixel from memory"]
    pub mod DMA_DONE {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Interrupt flag to indicate that which FIFO in the pixel blending underflowed"]
    pub mod FIFO_EMPTY {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Interrupt Enable Register for domain 0"]
pub mod INT_ENABLE_D0 {
    #[doc = "Enable Interrupt flag to indicate that the vertical synchronization phase(The beginning of a frame)"]
    pub mod VSYNC_EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "VSYNC interrupt disable"]
            pub const VSYNC_EN_0: u32 = 0;
            #[doc = "VSYNC interrupt enable"]
            pub const VSYNC_EN_1: u32 = 0x01;
        }
    }
    #[doc = "Enable Interrupt flag to indicate the output buffer underrun condition"]
    pub mod UNDERRUN_EN {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Output buffer underrun disable"]
            pub const UNDERRUN_EN_0: u32 = 0;
            #[doc = "Output buffer underrun enable"]
            pub const UNDERRUN_EN_1: u32 = 0x01;
        }
    }
    #[doc = "Enable Interrupt flag to indicate vertical blanking period"]
    pub mod VS_BLANK_EN {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Vertical blanking start interrupt disable"]
            pub const VS_BLANK_EN_0: u32 = 0;
            #[doc = "Vertical blanking start interrupt enable"]
            pub const VS_BLANK_EN_1: u32 = 0x01;
        }
    }
    #[doc = "Enable Interrupt flag to indicate that which PLANE has Read Error on the AXI interface"]
    pub mod DMA_ERR_EN {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enable Interrupt flag to indicate that which PLANE has fetched the last pixel from memory"]
    pub mod DMA_DONE_EN {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enable Interrupt flag to indicate that which FIFO in the pixel blending underflowed"]
    pub mod FIFO_EMPTY_EN {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Interrupt Status Register for domain 1"]
pub mod INT_STATUS_D1 {
    #[doc = "Interrupt flag to indicate that the vertical synchronization phase(The beginning of a frame)"]
    pub mod VSYNC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Interrupt flag to indicate the output buffer underrun condition"]
    pub mod UNDERRUN {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Interrupt flag to indicate vertical blanking period"]
    pub mod VS_BLANK {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Interrupt flag to indicate that which PLANE has Read Error on the AXI interface"]
    pub mod DMA_ERR {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Interrupt flag to indicate that which PLANE has fetched the last pixel from memory"]
    pub mod DMA_DONE {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Interrupt flag to indicate that which FIFO in the pixel blending underflowed"]
    pub mod FIFO_EMPTY {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Interrupt Enable Register for domain 1"]
pub mod INT_ENABLE_D1 {
    #[doc = "Enable Interrupt flag to indicate that the vertical synchronization phase(The beginning of a frame)"]
    pub mod VSYNC_EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enable Interrupt flag to indicate the output buffer underrun condition"]
    pub mod UNDERRUN_EN {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enable Interrupt flag to indicate vertical blanking period"]
    pub mod VS_BLANK_EN {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enable Interrupt flag to indicate that which PLANE has Read Error on the AXI interface"]
    pub mod DMA_ERR_EN {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enable Interrupt flag to indicate that which PLANE has fetched the last pixel from memory"]
    pub mod DMA_DONE_EN {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enable Interrupt flag to indicate that which FIFO in the pixel blending underflowed"]
    pub mod FIFO_EMPTY_EN {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Parallel Data Interface Parameter Register"]
pub mod PDI_PARA {
    #[doc = "Polarity of PDI input HSYNC"]
    pub mod INV_PDI_HS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "HSYNC is active HIGH"]
            pub const INV_PDI_HS_0: u32 = 0;
            #[doc = "HSYNC is active LOW"]
            pub const INV_PDI_HS_1: u32 = 0x01;
        }
    }
    #[doc = "Polarity of PDI input VSYNC"]
    pub mod INV_PDI_VS {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "VSYNC is active HIGH"]
            pub const INV_PDI_VS_0: u32 = 0;
            #[doc = "VSYNC is active LOW"]
            pub const INV_PDI_VS_1: u32 = 0x01;
        }
    }
    #[doc = "Polarity of PDI input Data Enable"]
    pub mod INV_PDI_DE {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Data enable is active HIGH"]
            pub const INV_PDI_DE_0: u32 = 0;
            #[doc = "Data enable is active LOW"]
            pub const INV_PDI_DE_1: u32 = 0x01;
        }
    }
    #[doc = "Polarity of PDI input Pixel Clock"]
    pub mod INV_PDI_PXCK {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Samples data on the falling edge"]
            pub const INV_PDI_PXCK_0: u32 = 0;
            #[doc = "Samples data on the rising edge"]
            pub const INV_PDI_PXCK_1: u32 = 0x01;
        }
    }
    #[doc = "The PDI mode for input data format"]
    pub mod MODE {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "32 bpp (ARGB8888)"]
            pub const MODE_0: u32 = 0;
            #[doc = "24 bpp (RGB888)"]
            pub const MODE_1: u32 = 0x01;
            #[doc = "24 bpp (RGB666)"]
            pub const MODE_2: u32 = 0x02;
            #[doc = "16 bpp (RGB565)"]
            pub const MODE_3: u32 = 0x03;
            #[doc = "16 bpp (RGB444)"]
            pub const MODE_4: u32 = 0x04;
            #[doc = "16 bpp (RGB555)"]
            pub const MODE_5: u32 = 0x05;
            #[doc = "16 bpp (YCbCr422)"]
            pub const MODE_6: u32 = 0x06;
        }
    }
    #[doc = "PDI selected on LCDIFv2 plane number"]
    pub mod PDI_SEL {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "PDI selected on LCDIFv2 plane 0"]
            pub const PDI_SEL_0: u32 = 0;
            #[doc = "PDI selected on LCDIFv2 plane 1"]
            pub const PDI_SEL_1: u32 = 0x01;
        }
    }
    #[doc = "Enable PDI input data to LCDIFv2 display"]
    pub mod PDI_EN {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable PDI input data"]
            pub const PDI_EN_0: u32 = 0;
            #[doc = "Enable PDI input data"]
            pub const PDI_EN_1: u32 = 0x01;
        }
    }
}
#[doc = "Control Descriptor Layer 1 Register"]
pub mod CTRLDESCL0_1 {
    #[doc = "Width of the layer in pixels"]
    pub mod WIDTH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Height of the layer in pixels"]
    pub mod HEIGHT {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Control Descriptor Layer 2 Register"]
pub mod CTRLDESCL0_2 {
    #[doc = "The horizontal position of left-hand column of the layer, where 0 is the left-hand column of the panel, only positive values are to the right the left-hand column of the panel"]
    pub mod POSX {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The vertical position of top row of the layer, where 0 is the top row of the panel, only positive values are below the top row of the panel"]
    pub mod POSY {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Control Descriptor Layer 3 Register"]
pub mod CTRLDESCL0_3 {
    #[doc = "Number of bytes between 2 vertically adjacent pixels in system memory. Byte granularity is supported, but SW should align to 64B boundry"]
    pub mod PITCH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Control Descriptor Layer 4 Register"]
pub mod CTRLDESCL0_4 {
    #[doc = "Address of layer data in the memory. The address programmed should be 64-bit aligned"]
    pub mod ADDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Control Descriptor Layer 5 Register"]
pub mod CTRLDESCL0_5 {
    #[doc = "Alpha Blending Mode"]
    pub mod AB_MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No alpha Blending (The SAFETY_EN bit need set to 1)"]
            pub const AB_MODE_0: u32 = 0;
            #[doc = "Blend with global ALPHA"]
            pub const AB_MODE_1: u32 = 0x01;
            #[doc = "Blend with embedded ALPHA"]
            pub const AB_MODE_2: u32 = 0x02;
            #[doc = "Blend with PoterDuff enable"]
            pub const AB_MODE_3: u32 = 0x03;
        }
    }
    #[doc = "PoterDuff factor mode"]
    pub mod PD_FACTOR_MODE {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Using 1"]
            pub const PD_FACTOR_MODE_0: u32 = 0;
            #[doc = "Using 0"]
            pub const PD_FACTOR_MODE_1: u32 = 0x01;
            #[doc = "Using straight alpha"]
            pub const PD_FACTOR_MODE_2: u32 = 0x02;
            #[doc = "Using inverse alpha"]
            pub const PD_FACTOR_MODE_3: u32 = 0x03;
        }
    }
    #[doc = "PoterDuff global alpha mode"]
    pub mod PD_GLOBAL_ALPHA_MODE {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Using global alpha"]
            pub const PD_GLOBAL_ALPHA_MODE_0: u32 = 0;
            #[doc = "Using local alpha"]
            pub const PD_GLOBAL_ALPHA_MODE_1: u32 = 0x01;
            #[doc = "Using scaled alpha"]
            pub const PD_GLOBAL_ALPHA_MODE_2: u32 = 0x02;
            #[doc = "Using scaled alpha"]
            pub const PD_GLOBAL_ALPHA_MODE_3: u32 = 0x03;
        }
    }
    #[doc = "PoterDuff alpha mode"]
    pub mod PD_ALPHA_MODE {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Straight mode for Porter Duff alpha"]
            pub const PD_ALPHA_MODE_0: u32 = 0;
            #[doc = "Inversed mode for Porter Duff alpha"]
            pub const PD_ALPHA_MODE_1: u32 = 0x01;
        }
    }
    #[doc = "PoterDuff alpha mode"]
    pub mod PD_COLOR_MODE {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Straight mode for Porter Duff color"]
            pub const PD_COLOR_MODE_0: u32 = 0;
            #[doc = "Inversed mode for Porter Duff color"]
            pub const PD_COLOR_MODE_1: u32 = 0x01;
        }
    }
    #[doc = "The YUV422 input format selection"]
    pub mod YUV_FORMAT {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The YVYU422 8bit sequence is U1,Y1,V1,Y2"]
            pub const YUV_FORMAT_0: u32 = 0;
            #[doc = "The YVYU422 8bit sequence is V1,Y1,U1,Y2"]
            pub const YUV_FORMAT_1: u32 = 0x01;
            #[doc = "The YVYU422 8bit sequence is Y1,U1,Y2,V1"]
            pub const YUV_FORMAT_2: u32 = 0x02;
            #[doc = "The YVYU422 8bit sequence is Y1,V1,Y2,U1"]
            pub const YUV_FORMAT_3: u32 = 0x03;
        }
    }
    #[doc = "Global Alpha"]
    pub mod GLOBAL_ALPHA {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Layer encoding format (bit per pixel)"]
    pub mod BPP {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "1 bpp"]
            pub const BPP_0: u32 = 0;
            #[doc = "2 bpp"]
            pub const BPP_1: u32 = 0x01;
            #[doc = "4 bpp"]
            pub const BPP_2: u32 = 0x02;
            #[doc = "8 bpp"]
            pub const BPP_3: u32 = 0x03;
            #[doc = "16 bpp (RGB565)"]
            pub const BPP_4: u32 = 0x04;
            #[doc = "16 bpp (ARGB1555)"]
            pub const BPP_5: u32 = 0x05;
            #[doc = "16 bpp (ARGB4444)"]
            pub const BPP_6: u32 = 0x06;
            #[doc = "YCbCr422 (Only layer 0/1 can support this format)"]
            pub const BPP_7: u32 = 0x07;
            #[doc = "24 bpp (RGB888)"]
            pub const BPP_8: u32 = 0x08;
            #[doc = "32 bpp (ARGB8888)"]
            pub const BPP_9: u32 = 0x09;
            #[doc = "32 bpp (ABGR8888)"]
            pub const BPP_10: u32 = 0x0a;
        }
    }
    #[doc = "Safety Mode Enable Bit"]
    pub mod SAFETY_EN {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Safety Mode is disabled"]
            pub const SAFETY_EN_0: u32 = 0;
            #[doc = "Safety Mode is enabled for this layer"]
            pub const SAFETY_EN_1: u32 = 0x01;
        }
    }
    #[doc = "Shadow Load Enable"]
    pub mod SHADOW_LOAD_EN {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enable the layer for DMA"]
    pub mod EN {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "OFF"]
            pub const EN_0: u32 = 0;
            #[doc = "ON"]
            pub const EN_1: u32 = 0x01;
        }
    }
}
#[doc = "Control Descriptor Layer 6 Register"]
pub mod CTRLDESCL0_6 {
    #[doc = "Background B component value"]
    pub mod BCLR_B {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Background G component value"]
    pub mod BCLR_G {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Background R component value"]
    pub mod BCLR_R {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Color Space Conversion Coefficient Register 0"]
pub mod CSC0_COEF0 {
    #[doc = "Two's compliment amplitude offset implicit in the Y data. For YUV, this is typically 0 and for YCbCr, this is typically -16 (0x1F0)"]
    pub mod Y_OFFSET {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Two's compliment phase offset implicit for CbCr data. Generally used for YCbCr to RGB conversion. YCbCr=0x180, YUV=0x000 (typically -128 or 0x180 to indicate normalized -0.5 to 0.5 range)"]
    pub mod UV_OFFSET {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Two's compliment Y multiplier coefficient. YUV=0x100 (1.000) YCbCr=0x12A (1.164)"]
    pub mod C0 {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enable the CSC unit in the LCDIFv2 plane data path"]
    pub mod ENABLE {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The CSC is bypassed and the input pixels are RGB data already"]
            pub const ENABLE_0: u32 = 0;
            #[doc = "The CSC is enabled and the pixels will be converted to RGB data"]
            pub const ENABLE_1: u32 = 0x01;
        }
    }
    #[doc = "This bit changes the behavior when performing U/V converting"]
    pub mod YCBCR_MODE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Converting YUV to RGB data"]
            pub const YCBCR_MODE_0: u32 = 0;
            #[doc = "Converting YCbCr to RGB data"]
            pub const YCBCR_MODE_1: u32 = 0x01;
        }
    }
}
#[doc = "Color Space Conversion Coefficient Register 1"]
pub mod CSC0_COEF1 {
    #[doc = "Two's compliment Blue U/Cb multiplier coefficient. YUV=0x208 (2.032) YCbCr=0x204 (2.017)"]
    pub mod C4 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Two's compliment Red V/Cr multiplier coefficient. YUV=0x123 (1.140) YCbCr=0x198 (1.596)"]
    pub mod C1 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Color Space Conversion Coefficient Register 2"]
pub mod CSC0_COEF2 {
    #[doc = "Two's compliment Green U/Cb multiplier coefficient. YUV=0x79C (-0.394) YCbCr=0x79C (-0.392)"]
    pub mod C3 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Two's compliment Green V/Cr multiplier coefficient. YUV=0x76B (-0.581) YCbCr=0x730 (-0.813)"]
    pub mod C2 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Control Descriptor Layer 1 Register"]
pub mod CTRLDESCL1_1 {
    #[doc = "Width of the layer in pixels"]
    pub mod WIDTH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Height of the layer in pixels"]
    pub mod HEIGHT {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Control Descriptor Layer 2 Register"]
pub mod CTRLDESCL1_2 {
    #[doc = "The horizontal position of left-hand column of the layer, where 0 is the left-hand column of the panel, only positive values are to the right the left-hand column of the panel"]
    pub mod POSX {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The vertical position of top row of the layer, where 0 is the top row of the panel, only positive values are below the top row of the panel"]
    pub mod POSY {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Control Descriptor Layer 3 Register"]
pub mod CTRLDESCL1_3 {
    #[doc = "Number of bytes between 2 vertically adjacent pixels in system memory. Byte granularity is supported, but SW should align to 64B boundry"]
    pub mod PITCH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Control Descriptor Layer 4 Register"]
pub mod CTRLDESCL1_4 {
    #[doc = "Address of layer data in the memory. The address programmed should be 64-bit aligned"]
    pub mod ADDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Control Descriptor Layer 5 Register"]
pub mod CTRLDESCL1_5 {
    #[doc = "Alpha Blending Mode"]
    pub mod AB_MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No alpha Blending (The SAFETY_EN bit need set to 1)"]
            pub const AB_MODE_0: u32 = 0;
            #[doc = "Blend with global ALPHA"]
            pub const AB_MODE_1: u32 = 0x01;
            #[doc = "Blend with embedded ALPHA"]
            pub const AB_MODE_2: u32 = 0x02;
            #[doc = "Blend with PoterDuff enable"]
            pub const AB_MODE_3: u32 = 0x03;
        }
    }
    #[doc = "PoterDuff factor mode"]
    pub mod PD_FACTOR_MODE {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Using 1"]
            pub const PD_FACTOR_MODE_0: u32 = 0;
            #[doc = "Using 0"]
            pub const PD_FACTOR_MODE_1: u32 = 0x01;
            #[doc = "Using straight alpha"]
            pub const PD_FACTOR_MODE_2: u32 = 0x02;
            #[doc = "Using inverse alpha"]
            pub const PD_FACTOR_MODE_3: u32 = 0x03;
        }
    }
    #[doc = "PoterDuff global alpha mode"]
    pub mod PD_GLOBAL_ALPHA_MODE {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Using global alpha"]
            pub const PD_GLOBAL_ALPHA_MODE_0: u32 = 0;
            #[doc = "Using local alpha"]
            pub const PD_GLOBAL_ALPHA_MODE_1: u32 = 0x01;
            #[doc = "Using scaled alpha"]
            pub const PD_GLOBAL_ALPHA_MODE_2: u32 = 0x02;
            #[doc = "Using scaled alpha"]
            pub const PD_GLOBAL_ALPHA_MODE_3: u32 = 0x03;
        }
    }
    #[doc = "PoterDuff alpha mode"]
    pub mod PD_ALPHA_MODE {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Straight mode for Porter Duff alpha"]
            pub const PD_ALPHA_MODE_0: u32 = 0;
            #[doc = "Inversed mode for Porter Duff alpha"]
            pub const PD_ALPHA_MODE_1: u32 = 0x01;
        }
    }
    #[doc = "PoterDuff alpha mode"]
    pub mod PD_COLOR_MODE {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Straight mode for Porter Duff color"]
            pub const PD_COLOR_MODE_0: u32 = 0;
            #[doc = "Inversed mode for Porter Duff color"]
            pub const PD_COLOR_MODE_1: u32 = 0x01;
        }
    }
    #[doc = "The YUV422 input format selection"]
    pub mod YUV_FORMAT {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The YVYU422 8bit sequence is U1,Y1,V1,Y2"]
            pub const YUV_FORMAT_0: u32 = 0;
            #[doc = "The YVYU422 8bit sequence is V1,Y1,U1,Y2"]
            pub const YUV_FORMAT_1: u32 = 0x01;
            #[doc = "The YVYU422 8bit sequence is Y1,U1,Y2,V1"]
            pub const YUV_FORMAT_2: u32 = 0x02;
            #[doc = "The YVYU422 8bit sequence is Y1,V1,Y2,U1"]
            pub const YUV_FORMAT_3: u32 = 0x03;
        }
    }
    #[doc = "Global Alpha"]
    pub mod GLOBAL_ALPHA {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Layer encoding format (bit per pixel)"]
    pub mod BPP {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "1 bpp"]
            pub const BPP_0: u32 = 0;
            #[doc = "2 bpp"]
            pub const BPP_1: u32 = 0x01;
            #[doc = "4 bpp"]
            pub const BPP_2: u32 = 0x02;
            #[doc = "8 bpp"]
            pub const BPP_3: u32 = 0x03;
            #[doc = "16 bpp (RGB565)"]
            pub const BPP_4: u32 = 0x04;
            #[doc = "16 bpp (ARGB1555)"]
            pub const BPP_5: u32 = 0x05;
            #[doc = "16 bpp (ARGB4444)"]
            pub const BPP_6: u32 = 0x06;
            #[doc = "YCbCr422 (Only layer 0/1 can support this format)"]
            pub const BPP_7: u32 = 0x07;
            #[doc = "24 bpp (RGB888)"]
            pub const BPP_8: u32 = 0x08;
            #[doc = "32 bpp (ARGB8888)"]
            pub const BPP_9: u32 = 0x09;
            #[doc = "32 bpp (ABGR8888)"]
            pub const BPP_10: u32 = 0x0a;
        }
    }
    #[doc = "Safety Mode Enable Bit"]
    pub mod SAFETY_EN {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Safety Mode is disabled"]
            pub const SAFETY_EN_0: u32 = 0;
            #[doc = "Safety Mode is enabled for this layer"]
            pub const SAFETY_EN_1: u32 = 0x01;
        }
    }
    #[doc = "Shadow Load Enable"]
    pub mod SHADOW_LOAD_EN {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enable the layer for DMA"]
    pub mod EN {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "OFF"]
            pub const EN_0: u32 = 0;
            #[doc = "ON"]
            pub const EN_1: u32 = 0x01;
        }
    }
}
#[doc = "Control Descriptor Layer 6 Register"]
pub mod CTRLDESCL1_6 {
    #[doc = "Background B component value"]
    pub mod BCLR_B {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Background G component value"]
    pub mod BCLR_G {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Background R component value"]
    pub mod BCLR_R {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Color Space Conversion Coefficient Register 0"]
pub mod CSC1_COEF0 {
    #[doc = "Two's compliment amplitude offset implicit in the Y data. For YUV, this is typically 0 and for YCbCr, this is typically -16 (0x1F0)"]
    pub mod Y_OFFSET {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Two's compliment phase offset implicit for CbCr data. Generally used for YCbCr to RGB conversion. YCbCr=0x180, YUV=0x000 (typically -128 or 0x180 to indicate normalized -0.5 to 0.5 range)"]
    pub mod UV_OFFSET {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Two's compliment Y multiplier coefficient. YUV=0x100 (1.000) YCbCr=0x12A (1.164)"]
    pub mod C0 {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enable the CSC unit in the LCDIFv2 plane data path"]
    pub mod ENABLE {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The CSC is bypassed and the input pixels are RGB data already"]
            pub const ENABLE_0: u32 = 0;
            #[doc = "The CSC is enabled and the pixels will be converted to RGB data"]
            pub const ENABLE_1: u32 = 0x01;
        }
    }
    #[doc = "This bit changes the behavior when performing U/V converting"]
    pub mod YCBCR_MODE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Converting YUV to RGB data"]
            pub const YCBCR_MODE_0: u32 = 0;
            #[doc = "Converting YCbCr to RGB data"]
            pub const YCBCR_MODE_1: u32 = 0x01;
        }
    }
}
#[doc = "Color Space Conversion Coefficient Register 1"]
pub mod CSC1_COEF1 {
    #[doc = "Two's compliment Blue U/Cb multiplier coefficient. YUV=0x208 (2.032) YCbCr=0x204 (2.017)"]
    pub mod C4 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Two's compliment Red V/Cr multiplier coefficient. YUV=0x123 (1.140) YCbCr=0x198 (1.596)"]
    pub mod C1 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Color Space Conversion Coefficient Register 2"]
pub mod CSC1_COEF2 {
    #[doc = "Two's compliment Green U/Cb multiplier coefficient. YUV=0x79C (-0.394) YCbCr=0x79C (-0.392)"]
    pub mod C3 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Two's compliment Green V/Cr multiplier coefficient. YUV=0x76B (-0.581) YCbCr=0x730 (-0.813)"]
    pub mod C2 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Control Descriptor Layer 1 Register"]
pub mod CTRLDESCL2_1 {
    #[doc = "Width of the layer in pixels"]
    pub mod WIDTH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Height of the layer in pixels"]
    pub mod HEIGHT {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Control Descriptor Layer 2 Register"]
pub mod CTRLDESCL2_2 {
    #[doc = "The horizontal position of left-hand column of the layer, where 0 is the left-hand column of the panel, only positive values are to the right the left-hand column of the panel"]
    pub mod POSX {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The vertical position of top row of the layer, where 0 is the top row of the panel, only positive values are below the top row of the panel"]
    pub mod POSY {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Control Descriptor Layer 3 Register"]
pub mod CTRLDESCL2_3 {
    #[doc = "Number of bytes between 2 vertically adjacent pixels in system memory. Byte granularity is supported, but SW should align to 64B boundry"]
    pub mod PITCH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Control Descriptor Layer 4 Register"]
pub mod CTRLDESCL2_4 {
    #[doc = "Address of layer data in the memory. The address programmed should be 64-bit aligned"]
    pub mod ADDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Control Descriptor Layer 5 Register"]
pub mod CTRLDESCL2_5 {
    #[doc = "Alpha Blending Mode"]
    pub mod AB_MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No alpha Blending (The SAFETY_EN bit need set to 1)"]
            pub const AB_MODE_0: u32 = 0;
            #[doc = "Blend with global ALPHA"]
            pub const AB_MODE_1: u32 = 0x01;
            #[doc = "Blend with embedded ALPHA"]
            pub const AB_MODE_2: u32 = 0x02;
            #[doc = "Blend with PoterDuff enable"]
            pub const AB_MODE_3: u32 = 0x03;
        }
    }
    #[doc = "PoterDuff factor mode"]
    pub mod PD_FACTOR_MODE {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Using 1"]
            pub const PD_FACTOR_MODE_0: u32 = 0;
            #[doc = "Using 0"]
            pub const PD_FACTOR_MODE_1: u32 = 0x01;
            #[doc = "Using straight alpha"]
            pub const PD_FACTOR_MODE_2: u32 = 0x02;
            #[doc = "Using inverse alpha"]
            pub const PD_FACTOR_MODE_3: u32 = 0x03;
        }
    }
    #[doc = "PoterDuff global alpha mode"]
    pub mod PD_GLOBAL_ALPHA_MODE {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Using global alpha"]
            pub const PD_GLOBAL_ALPHA_MODE_0: u32 = 0;
            #[doc = "Using local alpha"]
            pub const PD_GLOBAL_ALPHA_MODE_1: u32 = 0x01;
            #[doc = "Using scaled alpha"]
            pub const PD_GLOBAL_ALPHA_MODE_2: u32 = 0x02;
            #[doc = "Using scaled alpha"]
            pub const PD_GLOBAL_ALPHA_MODE_3: u32 = 0x03;
        }
    }
    #[doc = "PoterDuff alpha mode"]
    pub mod PD_ALPHA_MODE {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Straight mode for Porter Duff alpha"]
            pub const PD_ALPHA_MODE_0: u32 = 0;
            #[doc = "Inversed mode for Porter Duff alpha"]
            pub const PD_ALPHA_MODE_1: u32 = 0x01;
        }
    }
    #[doc = "PoterDuff alpha mode"]
    pub mod PD_COLOR_MODE {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Straight mode for Porter Duff color"]
            pub const PD_COLOR_MODE_0: u32 = 0;
            #[doc = "Inversed mode for Porter Duff color"]
            pub const PD_COLOR_MODE_1: u32 = 0x01;
        }
    }
    #[doc = "The YUV422 input format selection"]
    pub mod YUV_FORMAT {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The YVYU422 8bit sequence is U1,Y1,V1,Y2"]
            pub const YUV_FORMAT_0: u32 = 0;
            #[doc = "The YVYU422 8bit sequence is V1,Y1,U1,Y2"]
            pub const YUV_FORMAT_1: u32 = 0x01;
            #[doc = "The YVYU422 8bit sequence is Y1,U1,Y2,V1"]
            pub const YUV_FORMAT_2: u32 = 0x02;
            #[doc = "The YVYU422 8bit sequence is Y1,V1,Y2,U1"]
            pub const YUV_FORMAT_3: u32 = 0x03;
        }
    }
    #[doc = "Global Alpha"]
    pub mod GLOBAL_ALPHA {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Layer encoding format (bit per pixel)"]
    pub mod BPP {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "1 bpp"]
            pub const BPP_0: u32 = 0;
            #[doc = "2 bpp"]
            pub const BPP_1: u32 = 0x01;
            #[doc = "4 bpp"]
            pub const BPP_2: u32 = 0x02;
            #[doc = "8 bpp"]
            pub const BPP_3: u32 = 0x03;
            #[doc = "16 bpp (RGB565)"]
            pub const BPP_4: u32 = 0x04;
            #[doc = "16 bpp (ARGB1555)"]
            pub const BPP_5: u32 = 0x05;
            #[doc = "16 bpp (ARGB4444)"]
            pub const BPP_6: u32 = 0x06;
            #[doc = "YCbCr422 (Only layer 0/1 can support this format)"]
            pub const BPP_7: u32 = 0x07;
            #[doc = "24 bpp (RGB888)"]
            pub const BPP_8: u32 = 0x08;
            #[doc = "32 bpp (ARGB8888)"]
            pub const BPP_9: u32 = 0x09;
            #[doc = "32 bpp (ABGR8888)"]
            pub const BPP_10: u32 = 0x0a;
        }
    }
    #[doc = "Safety Mode Enable Bit"]
    pub mod SAFETY_EN {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Safety Mode is disabled"]
            pub const SAFETY_EN_0: u32 = 0;
            #[doc = "Safety Mode is enabled for this layer"]
            pub const SAFETY_EN_1: u32 = 0x01;
        }
    }
    #[doc = "Shadow Load Enable"]
    pub mod SHADOW_LOAD_EN {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enable the layer for DMA"]
    pub mod EN {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "OFF"]
            pub const EN_0: u32 = 0;
            #[doc = "ON"]
            pub const EN_1: u32 = 0x01;
        }
    }
}
#[doc = "Control Descriptor Layer 6 Register"]
pub mod CTRLDESCL2_6 {
    #[doc = "Background B component value"]
    pub mod BCLR_B {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Background G component value"]
    pub mod BCLR_G {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Background R component value"]
    pub mod BCLR_R {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Control Descriptor Layer 1 Register"]
pub mod CTRLDESCL3_1 {
    #[doc = "Width of the layer in pixels"]
    pub mod WIDTH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Height of the layer in pixels"]
    pub mod HEIGHT {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Control Descriptor Layer 2 Register"]
pub mod CTRLDESCL3_2 {
    #[doc = "The horizontal position of left-hand column of the layer, where 0 is the left-hand column of the panel, only positive values are to the right the left-hand column of the panel"]
    pub mod POSX {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The vertical position of top row of the layer, where 0 is the top row of the panel, only positive values are below the top row of the panel"]
    pub mod POSY {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Control Descriptor Layer 3 Register"]
pub mod CTRLDESCL3_3 {
    #[doc = "Number of bytes between 2 vertically adjacent pixels in system memory. Byte granularity is supported, but SW should align to 64B boundry"]
    pub mod PITCH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Control Descriptor Layer 4 Register"]
pub mod CTRLDESCL3_4 {
    #[doc = "Address of layer data in the memory. The address programmed should be 64-bit aligned"]
    pub mod ADDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Control Descriptor Layer 5 Register"]
pub mod CTRLDESCL3_5 {
    #[doc = "Alpha Blending Mode"]
    pub mod AB_MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No alpha Blending (The SAFETY_EN bit need set to 1)"]
            pub const AB_MODE_0: u32 = 0;
            #[doc = "Blend with global ALPHA"]
            pub const AB_MODE_1: u32 = 0x01;
            #[doc = "Blend with embedded ALPHA"]
            pub const AB_MODE_2: u32 = 0x02;
            #[doc = "Blend with PoterDuff enable"]
            pub const AB_MODE_3: u32 = 0x03;
        }
    }
    #[doc = "PoterDuff factor mode"]
    pub mod PD_FACTOR_MODE {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Using 1"]
            pub const PD_FACTOR_MODE_0: u32 = 0;
            #[doc = "Using 0"]
            pub const PD_FACTOR_MODE_1: u32 = 0x01;
            #[doc = "Using straight alpha"]
            pub const PD_FACTOR_MODE_2: u32 = 0x02;
            #[doc = "Using inverse alpha"]
            pub const PD_FACTOR_MODE_3: u32 = 0x03;
        }
    }
    #[doc = "PoterDuff global alpha mode"]
    pub mod PD_GLOBAL_ALPHA_MODE {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Using global alpha"]
            pub const PD_GLOBAL_ALPHA_MODE_0: u32 = 0;
            #[doc = "Using local alpha"]
            pub const PD_GLOBAL_ALPHA_MODE_1: u32 = 0x01;
            #[doc = "Using scaled alpha"]
            pub const PD_GLOBAL_ALPHA_MODE_2: u32 = 0x02;
            #[doc = "Using scaled alpha"]
            pub const PD_GLOBAL_ALPHA_MODE_3: u32 = 0x03;
        }
    }
    #[doc = "PoterDuff alpha mode"]
    pub mod PD_ALPHA_MODE {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Straight mode for Porter Duff alpha"]
            pub const PD_ALPHA_MODE_0: u32 = 0;
            #[doc = "Inversed mode for Porter Duff alpha"]
            pub const PD_ALPHA_MODE_1: u32 = 0x01;
        }
    }
    #[doc = "PoterDuff alpha mode"]
    pub mod PD_COLOR_MODE {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Straight mode for Porter Duff color"]
            pub const PD_COLOR_MODE_0: u32 = 0;
            #[doc = "Inversed mode for Porter Duff color"]
            pub const PD_COLOR_MODE_1: u32 = 0x01;
        }
    }
    #[doc = "The YUV422 input format selection"]
    pub mod YUV_FORMAT {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The YVYU422 8bit sequence is U1,Y1,V1,Y2"]
            pub const YUV_FORMAT_0: u32 = 0;
            #[doc = "The YVYU422 8bit sequence is V1,Y1,U1,Y2"]
            pub const YUV_FORMAT_1: u32 = 0x01;
            #[doc = "The YVYU422 8bit sequence is Y1,U1,Y2,V1"]
            pub const YUV_FORMAT_2: u32 = 0x02;
            #[doc = "The YVYU422 8bit sequence is Y1,V1,Y2,U1"]
            pub const YUV_FORMAT_3: u32 = 0x03;
        }
    }
    #[doc = "Global Alpha"]
    pub mod GLOBAL_ALPHA {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Layer encoding format (bit per pixel)"]
    pub mod BPP {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "1 bpp"]
            pub const BPP_0: u32 = 0;
            #[doc = "2 bpp"]
            pub const BPP_1: u32 = 0x01;
            #[doc = "4 bpp"]
            pub const BPP_2: u32 = 0x02;
            #[doc = "8 bpp"]
            pub const BPP_3: u32 = 0x03;
            #[doc = "16 bpp (RGB565)"]
            pub const BPP_4: u32 = 0x04;
            #[doc = "16 bpp (ARGB1555)"]
            pub const BPP_5: u32 = 0x05;
            #[doc = "16 bpp (ARGB4444)"]
            pub const BPP_6: u32 = 0x06;
            #[doc = "YCbCr422 (Only layer 0/1 can support this format)"]
            pub const BPP_7: u32 = 0x07;
            #[doc = "24 bpp (RGB888)"]
            pub const BPP_8: u32 = 0x08;
            #[doc = "32 bpp (ARGB8888)"]
            pub const BPP_9: u32 = 0x09;
            #[doc = "32 bpp (ABGR8888)"]
            pub const BPP_10: u32 = 0x0a;
        }
    }
    #[doc = "Safety Mode Enable Bit"]
    pub mod SAFETY_EN {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Safety Mode is disabled"]
            pub const SAFETY_EN_0: u32 = 0;
            #[doc = "Safety Mode is enabled for this layer"]
            pub const SAFETY_EN_1: u32 = 0x01;
        }
    }
    #[doc = "Shadow Load Enable"]
    pub mod SHADOW_LOAD_EN {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enable the layer for DMA"]
    pub mod EN {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "OFF"]
            pub const EN_0: u32 = 0;
            #[doc = "ON"]
            pub const EN_1: u32 = 0x01;
        }
    }
}
#[doc = "Control Descriptor Layer 6 Register"]
pub mod CTRLDESCL3_6 {
    #[doc = "Background B component value"]
    pub mod BCLR_B {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Background G component value"]
    pub mod BCLR_G {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Background R component value"]
    pub mod BCLR_R {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Control Descriptor Layer 1 Register"]
pub mod CTRLDESCL4_1 {
    #[doc = "Width of the layer in pixels"]
    pub mod WIDTH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Height of the layer in pixels"]
    pub mod HEIGHT {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Control Descriptor Layer 2 Register"]
pub mod CTRLDESCL4_2 {
    #[doc = "The horizontal position of left-hand column of the layer, where 0 is the left-hand column of the panel, only positive values are to the right the left-hand column of the panel"]
    pub mod POSX {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The vertical position of top row of the layer, where 0 is the top row of the panel, only positive values are below the top row of the panel"]
    pub mod POSY {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Control Descriptor Layer 3 Register"]
pub mod CTRLDESCL4_3 {
    #[doc = "Number of bytes between 2 vertically adjacent pixels in system memory. Byte granularity is supported, but SW should align to 64B boundry"]
    pub mod PITCH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Control Descriptor Layer 4 Register"]
pub mod CTRLDESCL4_4 {
    #[doc = "Address of layer data in the memory. The address programmed should be 64-bit aligned"]
    pub mod ADDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Control Descriptor Layer 5 Register"]
pub mod CTRLDESCL4_5 {
    #[doc = "Alpha Blending Mode"]
    pub mod AB_MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No alpha Blending (The SAFETY_EN bit need set to 1)"]
            pub const AB_MODE_0: u32 = 0;
            #[doc = "Blend with global ALPHA"]
            pub const AB_MODE_1: u32 = 0x01;
            #[doc = "Blend with embedded ALPHA"]
            pub const AB_MODE_2: u32 = 0x02;
            #[doc = "Blend with PoterDuff enable"]
            pub const AB_MODE_3: u32 = 0x03;
        }
    }
    #[doc = "PoterDuff factor mode"]
    pub mod PD_FACTOR_MODE {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Using 1"]
            pub const PD_FACTOR_MODE_0: u32 = 0;
            #[doc = "Using 0"]
            pub const PD_FACTOR_MODE_1: u32 = 0x01;
            #[doc = "Using straight alpha"]
            pub const PD_FACTOR_MODE_2: u32 = 0x02;
            #[doc = "Using inverse alpha"]
            pub const PD_FACTOR_MODE_3: u32 = 0x03;
        }
    }
    #[doc = "PoterDuff global alpha mode"]
    pub mod PD_GLOBAL_ALPHA_MODE {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Using global alpha"]
            pub const PD_GLOBAL_ALPHA_MODE_0: u32 = 0;
            #[doc = "Using local alpha"]
            pub const PD_GLOBAL_ALPHA_MODE_1: u32 = 0x01;
            #[doc = "Using scaled alpha"]
            pub const PD_GLOBAL_ALPHA_MODE_2: u32 = 0x02;
            #[doc = "Using scaled alpha"]
            pub const PD_GLOBAL_ALPHA_MODE_3: u32 = 0x03;
        }
    }
    #[doc = "PoterDuff alpha mode"]
    pub mod PD_ALPHA_MODE {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Straight mode for Porter Duff alpha"]
            pub const PD_ALPHA_MODE_0: u32 = 0;
            #[doc = "Inversed mode for Porter Duff alpha"]
            pub const PD_ALPHA_MODE_1: u32 = 0x01;
        }
    }
    #[doc = "PoterDuff alpha mode"]
    pub mod PD_COLOR_MODE {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Straight mode for Porter Duff color"]
            pub const PD_COLOR_MODE_0: u32 = 0;
            #[doc = "Inversed mode for Porter Duff color"]
            pub const PD_COLOR_MODE_1: u32 = 0x01;
        }
    }
    #[doc = "The YUV422 input format selection"]
    pub mod YUV_FORMAT {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The YVYU422 8bit sequence is U1,Y1,V1,Y2"]
            pub const YUV_FORMAT_0: u32 = 0;
            #[doc = "The YVYU422 8bit sequence is V1,Y1,U1,Y2"]
            pub const YUV_FORMAT_1: u32 = 0x01;
            #[doc = "The YVYU422 8bit sequence is Y1,U1,Y2,V1"]
            pub const YUV_FORMAT_2: u32 = 0x02;
            #[doc = "The YVYU422 8bit sequence is Y1,V1,Y2,U1"]
            pub const YUV_FORMAT_3: u32 = 0x03;
        }
    }
    #[doc = "Global Alpha"]
    pub mod GLOBAL_ALPHA {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Layer encoding format (bit per pixel)"]
    pub mod BPP {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "1 bpp"]
            pub const BPP_0: u32 = 0;
            #[doc = "2 bpp"]
            pub const BPP_1: u32 = 0x01;
            #[doc = "4 bpp"]
            pub const BPP_2: u32 = 0x02;
            #[doc = "8 bpp"]
            pub const BPP_3: u32 = 0x03;
            #[doc = "16 bpp (RGB565)"]
            pub const BPP_4: u32 = 0x04;
            #[doc = "16 bpp (ARGB1555)"]
            pub const BPP_5: u32 = 0x05;
            #[doc = "16 bpp (ARGB4444)"]
            pub const BPP_6: u32 = 0x06;
            #[doc = "YCbCr422 (Only layer 0/1 can support this format)"]
            pub const BPP_7: u32 = 0x07;
            #[doc = "24 bpp (RGB888)"]
            pub const BPP_8: u32 = 0x08;
            #[doc = "32 bpp (ARGB8888)"]
            pub const BPP_9: u32 = 0x09;
            #[doc = "32 bpp (ABGR8888)"]
            pub const BPP_10: u32 = 0x0a;
        }
    }
    #[doc = "Safety Mode Enable Bit"]
    pub mod SAFETY_EN {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Safety Mode is disabled"]
            pub const SAFETY_EN_0: u32 = 0;
            #[doc = "Safety Mode is enabled for this layer"]
            pub const SAFETY_EN_1: u32 = 0x01;
        }
    }
    #[doc = "Shadow Load Enable"]
    pub mod SHADOW_LOAD_EN {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enable the layer for DMA"]
    pub mod EN {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "OFF"]
            pub const EN_0: u32 = 0;
            #[doc = "ON"]
            pub const EN_1: u32 = 0x01;
        }
    }
}
#[doc = "Control Descriptor Layer 6 Register"]
pub mod CTRLDESCL4_6 {
    #[doc = "Background B component value"]
    pub mod BCLR_B {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Background G component value"]
    pub mod BCLR_G {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Background R component value"]
    pub mod BCLR_R {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Control Descriptor Layer 1 Register"]
pub mod CTRLDESCL5_1 {
    #[doc = "Width of the layer in pixels"]
    pub mod WIDTH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Height of the layer in pixels"]
    pub mod HEIGHT {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Control Descriptor Layer 2 Register"]
pub mod CTRLDESCL5_2 {
    #[doc = "The horizontal position of left-hand column of the layer, where 0 is the left-hand column of the panel, only positive values are to the right the left-hand column of the panel"]
    pub mod POSX {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The vertical position of top row of the layer, where 0 is the top row of the panel, only positive values are below the top row of the panel"]
    pub mod POSY {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Control Descriptor Layer 3 Register"]
pub mod CTRLDESCL5_3 {
    #[doc = "Number of bytes between 2 vertically adjacent pixels in system memory. Byte granularity is supported, but SW should align to 64B boundry"]
    pub mod PITCH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Control Descriptor Layer 4 Register"]
pub mod CTRLDESCL5_4 {
    #[doc = "Address of layer data in the memory. The address programmed should be 64-bit aligned"]
    pub mod ADDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Control Descriptor Layer 5 Register"]
pub mod CTRLDESCL5_5 {
    #[doc = "Alpha Blending Mode"]
    pub mod AB_MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No alpha Blending (The SAFETY_EN bit need set to 1)"]
            pub const AB_MODE_0: u32 = 0;
            #[doc = "Blend with global ALPHA"]
            pub const AB_MODE_1: u32 = 0x01;
            #[doc = "Blend with embedded ALPHA"]
            pub const AB_MODE_2: u32 = 0x02;
            #[doc = "Blend with PoterDuff enable"]
            pub const AB_MODE_3: u32 = 0x03;
        }
    }
    #[doc = "PoterDuff factor mode"]
    pub mod PD_FACTOR_MODE {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Using 1"]
            pub const PD_FACTOR_MODE_0: u32 = 0;
            #[doc = "Using 0"]
            pub const PD_FACTOR_MODE_1: u32 = 0x01;
            #[doc = "Using straight alpha"]
            pub const PD_FACTOR_MODE_2: u32 = 0x02;
            #[doc = "Using inverse alpha"]
            pub const PD_FACTOR_MODE_3: u32 = 0x03;
        }
    }
    #[doc = "PoterDuff global alpha mode"]
    pub mod PD_GLOBAL_ALPHA_MODE {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Using global alpha"]
            pub const PD_GLOBAL_ALPHA_MODE_0: u32 = 0;
            #[doc = "Using local alpha"]
            pub const PD_GLOBAL_ALPHA_MODE_1: u32 = 0x01;
            #[doc = "Using scaled alpha"]
            pub const PD_GLOBAL_ALPHA_MODE_2: u32 = 0x02;
            #[doc = "Using scaled alpha"]
            pub const PD_GLOBAL_ALPHA_MODE_3: u32 = 0x03;
        }
    }
    #[doc = "PoterDuff alpha mode"]
    pub mod PD_ALPHA_MODE {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Straight mode for Porter Duff alpha"]
            pub const PD_ALPHA_MODE_0: u32 = 0;
            #[doc = "Inversed mode for Porter Duff alpha"]
            pub const PD_ALPHA_MODE_1: u32 = 0x01;
        }
    }
    #[doc = "PoterDuff alpha mode"]
    pub mod PD_COLOR_MODE {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Straight mode for Porter Duff color"]
            pub const PD_COLOR_MODE_0: u32 = 0;
            #[doc = "Inversed mode for Porter Duff color"]
            pub const PD_COLOR_MODE_1: u32 = 0x01;
        }
    }
    #[doc = "The YUV422 input format selection"]
    pub mod YUV_FORMAT {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The YVYU422 8bit sequence is U1,Y1,V1,Y2"]
            pub const YUV_FORMAT_0: u32 = 0;
            #[doc = "The YVYU422 8bit sequence is V1,Y1,U1,Y2"]
            pub const YUV_FORMAT_1: u32 = 0x01;
            #[doc = "The YVYU422 8bit sequence is Y1,U1,Y2,V1"]
            pub const YUV_FORMAT_2: u32 = 0x02;
            #[doc = "The YVYU422 8bit sequence is Y1,V1,Y2,U1"]
            pub const YUV_FORMAT_3: u32 = 0x03;
        }
    }
    #[doc = "Global Alpha"]
    pub mod GLOBAL_ALPHA {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Layer encoding format (bit per pixel)"]
    pub mod BPP {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "1 bpp"]
            pub const BPP_0: u32 = 0;
            #[doc = "2 bpp"]
            pub const BPP_1: u32 = 0x01;
            #[doc = "4 bpp"]
            pub const BPP_2: u32 = 0x02;
            #[doc = "8 bpp"]
            pub const BPP_3: u32 = 0x03;
            #[doc = "16 bpp (RGB565)"]
            pub const BPP_4: u32 = 0x04;
            #[doc = "16 bpp (ARGB1555)"]
            pub const BPP_5: u32 = 0x05;
            #[doc = "16 bpp (ARGB4444)"]
            pub const BPP_6: u32 = 0x06;
            #[doc = "YCbCr422 (Only layer 0/1 can support this format)"]
            pub const BPP_7: u32 = 0x07;
            #[doc = "24 bpp (RGB888)"]
            pub const BPP_8: u32 = 0x08;
            #[doc = "32 bpp (ARGB8888)"]
            pub const BPP_9: u32 = 0x09;
            #[doc = "32 bpp (ABGR8888)"]
            pub const BPP_10: u32 = 0x0a;
        }
    }
    #[doc = "Safety Mode Enable Bit"]
    pub mod SAFETY_EN {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Safety Mode is disabled"]
            pub const SAFETY_EN_0: u32 = 0;
            #[doc = "Safety Mode is enabled for this layer"]
            pub const SAFETY_EN_1: u32 = 0x01;
        }
    }
    #[doc = "Shadow Load Enable"]
    pub mod SHADOW_LOAD_EN {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enable the layer for DMA"]
    pub mod EN {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "OFF"]
            pub const EN_0: u32 = 0;
            #[doc = "ON"]
            pub const EN_1: u32 = 0x01;
        }
    }
}
#[doc = "Control Descriptor Layer 6 Register"]
pub mod CTRLDESCL5_6 {
    #[doc = "Background B component value"]
    pub mod BCLR_B {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Background G component value"]
    pub mod BCLR_G {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Background R component value"]
    pub mod BCLR_R {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Control Descriptor Layer 1 Register"]
pub mod CTRLDESCL6_1 {
    #[doc = "Width of the layer in pixels"]
    pub mod WIDTH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Height of the layer in pixels"]
    pub mod HEIGHT {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Control Descriptor Layer 2 Register"]
pub mod CTRLDESCL6_2 {
    #[doc = "The horizontal position of left-hand column of the layer, where 0 is the left-hand column of the panel, only positive values are to the right the left-hand column of the panel"]
    pub mod POSX {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The vertical position of top row of the layer, where 0 is the top row of the panel, only positive values are below the top row of the panel"]
    pub mod POSY {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Control Descriptor Layer 3 Register"]
pub mod CTRLDESCL6_3 {
    #[doc = "Number of bytes between 2 vertically adjacent pixels in system memory. Byte granularity is supported, but SW should align to 64B boundry"]
    pub mod PITCH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Control Descriptor Layer 4 Register"]
pub mod CTRLDESCL6_4 {
    #[doc = "Address of layer data in the memory. The address programmed should be 64-bit aligned"]
    pub mod ADDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Control Descriptor Layer 5 Register"]
pub mod CTRLDESCL6_5 {
    #[doc = "Alpha Blending Mode"]
    pub mod AB_MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No alpha Blending (The SAFETY_EN bit need set to 1)"]
            pub const AB_MODE_0: u32 = 0;
            #[doc = "Blend with global ALPHA"]
            pub const AB_MODE_1: u32 = 0x01;
            #[doc = "Blend with embedded ALPHA"]
            pub const AB_MODE_2: u32 = 0x02;
            #[doc = "Blend with PoterDuff enable"]
            pub const AB_MODE_3: u32 = 0x03;
        }
    }
    #[doc = "PoterDuff factor mode"]
    pub mod PD_FACTOR_MODE {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Using 1"]
            pub const PD_FACTOR_MODE_0: u32 = 0;
            #[doc = "Using 0"]
            pub const PD_FACTOR_MODE_1: u32 = 0x01;
            #[doc = "Using straight alpha"]
            pub const PD_FACTOR_MODE_2: u32 = 0x02;
            #[doc = "Using inverse alpha"]
            pub const PD_FACTOR_MODE_3: u32 = 0x03;
        }
    }
    #[doc = "PoterDuff global alpha mode"]
    pub mod PD_GLOBAL_ALPHA_MODE {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Using global alpha"]
            pub const PD_GLOBAL_ALPHA_MODE_0: u32 = 0;
            #[doc = "Using local alpha"]
            pub const PD_GLOBAL_ALPHA_MODE_1: u32 = 0x01;
            #[doc = "Using scaled alpha"]
            pub const PD_GLOBAL_ALPHA_MODE_2: u32 = 0x02;
            #[doc = "Using scaled alpha"]
            pub const PD_GLOBAL_ALPHA_MODE_3: u32 = 0x03;
        }
    }
    #[doc = "PoterDuff alpha mode"]
    pub mod PD_ALPHA_MODE {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Straight mode for Porter Duff alpha"]
            pub const PD_ALPHA_MODE_0: u32 = 0;
            #[doc = "Inversed mode for Porter Duff alpha"]
            pub const PD_ALPHA_MODE_1: u32 = 0x01;
        }
    }
    #[doc = "PoterDuff alpha mode"]
    pub mod PD_COLOR_MODE {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Straight mode for Porter Duff color"]
            pub const PD_COLOR_MODE_0: u32 = 0;
            #[doc = "Inversed mode for Porter Duff color"]
            pub const PD_COLOR_MODE_1: u32 = 0x01;
        }
    }
    #[doc = "The YUV422 input format selection"]
    pub mod YUV_FORMAT {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The YVYU422 8bit sequence is U1,Y1,V1,Y2"]
            pub const YUV_FORMAT_0: u32 = 0;
            #[doc = "The YVYU422 8bit sequence is V1,Y1,U1,Y2"]
            pub const YUV_FORMAT_1: u32 = 0x01;
            #[doc = "The YVYU422 8bit sequence is Y1,U1,Y2,V1"]
            pub const YUV_FORMAT_2: u32 = 0x02;
            #[doc = "The YVYU422 8bit sequence is Y1,V1,Y2,U1"]
            pub const YUV_FORMAT_3: u32 = 0x03;
        }
    }
    #[doc = "Global Alpha"]
    pub mod GLOBAL_ALPHA {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Layer encoding format (bit per pixel)"]
    pub mod BPP {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "1 bpp"]
            pub const BPP_0: u32 = 0;
            #[doc = "2 bpp"]
            pub const BPP_1: u32 = 0x01;
            #[doc = "4 bpp"]
            pub const BPP_2: u32 = 0x02;
            #[doc = "8 bpp"]
            pub const BPP_3: u32 = 0x03;
            #[doc = "16 bpp (RGB565)"]
            pub const BPP_4: u32 = 0x04;
            #[doc = "16 bpp (ARGB1555)"]
            pub const BPP_5: u32 = 0x05;
            #[doc = "16 bpp (ARGB4444)"]
            pub const BPP_6: u32 = 0x06;
            #[doc = "YCbCr422 (Only layer 0/1 can support this format)"]
            pub const BPP_7: u32 = 0x07;
            #[doc = "24 bpp (RGB888)"]
            pub const BPP_8: u32 = 0x08;
            #[doc = "32 bpp (ARGB8888)"]
            pub const BPP_9: u32 = 0x09;
            #[doc = "32 bpp (ABGR8888)"]
            pub const BPP_10: u32 = 0x0a;
        }
    }
    #[doc = "Safety Mode Enable Bit"]
    pub mod SAFETY_EN {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Safety Mode is disabled"]
            pub const SAFETY_EN_0: u32 = 0;
            #[doc = "Safety Mode is enabled for this layer"]
            pub const SAFETY_EN_1: u32 = 0x01;
        }
    }
    #[doc = "Shadow Load Enable"]
    pub mod SHADOW_LOAD_EN {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enable the layer for DMA"]
    pub mod EN {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "OFF"]
            pub const EN_0: u32 = 0;
            #[doc = "ON"]
            pub const EN_1: u32 = 0x01;
        }
    }
}
#[doc = "Control Descriptor Layer 6 Register"]
pub mod CTRLDESCL6_6 {
    #[doc = "Background B component value"]
    pub mod BCLR_B {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Background G component value"]
    pub mod BCLR_G {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Background R component value"]
    pub mod BCLR_R {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Control Descriptor Layer 1 Register"]
pub mod CTRLDESCL7_1 {
    #[doc = "Width of the layer in pixels"]
    pub mod WIDTH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Height of the layer in pixels"]
    pub mod HEIGHT {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Control Descriptor Layer 2 Register"]
pub mod CTRLDESCL7_2 {
    #[doc = "The horizontal position of left-hand column of the layer, where 0 is the left-hand column of the panel, only positive values are to the right the left-hand column of the panel"]
    pub mod POSX {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The vertical position of top row of the layer, where 0 is the top row of the panel, only positive values are below the top row of the panel"]
    pub mod POSY {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Control Descriptor Layer 3 Register"]
pub mod CTRLDESCL7_3 {
    #[doc = "Number of bytes between 2 vertically adjacent pixels in system memory. Byte granularity is supported, but SW should align to 64B boundry"]
    pub mod PITCH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Control Descriptor Layer 4 Register"]
pub mod CTRLDESCL7_4 {
    #[doc = "Address of layer data in the memory. The address programmed should be 64-bit aligned"]
    pub mod ADDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Control Descriptor Layer 5 Register"]
pub mod CTRLDESCL7_5 {
    #[doc = "Alpha Blending Mode"]
    pub mod AB_MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No alpha Blending (The SAFETY_EN bit need set to 1)"]
            pub const AB_MODE_0: u32 = 0;
            #[doc = "Blend with global ALPHA"]
            pub const AB_MODE_1: u32 = 0x01;
            #[doc = "Blend with embedded ALPHA"]
            pub const AB_MODE_2: u32 = 0x02;
            #[doc = "Blend with PoterDuff enable"]
            pub const AB_MODE_3: u32 = 0x03;
        }
    }
    #[doc = "PoterDuff factor mode"]
    pub mod PD_FACTOR_MODE {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Using 1"]
            pub const PD_FACTOR_MODE_0: u32 = 0;
            #[doc = "Using 0"]
            pub const PD_FACTOR_MODE_1: u32 = 0x01;
            #[doc = "Using straight alpha"]
            pub const PD_FACTOR_MODE_2: u32 = 0x02;
            #[doc = "Using inverse alpha"]
            pub const PD_FACTOR_MODE_3: u32 = 0x03;
        }
    }
    #[doc = "PoterDuff global alpha mode"]
    pub mod PD_GLOBAL_ALPHA_MODE {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Using global alpha"]
            pub const PD_GLOBAL_ALPHA_MODE_0: u32 = 0;
            #[doc = "Using local alpha"]
            pub const PD_GLOBAL_ALPHA_MODE_1: u32 = 0x01;
            #[doc = "Using scaled alpha"]
            pub const PD_GLOBAL_ALPHA_MODE_2: u32 = 0x02;
            #[doc = "Using scaled alpha"]
            pub const PD_GLOBAL_ALPHA_MODE_3: u32 = 0x03;
        }
    }
    #[doc = "PoterDuff alpha mode"]
    pub mod PD_ALPHA_MODE {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Straight mode for Porter Duff alpha"]
            pub const PD_ALPHA_MODE_0: u32 = 0;
            #[doc = "Inversed mode for Porter Duff alpha"]
            pub const PD_ALPHA_MODE_1: u32 = 0x01;
        }
    }
    #[doc = "PoterDuff alpha mode"]
    pub mod PD_COLOR_MODE {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Straight mode for Porter Duff color"]
            pub const PD_COLOR_MODE_0: u32 = 0;
            #[doc = "Inversed mode for Porter Duff color"]
            pub const PD_COLOR_MODE_1: u32 = 0x01;
        }
    }
    #[doc = "The YUV422 input format selection"]
    pub mod YUV_FORMAT {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The YVYU422 8bit sequence is U1,Y1,V1,Y2"]
            pub const YUV_FORMAT_0: u32 = 0;
            #[doc = "The YVYU422 8bit sequence is V1,Y1,U1,Y2"]
            pub const YUV_FORMAT_1: u32 = 0x01;
            #[doc = "The YVYU422 8bit sequence is Y1,U1,Y2,V1"]
            pub const YUV_FORMAT_2: u32 = 0x02;
            #[doc = "The YVYU422 8bit sequence is Y1,V1,Y2,U1"]
            pub const YUV_FORMAT_3: u32 = 0x03;
        }
    }
    #[doc = "Global Alpha"]
    pub mod GLOBAL_ALPHA {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Layer encoding format (bit per pixel)"]
    pub mod BPP {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "1 bpp"]
            pub const BPP_0: u32 = 0;
            #[doc = "2 bpp"]
            pub const BPP_1: u32 = 0x01;
            #[doc = "4 bpp"]
            pub const BPP_2: u32 = 0x02;
            #[doc = "8 bpp"]
            pub const BPP_3: u32 = 0x03;
            #[doc = "16 bpp (RGB565)"]
            pub const BPP_4: u32 = 0x04;
            #[doc = "16 bpp (ARGB1555)"]
            pub const BPP_5: u32 = 0x05;
            #[doc = "16 bpp (ARGB4444)"]
            pub const BPP_6: u32 = 0x06;
            #[doc = "YCbCr422 (Only layer 0/1 can support this format)"]
            pub const BPP_7: u32 = 0x07;
            #[doc = "24 bpp (RGB888)"]
            pub const BPP_8: u32 = 0x08;
            #[doc = "32 bpp (ARGB8888)"]
            pub const BPP_9: u32 = 0x09;
            #[doc = "32 bpp (ABGR8888)"]
            pub const BPP_10: u32 = 0x0a;
        }
    }
    #[doc = "Safety Mode Enable Bit"]
    pub mod SAFETY_EN {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Safety Mode is disabled"]
            pub const SAFETY_EN_0: u32 = 0;
            #[doc = "Safety Mode is enabled for this layer"]
            pub const SAFETY_EN_1: u32 = 0x01;
        }
    }
    #[doc = "Shadow Load Enable"]
    pub mod SHADOW_LOAD_EN {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enable the layer for DMA"]
    pub mod EN {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "OFF"]
            pub const EN_0: u32 = 0;
            #[doc = "ON"]
            pub const EN_1: u32 = 0x01;
        }
    }
}
#[doc = "Control Descriptor Layer 6 Register"]
pub mod CTRLDESCL7_6 {
    #[doc = "Background B component value"]
    pub mod BCLR_B {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Background G component value"]
    pub mod BCLR_G {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Background R component value"]
    pub mod BCLR_R {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "LCDIFv2 CLUT load Register"]
pub mod CLUT_LOAD {
    #[doc = "CLUT Update Enable"]
    pub mod CLUT_UPDATE_EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Selected CLUT Number"]
    pub mod SEL_CLUT_NUM {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
