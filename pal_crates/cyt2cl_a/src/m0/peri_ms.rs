#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00..0x400 - Programmable protection structure pair"]
    pub ppu_pr: [PPU_PR; 16],
    _reserved1: [u8; 0x0400],
    #[doc = "0x800..0x7880 - Fixed protection structure pair"]
    pub ppu_fx: [PPU_FX; 450],
}
#[doc = "Programmable protection structure pair"]
pub use self::ppu_pr::PPU_PR;
#[doc = r"Cluster"]
#[doc = "Programmable protection structure pair"]
pub mod ppu_pr;
#[doc = "Fixed protection structure pair"]
pub use self::ppu_fx::PPU_FX;
#[doc = r"Cluster"]
#[doc = "Fixed protection structure pair"]
pub mod ppu_fx;
