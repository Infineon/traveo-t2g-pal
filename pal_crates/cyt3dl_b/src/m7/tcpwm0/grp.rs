#[doc = r"Register block"]
#[repr(C)]
pub struct GRP {
    #[doc = "0x00..0x1300 - Timer/Counter/PWM Counter Module"]
    pub cnt: [CNT; 38],
}
#[doc = "Timer/Counter/PWM Counter Module"]
pub use self::cnt::CNT;
#[doc = r"Cluster"]
#[doc = "Timer/Counter/PWM Counter Module"]
pub mod cnt;
