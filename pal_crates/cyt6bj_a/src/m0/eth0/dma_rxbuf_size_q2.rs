#[doc = "Register `DMA_RXBUF_SIZE_Q2` reader"]
pub struct R(crate::R<DMA_RXBUF_SIZE_Q2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_RXBUF_SIZE_Q2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_RXBUF_SIZE_Q2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_RXBUF_SIZE_Q2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMA_RXBUF_SIZE_Q2` writer"]
pub struct W(crate::W<DMA_RXBUF_SIZE_Q2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMA_RXBUF_SIZE_Q2_SPEC>;
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
impl From<crate::W<DMA_RXBUF_SIZE_Q2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMA_RXBUF_SIZE_Q2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DMA_RX_Q_BUF_SIZE` reader - DMA receive buffer size in system memory. The value defined by these bits determines the size of buffer to use in main system memory when writing received data. The value is defined in multiples of 64 bytes. 0x01 corresponds to buffers of 64 bytes. 0x02 corresponds to 128 bytes etc. For example: 0x02: 128 byte 0x18: 1536 byte (1*max length frame/buffer) 0xA0: 10240 byte (1*10K jumbo frame/buffer) Note that this value should never be written as zero. Note. The reset value of this field is equal to the gem_rx_buffer_length_def define, which is user configurable."]
pub type DMA_RX_Q_BUF_SIZE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DMA_RX_Q_BUF_SIZE` writer - DMA receive buffer size in system memory. The value defined by these bits determines the size of buffer to use in main system memory when writing received data. The value is defined in multiples of 64 bytes. 0x01 corresponds to buffers of 64 bytes. 0x02 corresponds to 128 bytes etc. For example: 0x02: 128 byte 0x18: 1536 byte (1*max length frame/buffer) 0xA0: 10240 byte (1*10K jumbo frame/buffer) Note that this value should never be written as zero. Note. The reset value of this field is equal to the gem_rx_buffer_length_def define, which is user configurable."]
pub type DMA_RX_Q_BUF_SIZE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DMA_RXBUF_SIZE_Q2_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - DMA receive buffer size in system memory. The value defined by these bits determines the size of buffer to use in main system memory when writing received data. The value is defined in multiples of 64 bytes. 0x01 corresponds to buffers of 64 bytes. 0x02 corresponds to 128 bytes etc. For example: 0x02: 128 byte 0x18: 1536 byte (1*max length frame/buffer) 0xA0: 10240 byte (1*10K jumbo frame/buffer) Note that this value should never be written as zero. Note. The reset value of this field is equal to the gem_rx_buffer_length_def define, which is user configurable."]
    #[inline(always)]
    pub fn dma_rx_q_buf_size(&self) -> DMA_RX_Q_BUF_SIZE_R {
        DMA_RX_Q_BUF_SIZE_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - DMA receive buffer size in system memory. The value defined by these bits determines the size of buffer to use in main system memory when writing received data. The value is defined in multiples of 64 bytes. 0x01 corresponds to buffers of 64 bytes. 0x02 corresponds to 128 bytes etc. For example: 0x02: 128 byte 0x18: 1536 byte (1*max length frame/buffer) 0xA0: 10240 byte (1*10K jumbo frame/buffer) Note that this value should never be written as zero. Note. The reset value of this field is equal to the gem_rx_buffer_length_def define, which is user configurable."]
    #[inline(always)]
    #[must_use]
    pub fn dma_rx_q_buf_size(&mut self) -> DMA_RX_Q_BUF_SIZE_W<0> {
        DMA_RX_Q_BUF_SIZE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Receive Buffer queue 2 Size\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_rxbuf_size_q2](index.html) module"]
pub struct DMA_RXBUF_SIZE_Q2_SPEC;
impl crate::RegisterSpec for DMA_RXBUF_SIZE_Q2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma_rxbuf_size_q2::R](R) reader structure"]
impl crate::Readable for DMA_RXBUF_SIZE_Q2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dma_rxbuf_size_q2::W](W) writer structure"]
impl crate::Writable for DMA_RXBUF_SIZE_Q2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMA_RXBUF_SIZE_Q2 to value 0x18"]
impl crate::Resettable for DMA_RXBUF_SIZE_Q2_SPEC {
    const RESET_VALUE: Self::Ux = 0x18;
}
