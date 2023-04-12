#[doc = r"Register block"]
#[repr(C)]
pub struct VRPU {
    #[doc = "0x00 - VRAM Protection for read master with ID=1"]
    pub rd1_ctl: RD1_CTL,
    #[doc = "0x04 - VRAM Protection for read master with ID=2"]
    pub rd2_ctl: RD2_CTL,
    #[doc = "0x08 - VRAM Protection for read master with ID=3"]
    pub rd3_ctl: RD3_CTL,
    #[doc = "0x0c - VRAM Protection for read master with ID=4"]
    pub rd4_ctl: RD4_CTL,
    #[doc = "0x10 - VRAM Protection for read master with ID=5"]
    pub rd5_ctl: RD5_CTL,
    #[doc = "0x14 - VRAM Protection for read master with ID=6"]
    pub rd6_ctl: RD6_CTL,
    #[doc = "0x18 - VRAM Protection for read master with ID=7"]
    pub rd7_ctl: RD7_CTL,
    #[doc = "0x1c - VRAM Protection for read master with ID=8"]
    pub rd8_ctl: RD8_CTL,
    #[doc = "0x20 - VRAM Protection for read master with ID=9"]
    pub rd9_ctl: RD9_CTL,
    #[doc = "0x24 - VRAM Protection for read master with ID=10"]
    pub rd10_ctl: RD10_CTL,
    #[doc = "0x28 - VRAM Protection for read master with ID=11"]
    pub rd11_ctl: RD11_CTL,
    #[doc = "0x2c - VRAM Protection for read master with ID=12"]
    pub rd12_ctl: RD12_CTL,
    #[doc = "0x30 - VRAM Protection for read master with ID=13"]
    pub rd13_ctl: RD13_CTL,
    #[doc = "0x34 - VRAM Protection for read master with ID=14"]
    pub rd14_ctl: RD14_CTL,
    #[doc = "0x38 - VRAM Protection for read master with ID=15"]
    pub rd15_ctl: RD15_CTL,
    #[doc = "0x3c - VRAM Protection for write master with ID=1"]
    pub wr1_ctl: WR1_CTL,
    #[doc = "0x40 - VRAM Protection for write master with ID=2"]
    pub wr2_ctl: WR2_CTL,
    #[doc = "0x44 - VRAM Protection for write master with ID=3"]
    pub wr3_ctl: WR3_CTL,
    #[doc = "0x48 - VRAM Protection for write master with ID=4"]
    pub wr4_ctl: WR4_CTL,
    #[doc = "0x4c - VRAM Protection for write master with ID=5"]
    pub wr5_ctl: WR5_CTL,
    #[doc = "0x50 - VRAM Protection for write master with ID=6"]
    pub wr6_ctl: WR6_CTL,
    #[doc = "0x54 - VRAM Protection for write master with ID=7"]
    pub wr7_ctl: WR7_CTL,
    #[doc = "0x58 - VRAM Protection for write master with ID=8"]
    pub wr8_ctl: WR8_CTL,
    #[doc = "0x5c - VRAM Protection for write master with ID=9"]
    pub wr9_ctl: WR9_CTL,
    #[doc = "0x60 - VRAM Protection for write master with ID=10"]
    pub wr10_ctl: WR10_CTL,
    #[doc = "0x64 - VRAM Protection for write master with ID=11"]
    pub wr11_ctl: WR11_CTL,
    #[doc = "0x68 - VRAM Protection for write master with ID=12"]
    pub wr12_ctl: WR12_CTL,
    #[doc = "0x6c - VRAM Protection for write master with ID=13"]
    pub wr13_ctl: WR13_CTL,
    #[doc = "0x70 - VRAM Protection for write master with ID=14"]
    pub wr14_ctl: WR14_CTL,
    #[doc = "0x74 - VRAM Protection for write master with ID=15"]
    pub wr15_ctl: WR15_CTL,
}
#[doc = "RD1_CTL (rw) register accessor: an alias for `Reg<RD1_CTL_SPEC>`"]
pub type RD1_CTL = crate::Reg<rd1_ctl::RD1_CTL_SPEC>;
#[doc = "VRAM Protection for read master with ID=1"]
pub mod rd1_ctl;
#[doc = "RD2_CTL (rw) register accessor: an alias for `Reg<RD2_CTL_SPEC>`"]
pub type RD2_CTL = crate::Reg<rd2_ctl::RD2_CTL_SPEC>;
#[doc = "VRAM Protection for read master with ID=2"]
pub mod rd2_ctl;
#[doc = "RD3_CTL (rw) register accessor: an alias for `Reg<RD3_CTL_SPEC>`"]
pub type RD3_CTL = crate::Reg<rd3_ctl::RD3_CTL_SPEC>;
#[doc = "VRAM Protection for read master with ID=3"]
pub mod rd3_ctl;
#[doc = "RD4_CTL (rw) register accessor: an alias for `Reg<RD4_CTL_SPEC>`"]
pub type RD4_CTL = crate::Reg<rd4_ctl::RD4_CTL_SPEC>;
#[doc = "VRAM Protection for read master with ID=4"]
pub mod rd4_ctl;
#[doc = "RD5_CTL (rw) register accessor: an alias for `Reg<RD5_CTL_SPEC>`"]
pub type RD5_CTL = crate::Reg<rd5_ctl::RD5_CTL_SPEC>;
#[doc = "VRAM Protection for read master with ID=5"]
pub mod rd5_ctl;
#[doc = "RD6_CTL (rw) register accessor: an alias for `Reg<RD6_CTL_SPEC>`"]
pub type RD6_CTL = crate::Reg<rd6_ctl::RD6_CTL_SPEC>;
#[doc = "VRAM Protection for read master with ID=6"]
pub mod rd6_ctl;
#[doc = "RD7_CTL (rw) register accessor: an alias for `Reg<RD7_CTL_SPEC>`"]
pub type RD7_CTL = crate::Reg<rd7_ctl::RD7_CTL_SPEC>;
#[doc = "VRAM Protection for read master with ID=7"]
pub mod rd7_ctl;
#[doc = "RD8_CTL (rw) register accessor: an alias for `Reg<RD8_CTL_SPEC>`"]
pub type RD8_CTL = crate::Reg<rd8_ctl::RD8_CTL_SPEC>;
#[doc = "VRAM Protection for read master with ID=8"]
pub mod rd8_ctl;
#[doc = "RD9_CTL (rw) register accessor: an alias for `Reg<RD9_CTL_SPEC>`"]
pub type RD9_CTL = crate::Reg<rd9_ctl::RD9_CTL_SPEC>;
#[doc = "VRAM Protection for read master with ID=9"]
pub mod rd9_ctl;
#[doc = "RD10_CTL (rw) register accessor: an alias for `Reg<RD10_CTL_SPEC>`"]
pub type RD10_CTL = crate::Reg<rd10_ctl::RD10_CTL_SPEC>;
#[doc = "VRAM Protection for read master with ID=10"]
pub mod rd10_ctl;
#[doc = "RD11_CTL (rw) register accessor: an alias for `Reg<RD11_CTL_SPEC>`"]
pub type RD11_CTL = crate::Reg<rd11_ctl::RD11_CTL_SPEC>;
#[doc = "VRAM Protection for read master with ID=11"]
pub mod rd11_ctl;
#[doc = "RD12_CTL (rw) register accessor: an alias for `Reg<RD12_CTL_SPEC>`"]
pub type RD12_CTL = crate::Reg<rd12_ctl::RD12_CTL_SPEC>;
#[doc = "VRAM Protection for read master with ID=12"]
pub mod rd12_ctl;
#[doc = "RD13_CTL (rw) register accessor: an alias for `Reg<RD13_CTL_SPEC>`"]
pub type RD13_CTL = crate::Reg<rd13_ctl::RD13_CTL_SPEC>;
#[doc = "VRAM Protection for read master with ID=13"]
pub mod rd13_ctl;
#[doc = "RD14_CTL (rw) register accessor: an alias for `Reg<RD14_CTL_SPEC>`"]
pub type RD14_CTL = crate::Reg<rd14_ctl::RD14_CTL_SPEC>;
#[doc = "VRAM Protection for read master with ID=14"]
pub mod rd14_ctl;
#[doc = "RD15_CTL (rw) register accessor: an alias for `Reg<RD15_CTL_SPEC>`"]
pub type RD15_CTL = crate::Reg<rd15_ctl::RD15_CTL_SPEC>;
#[doc = "VRAM Protection for read master with ID=15"]
pub mod rd15_ctl;
#[doc = "WR1_CTL (rw) register accessor: an alias for `Reg<WR1_CTL_SPEC>`"]
pub type WR1_CTL = crate::Reg<wr1_ctl::WR1_CTL_SPEC>;
#[doc = "VRAM Protection for write master with ID=1"]
pub mod wr1_ctl;
#[doc = "WR2_CTL (rw) register accessor: an alias for `Reg<WR2_CTL_SPEC>`"]
pub type WR2_CTL = crate::Reg<wr2_ctl::WR2_CTL_SPEC>;
#[doc = "VRAM Protection for write master with ID=2"]
pub mod wr2_ctl;
#[doc = "WR3_CTL (rw) register accessor: an alias for `Reg<WR3_CTL_SPEC>`"]
pub type WR3_CTL = crate::Reg<wr3_ctl::WR3_CTL_SPEC>;
#[doc = "VRAM Protection for write master with ID=3"]
pub mod wr3_ctl;
#[doc = "WR4_CTL (rw) register accessor: an alias for `Reg<WR4_CTL_SPEC>`"]
pub type WR4_CTL = crate::Reg<wr4_ctl::WR4_CTL_SPEC>;
#[doc = "VRAM Protection for write master with ID=4"]
pub mod wr4_ctl;
#[doc = "WR5_CTL (rw) register accessor: an alias for `Reg<WR5_CTL_SPEC>`"]
pub type WR5_CTL = crate::Reg<wr5_ctl::WR5_CTL_SPEC>;
#[doc = "VRAM Protection for write master with ID=5"]
pub mod wr5_ctl;
#[doc = "WR6_CTL (rw) register accessor: an alias for `Reg<WR6_CTL_SPEC>`"]
pub type WR6_CTL = crate::Reg<wr6_ctl::WR6_CTL_SPEC>;
#[doc = "VRAM Protection for write master with ID=6"]
pub mod wr6_ctl;
#[doc = "WR7_CTL (rw) register accessor: an alias for `Reg<WR7_CTL_SPEC>`"]
pub type WR7_CTL = crate::Reg<wr7_ctl::WR7_CTL_SPEC>;
#[doc = "VRAM Protection for write master with ID=7"]
pub mod wr7_ctl;
#[doc = "WR8_CTL (rw) register accessor: an alias for `Reg<WR8_CTL_SPEC>`"]
pub type WR8_CTL = crate::Reg<wr8_ctl::WR8_CTL_SPEC>;
#[doc = "VRAM Protection for write master with ID=8"]
pub mod wr8_ctl;
#[doc = "WR9_CTL (rw) register accessor: an alias for `Reg<WR9_CTL_SPEC>`"]
pub type WR9_CTL = crate::Reg<wr9_ctl::WR9_CTL_SPEC>;
#[doc = "VRAM Protection for write master with ID=9"]
pub mod wr9_ctl;
#[doc = "WR10_CTL (rw) register accessor: an alias for `Reg<WR10_CTL_SPEC>`"]
pub type WR10_CTL = crate::Reg<wr10_ctl::WR10_CTL_SPEC>;
#[doc = "VRAM Protection for write master with ID=10"]
pub mod wr10_ctl;
#[doc = "WR11_CTL (rw) register accessor: an alias for `Reg<WR11_CTL_SPEC>`"]
pub type WR11_CTL = crate::Reg<wr11_ctl::WR11_CTL_SPEC>;
#[doc = "VRAM Protection for write master with ID=11"]
pub mod wr11_ctl;
#[doc = "WR12_CTL (rw) register accessor: an alias for `Reg<WR12_CTL_SPEC>`"]
pub type WR12_CTL = crate::Reg<wr12_ctl::WR12_CTL_SPEC>;
#[doc = "VRAM Protection for write master with ID=12"]
pub mod wr12_ctl;
#[doc = "WR13_CTL (rw) register accessor: an alias for `Reg<WR13_CTL_SPEC>`"]
pub type WR13_CTL = crate::Reg<wr13_ctl::WR13_CTL_SPEC>;
#[doc = "VRAM Protection for write master with ID=13"]
pub mod wr13_ctl;
#[doc = "WR14_CTL (rw) register accessor: an alias for `Reg<WR14_CTL_SPEC>`"]
pub type WR14_CTL = crate::Reg<wr14_ctl::WR14_CTL_SPEC>;
#[doc = "VRAM Protection for write master with ID=14"]
pub mod wr14_ctl;
#[doc = "WR15_CTL (rw) register accessor: an alias for `Reg<WR15_CTL_SPEC>`"]
pub type WR15_CTL = crate::Reg<wr15_ctl::WR15_CTL_SPEC>;
#[doc = "VRAM Protection for write master with ID=15"]
pub mod wr15_ctl;
