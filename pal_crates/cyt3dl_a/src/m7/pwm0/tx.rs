#[doc = r"Register block"]
#[repr(C)]
pub struct TX {
    #[doc = "0x00 - Control"]
    pub ctl: CTL,
    _reserved1: [u8; 0x0c],
    #[doc = "0x10 - Interface control"]
    pub if_ctl: IF_CTL,
    #[doc = "0x14 - Double control"]
    pub double_ctl: DOUBLE_CTL,
    #[doc = "0x18 - Gain control"]
    pub gain_ctl: GAIN_CTL,
    _reserved4: [u8; 0x04],
    #[doc = "0x20 - PWM control 0"]
    pub pwm_ctl0: PWM_CTL0,
    #[doc = "0x24 - PWM control 1"]
    pub pwm_ctl1: PWM_CTL1,
    #[doc = "0x28 - PWM control 2"]
    pub pwm_ctl2: PWM_CTL2,
    #[doc = "0x2c - PWM control 3"]
    pub pwm_ctl3: PWM_CTL3,
    _reserved8: [u8; 0x50],
    #[doc = "0x80 - TX FIFO control"]
    pub tx_fifo_ctl: TX_FIFO_CTL,
    #[doc = "0x84 - TX FIFO status"]
    pub tx_fifo_status: TX_FIFO_STATUS,
    #[doc = "0x88 - TX FIFO write"]
    pub tx_fifo_wr: TX_FIFO_WR,
    _reserved11: [u8; 0x34],
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
#[doc = "Control"]
pub mod ctl;
#[doc = "IF_CTL (rw) register accessor: an alias for `Reg<IF_CTL_SPEC>`"]
pub type IF_CTL = crate::Reg<if_ctl::IF_CTL_SPEC>;
#[doc = "Interface control"]
pub mod if_ctl;
#[doc = "DOUBLE_CTL (rw) register accessor: an alias for `Reg<DOUBLE_CTL_SPEC>`"]
pub type DOUBLE_CTL = crate::Reg<double_ctl::DOUBLE_CTL_SPEC>;
#[doc = "Double control"]
pub mod double_ctl;
#[doc = "GAIN_CTL (rw) register accessor: an alias for `Reg<GAIN_CTL_SPEC>`"]
pub type GAIN_CTL = crate::Reg<gain_ctl::GAIN_CTL_SPEC>;
#[doc = "Gain control"]
pub mod gain_ctl;
#[doc = "PWM_CTL0 (rw) register accessor: an alias for `Reg<PWM_CTL0_SPEC>`"]
pub type PWM_CTL0 = crate::Reg<pwm_ctl0::PWM_CTL0_SPEC>;
#[doc = "PWM control 0"]
pub mod pwm_ctl0;
#[doc = "PWM_CTL1 (rw) register accessor: an alias for `Reg<PWM_CTL1_SPEC>`"]
pub type PWM_CTL1 = crate::Reg<pwm_ctl1::PWM_CTL1_SPEC>;
#[doc = "PWM control 1"]
pub mod pwm_ctl1;
#[doc = "PWM_CTL2 (rw) register accessor: an alias for `Reg<PWM_CTL2_SPEC>`"]
pub type PWM_CTL2 = crate::Reg<pwm_ctl2::PWM_CTL2_SPEC>;
#[doc = "PWM control 2"]
pub mod pwm_ctl2;
#[doc = "PWM_CTL3 (rw) register accessor: an alias for `Reg<PWM_CTL3_SPEC>`"]
pub type PWM_CTL3 = crate::Reg<pwm_ctl3::PWM_CTL3_SPEC>;
#[doc = "PWM control 3"]
pub mod pwm_ctl3;
#[doc = "TX_FIFO_CTL (rw) register accessor: an alias for `Reg<TX_FIFO_CTL_SPEC>`"]
pub type TX_FIFO_CTL = crate::Reg<tx_fifo_ctl::TX_FIFO_CTL_SPEC>;
#[doc = "TX FIFO control"]
pub mod tx_fifo_ctl;
#[doc = "TX_FIFO_STATUS (r) register accessor: an alias for `Reg<TX_FIFO_STATUS_SPEC>`"]
pub type TX_FIFO_STATUS = crate::Reg<tx_fifo_status::TX_FIFO_STATUS_SPEC>;
#[doc = "TX FIFO status"]
pub mod tx_fifo_status;
#[doc = "TX_FIFO_WR (w) register accessor: an alias for `Reg<TX_FIFO_WR_SPEC>`"]
pub type TX_FIFO_WR = crate::Reg<tx_fifo_wr::TX_FIFO_WR_SPEC>;
#[doc = "TX FIFO write"]
pub mod tx_fifo_wr;
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
