#[doc = r"Register block"]
#[repr(C)]
pub struct MPC {
    #[doc = "0x00 - Config register with error response, RegionID PPC_MPC_MAIN is the security owner PC. The error response configuration is located in CFG.RESPONSE, only one such configuration exists applying to all protection contexts in the system."]
    pub cfg: CFG,
    _reserved1: [u8; 0x0c],
    #[doc = "0x10 - Interrupt"]
    pub intr: INTR,
    #[doc = "0x14 - Interrupt set"]
    pub intr_set: INTR_SET,
    #[doc = "0x18 - Interrupt mask"]
    pub intr_mask: INTR_MASK,
    #[doc = "0x1c - Interrupt masked"]
    pub intr_masked: INTR_MASKED,
    #[doc = "0x20 - Infor about violation"]
    pub intr_info1: INTR_INFO1,
    #[doc = "0x24 - Infor about violation"]
    pub intr_info2: INTR_INFO2,
    _reserved7: [u8; 0xd8],
    #[doc = "0x100 - Control register with lock bit and auto-increment only (Separate CTRL for each PC depends on access_pc)"]
    pub ctrl: CTRL,
    #[doc = "0x104 - Max value of block-based index register"]
    pub blk_max: BLK_MAX,
    #[doc = "0x108 - Block size &amp; initialization in progress"]
    pub blk_cfg: BLK_CFG,
    #[doc = "0x10c - Index of 32-block group accessed through BLK_LUT (Separate IDX for each PC depending on access_pc)"]
    pub blk_idx: BLK_IDX,
    #[doc = "0x110 - NS status for 32 blocks at BLK_IDX with PC=&lt;access_pc>"]
    pub blk_lut: BLK_LUT,
    _reserved12: [u8; 0xec],
    #[doc = "0x200 - Control register with lock bit and auto-increment only"]
    pub rot_ctrl: ROT_CTRL,
    #[doc = "0x204 - Sets block-size to match memory size (external memory only)"]
    pub rot_cfg: ROT_CFG,
    #[doc = "0x208 - Max value of block-based index register for ROT"]
    pub rot_blk_max: ROT_BLK_MAX,
    #[doc = "0x20c - Same as BLK_CFG"]
    pub rot_blk_cfg: ROT_BLK_CFG,
    #[doc = "0x210 - Index of 8-block group accessed through ROT_BLK_LUT_*"]
    pub rot_blk_idx: ROT_BLK_IDX,
    #[doc = "0x214 - Protection context of 8-block group accesses through ROT_BLK_LUT"]
    pub rot_blk_pc: ROT_BLK_PC,
    #[doc = "0x218 - (R,W,NS) bits for 8 blocks at ROT_BLK_IDX for PC=ROT_BKL_PC"]
    pub rot_blk_lut: ROT_BLK_LUT,
}
#[doc = "CFG (rw) register accessor: an alias for `Reg<CFG_SPEC>`"]
pub type CFG = crate::Reg<cfg::CFG_SPEC>;
#[doc = "Config register with error response, RegionID PPC_MPC_MAIN is the security owner PC. The error response configuration is located in CFG.RESPONSE, only one such configuration exists applying to all protection contexts in the system."]
pub mod cfg;
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
#[doc = "INTR_INFO1 (r) register accessor: an alias for `Reg<INTR_INFO1_SPEC>`"]
pub type INTR_INFO1 = crate::Reg<intr_info1::INTR_INFO1_SPEC>;
#[doc = "Infor about violation"]
pub mod intr_info1;
#[doc = "INTR_INFO2 (r) register accessor: an alias for `Reg<INTR_INFO2_SPEC>`"]
pub type INTR_INFO2 = crate::Reg<intr_info2::INTR_INFO2_SPEC>;
#[doc = "Infor about violation"]
pub mod intr_info2;
#[doc = "CTRL (rw) register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Control register with lock bit and auto-increment only (Separate CTRL for each PC depends on access_pc)"]
pub mod ctrl;
#[doc = "BLK_MAX (r) register accessor: an alias for `Reg<BLK_MAX_SPEC>`"]
pub type BLK_MAX = crate::Reg<blk_max::BLK_MAX_SPEC>;
#[doc = "Max value of block-based index register"]
pub mod blk_max;
#[doc = "BLK_CFG (r) register accessor: an alias for `Reg<BLK_CFG_SPEC>`"]
pub type BLK_CFG = crate::Reg<blk_cfg::BLK_CFG_SPEC>;
#[doc = "Block size &amp; initialization in progress"]
pub mod blk_cfg;
#[doc = "BLK_IDX (rw) register accessor: an alias for `Reg<BLK_IDX_SPEC>`"]
pub type BLK_IDX = crate::Reg<blk_idx::BLK_IDX_SPEC>;
#[doc = "Index of 32-block group accessed through BLK_LUT (Separate IDX for each PC depending on access_pc)"]
pub mod blk_idx;
#[doc = "BLK_LUT (rw) register accessor: an alias for `Reg<BLK_LUT_SPEC>`"]
pub type BLK_LUT = crate::Reg<blk_lut::BLK_LUT_SPEC>;
#[doc = "NS status for 32 blocks at BLK_IDX with PC=&lt;access_pc>"]
pub mod blk_lut;
#[doc = "ROT_CTRL (rw) register accessor: an alias for `Reg<ROT_CTRL_SPEC>`"]
pub type ROT_CTRL = crate::Reg<rot_ctrl::ROT_CTRL_SPEC>;
#[doc = "Control register with lock bit and auto-increment only"]
pub mod rot_ctrl;
#[doc = "ROT_CFG (rw) register accessor: an alias for `Reg<ROT_CFG_SPEC>`"]
pub type ROT_CFG = crate::Reg<rot_cfg::ROT_CFG_SPEC>;
#[doc = "Sets block-size to match memory size (external memory only)"]
pub mod rot_cfg;
#[doc = "ROT_BLK_MAX (r) register accessor: an alias for `Reg<ROT_BLK_MAX_SPEC>`"]
pub type ROT_BLK_MAX = crate::Reg<rot_blk_max::ROT_BLK_MAX_SPEC>;
#[doc = "Max value of block-based index register for ROT"]
pub mod rot_blk_max;
#[doc = "ROT_BLK_CFG (r) register accessor: an alias for `Reg<ROT_BLK_CFG_SPEC>`"]
pub type ROT_BLK_CFG = crate::Reg<rot_blk_cfg::ROT_BLK_CFG_SPEC>;
#[doc = "Same as BLK_CFG"]
pub mod rot_blk_cfg;
#[doc = "ROT_BLK_IDX (rw) register accessor: an alias for `Reg<ROT_BLK_IDX_SPEC>`"]
pub type ROT_BLK_IDX = crate::Reg<rot_blk_idx::ROT_BLK_IDX_SPEC>;
#[doc = "Index of 8-block group accessed through ROT_BLK_LUT_*"]
pub mod rot_blk_idx;
#[doc = "ROT_BLK_PC (rw) register accessor: an alias for `Reg<ROT_BLK_PC_SPEC>`"]
pub type ROT_BLK_PC = crate::Reg<rot_blk_pc::ROT_BLK_PC_SPEC>;
#[doc = "Protection context of 8-block group accesses through ROT_BLK_LUT"]
pub mod rot_blk_pc;
#[doc = "ROT_BLK_LUT (rw) register accessor: an alias for `Reg<ROT_BLK_LUT_SPEC>`"]
pub type ROT_BLK_LUT = crate::Reg<rot_blk_lut::ROT_BLK_LUT_SPEC>;
#[doc = "(R,W,NS) bits for 8 blocks at ROT_BLK_IDX for PC=ROT_BKL_PC"]
pub mod rot_blk_lut;
