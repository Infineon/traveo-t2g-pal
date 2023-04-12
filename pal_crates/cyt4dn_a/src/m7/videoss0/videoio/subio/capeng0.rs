#[doc = r"Register block"]
#[repr(C)]
pub struct CAPENG0 {
    #[doc = "0x00 - Top-Level Configuration"]
    pub capengcfg: CAPENGCFG,
    _reserved1: [u8; 0x03fc],
    #[doc = "0x400..0x448 - Frame Capture Unit"]
    pub framecap: FRAMECAP,
}
#[doc = "Top-Level Configuration"]
pub use self::capengcfg::CAPENGCFG;
#[doc = r"Cluster"]
#[doc = "Top-Level Configuration"]
pub mod capengcfg;
#[doc = "Frame Capture Unit"]
pub use self::framecap::FRAMECAP;
#[doc = r"Cluster"]
#[doc = "Frame Capture Unit"]
pub mod framecap;
