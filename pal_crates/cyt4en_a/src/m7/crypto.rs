#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control"]
    pub ctl: CTL,
    _reserved1: [u8; 0x04],
    #[doc = "0x08 - SRAM power control"]
    pub ram_pwr_ctl: RAM_PWR_CTL,
    #[doc = "0x0c - SRAM power delay control"]
    pub ram_pwr_delay_ctl: RAM_PWR_DELAY_CTL,
    #[doc = "0x10 - ECC control"]
    pub ecc_ctl: ECC_CTL,
    _reserved4: [u8; 0x0c],
    #[doc = "0x20 - Error status 0"]
    pub error_status0: ERROR_STATUS0,
    #[doc = "0x24 - Error status 1"]
    pub error_status1: ERROR_STATUS1,
    _reserved6: [u8; 0xd8],
    #[doc = "0x100 - Interrupt register"]
    pub intr: INTR,
    #[doc = "0x104 - Interrupt set register"]
    pub intr_set: INTR_SET,
    #[doc = "0x108 - Interrupt mask register"]
    pub intr_mask: INTR_MASK,
    #[doc = "0x10c - Interrupt masked register"]
    pub intr_masked: INTR_MASKED,
    _reserved10: [u8; 0xf0],
    #[doc = "0x200 - Pseudo random LFSR control 0"]
    pub pr_lfsr_ctl0: PR_LFSR_CTL0,
    #[doc = "0x204 - Pseudo random LFSR control 1"]
    pub pr_lfsr_ctl1: PR_LFSR_CTL1,
    #[doc = "0x208 - Pseudo random LFSR control 2"]
    pub pr_lfsr_ctl2: PR_LFSR_CTL2,
    #[doc = "0x20c - Pseudo random maximum control"]
    pub pr_max_ctl: PR_MAX_CTL,
    #[doc = "0x210 - Pseudo random command"]
    pub pr_cmd: PR_CMD,
    _reserved15: [u8; 0x04],
    #[doc = "0x218 - Pseudo random result"]
    pub pr_result: PR_RESULT,
    _reserved16: [u8; 0x64],
    #[doc = "0x280 - True random control 0"]
    pub tr_ctl0: TR_CTL0,
    #[doc = "0x284 - True random control 1"]
    pub tr_ctl1: TR_CTL1,
    #[doc = "0x288 - True random control 2"]
    pub tr_ctl2: TR_CTL2,
    #[doc = "0x28c - True random status"]
    pub tr_status: TR_STATUS,
    #[doc = "0x290 - True random command"]
    pub tr_cmd: TR_CMD,
    _reserved21: [u8; 0x04],
    #[doc = "0x298 - True random result"]
    pub tr_result: TR_RESULT,
    _reserved22: [u8; 0x04],
    #[doc = "0x2a0 - True random GARO control"]
    pub tr_garo_ctl: TR_GARO_CTL,
    #[doc = "0x2a4 - True random FIRO control"]
    pub tr_firo_ctl: TR_FIRO_CTL,
    _reserved24: [u8; 0x18],
    #[doc = "0x2c0 - True random monitor control"]
    pub tr_mon_ctl: TR_MON_CTL,
    _reserved25: [u8; 0x04],
    #[doc = "0x2c8 - True random monitor command"]
    pub tr_mon_cmd: TR_MON_CMD,
    _reserved26: [u8; 0x04],
    #[doc = "0x2d0 - True random monitor RC control"]
    pub tr_mon_rc_ctl: TR_MON_RC_CTL,
    _reserved27: [u8; 0x04],
    #[doc = "0x2d8 - True random monitor RC status 0"]
    pub tr_mon_rc_status0: TR_MON_RC_STATUS0,
    #[doc = "0x2dc - True random monitor RC status 1"]
    pub tr_mon_rc_status1: TR_MON_RC_STATUS1,
    #[doc = "0x2e0 - True random monitor AP control"]
    pub tr_mon_ap_ctl: TR_MON_AP_CTL,
    _reserved30: [u8; 0x04],
    #[doc = "0x2e8 - True random monitor AP status 0"]
    pub tr_mon_ap_status0: TR_MON_AP_STATUS0,
    #[doc = "0x2ec - True random monitor AP status 1"]
    pub tr_mon_ap_status1: TR_MON_AP_STATUS1,
    _reserved32: [u8; 0x0d14],
    #[doc = "0x1004 - Status"]
    pub status: STATUS,
    _reserved33: [u8; 0x38],
    #[doc = "0x1040 - Instruction FIFO control"]
    pub instr_ff_ctl: INSTR_FF_CTL,
    #[doc = "0x1044 - Instruction FIFO status"]
    pub instr_ff_status: INSTR_FF_STATUS,
    #[doc = "0x1048 - Instruction FIFO write"]
    pub instr_ff_wr: INSTR_FF_WR,
    _reserved36: [u8; 0x74],
    #[doc = "0x10c0 - Load 0 FIFO status"]
    pub load0_ff_status: LOAD0_FF_STATUS,
    _reserved37: [u8; 0x0c],
    #[doc = "0x10d0 - Load 1 FIFO status"]
    pub load1_ff_status: LOAD1_FF_STATUS,
    _reserved38: [u8; 0x1c],
    #[doc = "0x10f0 - Store FIFO status"]
    pub store_ff_status: STORE_FF_STATUS,
    _reserved39: [u8; 0x0c],
    #[doc = "0x1100 - AES control"]
    pub aes_ctl: AES_CTL,
    _reserved40: [u8; 0x7c],
    #[doc = "0x1180 - Result"]
    pub result: RESULT,
    _reserved41: [u8; 0x027c],
    #[doc = "0x1400 - CRC control"]
    pub crc_ctl: CRC_CTL,
    _reserved42: [u8; 0x0c],
    #[doc = "0x1410 - CRC data control"]
    pub crc_data_ctl: CRC_DATA_CTL,
    _reserved43: [u8; 0x0c],
    #[doc = "0x1420 - CRC polynomial control"]
    pub crc_pol_ctl: CRC_POL_CTL,
    _reserved44: [u8; 0x1c],
    #[doc = "0x1440 - CRC remainder control"]
    pub crc_rem_ctl: CRC_REM_CTL,
    _reserved45: [u8; 0x04],
    #[doc = "0x1448 - CRC remainder result"]
    pub crc_rem_result: CRC_REM_RESULT,
    _reserved46: [u8; 0x34],
    #[doc = "0x1480 - Vector unit control 0"]
    pub vu_ctl0: VU_CTL0,
    #[doc = "0x1484 - Vector unit control 1"]
    pub vu_ctl1: VU_CTL1,
    #[doc = "0x1488 - Vector unit control 2"]
    pub vu_ctl2: VU_CTL2,
    _reserved49: [u8; 0x04],
    #[doc = "0x1490 - Vector unit status"]
    pub vu_status: VU_STATUS,
    _reserved50: [u8; 0x2c],
    #[doc = "0x14c0..0x1500 - Vector unit register-file"]
    pub vu_rf_data: [VU_RF_DATA; 16],
    _reserved51: [u8; 0x0b00],
    #[doc = "0x2000 - Device key address 0 control"]
    pub dev_key_addr0_ctl: DEV_KEY_ADDR0_CTL,
    #[doc = "0x2004 - Device key address 0"]
    pub dev_key_addr0: DEV_KEY_ADDR0,
    _reserved53: [u8; 0x18],
    #[doc = "0x2020 - Device key address 1 control"]
    pub dev_key_addr1_ctl: DEV_KEY_ADDR1_CTL,
    #[doc = "0x2024 - Device key address 1 control"]
    pub dev_key_addr1: DEV_KEY_ADDR1,
    _reserved55: [u8; 0x58],
    #[doc = "0x2080 - Device key status"]
    pub dev_key_status: DEV_KEY_STATUS,
    _reserved56: [u8; 0x7c],
    #[doc = "0x2100 - Device key control 0"]
    pub dev_key_ctl0: DEV_KEY_CTL0,
    _reserved57: [u8; 0x1c],
    #[doc = "0x2120 - Device key control 1"]
    pub dev_key_ctl1: DEV_KEY_CTL1,
}
#[doc = "CTL (rw) register accessor: an alias for `Reg<CTL_SPEC>`"]
pub type CTL = crate::Reg<ctl::CTL_SPEC>;
#[doc = "Control"]
pub mod ctl;
#[doc = "RAM_PWR_CTL (rw) register accessor: an alias for `Reg<RAM_PWR_CTL_SPEC>`"]
pub type RAM_PWR_CTL = crate::Reg<ram_pwr_ctl::RAM_PWR_CTL_SPEC>;
#[doc = "SRAM power control"]
pub mod ram_pwr_ctl;
#[doc = "RAM_PWR_DELAY_CTL (rw) register accessor: an alias for `Reg<RAM_PWR_DELAY_CTL_SPEC>`"]
pub type RAM_PWR_DELAY_CTL = crate::Reg<ram_pwr_delay_ctl::RAM_PWR_DELAY_CTL_SPEC>;
#[doc = "SRAM power delay control"]
pub mod ram_pwr_delay_ctl;
#[doc = "ECC_CTL (rw) register accessor: an alias for `Reg<ECC_CTL_SPEC>`"]
pub type ECC_CTL = crate::Reg<ecc_ctl::ECC_CTL_SPEC>;
#[doc = "ECC control"]
pub mod ecc_ctl;
#[doc = "ERROR_STATUS0 (r) register accessor: an alias for `Reg<ERROR_STATUS0_SPEC>`"]
pub type ERROR_STATUS0 = crate::Reg<error_status0::ERROR_STATUS0_SPEC>;
#[doc = "Error status 0"]
pub mod error_status0;
#[doc = "ERROR_STATUS1 (rw) register accessor: an alias for `Reg<ERROR_STATUS1_SPEC>`"]
pub type ERROR_STATUS1 = crate::Reg<error_status1::ERROR_STATUS1_SPEC>;
#[doc = "Error status 1"]
pub mod error_status1;
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
#[doc = "PR_LFSR_CTL0 (rw) register accessor: an alias for `Reg<PR_LFSR_CTL0_SPEC>`"]
pub type PR_LFSR_CTL0 = crate::Reg<pr_lfsr_ctl0::PR_LFSR_CTL0_SPEC>;
#[doc = "Pseudo random LFSR control 0"]
pub mod pr_lfsr_ctl0;
#[doc = "PR_LFSR_CTL1 (rw) register accessor: an alias for `Reg<PR_LFSR_CTL1_SPEC>`"]
pub type PR_LFSR_CTL1 = crate::Reg<pr_lfsr_ctl1::PR_LFSR_CTL1_SPEC>;
#[doc = "Pseudo random LFSR control 1"]
pub mod pr_lfsr_ctl1;
#[doc = "PR_LFSR_CTL2 (rw) register accessor: an alias for `Reg<PR_LFSR_CTL2_SPEC>`"]
pub type PR_LFSR_CTL2 = crate::Reg<pr_lfsr_ctl2::PR_LFSR_CTL2_SPEC>;
#[doc = "Pseudo random LFSR control 2"]
pub mod pr_lfsr_ctl2;
#[doc = "PR_MAX_CTL (rw) register accessor: an alias for `Reg<PR_MAX_CTL_SPEC>`"]
pub type PR_MAX_CTL = crate::Reg<pr_max_ctl::PR_MAX_CTL_SPEC>;
#[doc = "Pseudo random maximum control"]
pub mod pr_max_ctl;
#[doc = "PR_CMD (rw) register accessor: an alias for `Reg<PR_CMD_SPEC>`"]
pub type PR_CMD = crate::Reg<pr_cmd::PR_CMD_SPEC>;
#[doc = "Pseudo random command"]
pub mod pr_cmd;
#[doc = "PR_RESULT (rw) register accessor: an alias for `Reg<PR_RESULT_SPEC>`"]
pub type PR_RESULT = crate::Reg<pr_result::PR_RESULT_SPEC>;
#[doc = "Pseudo random result"]
pub mod pr_result;
#[doc = "TR_CTL0 (rw) register accessor: an alias for `Reg<TR_CTL0_SPEC>`"]
pub type TR_CTL0 = crate::Reg<tr_ctl0::TR_CTL0_SPEC>;
#[doc = "True random control 0"]
pub mod tr_ctl0;
#[doc = "TR_CTL1 (rw) register accessor: an alias for `Reg<TR_CTL1_SPEC>`"]
pub type TR_CTL1 = crate::Reg<tr_ctl1::TR_CTL1_SPEC>;
#[doc = "True random control 1"]
pub mod tr_ctl1;
#[doc = "TR_CTL2 (rw) register accessor: an alias for `Reg<TR_CTL2_SPEC>`"]
pub type TR_CTL2 = crate::Reg<tr_ctl2::TR_CTL2_SPEC>;
#[doc = "True random control 2"]
pub mod tr_ctl2;
#[doc = "TR_STATUS (r) register accessor: an alias for `Reg<TR_STATUS_SPEC>`"]
pub type TR_STATUS = crate::Reg<tr_status::TR_STATUS_SPEC>;
#[doc = "True random status"]
pub mod tr_status;
#[doc = "TR_CMD (rw) register accessor: an alias for `Reg<TR_CMD_SPEC>`"]
pub type TR_CMD = crate::Reg<tr_cmd::TR_CMD_SPEC>;
#[doc = "True random command"]
pub mod tr_cmd;
#[doc = "TR_RESULT (rw) register accessor: an alias for `Reg<TR_RESULT_SPEC>`"]
pub type TR_RESULT = crate::Reg<tr_result::TR_RESULT_SPEC>;
#[doc = "True random result"]
pub mod tr_result;
#[doc = "TR_GARO_CTL (rw) register accessor: an alias for `Reg<TR_GARO_CTL_SPEC>`"]
pub type TR_GARO_CTL = crate::Reg<tr_garo_ctl::TR_GARO_CTL_SPEC>;
#[doc = "True random GARO control"]
pub mod tr_garo_ctl;
#[doc = "TR_FIRO_CTL (rw) register accessor: an alias for `Reg<TR_FIRO_CTL_SPEC>`"]
pub type TR_FIRO_CTL = crate::Reg<tr_firo_ctl::TR_FIRO_CTL_SPEC>;
#[doc = "True random FIRO control"]
pub mod tr_firo_ctl;
#[doc = "TR_MON_CTL (rw) register accessor: an alias for `Reg<TR_MON_CTL_SPEC>`"]
pub type TR_MON_CTL = crate::Reg<tr_mon_ctl::TR_MON_CTL_SPEC>;
#[doc = "True random monitor control"]
pub mod tr_mon_ctl;
#[doc = "TR_MON_CMD (rw) register accessor: an alias for `Reg<TR_MON_CMD_SPEC>`"]
pub type TR_MON_CMD = crate::Reg<tr_mon_cmd::TR_MON_CMD_SPEC>;
#[doc = "True random monitor command"]
pub mod tr_mon_cmd;
#[doc = "TR_MON_RC_CTL (rw) register accessor: an alias for `Reg<TR_MON_RC_CTL_SPEC>`"]
pub type TR_MON_RC_CTL = crate::Reg<tr_mon_rc_ctl::TR_MON_RC_CTL_SPEC>;
#[doc = "True random monitor RC control"]
pub mod tr_mon_rc_ctl;
#[doc = "TR_MON_RC_STATUS0 (r) register accessor: an alias for `Reg<TR_MON_RC_STATUS0_SPEC>`"]
pub type TR_MON_RC_STATUS0 = crate::Reg<tr_mon_rc_status0::TR_MON_RC_STATUS0_SPEC>;
#[doc = "True random monitor RC status 0"]
pub mod tr_mon_rc_status0;
#[doc = "TR_MON_RC_STATUS1 (r) register accessor: an alias for `Reg<TR_MON_RC_STATUS1_SPEC>`"]
pub type TR_MON_RC_STATUS1 = crate::Reg<tr_mon_rc_status1::TR_MON_RC_STATUS1_SPEC>;
#[doc = "True random monitor RC status 1"]
pub mod tr_mon_rc_status1;
#[doc = "TR_MON_AP_CTL (rw) register accessor: an alias for `Reg<TR_MON_AP_CTL_SPEC>`"]
pub type TR_MON_AP_CTL = crate::Reg<tr_mon_ap_ctl::TR_MON_AP_CTL_SPEC>;
#[doc = "True random monitor AP control"]
pub mod tr_mon_ap_ctl;
#[doc = "TR_MON_AP_STATUS0 (r) register accessor: an alias for `Reg<TR_MON_AP_STATUS0_SPEC>`"]
pub type TR_MON_AP_STATUS0 = crate::Reg<tr_mon_ap_status0::TR_MON_AP_STATUS0_SPEC>;
#[doc = "True random monitor AP status 0"]
pub mod tr_mon_ap_status0;
#[doc = "TR_MON_AP_STATUS1 (r) register accessor: an alias for `Reg<TR_MON_AP_STATUS1_SPEC>`"]
pub type TR_MON_AP_STATUS1 = crate::Reg<tr_mon_ap_status1::TR_MON_AP_STATUS1_SPEC>;
#[doc = "True random monitor AP status 1"]
pub mod tr_mon_ap_status1;
#[doc = "STATUS (r) register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Status"]
pub mod status;
#[doc = "INSTR_FF_CTL (rw) register accessor: an alias for `Reg<INSTR_FF_CTL_SPEC>`"]
pub type INSTR_FF_CTL = crate::Reg<instr_ff_ctl::INSTR_FF_CTL_SPEC>;
#[doc = "Instruction FIFO control"]
pub mod instr_ff_ctl;
#[doc = "INSTR_FF_STATUS (r) register accessor: an alias for `Reg<INSTR_FF_STATUS_SPEC>`"]
pub type INSTR_FF_STATUS = crate::Reg<instr_ff_status::INSTR_FF_STATUS_SPEC>;
#[doc = "Instruction FIFO status"]
pub mod instr_ff_status;
#[doc = "INSTR_FF_WR (w) register accessor: an alias for `Reg<INSTR_FF_WR_SPEC>`"]
pub type INSTR_FF_WR = crate::Reg<instr_ff_wr::INSTR_FF_WR_SPEC>;
#[doc = "Instruction FIFO write"]
pub mod instr_ff_wr;
#[doc = "LOAD0_FF_STATUS (r) register accessor: an alias for `Reg<LOAD0_FF_STATUS_SPEC>`"]
pub type LOAD0_FF_STATUS = crate::Reg<load0_ff_status::LOAD0_FF_STATUS_SPEC>;
#[doc = "Load 0 FIFO status"]
pub mod load0_ff_status;
#[doc = "LOAD1_FF_STATUS (r) register accessor: an alias for `Reg<LOAD1_FF_STATUS_SPEC>`"]
pub type LOAD1_FF_STATUS = crate::Reg<load1_ff_status::LOAD1_FF_STATUS_SPEC>;
#[doc = "Load 1 FIFO status"]
pub mod load1_ff_status;
#[doc = "STORE_FF_STATUS (r) register accessor: an alias for `Reg<STORE_FF_STATUS_SPEC>`"]
pub type STORE_FF_STATUS = crate::Reg<store_ff_status::STORE_FF_STATUS_SPEC>;
#[doc = "Store FIFO status"]
pub mod store_ff_status;
#[doc = "AES_CTL (rw) register accessor: an alias for `Reg<AES_CTL_SPEC>`"]
pub type AES_CTL = crate::Reg<aes_ctl::AES_CTL_SPEC>;
#[doc = "AES control"]
pub mod aes_ctl;
#[doc = "RESULT (rw) register accessor: an alias for `Reg<RESULT_SPEC>`"]
pub type RESULT = crate::Reg<result::RESULT_SPEC>;
#[doc = "Result"]
pub mod result;
#[doc = "CRC_CTL (rw) register accessor: an alias for `Reg<CRC_CTL_SPEC>`"]
pub type CRC_CTL = crate::Reg<crc_ctl::CRC_CTL_SPEC>;
#[doc = "CRC control"]
pub mod crc_ctl;
#[doc = "CRC_DATA_CTL (rw) register accessor: an alias for `Reg<CRC_DATA_CTL_SPEC>`"]
pub type CRC_DATA_CTL = crate::Reg<crc_data_ctl::CRC_DATA_CTL_SPEC>;
#[doc = "CRC data control"]
pub mod crc_data_ctl;
#[doc = "CRC_POL_CTL (rw) register accessor: an alias for `Reg<CRC_POL_CTL_SPEC>`"]
pub type CRC_POL_CTL = crate::Reg<crc_pol_ctl::CRC_POL_CTL_SPEC>;
#[doc = "CRC polynomial control"]
pub mod crc_pol_ctl;
#[doc = "CRC_REM_CTL (rw) register accessor: an alias for `Reg<CRC_REM_CTL_SPEC>`"]
pub type CRC_REM_CTL = crate::Reg<crc_rem_ctl::CRC_REM_CTL_SPEC>;
#[doc = "CRC remainder control"]
pub mod crc_rem_ctl;
#[doc = "CRC_REM_RESULT (r) register accessor: an alias for `Reg<CRC_REM_RESULT_SPEC>`"]
pub type CRC_REM_RESULT = crate::Reg<crc_rem_result::CRC_REM_RESULT_SPEC>;
#[doc = "CRC remainder result"]
pub mod crc_rem_result;
#[doc = "VU_CTL0 (rw) register accessor: an alias for `Reg<VU_CTL0_SPEC>`"]
pub type VU_CTL0 = crate::Reg<vu_ctl0::VU_CTL0_SPEC>;
#[doc = "Vector unit control 0"]
pub mod vu_ctl0;
#[doc = "VU_CTL1 (rw) register accessor: an alias for `Reg<VU_CTL1_SPEC>`"]
pub type VU_CTL1 = crate::Reg<vu_ctl1::VU_CTL1_SPEC>;
#[doc = "Vector unit control 1"]
pub mod vu_ctl1;
#[doc = "VU_CTL2 (rw) register accessor: an alias for `Reg<VU_CTL2_SPEC>`"]
pub type VU_CTL2 = crate::Reg<vu_ctl2::VU_CTL2_SPEC>;
#[doc = "Vector unit control 2"]
pub mod vu_ctl2;
#[doc = "VU_STATUS (r) register accessor: an alias for `Reg<VU_STATUS_SPEC>`"]
pub type VU_STATUS = crate::Reg<vu_status::VU_STATUS_SPEC>;
#[doc = "Vector unit status"]
pub mod vu_status;
#[doc = "VU_RF_DATA (r) register accessor: an alias for `Reg<VU_RF_DATA_SPEC>`"]
pub type VU_RF_DATA = crate::Reg<vu_rf_data::VU_RF_DATA_SPEC>;
#[doc = "Vector unit register-file"]
pub mod vu_rf_data;
#[doc = "DEV_KEY_ADDR0_CTL (rw) register accessor: an alias for `Reg<DEV_KEY_ADDR0_CTL_SPEC>`"]
pub type DEV_KEY_ADDR0_CTL = crate::Reg<dev_key_addr0_ctl::DEV_KEY_ADDR0_CTL_SPEC>;
#[doc = "Device key address 0 control"]
pub mod dev_key_addr0_ctl;
#[doc = "DEV_KEY_ADDR0 (rw) register accessor: an alias for `Reg<DEV_KEY_ADDR0_SPEC>`"]
pub type DEV_KEY_ADDR0 = crate::Reg<dev_key_addr0::DEV_KEY_ADDR0_SPEC>;
#[doc = "Device key address 0"]
pub mod dev_key_addr0;
#[doc = "DEV_KEY_ADDR1_CTL (rw) register accessor: an alias for `Reg<DEV_KEY_ADDR1_CTL_SPEC>`"]
pub type DEV_KEY_ADDR1_CTL = crate::Reg<dev_key_addr1_ctl::DEV_KEY_ADDR1_CTL_SPEC>;
#[doc = "Device key address 1 control"]
pub mod dev_key_addr1_ctl;
#[doc = "DEV_KEY_ADDR1 (rw) register accessor: an alias for `Reg<DEV_KEY_ADDR1_SPEC>`"]
pub type DEV_KEY_ADDR1 = crate::Reg<dev_key_addr1::DEV_KEY_ADDR1_SPEC>;
#[doc = "Device key address 1 control"]
pub mod dev_key_addr1;
#[doc = "DEV_KEY_STATUS (r) register accessor: an alias for `Reg<DEV_KEY_STATUS_SPEC>`"]
pub type DEV_KEY_STATUS = crate::Reg<dev_key_status::DEV_KEY_STATUS_SPEC>;
#[doc = "Device key status"]
pub mod dev_key_status;
#[doc = "DEV_KEY_CTL0 (rw) register accessor: an alias for `Reg<DEV_KEY_CTL0_SPEC>`"]
pub type DEV_KEY_CTL0 = crate::Reg<dev_key_ctl0::DEV_KEY_CTL0_SPEC>;
#[doc = "Device key control 0"]
pub mod dev_key_ctl0;
#[doc = "DEV_KEY_CTL1 (rw) register accessor: an alias for `Reg<DEV_KEY_CTL1_SPEC>`"]
pub type DEV_KEY_CTL1 = crate::Reg<dev_key_ctl1::DEV_KEY_CTL1_SPEC>;
#[doc = "Device key control 1"]
pub mod dev_key_ctl1;
