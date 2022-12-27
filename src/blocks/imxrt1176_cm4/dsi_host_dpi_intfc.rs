#[doc = "DSI Host DPI Interface"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "PEXEL_PAYLOAD_SIZE"]
    pub PIXEL_PAYLOAD_SIZE: crate::RWRegister<u32>,
    #[doc = "PIXEL_FIFO_SEND_LEVEL"]
    pub PIXEL_FIFO_SEND_LEVEL: crate::RWRegister<u32>,
    #[doc = "INTERFACE_COLOR_CODING"]
    pub INTERFACE_COLOR_CODING: crate::RWRegister<u32>,
    #[doc = "PIXEL_FORMAT"]
    pub PIXEL_FORMAT: crate::RWRegister<u32>,
    #[doc = "VSYNC_POLARITY"]
    pub VSYNC_POLARITY: crate::RWRegister<u32>,
    #[doc = "HSYNC_POLARITY"]
    pub HSYNC_POLARITY: crate::RWRegister<u32>,
    #[doc = "VIDEO_MODE"]
    pub VIDEO_MODE: crate::RWRegister<u32>,
    #[doc = "HFP"]
    pub HFP: crate::RWRegister<u32>,
    #[doc = "HBP"]
    pub HBP: crate::RWRegister<u32>,
    #[doc = "HSA"]
    pub HSA: crate::RWRegister<u32>,
    #[doc = "ENABLE_MULT_PKTS"]
    pub ENABLE_MULT_PKTS: crate::RWRegister<u32>,
    #[doc = "VBP"]
    pub VBP: crate::RWRegister<u32>,
    #[doc = "VFP"]
    pub VFP: crate::RWRegister<u32>,
    #[doc = "BLLP_MODE"]
    pub BLLP_MODE: crate::RWRegister<u32>,
    #[doc = "USE_NULL_PKT_BLLP"]
    pub USE_NULL_PKT_BLLP: crate::RWRegister<u32>,
    #[doc = "VACTIVE"]
    pub VACTIVE: crate::RWRegister<u32>,
}
#[doc = "PEXEL_PAYLOAD_SIZE"]
pub mod PIXEL_PAYLOAD_SIZE {
    #[doc = "Maximum number of pixels that should be sent as one DSI packet. Recommended to be evenly divisible by the line size (in pixels)."]
    pub mod PAYLOAD_SIZE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "PIXEL_FIFO_SEND_LEVEL"]
pub mod PIXEL_FIFO_SEND_LEVEL {
    #[doc = "In order to optimize DSI utility, the DPI bridge buffers a certain number of DPI pixels before initiating a DSI packet. This configuration port controls the level at which the DPI Host bridge begins sending pixels."]
    pub mod FIFO_SEND_LEVEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "INTERFACE_COLOR_CODING"]
