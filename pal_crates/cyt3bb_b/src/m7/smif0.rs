#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control"]
    pub ctl: CTL,
    #[doc = "0x04 - Status"]
    pub status: STATUS,
    _reserved2: [u8; 0x08],
    #[doc = "0x10 - Internal Clocking Delay Tap Select Register 0"]
    pub int_clock_delay_tap_sel0: INT_CLOCK_DELAY_TAP_SEL0,
    #[doc = "0x14 - Internal Clocking Delay Tap Select Register 1"]
    pub int_clock_delay_tap_sel1: INT_CLOCK_DELAY_TAP_SEL1,
    #[doc = "0x18 - Data Learning Pattern"]
    pub dlp: DLP,
    _reserved5: [u8; 0x04],
    #[doc = "0x20 - Data Learning Status Register 0"]
    pub dl_status0: DL_STATUS0,
    #[doc = "0x24 - Data Learning Status Register 1"]
    pub dl_status1: DL_STATUS1,
    _reserved7: [u8; 0x08],
    #[doc = "0x30 - Delay Tap Select Register"]
    pub delay_tap_sel: DELAY_TAP_SEL,
    _reserved8: [u8; 0x10],
    #[doc = "0x44 - Transmitter command FIFO status"]
    pub tx_cmd_fifo_status: TX_CMD_FIFO_STATUS,
    _reserved9: [u8; 0x08],
    #[doc = "0x50 - Transmitter command FIFO write"]
    pub tx_cmd_fifo_wr: TX_CMD_FIFO_WR,
    _reserved10: [u8; 0x2c],
    #[doc = "0x80 - Transmitter data FIFO control"]
    pub tx_data_fifo_ctl: TX_DATA_FIFO_CTL,
    #[doc = "0x84 - Transmitter data FIFO status"]
    pub tx_data_fifo_status: TX_DATA_FIFO_STATUS,
    _reserved12: [u8; 0x08],
    #[doc = "0x90 - Transmitter data FIFO write"]
    pub tx_data_fifo_wr1: TX_DATA_FIFO_WR1,
    #[doc = "0x94 - Transmitter data FIFO write"]
    pub tx_data_fifo_wr2: TX_DATA_FIFO_WR2,
    #[doc = "0x98 - Transmitter data FIFO write"]
    pub tx_data_fifo_wr4: TX_DATA_FIFO_WR4,
    #[doc = "0x9c - Transmitter data FIFO write"]
    pub tx_data_fifo_wr1odd: TX_DATA_FIFO_WR1ODD,
    _reserved16: [u8; 0x20],
    #[doc = "0xc0 - Receiver data MMIO FIFO control"]
    pub rx_data_mmio_fifo_ctl: RX_DATA_MMIO_FIFO_CTL,
    #[doc = "0xc4 - Receiver data MMIO FIFO status"]
    pub rx_data_mmio_fifo_status: RX_DATA_MMIO_FIFO_STATUS,
    #[doc = "0xc8 - Receiver data FIFO status"]
    pub rx_data_fifo_status: RX_DATA_FIFO_STATUS,
    _reserved19: [u8; 0x04],
    #[doc = "0xd0 - Receiver data MMIO FIFO read"]
    pub rx_data_mmio_fifo_rd1: RX_DATA_MMIO_FIFO_RD1,
    #[doc = "0xd4 - Receiver data MMIO FIFO read"]
    pub rx_data_mmio_fifo_rd2: RX_DATA_MMIO_FIFO_RD2,
    #[doc = "0xd8 - Receiver data MMIO FIFO read"]
    pub rx_data_mmio_fifo_rd4: RX_DATA_MMIO_FIFO_RD4,
    _reserved22: [u8; 0x04],
    #[doc = "0xe0 - Receiver data MMIO FIFO silent read"]
    pub rx_data_mmio_fifo_rd1_silent: RX_DATA_MMIO_FIFO_RD1_SILENT,
    _reserved23: [u8; 0x1c],
    #[doc = "0x100 - Slow cache control"]
    pub slow_ca_ctl: SLOW_CA_CTL,
    _reserved24: [u8; 0x04],
    #[doc = "0x108 - Slow cache command"]
    pub slow_ca_cmd: SLOW_CA_CMD,
    _reserved25: [u8; 0x74],
    #[doc = "0x180 - Fast cache control"]
    pub fast_ca_ctl: FAST_CA_CTL,
    _reserved26: [u8; 0x04],
    #[doc = "0x188 - Fast cache command"]
    pub fast_ca_cmd: FAST_CA_CMD,
    _reserved27: [u8; 0x74],
    #[doc = "0x200 - Cryptography Command"]
    pub crypto_cmd: CRYPTO_CMD,
    _reserved28: [u8; 0x1c],
    #[doc = "0x220 - Cryptography input 0"]
    pub crypto_input0: CRYPTO_INPUT0,
    #[doc = "0x224 - Cryptography input 1"]
    pub crypto_input1: CRYPTO_INPUT1,
    #[doc = "0x228 - Cryptography input 2"]
    pub crypto_input2: CRYPTO_INPUT2,
    #[doc = "0x22c - Cryptography input 3"]
    pub crypto_input3: CRYPTO_INPUT3,
    _reserved32: [u8; 0x10],
    #[doc = "0x240 - Cryptography key 0"]
    pub crypto_key0: CRYPTO_KEY0,
    #[doc = "0x244 - Cryptography key 1"]
    pub crypto_key1: CRYPTO_KEY1,
    #[doc = "0x248 - Cryptography key 2"]
    pub crypto_key2: CRYPTO_KEY2,
    #[doc = "0x24c - Cryptography key 3"]
    pub crypto_key3: CRYPTO_KEY3,
    _reserved36: [u8; 0x10],
    #[doc = "0x260 - Cryptography output 0"]
    pub crypto_output0: CRYPTO_OUTPUT0,
    #[doc = "0x264 - Cryptography output 1"]
    pub crypto_output1: CRYPTO_OUTPUT1,
    #[doc = "0x268 - Cryptography output 2"]
    pub crypto_output2: CRYPTO_OUTPUT2,
    #[doc = "0x26c - Cryptography output 3"]
    pub crypto_output3: CRYPTO_OUTPUT3,
    _reserved40: [u8; 0x90],
    #[doc = "0x300 - CRC Command"]
    pub crc_cmd: CRC_CMD,
    _reserved41: [u8; 0x1c],
    #[doc = "0x320 - CRC input 0"]
    pub crc_input0: CRC_INPUT0,
    #[doc = "0x324 - CRC input 1"]
    pub crc_input1: CRC_INPUT1,
    _reserved43: [u8; 0x18],
    #[doc = "0x340 - CRC output"]
    pub crc_output: CRC_OUTPUT,
    _reserved44: [u8; 0x047c],
    #[doc = "0x7c0 - Interrupt register"]
    pub intr: INTR,
    #[doc = "0x7c4 - Interrupt set register"]
    pub intr_set: INTR_SET,
    #[doc = "0x7c8 - Interrupt mask register"]
    pub intr_mask: INTR_MASK,
    #[doc = "0x7cc - Interrupt masked register"]
    pub intr_masked: INTR_MASKED,
    _reserved48: [u8; 0x30],
    #[doc = "0x800..0x878 - Device (only used in XIP mode)"]
    pub device0: DEVICE,
    _reserved49: [u8; 0x08],
    #[doc = "0x880..0x8f8 - Device (only used in XIP mode)"]
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
#[doc = "INT_CLOCK_DELAY_TAP_SEL0 (rw) register accessor: an alias for `Reg<INT_CLOCK_DELAY_TAP_SEL0_SPEC>`"]
pub type INT_CLOCK_DELAY_TAP_SEL0 =
    crate::Reg<int_clock_delay_tap_sel0::INT_CLOCK_DELAY_TAP_SEL0_SPEC>;
