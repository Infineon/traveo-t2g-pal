#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00..0x1bc - FIFO wrapper around M_TTCAN 3PIP, to enable DMA"]
    pub ch0: CH,
    _reserved1: [u8; 0x44],
    #[doc = "0x200..0x3bc - FIFO wrapper around M_TTCAN 3PIP, to enable DMA"]
    pub ch1: CH,
    _reserved2: [u8; 0x44],
    #[doc = "0x400..0x5bc - FIFO wrapper around M_TTCAN 3PIP, to enable DMA"]
    pub ch2: CH,
    _reserved3: [u8; 0x0a44],
    #[doc = "0x1000 - Global CAN control register"]
    pub ctl: CTL,
    #[doc = "0x1004 - Global CAN status register"]
    pub status: STATUS,
    _reserved5: [u8; 0x08],
    #[doc = "0x1010 - Consolidated interrupt0 cause register"]
    pub intr0_cause: INTR0_CAUSE,
    #[doc = "0x1014 - Consolidated interrupt1 cause register"]
    pub intr1_cause: INTR1_CAUSE,
    _reserved7: [u8; 0x08],
    #[doc = "0x1020 - Time Stamp control register"]
    pub ts_ctl: TS_CTL,
    #[doc = "0x1024 - Time Stamp counter value"]
    pub ts_cnt: TS_CNT,
    _reserved9: [u8; 0x58],
    #[doc = "0x1080 - ECC control"]
    pub ecc_ctl: ECC_CTL,
    #[doc = "0x1084 - ECC error injection"]
    pub ecc_err_inj: ECC_ERR_INJ,
}
#[doc = "FIFO wrapper around M_TTCAN 3PIP, to enable DMA"]
pub use self::ch::CH;
#[doc = r"Cluster"]
#[doc = "FIFO wrapper around M_TTCAN 3PIP, to enable DMA"]
pub mod ch;
#[doc = "CTL (rw) register accessor: an alias for `Reg<CTL_SPEC>`"]
pub type CTL = crate::Reg<ctl::CTL_SPEC>;
#[doc = "Global CAN control register"]
pub mod ctl;
#[doc = "STATUS (r) register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Global CAN status register"]
pub mod status;
#[doc = "INTR0_CAUSE (r) register accessor: an alias for `Reg<INTR0_CAUSE_SPEC>`"]
pub type INTR0_CAUSE = crate::Reg<intr0_cause::INTR0_CAUSE_SPEC>;
#[doc = "Consolidated interrupt0 cause register"]
pub mod intr0_cause;
#[doc = "INTR1_CAUSE (r) register accessor: an alias for `Reg<INTR1_CAUSE_SPEC>`"]
pub type INTR1_CAUSE = crate::Reg<intr1_cause::INTR1_CAUSE_SPEC>;
#[doc = "Consolidated interrupt1 cause register"]
pub mod intr1_cause;
#[doc = "TS_CTL (rw) register accessor: an alias for `Reg<TS_CTL_SPEC>`"]
pub type TS_CTL = crate::Reg<ts_ctl::TS_CTL_SPEC>;
#[doc = "Time Stamp control register"]
pub mod ts_ctl;
#[doc = "TS_CNT (rw) register accessor: an alias for `Reg<TS_CNT_SPEC>`"]
pub type TS_CNT = crate::Reg<ts_cnt::TS_CNT_SPEC>;
#[doc = "Time Stamp counter value"]
pub mod ts_cnt;
#[doc = "ECC_CTL (rw) register accessor: an alias for `Reg<ECC_CTL_SPEC>`"]
pub type ECC_CTL = crate::Reg<ecc_ctl::ECC_CTL_SPEC>;
#[doc = "ECC control"]
pub mod ecc_ctl;
#[doc = "ECC_ERR_INJ (rw) register accessor: an alias for `Reg<ECC_ERR_INJ_SPEC>`"]
pub type ECC_ERR_INJ = crate::Reg<ecc_err_inj::ECC_ERR_INJ_SPEC>;
#[doc = "ECC error injection"]
pub mod ecc_err_inj;
