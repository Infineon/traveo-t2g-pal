#[doc = r"Register block"]
#[repr(C)]
pub struct CLUT {
    #[doc = "0x00 - CLUT static control register"]
    pub staticcontrol: STATICCONTROL,
    #[doc = "0x04 - CLUT unshadowed control register"]
    pub unshadowedcontrol: UNSHADOWEDCONTROL,
    #[doc = "0x08 - CLUT control register"]
    pub control: CONTROL,
    #[doc = "0x0c - CLUT status register"]
    pub status: STATUS,
    #[doc = "0x10 - Value of last received control word, for debugging"]
    pub lastcontrolword: LASTCONTROLWORD,
    _reserved5: [u8; 0x03ec],
    #[doc = "0x400..0x800 - Look Up Table"]
    pub lut: [LUT; 256],
}
#[doc = "STATICCONTROL (rw) register accessor: an alias for `Reg<STATICCONTROL_SPEC>`"]
pub type STATICCONTROL = crate::Reg<staticcontrol::STATICCONTROL_SPEC>;
#[doc = "CLUT static control register"]
pub mod staticcontrol;
#[doc = "UNSHADOWEDCONTROL (rw) register accessor: an alias for `Reg<UNSHADOWEDCONTROL_SPEC>`"]
pub type UNSHADOWEDCONTROL = crate::Reg<unshadowedcontrol::UNSHADOWEDCONTROL_SPEC>;
#[doc = "CLUT unshadowed control register"]
pub mod unshadowedcontrol;
#[doc = "CONTROL (rw) register accessor: an alias for `Reg<CONTROL_SPEC>`"]
pub type CONTROL = crate::Reg<control::CONTROL_SPEC>;
#[doc = "CLUT control register"]
pub mod control;
#[doc = "STATUS (rw) register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "CLUT status register"]
pub mod status;
#[doc = "LASTCONTROLWORD (r) register accessor: an alias for `Reg<LASTCONTROLWORD_SPEC>`"]
pub type LASTCONTROLWORD = crate::Reg<lastcontrolword::LASTCONTROLWORD_SPEC>;
#[doc = "Value of last received control word, for debugging"]
pub mod lastcontrolword;
#[doc = "LUT (rw) register accessor: an alias for `Reg<LUT_SPEC>`"]
pub type LUT = crate::Reg<lut::LUT_SPEC>;
#[doc = "Look Up Table"]
pub mod lut;
