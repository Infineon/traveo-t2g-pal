#[doc = r"Register block"]
#[repr(C)]
pub struct TCON0 {
    #[doc = "0x00..0x100 - The 64 Sequencer Position Definitions registers define the X/Y scan positions of the sequencers, hold their output value and assign the sequencer to an odd/even field"]
    pub ssqcnts: [SSQCNTS; 64],
    _reserved1: [u8; 0x0300],
    #[doc = "0x400 - This bitfield sets the sequencer cycle length. The value set here -1 is the number of sequencer cycles"]
    pub ssqcycle: SSQCYCLE,
    #[doc = "0x404 - TCON Software Reset - Reset all tcon registers except configuration registers. Detailed description in specification document Note 1/ if tsig\\[11\\]
pulse=n*pixel_period, (n-1)*0xFF will be blended between ResetWordStart and ResetWordEnd into miniLVDS stream Note 2/ if( EnResetWord=0) Reset-Pulse (ResetWordStart,ResetWordEnd) won't be blended into miniLVDS stream. Pixels will be transferred unchanged"]
    pub swreset: SWRESET,
    #[doc = "0x408 - TCON Control register"]
    pub tcon_ctrl: TCON_CTRL,
    #[doc = "0x40c - Controls inversion of output polarity when connected IO cells operate in RSDS mode"]
    pub rsdsinvctrl: RSDSINVCTRL,
    #[doc = "0x410 - Mapping of 24 bit RGB or Timing Generator TSig\\[5-0\\]
to bit 0 .. 3"]
    pub mapbit3_0: MAPBIT3_0,
    #[doc = "0x414 - Mapping of 24 bit RGB or Timing Generator TSig\\[5-0\\]
to bit 4 .. 7"]
    pub mapbit7_4: MAPBIT7_4,
    #[doc = "0x418 - Mapping of 24 bit RGB or Timing Generator TSig\\[5-0\\]
to bit 8 .. 11"]
    pub mapbit11_8: MAPBIT11_8,
    #[doc = "0x41c - Mapping of 24 bit RGB or Timing Generator TSig\\[5-0\\]
to bit 12 .. 15"]
    pub mapbit15_12: MAPBIT15_12,
    #[doc = "0x420 - Mapping of 24 bit RGB or Timing Generator TSig\\[5-0\\]
to bit 16 .. 19"]
    pub mapbit19_16: MAPBIT19_16,
    #[doc = "0x424 - Mapping of 24 bit RGB or Timing Generator TSig\\[5-0\\]
to bit 20 .. 23"]
    pub mapbit23_20: MAPBIT23_20,
    #[doc = "0x428 - Mapping of 24 bit RGB or Timing Generator TSig\\[5-0\\]
to bit 24 .. 27"]
    pub mapbit27_24: MAPBIT27_24,
    #[doc = "0x42c - Same as MapBit3_0 for 2nd channel"]
    pub mapbit3_0_dual: MAPBIT3_0_DUAL,
    #[doc = "0x430 - Same as MapBit7_4 for 2nd channel"]
    pub mapbit7_4_dual: MAPBIT7_4_DUAL,
    #[doc = "0x434 - Same as MapBit11_8 for 2nd channel"]
    pub mapbit11_8_dual: MAPBIT11_8_DUAL,
    #[doc = "0x438 - Same as MapBit15_12 for 2nd channel"]
    pub mapbit15_12_dual: MAPBIT15_12_DUAL,
    #[doc = "0x43c - Same as MapBit19_16 for 2nd channel"]
    pub mapbit19_16_dual: MAPBIT19_16_DUAL,
    #[doc = "0x440 - Same as MapBit23_20 for 2nd channel"]
    pub mapbit23_20_dual: MAPBIT23_20_DUAL,
    #[doc = "0x444 - Same as MapBit27_24 for 2nd channel"]
    pub mapbit27_24_dual: MAPBIT27_24_DUAL,
    #[doc = "0x448 - Sync pulse generator 0, 'Switch on' position"]
    pub spg0poson: SPG0POSON,
    #[doc = "0x44c - The Sequencer Pulse Generator 0 Mask Enable register is used to mask the enable of SPG 0"]
    pub spg0maskon: SPG0MASKON,
    #[doc = "0x450 - Sync pulse generator 0, 'Switch off' position"]
    pub spg0posoff: SPG0POSOFF,
    #[doc = "0x454 - The Sequencer Pulse Generator 0 Mask Enable register is used to mask the disable of SPG 0"]
    pub spg0maskoff: SPG0MASKOFF,
    #[doc = "0x458 - Sync pulse generator 1, 'Switch on' position"]
    pub spg1poson: SPG1POSON,
    #[doc = "0x45c - The Sequencer Pulse Generator 1 Mask Enable register is used to mask the enable of SPG 1"]
    pub spg1maskon: SPG1MASKON,
    #[doc = "0x460 - Sync pulse generator 1, 'Switch off' position"]
    pub spg1posoff: SPG1POSOFF,
    #[doc = "0x464 - The Sequencer Pulse Generator 1 Mask Enable register is used to mask the disable of SPG 1"]
    pub spg1maskoff: SPG1MASKOFF,
    #[doc = "0x468 - Sync pulse generator 2, 'Switch on' position"]
    pub spg2poson: SPG2POSON,
    #[doc = "0x46c - The Sequencer Pulse Generator 2 Mask Enable register is used to mask the enable of SPG 2"]
    pub spg2maskon: SPG2MASKON,
    #[doc = "0x470 - Sync pulse generator 2, 'Switch off' position"]
    pub spg2posoff: SPG2POSOFF,
    #[doc = "0x474 - The Sequencer Pulse Generator 2 Mask Enable register is used to mask the disable of SPG 2"]
    pub spg2maskoff: SPG2MASKOFF,
    #[doc = "0x478 - Sync pulse generator 3, 'Switch on' position"]
    pub spg3poson: SPG3POSON,
    #[doc = "0x47c - The Sequencer Pulse Generator 3 Mask Enable register is used to mask the enable of SPG 3"]
    pub spg3maskon: SPG3MASKON,
    #[doc = "0x480 - Sync pulse generator 3, 'Switch off' position"]
    pub spg3posoff: SPG3POSOFF,
    #[doc = "0x484 - The Sequencer Pulse Generator 3 Mask Enable register is used to mask the disable of SPG 3"]
    pub spg3maskoff: SPG3MASKOFF,
    #[doc = "0x488 - Sync pulse generator 4, 'Switch on' position"]
    pub spg4poson: SPG4POSON,
    #[doc = "0x48c - The Sequencer Pulse Generator 4 Mask Enable register is used to mask the enable of SPG 4"]
    pub spg4maskon: SPG4MASKON,
    #[doc = "0x490 - Sync pulse generator 4, 'Switch off' position"]
    pub spg4posoff: SPG4POSOFF,
    #[doc = "0x494 - The Sequencer Pulse Generator 4 Mask Enable register is used to mask the disable of SPG 4"]
    pub spg4maskoff: SPG4MASKOFF,
    #[doc = "0x498 - Sync pulse generator 5, 'Switch on' position"]
    pub spg5poson: SPG5POSON,
    #[doc = "0x49c - The Sequencer Pulse Generator 5 Mask Enable register is used to mask the enable of SPG 5"]
    pub spg5maskon: SPG5MASKON,
    #[doc = "0x4a0 - Sync pulse generator 5, 'Switch off' position"]
    pub spg5posoff: SPG5POSOFF,
    #[doc = "0x4a4 - The Sequencer Pulse Generator 5 Mask Enable register is used to mask the disable of SPG 5"]
    pub spg5maskoff: SPG5MASKOFF,
    #[doc = "0x4a8 - Sync pulse generator 6, 'Switch on' position"]
    pub spg6poson: SPG6POSON,
    #[doc = "0x4ac - The Sequencer Pulse Generator 6 Mask Enable register is used to mask the enable of SPG 6"]
    pub spg6maskon: SPG6MASKON,
    #[doc = "0x4b0 - Sync pulse generator 6, 'Switch off' position"]
    pub spg6posoff: SPG6POSOFF,
    #[doc = "0x4b4 - The Sequencer Pulse Generator 6 Mask Enable register is used to mask the disable of SPG 6"]
    pub spg6maskoff: SPG6MASKOFF,
    #[doc = "0x4b8 - Sync pulse generator 7, 'Switch on' position"]
    pub spg7poson: SPG7POSON,
    #[doc = "0x4bc - The Sequencer Pulse Generator 7 Mask Enable register is used to mask the enable of SPG 7"]
    pub spg7maskon: SPG7MASKON,
    #[doc = "0x4c0 - Sync pulse generator 7, 'Switch off' position"]
    pub spg7posoff: SPG7POSOFF,
    #[doc = "0x4c4 - The Sequencer Pulse Generator 7 Mask Enable register is used to mask the disable of SPG 7"]
    pub spg7maskoff: SPG7MASKOFF,
    #[doc = "0x4c8 - Sync pulse generator 8, 'Switch on' position"]
    pub spg8poson: SPG8POSON,
    #[doc = "0x4cc - The Sequencer Pulse Generator 8 Mask Enable register is used to mask the enable of SPG 8"]
    pub spg8maskon: SPG8MASKON,
    #[doc = "0x4d0 - Sync pulse generator 8, 'Switch off' position"]
    pub spg8posoff: SPG8POSOFF,
    #[doc = "0x4d4 - The Sequencer Pulse Generator 8 Mask Enable register is used to mask the disable of SPG 8"]
    pub spg8maskoff: SPG8MASKOFF,
    #[doc = "0x4d8 - Sync pulse generator 9, 'Switch on' position"]
    pub spg9poson: SPG9POSON,
    #[doc = "0x4dc - The Sequencer Pulse Generator 9 Mask Enable register is used to mask the enable of SPG 9"]
    pub spg9maskon: SPG9MASKON,
    #[doc = "0x4e0 - Sync pulse generator 9, 'Switch off' position"]
    pub spg9posoff: SPG9POSOFF,
    #[doc = "0x4e4 - The Sequencer Pulse Generator 9 Mask Enable register is used to mask the disable of SPG 9"]
    pub spg9maskoff: SPG9MASKOFF,
    #[doc = "0x4e8 - Sync pulse generator 10, 'Switch on' position"]
    pub spg10poson: SPG10POSON,
    #[doc = "0x4ec - The Sequencer Pulse Generator 10 Mask Enable register is used to mask the enable of SPG 10"]
    pub spg10maskon: SPG10MASKON,
    #[doc = "0x4f0 - Sync pulse generator 10, 'Switch off' position"]
    pub spg10posoff: SPG10POSOFF,
    #[doc = "0x4f4 - The Sequencer Pulse Generator 10 Mask Enable register is used to mask the disable of SPG 10"]
    pub spg10maskoff: SPG10MASKOFF,
    #[doc = "0x4f8 - Sync pulse generator 11, 'Switch on' position"]
    pub spg11poson: SPG11POSON,
    #[doc = "0x4fc - The Sequencer Pulse Generator 11 Mask Enable register is used to mask the enable of SPG 11"]
    pub spg11maskon: SPG11MASKON,
    #[doc = "0x500 - Sync pulse generator 11, 'Switch off' position"]
    pub spg11posoff: SPG11POSOFF,
    #[doc = "0x504 - The Sequencer Pulse Generator 11 Mask Enable register is used to mask the disable of SPG 11"]
    pub spg11maskoff: SPG11MASKOFF,
    #[doc = "0x508 - Selection of input signals of sync mixer"]
    pub smx0sigs: SMX0SIGS,
    #[doc = "0x50c - The sync mixer output is the result of the function table a=s4*2**4+s3*2**3+s2*2**2+s1*2**1+s0*2**0 whereby a is bit number and s result of sync mixer input selection"]
    pub smx0fcttable: SMX0FCTTABLE,
    #[doc = "0x510 - Selection of input signals of sync mixer"]
    pub smx1sigs: SMX1SIGS,
    #[doc = "0x514 - The sync mixer output is the result of the function table a=s4*2**4+s3*2**3+s2*2**2+s1*2**1+s0*2**0 whereby a is bit number and s result of sync mixer input selection"]
    pub smx1fcttable: SMX1FCTTABLE,
    #[doc = "0x518 - Selection of input signals of sync mixer"]
    pub smx2sigs: SMX2SIGS,
    #[doc = "0x51c - The sync mixer output is the result of the function table a=s4*2**4+s**3*2**3+s2*2**2+s1*2**1+s0*2**0 whereby a is bit number and s result of sync mixer input selection"]
    pub smx2fcttable: SMX2FCTTABLE,
    #[doc = "0x520 - Selection of input signals of sync mixer"]
    pub smx3sigs: SMX3SIGS,
    #[doc = "0x524 - The sync mixer output is the result of the function table a=s4*2**4+s3*2**3+s2*2**2+s1*2**1+s0*2**0 whereby a is bit number and s result of sync mixer input selection"]
    pub smx3fcttable: SMX3FCTTABLE,
    #[doc = "0x528 - Selection of input signals of sync mixer"]
    pub smx4sigs: SMX4SIGS,
    #[doc = "0x52c - The sync mixer output is the result of the function table a=s4*2**4+s3*2**3+s2*2**2+s1*2**1+s0*2**0 whereby a is bit number and s result of sync mixer input selection"]
    pub smx4fcttable: SMX4FCTTABLE,
    #[doc = "0x530 - Selection of input signals of sync mixer"]
    pub smx5sigs: SMX5SIGS,
    #[doc = "0x534 - The sync mixer output is the result of the function table a=s4*2**4+s3*2**3+s2*2**2+s1*2**1+s0*2**0 whereby a is bit number and s result of sync mixer input selection"]
    pub smx5fcttable: SMX5FCTTABLE,
    #[doc = "0x538 - Selection of input signals of sync mixer"]
    pub smx6sigs: SMX6SIGS,
    #[doc = "0x53c - The sync mixer output is the result of the function table a=s4*2**4+s3*2**3+s2*2**2+s1*2**1+s0*2**0 whereby a is bit number and s result of sync mixer input selection"]
    pub smx6fcttable: SMX6FCTTABLE,
    #[doc = "0x540 - Selection of input signals of sync mixer"]
    pub smx7sigs: SMX7SIGS,
    #[doc = "0x544 - The sync mixer output is the result of the function table a=s4*2**4+s3*2**3+s2*2**2+s1*2**1+s0*2**0 whereby a is bit number and s result of sync mixer input selection"]
    pub smx7fcttable: SMX7FCTTABLE,
    #[doc = "0x548 - Selection of input signals of sync mixer"]
    pub smx8sigs: SMX8SIGS,
    #[doc = "0x54c - The sync mixer output is the result of the function table a=s4*2**4+s3*2**3+s2*2**2+s1*2**1+s0*2**0 whereby a is bit number and s result of sync mixer input selection"]
    pub smx8fcttable: SMX8FCTTABLE,
    #[doc = "0x550 - Selection of input signals of sync mixer"]
    pub smx9sigs: SMX9SIGS,
    #[doc = "0x554 - The sync mixer output is the result of the function table a=s4*2**4+s3*2**3+s2*2**2+s1*2**1+s0*2**0 whereby a is bit number and s result of sync mixer input selection"]
    pub smx9fcttable: SMX9FCTTABLE,
    #[doc = "0x558 - Selection of input signals of sync mixer"]
    pub smx10sigs: SMX10SIGS,
    #[doc = "0x55c - The sync mixer output is the result of the function table a=s4*2**4+s3*2**3+s2*2**2+s1*2**1+s0*2**0 whereby a is bit number and s result of sync mixer input selection"]
    pub smx10fcttable: SMX10FCTTABLE,
    #[doc = "0x560 - Selection of input signals of sync mixer"]
    pub smx11sigs: SMX11SIGS,
    #[doc = "0x564 - The sync mixer output is the result of the function table a=s4*2**4+s3*2**3+s2*2**2+s1*2**1+s0*2**0 whereby a is bit number and s result of sync mixer input selection"]
    pub smx11fcttable: SMX11FCTTABLE,
}
#[doc = "SSQCNTS (rw) register accessor: an alias for `Reg<SSQCNTS_SPEC>`"]
pub type SSQCNTS = crate::Reg<ssqcnts::SSQCNTS_SPEC>;
#[doc = "The 64 Sequencer Position Definitions registers define the X/Y scan positions of the sequencers, hold their output value and assign the sequencer to an odd/even field"]
pub mod ssqcnts;
#[doc = "SSQCYCLE (rw) register accessor: an alias for `Reg<SSQCYCLE_SPEC>`"]
pub type SSQCYCLE = crate::Reg<ssqcycle::SSQCYCLE_SPEC>;
#[doc = "This bitfield sets the sequencer cycle length. The value set here -1 is the number of sequencer cycles"]
pub mod ssqcycle;
#[doc = "SWRESET (rw) register accessor: an alias for `Reg<SWRESET_SPEC>`"]
pub type SWRESET = crate::Reg<swreset::SWRESET_SPEC>;
#[doc = "TCON Software Reset - Reset all tcon registers except configuration registers. Detailed description in specification document Note 1/ if tsig\\[11\\]
pulse=n*pixel_period, (n-1)*0xFF will be blended between ResetWordStart and ResetWordEnd into miniLVDS stream Note 2/ if( EnResetWord=0) Reset-Pulse (ResetWordStart,ResetWordEnd) won't be blended into miniLVDS stream. Pixels will be transferred unchanged"]
pub mod swreset;
#[doc = "TCON_CTRL (rw) register accessor: an alias for `Reg<TCON_CTRL_SPEC>`"]
pub type TCON_CTRL = crate::Reg<tcon_ctrl::TCON_CTRL_SPEC>;
#[doc = "TCON Control register"]
pub mod tcon_ctrl;
#[doc = "RSDSINVCTRL (rw) register accessor: an alias for `Reg<RSDSINVCTRL_SPEC>`"]
pub type RSDSINVCTRL = crate::Reg<rsdsinvctrl::RSDSINVCTRL_SPEC>;
#[doc = "Controls inversion of output polarity when connected IO cells operate in RSDS mode"]
pub mod rsdsinvctrl;
#[doc = "MAPBIT3_0 (rw) register accessor: an alias for `Reg<MAPBIT3_0_SPEC>`"]
pub type MAPBIT3_0 = crate::Reg<mapbit3_0::MAPBIT3_0_SPEC>;
#[doc = "Mapping of 24 bit RGB or Timing Generator TSig\\[5-0\\]
to bit 0 .. 3"]
pub mod mapbit3_0;
#[doc = "MAPBIT7_4 (rw) register accessor: an alias for `Reg<MAPBIT7_4_SPEC>`"]
pub type MAPBIT7_4 = crate::Reg<mapbit7_4::MAPBIT7_4_SPEC>;
#[doc = "Mapping of 24 bit RGB or Timing Generator TSig\\[5-0\\]
to bit 4 .. 7"]
pub mod mapbit7_4;
#[doc = "MAPBIT11_8 (rw) register accessor: an alias for `Reg<MAPBIT11_8_SPEC>`"]
pub type MAPBIT11_8 = crate::Reg<mapbit11_8::MAPBIT11_8_SPEC>;
#[doc = "Mapping of 24 bit RGB or Timing Generator TSig\\[5-0\\]
to bit 8 .. 11"]
pub mod mapbit11_8;
#[doc = "MAPBIT15_12 (rw) register accessor: an alias for `Reg<MAPBIT15_12_SPEC>`"]
pub type MAPBIT15_12 = crate::Reg<mapbit15_12::MAPBIT15_12_SPEC>;
#[doc = "Mapping of 24 bit RGB or Timing Generator TSig\\[5-0\\]
to bit 12 .. 15"]
pub mod mapbit15_12;
#[doc = "MAPBIT19_16 (rw) register accessor: an alias for `Reg<MAPBIT19_16_SPEC>`"]
pub type MAPBIT19_16 = crate::Reg<mapbit19_16::MAPBIT19_16_SPEC>;
#[doc = "Mapping of 24 bit RGB or Timing Generator TSig\\[5-0\\]
to bit 16 .. 19"]
pub mod mapbit19_16;
#[doc = "MAPBIT23_20 (rw) register accessor: an alias for `Reg<MAPBIT23_20_SPEC>`"]
pub type MAPBIT23_20 = crate::Reg<mapbit23_20::MAPBIT23_20_SPEC>;
#[doc = "Mapping of 24 bit RGB or Timing Generator TSig\\[5-0\\]
to bit 20 .. 23"]
pub mod mapbit23_20;
#[doc = "MAPBIT27_24 (rw) register accessor: an alias for `Reg<MAPBIT27_24_SPEC>`"]
pub type MAPBIT27_24 = crate::Reg<mapbit27_24::MAPBIT27_24_SPEC>;
#[doc = "Mapping of 24 bit RGB or Timing Generator TSig\\[5-0\\]
to bit 24 .. 27"]
pub mod mapbit27_24;
#[doc = "MAPBIT3_0_DUAL (rw) register accessor: an alias for `Reg<MAPBIT3_0_DUAL_SPEC>`"]
pub type MAPBIT3_0_DUAL = crate::Reg<mapbit3_0_dual::MAPBIT3_0_DUAL_SPEC>;
#[doc = "Same as MapBit3_0 for 2nd channel"]
pub mod mapbit3_0_dual;
#[doc = "MAPBIT7_4_DUAL (rw) register accessor: an alias for `Reg<MAPBIT7_4_DUAL_SPEC>`"]
pub type MAPBIT7_4_DUAL = crate::Reg<mapbit7_4_dual::MAPBIT7_4_DUAL_SPEC>;
#[doc = "Same as MapBit7_4 for 2nd channel"]
pub mod mapbit7_4_dual;
#[doc = "MAPBIT11_8_DUAL (rw) register accessor: an alias for `Reg<MAPBIT11_8_DUAL_SPEC>`"]
pub type MAPBIT11_8_DUAL = crate::Reg<mapbit11_8_dual::MAPBIT11_8_DUAL_SPEC>;
#[doc = "Same as MapBit11_8 for 2nd channel"]
pub mod mapbit11_8_dual;
#[doc = "MAPBIT15_12_DUAL (rw) register accessor: an alias for `Reg<MAPBIT15_12_DUAL_SPEC>`"]
pub type MAPBIT15_12_DUAL = crate::Reg<mapbit15_12_dual::MAPBIT15_12_DUAL_SPEC>;
#[doc = "Same as MapBit15_12 for 2nd channel"]
pub mod mapbit15_12_dual;
#[doc = "MAPBIT19_16_DUAL (rw) register accessor: an alias for `Reg<MAPBIT19_16_DUAL_SPEC>`"]
pub type MAPBIT19_16_DUAL = crate::Reg<mapbit19_16_dual::MAPBIT19_16_DUAL_SPEC>;
#[doc = "Same as MapBit19_16 for 2nd channel"]
pub mod mapbit19_16_dual;
#[doc = "MAPBIT23_20_DUAL (rw) register accessor: an alias for `Reg<MAPBIT23_20_DUAL_SPEC>`"]
pub type MAPBIT23_20_DUAL = crate::Reg<mapbit23_20_dual::MAPBIT23_20_DUAL_SPEC>;
#[doc = "Same as MapBit23_20 for 2nd channel"]
pub mod mapbit23_20_dual;
#[doc = "MAPBIT27_24_DUAL (rw) register accessor: an alias for `Reg<MAPBIT27_24_DUAL_SPEC>`"]
pub type MAPBIT27_24_DUAL = crate::Reg<mapbit27_24_dual::MAPBIT27_24_DUAL_SPEC>;
#[doc = "Same as MapBit27_24 for 2nd channel"]
pub mod mapbit27_24_dual;
#[doc = "SPG0POSON (rw) register accessor: an alias for `Reg<SPG0POSON_SPEC>`"]
pub type SPG0POSON = crate::Reg<spg0poson::SPG0POSON_SPEC>;
#[doc = "Sync pulse generator 0, 'Switch on' position"]
pub mod spg0poson;
#[doc = "SPG0MASKON (rw) register accessor: an alias for `Reg<SPG0MASKON_SPEC>`"]
pub type SPG0MASKON = crate::Reg<spg0maskon::SPG0MASKON_SPEC>;
#[doc = "The Sequencer Pulse Generator 0 Mask Enable register is used to mask the enable of SPG 0"]
pub mod spg0maskon;
#[doc = "SPG0POSOFF (rw) register accessor: an alias for `Reg<SPG0POSOFF_SPEC>`"]
pub type SPG0POSOFF = crate::Reg<spg0posoff::SPG0POSOFF_SPEC>;
#[doc = "Sync pulse generator 0, 'Switch off' position"]
pub mod spg0posoff;
#[doc = "SPG0MASKOFF (rw) register accessor: an alias for `Reg<SPG0MASKOFF_SPEC>`"]
pub type SPG0MASKOFF = crate::Reg<spg0maskoff::SPG0MASKOFF_SPEC>;
#[doc = "The Sequencer Pulse Generator 0 Mask Enable register is used to mask the disable of SPG 0"]
pub mod spg0maskoff;
#[doc = "SPG1POSON (rw) register accessor: an alias for `Reg<SPG1POSON_SPEC>`"]
pub type SPG1POSON = crate::Reg<spg1poson::SPG1POSON_SPEC>;
#[doc = "Sync pulse generator 1, 'Switch on' position"]
pub mod spg1poson;
#[doc = "SPG1MASKON (rw) register accessor: an alias for `Reg<SPG1MASKON_SPEC>`"]
pub type SPG1MASKON = crate::Reg<spg1maskon::SPG1MASKON_SPEC>;
#[doc = "The Sequencer Pulse Generator 1 Mask Enable register is used to mask the enable of SPG 1"]
pub mod spg1maskon;
#[doc = "SPG1POSOFF (rw) register accessor: an alias for `Reg<SPG1POSOFF_SPEC>`"]
pub type SPG1POSOFF = crate::Reg<spg1posoff::SPG1POSOFF_SPEC>;
#[doc = "Sync pulse generator 1, 'Switch off' position"]
pub mod spg1posoff;
#[doc = "SPG1MASKOFF (rw) register accessor: an alias for `Reg<SPG1MASKOFF_SPEC>`"]
pub type SPG1MASKOFF = crate::Reg<spg1maskoff::SPG1MASKOFF_SPEC>;
#[doc = "The Sequencer Pulse Generator 1 Mask Enable register is used to mask the disable of SPG 1"]
pub mod spg1maskoff;
#[doc = "SPG2POSON (rw) register accessor: an alias for `Reg<SPG2POSON_SPEC>`"]
pub type SPG2POSON = crate::Reg<spg2poson::SPG2POSON_SPEC>;
#[doc = "Sync pulse generator 2, 'Switch on' position"]
pub mod spg2poson;
#[doc = "SPG2MASKON (rw) register accessor: an alias for `Reg<SPG2MASKON_SPEC>`"]
pub type SPG2MASKON = crate::Reg<spg2maskon::SPG2MASKON_SPEC>;
#[doc = "The Sequencer Pulse Generator 2 Mask Enable register is used to mask the enable of SPG 2"]
pub mod spg2maskon;
#[doc = "SPG2POSOFF (rw) register accessor: an alias for `Reg<SPG2POSOFF_SPEC>`"]
pub type SPG2POSOFF = crate::Reg<spg2posoff::SPG2POSOFF_SPEC>;
#[doc = "Sync pulse generator 2, 'Switch off' position"]
pub mod spg2posoff;
#[doc = "SPG2MASKOFF (rw) register accessor: an alias for `Reg<SPG2MASKOFF_SPEC>`"]
pub type SPG2MASKOFF = crate::Reg<spg2maskoff::SPG2MASKOFF_SPEC>;
#[doc = "The Sequencer Pulse Generator 2 Mask Enable register is used to mask the disable of SPG 2"]
pub mod spg2maskoff;
#[doc = "SPG3POSON (rw) register accessor: an alias for `Reg<SPG3POSON_SPEC>`"]
pub type SPG3POSON = crate::Reg<spg3poson::SPG3POSON_SPEC>;
#[doc = "Sync pulse generator 3, 'Switch on' position"]
pub mod spg3poson;
#[doc = "SPG3MASKON (rw) register accessor: an alias for `Reg<SPG3MASKON_SPEC>`"]
pub type SPG3MASKON = crate::Reg<spg3maskon::SPG3MASKON_SPEC>;
#[doc = "The Sequencer Pulse Generator 3 Mask Enable register is used to mask the enable of SPG 3"]
pub mod spg3maskon;
#[doc = "SPG3POSOFF (rw) register accessor: an alias for `Reg<SPG3POSOFF_SPEC>`"]
pub type SPG3POSOFF = crate::Reg<spg3posoff::SPG3POSOFF_SPEC>;
#[doc = "Sync pulse generator 3, 'Switch off' position"]
pub mod spg3posoff;
#[doc = "SPG3MASKOFF (rw) register accessor: an alias for `Reg<SPG3MASKOFF_SPEC>`"]
pub type SPG3MASKOFF = crate::Reg<spg3maskoff::SPG3MASKOFF_SPEC>;
#[doc = "The Sequencer Pulse Generator 3 Mask Enable register is used to mask the disable of SPG 3"]
pub mod spg3maskoff;
#[doc = "SPG4POSON (rw) register accessor: an alias for `Reg<SPG4POSON_SPEC>`"]
pub type SPG4POSON = crate::Reg<spg4poson::SPG4POSON_SPEC>;
#[doc = "Sync pulse generator 4, 'Switch on' position"]
pub mod spg4poson;
#[doc = "SPG4MASKON (rw) register accessor: an alias for `Reg<SPG4MASKON_SPEC>`"]
pub type SPG4MASKON = crate::Reg<spg4maskon::SPG4MASKON_SPEC>;
#[doc = "The Sequencer Pulse Generator 4 Mask Enable register is used to mask the enable of SPG 4"]
pub mod spg4maskon;
#[doc = "SPG4POSOFF (rw) register accessor: an alias for `Reg<SPG4POSOFF_SPEC>`"]
pub type SPG4POSOFF = crate::Reg<spg4posoff::SPG4POSOFF_SPEC>;
#[doc = "Sync pulse generator 4, 'Switch off' position"]
pub mod spg4posoff;
#[doc = "SPG4MASKOFF (rw) register accessor: an alias for `Reg<SPG4MASKOFF_SPEC>`"]
pub type SPG4MASKOFF = crate::Reg<spg4maskoff::SPG4MASKOFF_SPEC>;
#[doc = "The Sequencer Pulse Generator 4 Mask Enable register is used to mask the disable of SPG 4"]
pub mod spg4maskoff;
#[doc = "SPG5POSON (rw) register accessor: an alias for `Reg<SPG5POSON_SPEC>`"]
pub type SPG5POSON = crate::Reg<spg5poson::SPG5POSON_SPEC>;
#[doc = "Sync pulse generator 5, 'Switch on' position"]
pub mod spg5poson;
#[doc = "SPG5MASKON (rw) register accessor: an alias for `Reg<SPG5MASKON_SPEC>`"]
pub type SPG5MASKON = crate::Reg<spg5maskon::SPG5MASKON_SPEC>;
#[doc = "The Sequencer Pulse Generator 5 Mask Enable register is used to mask the enable of SPG 5"]
pub mod spg5maskon;
#[doc = "SPG5POSOFF (rw) register accessor: an alias for `Reg<SPG5POSOFF_SPEC>`"]
pub type SPG5POSOFF = crate::Reg<spg5posoff::SPG5POSOFF_SPEC>;
#[doc = "Sync pulse generator 5, 'Switch off' position"]
pub mod spg5posoff;
#[doc = "SPG5MASKOFF (rw) register accessor: an alias for `Reg<SPG5MASKOFF_SPEC>`"]
pub type SPG5MASKOFF = crate::Reg<spg5maskoff::SPG5MASKOFF_SPEC>;
#[doc = "The Sequencer Pulse Generator 5 Mask Enable register is used to mask the disable of SPG 5"]
pub mod spg5maskoff;
#[doc = "SPG6POSON (rw) register accessor: an alias for `Reg<SPG6POSON_SPEC>`"]
pub type SPG6POSON = crate::Reg<spg6poson::SPG6POSON_SPEC>;
#[doc = "Sync pulse generator 6, 'Switch on' position"]
pub mod spg6poson;
#[doc = "SPG6MASKON (rw) register accessor: an alias for `Reg<SPG6MASKON_SPEC>`"]
pub type SPG6MASKON = crate::Reg<spg6maskon::SPG6MASKON_SPEC>;
#[doc = "The Sequencer Pulse Generator 6 Mask Enable register is used to mask the enable of SPG 6"]
pub mod spg6maskon;
#[doc = "SPG6POSOFF (rw) register accessor: an alias for `Reg<SPG6POSOFF_SPEC>`"]
pub type SPG6POSOFF = crate::Reg<spg6posoff::SPG6POSOFF_SPEC>;
#[doc = "Sync pulse generator 6, 'Switch off' position"]
pub mod spg6posoff;
#[doc = "SPG6MASKOFF (rw) register accessor: an alias for `Reg<SPG6MASKOFF_SPEC>`"]
pub type SPG6MASKOFF = crate::Reg<spg6maskoff::SPG6MASKOFF_SPEC>;
#[doc = "The Sequencer Pulse Generator 6 Mask Enable register is used to mask the disable of SPG 6"]
pub mod spg6maskoff;
#[doc = "SPG7POSON (rw) register accessor: an alias for `Reg<SPG7POSON_SPEC>`"]
pub type SPG7POSON = crate::Reg<spg7poson::SPG7POSON_SPEC>;
#[doc = "Sync pulse generator 7, 'Switch on' position"]
pub mod spg7poson;
#[doc = "SPG7MASKON (rw) register accessor: an alias for `Reg<SPG7MASKON_SPEC>`"]
pub type SPG7MASKON = crate::Reg<spg7maskon::SPG7MASKON_SPEC>;
#[doc = "The Sequencer Pulse Generator 7 Mask Enable register is used to mask the enable of SPG 7"]
pub mod spg7maskon;
#[doc = "SPG7POSOFF (rw) register accessor: an alias for `Reg<SPG7POSOFF_SPEC>`"]
pub type SPG7POSOFF = crate::Reg<spg7posoff::SPG7POSOFF_SPEC>;
#[doc = "Sync pulse generator 7, 'Switch off' position"]
pub mod spg7posoff;
#[doc = "SPG7MASKOFF (rw) register accessor: an alias for `Reg<SPG7MASKOFF_SPEC>`"]
pub type SPG7MASKOFF = crate::Reg<spg7maskoff::SPG7MASKOFF_SPEC>;
#[doc = "The Sequencer Pulse Generator 7 Mask Enable register is used to mask the disable of SPG 7"]
pub mod spg7maskoff;
#[doc = "SPG8POSON (rw) register accessor: an alias for `Reg<SPG8POSON_SPEC>`"]
pub type SPG8POSON = crate::Reg<spg8poson::SPG8POSON_SPEC>;
#[doc = "Sync pulse generator 8, 'Switch on' position"]
pub mod spg8poson;
#[doc = "SPG8MASKON (rw) register accessor: an alias for `Reg<SPG8MASKON_SPEC>`"]
pub type SPG8MASKON = crate::Reg<spg8maskon::SPG8MASKON_SPEC>;
#[doc = "The Sequencer Pulse Generator 8 Mask Enable register is used to mask the enable of SPG 8"]
pub mod spg8maskon;
#[doc = "SPG8POSOFF (rw) register accessor: an alias for `Reg<SPG8POSOFF_SPEC>`"]
pub type SPG8POSOFF = crate::Reg<spg8posoff::SPG8POSOFF_SPEC>;
#[doc = "Sync pulse generator 8, 'Switch off' position"]
pub mod spg8posoff;
#[doc = "SPG8MASKOFF (rw) register accessor: an alias for `Reg<SPG8MASKOFF_SPEC>`"]
pub type SPG8MASKOFF = crate::Reg<spg8maskoff::SPG8MASKOFF_SPEC>;
#[doc = "The Sequencer Pulse Generator 8 Mask Enable register is used to mask the disable of SPG 8"]
pub mod spg8maskoff;
#[doc = "SPG9POSON (rw) register accessor: an alias for `Reg<SPG9POSON_SPEC>`"]
pub type SPG9POSON = crate::Reg<spg9poson::SPG9POSON_SPEC>;
#[doc = "Sync pulse generator 9, 'Switch on' position"]
pub mod spg9poson;
#[doc = "SPG9MASKON (rw) register accessor: an alias for `Reg<SPG9MASKON_SPEC>`"]
pub type SPG9MASKON = crate::Reg<spg9maskon::SPG9MASKON_SPEC>;
#[doc = "The Sequencer Pulse Generator 9 Mask Enable register is used to mask the enable of SPG 9"]
pub mod spg9maskon;
#[doc = "SPG9POSOFF (rw) register accessor: an alias for `Reg<SPG9POSOFF_SPEC>`"]
pub type SPG9POSOFF = crate::Reg<spg9posoff::SPG9POSOFF_SPEC>;
#[doc = "Sync pulse generator 9, 'Switch off' position"]
pub mod spg9posoff;
#[doc = "SPG9MASKOFF (rw) register accessor: an alias for `Reg<SPG9MASKOFF_SPEC>`"]
pub type SPG9MASKOFF = crate::Reg<spg9maskoff::SPG9MASKOFF_SPEC>;
#[doc = "The Sequencer Pulse Generator 9 Mask Enable register is used to mask the disable of SPG 9"]
pub mod spg9maskoff;
#[doc = "SPG10POSON (rw) register accessor: an alias for `Reg<SPG10POSON_SPEC>`"]
pub type SPG10POSON = crate::Reg<spg10poson::SPG10POSON_SPEC>;
#[doc = "Sync pulse generator 10, 'Switch on' position"]
pub mod spg10poson;
#[doc = "SPG10MASKON (rw) register accessor: an alias for `Reg<SPG10MASKON_SPEC>`"]
pub type SPG10MASKON = crate::Reg<spg10maskon::SPG10MASKON_SPEC>;
#[doc = "The Sequencer Pulse Generator 10 Mask Enable register is used to mask the enable of SPG 10"]
pub mod spg10maskon;
#[doc = "SPG10POSOFF (rw) register accessor: an alias for `Reg<SPG10POSOFF_SPEC>`"]
pub type SPG10POSOFF = crate::Reg<spg10posoff::SPG10POSOFF_SPEC>;
#[doc = "Sync pulse generator 10, 'Switch off' position"]
pub mod spg10posoff;
#[doc = "SPG10MASKOFF (rw) register accessor: an alias for `Reg<SPG10MASKOFF_SPEC>`"]
pub type SPG10MASKOFF = crate::Reg<spg10maskoff::SPG10MASKOFF_SPEC>;
#[doc = "The Sequencer Pulse Generator 10 Mask Enable register is used to mask the disable of SPG 10"]
pub mod spg10maskoff;
#[doc = "SPG11POSON (rw) register accessor: an alias for `Reg<SPG11POSON_SPEC>`"]
pub type SPG11POSON = crate::Reg<spg11poson::SPG11POSON_SPEC>;
#[doc = "Sync pulse generator 11, 'Switch on' position"]
pub mod spg11poson;
#[doc = "SPG11MASKON (rw) register accessor: an alias for `Reg<SPG11MASKON_SPEC>`"]
pub type SPG11MASKON = crate::Reg<spg11maskon::SPG11MASKON_SPEC>;
#[doc = "The Sequencer Pulse Generator 11 Mask Enable register is used to mask the enable of SPG 11"]
pub mod spg11maskon;
#[doc = "SPG11POSOFF (rw) register accessor: an alias for `Reg<SPG11POSOFF_SPEC>`"]
pub type SPG11POSOFF = crate::Reg<spg11posoff::SPG11POSOFF_SPEC>;
#[doc = "Sync pulse generator 11, 'Switch off' position"]
pub mod spg11posoff;
#[doc = "SPG11MASKOFF (rw) register accessor: an alias for `Reg<SPG11MASKOFF_SPEC>`"]
pub type SPG11MASKOFF = crate::Reg<spg11maskoff::SPG11MASKOFF_SPEC>;
#[doc = "The Sequencer Pulse Generator 11 Mask Enable register is used to mask the disable of SPG 11"]
pub mod spg11maskoff;
#[doc = "SMX0SIGS (rw) register accessor: an alias for `Reg<SMX0SIGS_SPEC>`"]
pub type SMX0SIGS = crate::Reg<smx0sigs::SMX0SIGS_SPEC>;
#[doc = "Selection of input signals of sync mixer"]
pub mod smx0sigs;
#[doc = "SMX0FCTTABLE (rw) register accessor: an alias for `Reg<SMX0FCTTABLE_SPEC>`"]
pub type SMX0FCTTABLE = crate::Reg<smx0fcttable::SMX0FCTTABLE_SPEC>;
#[doc = "The sync mixer output is the result of the function table a=s4*2**4+s3*2**3+s2*2**2+s1*2**1+s0*2**0 whereby a is bit number and s result of sync mixer input selection"]
pub mod smx0fcttable;
#[doc = "SMX1SIGS (rw) register accessor: an alias for `Reg<SMX1SIGS_SPEC>`"]
pub type SMX1SIGS = crate::Reg<smx1sigs::SMX1SIGS_SPEC>;
#[doc = "Selection of input signals of sync mixer"]
pub mod smx1sigs;
#[doc = "SMX1FCTTABLE (rw) register accessor: an alias for `Reg<SMX1FCTTABLE_SPEC>`"]
pub type SMX1FCTTABLE = crate::Reg<smx1fcttable::SMX1FCTTABLE_SPEC>;
#[doc = "The sync mixer output is the result of the function table a=s4*2**4+s3*2**3+s2*2**2+s1*2**1+s0*2**0 whereby a is bit number and s result of sync mixer input selection"]
pub mod smx1fcttable;
#[doc = "SMX2SIGS (rw) register accessor: an alias for `Reg<SMX2SIGS_SPEC>`"]
pub type SMX2SIGS = crate::Reg<smx2sigs::SMX2SIGS_SPEC>;
#[doc = "Selection of input signals of sync mixer"]
pub mod smx2sigs;
#[doc = "SMX2FCTTABLE (rw) register accessor: an alias for `Reg<SMX2FCTTABLE_SPEC>`"]
pub type SMX2FCTTABLE = crate::Reg<smx2fcttable::SMX2FCTTABLE_SPEC>;
#[doc = "The sync mixer output is the result of the function table a=s4*2**4+s**3*2**3+s2*2**2+s1*2**1+s0*2**0 whereby a is bit number and s result of sync mixer input selection"]
pub mod smx2fcttable;
#[doc = "SMX3SIGS (rw) register accessor: an alias for `Reg<SMX3SIGS_SPEC>`"]
pub type SMX3SIGS = crate::Reg<smx3sigs::SMX3SIGS_SPEC>;
#[doc = "Selection of input signals of sync mixer"]
pub mod smx3sigs;
#[doc = "SMX3FCTTABLE (rw) register accessor: an alias for `Reg<SMX3FCTTABLE_SPEC>`"]
pub type SMX3FCTTABLE = crate::Reg<smx3fcttable::SMX3FCTTABLE_SPEC>;
#[doc = "The sync mixer output is the result of the function table a=s4*2**4+s3*2**3+s2*2**2+s1*2**1+s0*2**0 whereby a is bit number and s result of sync mixer input selection"]
pub mod smx3fcttable;
#[doc = "SMX4SIGS (rw) register accessor: an alias for `Reg<SMX4SIGS_SPEC>`"]
pub type SMX4SIGS = crate::Reg<smx4sigs::SMX4SIGS_SPEC>;
#[doc = "Selection of input signals of sync mixer"]
pub mod smx4sigs;
#[doc = "SMX4FCTTABLE (rw) register accessor: an alias for `Reg<SMX4FCTTABLE_SPEC>`"]
pub type SMX4FCTTABLE = crate::Reg<smx4fcttable::SMX4FCTTABLE_SPEC>;
#[doc = "The sync mixer output is the result of the function table a=s4*2**4+s3*2**3+s2*2**2+s1*2**1+s0*2**0 whereby a is bit number and s result of sync mixer input selection"]
pub mod smx4fcttable;
#[doc = "SMX5SIGS (rw) register accessor: an alias for `Reg<SMX5SIGS_SPEC>`"]
pub type SMX5SIGS = crate::Reg<smx5sigs::SMX5SIGS_SPEC>;
#[doc = "Selection of input signals of sync mixer"]
pub mod smx5sigs;
#[doc = "SMX5FCTTABLE (rw) register accessor: an alias for `Reg<SMX5FCTTABLE_SPEC>`"]
pub type SMX5FCTTABLE = crate::Reg<smx5fcttable::SMX5FCTTABLE_SPEC>;
#[doc = "The sync mixer output is the result of the function table a=s4*2**4+s3*2**3+s2*2**2+s1*2**1+s0*2**0 whereby a is bit number and s result of sync mixer input selection"]
pub mod smx5fcttable;
#[doc = "SMX6SIGS (rw) register accessor: an alias for `Reg<SMX6SIGS_SPEC>`"]
pub type SMX6SIGS = crate::Reg<smx6sigs::SMX6SIGS_SPEC>;
#[doc = "Selection of input signals of sync mixer"]
pub mod smx6sigs;
#[doc = "SMX6FCTTABLE (rw) register accessor: an alias for `Reg<SMX6FCTTABLE_SPEC>`"]
pub type SMX6FCTTABLE = crate::Reg<smx6fcttable::SMX6FCTTABLE_SPEC>;
#[doc = "The sync mixer output is the result of the function table a=s4*2**4+s3*2**3+s2*2**2+s1*2**1+s0*2**0 whereby a is bit number and s result of sync mixer input selection"]
pub mod smx6fcttable;
#[doc = "SMX7SIGS (rw) register accessor: an alias for `Reg<SMX7SIGS_SPEC>`"]
pub type SMX7SIGS = crate::Reg<smx7sigs::SMX7SIGS_SPEC>;
#[doc = "Selection of input signals of sync mixer"]
pub mod smx7sigs;
#[doc = "SMX7FCTTABLE (rw) register accessor: an alias for `Reg<SMX7FCTTABLE_SPEC>`"]
pub type SMX7FCTTABLE = crate::Reg<smx7fcttable::SMX7FCTTABLE_SPEC>;
#[doc = "The sync mixer output is the result of the function table a=s4*2**4+s3*2**3+s2*2**2+s1*2**1+s0*2**0 whereby a is bit number and s result of sync mixer input selection"]
pub mod smx7fcttable;
#[doc = "SMX8SIGS (rw) register accessor: an alias for `Reg<SMX8SIGS_SPEC>`"]
pub type SMX8SIGS = crate::Reg<smx8sigs::SMX8SIGS_SPEC>;
#[doc = "Selection of input signals of sync mixer"]
pub mod smx8sigs;
#[doc = "SMX8FCTTABLE (rw) register accessor: an alias for `Reg<SMX8FCTTABLE_SPEC>`"]
pub type SMX8FCTTABLE = crate::Reg<smx8fcttable::SMX8FCTTABLE_SPEC>;
#[doc = "The sync mixer output is the result of the function table a=s4*2**4+s3*2**3+s2*2**2+s1*2**1+s0*2**0 whereby a is bit number and s result of sync mixer input selection"]
pub mod smx8fcttable;
#[doc = "SMX9SIGS (rw) register accessor: an alias for `Reg<SMX9SIGS_SPEC>`"]
pub type SMX9SIGS = crate::Reg<smx9sigs::SMX9SIGS_SPEC>;
#[doc = "Selection of input signals of sync mixer"]
pub mod smx9sigs;
#[doc = "SMX9FCTTABLE (rw) register accessor: an alias for `Reg<SMX9FCTTABLE_SPEC>`"]
pub type SMX9FCTTABLE = crate::Reg<smx9fcttable::SMX9FCTTABLE_SPEC>;
#[doc = "The sync mixer output is the result of the function table a=s4*2**4+s3*2**3+s2*2**2+s1*2**1+s0*2**0 whereby a is bit number and s result of sync mixer input selection"]
pub mod smx9fcttable;
#[doc = "SMX10SIGS (rw) register accessor: an alias for `Reg<SMX10SIGS_SPEC>`"]
pub type SMX10SIGS = crate::Reg<smx10sigs::SMX10SIGS_SPEC>;
#[doc = "Selection of input signals of sync mixer"]
pub mod smx10sigs;
#[doc = "SMX10FCTTABLE (rw) register accessor: an alias for `Reg<SMX10FCTTABLE_SPEC>`"]
pub type SMX10FCTTABLE = crate::Reg<smx10fcttable::SMX10FCTTABLE_SPEC>;
#[doc = "The sync mixer output is the result of the function table a=s4*2**4+s3*2**3+s2*2**2+s1*2**1+s0*2**0 whereby a is bit number and s result of sync mixer input selection"]
pub mod smx10fcttable;
#[doc = "SMX11SIGS (rw) register accessor: an alias for `Reg<SMX11SIGS_SPEC>`"]
pub type SMX11SIGS = crate::Reg<smx11sigs::SMX11SIGS_SPEC>;
#[doc = "Selection of input signals of sync mixer"]
pub mod smx11sigs;
#[doc = "SMX11FCTTABLE (rw) register accessor: an alias for `Reg<SMX11FCTTABLE_SPEC>`"]
pub type SMX11FCTTABLE = crate::Reg<smx11fcttable::SMX11FCTTABLE_SPEC>;
#[doc = "The sync mixer output is the result of the function table a=s4*2**4+s3*2**3+s2*2**2+s1*2**1+s0*2**0 whereby a is bit number and s result of sync mixer input selection"]
pub mod smx11fcttable;
