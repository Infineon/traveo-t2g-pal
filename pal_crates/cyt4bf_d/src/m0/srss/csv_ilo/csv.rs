#[doc = r"Register block"]
#[repr(C)]
pub struct CSV {
    #[doc = "0x00 - Clock Supervision Reference Control"]
    pub ref_ctl: REF_CTL,
    #[doc = "0x04 - Clock Supervision Reference Limits"]
    pub ref_limit: REF_LIMIT,
    #[doc = "0x08 - Clock Supervision Monitor Control"]
    pub mon_ctl: MON_CTL,
}
#[doc = "REF_CTL (rw) register accessor: an alias for `Reg<REF_CTL_SPEC>`"]
pub type REF_CTL = crate::Reg<ref_ctl::REF_CTL_SPEC>;
#[doc = "Clock Supervision Reference Control"]
pub mod ref_ctl;
#[doc = "REF_LIMIT (rw) register accessor: an alias for `Reg<REF_LIMIT_SPEC>`"]
pub type REF_LIMIT = crate::Reg<ref_limit::REF_LIMIT_SPEC>;
#[doc = "Clock Supervision Reference Limits"]
pub mod ref_limit;
#[doc = "MON_CTL (rw) register accessor: an alias for `Reg<MON_CTL_SPEC>`"]
pub type MON_CTL = crate::Reg<mon_ctl::MON_CTL_SPEC>;
#[doc = "Clock Supervision Monitor Control"]
pub mod mon_ctl;
