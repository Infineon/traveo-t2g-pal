#[doc = r"Register block"]
#[repr(C)]
pub struct BLITBLEND {
    #[doc = "0x00 - BlitBlend static control register"]
    pub staticcontrol: STATICCONTROL,
    #[doc = "0x04 - BlitBlend control register"]
    pub control: CONTROL,
    #[doc = "0x08 - Neutral border setup register"]
    pub neutralborder: NEUTRALBORDER,
    #[doc = "0x0c - Constant color register"]
    pub constantcolor: CONSTANTCOLOR,
    #[doc = "0x10 - Open GL RGB blending factors"]
    pub colorredblendfunction: COLORREDBLENDFUNCTION,
    #[doc = "0x14 - Open GL RGB blending factors"]
    pub colorgreenblendfunction: COLORGREENBLENDFUNCTION,
    #[doc = "0x18 - Open GL RGB blending factors"]
    pub colorblueblendfunction: COLORBLUEBLENDFUNCTION,
    #[doc = "0x1c - Open GL alpha blending factors"]
    pub alphablendfunction: ALPHABLENDFUNCTION,
    #[doc = "0x20 - Open GL and Open VG blending modes for colors red and green"]
    pub blendmode1: BLENDMODE1,
    #[doc = "0x24 - Open GL and Open VG blending modes for color blue and alpha"]
    pub blendmode2: BLENDMODE2,
    #[doc = "0x28 - Direct Control of the BlitBlend Datapath multiplexers, do not change"]
    pub directsetup: DIRECTSETUP,
    #[doc = "0x2c - Value of last received primary control word"]
    pub primcontrolword: PRIMCONTROLWORD,
    #[doc = "0x30 - Value of last received secondary control word"]
    pub seccontrolword: SECCONTROLWORD,
}
#[doc = "STATICCONTROL (rw) register accessor: an alias for `Reg<STATICCONTROL_SPEC>`"]
pub type STATICCONTROL = crate::Reg<staticcontrol::STATICCONTROL_SPEC>;
#[doc = "BlitBlend static control register"]
pub mod staticcontrol;
#[doc = "CONTROL (rw) register accessor: an alias for `Reg<CONTROL_SPEC>`"]
pub type CONTROL = crate::Reg<control::CONTROL_SPEC>;
#[doc = "BlitBlend control register"]
pub mod control;
#[doc = "NEUTRALBORDER (rw) register accessor: an alias for `Reg<NEUTRALBORDER_SPEC>`"]
pub type NEUTRALBORDER = crate::Reg<neutralborder::NEUTRALBORDER_SPEC>;
#[doc = "Neutral border setup register"]
pub mod neutralborder;
#[doc = "CONSTANTCOLOR (rw) register accessor: an alias for `Reg<CONSTANTCOLOR_SPEC>`"]
pub type CONSTANTCOLOR = crate::Reg<constantcolor::CONSTANTCOLOR_SPEC>;
#[doc = "Constant color register"]
pub mod constantcolor;
#[doc = "COLORREDBLENDFUNCTION (rw) register accessor: an alias for `Reg<COLORREDBLENDFUNCTION_SPEC>`"]
pub type COLORREDBLENDFUNCTION = crate::Reg<colorredblendfunction::COLORREDBLENDFUNCTION_SPEC>;
#[doc = "Open GL RGB blending factors"]
pub mod colorredblendfunction;
#[doc = "COLORGREENBLENDFUNCTION (rw) register accessor: an alias for `Reg<COLORGREENBLENDFUNCTION_SPEC>`"]
pub type COLORGREENBLENDFUNCTION =
    crate::Reg<colorgreenblendfunction::COLORGREENBLENDFUNCTION_SPEC>;
#[doc = "Open GL RGB blending factors"]
pub mod colorgreenblendfunction;
#[doc = "COLORBLUEBLENDFUNCTION (rw) register accessor: an alias for `Reg<COLORBLUEBLENDFUNCTION_SPEC>`"]
pub type COLORBLUEBLENDFUNCTION = crate::Reg<colorblueblendfunction::COLORBLUEBLENDFUNCTION_SPEC>;
#[doc = "Open GL RGB blending factors"]
pub mod colorblueblendfunction;
#[doc = "ALPHABLENDFUNCTION (rw) register accessor: an alias for `Reg<ALPHABLENDFUNCTION_SPEC>`"]
pub type ALPHABLENDFUNCTION = crate::Reg<alphablendfunction::ALPHABLENDFUNCTION_SPEC>;
#[doc = "Open GL alpha blending factors"]
pub mod alphablendfunction;
#[doc = "BLENDMODE1 (rw) register accessor: an alias for `Reg<BLENDMODE1_SPEC>`"]
pub type BLENDMODE1 = crate::Reg<blendmode1::BLENDMODE1_SPEC>;
#[doc = "Open GL and Open VG blending modes for colors red and green"]
pub mod blendmode1;
#[doc = "BLENDMODE2 (rw) register accessor: an alias for `Reg<BLENDMODE2_SPEC>`"]
pub type BLENDMODE2 = crate::Reg<blendmode2::BLENDMODE2_SPEC>;
#[doc = "Open GL and Open VG blending modes for color blue and alpha"]
pub mod blendmode2;
#[doc = "DIRECTSETUP (rw) register accessor: an alias for `Reg<DIRECTSETUP_SPEC>`"]
pub type DIRECTSETUP = crate::Reg<directsetup::DIRECTSETUP_SPEC>;
#[doc = "Direct Control of the BlitBlend Datapath multiplexers, do not change"]
pub mod directsetup;
#[doc = "PRIMCONTROLWORD (r) register accessor: an alias for `Reg<PRIMCONTROLWORD_SPEC>`"]
pub type PRIMCONTROLWORD = crate::Reg<primcontrolword::PRIMCONTROLWORD_SPEC>;
#[doc = "Value of last received primary control word"]
pub mod primcontrolword;
#[doc = "SECCONTROLWORD (r) register accessor: an alias for `Reg<SECCONTROLWORD_SPEC>`"]
pub type SECCONTROLWORD = crate::Reg<seccontrolword::SECCONTROLWORD_SPEC>;
#[doc = "Value of last received secondary control word"]
pub mod seccontrolword;
