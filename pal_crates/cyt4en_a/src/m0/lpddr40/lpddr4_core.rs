#[doc = r"Register block"]
#[repr(C)]
pub struct LPDDR4_CORE {
    _reserved0: [u8; 0x0c],
    #[doc = "0x0c - User Command Interface"]
    pub uci: UCI,
    _reserved1: [u8; 0xf0],
    #[doc = "0x100 - Dynamo Control Register"]
    pub dmctl: DMCTL,
    #[doc = "0x104 - Dynamo Configuration Register"]
    pub dmcfg: DMCFG,
    #[doc = "0x108 - LPDDR Mode Register 1"]
    pub lpmr1: LPMR1,
    #[doc = "0x10c - LPDDR Mode Register 2"]
    pub lpmr2: LPMR2,
    #[doc = "0x110 - LPDDR Mode Register 3"]
    pub lpmr3: LPMR3,
    #[doc = "0x114 - LPDDR Mode Register 11"]
    pub lpmr11: LPMR11,
    #[doc = "0x118 - LPDDR Mode Register 12"]
    pub lpmr12: LPMR12,
    #[doc = "0x11c - LPDDR Mode Register 13"]
    pub lpmr13: LPMR13,
    #[doc = "0x120 - LPDDR Mode Register 14"]
    pub lpmr14: LPMR14,
    #[doc = "0x124 - LPDDR Mode Register 22"]
    pub lpmr22: LPMR22,
    #[doc = "0x128 - Route Configuration 0 - Route 0"]
    pub rtcfg0_rt0: RTCFG0_RT0,
    #[doc = "0x12c - Route Configuration 0 - Route 1"]
    pub rtcfg0_rt1: RTCFG0_RT1,
    #[doc = "0x130 - Route Configuration 0 - Route 2"]
    pub rtcfg0_rt2: RTCFG0_RT2,
    #[doc = "0x134 - Route Configuration 0 - Route 3"]
    pub rtcfg0_rt3: RTCFG0_RT3,
    #[doc = "0x138 - Route Configuration 1 - Route 0"]
    pub rtcfg1_rt0: RTCFG1_RT0,
    #[doc = "0x13c - Route Configuration 1 - Route 1"]
    pub rtcfg1_rt1: RTCFG1_RT1,
    #[doc = "0x140 - Route Configuration 1 - Route 2"]
    pub rtcfg1_rt2: RTCFG1_RT2,
    #[doc = "0x144 - Route Configuration 1 - Route 3"]
    pub rtcfg1_rt3: RTCFG1_RT3,
    #[doc = "0x148 - DRAM Address Register 0"]
    pub addr0: ADDR0,
    #[doc = "0x14c - DRAM Address Register 1"]
    pub addr1: ADDR1,
    #[doc = "0x150 - DRAM Address Register 2"]
    pub addr2: ADDR2,
    #[doc = "0x154 - DRAM Address Register 3"]
    pub addr3: ADDR3,
    #[doc = "0x158 - DRAM Address Register 4"]
    pub addr4: ADDR4,
    #[doc = "0x15c - DRAM Address Register 5"]
    pub addr5: ADDR5,
    #[doc = "0x160 - PHY Register"]
    pub phy: PHY,
    #[doc = "0x164 - PHY Operation Mode Register"]
    pub pom: POM,
    #[doc = "0x168 - DLL Control Register for PHY Command Module - Channel 0"]
    pub dllctlca_ch0: DLLCTLCA_CH0,
    #[doc = "0x16c - DLL Control Register for PHY Command Module - Channel 1"]
    pub dllctlca_ch1: DLLCTLCA_CH1,
    #[doc = "0x170 - DLL Control Register for PHY Data Module"]
    pub dllctldq_sl0: DLLCTLDQ_SL0,
    #[doc = "0x174 - DLL Control Register for PHY Data Module"]
    pub dllctldq_sl1: DLLCTLDQ_SL1,
    #[doc = "0x178 - DLL Control Register for PHY Data Module"]
    pub dllctldq_sl2: DLLCTLDQ_SL2,
    #[doc = "0x17c - DLL Control Register for PHY Data Module"]
    pub dllctldq_sl3: DLLCTLDQ_SL3,
    #[doc = "0x180 - PHY Read Training General Control Register 0"]
    pub rtgc0: RTGC0,
    #[doc = "0x184 - PHY Read Training General Control Register 1"]
    pub rtgc1: RTGC1,
    #[doc = "0x188 - PHY Sanity Check Address Register"]
    pub ptar: PTAR,
    #[doc = "0x18c - PHY VREF Training General Control Register"]
    pub vtgc: VTGC,
    #[doc = "0x190 - PHY BIST Control Register"]
    pub pbcr: PBCR,
    #[doc = "0x194 - PHY Command Module IO Control Register - Channel 0"]
    pub cior_ch0: CIOR_CH0,
    #[doc = "0x198 - PHY Command Module IO Control Register - Channel 1"]
    pub cior_ch1: CIOR_CH1,
    #[doc = "0x19c - PHY Data Module IO Control Register"]
    pub dior_sl0: DIOR_SL0,
    #[doc = "0x1a0 - PHY Data Module IO Control Register"]
    pub dior_sl1: DIOR_SL1,
    #[doc = "0x1a4 - PHY Data Module IO Control Register"]
    pub dior_sl2: DIOR_SL2,
    #[doc = "0x1a8 - PHY Data Module IO Control Register"]
    pub dior_sl3: DIOR_SL3,
    #[doc = "0x1ac - PHY Compensation Control Register - Channel 0"]
    pub pccr_ch0: PCCR_CH0,
    #[doc = "0x1b0 - PHY Compensation Control Register - Channel 1"]
    pub pccr_ch1: PCCR_CH1,
    #[doc = "0x1b4 - DQS2DQ Delay Control Register"]
    pub dqsdqcr: DQSDQCR,
    #[doc = "0x1b8 - PHY Training Setting Register 0"]
    pub ptsr0: PTSR0,
    #[doc = "0x1bc - PHY Training Setting Register 1"]
    pub ptsr1: PTSR1,
    #[doc = "0x1c0 - PHY Training Setting Register 2"]
    pub ptsr2: PTSR2,
    #[doc = "0x1c4 - PHY Training Setting Register 3"]
    pub ptsr3: PTSR3,
    #[doc = "0x1c8 - PHY Training Setting Register 4"]
    pub ptsr4: PTSR4,
    #[doc = "0x1cc - PHY Training Setting Register 5"]
    pub ptsr5: PTSR5,
    #[doc = "0x1d0 - PHY Training Setting Register 6"]
    pub ptsr6: PTSR6,
    #[doc = "0x1d4 - PHY Training Setting Register 7"]
    pub ptsr7: PTSR7,
    #[doc = "0x1d8 - PHY Training Setting Register 8"]
    pub ptsr8: PTSR8,
    #[doc = "0x1dc - PHY Training Setting Register 9"]
    pub ptsr9: PTSR9,
    #[doc = "0x1e0 - PHY Training Setting Register 10"]
    pub ptsr10: PTSR10,
    #[doc = "0x1e4 - PHY Training Setting Register 11"]
    pub ptsr11: PTSR11,
    #[doc = "0x1e8 - PHY Training Setting Register 12"]
    pub ptsr12: PTSR12,
    #[doc = "0x1ec - PHY Training Setting Register 13"]
    pub ptsr13: PTSR13,
    #[doc = "0x1f0 - PHY Training Setting Register 14"]
    pub ptsr14: PTSR14,
    #[doc = "0x1f4 - PHY Training Setting Register 15"]
    pub ptsr15: PTSR15,
    #[doc = "0x1f8 - PHY Training Setting Register 16"]
    pub ptsr16: PTSR16,
    #[doc = "0x1fc - PHY Training Setting Register 17"]
    pub ptsr17: PTSR17,
    #[doc = "0x200 - PHY Training Setting Register 18"]
    pub ptsr18: PTSR18,
    #[doc = "0x204 - PHY Training Setting Register 19"]
    pub ptsr19: PTSR19,
    #[doc = "0x208 - PHY Training Setting Register 20"]
    pub ptsr20: PTSR20,
    #[doc = "0x20c - PHY Training Setting Register 21"]
    pub ptsr21: PTSR21,
    #[doc = "0x210 - PHY Training Setting Register 22"]
    pub ptsr22: PTSR22,
    #[doc = "0x214 - PHY Training Setting Register 23"]
    pub ptsr23: PTSR23,
    #[doc = "0x218 - PHY Training Setting Register 24"]
    pub ptsr24: PTSR24,
    #[doc = "0x21c - PHY Training Setting Register 25"]
    pub ptsr25: PTSR25,
    #[doc = "0x220 - Timing Register 0"]
    pub treg0: TREG0,
    #[doc = "0x224 - Timing Register 1"]
    pub treg1: TREG1,
    #[doc = "0x228 - Timing Register 2"]
    pub treg2: TREG2,
    #[doc = "0x22c - Timing Register 3"]
    pub treg3: TREG3,
    #[doc = "0x230 - Timing Register 4"]
    pub treg4: TREG4,
    #[doc = "0x234 - Timing Register 5"]
    pub treg5: TREG5,
    #[doc = "0x238 - Timing Register 6"]
    pub treg6: TREG6,
    #[doc = "0x23c - Timing Register 7"]
    pub treg7: TREG7,
    #[doc = "0x240 - Timing Register 8"]
    pub treg8: TREG8,
    #[doc = "0x244 - Timing Register 9"]
    pub treg9: TREG9,
    #[doc = "0x248 - Timing Register 10"]
    pub treg10: TREG10,
    #[doc = "0x24c - Timing Register 11"]
    pub treg11: TREG11,
    #[doc = "0x250 - Timing Register 12"]
    pub treg12: TREG12,
    #[doc = "0x254 - Timing Register 13"]
    pub treg13: TREG13,
    #[doc = "0x258 - Timing Register 14"]
    pub treg14: TREG14,
    #[doc = "0x25c - Timing Register 15"]
    pub treg15: TREG15,
    #[doc = "0x260 - BIST Configuration Register - Channel 0"]
    pub bistcfg_ch0: BISTCFG_CH0,
    #[doc = "0x264 - BIST Configuration Register - Channel 1"]
    pub bistcfg_ch1: BISTCFG_CH1,
    #[doc = "0x268 - BIST Start Address - Channel 0"]
    pub biststaddr_ch0: BISTSTADDR_CH0,
    #[doc = "0x26c - BIST Start Address - Channel 1"]
    pub biststaddr_ch1: BISTSTADDR_CH1,
    #[doc = "0x270 - BIST End Address - Channel 0"]
    pub bistedaddr_ch0: BISTEDADDR_CH0,
    #[doc = "0x274 - BIST End Address - Channel 1"]
    pub bistedaddr_ch1: BISTEDADDR_CH1,
    #[doc = "0x278 - BIST March Element Register 0 - Channel 0"]
    pub bistm0_ch0: BISTM0_CH0,
    #[doc = "0x27c - BIST March Element Register 0 - Channel 1"]
    pub bistm0_ch1: BISTM0_CH1,
    #[doc = "0x280 - BIST March Element Register 1 - Channel 0"]
    pub bistm1_ch0: BISTM1_CH0,
    #[doc = "0x284 - BIST March Element Register 1 - Channel 1"]
    pub bistm1_ch1: BISTM1_CH1,
    #[doc = "0x288 - BIST March Element Register 2 - Channel 0"]
    pub bistm2_ch0: BISTM2_CH0,
    #[doc = "0x28c - BIST March Element Register 2 - Channel 1"]
    pub bistm2_ch1: BISTM2_CH1,
    #[doc = "0x290 - BIST March Element Register 3 - Channel 0"]
    pub bistm3_ch0: BISTM3_CH0,
    #[doc = "0x294 - BIST March Element Register 3 - Channel 1"]
    pub bistm3_ch1: BISTM3_CH1,
    #[doc = "0x298 - BIST March Element Register 4 - Channel 0"]
    pub bistm4_ch0: BISTM4_CH0,
    #[doc = "0x29c - BIST March Element Register 4 - Channel 1"]
    pub bistm4_ch1: BISTM4_CH1,
    #[doc = "0x2a0 - BIST March Element Register 5 - Channel 0"]
    pub bistm5_ch0: BISTM5_CH0,
    #[doc = "0x2a4 - BIST March Element Register 5 - Channel 1"]
    pub bistm5_ch1: BISTM5_CH1,
    #[doc = "0x2a8 - BIST March Element Register 6 - Channel 0"]
    pub bistm6_ch0: BISTM6_CH0,
    #[doc = "0x2ac - BIST March Element Register 6 - Channel 1"]
    pub bistm6_ch1: BISTM6_CH1,
    #[doc = "0x2b0 - BIST March Element Register 7 - Channel 0"]
    pub bistm7_ch0: BISTM7_CH0,
    #[doc = "0x2b4 - BIST March Element Register 7 - Channel 1"]
    pub bistm7_ch1: BISTM7_CH1,
    #[doc = "0x2b8 - BIST March Element Register 8 - Channel 0"]
    pub bistm8_ch0: BISTM8_CH0,
    #[doc = "0x2bc - BIST March Element Register 8 - Channel 1"]
    pub bistm8_ch1: BISTM8_CH1,
    #[doc = "0x2c0 - BIST March Element Register 9 - Channel 0"]
    pub bistm9_ch0: BISTM9_CH0,
    #[doc = "0x2c4 - BIST March Element Register 9 - Channel 1"]
    pub bistm9_ch1: BISTM9_CH1,
    #[doc = "0x2c8 - BIST March Element Register 10 - Channel 0"]
    pub bistm10_ch0: BISTM10_CH0,
    #[doc = "0x2cc - BIST March Element Register 10 - Channel 1"]
    pub bistm10_ch1: BISTM10_CH1,
    #[doc = "0x2d0 - BIST March Element Register 11 - Channel 0"]
    pub bistm11_ch0: BISTM11_CH0,
    #[doc = "0x2d4 - BIST March Element Register 11 - Channel 1"]
    pub bistm11_ch1: BISTM11_CH1,
    #[doc = "0x2d8 - BIST March Element Register 12 - Channel 0"]
    pub bistm12_ch0: BISTM12_CH0,
    #[doc = "0x2dc - BIST March Element Register 12 - Channel 1"]
    pub bistm12_ch1: BISTM12_CH1,
    #[doc = "0x2e0 - BIST March Element Register 13 - Channel 0"]
    pub bistm13_ch0: BISTM13_CH0,
    #[doc = "0x2e4 - BIST March Element Register 13 - Channel 1"]
    pub bistm13_ch1: BISTM13_CH1,
    #[doc = "0x2e8 - BIST March Element Register 14 - Channel 0"]
    pub bistm14_ch0: BISTM14_CH0,
    #[doc = "0x2ec - BIST March Element Register 14 - Channel 1"]
    pub bistm14_ch1: BISTM14_CH1,
    #[doc = "0x2f0 - BIST March Element Register 15 - Channel 0"]
    pub bistm15_ch0: BISTM15_CH0,
    #[doc = "0x2f4 - BIST March Element Register 15 - Channel 1"]
    pub bistm15_ch1: BISTM15_CH1,
    #[doc = "0x2f8 - ADFT Register"]
    pub adft: ADFT,
    #[doc = "0x2fc - Output Bypass Enable Register 0"]
    pub outbypen0: OUTBYPEN0,
    #[doc = "0x300 - Output Bypass Enable Register 1"]
    pub outbypen1: OUTBYPEN1,
    #[doc = "0x304 - Output Data In Register 0"]
    pub outd0: OUTD0,
    #[doc = "0x308 - Output Data In Register 1"]
    pub outd1: OUTD1,
    #[doc = "0x30c - Inline ECC Register 0"]
    pub inecc0: INECC0,
    #[doc = "0x310 - Inline ECC Register 1"]
    pub inecc1: INECC1,
    #[doc = "0x314 - Inline ECC Register 2"]
    pub inecc2: INECC2,
    _reserved135: [u8; 0x0ae8],
    #[doc = "0xe00 - Device ID Status Register"]
    pub dvstt0: DVSTT0,
    #[doc = "0xe04 - Device Mode Status Register"]
    pub dvstt1: DVSTT1,
    #[doc = "0xe08 - Operation Status Register 1 - Channel 0"]
    pub opstt_ch0: OPSTT_CH0,
    #[doc = "0xe0c - Operation Status Register 1 - Channel 1"]
    pub opstt_ch1: OPSTT_CH1,
    #[doc = "0xe10 - Interrupt Status Register - Channel 0"]
    pub intstt_ch0: INTSTT_CH0,
    #[doc = "0xe14 - Interrupt Status Register - Channel 1"]
    pub intstt_ch1: INTSTT_CH1,
    #[doc = "0xe18 - BIST Status 0"]
    pub biststt0: BISTSTT0,
    #[doc = "0xe1c - BIST Status 1"]
    pub biststt1: BISTSTT1,
    #[doc = "0xe20 - DDR BIST Status Register - Channel 0"]
    pub ddrbiststt_ch0: DDRBISTSTT_CH0,
    #[doc = "0xe24 - DDR BIST Status Register - Channel 1"]
    pub ddrbiststt_ch1: DDRBISTSTT_CH1,
    #[doc = "0xe28 - PHY Operation Status Register"]
    pub pos: POS,
    #[doc = "0xe2c - PHY Training Status Register 0"]
    pub pts0: PTS0,
    #[doc = "0xe30 - PHY Training Status Register 1"]
    pub pts1: PTS1,
    #[doc = "0xe34 - PHY Training Status Register 2"]
    pub pts2: PTS2,
    #[doc = "0xe38 - PHY Training Status Register 3"]
    pub pts3: PTS3,
    #[doc = "0xe3c - DLL Status Register for PHY Command Module"]
    pub dllsttca: DLLSTTCA,
    #[doc = "0xe40 - DLL Status Register for PHY Data Module 0"]
    pub dllsttdq0: DLLSTTDQ0,
    #[doc = "0xe44 - DLL Status Register for PHY Data Module 1"]
    pub dllsttdq1: DLLSTTDQ1,
    #[doc = "0xe48 - PHY BIST Status Register"]
    pub pbsr: PBSR,
    #[doc = "0xe4c - PHY Compensation Status Register - Channel 0"]
    pub pcsr_ch0: PCSR_CH0,
    #[doc = "0xe50 - PHY Compensation Status Register - Channel 1"]
    pub pcsr_ch1: PCSR_CH1,
    #[doc = "0xe54 - MR Readout Register - Channel 0"]
    pub mrr_ch0: MRR_CH0,
    #[doc = "0xe58 - MR Readout Register - Channel 1"]
    pub mrr_ch1: MRR_CH1,
    #[doc = "0xe5c - Shadow LPDDR Mode Register 1"]
    pub shad_lpmr1: SHAD_LPMR1,
    #[doc = "0xe60 - Shadow LPDDR Mode Register 2"]
    pub shad_lpmr2: SHAD_LPMR2,
    #[doc = "0xe64 - Shadow LPDDR Mode Register 3"]
    pub shad_lpmr3: SHAD_LPMR3,
    #[doc = "0xe68 - Shadow LPDDR Mode Register 11"]
    pub shad_lpmr11: SHAD_LPMR11,
    #[doc = "0xe6c - Shadow LPDDR Mode Register 12"]
    pub shad_lpmr12: SHAD_LPMR12,
    #[doc = "0xe70 - Shadow LPDDR Mode Register 13"]
    pub shad_lpmr13: SHAD_LPMR13,
    #[doc = "0xe74 - Shadow LPDDR Mode Register 14"]
    pub shad_lpmr14: SHAD_LPMR14,
    #[doc = "0xe78 - Shadow LPDDR Mode Register 22"]
    pub shad_lpmr22: SHAD_LPMR22,
    #[doc = "0xe7c - PHY Response Register"]
    pub resp: RESP,
    #[doc = "0xe80 - Read Data Register 0"]
    pub data0: DATA0,
    #[doc = "0xe84 - Read Data Register 1"]
    pub data1: DATA1,
    #[doc = "0xe88 - Read Data Register 2"]
    pub data2: DATA2,
    #[doc = "0xe8c - Read Data Register 3"]
    pub data3: DATA3,
    #[doc = "0xe90 - Read Data Register 4"]
    pub data4: DATA4,
    #[doc = "0xe94 - Read Data Register 5"]
    pub data5: DATA5,
    #[doc = "0xe98 - Read Data Register 6"]
    pub data6: DATA6,
    #[doc = "0xe9c - Read Data Register 7"]
    pub data7: DATA7,
    #[doc = "0xea0 - Read Data Register 8"]
    pub data8: DATA8,
    #[doc = "0xea4 - Inline ECC Status Register 0"]
    pub ineccstt0: INECCSTT0,
    #[doc = "0xea8 - Inline ECC Status Register 1"]
    pub ineccstt1: INECCSTT1,
    #[doc = "0xeac - Inline ECC Status Register 2"]
    pub ineccstt2: INECCSTT2,
    #[doc = "0xeb0 - Inline ECC Status Register 3"]
    pub ineccstt3: INECCSTT3,
    #[doc = "0xeb4 - Inline ECC Status Register 4"]
    pub ineccstt4: INECCSTT4,
}
#[doc = "UCI (rw) register accessor: an alias for `Reg<UCI_SPEC>`"]
pub type UCI = crate::Reg<uci::UCI_SPEC>;
#[doc = "User Command Interface"]
pub mod uci;
#[doc = "DMCTL (rw) register accessor: an alias for `Reg<DMCTL_SPEC>`"]
pub type DMCTL = crate::Reg<dmctl::DMCTL_SPEC>;
#[doc = "Dynamo Control Register"]
pub mod dmctl;
#[doc = "DMCFG (rw) register accessor: an alias for `Reg<DMCFG_SPEC>`"]
pub type DMCFG = crate::Reg<dmcfg::DMCFG_SPEC>;
#[doc = "Dynamo Configuration Register"]
pub mod dmcfg;
#[doc = "LPMR1 (rw) register accessor: an alias for `Reg<LPMR1_SPEC>`"]
pub type LPMR1 = crate::Reg<lpmr1::LPMR1_SPEC>;
#[doc = "LPDDR Mode Register 1"]
pub mod lpmr1;
#[doc = "LPMR2 (rw) register accessor: an alias for `Reg<LPMR2_SPEC>`"]
pub type LPMR2 = crate::Reg<lpmr2::LPMR2_SPEC>;
#[doc = "LPDDR Mode Register 2"]
pub mod lpmr2;
#[doc = "LPMR3 (rw) register accessor: an alias for `Reg<LPMR3_SPEC>`"]
pub type LPMR3 = crate::Reg<lpmr3::LPMR3_SPEC>;
#[doc = "LPDDR Mode Register 3"]
pub mod lpmr3;
#[doc = "LPMR11 (rw) register accessor: an alias for `Reg<LPMR11_SPEC>`"]
pub type LPMR11 = crate::Reg<lpmr11::LPMR11_SPEC>;
#[doc = "LPDDR Mode Register 11"]
pub mod lpmr11;
#[doc = "LPMR12 (rw) register accessor: an alias for `Reg<LPMR12_SPEC>`"]
pub type LPMR12 = crate::Reg<lpmr12::LPMR12_SPEC>;
#[doc = "LPDDR Mode Register 12"]
pub mod lpmr12;
#[doc = "LPMR13 (rw) register accessor: an alias for `Reg<LPMR13_SPEC>`"]
pub type LPMR13 = crate::Reg<lpmr13::LPMR13_SPEC>;
#[doc = "LPDDR Mode Register 13"]
pub mod lpmr13;
#[doc = "LPMR14 (rw) register accessor: an alias for `Reg<LPMR14_SPEC>`"]
pub type LPMR14 = crate::Reg<lpmr14::LPMR14_SPEC>;
#[doc = "LPDDR Mode Register 14"]
pub mod lpmr14;
#[doc = "LPMR22 (rw) register accessor: an alias for `Reg<LPMR22_SPEC>`"]
pub type LPMR22 = crate::Reg<lpmr22::LPMR22_SPEC>;
#[doc = "LPDDR Mode Register 22"]
pub mod lpmr22;
#[doc = "RTCFG0_RT0 (rw) register accessor: an alias for `Reg<RTCFG0_RT0_SPEC>`"]
pub type RTCFG0_RT0 = crate::Reg<rtcfg0_rt0::RTCFG0_RT0_SPEC>;
#[doc = "Route Configuration 0 - Route 0"]
pub mod rtcfg0_rt0;
#[doc = "RTCFG0_RT1 (rw) register accessor: an alias for `Reg<RTCFG0_RT1_SPEC>`"]
pub type RTCFG0_RT1 = crate::Reg<rtcfg0_rt1::RTCFG0_RT1_SPEC>;
#[doc = "Route Configuration 0 - Route 1"]
pub mod rtcfg0_rt1;
#[doc = "RTCFG0_RT2 (rw) register accessor: an alias for `Reg<RTCFG0_RT2_SPEC>`"]
pub type RTCFG0_RT2 = crate::Reg<rtcfg0_rt2::RTCFG0_RT2_SPEC>;
#[doc = "Route Configuration 0 - Route 2"]
pub mod rtcfg0_rt2;
#[doc = "RTCFG0_RT3 (rw) register accessor: an alias for `Reg<RTCFG0_RT3_SPEC>`"]
pub type RTCFG0_RT3 = crate::Reg<rtcfg0_rt3::RTCFG0_RT3_SPEC>;
#[doc = "Route Configuration 0 - Route 3"]
pub mod rtcfg0_rt3;
#[doc = "RTCFG1_RT0 (rw) register accessor: an alias for `Reg<RTCFG1_RT0_SPEC>`"]
pub type RTCFG1_RT0 = crate::Reg<rtcfg1_rt0::RTCFG1_RT0_SPEC>;
#[doc = "Route Configuration 1 - Route 0"]
pub mod rtcfg1_rt0;
#[doc = "RTCFG1_RT1 (rw) register accessor: an alias for `Reg<RTCFG1_RT1_SPEC>`"]
pub type RTCFG1_RT1 = crate::Reg<rtcfg1_rt1::RTCFG1_RT1_SPEC>;
#[doc = "Route Configuration 1 - Route 1"]
pub mod rtcfg1_rt1;
#[doc = "RTCFG1_RT2 (rw) register accessor: an alias for `Reg<RTCFG1_RT2_SPEC>`"]
pub type RTCFG1_RT2 = crate::Reg<rtcfg1_rt2::RTCFG1_RT2_SPEC>;
#[doc = "Route Configuration 1 - Route 2"]
pub mod rtcfg1_rt2;
#[doc = "RTCFG1_RT3 (rw) register accessor: an alias for `Reg<RTCFG1_RT3_SPEC>`"]
pub type RTCFG1_RT3 = crate::Reg<rtcfg1_rt3::RTCFG1_RT3_SPEC>;
#[doc = "Route Configuration 1 - Route 3"]
pub mod rtcfg1_rt3;
#[doc = "ADDR0 (rw) register accessor: an alias for `Reg<ADDR0_SPEC>`"]
pub type ADDR0 = crate::Reg<addr0::ADDR0_SPEC>;
#[doc = "DRAM Address Register 0"]
pub mod addr0;
#[doc = "ADDR1 (rw) register accessor: an alias for `Reg<ADDR1_SPEC>`"]
pub type ADDR1 = crate::Reg<addr1::ADDR1_SPEC>;
#[doc = "DRAM Address Register 1"]
pub mod addr1;
#[doc = "ADDR2 (rw) register accessor: an alias for `Reg<ADDR2_SPEC>`"]
pub type ADDR2 = crate::Reg<addr2::ADDR2_SPEC>;
#[doc = "DRAM Address Register 2"]
pub mod addr2;
#[doc = "ADDR3 (rw) register accessor: an alias for `Reg<ADDR3_SPEC>`"]
pub type ADDR3 = crate::Reg<addr3::ADDR3_SPEC>;
#[doc = "DRAM Address Register 3"]
pub mod addr3;
#[doc = "ADDR4 (rw) register accessor: an alias for `Reg<ADDR4_SPEC>`"]
pub type ADDR4 = crate::Reg<addr4::ADDR4_SPEC>;
#[doc = "DRAM Address Register 4"]
pub mod addr4;
#[doc = "ADDR5 (rw) register accessor: an alias for `Reg<ADDR5_SPEC>`"]
pub type ADDR5 = crate::Reg<addr5::ADDR5_SPEC>;
#[doc = "DRAM Address Register 5"]
pub mod addr5;
#[doc = "PHY (rw) register accessor: an alias for `Reg<PHY_SPEC>`"]
pub type PHY = crate::Reg<phy::PHY_SPEC>;
#[doc = "PHY Register"]
pub mod phy;
#[doc = "POM (rw) register accessor: an alias for `Reg<POM_SPEC>`"]
pub type POM = crate::Reg<pom::POM_SPEC>;
#[doc = "PHY Operation Mode Register"]
pub mod pom;
#[doc = "DLLCTLCA_CH0 (rw) register accessor: an alias for `Reg<DLLCTLCA_CH0_SPEC>`"]
pub type DLLCTLCA_CH0 = crate::Reg<dllctlca_ch0::DLLCTLCA_CH0_SPEC>;
#[doc = "DLL Control Register for PHY Command Module - Channel 0"]
pub mod dllctlca_ch0;
#[doc = "DLLCTLCA_CH1 (rw) register accessor: an alias for `Reg<DLLCTLCA_CH1_SPEC>`"]
pub type DLLCTLCA_CH1 = crate::Reg<dllctlca_ch1::DLLCTLCA_CH1_SPEC>;
#[doc = "DLL Control Register for PHY Command Module - Channel 1"]
pub mod dllctlca_ch1;
#[doc = "DLLCTLDQ_SL0 (rw) register accessor: an alias for `Reg<DLLCTLDQ_SL0_SPEC>`"]
pub type DLLCTLDQ_SL0 = crate::Reg<dllctldq_sl0::DLLCTLDQ_SL0_SPEC>;
#[doc = "DLL Control Register for PHY Data Module"]
pub mod dllctldq_sl0;
#[doc = "DLLCTLDQ_SL1 (rw) register accessor: an alias for `Reg<DLLCTLDQ_SL1_SPEC>`"]
pub type DLLCTLDQ_SL1 = crate::Reg<dllctldq_sl1::DLLCTLDQ_SL1_SPEC>;
#[doc = "DLL Control Register for PHY Data Module"]
pub mod dllctldq_sl1;
#[doc = "DLLCTLDQ_SL2 (rw) register accessor: an alias for `Reg<DLLCTLDQ_SL2_SPEC>`"]
pub type DLLCTLDQ_SL2 = crate::Reg<dllctldq_sl2::DLLCTLDQ_SL2_SPEC>;
#[doc = "DLL Control Register for PHY Data Module"]
pub mod dllctldq_sl2;
#[doc = "DLLCTLDQ_SL3 (rw) register accessor: an alias for `Reg<DLLCTLDQ_SL3_SPEC>`"]
pub type DLLCTLDQ_SL3 = crate::Reg<dllctldq_sl3::DLLCTLDQ_SL3_SPEC>;
#[doc = "DLL Control Register for PHY Data Module"]
pub mod dllctldq_sl3;
#[doc = "RTGC0 (rw) register accessor: an alias for `Reg<RTGC0_SPEC>`"]
pub type RTGC0 = crate::Reg<rtgc0::RTGC0_SPEC>;
#[doc = "PHY Read Training General Control Register 0"]
pub mod rtgc0;
#[doc = "RTGC1 (rw) register accessor: an alias for `Reg<RTGC1_SPEC>`"]
pub type RTGC1 = crate::Reg<rtgc1::RTGC1_SPEC>;
#[doc = "PHY Read Training General Control Register 1"]
pub mod rtgc1;
#[doc = "PTAR (rw) register accessor: an alias for `Reg<PTAR_SPEC>`"]
pub type PTAR = crate::Reg<ptar::PTAR_SPEC>;
#[doc = "PHY Sanity Check Address Register"]
pub mod ptar;
#[doc = "VTGC (rw) register accessor: an alias for `Reg<VTGC_SPEC>`"]
pub type VTGC = crate::Reg<vtgc::VTGC_SPEC>;
#[doc = "PHY VREF Training General Control Register"]
pub mod vtgc;
#[doc = "PBCR (rw) register accessor: an alias for `Reg<PBCR_SPEC>`"]
pub type PBCR = crate::Reg<pbcr::PBCR_SPEC>;
#[doc = "PHY BIST Control Register"]
pub mod pbcr;
#[doc = "CIOR_CH0 (rw) register accessor: an alias for `Reg<CIOR_CH0_SPEC>`"]
pub type CIOR_CH0 = crate::Reg<cior_ch0::CIOR_CH0_SPEC>;
#[doc = "PHY Command Module IO Control Register - Channel 0"]
pub mod cior_ch0;
#[doc = "CIOR_CH1 (rw) register accessor: an alias for `Reg<CIOR_CH1_SPEC>`"]
pub type CIOR_CH1 = crate::Reg<cior_ch1::CIOR_CH1_SPEC>;
#[doc = "PHY Command Module IO Control Register - Channel 1"]
pub mod cior_ch1;
#[doc = "DIOR_SL0 (rw) register accessor: an alias for `Reg<DIOR_SL0_SPEC>`"]
pub type DIOR_SL0 = crate::Reg<dior_sl0::DIOR_SL0_SPEC>;
#[doc = "PHY Data Module IO Control Register"]
pub mod dior_sl0;
#[doc = "DIOR_SL1 (rw) register accessor: an alias for `Reg<DIOR_SL1_SPEC>`"]
pub type DIOR_SL1 = crate::Reg<dior_sl1::DIOR_SL1_SPEC>;
#[doc = "PHY Data Module IO Control Register"]
pub mod dior_sl1;
#[doc = "DIOR_SL2 (rw) register accessor: an alias for `Reg<DIOR_SL2_SPEC>`"]
pub type DIOR_SL2 = crate::Reg<dior_sl2::DIOR_SL2_SPEC>;
#[doc = "PHY Data Module IO Control Register"]
pub mod dior_sl2;
#[doc = "DIOR_SL3 (rw) register accessor: an alias for `Reg<DIOR_SL3_SPEC>`"]
pub type DIOR_SL3 = crate::Reg<dior_sl3::DIOR_SL3_SPEC>;
#[doc = "PHY Data Module IO Control Register"]
pub mod dior_sl3;
#[doc = "PCCR_CH0 (rw) register accessor: an alias for `Reg<PCCR_CH0_SPEC>`"]
pub type PCCR_CH0 = crate::Reg<pccr_ch0::PCCR_CH0_SPEC>;
#[doc = "PHY Compensation Control Register - Channel 0"]
pub mod pccr_ch0;
#[doc = "PCCR_CH1 (rw) register accessor: an alias for `Reg<PCCR_CH1_SPEC>`"]
pub type PCCR_CH1 = crate::Reg<pccr_ch1::PCCR_CH1_SPEC>;
#[doc = "PHY Compensation Control Register - Channel 1"]
pub mod pccr_ch1;
#[doc = "DQSDQCR (rw) register accessor: an alias for `Reg<DQSDQCR_SPEC>`"]
pub type DQSDQCR = crate::Reg<dqsdqcr::DQSDQCR_SPEC>;
#[doc = "DQS2DQ Delay Control Register"]
pub mod dqsdqcr;
#[doc = "PTSR0 (rw) register accessor: an alias for `Reg<PTSR0_SPEC>`"]
pub type PTSR0 = crate::Reg<ptsr0::PTSR0_SPEC>;
#[doc = "PHY Training Setting Register 0"]
pub mod ptsr0;
#[doc = "PTSR1 (rw) register accessor: an alias for `Reg<PTSR1_SPEC>`"]
pub type PTSR1 = crate::Reg<ptsr1::PTSR1_SPEC>;
#[doc = "PHY Training Setting Register 1"]
pub mod ptsr1;
#[doc = "PTSR2 (rw) register accessor: an alias for `Reg<PTSR2_SPEC>`"]
pub type PTSR2 = crate::Reg<ptsr2::PTSR2_SPEC>;
#[doc = "PHY Training Setting Register 2"]
pub mod ptsr2;
#[doc = "PTSR3 (rw) register accessor: an alias for `Reg<PTSR3_SPEC>`"]
pub type PTSR3 = crate::Reg<ptsr3::PTSR3_SPEC>;
#[doc = "PHY Training Setting Register 3"]
pub mod ptsr3;
#[doc = "PTSR4 (rw) register accessor: an alias for `Reg<PTSR4_SPEC>`"]
pub type PTSR4 = crate::Reg<ptsr4::PTSR4_SPEC>;
#[doc = "PHY Training Setting Register 4"]
pub mod ptsr4;
#[doc = "PTSR5 (rw) register accessor: an alias for `Reg<PTSR5_SPEC>`"]
pub type PTSR5 = crate::Reg<ptsr5::PTSR5_SPEC>;
#[doc = "PHY Training Setting Register 5"]
pub mod ptsr5;
#[doc = "PTSR6 (rw) register accessor: an alias for `Reg<PTSR6_SPEC>`"]
pub type PTSR6 = crate::Reg<ptsr6::PTSR6_SPEC>;
#[doc = "PHY Training Setting Register 6"]
pub mod ptsr6;
#[doc = "PTSR7 (rw) register accessor: an alias for `Reg<PTSR7_SPEC>`"]
pub type PTSR7 = crate::Reg<ptsr7::PTSR7_SPEC>;
#[doc = "PHY Training Setting Register 7"]
pub mod ptsr7;
#[doc = "PTSR8 (rw) register accessor: an alias for `Reg<PTSR8_SPEC>`"]
pub type PTSR8 = crate::Reg<ptsr8::PTSR8_SPEC>;
#[doc = "PHY Training Setting Register 8"]
pub mod ptsr8;
#[doc = "PTSR9 (rw) register accessor: an alias for `Reg<PTSR9_SPEC>`"]
pub type PTSR9 = crate::Reg<ptsr9::PTSR9_SPEC>;
#[doc = "PHY Training Setting Register 9"]
pub mod ptsr9;
#[doc = "PTSR10 (rw) register accessor: an alias for `Reg<PTSR10_SPEC>`"]
pub type PTSR10 = crate::Reg<ptsr10::PTSR10_SPEC>;
#[doc = "PHY Training Setting Register 10"]
pub mod ptsr10;
#[doc = "PTSR11 (rw) register accessor: an alias for `Reg<PTSR11_SPEC>`"]
pub type PTSR11 = crate::Reg<ptsr11::PTSR11_SPEC>;
#[doc = "PHY Training Setting Register 11"]
pub mod ptsr11;
#[doc = "PTSR12 (rw) register accessor: an alias for `Reg<PTSR12_SPEC>`"]
pub type PTSR12 = crate::Reg<ptsr12::PTSR12_SPEC>;
#[doc = "PHY Training Setting Register 12"]
pub mod ptsr12;
#[doc = "PTSR13 (rw) register accessor: an alias for `Reg<PTSR13_SPEC>`"]
pub type PTSR13 = crate::Reg<ptsr13::PTSR13_SPEC>;
#[doc = "PHY Training Setting Register 13"]
pub mod ptsr13;
#[doc = "PTSR14 (rw) register accessor: an alias for `Reg<PTSR14_SPEC>`"]
pub type PTSR14 = crate::Reg<ptsr14::PTSR14_SPEC>;
#[doc = "PHY Training Setting Register 14"]
pub mod ptsr14;
#[doc = "PTSR15 (rw) register accessor: an alias for `Reg<PTSR15_SPEC>`"]
pub type PTSR15 = crate::Reg<ptsr15::PTSR15_SPEC>;
#[doc = "PHY Training Setting Register 15"]
pub mod ptsr15;
#[doc = "PTSR16 (rw) register accessor: an alias for `Reg<PTSR16_SPEC>`"]
pub type PTSR16 = crate::Reg<ptsr16::PTSR16_SPEC>;
#[doc = "PHY Training Setting Register 16"]
pub mod ptsr16;
#[doc = "PTSR17 (rw) register accessor: an alias for `Reg<PTSR17_SPEC>`"]
pub type PTSR17 = crate::Reg<ptsr17::PTSR17_SPEC>;
#[doc = "PHY Training Setting Register 17"]
pub mod ptsr17;
#[doc = "PTSR18 (rw) register accessor: an alias for `Reg<PTSR18_SPEC>`"]
pub type PTSR18 = crate::Reg<ptsr18::PTSR18_SPEC>;
#[doc = "PHY Training Setting Register 18"]
pub mod ptsr18;
#[doc = "PTSR19 (rw) register accessor: an alias for `Reg<PTSR19_SPEC>`"]
pub type PTSR19 = crate::Reg<ptsr19::PTSR19_SPEC>;
#[doc = "PHY Training Setting Register 19"]
pub mod ptsr19;
#[doc = "PTSR20 (rw) register accessor: an alias for `Reg<PTSR20_SPEC>`"]
pub type PTSR20 = crate::Reg<ptsr20::PTSR20_SPEC>;
#[doc = "PHY Training Setting Register 20"]
pub mod ptsr20;
#[doc = "PTSR21 (rw) register accessor: an alias for `Reg<PTSR21_SPEC>`"]
pub type PTSR21 = crate::Reg<ptsr21::PTSR21_SPEC>;
#[doc = "PHY Training Setting Register 21"]
pub mod ptsr21;
#[doc = "PTSR22 (rw) register accessor: an alias for `Reg<PTSR22_SPEC>`"]
pub type PTSR22 = crate::Reg<ptsr22::PTSR22_SPEC>;
#[doc = "PHY Training Setting Register 22"]
pub mod ptsr22;
#[doc = "PTSR23 (rw) register accessor: an alias for `Reg<PTSR23_SPEC>`"]
pub type PTSR23 = crate::Reg<ptsr23::PTSR23_SPEC>;
#[doc = "PHY Training Setting Register 23"]
pub mod ptsr23;
#[doc = "PTSR24 (rw) register accessor: an alias for `Reg<PTSR24_SPEC>`"]
pub type PTSR24 = crate::Reg<ptsr24::PTSR24_SPEC>;
#[doc = "PHY Training Setting Register 24"]
pub mod ptsr24;
#[doc = "PTSR25 (rw) register accessor: an alias for `Reg<PTSR25_SPEC>`"]
pub type PTSR25 = crate::Reg<ptsr25::PTSR25_SPEC>;
#[doc = "PHY Training Setting Register 25"]
pub mod ptsr25;
#[doc = "TREG0 (rw) register accessor: an alias for `Reg<TREG0_SPEC>`"]
pub type TREG0 = crate::Reg<treg0::TREG0_SPEC>;
#[doc = "Timing Register 0"]
pub mod treg0;
#[doc = "TREG1 (rw) register accessor: an alias for `Reg<TREG1_SPEC>`"]
pub type TREG1 = crate::Reg<treg1::TREG1_SPEC>;
#[doc = "Timing Register 1"]
pub mod treg1;
#[doc = "TREG2 (rw) register accessor: an alias for `Reg<TREG2_SPEC>`"]
pub type TREG2 = crate::Reg<treg2::TREG2_SPEC>;
#[doc = "Timing Register 2"]
pub mod treg2;
#[doc = "TREG3 (rw) register accessor: an alias for `Reg<TREG3_SPEC>`"]
pub type TREG3 = crate::Reg<treg3::TREG3_SPEC>;
#[doc = "Timing Register 3"]
pub mod treg3;
#[doc = "TREG4 (rw) register accessor: an alias for `Reg<TREG4_SPEC>`"]
pub type TREG4 = crate::Reg<treg4::TREG4_SPEC>;
#[doc = "Timing Register 4"]
pub mod treg4;
#[doc = "TREG5 (rw) register accessor: an alias for `Reg<TREG5_SPEC>`"]
pub type TREG5 = crate::Reg<treg5::TREG5_SPEC>;
#[doc = "Timing Register 5"]
pub mod treg5;
#[doc = "TREG6 (rw) register accessor: an alias for `Reg<TREG6_SPEC>`"]
pub type TREG6 = crate::Reg<treg6::TREG6_SPEC>;
#[doc = "Timing Register 6"]
pub mod treg6;
#[doc = "TREG7 (rw) register accessor: an alias for `Reg<TREG7_SPEC>`"]
pub type TREG7 = crate::Reg<treg7::TREG7_SPEC>;
#[doc = "Timing Register 7"]
pub mod treg7;
#[doc = "TREG8 (rw) register accessor: an alias for `Reg<TREG8_SPEC>`"]
pub type TREG8 = crate::Reg<treg8::TREG8_SPEC>;
#[doc = "Timing Register 8"]
pub mod treg8;
#[doc = "TREG9 (rw) register accessor: an alias for `Reg<TREG9_SPEC>`"]
pub type TREG9 = crate::Reg<treg9::TREG9_SPEC>;
#[doc = "Timing Register 9"]
pub mod treg9;
#[doc = "TREG10 (rw) register accessor: an alias for `Reg<TREG10_SPEC>`"]
pub type TREG10 = crate::Reg<treg10::TREG10_SPEC>;
#[doc = "Timing Register 10"]
pub mod treg10;
#[doc = "TREG11 (rw) register accessor: an alias for `Reg<TREG11_SPEC>`"]
pub type TREG11 = crate::Reg<treg11::TREG11_SPEC>;
#[doc = "Timing Register 11"]
pub mod treg11;
#[doc = "TREG12 (rw) register accessor: an alias for `Reg<TREG12_SPEC>`"]
pub type TREG12 = crate::Reg<treg12::TREG12_SPEC>;
#[doc = "Timing Register 12"]
pub mod treg12;
#[doc = "TREG13 (rw) register accessor: an alias for `Reg<TREG13_SPEC>`"]
pub type TREG13 = crate::Reg<treg13::TREG13_SPEC>;
#[doc = "Timing Register 13"]
pub mod treg13;
#[doc = "TREG14 (rw) register accessor: an alias for `Reg<TREG14_SPEC>`"]
pub type TREG14 = crate::Reg<treg14::TREG14_SPEC>;
#[doc = "Timing Register 14"]
pub mod treg14;
#[doc = "TREG15 (rw) register accessor: an alias for `Reg<TREG15_SPEC>`"]
pub type TREG15 = crate::Reg<treg15::TREG15_SPEC>;
#[doc = "Timing Register 15"]
pub mod treg15;
#[doc = "BISTCFG_CH0 (rw) register accessor: an alias for `Reg<BISTCFG_CH0_SPEC>`"]
pub type BISTCFG_CH0 = crate::Reg<bistcfg_ch0::BISTCFG_CH0_SPEC>;
#[doc = "BIST Configuration Register - Channel 0"]
pub mod bistcfg_ch0;
#[doc = "BISTCFG_CH1 (rw) register accessor: an alias for `Reg<BISTCFG_CH1_SPEC>`"]
pub type BISTCFG_CH1 = crate::Reg<bistcfg_ch1::BISTCFG_CH1_SPEC>;
#[doc = "BIST Configuration Register - Channel 1"]
pub mod bistcfg_ch1;
#[doc = "BISTSTADDR_CH0 (rw) register accessor: an alias for `Reg<BISTSTADDR_CH0_SPEC>`"]
pub type BISTSTADDR_CH0 = crate::Reg<biststaddr_ch0::BISTSTADDR_CH0_SPEC>;
#[doc = "BIST Start Address - Channel 0"]
pub mod biststaddr_ch0;
#[doc = "BISTSTADDR_CH1 (rw) register accessor: an alias for `Reg<BISTSTADDR_CH1_SPEC>`"]
pub type BISTSTADDR_CH1 = crate::Reg<biststaddr_ch1::BISTSTADDR_CH1_SPEC>;
#[doc = "BIST Start Address - Channel 1"]
pub mod biststaddr_ch1;
#[doc = "BISTEDADDR_CH0 (rw) register accessor: an alias for `Reg<BISTEDADDR_CH0_SPEC>`"]
pub type BISTEDADDR_CH0 = crate::Reg<bistedaddr_ch0::BISTEDADDR_CH0_SPEC>;
#[doc = "BIST End Address - Channel 0"]
pub mod bistedaddr_ch0;
#[doc = "BISTEDADDR_CH1 (rw) register accessor: an alias for `Reg<BISTEDADDR_CH1_SPEC>`"]
pub type BISTEDADDR_CH1 = crate::Reg<bistedaddr_ch1::BISTEDADDR_CH1_SPEC>;
#[doc = "BIST End Address - Channel 1"]
pub mod bistedaddr_ch1;
#[doc = "BISTM0_CH0 (rw) register accessor: an alias for `Reg<BISTM0_CH0_SPEC>`"]
pub type BISTM0_CH0 = crate::Reg<bistm0_ch0::BISTM0_CH0_SPEC>;
#[doc = "BIST March Element Register 0 - Channel 0"]
pub mod bistm0_ch0;
#[doc = "BISTM0_CH1 (rw) register accessor: an alias for `Reg<BISTM0_CH1_SPEC>`"]
pub type BISTM0_CH1 = crate::Reg<bistm0_ch1::BISTM0_CH1_SPEC>;
#[doc = "BIST March Element Register 0 - Channel 1"]
pub mod bistm0_ch1;
#[doc = "BISTM1_CH0 (rw) register accessor: an alias for `Reg<BISTM1_CH0_SPEC>`"]
pub type BISTM1_CH0 = crate::Reg<bistm1_ch0::BISTM1_CH0_SPEC>;
#[doc = "BIST March Element Register 1 - Channel 0"]
pub mod bistm1_ch0;
#[doc = "BISTM1_CH1 (rw) register accessor: an alias for `Reg<BISTM1_CH1_SPEC>`"]
pub type BISTM1_CH1 = crate::Reg<bistm1_ch1::BISTM1_CH1_SPEC>;
#[doc = "BIST March Element Register 1 - Channel 1"]
pub mod bistm1_ch1;
#[doc = "BISTM2_CH0 (rw) register accessor: an alias for `Reg<BISTM2_CH0_SPEC>`"]
pub type BISTM2_CH0 = crate::Reg<bistm2_ch0::BISTM2_CH0_SPEC>;
#[doc = "BIST March Element Register 2 - Channel 0"]
pub mod bistm2_ch0;
#[doc = "BISTM2_CH1 (rw) register accessor: an alias for `Reg<BISTM2_CH1_SPEC>`"]
pub type BISTM2_CH1 = crate::Reg<bistm2_ch1::BISTM2_CH1_SPEC>;
#[doc = "BIST March Element Register 2 - Channel 1"]
pub mod bistm2_ch1;
#[doc = "BISTM3_CH0 (rw) register accessor: an alias for `Reg<BISTM3_CH0_SPEC>`"]
pub type BISTM3_CH0 = crate::Reg<bistm3_ch0::BISTM3_CH0_SPEC>;
#[doc = "BIST March Element Register 3 - Channel 0"]
pub mod bistm3_ch0;
#[doc = "BISTM3_CH1 (rw) register accessor: an alias for `Reg<BISTM3_CH1_SPEC>`"]
pub type BISTM3_CH1 = crate::Reg<bistm3_ch1::BISTM3_CH1_SPEC>;
#[doc = "BIST March Element Register 3 - Channel 1"]
pub mod bistm3_ch1;
#[doc = "BISTM4_CH0 (rw) register accessor: an alias for `Reg<BISTM4_CH0_SPEC>`"]
pub type BISTM4_CH0 = crate::Reg<bistm4_ch0::BISTM4_CH0_SPEC>;
#[doc = "BIST March Element Register 4 - Channel 0"]
pub mod bistm4_ch0;
#[doc = "BISTM4_CH1 (rw) register accessor: an alias for `Reg<BISTM4_CH1_SPEC>`"]
pub type BISTM4_CH1 = crate::Reg<bistm4_ch1::BISTM4_CH1_SPEC>;
#[doc = "BIST March Element Register 4 - Channel 1"]
pub mod bistm4_ch1;
#[doc = "BISTM5_CH0 (rw) register accessor: an alias for `Reg<BISTM5_CH0_SPEC>`"]
pub type BISTM5_CH0 = crate::Reg<bistm5_ch0::BISTM5_CH0_SPEC>;
#[doc = "BIST March Element Register 5 - Channel 0"]
pub mod bistm5_ch0;
#[doc = "BISTM5_CH1 (rw) register accessor: an alias for `Reg<BISTM5_CH1_SPEC>`"]
pub type BISTM5_CH1 = crate::Reg<bistm5_ch1::BISTM5_CH1_SPEC>;
#[doc = "BIST March Element Register 5 - Channel 1"]
pub mod bistm5_ch1;
#[doc = "BISTM6_CH0 (rw) register accessor: an alias for `Reg<BISTM6_CH0_SPEC>`"]
pub type BISTM6_CH0 = crate::Reg<bistm6_ch0::BISTM6_CH0_SPEC>;
#[doc = "BIST March Element Register 6 - Channel 0"]
pub mod bistm6_ch0;
#[doc = "BISTM6_CH1 (rw) register accessor: an alias for `Reg<BISTM6_CH1_SPEC>`"]
pub type BISTM6_CH1 = crate::Reg<bistm6_ch1::BISTM6_CH1_SPEC>;
#[doc = "BIST March Element Register 6 - Channel 1"]
pub mod bistm6_ch1;
#[doc = "BISTM7_CH0 (rw) register accessor: an alias for `Reg<BISTM7_CH0_SPEC>`"]
pub type BISTM7_CH0 = crate::Reg<bistm7_ch0::BISTM7_CH0_SPEC>;
#[doc = "BIST March Element Register 7 - Channel 0"]
pub mod bistm7_ch0;
#[doc = "BISTM7_CH1 (rw) register accessor: an alias for `Reg<BISTM7_CH1_SPEC>`"]
pub type BISTM7_CH1 = crate::Reg<bistm7_ch1::BISTM7_CH1_SPEC>;
#[doc = "BIST March Element Register 7 - Channel 1"]
pub mod bistm7_ch1;
#[doc = "BISTM8_CH0 (rw) register accessor: an alias for `Reg<BISTM8_CH0_SPEC>`"]
pub type BISTM8_CH0 = crate::Reg<bistm8_ch0::BISTM8_CH0_SPEC>;
#[doc = "BIST March Element Register 8 - Channel 0"]
pub mod bistm8_ch0;
#[doc = "BISTM8_CH1 (rw) register accessor: an alias for `Reg<BISTM8_CH1_SPEC>`"]
pub type BISTM8_CH1 = crate::Reg<bistm8_ch1::BISTM8_CH1_SPEC>;
#[doc = "BIST March Element Register 8 - Channel 1"]
pub mod bistm8_ch1;
#[doc = "BISTM9_CH0 (rw) register accessor: an alias for `Reg<BISTM9_CH0_SPEC>`"]
pub type BISTM9_CH0 = crate::Reg<bistm9_ch0::BISTM9_CH0_SPEC>;
#[doc = "BIST March Element Register 9 - Channel 0"]
pub mod bistm9_ch0;
#[doc = "BISTM9_CH1 (rw) register accessor: an alias for `Reg<BISTM9_CH1_SPEC>`"]
pub type BISTM9_CH1 = crate::Reg<bistm9_ch1::BISTM9_CH1_SPEC>;
#[doc = "BIST March Element Register 9 - Channel 1"]
pub mod bistm9_ch1;
#[doc = "BISTM10_CH0 (rw) register accessor: an alias for `Reg<BISTM10_CH0_SPEC>`"]
pub type BISTM10_CH0 = crate::Reg<bistm10_ch0::BISTM10_CH0_SPEC>;
#[doc = "BIST March Element Register 10 - Channel 0"]
pub mod bistm10_ch0;
#[doc = "BISTM10_CH1 (rw) register accessor: an alias for `Reg<BISTM10_CH1_SPEC>`"]
pub type BISTM10_CH1 = crate::Reg<bistm10_ch1::BISTM10_CH1_SPEC>;
#[doc = "BIST March Element Register 10 - Channel 1"]
pub mod bistm10_ch1;
#[doc = "BISTM11_CH0 (rw) register accessor: an alias for `Reg<BISTM11_CH0_SPEC>`"]
pub type BISTM11_CH0 = crate::Reg<bistm11_ch0::BISTM11_CH0_SPEC>;
#[doc = "BIST March Element Register 11 - Channel 0"]
pub mod bistm11_ch0;
#[doc = "BISTM11_CH1 (rw) register accessor: an alias for `Reg<BISTM11_CH1_SPEC>`"]
pub type BISTM11_CH1 = crate::Reg<bistm11_ch1::BISTM11_CH1_SPEC>;
#[doc = "BIST March Element Register 11 - Channel 1"]
pub mod bistm11_ch1;
#[doc = "BISTM12_CH0 (rw) register accessor: an alias for `Reg<BISTM12_CH0_SPEC>`"]
pub type BISTM12_CH0 = crate::Reg<bistm12_ch0::BISTM12_CH0_SPEC>;
#[doc = "BIST March Element Register 12 - Channel 0"]
pub mod bistm12_ch0;
#[doc = "BISTM12_CH1 (rw) register accessor: an alias for `Reg<BISTM12_CH1_SPEC>`"]
pub type BISTM12_CH1 = crate::Reg<bistm12_ch1::BISTM12_CH1_SPEC>;
#[doc = "BIST March Element Register 12 - Channel 1"]
pub mod bistm12_ch1;
#[doc = "BISTM13_CH0 (rw) register accessor: an alias for `Reg<BISTM13_CH0_SPEC>`"]
pub type BISTM13_CH0 = crate::Reg<bistm13_ch0::BISTM13_CH0_SPEC>;
#[doc = "BIST March Element Register 13 - Channel 0"]
pub mod bistm13_ch0;
#[doc = "BISTM13_CH1 (rw) register accessor: an alias for `Reg<BISTM13_CH1_SPEC>`"]
pub type BISTM13_CH1 = crate::Reg<bistm13_ch1::BISTM13_CH1_SPEC>;
#[doc = "BIST March Element Register 13 - Channel 1"]
pub mod bistm13_ch1;
#[doc = "BISTM14_CH0 (rw) register accessor: an alias for `Reg<BISTM14_CH0_SPEC>`"]
pub type BISTM14_CH0 = crate::Reg<bistm14_ch0::BISTM14_CH0_SPEC>;
#[doc = "BIST March Element Register 14 - Channel 0"]
pub mod bistm14_ch0;
#[doc = "BISTM14_CH1 (rw) register accessor: an alias for `Reg<BISTM14_CH1_SPEC>`"]
pub type BISTM14_CH1 = crate::Reg<bistm14_ch1::BISTM14_CH1_SPEC>;
#[doc = "BIST March Element Register 14 - Channel 1"]
pub mod bistm14_ch1;
#[doc = "BISTM15_CH0 (rw) register accessor: an alias for `Reg<BISTM15_CH0_SPEC>`"]
pub type BISTM15_CH0 = crate::Reg<bistm15_ch0::BISTM15_CH0_SPEC>;
#[doc = "BIST March Element Register 15 - Channel 0"]
pub mod bistm15_ch0;
#[doc = "BISTM15_CH1 (rw) register accessor: an alias for `Reg<BISTM15_CH1_SPEC>`"]
pub type BISTM15_CH1 = crate::Reg<bistm15_ch1::BISTM15_CH1_SPEC>;
#[doc = "BIST March Element Register 15 - Channel 1"]
pub mod bistm15_ch1;
#[doc = "ADFT (rw) register accessor: an alias for `Reg<ADFT_SPEC>`"]
pub type ADFT = crate::Reg<adft::ADFT_SPEC>;
#[doc = "ADFT Register"]
pub mod adft;
#[doc = "OUTBYPEN0 (rw) register accessor: an alias for `Reg<OUTBYPEN0_SPEC>`"]
pub type OUTBYPEN0 = crate::Reg<outbypen0::OUTBYPEN0_SPEC>;
#[doc = "Output Bypass Enable Register 0"]
pub mod outbypen0;
#[doc = "OUTBYPEN1 (rw) register accessor: an alias for `Reg<OUTBYPEN1_SPEC>`"]
pub type OUTBYPEN1 = crate::Reg<outbypen1::OUTBYPEN1_SPEC>;
#[doc = "Output Bypass Enable Register 1"]
pub mod outbypen1;
#[doc = "OUTD0 (rw) register accessor: an alias for `Reg<OUTD0_SPEC>`"]
pub type OUTD0 = crate::Reg<outd0::OUTD0_SPEC>;
#[doc = "Output Data In Register 0"]
pub mod outd0;
#[doc = "OUTD1 (rw) register accessor: an alias for `Reg<OUTD1_SPEC>`"]
pub type OUTD1 = crate::Reg<outd1::OUTD1_SPEC>;
#[doc = "Output Data In Register 1"]
pub mod outd1;
#[doc = "INECC0 (rw) register accessor: an alias for `Reg<INECC0_SPEC>`"]
pub type INECC0 = crate::Reg<inecc0::INECC0_SPEC>;
#[doc = "Inline ECC Register 0"]
pub mod inecc0;
#[doc = "INECC1 (rw) register accessor: an alias for `Reg<INECC1_SPEC>`"]
pub type INECC1 = crate::Reg<inecc1::INECC1_SPEC>;
#[doc = "Inline ECC Register 1"]
pub mod inecc1;
#[doc = "INECC2 (rw) register accessor: an alias for `Reg<INECC2_SPEC>`"]
pub type INECC2 = crate::Reg<inecc2::INECC2_SPEC>;
#[doc = "Inline ECC Register 2"]
pub mod inecc2;
#[doc = "DVSTT0 (r) register accessor: an alias for `Reg<DVSTT0_SPEC>`"]
pub type DVSTT0 = crate::Reg<dvstt0::DVSTT0_SPEC>;
#[doc = "Device ID Status Register"]
pub mod dvstt0;
#[doc = "DVSTT1 (r) register accessor: an alias for `Reg<DVSTT1_SPEC>`"]
pub type DVSTT1 = crate::Reg<dvstt1::DVSTT1_SPEC>;
#[doc = "Device Mode Status Register"]
pub mod dvstt1;
#[doc = "OPSTT_CH0 (r) register accessor: an alias for `Reg<OPSTT_CH0_SPEC>`"]
pub type OPSTT_CH0 = crate::Reg<opstt_ch0::OPSTT_CH0_SPEC>;
#[doc = "Operation Status Register 1 - Channel 0"]
pub mod opstt_ch0;
#[doc = "OPSTT_CH1 (r) register accessor: an alias for `Reg<OPSTT_CH1_SPEC>`"]
pub type OPSTT_CH1 = crate::Reg<opstt_ch1::OPSTT_CH1_SPEC>;
#[doc = "Operation Status Register 1 - Channel 1"]
pub mod opstt_ch1;
#[doc = "INTSTT_CH0 (r) register accessor: an alias for `Reg<INTSTT_CH0_SPEC>`"]
pub type INTSTT_CH0 = crate::Reg<intstt_ch0::INTSTT_CH0_SPEC>;
#[doc = "Interrupt Status Register - Channel 0"]
pub mod intstt_ch0;
#[doc = "INTSTT_CH1 (r) register accessor: an alias for `Reg<INTSTT_CH1_SPEC>`"]
pub type INTSTT_CH1 = crate::Reg<intstt_ch1::INTSTT_CH1_SPEC>;
#[doc = "Interrupt Status Register - Channel 1"]
pub mod intstt_ch1;
#[doc = "BISTSTT0 (r) register accessor: an alias for `Reg<BISTSTT0_SPEC>`"]
pub type BISTSTT0 = crate::Reg<biststt0::BISTSTT0_SPEC>;
#[doc = "BIST Status 0"]
pub mod biststt0;
#[doc = "BISTSTT1 (r) register accessor: an alias for `Reg<BISTSTT1_SPEC>`"]
pub type BISTSTT1 = crate::Reg<biststt1::BISTSTT1_SPEC>;
#[doc = "BIST Status 1"]
pub mod biststt1;
#[doc = "DDRBISTSTT_CH0 (r) register accessor: an alias for `Reg<DDRBISTSTT_CH0_SPEC>`"]
pub type DDRBISTSTT_CH0 = crate::Reg<ddrbiststt_ch0::DDRBISTSTT_CH0_SPEC>;
#[doc = "DDR BIST Status Register - Channel 0"]
pub mod ddrbiststt_ch0;
#[doc = "DDRBISTSTT_CH1 (r) register accessor: an alias for `Reg<DDRBISTSTT_CH1_SPEC>`"]
pub type DDRBISTSTT_CH1 = crate::Reg<ddrbiststt_ch1::DDRBISTSTT_CH1_SPEC>;
#[doc = "DDR BIST Status Register - Channel 1"]
pub mod ddrbiststt_ch1;
#[doc = "POS (r) register accessor: an alias for `Reg<POS_SPEC>`"]
pub type POS = crate::Reg<pos::POS_SPEC>;
#[doc = "PHY Operation Status Register"]
pub mod pos;
#[doc = "PTS0 (r) register accessor: an alias for `Reg<PTS0_SPEC>`"]
pub type PTS0 = crate::Reg<pts0::PTS0_SPEC>;
#[doc = "PHY Training Status Register 0"]
pub mod pts0;
#[doc = "PTS1 (r) register accessor: an alias for `Reg<PTS1_SPEC>`"]
pub type PTS1 = crate::Reg<pts1::PTS1_SPEC>;
#[doc = "PHY Training Status Register 1"]
pub mod pts1;
#[doc = "PTS2 (r) register accessor: an alias for `Reg<PTS2_SPEC>`"]
pub type PTS2 = crate::Reg<pts2::PTS2_SPEC>;
#[doc = "PHY Training Status Register 2"]
pub mod pts2;
#[doc = "PTS3 (r) register accessor: an alias for `Reg<PTS3_SPEC>`"]
pub type PTS3 = crate::Reg<pts3::PTS3_SPEC>;
#[doc = "PHY Training Status Register 3"]
pub mod pts3;
#[doc = "DLLSTTCA (r) register accessor: an alias for `Reg<DLLSTTCA_SPEC>`"]
pub type DLLSTTCA = crate::Reg<dllsttca::DLLSTTCA_SPEC>;
#[doc = "DLL Status Register for PHY Command Module"]
pub mod dllsttca;
#[doc = "DLLSTTDQ0 (r) register accessor: an alias for `Reg<DLLSTTDQ0_SPEC>`"]
pub type DLLSTTDQ0 = crate::Reg<dllsttdq0::DLLSTTDQ0_SPEC>;
#[doc = "DLL Status Register for PHY Data Module 0"]
pub mod dllsttdq0;
#[doc = "DLLSTTDQ1 (r) register accessor: an alias for `Reg<DLLSTTDQ1_SPEC>`"]
pub type DLLSTTDQ1 = crate::Reg<dllsttdq1::DLLSTTDQ1_SPEC>;
#[doc = "DLL Status Register for PHY Data Module 1"]
pub mod dllsttdq1;
#[doc = "PBSR (r) register accessor: an alias for `Reg<PBSR_SPEC>`"]
pub type PBSR = crate::Reg<pbsr::PBSR_SPEC>;
#[doc = "PHY BIST Status Register"]
pub mod pbsr;
#[doc = "PCSR_CH0 (r) register accessor: an alias for `Reg<PCSR_CH0_SPEC>`"]
pub type PCSR_CH0 = crate::Reg<pcsr_ch0::PCSR_CH0_SPEC>;
#[doc = "PHY Compensation Status Register - Channel 0"]
pub mod pcsr_ch0;
#[doc = "PCSR_CH1 (r) register accessor: an alias for `Reg<PCSR_CH1_SPEC>`"]
pub type PCSR_CH1 = crate::Reg<pcsr_ch1::PCSR_CH1_SPEC>;
#[doc = "PHY Compensation Status Register - Channel 1"]
pub mod pcsr_ch1;
#[doc = "MRR_CH0 (r) register accessor: an alias for `Reg<MRR_CH0_SPEC>`"]
pub type MRR_CH0 = crate::Reg<mrr_ch0::MRR_CH0_SPEC>;
#[doc = "MR Readout Register - Channel 0"]
pub mod mrr_ch0;
#[doc = "MRR_CH1 (r) register accessor: an alias for `Reg<MRR_CH1_SPEC>`"]
pub type MRR_CH1 = crate::Reg<mrr_ch1::MRR_CH1_SPEC>;
#[doc = "MR Readout Register - Channel 1"]
pub mod mrr_ch1;
#[doc = "SHAD_LPMR1 (r) register accessor: an alias for `Reg<SHAD_LPMR1_SPEC>`"]
pub type SHAD_LPMR1 = crate::Reg<shad_lpmr1::SHAD_LPMR1_SPEC>;
#[doc = "Shadow LPDDR Mode Register 1"]
pub mod shad_lpmr1;
#[doc = "SHAD_LPMR2 (r) register accessor: an alias for `Reg<SHAD_LPMR2_SPEC>`"]
pub type SHAD_LPMR2 = crate::Reg<shad_lpmr2::SHAD_LPMR2_SPEC>;
#[doc = "Shadow LPDDR Mode Register 2"]
pub mod shad_lpmr2;
#[doc = "SHAD_LPMR3 (r) register accessor: an alias for `Reg<SHAD_LPMR3_SPEC>`"]
pub type SHAD_LPMR3 = crate::Reg<shad_lpmr3::SHAD_LPMR3_SPEC>;
#[doc = "Shadow LPDDR Mode Register 3"]
pub mod shad_lpmr3;
#[doc = "SHAD_LPMR11 (r) register accessor: an alias for `Reg<SHAD_LPMR11_SPEC>`"]
pub type SHAD_LPMR11 = crate::Reg<shad_lpmr11::SHAD_LPMR11_SPEC>;
#[doc = "Shadow LPDDR Mode Register 11"]
pub mod shad_lpmr11;
#[doc = "SHAD_LPMR12 (r) register accessor: an alias for `Reg<SHAD_LPMR12_SPEC>`"]
pub type SHAD_LPMR12 = crate::Reg<shad_lpmr12::SHAD_LPMR12_SPEC>;
#[doc = "Shadow LPDDR Mode Register 12"]
pub mod shad_lpmr12;
#[doc = "SHAD_LPMR13 (r) register accessor: an alias for `Reg<SHAD_LPMR13_SPEC>`"]
pub type SHAD_LPMR13 = crate::Reg<shad_lpmr13::SHAD_LPMR13_SPEC>;
#[doc = "Shadow LPDDR Mode Register 13"]
pub mod shad_lpmr13;
#[doc = "SHAD_LPMR14 (r) register accessor: an alias for `Reg<SHAD_LPMR14_SPEC>`"]
pub type SHAD_LPMR14 = crate::Reg<shad_lpmr14::SHAD_LPMR14_SPEC>;
#[doc = "Shadow LPDDR Mode Register 14"]
pub mod shad_lpmr14;
#[doc = "SHAD_LPMR22 (r) register accessor: an alias for `Reg<SHAD_LPMR22_SPEC>`"]
pub type SHAD_LPMR22 = crate::Reg<shad_lpmr22::SHAD_LPMR22_SPEC>;
#[doc = "Shadow LPDDR Mode Register 22"]
pub mod shad_lpmr22;
#[doc = "RESP (r) register accessor: an alias for `Reg<RESP_SPEC>`"]
pub type RESP = crate::Reg<resp::RESP_SPEC>;
#[doc = "PHY Response Register"]
pub mod resp;
#[doc = "DATA0 (r) register accessor: an alias for `Reg<DATA0_SPEC>`"]
pub type DATA0 = crate::Reg<data0::DATA0_SPEC>;
#[doc = "Read Data Register 0"]
pub mod data0;
#[doc = "DATA1 (r) register accessor: an alias for `Reg<DATA1_SPEC>`"]
pub type DATA1 = crate::Reg<data1::DATA1_SPEC>;
#[doc = "Read Data Register 1"]
pub mod data1;
#[doc = "DATA2 (r) register accessor: an alias for `Reg<DATA2_SPEC>`"]
pub type DATA2 = crate::Reg<data2::DATA2_SPEC>;
#[doc = "Read Data Register 2"]
pub mod data2;
#[doc = "DATA3 (r) register accessor: an alias for `Reg<DATA3_SPEC>`"]
pub type DATA3 = crate::Reg<data3::DATA3_SPEC>;
#[doc = "Read Data Register 3"]
pub mod data3;
#[doc = "DATA4 (r) register accessor: an alias for `Reg<DATA4_SPEC>`"]
pub type DATA4 = crate::Reg<data4::DATA4_SPEC>;
#[doc = "Read Data Register 4"]
pub mod data4;
#[doc = "DATA5 (r) register accessor: an alias for `Reg<DATA5_SPEC>`"]
pub type DATA5 = crate::Reg<data5::DATA5_SPEC>;
#[doc = "Read Data Register 5"]
pub mod data5;
#[doc = "DATA6 (r) register accessor: an alias for `Reg<DATA6_SPEC>`"]
pub type DATA6 = crate::Reg<data6::DATA6_SPEC>;
#[doc = "Read Data Register 6"]
pub mod data6;
#[doc = "DATA7 (r) register accessor: an alias for `Reg<DATA7_SPEC>`"]
pub type DATA7 = crate::Reg<data7::DATA7_SPEC>;
#[doc = "Read Data Register 7"]
pub mod data7;
#[doc = "DATA8 (r) register accessor: an alias for `Reg<DATA8_SPEC>`"]
pub type DATA8 = crate::Reg<data8::DATA8_SPEC>;
#[doc = "Read Data Register 8"]
pub mod data8;
#[doc = "INECCSTT0 (r) register accessor: an alias for `Reg<INECCSTT0_SPEC>`"]
pub type INECCSTT0 = crate::Reg<ineccstt0::INECCSTT0_SPEC>;
#[doc = "Inline ECC Status Register 0"]
pub mod ineccstt0;
#[doc = "INECCSTT1 (r) register accessor: an alias for `Reg<INECCSTT1_SPEC>`"]
pub type INECCSTT1 = crate::Reg<ineccstt1::INECCSTT1_SPEC>;
#[doc = "Inline ECC Status Register 1"]
pub mod ineccstt1;
#[doc = "INECCSTT2 (r) register accessor: an alias for `Reg<INECCSTT2_SPEC>`"]
pub type INECCSTT2 = crate::Reg<ineccstt2::INECCSTT2_SPEC>;
#[doc = "Inline ECC Status Register 2"]
pub mod ineccstt2;
#[doc = "INECCSTT3 (r) register accessor: an alias for `Reg<INECCSTT3_SPEC>`"]
pub type INECCSTT3 = crate::Reg<ineccstt3::INECCSTT3_SPEC>;
#[doc = "Inline ECC Status Register 3"]
pub mod ineccstt3;
#[doc = "INECCSTT4 (rw) register accessor: an alias for `Reg<INECCSTT4_SPEC>`"]
pub type INECCSTT4 = crate::Reg<ineccstt4::INECCSTT4_SPEC>;
#[doc = "Inline ECC Status Register 4"]
pub mod ineccstt4;
