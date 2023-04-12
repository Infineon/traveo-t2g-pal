#[doc = r"Register block"]
#[repr(C)]
pub struct BLITENG {
    #[doc = "0x00..0x4000 - Top-Level"]
    pub blitengcfg: BLITENGCFG,
    #[doc = "0x4000..0x4094 - Fetch SRC"]
    pub fetchsrc: FETCHSRC,
    _reserved2: [u8; 0x036c],
    #[doc = "0x4400..0x4494 - Fetch DST (only for IBO)"]
    pub fetchdst: FETCHDST,
    _reserved3: [u8; 0x036c],
    #[doc = "0x4800..0x4894 - Fetch MASK (only for IBO)"]
    pub fetchmask: FETCHMASK,
    _reserved4: [u8; 0x076c],
    #[doc = "0x5000..0x5018 - ROp"]
    pub rop: ROP,
    _reserved5: [u8; 0x07e8],
    #[doc = "0x5800..0x6000 - CLut (only for IBO)"]
    pub clut: CLUT,
    #[doc = "0x6000..0x6030 - Matrix"]
    pub matrix: MATRIX,
    _reserved7: [u8; 0x03d0],
    #[doc = "0x6400..0x6454 - GPScaler (only for IBO)"]
    pub gpscaler: GPSCALER,
    _reserved8: [u8; 0x03ac],
    #[doc = "0x6800..0x6834 - BlitBlend"]
    pub blitblend: BLITBLEND,
    _reserved9: [u8; 0x03cc],
    #[doc = "0x6c00..0x6c50 - Store"]
    pub store: STORE,
}
#[doc = "Top-Level"]
pub use self::blitengcfg::BLITENGCFG;
#[doc = r"Cluster"]
#[doc = "Top-Level"]
pub mod blitengcfg;
#[doc = "Fetch SRC"]
pub use self::fetchsrc::FETCHSRC;
#[doc = r"Cluster"]
#[doc = "Fetch SRC"]
pub mod fetchsrc;
#[doc = "Fetch DST (only for IBO)"]
pub use self::fetchdst::FETCHDST;
#[doc = r"Cluster"]
#[doc = "Fetch DST (only for IBO)"]
pub mod fetchdst;
#[doc = "Fetch MASK (only for IBO)"]
pub use self::fetchmask::FETCHMASK;
#[doc = r"Cluster"]
#[doc = "Fetch MASK (only for IBO)"]
pub mod fetchmask;
#[doc = "ROp"]
pub use self::rop::ROP;
#[doc = r"Cluster"]
#[doc = "ROp"]
pub mod rop;
#[doc = "CLut (only for IBO)"]
pub use self::clut::CLUT;
#[doc = r"Cluster"]
#[doc = "CLut (only for IBO)"]
pub mod clut;
#[doc = "Matrix"]
pub use self::matrix::MATRIX;
#[doc = r"Cluster"]
#[doc = "Matrix"]
pub mod matrix;
#[doc = "GPScaler (only for IBO)"]
pub use self::gpscaler::GPSCALER;
#[doc = r"Cluster"]
#[doc = "GPScaler (only for IBO)"]
pub mod gpscaler;
#[doc = "BlitBlend"]
pub use self::blitblend::BLITBLEND;
#[doc = r"Cluster"]
#[doc = "BlitBlend"]
pub mod blitblend;
#[doc = "Store"]
pub use self::store::STORE;
#[doc = r"Cluster"]
#[doc = "Store"]
pub mod store;
