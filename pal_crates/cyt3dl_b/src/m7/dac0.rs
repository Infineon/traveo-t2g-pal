#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control"]
    pub ctl: CTL,
    #[doc = "0x04 - Interface Control"]
    pub if_ctl: IF_CTL,
    #[doc = "0x08 - Count"]
    pub count: COUNT,
    _reserved3: [u8; 0x04],
    #[doc = "0x10 - Status"]
    pub status: STATUS,
    _reserved4: [u8; 0x6c],
    #[doc = "0x80 - TX FIFO control"]
    pub tx_fifo_ctl: TX_FIFO_CTL,
    #[doc = "0x84 - TX FIFO status"]
    pub tx_fifo_status: TX_FIFO_STATUS,
    #[doc = "0x88 - TX FIFO write"]
    pub tx_fifo_wr: TX_FIFO_WR,
    _reserved7: [u8; 0x34],
    #[doc = "0xc0 - Interrupt"]
    pub intr_tx: INTR_TX,
    #[doc = "0xc4 - Interrupt set"]
    pub intr_tx_set: INTR_TX_SET,
    #[doc = "0xc8 - Interrupt mask"]
    pub intr_tx_mask: INTR_TX_MASK,
    #[doc = "0xcc - Interrupt masked"]
    pub intr_tx_masked: INTR_TX_MASKED,
}
#[doc = "CTL (rw) register accessor: an alias for `Reg<CTL_SPEC>`"]
pub type CTL = crate::Reg<ctl::CTL_SPEC>;
#[doc = "Control"]
pub mod ctl;
#[doc = "IF_CTL (rw) register accessor: an alias for `Reg<IF_CTL_SPEC>`"]
pub type IF_CTL = crate::Reg<if_ctl::IF_CTL_SPEC>;
#[doc = "Interface Control"]
pub mod if_ctl;
#[doc = "COUNT (rw) register accessor: an alias for `Reg<COUNT_SPEC>`"]
pub type COUNT = crate::Reg<count::COUNT_SPEC>;
#[doc = "Count"]
pub mod count;
#[doc = "STATUS (r) register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Status"]
pub mod status;
#[doc = "TX_FIFO_CTL (rw) register accessor: an alias for `Reg<TX_FIFO_CTL_SPEC>`"]
pub type TX_FIFO_CTL = crate::Reg<tx_fifo_ctl::TX_FIFO_CTL_SPEC>;
#[doc = "TX FIFO control"]
pub mod tx_fifo_ctl;
#[doc = "TX_FIFO_STATUS (r) register accessor: an alias for `Reg<TX_FIFO_STATUS_SPEC>`"]
pub type TX_FIFO_STATUS = crate::Reg<tx_fifo_status::TX_FIFO_STATUS_SPEC>;
#[doc = "TX FIFO status"]
pub mod tx_fifo_status;
#[doc = "TX_FIFO_WR (w) register accessor: an alias for `Reg<TX_FIFO_WR_SPEC>`"]
pub type TX_FIFO_WR = crate::Reg<tx_fifo_wr::TX_FIFO_WR_SPEC>;
#[doc = "TX FIFO write"]
pub mod tx_fifo_wr;
#[doc = "INTR_TX (rw) register accessor: an alias for `Reg<INTR_TX_SPEC>`"]
pub type INTR_TX = crate::Reg<intr_tx::INTR_TX_SPEC>;
#[doc = "Interrupt"]
pub mod intr_tx;
#[doc = "INTR_TX_SET (rw) register accessor: an alias for `Reg<INTR_TX_SET_SPEC>`"]
pub type INTR_TX_SET = crate::Reg<intr_tx_set::INTR_TX_SET_SPEC>;
#[doc = "Interrupt set"]
pub mod intr_tx_set;
#[doc = "INTR_TX_MASK (rw) register accessor: an alias for `Reg<INTR_TX_MASK_SPEC>`"]
pub type INTR_TX_MASK = crate::Reg<intr_tx_mask::INTR_TX_MASK_SPEC>;
#[doc = "Interrupt mask"]
pub mod intr_tx_mask;
#[doc = "INTR_TX_MASKED (r) register accessor: an alias for `Reg<INTR_TX_MASKED_SPEC>`"]
pub type INTR_TX_MASKED = crate::Reg<intr_tx_masked::INTR_TX_MASKED_SPEC>;
#[doc = "Interrupt masked"]
pub mod intr_tx_masked;
