#[doc = r"Register block"]
#[repr(C)]
pub struct MIPICSI_CORE {
    #[doc = "0x00 - CFG_NUM_LANES is a register within the CSI-2 RX Controller Core."]
    pub cfg_num_lanes: CFG_NUM_LANES,
    #[doc = "0x04 - CFG_CLK_LANE_EN is a register within the CSI-2 RX Controller Core."]
    pub cfg_clk_lane_en: CFG_CLK_LANE_EN,
    #[doc = "0x08 - CFG_DATA_LANE_EN is a register within the CSI-2 RX Controller Core."]
    pub cfg_data_lane_en: CFG_DATA_LANE_EN,
    #[doc = "0x0c - CFG_FLUSH_COUNT is a register within the CSI-2 RX Controller Core."]
    pub cfg_flush_count: CFG_FLUSH_COUNT,
    #[doc = "0x10 - CFG_BIT_ERR is a register within the CSI-2 RX Controller Core."]
    pub cfg_bit_err: CFG_BIT_ERR,
    #[doc = "0x14 - IRQ_STATUS is a register within the CSI-2 RX Controller Core."]
    pub irq_status: IRQ_STATUS,
    #[doc = "0x18 - IRQ_ENABLE is a register within the CSI-2 RX Controller Core."]
    pub irq_enable: IRQ_ENABLE,
    #[doc = "0x1c - IRQ_CLR is a register within the CSI-2 RX Controller Core."]
    pub irq_clr: IRQ_CLR,
    #[doc = "0x20 - ULPS_CLK_STATUS is a register within the CSI-2 RX Controller Core."]
    pub ulps_clk_status: ULPS_CLK_STATUS,
    #[doc = "0x24 - ULPS_STATUS is a register within the CSI-2 RX Controller Core."]
    pub ulps_status: ULPS_STATUS,
    #[doc = "0x28 - ULPS_CLK_MARK_STATUS is a register within the CSI-2 RX Controller Core."]
    pub ulps_clk_mark_status: ULPS_CLK_MARK_STATUS,
    #[doc = "0x2c - ULPS_MARK_STATUS is a register within the CSI-2 RX Controller Core."]
    pub ulps_mark_status: ULPS_MARK_STATUS,
    _reserved12: [u8; 0x28],
    #[doc = "0x58 - CFG_DISABLE_PAYLOAD_0 is a register within the CSI-2 RX Controller Core."]
    pub cfg_disable_payload_0: CFG_DISABLE_PAYLOAD_0,
    #[doc = "0x5c - CFG_DISABLE_PAYLOAD_1 is a register within the CSI-2 RX Controller Core."]
    pub cfg_disable_payload_1: CFG_DISABLE_PAYLOAD_1,
}
#[doc = "CFG_NUM_LANES (rw) register accessor: an alias for `Reg<CFG_NUM_LANES_SPEC>`"]
pub type CFG_NUM_LANES = crate::Reg<cfg_num_lanes::CFG_NUM_LANES_SPEC>;
#[doc = "CFG_NUM_LANES is a register within the CSI-2 RX Controller Core."]
pub mod cfg_num_lanes;
#[doc = "CFG_CLK_LANE_EN (rw) register accessor: an alias for `Reg<CFG_CLK_LANE_EN_SPEC>`"]
pub type CFG_CLK_LANE_EN = crate::Reg<cfg_clk_lane_en::CFG_CLK_LANE_EN_SPEC>;
#[doc = "CFG_CLK_LANE_EN is a register within the CSI-2 RX Controller Core."]
pub mod cfg_clk_lane_en;
#[doc = "CFG_DATA_LANE_EN (rw) register accessor: an alias for `Reg<CFG_DATA_LANE_EN_SPEC>`"]
pub type CFG_DATA_LANE_EN = crate::Reg<cfg_data_lane_en::CFG_DATA_LANE_EN_SPEC>;
#[doc = "CFG_DATA_LANE_EN is a register within the CSI-2 RX Controller Core."]
pub mod cfg_data_lane_en;
#[doc = "CFG_FLUSH_COUNT (rw) register accessor: an alias for `Reg<CFG_FLUSH_COUNT_SPEC>`"]
pub type CFG_FLUSH_COUNT = crate::Reg<cfg_flush_count::CFG_FLUSH_COUNT_SPEC>;
#[doc = "CFG_FLUSH_COUNT is a register within the CSI-2 RX Controller Core."]
pub mod cfg_flush_count;
#[doc = "CFG_BIT_ERR (r) register accessor: an alias for `Reg<CFG_BIT_ERR_SPEC>`"]
pub type CFG_BIT_ERR = crate::Reg<cfg_bit_err::CFG_BIT_ERR_SPEC>;
#[doc = "CFG_BIT_ERR is a register within the CSI-2 RX Controller Core."]
pub mod cfg_bit_err;
#[doc = "IRQ_STATUS (r) register accessor: an alias for `Reg<IRQ_STATUS_SPEC>`"]
pub type IRQ_STATUS = crate::Reg<irq_status::IRQ_STATUS_SPEC>;
#[doc = "IRQ_STATUS is a register within the CSI-2 RX Controller Core."]
pub mod irq_status;
#[doc = "IRQ_ENABLE (rw) register accessor: an alias for `Reg<IRQ_ENABLE_SPEC>`"]
pub type IRQ_ENABLE = crate::Reg<irq_enable::IRQ_ENABLE_SPEC>;
#[doc = "IRQ_ENABLE is a register within the CSI-2 RX Controller Core."]
pub mod irq_enable;
#[doc = "IRQ_CLR (rw) register accessor: an alias for `Reg<IRQ_CLR_SPEC>`"]
pub type IRQ_CLR = crate::Reg<irq_clr::IRQ_CLR_SPEC>;
#[doc = "IRQ_CLR is a register within the CSI-2 RX Controller Core."]
pub mod irq_clr;
#[doc = "ULPS_CLK_STATUS (r) register accessor: an alias for `Reg<ULPS_CLK_STATUS_SPEC>`"]
pub type ULPS_CLK_STATUS = crate::Reg<ulps_clk_status::ULPS_CLK_STATUS_SPEC>;
#[doc = "ULPS_CLK_STATUS is a register within the CSI-2 RX Controller Core."]
pub mod ulps_clk_status;
#[doc = "ULPS_STATUS (r) register accessor: an alias for `Reg<ULPS_STATUS_SPEC>`"]
pub type ULPS_STATUS = crate::Reg<ulps_status::ULPS_STATUS_SPEC>;
#[doc = "ULPS_STATUS is a register within the CSI-2 RX Controller Core."]
pub mod ulps_status;
#[doc = "ULPS_CLK_MARK_STATUS (r) register accessor: an alias for `Reg<ULPS_CLK_MARK_STATUS_SPEC>`"]
pub type ULPS_CLK_MARK_STATUS = crate::Reg<ulps_clk_mark_status::ULPS_CLK_MARK_STATUS_SPEC>;
#[doc = "ULPS_CLK_MARK_STATUS is a register within the CSI-2 RX Controller Core."]
pub mod ulps_clk_mark_status;
#[doc = "ULPS_MARK_STATUS (r) register accessor: an alias for `Reg<ULPS_MARK_STATUS_SPEC>`"]
pub type ULPS_MARK_STATUS = crate::Reg<ulps_mark_status::ULPS_MARK_STATUS_SPEC>;
#[doc = "ULPS_MARK_STATUS is a register within the CSI-2 RX Controller Core."]
pub mod ulps_mark_status;
#[doc = "CFG_DISABLE_PAYLOAD_0 (rw) register accessor: an alias for `Reg<CFG_DISABLE_PAYLOAD_0_SPEC>`"]
pub type CFG_DISABLE_PAYLOAD_0 = crate::Reg<cfg_disable_payload_0::CFG_DISABLE_PAYLOAD_0_SPEC>;
#[doc = "CFG_DISABLE_PAYLOAD_0 is a register within the CSI-2 RX Controller Core."]
pub mod cfg_disable_payload_0;
#[doc = "CFG_DISABLE_PAYLOAD_1 (rw) register accessor: an alias for `Reg<CFG_DISABLE_PAYLOAD_1_SPEC>`"]
pub type CFG_DISABLE_PAYLOAD_1 = crate::Reg<cfg_disable_payload_1::CFG_DISABLE_PAYLOAD_1_SPEC>;
#[doc = "CFG_DISABLE_PAYLOAD_1 is a register within the CSI-2 RX Controller Core."]
pub mod cfg_disable_payload_1;
