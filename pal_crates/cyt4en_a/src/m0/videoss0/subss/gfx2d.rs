#[doc = r"Register block"]
#[repr(C)]
pub struct GFX2D {
    #[doc = "0x00..0x80 - Graphics 2D Core config"]
    pub gfx2dcfg: GFX2DCFG,
    _reserved1: [u8; 0xff80],
    #[doc = "0x10000..0x100f4 - Command Sequencer"]
    pub cmdseq: CMDSEQ,
    _reserved2: [u8; 0xff0c],
    #[doc = "0x20000..0x26c50 - Blit Engine"]
    pub bliteng: BLITENG,
    _reserved3: [u8; 0x93b0],
    #[doc = "0x30000..0x30064 - Drawing Engine"]
    pub draweng: DRAWENG,
}
#[doc = "Graphics 2D Core config"]
pub use self::gfx2dcfg::GFX2DCFG;
#[doc = r"Cluster"]
#[doc = "Graphics 2D Core config"]
pub mod gfx2dcfg;
#[doc = "Command Sequencer"]
pub use self::cmdseq::CMDSEQ;
#[doc = r"Cluster"]
#[doc = "Command Sequencer"]
pub mod cmdseq;
#[doc = "Blit Engine"]
pub use self::bliteng::BLITENG;
#[doc = r"Cluster"]
#[doc = "Blit Engine"]
pub mod bliteng;
#[doc = "Drawing Engine"]
pub use self::draweng::DRAWENG;
#[doc = r"Cluster"]
#[doc = "Drawing Engine"]
pub mod draweng;
