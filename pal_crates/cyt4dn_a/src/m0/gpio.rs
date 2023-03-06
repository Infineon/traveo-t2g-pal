#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00..0x64 - GPIO port registers"]
    pub prt0: PRT,
    _reserved1: [u8; 0x1c],
    #[doc = "0x80..0xe4 - GPIO port registers"]
    pub prt1: PRT,
    _reserved2: [u8; 0x1c],
    #[doc = "0x100..0x164 - GPIO port registers"]
    pub prt2: PRT,
    _reserved3: [u8; 0x1c],
    #[doc = "0x180..0x1e4 - GPIO port registers"]
    pub prt3: PRT,
    _reserved4: [u8; 0x1c],
    #[doc = "0x200..0x264 - GPIO port registers"]
    pub prt4: PRT,
    _reserved5: [u8; 0x1c],
    #[doc = "0x280..0x2e4 - GPIO port registers"]
    pub prt5: PRT,
    _reserved6: [u8; 0x1c],
    #[doc = "0x300..0x364 - GPIO port registers"]
    pub prt6: PRT,
    _reserved7: [u8; 0x1c],
    #[doc = "0x380..0x3e4 - GPIO port registers"]
    pub prt7: PRT,
    _reserved8: [u8; 0x1c],
    #[doc = "0x400..0x464 - GPIO port registers"]
    pub prt8: PRT,
    _reserved9: [u8; 0x1c],
    #[doc = "0x480..0x4e4 - GPIO port registers"]
    pub prt9: PRT,
    _reserved10: [u8; 0x1c],
    #[doc = "0x500..0x564 - GPIO port registers"]
    pub prt10: PRT,
    _reserved11: [u8; 0x1c],
    #[doc = "0x580..0x5e4 - GPIO port registers"]
    pub prt11: PRT,
    _reserved12: [u8; 0x1c],
    #[doc = "0x600..0x664 - GPIO port registers"]
    pub prt12: PRT,
    _reserved13: [u8; 0x1c],
    #[doc = "0x680..0x6e4 - GPIO port registers"]
    pub prt13: PRT,
    _reserved14: [u8; 0x1c],
    #[doc = "0x700..0x764 - GPIO port registers"]
    pub prt14: PRT,
    _reserved15: [u8; 0x1c],
    #[doc = "0x780..0x7e4 - GPIO port registers"]
    pub prt15: PRT,
    _reserved16: [u8; 0x1c],
    #[doc = "0x800..0x864 - GPIO port registers"]
    pub prt16: PRT,
    _reserved17: [u8; 0x1c],
    #[doc = "0x880..0x8e4 - GPIO port registers"]
    pub prt17: PRT,
    _reserved18: [u8; 0x1c],
    #[doc = "0x900..0x964 - GPIO port registers"]
    pub prt18: PRT,
    _reserved19: [u8; 0x1c],
    #[doc = "0x980..0x9e4 - GPIO port registers"]
    pub prt19: PRT,
    _reserved20: [u8; 0x1c],
    #[doc = "0xa00..0xa64 - GPIO port registers"]
    pub prt20: PRT,
    _reserved21: [u8; 0x1c],
    #[doc = "0xa80..0xae4 - GPIO port registers"]
    pub prt21: PRT,
    _reserved22: [u8; 0x1c],
    #[doc = "0xb00..0xb64 - GPIO port registers"]
    pub prt22: PRT,
    _reserved23: [u8; 0x1c],
    #[doc = "0xb80..0xbe4 - GPIO port registers"]
    pub prt23: PRT,
    _reserved24: [u8; 0x1c],
    #[doc = "0xc00..0xc64 - GPIO port registers"]
    pub prt24: PRT,
    _reserved25: [u8; 0x1c],
    #[doc = "0xc80..0xce4 - GPIO port registers"]
    pub prt25: PRT,
    _reserved26: [u8; 0x1c],
    #[doc = "0xd00..0xd64 - GPIO port registers"]
    pub prt26: PRT,
    _reserved27: [u8; 0x1c],
    #[doc = "0xd80..0xde4 - GPIO port registers"]
    pub prt27: PRT,
    _reserved28: [u8; 0x1c],
    #[doc = "0xe00..0xe64 - GPIO port registers"]
    pub prt28: PRT,
    _reserved29: [u8; 0x319c],
    #[doc = "0x4000 - Interrupt port cause register 0"]
    pub intr_cause0: INTR_CAUSE0,
    #[doc = "0x4004 - Interrupt port cause register 1"]
    pub intr_cause1: INTR_CAUSE1,
    #[doc = "0x4008 - Interrupt port cause register 2"]
    pub intr_cause2: INTR_CAUSE2,
    #[doc = "0x400c - Interrupt port cause register 3"]
    pub intr_cause3: INTR_CAUSE3,
    #[doc = "0x4010 - Extern power supply detection register"]
    pub vdd_active: VDD_ACTIVE,
    #[doc = "0x4014 - Supply detection interrupt register"]
    pub vdd_intr: VDD_INTR,
    #[doc = "0x4018 - Supply detection interrupt mask register"]
    pub vdd_intr_mask: VDD_INTR_MASK,
    #[doc = "0x401c - Supply detection interrupt masked register"]
    pub vdd_intr_masked: VDD_INTR_MASKED,
    #[doc = "0x4020 - Supply detection interrupt set register"]
    pub vdd_intr_set: VDD_INTR_SET,
}
#[doc = "GPIO port registers"]
pub use self::prt::PRT;
#[doc = r"Cluster"]
#[doc = "GPIO port registers"]
pub mod prt;
#[doc = "INTR_CAUSE0 (r) register accessor: an alias for `Reg<INTR_CAUSE0_SPEC>`"]
pub type INTR_CAUSE0 = crate::Reg<intr_cause0::INTR_CAUSE0_SPEC>;
#[doc = "Interrupt port cause register 0"]
pub mod intr_cause0;
#[doc = "INTR_CAUSE1 (r) register accessor: an alias for `Reg<INTR_CAUSE1_SPEC>`"]
pub type INTR_CAUSE1 = crate::Reg<intr_cause1::INTR_CAUSE1_SPEC>;
#[doc = "Interrupt port cause register 1"]
pub mod intr_cause1;
#[doc = "INTR_CAUSE2 (r) register accessor: an alias for `Reg<INTR_CAUSE2_SPEC>`"]
pub type INTR_CAUSE2 = crate::Reg<intr_cause2::INTR_CAUSE2_SPEC>;
#[doc = "Interrupt port cause register 2"]
pub mod intr_cause2;
#[doc = "INTR_CAUSE3 (r) register accessor: an alias for `Reg<INTR_CAUSE3_SPEC>`"]
pub type INTR_CAUSE3 = crate::Reg<intr_cause3::INTR_CAUSE3_SPEC>;
#[doc = "Interrupt port cause register 3"]
pub mod intr_cause3;
#[doc = "VDD_ACTIVE (r) register accessor: an alias for `Reg<VDD_ACTIVE_SPEC>`"]
pub type VDD_ACTIVE = crate::Reg<vdd_active::VDD_ACTIVE_SPEC>;
#[doc = "Extern power supply detection register"]
pub mod vdd_active;
#[doc = "VDD_INTR (rw) register accessor: an alias for `Reg<VDD_INTR_SPEC>`"]
pub type VDD_INTR = crate::Reg<vdd_intr::VDD_INTR_SPEC>;
#[doc = "Supply detection interrupt register"]
pub mod vdd_intr;
#[doc = "VDD_INTR_MASK (rw) register accessor: an alias for `Reg<VDD_INTR_MASK_SPEC>`"]
pub type VDD_INTR_MASK = crate::Reg<vdd_intr_mask::VDD_INTR_MASK_SPEC>;
#[doc = "Supply detection interrupt mask register"]
pub mod vdd_intr_mask;
#[doc = "VDD_INTR_MASKED (r) register accessor: an alias for `Reg<VDD_INTR_MASKED_SPEC>`"]
pub type VDD_INTR_MASKED = crate::Reg<vdd_intr_masked::VDD_INTR_MASKED_SPEC>;
#[doc = "Supply detection interrupt masked register"]
pub mod vdd_intr_masked;
#[doc = "VDD_INTR_SET (rw) register accessor: an alias for `Reg<VDD_INTR_SET_SPEC>`"]
pub type VDD_INTR_SET = crate::Reg<vdd_intr_set::VDD_INTR_SET_SPEC>;
#[doc = "Supply detection interrupt set register"]
pub mod vdd_intr_set;
