#[doc = "Register `DMA_RXBUF_SIZE_Q8` reader"]
pub struct R(crate::R<DMA_RXBUF_SIZE_Q8_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_RXBUF_SIZE_Q8_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_RXBUF_SIZE_Q8_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_RXBUF_SIZE_Q8_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `REMOVED_31_0` reader - Write ignore, read 0"]
pub type REMOVED_31_0_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Write ignore, read 0"]
    #[inline(always)]
    pub fn removed_31_0(&self) -> REMOVED_31_0_R {
        REMOVED_31_0_R::new(self.bits)
    }
}
#[doc = "dma_rxbuf_size_q8 to dma_rxbuf_size_q15 doesn't present. Access to the register returns AHB error.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_rxbuf_size_q8](index.html) module"]
pub struct DMA_RXBUF_SIZE_Q8_SPEC;
impl crate::RegisterSpec for DMA_RXBUF_SIZE_Q8_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma_rxbuf_size_q8::R](R) reader structure"]
impl crate::Readable for DMA_RXBUF_SIZE_Q8_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DMA_RXBUF_SIZE_Q8 to value 0"]
impl crate::Resettable for DMA_RXBUF_SIZE_Q8_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
