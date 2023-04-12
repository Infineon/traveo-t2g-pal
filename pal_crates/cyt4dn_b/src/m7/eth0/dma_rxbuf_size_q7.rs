#[doc = "Register `DMA_RXBUF_SIZE_Q7` reader"]
pub struct R(crate::R<DMA_RXBUF_SIZE_Q7_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_RXBUF_SIZE_Q7_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_RXBUF_SIZE_Q7_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_RXBUF_SIZE_Q7_SPEC>) -> Self {
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
#[doc = "Not presents.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_rxbuf_size_q7](index.html) module"]
pub struct DMA_RXBUF_SIZE_Q7_SPEC;
impl crate::RegisterSpec for DMA_RXBUF_SIZE_Q7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma_rxbuf_size_q7::R](R) reader structure"]
impl crate::Readable for DMA_RXBUF_SIZE_Q7_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DMA_RXBUF_SIZE_Q7 to value 0"]
impl crate::Resettable for DMA_RXBUF_SIZE_Q7_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
