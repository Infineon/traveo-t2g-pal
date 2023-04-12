#[doc = r"Register block"]
#[repr(C)]
pub struct MIPICSI {
    #[doc = "0x00..0x260 - MIPICSI Structure"]
    pub mipicsi_struct: MIPICSI_STRUCT,
}
#[doc = "MIPICSI Structure"]
pub use self::mipicsi_struct::MIPICSI_STRUCT;
#[doc = r"Cluster"]
#[doc = "MIPICSI Structure"]
pub mod mipicsi_struct;
