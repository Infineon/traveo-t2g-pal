#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Identity"]
    pub identity: IDENTITY,
    #[doc = "0x04 - CM7 0 status"]
    pub cm7_0_status: CM7_0_STATUS,
    #[doc = "0x08 - Fast 0 clock control"]
    pub fast_0_clock_ctl: FAST_0_CLOCK_CTL,
    #[doc = "0x0c - CM7 0 control"]
    pub cm7_0_ctl: CM7_0_CTL,
    _reserved4: [u8; 0xf0],
    #[doc = "0x100..0x140 - CM7 0 interrupt status"]
    pub cm7_0_int_status: [CM7_0_INT_STATUS; 16],
    _reserved5: [u8; 0xc0],
    #[doc = "0x200 - CM7 0 vector table base"]
    pub cm7_0_vector_table_base: CM7_0_VECTOR_TABLE_BASE,
    _reserved6: [u8; 0x3c],
    #[doc = "0x240..0x250 - CM7 0 NMI control"]
    pub cm7_0_nmi_ctl: [CM7_0_NMI_CTL; 4],
    _reserved7: [u8; 0xb0],
    #[doc = "0x300 - UDB power control"]
    pub udb_pwr_ctl: UDB_PWR_CTL,
    #[doc = "0x304 - UDB power control"]
    pub udb_pwr_delay_ctl: UDB_PWR_DELAY_CTL,
    _reserved9: [u8; 0x18],
    #[doc = "0x320 - Trace and debug clock control"]
    pub trc_dbg_clock_ctl: TRC_DBG_CLOCK_CTL,
    _reserved10: [u8; 0xe0],
    #[doc = "0x404 - CM7 1status"]
    pub cm7_1_status: CM7_1_STATUS,
    #[doc = "0x408 - Fast 1 clock control"]
    pub fast_1_clock_ctl: FAST_1_CLOCK_CTL,
    #[doc = "0x40c - CM7 1 control"]
    pub cm7_1_ctl: CM7_1_CTL,
    _reserved13: [u8; 0xf0],
    #[doc = "0x500..0x540 - CM7 1 interrupt status"]
    pub cm7_1_int_status: [CM7_1_INT_STATUS; 16],
    _reserved14: [u8; 0xc0],
    #[doc = "0x600 - CM7 1 vector table base"]
    pub cm7_1_vector_table_base: CM7_1_VECTOR_TABLE_BASE,
    _reserved15: [u8; 0x3c],
    #[doc = "0x640..0x650 - CM7 1 NMI control"]
    pub cm7_1_nmi_ctl: [CM7_1_NMI_CTL; 4],
    _reserved16: [u8; 0x01b4],
    #[doc = "0x804 - CM7 2 status"]
    pub cm7_2_status: CM7_2_STATUS,
    #[doc = "0x808 - Fast 2 clock control"]
    pub fast_2_clock_ctl: FAST_2_CLOCK_CTL,
    #[doc = "0x80c - CM7 2 control"]
    pub cm7_2_ctl: CM7_2_CTL,
    _reserved19: [u8; 0xf0],
    #[doc = "0x900..0x940 - CM7 2 interrupt status"]
    pub cm7_2_int_status: [CM7_2_INT_STATUS; 16],
    _reserved20: [u8; 0xc0],
    #[doc = "0xa00 - CM7 2 vector table base"]
    pub cm7_2_vector_table_base: CM7_2_VECTOR_TABLE_BASE,
    _reserved21: [u8; 0x3c],
    #[doc = "0xa40..0xa50 - CM7 2 NMI control"]
    pub cm7_2_nmi_ctl: [CM7_2_NMI_CTL; 4],
    _reserved22: [u8; 0x01b4],
    #[doc = "0xc04 - CM7 3 status"]
    pub cm7_3_status: CM7_3_STATUS,
    #[doc = "0xc08 - Fast 3 clock control"]
    pub fast_3_clock_ctl: FAST_3_CLOCK_CTL,
    #[doc = "0xc0c - CM7 3 control"]
    pub cm7_3_ctl: CM7_3_CTL,
    _reserved25: [u8; 0xf0],
    #[doc = "0xd00..0xd40 - CM7 3 interrupt status"]
    pub cm7_3_int_status: [CM7_3_INT_STATUS; 16],
    _reserved26: [u8; 0xc0],
    #[doc = "0xe00 - CM7 3 vector table base"]
    pub cm7_3_vector_table_base: CM7_3_VECTOR_TABLE_BASE,
    _reserved27: [u8; 0x3c],
    #[doc = "0xe40..0xe50 - CM7 3 NMI control"]
    pub cm7_3_nmi_ctl: [CM7_3_NMI_CTL; 4],
    _reserved28: [u8; 0x01b0],
    #[doc = "0x1000 - CM0+ control"]
    pub cm0_ctl: CM0_CTL,
    #[doc = "0x1004 - CM0+ status"]
    pub cm0_status: CM0_STATUS,
    #[doc = "0x1008 - Slow clock control"]
    pub slow_clock_ctl: SLOW_CLOCK_CTL,
    #[doc = "0x100c - Peripheral interconnect clock control"]
    pub peri_clock_ctl: PERI_CLOCK_CTL,
    #[doc = "0x1010 - Memory clock control"]
    pub mem_clock_ctl: MEM_CLOCK_CTL,
    _reserved33: [u8; 0xec],
    #[doc = "0x1100 - CM0+ interrupt 0 status"]
    pub cm0_int0_status: CM0_INT0_STATUS,
    #[doc = "0x1104 - CM0+ interrupt 1 status"]
    pub cm0_int1_status: CM0_INT1_STATUS,
    #[doc = "0x1108 - CM0+ interrupt 2 status"]
    pub cm0_int2_status: CM0_INT2_STATUS,
    #[doc = "0x110c - CM0+ interrupt 3 status"]
    pub cm0_int3_status: CM0_INT3_STATUS,
    #[doc = "0x1110 - CM0+ interrupt 4 status"]
    pub cm0_int4_status: CM0_INT4_STATUS,
    #[doc = "0x1114 - CM0+ interrupt 5 status"]
    pub cm0_int5_status: CM0_INT5_STATUS,
    #[doc = "0x1118 - CM0+ interrupt 6 status"]
    pub cm0_int6_status: CM0_INT6_STATUS,
    #[doc = "0x111c - CM0+ interrupt 7 status"]
    pub cm0_int7_status: CM0_INT7_STATUS,
    #[doc = "0x1120 - CM0+ vector table base"]
    pub cm0_vector_table_base: CM0_VECTOR_TABLE_BASE,
    _reserved42: [u8; 0x1c],
    #[doc = "0x1140..0x1150 - CM0+ NMI control"]
    pub cm0_nmi_ctl: [CM0_NMI_CTL; 4],
    _reserved43: [u8; 0xb0],
    #[doc = "0x1200 - CM7 0 power control"]
    pub cm7_0_pwr_ctl: CM7_0_PWR_CTL,
    #[doc = "0x1204 - CM7 0 power delay control"]
    pub cm7_0_pwr_delay_ctl: CM7_0_PWR_DELAY_CTL,
    _reserved45: [u8; 0x08],
    #[doc = "0x1210 - CM7 1 power control"]
    pub cm7_1_pwr_ctl: CM7_1_PWR_CTL,
    #[doc = "0x1214 - CM7 1 power delay control"]
    pub cm7_1_pwr_delay_ctl: CM7_1_PWR_DELAY_CTL,
    _reserved47: [u8; 0x08],
    #[doc = "0x1220 - CM7 2 power control"]
    pub cm7_2_pwr_ctl: CM7_2_PWR_CTL,
    #[doc = "0x1224 - CM7 2 power delay control"]
    pub cm7_2_pwr_delay_ctl: CM7_2_PWR_DELAY_CTL,
    _reserved49: [u8; 0x08],
    #[doc = "0x1230 - CM7 3 power control"]
    pub cm7_3_pwr_ctl: CM7_3_PWR_CTL,
    #[doc = "0x1234 - CM7 3 power delay control"]
    pub cm7_3_pwr_delay_ctl: CM7_3_PWR_DELAY_CTL,
    _reserved51: [u8; 0xc8],
    #[doc = "0x1300 - RAM 0 control"]
    pub ram0_ctl0: RAM0_CTL0,
    #[doc = "0x1304 - RAM 0 status"]
    pub ram0_status: RAM0_STATUS,
    _reserved53: [u8; 0x38],
    #[doc = "0x1340..0x1380 - RAM 0 power control"]
    pub ram0_pwr_macro_ctl: [RAM0_PWR_MACRO_CTL; 16],
    #[doc = "0x1380 - RAM 1 control"]
    pub ram1_ctl0: RAM1_CTL0,
    #[doc = "0x1384 - RAM 1 status"]
    pub ram1_status: RAM1_STATUS,
    #[doc = "0x1388 - RAM 1 power control"]
    pub ram1_pwr_ctl: RAM1_PWR_CTL,
    _reserved57: [u8; 0x14],
    #[doc = "0x13a0 - RAM 2 control"]
    pub ram2_ctl0: RAM2_CTL0,
    #[doc = "0x13a4 - RAM 2 status"]
    pub ram2_status: RAM2_STATUS,
    #[doc = "0x13a8 - RAM 2 power control"]
    pub ram2_pwr_ctl: RAM2_PWR_CTL,
    _reserved60: [u8; 0x14],
    #[doc = "0x13c0 - Power up delay used for all SRAM power domains"]
    pub ram_pwr_delay_ctl: RAM_PWR_DELAY_CTL,
    #[doc = "0x13c4 - ROM control"]
    pub rom_ctl: ROM_CTL,
    #[doc = "0x13c8 - ECC control"]
    pub ecc_ctl: ECC_CTL,
    _reserved63: [u8; 0x34],
    #[doc = "0x1400 - Product identifier and version (same as CoreSight RomTables)"]
    pub product_id: PRODUCT_ID,
    _reserved64: [u8; 0x0c],
    #[doc = "0x1410 - Debug port status"]
    pub dp_status: DP_STATUS,
    #[doc = "0x1414 - Access port control"]
    pub ap_ctl: AP_CTL,
    _reserved66: [u8; 0xe8],
    #[doc = "0x1500 - Buffer control"]
    pub buff_ctl: BUFF_CTL,
    _reserved67: [u8; 0xfc],
    #[doc = "0x1600 - SysTick timer control"]
    pub systick_ctl: SYSTICK_CTL,
    _reserved68: [u8; 0x0100],
    #[doc = "0x1704 - Memory BIST status"]
    pub mbist_stat: MBIST_STAT,
    _reserved69: [u8; 0xf8],
    #[doc = "0x1800 - Calibration support set and read"]
    pub cal_sup_set: CAL_SUP_SET,
    #[doc = "0x1804 - Calibration support clear and reset"]
    pub cal_sup_clr: CAL_SUP_CLR,
    _reserved71: [u8; 0x07f8],
    #[doc = "0x2000 - CM0+ protection context control"]
    pub cm0_pc_ctl: CM0_PC_CTL,
    _reserved72: [u8; 0x3c],
    #[doc = "0x2040 - CM0+ protection context 0 handler"]
    pub cm0_pc0_handler: CM0_PC0_HANDLER,
    #[doc = "0x2044 - CM0+ protection context 1 handler"]
    pub cm0_pc1_handler: CM0_PC1_HANDLER,
    #[doc = "0x2048 - CM0+ protection context 2 handler"]
    pub cm0_pc2_handler: CM0_PC2_HANDLER,
    #[doc = "0x204c - CM0+ protection context 3 handler"]
    pub cm0_pc3_handler: CM0_PC3_HANDLER,
    _reserved76: [u8; 0x74],
    #[doc = "0x20c4 - Protection status"]
    pub protection: PROTECTION,
    _reserved77: [u8; 0x38],
    #[doc = "0x2100 - ROM trim control"]
    pub trim_rom_ctl: TRIM_ROM_CTL,
    #[doc = "0x2104 - RAM trim control for less than 100MHz SRAMs"]
    pub trim_ram_ctl: TRIM_RAM_CTL,
    #[doc = "0x2108 - RAM trim control for 100MHz - 200MHz SRAMs"]
    pub trim_ram200_ctl: TRIM_RAM200_CTL,
    #[doc = "0x210c - RAM trim control for more than 200MHz SRAMs"]
    pub trim_ram350_ctl: TRIM_RAM350_CTL,
    _reserved81: [u8; 0x5ef0],
    #[doc = "0x8000..0x8ffc - CM0+ system interrupt control"]
    pub cm0_system_int_ctl: [CM0_SYSTEM_INT_CTL; 1023],
    _reserved82: [u8; 0x1004],
    #[doc = "0xa000..0xaffc - CM7 0 system interrupt control"]
    pub cm7_0_system_int_ctl: [CM7_0_SYSTEM_INT_CTL; 1023],
    _reserved83: [u8; 0x1004],
    #[doc = "0xc000..0xcffc - CM7 1 system interrupt control"]
    pub cm7_1_system_int_ctl: [CM7_1_SYSTEM_INT_CTL; 1023],
    _reserved84: [u8; 0x04],
    #[doc = "0xd000..0xdffc - CM7 2 system interrupt control"]
    pub cm7_2_system_int_ctl: [CM7_2_SYSTEM_INT_CTL; 1023],
    _reserved85: [u8; 0x04],
    #[doc = "0xe000..0xeffc - CM7 3 system interrupt control"]
    pub cm7_3_system_int_ctl: [CM7_3_SYSTEM_INT_CTL; 1023],
}
#[doc = "IDENTITY (r) register accessor: an alias for `Reg<IDENTITY_SPEC>`"]
pub type IDENTITY = crate::Reg<identity::IDENTITY_SPEC>;
#[doc = "Identity"]
pub mod identity;
#[doc = "CM7_0_STATUS (r) register accessor: an alias for `Reg<CM7_0_STATUS_SPEC>`"]
pub type CM7_0_STATUS = crate::Reg<cm7_0_status::CM7_0_STATUS_SPEC>;
#[doc = "CM7 0 status"]
pub mod cm7_0_status;
#[doc = "FAST_0_CLOCK_CTL (rw) register accessor: an alias for `Reg<FAST_0_CLOCK_CTL_SPEC>`"]
pub type FAST_0_CLOCK_CTL = crate::Reg<fast_0_clock_ctl::FAST_0_CLOCK_CTL_SPEC>;
#[doc = "Fast 0 clock control"]
pub mod fast_0_clock_ctl;
#[doc = "CM7_0_CTL (rw) register accessor: an alias for `Reg<CM7_0_CTL_SPEC>`"]
pub type CM7_0_CTL = crate::Reg<cm7_0_ctl::CM7_0_CTL_SPEC>;
#[doc = "CM7 0 control"]
pub mod cm7_0_ctl;
#[doc = "CM7_0_INT_STATUS (r) register accessor: an alias for `Reg<CM7_0_INT_STATUS_SPEC>`"]
pub type CM7_0_INT_STATUS = crate::Reg<cm7_0_int_status::CM7_0_INT_STATUS_SPEC>;
#[doc = "CM7 0 interrupt status"]
pub mod cm7_0_int_status;
#[doc = "CM7_0_VECTOR_TABLE_BASE (rw) register accessor: an alias for `Reg<CM7_0_VECTOR_TABLE_BASE_SPEC>`"]
pub type CM7_0_VECTOR_TABLE_BASE =
    crate::Reg<cm7_0_vector_table_base::CM7_0_VECTOR_TABLE_BASE_SPEC>;
