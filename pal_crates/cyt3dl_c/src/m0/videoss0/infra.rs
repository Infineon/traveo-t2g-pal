#[doc = r"Register block"]
#[repr(C)]
pub struct INFRA {
    #[doc = "0x00..0x21e8 - VRPU configuration."]
    pub vrpu: VRPU,
    _reserved1: [u8; 0x1e18],
    #[doc = "0x4000 - MPU configuration for read masters."]
    pub gfx_mpu_rd0: GFX_MPU_RD,
    _reserved2: [u8; 0x03fc],
    #[doc = "0x4400 - MPU configuration for read masters."]
    pub gfx_mpu_rd1: GFX_MPU_RD,
    _reserved3: [u8; 0x03fc],
    #[doc = "0x4800 - MPU configuration for read masters."]
    pub gfx_mpu_rd2: GFX_MPU_RD,
    _reserved4: [u8; 0x03fc],
    #[doc = "0x4c00 - MPU configuration for read masters."]
    pub gfx_mpu_rd3: GFX_MPU_RD,
    _reserved5: [u8; 0x03fc],
    #[doc = "0x5000 - MPU configuration for read masters."]
    pub gfx_mpu_rd4: GFX_MPU_RD,
    _reserved6: [u8; 0x03fc],
    #[doc = "0x5400 - MPU configuration for read masters."]
    pub gfx_mpu_rd5: GFX_MPU_RD,
    _reserved7: [u8; 0x03fc],
    #[doc = "0x5800 - MPU configuration for read masters."]
    pub gfx_mpu_rd6: GFX_MPU_RD,
    _reserved8: [u8; 0x03fc],
    #[doc = "0x5c00 - MPU configuration for read masters."]
    pub gfx_mpu_rd7: GFX_MPU_RD,
    _reserved9: [u8; 0x03fc],
    #[doc = "0x6000 - MPU configuration for read masters."]
    pub gfx_mpu_rd8: GFX_MPU_RD,
    _reserved10: [u8; 0x03fc],
    #[doc = "0x6400 - MPU configuration for read masters."]
    pub gfx_mpu_rd9: GFX_MPU_RD,
    _reserved11: [u8; 0x03fc],
    #[doc = "0x6800 - MPU configuration for read masters."]
    pub gfx_mpu_rd10: GFX_MPU_RD,
    _reserved12: [u8; 0x03fc],
    #[doc = "0x6c00 - MPU configuration for read masters."]
    pub gfx_mpu_rd11: GFX_MPU_RD,
    _reserved13: [u8; 0x03fc],
    #[doc = "0x7000 - MPU configuration for read masters."]
    pub gfx_mpu_rd12: GFX_MPU_RD,
    _reserved14: [u8; 0x03fc],
    #[doc = "0x7400 - MPU configuration for read masters."]
    pub gfx_mpu_rd13: GFX_MPU_RD,
    _reserved15: [u8; 0x03fc],
    #[doc = "0x7800 - MPU configuration for read masters."]
    pub gfx_mpu_rd14: GFX_MPU_RD,
    _reserved16: [u8; 0x03fc],
    #[doc = "0x7c00 - MPU configuration for read masters."]
    pub gfx_mpu_rd15: GFX_MPU_RD,
    _reserved17: [u8; 0x03fc],
    #[doc = "0x8000 - MPU configuration for write masters."]
    pub gfx_mpu_wr0: GFX_MPU_WR,
    _reserved18: [u8; 0x03fc],
    #[doc = "0x8400 - MPU configuration for write masters."]
    pub gfx_mpu_wr1: GFX_MPU_WR,
    _reserved19: [u8; 0x03fc],
    #[doc = "0x8800 - MPU configuration for write masters."]
    pub gfx_mpu_wr2: GFX_MPU_WR,
    _reserved20: [u8; 0x03fc],
    #[doc = "0x8c00 - MPU configuration for write masters."]
    pub gfx_mpu_wr3: GFX_MPU_WR,
    _reserved21: [u8; 0x03fc],
    #[doc = "0x9000 - MPU configuration for write masters."]
    pub gfx_mpu_wr4: GFX_MPU_WR,
    _reserved22: [u8; 0x03fc],
    #[doc = "0x9400 - MPU configuration for write masters."]
    pub gfx_mpu_wr5: GFX_MPU_WR,
    _reserved23: [u8; 0x03fc],
    #[doc = "0x9800 - MPU configuration for write masters."]
    pub gfx_mpu_wr6: GFX_MPU_WR,
    _reserved24: [u8; 0x03fc],
    #[doc = "0x9c00 - MPU configuration for write masters."]
    pub gfx_mpu_wr7: GFX_MPU_WR,
    _reserved25: [u8; 0x03fc],
    #[doc = "0xa000 - MPU configuration for write masters."]
    pub gfx_mpu_wr8: GFX_MPU_WR,
    _reserved26: [u8; 0x03fc],
    #[doc = "0xa400 - MPU configuration for write masters."]
    pub gfx_mpu_wr9: GFX_MPU_WR,
    _reserved27: [u8; 0x03fc],
    #[doc = "0xa800 - MPU configuration for write masters."]
    pub gfx_mpu_wr10: GFX_MPU_WR,
    _reserved28: [u8; 0x03fc],
    #[doc = "0xac00 - MPU configuration for write masters."]
    pub gfx_mpu_wr11: GFX_MPU_WR,
    _reserved29: [u8; 0x03fc],
    #[doc = "0xb000 - MPU configuration for write masters."]
    pub gfx_mpu_wr12: GFX_MPU_WR,
    _reserved30: [u8; 0x03fc],
    #[doc = "0xb400 - MPU configuration for write masters."]
    pub gfx_mpu_wr13: GFX_MPU_WR,
    _reserved31: [u8; 0x03fc],
    #[doc = "0xb800 - MPU configuration for write masters."]
    pub gfx_mpu_wr14: GFX_MPU_WR,
    _reserved32: [u8; 0x03fc],
    #[doc = "0xbc00 - MPU configuration for write masters."]
    pub gfx_mpu_wr15: GFX_MPU_WR,
}
#[doc = "VRPU configuration."]
pub use self::vrpu::VRPU;
#[doc = r"Cluster"]
#[doc = "VRPU configuration."]
pub mod vrpu;
#[doc = "MPU configuration for read masters."]
pub use self::gfx_mpu_rd::GFX_MPU_RD;
#[doc = r"Cluster"]
#[doc = "MPU configuration for read masters."]
pub mod gfx_mpu_rd;
#[doc = "MPU configuration for write masters."]
pub use self::gfx_mpu_wr::GFX_MPU_WR;
#[doc = r"Cluster"]
#[doc = "MPU configuration for write masters."]
pub mod gfx_mpu_wr;
