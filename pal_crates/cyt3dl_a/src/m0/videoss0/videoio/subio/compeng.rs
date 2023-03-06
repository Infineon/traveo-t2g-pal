#[doc = r"Register block"]
#[repr(C)]
pub struct COMPENG {
    #[doc = "0x00..0x1344 - Top-Level"]
    pub compengcfg: COMPENGCFG,
    _reserved1: [u8; 0x0cbc],
    #[doc = "0x2000..0x2014 - ConstFrame #0 (Content)"]
    pub constframe0: CONSTFRAME0,
    _reserved2: [u8; 0x03ec],
    #[doc = "0x2400..0x2420 - ExtDst #0 (Content)"]
    pub extdst0: EXTDST0,
    _reserved3: [u8; 0x03e0],
    #[doc = "0x2800..0x2814 - ConstFrame #4 (Safety)"]
    pub constframe4: CONSTFRAME4,
    _reserved4: [u8; 0x03ec],
    #[doc = "0x2c00..0x2c20 - ExtDst #4 (Safety)"]
    pub extdst4: EXTDST4,
    _reserved5: [u8; 0x03e0],
    #[doc = "0x3000..0x3014 - ConstFrame #1 (Content)"]
    pub constframe1: CONSTFRAME1,
    _reserved6: [u8; 0x03ec],
    #[doc = "0x3400..0x3420 - ExtDst #1 (Content)"]
    pub extdst1: EXTDST1,
    _reserved7: [u8; 0x03e0],
    #[doc = "0x3800..0x3814 - ConstFrame #5 (Safety)"]
    pub constframe5: CONSTFRAME5,
    _reserved8: [u8; 0x03ec],
    #[doc = "0x3c00..0x3c20 - ExtDst #5 (Safety)"]
    pub extdst5: EXTDST5,
    _reserved9: [u8; 0x03e0],
    #[doc = "0x4000..0x4024 - ExtSrc #0 (Capture)"]
    pub extsrc4: EXTSRC4,
    _reserved10: [u8; 0x03dc],
    #[doc = "0x4400..0x4450 - Store #0 (Capture)"]
    pub store4: STORE4,
    _reserved11: [u8; 0x03b0],
    #[doc = "0x4800..0x496c - FetchLayer #0 (Display)"]
    pub fetchlayer0: FETCHLAYER0,
    _reserved12: [u8; 0x0294],
    #[doc = "0x4c00..0x4c6c - FetchDecode #4 (Capture)"]
    pub fetchdecode4: FETCHDECODE4,
    _reserved13: [u8; 0x0394],
    #[doc = "0x5000..0x504c - FetchEco #4 (Capture)"]
    pub fetcheco4: FETCHECO4,
    _reserved14: [u8; 0x07b4],
    #[doc = "0x5800..0x5984 - FetchWarp #1 (Display)"]
    pub fetchwarp1: FETCHWARP1,
    _reserved15: [u8; 0x027c],
    #[doc = "0x5c00..0x5c4c - FetchEco #1 (Display)"]
    pub fetcheco1: FETCHECO1,
    _reserved16: [u8; 0x03b4],
    #[doc = "0x6000..0x616c - FetchLayer #1 (Display)"]
    pub fetchlayer1: FETCHLAYER1,
    _reserved17: [u8; 0x0294],
    #[doc = "0x6400..0x646c - FetchDecode #0 (Display)"]
    pub fetchdecode0: FETCHDECODE0,
    _reserved18: [u8; 0x0394],
    #[doc = "0x6800..0x6810 - Video Processing Block #0 GammaCor (Capture)"]
    pub gammacor4: GAMMACOR4,
    _reserved19: [u8; 0x03f0],
    #[doc = "0x6c00..0x6c30 - Video Processing Block #0 Matrix (Capture)"]
    pub matrix4: MATRIX4,
    _reserved20: [u8; 0x03d0],
    #[doc = "0x7000..0x7054 - GPscaler #4 (Capture)"]
    pub gpscaler4: GPSCALER4,
    _reserved21: [u8; 0x03ac],
    #[doc = "0x7400..0x7444 - Video Processing Block #0 Histogram (Capture)"]
    pub histogram4: HISTOGRAM4,
    _reserved22: [u8; 0x03bc],
    #[doc = "0x7800..0x7810 - LayerBlend #1 (Display, Alpha Plane 1)"]
    pub layerblend1: LAYERBLEND1,
    _reserved23: [u8; 0x03f0],
    #[doc = "0x7c00..0x7c10 - LayerBlend #2 (Display, Alpha Plane 2)"]
    pub layerblend2: LAYERBLEND2,
    _reserved24: [u8; 0x03f0],
    #[doc = "0x8000..0x8010 - LayerBlend #3 (Display, Alpha Plane 3)"]
    pub layerblend3: LAYERBLEND3,
    _reserved25: [u8; 0x03f0],
    #[doc = "0x8400..0x8410 - LayerBlend #4 (Display, Alpha Plane 4)"]
    pub layerblend4: LAYERBLEND4,
    _reserved26: [u8; 0x03f0],
    #[doc = "0x8800..0x8810 - LayerBlend #5 (Display, Alpha Plane 5)"]
    pub layerblend5: LAYERBLEND5,
    _reserved27: [u8; 0x03f0],
    #[doc = "0x8c00..0x8c24 - ExtSrc #8 (Display)"]
    pub extsrc8: EXTSRC8,
}
#[doc = "Top-Level"]
pub use self::compengcfg::COMPENGCFG;
#[doc = r"Cluster"]
#[doc = "Top-Level"]
pub mod compengcfg;
#[doc = "ConstFrame #0 (Content)"]
pub use self::constframe0::CONSTFRAME0;
#[doc = r"Cluster"]
#[doc = "ConstFrame #0 (Content)"]
pub mod constframe0;
#[doc = "ExtDst #0 (Content)"]
pub use self::extdst0::EXTDST0;
#[doc = r"Cluster"]
#[doc = "ExtDst #0 (Content)"]
pub mod extdst0;
#[doc = "ConstFrame #4 (Safety)"]
pub use self::constframe4::CONSTFRAME4;
#[doc = r"Cluster"]
#[doc = "ConstFrame #4 (Safety)"]
pub mod constframe4;
#[doc = "ExtDst #4 (Safety)"]
pub use self::extdst4::EXTDST4;
#[doc = r"Cluster"]
#[doc = "ExtDst #4 (Safety)"]
pub mod extdst4;
#[doc = "ConstFrame #1 (Content)"]
pub use self::constframe1::CONSTFRAME1;
#[doc = r"Cluster"]
#[doc = "ConstFrame #1 (Content)"]
pub mod constframe1;
#[doc = "ExtDst #1 (Content)"]
pub use self::extdst1::EXTDST1;
#[doc = r"Cluster"]
#[doc = "ExtDst #1 (Content)"]
pub mod extdst1;
#[doc = "ConstFrame #5 (Safety)"]
pub use self::constframe5::CONSTFRAME5;
#[doc = r"Cluster"]
#[doc = "ConstFrame #5 (Safety)"]
pub mod constframe5;
#[doc = "ExtDst #5 (Safety)"]
pub use self::extdst5::EXTDST5;
#[doc = r"Cluster"]
#[doc = "ExtDst #5 (Safety)"]
pub mod extdst5;
#[doc = "ExtSrc #0 (Capture)"]
pub use self::extsrc4::EXTSRC4;
#[doc = r"Cluster"]
#[doc = "ExtSrc #0 (Capture)"]
pub mod extsrc4;
#[doc = "Store #0 (Capture)"]
pub use self::store4::STORE4;
#[doc = r"Cluster"]
#[doc = "Store #0 (Capture)"]
pub mod store4;
#[doc = "FetchLayer #0 (Display)"]
pub use self::fetchlayer0::FETCHLAYER0;
#[doc = r"Cluster"]
#[doc = "FetchLayer #0 (Display)"]
pub mod fetchlayer0;
#[doc = "FetchDecode #4 (Capture)"]
pub use self::fetchdecode4::FETCHDECODE4;
#[doc = r"Cluster"]
#[doc = "FetchDecode #4 (Capture)"]
pub mod fetchdecode4;
#[doc = "FetchEco #4 (Capture)"]
pub use self::fetcheco4::FETCHECO4;
#[doc = r"Cluster"]
#[doc = "FetchEco #4 (Capture)"]
pub mod fetcheco4;
#[doc = "FetchWarp #1 (Display)"]
pub use self::fetchwarp1::FETCHWARP1;
#[doc = r"Cluster"]
#[doc = "FetchWarp #1 (Display)"]
pub mod fetchwarp1;
#[doc = "FetchEco #1 (Display)"]
pub use self::fetcheco1::FETCHECO1;
#[doc = r"Cluster"]
#[doc = "FetchEco #1 (Display)"]
pub mod fetcheco1;
#[doc = "FetchLayer #1 (Display)"]
pub use self::fetchlayer1::FETCHLAYER1;
#[doc = r"Cluster"]
#[doc = "FetchLayer #1 (Display)"]
pub mod fetchlayer1;
#[doc = "FetchDecode #0 (Display)"]
pub use self::fetchdecode0::FETCHDECODE0;
#[doc = r"Cluster"]
#[doc = "FetchDecode #0 (Display)"]
pub mod fetchdecode0;
#[doc = "Video Processing Block #0 GammaCor (Capture)"]
pub use self::gammacor4::GAMMACOR4;
#[doc = r"Cluster"]
#[doc = "Video Processing Block #0 GammaCor (Capture)"]
pub mod gammacor4;
#[doc = "Video Processing Block #0 Matrix (Capture)"]
pub use self::matrix4::MATRIX4;
#[doc = r"Cluster"]
#[doc = "Video Processing Block #0 Matrix (Capture)"]
pub mod matrix4;
#[doc = "GPscaler #4 (Capture)"]
pub use self::gpscaler4::GPSCALER4;
#[doc = r"Cluster"]
#[doc = "GPscaler #4 (Capture)"]
pub mod gpscaler4;
#[doc = "Video Processing Block #0 Histogram (Capture)"]
pub use self::histogram4::HISTOGRAM4;
#[doc = r"Cluster"]
#[doc = "Video Processing Block #0 Histogram (Capture)"]
pub mod histogram4;
#[doc = "LayerBlend #1 (Display, Alpha Plane 1)"]
pub use self::layerblend1::LAYERBLEND1;
#[doc = r"Cluster"]
#[doc = "LayerBlend #1 (Display, Alpha Plane 1)"]
pub mod layerblend1;
#[doc = "LayerBlend #2 (Display, Alpha Plane 2)"]
pub use self::layerblend2::LAYERBLEND2;
#[doc = r"Cluster"]
#[doc = "LayerBlend #2 (Display, Alpha Plane 2)"]
pub mod layerblend2;
#[doc = "LayerBlend #3 (Display, Alpha Plane 3)"]
pub use self::layerblend3::LAYERBLEND3;
#[doc = r"Cluster"]
#[doc = "LayerBlend #3 (Display, Alpha Plane 3)"]
pub mod layerblend3;
#[doc = "LayerBlend #4 (Display, Alpha Plane 4)"]
pub use self::layerblend4::LAYERBLEND4;
#[doc = r"Cluster"]
#[doc = "LayerBlend #4 (Display, Alpha Plane 4)"]
pub mod layerblend4;
#[doc = "LayerBlend #5 (Display, Alpha Plane 5)"]
pub use self::layerblend5::LAYERBLEND5;
#[doc = r"Cluster"]
#[doc = "LayerBlend #5 (Display, Alpha Plane 5)"]
pub mod layerblend5;
#[doc = "ExtSrc #8 (Display)"]
pub use self::extsrc8::EXTSRC8;
#[doc = r"Cluster"]
#[doc = "ExtSrc #8 (Display)"]
pub mod extsrc8;