#[doc = "CM7 0 vector table base"]
pub mod cm7_0_vector_table_base;
#[doc = "CM7_0_NMI_CTL (rw) register accessor: an alias for `Reg<CM7_0_NMI_CTL_SPEC>`"]
pub type CM7_0_NMI_CTL = crate::Reg<cm7_0_nmi_ctl::CM7_0_NMI_CTL_SPEC>;
#[doc = "CM7 0 NMI control"]
pub mod cm7_0_nmi_ctl;
#[doc = "UDB_PWR_CTL (rw) register accessor: an alias for `Reg<UDB_PWR_CTL_SPEC>`"]
pub type UDB_PWR_CTL = crate::Reg<udb_pwr_ctl::UDB_PWR_CTL_SPEC>;
#[doc = "UDB power control"]
pub mod udb_pwr_ctl;
#[doc = "UDB_PWR_DELAY_CTL (rw) register accessor: an alias for `Reg<UDB_PWR_DELAY_CTL_SPEC>`"]
pub type UDB_PWR_DELAY_CTL = crate::Reg<udb_pwr_delay_ctl::UDB_PWR_DELAY_CTL_SPEC>;
#[doc = "UDB power control"]
pub mod udb_pwr_delay_ctl;
#[doc = "TRC_DBG_CLOCK_CTL (rw) register accessor: an alias for `Reg<TRC_DBG_CLOCK_CTL_SPEC>`"]
pub type TRC_DBG_CLOCK_CTL = crate::Reg<trc_dbg_clock_ctl::TRC_DBG_CLOCK_CTL_SPEC>;
#[doc = "Trace and debug clock control"]
pub mod trc_dbg_clock_ctl;
#[doc = "CM7_1_STATUS (r) register accessor: an alias for `Reg<CM7_1_STATUS_SPEC>`"]
pub type CM7_1_STATUS = crate::Reg<cm7_1_status::CM7_1_STATUS_SPEC>;
#[doc = "CM7 1status"]
pub mod cm7_1_status;
#[doc = "FAST_1_CLOCK_CTL (rw) register accessor: an alias for `Reg<FAST_1_CLOCK_CTL_SPEC>`"]
pub type FAST_1_CLOCK_CTL = crate::Reg<fast_1_clock_ctl::FAST_1_CLOCK_CTL_SPEC>;
#[doc = "Fast 1 clock control"]
pub mod fast_1_clock_ctl;
#[doc = "CM7_1_CTL (rw) register accessor: an alias for `Reg<CM7_1_CTL_SPEC>`"]
pub type CM7_1_CTL = crate::Reg<cm7_1_ctl::CM7_1_CTL_SPEC>;
#[doc = "CM7 1 control"]
pub mod cm7_1_ctl;
#[doc = "CM7_1_INT_STATUS (r) register accessor: an alias for `Reg<CM7_1_INT_STATUS_SPEC>`"]
pub type CM7_1_INT_STATUS = crate::Reg<cm7_1_int_status::CM7_1_INT_STATUS_SPEC>;
#[doc = "CM7 1 interrupt status"]
pub mod cm7_1_int_status;
#[doc = "CM7_1_VECTOR_TABLE_BASE (rw) register accessor: an alias for `Reg<CM7_1_VECTOR_TABLE_BASE_SPEC>`"]
pub type CM7_1_VECTOR_TABLE_BASE =
    crate::Reg<cm7_1_vector_table_base::CM7_1_VECTOR_TABLE_BASE_SPEC>;
