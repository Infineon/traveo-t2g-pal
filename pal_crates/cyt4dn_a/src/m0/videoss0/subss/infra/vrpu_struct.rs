#[doc = r"Register block"]
#[repr(C)]
pub struct VRPU_STRUCT {
    #[doc = "0x00 - ADDR0"]
    pub addr0: ADDR0,
    #[doc = "0x04 - ATT0"]
    pub att0: ATT0,
    #[doc = "0x08 - ADDR1"]
    pub addr1: ADDR1,
    #[doc = "0x0c - ATT1"]
    pub att1: ATT1,
}
#[doc = "ADDR0 (rw) register accessor: an alias for `Reg<ADDR0_SPEC>`"]
pub type ADDR0 = crate::Reg<addr0::ADDR0_SPEC>;
#[doc = "ADDR0"]
pub mod addr0;
#[doc = "ATT0 (rw) register accessor: an alias for `Reg<ATT0_SPEC>`"]
pub type ATT0 = crate::Reg<att0::ATT0_SPEC>;
#[doc = "ATT0"]
pub mod att0;
#[doc = "ADDR1 (r) register accessor: an alias for `Reg<ADDR1_SPEC>`"]
pub type ADDR1 = crate::Reg<addr1::ADDR1_SPEC>;
#[doc = "ADDR1"]
pub mod addr1;
#[doc = "ATT1 (rw) register accessor: an alias for `Reg<ATT1_SPEC>`"]
pub type ATT1 = crate::Reg<att1::ATT1_SPEC>;
#[doc = "ATT1"]
pub mod att1;
