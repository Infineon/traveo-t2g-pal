#[doc = "Register `LBOALLFETCHESACTIVECOUNTER` reader"]
pub struct R(crate::R<LBOALLFETCHESACTIVECOUNTER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LBOALLFETCHESACTIVECOUNTER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LBOALLFETCHESACTIVECOUNTER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LBOALLFETCHESACTIVECOUNTER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `LBOALLFETCHESACTIVECOUNTER` reader - The internal counter is incremented with each 16th clock when all source fetch pipelines are active. Only active in LBO and when LBOUtilizationMeasurementEn is set. This field is updated with SaveAndResetMeasurements pulse and internal counter is reset."]
pub type LBOALLFETCHESACTIVECOUNTER_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - The internal counter is incremented with each 16th clock when all source fetch pipelines are active. Only active in LBO and when LBOUtilizationMeasurementEn is set. This field is updated with SaveAndResetMeasurements pulse and internal counter is reset."]
    #[inline(always)]
    pub fn lboallfetchesactivecounter(&self) -> LBOALLFETCHESACTIVECOUNTER_R {
        LBOALLFETCHESACTIVECOUNTER_R::new(self.bits)
    }
}
#[doc = "One of the LBO utilization measurement registers.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lboallfetchesactivecounter](index.html) module"]
pub struct LBOALLFETCHESACTIVECOUNTER_SPEC;
impl crate::RegisterSpec for LBOALLFETCHESACTIVECOUNTER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lboallfetchesactivecounter::R](R) reader structure"]
impl crate::Readable for LBOALLFETCHESACTIVECOUNTER_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets LBOALLFETCHESACTIVECOUNTER to value 0"]
impl crate::Resettable for LBOALLFETCHESACTIVECOUNTER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
