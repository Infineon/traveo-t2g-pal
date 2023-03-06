#[doc = r"Register block"]
#[repr(C)]
pub struct MU {
    #[doc = "0x00 - Timer control"]
    pub tmr_ctl: TMR_CTL,
    #[doc = "0x04 - Timer status"]
    pub tmr_status: TMR_STATUS,
    #[doc = "0x08 - AXI port select"]
    pub port_select: PORT_SELECT,
    _reserved3: [u8; 0x04],
    #[doc = "0x10 - Transaction filter"]
    pub filter: FILTER,
    #[doc = "0x14 - Transaction filter mask"]
    pub filter_mask: FILTER_MASK,
    #[doc = "0x18 - Accumulated outstanding transactions"]
    pub ot_ac: OT_AC,
    _reserved6: [u8; 0x04],
    #[doc = "0x20 - Address transfer counter"]
    pub addr_cnt: ADDR_CNT,
    #[doc = "0x24 - Address stall counter"]
    pub addr_stall_cnt: ADDR_STALL_CNT,
    #[doc = "0x28 - Data transfer counter"]
    pub data_cnt: DATA_CNT,
    #[doc = "0x2c - Data stall counter"]
    pub data_stall_cnt: DATA_STALL_CNT,
}
#[doc = "TMR_CTL (rw) register accessor: an alias for `Reg<TMR_CTL_SPEC>`"]
pub type TMR_CTL = crate::Reg<tmr_ctl::TMR_CTL_SPEC>;
#[doc = "Timer control"]
pub mod tmr_ctl;
#[doc = "TMR_STATUS (r) register accessor: an alias for `Reg<TMR_STATUS_SPEC>`"]
pub type TMR_STATUS = crate::Reg<tmr_status::TMR_STATUS_SPEC>;
#[doc = "Timer status"]
pub mod tmr_status;
#[doc = "PORT_SELECT (rw) register accessor: an alias for `Reg<PORT_SELECT_SPEC>`"]
pub type PORT_SELECT = crate::Reg<port_select::PORT_SELECT_SPEC>;
#[doc = "AXI port select"]
pub mod port_select;
#[doc = "FILTER (rw) register accessor: an alias for `Reg<FILTER_SPEC>`"]
pub type FILTER = crate::Reg<filter::FILTER_SPEC>;
#[doc = "Transaction filter"]
pub mod filter;
#[doc = "FILTER_MASK (rw) register accessor: an alias for `Reg<FILTER_MASK_SPEC>`"]
pub type FILTER_MASK = crate::Reg<filter_mask::FILTER_MASK_SPEC>;
#[doc = "Transaction filter mask"]
pub mod filter_mask;
#[doc = "OT_AC (r) register accessor: an alias for `Reg<OT_AC_SPEC>`"]
pub type OT_AC = crate::Reg<ot_ac::OT_AC_SPEC>;
#[doc = "Accumulated outstanding transactions"]
pub mod ot_ac;
#[doc = "ADDR_CNT (r) register accessor: an alias for `Reg<ADDR_CNT_SPEC>`"]
pub type ADDR_CNT = crate::Reg<addr_cnt::ADDR_CNT_SPEC>;
#[doc = "Address transfer counter"]
pub mod addr_cnt;
#[doc = "ADDR_STALL_CNT (r) register accessor: an alias for `Reg<ADDR_STALL_CNT_SPEC>`"]
pub type ADDR_STALL_CNT = crate::Reg<addr_stall_cnt::ADDR_STALL_CNT_SPEC>;
#[doc = "Address stall counter"]
pub mod addr_stall_cnt;
#[doc = "DATA_CNT (r) register accessor: an alias for `Reg<DATA_CNT_SPEC>`"]
pub type DATA_CNT = crate::Reg<data_cnt::DATA_CNT_SPEC>;
#[doc = "Data transfer counter"]
pub mod data_cnt;
#[doc = "DATA_STALL_CNT (r) register accessor: an alias for `Reg<DATA_STALL_CNT_SPEC>`"]
pub type DATA_STALL_CNT = crate::Reg<data_stall_cnt::DATA_STALL_CNT_SPEC>;
#[doc = "Data stall counter"]
pub mod data_stall_cnt;
