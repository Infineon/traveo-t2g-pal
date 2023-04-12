#[doc = r"Register block"]
#[repr(C)]
pub struct DITHER1 {
    #[doc = "0x00 - Dither Unit common control."]
    pub control: CONTROL,
    #[doc = "0x04 - Dither Unit processing control."]
    pub dithercontrol: DITHERCONTROL,
    #[doc = "0x08 - Dither Unit release."]
    pub release: RELEASE,
}
#[doc = "CONTROL (rw) register accessor: an alias for `Reg<CONTROL_SPEC>`"]
pub type CONTROL = crate::Reg<control::CONTROL_SPEC>;
#[doc = "Dither Unit common control."]
pub mod control;
#[doc = "DITHERCONTROL (rw) register accessor: an alias for `Reg<DITHERCONTROL_SPEC>`"]
pub type DITHERCONTROL = crate::Reg<dithercontrol::DITHERCONTROL_SPEC>;
#[doc = "Dither Unit processing control."]
pub mod dithercontrol;
#[doc = "RELEASE (r) register accessor: an alias for `Reg<RELEASE_SPEC>`"]
pub type RELEASE = crate::Reg<release::RELEASE_SPEC>;
#[doc = "Dither Unit release."]
pub mod release;
