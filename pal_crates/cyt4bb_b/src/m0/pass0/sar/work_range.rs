#[doc = "Register `WORK_RANGE` reader"]
pub struct R(crate::R<WORK_RANGE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WORK_RANGE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WORK_RANGE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WORK_RANGE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RANGE` reader - N/A"]
pub type RANGE_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - N/A"]
    #[inline(always)]
    pub fn range(&self) -> RANGE_R {
        RANGE_R::new(self.bits)
    }
}
#[doc = "Range detected\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [work_range](index.html) module"]
pub struct WORK_RANGE_SPEC;
impl crate::RegisterSpec for WORK_RANGE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [work_range::R](R) reader structure"]
impl crate::Readable for WORK_RANGE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets WORK_RANGE to value 0"]
impl crate::Resettable for WORK_RANGE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