#[doc = "CM7 1 vector table base"]
pub mod cm7_1_vector_table_base;
#[doc = "CM7_1_NMI_CTL (rw) register accessor: an alias for `Reg<CM7_1_NMI_CTL_SPEC>`"]
pub type CM7_1_NMI_CTL = crate::Reg<cm7_1_nmi_ctl::CM7_1_NMI_CTL_SPEC>;
#[doc = "CM7 1 NMI control"]
pub mod cm7_1_nmi_ctl;
#[doc = "CM7_2_STATUS (r) register accessor: an alias for `Reg<CM7_2_STATUS_SPEC>`"]
pub type CM7_2_STATUS = crate::Reg<cm7_2_status::CM7_2_STATUS_SPEC>;
#[doc = "CM7 2 status"]
pub mod cm7_2_status;
#[doc = "FAST_2_CLOCK_CTL (rw) register accessor: an alias for `Reg<FAST_2_CLOCK_CTL_SPEC>`"]
pub type FAST_2_CLOCK_CTL = crate::Reg<fast_2_clock_ctl::FAST_2_CLOCK_CTL_SPEC>;
#[doc = "Fast 2 clock control"]
pub mod fast_2_clock_ctl;
#[doc = "CM7_2_CTL (rw) register accessor: an alias for `Reg<CM7_2_CTL_SPEC>`"]
pub type CM7_2_CTL = crate::Reg<cm7_2_ctl::CM7_2_CTL_SPEC>;
#[doc = "CM7 2 control"]
pub mod cm7_2_ctl;
#[doc = "CM7_2_INT_STATUS (r) register accessor: an alias for `Reg<CM7_2_INT_STATUS_SPEC>`"]
pub type CM7_2_INT_STATUS = crate::Reg<cm7_2_int_status::CM7_2_INT_STATUS_SPEC>;
#[doc = "CM7 2 interrupt status"]
pub mod cm7_2_int_status;
#[doc = "CM7_2_VECTOR_TABLE_BASE (rw) register accessor: an alias for `Reg<CM7_2_VECTOR_TABLE_BASE_SPEC>`"]
pub type CM7_2_VECTOR_TABLE_BASE =
    crate::Reg<cm7_2_vector_table_base::CM7_2_VECTOR_TABLE_BASE_SPEC>;
