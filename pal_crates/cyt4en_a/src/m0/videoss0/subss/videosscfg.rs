#[doc = r"Register block"]
#[repr(C)]
pub struct VIDEOSSCFG {
    #[doc = "0x00 - IP Control Register"]
    pub ctl: CTL,
    #[doc = "0x04 - IP Control Register for Graphics 2D Core"]
    pub gfx2d_ctl: GFX2D_CTL,
    #[doc = "0x08 - IP and Design Release Identification"]
    pub ipidentifier: IPIDENTIFIER,
    #[doc = "0x0c - IP Design Configuration"]
    pub ipconfiguration: IPCONFIGURATION,
    #[doc = "0x10 - Display 0 Clock Configuration Register"]
    pub clkdsp0cfg: CLKDSP0CFG,
    #[doc = "0x14 - Display 1 Clock Configuration Register"]
    pub clkdsp1cfg: CLKDSP1CFG,
    #[doc = "0x18 - Capture 0 Configuration"]
    pub cap0cfg: CAP0CFG,
    #[doc = "0x1c - FPDLink Configuration"]
    pub fpdlinkcfg: FPDLINKCFG,
    #[doc = "0x20 - Enable for Display Engine 0"]
    pub dsp0_ctl: DSP0_CTL,
    #[doc = "0x24 - Enable for Display Engine 1"]
    pub dsp1_ctl: DSP1_CTL,
    #[doc = "0x28 - Pin multiplexer for capture input from TTL."]
    pub ttl0_mux0: TTL0_MUX0,
    #[doc = "0x2c - Pin multiplexer for capture input from TTL."]
    pub ttl0_mux1: TTL0_MUX1,
    #[doc = "0x30 - Pin multiplexer for capture input from TTL."]
    pub ttl0_mux2: TTL0_MUX2,
    #[doc = "0x34 - Pin multiplexer for capture input from TTL."]
    pub ttl0_mux3: TTL0_MUX3,
    #[doc = "0x38 - Pin multiplexer for capture input from TTL."]
    pub ttl0_mux4: TTL0_MUX4,
    #[doc = "0x3c - Pin multiplexer for capture input from TTL."]
    pub ttl0_mux5: TTL0_MUX5,
    #[doc = "0x40 - Pin multiplexer for capture input from TTL."]
    pub ttl0_mux6: TTL0_MUX6,
}
#[doc = "CTL (rw) register accessor: an alias for `Reg<CTL_SPEC>`"]
pub type CTL = crate::Reg<ctl::CTL_SPEC>;
#[doc = "IP Control Register"]
pub mod ctl;
#[doc = "GFX2D_CTL (rw) register accessor: an alias for `Reg<GFX2D_CTL_SPEC>`"]
pub type GFX2D_CTL = crate::Reg<gfx2d_ctl::GFX2D_CTL_SPEC>;
#[doc = "IP Control Register for Graphics 2D Core"]
pub mod gfx2d_ctl;
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
#[doc = "Capture 0 Configuration"]
pub mod cap0cfg;
#[doc = "FPDLINKCFG (rw) register accessor: an alias for `Reg<FPDLINKCFG_SPEC>`"]
pub type FPDLINKCFG = crate::Reg<fpdlinkcfg::FPDLINKCFG_SPEC>;
#[doc = "FPDLink Configuration"]
pub mod fpdlinkcfg;
#[doc = "DSP0_CTL (rw) register accessor: an alias for `Reg<DSP0_CTL_SPEC>`"]
pub type DSP0_CTL = crate::Reg<dsp0_ctl::DSP0_CTL_SPEC>;
#[doc = "Enable for Display Engine 0"]
pub mod dsp0_ctl;
#[doc = "DSP1_CTL (rw) register accessor: an alias for `Reg<DSP1_CTL_SPEC>`"]
pub type DSP1_CTL = crate::Reg<dsp1_ctl::DSP1_CTL_SPEC>;
#[doc = "Enable for Display Engine 1"]
pub mod dsp1_ctl;
#[doc = "TTL0_MUX0 (rw) register accessor: an alias for `Reg<TTL0_MUX0_SPEC>`"]
pub type TTL0_MUX0 = crate::Reg<ttl0_mux0::TTL0_MUX0_SPEC>;
#[doc = "Pin multiplexer for capture input from TTL."]
pub mod ttl0_mux0;
#[doc = "TTL0_MUX1 (rw) register accessor: an alias for `Reg<TTL0_MUX1_SPEC>`"]
pub type TTL0_MUX1 = crate::Reg<ttl0_mux1::TTL0_MUX1_SPEC>;
#[doc = "Pin multiplexer for capture input from TTL."]
pub mod ttl0_mux1;
#[doc = "TTL0_MUX2 (rw) register accessor: an alias for `Reg<TTL0_MUX2_SPEC>`"]
pub type TTL0_MUX2 = crate::Reg<ttl0_mux2::TTL0_MUX2_SPEC>;
#[doc = "Pin multiplexer for capture input from TTL."]
pub mod ttl0_mux2;
#[doc = "TTL0_MUX3 (rw) register accessor: an alias for `Reg<TTL0_MUX3_SPEC>`"]
pub type TTL0_MUX3 = crate::Reg<ttl0_mux3::TTL0_MUX3_SPEC>;
#[doc = "Pin multiplexer for capture input from TTL."]
pub mod ttl0_mux3;
#[doc = "TTL0_MUX4 (rw) register accessor: an alias for `Reg<TTL0_MUX4_SPEC>`"]
pub type TTL0_MUX4 = crate::Reg<ttl0_mux4::TTL0_MUX4_SPEC>;
#[doc = "Pin multiplexer for capture input from TTL."]
pub mod ttl0_mux4;
#[doc = "TTL0_MUX5 (rw) register accessor: an alias for `Reg<TTL0_MUX5_SPEC>`"]
pub type TTL0_MUX5 = crate::Reg<ttl0_mux5::TTL0_MUX5_SPEC>;
#[doc = "Pin multiplexer for capture input from TTL."]
pub mod ttl0_mux5;
#[doc = "TTL0_MUX6 (rw) register accessor: an alias for `Reg<TTL0_MUX6_SPEC>`"]
pub type TTL0_MUX6 = crate::Reg<ttl0_mux6::TTL0_MUX6_SPEC>;
#[doc = "Pin multiplexer for capture input from TTL."]
pub mod ttl0_mux6;
