#[doc = "DSI HOST APB PKT Interface"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "TX_PAYLOAD"]
    pub TX_PAYLOAD: crate::RWRegister<u32>,
    #[doc = "PKT_CONTROL"]
    pub PKT_CONTROL: crate::RWRegister<u32>,
    #[doc = "SEND_PACKET"]
    pub SEND_PACKET: crate::RWRegister<u32>,
    #[doc = "PKT_STATUS"]
    pub PKT_STATUS: crate::RORegister<u32>,
    #[doc = "PKT_FIFO_WR_LEVEL"]
    pub PKT_FIFO_WR_LEVEL: crate::RORegister<u32>,
    #[doc = "PKT_FIFO_RD_LEVEL"]
    pub PKT_FIFO_RD_LEVEL: crate::RORegister<u32>,
    #[doc = "PKT_RX_PAYLOAD"]
    pub PKT_RX_PAYLOAD: crate::RORegister<u32>,
    #[doc = "PKT_RX_PKT_HEADER"]
    pub PKT_RX_PKT_HEADER: crate::RORegister<u32>,
    #[doc = "IRQ_STATUS"]
    pub IRQ_STATUS: crate::RORegister<u32>,
    #[doc = "IRQ_STATUS2"]
    pub IRQ_STATUS2: crate::RORegister<u32>,
    #[doc = "IRQ_MASK"]
    pub IRQ_MASK: crate::RWRegister<u32>,
    #[doc = "IRQ_MASK2"]
    pub IRQ_MASK2: crate::RWRegister<u32>,
}
#[doc = "TX_PAYLOAD"]
pub mod TX_PAYLOAD {
    #[doc = "Tx Payload data write register. Write to this register loads the payload FIFO with 32 bit values."]
    pub mod PAYLOAD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "PKT_CONTROL"]
pub mod PKT_CONTROL {
    #[doc = "Tx packet control"]
    pub mod CTRL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SEND_PACKET"]
pub mod SEND_PACKET {
    #[doc = "Tx send packet, writing to this register causes the packet described in dsi_host_pkt_control to be sent."]
    pub mod TX_SEND {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Packet not sent"]
            pub const TX_SEND_0: u32 = 0;
            #[doc = "Packet is sent"]
            pub const TX_SEND_1: u32 = 0x01;
        }
    }
}
#[doc = "PKT_STATUS"]
pub mod PKT_STATUS {
    #[doc = "Status of APB to packet interface."]
    pub mod STATUS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "PKT_FIFO_WR_LEVEL"]
pub mod PKT_FIFO_WR_LEVEL {
    #[doc = "Write level of APB to pkt interface FIFO"]
    pub mod WR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "PKT_FIFO_RD_LEVEL"]
pub mod PKT_FIFO_RD_LEVEL {
    #[doc = "Read level of APB to pkt interface FIFO"]
    pub mod RD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "PKT_RX_PAYLOAD"]
pub mod PKT_RX_PAYLOAD {
    #[doc = "APB to pkt interface Rx payload read"]
    pub mod PAYLOAD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "PKT_RX_PKT_HEADER"]
pub mod PKT_RX_PKT_HEADER {
    #[doc = "APB to pkt interface Rx packet header"]
    pub mod HEADER {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "IRQ_STATUS"]
pub mod IRQ_STATUS {
    #[doc = "Status of APB to packet interface."]
    pub mod STATUS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "IRQ_STATUS2"]
pub mod IRQ_STATUS2 {
    #[doc = "Status of APB to packet interface part 2, read part 2 first then dsi_host_irq_status. Reading dsi_host_irq_status will clear both status and status2."]
    pub mod STATUS2 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "IRQ_MASK"]
pub mod IRQ_MASK {
    #[doc = "IRQ Mask"]
    pub mod MASK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "IRQ_MASK2"]
pub mod IRQ_MASK2 {
    #[doc = "IRQ mask 2"]
    pub mod MASK2 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
