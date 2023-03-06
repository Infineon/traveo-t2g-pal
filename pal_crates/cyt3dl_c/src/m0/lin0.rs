#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Error control"]
    pub error_ctl: ERROR_CTL,
    #[doc = "0x04 - Test control"]
    pub test_ctl: TEST_CTL,
    _reserved2: [u8; 0x7ff8],
    #[doc = "0x8000..0x80d0 - LIN channel structure"]
    pub ch0: CH,
    _reserved3: [u8; 0x30],
    #[doc = "0x8100..0x81d0 - LIN channel structure"]
    pub ch1: CH,
}
#[doc = "ERROR_CTL (rw) register accessor: an alias for `Reg<ERROR_CTL_SPEC>`"]
pub type ERROR_CTL = crate::Reg<error_ctl::ERROR_CTL_SPEC>;
#[doc = "Error control"]
pub mod error_ctl;
#[doc = "TEST_CTL (rw) register accessor: an alias for `Reg<TEST_CTL_SPEC>`"]
pub type TEST_CTL = crate::Reg<test_ctl::TEST_CTL_SPEC>;
#[doc = "Test control"]
pub mod test_ctl;
#[doc = "LIN channel structure"]
pub use self::ch::CH;
#[doc = r"Cluster"]
#[doc = "LIN channel structure"]
pub mod ch;
