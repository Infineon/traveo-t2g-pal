#[doc = r"Register block"]
#[repr(C)]
pub struct FETCHLAYER1 {
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
    #[doc = "0x30 - Source buffer base address of layer 1."]
    pub baseaddress1: BASEADDRESS1,
    #[doc = "0x34 - Source buffer attributes for layer 1."]
    pub sourcebufferattributes1: SOURCEBUFFERATTRIBUTES1,
    #[doc = "0x38 - Source buffer dimensions of layer 1,"]
    pub sourcebufferdimension1: SOURCEBUFFERDIMENSION1,
    #[doc = "0x3c - Size of color components for RGB, YUV and index formats (layer 1)."]
    pub colorcomponentbits1: COLORCOMPONENTBITS1,
    #[doc = "0x40 - Bit position of color components for RGB, YUV and index formats (layer 1)."]
    pub colorcomponentshift1: COLORCOMPONENTSHIFT1,
    #[doc = "0x44 - Position of layer 1 within the destination frame."]
    pub layeroffset1: LAYEROFFSET1,
    #[doc = "0x48 - Clip window position for layer 1."]
    pub clipwindowoffset1: CLIPWINDOWOFFSET1,
    #[doc = "0x4c - Clip window size for layer 1."]
    pub clipwindowdimensions1: CLIPWINDOWDIMENSIONS1,
    #[doc = "0x50 - Constant color for layer 1."]
    pub constantcolor1: CONSTANTCOLOR1,
    #[doc = "0x54 - Common properties of layer 1."]
    pub layerproperty1: LAYERPROPERTY1,
    #[doc = "0x58 - Source buffer base address of layer 2."]
    pub baseaddress2: BASEADDRESS2,
    #[doc = "0x5c - Source buffer attributes for layer 2."]
    pub sourcebufferattributes2: SOURCEBUFFERATTRIBUTES2,
    #[doc = "0x60 - Source buffer dimension of layer 2."]
    pub sourcebufferdimension2: SOURCEBUFFERDIMENSION2,
    #[doc = "0x64 - Size of color components for RGB, YUV and index formats (layer 2)."]
    pub colorcomponentbits2: COLORCOMPONENTBITS2,
    #[doc = "0x68 - Bit position of color components for RGB, YUV and index formats (layer 2)."]
    pub colorcomponentshift2: COLORCOMPONENTSHIFT2,
    #[doc = "0x6c - Position of layer 2 within the destination frame."]
    pub layeroffset2: LAYEROFFSET2,
    #[doc = "0x70 - Clip window position for layer 2."]
    pub clipwindowoffset2: CLIPWINDOWOFFSET2,
    #[doc = "0x74 - Clip window size for layer 2."]
    pub clipwindowdimensions2: CLIPWINDOWDIMENSIONS2,
    #[doc = "0x78 - Constant color for layer 2."]
    pub constantcolor2: CONSTANTCOLOR2,
    #[doc = "0x7c - Common properties of layer 2."]
    pub layerproperty2: LAYERPROPERTY2,
    #[doc = "0x80 - Source buffer base address of layer 3."]
    pub baseaddress3: BASEADDRESS3,
    #[doc = "0x84 - Source buffer attributes for layer 3."]
    pub sourcebufferattributes3: SOURCEBUFFERATTRIBUTES3,
    #[doc = "0x88 - Source buffer dimension of layer 3."]
    pub sourcebufferdimension3: SOURCEBUFFERDIMENSION3,
    #[doc = "0x8c - Size of color components for RGB, YUV and index formats (layer 3)."]
    pub colorcomponentbits3: COLORCOMPONENTBITS3,
    #[doc = "0x90 - Bit position of color components for RGB, YUV and index formats (layer 3)."]
    pub colorcomponentshift3: COLORCOMPONENTSHIFT3,
    #[doc = "0x94 - Position of layer 3 within the destination frame."]
    pub layeroffset3: LAYEROFFSET3,
    #[doc = "0x98 - Clip window position for layer 3."]
    pub clipwindowoffset3: CLIPWINDOWOFFSET3,
    #[doc = "0x9c - Clip window size for layer 3."]
    pub clipwindowdimensions3: CLIPWINDOWDIMENSIONS3,
    #[doc = "0xa0 - Constant color for layer 3."]
    pub constantcolor3: CONSTANTCOLOR3,
    #[doc = "0xa4 - Common properties of layer 3."]
    pub layerproperty3: LAYERPROPERTY3,
    #[doc = "0xa8 - Source buffer base address of layer 4."]
    pub baseaddress4: BASEADDRESS4,
    #[doc = "0xac - Source buffer attributes for layer 4."]
    pub sourcebufferattributes4: SOURCEBUFFERATTRIBUTES4,
    #[doc = "0xb0 - Source buffer dimension of layer 4."]
    pub sourcebufferdimension4: SOURCEBUFFERDIMENSION4,
    #[doc = "0xb4 - Size of color components for RGB, YUV and index formats (layer 4)."]
    pub colorcomponentbits4: COLORCOMPONENTBITS4,
    #[doc = "0xb8 - Bit position of color components for RGB, YUV and index formats (layer 4)."]
    pub colorcomponentshift4: COLORCOMPONENTSHIFT4,
    #[doc = "0xbc - Position of layer 4 within the destination frame."]
    pub layeroffset4: LAYEROFFSET4,
    #[doc = "0xc0 - Clip window position for layer 4."]
    pub clipwindowoffset4: CLIPWINDOWOFFSET4,
    #[doc = "0xc4 - Clip window size for layer 4."]
    pub clipwindowdimensions4: CLIPWINDOWDIMENSIONS4,
    #[doc = "0xc8 - Constant color for layer 4."]
    pub constantcolor4: CONSTANTCOLOR4,
    #[doc = "0xcc - Common properties of layer 4."]
    pub layerproperty4: LAYERPROPERTY4,
    #[doc = "0xd0 - Source buffer base address of layer 5."]
    pub baseaddress5: BASEADDRESS5,
    #[doc = "0xd4 - Source buffer attributes for layer 5."]
    pub sourcebufferattributes5: SOURCEBUFFERATTRIBUTES5,
    #[doc = "0xd8 - Source buffer dimension of layer 5."]
    pub sourcebufferdimension5: SOURCEBUFFERDIMENSION5,
    #[doc = "0xdc - Size of color components for RGB, YUV and index formats (layer 5)."]
    pub colorcomponentbits5: COLORCOMPONENTBITS5,
    #[doc = "0xe0 - Bit position of color components for RGB, YUV and index formats (layer 5)."]
    pub colorcomponentshift5: COLORCOMPONENTSHIFT5,
    #[doc = "0xe4 - Position of layer 5 within the destination frame."]
    pub layeroffset5: LAYEROFFSET5,
    #[doc = "0xe8 - Clip window position for layer 5."]
    pub clipwindowoffset5: CLIPWINDOWOFFSET5,
    #[doc = "0xec - Clip window size for layer 5."]
    pub clipwindowdimensions5: CLIPWINDOWDIMENSIONS5,
    #[doc = "0xf0 - Constant color for layer 5."]
    pub constantcolor5: CONSTANTCOLOR5,
    #[doc = "0xf4 - Common properties of layer 5."]
    pub layerproperty5: LAYERPROPERTY5,
    #[doc = "0xf8 - Source buffer base address of layer 6."]
    pub baseaddress6: BASEADDRESS6,
    #[doc = "0xfc - Source buffer attributes for layer 6."]
    pub sourcebufferattributes6: SOURCEBUFFERATTRIBUTES6,
    #[doc = "0x100 - Source buffer dimension of layer 6."]
    pub sourcebufferdimension6: SOURCEBUFFERDIMENSION6,
    #[doc = "0x104 - Size of color components for RGB, YUV and index formats (layer 6)."]
    pub colorcomponentbits6: COLORCOMPONENTBITS6,
    #[doc = "0x108 - Bit position of color components for RGB, YUV and index formats (layer 6)."]
    pub colorcomponentshift6: COLORCOMPONENTSHIFT6,
    #[doc = "0x10c - Position of layer 1 within the destination frame."]
    pub layeroffset6: LAYEROFFSET6,
    #[doc = "0x110 - Clip window position for layer 6."]
    pub clipwindowoffset6: CLIPWINDOWOFFSET6,
    #[doc = "0x114 - Clip window size for layer 6."]
    pub clipwindowdimensions6: CLIPWINDOWDIMENSIONS6,
    #[doc = "0x118 - Constant color for layer 6."]
    pub constantcolor6: CONSTANTCOLOR6,
    #[doc = "0x11c - Common properties of layer 6."]
    pub layerproperty6: LAYERPROPERTY6,
    #[doc = "0x120 - Source buffer base address of layer 7."]
    pub baseaddress7: BASEADDRESS7,
    #[doc = "0x124 - Source buffer stride for layer 7."]
    pub sourcebufferattributes7: SOURCEBUFFERATTRIBUTES7,
    #[doc = "0x128 - Source buffer dimension of layer 7."]
    pub sourcebufferdimension7: SOURCEBUFFERDIMENSION7,
    #[doc = "0x12c - Size of color components for RGB, YUV and index formats (layer 7)."]
    pub colorcomponentbits7: COLORCOMPONENTBITS7,
    #[doc = "0x130 - Bit position of color components for RGB, YUV and index formats (layer 7)."]
    pub colorcomponentshift7: COLORCOMPONENTSHIFT7,
    #[doc = "0x134 - Position of layer 7 within the destination frame."]
    pub layeroffset7: LAYEROFFSET7,
    #[doc = "0x138 - Clip window position for layer 7."]
    pub clipwindowoffset7: CLIPWINDOWOFFSET7,
    #[doc = "0x13c - Clip window size for layer 7."]
    pub clipwindowdimensions7: CLIPWINDOWDIMENSIONS7,
    #[doc = "0x140 - Constant color for layer 7."]
    pub constantcolor7: CONSTANTCOLOR7,
    #[doc = "0x144 - Common properties of layer 7."]
    pub layerproperty7: LAYERPROPERTY7,
    #[doc = "0x148 - Output frame dimension."]
    pub framedimensions: FRAMEDIMENSIONS,
    #[doc = "0x14c - Resampling options for output frame."]
    pub frameresampling: FRAMERESAMPLING,
    #[doc = "0x150 - Shared common control settings for all layers."]
    pub control: CONTROL,
    #[doc = "0x154 - Shadow load enable flags for all layers."]
    pub triggerenable: TRIGGERENABLE,
    #[doc = "0x158 - Shadow load trigger."]
    pub controltrigger: CONTROLTRIGGER,
    #[doc = "0x15c - Frame start trigger."]
    pub start: START,
    #[doc = "0x160 - Fetch unit type."]
    pub fetchtype: FETCHTYPE,
    #[doc = "0x164 - Burst buffer properties."]
    pub burstbufferproperties: BURSTBUFFERPROPERTIES,
    #[doc = "0x168 - Status information."]
    pub status: STATUS,
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
#[doc = "BASEADDRESS1 (rw) register accessor: an alias for `Reg<BASEADDRESS1_SPEC>`"]
pub type BASEADDRESS1 = crate::Reg<baseaddress1::BASEADDRESS1_SPEC>;
#[doc = "Source buffer base address of layer 1."]
pub mod baseaddress1;
#[doc = "SOURCEBUFFERATTRIBUTES1 (rw) register accessor: an alias for `Reg<SOURCEBUFFERATTRIBUTES1_SPEC>`"]
pub type SOURCEBUFFERATTRIBUTES1 =
    crate::Reg<sourcebufferattributes1::SOURCEBUFFERATTRIBUTES1_SPEC>;
#[doc = "Source buffer attributes for layer 1."]
pub mod sourcebufferattributes1;
#[doc = "SOURCEBUFFERDIMENSION1 (rw) register accessor: an alias for `Reg<SOURCEBUFFERDIMENSION1_SPEC>`"]
pub type SOURCEBUFFERDIMENSION1 = crate::Reg<sourcebufferdimension1::SOURCEBUFFERDIMENSION1_SPEC>;
#[doc = "Source buffer dimensions of layer 1,"]
pub mod sourcebufferdimension1;
#[doc = "COLORCOMPONENTBITS1 (rw) register accessor: an alias for `Reg<COLORCOMPONENTBITS1_SPEC>`"]
pub type COLORCOMPONENTBITS1 = crate::Reg<colorcomponentbits1::COLORCOMPONENTBITS1_SPEC>;
#[doc = "Size of color components for RGB, YUV and index formats (layer 1)."]
pub mod colorcomponentbits1;
#[doc = "COLORCOMPONENTSHIFT1 (rw) register accessor: an alias for `Reg<COLORCOMPONENTSHIFT1_SPEC>`"]
pub type COLORCOMPONENTSHIFT1 = crate::Reg<colorcomponentshift1::COLORCOMPONENTSHIFT1_SPEC>;
#[doc = "Bit position of color components for RGB, YUV and index formats (layer 1)."]
pub mod colorcomponentshift1;
#[doc = "LAYEROFFSET1 (rw) register accessor: an alias for `Reg<LAYEROFFSET1_SPEC>`"]
pub type LAYEROFFSET1 = crate::Reg<layeroffset1::LAYEROFFSET1_SPEC>;
#[doc = "Position of layer 1 within the destination frame."]
pub mod layeroffset1;
#[doc = "CLIPWINDOWOFFSET1 (rw) register accessor: an alias for `Reg<CLIPWINDOWOFFSET1_SPEC>`"]
pub type CLIPWINDOWOFFSET1 = crate::Reg<clipwindowoffset1::CLIPWINDOWOFFSET1_SPEC>;
#[doc = "Clip window position for layer 1."]
pub mod clipwindowoffset1;
#[doc = "CLIPWINDOWDIMENSIONS1 (rw) register accessor: an alias for `Reg<CLIPWINDOWDIMENSIONS1_SPEC>`"]
pub type CLIPWINDOWDIMENSIONS1 = crate::Reg<clipwindowdimensions1::CLIPWINDOWDIMENSIONS1_SPEC>;
#[doc = "Clip window size for layer 1."]
pub mod clipwindowdimensions1;
#[doc = "CONSTANTCOLOR1 (rw) register accessor: an alias for `Reg<CONSTANTCOLOR1_SPEC>`"]
pub type CONSTANTCOLOR1 = crate::Reg<constantcolor1::CONSTANTCOLOR1_SPEC>;
#[doc = "Constant color for layer 1."]
pub mod constantcolor1;
#[doc = "LAYERPROPERTY1 (rw) register accessor: an alias for `Reg<LAYERPROPERTY1_SPEC>`"]
pub type LAYERPROPERTY1 = crate::Reg<layerproperty1::LAYERPROPERTY1_SPEC>;
#[doc = "Common properties of layer 1."]
pub mod layerproperty1;
#[doc = "BASEADDRESS2 (rw) register accessor: an alias for `Reg<BASEADDRESS2_SPEC>`"]
pub type BASEADDRESS2 = crate::Reg<baseaddress2::BASEADDRESS2_SPEC>;
#[doc = "Source buffer base address of layer 2."]
pub mod baseaddress2;
#[doc = "SOURCEBUFFERATTRIBUTES2 (rw) register accessor: an alias for `Reg<SOURCEBUFFERATTRIBUTES2_SPEC>`"]
pub type SOURCEBUFFERATTRIBUTES2 =
    crate::Reg<sourcebufferattributes2::SOURCEBUFFERATTRIBUTES2_SPEC>;
#[doc = "Source buffer attributes for layer 2."]
pub mod sourcebufferattributes2;
#[doc = "SOURCEBUFFERDIMENSION2 (rw) register accessor: an alias for `Reg<SOURCEBUFFERDIMENSION2_SPEC>`"]
pub type SOURCEBUFFERDIMENSION2 = crate::Reg<sourcebufferdimension2::SOURCEBUFFERDIMENSION2_SPEC>;
#[doc = "Source buffer dimension of layer 2."]
pub mod sourcebufferdimension2;
#[doc = "COLORCOMPONENTBITS2 (rw) register accessor: an alias for `Reg<COLORCOMPONENTBITS2_SPEC>`"]
pub type COLORCOMPONENTBITS2 = crate::Reg<colorcomponentbits2::COLORCOMPONENTBITS2_SPEC>;
#[doc = "Size of color components for RGB, YUV and index formats (layer 2)."]
pub mod colorcomponentbits2;
#[doc = "COLORCOMPONENTSHIFT2 (rw) register accessor: an alias for `Reg<COLORCOMPONENTSHIFT2_SPEC>`"]
pub type COLORCOMPONENTSHIFT2 = crate::Reg<colorcomponentshift2::COLORCOMPONENTSHIFT2_SPEC>;
#[doc = "Bit position of color components for RGB, YUV and index formats (layer 2)."]
pub mod colorcomponentshift2;
#[doc = "LAYEROFFSET2 (rw) register accessor: an alias for `Reg<LAYEROFFSET2_SPEC>`"]
pub type LAYEROFFSET2 = crate::Reg<layeroffset2::LAYEROFFSET2_SPEC>;
#[doc = "Position of layer 2 within the destination frame."]
pub mod layeroffset2;
#[doc = "CLIPWINDOWOFFSET2 (rw) register accessor: an alias for `Reg<CLIPWINDOWOFFSET2_SPEC>`"]
pub type CLIPWINDOWOFFSET2 = crate::Reg<clipwindowoffset2::CLIPWINDOWOFFSET2_SPEC>;
#[doc = "Clip window position for layer 2."]
pub mod clipwindowoffset2;
#[doc = "CLIPWINDOWDIMENSIONS2 (rw) register accessor: an alias for `Reg<CLIPWINDOWDIMENSIONS2_SPEC>`"]
pub type CLIPWINDOWDIMENSIONS2 = crate::Reg<clipwindowdimensions2::CLIPWINDOWDIMENSIONS2_SPEC>;
#[doc = "Clip window size for layer 2."]
pub mod clipwindowdimensions2;
#[doc = "CONSTANTCOLOR2 (rw) register accessor: an alias for `Reg<CONSTANTCOLOR2_SPEC>`"]
pub type CONSTANTCOLOR2 = crate::Reg<constantcolor2::CONSTANTCOLOR2_SPEC>;
#[doc = "Constant color for layer 2."]
pub mod constantcolor2;
#[doc = "LAYERPROPERTY2 (rw) register accessor: an alias for `Reg<LAYERPROPERTY2_SPEC>`"]
pub type LAYERPROPERTY2 = crate::Reg<layerproperty2::LAYERPROPERTY2_SPEC>;
#[doc = "Common properties of layer 2."]
pub mod layerproperty2;
#[doc = "BASEADDRESS3 (rw) register accessor: an alias for `Reg<BASEADDRESS3_SPEC>`"]
pub type BASEADDRESS3 = crate::Reg<baseaddress3::BASEADDRESS3_SPEC>;
#[doc = "Source buffer base address of layer 3."]
pub mod baseaddress3;
#[doc = "SOURCEBUFFERATTRIBUTES3 (rw) register accessor: an alias for `Reg<SOURCEBUFFERATTRIBUTES3_SPEC>`"]
pub type SOURCEBUFFERATTRIBUTES3 =
    crate::Reg<sourcebufferattributes3::SOURCEBUFFERATTRIBUTES3_SPEC>;
#[doc = "Source buffer attributes for layer 3."]
pub mod sourcebufferattributes3;
#[doc = "SOURCEBUFFERDIMENSION3 (rw) register accessor: an alias for `Reg<SOURCEBUFFERDIMENSION3_SPEC>`"]
pub type SOURCEBUFFERDIMENSION3 = crate::Reg<sourcebufferdimension3::SOURCEBUFFERDIMENSION3_SPEC>;
#[doc = "Source buffer dimension of layer 3."]
pub mod sourcebufferdimension3;
#[doc = "COLORCOMPONENTBITS3 (rw) register accessor: an alias for `Reg<COLORCOMPONENTBITS3_SPEC>`"]
pub type COLORCOMPONENTBITS3 = crate::Reg<colorcomponentbits3::COLORCOMPONENTBITS3_SPEC>;
#[doc = "Size of color components for RGB, YUV and index formats (layer 3)."]
pub mod colorcomponentbits3;
#[doc = "COLORCOMPONENTSHIFT3 (rw) register accessor: an alias for `Reg<COLORCOMPONENTSHIFT3_SPEC>`"]
pub type COLORCOMPONENTSHIFT3 = crate::Reg<colorcomponentshift3::COLORCOMPONENTSHIFT3_SPEC>;
#[doc = "Bit position of color components for RGB, YUV and index formats (layer 3)."]
pub mod colorcomponentshift3;
#[doc = "LAYEROFFSET3 (rw) register accessor: an alias for `Reg<LAYEROFFSET3_SPEC>`"]
pub type LAYEROFFSET3 = crate::Reg<layeroffset3::LAYEROFFSET3_SPEC>;
#[doc = "Position of layer 3 within the destination frame."]
pub mod layeroffset3;
#[doc = "CLIPWINDOWOFFSET3 (rw) register accessor: an alias for `Reg<CLIPWINDOWOFFSET3_SPEC>`"]
pub type CLIPWINDOWOFFSET3 = crate::Reg<clipwindowoffset3::CLIPWINDOWOFFSET3_SPEC>;
#[doc = "Clip window position for layer 3."]
pub mod clipwindowoffset3;
#[doc = "CLIPWINDOWDIMENSIONS3 (rw) register accessor: an alias for `Reg<CLIPWINDOWDIMENSIONS3_SPEC>`"]
pub type CLIPWINDOWDIMENSIONS3 = crate::Reg<clipwindowdimensions3::CLIPWINDOWDIMENSIONS3_SPEC>;
#[doc = "Clip window size for layer 3."]
pub mod clipwindowdimensions3;
#[doc = "CONSTANTCOLOR3 (rw) register accessor: an alias for `Reg<CONSTANTCOLOR3_SPEC>`"]
pub type CONSTANTCOLOR3 = crate::Reg<constantcolor3::CONSTANTCOLOR3_SPEC>;
#[doc = "Constant color for layer 3."]
pub mod constantcolor3;
#[doc = "LAYERPROPERTY3 (rw) register accessor: an alias for `Reg<LAYERPROPERTY3_SPEC>`"]
pub type LAYERPROPERTY3 = crate::Reg<layerproperty3::LAYERPROPERTY3_SPEC>;
#[doc = "Common properties of layer 3."]
pub mod layerproperty3;
#[doc = "BASEADDRESS4 (rw) register accessor: an alias for `Reg<BASEADDRESS4_SPEC>`"]
pub type BASEADDRESS4 = crate::Reg<baseaddress4::BASEADDRESS4_SPEC>;
#[doc = "Source buffer base address of layer 4."]
pub mod baseaddress4;
#[doc = "SOURCEBUFFERATTRIBUTES4 (rw) register accessor: an alias for `Reg<SOURCEBUFFERATTRIBUTES4_SPEC>`"]
pub type SOURCEBUFFERATTRIBUTES4 =
    crate::Reg<sourcebufferattributes4::SOURCEBUFFERATTRIBUTES4_SPEC>;
#[doc = "Source buffer attributes for layer 4."]
pub mod sourcebufferattributes4;
#[doc = "SOURCEBUFFERDIMENSION4 (rw) register accessor: an alias for `Reg<SOURCEBUFFERDIMENSION4_SPEC>`"]
pub type SOURCEBUFFERDIMENSION4 = crate::Reg<sourcebufferdimension4::SOURCEBUFFERDIMENSION4_SPEC>;
#[doc = "Source buffer dimension of layer 4."]
pub mod sourcebufferdimension4;
#[doc = "COLORCOMPONENTBITS4 (rw) register accessor: an alias for `Reg<COLORCOMPONENTBITS4_SPEC>`"]
pub type COLORCOMPONENTBITS4 = crate::Reg<colorcomponentbits4::COLORCOMPONENTBITS4_SPEC>;
#[doc = "Size of color components for RGB, YUV and index formats (layer 4)."]
pub mod colorcomponentbits4;
#[doc = "COLORCOMPONENTSHIFT4 (rw) register accessor: an alias for `Reg<COLORCOMPONENTSHIFT4_SPEC>`"]
pub type COLORCOMPONENTSHIFT4 = crate::Reg<colorcomponentshift4::COLORCOMPONENTSHIFT4_SPEC>;
#[doc = "Bit position of color components for RGB, YUV and index formats (layer 4)."]
pub mod colorcomponentshift4;
#[doc = "LAYEROFFSET4 (rw) register accessor: an alias for `Reg<LAYEROFFSET4_SPEC>`"]
pub type LAYEROFFSET4 = crate::Reg<layeroffset4::LAYEROFFSET4_SPEC>;
#[doc = "Position of layer 4 within the destination frame."]
pub mod layeroffset4;
#[doc = "CLIPWINDOWOFFSET4 (rw) register accessor: an alias for `Reg<CLIPWINDOWOFFSET4_SPEC>`"]
pub type CLIPWINDOWOFFSET4 = crate::Reg<clipwindowoffset4::CLIPWINDOWOFFSET4_SPEC>;
#[doc = "Clip window position for layer 4."]
pub mod clipwindowoffset4;
#[doc = "CLIPWINDOWDIMENSIONS4 (rw) register accessor: an alias for `Reg<CLIPWINDOWDIMENSIONS4_SPEC>`"]
pub type CLIPWINDOWDIMENSIONS4 = crate::Reg<clipwindowdimensions4::CLIPWINDOWDIMENSIONS4_SPEC>;
#[doc = "Clip window size for layer 4."]
pub mod clipwindowdimensions4;
#[doc = "CONSTANTCOLOR4 (rw) register accessor: an alias for `Reg<CONSTANTCOLOR4_SPEC>`"]
pub type CONSTANTCOLOR4 = crate::Reg<constantcolor4::CONSTANTCOLOR4_SPEC>;
#[doc = "Constant color for layer 4."]
pub mod constantcolor4;
#[doc = "LAYERPROPERTY4 (rw) register accessor: an alias for `Reg<LAYERPROPERTY4_SPEC>`"]
pub type LAYERPROPERTY4 = crate::Reg<layerproperty4::LAYERPROPERTY4_SPEC>;
#[doc = "Common properties of layer 4."]
pub mod layerproperty4;
#[doc = "BASEADDRESS5 (rw) register accessor: an alias for `Reg<BASEADDRESS5_SPEC>`"]
pub type BASEADDRESS5 = crate::Reg<baseaddress5::BASEADDRESS5_SPEC>;
#[doc = "Source buffer base address of layer 5."]
pub mod baseaddress5;
#[doc = "SOURCEBUFFERATTRIBUTES5 (rw) register accessor: an alias for `Reg<SOURCEBUFFERATTRIBUTES5_SPEC>`"]
pub type SOURCEBUFFERATTRIBUTES5 =
    crate::Reg<sourcebufferattributes5::SOURCEBUFFERATTRIBUTES5_SPEC>;
#[doc = "Source buffer attributes for layer 5."]
pub mod sourcebufferattributes5;
#[doc = "SOURCEBUFFERDIMENSION5 (rw) register accessor: an alias for `Reg<SOURCEBUFFERDIMENSION5_SPEC>`"]
pub type SOURCEBUFFERDIMENSION5 = crate::Reg<sourcebufferdimension5::SOURCEBUFFERDIMENSION5_SPEC>;
#[doc = "Source buffer dimension of layer 5."]
pub mod sourcebufferdimension5;
#[doc = "COLORCOMPONENTBITS5 (rw) register accessor: an alias for `Reg<COLORCOMPONENTBITS5_SPEC>`"]
pub type COLORCOMPONENTBITS5 = crate::Reg<colorcomponentbits5::COLORCOMPONENTBITS5_SPEC>;
#[doc = "Size of color components for RGB, YUV and index formats (layer 5)."]
pub mod colorcomponentbits5;
#[doc = "COLORCOMPONENTSHIFT5 (rw) register accessor: an alias for `Reg<COLORCOMPONENTSHIFT5_SPEC>`"]
pub type COLORCOMPONENTSHIFT5 = crate::Reg<colorcomponentshift5::COLORCOMPONENTSHIFT5_SPEC>;
#[doc = "Bit position of color components for RGB, YUV and index formats (layer 5)."]
pub mod colorcomponentshift5;
#[doc = "LAYEROFFSET5 (rw) register accessor: an alias for `Reg<LAYEROFFSET5_SPEC>`"]
pub type LAYEROFFSET5 = crate::Reg<layeroffset5::LAYEROFFSET5_SPEC>;
#[doc = "Position of layer 5 within the destination frame."]
pub mod layeroffset5;
#[doc = "CLIPWINDOWOFFSET5 (rw) register accessor: an alias for `Reg<CLIPWINDOWOFFSET5_SPEC>`"]
pub type CLIPWINDOWOFFSET5 = crate::Reg<clipwindowoffset5::CLIPWINDOWOFFSET5_SPEC>;
#[doc = "Clip window position for layer 5."]
pub mod clipwindowoffset5;
#[doc = "CLIPWINDOWDIMENSIONS5 (rw) register accessor: an alias for `Reg<CLIPWINDOWDIMENSIONS5_SPEC>`"]
pub type CLIPWINDOWDIMENSIONS5 = crate::Reg<clipwindowdimensions5::CLIPWINDOWDIMENSIONS5_SPEC>;
#[doc = "Clip window size for layer 5."]
pub mod clipwindowdimensions5;
#[doc = "CONSTANTCOLOR5 (rw) register accessor: an alias for `Reg<CONSTANTCOLOR5_SPEC>`"]
pub type CONSTANTCOLOR5 = crate::Reg<constantcolor5::CONSTANTCOLOR5_SPEC>;
#[doc = "Constant color for layer 5."]
pub mod constantcolor5;
#[doc = "LAYERPROPERTY5 (rw) register accessor: an alias for `Reg<LAYERPROPERTY5_SPEC>`"]
pub type LAYERPROPERTY5 = crate::Reg<layerproperty5::LAYERPROPERTY5_SPEC>;
#[doc = "Common properties of layer 5."]
pub mod layerproperty5;
#[doc = "BASEADDRESS6 (rw) register accessor: an alias for `Reg<BASEADDRESS6_SPEC>`"]
pub type BASEADDRESS6 = crate::Reg<baseaddress6::BASEADDRESS6_SPEC>;
#[doc = "Source buffer base address of layer 6."]
pub mod baseaddress6;
#[doc = "SOURCEBUFFERATTRIBUTES6 (rw) register accessor: an alias for `Reg<SOURCEBUFFERATTRIBUTES6_SPEC>`"]
pub type SOURCEBUFFERATTRIBUTES6 =
    crate::Reg<sourcebufferattributes6::SOURCEBUFFERATTRIBUTES6_SPEC>;
#[doc = "Source buffer attributes for layer 6."]
pub mod sourcebufferattributes6;
#[doc = "SOURCEBUFFERDIMENSION6 (rw) register accessor: an alias for `Reg<SOURCEBUFFERDIMENSION6_SPEC>`"]
pub type SOURCEBUFFERDIMENSION6 = crate::Reg<sourcebufferdimension6::SOURCEBUFFERDIMENSION6_SPEC>;
#[doc = "Source buffer dimension of layer 6."]
pub mod sourcebufferdimension6;
#[doc = "COLORCOMPONENTBITS6 (rw) register accessor: an alias for `Reg<COLORCOMPONENTBITS6_SPEC>`"]
pub type COLORCOMPONENTBITS6 = crate::Reg<colorcomponentbits6::COLORCOMPONENTBITS6_SPEC>;
#[doc = "Size of color components for RGB, YUV and index formats (layer 6)."]
pub mod colorcomponentbits6;
#[doc = "COLORCOMPONENTSHIFT6 (rw) register accessor: an alias for `Reg<COLORCOMPONENTSHIFT6_SPEC>`"]
pub type COLORCOMPONENTSHIFT6 = crate::Reg<colorcomponentshift6::COLORCOMPONENTSHIFT6_SPEC>;
#[doc = "Bit position of color components for RGB, YUV and index formats (layer 6)."]
pub mod colorcomponentshift6;
#[doc = "LAYEROFFSET6 (rw) register accessor: an alias for `Reg<LAYEROFFSET6_SPEC>`"]
pub type LAYEROFFSET6 = crate::Reg<layeroffset6::LAYEROFFSET6_SPEC>;
#[doc = "Position of layer 1 within the destination frame."]
pub mod layeroffset6;
#[doc = "CLIPWINDOWOFFSET6 (rw) register accessor: an alias for `Reg<CLIPWINDOWOFFSET6_SPEC>`"]
pub type CLIPWINDOWOFFSET6 = crate::Reg<clipwindowoffset6::CLIPWINDOWOFFSET6_SPEC>;
#[doc = "Clip window position for layer 6."]
pub mod clipwindowoffset6;
#[doc = "CLIPWINDOWDIMENSIONS6 (rw) register accessor: an alias for `Reg<CLIPWINDOWDIMENSIONS6_SPEC>`"]
pub type CLIPWINDOWDIMENSIONS6 = crate::Reg<clipwindowdimensions6::CLIPWINDOWDIMENSIONS6_SPEC>;
#[doc = "Clip window size for layer 6."]
pub mod clipwindowdimensions6;
#[doc = "CONSTANTCOLOR6 (rw) register accessor: an alias for `Reg<CONSTANTCOLOR6_SPEC>`"]
pub type CONSTANTCOLOR6 = crate::Reg<constantcolor6::CONSTANTCOLOR6_SPEC>;
#[doc = "Constant color for layer 6."]
pub mod constantcolor6;
#[doc = "LAYERPROPERTY6 (rw) register accessor: an alias for `Reg<LAYERPROPERTY6_SPEC>`"]
pub type LAYERPROPERTY6 = crate::Reg<layerproperty6::LAYERPROPERTY6_SPEC>;
#[doc = "Common properties of layer 6."]
pub mod layerproperty6;
#[doc = "BASEADDRESS7 (rw) register accessor: an alias for `Reg<BASEADDRESS7_SPEC>`"]
pub type BASEADDRESS7 = crate::Reg<baseaddress7::BASEADDRESS7_SPEC>;
#[doc = "Source buffer base address of layer 7."]
pub mod baseaddress7;
#[doc = "SOURCEBUFFERATTRIBUTES7 (rw) register accessor: an alias for `Reg<SOURCEBUFFERATTRIBUTES7_SPEC>`"]
pub type SOURCEBUFFERATTRIBUTES7 =
    crate::Reg<sourcebufferattributes7::SOURCEBUFFERATTRIBUTES7_SPEC>;
#[doc = "Source buffer stride for layer 7."]
pub mod sourcebufferattributes7;
#[doc = "SOURCEBUFFERDIMENSION7 (rw) register accessor: an alias for `Reg<SOURCEBUFFERDIMENSION7_SPEC>`"]
pub type SOURCEBUFFERDIMENSION7 = crate::Reg<sourcebufferdimension7::SOURCEBUFFERDIMENSION7_SPEC>;
#[doc = "Source buffer dimension of layer 7."]
pub mod sourcebufferdimension7;
#[doc = "COLORCOMPONENTBITS7 (rw) register accessor: an alias for `Reg<COLORCOMPONENTBITS7_SPEC>`"]
pub type COLORCOMPONENTBITS7 = crate::Reg<colorcomponentbits7::COLORCOMPONENTBITS7_SPEC>;
#[doc = "Size of color components for RGB, YUV and index formats (layer 7)."]
pub mod colorcomponentbits7;
#[doc = "COLORCOMPONENTSHIFT7 (rw) register accessor: an alias for `Reg<COLORCOMPONENTSHIFT7_SPEC>`"]
pub type COLORCOMPONENTSHIFT7 = crate::Reg<colorcomponentshift7::COLORCOMPONENTSHIFT7_SPEC>;
#[doc = "Bit position of color components for RGB, YUV and index formats (layer 7)."]
pub mod colorcomponentshift7;
#[doc = "LAYEROFFSET7 (rw) register accessor: an alias for `Reg<LAYEROFFSET7_SPEC>`"]
pub type LAYEROFFSET7 = crate::Reg<layeroffset7::LAYEROFFSET7_SPEC>;
#[doc = "Position of layer 7 within the destination frame."]
pub mod layeroffset7;
#[doc = "CLIPWINDOWOFFSET7 (rw) register accessor: an alias for `Reg<CLIPWINDOWOFFSET7_SPEC>`"]
pub type CLIPWINDOWOFFSET7 = crate::Reg<clipwindowoffset7::CLIPWINDOWOFFSET7_SPEC>;
#[doc = "Clip window position for layer 7."]
pub mod clipwindowoffset7;
#[doc = "CLIPWINDOWDIMENSIONS7 (rw) register accessor: an alias for `Reg<CLIPWINDOWDIMENSIONS7_SPEC>`"]
pub type CLIPWINDOWDIMENSIONS7 = crate::Reg<clipwindowdimensions7::CLIPWINDOWDIMENSIONS7_SPEC>;
#[doc = "Clip window size for layer 7."]
pub mod clipwindowdimensions7;
#[doc = "CONSTANTCOLOR7 (rw) register accessor: an alias for `Reg<CONSTANTCOLOR7_SPEC>`"]
pub type CONSTANTCOLOR7 = crate::Reg<constantcolor7::CONSTANTCOLOR7_SPEC>;
#[doc = "Constant color for layer 7."]
pub mod constantcolor7;
#[doc = "LAYERPROPERTY7 (rw) register accessor: an alias for `Reg<LAYERPROPERTY7_SPEC>`"]
pub type LAYERPROPERTY7 = crate::Reg<layerproperty7::LAYERPROPERTY7_SPEC>;
#[doc = "Common properties of layer 7."]
pub mod layerproperty7;
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
#[doc = "TRIGGERENABLE (rw) register accessor: an alias for `Reg<TRIGGERENABLE_SPEC>`"]
pub type TRIGGERENABLE = crate::Reg<triggerenable::TRIGGERENABLE_SPEC>;
#[doc = "Shadow load enable flags for all layers."]
pub mod triggerenable;
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
#[doc = "STATUS (rw) register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Status information."]
pub mod status;
