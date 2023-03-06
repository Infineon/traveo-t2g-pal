#[doc = r"Register block"]
#[repr(C)]
pub struct SUBSS {
    #[doc = "0x00..0x44 - Subsystem Control."]
    pub videosscfg: VIDEOSSCFG,
    _reserved1: [u8; 0x07bc],
    #[doc = "0x800..0x8b0 - Graphics Cache for Blit and Composition Engine."]
    pub gfxcachepix: GFXCACHEPIX,
    _reserved2: [u8; 0x0350],
    #[doc = "0xc00..0xc8c - Graphics Cache for Command Sequencer."]
    pub gfxcachecmd: GFXCACHECMD,
    _reserved3: [u8; 0x0374],
    #[doc = "0x1000..0x1028 - Load Balancing Crossbar for Blit Engine."]
    pub lbcbarblit: LBCBARBLIT,
    _reserved4: [u8; 0x03d8],
    #[doc = "0x1400..0x1424 - Crossbar for JPEG Decoder, Drawing Engine and Command Sequencer."]
    pub lbcbarjpgcmd: LBCBARJPGCMD,
    _reserved5: [u8; 0x03dc],
    #[doc = "0x1800..0x1824 - Crossbar for Display Layers (FetchWarp1, FetchLayer1, FetchEco1)."]
    pub lbcbarvideo0: LBCBARVIDEO0,
    _reserved6: [u8; 0x03dc],
    #[doc = "0x1c00..0x1c24 - Crossbar for Display Layers (FetchDecode0, FetchDecode4, FetchEco4)."]
    pub lbcbarvideo1: LBCBARVIDEO1,
    _reserved7: [u8; 0x0003_e3dc],
    #[doc = "0x40000..0x70064 - Graphics 2D Core"]
    pub gfx2d: GFX2D,
}
#[doc = "Subsystem Control."]
pub use self::videosscfg::VIDEOSSCFG;
#[doc = r"Cluster"]
#[doc = "Subsystem Control."]
pub mod videosscfg;
#[doc = "Graphics Cache for Blit and Composition Engine."]
pub use self::gfxcachepix::GFXCACHEPIX;
#[doc = r"Cluster"]
#[doc = "Graphics Cache for Blit and Composition Engine."]
pub mod gfxcachepix;
#[doc = "Graphics Cache for Command Sequencer."]
pub use self::gfxcachecmd::GFXCACHECMD;
#[doc = r"Cluster"]
#[doc = "Graphics Cache for Command Sequencer."]
pub mod gfxcachecmd;
#[doc = "Load Balancing Crossbar for Blit Engine."]
pub use self::lbcbarblit::LBCBARBLIT;
#[doc = r"Cluster"]
#[doc = "Load Balancing Crossbar for Blit Engine."]
pub mod lbcbarblit;
#[doc = "Crossbar for JPEG Decoder, Drawing Engine and Command Sequencer."]
pub use self::lbcbarjpgcmd::LBCBARJPGCMD;
#[doc = r"Cluster"]
#[doc = "Crossbar for JPEG Decoder, Drawing Engine and Command Sequencer."]
pub mod lbcbarjpgcmd;
#[doc = "Crossbar for Display Layers (FetchWarp1, FetchLayer1, FetchEco1)."]
pub use self::lbcbarvideo0::LBCBARVIDEO0;
#[doc = r"Cluster"]
#[doc = "Crossbar for Display Layers (FetchWarp1, FetchLayer1, FetchEco1)."]
pub mod lbcbarvideo0;
#[doc = "Crossbar for Display Layers (FetchDecode0, FetchDecode4, FetchEco4)."]
pub use self::lbcbarvideo1::LBCBARVIDEO1;
#[doc = r"Cluster"]
#[doc = "Crossbar for Display Layers (FetchDecode0, FetchDecode4, FetchEco4)."]
pub mod lbcbarvideo1;
#[doc = "Graphics 2D Core"]
pub use self::gfx2d::GFX2D;
#[doc = r"Cluster"]
#[doc = "Graphics 2D Core"]
pub mod gfx2d;
