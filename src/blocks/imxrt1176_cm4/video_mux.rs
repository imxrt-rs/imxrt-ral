#[doc = "VIDEO_MUX"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "Video mux Control Register"]
    pub VID_MUX_CTRL: crate::RWRegister<u32>,
    #[doc = "Video mux Control Register"]
    pub VID_MUX_CTRL_SET: crate::RWRegister<u32>,
    #[doc = "Video mux Control Register"]
    pub VID_MUX_CTRL_CLR: crate::RWRegister<u32>,
    #[doc = "Video mux Control Register"]
    pub VID_MUX_CTRL_TOG: crate::RWRegister<u32>,
    _reserved0: [u8; 0x10],
    #[doc = "Pixel Link Master(PLM) Control Register"]
    pub PLM_CTRL: crate::RWRegister<u32>,
    #[doc = "Pixel Link Master(PLM) Control Register"]
    pub PLM_CTRL_SET: crate::RWRegister<u32>,
    #[doc = "Pixel Link Master(PLM) Control Register"]
    pub PLM_CTRL_CLR: crate::RWRegister<u32>,
    #[doc = "Pixel Link Master(PLM) Control Register"]
    pub PLM_CTRL_TOG: crate::RWRegister<u32>,
    #[doc = "YUV420 Control Register"]
    pub YUV420_CTRL: crate::RWRegister<u32>,
    #[doc = "YUV420 Control Register"]
    pub YUV420_CTRL_SET: crate::RWRegister<u32>,
    #[doc = "YUV420 Control Register"]
    pub YUV420_CTRL_CLR: crate::RWRegister<u32>,
    #[doc = "YUV420 Control Register"]
    pub YUV420_CTRL_TOG: crate::RWRegister<u32>,
    _reserved1: [u8; 0x10],
    #[doc = "Data Disable Register"]
    pub CFG_DT_DISABLE: crate::RWRegister<u32>,
    #[doc = "Data Disable Register"]
    pub CFG_DT_DISABLE_SET: crate::RWRegister<u32>,
    #[doc = "Data Disable Register"]
    pub CFG_DT_DISABLE_CLR: crate::RWRegister<u32>,
    #[doc = "Data Disable Register"]
    pub CFG_DT_DISABLE_TOG: crate::RWRegister<u32>,
    _reserved2: [u8; 0x10],
    #[doc = "MIPI DSI Control Register"]
    pub MIPI_DSI_CTRL: crate::RWRegister<u32>,
    #[doc = "MIPI DSI Control Register"]
    pub MIPI_DSI_CTRL_SET: crate::RWRegister<u32>,
    #[doc = "MIPI DSI Control Register"]
    pub MIPI_DSI_CTRL_CLR: crate::RWRegister<u32>,
    #[doc = "MIPI DSI Control Register"]
    pub MIPI_DSI_CTRL_TOG: crate::RWRegister<u32>,
}
#[doc = "Video mux Control Register"]
pub mod VID_MUX_CTRL {
    #[doc = "CSI sensor data input mux selector"]
    pub mod CSI_SEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "CSI sensor data is from Parallel CSI"]
            pub const PARALLEL_CSI: u32 = 0;
            #[doc = "CSI sensor data is from MIPI CSI"]
            pub const MIPI_CSI: u32 = 0x01;
        }
    }
    #[doc = "LCDIF2 sensor data input mux selector"]
    pub mod LCDIF2_SEL {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "LCDIFv2 sensor data is from Parallel CSI"]
            pub const PARALLEL_CSI: u32 = 0;
            #[doc = "LCDIFv2 sensor data is from MIPI CSI"]
            pub const MIPI_CSI: u32 = 0x01;
        }
    }
    #[doc = "MIPI DSI video data input mux selector"]
    pub mod MIPI_DSI_SEL {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MIPI DSI video data is from eLCDIF"]
            pub const PARALLEL_CSI: u32 = 0;
            #[doc = "MIPI DSI video data is from LCDIFv2"]
            pub const MIPI_CSI: u32 = 0x01;
        }
    }
    #[doc = "Parallel LCDIF video data input mux selector"]
    pub mod PARA_LCD_SEL {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Parallel LCDIF video data is from eLCDIF"]
            pub const PARALLEL_CSI: u32 = 0;
            #[doc = "Parallel LCDIF video data is from LCDIFv2"]
            pub const MIPI_CSI: u32 = 0x01;
        }
    }
}
#[doc = "Video mux Control Register"]
pub mod VID_MUX_CTRL_SET {
    #[doc = "CSI sensor data input mux selector"]
    pub mod CSI_SEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "LCDIF2 sensor data input mux selector"]
    pub mod LCDIF2_SEL {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "MIPI DSI video data input mux selector"]
    pub mod MIPI_DSI_SEL {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Parallel LCDIF video data input mux selector"]
    pub mod PARA_LCD_SEL {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Video mux Control Register"]
pub mod VID_MUX_CTRL_CLR {
    #[doc = "CSI sensor data input mux selector"]
    pub mod CSI_SEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "LCDIF2 sensor data input mux selector"]
    pub mod LCDIF2_SEL {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "MIPI DSI video data input mux selector"]
    pub mod MIPI_DSI_SEL {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Parallel LCDIF video data input mux selector"]
    pub mod PARA_LCD_SEL {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Video mux Control Register"]
pub mod VID_MUX_CTRL_TOG {
    #[doc = "CSI sensor data input mux selector"]
    pub mod CSI_SEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "LCDIF2 sensor data input mux selector"]
    pub mod LCDIF2_SEL {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "MIPI DSI video data input mux selector"]
    pub mod MIPI_DSI_SEL {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Parallel LCDIF video data input mux selector"]
    pub mod PARA_LCD_SEL {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Pixel Link Master(PLM) Control Register"]
pub mod PLM_CTRL {
    #[doc = "Enable the output of HYSNC and VSYNC"]
    pub mod ENABLE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No active HSYNC and VSYNC output"]
            pub const NO_ACTIVE: u32 = 0;
            #[doc = "Active HSYNC and VSYNC output"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "VSYNC override"]
    pub mod VSYNC_OVERRIDE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "VSYNC is not asserted"]
            pub const DEASSERT: u32 = 0;
            #[doc = "VSYNC is asserted"]
            pub const ASSERT: u32 = 0x01;
        }
    }
    #[doc = "HSYNC override"]
    pub mod HSYNC_OVERRIDE {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "HSYNC is not asserted"]
            pub const DEASSERT: u32 = 0;
            #[doc = "HSYNC is asserted"]
            pub const ASSERT: u32 = 0x01;
        }
    }
    #[doc = "Valid override"]
    pub mod VALID_OVERRIDE {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "HSYNC and VSYNC is asserted"]
            pub const ASSERT: u32 = 0;
            #[doc = "HSYNC and VSYNC is not asserted"]
            pub const DEASSERT: u32 = 0x01;
        }
    }
    #[doc = "Polarity of HYSNC/VSYNC"]
    pub mod POLARITY {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Keep the current polarity of HSYNC and VSYNC"]
            pub const KEEP: u32 = 0;
            #[doc = "Invert the polarity of HSYNC and VSYNC"]
            pub const INVERT: u32 = 0x01;
        }
    }
}
#[doc = "Pixel Link Master(PLM) Control Register"]
pub mod PLM_CTRL_SET {
    #[doc = "Enable the output of HYSNC and VSYNC"]
    pub mod ENABLE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "VSYNC override"]
    pub mod VSYNC_OVERRIDE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "HSYNC override"]
    pub mod HSYNC_OVERRIDE {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Valid override"]
    pub mod VALID_OVERRIDE {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Polarity of HYSNC/VSYNC"]
    pub mod POLARITY {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Pixel Link Master(PLM) Control Register"]
pub mod PLM_CTRL_CLR {
    #[doc = "Enable the output of HYSNC and VSYNC"]
    pub mod ENABLE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "VSYNC override"]
    pub mod VSYNC_OVERRIDE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "HSYNC override"]
    pub mod HSYNC_OVERRIDE {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Valid override"]
    pub mod VALID_OVERRIDE {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Polarity of HYSNC/VSYNC"]
    pub mod POLARITY {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Pixel Link Master(PLM) Control Register"]
pub mod PLM_CTRL_TOG {
    #[doc = "Enable the output of HYSNC and VSYNC"]
    pub mod ENABLE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "VSYNC override"]
    pub mod VSYNC_OVERRIDE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "HSYNC override"]
    pub mod HSYNC_OVERRIDE {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Valid override"]
    pub mod VALID_OVERRIDE {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Polarity of HYSNC/VSYNC"]
    pub mod POLARITY {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "YUV420 Control Register"]
pub mod YUV420_CTRL {
    #[doc = "Data type of First Line"]
    pub mod FST_LN_DATA_TYPE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Odd (default)"]
            pub const ODD: u32 = 0;
            #[doc = "Even"]
            pub const EVEN: u32 = 0x01;
        }
    }
}
#[doc = "YUV420 Control Register"]
pub mod YUV420_CTRL_SET {
    #[doc = "Data type of First Line"]
    pub mod FST_LN_DATA_TYPE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "YUV420 Control Register"]
pub mod YUV420_CTRL_CLR {
    #[doc = "Data type of First Line"]
    pub mod FST_LN_DATA_TYPE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "YUV420 Control Register"]
pub mod YUV420_CTRL_TOG {
    #[doc = "Data type of First Line"]
    pub mod FST_LN_DATA_TYPE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Data Disable Register"]
pub mod CFG_DT_DISABLE {
    #[doc = "Data Type Disable"]
    pub mod CFG_DT_DISABLE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Data Disable Register"]
pub mod CFG_DT_DISABLE_SET {
    #[doc = "Data Type Disable"]
    pub mod CFG_DT_DISABLE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Data Disable Register"]
pub mod CFG_DT_DISABLE_CLR {
    #[doc = "Data Type Disable"]
    pub mod CFG_DT_DISABLE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Data Disable Register"]
pub mod CFG_DT_DISABLE_TOG {
    #[doc = "Data Type Disable"]
    pub mod CFG_DT_DISABLE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "MIPI DSI Control Register"]
pub mod MIPI_DSI_CTRL {
    #[doc = "Shut Down - Control to shutdown display (type 4 only)"]
    pub mod DPI_SD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No effect"]
            pub const NO: u32 = 0;
            #[doc = "Send shutdown command"]
            pub const SENDCMD: u32 = 0x01;
        }
    }
    #[doc = "Color Mode control"]
    pub mod DPI_CM {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Normal Mode"]
            pub const NORMAL: u32 = 0;
            #[doc = "Low-color mode"]
            pub const LOWCLR: u32 = 0x01;
        }
    }
}
#[doc = "MIPI DSI Control Register"]
pub mod MIPI_DSI_CTRL_SET {
    #[doc = "Shut Down - Control to shutdown display (type 4 only)"]
    pub mod DPI_SD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Color Mode control"]
    pub mod DPI_CM {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "MIPI DSI Control Register"]
pub mod MIPI_DSI_CTRL_CLR {
    #[doc = "Shut Down - Control to shutdown display (type 4 only)"]
    pub mod DPI_SD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Color Mode control"]
    pub mod DPI_CM {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "MIPI DSI Control Register"]
pub mod MIPI_DSI_CTRL_TOG {
    #[doc = "Shut Down - Control to shutdown display (type 4 only)"]
    pub mod DPI_SD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Color Mode control"]
    pub mod DPI_CM {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
