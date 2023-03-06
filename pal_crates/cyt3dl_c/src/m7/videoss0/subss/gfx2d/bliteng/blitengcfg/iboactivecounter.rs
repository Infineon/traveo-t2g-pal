#[doc = "Register `IBOACTIVECOUNTER` reader"]
pub struct R(crate::R<IBOACTIVECOUNTER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IBOACTIVECOUNTER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IBOACTIVECOUNTER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IBOACTIVECOUNTER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `IBOACTIVECOUNTER` reader - The internal LBO counter is incremented with each 16th clock cycle when BlitEng is executing IBO operation and when BlitEngUtilizationMeasurementEn is set. This field is updated with SaveAndResetMeasurements pulse and internal counter is reset."]
pub type IBOACTIVECOUNTER_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - The internal LBO counter is incremented with each 16th clock cycle when BlitEng is executing IBO operation and when BlitEngUtilizationMeasurementEn is set. This field is updated with SaveAndResetMeasurements pulse and internal counter is reset."]
    #[inline(always)]
    pub fn iboactivecounter(&self) -> IBOACTIVECOUNTER_R {
        IBOACTIVECOUNTER_R::new(self.bits)
    }
}
#[doc = "One of BlitEng Utilization Measurement registers.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iboactivecounter](index.html) module"]
pub struct IBOACTIVECOUNTER_SPEC;
impl crate::RegisterSpec for IBOACTIVECOUNTER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [iboactivecounter::R](R) reader structure"]
impl crate::Readable for IBOACTIVECOUNTER_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets IBOACTIVECOUNTER to value 0"]
impl crate::Resettable for IBOACTIVECOUNTER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
