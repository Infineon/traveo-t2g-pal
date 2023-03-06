#[doc = "Register `TRANSMIT_STATUS` reader"]
pub struct R(crate::R<TRANSMIT_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TRANSMIT_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TRANSMIT_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TRANSMIT_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TRANSMIT_STATUS` writer"]
pub struct W(crate::W<TRANSMIT_STATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TRANSMIT_STATUS_SPEC>;
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
impl From<crate::W<TRANSMIT_STATUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TRANSMIT_STATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USED_BIT_READ` reader - Used bit read - set when a transmit buffer descriptor is read with its used bit set. Cleared by writing a one to this bit."]
pub type USED_BIT_READ_R = crate::BitReader<bool>;
#[doc = "Field `USED_BIT_READ` writer - Used bit read - set when a transmit buffer descriptor is read with its used bit set. Cleared by writing a one to this bit."]
pub type USED_BIT_READ_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TRANSMIT_STATUS_SPEC, bool, O>;
#[doc = "Field `COLLISION_OCCURRED` reader - Collision occurred - set by the assertion of collision. Cleared by writing a one to this bit. When operating in 10/100 mode, this status indicates either a collision or a late collision. In gigabit mode, this status is not set for a late collision."]
pub type COLLISION_OCCURRED_R = crate::BitReader<bool>;
#[doc = "Field `COLLISION_OCCURRED` writer - Collision occurred - set by the assertion of collision. Cleared by writing a one to this bit. When operating in 10/100 mode, this status indicates either a collision or a late collision. In gigabit mode, this status is not set for a late collision."]
pub type COLLISION_OCCURRED_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TRANSMIT_STATUS_SPEC, bool, O>;
#[doc = "Field `RETRY_LIMIT_EXCEEDED` reader - Retry limit exceeded - cleared by writing a one to this bit."]
pub type RETRY_LIMIT_EXCEEDED_R = crate::BitReader<bool>;
#[doc = "Field `RETRY_LIMIT_EXCEEDED` writer - Retry limit exceeded - cleared by writing a one to this bit."]
pub type RETRY_LIMIT_EXCEEDED_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TRANSMIT_STATUS_SPEC, bool, O>;
#[doc = "Field `TRANSMIT_GO` reader - Transmit go - if high transmit is active. When using the exposed FIFO interface, this bit represents bit 3 of the network control register. When using the DMA interface this bit represents the tx_go variable as specified in the transmit buffer description."]
pub type TRANSMIT_GO_R = crate::BitReader<bool>;
#[doc = "Field `AMBA_ERROR123` reader - Transmit frame corruption due to AMBA (AXI) errors. Set if an error occurs whilst midway through reading transmit frame from external memory including RRESP or BRESP errors and buffers exhausted mid frame (if the buffers run out during transmission of a frame then transmission stops, FCS shall be bad and tx_er asserted). Also set in DMA packet buffer mode if single frame is too large for configured packet buffer memory size. Cleared by writing a one to this bit."]
pub type AMBA_ERROR123_R = crate::BitReader<bool>;
#[doc = "Field `AMBA_ERROR123` writer - Transmit frame corruption due to AMBA (AXI) errors. Set if an error occurs whilst midway through reading transmit frame from external memory including RRESP or BRESP errors and buffers exhausted mid frame (if the buffers run out during transmission of a frame then transmission stops, FCS shall be bad and tx_er asserted). Also set in DMA packet buffer mode if single frame is too large for configured packet buffer memory size. Cleared by writing a one to this bit."]
pub type AMBA_ERROR123_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TRANSMIT_STATUS_SPEC, bool, O>;
#[doc = "Field `TRANSMIT_COMPLETE123` reader - Transmit complete - set when a frame has been transmitted. Cleared by writing a one to this bit."]
pub type TRANSMIT_COMPLETE123_R = crate::BitReader<bool>;
#[doc = "Field `TRANSMIT_COMPLETE123` writer - Transmit complete - set when a frame has been transmitted. Cleared by writing a one to this bit."]
pub type TRANSMIT_COMPLETE123_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TRANSMIT_STATUS_SPEC, bool, O>;
#[doc = "Field `TRANSMIT_UNDER_RUN123` reader - Transmit under run - this bit is set if the transmitter was forced to terminate a frame that it had already began transmitting due to further data being unavailable. This bit is set if a transmitter status write back has not completed when another status write back is attempted. When using the DMA interface configured for packet buffer mode, this bit will never be set. Cleared by writing a 1."]
pub type TRANSMIT_UNDER_RUN123_R = crate::BitReader<bool>;
#[doc = "Field `TRANSMIT_UNDER_RUN123` writer - Transmit under run - this bit is set if the transmitter was forced to terminate a frame that it had already began transmitting due to further data being unavailable. This bit is set if a transmitter status write back has not completed when another status write back is attempted. When using the DMA interface configured for packet buffer mode, this bit will never be set. Cleared by writing a 1."]
pub type TRANSMIT_UNDER_RUN123_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TRANSMIT_STATUS_SPEC, bool, O>;
#[doc = "Field `LATE_COLLISION_OCCURRED` reader - Late collision occurred - only set if the condition occurs in gigabit mode, as retry is not attempted. Cleared by writing a one to this bit."]
pub type LATE_COLLISION_OCCURRED_R = crate::BitReader<bool>;
#[doc = "Field `LATE_COLLISION_OCCURRED` writer - Late collision occurred - only set if the condition occurs in gigabit mode, as retry is not attempted. Cleared by writing a one to this bit."]
pub type LATE_COLLISION_OCCURRED_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TRANSMIT_STATUS_SPEC, bool, O>;
#[doc = "Field `RESP_NOT_OK123` reader - bresp/hresp not OK - set when the DMA block sees bresp/hresp not OK. Cleared by writing a one to this bit."]
pub type RESP_NOT_OK123_R = crate::BitReader<bool>;
#[doc = "Field `RESP_NOT_OK123` writer - bresp/hresp not OK - set when the DMA block sees bresp/hresp not OK. Cleared by writing a one to this bit."]
pub type RESP_NOT_OK123_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TRANSMIT_STATUS_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Used bit read - set when a transmit buffer descriptor is read with its used bit set. Cleared by writing a one to this bit."]
    #[inline(always)]
    pub fn used_bit_read(&self) -> USED_BIT_READ_R {
        USED_BIT_READ_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Collision occurred - set by the assertion of collision. Cleared by writing a one to this bit. When operating in 10/100 mode, this status indicates either a collision or a late collision. In gigabit mode, this status is not set for a late collision."]
    #[inline(always)]
    pub fn collision_occurred(&self) -> COLLISION_OCCURRED_R {
        COLLISION_OCCURRED_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Retry limit exceeded - cleared by writing a one to this bit."]
    #[inline(always)]
    pub fn retry_limit_exceeded(&self) -> RETRY_LIMIT_EXCEEDED_R {
        RETRY_LIMIT_EXCEEDED_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transmit go - if high transmit is active. When using the exposed FIFO interface, this bit represents bit 3 of the network control register. When using the DMA interface this bit represents the tx_go variable as specified in the transmit buffer description."]
    #[inline(always)]
    pub fn transmit_go(&self) -> TRANSMIT_GO_R {
        TRANSMIT_GO_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Transmit frame corruption due to AMBA (AXI) errors. Set if an error occurs whilst midway through reading transmit frame from external memory including RRESP or BRESP errors and buffers exhausted mid frame (if the buffers run out during transmission of a frame then transmission stops, FCS shall be bad and tx_er asserted). Also set in DMA packet buffer mode if single frame is too large for configured packet buffer memory size. Cleared by writing a one to this bit."]
    #[inline(always)]
    pub fn amba_error123(&self) -> AMBA_ERROR123_R {
        AMBA_ERROR123_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Transmit complete - set when a frame has been transmitted. Cleared by writing a one to this bit."]
    #[inline(always)]
    pub fn transmit_complete123(&self) -> TRANSMIT_COMPLETE123_R {
        TRANSMIT_COMPLETE123_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Transmit under run - this bit is set if the transmitter was forced to terminate a frame that it had already began transmitting due to further data being unavailable. This bit is set if a transmitter status write back has not completed when another status write back is attempted. When using the DMA interface configured for packet buffer mode, this bit will never be set. Cleared by writing a 1."]
    #[inline(always)]
    pub fn transmit_under_run123(&self) -> TRANSMIT_UNDER_RUN123_R {
        TRANSMIT_UNDER_RUN123_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Late collision occurred - only set if the condition occurs in gigabit mode, as retry is not attempted. Cleared by writing a one to this bit."]
    #[inline(always)]
    pub fn late_collision_occurred(&self) -> LATE_COLLISION_OCCURRED_R {
        LATE_COLLISION_OCCURRED_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - bresp/hresp not OK - set when the DMA block sees bresp/hresp not OK. Cleared by writing a one to this bit."]
    #[inline(always)]
    pub fn resp_not_ok123(&self) -> RESP_NOT_OK123_R {
        RESP_NOT_OK123_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Used bit read - set when a transmit buffer descriptor is read with its used bit set. Cleared by writing a one to this bit."]
    #[inline(always)]
    #[must_use]
    pub fn used_bit_read(&mut self) -> USED_BIT_READ_W<0> {
        USED_BIT_READ_W::new(self)
    }
    #[doc = "Bit 1 - Collision occurred - set by the assertion of collision. Cleared by writing a one to this bit. When operating in 10/100 mode, this status indicates either a collision or a late collision. In gigabit mode, this status is not set for a late collision."]
    #[inline(always)]
    #[must_use]
    pub fn collision_occurred(&mut self) -> COLLISION_OCCURRED_W<1> {
        COLLISION_OCCURRED_W::new(self)
    }
    #[doc = "Bit 2 - Retry limit exceeded - cleared by writing a one to this bit."]
    #[inline(always)]
    #[must_use]
    pub fn retry_limit_exceeded(&mut self) -> RETRY_LIMIT_EXCEEDED_W<2> {
        RETRY_LIMIT_EXCEEDED_W::new(self)
    }
    #[doc = "Bit 4 - Transmit frame corruption due to AMBA (AXI) errors. Set if an error occurs whilst midway through reading transmit frame from external memory including RRESP or BRESP errors and buffers exhausted mid frame (if the buffers run out during transmission of a frame then transmission stops, FCS shall be bad and tx_er asserted). Also set in DMA packet buffer mode if single frame is too large for configured packet buffer memory size. Cleared by writing a one to this bit."]
    #[inline(always)]
    #[must_use]
    pub fn amba_error123(&mut self) -> AMBA_ERROR123_W<4> {
        AMBA_ERROR123_W::new(self)
    }
    #[doc = "Bit 5 - Transmit complete - set when a frame has been transmitted. Cleared by writing a one to this bit."]
    #[inline(always)]
    #[must_use]
    pub fn transmit_complete123(&mut self) -> TRANSMIT_COMPLETE123_W<5> {
        TRANSMIT_COMPLETE123_W::new(self)
    }
    #[doc = "Bit 6 - Transmit under run - this bit is set if the transmitter was forced to terminate a frame that it had already began transmitting due to further data being unavailable. This bit is set if a transmitter status write back has not completed when another status write back is attempted. When using the DMA interface configured for packet buffer mode, this bit will never be set. Cleared by writing a 1."]
    #[inline(always)]
    #[must_use]
    pub fn transmit_under_run123(&mut self) -> TRANSMIT_UNDER_RUN123_W<6> {
        TRANSMIT_UNDER_RUN123_W::new(self)
    }
    #[doc = "Bit 7 - Late collision occurred - only set if the condition occurs in gigabit mode, as retry is not attempted. Cleared by writing a one to this bit."]
    #[inline(always)]
    #[must_use]
    pub fn late_collision_occurred(&mut self) -> LATE_COLLISION_OCCURRED_W<7> {
        LATE_COLLISION_OCCURRED_W::new(self)
    }
    #[doc = "Bit 8 - bresp/hresp not OK - set when the DMA block sees bresp/hresp not OK. Cleared by writing a one to this bit."]
    #[inline(always)]
    #[must_use]
    pub fn resp_not_ok123(&mut self) -> RESP_NOT_OK123_W<8> {
        RESP_NOT_OK123_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register, when read, provides details of the status of a transmit. Once read, individual bits may be cleared by writing 1 to them. It is not possible to set a bit to 1 by writing to the register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [transmit_status](index.html) module"]
pub struct TRANSMIT_STATUS_SPEC;
impl crate::RegisterSpec for TRANSMIT_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [transmit_status::R](R) reader structure"]
impl crate::Readable for TRANSMIT_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [transmit_status::W](W) writer structure"]
impl crate::Writable for TRANSMIT_STATUS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TRANSMIT_STATUS to value 0"]
impl crate::Resettable for TRANSMIT_STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
