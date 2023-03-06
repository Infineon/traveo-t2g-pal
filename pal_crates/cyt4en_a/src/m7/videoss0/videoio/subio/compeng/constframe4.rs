#[doc = r"Register block"]
#[repr(C)]
pub struct CONSTFRAME4 {
    #[doc = "0x00 - ConstFrame unit static control register"]
    pub staticcontrol: STATICCONTROL,
    #[doc = "0x04 - Output frame dimensions."]
    pub framedimensions: FRAMEDIMENSIONS,
    #[doc = "0x08 - 8-bit constant frame color components."]
    pub constantcolor: CONSTANTCOLOR,
    #[doc = "0x0c - LSBits for 10-bit constant frame color components (optional)."]
    pub constantcolorlsbits: CONSTANTCOLORLSBITS,
    #[doc = "0x10 - ConstFrame unit trigger register"]
    pub controltrigger: CONTROLTRIGGER,
    #[doc = "0x14 - ConstFrame unit start register"]
    pub start: START,
}
#[doc = "STATICCONTROL (rw) register accessor: an alias for `Reg<STATICCONTROL_SPEC>`"]
pub type STATICCONTROL = crate::Reg<staticcontrol::STATICCONTROL_SPEC>;
#[doc = "ConstFrame unit static control register"]
pub mod staticcontrol;
#[doc = "FRAMEDIMENSIONS (rw) register accessor: an alias for `Reg<FRAMEDIMENSIONS_SPEC>`"]
pub type FRAMEDIMENSIONS = crate::Reg<framedimensions::FRAMEDIMENSIONS_SPEC>;
#[doc = "Output frame dimensions."]
pub mod framedimensions;
#[doc = "CONSTANTCOLOR (rw) register accessor: an alias for `Reg<CONSTANTCOLOR_SPEC>`"]
pub type CONSTANTCOLOR = crate::Reg<constantcolor::CONSTANTCOLOR_SPEC>;
#[doc = "8-bit constant frame color components."]
pub mod constantcolor;
#[doc = "CONSTANTCOLORLSBITS (rw) register accessor: an alias for `Reg<CONSTANTCOLORLSBITS_SPEC>`"]
pub type CONSTANTCOLORLSBITS = crate::Reg<constantcolorlsbits::CONSTANTCOLORLSBITS_SPEC>;
#[doc = "LSBits for 10-bit constant frame color components (optional)."]
pub mod constantcolorlsbits;
#[doc = "CONTROLTRIGGER (w) register accessor: an alias for `Reg<CONTROLTRIGGER_SPEC>`"]
pub type CONTROLTRIGGER = crate::Reg<controltrigger::CONTROLTRIGGER_SPEC>;
#[doc = "ConstFrame unit trigger register"]
pub mod controltrigger;
#[doc = "START (w) register accessor: an alias for `Reg<START_SPEC>`"]
pub type START = crate::Reg<start::START_SPEC>;
#[doc = "ConstFrame unit start register"]
pub mod start;
