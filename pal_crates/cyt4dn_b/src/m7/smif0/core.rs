#[doc = r"Register block"]
#[repr(C)]
pub struct CORE {
    #[doc = "0x00 - Control"]
    pub ctl: CTL,
    #[doc = "0x04 - Status"]
    pub status: STATUS,
    #[doc = "0x08 - Control 2"]
    pub ctl2: CTL2,
    _reserved3: [u8; 0x04],
    #[doc = "0x10 - DLP Delay Tap Select Register 0"]
    pub dlp_delay_tap_sel0: DLP_DELAY_TAP_SEL0,
    #[doc = "0x14 - DLP Delay Tap Select Register 1"]
    pub dlp_delay_tap_sel1: DLP_DELAY_TAP_SEL1,
    #[doc = "0x18 - DLP Control Register"]
    pub dlp_ctl: DLP_CTL,
    _reserved6: [u8; 0x04],
    #[doc = "0x20 - DLP Status Register 0"]
    pub dlp_status0: DLP_STATUS0,
    #[doc = "0x24 - DLP Status Register 1"]
    pub dlp_status1: DLP_STATUS1,
    _reserved8: [u8; 0x1c],
    #[doc = "0x44 - Transmitter command FIFO status"]
    pub tx_cmd_fifo_status: TX_CMD_FIFO_STATUS,
    #[doc = "0x48 - Transmitter command MMIO FIFO status"]
    pub tx_cmd_mmio_fifo_status: TX_CMD_MMIO_FIFO_STATUS,
    _reserved10: [u8; 0x04],
    #[doc = "0x50 - Transmitter command MMIO FIFO write"]
    pub tx_cmd_mmio_fifo_wr: TX_CMD_MMIO_FIFO_WR,
    _reserved11: [u8; 0x2c],
    #[doc = "0x80 - Transmitter data MMIO FIFO control"]
    pub tx_data_mmio_fifo_ctl: TX_DATA_MMIO_FIFO_CTL,
    #[doc = "0x84 - Transmitter data FIFO status"]
    pub tx_data_fifo_status: TX_DATA_FIFO_STATUS,
    #[doc = "0x88 - Transmitter data MMIO FIFO status"]
    pub tx_data_mmio_fifo_status: TX_DATA_MMIO_FIFO_STATUS,
    _reserved14: [u8; 0x04],
    #[doc = "0x90 - Transmitter data MMIO FIFO write"]
    pub tx_data_mmio_fifo_wr1: TX_DATA_MMIO_FIFO_WR1,
    #[doc = "0x94 - Transmitter data MMIO FIFO write"]
    pub tx_data_mmio_fifo_wr2: TX_DATA_MMIO_FIFO_WR2,
    #[doc = "0x98 - Transmitter data MMIO FIFO write"]
    pub tx_data_mmio_fifo_wr4: TX_DATA_MMIO_FIFO_WR4,
    #[doc = "0x9c - Transmitter data MMIO FIFO write"]
    pub tx_data_mmio_fifo_wr1odd: TX_DATA_MMIO_FIFO_WR1ODD,
    _reserved18: [u8; 0x20],
    #[doc = "0xc0 - Receiver data MMIO FIFO control"]
    pub rx_data_mmio_fifo_ctl: RX_DATA_MMIO_FIFO_CTL,
    #[doc = "0xc4 - Receiver data MMIO FIFO status"]
    pub rx_data_mmio_fifo_status: RX_DATA_MMIO_FIFO_STATUS,
    #[doc = "0xc8 - Receiver data FIFO status"]
    pub rx_data_fifo_status: RX_DATA_FIFO_STATUS,
    _reserved21: [u8; 0x04],
    #[doc = "0xd0 - Receiver data MMIO FIFO read"]
    pub rx_data_mmio_fifo_rd1: RX_DATA_MMIO_FIFO_RD1,
    #[doc = "0xd4 - Receiver data MMIO FIFO read"]
    pub rx_data_mmio_fifo_rd2: RX_DATA_MMIO_FIFO_RD2,
    #[doc = "0xd8 - Receiver data MMIO FIFO read"]
    pub rx_data_mmio_fifo_rd4: RX_DATA_MMIO_FIFO_RD4,
    _reserved24: [u8; 0x04],
    #[doc = "0xe0 - Receiver data MMIO FIFO silent read"]
    pub rx_data_mmio_fifo_rd1_silent: RX_DATA_MMIO_FIFO_RD1_SILENT,
    _reserved25: [u8; 0x1c],
    #[doc = "0x100 - Slow cache control"]
    pub slow_ca_ctl: SLOW_CA_CTL,
    _reserved26: [u8; 0x04],
    #[doc = "0x108 - Slow cache command"]
    pub slow_ca_cmd: SLOW_CA_CMD,
    _reserved27: [u8; 0x74],
    #[doc = "0x180 - Fast cache control"]
    pub fast_ca_ctl: FAST_CA_CTL,
    _reserved28: [u8; 0x04],
    #[doc = "0x188 - Fast cache command"]
    pub fast_ca_cmd: FAST_CA_CMD,
    _reserved29: [u8; 0x74],
    #[doc = "0x200..0x270 - Cryptography registers (one set for each key)"]
    pub smif_crypto0: SMIF_CRYPTO,
    _reserved30: [u8; 0x10],
    #[doc = "0x280..0x2f0 - Cryptography registers (one set for each key)"]
    pub smif_crypto1: SMIF_CRYPTO,
    _reserved31: [u8; 0x10],
    #[doc = "0x300..0x370 - Cryptography registers (one set for each key)"]
    pub smif_crypto2: SMIF_CRYPTO,
    _reserved32: [u8; 0x10],
    #[doc = "0x380..0x3f0 - Cryptography registers (one set for each key)"]
    pub smif_crypto3: SMIF_CRYPTO,
    _reserved33: [u8; 0x10],
    #[doc = "0x400..0x470 - Cryptography registers (one set for each key)"]
    pub smif_crypto4: SMIF_CRYPTO,
    _reserved34: [u8; 0x10],
    #[doc = "0x480..0x4f0 - Cryptography registers (one set for each key)"]
    pub smif_crypto5: SMIF_CRYPTO,
    _reserved35: [u8; 0x10],
    #[doc = "0x500..0x570 - Cryptography registers (one set for each key)"]
    pub smif_crypto6: SMIF_CRYPTO,
    _reserved36: [u8; 0x10],
    #[doc = "0x580..0x5f0 - Cryptography registers (one set for each key)"]
    pub smif_crypto7: SMIF_CRYPTO,
    _reserved37: [u8; 0x10],
    #[doc = "0x600 - CRC Command"]
    pub crc_cmd: CRC_CMD,
    _reserved38: [u8; 0x1c],
    #[doc = "0x620 - CRC input 0"]
    pub crc_input0: CRC_INPUT0,
    #[doc = "0x624 - CRC input 1"]
    pub crc_input1: CRC_INPUT1,
    _reserved40: [u8; 0x18],
    #[doc = "0x640 - CRC output"]
    pub crc_output: CRC_OUTPUT,
    _reserved41: [u8; 0x017c],
    #[doc = "0x7c0 - Interrupt register"]
    pub intr: INTR,
    #[doc = "0x7c4 - Interrupt set register"]
    pub intr_set: INTR_SET,
    #[doc = "0x7c8 - Interrupt mask register"]
    pub intr_mask: INTR_MASK,
    #[doc = "0x7cc - Interrupt masked register"]
    pub intr_masked: INTR_MASKED,
    _reserved45: [u8; 0x30],
    #[doc = "0x800..0x878 - Device (only used for XIP acceses)"]
    pub device0: DEVICE,
    _reserved46: [u8; 0x08],
    #[doc = "0x880..0x8f8 - Device (only used for XIP acceses)"]
    pub device1: DEVICE,
}
#[doc = "CTL (rw) register accessor: an alias for `Reg<CTL_SPEC>`"]
pub type CTL = crate::Reg<ctl::CTL_SPEC>;
#[doc = "Control"]
pub mod ctl;
#[doc = "STATUS (r) register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Status"]
pub mod status;
#[doc = "CTL2 (rw) register accessor: an alias for `Reg<CTL2_SPEC>`"]
pub type CTL2 = crate::Reg<ctl2::CTL2_SPEC>;
#[doc = "Control 2"]
pub mod ctl2;
#[doc = "DLP_DELAY_TAP_SEL0 (r) register accessor: an alias for `Reg<DLP_DELAY_TAP_SEL0_SPEC>`"]
pub type DLP_DELAY_TAP_SEL0 = crate::Reg<dlp_delay_tap_sel0::DLP_DELAY_TAP_SEL0_SPEC>;
#[doc = "DLP Delay Tap Select Register 0"]
pub mod dlp_delay_tap_sel0;
#[doc = "DLP_DELAY_TAP_SEL1 (r) register accessor: an alias for `Reg<DLP_DELAY_TAP_SEL1_SPEC>`"]
pub type DLP_DELAY_TAP_SEL1 = crate::Reg<dlp_delay_tap_sel1::DLP_DELAY_TAP_SEL1_SPEC>;
#[doc = "DLP Delay Tap Select Register 1"]
pub mod dlp_delay_tap_sel1;
#[doc = "DLP_CTL (rw) register accessor: an alias for `Reg<DLP_CTL_SPEC>`"]
pub type DLP_CTL = crate::Reg<dlp_ctl::DLP_CTL_SPEC>;
#[doc = "DLP Control Register"]
pub mod dlp_ctl;
#[doc = "DLP_STATUS0 (r) register accessor: an alias for `Reg<DLP_STATUS0_SPEC>`"]
pub type DLP_STATUS0 = crate::Reg<dlp_status0::DLP_STATUS0_SPEC>;
#[doc = "DLP Status Register 0"]
pub mod dlp_status0;
#[doc = "DLP_STATUS1 (r) register accessor: an alias for `Reg<DLP_STATUS1_SPEC>`"]
pub type DLP_STATUS1 = crate::Reg<dlp_status1::DLP_STATUS1_SPEC>;
#[doc = "DLP Status Register 1"]
pub mod dlp_status1;
#[doc = "TX_CMD_FIFO_STATUS (r) register accessor: an alias for `Reg<TX_CMD_FIFO_STATUS_SPEC>`"]
pub type TX_CMD_FIFO_STATUS = crate::Reg<tx_cmd_fifo_status::TX_CMD_FIFO_STATUS_SPEC>;
#[doc = "Transmitter command FIFO status"]
pub mod tx_cmd_fifo_status;
#[doc = "TX_CMD_MMIO_FIFO_STATUS (r) register accessor: an alias for `Reg<TX_CMD_MMIO_FIFO_STATUS_SPEC>`"]
pub type TX_CMD_MMIO_FIFO_STATUS =
    crate::Reg<tx_cmd_mmio_fifo_status::TX_CMD_MMIO_FIFO_STATUS_SPEC>;
