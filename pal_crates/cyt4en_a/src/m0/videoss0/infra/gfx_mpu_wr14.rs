#[doc = r"Register block"]
#[repr(C)]
pub struct GFX_MPU_WR14 {
    #[doc = "0x00 - Memory protection context for AXI write requests."]
    pub wr_ctl: WR_CTL,
}
#[doc = "WR_CTL (rw) register accessor: an alias for `Reg<WR_CTL_SPEC>`"]
pub type WR_CTL = crate::Reg<wr_ctl::WR_CTL_SPEC>;
#[doc = "Memory protection context for AXI write requests."]
pub mod wr_ctl;
