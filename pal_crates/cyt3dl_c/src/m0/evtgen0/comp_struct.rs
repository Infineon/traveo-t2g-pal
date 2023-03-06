#[doc = r"Register block"]
#[repr(C)]
pub struct COMP_STRUCT {
    #[doc = "0x00 - Comparator control"]
    pub comp_ctl: COMP_CTL,
    #[doc = "0x04 - Comparator 0 (Active functionality)"]
    pub comp0: COMP0,
    #[doc = "0x08 - Comparator 1 (DeepSleep functionality)"]
    pub comp1: COMP1,
}
#[doc = "COMP_CTL (rw) register accessor: an alias for `Reg<COMP_CTL_SPEC>`"]
pub type COMP_CTL = crate::Reg<comp_ctl::COMP_CTL_SPEC>;
#[doc = "Comparator control"]
pub mod comp_ctl;
#[doc = "COMP0 (rw) register accessor: an alias for `Reg<COMP0_SPEC>`"]
pub type COMP0 = crate::Reg<comp0::COMP0_SPEC>;
#[doc = "Comparator 0 (Active functionality)"]
pub mod comp0;
#[doc = "COMP1 (rw) register accessor: an alias for `Reg<COMP1_SPEC>`"]
pub type COMP1 = crate::Reg<comp1::COMP1_SPEC>;
#[doc = "Comparator 1 (DeepSleep functionality)"]
pub mod comp1;
