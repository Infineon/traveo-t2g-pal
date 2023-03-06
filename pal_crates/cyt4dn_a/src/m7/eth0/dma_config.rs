#[doc = "Register `DMA_CONFIG` reader"]
pub struct R(crate::R<DMA_CONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_CONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_CONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_CONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMA_CONFIG` writer"]
pub struct W(crate::W<DMA_CONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMA_CONFIG_SPEC>;
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
impl From<crate::W<DMA_CONFIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMA_CONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AMBA_BURST_LENGTH` reader - Selects the burst length to use on the AMBA (AHB/AXI) when transferring frame data. Not used for DMA management operations and only used where space and data size allow and respecting AXI/AHB burst boundary rules. One-hot priority encoding enforced automatically on register writes as follows, where x represents don't care: 1xxxx: Attempt to use bursts of up to 16. 01xxx: Attempt to use bursts of up to 8. 001xx: Attempt to use bursts of up to 4. 0001x: Always use SINGLE bursts. 00001: Always use SINGLE bursts. 00000: Attempt to use bursts of up to 256. Note: MXETH only supports burst length up to 16."]
pub type AMBA_BURST_LENGTH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `AMBA_BURST_LENGTH` writer - Selects the burst length to use on the AMBA (AHB/AXI) when transferring frame data. Not used for DMA management operations and only used where space and data size allow and respecting AXI/AHB burst boundary rules. One-hot priority encoding enforced automatically on register writes as follows, where x represents don't care: 1xxxx: Attempt to use bursts of up to 16. 01xxx: Attempt to use bursts of up to 8. 001xx: Attempt to use bursts of up to 4. 0001x: Always use SINGLE bursts. 00001: Always use SINGLE bursts. 00000: Attempt to use bursts of up to 256. Note: MXETH only supports burst length up to 16."]
pub type AMBA_BURST_LENGTH_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DMA_CONFIG_SPEC, u8, u8, 5, O>;
#[doc = "Field `HDR_DATA_SPLITTING_EN` reader - Enable header data Splitting. When set, receive frames will be forwarded to main memory using a minimum of two DMA data buffers. The first X data buffers will contain the frame header, consisting of the Ethernet,VLAN,(IPv4 or IPv6),(TCP or UDP). X= (frame header size divided by rx_buf_size as defined in bits 23:16 of this register). The last Y data buffers will contain the frame payload. Y= (frame payload size divided by rx_buf_size). Note that for non VLAN/IP/TCP/UDP frames, the header will always be 14 bytes. When this feature is disabled, the frame is forwarded to main memory in blocks of rx_buf_size."]
pub type HDR_DATA_SPLITTING_EN_R = crate::BitReader<bool>;
#[doc = "Field `HDR_DATA_SPLITTING_EN` writer - Enable header data Splitting. When set, receive frames will be forwarded to main memory using a minimum of two DMA data buffers. The first X data buffers will contain the frame header, consisting of the Ethernet,VLAN,(IPv4 or IPv6),(TCP or UDP). X= (frame header size divided by rx_buf_size as defined in bits 23:16 of this register). The last Y data buffers will contain the frame payload. Y= (frame payload size divided by rx_buf_size). Note that for non VLAN/IP/TCP/UDP frames, the header will always be 14 bytes. When this feature is disabled, the frame is forwarded to main memory in blocks of rx_buf_size."]
pub type HDR_DATA_SPLITTING_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DMA_CONFIG_SPEC, bool, O>;
#[doc = "Field `ENDIAN_SWAP_MANAGEMENT` reader - endian swap mode enable for management descriptor accesses. When set, selects swapped endianism for AMBA (AXI) transfers. When clear, selects little endian mode. Note, the default value of this bit equals to `gem_endian_swap_def, which is design time configuration."]
pub type ENDIAN_SWAP_MANAGEMENT_R = crate::BitReader<bool>;
#[doc = "Field `ENDIAN_SWAP_MANAGEMENT` writer - endian swap mode enable for management descriptor accesses. When set, selects swapped endianism for AMBA (AXI) transfers. When clear, selects little endian mode. Note, the default value of this bit equals to `gem_endian_swap_def, which is design time configuration."]
pub type ENDIAN_SWAP_MANAGEMENT_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DMA_CONFIG_SPEC, bool, O>;
#[doc = "Field `ENDIAN_SWAP_PACKET` reader - endian swap mode enable for packet data accesses. When set, selects swapped endianism for AMBA (AXI) transfers. When clear, selects little endian mode. Note, the default value of this bit equals to `gem_endian_swap_def, which is design time configuration."]
pub type ENDIAN_SWAP_PACKET_R = crate::BitReader<bool>;
#[doc = "Field `ENDIAN_SWAP_PACKET` writer - endian swap mode enable for packet data accesses. When set, selects swapped endianism for AMBA (AXI) transfers. When clear, selects little endian mode. Note, the default value of this bit equals to `gem_endian_swap_def, which is design time configuration."]
pub type ENDIAN_SWAP_PACKET_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DMA_CONFIG_SPEC, bool, O>;
#[doc = "Field `RX_PBUF_SIZE` reader - N/A"]
pub type RX_PBUF_SIZE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RX_PBUF_SIZE` writer - N/A"]
pub type RX_PBUF_SIZE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DMA_CONFIG_SPEC, u8, u8, 2, O>;
#[doc = "Field `TX_PBUF_SIZE` reader - N/A"]
pub type TX_PBUF_SIZE_R = crate::BitReader<bool>;
#[doc = "Field `TX_PBUF_SIZE` writer - N/A"]
pub type TX_PBUF_SIZE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_CONFIG_SPEC, bool, O>;
#[doc = "Field `TX_PBUF_TCP_EN` reader - N/A"]
pub type TX_PBUF_TCP_EN_R = crate::BitReader<bool>;
#[doc = "Field `TX_PBUF_TCP_EN` writer - N/A"]
pub type TX_PBUF_TCP_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_CONFIG_SPEC, bool, O>;
#[doc = "Field `INFINITE_LAST_DBUF_SIZE_EN` reader - Forces the DMA to consider the data buffer pointed to by last descriptor in the descriptor list to be of infinite size."]
pub type INFINITE_LAST_DBUF_SIZE_EN_R = crate::BitReader<bool>;
#[doc = "Field `INFINITE_LAST_DBUF_SIZE_EN` writer - Forces the DMA to consider the data buffer pointed to by last descriptor in the descriptor list to be of infinite size."]
pub type INFINITE_LAST_DBUF_SIZE_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DMA_CONFIG_SPEC, bool, O>;
#[doc = "Field `CRC_ERROR_REPORT` reader - When the bit is set, bit 16 of the receive buffer descriptor will represent FCS/CRC error (only if frames with FCS are copied to memory as enabled by bit 26 in the network config register). When this bit is clear, bit 16 of the receive buffer descriptor will represent the Canonical format indicator (CFI) bit as extracted from the receive frame (if the receive buffer descriptor is pointing to the last data buffer of the receive frame and the received frame was VLAN tagged)."]
pub type CRC_ERROR_REPORT_R = crate::BitReader<bool>;
#[doc = "Field `CRC_ERROR_REPORT` writer - When the bit is set, bit 16 of the receive buffer descriptor will represent FCS/CRC error (only if frames with FCS are copied to memory as enabled by bit 26 in the network config register). When this bit is clear, bit 16 of the receive buffer descriptor will represent the Canonical format indicator (CFI) bit as extracted from the receive frame (if the receive buffer descriptor is pointing to the last data buffer of the receive frame and the received frame was VLAN tagged)."]
pub type CRC_ERROR_REPORT_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_CONFIG_SPEC, bool, O>;
#[doc = "Field `RX_BUF_SIZE` reader - DMA receive buffer size in external AMBA (AXI) system memory. The value defined by these bits determines the size of buffer to use in main system memory when writing received data. The value is defined in multiples of 64 bytes. 0x01 corresponds to buffers of 64 bytes 0x02 corresponds to 128 bytes etc. For example: 0x02: 128 byte. 0x18: 1536 byte (1*max length frame/buffer) 0xA0: 10240 byte (1*10K jumbo frame/buffer) Note that this value should never be written as zero. Note. The reset value of this field is equal to the gem_rx_buffer_length_def define, which is user configurable."]
pub type RX_BUF_SIZE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RX_BUF_SIZE` writer - DMA receive buffer size in external AMBA (AXI) system memory. The value defined by these bits determines the size of buffer to use in main system memory when writing received data. The value is defined in multiples of 64 bytes. 0x01 corresponds to buffers of 64 bytes 0x02 corresponds to 128 bytes etc. For example: 0x02: 128 byte. 0x18: 1536 byte (1*max length frame/buffer) 0xA0: 10240 byte (1*10K jumbo frame/buffer) Note that this value should never be written as zero. Note. The reset value of this field is equal to the gem_rx_buffer_length_def define, which is user configurable."]
pub type RX_BUF_SIZE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DMA_CONFIG_SPEC, u8, u8, 8, O>;
#[doc = "Field `FORCE_DISCARD_ON_ERR` reader - Auto Discard RX pkts during lack of resource. When set, the GEM DMA will automatically discard receive packets from the receiver packet buffer memory when no AMBA (AXI) resource is available. When low, then received packets will remain to be stored in the SRAM based packet buffer until AMBA (AXI) buffer resource next becomes available. A write to this bit is ignored if the DMA is not configured in the packet buffer full store and forward mode."]
pub type FORCE_DISCARD_ON_ERR_R = crate::BitReader<bool>;
#[doc = "Field `FORCE_DISCARD_ON_ERR` writer - Auto Discard RX pkts during lack of resource. When set, the GEM DMA will automatically discard receive packets from the receiver packet buffer memory when no AMBA (AXI) resource is available. When low, then received packets will remain to be stored in the SRAM based packet buffer until AMBA (AXI) buffer resource next becomes available. A write to this bit is ignored if the DMA is not configured in the packet buffer full store and forward mode."]
pub type FORCE_DISCARD_ON_ERR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DMA_CONFIG_SPEC, bool, O>;
#[doc = "Field `FORCE_MAX_AMBA_BURST_RX` reader - Force max length bursts on RX. Force the RX DMA to always issue max length bursts on EOP(end of packet) or EOB(end of buffer)transfers, even if there is less than max burst real packet data required to write. Any extra bytes of pad data is set to 0x00. Does not apply on bursts that break 1k boundary rule."]
pub type FORCE_MAX_AMBA_BURST_RX_R = crate::BitReader<bool>;
#[doc = "Field `FORCE_MAX_AMBA_BURST_RX` writer - Force max length bursts on RX. Force the RX DMA to always issue max length bursts on EOP(end of packet) or EOB(end of buffer)transfers, even if there is less than max burst real packet data required to write. Any extra bytes of pad data is set to 0x00. Does not apply on bursts that break 1k boundary rule."]
pub type FORCE_MAX_AMBA_BURST_RX_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DMA_CONFIG_SPEC, bool, O>;
#[doc = "Field `FORCE_MAX_AMBA_BURST_TX` reader - Force max length bursts on TX. Force the TX DMA to always issue max length bursts on EOP(end of packet) or EOB(end of buffer) transfers as defined by bits 4:0 of this register, even when there is less that max burst data bytes to read. Residual data read is ignored. Does not apply on bursts that break 1k boundary rule."]
pub type FORCE_MAX_AMBA_BURST_TX_R = crate::BitReader<bool>;
#[doc = "Field `FORCE_MAX_AMBA_BURST_TX` writer - Force max length bursts on TX. Force the TX DMA to always issue max length bursts on EOP(end of packet) or EOB(end of buffer) transfers as defined by bits 4:0 of this register, even when there is less that max burst data bytes to read. Residual data read is ignored. Does not apply on bursts that break 1k boundary rule."]
pub type FORCE_MAX_AMBA_BURST_TX_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DMA_CONFIG_SPEC, bool, O>;
#[doc = "Field `RX_BD_EXTENDED_MODE_EN` reader - Enable RX extended BD mode. See RX BD control register definition for description of feature."]
pub type RX_BD_EXTENDED_MODE_EN_R = crate::BitReader<bool>;
#[doc = "Field `RX_BD_EXTENDED_MODE_EN` writer - Enable RX extended BD mode. See RX BD control register definition for description of feature."]
pub type RX_BD_EXTENDED_MODE_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DMA_CONFIG_SPEC, bool, O>;
#[doc = "Field `TX_BD_EXTENDED_MODE_EN` reader - Enable TX extended BD mode. See TX BD control register definition for description of feature."]
pub type TX_BD_EXTENDED_MODE_EN_R = crate::BitReader<bool>;
#[doc = "Field `TX_BD_EXTENDED_MODE_EN` writer - Enable TX extended BD mode. See TX BD control register definition for description of feature."]
pub type TX_BD_EXTENDED_MODE_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DMA_CONFIG_SPEC, bool, O>;
#[doc = "Field `DMA_ADDR_BUS_WIDTH_1` reader - DMA address bus width. 0 = 32b, 1 = 64b. MXETH only supports 32b DMA address bus width, so must set it 0."]
pub type DMA_ADDR_BUS_WIDTH_1_R = crate::BitReader<bool>;
#[doc = "Field `DMA_ADDR_BUS_WIDTH_1` writer - DMA address bus width. 0 = 32b, 1 = 64b. MXETH only supports 32b DMA address bus width, so must set it 0."]
pub type DMA_ADDR_BUS_WIDTH_1_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DMA_CONFIG_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:4 - Selects the burst length to use on the AMBA (AHB/AXI) when transferring frame data. Not used for DMA management operations and only used where space and data size allow and respecting AXI/AHB burst boundary rules. One-hot priority encoding enforced automatically on register writes as follows, where x represents don't care: 1xxxx: Attempt to use bursts of up to 16. 01xxx: Attempt to use bursts of up to 8. 001xx: Attempt to use bursts of up to 4. 0001x: Always use SINGLE bursts. 00001: Always use SINGLE bursts. 00000: Attempt to use bursts of up to 256. Note: MXETH only supports burst length up to 16."]
    #[inline(always)]
    pub fn amba_burst_length(&self) -> AMBA_BURST_LENGTH_R {
        AMBA_BURST_LENGTH_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 5 - Enable header data Splitting. When set, receive frames will be forwarded to main memory using a minimum of two DMA data buffers. The first X data buffers will contain the frame header, consisting of the Ethernet,VLAN,(IPv4 or IPv6),(TCP or UDP). X= (frame header size divided by rx_buf_size as defined in bits 23:16 of this register). The last Y data buffers will contain the frame payload. Y= (frame payload size divided by rx_buf_size). Note that for non VLAN/IP/TCP/UDP frames, the header will always be 14 bytes. When this feature is disabled, the frame is forwarded to main memory in blocks of rx_buf_size."]
    #[inline(always)]
    pub fn hdr_data_splitting_en(&self) -> HDR_DATA_SPLITTING_EN_R {
        HDR_DATA_SPLITTING_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - endian swap mode enable for management descriptor accesses. When set, selects swapped endianism for AMBA (AXI) transfers. When clear, selects little endian mode. Note, the default value of this bit equals to `gem_endian_swap_def, which is design time configuration."]
    #[inline(always)]
    pub fn endian_swap_management(&self) -> ENDIAN_SWAP_MANAGEMENT_R {
        ENDIAN_SWAP_MANAGEMENT_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - endian swap mode enable for packet data accesses. When set, selects swapped endianism for AMBA (AXI) transfers. When clear, selects little endian mode. Note, the default value of this bit equals to `gem_endian_swap_def, which is design time configuration."]
    #[inline(always)]
    pub fn endian_swap_packet(&self) -> ENDIAN_SWAP_PACKET_R {
        ENDIAN_SWAP_PACKET_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - N/A"]
    #[inline(always)]
    pub fn rx_pbuf_size(&self) -> RX_PBUF_SIZE_R {
        RX_PBUF_SIZE_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - N/A"]
    #[inline(always)]
    pub fn tx_pbuf_size(&self) -> TX_PBUF_SIZE_R {
        TX_PBUF_SIZE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - N/A"]
    #[inline(always)]
    pub fn tx_pbuf_tcp_en(&self) -> TX_PBUF_TCP_EN_R {
        TX_PBUF_TCP_EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Forces the DMA to consider the data buffer pointed to by last descriptor in the descriptor list to be of infinite size."]
    #[inline(always)]
    pub fn infinite_last_dbuf_size_en(&self) -> INFINITE_LAST_DBUF_SIZE_EN_R {
        INFINITE_LAST_DBUF_SIZE_EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - When the bit is set, bit 16 of the receive buffer descriptor will represent FCS/CRC error (only if frames with FCS are copied to memory as enabled by bit 26 in the network config register). When this bit is clear, bit 16 of the receive buffer descriptor will represent the Canonical format indicator (CFI) bit as extracted from the receive frame (if the receive buffer descriptor is pointing to the last data buffer of the receive frame and the received frame was VLAN tagged)."]
    #[inline(always)]
    pub fn crc_error_report(&self) -> CRC_ERROR_REPORT_R {
        CRC_ERROR_REPORT_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 16:23 - DMA receive buffer size in external AMBA (AXI) system memory. The value defined by these bits determines the size of buffer to use in main system memory when writing received data. The value is defined in multiples of 64 bytes. 0x01 corresponds to buffers of 64 bytes 0x02 corresponds to 128 bytes etc. For example: 0x02: 128 byte. 0x18: 1536 byte (1*max length frame/buffer) 0xA0: 10240 byte (1*10K jumbo frame/buffer) Note that this value should never be written as zero. Note. The reset value of this field is equal to the gem_rx_buffer_length_def define, which is user configurable."]
    #[inline(always)]
    pub fn rx_buf_size(&self) -> RX_BUF_SIZE_R {
        RX_BUF_SIZE_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - Auto Discard RX pkts during lack of resource. When set, the GEM DMA will automatically discard receive packets from the receiver packet buffer memory when no AMBA (AXI) resource is available. When low, then received packets will remain to be stored in the SRAM based packet buffer until AMBA (AXI) buffer resource next becomes available. A write to this bit is ignored if the DMA is not configured in the packet buffer full store and forward mode."]
    #[inline(always)]
    pub fn force_discard_on_err(&self) -> FORCE_DISCARD_ON_ERR_R {
        FORCE_DISCARD_ON_ERR_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Force max length bursts on RX. Force the RX DMA to always issue max length bursts on EOP(end of packet) or EOB(end of buffer)transfers, even if there is less than max burst real packet data required to write. Any extra bytes of pad data is set to 0x00. Does not apply on bursts that break 1k boundary rule."]
    #[inline(always)]
    pub fn force_max_amba_burst_rx(&self) -> FORCE_MAX_AMBA_BURST_RX_R {
        FORCE_MAX_AMBA_BURST_RX_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Force max length bursts on TX. Force the TX DMA to always issue max length bursts on EOP(end of packet) or EOB(end of buffer) transfers as defined by bits 4:0 of this register, even when there is less that max burst data bytes to read. Residual data read is ignored. Does not apply on bursts that break 1k boundary rule."]
    #[inline(always)]
    pub fn force_max_amba_burst_tx(&self) -> FORCE_MAX_AMBA_BURST_TX_R {
        FORCE_MAX_AMBA_BURST_TX_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 28 - Enable RX extended BD mode. See RX BD control register definition for description of feature."]
    #[inline(always)]
    pub fn rx_bd_extended_mode_en(&self) -> RX_BD_EXTENDED_MODE_EN_R {
        RX_BD_EXTENDED_MODE_EN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Enable TX extended BD mode. See TX BD control register definition for description of feature."]
    #[inline(always)]
    pub fn tx_bd_extended_mode_en(&self) -> TX_BD_EXTENDED_MODE_EN_R {
        TX_BD_EXTENDED_MODE_EN_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - DMA address bus width. 0 = 32b, 1 = 64b. MXETH only supports 32b DMA address bus width, so must set it 0."]
    #[inline(always)]
    pub fn dma_addr_bus_width_1(&self) -> DMA_ADDR_BUS_WIDTH_1_R {
        DMA_ADDR_BUS_WIDTH_1_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Selects the burst length to use on the AMBA (AHB/AXI) when transferring frame data. Not used for DMA management operations and only used where space and data size allow and respecting AXI/AHB burst boundary rules. One-hot priority encoding enforced automatically on register writes as follows, where x represents don't care: 1xxxx: Attempt to use bursts of up to 16. 01xxx: Attempt to use bursts of up to 8. 001xx: Attempt to use bursts of up to 4. 0001x: Always use SINGLE bursts. 00001: Always use SINGLE bursts. 00000: Attempt to use bursts of up to 256. Note: MXETH only supports burst length up to 16."]
    #[inline(always)]
    #[must_use]
    pub fn amba_burst_length(&mut self) -> AMBA_BURST_LENGTH_W<0> {
        AMBA_BURST_LENGTH_W::new(self)
    }
    #[doc = "Bit 5 - Enable header data Splitting. When set, receive frames will be forwarded to main memory using a minimum of two DMA data buffers. The first X data buffers will contain the frame header, consisting of the Ethernet,VLAN,(IPv4 or IPv6),(TCP or UDP). X= (frame header size divided by rx_buf_size as defined in bits 23:16 of this register). The last Y data buffers will contain the frame payload. Y= (frame payload size divided by rx_buf_size). Note that for non VLAN/IP/TCP/UDP frames, the header will always be 14 bytes. When this feature is disabled, the frame is forwarded to main memory in blocks of rx_buf_size."]
    #[inline(always)]
    #[must_use]
    pub fn hdr_data_splitting_en(&mut self) -> HDR_DATA_SPLITTING_EN_W<5> {
        HDR_DATA_SPLITTING_EN_W::new(self)
    }
    #[doc = "Bit 6 - endian swap mode enable for management descriptor accesses. When set, selects swapped endianism for AMBA (AXI) transfers. When clear, selects little endian mode. Note, the default value of this bit equals to `gem_endian_swap_def, which is design time configuration."]
    #[inline(always)]
    #[must_use]
    pub fn endian_swap_management(&mut self) -> ENDIAN_SWAP_MANAGEMENT_W<6> {
        ENDIAN_SWAP_MANAGEMENT_W::new(self)
    }
    #[doc = "Bit 7 - endian swap mode enable for packet data accesses. When set, selects swapped endianism for AMBA (AXI) transfers. When clear, selects little endian mode. Note, the default value of this bit equals to `gem_endian_swap_def, which is design time configuration."]
    #[inline(always)]
    #[must_use]
    pub fn endian_swap_packet(&mut self) -> ENDIAN_SWAP_PACKET_W<7> {
        ENDIAN_SWAP_PACKET_W::new(self)
    }
    #[doc = "Bits 8:9 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn rx_pbuf_size(&mut self) -> RX_PBUF_SIZE_W<8> {
        RX_PBUF_SIZE_W::new(self)
    }
    #[doc = "Bit 10 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn tx_pbuf_size(&mut self) -> TX_PBUF_SIZE_W<10> {
        TX_PBUF_SIZE_W::new(self)
    }
    #[doc = "Bit 11 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn tx_pbuf_tcp_en(&mut self) -> TX_PBUF_TCP_EN_W<11> {
        TX_PBUF_TCP_EN_W::new(self)
    }
    #[doc = "Bit 12 - Forces the DMA to consider the data buffer pointed to by last descriptor in the descriptor list to be of infinite size."]
    #[inline(always)]
    #[must_use]
    pub fn infinite_last_dbuf_size_en(&mut self) -> INFINITE_LAST_DBUF_SIZE_EN_W<12> {
        INFINITE_LAST_DBUF_SIZE_EN_W::new(self)
    }
    #[doc = "Bit 13 - When the bit is set, bit 16 of the receive buffer descriptor will represent FCS/CRC error (only if frames with FCS are copied to memory as enabled by bit 26 in the network config register). When this bit is clear, bit 16 of the receive buffer descriptor will represent the Canonical format indicator (CFI) bit as extracted from the receive frame (if the receive buffer descriptor is pointing to the last data buffer of the receive frame and the received frame was VLAN tagged)."]
    #[inline(always)]
    #[must_use]
    pub fn crc_error_report(&mut self) -> CRC_ERROR_REPORT_W<13> {
        CRC_ERROR_REPORT_W::new(self)
    }
    #[doc = "Bits 16:23 - DMA receive buffer size in external AMBA (AXI) system memory. The value defined by these bits determines the size of buffer to use in main system memory when writing received data. The value is defined in multiples of 64 bytes. 0x01 corresponds to buffers of 64 bytes 0x02 corresponds to 128 bytes etc. For example: 0x02: 128 byte. 0x18: 1536 byte (1*max length frame/buffer) 0xA0: 10240 byte (1*10K jumbo frame/buffer) Note that this value should never be written as zero. Note. The reset value of this field is equal to the gem_rx_buffer_length_def define, which is user configurable."]
    #[inline(always)]
    #[must_use]
    pub fn rx_buf_size(&mut self) -> RX_BUF_SIZE_W<16> {
        RX_BUF_SIZE_W::new(self)
    }
    #[doc = "Bit 24 - Auto Discard RX pkts during lack of resource. When set, the GEM DMA will automatically discard receive packets from the receiver packet buffer memory when no AMBA (AXI) resource is available. When low, then received packets will remain to be stored in the SRAM based packet buffer until AMBA (AXI) buffer resource next becomes available. A write to this bit is ignored if the DMA is not configured in the packet buffer full store and forward mode."]
    #[inline(always)]
    #[must_use]
    pub fn force_discard_on_err(&mut self) -> FORCE_DISCARD_ON_ERR_W<24> {
        FORCE_DISCARD_ON_ERR_W::new(self)
    }
    #[doc = "Bit 25 - Force max length bursts on RX. Force the RX DMA to always issue max length bursts on EOP(end of packet) or EOB(end of buffer)transfers, even if there is less than max burst real packet data required to write. Any extra bytes of pad data is set to 0x00. Does not apply on bursts that break 1k boundary rule."]
    #[inline(always)]
    #[must_use]
    pub fn force_max_amba_burst_rx(&mut self) -> FORCE_MAX_AMBA_BURST_RX_W<25> {
        FORCE_MAX_AMBA_BURST_RX_W::new(self)
    }
    #[doc = "Bit 26 - Force max length bursts on TX. Force the TX DMA to always issue max length bursts on EOP(end of packet) or EOB(end of buffer) transfers as defined by bits 4:0 of this register, even when there is less that max burst data bytes to read. Residual data read is ignored. Does not apply on bursts that break 1k boundary rule."]
    #[inline(always)]
    #[must_use]
    pub fn force_max_amba_burst_tx(&mut self) -> FORCE_MAX_AMBA_BURST_TX_W<26> {
        FORCE_MAX_AMBA_BURST_TX_W::new(self)
    }
    #[doc = "Bit 28 - Enable RX extended BD mode. See RX BD control register definition for description of feature."]
    #[inline(always)]
    #[must_use]
    pub fn rx_bd_extended_mode_en(&mut self) -> RX_BD_EXTENDED_MODE_EN_W<28> {
        RX_BD_EXTENDED_MODE_EN_W::new(self)
    }
    #[doc = "Bit 29 - Enable TX extended BD mode. See TX BD control register definition for description of feature."]
    #[inline(always)]
    #[must_use]
    pub fn tx_bd_extended_mode_en(&mut self) -> TX_BD_EXTENDED_MODE_EN_W<29> {
        TX_BD_EXTENDED_MODE_EN_W::new(self)
    }
    #[doc = "Bit 30 - DMA address bus width. 0 = 32b, 1 = 64b. MXETH only supports 32b DMA address bus width, so must set it 0."]
    #[inline(always)]
    #[must_use]
    pub fn dma_addr_bus_width_1(&mut self) -> DMA_ADDR_BUS_WIDTH_1_W<30> {
        DMA_ADDR_BUS_WIDTH_1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_config](index.html) module"]
pub struct DMA_CONFIG_SPEC;
impl crate::RegisterSpec for DMA_CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma_config::R](R) reader structure"]
impl crate::Readable for DMA_CONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dma_config::W](W) writer structure"]
impl crate::Writable for DMA_CONFIG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMA_CONFIG to value 0x0018_0704"]
impl crate::Resettable for DMA_CONFIG_SPEC {
    const RESET_VALUE: Self::Ux = 0x0018_0704;
}
