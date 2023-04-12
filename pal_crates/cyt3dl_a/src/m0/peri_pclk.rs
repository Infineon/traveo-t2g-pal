#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00..0x1ffc - PERI clock domains"]
    pub gr0: GR,
    _reserved1: [u8; 0x04],
    #[doc = "0x2000..0x3ffc - PERI clock domains"]
    pub gr1: GR,
}
#[doc = "PERI clock domains"]
pub use self::gr::GR;
#[doc = r"Cluster"]
#[doc = "PERI clock domains"]
pub mod gr;
