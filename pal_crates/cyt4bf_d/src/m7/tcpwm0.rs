#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00..0x2a00 - Group of counters"]
    pub grp0: GRP,
    _reserved1: [u8; 0x5600],
    #[doc = "0x8000..0xaa00 - Group of counters"]
    pub grp1: GRP,
    _reserved2: [u8; 0x5600],
    #[doc = "0x10000..0x12a00 - Group of counters"]
    pub grp2: GRP,
}
#[doc = "Group of counters"]
pub use self::grp::GRP;
#[doc = r"Cluster"]
#[doc = "Group of counters"]
pub mod grp;
