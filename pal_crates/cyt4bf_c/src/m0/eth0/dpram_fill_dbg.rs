#[doc = "Register `DPRAM_FILL_DBG` reader"]
pub struct R(crate::R<DPRAM_FILL_DBG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DPRAM_FILL_DBG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DPRAM_FILL_DBG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DPRAM_FILL_DBG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DPRAM_FILL_DBG` writer"]
pub struct W(crate::W<DPRAM_FILL_DBG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DPRAM_FILL_DBG_SPEC>;
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
impl From<crate::W<DPRAM_FILL_DBG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DPRAM_FILL_DBG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DMA_TX_RX_FILL_LEVEL_SELECT` reader - TX/RX Fill Level select - report the fill level for the TX or RX packet buffer."]
pub type DMA_TX_RX_FILL_LEVEL_SELECT_R = crate::BitReader<bool>;
#[doc = "Field `DMA_TX_RX_FILL_LEVEL_SELECT` writer - TX/RX Fill Level select - report the fill level for the TX or RX packet buffer."]
pub type DMA_TX_RX_FILL_LEVEL_SELECT_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DPRAM_FILL_DBG_SPEC, bool, O>;
#[doc = "Field `DMA_TX_Q_FILL_LEVEL_SELECT` reader - TX queue fill level select - select what TX queue to report fill levels for."]
pub type DMA_TX_Q_FILL_LEVEL_SELECT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DMA_TX_Q_FILL_LEVEL_SELECT` writer - TX queue fill level select - select what TX queue to report fill levels for."]
pub type DMA_TX_Q_FILL_LEVEL_SELECT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DPRAM_FILL_DBG_SPEC, u8, u8, 4, O>;
#[doc = "Field `DMA_TX_RX_FILL_LEVEL` reader - Fill Level - TX or RX packet buffer fill level, selected by the tx_q_fill_level_select and tx_rx_fill_level_select registers. Read this register to determine the fill level."]
pub type DMA_TX_RX_FILL_LEVEL_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bit 0 - TX/RX Fill Level select - report the fill level for the TX or RX packet buffer."]
    #[inline(always)]
    pub fn dma_tx_rx_fill_level_select(&self) -> DMA_TX_RX_FILL_LEVEL_SELECT_R {
        DMA_TX_RX_FILL_LEVEL_SELECT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:7 - TX queue fill level select - select what TX queue to report fill levels for."]
    #[inline(always)]
    pub fn dma_tx_q_fill_level_select(&self) -> DMA_TX_Q_FILL_LEVEL_SELECT_R {
        DMA_TX_Q_FILL_LEVEL_SELECT_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 16:31 - Fill Level - TX or RX packet buffer fill level, selected by the tx_q_fill_level_select and tx_rx_fill_level_select registers. Read this register to determine the fill level."]
    #[inline(always)]
    pub fn dma_tx_rx_fill_level(&self) -> DMA_TX_RX_FILL_LEVEL_R {
        DMA_TX_RX_FILL_LEVEL_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - TX/RX Fill Level select - report the fill level for the TX or RX packet buffer."]
    #[inline(always)]
    #[must_use]
    pub fn dma_tx_rx_fill_level_select(&mut self) -> DMA_TX_RX_FILL_LEVEL_SELECT_W<0> {
        DMA_TX_RX_FILL_LEVEL_SELECT_W::new(self)
    }
    #[doc = "Bits 4:7 - TX queue fill level select - select what TX queue to report fill levels for."]
    #[inline(always)]
    #[must_use]
    pub fn dma_tx_q_fill_level_select(&mut self) -> DMA_TX_Q_FILL_LEVEL_SELECT_W<4> {
        DMA_TX_Q_FILL_LEVEL_SELECT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The fill levels for the TX &amp; RX packet buffers can be read using this register, including the fill level for each queue in the TX direction.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dpram_fill_dbg](index.html) module"]
pub struct DPRAM_FILL_DBG_SPEC;
impl crate::RegisterSpec for DPRAM_FILL_DBG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dpram_fill_dbg::R](R) reader structure"]
impl crate::Readable for DPRAM_FILL_DBG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dpram_fill_dbg::W](W) writer structure"]
impl crate::Writable for DPRAM_FILL_DBG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DPRAM_FILL_DBG to value 0"]
impl crate::Resettable for DPRAM_FILL_DBG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
