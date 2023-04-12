#[doc = r"Register block"]
#[repr(C)]
pub struct FETCHECO4 {
    #[doc = "0x00 - Common static control options."]
    pub staticcontrol: STATICCONTROL,
    #[doc = "0x04 - AXI interface buffer management register"]
    pub burstbuffermanagement: BURSTBUFFERMANAGEMENT,
    #[doc = "0x08 - Source buffer base address of layer 0."]
    pub baseaddress0: BASEADDRESS0,
    #[doc = "0x0c - Source buffer attributes for layer 0."]
    pub sourcebufferattributes0: SOURCEBUFFERATTRIBUTES0,
    #[doc = "0x10 - Source buffer dimension of layer 0."]
    pub sourcebufferdimension0: SOURCEBUFFERDIMENSION0,
    #[doc = "0x14 - Size of color components for RGB, YUV and index formats (layer 0)."]
    pub colorcomponentbits0: COLORCOMPONENTBITS0,
    #[doc = "0x18 - Bit position of color components for RGB, YUV and index formats (layer 0)."]
    pub colorcomponentshift0: COLORCOMPONENTSHIFT0,
    #[doc = "0x1c - Position of layer 0 within the destination frame."]
    pub layeroffset0: LAYEROFFSET0,
    #[doc = "0x20 - Clip window position for layer 0."]
    pub clipwindowoffset0: CLIPWINDOWOFFSET0,
    #[doc = "0x24 - Clip window size for layer 0."]
    pub clipwindowdimensions0: CLIPWINDOWDIMENSIONS0,
    #[doc = "0x28 - Constant color for layer 0."]
    pub constantcolor0: CONSTANTCOLOR0,
    #[doc = "0x2c - Common properties of layer 0."]
    pub layerproperty0: LAYERPROPERTY0,
    #[doc = "0x30 - Output frame dimension."]
    pub framedimensions: FRAMEDIMENSIONS,
    #[doc = "0x34 - Resampling options for output frame."]
    pub frameresampling: FRAMERESAMPLING,
    #[doc = "0x38 - Shared common control settings for all layers."]
    pub control: CONTROL,
    #[doc = "0x3c - Shadow load trigger."]
    pub controltrigger: CONTROLTRIGGER,
    #[doc = "0x40 - Frame start trigger."]
    pub start: START,
    #[doc = "0x44 - Fetch unit type."]
    pub fetchtype: FETCHTYPE,
    #[doc = "0x48 - Burst buffer properties."]
    pub burstbufferproperties: BURSTBUFFERPROPERTIES,
}
#[doc = "STATICCONTROL (rw) register accessor: an alias for `Reg<STATICCONTROL_SPEC>`"]
pub type STATICCONTROL = crate::Reg<staticcontrol::STATICCONTROL_SPEC>;
#[doc = "Common static control options."]
pub mod staticcontrol;
#[doc = "BURSTBUFFERMANAGEMENT (rw) register accessor: an alias for `Reg<BURSTBUFFERMANAGEMENT_SPEC>`"]
pub type BURSTBUFFERMANAGEMENT = crate::Reg<burstbuffermanagement::BURSTBUFFERMANAGEMENT_SPEC>;
#[doc = "AXI interface buffer management register"]
pub mod burstbuffermanagement;
#[doc = "BASEADDRESS0 (rw) register accessor: an alias for `Reg<BASEADDRESS0_SPEC>`"]
pub type BASEADDRESS0 = crate::Reg<baseaddress0::BASEADDRESS0_SPEC>;
#[doc = "Source buffer base address of layer 0."]
pub mod baseaddress0;
#[doc = "SOURCEBUFFERATTRIBUTES0 (rw) register accessor: an alias for `Reg<SOURCEBUFFERATTRIBUTES0_SPEC>`"]
pub type SOURCEBUFFERATTRIBUTES0 =
    crate::Reg<sourcebufferattributes0::SOURCEBUFFERATTRIBUTES0_SPEC>;
