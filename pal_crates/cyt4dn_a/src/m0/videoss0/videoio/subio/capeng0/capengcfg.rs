#[doc = r"Register block"]
#[repr(C)]
pub struct CAPENGCFG {
    #[doc = "0x00 - DUMMY Register for ."]
    pub dummy: DUMMY,
}
#[doc = "DUMMY (rw) register accessor: an alias for `Reg<DUMMY_SPEC>`"]
pub type DUMMY = crate::Reg<dummy::DUMMY_SPEC>;
#[doc = "DUMMY Register for ."]
pub mod dummy;
