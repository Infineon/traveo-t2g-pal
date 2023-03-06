#[doc = r"Register block"]
#[repr(C)]
pub struct VIDEOIOCFG {
    #[doc = "0x00 - Interrupt status register 0"]
    pub intr_videoio0: INTR_VIDEOIO0,
    #[doc = "0x04 - Interrupt Status register 1"]
    pub intr_videoio1: INTR_VIDEOIO1,
    #[doc = "0x08 - Interrupt set register 0"]
    pub intr_videoio0_set: INTR_VIDEOIO0_SET,
    #[doc = "0x0c - Interrupt set register 1"]
    pub intr_videoio1_set: INTR_VIDEOIO1_SET,
    #[doc = "0x10 - Interrupt mask register 0"]
    pub intr_videoio0_mask: INTR_VIDEOIO0_MASK,
    #[doc = "0x14 - Interrupt mask register 1"]
    pub intr_videoio1_mask: INTR_VIDEOIO1_MASK,
    #[doc = "0x18 - Interrupt masked register 0"]
    pub intr_videoio0_masked: INTR_VIDEOIO0_MASKED,
    #[doc = "0x1c - Interrupt masked register 1"]
    pub intr_videoio1_masked: INTR_VIDEOIO1_MASKED,
    #[doc = "0x20 - Interrupt Status register 0"]
    pub intr_videoio0_safety: INTR_VIDEOIO0_SAFETY,
    #[doc = "0x24 - Interrupt Status register 1"]
    pub intr_videoio1_safety: INTR_VIDEOIO1_SAFETY,
    #[doc = "0x28 - Interrupt set register 0"]
    pub intr_videoio0_safety_set: INTR_VIDEOIO0_SAFETY_SET,
    #[doc = "0x2c - Interrupt set register 1"]
    pub intr_videoio1_safety_set: INTR_VIDEOIO1_SAFETY_SET,
    #[doc = "0x30 - Interrupt mask register 0"]
    pub intr_videoio0_safety_mask: INTR_VIDEOIO0_SAFETY_MASK,
    #[doc = "0x34 - Interrupt mask register 1"]
    pub intr_videoio1_safety_mask: INTR_VIDEOIO1_SAFETY_MASK,
    #[doc = "0x38 - Interrupt masked register 0"]
    pub intr_videoio0_safety_masked: INTR_VIDEOIO0_SAFETY_MASKED,
    #[doc = "0x3c - Interrupt masked register 1"]
    pub intr_videoio1_safety_masked: INTR_VIDEOIO1_SAFETY_MASKED,
}
#[doc = "INTR_VIDEOIO0 (rw) register accessor: an alias for `Reg<INTR_VIDEOIO0_SPEC>`"]
pub type INTR_VIDEOIO0 = crate::Reg<intr_videoio0::INTR_VIDEOIO0_SPEC>;
#[doc = "Interrupt status register 0"]
pub mod intr_videoio0;
#[doc = "INTR_VIDEOIO1 (rw) register accessor: an alias for `Reg<INTR_VIDEOIO1_SPEC>`"]
pub type INTR_VIDEOIO1 = crate::Reg<intr_videoio1::INTR_VIDEOIO1_SPEC>;
#[doc = "Interrupt Status register 1"]
pub mod intr_videoio1;
#[doc = "INTR_VIDEOIO0_SET (w) register accessor: an alias for `Reg<INTR_VIDEOIO0_SET_SPEC>`"]
pub type INTR_VIDEOIO0_SET = crate::Reg<intr_videoio0_set::INTR_VIDEOIO0_SET_SPEC>;
#[doc = "Interrupt set register 0"]
pub mod intr_videoio0_set;
#[doc = "INTR_VIDEOIO1_SET (w) register accessor: an alias for `Reg<INTR_VIDEOIO1_SET_SPEC>`"]
pub type INTR_VIDEOIO1_SET = crate::Reg<intr_videoio1_set::INTR_VIDEOIO1_SET_SPEC>;
#[doc = "Interrupt set register 1"]
pub mod intr_videoio1_set;
#[doc = "INTR_VIDEOIO0_MASK (rw) register accessor: an alias for `Reg<INTR_VIDEOIO0_MASK_SPEC>`"]
pub type INTR_VIDEOIO0_MASK = crate::Reg<intr_videoio0_mask::INTR_VIDEOIO0_MASK_SPEC>;
#[doc = "Interrupt mask register 0"]
pub mod intr_videoio0_mask;
#[doc = "INTR_VIDEOIO1_MASK (rw) register accessor: an alias for `Reg<INTR_VIDEOIO1_MASK_SPEC>`"]
pub type INTR_VIDEOIO1_MASK = crate::Reg<intr_videoio1_mask::INTR_VIDEOIO1_MASK_SPEC>;
#[doc = "Interrupt mask register 1"]
pub mod intr_videoio1_mask;
#[doc = "INTR_VIDEOIO0_MASKED (r) register accessor: an alias for `Reg<INTR_VIDEOIO0_MASKED_SPEC>`"]
pub type INTR_VIDEOIO0_MASKED = crate::Reg<intr_videoio0_masked::INTR_VIDEOIO0_MASKED_SPEC>;
#[doc = "Interrupt masked register 0"]
pub mod intr_videoio0_masked;
#[doc = "INTR_VIDEOIO1_MASKED (r) register accessor: an alias for `Reg<INTR_VIDEOIO1_MASKED_SPEC>`"]
pub type INTR_VIDEOIO1_MASKED = crate::Reg<intr_videoio1_masked::INTR_VIDEOIO1_MASKED_SPEC>;
#[doc = "Interrupt masked register 1"]
pub mod intr_videoio1_masked;
#[doc = "INTR_VIDEOIO0_SAFETY (rw) register accessor: an alias for `Reg<INTR_VIDEOIO0_SAFETY_SPEC>`"]
pub type INTR_VIDEOIO0_SAFETY = crate::Reg<intr_videoio0_safety::INTR_VIDEOIO0_SAFETY_SPEC>;
#[doc = "Interrupt Status register 0"]
pub mod intr_videoio0_safety;
#[doc = "INTR_VIDEOIO1_SAFETY (rw) register accessor: an alias for `Reg<INTR_VIDEOIO1_SAFETY_SPEC>`"]
pub type INTR_VIDEOIO1_SAFETY = crate::Reg<intr_videoio1_safety::INTR_VIDEOIO1_SAFETY_SPEC>;
#[doc = "Interrupt Status register 1"]
pub mod intr_videoio1_safety;
#[doc = "INTR_VIDEOIO0_SAFETY_SET (w) register accessor: an alias for `Reg<INTR_VIDEOIO0_SAFETY_SET_SPEC>`"]
pub type INTR_VIDEOIO0_SAFETY_SET =
    crate::Reg<intr_videoio0_safety_set::INTR_VIDEOIO0_SAFETY_SET_SPEC>;
