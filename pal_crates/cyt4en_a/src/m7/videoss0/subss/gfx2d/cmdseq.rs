#[doc = r"Register block"]
#[repr(C)]
pub struct CMDSEQ {
    #[doc = "0x00 - Task buffer 0 address register"]
    pub taskbufferaddress0: TASKBUFFERADDRESS0,
    #[doc = "0x04 - Task buffer 0 size register"]
    pub taskbuffersize0: TASKBUFFERSIZE0,
    #[doc = "0x08 - Task buffer 0 stop offset register"]
    pub taskbufferstopoffset0: TASKBUFFERSTOPOFFSET0,
    #[doc = "0x0c - Task buffer 1 address register"]
    pub taskbufferaddress1: TASKBUFFERADDRESS1,
    #[doc = "0x10 - Task buffer 1 size register"]
    pub taskbuffersize1: TASKBUFFERSIZE1,
    #[doc = "0x14 - Task buffer 1 stop offset register"]
    pub taskbufferstopoffset1: TASKBUFFERSTOPOFFSET1,
    #[doc = "0x18 - Task buffer 2 address register"]
    pub taskbufferaddress2: TASKBUFFERADDRESS2,
    #[doc = "0x1c - Task buffer 2 size register"]
    pub taskbuffersize2: TASKBUFFERSIZE2,
    #[doc = "0x20 - Task buffer 2 stop offset register"]
    pub taskbufferstopoffset2: TASKBUFFERSTOPOFFSET2,
    #[doc = "0x24 - Task buffer 3 address register"]
    pub taskbufferaddress3: TASKBUFFERADDRESS3,
    #[doc = "0x28 - Task buffer 3 size register"]
    pub taskbuffersize3: TASKBUFFERSIZE3,
    #[doc = "0x2c - Task buffer 3 stop offset register"]
    pub taskbufferstopoffset3: TASKBUFFERSTOPOFFSET3,
    #[doc = "0x30 - Task buffer 4 address register"]
    pub taskbufferaddress4: TASKBUFFERADDRESS4,
    #[doc = "0x34 - Task buffer 4 size register"]
    pub taskbuffersize4: TASKBUFFERSIZE4,
    #[doc = "0x38 - Task buffer 4 stop offset register"]
    pub taskbufferstopoffset4: TASKBUFFERSTOPOFFSET4,
    #[doc = "0x3c - Task buffer 5 address register"]
    pub taskbufferaddress5: TASKBUFFERADDRESS5,
    #[doc = "0x40 - Task buffer 5 size register"]
    pub taskbuffersize5: TASKBUFFERSIZE5,
    #[doc = "0x44 - Task buffer 5 stop offset register"]
    pub taskbufferstopoffset5: TASKBUFFERSTOPOFFSET5,
    #[doc = "0x48 - Task buffer 6 address register"]
    pub taskbufferaddress6: TASKBUFFERADDRESS6,
    #[doc = "0x4c - Task buffer 6 size register"]
    pub taskbuffersize6: TASKBUFFERSIZE6,
    #[doc = "0x50 - Task buffer 6 stop offset register"]
    pub taskbufferstopoffset6: TASKBUFFERSTOPOFFSET6,
    #[doc = "0x54 - Task buffer 7 address register"]
    pub taskbufferaddress7: TASKBUFFERADDRESS7,
    #[doc = "0x58 - Task buffer 7 size register"]
    pub taskbuffersize7: TASKBUFFERSIZE7,
    #[doc = "0x5c - Task buffer 7 stop offset register"]
    pub taskbufferstopoffset7: TASKBUFFERSTOPOFFSET7,
    #[doc = "0x60 - Task buffer 0 execute offset register"]
    pub taskbufferexecoffset0: TASKBUFFEREXECOFFSET0,
    #[doc = "0x64 - Task buffer 1 execute offset register"]
    pub taskbufferexecoffset1: TASKBUFFEREXECOFFSET1,
    #[doc = "0x68 - Task buffer 2 execute offset register"]
    pub taskbufferexecoffset2: TASKBUFFEREXECOFFSET2,
    #[doc = "0x6c - Task buffer 3 execute offset register"]
    pub taskbufferexecoffset3: TASKBUFFEREXECOFFSET3,
    #[doc = "0x70 - Task buffer 4 execute offset register"]
    pub taskbufferexecoffset4: TASKBUFFEREXECOFFSET4,
    #[doc = "0x74 - Task buffer 5 execute offset register"]
    pub taskbufferexecoffset5: TASKBUFFEREXECOFFSET5,
    #[doc = "0x78 - Task buffer 6 execute offset register"]
    pub taskbufferexecoffset6: TASKBUFFEREXECOFFSET6,
    #[doc = "0x7c - Task buffer 7 execute offset register"]
    pub taskbufferexecoffset7: TASKBUFFEREXECOFFSET7,
    #[doc = "0x80 - Status register"]
    pub status: STATUS,
    #[doc = "0x84 - Error code register, each field of this register is one bit wide and corresponds to an error event. If the value of this register is zero, than there are no errors."]
    pub errorcode: ERRORCODE,
    #[doc = "0x88 - Provides HW specific information."]
    pub designconfiguration: DESIGNCONFIGURATION,
    #[doc = "0x8c - Last fetched address of scheduler."]
    pub scheduleraddress: SCHEDULERADDRESS,
    #[doc = "0x90 - Last fetched address of programmer."]
    pub programmeraddress: PROGRAMMERADDRESS,
    #[doc = "0x94 - General purpose Register. (Internal register with id = 0)"]
    pub gpr0: GPR0,
    #[doc = "0x98 - General purpose Register. (Internal register with id = 1)"]
    pub gpr1: GPR1,
    #[doc = "0x9c - General purpose Register. (Internal register with id = 2)"]
    pub gpr2: GPR2,
    #[doc = "0xa0 - General purpose Register. (Internal register with id = 3)"]
    pub gpr3: GPR3,
    #[doc = "0xa4 - General purpose Register. (Internal register with id = 4)"]
    pub gpr4: GPR4,
    #[doc = "0xa8 - General purpose Register. (Internal register with id = 5)"]
    pub gpr5: GPR5,
    #[doc = "0xac - General purpose Register. (Internal register with id = 6)"]
    pub gpr6: GPR6,
    #[doc = "0xb0 - General purpose Register. (Internal register with id = 7)"]
    pub gpr7: GPR7,
    #[doc = "0xb4 - General purpose Register. (Internal register with id = 8)"]
    pub gpr8: GPR8,
    #[doc = "0xb8 - General purpose Register. (Internal register with id = 9)"]
    pub gpr9: GPR9,
    #[doc = "0xbc - General purpose Register. (Internal register with id = 10)"]
    pub gpr10: GPR10,
    #[doc = "0xc0 - General purpose Register. (Internal register with id = 11)"]
    pub gpr11: GPR11,
    #[doc = "0xc4 - General purpose Register. (Internal register with id = 12)"]
    pub gpr12: GPR12,
    #[doc = "0xc8 - General purpose Register. (Internal register with id = 13)"]
    pub gpr13: GPR13,
    #[doc = "0xcc - General purpose Register. (Internal register with id = 14)"]
    pub gpr14: GPR14,
    #[doc = "0xd0 - General purpose Register. (Internal register with id = 15)"]
    pub gpr15: GPR15,
    #[doc = "0xd4 - Special Purpose Register. (Internal register with id = 16)"]
    pub spr0: SPR0,
    #[doc = "0xd8 - Special Purpose Register (internal register with id = 17)"]
    pub spr1: SPR1,
    #[doc = "0xdc - Special Purpose Register (internal register with id = 18)"]
    pub spr2: SPR2,
    #[doc = "0xe0 - Special Purpose Register (internal register with id = 19)"]
    pub spr3: SPR3,
    #[doc = "0xe4 - Special Purpose Register (internal register with id = 20)"]
    pub spr4: SPR4,
    #[doc = "0xe8 - Special Purpose Register (internal register with id = 21)"]
    pub spr5: SPR5,
    #[doc = "0xec - Special Purpose Register (internal register with id = 22)"]
    pub spr6: SPR6,
    #[doc = "0xf0 - Special Purpose Register (internal register with id = 23)"]
    pub spr7: SPR7,
}
#[doc = "TASKBUFFERADDRESS0 (rw) register accessor: an alias for `Reg<TASKBUFFERADDRESS0_SPEC>`"]
pub type TASKBUFFERADDRESS0 = crate::Reg<taskbufferaddress0::TASKBUFFERADDRESS0_SPEC>;
#[doc = "Task buffer 0 address register"]
pub mod taskbufferaddress0;
#[doc = "TASKBUFFERSIZE0 (rw) register accessor: an alias for `Reg<TASKBUFFERSIZE0_SPEC>`"]
pub type TASKBUFFERSIZE0 = crate::Reg<taskbuffersize0::TASKBUFFERSIZE0_SPEC>;
#[doc = "Task buffer 0 size register"]
pub mod taskbuffersize0;
#[doc = "TASKBUFFERSTOPOFFSET0 (rw) register accessor: an alias for `Reg<TASKBUFFERSTOPOFFSET0_SPEC>`"]
pub type TASKBUFFERSTOPOFFSET0 = crate::Reg<taskbufferstopoffset0::TASKBUFFERSTOPOFFSET0_SPEC>;
#[doc = "Task buffer 0 stop offset register"]
pub mod taskbufferstopoffset0;
#[doc = "TASKBUFFERADDRESS1 (rw) register accessor: an alias for `Reg<TASKBUFFERADDRESS1_SPEC>`"]
pub type TASKBUFFERADDRESS1 = crate::Reg<taskbufferaddress1::TASKBUFFERADDRESS1_SPEC>;
#[doc = "Task buffer 1 address register"]
pub mod taskbufferaddress1;
#[doc = "TASKBUFFERSIZE1 (rw) register accessor: an alias for `Reg<TASKBUFFERSIZE1_SPEC>`"]
pub type TASKBUFFERSIZE1 = crate::Reg<taskbuffersize1::TASKBUFFERSIZE1_SPEC>;
#[doc = "Task buffer 1 size register"]
pub mod taskbuffersize1;
#[doc = "TASKBUFFERSTOPOFFSET1 (rw) register accessor: an alias for `Reg<TASKBUFFERSTOPOFFSET1_SPEC>`"]
pub type TASKBUFFERSTOPOFFSET1 = crate::Reg<taskbufferstopoffset1::TASKBUFFERSTOPOFFSET1_SPEC>;
#[doc = "Task buffer 1 stop offset register"]
pub mod taskbufferstopoffset1;
#[doc = "TASKBUFFERADDRESS2 (rw) register accessor: an alias for `Reg<TASKBUFFERADDRESS2_SPEC>`"]
pub type TASKBUFFERADDRESS2 = crate::Reg<taskbufferaddress2::TASKBUFFERADDRESS2_SPEC>;
#[doc = "Task buffer 2 address register"]
pub mod taskbufferaddress2;
#[doc = "TASKBUFFERSIZE2 (rw) register accessor: an alias for `Reg<TASKBUFFERSIZE2_SPEC>`"]
pub type TASKBUFFERSIZE2 = crate::Reg<taskbuffersize2::TASKBUFFERSIZE2_SPEC>;
#[doc = "Task buffer 2 size register"]
pub mod taskbuffersize2;
#[doc = "TASKBUFFERSTOPOFFSET2 (rw) register accessor: an alias for `Reg<TASKBUFFERSTOPOFFSET2_SPEC>`"]
pub type TASKBUFFERSTOPOFFSET2 = crate::Reg<taskbufferstopoffset2::TASKBUFFERSTOPOFFSET2_SPEC>;
#[doc = "Task buffer 2 stop offset register"]
pub mod taskbufferstopoffset2;
#[doc = "TASKBUFFERADDRESS3 (rw) register accessor: an alias for `Reg<TASKBUFFERADDRESS3_SPEC>`"]
pub type TASKBUFFERADDRESS3 = crate::Reg<taskbufferaddress3::TASKBUFFERADDRESS3_SPEC>;
#[doc = "Task buffer 3 address register"]
pub mod taskbufferaddress3;
#[doc = "TASKBUFFERSIZE3 (rw) register accessor: an alias for `Reg<TASKBUFFERSIZE3_SPEC>`"]
pub type TASKBUFFERSIZE3 = crate::Reg<taskbuffersize3::TASKBUFFERSIZE3_SPEC>;
#[doc = "Task buffer 3 size register"]
pub mod taskbuffersize3;
#[doc = "TASKBUFFERSTOPOFFSET3 (rw) register accessor: an alias for `Reg<TASKBUFFERSTOPOFFSET3_SPEC>`"]
pub type TASKBUFFERSTOPOFFSET3 = crate::Reg<taskbufferstopoffset3::TASKBUFFERSTOPOFFSET3_SPEC>;
#[doc = "Task buffer 3 stop offset register"]
pub mod taskbufferstopoffset3;
#[doc = "TASKBUFFERADDRESS4 (rw) register accessor: an alias for `Reg<TASKBUFFERADDRESS4_SPEC>`"]
pub type TASKBUFFERADDRESS4 = crate::Reg<taskbufferaddress4::TASKBUFFERADDRESS4_SPEC>;
#[doc = "Task buffer 4 address register"]
pub mod taskbufferaddress4;
#[doc = "TASKBUFFERSIZE4 (rw) register accessor: an alias for `Reg<TASKBUFFERSIZE4_SPEC>`"]
pub type TASKBUFFERSIZE4 = crate::Reg<taskbuffersize4::TASKBUFFERSIZE4_SPEC>;
#[doc = "Task buffer 4 size register"]
pub mod taskbuffersize4;
#[doc = "TASKBUFFERSTOPOFFSET4 (rw) register accessor: an alias for `Reg<TASKBUFFERSTOPOFFSET4_SPEC>`"]
pub type TASKBUFFERSTOPOFFSET4 = crate::Reg<taskbufferstopoffset4::TASKBUFFERSTOPOFFSET4_SPEC>;
#[doc = "Task buffer 4 stop offset register"]
pub mod taskbufferstopoffset4;
#[doc = "TASKBUFFERADDRESS5 (rw) register accessor: an alias for `Reg<TASKBUFFERADDRESS5_SPEC>`"]
pub type TASKBUFFERADDRESS5 = crate::Reg<taskbufferaddress5::TASKBUFFERADDRESS5_SPEC>;
#[doc = "Task buffer 5 address register"]
pub mod taskbufferaddress5;
#[doc = "TASKBUFFERSIZE5 (rw) register accessor: an alias for `Reg<TASKBUFFERSIZE5_SPEC>`"]
pub type TASKBUFFERSIZE5 = crate::Reg<taskbuffersize5::TASKBUFFERSIZE5_SPEC>;
#[doc = "Task buffer 5 size register"]
pub mod taskbuffersize5;
#[doc = "TASKBUFFERSTOPOFFSET5 (rw) register accessor: an alias for `Reg<TASKBUFFERSTOPOFFSET5_SPEC>`"]
pub type TASKBUFFERSTOPOFFSET5 = crate::Reg<taskbufferstopoffset5::TASKBUFFERSTOPOFFSET5_SPEC>;
#[doc = "Task buffer 5 stop offset register"]
pub mod taskbufferstopoffset5;
#[doc = "TASKBUFFERADDRESS6 (rw) register accessor: an alias for `Reg<TASKBUFFERADDRESS6_SPEC>`"]
pub type TASKBUFFERADDRESS6 = crate::Reg<taskbufferaddress6::TASKBUFFERADDRESS6_SPEC>;
#[doc = "Task buffer 6 address register"]
pub mod taskbufferaddress6;
#[doc = "TASKBUFFERSIZE6 (rw) register accessor: an alias for `Reg<TASKBUFFERSIZE6_SPEC>`"]
pub type TASKBUFFERSIZE6 = crate::Reg<taskbuffersize6::TASKBUFFERSIZE6_SPEC>;
#[doc = "Task buffer 6 size register"]
pub mod taskbuffersize6;
#[doc = "TASKBUFFERSTOPOFFSET6 (rw) register accessor: an alias for `Reg<TASKBUFFERSTOPOFFSET6_SPEC>`"]
pub type TASKBUFFERSTOPOFFSET6 = crate::Reg<taskbufferstopoffset6::TASKBUFFERSTOPOFFSET6_SPEC>;
#[doc = "Task buffer 6 stop offset register"]
pub mod taskbufferstopoffset6;
#[doc = "TASKBUFFERADDRESS7 (rw) register accessor: an alias for `Reg<TASKBUFFERADDRESS7_SPEC>`"]
pub type TASKBUFFERADDRESS7 = crate::Reg<taskbufferaddress7::TASKBUFFERADDRESS7_SPEC>;
#[doc = "Task buffer 7 address register"]
pub mod taskbufferaddress7;
#[doc = "TASKBUFFERSIZE7 (rw) register accessor: an alias for `Reg<TASKBUFFERSIZE7_SPEC>`"]
pub type TASKBUFFERSIZE7 = crate::Reg<taskbuffersize7::TASKBUFFERSIZE7_SPEC>;
#[doc = "Task buffer 7 size register"]
pub mod taskbuffersize7;
#[doc = "TASKBUFFERSTOPOFFSET7 (rw) register accessor: an alias for `Reg<TASKBUFFERSTOPOFFSET7_SPEC>`"]
pub type TASKBUFFERSTOPOFFSET7 = crate::Reg<taskbufferstopoffset7::TASKBUFFERSTOPOFFSET7_SPEC>;
#[doc = "Task buffer 7 stop offset register"]
pub mod taskbufferstopoffset7;
#[doc = "TASKBUFFEREXECOFFSET0 (r) register accessor: an alias for `Reg<TASKBUFFEREXECOFFSET0_SPEC>`"]
pub type TASKBUFFEREXECOFFSET0 = crate::Reg<taskbufferexecoffset0::TASKBUFFEREXECOFFSET0_SPEC>;
#[doc = "Task buffer 0 execute offset register"]
pub mod taskbufferexecoffset0;
#[doc = "TASKBUFFEREXECOFFSET1 (r) register accessor: an alias for `Reg<TASKBUFFEREXECOFFSET1_SPEC>`"]
pub type TASKBUFFEREXECOFFSET1 = crate::Reg<taskbufferexecoffset1::TASKBUFFEREXECOFFSET1_SPEC>;
#[doc = "Task buffer 1 execute offset register"]
pub mod taskbufferexecoffset1;
#[doc = "TASKBUFFEREXECOFFSET2 (r) register accessor: an alias for `Reg<TASKBUFFEREXECOFFSET2_SPEC>`"]
pub type TASKBUFFEREXECOFFSET2 = crate::Reg<taskbufferexecoffset2::TASKBUFFEREXECOFFSET2_SPEC>;
#[doc = "Task buffer 2 execute offset register"]
pub mod taskbufferexecoffset2;
#[doc = "TASKBUFFEREXECOFFSET3 (r) register accessor: an alias for `Reg<TASKBUFFEREXECOFFSET3_SPEC>`"]
pub type TASKBUFFEREXECOFFSET3 = crate::Reg<taskbufferexecoffset3::TASKBUFFEREXECOFFSET3_SPEC>;
#[doc = "Task buffer 3 execute offset register"]
pub mod taskbufferexecoffset3;
#[doc = "TASKBUFFEREXECOFFSET4 (r) register accessor: an alias for `Reg<TASKBUFFEREXECOFFSET4_SPEC>`"]
pub type TASKBUFFEREXECOFFSET4 = crate::Reg<taskbufferexecoffset4::TASKBUFFEREXECOFFSET4_SPEC>;
#[doc = "Task buffer 4 execute offset register"]
pub mod taskbufferexecoffset4;
#[doc = "TASKBUFFEREXECOFFSET5 (r) register accessor: an alias for `Reg<TASKBUFFEREXECOFFSET5_SPEC>`"]
pub type TASKBUFFEREXECOFFSET5 = crate::Reg<taskbufferexecoffset5::TASKBUFFEREXECOFFSET5_SPEC>;
#[doc = "Task buffer 5 execute offset register"]
pub mod taskbufferexecoffset5;
#[doc = "TASKBUFFEREXECOFFSET6 (r) register accessor: an alias for `Reg<TASKBUFFEREXECOFFSET6_SPEC>`"]
pub type TASKBUFFEREXECOFFSET6 = crate::Reg<taskbufferexecoffset6::TASKBUFFEREXECOFFSET6_SPEC>;
#[doc = "Task buffer 6 execute offset register"]
pub mod taskbufferexecoffset6;
#[doc = "TASKBUFFEREXECOFFSET7 (r) register accessor: an alias for `Reg<TASKBUFFEREXECOFFSET7_SPEC>`"]
pub type TASKBUFFEREXECOFFSET7 = crate::Reg<taskbufferexecoffset7::TASKBUFFEREXECOFFSET7_SPEC>;
#[doc = "Task buffer 7 execute offset register"]
pub mod taskbufferexecoffset7;
#[doc = "STATUS (r) register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Status register"]
pub mod status;
#[doc = "ERRORCODE (r) register accessor: an alias for `Reg<ERRORCODE_SPEC>`"]
pub type ERRORCODE = crate::Reg<errorcode::ERRORCODE_SPEC>;
#[doc = "Error code register, each field of this register is one bit wide and corresponds to an error event. If the value of this register is zero, than there are no errors."]
pub mod errorcode;
#[doc = "DESIGNCONFIGURATION (r) register accessor: an alias for `Reg<DESIGNCONFIGURATION_SPEC>`"]
pub type DESIGNCONFIGURATION = crate::Reg<designconfiguration::DESIGNCONFIGURATION_SPEC>;
#[doc = "Provides HW specific information."]
pub mod designconfiguration;
#[doc = "SCHEDULERADDRESS (r) register accessor: an alias for `Reg<SCHEDULERADDRESS_SPEC>`"]
pub type SCHEDULERADDRESS = crate::Reg<scheduleraddress::SCHEDULERADDRESS_SPEC>;
#[doc = "Last fetched address of scheduler."]
pub mod scheduleraddress;
#[doc = "PROGRAMMERADDRESS (r) register accessor: an alias for `Reg<PROGRAMMERADDRESS_SPEC>`"]
pub type PROGRAMMERADDRESS = crate::Reg<programmeraddress::PROGRAMMERADDRESS_SPEC>;
#[doc = "Last fetched address of programmer."]
pub mod programmeraddress;
#[doc = "GPR0 (r) register accessor: an alias for `Reg<GPR0_SPEC>`"]
pub type GPR0 = crate::Reg<gpr0::GPR0_SPEC>;
#[doc = "General purpose Register. (Internal register with id = 0)"]
pub mod gpr0;
#[doc = "GPR1 (r) register accessor: an alias for `Reg<GPR1_SPEC>`"]
pub type GPR1 = crate::Reg<gpr1::GPR1_SPEC>;
#[doc = "General purpose Register. (Internal register with id = 1)"]
pub mod gpr1;
#[doc = "GPR2 (r) register accessor: an alias for `Reg<GPR2_SPEC>`"]
pub type GPR2 = crate::Reg<gpr2::GPR2_SPEC>;
#[doc = "General purpose Register. (Internal register with id = 2)"]
pub mod gpr2;
#[doc = "GPR3 (r) register accessor: an alias for `Reg<GPR3_SPEC>`"]
pub type GPR3 = crate::Reg<gpr3::GPR3_SPEC>;
#[doc = "General purpose Register. (Internal register with id = 3)"]
pub mod gpr3;
#[doc = "GPR4 (r) register accessor: an alias for `Reg<GPR4_SPEC>`"]
pub type GPR4 = crate::Reg<gpr4::GPR4_SPEC>;
#[doc = "General purpose Register. (Internal register with id = 4)"]
pub mod gpr4;
#[doc = "GPR5 (r) register accessor: an alias for `Reg<GPR5_SPEC>`"]
pub type GPR5 = crate::Reg<gpr5::GPR5_SPEC>;
#[doc = "General purpose Register. (Internal register with id = 5)"]
pub mod gpr5;
#[doc = "GPR6 (r) register accessor: an alias for `Reg<GPR6_SPEC>`"]
pub type GPR6 = crate::Reg<gpr6::GPR6_SPEC>;
#[doc = "General purpose Register. (Internal register with id = 6)"]
pub mod gpr6;
#[doc = "GPR7 (r) register accessor: an alias for `Reg<GPR7_SPEC>`"]
pub type GPR7 = crate::Reg<gpr7::GPR7_SPEC>;
#[doc = "General purpose Register. (Internal register with id = 7)"]
pub mod gpr7;
#[doc = "GPR8 (r) register accessor: an alias for `Reg<GPR8_SPEC>`"]
pub type GPR8 = crate::Reg<gpr8::GPR8_SPEC>;
#[doc = "General purpose Register. (Internal register with id = 8)"]
pub mod gpr8;
#[doc = "GPR9 (r) register accessor: an alias for `Reg<GPR9_SPEC>`"]
pub type GPR9 = crate::Reg<gpr9::GPR9_SPEC>;
#[doc = "General purpose Register. (Internal register with id = 9)"]
pub mod gpr9;
#[doc = "GPR10 (r) register accessor: an alias for `Reg<GPR10_SPEC>`"]
pub type GPR10 = crate::Reg<gpr10::GPR10_SPEC>;
#[doc = "General purpose Register. (Internal register with id = 10)"]
pub mod gpr10;
#[doc = "GPR11 (r) register accessor: an alias for `Reg<GPR11_SPEC>`"]
pub type GPR11 = crate::Reg<gpr11::GPR11_SPEC>;
#[doc = "General purpose Register. (Internal register with id = 11)"]
pub mod gpr11;
#[doc = "GPR12 (r) register accessor: an alias for `Reg<GPR12_SPEC>`"]
pub type GPR12 = crate::Reg<gpr12::GPR12_SPEC>;
#[doc = "General purpose Register. (Internal register with id = 12)"]
pub mod gpr12;
#[doc = "GPR13 (r) register accessor: an alias for `Reg<GPR13_SPEC>`"]
pub type GPR13 = crate::Reg<gpr13::GPR13_SPEC>;
#[doc = "General purpose Register. (Internal register with id = 13)"]
pub mod gpr13;
#[doc = "GPR14 (r) register accessor: an alias for `Reg<GPR14_SPEC>`"]
pub type GPR14 = crate::Reg<gpr14::GPR14_SPEC>;
#[doc = "General purpose Register. (Internal register with id = 14)"]
pub mod gpr14;
#[doc = "GPR15 (r) register accessor: an alias for `Reg<GPR15_SPEC>`"]
pub type GPR15 = crate::Reg<gpr15::GPR15_SPEC>;
#[doc = "General purpose Register. (Internal register with id = 15)"]
pub mod gpr15;
#[doc = "SPR0 (r) register accessor: an alias for `Reg<SPR0_SPEC>`"]
pub type SPR0 = crate::Reg<spr0::SPR0_SPEC>;
#[doc = "Special Purpose Register. (Internal register with id = 16)"]
pub mod spr0;
#[doc = "SPR1 (r) register accessor: an alias for `Reg<SPR1_SPEC>`"]
pub type SPR1 = crate::Reg<spr1::SPR1_SPEC>;
#[doc = "Special Purpose Register (internal register with id = 17)"]
pub mod spr1;
#[doc = "SPR2 (r) register accessor: an alias for `Reg<SPR2_SPEC>`"]
pub type SPR2 = crate::Reg<spr2::SPR2_SPEC>;
#[doc = "Special Purpose Register (internal register with id = 18)"]
pub mod spr2;
#[doc = "SPR3 (r) register accessor: an alias for `Reg<SPR3_SPEC>`"]
pub type SPR3 = crate::Reg<spr3::SPR3_SPEC>;
#[doc = "Special Purpose Register (internal register with id = 19)"]
pub mod spr3;
#[doc = "SPR4 (r) register accessor: an alias for `Reg<SPR4_SPEC>`"]
pub type SPR4 = crate::Reg<spr4::SPR4_SPEC>;
#[doc = "Special Purpose Register (internal register with id = 20)"]
pub mod spr4;
#[doc = "SPR5 (r) register accessor: an alias for `Reg<SPR5_SPEC>`"]
pub type SPR5 = crate::Reg<spr5::SPR5_SPEC>;
#[doc = "Special Purpose Register (internal register with id = 21)"]
pub mod spr5;
#[doc = "SPR6 (r) register accessor: an alias for `Reg<SPR6_SPEC>`"]
pub type SPR6 = crate::Reg<spr6::SPR6_SPEC>;
#[doc = "Special Purpose Register (internal register with id = 22)"]
pub mod spr6;
#[doc = "SPR7 (r) register accessor: an alias for `Reg<SPR7_SPEC>`"]
pub type SPR7 = crate::Reg<spr7::SPR7_SPEC>;
#[doc = "Special Purpose Register (internal register with id = 23)"]
pub mod spr7;
