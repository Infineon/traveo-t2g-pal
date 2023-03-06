#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x40],
    #[doc = "0x40 - High Voltage / Low Voltage Detector (HVLVD) Status Register"]
    pub pwr_lvd_status: PWR_LVD_STATUS,
    #[doc = "0x44 - High Voltage / Low Voltage Detector (HVLVD) Status Register #2"]
    pub pwr_lvd_status2: PWR_LVD_STATUS2,
    _reserved2: [u8; 0xb8],
    #[doc = "0x100..0x140 - Clock DSI Select Register"]
    pub clk_dsi_select: [CLK_DSI_SELECT; 16],
    #[doc = "0x140 - Fast Clock Output Select Register"]
    pub clk_output_fast: CLK_OUTPUT_FAST,
    #[doc = "0x144 - Slow Clock Output Select Register"]
    pub clk_output_slow: CLK_OUTPUT_SLOW,
    #[doc = "0x148 - Clock Calibration Counter 1"]
    pub clk_cal_cnt1: CLK_CAL_CNT1,
    #[doc = "0x14c - Clock Calibration Counter 2"]
    pub clk_cal_cnt2: CLK_CAL_CNT2,
    _reserved7: [u8; 0xb0],
    #[doc = "0x200 - SRSS Interrupt Register"]
    pub srss_intr: SRSS_INTR,
    #[doc = "0x204 - SRSS Interrupt Set Register"]
    pub srss_intr_set: SRSS_INTR_SET,
    #[doc = "0x208 - SRSS Interrupt Mask Register"]
    pub srss_intr_mask: SRSS_INTR_MASK,
    #[doc = "0x20c - SRSS Interrupt Masked Register"]
    pub srss_intr_masked: SRSS_INTR_MASKED,
    _reserved11: [u8; 0x0df0],
    #[doc = "0x1000 - Power Mode Control"]
    pub pwr_ctl: PWR_CTL,
    #[doc = "0x1004 - Power Mode Control 2"]
    pub pwr_ctl2: PWR_CTL2,
    #[doc = "0x1008 - HIBERNATE Mode Register"]
    pub pwr_hibernate: PWR_HIBERNATE,
    _reserved14: [u8; 0x04],
    #[doc = "0x1010 - Buck Control Register"]
    pub pwr_buck_ctl: PWR_BUCK_CTL,
    #[doc = "0x1014 - Buck Control Register 2"]
    pub pwr_buck_ctl2: PWR_BUCK_CTL2,
    #[doc = "0x1018 - Supply Supervision Control Register"]
    pub pwr_ssv_ctl: PWR_SSV_CTL,
    #[doc = "0x101c - Supply Supervision Status Register"]
    pub pwr_ssv_status: PWR_SSV_STATUS,
    #[doc = "0x1020 - High Voltage / Low Voltage Detector (HVLVD) Configuration Register"]
    pub pwr_lvd_ctl: PWR_LVD_CTL,
    #[doc = "0x1024 - High Voltage / Low Voltage Detector (HVLVD) Configuration Register #2"]
    pub pwr_lvd_ctl2: PWR_LVD_CTL2,
    #[doc = "0x1028 - REGHC Control Register"]
    pub pwr_reghc_ctl: PWR_REGHC_CTL,
    #[doc = "0x102c - REGHC Status Register"]
    pub pwr_reghc_status: PWR_REGHC_STATUS,
    #[doc = "0x1030 - REGHC Control Register 2"]
    pub pwr_reghc_ctl2: PWR_REGHC_CTL2,
    _reserved23: [u8; 0x04],
    #[doc = "0x1038 - REGHC Control Register 4"]
    pub pwr_reghc_ctl4: PWR_REGHC_CTL4,
    _reserved24: [u8; 0x04],
    #[doc = "0x1040..0x1080 - HIBERNATE Data Register"]
    pub pwr_hib_data: [PWR_HIB_DATA; 16],
    _reserved25: [u8; 0x40],
    #[doc = "0x10c0 - PMIC Control Register"]
    pub pwr_pmic_ctl: PWR_PMIC_CTL,
    #[doc = "0x10c4 - PMIC Status Register"]
    pub pwr_pmic_status: PWR_PMIC_STATUS,
    #[doc = "0x10c8 - PMIC Control Register 2"]
    pub pwr_pmic_ctl2: PWR_PMIC_CTL2,
    _reserved28: [u8; 0x04],
    #[doc = "0x10d0 - PMIC Control Register 4"]
    pub pwr_pmic_ctl4: PWR_PMIC_CTL4,
    _reserved29: [u8; 0x012c],
    #[doc = "0x1200..0x1240 - Clock Path Select Register"]
    pub clk_path_select: [CLK_PATH_SELECT; 16],
    #[doc = "0x1240..0x1280 - Clock Root Select Register"]
    pub clk_root_select: [CLK_ROOT_SELECT; 16],
    _reserved31: [u8; 0x0180],
    #[doc = "0x1400..0x142c - Clock Supervisor (CSV) registers for Root clocks"]
    pub csv_hf: CSV_HF,
    _reserved32: [u8; 0xd4],
    #[doc = "0x1500 - Clock selection register"]
    pub clk_select: CLK_SELECT,
    #[doc = "0x1504 - Timer Clock Control Register"]
    pub clk_timer_ctl: CLK_TIMER_CTL,
    #[doc = "0x1508 - ILO0 Configuration"]
    pub clk_ilo0_config: CLK_ILO0_CONFIG,
    #[doc = "0x150c - ILO1 Configuration"]
    pub clk_ilo1_config: CLK_ILO1_CONFIG,
    _reserved36: [u8; 0x08],
    #[doc = "0x1518 - IMO Configuration"]
    pub clk_imo_config: CLK_IMO_CONFIG,
    #[doc = "0x151c - ECO Configuration Register"]
    pub clk_eco_config: CLK_ECO_CONFIG,
    #[doc = "0x1520 - ECO Prescaler Configuration Register"]
    pub clk_eco_prescale: CLK_ECO_PRESCALE,
    #[doc = "0x1524 - ECO Status Register"]
    pub clk_eco_status: CLK_ECO_STATUS,
    #[doc = "0x1528 - Precision ILO Configuration Register"]
    pub clk_pilo_config: CLK_PILO_CONFIG,
    _reserved41: [u8; 0x04],
    #[doc = "0x1530 - FLL Configuration Register"]
    pub clk_fll_config: CLK_FLL_CONFIG,
    #[doc = "0x1534 - FLL Configuration Register 2"]
    pub clk_fll_config2: CLK_FLL_CONFIG2,
    #[doc = "0x1538 - FLL Configuration Register 3"]
    pub clk_fll_config3: CLK_FLL_CONFIG3,
    #[doc = "0x153c - FLL Configuration Register 4"]
    pub clk_fll_config4: CLK_FLL_CONFIG4,
    #[doc = "0x1540 - FLL Status Register"]
    pub clk_fll_status: CLK_FLL_STATUS,
    #[doc = "0x1544 - ECO Configuration Register 2"]
    pub clk_eco_config2: CLK_ECO_CONFIG2,
    _reserved47: [u8; 0xb8],
    #[doc = "0x1600..0x163c - PLL Configuration Register"]
    pub clk_pll_config: [CLK_PLL_CONFIG; 15],
    _reserved48: [u8; 0x04],
    #[doc = "0x1640..0x167c - PLL Status Register"]
    pub clk_pll_status: [CLK_PLL_STATUS; 15],
    _reserved49: [u8; 0x84],
    #[doc = "0x1700 - Select CSV Reference clock for Active domain"]
    pub csv_ref_sel: CSV_REF_SEL,
    _reserved50: [u8; 0x0c],
    #[doc = "0x1710..0x171c - CSV registers for the CSV Reference clock"]
    pub csv_ref: CSV_REF,
    _reserved51: [u8; 0x04],
    #[doc = "0x1720..0x172c - CSV registers for LF clock"]
    pub csv_lf: CSV_LF,
    _reserved52: [u8; 0x04],
    #[doc = "0x1730..0x173c - CSV registers for HVILO clock"]
    pub csv_ilo: CSV_ILO,
    _reserved53: [u8; 0xc4],
    #[doc = "0x1800 - Reset Cause Observation Register"]
    pub res_cause: RES_CAUSE,
    #[doc = "0x1804 - Reset Cause Observation Register 2"]
    pub res_cause2: RES_CAUSE2,
    _reserved55: [u8; 0xf8],
    #[doc = "0x1900..0x19f0 - 400MHz PLL Configuration Register"]
    pub clk_pll400m: [CLK_PLL400M; 15],
    _reserved56: [u8; 0x0664],
    #[doc = "0x2054 - SECURE TEST and FIRMWARE TEST Key control register"]
    pub tst_xres_secure: TST_XRES_SECURE,
    _reserved57: [u8; 0x24],
    #[doc = "0x207c - Programmable XRES Control Register"]
    pub res_pxres_ctl: RES_PXRES_CTL,
    _reserved58: [u8; 0x0f88],
    #[doc = "0x3008 - Wakeup Trim Register"]
    pub pwr_trim_wake_ctl: PWR_TRIM_WAKE_CTL,
    _reserved59: [u8; 0x08],
    #[doc = "0x3014 - ILO0 Trim Register"]
    pub clk_trim_ilo0_ctl: CLK_TRIM_ILO0_CTL,
    _reserved60: [u8; 0xf0],
    #[doc = "0x3108 - Power System Trim Register"]
    pub pwr_trim_pwrsys_ctl: PWR_TRIM_PWRSYS_CTL,
    _reserved61: [u8; 0x08],
    #[doc = "0x3114 - PILO Trim Register"]
    pub clk_trim_pilo_ctl: CLK_TRIM_PILO_CTL,
    #[doc = "0x3118 - PILO Trim Register 2"]
    pub clk_trim_pilo_ctl2: CLK_TRIM_PILO_CTL2,
    #[doc = "0x311c - PILO Trim Register 3"]
    pub clk_trim_pilo_ctl3: CLK_TRIM_PILO_CTL3,
    _reserved64: [u8; 0x0100],
    #[doc = "0x3220 - ILO1 Trim Register"]
    pub clk_trim_ilo1_ctl: CLK_TRIM_ILO1_CTL,
    _reserved65: [u8; 0x4ddc],
    #[doc = "0x8000..0x80b0 - Multi-Counter Watchdog Timer"]
    pub mcwdt0: MCWDT,
    _reserved66: [u8; 0x50],
    #[doc = "0x8100..0x81b0 - Multi-Counter Watchdog Timer"]
    pub mcwdt1: MCWDT,
    _reserved67: [u8; 0x3e50],
    #[doc = "0xc000..0xc060 - Watchdog Timer"]
    pub wdt: WDT,
}
#[doc = "PWR_LVD_STATUS (r) register accessor: an alias for `Reg<PWR_LVD_STATUS_SPEC>`"]
pub type PWR_LVD_STATUS = crate::Reg<pwr_lvd_status::PWR_LVD_STATUS_SPEC>;
#[doc = "High Voltage / Low Voltage Detector (HVLVD) Status Register"]
pub mod pwr_lvd_status;
#[doc = "PWR_LVD_STATUS2 (r) register accessor: an alias for `Reg<PWR_LVD_STATUS2_SPEC>`"]
pub type PWR_LVD_STATUS2 = crate::Reg<pwr_lvd_status2::PWR_LVD_STATUS2_SPEC>;
#[doc = "High Voltage / Low Voltage Detector (HVLVD) Status Register #2"]
pub mod pwr_lvd_status2;
#[doc = "CLK_DSI_SELECT (rw) register accessor: an alias for `Reg<CLK_DSI_SELECT_SPEC>`"]
pub type CLK_DSI_SELECT = crate::Reg<clk_dsi_select::CLK_DSI_SELECT_SPEC>;
#[doc = "Clock DSI Select Register"]
pub mod clk_dsi_select;
#[doc = "CLK_OUTPUT_FAST (rw) register accessor: an alias for `Reg<CLK_OUTPUT_FAST_SPEC>`"]
pub type CLK_OUTPUT_FAST = crate::Reg<clk_output_fast::CLK_OUTPUT_FAST_SPEC>;
#[doc = "Fast Clock Output Select Register"]
pub mod clk_output_fast;
#[doc = "CLK_OUTPUT_SLOW (rw) register accessor: an alias for `Reg<CLK_OUTPUT_SLOW_SPEC>`"]
pub type CLK_OUTPUT_SLOW = crate::Reg<clk_output_slow::CLK_OUTPUT_SLOW_SPEC>;
#[doc = "Slow Clock Output Select Register"]
pub mod clk_output_slow;
#[doc = "CLK_CAL_CNT1 (rw) register accessor: an alias for `Reg<CLK_CAL_CNT1_SPEC>`"]
pub type CLK_CAL_CNT1 = crate::Reg<clk_cal_cnt1::CLK_CAL_CNT1_SPEC>;
#[doc = "Clock Calibration Counter 1"]
pub mod clk_cal_cnt1;
#[doc = "CLK_CAL_CNT2 (r) register accessor: an alias for `Reg<CLK_CAL_CNT2_SPEC>`"]
pub type CLK_CAL_CNT2 = crate::Reg<clk_cal_cnt2::CLK_CAL_CNT2_SPEC>;
#[doc = "Clock Calibration Counter 2"]
pub mod clk_cal_cnt2;
#[doc = "SRSS_INTR (rw) register accessor: an alias for `Reg<SRSS_INTR_SPEC>`"]
pub type SRSS_INTR = crate::Reg<srss_intr::SRSS_INTR_SPEC>;
#[doc = "SRSS Interrupt Register"]
pub mod srss_intr;
#[doc = "SRSS_INTR_SET (rw) register accessor: an alias for `Reg<SRSS_INTR_SET_SPEC>`"]
pub type SRSS_INTR_SET = crate::Reg<srss_intr_set::SRSS_INTR_SET_SPEC>;
#[doc = "SRSS Interrupt Set Register"]
pub mod srss_intr_set;
#[doc = "SRSS_INTR_MASK (rw) register accessor: an alias for `Reg<SRSS_INTR_MASK_SPEC>`"]
pub type SRSS_INTR_MASK = crate::Reg<srss_intr_mask::SRSS_INTR_MASK_SPEC>;
#[doc = "SRSS Interrupt Mask Register"]
pub mod srss_intr_mask;
#[doc = "SRSS_INTR_MASKED (r) register accessor: an alias for `Reg<SRSS_INTR_MASKED_SPEC>`"]
pub type SRSS_INTR_MASKED = crate::Reg<srss_intr_masked::SRSS_INTR_MASKED_SPEC>;
#[doc = "SRSS Interrupt Masked Register"]
pub mod srss_intr_masked;
#[doc = "PWR_CTL (r) register accessor: an alias for `Reg<PWR_CTL_SPEC>`"]
pub type PWR_CTL = crate::Reg<pwr_ctl::PWR_CTL_SPEC>;
#[doc = "Power Mode Control"]
pub mod pwr_ctl;
#[doc = "PWR_CTL2 (rw) register accessor: an alias for `Reg<PWR_CTL2_SPEC>`"]
pub type PWR_CTL2 = crate::Reg<pwr_ctl2::PWR_CTL2_SPEC>;
#[doc = "Power Mode Control 2"]
pub mod pwr_ctl2;
#[doc = "PWR_HIBERNATE (rw) register accessor: an alias for `Reg<PWR_HIBERNATE_SPEC>`"]
pub type PWR_HIBERNATE = crate::Reg<pwr_hibernate::PWR_HIBERNATE_SPEC>;
#[doc = "HIBERNATE Mode Register"]
pub mod pwr_hibernate;
#[doc = "PWR_BUCK_CTL (rw) register accessor: an alias for `Reg<PWR_BUCK_CTL_SPEC>`"]
pub type PWR_BUCK_CTL = crate::Reg<pwr_buck_ctl::PWR_BUCK_CTL_SPEC>;
#[doc = "Buck Control Register"]
pub mod pwr_buck_ctl;
#[doc = "PWR_BUCK_CTL2 (rw) register accessor: an alias for `Reg<PWR_BUCK_CTL2_SPEC>`"]
pub type PWR_BUCK_CTL2 = crate::Reg<pwr_buck_ctl2::PWR_BUCK_CTL2_SPEC>;
#[doc = "Buck Control Register 2"]
pub mod pwr_buck_ctl2;
#[doc = "PWR_SSV_CTL (rw) register accessor: an alias for `Reg<PWR_SSV_CTL_SPEC>`"]
pub type PWR_SSV_CTL = crate::Reg<pwr_ssv_ctl::PWR_SSV_CTL_SPEC>;
#[doc = "Supply Supervision Control Register"]
pub mod pwr_ssv_ctl;
#[doc = "PWR_SSV_STATUS (r) register accessor: an alias for `Reg<PWR_SSV_STATUS_SPEC>`"]
pub type PWR_SSV_STATUS = crate::Reg<pwr_ssv_status::PWR_SSV_STATUS_SPEC>;
#[doc = "Supply Supervision Status Register"]
pub mod pwr_ssv_status;
#[doc = "PWR_LVD_CTL (rw) register accessor: an alias for `Reg<PWR_LVD_CTL_SPEC>`"]
pub type PWR_LVD_CTL = crate::Reg<pwr_lvd_ctl::PWR_LVD_CTL_SPEC>;
#[doc = "High Voltage / Low Voltage Detector (HVLVD) Configuration Register"]
pub mod pwr_lvd_ctl;
#[doc = "PWR_LVD_CTL2 (rw) register accessor: an alias for `Reg<PWR_LVD_CTL2_SPEC>`"]
pub type PWR_LVD_CTL2 = crate::Reg<pwr_lvd_ctl2::PWR_LVD_CTL2_SPEC>;
#[doc = "High Voltage / Low Voltage Detector (HVLVD) Configuration Register #2"]
pub mod pwr_lvd_ctl2;
#[doc = "PWR_REGHC_CTL (rw) register accessor: an alias for `Reg<PWR_REGHC_CTL_SPEC>`"]
pub type PWR_REGHC_CTL = crate::Reg<pwr_reghc_ctl::PWR_REGHC_CTL_SPEC>;
#[doc = "REGHC Control Register"]
pub mod pwr_reghc_ctl;
#[doc = "PWR_REGHC_STATUS (r) register accessor: an alias for `Reg<PWR_REGHC_STATUS_SPEC>`"]
pub type PWR_REGHC_STATUS = crate::Reg<pwr_reghc_status::PWR_REGHC_STATUS_SPEC>;
#[doc = "REGHC Status Register"]
pub mod pwr_reghc_status;
#[doc = "PWR_REGHC_CTL2 (rw) register accessor: an alias for `Reg<PWR_REGHC_CTL2_SPEC>`"]
pub type PWR_REGHC_CTL2 = crate::Reg<pwr_reghc_ctl2::PWR_REGHC_CTL2_SPEC>;
#[doc = "REGHC Control Register 2"]
pub mod pwr_reghc_ctl2;
#[doc = "PWR_REGHC_CTL4 (rw) register accessor: an alias for `Reg<PWR_REGHC_CTL4_SPEC>`"]
pub type PWR_REGHC_CTL4 = crate::Reg<pwr_reghc_ctl4::PWR_REGHC_CTL4_SPEC>;
#[doc = "REGHC Control Register 4"]
pub mod pwr_reghc_ctl4;
#[doc = "PWR_HIB_DATA (rw) register accessor: an alias for `Reg<PWR_HIB_DATA_SPEC>`"]
pub type PWR_HIB_DATA = crate::Reg<pwr_hib_data::PWR_HIB_DATA_SPEC>;
#[doc = "HIBERNATE Data Register"]
pub mod pwr_hib_data;
#[doc = "PWR_PMIC_CTL (rw) register accessor: an alias for `Reg<PWR_PMIC_CTL_SPEC>`"]
pub type PWR_PMIC_CTL = crate::Reg<pwr_pmic_ctl::PWR_PMIC_CTL_SPEC>;
#[doc = "PMIC Control Register"]
pub mod pwr_pmic_ctl;
#[doc = "PWR_PMIC_STATUS (r) register accessor: an alias for `Reg<PWR_PMIC_STATUS_SPEC>`"]
pub type PWR_PMIC_STATUS = crate::Reg<pwr_pmic_status::PWR_PMIC_STATUS_SPEC>;
#[doc = "PMIC Status Register"]
pub mod pwr_pmic_status;
#[doc = "PWR_PMIC_CTL2 (rw) register accessor: an alias for `Reg<PWR_PMIC_CTL2_SPEC>`"]
pub type PWR_PMIC_CTL2 = crate::Reg<pwr_pmic_ctl2::PWR_PMIC_CTL2_SPEC>;
#[doc = "PMIC Control Register 2"]
pub mod pwr_pmic_ctl2;
#[doc = "PWR_PMIC_CTL4 (rw) register accessor: an alias for `Reg<PWR_PMIC_CTL4_SPEC>`"]
pub type PWR_PMIC_CTL4 = crate::Reg<pwr_pmic_ctl4::PWR_PMIC_CTL4_SPEC>;
#[doc = "PMIC Control Register 4"]
pub mod pwr_pmic_ctl4;
#[doc = "CLK_PATH_SELECT (rw) register accessor: an alias for `Reg<CLK_PATH_SELECT_SPEC>`"]
pub type CLK_PATH_SELECT = crate::Reg<clk_path_select::CLK_PATH_SELECT_SPEC>;
#[doc = "Clock Path Select Register"]
pub mod clk_path_select;
#[doc = "CLK_ROOT_SELECT (rw) register accessor: an alias for `Reg<CLK_ROOT_SELECT_SPEC>`"]
pub type CLK_ROOT_SELECT = crate::Reg<clk_root_select::CLK_ROOT_SELECT_SPEC>;
#[doc = "Clock Root Select Register"]
pub mod clk_root_select;
#[doc = "Clock Supervisor (CSV) registers for Root clocks"]
pub use self::csv_hf::CSV_HF;
#[doc = r"Cluster"]
#[doc = "Clock Supervisor (CSV) registers for Root clocks"]
pub mod csv_hf;
#[doc = "CLK_SELECT (rw) register accessor: an alias for `Reg<CLK_SELECT_SPEC>`"]
pub type CLK_SELECT = crate::Reg<clk_select::CLK_SELECT_SPEC>;
#[doc = "Clock selection register"]
pub mod clk_select;
#[doc = "CLK_TIMER_CTL (rw) register accessor: an alias for `Reg<CLK_TIMER_CTL_SPEC>`"]
pub type CLK_TIMER_CTL = crate::Reg<clk_timer_ctl::CLK_TIMER_CTL_SPEC>;
#[doc = "Timer Clock Control Register"]
pub mod clk_timer_ctl;
#[doc = "CLK_ILO0_CONFIG (rw) register accessor: an alias for `Reg<CLK_ILO0_CONFIG_SPEC>`"]
pub type CLK_ILO0_CONFIG = crate::Reg<clk_ilo0_config::CLK_ILO0_CONFIG_SPEC>;
#[doc = "ILO0 Configuration"]
pub mod clk_ilo0_config;
#[doc = "CLK_ILO1_CONFIG (rw) register accessor: an alias for `Reg<CLK_ILO1_CONFIG_SPEC>`"]
pub type CLK_ILO1_CONFIG = crate::Reg<clk_ilo1_config::CLK_ILO1_CONFIG_SPEC>;
#[doc = "ILO1 Configuration"]
pub mod clk_ilo1_config;
#[doc = "CLK_IMO_CONFIG (rw) register accessor: an alias for `Reg<CLK_IMO_CONFIG_SPEC>`"]
pub type CLK_IMO_CONFIG = crate::Reg<clk_imo_config::CLK_IMO_CONFIG_SPEC>;
#[doc = "IMO Configuration"]
pub mod clk_imo_config;
#[doc = "CLK_ECO_CONFIG (rw) register accessor: an alias for `Reg<CLK_ECO_CONFIG_SPEC>`"]
pub type CLK_ECO_CONFIG = crate::Reg<clk_eco_config::CLK_ECO_CONFIG_SPEC>;
#[doc = "ECO Configuration Register"]
pub mod clk_eco_config;
#[doc = "CLK_ECO_PRESCALE (rw) register accessor: an alias for `Reg<CLK_ECO_PRESCALE_SPEC>`"]
pub type CLK_ECO_PRESCALE = crate::Reg<clk_eco_prescale::CLK_ECO_PRESCALE_SPEC>;
#[doc = "ECO Prescaler Configuration Register"]
pub mod clk_eco_prescale;
#[doc = "CLK_ECO_STATUS (r) register accessor: an alias for `Reg<CLK_ECO_STATUS_SPEC>`"]
pub type CLK_ECO_STATUS = crate::Reg<clk_eco_status::CLK_ECO_STATUS_SPEC>;
#[doc = "ECO Status Register"]
pub mod clk_eco_status;
#[doc = "CLK_PILO_CONFIG (rw) register accessor: an alias for `Reg<CLK_PILO_CONFIG_SPEC>`"]
pub type CLK_PILO_CONFIG = crate::Reg<clk_pilo_config::CLK_PILO_CONFIG_SPEC>;
#[doc = "Precision ILO Configuration Register"]
pub mod clk_pilo_config;
#[doc = "CLK_FLL_CONFIG (rw) register accessor: an alias for `Reg<CLK_FLL_CONFIG_SPEC>`"]
pub type CLK_FLL_CONFIG = crate::Reg<clk_fll_config::CLK_FLL_CONFIG_SPEC>;
#[doc = "FLL Configuration Register"]
pub mod clk_fll_config;
#[doc = "CLK_FLL_CONFIG2 (rw) register accessor: an alias for `Reg<CLK_FLL_CONFIG2_SPEC>`"]
pub type CLK_FLL_CONFIG2 = crate::Reg<clk_fll_config2::CLK_FLL_CONFIG2_SPEC>;
#[doc = "FLL Configuration Register 2"]
pub mod clk_fll_config2;
#[doc = "CLK_FLL_CONFIG3 (rw) register accessor: an alias for `Reg<CLK_FLL_CONFIG3_SPEC>`"]
pub type CLK_FLL_CONFIG3 = crate::Reg<clk_fll_config3::CLK_FLL_CONFIG3_SPEC>;
#[doc = "FLL Configuration Register 3"]
pub mod clk_fll_config3;
#[doc = "CLK_FLL_CONFIG4 (rw) register accessor: an alias for `Reg<CLK_FLL_CONFIG4_SPEC>`"]
pub type CLK_FLL_CONFIG4 = crate::Reg<clk_fll_config4::CLK_FLL_CONFIG4_SPEC>;
#[doc = "FLL Configuration Register 4"]
pub mod clk_fll_config4;
#[doc = "CLK_FLL_STATUS (rw) register accessor: an alias for `Reg<CLK_FLL_STATUS_SPEC>`"]
pub type CLK_FLL_STATUS = crate::Reg<clk_fll_status::CLK_FLL_STATUS_SPEC>;
#[doc = "FLL Status Register"]
pub mod clk_fll_status;
#[doc = "CLK_ECO_CONFIG2 (rw) register accessor: an alias for `Reg<CLK_ECO_CONFIG2_SPEC>`"]
pub type CLK_ECO_CONFIG2 = crate::Reg<clk_eco_config2::CLK_ECO_CONFIG2_SPEC>;
#[doc = "ECO Configuration Register 2"]
pub mod clk_eco_config2;
#[doc = "CLK_PLL_CONFIG (rw) register accessor: an alias for `Reg<CLK_PLL_CONFIG_SPEC>`"]
pub type CLK_PLL_CONFIG = crate::Reg<clk_pll_config::CLK_PLL_CONFIG_SPEC>;
#[doc = "PLL Configuration Register"]
pub mod clk_pll_config;
#[doc = "CLK_PLL_STATUS (rw) register accessor: an alias for `Reg<CLK_PLL_STATUS_SPEC>`"]
pub type CLK_PLL_STATUS = crate::Reg<clk_pll_status::CLK_PLL_STATUS_SPEC>;
#[doc = "PLL Status Register"]
pub mod clk_pll_status;
#[doc = "CSV_REF_SEL (rw) register accessor: an alias for `Reg<CSV_REF_SEL_SPEC>`"]
pub type CSV_REF_SEL = crate::Reg<csv_ref_sel::CSV_REF_SEL_SPEC>;
#[doc = "Select CSV Reference clock for Active domain"]
pub mod csv_ref_sel;
#[doc = "CSV registers for the CSV Reference clock"]
pub use self::csv_ref::CSV_REF;
#[doc = r"Cluster"]
#[doc = "CSV registers for the CSV Reference clock"]
pub mod csv_ref;
#[doc = "CSV registers for LF clock"]
pub use self::csv_lf::CSV_LF;
#[doc = r"Cluster"]
#[doc = "CSV registers for LF clock"]
pub mod csv_lf;
#[doc = "CSV registers for HVILO clock"]
pub use self::csv_ilo::CSV_ILO;
#[doc = r"Cluster"]
#[doc = "CSV registers for HVILO clock"]
pub mod csv_ilo;
#[doc = "RES_CAUSE (rw) register accessor: an alias for `Reg<RES_CAUSE_SPEC>`"]
pub type RES_CAUSE = crate::Reg<res_cause::RES_CAUSE_SPEC>;
#[doc = "Reset Cause Observation Register"]
pub mod res_cause;
#[doc = "RES_CAUSE2 (rw) register accessor: an alias for `Reg<RES_CAUSE2_SPEC>`"]
pub type RES_CAUSE2 = crate::Reg<res_cause2::RES_CAUSE2_SPEC>;
#[doc = "Reset Cause Observation Register 2"]
pub mod res_cause2;
#[doc = "400MHz PLL Configuration Register"]
pub use self::clk_pll400m::CLK_PLL400M;
#[doc = r"Cluster"]
#[doc = "400MHz PLL Configuration Register"]
pub mod clk_pll400m;
#[doc = "TST_XRES_SECURE (rw) register accessor: an alias for `Reg<TST_XRES_SECURE_SPEC>`"]
pub type TST_XRES_SECURE = crate::Reg<tst_xres_secure::TST_XRES_SECURE_SPEC>;
#[doc = "SECURE TEST and FIRMWARE TEST Key control register"]
pub mod tst_xres_secure;
#[doc = "RES_PXRES_CTL (w) register accessor: an alias for `Reg<RES_PXRES_CTL_SPEC>`"]
pub type RES_PXRES_CTL = crate::Reg<res_pxres_ctl::RES_PXRES_CTL_SPEC>;
#[doc = "Programmable XRES Control Register"]
pub mod res_pxres_ctl;
#[doc = "PWR_TRIM_WAKE_CTL (rw) register accessor: an alias for `Reg<PWR_TRIM_WAKE_CTL_SPEC>`"]
pub type PWR_TRIM_WAKE_CTL = crate::Reg<pwr_trim_wake_ctl::PWR_TRIM_WAKE_CTL_SPEC>;
#[doc = "Wakeup Trim Register"]
pub mod pwr_trim_wake_ctl;
#[doc = "CLK_TRIM_ILO0_CTL (rw) register accessor: an alias for `Reg<CLK_TRIM_ILO0_CTL_SPEC>`"]
pub type CLK_TRIM_ILO0_CTL = crate::Reg<clk_trim_ilo0_ctl::CLK_TRIM_ILO0_CTL_SPEC>;
#[doc = "ILO0 Trim Register"]
pub mod clk_trim_ilo0_ctl;
#[doc = "PWR_TRIM_PWRSYS_CTL (rw) register accessor: an alias for `Reg<PWR_TRIM_PWRSYS_CTL_SPEC>`"]
pub type PWR_TRIM_PWRSYS_CTL = crate::Reg<pwr_trim_pwrsys_ctl::PWR_TRIM_PWRSYS_CTL_SPEC>;
#[doc = "Power System Trim Register"]
pub mod pwr_trim_pwrsys_ctl;
#[doc = "CLK_TRIM_PILO_CTL (rw) register accessor: an alias for `Reg<CLK_TRIM_PILO_CTL_SPEC>`"]
pub type CLK_TRIM_PILO_CTL = crate::Reg<clk_trim_pilo_ctl::CLK_TRIM_PILO_CTL_SPEC>;
#[doc = "PILO Trim Register"]
pub mod clk_trim_pilo_ctl;
#[doc = "CLK_TRIM_PILO_CTL2 (rw) register accessor: an alias for `Reg<CLK_TRIM_PILO_CTL2_SPEC>`"]
pub type CLK_TRIM_PILO_CTL2 = crate::Reg<clk_trim_pilo_ctl2::CLK_TRIM_PILO_CTL2_SPEC>;
#[doc = "PILO Trim Register 2"]
pub mod clk_trim_pilo_ctl2;
#[doc = "CLK_TRIM_PILO_CTL3 (rw) register accessor: an alias for `Reg<CLK_TRIM_PILO_CTL3_SPEC>`"]
pub type CLK_TRIM_PILO_CTL3 = crate::Reg<clk_trim_pilo_ctl3::CLK_TRIM_PILO_CTL3_SPEC>;
#[doc = "PILO Trim Register 3"]
pub mod clk_trim_pilo_ctl3;
#[doc = "CLK_TRIM_ILO1_CTL (rw) register accessor: an alias for `Reg<CLK_TRIM_ILO1_CTL_SPEC>`"]
pub type CLK_TRIM_ILO1_CTL = crate::Reg<clk_trim_ilo1_ctl::CLK_TRIM_ILO1_CTL_SPEC>;
#[doc = "ILO1 Trim Register"]
pub mod clk_trim_ilo1_ctl;
#[doc = "Multi-Counter Watchdog Timer"]
pub use self::mcwdt::MCWDT;
#[doc = r"Cluster"]
#[doc = "Multi-Counter Watchdog Timer"]
pub mod mcwdt;
#[doc = "Watchdog Timer"]
pub use self::wdt::WDT;
#[doc = r"Cluster"]
#[doc = "Watchdog Timer"]
pub mod wdt;
