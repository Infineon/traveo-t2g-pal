#[doc = r"Register block"]
#[repr(C)]
pub struct MIPICSI_WRAP {
    #[doc = "0x00 - Control"]
    pub ctl: CTL,
    #[doc = "0x04 - D-PHY Control"]
    pub dphy_ctl: DPHY_CTL,
    #[doc = "0x08 - Clock Control"]
    pub clock_ctl: CLOCK_CTL,
    _reserved3: [u8; 0x34],
    #[doc = "0x40 - Calibration Control"]
    pub cal_ctl: CAL_CTL,
    #[doc = "0x44 - Calibration Status"]
    pub cal_stat: CAL_STAT,
    _reserved5: [u8; 0x78],
    #[doc = "0xc0 - Timing Data Lane Control"]
    pub tmg_data_ctl: TMG_DATA_CTL,
    #[doc = "0xc4 - Timing Clock Lane Control"]
    pub tmg_clock_ctl: TMG_CLOCK_CTL,
}
#[doc = "CTL (rw) register accessor: an alias for `Reg<CTL_SPEC>`"]
pub type CTL = crate::Reg<ctl::CTL_SPEC>;
#[doc = "Control"]
pub mod ctl;
#[doc = "DPHY_CTL (rw) register accessor: an alias for `Reg<DPHY_CTL_SPEC>`"]
pub type DPHY_CTL = crate::Reg<dphy_ctl::DPHY_CTL_SPEC>;
#[doc = "D-PHY Control"]
pub mod dphy_ctl;
#[doc = "CLOCK_CTL (rw) register accessor: an alias for `Reg<CLOCK_CTL_SPEC>`"]
pub type CLOCK_CTL = crate::Reg<clock_ctl::CLOCK_CTL_SPEC>;
#[doc = "Clock Control"]
pub mod clock_ctl;
#[doc = "CAL_CTL (rw) register accessor: an alias for `Reg<CAL_CTL_SPEC>`"]
pub type CAL_CTL = crate::Reg<cal_ctl::CAL_CTL_SPEC>;
#[doc = "Calibration Control"]
pub mod cal_ctl;
#[doc = "CAL_STAT (r) register accessor: an alias for `Reg<CAL_STAT_SPEC>`"]
pub type CAL_STAT = crate::Reg<cal_stat::CAL_STAT_SPEC>;
#[doc = "Calibration Status"]
pub mod cal_stat;
#[doc = "TMG_DATA_CTL (rw) register accessor: an alias for `Reg<TMG_DATA_CTL_SPEC>`"]
pub type TMG_DATA_CTL = crate::Reg<tmg_data_ctl::TMG_DATA_CTL_SPEC>;
#[doc = "Timing Data Lane Control"]
pub mod tmg_data_ctl;
#[doc = "TMG_CLOCK_CTL (rw) register accessor: an alias for `Reg<TMG_CLOCK_CTL_SPEC>`"]
pub type TMG_CLOCK_CTL = crate::Reg<tmg_clock_ctl::TMG_CLOCK_CTL_SPEC>;
#[doc = "Timing Clock Lane Control"]
pub mod tmg_clock_ctl;
