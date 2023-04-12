#[doc = r"Register block"]
#[repr(C)]
pub struct CH {
    #[doc = "0x00 - Control 0"]
    pub ctl0: CTL0,
    #[doc = "0x04 - Control 1"]
    pub ctl1: CTL1,
    #[doc = "0x08 - Control 2"]
    pub ctl2: CTL2,
    #[doc = "0x0c - Status"]
    pub status: STATUS,
    #[doc = "0x10 - Command"]
    pub cmd: CMD,
    _reserved5: [u8; 0x2c],
    #[doc = "0x40 - TX/RX status"]
    pub tx_rx_status: TX_RX_STATUS,
    _reserved6: [u8; 0x0c],
    #[doc = "0x50 - TXPID and Frame Information"]
    pub txpid_fi: TXPID_FI,
    #[doc = "0x54 - RXPID and Frame Information"]
    pub rxpid_fi: RXPID_FI,
    #[doc = "0x58 - CRC"]
    pub crc: CRC,
    _reserved9: [u8; 0x24],
    #[doc = "0x80 - TX FIFO control"]
    pub tx_fifo_ctl: TX_FIFO_CTL,
    #[doc = "0x84 - TX FIFO status"]
    pub tx_fifo_status: TX_FIFO_STATUS,
    #[doc = "0x88 - TX FIFO write"]
    pub tx_fifo_wr: TX_FIFO_WR,
    _reserved12: [u8; 0x14],
    #[doc = "0xa0 - RX FIFO control"]
    pub rx_fifo_ctl: RX_FIFO_CTL,
    #[doc = "0xa4 - RX FIFO status"]
    pub rx_fifo_status: RX_FIFO_STATUS,
    #[doc = "0xa8 - RX FIFO read"]
    pub rx_fifo_rd: RX_FIFO_RD,
    #[doc = "0xac - RX FIFO silent read"]
    pub rx_fifo_rd_silent: RX_FIFO_RD_SILENT,
    _reserved16: [u8; 0x10],
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
#[doc = "CTL2 (rw) register accessor: an alias for `Reg<CTL2_SPEC>`"]
pub type CTL2 = crate::Reg<ctl2::CTL2_SPEC>;
#[doc = "Control 2"]
pub mod ctl2;
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
#[doc = "TXPID_FI (rw) register accessor: an alias for `Reg<TXPID_FI_SPEC>`"]
pub type TXPID_FI = crate::Reg<txpid_fi::TXPID_FI_SPEC>;
#[doc = "TXPID and Frame Information"]
pub mod txpid_fi;
#[doc = "RXPID_FI (r) register accessor: an alias for `Reg<RXPID_FI_SPEC>`"]
pub type RXPID_FI = crate::Reg<rxpid_fi::RXPID_FI_SPEC>;
#[doc = "RXPID and Frame Information"]
pub mod rxpid_fi;
#[doc = "CRC (r) register accessor: an alias for `Reg<CRC_SPEC>`"]
pub type CRC = crate::Reg<crc::CRC_SPEC>;
#[doc = "CRC"]
pub mod crc;
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