#[doc = "Internal Clocking Delay Tap Select Register 0"]
pub mod int_clock_delay_tap_sel0;
#[doc = "INT_CLOCK_DELAY_TAP_SEL1 (rw) register accessor: an alias for `Reg<INT_CLOCK_DELAY_TAP_SEL1_SPEC>`"]
pub type INT_CLOCK_DELAY_TAP_SEL1 =
    crate::Reg<int_clock_delay_tap_sel1::INT_CLOCK_DELAY_TAP_SEL1_SPEC>;
#[doc = "Internal Clocking Delay Tap Select Register 1"]
pub mod int_clock_delay_tap_sel1;
#[doc = "DLP (rw) register accessor: an alias for `Reg<DLP_SPEC>`"]
pub type DLP = crate::Reg<dlp::DLP_SPEC>;
#[doc = "Data Learning Pattern"]
pub mod dlp;
#[doc = "DL_STATUS0 (r) register accessor: an alias for `Reg<DL_STATUS0_SPEC>`"]
pub type DL_STATUS0 = crate::Reg<dl_status0::DL_STATUS0_SPEC>;
#[doc = "Data Learning Status Register 0"]
pub mod dl_status0;
#[doc = "DL_STATUS1 (r) register accessor: an alias for `Reg<DL_STATUS1_SPEC>`"]
pub type DL_STATUS1 = crate::Reg<dl_status1::DL_STATUS1_SPEC>;
#[doc = "Data Learning Status Register 1"]
pub mod dl_status1;
#[doc = "DELAY_TAP_SEL (rw) register accessor: an alias for `Reg<DELAY_TAP_SEL_SPEC>`"]
pub type DELAY_TAP_SEL = crate::Reg<delay_tap_sel::DELAY_TAP_SEL_SPEC>;
#[doc = "Delay Tap Select Register"]
pub mod delay_tap_sel;
#[doc = "TX_CMD_FIFO_STATUS (r) register accessor: an alias for `Reg<TX_CMD_FIFO_STATUS_SPEC>`"]
pub type TX_CMD_FIFO_STATUS = crate::Reg<tx_cmd_fifo_status::TX_CMD_FIFO_STATUS_SPEC>;
#[doc = "Transmitter command FIFO status"]
pub mod tx_cmd_fifo_status;
#[doc = "TX_CMD_FIFO_WR (w) register accessor: an alias for `Reg<TX_CMD_FIFO_WR_SPEC>`"]
pub type TX_CMD_FIFO_WR = crate::Reg<tx_cmd_fifo_wr::TX_CMD_FIFO_WR_SPEC>;
#[doc = "Transmitter command FIFO write"]
pub mod tx_cmd_fifo_wr;
#[doc = "TX_DATA_FIFO_CTL (rw) register accessor: an alias for `Reg<TX_DATA_FIFO_CTL_SPEC>`"]
pub type TX_DATA_FIFO_CTL = crate::Reg<tx_data_fifo_ctl::TX_DATA_FIFO_CTL_SPEC>;
#[doc = "Transmitter data FIFO control"]
pub mod tx_data_fifo_ctl;
#[doc = "TX_DATA_FIFO_STATUS (r) register accessor: an alias for `Reg<TX_DATA_FIFO_STATUS_SPEC>`"]
pub type TX_DATA_FIFO_STATUS = crate::Reg<tx_data_fifo_status::TX_DATA_FIFO_STATUS_SPEC>;
#[doc = "Transmitter data FIFO status"]
pub mod tx_data_fifo_status;
#[doc = "TX_DATA_FIFO_WR1 (w) register accessor: an alias for `Reg<TX_DATA_FIFO_WR1_SPEC>`"]
pub type TX_DATA_FIFO_WR1 = crate::Reg<tx_data_fifo_wr1::TX_DATA_FIFO_WR1_SPEC>;
#[doc = "Transmitter data FIFO write"]
pub mod tx_data_fifo_wr1;
#[doc = "TX_DATA_FIFO_WR2 (w) register accessor: an alias for `Reg<TX_DATA_FIFO_WR2_SPEC>`"]
pub type TX_DATA_FIFO_WR2 = crate::Reg<tx_data_fifo_wr2::TX_DATA_FIFO_WR2_SPEC>;
#[doc = "Transmitter data FIFO write"]
pub mod tx_data_fifo_wr2;
#[doc = "TX_DATA_FIFO_WR4 (w) register accessor: an alias for `Reg<TX_DATA_FIFO_WR4_SPEC>`"]
pub type TX_DATA_FIFO_WR4 = crate::Reg<tx_data_fifo_wr4::TX_DATA_FIFO_WR4_SPEC>;
#[doc = "Transmitter data FIFO write"]
pub mod tx_data_fifo_wr4;
#[doc = "TX_DATA_FIFO_WR1ODD (w) register accessor: an alias for `Reg<TX_DATA_FIFO_WR1ODD_SPEC>`"]
pub type TX_DATA_FIFO_WR1ODD = crate::Reg<tx_data_fifo_wr1odd::TX_DATA_FIFO_WR1ODD_SPEC>;
#[doc = "Transmitter data FIFO write"]
pub mod tx_data_fifo_wr1odd;
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
#[doc = "CRYPTO_CMD (rw) register accessor: an alias for `Reg<CRYPTO_CMD_SPEC>`"]
pub type CRYPTO_CMD = crate::Reg<crypto_cmd::CRYPTO_CMD_SPEC>;
#[doc = "Cryptography Command"]
pub mod crypto_cmd;
#[doc = "CRYPTO_INPUT0 (rw) register accessor: an alias for `Reg<CRYPTO_INPUT0_SPEC>`"]
pub type CRYPTO_INPUT0 = crate::Reg<crypto_input0::CRYPTO_INPUT0_SPEC>;
#[doc = "Cryptography input 0"]
pub mod crypto_input0;
#[doc = "CRYPTO_INPUT1 (rw) register accessor: an alias for `Reg<CRYPTO_INPUT1_SPEC>`"]
pub type CRYPTO_INPUT1 = crate::Reg<crypto_input1::CRYPTO_INPUT1_SPEC>;
#[doc = "Cryptography input 1"]
pub mod crypto_input1;
#[doc = "CRYPTO_INPUT2 (rw) register accessor: an alias for `Reg<CRYPTO_INPUT2_SPEC>`"]
pub type CRYPTO_INPUT2 = crate::Reg<crypto_input2::CRYPTO_INPUT2_SPEC>;
#[doc = "Cryptography input 2"]
pub mod crypto_input2;
#[doc = "CRYPTO_INPUT3 (rw) register accessor: an alias for `Reg<CRYPTO_INPUT3_SPEC>`"]
pub type CRYPTO_INPUT3 = crate::Reg<crypto_input3::CRYPTO_INPUT3_SPEC>;
#[doc = "Cryptography input 3"]
pub mod crypto_input3;
#[doc = "CRYPTO_KEY0 (w) register accessor: an alias for `Reg<CRYPTO_KEY0_SPEC>`"]
pub type CRYPTO_KEY0 = crate::Reg<crypto_key0::CRYPTO_KEY0_SPEC>;
#[doc = "Cryptography key 0"]
pub mod crypto_key0;
#[doc = "CRYPTO_KEY1 (w) register accessor: an alias for `Reg<CRYPTO_KEY1_SPEC>`"]
pub type CRYPTO_KEY1 = crate::Reg<crypto_key1::CRYPTO_KEY1_SPEC>;
#[doc = "Cryptography key 1"]
pub mod crypto_key1;
#[doc = "CRYPTO_KEY2 (w) register accessor: an alias for `Reg<CRYPTO_KEY2_SPEC>`"]
pub type CRYPTO_KEY2 = crate::Reg<crypto_key2::CRYPTO_KEY2_SPEC>;
#[doc = "Cryptography key 2"]
pub mod crypto_key2;
#[doc = "CRYPTO_KEY3 (w) register accessor: an alias for `Reg<CRYPTO_KEY3_SPEC>`"]
pub type CRYPTO_KEY3 = crate::Reg<crypto_key3::CRYPTO_KEY3_SPEC>;
#[doc = "Cryptography key 3"]
pub mod crypto_key3;
#[doc = "CRYPTO_OUTPUT0 (rw) register accessor: an alias for `Reg<CRYPTO_OUTPUT0_SPEC>`"]
pub type CRYPTO_OUTPUT0 = crate::Reg<crypto_output0::CRYPTO_OUTPUT0_SPEC>;
#[doc = "Cryptography output 0"]
pub mod crypto_output0;
#[doc = "CRYPTO_OUTPUT1 (rw) register accessor: an alias for `Reg<CRYPTO_OUTPUT1_SPEC>`"]
pub type CRYPTO_OUTPUT1 = crate::Reg<crypto_output1::CRYPTO_OUTPUT1_SPEC>;
#[doc = "Cryptography output 1"]
pub mod crypto_output1;
#[doc = "CRYPTO_OUTPUT2 (rw) register accessor: an alias for `Reg<CRYPTO_OUTPUT2_SPEC>`"]
pub type CRYPTO_OUTPUT2 = crate::Reg<crypto_output2::CRYPTO_OUTPUT2_SPEC>;
#[doc = "Cryptography output 2"]
pub mod crypto_output2;
#[doc = "CRYPTO_OUTPUT3 (rw) register accessor: an alias for `Reg<CRYPTO_OUTPUT3_SPEC>`"]
pub type CRYPTO_OUTPUT3 = crate::Reg<crypto_output3::CRYPTO_OUTPUT3_SPEC>;
#[doc = "Cryptography output 3"]
pub mod crypto_output3;
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
#[doc = "Device (only used in XIP mode)"]
pub use self::device::DEVICE;
#[doc = r"Cluster"]
#[doc = "Device (only used in XIP mode)"]
pub mod device;
