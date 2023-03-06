#[doc = r"Register block"]
#[repr(C)]
pub struct MIXER_TX_STRUCT {
    #[doc = "0x00 - TX control"]
    pub tx_ctl: TX_CTL,
    _reserved1: [u8; 0x0c],
    #[doc = "0x10 - TX interface control"]
    pub tx_if_ctl: TX_IF_CTL,
    _reserved2: [u8; 0x6c],
    #[doc = "0x80 - TX FIFO control"]
    pub tx_fifo_ctl: TX_FIFO_CTL,
    _reserved3: [u8; 0x3c],
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
#[doc = "TX_FIFO_CTL (rw) register accessor: an alias for `Reg<TX_FIFO_CTL_SPEC>`"]
pub type TX_FIFO_CTL = crate::Reg<tx_fifo_ctl::TX_FIFO_CTL_SPEC>;
#[doc = "TX FIFO control"]
pub mod tx_fifo_ctl;
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
