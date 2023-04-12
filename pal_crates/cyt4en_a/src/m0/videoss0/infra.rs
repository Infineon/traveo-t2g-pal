#[doc = r"Register block"]
#[repr(C)]
pub struct INFRA {
    #[doc = "0x00..0x80 - VRPU configuration."]
    pub vrpu: VRPU,
    _reserved1: [u8; 0x3f80],
    #[doc = "0x4000 - MPU configuration for read master 0 (not used)."]
    pub gfx_mpu_rd0: GFX_MPU_RD0,
    _reserved2: [u8; 0x03fc],
    #[doc = "0x4400 - MPU configuration for read master 1 (fetchdecode0)."]
    pub gfx_mpu_rd1: GFX_MPU_RD1,
    _reserved3: [u8; 0x03fc],
    #[doc = "0x4800 - MPU configuration for read master 2 (fetchlayer0)."]
    pub gfx_mpu_rd2: GFX_MPU_RD2,
    _reserved4: [u8; 0x03fc],
    #[doc = "0x4c00 - MPU configuration for read master 3 (fetchwarp1)."]
    pub gfx_mpu_rd3: GFX_MPU_RD3,
    _reserved5: [u8; 0x03fc],
    #[doc = "0x5000 - MPU configuration for read master 4 (fetcheco1)."]
    pub gfx_mpu_rd4: GFX_MPU_RD4,
    _reserved6: [u8; 0x03fc],
    #[doc = "0x5400 - MPU configuration for read master 5 (fetcheco4)."]
    pub gfx_mpu_rd5: GFX_MPU_RD5,
    _reserved7: [u8; 0x03fc],
    #[doc = "0x5800 - MPU configuration for read master 6 (fetchlayer1)."]
    pub gfx_mpu_rd6: GFX_MPU_RD6,
    _reserved8: [u8; 0x03fc],
    #[doc = "0x5c00 - MPU configuration for read master 7 (fetchdecode4)."]
    pub gfx_mpu_rd7: GFX_MPU_RD7,
    _reserved9: [u8; 0x03fc],
    #[doc = "0x6000 - MPU configuration for read master 8 (fetchblit0/1/2)."]
    pub gfx_mpu_rd8: GFX_MPU_RD8,
    _reserved10: [u8; 0x03fc],
    #[doc = "0x6400 - MPU configuration for read master 9 (fetchblit0/1/2)."]
    pub gfx_mpu_rd9: GFX_MPU_RD9,
    _reserved11: [u8; 0x03fc],
    #[doc = "0x6800 - MPU configuration for read master 10 (fetchblit0/1/2)."]
    pub gfx_mpu_rd10: GFX_MPU_RD10,
    _reserved12: [u8; 0x03fc],
    #[doc = "0x6c00 - MPU configuration for read master 11 (cmdseqrd)."]
    pub gfx_mpu_rd11: GFX_MPU_RD11,
    _reserved13: [u8; 0x03fc],
    #[doc = "0x7000 - MPU configuration for read master 12 (drawengrd)."]
    pub gfx_mpu_rd12: GFX_MPU_RD12,
    _reserved14: [u8; 0x03fc],
    #[doc = "0x7400 - MPU configuration for read master 13 (jpegdecrd)."]
    pub gfx_mpu_rd13: GFX_MPU_RD13,
    _reserved15: [u8; 0x03fc],
    #[doc = "0x7800 - MPU configuration for read master 14 (not used)."]
    pub gfx_mpu_rd14: GFX_MPU_RD14,
    _reserved16: [u8; 0x03fc],
    #[doc = "0x7c00 - MPU configuration for read master 15 (not used)."]
    pub gfx_mpu_rd15: GFX_MPU_RD15,
    _reserved17: [u8; 0x03fc],
    #[doc = "0x8000 - MPU configuration for write master 0 (not used)."]
    pub gfx_mpu_wr0: GFX_MPU_WR0,
    _reserved18: [u8; 0x03fc],
    #[doc = "0x8400 - MPU configuration for write master 1 (not used)."]
    pub gfx_mpu_wr1: GFX_MPU_WR1,
    _reserved19: [u8; 0x03fc],
    #[doc = "0x8800 - MPU configuration for write master 2 (not used)."]
    pub gfx_mpu_wr2: GFX_MPU_WR2,
    _reserved20: [u8; 0x03fc],
    #[doc = "0x8c00 - MPU configuration for write master 3 (not used)."]
    pub gfx_mpu_wr3: GFX_MPU_WR3,
    _reserved21: [u8; 0x03fc],
    #[doc = "0x9000 - MPU configuration for write master 4 (not used)."]
    pub gfx_mpu_wr4: GFX_MPU_WR4,
    _reserved22: [u8; 0x03fc],
    #[doc = "0x9400 - MPU configuration for write master 5 (not used)."]
    pub gfx_mpu_wr5: GFX_MPU_WR5,
    _reserved23: [u8; 0x03fc],
    #[doc = "0x9800 - MPU configuration for write master 6 (not used)."]
    pub gfx_mpu_wr6: GFX_MPU_WR6,
    _reserved24: [u8; 0x03fc],
    #[doc = "0x9c00 - MPU configuration for write master 7 (jpegdecwr)."]
    pub gfx_mpu_wr7: GFX_MPU_WR7,
    _reserved25: [u8; 0x03fc],
    #[doc = "0xa000 - MPU configuration for write master 8 (not used)."]
    pub gfx_mpu_wr8: GFX_MPU_WR8,
    _reserved26: [u8; 0x03fc],
    #[doc = "0xa400 - MPU configuration for write master 9 (not used)."]
    pub gfx_mpu_wr9: GFX_MPU_WR9,
    _reserved27: [u8; 0x03fc],
    #[doc = "0xa800 - MPU configuration for write master 10 (not used)."]
    pub gfx_mpu_wr10: GFX_MPU_WR10,
    _reserved28: [u8; 0x03fc],
    #[doc = "0xac00 - MPU configuration for write master 11 (not used)."]
    pub gfx_mpu_wr11: GFX_MPU_WR11,
    _reserved29: [u8; 0x03fc],
    #[doc = "0xb000 - MPU configuration for write master 12 (drawengwr)."]
    pub gfx_mpu_wr12: GFX_MPU_WR12,
    _reserved30: [u8; 0x03fc],
    #[doc = "0xb400 - MPU configuration for write master 13 (store4)."]
    pub gfx_mpu_wr13: GFX_MPU_WR13,
    _reserved31: [u8; 0x03fc],
    #[doc = "0xb800 - MPU configuration for write master 14 (storeblit)."]
    pub gfx_mpu_wr14: GFX_MPU_WR14,
    _reserved32: [u8; 0x03fc],
    #[doc = "0xbc00 - MPU configuration for write master 15 (not used)."]
    pub gfx_mpu_wr15: GFX_MPU_WR15,
}
#[doc = "VRPU configuration."]
pub use self::vrpu::VRPU;
#[doc = r"Cluster"]
#[doc = "VRPU configuration."]
pub mod vrpu;
#[doc = "MPU configuration for read master 0 (not used)."]
pub use self::gfx_mpu_rd0::GFX_MPU_RD0;
#[doc = r"Cluster"]
#[doc = "MPU configuration for read master 0 (not used)."]
pub mod gfx_mpu_rd0;
#[doc = "MPU configuration for read master 1 (fetchdecode0)."]
pub use self::gfx_mpu_rd1::GFX_MPU_RD1;
#[doc = r"Cluster"]
#[doc = "MPU configuration for read master 1 (fetchdecode0)."]
pub mod gfx_mpu_rd1;
#[doc = "MPU configuration for read master 2 (fetchlayer0)."]
pub use self::gfx_mpu_rd2::GFX_MPU_RD2;
#[doc = r"Cluster"]
#[doc = "MPU configuration for read master 2 (fetchlayer0)."]
pub mod gfx_mpu_rd2;
#[doc = "MPU configuration for read master 3 (fetchwarp1)."]
pub use self::gfx_mpu_rd3::GFX_MPU_RD3;
#[doc = r"Cluster"]
#[doc = "MPU configuration for read master 3 (fetchwarp1)."]
pub mod gfx_mpu_rd3;
#[doc = "MPU configuration for read master 4 (fetcheco1)."]
pub use self::gfx_mpu_rd4::GFX_MPU_RD4;
#[doc = r"Cluster"]
#[doc = "MPU configuration for read master 4 (fetcheco1)."]
pub mod gfx_mpu_rd4;
#[doc = "MPU configuration for read master 5 (fetcheco4)."]
pub use self::gfx_mpu_rd5::GFX_MPU_RD5;
#[doc = r"Cluster"]
#[doc = "MPU configuration for read master 5 (fetcheco4)."]
pub mod gfx_mpu_rd5;
#[doc = "MPU configuration for read master 6 (fetchlayer1)."]
pub use self::gfx_mpu_rd6::GFX_MPU_RD6;
#[doc = r"Cluster"]
#[doc = "MPU configuration for read master 6 (fetchlayer1)."]
pub mod gfx_mpu_rd6;
#[doc = "MPU configuration for read master 7 (fetchdecode4)."]
pub use self::gfx_mpu_rd7::GFX_MPU_RD7;
#[doc = r"Cluster"]
#[doc = "MPU configuration for read master 7 (fetchdecode4)."]
pub mod gfx_mpu_rd7;
#[doc = "MPU configuration for read master 8 (fetchblit0/1/2)."]
pub use self::gfx_mpu_rd8::GFX_MPU_RD8;
#[doc = r"Cluster"]
#[doc = "MPU configuration for read master 8 (fetchblit0/1/2)."]
pub mod gfx_mpu_rd8;
#[doc = "MPU configuration for read master 9 (fetchblit0/1/2)."]
pub use self::gfx_mpu_rd9::GFX_MPU_RD9;
#[doc = r"Cluster"]
#[doc = "MPU configuration for read master 9 (fetchblit0/1/2)."]
pub mod gfx_mpu_rd9;
#[doc = "MPU configuration for read master 10 (fetchblit0/1/2)."]
pub use self::gfx_mpu_rd10::GFX_MPU_RD10;
#[doc = r"Cluster"]
#[doc = "MPU configuration for read master 10 (fetchblit0/1/2)."]
pub mod gfx_mpu_rd10;
#[doc = "MPU configuration for read master 11 (cmdseqrd)."]
pub use self::gfx_mpu_rd11::GFX_MPU_RD11;
#[doc = r"Cluster"]
#[doc = "MPU configuration for read master 11 (cmdseqrd)."]
pub mod gfx_mpu_rd11;
#[doc = "MPU configuration for read master 12 (drawengrd)."]
pub use self::gfx_mpu_rd12::GFX_MPU_RD12;
#[doc = r"Cluster"]
#[doc = "MPU configuration for read master 12 (drawengrd)."]
pub mod gfx_mpu_rd12;
#[doc = "MPU configuration for read master 13 (jpegdecrd)."]
pub use self::gfx_mpu_rd13::GFX_MPU_RD13;
#[doc = r"Cluster"]
#[doc = "MPU configuration for read master 13 (jpegdecrd)."]
pub mod gfx_mpu_rd13;
#[doc = "MPU configuration for read master 14 (not used)."]
pub use self::gfx_mpu_rd14::GFX_MPU_RD14;
#[doc = r"Cluster"]
#[doc = "MPU configuration for read master 14 (not used)."]
pub mod gfx_mpu_rd14;
#[doc = "MPU configuration for read master 15 (not used)."]
pub use self::gfx_mpu_rd15::GFX_MPU_RD15;
#[doc = r"Cluster"]
#[doc = "MPU configuration for read master 15 (not used)."]
pub mod gfx_mpu_rd15;
#[doc = "MPU configuration for write master 0 (not used)."]
pub use self::gfx_mpu_wr0::GFX_MPU_WR0;
#[doc = r"Cluster"]
#[doc = "MPU configuration for write master 0 (not used)."]
pub mod gfx_mpu_wr0;
#[doc = "MPU configuration for write master 1 (not used)."]
pub use self::gfx_mpu_wr1::GFX_MPU_WR1;
#[doc = r"Cluster"]
#[doc = "MPU configuration for write master 1 (not used)."]
pub mod gfx_mpu_wr1;
#[doc = "MPU configuration for write master 2 (not used)."]
pub use self::gfx_mpu_wr2::GFX_MPU_WR2;
#[doc = r"Cluster"]
#[doc = "MPU configuration for write master 2 (not used)."]
pub mod gfx_mpu_wr2;
#[doc = "MPU configuration for write master 3 (not used)."]
pub use self::gfx_mpu_wr3::GFX_MPU_WR3;
#[doc = r"Cluster"]
#[doc = "MPU configuration for write master 3 (not used)."]
pub mod gfx_mpu_wr3;
#[doc = "MPU configuration for write master 4 (not used)."]
pub use self::gfx_mpu_wr4::GFX_MPU_WR4;
#[doc = r"Cluster"]
#[doc = "MPU configuration for write master 4 (not used)."]
pub mod gfx_mpu_wr4;
#[doc = "MPU configuration for write master 5 (not used)."]
pub use self::gfx_mpu_wr5::GFX_MPU_WR5;
#[doc = r"Cluster"]
#[doc = "MPU configuration for write master 5 (not used)."]
pub mod gfx_mpu_wr5;
#[doc = "MPU configuration for write master 6 (not used)."]
pub use self::gfx_mpu_wr6::GFX_MPU_WR6;
#[doc = r"Cluster"]
#[doc = "MPU configuration for write master 6 (not used)."]
pub mod gfx_mpu_wr6;
#[doc = "MPU configuration for write master 7 (jpegdecwr)."]
pub use self::gfx_mpu_wr7::GFX_MPU_WR7;
#[doc = r"Cluster"]
#[doc = "MPU configuration for write master 7 (jpegdecwr)."]
pub mod gfx_mpu_wr7;
#[doc = "MPU configuration for write master 8 (not used)."]
pub use self::gfx_mpu_wr8::GFX_MPU_WR8;
#[doc = r"Cluster"]
#[doc = "MPU configuration for write master 8 (not used)."]
pub mod gfx_mpu_wr8;
#[doc = "MPU configuration for write master 9 (not used)."]
pub use self::gfx_mpu_wr9::GFX_MPU_WR9;
#[doc = r"Cluster"]
#[doc = "MPU configuration for write master 9 (not used)."]
pub mod gfx_mpu_wr9;
#[doc = "MPU configuration for write master 10 (not used)."]
pub use self::gfx_mpu_wr10::GFX_MPU_WR10;
#[doc = r"Cluster"]
#[doc = "MPU configuration for write master 10 (not used)."]
pub mod gfx_mpu_wr10;
#[doc = "MPU configuration for write master 11 (not used)."]
pub use self::gfx_mpu_wr11::GFX_MPU_WR11;
#[doc = r"Cluster"]
#[doc = "MPU configuration for write master 11 (not used)."]
pub mod gfx_mpu_wr11;
#[doc = "MPU configuration for write master 12 (drawengwr)."]
pub use self::gfx_mpu_wr12::GFX_MPU_WR12;
#[doc = r"Cluster"]
#[doc = "MPU configuration for write master 12 (drawengwr)."]
pub mod gfx_mpu_wr12;
#[doc = "MPU configuration for write master 13 (store4)."]
pub use self::gfx_mpu_wr13::GFX_MPU_WR13;
#[doc = r"Cluster"]
#[doc = "MPU configuration for write master 13 (store4)."]
pub mod gfx_mpu_wr13;
#[doc = "MPU configuration for write master 14 (storeblit)."]
pub use self::gfx_mpu_wr14::GFX_MPU_WR14;
#[doc = r"Cluster"]
#[doc = "MPU configuration for write master 14 (storeblit)."]
pub mod gfx_mpu_wr14;
#[doc = "MPU configuration for write master 15 (not used)."]
pub use self::gfx_mpu_wr15::GFX_MPU_WR15;
#[doc = r"Cluster"]
#[doc = "MPU configuration for write master 15 (not used)."]
pub mod gfx_mpu_wr15;