#[doc = "Transmitter command MMIO FIFO status"]
pub mod tx_cmd_mmio_fifo_status;
#[doc = "TX_CMD_MMIO_FIFO_WR (w) register accessor: an alias for `Reg<TX_CMD_MMIO_FIFO_WR_SPEC>`"]
pub type TX_CMD_MMIO_FIFO_WR = crate::Reg<tx_cmd_mmio_fifo_wr::TX_CMD_MMIO_FIFO_WR_SPEC>;
#[doc = "Transmitter command MMIO FIFO write"]
pub mod tx_cmd_mmio_fifo_wr;
#[doc = "TX_DATA_MMIO_FIFO_CTL (rw) register accessor: an alias for `Reg<TX_DATA_MMIO_FIFO_CTL_SPEC>`"]
pub type TX_DATA_MMIO_FIFO_CTL = crate::Reg<tx_data_mmio_fifo_ctl::TX_DATA_MMIO_FIFO_CTL_SPEC>;
#[doc = "Transmitter data MMIO FIFO control"]
pub mod tx_data_mmio_fifo_ctl;
#[doc = "TX_DATA_FIFO_STATUS (r) register accessor: an alias for `Reg<TX_DATA_FIFO_STATUS_SPEC>`"]
pub type TX_DATA_FIFO_STATUS = crate::Reg<tx_data_fifo_status::TX_DATA_FIFO_STATUS_SPEC>;
#[doc = "Transmitter data FIFO status"]
pub mod tx_data_fifo_status;
#[doc = "TX_DATA_MMIO_FIFO_STATUS (r) register accessor: an alias for `Reg<TX_DATA_MMIO_FIFO_STATUS_SPEC>`"]
pub type TX_DATA_MMIO_FIFO_STATUS =
    crate::Reg<tx_data_mmio_fifo_status::TX_DATA_MMIO_FIFO_STATUS_SPEC>;
