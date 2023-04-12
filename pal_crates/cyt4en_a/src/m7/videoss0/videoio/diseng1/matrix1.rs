#[doc = r"Register block"]
#[repr(C)]
pub struct MATRIX1 {
    #[doc = "0x00 - Color Matrix static control register"]
    pub staticcontrol: STATICCONTROL,
    #[doc = "0x04 - Color Matrix control register"]
    pub control: CONTROL,
    #[doc = "0x08 - Matrix values for calculation of the red output value."]
    pub red0: RED0,
    #[doc = "0x0c - Matrix values for calculation of the red output value."]
    pub red1: RED1,
    #[doc = "0x10 - Matrix values for calculation of the green output value."]
    pub green0: GREEN0,
    #[doc = "0x14 - Matrix values for calculation of the green output value."]
    pub green1: GREEN1,
    #[doc = "0x18 - Matrix values for calculation of the blue output value."]
    pub blue0: BLUE0,
    #[doc = "0x1c - Matrix values for calculation of the blue output value."]
    pub blue1: BLUE1,
    #[doc = "0x20 - Matrix values for calculation of the alpha output value."]
    pub alpha0: ALPHA0,
    #[doc = "0x24 - Matrix values for calculation of the alpha output value."]
    pub alpha1: ALPHA1,
    #[doc = "0x28 - Offset vectors for red and green output."]
    pub offsetvector0: OFFSETVECTOR0,
    #[doc = "0x2c - Offset vectors for blue and alpha output."]
    pub offsetvector1: OFFSETVECTOR1,
}
#[doc = "STATICCONTROL (rw) register accessor: an alias for `Reg<STATICCONTROL_SPEC>`"]
pub type STATICCONTROL = crate::Reg<staticcontrol::STATICCONTROL_SPEC>;
#[doc = "Color Matrix static control register"]
pub mod staticcontrol;
#[doc = "CONTROL (rw) register accessor: an alias for `Reg<CONTROL_SPEC>`"]
pub type CONTROL = crate::Reg<control::CONTROL_SPEC>;
#[doc = "Color Matrix control register"]
pub mod control;
#[doc = "RED0 (rw) register accessor: an alias for `Reg<RED0_SPEC>`"]
pub type RED0 = crate::Reg<red0::RED0_SPEC>;
#[doc = "Matrix values for calculation of the red output value."]
pub mod red0;
#[doc = "RED1 (rw) register accessor: an alias for `Reg<RED1_SPEC>`"]
pub type RED1 = crate::Reg<red1::RED1_SPEC>;
#[doc = "Matrix values for calculation of the red output value."]
pub mod red1;
#[doc = "GREEN0 (rw) register accessor: an alias for `Reg<GREEN0_SPEC>`"]
pub type GREEN0 = crate::Reg<green0::GREEN0_SPEC>;
#[doc = "Matrix values for calculation of the green output value."]
pub mod green0;
#[doc = "GREEN1 (rw) register accessor: an alias for `Reg<GREEN1_SPEC>`"]
pub type GREEN1 = crate::Reg<green1::GREEN1_SPEC>;
#[doc = "Matrix values for calculation of the green output value."]
pub mod green1;
#[doc = "BLUE0 (rw) register accessor: an alias for `Reg<BLUE0_SPEC>`"]
pub type BLUE0 = crate::Reg<blue0::BLUE0_SPEC>;
#[doc = "Matrix values for calculation of the blue output value."]
pub mod blue0;
#[doc = "BLUE1 (rw) register accessor: an alias for `Reg<BLUE1_SPEC>`"]
pub type BLUE1 = crate::Reg<blue1::BLUE1_SPEC>;
#[doc = "Matrix values for calculation of the blue output value."]
pub mod blue1;
#[doc = "ALPHA0 (rw) register accessor: an alias for `Reg<ALPHA0_SPEC>`"]
pub type ALPHA0 = crate::Reg<alpha0::ALPHA0_SPEC>;
#[doc = "Matrix values for calculation of the alpha output value."]
pub mod alpha0;
#[doc = "ALPHA1 (rw) register accessor: an alias for `Reg<ALPHA1_SPEC>`"]
pub type ALPHA1 = crate::Reg<alpha1::ALPHA1_SPEC>;
#[doc = "Matrix values for calculation of the alpha output value."]
pub mod alpha1;
#[doc = "OFFSETVECTOR0 (rw) register accessor: an alias for `Reg<OFFSETVECTOR0_SPEC>`"]
pub type OFFSETVECTOR0 = crate::Reg<offsetvector0::OFFSETVECTOR0_SPEC>;
#[doc = "Offset vectors for red and green output."]
pub mod offsetvector0;
#[doc = "OFFSETVECTOR1 (rw) register accessor: an alias for `Reg<OFFSETVECTOR1_SPEC>`"]
pub type OFFSETVECTOR1 = crate::Reg<offsetvector1::OFFSETVECTOR1_SPEC>;
#[doc = "Offset vectors for blue and alpha output."]
pub mod offsetvector1;
