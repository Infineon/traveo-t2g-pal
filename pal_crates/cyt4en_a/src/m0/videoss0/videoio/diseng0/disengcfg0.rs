#[doc = r"Register block"]
#[repr(C)]
pub struct DISENGCFG0 {
    #[doc = "0x00 - Display engine control register."]
    pub ctl: CTL,
    #[doc = "0x04 - Polarity control for TCon input and corresponding top-level output (TCon by-pass port)."]
    pub polarityctrl: POLARITYCTRL,
    _reserved2: [u8; 0x08],
    #[doc = "0x10 - Tap selection for Signature."]
    pub sigselect: SIGSELECT,
    #[doc = "0x14 - Tap selection for Histogram."]
    pub histogramselect: HISTOGRAMSELECT,
    _reserved4: [u8; 0x08],
    #[doc = "0x20 - Tap selection for Frame Dump."]
    pub dumpselect: DUMPSELECT,
}
#[doc = "CTL (rw) register accessor: an alias for `Reg<CTL_SPEC>`"]
pub type CTL = crate::Reg<ctl::CTL_SPEC>;
#[doc = "Display engine control register."]
pub mod ctl;
#[doc = "POLARITYCTRL (rw) register accessor: an alias for `Reg<POLARITYCTRL_SPEC>`"]
pub type POLARITYCTRL = crate::Reg<polarityctrl::POLARITYCTRL_SPEC>;
#[doc = "Polarity control for TCon input and corresponding top-level output (TCon by-pass port)."]
pub mod polarityctrl;
#[doc = "SIGSELECT (rw) register accessor: an alias for `Reg<SIGSELECT_SPEC>`"]
pub type SIGSELECT = crate::Reg<sigselect::SIGSELECT_SPEC>;
#[doc = "Tap selection for Signature."]
pub mod sigselect;
#[doc = "HISTOGRAMSELECT (rw) register accessor: an alias for `Reg<HISTOGRAMSELECT_SPEC>`"]
pub type HISTOGRAMSELECT = crate::Reg<histogramselect::HISTOGRAMSELECT_SPEC>;
#[doc = "Tap selection for Histogram."]
pub mod histogramselect;
#[doc = "DUMPSELECT (rw) register accessor: an alias for `Reg<DUMPSELECT_SPEC>`"]
pub type DUMPSELECT = crate::Reg<dumpselect::DUMPSELECT_SPEC>;
#[doc = "Tap selection for Frame Dump."]
pub mod dumpselect;
