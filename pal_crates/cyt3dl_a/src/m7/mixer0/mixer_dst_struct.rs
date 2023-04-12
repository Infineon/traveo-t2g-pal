#[doc = r"Register block"]
#[repr(C)]
pub struct MIXER_DST_STRUCT {
    #[doc = "0x00 - Destination control"]
    pub dst_ctl: DST_CTL,
    _reserved1: [u8; 0x10],
    #[doc = "0x14 - Destination fade control"]
    pub dst_fade_ctl: DST_FADE_CTL,
    #[doc = "0x18 - Destination fade status"]
    pub dst_fade_status: DST_FADE_STATUS,
    #[doc = "0x1c - Destination fade command"]
    pub dst_fade_cmd: DST_FADE_CMD,
    #[doc = "0x20 - Destination gain control"]
    pub dst_gain_ctl: DST_GAIN_CTL,
    _reserved5: [u8; 0x5c],
    #[doc = "0x80 - Destination FIFO control"]
    pub dst_fifo_ctl: DST_FIFO_CTL,
    #[doc = "0x84 - Destination FIFO status"]
    pub dst_fifo_status: DST_FIFO_STATUS,
    #[doc = "0x88 - Destination FIFO read"]
    pub dst_fifo_rd: DST_FIFO_RD,
    #[doc = "0x8c - Destination FIFO silent read"]
    pub dst_fifo_rd_silent: DST_FIFO_RD_SILENT,
    _reserved9: [u8; 0x30],
    #[doc = "0xc0 - Interrupt"]
    pub intr_dst: INTR_DST,
    #[doc = "0xc4 - Interrupt set"]
    pub intr_dst_set: INTR_DST_SET,
    #[doc = "0xc8 - Interrupt mask"]
    pub intr_dst_mask: INTR_DST_MASK,
    #[doc = "0xcc - Interrupt masked"]
    pub intr_dst_masked: INTR_DST_MASKED,
}
#[doc = "DST_CTL (rw) register accessor: an alias for `Reg<DST_CTL_SPEC>`"]
pub type DST_CTL = crate::Reg<dst_ctl::DST_CTL_SPEC>;
#[doc = "Destination control"]
pub mod dst_ctl;
#[doc = "DST_FADE_CTL (rw) register accessor: an alias for `Reg<DST_FADE_CTL_SPEC>`"]
pub type DST_FADE_CTL = crate::Reg<dst_fade_ctl::DST_FADE_CTL_SPEC>;
#[doc = "Destination fade control"]
pub mod dst_fade_ctl;
#[doc = "DST_FADE_STATUS (r) register accessor: an alias for `Reg<DST_FADE_STATUS_SPEC>`"]
pub type DST_FADE_STATUS = crate::Reg<dst_fade_status::DST_FADE_STATUS_SPEC>;
#[doc = "Destination fade status"]
pub mod dst_fade_status;
#[doc = "DST_FADE_CMD (rw) register accessor: an alias for `Reg<DST_FADE_CMD_SPEC>`"]
pub type DST_FADE_CMD = crate::Reg<dst_fade_cmd::DST_FADE_CMD_SPEC>;
#[doc = "Destination fade command"]
pub mod dst_fade_cmd;
#[doc = "DST_GAIN_CTL (rw) register accessor: an alias for `Reg<DST_GAIN_CTL_SPEC>`"]
pub type DST_GAIN_CTL = crate::Reg<dst_gain_ctl::DST_GAIN_CTL_SPEC>;
#[doc = "Destination gain control"]
pub mod dst_gain_ctl;
#[doc = "DST_FIFO_CTL (rw) register accessor: an alias for `Reg<DST_FIFO_CTL_SPEC>`"]
pub type DST_FIFO_CTL = crate::Reg<dst_fifo_ctl::DST_FIFO_CTL_SPEC>;
#[doc = "Destination FIFO control"]
pub mod dst_fifo_ctl;
#[doc = "DST_FIFO_STATUS (r) register accessor: an alias for `Reg<DST_FIFO_STATUS_SPEC>`"]
pub type DST_FIFO_STATUS = crate::Reg<dst_fifo_status::DST_FIFO_STATUS_SPEC>;
#[doc = "Destination FIFO status"]
pub mod dst_fifo_status;
#[doc = "DST_FIFO_RD (r) register accessor: an alias for `Reg<DST_FIFO_RD_SPEC>`"]
pub type DST_FIFO_RD = crate::Reg<dst_fifo_rd::DST_FIFO_RD_SPEC>;
#[doc = "Destination FIFO read"]
pub mod dst_fifo_rd;
#[doc = "DST_FIFO_RD_SILENT (r) register accessor: an alias for `Reg<DST_FIFO_RD_SILENT_SPEC>`"]
pub type DST_FIFO_RD_SILENT = crate::Reg<dst_fifo_rd_silent::DST_FIFO_RD_SILENT_SPEC>;
#[doc = "Destination FIFO silent read"]
pub mod dst_fifo_rd_silent;
#[doc = "INTR_DST (rw) register accessor: an alias for `Reg<INTR_DST_SPEC>`"]
pub type INTR_DST = crate::Reg<intr_dst::INTR_DST_SPEC>;
#[doc = "Interrupt"]
pub mod intr_dst;
#[doc = "INTR_DST_SET (rw) register accessor: an alias for `Reg<INTR_DST_SET_SPEC>`"]
pub type INTR_DST_SET = crate::Reg<intr_dst_set::INTR_DST_SET_SPEC>;
#[doc = "Interrupt set"]
pub mod intr_dst_set;
#[doc = "INTR_DST_MASK (rw) register accessor: an alias for `Reg<INTR_DST_MASK_SPEC>`"]
pub type INTR_DST_MASK = crate::Reg<intr_dst_mask::INTR_DST_MASK_SPEC>;
#[doc = "Interrupt mask"]
pub mod intr_dst_mask;
#[doc = "INTR_DST_MASKED (r) register accessor: an alias for `Reg<INTR_DST_MASKED_SPEC>`"]
pub type INTR_DST_MASKED = crate::Reg<intr_dst_masked::INTR_DST_MASKED_SPEC>;
#[doc = "Interrupt masked"]
pub mod intr_dst_masked;
