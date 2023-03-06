#[doc = "Register `DESIGNCFG_DEBUG5` reader"]
pub struct R(crate::R<DESIGNCFG_DEBUG5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DESIGNCFG_DEBUG5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DESIGNCFG_DEBUG5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DESIGNCFG_DEBUG5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RX_FIFO_CNT_WIDTH` reader - Takes the value of the `gem_rx_fifo_cnt_width DEFINE"]
pub type RX_FIFO_CNT_WIDTH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TX_FIFO_CNT_WIDTH` reader - Takes the value of the `gem_tx_fifo_cnt_width DEFINE"]
pub type TX_FIFO_CNT_WIDTH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TSU` reader - Takes the value of the `gem_tsu DEFINE"]
pub type TSU_R = crate::BitReader<bool>;
#[doc = "Field `PHY_IDENT` reader - Takes the value of the `gem_phy_ident DEFINE"]
pub type PHY_IDENT_R = crate::BitReader<bool>;
#[doc = "Field `DMA_BUS_WIDTH_DEF` reader - Takes the value of the `gem_dma_bus_width_def DEFINE"]
pub type DMA_BUS_WIDTH_DEF_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MDC_CLOCK_DIV` reader - Takes the value of the `gem_mdc_clock_div DEFINE"]
pub type MDC_CLOCK_DIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ENDIAN_SWAP_DEF` reader - Takes the value of the `gem_endian_swap_def DEFINE"]
pub type ENDIAN_SWAP_DEF_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RX_PBUF_SIZE_DEF` reader - Takes the value of the `gem_rx_pbuf_size_def DEFINE"]
pub type RX_PBUF_SIZE_DEF_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TX_PBUF_SIZE_DEF` reader - Takes the value of the `gem_tx_pbuf_size_def DEFINE"]
pub type TX_PBUF_SIZE_DEF_R = crate::BitReader<bool>;
#[doc = "Field `RX_BUFFER_LENGTH_DEF` reader - Takes the value of the `gem_rx_buffer_length_def DEFINE"]
pub type RX_BUFFER_LENGTH_DEF_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TSU_CLK` reader - Takes the value of the `gem_tsu_clk DEFINE"]
pub type TSU_CLK_R = crate::BitReader<bool>;
#[doc = "Field `AXI_PROT_VALUE` reader - Takes the value of the `gem_axi_prot_value DEFINE"]
pub type AXI_PROT_VALUE_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:3 - Takes the value of the `gem_rx_fifo_cnt_width DEFINE"]
    #[inline(always)]
    pub fn rx_fifo_cnt_width(&self) -> RX_FIFO_CNT_WIDTH_R {
        RX_FIFO_CNT_WIDTH_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Takes the value of the `gem_tx_fifo_cnt_width DEFINE"]
    #[inline(always)]
    pub fn tx_fifo_cnt_width(&self) -> TX_FIFO_CNT_WIDTH_R {
        TX_FIFO_CNT_WIDTH_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Takes the value of the `gem_tsu DEFINE"]
    #[inline(always)]
    pub fn tsu(&self) -> TSU_R {
        TSU_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Takes the value of the `gem_phy_ident DEFINE"]
    #[inline(always)]
    pub fn phy_ident(&self) -> PHY_IDENT_R {
        PHY_IDENT_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:11 - Takes the value of the `gem_dma_bus_width_def DEFINE"]
    #[inline(always)]
    pub fn dma_bus_width_def(&self) -> DMA_BUS_WIDTH_DEF_R {
        DMA_BUS_WIDTH_DEF_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:14 - Takes the value of the `gem_mdc_clock_div DEFINE"]
    #[inline(always)]
    pub fn mdc_clock_div(&self) -> MDC_CLOCK_DIV_R {
        MDC_CLOCK_DIV_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 15:16 - Takes the value of the `gem_endian_swap_def DEFINE"]
    #[inline(always)]
    pub fn endian_swap_def(&self) -> ENDIAN_SWAP_DEF_R {
        ENDIAN_SWAP_DEF_R::new(((self.bits >> 15) & 3) as u8)
    }
    #[doc = "Bits 17:18 - Takes the value of the `gem_rx_pbuf_size_def DEFINE"]
    #[inline(always)]
    pub fn rx_pbuf_size_def(&self) -> RX_PBUF_SIZE_DEF_R {
        RX_PBUF_SIZE_DEF_R::new(((self.bits >> 17) & 3) as u8)
    }
    #[doc = "Bit 19 - Takes the value of the `gem_tx_pbuf_size_def DEFINE"]
    #[inline(always)]
    pub fn tx_pbuf_size_def(&self) -> TX_PBUF_SIZE_DEF_R {
        TX_PBUF_SIZE_DEF_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:27 - Takes the value of the `gem_rx_buffer_length_def DEFINE"]
    #[inline(always)]
    pub fn rx_buffer_length_def(&self) -> RX_BUFFER_LENGTH_DEF_R {
        RX_BUFFER_LENGTH_DEF_R::new(((self.bits >> 20) & 0xff) as u8)
    }
    #[doc = "Bit 28 - Takes the value of the `gem_tsu_clk DEFINE"]
    #[inline(always)]
    pub fn tsu_clk(&self) -> TSU_CLK_R {
        TSU_CLK_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bits 29:31 - Takes the value of the `gem_axi_prot_value DEFINE"]
    #[inline(always)]
    pub fn axi_prot_value(&self) -> AXI_PROT_VALUE_R {
        AXI_PROT_VALUE_R::new(((self.bits >> 29) & 7) as u8)
    }
}
#[doc = "Design Configuration Register 5\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [designcfg_debug5](index.html) module"]
pub struct DESIGNCFG_DEBUG5_SPEC;
impl crate::RegisterSpec for DESIGNCFG_DEBUG5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [designcfg_debug5::R](R) reader structure"]
impl crate::Readable for DESIGNCFG_DEBUG5_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DESIGNCFG_DEBUG5 to value 0x502e_2744"]
impl crate::Resettable for DESIGNCFG_DEBUG5_SPEC {
    const RESET_VALUE: Self::Ux = 0x502e_2744;
}
