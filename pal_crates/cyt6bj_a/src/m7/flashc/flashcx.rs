#[doc = r"Register block"]
#[repr(C)]
pub struct FLASHCX {
    #[doc = "0x00 - Control"]
    pub flash_ctl: FLASH_CTL,
    #[doc = "0x04 - Flash power control"]
    pub flash_pwr_ctl: FLASH_PWR_CTL,
    #[doc = "0x08 - Command"]
    pub flash_cmd: FLASH_CMD,
    _reserved3: [u8; 0x0294],
    #[doc = "0x2a0 - ECC control"]
    pub ecc_ctl: ECC_CTL,
    _reserved4: [u8; 0x0c],
    #[doc = "0x2b0 - eCT Flash SRAM ECC control 0"]
    pub fm_sram_ecc_ctl0: FM_SRAM_ECC_CTL0,
    #[doc = "0x2b4 - eCT Flash SRAM ECC control 1"]
    pub fm_sram_ecc_ctl1: FM_SRAM_ECC_CTL1,
    #[doc = "0x2b8 - eCT Flash SRAM ECC control 2"]
    pub fm_sram_ecc_ctl2: FM_SRAM_ECC_CTL2,
    #[doc = "0x2bc - eCT Flash SRAM ECC control 3"]
    pub fm_sram_ecc_ctl3: FM_SRAM_ECC_CTL3,
    _reserved8: [u8; 0x0140],
    #[doc = "0x400 - CM0+ cache control"]
    pub cm0_ca_ctl0: CM0_CA_CTL0,
    #[doc = "0x404 - CM0+ cache control"]
    pub cm0_ca_ctl1: CM0_CA_CTL1,
    #[doc = "0x408 - CM0+ cache control"]
    pub cm0_ca_ctl2: CM0_CA_CTL2,
    _reserved11: [u8; 0x34],
    #[doc = "0x440 - CM0+ cache status 0"]
    pub cm0_ca_status0: CM0_CA_STATUS0,
    #[doc = "0x444 - CM0+ cache status 1"]
    pub cm0_ca_status1: CM0_CA_STATUS1,
    #[doc = "0x448 - CM0+ cache status 2"]
    pub cm0_ca_status2: CM0_CA_STATUS2,
    _reserved14: [u8; 0x14],
    #[doc = "0x460 - CM0+ interface status"]
    pub cm0_status: CM0_STATUS,
    _reserved15: [u8; 0x7c],
    #[doc = "0x4e0 - CM7 #0 interface status"]
    pub cm7_0_status: CM7_0_STATUS,
    _reserved16: [u8; 0x7c],
    #[doc = "0x560 - CM7 #1 interface status"]
    pub cm7_1_status: CM7_1_STATUS,
    #[doc = "0x564 - CM7 #2 interface status"]
    pub cm7_2_status: CM7_2_STATUS,
    #[doc = "0x568 - CM7 #3 interface status"]
    pub cm7_3_status: CM7_3_STATUS,
    _reserved19: [u8; 0x14],
    #[doc = "0x580 - Cryptography buffer control"]
    pub crypto_buff_ctl: CRYPTO_BUFF_CTL,
    _reserved20: [u8; 0x7c],
    #[doc = "0x600 - Datawire 0 buffer control"]
    pub dw0_buff_ctl: DW0_BUFF_CTL,
    _reserved21: [u8; 0x7c],
    #[doc = "0x680 - Datawire 1 buffer control"]
    pub dw1_buff_ctl: DW1_BUFF_CTL,
    _reserved22: [u8; 0x7c],
    #[doc = "0x700 - DMA controller buffer control"]
    pub dmac_buff_ctl: DMAC_BUFF_CTL,
    _reserved23: [u8; 0x7c],
    #[doc = "0x780 - Slow external master 0 buffer control"]
    pub slow0_ms_buff_ctl: SLOW0_MS_BUFF_CTL,
    _reserved24: [u8; 0x7c],
    #[doc = "0x800 - Slow external master 1 buffer control"]
    pub slow1_ms_buff_ctl: SLOW1_MS_BUFF_CTL,
    _reserved25: [u8; 0xe7fc],
    #[doc = "0xf000..0xf504 - Flash Macro Registers"]
    pub fm_ctl_ect: FM_CTL_ECT,
}
#[doc = "FLASH_CTL (rw) register accessor: an alias for `Reg<FLASH_CTL_SPEC>`"]
pub type FLASH_CTL = crate::Reg<flash_ctl::FLASH_CTL_SPEC>;
#[doc = "Control"]
pub mod flash_ctl;
#[doc = "FLASH_PWR_CTL (rw) register accessor: an alias for `Reg<FLASH_PWR_CTL_SPEC>`"]
pub type FLASH_PWR_CTL = crate::Reg<flash_pwr_ctl::FLASH_PWR_CTL_SPEC>;
#[doc = "Flash power control"]
pub mod flash_pwr_ctl;
#[doc = "FLASH_CMD (rw) register accessor: an alias for `Reg<FLASH_CMD_SPEC>`"]
pub type FLASH_CMD = crate::Reg<flash_cmd::FLASH_CMD_SPEC>;
#[doc = "Command"]
pub mod flash_cmd;
#[doc = "ECC_CTL (rw) register accessor: an alias for `Reg<ECC_CTL_SPEC>`"]
pub type ECC_CTL = crate::Reg<ecc_ctl::ECC_CTL_SPEC>;
#[doc = "ECC control"]
pub mod ecc_ctl;
#[doc = "FM_SRAM_ECC_CTL0 (rw) register accessor: an alias for `Reg<FM_SRAM_ECC_CTL0_SPEC>`"]
pub type FM_SRAM_ECC_CTL0 = crate::Reg<fm_sram_ecc_ctl0::FM_SRAM_ECC_CTL0_SPEC>;
#[doc = "eCT Flash SRAM ECC control 0"]
pub mod fm_sram_ecc_ctl0;
#[doc = "FM_SRAM_ECC_CTL1 (rw) register accessor: an alias for `Reg<FM_SRAM_ECC_CTL1_SPEC>`"]
pub type FM_SRAM_ECC_CTL1 = crate::Reg<fm_sram_ecc_ctl1::FM_SRAM_ECC_CTL1_SPEC>;
#[doc = "eCT Flash SRAM ECC control 1"]
pub mod fm_sram_ecc_ctl1;
#[doc = "FM_SRAM_ECC_CTL2 (r) register accessor: an alias for `Reg<FM_SRAM_ECC_CTL2_SPEC>`"]
pub type FM_SRAM_ECC_CTL2 = crate::Reg<fm_sram_ecc_ctl2::FM_SRAM_ECC_CTL2_SPEC>;
#[doc = "eCT Flash SRAM ECC control 2"]
pub mod fm_sram_ecc_ctl2;
#[doc = "FM_SRAM_ECC_CTL3 (rw) register accessor: an alias for `Reg<FM_SRAM_ECC_CTL3_SPEC>`"]
pub type FM_SRAM_ECC_CTL3 = crate::Reg<fm_sram_ecc_ctl3::FM_SRAM_ECC_CTL3_SPEC>;
#[doc = "eCT Flash SRAM ECC control 3"]
pub mod fm_sram_ecc_ctl3;
#[doc = "CM0_CA_CTL0 (rw) register accessor: an alias for `Reg<CM0_CA_CTL0_SPEC>`"]
pub type CM0_CA_CTL0 = crate::Reg<cm0_ca_ctl0::CM0_CA_CTL0_SPEC>;
#[doc = "CM0+ cache control"]
pub mod cm0_ca_ctl0;
#[doc = "CM0_CA_CTL1 (rw) register accessor: an alias for `Reg<CM0_CA_CTL1_SPEC>`"]
pub type CM0_CA_CTL1 = crate::Reg<cm0_ca_ctl1::CM0_CA_CTL1_SPEC>;
#[doc = "CM0+ cache control"]
pub mod cm0_ca_ctl1;
#[doc = "CM0_CA_CTL2 (rw) register accessor: an alias for `Reg<CM0_CA_CTL2_SPEC>`"]
pub type CM0_CA_CTL2 = crate::Reg<cm0_ca_ctl2::CM0_CA_CTL2_SPEC>;
#[doc = "CM0+ cache control"]
pub mod cm0_ca_ctl2;
#[doc = "CM0_CA_STATUS0 (r) register accessor: an alias for `Reg<CM0_CA_STATUS0_SPEC>`"]
pub type CM0_CA_STATUS0 = crate::Reg<cm0_ca_status0::CM0_CA_STATUS0_SPEC>;
#[doc = "CM0+ cache status 0"]
pub mod cm0_ca_status0;
#[doc = "CM0_CA_STATUS1 (r) register accessor: an alias for `Reg<CM0_CA_STATUS1_SPEC>`"]
pub type CM0_CA_STATUS1 = crate::Reg<cm0_ca_status1::CM0_CA_STATUS1_SPEC>;
#[doc = "CM0+ cache status 1"]
pub mod cm0_ca_status1;
#[doc = "CM0_CA_STATUS2 (r) register accessor: an alias for `Reg<CM0_CA_STATUS2_SPEC>`"]
pub type CM0_CA_STATUS2 = crate::Reg<cm0_ca_status2::CM0_CA_STATUS2_SPEC>;
#[doc = "CM0+ cache status 2"]
pub mod cm0_ca_status2;
#[doc = "CM0_STATUS (rw) register accessor: an alias for `Reg<CM0_STATUS_SPEC>`"]
pub type CM0_STATUS = crate::Reg<cm0_status::CM0_STATUS_SPEC>;
#[doc = "CM0+ interface status"]
pub mod cm0_status;
#[doc = "CM7_0_STATUS (rw) register accessor: an alias for `Reg<CM7_0_STATUS_SPEC>`"]
pub type CM7_0_STATUS = crate::Reg<cm7_0_status::CM7_0_STATUS_SPEC>;
#[doc = "CM7 #0 interface status"]
pub mod cm7_0_status;
#[doc = "CM7_1_STATUS (rw) register accessor: an alias for `Reg<CM7_1_STATUS_SPEC>`"]
pub type CM7_1_STATUS = crate::Reg<cm7_1_status::CM7_1_STATUS_SPEC>;
#[doc = "CM7 #1 interface status"]
pub mod cm7_1_status;
#[doc = "CM7_2_STATUS (rw) register accessor: an alias for `Reg<CM7_2_STATUS_SPEC>`"]
pub type CM7_2_STATUS = crate::Reg<cm7_2_status::CM7_2_STATUS_SPEC>;
#[doc = "CM7 #2 interface status"]
pub mod cm7_2_status;
#[doc = "CM7_3_STATUS (rw) register accessor: an alias for `Reg<CM7_3_STATUS_SPEC>`"]
pub type CM7_3_STATUS = crate::Reg<cm7_3_status::CM7_3_STATUS_SPEC>;
#[doc = "CM7 #3 interface status"]
pub mod cm7_3_status;
#[doc = "CRYPTO_BUFF_CTL (rw) register accessor: an alias for `Reg<CRYPTO_BUFF_CTL_SPEC>`"]
pub type CRYPTO_BUFF_CTL = crate::Reg<crypto_buff_ctl::CRYPTO_BUFF_CTL_SPEC>;
#[doc = "Cryptography buffer control"]
pub mod crypto_buff_ctl;
#[doc = "DW0_BUFF_CTL (rw) register accessor: an alias for `Reg<DW0_BUFF_CTL_SPEC>`"]
pub type DW0_BUFF_CTL = crate::Reg<dw0_buff_ctl::DW0_BUFF_CTL_SPEC>;
#[doc = "Datawire 0 buffer control"]
pub mod dw0_buff_ctl;
#[doc = "DW1_BUFF_CTL (rw) register accessor: an alias for `Reg<DW1_BUFF_CTL_SPEC>`"]
pub type DW1_BUFF_CTL = crate::Reg<dw1_buff_ctl::DW1_BUFF_CTL_SPEC>;
#[doc = "Datawire 1 buffer control"]
pub mod dw1_buff_ctl;
#[doc = "DMAC_BUFF_CTL (rw) register accessor: an alias for `Reg<DMAC_BUFF_CTL_SPEC>`"]
pub type DMAC_BUFF_CTL = crate::Reg<dmac_buff_ctl::DMAC_BUFF_CTL_SPEC>;
#[doc = "DMA controller buffer control"]
pub mod dmac_buff_ctl;
#[doc = "SLOW0_MS_BUFF_CTL (rw) register accessor: an alias for `Reg<SLOW0_MS_BUFF_CTL_SPEC>`"]
pub type SLOW0_MS_BUFF_CTL = crate::Reg<slow0_ms_buff_ctl::SLOW0_MS_BUFF_CTL_SPEC>;
#[doc = "Slow external master 0 buffer control"]
pub mod slow0_ms_buff_ctl;
#[doc = "SLOW1_MS_BUFF_CTL (rw) register accessor: an alias for `Reg<SLOW1_MS_BUFF_CTL_SPEC>`"]
pub type SLOW1_MS_BUFF_CTL = crate::Reg<slow1_ms_buff_ctl::SLOW1_MS_BUFF_CTL_SPEC>;
#[doc = "Slow external master 1 buffer control"]
pub mod slow1_ms_buff_ctl;
#[doc = "Flash Macro Registers"]
pub use self::fm_ctl_ect::FM_CTL_ECT;
#[doc = r"Cluster"]
#[doc = "Flash Macro Registers"]
pub mod fm_ctl_ect;
