#[doc = "Register `PBUF_TXCUTTHRU` reader"]
pub struct R(crate::R<PBUF_TXCUTTHRU_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PBUF_TXCUTTHRU_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PBUF_TXCUTTHRU_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PBUF_TXCUTTHRU_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PBUF_TXCUTTHRU` writer"]
pub struct W(crate::W<PBUF_TXCUTTHRU_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PBUF_TXCUTTHRU_SPEC>;
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
impl From<crate::W<PBUF_TXCUTTHRU_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PBUF_TXCUTTHRU_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DMA_TX_CUTTHRU_THRESHOLD` reader - Watermark value. This value must be >= 0x9. The reset value depends on the value of the configuration option `gem_tx_pbuf_addr, which is defined in the verilog defs configuration file. The value chosen for the generation of the userguide was `gem_tx_pbuf_addr = 9"]
pub type DMA_TX_CUTTHRU_THRESHOLD_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DMA_TX_CUTTHRU_THRESHOLD` writer - Watermark value. This value must be >= 0x9. The reset value depends on the value of the configuration option `gem_tx_pbuf_addr, which is defined in the verilog defs configuration file. The value chosen for the generation of the userguide was `gem_tx_pbuf_addr = 9"]
pub type DMA_TX_CUTTHRU_THRESHOLD_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PBUF_TXCUTTHRU_SPEC, u16, u16, 9, O>;
#[doc = "Field `DMA_TX_CUTTHRU` reader - Enable TX partial store and forward operation"]
pub type DMA_TX_CUTTHRU_R = crate::BitReader<bool>;
#[doc = "Field `DMA_TX_CUTTHRU` writer - Enable TX partial store and forward operation"]
pub type DMA_TX_CUTTHRU_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PBUF_TXCUTTHRU_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:8 - Watermark value. This value must be >= 0x9. The reset value depends on the value of the configuration option `gem_tx_pbuf_addr, which is defined in the verilog defs configuration file. The value chosen for the generation of the userguide was `gem_tx_pbuf_addr = 9"]
    #[inline(always)]
    pub fn dma_tx_cutthru_threshold(&self) -> DMA_TX_CUTTHRU_THRESHOLD_R {
        DMA_TX_CUTTHRU_THRESHOLD_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bit 31 - Enable TX partial store and forward operation"]
    #[inline(always)]
    pub fn dma_tx_cutthru(&self) -> DMA_TX_CUTTHRU_R {
        DMA_TX_CUTTHRU_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:8 - Watermark value. This value must be >= 0x9. The reset value depends on the value of the configuration option `gem_tx_pbuf_addr, which is defined in the verilog defs configuration file. The value chosen for the generation of the userguide was `gem_tx_pbuf_addr = 9"]
    #[inline(always)]
    #[must_use]
    pub fn dma_tx_cutthru_threshold(&mut self) -> DMA_TX_CUTTHRU_THRESHOLD_W<0> {
        DMA_TX_CUTTHRU_THRESHOLD_W::new(self)
    }
    #[doc = "Bit 31 - Enable TX partial store and forward operation"]
    #[inline(always)]
    #[must_use]
    pub fn dma_tx_cutthru(&mut self) -> DMA_TX_CUTTHRU_W<31> {
        DMA_TX_CUTTHRU_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Partial store and forward is only applicable when using the DMA configured in SRAM based packet buffer mode. It is also not available when using multi buffer frames. TX Partial Store and Forward\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pbuf_txcutthru](index.html) module"]
pub struct PBUF_TXCUTTHRU_SPEC;
impl crate::RegisterSpec for PBUF_TXCUTTHRU_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pbuf_txcutthru::R](R) reader structure"]
impl crate::Readable for PBUF_TXCUTTHRU_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pbuf_txcutthru::W](W) writer structure"]
impl crate::Writable for PBUF_TXCUTTHRU_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PBUF_TXCUTTHRU to value 0x01ff"]
impl crate::Resettable for PBUF_TXCUTTHRU_SPEC {
    const RESET_VALUE: Self::Ux = 0x01ff;
}
