#[doc = "Register `TR_PEND` reader"]
pub struct R(crate::R<TR_PEND_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TR_PEND_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TR_PEND_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TR_PEND_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TR_PEND` reader - Trigger Pending. Hardware will set this bit if a hardware trigger is received."]
pub type TR_PEND_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Trigger Pending. Hardware will set this bit if a hardware trigger is received."]
    #[inline(always)]
    pub fn tr_pend(&self) -> TR_PEND_R {
        TR_PEND_R::new(self.bits)
    }
}
#[doc = "Trigger pending status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tr_pend](index.html) module"]
pub struct TR_PEND_SPEC;
impl crate::RegisterSpec for TR_PEND_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tr_pend::R](R) reader structure"]
impl crate::Readable for TR_PEND_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TR_PEND to value 0"]
impl crate::Resettable for TR_PEND_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