#[doc = "Transmitter data MMIO FIFO status"]
pub mod tx_data_mmio_fifo_status;
#[doc = "TX_DATA_MMIO_FIFO_WR1 (w) register accessor: an alias for `Reg<TX_DATA_MMIO_FIFO_WR1_SPEC>`"]
pub type TX_DATA_MMIO_FIFO_WR1 = crate::Reg<tx_data_mmio_fifo_wr1::TX_DATA_MMIO_FIFO_WR1_SPEC>;
#[doc = "Transmitter data MMIO FIFO write"]
pub mod tx_data_mmio_fifo_wr1;
#[doc = "TX_DATA_MMIO_FIFO_WR2 (w) register accessor: an alias for `Reg<TX_DATA_MMIO_FIFO_WR2_SPEC>`"]
pub type TX_DATA_MMIO_FIFO_WR2 = crate::Reg<tx_data_mmio_fifo_wr2::TX_DATA_MMIO_FIFO_WR2_SPEC>;
#[doc = "Transmitter data MMIO FIFO write"]
pub mod tx_data_mmio_fifo_wr2;
#[doc = "TX_DATA_MMIO_FIFO_WR4 (w) register accessor: an alias for `Reg<TX_DATA_MMIO_FIFO_WR4_SPEC>`"]
pub type TX_DATA_MMIO_FIFO_WR4 = crate::Reg<tx_data_mmio_fifo_wr4::TX_DATA_MMIO_FIFO_WR4_SPEC>;
#[doc = "Transmitter data MMIO FIFO write"]
pub mod tx_data_mmio_fifo_wr4;
#[doc = "TX_DATA_MMIO_FIFO_WR1ODD (w) register accessor: an alias for `Reg<TX_DATA_MMIO_FIFO_WR1ODD_SPEC>`"]
pub type TX_DATA_MMIO_FIFO_WR1ODD =
    crate::Reg<tx_data_mmio_fifo_wr1odd::TX_DATA_MMIO_FIFO_WR1ODD_SPEC>;
