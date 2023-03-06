#[doc = r"Register block"]
#[repr(C)]
pub struct VIDEOSSCFG {
    #[doc = "0x00 - IP Control Register"]
    pub ipctrl: IPCTRL,
    #[doc = "0x04 - IP Control Register for Graphics 2D Core"]
    pub ipctrlgfx2d: IPCTRLGFX2D,
    #[doc = "0x08 - IP and Design Release Identification"]
    pub ipidentifier: IPIDENTIFIER,
    #[doc = "0x0c - IP Design Configuration"]
    pub ipconfiguration: IPCONFIGURATION,
    #[doc = "0x10 - Display 0 Clock Configuration Register"]
    pub clkdsp0cfg: CLKDSP0CFG,
    #[doc = "0x14 - Display 1 Clock Configuration Register"]
    pub clkdsp1cfg: CLKDSP1CFG,
    #[doc = "0x18 - Capture 0 Confifuration"]
    pub cap0cfg: CAP0CFG,
    #[doc = "0x1c - FPDLink Configuration"]
    pub fpdlinkcfg: FPDLINKCFG,
}
#[doc = "IPCTRL (rw) register accessor: an alias for `Reg<IPCTRL_SPEC>`"]
pub type IPCTRL = crate::Reg<ipctrl::IPCTRL_SPEC>;
#[doc = "IP Control Register"]
pub mod ipctrl;
#[doc = "IPCTRLGFX2D (rw) register accessor: an alias for `Reg<IPCTRLGFX2D_SPEC>`"]
pub type IPCTRLGFX2D = crate::Reg<ipctrlgfx2d::IPCTRLGFX2D_SPEC>;
#[doc = "IP Control Register for Graphics 2D Core"]
pub mod ipctrlgfx2d;
#[doc = "IPIDENTIFIER (r) register accessor: an alias for `Reg<IPIDENTIFIER_SPEC>`"]
pub type IPIDENTIFIER = crate::Reg<ipidentifier::IPIDENTIFIER_SPEC>;
#[doc = "IP and Design Release Identification"]
pub mod ipidentifier;
#[doc = "IPCONFIGURATION (r) register accessor: an alias for `Reg<IPCONFIGURATION_SPEC>`"]
pub type IPCONFIGURATION = crate::Reg<ipconfiguration::IPCONFIGURATION_SPEC>;
#[doc = "IP Design Configuration"]
pub mod ipconfiguration;
#[doc = "CLKDSP0CFG (rw) register accessor: an alias for `Reg<CLKDSP0CFG_SPEC>`"]
pub type CLKDSP0CFG = crate::Reg<clkdsp0cfg::CLKDSP0CFG_SPEC>;
#[doc = "Display 0 Clock Configuration Register"]
pub mod clkdsp0cfg;
#[doc = "CLKDSP1CFG (rw) register accessor: an alias for `Reg<CLKDSP1CFG_SPEC>`"]
pub type CLKDSP1CFG = crate::Reg<clkdsp1cfg::CLKDSP1CFG_SPEC>;
#[doc = "Display 1 Clock Configuration Register"]
pub mod clkdsp1cfg;
#[doc = "CAP0CFG (rw) register accessor: an alias for `Reg<CAP0CFG_SPEC>`"]
pub type CAP0CFG = crate::Reg<cap0cfg::CAP0CFG_SPEC>;
#[doc = "Capture 0 Confifuration"]
pub mod cap0cfg;
#[doc = "FPDLINKCFG (rw) register accessor: an alias for `Reg<FPDLINKCFG_SPEC>`"]
pub type FPDLINKCFG = crate::Reg<fpdlinkcfg::FPDLINKCFG_SPEC>;
#[doc = "FPDLink Configuration"]
pub mod fpdlinkcfg;
