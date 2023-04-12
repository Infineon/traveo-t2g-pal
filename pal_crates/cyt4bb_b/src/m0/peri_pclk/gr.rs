#[doc = r"Register block"]
#[repr(C)]
pub struct GR {
    #[doc = "0x00 - Divider command"]
    pub div_cmd: DIV_CMD,
    _reserved1: [u8; 0x0bfc],
    #[doc = "0xc00..0x1000 - Clock control"]
    pub clock_ctl: [CLOCK_CTL; 256],
    #[doc = "0x1000..0x1400 - Divider control (for 8.0 divider)"]
    pub div_8_ctl: [DIV_8_CTL; 256],
    #[doc = "0x1400..0x1800 - Divider control (for 16.0 divider)"]
    pub div_16_ctl: [DIV_16_CTL; 256],
    #[doc = "0x1800..0x1c00 - Divider control (for 16.5 divider)"]
    pub div_16_5_ctl: [DIV_16_5_CTL; 256],
    #[doc = "0x1c00..0x1ffc - Divider control (for 24.5 divider)"]
    pub div_24_5_ctl: [DIV_24_5_CTL; 255],
}
#[doc = "DIV_CMD (rw) register accessor: an alias for `Reg<DIV_CMD_SPEC>`"]
pub type DIV_CMD = crate::Reg<div_cmd::DIV_CMD_SPEC>;
#[doc = "Divider command"]
pub mod div_cmd;
#[doc = "CLOCK_CTL (rw) register accessor: an alias for `Reg<CLOCK_CTL_SPEC>`"]
pub type CLOCK_CTL = crate::Reg<clock_ctl::CLOCK_CTL_SPEC>;
#[doc = "Clock control"]
pub mod clock_ctl;
#[doc = "DIV_8_CTL (rw) register accessor: an alias for `Reg<DIV_8_CTL_SPEC>`"]
pub type DIV_8_CTL = crate::Reg<div_8_ctl::DIV_8_CTL_SPEC>;
#[doc = "Divider control (for 8.0 divider)"]
pub mod div_8_ctl;
#[doc = "DIV_16_CTL (rw) register accessor: an alias for `Reg<DIV_16_CTL_SPEC>`"]
pub type DIV_16_CTL = crate::Reg<div_16_ctl::DIV_16_CTL_SPEC>;
#[doc = "Divider control (for 16.0 divider)"]
pub mod div_16_ctl;
#[doc = "DIV_16_5_CTL (rw) register accessor: an alias for `Reg<DIV_16_5_CTL_SPEC>`"]
pub type DIV_16_5_CTL = crate::Reg<div_16_5_ctl::DIV_16_5_CTL_SPEC>;
#[doc = "Divider control (for 16.5 divider)"]
pub mod div_16_5_ctl;
#[doc = "DIV_24_5_CTL (rw) register accessor: an alias for `Reg<DIV_24_5_CTL_SPEC>`"]
pub type DIV_24_5_CTL = crate::Reg<div_24_5_ctl::DIV_24_5_CTL_SPEC>;
#[doc = "Divider control (for 24.5 divider)"]
pub mod div_24_5_ctl;
