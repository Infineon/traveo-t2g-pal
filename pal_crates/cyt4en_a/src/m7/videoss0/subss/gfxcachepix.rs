#[doc = r"Register block"]
#[repr(C)]
pub struct GFXCACHEPIX {
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
    #[doc = "0x8c - Cache bypass counter."]
    pub ch1_bypass: CH1_BYPASS,
    #[doc = "0x90 - Cache miss counter."]
    pub ch1_miss: CH1_MISS,
    #[doc = "0x94 - Cache hit counter."]
    pub ch1_hit: CH1_HIT,
    #[doc = "0x98 - Cache bypass counter."]
    pub ch2_bypass: CH2_BYPASS,
    #[doc = "0x9c - Cache miss counter."]
    pub ch2_miss: CH2_MISS,
    #[doc = "0xa0 - Cache hit counter."]
    pub ch2_hit: CH2_HIT,
    #[doc = "0xa4 - Cache bypass counter."]
    pub ch3_bypass: CH3_BYPASS,
    #[doc = "0xa8 - Cache miss counter."]
    pub ch3_miss: CH3_MISS,
    #[doc = "0xac - Cache hit counter."]
    pub ch3_hit: CH3_HIT,
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
#[doc = "CH1_BYPASS (r) register accessor: an alias for `Reg<CH1_BYPASS_SPEC>`"]
pub type CH1_BYPASS = crate::Reg<ch1_bypass::CH1_BYPASS_SPEC>;
#[doc = "Cache bypass counter."]
pub mod ch1_bypass;
#[doc = "CH1_MISS (r) register accessor: an alias for `Reg<CH1_MISS_SPEC>`"]
pub type CH1_MISS = crate::Reg<ch1_miss::CH1_MISS_SPEC>;
#[doc = "Cache miss counter."]
pub mod ch1_miss;
#[doc = "CH1_HIT (r) register accessor: an alias for `Reg<CH1_HIT_SPEC>`"]
pub type CH1_HIT = crate::Reg<ch1_hit::CH1_HIT_SPEC>;
#[doc = "Cache hit counter."]
pub mod ch1_hit;
#[doc = "CH2_BYPASS (r) register accessor: an alias for `Reg<CH2_BYPASS_SPEC>`"]
pub type CH2_BYPASS = crate::Reg<ch2_bypass::CH2_BYPASS_SPEC>;
#[doc = "Cache bypass counter."]
pub mod ch2_bypass;
#[doc = "CH2_MISS (r) register accessor: an alias for `Reg<CH2_MISS_SPEC>`"]
pub type CH2_MISS = crate::Reg<ch2_miss::CH2_MISS_SPEC>;
#[doc = "Cache miss counter."]
pub mod ch2_miss;
#[doc = "CH2_HIT (r) register accessor: an alias for `Reg<CH2_HIT_SPEC>`"]
pub type CH2_HIT = crate::Reg<ch2_hit::CH2_HIT_SPEC>;
#[doc = "Cache hit counter."]
pub mod ch2_hit;
#[doc = "CH3_BYPASS (r) register accessor: an alias for `Reg<CH3_BYPASS_SPEC>`"]
pub type CH3_BYPASS = crate::Reg<ch3_bypass::CH3_BYPASS_SPEC>;
#[doc = "Cache bypass counter."]
pub mod ch3_bypass;
#[doc = "CH3_MISS (r) register accessor: an alias for `Reg<CH3_MISS_SPEC>`"]
pub type CH3_MISS = crate::Reg<ch3_miss::CH3_MISS_SPEC>;
#[doc = "Cache miss counter."]
pub mod ch3_miss;
#[doc = "CH3_HIT (r) register accessor: an alias for `Reg<CH3_HIT_SPEC>`"]
pub type CH3_HIT = crate::Reg<ch3_hit::CH3_HIT_SPEC>;
#[doc = "Cache hit counter."]
pub mod ch3_hit;
