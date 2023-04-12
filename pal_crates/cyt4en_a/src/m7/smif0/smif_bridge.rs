#[doc = r"Register block"]
#[repr(C)]
pub struct SMIF_BRIDGE {
    #[doc = "0x00 - Global control registers for the bridge"]
    pub ctl: CTL,
    _reserved1: [u8; 0x0ffc],
    #[doc = "0x1000..0x1020 - 'Remap regions' which define how XIP accesses can be remapped into SMIF physical spaces"]
    pub smif_remap_region0: SMIF_REMAP_REGION,
    _reserved2: [u8; 0xe0],
    #[doc = "0x1100..0x1120 - 'Remap regions' which define how XIP accesses can be remapped into SMIF physical spaces"]
    pub smif_remap_region1: SMIF_REMAP_REGION,
    _reserved3: [u8; 0xe0],
    #[doc = "0x1200..0x1220 - 'Remap regions' which define how XIP accesses can be remapped into SMIF physical spaces"]
    pub smif_remap_region2: SMIF_REMAP_REGION,
    _reserved4: [u8; 0xe0],
    #[doc = "0x1300..0x1320 - 'Remap regions' which define how XIP accesses can be remapped into SMIF physical spaces"]
    pub smif_remap_region3: SMIF_REMAP_REGION,
    _reserved5: [u8; 0xe0],
    #[doc = "0x1400..0x1420 - 'Remap regions' which define how XIP accesses can be remapped into SMIF physical spaces"]
    pub smif_remap_region4: SMIF_REMAP_REGION,
    _reserved6: [u8; 0xe0],
    #[doc = "0x1500..0x1520 - 'Remap regions' which define how XIP accesses can be remapped into SMIF physical spaces"]
    pub smif_remap_region5: SMIF_REMAP_REGION,
    _reserved7: [u8; 0xe0],
    #[doc = "0x1600..0x1620 - 'Remap regions' which define how XIP accesses can be remapped into SMIF physical spaces"]
    pub smif_remap_region6: SMIF_REMAP_REGION,
    _reserved8: [u8; 0xe0],
    #[doc = "0x1700..0x1720 - 'Remap regions' which define how XIP accesses can be remapped into SMIF physical spaces"]
    pub smif_remap_region7: SMIF_REMAP_REGION,
}
#[doc = "CTL (rw) register accessor: an alias for `Reg<CTL_SPEC>`"]
pub type CTL = crate::Reg<ctl::CTL_SPEC>;
#[doc = "Global control registers for the bridge"]
pub mod ctl;
#[doc = "'Remap regions' which define how XIP accesses can be remapped into SMIF physical spaces"]
pub use self::smif_remap_region::SMIF_REMAP_REGION;
#[doc = r"Cluster"]
#[doc = "'Remap regions' which define how XIP accesses can be remapped into SMIF physical spaces"]
pub mod smif_remap_region;
