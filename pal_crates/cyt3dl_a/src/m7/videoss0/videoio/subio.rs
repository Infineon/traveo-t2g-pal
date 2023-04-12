#[doc = r"Register block"]
#[repr(C)]
pub struct SUBIO {
    #[doc = "0x00..0x40 - Video I/O Core Configuration"]
    pub videoiocfg: VIDEOIOCFG,
    _reserved1: [u8; 0x03c0],
    #[doc = "0x400..0x418 - Frame Dump Unit"]
    pub framedump: FRAMEDUMP,
    _reserved2: [u8; 0x0be8],
    #[doc = "0x1000..0x1448 - Capture Engine"]
    pub capeng0: CAPENG0,
    _reserved3: [u8; 0xebb8],
    #[doc = "0x10000..0x18c24 - Composition Engine"]
    pub compeng: COMPENG,
}
#[doc = "Video I/O Core Configuration"]
pub use self::videoiocfg::VIDEOIOCFG;
#[doc = r"Cluster"]
#[doc = "Video I/O Core Configuration"]
pub mod videoiocfg;
#[doc = "Frame Dump Unit"]
pub use self::framedump::FRAMEDUMP;
#[doc = r"Cluster"]
#[doc = "Frame Dump Unit"]
pub mod framedump;
#[doc = "Capture Engine"]
pub use self::capeng0::CAPENG0;
#[doc = r"Cluster"]
#[doc = "Capture Engine"]
pub mod capeng0;
#[doc = "Composition Engine"]
pub use self::compeng::COMPENG;
#[doc = r"Cluster"]
#[doc = "Composition Engine"]
pub mod compeng;
