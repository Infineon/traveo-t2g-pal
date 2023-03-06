#[doc = "Register `INT_Q1_DISABLE` writer"]
pub struct W(crate::W<INT_Q1_DISABLE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INT_Q1_DISABLE_SPEC>;
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
impl From<crate::W<INT_Q1_DISABLE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INT_Q1_DISABLE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DISABLE_RECEIVE_COMPLETE_INTERRUPT` writer - Disable Receive complete interrupt"]
pub type DISABLE_RECEIVE_COMPLETE_INTERRUPT_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_Q1_DISABLE_SPEC, bool, O>;
#[doc = "Field `DISABLE_RX_USED_BIT_READ_INTERRUPT` writer - Disable RX used bit read interrupt"]
pub type DISABLE_RX_USED_BIT_READ_INTERRUPT_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_Q1_DISABLE_SPEC, bool, O>;
#[doc = "Field `DISABLE_RETRY_LIMIT_EXCEEDED_OR_LATE_COLLISION_INTERRUPT` writer - Disable Retry limit exceeded or late collision interrupt"]
pub type DISABLE_RETRY_LIMIT_EXCEEDED_OR_LATE_COLLISION_INTERRUPT_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_Q1_DISABLE_SPEC, bool, O>;
#[doc = "Field `DISABLE_TRANSMIT_FRAME_CORRUPTION_DUE_TO_AMBA_ERROR_INTERRUPT` writer - Disable Transmit frame corruption due to AMBA (AXI) error interrupt"]
pub type DISABLE_TRANSMIT_FRAME_CORRUPTION_DUE_TO_AMBA_ERROR_INTERRUPT_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_Q1_DISABLE_SPEC, bool, O>;
#[doc = "Field `DISABLE_TRANSMIT_COMPLETE_INTERRUPT` writer - Disable Transmit complete interrupt"]
pub type DISABLE_TRANSMIT_COMPLETE_INTERRUPT_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_Q1_DISABLE_SPEC, bool, O>;
#[doc = "Field `DISABLE_RESP_NOT_OK_INTERRUPT` writer - Disable bresp not OK interrupt"]
pub type DISABLE_RESP_NOT_OK_INTERRUPT_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_Q1_DISABLE_SPEC, bool, O>;
impl W {
    #[doc = "Bit 1 - Disable Receive complete interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn disable_receive_complete_interrupt(
        &mut self,
    ) -> DISABLE_RECEIVE_COMPLETE_INTERRUPT_W<1> {
        DISABLE_RECEIVE_COMPLETE_INTERRUPT_W::new(self)
    }
    #[doc = "Bit 2 - Disable RX used bit read interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn disable_rx_used_bit_read_interrupt(
        &mut self,
    ) -> DISABLE_RX_USED_BIT_READ_INTERRUPT_W<2> {
        DISABLE_RX_USED_BIT_READ_INTERRUPT_W::new(self)
    }
    #[doc = "Bit 5 - Disable Retry limit exceeded or late collision interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn disable_retry_limit_exceeded_or_late_collision_interrupt(
        &mut self,
    ) -> DISABLE_RETRY_LIMIT_EXCEEDED_OR_LATE_COLLISION_INTERRUPT_W<5> {
        DISABLE_RETRY_LIMIT_EXCEEDED_OR_LATE_COLLISION_INTERRUPT_W::new(self)
    }
    #[doc = "Bit 6 - Disable Transmit frame corruption due to AMBA (AXI) error interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn disable_transmit_frame_corruption_due_to_amba_error_interrupt(
        &mut self,
    ) -> DISABLE_TRANSMIT_FRAME_CORRUPTION_DUE_TO_AMBA_ERROR_INTERRUPT_W<6> {
        DISABLE_TRANSMIT_FRAME_CORRUPTION_DUE_TO_AMBA_ERROR_INTERRUPT_W::new(self)
    }
    #[doc = "Bit 7 - Disable Transmit complete interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn disable_transmit_complete_interrupt(
        &mut self,
    ) -> DISABLE_TRANSMIT_COMPLETE_INTERRUPT_W<7> {
        DISABLE_TRANSMIT_COMPLETE_INTERRUPT_W::new(self)
    }
    #[doc = "Bit 11 - Disable bresp not OK interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn disable_resp_not_ok_interrupt(&mut self) -> DISABLE_RESP_NOT_OK_INTERRUPT_W<11> {
        DISABLE_RESP_NOT_OK_INTERRUPT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Writing a 1 to the relevant bit location disables that particular interrupt. This register is write only and when read will return zero.\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_q1_disable](index.html) module"]
pub struct INT_Q1_DISABLE_SPEC;
impl crate::RegisterSpec for INT_Q1_DISABLE_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [int_q1_disable::W](W) writer structure"]
impl crate::Writable for INT_Q1_DISABLE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INT_Q1_DISABLE to value 0"]
impl crate::Resettable for INT_Q1_DISABLE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
