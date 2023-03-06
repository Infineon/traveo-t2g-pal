#[doc = r"Register block"]
#[repr(C)]
pub struct TDM_TX_STRUCT {
    #[doc = "0x00 - TX control"]
    pub tx_ctl: TX_CTL,
    _reserved1: [u8; 0x0c],
    #[doc = "0x10 - TX interface control"]
    pub tx_if_ctl: TX_IF_CTL,
    #[doc = "0x14 - TX channel control"]
    pub tx_ch_ctl: TX_CH_CTL,
    _reserved3: [u8; 0x08],
    #[doc = "0x20 - TX test control"]
    pub tx_test_ctl: TX_TEST_CTL,
    _reserved4: [u8; 0x5c],
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
#[doc = "TX_CTL (rw) register accessor: an alias for `Reg<TX_CTL_SPEC>`"]
pub type TX_CTL = crate::Reg<tx_ctl::TX_CTL_SPEC>;
#[doc = "TX control"]
pub mod tx_ctl;
#[doc = "TX_IF_CTL (rw) register accessor: an alias for `Reg<TX_IF_CTL_SPEC>`"]
pub type TX_IF_CTL = crate::Reg<tx_if_ctl::TX_IF_CTL_SPEC>;
#[doc = "TX interface control"]
pub mod tx_if_ctl;
#[doc = "TX_CH_CTL (rw) register accessor: an alias for `Reg<TX_CH_CTL_SPEC>`"]
pub type TX_CH_CTL = crate::Reg<tx_ch_ctl::TX_CH_CTL_SPEC>;
#[doc = "TX channel control"]
pub mod tx_ch_ctl;
#[doc = "TX_TEST_CTL (rw) register accessor: an alias for `Reg<TX_TEST_CTL_SPEC>`"]
pub type TX_TEST_CTL = crate::Reg<tx_test_ctl::TX_TEST_CTL_SPEC>;
#[doc = "TX test control"]
pub mod tx_test_ctl;
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
