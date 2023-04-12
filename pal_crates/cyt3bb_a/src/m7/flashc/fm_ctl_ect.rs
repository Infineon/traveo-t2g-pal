#[doc = r"Register block"]
#[repr(C)]
pub struct FM_CTL_ECT {
    #[doc = "0x00 - Flash Macro Control"]
    pub fm_ctl: FM_CTL,
    #[doc = "0x04 - Flash Macro Margin Mode on Code Flash"]
    pub fm_code_margin: FM_CODE_MARGIN,
    #[doc = "0x08 - Flash Macro Address"]
    pub fm_addr: FM_ADDR,
    #[doc = "0x0c - Flash Density Information"]
    pub geomtry: GEOMTRY,
    _reserved4: [u8; 0x10],
    #[doc = "0x20 - Interrupt"]
    pub intr: INTR,
    #[doc = "0x24 - Interrupt Set"]
    pub intr_set: INTR_SET,
    #[doc = "0x28 - Interrupt Mask"]
    pub intr_mask: INTR_MASK,
    #[doc = "0x2c - Interrupt Masked"]
    pub intr_masked: INTR_MASKED,
    #[doc = "0x30 - ECC Data In override information and control bits"]
    pub ecc_override: ECC_OVERRIDE,
    _reserved9: [u8; 0x0c],
    #[doc = "0x40 - Flash macro data_in\\[31 to 0\\]
both Code and Work Flash"]
    pub fm_data: FM_DATA,
    _reserved10: [u8; 0x20],
    #[doc = "0x64 - Bookmark register - keeps the current FW HV seq"]
    pub bookmark: BOOKMARK,
    _reserved11: [u8; 0x0398],
    #[doc = "0x400 - Main (Code) Flash Security enable"]
    pub main_flash_safety: MAIN_FLASH_SAFETY,
    #[doc = "0x404 - Status read from Flash Macro"]
    pub status: STATUS,
    _reserved13: [u8; 0xf8],
    #[doc = "0x500 - Work Flash Security enable"]
    pub work_flash_safety: WORK_FLASH_SAFETY,
}
#[doc = "FM_CTL (rw) register accessor: an alias for `Reg<FM_CTL_SPEC>`"]
pub type FM_CTL = crate::Reg<fm_ctl::FM_CTL_SPEC>;
#[doc = "Flash Macro Control"]
pub mod fm_ctl;
#[doc = "FM_CODE_MARGIN (rw) register accessor: an alias for `Reg<FM_CODE_MARGIN_SPEC>`"]
pub type FM_CODE_MARGIN = crate::Reg<fm_code_margin::FM_CODE_MARGIN_SPEC>;
#[doc = "Flash Macro Margin Mode on Code Flash"]
pub mod fm_code_margin;
#[doc = "FM_ADDR (w) register accessor: an alias for `Reg<FM_ADDR_SPEC>`"]
pub type FM_ADDR = crate::Reg<fm_addr::FM_ADDR_SPEC>;
#[doc = "Flash Macro Address"]
pub mod fm_addr;
#[doc = "GEOMTRY (r) register accessor: an alias for `Reg<GEOMTRY_SPEC>`"]
pub type GEOMTRY = crate::Reg<geomtry::GEOMTRY_SPEC>;
#[doc = "Flash Density Information"]
pub mod geomtry;
#[doc = "INTR (rw) register accessor: an alias for `Reg<INTR_SPEC>`"]
pub type INTR = crate::Reg<intr::INTR_SPEC>;
#[doc = "Interrupt"]
pub mod intr;
#[doc = "INTR_SET (rw) register accessor: an alias for `Reg<INTR_SET_SPEC>`"]
pub type INTR_SET = crate::Reg<intr_set::INTR_SET_SPEC>;
#[doc = "Interrupt Set"]
pub mod intr_set;
#[doc = "INTR_MASK (rw) register accessor: an alias for `Reg<INTR_MASK_SPEC>`"]
pub type INTR_MASK = crate::Reg<intr_mask::INTR_MASK_SPEC>;
#[doc = "Interrupt Mask"]
pub mod intr_mask;
#[doc = "INTR_MASKED (r) register accessor: an alias for `Reg<INTR_MASKED_SPEC>`"]
pub type INTR_MASKED = crate::Reg<intr_masked::INTR_MASKED_SPEC>;
#[doc = "Interrupt Masked"]
pub mod intr_masked;
#[doc = "ECC_OVERRIDE (w) register accessor: an alias for `Reg<ECC_OVERRIDE_SPEC>`"]
pub type ECC_OVERRIDE = crate::Reg<ecc_override::ECC_OVERRIDE_SPEC>;
#[doc = "ECC Data In override information and control bits"]
pub mod ecc_override;
#[doc = "FM_DATA (w) register accessor: an alias for `Reg<FM_DATA_SPEC>`"]
pub type FM_DATA = crate::Reg<fm_data::FM_DATA_SPEC>;
#[doc = "Flash macro data_in\\[31 to 0\\]
both Code and Work Flash"]
pub mod fm_data;
#[doc = "BOOKMARK (rw) register accessor: an alias for `Reg<BOOKMARK_SPEC>`"]
pub type BOOKMARK = crate::Reg<bookmark::BOOKMARK_SPEC>;
#[doc = "Bookmark register - keeps the current FW HV seq"]
pub mod bookmark;
#[doc = "MAIN_FLASH_SAFETY (rw) register accessor: an alias for `Reg<MAIN_FLASH_SAFETY_SPEC>`"]
pub type MAIN_FLASH_SAFETY = crate::Reg<main_flash_safety::MAIN_FLASH_SAFETY_SPEC>;
#[doc = "Main (Code) Flash Security enable"]
pub mod main_flash_safety;
#[doc = "STATUS (r) register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Status read from Flash Macro"]
pub mod status;
#[doc = "WORK_FLASH_SAFETY (rw) register accessor: an alias for `Reg<WORK_FLASH_SAFETY_SPEC>`"]
pub type WORK_FLASH_SAFETY = crate::Reg<work_flash_safety::WORK_FLASH_SAFETY_SPEC>;
#[doc = "Work Flash Security enable"]
pub mod work_flash_safety;
