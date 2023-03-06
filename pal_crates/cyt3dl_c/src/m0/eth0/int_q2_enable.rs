#[doc = "Register `INT_Q2_ENABLE` writer"]
pub struct W(crate::W<INT_Q2_ENABLE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INT_Q2_ENABLE_SPEC>;
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
impl From<crate::W<INT_Q2_ENABLE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INT_Q2_ENABLE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENABLE_RECEIVE_COMPLETE_INTERRUPT` writer - Enable Receive complete interrupt"]
pub type ENABLE_RECEIVE_COMPLETE_INTERRUPT_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_Q2_ENABLE_SPEC, bool, O>;
#[doc = "Field `ENABLE_RX_USED_BIT_READ_INTERRUPT` writer - Enable RX used bit read interrupt"]
pub type ENABLE_RX_USED_BIT_READ_INTERRUPT_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_Q2_ENABLE_SPEC, bool, O>;
#[doc = "Field `ENABLE_RETRY_LIMIT_EXCEEDED_OR_LATE_COLLISION_INTERRUPT` writer - Enable Retry limit exceeded or late collision interrupt"]
pub type ENABLE_RETRY_LIMIT_EXCEEDED_OR_LATE_COLLISION_INTERRUPT_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_Q2_ENABLE_SPEC, bool, O>;
#[doc = "Field `ENABLE_TRANSMIT_FRAME_CORRUPTION_DUE_TO_AMBA_ERROR_INTERRUPT` writer - Enable Transmit frame corruption due to AMBA (AXI/AHB) error interrupt"]
pub type ENABLE_TRANSMIT_FRAME_CORRUPTION_DUE_TO_AMBA_ERROR_INTERRUPT_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_Q2_ENABLE_SPEC, bool, O>;
#[doc = "Field `ENABLE_TRANSMIT_COMPLETE_INTERRUPT` writer - Enable Transmit complete interrupt"]
pub type ENABLE_TRANSMIT_COMPLETE_INTERRUPT_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_Q2_ENABLE_SPEC, bool, O>;
#[doc = "Field `ENABLE_RESP_NOT_OK_INTERRUPT` writer - Enable bresp not OK interrupt"]
pub type ENABLE_RESP_NOT_OK_INTERRUPT_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_Q2_ENABLE_SPEC, bool, O>;
impl W {
    #[doc = "Bit 1 - Enable Receive complete interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn enable_receive_complete_interrupt(&mut self) -> ENABLE_RECEIVE_COMPLETE_INTERRUPT_W<1> {
        ENABLE_RECEIVE_COMPLETE_INTERRUPT_W::new(self)
    }
    #[doc = "Bit 2 - Enable RX used bit read interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn enable_rx_used_bit_read_interrupt(&mut self) -> ENABLE_RX_USED_BIT_READ_INTERRUPT_W<2> {
        ENABLE_RX_USED_BIT_READ_INTERRUPT_W::new(self)
    }
    #[doc = "Bit 5 - Enable Retry limit exceeded or late collision interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn enable_retry_limit_exceeded_or_late_collision_interrupt(
        &mut self,
    ) -> ENABLE_RETRY_LIMIT_EXCEEDED_OR_LATE_COLLISION_INTERRUPT_W<5> {
        ENABLE_RETRY_LIMIT_EXCEEDED_OR_LATE_COLLISION_INTERRUPT_W::new(self)
    }
    #[doc = "Bit 6 - Enable Transmit frame corruption due to AMBA (AXI/AHB) error interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn enable_transmit_frame_corruption_due_to_amba_error_interrupt(
        &mut self,
    ) -> ENABLE_TRANSMIT_FRAME_CORRUPTION_DUE_TO_AMBA_ERROR_INTERRUPT_W<6> {
        ENABLE_TRANSMIT_FRAME_CORRUPTION_DUE_TO_AMBA_ERROR_INTERRUPT_W::new(self)
    }
    #[doc = "Bit 7 - Enable Transmit complete interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn enable_transmit_complete_interrupt(
        &mut self,
    ) -> ENABLE_TRANSMIT_COMPLETE_INTERRUPT_W<7> {
        ENABLE_TRANSMIT_COMPLETE_INTERRUPT_W::new(self)
    }
    #[doc = "Bit 11 - Enable bresp not OK interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn enable_resp_not_ok_interrupt(&mut self) -> ENABLE_RESP_NOT_OK_INTERRUPT_W<11> {
        ENABLE_RESP_NOT_OK_INTERRUPT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "At reset all interrupts are disabled. Writing a one to the relevant bit location enables the required interrupt. This register is write only and when read will return zero.\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_q2_enable](index.html) module"]
pub struct INT_Q2_ENABLE_SPEC;
impl crate::RegisterSpec for INT_Q2_ENABLE_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [int_q2_enable::W](W) writer structure"]
impl crate::Writable for INT_Q2_ENABLE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INT_Q2_ENABLE to value 0"]
impl crate::Resettable for INT_Q2_ENABLE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
