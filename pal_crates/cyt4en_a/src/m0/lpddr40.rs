#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00..0x4c - HOBTO specific wrapper"]
    pub lpddr4_wrapper: LPDDR4_WRAPPER,
    _reserved1: [u8; 0x03b4],
    #[doc = "0x400..0x670 - AXI performance counter"]
    pub axi_perf_cnt: AXI_PERF_CNT,
    _reserved2: [u8; 0x0990],
    #[doc = "0x1000..0x1eb8 - LPDDR4 core (3PIP)"]
    pub lpddr4_core: LPDDR4_CORE,
    _reserved3: [u8; 0x2148],
    #[doc = "0x4000..0x61e8 - EMPU configuration"]
    pub empu: EMPU,
}
#[doc = "HOBTO specific wrapper"]
pub use self::lpddr4_wrapper::LPDDR4_WRAPPER;
#[doc = r"Cluster"]
#[doc = "HOBTO specific wrapper"]
pub mod lpddr4_wrapper;
#[doc = "AXI performance counter"]
pub use self::axi_perf_cnt::AXI_PERF_CNT;
#[doc = r"Cluster"]
#[doc = "AXI performance counter"]
pub mod axi_perf_cnt;
#[doc = "LPDDR4 core (3PIP)"]
pub use self::lpddr4_core::LPDDR4_CORE;
#[doc = r"Cluster"]
#[doc = "LPDDR4 core (3PIP)"]
pub mod lpddr4_core;
#[doc = "EMPU configuration"]
pub use self::empu::EMPU;
#[doc = r"Cluster"]
#[doc = "EMPU configuration"]
pub mod empu;
