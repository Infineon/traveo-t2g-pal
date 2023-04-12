#[doc = r"Register block"]
#[repr(C)]
pub struct LBCBARVIDEO0 {
    #[doc = "0x00 - Static control settings."]
    pub static_ctl: STATIC_CTL,
    _reserved1: [u8; 0x0c],
    #[doc = "0x10 - Configuration for static arbitration mode."]
    pub static_mode_cfg: STATIC_MODE_CFG,
    _reserved2: [u8; 0x0c],
    #[doc = "0x20 - Common status information."]
    pub status: STATUS,
}
#[doc = "STATIC_CTL (rw) register accessor: an alias for `Reg<STATIC_CTL_SPEC>`"]
pub type STATIC_CTL = crate::Reg<static_ctl::STATIC_CTL_SPEC>;
#[doc = "Static control settings."]
pub mod static_ctl;
#[doc = "STATIC_MODE_CFG (rw) register accessor: an alias for `Reg<STATIC_MODE_CFG_SPEC>`"]
pub type STATIC_MODE_CFG = crate::Reg<static_mode_cfg::STATIC_MODE_CFG_SPEC>;
#[doc = "Configuration for static arbitration mode."]
pub mod static_mode_cfg;
#[doc = "STATUS (r) register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Common status information."]
pub mod status;
