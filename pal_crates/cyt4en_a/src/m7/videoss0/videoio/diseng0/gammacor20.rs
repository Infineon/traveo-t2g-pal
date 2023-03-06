#[doc = r"Register block"]
#[repr(C)]
pub struct GAMMACOR20 {
    #[doc = "0x00 - Static control settings."]
    pub staticcontrol: STATICCONTROL,
    #[doc = "0x04 - Start values for look-up table programming."]
    pub lutstart: LUTSTART,
    #[doc = "0x08 - Delta values for look-up table programming."]
    pub lutdeltas: LUTDELTAS,
    #[doc = "0x0c - Dynamic control settings."]
    pub control: CONTROL,
}
#[doc = "STATICCONTROL (rw) register accessor: an alias for `Reg<STATICCONTROL_SPEC>`"]
pub type STATICCONTROL = crate::Reg<staticcontrol::STATICCONTROL_SPEC>;
#[doc = "Static control settings."]
pub mod staticcontrol;
#[doc = "LUTSTART (w) register accessor: an alias for `Reg<LUTSTART_SPEC>`"]
pub type LUTSTART = crate::Reg<lutstart::LUTSTART_SPEC>;
#[doc = "Start values for look-up table programming."]
pub mod lutstart;
#[doc = "LUTDELTAS (w) register accessor: an alias for `Reg<LUTDELTAS_SPEC>`"]
pub type LUTDELTAS = crate::Reg<lutdeltas::LUTDELTAS_SPEC>;
#[doc = "Delta values for look-up table programming."]
pub mod lutdeltas;
#[doc = "CONTROL (rw) register accessor: an alias for `Reg<CONTROL_SPEC>`"]
pub type CONTROL = crate::Reg<control::CONTROL_SPEC>;
#[doc = "Dynamic control settings."]
pub mod control;
