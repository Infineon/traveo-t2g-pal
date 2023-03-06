#[doc = "Register `PERFCOUNTERFRONT` reader"]
pub struct R(crate::R<PERFCOUNTERFRONT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PERFCOUNTERFRONT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PERFCOUNTERFRONT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PERFCOUNTERFRONT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PERFRESULTFRONT` reader - Returns the performance counter value of the front pipeline. For the total cycle count add PerfResultFront and PerfResultBack. For debug purposes only, read when stable only."]
pub type PERFRESULTFRONT_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Returns the performance counter value of the front pipeline. For the total cycle count add PerfResultFront and PerfResultBack. For debug purposes only, read when stable only."]
    #[inline(always)]
    pub fn perfresultfront(&self) -> PERFRESULTFRONT_R {
        PERFRESULTFRONT_R::new(self.bits)
    }
}
#[doc = "Performance counter result\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [perfcounterfront](index.html) module"]
pub struct PERFCOUNTERFRONT_SPEC;
impl crate::RegisterSpec for PERFCOUNTERFRONT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [perfcounterfront::R](R) reader structure"]
impl crate::Readable for PERFCOUNTERFRONT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PERFCOUNTERFRONT to value 0"]
impl crate::Resettable for PERFCOUNTERFRONT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
