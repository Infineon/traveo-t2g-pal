#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control"]
    pub ctl: CTL,
    #[doc = "0x04 - Control clear"]
    pub ctl_clr: CTL_CLR,
    #[doc = "0x08 - Control set"]
    pub ctl_set: CTL_SET,
    _reserved3: [u8; 0x7ff4],
    #[doc = "0x8000..0x80d0 - PWM structure"]
    pub tx0: TX,
    _reserved4: [u8; 0x30],
    #[doc = "0x8100..0x81d0 - PWM structure"]
    pub tx1: TX,
}
#[doc = "CTL (rw) register accessor: an alias for `Reg<CTL_SPEC>`"]
pub type CTL = crate::Reg<ctl::CTL_SPEC>;
#[doc = "Control"]
pub mod ctl;
#[doc = "CTL_CLR (rw) register accessor: an alias for `Reg<CTL_CLR_SPEC>`"]
pub type CTL_CLR = crate::Reg<ctl_clr::CTL_CLR_SPEC>;
#[doc = "Control clear"]
pub mod ctl_clr;
#[doc = "CTL_SET (rw) register accessor: an alias for `Reg<CTL_SET_SPEC>`"]
pub type CTL_SET = crate::Reg<ctl_set::CTL_SET_SPEC>;
#[doc = "Control set"]
pub mod ctl_set;
#[doc = "PWM structure"]
pub use self::tx::TX;
#[doc = r"Cluster"]
#[doc = "PWM structure"]
pub mod tx;
