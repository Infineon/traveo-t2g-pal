#[doc = r"Register block"]
#[repr(C)]
pub struct M_TTCAN {
    #[doc = "0x00 - Core Release Register"]
    pub crel: CREL,
    #[doc = "0x04 - Endian Register"]
    pub endn: ENDN,
    _reserved2: [u8; 0x04],
    #[doc = "0x0c - Data Bit Timing &amp; Prescaler Register"]
    pub dbtp: DBTP,
    #[doc = "0x10 - Test Register"]
    pub test: TEST,
    #[doc = "0x14 - RAM Watchdog"]
    pub rwd: RWD,
    #[doc = "0x18 - CC Control Register"]
    pub cccr: CCCR,
    #[doc = "0x1c - Nominal Bit Timing &amp; Prescaler Register"]
    pub nbtp: NBTP,
    #[doc = "0x20 - Timestamp Counter Configuration"]
    pub tscc: TSCC,
    #[doc = "0x24 - Timestamp Counter Value"]
    pub tscv: TSCV,
    #[doc = "0x28 - Timeout Counter Configuration"]
    pub tocc: TOCC,
    #[doc = "0x2c - Timeout Counter Value"]
    pub tocv: TOCV,
    _reserved11: [u8; 0x10],
    #[doc = "0x40 - Error Counter Register"]
    pub ecr: ECR,
    #[doc = "0x44 - Protocol Status Register"]
    pub psr: PSR,
    #[doc = "0x48 - Transmitter Delay Compensation Register"]
    pub tdcr: TDCR,
    _reserved14: [u8; 0x04],
    #[doc = "0x50 - Interrupt Register"]
    pub ir: IR,
    #[doc = "0x54 - Interrupt Enable"]
    pub ie: IE,
    #[doc = "0x58 - Interrupt Line Select"]
    pub ils: ILS,
    #[doc = "0x5c - Interrupt Line Enable"]
    pub ile: ILE,
    _reserved18: [u8; 0x20],
    #[doc = "0x80 - Global Filter Configuration"]
    pub gfc: GFC,
    #[doc = "0x84 - Standard ID Filter Configuration"]
    pub sidfc: SIDFC,
    #[doc = "0x88 - Extended ID Filter Configuration"]
    pub xidfc: XIDFC,
    _reserved21: [u8; 0x04],
    #[doc = "0x90 - Extended ID AND Mask"]
    pub xidam: XIDAM,
    #[doc = "0x94 - High Priority Message Status"]
    pub hpms: HPMS,
    #[doc = "0x98 - New Data 1"]
    pub ndat1: NDAT1,
    #[doc = "0x9c - New Data 2"]
    pub ndat2: NDAT2,
    #[doc = "0xa0 - Rx FIFO 0 Configuration"]
    pub rxf0c: RXF0C,
    #[doc = "0xa4 - Rx FIFO 0 Status"]
    pub rxf0s: RXF0S,
    #[doc = "0xa8 - Rx FIFO 0 Acknowledge"]
    pub rxf0a: RXF0A,
    #[doc = "0xac - Rx Buffer Configuration"]
    pub rxbc: RXBC,
    #[doc = "0xb0 - Rx FIFO 1 Configuration"]
    pub rxf1c: RXF1C,
    #[doc = "0xb4 - Rx FIFO 1 Status"]
    pub rxf1s: RXF1S,
    #[doc = "0xb8 - Rx FIFO 1 Acknowledge"]
    pub rxf1a: RXF1A,
    #[doc = "0xbc - Rx Buffer / FIFO Element Size Configuration"]
    pub rxesc: RXESC,
    #[doc = "0xc0 - Tx Buffer Configuration"]
    pub txbc: TXBC,
    #[doc = "0xc4 - Tx FIFO/Queue Status"]
    pub txfqs: TXFQS,
    #[doc = "0xc8 - Tx Buffer Element Size Configuration"]
    pub txesc: TXESC,
    #[doc = "0xcc - Tx Buffer Request Pending"]
    pub txbrp: TXBRP,
    #[doc = "0xd0 - Tx Buffer Add Request"]
    pub txbar: TXBAR,
    #[doc = "0xd4 - Tx Buffer Cancellation Request"]
    pub txbcr: TXBCR,
    #[doc = "0xd8 - Tx Buffer Transmission Occurred"]
    pub txbto: TXBTO,
    #[doc = "0xdc - Tx Buffer Cancellation Finished"]
    pub txbcf: TXBCF,
    #[doc = "0xe0 - Tx Buffer Transmission Interrupt Enable"]
    pub txbtie: TXBTIE,
    #[doc = "0xe4 - Tx Buffer Cancellation Finished Interrupt Enable"]
    pub txbcie: TXBCIE,
    _reserved43: [u8; 0x08],
    #[doc = "0xf0 - Tx Event FIFO Configuration"]
    pub txefc: TXEFC,
    #[doc = "0xf4 - Tx Event FIFO Status"]
    pub txefs: TXEFS,
    #[doc = "0xf8 - Tx Event FIFO Acknowledge"]
    pub txefa: TXEFA,
    _reserved46: [u8; 0x04],
    #[doc = "0x100 - TT Trigger Memory Configuration"]
    pub tttmc: TTTMC,
    #[doc = "0x104 - TT Reference Message Configuration"]
    pub ttrmc: TTRMC,
    #[doc = "0x108 - TT Operation Configuration"]
    pub ttocf: TTOCF,
    #[doc = "0x10c - TT Matrix Limits"]
    pub ttmlm: TTMLM,
    #[doc = "0x110 - TUR Configuration"]
    pub turcf: TURCF,
    #[doc = "0x114 - TT Operation Control"]
    pub ttocn: TTOCN,
    #[doc = "0x118 - TT Global Time Preset"]
    pub ttgtp: TTGTP,
    #[doc = "0x11c - TT Time Mark"]
    pub tttmk: TTTMK,
    #[doc = "0x120 - TT Interrupt Register"]
    pub ttir: TTIR,
    #[doc = "0x124 - TT Interrupt Enable"]
    pub ttie: TTIE,
    #[doc = "0x128 - TT Interrupt Line Select"]
    pub ttils: TTILS,
    #[doc = "0x12c - TT Operation Status"]
    pub ttost: TTOST,
    #[doc = "0x130 - TUR Numerator Actual"]
    pub turna: TURNA,
    #[doc = "0x134 - TT Local &amp; Global Time"]
    pub ttlgt: TTLGT,
    #[doc = "0x138 - TT Cycle Time &amp; Count"]
    pub ttctc: TTCTC,
    #[doc = "0x13c - TT Capture Time"]
    pub ttcpt: TTCPT,
    #[doc = "0x140 - TT Cycle Sync Mark"]
    pub ttcsm: TTCSM,
}
#[doc = "CREL (r) register accessor: an alias for `Reg<CREL_SPEC>`"]
pub type CREL = crate::Reg<crel::CREL_SPEC>;
#[doc = "Core Release Register"]
pub mod crel;
#[doc = "ENDN (r) register accessor: an alias for `Reg<ENDN_SPEC>`"]
pub type ENDN = crate::Reg<endn::ENDN_SPEC>;
#[doc = "Endian Register"]
pub mod endn;
#[doc = "DBTP (rw) register accessor: an alias for `Reg<DBTP_SPEC>`"]
pub type DBTP = crate::Reg<dbtp::DBTP_SPEC>;
#[doc = "Data Bit Timing &amp; Prescaler Register"]
pub mod dbtp;
#[doc = "TEST (rw) register accessor: an alias for `Reg<TEST_SPEC>`"]
pub type TEST = crate::Reg<test::TEST_SPEC>;
#[doc = "Test Register"]
pub mod test;
#[doc = "RWD (rw) register accessor: an alias for `Reg<RWD_SPEC>`"]
pub type RWD = crate::Reg<rwd::RWD_SPEC>;
#[doc = "RAM Watchdog"]
pub mod rwd;
#[doc = "CCCR (rw) register accessor: an alias for `Reg<CCCR_SPEC>`"]
pub type CCCR = crate::Reg<cccr::CCCR_SPEC>;
#[doc = "CC Control Register"]
pub mod cccr;
#[doc = "NBTP (rw) register accessor: an alias for `Reg<NBTP_SPEC>`"]
pub type NBTP = crate::Reg<nbtp::NBTP_SPEC>;
#[doc = "Nominal Bit Timing &amp; Prescaler Register"]
pub mod nbtp;
#[doc = "TSCC (rw) register accessor: an alias for `Reg<TSCC_SPEC>`"]
pub type TSCC = crate::Reg<tscc::TSCC_SPEC>;
#[doc = "Timestamp Counter Configuration"]
pub mod tscc;
#[doc = "TSCV (rw) register accessor: an alias for `Reg<TSCV_SPEC>`"]
pub type TSCV = crate::Reg<tscv::TSCV_SPEC>;
#[doc = "Timestamp Counter Value"]
pub mod tscv;
#[doc = "TOCC (rw) register accessor: an alias for `Reg<TOCC_SPEC>`"]
pub type TOCC = crate::Reg<tocc::TOCC_SPEC>;
#[doc = "Timeout Counter Configuration"]
pub mod tocc;
#[doc = "TOCV (rw) register accessor: an alias for `Reg<TOCV_SPEC>`"]
pub type TOCV = crate::Reg<tocv::TOCV_SPEC>;
#[doc = "Timeout Counter Value"]
pub mod tocv;
#[doc = "ECR (r) register accessor: an alias for `Reg<ECR_SPEC>`"]
pub type ECR = crate::Reg<ecr::ECR_SPEC>;
#[doc = "Error Counter Register"]
pub mod ecr;
#[doc = "PSR (r) register accessor: an alias for `Reg<PSR_SPEC>`"]
pub type PSR = crate::Reg<psr::PSR_SPEC>;
#[doc = "Protocol Status Register"]
pub mod psr;
#[doc = "TDCR (rw) register accessor: an alias for `Reg<TDCR_SPEC>`"]
pub type TDCR = crate::Reg<tdcr::TDCR_SPEC>;
#[doc = "Transmitter Delay Compensation Register"]
pub mod tdcr;
#[doc = "IR (rw) register accessor: an alias for `Reg<IR_SPEC>`"]
pub type IR = crate::Reg<ir::IR_SPEC>;
#[doc = "Interrupt Register"]
pub mod ir;
#[doc = "IE (rw) register accessor: an alias for `Reg<IE_SPEC>`"]
pub type IE = crate::Reg<ie::IE_SPEC>;
#[doc = "Interrupt Enable"]
pub mod ie;
#[doc = "ILS (rw) register accessor: an alias for `Reg<ILS_SPEC>`"]
pub type ILS = crate::Reg<ils::ILS_SPEC>;
#[doc = "Interrupt Line Select"]
pub mod ils;
#[doc = "ILE (rw) register accessor: an alias for `Reg<ILE_SPEC>`"]
pub type ILE = crate::Reg<ile::ILE_SPEC>;
#[doc = "Interrupt Line Enable"]
pub mod ile;
#[doc = "GFC (rw) register accessor: an alias for `Reg<GFC_SPEC>`"]
pub type GFC = crate::Reg<gfc::GFC_SPEC>;
#[doc = "Global Filter Configuration"]
pub mod gfc;
#[doc = "SIDFC (rw) register accessor: an alias for `Reg<SIDFC_SPEC>`"]
pub type SIDFC = crate::Reg<sidfc::SIDFC_SPEC>;
#[doc = "Standard ID Filter Configuration"]
pub mod sidfc;
#[doc = "XIDFC (rw) register accessor: an alias for `Reg<XIDFC_SPEC>`"]
pub type XIDFC = crate::Reg<xidfc::XIDFC_SPEC>;
#[doc = "Extended ID Filter Configuration"]
pub mod xidfc;
#[doc = "XIDAM (rw) register accessor: an alias for `Reg<XIDAM_SPEC>`"]
pub type XIDAM = crate::Reg<xidam::XIDAM_SPEC>;
#[doc = "Extended ID AND Mask"]
pub mod xidam;
#[doc = "HPMS (r) register accessor: an alias for `Reg<HPMS_SPEC>`"]
pub type HPMS = crate::Reg<hpms::HPMS_SPEC>;
#[doc = "High Priority Message Status"]
pub mod hpms;
#[doc = "NDAT1 (rw) register accessor: an alias for `Reg<NDAT1_SPEC>`"]
pub type NDAT1 = crate::Reg<ndat1::NDAT1_SPEC>;
#[doc = "New Data 1"]
pub mod ndat1;
#[doc = "NDAT2 (rw) register accessor: an alias for `Reg<NDAT2_SPEC>`"]
pub type NDAT2 = crate::Reg<ndat2::NDAT2_SPEC>;
#[doc = "New Data 2"]
pub mod ndat2;
#[doc = "RXF0C (rw) register accessor: an alias for `Reg<RXF0C_SPEC>`"]
pub type RXF0C = crate::Reg<rxf0c::RXF0C_SPEC>;
#[doc = "Rx FIFO 0 Configuration"]
pub mod rxf0c;
#[doc = "RXF0S (r) register accessor: an alias for `Reg<RXF0S_SPEC>`"]
pub type RXF0S = crate::Reg<rxf0s::RXF0S_SPEC>;
#[doc = "Rx FIFO 0 Status"]
pub mod rxf0s;
#[doc = "RXF0A (rw) register accessor: an alias for `Reg<RXF0A_SPEC>`"]
pub type RXF0A = crate::Reg<rxf0a::RXF0A_SPEC>;
#[doc = "Rx FIFO 0 Acknowledge"]
pub mod rxf0a;
#[doc = "RXBC (rw) register accessor: an alias for `Reg<RXBC_SPEC>`"]
pub type RXBC = crate::Reg<rxbc::RXBC_SPEC>;
#[doc = "Rx Buffer Configuration"]
pub mod rxbc;
#[doc = "RXF1C (rw) register accessor: an alias for `Reg<RXF1C_SPEC>`"]
pub type RXF1C = crate::Reg<rxf1c::RXF1C_SPEC>;
#[doc = "Rx FIFO 1 Configuration"]
pub mod rxf1c;
#[doc = "RXF1S (r) register accessor: an alias for `Reg<RXF1S_SPEC>`"]
pub type RXF1S = crate::Reg<rxf1s::RXF1S_SPEC>;
#[doc = "Rx FIFO 1 Status"]
pub mod rxf1s;
#[doc = "RXF1A (rw) register accessor: an alias for `Reg<RXF1A_SPEC>`"]
pub type RXF1A = crate::Reg<rxf1a::RXF1A_SPEC>;
#[doc = "Rx FIFO 1 Acknowledge"]
pub mod rxf1a;
#[doc = "RXESC (rw) register accessor: an alias for `Reg<RXESC_SPEC>`"]
pub type RXESC = crate::Reg<rxesc::RXESC_SPEC>;
#[doc = "Rx Buffer / FIFO Element Size Configuration"]
pub mod rxesc;
#[doc = "TXBC (rw) register accessor: an alias for `Reg<TXBC_SPEC>`"]
pub type TXBC = crate::Reg<txbc::TXBC_SPEC>;
#[doc = "Tx Buffer Configuration"]
pub mod txbc;
#[doc = "TXFQS (r) register accessor: an alias for `Reg<TXFQS_SPEC>`"]
pub type TXFQS = crate::Reg<txfqs::TXFQS_SPEC>;
#[doc = "Tx FIFO/Queue Status"]
pub mod txfqs;
#[doc = "TXESC (rw) register accessor: an alias for `Reg<TXESC_SPEC>`"]
pub type TXESC = crate::Reg<txesc::TXESC_SPEC>;
#[doc = "Tx Buffer Element Size Configuration"]
pub mod txesc;
#[doc = "TXBRP (r) register accessor: an alias for `Reg<TXBRP_SPEC>`"]
pub type TXBRP = crate::Reg<txbrp::TXBRP_SPEC>;
#[doc = "Tx Buffer Request Pending"]
pub mod txbrp;
#[doc = "TXBAR (rw) register accessor: an alias for `Reg<TXBAR_SPEC>`"]
pub type TXBAR = crate::Reg<txbar::TXBAR_SPEC>;
#[doc = "Tx Buffer Add Request"]
pub mod txbar;
#[doc = "TXBCR (rw) register accessor: an alias for `Reg<TXBCR_SPEC>`"]
pub type TXBCR = crate::Reg<txbcr::TXBCR_SPEC>;
#[doc = "Tx Buffer Cancellation Request"]
pub mod txbcr;
#[doc = "TXBTO (r) register accessor: an alias for `Reg<TXBTO_SPEC>`"]
pub type TXBTO = crate::Reg<txbto::TXBTO_SPEC>;
#[doc = "Tx Buffer Transmission Occurred"]
pub mod txbto;
#[doc = "TXBCF (r) register accessor: an alias for `Reg<TXBCF_SPEC>`"]
pub type TXBCF = crate::Reg<txbcf::TXBCF_SPEC>;
#[doc = "Tx Buffer Cancellation Finished"]
pub mod txbcf;
#[doc = "TXBTIE (rw) register accessor: an alias for `Reg<TXBTIE_SPEC>`"]
pub type TXBTIE = crate::Reg<txbtie::TXBTIE_SPEC>;
#[doc = "Tx Buffer Transmission Interrupt Enable"]
pub mod txbtie;
#[doc = "TXBCIE (rw) register accessor: an alias for `Reg<TXBCIE_SPEC>`"]
pub type TXBCIE = crate::Reg<txbcie::TXBCIE_SPEC>;
#[doc = "Tx Buffer Cancellation Finished Interrupt Enable"]
pub mod txbcie;
#[doc = "TXEFC (rw) register accessor: an alias for `Reg<TXEFC_SPEC>`"]
pub type TXEFC = crate::Reg<txefc::TXEFC_SPEC>;
#[doc = "Tx Event FIFO Configuration"]
pub mod txefc;
#[doc = "TXEFS (r) register accessor: an alias for `Reg<TXEFS_SPEC>`"]
pub type TXEFS = crate::Reg<txefs::TXEFS_SPEC>;
#[doc = "Tx Event FIFO Status"]
pub mod txefs;
#[doc = "TXEFA (rw) register accessor: an alias for `Reg<TXEFA_SPEC>`"]
pub type TXEFA = crate::Reg<txefa::TXEFA_SPEC>;
#[doc = "Tx Event FIFO Acknowledge"]
pub mod txefa;
#[doc = "TTTMC (rw) register accessor: an alias for `Reg<TTTMC_SPEC>`"]
pub type TTTMC = crate::Reg<tttmc::TTTMC_SPEC>;
#[doc = "TT Trigger Memory Configuration"]
pub mod tttmc;
#[doc = "TTRMC (rw) register accessor: an alias for `Reg<TTRMC_SPEC>`"]
pub type TTRMC = crate::Reg<ttrmc::TTRMC_SPEC>;
#[doc = "TT Reference Message Configuration"]
pub mod ttrmc;
#[doc = "TTOCF (rw) register accessor: an alias for `Reg<TTOCF_SPEC>`"]
pub type TTOCF = crate::Reg<ttocf::TTOCF_SPEC>;
#[doc = "TT Operation Configuration"]
pub mod ttocf;
#[doc = "TTMLM (rw) register accessor: an alias for `Reg<TTMLM_SPEC>`"]
pub type TTMLM = crate::Reg<ttmlm::TTMLM_SPEC>;
#[doc = "TT Matrix Limits"]
pub mod ttmlm;
#[doc = "TURCF (rw) register accessor: an alias for `Reg<TURCF_SPEC>`"]
pub type TURCF = crate::Reg<turcf::TURCF_SPEC>;
#[doc = "TUR Configuration"]
pub mod turcf;
#[doc = "TTOCN (rw) register accessor: an alias for `Reg<TTOCN_SPEC>`"]
pub type TTOCN = crate::Reg<ttocn::TTOCN_SPEC>;
#[doc = "TT Operation Control"]
pub mod ttocn;
#[doc = "TTGTP (rw) register accessor: an alias for `Reg<TTGTP_SPEC>`"]
pub type TTGTP = crate::Reg<ttgtp::TTGTP_SPEC>;
#[doc = "TT Global Time Preset"]
pub mod ttgtp;
#[doc = "TTTMK (rw) register accessor: an alias for `Reg<TTTMK_SPEC>`"]
pub type TTTMK = crate::Reg<tttmk::TTTMK_SPEC>;
#[doc = "TT Time Mark"]
pub mod tttmk;
#[doc = "TTIR (rw) register accessor: an alias for `Reg<TTIR_SPEC>`"]
pub type TTIR = crate::Reg<ttir::TTIR_SPEC>;
#[doc = "TT Interrupt Register"]
pub mod ttir;
#[doc = "TTIE (rw) register accessor: an alias for `Reg<TTIE_SPEC>`"]
pub type TTIE = crate::Reg<ttie::TTIE_SPEC>;
#[doc = "TT Interrupt Enable"]
pub mod ttie;
#[doc = "TTILS (rw) register accessor: an alias for `Reg<TTILS_SPEC>`"]
pub type TTILS = crate::Reg<ttils::TTILS_SPEC>;
#[doc = "TT Interrupt Line Select"]
pub mod ttils;
#[doc = "TTOST (r) register accessor: an alias for `Reg<TTOST_SPEC>`"]
pub type TTOST = crate::Reg<ttost::TTOST_SPEC>;
#[doc = "TT Operation Status"]
pub mod ttost;
#[doc = "TURNA (r) register accessor: an alias for `Reg<TURNA_SPEC>`"]
pub type TURNA = crate::Reg<turna::TURNA_SPEC>;
#[doc = "TUR Numerator Actual"]
pub mod turna;
#[doc = "TTLGT (r) register accessor: an alias for `Reg<TTLGT_SPEC>`"]
pub type TTLGT = crate::Reg<ttlgt::TTLGT_SPEC>;
#[doc = "TT Local &amp; Global Time"]
pub mod ttlgt;
#[doc = "TTCTC (r) register accessor: an alias for `Reg<TTCTC_SPEC>`"]
pub type TTCTC = crate::Reg<ttctc::TTCTC_SPEC>;
#[doc = "TT Cycle Time &amp; Count"]
pub mod ttctc;
#[doc = "TTCPT (r) register accessor: an alias for `Reg<TTCPT_SPEC>`"]
pub type TTCPT = crate::Reg<ttcpt::TTCPT_SPEC>;
#[doc = "TT Capture Time"]
pub mod ttcpt;
#[doc = "TTCSM (r) register accessor: an alias for `Reg<TTCSM_SPEC>`"]
pub type TTCSM = crate::Reg<ttcsm::TTCSM_SPEC>;
#[doc = "TT Cycle Sync Mark"]
pub mod ttcsm;
