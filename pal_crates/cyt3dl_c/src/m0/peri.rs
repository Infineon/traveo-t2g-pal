#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0200],
    #[doc = "0x200 - Timeout control"]
    pub timeout_ctl: TIMEOUT_CTL,
    _reserved1: [u8; 0x1c],
    #[doc = "0x220 - Trigger command"]
    pub tr_cmd: TR_CMD,
    _reserved2: [u8; 0x1ddc],
    #[doc = "0x2000 - ECC control"]
    pub ecc_ctl: ECC_CTL,
    _reserved3: [u8; 0x1ffc],
    #[doc = "0x4000..0x4014 - Peripheral group structure"]
    pub gr0: GR,
    _reserved4: [u8; 0x2c],
    #[doc = "0x4040..0x4054 - Peripheral group structure"]
    pub gr1: GR,
    _reserved5: [u8; 0x2c],
    #[doc = "0x4080..0x4094 - Peripheral group structure"]
    pub gr2: GR,
    _reserved6: [u8; 0x2c],
    #[doc = "0x40c0..0x40d4 - Peripheral group structure"]
    pub gr3: GR,
    _reserved7: [u8; 0x2c],
    #[doc = "0x4100..0x4114 - Peripheral group structure"]
    pub gr4: GR,
    _reserved8: [u8; 0x2c],
    #[doc = "0x4140..0x4154 - Peripheral group structure"]
    pub gr5: GR,
    _reserved9: [u8; 0x2c],
    #[doc = "0x4180..0x4194 - Peripheral group structure"]
    pub gr6: GR,
    _reserved10: [u8; 0x2c],
    #[doc = "0x41c0..0x41d4 - Peripheral group structure"]
    pub gr7: GR,
    _reserved11: [u8; 0x2c],
    #[doc = "0x4200..0x4214 - Peripheral group structure"]
    pub gr8: GR,
    _reserved12: [u8; 0x2c],
    #[doc = "0x4240..0x4254 - Peripheral group structure"]
    pub gr9: GR,
    _reserved13: [u8; 0x2c],
    #[doc = "0x4280..0x4294 - Peripheral group structure"]
    pub gr10: GR,
    _reserved14: [u8; 0x3d6c],
    #[doc = "0x8000..0xb800 - Trigger group"]
    pub tr_gr: [TR_GR; 14],
    _reserved15: [u8; 0x0800],
    #[doc = "0xc000..0xe000 - Trigger 1-to-1 group"]
    pub tr_1to1_gr: [TR_1TO1_GR; 8],
}
#[doc = "TIMEOUT_CTL (rw) register accessor: an alias for `Reg<TIMEOUT_CTL_SPEC>`"]
pub type TIMEOUT_CTL = crate::Reg<timeout_ctl::TIMEOUT_CTL_SPEC>;
#[doc = "Timeout control"]
pub mod timeout_ctl;
#[doc = "TR_CMD (rw) register accessor: an alias for `Reg<TR_CMD_SPEC>`"]
pub type TR_CMD = crate::Reg<tr_cmd::TR_CMD_SPEC>;
#[doc = "Trigger command"]
pub mod tr_cmd;
#[doc = "ECC_CTL (rw) register accessor: an alias for `Reg<ECC_CTL_SPEC>`"]
pub type ECC_CTL = crate::Reg<ecc_ctl::ECC_CTL_SPEC>;
#[doc = "ECC control"]
pub mod ecc_ctl;
#[doc = "Peripheral group structure"]
pub use self::gr::GR;
#[doc = r"Cluster"]
#[doc = "Peripheral group structure"]
pub mod gr;
#[doc = "Trigger group"]
pub use self::tr_gr::TR_GR;
#[doc = r"Cluster"]
#[doc = "Trigger group"]
pub mod tr_gr;
#[doc = "Trigger 1-to-1 group"]
pub use self::tr_1to1_gr::TR_1TO1_GR;
#[doc = r"Cluster"]
#[doc = "Trigger 1-to-1 group"]
pub mod tr_1to1_gr;
