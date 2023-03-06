#[doc = r"Register block"]
#[repr(C)]
pub struct EXTSRC4 {
    #[doc = "0x00 - ExtSrc static control register"]
    pub staticcontrol: STATICCONTROL,
    #[doc = "0x04 - Clip window offset, to generate a clipping of the frame. It has to be within the input frame."]
    pub clipwindowoffset: CLIPWINDOWOFFSET,
    #[doc = "0x08 - Define the clip window dimension. If the clip window feature is enabled this dimension is used for the new frame dimension. Note that the clip window has to be smaller or equal to the original frame dimensions. The new frame has to be within the active area of the original frame."]
    pub clipwindowdimension: CLIPWINDOWDIMENSION,
    #[doc = "0x0c - Color component size of raw input data. Please note that the width must be equal or lower than the output width."]
    pub colorcomponentbits: COLORCOMPONENTBITS,
    #[doc = "0x10 - Color component offset of raw input data."]
    pub colorcomponentshift: COLORCOMPONENTSHIFT,
    #[doc = "0x14 - Constant color register"]
    pub constantcolor: CONSTANTCOLOR,
    #[doc = "0x18 - Common control settings."]
    pub control: CONTROL,
    #[doc = "0x1c - ExtSrc unit trigger token generation"]
    pub controltrigger: CONTROLTRIGGER,
    #[doc = "0x20 - ExtSrc unit start register"]
    pub start: START,
}
#[doc = "STATICCONTROL (rw) register accessor: an alias for `Reg<STATICCONTROL_SPEC>`"]
pub type STATICCONTROL = crate::Reg<staticcontrol::STATICCONTROL_SPEC>;
#[doc = "ExtSrc static control register"]
pub mod staticcontrol;
#[doc = "CLIPWINDOWOFFSET (rw) register accessor: an alias for `Reg<CLIPWINDOWOFFSET_SPEC>`"]
pub type CLIPWINDOWOFFSET = crate::Reg<clipwindowoffset::CLIPWINDOWOFFSET_SPEC>;
#[doc = "Clip window offset, to generate a clipping of the frame. It has to be within the input frame."]
pub mod clipwindowoffset;
#[doc = "CLIPWINDOWDIMENSION (rw) register accessor: an alias for `Reg<CLIPWINDOWDIMENSION_SPEC>`"]
pub type CLIPWINDOWDIMENSION = crate::Reg<clipwindowdimension::CLIPWINDOWDIMENSION_SPEC>;
#[doc = "Define the clip window dimension. If the clip window feature is enabled this dimension is used for the new frame dimension. Note that the clip window has to be smaller or equal to the original frame dimensions. The new frame has to be within the active area of the original frame."]
pub mod clipwindowdimension;
#[doc = "COLORCOMPONENTBITS (rw) register accessor: an alias for `Reg<COLORCOMPONENTBITS_SPEC>`"]
pub type COLORCOMPONENTBITS = crate::Reg<colorcomponentbits::COLORCOMPONENTBITS_SPEC>;
#[doc = "Color component size of raw input data. Please note that the width must be equal or lower than the output width."]
pub mod colorcomponentbits;
#[doc = "COLORCOMPONENTSHIFT (rw) register accessor: an alias for `Reg<COLORCOMPONENTSHIFT_SPEC>`"]
pub type COLORCOMPONENTSHIFT = crate::Reg<colorcomponentshift::COLORCOMPONENTSHIFT_SPEC>;
#[doc = "Color component offset of raw input data."]
pub mod colorcomponentshift;
#[doc = "CONSTANTCOLOR (rw) register accessor: an alias for `Reg<CONSTANTCOLOR_SPEC>`"]
pub type CONSTANTCOLOR = crate::Reg<constantcolor::CONSTANTCOLOR_SPEC>;
#[doc = "Constant color register"]
pub mod constantcolor;
#[doc = "CONTROL (rw) register accessor: an alias for `Reg<CONTROL_SPEC>`"]
pub type CONTROL = crate::Reg<control::CONTROL_SPEC>;
#[doc = "Common control settings."]
pub mod control;
#[doc = "CONTROLTRIGGER (w) register accessor: an alias for `Reg<CONTROLTRIGGER_SPEC>`"]
pub type CONTROLTRIGGER = crate::Reg<controltrigger::CONTROLTRIGGER_SPEC>;
#[doc = "ExtSrc unit trigger token generation"]
pub mod controltrigger;
#[doc = "START (w) register accessor: an alias for `Reg<START_SPEC>`"]
pub type START = crate::Reg<start::START_SPEC>;
#[doc = "ExtSrc unit start register"]
pub mod start;
