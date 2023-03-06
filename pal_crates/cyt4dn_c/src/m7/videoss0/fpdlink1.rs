#[doc = r"Register block"]
#[repr(C)]
pub struct FPDLINK1 {
    #[doc = "0x00 - IP Control register for FPDLink"]
    pub ctl: CTL,
    #[doc = "0x04 - IP Command register for FPDLink"]
    pub cmd: CMD,
    #[doc = "0x08 - Control register for CLK_GEN"]
    pub clk_gen_cmd: CLK_GEN_CMD,
    #[doc = "0x0c - Configuration register for CKGEN"]
    pub clk_gen_ctl: CLK_GEN_CTL,
    #[doc = "0x10 - Status register for CKGEN"]
    pub clk_gen_status: CLK_GEN_STATUS,
}
#[doc = "CTL (rw) register accessor: an alias for `Reg<CTL_SPEC>`"]
pub type CTL = crate::Reg<ctl::CTL_SPEC>;
#[doc = "IP Control register for FPDLink"]
pub mod ctl;
#[doc = "CMD (rw) register accessor: an alias for `Reg<CMD_SPEC>`"]
pub type CMD = crate::Reg<cmd::CMD_SPEC>;
#[doc = "IP Command register for FPDLink"]
pub mod cmd;
#[doc = "CLK_GEN_CMD (rw) register accessor: an alias for `Reg<CLK_GEN_CMD_SPEC>`"]
pub type CLK_GEN_CMD = crate::Reg<clk_gen_cmd::CLK_GEN_CMD_SPEC>;
#[doc = "Control register for CLK_GEN"]
pub mod clk_gen_cmd;
#[doc = "CLK_GEN_CTL (rw) register accessor: an alias for `Reg<CLK_GEN_CTL_SPEC>`"]
pub type CLK_GEN_CTL = crate::Reg<clk_gen_ctl::CLK_GEN_CTL_SPEC>;
#[doc = "Configuration register for CKGEN"]
pub mod clk_gen_ctl;
#[doc = "CLK_GEN_STATUS (r) register accessor: an alias for `Reg<CLK_GEN_STATUS_SPEC>`"]
pub type CLK_GEN_STATUS = crate::Reg<clk_gen_status::CLK_GEN_STATUS_SPEC>;
#[doc = "Status register for CKGEN"]
pub mod clk_gen_status;
