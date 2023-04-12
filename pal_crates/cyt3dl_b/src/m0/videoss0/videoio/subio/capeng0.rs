#[doc = r"Register block"]
#[repr(C)]
pub struct CAPENG0 {
    _reserved0: [u8; 0x0400],
    #[doc = "0x400..0x448 - Frame Capture Unit"]
    pub framecap: FRAMECAP,
}
#[doc = "Frame Capture Unit"]
pub use self::framecap::FRAMECAP;
#[doc = r"Cluster"]
#[doc = "Frame Capture Unit"]
pub mod framecap;
