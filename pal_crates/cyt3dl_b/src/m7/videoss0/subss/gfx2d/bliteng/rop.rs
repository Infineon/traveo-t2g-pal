#[doc = r"Register block"]
#[repr(C)]
pub struct ROP {
    #[doc = "0x00 - Raster Operation static control register"]
    pub staticcontrol: STATICCONTROL,
    #[doc = "0x04 - Raster Operation control register"]
    pub control: CONTROL,
    #[doc = "0x08 - ROP operation indices"]
    pub rasteroperationindices: RASTEROPERATIONINDICES,
    #[doc = "0x0c - Value of last received primary control word"]
    pub primcontrolword: PRIMCONTROLWORD,
    #[doc = "0x10 - Value of last received secondary control word"]
    pub seccontrolword: SECCONTROLWORD,
    #[doc = "0x14 - Value of last received tertiary control word"]
    pub tertcontrolword: TERTCONTROLWORD,
}
#[doc = "STATICCONTROL (rw) register accessor: an alias for `Reg<STATICCONTROL_SPEC>`"]
pub type STATICCONTROL = crate::Reg<staticcontrol::STATICCONTROL_SPEC>;
#[doc = "Raster Operation static control register"]
pub mod staticcontrol;
#[doc = "CONTROL (rw) register accessor: an alias for `Reg<CONTROL_SPEC>`"]
pub type CONTROL = crate::Reg<control::CONTROL_SPEC>;
#[doc = "Raster Operation control register"]
pub mod control;
#[doc = "RASTEROPERATIONINDICES (rw) register accessor: an alias for `Reg<RASTEROPERATIONINDICES_SPEC>`"]
pub type RASTEROPERATIONINDICES = crate::Reg<rasteroperationindices::RASTEROPERATIONINDICES_SPEC>;
#[doc = "ROP operation indices"]
pub mod rasteroperationindices;
#[doc = "PRIMCONTROLWORD (r) register accessor: an alias for `Reg<PRIMCONTROLWORD_SPEC>`"]
pub type PRIMCONTROLWORD = crate::Reg<primcontrolword::PRIMCONTROLWORD_SPEC>;
#[doc = "Value of last received primary control word"]
pub mod primcontrolword;
#[doc = "SECCONTROLWORD (r) register accessor: an alias for `Reg<SECCONTROLWORD_SPEC>`"]
pub type SECCONTROLWORD = crate::Reg<seccontrolword::SECCONTROLWORD_SPEC>;
#[doc = "Value of last received secondary control word"]
pub mod seccontrolword;
#[doc = "TERTCONTROLWORD (r) register accessor: an alias for `Reg<TERTCONTROLWORD_SPEC>`"]
pub type TERTCONTROLWORD = crate::Reg<tertcontrolword::TERTCONTROLWORD_SPEC>;
#[doc = "Value of last received tertiary control word"]
pub mod tertcontrolword;
