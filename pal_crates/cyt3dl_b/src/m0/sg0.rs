#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x8000],
    #[doc = "0x8000..0x80d0 - Sound generator structure"]
    pub sg_struct0: SG_STRUCT,
    _reserved1: [u8; 0x30],
    #[doc = "0x8100..0x81d0 - Sound generator structure"]
    pub sg_struct1: SG_STRUCT,
    _reserved2: [u8; 0x30],
    #[doc = "0x8200..0x82d0 - Sound generator structure"]
    pub sg_struct2: SG_STRUCT,
    _reserved3: [u8; 0x30],
    #[doc = "0x8300..0x83d0 - Sound generator structure"]
    pub sg_struct3: SG_STRUCT,
    _reserved4: [u8; 0x30],
    #[doc = "0x8400..0x84d0 - Sound generator structure"]
    pub sg_struct4: SG_STRUCT,
}
#[doc = "Sound generator structure"]
pub use self::sg_struct::SG_STRUCT;
#[doc = r"Cluster"]
#[doc = "Sound generator structure"]
pub mod sg_struct;
