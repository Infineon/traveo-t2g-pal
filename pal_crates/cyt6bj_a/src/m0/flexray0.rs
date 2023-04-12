#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub ctl: CTL,
    #[doc = "0x04 - DMA Control Register"]
    pub dma_ctl: DMA_CTL,
    _reserved2: [u8; 0x08],
    #[doc = "0x10 - Test Register 1"]
    pub test1: TEST1,
    #[doc = "0x14 - Test Register 2"]
    pub test2: TEST2,
    _reserved4: [u8; 0x04],
    #[doc = "0x1c - Lock Register"]
    pub lck: LCK,
    #[doc = "0x20 - Error Interrupt Register"]
    pub eir: EIR,
    #[doc = "0x24 - Status Interrupt Register"]
    pub sir: SIR,
    #[doc = "0x28 - Error Interrupt Line Select"]
    pub eils: EILS,
    #[doc = "0x2c - Status Interrupt Line Select"]
    pub sils: SILS,
    #[doc = "0x30 - Error Interrupt Enable Set"]
    pub eies: EIES,
    #[doc = "0x34 - Error Interrupt Enable Reset"]
    pub eier: EIER,
    #[doc = "0x38 - Status Interrupt Enable Set"]
    pub sies: SIES,
    #[doc = "0x3c - Status Interrupt Enable Reset"]
    pub sier: SIER,
    #[doc = "0x40 - Interrupt Line Enable"]
    pub ile: ILE,
    #[doc = "0x44 - Timer 0 Configuration"]
    pub t0c: T0C,
    #[doc = "0x48 - Timer 1 Configuration"]
    pub t1c: T1C,
    #[doc = "0x4c - Stop Watch Register 1"]
    pub stpw1: STPW1,
    #[doc = "0x50 - Stop Watch Register 2"]
    pub stpw2: STPW2,
    _reserved18: [u8; 0x2c],
    #[doc = "0x80 - SUC Configuration Register 1"]
    pub succ1: SUCC1,
    #[doc = "0x84 - SUC Configuration Register 2"]
    pub succ2: SUCC2,
    #[doc = "0x88 - SUC Configuration Register 3"]
    pub succ3: SUCC3,
    #[doc = "0x8c - NEM Configuration Register"]
    pub nemc: NEMC,
    #[doc = "0x90 - PRT Configuration Register 1"]
    pub prtc1: PRTC1,
    #[doc = "0x94 - PRT Configuration Register 2"]
    pub prtc2: PRTC2,
    #[doc = "0x98 - MHD Configuration Register"]
    pub mhdc: MHDC,
    _reserved25: [u8; 0x04],
    #[doc = "0xa0 - GTU Configuration Register 1"]
    pub gtuc1: GTUC1,
    #[doc = "0xa4 - GTU Configuration Register 2"]
    pub gtuc2: GTUC2,
    #[doc = "0xa8 - GTU Configuration Register 3"]
    pub gtuc3: GTUC3,
    #[doc = "0xac - GTU Configuration Register 4"]
    pub gtuc4: GTUC4,
    #[doc = "0xb0 - GTU Configuration Register 5"]
    pub gtuc5: GTUC5,
    #[doc = "0xb4 - GTU Configuration Register 6"]
    pub gtuc6: GTUC6,
    #[doc = "0xb8 - GTU Configuration Register 7"]
    pub gtuc7: GTUC7,
    #[doc = "0xbc - GTU Configuration Register 8"]
    pub gtuc8: GTUC8,
    #[doc = "0xc0 - GTU Configuration Register 9"]
    pub gtuc9: GTUC9,
    #[doc = "0xc4 - GTU Configuration Register 10"]
    pub gtuc10: GTUC10,
    #[doc = "0xc8 - GTU Configuration Register 11"]
    pub gtuc11: GTUC11,
    _reserved36: [u8; 0x34],
    #[doc = "0x100 - CC Status Vector"]
    pub ccsv: CCSV,
    #[doc = "0x104 - CC Error Vector"]
    pub ccev: CCEV,
    _reserved38: [u8; 0x08],
    #[doc = "0x110 - Slot Counter Value"]
    pub scv: SCV,
    #[doc = "0x114 - Macrotick and Cycle Counter Value"]
    pub mtccv: MTCCV,
    #[doc = "0x118 - Rate Correction Value"]
    pub rcv: RCV,
    #[doc = "0x11c - Offset Correction Value"]
    pub ocv: OCV,
    #[doc = "0x120 - Sync Frame Status"]
    pub sfs: SFS,
    #[doc = "0x124 - Symbol Window and NIT Status"]
    pub swnit: SWNIT,
    #[doc = "0x128 - Aggregated Channel Status"]
    pub acs: ACS,
    _reserved45: [u8; 0x04],
    #[doc = "0x130..0x16c - Even Sync ID \\[1...15\\]"]
    pub esid: [ESID; 15],
    _reserved46: [u8; 0x04],
    #[doc = "0x170..0x1ac - Odd Sync ID \\[1...15\\]"]
    pub osid: [OSID; 15],
    _reserved47: [u8; 0x04],
    #[doc = "0x1b0 - Network Management Vector 1"]
    pub nmv1: NMV1,
    #[doc = "0x1b4 - Network Management Vector 2"]
    pub nmv2: NMV2,
    #[doc = "0x1b8 - Network Management Vector 3"]
    pub nmv3: NMV3,
    _reserved50: [u8; 0x0144],
    #[doc = "0x300 - Message RAM Configuration"]
    pub mrc: MRC,
    #[doc = "0x304 - FIFO Rejection Filter"]
    pub frf: FRF,
    #[doc = "0x308 - FIFO Rejection Filter Mask"]
    pub frfm: FRFM,
    #[doc = "0x30c - FIFO Critical Level"]
    pub fcl: FCL,
    #[doc = "0x310 - Message Handler Status"]
    pub mhds: MHDS,
    #[doc = "0x314 - Last Dynamic Transmit Slot"]
    pub ldts: LDTS,
    #[doc = "0x318 - FIFO Status Register"]
    pub fsr: FSR,
    #[doc = "0x31c - Message Handler Constraints Flags"]
    pub mhdf: MHDF,
    #[doc = "0x320 - Transmission Request 1"]
    pub txrq1: TXRQ1,
    #[doc = "0x324 - Transmission Request 2"]
    pub txrq2: TXRQ2,
    #[doc = "0x328 - Transmission Request 3"]
    pub txrq3: TXRQ3,
    #[doc = "0x32c - Transmission Request 4"]
    pub txrq4: TXRQ4,
    #[doc = "0x330 - New Data 1"]
    pub ndat1: NDAT1,
    #[doc = "0x334 - New Data 2"]
    pub ndat2: NDAT2,
    #[doc = "0x338 - New Data 3"]
    pub ndat3: NDAT3,
    #[doc = "0x33c - New Data 4"]
    pub ndat4: NDAT4,
    #[doc = "0x340 - Message Buffer Status Changed 1"]
    pub mbsc1: MBSC1,
    #[doc = "0x344 - Message Buffer Status Changed 2"]
    pub mbsc2: MBSC2,
    #[doc = "0x348 - Message Buffer Status Changed 3"]
    pub mbsc3: MBSC3,
    #[doc = "0x34c - Message Buffer Status Changed 4"]
    pub mbsc4: MBSC4,
    _reserved70: [u8; 0xa0],
    #[doc = "0x3f0 - Core Release Register"]
    pub crel: CREL,
    #[doc = "0x3f4 - Endian Register"]
    pub endn: ENDN,
    _reserved72: [u8; 0x08],
    #[doc = "0x400..0x500 - Write Data Section \\[1...64\\]"]
    pub wrds: [WRDS; 64],
    #[doc = "0x500 - Write Header Section 1"]
    pub wrhs1: WRHS1,
    #[doc = "0x504 - Write Header Section 2"]
    pub wrhs2: WRHS2,
    #[doc = "0x508 - Write Header Section 3"]
    pub wrhs3: WRHS3,
    _reserved76: [u8; 0x04],
    #[doc = "0x510 - Input Buffer Command Mask"]
    pub ibcm: IBCM,
    #[doc = "0x514 - Input Buffer Command Request"]
    pub ibcr: IBCR,
    _reserved78: [u8; 0xe8],
    #[doc = "0x600..0x700 - Read Data Section \\[1...64\\]"]
    pub rdds: [RDDS; 64],
    #[doc = "0x700 - Read Header Section 1"]
    pub rdhs1: RDHS1,
    #[doc = "0x704 - Read Header Section 2"]
    pub rdhs2: RDHS2,
    #[doc = "0x708 - Read Header Section 3"]
    pub rdhs3: RDHS3,
    #[doc = "0x70c - Message Buffer Status"]
    pub mbs: MBS,
    #[doc = "0x710 - Output Buffer Command Mask"]
    pub obcm: OBCM,
    #[doc = "0x714 - Output Buffer Command Request"]
    pub obcr: OBCR,
    _reserved85: [u8; 0x04d8],
    #[doc = "0xbf0 - Write Header Section 1 (2nd mirror)"]
    pub wrhs1_mir2: WRHS1_MIR2,
    #[doc = "0xbf4 - Write Header Section 2 (2nd mirror)"]
    pub wrhs2_mir2: WRHS2_MIR2,
    #[doc = "0xbf8 - Write Header Section 3 (2nd mirror)"]
    pub wrhs3_mir2: WRHS3_MIR2,
    _reserved88: [u8; 0x04],
    #[doc = "0xc00..0xd00 - Write Data Section \\[1...64\\]
