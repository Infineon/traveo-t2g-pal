#[doc = "Register `PBUF_RXCUTTHRU` reader"]
pub struct R(crate::R<PBUF_RXCUTTHRU_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PBUF_RXCUTTHRU_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PBUF_RXCUTTHRU_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PBUF_RXCUTTHRU_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PBUF_RXCUTTHRU` writer"]
pub struct W(crate::W<PBUF_RXCUTTHRU_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PBUF_RXCUTTHRU_SPEC>;
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
impl From<crate::W<PBUF_RXCUTTHRU_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PBUF_RXCUTTHRU_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DMA_RX_CUTTHRU_THRESHOLD` reader - Watermark value. The reset value depends on the value of the configuration option `gem_rx_pbuf_addr, which is defined in the verilog defs configuration file. The value chosen for the generation of the userguide was `gem_rx_pbuf_addr = 8."]
pub type DMA_RX_CUTTHRU_THRESHOLD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DMA_RX_CUTTHRU_THRESHOLD` writer - Watermark value. The reset value depends on the value of the configuration option `gem_rx_pbuf_addr, which is defined in the verilog defs configuration file. The value chosen for the generation of the userguide was `gem_rx_pbuf_addr = 8."]
pub type DMA_RX_CUTTHRU_THRESHOLD_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PBUF_RXCUTTHRU_SPEC, u8, u8, 8, O>;
#[doc = "Field `DMA_RX_CUTTHRU` reader - Enable RX partial store and forward operation"]
pub type DMA_RX_CUTTHRU_R = crate::BitReader<bool>;
#[doc = "Field `DMA_RX_CUTTHRU` writer - Enable RX partial store and forward operation"]
pub type DMA_RX_CUTTHRU_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PBUF_RXCUTTHRU_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:7 - Watermark value. The reset value depends on the value of the configuration option `gem_rx_pbuf_addr, which is defined in the verilog defs configuration file. The value chosen for the generation of the userguide was `gem_rx_pbuf_addr = 8."]
    #[inline(always)]
    pub fn dma_rx_cutthru_threshold(&self) -> DMA_RX_CUTTHRU_THRESHOLD_R {
        DMA_RX_CUTTHRU_THRESHOLD_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 31 - Enable RX partial store and forward operation"]
    #[inline(always)]
    pub fn dma_rx_cutthru(&self) -> DMA_RX_CUTTHRU_R {
        DMA_RX_CUTTHRU_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Watermark value. The reset value depends on the value of the configuration option `gem_rx_pbuf_addr, which is defined in the verilog defs configuration file. The value chosen for the generation of the userguide was `gem_rx_pbuf_addr = 8."]
    #[inline(always)]
    #[must_use]
    pub fn dma_rx_cutthru_threshold(&mut self) -> DMA_RX_CUTTHRU_THRESHOLD_W<0> {
        DMA_RX_CUTTHRU_THRESHOLD_W::new(self)
    }
    #[doc = "Bit 31 - Enable RX partial store and forward operation"]
    #[inline(always)]
    #[must_use]
    pub fn dma_rx_cutthru(&mut self) -> DMA_RX_CUTTHRU_W<31> {
        DMA_RX_CUTTHRU_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RX Partial Store and Forward\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pbuf_rxcutthru](index.html) module"]
pub struct PBUF_RXCUTTHRU_SPEC;
impl crate::RegisterSpec for PBUF_RXCUTTHRU_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pbuf_rxcutthru::R](R) reader structure"]
impl crate::Readable for PBUF_RXCUTTHRU_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pbuf_rxcutthru::W](W) writer structure"]
impl crate::Writable for PBUF_RXCUTTHRU_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PBUF_RXCUTTHRU to value 0xff"]
impl crate::Resettable for PBUF_RXCUTTHRU_SPEC {
    const RESET_VALUE: Self::Ux = 0xff;
}
