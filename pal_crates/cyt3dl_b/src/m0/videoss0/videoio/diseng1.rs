#[doc = r"Register block"]
#[repr(C)]
pub struct DISENG1 {
    #[doc = "0x00..0x24 - Top-Level Configuration"]
    pub disengcfg1: DISENGCFG1,
    _reserved1: [u8; 0x0fdc],
    #[doc = "0x1000..0x1138 - Signature Unit"]
    pub sig1: SIG1,
    _reserved2: [u8; 0x0ec8],
    #[doc = "0x2000..0x2098 - Frame Generator"]
    pub framegen1: FRAMEGEN1,
    _reserved3: [u8; 0x0368],
    #[doc = "0x2400..0x2410 - Gamma Correction Unit"]
    pub gammacor1: GAMMACOR1,
    _reserved4: [u8; 0x03f0],
    #[doc = "0x2800..0x280c - Dither Unit"]
    pub dither1: DITHER1,
    _reserved5: [u8; 0x07f4],
    #[doc = "0x3000..0x3568 - Timing Controller"]
    pub tcon1: TCON1,
}
#[doc = "Top-Level Configuration"]
pub use self::disengcfg1::DISENGCFG1;
#[doc = r"Cluster"]
#[doc = "Top-Level Configuration"]
pub mod disengcfg1;
#[doc = "Signature Unit"]
pub use self::sig1::SIG1;
#[doc = r"Cluster"]
#[doc = "Signature Unit"]
pub mod sig1;
#[doc = "Frame Generator"]
pub use self::framegen1::FRAMEGEN1;
#[doc = r"Cluster"]
#[doc = "Frame Generator"]
pub mod framegen1;
#[doc = "Gamma Correction Unit"]
pub use self::gammacor1::GAMMACOR1;
#[doc = r"Cluster"]
#[doc = "Gamma Correction Unit"]
pub mod gammacor1;
#[doc = "Dither Unit"]
pub use self::dither1::DITHER1;
#[doc = r"Cluster"]
#[doc = "Dither Unit"]
pub mod dither1;
#[doc = "Timing Controller"]
pub use self::tcon1::TCON1;
#[doc = r"Cluster"]
#[doc = "Timing Controller"]
pub mod tcon1;
