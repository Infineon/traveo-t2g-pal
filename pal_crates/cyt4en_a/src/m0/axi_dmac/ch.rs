#[doc = r"Register block"]
#[repr(C)]
pub struct CH {
    #[doc = "0x00 - Channel control"]
    pub ctl: CTL,
    #[doc = "0x04 - Channel status"]
    pub status: STATUS,
    _reserved2: [u8; 0x08],
    #[doc = "0x10 - Channel current X and Y indices"]
    pub idx: IDX,
    #[doc = "0x14 - Channel current source address"]
    pub src: SRC,
    #[doc = "0x18 - Channel current destination address"]
    pub dst: DST,
    #[doc = "0x1c - Channel current M index"]
    pub m_idx: M_IDX,
    #[doc = "0x20 - Channel current descriptor pointer"]
    pub curr: CURR,
    _reserved7: [u8; 0x04],
    #[doc = "0x28 - Channel software trigger"]
    pub tr_cmd: TR_CMD,
    _reserved8: [u8; 0x14],
    #[doc = "0x40 - Channel descriptor status"]
    pub descr_status: DESCR_STATUS,
    _reserved9: [u8; 0x0c],
    #[doc = "0x50 - Channel descriptor control"]
    pub descr_ctl: DESCR_CTL,
    #[doc = "0x54 - Channel descriptor source"]
    pub descr_src: DESCR_SRC,
    #[doc = "0x58 - Channel descriptor destination"]
    pub descr_dst: DESCR_DST,
    #[doc = "0x5c - Channel descriptor M size"]
    pub descr_m_size: DESCR_M_SIZE,
    #[doc = "0x60 - Channel descriptor X size"]
    pub descr_x_size: DESCR_X_SIZE,
    #[doc = "0x64 - Channel descriptor X increment"]
    pub descr_x_incr: DESCR_X_INCR,
    #[doc = "0x68 - Channel descriptor Y size"]
    pub descr_y_size: DESCR_Y_SIZE,
    #[doc = "0x6c - Channel descriptor Y increment"]
    pub descr_y_incr: DESCR_Y_INCR,
    #[doc = "0x70 - Channel descriptor next pointer"]
    pub descr_next: DESCR_NEXT,
    _reserved18: [u8; 0x0c],
    #[doc = "0x80 - Interrupt"]
    pub intr: INTR,
    #[doc = "0x84 - Interrupt set"]
    pub intr_set: INTR_SET,
    #[doc = "0x88 - Interrupt mask"]
    pub intr_mask: INTR_MASK,
    #[doc = "0x8c - Interrupt masked"]
    pub intr_masked: INTR_MASKED,
}
#[doc = "CTL (rw) register accessor: an alias for `Reg<CTL_SPEC>`"]
pub type CTL = crate::Reg<ctl::CTL_SPEC>;
#[doc = "Channel control"]
pub mod ctl;
#[doc = "STATUS (r) register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Channel status"]
pub mod status;
#[doc = "IDX (r) register accessor: an alias for `Reg<IDX_SPEC>`"]
pub type IDX = crate::Reg<idx::IDX_SPEC>;
#[doc = "Channel current X and Y indices"]
pub mod idx;
#[doc = "SRC (r) register accessor: an alias for `Reg<SRC_SPEC>`"]
pub type SRC = crate::Reg<src::SRC_SPEC>;
#[doc = "Channel current source address"]
pub mod src;
#[doc = "DST (r) register accessor: an alias for `Reg<DST_SPEC>`"]
pub type DST = crate::Reg<dst::DST_SPEC>;
#[doc = "Channel current destination address"]
pub mod dst;
#[doc = "M_IDX (r) register accessor: an alias for `Reg<M_IDX_SPEC>`"]
pub type M_IDX = crate::Reg<m_idx::M_IDX_SPEC>;
#[doc = "Channel current M index"]
pub mod m_idx;
#[doc = "CURR (rw) register accessor: an alias for `Reg<CURR_SPEC>`"]
pub type CURR = crate::Reg<curr::CURR_SPEC>;
#[doc = "Channel current descriptor pointer"]
pub mod curr;
#[doc = "TR_CMD (rw) register accessor: an alias for `Reg<TR_CMD_SPEC>`"]
pub type TR_CMD = crate::Reg<tr_cmd::TR_CMD_SPEC>;
#[doc = "Channel software trigger"]
pub mod tr_cmd;
#[doc = "DESCR_STATUS (r) register accessor: an alias for `Reg<DESCR_STATUS_SPEC>`"]
pub type DESCR_STATUS = crate::Reg<descr_status::DESCR_STATUS_SPEC>;
#[doc = "Channel descriptor status"]
pub mod descr_status;
#[doc = "DESCR_CTL (r) register accessor: an alias for `Reg<DESCR_CTL_SPEC>`"]
pub type DESCR_CTL = crate::Reg<descr_ctl::DESCR_CTL_SPEC>;
#[doc = "Channel descriptor control"]
pub mod descr_ctl;
#[doc = "DESCR_SRC (r) register accessor: an alias for `Reg<DESCR_SRC_SPEC>`"]
pub type DESCR_SRC = crate::Reg<descr_src::DESCR_SRC_SPEC>;
#[doc = "Channel descriptor source"]
pub mod descr_src;
#[doc = "DESCR_DST (r) register accessor: an alias for `Reg<DESCR_DST_SPEC>`"]
pub type DESCR_DST = crate::Reg<descr_dst::DESCR_DST_SPEC>;
#[doc = "Channel descriptor destination"]
pub mod descr_dst;
#[doc = "DESCR_M_SIZE (r) register accessor: an alias for `Reg<DESCR_M_SIZE_SPEC>`"]
pub type DESCR_M_SIZE = crate::Reg<descr_m_size::DESCR_M_SIZE_SPEC>;
#[doc = "Channel descriptor M size"]
pub mod descr_m_size;
#[doc = "DESCR_X_SIZE (r) register accessor: an alias for `Reg<DESCR_X_SIZE_SPEC>`"]
pub type DESCR_X_SIZE = crate::Reg<descr_x_size::DESCR_X_SIZE_SPEC>;
#[doc = "Channel descriptor X size"]
pub mod descr_x_size;
#[doc = "DESCR_X_INCR (r) register accessor: an alias for `Reg<DESCR_X_INCR_SPEC>`"]
pub type DESCR_X_INCR = crate::Reg<descr_x_incr::DESCR_X_INCR_SPEC>;
#[doc = "Channel descriptor X increment"]
pub mod descr_x_incr;
#[doc = "DESCR_Y_SIZE (r) register accessor: an alias for `Reg<DESCR_Y_SIZE_SPEC>`"]
pub type DESCR_Y_SIZE = crate::Reg<descr_y_size::DESCR_Y_SIZE_SPEC>;
#[doc = "Channel descriptor Y size"]
pub mod descr_y_size;
#[doc = "DESCR_Y_INCR (r) register accessor: an alias for `Reg<DESCR_Y_INCR_SPEC>`"]
pub type DESCR_Y_INCR = crate::Reg<descr_y_incr::DESCR_Y_INCR_SPEC>;
#[doc = "Channel descriptor Y increment"]
pub mod descr_y_incr;
#[doc = "DESCR_NEXT (r) register accessor: an alias for `Reg<DESCR_NEXT_SPEC>`"]
pub type DESCR_NEXT = crate::Reg<descr_next::DESCR_NEXT_SPEC>;
#[doc = "Channel descriptor next pointer"]
pub mod descr_next;
#[doc = "INTR (rw) register accessor: an alias for `Reg<INTR_SPEC>`"]
pub type INTR = crate::Reg<intr::INTR_SPEC>;
#[doc = "Interrupt"]
pub mod intr;
#[doc = "INTR_SET (rw) register accessor: an alias for `Reg<INTR_SET_SPEC>`"]
pub type INTR_SET = crate::Reg<intr_set::INTR_SET_SPEC>;
#[doc = "Interrupt set"]
pub mod intr_set;
#[doc = "INTR_MASK (rw) register accessor: an alias for `Reg<INTR_MASK_SPEC>`"]
pub type INTR_MASK = crate::Reg<intr_mask::INTR_MASK_SPEC>;
#[doc = "Interrupt mask"]
pub mod intr_mask;
#[doc = "INTR_MASKED (r) register accessor: an alias for `Reg<INTR_MASKED_SPEC>`"]
pub type INTR_MASKED = crate::Reg<intr_masked::INTR_MASKED_SPEC>;
#[doc = "Interrupt masked"]
pub mod intr_masked;
