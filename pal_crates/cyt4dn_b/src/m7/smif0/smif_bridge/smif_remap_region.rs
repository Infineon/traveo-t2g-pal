#[doc = r"Register block"]
#[repr(C)]
pub struct SMIF_REMAP_REGION {
    #[doc = "0x00 - Control bits for remap region"]
    pub ctl: CTL,
    _reserved1: [u8; 0x0c],
    #[doc = "0x10 - Base address of remap region"]
    pub addr: ADDR,
    #[doc = "0x14 - Mask value to be paired with ADDR"]
    pub mask: MASK,
    #[doc = "0x18 - Base address for remaps into SMIF0 physical memory space"]
    pub smif0_remap: SMIF0_REMAP,
    #[doc = "0x1c - Base address for remaps into SMIF1 physical memory space"]
    pub smif1_remap: SMIF1_REMAP,
}
#[doc = "CTL (rw) register accessor: an alias for `Reg<CTL_SPEC>`"]
pub type CTL = crate::Reg<ctl::CTL_SPEC>;
#[doc = "Control bits for remap region"]
pub mod ctl;
#[doc = "ADDR (rw) register accessor: an alias for `Reg<ADDR_SPEC>`"]
pub type ADDR = crate::Reg<addr::ADDR_SPEC>;
#[doc = "Base address of remap region"]
pub mod addr;
#[doc = "MASK (rw) register accessor: an alias for `Reg<MASK_SPEC>`"]
pub type MASK = crate::Reg<mask::MASK_SPEC>;
#[doc = "Mask value to be paired with ADDR"]
pub mod mask;
#[doc = "SMIF0_REMAP (rw) register accessor: an alias for `Reg<SMIF0_REMAP_SPEC>`"]
pub type SMIF0_REMAP = crate::Reg<smif0_remap::SMIF0_REMAP_SPEC>;
#[doc = "Base address for remaps into SMIF0 physical memory space"]
pub mod smif0_remap;
#[doc = "SMIF1_REMAP (rw) register accessor: an alias for `Reg<SMIF1_REMAP_SPEC>`"]
pub type SMIF1_REMAP = crate::Reg<smif1_remap::SMIF1_REMAP_SPEC>;
#[doc = "Base address for remaps into SMIF1 physical memory space"]
pub mod smif1_remap;
