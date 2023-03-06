#[doc = r"Register block"]
#[repr(C)]
pub struct TDM_STRUCT {
    #[doc = "0x00..0xd0 - TDM TX structure"]
    pub tdm_tx_struct: TDM_TX_STRUCT,
    _reserved1: [u8; 0x30],
    #[doc = "0x100..0x1d0 - TDM RX structure"]
    pub tdm_rx_struct: TDM_RX_STRUCT,
}
#[doc = "TDM TX structure"]
pub use self::tdm_tx_struct::TDM_TX_STRUCT;
#[doc = r"Cluster"]
#[doc = "TDM TX structure"]
pub mod tdm_tx_struct;
#[doc = "TDM RX structure"]
pub use self::tdm_rx_struct::TDM_RX_STRUCT;
#[doc = r"Cluster"]
#[doc = "TDM RX structure"]
pub mod tdm_rx_struct;
