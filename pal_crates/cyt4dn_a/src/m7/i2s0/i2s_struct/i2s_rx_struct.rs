#[doc = r"Register block"]
#[repr(C)]
pub struct I2S_RX_STRUCT {
    #[doc = "0x00 - RX control"]
    pub rx_ctl: RX_CTL,
    _reserved1: [u8; 0x0c],
    #[doc = "0x10 - RX interface control"]
    pub rx_if_ctl: RX_IF_CTL,
    _reserved2: [u8; 0x0c],
    #[doc = "0x20 - RX test control"]
    pub rx_test_ctl: RX_TEST_CTL,
    _reserved3: [u8; 0x5c],
    #[doc = "0x80 - RX FIFO control"]
    pub rx_fifo_ctl: RX_FIFO_CTL,
    #[doc = "0x84 - RX FIFO status"]
    pub rx_fifo_status: RX_FIFO_STATUS,
    #[doc = "0x88 - RX FIFO read"]
    pub rx_fifo_rd: RX_FIFO_RD,
    #[doc = "0x8c - RX FIFO silent read"]
    pub rx_fifo_rd_silent: RX_FIFO_RD_SILENT,
    _reserved7: [u8; 0x30],
    #[doc = "0xc0 - Interrupt"]
    pub intr_rx: INTR_RX,
    #[doc = "0xc4 - Interrupt set"]
    pub intr_rx_set: INTR_RX_SET,
    #[doc = "0xc8 - Interrupt mask"]
    pub intr_rx_mask: INTR_RX_MASK,
    #[doc = "0xcc - Interrupt masked"]
    pub intr_rx_masked: INTR_RX_MASKED,
}
#[doc = "RX_CTL (rw) register accessor: an alias for `Reg<RX_CTL_SPEC>`"]
pub type RX_CTL = crate::Reg<rx_ctl::RX_CTL_SPEC>;
#[doc = "RX control"]
pub mod rx_ctl;
#[doc = "RX_IF_CTL (rw) register accessor: an alias for `Reg<RX_IF_CTL_SPEC>`"]
pub type RX_IF_CTL = crate::Reg<rx_if_ctl::RX_IF_CTL_SPEC>;
#[doc = "RX interface control"]
pub mod rx_if_ctl;
#[doc = "RX_TEST_CTL (rw) register accessor: an alias for `Reg<RX_TEST_CTL_SPEC>`"]
pub type RX_TEST_CTL = crate::Reg<rx_test_ctl::RX_TEST_CTL_SPEC>;
#[doc = "RX test control"]
pub mod rx_test_ctl;
#[doc = "RX_FIFO_CTL (rw) register accessor: an alias for `Reg<RX_FIFO_CTL_SPEC>`"]
pub type RX_FIFO_CTL = crate::Reg<rx_fifo_ctl::RX_FIFO_CTL_SPEC>;
#[doc = "RX FIFO control"]
pub mod rx_fifo_ctl;
#[doc = "RX_FIFO_STATUS (r) register accessor: an alias for `Reg<RX_FIFO_STATUS_SPEC>`"]
pub type RX_FIFO_STATUS = crate::Reg<rx_fifo_status::RX_FIFO_STATUS_SPEC>;
#[doc = "RX FIFO status"]
pub mod rx_fifo_status;
#[doc = "RX_FIFO_RD (r) register accessor: an alias for `Reg<RX_FIFO_RD_SPEC>`"]
pub type RX_FIFO_RD = crate::Reg<rx_fifo_rd::RX_FIFO_RD_SPEC>;
#[doc = "RX FIFO read"]
pub mod rx_fifo_rd;
#[doc = "RX_FIFO_RD_SILENT (r) register accessor: an alias for `Reg<RX_FIFO_RD_SILENT_SPEC>`"]
pub type RX_FIFO_RD_SILENT = crate::Reg<rx_fifo_rd_silent::RX_FIFO_RD_SILENT_SPEC>;
#[doc = "RX FIFO silent read"]
pub mod rx_fifo_rd_silent;
#[doc = "INTR_RX (rw) register accessor: an alias for `Reg<INTR_RX_SPEC>`"]
pub type INTR_RX = crate::Reg<intr_rx::INTR_RX_SPEC>;
#[doc = "Interrupt"]
pub mod intr_rx;
#[doc = "INTR_RX_SET (rw) register accessor: an alias for `Reg<INTR_RX_SET_SPEC>`"]
pub type INTR_RX_SET = crate::Reg<intr_rx_set::INTR_RX_SET_SPEC>;
#[doc = "Interrupt set"]
pub mod intr_rx_set;
#[doc = "INTR_RX_MASK (rw) register accessor: an alias for `Reg<INTR_RX_MASK_SPEC>`"]
pub type INTR_RX_MASK = crate::Reg<intr_rx_mask::INTR_RX_MASK_SPEC>;
#[doc = "Interrupt mask"]
pub mod intr_rx_mask;
#[doc = "INTR_RX_MASKED (r) register accessor: an alias for `Reg<INTR_RX_MASKED_SPEC>`"]
pub type INTR_RX_MASKED = crate::Reg<intr_rx_masked::INTR_RX_MASKED_SPEC>;
#[doc = "Interrupt masked"]
pub mod intr_rx_masked;
