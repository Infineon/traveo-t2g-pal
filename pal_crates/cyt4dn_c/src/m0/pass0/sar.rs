#[doc = r"Register block"]
#[repr(C)]
pub struct SAR {
    #[doc = "0x00 - Analog control register."]
    pub ctl: CTL,
    #[doc = "0x04 - Diagnostic Reference control register."]
    pub diag_ctl: DIAG_CTL,
    _reserved2: [u8; 0x08],
    #[doc = "0x10 - Preconditioning control register."]
    pub precond_ctl: PRECOND_CTL,
    _reserved3: [u8; 0x6c],
    #[doc = "0x80 - Current analog calibration values"]
    pub ana_cal: ANA_CAL,
    #[doc = "0x84 - Current digital calibration values"]
    pub dig_cal: DIG_CAL,
    _reserved5: [u8; 0x08],
    #[doc = "0x90 - Alternate analog calibration values"]
    pub ana_cal_alt: ANA_CAL_ALT,
    #[doc = "0x94 - Alternate digital calibration values"]
    pub dig_cal_alt: DIG_CAL_ALT,
    #[doc = "0x98 - Calibration update command"]
    pub cal_upd_cmd: CAL_UPD_CMD,
    _reserved8: [u8; 0x64],
    #[doc = "0x100 - Trigger pending status"]
    pub tr_pend: TR_PEND,
    _reserved9: [u8; 0x7c],
    #[doc = "0x180 - Channel working data register 'valid' bits"]
    pub work_valid: WORK_VALID,
    #[doc = "0x184 - Range detected"]
    pub work_range: WORK_RANGE,
    #[doc = "0x188 - Range detect above Hi flag"]
    pub work_range_hi: WORK_RANGE_HI,
    #[doc = "0x18c - Pulse detected"]
    pub work_pulse: WORK_PULSE,
    _reserved13: [u8; 0x10],
    #[doc = "0x1a0 - Channel result data register 'valid' bits"]
    pub result_valid: RESULT_VALID,
    #[doc = "0x1a4 - Channel Range above Hi flags"]
    pub result_range_hi: RESULT_RANGE_HI,
    _reserved15: [u8; 0x58],
    #[doc = "0x200 - Current status of internal SAR registers (mostly for debug)"]
    pub status: STATUS,
    #[doc = "0x204 - Current averaging status (for debug)"]
    pub avg_stat: AVG_STAT,
    _reserved17: [u8; 0x05f8],
    #[doc = "0x800..0x1000 - Channel structure"]
    pub ch: [CH; 32],
}
#[doc = "CTL (rw) register accessor: an alias for `Reg<CTL_SPEC>`"]
pub type CTL = crate::Reg<ctl::CTL_SPEC>;
#[doc = "Analog control register."]
pub mod ctl;
#[doc = "DIAG_CTL (rw) register accessor: an alias for `Reg<DIAG_CTL_SPEC>`"]
pub type DIAG_CTL = crate::Reg<diag_ctl::DIAG_CTL_SPEC>;
#[doc = "Diagnostic Reference control register."]
pub mod diag_ctl;
#[doc = "PRECOND_CTL (rw) register accessor: an alias for `Reg<PRECOND_CTL_SPEC>`"]
pub type PRECOND_CTL = crate::Reg<precond_ctl::PRECOND_CTL_SPEC>;
#[doc = "Preconditioning control register."]
pub mod precond_ctl;
#[doc = "ANA_CAL (rw) register accessor: an alias for `Reg<ANA_CAL_SPEC>`"]
pub type ANA_CAL = crate::Reg<ana_cal::ANA_CAL_SPEC>;
#[doc = "Current analog calibration values"]
pub mod ana_cal;
#[doc = "DIG_CAL (rw) register accessor: an alias for `Reg<DIG_CAL_SPEC>`"]
pub type DIG_CAL = crate::Reg<dig_cal::DIG_CAL_SPEC>;
#[doc = "Current digital calibration values"]
pub mod dig_cal;
#[doc = "ANA_CAL_ALT (rw) register accessor: an alias for `Reg<ANA_CAL_ALT_SPEC>`"]
pub type ANA_CAL_ALT = crate::Reg<ana_cal_alt::ANA_CAL_ALT_SPEC>;
#[doc = "Alternate analog calibration values"]
pub mod ana_cal_alt;
#[doc = "DIG_CAL_ALT (rw) register accessor: an alias for `Reg<DIG_CAL_ALT_SPEC>`"]
pub type DIG_CAL_ALT = crate::Reg<dig_cal_alt::DIG_CAL_ALT_SPEC>;
#[doc = "Alternate digital calibration values"]
pub mod dig_cal_alt;
#[doc = "CAL_UPD_CMD (rw) register accessor: an alias for `Reg<CAL_UPD_CMD_SPEC>`"]
pub type CAL_UPD_CMD = crate::Reg<cal_upd_cmd::CAL_UPD_CMD_SPEC>;
#[doc = "Calibration update command"]
pub mod cal_upd_cmd;
#[doc = "TR_PEND (r) register accessor: an alias for `Reg<TR_PEND_SPEC>`"]
pub type TR_PEND = crate::Reg<tr_pend::TR_PEND_SPEC>;
#[doc = "Trigger pending status"]
pub mod tr_pend;
#[doc = "WORK_VALID (r) register accessor: an alias for `Reg<WORK_VALID_SPEC>`"]
pub type WORK_VALID = crate::Reg<work_valid::WORK_VALID_SPEC>;
#[doc = "Channel working data register 'valid' bits"]
pub mod work_valid;
#[doc = "WORK_RANGE (r) register accessor: an alias for `Reg<WORK_RANGE_SPEC>`"]
pub type WORK_RANGE = crate::Reg<work_range::WORK_RANGE_SPEC>;
#[doc = "Range detected"]
pub mod work_range;
#[doc = "WORK_RANGE_HI (r) register accessor: an alias for `Reg<WORK_RANGE_HI_SPEC>`"]
pub type WORK_RANGE_HI = crate::Reg<work_range_hi::WORK_RANGE_HI_SPEC>;
#[doc = "Range detect above Hi flag"]
pub mod work_range_hi;
#[doc = "WORK_PULSE (r) register accessor: an alias for `Reg<WORK_PULSE_SPEC>`"]
pub type WORK_PULSE = crate::Reg<work_pulse::WORK_PULSE_SPEC>;
#[doc = "Pulse detected"]
pub mod work_pulse;
#[doc = "RESULT_VALID (r) register accessor: an alias for `Reg<RESULT_VALID_SPEC>`"]
pub type RESULT_VALID = crate::Reg<result_valid::RESULT_VALID_SPEC>;
#[doc = "Channel result data register 'valid' bits"]
pub mod result_valid;
#[doc = "RESULT_RANGE_HI (r) register accessor: an alias for `Reg<RESULT_RANGE_HI_SPEC>`"]
pub type RESULT_RANGE_HI = crate::Reg<result_range_hi::RESULT_RANGE_HI_SPEC>;
#[doc = "Channel Range above Hi flags"]
pub mod result_range_hi;
#[doc = "STATUS (r) register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Current status of internal SAR registers (mostly for debug)"]
pub mod status;
#[doc = "AVG_STAT (r) register accessor: an alias for `Reg<AVG_STAT_SPEC>`"]
pub type AVG_STAT = crate::Reg<avg_stat::AVG_STAT_SPEC>;
#[doc = "Current averaging status (for debug)"]
pub mod avg_stat;
#[doc = "Channel structure"]
pub use self::ch::CH;
#[doc = r"Cluster"]
#[doc = "Channel structure"]
pub mod ch;
