#[doc = r"Register block"]
#[repr(C)]
pub struct MIXER_SRC_STRUCT {
    #[doc = "0x00 - Source control"]
    pub src_ctl: SRC_CTL,
    #[doc = "0x04 - Source status"]
    pub src_status: SRC_STATUS,
    _reserved2: [u8; 0x0c],
    #[doc = "0x14 - Source fade control"]
    pub src_fade_ctl: SRC_FADE_CTL,
    #[doc = "0x18 - Source fade status"]
    pub src_fade_status: SRC_FADE_STATUS,
    #[doc = "0x1c - Source fade command"]
    pub src_fade_cmd: SRC_FADE_CMD,
    #[doc = "0x20 - Source gain control"]
    pub src_gain_ctl: SRC_GAIN_CTL,
    _reserved6: [u8; 0x5c],
    #[doc = "0x80 - Source FIFO control"]
    pub src_fifo_ctl: SRC_FIFO_CTL,
    #[doc = "0x84 - Source FIFO status"]
    pub src_fifo_status: SRC_FIFO_STATUS,
    #[doc = "0x88 - Source FIFO write"]
    pub src_fifo_wr: SRC_FIFO_WR,
    _reserved9: [u8; 0x34],
    #[doc = "0xc0 - Interrupt"]
    pub intr_src: INTR_SRC,
    #[doc = "0xc4 - Interrupt set"]
    pub intr_src_set: INTR_SRC_SET,
    #[doc = "0xc8 - Interrupt mask"]
    pub intr_src_mask: INTR_SRC_MASK,
    #[doc = "0xcc - Interrupt masked"]
    pub intr_src_masked: INTR_SRC_MASKED,
}
#[doc = "SRC_CTL (rw) register accessor: an alias for `Reg<SRC_CTL_SPEC>`"]
pub type SRC_CTL = crate::Reg<src_ctl::SRC_CTL_SPEC>;
#[doc = "Source control"]
pub mod src_ctl;
#[doc = "SRC_STATUS (r) register accessor: an alias for `Reg<SRC_STATUS_SPEC>`"]
pub type SRC_STATUS = crate::Reg<src_status::SRC_STATUS_SPEC>;
#[doc = "Source status"]
pub mod src_status;
#[doc = "SRC_FADE_CTL (rw) register accessor: an alias for `Reg<SRC_FADE_CTL_SPEC>`"]
pub type SRC_FADE_CTL = crate::Reg<src_fade_ctl::SRC_FADE_CTL_SPEC>;
#[doc = "Source fade control"]
pub mod src_fade_ctl;
#[doc = "SRC_FADE_STATUS (r) register accessor: an alias for `Reg<SRC_FADE_STATUS_SPEC>`"]
pub type SRC_FADE_STATUS = crate::Reg<src_fade_status::SRC_FADE_STATUS_SPEC>;
#[doc = "Source fade status"]
pub mod src_fade_status;
#[doc = "SRC_FADE_CMD (rw) register accessor: an alias for `Reg<SRC_FADE_CMD_SPEC>`"]
pub type SRC_FADE_CMD = crate::Reg<src_fade_cmd::SRC_FADE_CMD_SPEC>;
#[doc = "Source fade command"]
pub mod src_fade_cmd;
#[doc = "SRC_GAIN_CTL (rw) register accessor: an alias for `Reg<SRC_GAIN_CTL_SPEC>`"]
pub type SRC_GAIN_CTL = crate::Reg<src_gain_ctl::SRC_GAIN_CTL_SPEC>;
#[doc = "Source gain control"]
pub mod src_gain_ctl;
#[doc = "SRC_FIFO_CTL (rw) register accessor: an alias for `Reg<SRC_FIFO_CTL_SPEC>`"]
pub type SRC_FIFO_CTL = crate::Reg<src_fifo_ctl::SRC_FIFO_CTL_SPEC>;
#[doc = "Source FIFO control"]
pub mod src_fifo_ctl;
#[doc = "SRC_FIFO_STATUS (r) register accessor: an alias for `Reg<SRC_FIFO_STATUS_SPEC>`"]
pub type SRC_FIFO_STATUS = crate::Reg<src_fifo_status::SRC_FIFO_STATUS_SPEC>;
#[doc = "Source FIFO status"]
pub mod src_fifo_status;
#[doc = "SRC_FIFO_WR (w) register accessor: an alias for `Reg<SRC_FIFO_WR_SPEC>`"]
pub type SRC_FIFO_WR = crate::Reg<src_fifo_wr::SRC_FIFO_WR_SPEC>;
#[doc = "Source FIFO write"]
pub mod src_fifo_wr;
#[doc = "INTR_SRC (rw) register accessor: an alias for `Reg<INTR_SRC_SPEC>`"]
pub type INTR_SRC = crate::Reg<intr_src::INTR_SRC_SPEC>;
#[doc = "Interrupt"]
pub mod intr_src;
#[doc = "INTR_SRC_SET (rw) register accessor: an alias for `Reg<INTR_SRC_SET_SPEC>`"]
pub type INTR_SRC_SET = crate::Reg<intr_src_set::INTR_SRC_SET_SPEC>;
#[doc = "Interrupt set"]
pub mod intr_src_set;
#[doc = "INTR_SRC_MASK (rw) register accessor: an alias for `Reg<INTR_SRC_MASK_SPEC>`"]
pub type INTR_SRC_MASK = crate::Reg<intr_src_mask::INTR_SRC_MASK_SPEC>;
#[doc = "Interrupt mask"]
pub mod intr_src_mask;
#[doc = "INTR_SRC_MASKED (r) register accessor: an alias for `Reg<INTR_SRC_MASKED_SPEC>`"]
pub type INTR_SRC_MASKED = crate::Reg<intr_src_masked::INTR_SRC_MASKED_SPEC>;
#[doc = "Interrupt masked"]
pub mod intr_src_masked;