(mirror)"]
    pub wrds_mir: [WRDS_MIR; 64],
    #[doc = "0xd00 - Write Header Section 1 (mirror)"]
    pub wrhs1_mir: WRHS1_MIR,
    #[doc = "0xd04 - Write Header Section 2 (mirror)"]
    pub wrhs2_mir: WRHS2_MIR,
    #[doc = "0xd08 - Write Header Section 3 (mirror)"]
    pub wrhs3_mir: WRHS3_MIR,
    _reserved92: [u8; 0x04],
    #[doc = "0xd10 - Input Buffer Command Mask (mirror)"]
    pub ibcm_mir: IBCM_MIR,
    #[doc = "0xd14 - Input Buffer Command Request (mirror)"]
    pub ibcr_mir: IBCR_MIR,
    _reserved94: [u8; 0xd8],
    #[doc = "0xdf0 - Read Header Section 1 (2nd mirror)"]
    pub rdhs1_mir2: RDHS1_MIR2,
    #[doc = "0xdf4 - Read Header Section 2 (2nd mirror)"]
    pub rdhs2_mir2: RDHS2_MIR2,
    #[doc = "0xdf8 - Read Header Section 3 (2nd mirror)"]
    pub rdhs3_mir2: RDHS3_MIR2,
    #[doc = "0xdfc - Message Buffer Status (2nd mirror)"]
    pub mbs_mir2: MBS_MIR2,
    #[doc = "0xe00..0xf00 - Read Data Section \\[1...64\\]