#[doc = "Source buffer attributes for layer 0."]
pub mod sourcebufferattributes0;
#[doc = "SOURCEBUFFERDIMENSION0 (rw) register accessor: an alias for `Reg<SOURCEBUFFERDIMENSION0_SPEC>`"]
pub type SOURCEBUFFERDIMENSION0 = crate::Reg<sourcebufferdimension0::SOURCEBUFFERDIMENSION0_SPEC>;
#[doc = "Source buffer dimension of layer 0."]
pub mod sourcebufferdimension0;
#[doc = "COLORCOMPONENTBITS0 (rw) register accessor: an alias for `Reg<COLORCOMPONENTBITS0_SPEC>`"]
pub type COLORCOMPONENTBITS0 = crate::Reg<colorcomponentbits0::COLORCOMPONENTBITS0_SPEC>;
#[doc = "Size of color components for RGB, YUV and index formats (layer 0)."]
pub mod colorcomponentbits0;
#[doc = "COLORCOMPONENTSHIFT0 (rw) register accessor: an alias for `Reg<COLORCOMPONENTSHIFT0_SPEC>`"]
pub type COLORCOMPONENTSHIFT0 = crate::Reg<colorcomponentshift0::COLORCOMPONENTSHIFT0_SPEC>;
#[doc = "Bit position of color components for RGB, YUV and index formats (layer 0)."]
pub mod colorcomponentshift0;
#[doc = "LAYEROFFSET0 (rw) register accessor: an alias for `Reg<LAYEROFFSET0_SPEC>`"]
pub type LAYEROFFSET0 = crate::Reg<layeroffset0::LAYEROFFSET0_SPEC>;
#[doc = "Position of layer 0 within the destination frame."]
pub mod layeroffset0;
#[doc = "CLIPWINDOWOFFSET0 (rw) register accessor: an alias for `Reg<CLIPWINDOWOFFSET0_SPEC>`"]
pub type CLIPWINDOWOFFSET0 = crate::Reg<clipwindowoffset0::CLIPWINDOWOFFSET0_SPEC>;
#[doc = "Clip window position for layer 0."]
pub mod clipwindowoffset0;
#[doc = "CLIPWINDOWDIMENSIONS0 (rw) register accessor: an alias for `Reg<CLIPWINDOWDIMENSIONS0_SPEC>`"]
pub type CLIPWINDOWDIMENSIONS0 = crate::Reg<clipwindowdimensions0::CLIPWINDOWDIMENSIONS0_SPEC>;
#[doc = "Clip window size for layer 0."]
pub mod clipwindowdimensions0;
#[doc = "CONSTANTCOLOR0 (rw) register accessor: an alias for `Reg<CONSTANTCOLOR0_SPEC>`"]
pub type CONSTANTCOLOR0 = crate::Reg<constantcolor0::CONSTANTCOLOR0_SPEC>;
#[doc = "Constant color for layer 0."]
pub mod constantcolor0;
#[doc = "LAYERPROPERTY0 (rw) register accessor: an alias for `Reg<LAYERPROPERTY0_SPEC>`"]
pub type LAYERPROPERTY0 = crate::Reg<layerproperty0::LAYERPROPERTY0_SPEC>;
#[doc = "Common properties of layer 0."]
pub mod layerproperty0;
#[doc = "FRAMEDIMENSIONS (rw) register accessor: an alias for `Reg<FRAMEDIMENSIONS_SPEC>`"]
pub type FRAMEDIMENSIONS = crate::Reg<framedimensions::FRAMEDIMENSIONS_SPEC>;
#[doc = "Output frame dimension."]
pub mod framedimensions;
#[doc = "FRAMERESAMPLING (rw) register accessor: an alias for `Reg<FRAMERESAMPLING_SPEC>`"]
pub type FRAMERESAMPLING = crate::Reg<frameresampling::FRAMERESAMPLING_SPEC>;
#[doc = "Resampling options for output frame."]
pub mod frameresampling;
#[doc = "CONTROL (rw) register accessor: an alias for `Reg<CONTROL_SPEC>`"]
pub type CONTROL = crate::Reg<control::CONTROL_SPEC>;
#[doc = "Shared common control settings for all layers."]
pub mod control;
#[doc = "CONTROLTRIGGER (w) register accessor: an alias for `Reg<CONTROLTRIGGER_SPEC>`"]
pub type CONTROLTRIGGER = crate::Reg<controltrigger::CONTROLTRIGGER_SPEC>;
#[doc = "Shadow load trigger."]
pub mod controltrigger;
#[doc = "START (w) register accessor: an alias for `Reg<START_SPEC>`"]
pub type START = crate::Reg<start::START_SPEC>;
#[doc = "Frame start trigger."]
pub mod start;
#[doc = "FETCHTYPE (r) register accessor: an alias for `Reg<FETCHTYPE_SPEC>`"]
pub type FETCHTYPE = crate::Reg<fetchtype::FETCHTYPE_SPEC>;
#[doc = "Fetch unit type."]
pub mod fetchtype;
#[doc = "BURSTBUFFERPROPERTIES (r) register accessor: an alias for `Reg<BURSTBUFFERPROPERTIES_SPEC>`"]
pub type BURSTBUFFERPROPERTIES = crate::Reg<burstbufferproperties::BURSTBUFFERPROPERTIES_SPEC>;
#[doc = "Burst buffer properties."]
pub mod burstbufferproperties;
