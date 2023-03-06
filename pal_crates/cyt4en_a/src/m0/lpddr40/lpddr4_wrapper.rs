#[doc = r"Register block"]
#[repr(C)]
pub struct LPDDR4_WRAPPER {
    #[doc = "0x00 - Control"]
    pub ctl: CTL,
    #[doc = "0x04 - 800MHz PLL Control Register"]
    pub pll800_clock_ctl: PLL800_CLOCK_CTL,
    _reserved2: [u8; 0x08],
    #[doc = "0x10 - 800MHz PLL Configuration Register"]
    pub pll800_config: PLL800_CONFIG,
    #[doc = "0x14 - 800MHz PLL Configuration Register 2 (SSCG)"]
    pub pll800_config2: PLL800_CONFIG2,
    _reserved4: [u8; 0x08],
    #[doc = "0x20 - 800MHz PLL Status Register"]
    pub pll800_status: PLL800_STATUS,
    _reserved5: [u8; 0x1c],
    #[doc = "0x40 - QoS Configuration Register for CPUSS master"]
    pub qos_cpuss: QOS_CPUSS,
    #[doc = "0x44 - QoS Configuration Register for VIDEOSS read master"]
    pub qos_videoss_rd: QOS_VIDEOSS_RD,
    #[doc = "0x48 - QoS Configuration Register for VIDEOSS write master"]
    pub qos_videoss_wr: QOS_VIDEOSS_WR,
}
#[doc = "CTL (rw) register accessor: an alias for `Reg<CTL_SPEC>`"]
pub type CTL = crate::Reg<ctl::CTL_SPEC>;
#[doc = "Control"]
pub mod ctl;
#[doc = "PLL800_CLOCK_CTL (rw) register accessor: an alias for `Reg<PLL800_CLOCK_CTL_SPEC>`"]
pub type PLL800_CLOCK_CTL = crate::Reg<pll800_clock_ctl::PLL800_CLOCK_CTL_SPEC>;
#[doc = "800MHz PLL Control Register"]
pub mod pll800_clock_ctl;
#[doc = "PLL800_CONFIG (rw) register accessor: an alias for `Reg<PLL800_CONFIG_SPEC>`"]
pub type PLL800_CONFIG = crate::Reg<pll800_config::PLL800_CONFIG_SPEC>;
#[doc = "800MHz PLL Configuration Register"]
pub mod pll800_config;
#[doc = "PLL800_CONFIG2 (rw) register accessor: an alias for `Reg<PLL800_CONFIG2_SPEC>`"]
pub type PLL800_CONFIG2 = crate::Reg<pll800_config2::PLL800_CONFIG2_SPEC>;
#[doc = "800MHz PLL Configuration Register 2 (SSCG)"]
pub mod pll800_config2;
#[doc = "PLL800_STATUS (rw) register accessor: an alias for `Reg<PLL800_STATUS_SPEC>`"]
pub type PLL800_STATUS = crate::Reg<pll800_status::PLL800_STATUS_SPEC>;
#[doc = "800MHz PLL Status Register"]
pub mod pll800_status;
#[doc = "QOS_CPUSS (rw) register accessor: an alias for `Reg<QOS_CPUSS_SPEC>`"]
pub type QOS_CPUSS = crate::Reg<qos_cpuss::QOS_CPUSS_SPEC>;
#[doc = "QoS Configuration Register for CPUSS master"]
pub mod qos_cpuss;
#[doc = "QOS_VIDEOSS_RD (rw) register accessor: an alias for `Reg<QOS_VIDEOSS_RD_SPEC>`"]
pub type QOS_VIDEOSS_RD = crate::Reg<qos_videoss_rd::QOS_VIDEOSS_RD_SPEC>;
#[doc = "QoS Configuration Register for VIDEOSS read master"]
pub mod qos_videoss_rd;
#[doc = "QOS_VIDEOSS_WR (rw) register accessor: an alias for `Reg<QOS_VIDEOSS_WR_SPEC>`"]
pub type QOS_VIDEOSS_WR = crate::Reg<qos_videoss_wr::QOS_VIDEOSS_WR_SPEC>;
#[doc = "QoS Configuration Register for VIDEOSS write master"]
pub mod qos_videoss_wr;