#[doc = "CM7 2 vector table base"]
pub mod cm7_2_vector_table_base;
#[doc = "CM7_2_NMI_CTL (rw) register accessor: an alias for `Reg<CM7_2_NMI_CTL_SPEC>`"]
pub type CM7_2_NMI_CTL = crate::Reg<cm7_2_nmi_ctl::CM7_2_NMI_CTL_SPEC>;
#[doc = "CM7 2 NMI control"]
pub mod cm7_2_nmi_ctl;
#[doc = "CM7_3_STATUS (r) register accessor: an alias for `Reg<CM7_3_STATUS_SPEC>`"]
pub type CM7_3_STATUS = crate::Reg<cm7_3_status::CM7_3_STATUS_SPEC>;
#[doc = "CM7 3 status"]
pub mod cm7_3_status;
#[doc = "FAST_3_CLOCK_CTL (rw) register accessor: an alias for `Reg<FAST_3_CLOCK_CTL_SPEC>`"]
pub type FAST_3_CLOCK_CTL = crate::Reg<fast_3_clock_ctl::FAST_3_CLOCK_CTL_SPEC>;
#[doc = "Fast 3 clock control"]
pub mod fast_3_clock_ctl;
#[doc = "CM7_3_CTL (rw) register accessor: an alias for `Reg<CM7_3_CTL_SPEC>`"]
pub type CM7_3_CTL = crate::Reg<cm7_3_ctl::CM7_3_CTL_SPEC>;
#[doc = "CM7 3 control"]
pub mod cm7_3_ctl;
#[doc = "CM7_3_INT_STATUS (r) register accessor: an alias for `Reg<CM7_3_INT_STATUS_SPEC>`"]
pub type CM7_3_INT_STATUS = crate::Reg<cm7_3_int_status::CM7_3_INT_STATUS_SPEC>;
#[doc = "CM7 3 interrupt status"]
pub mod cm7_3_int_status;
#[doc = "CM7_3_VECTOR_TABLE_BASE (rw) register accessor: an alias for `Reg<CM7_3_VECTOR_TABLE_BASE_SPEC>`"]
pub type CM7_3_VECTOR_TABLE_BASE =
    crate::Reg<cm7_3_vector_table_base::CM7_3_VECTOR_TABLE_BASE_SPEC>;
