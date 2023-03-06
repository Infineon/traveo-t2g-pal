#[doc = "Register `DMA_RXBUF_SIZE_Q3` reader"]
pub struct R(crate::R<DMA_RXBUF_SIZE_Q3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_RXBUF_SIZE_Q3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_RXBUF_SIZE_Q3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_RXBUF_SIZE_Q3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `REMOVED_31_0` reader - DMA receive buffer size in system memory. The value defined by these bits determines the size of buffer to use in main system memory when writing received data. The value is defined in multiples of 64 bytes. 0x01 corresponds to buffers of 64 bytes. 0x02 corresponds to 128 bytes etc. For example: 0x02: 128 byte 0x18: 1536 byte (1*max length frame/buffer) 0xA0: 10240 byte (1*10K jumbo frame/buffer) Note that this value should never be written as zero. Note. The reset value of this field is equal to the gem_rx_buffer_length_def define, which is user configurable."]
pub type REMOVED_31_0_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - DMA receive buffer size in system memory. The value defined by these bits determines the size of buffer to use in main system memory when writing received data. The value is defined in multiples of 64 bytes. 0x01 corresponds to buffers of 64 bytes. 0x02 corresponds to 128 bytes etc. For example: 0x02: 128 byte 0x18: 1536 byte (1*max length frame/buffer) 0xA0: 10240 byte (1*10K jumbo frame/buffer) Note that this value should never be written as zero. Note. The reset value of this field is equal to the gem_rx_buffer_length_def define, which is user configurable."]
    #[inline(always)]
    pub fn removed_31_0(&self) -> REMOVED_31_0_R {
        REMOVED_31_0_R::new(self.bits)
    }
}
#[doc = "dma_rxbuf_size_q3 to dma_rxbuf_size_q7 doesn't present.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_rxbuf_size_q3](index.html) module"]
pub struct DMA_RXBUF_SIZE_Q3_SPEC;
impl crate::RegisterSpec for DMA_RXBUF_SIZE_Q3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma_rxbuf_size_q3::R](R) reader structure"]
impl crate::Readable for DMA_RXBUF_SIZE_Q3_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DMA_RXBUF_SIZE_Q3 to value 0"]
impl crate::Resettable for DMA_RXBUF_SIZE_Q3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
