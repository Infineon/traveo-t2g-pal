#[doc = r"Register block"]
#[repr(C)]
pub struct LAYERBLEND1 {
    #[doc = "0x00 - Static control settings."]
    pub staticcontrol: STATICCONTROL,
    #[doc = "0x04 - Common control settings."]
    pub control: CONTROL,
    #[doc = "0x08 - Options for blend operations"]
    pub blendcontrol: BLENDCONTROL,
    #[doc = "0x0c - Position of secondary (overlay) input frame"]
    pub position: POSITION,
}
#[doc = "STATICCONTROL (rw) register accessor: an alias for `Reg<STATICCONTROL_SPEC>`"]
pub type STATICCONTROL = crate::Reg<staticcontrol::STATICCONTROL_SPEC>;
#[doc = "Static control settings."]
pub mod staticcontrol;
#[doc = "CONTROL (rw) register accessor: an alias for `Reg<CONTROL_SPEC>`"]
pub type CONTROL = crate::Reg<control::CONTROL_SPEC>;
#[doc = "Common control settings."]
pub mod control;
#[doc = "BLENDCONTROL (rw) register accessor: an alias for `Reg<BLENDCONTROL_SPEC>`"]
pub type BLENDCONTROL = crate::Reg<blendcontrol::BLENDCONTROL_SPEC>;
#[doc = "Options for blend operations"]
pub mod blendcontrol;
#[doc = "POSITION (rw) register accessor: an alias for `Reg<POSITION_SPEC>`"]
pub type POSITION = crate::Reg<position::POSITION_SPEC>;
#[doc = "Position of secondary (overlay) input frame"]
pub mod position;