#[doc = "Interrupt set register 0"]
pub mod intr_videoio0_safety_set;
#[doc = "INTR_VIDEOIO1_SAFETY_SET (w) register accessor: an alias for `Reg<INTR_VIDEOIO1_SAFETY_SET_SPEC>`"]
pub type INTR_VIDEOIO1_SAFETY_SET =
    crate::Reg<intr_videoio1_safety_set::INTR_VIDEOIO1_SAFETY_SET_SPEC>;
#[doc = "Interrupt set register 1"]
pub mod intr_videoio1_safety_set;
#[doc = "INTR_VIDEOIO0_SAFETY_MASK (rw) register accessor: an alias for `Reg<INTR_VIDEOIO0_SAFETY_MASK_SPEC>`"]
pub type INTR_VIDEOIO0_SAFETY_MASK =
    crate::Reg<intr_videoio0_safety_mask::INTR_VIDEOIO0_SAFETY_MASK_SPEC>;
#[doc = "Interrupt mask register 0"]
pub mod intr_videoio0_safety_mask;
#[doc = "INTR_VIDEOIO1_SAFETY_MASK (rw) register accessor: an alias for `Reg<INTR_VIDEOIO1_SAFETY_MASK_SPEC>`"]
pub type INTR_VIDEOIO1_SAFETY_MASK =
    crate::Reg<intr_videoio1_safety_mask::INTR_VIDEOIO1_SAFETY_MASK_SPEC>;
#[doc = "Interrupt mask register 1"]
pub mod intr_videoio1_safety_mask;
#[doc = "INTR_VIDEOIO0_SAFETY_MASKED (r) register accessor: an alias for `Reg<INTR_VIDEOIO0_SAFETY_MASKED_SPEC>`"]
pub type INTR_VIDEOIO0_SAFETY_MASKED =
    crate::Reg<intr_videoio0_safety_masked::INTR_VIDEOIO0_SAFETY_MASKED_SPEC>;
#[doc = "Interrupt masked register 0"]
pub mod intr_videoio0_safety_masked;
#[doc = "INTR_VIDEOIO1_SAFETY_MASKED (r) register accessor: an alias for `Reg<INTR_VIDEOIO1_SAFETY_MASKED_SPEC>`"]
pub type INTR_VIDEOIO1_SAFETY_MASKED =
    crate::Reg<intr_videoio1_safety_masked::INTR_VIDEOIO1_SAFETY_MASKED_SPEC>;
#[doc = "Interrupt masked register 1"]
pub mod intr_videoio1_safety_masked;