(mirror)"]
    pub rdds_mir: [RDDS_MIR; 64],
    #[doc = "0xf00 - Read Header Section 1 (mirror)"]
    pub rdhs1_mir: RDHS1_MIR,
    #[doc = "0xf04 - Read Header Section 2 (mirror)"]
    pub rdhs2_mir: RDHS2_MIR,
    #[doc = "0xf08 - Read Header Section 3 (mirror)"]
    pub rdhs3_mir: RDHS3_MIR,
    #[doc = "0xf0c - Message Buffer Status (mirror)"]
    pub mbs_mir: MBS_MIR,
    #[doc = "0xf10 - Output Buffer Command Mask (mirror)"]
    pub obcm_mir: OBCM_MIR,
    #[doc = "0xf14 - Output Buffer Command Request (mirror)"]
    pub obcr_mir: OBCR_MIR,
}
#[doc = "CTL (rw) register accessor: an alias for `Reg<CTL_SPEC>`"]
pub type CTL = crate::Reg<ctl::CTL_SPEC>;
#[doc = "Control Register"]
pub mod ctl;
#[doc = "DMA_CTL (rw) register accessor: an alias for `Reg<DMA_CTL_SPEC>`"]
pub type DMA_CTL = crate::Reg<dma_ctl::DMA_CTL_SPEC>;
#[doc = "DMA Control Register"]
pub mod dma_ctl;
#[doc = "TEST1 (rw) register accessor: an alias for `Reg<TEST1_SPEC>`"]
pub type TEST1 = crate::Reg<test1::TEST1_SPEC>;
#[doc = "Test Register 1"]
pub mod test1;
#[doc = "TEST2 (rw) register accessor: an alias for `Reg<TEST2_SPEC>`"]
pub type TEST2 = crate::Reg<test2::TEST2_SPEC>;
#[doc = "Test Register 2"]
pub mod test2;
#[doc = "LCK (w) register accessor: an alias for `Reg<LCK_SPEC>`"]
pub type LCK = crate::Reg<lck::LCK_SPEC>;
#[doc = "Lock Register"]
pub mod lck;
#[doc = "EIR (rw) register accessor: an alias for `Reg<EIR_SPEC>`"]
pub type EIR = crate::Reg<eir::EIR_SPEC>;
#[doc = "Error Interrupt Register"]
pub mod eir;
#[doc = "SIR (rw) register accessor: an alias for `Reg<SIR_SPEC>`"]
pub type SIR = crate::Reg<sir::SIR_SPEC>;
#[doc = "Status Interrupt Register"]
pub mod sir;
#[doc = "EILS (rw) register accessor: an alias for `Reg<EILS_SPEC>`"]
pub type EILS = crate::Reg<eils::EILS_SPEC>;
#[doc = "Error Interrupt Line Select"]
pub mod eils;
#[doc = "SILS (rw) register accessor: an alias for `Reg<SILS_SPEC>`"]
pub type SILS = crate::Reg<sils::SILS_SPEC>;
#[doc = "Status Interrupt Line Select"]
pub mod sils;
#[doc = "EIES (rw) register accessor: an alias for `Reg<EIES_SPEC>`"]
pub type EIES = crate::Reg<eies::EIES_SPEC>;
#[doc = "Error Interrupt Enable Set"]
pub mod eies;
#[doc = "EIER (rw) register accessor: an alias for `Reg<EIER_SPEC>`"]
pub type EIER = crate::Reg<eier::EIER_SPEC>;
#[doc = "Error Interrupt Enable Reset"]
pub mod eier;
#[doc = "SIES (rw) register accessor: an alias for `Reg<SIES_SPEC>`"]
pub type SIES = crate::Reg<sies::SIES_SPEC>;
#[doc = "Status Interrupt Enable Set"]
pub mod sies;
#[doc = "SIER (rw) register accessor: an alias for `Reg<SIER_SPEC>`"]
pub type SIER = crate::Reg<sier::SIER_SPEC>;
#[doc = "Status Interrupt Enable Reset"]
pub mod sier;
#[doc = "ILE (rw) register accessor: an alias for `Reg<ILE_SPEC>`"]
pub type ILE = crate::Reg<ile::ILE_SPEC>;
#[doc = "Interrupt Line Enable"]
pub mod ile;
#[doc = "T0C (rw) register accessor: an alias for `Reg<T0C_SPEC>`"]
pub type T0C = crate::Reg<t0c::T0C_SPEC>;
#[doc = "Timer 0 Configuration"]
pub mod t0c;
#[doc = "T1C (rw) register accessor: an alias for `Reg<T1C_SPEC>`"]
pub type T1C = crate::Reg<t1c::T1C_SPEC>;
#[doc = "Timer 1 Configuration"]
pub mod t1c;
#[doc = "STPW1 (rw) register accessor: an alias for `Reg<STPW1_SPEC>`"]
pub type STPW1 = crate::Reg<stpw1::STPW1_SPEC>;
#[doc = "Stop Watch Register 1"]
pub mod stpw1;
#[doc = "STPW2 (r) register accessor: an alias for `Reg<STPW2_SPEC>`"]
pub type STPW2 = crate::Reg<stpw2::STPW2_SPEC>;
#[doc = "Stop Watch Register 2"]
pub mod stpw2;
#[doc = "SUCC1 (rw) register accessor: an alias for `Reg<SUCC1_SPEC>`"]
pub type SUCC1 = crate::Reg<succ1::SUCC1_SPEC>;
#[doc = "SUC Configuration Register 1"]
pub mod succ1;
#[doc = "SUCC2 (rw) register accessor: an alias for `Reg<SUCC2_SPEC>`"]
pub type SUCC2 = crate::Reg<succ2::SUCC2_SPEC>;
#[doc = "SUC Configuration Register 2"]
pub mod succ2;
#[doc = "SUCC3 (rw) register accessor: an alias for `Reg<SUCC3_SPEC>`"]
pub type SUCC3 = crate::Reg<succ3::SUCC3_SPEC>;
#[doc = "SUC Configuration Register 3"]
pub mod succ3;
#[doc = "NEMC (rw) register accessor: an alias for `Reg<NEMC_SPEC>`"]
pub type NEMC = crate::Reg<nemc::NEMC_SPEC>;
#[doc = "NEM Configuration Register"]
pub mod nemc;
#[doc = "PRTC1 (rw) register accessor: an alias for `Reg<PRTC1_SPEC>`"]
pub type PRTC1 = crate::Reg<prtc1::PRTC1_SPEC>;
#[doc = "PRT Configuration Register 1"]
pub mod prtc1;
#[doc = "PRTC2 (rw) register accessor: an alias for `Reg<PRTC2_SPEC>`"]
pub type PRTC2 = crate::Reg<prtc2::PRTC2_SPEC>;
#[doc = "PRT Configuration Register 2"]
pub mod prtc2;
#[doc = "MHDC (rw) register accessor: an alias for `Reg<MHDC_SPEC>`"]
pub type MHDC = crate::Reg<mhdc::MHDC_SPEC>;
#[doc = "MHD Configuration Register"]
pub mod mhdc;
#[doc = "GTUC1 (rw) register accessor: an alias for `Reg<GTUC1_SPEC>`"]
pub type GTUC1 = crate::Reg<gtuc1::GTUC1_SPEC>;
#[doc = "GTU Configuration Register 1"]
pub mod gtuc1;
#[doc = "GTUC2 (rw) register accessor: an alias for `Reg<GTUC2_SPEC>`"]
pub type GTUC2 = crate::Reg<gtuc2::GTUC2_SPEC>;
#[doc = "GTU Configuration Register 2"]
pub mod gtuc2;
#[doc = "GTUC3 (rw) register accessor: an alias for `Reg<GTUC3_SPEC>`"]
pub type GTUC3 = crate::Reg<gtuc3::GTUC3_SPEC>;
#[doc = "GTU Configuration Register 3"]
pub mod gtuc3;
#[doc = "GTUC4 (rw) register accessor: an alias for `Reg<GTUC4_SPEC>`"]
pub type GTUC4 = crate::Reg<gtuc4::GTUC4_SPEC>;
#[doc = "GTU Configuration Register 4"]
pub mod gtuc4;
#[doc = "GTUC5 (rw) register accessor: an alias for `Reg<GTUC5_SPEC>`"]
pub type GTUC5 = crate::Reg<gtuc5::GTUC5_SPEC>;
#[doc = "GTU Configuration Register 5"]
pub mod gtuc5;
#[doc = "GTUC6 (rw) register accessor: an alias for `Reg<GTUC6_SPEC>`"]
pub type GTUC6 = crate::Reg<gtuc6::GTUC6_SPEC>;
#[doc = "GTU Configuration Register 6"]
pub mod gtuc6;
#[doc = "GTUC7 (rw) register accessor: an alias for `Reg<GTUC7_SPEC>`"]
pub type GTUC7 = crate::Reg<gtuc7::GTUC7_SPEC>;
#[doc = "GTU Configuration Register 7"]
pub mod gtuc7;
#[doc = "GTUC8 (rw) register accessor: an alias for `Reg<GTUC8_SPEC>`"]
pub type GTUC8 = crate::Reg<gtuc8::GTUC8_SPEC>;
#[doc = "GTU Configuration Register 8"]
pub mod gtuc8;
#[doc = "GTUC9 (rw) register accessor: an alias for `Reg<GTUC9_SPEC>`"]
pub type GTUC9 = crate::Reg<gtuc9::GTUC9_SPEC>;
#[doc = "GTU Configuration Register 9"]
pub mod gtuc9;
#[doc = "GTUC10 (rw) register accessor: an alias for `Reg<GTUC10_SPEC>`"]
pub type GTUC10 = crate::Reg<gtuc10::GTUC10_SPEC>;
#[doc = "GTU Configuration Register 10"]
pub mod gtuc10;
#[doc = "GTUC11 (rw) register accessor: an alias for `Reg<GTUC11_SPEC>`"]
pub type GTUC11 = crate::Reg<gtuc11::GTUC11_SPEC>;
#[doc = "GTU Configuration Register 11"]
pub mod gtuc11;
#[doc = "CCSV (r) register accessor: an alias for `Reg<CCSV_SPEC>`"]
pub type CCSV = crate::Reg<ccsv::CCSV_SPEC>;
#[doc = "CC Status Vector"]
pub mod ccsv;
#[doc = "CCEV (r) register accessor: an alias for `Reg<CCEV_SPEC>`"]
pub type CCEV = crate::Reg<ccev::CCEV_SPEC>;
#[doc = "CC Error Vector"]
pub mod ccev;
#[doc = "SCV (r) register accessor: an alias for `Reg<SCV_SPEC>`"]
pub type SCV = crate::Reg<scv::SCV_SPEC>;
#[doc = "Slot Counter Value"]
pub mod scv;
#[doc = "MTCCV (r) register accessor: an alias for `Reg<MTCCV_SPEC>`"]
pub type MTCCV = crate::Reg<mtccv::MTCCV_SPEC>;
#[doc = "Macrotick and Cycle Counter Value"]
pub mod mtccv;
#[doc = "RCV (r) register accessor: an alias for `Reg<RCV_SPEC>`"]
pub type RCV = crate::Reg<rcv::RCV_SPEC>;
#[doc = "Rate Correction Value"]
pub mod rcv;
#[doc = "OCV (r) register accessor: an alias for `Reg<OCV_SPEC>`"]
pub type OCV = crate::Reg<ocv::OCV_SPEC>;
#[doc = "Offset Correction Value"]
pub mod ocv;
#[doc = "SFS (r) register accessor: an alias for `Reg<SFS_SPEC>`"]
pub type SFS = crate::Reg<sfs::SFS_SPEC>;
#[doc = "Sync Frame Status"]
pub mod sfs;
#[doc = "SWNIT (r) register accessor: an alias for `Reg<SWNIT_SPEC>`"]
pub type SWNIT = crate::Reg<swnit::SWNIT_SPEC>;
#[doc = "Symbol Window and NIT Status"]
pub mod swnit;
#[doc = "ACS (rw) register accessor: an alias for `Reg<ACS_SPEC>`"]
pub type ACS = crate::Reg<acs::ACS_SPEC>;
#[doc = "Aggregated Channel Status"]
pub mod acs;
#[doc = "ESID (r) register accessor: an alias for `Reg<ESID_SPEC>`"]
pub type ESID = crate::Reg<esid::ESID_SPEC>;
#[doc = "Even Sync ID \\[1...15\\]"]
pub mod esid;
#[doc = "OSID (r) register accessor: an alias for `Reg<OSID_SPEC>`"]
pub type OSID = crate::Reg<osid::OSID_SPEC>;
#[doc = "Odd Sync ID \\[1...15\\]"]
pub mod osid;
#[doc = "NMV1 (r) register accessor: an alias for `Reg<NMV1_SPEC>`"]
pub type NMV1 = crate::Reg<nmv1::NMV1_SPEC>;
#[doc = "Network Management Vector 1"]
pub mod nmv1;
#[doc = "NMV2 (r) register accessor: an alias for `Reg<NMV2_SPEC>`"]
pub type NMV2 = crate::Reg<nmv2::NMV2_SPEC>;
#[doc = "Network Management Vector 2"]
pub mod nmv2;
#[doc = "NMV3 (r) register accessor: an alias for `Reg<NMV3_SPEC>`"]
pub type NMV3 = crate::Reg<nmv3::NMV3_SPEC>;
#[doc = "Network Management Vector 3"]
pub mod nmv3;
#[doc = "MRC (rw) register accessor: an alias for `Reg<MRC_SPEC>`"]
pub type MRC = crate::Reg<mrc::MRC_SPEC>;
#[doc = "Message RAM Configuration"]
pub mod mrc;
#[doc = "FRF (rw) register accessor: an alias for `Reg<FRF_SPEC>`"]
pub type FRF = crate::Reg<frf::FRF_SPEC>;
#[doc = "FIFO Rejection Filter"]
pub mod frf;
#[doc = "FRFM (rw) register accessor: an alias for `Reg<FRFM_SPEC>`"]
pub type FRFM = crate::Reg<frfm::FRFM_SPEC>;
#[doc = "FIFO Rejection Filter Mask"]
pub mod frfm;
#[doc = "FCL (rw) register accessor: an alias for `Reg<FCL_SPEC>`"]
pub type FCL = crate::Reg<fcl::FCL_SPEC>;
#[doc = "FIFO Critical Level"]
pub mod fcl;
#[doc = "MHDS (rw) register accessor: an alias for `Reg<MHDS_SPEC>`"]
pub type MHDS = crate::Reg<mhds::MHDS_SPEC>;
#[doc = "Message Handler Status"]
pub mod mhds;
#[doc = "LDTS (r) register accessor: an alias for `Reg<LDTS_SPEC>`"]
pub type LDTS = crate::Reg<ldts::LDTS_SPEC>;
#[doc = "Last Dynamic Transmit Slot"]
pub mod ldts;
#[doc = "FSR (r) register accessor: an alias for `Reg<FSR_SPEC>`"]
pub type FSR = crate::Reg<fsr::FSR_SPEC>;
#[doc = "FIFO Status Register"]
pub mod fsr;
#[doc = "MHDF (rw) register accessor: an alias for `Reg<MHDF_SPEC>`"]
pub type MHDF = crate::Reg<mhdf::MHDF_SPEC>;
#[doc = "Message Handler Constraints Flags"]
pub mod mhdf;
#[doc = "TXRQ1 (r) register accessor: an alias for `Reg<TXRQ1_SPEC>`"]
pub type TXRQ1 = crate::Reg<txrq1::TXRQ1_SPEC>;
#[doc = "Transmission Request 1"]
pub mod txrq1;
#[doc = "TXRQ2 (r) register accessor: an alias for `Reg<TXRQ2_SPEC>`"]
pub type TXRQ2 = crate::Reg<txrq2::TXRQ2_SPEC>;
#[doc = "Transmission Request 2"]
pub mod txrq2;
#[doc = "TXRQ3 (r) register accessor: an alias for `Reg<TXRQ3_SPEC>`"]
pub type TXRQ3 = crate::Reg<txrq3::TXRQ3_SPEC>;
#[doc = "Transmission Request 3"]
pub mod txrq3;
#[doc = "TXRQ4 (r) register accessor: an alias for `Reg<TXRQ4_SPEC>`"]
pub type TXRQ4 = crate::Reg<txrq4::TXRQ4_SPEC>;
#[doc = "Transmission Request 4"]
pub mod txrq4;
#[doc = "NDAT1 (r) register accessor: an alias for `Reg<NDAT1_SPEC>`"]
pub type NDAT1 = crate::Reg<ndat1::NDAT1_SPEC>;
#[doc = "New Data 1"]
pub mod ndat1;
#[doc = "NDAT2 (r) register accessor: an alias for `Reg<NDAT2_SPEC>`"]
pub type NDAT2 = crate::Reg<ndat2::NDAT2_SPEC>;
#[doc = "New Data 2"]
pub mod ndat2;
#[doc = "NDAT3 (r) register accessor: an alias for `Reg<NDAT3_SPEC>`"]
pub type NDAT3 = crate::Reg<ndat3::NDAT3_SPEC>;
#[doc = "New Data 3"]
pub mod ndat3;
#[doc = "NDAT4 (r) register accessor: an alias for `Reg<NDAT4_SPEC>`"]
pub type NDAT4 = crate::Reg<ndat4::NDAT4_SPEC>;
#[doc = "New Data 4"]
pub mod ndat4;
#[doc = "MBSC1 (r) register accessor: an alias for `Reg<MBSC1_SPEC>`"]
pub type MBSC1 = crate::Reg<mbsc1::MBSC1_SPEC>;
#[doc = "Message Buffer Status Changed 1"]
pub mod mbsc1;
#[doc = "MBSC2 (r) register accessor: an alias for `Reg<MBSC2_SPEC>`"]
pub type MBSC2 = crate::Reg<mbsc2::MBSC2_SPEC>;
#[doc = "Message Buffer Status Changed 2"]
pub mod mbsc2;
#[doc = "MBSC3 (r) register accessor: an alias for `Reg<MBSC3_SPEC>`"]
pub type MBSC3 = crate::Reg<mbsc3::MBSC3_SPEC>;
#[doc = "Message Buffer Status Changed 3"]
pub mod mbsc3;
#[doc = "MBSC4 (r) register accessor: an alias for `Reg<MBSC4_SPEC>`"]
pub type MBSC4 = crate::Reg<mbsc4::MBSC4_SPEC>;
#[doc = "Message Buffer Status Changed 4"]
pub mod mbsc4;
#[doc = "CREL (r) register accessor: an alias for `Reg<CREL_SPEC>`"]
pub type CREL = crate::Reg<crel::CREL_SPEC>;
#[doc = "Core Release Register"]
pub mod crel;
#[doc = "ENDN (r) register accessor: an alias for `Reg<ENDN_SPEC>`"]
pub type ENDN = crate::Reg<endn::ENDN_SPEC>;
#[doc = "Endian Register"]
pub mod endn;
#[doc = "WRDS (rw) register accessor: an alias for `Reg<WRDS_SPEC>`"]
pub type WRDS = crate::Reg<wrds::WRDS_SPEC>;
#[doc = "Write Data Section \\[1...64\\]"]
pub mod wrds;
#[doc = "WRHS1 (rw) register accessor: an alias for `Reg<WRHS1_SPEC>`"]
pub type WRHS1 = crate::Reg<wrhs1::WRHS1_SPEC>;
#[doc = "Write Header Section 1"]
pub mod wrhs1;
#[doc = "WRHS2 (rw) register accessor: an alias for `Reg<WRHS2_SPEC>`"]
pub type WRHS2 = crate::Reg<wrhs2::WRHS2_SPEC>;
#[doc = "Write Header Section 2"]
pub mod wrhs2;
#[doc = "WRHS3 (rw) register accessor: an alias for `Reg<WRHS3_SPEC>`"]
pub type WRHS3 = crate::Reg<wrhs3::WRHS3_SPEC>;
#[doc = "Write Header Section 3"]
pub mod wrhs3;
#[doc = "IBCM (rw) register accessor: an alias for `Reg<IBCM_SPEC>`"]
pub type IBCM = crate::Reg<ibcm::IBCM_SPEC>;
#[doc = "Input Buffer Command Mask"]
pub mod ibcm;
#[doc = "IBCR (rw) register accessor: an alias for `Reg<IBCR_SPEC>`"]
pub type IBCR = crate::Reg<ibcr::IBCR_SPEC>;
#[doc = "Input Buffer Command Request"]
pub mod ibcr;
#[doc = "RDDS (r) register accessor: an alias for `Reg<RDDS_SPEC>`"]
pub type RDDS = crate::Reg<rdds::RDDS_SPEC>;
#[doc = "Read Data Section \\[1...64\\]"]
pub mod rdds;
#[doc = "RDHS1 (r) register accessor: an alias for `Reg<RDHS1_SPEC>`"]
pub type RDHS1 = crate::Reg<rdhs1::RDHS1_SPEC>;
#[doc = "Read Header Section 1"]
pub mod rdhs1;
#[doc = "RDHS2 (r) register accessor: an alias for `Reg<RDHS2_SPEC>`"]
pub type RDHS2 = crate::Reg<rdhs2::RDHS2_SPEC>;
#[doc = "Read Header Section 2"]
pub mod rdhs2;
#[doc = "RDHS3 (r) register accessor: an alias for `Reg<RDHS3_SPEC>`"]
pub type RDHS3 = crate::Reg<rdhs3::RDHS3_SPEC>;
#[doc = "Read Header Section 3"]
pub mod rdhs3;
#[doc = "MBS (r) register accessor: an alias for `Reg<MBS_SPEC>`"]
pub type MBS = crate::Reg<mbs::MBS_SPEC>;
#[doc = "Message Buffer Status"]
pub mod mbs;
#[doc = "OBCM (rw) register accessor: an alias for `Reg<OBCM_SPEC>`"]
pub type OBCM = crate::Reg<obcm::OBCM_SPEC>;
#[doc = "Output Buffer Command Mask"]
pub mod obcm;
#[doc = "OBCR (rw) register accessor: an alias for `Reg<OBCR_SPEC>`"]
pub type OBCR = crate::Reg<obcr::OBCR_SPEC>;
#[doc = "Output Buffer Command Request"]
pub mod obcr;
#[doc = "WRHS1_MIR2 (rw) register accessor: an alias for `Reg<WRHS1_MIR2_SPEC>`"]
pub type WRHS1_MIR2 = crate::Reg<wrhs1_mir2::WRHS1_MIR2_SPEC>;
#[doc = "Write Header Section 1 (2nd mirror)"]
pub mod wrhs1_mir2;
#[doc = "WRHS2_MIR2 (rw) register accessor: an alias for `Reg<WRHS2_MIR2_SPEC>`"]
pub type WRHS2_MIR2 = crate::Reg<wrhs2_mir2::WRHS2_MIR2_SPEC>;
#[doc = "Write Header Section 2 (2nd mirror)"]
pub mod wrhs2_mir2;
#[doc = "WRHS3_MIR2 (rw) register accessor: an alias for `Reg<WRHS3_MIR2_SPEC>`"]
pub type WRHS3_MIR2 = crate::Reg<wrhs3_mir2::WRHS3_MIR2_SPEC>;
#[doc = "Write Header Section 3 (2nd mirror)"]
pub mod wrhs3_mir2;
#[doc = "WRDS_MIR (rw) register accessor: an alias for `Reg<WRDS_MIR_SPEC>`"]
pub type WRDS_MIR = crate::Reg<wrds_mir::WRDS_MIR_SPEC>;
#[doc = "Write Data Section \\[1...64\\]
(mirror)"]
pub mod wrds_mir;
#[doc = "WRHS1_MIR (rw) register accessor: an alias for `Reg<WRHS1_MIR_SPEC>`"]
pub type WRHS1_MIR = crate::Reg<wrhs1_mir::WRHS1_MIR_SPEC>;
#[doc = "Write Header Section 1 (mirror)"]
pub mod wrhs1_mir;
#[doc = "WRHS2_MIR (rw) register accessor: an alias for `Reg<WRHS2_MIR_SPEC>`"]
pub type WRHS2_MIR = crate::Reg<wrhs2_mir::WRHS2_MIR_SPEC>;
#[doc = "Write Header Section 2 (mirror)"]
pub mod wrhs2_mir;
#[doc = "WRHS3_MIR (rw) register accessor: an alias for `Reg<WRHS3_MIR_SPEC>`"]
pub type WRHS3_MIR = crate::Reg<wrhs3_mir::WRHS3_MIR_SPEC>;
#[doc = "Write Header Section 3 (mirror)"]
pub mod wrhs3_mir;
#[doc = "IBCM_MIR (rw) register accessor: an alias for `Reg<IBCM_MIR_SPEC>`"]
pub type IBCM_MIR = crate::Reg<ibcm_mir::IBCM_MIR_SPEC>;
#[doc = "Input Buffer Command Mask (mirror)"]
pub mod ibcm_mir;
#[doc = "IBCR_MIR (rw) register accessor: an alias for `Reg<IBCR_MIR_SPEC>`"]
pub type IBCR_MIR = crate::Reg<ibcr_mir::IBCR_MIR_SPEC>;
#[doc = "Input Buffer Command Request (mirror)"]
pub mod ibcr_mir;
#[doc = "RDHS1_MIR2 (r) register accessor: an alias for `Reg<RDHS1_MIR2_SPEC>`"]
pub type RDHS1_MIR2 = crate::Reg<rdhs1_mir2::RDHS1_MIR2_SPEC>;
#[doc = "Read Header Section 1 (2nd mirror)"]
pub mod rdhs1_mir2;
#[doc = "RDHS2_MIR2 (r) register accessor: an alias for `Reg<RDHS2_MIR2_SPEC>`"]
pub type RDHS2_MIR2 = crate::Reg<rdhs2_mir2::RDHS2_MIR2_SPEC>;
#[doc = "Read Header Section 2 (2nd mirror)"]
pub mod rdhs2_mir2;
#[doc = "RDHS3_MIR2 (r) register accessor: an alias for `Reg<RDHS3_MIR2_SPEC>`"]
pub type RDHS3_MIR2 = crate::Reg<rdhs3_mir2::RDHS3_MIR2_SPEC>;
#[doc = "Read Header Section 3 (2nd mirror)"]
pub mod rdhs3_mir2;
#[doc = "MBS_MIR2 (r) register accessor: an alias for `Reg<MBS_MIR2_SPEC>`"]
pub type MBS_MIR2 = crate::Reg<mbs_mir2::MBS_MIR2_SPEC>;
#[doc = "Message Buffer Status (2nd mirror)"]
pub mod mbs_mir2;
#[doc = "RDDS_MIR (r) register accessor: an alias for `Reg<RDDS_MIR_SPEC>`"]
pub type RDDS_MIR = crate::Reg<rdds_mir::RDDS_MIR_SPEC>;
#[doc = "Read Data Section \\[1...64\\]
(mirror)"]
pub mod rdds_mir;
#[doc = "RDHS1_MIR (r) register accessor: an alias for `Reg<RDHS1_MIR_SPEC>`"]
pub type RDHS1_MIR = crate::Reg<rdhs1_mir::RDHS1_MIR_SPEC>;
#[doc = "Read Header Section 1 (mirror)"]
pub mod rdhs1_mir;
#[doc = "RDHS2_MIR (r) register accessor: an alias for `Reg<RDHS2_MIR_SPEC>`"]
pub type RDHS2_MIR = crate::Reg<rdhs2_mir::RDHS2_MIR_SPEC>;
#[doc = "Read Header Section 2 (mirror)"]
pub mod rdhs2_mir;
#[doc = "RDHS3_MIR (r) register accessor: an alias for `Reg<RDHS3_MIR_SPEC>`"]
pub type RDHS3_MIR = crate::Reg<rdhs3_mir::RDHS3_MIR_SPEC>;
#[doc = "Read Header Section 3 (mirror)"]
pub mod rdhs3_mir;
#[doc = "MBS_MIR (r) register accessor: an alias for `Reg<MBS_MIR_SPEC>`"]
pub type MBS_MIR = crate::Reg<mbs_mir::MBS_MIR_SPEC>;
#[doc = "Message Buffer Status (mirror)"]
pub mod mbs_mir;
#[doc = "OBCM_MIR (rw) register accessor: an alias for `Reg<OBCM_MIR_SPEC>`"]
pub type OBCM_MIR = crate::Reg<obcm_mir::OBCM_MIR_SPEC>;
#[doc = "Output Buffer Command Mask (mirror)"]
pub mod obcm_mir;
#[doc = "OBCR_MIR (rw) register accessor: an alias for `Reg<OBCR_MIR_SPEC>`"]
pub type OBCR_MIR = crate::Reg<obcr_mir::OBCR_MIR_SPEC>;
#[doc = "Output Buffer Command Request (mirror)"]
pub mod obcr_mir;
