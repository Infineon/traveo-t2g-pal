#[doc = "Register `NETWORK_CONFIG` reader"]
pub struct R(crate::R<NETWORK_CONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NETWORK_CONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NETWORK_CONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NETWORK_CONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `NETWORK_CONFIG` writer"]
pub struct W(crate::W<NETWORK_CONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NETWORK_CONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<NETWORK_CONFIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NETWORK_CONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SPEED` reader - Speed - set to logic one to indicate 100Mbps operation, logic zero for 10Mbps."]
pub type SPEED_R = crate::BitReader<bool>;
#[doc = "Field `SPEED` writer - Speed - set to logic one to indicate 100Mbps operation, logic zero for 10Mbps."]
pub type SPEED_W<'a, const O: u8> = crate::BitWriter<'a, u32, NETWORK_CONFIG_SPEC, bool, O>;
#[doc = "Field `FULL_DUPLEX` reader - Full duplex - if set to logic one, the transmit block ignores the state of collision and carrier sense and allows receive while transmitting. Also controls the half_duplex pin."]
pub type FULL_DUPLEX_R = crate::BitReader<bool>;
#[doc = "Field `FULL_DUPLEX` writer - Full duplex - if set to logic one, the transmit block ignores the state of collision and carrier sense and allows receive while transmitting. Also controls the half_duplex pin."]
pub type FULL_DUPLEX_W<'a, const O: u8> = crate::BitWriter<'a, u32, NETWORK_CONFIG_SPEC, bool, O>;
#[doc = "Field `DISCARD_NON_VLAN_FRAMES` reader - Discard non-VLAN frames - when set only VLAN tagged frames will be passed to the address matching logic."]
pub type DISCARD_NON_VLAN_FRAMES_R = crate::BitReader<bool>;
#[doc = "Field `DISCARD_NON_VLAN_FRAMES` writer - Discard non-VLAN frames - when set only VLAN tagged frames will be passed to the address matching logic."]
pub type DISCARD_NON_VLAN_FRAMES_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, NETWORK_CONFIG_SPEC, bool, O>;
#[doc = "Field `JUMBO_FRAMES` reader - Jumbo frames - set to one to enable jumbo frames up to `gem_jumbo_max_length bytes to be accepted. The default length is 10,240 bytes."]
pub type JUMBO_FRAMES_R = crate::BitReader<bool>;
#[doc = "Field `JUMBO_FRAMES` writer - Jumbo frames - set to one to enable jumbo frames up to `gem_jumbo_max_length bytes to be accepted. The default length is 10,240 bytes."]
pub type JUMBO_FRAMES_W<'a, const O: u8> = crate::BitWriter<'a, u32, NETWORK_CONFIG_SPEC, bool, O>;
#[doc = "Field `COPY_ALL_FRAMES` reader - Copy all frames - when set to logic one, all valid frames will be accepted."]
pub type COPY_ALL_FRAMES_R = crate::BitReader<bool>;
#[doc = "Field `COPY_ALL_FRAMES` writer - Copy all frames - when set to logic one, all valid frames will be accepted."]
pub type COPY_ALL_FRAMES_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, NETWORK_CONFIG_SPEC, bool, O>;
#[doc = "Field `NO_BROADCAST` reader - No broadcast - when set to logic one, frames addressed to the broadcast address of all ones will not be accepted."]
pub type NO_BROADCAST_R = crate::BitReader<bool>;
#[doc = "Field `NO_BROADCAST` writer - No broadcast - when set to logic one, frames addressed to the broadcast address of all ones will not be accepted."]
pub type NO_BROADCAST_W<'a, const O: u8> = crate::BitWriter<'a, u32, NETWORK_CONFIG_SPEC, bool, O>;
#[doc = "Field `MULTICAST_HASH_ENABLE` reader - Multicast hash enable - when set, multicast frames will be accepted when the 6 bit hash function of the destination address points to a bit that is set in the hash register."]
pub type MULTICAST_HASH_ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `MULTICAST_HASH_ENABLE` writer - Multicast hash enable - when set, multicast frames will be accepted when the 6 bit hash function of the destination address points to a bit that is set in the hash register."]
pub type MULTICAST_HASH_ENABLE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, NETWORK_CONFIG_SPEC, bool, O>;
#[doc = "Field `UNICAST_HASH_ENABLE` reader - Unicast hash enable - when set, unicast frames will be accepted when the 6 bit hash function of the destination address points to a bit that is set in the hash register."]
pub type UNICAST_HASH_ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `UNICAST_HASH_ENABLE` writer - Unicast hash enable - when set, unicast frames will be accepted when the 6 bit hash function of the destination address points to a bit that is set in the hash register."]
pub type UNICAST_HASH_ENABLE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, NETWORK_CONFIG_SPEC, bool, O>;
#[doc = "Field `RECEIVE_1536_BYTE_FRAMES` reader - Receive 1536 byte frames - setting this bit means the GEM will accept frames up to 1536 bytes in length. Normally the GEM would reject any frame above 1518 bytes."]
pub type RECEIVE_1536_BYTE_FRAMES_R = crate::BitReader<bool>;
#[doc = "Field `RECEIVE_1536_BYTE_FRAMES` writer - Receive 1536 byte frames - setting this bit means the GEM will accept frames up to 1536 bytes in length. Normally the GEM would reject any frame above 1518 bytes."]
pub type RECEIVE_1536_BYTE_FRAMES_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, NETWORK_CONFIG_SPEC, bool, O>;
#[doc = "Field `EXTERNAL_ADDRESS_MATCH_ENABLE` reader - External address match enable - when set the external address match interface can be used to copy frames to memory."]
pub type EXTERNAL_ADDRESS_MATCH_ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `EXTERNAL_ADDRESS_MATCH_ENABLE` writer - External address match enable - when set the external address match interface can be used to copy frames to memory."]
pub type EXTERNAL_ADDRESS_MATCH_ENABLE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, NETWORK_CONFIG_SPEC, bool, O>;
#[doc = "Field `GIGABIT_MODE_ENABLE` reader - Gigabit mode enable - setting this bit configures the GEM for 1000 Mbps operation. 0: 10/100 operation using MII interface 1: Gigabit operation using GMI interface"]
pub type GIGABIT_MODE_ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `GIGABIT_MODE_ENABLE` writer - Gigabit mode enable - setting this bit configures the GEM for 1000 Mbps operation. 0: 10/100 operation using MII interface 1: Gigabit operation using GMI interface"]
pub type GIGABIT_MODE_ENABLE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, NETWORK_CONFIG_SPEC, bool, O>;
#[doc = "Field `PCS_SELECT` reader - PCS select - selects between MII/GMII and TBI"]
pub type PCS_SELECT_R = crate::BitReader<bool>;
#[doc = "Field `PCS_SELECT` writer - PCS select - selects between MII/GMII and TBI"]
pub type PCS_SELECT_W<'a, const O: u8> = crate::BitWriter<'a, u32, NETWORK_CONFIG_SPEC, bool, O>;
#[doc = "Field `RETRY_TEST` reader - Retry test - must be set to zero for normal operation. If set to one the backoff between collisions will always be one slot time. Setting this bit to one helps test the too many retries condition. Also used in the pause frame tests to reduce the pause counter's decrement time from 512 bit times, to every rx_clk cycle."]
pub type RETRY_TEST_R = crate::BitReader<bool>;
#[doc = "Field `RETRY_TEST` writer - Retry test - must be set to zero for normal operation. If set to one the backoff between collisions will always be one slot time. Setting this bit to one helps test the too many retries condition. Also used in the pause frame tests to reduce the pause counter's decrement time from 512 bit times, to every rx_clk cycle."]
pub type RETRY_TEST_W<'a, const O: u8> = crate::BitWriter<'a, u32, NETWORK_CONFIG_SPEC, bool, O>;
#[doc = "Field `PAUSE_ENABLE` reader - Pause enable - when set, transmission will pause if a non zero 802.3 classic pause frame is received and PFC has not been negotiated."]
pub type PAUSE_ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `PAUSE_ENABLE` writer - Pause enable - when set, transmission will pause if a non zero 802.3 classic pause frame is received and PFC has not been negotiated."]
pub type PAUSE_ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, NETWORK_CONFIG_SPEC, bool, O>;
#[doc = "Field `RECEIVE_BUFFER_OFFSET` reader - Receive buffer offset - indicates the number of bytes by which the received data is offset from the start of the receive buffer."]
pub type RECEIVE_BUFFER_OFFSET_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RECEIVE_BUFFER_OFFSET` writer - Receive buffer offset - indicates the number of bytes by which the received data is offset from the start of the receive buffer."]
pub type RECEIVE_BUFFER_OFFSET_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, NETWORK_CONFIG_SPEC, u8, u8, 2, O>;
#[doc = "Field `LENGTH_FIELD_ERROR_FRAME_DISCARD` reader - Length field error frame discard - setting this bit causes frames with a measured length shorter than the extracted length field (as indicated by bytes 13 and 14 in a non-VLAN tagged frame) to be discarded. This only applies to frames with a length field less than 0x0600."]
pub type LENGTH_FIELD_ERROR_FRAME_DISCARD_R = crate::BitReader<bool>;
#[doc = "Field `LENGTH_FIELD_ERROR_FRAME_DISCARD` writer - Length field error frame discard - setting this bit causes frames with a measured length shorter than the extracted length field (as indicated by bytes 13 and 14 in a non-VLAN tagged frame) to be discarded. This only applies to frames with a length field less than 0x0600."]
pub type LENGTH_FIELD_ERROR_FRAME_DISCARD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, NETWORK_CONFIG_SPEC, bool, O>;
#[doc = "Field `FCS_REMOVE` reader - FCS remove - setting this bit will cause received frames to be written to memory without their frame check sequence (last 4 bytes). The frame length indicated will be reduced by four bytes in this mode."]
pub type FCS_REMOVE_R = crate::BitReader<bool>;
#[doc = "Field `FCS_REMOVE` writer - FCS remove - setting this bit will cause received frames to be written to memory without their frame check sequence (last 4 bytes). The frame length indicated will be reduced by four bytes in this mode."]
pub type FCS_REMOVE_W<'a, const O: u8> = crate::BitWriter<'a, u32, NETWORK_CONFIG_SPEC, bool, O>;
#[doc = "Field `MDC_CLOCK_DIVISION` reader - MDC clock division - set according to pclk speed. These three bits determine the number pclk will be divided by to generate MDC. For conformance with the 802.3 specification, MDC must not exceed 2.5 MHz (MDC is only active during MDIO read and write operations). The reset value for this can be changed by defining a new value for gem_mdc_clock_div in gem_defs.v 000: divide pclk by 8 (pclk up to 20 MHz) 001: divide pclk by 16 (pclk up to 40 MHz) 010: divide pclk by 32 (pclk up to 80 MHz) 011: divide pclk by 48 (pclk up to 120MHz) 100: divide pclk by 64 (pclk up to 160 MHz) 101: divide pclk by 96 (pclk up to 240 MHz) 110: divide pclk by 128 (pclk up to 320 MHz) 111: divide pclk by 224 (pclk up to 540 MHz). Note. The reset value of this field is equal to the gem_mdc_clock_div define, which is user configurable."]
pub type MDC_CLOCK_DIVISION_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MDC_CLOCK_DIVISION` writer - MDC clock division - set according to pclk speed. These three bits determine the number pclk will be divided by to generate MDC. For conformance with the 802.3 specification, MDC must not exceed 2.5 MHz (MDC is only active during MDIO read and write operations). The reset value for this can be changed by defining a new value for gem_mdc_clock_div in gem_defs.v 000: divide pclk by 8 (pclk up to 20 MHz) 001: divide pclk by 16 (pclk up to 40 MHz) 010: divide pclk by 32 (pclk up to 80 MHz) 011: divide pclk by 48 (pclk up to 120MHz) 100: divide pclk by 64 (pclk up to 160 MHz) 101: divide pclk by 96 (pclk up to 240 MHz) 110: divide pclk by 128 (pclk up to 320 MHz) 111: divide pclk by 224 (pclk up to 540 MHz). Note. The reset value of this field is equal to the gem_mdc_clock_div define, which is user configurable."]
pub type MDC_CLOCK_DIVISION_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, NETWORK_CONFIG_SPEC, u8, u8, 3, O>;
#[doc = "Field `DATA_BUS_WIDTH` reader - Data bus width - set according to AMBA (AXI/AHB) or external FIFO data bus width. The reset value for this can be changed by defining a new value for gem_dma_bus_width_def in gem_defs. Only valid bus widths may be written if the system is configured to a maximum width less than 128-bits. 00: 32 bit data bus width 01: 64 bit AMBA (AXI/AHB) data bus width 10: 128 bit AMBA (AXI/AHB) data bus width 11: 128 bit AMBA (AXI/AHB) data bus width. Note. The reset value of this field is equal to the gem_dma_bus_width_def define, which is user configurable. Note: For AXI_MASTER_PRESENT=1, MXETH only supports 64b DMA data bus width, so must set it 1. For AXI_MASTER_PRESENT=0, MXETH only supports 32b DMA data bus width, so must set it 0."]
pub type DATA_BUS_WIDTH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DATA_BUS_WIDTH` writer - Data bus width - set according to AMBA (AXI/AHB) or external FIFO data bus width. The reset value for this can be changed by defining a new value for gem_dma_bus_width_def in gem_defs. Only valid bus widths may be written if the system is configured to a maximum width less than 128-bits. 00: 32 bit data bus width 01: 64 bit AMBA (AXI/AHB) data bus width 10: 128 bit AMBA (AXI/AHB) data bus width 11: 128 bit AMBA (AXI/AHB) data bus width. Note. The reset value of this field is equal to the gem_dma_bus_width_def define, which is user configurable. Note: For AXI_MASTER_PRESENT=1, MXETH only supports 64b DMA data bus width, so must set it 1. For AXI_MASTER_PRESENT=0, MXETH only supports 32b DMA data bus width, so must set it 0."]
pub type DATA_BUS_WIDTH_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, NETWORK_CONFIG_SPEC, u8, u8, 2, O>;
#[doc = "Field `DISABLE_COPY_OF_PAUSE_FRAMES` reader - Disable copy of pause frames - set to one to prevent pause frames being copied to memory. When set, neither control frames with type id 8808, nor pause frames with destination address 010000c28001 are copied to memory, this functionality was enhanced in release 1p09 (mxeth uses 1p09). Note that valid pause frames received will still increment pause statistics and pause the transmission of frames as required."]
pub type DISABLE_COPY_OF_PAUSE_FRAMES_R = crate::BitReader<bool>;
#[doc = "Field `DISABLE_COPY_OF_PAUSE_FRAMES` writer - Disable copy of pause frames - set to one to prevent pause frames being copied to memory. When set, neither control frames with type id 8808, nor pause frames with destination address 010000c28001 are copied to memory, this functionality was enhanced in release 1p09 (mxeth uses 1p09). Note that valid pause frames received will still increment pause statistics and pause the transmission of frames as required."]
pub type DISABLE_COPY_OF_PAUSE_FRAMES_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, NETWORK_CONFIG_SPEC, bool, O>;
#[doc = "Field `RECEIVE_CHECKSUM_OFFLOAD_ENABLE` reader - Receive checksum offload enable - when set, the receive checksum engine is enabled. Frames with bad IP, TCP or UDP checksums are discarded."]
pub type RECEIVE_CHECKSUM_OFFLOAD_ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `RECEIVE_CHECKSUM_OFFLOAD_ENABLE` writer - Receive checksum offload enable - when set, the receive checksum engine is enabled. Frames with bad IP, TCP or UDP checksums are discarded."]
pub type RECEIVE_CHECKSUM_OFFLOAD_ENABLE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, NETWORK_CONFIG_SPEC, bool, O>;
#[doc = "Field `EN_HALF_DUPLEX_RX` reader - Enable frames to be received in half-duplex mode while transmitting. Must set '0', MXETH only supports full-duplex mode."]
pub type EN_HALF_DUPLEX_RX_R = crate::BitReader<bool>;
#[doc = "Field `EN_HALF_DUPLEX_RX` writer - Enable frames to be received in half-duplex mode while transmitting. Must set '0', MXETH only supports full-duplex mode."]
pub type EN_HALF_DUPLEX_RX_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, NETWORK_CONFIG_SPEC, bool, O>;
#[doc = "Field `IGNORE_RX_FCS` reader - Ignore RX FCS - when set frames with FCS/CRC errors will not be rejected. FCS error statistics will still be collected for frames with bad FCS and FCS status will be recorded in frame's DMA descriptor. For normal operation this bit must be set to zero."]
pub type IGNORE_RX_FCS_R = crate::BitReader<bool>;
#[doc = "Field `IGNORE_RX_FCS` writer - Ignore RX FCS - when set frames with FCS/CRC errors will not be rejected. FCS error statistics will still be collected for frames with bad FCS and FCS status will be recorded in frame's DMA descriptor. For normal operation this bit must be set to zero."]
pub type IGNORE_RX_FCS_W<'a, const O: u8> = crate::BitWriter<'a, u32, NETWORK_CONFIG_SPEC, bool, O>;
#[doc = "Field `SGMII_MODE_ENABLE` reader - SGMII mode enable - changes behaviour of the auto-negotiation advertisement and link partner ability registers to meet the requirements of SGMII and reduces the duration of the link timer from 10 ms to 1.6 ms."]
pub type SGMII_MODE_ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `SGMII_MODE_ENABLE` writer - SGMII mode enable - changes behaviour of the auto-negotiation advertisement and link partner ability registers to meet the requirements of SGMII and reduces the duration of the link timer from 10 ms to 1.6 ms."]
pub type SGMII_MODE_ENABLE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, NETWORK_CONFIG_SPEC, bool, O>;
#[doc = "Field `IPG_STRETCH_ENABLE` reader - IPG stretch enable - when set the transmit IPG can be increased above 96 bit times depending on the previous frame length using the IPG stretch register."]
pub type IPG_STRETCH_ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `IPG_STRETCH_ENABLE` writer - IPG stretch enable - when set the transmit IPG can be increased above 96 bit times depending on the previous frame length using the IPG stretch register."]
pub type IPG_STRETCH_ENABLE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, NETWORK_CONFIG_SPEC, bool, O>;
#[doc = "Field `NSP_CHANGE` reader - Receive bad preamble. When set frames with non-standard preamble are not rejected."]
pub type NSP_CHANGE_R = crate::BitReader<bool>;
#[doc = "Field `NSP_CHANGE` writer - Receive bad preamble. When set frames with non-standard preamble are not rejected."]
pub type NSP_CHANGE_W<'a, const O: u8> = crate::BitWriter<'a, u32, NETWORK_CONFIG_SPEC, bool, O>;
#[doc = "Field `IGNORE_IPG_RX_ER` reader - Ignore IPG rx_er. When set rx_er has no effect on the GEMs operation when rx_dv is low. Set this when using the RGMII wrapper in half-duplex mode."]
pub type IGNORE_IPG_RX_ER_R = crate::BitReader<bool>;
#[doc = "Field `IGNORE_IPG_RX_ER` writer - Ignore IPG rx_er. When set rx_er has no effect on the GEMs operation when rx_dv is low. Set this when using the RGMII wrapper in half-duplex mode."]
pub type IGNORE_IPG_RX_ER_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, NETWORK_CONFIG_SPEC, bool, O>;
#[doc = "Field `RSVD_31` reader - N/A"]
pub type RSVD_31_R = crate::BitReader<bool>;
#[doc = "Field `RSVD_31` writer - N/A"]
pub type RSVD_31_W<'a, const O: u8> = crate::BitWriter<'a, u32, NETWORK_CONFIG_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Speed - set to logic one to indicate 100Mbps operation, logic zero for 10Mbps."]
    #[inline(always)]
    pub fn speed(&self) -> SPEED_R {
        SPEED_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Full duplex - if set to logic one, the transmit block ignores the state of collision and carrier sense and allows receive while transmitting. Also controls the half_duplex pin."]
    #[inline(always)]
    pub fn full_duplex(&self) -> FULL_DUPLEX_R {
        FULL_DUPLEX_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Discard non-VLAN frames - when set only VLAN tagged frames will be passed to the address matching logic."]
    #[inline(always)]
    pub fn discard_non_vlan_frames(&self) -> DISCARD_NON_VLAN_FRAMES_R {
        DISCARD_NON_VLAN_FRAMES_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Jumbo frames - set to one to enable jumbo frames up to `gem_jumbo_max_length bytes to be accepted. The default length is 10,240 bytes."]
    #[inline(always)]
    pub fn jumbo_frames(&self) -> JUMBO_FRAMES_R {
        JUMBO_FRAMES_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Copy all frames - when set to logic one, all valid frames will be accepted."]
    #[inline(always)]
    pub fn copy_all_frames(&self) -> COPY_ALL_FRAMES_R {
        COPY_ALL_FRAMES_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - No broadcast - when set to logic one, frames addressed to the broadcast address of all ones will not be accepted."]
    #[inline(always)]
    pub fn no_broadcast(&self) -> NO_BROADCAST_R {
        NO_BROADCAST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Multicast hash enable - when set, multicast frames will be accepted when the 6 bit hash function of the destination address points to a bit that is set in the hash register."]
    #[inline(always)]
    pub fn multicast_hash_enable(&self) -> MULTICAST_HASH_ENABLE_R {
        MULTICAST_HASH_ENABLE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Unicast hash enable - when set, unicast frames will be accepted when the 6 bit hash function of the destination address points to a bit that is set in the hash register."]
    #[inline(always)]
    pub fn unicast_hash_enable(&self) -> UNICAST_HASH_ENABLE_R {
        UNICAST_HASH_ENABLE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Receive 1536 byte frames - setting this bit means the GEM will accept frames up to 1536 bytes in length. Normally the GEM would reject any frame above 1518 bytes."]
    #[inline(always)]
    pub fn receive_1536_byte_frames(&self) -> RECEIVE_1536_BYTE_FRAMES_R {
        RECEIVE_1536_BYTE_FRAMES_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - External address match enable - when set the external address match interface can be used to copy frames to memory."]
    #[inline(always)]
    pub fn external_address_match_enable(&self) -> EXTERNAL_ADDRESS_MATCH_ENABLE_R {
        EXTERNAL_ADDRESS_MATCH_ENABLE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Gigabit mode enable - setting this bit configures the GEM for 1000 Mbps operation. 0: 10/100 operation using MII interface 1: Gigabit operation using GMI interface"]
    #[inline(always)]
    pub fn gigabit_mode_enable(&self) -> GIGABIT_MODE_ENABLE_R {
        GIGABIT_MODE_ENABLE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PCS select - selects between MII/GMII and TBI"]
    #[inline(always)]
    pub fn pcs_select(&self) -> PCS_SELECT_R {
        PCS_SELECT_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Retry test - must be set to zero for normal operation. If set to one the backoff between collisions will always be one slot time. Setting this bit to one helps test the too many retries condition. Also used in the pause frame tests to reduce the pause counter's decrement time from 512 bit times, to every rx_clk cycle."]
    #[inline(always)]
    pub fn retry_test(&self) -> RETRY_TEST_R {
        RETRY_TEST_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Pause enable - when set, transmission will pause if a non zero 802.3 classic pause frame is received and PFC has not been negotiated."]
    #[inline(always)]
    pub fn pause_enable(&self) -> PAUSE_ENABLE_R {
        PAUSE_ENABLE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:15 - Receive buffer offset - indicates the number of bytes by which the received data is offset from the start of the receive buffer."]
    #[inline(always)]
    pub fn receive_buffer_offset(&self) -> RECEIVE_BUFFER_OFFSET_R {
        RECEIVE_BUFFER_OFFSET_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bit 16 - Length field error frame discard - setting this bit causes frames with a measured length shorter than the extracted length field (as indicated by bytes 13 and 14 in a non-VLAN tagged frame) to be discarded. This only applies to frames with a length field less than 0x0600."]
    #[inline(always)]
    pub fn length_field_error_frame_discard(&self) -> LENGTH_FIELD_ERROR_FRAME_DISCARD_R {
        LENGTH_FIELD_ERROR_FRAME_DISCARD_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - FCS remove - setting this bit will cause received frames to be written to memory without their frame check sequence (last 4 bytes). The frame length indicated will be reduced by four bytes in this mode."]
    #[inline(always)]
    pub fn fcs_remove(&self) -> FCS_REMOVE_R {
        FCS_REMOVE_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:20 - MDC clock division - set according to pclk speed. These three bits determine the number pclk will be divided by to generate MDC. For conformance with the 802.3 specification, MDC must not exceed 2.5 MHz (MDC is only active during MDIO read and write operations). The reset value for this can be changed by defining a new value for gem_mdc_clock_div in gem_defs.v 000: divide pclk by 8 (pclk up to 20 MHz) 001: divide pclk by 16 (pclk up to 40 MHz) 010: divide pclk by 32 (pclk up to 80 MHz) 011: divide pclk by 48 (pclk up to 120MHz) 100: divide pclk by 64 (pclk up to 160 MHz) 101: divide pclk by 96 (pclk up to 240 MHz) 110: divide pclk by 128 (pclk up to 320 MHz) 111: divide pclk by 224 (pclk up to 540 MHz). Note. The reset value of this field is equal to the gem_mdc_clock_div define, which is user configurable."]
    #[inline(always)]
    pub fn mdc_clock_division(&self) -> MDC_CLOCK_DIVISION_R {
        MDC_CLOCK_DIVISION_R::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bits 21:22 - Data bus width - set according to AMBA (AXI/AHB) or external FIFO data bus width. The reset value for this can be changed by defining a new value for gem_dma_bus_width_def in gem_defs. Only valid bus widths may be written if the system is configured to a maximum width less than 128-bits. 00: 32 bit data bus width 01: 64 bit AMBA (AXI/AHB) data bus width 10: 128 bit AMBA (AXI/AHB) data bus width 11: 128 bit AMBA (AXI/AHB) data bus width. Note. The reset value of this field is equal to the gem_dma_bus_width_def define, which is user configurable. Note: For AXI_MASTER_PRESENT=1, MXETH only supports 64b DMA data bus width, so must set it 1. For AXI_MASTER_PRESENT=0, MXETH only supports 32b DMA data bus width, so must set it 0."]
    #[inline(always)]
    pub fn data_bus_width(&self) -> DATA_BUS_WIDTH_R {
        DATA_BUS_WIDTH_R::new(((self.bits >> 21) & 3) as u8)
    }
    #[doc = "Bit 23 - Disable copy of pause frames - set to one to prevent pause frames being copied to memory. When set, neither control frames with type id 8808, nor pause frames with destination address 010000c28001 are copied to memory, this functionality was enhanced in release 1p09 (mxeth uses 1p09). Note that valid pause frames received will still increment pause statistics and pause the transmission of frames as required."]
    #[inline(always)]
    pub fn disable_copy_of_pause_frames(&self) -> DISABLE_COPY_OF_PAUSE_FRAMES_R {
        DISABLE_COPY_OF_PAUSE_FRAMES_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Receive checksum offload enable - when set, the receive checksum engine is enabled. Frames with bad IP, TCP or UDP checksums are discarded."]
    #[inline(always)]
    pub fn receive_checksum_offload_enable(&self) -> RECEIVE_CHECKSUM_OFFLOAD_ENABLE_R {
        RECEIVE_CHECKSUM_OFFLOAD_ENABLE_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Enable frames to be received in half-duplex mode while transmitting. Must set '0', MXETH only supports full-duplex mode."]
    #[inline(always)]
    pub fn en_half_duplex_rx(&self) -> EN_HALF_DUPLEX_RX_R {
        EN_HALF_DUPLEX_RX_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Ignore RX FCS - when set frames with FCS/CRC errors will not be rejected. FCS error statistics will still be collected for frames with bad FCS and FCS status will be recorded in frame's DMA descriptor. For normal operation this bit must be set to zero."]
    #[inline(always)]
    pub fn ignore_rx_fcs(&self) -> IGNORE_RX_FCS_R {
        IGNORE_RX_FCS_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - SGMII mode enable - changes behaviour of the auto-negotiation advertisement and link partner ability registers to meet the requirements of SGMII and reduces the duration of the link timer from 10 ms to 1.6 ms."]
    #[inline(always)]
    pub fn sgmii_mode_enable(&self) -> SGMII_MODE_ENABLE_R {
        SGMII_MODE_ENABLE_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - IPG stretch enable - when set the transmit IPG can be increased above 96 bit times depending on the previous frame length using the IPG stretch register."]
    #[inline(always)]
    pub fn ipg_stretch_enable(&self) -> IPG_STRETCH_ENABLE_R {
        IPG_STRETCH_ENABLE_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Receive bad preamble. When set frames with non-standard preamble are not rejected."]
    #[inline(always)]
    pub fn nsp_change(&self) -> NSP_CHANGE_R {
        NSP_CHANGE_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Ignore IPG rx_er. When set rx_er has no effect on the GEMs operation when rx_dv is low. Set this when using the RGMII wrapper in half-duplex mode."]
    #[inline(always)]
    pub fn ignore_ipg_rx_er(&self) -> IGNORE_IPG_RX_ER_R {
        IGNORE_IPG_RX_ER_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - N/A"]
    #[inline(always)]
    pub fn rsvd_31(&self) -> RSVD_31_R {
        RSVD_31_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Speed - set to logic one to indicate 100Mbps operation, logic zero for 10Mbps."]
    #[inline(always)]
    #[must_use]
    pub fn speed(&mut self) -> SPEED_W<0> {
        SPEED_W::new(self)
    }
    #[doc = "Bit 1 - Full duplex - if set to logic one, the transmit block ignores the state of collision and carrier sense and allows receive while transmitting. Also controls the half_duplex pin."]
    #[inline(always)]
    #[must_use]
    pub fn full_duplex(&mut self) -> FULL_DUPLEX_W<1> {
        FULL_DUPLEX_W::new(self)
    }
    #[doc = "Bit 2 - Discard non-VLAN frames - when set only VLAN tagged frames will be passed to the address matching logic."]
    #[inline(always)]
    #[must_use]
    pub fn discard_non_vlan_frames(&mut self) -> DISCARD_NON_VLAN_FRAMES_W<2> {
        DISCARD_NON_VLAN_FRAMES_W::new(self)
    }
    #[doc = "Bit 3 - Jumbo frames - set to one to enable jumbo frames up to `gem_jumbo_max_length bytes to be accepted. The default length is 10,240 bytes."]
    #[inline(always)]
    #[must_use]
    pub fn jumbo_frames(&mut self) -> JUMBO_FRAMES_W<3> {
        JUMBO_FRAMES_W::new(self)
    }
    #[doc = "Bit 4 - Copy all frames - when set to logic one, all valid frames will be accepted."]
    #[inline(always)]
    #[must_use]
    pub fn copy_all_frames(&mut self) -> COPY_ALL_FRAMES_W<4> {
        COPY_ALL_FRAMES_W::new(self)
    }
    #[doc = "Bit 5 - No broadcast - when set to logic one, frames addressed to the broadcast address of all ones will not be accepted."]
    #[inline(always)]
    #[must_use]
    pub fn no_broadcast(&mut self) -> NO_BROADCAST_W<5> {
        NO_BROADCAST_W::new(self)
    }
    #[doc = "Bit 6 - Multicast hash enable - when set, multicast frames will be accepted when the 6 bit hash function of the destination address points to a bit that is set in the hash register."]
    #[inline(always)]
    #[must_use]
    pub fn multicast_hash_enable(&mut self) -> MULTICAST_HASH_ENABLE_W<6> {
        MULTICAST_HASH_ENABLE_W::new(self)
    }
    #[doc = "Bit 7 - Unicast hash enable - when set, unicast frames will be accepted when the 6 bit hash function of the destination address points to a bit that is set in the hash register."]
    #[inline(always)]
    #[must_use]
    pub fn unicast_hash_enable(&mut self) -> UNICAST_HASH_ENABLE_W<7> {
        UNICAST_HASH_ENABLE_W::new(self)
    }
    #[doc = "Bit 8 - Receive 1536 byte frames - setting this bit means the GEM will accept frames up to 1536 bytes in length. Normally the GEM would reject any frame above 1518 bytes."]
    #[inline(always)]
    #[must_use]
    pub fn receive_1536_byte_frames(&mut self) -> RECEIVE_1536_BYTE_FRAMES_W<8> {
        RECEIVE_1536_BYTE_FRAMES_W::new(self)
    }
    #[doc = "Bit 9 - External address match enable - when set the external address match interface can be used to copy frames to memory."]
    #[inline(always)]
    #[must_use]
    pub fn external_address_match_enable(&mut self) -> EXTERNAL_ADDRESS_MATCH_ENABLE_W<9> {
        EXTERNAL_ADDRESS_MATCH_ENABLE_W::new(self)
    }
    #[doc = "Bit 10 - Gigabit mode enable - setting this bit configures the GEM for 1000 Mbps operation. 0: 10/100 operation using MII interface 1: Gigabit operation using GMI interface"]
    #[inline(always)]
    #[must_use]
    pub fn gigabit_mode_enable(&mut self) -> GIGABIT_MODE_ENABLE_W<10> {
        GIGABIT_MODE_ENABLE_W::new(self)
    }
    #[doc = "Bit 11 - PCS select - selects between MII/GMII and TBI"]
    #[inline(always)]
    #[must_use]
    pub fn pcs_select(&mut self) -> PCS_SELECT_W<11> {
        PCS_SELECT_W::new(self)
    }
    #[doc = "Bit 12 - Retry test - must be set to zero for normal operation. If set to one the backoff between collisions will always be one slot time. Setting this bit to one helps test the too many retries condition. Also used in the pause frame tests to reduce the pause counter's decrement time from 512 bit times, to every rx_clk cycle."]
    #[inline(always)]
    #[must_use]
    pub fn retry_test(&mut self) -> RETRY_TEST_W<12> {
        RETRY_TEST_W::new(self)
    }
    #[doc = "Bit 13 - Pause enable - when set, transmission will pause if a non zero 802.3 classic pause frame is received and PFC has not been negotiated."]
    #[inline(always)]
    #[must_use]
    pub fn pause_enable(&mut self) -> PAUSE_ENABLE_W<13> {
        PAUSE_ENABLE_W::new(self)
    }
    #[doc = "Bits 14:15 - Receive buffer offset - indicates the number of bytes by which the received data is offset from the start of the receive buffer."]
    #[inline(always)]
    #[must_use]
    pub fn receive_buffer_offset(&mut self) -> RECEIVE_BUFFER_OFFSET_W<14> {
        RECEIVE_BUFFER_OFFSET_W::new(self)
    }
    #[doc = "Bit 16 - Length field error frame discard - setting this bit causes frames with a measured length shorter than the extracted length field (as indicated by bytes 13 and 14 in a non-VLAN tagged frame) to be discarded. This only applies to frames with a length field less than 0x0600."]
    #[inline(always)]
    #[must_use]
    pub fn length_field_error_frame_discard(&mut self) -> LENGTH_FIELD_ERROR_FRAME_DISCARD_W<16> {
        LENGTH_FIELD_ERROR_FRAME_DISCARD_W::new(self)
    }
    #[doc = "Bit 17 - FCS remove - setting this bit will cause received frames to be written to memory without their frame check sequence (last 4 bytes). The frame length indicated will be reduced by four bytes in this mode."]
    #[inline(always)]
    #[must_use]
    pub fn fcs_remove(&mut self) -> FCS_REMOVE_W<17> {
        FCS_REMOVE_W::new(self)
    }
    #[doc = "Bits 18:20 - MDC clock division - set according to pclk speed. These three bits determine the number pclk will be divided by to generate MDC. For conformance with the 802.3 specification, MDC must not exceed 2.5 MHz (MDC is only active during MDIO read and write operations). The reset value for this can be changed by defining a new value for gem_mdc_clock_div in gem_defs.v 000: divide pclk by 8 (pclk up to 20 MHz) 001: divide pclk by 16 (pclk up to 40 MHz) 010: divide pclk by 32 (pclk up to 80 MHz) 011: divide pclk by 48 (pclk up to 120MHz) 100: divide pclk by 64 (pclk up to 160 MHz) 101: divide pclk by 96 (pclk up to 240 MHz) 110: divide pclk by 128 (pclk up to 320 MHz) 111: divide pclk by 224 (pclk up to 540 MHz). Note. The reset value of this field is equal to the gem_mdc_clock_div define, which is user configurable."]
    #[inline(always)]
    #[must_use]
    pub fn mdc_clock_division(&mut self) -> MDC_CLOCK_DIVISION_W<18> {
        MDC_CLOCK_DIVISION_W::new(self)
    }
    #[doc = "Bits 21:22 - Data bus width - set according to AMBA (AXI/AHB) or external FIFO data bus width. The reset value for this can be changed by defining a new value for gem_dma_bus_width_def in gem_defs. Only valid bus widths may be written if the system is configured to a maximum width less than 128-bits. 00: 32 bit data bus width 01: 64 bit AMBA (AXI/AHB) data bus width 10: 128 bit AMBA (AXI/AHB) data bus width 11: 128 bit AMBA (AXI/AHB) data bus width. Note. The reset value of this field is equal to the gem_dma_bus_width_def define, which is user configurable. Note: For AXI_MASTER_PRESENT=1, MXETH only supports 64b DMA data bus width, so must set it 1. For AXI_MASTER_PRESENT=0, MXETH only supports 32b DMA data bus width, so must set it 0."]
    #[inline(always)]
    #[must_use]
    pub fn data_bus_width(&mut self) -> DATA_BUS_WIDTH_W<21> {
        DATA_BUS_WIDTH_W::new(self)
    }
    #[doc = "Bit 23 - Disable copy of pause frames - set to one to prevent pause frames being copied to memory. When set, neither control frames with type id 8808, nor pause frames with destination address 010000c28001 are copied to memory, this functionality was enhanced in release 1p09 (mxeth uses 1p09). Note that valid pause frames received will still increment pause statistics and pause the transmission of frames as required."]
    #[inline(always)]
    #[must_use]
    pub fn disable_copy_of_pause_frames(&mut self) -> DISABLE_COPY_OF_PAUSE_FRAMES_W<23> {
        DISABLE_COPY_OF_PAUSE_FRAMES_W::new(self)
    }
    #[doc = "Bit 24 - Receive checksum offload enable - when set, the receive checksum engine is enabled. Frames with bad IP, TCP or UDP checksums are discarded."]
    #[inline(always)]
    #[must_use]
    pub fn receive_checksum_offload_enable(&mut self) -> RECEIVE_CHECKSUM_OFFLOAD_ENABLE_W<24> {
        RECEIVE_CHECKSUM_OFFLOAD_ENABLE_W::new(self)
    }
    #[doc = "Bit 25 - Enable frames to be received in half-duplex mode while transmitting. Must set '0', MXETH only supports full-duplex mode."]
    #[inline(always)]
    #[must_use]
    pub fn en_half_duplex_rx(&mut self) -> EN_HALF_DUPLEX_RX_W<25> {
        EN_HALF_DUPLEX_RX_W::new(self)
    }
    #[doc = "Bit 26 - Ignore RX FCS - when set frames with FCS/CRC errors will not be rejected. FCS error statistics will still be collected for frames with bad FCS and FCS status will be recorded in frame's DMA descriptor. For normal operation this bit must be set to zero."]
    #[inline(always)]
    #[must_use]
    pub fn ignore_rx_fcs(&mut self) -> IGNORE_RX_FCS_W<26> {
        IGNORE_RX_FCS_W::new(self)
    }
    #[doc = "Bit 27 - SGMII mode enable - changes behaviour of the auto-negotiation advertisement and link partner ability registers to meet the requirements of SGMII and reduces the duration of the link timer from 10 ms to 1.6 ms."]
    #[inline(always)]
    #[must_use]
    pub fn sgmii_mode_enable(&mut self) -> SGMII_MODE_ENABLE_W<27> {
        SGMII_MODE_ENABLE_W::new(self)
    }
    #[doc = "Bit 28 - IPG stretch enable - when set the transmit IPG can be increased above 96 bit times depending on the previous frame length using the IPG stretch register."]
    #[inline(always)]
    #[must_use]
    pub fn ipg_stretch_enable(&mut self) -> IPG_STRETCH_ENABLE_W<28> {
        IPG_STRETCH_ENABLE_W::new(self)
    }
    #[doc = "Bit 29 - Receive bad preamble. When set frames with non-standard preamble are not rejected."]
    #[inline(always)]
    #[must_use]
    pub fn nsp_change(&mut self) -> NSP_CHANGE_W<29> {
        NSP_CHANGE_W::new(self)
    }
    #[doc = "Bit 30 - Ignore IPG rx_er. When set rx_er has no effect on the GEMs operation when rx_dv is low. Set this when using the RGMII wrapper in half-duplex mode."]
    #[inline(always)]
    #[must_use]
    pub fn ignore_ipg_rx_er(&mut self) -> IGNORE_IPG_RX_ER_W<30> {
        IGNORE_IPG_RX_ER_W::new(self)
    }
    #[doc = "Bit 31 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn rsvd_31(&mut self) -> RSVD_31_W<31> {
        RSVD_31_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The network configuration register contains functions for setting the mode of operation for the Gigabit Ethernet MAC.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [network_config](index.html) module"]
pub struct NETWORK_CONFIG_SPEC;
impl crate::RegisterSpec for NETWORK_CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [network_config::R](R) reader structure"]
impl crate::Readable for NETWORK_CONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [network_config::W](W) writer structure"]
impl crate::Writable for NETWORK_CONFIG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets NETWORK_CONFIG to value 0x002c_0000"]
impl crate::Resettable for NETWORK_CONFIG_SPEC {
    const RESET_VALUE: Self::Ux = 0x002c_0000;
}
