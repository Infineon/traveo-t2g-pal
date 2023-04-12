#[doc = "Register `RECEIVE_Q7_PTR` reader"]
pub struct R(crate::R<RECEIVE_Q7_PTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RECEIVE_Q7_PTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RECEIVE_Q7_PTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RECEIVE_Q7_PTR_SPEC>) -> Self {
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
#[doc = "Not presents.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [receive_q7_ptr](index.html) module"]
pub struct RECEIVE_Q7_PTR_SPEC;
impl crate::RegisterSpec for RECEIVE_Q7_PTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [receive_q7_ptr::R](R) reader structure"]
impl crate::Readable for RECEIVE_Q7_PTR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RECEIVE_Q7_PTR to value 0"]
impl crate::Resettable for RECEIVE_Q7_PTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
