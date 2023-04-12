#[doc = "Register `COMP0_STATUS` reader"]
pub struct R(crate::R<COMP0_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<COMP0_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<COMP0_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<COMP0_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `COMP0_OUT` reader - Active comparator 'comp0_out\\[\\]' outputs."]
pub type COMP0_OUT_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Active comparator 'comp0_out\\[\\]' outputs."]
    #[inline(always)]
    pub fn comp0_out(&self) -> COMP0_OUT_R {
        COMP0_OUT_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Comparator structures comparator 0 status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [comp0_status](index.html) module"]
pub struct COMP0_STATUS_SPEC;
impl crate::RegisterSpec for COMP0_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [comp0_status::R](R) reader structure"]
impl crate::Readable for COMP0_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets COMP0_STATUS to value 0"]
impl crate::Resettable for COMP0_STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
