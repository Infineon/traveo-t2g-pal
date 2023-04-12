#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - IP control register."]
    pub ctl: CTL,
    _reserved1: [u8; 0x0ffc],
    #[doc = "0x1000..0x10c0 - Data fetch, store and interrupt control registers."]
    pub hf: HF,
    _reserved2: [u8; 0x0740],
    #[doc = "0x1800..0x1ce8 - JPEG decoder core registers."]
    pub dec: DEC,
}
#[doc = "CTL (rw) register accessor: an alias for `Reg<CTL_SPEC>`"]
pub type CTL = crate::Reg<ctl::CTL_SPEC>;
#[doc = "IP control register."]
pub mod ctl;
#[doc = "Data fetch, store and interrupt control registers."]
pub use self::hf::HF;
#[doc = r"Cluster"]
#[doc = "Data fetch, store and interrupt control registers."]
pub mod hf;
#[doc = "JPEG decoder core registers."]
pub use self::dec::DEC;
#[doc = r"Cluster"]
#[doc = "JPEG decoder core registers."]
pub mod dec;
