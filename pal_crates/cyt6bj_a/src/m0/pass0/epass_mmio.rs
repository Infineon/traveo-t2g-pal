#[doc = r"Register block"]
#[repr(C)]
pub struct EPASS_MMIO {
    #[doc = "0x00 - PASS control register"]
    pub pass_ctl: PASS_CTL,
    _reserved1: [u8; 0x1c],
    #[doc = "0x20..0x30 - per SAR generic input trigger select"]
    pub sar_tr_in_sel: [SAR_TR_IN_SEL; 4],
    _reserved2: [u8; 0x10],
    #[doc = "0x40..0x50 - per SAR generic output trigger select"]
    pub sar_tr_out_sel: [SAR_TR_OUT_SEL; 4],
    _reserved3: [u8; 0x30],
    #[doc = "0x80 - Test control bits"]
    pub test_ctl: TEST_CTL,
}
#[doc = "PASS_CTL (rw) register accessor: an alias for `Reg<PASS_CTL_SPEC>`"]
pub type PASS_CTL = crate::Reg<pass_ctl::PASS_CTL_SPEC>;
#[doc = "PASS control register"]
pub mod pass_ctl;
#[doc = "SAR_TR_IN_SEL (rw) register accessor: an alias for `Reg<SAR_TR_IN_SEL_SPEC>`"]
pub type SAR_TR_IN_SEL = crate::Reg<sar_tr_in_sel::SAR_TR_IN_SEL_SPEC>;
#[doc = "per SAR generic input trigger select"]
pub mod sar_tr_in_sel;
#[doc = "SAR_TR_OUT_SEL (rw) register accessor: an alias for `Reg<SAR_TR_OUT_SEL_SPEC>`"]
pub type SAR_TR_OUT_SEL = crate::Reg<sar_tr_out_sel::SAR_TR_OUT_SEL_SPEC>;
#[doc = "per SAR generic output trigger select"]
pub mod sar_tr_out_sel;
#[doc = "TEST_CTL (rw) register accessor: an alias for `Reg<TEST_CTL_SPEC>`"]
pub type TEST_CTL = crate::Reg<test_ctl::TEST_CTL_SPEC>;
#[doc = "Test control bits"]
pub mod test_ctl;
