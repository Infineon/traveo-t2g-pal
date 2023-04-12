#[doc = "Register `INT_Q2_STATUS` reader"]
pub struct R(crate::R<INT_Q2_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INT_Q2_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INT_Q2_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INT_Q2_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RECEIVE_COMPLETE` reader - Receive complete"]
pub type RECEIVE_COMPLETE_R = crate::BitReader<bool>;
#[doc = "Field `RX_USED_BIT_READ` reader - RX used bit read"]
pub type RX_USED_BIT_READ_R = crate::BitReader<bool>;
#[doc = "Field `RETRY_LIMIT_EXCEEDED_OR_LATE_COLLISION` reader - Retry limit exceeded or late collision"]
pub type RETRY_LIMIT_EXCEEDED_OR_LATE_COLLISION_R = crate::BitReader<bool>;
#[doc = "Field `AMBA_ERROR` reader - Transmit frame corruption due to AMBA (AXI) error set if an error occurs whilst midway through reading transmit frame from the external memory, including RRESP and BRESP errors (AXI) and buffers exhausted mid frame"]
pub type AMBA_ERROR_R = crate::BitReader<bool>;
#[doc = "Field `TRANSMIT_COMPLETE` reader - Transmit complete"]
pub type TRANSMIT_COMPLETE_R = crate::BitReader<bool>;
#[doc = "Field `RESP_NOT_OK` reader - bresp not OK"]
pub type RESP_NOT_OK_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 1 - Receive complete"]
    #[inline(always)]
    pub fn receive_complete(&self) -> RECEIVE_COMPLETE_R {
        RECEIVE_COMPLETE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RX used bit read"]
    #[inline(always)]
    pub fn rx_used_bit_read(&self) -> RX_USED_BIT_READ_R {
        RX_USED_BIT_READ_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 5 - Retry limit exceeded or late collision"]
    #[inline(always)]
    pub fn retry_limit_exceeded_or_late_collision(
        &self,
    ) -> RETRY_LIMIT_EXCEEDED_OR_LATE_COLLISION_R {
        RETRY_LIMIT_EXCEEDED_OR_LATE_COLLISION_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Transmit frame corruption due to AMBA (AXI) error set if an error occurs whilst midway through reading transmit frame from the external memory, including RRESP and BRESP errors (AXI) and buffers exhausted mid frame"]
    #[inline(always)]
    pub fn amba_error(&self) -> AMBA_ERROR_R {
        AMBA_ERROR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Transmit complete"]
    #[inline(always)]
    pub fn transmit_complete(&self) -> TRANSMIT_COMPLETE_R {
        TRANSMIT_COMPLETE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 11 - bresp not OK"]
    #[inline(always)]
    pub fn resp_not_ok(&self) -> RESP_NOT_OK_R {
        RESP_NOT_OK_R::new(((self.bits >> 11) & 1) != 0)
    }
}
#[doc = "Priority queue Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_q2_status](index.html) module"]
pub struct INT_Q2_STATUS_SPEC;
impl crate::RegisterSpec for INT_Q2_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [int_q2_status::R](R) reader structure"]
impl crate::Readable for INT_Q2_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets INT_Q2_STATUS to value 0"]
impl crate::Resettable for INT_Q2_STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