#[doc = "Transmitter data MMIO FIFO write"]
pub mod tx_data_mmio_fifo_wr1odd;
#[doc = "RX_DATA_MMIO_FIFO_CTL (rw) register accessor: an alias for `Reg<RX_DATA_MMIO_FIFO_CTL_SPEC>`"]
pub type RX_DATA_MMIO_FIFO_CTL = crate::Reg<rx_data_mmio_fifo_ctl::RX_DATA_MMIO_FIFO_CTL_SPEC>;
#[doc = "Receiver data MMIO FIFO control"]
pub mod rx_data_mmio_fifo_ctl;
#[doc = "RX_DATA_MMIO_FIFO_STATUS (r) register accessor: an alias for `Reg<RX_DATA_MMIO_FIFO_STATUS_SPEC>`"]
pub type RX_DATA_MMIO_FIFO_STATUS =
    crate::Reg<rx_data_mmio_fifo_status::RX_DATA_MMIO_FIFO_STATUS_SPEC>;
#[doc = "Receiver data MMIO FIFO status"]
pub mod rx_data_mmio_fifo_status;
#[doc = "RX_DATA_FIFO_STATUS (r) register accessor: an alias for `Reg<RX_DATA_FIFO_STATUS_SPEC>`"]
pub type RX_DATA_FIFO_STATUS = crate::Reg<rx_data_fifo_status::RX_DATA_FIFO_STATUS_SPEC>;
#[doc = "Receiver data FIFO status"]
pub mod rx_data_fifo_status;
#[doc = "RX_DATA_MMIO_FIFO_RD1 (r) register accessor: an alias for `Reg<RX_DATA_MMIO_FIFO_RD1_SPEC>`"]
pub type RX_DATA_MMIO_FIFO_RD1 = crate::Reg<rx_data_mmio_fifo_rd1::RX_DATA_MMIO_FIFO_RD1_SPEC>;
#[doc = "Receiver data MMIO FIFO read"]
pub mod rx_data_mmio_fifo_rd1;
#[doc = "RX_DATA_MMIO_FIFO_RD2 (r) register accessor: an alias for `Reg<RX_DATA_MMIO_FIFO_RD2_SPEC>`"]
pub type RX_DATA_MMIO_FIFO_RD2 = crate::Reg<rx_data_mmio_fifo_rd2::RX_DATA_MMIO_FIFO_RD2_SPEC>;
#[doc = "Receiver data MMIO FIFO read"]
pub mod rx_data_mmio_fifo_rd2;
#[doc = "RX_DATA_MMIO_FIFO_RD4 (r) register accessor: an alias for `Reg<RX_DATA_MMIO_FIFO_RD4_SPEC>`"]
pub type RX_DATA_MMIO_FIFO_RD4 = crate::Reg<rx_data_mmio_fifo_rd4::RX_DATA_MMIO_FIFO_RD4_SPEC>;
#[doc = "Receiver data MMIO FIFO read"]
pub mod rx_data_mmio_fifo_rd4;
#[doc = "RX_DATA_MMIO_FIFO_RD1_SILENT (r) register accessor: an alias for `Reg<RX_DATA_MMIO_FIFO_RD1_SILENT_SPEC>`"]
pub type RX_DATA_MMIO_FIFO_RD1_SILENT =
    crate::Reg<rx_data_mmio_fifo_rd1_silent::RX_DATA_MMIO_FIFO_RD1_SILENT_SPEC>;
