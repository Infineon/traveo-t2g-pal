#[doc = r"Register block"]
#[repr(C)]
pub struct VIDEOIO {
    #[doc = "0x00..0x18c24 - Sub Interface"]
    pub subio: SUBIO,
    _reserved1: [u8; 0x73dc],
    #[doc = "0x20000..0x23568 - Display Engine"]
    pub diseng0: DISENG0,
    _reserved2: [u8; 0x0a98],
    #[doc = "0x24000..0x27568 - Display Engine"]
    pub diseng1: DISENG1,
}
#[doc = "Sub Interface"]
pub use self::subio::SUBIO;
#[doc = r"Cluster"]
#[doc = "Sub Interface"]
pub mod subio;
#[doc = "Display Engine"]
pub use self::diseng0::DISENG0;
#[doc = r"Cluster"]
#[doc = "Display Engine"]
pub mod diseng0;
#[doc = "Display Engine"]
pub use self::diseng1::DISENG1;
#[doc = r"Cluster"]
#[doc = "Display Engine"]
pub mod diseng1;
