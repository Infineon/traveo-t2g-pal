#[doc = r"Register block"]
#[repr(C)]
pub struct FRAMEGEN0 {
    #[doc = "0x00 - FrameGen Static Control Register"]
    pub fgstctrl: FGSTCTRL,
    #[doc = "0x04 - FrameGen Horizontal Timing Config Register 1"]
    pub htcfg1: HTCFG1,
    #[doc = "0x08 - FrameGen Horizontal Timing Config Register 2"]
    pub htcfg2: HTCFG2,
    #[doc = "0x0c - FrameGen Vertical Timing Config Register 1"]
    pub vtcfg1: VTCFG1,
    #[doc = "0x10 - FrameGen Vertical Timing Config Register 2"]
    pub vtcfg2: VTCFG2,
    #[doc = "0x14 - Coordinates of the trigger point for generation of the Int0 interrupt signal"]
    pub int0config: INT0CONFIG,
    #[doc = "0x18 - Coordinates of the trigger point for generation of the Int1 interrupt signal"]
    pub int1config: INT1CONFIG,
    #[doc = "0x1c - Coordinates of the trigger point for generation of the Int2 interrupt signal"]
    pub int2config: INT2CONFIG,
    #[doc = "0x20 - Coordinates of the trigger point for generation of the Int3 interrupt signal"]
    pub int3config: INT3CONFIG,
    #[doc = "0x24 - Coordinates of the trigger point for generation of the primary kick signal"]
    pub pkickconfig: PKICKCONFIG,
    #[doc = "0x28 - Coordinates of the trigger point for generation of the secondary kick signal"]
    pub skickconfig: SKICKCONFIG,
    #[doc = "0x2c - Configuration register for controlling the behavior of the SecSyncStat field in the FgSecChStat register."]
    pub secstatconfig: SECSTATCONFIG,
    #[doc = "0x30 - FrameGen Skew Regulation Control Register 1."]
    pub fgsrcr1: FGSRCR1,
    #[doc = "0x34 - FrameGen Skew Regulation Control Register 2"]
    pub fgsrcr2: FGSRCR2,
    #[doc = "0x38 - FrameGen Skew Regulation Control Register 3"]
    pub fgsrcr3: FGSRCR3,
    #[doc = "0x3c - FrameGen Skew Regulation Control Register 4"]
    pub fgsrcr4: FGSRCR4,
    #[doc = "0x40 - FrameGen Skew Regulation Control Register 5"]
    pub fgsrcr5: FGSRCR5,
    #[doc = "0x44 - FrameGen Skew Regulation Control Register 6"]
    pub fgsrcr6: FGSRCR6,
    #[doc = "0x48 - FrameGen Kick System Debug Register"]
    pub fgksdr: FGKSDR,
    #[doc = "0x4c - FrameGen Primary Area Config Register 1 (shadowed)"]
    pub pacfg: PACFG,
    #[doc = "0x50 - FrameGen Secondary Area Config Register 1 (shadowed)"]
    pub sacfg: SACFG,
    #[doc = "0x54 - FrameGen Input Control Register (shadowed)"]
    pub fginctrl: FGINCTRL,
    #[doc = "0x58 - FrameGen Input Control Panic Register (shadowed)"]
    pub fginctrlpanic: FGINCTRLPANIC,
    #[doc = "0x5c - FrameGen Constant Color Register (shadowed)"]
    pub fgccr: FGCCR,
    #[doc = "0x60 - Options for blend operations. (shadowed)"]
    pub blendcontrol: BLENDCONTROL,
    #[doc = "0x64 - FrameGen Enable Register"]
    pub fgenable: FGENABLE,
    #[doc = "0x68 - FrameGen Shadow Load Register"]
    pub fgslr: FGSLR,
    #[doc = "0x6c - FrameGen Enable Status Register"]
    pub fgensts: FGENSTS,
    #[doc = "0x70 - Time stamp status."]
    pub fgtimestamp: FGTIMESTAMP,
    #[doc = "0x74 - FrameGen Channel Status Register"]
    pub fgchstat: FGCHSTAT,
    #[doc = "0x78 - FrameGen Channel Status Clear Register"]
    pub fgchstatclr: FGCHSTATCLR,
    #[doc = "0x7c - FrameGen Skew Monitor Register for Secondary Channel Skew Control"]
    pub fgskewmon: FGSKEWMON,
    #[doc = "0x80 - FrameGen Secondary FIFO Min Fill Register"]
    pub fgsfifomin: FGSFIFOMIN,
    #[doc = "0x84 - FrameGen Secondary FIFO Max Fill Register"]
    pub fgsfifomax: FGSFIFOMAX,
    #[doc = "0x88 - FrameGen Secondary FIFO Fill Clear Register"]
    pub fgsfifofillclr: FGSFIFOFILLCLR,
    #[doc = "0x8c - FrameGen Skew Regulation ExtraPolation Debug Register"]
    pub fgsrepd: FGSREPD,
    #[doc = "0x90 - FrameGen Skew Regulation Frame Total Debug Register"]
    pub fgsrftd: FGSRFTD,
    #[doc = "0x94 - Current (x,y) positions of the timing generator register in FrameGen timing generator."]
    pub fgtgposition: FGTGPOSITION,
}
#[doc = "FGSTCTRL (rw) register accessor: an alias for `Reg<FGSTCTRL_SPEC>`"]
pub type FGSTCTRL = crate::Reg<fgstctrl::FGSTCTRL_SPEC>;
#[doc = "FrameGen Static Control Register"]
pub mod fgstctrl;
#[doc = "HTCFG1 (rw) register accessor: an alias for `Reg<HTCFG1_SPEC>`"]
pub type HTCFG1 = crate::Reg<htcfg1::HTCFG1_SPEC>;
#[doc = "FrameGen Horizontal Timing Config Register 1"]
pub mod htcfg1;
#[doc = "HTCFG2 (rw) register accessor: an alias for `Reg<HTCFG2_SPEC>`"]
pub type HTCFG2 = crate::Reg<htcfg2::HTCFG2_SPEC>;
#[doc = "FrameGen Horizontal Timing Config Register 2"]
pub mod htcfg2;
#[doc = "VTCFG1 (rw) register accessor: an alias for `Reg<VTCFG1_SPEC>`"]
pub type VTCFG1 = crate::Reg<vtcfg1::VTCFG1_SPEC>;
#[doc = "FrameGen Vertical Timing Config Register 1"]
pub mod vtcfg1;
#[doc = "VTCFG2 (rw) register accessor: an alias for `Reg<VTCFG2_SPEC>`"]
pub type VTCFG2 = crate::Reg<vtcfg2::VTCFG2_SPEC>;
#[doc = "FrameGen Vertical Timing Config Register 2"]
pub mod vtcfg2;
#[doc = "INT0CONFIG (rw) register accessor: an alias for `Reg<INT0CONFIG_SPEC>`"]
pub type INT0CONFIG = crate::Reg<int0config::INT0CONFIG_SPEC>;
#[doc = "Coordinates of the trigger point for generation of the Int0 interrupt signal"]
pub mod int0config;
#[doc = "INT1CONFIG (rw) register accessor: an alias for `Reg<INT1CONFIG_SPEC>`"]
pub type INT1CONFIG = crate::Reg<int1config::INT1CONFIG_SPEC>;
#[doc = "Coordinates of the trigger point for generation of the Int1 interrupt signal"]
pub mod int1config;
#[doc = "INT2CONFIG (rw) register accessor: an alias for `Reg<INT2CONFIG_SPEC>`"]
pub type INT2CONFIG = crate::Reg<int2config::INT2CONFIG_SPEC>;
#[doc = "Coordinates of the trigger point for generation of the Int2 interrupt signal"]
pub mod int2config;
#[doc = "INT3CONFIG (rw) register accessor: an alias for `Reg<INT3CONFIG_SPEC>`"]
pub type INT3CONFIG = crate::Reg<int3config::INT3CONFIG_SPEC>;
#[doc = "Coordinates of the trigger point for generation of the Int3 interrupt signal"]
pub mod int3config;
#[doc = "PKICKCONFIG (rw) register accessor: an alias for `Reg<PKICKCONFIG_SPEC>`"]
pub type PKICKCONFIG = crate::Reg<pkickconfig::PKICKCONFIG_SPEC>;
#[doc = "Coordinates of the trigger point for generation of the primary kick signal"]
pub mod pkickconfig;
#[doc = "SKICKCONFIG (rw) register accessor: an alias for `Reg<SKICKCONFIG_SPEC>`"]
pub type SKICKCONFIG = crate::Reg<skickconfig::SKICKCONFIG_SPEC>;
#[doc = "Coordinates of the trigger point for generation of the secondary kick signal"]
pub mod skickconfig;
#[doc = "SECSTATCONFIG (rw) register accessor: an alias for `Reg<SECSTATCONFIG_SPEC>`"]
pub type SECSTATCONFIG = crate::Reg<secstatconfig::SECSTATCONFIG_SPEC>;
#[doc = "Configuration register for controlling the behavior of the SecSyncStat field in the FgSecChStat register."]
pub mod secstatconfig;
#[doc = "FGSRCR1 (rw) register accessor: an alias for `Reg<FGSRCR1_SPEC>`"]
pub type FGSRCR1 = crate::Reg<fgsrcr1::FGSRCR1_SPEC>;
#[doc = "FrameGen Skew Regulation Control Register 1."]
pub mod fgsrcr1;
#[doc = "FGSRCR2 (rw) register accessor: an alias for `Reg<FGSRCR2_SPEC>`"]
pub type FGSRCR2 = crate::Reg<fgsrcr2::FGSRCR2_SPEC>;
#[doc = "FrameGen Skew Regulation Control Register 2"]
pub mod fgsrcr2;
#[doc = "FGSRCR3 (rw) register accessor: an alias for `Reg<FGSRCR3_SPEC>`"]
pub type FGSRCR3 = crate::Reg<fgsrcr3::FGSRCR3_SPEC>;
#[doc = "FrameGen Skew Regulation Control Register 3"]
pub mod fgsrcr3;
#[doc = "FGSRCR4 (rw) register accessor: an alias for `Reg<FGSRCR4_SPEC>`"]
pub type FGSRCR4 = crate::Reg<fgsrcr4::FGSRCR4_SPEC>;
#[doc = "FrameGen Skew Regulation Control Register 4"]
pub mod fgsrcr4;
#[doc = "FGSRCR5 (rw) register accessor: an alias for `Reg<FGSRCR5_SPEC>`"]
pub type FGSRCR5 = crate::Reg<fgsrcr5::FGSRCR5_SPEC>;
#[doc = "FrameGen Skew Regulation Control Register 5"]
pub mod fgsrcr5;
#[doc = "FGSRCR6 (rw) register accessor: an alias for `Reg<FGSRCR6_SPEC>`"]
pub type FGSRCR6 = crate::Reg<fgsrcr6::FGSRCR6_SPEC>;
#[doc = "FrameGen Skew Regulation Control Register 6"]
pub mod fgsrcr6;
#[doc = "FGKSDR (rw) register accessor: an alias for `Reg<FGKSDR_SPEC>`"]
pub type FGKSDR = crate::Reg<fgksdr::FGKSDR_SPEC>;
#[doc = "FrameGen Kick System Debug Register"]
pub mod fgksdr;
#[doc = "PACFG (rw) register accessor: an alias for `Reg<PACFG_SPEC>`"]
pub type PACFG = crate::Reg<pacfg::PACFG_SPEC>;
#[doc = "FrameGen Primary Area Config Register 1 (shadowed)"]
pub mod pacfg;
#[doc = "SACFG (rw) register accessor: an alias for `Reg<SACFG_SPEC>`"]
pub type SACFG = crate::Reg<sacfg::SACFG_SPEC>;
#[doc = "FrameGen Secondary Area Config Register 1 (shadowed)"]
pub mod sacfg;
#[doc = "FGINCTRL (rw) register accessor: an alias for `Reg<FGINCTRL_SPEC>`"]
pub type FGINCTRL = crate::Reg<fginctrl::FGINCTRL_SPEC>;
#[doc = "FrameGen Input Control Register (shadowed)"]
pub mod fginctrl;
#[doc = "FGINCTRLPANIC (rw) register accessor: an alias for `Reg<FGINCTRLPANIC_SPEC>`"]
pub type FGINCTRLPANIC = crate::Reg<fginctrlpanic::FGINCTRLPANIC_SPEC>;
#[doc = "FrameGen Input Control Panic Register (shadowed)"]
pub mod fginctrlpanic;
#[doc = "FGCCR (rw) register accessor: an alias for `Reg<FGCCR_SPEC>`"]
pub type FGCCR = crate::Reg<fgccr::FGCCR_SPEC>;
#[doc = "FrameGen Constant Color Register (shadowed)"]
pub mod fgccr;
#[doc = "BLENDCONTROL (rw) register accessor: an alias for `Reg<BLENDCONTROL_SPEC>`"]
pub type BLENDCONTROL = crate::Reg<blendcontrol::BLENDCONTROL_SPEC>;
#[doc = "Options for blend operations. (shadowed)"]
pub mod blendcontrol;
#[doc = "FGENABLE (rw) register accessor: an alias for `Reg<FGENABLE_SPEC>`"]
pub type FGENABLE = crate::Reg<fgenable::FGENABLE_SPEC>;
#[doc = "FrameGen Enable Register"]
pub mod fgenable;
#[doc = "FGSLR (w) register accessor: an alias for `Reg<FGSLR_SPEC>`"]
pub type FGSLR = crate::Reg<fgslr::FGSLR_SPEC>;
#[doc = "FrameGen Shadow Load Register"]
pub mod fgslr;
#[doc = "FGENSTS (r) register accessor: an alias for `Reg<FGENSTS_SPEC>`"]
pub type FGENSTS = crate::Reg<fgensts::FGENSTS_SPEC>;
#[doc = "FrameGen Enable Status Register"]
pub mod fgensts;
#[doc = "FGTIMESTAMP (r) register accessor: an alias for `Reg<FGTIMESTAMP_SPEC>`"]
pub type FGTIMESTAMP = crate::Reg<fgtimestamp::FGTIMESTAMP_SPEC>;
#[doc = "Time stamp status."]
pub mod fgtimestamp;
#[doc = "FGCHSTAT (r) register accessor: an alias for `Reg<FGCHSTAT_SPEC>`"]
pub type FGCHSTAT = crate::Reg<fgchstat::FGCHSTAT_SPEC>;
#[doc = "FrameGen Channel Status Register"]
pub mod fgchstat;
#[doc = "FGCHSTATCLR (w) register accessor: an alias for `Reg<FGCHSTATCLR_SPEC>`"]
pub type FGCHSTATCLR = crate::Reg<fgchstatclr::FGCHSTATCLR_SPEC>;
#[doc = "FrameGen Channel Status Clear Register"]
pub mod fgchstatclr;
#[doc = "FGSKEWMON (r) register accessor: an alias for `Reg<FGSKEWMON_SPEC>`"]
pub type FGSKEWMON = crate::Reg<fgskewmon::FGSKEWMON_SPEC>;
#[doc = "FrameGen Skew Monitor Register for Secondary Channel Skew Control"]
pub mod fgskewmon;
#[doc = "FGSFIFOMIN (r) register accessor: an alias for `Reg<FGSFIFOMIN_SPEC>`"]
pub type FGSFIFOMIN = crate::Reg<fgsfifomin::FGSFIFOMIN_SPEC>;
#[doc = "FrameGen Secondary FIFO Min Fill Register"]
pub mod fgsfifomin;
#[doc = "FGSFIFOMAX (r) register accessor: an alias for `Reg<FGSFIFOMAX_SPEC>`"]
pub type FGSFIFOMAX = crate::Reg<fgsfifomax::FGSFIFOMAX_SPEC>;
#[doc = "FrameGen Secondary FIFO Max Fill Register"]
pub mod fgsfifomax;
#[doc = "FGSFIFOFILLCLR (w) register accessor: an alias for `Reg<FGSFIFOFILLCLR_SPEC>`"]
pub type FGSFIFOFILLCLR = crate::Reg<fgsfifofillclr::FGSFIFOFILLCLR_SPEC>;
#[doc = "FrameGen Secondary FIFO Fill Clear Register"]
pub mod fgsfifofillclr;
#[doc = "FGSREPD (r) register accessor: an alias for `Reg<FGSREPD_SPEC>`"]
pub type FGSREPD = crate::Reg<fgsrepd::FGSREPD_SPEC>;
#[doc = "FrameGen Skew Regulation ExtraPolation Debug Register"]
pub mod fgsrepd;
#[doc = "FGSRFTD (r) register accessor: an alias for `Reg<FGSRFTD_SPEC>`"]
pub type FGSRFTD = crate::Reg<fgsrftd::FGSRFTD_SPEC>;
#[doc = "FrameGen Skew Regulation Frame Total Debug Register"]
pub mod fgsrftd;
#[doc = "FGTGPOSITION (r) register accessor: an alias for `Reg<FGTGPOSITION_SPEC>`"]
pub type FGTGPOSITION = crate::Reg<fgtgposition::FGTGPOSITION_SPEC>;
#[doc = "Current (x,y) positions of the timing generator register in FrameGen timing generator."]
pub mod fgtgposition;
