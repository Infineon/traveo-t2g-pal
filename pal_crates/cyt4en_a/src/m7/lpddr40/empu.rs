#[doc = r"Register block"]
#[repr(C)]
pub struct EMPU {
    _reserved0: [u8; 0x2000],
    #[doc = "0x2000..0x2028 - EMPU structures."]
    pub empu_struct0: EMPU_STRUCT,
    _reserved1: [u8; 0x18],
    #[doc = "0x2040..0x2068 - EMPU structures."]
    pub empu_struct1: EMPU_STRUCT,
    _reserved2: [u8; 0x18],
    #[doc = "0x2080..0x20a8 - EMPU structures."]
    pub empu_struct2: EMPU_STRUCT,
    _reserved3: [u8; 0x18],
    #[doc = "0x20c0..0x20e8 - EMPU structures."]
    pub empu_struct3: EMPU_STRUCT,
    _reserved4: [u8; 0x18],
    #[doc = "0x2100..0x2128 - EMPU structures."]
    pub empu_struct4: EMPU_STRUCT,
    _reserved5: [u8; 0x18],
    #[doc = "0x2140..0x2168 - EMPU structures."]
    pub empu_struct5: EMPU_STRUCT,
    _reserved6: [u8; 0x18],
    #[doc = "0x2180..0x21a8 - EMPU structures."]
    pub empu_struct6: EMPU_STRUCT,
    _reserved7: [u8; 0x18],
    #[doc = "0x21c0..0x21e8 - EMPU structures."]
    pub empu_struct7: EMPU_STRUCT,
}
#[doc = "EMPU structures."]
pub use self::empu_struct::EMPU_STRUCT;
#[doc = r"Cluster"]
#[doc = "EMPU structures."]
pub mod empu_struct;
