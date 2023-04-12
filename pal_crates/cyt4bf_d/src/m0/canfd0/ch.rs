#[doc = r"Register block"]
#[repr(C)]
pub struct CH {
    #[doc = "0x00..0x144 - TTCAN 3PIP, includes FD"]
    pub m_ttcan: M_TTCAN,
    _reserved1: [u8; 0x3c],
    #[doc = "0x180 - Receive FIFO Top control"]
    pub rxftop_ctl: RXFTOP_CTL,
    _reserved2: [u8; 0x1c],
    #[doc = "0x1a0 - Receive FIFO 0 Top Status"]
    pub rxftop0_stat: RXFTOP0_STAT,
    _reserved3: [u8; 0x04],
    #[doc = "0x1a8 - Receive FIFO 0 Top Data"]
    pub rxftop0_data: RXFTOP0_DATA,
    _reserved4: [u8; 0x04],
    #[doc = "0x1b0 - Receive FIFO 1 Top Status"]
    pub rxftop1_stat: RXFTOP1_STAT,
    _reserved5: [u8; 0x04],
    #[doc = "0x1b8 - Receive FIFO 1 Top Data"]
    pub rxftop1_data: RXFTOP1_DATA,
}
#[doc = "TTCAN 3PIP, includes FD"]
pub use self::m_ttcan::M_TTCAN;
#[doc = r"Cluster"]
#[doc = "TTCAN 3PIP, includes FD"]
pub mod m_ttcan;
#[doc = "RXFTOP_CTL (rw) register accessor: an alias for `Reg<RXFTOP_CTL_SPEC>`"]
pub type RXFTOP_CTL = crate::Reg<rxftop_ctl::RXFTOP_CTL_SPEC>;
#[doc = "Receive FIFO Top control"]
pub mod rxftop_ctl;
#[doc = "RXFTOP0_STAT (r) register accessor: an alias for `Reg<RXFTOP0_STAT_SPEC>`"]
pub type RXFTOP0_STAT = crate::Reg<rxftop0_stat::RXFTOP0_STAT_SPEC>;
#[doc = "Receive FIFO 0 Top Status"]
pub mod rxftop0_stat;
#[doc = "RXFTOP0_DATA (r) register accessor: an alias for `Reg<RXFTOP0_DATA_SPEC>`"]
pub type RXFTOP0_DATA = crate::Reg<rxftop0_data::RXFTOP0_DATA_SPEC>;
#[doc = "Receive FIFO 0 Top Data"]
pub mod rxftop0_data;
#[doc = "RXFTOP1_STAT (r) register accessor: an alias for `Reg<RXFTOP1_STAT_SPEC>`"]
pub type RXFTOP1_STAT = crate::Reg<rxftop1_stat::RXFTOP1_STAT_SPEC>;
#[doc = "Receive FIFO 1 Top Status"]
pub mod rxftop1_stat;
#[doc = "RXFTOP1_DATA (r) register accessor: an alias for `Reg<RXFTOP1_DATA_SPEC>`"]
pub type RXFTOP1_DATA = crate::Reg<rxftop1_data::RXFTOP1_DATA_SPEC>;
#[doc = "Receive FIFO 1 Top Data"]
pub mod rxftop1_data;
