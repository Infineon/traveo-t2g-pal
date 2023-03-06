#[doc = r"Register block"]
#[repr(C)]
pub struct EXTDST1 {
    #[doc = "0x00 - External Destination static control register"]
    pub staticcontrol: STATICCONTROL,
    #[doc = "0x04 - External Destination shadowed control register"]
    pub control: CONTROL,
    #[doc = "0x08 - External Destination software kick"]
    pub softwarekick: SOFTWAREKICK,
    #[doc = "0x0c - External Destination Unit current status"]
    pub status: STATUS,
    #[doc = "0x10 - Value of last received control word"]
    pub controlword: CONTROLWORD,
    #[doc = "0x14 - pixel count of currently running frame"]
    pub curpixelcnt: CURPIXELCNT,
    #[doc = "0x18 - pixel count between last two control words"]
    pub lastpixelcnt: LASTPIXELCNT,
    #[doc = "0x1c - Performance counter result"]
    pub perfcounter: PERFCOUNTER,
}
#[doc = "STATICCONTROL (rw) register accessor: an alias for `Reg<STATICCONTROL_SPEC>`"]
pub type STATICCONTROL = crate::Reg<staticcontrol::STATICCONTROL_SPEC>;
#[doc = "External Destination static control register"]
pub mod staticcontrol;
#[doc = "CONTROL (rw) register accessor: an alias for `Reg<CONTROL_SPEC>`"]
pub type CONTROL = crate::Reg<control::CONTROL_SPEC>;
#[doc = "External Destination shadowed control register"]
pub mod control;
#[doc = "SOFTWAREKICK (w) register accessor: an alias for `Reg<SOFTWAREKICK_SPEC>`"]
pub type SOFTWAREKICK = crate::Reg<softwarekick::SOFTWAREKICK_SPEC>;
#[doc = "External Destination software kick"]
pub mod softwarekick;
#[doc = "STATUS (rw) register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "External Destination Unit current status"]
pub mod status;
#[doc = "CONTROLWORD (r) register accessor: an alias for `Reg<CONTROLWORD_SPEC>`"]
pub type CONTROLWORD = crate::Reg<controlword::CONTROLWORD_SPEC>;
#[doc = "Value of last received control word"]
pub mod controlword;
#[doc = "CURPIXELCNT (r) register accessor: an alias for `Reg<CURPIXELCNT_SPEC>`"]
pub type CURPIXELCNT = crate::Reg<curpixelcnt::CURPIXELCNT_SPEC>;
#[doc = "pixel count of currently running frame"]
pub mod curpixelcnt;
#[doc = "LASTPIXELCNT (r) register accessor: an alias for `Reg<LASTPIXELCNT_SPEC>`"]
pub type LASTPIXELCNT = crate::Reg<lastpixelcnt::LASTPIXELCNT_SPEC>;
#[doc = "pixel count between last two control words"]
pub mod lastpixelcnt;
#[doc = "PERFCOUNTER (r) register accessor: an alias for `Reg<PERFCOUNTER_SPEC>`"]
pub type PERFCOUNTER = crate::Reg<perfcounter::PERFCOUNTER_SPEC>;
#[doc = "Performance counter result"]
pub mod perfcounter;
