#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x8000],
    #[doc = "0x8000..0x81d0 - I2S structure"]
    pub i2s_struct0: I2S_STRUCT,
    _reserved1: [u8; 0x30],
    #[doc = "0x8200..0x83d0 - I2S structure"]
    pub i2s_struct1: I2S_STRUCT,
    _reserved2: [u8; 0x30],
    #[doc = "0x8400..0x85d0 - I2S structure"]
    pub i2s_struct2: I2S_STRUCT,
    _reserved3: [u8; 0x30],
    #[doc = "0x8600..0x87d0 - I2S structure"]
    pub i2s_struct3: I2S_STRUCT,
}
#[doc = "I2S structure"]
pub use self::i2s_struct::I2S_STRUCT;
#[doc = r"Cluster"]
#[doc = "I2S structure"]
pub mod i2s_struct;
