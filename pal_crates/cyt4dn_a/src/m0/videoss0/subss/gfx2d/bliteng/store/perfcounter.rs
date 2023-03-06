#[doc = "Register `PERFCOUNTER` reader"]
pub struct R(crate::R<PERFCOUNTER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PERFCOUNTER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PERFCOUNTER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PERFCOUNTER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PERFRESULT` reader - Returns the performance counter value. Please note that a sw reset during a frame can potentially produce invalid results in the first frame afterwards. For debug purposes only, read when stable only, otherwise read data might be corrupted."]
pub type PERFRESULT_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Returns the performance counter value. Please note that a sw reset during a frame can potentially produce invalid results in the first frame afterwards. For debug purposes only, read when stable only, otherwise read data might be corrupted."]
    #[inline(always)]
    pub fn perfresult(&self) -> PERFRESULT_R {
        PERFRESULT_R::new(self.bits)
    }
}
#[doc = "Performance counter result\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [perfcounter](index.html) module"]
pub struct PERFCOUNTER_SPEC;
impl crate::RegisterSpec for PERFCOUNTER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [perfcounter::R](R) reader structure"]
impl crate::Readable for PERFCOUNTER_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PERFCOUNTER to value 0"]
impl crate::Resettable for PERFCOUNTER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
