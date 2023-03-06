#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00..0x70064 - Sub Interface"]
    pub subss: SUBSS,
    _reserved1: [u8; 0xff9c],
    #[doc = "0x80000..0xa7568 - Video I/O Core"]
    pub videoio: VIDEOIO,
    _reserved2: [u8; 0x0001_8a98],
    #[doc = "0xc0000..0xc0014 - FPD-Link Interface"]
    pub fpdlink0: FPDLINK0,
    _reserved3: [u8; 0x7fec],
    #[doc = "0xc8000..0xc8014 - FPD-Link Interface"]
    pub fpdlink1: FPDLINK1,
    _reserved4: [u8; 0x7fec],
    #[doc = "0xd0000..0xd0260 - MIPICSI"]
    pub mipicsi: MIPICSI,
    _reserved5: [u8; 0x0001_fda0],
    #[doc = "0xf0000..0xfbc04 - Bus infrastructure"]
    pub infra: INFRA,
}
#[doc = "Sub Interface"]
pub use self::subss::SUBSS;
#[doc = r"Cluster"]
#[doc = "Sub Interface"]
pub mod subss;
#[doc = "Video I/O Core"]
pub use self::videoio::VIDEOIO;
#[doc = r"Cluster"]
#[doc = "Video I/O Core"]
pub mod videoio;
#[doc = "FPD-Link Interface"]
pub use self::fpdlink0::FPDLINK0;
#[doc = r"Cluster"]
#[doc = "FPD-Link Interface"]
pub mod fpdlink0;
#[doc = "FPD-Link Interface"]
pub use self::fpdlink1::FPDLINK1;
#[doc = r"Cluster"]
#[doc = "FPD-Link Interface"]
pub mod fpdlink1;
#[doc = "MIPICSI"]
pub use self::mipicsi::MIPICSI;
#[doc = r"Cluster"]
#[doc = "MIPICSI"]
pub mod mipicsi;
#[doc = "Bus infrastructure"]
pub use self::infra::INFRA;
#[doc = r"Cluster"]
#[doc = "Bus infrastructure"]
pub mod infra;
