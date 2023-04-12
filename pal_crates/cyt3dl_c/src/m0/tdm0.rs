#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x8000],
    #[doc = "0x8000..0x81d0 - TDM structure"]
    pub tdm_struct0: TDM_STRUCT,
    _reserved1: [u8; 0x30],
    #[doc = "0x8200..0x83d0 - TDM structure"]
    pub tdm_struct1: TDM_STRUCT,
    _reserved2: [u8; 0x30],
    #[doc = "0x8400..0x85d0 - TDM structure"]
    pub tdm_struct2: TDM_STRUCT,
    _reserved3: [u8; 0x30],
    #[doc = "0x8600..0x87d0 - TDM structure"]
    pub tdm_struct3: TDM_STRUCT,
}
#[doc = "TDM structure"]
pub use self::tdm_struct::TDM_STRUCT;
#[doc = r"Cluster"]
#[doc = "TDM structure"]
pub mod tdm_struct;
