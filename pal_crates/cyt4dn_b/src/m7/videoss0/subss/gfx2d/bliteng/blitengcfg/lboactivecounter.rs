#[doc = "Register `LBOACTIVECOUNTER` reader"]
pub struct R(crate::R<LBOACTIVECOUNTER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LBOACTIVECOUNTER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LBOACTIVECOUNTER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LBOACTIVECOUNTER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `LBOACTIVECOUNTER` reader - The internal LBO counter is incremented with each 16th clock cycle when BlitEng is executing LBO operation and when BlitEngUtilizationMeasurementEn is set. This field is updated with SaveAndResetMeasurements pulse and internal counter is reset."]
pub type LBOACTIVECOUNTER_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - The internal LBO counter is incremented with each 16th clock cycle when BlitEng is executing LBO operation and when BlitEngUtilizationMeasurementEn is set. This field is updated with SaveAndResetMeasurements pulse and internal counter is reset."]
    #[inline(always)]
    pub fn lboactivecounter(&self) -> LBOACTIVECOUNTER_R {
        LBOACTIVECOUNTER_R::new(self.bits)
    }
}
#[doc = "One of BlitEng Utilization Measurement registers.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lboactivecounter](index.html) module"]
pub struct LBOACTIVECOUNTER_SPEC;
impl crate::RegisterSpec for LBOACTIVECOUNTER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lboactivecounter::R](R) reader structure"]
impl crate::Readable for LBOACTIVECOUNTER_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets LBOACTIVECOUNTER to value 0"]
impl crate::Resettable for LBOACTIVECOUNTER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
