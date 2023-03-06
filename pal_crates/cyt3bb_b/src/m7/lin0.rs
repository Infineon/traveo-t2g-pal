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
    _reserved4: [u8; 0x30],
    #[doc = "0x8200..0x82d0 - LIN channel structure"]
    pub ch2: CH,
    _reserved5: [u8; 0x30],
    #[doc = "0x8300..0x83d0 - LIN channel structure"]
    pub ch3: CH,
    _reserved6: [u8; 0x30],
    #[doc = "0x8400..0x84d0 - LIN channel structure"]
    pub ch4: CH,
    _reserved7: [u8; 0x30],
    #[doc = "0x8500..0x85d0 - LIN channel structure"]
    pub ch5: CH,
    _reserved8: [u8; 0x30],
    #[doc = "0x8600..0x86d0 - LIN channel structure"]
    pub ch6: CH,
    _reserved9: [u8; 0x30],
    #[doc = "0x8700..0x87d0 - LIN channel structure"]
    pub ch7: CH,
    _reserved10: [u8; 0x30],
    #[doc = "0x8800..0x88d0 - LIN channel structure"]
    pub ch8: CH,
    _reserved11: [u8; 0x30],
    #[doc = "0x8900..0x89d0 - LIN channel structure"]
    pub ch9: CH,
    _reserved12: [u8; 0x30],
    #[doc = "0x8a00..0x8ad0 - LIN channel structure"]
    pub ch10: CH,
    _reserved13: [u8; 0x30],
    #[doc = "0x8b00..0x8bd0 - LIN channel structure"]
    pub ch11: CH,
    _reserved14: [u8; 0x30],
    #[doc = "0x8c00..0x8cd0 - LIN channel structure"]
    pub ch12: CH,
    _reserved15: [u8; 0x30],
    #[doc = "0x8d00..0x8dd0 - LIN channel structure"]
    pub ch13: CH,
    _reserved16: [u8; 0x30],
    #[doc = "0x8e00..0x8ed0 - LIN channel structure"]
    pub ch14: CH,
    _reserved17: [u8; 0x30],
    #[doc = "0x8f00..0x8fd0 - LIN channel structure"]
    pub ch15: CH,
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
