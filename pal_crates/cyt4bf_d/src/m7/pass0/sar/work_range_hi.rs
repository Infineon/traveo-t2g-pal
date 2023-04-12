#[doc = "Register `WORK_RANGE_HI` reader"]
pub struct R(crate::R<WORK_RANGE_HI_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WORK_RANGE_HI_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WORK_RANGE_HI_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WORK_RANGE_HI_SPEC>) -> Self {
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
#[doc = "Range detect above Hi flag\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [work_range_hi](index.html) module"]
pub struct WORK_RANGE_HI_SPEC;
impl crate::RegisterSpec for WORK_RANGE_HI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [work_range_hi::R](R) reader structure"]
impl crate::Readable for WORK_RANGE_HI_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets WORK_RANGE_HI to value 0"]
impl crate::Resettable for WORK_RANGE_HI_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
