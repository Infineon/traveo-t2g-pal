#[doc = "Register `PERFMEASUREMENTCLOCKCOUNTERRUNNING` reader"]
pub struct R(crate::R<PERFMEASUREMENTCLOCKCOUNTERRUNNING_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PERFMEASUREMENTCLOCKCOUNTERRUNNING_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PERFMEASUREMENTCLOCKCOUNTERRUNNING_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PERFMEASUREMENTCLOCKCOUNTERRUNNING_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CLOCKCOUNTERRUNNING` reader - ClockCounterRunning is incremented each clock cycle."]
pub type CLOCKCOUNTERRUNNING_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - ClockCounterRunning is incremented each clock cycle."]
    #[inline(always)]
    pub fn clockcounterrunning(&self) -> CLOCKCOUNTERRUNNING_R {
        CLOCKCOUNTERRUNNING_R::new(self.bits)
    }
}
#[doc = "Performance measurement register for BlitEng containing clock cycle counter. Measurement is active only if PerfMeasurementEn is set.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [perfmeasurementclockcounterrunning](index.html) module"]
pub struct PERFMEASUREMENTCLOCKCOUNTERRUNNING_SPEC;
impl crate::RegisterSpec for PERFMEASUREMENTCLOCKCOUNTERRUNNING_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [perfmeasurementclockcounterrunning::R](R) reader structure"]
impl crate::Readable for PERFMEASUREMENTCLOCKCOUNTERRUNNING_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PERFMEASUREMENTCLOCKCOUNTERRUNNING to value 0"]
impl crate::Resettable for PERFMEASUREMENTCLOCKCOUNTERRUNNING_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
