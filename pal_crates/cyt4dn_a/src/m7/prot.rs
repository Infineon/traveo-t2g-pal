#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00..0x23e8 - SMPU"]
    pub smpu: SMPU,
    _reserved1: [u8; 0x1c18],
    #[doc = "0x4000..0x43e8 - MPU"]
    pub mpu0: MPU,
    _reserved2: [u8; 0x18],
    #[doc = "0x4400..0x47e8 - MPU"]
    pub mpu1: MPU,
    _reserved3: [u8; 0x18],
    #[doc = "0x4800..0x4be8 - MPU"]
    pub mpu2: MPU,
    _reserved4: [u8; 0x18],
    #[doc = "0x4c00..0x4fe8 - MPU"]
    pub mpu3: MPU,
    _reserved5: [u8; 0x18],
    #[doc = "0x5000..0x53e8 - MPU"]
    pub mpu4: MPU,
    _reserved6: [u8; 0x18],
    #[doc = "0x5400..0x57e8 - MPU"]
    pub mpu5: MPU,
    _reserved7: [u8; 0x18],
    #[doc = "0x5800..0x5be8 - MPU"]
    pub mpu6: MPU,
    _reserved8: [u8; 0x18],
    #[doc = "0x5c00..0x5fe8 - MPU"]
    pub mpu7: MPU,
    _reserved9: [u8; 0x18],
    #[doc = "0x6000..0x63e8 - MPU"]
    pub mpu8: MPU,
    _reserved10: [u8; 0x18],
    #[doc = "0x6400..0x67e8 - MPU"]
    pub mpu9: MPU,
    _reserved11: [u8; 0x18],
    #[doc = "0x6800..0x6be8 - MPU"]
    pub mpu10: MPU,
    _reserved12: [u8; 0x18],
    #[doc = "0x6c00..0x6fe8 - MPU"]
    pub mpu11: MPU,
    _reserved13: [u8; 0x18],
    #[doc = "0x7000..0x73e8 - MPU"]
    pub mpu12: MPU,
    _reserved14: [u8; 0x18],
    #[doc = "0x7400..0x77e8 - MPU"]
    pub mpu13: MPU,
    _reserved15: [u8; 0x18],
    #[doc = "0x7800..0x7be8 - MPU"]
    pub mpu14: MPU,
    _reserved16: [u8; 0x18],
    #[doc = "0x7c00..0x7fe8 - MPU"]
    pub mpu15: MPU,
}
#[doc = "SMPU"]
pub use self::smpu::SMPU;
#[doc = r"Cluster"]
#[doc = "SMPU"]
pub mod smpu;
#[doc = "MPU"]
pub use self::mpu::MPU;
#[doc = r"Cluster"]
#[doc = "MPU"]
pub mod mpu;
