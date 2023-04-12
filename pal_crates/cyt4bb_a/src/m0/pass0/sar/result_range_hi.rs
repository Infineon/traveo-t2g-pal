#[doc = "Register `RESULT_RANGE_HI` reader"]
pub struct R(crate::R<RESULT_RANGE_HI_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RESULT_RANGE_HI_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RESULT_RANGE_HI_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RESULT_RANGE_HI_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ABOVE_HI` reader - Out of range was detected and the value was above the Hi threshold"]
pub type ABOVE_HI_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Out of range was detected and the value was above the Hi threshold"]
    #[inline(always)]
    pub fn above_hi(&self) -> ABOVE_HI_R {
        ABOVE_HI_R::new(self.bits)
    }
}
#[doc = "Channel Range above Hi flags\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [result_range_hi](index.html) module"]
pub struct RESULT_RANGE_HI_SPEC;
impl crate::RegisterSpec for RESULT_RANGE_HI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [result_range_hi::R](R) reader structure"]
impl crate::Readable for RESULT_RANGE_HI_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RESULT_RANGE_HI to value 0"]
impl crate::Resettable for RESULT_RANGE_HI_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
