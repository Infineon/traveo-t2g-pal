#[doc = r"Register block"]
#[repr(C)]
pub struct FRAMECAP {
    #[doc = "0x00 - Capture mode configuration. FrameCap input mode configuration."]
    pub capturemode: CAPTUREMODE,
    #[doc = "0x04 - FrameCap control register for starting the unit"]
    pub ctr: CTR,
    #[doc = "0x08 - FrameCap sync signal polarity configuration."]
    pub spr: SPR,
    #[doc = "0x0c - FrameCap control register for starting the mode detection"]
    pub mdr: MDR,
    #[doc = "0x10 - Frame dimension (progressive frame and field 0)."]
    pub fdr: FDR,
    #[doc = "0x14 - Frame dimension (field 1)."]
    pub fdr1: FDR1,
    #[doc = "0x18 - FrameCap kick delay configuration register"]
    pub kdr: KDR,
    #[doc = "0x1c - FrameCap Sync frame number configuration register"]
    pub scr: SCR,
    #[doc = "0x20 - Framecap control register for capture clock supervision"]
    pub csvr: CSVR,
    #[doc = "0x24 - FrameCap status clear register. Clears the status bits of Sts register, Mdr_SizeChange and Csv_lost field."]
    pub stsclr: STSCLR,
    #[doc = "0x28 - FrameCap status register. Current status of the FrameCap module."]
    pub sts: STS,
    #[doc = "0x2c - FrameCap mode detection status register for the active part of the video frames.(bit locked when MdrCmrDone=1)."]
    pub mdsts0: MDSTS0,
    #[doc = "0x30 - FrameCap mode detection status register. The current (x,y) position of the capture stream (with respect to the output)."]
    pub mdsts1: MDSTS1,
    #[doc = "0x34 - FrameCap frame counter register indicating the corresponding clk_axi cycles."]
    pub frcnt: FRCNT,
    #[doc = "0x38 - FrameCap line counter register."]
    pub frlinecount: FRLINECOUNT,
    #[doc = "0x3c - clk_axi cycle number of a frame. (bit locked when MdrCmrDone=1)."]
    pub cmsts1: CMSTS1,
    #[doc = "0x40 - clk_axi cycle number of totwidth and actwidth of a frame. (bit locked when MdrCmrDone=1)."]
    pub cmsts2: CMSTS2,
    #[doc = "0x44 - ITU error status register."]
    pub itusts: ITUSTS,
}
#[doc = "CAPTUREMODE (rw) register accessor: an alias for `Reg<CAPTUREMODE_SPEC>`"]
pub type CAPTUREMODE = crate::Reg<capturemode::CAPTUREMODE_SPEC>;
#[doc = "Capture mode configuration. FrameCap input mode configuration."]
pub mod capturemode;
#[doc = "CTR (rw) register accessor: an alias for `Reg<CTR_SPEC>`"]
pub type CTR = crate::Reg<ctr::CTR_SPEC>;
#[doc = "FrameCap control register for starting the unit"]
pub mod ctr;
#[doc = "SPR (rw) register accessor: an alias for `Reg<SPR_SPEC>`"]
pub type SPR = crate::Reg<spr::SPR_SPEC>;
#[doc = "FrameCap sync signal polarity configuration."]
pub mod spr;
#[doc = "MDR (rw) register accessor: an alias for `Reg<MDR_SPEC>`"]
pub type MDR = crate::Reg<mdr::MDR_SPEC>;
#[doc = "FrameCap control register for starting the mode detection"]
pub mod mdr;
#[doc = "FDR (rw) register accessor: an alias for `Reg<FDR_SPEC>`"]
pub type FDR = crate::Reg<fdr::FDR_SPEC>;
#[doc = "Frame dimension (progressive frame and field 0)."]
pub mod fdr;
#[doc = "FDR1 (rw) register accessor: an alias for `Reg<FDR1_SPEC>`"]
pub type FDR1 = crate::Reg<fdr1::FDR1_SPEC>;
#[doc = "Frame dimension (field 1)."]
pub mod fdr1;
#[doc = "KDR (rw) register accessor: an alias for `Reg<KDR_SPEC>`"]
pub type KDR = crate::Reg<kdr::KDR_SPEC>;
#[doc = "FrameCap kick delay configuration register"]
pub mod kdr;
#[doc = "SCR (rw) register accessor: an alias for `Reg<SCR_SPEC>`"]
pub type SCR = crate::Reg<scr::SCR_SPEC>;
#[doc = "FrameCap Sync frame number configuration register"]
pub mod scr;
#[doc = "CSVR (rw) register accessor: an alias for `Reg<CSVR_SPEC>`"]
pub type CSVR = crate::Reg<csvr::CSVR_SPEC>;
#[doc = "Framecap control register for capture clock supervision"]
pub mod csvr;
#[doc = "STSCLR (w) register accessor: an alias for `Reg<STSCLR_SPEC>`"]
pub type STSCLR = crate::Reg<stsclr::STSCLR_SPEC>;
#[doc = "FrameCap status clear register. Clears the status bits of Sts register, Mdr_SizeChange and Csv_lost field."]
pub mod stsclr;
#[doc = "STS (r) register accessor: an alias for `Reg<STS_SPEC>`"]
pub type STS = crate::Reg<sts::STS_SPEC>;
#[doc = "FrameCap status register. Current status of the FrameCap module."]
pub mod sts;
#[doc = "MDSTS0 (r) register accessor: an alias for `Reg<MDSTS0_SPEC>`"]
pub type MDSTS0 = crate::Reg<mdsts0::MDSTS0_SPEC>;
#[doc = "FrameCap mode detection status register for the active part of the video frames.(bit locked when MdrCmrDone=1)."]
pub mod mdsts0;
#[doc = "MDSTS1 (r) register accessor: an alias for `Reg<MDSTS1_SPEC>`"]
pub type MDSTS1 = crate::Reg<mdsts1::MDSTS1_SPEC>;
#[doc = "FrameCap mode detection status register. The current (x,y) position of the capture stream (with respect to the output)."]
pub mod mdsts1;
#[doc = "FRCNT (r) register accessor: an alias for `Reg<FRCNT_SPEC>`"]
pub type FRCNT = crate::Reg<frcnt::FRCNT_SPEC>;
#[doc = "FrameCap frame counter register indicating the corresponding clk_axi cycles."]
pub mod frcnt;
#[doc = "FRLINECOUNT (r) register accessor: an alias for `Reg<FRLINECOUNT_SPEC>`"]
pub type FRLINECOUNT = crate::Reg<frlinecount::FRLINECOUNT_SPEC>;
#[doc = "FrameCap line counter register."]
pub mod frlinecount;
#[doc = "CMSTS1 (r) register accessor: an alias for `Reg<CMSTS1_SPEC>`"]
pub type CMSTS1 = crate::Reg<cmsts1::CMSTS1_SPEC>;
#[doc = "clk_axi cycle number of a frame. (bit locked when MdrCmrDone=1)."]
pub mod cmsts1;
#[doc = "CMSTS2 (r) register accessor: an alias for `Reg<CMSTS2_SPEC>`"]
pub type CMSTS2 = crate::Reg<cmsts2::CMSTS2_SPEC>;
#[doc = "clk_axi cycle number of totwidth and actwidth of a frame. (bit locked when MdrCmrDone=1)."]
pub mod cmsts2;
#[doc = "ITUSTS (rw) register accessor: an alias for `Reg<ITUSTS_SPEC>`"]
pub type ITUSTS = crate::Reg<itusts::ITUSTS_SPEC>;
#[doc = "ITU error status register."]
pub mod itusts;
