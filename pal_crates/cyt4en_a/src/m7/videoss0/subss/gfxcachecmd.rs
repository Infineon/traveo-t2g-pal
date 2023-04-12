#[doc = r"Register block"]
#[repr(C)]
pub struct GFXCACHECMD {
    #[doc = "0x00 - Static control register."]
    pub static_control: STATIC_CONTROL,
    _reserved1: [u8; 0x1c],
    #[doc = "0x20 - Control of the GfxCache."]
    pub control: CONTROL,
    _reserved2: [u8; 0x1c],
    #[doc = "0x40 - Status information"]
    pub status: STATUS,
    _reserved3: [u8; 0x3c],
    #[doc = "0x80 - Cache bypass counter."]
    pub ch0_bypass: CH0_BYPASS,
    #[doc = "0x84 - Cache miss counter."]
    pub ch0_miss: CH0_MISS,
    #[doc = "0x88 - Cache hit counter."]
    pub ch0_hit: CH0_HIT,
}
#[doc = "STATIC_CONTROL (rw) register accessor: an alias for `Reg<STATIC_CONTROL_SPEC>`"]
pub type STATIC_CONTROL = crate::Reg<static_control::STATIC_CONTROL_SPEC>;
#[doc = "Static control register."]
pub mod static_control;
#[doc = "CONTROL (w) register accessor: an alias for `Reg<CONTROL_SPEC>`"]
pub type CONTROL = crate::Reg<control::CONTROL_SPEC>;
#[doc = "Control of the GfxCache."]
pub mod control;
#[doc = "STATUS (r) register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Status information"]
pub mod status;
#[doc = "CH0_BYPASS (r) register accessor: an alias for `Reg<CH0_BYPASS_SPEC>`"]
pub type CH0_BYPASS = crate::Reg<ch0_bypass::CH0_BYPASS_SPEC>;
#[doc = "Cache bypass counter."]
pub mod ch0_bypass;
#[doc = "CH0_MISS (r) register accessor: an alias for `Reg<CH0_MISS_SPEC>`"]
pub type CH0_MISS = crate::Reg<ch0_miss::CH0_MISS_SPEC>;
#[doc = "Cache miss counter."]
pub mod ch0_miss;
#[doc = "CH0_HIT (r) register accessor: an alias for `Reg<CH0_HIT_SPEC>`"]
pub type CH0_HIT = crate::Reg<ch0_hit::CH0_HIT_SPEC>;
#[doc = "Cache hit counter."]
pub mod ch0_hit;
