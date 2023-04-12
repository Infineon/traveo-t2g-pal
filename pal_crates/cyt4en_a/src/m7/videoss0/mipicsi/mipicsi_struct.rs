#[doc = r"Register block"]
#[repr(C)]
pub struct MIPICSI_STRUCT {
    #[doc = "0x00..0xc8 - MIPI CSI-2 D-PHY wrapper configuration and status"]
    pub mipicsi_wrap: MIPICSI_WRAP,
    _reserved1: [u8; 0x0138],
    #[doc = "0x200..0x260 - MIPICSI2 RX Core through APB interface (3PIP)"]
    pub mipicsi_core: MIPICSI_CORE,
}
#[doc = "MIPI CSI-2 D-PHY wrapper configuration and status"]
pub use self::mipicsi_wrap::MIPICSI_WRAP;
#[doc = r"Cluster"]
#[doc = "MIPI CSI-2 D-PHY wrapper configuration and status"]
pub mod mipicsi_wrap;
#[doc = "MIPICSI2 RX Core through APB interface (3PIP)"]
pub use self::mipicsi_core::MIPICSI_CORE;
#[doc = r"Cluster"]
#[doc = "MIPICSI2 RX Core through APB interface (3PIP)"]
pub mod mipicsi_core;
