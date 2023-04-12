#[doc = r"Register block"]
#[repr(C)]
pub struct CNT {
    #[doc = "0x00 - Counter control register"]
    pub ctrl: CTRL,
    #[doc = "0x04 - Counter status register"]
    pub status: STATUS,
    #[doc = "0x08 - Counter count register"]
    pub counter: COUNTER,
    _reserved3: [u8; 0x04],
    #[doc = "0x10 - Counter compare/capture 0 register"]
    pub cc0: CC0,
    #[doc = "0x14 - Counter buffered compare/capture 0 register"]
    pub cc0_buff: CC0_BUFF,
    #[doc = "0x18 - Counter compare/capture 1 register"]
    pub cc1: CC1,
    #[doc = "0x1c - Counter buffered compare/capture 1 register"]
    pub cc1_buff: CC1_BUFF,
    #[doc = "0x20 - Counter period register"]
    pub period: PERIOD,
    #[doc = "0x24 - Counter buffered period register"]
    pub period_buff: PERIOD_BUFF,
    #[doc = "0x28 - Counter line selection register"]
    pub line_sel: LINE_SEL,
    #[doc = "0x2c - Counter buffered line selection register"]
    pub line_sel_buff: LINE_SEL_BUFF,
    #[doc = "0x30 - Counter PWM dead time register"]
    pub dt: DT,
    _reserved12: [u8; 0x0c],
    #[doc = "0x40 - Counter trigger command register"]
    pub tr_cmd: TR_CMD,
    #[doc = "0x44 - Counter input trigger selection register 0"]
    pub tr_in_sel0: TR_IN_SEL0,
    #[doc = "0x48 - Counter input trigger selection register 1"]
    pub tr_in_sel1: TR_IN_SEL1,
    #[doc = "0x4c - Counter input trigger edge selection register"]
    pub tr_in_edge_sel: TR_IN_EDGE_SEL,
    #[doc = "0x50 - Counter trigger PWM control register"]
    pub tr_pwm_ctrl: TR_PWM_CTRL,
    #[doc = "0x54 - Counter output trigger selection register"]
    pub tr_out_sel: TR_OUT_SEL,
    _reserved18: [u8; 0x18],
    #[doc = "0x70 - Interrupt request register"]
    pub intr: INTR,
    #[doc = "0x74 - Interrupt set request register"]
    pub intr_set: INTR_SET,
    #[doc = "0x78 - Interrupt mask register"]
    pub intr_mask: INTR_MASK,
    #[doc = "0x7c - Interrupt masked request register"]
    pub intr_masked: INTR_MASKED,
}
#[doc = "CTRL (rw) register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Counter control register"]
pub mod ctrl;
#[doc = "STATUS (r) register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Counter status register"]
pub mod status;
#[doc = "COUNTER (rw) register accessor: an alias for `Reg<COUNTER_SPEC>`"]
pub type COUNTER = crate::Reg<counter::COUNTER_SPEC>;
#[doc = "Counter count register"]
pub mod counter;
#[doc = "CC0 (rw) register accessor: an alias for `Reg<CC0_SPEC>`"]
pub type CC0 = crate::Reg<cc0::CC0_SPEC>;
#[doc = "Counter compare/capture 0 register"]
pub mod cc0;
#[doc = "CC0_BUFF (rw) register accessor: an alias for `Reg<CC0_BUFF_SPEC>`"]
pub type CC0_BUFF = crate::Reg<cc0_buff::CC0_BUFF_SPEC>;
#[doc = "Counter buffered compare/capture 0 register"]
pub mod cc0_buff;
#[doc = "CC1 (rw) register accessor: an alias for `Reg<CC1_SPEC>`"]
pub type CC1 = crate::Reg<cc1::CC1_SPEC>;
#[doc = "Counter compare/capture 1 register"]
pub mod cc1;
#[doc = "CC1_BUFF (rw) register accessor: an alias for `Reg<CC1_BUFF_SPEC>`"]
pub type CC1_BUFF = crate::Reg<cc1_buff::CC1_BUFF_SPEC>;
#[doc = "Counter buffered compare/capture 1 register"]
pub mod cc1_buff;
#[doc = "PERIOD (rw) register accessor: an alias for `Reg<PERIOD_SPEC>`"]
pub type PERIOD = crate::Reg<period::PERIOD_SPEC>;
#[doc = "Counter period register"]
pub mod period;
#[doc = "PERIOD_BUFF (rw) register accessor: an alias for `Reg<PERIOD_BUFF_SPEC>`"]
pub type PERIOD_BUFF = crate::Reg<period_buff::PERIOD_BUFF_SPEC>;
#[doc = "Counter buffered period register"]
pub mod period_buff;
#[doc = "LINE_SEL (rw) register accessor: an alias for `Reg<LINE_SEL_SPEC>`"]
pub type LINE_SEL = crate::Reg<line_sel::LINE_SEL_SPEC>;
#[doc = "Counter line selection register"]
pub mod line_sel;
#[doc = "LINE_SEL_BUFF (rw) register accessor: an alias for `Reg<LINE_SEL_BUFF_SPEC>`"]
pub type LINE_SEL_BUFF = crate::Reg<line_sel_buff::LINE_SEL_BUFF_SPEC>;
#[doc = "Counter buffered line selection register"]
pub mod line_sel_buff;
#[doc = "DT (rw) register accessor: an alias for `Reg<DT_SPEC>`"]
pub type DT = crate::Reg<dt::DT_SPEC>;
#[doc = "Counter PWM dead time register"]
pub mod dt;
#[doc = "TR_CMD (rw) register accessor: an alias for `Reg<TR_CMD_SPEC>`"]
pub type TR_CMD = crate::Reg<tr_cmd::TR_CMD_SPEC>;
#[doc = "Counter trigger command register"]
pub mod tr_cmd;
#[doc = "TR_IN_SEL0 (rw) register accessor: an alias for `Reg<TR_IN_SEL0_SPEC>`"]
pub type TR_IN_SEL0 = crate::Reg<tr_in_sel0::TR_IN_SEL0_SPEC>;
#[doc = "Counter input trigger selection register 0"]
pub mod tr_in_sel0;
#[doc = "TR_IN_SEL1 (rw) register accessor: an alias for `Reg<TR_IN_SEL1_SPEC>`"]
pub type TR_IN_SEL1 = crate::Reg<tr_in_sel1::TR_IN_SEL1_SPEC>;
#[doc = "Counter input trigger selection register 1"]
pub mod tr_in_sel1;
#[doc = "TR_IN_EDGE_SEL (rw) register accessor: an alias for `Reg<TR_IN_EDGE_SEL_SPEC>`"]
pub type TR_IN_EDGE_SEL = crate::Reg<tr_in_edge_sel::TR_IN_EDGE_SEL_SPEC>;
#[doc = "Counter input trigger edge selection register"]
pub mod tr_in_edge_sel;
#[doc = "TR_PWM_CTRL (rw) register accessor: an alias for `Reg<TR_PWM_CTRL_SPEC>`"]
pub type TR_PWM_CTRL = crate::Reg<tr_pwm_ctrl::TR_PWM_CTRL_SPEC>;
#[doc = "Counter trigger PWM control register"]
pub mod tr_pwm_ctrl;
#[doc = "TR_OUT_SEL (rw) register accessor: an alias for `Reg<TR_OUT_SEL_SPEC>`"]
pub type TR_OUT_SEL = crate::Reg<tr_out_sel::TR_OUT_SEL_SPEC>;
#[doc = "Counter output trigger selection register"]
pub mod tr_out_sel;
#[doc = "INTR (rw) register accessor: an alias for `Reg<INTR_SPEC>`"]
pub type INTR = crate::Reg<intr::INTR_SPEC>;
#[doc = "Interrupt request register"]
pub mod intr;
#[doc = "INTR_SET (rw) register accessor: an alias for `Reg<INTR_SET_SPEC>`"]
pub type INTR_SET = crate::Reg<intr_set::INTR_SET_SPEC>;
#[doc = "Interrupt set request register"]
pub mod intr_set;
#[doc = "INTR_MASK (rw) register accessor: an alias for `Reg<INTR_MASK_SPEC>`"]
pub type INTR_MASK = crate::Reg<intr_mask::INTR_MASK_SPEC>;
#[doc = "Interrupt mask register"]
pub mod intr_mask;
#[doc = "INTR_MASKED (r) register accessor: an alias for `Reg<INTR_MASKED_SPEC>`"]
pub type INTR_MASKED = crate::Reg<intr_masked::INTR_MASKED_SPEC>;
#[doc = "Interrupt masked request register"]
pub mod intr_masked;
