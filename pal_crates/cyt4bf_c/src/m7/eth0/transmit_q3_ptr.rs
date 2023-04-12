#[doc = "Register `TRANSMIT_Q3_PTR` reader"]
pub struct R(crate::R<TRANSMIT_Q3_PTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TRANSMIT_Q3_PTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TRANSMIT_Q3_PTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TRANSMIT_Q3_PTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `REMOVED_31_0` reader - Disable queue if set to 1. This can be used to reduce the number of active queues and should only be changed while transmit is not enabled."]
pub type REMOVED_31_0_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Disable queue if set to 1. This can be used to reduce the number of active queues and should only be changed while transmit is not enabled."]
    #[inline(always)]
    pub fn removed_31_0(&self) -> REMOVED_31_0_R {
        REMOVED_31_0_R::new(self.bits)
    }
}
#[doc = "transmit_q3_ptr to transmit_q15_ptr doesn't present. Access to the register returns AHB error.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [transmit_q3_ptr](index.html) module"]
pub struct TRANSMIT_Q3_PTR_SPEC;
impl crate::RegisterSpec for TRANSMIT_Q3_PTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [transmit_q3_ptr::R](R) reader structure"]
impl crate::Readable for TRANSMIT_Q3_PTR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TRANSMIT_Q3_PTR to value 0"]
impl crate::Resettable for TRANSMIT_Q3_PTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
