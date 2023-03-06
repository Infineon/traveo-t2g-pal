#[doc = r"Register block"]
#[repr(C)]
pub struct GFX_MPU_RD2 {
    #[doc = "0x00 - Memory protection context for AXI read requests."]
    pub rd_ctl: RD_CTL,
}
#[doc = "RD_CTL (rw) register accessor: an alias for `Reg<RD_CTL_SPEC>`"]
pub type RD_CTL = crate::Reg<rd_ctl::RD_CTL_SPEC>;
#[doc = "Memory protection context for AXI read requests."]
pub mod rd_ctl;
