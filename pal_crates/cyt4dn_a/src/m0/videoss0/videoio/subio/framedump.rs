#[doc = r"Register block"]
#[repr(C)]
pub struct FRAMEDUMP {
    #[doc = "0x00 - Control settings."]
    pub control: CONTROL,
    #[doc = "0x04 - Dump window size."]
    pub framesize: FRAMESIZE,
    #[doc = "0x08 - Dump window position."]
    pub inputsetup: INPUTSETUP,
    #[doc = "0x0c - Enable register."]
    pub enable: ENABLE,
    #[doc = "0x10 - Trigger bits."]
    pub trigger: TRIGGER,
    #[doc = "0x14 - Status register."]
    pub status: STATUS,
}
#[doc = "CONTROL (rw) register accessor: an alias for `Reg<CONTROL_SPEC>`"]
pub type CONTROL = crate::Reg<control::CONTROL_SPEC>;
#[doc = "Control settings."]
pub mod control;
#[doc = "FRAMESIZE (rw) register accessor: an alias for `Reg<FRAMESIZE_SPEC>`"]
pub type FRAMESIZE = crate::Reg<framesize::FRAMESIZE_SPEC>;
#[doc = "Dump window size."]
pub mod framesize;
#[doc = "INPUTSETUP (rw) register accessor: an alias for `Reg<INPUTSETUP_SPEC>`"]
pub type INPUTSETUP = crate::Reg<inputsetup::INPUTSETUP_SPEC>;
#[doc = "Dump window position."]
pub mod inputsetup;
#[doc = "ENABLE (rw) register accessor: an alias for `Reg<ENABLE_SPEC>`"]
pub type ENABLE = crate::Reg<enable::ENABLE_SPEC>;
#[doc = "Enable register."]
pub mod enable;
#[doc = "TRIGGER (rw) register accessor: an alias for `Reg<TRIGGER_SPEC>`"]
pub type TRIGGER = crate::Reg<trigger::TRIGGER_SPEC>;
#[doc = "Trigger bits."]
pub mod trigger;
#[doc = "STATUS (rw) register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Status register."]
pub mod status;