pub mod INTERFACE_COLOR_CODING {
    #[doc = "Sets the distribution of RGB bits within the 24-bit d bus, as specified by the DPI specification."]
    pub mod RGB_CONFIG {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "16-bit Configuration 1"]
            pub const RGB_CONFIG_0: u32 = 0;
            #[doc = "16-bit Configuration 2"]
            pub const RGB_CONFIG_1: u32 = 0x01;
            #[doc = "16-bit Configuration 3"]
            pub const RGB_CONFIG_2: u32 = 0x02;
            #[doc = "18-bit Configuration 1"]
            pub const RGB_CONFIG_3: u32 = 0x03;
            #[doc = "18-bit Configuration 2"]
            pub const RGB_CONFIG_4: u32 = 0x04;
            #[doc = "24-bit"]
            pub const RGB_CONFIG_5: u32 = 0x05;
        }
    }
}
#[doc = "PIXEL_FORMAT"]
pub mod PIXEL_FORMAT {
    #[doc = "Sets the DSI packet type of the pixels"]
    pub mod PIXEL_FORMAT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "16 bit"]
            pub const PIXEL_FORMAT_0: u32 = 0;
            #[doc = "18 bit"]
            pub const PIXEL_FORMAT_1: u32 = 0x01;
            #[doc = "18 bit loosely packed"]
            pub const PIXEL_FORMAT_2: u32 = 0x02;
            #[doc = "24 bit"]
            pub const PIXEL_FORMAT_3: u32 = 0x03;
        }
    }
}
#[doc = "VSYNC_POLARITY"]
pub mod VSYNC_POLARITY {
    #[doc = "Sets polarity of dpi_vsync_input"]
    pub mod VSYNC_POLARITY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "active low"]
            pub const VSYNC_POLARITY_0: u32 = 0;
            #[doc = "active high"]
            pub const VSYNC_POLARITY_1: u32 = 0x01;
        }
    }
}
#[doc = "HSYNC_POLARITY"]
pub mod HSYNC_POLARITY {
    #[doc = "Sets polarity of dpi_hsync_input"]
    pub mod HSYNC_POLARITY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "active low"]
            pub const HSYNC_POLARITY_0: u32 = 0;
            #[doc = "active high"]
            pub const HSYNC_POLARITY_1: u32 = 0x01;
        }
    }
}
#[doc = "VIDEO_MODE"]
pub mod VIDEO_MODE {
    #[doc = "Select DSI video mode that the host DPI module should generate packets for."]
    pub mod VIDEO_MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Non-Burst mode with Sync Pulses"]
            pub const VIDEO_MODE_0: u32 = 0;
            #[doc = "Non-Burst mode with Sync Events"]
            pub const VIDEO_MODE_1: u32 = 0x01;
            #[doc = "Burst mode"]
            pub const VIDEO_MODE_2: u32 = 0x02;
        }
    }
}
#[doc = "HFP"]
pub mod HFP {
    #[doc = "Sets the DSI packet payload size, in bytes, of the horizontal front porch blanking packet."]
    pub mod PAYLOAD_SIZE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "HBP"]
pub mod HBP {
    #[doc = "Sets the DSI packet payload size, in bytes, of the horizontal back porch blanking packet."]
    pub mod PAYLOAD_SIZE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "HSA"]
pub mod HSA {
    #[doc = "Sets the DSI packet payload size, in bytes, of the horizontal sync width filler blanking packet."]
    pub mod PAYLOAD_SIZE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ENABLE_MULT_PKTS"]
pub mod ENABLE_MULT_PKTS {
    #[doc = "Enable Multiple packets per video line. When enabled, PIXEL_PAYLOAD_SIZE\\[PAYLOAD_SIZE\\] must be set to exactly half the size of the video line"]
    pub mod ENABLE_MULT_PKTS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Video Line is sent in a single packet"]
            pub const ENABLE_MULT_PKTS_0: u32 = 0;
            #[doc = "Video Line is sent in two packets"]
            pub const ENABLE_MULT_PKTS_1: u32 = 0x01;
        }
    }
}
#[doc = "VBP"]
pub mod VBP {
    #[doc = "Sets the number of lines in the vertical back porch."]
    pub mod NUM_LINES {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "VFP"]
pub mod VFP {
    #[doc = "Sets the number of lines in the vertical front porch."]
    pub mod NUM_LINES {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "BLLP_MODE"]
pub mod BLLP_MODE {
    #[doc = "Optimize bllp periods to Low Power mode when possible"]
    pub mod LP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Blanking packets are sent during BLLP periods"]
            pub const LP_0: u32 = 0;
            #[doc = "LP mode is used for BLLP periods"]
            pub const LP_1: u32 = 0x01;
        }
    }
}
#[doc = "USE_NULL_PKT_BLLP"]
pub mod USE_NULL_PKT_BLLP {
    #[doc = "Selects type of blanking packet to be sent during bllp"]
    pub mod NULL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Blanking packet used in bllp region 1"]
            pub const NULL_0: u32 = 0;
            #[doc = "Null packet used in bllp region"]
            pub const NULL_1: u32 = 0x01;
        }
    }
}
#[doc = "VACTIVE"]
pub mod VACTIVE {
    #[doc = "Sets the number of lines in the vertical active aread."]
    pub mod NUM_LINES {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
