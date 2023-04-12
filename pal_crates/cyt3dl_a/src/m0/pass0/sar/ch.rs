#[doc = r"Register block"]
#[repr(C)]
pub struct CH {
    #[doc = "0x00 - Trigger control."]
    pub tr_ctl: TR_CTL,
    #[doc = "0x04 - Sample control."]
    pub sample_ctl: SAMPLE_CTL,
    #[doc = "0x08 - Post processing control"]
    pub post_ctl: POST_CTL,
    #[doc = "0x0c - Range thresholds"]
    pub range_ctl: RANGE_CTL,
    #[doc = "0x10 - Interrupt request register."]
    pub intr: INTR,
    #[doc = "0x14 - Interrupt set request register"]
    pub intr_set: INTR_SET,
    #[doc = "0x18 - Interrupt mask register."]
    pub intr_mask: INTR_MASK,
    #[doc = "0x1c - Interrupt masked request register"]
    pub intr_masked: INTR_MASKED,
    #[doc = "0x20 - Working data register"]
    pub work: WORK,
    #[doc = "0x24 - Result data register"]
    pub result: RESULT,
    #[doc = "0x28 - Group status register"]
    pub grp_stat: GRP_STAT,
    _reserved11: [u8; 0x0c],
    #[doc = "0x38 - Enable register"]
    pub enable: ENABLE,
    #[doc = "0x3c - Software triggers"]
    pub tr_cmd: TR_CMD,
}
#[doc = "TR_CTL (rw) register accessor: an alias for `Reg<TR_CTL_SPEC>`"]
pub type TR_CTL = crate::Reg<tr_ctl::TR_CTL_SPEC>;
#[doc = "Trigger control."]
pub mod tr_ctl;
#[doc = "SAMPLE_CTL (rw) register accessor: an alias for `Reg<SAMPLE_CTL_SPEC>`"]
pub type SAMPLE_CTL = crate::Reg<sample_ctl::SAMPLE_CTL_SPEC>;
#[doc = "Sample control."]
pub mod sample_ctl;
#[doc = "POST_CTL (rw) register accessor: an alias for `Reg<POST_CTL_SPEC>`"]
pub type POST_CTL = crate::Reg<post_ctl::POST_CTL_SPEC>;
#[doc = "Post processing control"]
pub mod post_ctl;
#[doc = "RANGE_CTL (rw) register accessor: an alias for `Reg<RANGE_CTL_SPEC>`"]
pub type RANGE_CTL = crate::Reg<range_ctl::RANGE_CTL_SPEC>;
#[doc = "Range thresholds"]
pub mod range_ctl;
#[doc = "INTR (rw) register accessor: an alias for `Reg<INTR_SPEC>`"]
pub type INTR = crate::Reg<intr::INTR_SPEC>;
#[doc = "Interrupt request register."]
pub mod intr;
#[doc = "INTR_SET (rw) register accessor: an alias for `Reg<INTR_SET_SPEC>`"]
pub type INTR_SET = crate::Reg<intr_set::INTR_SET_SPEC>;
#[doc = "Interrupt set request register"]
pub mod intr_set;
#[doc = "INTR_MASK (rw) register accessor: an alias for `Reg<INTR_MASK_SPEC>`"]
pub type INTR_MASK = crate::Reg<intr_mask::INTR_MASK_SPEC>;
#[doc = "Interrupt mask register."]
pub mod intr_mask;
#[doc = "INTR_MASKED (r) register accessor: an alias for `Reg<INTR_MASKED_SPEC>`"]
pub type INTR_MASKED = crate::Reg<intr_masked::INTR_MASKED_SPEC>;
#[doc = "Interrupt masked request register"]
pub mod intr_masked;
#[doc = "WORK (r) register accessor: an alias for `Reg<WORK_SPEC>`"]
pub type WORK = crate::Reg<work::WORK_SPEC>;
#[doc = "Working data register"]
pub mod work;
#[doc = "RESULT (r) register accessor: an alias for `Reg<RESULT_SPEC>`"]
pub type RESULT = crate::Reg<result::RESULT_SPEC>;
#[doc = "Result data register"]
pub mod result;
#[doc = "GRP_STAT (r) register accessor: an alias for `Reg<GRP_STAT_SPEC>`"]
pub type GRP_STAT = crate::Reg<grp_stat::GRP_STAT_SPEC>;
#[doc = "Group status register"]
pub mod grp_stat;
#[doc = "ENABLE (rw) register accessor: an alias for `Reg<ENABLE_SPEC>`"]
pub type ENABLE = crate::Reg<enable::ENABLE_SPEC>;
#[doc = "Enable register"]
pub mod enable;
#[doc = "TR_CMD (rw) register accessor: an alias for `Reg<TR_CMD_SPEC>`"]
pub type TR_CMD = crate::Reg<tr_cmd::TR_CMD_SPEC>;
#[doc = "Software triggers"]
pub mod tr_cmd;
