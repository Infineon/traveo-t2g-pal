#[doc = r"Register block"]
#[repr(C)]
pub struct COMPENGCFG {
    #[doc = "0x00..0x1000 - Shared Color Palette Memory."]
    pub palette: [PALETTE; 1024],
    #[doc = "0x1000 - Safety mask for extdst0"]
    pub extdst0_safetymask: EXTDST0_SAFETYMASK,
    #[doc = "0x1004 - Safety mask for extdst4"]
    pub extdst4_safetymask: EXTDST4_SAFETYMASK,
    #[doc = "0x1008 - Safety mask for extdst1"]
    pub extdst1_safetymask: EXTDST1_SAFETYMASK,
    #[doc = "0x100c - Safety mask for extdst5"]
    pub extdst5_safetymask: EXTDST5_SAFETYMASK,
    #[doc = "0x1010 - Safety mask for store4"]
    pub store4_safetymask: STORE4_SAFETYMASK,
    _reserved6: [u8; 0x0c],
    #[doc = "0x1020 - Status information for pixel engine configuration of constframe0"]
    pub constframe0_status: CONSTFRAME0_STATUS,
    _reserved7: [u8; 0x1c],
    #[doc = "0x1040 - Static pixel engine configuration for extdst0"]
    pub extdst0_static: EXTDST0_STATIC,
    #[doc = "0x1044 - Dynamic pixel engine configuration for extdst0"]
    pub extdst0_dynamic: EXTDST0_DYNAMIC,
    #[doc = "0x1048 - ShadowLoadRequest register for endpoint extdst0"]
    pub extdst0_request: EXTDST0_REQUEST,
    #[doc = "0x104c - Trigger bits for pixel engine configuration of extdst0"]
    pub extdst0_trigger: EXTDST0_TRIGGER,
    #[doc = "0x1050 - Status information for pixel engine configuration of extdst0"]
    pub extdst0_status: EXTDST0_STATUS,
    _reserved12: [u8; 0x2c],
    #[doc = "0x1080 - Status information for pixel engine configuration of constframe4"]
    pub constframe4_status: CONSTFRAME4_STATUS,
    _reserved13: [u8; 0x1c],
    #[doc = "0x10a0 - Static pixel engine configuration for extdst4"]
    pub extdst4_static: EXTDST4_STATIC,
    #[doc = "0x10a4 - Dynamic pixel engine configuration for extdst4"]
    pub extdst4_dynamic: EXTDST4_DYNAMIC,
    #[doc = "0x10a8 - ShadowLoadRequest register for endpoint extdst4"]
    pub extdst4_request: EXTDST4_REQUEST,
    #[doc = "0x10ac - Trigger bits for pixel engine configuration of extdst4"]
    pub extdst4_trigger: EXTDST4_TRIGGER,
    #[doc = "0x10b0 - Status information for pixel engine configuration of extdst4"]
    pub extdst4_status: EXTDST4_STATUS,
    _reserved18: [u8; 0x0c],
    #[doc = "0x10c0 - Status information for pixel engine configuration of constframe1"]
    pub constframe1_status: CONSTFRAME1_STATUS,
    _reserved19: [u8; 0x1c],
    #[doc = "0x10e0 - Static pixel engine configuration for extdst1"]
    pub extdst1_static: EXTDST1_STATIC,
    #[doc = "0x10e4 - Dynamic pixel engine configuration for extdst1"]
    pub extdst1_dynamic: EXTDST1_DYNAMIC,
    #[doc = "0x10e8 - ShadowLoadRequest register for endpoint extdst1"]
    pub extdst1_request: EXTDST1_REQUEST,
    #[doc = "0x10ec - Trigger bits for pixel engine configuration of extdst1"]
    pub extdst1_trigger: EXTDST1_TRIGGER,
    #[doc = "0x10f0 - Status information for pixel engine configuration of extdst1"]
    pub extdst1_status: EXTDST1_STATUS,
    _reserved24: [u8; 0x0c],
    #[doc = "0x1100 - Status information for pixel engine configuration of constframe5"]
    pub constframe5_status: CONSTFRAME5_STATUS,
    _reserved25: [u8; 0x1c],
    #[doc = "0x1120 - Static pixel engine configuration for extdst5"]
    pub extdst5_static: EXTDST5_STATIC,
    #[doc = "0x1124 - Dynamic pixel engine configuration for extdst5"]
    pub extdst5_dynamic: EXTDST5_DYNAMIC,
    #[doc = "0x1128 - ShadowLoadRequest register for endpoint extdst5"]
    pub extdst5_request: EXTDST5_REQUEST,
    #[doc = "0x112c - Trigger bits for pixel engine configuration of extdst5"]
    pub extdst5_trigger: EXTDST5_TRIGGER,
    #[doc = "0x1130 - Status information for pixel engine configuration of extdst5"]
    pub extdst5_status: EXTDST5_STATUS,
    _reserved30: [u8; 0x0c],
    #[doc = "0x1140 - Status information for pixel engine configuration of extsrc4"]
    pub extsrc4_status: EXTSRC4_STATUS,
    _reserved31: [u8; 0x1c],
    #[doc = "0x1160 - Static pixel engine configuration for store4"]
    pub store4_static: STORE4_STATIC,
    #[doc = "0x1164 - Dynamic pixel engine configuration for store4"]
    pub store4_dynamic: STORE4_DYNAMIC,
    #[doc = "0x1168 - ShadowLoadRequest register for endpoint store4"]
    pub store4_request: STORE4_REQUEST,
    #[doc = "0x116c - Trigger bits for pixel engine configuration of store4"]
    pub store4_trigger: STORE4_TRIGGER,
    #[doc = "0x1170 - Status information for pixel engine configuration of store4"]
    pub store4_status: STORE4_STATUS,
    _reserved36: [u8; 0x0c],
    #[doc = "0x1180 - Status information for pixel engine configuration of fetchlayer0"]
    pub fetchlayer0_status: FETCHLAYER0_STATUS,
    _reserved37: [u8; 0x1c],
    #[doc = "0x11a0 - Dynamic pixel engine configuration for fetchdecode4"]
    pub fetchdecode4_dynamic: FETCHDECODE4_DYNAMIC,
    #[doc = "0x11a4 - Status information for pixel engine configuration of fetchdecode4"]
    pub fetchdecode4_status: FETCHDECODE4_STATUS,
    _reserved39: [u8; 0x18],
    #[doc = "0x11c0 - Status information for pixel engine configuration of fetcheco4"]
    pub fetcheco4_status: FETCHECO4_STATUS,
    _reserved40: [u8; 0x3c],
    #[doc = "0x1200 - Dynamic pixel engine configuration for fetchwarp1"]
    pub fetchwarp1_dynamic: FETCHWARP1_DYNAMIC,
    #[doc = "0x1204 - Status information for pixel engine configuration of fetchwarp1"]
    pub fetchwarp1_status: FETCHWARP1_STATUS,
    #[doc = "0x1208 - Status information for pixel engine configuration of fetcheco1"]
    pub fetcheco1_status: FETCHECO1_STATUS,
    _reserved43: [u8; 0x14],
    #[doc = "0x1220 - Status information for pixel engine configuration of fetchlayer1"]
    pub fetchlayer1_status: FETCHLAYER1_STATUS,
    _reserved44: [u8; 0x1c],
    #[doc = "0x1240 - Dynamic pixel engine configuration for fetchdecode0"]
    pub fetchdecode0_dynamic: FETCHDECODE0_DYNAMIC,
    #[doc = "0x1244 - Status information for pixel engine configuration of fetchdecode0"]
    pub fetchdecode0_status: FETCHDECODE0_STATUS,
    _reserved46: [u8; 0x18],
    #[doc = "0x1260 - Dynamic pixel engine configuration for gammacor4"]
    pub gammacor4_dynamic: GAMMACOR4_DYNAMIC,
    #[doc = "0x1264 - Status information for pixel engine configuration of gammacor4"]
    pub gammacor4_status: GAMMACOR4_STATUS,
    _reserved48: [u8; 0x08],
    #[doc = "0x1270 - Dynamic pixel engine configuration for matrix4"]
    pub matrix4_dynamic: MATRIX4_DYNAMIC,
    #[doc = "0x1274 - Status information for pixel engine configuration of matrix4"]
    pub matrix4_status: MATRIX4_STATUS,
    _reserved50: [u8; 0x08],
    #[doc = "0x1280 - Dynamic pixel engine configuration for gpscaler4"]
    pub gpscaler4_dynamic: GPSCALER4_DYNAMIC,
    #[doc = "0x1284 - Status information for pixel engine configuration of gpscaler4"]
    pub gpscaler4_status: GPSCALER4_STATUS,
    _reserved52: [u8; 0x08],
    #[doc = "0x1290 - Dynamic pixel engine configuration for histogram4"]
    pub histogram4_dynamic: HISTOGRAM4_DYNAMIC,
    #[doc = "0x1294 - Status information for pixel engine configuration of histogram4"]
    pub histogram4_status: HISTOGRAM4_STATUS,
    _reserved54: [u8; 0x08],
    #[doc = "0x12a0 - Dynamic pixel engine configuration for layerblend1"]
    pub layerblend1_dynamic: LAYERBLEND1_DYNAMIC,
    #[doc = "0x12a4 - Status information for pixel engine configuration of layerblend1"]
    pub layerblend1_status: LAYERBLEND1_STATUS,
    _reserved56: [u8; 0x18],
    #[doc = "0x12c0 - Dynamic pixel engine configuration for layerblend2"]
    pub layerblend2_dynamic: LAYERBLEND2_DYNAMIC,
    #[doc = "0x12c4 - Status information for pixel engine configuration of layerblend2"]
    pub layerblend2_status: LAYERBLEND2_STATUS,
    _reserved58: [u8; 0x18],
    #[doc = "0x12e0 - Dynamic pixel engine configuration for layerblend3"]
    pub layerblend3_dynamic: LAYERBLEND3_DYNAMIC,
    #[doc = "0x12e4 - Status information for pixel engine configuration of layerblend3"]
    pub layerblend3_status: LAYERBLEND3_STATUS,
    _reserved60: [u8; 0x18],
    #[doc = "0x1300 - Dynamic pixel engine configuration for layerblend4"]
    pub layerblend4_dynamic: LAYERBLEND4_DYNAMIC,
    #[doc = "0x1304 - Status information for pixel engine configuration of layerblend4"]
    pub layerblend4_status: LAYERBLEND4_STATUS,
    _reserved62: [u8; 0x18],
    #[doc = "0x1320 - Dynamic pixel engine configuration for layerblend5"]
    pub layerblend5_dynamic: LAYERBLEND5_DYNAMIC,
    #[doc = "0x1324 - Status information for pixel engine configuration of layerblend5"]
    pub layerblend5_status: LAYERBLEND5_STATUS,
    _reserved64: [u8; 0x18],
    #[doc = "0x1340 - Status information for pixel engine configuration of extsrc8"]
    pub extsrc8_status: EXTSRC8_STATUS,
}
#[doc = "PALETTE (rw) register accessor: an alias for `Reg<PALETTE_SPEC>`"]
pub type PALETTE = crate::Reg<palette::PALETTE_SPEC>;
#[doc = "Shared Color Palette Memory."]
pub mod palette;
#[doc = "EXTDST0_SAFETYMASK (rw) register accessor: an alias for `Reg<EXTDST0_SAFETYMASK_SPEC>`"]
pub type EXTDST0_SAFETYMASK = crate::Reg<extdst0_safetymask::EXTDST0_SAFETYMASK_SPEC>;
#[doc = "Safety mask for extdst0"]
pub mod extdst0_safetymask;
#[doc = "EXTDST4_SAFETYMASK (rw) register accessor: an alias for `Reg<EXTDST4_SAFETYMASK_SPEC>`"]
pub type EXTDST4_SAFETYMASK = crate::Reg<extdst4_safetymask::EXTDST4_SAFETYMASK_SPEC>;
#[doc = "Safety mask for extdst4"]
pub mod extdst4_safetymask;
#[doc = "EXTDST1_SAFETYMASK (rw) register accessor: an alias for `Reg<EXTDST1_SAFETYMASK_SPEC>`"]
pub type EXTDST1_SAFETYMASK = crate::Reg<extdst1_safetymask::EXTDST1_SAFETYMASK_SPEC>;
#[doc = "Safety mask for extdst1"]
pub mod extdst1_safetymask;
#[doc = "EXTDST5_SAFETYMASK (rw) register accessor: an alias for `Reg<EXTDST5_SAFETYMASK_SPEC>`"]
pub type EXTDST5_SAFETYMASK = crate::Reg<extdst5_safetymask::EXTDST5_SAFETYMASK_SPEC>;
#[doc = "Safety mask for extdst5"]
pub mod extdst5_safetymask;
#[doc = "STORE4_SAFETYMASK (rw) register accessor: an alias for `Reg<STORE4_SAFETYMASK_SPEC>`"]
pub type STORE4_SAFETYMASK = crate::Reg<store4_safetymask::STORE4_SAFETYMASK_SPEC>;
#[doc = "Safety mask for store4"]
pub mod store4_safetymask;
#[doc = "CONSTFRAME0_STATUS (r) register accessor: an alias for `Reg<CONSTFRAME0_STATUS_SPEC>`"]
pub type CONSTFRAME0_STATUS = crate::Reg<constframe0_status::CONSTFRAME0_STATUS_SPEC>;
#[doc = "Status information for pixel engine configuration of constframe0"]
pub mod constframe0_status;
#[doc = "EXTDST0_STATIC (rw) register accessor: an alias for `Reg<EXTDST0_STATIC_SPEC>`"]
pub type EXTDST0_STATIC = crate::Reg<extdst0_static::EXTDST0_STATIC_SPEC>;
#[doc = "Static pixel engine configuration for extdst0"]
pub mod extdst0_static;
#[doc = "EXTDST0_DYNAMIC (rw) register accessor: an alias for `Reg<EXTDST0_DYNAMIC_SPEC>`"]
pub type EXTDST0_DYNAMIC = crate::Reg<extdst0_dynamic::EXTDST0_DYNAMIC_SPEC>;
#[doc = "Dynamic pixel engine configuration for extdst0"]
pub mod extdst0_dynamic;
#[doc = "EXTDST0_REQUEST (rw) register accessor: an alias for `Reg<EXTDST0_REQUEST_SPEC>`"]
pub type EXTDST0_REQUEST = crate::Reg<extdst0_request::EXTDST0_REQUEST_SPEC>;
#[doc = "ShadowLoadRequest register for endpoint extdst0"]
pub mod extdst0_request;
#[doc = "EXTDST0_TRIGGER (w) register accessor: an alias for `Reg<EXTDST0_TRIGGER_SPEC>`"]
pub type EXTDST0_TRIGGER = crate::Reg<extdst0_trigger::EXTDST0_TRIGGER_SPEC>;
#[doc = "Trigger bits for pixel engine configuration of extdst0"]
pub mod extdst0_trigger;
#[doc = "EXTDST0_STATUS (r) register accessor: an alias for `Reg<EXTDST0_STATUS_SPEC>`"]
pub type EXTDST0_STATUS = crate::Reg<extdst0_status::EXTDST0_STATUS_SPEC>;
#[doc = "Status information for pixel engine configuration of extdst0"]
pub mod extdst0_status;
#[doc = "CONSTFRAME4_STATUS (r) register accessor: an alias for `Reg<CONSTFRAME4_STATUS_SPEC>`"]
pub type CONSTFRAME4_STATUS = crate::Reg<constframe4_status::CONSTFRAME4_STATUS_SPEC>;
#[doc = "Status information for pixel engine configuration of constframe4"]
pub mod constframe4_status;
#[doc = "EXTDST4_STATIC (rw) register accessor: an alias for `Reg<EXTDST4_STATIC_SPEC>`"]
pub type EXTDST4_STATIC = crate::Reg<extdst4_static::EXTDST4_STATIC_SPEC>;
#[doc = "Static pixel engine configuration for extdst4"]
pub mod extdst4_static;
#[doc = "EXTDST4_DYNAMIC (rw) register accessor: an alias for `Reg<EXTDST4_DYNAMIC_SPEC>`"]
pub type EXTDST4_DYNAMIC = crate::Reg<extdst4_dynamic::EXTDST4_DYNAMIC_SPEC>;
#[doc = "Dynamic pixel engine configuration for extdst4"]
pub mod extdst4_dynamic;
#[doc = "EXTDST4_REQUEST (rw) register accessor: an alias for `Reg<EXTDST4_REQUEST_SPEC>`"]
pub type EXTDST4_REQUEST = crate::Reg<extdst4_request::EXTDST4_REQUEST_SPEC>;
#[doc = "ShadowLoadRequest register for endpoint extdst4"]
pub mod extdst4_request;
#[doc = "EXTDST4_TRIGGER (w) register accessor: an alias for `Reg<EXTDST4_TRIGGER_SPEC>`"]
pub type EXTDST4_TRIGGER = crate::Reg<extdst4_trigger::EXTDST4_TRIGGER_SPEC>;
#[doc = "Trigger bits for pixel engine configuration of extdst4"]
pub mod extdst4_trigger;
#[doc = "EXTDST4_STATUS (r) register accessor: an alias for `Reg<EXTDST4_STATUS_SPEC>`"]
pub type EXTDST4_STATUS = crate::Reg<extdst4_status::EXTDST4_STATUS_SPEC>;
#[doc = "Status information for pixel engine configuration of extdst4"]
pub mod extdst4_status;
#[doc = "CONSTFRAME1_STATUS (r) register accessor: an alias for `Reg<CONSTFRAME1_STATUS_SPEC>`"]
pub type CONSTFRAME1_STATUS = crate::Reg<constframe1_status::CONSTFRAME1_STATUS_SPEC>;
#[doc = "Status information for pixel engine configuration of constframe1"]
pub mod constframe1_status;
#[doc = "EXTDST1_STATIC (rw) register accessor: an alias for `Reg<EXTDST1_STATIC_SPEC>`"]
pub type EXTDST1_STATIC = crate::Reg<extdst1_static::EXTDST1_STATIC_SPEC>;
#[doc = "Static pixel engine configuration for extdst1"]
pub mod extdst1_static;
#[doc = "EXTDST1_DYNAMIC (rw) register accessor: an alias for `Reg<EXTDST1_DYNAMIC_SPEC>`"]
pub type EXTDST1_DYNAMIC = crate::Reg<extdst1_dynamic::EXTDST1_DYNAMIC_SPEC>;
#[doc = "Dynamic pixel engine configuration for extdst1"]
pub mod extdst1_dynamic;
#[doc = "EXTDST1_REQUEST (rw) register accessor: an alias for `Reg<EXTDST1_REQUEST_SPEC>`"]
pub type EXTDST1_REQUEST = crate::Reg<extdst1_request::EXTDST1_REQUEST_SPEC>;
#[doc = "ShadowLoadRequest register for endpoint extdst1"]
pub mod extdst1_request;
#[doc = "EXTDST1_TRIGGER (w) register accessor: an alias for `Reg<EXTDST1_TRIGGER_SPEC>`"]
pub type EXTDST1_TRIGGER = crate::Reg<extdst1_trigger::EXTDST1_TRIGGER_SPEC>;
#[doc = "Trigger bits for pixel engine configuration of extdst1"]
pub mod extdst1_trigger;
#[doc = "EXTDST1_STATUS (r) register accessor: an alias for `Reg<EXTDST1_STATUS_SPEC>`"]
pub type EXTDST1_STATUS = crate::Reg<extdst1_status::EXTDST1_STATUS_SPEC>;
#[doc = "Status information for pixel engine configuration of extdst1"]
pub mod extdst1_status;
#[doc = "CONSTFRAME5_STATUS (r) register accessor: an alias for `Reg<CONSTFRAME5_STATUS_SPEC>`"]
pub type CONSTFRAME5_STATUS = crate::Reg<constframe5_status::CONSTFRAME5_STATUS_SPEC>;
#[doc = "Status information for pixel engine configuration of constframe5"]
pub mod constframe5_status;
#[doc = "EXTDST5_STATIC (rw) register accessor: an alias for `Reg<EXTDST5_STATIC_SPEC>`"]
pub type EXTDST5_STATIC = crate::Reg<extdst5_static::EXTDST5_STATIC_SPEC>;
#[doc = "Static pixel engine configuration for extdst5"]
pub mod extdst5_static;
#[doc = "EXTDST5_DYNAMIC (rw) register accessor: an alias for `Reg<EXTDST5_DYNAMIC_SPEC>`"]
pub type EXTDST5_DYNAMIC = crate::Reg<extdst5_dynamic::EXTDST5_DYNAMIC_SPEC>;
#[doc = "Dynamic pixel engine configuration for extdst5"]
pub mod extdst5_dynamic;
#[doc = "EXTDST5_REQUEST (rw) register accessor: an alias for `Reg<EXTDST5_REQUEST_SPEC>`"]
pub type EXTDST5_REQUEST = crate::Reg<extdst5_request::EXTDST5_REQUEST_SPEC>;
#[doc = "ShadowLoadRequest register for endpoint extdst5"]
pub mod extdst5_request;
#[doc = "EXTDST5_TRIGGER (w) register accessor: an alias for `Reg<EXTDST5_TRIGGER_SPEC>`"]
pub type EXTDST5_TRIGGER = crate::Reg<extdst5_trigger::EXTDST5_TRIGGER_SPEC>;
#[doc = "Trigger bits for pixel engine configuration of extdst5"]
pub mod extdst5_trigger;
#[doc = "EXTDST5_STATUS (r) register accessor: an alias for `Reg<EXTDST5_STATUS_SPEC>`"]
pub type EXTDST5_STATUS = crate::Reg<extdst5_status::EXTDST5_STATUS_SPEC>;
#[doc = "Status information for pixel engine configuration of extdst5"]
pub mod extdst5_status;
#[doc = "EXTSRC4_STATUS (r) register accessor: an alias for `Reg<EXTSRC4_STATUS_SPEC>`"]
pub type EXTSRC4_STATUS = crate::Reg<extsrc4_status::EXTSRC4_STATUS_SPEC>;
#[doc = "Status information for pixel engine configuration of extsrc4"]
pub mod extsrc4_status;
#[doc = "STORE4_STATIC (rw) register accessor: an alias for `Reg<STORE4_STATIC_SPEC>`"]
pub type STORE4_STATIC = crate::Reg<store4_static::STORE4_STATIC_SPEC>;
#[doc = "Static pixel engine configuration for store4"]
pub mod store4_static;
#[doc = "STORE4_DYNAMIC (rw) register accessor: an alias for `Reg<STORE4_DYNAMIC_SPEC>`"]
pub type STORE4_DYNAMIC = crate::Reg<store4_dynamic::STORE4_DYNAMIC_SPEC>;
#[doc = "Dynamic pixel engine configuration for store4"]
pub mod store4_dynamic;
#[doc = "STORE4_REQUEST (rw) register accessor: an alias for `Reg<STORE4_REQUEST_SPEC>`"]
pub type STORE4_REQUEST = crate::Reg<store4_request::STORE4_REQUEST_SPEC>;
#[doc = "ShadowLoadRequest register for endpoint store4"]
pub mod store4_request;
#[doc = "STORE4_TRIGGER (w) register accessor: an alias for `Reg<STORE4_TRIGGER_SPEC>`"]
pub type STORE4_TRIGGER = crate::Reg<store4_trigger::STORE4_TRIGGER_SPEC>;
#[doc = "Trigger bits for pixel engine configuration of store4"]
pub mod store4_trigger;
#[doc = "STORE4_STATUS (r) register accessor: an alias for `Reg<STORE4_STATUS_SPEC>`"]
pub type STORE4_STATUS = crate::Reg<store4_status::STORE4_STATUS_SPEC>;
#[doc = "Status information for pixel engine configuration of store4"]
pub mod store4_status;
#[doc = "FETCHLAYER0_STATUS (r) register accessor: an alias for `Reg<FETCHLAYER0_STATUS_SPEC>`"]
pub type FETCHLAYER0_STATUS = crate::Reg<fetchlayer0_status::FETCHLAYER0_STATUS_SPEC>;
#[doc = "Status information for pixel engine configuration of fetchlayer0"]
pub mod fetchlayer0_status;
#[doc = "FETCHDECODE4_DYNAMIC (rw) register accessor: an alias for `Reg<FETCHDECODE4_DYNAMIC_SPEC>`"]
pub type FETCHDECODE4_DYNAMIC = crate::Reg<fetchdecode4_dynamic::FETCHDECODE4_DYNAMIC_SPEC>;
#[doc = "Dynamic pixel engine configuration for fetchdecode4"]
pub mod fetchdecode4_dynamic;
#[doc = "FETCHDECODE4_STATUS (r) register accessor: an alias for `Reg<FETCHDECODE4_STATUS_SPEC>`"]
pub type FETCHDECODE4_STATUS = crate::Reg<fetchdecode4_status::FETCHDECODE4_STATUS_SPEC>;
#[doc = "Status information for pixel engine configuration of fetchdecode4"]
pub mod fetchdecode4_status;
#[doc = "FETCHECO4_STATUS (r) register accessor: an alias for `Reg<FETCHECO4_STATUS_SPEC>`"]
pub type FETCHECO4_STATUS = crate::Reg<fetcheco4_status::FETCHECO4_STATUS_SPEC>;
#[doc = "Status information for pixel engine configuration of fetcheco4"]
pub mod fetcheco4_status;
#[doc = "FETCHWARP1_DYNAMIC (rw) register accessor: an alias for `Reg<FETCHWARP1_DYNAMIC_SPEC>`"]
pub type FETCHWARP1_DYNAMIC = crate::Reg<fetchwarp1_dynamic::FETCHWARP1_DYNAMIC_SPEC>;
#[doc = "Dynamic pixel engine configuration for fetchwarp1"]
pub mod fetchwarp1_dynamic;
#[doc = "FETCHWARP1_STATUS (r) register accessor: an alias for `Reg<FETCHWARP1_STATUS_SPEC>`"]
pub type FETCHWARP1_STATUS = crate::Reg<fetchwarp1_status::FETCHWARP1_STATUS_SPEC>;
#[doc = "Status information for pixel engine configuration of fetchwarp1"]
pub mod fetchwarp1_status;
#[doc = "FETCHECO1_STATUS (r) register accessor: an alias for `Reg<FETCHECO1_STATUS_SPEC>`"]
pub type FETCHECO1_STATUS = crate::Reg<fetcheco1_status::FETCHECO1_STATUS_SPEC>;
#[doc = "Status information for pixel engine configuration of fetcheco1"]
pub mod fetcheco1_status;
#[doc = "FETCHLAYER1_STATUS (r) register accessor: an alias for `Reg<FETCHLAYER1_STATUS_SPEC>`"]
pub type FETCHLAYER1_STATUS = crate::Reg<fetchlayer1_status::FETCHLAYER1_STATUS_SPEC>;
#[doc = "Status information for pixel engine configuration of fetchlayer1"]
pub mod fetchlayer1_status;
#[doc = "FETCHDECODE0_DYNAMIC (rw) register accessor: an alias for `Reg<FETCHDECODE0_DYNAMIC_SPEC>`"]
pub type FETCHDECODE0_DYNAMIC = crate::Reg<fetchdecode0_dynamic::FETCHDECODE0_DYNAMIC_SPEC>;
#[doc = "Dynamic pixel engine configuration for fetchdecode0"]
pub mod fetchdecode0_dynamic;
#[doc = "FETCHDECODE0_STATUS (r) register accessor: an alias for `Reg<FETCHDECODE0_STATUS_SPEC>`"]
pub type FETCHDECODE0_STATUS = crate::Reg<fetchdecode0_status::FETCHDECODE0_STATUS_SPEC>;
#[doc = "Status information for pixel engine configuration of fetchdecode0"]
pub mod fetchdecode0_status;
#[doc = "GAMMACOR4_DYNAMIC (rw) register accessor: an alias for `Reg<GAMMACOR4_DYNAMIC_SPEC>`"]
pub type GAMMACOR4_DYNAMIC = crate::Reg<gammacor4_dynamic::GAMMACOR4_DYNAMIC_SPEC>;
#[doc = "Dynamic pixel engine configuration for gammacor4"]
pub mod gammacor4_dynamic;
#[doc = "GAMMACOR4_STATUS (r) register accessor: an alias for `Reg<GAMMACOR4_STATUS_SPEC>`"]
pub type GAMMACOR4_STATUS = crate::Reg<gammacor4_status::GAMMACOR4_STATUS_SPEC>;
#[doc = "Status information for pixel engine configuration of gammacor4"]
pub mod gammacor4_status;
#[doc = "MATRIX4_DYNAMIC (rw) register accessor: an alias for `Reg<MATRIX4_DYNAMIC_SPEC>`"]
pub type MATRIX4_DYNAMIC = crate::Reg<matrix4_dynamic::MATRIX4_DYNAMIC_SPEC>;
#[doc = "Dynamic pixel engine configuration for matrix4"]
pub mod matrix4_dynamic;
#[doc = "MATRIX4_STATUS (r) register accessor: an alias for `Reg<MATRIX4_STATUS_SPEC>`"]
pub type MATRIX4_STATUS = crate::Reg<matrix4_status::MATRIX4_STATUS_SPEC>;
#[doc = "Status information for pixel engine configuration of matrix4"]
pub mod matrix4_status;
#[doc = "GPSCALER4_DYNAMIC (rw) register accessor: an alias for `Reg<GPSCALER4_DYNAMIC_SPEC>`"]
pub type GPSCALER4_DYNAMIC = crate::Reg<gpscaler4_dynamic::GPSCALER4_DYNAMIC_SPEC>;
#[doc = "Dynamic pixel engine configuration for gpscaler4"]
pub mod gpscaler4_dynamic;
#[doc = "GPSCALER4_STATUS (r) register accessor: an alias for `Reg<GPSCALER4_STATUS_SPEC>`"]
pub type GPSCALER4_STATUS = crate::Reg<gpscaler4_status::GPSCALER4_STATUS_SPEC>;
#[doc = "Status information for pixel engine configuration of gpscaler4"]
pub mod gpscaler4_status;
#[doc = "HISTOGRAM4_DYNAMIC (rw) register accessor: an alias for `Reg<HISTOGRAM4_DYNAMIC_SPEC>`"]
pub type HISTOGRAM4_DYNAMIC = crate::Reg<histogram4_dynamic::HISTOGRAM4_DYNAMIC_SPEC>;
#[doc = "Dynamic pixel engine configuration for histogram4"]
pub mod histogram4_dynamic;
#[doc = "HISTOGRAM4_STATUS (r) register accessor: an alias for `Reg<HISTOGRAM4_STATUS_SPEC>`"]
pub type HISTOGRAM4_STATUS = crate::Reg<histogram4_status::HISTOGRAM4_STATUS_SPEC>;
#[doc = "Status information for pixel engine configuration of histogram4"]
pub mod histogram4_status;
#[doc = "LAYERBLEND1_DYNAMIC (rw) register accessor: an alias for `Reg<LAYERBLEND1_DYNAMIC_SPEC>`"]
pub type LAYERBLEND1_DYNAMIC = crate::Reg<layerblend1_dynamic::LAYERBLEND1_DYNAMIC_SPEC>;
#[doc = "Dynamic pixel engine configuration for layerblend1"]
pub mod layerblend1_dynamic;
#[doc = "LAYERBLEND1_STATUS (r) register accessor: an alias for `Reg<LAYERBLEND1_STATUS_SPEC>`"]
pub type LAYERBLEND1_STATUS = crate::Reg<layerblend1_status::LAYERBLEND1_STATUS_SPEC>;
#[doc = "Status information for pixel engine configuration of layerblend1"]
pub mod layerblend1_status;
#[doc = "LAYERBLEND2_DYNAMIC (rw) register accessor: an alias for `Reg<LAYERBLEND2_DYNAMIC_SPEC>`"]
pub type LAYERBLEND2_DYNAMIC = crate::Reg<layerblend2_dynamic::LAYERBLEND2_DYNAMIC_SPEC>;
#[doc = "Dynamic pixel engine configuration for layerblend2"]
pub mod layerblend2_dynamic;
#[doc = "LAYERBLEND2_STATUS (r) register accessor: an alias for `Reg<LAYERBLEND2_STATUS_SPEC>`"]
pub type LAYERBLEND2_STATUS = crate::Reg<layerblend2_status::LAYERBLEND2_STATUS_SPEC>;
#[doc = "Status information for pixel engine configuration of layerblend2"]
pub mod layerblend2_status;
#[doc = "LAYERBLEND3_DYNAMIC (rw) register accessor: an alias for `Reg<LAYERBLEND3_DYNAMIC_SPEC>`"]
pub type LAYERBLEND3_DYNAMIC = crate::Reg<layerblend3_dynamic::LAYERBLEND3_DYNAMIC_SPEC>;
#[doc = "Dynamic pixel engine configuration for layerblend3"]
pub mod layerblend3_dynamic;
#[doc = "LAYERBLEND3_STATUS (r) register accessor: an alias for `Reg<LAYERBLEND3_STATUS_SPEC>`"]
pub type LAYERBLEND3_STATUS = crate::Reg<layerblend3_status::LAYERBLEND3_STATUS_SPEC>;
#[doc = "Status information for pixel engine configuration of layerblend3"]
pub mod layerblend3_status;
#[doc = "LAYERBLEND4_DYNAMIC (rw) register accessor: an alias for `Reg<LAYERBLEND4_DYNAMIC_SPEC>`"]
pub type LAYERBLEND4_DYNAMIC = crate::Reg<layerblend4_dynamic::LAYERBLEND4_DYNAMIC_SPEC>;
#[doc = "Dynamic pixel engine configuration for layerblend4"]
pub mod layerblend4_dynamic;
#[doc = "LAYERBLEND4_STATUS (r) register accessor: an alias for `Reg<LAYERBLEND4_STATUS_SPEC>`"]
pub type LAYERBLEND4_STATUS = crate::Reg<layerblend4_status::LAYERBLEND4_STATUS_SPEC>;
#[doc = "Status information for pixel engine configuration of layerblend4"]
pub mod layerblend4_status;
#[doc = "LAYERBLEND5_DYNAMIC (rw) register accessor: an alias for `Reg<LAYERBLEND5_DYNAMIC_SPEC>`"]
pub type LAYERBLEND5_DYNAMIC = crate::Reg<layerblend5_dynamic::LAYERBLEND5_DYNAMIC_SPEC>;
#[doc = "Dynamic pixel engine configuration for layerblend5"]
pub mod layerblend5_dynamic;
#[doc = "LAYERBLEND5_STATUS (r) register accessor: an alias for `Reg<LAYERBLEND5_STATUS_SPEC>`"]
pub type LAYERBLEND5_STATUS = crate::Reg<layerblend5_status::LAYERBLEND5_STATUS_SPEC>;
#[doc = "Status information for pixel engine configuration of layerblend5"]
pub mod layerblend5_status;
#[doc = "EXTSRC8_STATUS (r) register accessor: an alias for `Reg<EXTSRC8_STATUS_SPEC>`"]
pub type EXTSRC8_STATUS = crate::Reg<extsrc8_status::EXTSRC8_STATUS_SPEC>;
#[doc = "Status information for pixel engine configuration of extsrc8"]
pub mod extsrc8_status;
