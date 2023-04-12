#[doc = r"Register block"]
#[repr(C)]
pub struct SIG0 {
    #[doc = "0x00 - Global configuration shared by all evaluation windows."]
    pub staticcontrol: STATICCONTROL,
    #[doc = "0x04 - Overlay color for evaluation windows in panic mode."]
    pub paniccolor: PANICCOLOR,
    #[doc = "0x08 - Control settings for evaluation window 0."]
    pub evalcontrol0: EVALCONTROL0,
    #[doc = "0x0c - Upper left corner of evaluation window 0."]
    pub evalupperleft0: EVALUPPERLEFT0,
    #[doc = "0x10 - Lower right corner of evaluation window 0."]
    pub evallowerright0: EVALLOWERRIGHT0,
    #[doc = "0x14 - Reference signature of red channel for evaluation window 0."]
    pub sigcrcredref0: SIGCRCREDREF0,
    #[doc = "0x18 - Reference signature of green channel for evaluation window 0."]
    pub sigcrcgreenref0: SIGCRCGREENREF0,
    #[doc = "0x1c - Reference signature of blue channel for evaluation window 0."]
    pub sigcrcblueref0: SIGCRCBLUEREF0,
    #[doc = "0x20 - Measured signature of red channel for evaluation window 0."]
    pub sigcrcred0: SIGCRCRED0,
    #[doc = "0x24 - Measured signature of green channel for evaluation window 0."]
    pub sigcrcgreen0: SIGCRCGREEN0,
    #[doc = "0x28 - Measured signature of blue channel for evaluation window 0."]
    pub sigcrcblue0: SIGCRCBLUE0,
    #[doc = "0x2c - Control settings for evaluation window 1."]
    pub evalcontrol1: EVALCONTROL1,
    #[doc = "0x30 - Upper left corner of evaluation window 1."]
    pub evalupperleft1: EVALUPPERLEFT1,
    #[doc = "0x34 - Lower right corner of evaluation window 1."]
    pub evallowerright1: EVALLOWERRIGHT1,
    #[doc = "0x38 - Reference signature of red channel for evaluation window 1."]
    pub sigcrcredref1: SIGCRCREDREF1,
    #[doc = "0x3c - Reference signature of green channel for evaluation window 1."]
    pub sigcrcgreenref1: SIGCRCGREENREF1,
    #[doc = "0x40 - Reference signature of blue channel for evaluation window 1."]
    pub sigcrcblueref1: SIGCRCBLUEREF1,
    #[doc = "0x44 - Measured signature of red channel for evaluation window 1."]
    pub sigcrcred1: SIGCRCRED1,
    #[doc = "0x48 - Measured signature of green channel for evaluation window 1."]
    pub sigcrcgreen1: SIGCRCGREEN1,
    #[doc = "0x4c - Measured signature of blue channel for evaluation window 1."]
    pub sigcrcblue1: SIGCRCBLUE1,
    #[doc = "0x50 - Control settings for evaluation window 2."]
    pub evalcontrol2: EVALCONTROL2,
    #[doc = "0x54 - Upper left corner of evaluation window 2."]
    pub evalupperleft2: EVALUPPERLEFT2,
    #[doc = "0x58 - Lower right corner of evaluation window 2."]
    pub evallowerright2: EVALLOWERRIGHT2,
    #[doc = "0x5c - Reference signature of red channel for evaluation window 2."]
    pub sigcrcredref2: SIGCRCREDREF2,
    #[doc = "0x60 - Reference signature of green channel for evaluation window 2."]
    pub sigcrcgreenref2: SIGCRCGREENREF2,
    #[doc = "0x64 - Reference signature of blue channel for evaluation window 2."]
    pub sigcrcblueref2: SIGCRCBLUEREF2,
    #[doc = "0x68 - Measured signature of red channel for evaluation window 2."]
    pub sigcrcred2: SIGCRCRED2,
    #[doc = "0x6c - Measured signature of green channel for evaluation window 2."]
    pub sigcrcgreen2: SIGCRCGREEN2,
    #[doc = "0x70 - Measured signature of blue channel for evaluation window 2."]
    pub sigcrcblue2: SIGCRCBLUE2,
    #[doc = "0x74 - Control settings for evaluation window 3."]
    pub evalcontrol3: EVALCONTROL3,
    #[doc = "0x78 - Upper left corner of evaluation window 3."]
    pub evalupperleft3: EVALUPPERLEFT3,
    #[doc = "0x7c - Lower right corner of evaluation window 3."]
    pub evallowerright3: EVALLOWERRIGHT3,
    #[doc = "0x80 - Reference signature of red channel for evaluation window 3."]
    pub sigcrcredref3: SIGCRCREDREF3,
    #[doc = "0x84 - Reference signature of green channel for evaluation window 3."]
    pub sigcrcgreenref3: SIGCRCGREENREF3,
    #[doc = "0x88 - Reference signature of blue channel for evaluation window 3."]
    pub sigcrcblueref3: SIGCRCBLUEREF3,
    #[doc = "0x8c - Measured signature of red channel for evaluation window 3."]
    pub sigcrcred3: SIGCRCRED3,
    #[doc = "0x90 - Measured signature of green channel for evaluation window 3."]
    pub sigcrcgreen3: SIGCRCGREEN3,
    #[doc = "0x94 - Measured signature of blue channel for evaluation window 3."]
    pub sigcrcblue3: SIGCRCBLUE3,
    #[doc = "0x98 - Control settings for evaluation window 4."]
    pub evalcontrol4: EVALCONTROL4,
    #[doc = "0x9c - Upper left corner of evaluation window 4."]
    pub evalupperleft4: EVALUPPERLEFT4,
    #[doc = "0xa0 - Lower right corner of evaluation window 4."]
    pub evallowerright4: EVALLOWERRIGHT4,
    #[doc = "0xa4 - Reference signature of red channel for evaluation window 4."]
    pub sigcrcredref4: SIGCRCREDREF4,
    #[doc = "0xa8 - Reference signature of green channel for evaluation window 4."]
    pub sigcrcgreenref4: SIGCRCGREENREF4,
    #[doc = "0xac - Reference signature of blue channel for evaluation window 4."]
    pub sigcrcblueref4: SIGCRCBLUEREF4,
    #[doc = "0xb0 - Measured signature of red channel for evaluation window 4."]
    pub sigcrcred4: SIGCRCRED4,
    #[doc = "0xb4 - Measured signature of green channel for evaluation window 4."]
    pub sigcrcgreen4: SIGCRCGREEN4,
    #[doc = "0xb8 - Measured signature of blue channel for evaluation window 4."]
    pub sigcrcblue4: SIGCRCBLUE4,
    #[doc = "0xbc - Control settings for evaluation window 5."]
    pub evalcontrol5: EVALCONTROL5,
    #[doc = "0xc0 - Upper left corner of evaluation window 5."]
    pub evalupperleft5: EVALUPPERLEFT5,
    #[doc = "0xc4 - Lower right corner of evaluation window 5."]
    pub evallowerright5: EVALLOWERRIGHT5,
    #[doc = "0xc8 - Reference signature of red channel for evaluation window 5."]
    pub sigcrcredref5: SIGCRCREDREF5,
    #[doc = "0xcc - Reference signature of green channel for evaluation window 5."]
    pub sigcrcgreenref5: SIGCRCGREENREF5,
    #[doc = "0xd0 - Reference signature of blue channel for evaluation window 5."]
    pub sigcrcblueref5: SIGCRCBLUEREF5,
    #[doc = "0xd4 - Measured signature of red channel for evaluation window 5."]
    pub sigcrcred5: SIGCRCRED5,
    #[doc = "0xd8 - Measured signature of green channel for evaluation window 5."]
    pub sigcrcgreen5: SIGCRCGREEN5,
    #[doc = "0xdc - Measured signature of blue channel for evaluation window 5."]
    pub sigcrcblue5: SIGCRCBLUE5,
    #[doc = "0xe0 - Control settings for evaluation window 6."]
    pub evalcontrol6: EVALCONTROL6,
    #[doc = "0xe4 - Upper left corner of evaluation window 6."]
    pub evalupperleft6: EVALUPPERLEFT6,
    #[doc = "0xe8 - Lower right corner of evaluation window 6."]
    pub evallowerright6: EVALLOWERRIGHT6,
    #[doc = "0xec - Reference signature of red channel for evaluation window 6."]
    pub sigcrcredref6: SIGCRCREDREF6,
    #[doc = "0xf0 - Reference signature of green channel for evaluation window 6."]
    pub sigcrcgreenref6: SIGCRCGREENREF6,
    #[doc = "0xf4 - Reference signature of blue channel for evaluation window 6."]
    pub sigcrcblueref6: SIGCRCBLUEREF6,
    #[doc = "0xf8 - Measured signature of red channel for evaluation window 6."]
    pub sigcrcred6: SIGCRCRED6,
    #[doc = "0xfc - Measured signature of green channel for evaluation window 6."]
    pub sigcrcgreen6: SIGCRCGREEN6,
    #[doc = "0x100 - Measured signature of blue channel for evaluation window 6."]
    pub sigcrcblue6: SIGCRCBLUE6,
    #[doc = "0x104 - Control settings for evaluation window 7."]
    pub evalcontrol7: EVALCONTROL7,
    #[doc = "0x108 - Upper left corner of evaluation window 7."]
    pub evalupperleft7: EVALUPPERLEFT7,
    #[doc = "0x10c - Lower right corner of evaluation window 7."]
    pub evallowerright7: EVALLOWERRIGHT7,
    #[doc = "0x110 - Reference signature of red channel for evaluation window 7."]
    pub sigcrcredref7: SIGCRCREDREF7,
    #[doc = "0x114 - Reference signature of green channel for evaluation window 7."]
    pub sigcrcgreenref7: SIGCRCGREENREF7,
    #[doc = "0x118 - Reference signature of blue channel for evaluation window 7."]
    pub sigcrcblueref7: SIGCRCBLUEREF7,
    #[doc = "0x11c - Measured signature of red channel for evaluation window 7."]
    pub sigcrcred7: SIGCRCRED7,
    #[doc = "0x120 - Measured signature of green channel for evaluation window 7."]
    pub sigcrcgreen7: SIGCRCGREEN7,
    #[doc = "0x124 - Measured signature of blue channel for evaluation window 7."]
    pub sigcrcblue7: SIGCRCBLUE7,
    #[doc = "0x128 - Shadow load control register."]
    pub shadowload: SHADOWLOAD,
    #[doc = "0x12c - Signature operation mode control."]
    pub continuousmode: CONTINUOUSMODE,
    #[doc = "0x130 - Signature measurement trigger."]
    pub softwarekick: SOFTWAREKICK,
    #[doc = "0x134 - Module status."]
    pub status: STATUS,
}
#[doc = "STATICCONTROL (rw) register accessor: an alias for `Reg<STATICCONTROL_SPEC>`"]
pub type STATICCONTROL = crate::Reg<staticcontrol::STATICCONTROL_SPEC>;
#[doc = "Global configuration shared by all evaluation windows."]
pub mod staticcontrol;
#[doc = "PANICCOLOR (rw) register accessor: an alias for `Reg<PANICCOLOR_SPEC>`"]
pub type PANICCOLOR = crate::Reg<paniccolor::PANICCOLOR_SPEC>;
#[doc = "Overlay color for evaluation windows in panic mode."]
pub mod paniccolor;
#[doc = "EVALCONTROL0 (rw) register accessor: an alias for `Reg<EVALCONTROL0_SPEC>`"]
pub type EVALCONTROL0 = crate::Reg<evalcontrol0::EVALCONTROL0_SPEC>;
#[doc = "Control settings for evaluation window 0."]
pub mod evalcontrol0;
#[doc = "EVALUPPERLEFT0 (rw) register accessor: an alias for `Reg<EVALUPPERLEFT0_SPEC>`"]
pub type EVALUPPERLEFT0 = crate::Reg<evalupperleft0::EVALUPPERLEFT0_SPEC>;
#[doc = "Upper left corner of evaluation window 0."]
pub mod evalupperleft0;
#[doc = "EVALLOWERRIGHT0 (rw) register accessor: an alias for `Reg<EVALLOWERRIGHT0_SPEC>`"]
pub type EVALLOWERRIGHT0 = crate::Reg<evallowerright0::EVALLOWERRIGHT0_SPEC>;
#[doc = "Lower right corner of evaluation window 0."]
pub mod evallowerright0;
#[doc = "SIGCRCREDREF0 (rw) register accessor: an alias for `Reg<SIGCRCREDREF0_SPEC>`"]
pub type SIGCRCREDREF0 = crate::Reg<sigcrcredref0::SIGCRCREDREF0_SPEC>;
#[doc = "Reference signature of red channel for evaluation window 0."]
pub mod sigcrcredref0;
#[doc = "SIGCRCGREENREF0 (rw) register accessor: an alias for `Reg<SIGCRCGREENREF0_SPEC>`"]
pub type SIGCRCGREENREF0 = crate::Reg<sigcrcgreenref0::SIGCRCGREENREF0_SPEC>;
#[doc = "Reference signature of green channel for evaluation window 0."]
pub mod sigcrcgreenref0;
#[doc = "SIGCRCBLUEREF0 (rw) register accessor: an alias for `Reg<SIGCRCBLUEREF0_SPEC>`"]
pub type SIGCRCBLUEREF0 = crate::Reg<sigcrcblueref0::SIGCRCBLUEREF0_SPEC>;
#[doc = "Reference signature of blue channel for evaluation window 0."]
pub mod sigcrcblueref0;
#[doc = "SIGCRCRED0 (r) register accessor: an alias for `Reg<SIGCRCRED0_SPEC>`"]
pub type SIGCRCRED0 = crate::Reg<sigcrcred0::SIGCRCRED0_SPEC>;
#[doc = "Measured signature of red channel for evaluation window 0."]
pub mod sigcrcred0;
#[doc = "SIGCRCGREEN0 (r) register accessor: an alias for `Reg<SIGCRCGREEN0_SPEC>`"]
pub type SIGCRCGREEN0 = crate::Reg<sigcrcgreen0::SIGCRCGREEN0_SPEC>;
#[doc = "Measured signature of green channel for evaluation window 0."]
pub mod sigcrcgreen0;
#[doc = "SIGCRCBLUE0 (r) register accessor: an alias for `Reg<SIGCRCBLUE0_SPEC>`"]
pub type SIGCRCBLUE0 = crate::Reg<sigcrcblue0::SIGCRCBLUE0_SPEC>;
#[doc = "Measured signature of blue channel for evaluation window 0."]
pub mod sigcrcblue0;
#[doc = "EVALCONTROL1 (rw) register accessor: an alias for `Reg<EVALCONTROL1_SPEC>`"]
pub type EVALCONTROL1 = crate::Reg<evalcontrol1::EVALCONTROL1_SPEC>;
#[doc = "Control settings for evaluation window 1."]
pub mod evalcontrol1;
#[doc = "EVALUPPERLEFT1 (rw) register accessor: an alias for `Reg<EVALUPPERLEFT1_SPEC>`"]
pub type EVALUPPERLEFT1 = crate::Reg<evalupperleft1::EVALUPPERLEFT1_SPEC>;
#[doc = "Upper left corner of evaluation window 1."]
pub mod evalupperleft1;
#[doc = "EVALLOWERRIGHT1 (rw) register accessor: an alias for `Reg<EVALLOWERRIGHT1_SPEC>`"]
pub type EVALLOWERRIGHT1 = crate::Reg<evallowerright1::EVALLOWERRIGHT1_SPEC>;
#[doc = "Lower right corner of evaluation window 1."]
pub mod evallowerright1;
#[doc = "SIGCRCREDREF1 (rw) register accessor: an alias for `Reg<SIGCRCREDREF1_SPEC>`"]
pub type SIGCRCREDREF1 = crate::Reg<sigcrcredref1::SIGCRCREDREF1_SPEC>;
#[doc = "Reference signature of red channel for evaluation window 1."]
pub mod sigcrcredref1;
#[doc = "SIGCRCGREENREF1 (rw) register accessor: an alias for `Reg<SIGCRCGREENREF1_SPEC>`"]
pub type SIGCRCGREENREF1 = crate::Reg<sigcrcgreenref1::SIGCRCGREENREF1_SPEC>;
#[doc = "Reference signature of green channel for evaluation window 1."]
pub mod sigcrcgreenref1;
#[doc = "SIGCRCBLUEREF1 (rw) register accessor: an alias for `Reg<SIGCRCBLUEREF1_SPEC>`"]
pub type SIGCRCBLUEREF1 = crate::Reg<sigcrcblueref1::SIGCRCBLUEREF1_SPEC>;
#[doc = "Reference signature of blue channel for evaluation window 1."]
pub mod sigcrcblueref1;
#[doc = "SIGCRCRED1 (r) register accessor: an alias for `Reg<SIGCRCRED1_SPEC>`"]
pub type SIGCRCRED1 = crate::Reg<sigcrcred1::SIGCRCRED1_SPEC>;
#[doc = "Measured signature of red channel for evaluation window 1."]
pub mod sigcrcred1;
#[doc = "SIGCRCGREEN1 (r) register accessor: an alias for `Reg<SIGCRCGREEN1_SPEC>`"]
pub type SIGCRCGREEN1 = crate::Reg<sigcrcgreen1::SIGCRCGREEN1_SPEC>;
#[doc = "Measured signature of green channel for evaluation window 1."]
pub mod sigcrcgreen1;
#[doc = "SIGCRCBLUE1 (r) register accessor: an alias for `Reg<SIGCRCBLUE1_SPEC>`"]
pub type SIGCRCBLUE1 = crate::Reg<sigcrcblue1::SIGCRCBLUE1_SPEC>;
#[doc = "Measured signature of blue channel for evaluation window 1."]
pub mod sigcrcblue1;
#[doc = "EVALCONTROL2 (rw) register accessor: an alias for `Reg<EVALCONTROL2_SPEC>`"]
pub type EVALCONTROL2 = crate::Reg<evalcontrol2::EVALCONTROL2_SPEC>;
#[doc = "Control settings for evaluation window 2."]
pub mod evalcontrol2;
#[doc = "EVALUPPERLEFT2 (rw) register accessor: an alias for `Reg<EVALUPPERLEFT2_SPEC>`"]
pub type EVALUPPERLEFT2 = crate::Reg<evalupperleft2::EVALUPPERLEFT2_SPEC>;
#[doc = "Upper left corner of evaluation window 2."]
pub mod evalupperleft2;
#[doc = "EVALLOWERRIGHT2 (rw) register accessor: an alias for `Reg<EVALLOWERRIGHT2_SPEC>`"]
pub type EVALLOWERRIGHT2 = crate::Reg<evallowerright2::EVALLOWERRIGHT2_SPEC>;
#[doc = "Lower right corner of evaluation window 2."]
pub mod evallowerright2;
#[doc = "SIGCRCREDREF2 (rw) register accessor: an alias for `Reg<SIGCRCREDREF2_SPEC>`"]
pub type SIGCRCREDREF2 = crate::Reg<sigcrcredref2::SIGCRCREDREF2_SPEC>;
#[doc = "Reference signature of red channel for evaluation window 2."]
pub mod sigcrcredref2;
#[doc = "SIGCRCGREENREF2 (rw) register accessor: an alias for `Reg<SIGCRCGREENREF2_SPEC>`"]
pub type SIGCRCGREENREF2 = crate::Reg<sigcrcgreenref2::SIGCRCGREENREF2_SPEC>;
#[doc = "Reference signature of green channel for evaluation window 2."]
pub mod sigcrcgreenref2;
#[doc = "SIGCRCBLUEREF2 (rw) register accessor: an alias for `Reg<SIGCRCBLUEREF2_SPEC>`"]
pub type SIGCRCBLUEREF2 = crate::Reg<sigcrcblueref2::SIGCRCBLUEREF2_SPEC>;
#[doc = "Reference signature of blue channel for evaluation window 2."]
pub mod sigcrcblueref2;
#[doc = "SIGCRCRED2 (r) register accessor: an alias for `Reg<SIGCRCRED2_SPEC>`"]
pub type SIGCRCRED2 = crate::Reg<sigcrcred2::SIGCRCRED2_SPEC>;
#[doc = "Measured signature of red channel for evaluation window 2."]
pub mod sigcrcred2;
#[doc = "SIGCRCGREEN2 (r) register accessor: an alias for `Reg<SIGCRCGREEN2_SPEC>`"]
pub type SIGCRCGREEN2 = crate::Reg<sigcrcgreen2::SIGCRCGREEN2_SPEC>;
#[doc = "Measured signature of green channel for evaluation window 2."]
pub mod sigcrcgreen2;
#[doc = "SIGCRCBLUE2 (r) register accessor: an alias for `Reg<SIGCRCBLUE2_SPEC>`"]
pub type SIGCRCBLUE2 = crate::Reg<sigcrcblue2::SIGCRCBLUE2_SPEC>;
#[doc = "Measured signature of blue channel for evaluation window 2."]
pub mod sigcrcblue2;
#[doc = "EVALCONTROL3 (rw) register accessor: an alias for `Reg<EVALCONTROL3_SPEC>`"]
pub type EVALCONTROL3 = crate::Reg<evalcontrol3::EVALCONTROL3_SPEC>;
#[doc = "Control settings for evaluation window 3."]
pub mod evalcontrol3;
#[doc = "EVALUPPERLEFT3 (rw) register accessor: an alias for `Reg<EVALUPPERLEFT3_SPEC>`"]
pub type EVALUPPERLEFT3 = crate::Reg<evalupperleft3::EVALUPPERLEFT3_SPEC>;
#[doc = "Upper left corner of evaluation window 3."]
pub mod evalupperleft3;
#[doc = "EVALLOWERRIGHT3 (rw) register accessor: an alias for `Reg<EVALLOWERRIGHT3_SPEC>`"]
pub type EVALLOWERRIGHT3 = crate::Reg<evallowerright3::EVALLOWERRIGHT3_SPEC>;
#[doc = "Lower right corner of evaluation window 3."]
pub mod evallowerright3;
#[doc = "SIGCRCREDREF3 (rw) register accessor: an alias for `Reg<SIGCRCREDREF3_SPEC>`"]
pub type SIGCRCREDREF3 = crate::Reg<sigcrcredref3::SIGCRCREDREF3_SPEC>;
#[doc = "Reference signature of red channel for evaluation window 3."]
pub mod sigcrcredref3;
#[doc = "SIGCRCGREENREF3 (rw) register accessor: an alias for `Reg<SIGCRCGREENREF3_SPEC>`"]
pub type SIGCRCGREENREF3 = crate::Reg<sigcrcgreenref3::SIGCRCGREENREF3_SPEC>;
#[doc = "Reference signature of green channel for evaluation window 3."]
pub mod sigcrcgreenref3;
#[doc = "SIGCRCBLUEREF3 (rw) register accessor: an alias for `Reg<SIGCRCBLUEREF3_SPEC>`"]
pub type SIGCRCBLUEREF3 = crate::Reg<sigcrcblueref3::SIGCRCBLUEREF3_SPEC>;
#[doc = "Reference signature of blue channel for evaluation window 3."]
pub mod sigcrcblueref3;
#[doc = "SIGCRCRED3 (r) register accessor: an alias for `Reg<SIGCRCRED3_SPEC>`"]
pub type SIGCRCRED3 = crate::Reg<sigcrcred3::SIGCRCRED3_SPEC>;
#[doc = "Measured signature of red channel for evaluation window 3."]
pub mod sigcrcred3;
#[doc = "SIGCRCGREEN3 (r) register accessor: an alias for `Reg<SIGCRCGREEN3_SPEC>`"]
pub type SIGCRCGREEN3 = crate::Reg<sigcrcgreen3::SIGCRCGREEN3_SPEC>;
#[doc = "Measured signature of green channel for evaluation window 3."]
pub mod sigcrcgreen3;
#[doc = "SIGCRCBLUE3 (r) register accessor: an alias for `Reg<SIGCRCBLUE3_SPEC>`"]
pub type SIGCRCBLUE3 = crate::Reg<sigcrcblue3::SIGCRCBLUE3_SPEC>;
#[doc = "Measured signature of blue channel for evaluation window 3."]
pub mod sigcrcblue3;
#[doc = "EVALCONTROL4 (rw) register accessor: an alias for `Reg<EVALCONTROL4_SPEC>`"]
pub type EVALCONTROL4 = crate::Reg<evalcontrol4::EVALCONTROL4_SPEC>;
#[doc = "Control settings for evaluation window 4."]
pub mod evalcontrol4;
#[doc = "EVALUPPERLEFT4 (rw) register accessor: an alias for `Reg<EVALUPPERLEFT4_SPEC>`"]
pub type EVALUPPERLEFT4 = crate::Reg<evalupperleft4::EVALUPPERLEFT4_SPEC>;
#[doc = "Upper left corner of evaluation window 4."]
pub mod evalupperleft4;
#[doc = "EVALLOWERRIGHT4 (rw) register accessor: an alias for `Reg<EVALLOWERRIGHT4_SPEC>`"]
pub type EVALLOWERRIGHT4 = crate::Reg<evallowerright4::EVALLOWERRIGHT4_SPEC>;
#[doc = "Lower right corner of evaluation window 4."]
pub mod evallowerright4;
#[doc = "SIGCRCREDREF4 (rw) register accessor: an alias for `Reg<SIGCRCREDREF4_SPEC>`"]
pub type SIGCRCREDREF4 = crate::Reg<sigcrcredref4::SIGCRCREDREF4_SPEC>;
#[doc = "Reference signature of red channel for evaluation window 4."]
pub mod sigcrcredref4;
#[doc = "SIGCRCGREENREF4 (rw) register accessor: an alias for `Reg<SIGCRCGREENREF4_SPEC>`"]
pub type SIGCRCGREENREF4 = crate::Reg<sigcrcgreenref4::SIGCRCGREENREF4_SPEC>;
#[doc = "Reference signature of green channel for evaluation window 4."]
pub mod sigcrcgreenref4;
#[doc = "SIGCRCBLUEREF4 (rw) register accessor: an alias for `Reg<SIGCRCBLUEREF4_SPEC>`"]
pub type SIGCRCBLUEREF4 = crate::Reg<sigcrcblueref4::SIGCRCBLUEREF4_SPEC>;
#[doc = "Reference signature of blue channel for evaluation window 4."]
pub mod sigcrcblueref4;
#[doc = "SIGCRCRED4 (r) register accessor: an alias for `Reg<SIGCRCRED4_SPEC>`"]
pub type SIGCRCRED4 = crate::Reg<sigcrcred4::SIGCRCRED4_SPEC>;
#[doc = "Measured signature of red channel for evaluation window 4."]
pub mod sigcrcred4;
#[doc = "SIGCRCGREEN4 (r) register accessor: an alias for `Reg<SIGCRCGREEN4_SPEC>`"]
pub type SIGCRCGREEN4 = crate::Reg<sigcrcgreen4::SIGCRCGREEN4_SPEC>;
#[doc = "Measured signature of green channel for evaluation window 4."]
pub mod sigcrcgreen4;
#[doc = "SIGCRCBLUE4 (r) register accessor: an alias for `Reg<SIGCRCBLUE4_SPEC>`"]
pub type SIGCRCBLUE4 = crate::Reg<sigcrcblue4::SIGCRCBLUE4_SPEC>;
#[doc = "Measured signature of blue channel for evaluation window 4."]
pub mod sigcrcblue4;
#[doc = "EVALCONTROL5 (rw) register accessor: an alias for `Reg<EVALCONTROL5_SPEC>`"]
pub type EVALCONTROL5 = crate::Reg<evalcontrol5::EVALCONTROL5_SPEC>;
#[doc = "Control settings for evaluation window 5."]
pub mod evalcontrol5;
#[doc = "EVALUPPERLEFT5 (rw) register accessor: an alias for `Reg<EVALUPPERLEFT5_SPEC>`"]
pub type EVALUPPERLEFT5 = crate::Reg<evalupperleft5::EVALUPPERLEFT5_SPEC>;
#[doc = "Upper left corner of evaluation window 5."]
pub mod evalupperleft5;
#[doc = "EVALLOWERRIGHT5 (rw) register accessor: an alias for `Reg<EVALLOWERRIGHT5_SPEC>`"]
pub type EVALLOWERRIGHT5 = crate::Reg<evallowerright5::EVALLOWERRIGHT5_SPEC>;
#[doc = "Lower right corner of evaluation window 5."]
pub mod evallowerright5;
#[doc = "SIGCRCREDREF5 (rw) register accessor: an alias for `Reg<SIGCRCREDREF5_SPEC>`"]
pub type SIGCRCREDREF5 = crate::Reg<sigcrcredref5::SIGCRCREDREF5_SPEC>;
#[doc = "Reference signature of red channel for evaluation window 5."]
pub mod sigcrcredref5;
#[doc = "SIGCRCGREENREF5 (rw) register accessor: an alias for `Reg<SIGCRCGREENREF5_SPEC>`"]
pub type SIGCRCGREENREF5 = crate::Reg<sigcrcgreenref5::SIGCRCGREENREF5_SPEC>;
#[doc = "Reference signature of green channel for evaluation window 5."]
pub mod sigcrcgreenref5;
#[doc = "SIGCRCBLUEREF5 (rw) register accessor: an alias for `Reg<SIGCRCBLUEREF5_SPEC>`"]
pub type SIGCRCBLUEREF5 = crate::Reg<sigcrcblueref5::SIGCRCBLUEREF5_SPEC>;
#[doc = "Reference signature of blue channel for evaluation window 5."]
pub mod sigcrcblueref5;
#[doc = "SIGCRCRED5 (r) register accessor: an alias for `Reg<SIGCRCRED5_SPEC>`"]
pub type SIGCRCRED5 = crate::Reg<sigcrcred5::SIGCRCRED5_SPEC>;
#[doc = "Measured signature of red channel for evaluation window 5."]
pub mod sigcrcred5;
#[doc = "SIGCRCGREEN5 (r) register accessor: an alias for `Reg<SIGCRCGREEN5_SPEC>`"]
pub type SIGCRCGREEN5 = crate::Reg<sigcrcgreen5::SIGCRCGREEN5_SPEC>;
#[doc = "Measured signature of green channel for evaluation window 5."]
pub mod sigcrcgreen5;
#[doc = "SIGCRCBLUE5 (r) register accessor: an alias for `Reg<SIGCRCBLUE5_SPEC>`"]
pub type SIGCRCBLUE5 = crate::Reg<sigcrcblue5::SIGCRCBLUE5_SPEC>;
#[doc = "Measured signature of blue channel for evaluation window 5."]
pub mod sigcrcblue5;
#[doc = "EVALCONTROL6 (rw) register accessor: an alias for `Reg<EVALCONTROL6_SPEC>`"]
pub type EVALCONTROL6 = crate::Reg<evalcontrol6::EVALCONTROL6_SPEC>;
#[doc = "Control settings for evaluation window 6."]
pub mod evalcontrol6;
#[doc = "EVALUPPERLEFT6 (rw) register accessor: an alias for `Reg<EVALUPPERLEFT6_SPEC>`"]
pub type EVALUPPERLEFT6 = crate::Reg<evalupperleft6::EVALUPPERLEFT6_SPEC>;
#[doc = "Upper left corner of evaluation window 6."]
pub mod evalupperleft6;
#[doc = "EVALLOWERRIGHT6 (rw) register accessor: an alias for `Reg<EVALLOWERRIGHT6_SPEC>`"]
pub type EVALLOWERRIGHT6 = crate::Reg<evallowerright6::EVALLOWERRIGHT6_SPEC>;
#[doc = "Lower right corner of evaluation window 6."]
pub mod evallowerright6;
#[doc = "SIGCRCREDREF6 (rw) register accessor: an alias for `Reg<SIGCRCREDREF6_SPEC>`"]
pub type SIGCRCREDREF6 = crate::Reg<sigcrcredref6::SIGCRCREDREF6_SPEC>;
#[doc = "Reference signature of red channel for evaluation window 6."]
pub mod sigcrcredref6;
#[doc = "SIGCRCGREENREF6 (rw) register accessor: an alias for `Reg<SIGCRCGREENREF6_SPEC>`"]
pub type SIGCRCGREENREF6 = crate::Reg<sigcrcgreenref6::SIGCRCGREENREF6_SPEC>;
#[doc = "Reference signature of green channel for evaluation window 6."]
pub mod sigcrcgreenref6;
#[doc = "SIGCRCBLUEREF6 (rw) register accessor: an alias for `Reg<SIGCRCBLUEREF6_SPEC>`"]
pub type SIGCRCBLUEREF6 = crate::Reg<sigcrcblueref6::SIGCRCBLUEREF6_SPEC>;
#[doc = "Reference signature of blue channel for evaluation window 6."]
pub mod sigcrcblueref6;
#[doc = "SIGCRCRED6 (r) register accessor: an alias for `Reg<SIGCRCRED6_SPEC>`"]
pub type SIGCRCRED6 = crate::Reg<sigcrcred6::SIGCRCRED6_SPEC>;
#[doc = "Measured signature of red channel for evaluation window 6."]
pub mod sigcrcred6;
#[doc = "SIGCRCGREEN6 (r) register accessor: an alias for `Reg<SIGCRCGREEN6_SPEC>`"]
pub type SIGCRCGREEN6 = crate::Reg<sigcrcgreen6::SIGCRCGREEN6_SPEC>;
#[doc = "Measured signature of green channel for evaluation window 6."]
pub mod sigcrcgreen6;
#[doc = "SIGCRCBLUE6 (r) register accessor: an alias for `Reg<SIGCRCBLUE6_SPEC>`"]
pub type SIGCRCBLUE6 = crate::Reg<sigcrcblue6::SIGCRCBLUE6_SPEC>;
#[doc = "Measured signature of blue channel for evaluation window 6."]
pub mod sigcrcblue6;
#[doc = "EVALCONTROL7 (rw) register accessor: an alias for `Reg<EVALCONTROL7_SPEC>`"]
pub type EVALCONTROL7 = crate::Reg<evalcontrol7::EVALCONTROL7_SPEC>;
#[doc = "Control settings for evaluation window 7."]
pub mod evalcontrol7;
#[doc = "EVALUPPERLEFT7 (rw) register accessor: an alias for `Reg<EVALUPPERLEFT7_SPEC>`"]
pub type EVALUPPERLEFT7 = crate::Reg<evalupperleft7::EVALUPPERLEFT7_SPEC>;
#[doc = "Upper left corner of evaluation window 7."]
pub mod evalupperleft7;
#[doc = "EVALLOWERRIGHT7 (rw) register accessor: an alias for `Reg<EVALLOWERRIGHT7_SPEC>`"]
pub type EVALLOWERRIGHT7 = crate::Reg<evallowerright7::EVALLOWERRIGHT7_SPEC>;
#[doc = "Lower right corner of evaluation window 7."]
pub mod evallowerright7;
#[doc = "SIGCRCREDREF7 (rw) register accessor: an alias for `Reg<SIGCRCREDREF7_SPEC>`"]
pub type SIGCRCREDREF7 = crate::Reg<sigcrcredref7::SIGCRCREDREF7_SPEC>;
#[doc = "Reference signature of red channel for evaluation window 7."]
pub mod sigcrcredref7;
#[doc = "SIGCRCGREENREF7 (rw) register accessor: an alias for `Reg<SIGCRCGREENREF7_SPEC>`"]
pub type SIGCRCGREENREF7 = crate::Reg<sigcrcgreenref7::SIGCRCGREENREF7_SPEC>;
#[doc = "Reference signature of green channel for evaluation window 7."]
pub mod sigcrcgreenref7;
#[doc = "SIGCRCBLUEREF7 (rw) register accessor: an alias for `Reg<SIGCRCBLUEREF7_SPEC>`"]
pub type SIGCRCBLUEREF7 = crate::Reg<sigcrcblueref7::SIGCRCBLUEREF7_SPEC>;
#[doc = "Reference signature of blue channel for evaluation window 7."]
pub mod sigcrcblueref7;
#[doc = "SIGCRCRED7 (r) register accessor: an alias for `Reg<SIGCRCRED7_SPEC>`"]
pub type SIGCRCRED7 = crate::Reg<sigcrcred7::SIGCRCRED7_SPEC>;
#[doc = "Measured signature of red channel for evaluation window 7."]
pub mod sigcrcred7;
#[doc = "SIGCRCGREEN7 (r) register accessor: an alias for `Reg<SIGCRCGREEN7_SPEC>`"]
pub type SIGCRCGREEN7 = crate::Reg<sigcrcgreen7::SIGCRCGREEN7_SPEC>;
#[doc = "Measured signature of green channel for evaluation window 7."]
pub mod sigcrcgreen7;
#[doc = "SIGCRCBLUE7 (r) register accessor: an alias for `Reg<SIGCRCBLUE7_SPEC>`"]
pub type SIGCRCBLUE7 = crate::Reg<sigcrcblue7::SIGCRCBLUE7_SPEC>;
#[doc = "Measured signature of blue channel for evaluation window 7."]
pub mod sigcrcblue7;
#[doc = "SHADOWLOAD (rw) register accessor: an alias for `Reg<SHADOWLOAD_SPEC>`"]
pub type SHADOWLOAD = crate::Reg<shadowload::SHADOWLOAD_SPEC>;
#[doc = "Shadow load control register."]
pub mod shadowload;
#[doc = "CONTINUOUSMODE (rw) register accessor: an alias for `Reg<CONTINUOUSMODE_SPEC>`"]
pub type CONTINUOUSMODE = crate::Reg<continuousmode::CONTINUOUSMODE_SPEC>;
#[doc = "Signature operation mode control."]
pub mod continuousmode;
#[doc = "SOFTWAREKICK (w) register accessor: an alias for `Reg<SOFTWAREKICK_SPEC>`"]
pub type SOFTWAREKICK = crate::Reg<softwarekick::SOFTWAREKICK_SPEC>;
#[doc = "Signature measurement trigger."]
pub mod softwarekick;
#[doc = "STATUS (r) register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Module status."]
pub mod status;
