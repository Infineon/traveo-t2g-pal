#[doc = r"Register block"]
#[repr(C)]
pub struct FETCHDST {
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
    #[doc = "0x38 - Warping control options."]
    pub warpcontrol: WARPCONTROL,
    #[doc = "0x3c - Start value X for affine/perspective warping."]
    pub perspstartx: PERSPSTARTX,
    #[doc = "0x40 - Start value Y for affine/perspective warping."]
    pub perspstarty: PERSPSTARTY,
    #[doc = "0x44 - DeltaXX increment for affine/perspective warping."]
    pub perspdeltaxx: PERSPDELTAXX,
    #[doc = "0x48 - DeltaYY increment for affine/perspective warping."]
    pub perspdeltayy: PERSPDELTAYY,
    #[doc = "0x4c - DeltaXY increment for affine/perspective warping."]
    pub perspdeltaxy: PERSPDELTAXY,
    #[doc = "0x50 - DeltaYX increment for affine/perspective warping."]
    pub perspdeltayx: PERSPDELTAYX,
    #[doc = "0x54 - Start value W for perspective warping."]
    pub perspstartw: PERSPSTARTW,
    #[doc = "0x58 - DeltaWX increment for perspective warping."]
    pub perspdeltaxw: PERSPDELTAXW,
    #[doc = "0x5c - DeltaWY increment for perspective warping."]
    pub perspdeltayw: PERSPDELTAYW,
    #[doc = "0x60 - Start value X for arbitrary warping."]
    pub arbstartx: ARBSTARTX,
    #[doc = "0x64 - Start value Y for arbitrary warping."]
    pub arbstarty: ARBSTARTY,
    #[doc = "0x68 - Start values for delta incrementation of arbitrary warping."]
    pub arbdelta: ARBDELTA,
    #[doc = "0x6c - FIR sequence control register."]
    pub firpositions: FIRPOSITIONS,
    #[doc = "0x70 - FIR coefficients register."]
    pub fircoefficients: FIRCOEFFICIENTS,
    #[doc = "0x74 - Control options for RLAD decompression."]
    pub decodecontrol: DECODECONTROL,
    #[doc = "0x78 - Source buffer length for compressed data."]
    pub sourcebufferlength: SOURCEBUFFERLENGTH,
    #[doc = "0x7c - Shared common control settings for all layers."]
    pub control: CONTROL,
    #[doc = "0x80 - Shadow load trigger."]
    pub controltrigger: CONTROLTRIGGER,
    #[doc = "0x84 - Frame start trigger."]
    pub start: START,
    #[doc = "0x88 - Fetch unit type."]
    pub fetchtype: FETCHTYPE,
    #[doc = "0x8c - Status information of the RLAD decoder."]
    pub decoderstatus: DECODERSTATUS,
    #[doc = "0x90 - Burst buffer properties."]
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
#[doc = "WARPCONTROL (rw) register accessor: an alias for `Reg<WARPCONTROL_SPEC>`"]
pub type WARPCONTROL = crate::Reg<warpcontrol::WARPCONTROL_SPEC>;
#[doc = "Warping control options."]
pub mod warpcontrol;
#[doc = "PERSPSTARTX (rw) register accessor: an alias for `Reg<PERSPSTARTX_SPEC>`"]
pub type PERSPSTARTX = crate::Reg<perspstartx::PERSPSTARTX_SPEC>;
#[doc = "Start value X for affine/perspective warping."]
pub mod perspstartx;
#[doc = "PERSPSTARTY (rw) register accessor: an alias for `Reg<PERSPSTARTY_SPEC>`"]
pub type PERSPSTARTY = crate::Reg<perspstarty::PERSPSTARTY_SPEC>;
#[doc = "Start value Y for affine/perspective warping."]
pub mod perspstarty;
#[doc = "PERSPDELTAXX (rw) register accessor: an alias for `Reg<PERSPDELTAXX_SPEC>`"]
pub type PERSPDELTAXX = crate::Reg<perspdeltaxx::PERSPDELTAXX_SPEC>;
#[doc = "DeltaXX increment for affine/perspective warping."]
pub mod perspdeltaxx;
#[doc = "PERSPDELTAYY (rw) register accessor: an alias for `Reg<PERSPDELTAYY_SPEC>`"]
pub type PERSPDELTAYY = crate::Reg<perspdeltayy::PERSPDELTAYY_SPEC>;
#[doc = "DeltaYY increment for affine/perspective warping."]
pub mod perspdeltayy;
#[doc = "PERSPDELTAXY (rw) register accessor: an alias for `Reg<PERSPDELTAXY_SPEC>`"]
pub type PERSPDELTAXY = crate::Reg<perspdeltaxy::PERSPDELTAXY_SPEC>;
#[doc = "DeltaXY increment for affine/perspective warping."]
pub mod perspdeltaxy;
#[doc = "PERSPDELTAYX (rw) register accessor: an alias for `Reg<PERSPDELTAYX_SPEC>`"]
pub type PERSPDELTAYX = crate::Reg<perspdeltayx::PERSPDELTAYX_SPEC>;
#[doc = "DeltaYX increment for affine/perspective warping."]
pub mod perspdeltayx;
#[doc = "PERSPSTARTW (rw) register accessor: an alias for `Reg<PERSPSTARTW_SPEC>`"]
pub type PERSPSTARTW = crate::Reg<perspstartw::PERSPSTARTW_SPEC>;
#[doc = "Start value W for perspective warping."]
pub mod perspstartw;
#[doc = "PERSPDELTAXW (rw) register accessor: an alias for `Reg<PERSPDELTAXW_SPEC>`"]
pub type PERSPDELTAXW = crate::Reg<perspdeltaxw::PERSPDELTAXW_SPEC>;
#[doc = "DeltaWX increment for perspective warping."]
pub mod perspdeltaxw;
#[doc = "PERSPDELTAYW (rw) register accessor: an alias for `Reg<PERSPDELTAYW_SPEC>`"]
pub type PERSPDELTAYW = crate::Reg<perspdeltayw::PERSPDELTAYW_SPEC>;
#[doc = "DeltaWY increment for perspective warping."]
pub mod perspdeltayw;
#[doc = "ARBSTARTX (rw) register accessor: an alias for `Reg<ARBSTARTX_SPEC>`"]
pub type ARBSTARTX = crate::Reg<arbstartx::ARBSTARTX_SPEC>;
#[doc = "Start value X for arbitrary warping."]
pub mod arbstartx;
#[doc = "ARBSTARTY (rw) register accessor: an alias for `Reg<ARBSTARTY_SPEC>`"]
pub type ARBSTARTY = crate::Reg<arbstarty::ARBSTARTY_SPEC>;
#[doc = "Start value Y for arbitrary warping."]
pub mod arbstarty;
#[doc = "ARBDELTA (rw) register accessor: an alias for `Reg<ARBDELTA_SPEC>`"]
pub type ARBDELTA = crate::Reg<arbdelta::ARBDELTA_SPEC>;
#[doc = "Start values for delta incrementation of arbitrary warping."]
pub mod arbdelta;
#[doc = "FIRPOSITIONS (rw) register accessor: an alias for `Reg<FIRPOSITIONS_SPEC>`"]
pub type FIRPOSITIONS = crate::Reg<firpositions::FIRPOSITIONS_SPEC>;
#[doc = "FIR sequence control register."]
pub mod firpositions;
#[doc = "FIRCOEFFICIENTS (rw) register accessor: an alias for `Reg<FIRCOEFFICIENTS_SPEC>`"]
pub type FIRCOEFFICIENTS = crate::Reg<fircoefficients::FIRCOEFFICIENTS_SPEC>;
#[doc = "FIR coefficients register."]
pub mod fircoefficients;
#[doc = "DECODECONTROL (rw) register accessor: an alias for `Reg<DECODECONTROL_SPEC>`"]
pub type DECODECONTROL = crate::Reg<decodecontrol::DECODECONTROL_SPEC>;
#[doc = "Control options for RLAD decompression."]
pub mod decodecontrol;
#[doc = "SOURCEBUFFERLENGTH (rw) register accessor: an alias for `Reg<SOURCEBUFFERLENGTH_SPEC>`"]
pub type SOURCEBUFFERLENGTH = crate::Reg<sourcebufferlength::SOURCEBUFFERLENGTH_SPEC>;
#[doc = "Source buffer length for compressed data."]
pub mod sourcebufferlength;
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
#[doc = "DECODERSTATUS (rw) register accessor: an alias for `Reg<DECODERSTATUS_SPEC>`"]
pub type DECODERSTATUS = crate::Reg<decoderstatus::DECODERSTATUS_SPEC>;
#[doc = "Status information of the RLAD decoder."]
pub mod decoderstatus;
#[doc = "BURSTBUFFERPROPERTIES (r) register accessor: an alias for `Reg<BURSTBUFFERPROPERTIES_SPEC>`"]
pub type BURSTBUFFERPROPERTIES = crate::Reg<burstbufferproperties::BURSTBUFFERPROPERTIES_SPEC>;
#[doc = "Burst buffer properties."]
pub mod burstbufferproperties;
