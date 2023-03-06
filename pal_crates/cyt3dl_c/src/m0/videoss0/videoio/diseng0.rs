#[doc = r"Register block"]
#[repr(C)]
pub struct DISENG0 {
    #[doc = "0x00..0x24 - Top-Level Configuration"]
    pub disengcfg0: DISENGCFG0,
    _reserved1: [u8; 0x0fdc],
    #[doc = "0x1000..0x1138 - Signature Unit"]
    pub sig0: SIG0,
    _reserved2: [u8; 0x0ec8],
    #[doc = "0x2000..0x2098 - Frame Generator"]
    pub framegen0: FRAMEGEN0,
    _reserved3: [u8; 0x0368],
    #[doc = "0x2400..0x2410 - Gamma Correction Unit"]
    pub gammacor0: GAMMACOR0,
    _reserved4: [u8; 0x03f0],
    #[doc = "0x2800..0x280c - Dither Unit"]
    pub dither0: DITHER0,
    _reserved5: [u8; 0x07f4],
    #[doc = "0x3000..0x3568 - Timing Controller"]
    pub tcon0: TCON0,
}
#[doc = "Top-Level Configuration"]
pub use self::disengcfg0::DISENGCFG0;
#[doc = r"Cluster"]
#[doc = "Top-Level Configuration"]
pub mod disengcfg0;
#[doc = "Signature Unit"]
pub use self::sig0::SIG0;
#[doc = r"Cluster"]
#[doc = "Signature Unit"]
pub mod sig0;
#[doc = "Frame Generator"]
pub use self::framegen0::FRAMEGEN0;
#[doc = r"Cluster"]
#[doc = "Frame Generator"]
pub mod framegen0;
#[doc = "Gamma Correction Unit"]
pub use self::gammacor0::GAMMACOR0;
#[doc = r"Cluster"]
#[doc = "Gamma Correction Unit"]
pub mod gammacor0;
#[doc = "Dither Unit"]
pub use self::dither0::DITHER0;
#[doc = r"Cluster"]
#[doc = "Dither Unit"]
pub mod dither0;
#[doc = "Timing Controller"]
pub use self::tcon0::TCON0;
#[doc = r"Cluster"]
#[doc = "Timing Controller"]
pub mod tcon0;
