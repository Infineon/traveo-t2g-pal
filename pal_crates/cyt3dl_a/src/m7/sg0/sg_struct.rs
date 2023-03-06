#[doc = r"Register block"]
#[repr(C)]
pub struct SG_STRUCT {
    #[doc = "0x00 - Source control"]
    pub ctl: CTL,
    _reserved1: [u8; 0x0c],
    #[doc = "0x10 - Interface control"]
    pub if_ctl: IF_CTL,
    _reserved2: [u8; 0x0c],
    #[doc = "0x20 - Time control"]
    pub time_ctl: TIME_CTL,
    #[doc = "0x24 - Tone control"]
    pub tone_ctl: TONE_CTL,
    #[doc = "0x28 - Amplitude control"]
    pub ampl_ctl: AMPL_CTL,
    #[doc = "0x2c - Step control"]
    pub step_ctl: STEP_CTL,
    #[doc = "0x30 - Buffered time control"]
    pub time_ctl_buff: TIME_CTL_BUFF,
    #[doc = "0x34 - Buffered tone control"]
    pub tone_ctl_buff: TONE_CTL_BUFF,
    #[doc = "0x38 - Buffered amplitude control"]
    pub ampl_ctl_buff: AMPL_CTL_BUFF,
    #[doc = "0x3c - Buffered step control"]
    pub step_ctl_buff: STEP_CTL_BUFF,
    _reserved10: [u8; 0x80],
    #[doc = "0xc0 - Interrupt"]
    pub intr_tx: INTR_TX,
    #[doc = "0xc4 - Interrupt set"]
    pub intr_tx_set: INTR_TX_SET,
    #[doc = "0xc8 - Interrupt mask"]
    pub intr_tx_mask: INTR_TX_MASK,
    #[doc = "0xcc - Interrupt masked"]
    pub intr_tx_masked: INTR_TX_MASKED,
}
#[doc = "CTL (rw) register accessor: an alias for `Reg<CTL_SPEC>`"]
pub type CTL = crate::Reg<ctl::CTL_SPEC>;
#[doc = "Source control"]
pub mod ctl;
#[doc = "IF_CTL (rw) register accessor: an alias for `Reg<IF_CTL_SPEC>`"]
pub type IF_CTL = crate::Reg<if_ctl::IF_CTL_SPEC>;
#[doc = "Interface control"]
pub mod if_ctl;
#[doc = "TIME_CTL (rw) register accessor: an alias for `Reg<TIME_CTL_SPEC>`"]
pub type TIME_CTL = crate::Reg<time_ctl::TIME_CTL_SPEC>;
#[doc = "Time control"]
pub mod time_ctl;
#[doc = "TONE_CTL (rw) register accessor: an alias for `Reg<TONE_CTL_SPEC>`"]
pub type TONE_CTL = crate::Reg<tone_ctl::TONE_CTL_SPEC>;
#[doc = "Tone control"]
pub mod tone_ctl;
#[doc = "AMPL_CTL (rw) register accessor: an alias for `Reg<AMPL_CTL_SPEC>`"]
pub type AMPL_CTL = crate::Reg<ampl_ctl::AMPL_CTL_SPEC>;
#[doc = "Amplitude control"]
pub mod ampl_ctl;
#[doc = "STEP_CTL (rw) register accessor: an alias for `Reg<STEP_CTL_SPEC>`"]
pub type STEP_CTL = crate::Reg<step_ctl::STEP_CTL_SPEC>;
#[doc = "Step control"]
pub mod step_ctl;
#[doc = "TIME_CTL_BUFF (rw) register accessor: an alias for `Reg<TIME_CTL_BUFF_SPEC>`"]
pub type TIME_CTL_BUFF = crate::Reg<time_ctl_buff::TIME_CTL_BUFF_SPEC>;
#[doc = "Buffered time control"]
pub mod time_ctl_buff;
#[doc = "TONE_CTL_BUFF (rw) register accessor: an alias for `Reg<TONE_CTL_BUFF_SPEC>`"]
pub type TONE_CTL_BUFF = crate::Reg<tone_ctl_buff::TONE_CTL_BUFF_SPEC>;
#[doc = "Buffered tone control"]
pub mod tone_ctl_buff;
#[doc = "AMPL_CTL_BUFF (rw) register accessor: an alias for `Reg<AMPL_CTL_BUFF_SPEC>`"]
pub type AMPL_CTL_BUFF = crate::Reg<ampl_ctl_buff::AMPL_CTL_BUFF_SPEC>;
#[doc = "Buffered amplitude control"]
pub mod ampl_ctl_buff;
#[doc = "STEP_CTL_BUFF (rw) register accessor: an alias for `Reg<STEP_CTL_BUFF_SPEC>`"]
pub type STEP_CTL_BUFF = crate::Reg<step_ctl_buff::STEP_CTL_BUFF_SPEC>;
#[doc = "Buffered step control"]
pub mod step_ctl_buff;
#[doc = "INTR_TX (rw) register accessor: an alias for `Reg<INTR_TX_SPEC>`"]
pub type INTR_TX = crate::Reg<intr_tx::INTR_TX_SPEC>;
#[doc = "Interrupt"]
pub mod intr_tx;
#[doc = "INTR_TX_SET (rw) register accessor: an alias for `Reg<INTR_TX_SET_SPEC>`"]
pub type INTR_TX_SET = crate::Reg<intr_tx_set::INTR_TX_SET_SPEC>;
#[doc = "Interrupt set"]
pub mod intr_tx_set;
#[doc = "INTR_TX_MASK (rw) register accessor: an alias for `Reg<INTR_TX_MASK_SPEC>`"]
pub type INTR_TX_MASK = crate::Reg<intr_tx_mask::INTR_TX_MASK_SPEC>;
#[doc = "Interrupt mask"]
pub mod intr_tx_mask;
#[doc = "INTR_TX_MASKED (r) register accessor: an alias for `Reg<INTR_TX_MASKED_SPEC>`"]
pub type INTR_TX_MASKED = crate::Reg<intr_tx_masked::INTR_TX_MASKED_SPEC>;
#[doc = "Interrupt masked"]
pub mod intr_tx_masked;
