#[doc = r"Register block"]
#[repr(C)]
pub struct INFRA {
    #[doc = "0x00..0x78 - MPU configuration."]
    pub gfx_mpu: GFX_MPU,
    _reserved1: [u8; 0x0388],
    #[doc = "0x400..0x478 - VRPU configuration."]
    pub vrpu: VRPU,
    _reserved2: [u8; 0x0b88],
    #[doc = "0x1000..0x1010 - VRPU structures."]
    pub vrpu_struct: VRPU_STRUCT,
}
#[doc = "MPU configuration."]
pub use self::gfx_mpu::GFX_MPU;
#[doc = r"Cluster"]
#[doc = "MPU configuration."]
pub mod gfx_mpu;
#[doc = "VRPU configuration."]
pub use self::vrpu::VRPU;
#[doc = r"Cluster"]
#[doc = "VRPU configuration."]
pub mod vrpu;
#[doc = "VRPU structures."]
pub use self::vrpu_struct::VRPU_STRUCT;
#[doc = r"Cluster"]
#[doc = "VRPU structures."]
pub mod vrpu_struct;
