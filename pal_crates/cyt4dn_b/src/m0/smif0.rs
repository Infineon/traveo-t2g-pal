#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00..0x1720 - AXI/AHB interleaving and FOTA bridge"]
    pub smif_bridge: SMIF_BRIDGE,
    _reserved1: [u8; 0x0001_e8e0],
    #[doc = "0x20000..0x208f8 - Serial Memory Interface"]
    pub core0: CORE,
    _reserved2: [u8; 0xf708],
    #[doc = "0x30000..0x308f8 - Serial Memory Interface"]
    pub core1: CORE,
}
#[doc = "AXI/AHB interleaving and FOTA bridge"]
pub use self::smif_bridge::SMIF_BRIDGE;
#[doc = r"Cluster"]
#[doc = "AXI/AHB interleaving and FOTA bridge"]
pub mod smif_bridge;
#[doc = "Serial Memory Interface"]
pub use self::core::CORE;
#[doc = r"Cluster"]
#[doc = "Serial Memory Interface"]
pub mod core;
