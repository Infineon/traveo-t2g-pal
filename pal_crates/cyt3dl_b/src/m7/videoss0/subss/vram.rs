#[doc = r"Register block"]
#[repr(C)]
pub struct VRAM {
    #[doc = "0x00 - N/A"]
    pub arbiter_priority: ARBITER_PRIORITY,
}
#[doc = "ARBITER_PRIORITY (rw) register accessor: an alias for `Reg<ARBITER_PRIORITY_SPEC>`"]
pub type ARBITER_PRIORITY = crate::Reg<arbiter_priority::ARBITER_PRIORITY_SPEC>;
#[doc = "N/A"]
pub mod arbiter_priority;
