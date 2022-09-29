#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control"]
    pub ctl: CTL,
    #[doc = "0x04 - Comparator structures comparator 0 status"]
    pub comp0_status: COMP0_STATUS,
    #[doc = "0x08 - Comparator structures comparator 1 status"]
    pub comp1_status: COMP1_STATUS,
    _reserved3: [u8; 0x04],
    #[doc = "0x10 - Counter status"]
    pub counter_status: COUNTER_STATUS,
    #[doc = "0x14 - Counter"]
    pub counter: COUNTER,
    _reserved5: [u8; 0x08],
    #[doc = "0x20 - Ratio control"]
    pub ratio_ctl: RATIO_CTL,
    #[doc = "0x24 - Ratio"]
    pub ratio: RATIO,
    _reserved7: [u8; 0x08],
    #[doc = "0x30 - Reference clock control"]
    pub ref_clock_ctl: REF_CLOCK_CTL,
    _reserved8: [u8; 0x06cc],
    #[doc = "0x700 - Interrupt"]
    pub intr: INTR,
    #[doc = "0x704 - Interrupt set"]
    pub intr_set: INTR_SET,
    #[doc = "0x708 - Interrupt mask"]
    pub intr_mask: INTR_MASK,
    #[doc = "0x70c - Interrupt masked"]
    pub intr_masked: INTR_MASKED,
    #[doc = "0x710 - DeepSleep interrupt"]
    pub intr_dpslp: INTR_DPSLP,
    #[doc = "0x714 - DeepSleep interrupt set"]
    pub intr_dpslp_set: INTR_DPSLP_SET,
    #[doc = "0x718 - DeepSleep interrupt mask"]
    pub intr_dpslp_mask: INTR_DPSLP_MASK,
    #[doc = "0x71c - DeepSleep interrupt masked"]
    pub intr_dpslp_masked: INTR_DPSLP_MASKED,
    _reserved16: [u8; 0xe0],
    #[doc = "0x800..0x80c - Comparator structure"]
    pub comp_struct0: COMP_STRUCT,
    _reserved17: [u8; 0x14],
    #[doc = "0x820..0x82c - Comparator structure"]
    pub comp_struct1: COMP_STRUCT,
    _reserved18: [u8; 0x14],
    #[doc = "0x840..0x84c - Comparator structure"]
    pub comp_struct2: COMP_STRUCT,
    _reserved19: [u8; 0x14],
    #[doc = "0x860..0x86c - Comparator structure"]
    pub comp_struct3: COMP_STRUCT,
    _reserved20: [u8; 0x14],
    #[doc = "0x880..0x88c - Comparator structure"]
    pub comp_struct4: COMP_STRUCT,
    _reserved21: [u8; 0x14],
    #[doc = "0x8a0..0x8ac - Comparator structure"]
    pub comp_struct5: COMP_STRUCT,
    _reserved22: [u8; 0x14],
    #[doc = "0x8c0..0x8cc - Comparator structure"]
    pub comp_struct6: COMP_STRUCT,
    _reserved23: [u8; 0x14],
    #[doc = "0x8e0..0x8ec - Comparator structure"]
    pub comp_struct7: COMP_STRUCT,
    _reserved24: [u8; 0x14],
    #[doc = "0x900..0x90c - Comparator structure"]
    pub comp_struct8: COMP_STRUCT,
    _reserved25: [u8; 0x14],
    #[doc = "0x920..0x92c - Comparator structure"]
    pub comp_struct9: COMP_STRUCT,
    _reserved26: [u8; 0x14],
    #[doc = "0x940..0x94c - Comparator structure"]
    pub comp_struct10: COMP_STRUCT,
}
#[doc = "CTL (rw) register accessor: an alias for `Reg<CTL_SPEC>`"]
pub type CTL = crate::Reg<ctl::CTL_SPEC>;
#[doc = "Control"]
pub mod ctl;
#[doc = "COMP0_STATUS (r) register accessor: an alias for `Reg<COMP0_STATUS_SPEC>`"]
pub type COMP0_STATUS = crate::Reg<comp0_status::COMP0_STATUS_SPEC>;
#[doc = "Comparator structures comparator 0 status"]
pub mod comp0_status;
#[doc = "COMP1_STATUS (r) register accessor: an alias for `Reg<COMP1_STATUS_SPEC>`"]
pub type COMP1_STATUS = crate::Reg<comp1_status::COMP1_STATUS_SPEC>;
#[doc = "Comparator structures comparator 1 status"]
pub mod comp1_status;
#[doc = "COUNTER_STATUS (r) register accessor: an alias for `Reg<COUNTER_STATUS_SPEC>`"]
pub type COUNTER_STATUS = crate::Reg<counter_status::COUNTER_STATUS_SPEC>;
#[doc = "Counter status"]
pub mod counter_status;
#[doc = "COUNTER (r) register accessor: an alias for `Reg<COUNTER_SPEC>`"]
pub type COUNTER = crate::Reg<counter::COUNTER_SPEC>;
#[doc = "Counter"]
pub mod counter;
#[doc = "RATIO_CTL (rw) register accessor: an alias for `Reg<RATIO_CTL_SPEC>`"]
pub type RATIO_CTL = crate::Reg<ratio_ctl::RATIO_CTL_SPEC>;
#[doc = "Ratio control"]
pub mod ratio_ctl;
#[doc = "RATIO (rw) register accessor: an alias for `Reg<RATIO_SPEC>`"]
pub type RATIO = crate::Reg<ratio::RATIO_SPEC>;
#[doc = "Ratio"]
pub mod ratio;
#[doc = "REF_CLOCK_CTL (rw) register accessor: an alias for `Reg<REF_CLOCK_CTL_SPEC>`"]
pub type REF_CLOCK_CTL = crate::Reg<ref_clock_ctl::REF_CLOCK_CTL_SPEC>;
#[doc = "Reference clock control"]
pub mod ref_clock_ctl;
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
#[doc = "INTR_DPSLP (rw) register accessor: an alias for `Reg<INTR_DPSLP_SPEC>`"]
pub type INTR_DPSLP = crate::Reg<intr_dpslp::INTR_DPSLP_SPEC>;
#[doc = "DeepSleep interrupt"]
pub mod intr_dpslp;
#[doc = "INTR_DPSLP_SET (rw) register accessor: an alias for `Reg<INTR_DPSLP_SET_SPEC>`"]
pub type INTR_DPSLP_SET = crate::Reg<intr_dpslp_set::INTR_DPSLP_SET_SPEC>;
#[doc = "DeepSleep interrupt set"]
pub mod intr_dpslp_set;
#[doc = "INTR_DPSLP_MASK (rw) register accessor: an alias for `Reg<INTR_DPSLP_MASK_SPEC>`"]
pub type INTR_DPSLP_MASK = crate::Reg<intr_dpslp_mask::INTR_DPSLP_MASK_SPEC>;
#[doc = "DeepSleep interrupt mask"]
pub mod intr_dpslp_mask;
#[doc = "INTR_DPSLP_MASKED (r) register accessor: an alias for `Reg<INTR_DPSLP_MASKED_SPEC>`"]
pub type INTR_DPSLP_MASKED = crate::Reg<intr_dpslp_masked::INTR_DPSLP_MASKED_SPEC>;
#[doc = "DeepSleep interrupt masked"]
pub mod intr_dpslp_masked;
#[doc = "Comparator structure"]
pub use self::comp_struct::COMP_STRUCT;
#[doc = r"Cluster"]
#[doc = "Comparator structure"]
pub mod comp_struct;
