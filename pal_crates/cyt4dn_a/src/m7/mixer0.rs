#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x8000],
    #[doc = "0x8000..0x80d0 - Mixer source structure"]
    pub mixer_src_struct0: MIXER_SRC_STRUCT,
    _reserved1: [u8; 0x30],
    #[doc = "0x8100..0x81d0 - Mixer source structure"]
    pub mixer_src_struct1: MIXER_SRC_STRUCT,
    _reserved2: [u8; 0x30],
    #[doc = "0x8200..0x82d0 - Mixer source structure"]
    pub mixer_src_struct2: MIXER_SRC_STRUCT,
    _reserved3: [u8; 0x30],
    #[doc = "0x8300..0x83d0 - Mixer source structure"]
    pub mixer_src_struct3: MIXER_SRC_STRUCT,
    _reserved4: [u8; 0x30],
    #[doc = "0x8400..0x84d0 - Mixer source structure"]
    pub mixer_src_struct4: MIXER_SRC_STRUCT,
    _reserved5: [u8; 0x3b30],
    #[doc = "0xc000..0xc0d0 - Mixer destination structure"]
    pub mixer_dst_struct: MIXER_DST_STRUCT,
    _reserved6: [u8; 0x0f30],
    #[doc = "0xd000..0xd0d0 - Mixer TX structure"]
    pub mixer_tx_struct: MIXER_TX_STRUCT,
}
#[doc = "Mixer source structure"]
pub use self::mixer_src_struct::MIXER_SRC_STRUCT;
#[doc = r"Cluster"]
#[doc = "Mixer source structure"]
pub mod mixer_src_struct;
#[doc = "Mixer destination structure"]
pub use self::mixer_dst_struct::MIXER_DST_STRUCT;
#[doc = r"Cluster"]
#[doc = "Mixer destination structure"]
pub mod mixer_dst_struct;
#[doc = "Mixer TX structure"]
pub use self::mixer_tx_struct::MIXER_TX_STRUCT;
#[doc = r"Cluster"]
#[doc = "Mixer TX structure"]
pub mod mixer_tx_struct;
