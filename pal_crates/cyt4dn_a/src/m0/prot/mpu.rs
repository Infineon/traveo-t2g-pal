#[doc = r"Register block"]
#[repr(C)]
pub struct MPU {
    #[doc = "0x00 - Master control"]
    pub ms_ctl: MS_CTL,
    #[doc = "0x04..0x200 - Master control read mirror"]
    pub ms_ctl_read_mir: [MS_CTL_READ_MIR; 127],
    #[doc = "0x200..0x208 - MPU structure"]
    pub mpu_struct0: MPU_STRUCT,
    _reserved3: [u8; 0x18],
    #[doc = "0x220..0x228 - MPU structure"]
    pub mpu_struct1: MPU_STRUCT,
    _reserved4: [u8; 0x18],
    #[doc = "0x240..0x248 - MPU structure"]
    pub mpu_struct2: MPU_STRUCT,
    _reserved5: [u8; 0x18],
    #[doc = "0x260..0x268 - MPU structure"]
    pub mpu_struct3: MPU_STRUCT,
    _reserved6: [u8; 0x18],
    #[doc = "0x280..0x288 - MPU structure"]
    pub mpu_struct4: MPU_STRUCT,
    _reserved7: [u8; 0x18],
    #[doc = "0x2a0..0x2a8 - MPU structure"]
    pub mpu_struct5: MPU_STRUCT,
    _reserved8: [u8; 0x18],
    #[doc = "0x2c0..0x2c8 - MPU structure"]
    pub mpu_struct6: MPU_STRUCT,
    _reserved9: [u8; 0x18],
    #[doc = "0x2e0..0x2e8 - MPU structure"]
    pub mpu_struct7: MPU_STRUCT,
    _reserved10: [u8; 0x18],
    #[doc = "0x300..0x308 - MPU structure"]
    pub mpu_struct8: MPU_STRUCT,
    _reserved11: [u8; 0x18],
    #[doc = "0x320..0x328 - MPU structure"]
    pub mpu_struct9: MPU_STRUCT,
    _reserved12: [u8; 0x18],
    #[doc = "0x340..0x348 - MPU structure"]
    pub mpu_struct10: MPU_STRUCT,
    _reserved13: [u8; 0x18],
    #[doc = "0x360..0x368 - MPU structure"]
    pub mpu_struct11: MPU_STRUCT,
    _reserved14: [u8; 0x18],
    #[doc = "0x380..0x388 - MPU structure"]
    pub mpu_struct12: MPU_STRUCT,
    _reserved15: [u8; 0x18],
    #[doc = "0x3a0..0x3a8 - MPU structure"]
    pub mpu_struct13: MPU_STRUCT,
    _reserved16: [u8; 0x18],
    #[doc = "0x3c0..0x3c8 - MPU structure"]
    pub mpu_struct14: MPU_STRUCT,
    _reserved17: [u8; 0x18],
    #[doc = "0x3e0..0x3e8 - MPU structure"]
    pub mpu_struct15: MPU_STRUCT,
}
#[doc = "MS_CTL (rw) register accessor: an alias for `Reg<MS_CTL_SPEC>`"]
pub type MS_CTL = crate::Reg<ms_ctl::MS_CTL_SPEC>;
#[doc = "Master control"]
pub mod ms_ctl;
#[doc = "MS_CTL_READ_MIR (r) register accessor: an alias for `Reg<MS_CTL_READ_MIR_SPEC>`"]
pub type MS_CTL_READ_MIR = crate::Reg<ms_ctl_read_mir::MS_CTL_READ_MIR_SPEC>;
#[doc = "Master control read mirror"]
pub mod ms_ctl_read_mir;
#[doc = "MPU structure"]
pub use self::mpu_struct::MPU_STRUCT;
#[doc = r"Cluster"]
#[doc = "MPU structure"]
pub mod mpu_struct;
