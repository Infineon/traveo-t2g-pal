#[doc = r"Register block"]
#[repr(C)]
pub struct FPDLINK0 {
    #[doc = "0x00 - IP Control register for fpdlink"]
    pub ctl: CTL,
    #[doc = "0x04 - IP Command register for fpdlink"]
    pub cmd: CMD,
    #[doc = "0x08 - Control register for CLK_GEN"]
    pub clk_gen_cmd: CLK_GEN_CMD,
    #[doc = "0x0c - Configuration register for CKGEN"]
    pub clk_gen_ctl: CLK_GEN_CTL,
    #[doc = "0x10 - Status register for CKGEN"]
    pub clk_gen_status: CLK_GEN_STATUS,
    #[doc = "0x14 - DFT control register for fpdlink"]
    pub dft_cmd: DFT_CMD,
    #[doc = "0x18 - DFT configuration register for fpdlink"]
    pub dft_ctl: DFT_CTL,
    #[doc = "0x1c - Digital DFT control register (select signals for Muxes used for analog signal checking purposes)"]
    pub ddft_ctl: DDFT_CTL,
    #[doc = "0x20 - BIST control register"]
    pub bist_ctl: BIST_CTL,
    #[doc = "0x24 - BIST control register"]
    pub bist_data: BIST_DATA,
    #[doc = "0x28 - BIST Loopback status register"]
    pub bist_lb_status: BIST_LB_STATUS,
    #[doc = "0x2c - BIST loopback count status register"]
    pub bist_cnt_status: BIST_CNT_STATUS,
}
#[doc = "CTL (rw) register accessor: an alias for `Reg<CTL_SPEC>`"]
pub type CTL = crate::Reg<ctl::CTL_SPEC>;
#[doc = "IP Control register for fpdlink"]
pub mod ctl;
#[doc = "CMD (rw) register accessor: an alias for `Reg<CMD_SPEC>`"]
pub type CMD = crate::Reg<cmd::CMD_SPEC>;
#[doc = "IP Command register for fpdlink"]
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
#[doc = "DFT_CMD (rw) register accessor: an alias for `Reg<DFT_CMD_SPEC>`"]
pub type DFT_CMD = crate::Reg<dft_cmd::DFT_CMD_SPEC>;
#[doc = "DFT control register for fpdlink"]
pub mod dft_cmd;
#[doc = "DFT_CTL (rw) register accessor: an alias for `Reg<DFT_CTL_SPEC>`"]
pub type DFT_CTL = crate::Reg<dft_ctl::DFT_CTL_SPEC>;
#[doc = "DFT configuration register for fpdlink"]
pub mod dft_ctl;
#[doc = "DDFT_CTL (rw) register accessor: an alias for `Reg<DDFT_CTL_SPEC>`"]
pub type DDFT_CTL = crate::Reg<ddft_ctl::DDFT_CTL_SPEC>;
#[doc = "Digital DFT control register (select signals for Muxes used for analog signal checking purposes)"]
pub mod ddft_ctl;
#[doc = "BIST_CTL (rw) register accessor: an alias for `Reg<BIST_CTL_SPEC>`"]
pub type BIST_CTL = crate::Reg<bist_ctl::BIST_CTL_SPEC>;
#[doc = "BIST control register"]
pub mod bist_ctl;
#[doc = "BIST_DATA (rw) register accessor: an alias for `Reg<BIST_DATA_SPEC>`"]
pub type BIST_DATA = crate::Reg<bist_data::BIST_DATA_SPEC>;
#[doc = "BIST control register"]
pub mod bist_data;
#[doc = "BIST_LB_STATUS (r) register accessor: an alias for `Reg<BIST_LB_STATUS_SPEC>`"]
pub type BIST_LB_STATUS = crate::Reg<bist_lb_status::BIST_LB_STATUS_SPEC>;
#[doc = "BIST Loopback status register"]
pub mod bist_lb_status;
#[doc = "BIST_CNT_STATUS (r) register accessor: an alias for `Reg<BIST_CNT_STATUS_SPEC>`"]
pub type BIST_CNT_STATUS = crate::Reg<bist_cnt_status::BIST_CNT_STATUS_SPEC>;
#[doc = "BIST loopback count status register"]
pub mod bist_cnt_status;
