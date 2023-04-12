#[doc = r"Register block"]
#[repr(C)]
pub struct AXI_PERF_CNT {
    #[doc = "0x00 - Control"]
    pub ctl: CTL,
    #[doc = "0x04 - Timer command"]
    pub tmr_cmd: TMR_CMD,
    _reserved2: [u8; 0x01f8],
    #[doc = "0x200..0x230 - Measurement unit"]
    pub mu0: MU,
    _reserved3: [u8; 0x10],
    #[doc = "0x240..0x270 - Measurement unit"]
    pub mu1: MU,
}
#[doc = "CTL (rw) register accessor: an alias for `Reg<CTL_SPEC>`"]
pub type CTL = crate::Reg<ctl::CTL_SPEC>;
#[doc = "Control"]
pub mod ctl;
#[doc = "TMR_CMD (rw) register accessor: an alias for `Reg<TMR_CMD_SPEC>`"]
pub type TMR_CMD = crate::Reg<tmr_cmd::TMR_CMD_SPEC>;
#[doc = "Timer command"]
pub mod tmr_cmd;
#[doc = "Measurement unit"]
pub use self::mu::MU;
#[doc = r"Cluster"]
#[doc = "Measurement unit"]
pub mod mu;
