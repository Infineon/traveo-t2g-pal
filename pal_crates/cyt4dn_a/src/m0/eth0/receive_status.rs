#[doc = "Register `RECEIVE_STATUS` reader"]
pub struct R(crate::R<RECEIVE_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RECEIVE_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RECEIVE_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RECEIVE_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RECEIVE_STATUS` writer"]
pub struct W(crate::W<RECEIVE_STATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RECEIVE_STATUS_SPEC>;
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
impl From<crate::W<RECEIVE_STATUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RECEIVE_STATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BUFFER_NOT_AVAILABLE` reader - Buffer not available - an attempt was made to get a new buffer and the pointer indicated that it was owned by the processor. The DMA will reread the pointer each time an end of frame is received until a valid pointer is found. This bit is set following each descriptor read attempt that fails, even if consecutive pointers are unsuccessful and software has in the mean time cleared the status flag. Cleared by writing a one to this bit."]
pub type BUFFER_NOT_AVAILABLE_R = crate::BitReader<bool>;
#[doc = "Field `BUFFER_NOT_AVAILABLE` writer - Buffer not available - an attempt was made to get a new buffer and the pointer indicated that it was owned by the processor. The DMA will reread the pointer each time an end of frame is received until a valid pointer is found. This bit is set following each descriptor read attempt that fails, even if consecutive pointers are unsuccessful and software has in the mean time cleared the status flag. Cleared by writing a one to this bit."]
pub type BUFFER_NOT_AVAILABLE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RECEIVE_STATUS_SPEC, bool, O>;
#[doc = "Field `FRAME_RECEIVED` reader - Frame received - one or more frames have been received and placed in memory. Cleared by writing a one to this bit."]
pub type FRAME_RECEIVED_R = crate::BitReader<bool>;
#[doc = "Field `FRAME_RECEIVED` writer - Frame received - one or more frames have been received and placed in memory. Cleared by writing a one to this bit."]
pub type FRAME_RECEIVED_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RECEIVE_STATUS_SPEC, bool, O>;
#[doc = "Field `RECEIVE_OVERRUN123` reader - Receive overrun - this bit is set if either the gem_dma RX FIFO or external RX FIFO were unable to store the receive frame due to a FIFO overflow, or if the receive status, reported by the gem_rx module to the gem_dma was not taken at end of frame. This bit is also set in DMA packet buffer mode if the packet buffer overflows. For DMA operation the buffer will be recovered if an overrun occurs. This bit is cleared by writing a one to it."]
pub type RECEIVE_OVERRUN123_R = crate::BitReader<bool>;
#[doc = "Field `RECEIVE_OVERRUN123` writer - Receive overrun - this bit is set if either the gem_dma RX FIFO or external RX FIFO were unable to store the receive frame due to a FIFO overflow, or if the receive status, reported by the gem_rx module to the gem_dma was not taken at end of frame. This bit is also set in DMA packet buffer mode if the packet buffer overflows. For DMA operation the buffer will be recovered if an overrun occurs. This bit is cleared by writing a one to it."]
pub type RECEIVE_OVERRUN123_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RECEIVE_STATUS_SPEC, bool, O>;
#[doc = "Field `RESP_NOT_OK1234` reader - bresp not OK - set when the DMA block sees bresp/hresp not OK. Cleared by writing a one to this bit."]
pub type RESP_NOT_OK1234_R = crate::BitReader<bool>;
#[doc = "Field `RESP_NOT_OK1234` writer - bresp not OK - set when the DMA block sees bresp/hresp not OK. Cleared by writing a one to this bit."]
pub type RESP_NOT_OK1234_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RECEIVE_STATUS_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Buffer not available - an attempt was made to get a new buffer and the pointer indicated that it was owned by the processor. The DMA will reread the pointer each time an end of frame is received until a valid pointer is found. This bit is set following each descriptor read attempt that fails, even if consecutive pointers are unsuccessful and software has in the mean time cleared the status flag. Cleared by writing a one to this bit."]
    #[inline(always)]
    pub fn buffer_not_available(&self) -> BUFFER_NOT_AVAILABLE_R {
        BUFFER_NOT_AVAILABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Frame received - one or more frames have been received and placed in memory. Cleared by writing a one to this bit."]
    #[inline(always)]
    pub fn frame_received(&self) -> FRAME_RECEIVED_R {
        FRAME_RECEIVED_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Receive overrun - this bit is set if either the gem_dma RX FIFO or external RX FIFO were unable to store the receive frame due to a FIFO overflow, or if the receive status, reported by the gem_rx module to the gem_dma was not taken at end of frame. This bit is also set in DMA packet buffer mode if the packet buffer overflows. For DMA operation the buffer will be recovered if an overrun occurs. This bit is cleared by writing a one to it."]
    #[inline(always)]
    pub fn receive_overrun123(&self) -> RECEIVE_OVERRUN123_R {
        RECEIVE_OVERRUN123_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - bresp not OK - set when the DMA block sees bresp/hresp not OK. Cleared by writing a one to this bit."]
    #[inline(always)]
    pub fn resp_not_ok1234(&self) -> RESP_NOT_OK1234_R {
        RESP_NOT_OK1234_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Buffer not available - an attempt was made to get a new buffer and the pointer indicated that it was owned by the processor. The DMA will reread the pointer each time an end of frame is received until a valid pointer is found. This bit is set following each descriptor read attempt that fails, even if consecutive pointers are unsuccessful and software has in the mean time cleared the status flag. Cleared by writing a one to this bit."]
    #[inline(always)]
    #[must_use]
    pub fn buffer_not_available(&mut self) -> BUFFER_NOT_AVAILABLE_W<0> {
        BUFFER_NOT_AVAILABLE_W::new(self)
    }
    #[doc = "Bit 1 - Frame received - one or more frames have been received and placed in memory. Cleared by writing a one to this bit."]
    #[inline(always)]
    #[must_use]
    pub fn frame_received(&mut self) -> FRAME_RECEIVED_W<1> {
        FRAME_RECEIVED_W::new(self)
    }
    #[doc = "Bit 2 - Receive overrun - this bit is set if either the gem_dma RX FIFO or external RX FIFO were unable to store the receive frame due to a FIFO overflow, or if the receive status, reported by the gem_rx module to the gem_dma was not taken at end of frame. This bit is also set in DMA packet buffer mode if the packet buffer overflows. For DMA operation the buffer will be recovered if an overrun occurs. This bit is cleared by writing a one to it."]
    #[inline(always)]
    #[must_use]
    pub fn receive_overrun123(&mut self) -> RECEIVE_OVERRUN123_W<2> {
        RECEIVE_OVERRUN123_W::new(self)
    }
    #[doc = "Bit 3 - bresp not OK - set when the DMA block sees bresp/hresp not OK. Cleared by writing a one to this bit."]
    #[inline(always)]
    #[must_use]
    pub fn resp_not_ok1234(&mut self) -> RESP_NOT_OK1234_W<3> {
        RESP_NOT_OK1234_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register, when read provides details of the status of a receive. Once read, individual bits may be cleared by writing 1 to them. It is not possible to set a bit to 1 by writing to the register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [receive_status](index.html) module"]
pub struct RECEIVE_STATUS_SPEC;
impl crate::RegisterSpec for RECEIVE_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [receive_status::R](R) reader structure"]
impl crate::Readable for RECEIVE_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [receive_status::W](W) writer structure"]
impl crate::Writable for RECEIVE_STATUS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RECEIVE_STATUS to value 0"]
impl crate::Resettable for RECEIVE_STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
