#[doc = r"Register block"]
#[repr(C)]
pub struct WDT {
    #[doc = "0x00 - WDT Control Register"]
    pub ctl: CTL,
    #[doc = "0x04 - WDT Lower Limit Register"]
    pub lower_limit: LOWER_LIMIT,
    #[doc = "0x08 - WDT Upper Limit Register"]
    pub upper_limit: UPPER_LIMIT,
    #[doc = "0x0c - WDT Warn Limit Register"]
    pub warn_limit: WARN_LIMIT,
    #[doc = "0x10 - WDT Configuration Register"]
    pub config: CONFIG,
    #[doc = "0x14 - WDT Count Register"]
    pub cnt: CNT,
    _reserved6: [u8; 0x28],
    #[doc = "0x40 - WDT Lock register"]
    pub lock: LOCK,
    #[doc = "0x44 - WDT Service register"]
    pub service: SERVICE,
    _reserved8: [u8; 0x08],
    #[doc = "0x50 - WDT Interrupt Register"]
    pub intr: INTR,
    #[doc = "0x54 - WDT Interrupt Set Register"]
    pub intr_set: INTR_SET,
    #[doc = "0x58 - WDT Interrupt Mask Register"]
    pub intr_mask: INTR_MASK,
    #[doc = "0x5c - WDT Interrupt Masked Register"]
    pub intr_masked: INTR_MASKED,
}
#[doc = "CTL (rw) register accessor: an alias for `Reg<CTL_SPEC>`"]
pub type CTL = crate::Reg<ctl::CTL_SPEC>;
#[doc = "WDT Control Register"]
pub mod ctl;
#[doc = "LOWER_LIMIT (rw) register accessor: an alias for `Reg<LOWER_LIMIT_SPEC>`"]
pub type LOWER_LIMIT = crate::Reg<lower_limit::LOWER_LIMIT_SPEC>;
#[doc = "WDT Lower Limit Register"]
pub mod lower_limit;
#[doc = "UPPER_LIMIT (rw) register accessor: an alias for `Reg<UPPER_LIMIT_SPEC>`"]
pub type UPPER_LIMIT = crate::Reg<upper_limit::UPPER_LIMIT_SPEC>;
#[doc = "WDT Upper Limit Register"]
pub mod upper_limit;
#[doc = "WARN_LIMIT (rw) register accessor: an alias for `Reg<WARN_LIMIT_SPEC>`"]
pub type WARN_LIMIT = crate::Reg<warn_limit::WARN_LIMIT_SPEC>;
#[doc = "WDT Warn Limit Register"]
pub mod warn_limit;
#[doc = "CONFIG (rw) register accessor: an alias for `Reg<CONFIG_SPEC>`"]
pub type CONFIG = crate::Reg<config::CONFIG_SPEC>;
#[doc = "WDT Configuration Register"]
pub mod config;
#[doc = "CNT (rw) register accessor: an alias for `Reg<CNT_SPEC>`"]
pub type CNT = crate::Reg<cnt::CNT_SPEC>;
#[doc = "WDT Count Register"]
pub mod cnt;
#[doc = "LOCK (rw) register accessor: an alias for `Reg<LOCK_SPEC>`"]
pub type LOCK = crate::Reg<lock::LOCK_SPEC>;
#[doc = "WDT Lock register"]
pub mod lock;
#[doc = "SERVICE (rw) register accessor: an alias for `Reg<SERVICE_SPEC>`"]
pub type SERVICE = crate::Reg<service::SERVICE_SPEC>;
#[doc = "WDT Service register"]
pub mod service;
#[doc = "INTR (rw) register accessor: an alias for `Reg<INTR_SPEC>`"]
pub type INTR = crate::Reg<intr::INTR_SPEC>;
#[doc = "WDT Interrupt Register"]
pub mod intr;
#[doc = "INTR_SET (rw) register accessor: an alias for `Reg<INTR_SET_SPEC>`"]
pub type INTR_SET = crate::Reg<intr_set::INTR_SET_SPEC>;
#[doc = "WDT Interrupt Set Register"]
pub mod intr_set;
#[doc = "INTR_MASK (rw) register accessor: an alias for `Reg<INTR_MASK_SPEC>`"]
pub type INTR_MASK = crate::Reg<intr_mask::INTR_MASK_SPEC>;
#[doc = "WDT Interrupt Mask Register"]
pub mod intr_mask;
#[doc = "INTR_MASKED (r) register accessor: an alias for `Reg<INTR_MASKED_SPEC>`"]
pub type INTR_MASKED = crate::Reg<intr_masked::INTR_MASKED_SPEC>;
#[doc = "WDT Interrupt Masked Register"]
pub mod intr_masked;