#[doc = "CM7 3 vector table base"]
pub mod cm7_3_vector_table_base;
#[doc = "CM7_3_NMI_CTL (rw) register accessor: an alias for `Reg<CM7_3_NMI_CTL_SPEC>`"]
pub type CM7_3_NMI_CTL = crate::Reg<cm7_3_nmi_ctl::CM7_3_NMI_CTL_SPEC>;
#[doc = "CM7 3 NMI control"]
pub mod cm7_3_nmi_ctl;
#[doc = "CM0_CTL (rw) register accessor: an alias for `Reg<CM0_CTL_SPEC>`"]
pub type CM0_CTL = crate::Reg<cm0_ctl::CM0_CTL_SPEC>;
#[doc = "CM0+ control"]
pub mod cm0_ctl;
#[doc = "CM0_STATUS (r) register accessor: an alias for `Reg<CM0_STATUS_SPEC>`"]
pub type CM0_STATUS = crate::Reg<cm0_status::CM0_STATUS_SPEC>;
#[doc = "CM0+ status"]
pub mod cm0_status;
#[doc = "SLOW_CLOCK_CTL (rw) register accessor: an alias for `Reg<SLOW_CLOCK_CTL_SPEC>`"]
pub type SLOW_CLOCK_CTL = crate::Reg<slow_clock_ctl::SLOW_CLOCK_CTL_SPEC>;
#[doc = "Slow clock control"]
pub mod slow_clock_ctl;
#[doc = "PERI_CLOCK_CTL (rw) register accessor: an alias for `Reg<PERI_CLOCK_CTL_SPEC>`"]
pub type PERI_CLOCK_CTL = crate::Reg<peri_clock_ctl::PERI_CLOCK_CTL_SPEC>;
#[doc = "Peripheral interconnect clock control"]
pub mod peri_clock_ctl;
#[doc = "MEM_CLOCK_CTL (rw) register accessor: an alias for `Reg<MEM_CLOCK_CTL_SPEC>`"]
pub type MEM_CLOCK_CTL = crate::Reg<mem_clock_ctl::MEM_CLOCK_CTL_SPEC>;
#[doc = "Memory clock control"]
pub mod mem_clock_ctl;
#[doc = "CM0_INT0_STATUS (r) register accessor: an alias for `Reg<CM0_INT0_STATUS_SPEC>`"]
pub type CM0_INT0_STATUS = crate::Reg<cm0_int0_status::CM0_INT0_STATUS_SPEC>;
#[doc = "CM0+ interrupt 0 status"]
pub mod cm0_int0_status;
#[doc = "CM0_INT1_STATUS (r) register accessor: an alias for `Reg<CM0_INT1_STATUS_SPEC>`"]
pub type CM0_INT1_STATUS = crate::Reg<cm0_int1_status::CM0_INT1_STATUS_SPEC>;
#[doc = "CM0+ interrupt 1 status"]
pub mod cm0_int1_status;
#[doc = "CM0_INT2_STATUS (r) register accessor: an alias for `Reg<CM0_INT2_STATUS_SPEC>`"]
pub type CM0_INT2_STATUS = crate::Reg<cm0_int2_status::CM0_INT2_STATUS_SPEC>;
#[doc = "CM0+ interrupt 2 status"]
pub mod cm0_int2_status;
#[doc = "CM0_INT3_STATUS (r) register accessor: an alias for `Reg<CM0_INT3_STATUS_SPEC>`"]
pub type CM0_INT3_STATUS = crate::Reg<cm0_int3_status::CM0_INT3_STATUS_SPEC>;
#[doc = "CM0+ interrupt 3 status"]
pub mod cm0_int3_status;
#[doc = "CM0_INT4_STATUS (r) register accessor: an alias for `Reg<CM0_INT4_STATUS_SPEC>`"]
pub type CM0_INT4_STATUS = crate::Reg<cm0_int4_status::CM0_INT4_STATUS_SPEC>;
#[doc = "CM0+ interrupt 4 status"]
pub mod cm0_int4_status;
#[doc = "CM0_INT5_STATUS (r) register accessor: an alias for `Reg<CM0_INT5_STATUS_SPEC>`"]
pub type CM0_INT5_STATUS = crate::Reg<cm0_int5_status::CM0_INT5_STATUS_SPEC>;
#[doc = "CM0+ interrupt 5 status"]
pub mod cm0_int5_status;
#[doc = "CM0_INT6_STATUS (r) register accessor: an alias for `Reg<CM0_INT6_STATUS_SPEC>`"]
pub type CM0_INT6_STATUS = crate::Reg<cm0_int6_status::CM0_INT6_STATUS_SPEC>;
#[doc = "CM0+ interrupt 6 status"]
pub mod cm0_int6_status;
#[doc = "CM0_INT7_STATUS (r) register accessor: an alias for `Reg<CM0_INT7_STATUS_SPEC>`"]
pub type CM0_INT7_STATUS = crate::Reg<cm0_int7_status::CM0_INT7_STATUS_SPEC>;
#[doc = "CM0+ interrupt 7 status"]
pub mod cm0_int7_status;
#[doc = "CM0_VECTOR_TABLE_BASE (rw) register accessor: an alias for `Reg<CM0_VECTOR_TABLE_BASE_SPEC>`"]
pub type CM0_VECTOR_TABLE_BASE = crate::Reg<cm0_vector_table_base::CM0_VECTOR_TABLE_BASE_SPEC>;
#[doc = "CM0+ vector table base"]
pub mod cm0_vector_table_base;
#[doc = "CM0_NMI_CTL (rw) register accessor: an alias for `Reg<CM0_NMI_CTL_SPEC>`"]
pub type CM0_NMI_CTL = crate::Reg<cm0_nmi_ctl::CM0_NMI_CTL_SPEC>;
#[doc = "CM0+ NMI control"]
pub mod cm0_nmi_ctl;
#[doc = "CM7_0_PWR_CTL (rw) register accessor: an alias for `Reg<CM7_0_PWR_CTL_SPEC>`"]
pub type CM7_0_PWR_CTL = crate::Reg<cm7_0_pwr_ctl::CM7_0_PWR_CTL_SPEC>;
#[doc = "CM7 0 power control"]
pub mod cm7_0_pwr_ctl;
#[doc = "CM7_0_PWR_DELAY_CTL (rw) register accessor: an alias for `Reg<CM7_0_PWR_DELAY_CTL_SPEC>`"]
pub type CM7_0_PWR_DELAY_CTL = crate::Reg<cm7_0_pwr_delay_ctl::CM7_0_PWR_DELAY_CTL_SPEC>;
#[doc = "CM7 0 power delay control"]
pub mod cm7_0_pwr_delay_ctl;
#[doc = "CM7_1_PWR_CTL (rw) register accessor: an alias for `Reg<CM7_1_PWR_CTL_SPEC>`"]
pub type CM7_1_PWR_CTL = crate::Reg<cm7_1_pwr_ctl::CM7_1_PWR_CTL_SPEC>;
#[doc = "CM7 1 power control"]
pub mod cm7_1_pwr_ctl;
#[doc = "CM7_1_PWR_DELAY_CTL (rw) register accessor: an alias for `Reg<CM7_1_PWR_DELAY_CTL_SPEC>`"]
pub type CM7_1_PWR_DELAY_CTL = crate::Reg<cm7_1_pwr_delay_ctl::CM7_1_PWR_DELAY_CTL_SPEC>;
#[doc = "CM7 1 power delay control"]
pub mod cm7_1_pwr_delay_ctl;
#[doc = "CM7_2_PWR_CTL (rw) register accessor: an alias for `Reg<CM7_2_PWR_CTL_SPEC>`"]
pub type CM7_2_PWR_CTL = crate::Reg<cm7_2_pwr_ctl::CM7_2_PWR_CTL_SPEC>;
#[doc = "CM7 2 power control"]
pub mod cm7_2_pwr_ctl;
#[doc = "CM7_2_PWR_DELAY_CTL (rw) register accessor: an alias for `Reg<CM7_2_PWR_DELAY_CTL_SPEC>`"]
pub type CM7_2_PWR_DELAY_CTL = crate::Reg<cm7_2_pwr_delay_ctl::CM7_2_PWR_DELAY_CTL_SPEC>;
#[doc = "CM7 2 power delay control"]
pub mod cm7_2_pwr_delay_ctl;
#[doc = "CM7_3_PWR_CTL (rw) register accessor: an alias for `Reg<CM7_3_PWR_CTL_SPEC>`"]
pub type CM7_3_PWR_CTL = crate::Reg<cm7_3_pwr_ctl::CM7_3_PWR_CTL_SPEC>;
#[doc = "CM7 3 power control"]
pub mod cm7_3_pwr_ctl;
#[doc = "CM7_3_PWR_DELAY_CTL (rw) register accessor: an alias for `Reg<CM7_3_PWR_DELAY_CTL_SPEC>`"]
pub type CM7_3_PWR_DELAY_CTL = crate::Reg<cm7_3_pwr_delay_ctl::CM7_3_PWR_DELAY_CTL_SPEC>;
#[doc = "CM7 3 power delay control"]
pub mod cm7_3_pwr_delay_ctl;
#[doc = "RAM0_CTL0 (rw) register accessor: an alias for `Reg<RAM0_CTL0_SPEC>`"]
pub type RAM0_CTL0 = crate::Reg<ram0_ctl0::RAM0_CTL0_SPEC>;
#[doc = "RAM 0 control"]
pub mod ram0_ctl0;
#[doc = "RAM0_STATUS (r) register accessor: an alias for `Reg<RAM0_STATUS_SPEC>`"]
pub type RAM0_STATUS = crate::Reg<ram0_status::RAM0_STATUS_SPEC>;
#[doc = "RAM 0 status"]
pub mod ram0_status;
#[doc = "RAM0_PWR_MACRO_CTL (rw) register accessor: an alias for `Reg<RAM0_PWR_MACRO_CTL_SPEC>`"]
pub type RAM0_PWR_MACRO_CTL = crate::Reg<ram0_pwr_macro_ctl::RAM0_PWR_MACRO_CTL_SPEC>;
#[doc = "RAM 0 power control"]
pub mod ram0_pwr_macro_ctl;
#[doc = "RAM1_CTL0 (rw) register accessor: an alias for `Reg<RAM1_CTL0_SPEC>`"]
pub type RAM1_CTL0 = crate::Reg<ram1_ctl0::RAM1_CTL0_SPEC>;
#[doc = "RAM 1 control"]
pub mod ram1_ctl0;
#[doc = "RAM1_STATUS (r) register accessor: an alias for `Reg<RAM1_STATUS_SPEC>`"]
pub type RAM1_STATUS = crate::Reg<ram1_status::RAM1_STATUS_SPEC>;
#[doc = "RAM 1 status"]
pub mod ram1_status;
#[doc = "RAM1_PWR_CTL (rw) register accessor: an alias for `Reg<RAM1_PWR_CTL_SPEC>`"]
pub type RAM1_PWR_CTL = crate::Reg<ram1_pwr_ctl::RAM1_PWR_CTL_SPEC>;
#[doc = "RAM 1 power control"]
pub mod ram1_pwr_ctl;
#[doc = "RAM2_CTL0 (rw) register accessor: an alias for `Reg<RAM2_CTL0_SPEC>`"]
pub type RAM2_CTL0 = crate::Reg<ram2_ctl0::RAM2_CTL0_SPEC>;
#[doc = "RAM 2 control"]
pub mod ram2_ctl0;
#[doc = "RAM2_STATUS (r) register accessor: an alias for `Reg<RAM2_STATUS_SPEC>`"]
pub type RAM2_STATUS = crate::Reg<ram2_status::RAM2_STATUS_SPEC>;
#[doc = "RAM 2 status"]
pub mod ram2_status;
#[doc = "RAM2_PWR_CTL (rw) register accessor: an alias for `Reg<RAM2_PWR_CTL_SPEC>`"]
pub type RAM2_PWR_CTL = crate::Reg<ram2_pwr_ctl::RAM2_PWR_CTL_SPEC>;
#[doc = "RAM 2 power control"]
pub mod ram2_pwr_ctl;
#[doc = "RAM_PWR_DELAY_CTL (rw) register accessor: an alias for `Reg<RAM_PWR_DELAY_CTL_SPEC>`"]
pub type RAM_PWR_DELAY_CTL = crate::Reg<ram_pwr_delay_ctl::RAM_PWR_DELAY_CTL_SPEC>;
#[doc = "Power up delay used for all SRAM power domains"]
pub mod ram_pwr_delay_ctl;
#[doc = "ROM_CTL (rw) register accessor: an alias for `Reg<ROM_CTL_SPEC>`"]
pub type ROM_CTL = crate::Reg<rom_ctl::ROM_CTL_SPEC>;
#[doc = "ROM control"]
pub mod rom_ctl;
#[doc = "ECC_CTL (rw) register accessor: an alias for `Reg<ECC_CTL_SPEC>`"]
pub type ECC_CTL = crate::Reg<ecc_ctl::ECC_CTL_SPEC>;
#[doc = "ECC control"]
pub mod ecc_ctl;
#[doc = "PRODUCT_ID (r) register accessor: an alias for `Reg<PRODUCT_ID_SPEC>`"]
pub type PRODUCT_ID = crate::Reg<product_id::PRODUCT_ID_SPEC>;
#[doc = "Product identifier and version (same as CoreSight RomTables)"]
pub mod product_id;
#[doc = "DP_STATUS (r) register accessor: an alias for `Reg<DP_STATUS_SPEC>`"]
pub type DP_STATUS = crate::Reg<dp_status::DP_STATUS_SPEC>;
#[doc = "Debug port status"]
pub mod dp_status;
#[doc = "AP_CTL (rw) register accessor: an alias for `Reg<AP_CTL_SPEC>`"]
pub type AP_CTL = crate::Reg<ap_ctl::AP_CTL_SPEC>;
#[doc = "Access port control"]
pub mod ap_ctl;
#[doc = "BUFF_CTL (rw) register accessor: an alias for `Reg<BUFF_CTL_SPEC>`"]
pub type BUFF_CTL = crate::Reg<buff_ctl::BUFF_CTL_SPEC>;
#[doc = "Buffer control"]
pub mod buff_ctl;
#[doc = "SYSTICK_CTL (rw) register accessor: an alias for `Reg<SYSTICK_CTL_SPEC>`"]
pub type SYSTICK_CTL = crate::Reg<systick_ctl::SYSTICK_CTL_SPEC>;
#[doc = "SysTick timer control"]
pub mod systick_ctl;
#[doc = "MBIST_STAT (r) register accessor: an alias for `Reg<MBIST_STAT_SPEC>`"]
pub type MBIST_STAT = crate::Reg<mbist_stat::MBIST_STAT_SPEC>;
#[doc = "Memory BIST status"]
pub mod mbist_stat;
#[doc = "CAL_SUP_SET (rw) register accessor: an alias for `Reg<CAL_SUP_SET_SPEC>`"]
pub type CAL_SUP_SET = crate::Reg<cal_sup_set::CAL_SUP_SET_SPEC>;
#[doc = "Calibration support set and read"]
pub mod cal_sup_set;
#[doc = "CAL_SUP_CLR (rw) register accessor: an alias for `Reg<CAL_SUP_CLR_SPEC>`"]
pub type CAL_SUP_CLR = crate::Reg<cal_sup_clr::CAL_SUP_CLR_SPEC>;
#[doc = "Calibration support clear and reset"]
pub mod cal_sup_clr;
#[doc = "CM0_PC_CTL (rw) register accessor: an alias for `Reg<CM0_PC_CTL_SPEC>`"]
pub type CM0_PC_CTL = crate::Reg<cm0_pc_ctl::CM0_PC_CTL_SPEC>;
#[doc = "CM0+ protection context control"]
pub mod cm0_pc_ctl;
#[doc = "CM0_PC0_HANDLER (rw) register accessor: an alias for `Reg<CM0_PC0_HANDLER_SPEC>`"]
pub type CM0_PC0_HANDLER = crate::Reg<cm0_pc0_handler::CM0_PC0_HANDLER_SPEC>;
#[doc = "CM0+ protection context 0 handler"]
pub mod cm0_pc0_handler;
#[doc = "CM0_PC1_HANDLER (rw) register accessor: an alias for `Reg<CM0_PC1_HANDLER_SPEC>`"]
pub type CM0_PC1_HANDLER = crate::Reg<cm0_pc1_handler::CM0_PC1_HANDLER_SPEC>;
#[doc = "CM0+ protection context 1 handler"]
pub mod cm0_pc1_handler;
#[doc = "CM0_PC2_HANDLER (rw) register accessor: an alias for `Reg<CM0_PC2_HANDLER_SPEC>`"]
pub type CM0_PC2_HANDLER = crate::Reg<cm0_pc2_handler::CM0_PC2_HANDLER_SPEC>;
#[doc = "CM0+ protection context 2 handler"]
pub mod cm0_pc2_handler;
#[doc = "CM0_PC3_HANDLER (rw) register accessor: an alias for `Reg<CM0_PC3_HANDLER_SPEC>`"]
pub type CM0_PC3_HANDLER = crate::Reg<cm0_pc3_handler::CM0_PC3_HANDLER_SPEC>;
#[doc = "CM0+ protection context 3 handler"]
pub mod cm0_pc3_handler;
#[doc = "PROTECTION (rw) register accessor: an alias for `Reg<PROTECTION_SPEC>`"]
pub type PROTECTION = crate::Reg<protection::PROTECTION_SPEC>;
#[doc = "Protection status"]
pub mod protection;
#[doc = "TRIM_ROM_CTL (rw) register accessor: an alias for `Reg<TRIM_ROM_CTL_SPEC>`"]
pub type TRIM_ROM_CTL = crate::Reg<trim_rom_ctl::TRIM_ROM_CTL_SPEC>;
#[doc = "ROM trim control"]
pub mod trim_rom_ctl;
#[doc = "TRIM_RAM_CTL (rw) register accessor: an alias for `Reg<TRIM_RAM_CTL_SPEC>`"]
pub type TRIM_RAM_CTL = crate::Reg<trim_ram_ctl::TRIM_RAM_CTL_SPEC>;
#[doc = "RAM trim control for less than 100MHz SRAMs"]
pub mod trim_ram_ctl;
#[doc = "TRIM_RAM200_CTL (rw) register accessor: an alias for `Reg<TRIM_RAM200_CTL_SPEC>`"]
pub type TRIM_RAM200_CTL = crate::Reg<trim_ram200_ctl::TRIM_RAM200_CTL_SPEC>;
#[doc = "RAM trim control for 100MHz - 200MHz SRAMs"]
pub mod trim_ram200_ctl;
#[doc = "TRIM_RAM350_CTL (rw) register accessor: an alias for `Reg<TRIM_RAM350_CTL_SPEC>`"]
pub type TRIM_RAM350_CTL = crate::Reg<trim_ram350_ctl::TRIM_RAM350_CTL_SPEC>;
#[doc = "RAM trim control for more than 200MHz SRAMs"]
pub mod trim_ram350_ctl;
#[doc = "CM0_SYSTEM_INT_CTL (rw) register accessor: an alias for `Reg<CM0_SYSTEM_INT_CTL_SPEC>`"]
pub type CM0_SYSTEM_INT_CTL = crate::Reg<cm0_system_int_ctl::CM0_SYSTEM_INT_CTL_SPEC>;
#[doc = "CM0+ system interrupt control"]
pub mod cm0_system_int_ctl;
#[doc = "CM7_0_SYSTEM_INT_CTL (rw) register accessor: an alias for `Reg<CM7_0_SYSTEM_INT_CTL_SPEC>`"]
pub type CM7_0_SYSTEM_INT_CTL = crate::Reg<cm7_0_system_int_ctl::CM7_0_SYSTEM_INT_CTL_SPEC>;
#[doc = "CM7 0 system interrupt control"]
pub mod cm7_0_system_int_ctl;
#[doc = "CM7_1_SYSTEM_INT_CTL (rw) register accessor: an alias for `Reg<CM7_1_SYSTEM_INT_CTL_SPEC>`"]
pub type CM7_1_SYSTEM_INT_CTL = crate::Reg<cm7_1_system_int_ctl::CM7_1_SYSTEM_INT_CTL_SPEC>;
#[doc = "CM7 1 system interrupt control"]
pub mod cm7_1_system_int_ctl;
#[doc = "CM7_2_SYSTEM_INT_CTL (rw) register accessor: an alias for `Reg<CM7_2_SYSTEM_INT_CTL_SPEC>`"]
pub type CM7_2_SYSTEM_INT_CTL = crate::Reg<cm7_2_system_int_ctl::CM7_2_SYSTEM_INT_CTL_SPEC>;
#[doc = "CM7 2 system interrupt control"]
pub mod cm7_2_system_int_ctl;
#[doc = "CM7_3_SYSTEM_INT_CTL (rw) register accessor: an alias for `Reg<CM7_3_SYSTEM_INT_CTL_SPEC>`"]
pub type CM7_3_SYSTEM_INT_CTL = crate::Reg<cm7_3_system_int_ctl::CM7_3_SYSTEM_INT_CTL_SPEC>;
#[doc = "CM7 3 system interrupt control"]
pub mod cm7_3_system_int_ctl;
