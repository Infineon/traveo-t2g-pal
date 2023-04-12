#[doc = r"Register block"]
#[repr(C)]
pub struct CH {
    #[doc = "0x00 - Control 0"]
    pub ctl0: CTL0,
    #[doc = "0x04 - Control 1"]
    pub ctl1: CTL1,
    #[doc = "0x08 - Status"]
    pub status: STATUS,
    _reserved3: [u8; 0x04],
    #[doc = "0x10 - Command"]
    pub cmd: CMD,
    _reserved4: [u8; 0x4c],
    #[doc = "0x60 - TX/RX status"]
    pub tx_rx_status: TX_RX_STATUS,
    _reserved5: [u8; 0x1c],
    #[doc = "0x80 - PID and checksum"]
    pub pid_checksum: PID_CHECKSUM,
    #[doc = "0x84 - Response data 0"]
    pub data0: DATA0,
    #[doc = "0x88 - Response data 1"]
    pub data1: DATA1,
    _reserved8: [u8; 0x34],
    #[doc = "0xc0 - Interrupt"]
    pub intr: INTR,
    #[doc = "0xc4 - Interrupt set"]
    pub intr_set: INTR_SET,
    #[doc = "0xc8 - Interrupt mask"]
    pub intr_mask: INTR_MASK,
    #[doc = "0xcc - Interrupt masked"]
    pub intr_masked: INTR_MASKED,
}
#[doc = "CTL0 (rw) register accessor: an alias for `Reg<CTL0_SPEC>`"]
pub type CTL0 = crate::Reg<ctl0::CTL0_SPEC>;
#[doc = "Control 0"]
pub mod ctl0;
#[doc = "CTL1 (rw) register accessor: an alias for `Reg<CTL1_SPEC>`"]
pub type CTL1 = crate::Reg<ctl1::CTL1_SPEC>;
#[doc = "Control 1"]
pub mod ctl1;
#[doc = "STATUS (r) register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Status"]
pub mod status;
#[doc = "CMD (rw) register accessor: an alias for `Reg<CMD_SPEC>`"]
pub type CMD = crate::Reg<cmd::CMD_SPEC>;
#[doc = "Command"]
pub mod cmd;
#[doc = "TX_RX_STATUS (rw) register accessor: an alias for `Reg<TX_RX_STATUS_SPEC>`"]
pub type TX_RX_STATUS = crate::Reg<tx_rx_status::TX_RX_STATUS_SPEC>;
#[doc = "TX/RX status"]
pub mod tx_rx_status;
#[doc = "PID_CHECKSUM (rw) register accessor: an alias for `Reg<PID_CHECKSUM_SPEC>`"]
pub type PID_CHECKSUM = crate::Reg<pid_checksum::PID_CHECKSUM_SPEC>;
#[doc = "PID and checksum"]
pub mod pid_checksum;
#[doc = "DATA0 (rw) register accessor: an alias for `Reg<DATA0_SPEC>`"]
pub type DATA0 = crate::Reg<data0::DATA0_SPEC>;
#[doc = "Response data 0"]
pub mod data0;
#[doc = "DATA1 (rw) register accessor: an alias for `Reg<DATA1_SPEC>`"]
pub type DATA1 = crate::Reg<data1::DATA1_SPEC>;
#[doc = "Response data 1"]
pub mod data1;
#[doc = "INTR (rw) register accessor: an alias for `Reg<INTR_SPEC>`"]
pub type INTR = crate::Reg<intr::INTR_SPEC>;
#[doc = "Interrupt"]
pub mod intr;
#[doc = "INTR_SET (rw) register accessor: an alias for `Reg<INTR_SET_SPEC>`"]
pub type INTR_SET = crate::Reg<intr_set::INTR_SET_SPEC>;
#[doc = "Interrupt set"]
pub mod intr_set;
#[doc = "INTR_MASK (rw) register accessor: an alias for `Reg<INTR_MASK_SPEC>`"]
pub type INTR_MASK = crate::Reg<intr_mask::INTR_MASK_SPEC>;
#[doc = "Interrupt mask"]
pub mod intr_mask;
#[doc = "INTR_MASKED (r) register accessor: an alias for `Reg<INTR_MASKED_SPEC>`"]
pub type INTR_MASKED = crate::Reg<intr_masked::INTR_MASKED_SPEC>;
#[doc = "Interrupt masked"]
pub mod intr_masked;
