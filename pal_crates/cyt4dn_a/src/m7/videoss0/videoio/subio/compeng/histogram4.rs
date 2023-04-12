#[doc = r"Register block"]
#[repr(C)]
pub struct HISTOGRAM4 {
    #[doc = "0x00 - Controls some settings concerning input components and alpha mask."]
    pub control: CONTROL,
    #[doc = "0x04 - Configures the histogram bins."]
    pub binproperties: BINPROPERTIES,
    #[doc = "0x08 - Bin counter threshold value."]
    pub bincntthresh: BINCNTTHRESH,
    #[doc = "0x0c - Coordinates of upper left corner of clip window."]
    pub clipwinupperleft: CLIPWINUPPERLEFT,
    #[doc = "0x10 - Dimensions of clip window."]
    pub clipwinsize: CLIPWINSIZE,
    #[doc = "0x14 - Measurement result status."]
    pub rsltrdy: RSLTRDY,
    #[doc = "0x18 - Additional information on the measurement results."]
    pub rsltfrminf: RSLTFRMINF,
    #[doc = "0x1c - Results of frame measurement for component 0."]
    pub rsltcomp0binidx: RSLTCOMP0BINIDX,
    #[doc = "0x20 - Results of frame measurement for component 1."]
    pub rsltcomp1binidx: RSLTCOMP1BINIDX,
    #[doc = "0x24 - Results of frame measurement for component 2."]
    pub rsltcomp2binidx: RSLTCOMP2BINIDX,
    #[doc = "0x28 - Sum of all component 0 values."]
    pub rsltcomp0sum: RSLTCOMP0SUM,
    #[doc = "0x2c - Sum of all component 1 values."]
    pub rsltcomp1sum: RSLTCOMP1SUM,
    #[doc = "0x30 - Sum of all component 2 values."]
    pub rsltcomp2sum: RSLTCOMP2SUM,
    #[doc = "0x34 - Bin counters of component 0 histogram. This register do NOT support debugger access followed by software access (they clear on read)"]
    pub rsltcomp0bincnt: RSLTCOMP0BINCNT,
    #[doc = "0x38 - Bin counters of component 1 histogram.This register do NOT support debugger access followed by software access (they clear on read)"]
    pub rsltcomp1bincnt: RSLTCOMP1BINCNT,
    #[doc = "0x3c - Bin counters of component 2 histogram.This register do NOT support debugger access followed by software access (they clear on read)"]
    pub rsltcomp2bincnt: RSLTCOMP2BINCNT,
    #[doc = "0x40 - Triggers for loading shadow registers and starting measurement with next input frame."]
    pub frmstrttrig: FRMSTRTTRIG,
}
#[doc = "CONTROL (rw) register accessor: an alias for `Reg<CONTROL_SPEC>`"]
pub type CONTROL = crate::Reg<control::CONTROL_SPEC>;
#[doc = "Controls some settings concerning input components and alpha mask."]
pub mod control;
#[doc = "BINPROPERTIES (rw) register accessor: an alias for `Reg<BINPROPERTIES_SPEC>`"]
pub type BINPROPERTIES = crate::Reg<binproperties::BINPROPERTIES_SPEC>;
#[doc = "Configures the histogram bins."]
pub mod binproperties;
#[doc = "BINCNTTHRESH (rw) register accessor: an alias for `Reg<BINCNTTHRESH_SPEC>`"]
pub type BINCNTTHRESH = crate::Reg<bincntthresh::BINCNTTHRESH_SPEC>;
#[doc = "Bin counter threshold value."]
pub mod bincntthresh;
#[doc = "CLIPWINUPPERLEFT (rw) register accessor: an alias for `Reg<CLIPWINUPPERLEFT_SPEC>`"]
pub type CLIPWINUPPERLEFT = crate::Reg<clipwinupperleft::CLIPWINUPPERLEFT_SPEC>;
#[doc = "Coordinates of upper left corner of clip window."]
pub mod clipwinupperleft;
#[doc = "CLIPWINSIZE (rw) register accessor: an alias for `Reg<CLIPWINSIZE_SPEC>`"]
pub type CLIPWINSIZE = crate::Reg<clipwinsize::CLIPWINSIZE_SPEC>;
#[doc = "Dimensions of clip window."]
pub mod clipwinsize;
#[doc = "RSLTRDY (r) register accessor: an alias for `Reg<RSLTRDY_SPEC>`"]
pub type RSLTRDY = crate::Reg<rsltrdy::RSLTRDY_SPEC>;
#[doc = "Measurement result status."]
pub mod rsltrdy;
#[doc = "RSLTFRMINF (r) register accessor: an alias for `Reg<RSLTFRMINF_SPEC>`"]
pub type RSLTFRMINF = crate::Reg<rsltfrminf::RSLTFRMINF_SPEC>;
#[doc = "Additional information on the measurement results."]
pub mod rsltfrminf;
#[doc = "RSLTCOMP0BINIDX (r) register accessor: an alias for `Reg<RSLTCOMP0BINIDX_SPEC>`"]
pub type RSLTCOMP0BINIDX = crate::Reg<rsltcomp0binidx::RSLTCOMP0BINIDX_SPEC>;
#[doc = "Results of frame measurement for component 0."]
pub mod rsltcomp0binidx;
#[doc = "RSLTCOMP1BINIDX (r) register accessor: an alias for `Reg<RSLTCOMP1BINIDX_SPEC>`"]
pub type RSLTCOMP1BINIDX = crate::Reg<rsltcomp1binidx::RSLTCOMP1BINIDX_SPEC>;
#[doc = "Results of frame measurement for component 1."]
pub mod rsltcomp1binidx;
#[doc = "RSLTCOMP2BINIDX (r) register accessor: an alias for `Reg<RSLTCOMP2BINIDX_SPEC>`"]
pub type RSLTCOMP2BINIDX = crate::Reg<rsltcomp2binidx::RSLTCOMP2BINIDX_SPEC>;
#[doc = "Results of frame measurement for component 2."]
pub mod rsltcomp2binidx;
#[doc = "RSLTCOMP0SUM (r) register accessor: an alias for `Reg<RSLTCOMP0SUM_SPEC>`"]
pub type RSLTCOMP0SUM = crate::Reg<rsltcomp0sum::RSLTCOMP0SUM_SPEC>;
#[doc = "Sum of all component 0 values."]
pub mod rsltcomp0sum;
#[doc = "RSLTCOMP1SUM (r) register accessor: an alias for `Reg<RSLTCOMP1SUM_SPEC>`"]
pub type RSLTCOMP1SUM = crate::Reg<rsltcomp1sum::RSLTCOMP1SUM_SPEC>;
#[doc = "Sum of all component 1 values."]
pub mod rsltcomp1sum;
#[doc = "RSLTCOMP2SUM (r) register accessor: an alias for `Reg<RSLTCOMP2SUM_SPEC>`"]
pub type RSLTCOMP2SUM = crate::Reg<rsltcomp2sum::RSLTCOMP2SUM_SPEC>;
#[doc = "Sum of all component 2 values."]
pub mod rsltcomp2sum;
#[doc = "RSLTCOMP0BINCNT (rw) register accessor: an alias for `Reg<RSLTCOMP0BINCNT_SPEC>`"]
pub type RSLTCOMP0BINCNT = crate::Reg<rsltcomp0bincnt::RSLTCOMP0BINCNT_SPEC>;
#[doc = "Bin counters of component 0 histogram. This register do NOT support debugger access followed by software access (they clear on read)"]
pub mod rsltcomp0bincnt;
#[doc = "RSLTCOMP1BINCNT (rw) register accessor: an alias for `Reg<RSLTCOMP1BINCNT_SPEC>`"]
pub type RSLTCOMP1BINCNT = crate::Reg<rsltcomp1bincnt::RSLTCOMP1BINCNT_SPEC>;
#[doc = "Bin counters of component 1 histogram.This register do NOT support debugger access followed by software access (they clear on read)"]
pub mod rsltcomp1bincnt;
#[doc = "RSLTCOMP2BINCNT (rw) register accessor: an alias for `Reg<RSLTCOMP2BINCNT_SPEC>`"]
pub type RSLTCOMP2BINCNT = crate::Reg<rsltcomp2bincnt::RSLTCOMP2BINCNT_SPEC>;
#[doc = "Bin counters of component 2 histogram.This register do NOT support debugger access followed by software access (they clear on read)"]
pub mod rsltcomp2bincnt;
#[doc = "FRMSTRTTRIG (w) register accessor: an alias for `Reg<FRMSTRTTRIG_SPEC>`"]
pub type FRMSTRTTRIG = crate::Reg<frmstrttrig::FRMSTRTTRIG_SPEC>;
#[doc = "Triggers for loading shadow registers and starting measurement with next input frame."]
pub mod frmstrttrig;
