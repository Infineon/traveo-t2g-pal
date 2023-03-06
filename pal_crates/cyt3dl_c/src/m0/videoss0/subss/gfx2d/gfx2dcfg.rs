#[doc = r"Register block"]
#[repr(C)]
pub struct GFX2DCFG {
    _reserved0: [u8; 0x20],
    #[doc = "0x20 - Interrupt cause register"]
    pub intr_gfx2d: INTR_GFX2D,
    #[doc = "0x24 - Interrupt set register"]
    pub intr_gfx2d_set: INTR_GFX2D_SET,
    #[doc = "0x28 - Interrupt mask register"]
    pub intr_gfx2d_mask: INTR_GFX2D_MASK,
    #[doc = "0x2c - Interrupt masked register"]
    pub intr_gfx2d_masked: INTR_GFX2D_MASKED,
    #[doc = "0x30 - Interrupt cause register for CmdSeq"]
    pub intr_cmdseq: INTR_CMDSEQ,
    #[doc = "0x34 - Interrupt set register"]
    pub intr_cmdseq_set: INTR_CMDSEQ_SET,
    _reserved6: [u8; 0x08],
    #[doc = "0x40..0x80 - General purpose config memory"]
    pub generalpurpose: [GENERALPURPOSE; 16],
}
#[doc = "INTR_GFX2D (rw) register accessor: an alias for `Reg<INTR_GFX2D_SPEC>`"]
pub type INTR_GFX2D = crate::Reg<intr_gfx2d::INTR_GFX2D_SPEC>;
#[doc = "Interrupt cause register"]
pub mod intr_gfx2d;
#[doc = "INTR_GFX2D_SET (w) register accessor: an alias for `Reg<INTR_GFX2D_SET_SPEC>`"]
pub type INTR_GFX2D_SET = crate::Reg<intr_gfx2d_set::INTR_GFX2D_SET_SPEC>;
#[doc = "Interrupt set register"]
pub mod intr_gfx2d_set;
#[doc = "INTR_GFX2D_MASK (rw) register accessor: an alias for `Reg<INTR_GFX2D_MASK_SPEC>`"]
pub type INTR_GFX2D_MASK = crate::Reg<intr_gfx2d_mask::INTR_GFX2D_MASK_SPEC>;
#[doc = "Interrupt mask register"]
pub mod intr_gfx2d_mask;
#[doc = "INTR_GFX2D_MASKED (r) register accessor: an alias for `Reg<INTR_GFX2D_MASKED_SPEC>`"]
pub type INTR_GFX2D_MASKED = crate::Reg<intr_gfx2d_masked::INTR_GFX2D_MASKED_SPEC>;
#[doc = "Interrupt masked register"]
pub mod intr_gfx2d_masked;
#[doc = "INTR_CMDSEQ (rw) register accessor: an alias for `Reg<INTR_CMDSEQ_SPEC>`"]
pub type INTR_CMDSEQ = crate::Reg<intr_cmdseq::INTR_CMDSEQ_SPEC>;
#[doc = "Interrupt cause register for CmdSeq"]
pub mod intr_cmdseq;
#[doc = "INTR_CMDSEQ_SET (w) register accessor: an alias for `Reg<INTR_CMDSEQ_SET_SPEC>`"]
pub type INTR_CMDSEQ_SET = crate::Reg<intr_cmdseq_set::INTR_CMDSEQ_SET_SPEC>;
#[doc = "Interrupt set register"]
pub mod intr_cmdseq_set;
#[doc = "GENERALPURPOSE (rw) register accessor: an alias for `Reg<GENERALPURPOSE_SPEC>`"]
pub type GENERALPURPOSE = crate::Reg<generalpurpose::GENERALPURPOSE_SPEC>;
#[doc = "General purpose config memory"]
pub mod generalpurpose;
