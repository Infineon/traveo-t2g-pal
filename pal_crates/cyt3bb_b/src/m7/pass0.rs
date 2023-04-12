#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00..0x3000 - SAR ADC with Sequencer for S40E"]
    pub sar: [SAR; 3],
    _reserved1: [u8; 0x000e_d000],
    #[doc = "0xf0000..0xf0050 - PASS top-level MMIO (Generic Triggers)"]
    pub epass_mmio: EPASS_MMIO,
}
#[doc = "SAR ADC with Sequencer for S40E"]
pub use self::sar::SAR;
#[doc = r"Cluster"]
#[doc = "SAR ADC with Sequencer for S40E"]
pub mod sar;
#[doc = "PASS top-level MMIO (Generic Triggers)"]
pub use self::epass_mmio::EPASS_MMIO;
#[doc = r"Cluster"]
#[doc = "PASS top-level MMIO (Generic Triggers)"]
pub mod epass_mmio;
