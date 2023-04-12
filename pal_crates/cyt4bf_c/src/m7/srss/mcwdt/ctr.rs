#[doc = r"Register block"]
#[repr(C)]
pub struct CTR {
    #[doc = "0x00 - MCWDT Subcounter Control Register"]
    pub ctl: CTL,
    #[doc = "0x04 - MCWDT Subcounter Lower Limit Register"]
    pub lower_limit: LOWER_LIMIT,
    #[doc = "0x08 - MCWDT Subcounter Upper Limit Register"]
    pub upper_limit: UPPER_LIMIT,
    #[doc = "0x0c - MCWDT Subcounter Warn Limit Register"]
    pub warn_limit: WARN_LIMIT,
    #[doc = "0x10 - MCWDT Subcounter Configuration Register"]
    pub config: CONFIG,
    #[doc = "0x14 - MCWDT Subcounter Count Register"]
    pub cnt: CNT,
}
#[doc = "CTL (rw) register accessor: an alias for `Reg<CTL_SPEC>`"]
pub type CTL = crate::Reg<ctl::CTL_SPEC>;
#[doc = "MCWDT Subcounter Control Register"]
pub mod ctl;
#[doc = "LOWER_LIMIT (rw) register accessor: an alias for `Reg<LOWER_LIMIT_SPEC>`"]
pub type LOWER_LIMIT = crate::Reg<lower_limit::LOWER_LIMIT_SPEC>;
#[doc = "MCWDT Subcounter Lower Limit Register"]
pub mod lower_limit;
#[doc = "UPPER_LIMIT (rw) register accessor: an alias for `Reg<UPPER_LIMIT_SPEC>`"]
pub type UPPER_LIMIT = crate::Reg<upper_limit::UPPER_LIMIT_SPEC>;
#[doc = "MCWDT Subcounter Upper Limit Register"]
pub mod upper_limit;
#[doc = "WARN_LIMIT (rw) register accessor: an alias for `Reg<WARN_LIMIT_SPEC>`"]
pub type WARN_LIMIT = crate::Reg<warn_limit::WARN_LIMIT_SPEC>;
#[doc = "MCWDT Subcounter Warn Limit Register"]
pub mod warn_limit;
#[doc = "CONFIG (rw) register accessor: an alias for `Reg<CONFIG_SPEC>`"]
pub type CONFIG = crate::Reg<config::CONFIG_SPEC>;
#[doc = "MCWDT Subcounter Configuration Register"]
pub mod config;
#[doc = "CNT (rw) register accessor: an alias for `Reg<CNT_SPEC>`"]
pub type CNT = crate::Reg<cnt::CNT_SPEC>;
#[doc = "MCWDT Subcounter Count Register"]
pub mod cnt;
