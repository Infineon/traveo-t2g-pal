#[doc = r"Register block"]
#[repr(C)]
pub struct CLK_PLL400M {
    #[doc = "0x00 - 400MHz PLL Configuration Register"]
    pub config: CONFIG,
    #[doc = "0x04 - 400MHz PLL Configuration Register 2"]
    pub config2: CONFIG2,
    #[doc = "0x08 - 400MHz PLL Configuration Register 3"]
    pub config3: CONFIG3,
    #[doc = "0x0c - 400MHz PLL Status Register"]
    pub status: STATUS,
}
#[doc = "CONFIG (rw) register accessor: an alias for `Reg<CONFIG_SPEC>`"]
pub type CONFIG = crate::Reg<config::CONFIG_SPEC>;
#[doc = "400MHz PLL Configuration Register"]
pub mod config;
#[doc = "CONFIG2 (rw) register accessor: an alias for `Reg<CONFIG2_SPEC>`"]
pub type CONFIG2 = crate::Reg<config2::CONFIG2_SPEC>;
#[doc = "400MHz PLL Configuration Register 2"]
pub mod config2;
#[doc = "CONFIG3 (rw) register accessor: an alias for `Reg<CONFIG3_SPEC>`"]
pub type CONFIG3 = crate::Reg<config3::CONFIG3_SPEC>;
#[doc = "400MHz PLL Configuration Register 3"]
pub mod config3;
#[doc = "STATUS (rw) register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "400MHz PLL Status Register"]
pub mod status;
