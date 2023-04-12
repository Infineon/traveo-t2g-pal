#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control"]
    pub ctl: CTL,
    #[doc = "0x04 - Enabled channels"]
    pub status: STATUS,
    #[doc = "0x08 - Active secure channels"]
    pub active_sec: ACTIVE_SEC,
    #[doc = "0x0c - Active non-secure channels"]
    pub active_nonsec: ACTIVE_NONSEC,
    _reserved4: [u8; 0x0ff0],
    #[doc = "0x1000..0x1090 - AXI DMA controller channel"]
    pub ch0: CH,
    _reserved5: [u8; 0x70],
    #[doc = "0x1100..0x1190 - AXI DMA controller channel"]
    pub ch1: CH,
    _reserved6: [u8; 0x70],
    #[doc = "0x1200..0x1290 - AXI DMA controller channel"]
    pub ch2: CH,
    _reserved7: [u8; 0x70],
    #[doc = "0x1300..0x1390 - AXI DMA controller channel"]
    pub ch3: CH,
}
#[doc = "CTL (rw) register accessor: an alias for `Reg<CTL_SPEC>`"]
pub type CTL = crate::Reg<ctl::CTL_SPEC>;
#[doc = "Control"]
pub mod ctl;
#[doc = "STATUS (r) register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Enabled channels"]
pub mod status;
#[doc = "ACTIVE_SEC (r) register accessor: an alias for `Reg<ACTIVE_SEC_SPEC>`"]
pub type ACTIVE_SEC = crate::Reg<active_sec::ACTIVE_SEC_SPEC>;
#[doc = "Active secure channels"]
pub mod active_sec;
#[doc = "ACTIVE_NONSEC (r) register accessor: an alias for `Reg<ACTIVE_NONSEC_SPEC>`"]
pub type ACTIVE_NONSEC = crate::Reg<active_nonsec::ACTIVE_NONSEC_SPEC>;
#[doc = "Active non-secure channels"]
pub mod active_nonsec;
#[doc = "AXI DMA controller channel"]
pub use self::ch::CH;
#[doc = r"Cluster"]
#[doc = "AXI DMA controller channel"]
pub mod ch;
