#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00..0x100 - IPC structure"]
    pub struct_: [STRUCT; 8],
    _reserved1: [u8; 0x0f00],
    #[doc = "0x1000..0x1010 - IPC interrupt structure"]
    pub intr_struct0: INTR_STRUCT,
    _reserved2: [u8; 0x10],
    #[doc = "0x1020..0x1030 - IPC interrupt structure"]
    pub intr_struct1: INTR_STRUCT,
    _reserved3: [u8; 0x10],
    #[doc = "0x1040..0x1050 - IPC interrupt structure"]
    pub intr_struct2: INTR_STRUCT,
    _reserved4: [u8; 0x10],
    #[doc = "0x1060..0x1070 - IPC interrupt structure"]
    pub intr_struct3: INTR_STRUCT,
    _reserved5: [u8; 0x10],
    #[doc = "0x1080..0x1090 - IPC interrupt structure"]
    pub intr_struct4: INTR_STRUCT,
    _reserved6: [u8; 0x10],
    #[doc = "0x10a0..0x10b0 - IPC interrupt structure"]
    pub intr_struct5: INTR_STRUCT,
    _reserved7: [u8; 0x10],
    #[doc = "0x10c0..0x10d0 - IPC interrupt structure"]
    pub intr_struct6: INTR_STRUCT,
    _reserved8: [u8; 0x10],
    #[doc = "0x10e0..0x10f0 - IPC interrupt structure"]
    pub intr_struct7: INTR_STRUCT,
}
#[doc = "IPC structure"]
pub use self::struct_::STRUCT;
#[doc = r"Cluster"]
#[doc = "IPC structure"]
pub mod struct_;
#[doc = "IPC interrupt structure"]
pub use self::intr_struct::INTR_STRUCT;
#[doc = r"Cluster"]
#[doc = "IPC interrupt structure"]
pub mod intr_struct;
