#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00..0xf504 - Flash Macro Registers"]
    pub flashcx: FLASHCX,
}
#[doc = "Flash Macro Registers"]
pub use self::flashcx::FLASHCX;
#[doc = r"Cluster"]
#[doc = "Flash Macro Registers"]
pub mod flashcx;
