#[doc = "Register `PERFCOUNTERBACK` reader"]
pub struct R(crate::R<PERFCOUNTERBACK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PERFCOUNTERBACK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PERFCOUNTERBACK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PERFCOUNTERBACK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PERFRESULTBACK` reader - Returns the performance counter value of the back pipeline. For the total cycle count add PerfResultFront and PerfResultBack. For debug purposes only, read when stable only."]
pub type PERFRESULTBACK_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Returns the performance counter value of the back pipeline. For the total cycle count add PerfResultFront and PerfResultBack. For debug purposes only, read when stable only."]
    #[inline(always)]
    pub fn perfresultback(&self) -> PERFRESULTBACK_R {
        PERFRESULTBACK_R::new(self.bits)
    }
}
#[doc = "Performance counter result\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [perfcounterback](index.html) module"]
pub struct PERFCOUNTERBACK_SPEC;
impl crate::RegisterSpec for PERFCOUNTERBACK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [perfcounterback::R](R) reader structure"]
impl crate::Readable for PERFCOUNTERBACK_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PERFCOUNTERBACK to value 0"]
impl crate::Resettable for PERFCOUNTERBACK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
