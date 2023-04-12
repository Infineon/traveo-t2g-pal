#[doc = "Register `DESIGNCFG_DEBUG10` reader"]
pub struct R(crate::R<DESIGNCFG_DEBUG10_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DESIGNCFG_DEBUG10_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DESIGNCFG_DEBUG10_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DESIGNCFG_DEBUG10_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `AXI_RX_DESCR_WR_BUFF_BITS` reader - Takes the value of the `gem_axi_rx_descr_wr_buff_bits DEFINE"]
pub type AXI_RX_DESCR_WR_BUFF_BITS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `AXI_TX_DESCR_WR_BUFF_BITS` reader - Takes the value of the `gem_axi_tx_descr_wr_buff_bits DEFINE"]
pub type AXI_TX_DESCR_WR_BUFF_BITS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `AXI_RX_DESCR_RD_BUFF_BITS` reader - Takes the value of the `gem_axi_rx_descr_rd_buff_bits DEFINE"]
pub type AXI_RX_DESCR_RD_BUFF_BITS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `AXI_TX_DESCR_RD_BUFF_BITS` reader - Takes the value of the `gem_axi_tx_descr_rd_buff_bits DEFINE"]
pub type AXI_TX_DESCR_RD_BUFF_BITS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `AXI_ACCESS_PIPELINE_BITS` reader - Takes the value of the `gem_axi_access_pipeline_bits DEFINE"]
pub type AXI_ACCESS_PIPELINE_BITS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RX_PBUF_DATA` reader - Takes the value of the `gem_rx_pbuf_data DEFINE. 1 - The RX DPRAM has a datawidth of 32bits. 2 - The RX DPRAM has a datawidth of 64bits. 4 - RX The DPRAM has a datawidth of 128bits"]
pub type RX_PBUF_DATA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TX_PBUF_DATA` reader - Takes the value of the `gem_tx_pbuf_data DEFINE. 1 - The TX DPRAM has a datawidth of 32bits. 2 - The TX DPRAM has a datawidth of 64bits. 4 - The TX DPRAM has a datawidth of 128bits"]
pub type TX_PBUF_DATA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EMAC_BUS_WIDTH` reader - Takes the value of the `gem_emac_bus_width DEFINE. 1 - The MAC has a datawidth of 32bits. 2 - The MAC has a datawidth of 64bits. 4 - The MAC has a datawidth of 128bits"]
pub type EMAC_BUS_WIDTH_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:3 - Takes the value of the `gem_axi_rx_descr_wr_buff_bits DEFINE"]
    #[inline(always)]
    pub fn axi_rx_descr_wr_buff_bits(&self) -> AXI_RX_DESCR_WR_BUFF_BITS_R {
        AXI_RX_DESCR_WR_BUFF_BITS_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Takes the value of the `gem_axi_tx_descr_wr_buff_bits DEFINE"]
    #[inline(always)]
    pub fn axi_tx_descr_wr_buff_bits(&self) -> AXI_TX_DESCR_WR_BUFF_BITS_R {
        AXI_TX_DESCR_WR_BUFF_BITS_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Takes the value of the `gem_axi_rx_descr_rd_buff_bits DEFINE"]
    #[inline(always)]
    pub fn axi_rx_descr_rd_buff_bits(&self) -> AXI_RX_DESCR_RD_BUFF_BITS_R {
        AXI_RX_DESCR_RD_BUFF_BITS_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Takes the value of the `gem_axi_tx_descr_rd_buff_bits DEFINE"]
    #[inline(always)]
    pub fn axi_tx_descr_rd_buff_bits(&self) -> AXI_TX_DESCR_RD_BUFF_BITS_R {
        AXI_TX_DESCR_RD_BUFF_BITS_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Takes the value of the `gem_axi_access_pipeline_bits DEFINE"]
    #[inline(always)]
    pub fn axi_access_pipeline_bits(&self) -> AXI_ACCESS_PIPELINE_BITS_R {
        AXI_ACCESS_PIPELINE_BITS_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Takes the value of the `gem_rx_pbuf_data DEFINE. 1 - The RX DPRAM has a datawidth of 32bits. 2 - The RX DPRAM has a datawidth of 64bits. 4 - RX The DPRAM has a datawidth of 128bits"]
    #[inline(always)]
    pub fn rx_pbuf_data(&self) -> RX_PBUF_DATA_R {
        RX_PBUF_DATA_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Takes the value of the `gem_tx_pbuf_data DEFINE. 1 - The TX DPRAM has a datawidth of 32bits. 2 - The TX DPRAM has a datawidth of 64bits. 4 - The TX DPRAM has a datawidth of 128bits"]
    #[inline(always)]
    pub fn tx_pbuf_data(&self) -> TX_PBUF_DATA_R {
        TX_PBUF_DATA_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Takes the value of the `gem_emac_bus_width DEFINE. 1 - The MAC has a datawidth of 32bits. 2 - The MAC has a datawidth of 64bits. 4 - The MAC has a datawidth of 128bits"]
    #[inline(always)]
    pub fn emac_bus_width(&self) -> EMAC_BUS_WIDTH_R {
        EMAC_BUS_WIDTH_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
#[doc = "Design Configuration Register 10\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [designcfg_debug10](index.html) module"]
pub struct DESIGNCFG_DEBUG10_SPEC;
impl crate::RegisterSpec for DESIGNCFG_DEBUG10_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [designcfg_debug10::R](R) reader structure"]
impl crate::Readable for DESIGNCFG_DEBUG10_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DESIGNCFG_DEBUG10 to value 0x1441_1111"]
impl crate::Resettable for DESIGNCFG_DEBUG10_SPEC {
    const RESET_VALUE: Self::Ux = 0x1441_1111;
}
