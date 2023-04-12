#[doc = r"Register block"]
#[repr(C)]
pub struct MCWDT {
    #[doc = "0x00..0x18 - MCWDT Configuration for Subcounter 0 and 1"]
    pub ctr0: CTR,
    _reserved1: [u8; 0x08],
    #[doc = "0x20..0x38 - MCWDT Configuration for Subcounter 0 and 1"]
    pub ctr1: CTR,
    _reserved2: [u8; 0x08],
    #[doc = "0x40 - MCWDT CPU selection register"]
    pub cpu_select: CPU_SELECT,
    _reserved3: [u8; 0x3c],
    #[doc = "0x80 - MCWDT Subcounter 2 Control register"]
    pub ctr2_ctl: CTR2_CTL,
    #[doc = "0x84 - MCWDT Subcounter 2 Configuration register"]
    pub ctr2_config: CTR2_CONFIG,
    #[doc = "0x88 - MCWDT Subcounter 2 Count Register"]
    pub ctr2_cnt: CTR2_CNT,
    _reserved6: [u8; 0x04],
    #[doc = "0x90 - MCWDT Lock Register"]
    pub lock: LOCK,
    #[doc = "0x94 - MCWDT Service Register"]
    pub service: SERVICE,
    _reserved8: [u8; 0x08],
    #[doc = "0xa0 - MCWDT Interrupt Register"]
    pub intr: INTR,
    #[doc = "0xa4 - MCWDT Interrupt Set Register"]
    pub intr_set: INTR_SET,
    #[doc = "0xa8 - MCWDT Interrupt Mask Register"]
    pub intr_mask: INTR_MASK,
    #[doc = "0xac - MCWDT Interrupt Masked Register"]
    pub intr_masked: INTR_MASKED,
}
#[doc = "MCWDT Configuration for Subcounter 0 and 1"]
pub use self::ctr::CTR;
#[doc = r"Cluster"]
#[doc = "MCWDT Configuration for Subcounter 0 and 1"]
pub mod ctr;
#[doc = "CPU_SELECT (rw) register accessor: an alias for `Reg<CPU_SELECT_SPEC>`"]
pub type CPU_SELECT = crate::Reg<cpu_select::CPU_SELECT_SPEC>;
#[doc = "MCWDT CPU selection register"]
pub mod cpu_select;
#[doc = "CTR2_CTL (rw) register accessor: an alias for `Reg<CTR2_CTL_SPEC>`"]
pub type CTR2_CTL = crate::Reg<ctr2_ctl::CTR2_CTL_SPEC>;
#[doc = "MCWDT Subcounter 2 Control register"]
pub mod ctr2_ctl;
#[doc = "CTR2_CONFIG (rw) register accessor: an alias for `Reg<CTR2_CONFIG_SPEC>`"]
pub type CTR2_CONFIG = crate::Reg<ctr2_config::CTR2_CONFIG_SPEC>;
#[doc = "MCWDT Subcounter 2 Configuration register"]
pub mod ctr2_config;
#[doc = "CTR2_CNT (rw) register accessor: an alias for `Reg<CTR2_CNT_SPEC>`"]
pub type CTR2_CNT = crate::Reg<ctr2_cnt::CTR2_CNT_SPEC>;
#[doc = "MCWDT Subcounter 2 Count Register"]
pub mod ctr2_cnt;
#[doc = "LOCK (rw) register accessor: an alias for `Reg<LOCK_SPEC>`"]
pub type LOCK = crate::Reg<lock::LOCK_SPEC>;
#[doc = "MCWDT Lock Register"]
pub mod lock;
#[doc = "SERVICE (rw) register accessor: an alias for `Reg<SERVICE_SPEC>`"]
pub type SERVICE = crate::Reg<service::SERVICE_SPEC>;
#[doc = "MCWDT Service Register"]
pub mod service;
#[doc = "INTR (rw) register accessor: an alias for `Reg<INTR_SPEC>`"]
pub type INTR = crate::Reg<intr::INTR_SPEC>;
#[doc = "MCWDT Interrupt Register"]
pub mod intr;
#[doc = "INTR_SET (rw) register accessor: an alias for `Reg<INTR_SET_SPEC>`"]
pub type INTR_SET = crate::Reg<intr_set::INTR_SET_SPEC>;
#[doc = "MCWDT Interrupt Set Register"]
pub mod intr_set;
#[doc = "INTR_MASK (rw) register accessor: an alias for `Reg<INTR_MASK_SPEC>`"]
pub type INTR_MASK = crate::Reg<intr_mask::INTR_MASK_SPEC>;
#[doc = "MCWDT Interrupt Mask Register"]
pub mod intr_mask;
#[doc = "INTR_MASKED (r) register accessor: an alias for `Reg<INTR_MASKED_SPEC>`"]
pub type INTR_MASKED = crate::Reg<intr_masked::INTR_MASKED_SPEC>;
#[doc = "MCWDT Interrupt Masked Register"]
pub mod intr_masked;
