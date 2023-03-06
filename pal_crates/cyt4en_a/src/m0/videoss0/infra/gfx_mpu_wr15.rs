#[doc = r"Register block"]
#[repr(C)]
pub struct GFX_MPU_WR15 {
    #[doc = "0x00 - Memory protection context for AXI write requests (not used)."]
    pub wr_ctl: WR_CTL,
}
#[doc = "WR_CTL (r) register accessor: an alias for `Reg<WR_CTL_SPEC>`"]
pub type WR_CTL = crate::Reg<wr_ctl::WR_CTL_SPEC>;
#[doc = "Memory protection context for AXI write requests (not used)."]
pub mod wr_ctl;