#[doc = "Receiver data MMIO FIFO silent read"]
pub mod rx_data_mmio_fifo_rd1_silent;
#[doc = "SLOW_CA_CTL (rw) register accessor: an alias for `Reg<SLOW_CA_CTL_SPEC>`"]
pub type SLOW_CA_CTL = crate::Reg<slow_ca_ctl::SLOW_CA_CTL_SPEC>;
#[doc = "Slow cache control"]
pub mod slow_ca_ctl;
#[doc = "SLOW_CA_CMD (rw) register accessor: an alias for `Reg<SLOW_CA_CMD_SPEC>`"]
pub type SLOW_CA_CMD = crate::Reg<slow_ca_cmd::SLOW_CA_CMD_SPEC>;
#[doc = "Slow cache command"]
pub mod slow_ca_cmd;
#[doc = "FAST_CA_CTL (rw) register accessor: an alias for `Reg<FAST_CA_CTL_SPEC>`"]
pub type FAST_CA_CTL = crate::Reg<fast_ca_ctl::FAST_CA_CTL_SPEC>;
#[doc = "Fast cache control"]
pub mod fast_ca_ctl;
#[doc = "FAST_CA_CMD (rw) register accessor: an alias for `Reg<FAST_CA_CMD_SPEC>`"]
pub type FAST_CA_CMD = crate::Reg<fast_ca_cmd::FAST_CA_CMD_SPEC>;
#[doc = "Fast cache command"]
pub mod fast_ca_cmd;
#[doc = "Cryptography registers (one set for each key)"]
pub use self::smif_crypto::SMIF_CRYPTO;
#[doc = r"Cluster"]
#[doc = "Cryptography registers (one set for each key)"]
pub mod smif_crypto;
#[doc = "CRC_CMD (rw) register accessor: an alias for `Reg<CRC_CMD_SPEC>`"]
pub type CRC_CMD = crate::Reg<crc_cmd::CRC_CMD_SPEC>;
#[doc = "CRC Command"]
pub mod crc_cmd;
#[doc = "CRC_INPUT0 (rw) register accessor: an alias for `Reg<CRC_INPUT0_SPEC>`"]
pub type CRC_INPUT0 = crate::Reg<crc_input0::CRC_INPUT0_SPEC>;
#[doc = "CRC input 0"]
pub mod crc_input0;
#[doc = "CRC_INPUT1 (rw) register accessor: an alias for `Reg<CRC_INPUT1_SPEC>`"]
pub type CRC_INPUT1 = crate::Reg<crc_input1::CRC_INPUT1_SPEC>;
#[doc = "CRC input 1"]
pub mod crc_input1;
#[doc = "CRC_OUTPUT (r) register accessor: an alias for `Reg<CRC_OUTPUT_SPEC>`"]
pub type CRC_OUTPUT = crate::Reg<crc_output::CRC_OUTPUT_SPEC>;
#[doc = "CRC output"]
pub mod crc_output;
#[doc = "INTR (rw) register accessor: an alias for `Reg<INTR_SPEC>`"]
pub type INTR = crate::Reg<intr::INTR_SPEC>;
#[doc = "Interrupt register"]
pub mod intr;
#[doc = "INTR_SET (rw) register accessor: an alias for `Reg<INTR_SET_SPEC>`"]
pub type INTR_SET = crate::Reg<intr_set::INTR_SET_SPEC>;
#[doc = "Interrupt set register"]
pub mod intr_set;
#[doc = "INTR_MASK (rw) register accessor: an alias for `Reg<INTR_MASK_SPEC>`"]
pub type INTR_MASK = crate::Reg<intr_mask::INTR_MASK_SPEC>;
#[doc = "Interrupt mask register"]
pub mod intr_mask;
#[doc = "INTR_MASKED (r) register accessor: an alias for `Reg<INTR_MASKED_SPEC>`"]
pub type INTR_MASKED = crate::Reg<intr_masked::INTR_MASKED_SPEC>;
#[doc = "Interrupt masked register"]
pub mod intr_masked;
#[doc = "Device (only used for XIP acceses)"]
pub use self::device::DEVICE;
#[doc = r"Cluster"]
#[doc = "Device (only used for XIP acceses)"]
pub mod device;
