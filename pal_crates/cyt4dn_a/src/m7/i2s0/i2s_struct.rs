#[doc = r"Register block"]
#[repr(C)]
pub struct I2S_STRUCT {
    #[doc = "0x00..0xd0 - I2S TX structure"]
    pub i2s_tx_struct: I2S_TX_STRUCT,
    _reserved1: [u8; 0x30],
    #[doc = "0x100..0x1d0 - I2S RX structure"]
    pub i2s_rx_struct: I2S_RX_STRUCT,
}
#[doc = "I2S TX structure"]
pub use self::i2s_tx_struct::I2S_TX_STRUCT;
#[doc = r"Cluster"]
#[doc = "I2S TX structure"]
pub mod i2s_tx_struct;
#[doc = "I2S RX structure"]
pub use self::i2s_rx_struct::I2S_RX_STRUCT;
#[doc = r"Cluster"]
#[doc = "I2S RX structure"]
pub mod i2s_rx_struct;
