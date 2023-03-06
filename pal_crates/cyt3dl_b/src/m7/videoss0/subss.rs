#[doc = r"Register block"]
#[repr(C)]
pub struct SUBSS {
    #[doc = "0x00..0x28 - Subsystem Control."]
    pub videosscfg: VIDEOSSCFG,
    _reserved1: [u8; 0x03d8],
    #[doc = "0x400 - Video RAM."]
    pub vram: VRAM,
    _reserved2: [u8; 0x0003_fbfc],
    #[doc = "0x40000..0x70064 - Graphics 2D Core"]
    pub gfx2d: GFX2D,
}
#[doc = "Subsystem Control."]
pub use self::videosscfg::VIDEOSSCFG;
#[doc = r"Cluster"]
#[doc = "Subsystem Control."]
pub mod videosscfg;
#[doc = "Video RAM."]
pub use self::vram::VRAM;
#[doc = r"Cluster"]
#[doc = "Video RAM."]
pub mod vram;
#[doc = "Graphics 2D Core"]
pub use self::gfx2d::GFX2D;
#[doc = r"Cluster"]
#[doc = "Graphics 2D Core"]
pub mod gfx2d;
